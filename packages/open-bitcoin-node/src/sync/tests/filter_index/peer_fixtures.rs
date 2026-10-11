// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net.cpp
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/test/functional/p2p_blockfilters.py

//! Genuine accepted spend histories, durable coins, immutable rows and peer sessions.

pub(in crate::sync::tests) use super::peer_evidence::with_accepted_stale_for_negative;
use super::{catch_up::fixtures::TurnHistory, *};
use crate::chainstate::BasicFilterStartupMode;

pub(in crate::sync::tests) struct PeerFilterFixture {
    pub(in crate::sync::tests::filter_index) history: TurnHistory,
    pub runtime: DurableSyncRuntime,
}

pub(in crate::sync::tests) struct PeerSparseFixture {
    pub(in crate::sync::tests::filter_index) history: TurnHistory,
    pub store: FjallNodeStore,
    pub network: crate::DurableNetworkHandle,
}

pub(in crate::sync::tests) fn complete_peer_handshake(runtime: &DurableSyncRuntime, peer: u64) {
    for message in [
        WireNetworkMessage::Version(open_bitcoin_network::VersionMessage {
            nonce: 999,
            ..Default::default()
        }),
        WireNetworkMessage::Verack,
    ] {
        runtime
            .network
            .receive_sync_message(
                peer,
                message,
                50_000,
                runtime.verify_flags,
                runtime.consensus_params,
            )
            .expect("genuine peer handshake");
    }
}

impl PeerSparseFixture {
    pub fn cleanup(self) {
        let Self {
            history,
            store,
            network,
        } = self;
        drop(network);
        drop(store);
        history.history.cleanup();
    }
}

impl PeerFilterFixture {
    pub fn new(count: usize) -> Self {
        Self::from_history(TurnHistory::new(count, 0, 1))
    }

    pub fn lagging(count: usize, indexed: usize) -> Self {
        let history = TurnHistory::new(count, 0, 1);
        drop(history.seed(indexed));
        let runtime = Self::open(&history);
        Self { history, runtime }
    }

    pub fn large() -> Self {
        use open_bitcoin_core::{
            chainstate::Chainstate,
            consensus::{ConsensusParams, ScriptVerifyFlags},
        };
        let mut history = TurnHistory::new(16, 0, 1);
        let mut state = Chainstate::from_snapshot(history.history.full.clone());
        let mut block = fixture_block(
            history
                .history
                .records
                .last()
                .expect("parent")
                .identity()
                .block_hash(),
            16,
        );
        block.transactions[0].inputs[0].script_sig =
            ScriptBuf::from_bytes(vec![1, 16, 0x51]).expect("height");
        for number in 0..40_000_u32 {
            let mut script = vec![4];
            script.extend_from_slice(&number.to_le_bytes());
            script.extend([0x75, 0x51]);
            block.transactions[0].outputs.push(TransactionOutput {
                value: Amount::ZERO,
                script_pubkey: ScriptBuf::from_bytes(script).expect("distinct legal script"),
            });
        }
        block.header.time = 2_600;
        block.header.merkle_root = block_merkle_root(&block.transactions).expect("merkle").0;
        block.header.nonce = (0..=u32::MAX)
            .find(|nonce| {
                block.header.nonce = *nonce;
                check_block_header(&block.header).is_ok()
            })
            .expect("genuine easy POW");
        let staged = state
            .stage_connect_block_with_current_time(
                &block,
                17,
                2_601,
                ScriptVerifyFlags::P2SH,
                ConsensusParams {
                    coinbase_maturity: 1,
                    ..Default::default()
                },
            )
            .expect("genuine consensus-valid large block");
        state
            .commit_staged_connect(staged)
            .expect("commit large block");
        let position = state.tip().expect("tip");
        let inputs = BasicFilterInputs::from_historical(
            &block,
            position,
            Some(HistoricalBlockUndo {
                block_hash: position.block_hash,
                undo: &state.undo_by_block()[&position.block_hash],
            }),
        )
        .expect("complete genuine history");
        history.history.records.push(
            StoredFilterRecord::generate(
                &inputs,
                position,
                history
                    .history
                    .records
                    .last()
                    .map(StoredFilterRecord::identity)
                    .as_ref(),
            )
            .expect("genuine generated filter"),
        );
        history.history.blocks.push(block);
        history.history.full = state.snapshot();
        Self::from_history(history)
    }

    pub fn sparse(count: usize) -> PeerSparseFixture {
        use open_bitcoin_core::chainstate::{Chainstate, CoinsCache, FlushPolicyTime};
        let history = TurnHistory::new(count, 0, 1);
        let store = FjallNodeStore::open(&history.history.path).expect("sparse store");
        store
            .seed_coins_from_snapshot(&history.history.full)
            .expect("genuine durable coins");
        let headers: Vec<_> = history
            .history
            .full
            .active_chain
            .iter()
            .map(|position| HeaderEntry {
                block_hash: position.block_hash,
                header: position.header.clone(),
                height: position.height,
                chain_work: position.chain_work,
            })
            .collect();
        store
            .save_header_entries(&headers, PersistMode::Sync)
            .expect("accepted headers");
        let fence = fence(&history.history.full.active_chain);
        store
            .initialize_basic_filter_state(&fence)
            .expect("explicit empty index");
        let cp = checkpoint(&history.history.records[2]);
        publish_current(
            &store,
            &fence,
            cp,
            cp.input_protection(),
            &history.history.records[..3],
        )
        .expect("genuine short persisted prefix");
        // This fixture measures refusal before index access. Construct the actual
        // network over genuine durable coins without activating catch-up on an
        // intentionally short prefix whose future input bodies are not retained.
        let state = Chainstate::from_coins_cache(
            CoinsCache::from_parent(store.coins_view()),
            history.history.full.active_chain.clone(),
            history.history.full.undo_by_block.clone(),
            None,
        );
        let manager = crate::ManagedChainstate::from_recovered_chainstate(
            crate::FjallChainstateStore::from_store(store.clone()),
            state,
            crate::chainstate::FlushLifecycle::ready(
                FlushPolicyTime::from_unix_seconds(0),
                FlushPolicyTime::from_unix_seconds(0),
                0,
                false,
            ),
        )
        .expect("actual durable coins owner");
        let mut network = crate::ManagedPeerNetwork::from_initialized_chainstate(
            manager,
            Default::default(),
            Default::default(),
            16,
            Default::default(),
            Default::default(),
            false,
        );
        let mut header_store = open_bitcoin_network::HeaderStore::default();
        header_store.seed_from_chain(&history.history.full.active_chain);
        network.seed_header_store(header_store);
        network.set_basic_filter_serving(true);
        network
            .connect_outbound_peer(1, 0)
            .expect("actual local version capability");
        PeerSparseFixture {
            history,
            store,
            network: crate::ManagedNetworkHandle::new(network),
        }
    }

    fn from_history(history: TurnHistory) -> Self {
        let count = history.history.records.len();
        let store = history.seed(count);
        drop(store);
        let runtime = Self::open(&history);
        Self { history, runtime }
    }

    fn open(history: &TurnHistory) -> DurableSyncRuntime {
        let runtime = DurableSyncRuntime::open_configured(
            FjallNodeStore::open(&history.history.path).expect("actual cold Fjall reopen"),
            sync_config(),
            BasicFilterStartupMode::Enabled,
        )
        .expect("genuine recovered node");
        runtime
            .network
            .configure_basic_peer_serving(true)
            .expect("explicit capability");
        runtime
            .network
            .connect_outbound_peer(1, 0)
            .expect("actual advertised peer session");
        complete_peer_handshake(&runtime, 1);
        runtime
    }

    pub fn reopen(self) -> Self {
        let Self { history, runtime } = self;
        drop(runtime);
        let runtime = Self::open(&history);
        Self { history, runtime }
    }

    pub fn hash(&self, height: usize) -> BlockHash {
        self.history.history.records[height].identity().block_hash()
    }

    pub fn cleanup(self) {
        let Self { history, runtime } = self;
        drop(runtime);
        history.history.cleanup();
    }
}

#[path = "../../../network/runtime_authority/filter_index/serving/tests.rs"]
mod phase160_serving_tests;

#[test]
fn phase160_managed_completeness_genuine_displaced_branch_survives_reorg() {
    use crate::network::{
        ManagedInboundResponsePlanItem as Item, filter_transport::FilterTransportSnapshot,
    };
    use phase160_serving_tests::{decoded_preparation, managed_receive, request_filters};
    // Arrange: fork A is genuinely accepted by the managed durable authority.
    let fixture = super::reorg::fixtures::ForkFixture::compact(16, 0, 16);
    fixture
        .runtime
        .network
        .configure_basic_peer_serving(true)
        .expect("capability");
    fixture
        .runtime
        .network
        .connect_outbound_peer(1, 0)
        .expect("peer");
    complete_peer_handshake(&fixture.runtime, 1);
    let first = fixture.fork(10, 8, 0x61);
    fixture.apply(&first);
    fixture.finish();
    fixture.flush();
    let stop = first
        .records
        .last()
        .expect("accepted stop")
        .identity()
        .block_hash();
    let before = fixture
        .runtime
        .network
        .maybe_basic_index_progress()
        .expect("progress");
    let held = managed_receive(&fixture.runtime, 1, request_filters(11, stop), false);
    assert_eq!(
        fixture
            .runtime
            .network
            .maybe_basic_index_progress()
            .expect("progress"),
        before
    );
    let Some(Item::PreparedFilterServe(mut response)) =
        held.inbound_response_plan.into_iter().next()
    else {
        panic!("complete prepared A");
    };
    let second = fixture.fork(10, 9, 0x62);
    // Act: actual branch replacement revokes the earlier complete preparation.
    fixture.apply(&second);
    assert!(
        !fixture
            .runtime
            .network
            .basic_filter_response_is_current(&response)
            .expect("revoked")
    );
    assert_eq!(response.progress().expect("prefix").messages, 0);
    assert!(response.next_frame().is_err());
    drop(response);
    assert_eq!(
        fixture
            .runtime
            .network
            .basic_filter_transport_snapshot()
            .expect("released"),
        FilterTransportSnapshot::default()
    );
    fixture.finish();
    assert_eq!(
        fixture
            .runtime
            .store()
            .validation_provenance(stop)
            .expect("genuine provenance"),
        crate::storage::validation_history::ValidationProvenance::ScriptsValid
    );
    let before = fixture
        .runtime
        .network
        .maybe_basic_index_progress()
        .expect("progress");
    let stale = decoded_preparation(
        &fixture.runtime,
        managed_receive(&fixture.runtime, 1, request_filters(11, stop), false),
    );
    // Assert: retained A is served by exact requested ancestry rather than B heights.
    assert_eq!(
        stale,
        first
            .records
            .iter()
            .map(
                |record| WireNetworkMessage::CFilter(open_bitcoin_network::CFilter {
                    filter_type: 0,
                    block_hash: record.identity().block_hash(),
                    filter: record.encoded_bytes().to_vec()
                })
            )
            .collect::<Vec<_>>()
    );
    assert_eq!(
        fixture
            .runtime
            .network
            .maybe_basic_index_progress()
            .expect("progress"),
        before
    );
    fixture.cleanup();
}

#[test]
fn phase160_complete_preparation_actual_encoding_work_exact_and_one_under() {
    use crate::network::filter_transport::{
        BasicFilterPrepareOutcome as Outcome, BasicFilterServeUnavailable as Unavailable,
        FilterTransportSnapshot,
    };
    use open_bitcoin_network::{
        basic_peer_resource::{BasicPeerServeAxis as Axis, BasicPeerServeBudget},
        policy::FilterRequest,
    };
    // Arrange
    let fixture = PeerFilterFixture::new(16);
    let handle = &fixture.runtime.network;
    let request = FilterRequest::Filters {
        filter_type: 0,
        start_height: 1,
        stop_hash: fixture.hash(10),
    };
    let (outcome, measured) = handle
        .measure_basic_filter_preparation_for_test(
            1,
            request,
            fixture.runtime.consensus_params,
            BasicPeerServeBudget::default(),
        )
        .expect("measured actual preparation");
    assert!(matches!(outcome, Outcome::Prepared(_)));
    drop(outcome);
    // Act / Assert: every finite axis admits exactly observed work and refuses one less.
    for axis in Axis::ALL {
        let amount = measured.get(axis);
        assert!(amount > 0, "{axis:?}");
        let (exact, work) = handle
            .measure_basic_filter_preparation_for_test(
                1,
                request,
                fixture.runtime.consensus_params,
                BasicPeerServeBudget::default().with_limit(axis, amount),
            )
            .expect("exact");
        assert!(matches!(exact, Outcome::Prepared(_)), "{axis:?}: {exact:?}");
        assert_eq!(work, measured);
        drop(exact);
        let (refused, _) = handle
            .measure_basic_filter_preparation_for_test(
                1,
                request,
                fixture.runtime.consensus_params,
                BasicPeerServeBudget::default().with_limit(axis, amount - 1),
            )
            .expect("bounded refusal");
        assert_eq!(
            refused,
            Outcome::Unavailable(Unavailable::ResourceRefused),
            "{axis:?}"
        );
        assert_eq!(
            handle.basic_filter_transport_snapshot().expect("released"),
            FilterTransportSnapshot::default()
        );
    }
    // Source copies, payload and frame copies plus checksum work are additional to reads.
    let mut read_budget = BasicPeerServeBudget::default();
    let read = handle
        .measure_basic_peer_reads_for_test(request, &mut read_budget)
        .expect("read measurement");
    assert!(read.result.expect("complete persisted read").is_some());
    assert!(measured.get(Axis::LogicalCopyBytes) > read.work.get(Axis::LogicalCopyBytes));
    assert!(measured.get(Axis::HashPaddedBytes) > read.work.get(Axis::HashPaddedBytes));
    assert_eq!(
        measured.get(Axis::EncodedSuffixBytes),
        read.work.get(Axis::EncodedSuffixBytes)
    );
    fixture.cleanup();
}

#[test]
fn phase160_managed_completeness_concurrent_raw_integrity_and_grant_loss_fail_closed() {
    use crate::network::filter_transport::{
        BasicFilterPrepareOutcome as Outcome, BasicFilterServeUnavailable as Unavailable,
        FilterTransportSnapshot,
    };
    use open_bitcoin_network::policy::FilterRequest;
    for raw in [false, true] {
        // Arrange
        let fixture = PeerFilterFixture::new(16);
        let handle = &fixture.runtime.network;
        let before = handle.maybe_basic_index_progress().expect("progress");
        let barrier = std::sync::Barrier::new(2);
        let writer_store = fixture.runtime.store().clone();
        let writer_handle = handle.clone();
        let raw_hash = fixture.hash(15);
        // Act: start the actual clone writer concurrently with the complete preparation.
        let outcome = std::thread::scope(|scope| {
            let start = &barrier;
            let worker = scope.spawn(move || {
                start.wait();
                if raw {
                    writer_store
                        .remove_validation_filter_for_test(raw_hash)
                        .expect("actual raw mutation");
                } else {
                    writer_handle
                        .configure_basic_peer_serving(false)
                        .expect("actual grant revocation");
                }
            });
            barrier.wait();
            let outcome = handle
                .prepare_basic_filter_response(
                    1,
                    FilterRequest::Filters {
                        filter_type: 0,
                        start_height: 1,
                        stop_hash: fixture.hash(10),
                    },
                    fixture.runtime.consensus_params,
                )
                .expect("bounded authority turn");
            worker.join().expect("writer");
            outcome
        });
        // Assert: every valid interleaving denies a live first frame, without owner work.
        match outcome {
            Outcome::Prepared(mut response) => {
                assert!(
                    !handle
                        .basic_filter_response_is_current(&response)
                        .expect("revoked")
                );
                assert_eq!(response.progress().expect("prefix").messages, 0);
                assert!(response.next_frame().is_err());
            }
            Outcome::Unavailable(Unavailable::StorageFailure | Unavailable::Invalidated) => {}
            other => panic!("unexpected race outcome: {other:?}"),
        }
        assert_eq!(
            handle.basic_filter_transport_snapshot().expect("released"),
            FilterTransportSnapshot::default()
        );
        assert_eq!(
            handle.maybe_basic_index_progress().expect("progress"),
            before
        );
        fixture.cleanup();
    }
}

#[test]
fn phase160_managed_completeness_handoff_keeps_local_buffers_charged_until_transfer() {
    use crate::network::filter_transport::{
        BasicFilterPrepareOutcome as Outcome, FilterTransportSnapshot,
    };
    use open_bitcoin_network::{
        basic_peer_resource::BASIC_PEER_MAX_SUFFIX_BYTES, policy::FilterRequest,
    };
    // Arrange
    let fixture = PeerFilterFixture::new(16);
    let handle = &fixture.runtime.network;
    let request = FilterRequest::Filters {
        filter_type: 0,
        start_height: 1,
        stop_hash: fixture.hash(2),
    };
    std::thread::scope(|scope| {
        let mut maybe_revoker = None;
        let (started_tx, started_rx) = std::sync::mpsc::channel();
        let revoker_handle = handle.clone();
        let params = fixture.runtime.consensus_params;
        // Act: pause exactly while complete frames are local, before registry transfer.
        let old = handle
            .inspect_basic_filter_handoff_for_test(1, request, params, |authority_held| {
                assert!(
                    authority_held,
                    "local encoded buffers must still own the authority handoff guard"
                );
                let snapshot = handle
                    .basic_filter_transport_snapshot()
                    .expect("reservation");
                assert_eq!(snapshot.requests, 1);
                assert_eq!(snapshot.suffix_bytes, BASIC_PEER_MAX_SUFFIX_BYTES);
                maybe_revoker = Some(scope.spawn(move || {
                    started_tx.send(()).expect("revoker started");
                    revoker_handle
                        .configure_basic_peer_serving(false)
                        .expect("disable");
                    revoker_handle
                        .configure_basic_peer_serving(true)
                        .expect("re-enable");
                    revoker_handle
                        .prepare_basic_filter_response(1, request, params)
                        .expect("later admission")
                }));
                started_rx.recv().expect("actual concurrent revoker");
                assert_eq!(
                    handle
                        .basic_filter_transport_snapshot()
                        .expect("still reserved"),
                    snapshot
                );
            })
            .expect("complete handoff");
        let new = maybe_revoker
            .take()
            .expect("revoker")
            .join()
            .expect("joined");
        // Assert: revocation/new admission happens after local buffers transfer or drop.
        let Outcome::Prepared(old) = old else {
            panic!("old prepared response");
        };
        let Outcome::Prepared(new) = new else {
            panic!("new prepared response");
        };
        assert!(
            !handle
                .basic_filter_response_is_current(&old)
                .expect("old revoked")
        );
        assert!(
            handle
                .basic_filter_response_is_current(&new)
                .expect("new current")
        );
        assert_eq!(old.progress().expect("unwritten").messages, 0);
        assert_eq!(
            handle
                .basic_filter_transport_snapshot()
                .expect("only new charged")
                .requests,
            1
        );
        drop(old);
        drop(new);
    });
    assert_eq!(
        handle.basic_filter_transport_snapshot().expect("released"),
        FilterTransportSnapshot::default()
    );
    fixture.cleanup();
}
