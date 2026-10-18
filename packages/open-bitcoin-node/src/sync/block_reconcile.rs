// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net.cpp
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/headerssync.cpp
// - packages/bitcoin-knots/src/sync.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp

use std::collections::{BTreeMap, BTreeSet, VecDeque};

use open_bitcoin_core::{
    chainstate::{AnchoredBlock, ChainPosition, ChainstateError},
    consensus::block_hash,
    primitives::{BlockHash, InventoryType},
};
use open_bitcoin_mempool::{PolicyTime, ReorgLifecycleContext};
use open_bitcoin_network::{HeaderEntry, PeerId, WireNetworkMessage};

use super::{DurableSyncRuntime, SyncRuntimeError, tip, types::SyncReconcileProgress};
use crate::{
    ManagedNetworkAuthorityError, ManagedNetworkError, StorageError, StorageNamespace,
    StorageRecoveryAction, network::BlockConnectDisposition,
};

pub(super) fn validate_block_limits(runtime: &DurableSyncRuntime) -> Result<(), SyncRuntimeError> {
    if runtime.config.max_blocks_in_flight_per_peer == 0 {
        return Err(SyncRuntimeError::ResourceLimit {
            message: "max_blocks_in_flight_per_peer is 0; increase the per-peer block budget to continue sync".to_string(),
        });
    }
    if runtime.config.max_blocks_in_flight_total == 0 {
        return Err(SyncRuntimeError::ResourceLimit {
            message:
                "max_blocks_in_flight_total is 0; increase the global block budget to continue sync"
                    .to_string(),
        });
    }
    Ok(())
}

pub(super) fn request_missing_blocks(
    runtime: &mut DurableSyncRuntime,
    peer_id: PeerId,
) -> Result<Vec<WireNetworkMessage>, SyncRuntimeError> {
    let available_global = runtime
        .config
        .max_blocks_in_flight_total
        .saturating_sub(runtime.inflight_blocks.len());
    if available_global == 0 {
        return Ok(Vec::new());
    }

    let active_chain_hashes = runtime
        .network
        .chainstate_snapshot()?
        .active_chain
        .into_iter()
        .map(|position| position.block_hash)
        .collect::<BTreeSet<_>>();
    let mut requested = Vec::new();
    for entry in runtime
        .network
        .requestable_header_entries(runtime.consensus_params)?
    {
        if requested.len() >= available_global
            || active_chain_hashes.contains(&entry.block_hash)
            || runtime.inflight_blocks.contains(&entry.block_hash)
        {
            continue;
        }
        if runtime.store.load_block(entry.block_hash)?.is_some() {
            runtime.network.note_local_block_hash(entry.block_hash)?;
            continue;
        }
        requested.push(entry.block_hash);
    }

    request_tracked_blocks(runtime, peer_id, &requested)
}

pub(super) fn request_tracked_blocks(
    runtime: &mut DurableSyncRuntime,
    peer_id: PeerId,
    block_hashes: &[BlockHash],
) -> Result<Vec<WireNetworkMessage>, SyncRuntimeError> {
    let available_global = runtime
        .config
        .max_blocks_in_flight_total
        .saturating_sub(runtime.inflight_blocks.len());
    let requested = block_hashes
        .iter()
        .copied()
        .filter(|block_hash| !runtime.inflight_blocks.contains(block_hash))
        .take(available_global)
        .collect::<Vec<_>>();
    let outbound = runtime
        .network
        .request_missing_blocks(peer_id, &requested)?;
    for message in &outbound {
        if let WireNetworkMessage::GetData(inventory) = message {
            for item in &inventory.inventory {
                if matches!(
                    item.inventory_type,
                    InventoryType::Block | InventoryType::WitnessBlock
                ) {
                    runtime
                        .inflight_blocks
                        .insert(BlockHash::from(item.object_hash));
                }
            }
        }
    }
    Ok(outbound)
}

pub(super) fn release_inflight_for_message(
    runtime: &mut DurableSyncRuntime,
    message: &WireNetworkMessage,
) {
    match message {
        WireNetworkMessage::Block(block) => {
            runtime.inflight_blocks.remove(&block_hash(&block.header));
        }
        WireNetworkMessage::NotFound(inventory) => {
            for item in &inventory.inventory {
                if matches!(
                    item.inventory_type,
                    InventoryType::Block | InventoryType::WitnessBlock
                ) {
                    runtime
                        .inflight_blocks
                        .remove(&BlockHash::from(item.object_hash));
                }
            }
        }
        _ => {}
    }
}

pub(super) fn reconcile_and_persist_best_chain(
    runtime: &mut DurableSyncRuntime,
    timestamp: i64,
) -> Result<SyncReconcileProgress, SyncRuntimeError> {
    let progress = reconcile_best_chain_inner(runtime, timestamp, false)?;
    if progress.should_persist_progress() {
        runtime.persist_progress_and_dispatch_tip()?;
    }
    runtime.record_reconcile_progress(progress.clone());
    Ok(progress)
}

#[cfg(test)]
pub(super) fn reconcile_best_chain(
    runtime: &mut DurableSyncRuntime,
    timestamp: i64,
) -> Result<SyncReconcileProgress, SyncRuntimeError> {
    reconcile_best_chain_inner(runtime, timestamp, false)
}

pub(super) fn reconcile_best_chain_for_live_session(
    runtime: &mut DurableSyncRuntime,
    timestamp: i64,
) -> Result<SyncReconcileProgress, SyncRuntimeError> {
    reconcile_best_chain_inner(runtime, timestamp, true)
}

fn reconcile_best_chain_inner(
    runtime: &mut DurableSyncRuntime,
    timestamp: i64,
    queue_durable_tip: bool,
) -> Result<SyncReconcileProgress, SyncRuntimeError> {
    let mut maybe_error = readmit_cached_bodies(runtime, timestamp)?;
    let active_chain = runtime.network.chainstate_snapshot()?.active_chain;
    let entries: BTreeMap<_, _> = runtime
        .network
        .header_entries()?
        .into_iter()
        .map(|entry| (entry.block_hash, entry))
        .collect();
    let mut candidates: VecDeque<_> = runtime.network.eligible_body_candidates()?.into();
    let mut attempted = BTreeSet::new();
    while let Some(candidate) = candidates.pop_front() {
        attempted.insert(candidate.block_hash);
        if active_chain.last().is_some_and(|tip| {
            candidate.block_hash == tip.block_hash || candidate.chain_work < tip.chain_work
        }) {
            continue;
        }
        let Some((common_prefix_len, suffix)) =
            candidate_suffix(candidate.block_hash, &entries, &active_chain)
        else {
            continue;
        };
        let result = activate_candidate(
            runtime,
            timestamp,
            queue_durable_tip,
            &active_chain,
            common_prefix_len,
            &suffix,
        );
        match result {
            Ok(progress) if progress.should_persist_progress() => return Ok(progress),
            Ok(_) => {}
            Err(ActivationFailure::Invalid(error)) => {
                if maybe_error.is_none() {
                    maybe_error = Some(error);
                }
                // The exact failed body revoked eligibility; obtain current candidates/grants.
                candidates = runtime
                    .network
                    .eligible_body_candidates()?
                    .into_iter()
                    .filter(|facts| !attempted.contains(&facts.block_hash))
                    .collect();
            }
            Err(ActivationFailure::Fatal(error)) => return Err(error),
        }
    }
    if let Some(error) = maybe_error {
        return Err(error);
    }
    waiting_progress(runtime, &active_chain)
}

enum ActivationFailure {
    Invalid(SyncRuntimeError),
    Fatal(SyncRuntimeError),
}

impl From<StorageError> for ActivationFailure {
    fn from(error: StorageError) -> Self {
        Self::Fatal(error.into())
    }
}

impl From<ManagedNetworkAuthorityError> for ActivationFailure {
    fn from(error: ManagedNetworkAuthorityError) -> Self {
        Self::Fatal(error.into())
    }
}

fn body_consensus_failure(error: &ManagedNetworkAuthorityError) -> bool {
    matches!(error, ManagedNetworkAuthorityError::Operation(ManagedNetworkError::Chainstate(error))
        if consensus_cause(error))
}

fn consensus_cause(error: &ChainstateError) -> bool {
    matches!(
        error,
        ChainstateError::BlockValidation { .. }
            | ChainstateError::TransactionValidation { .. }
            | ChainstateError::MissingCoin { .. }
            | ChainstateError::OutputOverwrite { .. }
    )
}

fn candidate_suffix(
    hash: BlockHash,
    entries: &BTreeMap<BlockHash, HeaderEntry>,
    active: &[ChainPosition],
) -> Option<(usize, Vec<HeaderEntry>)> {
    let mut current = hash;
    let mut suffix = Vec::new();
    for _ in 0..=entries.len() {
        if let Some(index) = active
            .iter()
            .position(|position| position.block_hash == current)
        {
            suffix.reverse();
            return Some((index + 1, suffix));
        }
        let entry = entries.get(&current)?;
        suffix.push(entry.clone());
        current = entry.header.previous_block_hash;
        if current == BlockHash::default() && active.is_empty() {
            suffix.reverse();
            return Some((0, suffix));
        }
    }
    None
}

fn activate_candidate(
    runtime: &mut DurableSyncRuntime,
    timestamp: i64,
    queue_durable_tip: bool,
    active_chain: &[ChainPosition],
    common_prefix_len: usize,
    suffix: &[HeaderEntry],
) -> Result<SyncReconcileProgress, ActivationFailure> {
    if common_prefix_len == active_chain.len() {
        return activate_extension(runtime, timestamp, queue_durable_tip, suffix);
    }

    let mut replacement_branch = Vec::new();
    let mut maybe_first_missing = None;
    let mut missing_count = 0_u64;
    for entry in suffix {
        let Some(block) = runtime.store.load_block(entry.block_hash)? else {
            missing_count = missing_count.saturating_add(1);
            if maybe_first_missing.is_none() {
                maybe_first_missing = Some((entry.height, entry.block_hash));
            }
            continue;
        };
        runtime.network.note_local_block_hash(entry.block_hash)?;
        replacement_branch.push(AnchoredBlock {
            block,
            chain_work: entry.chain_work,
        });
    }
    if let Some((first_missing_height, first_missing_hash)) = maybe_first_missing {
        return Ok(SyncReconcileProgress::BranchCompetitionAwaitingBodies {
            missing_count,
            first_missing_height: u64::from(first_missing_height),
            first_missing_hash: tip::block_hash_hex(first_missing_hash),
        });
    }
    if replacement_branch.is_empty() {
        return Ok(SyncReconcileProgress::NoChange);
    }

    let mut disconnect_blocks = Vec::new();
    for position in active_chain.iter().skip(common_prefix_len).rev() {
        let Some(block) = runtime.store.load_block(position.block_hash)? else {
            return Err(ActivationFailure::Fatal(SyncRuntimeError::Storage(
                crate::StorageError::Corruption {
                    namespace: StorageNamespace::BlockIndex,
                    detail: format!(
                        "missing durable block body for active chain block {:?}",
                        position.block_hash
                    ),
                    action: StorageRecoveryAction::Repair,
                },
            )));
        };
        disconnect_blocks.push(block);
    }

    let transition = runtime.network.reorg_to_branch(
        &disconnect_blocks,
        &replacement_branch,
        reorg_lifecycle_context(timestamp),
        runtime.verify_flags,
        runtime.consensus_params,
    );
    let transition = match transition {
        Ok(transition) => transition,
        Err(ManagedNetworkAuthorityError::Operation(ManagedNetworkError::Chainstate(
            ChainstateError::BranchNotPreferred,
        ))) => return Ok(SyncReconcileProgress::SideBranchPreserved),
        Err(error) => {
            let invalid = matches!(&error, ManagedNetworkAuthorityError::Operation(
                ManagedNetworkError::Chainstate(ChainstateError::ReplacementBlock { source, .. }))
                if consensus_cause(source));
            let error = reorg_runtime_error(error);
            return Err(if invalid {
                ActivationFailure::Invalid(error)
            } else {
                ActivationFailure::Fatal(error)
            });
        }
    };
    let common_ancestor = active_chain
        .get(common_prefix_len.saturating_sub(1))
        .or_else(|| active_chain.first());
    let Some(common_ancestor) = common_ancestor else {
        return Ok(SyncReconcileProgress::NoChange);
    };
    let Some(final_active_tip) = runtime.network.maybe_chain_tip()? else {
        return Ok(SyncReconcileProgress::NoChange);
    };
    if queue_durable_tip && let Some(final_block) = replacement_branch.last() {
        runtime.queue_durable_tip_advanced(final_block.block.clone());
    }

    Ok(SyncReconcileProgress::ReorgPersisted(reorg_evidence(
        common_ancestor,
        &transition,
        &final_active_tip,
    )))
}

fn activate_extension(
    runtime: &mut DurableSyncRuntime,
    timestamp: i64,
    queue_durable_tip: bool,
    suffix: &[HeaderEntry],
) -> Result<SyncReconcileProgress, ActivationFailure> {
    let mut connected_count = 0_u64;
    let mut maybe_final_connected_block = None;
    for entry in suffix {
        let Some(block) = runtime.store.load_block(entry.block_hash)? else {
            break;
        };
        runtime.network.note_local_block_hash(entry.block_hash)?;
        let disposition = runtime.network.connect_stored_block(
            &block,
            entry.chain_work,
            timestamp,
            runtime.verify_flags,
            runtime.consensus_params,
        );
        let disposition = match disposition {
            Ok(disposition) => disposition,
            Err(error) if body_consensus_failure(&error) => {
                if connected_count == 0 {
                    return Err(ActivationFailure::Invalid(error.into()));
                }
                // Report the actually accepted prefix, then reacquire ancestry next turn.
                break;
            }
            Err(error) => return Err(error.into()),
        };
        if matches!(disposition, BlockConnectDisposition::Connected(_)) {
            connected_count = connected_count.saturating_add(1);
            maybe_final_connected_block = Some(block);
            continue;
        }
        break;
    }
    if connected_count > 0 {
        if queue_durable_tip && let Some(block) = maybe_final_connected_block {
            runtime.queue_durable_tip_advanced(block);
        }
        return Ok(SyncReconcileProgress::ExtendedActiveChain { connected_count });
    }
    Ok(SyncReconcileProgress::NoChange)
}

fn waiting_progress(
    runtime: &DurableSyncRuntime,
    active: &[ChainPosition],
) -> Result<SyncReconcileProgress, SyncRuntimeError> {
    let best = runtime.network.best_chain_entries()?;
    let mut missing = Vec::new();
    let mut has_side = false;
    for entry in best {
        if active
            .iter()
            .any(|position| position.block_hash == entry.block_hash)
        {
            continue;
        }
        has_side = true;
        if runtime.store.load_block(entry.block_hash)?.is_none() {
            missing.push(entry);
        }
    }
    if let Some(first) = missing.first() {
        return Ok(SyncReconcileProgress::BranchCompetitionAwaitingBodies {
            missing_count: missing.len() as u64,
            first_missing_height: u64::from(first.height),
            first_missing_hash: tip::block_hash_hex(first.block_hash),
        });
    }
    Ok(if has_side {
        SyncReconcileProgress::SideBranchPreserved
    } else {
        SyncReconcileProgress::NoChange
    })
}

fn readmit_cached_bodies(
    runtime: &DurableSyncRuntime,
    timestamp: i64,
) -> Result<Option<SyncRuntimeError>, SyncRuntimeError> {
    // This traversal supplies ancestry for validation, never historical arrival order.
    let mut entries = runtime
        .network
        .requestable_header_entries(runtime.consensus_params)?;
    entries.sort_by_key(|entry| entry.height);
    let mut maybe_error = None;
    for entry in entries {
        let maybe_facts = runtime.network.maybe_body_eligibility(entry.block_hash)?;
        let maybe_block = runtime.store.load_block(entry.block_hash)?;
        if maybe_block.is_none() {
            runtime.network.mark_body_unavailable(entry.block_hash)?;
            continue;
        }
        if maybe_facts.is_some() {
            continue;
        }
        if let Some(block) = maybe_block {
            match runtime.network.retain_cached_candidate(
                &block,
                runtime.consensus_params,
                timestamp,
            ) {
                Ok(()) => {}
                Err(error) if body_consensus_failure(&error) => {
                    if maybe_error.is_none() {
                        maybe_error = Some(error.into());
                    }
                }
                Err(error) => return Err(error.into()),
            }
        }
    }
    Ok(maybe_error)
}

pub(super) const fn reorg_lifecycle_context(timestamp: i64) -> ReorgLifecycleContext {
    ReorgLifecycleContext::new(PolicyTime::from_unix_seconds(timestamp))
}

fn reorg_runtime_error(error: ManagedNetworkAuthorityError) -> SyncRuntimeError {
    match error {
        ManagedNetworkAuthorityError::Operation(ManagedNetworkError::Chainstate(
            ChainstateError::CoinsStorage { detail },
        )) if detail.contains("BASIC required") || detail.contains("BASIC genuine historical") => {
            SyncRuntimeError::Storage(StorageError::Corruption {
                namespace: StorageNamespace::Chainstate,
                detail,
                action: StorageRecoveryAction::Repair,
            })
        }
        ManagedNetworkAuthorityError::Operation(ManagedNetworkError::Chainstate(
            ChainstateError::MissingUndo { block_hash },
        )) => SyncRuntimeError::Storage(StorageError::Corruption {
            namespace: StorageNamespace::Chainstate,
            detail: format!("missing undo data for active chain block {block_hash:?}"),
            action: StorageRecoveryAction::Repair,
        }),
        other => other.into(),
    }
}

fn reorg_evidence(
    common_ancestor: &ChainPosition,
    transition: &open_bitcoin_core::chainstate::ChainTransition,
    final_active_tip: &ChainPosition,
) -> crate::status::SyncReorgEvidence {
    crate::status::SyncReorgEvidence {
        common_ancestor_height: u64::from(common_ancestor.height),
        common_ancestor_hash: tip::block_hash_hex(common_ancestor.block_hash),
        disconnected_count: transition.disconnected.len() as u64,
        connected_count: transition.connected.len() as u64,
        final_active_height: u64::from(final_active_tip.height),
        final_active_hash: tip::block_hash_hex(final_active_tip.block_hash),
        fully_persisted: true,
    }
}

#[cfg(test)]
#[path = "tests/phase158_preflight.rs"]
mod phase158_tests;
