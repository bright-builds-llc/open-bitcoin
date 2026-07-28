// Parity breadcrumbs:
// - packages/bitcoin-knots/doc/policy/packages.md
// - packages/bitcoin-knots/src/kernel/mempool_options.h
// - packages/bitcoin-knots/src/kernel/mempool_removal_reason.h
// - packages/bitcoin-knots/src/policy/packages.cpp
// - packages/bitcoin-knots/src/policy/policy.h
// - packages/bitcoin-knots/src/policy/rbf.cpp
// - packages/bitcoin-knots/src/rpc/mempool.cpp
// - packages/bitcoin-knots/src/txmempool.cpp
// - packages/bitcoin-knots/src/txmempool.h

mod context_cases;
mod expiry_cases;
mod fee_cases;
#[allow(deprecated)] // Compatibility summary regressions remain until Plan 130-07.
mod lifecycle_cases;
mod lifecycle_delta_cases;
#[allow(deprecated)] // Compatibility projection regressions remain until Plans 130-05/130-11.
mod outcome_cases;
mod package_admission_cases;
mod package_parity_cases;
mod package_policy_cases;
mod prepared_lifecycle_cases;
#[path = "tests/prepared_maintenance_cases.rs"]
mod prepared_lifecycle_cases_maintenance;
mod pressure_cases;
mod prospective_failure_cases;
mod prospective_oracle_cases;
mod resource_cases;
mod revision_cases;
mod rolling_fee_cases;
mod sustained_pressure_cases;

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

use super::*;
use crate::{
    LimitDirection, LimitKind, MempoolCapacity, MempoolEntry, MempoolError, PolicyConfig,
    RbfPolicy, TransactionVirtualSize,
};

const EASY_BITS: u32 = 0x207f_ffff;

pub(super) fn script(bytes: &[u8]) -> ScriptBuf {
    ScriptBuf::from_bytes(bytes.to_vec()).expect("valid script")
}

pub(super) fn redeem_script() -> ScriptBuf {
    script(&[0x51])
}

pub(super) fn p2sh_script() -> ScriptBuf {
    let redeem_hash = hash160(redeem_script().as_bytes());
    let mut bytes = vec![0xa9, 20];
    bytes.extend_from_slice(&redeem_hash);
    bytes.push(0x87);
    script(&bytes)
}

pub(super) fn serialized_script_num(value: i64) -> Vec<u8> {
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

pub(super) fn coinbase_transaction(height: u32, value: i64) -> Transaction {
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

pub(super) fn spend_transaction(
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

pub(super) fn non_standard_spend(previous_txid: Txid) -> Transaction {
    let mut transaction = spend_transaction(
        previous_txid,
        0,
        499_000_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    transaction.outputs[0].script_pubkey = script(&[0x51]);
    transaction
}

pub(super) fn build_block(previous_block_hash: BlockHash, height: u32, value: i64) -> Block {
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

pub(super) fn mine_header(block: &mut Block) {
    block.header.nonce = (0..=u32::MAX)
        .find(|nonce| {
            block.header.nonce = *nonce;
            check_block_header(&block.header).is_ok()
        })
        .expect("expected nonce at easy target");
}

pub(super) fn sample_chainstate_snapshot(block_count: u32) -> (ChainstateSnapshot, Vec<Txid>) {
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

pub(super) fn submit(
    mempool: &mut Mempool,
    snapshot: &ChainstateSnapshot,
    transaction: Transaction,
) -> Result<crate::AdmissionResult, MempoolError> {
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
        crate::AdmissionContext::legacy_unknown(),
    )
}

mod accepts_standard_confirmed_spend;
mod admission_maps_validation_errors_and_replacement_policy_edges;
