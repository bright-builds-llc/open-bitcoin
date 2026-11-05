// Parity breadcrumbs:
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/txdb.cpp

use super::*;
use crate::{
    DurableSyncRuntime, ManagedNetworkHandle, SyncNetwork, SyncRuntimeConfig,
    chainstate::{BasicFilterStartupMode, validation_history::tests::phase167 as genuine},
};
use open_bitcoin_core::{
    chainstate::{FlushMode, FlushPolicyTime},
    consensus::ScriptVerifyFlags,
};
use std::collections::BTreeMap;

type Rows = BTreeMap<String, BTreeMap<Vec<u8>, Vec<u8>>>;

fn rows(store: &FjallNodeStore) -> Rows {
    let records = inspection::InspectedRecords::read_existing(&store.db).expect("native rows");
    let rows: Rows = records
        .namespaces()
        .map(|name| {
            (
                name.to_owned(),
                records.maybe_records(name).expect("present").clone(),
            )
        })
        .collect();
    assert_eq!(rows.len(), 9, "complete required catalog");
    rows
}

fn physical_only(store: &FjallNodeStore) {
    // Hold the existing handle before the measured logical/physical interval.
    let keyspace = store
        .db
        .keyspace("block_index", fjall::KeyspaceCreateOptions::default)
        .expect("existing nonempty namespace");
    let before = rows(store);
    let stamp = store.db.stamp().expect("idle logical baseline");
    let physical = store.db.seqno();
    keyspace
        .rotate_memtable_and_wait()
        .expect("actual native rotation");
    let after = store.db.seqno();
    assert!(after > physical, "native allocator actually advanced");
    assert_eq!(
        rows(store),
        before,
        "complete nine maps and catalog unchanged"
    );
    store
        .db
        .check_stamp(&stamp)
        .expect("physical maintenance is benign");
    eprintln!(
        "CANONICAL_PHYSICAL before_seq={physical} after_seq={after} catalog_equal=true all9_maps_equal=true"
    );
}

fn config() -> SyncRuntimeConfig {
    SyncRuntimeConfig {
        network: SyncNetwork::Regtest,
        ..Default::default()
    }
}

#[test]
fn phase167_logical_seal_freshness_genesis_physical_rotation_consumes_real_owner() {
    // Arrange
    let source = source();
    let opened = open(&source, CanonicalChain::Regtest);
    let weak = Arc::downgrade(&opened.store.filter_publication);
    let expected = rows(&opened.store);
    // Act
    physical_only(&opened.store);
    opened
        .recovered
        .check(&opened.store)
        .expect("original genesis seal");
    let runtime = DurableSyncRuntime::open_canonical_genesis_configured(
        opened,
        config(),
        BasicFilterStartupMode::Disabled,
    )
    .expect("actual runtime consumes original maintained seal");
    // Assert
    assert_eq!(
        runtime
            .store()
            .load_header_entries()
            .expect("headers")
            .expect("present")
            .entries
            .len(),
        1
    );
    drop(runtime);
    assert!(weak.upgrade().is_none(), "all actual owners released");
    let reopened = reopen(&source, CanonicalChain::Regtest);
    // Runtime configuration may persist settings, so the initial seal's bytes
    // were checked before consumption; safe reopen proves the resulting source.
    assert_eq!(
        rows(&reopened.store).keys().collect::<Vec<_>>(),
        expected.keys().collect::<Vec<_>>()
    );
    assert!(reopened.into_parts().is_ok());
    fs::remove_dir_all(source).expect("cleanup");
}

fn mutate(store: &FjallNodeStore, case: &str) {
    match case {
        "identical" => {
            let identity = store
                .schema
                .get(records::IDENTITY_KEY)
                .expect("read")
                .expect("identity");
            store
                .schema
                .insert(records::IDENTITY_KEY, identity)
                .expect("identical live write");
        }
        "aba" => {
            let identity = store
                .schema
                .get(records::IDENTITY_KEY)
                .expect("read")
                .expect("identity");
            store
                .schema
                .insert(records::IDENTITY_KEY, b"intervening")
                .expect("B");
            store
                .schema
                .insert(records::IDENTITY_KEY, identity)
                .expect("A restored");
        }
        "empty-add-delete" => {
            store.runtime.insert("ephemeral", b"x").expect("add");
            store.runtime.remove("ephemeral").expect("delete");
        }
        "clear-restore" => {
            let schema: Vec<_> = store
                .schema
                .iter()
                .map(|row| row.into_inner().expect("row"))
                .collect();
            store.schema.clear().expect("actual clear");
            for (key, value) in schema {
                store.schema.insert(key, value).expect("restore");
            }
        }
        "catalog-recreate" => {
            store
                .db
                .delete_keyspace(store.runtime.clone())
                .expect("delete empty required namespace");
            store
                .db
                .keyspace("runtime", fjall::KeyspaceCreateOptions::default)
                .expect("same name restored");
        }
        _ => unreachable!("closed test case set"),
    }
}

const MUTATIONS: [&str; 5] = [
    "identical",
    "aba",
    "empty-add-delete",
    "clear-restore",
    "catalog-recreate",
];

#[test]
fn phase167_logical_seal_freshness_genesis_live_writes_refuse_identical_full_source() {
    for case in MUTATIONS {
        // Arrange
        let source = source();
        let opened = open(&source, CanonicalChain::Regtest);
        let before = rows(&opened.store);
        // Act
        mutate(&opened.store, case);
        // Assert
        assert_eq!(
            rows(&opened.store),
            before,
            "complete source restored: {case}"
        );
        prove_live(&opened.store, CanonicalChain::Regtest)
            .expect("full original genesis proof still valid");
        assert!(
            matches!(opened.into_parts(), Err(Refusal::ChangedStore)),
            "actual live mutation {case}"
        );
        drop(reopen(&source, CanonicalChain::Regtest));
        fs::remove_dir_all(source).expect("cleanup");
    }
}

fn with_history(
    mode: BasicFilterStartupMode,
    check: impl FnOnce(&Path, OpenedCanonicalHistory, open_bitcoin_core::chainstate::ChainstateSnapshot),
) {
    genuine::with_mode(mode, |manager, store, params| {
        let path = store.datadir().to_path_buf();
        let weak = Arc::downgrade(&store.filter_publication);
        let mut network = genuine::network(manager);
        let block = genuine::child(&network.maybe_chain_tip().expect("genesis").header, 1);
        genuine::admit(&mut network, std::slice::from_ref(&block), params);
        store
            .save_block(&block, PersistMode::Sync)
            .expect("real retained body");
        network
            .connect_local_block(&block, ScriptVerifyFlags::NONE, params)
            .expect("genuine managed acceptance");
        network
            .persist_validation_header_snapshot()
            .expect("trusted header checkpoint");
        network
            .chainstate_mut()
            .flush_with_mode(
                FlushMode::Always,
                FlushPolicyTime::from_unix_seconds(60_001),
                u64::MAX,
            )
            .expect("real complete durable coins checkpoint");
        let network = ManagedNetworkHandle::new(network);
        if mode == BasicFilterStartupMode::Enabled {
            network
                .initialize_basic_filter_index_owner()
                .expect("actual BASIC owner");
            for _ in 0..=2 {
                let turn = network
                    .drive_basic_filter_index_turn()
                    .expect("bounded actual BASIC turn");
                if turn.maybe_accepted_lag == Some(0) {
                    break;
                }
            }
            assert_eq!(
                network
                    .maybe_basic_index_progress()
                    .expect("progress")
                    .expect("enabled")
                    .maybe_safe_durable_endpoint()
                    .expect("safe")
                    .height(),
                1
            );
        }
        let expected = network
            .chainstate_snapshot()
            .expect("actual accepted state");
        assert_eq!(expected.active_chain.len(), 2);
        drop(network);
        drop(store);
        assert!(
            weak.upgrade().is_none(),
            "all original owners dropped before safe history reopen"
        );
        let opened =
            FjallNodeStore::reopen_canonical_history_existing(&path, CanonicalChain::Regtest)
                .expect("genuinely earned supported history through real safe factory");
        assert_eq!(
            opened
                .store
                .wallet_scan_chainstate_snapshot()
                .expect("stored full snapshot")
                .expect("checkpoint"),
            expected
        );
        check(&path, opened, expected);
    });
}

#[test]
fn phase167_logical_seal_freshness_history_physical_rotation_consumes_real_owner() {
    for mode in [
        BasicFilterStartupMode::Disabled,
        BasicFilterStartupMode::Enabled,
    ] {
        with_history(mode, |path, opened, expected| {
            // Arrange
            let weak = Arc::downgrade(&opened.store.filter_publication);
            let before = rows(&opened.store);
            let digest = opened.recovered.records.source_digest();
            // Act
            physical_only(&opened.store);
            opened
                .recovered
                .check(&opened.store)
                .expect("original maintained history seal");
            assert_eq!(opened.recovered.records.source_digest(), digest);
            assert_eq!(rows(&opened.store), before);
            let runtime =
                DurableSyncRuntime::open_canonical_history_configured(opened, config(), mode)
                    .expect("actual runtime consumes original history owner");
            // Assert
            assert_eq!(
                runtime
                    .network_handle()
                    .chainstate_snapshot()
                    .expect("restored full state"),
                expected
            );
            drop(runtime);
            assert!(
                weak.upgrade().is_none(),
                "all maintained runtime owners released"
            );
            let reopened =
                FjallNodeStore::reopen_canonical_history_existing(path, CanonicalChain::Regtest)
                    .expect("next actual safe reopen after all maintained owners drop");
            assert_eq!(
                reopened
                    .store
                    .wallet_scan_chainstate_snapshot()
                    .expect("stored full snapshot")
                    .expect("checkpoint"),
                expected
            );
            let (store, recovered) = reopened.into_parts().expect("next current original seal");
            let (_, _, replay) = recovered
                .into_parts(&store)
                .expect("full source-bound proof consumed again");
            assert_eq!(
                replay.admission_snapshot().expect("complete replay"),
                expected
            );
        });
    }
}

#[test]
fn phase167_logical_seal_freshness_history_live_writes_refuse_identical_full_source() {
    for case in MUTATIONS {
        with_history(
            BasicFilterStartupMode::Disabled,
            |path, opened, expected| {
                // Arrange
                let before = rows(&opened.store);
                let digest = opened.recovered.records.source_digest();
                // Act
                mutate(&opened.store, case);
                // Assert
                assert_eq!(
                    rows(&opened.store),
                    before,
                    "all nine restored maps: {case}"
                );
                let actual = inspection::InspectedRecords::read_existing(&opened.store.db)
                    .expect("actual bytes");
                let actual =
                    crate::storage::canonical_history::records::CanonicalHistoryRecords::decode(
                        &actual,
                    )
                    .expect("full source decode");
                assert_eq!(actual.source_digest(), digest);
                assert_eq!(
                    opened
                        .store
                        .wallet_scan_chainstate_snapshot()
                        .expect("stored full snapshot")
                        .expect("checkpoint"),
                    expected
                );
                assert!(
                    matches!(opened.into_parts(), Err(Refusal::ChangedStore)),
                    "live history mutation {case}"
                );
                let reopened = FjallNodeStore::reopen_canonical_history_existing(
                    path,
                    CanonicalChain::Regtest,
                )
                .expect("fresh proof of same supported bytes");
                assert_eq!(
                    reopened
                        .store
                        .wallet_scan_chainstate_snapshot()
                        .expect("stored full snapshot")
                        .expect("checkpoint"),
                    expected
                );
            },
        );
    }
}

#[test]
fn phase167_logical_seal_freshness_history_foreign_open_refuses_full_source_match() {
    with_history(BasicFilterStartupMode::Disabled, |_, opened, _| {
        with_history(BasicFilterStartupMode::Disabled, |_, foreign, _| {
            // Arrange
            assert_eq!(rows(&opened.store), rows(&foreign.store));
            assert_eq!(
                opened.recovered.records.source_digest(),
                foreign.recovered.records.source_digest()
            );
            // Act / Assert
            assert!(matches!(
                opened.recovered.check(&foreign.store),
                Err(Refusal::ChangedStore)
            ));
            assert!(opened.into_parts().is_ok());
            assert!(foreign.into_parts().is_ok());
        });
    });
}
