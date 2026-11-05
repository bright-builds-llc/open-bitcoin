// Parity breadcrumbs:
// - packages/bitcoin-knots/src/blockencodings.h
// - packages/bitcoin-knots/src/blockencodings.cpp
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/net_processing.h
// - packages/bitcoin-knots/test/functional/p2p_compactblocks.py

use std::collections::BTreeMap;

use open_bitcoin_codec::{
    BlockTransactions, BlockTransactionsRequest, CodecError, CompactBlockPayload,
};
use open_bitcoin_primitives::{Block, BlockHash, BlockHeader, InventoryType, Transaction, Wtxid};

pub const MAX_COMPACT_BLOCK_DOWNLOAD_DEPTH: u32 = 6;
pub const COMPACT_BLOCK_DOWNLOAD_TIMEOUT_SECONDS: i64 = 60;

use crate::block_serving::BlockRelayActivationPolicy;
use crate::compact_reconstruction::{
    CompactBlockTxnMisbehavior, CompactBlockTxnOutcome, CompactReconstructionInvalidReason,
    CompactReconstructionOutcome, PartialCompactBlock, apply_block_transactions, fill_block,
    init_partial_compact_block,
};

mod completion;
#[cfg(test)]
use completion::completion_actions_from_outcome;
pub(crate) use completion::{
    compact_download_actions_to_peer_actions_selected, complete_applied_download_selected,
    completed_or_fallback_init_selected, finish_applied_handle_selected,
    handle_block_transactions_selected, init_compact_block_download_selected,
    try_complete_compact_download_selected,
};

#[cfg(test)]
mod tests;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompactDownloadInFlight {
    pub partial: PartialCompactBlock,
    pub getblocktxn_in_flight: bool,
    pub requested_indexes: Vec<u16>,
    pub started_at_unix: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CompactBlockDownloadEligibilityInput {
    pub local_best_height: i32,
    pub best_tip_hash: BlockHash,
    pub maybe_known_block_height: Option<u32>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompactBlockDownloadEligibility {
    Eligible,
    TooFarFromTip,
    UnknownHeaderNotExtendingTip,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompactDownloadTimeoutExpired {
    pub block_hash: BlockHash,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CompactDownloadPeerState {
    pub in_flight: BTreeMap<BlockHash, CompactDownloadInFlight>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompactDownloadCleanupCause {
    PeerDisconnect,
    Timeout,
    Reorg,
    RuntimeRestart,
    BlockConnected,
}

impl CompactDownloadCleanupCause {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PeerDisconnect => "compact_download_peer_disconnect",
            Self::Timeout => "compact_download_timeout",
            Self::Reorg => "compact_download_reorg",
            Self::RuntimeRestart => "compact_download_restart",
            Self::BlockConnected => "compact_download_block_connected",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScheduleMissingTransactionSuppressionReason {
    CompactRelayDisabled,
    PeerNotCompactCapable,
    NoInFlightBlock,
    NoMissingIndexes,
    DuplicateInFlightRequest,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompactDownloadSuppressionReason {
    CompactReconstructionFailed,
    CompactDownloadTimeout,
    CompactPeerIneligible,
    CompactReconstructionInvalid,
    CompactBlockAlreadyInFlight,
}

impl CompactDownloadSuppressionReason {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CompactReconstructionFailed => "compact_reconstruction_failed",
            Self::CompactDownloadTimeout => "compact_download_timeout",
            Self::CompactPeerIneligible => "compact_peer_ineligible",
            Self::CompactReconstructionInvalid => "compact_reconstruction_invalid",
            Self::CompactBlockAlreadyInFlight => "compact_block_already_in_flight",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FullBlockFetch {
    pub block_hash: BlockHash,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CompactDownloadCompletionOutcome {
    Completed(Block),
    Fallback(FullBlockFetch),
    Suppressed(CompactDownloadSuppressionReason),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CompactDownloadAction {
    SendGetBlockTxn(BlockTransactionsRequest),
    RequestFullBlock(FullBlockFetch),
    ReceivedBlock(Block),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScheduleMissingTransactionOutcome {
    Scheduled { request: BlockTransactionsRequest },
    Suppressed(ScheduleMissingTransactionSuppressionReason),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CompactBlockInitOutcome {
    Ready {
        block_hash: BlockHash,
        actions: Vec<CompactDownloadAction>,
    },
    Completed {
        block: Block,
    },
    Fallback(FullBlockFetch),
    Misbehavior(CompactReconstructionInvalidReason),
    Suppressed(CompactDownloadSuppressionReason),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CompactBlockTxnHandleOutcome {
    Progress { actions: Vec<CompactDownloadAction> },
    Misbehavior(CompactBlockTxnMisbehavior),
    UnexpectedBlockHash,
    NoMatchingInFlight,
}

impl CompactDownloadPeerState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn cleanup(&mut self, cause: CompactDownloadCleanupCause) -> usize {
        let _ = cause;
        let cleared = self.in_flight.len();
        self.in_flight.clear();
        cleared
    }

    pub fn cleanup_block(&mut self, block_hash: BlockHash) -> bool {
        self.in_flight.remove(&block_hash).is_some()
    }
}

pub fn absolute_indexes_to_differential_deltas(missing: &[u16]) -> Vec<u64> {
    let mut deltas = Vec::with_capacity(missing.len());
    let mut previous_end = 0_u64;

    for index in missing {
        let absolute = u64::from(*index);
        let delta = absolute.saturating_sub(previous_end);
        deltas.push(delta);
        previous_end = absolute.saturating_add(1);
    }

    deltas
}

pub fn build_get_block_transactions_request(
    block_hash: BlockHash,
    missing_indexes: &[u16],
) -> BlockTransactionsRequest {
    BlockTransactionsRequest {
        block_hash,
        index_deltas: absolute_indexes_to_differential_deltas(missing_indexes),
    }
}

pub fn peer_supports_compact_download(peer_compact_capable: bool) -> bool {
    peer_compact_capable
}

pub fn evaluate_compact_block_download_eligibility(
    header: &BlockHeader,
    input: CompactBlockDownloadEligibilityInput,
) -> CompactBlockDownloadEligibility {
    if header.is_null() {
        return CompactBlockDownloadEligibility::TooFarFromTip;
    }

    let best_height = input.local_best_height.max(0) as u32;

    if let Some(block_height) = input.maybe_known_block_height {
        if block_height > best_height.saturating_add(1) {
            return CompactBlockDownloadEligibility::TooFarFromTip;
        }

        if best_height.saturating_sub(block_height) > MAX_COMPACT_BLOCK_DOWNLOAD_DEPTH {
            return CompactBlockDownloadEligibility::TooFarFromTip;
        }

        return CompactBlockDownloadEligibility::Eligible;
    }

    if header.previous_block_hash == input.best_tip_hash {
        return CompactBlockDownloadEligibility::Eligible;
    }

    CompactBlockDownloadEligibility::UnknownHeaderNotExtendingTip
}

pub fn expire_stale_compact_downloads(
    peer_state: &mut CompactDownloadPeerState,
    now_unix_seconds: i64,
    timeout_seconds: i64,
) -> Vec<CompactDownloadTimeoutExpired> {
    let mut expired = Vec::new();
    peer_state.in_flight.retain(|block_hash, in_flight| {
        let age = now_unix_seconds.saturating_sub(in_flight.started_at_unix);
        if age > timeout_seconds {
            expired.push(CompactDownloadTimeoutExpired {
                block_hash: *block_hash,
            });
            false
        } else {
            true
        }
    });
    expired
}

pub fn schedule_missing_transaction_request(
    activation: BlockRelayActivationPolicy,
    peer_compact_capable: bool,
    peer_state: &mut CompactDownloadPeerState,
    block_hash: BlockHash,
) -> ScheduleMissingTransactionOutcome {
    if !activation.compact_relay.enabled {
        return ScheduleMissingTransactionOutcome::Suppressed(
            ScheduleMissingTransactionSuppressionReason::CompactRelayDisabled,
        );
    }

    if !peer_supports_compact_download(peer_compact_capable) {
        return ScheduleMissingTransactionOutcome::Suppressed(
            ScheduleMissingTransactionSuppressionReason::PeerNotCompactCapable,
        );
    }

    let Some(in_flight) = peer_state.in_flight.get_mut(&block_hash) else {
        return ScheduleMissingTransactionOutcome::Suppressed(
            ScheduleMissingTransactionSuppressionReason::NoInFlightBlock,
        );
    };

    if in_flight.getblocktxn_in_flight {
        return ScheduleMissingTransactionOutcome::Suppressed(
            ScheduleMissingTransactionSuppressionReason::DuplicateInFlightRequest,
        );
    }

    let missing_indexes = in_flight.partial.missing_transaction_indexes();
    if missing_indexes.is_empty() {
        return ScheduleMissingTransactionOutcome::Suppressed(
            ScheduleMissingTransactionSuppressionReason::NoMissingIndexes,
        );
    }

    let request = build_get_block_transactions_request(block_hash, &missing_indexes);
    in_flight.getblocktxn_in_flight = true;
    in_flight.requested_indexes = missing_indexes;

    ScheduleMissingTransactionOutcome::Scheduled { request }
}

pub fn try_complete_compact_download(
    block_hash: BlockHash,
    in_flight: &CompactDownloadInFlight,
    check_witness_root: bool,
) -> Result<CompactDownloadCompletionOutcome, CodecError> {
    try_complete_compact_download_selected(block_hash, in_flight, check_witness_root)
}

pub fn compact_download_actions_to_peer_actions(
    actions: Vec<CompactDownloadAction>,
    inventory_type: InventoryType,
) -> Vec<crate::PeerAction> {
    compact_download_actions_to_peer_actions_selected(actions, inventory_type)
}

pub fn init_compact_block_download<'a>(
    relay: (BlockRelayActivationPolicy, bool),
    peer_state: &mut CompactDownloadPeerState,
    payload: &CompactBlockPayload,
    selection: (CompactBlockDownloadEligibilityInput, i64, bool),
    candidates: impl IntoIterator<Item = (&'a Wtxid, &'a Transaction)>,
    extra_transactions: impl IntoIterator<Item = (&'a Wtxid, &'a Transaction)>,
) -> Result<CompactBlockInitOutcome, CodecError> {
    init_compact_block_download_selected(
        relay,
        peer_state,
        payload,
        selection,
        candidates,
        extra_transactions,
    )
}

pub fn handle_block_transactions(
    activation: BlockRelayActivationPolicy,
    peer_compact_capable: bool,
    peer_state: &mut CompactDownloadPeerState,
    response: &BlockTransactions,
    check_witness_root: bool,
) -> Result<CompactBlockTxnHandleOutcome, CodecError> {
    handle_block_transactions_selected(
        activation,
        peer_compact_capable,
        peer_state,
        response,
        check_witness_root,
    )
}

pub fn cleanup_compact_download_peer(
    peer_state: &mut CompactDownloadPeerState,
    cause: CompactDownloadCleanupCause,
) -> usize {
    peer_state.cleanup(cause)
}

pub fn cleanup_compact_download_on_block_connected(
    peer_state: &mut CompactDownloadPeerState,
    connected_block_hash: BlockHash,
) -> bool {
    peer_state.cleanup_block(connected_block_hash)
}

fn scheduled_init_request(
    block_hash: BlockHash,
    peer_state: &mut CompactDownloadPeerState,
    schedule: ScheduleMissingTransactionOutcome,
) -> Result<BlockTransactionsRequest, CompactBlockInitOutcome> {
    match schedule {
        ScheduleMissingTransactionOutcome::Scheduled { request } => Ok(request),
        ScheduleMissingTransactionOutcome::Suppressed(_) => {
            peer_state.in_flight.remove(&block_hash);
            Err(CompactBlockInitOutcome::Fallback(FullBlockFetch {
                block_hash,
            }))
        }
    }
}

fn finalize_ready_init(
    block_hash: BlockHash,
    peer_state: &mut CompactDownloadPeerState,
    schedule: ScheduleMissingTransactionOutcome,
) -> CompactBlockInitOutcome {
    let request = match scheduled_init_request(block_hash, peer_state, schedule) {
        Ok(request) => request,
        Err(outcome) => return outcome,
    };

    CompactBlockInitOutcome::Ready {
        block_hash,
        actions: vec![CompactDownloadAction::SendGetBlockTxn(request)],
    }
}

#[cfg(test)]
fn take_in_flight_download(
    peer_state: &mut CompactDownloadPeerState,
    block_hash: BlockHash,
) -> Result<CompactDownloadInFlight, CompactBlockTxnHandleOutcome> {
    peer_state
        .in_flight
        .remove(&block_hash)
        .ok_or(CompactBlockTxnHandleOutcome::NoMatchingInFlight)
}

#[cfg(test)]
fn activated_policy() -> BlockRelayActivationPolicy {
    BlockRelayActivationPolicy {
        compact_relay: crate::block_serving::CompactRelayActivationConfig { enabled: true },
        ..BlockRelayActivationPolicy::default()
    }
}
