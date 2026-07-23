// Parity breadcrumbs:
// - packages/bitcoin-knots/src/txmempool.h
// - packages/bitcoin-knots/src/txmempool.cpp
// - packages/bitcoin-knots/src/rpc/mempool.cpp
// - packages/bitcoin-knots/test/functional/mempool_limit.py

use std::collections::{BTreeSet, HashMap};
use std::error::Error as _;

use open_bitcoin_consensus::{transaction_txid, transaction_wtxid};
use open_bitcoin_primitives::{
    Amount, OutPoint, ScriptBuf, ScriptWitness, Transaction, TransactionInput, TransactionOutput,
    Txid,
};

use crate::resource::{checked_product, checked_sum};
use crate::{
    AccountedMempoolMemory, MEMPOOL_RESOURCE_ACCOUNTING_VERSION, Mempool, MempoolCapacity,
    MempoolEntry, MempoolError, MempoolResourceLedger, PolicyConfig, ResourceAccountingError,
    TransactionVirtualSize, accounted_memory_for_entry, build_resource_ledger,
    recompute_resource_ledger,
};

use super::{sample_chainstate_snapshot, spend_transaction, submit};

fn sample_transaction(witness: ScriptWitness) -> Transaction {
    Transaction {
        version: 2,
        inputs: vec![TransactionInput {
            previous_output: OutPoint {
                txid: Txid::from_byte_array([1_u8; 32]),
                vout: 0,
            },
            script_sig: ScriptBuf::from_bytes(vec![0x01, 0x51]).expect("valid script"),
            sequence: TransactionInput::SEQUENCE_FINAL,
            witness,
        }],
        outputs: vec![TransactionOutput {
            value: Amount::from_sats(1_000).expect("valid amount"),
            script_pubkey: ScriptBuf::from_bytes(vec![0x51]).expect("valid script"),
        }],
        lock_time: 0,
    }
}

fn sample_entry(witness: ScriptWitness, virtual_size: usize) -> MempoolEntry {
    let transaction = sample_transaction(witness);
    let txid = transaction_txid(&transaction).expect("txid");
    let wtxid = transaction_wtxid(&transaction).expect("wtxid");
    MempoolEntry::new(
        transaction,
        txid,
        wtxid,
        Amount::from_sats(1_000).expect("valid amount"),
        TransactionVirtualSize::new(virtual_size),
        400,
        1,
        crate::MempoolEntryMetadata::legacy_unknown(),
    )
}

#[test]
fn empty_state_has_zero_resource_totals() {
    // Arrange
    let entries = HashMap::new();
    let spent_outpoints = HashMap::new();

    // Act
    let ledger =
        recompute_resource_ledger(&entries, &spent_outpoints).expect("empty state accounts");

    // Assert
    assert_eq!(ledger.total_virtual_size(), TransactionVirtualSize::ZERO);
    assert_eq!(ledger.accounted_memory(), AccountedMempoolMemory::ZERO);
}

#[test]
fn witness_and_script_payloads_increase_accounted_memory() {
    // Arrange
    let entry = sample_entry(
        ScriptWitness::new(vec![vec![0xaa; 64], vec![0xbb; 32]]),
        100,
    );

    // Act
    let accounted = accounted_memory_for_entry(&entry).expect("entry accounts");

    // Assert
    assert!(accounted.as_usize() > entry.virtual_size.as_usize());
}

#[test]
fn parent_and_child_relations_contribute_identity_storage() {
    // Arrange
    let entry = sample_entry(ScriptWitness::default(), 100);
    let without_relations = accounted_memory_for_entry(&entry).expect("entry accounts");
    let mut related_entry = entry;
    related_entry.parents = BTreeSet::from([Txid::from_byte_array([2_u8; 32])]);
    related_entry.children = BTreeSet::from([Txid::from_byte_array([3_u8; 32])]);

    // Act
    let with_relations = accounted_memory_for_entry(&related_entry).expect("relations account");

    // Assert
    assert!(with_relations > without_relations);
}

#[test]
fn spent_outpoint_index_contributes_key_and_value_storage() {
    // Arrange
    let entries = HashMap::new();
    let empty_spent_outpoints = HashMap::new();
    let spent_outpoints = HashMap::from([(
        OutPoint {
            txid: Txid::from_byte_array([4_u8; 32]),
            vout: 1,
        },
        Txid::from_byte_array([5_u8; 32]),
    )]);

    // Act
    let empty =
        recompute_resource_ledger(&entries, &empty_spent_outpoints).expect("empty index accounts");
    let populated =
        recompute_resource_ledger(&entries, &spent_outpoints).expect("spent index accounts");

    // Assert
    assert!(populated.accounted_memory() > empty.accounted_memory());
}

#[test]
fn checked_resource_addition_reports_the_overflowing_component() {
    // Arrange
    let maximum = AccountedMempoolMemory::new(usize::MAX);
    let one = AccountedMempoolMemory::new(1);

    // Act
    let error = maximum
        .checked_add(one, "test component")
        .expect_err("overflow must fail closed");

    // Assert
    assert_eq!(
        error,
        ResourceAccountingError::Overflow {
            component: "test component"
        }
    );
}

#[test]
fn distinct_resource_values_expose_only_explicit_checked_operations() {
    // Arrange
    let virtual_size = TransactionVirtualSize::new(10);
    let accounted_memory = AccountedMempoolMemory::new(20);
    let capacity = MempoolCapacity::new(30);

    // Act
    let virtual_size_sum = virtual_size
        .checked_add(TransactionVirtualSize::new(5), "vsize sum")
        .expect("vsize sum");
    let accounted_sum = accounted_memory
        .checked_add(AccountedMempoolMemory::new(5), "accounted sum")
        .expect("accounted sum");
    let capacity_sum = capacity
        .checked_add(MempoolCapacity::new(5), "capacity sum")
        .expect("capacity sum");
    let virtual_size_error = TransactionVirtualSize::new(usize::MAX)
        .checked_add(TransactionVirtualSize::new(1), "vsize overflow")
        .expect_err("vsize overflow");
    let capacity_error = MempoolCapacity::new(usize::MAX)
        .checked_add(MempoolCapacity::new(1), "capacity overflow")
        .expect_err("capacity overflow");

    // Assert
    assert_eq!(MEMPOOL_RESOURCE_ACCOUNTING_VERSION, 1);
    assert_eq!(virtual_size_sum.as_usize(), 15);
    assert_eq!(accounted_sum.as_usize(), 25);
    assert_eq!(capacity_sum.as_usize(), 35);
    assert_eq!(
        TransactionVirtualSize::default(),
        TransactionVirtualSize::ZERO
    );
    assert_eq!(
        AccountedMempoolMemory::default(),
        AccountedMempoolMemory::ZERO
    );
    assert_eq!(MempoolCapacity::ZERO.as_usize(), 0);
    assert_eq!(MempoolCapacity::default(), MempoolCapacity::ZERO);
    assert!(virtual_size > TransactionVirtualSize::ZERO);
    assert!(accounted_memory > AccountedMempoolMemory::ZERO);
    assert!(capacity > MempoolCapacity::ZERO);
    assert_eq!(format!("{virtual_size:?}"), "TransactionVirtualSize(10)");
    assert_eq!(
        format!("{accounted_memory:?}"),
        "AccountedMempoolMemory(20)"
    );
    assert_eq!(format!("{capacity:?}"), "MempoolCapacity(30)");
    assert_eq!(
        virtual_size_error.to_string(),
        "mempool resource accounting overflow: vsize overflow"
    );
    assert!(virtual_size_error.source().is_none());
    assert!(format!("{virtual_size_error:?}").contains("vsize overflow"));
    assert!(matches!(
        capacity_error,
        ResourceAccountingError::Overflow {
            component: "capacity overflow"
        }
    ));
}

#[test]
fn cached_build_path_matches_independent_recomputation() {
    // Arrange
    let entry = sample_entry(ScriptWitness::new(vec![vec![0xcc; 8]]), 100);
    let txid = entry.txid;
    let spent_outpoints =
        HashMap::from([(entry.transaction.inputs[0].previous_output.clone(), txid)]);
    let entries = HashMap::from([(txid, entry)]);

    // Act
    let cached = build_resource_ledger(&entries, &spent_outpoints).expect("cached ledger");
    let oracle = recompute_resource_ledger(&entries, &spent_outpoints).expect("oracle");

    // Assert
    assert_eq!(cached, oracle);
    assert_eq!(
        MempoolResourceLedger::default(),
        MempoolResourceLedger::ZERO
    );
    assert!(format!("{cached:?}").contains("MempoolResourceLedger"));
    assert_eq!(cached.total_virtual_size().as_usize(), 100);
    assert!(cached.accounted_memory().as_usize() > 100);
}

#[test]
fn aggregate_addition_overflow_fails_closed() {
    // Arrange
    let total = usize::MAX;

    // Act
    let error = checked_sum(total, 1, "aggregate").expect_err("overflow must fail closed");

    // Assert
    assert_eq!(
        error,
        ResourceAccountingError::Overflow {
            component: "aggregate"
        }
    );
}

#[test]
fn component_product_overflow_fails_closed() {
    // Arrange
    let count = usize::MAX;

    // Act
    let error =
        checked_product(count, 2, "component product").expect_err("overflow must fail closed");

    // Assert
    assert_eq!(
        error,
        ResourceAccountingError::Overflow {
            component: "component product"
        }
    );
}

#[test]
fn ledger_entry_overflow_fails_closed() {
    // Arrange
    let mut ledger = MempoolResourceLedger::new(
        TransactionVirtualSize::new(usize::MAX),
        AccountedMempoolMemory::ZERO,
    );
    let entry = sample_entry(ScriptWitness::default(), 1);

    // Act
    let error = ledger
        .checked_add_entry(&entry)
        .expect_err("ledger vsize overflow must fail closed");

    // Assert
    assert!(matches!(
        error,
        ResourceAccountingError::Overflow {
            component: "total transaction virtual size"
        }
    ));
}

#[test]
fn spent_index_count_overflow_fails_closed() {
    // Arrange
    let mut ledger = MempoolResourceLedger::ZERO;

    // Act
    let error = ledger
        .checked_add_spent_outpoints(usize::MAX)
        .expect_err("spent index overflow must fail closed");

    // Assert
    assert!(matches!(
        error,
        ResourceAccountingError::Overflow {
            component: "total spent-outpoint accounted memory"
        }
    ));
}

#[test]
fn cached_resource_ledger_matches_recomputation_oracle() {
    // Arrange
    let (snapshot, coinbase_txids) = sample_chainstate_snapshot(3);
    let original = spend_transaction(
        coinbase_txids[0],
        0,
        499_999_000,
        TransactionInput::MAX_SEQUENCE_NONFINAL - 1,
    );
    let original_txid = transaction_txid(&original).expect("original txid");
    let child = spend_transaction(
        original_txid,
        0,
        499_998_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    let replacement = spend_transaction(
        coinbase_txids[0],
        0,
        499_997_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    let confirmed = spend_transaction(
        coinbase_txids[1],
        0,
        499_999_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    let mut mempool = Mempool::default();

    // Act
    submit(&mut mempool, &snapshot, original).expect("original");
    assert_ledger_matches_oracle(&mempool);
    submit(&mut mempool, &snapshot, child).expect("child");
    assert_ledger_matches_oracle(&mempool);
    submit(&mut mempool, &snapshot, replacement).expect("replacement");
    assert_ledger_matches_oracle(&mempool);
    submit(&mut mempool, &snapshot, confirmed.clone()).expect("confirmed candidate");
    mempool
        .remove_for_connected_transactions_transition([&confirmed])
        .expect("block removal");

    // Assert
    assert_ledger_matches_oracle(&mempool);
}

#[test]
fn legacy_vsize_trim_limit_is_independent_from_accounted_capacity() {
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
    let config = PolicyConfig {
        mempool_capacity: MempoolCapacity::new(1),
        legacy_vsize_trim_limit: TransactionVirtualSize::new(140),
        ..PolicyConfig::default()
    };
    let mut mempool = Mempool::new(config);

    // Act
    let low_fee_result = submit(&mut mempool, &snapshot, low_fee).expect("low fee");
    let high_fee_result = submit(&mut mempool, &snapshot, high_fee).expect("high fee");

    // Assert
    assert_eq!(high_fee_result.evicted, vec![low_fee_result.accepted]);
    assert_ledger_matches_oracle(&mempool);
    assert_eq!(
        PolicyConfig::default().mempool_capacity,
        MempoolCapacity::new(300_000_000)
    );
    assert_eq!(
        PolicyConfig::default().legacy_vsize_trim_limit,
        TransactionVirtualSize::new(300_000_000)
    );
}

fn assert_ledger_matches_oracle(mempool: &Mempool) {
    let oracle = recompute_resource_ledger(mempool.entries(), &mempool.spent_outpoints)
        .expect("oracle recomputation");
    assert_eq!(mempool.resource_ledger(), oracle);
}

#[test]
fn accounting_failure_maps_to_internal_invariant() {
    // Arrange
    let source = ResourceAccountingError::Overflow {
        component: "test mapping",
    };

    // Act
    let error = super::super::resource_invariant_error(source);

    // Assert
    assert!(matches!(error, MempoolError::InternalInvariant { .. }));
    assert!(error.to_string().contains("test mapping"));
}
