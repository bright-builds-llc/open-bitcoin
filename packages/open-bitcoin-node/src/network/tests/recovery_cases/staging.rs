// Parity breadcrumbs:
// - packages/bitcoin-knots/src/node/mempool_persist.cpp
// - packages/bitcoin-knots/src/txmempool.cpp
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/test/functional/mempool_persist.py

use super::*;

#[test]
fn staged_recovery_applies_exact_expiry_cutoff_and_preserves_restart_baseline() {
    // Arrange
    let config = PolicyConfig {
        mempool_expiry_hours: 1,
        ..PolicyConfig::default()
    };
    let (network, coinbase_txids, _latest_block) =
        relay_enabled_network_with_chain(1_093, 3, config.clone());
    let expired = spend_transaction(coinbase_txids[0], 499_999_000);
    let cutoff_survivor = spend_transaction(coinbase_txids[1], 499_998_000);
    let expired_identity = MempoolMemberIdentity {
        txid: txid(&expired),
        wtxid: wtxid(&expired),
    };
    let survivor_identity = MempoolMemberIdentity {
        txid: txid(&cutoff_survivor),
        wtxid: wtxid(&cutoff_survivor),
    };
    let captured_at = PolicyTime::from_unix_seconds(2_000);
    let snapshot = MempoolSnapshot::try_new_current(
        CapturedMempoolGeneration::new(42),
        captured_at,
        vec![
            MempoolSnapshotRecord::try_from_canonical(
                expired,
                MempoolAcceptanceTime::Known(PolicyTime::from_unix_seconds(1_000)),
            )
            .expect("expired source record"),
            MempoolSnapshotRecord::try_from_canonical(
                cutoff_survivor,
                MempoolAcceptanceTime::Known(PolicyTime::from_unix_seconds(1_001)),
            )
            .expect("cutoff source record"),
        ],
        BTreeSet::from([expired_identity, survivor_identity]),
    )
    .expect("current snapshot");
    let startup_at = PolicyTime::from_unix_seconds(4_601);

    // Act
    let prepared = network
        .prepare_mempool_recovery_at(&snapshot, verify_flags(), consensus_params(), startup_at)
        .expect("prepare staged recovery");
    let summary = ManagedMempoolRecoverySummary::from_records(prepared.recovery_records().to_vec());
    let fresh = open_bitcoin_mempool::Mempool::new(config);

    // Assert
    assert!(network.mempool().mempool().entries().is_empty());
    assert_eq!(summary.recovered_count, 1);
    assert_eq!(summary.dropped_expired_count, 1);
    assert!(
        prepared
            .staged_mempool()
            .entry(&expired_identity.txid)
            .is_none()
    );
    let survivor_entry = prepared
        .staged_mempool()
        .entry(&survivor_identity.txid)
        .expect("cutoff survivor entry");
    assert_eq!(survivor_entry.metadata.origin, MempoolOrigin::Local);
    assert_eq!(survivor_entry.metadata.relay_intent, RelayIntent::Requested);
    assert_eq!(
        prepared.unbroadcast_members(),
        &BTreeSet::from([survivor_identity])
    );
    assert_eq!(
        prepared.captured_generation(),
        Some(CapturedMempoolGeneration::new(42))
    );
    assert_eq!(prepared.captured_at(), Some(captured_at));
    assert_eq!(prepared.startup_at(), startup_at);
    assert_eq!(
        prepared.staged_mempool().rolling_mempool_fee_rate(),
        fresh.rolling_mempool_fee_rate()
    );
    assert_eq!(
        prepared.staged_mempool().rolling_fee_decay_gate_open(),
        fresh.rolling_fee_decay_gate_open()
    );
    assert_eq!(
        prepared.staged_mempool().rolling_fee_last_update(),
        fresh.rolling_fee_last_update()
    );
}

#[test]
fn staged_recovery_salvages_independent_and_chainstate_backed_components() {
    // Arrange
    let (network, coinbase_txids, _latest_block) =
        relay_enabled_network_with_chain(1_094, 3, PolicyConfig::default());
    let chainstate_backed = spend_transaction(coinbase_txids[0], 499_999_000);
    let absent_parent = spend_transaction(Txid::from_byte_array([94_u8; 32]), 9_000);
    let absent_child = spend_transaction(txid(&absent_parent), 8_000);
    let snapshot = snapshot_from_transactions(vec![
        absent_child.clone(),
        chainstate_backed.clone(),
        absent_parent.clone(),
    ]);

    // Act
    let prepared = network
        .prepare_mempool_recovery_at(
            &snapshot,
            verify_flags(),
            consensus_params(),
            PolicyTime::from_unix_seconds(5_000),
        )
        .expect("prepare staged recovery");
    let summary = ManagedMempoolRecoverySummary::from_records(prepared.recovery_records().to_vec());

    // Assert
    assert_eq!(summary.recovered_count, 1);
    assert_eq!(summary.dropped_missing_parent_count, 2);
    let recovered_entry = prepared
        .staged_mempool()
        .entry(&txid(&chainstate_backed))
        .expect("chainstate-backed entry");
    assert_eq!(
        recovered_entry.metadata.accepted_at,
        MempoolAcceptanceTime::LegacyUnknown
    );
    assert_eq!(
        recovered_entry.metadata.origin,
        MempoolOrigin::RecoveryUnknown
    );
    assert_eq!(
        recovered_entry.metadata.relay_intent,
        RelayIntent::NotRequested
    );
    assert!(
        prepared
            .staged_mempool()
            .entry(&txid(&absent_parent))
            .is_none()
    );
    assert!(
        prepared
            .staged_mempool()
            .entry(&txid(&absent_child))
            .is_none()
    );
}
