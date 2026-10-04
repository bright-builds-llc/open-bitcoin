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
                height as u128 + 1,
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
    raw_namespace(&fixture.path, "chainstate", |space| {
        space.remove("chain_meta").expect("missing metadata");
    });
    let before = snapshot_index(&fixture.path);
    // Act
    let result = fixture.open_runtime();
    // Assert
    assert_filter_refusal(result, "durable active metadata is absent or empty");
    assert_eq!(snapshot_index(&fixture.path), before);
    let store = FjallNodeStore::open(&fixture.path).expect("inspect");
    assert_payload(&store, &fixture, true, true);
    assert_eq!(
        store.maybe_prune_intent().expect("intent"),
        Some(fixture.intent)
    );
    drop(store);
    fixture.cleanup();
}

#[test]
fn filter_index_production_reopen_preserves_stronger_saved_protection() {
    // Arrange
    let fixture = FilterStartupFixture::new("filter-stronger-protection", 1, Some(16));
    let store = FjallNodeStore::open(&fixture.path).expect("store");
    store
        .sync_prune_locks(&[IndexInputProtection::FromHeight(0)
            .maybe_prune_lock()
            .expect("lock")])
        .expect("stronger saved protection");
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
