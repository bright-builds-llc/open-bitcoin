// Parity breadcrumbs:
// - packages/bitcoin-knots/src/node/chainstate.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp
// - packages/bitcoin-knots/src/validation.cpp

//! Shell-owned CanFlush init and ordered `decide_flush` execution.

use std::path::Path;

use open_bitcoin_core::{
    chainstate::{
        BlockUndo, ChainPosition, ChainstateError, CoinsCache, CoinsView, FlushDecision, FlushMode,
        FlushPolicyInput, FlushPolicyTime, RecoveryDecision, decide_flush, decide_recovery,
    },
    primitives::{Block, BlockHash},
};
use open_bitcoin_network::HeaderEntry;

use super::replay_interrupted_flush;
use crate::status::{
    CoinsRecoveryOutcome, FieldAvailability, HaveBytesEvidence, ReadinessLabel,
    project_chainstate_durability,
};
use crate::storage::{
    FjallNodeStore, PersistMode, StorageError, StorageNamespace, StorageRecoveryAction,
    coins_view::FjallCoinsView,
};

#[cfg(test)]
mod tests;

/// Knots default `-dbcache` kernel budget (450 MiB).
pub const DEFAULT_KERNEL_CACHE_BYTES: u64 = 450 * 1024 * 1024;
/// Knots minimum dbcache (4 MiB).
pub const MIN_DBCACHE_BYTES: u64 = 4 * 1024 * 1024;
/// Knots coins-DB cache cap (8 MiB).
pub const COINS_DB_CACHE_CAP_BYTES: u64 = 8 * 1024 * 1024;

/// Knots leftover coins-cache budget: `max(4 MiB, 450 MiB − 8 MiB)` = 442 MiB.
pub const fn default_coins_cache_byte_limit() -> u64 {
    let leftover = DEFAULT_KERNEL_CACHE_BYTES.saturating_sub(COINS_DB_CACHE_CAP_BYTES);
    if leftover > MIN_DBCACHE_BYTES {
        leftover
    } else {
        MIN_DBCACHE_BYTES
    }
}

/// CanFlush-style readiness after the D-11 init sequence.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ManagerReadiness {
    NotReady,
    ReadyToFlush,
}

/// One owner for cache defaults, readiness, and ordered flush execution.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FlushLifecycle {
    readiness: ManagerReadiness,
    cache_byte_limit: u64,
    mempool_leftover_bytes: u64,
    next_write: FlushPolicyTime,
    memory_pressure: bool,
    maybe_last_write_decision: Option<FlushDecision>,
    maybe_recovery_outcome: Option<CoinsRecoveryOutcome>,
}

/// Outcome of one `execute_flush` call after `decide_flush`.
#[derive(Debug)]
pub struct FlushExecution {
    pub decision: FlushDecision,
    pub wrote_coins: bool,
}

/// Injected persist sink so undo/index abort tests do not chmod a Fjall datadir.
pub trait FlushPersistSink {
    fn persist_block(&mut self, block: &Block) -> Result<(), StorageError>;
    fn persist_undo(&mut self, hash: BlockHash, undo: &BlockUndo) -> Result<(), StorageError>;
    fn persist_header_entries(&mut self, entries: &[HeaderEntry]) -> Result<(), StorageError>;
    fn persist_chain_meta(&mut self, active_chain: &[ChainPosition]) -> Result<(), StorageError>;
    fn disk_free_bytes(&self) -> u64 {
        u64::MAX
    }
}

impl FlushPersistSink for FjallNodeStore {
    fn persist_block(&mut self, block: &Block) -> Result<(), StorageError> {
        FjallNodeStore::save_block(self, block, PersistMode::Flush).map(|_| ())
    }

    fn persist_undo(&mut self, hash: BlockHash, undo: &BlockUndo) -> Result<(), StorageError> {
        FjallNodeStore::save_undo(self, hash, undo, PersistMode::Flush)
    }

    fn persist_header_entries(&mut self, entries: &[HeaderEntry]) -> Result<(), StorageError> {
        if entries.is_empty() {
            return Ok(());
        }
        FjallNodeStore::save_header_entries(self, entries, PersistMode::Flush)
    }

    fn persist_chain_meta(&mut self, active_chain: &[ChainPosition]) -> Result<(), StorageError> {
        FjallNodeStore::save_chain_meta(self, active_chain, PersistMode::Flush)
    }

    fn disk_free_bytes(&self) -> u64 {
        probe_disk_free_bytes(self.datadir())
    }
}

/// Completes coins open → recover → empty cache → ReadyToFlush (D-11).
pub fn initialize(
    store: &FjallNodeStore,
    _now: FlushPolicyTime,
    next_write: FlushPolicyTime,
    mempool_leftover_bytes: u64,
    memory_pressure: bool,
    _disk_free_bytes: u64,
) -> Result<(FlushLifecycle, FjallCoinsView, CoinsCache<FjallCoinsView>), StorageError> {
    let mut lifecycle = FlushLifecycle {
        readiness: ManagerReadiness::NotReady,
        cache_byte_limit: default_coins_cache_byte_limit(),
        mempool_leftover_bytes,
        next_write,
        memory_pressure,
        maybe_last_write_decision: None,
        maybe_recovery_outcome: None,
    };
    let view = FjallCoinsView::from_store(store);
    let heads = view
        .head_blocks()
        .map_err(|error| annotate_fail_closed(map_chainstate(error), false))?;
    let decision = decide_recovery(heads.len());
    let recovered = apply_recovery_decision(store, view, decision).map_err(|error| {
        annotate_fail_closed(
            error,
            matches!(decision, RecoveryDecision::InterruptedTwoHeads),
        )
    })?;
    let maybe_best_block = recovered.best_block().map_err(map_chainstate)?;
    lifecycle.maybe_recovery_outcome = Some(coins_recovery_outcome_after_success(
        decision,
        maybe_best_block,
    )?);
    lifecycle.readiness = ManagerReadiness::ReadyToFlush;
    let cache = CoinsCache::from_parent(recovered);
    Ok((lifecycle, FjallCoinsView::from_store(store), cache))
}

impl FlushLifecycle {
    pub const fn readiness(&self) -> ManagerReadiness {
        self.readiness
    }

    pub const fn set_next_write(&mut self, next_write: FlushPolicyTime) {
        self.next_write = next_write;
    }

    /// Memory `from_store` / tests only. Production Fjall uses `initialize` (Plan 04 / 06).
    pub fn ready(
        _now: FlushPolicyTime,
        next_write: FlushPolicyTime,
        mempool_leftover_bytes: u64,
        memory_pressure: bool,
    ) -> Self {
        Self {
            readiness: ManagerReadiness::ReadyToFlush,
            cache_byte_limit: default_coins_cache_byte_limit(),
            mempool_leftover_bytes,
            next_write,
            memory_pressure,
            maybe_last_write_decision: None,
            maybe_recovery_outcome: Some(CoinsRecoveryOutcome::Consistent),
        }
    }

    pub const fn maybe_last_write_decision(&self) -> Option<FlushDecision> {
        self.maybe_last_write_decision
    }

    pub const fn maybe_recovery_outcome(&self) -> Option<CoinsRecoveryOutcome> {
        self.maybe_recovery_outcome
    }

    #[allow(clippy::too_many_arguments)]
    pub fn project_chainstate_durability(
        &self,
        estimated_cache_bytes: u64,
        cache_entry_count: u64,
        now: FlushPolicyTime,
        disk_free_bytes: u64,
        maybe_coins_best_block: Option<BlockHash>,
        maybe_coins_best_block_height: Option<u64>,
        have_bytes: HaveBytesEvidence,
    ) -> FieldAvailability<crate::status::ChainstateDurabilityEvidence> {
        project_chainstate_durability(
            self.maybe_last_write_decision,
            self.maybe_recovery_outcome,
            match self.readiness {
                ManagerReadiness::NotReady => ReadinessLabel::NotReady,
                ManagerReadiness::ReadyToFlush => ReadinessLabel::ReadyToFlush,
            },
            estimated_cache_bytes,
            self.cache_byte_limit,
            self.mempool_leftover_bytes,
            cache_entry_count,
            now,
            self.next_write,
            self.memory_pressure,
            disk_free_bytes,
            maybe_coins_best_block,
            maybe_coins_best_block_height,
            have_bytes,
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub fn execute_flush<V: CoinsView, S: FlushPersistSink>(
        &mut self,
        sink: &mut S,
        cache: &mut CoinsCache<V>,
        mode: FlushMode,
        now: FlushPolicyTime,
        disk_free_bytes: u64,
        undo_window: &[(BlockHash, BlockUndo)],
        header_entries: &[HeaderEntry],
        block_payloads: &[Block],
        active_chain: &[ChainPosition],
    ) -> Result<FlushExecution, StorageError> {
        if self.readiness != ManagerReadiness::ReadyToFlush {
            return Err(coins_corruption("flush before ready"));
        }

        let decision = decide_flush(FlushPolicyInput {
            mode,
            cache_bytes: cache.estimated_cache_bytes(),
            cache_byte_limit: self.cache_byte_limit,
            mempool_leftover_bytes: self.mempool_leftover_bytes,
            now,
            next_write: self.next_write,
            memory_pressure: self.memory_pressure,
            disk_free_bytes,
            cache_entry_count: cache.cache_entry_count(),
        });
        if mode != FlushMode::None {
            self.maybe_last_write_decision = Some(decision);
        }
        let Some(write_kind) = coins_write_kind(decision) else {
            if matches!(decision, FlushDecision::RefuseDiskSpace(_)) {
                return Err(refuse_disk_space());
            }
            return Ok(FlushExecution {
                decision,
                wrote_coins: false,
            });
        };

        persist_ordered_prefix(sink, block_payloads, undo_window, header_entries)?;
        match write_kind {
            CoinsWriteKind::Flush => cache.flush().map_err(map_chainstate)?,
            CoinsWriteKind::Sync => cache.sync().map_err(map_chainstate)?,
        }
        sink.persist_chain_meta(active_chain)?;
        Ok(FlushExecution {
            decision,
            wrote_coins: true,
        })
    }

    #[cfg(test)]
    pub(crate) fn ready_for_test(
        cache_byte_limit: u64,
        mempool_leftover_bytes: u64,
        next_write: FlushPolicyTime,
        memory_pressure: bool,
    ) -> Self {
        Self {
            readiness: ManagerReadiness::ReadyToFlush,
            cache_byte_limit,
            mempool_leftover_bytes,
            next_write,
            memory_pressure,
            maybe_last_write_decision: None,
            maybe_recovery_outcome: Some(CoinsRecoveryOutcome::Consistent),
        }
    }

    #[cfg(test)]
    pub(crate) fn not_ready_for_test() -> Self {
        Self {
            readiness: ManagerReadiness::NotReady,
            cache_byte_limit: default_coins_cache_byte_limit(),
            mempool_leftover_bytes: 0,
            next_write: FlushPolicyTime::from_unix_seconds(0),
            memory_pressure: false,
            maybe_last_write_decision: None,
            maybe_recovery_outcome: None,
        }
    }
}

/// Available bytes for the datadir filesystem. Callers may also inject a fact.
///
/// Uses `fs4` so this crate can stay `forbid(unsafe_code)`. Probe failure
/// reports `0` so the disk-space guard can refuse writes. Keep `u64::MAX`
/// only for explicit unprobed test paths via `FlushPersistSink::disk_free_bytes`.
pub fn probe_disk_free_bytes(datadir: &Path) -> u64 {
    fs4::available_space(datadir).unwrap_or(0)
}

fn apply_recovery_decision(
    store: &FjallNodeStore,
    view: FjallCoinsView,
    decision: RecoveryDecision,
) -> Result<FjallCoinsView, StorageError> {
    match decision {
        RecoveryDecision::ConsistentEmptyHeads | RecoveryDecision::OneHead => Ok(view),
        RecoveryDecision::InterruptedTwoHeads => replay_interrupted_flush(store, view),
        RecoveryDecision::InconsistentOtherCount { count } => Err(coins_corruption(format!(
            "fail_closed unexpected head_blocks count {count}"
        ))),
    }
}

pub(crate) fn coins_recovery_outcome_after_success(
    decision: RecoveryDecision,
    maybe_best_block: Option<BlockHash>,
) -> Result<CoinsRecoveryOutcome, StorageError> {
    match decision {
        RecoveryDecision::InterruptedTwoHeads => Ok(CoinsRecoveryOutcome::Replayed),
        RecoveryDecision::ConsistentEmptyHeads | RecoveryDecision::OneHead => {
            if maybe_best_block.is_some() {
                Ok(CoinsRecoveryOutcome::Consistent)
            } else {
                Ok(CoinsRecoveryOutcome::FailClosed)
            }
        }
        RecoveryDecision::InconsistentOtherCount { count } => Err(coins_corruption(format!(
            "fail_closed unexpected head_blocks count {count}"
        ))),
    }
}

fn annotate_fail_closed(error: StorageError, interrupted: bool) -> StorageError {
    let (namespace, action, detail) = match error {
        StorageError::InterruptedWrite { namespace, action } => (
            namespace,
            action,
            if interrupted {
                "fail_closed interrupted replay could not finish".to_string()
            } else {
                "fail_closed".to_string()
            },
        ),
        StorageError::Corruption {
            namespace,
            detail,
            action,
        } => (
            namespace,
            action,
            if detail.contains("fail_closed") {
                if interrupted && !detail.contains("interrupted") {
                    format!("fail_closed interrupted {detail}")
                } else {
                    detail
                }
            } else if interrupted {
                format!("fail_closed interrupted {detail}")
            } else {
                format!("fail_closed {detail}")
            },
        ),
        other => return other,
    };
    StorageError::Corruption {
        namespace,
        detail,
        action,
    }
}

enum CoinsWriteKind {
    Flush,
    Sync,
}

const fn coins_write_kind(decision: FlushDecision) -> Option<CoinsWriteKind> {
    match decision {
        FlushDecision::Flush(_) => Some(CoinsWriteKind::Flush),
        FlushDecision::Sync(_) => Some(CoinsWriteKind::Sync),
        FlushDecision::None(_) | FlushDecision::RefuseDiskSpace(_) => None,
    }
}

fn persist_ordered_prefix<S: FlushPersistSink>(
    sink: &mut S,
    block_payloads: &[Block],
    undo_window: &[(BlockHash, BlockUndo)],
    header_entries: &[HeaderEntry],
) -> Result<(), StorageError> {
    for block in block_payloads {
        sink.persist_block(block)?;
    }
    for (hash, undo) in undo_window {
        sink.persist_undo(*hash, undo)?;
    }
    sink.persist_header_entries(header_entries)
}

fn map_chainstate(error: ChainstateError) -> StorageError {
    match error {
        ChainstateError::InterruptedWrite { .. } => StorageError::InterruptedWrite {
            namespace: StorageNamespace::Coins,
            action: StorageRecoveryAction::Reindex,
        },
        ChainstateError::CoinsStorage { detail } => coins_corruption(detail),
        other => coins_corruption(other),
    }
}

fn refuse_disk_space() -> StorageError {
    StorageError::Corruption {
        namespace: StorageNamespace::Coins,
        detail: "refuse disk space".to_string(),
        action: StorageRecoveryAction::Repair,
    }
}

fn coins_corruption(detail: impl core::fmt::Display) -> StorageError {
    StorageError::Corruption {
        namespace: StorageNamespace::Coins,
        detail: detail.to_string(),
        action: StorageRecoveryAction::Repair,
    }
}
