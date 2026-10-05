// Parity breadcrumbs:
// - packages/bitcoin-knots/src/index/base.cpp
// - packages/bitcoin-knots/src/index/blockfilterindex.cpp
// - packages/bitcoin-knots/src/validation.cpp

use super::{recovery::ValidatedHistory, *};
use crate::chainstate::{ManagedChainstate, MemoryChainstateStore};
use open_bitcoin_core::{
    chainstate::filter_index::{
        catch_up::{AcceptedIndexTarget, BasicIndexProgress},
        lifecycle::IndexGeneration,
    },
    consensus::{ConsensusParams, ScriptVerifyFlags},
};

fn owner(history: &ValidatedHistory, behind: bool) -> BasicIndexProgress {
    let tip = history.old.active_chain.last().expect("funding tip");
    let maybe_endpoint = (!behind).then(|| history.records[1].identity());
    BasicIndexProgress::new(
        IndexGeneration::new(1),
        history.old.active_chain[0].block_hash,
        AcceptedIndexTarget::new(tip.height, tip.block_hash),
        maybe_endpoint,
        maybe_endpoint,
        IndexInputProtection::FromHeight(if behind { 0 } else { 2 }),
    )
    .expect("progress")
}

fn params() -> ConsensusParams {
    ConsensusParams {
        coinbase_maturity: 1,
        ..ConsensusParams::default()
    }
}

#[test]
fn phase157_accepted_direct_and_staged_capture_historical_and_same_block_scripts() {
    for staged in [false, true] {
        // Arrange
        let history = ValidatedHistory::new("accepted-complete", false);
        let mut managed = ManagedChainstate::from_store(MemoryChainstateStore::from_snapshot(
            history.old.clone(),
        ));
        managed
            .install_basic_index_owner(owner(&history, false))
            .expect("owner");
        let block = &history.blocks[2];
        // Act
        if staged {
            let prepared = managed
                .prepare_connect_block(block, 3, ScriptVerifyFlags::P2SH, params())
                .expect("prepare");
            managed.commit_prepared_connect(prepared).expect("absorb");
        } else {
            managed
                .connect_block(block, 3, ScriptVerifyFlags::P2SH, params())
                .expect("direct");
        }
        // Assert
        let facts = managed
            .maybe_accepted_basic_facts()
            .expect("complete one-block facts");
        assert_eq!(
            facts
                .inputs()
                .expect("inputs")
                .spent_scripts()
                .collect::<Vec<_>>(),
            vec![&[0x51][..], &[0x52][..]]
        );
        let actual = StoredFilterRecord::generate(
            &facts.inputs().expect("inputs"),
            facts.position(),
            Some(&history.records[1].identity()),
        )
        .expect("generated complete facts");
        assert_eq!(actual, history.records[2]);
        assert!(facts.work().cloned_bytes > 0);
        assert_eq!(facts.work().script_items, 5);
        assert_eq!(
            managed
                .maybe_basic_index_progress()
                .expect("owner")
                .accepted_target()
                .height(),
            2
        );
        let prepared = managed
            .maybe_basic_index_progress()
            .expect("progress")
            .prepare_turn()
            .expect("turn");
        managed
            .complete_basic_index_turn(prepared, &[actual.identity()])
            .expect("achieved publication reducer");
        assert!(managed.maybe_accepted_basic_facts().is_none());
    }
}

#[test]
fn phase157_accepted_behind_target_does_not_retain_out_of_order_facts() {
    // Arrange
    let history = ValidatedHistory::new("accepted-behind", false);
    let mut managed =
        ManagedChainstate::from_store(MemoryChainstateStore::from_snapshot(history.old.clone()));
    managed
        .install_basic_index_owner(owner(&history, true))
        .expect("owner");
    // Act
    managed
        .connect_block(&history.blocks[2], 3, ScriptVerifyFlags::P2SH, params())
        .expect("accepted");
    // Assert
    let progress = managed.maybe_basic_index_progress().expect("owner");
    assert_eq!(progress.accepted_target().height(), 2);
    assert_eq!(progress.maybe_next_height().expect("next"), Some(0));
    assert!(managed.maybe_accepted_basic_facts().is_none());
    assert!(!progress.initially_synchronized());
}

#[test]
fn phase157_accepted_later_metadata_failure_pauses_same_owner_without_erasing_tip() {
    // Arrange
    let history = ValidatedHistory::new("accepted-late-metadata", false);
    let store = history.seed(1);
    let runtime = DurableSyncRuntime::open_configured(
        store,
        sync_config(),
        crate::chainstate::BasicFilterStartupMode::Enabled,
    )
    .expect("actual configured runtime");
    runtime
        .network
        .install_basic_index_owner_for_test(owner(&history, false))
        .expect("owner");
    runtime
        .network
        .connect_stored_block(
            &history.blocks[2],
            3,
            i64::from(history.blocks[2].header.time),
            ScriptVerifyFlags::P2SH,
            params(),
        )
        .expect("ordinary accepted connect");
    runtime.store().set_basic_filter_fault(
        crate::storage::fjall_store::filters::FilterPublicationFault::BeforeChainMeta,
    );
    // Act
    let result = runtime.network.flush_coins(
        open_bitcoin_core::chainstate::FlushMode::Always,
        open_bitcoin_core::chainstate::FlushPolicyTime::from_unix_seconds(0),
        u64::MAX,
    );
    // Assert
    assert!(result.is_err());
    let progress = runtime
        .network
        .maybe_basic_index_progress()
        .expect("authority read")
        .expect("owner");
    assert_eq!(progress.accepted_target().height(), 2);
    assert_eq!(
        progress.state(),
        open_bitcoin_core::chainstate::filter_index::catch_up::BasicIndexState::Paused(
            open_bitcoin_core::chainstate::filter_index::catch_up::BasicIndexPause::Persistence
        )
    );
    assert_eq!(
        runtime
            .store()
            .maybe_basic_filter_checkpoint()
            .expect("cursor"),
        Some(checkpoint(&history.records[1]))
    );
    drop(runtime);
    history.cleanup();
}

#[test]
fn phase157_accepted_ordinary_local_persist_faults_keep_complete_facts_and_saved_protection() {
    use crate::storage::fjall_store::filters::FilterPublicationFault;
    for fault in [
        FilterPublicationFault::BeforeUndo,
        FilterPublicationFault::BeforeCoins,
        FilterPublicationFault::BeforeChainMeta,
    ] {
        // Arrange
        let history = ValidatedHistory::new("accepted-local-fault", false);
        let store = history.seed(1);
        let runtime = DurableSyncRuntime::open_configured(
            store,
            sync_config(),
            crate::chainstate::BasicFilterStartupMode::Enabled,
        )
        .expect("actual runtime");
        runtime
            .network
            .install_basic_index_owner_for_test(owner(&history, false))
            .expect("owner");
        runtime
            .network
            .force_basic_index_flush_for_test()
            .expect("existing zero-cache pressure policy");
        runtime.store().set_basic_filter_fault(fault);
        // Act
        let result = runtime.network.connect_local_block(
            &history.blocks[2],
            ScriptVerifyFlags::P2SH,
            params(),
        );
        // Assert
        assert!(result.is_err(), "ordinary persist reports {fault:?}");
        assert_eq!(
            runtime
                .network
                .maybe_chain_tip()
                .expect("tip")
                .expect("accepted")
                .height,
            2
        );
        let progress = runtime
            .network
            .maybe_basic_index_progress()
            .expect("read")
            .expect("owner");
        assert_eq!(progress.accepted_target().height(), 2);
        assert!(matches!(
            progress.state(),
            open_bitcoin_core::chainstate::filter_index::catch_up::BasicIndexState::Paused(_)
        ));
        let facts = runtime
            .network
            .accepted_basic_facts_for_test()
            .expect("read")
            .expect("facts survive persist error");
        assert_eq!(facts.spent_scripts, vec![vec![0x51], vec![0x52]]);
        assert!(facts.work.cloned_bytes > 0);
        assert_eq!(
            runtime
                .store()
                .maybe_basic_filter_checkpoint()
                .expect("cursor"),
            Some(checkpoint(&history.records[1]))
        );
        drop(runtime);
        let reopened = FjallNodeStore::open(&history.path).expect("real reopen after fault");
        assert_eq!(
            reopened
                .maybe_basic_filter_checkpoint()
                .expect("persisted cursor"),
            Some(checkpoint(&history.records[1]))
        );
        assert_eq!(
            reopened.load_prune_locks().expect("saved protection"),
            vec![
                IndexInputProtection::FromHeight(2)
                    .maybe_prune_lock()
                    .expect("lock")
            ]
        );
        drop(reopened);
        history.cleanup();
    }
}

#[test]
fn phase157_accepted_requested_body_save_failure_retains_accepted_target_and_safe_cursor() {
    requested_failure(crate::storage::fjall_store::filters::FilterPublicationFault::BeforeBody);
}

#[test]
fn phase157_accepted_requested_persist_faults_retain_accepted_target_and_safe_cursor() {
    use crate::storage::fjall_store::filters::FilterPublicationFault;
    for fault in [
        FilterPublicationFault::BeforeUndo,
        FilterPublicationFault::BeforeCoins,
        FilterPublicationFault::BeforeChainMeta,
    ] {
        requested_failure(fault);
    }
}

fn requested_failure(fault: crate::storage::fjall_store::filters::FilterPublicationFault) {
    // Arrange
    use crate::storage::fjall_store::filters::FilterPublicationFault;
    let name = match fault {
        FilterPublicationFault::BeforeBody => "accepted-requested-body",
        FilterPublicationFault::BeforeUndo => "accepted-requested-undo",
        FilterPublicationFault::BeforeCoins => "accepted-requested-coins",
        FilterPublicationFault::BeforeChainMeta => "accepted-requested-meta",
        _ => panic!("unexpected requested fixture fault"),
    };
    let history = ValidatedHistory::new(name, false);
    let store = history.seed(1);
    drop(store);
    raw_index(&history.path, |index| {
        let key = codec::record_key(block_hash(&history.blocks[2].header)).replacen(
            codec::RECORD_PREFIX,
            "block:",
            1,
        );
        index.remove(key).expect("future unrequested body absent");
    });
    let store = FjallNodeStore::open(&history.path).expect("real reopen without future body");
    assert!(
        !store
            .has_block(block_hash(&history.blocks[2].header))
            .expect("absent future body")
    );
    let mut runtime = DurableSyncRuntime::open_configured(
        store,
        sync_config(),
        crate::chainstate::BasicFilterStartupMode::Enabled,
    )
    .expect("actual runtime");
    runtime.consensus_params = params();
    runtime
        .network
        .install_basic_index_owner_for_test(owner(&history, false))
        .expect("owner");
    if fault != crate::storage::fjall_store::filters::FilterPublicationFault::BeforeBody {
        runtime
            .network
            .force_basic_index_flush_for_test()
            .expect("existing pressure policy");
    }
    runtime.store().set_basic_filter_fault(fault);
    let block = &history.blocks[2];
    let mut transport = ScriptedTransport::new(vec![vec![
        WireNetworkMessage::Version(VersionMessage {
            start_height: 2,
            ..VersionMessage::default()
        }),
        WireNetworkMessage::Verack,
        WireNetworkMessage::Headers(HeadersMessage {
            headers: vec![block.header.clone()],
        }),
        WireNetworkMessage::Block(block.clone()),
    ]]);
    // Act
    let result = runtime.sync_once(&mut transport, i64::from(block.header.time));
    // Assert
    assert!(
        getdata_block_hashes(&transport.sent_messages()).contains(&block_hash(&block.header)),
        "real requested block"
    );
    let summary = result.expect("sync returns its ordinary bounded peer failure report");
    assert_eq!(summary.failed_peers, 1);
    assert_eq!(summary.blocks_received, 0);
    assert!(
        summary.peer_outcomes[0].maybe_error.is_some(),
        "ordinary caller error is visible for {fault:?}"
    );
    assert_eq!(
        runtime
            .network
            .maybe_chain_tip()
            .expect("tip")
            .expect("accepted")
            .height,
        2
    );
    let progress = runtime
        .network
        .maybe_basic_index_progress()
        .expect("read")
        .expect("owner");
    assert_eq!(progress.accepted_target().height(), 2);
    assert!(matches!(
        progress.state(),
        open_bitcoin_core::chainstate::filter_index::catch_up::BasicIndexState::Paused(_)
    ));
    assert_eq!(
        runtime
            .store()
            .maybe_basic_filter_checkpoint()
            .expect("cursor"),
        Some(checkpoint(&history.records[1]))
    );
    assert!(
        !runtime
            .store()
            .has_block(block_hash(&block.header))
            .expect("body save did not happen")
    );
    drop(runtime);
    history.cleanup();
}

#[test]
fn phase157_accepted_genuine_direct_persist_faults_keep_target_before_error() {
    use crate::chainstate::{
        BasicFilterStartupMode, FjallChainstateStore, FlushLifecycle, initialize_configured,
    };
    use crate::storage::fjall_store::filters::FilterPublicationFault;
    use open_bitcoin_core::chainstate::{Chainstate, FlushPolicyTime};
    for fault in [
        FilterPublicationFault::BeforeUndo,
        FilterPublicationFault::BeforeCoins,
        FilterPublicationFault::BeforeChainMeta,
    ] {
        // Arrange
        let history = ValidatedHistory::new("accepted-direct-fault", false);
        let store = history.seed(1);
        let now = FlushPolicyTime::from_unix_seconds(0);
        let (_, _, cache) = initialize_configured(
            &store,
            now,
            now,
            0,
            false,
            u64::MAX,
            BasicFilterStartupMode::Enabled,
        )
        .expect("genuine initialization");
        let (positions, counts) = store.load_chain_meta_for_open().expect("metadata");
        let state = Chainstate::from_coins_cache(
            cache,
            positions,
            store.load_all_undo_records().expect("undo"),
            counts,
        );
        let mut managed = ManagedChainstate::from_recovered_chainstate(
            FjallChainstateStore::from_store(store.clone()),
            state,
            FlushLifecycle::ready_for_test(0, 0, now, false),
        )
        .expect("same genuine recovered parent");
        managed
            .install_basic_index_owner(owner(&history, false))
            .expect("owner");
        store.set_basic_filter_fault(fault);
        // Act
        let result =
            managed.connect_block(&history.blocks[2], 3, ScriptVerifyFlags::P2SH, params());
        // Assert
        assert!(result.is_err());
        assert_eq!(managed.chainstate().tip().expect("accepted").height, 2);
        assert_eq!(
            managed
                .maybe_basic_index_progress()
                .expect("owner")
                .accepted_target()
                .height(),
            2
        );
        assert!(managed.maybe_basic_index_failure().is_some());
        assert_eq!(
            managed
                .maybe_accepted_basic_facts()
                .expect("complete facts")
                .inputs()
                .expect("inputs")
                .spent_scripts()
                .collect::<Vec<_>>(),
            vec![&[0x51][..], &[0x52][..]]
        );
        assert_eq!(
            store
                .maybe_basic_filter_checkpoint()
                .expect("old saved cursor"),
            Some(checkpoint(&history.records[1]))
        );
        drop(managed);
        drop(store);
        history.cleanup();
    }
}

#[test]
fn phase157_accepted_persist_failure_still_applies_dependent_mempool_projection() {
    // Arrange
    let history = ValidatedHistory::new("accepted-mempool-fault", false);
    let store = history.seed(1);
    let runtime = DurableSyncRuntime::open_configured(
        store,
        sync_config(),
        crate::chainstate::BasicFilterStartupMode::Enabled,
    )
    .expect("runtime");
    runtime
        .network
        .install_basic_index_owner_for_test(owner(&history, false))
        .expect("owner");
    let mut block = history.blocks[2].clone();
    block.transactions.truncate(2);
    let mut output_script = vec![0xa9, 0x14];
    output_script.extend([0_u8; 20]);
    output_script.push(0x87);
    block.transactions[1].outputs[0].script_pubkey =
        ScriptBuf::from_bytes(output_script).expect("standard output");
    block.header.merkle_root = block_merkle_root(&block.transactions).expect("merkle").0;
    mine_header(&mut block);
    let admission = runtime
        .network
        .submit_local_transaction_outcome_at(
            block.transactions[1].clone(),
            ScriptVerifyFlags::P2SH,
            params(),
            1_100,
            open_bitcoin_mempool::RelayIntent::Requested,
        )
        .expect("admission");
    assert!(matches!(
        admission,
        open_bitcoin_mempool::MempoolOutcome::Accepted { .. }
    ));
    assert_eq!(
        runtime
            .network
            .mempool_info()
            .expect("before")
            .transaction_count,
        1
    );
    let before = runtime
        .network
        .operator_snapshot()
        .expect("before projection");
    runtime
        .network
        .force_basic_index_flush_for_test()
        .expect("due policy");
    runtime.store().set_basic_filter_fault(
        crate::storage::fjall_store::filters::FilterPublicationFault::BeforeCoins,
    );
    // Act
    let result = runtime
        .network
        .connect_local_block(&block, ScriptVerifyFlags::P2SH, params());
    // Assert
    assert!(result.is_err());
    assert_eq!(
        runtime
            .network
            .mempool_info()
            .expect("core accepted removal")
            .transaction_count,
        0
    );
    let after = runtime
        .network
        .operator_snapshot()
        .expect("dependent accepted removal");
    assert_eq!(
        after.admission().cleared,
        before.admission().cleared + 1,
        "dependent evidence must follow the accepted core even on persist error"
    );
    assert_eq!(
        runtime
            .network
            .maybe_chain_tip()
            .expect("tip")
            .expect("accepted")
            .height,
        2
    );
    drop(runtime);
    history.cleanup();
}
