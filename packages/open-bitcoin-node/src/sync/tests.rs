// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/test/functional/p2p_initial_headers_sync.py

use std::{
    cell::RefCell,
    collections::VecDeque,
    fs, io,
    net::{IpAddr, Ipv4Addr, SocketAddr},
    path::{Path, PathBuf},
    rc::Rc,
    sync::{Arc, Mutex},
    time::{SystemTime, UNIX_EPOCH},
};

use open_bitcoin_core::{
    chainstate::{ChainPosition, ChainstateSnapshot},
    consensus::{block_hash, block_merkle_root, check_block_header},
    primitives::{
        Amount, Block, BlockHash, BlockHeader, InventoryType, InventoryVector, MerkleRoot,
        OutPoint, ScriptBuf, ScriptWitness, Transaction, TransactionInput, TransactionOutput,
    },
};
use open_bitcoin_mempool::PolicyTime;
use open_bitcoin_network::{
    HeaderEntry, HeadersMessage, InventoryList, PeerId, VersionMessage, WireNetworkMessage,
};

use super::types::SyncReconcileProgress;
use super::{
    DurableSyncRuntime, PeerContribution, PeerFailureReason, PeerSyncOutcome, PeerSyncState,
    ResolvedSyncPeerAddress, SyncNetwork, SyncPeerAddress, SyncPeerReceiveOutcome,
    SyncPeerResolver, SyncPeerSession, SyncPeerSource, SyncRunSummary, SyncRuntimeConfig,
    SyncRuntimeError, SyncStopReason, SyncTransport, TcpPeerTransport,
};
use crate::{
    FieldAvailability, FjallNodeStore, LogRetentionPolicy, MetricKind, MetricRetentionPolicy,
    MetricSample, PersistMode, RuntimeMetadata, StorageError, StorageNamespace,
    StorageRecoveryAction,
    logging::{
        BLOCK_RELAY_LOG_SOURCE, StructuredLogLevel, StructuredLogRecord, writer::load_log_status,
    },
    status::{
        BestKnownTipSource, BestKnownTipStatus, BlockRelayEvidenceStatus,
        BlockServingActivationEvidence, BlockServingEligibilityCounters,
        BlockServingEvidenceStatus, BlockServingStatusCounters, CompactRelayAnnouncementCounters,
        CompactRelayCleanupCounters, CompactRelayFallbackCounters, CompactRelayInFlightCounters,
        CompactRelayMissingTransactionCounters, CompactRelayNegotiationCounters,
        CompactRelayReconstructionCounters, DurableSyncState, HealthSignal, HealthSignalLevel,
        InboundHandshakeStatusCounts, InboundPeerServingStatus, NoProgressDiagnosis,
        PeerContributionEvidence, PeerContributionKind, PeerTipAgreement, PeerTipAgreementStatus,
        ProgressCreditEvidence, ProgressCreditKind, RejectedProgressActivityKind,
        StallDiagnosisEvidence, StalledSubsystem, StayCurrentStatus, SyncLifecycleState,
        SyncProgress, SyncProgressSignal, SyncReconcileProgressStatus, SyncRecoveryCategory,
        SyncReorgEvidence, SyncResourcePressure, SyncStatus, TipFreshnessStatus,
        inbound_status_unavailable,
    },
};

mod production_announcement_transport_cases;
mod runtime_projection_cases;
mod runtime_timing_cases;
mod runtime_write_evidence_cases;
mod soak;

const EASY_BITS: u32 = 0x207f_ffff;

fn serialized_label<T>(value: T) -> String
where
    T: serde::Serialize,
{
    serde_json::to_value(value)
        .expect("status label serializes")
        .as_str()
        .expect("status label is a string")
        .to_string()
}

mod block_response;

mod phase70_peer;

mod wallet_rescan_runtime;

mod block_requests;
mod bounded_unattended_runtime;
mod durable_tip;
mod errors_and_live;
mod inflight_failures;
mod metrics_persistence;
mod no_progress_diagnosis;
mod peer_failure_rotation;
mod peer_resolution;
mod reorg_reconciliation;
mod restart_and_recovery;
mod restart_chainstate;
mod restart_resume_matrix;
mod status_projection;
mod stay_current_persistence;
mod stay_current_progress;
mod structured_runtime_logging;
mod summary_metrics_and_logs;
mod sync_cycles;
mod synthetic_long_chain;

mod support_transport;
use support_transport::{
    ErrorAfterMessagesTransport, ScriptedResolver, ScriptedSession, ScriptedTransport,
    resolved_manual_peer,
};
mod support_runtime;
use support_runtime::{
    connect_runtime_peer, durable_tip_capture, headers_script, load_structured_log_records,
    remove_dir_if_exists, sync_config, sync_config_with_log_dir, temp_store_path,
    two_peer_sync_config, version_verack_script,
};
mod support_blocks;
use support_blocks::{
    block_hash_hex, build_block, build_branch_block, coinbase_transaction, getdata_block_hashes,
    header, mine_header, notfound_for_block, phase70_branch_blocks,
    phase70_save_reorg_ready_branch, save_best_chain_with_active_blocks,
    save_chain_headers_snapshot_and_blocks, script,
};
mod support_status;
use support_status::{
    assert_no_progress_status, assert_progress_credit_unavailable, assert_rejected_activity,
    available_last_peer_contribution, available_last_useful_work, available_progress_credit,
    available_stall_diagnosis, block_relay_status_for_metrics, inbound_status_for_metrics,
    peer_outcome, peer_outcome_with_contribution, summary_with_peer_failure,
};
