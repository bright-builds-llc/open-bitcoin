// Parity breadcrumbs:
// - packages/bitcoin-knots/src/index/base.cpp
// - packages/bitcoin-knots/src/index/blockfilterindex.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp

use super::*;
use crate::storage::fjall_store::filters::FilterPublicationFault;

#[test]
fn filter_index_production_reopen_reconcile_publication_fault_preserves_inputs_and_safe_authority()
{
    for fault in [
        FilterPublicationFault::BeforeCheckpoint,
        FilterPublicationFault::BeforeProtection,
        FilterPublicationFault::AfterCommit,
    ] {
        // Arrange
        let fixture = FilterStartupFixture::new("filter-startup-publish-fault", 20, Some(3));
        let store = FjallNodeStore::open(&fixture.path).expect("store");
        seed_authority(&store, &fixture.positions[..2]);
        drop(store);
        raw_index(&fixture.path, |index| {
            index.remove("prune_intent").expect("no intent");
        });
        let before = snapshot_index(&fixture.path);
        let store = FjallNodeStore::open(&fixture.path).expect("real reopen");
        store.set_basic_filter_fault(fault);
        // Act
        let result = DurableSyncRuntime::open(store, sync_config());
        // Assert
        let Err(SyncRuntimeError::Storage(error)) = result else {
            panic!("publication failure must prevent readiness");
        };
        assert_eq!(
            error.recovery_action(),
            Some(StorageRecoveryAction::Restart)
        );
        assert!(
            error
                .to_string()
                .contains("injected BASIC publication failure")
        );
        let store = FjallNodeStore::open(&fixture.path).expect("inspect durable fault");
        assert_payload(&store, &fixture, true, true);
        let endpoint = if fault == FilterPublicationFault::AfterCommit {
            1
        } else {
            3
        };
        assert_eq!(
            store.maybe_basic_filter_checkpoint().expect("checkpoint"),
            Some(checkpoint(&fixture.records[endpoint]))
        );
        assert_eq!(
            store.load_prune_locks().expect("protection"),
            vec![
                IndexInputProtection::FromHeight(endpoint as u32 + 1)
                    .maybe_prune_lock()
                    .expect("lock")
            ]
        );
        for record in &fixture.records {
            assert_eq!(
                store
                    .load_basic_filter_record(record.identity().block_hash())
                    .expect("immutable record"),
                Some(record.clone())
            );
        }
        drop(store);
        if fault != FilterPublicationFault::AfterCommit {
            assert_eq!(snapshot_index(&fixture.path), before);
        }
        fixture.cleanup();
    }
}
