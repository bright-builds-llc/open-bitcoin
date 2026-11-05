// Parity breadcrumbs:
// - packages/bitcoin-knots/src/consensus/tx_verify.cpp
// - packages/bitcoin-knots/src/script/script.cpp
// - packages/bitcoin-knots/src/script/interpreter.cpp

use super::{BlockSigopBudget, transaction_sigop_cost};
use crate::context::{
    ConsensusParams, ScriptVerifyFlags, SpentOutput, TransactionInputContext,
    TransactionValidationContext,
};
use crate::script::{count_legacy_sigops, count_p2sh_sigops};
use open_bitcoin_primitives::{
    Amount, OutPoint, ScriptBuf, ScriptWitness, Transaction, TransactionInput, TransactionOutput,
};

fn script(bytes: &[u8]) -> ScriptBuf {
    ScriptBuf::from_bytes(bytes.to_vec()).expect("bounded fixture script")
}

fn spending(
    script_sig: &[u8],
    script_pubkey: &[u8],
    flags: ScriptVerifyFlags,
) -> (Transaction, TransactionValidationContext) {
    let transaction = Transaction {
        version: 2,
        inputs: vec![TransactionInput {
            previous_output: OutPoint {
                vout: 0,
                ..OutPoint::null()
            },
            script_sig: script(script_sig),
            sequence: TransactionInput::SEQUENCE_FINAL,
            witness: ScriptWitness::default(),
        }],
        outputs: vec![TransactionOutput {
            value: Amount::ZERO,
            script_pubkey: ScriptBuf::default(),
        }],
        lock_time: 0,
    };
    let context = TransactionValidationContext {
        inputs: vec![TransactionInputContext {
            spent_output: SpentOutput {
                value: Amount::ZERO,
                script_pubkey: script(script_pubkey),
                is_coinbase: false,
            },
            created_height: 0,
            created_median_time_past: 0,
        }],
        spend_height: 1,
        block_time: 1,
        median_time_past: 0,
        verify_flags: flags,
        consensus_params: ConsensusParams::default(),
    };
    (transaction, context)
}

fn p2sh() -> Vec<u8> {
    let mut bytes = vec![0xa9, 20];
    bytes.extend_from_slice(&[1; 20]);
    bytes.push(0x87);
    bytes
}

#[test]
fn phase167_sigops_p2sh_disabled_exception_counts_no_redeem_ops() {
    // Arrange
    let (transaction, context) = spending(&[1, 0xac], &p2sh(), ScriptVerifyFlags::NONE);
    // Act / Assert
    assert_eq!(
        crate::block::split_sigop_cost(&transaction, &context).expect("count"),
        0
    );
}

#[test]
fn phase167_sigops_malformed_getop_returns_count_so_far() {
    // Arrange
    let malformed = script(&[0xac, 0x4c]);
    // Act / Assert
    assert_eq!(count_legacy_sigops(&malformed), Ok(1));
}

#[test]
fn phase167_sigops_accurate_op_zero_multisig_is_twenty() {
    // Arrange
    let redeem = script(&[2, 0, 0xae]);
    // Act / Assert
    assert_eq!(count_p2sh_sigops(&redeem, &script(&p2sh())), Ok(20));
}

fn coins(context: &TransactionValidationContext) -> Vec<SpentOutput> {
    context
        .inputs
        .iter()
        .map(|input| input.spent_output.clone())
        .collect()
}

#[test]
fn phase167_sigops_exact_eighty_thousand_and_one_over() {
    // Arrange
    let mut witness_program = vec![0, 20];
    witness_program.extend_from_slice(&[1; 20]);
    let (mut transaction, context) = spending(
        &[],
        &witness_program,
        ScriptVerifyFlags::P2SH | ScriptVerifyFlags::WITNESS,
    );
    transaction.outputs = vec![
        TransactionOutput {
            value: Amount::ZERO,
            script_pubkey: script(&vec![0xac; 10_000])
        };
        2
    ];
    let mut exact = BlockSigopBudget::new();
    // Act / Assert
    assert_eq!(
        exact.add_transaction(&transaction, &coins(&context), ScriptVerifyFlags::NONE),
        Ok(())
    );
    assert_eq!(exact.cost(), 80_000);
    let mut over = BlockSigopBudget::new();
    assert_eq!(
        over.add_transaction(&transaction, &coins(&context), context.verify_flags)
            .expect_err("80001")
            .reject_reason,
        "bad-blk-sigops"
    );
    assert_eq!(over.cost(), 0);
}

#[test]
fn phase167_sigops_aggregate_includes_coinbase_and_every_transaction() {
    // Arrange
    let (mut coinbase, _) = spending(&[], &[], ScriptVerifyFlags::NONE);
    coinbase.inputs[0].previous_output = OutPoint::null();
    coinbase.outputs[0].script_pubkey = script(&[0xac]);
    let (mut transaction, context) = spending(&[], &[], ScriptVerifyFlags::NONE);
    transaction.outputs = [10_000, 9_999]
        .into_iter()
        .map(|count| TransactionOutput {
            value: Amount::ZERO,
            script_pubkey: script(&vec![0xac; count]),
        })
        .collect();
    let mut budget = BlockSigopBudget::new();
    // Act / Assert
    budget
        .add_transaction(&coinbase, &[], ScriptVerifyFlags::NONE)
        .expect("coinbase four");
    budget
        .add_transaction(&transaction, &coins(&context), context.verify_flags)
        .expect("remaining79996");
    assert_eq!(budget.cost(), 80_000);
    assert_eq!(
        budget
            .add_transaction(&coinbase, &[], ScriptVerifyFlags::NONE)
            .expect_err("aggregate excess")
            .reject_reason,
        "bad-blk-sigops"
    );
    assert_eq!(budget.cost(), 80_000);
}

#[test]
fn phase167_sigops_missing_or_extra_coins_refuse_even_without_flags() {
    // Arrange
    let (transaction, context) = spending(&[], &[], ScriptVerifyFlags::NONE);
    let mut budget = BlockSigopBudget::new();
    // Act / Assert
    for spent in [vec![], vec![context.inputs[0].spent_output.clone(); 2]] {
        assert_eq!(
            budget
                .add_transaction(&transaction, &spent, ScriptVerifyFlags::NONE)
                .expect_err("exact coins required")
                .reject_reason,
            "bad-txns-inputs-missingorspent"
        );
        assert_eq!(budget.cost(), 0);
    }
}

#[test]
fn phase167_sigops_checked_aggregate_arithmetic_retains_counter() {
    // Arrange
    let mut budget = BlockSigopBudget { cost: usize::MAX };
    // Act / Assert
    assert_eq!(
        budget
            .add_cost(1)
            .expect_err("checked overflow")
            .reject_reason,
        "bad-blk-sigops"
    );
    assert_eq!(budget.cost(), usize::MAX);
}

#[test]
fn phase167_sigops_p2sh_scaling_and_flag_sensitive_contribution() {
    // Arrange
    let (transaction, context) = spending(&[3, 0x52, 0xae, 0xac], &p2sh(), ScriptVerifyFlags::P2SH);
    // Act / Assert: accurate2-of-N multisig plus CHECKSIG, scaled once.
    assert_eq!(
        transaction_sigop_cost(&transaction, &coins(&context), context.verify_flags),
        Ok(12)
    );
    assert_eq!(
        transaction_sigop_cost(&transaction, &coins(&context), ScriptVerifyFlags::NONE),
        Ok(0)
    );
}

#[test]
fn phase167_sigops_witness_script_uses_accurate_unscaled_count() {
    // Arrange
    let mut program = vec![0, 32];
    program.extend_from_slice(&[1; 32]);
    let (mut transaction, context) = spending(
        &[],
        &program,
        ScriptVerifyFlags::P2SH | ScriptVerifyFlags::WITNESS,
    );
    transaction.inputs[0].witness = ScriptWitness::new(vec![vec![0x53, 0xae, 0xad]]);
    // Act / Assert
    assert_eq!(
        transaction_sigop_cost(&transaction, &coins(&context), context.verify_flags),
        Ok(4)
    );
    assert_eq!(
        transaction_sigop_cost(&transaction, &coins(&context), ScriptVerifyFlags::P2SH),
        Ok(0)
    );
    transaction.inputs[0].witness = ScriptWitness::default();
    assert_eq!(
        transaction_sigop_cost(&transaction, &coins(&context), context.verify_flags),
        Ok(0)
    );
}

#[test]
fn phase167_sigops_nested_witness_is_counted_once_and_taproot_zero() {
    // Arrange
    let mut redeem = vec![22, 0, 20];
    redeem.extend_from_slice(&[1; 20]);
    let (transaction, context) = spending(
        &redeem,
        &p2sh(),
        ScriptVerifyFlags::P2SH | ScriptVerifyFlags::WITNESS,
    );
    // Act / Assert
    assert_eq!(
        transaction_sigop_cost(&transaction, &coins(&context), context.verify_flags),
        Ok(1)
    );
    let mut taproot = vec![0x51, 32];
    taproot.extend_from_slice(&[1; 32]);
    let (transaction, context) = spending(
        &[],
        &taproot,
        context.verify_flags | ScriptVerifyFlags::TAPROOT,
    );
    assert_eq!(
        transaction_sigop_cost(&transaction, &coins(&context), context.verify_flags),
        Ok(0)
    );
}

#[test]
fn phase167_sigops_pushdata_larger_than_interpreter_element_limit_still_iterates() {
    // Arrange: GetOp has no520-byte execution limit.
    let mut bytes = vec![0x4d, 0x09, 0x02];
    bytes.extend_from_slice(&[0xac; 521]);
    bytes.push(0xac);
    // Act / Assert
    assert_eq!(count_legacy_sigops(&script(&bytes)), Ok(1));
    assert_eq!(
        crate::script::eval_script(&mut Vec::new(), &script(&bytes)),
        Err(crate::script::ScriptError::PushSize(521))
    );
}

#[test]
fn phase167_sigops_malformed_redeem_keeps_prior_count() {
    // Arrange
    let (transaction, context) = spending(&[2, 0xac, 0x4c], &p2sh(), ScriptVerifyFlags::P2SH);
    // Act / Assert
    assert_eq!(
        transaction_sigop_cost(&transaction, &coins(&context), context.verify_flags),
        Ok(4)
    );
}

#[test]
fn phase167_sigops_all_truncated_push_forms_stop_after_prior_count() {
    for bytes in [
        vec![0xac, 2, 0],
        vec![0xac, 0x4c],
        vec![0xac, 0x4c, 2, 0],
        vec![0xac, 0x4d, 1],
        vec![0xac, 0x4e, 1, 0, 0],
    ] {
        // Arrange / Act / Assert
        assert_eq!(count_legacy_sigops(&script(&bytes)), Ok(1));
    }
    assert_eq!(count_legacy_sigops(&script(&[1])), Ok(0));
    assert_eq!(count_legacy_sigops(&script(&[2, 0xac, 0xae, 0xac])), Ok(1));
}

#[test]
fn phase167_sigops_accurate_multisig_counts_only_immediate_op_one_through_sixteen() {
    for opcode in [0xae, 0xaf] {
        for (prefix, expected) in [
            (vec![], 20),
            (vec![0], 20),
            (vec![0x4f], 20),
            (vec![1, 1], 20),
            (vec![0x51], 1),
            (vec![0x60], 16),
        ] {
            // Arrange
            let mut redeem = prefix;
            redeem.push(opcode);
            let mut bytes = vec![redeem.len() as u8];
            bytes.extend_from_slice(&redeem);
            // Act / Assert
            assert_eq!(
                count_p2sh_sigops(&script(&bytes), &script(&p2sh())),
                Ok(expected)
            );
            assert_eq!(count_legacy_sigops(&script(&redeem)), Ok(20));
        }
    }
}

#[test]
fn phase167_sigops_p2sh_final_raw_data_clears_on_small_or_reserved_opcode() {
    for (bytes, expected) in [
        (vec![1, 0xac], 1),
        (vec![1, 0xac, 0x51], 0),
        (vec![1, 0xac, 0x50], 0),
        (vec![0x50, 1, 0xac], 1),
        (vec![1, 0xac, 0x4c], 0),
        (vec![1, 0xac, 0x61], 0),
    ] {
        // Arrange / Act / Assert
        assert_eq!(
            count_p2sh_sigops(&script(&bytes), &script(&p2sh())),
            Ok(expected)
        );
    }
}

#[test]
fn phase167_sigops_malformed_witness_script_counts_before_execution_failure() {
    // Arrange
    let mut program = vec![0, 32];
    program.extend_from_slice(&[1; 32]);
    let (mut transaction, context) = spending(
        &[],
        &program,
        ScriptVerifyFlags::P2SH | ScriptVerifyFlags::WITNESS,
    );
    transaction.inputs[0].witness = ScriptWitness::new(vec![vec![0xac, 0x4c]]);
    // Act / Assert
    assert_eq!(
        transaction_sigop_cost(&transaction, &coins(&context), context.verify_flags),
        Ok(1)
    );
    assert_eq!(
        crate::script::eval_script(&mut Vec::new(), &script(&[0x4c])),
        Err(crate::script::ScriptError::TruncatedPushData)
    );
}

#[test]
fn phase167_sigops_nested_witness_uses_final_raw_push() {
    let mut redeem = vec![22, 0, 20];
    redeem.extend_from_slice(&[1; 20]);
    for (prefix, suffix, expected) in [
        (vec![], vec![], 1),
        (vec![0x50], vec![], 1),
        (vec![], vec![0x51], 0),
        (vec![], vec![0x4c], 0),
    ] {
        // Arrange
        let bytes = [prefix, redeem.clone(), suffix].concat();
        let (transaction, context) = spending(
            &bytes,
            &p2sh(),
            ScriptVerifyFlags::P2SH | ScriptVerifyFlags::WITNESS,
        );
        // Act / Assert
        assert_eq!(
            transaction_sigop_cost(&transaction, &coins(&context), context.verify_flags),
            Ok(expected)
        );
    }
}
