// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

//! Shared chainstate-durability operator evidence contract.
//!
//! This field is distinct from `recovery_evidence` and `block_relay`. Serialized
//! values must not include `pruned`, `block_status_pruned`, `peer_id`,
//! `getblock`, `txid:vout`, or `cmpctblock` tokens.

use open_bitcoin_core::{
    chainstate::{
        CoinsCacheSizeState, FlushDecision, FlushMode, FlushPolicyInput, FlushPolicyTime,
        LastFlushReason, decide_flush,
    },
    primitives::BlockHash,
};
use serde::{Deserialize, Serialize};

use super::FieldAvailability;

/// Stable unavailable reason for unprojected or stopped runtimes (D-02).
pub const CHAINSTATE_DURABILITY_UNAVAILABLE_REASON: &str =
    "chainstate durability evidence unavailable";

/// Current coins-cache occupancy class from `decide_flush(FlushMode::None)`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CacheSizeLabel {
    Ok,
    Large,
    Critical,
}

impl CacheSizeLabel {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Ok => "ok",
            Self::Large => "large",
            Self::Critical => "critical",
        }
    }
}

/// Last real `execute_flush` reason; never overwritten by a later `None` classify.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LastFlushReasonLabel {
    None,
    Needed,
    Periodic,
    Always,
    FailedDisk,
}

impl LastFlushReasonLabel {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Needed => "needed",
            Self::Periodic => "periodic",
            Self::Always => "always",
            Self::FailedDisk => "failed_disk",
        }
    }
}

/// Last real write-kind from `FlushDecision`, not from occupancy-only `None`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WriteKindLabel {
    None,
    Flush,
    Sync,
    RefuseDiskSpace,
}

impl WriteKindLabel {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Flush => "flush",
            Self::Sync => "sync",
            Self::RefuseDiskSpace => "refuse_disk_space",
        }
    }
}

/// CanFlush readiness projected from `ManagerReadiness`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReadinessLabel {
    NotReady,
    ReadyToFlush,
}

impl ReadinessLabel {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotReady => "not_ready",
            Self::ReadyToFlush => "ready_to_flush",
        }
    }
}

/// Coins-marker recovery outcome; distinct from `SyncRecoveryCategory`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CoinsRecoveryOutcome {
    Consistent,
    Replayed,
    Interrupted,
    FailClosed,
}

impl CoinsRecoveryOutcome {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Consistent => "consistent",
            Self::Replayed => "replayed",
            Self::Interrupted => "interrupted",
            Self::FailClosed => "fail_closed",
        }
    }

    pub const fn omits_coins_best_block_without_tip(self) -> bool {
        matches!(self, Self::FailClosed | Self::Interrupted)
    }
}

/// Have-bytes versus do-not serving status. Do-not is unavailable when payload is absent.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ServingStatusLabel {
    Available,
    Unavailable,
}

impl ServingStatusLabel {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Available => "available",
            Self::Unavailable => "unavailable",
        }
    }
}

/// Last/aggregate have-bytes labels plus the three D-12 counters.
///
/// This is an I/O-free projection input. It is not a request log and does not
/// store `block_hash`, `peer_id`, or `pruned` counts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct HaveBytesEvidence {
    pub last_serving_status: ServingStatusLabel,
    pub last_payload_present: bool,
    pub last_index_known: bool,
    pub last_validated_on_active_chain: bool,
    pub available_count: u64,
    pub unavailable_count: u64,
    pub index_known_without_payload_count: u64,
}

impl HaveBytesEvidence {
    pub const fn empty() -> Self {
        Self {
            last_serving_status: ServingStatusLabel::Unavailable,
            last_payload_present: false,
            last_index_known: false,
            last_validated_on_active_chain: false,
            available_count: 0,
            unavailable_count: 0,
            index_known_without_payload_count: 0,
        }
    }
}

impl Default for HaveBytesEvidence {
    fn default() -> Self {
        Self::empty()
    }
}

/// Shared CSOBS snapshot payload. Height+hash only; no coin dumps.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ChainstateDurabilityEvidence {
    pub cache_size: CacheSizeLabel,
    pub last_flush_reason: LastFlushReasonLabel,
    pub write_kind: WriteKindLabel,
    pub readiness: ReadinessLabel,
    pub cache_bytes: u64,
    pub cache_byte_limit: u64,
    pub recovery_outcome: CoinsRecoveryOutcome,
    pub maybe_coins_best_block_height: Option<u64>,
    pub maybe_coins_best_block_hash: Option<String>,
    pub last_serving_status: ServingStatusLabel,
    pub last_payload_present: bool,
    pub last_index_known: bool,
    pub last_validated_on_active_chain: bool,
    pub available_count: u64,
    pub unavailable_count: u64,
    pub index_known_without_payload_count: u64,
}

impl ChainstateDurabilityEvidence {
    pub fn default_unavailable() -> FieldAvailability<Self> {
        FieldAvailability::unavailable(CHAINSTATE_DURABILITY_UNAVAILABLE_REASON)
    }

    pub fn apply_have_bytes(&mut self, have_bytes: HaveBytesEvidence) {
        self.last_serving_status = have_bytes.last_serving_status;
        self.last_payload_present = have_bytes.last_payload_present;
        self.last_index_known = have_bytes.last_index_known;
        self.last_validated_on_active_chain = have_bytes.last_validated_on_active_chain;
        self.available_count = have_bytes.available_count;
        self.unavailable_count = have_bytes.unavailable_count;
        self.index_known_without_payload_count = have_bytes.index_known_without_payload_count;
    }
}

impl Default for FieldAvailability<ChainstateDurabilityEvidence> {
    fn default() -> Self {
        ChainstateDurabilityEvidence::default_unavailable()
    }
}

/// Projects retained manager facts plus current occupancy via `FlushMode::None`.
#[allow(clippy::too_many_arguments)]
pub fn project_chainstate_durability(
    maybe_last_write_decision: Option<FlushDecision>,
    maybe_recovery_outcome: Option<CoinsRecoveryOutcome>,
    readiness: ReadinessLabel,
    estimated_cache_bytes: u64,
    cache_byte_limit: u64,
    mempool_leftover_bytes: u64,
    cache_entry_count: u64,
    now: FlushPolicyTime,
    next_write: FlushPolicyTime,
    memory_pressure: bool,
    disk_free_bytes: u64,
    maybe_coins_best_block: Option<BlockHash>,
    maybe_coins_best_block_height: Option<u64>,
    have_bytes: HaveBytesEvidence,
) -> FieldAvailability<ChainstateDurabilityEvidence> {
    let Some(recovery_outcome) = maybe_recovery_outcome else {
        return ChainstateDurabilityEvidence::default_unavailable();
    };

    let occupancy = decide_flush(FlushPolicyInput {
        mode: FlushMode::None,
        cache_bytes: estimated_cache_bytes,
        cache_byte_limit,
        mempool_leftover_bytes,
        now,
        next_write,
        memory_pressure,
        disk_free_bytes,
        cache_entry_count,
    });
    let (last_flush_reason, write_kind) = match maybe_last_write_decision {
        Some(decision) => (
            last_flush_reason_label(decision.reason()),
            write_kind_label(decision),
        ),
        None => (LastFlushReasonLabel::None, WriteKindLabel::None),
    };
    let (maybe_coins_best_block_height, maybe_coins_best_block_hash) = coins_best_block_fields(
        recovery_outcome,
        maybe_coins_best_block,
        maybe_coins_best_block_height,
    );
    let mut evidence = ChainstateDurabilityEvidence {
        cache_size: cache_size_label(occupancy.cache_size()),
        last_flush_reason,
        write_kind,
        readiness,
        cache_bytes: estimated_cache_bytes,
        cache_byte_limit,
        recovery_outcome,
        maybe_coins_best_block_height,
        maybe_coins_best_block_hash,
        last_serving_status: ServingStatusLabel::Unavailable,
        last_payload_present: false,
        last_index_known: false,
        last_validated_on_active_chain: false,
        available_count: 0,
        unavailable_count: 0,
        index_known_without_payload_count: 0,
    };
    evidence.apply_have_bytes(have_bytes);
    FieldAvailability::available(evidence)
}

fn coins_best_block_fields(
    recovery_outcome: CoinsRecoveryOutcome,
    maybe_coins_best_block: Option<BlockHash>,
    maybe_coins_best_block_height: Option<u64>,
) -> (Option<u64>, Option<String>) {
    if recovery_outcome.omits_coins_best_block_without_tip() && maybe_coins_best_block.is_none() {
        return (None, None);
    }
    let Some(block_hash) = maybe_coins_best_block else {
        return (None, None);
    };
    (
        maybe_coins_best_block_height,
        Some(block_hash_hex(block_hash)),
    )
}

fn cache_size_label(cache_size: CoinsCacheSizeState) -> CacheSizeLabel {
    match cache_size {
        CoinsCacheSizeState::Ok => CacheSizeLabel::Ok,
        CoinsCacheSizeState::Large => CacheSizeLabel::Large,
        CoinsCacheSizeState::Critical => CacheSizeLabel::Critical,
    }
}

fn last_flush_reason_label(reason: LastFlushReason) -> LastFlushReasonLabel {
    match reason {
        LastFlushReason::None => LastFlushReasonLabel::None,
        LastFlushReason::Needed => LastFlushReasonLabel::Needed,
        LastFlushReason::Periodic => LastFlushReasonLabel::Periodic,
        LastFlushReason::Always => LastFlushReasonLabel::Always,
        LastFlushReason::FailedDisk => LastFlushReasonLabel::FailedDisk,
    }
}

fn write_kind_label(decision: FlushDecision) -> WriteKindLabel {
    match decision {
        FlushDecision::None(_) => WriteKindLabel::None,
        FlushDecision::Flush(_) => WriteKindLabel::Flush,
        FlushDecision::Sync(_) => WriteKindLabel::Sync,
        FlushDecision::RefuseDiskSpace(_) => WriteKindLabel::RefuseDiskSpace,
    }
}

fn block_hash_hex(block_hash: BlockHash) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let bytes = block_hash.as_bytes();
    let mut encoded = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        encoded.push(HEX[(byte >> 4) as usize] as char);
        encoded.push(HEX[(byte & 0x0f) as usize] as char);
    }
    encoded
}

#[cfg(test)]
mod tests;
