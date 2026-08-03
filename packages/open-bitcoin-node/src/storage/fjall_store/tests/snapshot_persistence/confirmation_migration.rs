// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

use super::*;

#[test]
fn legacy_confirmation_evidence_migrates_from_blocks_before_recovery_classification() {
    // Arrange
    let path = temp_store_path("legacy-confirmation-migration");
    remove_dir_if_exists(&path);
    let genesis = checkpoint_test_block(BlockHash::from_byte_array([0; 32]), 0, 500_000_000);
    let confirmed = checkpoint_spend(
        transaction_txid(&genesis.transactions[0]).expect("genesis txid"),
        499_999_000,
    );
    let confirmed_txid = transaction_txid(&confirmed).expect("confirmed txid");
    let confirming_block = checkpoint_test_block_with_transactions(
        block_hash(&genesis.header),
        1,
        500_001_000,
        vec![confirmed.clone()],
    );
    let final_spend = checkpoint_spend(confirmed_txid, 499_998_000);
    let spending_block = checkpoint_test_block_with_transactions(
        block_hash(&confirming_block.header),
        2,
        500_001_000,
        vec![final_spend],
    );
    let legacy_chainstate = ChainstateSnapshot::new(
        vec![
            ChainPosition::new(genesis.header.clone(), 0, 1, i64::from(genesis.header.time)),
            ChainPosition::new(
                confirming_block.header.clone(),
                1,
                2,
                i64::from(confirming_block.header.time),
            ),
            ChainPosition::new(
                spending_block.header.clone(),
                2,
                3,
                i64::from(spending_block.header.time),
            ),
        ],
        HashMap::new(),
        HashMap::new(),
    );
    let captured_at = PolicyTime::new(i64::from(spending_block.header.time));
    let mempool_snapshot = MempoolSnapshot::try_new_current(
        CapturedMempoolGeneration::new(1),
        captured_at,
        vec![
            MempoolSnapshotRecord::try_from_canonical(
                confirmed,
                MempoolAcceptanceTime::Known(captured_at),
            )
            .expect("confirmed snapshot record"),
        ],
        BTreeSet::new(),
    )
    .expect("mempool snapshot");
    {
        let store = FjallNodeStore::open(&path).expect("open store");
        for block in [&genesis, &confirming_block, &spending_block] {
            store
                .save_block(block, PersistMode::Sync)
                .expect("save active-chain block");
        }
        store
            .save_chainstate_snapshot(&legacy_chainstate, PersistMode::Sync)
            .expect("save legacy chainstate");
        store
            .save_mempool_snapshot(&mempool_snapshot, PersistMode::Sync)
            .expect("save mempool snapshot");
    }
    let reopened = FjallNodeStore::open(&path).expect("reopen store");

    // Act
    let migrated = reopened
        .load_chainstate_snapshot_with_confirmation_migration()
        .expect("migrate confirmation evidence")
        .expect("chainstate snapshot");
    let network = ManagedPeerNetwork::new(
        MemoryChainstateStore::from_snapshot(migrated.clone()),
        LocalPeerConfig::default(),
        PolicyConfig::default(),
    );
    let handle = ManagedNetworkHandle::from_network_fixture(network);
    let prepared = handle
        .prepare_mempool_recovery_at(
            &mempool_snapshot,
            ScriptVerifyFlags::P2SH,
            ConsensusParams::default(),
            captured_at,
        )
        .expect("prepare migrated recovery");
    let summary = handle
        .install_mempool_recovery(prepared)
        .expect("install migrated recovery");

    // Assert
    assert_eq!(
        migrated
            .maybe_confirmed_txid_counts
            .as_ref()
            .and_then(|counts| counts.get(&confirmed_txid)),
        Some(&1)
    );
    assert_eq!(
        summary.records[0].status,
        MempoolRecoveryStatus::DroppedConfirmed
    );
    assert_eq!(
        reopened
            .load_chainstate_snapshot()
            .expect("reload persisted migration"),
        Some(migrated)
    );
    remove_dir_if_exists(&path);
}

#[test]
fn legacy_confirmation_migration_fails_closed_when_active_block_is_missing() {
    // Arrange
    let path = temp_store_path("legacy-confirmation-missing-block");
    remove_dir_if_exists(&path);
    let block = checkpoint_test_block(BlockHash::from_byte_array([0; 32]), 0, 500_000_000);
    let legacy_chainstate = ChainstateSnapshot::new(
        vec![ChainPosition::new(block.header, 0, 1, 1_231_006_500)],
        HashMap::new(),
        HashMap::new(),
    );
    let store = FjallNodeStore::open(&path).expect("open store");
    store
        .save_chainstate_snapshot(&legacy_chainstate, PersistMode::Sync)
        .expect("save legacy chainstate");

    // Act
    let error = store
        .load_chainstate_snapshot_with_confirmation_migration()
        .expect_err("missing durable block must fail closed");

    // Assert
    assert!(matches!(
        error,
        StorageError::Corruption {
            namespace: StorageNamespace::Chainstate,
            ref detail,
            ..
        } if detail.contains("required for confirmation migration")
    ));
    remove_dir_if_exists(&path);
}

#[test]
fn legacy_confirmation_migration_rejects_block_stored_under_wrong_active_key() {
    // Arrange
    let path = temp_store_path("legacy-confirmation-wrong-block");
    remove_dir_if_exists(&path);
    let expected = checkpoint_test_block(BlockHash::from_byte_array([0; 32]), 0, 500_000_000);
    let mut different = checkpoint_test_block(BlockHash::from_byte_array([0; 32]), 1, 500_000_000);
    different.header.time = different.header.time.saturating_add(1);
    let expected_hash = block_hash(&expected.header);
    let legacy_chainstate = ChainstateSnapshot::new(
        vec![ChainPosition::new(expected.header, 0, 1, 1_231_006_500)],
        HashMap::new(),
        HashMap::new(),
    );
    let store = FjallNodeStore::open(&path).expect("open store");
    store
        .save_chainstate_snapshot(&legacy_chainstate, PersistMode::Sync)
        .expect("save legacy chainstate");
    let different_bytes = open_bitcoin_core::codec::encode_block(&different).expect("encode block");
    store
        .put_bytes(
            StorageNamespace::BlockIndex,
            &super::super::super::block_key(expected_hash),
            different_bytes,
            PersistMode::Sync,
        )
        .expect("store mismatched block under active key");

    // Act
    let error = store
        .load_chainstate_snapshot_with_confirmation_migration()
        .expect_err("mismatched active-chain block must fail closed");

    // Assert
    assert!(matches!(
        error,
        StorageError::Corruption {
            namespace: StorageNamespace::Chainstate,
            ref detail,
            ..
        } if detail.contains("active-chain block identity mismatch")
    ));
    remove_dir_if_exists(&path);
}
