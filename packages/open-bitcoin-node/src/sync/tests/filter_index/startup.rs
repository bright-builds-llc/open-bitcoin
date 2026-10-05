// Parity breadcrumbs:
// - packages/bitcoin-knots/src/index/base.cpp
// - packages/bitcoin-knots/src/index/blockfilterindex.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp

use super::*;

mod configured;
mod faults;
mod recovery;

#[test]
fn filter_index_production_reopen_refuses_required_history_before_prune() {
    // Arrange
    let fixture = FilterStartupFixture::new("filter-required-history", 20, Some(1));
    let before = snapshot_index(&fixture.path);
    // Act
    let result = fixture.open_runtime();
    // Assert
    assert_filter_refusal(result, "prune intent targets required BASIC input");
    assert_eq!(snapshot_index(&fixture.path), before);
    let store = FjallNodeStore::open(&fixture.path).expect("inspect refusal");
    assert_payload(&store, &fixture, true, true);
    assert_eq!(
        store.maybe_prune_intent().expect("intent"),
        Some(fixture.intent)
    );
    assert_eq!(
        store.maybe_basic_filter_checkpoint().expect("checkpoint"),
        Some(checkpoint(&fixture.records[1]))
    );
    assert_eq!(
        store.basic_filter_projection(1).expect("projection"),
        fixture.positions[1].block_hash
    );
    assert_eq!(
        store
            .load_basic_filter_record(fixture.positions[0].block_hash)
            .expect("record"),
        Some(fixture.records[0].clone())
    );
    assert_eq!(
        store.load_prune_locks().expect("protection"),
        vec![
            IndexInputProtection::FromHeight(2)
                .maybe_prune_lock()
                .expect("lock")
        ]
    );
    drop(store);
    fixture.cleanup();
}

#[test]
fn filter_index_production_reopen_directly_protects_genesis_and_height_one() {
    for height in [0, 1, u32::MAX] {
        // Arrange
        let fixture = FilterStartupFixture::new("filter-direct-protection", height.min(1), Some(0));
        let store = FjallNodeStore::open(&fixture.path).expect("store");
        if height == 0 {
            store
                .write_raw_for_test(
                    StorageNamespace::BlockIndex,
                    codec::STATE_KEY,
                    codec::encode_state(codec::StoredFilterState {
                        maybe_endpoint: None,
                        fence_height: 400,
                        fence_hash: fixture.positions[400].block_hash,
                        protection: IndexInputProtection::FromHeight(0),
                    }),
                )
                .expect("empty checkpoint");
            seed_raw_basic_protection(&store, IndexInputProtection::FromHeight(0));
        }
        if height == u32::MAX {
            seed_raw_prune_intent(
                &store,
                PruneIntent {
                    height,
                    block_hash: fixture.intent.block_hash,
                },
            );
        }
        drop(store);
        let before = snapshot_index(&fixture.path);
        // Act
        let result = fixture.open_runtime();
        // Assert
        assert_filter_refusal(result, "prune intent targets required BASIC input");
        assert_eq!(snapshot_index(&fixture.path), before);
        let store = FjallNodeStore::open(&fixture.path).expect("inspect");
        assert_payload(&store, &fixture, true, height != 0);
        assert_eq!(
            store
                .maybe_prune_intent()
                .expect("intent")
                .expect("live")
                .height,
            height
        );
        drop(store);
        fixture.cleanup();
    }
}

#[test]
fn filter_index_production_reopen_finishes_safe_indexed_intent() {
    // Arrange
    let fixture = FilterStartupFixture::new("filter-safe-indexed", 1, Some(16));
    let before = snapshot_index(&fixture.path);
    // Act
    let runtime = fixture.open_runtime().expect("safe production startup");
    // Assert
    assert_payload(runtime.store(), &fixture, false, false);
    assert_eq!(runtime.store().maybe_prune_intent().expect("intent"), None);
    assert_eq!(
        runtime
            .store()
            .maybe_basic_filter_checkpoint()
            .expect("checkpoint"),
        Some(checkpoint(&fixture.records[16]))
    );
    assert!(runtime.store().load_have_pruned().expect("pruned"));
    drop(runtime);
    let after = snapshot_index(&fixture.path);
    let filters = |rows: Vec<(Vec<u8>, Vec<u8>)>| {
        rows.into_iter()
            .filter(|(key, _)| key.starts_with(b"basic_filter:v1:"))
            .collect::<Vec<_>>()
    };
    assert_eq!(filters(after), filters(before));
    fixture.cleanup();
}

#[test]
fn filter_index_production_reopen_preserves_legacy_safe_resume() {
    // Arrange
    let fixture = FilterStartupFixture::new("filter-legacy-safe", 1, None);
    // Act
    let runtime = fixture.open_runtime().expect("legacy production startup");
    // Assert
    assert_payload(runtime.store(), &fixture, false, false);
    assert_eq!(runtime.store().maybe_prune_intent().expect("intent"), None);
    assert_eq!(
        runtime.store().maybe_basic_filter_state().expect("state"),
        None
    );
    assert_eq!(
        runtime.store().basic_filter_artifacts().expect("artifacts"),
        (false, false)
    );
    assert!(
        runtime
            .store()
            .load_prune_locks()
            .expect("locks")
            .is_empty()
    );
    drop(runtime);
    fixture.cleanup();
}

fn refuse_mutated_index(
    name: &str,
    category: &str,
    mutation: impl FnOnce(&fjall::Keyspace, &FilterStartupFixture),
) {
    // Arrange
    let fixture = FilterStartupFixture::new(name, 20, Some(1));
    raw_index(&fixture.path, |index| mutation(index, &fixture));
    let before = snapshot_index(&fixture.path);
    // Act
    let result = fixture.open_runtime();
    // Assert
    assert_filter_refusal(result, category);
    assert_eq!(
        snapshot_index(&fixture.path),
        before,
        "every immutable/state/projection/lock/intent byte survives"
    );
    let store = FjallNodeStore::open(&fixture.path).expect("inspect refusal");
    assert_payload(&store, &fixture, true, true);
    assert_eq!(
        store.maybe_prune_intent().expect("intent"),
        Some(fixture.intent)
    );
    assert!(!store.load_have_pruned().expect("never pruned"));
    drop(store);
    fixture.cleanup();
}

fn lock_bytes(name: &str, first: u32, last: u32) -> Vec<u8> {
    let mut bytes = 1_u32.to_le_bytes().to_vec();
    bytes.extend_from_slice(
        &u16::try_from(name.len())
            .expect("name length")
            .to_le_bytes(),
    );
    bytes.extend_from_slice(name.as_bytes());
    bytes.extend_from_slice(&first.to_le_bytes());
    bytes.extend_from_slice(&last.to_le_bytes());
    bytes
}

#[test]
fn filter_index_production_reopen_refuses_missing_reserved_protection() {
    refuse_mutated_index(
        "filter-missing-lock",
        "missing BASIC reserved protection",
        |index, _| {
            index.remove("prune_locks").expect("remove saved lock");
        },
    );
}

#[test]
fn filter_index_production_reopen_ordinary_operator_lock_cannot_replace_reserved_protection() {
    refuse_mutated_index(
        "filter-ordinary-lock",
        "missing BASIC reserved protection",
        |index, _| {
            index
                .insert("prune_locks", lock_bytes("rescan", 2, 20))
                .expect("operator-only lock");
        },
    );
}

#[test]
fn filter_index_production_reopen_refuses_weak_reserved_protection() {
    refuse_mutated_index(
        "filter-weak-lock",
        "weak BASIC reserved protection",
        |index, _| {
            index
                .insert(
                    "prune_locks",
                    lock_bytes(BASIC_INDEX_PRUNE_LOCK, 3, u32::MAX - 10),
                )
                .expect("weak lock");
        },
    );
}

#[test]
fn filter_index_production_reopen_refuses_overflowing_reserved_protection() {
    refuse_mutated_index(
        "filter-max-lock",
        "reserved BASIC protection range is malformed",
        |index, _| {
            index
                .insert(
                    "prune_locks",
                    lock_bytes(BASIC_INDEX_PRUNE_LOCK, 2, u32::MAX),
                )
                .expect("overflow lock");
        },
    );
}

#[test]
fn filter_index_production_reopen_refuses_corrupt_reserved_protection() {
    refuse_mutated_index(
        "filter-corrupt-lock",
        "truncated prune record",
        |index, _| {
            index.insert("prune_locks", vec![1]).expect("corrupt lock");
        },
    );
}

#[test]
fn filter_index_production_reopen_refuses_corrupt_checkpoint_state() {
    refuse_mutated_index("filter-corrupt-state", "BASIC", |index, _| {
        index
            .insert(codec::STATE_KEY, vec![1])
            .expect("corrupt state");
    });
}

#[test]
fn filter_index_production_reopen_refuses_partial_missing_state() {
    refuse_mutated_index(
        "filter-missing-state",
        "BASIC state is absent but index artifacts exist",
        |index, _| {
            index.remove(codec::STATE_KEY).expect("missing state");
        },
    );
}

#[test]
fn filter_index_production_reopen_refuses_corrupt_immutable_filter() {
    refuse_mutated_index("filter-corrupt-record", "BASIC", |index, fixture| {
        let key = codec::record_key(fixture.positions[1].block_hash);
        let mut bytes = index.get(&key).expect("read").expect("record").to_vec();
        *bytes.last_mut().expect("encoded filter") ^= 1;
        index.insert(key, bytes).expect("corrupt record");
    });
}

#[test]
fn filter_index_production_reopen_refuses_corrupt_projection() {
    refuse_mutated_index(
        "filter-corrupt-projection",
        "BASIC suffix projection record height",
        |index, fixture| {
            index
                .insert(
                    codec::active_key(1),
                    codec::encode_projection(1, fixture.positions[0].block_hash),
                )
                .expect("wrong projection");
        },
    );
}

#[test]
fn filter_index_production_reopen_refuses_missing_recovered_coins_authority() {
    // Arrange
    let fixture = FilterStartupFixture::new("filter-missing-B", 20, Some(1));
    let db = fjall::Database::builder(&fixture.path)
        .open()
        .expect("raw reopen");
    let coins = db
        .keyspace("coins", fjall::KeyspaceCreateOptions::default)
        .expect("coins");
    coins.remove(encode_best_block_key()).expect("remove B");
    db.persist(fjall::PersistMode::SyncAll).expect("sync");
    drop(coins);
    drop(db);
    let before = snapshot_index(&fixture.path);
    // Act
    let result = fixture.open_runtime();
    // Assert
    assert_filter_refusal(result, "recovered coins best block is absent");
    assert_eq!(snapshot_index(&fixture.path), before);
    let store = FjallNodeStore::open(&fixture.path).expect("inspect");
    assert_payload(&store, &fixture, true, true);
    drop(store);
    fixture.cleanup();
}

#[test]
fn filter_index_production_reopen_refuses_coins_metadata_disagreement() {
    // Arrange
    let fixture = FilterStartupFixture::new("filter-B-mismatch", 20, Some(1));
    let store = FjallNodeStore::open(&fixture.path).expect("store");
    store
        .coins_view()
        .write_raw_bytes(
            &encode_best_block_key(),
            encode_best_block_value(fixture.positions[1].block_hash),
        )
        .expect("older B");
    drop(store);
    let before = snapshot_index(&fixture.path);
    // Act
    let result = fixture.open_runtime();
    // Assert
    assert_filter_refusal(result, "coins best block differs from durable metadata tip");
    assert_eq!(snapshot_index(&fixture.path), before);
    let store = FjallNodeStore::open(&fixture.path).expect("inspect");
    assert_payload(&store, &fixture, true, true);
    drop(store);
    fixture.cleanup();
}

#[test]
fn filter_index_production_reopen_rewinds_ahead_checkpoint_and_strengthens_protection() {
    // Arrange
    let fixture = FilterStartupFixture::new("filter-ahead-rewind", 20, Some(3));
    let store = FjallNodeStore::open(&fixture.path).expect("store");
    seed_authority(&store, &fixture.positions[..2]);
    drop(store);
    raw_index(&fixture.path, |index| {
        index.remove("prune_intent").expect("no intent");
    });
    let before = snapshot_index(&fixture.path);
    // Act
    let runtime = fixture.open_runtime().expect("recovered startup");
    // Assert
    assert_eq!(
        runtime
            .store()
            .maybe_basic_filter_checkpoint()
            .expect("checkpoint"),
        Some(checkpoint(&fixture.records[1]))
    );
    assert_eq!(
        runtime.store().load_prune_locks().expect("protection"),
        vec![
            IndexInputProtection::FromHeight(2)
                .maybe_prune_lock()
                .expect("lock")
        ]
    );
    assert_eq!(
        runtime
            .store()
            .maybe_active_basic_filter_record(2)
            .expect("hidden suffix"),
        None
    );
    assert_payload(runtime.store(), &fixture, true, true);
    drop(runtime);
    let store = FjallNodeStore::open(&fixture.path).expect("durable reconciliation");
    let state = store
        .maybe_basic_filter_state()
        .expect("state")
        .expect("saved");
    assert_eq!(
        state.maybe_endpoint,
        Some((1, fixture.positions[1].block_hash))
    );
    assert_eq!(state.fence_height, 1);
    assert_eq!(state.fence_hash, fixture.positions[1].block_hash);
    assert_eq!(state.protection, IndexInputProtection::FromHeight(2));
    drop(store);
    let immutable_and_projection = |rows: Vec<(Vec<u8>, Vec<u8>)>| {
        rows.into_iter()
            .filter(|(key, _)| {
                key.starts_with(codec::RECORD_PREFIX.as_bytes())
                    || key.starts_with(codec::ACTIVE_PREFIX.as_bytes())
            })
            .collect::<Vec<_>>()
    };
    assert_eq!(
        immutable_and_projection(snapshot_index(&fixture.path)),
        immutable_and_projection(before)
    );
    fixture.cleanup();
}

#[test]
fn filter_index_production_reopen_checks_rewound_required_input_before_mutation() {
    // Arrange
    let fixture = FilterStartupFixture::new("filter-rewind-unsafe", 2, Some(3));
    let store = FjallNodeStore::open(&fixture.path).expect("store");
    seed_authority(&store, &fixture.positions[..2]);
    drop(store);
    let before = snapshot_index(&fixture.path);
    // Act
    let result = fixture.open_runtime();
    // Assert
    assert_filter_refusal(result, "prune intent targets required BASIC input");
    assert_eq!(snapshot_index(&fixture.path), before);
    let store = FjallNodeStore::open(&fixture.path).expect("inspect");
    assert_payload(&store, &fixture, true, true);
    assert_eq!(
        store
            .maybe_basic_filter_checkpoint()
            .expect("old checkpoint"),
        Some(checkpoint(&fixture.records[3]))
    );
    drop(store);
    fixture.cleanup();
}

#[test]
fn filter_index_production_reopen_preserves_required_single_mate() {
    for body in [false, true] {
        // Arrange
        let fixture = FilterStartupFixture::new("filter-unsafe-single-mate", 20, Some(1));
        let hex: String = fixture
            .intent
            .block_hash
            .as_bytes()
            .iter()
            .map(|byte| format!("{byte:02x}"))
            .collect();
        let namespace = if body { "chainstate" } else { "block_index" };
        let key = if body {
            format!("undo:{hex}")
        } else {
            format!("block:{hex}")
        };
        raw_namespace(&fixture.path, namespace, |space| {
            space.remove(key).expect("missing mate");
        });
        let before = snapshot_index(&fixture.path);
        // Act
        let result = fixture.open_runtime();
        // Assert
        assert_filter_refusal(result, "prune intent targets required BASIC input");
        assert_eq!(snapshot_index(&fixture.path), before);
        let store = FjallNodeStore::open(&fixture.path).expect("inspect");
        assert_payload(&store, &fixture, body, !body);
        assert_eq!(
            store.maybe_prune_intent().expect("intent"),
            Some(fixture.intent)
        );
        drop(store);
        fixture.cleanup();
    }
}

#[test]
fn filter_index_production_reopen_finishes_safe_single_mate() {
    for body in [false, true] {
        // Arrange
        let fixture = FilterStartupFixture::new("filter-safe-single-mate", 1, Some(16));
        let hex: String = fixture
            .intent
            .block_hash
            .as_bytes()
            .iter()
            .map(|byte| format!("{byte:02x}"))
            .collect();
        let namespace = if body { "chainstate" } else { "block_index" };
        let key = if body {
            format!("undo:{hex}")
        } else {
            format!("block:{hex}")
        };
        raw_namespace(&fixture.path, namespace, |space| {
            space.remove(key).expect("missing mate");
        });
        // Act
        let runtime = fixture.open_runtime().expect("safe mate resume");
        // Assert
        assert_payload(runtime.store(), &fixture, false, false);
        assert_eq!(runtime.store().maybe_prune_intent().expect("intent"), None);
        assert_eq!(
            runtime
                .store()
                .maybe_basic_filter_checkpoint()
                .expect("checkpoint"),
            Some(checkpoint(&fixture.records[16]))
        );
        drop(runtime);
        fixture.cleanup();
    }
}

#[test]
fn filter_index_production_reopen_refuses_each_lone_partial_artifact() {
    for retained in ["record", "projection", "reserved"] {
        // Arrange
        let fixture = FilterStartupFixture::new("filter-lone-partial", 20, Some(1));
        raw_index(&fixture.path, |index| {
            let keys: Vec<Vec<u8>> = index
                .iter()
                .map(|guard| guard.into_inner().expect("row").0.to_vec())
                .collect();
            for key in keys {
                let keep = match retained {
                    "record" => key.starts_with(codec::RECORD_PREFIX.as_bytes()),
                    "projection" => key.starts_with(codec::ACTIVE_PREFIX.as_bytes()),
                    _ => key == b"prune_locks",
                };
                if !keep && (key.starts_with(b"basic_filter:v1:") || key == b"prune_locks") {
                    index.remove(key).expect("partial artifact");
                }
            }
        });
        let before = snapshot_index(&fixture.path);
        // Act
        let result = fixture.open_runtime();
        // Assert
        assert_filter_refusal(result, "BASIC state is absent but index artifacts exist");
        assert_eq!(snapshot_index(&fixture.path), before);
        let store = FjallNodeStore::open(&fixture.path).expect("inspect");
        assert_payload(&store, &fixture, true, true);
        drop(store);
        fixture.cleanup();
    }
}

#[test]
fn filter_index_production_reopen_refuses_corrupt_live_intent_before_reconciliation() {
    // Arrange
    let fixture = FilterStartupFixture::new("filter-corrupt-intent", 20, Some(1));
    raw_index(&fixture.path, |index| {
        index
            .insert("prune_intent", vec![1])
            .expect("corrupt intent");
    });
    let before = snapshot_index(&fixture.path);
    // Act
    let result = fixture.open_runtime();
    // Assert
    assert_filter_refusal(result, "prune_intent");
    assert_eq!(snapshot_index(&fixture.path), before);
    let store = FjallNodeStore::open(&fixture.path).expect("inspect");
    assert_payload(&store, &fixture, true, true);
    assert!(store.maybe_prune_intent().is_err());
    drop(store);
    fixture.cleanup();
}
