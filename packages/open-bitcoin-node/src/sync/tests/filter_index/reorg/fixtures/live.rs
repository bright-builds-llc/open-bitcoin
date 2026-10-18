// Parity breadcrumbs:
// - packages/bitcoin-knots/src/index/base.cpp
// - packages/bitcoin-knots/src/index/blockfilterindex.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp

use super::*;

#[test]
fn phase165_candidates_local_storage_failure_cannot_be_hidden_by_viable_sibling() {
    // Arrange: only the stronger deep fork requires A15's physical undo.
    let mut fixture = ForkFixture::compact(16, 8, 24);
    let strongest = fixture.fork(10, 17, 0x6d);
    let viable = fixture.fork(20, 6, 0x6e);
    for anchored in strongest.connected.iter().chain(&viable.connected) {
        fixture
            .runtime
            .store()
            .save_block(&anchored.block, PersistMode::Sync)
            .expect("actual received body");
        fixture
            .runtime
            .network
            .retain_fixture_body_before_activation(
                &anchored.block,
                fixture.runtime.consensus_params,
            )
            .expect("actual checked retained body");
    }
    let lost = fixture.history.records[15].identity().block_hash();
    fixture
        .runtime
        .store()
        .delete_basic_input_for_test(lost, true)
        .expect("actual required deep undo loss");
    let before = fixture
        .runtime
        .network
        .chainstate_snapshot()
        .expect("before");
    let index = fixture
        .runtime
        .network
        .maybe_basic_index_progress()
        .expect("before");
    // Act
    let result = crate::sync::block_reconcile::reconcile_best_chain(&mut fixture.runtime, 50_000);
    // Assert: the local refusal is fatal for this turn, even though another fork is viable.
    assert!(
        matches!(
            result,
            Err(SyncRuntimeError::Storage(StorageError::Corruption { .. }))
        ),
        "{result:?}"
    );
    assert_eq!(
        fixture
            .runtime
            .network
            .chainstate_snapshot()
            .expect("unchanged"),
        before
    );
    assert_eq!(
        fixture
            .runtime
            .network
            .maybe_basic_index_progress()
            .expect("unchanged"),
        index
    );
    // Act: prove the sibling can succeed without replacing the actually lost source.
    fixture
        .runtime
        .network
        .reorg_to_branch(
            &viable.disconnect,
            &viable.connected,
            ReorgLifecycleContext::new(PolicyTime::from_unix_seconds(50_000)),
            fixture.runtime.verify_flags,
            fixture.runtime.consensus_params,
        )
        .expect("actual viable shallower fork");
    // Assert
    assert!(
        !fixture
            .runtime
            .store()
            .has_undo(lost)
            .expect("lost source is not repaired")
    );
    assert_eq!(
        fixture
            .runtime
            .network
            .maybe_chain_tip()
            .expect("read")
            .expect("tip")
            .block_hash,
        viable
            .records
            .last()
            .expect("target")
            .identity()
            .block_hash()
    );
    fixture.cleanup();
}

#[test]
fn phase165_fixture_basic_attachment_refuses_raw_and_mechanical_sources_without_fields() {
    use crate::chainstate::{FjallChainstateStore, ManagedChainstate, initialize_configured};
    // Arrange
    for mechanical in [false, true] {
        let path = reserved_filter_path("165-basic-raw-source");
        let store = FjallNodeStore::open(&path).expect("real store");
        let now = FlushPolicyTime::from_unix_seconds(0);
        let (lifecycle, _, cache) = initialize_configured(
            &store,
            now,
            now,
            0,
            false,
            u64::MAX,
            BasicFilterStartupMode::PreserveSaved,
        )
        .expect("actual initializer");
        let source = if mechanical {
            Chainstate::from_coins_cache_for_mechanical_fixture(
                cache,
                Vec::new(),
                Default::default(),
                None,
            )
        } else {
            Chainstate::from_coins_cache(cache, Vec::new(), Default::default(), None)
        };
        let mut manager = ManagedChainstate::from_recovered_chainstate(
            FjallChainstateStore::from_store(store.clone()),
            source,
            lifecycle,
        )
        .expect("real raw owner");
        let before = manager.chainstate().overlay_snapshot();
        // Act
        let result = manager.attach_fixture_basic_owner_after_flush(Default::default());
        // Assert
        assert!(matches!(
            result,
            Err(open_bitcoin_core::chainstate::ChainstateError::UntrustedWorkLineage)
        ));
        assert_eq!(manager.chainstate().overlay_snapshot(), before);
        assert!(manager.maybe_basic_index_progress().is_none());
        drop(manager);
        drop(store);
        fs::remove_dir_all(path).expect("cleanup");
    }
}

#[test]
fn phase165_fixture_basic_attachment_refuses_unflushed_or_missing_physical_undo() {
    // Arrange
    let path = reserved_filter_path("165-basic-physical-source");
    let mut runtime =
        DurableSyncRuntime::open(FjallNodeStore::open(&path).expect("store"), sync_config())
            .expect("runtime");
    runtime.consensus_params.coinbase_maturity = 1;
    let root = crate::sync::tests::build_block(BlockHash::default(), 0);
    crate::sync::tests::support_runtime::initialize_fixture_runtime_genesis(&runtime, &root);
    let child = crate::sync::tests::build_block(block_hash(&root.header), 1);
    connect_actual(&runtime, &child);
    let before = runtime.network.chainstate_snapshot().expect("before");
    let order = runtime
        .network
        .maybe_body_eligibility(block_hash(&child.header))
        .expect("before");
    // Act / Assert: no proof exists for the unflushed live tip.
    assert!(
        runtime
            .network
            .attach_fixture_basic_owner_after_flush(runtime.consensus_params)
            .is_err()
    );
    assert_eq!(
        runtime.network.chainstate_snapshot().expect("unchanged"),
        before
    );
    assert_eq!(
        runtime
            .network
            .maybe_body_eligibility(block_hash(&child.header))
            .expect("unchanged"),
        order
    );
    assert!(
        runtime
            .network
            .maybe_basic_index_progress()
            .expect("no owner")
            .is_none()
    );
    runtime
        .network
        .persist_validation_header_snapshot()
        .expect("actual admitted header snapshot");
    runtime
        .network
        .flush_coins(
            FlushMode::Always,
            FlushPolicyTime::from_unix_seconds(50_001),
            u64::MAX,
        )
        .expect("real full flush");
    runtime
        .network
        .persist_missing_live_fixture_counts()
        .expect("actual exact counts");
    runtime
        .store()
        .initialize_basic_filter_state(&fence(&before.active_chain))
        .expect("real BASIC proof");
    runtime
        .network
        .enable_basic_filter_index()
        .expect("configured BASIC");
    runtime
        .store()
        .delete_basic_input_for_test(block_hash(&child.header), true)
        .expect("actual required undo loss");
    // Act / Assert: the missing physical input is never restored or imported from memory.
    assert!(
        runtime
            .network
            .attach_fixture_basic_owner_after_flush(runtime.consensus_params)
            .is_err()
    );
    assert_eq!(
        runtime.network.chainstate_snapshot().expect("unchanged"),
        before
    );
    assert!(
        runtime
            .network
            .maybe_basic_index_progress()
            .expect("no owner")
            .is_none()
    );
    assert!(
        runtime
            .store()
            .load_undo(block_hash(&child.header))
            .expect("physical absence")
            .is_none()
    );
    drop(runtime);
    fs::remove_dir_all(path).expect("cleanup");
}

#[test]
fn phase165_fixture_basic_guarded_attachment_verifier_rejects_foreign_and_stale_proofs() {
    // Arrange
    let fixture = ForkFixture::compact(16, 8, 24);
    let foreign = ForkFixture::compact(16, 8, 24);
    let proof = fixture
        .runtime
        .store()
        .maybe_basic_filter_append_proof()
        .expect("proof")
        .expect("live proof");
    let before = fixture
        .runtime
        .network
        .chainstate_snapshot()
        .expect("before");
    let progress = fixture
        .runtime
        .network
        .maybe_basic_index_progress()
        .expect("before");
    // Act / Assert
    {
        let control = foreign
            .runtime
            .store()
            .filter_publication_guard()
            .expect("foreign actual serialization");
        assert!(
            foreign
                .runtime
                .store()
                .check_fixture_append_proof_guarded(&proof, &control)
                .is_err()
        );
    }
    fixture
        .runtime
        .store()
        .invalidate_basic_filter_append()
        .expect("expire actual proof");
    {
        let control = fixture
            .runtime
            .store()
            .filter_publication_guard()
            .expect("same actual serialization");
        assert!(
            fixture
                .runtime
                .store()
                .check_fixture_append_proof_guarded(&proof, &control)
                .is_err()
        );
    }
    assert_eq!(
        fixture
            .runtime
            .network
            .chainstate_snapshot()
            .expect("after"),
        before
    );
    assert_eq!(
        fixture
            .runtime
            .network
            .maybe_basic_index_progress()
            .expect("after"),
        progress
    );
    assert!(
        fixture
            .runtime
            .network
            .attach_fixture_basic_owner_after_flush(fixture.runtime.consensus_params)
            .is_err(),
        "existing lineage and BASIC owner cannot be overwritten"
    );
    fixture.cleanup();
    foreign.cleanup();
}

#[test]
fn phase165_candidates_configured_basic_equal_work_uses_actual_early_body_history() {
    // Arrange
    let mut fixture = ForkFixture::compact_early(16, 8, 24, 10, 13, 0x6c);
    let branch = fixture.fork(10, 13, 0x6c);
    let old_tip = fixture.history.records[23].identity().block_hash();
    for anchored in &branch.connected {
        fixture
            .runtime
            .store()
            .save_block(&anchored.block, PersistMode::Sync)
            .expect("received durable side bytes");
    }
    assert_eq!(
        fixture
            .runtime
            .network
            .best_chain_entries()
            .expect("headers")
            .last()
            .expect("first header tip")
            .block_hash,
        old_tip
    );
    // Act
    let progress = crate::sync::block_reconcile::reconcile_best_chain(&mut fixture.runtime, 50_000)
        .expect("actual sync atomic replacement");
    fixture.finish();
    fixture.flush();
    // Assert
    assert!(matches!(progress, SyncReconcileProgress::ReorgPersisted(_)));
    super::super::branches::assert_active(&fixture, &branch, 10);
    let active = fixture
        .runtime
        .network
        .maybe_chain_tip()
        .expect("read")
        .expect("active");
    let index = fixture
        .runtime
        .network
        .maybe_basic_index_progress()
        .expect("read")
        .expect("BASIC");
    assert_eq!(index.accepted_target().block_hash(), active.block_hash);
    assert_eq!(
        fixture
            .runtime
            .network
            .best_chain_entries()
            .expect("headers")
            .last()
            .expect("unchanged first header tip")
            .block_hash,
        old_tip
    );
    fixture.cleanup();
}

impl ForkFixture {
    pub fn compact_early(
        prefix: usize,
        suffix: usize,
        indexed: usize,
        ancestor: usize,
        count: usize,
        tag: u8,
    ) -> Self {
        // The first instance supplies only an independent body/filter recipe.
        let recipe = Self::compact(prefix, suffix, indexed);
        let branch = recipe.fork(ancestor, count, tag);
        let Self {
            history, runtime, ..
        } = recipe;
        drop(runtime);
        std::fs::remove_dir_all(&history.path)
            .expect("discard oracle-only store before live construction");
        let mut runtime = DurableSyncRuntime::open(
            FjallNodeStore::open(&history.path).expect("fresh actual live store"),
            sync_config(),
        )
        .expect("fresh actual live runtime");
        runtime.consensus_params.coinbase_maturity = 1;
        crate::sync::tests::support_runtime::initialize_fixture_runtime_genesis(
            &runtime,
            &history.blocks[0],
        );
        connect_runtime_peer(&mut runtime, 165, (prefix + suffix) as i32);
        for headers in [
            history.blocks[1..]
                .iter()
                .map(|block| block.header.clone())
                .collect(),
            branch
                .connected
                .iter()
                .map(|anchored| anchored.block.header.clone())
                .collect(),
        ] {
            runtime
                .network
                .receive_sync_message(
                    165,
                    WireNetworkMessage::Headers(HeadersMessage { headers }),
                    2_000_000_000,
                    runtime.verify_flags,
                    runtime.consensus_params,
                )
                .expect("actual header intake and durable admission identities before body order");
        }
        for block in &history.blocks[1..=ancestor] {
            connect_actual(&runtime, block);
        }
        for anchored in &branch.connected {
            runtime
                .network
                .retain_fixture_body_before_activation(&anchored.block, runtime.consensus_params)
                .expect("actual early checked side receipt, no supplied order");
        }
        for block in &history.blocks[ancestor + 1..] {
            connect_actual(&runtime, block);
        }
        runtime
            .network
            .persist_validation_header_snapshot()
            .expect("actual admitted header snapshot before flush");
        runtime
            .network
            .flush_coins(
                FlushMode::Always,
                FlushPolicyTime::from_unix_seconds(50_001),
                u64::MAX,
            )
            .expect("actual original durable coins checkpoint");
        runtime
            .network
            .persist_missing_live_fixture_counts()
            .expect("exact fixture counts after real flush");
        runtime
            .store()
            .initialize_basic_filter_state(&fence(&history.full.active_chain))
            .expect("initial BASIC state");
        for records in history.records[..indexed].chunks(128) {
            let cp = checkpoint(records.last().expect("nonempty prefix"));
            publish_current(
                runtime.store(),
                &fence(&history.full.active_chain),
                cp,
                cp.input_protection(),
                records,
            )
            .expect("independent original BASIC prefix");
        }
        runtime
            .network
            .enable_basic_filter_index()
            .expect("configured live BASIC");
        runtime
            .network
            .attach_fixture_basic_owner_after_flush(runtime.consensus_params)
            .expect("genuine same-live BASIC owner");
        runtime
            .network
            .drive_basic_filter_index_turn()
            .expect("same real startup turn");
        Self {
            history,
            runtime,
            standard_maturity: false,
        }
    }
}

pub(super) fn retain_available_active(runtime: &DurableSyncRuntime) {
    for position in runtime
        .network
        .chainstate_snapshot()
        .expect("actual active facts")
        .active_chain
    {
        if let Some(block) = runtime
            .store()
            .load_block(position.block_hash)
            .expect("actual retained body read")
        {
            runtime
                .network
                .retain_cached_candidate(&block, runtime.consensus_params, 2_000_000_000)
                .expect("full checked readmission, arrival history unknown");
        }
    }
}

fn connect_actual(runtime: &DurableSyncRuntime, block: &Block) {
    runtime
        .store()
        .save_block(block, PersistMode::Sync)
        .expect("retained actual live body");
    let disposition = runtime
        .network
        .connect_stored_block(
            block,
            open_bitcoin_core::consensus::chain_proof::ProofWork::ZERO,
            2_000_000_000,
            runtime.verify_flags,
            runtime.consensus_params,
        )
        .expect("actual live acceptance");
    assert!(matches!(
        disposition,
        crate::network::BlockConnectDisposition::Connected(_)
    ));
}
