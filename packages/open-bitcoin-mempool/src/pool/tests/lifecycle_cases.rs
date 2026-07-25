// Parity breadcrumbs:
// - packages/bitcoin-knots/src/kernel/mempool_removal_reason.h
// - packages/bitcoin-knots/src/txmempool.h
// - packages/bitcoin-knots/src/txmempool.cpp
// - packages/bitcoin-knots/src/validation.cpp

use std::collections::BTreeMap;
use std::hint::black_box;

use open_bitcoin_codec::CodecError;
use open_bitcoin_consensus::transaction_txid;
use open_bitcoin_primitives::{BlockHash, TransactionInput, Txid};

use super::super::lifecycle::{
    MempoolRemovalFact, capacity_status, record_removal_fact, txid_serialization_error,
};
use super::{build_block, sample_chainstate_snapshot, spend_transaction, submit};
use crate::{
    AccountedMempoolMemory, BlockLifecycleContext, Mempool, MempoolCapacity,
    MempoolCapacityEnforcement, MempoolCapacityStatus, MempoolError, MempoolMemberIdentity,
    MempoolRemovalCause, MempoolRemovalRole, PolicyConfig, PolicyTime, RollingFeeParityStatus,
    TransactionVirtualSize,
};

#[test]
fn lifecycle_pressure_summary_reports_capacity_and_fee_floor() {
    // Arrange
    let mempool = Mempool::new(PolicyConfig {
        mempool_capacity: MempoolCapacity::new(12_345),
        static_relay_fee_rate: crate::StaticRelayFeeRate::new(crate::FeeRate::from_sats_per_kvb(
            2_000,
        )),
        incremental_relay_fee_rate: crate::IncrementalRelayFeeRate::new(
            crate::FeeRate::from_sats_per_kvb(3_000),
        ),
        ..PolicyConfig::default()
    });

    // Act
    let summary = mempool.pressure_summary();

    // Assert
    assert_eq!(summary.transaction_count, 0);
    assert_eq!(summary.total_virtual_size, TransactionVirtualSize::ZERO);
    assert_eq!(summary.accounted_memory, AccountedMempoolMemory::ZERO);
    assert_eq!(summary.mempool_capacity, MempoolCapacity::new(12_345));
    assert_eq!(
        summary.static_relay_fee_rate,
        crate::StaticRelayFeeRate::new(crate::FeeRate::from_sats_per_kvb(2_000))
    );
    assert_eq!(
        summary.incremental_relay_fee_rate,
        crate::IncrementalRelayFeeRate::new(crate::FeeRate::from_sats_per_kvb(3_000))
    );
    assert_eq!(
        summary.rolling_mempool_fee_rate,
        crate::RollingMempoolFeeRate::ZERO
    );
    assert_eq!(
        summary.effective_admission_fee_rate.fee_rate(),
        crate::FeeRate::from_sats_per_kvb(2_000)
    );
    assert_eq!(summary.capacity_status, MempoolCapacityStatus::Empty);
    assert_eq!(summary.capacity_status.as_str(), "empty");
    assert_eq!(
        summary.capacity_enforcement,
        MempoolCapacityEnforcement::AccountedMemory
    );
    assert_eq!(summary.capacity_enforcement.as_str(), "accounted_memory");
    assert_eq!(summary.rolling_fee_parity, RollingFeeParityStatus::Active);
    assert_eq!(summary.rolling_fee_parity.as_str(), "active");
}

#[test]
fn lifecycle_labels_and_capacity_statuses_are_stable() {
    // Arrange
    let causes = [
        (MempoolRemovalCause::BlockConfirmation, "block_confirmation"),
        (MempoolRemovalCause::BlockConflict, "block_conflict"),
        (MempoolRemovalCause::Pressure, "pressure"),
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
    let cause_debugs = causes
        .iter()
        .map(|(cause, _label)| format!("{:?}", black_box(*cause)))
        .collect::<Vec<_>>();
    let capacity_debugs = capacity_results
        .iter()
        .map(|(status, _expected, _label)| format!("{:?}", black_box(*status)))
        .collect::<Vec<_>>();
    let rolling_fee = black_box(RollingFeeParityStatus::Active);
    let rolling_fee_clone = rolling_fee;

    // Assert
    for (cause, label) in causes {
        assert_eq!(black_box(cause).as_str(), label);
    }
    for (status, expected, label) in capacity_results {
        assert_eq!(status, expected);
        assert_eq!(black_box(status).as_str(), label);
    }
    assert_eq!(
        cause_debugs,
        ["BlockConfirmation", "BlockConflict", "Pressure"]
    );
    assert_eq!(
        capacity_debugs,
        ["Empty", "UnderCapacity", "AtCapacity", "OverCapacity"]
    );
    assert_eq!(rolling_fee_clone, RollingFeeParityStatus::Active);
    assert_eq!(format!("{rolling_fee:?}"), "Active");
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
    let delta = mempool
        .remove_for_connected_block_transition(
            &block,
            BlockLifecycleContext::new(PolicyTime::new(70), 3),
        )
        .expect("block cleanup");

    // Assert
    assert_eq!(delta.removed.len(), 1);
    assert_eq!(delta.removed[0].member.txid, parent_txid);
    assert_eq!(
        delta.removed[0].cause,
        MempoolRemovalCause::BlockConfirmation
    );
    assert_eq!(delta.removed[0].cause.as_str(), "block_confirmation");
    assert_eq!(delta.removed[0].role, MempoolRemovalRole::Direct);
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
    let mut block = build_block(BlockHash::from_byte_array([0_u8; 32]), 3, 499_999_000);
    block.transactions.push(replacement);

    // Act
    let delta = mempool
        .remove_for_connected_block_transition(
            &block,
            BlockLifecycleContext::new(PolicyTime::new(70), 3),
        )
        .expect("block cleanup");

    // Assert
    assert_eq!(delta.removed.len(), 2);
    assert!(delta.removed.iter().any(|removal| {
        removal.member.txid == original_txid
            && removal.cause == MempoolRemovalCause::BlockConflict
            && removal.role == MempoolRemovalRole::Direct
    }));
    assert!(delta.removed.iter().any(|removal| {
        removal.member.txid == descendant_txid
            && removal.cause == MempoolRemovalCause::BlockConflict
            && removal.role == MempoolRemovalRole::Descendant
    }));
    assert!(mempool.entry(&original_txid).is_none());
    assert!(mempool.entry(&descendant_txid).is_none());
    assert_eq!(mempool.total_virtual_size(), TransactionVirtualSize::ZERO);
    assert_eq!(
        mempool.pressure_summary().capacity_status,
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
    let mut block = build_block(BlockHash::from_byte_array([0_u8; 32]), 3, 499_999_000);
    block.transactions.push(unrelated);

    // Act
    let delta = mempool
        .remove_for_connected_block_transition(
            &block,
            BlockLifecycleContext::new(PolicyTime::new(70), 3),
        )
        .expect("block cleanup");

    // Assert
    assert!(delta.removed.is_empty());
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
    let mut mempool = Mempool::new(PolicyConfig::default());

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
fn record_removal_fact_keeps_cause_and_role_independent() {
    // Arrange
    let txid = Txid::from_byte_array([1_u8; 32]);
    let mut removals = BTreeMap::new();

    // Act
    record_removal_fact(
        &mut removals,
        txid,
        MempoolRemovalFact {
            cause: MempoolRemovalCause::Pressure,
            role: MempoolRemovalRole::Descendant,
        },
    );
    record_removal_fact(
        &mut removals,
        txid,
        MempoolRemovalFact {
            cause: MempoolRemovalCause::BlockConflict,
            role: MempoolRemovalRole::Direct,
        },
    );

    // Assert
    assert_eq!(
        removals.get(&txid),
        Some(&MempoolRemovalFact {
            cause: MempoolRemovalCause::BlockConflict,
            role: MempoolRemovalRole::Direct,
        })
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
        member: MempoolMemberIdentity { txid, wtxid },
        cause: MempoolRemovalCause::Pressure,
        role: MempoolRemovalRole::Direct,
    };
    let pressure = crate::MempoolPressureSummary {
        transaction_count: 1,
        total_virtual_size: TransactionVirtualSize::new(2),
        accounted_memory: AccountedMempoolMemory::new(3),
        mempool_capacity: MempoolCapacity::new(4),
        static_relay_fee_rate: crate::StaticRelayFeeRate::new(crate::FeeRate::from_sats_per_kvb(5)),
        incremental_relay_fee_rate: crate::IncrementalRelayFeeRate::new(
            crate::FeeRate::from_sats_per_kvb(6),
        ),
        rolling_mempool_fee_rate: crate::RollingMempoolFeeRate::new(
            crate::FeeRate::from_sats_per_kvb(7),
        ),
        effective_admission_fee_rate: crate::effective_admission_fee_rate(
            crate::StaticRelayFeeRate::new(crate::FeeRate::from_sats_per_kvb(5)),
            crate::RollingMempoolFeeRate::new(crate::FeeRate::from_sats_per_kvb(7)),
        ),
        capacity_status: MempoolCapacityStatus::OverCapacity,
        capacity_enforcement: crate::MempoolCapacityEnforcement::AccountedMemory,
        rolling_fee_parity: RollingFeeParityStatus::Active,
    };
    let summary = crate::MempoolLifecycleSummary {
        removed: vec![removal.clone()],
        pressure: pressure.clone(),
    };

    // Act
    let debug_text = format!("{:?}{:?}{:?}", removal, pressure, summary);
    let cloned_summary = summary.clone();
    // Assert
    assert!(debug_text.contains("MempoolLifecycleSummary"));
    assert_eq!(cloned_summary, summary);
}
