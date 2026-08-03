// Parity breadcrumbs:
// - packages/bitcoin-knots/src/node/mempool_persist.cpp
// - packages/bitcoin-knots/src/txmempool.cpp
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/test/functional/mempool_persist.py

use super::*;
use crate::network::ManagedNetworkHandle;
use crate::network::lifecycle_projection::{
    RecoveryInstallFailureGuard, RecoveryInstallFailurePoint,
};

fn current_snapshot(
    transaction: Transaction,
    generation: u64,
    captured_at: PolicyTime,
) -> MempoolSnapshot {
    let member = MempoolMemberIdentity {
        txid: txid(&transaction),
        wtxid: wtxid(&transaction),
    };
    MempoolSnapshot::try_new_current(
        CapturedMempoolGeneration::new(generation),
        captured_at,
        vec![
            MempoolSnapshotRecord::try_from_canonical(
                transaction,
                MempoolAcceptanceTime::Known(captured_at),
            )
            .expect("canonical recovery record"),
        ],
        BTreeSet::from([member]),
    )
    .expect("current recovery snapshot")
}

#[test]
fn installed_recovery_is_clean_then_the_next_mutation_opens_one_loss_generation() {
    // Arrange
    let (network, coinbase_txids, _latest_block) =
        relay_enabled_network_with_chain(1_098, 3, PolicyConfig::default());
    let recovered = spend_transaction(coinbase_txids[0], 499_999_000);
    let recovered_txid = txid(&recovered);
    let recovered_wtxid = wtxid(&recovered);
    let next = spend_transaction(coinbase_txids[1], 499_998_000);
    let captured_at = PolicyTime::from_unix_seconds(10_000);
    let snapshot = current_snapshot(recovered, 42, captured_at);
    let handle = ManagedNetworkHandle::from_network_fixture(network);
    let prepared = handle
        .prepare_mempool_recovery_at(
            &snapshot,
            verify_flags(),
            consensus_params(),
            PolicyTime::from_unix_seconds(10_100),
        )
        .expect("prepare recovery outside authority");

    // Act
    let summary = handle
        .install_mempool_recovery(prepared)
        .expect("install recovery once");

    // Assert
    assert_eq!(summary.recovered_count, 1);
    let installed = handle
        .authority_snapshot_for_test()
        .expect("installed authority snapshot");
    assert!(
        installed
            .mempool()
            .mempool()
            .entry(&recovered_txid)
            .is_some()
    );
    assert!(installed.transactions_by_txid.contains_key(&recovered_txid));
    assert!(
        installed
            .transactions_by_wtxid
            .contains_key(&recovered_wtxid)
    );
    assert_eq!(installed.relay_fanout_info().known_transactions, 1);
    assert_eq!(installed.relay_fanout_info().queued_transactions, 0);
    assert!(installed.relay_fanout_info().latest_actions.is_empty());
    assert_eq!(
        installed.unbroadcast_members(),
        &BTreeSet::from([MempoolMemberIdentity {
            txid: recovered_txid,
            wtxid: recovered_wtxid,
        }])
    );
    assert_eq!(installed.reconcile_lifecycle_projection().counts(), [0; 7]);
    let clean_evidence = handle
        .checkpoint_evidence(PolicyTime::from_unix_seconds(10_100), 300)
        .expect("clean recovery evidence");
    assert_eq!(clean_evidence.current_generation, 42);
    assert_eq!(clean_evidence.maybe_last_durable_generation, Some(42));
    assert_eq!(clean_evidence.maybe_dirty_generation, None);
    assert_eq!(clean_evidence.maybe_in_flight_generation, None);
    assert_eq!(clean_evidence.maybe_generation_loss_range, None);

    // Act
    handle
        .submit_local_transaction_outcome_at(
            next,
            verify_flags(),
            consensus_params(),
            10_101,
            RelayIntent::NotRequested,
        )
        .expect("first post-recovery mutation");

    // Assert
    let dirty_evidence = handle
        .checkpoint_evidence(PolicyTime::from_unix_seconds(10_101), 300)
        .expect("dirty recovery evidence");
    assert_eq!(dirty_evidence.current_generation, 43);
    assert_eq!(dirty_evidence.maybe_last_durable_generation, Some(42));
    assert_eq!(dirty_evidence.maybe_dirty_generation, Some(43));
    assert_eq!(
        dirty_evidence
            .maybe_generation_loss_range
            .expect("post-recovery loss interval")
            .maybe_after_generation,
        Some(42)
    );
    let mutated = handle
        .authority_snapshot_for_test()
        .expect("post-recovery mutation authority");
    assert_eq!(mutated.reconcile_lifecycle_projection().counts(), [0; 7]);
}

#[test]
fn stale_and_non_fresh_recovery_install_preserve_the_exact_authority_aggregate() {
    // Arrange
    let (first_network, coinbase_txids, _latest_block) =
        relay_enabled_network_with_chain(1_099, 3, PolicyConfig::default());
    let snapshot = current_snapshot(
        spend_transaction(coinbase_txids[0], 499_999_000),
        9,
        PolicyTime::from_unix_seconds(9_000),
    );
    let first = ManagedNetworkHandle::from_network_fixture(first_network);
    let stale = first
        .prepare_mempool_recovery_at(
            &snapshot,
            verify_flags(),
            consensus_params(),
            PolicyTime::from_unix_seconds(9_100),
        )
        .expect("prepare stale recovery");
    let (second_network, _coinbase_txids, _latest_block) =
        relay_enabled_network_with_chain(1_100, 3, PolicyConfig::default());
    let second = ManagedNetworkHandle::from_network_fixture(second_network);
    let stale_baseline = second
        .authority_debug_snapshot_for_test()
        .expect("stale baseline");

    // Act
    let stale_error = second.install_mempool_recovery(stale);

    // Assert
    assert!(stale_error.is_err());
    assert_eq!(
        second
            .authority_debug_snapshot_for_test()
            .expect("stale aggregate after failure"),
        stale_baseline
    );

    // Arrange
    let prepared = first
        .prepare_mempool_recovery_at(
            &snapshot,
            verify_flags(),
            consensus_params(),
            PolicyTime::from_unix_seconds(9_100),
        )
        .expect("prepare current recovery");
    first
        .submit_local_transaction_outcome_at(
            spend_transaction(coinbase_txids[1], 499_998_000),
            verify_flags(),
            consensus_params(),
            9_101,
            RelayIntent::NotRequested,
        )
        .expect("make authority non-fresh");
    let non_fresh_baseline = first
        .authority_debug_snapshot_for_test()
        .expect("non-fresh baseline");

    // Act
    let non_fresh_error = first.install_mempool_recovery(prepared);

    // Assert
    assert!(non_fresh_error.is_err());
    assert_eq!(
        first
            .authority_debug_snapshot_for_test()
            .expect("non-fresh aggregate after failure"),
        non_fresh_baseline
    );
}

#[test]
fn recovery_install_rejects_a_chainstate_change_after_preparation() {
    // Arrange
    let (network, coinbase_txids, latest_block) =
        relay_enabled_network_with_chain(1_107, 3, PolicyConfig::default());
    let snapshot = current_snapshot(
        spend_transaction(coinbase_txids[0], 499_999_000),
        23,
        PolicyTime::from_unix_seconds(23_000),
    );
    let handle = ManagedNetworkHandle::from_network_fixture(network);
    let prepared = handle
        .prepare_mempool_recovery_at(
            &snapshot,
            verify_flags(),
            consensus_params(),
            PolicyTime::from_unix_seconds(23_100),
        )
        .expect("prepare recovery");
    let next_block = build_block(block_hash(&latest_block.header), 3, 500_000_000);
    handle
        .connect_local_block(&next_block, verify_flags(), consensus_params())
        .expect("advance chainstate after preparation");
    let baseline = handle
        .authority_debug_snapshot_for_test()
        .expect("authority baseline");

    // Act
    let error = handle.install_mempool_recovery(prepared);

    // Assert
    assert!(error.is_err());
    assert_eq!(
        handle
            .authority_debug_snapshot_for_test()
            .expect("authority after stale install"),
        baseline
    );
}

#[test]
fn recovery_keeps_unbroadcast_only_for_the_exact_surviving_member() {
    // Arrange
    let (network, coinbase_txids, _latest_block) =
        relay_enabled_network_with_chain(1_108, 3, PolicyConfig::default());
    let surviving = spend_transaction(coinbase_txids[0], 499_999_000);
    let mut same_txid_with_witness = surviving.clone();
    same_txid_with_witness.inputs[0].witness =
        open_bitcoin_core::primitives::ScriptWitness::new(vec![vec![1]]);
    assert_eq!(txid(&surviving), txid(&same_txid_with_witness));
    assert_ne!(wtxid(&surviving), wtxid(&same_txid_with_witness));
    let plain_identity = MempoolMemberIdentity {
        txid: txid(&surviving),
        wtxid: wtxid(&surviving),
    };
    let witnessed_identity = MempoolMemberIdentity {
        txid: txid(&same_txid_with_witness),
        wtxid: wtxid(&same_txid_with_witness),
    };
    let persisted_only = plain_identity.max(witnessed_identity);
    let captured_at = PolicyTime::from_unix_seconds(24_000);
    let snapshot = MempoolSnapshot::try_new_current(
        CapturedMempoolGeneration::new(24),
        captured_at,
        vec![
            MempoolSnapshotRecord::try_from_canonical(
                surviving,
                MempoolAcceptanceTime::Known(captured_at),
            )
            .expect("surviving record"),
            MempoolSnapshotRecord::try_from_canonical(
                same_txid_with_witness,
                MempoolAcceptanceTime::Known(captured_at),
            )
            .expect("alternate witness record"),
        ],
        BTreeSet::from([persisted_only]),
    )
    .expect("same-txid recovery snapshot");

    // Act
    let prepared = network
        .prepare_mempool_recovery_at(
            &snapshot,
            verify_flags(),
            consensus_params(),
            PolicyTime::from_unix_seconds(24_100),
        )
        .expect("prepare recovery");

    // Assert
    assert!(prepared.unbroadcast_members().is_empty());
}

#[test]
fn every_injected_recovery_install_validation_failure_preserves_the_exact_aggregate() {
    for point in RecoveryInstallFailurePoint::ALL {
        // Arrange
        let (network, coinbase_txids, _latest_block) =
            relay_enabled_network_with_chain(1_101 + point as u64, 2, PolicyConfig::default());
        let snapshot = current_snapshot(
            spend_transaction(coinbase_txids[0], 499_999_000),
            17,
            PolicyTime::from_unix_seconds(17_000),
        );
        let handle = ManagedNetworkHandle::from_network_fixture(network);
        let prepared = handle
            .prepare_mempool_recovery_at(
                &snapshot,
                verify_flags(),
                consensus_params(),
                PolicyTime::from_unix_seconds(17_100),
            )
            .expect("prepare injected recovery");
        let baseline = handle
            .authority_debug_snapshot_for_test()
            .expect("injected baseline");
        let _guard = RecoveryInstallFailureGuard::inject(point);

        // Act
        let error = handle.install_mempool_recovery(prepared);

        // Assert
        assert!(error.is_err(), "{point:?}");
        assert_eq!(
            handle
                .authority_debug_snapshot_for_test()
                .expect("injected aggregate after failure"),
            baseline,
            "{point:?}"
        );
    }
}

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
