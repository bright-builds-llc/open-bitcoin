// Parity breadcrumbs:
// - packages/bitcoin-knots/src/txmempool.h
// - packages/bitcoin-knots/src/txmempool.cpp
// - packages/bitcoin-knots/src/policy/policy.h
// - packages/bitcoin-knots/src/policy/rbf.cpp
// - packages/bitcoin-knots/src/policy/packages.cpp
// - packages/bitcoin-knots/src/rpc/mempool.cpp
// - packages/bitcoin-knots/doc/policy/packages.md

use open_bitcoin_consensus::{ConsensusParams, ScriptVerifyFlags, transaction_txid};
use open_bitcoin_primitives::TransactionInput;

use super::{sample_chainstate_snapshot, spend_transaction, submit};
use crate::{
    AdmissionContext, FeeRate, IncrementalRelayFeeRate, Mempool, MempoolCapacity,
    MempoolRemovalCause, MempoolRemovalRole, PolicyConfig, RollingMempoolFeeRate,
    TransactionVirtualSize, recompute_resource_ledger,
};

fn submit_transition(
    mempool: &mut Mempool,
    snapshot: &open_bitcoin_chainstate::ChainstateSnapshot,
    transaction: open_bitcoin_primitives::Transaction,
) -> crate::MempoolTransition {
    mempool
        .accept_transaction_transition_with_context(
            transaction,
            snapshot,
            ScriptVerifyFlags::P2SH
                | ScriptVerifyFlags::CHECKLOCKTIMEVERIFY
                | ScriptVerifyFlags::CHECKSEQUENCEVERIFY,
            ConsensusParams {
                coinbase_maturity: 1,
                ..ConsensusParams::default()
            },
            AdmissionContext::legacy_unknown(),
        )
        .expect("transition outcome")
}

fn accounted_usage_for_single_spend(
    snapshot: &open_bitcoin_chainstate::ChainstateSnapshot,
    coinbase_txid: open_bitcoin_primitives::Txid,
) -> usize {
    let mut probe = Mempool::default();
    let spend = spend_transaction(
        coinbase_txid,
        0,
        499_999_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    submit(&mut probe, snapshot, spend).expect("probe admission");
    probe.accounted_memory().as_usize()
}

#[test]
fn accounted_capacity_trim_evicts_until_usage_within_capacity() {
    // Arrange
    let (snapshot, coinbase_txids) = sample_chainstate_snapshot(3);
    let one_entry_usage = accounted_usage_for_single_spend(&snapshot, coinbase_txids[2]);
    let low_fee = spend_transaction(
        coinbase_txids[0],
        0,
        499_999_200,
        TransactionInput::SEQUENCE_FINAL,
    );
    let high_fee = spend_transaction(
        coinbase_txids[1],
        0,
        499_998_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    // Capacity fits one accounted entry; leftover legacy vsize limit would evict if still active.
    let mut mempool = Mempool::new(PolicyConfig {
        mempool_capacity: MempoolCapacity::new(one_entry_usage),
        legacy_vsize_trim_limit: TransactionVirtualSize::new(1),
        ..PolicyConfig::default()
    });

    // Act
    let low_fee_result = submit(&mut mempool, &snapshot, low_fee).expect("low fee admission");
    let high_fee_result = submit(&mut mempool, &snapshot, high_fee).expect("high fee admission");

    // Assert
    assert_eq!(high_fee_result.evicted, vec![low_fee_result.accepted]);
    assert!(
        mempool.accounted_memory().as_usize() <= mempool.config().mempool_capacity.as_usize(),
        "accounted memory must drive trim to within MempoolCapacity"
    );
    assert!(
        mempool.total_virtual_size().as_usize()
            > mempool.config().legacy_vsize_trim_limit.as_usize(),
        "total_virtual_size must not be the active trim limiter"
    );
    assert!(mempool.entry(&low_fee_result.accepted).is_none());
    assert!(mempool.entry(&high_fee_result.accepted).is_some());
}

#[test]
fn pressure_bump_uses_descendant_package_feerate_plus_incremental() {
    // Arrange
    let (snapshot, coinbase_txids) = sample_chainstate_snapshot(3);
    let low_fee_parent = spend_transaction(
        coinbase_txids[0],
        0,
        499_999_900,
        TransactionInput::SEQUENCE_FINAL,
    );
    let low_fee_parent_txid = transaction_txid(&low_fee_parent).expect("parent txid");
    let low_fee_child = spend_transaction(
        low_fee_parent_txid,
        0,
        499_998_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    let high_fee = spend_transaction(
        coinbase_txids[1],
        0,
        499_997_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    let incremental = IncrementalRelayFeeRate::new(FeeRate::from_sats_per_kvb(1_000));
    let mut staging = Mempool::new(PolicyConfig {
        incremental_relay_fee_rate: incremental,
        ..PolicyConfig::default()
    });
    submit(&mut staging, &snapshot, low_fee_parent.clone()).expect("stage parent");
    submit(&mut staging, &snapshot, low_fee_child.clone()).expect("stage child");
    let parent_entry = staging
        .entry(&low_fee_parent_txid)
        .expect("staged parent entry")
        .clone();
    let package_feerate = FeeRate::from_fee_sats_and_vbytes(
        parent_entry.descendant_stats.total_fee_sats,
        parent_entry.descendant_stats.virtual_size,
    );
    let expected_rolling = FeeRate::from_sats_per_kvb(
        package_feerate
            .sats_per_kvb()
            .saturating_add(incremental.fee_rate().sats_per_kvb()),
    );
    let package_usage = staging.accounted_memory().as_usize();
    let mut mempool = Mempool::new(PolicyConfig {
        mempool_capacity: MempoolCapacity::new(package_usage),
        incremental_relay_fee_rate: incremental,
        legacy_vsize_trim_limit: TransactionVirtualSize::new(300_000_000),
        ..PolicyConfig::default()
    });
    submit(&mut mempool, &snapshot, low_fee_parent).expect("parent admission");
    submit(&mut mempool, &snapshot, low_fee_child).expect("child admission");

    // Act
    submit(&mut mempool, &snapshot, high_fee).expect("pressure admission");

    // Assert
    assert_eq!(
        mempool.rolling_mempool_fee_rate().fee_rate().sats_per_kvb(),
        expected_rolling.sats_per_kvb(),
        "rolling floor must bump from package feerate + incremental"
    );
    assert!(mempool.entry(&low_fee_parent_txid).is_none());
}

#[test]
fn pressure_bump_skips_when_not_strictly_greater() {
    // Arrange
    let (snapshot, coinbase_txids) = sample_chainstate_snapshot(3);
    let low_fee = spend_transaction(
        coinbase_txids[0],
        0,
        499_999_200,
        TransactionInput::SEQUENCE_FINAL,
    );
    let high_fee = spend_transaction(
        coinbase_txids[1],
        0,
        499_998_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    let one_entry_usage = accounted_usage_for_single_spend(&snapshot, coinbase_txids[2]);
    let mut mempool = Mempool::new(PolicyConfig {
        mempool_capacity: MempoolCapacity::new(one_entry_usage),
        legacy_vsize_trim_limit: TransactionVirtualSize::new(300_000_000),
        ..PolicyConfig::default()
    });
    submit(&mut mempool, &snapshot, low_fee).expect("low fee admission");
    let victim = mempool
        .entries()
        .values()
        .next()
        .expect("victim entry after low-fee admit")
        .clone();
    let package_plus_incremental = FeeRate::from_fee_sats_and_vbytes(
        victim.descendant_stats.total_fee_sats,
        victim.descendant_stats.virtual_size,
    )
    .sats_per_kvb()
    .saturating_add(
        mempool
            .config()
            .incremental_relay_fee_rate
            .fee_rate()
            .sats_per_kvb(),
    );
    let preset_rolling =
        RollingMempoolFeeRate::new(FeeRate::from_sats_per_kvb(package_plus_incremental + 1));
    mempool.set_rolling_mempool_fee_rate(preset_rolling);

    // Act
    let admission = submit(&mut mempool, &snapshot, high_fee).expect("high fee admission");

    // Assert
    assert!(!admission.evicted.is_empty(), "expected pressure eviction");
    assert_eq!(mempool.rolling_mempool_fee_rate(), preset_rolling);
}

#[test]
fn pressure_removes_victim_and_descendants_with_roles() {
    // Arrange
    let (snapshot, coinbase_txids) = sample_chainstate_snapshot(3);
    let low_fee_parent = spend_transaction(
        coinbase_txids[0],
        0,
        499_999_900,
        TransactionInput::SEQUENCE_FINAL,
    );
    let low_fee_parent_txid = transaction_txid(&low_fee_parent).expect("parent txid");
    let low_fee_child = spend_transaction(
        low_fee_parent_txid,
        0,
        499_998_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    let low_fee_child_txid = transaction_txid(&low_fee_child).expect("child txid");
    let high_fee = spend_transaction(
        coinbase_txids[1],
        0,
        499_997_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    let mut staging = Mempool::default();
    submit(&mut staging, &snapshot, low_fee_parent.clone()).expect("stage parent");
    submit(&mut staging, &snapshot, low_fee_child.clone()).expect("stage child");
    let package_usage = staging.accounted_memory().as_usize();
    let mut mempool = Mempool::new(PolicyConfig {
        mempool_capacity: MempoolCapacity::new(package_usage),
        legacy_vsize_trim_limit: TransactionVirtualSize::new(300_000_000),
        ..PolicyConfig::default()
    });
    submit(&mut mempool, &snapshot, low_fee_parent).expect("parent admission");
    submit(&mut mempool, &snapshot, low_fee_child).expect("child admission");

    // Act
    let transition = submit_transition(&mut mempool, &snapshot, high_fee);

    // Assert
    assert!(transition.delta.removed.iter().any(|removal| {
        removal.member.txid == low_fee_parent_txid
            && removal.cause == MempoolRemovalCause::Pressure
            && removal.role == MempoolRemovalRole::Direct
    }));
    assert!(transition.delta.removed.iter().any(|removal| {
        removal.member.txid == low_fee_child_txid
            && removal.cause == MempoolRemovalCause::Pressure
            && removal.role == MempoolRemovalRole::Descendant
    }));
    assert!(mempool.entry(&low_fee_parent_txid).is_none());
    assert!(mempool.entry(&low_fee_child_txid).is_none());
    for entry in mempool.entries().values() {
        for parent_txid in &entry.parents {
            assert!(
                mempool.entry(parent_txid).is_some(),
                "orphan child retained after pressure removal"
            );
        }
    }
    let oracle = recompute_resource_ledger(mempool.entries(), &mempool.spent_outpoints)
        .expect("oracle recomputation");
    assert_eq!(mempool.resource_ledger(), oracle);
}
