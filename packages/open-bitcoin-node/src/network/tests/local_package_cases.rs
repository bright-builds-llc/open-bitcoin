// Parity breadcrumbs:
// - packages/bitcoin-knots/src/rpc/mempool.cpp
// - packages/bitcoin-knots/src/validation.cpp

use open_bitcoin_core::{
    consensus::{block_hash, transaction_txid},
    primitives::{BlockHash, Transaction, Txid},
};
use open_bitcoin_mempool::{PackageMemberResult, PackageShapeError, PolicyConfig, RelayIntent};

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
