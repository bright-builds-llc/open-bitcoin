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

mod live_fixture_support {
    use super::*;
    use crate::sync::live_fixture::{
        CANONICAL_REFUSAL, initialize_live_fixture_source_graph, take_stage_observations,
    };
    use crate::{
        FjallChainstateStore, FjallCoinsView, ManagedChainstate, ManagedPeerNetwork,
        chainstate::{BasicFilterStartupMode, FlushLifecycle, initialize_configured},
    };
    use open_bitcoin_core::{
        chainstate::{
            Chainstate, CoinsBatch, CoinsCacheEntry, CoinsView, FlushMode, FlushPolicyTime,
        },
        consensus::{CanonicalChain, ConsensusParams, ScriptVerifyFlags, chain_proof::ProofWork},
    };
    use open_bitcoin_network::{BlockRelayActivationPolicy, RelayActivationConfig};

    pub(super) fn aliased(kind: u8) {
        let mut fixture = Fixture::new(1);
        poison(&fixture.store, fixture.source(), kind);
        fixture
            .source()
            .chainstate()
            .validate_live_origin()
            .expect("still genuine");
        fixture
            .source()
            .chainstate()
            .bind_live_work_source(params())
            .expect("same binding");
        assert_eq!(
            fixture
                .source()
                .chainstate()
                .coins()
                .complete_snapshot()
                .expect("source"),
            fixture
                .store
                .coins_view()
                .complete_snapshot()
                .expect("aliased physical")
        );
        assert!(
            fixture
                .refuse(params(), false)
                .to_string()
                .contains("independent fixture physical")
        );
        fixture.cleanup();
    }
    pub(super) fn canonical_case(chain: CanonicalChain) {
        // Arrange
        let genesis = chain.spec().genesis_block();
        let selected = ConsensusParams {
            pow_limit_bits: genesis.header.bits,
            enforce_bip34_height_in_coinbase: false,
            ..Default::default()
        };
        let mut fixture = Fixture::from_blocks(vec![genesis], selected, true);
        fixture
            .source()
            .chainstate()
            .validate_live_origin()
            .expect("genuine canonical source");
        fixture
            .source()
            .chainstate()
            .bind_live_work_source(selected)
            .expect("exact selected params");
        // Act / Assert
        assert_eq!(
            fixture.refuse(selected, true),
            SyncRuntimeError::InvalidData {
                message: CANONICAL_REFUSAL.to_owned()
            }
        );
        fixture.cleanup();
    }

    pub(super) fn historical_order_controls() {
        // Arrange
        let mut fixture = Fixture::new(2);
        let captured =
            crate::sync::live_fixture::capture_live_fixture_source(fixture.source(), params())
                .expect("capture");
        let mut network = prepared(fixture.take(), &captured);
        assert!(
            network
                .chainstate()
                .maybe_accepted_body_preference()
                .is_none()
        );
        let historical = &fixture.blocks[1];
        network
            .retain_checked_candidate(historical, params(), 2_000_000_000)
            .expect("actual duplicate");
        let copied = network.peer_manager().header_store().clone();
        network.peer_manager_mut().seed_header_store(copied);
        assert!(
            network
                .maybe_body_eligibility(block_hash(&historical.header))
                .expect("copied history")
                .maybe_sequence
                .is_none()
        );
        let child = build_block(block_hash(&historical.header), 2);
        let side = build_branch_block(block_hash(&historical.header), 2, 10);
        network
            .connect_local_block(&child, ScriptVerifyFlags::P2SH, params())
            .expect("actual strict extension");
        assert!(
            network
                .maybe_body_eligibility(block_hash(&child.header))
                .expect("sticky unknown descendant")
                .maybe_sequence
                .is_none()
        );
        let before = network.chainstate_snapshot().expect("actual source before");
        // Act
        let result = network.reorg_to_branch(
            &[child],
            &[open_bitcoin_core::chainstate::AnchoredBlock {
                block: side,
                chain_work: ProofWork::ZERO,
            }],
            open_bitcoin_mempool::ReorgLifecycleContext::new(
                open_bitcoin_mempool::PolicyTime::from_unix_seconds(2_000_000_000),
            ),
            ScriptVerifyFlags::P2SH,
            params(),
        );
        // Assert
        assert!(
            result.is_err(),
            "duplicates and copied facts never mint historical equal preference"
        );
        assert_eq!(
            network.chainstate_snapshot().expect("unchanged source"),
            before
        );
        drop(network);
        fixture.cleanup();
    }

    type Source = ManagedChainstate<FjallChainstateStore, FjallCoinsView>;
    type Network = ManagedPeerNetwork<FjallChainstateStore, FjallCoinsView>;
    pub(super) fn params() -> ConsensusParams {
        ConsensusParams {
            coinbase_maturity: 1,
            ..Default::default()
        }
    }
    pub(super) fn lifecycle() -> FlushLifecycle {
        let now = FlushPolicyTime::from_unix_seconds(0);
        FlushLifecycle::ready(now, now, 0, false)
    }
    pub(super) struct Fixture {
        pub(super) path: PathBuf,
        pub(super) store: FjallNodeStore,
        pub(super) maybe_source: Option<Source>,
        pub(super) blocks: Vec<Block>,
    }
    impl Fixture {
        pub(super) fn new(count: u32) -> Self {
            let blocks = (0..count).fold(Vec::<Block>::new(), |mut blocks, height| {
                let parent = blocks
                    .last()
                    .map_or(BlockHash::default(), |b| block_hash(&b.header));
                blocks.push(build_block(parent, height));
                blocks
            });
            Self::from_blocks(blocks, params(), true)
        }
        pub(super) fn from_blocks(
            blocks: Vec<Block>,
            selected: ConsensusParams,
            flush: bool,
        ) -> Self {
            static NEXT_SOURCE: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
            let unique = NEXT_SOURCE.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            let path = temp_store_path(&format!("phase165-live-source-{unique}"));
            let store = FjallNodeStore::open(&path).expect("store");
            let mut core =
                Chainstate::from_parent(store.coins_view(), vec![], Default::default(), None);
            Chainstate::new()
                .rebind_into(&mut core)
                .expect("genuine empty");
            let mut source = ManagedChainstate::from_chainstate(
                FjallChainstateStore::from_store(store.clone()),
                core,
                lifecycle(),
            );
            for block in &blocks {
                source
                    .connect_block(block, ProofWork::ZERO, ScriptVerifyFlags::P2SH, selected)
                    .expect("actual managed acceptance");
            }
            if flush && !blocks.is_empty() {
                let (mut sink, mut same_core) = source.into_parts();
                let undo = same_core
                    .undo_by_block()
                    .iter()
                    .map(|(hash, undo)| (*hash, undo.clone()))
                    .collect::<Vec<_>>();
                let positions = same_core.active_chain().to_vec();
                let mut lifecycle = lifecycle();
                lifecycle
                    .execute_flush(
                        &mut sink,
                        &mut same_core,
                        FlushMode::Always,
                        FlushPolicyTime::from_unix_seconds(1),
                        u64::MAX,
                        &undo,
                        &[],
                        &blocks,
                        &positions,
                    )
                    .expect("same core real flush");
                source = ManagedChainstate::from_chainstate(sink, same_core, lifecycle);
            }
            take_stage_observations();
            Self {
                path,
                store,
                maybe_source: Some(source),
                blocks,
            }
        }
        pub(super) fn take(&mut self) -> Source {
            self.maybe_source.take().expect("owned source")
        }
        pub(super) fn source(&self) -> &Source {
            self.maybe_source.as_ref().expect("borrow source")
        }
        pub(super) fn cleanup(self) {
            drop(self.maybe_source);
            drop(self.store);
            remove_dir_if_exists(&self.path);
        }
        pub(super) fn refuse(&mut self, selected: ConsensusParams, pure: bool) -> SyncRuntimeError {
            // Arrange
            let before = observations(&self.store);
            take_stage_observations();
            // Act
            let result = open(self.take(), selected, BasicFilterStartupMode::Disabled);
            // Assert
            let error = result.err().expect("source must refuse");
            assert_eq!(take_stage_observations(), [usize::from(!pure), 0]);
            assert_eq!(
                observations(&self.store),
                before,
                "pre-init physical/index/prune unchanged"
            );
            error
        }
    }
    fn observations(store: &FjallNodeStore) -> String {
        format!(
            "{:?}{:?}{:?}",
            store.basic_operator_namespaces_for_test(),
            store.maybe_prune_intent(),
            store.load_have_pruned()
        )
    }
    pub(super) fn open(
        source: Source,
        selected: ConsensusParams,
        mode: BasicFilterStartupMode,
    ) -> Result<DurableSyncRuntime, SyncRuntimeError> {
        DurableSyncRuntime::from_live_fixture_source(
            source,
            sync_config(),
            selected,
            mode,
            RelayActivationConfig::default(),
            BlockRelayActivationPolicy::default(),
            false,
        )
    }

    pub(super) fn poison(store: &FjallNodeStore, source: &Source, kind: u8) {
        let current = source
            .chainstate()
            .coins()
            .complete_snapshot()
            .expect("source coins");
        let (point, coin) = current.coins.iter().next().expect("coin");
        let mut changed = coin.clone();
        changed.created_height += 1;
        let (point, entry) = match kind {
            0 => (
                OutPoint {
                    txid: open_bitcoin_core::primitives::Txid::from_byte_array([91; 32]),
                    vout: 0,
                },
                CoinsCacheEntry::unspent_dirty(changed),
            ),
            1 => (point.clone(), CoinsCacheEntry::spent_dirty()),
            _ => (point.clone(), CoinsCacheEntry::unspent_dirty(changed)),
        };
        store
            .coins_view()
            .batch_write(
                CoinsBatch {
                    entries: [(point, entry)].into(),
                },
                current.maybe_best_block,
            )
            .expect("ordinary writer SAME B");
    }

    pub(super) fn prepared(
        source: Source,
        captured: &crate::sync::live_fixture::CapturedFixtureIdentity,
    ) -> Network {
        let (sink, same_core) = source.into_parts();
        let now = FlushPolicyTime::from_unix_seconds(0);
        let (lifecycle, _view, _cache) = initialize_configured(
            sink.inner(),
            now,
            now,
            0,
            false,
            u64::MAX,
            BasicFilterStartupMode::Enabled,
        )
        .expect("actual normal initializer");
        let managed = ManagedChainstate::from_recovered_chainstate(sink, same_core, lifecycle)
            .expect("same recovered genuine source");
        let store = managed.store().inner().clone();
        let mut network = DurableSyncRuntime::prepare_initialized_network(
            &store,
            managed,
            &sync_config(),
            BasicFilterStartupMode::Enabled,
            RelayActivationConfig::default(),
            BlockRelayActivationPolicy::default(),
            false,
        )
        .expect("normal preparation");
        initialize_live_fixture_source_graph(&mut network, captured).expect("actual source graph");
        network
    }
    pub(super) fn masked(kind: u8) {
        // Arrange: real flush, then genuine validated-source rebind retaining the target mask.
        let mut fixture = Fixture::new(1);
        let (sink, mut same_core) = fixture.take().into_parts();
        let expected = same_core
            .coins()
            .complete_snapshot()
            .expect("actual flushed state");
        let (existing, coin) = expected.coins.iter().next().expect("actual coin");
        let point = if kind == 0 {
            OutPoint {
                txid: open_bitcoin_core::primitives::Txid::from_byte_array([92; 32]),
                vout: 0,
            }
        } else {
            existing.clone()
        };
        let mask = if kind == 0 {
            CoinsCacheEntry::spent_dirty()
        } else {
            CoinsCacheEntry::unspent_dirty(coin.clone())
        };
        same_core.coins_mut().absorb_batch_write(
            CoinsBatch {
                entries: [(point.clone(), mask)].into(),
            },
            expected.maybe_best_block,
        );
        assert!(
            same_core.validate_live_origin().is_err(),
            "cache mutation taints origin"
        );
        let mut genuine = Chainstate::new();
        for block in &fixture.blocks {
            genuine
                .connect_block_with_current_time(
                    block,
                    ProofWork::ZERO,
                    i64::from(block.header.time),
                    ScriptVerifyFlags::P2SH,
                    params(),
                )
                .expect("actual independent full prefix");
        }
        genuine
            .rebind_into(&mut same_core)
            .expect("consuming genuine source preserves target flags");
        same_core
            .validate_live_origin()
            .expect("genuinely rebound origin");
        same_core
            .bind_live_work_source(params())
            .expect("same full params");
        assert_eq!(
            same_core
                .coins()
                .complete_snapshot()
                .expect("retained mask"),
            expected
        );
        assert_eq!(
            fixture
                .store
                .coins_view()
                .complete_snapshot()
                .expect("physical prerequisites"),
            expected
        );
        assert_eq!(
            fixture
                .store
                .load_chain_meta_for_open()
                .expect("physical metadata")
                .0,
            same_core.active_chain()
        );
        assert_eq!(
            fixture
                .store
                .load_all_undo_records()
                .expect("physical undo"),
            *same_core.undo_by_block()
        );
        let source = ManagedChainstate::from_chainstate(sink, same_core, lifecycle());
        assert!(!source.has_pending_validation_history());
        let mut changed = coin.clone();
        changed.created_height += 9;
        let entry = if kind == 1 {
            CoinsCacheEntry::spent_dirty()
        } else {
            CoinsCacheEntry::unspent_dirty(changed)
        };
        fixture
            .store
            .coins_view()
            .batch_write(
                CoinsBatch {
                    entries: [(point, entry)].into(),
                },
                expected.maybe_best_block,
            )
            .expect("ordinary physical writer SAME B");
        assert_eq!(
            source
                .chainstate()
                .coins()
                .complete_snapshot()
                .expect("cache masks poison"),
            expected
        );
        assert_ne!(
            fixture
                .store
                .coins_view()
                .complete_snapshot()
                .expect("physical poison"),
            expected
        );
        fixture.maybe_source = Some(source);
        // Act / Assert
        assert!(
            fixture
                .refuse(params(), false)
                .to_string()
                .contains("independent fixture physical")
        );
        fixture.cleanup();
    }
}

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

mod peer_filter_transport;
mod phase165_candidates;
mod phase165_live_fixture;
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
mod filter_index;
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
    connect_runtime_peer, durable_tip_capture, flush_coins_always, headers_script,
    load_structured_log_records, remove_dir_if_exists, sync_config, sync_config_with_log_dir,
    temp_store_path, two_peer_sync_config, version_verack_script,
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
