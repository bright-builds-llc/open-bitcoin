// Parity breadcrumbs:
// - packages/bitcoin-knots/src/index/base.cpp
// - packages/bitcoin-knots/src/index/blockfilterindex.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp

//! Real effect faults on accepted normal-maturity active and displaced history.

use super::{missing_history::*, *};
use crate::storage::fjall_store::filters::FilterPublicationFault;
use core_types::chainstate::AnchoredBlock;
use open_bitcoin_mempool::{PolicyTime, ReorgLifecycleContext};

pub(super) fn saved_with_stale(cleanup: &Cleanup) -> (AcceptedRecipe, StoredFilterRecord) {
    let mut fixture =
        AcceptedRecipe::at(cleanup.path.clone(), 102, BasicFilterStartupMode::Enabled);
    fixture.finish();
    fixture.flush();
    let branch = fixture.recipe.fork(101, 1, 0x63);
    let block = &branch[0];
    fixture
        .runtime
        .store()
        .save_block(block, PersistMode::Sync)
        .expect("actual alternate body");
    fixture
        .runtime
        .network_handle()
        .connect_local_block(block, fixture.recipe.flags, fixture.recipe.params)
        .expect("genuine alternate receipt");
    fixture.finish();
    let stale = fixture
        .runtime
        .store()
        .load_basic_filter_record(block_hash(&block.header))
        .expect("alternate row")
        .expect("indexed alternate");
    let accepted_undo = fixture
        .runtime
        .network_handle()
        .chainstate_snapshot()
        .expect("actual alternate undo")
        .undo_by_block[&block_hash(&block.header)]
        .clone();
    fixture
        .runtime
        .store()
        .save_undo(block_hash(&block.header), &accepted_undo, PersistMode::Sync)
        .expect("persist actual displaced mate without changing coins authority");
    let recipe = recipe::Recipe::new(fixture.recipe.params, fixture.recipe.flags, 105);
    let connected: Vec<_> = recipe.blocks[102..]
        .iter()
        .map(|block| {
            fixture
                .runtime
                .store()
                .save_block(block, PersistMode::Sync)
                .expect("ordinary replacement body");
            AnchoredBlock {
                block: block.clone(),
                chain_work: u128::from(fixture.receipts.len() as u32) + 1000,
            }
        })
        .collect();
    fixture
        .runtime
        .network_handle()
        .reorg_to_branch(
            &branch,
            &connected,
            ReorgLifecycleContext::new(PolicyTime::from_unix_seconds(50_000)),
            recipe.flags,
            recipe.params,
        )
        .expect("ordinary genuine shallow reorg");
    fixture.receipts = fixture
        .runtime
        .network_handle()
        .chainstate_snapshot()
        .expect("accepted replacement positions")
        .active_chain;
    fixture.recipe = recipe;
    assert_eq!(fixture.receipts.len(), 105);
    assert_authority(fixture.runtime.store(), &fixture.receipts, 101);
    assert_eq!(
        fixture
            .runtime
            .store()
            .load_basic_filter_record(stale.identity().block_hash())
            .expect("retained stale row"),
        Some(stale.clone())
    );
    (fixture, stale)
}

pub(super) fn raw(store: &FjallNodeStore) -> Vec<Vec<(Vec<u8>, Vec<u8>)>> {
    store.basic_operator_namespaces_for_test()[1..4].to_vec()
}

fn immutable(rows: &[Vec<(Vec<u8>, Vec<u8>)>]) -> Vec<(Vec<u8>, Vec<u8>)> {
    rows[0]
        .iter()
        .filter(|(key, _)| key.starts_with(codec::RECORD_PREFIX.as_bytes()))
        .cloned()
        .collect()
}

pub(super) fn assert_pairs(
    store: &FjallNodeStore,
    positions: &[ChainPosition],
    expected_undo: usize,
) {
    for position in positions {
        assert!(
            store
                .has_block(position.block_hash)
                .expect("retained physical body")
        );
        if position.height > 0 {
            assert_eq!(
                store.has_undo(position.block_hash).expect("physical undo"),
                position.height as usize <= expected_undo
            );
        }
    }
    assert_eq!(
        store.maybe_prune_intent().expect("no invented intent"),
        None
    );
    assert!(!store.load_have_pruned().expect("no invented pruning"));
}

fn assert_poison(fixture: &AcceptedRecipe) {
    let network = fixture.runtime.network_handle();
    assert!(network.drive_basic_filter_index_turn().is_err());
    assert!(
        network
            .basic_filter_query(fixture.receipts[20].block_hash)
            .is_err()
    );
    assert!(
        fixture
            .runtime
            .store()
            .disable_basic_filter_index()
            .is_err(),
        "poison cannot release reserved protection"
    );
    assert!(network.clear_prune_lock(BASIC_INDEX_PRUNE_LOCK).is_err());
}

fn coins_fault(fault: FilterPublicationFault) {
    // Arrange: genuine saved h101, actual accepted replacement h102..104, stale row.
    let cleanup = Cleanup::new();
    let (fixture, stale) = saved_with_stale(&cleanup);
    fixture.finish();
    let store = fixture.runtime.store();
    let before = raw(store);
    let maybe_headers_before = store.load_header_entries().expect("saved trusted headers");
    let old_rows = rows(store, &fixture.receipts);
    let maybe_old_checkpoint = store.maybe_basic_filter_checkpoint().expect("safe101");
    let old_locks = store.load_prune_locks().expect("protected102");
    let maybe_old_owner = store
        .maybe_basic_filter_lifecycle_for_test()
        .expect("owner");
    let progress = fixture
        .runtime
        .network_handle()
        .maybe_basic_index_progress()
        .expect("owner")
        .expect("enabled");
    assert_pairs(store, &fixture.receipts, 101);
    store.set_basic_filter_fault(fault);
    // Act: ordinary Always reaches actual undo/coins/metadata effects.
    let result = fixture.runtime.network_handle().flush_coins(
        FlushMode::Always,
        FlushPolicyTime::from_unix_seconds(500_000),
        u64::MAX,
    );
    // Assert: public adapter retains its typed LifecycleEffect category.
    assert!(matches!(
        result,
        Err(crate::ManagedNetworkAuthorityError::LifecycleEffect(_))
    ));
    assert_poison(&fixture);
    let maybe_headers_after = store
        .load_header_entries()
        .expect("actual selected header prefix");
    assert_eq!(
        maybe_headers_after,
        store
            .load_block_index_entries()
            .expect("atomic paired header snapshot")
    );
    if fault == FilterPublicationFault::BeforeUndo {
        assert_eq!(maybe_headers_after, maybe_headers_before);
    } else {
        assert_header_prefix(&fixture, &stale);
    }
    let changed = fault == FilterPublicationFault::BeforeChainMeta;
    assert_eq!(
        store.coins_view().best_block().expect("actual B"),
        Some(fixture.receipts[if changed { 104 } else { 101 }].block_hash)
    );
    assert_eq!(
        store
            .load_chain_meta_for_open()
            .expect("independent metadata")
            .0,
        fixture.receipts[..=101]
    );
    assert_eq!(
        store.maybe_basic_filter_checkpoint().expect("safe"),
        maybe_old_checkpoint
    );
    assert_eq!(store.load_prune_locks().expect("protection"), old_locks);
    assert_eq!(
        store
            .maybe_basic_filter_lifecycle_for_test()
            .expect("owner"),
        maybe_old_owner
    );
    assert_eq!(rows(store, &fixture.receipts), old_rows);
    assert_eq!(immutable(&raw(store)), immutable(&before));
    assert_eq!(
        store
            .load_basic_filter_record(stale.identity().block_hash())
            .expect("stale"),
        Some(stale)
    );
    assert_eq!(
        fixture
            .runtime
            .network_handle()
            .maybe_basic_index_progress()
            .expect("retained facts")
            .expect("owner")
            .maybe_safe_durable_endpoint(),
        progress.maybe_safe_durable_endpoint()
    );
    let undo_height = if fault == FilterPublicationFault::BeforeUndo {
        101
    } else {
        104
    };
    assert_pairs(store, &fixture.receipts, undo_height);
    let positions = fixture.receipts.clone();
    let after = raw(store);
    drop(fixture);
    if changed {
        assert_refusals(
            &cleanup.path,
            "coins best block differs from durable metadata",
        );
    } else {
        let runtime = open(&cleanup.path).expect("old actual coins prefix recovers");
        assert_authority(runtime.store(), &positions, 101);
        assert_eq!(
            runtime
                .store()
                .maybe_basic_filter_checkpoint()
                .expect("saved safe"),
            maybe_old_checkpoint
        );
        assert_eq!(
            runtime
                .store()
                .load_prune_locks()
                .expect("saved protection"),
            old_locks
        );
        for position in &positions[102..] {
            assert_eq!(
                runtime
                    .store()
                    .maybe_active_basic_filter_record(position.height)
                    .expect("inactive ahead rows"),
                None
            );
        }
        drop(runtime);
    }
    assert_eq!(
        snapshot(&cleanup.path),
        after,
        "closed recovery never rewrites durable facts"
    );
    let store = FjallNodeStore::open(&cleanup.path).expect("post-reopen inspection");
    assert_pairs(&store, &positions, undo_height);
    assert_eq!(
        store
            .load_header_entries()
            .expect("closed selected header prefix"),
        maybe_headers_after
    );
    drop(store);
    eprintln!(
        "phase162 publication coins: boundary={fault:?} coins_changed={changed} metadata_height=101 safe_height=101 prefix_preserved=true"
    );
}

fn assert_header_prefix(fixture: &AcceptedRecipe, stale: &StoredFilterRecord) {
    let headers = fixture
        .runtime
        .store()
        .load_header_entries()
        .expect("selected header prefix")
        .expect("prefix completed before coins");
    for position in &fixture.receipts {
        assert!(
            headers
                .entries
                .iter()
                .any(|entry| entry.block_hash == position.block_hash)
        );
    }
    assert!(
        headers
            .entries
            .iter()
            .any(|entry| entry.block_hash == stale.identity().block_hash())
    );
}

fn index_fault(fault: FilterPublicationFault) {
    // Arrange: settle genuine coins h104, keep saved BASIC checkpoint h101.
    let cleanup = Cleanup::new();
    let (fixture, stale) = saved_with_stale(&cleanup);
    flush_only(&fixture.runtime);
    let store = fixture.runtime.store();
    let before = raw(store);
    let maybe_old_checkpoint = store.maybe_basic_filter_checkpoint().expect("safe101");
    let old_locks = store.load_prune_locks().expect("protected102");
    let maybe_old_owner = store
        .maybe_basic_filter_lifecycle_for_test()
        .expect("owner");
    let old_rows = rows(store, &fixture.receipts);
    let expected_rows = expected_complete_rows(&fixture, old_rows.clone());
    let old_progress = fixture
        .runtime
        .network_handle()
        .maybe_basic_index_progress()
        .expect("progress")
        .expect("owner");
    assert_eq!(old_rows.len(), 102);
    store.set_basic_filter_fault(fault);
    // Act: actual ordinary append builds its real atomic records/state/lock batch.
    let result = fixture
        .runtime
        .network_handle()
        .drive_basic_filter_index_turn();
    // Assert
    assert!(matches!(
        result,
        Err(crate::ManagedNetworkAuthorityError::LifecycleEffect(_))
    ));
    assert_poison(&fixture);
    assert_authority(store, &fixture.receipts, 104);
    assert_pairs(store, &fixture.receipts, 104);
    assert_eq!(
        store
            .maybe_basic_filter_lifecycle_for_test()
            .expect("owner"),
        maybe_old_owner
    );
    assert_eq!(
        store
            .load_basic_filter_record(stale.identity().block_hash())
            .expect("stale"),
        Some(stale)
    );
    let current = rows(store, &fixture.receipts);
    let committed = fault == FilterPublicationFault::AfterCommit;
    assert_eq!(current.len(), if committed { 105 } else { 102 });
    assert_eq!(current[..102], old_rows);
    let maybe_expected_checkpoint = if committed {
        Some(checkpoint(&current[104]))
    } else {
        maybe_old_checkpoint
    };
    let expected_locks = if committed {
        vec![
            IndexInputProtection::FromHeight(105)
                .maybe_prune_lock()
                .expect("committed protected105"),
        ]
    } else {
        old_locks
    };
    assert_eq!(
        store
            .maybe_basic_filter_checkpoint()
            .expect("actual committed safe"),
        maybe_expected_checkpoint
    );
    assert_eq!(
        store
            .load_prune_locks()
            .expect("actual committed protection"),
        expected_locks
    );
    if !committed {
        assert_eq!(raw(store), before, "no partial durable batch");
    }
    let progress = fixture
        .runtime
        .network_handle()
        .maybe_basic_index_progress()
        .expect("failed acknowledgement facts")
        .expect("owner");
    assert_eq!(
        progress.maybe_processed_endpoint(),
        old_progress.maybe_processed_endpoint()
    );
    assert_eq!(
        progress.maybe_safe_durable_endpoint(),
        old_progress.maybe_safe_durable_endpoint()
    );
    let positions = fixture.receipts.clone();
    let after = raw(store);
    drop(fixture);
    index_reopen(&cleanup.path, &positions, &expected_rows, &after, committed);
    eprintln!(
        "phase162 publication index: boundary={fault:?} committed={committed} successful_ack=false durable_rows={} prefix_preserved=true",
        current.len()
    );
}

fn expected_complete_rows(
    fixture: &AcceptedRecipe,
    mut records: Vec<StoredFilterRecord>,
) -> Vec<StoredFilterRecord> {
    for position in &fixture.receipts[records.len()..] {
        let undo = fixture
            .runtime
            .store()
            .load_undo(position.block_hash)
            .expect("actual persisted source undo")
            .expect("required undo");
        let inputs = BasicFilterInputs::from_historical(
            &fixture.recipe.blocks[position.height as usize],
            position,
            Some(HistoricalBlockUndo {
                block_hash: position.block_hash,
                undo: &undo,
            }),
        )
        .expect("actual normal-maturity historical/same-block inputs");
        let previous = records.last().expect("valid prefix").identity();
        records.push(
            StoredFilterRecord::generate(&inputs, position, Some(&previous))
                .expect("exact source-derived candidate"),
        );
    }
    records
}

fn index_reopen(
    path: &Path,
    positions: &[ChainPosition],
    expected: &[StoredFilterRecord],
    after: &[Vec<(Vec<u8>, Vec<u8>)>],
    committed: bool,
) {
    let mut expected_union: std::collections::BTreeMap<_, _> =
        immutable(after).into_iter().collect();
    for record in expected {
        expected_union.insert(
            codec::record_key(record.identity().block_hash()).into_bytes(),
            codec::encode_record(record),
        );
    }
    let runtime = open(path).expect("configured reopen and genuine bounded startup catch-up");
    assert_authority(runtime.store(), positions, 104);
    assert!(
        rows(runtime.store(), positions) == expected,
        "exact accepted-source rows including startup retry"
    );
    assert_eq!(
        runtime
            .store()
            .maybe_basic_filter_checkpoint()
            .expect("recovered safe104"),
        Some(checkpoint(&expected[104]))
    );
    assert_eq!(
        runtime
            .store()
            .load_prune_locks()
            .expect("recovered protected105"),
        vec![
            IndexInputProtection::FromHeight(105)
                .maybe_prune_lock()
                .expect("lock")
        ]
    );
    assert_pairs(runtime.store(), positions, 104);
    let settled = raw(runtime.store());
    assert!(
        immutable(&settled)
            .into_iter()
            .collect::<std::collections::BTreeMap<_, _>>()
            == expected_union,
        "exact active and retained stale immutable union"
    );
    if committed {
        assert!(
            settled == after,
            "already committed batch needs no new effects"
        );
    } else {
        let mut allowed = vec![codec::STATE_KEY.to_owned(), "prune_locks".to_owned()];
        for position in &positions[102..] {
            allowed.push(codec::record_key(position.block_hash));
            allowed.push(codec::active_key(position.height));
        }
        let unaffected = |rows: &[(Vec<u8>, Vec<u8>)]| {
            rows.iter()
                .filter(|(key, _)| !allowed.iter().any(|allowed| allowed.as_bytes() == key))
                .cloned()
                .collect::<Vec<_>>()
        };
        assert!(
            unaffected(&settled[0]) == unaffected(&after[0]),
            "startup changes only exact suffix and committed checkpoint/protection"
        );
        assert!(
            settled[1..] == after[1..],
            "coins/metadata/physical undo unchanged"
        );
    }
    drop(runtime);
    let runtime = open(path).expect("second stable cold reopen");
    assert!(rows(runtime.store(), positions) == expected);
    drop(runtime);
    assert!(snapshot(path) == settled, "second reopen is byte stable");
}

#[test]
fn phase162_publication_fault_before_undo() {
    coins_fault(FilterPublicationFault::BeforeUndo);
}
#[test]
fn phase162_publication_fault_before_coins() {
    coins_fault(FilterPublicationFault::BeforeCoins);
}
#[test]
fn phase162_publication_fault_before_chain_meta() {
    coins_fault(FilterPublicationFault::BeforeChainMeta);
}
#[test]
fn phase162_publication_fault_before_records() {
    index_fault(FilterPublicationFault::BeforeRecords);
}
#[test]
fn phase162_publication_fault_before_checkpoint() {
    index_fault(FilterPublicationFault::BeforeCheckpoint);
}
#[test]
fn phase162_publication_fault_before_protection() {
    index_fault(FilterPublicationFault::BeforeProtection);
}
#[test]
fn phase162_publication_fault_before_commit() {
    index_fault(FilterPublicationFault::BeforeCommit);
}
#[test]
fn phase162_publication_fault_after_commit() {
    index_fault(FilterPublicationFault::AfterCommit);
}
