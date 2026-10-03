// Parity breadcrumbs:
// - packages/bitcoin-knots/src/wallet/wallet.cpp
// - packages/bitcoin-knots/src/wallet/rpc/transactions.cpp
// - packages/bitcoin-knots/src/coins.cpp

//! Staged full-wallet replacement with durable payload eligibility.

use std::collections::{BTreeMap, BTreeSet};

use open_bitcoin_core::{chainstate::ChainstateSnapshot, primitives::BlockHash, wallet::Wallet};

use super::WalletRegistryError;
use crate::{FjallNodeStore, StorageError, StorageNamespace};

/// The stage at which replacement eligibility was refused.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WalletRescanEligibilityBoundary {
    Authority,
    Selection,
    Requested,
    Creating,
}

/// Original error and safe, known facts about a refused staged replacement.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WalletRescanEligibilityFailure {
    pub boundary: WalletRescanEligibilityBoundary,
    pub maybe_height: Option<u32>,
    pub maybe_block_hash: Option<BlockHash>,
    pub error: WalletRegistryError,
}

impl WalletRescanEligibilityFailure {
    /// Wraps an authority/registry error without exposing its diagnostic text.
    pub fn authority(error: WalletRegistryError) -> Self {
        Self {
            boundary: WalletRescanEligibilityBoundary::Authority,
            maybe_height: None,
            maybe_block_hash: None,
            error,
        }
    }

    /// Returns durable evidence that never includes backend paths or error text.
    pub fn safe_detail(&self) -> String {
        let boundary = match self.boundary {
            WalletRescanEligibilityBoundary::Authority => "authority",
            WalletRescanEligibilityBoundary::Selection => "selection",
            WalletRescanEligibilityBoundary::Requested => "requested",
            WalletRescanEligibilityBoundary::Creating => "creating",
        };
        let category = match &self.error {
            WalletRegistryError::Storage(error) => match error {
                StorageError::InvalidSchemaVersion { .. } => "InvalidSchemaVersion",
                StorageError::SchemaMismatch { .. } => "SchemaMismatch",
                StorageError::Corruption { .. } => "Corruption",
                StorageError::RecoveryMarkerCorruption { .. } => "RecoveryMarkerCorruption",
                StorageError::UnavailableNamespace { .. } => "UnavailableNamespace",
                StorageError::InterruptedWrite { .. } => "InterruptedWrite",
                StorageError::BackendFailure { .. } => "BackendFailure",
            },
            WalletRegistryError::Wallet(_) => "Wallet",
            WalletRegistryError::DuplicateWalletName(_) => "DuplicateWalletName",
            WalletRegistryError::UnknownWallet(_) => "UnknownWallet",
            WalletRegistryError::StaleSelection(_) => "StaleSelection",
        };
        let mut detail = format!("wallet rescan {boundary}: {category}");
        if let Some(height) = self.maybe_height {
            detail.push_str(&format!(" height {height}"));
        }
        if let Some(hash) = self.maybe_block_hash {
            detail.push_str(&format!(" hash {hash:?}"));
        }
        detail
    }
}

/// A replacement wallet whose required payloads have all been checked.
#[derive(Debug)]
pub struct PreparedWalletRescan {
    pub wallet: Wallet,
    pub through_height: u32,
    pub maybe_tip_median_time_past: Option<i64>,
}

/// Refuses interrupted durable coins markers without repairing or loading leftovers.
pub fn guard_durable_wallet_rescan_authority(
    store: &FjallNodeStore,
) -> Result<(), WalletRescanEligibilityFailure> {
    store
        .check_wallet_scan_authority()
        .map_err(|error| WalletRescanEligibilityFailure::authority(error.into()))
}

/// Prepares a full replacement from adapter authority, checking durable payloads.
///
/// Presence probes and saving are separate operations, not a storage transaction.
pub fn prepare_durable_wallet_rescan(
    store: &FjallNodeStore,
    prior_wallet: &Wallet,
    authority: &ChainstateSnapshot,
    start_height: u32,
    through_height: u32,
) -> Result<PreparedWalletRescan, WalletRescanEligibilityFailure> {
    guard_durable_wallet_rescan_authority(store)?;
    prepare_wallet_rescan_with_probe(
        prior_wallet,
        authority,
        start_height,
        through_height,
        |hash| store.has_block(hash),
    )
}

/// Stages pure wallet selection and checks requested and selected creating heights.
///
/// Callers supplying a probe must validate their authority markers first. An empty
/// requested interval still checks every selected replacement entry.
pub fn prepare_wallet_rescan_with_probe(
    prior_wallet: &Wallet,
    authority: &ChainstateSnapshot,
    start_height: u32,
    through_height: u32,
    mut probe: impl FnMut(BlockHash) -> Result<bool, StorageError>,
) -> Result<PreparedWalletRescan, WalletRescanEligibilityFailure> {
    let snapshot = partial_chainstate_snapshot(authority, through_height);
    let mut wallet = Wallet::from_snapshot(prior_wallet.snapshot());
    wallet
        .rescan_chainstate(&snapshot)
        .map_err(|error| WalletRescanEligibilityFailure {
            boundary: WalletRescanEligibilityBoundary::Selection,
            maybe_height: None,
            maybe_block_hash: None,
            error: error.into(),
        })?;
    let mut required = BTreeMap::new();
    for height in start_height..=through_height {
        required.insert(height, WalletRescanEligibilityBoundary::Requested);
    }
    for coin in wallet.utxos() {
        required.insert(
            coin.created_height,
            WalletRescanEligibilityBoundary::Creating,
        );
    }
    let mut probed = BTreeSet::new();
    for (height, boundary) in required {
        let maybe_hash = snapshot
            .active_chain
            .iter()
            .find(|position| position.height == height)
            .map(|position| position.block_hash);
        let failure = |error| WalletRescanEligibilityFailure {
            boundary,
            maybe_height: Some(height),
            maybe_block_hash: maybe_hash,
            error: WalletRegistryError::Storage(error),
        };
        let hash = maybe_hash.ok_or_else(|| failure(unavailable_payload()))?;
        if !probed.insert(hash) {
            continue;
        }
        if !probe(hash).map_err(failure)? {
            return Err(failure(unavailable_payload()));
        }
    }
    Ok(PreparedWalletRescan {
        maybe_tip_median_time_past: snapshot.tip().map(|tip| tip.median_time_past),
        wallet,
        through_height,
    })
}

fn unavailable_payload() -> StorageError {
    StorageError::UnavailableNamespace {
        namespace: StorageNamespace::BlockIndex,
    }
}

fn partial_chainstate_snapshot(
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
        .filter(|(hash, _)| active_hashes.contains(hash))
        .map(|(hash, undo)| (*hash, undo.clone()))
        .collect();
    ChainstateSnapshot::new(active_chain, utxos, undo_by_block)
}

#[cfg(test)]
mod tests;
