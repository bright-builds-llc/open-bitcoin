// Parity breadcrumbs:
// - packages/bitcoin-knots/src/policy/packages.h
// - packages/bitcoin-knots/src/policy/packages.cpp
// - packages/bitcoin-knots/src/validation.h
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/test/txpackage_tests.cpp

use std::collections::HashMap;

use open_bitcoin_chainstate::{ChainstateSnapshot, Coin};
use open_bitcoin_consensus::{transaction_txid, transaction_wtxid};
use open_bitcoin_primitives::{
    Amount, OutPoint, ScriptBuf, ScriptWitness, Transaction, TransactionInput, TransactionOutput,
    Txid, Wtxid,
};

use super::shape::transaction_encoding_error;
use super::*;
use crate::{
    FeeRate, MempoolMemberIdentity, MempoolRejectionCategory, TransactionVirtualSize,
    transaction_weight_and_virtual_size,
};

fn transaction_with_input(seed: u8) -> Transaction {
    Transaction {
        version: 2,
        inputs: vec![TransactionInput {
            previous_output: OutPoint {
                txid: Txid::from_byte_array([seed; 32]),
                vout: u32::from(seed),
            },
            script_sig: ScriptBuf::default(),
            sequence: TransactionInput::SEQUENCE_FINAL,
            witness: ScriptWitness::default(),
        }],
        outputs: vec![TransactionOutput {
            value: Amount::from_sats(i64::from(seed) + 1).expect("fixture amount"),
            script_pubkey: ScriptBuf::default(),
        }],
        lock_time: u32::from(seed),
    }
}

fn child_of(parent: &Transaction, seed: u8) -> Transaction {
    let parent_txid = transaction_txid(parent).expect("fixture parent txid");
    Transaction {
        inputs: vec![TransactionInput {
            previous_output: OutPoint {
                txid: parent_txid,
                vout: 0,
            },
            ..transaction_with_input(seed).inputs[0].clone()
        }],
        ..transaction_with_input(seed)
    }
}

fn transaction_with_weight(target_weight: usize) -> Transaction {
    let mut payload_len = target_weight.saturating_sub(256);
    for _ in 0..4 {
        let mut transaction = transaction_with_input(201);
        transaction.inputs[0].witness = ScriptWitness::new(vec![vec![0_u8; payload_len]]);
        let (weight, _) =
            transaction_weight_and_virtual_size(&transaction).expect("fixture weight");
        if weight == target_weight {
            return transaction;
        }
        if weight < target_weight {
            payload_len += target_weight - weight;
        } else {
            payload_len -= weight - target_weight;
        }
    }

    panic!("unable to construct exact-weight transaction fixture");
}

fn snapshot_with_utxo(outpoint: OutPoint) -> ChainstateSnapshot {
    let coin = Coin {
        output: TransactionOutput {
            value: Amount::from_sats(10_000).expect("fixture amount"),
            script_pubkey: ScriptBuf::default(),
        },
        is_coinbase: false,
        created_height: 1,
        created_median_time_past: 1,
    };
    ChainstateSnapshot::new(vec![], HashMap::from([(outpoint, coin)]), HashMap::new())
}

fn wtxid_from_display_hex(display_hex: &str) -> Wtxid {
    let mut bytes = decode_hex(display_hex);
    bytes.reverse();
    Wtxid::from_byte_array(bytes.try_into().expect("32-byte wtxid fixture"))
}

fn decode_hex(input: &str) -> Vec<u8> {
    input
        .as_bytes()
        .chunks_exact(2)
        .map(|pair| {
            let high = char::from(pair[0]).to_digit(16).expect("hex fixture");
            let low = char::from(pair[1]).to_digit(16).expect("hex fixture");
            ((high << 4) | low) as u8
        })
        .collect()
}

fn report_fixture() -> WellFormedPackage {
    WellFormedPackage::try_from(vec![transaction_with_input(70), transaction_with_input(71)])
        .expect("report fixture package")
}

fn report_identity(package: &WellFormedPackage, index: usize) -> MempoolMemberIdentity {
    package
        .maybe_identity_at(index)
        .expect("fixture identity at index")
}

fn fee_group(
    id: EffectiveFeeGroupId,
    ordered_wtxids: Vec<Wtxid>,
) -> Result<EffectiveFeeGroup, super::EffectiveFeeGroupError> {
    let base_fee_sats = Amount::from_sats(200).expect("valid base fee");
    let modified_fee_sats = Amount::from_sats(300).expect("valid modified fee");
    let virtual_size = TransactionVirtualSize::new(100);
    let effective_fee_rate = FeeRate::from_fee_sats_and_vbytes(300, virtual_size);
    EffectiveFeeGroup::try_new(
        id,
        ordered_wtxids,
        base_fee_sats,
        modified_fee_sats,
        virtual_size,
        effective_fee_rate,
    )
}

fn finally_present(
    requested: MempoolMemberIdentity,
    effective_fee_group_id: EffectiveFeeGroupId,
) -> PackageMemberResult {
    PackageMemberResult::FinallyPresent(NewlyPresent {
        requested,
        effective_fee_group_id,
    })
}

mod empty_package_is_rejected;
mod ephemeral_policy_failure_preserves_requested_identity;
mod too_many_member_results_are_rejected;
