// Parity breadcrumbs:
// - packages/bitcoin-knots/src/node/mempool_persist.cpp
// - packages/bitcoin-knots/src/txmempool.cpp
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/test/functional/mempool_persist.py

use open_bitcoin_network::{RetryDecisionContext, RetryJitterSeconds, next_retry_due_unix_seconds};

use super::*;
use crate::network::lifecycle_projection::{
    RecoveryInstallFailureGuard, RecoveryInstallFailurePoint,
};

fn member_identity(transaction: &Transaction) -> MempoolMemberIdentity {
    MempoolMemberIdentity {
        txid: txid(transaction),
        wtxid: wtxid(transaction),
    }
}

fn injected_retry_context() -> RetryDecisionContext {
    RetryDecisionContext::new(2_000, RetryJitterSeconds::new(10).expect("valid jitter"))
}

#[test]
fn recovery_restart_preserves_local_package_unbroadcast_remints_retry_and_keeps_injected_install_failure_inert()
 {
    // Arrange
    let (source_network, coinbase_txids, _latest_block) =
        relay_enabled_network_with_chain(1_138, 2, PolicyConfig::default());
    let parent = spend_transaction(coinbase_txids[0], 499_999_000);
    let child = spend_transaction(txid(&parent), 499_998_000);
    let parent_identity = member_identity(&parent);
    let child_identity = member_identity(&child);
    let source = ManagedNetworkHandle::from_network_fixture(source_network);
    source
        .submit_local_transaction_outcome_at(
            parent.clone(),
            verify_flags(),
            consensus_params(),
            90,
            RelayIntent::Requested,
        )
        .expect("admit local parent");
    source
        .submit_local_transaction_outcome_at(
            child.clone(),
            verify_flags(),
            consensus_params(),
            91,
            RelayIntent::Requested,
        )
        .expect("admit local child");
    let captured_at = PolicyTime::from_unix_seconds(100);
    let snapshot = MempoolSnapshot::try_new_current(
        CapturedMempoolGeneration::new(13),
        captured_at,
        vec![
            MempoolSnapshotRecord::try_from_canonical(
                parent,
                MempoolAcceptanceTime::Known(PolicyTime::from_unix_seconds(90)),
            )
            .expect("canonical parent record"),
            MempoolSnapshotRecord::try_from_canonical(
                child,
                MempoolAcceptanceTime::Known(PolicyTime::from_unix_seconds(91)),
            )
            .expect("canonical child record"),
        ],
        BTreeSet::from([parent_identity, child_identity]),
    )
    .expect("source-only package snapshot");

    let (target_network, _target_coinbase_txids, _target_block) =
        relay_enabled_network_with_chain(1_139, 2, PolicyConfig::default());
    let handle = ManagedNetworkHandle::from_network_fixture(target_network);
    let injected_prepared = handle
        .prepare_mempool_recovery_at(
            &snapshot,
            verify_flags(),
            consensus_params(),
            PolicyTime::from_unix_seconds(8_700),
        )
        .expect("prepare injected recovery");
    handle
        .set_retry_timer_for_test(Some(1_000), Some(parent_identity))
        .expect("seed retry timer");
    let baseline = handle
        .authority_debug_snapshot_for_test()
        .expect("pre-install baseline");

    // Act
    let injected_error = {
        let _guard =
            RecoveryInstallFailureGuard::inject(RecoveryInstallFailurePoint::ProjectionBuild);
        handle.install_mempool_recovery(injected_prepared)
    };

    // Assert
    assert!(injected_error.is_err());
    assert_eq!(
        handle
            .authority_debug_snapshot_for_test()
            .expect("aggregate after injected failure"),
        baseline
    );

    // Act
    let prepared = handle
        .prepare_mempool_recovery_at(
            &snapshot,
            verify_flags(),
            consensus_params(),
            PolicyTime::from_unix_seconds(8_700),
        )
        .expect("prepare successful recovery");
    let summary = handle
        .install_mempool_recovery(prepared)
        .expect("install prepared recovery");
    let installed = handle
        .authority_snapshot_for_test()
        .expect("installed authority");

    // Assert
    assert_eq!(summary.recovered_count, 2);
    assert!(
        installed
            .mempool()
            .mempool()
            .entry(&parent_identity.txid)
            .is_some()
    );
    assert!(
        installed
            .mempool()
            .mempool()
            .entry(&child_identity.txid)
            .is_some()
    );
    assert_eq!(
        installed.unbroadcast_members(),
        &BTreeSet::from([parent_identity, child_identity])
    );
    assert_eq!(installed.relay_fanout_info().known_transactions, 2);
    assert_eq!(installed.relay_fanout_info().queued_transactions, 0);
    assert_eq!(installed.relay_serving_info().serveable_transactions, 2);
    assert_eq!(
        installed
            .mempool()
            .mempool()
            .rolling_mempool_fee_rate()
            .fee_rate()
            .sats_per_kvb(),
        0
    );
    assert_eq!(
        handle.retry_timer_for_test().expect("cleared timer"),
        (None, None)
    );

    // Act
    let context = injected_retry_context();
    let outcome = handle
        .maintenance_tick(context)
        .expect("remint from injected context");

    // Assert
    assert_eq!(
        outcome.maybe_next_due_unix_seconds,
        next_retry_due_unix_seconds(context)
    );
    assert_eq!(
        handle.retry_timer_for_test().expect("reminted timer").0,
        next_retry_due_unix_seconds(context)
    );
}
