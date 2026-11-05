// Parity breadcrumbs:
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/txdb.cpp

use super::*;
use crate::storage::{filter_index as filters, fjall_store::tracked::Keyspace};
use crate::{DurableSyncRuntime, SyncNetwork, SyncRuntimeConfig};
use open_bitcoin_core::chainstate::CoinsView;
use open_bitcoin_core::consensus::{CanonicalChain, block_hash};
use std::cell::RefCell;
use std::sync::{Arc, Barrier};

type CaptureAction = Box<dyn FnOnce(&FjallNodeStore)>;
thread_local! {
    static CAPTURE_ACTION: RefCell<Option<(usize, CaptureAction)>> = const {RefCell::new(None)};
}

fn on_capture(skip: usize, action: impl FnOnce(&FjallNodeStore) + 'static) {
    CAPTURE_ACTION.with_borrow_mut(|maybe_action| {
        assert!(maybe_action.is_none(), "previous capture action consumed");
        *maybe_action = Some((skip, Box::new(action)));
    });
}

pub(super) fn during_capture(store: &FjallNodeStore) {
    let maybe_action = CAPTURE_ACTION.with_borrow_mut(|maybe_action| {
        if let Some((skip, _)) = maybe_action.as_mut()
            && *skip != 0
        {
            *skip -= 1;
            return None;
        }
        maybe_action.take().map(|(_, action)| action)
    });
    if let Some(action) = maybe_action {
        action(store);
    }
}

fn keyspace(store: &FjallNodeStore, name: &str) -> Keyspace {
    store
        .database()
        .keyspace(name, fjall::KeyspaceCreateOptions::default)
        .expect("existing namespace")
}

fn rows(store: &FjallNodeStore) -> Namespaces {
    store
        .inspect_submission_inventory()
        .expect("complete all9 maps")
        .rows
}

fn rewrite(store: &FjallNodeStore) {
    let observed = store.inspect_submission_inventory().expect("stable bytes");
    let (key, bytes) = observed.rows["schema"]
        .first_key_value()
        .expect("schema row");
    keyspace(store, "schema")
        .insert(key.clone(), bytes.clone())
        .expect("actual identical insert");
}

fn physical_only(store: &FjallNodeStore) {
    let rows = store
        .inspect_submission_inventory()
        .expect("before maintenance");
    let before = store.database().visible_seqno();
    keyspace(store, "block_index")
        .rotate_memtable_and_wait()
        .expect("actual rotation");
    let after = store.database().visible_seqno();
    let achieved = store
        .inspect_submission_inventory()
        .expect("after maintenance");
    assert!(after > before, "actual physical sequence advanced");
    assert_eq!(rows.catalog, achieved.catalog);
    assert_eq!(rows.rows, achieved.rows, "all nine namespace maps");
    rows.check_current_source()
        .expect("logical observation still current");
    eprintln!(
        "INVENTORY_PHYSICAL before={before} after={after} all9_maps_equal=true catalog_equal=true"
    );
}

#[test]
fn phase167_submission_inventory_logical_freshness_physical_only_is_current() {
    genuine::with_manager(|manager, store, _| {
        // Arrange / Act / Assert
        physical_only(&store);
        drop(manager);
    });
}

#[test]
fn phase167_submission_inventory_logical_freshness_restored_rows_are_stale() {
    for case in [
        "identical",
        "aba",
        "empty-add-delete",
        "nonexistent-delete",
        "clear-restore",
        "catalog-recreate",
    ] {
        genuine::with_manager(|manager, store, _| {
            // Arrange
            let observed = store
                .inspect_submission_inventory()
                .expect("complete original source");
            let schema = keyspace(&store, "schema");
            let runtime = keyspace(&store, "runtime");
            let (key, bytes) = observed.rows["schema"]
                .first_key_value()
                .expect("schema row");
            assert!(observed.rows["runtime"].is_empty());
            // Act
            match case {
                "identical" => schema
                    .clone()
                    .insert(key.clone(), bytes.clone())
                    .expect("same-value clone write"),
                "aba" => {
                    schema.insert(key.clone(), b"intervening").expect("B");
                    schema
                        .insert(key.clone(), bytes.clone())
                        .expect("A restored");
                }
                "empty-add-delete" => {
                    runtime.insert("snapshot", b"temporary").expect("add");
                    runtime.remove("snapshot").expect("delete");
                }
                "nonexistent-delete" => {
                    runtime.remove("missing").expect("actual absent delete");
                    runtime
                        .rotate_memtable_and_wait()
                        .expect("actual maintenance");
                }
                "clear-restore" => {
                    schema.clear().expect("actual clear");
                    for (key, bytes) in &observed.rows["schema"] {
                        schema
                            .insert(key.clone(), bytes.clone())
                            .expect("restore all rows");
                    }
                }
                "catalog-recreate" => {
                    store
                        .database()
                        .clone()
                        .delete_keyspace(runtime)
                        .expect("delete required empty namespace");
                    drop(keyspace(&store, "runtime"));
                }
                _ => unreachable!("closed mutation cases"),
            }
            let achieved = store
                .inspect_submission_inventory()
                .expect("new stable complete capture");
            // Assert
            assert_eq!(observed.catalog, achieved.catalog, "{case}");
            assert_eq!(observed.rows, achieved.rows, "{case}: all9 rows restored");
            assert_eq!(observed.source_digest(), achieved.source_digest(), "{case}");
            assert!(
                observed.check_current_source().is_err(),
                "{case}: equality cannot forgive a write"
            );
            observed
                .compare_achieved(&store)
                .expect("fresh achieved equality permits expected own writes");
            achieved.check_current_source().expect("new idle baseline");
            drop(manager);
        });
    }
}

#[test]
fn phase167_submission_inventory_logical_freshness_foreign_open_and_clone_identity() {
    genuine::with_manager(|manager, store, _| {
        // Arrange
        let observed = store
            .inspect_submission_inventory()
            .expect("held observation");
        let stamp = store.database().stamp().expect("original stamp");
        // Act / Assert
        store
            .database()
            .clone()
            .check_stamp(&stamp)
            .expect("same actual open clone");
        genuine::with_manager(|other_manager, other, _| {
            assert_eq!(
                observed.rows,
                rows(&other),
                "foreign source bytes also equal"
            );
            assert!(
                other.database().check_stamp(&stamp).is_err(),
                "foreign identity refuses"
            );
            assert!(
                observed.compare_achieved(&other).is_err(),
                "foreign achieved store refuses"
            );
            drop(other_manager);
        });
        observed
            .check_current_source()
            .expect("foreign activity cannot alter original");
        drop(manager);
    });
}

#[test]
fn phase167_submission_inventory_logical_freshness_capture_and_reread_writes_refuse() {
    genuine::with_manager(|manager, store, _| {
        // Arrange / Act / Assert: consume hook before recursive byte capture.
        on_capture(0, rewrite);
        assert!(
            store.inspect_submission_inventory().is_err(),
            "same-value during initial capture"
        );
        let observed = store
            .inspect_submission_inventory()
            .expect("fresh original");
        on_capture(0, rewrite);
        assert!(
            observed.check_current_source().is_err(),
            "same-value during actual digest reread"
        );
        let current = store
            .inspect_submission_inventory()
            .expect("fresh baseline");
        on_capture(0, physical_only);
        current
            .check_current_source()
            .expect("physical rotation during reread remains benign");
        drop(manager);
    });
}

struct BlockingValue {
    entered: Arc<Barrier>,
    resume: Arc<Barrier>,
    bytes: Vec<u8>,
}

impl From<BlockingValue> for fjall::UserValue {
    fn from(value: BlockingValue) -> Self {
        value.entered.wait();
        value.resume.wait();
        value.bytes.into()
    }
}

#[test]
fn phase167_submission_inventory_logical_freshness_inflight_native_writer_refuses() {
    genuine::with_manager(|manager, store, _| {
        // Arrange: native value conversion pauses AFTER tracked BEGIN, before journal I/O.
        let observed = store.inspect_submission_inventory().expect("old source");
        let (key, bytes) = observed.rows["schema"].first_key_value().expect("schema");
        let schema = keyspace(&store, "schema");
        let entered = Arc::new(Barrier::new(2));
        let resume = Arc::new(Barrier::new(2));
        let value = BlockingValue {
            entered: Arc::clone(&entered),
            resume: Arc::clone(&resume),
            bytes: bytes.clone(),
        };
        let key = key.clone();
        let writer = std::thread::spawn(move || schema.insert(key, value));
        entered.wait();
        // Act: save results; ALWAYS release and join before assertions can unwind.
        let stamp_refused = store.database().stamp().is_err();
        let old_refused = observed.check_current_source().is_err();
        let capture_refused = store.inspect_submission_inventory().is_err();
        resume.wait();
        let result = writer.join();
        // Assert
        result
            .expect("writer joins")
            .expect("actual same-value native insert");
        assert!(stamp_refused && old_refused && capture_refused);
        assert!(
            observed.check_current_source().is_err(),
            "finished write remains stale"
        );
        let fresh = store
            .inspect_submission_inventory()
            .expect("idle new capture");
        assert_eq!(fresh.rows, observed.rows);
        fresh
            .check_current_source()
            .expect("active writer cleaned up");
        drop(manager);
    });
}

fn config() -> SyncRuntimeConfig {
    SyncRuntimeConfig {
        network: SyncNetwork::Regtest,
        ..Default::default()
    }
}

fn temporary() -> std::path::PathBuf {
    let mut nonce = [0; 16];
    getrandom::fill(&mut nonce).expect("nonce");
    std::fs::canonicalize(std::env::temp_dir())
        .expect("physical parent")
        .join(format!(
            "phase167-inventory-freshness-{:x}",
            u128::from_le_bytes(nonce)
        ))
}

fn wire(block: &Block) -> open_bitcoin_core::codec::block::wire::WireBlock {
    let bytes = open_bitcoin_core::codec::encode_block(block).expect("complete body");
    let (raw, used) = open_bitcoin_core::codec::block::wire::parse_submission_block_prefix(&bytes)
        .expect("raw prefix");
    assert_eq!(used, bytes.len());
    raw
}

fn owned_state(runtime: &DurableSyncRuntime) -> impl std::fmt::Debug + PartialEq + use<> {
    let handle = runtime.network_handle();
    let store = runtime.store();
    (
        handle
            .chainstate_snapshot()
            .expect("complete effective accepted state"),
        store
            .coins_view()
            .complete_snapshot()
            .expect("durable Coins"),
        store.maybe_basic_filter_state().expect("stored BASIC"),
        handle
            .maybe_basic_index_progress()
            .expect("live BASIC")
            .map(|progress| {
                (
                    progress.generation(),
                    progress.accepted_target(),
                    progress.maybe_processed_endpoint(),
                    progress.maybe_safe_durable_endpoint(),
                    progress.protection(),
                    progress.initially_synchronized(),
                    progress.state(),
                )
            }),
        handle.mempool_info().expect("actual owned mempool"),
    )
}

fn assert_basic(runtime: &DurableSyncRuntime, block: &Block, mode: BasicFilterStartupMode) {
    let store = runtime.store();
    let handle = runtime.network_handle();
    let snapshot = handle
        .chainstate_snapshot()
        .expect("complete accepted snapshot");
    let position = snapshot.tip().expect("tip");
    assert_eq!(position.header, block.header);
    let captured = store
        .inspect_submission_inventory()
        .expect("all9 supported maps");
    assert_eq!(captured.rows.len(), 9);
    captured.check_current_source().expect("stable source");
    captured
        .compare_achieved(store)
        .expect("whole achieved map gate");
    store
        .check_submission_physical_inventory()
        .expect("final actual physical gate");
    if mode == BasicFilterStartupMode::Disabled {
        assert!(
            handle
                .maybe_basic_index_progress()
                .expect("disabled progress")
                .is_none()
        );
        assert!(
            !captured.rows["block_index"]
                .keys()
                .any(|key| key.starts_with(b"basic_filter:v1:record:"))
        );
        return;
    }
    let progress = handle
        .maybe_basic_index_progress()
        .expect("progress")
        .expect("enabled owner");
    let root = block_hash(&CanonicalChain::Regtest.spec().genesis_header());
    assert_eq!(
        progress.branch_identity(),
        root,
        "same incarnation retains actual startup anchor"
    );
    assert_eq!(
        progress
            .maybe_safe_durable_endpoint()
            .expect("durable endpoint")
            .block_hash(),
        position.block_hash
    );
    let codec = &captured.rows["block_index"];
    let parent_key = filters::record_key(position.previous_block_hash());
    let parent = filters::decode_record(
        &parent_key,
        codec
            .get(parent_key.as_bytes())
            .expect("actual predecessor bytes"),
        None,
    )
    .expect("complete predecessor");
    let inputs = BasicFilterInputs::from_historical(
        block,
        position,
        Some(HistoricalBlockUndo {
            block_hash: position.block_hash,
            undo: snapshot
                .undo_by_block
                .get(&position.block_hash)
                .expect("genuine complete accepted undo"),
        }),
    )
    .expect("genuine filter inputs");
    let expected =
        filters::StoredFilterRecord::generate(&inputs, position, Some(&parent.identity()))
            .expect("independent full BASIC oracle");
    let key = filters::record_key(position.block_hash);
    let actual = filters::decode_record(
        &key,
        codec.get(key.as_bytes()).expect("actual immutable record"),
        Some(&parent.identity()),
    )
    .expect("full actual record");
    assert_eq!(actual, expected, "all StoredFilterRecord fields and bytes");
    let epoch = store
        .with_basic_filter_query(|reader| reader.integrity_epoch())
        .expect("actual fresh read epoch");
    assert!(
        store
            .basic_peer_epoch_is_current(&epoch)
            .expect("same publication epoch")
    );
}

#[test]
fn phase167_submission_inventory_logical_freshness_command_modes_maintenance_reopen_and_next() {
    for mode in [
        BasicFilterStartupMode::Disabled,
        BasicFilterStartupMode::Enabled,
    ] {
        // Arrange: only genuine public factories and inferred command/outcome APIs.
        let path = temporary();
        let opened = FjallNodeStore::open_canonical_genesis(&path, CanonicalChain::Regtest)
            .expect("proved genesis");
        let (runtime, maybe_command) =
            DurableSyncRuntime::open_genesis_submission_configured(opened, config(), mode)
                .expect("actual configured genesis owner");
        let command = maybe_command.expect("Regtest command");
        let genesis = CanonicalChain::Regtest.spec().genesis_block();
        let block = genuine::child(&genesis.header, 1);
        let old = runtime
            .store()
            .database()
            .stamp()
            .expect("pre-own-write observation");
        on_capture(0, physical_only);
        // Act: physical maintenance occurs inside the actual command observation.
        let position = command
            .submit(wire(&block))
            .expect("real maintained command succeeds")
            .fold(
                |position| position,
                |_| panic!("duplicate"),
                |reason| panic!("{reason}"),
                || panic!("inconclusive"),
            );
        // Assert
        assert_eq!(
            position,
            runtime
                .network_handle()
                .maybe_chain_tip()
                .expect("accepted tip")
                .expect("present")
        );
        assert!(
            runtime.store().database().check_stamp(&old).is_err(),
            "actual own writes require fresh achieved capture"
        );
        assert_basic(&runtime, &block, mode);
        let before = owned_state(&runtime);
        let before_rows = rows(runtime.store());
        drop(old);
        drop(command);
        drop(runtime);
        let (runtime, maybe_command) =
            DurableSyncRuntime::open_canonical_submission_configured(&path, config(), mode)
                .expect("all-handle configured safe reopen");
        let command = maybe_command.expect("new actual command");
        assert_eq!(owned_state(&runtime), before);
        assert_eq!(rows(runtime.store()), before_rows, "complete reopened maps");
        if mode == BasicFilterStartupMode::Enabled {
            assert_eq!(
                runtime
                    .network_handle()
                    .maybe_basic_index_progress()
                    .expect("fresh progress")
                    .expect("enabled")
                    .branch_identity(),
                position.block_hash,
                "reopened owner has actual new recovery anchor"
            );
        }
        let next = genuine::child(&block.header, 2);
        command
            .submit(wire(&next))
            .expect("valid next in new incarnation")
            .fold(
                |position| assert_eq!(position.height, 2),
                |_| panic!("duplicate"),
                |reason| panic!("{reason}"),
                || panic!("inconclusive"),
            );
        assert_eq!(
            runtime
                .store()
                .load_block(block_hash(&next.header))
                .expect("actual durable next"),
            Some(next)
        );
        drop(command);
        drop(runtime);
        std::fs::remove_dir_all(path).expect("all actual owners released");
    }
}

#[test]
fn phase167_submission_inventory_logical_freshness_command_conflicts_preserve_honest_state() {
    for mode in [
        BasicFilterStartupMode::Disabled,
        BasicFilterStartupMode::Enabled,
    ] {
        for late in [false, true] {
            // Arrange
            let path = temporary();
            let (runtime, maybe_command) =
                DurableSyncRuntime::open_canonical_submission_configured(&path, config(), mode)
                    .expect("actual owner");
            let command = maybe_command.expect("Regtest command");
            let genesis = CanonicalChain::Regtest.spec().genesis_block();
            let block = genuine::child(&genesis.header, 1);
            let before = owned_state(&runtime);
            let before_rows = rows(runtime.store());
            // Five actual captures: early, early reread, prepared, prepared reread, achieved.
            on_capture(if late { 4 } else { 0 }, rewrite);
            // Act
            let result = command.submit(wire(&block));
            // Assert: operational error, never a consensus rejection or fake rollback.
            assert!(
                result.is_err(),
                "actual logical conflict is operational: {result:?}"
            );
            CAPTURE_ACTION.with_borrow(|maybe_action| {
                assert!(maybe_action.is_none(), "actual scheduled capture ran")
            });
            if late {
                assert_ne!(
                    owned_state(&runtime),
                    before,
                    "post-absorption state stays advanced"
                );
                assert_eq!(
                    runtime
                        .network_handle()
                        .maybe_chain_tip()
                        .expect("advanced tip")
                        .expect("tip")
                        .height,
                    1
                );
                assert_basic(&runtime, &block, mode);
            } else {
                assert_eq!(
                    owned_state(&runtime),
                    before,
                    "pre-effect accepted state unchanged"
                );
                assert_eq!(rows(runtime.store()), before_rows);
            }
            let expected = owned_state(&runtime);
            drop(result);
            drop(command);
            drop(runtime);
            let (runtime, maybe_command) =
                DurableSyncRuntime::open_canonical_submission_configured(&path, config(), mode)
                    .expect("actual safe source after drop-all");
            let command = maybe_command.expect("reopened command");
            assert_eq!(owned_state(&runtime), expected);
            let next = if late {
                genuine::child(&block.header, 2)
            } else {
                block.clone()
            };
            command
                .submit(wire(&next))
                .expect("valid next from honest durable state")
                .fold(
                    |position| assert_eq!(position.header, next.header),
                    |_| panic!("duplicate"),
                    |reason| panic!("{reason}"),
                    || panic!("inconclusive"),
                );
            drop(command);
            drop(runtime);
            std::fs::remove_dir_all(path).expect("cleanup");
        }
    }
}
