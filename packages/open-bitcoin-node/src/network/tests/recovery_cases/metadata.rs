// Parity breadcrumbs:
// - packages/bitcoin-knots/src/node/mempool_persist.cpp
// - packages/bitcoin-knots/src/txmempool.cpp
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/test/functional/mempool_persist.py

use super::*;

#[test]
fn recovery_metadata_managed_local_requested_preserves_facts_and_fanout() {
    // Arrange
    let (mut source, coinbase_txids, _latest_block) =
        relay_enabled_network_with_chain(1_090, 2, PolicyConfig::default());
    let transaction = spend_transaction(coinbase_txids[0], 499_999_000);
    let transaction_txid = txid(&transaction);
    source
        .submit_local_transaction_outcome_at(
            transaction.clone(),
            verify_flags(),
            consensus_params(),
            90,
            RelayIntent::Requested,
        )
        .expect("admit local requested");
    let acceptance_time = MempoolAcceptanceTime::Known(PolicyTime::from_unix_seconds(90));
    let snapshot = MempoolSnapshot::try_new_current(
        CapturedMempoolGeneration::new(7),
        PolicyTime::from_unix_seconds(100),
        vec![
            MempoolSnapshotRecord::try_from_canonical(transaction, acceptance_time)
                .expect("canonical snapshot record"),
        ],
        BTreeSet::from([MempoolMemberIdentity {
            txid: transaction_txid,
            wtxid: source
                .mempool()
                .mempool()
                .entry(&transaction_txid)
                .expect("source entry")
                .wtxid,
        }]),
    )
    .expect("current snapshot");
    let expected = MempoolEntryMetadata::new(
        MempoolAcceptanceTime::Known(PolicyTime::from_unix_seconds(90)),
        MempoolOrigin::Local,
        RelayIntent::Requested,
    );
    let (recovered, _coinbase_txids, _latest_block) =
        relay_enabled_network_with_chain(1_091, 2, PolicyConfig::default());

    // Act
    let (recovered_handle, summary) = prepare_and_install_mempool_recovery(
        recovered,
        &snapshot,
        PolicyTime::from_unix_seconds(8_700),
    );
    let recovered = recovered_handle
        .authority_snapshot_for_test()
        .expect("known-local authority");

    // Assert
    assert_eq!(summary.recovered_count, 1);
    assert_eq!(snapshot.records[0].acceptance_time, acceptance_time);
    let entry = recovered
        .mempool()
        .mempool()
        .entry(&transaction_txid)
        .expect("recovered entry");
    assert_eq!(entry.metadata, expected);
    assert!(entry.metadata.is_retry_eligible(true));
    assert_eq!(recovered.relay_fanout_info().known_transactions, 1);
    assert_eq!(recovered.relay_fanout_info().queued_transactions, 0);
}

#[test]
fn recovery_metadata_managed_duplicate_uses_preserved_age_without_historical_origin() {
    // Arrange
    let (network, coinbase_txids, _latest_block) =
        relay_enabled_network_with_chain(1_092, 2, PolicyConfig::default());
    let transaction = spend_transaction(coinbase_txids[0], 499_999_000);
    let transaction_txid = txid(&transaction);
    let original = MempoolEntryMetadata::new(
        MempoolAcceptanceTime::Known(PolicyTime::from_unix_seconds(90)),
        MempoolOrigin::Local,
        RelayIntent::Requested,
    );
    let conflicting = MempoolEntryMetadata::new(
        MempoolAcceptanceTime::Known(PolicyTime::from_unix_seconds(999)),
        MempoolOrigin::Peer,
        RelayIntent::NotRequested,
    );
    let snapshot = MempoolSnapshot::from_legacy_v1(vec![
        snapshot_record_with_metadata(transaction.clone(), original),
        snapshot_record_with_metadata(transaction, conflicting),
    ]);

    // Act
    let (handle, summary) = prepare_and_install_mempool_recovery(
        network,
        &snapshot,
        PolicyTime::from_unix_seconds(8_800),
    );
    let network = handle
        .authority_snapshot_for_test()
        .expect("duplicate-metadata authority");

    // Assert
    assert_eq!(summary.recovered_count, 1);
    assert_eq!(summary.dropped_duplicate_count, 1);
    assert_eq!(
        network
            .mempool()
            .mempool()
            .entry(&transaction_txid)
            .expect("original")
            .metadata,
        MempoolEntryMetadata::new(
            original.accepted_at,
            MempoolOrigin::RecoveryUnknown,
            RelayIntent::NotRequested,
        )
    );
}
