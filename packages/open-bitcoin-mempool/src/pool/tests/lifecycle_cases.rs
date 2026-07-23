// Parity breadcrumbs:
// - packages/bitcoin-knots/src/txmempool.h
// - packages/bitcoin-knots/src/txmempool.cpp
// - packages/bitcoin-knots/src/validation.cpp

use std::collections::BTreeMap;
use std::hint::black_box;

use open_bitcoin_codec::CodecError;
use open_bitcoin_consensus::transaction_txid;
use open_bitcoin_primitives::{BlockHash, TransactionInput, Txid};

use super::super::lifecycle::{capacity_status, record_reason, txid_serialization_error};
use super::{build_block, sample_chainstate_snapshot, spend_transaction, submit};
use crate::{
    AccountedMempoolMemory, Mempool, MempoolCapacity, MempoolCapacityStatus, MempoolError,
    MempoolLifecycleRemovalReason, PolicyConfig, RollingFeeParityStatus, TransactionVirtualSize,
    transaction_weight_and_virtual_size,
};

#[test]
fn lifecycle_pressure_summary_reports_capacity_and_fee_floor() {
    // Arrange
    let mempool = Mempool::new(PolicyConfig {
        mempool_capacity: MempoolCapacity::new(12_345),
        min_relay_feerate: crate::FeeRate::from_sats_per_kvb(2_000),
        incremental_relay_feerate: crate::FeeRate::from_sats_per_kvb(3_000),
        ..PolicyConfig::default()
    });

    // Act
    let summary = mempool.pressure_summary();

    // Assert
    assert_eq!(summary.transaction_count, 0);
    assert_eq!(summary.total_virtual_size, TransactionVirtualSize::ZERO);
    assert_eq!(summary.accounted_memory, AccountedMempoolMemory::ZERO);
    assert_eq!(summary.mempool_capacity, MempoolCapacity::new(12_345));
    assert_eq!(summary.min_relay_feerate_sats_per_kvb, 2_000);
    assert_eq!(summary.incremental_relay_feerate_sats_per_kvb, 3_000);
    assert_eq!(summary.capacity_status, MempoolCapacityStatus::Empty);
    assert_eq!(summary.capacity_status.as_str(), "empty");
    assert_eq!(summary.rolling_fee_parity, RollingFeeParityStatus::Deferred);
    assert_eq!(summary.rolling_fee_parity.as_str(), "deferred");
}

#[test]
fn lifecycle_labels_and_capacity_statuses_are_stable() {
    // Arrange
    let reasons = [
        (MempoolLifecycleRemovalReason::Confirmed, "confirmed"),
        (MempoolLifecycleRemovalReason::Conflict, "conflict"),
        (MempoolLifecycleRemovalReason::Descendant, "descendant"),
        (MempoolLifecycleRemovalReason::Trimmed, "trimmed"),
    ];
    let capacities = [
        (0, 10, MempoolCapacityStatus::Empty, "empty"),
        (
            1,
            10,
            MempoolCapacityStatus::UnderCapacity,
            "under_capacity",
        ),
        (10, 10, MempoolCapacityStatus::AtCapacity, "at_capacity"),
        (11, 10, MempoolCapacityStatus::OverCapacity, "over_capacity"),
    ];

    // Act
    let capacity_results = capacities
        .into_iter()
        .map(|(total, max, expected, label)| {
            let status = capacity_status(
                AccountedMempoolMemory::new(black_box(total)),
                MempoolCapacity::new(black_box(max)),
            );
            (status, expected, label)
        })
        .collect::<Vec<_>>();
    let reason_debugs = reasons
        .iter()
        .map(|(reason, _label)| format!("{:?}", black_box(*reason)))
        .collect::<Vec<_>>();
    let capacity_debugs = capacity_results
        .iter()
        .map(|(status, _expected, _label)| format!("{:?}", black_box(*status)))
        .collect::<Vec<_>>();
    let rolling_fee = black_box(RollingFeeParityStatus::Deferred);
    let rolling_fee_clone = rolling_fee;

    // Assert
    for (reason, label) in reasons {
        assert_eq!(black_box(reason).as_str(), label);
    }
    for (status, expected, label) in capacity_results {
        assert_eq!(status, expected);
        assert_eq!(black_box(status).as_str(), label);
    }
    assert_eq!(
        reason_debugs,
        ["Confirmed", "Conflict", "Descendant", "Trimmed"]
    );
    assert_eq!(
        capacity_debugs,
        ["Empty", "UnderCapacity", "AtCapacity", "OverCapacity"]
    );
    assert_eq!(rolling_fee_clone, RollingFeeParityStatus::Deferred);
    assert_eq!(format!("{rolling_fee:?}"), "Deferred");
}

#[test]
fn block_connect_removes_confirmed_transaction_and_recomputes_indexes() {
    // Arrange
    let (snapshot, coinbase_txids) = sample_chainstate_snapshot(2);
    let parent = spend_transaction(
        coinbase_txids[0],
        0,
        499_999_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    let parent_txid = transaction_txid(&parent).expect("parent txid");
    let child = spend_transaction(
        parent_txid,
        0,
        499_998_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    let child_txid = transaction_txid(&child).expect("child txid");
    let mut mempool = Mempool::default();
    submit(&mut mempool, &snapshot, parent.clone()).expect("parent admission");
    submit(&mut mempool, &snapshot, child).expect("child admission");
    let mut block = build_block(BlockHash::from_byte_array([0_u8; 32]), 3, 499_999_000);
    block.transactions.push(parent.clone());

    // Act
    let summary = mempool
        .remove_for_connected_block(&block)
        .expect("block cleanup");

    // Assert
    assert_eq!(summary.removed.len(), 1);
    assert_eq!(summary.removed[0].txid, parent_txid);
    assert_eq!(
        summary.removed[0].reason,
        MempoolLifecycleRemovalReason::Confirmed
    );
    assert_eq!(summary.removed[0].reason.as_str(), "confirmed");
    assert!(mempool.entry(&parent_txid).is_none());
    let child_entry = mempool.entry(&child_txid).expect("child remains");
    assert!(child_entry.parents.is_empty());
    assert_eq!(child_entry.ancestor_stats.count, 1);
    assert_eq!(mempool.total_virtual_size(), child_entry.virtual_size);
}

#[test]
fn block_connect_removes_conflict_and_descendants() {
    // Arrange
    let (snapshot, coinbase_txids) = sample_chainstate_snapshot(2);
    let original = spend_transaction(
        coinbase_txids[0],
        0,
        499_999_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    let original_txid = transaction_txid(&original).expect("original txid");
    let descendant = spend_transaction(
        original_txid,
        0,
        499_998_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    let descendant_txid = transaction_txid(&descendant).expect("descendant txid");
    let replacement = spend_transaction(
        coinbase_txids[0],
        0,
        499_997_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    let mut mempool = Mempool::default();
    submit(&mut mempool, &snapshot, original).expect("original admission");
    submit(&mut mempool, &snapshot, descendant).expect("descendant admission");

    // Act
    let summary = mempool
        .remove_for_connected_transactions([&replacement])
        .expect("block cleanup");

    // Assert
    assert_eq!(summary.removed.len(), 2);
    assert!(summary.removed.iter().any(|removal| {
        removal.txid == original_txid && removal.reason == MempoolLifecycleRemovalReason::Conflict
    }));
    assert!(summary.removed.iter().any(|removal| {
        removal.txid == descendant_txid
            && removal.reason == MempoolLifecycleRemovalReason::Descendant
    }));
    assert!(mempool.entry(&original_txid).is_none());
    assert!(mempool.entry(&descendant_txid).is_none());
    assert_eq!(mempool.total_virtual_size(), TransactionVirtualSize::ZERO);
    assert_eq!(
        summary.pressure.capacity_status,
        MempoolCapacityStatus::Empty
    );
}

#[test]
fn block_connect_without_matches_returns_empty_summary() {
    // Arrange
    let (snapshot, coinbase_txids) = sample_chainstate_snapshot(2);
    let existing = spend_transaction(
        coinbase_txids[0],
        0,
        499_999_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    let unrelated = spend_transaction(
        coinbase_txids[1],
        0,
        499_999_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    let mut mempool = Mempool::default();
    let accepted = submit(&mut mempool, &snapshot, existing).expect("admission");

    // Act
    let summary = mempool
        .remove_for_connected_transactions([&unrelated])
        .expect("block cleanup");

    // Assert
    assert!(summary.removed.is_empty());
    assert!(mempool.entry(&accepted.accepted).is_some());
}

#[test]
fn lifecycle_pressure_summary_uses_accounted_capacity_after_admission() {
    // Arrange
    let (snapshot, coinbase_txids) = sample_chainstate_snapshot(1);
    let transaction = spend_transaction(
        coinbase_txids[0],
        0,
        499_999_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    let (_weight, virtual_size) =
        transaction_weight_and_virtual_size(&transaction).expect("transaction size");
    let mut mempool = Mempool::new(PolicyConfig {
        legacy_vsize_trim_limit: TransactionVirtualSize::new(virtual_size),
        ..PolicyConfig::default()
    });

    // Act
    submit(&mut mempool, &snapshot, transaction).expect("admission");
    let summary = mempool.pressure_summary();

    // Assert
    assert_eq!(
        summary.capacity_status,
        MempoolCapacityStatus::UnderCapacity
    );
}

#[test]
fn record_reason_keeps_highest_priority_lifecycle_label() {
    // Arrange
    let txid = Txid::from_byte_array([1_u8; 32]);
    let mut reasons = BTreeMap::new();

    // Act
    record_reason(&mut reasons, txid, MempoolLifecycleRemovalReason::Trimmed);
    record_reason(
        &mut reasons,
        txid,
        MempoolLifecycleRemovalReason::Descendant,
    );
    record_reason(&mut reasons, txid, MempoolLifecycleRemovalReason::Confirmed);
    record_reason(&mut reasons, txid, MempoolLifecycleRemovalReason::Conflict);

    // Assert
    assert_eq!(
        reasons.get(&txid),
        Some(&MempoolLifecycleRemovalReason::Confirmed)
    );
}

#[test]
fn txid_serialization_error_maps_to_mempool_validation_error() {
    // Arrange
    let codec_error = CodecError::CompactSizeTooLarge(33_554_433);

    // Act
    let error = txid_serialization_error(codec_error);

    // Assert
    assert!(matches!(error, MempoolError::Validation { .. }));
    assert!(error.to_string().contains("transaction txid"));
}

#[test]
fn lifecycle_public_types_cover_debug_clone_and_equality_contracts() {
    // Arrange
    let txid = Txid::from_byte_array([2_u8; 32]);
    let wtxid = open_bitcoin_primitives::Wtxid::from_byte_array([3_u8; 32]);
    let removal = crate::MempoolLifecycleRemoval {
        txid,
        wtxid,
        reason: MempoolLifecycleRemovalReason::Trimmed,
    };
    let pressure = crate::MempoolPressureSummary {
        transaction_count: 1,
        total_virtual_size: TransactionVirtualSize::new(2),
        accounted_memory: AccountedMempoolMemory::new(3),
        mempool_capacity: MempoolCapacity::new(4),
        min_relay_feerate_sats_per_kvb: 5,
        incremental_relay_feerate_sats_per_kvb: 6,
        capacity_status: MempoolCapacityStatus::OverCapacity,
        rolling_fee_parity: RollingFeeParityStatus::Deferred,
    };
    let summary = crate::MempoolLifecycleSummary {
        removed: vec![removal.clone()],
        pressure: pressure.clone(),
    };

    // Act
    let debug_text = format!("{:?}{:?}{:?}", removal, pressure, summary);
    let cloned_summary = summary.clone();
    let priorities = [
        MempoolLifecycleRemovalReason::Confirmed.priority(),
        MempoolLifecycleRemovalReason::Conflict.priority(),
        MempoolLifecycleRemovalReason::Descendant.priority(),
        MempoolLifecycleRemovalReason::Trimmed.priority(),
    ];

    // Assert
    assert!(debug_text.contains("MempoolLifecycleSummary"));
    assert_eq!(cloned_summary, summary);
    assert_eq!(priorities, [0, 1, 2, 3]);
}
