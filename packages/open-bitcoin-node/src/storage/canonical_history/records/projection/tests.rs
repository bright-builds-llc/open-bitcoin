// Parity breadcrumbs:
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/txdb.cpp

use super::*;
use crate::chainstate::BasicFilterStartupMode;
use crate::chainstate::validation_history::tests::phase167 as genuine;
use crate::storage::StorageNamespace;
use open_bitcoin_core::{
    chainstate::{BasicFilterInputs, HistoricalBlockUndo},
    consensus::ScriptVerifyFlags,
};

mod freshness;

thread_local! {static CAPTURE_WRITE: std::cell::Cell<bool> = const {std::cell::Cell::new(false)};}
pub(super) fn during_capture(store: &FjallNodeStore) {
    freshness::during_capture(store);
    if CAPTURE_WRITE.replace(false) {
        store
            .write_raw_for_test(StorageNamespace::Runtime, "snapshot", vec![1])
            .expect("capture interleave");
    }
}

#[test]
fn phase167_submission_inventory_actual_stale_source_write_is_refused() {
    genuine::with_manager(|manager, store, _| {
        // Arrange
        let observed = store
            .inspect_submission_inventory()
            .expect("actual capture");
        let before = observed.source_digest();
        store
            .write_raw_for_test(StorageNamespace::Runtime, "snapshot", vec![1])
            .expect("actual public store writer");
        // Act / Assert
        assert!(
            observed.check_current_source().is_err(),
            "stale capture cannot authorize effects"
        );
        assert_ne!(
            store
                .inspect_submission_inventory()
                .expect("changed capture")
                .source_digest(),
            before
        );
        drop(manager);
    });
}

#[test]
fn phase167_submission_inventory_actual_capture_same_store_and_all_namespaces() {
    genuine::with_manager(|manager, store, _| {
        // Arrange
        let observed = store
            .inspect_submission_inventory()
            .expect("actual capture");
        // Act / Assert
        assert_eq!(observed.rows.len(), 9);
        observed.check_current_source().expect("current");
        observed.into_achieved_check()(&store).expect("same complete bytes");
        store
            .check_submission_physical_inventory()
            .expect("actual physical envelope");
        drop(manager);
    });
}

#[test]
fn phase167_submission_inventory_single_policy_rejects_missing_unknown_and_bounds() {
    genuine::with_manager(|manager, store, _| {
        // Arrange
        let observed = store.inspect_submission_inventory().expect("actual rows");
        for namespace in observed.rows.keys() {
            let mut rows = observed.rows.clone();
            rows.remove(namespace);
            // Act / Assert: existing namespace gate remains separate from map policy.
            assert!(
                crate::storage::fjall_store::inspection::require_existing_namespace_catalog(
                    rows.keys().map(String::as_str)
                )
                .is_err()
            );
        }
        let mut rows = observed.rows.clone();
        rows.insert("foreign".to_owned(), Default::default());
        assert!(inventory(&rows).is_err());
        let mut rows = observed.rows.clone();
        rows.get_mut("block_index")
            .expect("present")
            .insert(b"unsupported".to_vec(), vec![1]);
        assert!(inventory(&rows).is_err());
        for length in [65536, 65537] {
            let mut rows = observed.rows.clone();
            rows.get_mut("block_index")
                .expect("present")
                .insert(b"block:boundary".to_vec(), vec![1; length]);
            assert_eq!(inventory(&rows).is_ok(), length == 65536);
        }
        for length in [4096, 4097] {
            let mut rows = observed.rows.clone();
            let mut key = b"wallet_snapshot:".to_vec();
            key.resize(length, b'x');
            rows.get_mut("wallet")
                .expect("present")
                .insert(key, vec![1]);
            assert_eq!(inventory(&rows).is_ok(), length == 4096);
        }
        drop(manager);
    });
}

#[test]
fn phase167_submission_inventory_capture_write_and_catalog_changes_refuse() {
    genuine::with_manager(|manager, store, _| {
        // Arrange / Act / Assert
        CAPTURE_WRITE.set(true);
        assert!(store.inspect_submission_inventory().is_err());
        let observed = store
            .inspect_submission_inventory()
            .expect("new stable capture");
        let foreign = store
            .database()
            .keyspace("foreign", fjall::KeyspaceCreateOptions::default)
            .expect("actual catalog change");
        assert!(observed.check_current_source().is_err());
        assert!(store.inspect_submission_inventory().is_err());
        drop(foreign);
        drop(manager);
    });
}

#[test]
fn phase167_submission_inventory_count_and_total_bytes_use_existing_policy() {
    genuine::with_manager(|manager, store, _| {
        // Arrange
        let observed = store.inspect_submission_inventory().expect("baseline");
        let mut rows = observed.rows.clone();
        let initial = rows.len() + rows.values().map(|rows| rows.len()).sum::<usize>();
        for index in initial..4096 {
            rows.get_mut("wallet")
                .expect("wallet")
                .insert(format!("wallet_snapshot:{index}").into_bytes(), vec![1]);
        }
        // Act / Assert
        assert!(inventory(&rows).is_ok());
        rows.get_mut("wallet")
            .expect("wallet")
            .insert(b"wallet_snapshot:extra".to_vec(), vec![1]);
        assert!(inventory(&rows).is_err());
        let mut rows = observed.rows.clone();
        let current = rows
            .iter()
            .map(|(name, rows)| {
                name.len()
                    + rows
                        .iter()
                        .map(|(key, value)| key.len() + value.len())
                        .sum::<usize>()
            })
            .sum::<usize>();
        let length = 536870912 - current - "snapshot".len();
        let mut bytes = vec![1; length + 1];
        bytes.truncate(length);
        rows.get_mut("runtime")
            .expect("runtime")
            .insert(b"snapshot".to_vec(), bytes);
        assert!(validate_rows(&rows).is_ok());
        rows.get_mut("runtime")
            .expect("runtime")
            .get_mut(b"snapshot".as_slice())
            .expect("bytes")
            .push(1);
        assert!(
            inventory(&rows).is_ok(),
            "unchanged row policy omits the trailing empty namespace cost"
        );
        assert!(
            validate_rows(&rows).is_err(),
            "actual reader budget checks every namespace and row"
        );
        drop(manager);
    });
}

#[test]
fn phase167_submission_inventory_foreign_store_and_unrelated_namespaces_are_explicit() {
    genuine::with_manager(|manager, store, _| {
        // Arrange
        for namespace in [
            StorageNamespace::Wallet,
            StorageNamespace::Metrics,
            StorageNamespace::Mempool,
            StorageNamespace::Runtime,
        ] {
            store
                .write_raw_for_test(namespace, "snapshot", vec![1, 2, 3])
                .expect("actual unrelated rows");
        }
        let observed = store
            .inspect_submission_inventory()
            .expect("complete capture");
        // Act / Assert
        for name in ["wallet", "metrics", "mempool", "runtime"] {
            assert_eq!(
                observed.rows[name].get(b"snapshot".as_slice()),
                Some(&vec![1, 2, 3])
            );
        }
        genuine::with_manager(|foreign, other, _| {
            assert!(
                observed.compare_achieved(&other).is_err(),
                "same shape cannot replace store identity"
            );
            drop(foreign);
        });
        observed.into_achieved_check()(&store).expect("all unrelated bytes retained");
        drop(manager);
    });
}

#[test]
fn phase167_submission_projection_complete_disabled_and_enabled_maps_match_actual_writes() {
    for mode in [
        BasicFilterStartupMode::Disabled,
        BasicFilterStartupMode::Enabled,
    ] {
        genuine::with_mode(mode, |manager, store, params| {
            // Arrange: obtain data from a real authorized stage before any absorption.
            let mut network = genuine::network(manager);
            let block = genuine::child(&network.maybe_chain_tip().expect("root").header, 1);
            genuine::admit(&mut network, std::slice::from_ref(&block), params);
            network
                .retain_checked_candidate(&block, params, 2_000_000_000)
                .expect("real body");
            let authorization = network
                .authorize_connect(&block, params)
                .expect("real authority");
            let prepared = network
                .chainstate()
                .prepare_authorized_connect(
                    authorization,
                    &block,
                    2_000_000_000,
                    ScriptVerifyFlags::NONE,
                    params,
                )
                .expect("real stage");
            let projected = network
                .chainstate()
                .project_prepared_submission(&prepared)
                .expect("complete prospective state");
            drop(prepared);
            let handle = crate::ManagedNetworkHandle::new(network);
            handle
                .initialize_basic_filter_index_owner()
                .expect("actual owner");
            handle
                .drive_basic_filter_index_turn()
                .expect("actual root generation");
            handle
                .persist_validation_header_snapshot()
                .expect("actual early header writes");
            let headers = handle.header_entries().expect("actual admitted headers");
            let position = projected.tip().expect("projected tip");
            let inputs = BasicFilterInputs::from_historical(
                &block,
                position,
                Some(HistoricalBlockUndo {
                    block_hash: position.block_hash,
                    undo: projected
                        .undo_by_block
                        .get(&position.block_hash)
                        .expect("genuine staged undo"),
                }),
            )
            .expect("genuine prepared facts");
            let mut inventory = store
                .inspect_submission_inventory()
                .expect("baseline after early writes");
            let progress = handle.maybe_basic_index_progress().expect("progress");
            if mode == BasicFilterStartupMode::Enabled {
                assert!(
                    store
                        .submission_basic_rows(position, None, progress)
                        .is_err(),
                    "missing prepared facts refuse"
                );
                assert!(
                    store
                        .submission_basic_rows(position, Some(&inputs), None)
                        .is_err(),
                    "missing held progress refuses"
                );
            }
            // Act
            inventory
                .project_prepared_connect(&block, &projected, &headers)
                .expect("complete normalized rows");
            inventory
                .project_basic(
                    position,
                    (mode == BasicFilterStartupMode::Enabled).then_some(&inputs),
                    progress,
                )
                .expect("exact future BASIC rows");
            inventory
                .check_current_source()
                .expect("no effects from projection");
            store
                .save_block(&block, crate::PersistMode::Sync)
                .expect("actual command body writer");
            handle
                .connect_local_block(&block, ScriptVerifyFlags::NONE, params)
                .expect("actual same-owner acceptance");
            handle
                .flush_coins(
                    open_bitcoin_core::chainstate::FlushMode::Always,
                    open_bitcoin_core::chainstate::FlushPolicyTime::from_unix_seconds(
                        2_000_000_000,
                    ),
                    crate::chainstate::probe_disk_free_bytes(store.datadir()),
                )
                .expect("actual full flush");
            handle
                .drive_basic_filter_index_turn()
                .expect("actual BASIC publication");
            if mode == BasicFilterStartupMode::Enabled {
                assert!(
                    store
                        .submission_basic_rows(position, Some(&inputs), progress)
                        .is_err(),
                    "old progress cannot forecast across actual advancement"
                );
            }
            // Assert
            assert_eq!(
                handle.chainstate_snapshot().expect("actual complete state"),
                projected
            );
            let achieved = store
                .inspect_submission_inventory()
                .expect("achieved observation");
            for (name, rows) in &inventory.rows {
                for key in rows.keys().chain(achieved.rows[name].keys()) {
                    if rows.get(key) != achieved.rows[name].get(key) {
                        eprintln!(
                            "MAP_DIFFERENCE mode={mode:?} namespace={name} key={} expected={:?} achieved={:?}",
                            String::from_utf8_lossy(key),
                            rows.get(key).map(Vec::len),
                            achieved.rows[name].get(key).map(Vec::len)
                        );
                    }
                }
            }
            inventory.into_achieved_check()(&store).expect("all nine namespace maps equal");
            drop(handle);
        });
    }
}

#[test]
fn phase167_submission_projection_earned_known_only_matches_actual_early_writes() {
    genuine::with_manager(|manager, store, params| {
        // Arrange
        let mut network = genuine::network(manager);
        let block = genuine::child(&network.maybe_chain_tip().expect("root").header, 1);
        let mut observed = store
            .inspect_submission_inventory()
            .expect("before real admission");
        // Act: only the actual earned header supplies identity and row facts.
        genuine::admit(&mut network, std::slice::from_ref(&block), params);
        let hash = open_bitcoin_core::consensus::block_hash(&block.header);
        let identity =
            history::BlockValidationIdentity::new(hash, block.header.previous_block_hash, 1)
                .expect("earned identity");
        observed
            .project_admitted_headers(&network.header_entries(), identity)
            .expect("exact early forecast");
        assert!(
            observed.check_current_source().is_err(),
            "the actual KnownOnly write changes the source"
        );
        network
            .persist_validation_header_snapshot()
            .expect("actual trusted snapshot");
        // Assert
        observed
            .compare_achieved(&store)
            .expect("all early namespace bytes match");
        drop(network);
    });
}

#[test]
fn phase167_submission_projection_segmented_replacements_remove_old_family_rows() {
    genuine::with_manager(|manager, store, _| {
        // Arrange: data-only serializer fixture, not a canonical-history claim.
        let root = store
            .load_header_entries()
            .expect("read")
            .expect("root")
            .entries[0]
            .clone();
        store
            .save_header_entries(std::slice::from_ref(&root), crate::PersistMode::Sync)
            .expect("legacy coverage invalidation");
        for count in [200, 1, 200] {
            let headers = vec![root.clone(); count];
            let mut observed = store
                .inspect_submission_inventory()
                .expect("old native families");
            // Act
            observed
                .header_rows(&headers)
                .expect("same bounded row producer");
            store
                .save_header_entries(&headers, crate::PersistMode::Sync)
                .expect("actual replacement");
            // Assert
            observed
                .compare_achieved(&store)
                .expect("no trailing old segments or unrelated changes");
        }
        drop(manager);
    });
}

#[test]
fn phase167_submission_guard_smoke_actual_factory_modes_reopen_and_next() {
    use crate::{DurableSyncRuntime, SyncNetwork, SyncRuntimeConfig};
    use open_bitcoin_core::consensus::CanonicalChain;
    let config = || SyncRuntimeConfig {
        network: SyncNetwork::Regtest,
        ..Default::default()
    };
    for mode in [
        BasicFilterStartupMode::Disabled,
        BasicFilterStartupMode::Enabled,
    ] {
        // Arrange
        let mut nonce = [0; 16];
        getrandom::fill(&mut nonce).expect("nonce");
        let path = std::fs::canonicalize(std::env::temp_dir())
            .expect("physical parent")
            .join(format!(
                "phase167-projection-smoke-{:x}",
                u128::from_le_bytes(nonce)
            ));
        let (runtime, command) =
            DurableSyncRuntime::open_canonical_submission_configured(&path, config(), mode)
                .expect("actual safe factory");
        let command = command.expect("inferred Regtest command");
        let root = CanonicalChain::Regtest.spec().genesis_block();
        let wire = |block: &open_bitcoin_core::primitives::Block| {
            let bytes = open_bitcoin_core::codec::encode_block(block).expect("complete body");
            let (raw, used) =
                open_bitcoin_core::codec::block::wire::parse_submission_block_prefix(&bytes)
                    .expect("complete raw prefix");
            assert_eq!(used, bytes.len());
            raw
        };
        command
            .submit(wire(&root))
            .expect("genuine genesis duplicate")
            .fold(
                |_| panic!("false acceptance"),
                |_| (),
                |reason| panic!("{reason}"),
                || panic!("inconclusive"),
            );
        let block = genuine::child(&root.header, 1);
        // Act: success requires the production achieved-map/physical checks.
        command
            .submit(wire(&block))
            .expect("guarded actual acceptance")
            .fold(
                |position| assert_eq!(position.height, 1),
                |_| panic!("duplicate"),
                |reason| panic!("{reason}"),
                || panic!("inconclusive"),
            );
        let before = runtime
            .network_handle()
            .chainstate_snapshot()
            .expect("complete accepted state");
        let inventory = runtime
            .store()
            .inspect_submission_inventory()
            .expect("actual achieved inventory");
        inventory.into_achieved_check()(runtime.store()).expect("inferred complete map gate");
        drop(command);
        drop(runtime);
        let (runtime, command) =
            DurableSyncRuntime::open_canonical_submission_configured(&path, config(), mode)
                .expect("drop-all configured safe reopen");
        let command = command.expect("reopened inferred command");
        // Assert
        assert_eq!(
            runtime
                .network_handle()
                .chainstate_snapshot()
                .expect("complete reopened state"),
            before
        );
        assert_eq!(
            runtime
                .store()
                .load_block(open_bitcoin_core::consensus::block_hash(&block.header))
                .expect("actual reopened body"),
            Some(block.clone())
        );
        let next = genuine::child(&block.header, 2);
        command
            .submit(wire(&next))
            .expect("valid next guarded acceptance")
            .fold(
                |position| assert_eq!(position.height, 2),
                |_| panic!("duplicate"),
                |reason| panic!("{reason}"),
                || panic!("inconclusive"),
            );
        drop(command);
        drop(runtime);
        std::fs::remove_dir_all(path).expect("all handles dropped");
    }
}
