// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

use super::*;

#[test]
fn fjall_store_reopens_saved_snapshots_and_metadata() {
    // Arrange
    let path = temp_store_path("reopen-snapshots");
    remove_dir_if_exists(&path);
    let chainstate = chainstate_snapshot();
    let wallet = wallet_snapshot();
    let headers = header_entries();
    let block = block(headers[0].block_hash, 3);
    let block_hash = block_hash(&block.header);
    let metrics = MetricsStorageSnapshot {
        samples: vec![MetricSample::new(MetricKind::SyncHeight, 1.0, 2)],
    };
    let metadata = RuntimeMetadata {
        last_clean_shutdown: true,
        ..RuntimeMetadata::default()
    };

    // Act
    {
        let store = FjallNodeStore::open(&path).expect("open store");
        store
            .save_chainstate_snapshot(&chainstate, PersistMode::Sync)
            .expect("save chainstate");
        store
            .save_wallet_snapshot(&wallet, PersistMode::Sync)
            .expect("save wallet");
        store
            .save_header_entries(&headers, PersistMode::Sync)
            .expect("save headers");
        store
            .save_block(&block, PersistMode::Sync)
            .expect("save block");
        store
            .save_metrics_snapshot(&metrics, PersistMode::Sync)
            .expect("save metrics");
        store
            .save_runtime_metadata(&metadata, PersistMode::Sync)
            .expect("save runtime metadata");
        plant_coins_best_block_for_schema2_reopen(&store);
    }
    let reopened = FjallNodeStore::open(&path).expect("reopen store");

    // Assert
    assert_eq!(
        reopened
            .load_chainstate_snapshot()
            .expect("load chainstate"),
        Some(chainstate)
    );
    assert_eq!(
        reopened.load_wallet_snapshot().expect("load wallet"),
        Some(wallet)
    );
    assert_eq!(
        reopened
            .load_header_entries()
            .expect("load headers")
            .expect("headers")
            .entries,
        headers
    );
    assert_eq!(
        reopened
            .load_block_index_entries()
            .expect("load block index")
            .expect("block index")
            .entries,
        headers
    );
    assert_eq!(
        reopened
            .load_header_store()
            .expect("load header store")
            .expect("header store")
            .best_height(),
        1
    );
    assert_eq!(
        reopened.load_block(block_hash).expect("load block"),
        Some(block)
    );
    assert_eq!(
        reopened
            .load_block(BlockHash::from_byte_array([99_u8; 32]))
            .expect("load missing block"),
        None
    );
    assert_eq!(
        reopened.load_metrics_snapshot().expect("load metrics"),
        Some(metrics)
    );
    assert_eq!(
        reopened.load_runtime_metadata().expect("load metadata"),
        Some(metadata)
    );

    remove_dir_if_exists(&path);
}
