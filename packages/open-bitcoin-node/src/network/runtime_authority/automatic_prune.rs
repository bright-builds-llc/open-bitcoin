// Parity breadcrumbs:
// - packages/bitcoin-knots/src/node/blockstorage.cpp
// - packages/bitcoin-knots/src/validation.cpp

//! Measured retention on the existing serialized flush owner.

use super::prune_flush::flush_and_evict_pruned_blocks;
use crate::chainstate::{FlushExecution, FlushPersistSink, PruneProtectionSnapshot};
use crate::storage::{
    StorageError, StorageNamespace, StorageRecoveryAction, fjall_store::PayloadUsageRevision,
};
use crate::{ChainstateStore, ManagedPeerNetwork};
use open_bitcoin_core::{
    chainstate::{
        AutomaticPruneInput, ChainPosition, CoinsView, FlushMode, FlushPolicyTime, PruneLockInfo,
        PruneMode, PrunePlan, plan_automatic_prune,
    },
    primitives::BlockHash,
};
use std::sync::Mutex;

/// Exact scans are coalesced independently of the coins checkpoint timer.
const PERIODIC_MEASUREMENT_INTERVAL_SECONDS: u64 = 60;

#[derive(Debug, Clone, PartialEq, Eq)]
struct MeasurementKey {
    revision: PayloadUsageRevision,
    tip_height: u32,
    tip_hash: BlockHash,
    mode: PruneMode,
    protection: PruneProtectionSnapshot,
}

pub(super) struct AutomaticPruneState {
    pub(super) prune_after_height: u32,
    maybe_completed_key: Option<MeasurementKey>,
    maybe_last_measurement_seconds: Option<u64>,
    maybe_last_measured_protection_identity: Option<PruneProtectionSnapshot>,
}

impl Default for AutomaticPruneState {
    fn default() -> Self {
        Self {
            prune_after_height: crate::SyncNetwork::Mainnet.prune_after_height(),
            maybe_completed_key: None,
            maybe_last_measurement_seconds: None,
            maybe_last_measured_protection_identity: None,
        }
    }
}

impl AutomaticPruneState {
    pub(super) fn set_network(&mut self, network: crate::SyncNetwork) {
        self.prune_after_height = network.prune_after_height();
        self.invalidate();
    }

    fn invalidate(&mut self) {
        self.maybe_completed_key = None;
        self.maybe_last_measurement_seconds = None;
        self.maybe_last_measured_protection_identity = None;
    }
}

pub(super) fn flush<S: ChainstateStore, V: CoinsView>(
    network: &mut ManagedPeerNetwork<S, V>,
    state: &Mutex<AutomaticPruneState>,
    mode: FlushMode,
    now: FlushPolicyTime,
    disk_free_bytes: u64,
) -> Result<FlushExecution, StorageError> {
    // Lock order is authority -> automatic state -> publication -> payload.
    // Storage calls release their guard before this invokes the nested flush effects.
    let mut state = state.lock().map_err(|_| state_error())?;
    let active_chain = network.chainstate().chainstate().active_chain();
    let prepared = prepare(
        network.chainstate().store(),
        active_chain,
        network.prune_mode(),
        mode,
        now,
        &mut state,
    );
    let (plan, locks, maybe_key) = match prepared {
        Ok(prepared) => prepared,
        Err(error) => {
            state.invalidate();
            return Err(error);
        }
    };
    let effective_mode = if plan.heights.is_empty() {
        mode
    } else {
        FlushMode::Always
    };
    let outcome =
        flush_and_evict_pruned_blocks(network, effective_mode, now, disk_free_bytes, &plan, &locks);
    if outcome.is_err() {
        state.invalidate();
        return outcome;
    }
    if let Some(key) = maybe_key {
        let current = network.chainstate().store().payload_usage_revision();
        state.maybe_completed_key = match current {
            Ok(revision) if revision.is_reusable() && revision == key.revision => Some(key),
            Ok(_) => None,
            Err(error) => {
                state.invalidate();
                return Err(error);
            }
        };
    }
    outcome
}

type PreparedPrune = (PrunePlan, Vec<PruneLockInfo>, Option<MeasurementKey>);

fn prepare<S: FlushPersistSink>(
    store: &S,
    active_chain: &[ChainPosition],
    prune_mode: PruneMode,
    mode: FlushMode,
    now: FlushPolicyTime,
    state: &mut AutomaticPruneState,
) -> Result<PreparedPrune, StorageError> {
    let empty = || (PrunePlan::default(), Vec::new(), None);
    if mode == FlushMode::None {
        return Ok(empty());
    }
    let PruneMode::Automatic { target_mib } = prune_mode else {
        return Ok(empty());
    };
    let Some(tip) = active_chain.last() else {
        return Ok(empty());
    };
    if tip.height <= state.prune_after_height || target_mib.checked_mul(1024 * 1024).is_none() {
        return Ok(empty());
    }
    let mut protection = store.load_prune_protection()?;
    protection.locks.sort_by(|left, right| {
        (&left.name, left.height_first, left.height_last).cmp(&(
            &right.name,
            right.height_first,
            right.height_last,
        ))
    });
    if state.maybe_last_measured_protection_identity.as_ref() != Some(&protection) {
        state.invalidate();
    }
    let locks = protection.locks().to_vec();
    let revision = store.payload_usage_revision()?;
    let key = MeasurementKey {
        revision,
        tip_height: tip.height,
        tip_hash: tip.block_hash,
        mode: prune_mode,
        protection: protection.clone(),
    };
    if mode != FlushMode::Always
        && revision.is_reusable()
        && state.maybe_completed_key.as_ref() == Some(&key)
    {
        return Ok(empty());
    }
    if mode == FlushMode::Periodic
        && state.maybe_last_measurement_seconds.is_some_and(|last| {
            now.unix_seconds().saturating_sub(last) < PERIODIC_MEASUREMENT_INTERVAL_SECONDS
        })
    {
        return Ok(empty());
    }
    state.maybe_completed_key = None;
    let mut usage = store.retained_payload_usage(active_chain)?;
    state.maybe_last_measurement_seconds = Some(now.unix_seconds());
    state.maybe_last_measured_protection_identity = Some(protection.clone());
    // The snapshot's captured revision is the only identity that belongs to these facts.
    if store.payload_usage_revision()? != usage.revision {
        return Ok(empty());
    }
    let key = MeasurementKey {
        revision: usage.revision,
        ..key
    };
    // Required bytes remain in total usage, but cannot pay the deletion budget.
    usage
        .height_sizes
        .retain(|height, _| !protection.protects_height(*height));
    let plan = plan_automatic_prune(&AutomaticPruneInput {
        tip: tip.height,
        prune_after_height: state.prune_after_height,
        mode: prune_mode,
        height_sizes: usage.height_sizes,
        current_usage_bytes: usage.current_usage_bytes,
        locks: locks.clone(),
    });
    Ok((plan, locks, Some(key)))
}

fn state_error() -> StorageError {
    StorageError::Corruption {
        namespace: StorageNamespace::BlockIndex,
        detail: "automatic prune state mutex poisoned".into(),
        action: StorageRecoveryAction::Repair,
    }
}

#[cfg(test)]
mod tests;
