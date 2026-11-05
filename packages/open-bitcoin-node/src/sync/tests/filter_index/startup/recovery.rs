// Parity breadcrumbs:
// - packages/bitcoin-knots/src/index/base.cpp
// - packages/bitcoin-knots/src/index/blockfilterindex.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp

use super::*;

#[test]
fn filter_index_production_reopen_refuses_same_height_saved_fence_conflict() {
    refuse_mutated_index(
        "filter-same-height-fence",
        "BASIC checkpoint differs from same-height saved fence",
        |index, fixture| {
            let state = codec::StoredFilterState {
                maybe_endpoint: Some((1, fixture.positions[1].block_hash)),
                fence_height: 1,
                fence_hash: fixture.positions[0].block_hash,
                protection: IndexInputProtection::FromHeight(2),
            };
            index
                .insert(codec::STATE_KEY, codec::encode_state(state))
                .expect("same-height conflicting fence");
        },
    );
}

#[test]
fn filter_index_production_reopen_reconciles_only_common_recovered_ancestry() {
    for first_changed in [0, 2] {
        // Arrange
        let mut fixture = FilterStartupFixture::new("filter-recovered-fork", 20, Some(3));
        for height in first_changed..=400 {
            let parent = if height == 0 {
                BlockHash::default()
            } else {
                fixture.positions[height - 1].block_hash
            };
            let mut block = fixture_block(parent, height as u32);
            block.header.nonce = 1;
            fixture.positions[height] = ChainPosition::new(
                block.header.clone(),
                height as u32,
                open_bitcoin_core::chainstate::ChainPosition::untrusted_fixture_work(
                    height as u128 + 1,
                ),
                i64::from(block.header.time),
            );
        }
        let store = FjallNodeStore::open(&fixture.path).expect("store");
        seed_authority(&store, &fixture.positions);
        drop(store);
        raw_index(&fixture.path, |index| {
            index.remove("prune_intent").expect("no intent");
        });
        let before = snapshot_index(&fixture.path);
        // Act
        let result = fixture.open_runtime();
        // Assert
        if first_changed == 0 {
            assert_filter_refusal(result, "BASIC index has no common recovered genesis");
            assert_eq!(snapshot_index(&fixture.path), before);
        } else {
            let runtime = result.expect("common genesis startup");
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
            for record in &fixture.records {
                assert_eq!(
                    runtime
                        .store()
                        .load_basic_filter_record(record.identity().block_hash())
                        .expect("immutable fork row"),
                    Some(record.clone())
                );
            }
            assert_eq!(
                runtime
                    .store()
                    .basic_filter_projection(3)
                    .expect("saved suffix"),
                fixture.records[3].identity().block_hash()
            );
            assert_eq!(
                runtime
                    .store()
                    .maybe_active_basic_filter_record(3)
                    .expect("hidden fork"),
                None
            );
            drop(runtime);
        }
        fixture.cleanup();
    }
}

#[test]
fn filter_index_production_reopen_refuses_missing_durable_metadata() {
    // Arrange
    let fixture = FilterStartupFixture::new("filter-missing-metadata", 20, Some(1));
    let store = FjallNodeStore::open(&fixture.path).expect("inspect metadata");
    let (positions, maybe_counts) = store.load_chain_meta_for_open().expect("complete metadata");
    assert_eq!(positions.len(), 401);
    assert_eq!(positions, fixture.positions);
    assert!(maybe_counts.is_none());
    let namespaces = store.basic_operator_namespaces_for_test();
    assert_eq!(namespaces.len(), 9);
    let chainstate = &namespaces[2];
    assert!(!chainstate.iter().any(|(key, _)| key == b"chain_meta"));
    assert!(
        chainstate
            .iter()
            .any(|(key, _)| key == b"chain_meta:segments:v1")
    );
    assert!(
        chainstate
            .iter()
            .any(|(key, _)| key.starts_with(b"chain_meta:segment:v1:"))
    );
    assert!(chainstate.iter().any(|(key, _)| key.starts_with(b"undo:")));
    drop(store);
    remove_metadata_family(&fixture.path);
    let store = FjallNodeStore::open(&fixture.path).expect("inspect absent metadata");
    let (positions, maybe_counts) = store.load_chain_meta_for_open().expect("absent metadata");
    assert!(positions.is_empty());
    assert!(maybe_counts.is_some_and(|counts| counts.is_empty()));
    let tampered_namespaces = store.basic_operator_namespaces_for_test();
    assert_eq!(tampered_namespaces.len(), 9);
    drop(store);
    let before = snapshot_index(&fixture.path);
    // Act
    let result = fixture.open_runtime();
    // Assert
    assert_filter_refusal(result, "durable active metadata is absent or empty");
    assert_eq!(snapshot_index(&fixture.path), before);
    let store = FjallNodeStore::open(&fixture.path).expect("inspect");
    assert_eq!(
        store.basic_operator_namespaces_for_test(),
        tampered_namespaces
    );
    assert_payload(&store, &fixture, true, true);
    assert_eq!(
        store.maybe_prune_intent().expect("intent"),
        Some(fixture.intent)
    );
    drop(store);
    fixture.cleanup();
}

fn is_metadata_key(key: &[u8]) -> bool {
    key == b"chain_meta" || key.starts_with(b"chain_meta:")
}

fn remove_metadata_family(path: &Path) {
    raw_namespace(path, "chainstate", |space| {
        let read_rows = || {
            space
                .iter()
                .map(|guard| {
                    let (key, value) = guard.into_inner().expect("chainstate row");
                    (key.to_vec(), value.to_vec())
                })
                .collect::<Vec<_>>()
        };
        let before = read_rows();
        let owned: Vec<_> = before
            .iter()
            .filter(|(key, _)| is_metadata_key(key))
            .collect();
        assert!(!owned.is_empty(), "metadata removal must change real rows");
        for (key, _) in owned {
            space
                .remove(key.as_slice())
                .expect("remove metadata family row");
        }
        let after = read_rows();
        assert!(!after.iter().any(|(key, _)| is_metadata_key(key)));
        let unrelated: Vec<_> = before
            .into_iter()
            .filter(|(key, _)| !is_metadata_key(key))
            .collect();
        assert_eq!(after, unrelated, "every unrelated chainstate byte survives");
    });
}

#[test]
fn filter_index_metadata_family_removal_preserves_legacy_prefix_collision() {
    // Arrange
    let path = reserved_filter_path("filter-legacy-metadata-family");
    let block = fixture_block(BlockHash::default(), 0);
    let position = ChainPosition::new(
        block.header.clone(),
        0,
        ChainPosition::untrusted_fixture_work(1),
        i64::from(block.header.time),
    );
    let store = FjallNodeStore::open(&path).expect("fresh legacy store");
    store
        .save_chain_meta(std::slice::from_ref(&position), PersistMode::Sync)
        .expect("genuine legacy metadata writer");
    assert_eq!(
        store.load_chain_meta_for_open().expect("legacy metadata").0,
        vec![position]
    );
    drop(store);
    raw_namespace(&path, "chainstate", |space| {
        assert!(space.get("chain_meta").expect("legacy root").is_some());
        assert!(
            space
                .get("chain_meta:segments:v1")
                .expect("no manifest")
                .is_none()
        );
        space
            .insert("chain_metadata", b"preserved unrelated bytes".as_slice())
            .expect("prefix collision");
    });
    // Act
    remove_metadata_family(&path);
    // Assert
    let store = FjallNodeStore::open(&path).expect("inspect legacy absence");
    let (positions, maybe_counts) = store.load_chain_meta_for_open().expect("absent metadata");
    assert!(positions.is_empty());
    assert!(maybe_counts.is_some_and(|counts| counts.is_empty()));
    let namespaces = store.basic_operator_namespaces_for_test();
    assert_eq!(namespaces.len(), 9);
    assert!(namespaces[2].contains(&(
        b"chain_metadata".to_vec(),
        b"preserved unrelated bytes".to_vec()
    )));
    assert!(!namespaces[2].iter().any(|(key, _)| is_metadata_key(key)));
    drop(store);
    fs::remove_dir_all(path).expect("cleanup fresh legacy fixture");
}

#[test]
fn filter_index_production_reopen_preserves_stronger_saved_protection() {
    // Arrange
    let fixture = FilterStartupFixture::new("filter-stronger-protection", 1, Some(16));
    let store = FjallNodeStore::open(&fixture.path).expect("store");
    seed_raw_basic_protection(&store, IndexInputProtection::FromHeight(0));
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
        store.load_prune_locks().expect("stronger protection"),
        vec![
            IndexInputProtection::FromHeight(0)
                .maybe_prune_lock()
                .expect("lock")
        ]
    );
    drop(store);
    fixture.cleanup();
}
