// Parity breadcrumbs:
// - packages/bitcoin-knots/src/txmempool.h
// - packages/bitcoin-knots/src/txmempool.cpp
// - packages/bitcoin-knots/src/policy/policy.h
// - packages/bitcoin-knots/src/policy/rbf.cpp
// - packages/bitcoin-knots/src/policy/packages.cpp

use open_bitcoin_chainstate::{Chainstate, ChainstateSnapshot};
use open_bitcoin_consensus::crypto::hash160;
use open_bitcoin_consensus::{
    ConsensusParams, ScriptVerifyFlags, block_merkle_root, check_block_header, transaction_txid,
};
use open_bitcoin_mempool::{
    AccountedMempoolMemory, AdmissionContext, BlockLifecycleContext, FeeRate,
    IncrementalRelayFeeRate, LimitDirection, LimitKind, Mempool, MempoolCapacity,
    MempoolCapacityStatus, MempoolError, MempoolLifecycleRemoval, MempoolLifecycleSummary,
    MempoolMemberIdentity, MempoolPressureSummary, MempoolRemovalCause, MempoolRemovalRole,
    PolicyConfig, PolicyTime, RbfPolicy, RollingFeeParityStatus, RollingMempoolFeeRate,
    StaticRelayFeeRate, TransactionVirtualSize, effective_admission_fee_rate,
};
use open_bitcoin_primitives::{
    Amount, Block, BlockHash, BlockHeader, OutPoint, ScriptBuf, ScriptWitness, Transaction,
    TransactionInput, TransactionOutput, Txid, Wtxid,
};

const EASY_BITS: u32 = 0x207f_ffff;

fn script(bytes: &[u8]) -> ScriptBuf {
    ScriptBuf::from_bytes(bytes.to_vec()).expect("valid script")
}

fn redeem_script() -> ScriptBuf {
    script(&[0x51])
}

fn p2sh_script() -> ScriptBuf {
    let redeem_hash = hash160(redeem_script().as_bytes());
    let mut bytes = vec![0xa9, 20];
    bytes.extend_from_slice(&redeem_hash);
    bytes.push(0x87);
    script(&bytes)
}

fn serialized_script_num(value: i64) -> Vec<u8> {
    if value == 0 {
        return vec![0x00];
    }

    let mut magnitude = value as u64;
    let mut encoded = Vec::new();
    while magnitude > 0 {
        encoded.push((magnitude & 0xff) as u8);
        magnitude >>= 8;
    }

    let mut script = Vec::with_capacity(encoded.len() + 2);
    script.push(encoded.len() as u8);
    script.extend(encoded);
    script.push(0x51);
    script
}

fn coinbase_transaction(height: u32, value: i64) -> Transaction {
    let mut script_sig = serialized_script_num(i64::from(height));
    script_sig.push(0x51);
    Transaction {
        version: 1,
        inputs: vec![TransactionInput {
            previous_output: OutPoint::null(),
            script_sig: script(&script_sig),
            sequence: TransactionInput::SEQUENCE_FINAL,
            witness: ScriptWitness::default(),
        }],
        outputs: vec![TransactionOutput {
            value: Amount::from_sats(value).expect("valid amount"),
            script_pubkey: p2sh_script(),
        }],
        lock_time: 0,
    }
}

fn spend_transaction(previous_txid: Txid, output_value: i64, sequence: u32) -> Transaction {
    Transaction {
        version: 2,
        inputs: vec![TransactionInput {
            previous_output: OutPoint {
                txid: previous_txid,
                vout: 0,
            },
            script_sig: script(&[0x01, 0x51]),
            sequence,
            witness: ScriptWitness::default(),
        }],
        outputs: vec![TransactionOutput {
            value: Amount::from_sats(output_value).expect("valid amount"),
            script_pubkey: p2sh_script(),
        }],
        lock_time: 0,
    }
}

fn non_standard_spend(previous_txid: Txid) -> Transaction {
    let mut transaction =
        spend_transaction(previous_txid, 499_000_000, TransactionInput::SEQUENCE_FINAL);
    transaction.outputs[0].script_pubkey = script(&[0x51]);
    transaction
}

fn build_block(previous_block_hash: BlockHash, height: u32, value: i64) -> Block {
    let transactions = vec![coinbase_transaction(height, value)];
    let (merkle_root, maybe_mutated) = block_merkle_root(&transactions).expect("merkle root");
    assert!(!maybe_mutated);

    let mut block = Block {
        header: BlockHeader {
            version: 1,
            previous_block_hash,
            merkle_root,
            time: 1_231_006_500 + height,
            bits: EASY_BITS,
            nonce: 0,
        },
        transactions,
    };
    mine_header(&mut block);
    block
}

fn mine_header(block: &mut Block) {
    block.header.nonce = (0..=u32::MAX)
        .find(|nonce| {
            block.header.nonce = *nonce;
            check_block_header(&block.header).is_ok()
        })
        .expect("expected nonce at easy target");
}

fn sample_chainstate_snapshot(block_count: u32) -> (ChainstateSnapshot, Vec<Txid>) {
    let mut chainstate = Chainstate::new();
    let mut previous_hash = BlockHash::from_byte_array([0_u8; 32]);
    let mut txids = Vec::new();

    for height in 0..block_count {
        let block = build_block(previous_hash, height, 500_000_000);
        let txid = transaction_txid(&block.transactions[0]).expect("txid");
        txids.push(txid);
        chainstate
            .connect_block(
                &block,
                u128::from(height + 1),
                ScriptVerifyFlags::P2SH
                    | ScriptVerifyFlags::CHECKLOCKTIMEVERIFY
                    | ScriptVerifyFlags::CHECKSEQUENCEVERIFY,
                ConsensusParams {
                    coinbase_maturity: 1,
                    ..ConsensusParams::default()
                },
            )
            .expect("block should connect");
        previous_hash = open_bitcoin_consensus::block_hash(&block.header);
    }

    (chainstate.snapshot(), txids)
}

fn submit(
    mempool: &mut Mempool,
    snapshot: &ChainstateSnapshot,
    transaction: Transaction,
) -> Result<open_bitcoin_mempool::AdmissionResult, MempoolError> {
    mempool.accept_transaction_with_context(
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
}

#[test]
fn standard_admission_tracks_public_entry_metrics() {
    let (snapshot, coinbase_txids) = sample_chainstate_snapshot(2);
    let mut mempool = Mempool::default();

    let result = submit(
        &mut mempool,
        &snapshot,
        spend_transaction(
            coinbase_txids[0],
            499_999_000,
            TransactionInput::SEQUENCE_FINAL,
        ),
    )
    .expect("standard tx should be admitted");
    let entry = mempool.entry(&result.accepted).expect("entry");

    assert!(result.replaced.is_empty());
    assert!(result.evicted.is_empty());
    assert_eq!(entry.ancestor_stats.count, 1);
    assert_eq!(entry.descendant_stats.count, 1);
}

#[test]
fn non_standard_outputs_fail_public_api_admission() {
    let (snapshot, coinbase_txids) = sample_chainstate_snapshot(2);
    let error = submit(
        &mut Mempool::default(),
        &snapshot,
        non_standard_spend(coinbase_txids[0]),
    )
    .expect_err("non-standard outputs should fail");

    assert!(matches!(error, MempoolError::NonStandard { .. }));
}

#[test]
fn replacement_requires_fee_bump_and_reports_replaced_txids() {
    let (snapshot, coinbase_txids) = sample_chainstate_snapshot(3);
    let original = spend_transaction(
        coinbase_txids[0],
        499_999_000,
        TransactionInput::MAX_SEQUENCE_NONFINAL - 1,
    );
    let lower_fee_replacement = spend_transaction(
        coinbase_txids[0],
        499_999_100,
        TransactionInput::SEQUENCE_FINAL,
    );
    let higher_fee_replacement = spend_transaction(
        coinbase_txids[0],
        499_998_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    let mut mempool = Mempool::new(PolicyConfig {
        rbf_policy: RbfPolicy::OptIn,
        ..PolicyConfig::default()
    });

    let original_result = submit(&mut mempool, &snapshot, original).expect("original");
    let lower_error = submit(&mut mempool, &snapshot, lower_fee_replacement)
        .expect_err("replacement should fail");
    let higher_result =
        submit(&mut mempool, &snapshot, higher_fee_replacement).expect("replacement");

    assert!(matches!(
        lower_error,
        MempoolError::ReplacementRejected { .. }
    ));
    assert_eq!(higher_result.replaced, vec![original_result.accepted]);
}

#[test]
fn ancestor_limit_and_eviction_truths_hold_through_public_api() {
    let (snapshot, coinbase_txids) = sample_chainstate_snapshot(3);
    let parent = spend_transaction(
        coinbase_txids[0],
        499_999_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    let parent_txid = transaction_txid(&parent).expect("txid");
    let child = spend_transaction(parent_txid, 499_998_000, TransactionInput::SEQUENCE_FINAL);
    let mut strict_mempool = Mempool::new(PolicyConfig {
        max_ancestor_count: 1,
        ..PolicyConfig::default()
    });

    submit(&mut strict_mempool, &snapshot, parent.clone()).expect("parent");
    let ancestor_error =
        submit(&mut strict_mempool, &snapshot, child).expect_err("ancestor limit should fail");
    assert!(matches!(
        ancestor_error,
        MempoolError::LimitExceeded {
            direction: LimitDirection::Ancestor,
            kind: LimitKind::Count,
            ..
        }
    ));

    let mut trim_mempool = Mempool::new(PolicyConfig {
        legacy_vsize_trim_limit: TransactionVirtualSize::new(140),
        ..PolicyConfig::default()
    });
    let low_fee_result = submit(
        &mut trim_mempool,
        &snapshot,
        spend_transaction(
            coinbase_txids[0],
            499_999_200,
            TransactionInput::SEQUENCE_FINAL,
        ),
    )
    .expect("low fee");
    let high_fee_result = submit(
        &mut trim_mempool,
        &snapshot,
        spend_transaction(
            coinbase_txids[1],
            499_998_000,
            TransactionInput::SEQUENCE_FINAL,
        ),
    )
    .expect("high fee");

    assert_eq!(high_fee_result.evicted, vec![low_fee_result.accepted]);
}

#[test]
fn lifecycle_cleanup_and_pressure_truths_hold_through_public_api() {
    let (snapshot, coinbase_txids) = sample_chainstate_snapshot(3);
    let parent = spend_transaction(
        coinbase_txids[0],
        499_999_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    let parent_txid = transaction_txid(&parent).expect("parent txid");
    let child = spend_transaction(parent_txid, 499_998_000, TransactionInput::SEQUENCE_FINAL);
    let child_txid = transaction_txid(&child).expect("child txid");
    let replacement = spend_transaction(
        coinbase_txids[0],
        499_997_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    let mut mempool = Mempool::default();

    submit(&mut mempool, &snapshot, parent.clone()).expect("parent");
    submit(&mut mempool, &snapshot, child).expect("child");
    let initial_pressure = mempool.pressure_summary();
    let empty_block = build_block(BlockHash::from_byte_array([0_u8; 32]), 3, 499_999_000);
    let empty_cleanup = mempool
        .remove_for_connected_block_transition(
            &empty_block,
            BlockLifecycleContext::new(PolicyTime::new(70), 3),
        )
        .expect("empty cleanup");
    let mut conflict_block = build_block(BlockHash::from_byte_array([0_u8; 32]), 3, 499_999_000);
    conflict_block.transactions.push(replacement);
    let conflict_cleanup = mempool
        .remove_for_connected_block_transition(
            &conflict_block,
            BlockLifecycleContext::new(PolicyTime::new(70), 3),
        )
        .expect("conflict cleanup");

    assert_eq!(
        initial_pressure.capacity_status,
        MempoolCapacityStatus::UnderCapacity
    );
    assert_eq!(initial_pressure.capacity_status.as_str(), "under_capacity");
    assert_eq!(
        initial_pressure.rolling_fee_parity,
        RollingFeeParityStatus::Deferred
    );
    assert_eq!(initial_pressure.rolling_fee_parity.as_str(), "deferred");
    assert!(empty_cleanup.removed.is_empty());
    assert!(conflict_cleanup.removed.iter().any(|removal| {
        removal.member.txid == parent_txid
            && removal.cause == MempoolRemovalCause::BlockConflict
            && removal.role == MempoolRemovalRole::Direct
    }));
    assert!(conflict_cleanup.removed.iter().any(|removal| {
        removal.member.txid == child_txid
            && removal.cause == MempoolRemovalCause::BlockConflict
            && removal.role == MempoolRemovalRole::Descendant
    }));
    assert_eq!(
        MempoolRemovalCause::BlockConfirmation.as_str(),
        "block_confirmation"
    );
    assert_eq!(
        MempoolRemovalCause::BlockConflict.as_str(),
        "block_conflict"
    );
    assert_eq!(MempoolRemovalRole::Descendant.as_str(), "descendant");
    assert_eq!(MempoolRemovalCause::Pressure.as_str(), "pressure");

    let removal = MempoolLifecycleRemoval {
        member: MempoolMemberIdentity {
            txid: Txid::from_byte_array([4_u8; 32]),
            wtxid: Wtxid::from_byte_array([5_u8; 32]),
        },
        cause: MempoolRemovalCause::Pressure,
        role: MempoolRemovalRole::Direct,
    };
    let pressure = MempoolPressureSummary {
        transaction_count: 1,
        total_virtual_size: TransactionVirtualSize::new(2),
        accounted_memory: AccountedMempoolMemory::new(3),
        mempool_capacity: MempoolCapacity::new(1),
        static_relay_fee_rate: StaticRelayFeeRate::new(FeeRate::from_sats_per_kvb(1_000)),
        incremental_relay_fee_rate: IncrementalRelayFeeRate::new(FeeRate::from_sats_per_kvb(1_000)),
        rolling_mempool_fee_rate: RollingMempoolFeeRate::ZERO,
        effective_admission_fee_rate: effective_admission_fee_rate(
            StaticRelayFeeRate::new(FeeRate::from_sats_per_kvb(1_000)),
            RollingMempoolFeeRate::ZERO,
        ),
        capacity_status: MempoolCapacityStatus::OverCapacity,
        rolling_fee_parity: RollingFeeParityStatus::Deferred,
    };
    let summary = MempoolLifecycleSummary {
        removed: vec![removal.clone()],
        pressure: pressure.clone(),
    };

    assert_eq!(MempoolCapacityStatus::Empty.as_str(), "empty");
    assert_eq!(MempoolCapacityStatus::AtCapacity.as_str(), "at_capacity");
    assert_eq!(
        MempoolCapacityStatus::OverCapacity.as_str(),
        "over_capacity"
    );
    assert!(format!("{removal:?}{pressure:?}{summary:?}").contains("Pressure"));
    assert_eq!(summary.clone(), summary);
}
