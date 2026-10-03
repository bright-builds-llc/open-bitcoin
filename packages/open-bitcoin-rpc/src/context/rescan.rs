// Parity breadcrumbs:
// - packages/bitcoin-knots/src/wallet/wallet.cpp
// - packages/bitcoin-knots/src/wallet/rpc/transactions.cpp
// - packages/bitcoin-knots/src/coins.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp

use std::collections::BTreeSet;

use open_bitcoin_node::core::chainstate::ChainstateSnapshot;
use open_bitcoin_node::core::primitives::BlockHash;
use open_bitcoin_node::core::wallet::Wallet;
use open_bitcoin_node::wallet_registry::rescan::{
    PreparedWalletRescan, WalletRescanEligibilityBoundary, WalletRescanEligibilityFailure,
    guard_durable_wallet_rescan_authority, prepare_durable_wallet_rescan,
};
use open_bitcoin_node::{
    FjallNodeStore, PersistMode, StorageError, StorageNamespace, WalletRegistry,
    WalletRegistryError, WalletRescanFreshness, WalletRescanJob, WalletRescanJobState,
};

use crate::{dispatch::network_authority_error_to_failure, error::RpcFailure};

use super::ManagedRpcContext;
use super::wallet_state::{
    WalletState, load_wallet_registry, resolve_selected_wallet_name, wallet_error_to_failure,
    wallet_registry_error_to_failure,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WalletFreshnessKind {
    Fresh,
    Partial,
    Scanning,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WalletFreshnessView {
    pub scanning: bool,
    pub freshness: WalletFreshnessKind,
    pub maybe_scanned_through_height: Option<u32>,
    pub maybe_target_height: Option<u32>,
    pub maybe_next_height: Option<u32>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WalletRescanExecution {
    pub start_height: u32,
    pub stop_height: u32,
    pub freshness: WalletFreshnessView,
}

impl<S: open_bitcoin_node::ChainstateStore, V: open_bitcoin_node::core::chainstate::CoinsView>
    ManagedRpcContext<S, V>
{
    pub fn wallet_rescan_job(&self) -> Result<Option<WalletRescanJob>, RpcFailure> {
        let WalletState::DurableNamedRegistry { store, .. } = &self.wallet_state else {
            return Ok(None);
        };
        let registry = load_wallet_registry(store)?;
        let wallet_name = resolve_selected_wallet_name(self.request_wallet_name(), &registry)?;
        Ok(registry.rescan_job(&wallet_name).cloned())
    }

    pub fn wallet_freshness(&self) -> Result<WalletFreshnessView, RpcFailure> {
        if let Some(job) = self.wallet_rescan_job()?
            && matches!(
                job.state,
                WalletRescanJobState::Pending | WalletRescanJobState::Scanning
            )
        {
            return Ok(WalletFreshnessView {
                scanning: true,
                freshness: match job.freshness {
                    WalletRescanFreshness::Fresh => WalletFreshnessKind::Fresh,
                    WalletRescanFreshness::Partial => WalletFreshnessKind::Partial,
                    WalletRescanFreshness::Scanning => WalletFreshnessKind::Scanning,
                },
                maybe_scanned_through_height: job.maybe_scanned_through_height,
                maybe_target_height: Some(job.target_tip_height),
                maybe_next_height: Some(job.next_height),
            });
        }

        let maybe_wallet_tip_height = self.wallet_snapshot()?.maybe_tip_height;
        let maybe_chain_tip_height = self
            .maybe_chain_tip()
            .map_err(network_authority_error_to_failure)?
            .map(|tip| tip.height);
        let freshness = match (maybe_wallet_tip_height, maybe_chain_tip_height) {
            (_, None) => WalletFreshnessKind::Fresh,
            (Some(wallet_tip_height), Some(chain_tip_height))
                if wallet_tip_height >= chain_tip_height =>
            {
                WalletFreshnessKind::Fresh
            }
            _ => WalletFreshnessKind::Partial,
        };

        Ok(WalletFreshnessView {
            scanning: false,
            freshness,
            maybe_scanned_through_height: maybe_wallet_tip_height,
            maybe_target_height: maybe_chain_tip_height,
            maybe_next_height: maybe_wallet_tip_height.map(|height| height.saturating_add(1)),
        })
    }

    pub fn rescan_wallet_range(
        &mut self,
        maybe_start_height: Option<u32>,
        maybe_stop_height: Option<u32>,
    ) -> Result<WalletRescanExecution, RpcFailure> {
        self.rescan_wallet_range_with(
            maybe_start_height,
            maybe_stop_height,
            guard_durable_wallet_rescan_authority,
            prepare_durable_wallet_rescan,
            save_failed_rescan_job,
        )
    }

    pub(super) fn rescan_wallet_range_with(
        &mut self,
        maybe_start_height: Option<u32>,
        maybe_stop_height: Option<u32>,
        guard: impl FnOnce(&FjallNodeStore) -> Result<(), WalletRescanEligibilityFailure>,
        prepare: impl FnOnce(
            &FjallNodeStore,
            &Wallet,
            &ChainstateSnapshot,
            u32,
            u32,
        ) -> Result<PreparedWalletRescan, WalletRescanEligibilityFailure>,
        mut save_failed: impl FnMut(
            &mut WalletRegistry,
            &FjallNodeStore,
            WalletRescanJob,
        ) -> Result<(), WalletRegistryError>,
    ) -> Result<WalletRescanExecution, RpcFailure> {
        // Resolve the persisted checkpoint before any fallible authority read.
        let mut maybe_durable = match &self.wallet_state {
            WalletState::Local(_) => None,
            WalletState::DurableNamedRegistry { store, .. } => {
                let registry = load_wallet_registry(store)?;
                let name = resolve_selected_wallet_name(self.request_wallet_name(), &registry)?;
                let wallet = registry
                    .wallet(&name)
                    .map_err(wallet_registry_error_to_failure)?;
                Some((store.clone(), registry, name, wallet))
            }
        };
        if let Some((store, registry, name, _)) = &mut maybe_durable
            && let Err(failure) = guard(store)
        {
            fail_existing_job(
                registry,
                store,
                name,
                failure.safe_detail(),
                &mut save_failed,
            )?;
            return Err(eligibility_error_to_failure(&failure));
        }
        // Keep the manager admission snapshot, including its unflushed coins overlay.
        let snapshot = match self.blockchain_snapshot() {
            Ok(snapshot) => snapshot,
            Err(error) => {
                if let Some((store, registry, name, _)) = &mut maybe_durable {
                    fail_existing_job(
                        registry,
                        store,
                        name,
                        "wallet rescan authority: unavailable".to_string(),
                        &mut save_failed,
                    )?;
                }
                return Err(network_authority_error_to_failure(error));
            }
        };
        let tip_height = snapshot.tip().map_or(0, |tip| tip.height);
        let maybe_wallet_tip = match maybe_durable.as_ref() {
            Some((_, _, _, wallet)) => wallet.snapshot().maybe_tip_height,
            None => self.wallet_snapshot()?.maybe_tip_height,
        };
        let start_height = maybe_start_height
            .unwrap_or_else(|| maybe_wallet_tip.map_or(0, |height| height.saturating_add(1)));
        let stop_height = maybe_stop_height.unwrap_or(tip_height);
        if start_height > stop_height {
            return Err(RpcFailure::invalid_params(
                "rescanblockchain start_height must be less than or equal to stop_height",
            ));
        }
        if stop_height > tip_height {
            return Err(RpcFailure::invalid_params(
                "rescanblockchain stop_height exceeds the active chain tip",
            ));
        }

        if let Some((store, mut registry, wallet_name, wallet)) = maybe_durable {
            let Some(target_tip_hash) = snapshot
                .active_chain
                .iter()
                .find(|position| position.height == stop_height)
                .map(|position| position.block_hash)
            else {
                let failure = WalletRescanEligibilityFailure {
                    boundary: WalletRescanEligibilityBoundary::Requested,
                    maybe_height: Some(stop_height),
                    maybe_block_hash: None,
                    error: StorageError::UnavailableNamespace {
                        namespace: StorageNamespace::BlockIndex,
                    }
                    .into(),
                };
                fail_existing_job(
                    &mut registry,
                    &store,
                    &wallet_name,
                    failure.safe_detail(),
                    &mut save_failed,
                )?;
                return Err(eligibility_error_to_failure(&failure));
            };
            let mut job = pending_rescan_job(
                &registry,
                &wallet_name,
                &wallet,
                target_tip_hash,
                stop_height,
            )
            .map_err(wallet_registry_error_to_failure)?;
            registry
                .save_rescan_job(&store, job.clone(), PersistMode::Sync)
                .map_err(wallet_registry_error_to_failure)?;
            let prepared = match prepare(&store, &wallet, &snapshot, start_height, stop_height) {
                Ok(prepared) => prepared,
                Err(failure) => {
                    job.mark_failed(failure.safe_detail());
                    save_failed(&mut registry, &store, job)
                        .map_err(wallet_registry_error_to_failure)?;
                    return Err(eligibility_error_to_failure(&failure));
                }
            };
            registry
                .save_wallet(&store, &wallet_name, &prepared.wallet, PersistMode::Sync)
                .map_err(wallet_registry_error_to_failure)?;
            job.mark_chunk_progress(stop_height, prepared.maybe_tip_median_time_past);
            registry
                .save_rescan_job(&store, job, PersistMode::Sync)
                .map_err(wallet_registry_error_to_failure)?;
        } else if let WalletState::Local(wallet) = &mut self.wallet_state {
            wallet
                .rescan_chainstate(&partial_chainstate_snapshot(&snapshot, stop_height))
                .map_err(wallet_error_to_failure)?;
        }

        let freshness = WalletFreshnessView {
            scanning: false,
            freshness: if stop_height < tip_height {
                WalletFreshnessKind::Partial
            } else {
                WalletFreshnessKind::Fresh
            },
            maybe_scanned_through_height: Some(stop_height),
            maybe_target_height: Some(tip_height),
            maybe_next_height: Some(stop_height.saturating_add(1)),
        };

        Ok(WalletRescanExecution {
            start_height,
            stop_height,
            freshness,
        })
    }
}

fn pending_rescan_job(
    registry: &WalletRegistry,
    name: &str,
    wallet: &Wallet,
    target_tip_hash: BlockHash,
    target_tip_height: u32,
) -> Result<WalletRescanJob, WalletRegistryError> {
    let mut job = if let Some(job) = registry.rescan_job(name).cloned() {
        job
    } else {
        let prior = wallet.snapshot();
        let mut job = WalletRescanJob::new(
            name,
            target_tip_hash,
            target_tip_height,
            prior
                .maybe_tip_height
                .map_or(0, |height| height.saturating_add(1)),
            prior.maybe_tip_height,
        )?;
        job.maybe_tip_median_time_past = prior.maybe_tip_median_time_past;
        job
    };
    job.target_tip_hash = target_tip_hash;
    job.target_tip_height = target_tip_height;
    job.state = WalletRescanJobState::Pending;
    job.maybe_error = None;
    // Job state supplies activity; freshness describes the retained checkpoint.
    job.freshness = match job.maybe_scanned_through_height {
        None => WalletRescanFreshness::Scanning,
        Some(height) if height >= target_tip_height => WalletRescanFreshness::Fresh,
        Some(_) => WalletRescanFreshness::Partial,
    };
    Ok(job)
}

fn save_failed_rescan_job(
    registry: &mut WalletRegistry,
    store: &FjallNodeStore,
    job: WalletRescanJob,
) -> Result<(), WalletRegistryError> {
    registry.save_rescan_job(store, job, PersistMode::Sync)
}

fn fail_existing_job(
    registry: &mut WalletRegistry,
    store: &FjallNodeStore,
    name: &str,
    detail: String,
    save_failed: &mut impl FnMut(
        &mut WalletRegistry,
        &FjallNodeStore,
        WalletRescanJob,
    ) -> Result<(), WalletRegistryError>,
) -> Result<(), RpcFailure> {
    if let Some(mut job) = registry.rescan_job(name).cloned() {
        job.mark_failed(detail);
        save_failed(registry, store, job).map_err(wallet_registry_error_to_failure)?;
    }
    Ok(())
}

fn eligibility_error_to_failure(failure: &WalletRescanEligibilityFailure) -> RpcFailure {
    let detail = failure.safe_detail();
    if matches!(
        failure.error,
        WalletRegistryError::Storage(StorageError::UnavailableNamespace { .. })
    ) {
        return RpcFailure::wallet_error(format!("missing block payload: {detail}"));
    }
    RpcFailure::wallet_error(detail)
}

pub(super) fn partial_chainstate_snapshot(
    snapshot: &ChainstateSnapshot,
    through_height: u32,
) -> ChainstateSnapshot {
    let active_chain = snapshot
        .active_chain
        .iter()
        .filter(|position| position.height <= through_height)
        .cloned()
        .collect::<Vec<_>>();
    let active_hashes = active_chain
        .iter()
        .map(|position| position.block_hash)
        .collect::<BTreeSet<_>>();
    let utxos = snapshot
        .utxos
        .iter()
        .filter(|(_, coin)| coin.created_height <= through_height)
        .map(|(outpoint, coin)| (outpoint.clone(), coin.clone()))
        .collect();
    let undo_by_block = snapshot
        .undo_by_block
        .iter()
        .filter(|(block_hash, _)| active_hashes.contains(block_hash))
        .map(|(block_hash, undo)| (*block_hash, undo.clone()))
        .collect();

    ChainstateSnapshot::new(active_chain, utxos, undo_by_block)
}
