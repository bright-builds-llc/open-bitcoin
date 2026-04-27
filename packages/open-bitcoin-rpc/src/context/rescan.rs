// Parity breadcrumbs:
// - packages/bitcoin-knots/src/bitcoind.cpp
// - packages/bitcoin-knots/src/rpc/protocol.h
// - packages/bitcoin-knots/src/rpc/request.cpp
// - packages/bitcoin-knots/src/rpc/server.cpp
// - packages/bitcoin-knots/src/rpc/blockchain.cpp
// - packages/bitcoin-knots/src/rpc/mempool.cpp
// - packages/bitcoin-knots/src/rpc/net.cpp
// - packages/bitcoin-knots/src/rpc/rawtransaction.cpp
// - packages/bitcoin-knots/test/functional/interface_rpc.py

use std::collections::BTreeSet;

use open_bitcoin_node::core::chainstate::ChainstateSnapshot;
use open_bitcoin_node::core::primitives::BlockHash;
use open_bitcoin_node::{
    PersistMode, WalletRescanFreshness, WalletRescanJob, WalletRescanJobState,
};

use crate::error::RpcFailure;

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

impl ManagedRpcContext {
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
        let maybe_chain_tip_height = self.maybe_chain_tip().map(|tip| tip.height);
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
        let snapshot = self.blockchain_snapshot();
        let tip_height = snapshot.tip().map_or(0, |tip| tip.height);
        let current_wallet_tip = self.wallet_snapshot()?.maybe_tip_height;
        let start_height = maybe_start_height
            .unwrap_or_else(|| current_wallet_tip.map_or(0, |height| height.saturating_add(1)));
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

        let partial_snapshot = partial_chainstate_snapshot(&snapshot, stop_height);
        let maybe_tip_median_time_past = partial_snapshot.tip().map(|tip| tip.median_time_past);

        match &mut self.wallet_state {
            WalletState::Local(wallet) => {
                wallet
                    .rescan_chainstate(&partial_snapshot)
                    .map_err(wallet_error_to_failure)?;
            }
            WalletState::DurableNamedRegistry {
                store,
                maybe_request_wallet_name,
            } => {
                let mut registry = load_wallet_registry(store)?;
                let wallet_name =
                    resolve_selected_wallet_name(maybe_request_wallet_name.as_deref(), &registry)?;
                let maybe_scanned_through_height = start_height.checked_sub(1);
                let target_tip_hash = partial_snapshot.tip().map_or_else(
                    || BlockHash::from_byte_array([0_u8; 32]),
                    |tip| tip.block_hash,
                );
                let mut job = WalletRescanJob::new(
                    wallet_name.clone(),
                    target_tip_hash,
                    stop_height,
                    start_height,
                    maybe_scanned_through_height,
                )
                .map_err(wallet_registry_error_to_failure)?;
                job.state = WalletRescanJobState::Pending;
                registry
                    .save_rescan_job(store, job.clone(), PersistMode::Sync)
                    .map_err(wallet_registry_error_to_failure)?;

                let mut wallet = registry
                    .wallet(&wallet_name)
                    .map_err(wallet_registry_error_to_failure)?;
                wallet
                    .rescan_chainstate(&partial_snapshot)
                    .map_err(wallet_error_to_failure)?;
                registry
                    .save_wallet(store, &wallet_name, &wallet, PersistMode::Sync)
                    .map_err(wallet_registry_error_to_failure)?;
                job.mark_chunk_progress(stop_height, maybe_tip_median_time_past);
                registry
                    .save_rescan_job(store, job, PersistMode::Sync)
                    .map_err(wallet_registry_error_to_failure)?;
            }
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
