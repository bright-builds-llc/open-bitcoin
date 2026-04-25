// Parity breadcrumbs:
// - packages/bitcoin-knots/src/txmempool.h
// - packages/bitcoin-knots/src/txmempool.cpp
// - packages/bitcoin-knots/src/policy/policy.h
// - packages/bitcoin-knots/src/policy/rbf.cpp
// - packages/bitcoin-knots/src/policy/packages.cpp

use open_bitcoin_chainstate::{Chainstate, ChainstateSnapshot};
use std::collections::{BTreeSet, HashMap};

use open_bitcoin_consensus::crypto::hash160;
use open_bitcoin_consensus::{
    ConsensusParams, ScriptVerifyFlags, block_merkle_root, check_block_header,
};
use open_bitcoin_primitives::{
    Amount, Block, BlockHash, BlockHeader, OutPoint, ScriptBuf, ScriptWitness, Transaction,
    TransactionInput, TransactionOutput, Txid,
};

use super::Mempool;
use crate::{LimitDirection, LimitKind, MempoolEntry, MempoolError, PolicyConfig, RbfPolicy};

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

fn spend_transaction(
    previous_txid: Txid,
    vout: u32,
    output_value: i64,
    sequence: u32,
) -> Transaction {
    Transaction {
        version: 2,
        inputs: vec![TransactionInput {
            previous_output: OutPoint {
                txid: previous_txid,
                vout,
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
    let mut transaction = spend_transaction(
        previous_txid,
        0,
        499_000_000,
        TransactionInput::SEQUENCE_FINAL,
    );
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
        let txid = open_bitcoin_consensus::transaction_txid(&block.transactions[0]).expect("txid");
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
) -> Result<crate::AdmissionResult, MempoolError> {
    mempool.accept_transaction(
        transaction,
        snapshot,
        ScriptVerifyFlags::P2SH
            | ScriptVerifyFlags::CHECKLOCKTIMEVERIFY
            | ScriptVerifyFlags::CHECKSEQUENCEVERIFY,
        ConsensusParams {
            coinbase_maturity: 1,
            ..ConsensusParams::default()
        },
    )
}

#[test]
fn accepts_standard_confirmed_spend() {
    let (snapshot, coinbase_txids) = sample_chainstate_snapshot(2);
    let transaction = spend_transaction(
        coinbase_txids[0],
        0,
        499_999_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    let mut mempool = Mempool::default();

    let result = submit(&mut mempool, &snapshot, transaction).expect("admission");
    let entry = mempool.entry(&result.accepted).expect("entry");

    assert!(result.replaced.is_empty());
    assert!(result.evicted.is_empty());
    assert_eq!(entry.ancestor_stats.count, 1);
}

#[test]
fn getters_expose_config_entries_and_total_virtual_size() {
    let (snapshot, coinbase_txids) = sample_chainstate_snapshot(2);
    let mut mempool = Mempool::default();
    let result = submit(
        &mut mempool,
        &snapshot,
        spend_transaction(
            coinbase_txids[0],
            0,
            499_999_000,
            TransactionInput::SEQUENCE_FINAL,
        ),
    )
    .expect("admission");

    assert_eq!(mempool.config().rbf_policy, RbfPolicy::Always);
    assert_eq!(mempool.entries().len(), 1);
    assert_eq!(
        mempool.total_virtual_size(),
        mempool.entry(&result.accepted).expect("entry").virtual_size
    );
}

#[test]
fn duplicate_transactions_and_missing_inputs_are_rejected() {
    let (snapshot, coinbase_txids) = sample_chainstate_snapshot(2);
    let transaction = spend_transaction(
        coinbase_txids[0],
        0,
        499_999_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    let mut mempool = Mempool::default();
    submit(&mut mempool, &snapshot, transaction.clone()).expect("first admission");

    let duplicate = submit(&mut mempool, &snapshot, transaction).expect_err("duplicate");
    assert!(matches!(
        duplicate,
        MempoolError::DuplicateTransaction { .. }
    ));

    let missing = submit(
        &mut Mempool::default(),
        &snapshot,
        spend_transaction(
            Txid::from_byte_array([8_u8; 32]),
            0,
            499_999_000,
            TransactionInput::SEQUENCE_FINAL,
        ),
    )
    .expect_err("missing input");
    assert!(matches!(missing, MempoolError::MissingInput { .. }));
}

#[test]
fn rejects_non_standard_output_scripts() {
    let (snapshot, coinbase_txids) = sample_chainstate_snapshot(2);
    let error = submit(
        &mut Mempool::default(),
        &snapshot,
        non_standard_spend(coinbase_txids[0]),
    )
    .expect_err("non-standard output should fail");

    assert!(matches!(error, MempoolError::NonStandard { .. }));
}

#[test]
fn rejects_entries_that_exceed_ancestor_limits() {
    let (snapshot, coinbase_txids) = sample_chainstate_snapshot(2);
    let parent = spend_transaction(
        coinbase_txids[0],
        0,
        499_999_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    let parent_txid = open_bitcoin_consensus::transaction_txid(&parent).expect("txid");
    let child = spend_transaction(
        parent_txid,
        0,
        499_998_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    let mut mempool = Mempool::new(PolicyConfig {
        max_ancestor_count: 1,
        ..PolicyConfig::default()
    });

    submit(&mut mempool, &snapshot, parent).expect("parent");
    let error = submit(&mut mempool, &snapshot, child).expect_err("limit should fail");

    assert!(matches!(
        error,
        MempoolError::LimitExceeded {
            direction: LimitDirection::Ancestor,
            kind: LimitKind::Count,
            ..
        }
    ));
}

#[test]
fn tracks_parent_child_and_ancestor_descendant_metrics() {
    let (snapshot, coinbase_txids) = sample_chainstate_snapshot(2);
    let parent = spend_transaction(
        coinbase_txids[0],
        0,
        499_999_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    let parent_txid = open_bitcoin_consensus::transaction_txid(&parent).expect("txid");
    let child = spend_transaction(
        parent_txid,
        0,
        499_998_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    let mut mempool = Mempool::default();

    submit(&mut mempool, &snapshot, parent).expect("parent");
    let child_result = submit(&mut mempool, &snapshot, child).expect("child");
    let child_entry = mempool.entry(&child_result.accepted).expect("child entry");
    let parent_entry = mempool.entry(&parent_txid).expect("parent entry");

    assert_eq!(child_entry.parents, BTreeSet::from([parent_txid]));
    assert_eq!(child_entry.ancestor_stats.count, 2);
    assert_eq!(
        parent_entry.children,
        BTreeSet::from([child_result.accepted])
    );
    assert_eq!(parent_entry.descendant_stats.count, 2);
}

#[test]
fn replacement_requires_a_fee_bump() {
    let (snapshot, coinbase_txids) = sample_chainstate_snapshot(3);
    let original = spend_transaction(
        coinbase_txids[0],
        0,
        499_999_000,
        TransactionInput::MAX_SEQUENCE_NONFINAL - 1,
    );
    let lower_fee_replacement = spend_transaction(
        coinbase_txids[0],
        0,
        499_999_100,
        TransactionInput::SEQUENCE_FINAL,
    );
    let higher_fee_replacement = spend_transaction(
        coinbase_txids[0],
        0,
        499_998_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    let mut mempool = Mempool::new(PolicyConfig {
        rbf_policy: RbfPolicy::OptIn,
        ..PolicyConfig::default()
    });

    let original_result = submit(&mut mempool, &snapshot, original).expect("original");
    let lower_error = submit(&mut mempool, &snapshot, lower_fee_replacement)
        .expect_err("lower fee replacement should fail");
    let higher_result =
        submit(&mut mempool, &snapshot, higher_fee_replacement).expect("replacement");

    assert!(matches!(
        lower_error,
        MempoolError::ReplacementRejected { .. }
    ));
    assert_eq!(higher_result.replaced, vec![original_result.accepted]);
}

#[test]
fn replacement_requires_opt_in_signal_when_policy_demands_it() {
    let (snapshot, coinbase_txids) = sample_chainstate_snapshot(2);
    let original = spend_transaction(
        coinbase_txids[0],
        0,
        499_999_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    let replacement = spend_transaction(
        coinbase_txids[0],
        0,
        499_998_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    let mut mempool = Mempool::new(PolicyConfig {
        rbf_policy: RbfPolicy::OptIn,
        ..PolicyConfig::default()
    });

    submit(&mut mempool, &snapshot, original).expect("original");
    let error = submit(&mut mempool, &snapshot, replacement).expect_err("opt-in required");

    assert!(matches!(error, MempoolError::ConflictNotAllowed { .. }));
}

#[test]
fn replacement_rejects_new_unconfirmed_inputs() {
    let (snapshot, coinbase_txids) = sample_chainstate_snapshot(3);
    let parent = spend_transaction(
        coinbase_txids[1],
        0,
        499_999_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    let parent_txid = open_bitcoin_consensus::transaction_txid(&parent).expect("txid");
    let original = spend_transaction(
        coinbase_txids[0],
        0,
        499_999_000,
        TransactionInput::MAX_SEQUENCE_NONFINAL - 1,
    );
    let original_txid = open_bitcoin_consensus::transaction_txid(&original).expect("txid");
    let mut replacement = spend_transaction(
        coinbase_txids[0],
        0,
        499_997_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    replacement.inputs.push(TransactionInput {
        previous_output: OutPoint {
            txid: parent_txid,
            vout: 0,
        },
        script_sig: script(&[0x01, 0x51]),
        sequence: TransactionInput::SEQUENCE_FINAL,
        witness: ScriptWitness::default(),
    });
    let mut mempool = Mempool::new(PolicyConfig {
        rbf_policy: RbfPolicy::Always,
        ..PolicyConfig::default()
    });

    submit(&mut mempool, &snapshot, parent).expect("parent");
    submit(&mut mempool, &snapshot, original).expect("original");
    assert!(mempool.entry(&original_txid).is_some());

    let error = submit(&mut mempool, &snapshot, replacement)
        .expect_err("replacement with new unconfirmed input should fail");

    assert!(matches!(error, MempoolError::ReplacementRejected { .. }));
}

#[test]
fn evicts_lowest_descendant_score_package_when_size_limit_is_exceeded() {
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
    let mut mempool = Mempool::new(PolicyConfig {
        max_mempool_virtual_size: 140,
        ..PolicyConfig::default()
    });

    let low_fee_result = submit(&mut mempool, &snapshot, low_fee).expect("low fee");
    let high_fee_result = submit(&mut mempool, &snapshot, high_fee).expect("high fee");

    assert_eq!(high_fee_result.evicted, vec![low_fee_result.accepted]);
    assert!(mempool.entry(&low_fee_result.accepted).is_none());
    assert!(mempool.entry(&high_fee_result.accepted).is_some());
}

#[test]
fn replacements_respect_disabled_policy() {
    let (snapshot, coinbase_txids) = sample_chainstate_snapshot(2);
    let original = spend_transaction(
        coinbase_txids[0],
        0,
        499_999_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    let replacement = spend_transaction(
        coinbase_txids[0],
        0,
        499_998_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    let mut mempool = Mempool::new(PolicyConfig {
        rbf_policy: RbfPolicy::Never,
        ..PolicyConfig::default()
    });

    submit(&mut mempool, &snapshot, original).expect("original");
    let error = submit(&mut mempool, &snapshot, replacement).expect_err("rbf disabled");

    assert!(matches!(error, MempoolError::ConflictNotAllowed { .. }));
}

#[test]
fn direct_helper_paths_cover_internal_edge_branches() {
    let empty_snapshot = ChainstateSnapshot::new(Vec::new(), HashMap::new(), HashMap::new());
    let context = super::build_validation_context(
        &empty_snapshot,
        Vec::new(),
        ScriptVerifyFlags::NONE,
        ConsensusParams::default(),
    );
    assert_eq!(context.spend_height, 0);
    assert_eq!(context.block_time, 0);

    let relay_error = super::enforce_min_relay_fee(&PolicyConfig::default(), 0, 100)
        .expect_err("fee floor should fail");
    assert!(matches!(relay_error, MempoolError::RelayFeeTooLow { .. }));

    let invalid_fee = super::amount_from_fee_sats(-1).expect_err("negative fee should fail");
    assert!(matches!(invalid_fee, MempoolError::Validation { .. }));
    let serialization_error = super::serialization_validation_error(
        "transaction txid",
        open_bitcoin_codec::CodecError::CompactSizeTooLarge(33_554_433),
    );
    assert!(matches!(
        serialization_error,
        MempoolError::Validation { .. }
    ));

    let candidate_txid = Txid::from_byte_array([7_u8; 32]);
    let candidate_transaction = spend_transaction(
        Txid::from_byte_array([6_u8; 32]),
        0,
        499_999_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    let candidate_entry = MempoolEntry::new(
        candidate_transaction.clone(),
        candidate_txid,
        open_bitcoin_consensus::transaction_wtxid(&candidate_transaction).expect("wtxid"),
        Amount::from_sats(100).expect("amount"),
        100,
        400,
        0,
    );
    let missing_candidate = super::validate_limits(
        &HashMap::from([(candidate_txid, candidate_entry)]),
        &PolicyConfig {
            max_ancestor_count: 0,
            ..PolicyConfig::default()
        },
        candidate_txid,
    );
    assert!(missing_candidate.is_err());

    assert!(super::select_eviction_candidate(&HashMap::new()).is_none());
    let missing_ancestors =
        super::collect_ancestors(&HashMap::new(), Txid::from_byte_array([1_u8; 32]));
    let missing_descendants =
        super::collect_descendants(&HashMap::new(), Txid::from_byte_array([1_u8; 32]));
    assert!(missing_ancestors.is_empty());
    assert!(missing_descendants.is_empty());
}

#[test]
fn admission_maps_validation_errors_and_replacement_policy_edges() {
    let (snapshot, coinbase_txids) = sample_chainstate_snapshot(3);
    let invalid = spend_transaction(
        coinbase_txids[0],
        0,
        500_000_001,
        TransactionInput::SEQUENCE_FINAL,
    );
    let validation_error =
        submit(&mut Mempool::default(), &snapshot, invalid).expect_err("invalid spend");
    assert!(matches!(validation_error, MempoolError::Validation { .. }));

    let original = spend_transaction(
        coinbase_txids[0],
        0,
        499_999_000,
        TransactionInput::MAX_SEQUENCE_NONFINAL - 1,
    );
    let mut mempool = Mempool::new(PolicyConfig {
        incremental_relay_feerate: crate::FeeRate::from_sats_per_kvb(10_000),
        ..PolicyConfig::default()
    });
    submit(&mut mempool, &snapshot, original).expect("original");
    let conflict_txid = *mempool.entries().keys().next().expect("conflict txid");

    let absolute_fee_error = mempool
        .validate_replacement(
            &spend_transaction(
                coinbase_txids[0],
                0,
                499_998_000,
                TransactionInput::SEQUENCE_FINAL,
            ),
            &BTreeSet::from([conflict_txid]),
            1_000,
            1,
        )
        .expect_err("absolute fee should fail");
    assert!(matches!(
        absolute_fee_error,
        MempoolError::ReplacementRejected { ref reason }
        if reason.contains("must exceed conflicting fee")
    ));

    let low_feerate_error = mempool
        .validate_replacement(
            &spend_transaction(
                coinbase_txids[0],
                0,
                499_998_000,
                TransactionInput::SEQUENCE_FINAL,
            ),
            &BTreeSet::from([conflict_txid]),
            2_000,
            2_000,
        )
        .expect_err("feerate should fail");
    assert!(matches!(
        low_feerate_error,
        MempoolError::ReplacementRejected { ref reason }
        if reason.contains("replacement feerate")
    ));

    let incremental_error = mempool
        .validate_replacement(
            &spend_transaction(
                coinbase_txids[0],
                0,
                499_998_000,
                TransactionInput::SEQUENCE_FINAL,
            ),
            &BTreeSet::from([conflict_txid]),
            1_001,
            10,
        )
        .expect_err("incremental relay bump should fail");
    assert!(matches!(
        incremental_error,
        MempoolError::ReplacementRejected { ref reason }
        if reason.contains("replacement fee bump")
    ));

    let stale_conflict = mempool.validate_replacement(
        &spend_transaction(
            coinbase_txids[0],
            0,
            499_998_000,
            TransactionInput::SEQUENCE_FINAL,
        ),
        &BTreeSet::from([Txid::from_byte_array([42_u8; 32])]),
        2_000,
        100,
    );
    assert!(stale_conflict.is_ok());
}

#[test]
fn helper_functions_cover_missing_vout_and_limit_branches() {
    let (snapshot, coinbase_txids) = sample_chainstate_snapshot(2);
    let parent = spend_transaction(
        coinbase_txids[0],
        0,
        499_999_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    let parent_txid = open_bitcoin_consensus::transaction_txid(&parent).expect("txid");
    let parent_wtxid = open_bitcoin_consensus::transaction_wtxid(&parent).expect("wtxid");
    let entries = HashMap::from([(
        parent_txid,
        MempoolEntry::new(
            parent,
            parent_txid,
            parent_wtxid,
            Amount::from_sats(1000).expect("amount"),
            100,
            400,
            0,
        ),
    )]);
    let missing_vout = super::derive_input_contexts(
        &spend_transaction(
            parent_txid,
            9,
            499_998_000,
            TransactionInput::SEQUENCE_FINAL,
        ),
        &snapshot,
        &entries,
    )
    .expect_err("missing vout should fail");
    assert!(matches!(missing_vout, MempoolError::MissingInput { .. }));

    let candidate_txid = Txid::from_byte_array([11_u8; 32]);
    let candidate = MempoolEntry::new(
        spend_transaction(
            coinbase_txids[1],
            0,
            499_999_000,
            TransactionInput::SEQUENCE_FINAL,
        ),
        candidate_txid,
        open_bitcoin_consensus::transaction_wtxid(&spend_transaction(
            coinbase_txids[1],
            0,
            499_999_000,
            TransactionInput::SEQUENCE_FINAL,
        ))
        .expect("wtxid"),
        Amount::from_sats(1000).expect("amount"),
        100,
        400,
        0,
    );
    let mut descendant_parent = candidate.clone();
    descendant_parent.descendant_stats = crate::AggregateStats::new(2, 200, 2_000);
    let oversized_ancestor = super::validate_limits(
        &HashMap::from([(candidate_txid, candidate.clone())]),
        &PolicyConfig {
            max_ancestor_virtual_size: 50,
            ..PolicyConfig::default()
        },
        candidate_txid,
    )
    .expect_err("ancestor vsize should fail");
    assert!(matches!(
        oversized_ancestor,
        MempoolError::LimitExceeded { .. }
    ));

    let descendant_limit = super::validate_limits(
        &HashMap::from([(candidate_txid, descendant_parent)]),
        &PolicyConfig {
            max_descendant_count: 1,
            ..PolicyConfig::default()
        },
        candidate_txid,
    )
    .expect_err("descendant count should fail");
    assert!(matches!(
        descendant_limit,
        MempoolError::LimitExceeded { .. }
    ));

    let mut descendant_size_parent = candidate;
    descendant_size_parent.descendant_stats = crate::AggregateStats::new(1, 200, 1_000);
    let descendant_size = super::validate_limits(
        &HashMap::from([(candidate_txid, descendant_size_parent)]),
        &PolicyConfig {
            max_descendant_virtual_size: 50,
            ..PolicyConfig::default()
        },
        candidate_txid,
    )
    .expect_err("descendant size should fail");
    assert!(matches!(
        descendant_size,
        MempoolError::LimitExceeded { .. }
    ));
}

#[test]
fn trim_and_graph_helpers_cover_remaining_internal_branches() {
    let empty_trimmed = super::trim_to_size(
        super::MempoolState {
            entries: HashMap::new(),
            spent_outpoints: HashMap::new(),
            total_virtual_size: 1,
        },
        &PolicyConfig {
            max_mempool_virtual_size: 0,
            ..PolicyConfig::default()
        },
    );
    assert!(empty_trimmed.1.is_empty());

    let base = spend_transaction(
        Txid::from_byte_array([1_u8; 32]),
        0,
        499_999_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    let base_txid = open_bitcoin_consensus::transaction_txid(&base).expect("txid");
    let base_wtxid = open_bitcoin_consensus::transaction_wtxid(&base).expect("wtxid");
    let left = spend_transaction(base_txid, 0, 499_998_000, TransactionInput::SEQUENCE_FINAL);
    let left_txid = open_bitcoin_consensus::transaction_txid(&left).expect("txid");
    let left_wtxid = open_bitcoin_consensus::transaction_wtxid(&left).expect("wtxid");
    let right = spend_transaction(base_txid, 0, 499_997_000, TransactionInput::SEQUENCE_FINAL);
    let right_txid = open_bitcoin_consensus::transaction_txid(&right).expect("txid");
    let right_wtxid = open_bitcoin_consensus::transaction_wtxid(&right).expect("wtxid");
    let mut leaf = spend_transaction(left_txid, 0, 499_996_000, TransactionInput::SEQUENCE_FINAL);
    leaf.inputs.push(TransactionInput {
        previous_output: OutPoint {
            txid: right_txid,
            vout: 0,
        },
        script_sig: script(&[0x01, 0x51]),
        sequence: TransactionInput::SEQUENCE_FINAL,
        witness: ScriptWitness::default(),
    });
    let leaf_txid = open_bitcoin_consensus::transaction_txid(&leaf).expect("txid");
    let leaf_wtxid = open_bitcoin_consensus::transaction_wtxid(&leaf).expect("wtxid");

    let entries = HashMap::from([
        (
            base_txid,
            MempoolEntry::new(
                base,
                base_txid,
                base_wtxid,
                Amount::from_sats(1000).expect("amount"),
                100,
                400,
                0,
            ),
        ),
        (
            left_txid,
            MempoolEntry::new(
                left,
                left_txid,
                left_wtxid,
                Amount::from_sats(1000).expect("amount"),
                100,
                400,
                0,
            ),
        ),
        (
            right_txid,
            MempoolEntry::new(
                right,
                right_txid,
                right_wtxid,
                Amount::from_sats(1000).expect("amount"),
                100,
                400,
                0,
            ),
        ),
        (
            leaf_txid,
            MempoolEntry::new(
                leaf,
                leaf_txid,
                leaf_wtxid,
                Amount::from_sats(1000).expect("amount"),
                100,
                400,
                0,
            ),
        ),
    ]);
    let recomputed = super::recompute_state(entries);
    let ancestors = super::collect_ancestors(&recomputed.entries, leaf_txid);
    let descendants = super::collect_descendants(&recomputed.entries, base_txid);
    assert!(ancestors.contains(&base_txid));
    assert!(descendants.contains(&leaf_txid));
}

#[test]
fn recompute_state_skips_invalid_parent_links_and_candidate_eviction_is_reported() {
    let txid = Txid::from_byte_array([4_u8; 32]);
    let invalid_parent = MempoolEntry::new(
        spend_transaction(
            Txid::from_byte_array([1_u8; 32]),
            1,
            499_999_000,
            TransactionInput::SEQUENCE_FINAL,
        ),
        txid,
        open_bitcoin_consensus::transaction_wtxid(&spend_transaction(
            Txid::from_byte_array([1_u8; 32]),
            1,
            499_999_000,
            TransactionInput::SEQUENCE_FINAL,
        ))
        .expect("wtxid"),
        Amount::from_sats(100).expect("amount"),
        100,
        400,
        0,
    );
    let state = super::recompute_state(HashMap::from([(txid, invalid_parent)]));
    assert!(state.entries.get(&txid).expect("entry").parents.is_empty());

    let (snapshot, coinbase_txids) = sample_chainstate_snapshot(2);
    let mut mempool = Mempool::new(PolicyConfig {
        max_mempool_virtual_size: 1,
        ..PolicyConfig::default()
    });
    let error = submit(
        &mut mempool,
        &snapshot,
        spend_transaction(
            coinbase_txids[0],
            0,
            499_999_000,
            TransactionInput::SEQUENCE_FINAL,
        ),
    )
    .expect_err("tiny mempool should evict candidate");

    assert!(matches!(error, MempoolError::CandidateEvicted { .. }));
}

#[test]
fn validate_limits_reports_missing_candidate_as_internal_invariant() {
    // Arrange
    let entries = HashMap::new();
    let config = PolicyConfig::default();
    let candidate_txid = Txid::from_byte_array([9_u8; 32]);

    // Act
    let error = super::validate_limits(&entries, &config, candidate_txid)
        .expect_err("missing candidate should be reported without panicking");

    // Assert
    assert!(matches!(error, MempoolError::InternalInvariant { .. }));
    assert!(error.to_string().contains("candidate"));
}

#[test]
fn validate_limits_reports_missing_ancestor_as_internal_invariant() {
    // Arrange
    let candidate_txid = Txid::from_byte_array([9_u8; 32]);
    let missing_ancestor_txid = Txid::from_byte_array([8_u8; 32]);
    let transaction = spend_transaction(
        Txid::from_byte_array([7_u8; 32]),
        0,
        499_999_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    let mut entry = MempoolEntry::new(
        transaction.clone(),
        candidate_txid,
        open_bitcoin_consensus::transaction_wtxid(&transaction).expect("wtxid"),
        Amount::from_sats(100).expect("amount"),
        100,
        400,
        0,
    );
    entry.parents.insert(missing_ancestor_txid);
    let entries = HashMap::from([(candidate_txid, entry)]);

    // Act
    let error = super::validate_limits(&entries, &PolicyConfig::default(), candidate_txid)
        .expect_err("missing ancestor should be reported without panicking");

    // Assert
    assert!(matches!(error, MempoolError::InternalInvariant { .. }));
    assert!(error.to_string().contains("ancestor"));
}
