// Parity breadcrumbs:
// - packages/bitcoin-knots/src/rpc/mempool.cpp
// - packages/bitcoin-knots/src/validation.cpp

use open_bitcoin_core::{
    consensus::{block_hash, transaction_txid},
    primitives::{BlockHash, Transaction, Txid},
};
use open_bitcoin_mempool::{
    MempoolMemberIdentity, MempoolOrigin, PackageMemberResult, PackageShapeError, PolicyConfig,
    RelayIntent, SubmittedPackageResult,
};

use super::{build_block, consensus_params, local_config, spend_transaction, verify_flags};
use crate::{
    ManagedNetworkAuthorityError, ManagedNetworkError, ManagedNetworkHandle, ManagedPeerNetwork,
    MemoryChainstateStore,
};

fn handle_with_spendable_chain(nonce: u64) -> (ManagedNetworkHandle, Vec<Txid>) {
    let mut network = ManagedPeerNetwork::new(
        MemoryChainstateStore::default(),
        local_config(nonce),
        PolicyConfig::default(),
    );
    let genesis = build_block(BlockHash::from_byte_array([0_u8; 32]), 0, 500_000_000);
    let spendable = build_block(block_hash(&genesis.header), 1, 500_000_000);
    let coinbase_txids = vec![
        transaction_txid(&genesis.transactions[0]).expect("genesis txid"),
        transaction_txid(&spendable.transactions[0]).expect("spendable txid"),
    ];
    network
        .connect_local_block(&genesis, verify_flags(), consensus_params())
        .expect("connect genesis");
    network
        .connect_local_block(&spendable, verify_flags(), consensus_params())
        .expect("connect spendable");
    (
        ManagedNetworkHandle::from_network_fixture(network),
        coinbase_txids,
    )
}

fn standard_spend(coinbase_txid: Txid) -> Transaction {
    spend_transaction(coinbase_txid, 499_999_000)
}

fn dirty_generation(handle: &ManagedNetworkHandle) -> Option<u64> {
    handle
        .authority_snapshot_for_test()
        .expect("authority snapshot")
        .dirty_generation()
        .map(|generation| generation.raw())
}

fn present_member_identity(submitted: &SubmittedPackageResult) -> MempoolMemberIdentity {
    let member = submitted
        .report
        .members()
        .first()
        .expect("submitted package reports a member");
    assert!(matches!(
        member,
        PackageMemberResult::FinallyPresent(_) | PackageMemberResult::AlreadyPresent(_)
    ));
    member.requested_identity()
}

#[test]
fn dry_run_local_package_returns_report_without_changing_mempool_count() {
    // Arrange
    let (handle, coinbase_txids) = handle_with_spendable_chain(137_020);
    let transaction = standard_spend(coinbase_txids[0]);
    let count_before = handle
        .mempool_info()
        .expect("mempool info before")
        .transaction_count;

    // Act
    let report = handle
        .dry_run_local_package(
            vec![transaction],
            verify_flags(),
            consensus_params(),
            80,
            RelayIntent::Requested,
        )
        .expect("valid one-member dry-run");

    // Assert
    assert!(matches!(
        report.members().first(),
        Some(PackageMemberResult::FinallyPresent(_))
    ));
    assert_eq!(
        handle
            .mempool_info()
            .expect("mempool info after")
            .transaction_count,
        count_before
    );
}

#[test]
fn dry_run_local_package_does_not_change_rolling_fee_or_relay_counters() {
    // Arrange
    let (handle, coinbase_txids) = handle_with_spendable_chain(137_021);
    let transaction = standard_spend(coinbase_txids[0]);
    let rolling_before = handle
        .mempool_info()
        .expect("mempool info before")
        .rolling_mempool_fee_rate_sats_per_kvb;
    let relay_before = handle
        .operator_snapshot()
        .expect("operator snapshot before")
        .relay()
        .clone();

    // Act
    handle
        .dry_run_local_package(
            vec![transaction],
            verify_flags(),
            consensus_params(),
            81,
            RelayIntent::Requested,
        )
        .expect("valid one-member dry-run");

    // Assert
    assert_eq!(
        handle
            .mempool_info()
            .expect("mempool info after")
            .rolling_mempool_fee_rate_sats_per_kvb,
        rolling_before
    );
    assert_eq!(
        handle
            .operator_snapshot()
            .expect("operator snapshot after")
            .relay(),
        &relay_before
    );
}

#[test]
fn dry_run_local_package_does_not_dirty_checkpoint_generation() {
    // Arrange
    let (handle, coinbase_txids) = handle_with_spendable_chain(137_022);
    let transaction = standard_spend(coinbase_txids[0]);
    let dirty_before = dirty_generation(&handle);

    // Act
    handle
        .dry_run_local_package(
            vec![transaction],
            verify_flags(),
            consensus_params(),
            82,
            RelayIntent::Requested,
        )
        .expect("valid one-member dry-run");

    // Assert
    assert_eq!(dirty_generation(&handle), dirty_before);
}

#[test]
fn dry_run_local_package_rejects_empty_without_mutation() {
    // Arrange
    let (handle, _) = handle_with_spendable_chain(137_023);
    let info_before = handle.mempool_info().expect("mempool info before");
    let relay_before = handle
        .operator_snapshot()
        .expect("operator snapshot before")
        .relay()
        .clone();
    let dirty_before = dirty_generation(&handle);

    // Act
    let result = handle.dry_run_local_package(
        Vec::new(),
        verify_flags(),
        consensus_params(),
        83,
        RelayIntent::Requested,
    );

    // Assert
    assert!(matches!(
        result,
        Err(ManagedNetworkAuthorityError::Operation(
            ManagedNetworkError::PackageShape(PackageShapeError::Empty)
        ))
    ));
    let info_after = handle.mempool_info().expect("mempool info after");
    assert_eq!(info_after.transaction_count, info_before.transaction_count);
    assert_eq!(
        info_after.rolling_mempool_fee_rate_sats_per_kvb,
        info_before.rolling_mempool_fee_rate_sats_per_kvb
    );
    assert_eq!(
        handle
            .operator_snapshot()
            .expect("operator snapshot after")
            .relay(),
        &relay_before
    );
    assert_eq!(dirty_generation(&handle), dirty_before);
}

#[test]
fn submit_local_package_admits_one_member_through_local_source() {
    // Arrange
    let (handle, coinbase_txids) = handle_with_spendable_chain(137_024);
    let transaction = standard_spend(coinbase_txids[0]);
    let count_before = handle
        .mempool_info()
        .expect("mempool info before")
        .transaction_count;

    // Act
    let submitted = handle
        .submit_local_package(
            vec![transaction],
            verify_flags(),
            consensus_params(),
            90,
            RelayIntent::Requested,
        )
        .expect("valid one-member local submit");

    // Assert
    let identity = present_member_identity(&submitted);
    assert_eq!(
        handle
            .mempool_info()
            .expect("mempool info after")
            .transaction_count,
        count_before + 1
    );
    let metadata = handle
        .mempool_entry_metadata(&identity.txid)
        .expect("metadata read")
        .expect("local member is present");
    assert_eq!(metadata.origin, MempoolOrigin::Local);
}

#[test]
fn submit_local_package_does_not_call_peer_bridge() {
    // Arrange
    let (handle, coinbase_txids) = handle_with_spendable_chain(137_025);
    let transaction = standard_spend(coinbase_txids[0]);

    // Act
    let submitted = handle
        .submit_local_package(
            vec![transaction],
            verify_flags(),
            consensus_params(),
            91,
            RelayIntent::Requested,
        )
        .expect("valid one-member local submit");

    // Assert
    let identity = present_member_identity(&submitted);
    let network = handle
        .authority_snapshot_for_test()
        .expect("authority snapshot");
    assert_eq!(network.orphan_count(), 0);
    assert!(
        !network
            .peer_manager()
            .reconsiderable_transaction_contains(identity.wtxid)
    );
    assert!(network.mempool().mempool().entry(&identity.txid).is_some());
    assert!(
        !handle
            .authority_debug_snapshot_for_test()
            .expect("debug snapshot")
            .contains("ManagedPeerPackageAdmission")
    );
}

#[test]
fn submit_local_package_not_requested_still_accepts_locally() {
    // Arrange
    let (handle, coinbase_txids) = handle_with_spendable_chain(137_026);
    let transaction = standard_spend(coinbase_txids[0]);

    // Act
    let submitted = handle
        .submit_local_package(
            vec![transaction],
            verify_flags(),
            consensus_params(),
            92,
            RelayIntent::NotRequested,
        )
        .expect("not-requested local submit still admits");

    // Assert
    let identity = present_member_identity(&submitted);
    let metadata = handle
        .mempool_entry_metadata(&identity.txid)
        .expect("metadata read")
        .expect("local member is present");
    assert_eq!(metadata.origin, MempoolOrigin::Local);
    assert_eq!(metadata.relay_intent, RelayIntent::NotRequested);
}

#[test]
fn submit_local_package_shape_error_does_not_apply_lifecycle() {
    // Arrange
    let (handle, _) = handle_with_spendable_chain(137_027);
    let count_before = handle
        .mempool_info()
        .expect("mempool info before")
        .transaction_count;
    let dirty_before = dirty_generation(&handle);

    // Act
    let result = handle.submit_local_package(
        Vec::new(),
        verify_flags(),
        consensus_params(),
        93,
        RelayIntent::Requested,
    );

    // Assert
    assert!(matches!(
        result,
        Err(ManagedNetworkAuthorityError::Operation(
            ManagedNetworkError::PackageShape(PackageShapeError::Empty)
        ))
    ));
    assert_eq!(
        handle
            .mempool_info()
            .expect("mempool info after")
            .transaction_count,
        count_before
    );
    assert_eq!(dirty_generation(&handle), dirty_before);
}
