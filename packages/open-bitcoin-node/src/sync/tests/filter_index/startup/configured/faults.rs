// Parity breadcrumbs:
// - packages/bitcoin-knots/src/index/base.cpp
// - packages/bitcoin-knots/src/index/blockfilterindex.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp

use super::*;
use open_bitcoin_core::chainstate::PruneLockInfo;

fn preserved_rows(rows: Vec<(Vec<u8>, Vec<u8>)>) -> Vec<(Vec<u8>, Vec<u8>)> {
    rows.into_iter()
        .filter(|(key, _)| key != codec::ownership::OWNER_KEY.as_bytes() && key != b"prune_locks")
        .collect()
}

#[test]
fn phase157_activation_configured_disable_faults_preserve_sources_and_truthful_owner() {
    for (fault, disabled, released) in [
        (FilterPublicationFault::BeforeDisable, false, false),
        (FilterPublicationFault::AfterDisable, true, false),
        (FilterPublicationFault::BeforeRelease, true, false),
        (FilterPublicationFault::AfterRelease, true, true),
    ] {
        // Arrange
        let fixture = FilterStartupFixture::new("phase157-disable-startup-fault", 20, Some(1));
        let store = FjallNodeStore::open(&fixture.path).expect("saved owner");
        let ordinary = PruneLockInfo {
            name: "operator".to_owned(),
            height_first: 100,
            height_last: 150,
        };
        let mut locks = store.load_prune_locks().expect("old locks");
        locks.push(ordinary.clone());
        store.sync_prune_locks(&locks).expect("ordinary lock");
        drop(store);
        let before = snapshot_index(&fixture.path);
        let history_before = snapshot_history(&fixture.path);
        let store = FjallNodeStore::open(&fixture.path).expect("faulted production open");
        store.set_basic_filter_fault(fault);

        // Act
        let result = DurableSyncRuntime::open_configured(
            store,
            sync_config(),
            BasicFilterStartupMode::Disabled,
        );

        // Assert
        assert!(result.is_err(), "injected {fault:?} must stop startup");
        let after = snapshot_index(&fixture.path);
        assert_eq!(
            preserved_rows(after.clone()),
            preserved_rows(before.clone())
        );
        assert_eq!(snapshot_history(&fixture.path), history_before);
        if !disabled {
            assert_eq!(after, before);
        }
        let store = FjallNodeStore::open(&fixture.path).expect("actual fault reopen");
        let owner = store
            .maybe_basic_filter_lifecycle_for_test()
            .expect("durable owner")
            .expect("owner");
        assert_eq!(matches!(owner, IndexLifecycle::Disabled { .. }), disabled);
        assert_eq!(
            owner.generation(),
            IndexGeneration::new(u64::from(disabled))
        );
        let locks = store.load_prune_locks().expect("durable locks");
        assert_eq!(
            locks.iter().any(|lock| lock.name == BASIC_INDEX_PRUNE_LOCK),
            !released
        );
        assert!(
            locks.contains(&ordinary),
            "disable only releases reserved ownership"
        );
        assert_payload(&store, &fixture, true, true);
        assert_eq!(
            store.maybe_prune_intent().expect("unexecuted intent"),
            Some(fixture.intent)
        );
        drop(store);
        let runtime =
            open_configured(&fixture.path, BasicFilterStartupMode::Disabled).expect("safe retry");
        assert_eq!(
            runtime
                .store
                .maybe_basic_filter_lifecycle_for_test()
                .expect("retry owner"),
            Some(IndexLifecycle::Disabled {
                generation: IndexGeneration::new(1)
            })
        );
        assert_eq!(
            runtime.store.load_prune_locks().expect("ordinary survives"),
            vec![ordinary]
        );
        assert_payload(&runtime.store, &fixture, false, false);
        drop(runtime);
        fixture.cleanup();
    }
}

#[test]
fn phase157_activation_configured_enable_faults_publish_atomic_owner_before_prune() {
    for (fault, enabled) in [
        (FilterPublicationFault::BeforeEnable, false),
        (FilterPublicationFault::AfterEnable, true),
    ] {
        // Arrange
        let fixture = FilterStartupFixture::new("phase157-enable-startup-fault", 1, Some(16));
        retain_required_suffix(&fixture);
        let store = FjallNodeStore::open(&fixture.path).expect("saved Active");
        store.disable_basic_filter_index().expect("saved Disabled");
        drop(store);
        let before = snapshot_index(&fixture.path);
        let history_before = snapshot_history(&fixture.path);
        let store = FjallNodeStore::open(&fixture.path).expect("faulted production open");
        store.set_basic_filter_fault(fault);

        // Act
        let result = DurableSyncRuntime::open_configured(
            store,
            sync_config(),
            BasicFilterStartupMode::Enabled,
        );

        // Assert
        assert!(result.is_err(), "injected {fault:?} must stop startup");
        let after = snapshot_index(&fixture.path);
        assert_eq!(
            preserved_rows(after.clone()),
            preserved_rows(before.clone())
        );
        assert_eq!(snapshot_history(&fixture.path), history_before);
        if !enabled {
            assert_eq!(after, before);
        }
        let store = FjallNodeStore::open(&fixture.path).expect("actual fault reopen");
        let owner = store
            .maybe_basic_filter_lifecycle_for_test()
            .expect("owner")
            .expect("saved");
        assert_eq!(matches!(owner, IndexLifecycle::Active { .. }), enabled);
        assert_eq!(
            owner.generation(),
            IndexGeneration::new(if enabled { 2 } else { 1 })
        );
        assert_eq!(
            !store.load_prune_locks().expect("protection").is_empty(),
            enabled
        );
        assert_payload(&store, &fixture, true, true);
        assert_eq!(
            store.maybe_prune_intent().expect("intent survives fault"),
            Some(fixture.intent)
        );
        drop(store);
        let runtime = open_configured(&fixture.path, BasicFilterStartupMode::Enabled)
            .expect("retry safe enable");
        assert_payload(&runtime.store, &fixture, false, false);
        assert_eq!(
            runtime
                .store
                .maybe_basic_filter_lifecycle_for_test()
                .expect("retry owner"),
            Some(IndexLifecycle::Active {
                generation: IndexGeneration::new(2)
            })
        );
        drop(runtime);
        fixture.cleanup();
    }
}
