// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

use super::*;
use crate::storage::fjall_store::FjallNodeStore;
use std::{collections::BTreeMap, path::PathBuf, sync::mpsc};

fn with_store(test: impl FnOnce(&FjallNodeStore)) {
    let mut nonce = [0_u8; 16];
    getrandom::fill(&mut nonce).expect("nonce");
    let path: PathBuf = std::fs::canonicalize(std::env::temp_dir())
        .expect("temp parent")
        .join(format!(
            "phase167-tracking-{:x}",
            u128::from_le_bytes(nonce)
        ));
    let store = FjallNodeStore::open_unrecovered(&path).expect("actual leased open");
    test(&store);
    drop(store);
    std::fs::remove_dir_all(path).expect("cleanup");
}

fn with_supported_store(test: impl FnOnce(&FjallNodeStore)) {
    let mut nonce = [0_u8; 16];
    getrandom::fill(&mut nonce).expect("nonce");
    let path = std::fs::canonicalize(std::env::temp_dir())
        .expect("temp parent")
        .join(format!(
            "phase167-supported-error-{:x}",
            u128::from_le_bytes(nonce)
        ));
    let (store, recovered) = FjallNodeStore::open_canonical_genesis(
        &path,
        open_bitcoin_core::consensus::CanonicalChain::Regtest,
    )
    .expect("genuine selected genesis factory")
    .into_parts()
    .expect("proved actual store");
    drop(recovered);
    test(&store);
    drop(store);
    std::fs::remove_dir_all(path).expect("cleanup");
}

fn rows(store: &FjallNodeStore) -> BTreeMap<String, BTreeMap<Vec<u8>, Vec<u8>>> {
    let records = InspectedRecords::read_existing(&store.db).expect("native reader");
    records
        .namespaces()
        .map(|name| {
            (
                name.to_owned(),
                records.maybe_records(name).expect("namespace").clone(),
            )
        })
        .collect()
}

#[test]
fn phase167_tracked_write_generation_every_held_field_and_accessor_shares_open() {
    with_store(|store| {
        // Arrange
        let fields = [
            &store.headers,
            &store.block_index,
            &store.chainstate,
            &store.coins,
            &store.wallet,
            &store.metrics,
            &store.mempool,
            &store.runtime,
            &store.schema,
        ];
        // Act / Assert
        for field in fields {
            assert!(Arc::ptr_eq(&store.db.shared, &field.shared));
            let stamp = store.db.stamp().expect("idle");
            field
                .clone()
                .insert("same", b"value")
                .expect("actual held write");
            assert!(store.db.check_stamp(&stamp).is_err());
        }
        let cloned = store.db.clone();
        let stamp = cloned.stamp().expect("idle clone");
        store.runtime.insert("clone", b"value").expect("write");
        assert!(cloned.check_stamp(&stamp).is_err());
    });
}

#[test]
fn phase167_tracked_write_generation_same_value_and_aba_remain_stale() {
    with_store(|store| {
        // Arrange
        store.runtime.insert("key", b"A").expect("initial");
        let before = rows(store);
        for values in [
            vec![b"A".as_slice()],
            vec![b"B".as_slice(), b"A".as_slice()],
        ] {
            let stamp = store.db.stamp().expect("idle");
            // Act
            for value in values {
                store.runtime.insert("key", value).expect("real write");
            }
            // Assert
            assert_eq!(rows(store), before);
            assert!(store.db.check_stamp(&stamp).is_err());
            assert!(store.db.stamp().is_ok());
        }
    });
}

#[test]
fn phase167_tracked_write_generation_empty_add_delete_and_tombstone_maintenance() {
    with_store(|store| {
        // Arrange
        let before = rows(store);
        let stamp = store.db.stamp().expect("idle");
        // Act
        store.runtime.insert("brief", b"value").expect("add");
        store.runtime.remove("brief").expect("delete");
        // Assert
        assert_eq!(rows(store), before);
        assert!(store.db.check_stamp(&stamp).is_err());
        let stamp = store.db.stamp().expect("new idle");
        store.runtime.remove("never-existed").expect("tombstone");
        store
            .runtime
            .rotate_memtable_and_wait()
            .expect("physical flush");
        store.runtime.major_compact().expect("physical compaction");
        assert_eq!(rows(store), before);
        assert!(store.db.check_stamp(&stamp).is_err());
        assert!(store.db.stamp().is_ok());
    });
}

#[test]
fn phase167_tracked_write_generation_clear_restore_and_weak_remove() {
    with_store(|store| {
        // Arrange
        store.runtime.insert("key", b"A").expect("initial");
        let before = rows(store);
        let stamp = store.db.stamp().expect("idle");
        // Act
        store.runtime.clear().expect("actual clear");
        store.runtime.insert("key", b"A").expect("restore");
        // Assert
        assert_eq!(rows(store), before);
        assert!(store.db.check_stamp(&stamp).is_err());
        let stamp = store.db.stamp().expect("idle");
        store
            .runtime
            .remove_weak("never-existed")
            .expect("weak tombstone");
        assert!(store.db.check_stamp(&stamp).is_err());
        assert_eq!(rows(store), before);
    });
}

#[test]
fn phase167_tracked_write_generation_catalog_same_name_recreation() {
    with_store(|store| {
        // Arrange
        let before = rows(store);
        let stamp = store.db.stamp().expect("idle");
        // Act
        store
            .db
            .delete_keyspace(store.runtime.clone())
            .expect("tracked delete");
        let recreated = store
            .db
            .keyspace("runtime", KeyspaceCreateOptions::default)
            .expect("tracked recreate");
        // Assert
        assert!(Arc::ptr_eq(&store.db.shared, &recreated.shared));
        assert_eq!(rows(store), before);
        assert!(store.db.check_stamp(&stamp).is_err());
    });
}

#[test]
fn phase167_tracked_write_generation_native_deleted_handle_error_closes_writer() {
    with_supported_store(|store| {
        // Arrange: delete only a temporary namespace before the measured interval.
        let temporary = store
            .db
            .keyspace("native-error", KeyspaceCreateOptions::default)
            .expect("temporary keyspace");
        let retained = temporary.clone();
        store.db.delete_keyspace(temporary).expect("tracked delete");
        let before = rows(store);
        assert_eq!(before.len(), 9);
        assert_eq!(
            store.db.keyspace_count(),
            9,
            "temporary catalog entry removed"
        );
        drop(
            store
                .inspect_submission_inventory()
                .expect("supported inventory after temporary deletion"),
        );
        let stamp = store.db.stamp().expect("idle after deletion");
        // Act
        let result = retained.insert("native-error", b"x");
        // Assert
        assert!(matches!(result, Err(fjall::Error::KeyspaceDeleted)));
        assert_eq!(rows(store), before);
        assert!(store.db.check_stamp(&stamp).is_err());
        let fresh = store.db.stamp().expect("actual Err closed active writer");
        store.db.check_stamp(&fresh).expect("fresh stamp usable");
    });
}

#[test]
fn phase167_tracked_write_generation_buffering_empty_and_nonempty_commit() {
    with_store(|store| {
        // Arrange
        store.runtime.insert("key", b"A").expect("initial");
        let before = rows(store);
        let stamp = store.db.stamp().expect("idle");
        let generation = stamp.generation;
        // Act
        store
            .db
            .batch()
            .durability(Some(PersistMode::SyncAll))
            .commit()
            .expect("empty noop");
        let mut batch = store.db.batch();
        batch.insert(&store.runtime, "key", b"A");
        batch.insert(&store.runtime, "brief", b"value");
        batch.remove(&store.runtime, "brief");
        // Assert: buffering is not a live write; nonempty commit counts once.
        store.db.check_stamp(&stamp).expect("buffering benign");
        assert_eq!(rows(store), before);
        batch.commit().expect("actual native batch");
        assert!(store.db.check_stamp(&stamp).is_err());
        assert_eq!(store.db.stamp().expect("idle").generation, generation + 1);
        assert_eq!(rows(store), before);
    });
}

#[test]
fn phase167_tracked_write_generation_foreign_stamp_and_batches_refuse_before_effects() {
    with_store(|store| {
        with_store(|other| {
            // Arrange
            let stamp = store.db.stamp().expect("idle");
            let foreign_stamp = other.db.stamp().expect("other idle");
            let before = rows(store);
            let other_before = rows(other);
            // Act / Assert: invalid foreign-only batches have an empty native buffer.
            for include_local in [false, true] {
                let mut batch = store.db.batch();
                if include_local {
                    batch.insert(&store.runtime, "local", b"x");
                }
                batch.insert(&other.runtime, "foreign", b"x");
                batch.remove(&other.runtime, "foreign");
                assert!(batch.commit().is_err());
                store.db.check_stamp(&stamp).expect("no BEGIN");
                other
                    .db
                    .check_stamp(&foreign_stamp)
                    .expect("no foreign effect");
                assert_eq!(rows(store), before);
                assert_eq!(rows(other), other_before);
            }
            assert!(store.db.check_stamp(&foreign_stamp).is_err());
            assert!(other.db.check_stamp(&stamp).is_err());
            assert!(store.db.delete_keyspace(other.runtime.clone()).is_err());
        })
    });
}

#[test]
fn phase167_tracked_write_generation_reads_lookup_snapshot_persist_and_rotation_benign() {
    with_store(|store| {
        // Arrange
        store.runtime.insert("key", b"A").expect("initial");
        let held = store
            .db
            .keyspace("runtime", || {
                std::panic::resume_unwind(Box::new("existing options must not run"))
            })
            .expect("existing lookup");
        let before = rows(store);
        let stamp = store.db.stamp().expect("idle");
        let physical = store.db.seqno();
        // Act
        let snapshot = store.db.snapshot();
        assert_eq!(
            snapshot
                .get(&held, "key")
                .expect("snapshot read")
                .as_deref(),
            Some(b"A".as_slice())
        );
        store
            .db
            .persist(PersistMode::SyncAll)
            .expect("physical persistence");
        held.rotate_memtable_and_wait().expect("physical rotation");
        // Assert
        assert!(store.db.seqno() > physical, "actual allocator advanced");
        assert_eq!(rows(store), before);
        store
            .db
            .check_stamp(&stamp)
            .expect("logical observation unchanged");
    });
}

struct ScheduledKey {
    entered: mpsc::Sender<()>,
    resume: mpsc::Receiver<()>,
    unwind: bool,
}

impl From<ScheduledKey> for UserKey {
    fn from(key: ScheduledKey) -> Self {
        key.entered
            .send(())
            .expect("entered actual native key conversion");
        key.resume.recv().expect("deterministic release");
        if key.unwind {
            std::panic::resume_unwind(Box::new("key conversion unwind"));
        }
        "scheduled".into()
    }
}

#[test]
fn phase167_tracked_write_generation_inflight_and_actual_native_unwind_close_writer() {
    for unwind in [false, true] {
        with_store(|store| {
            // Arrange: data conversion runs within the real native insert after BEGIN.
            let (entered_tx, entered_rx) = mpsc::channel();
            let (resume_tx, resume_rx) = mpsc::channel();
            let keyspace = store.runtime.clone();
            let stamp = store.db.stamp().expect("idle");
            let generation = stamp.generation;
            let worker = std::thread::spawn(move || {
                keyspace.insert(
                    ScheduledKey {
                        entered: entered_tx,
                        resume: resume_rx,
                        unwind,
                    },
                    b"x",
                )
            });
            entered_rx.recv().expect("native mutation entered");
            // Act / Assert: no generation mutex spans this blocked native write.
            assert!(store.db.stamp().is_err());
            assert!(store.db.check_stamp(&stamp).is_err());
            resume_tx.send(()).expect("release");
            let result = worker.join();
            assert_eq!(result.is_err(), unwind);
            if let Ok(result) = result {
                result.expect("native success");
            }
            let fresh = store.db.stamp().expect("Ok and unwind close active count");
            assert_eq!(
                fresh.generation,
                generation + 1,
                "BEGIN only, no Drop increment"
            );
            store.db.check_stamp(&fresh).expect("new idle stamp");
            assert!(store.db.check_stamp(&stamp).is_err());
            assert_eq!(
                store
                    .runtime
                    .contains_key("scheduled")
                    .expect("actual data"),
                !unwind
            );
        });
    }
}

#[test]
fn phase167_tracked_write_generation_overflow_permanently_refuses() {
    with_store(|store| {
        // Arrange
        store.db.shared.generation.lock().expect("state").value = u64::MAX;
        let stamp = store.db.stamp().expect("last idle generation");
        let before = rows(store);
        // Act
        assert!(store.runtime.insert("no-wrap", b"x").is_err());
        // Assert
        assert!(store.db.stamp().is_err());
        assert!(store.db.check_stamp(&stamp).is_err());
        assert!(store.runtime.remove("never").is_err());
        assert_eq!(rows(store), before);
        store
            .db
            .batch()
            .commit()
            .expect("native empty noop precedes mutation gate");
    });
}

#[test]
fn phase167_tracked_write_generation_poison_and_guard_drop_never_double_panic() {
    with_store(|store| {
        // Arrange
        let stamp = store.db.stamp().expect("idle");
        let guard = store.db.shared.begin().expect("actual active guard");
        // Act: unwind while poisoning the state and dropping an active guard.
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let _guard = guard;
            let _state = store.db.shared.generation.lock().expect("state");
            std::panic::resume_unwind(Box::new("poison control"));
        }));
        // Assert
        assert!(result.is_err());
        assert!(store.db.stamp().is_err());
        assert!(store.db.check_stamp(&stamp).is_err());
        assert!(store.runtime.insert("no-recovery", b"x").is_err());
        assert!(
            !store
                .runtime
                .contains_key("no-recovery")
                .expect("no effect")
        );
    });
}
