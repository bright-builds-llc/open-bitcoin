// Parity breadcrumbs:
// - packages/bitcoin-knots/src/script/interpreter.cpp
// - packages/bitcoin-knots/src/script/script.h
// - packages/bitcoin-knots/src/primitives/transaction.h

use super::phase167_scripts::{body, funded, spending};
use super::{Chainstate, ScriptBuf, ScriptVerifyFlags, Transaction, script};
use open_bitcoin_consensus::script::{ScriptInputVerificationContext, verify_input_script};
use open_bitcoin_consensus::{ScriptError, ScriptExecutionData, TransactionInputContext};

fn push_number_bytes(bytes: &[u8]) -> ScriptBuf {
    let mut encoded = vec![bytes.len() as u8];
    encoded.extend_from_slice(bytes);
    script(&encoded)
}

fn probe_input(
    chain: &Chainstate,
    transaction: &Transaction,
    flags: ScriptVerifyFlags,
    index: usize,
) -> Result<(), ScriptError> {
    let before = chain.snapshot();
    let block = body(chain, transaction.clone());
    let history = super::super::candidate_rules::history_from_chain(
        open_bitcoin_consensus::CanonicalChain::Regtest
            .spec()
            .rules(),
        &block.header,
        &chain.active_chain,
        4096,
    )
    .expect("earned branch");
    let coin = chain
        .get_coin(&transaction.inputs[0].previous_output)
        .expect("coin read")
        .expect("funded output");
    let input = TransactionInputContext {
        spent_output: coin.as_spent_output(),
        created_height: coin.created_height,
        created_median_time_past: coin.created_median_time_past,
    };
    let context = open_bitcoin_consensus::TransactionValidationContext::from_candidate_history(
        transaction,
        vec![input],
        &history,
    )
    .expect("derived input context");
    let precomputed = context
        .precompute(transaction)
        .expect("precompute real transaction");
    let result = verify_input_script(ScriptInputVerificationContext {
        script_sig: &transaction.inputs[0].script_sig,
        script_pubkey: &context.inputs[0].spent_output.script_pubkey,
        witness: &transaction.inputs[0].witness,
        transaction,
        input_index: index,
        spent_input: &context.inputs[0],
        validation_context: &context,
        spent_amount: coin.output.value,
        verify_flags: flags,
        precomputed: &precomputed,
        execution_data: &mut ScriptExecutionData::default(),
    });
    assert_eq!(
        chain.snapshot(),
        before,
        "interpreter probe cannot mutate coins/undo/work"
    );
    result
}

#[test]
fn phase167_locktime_cltv_funded_interpreter_transaction_boundaries() {
    let (chain, funding) = funded(&[0xb1, 0x75, 0x51], 1);
    let flag = ScriptVerifyFlags::CHECKLOCKTIMEVERIFY;
    // Arrange / Act / Assert: canonical finality is separate; these are interpreter probes.
    for (operand, lock_time, sequence, expected) in [
        (vec![100], 100, 100, Ok(())),
        (vec![101], 100, 100, Err(ScriptError::UnsatisfiedLockTime)),
        (
            vec![100],
            100,
            u32::MAX,
            Err(ScriptError::UnsatisfiedLockTime),
        ),
        (vec![0xff, 0x64, 0xcd, 0x1d], 499_999_999, 0, Ok(())),
        (vec![0, 0x65, 0xcd, 0x1d], 500_000_000, 0, Ok(())),
        (
            vec![0, 0x65, 0xcd, 0x1d],
            499_999_999,
            0,
            Err(ScriptError::UnsatisfiedLockTime),
        ),
        (
            vec![0xff, 0x64, 0xcd, 0x1d],
            500_000_000,
            0,
            Err(ScriptError::UnsatisfiedLockTime),
        ),
        (vec![0xff, 0xff, 0xff, 0xff, 0], u32::MAX, 0, Ok(())),
        (
            vec![0, 0, 0, 0, 1],
            0,
            0,
            Err(ScriptError::UnsatisfiedLockTime),
        ),
        (
            vec![0xff, 0xff, 0xff, 0xff, 0x7f],
            u32::MAX,
            0,
            Err(ScriptError::UnsatisfiedLockTime),
        ),
    ] {
        let mut tx = spending(&funding);
        tx.inputs[0].script_sig = push_number_bytes(&operand);
        tx.inputs[0].sequence = sequence;
        tx.lock_time = lock_time;
        assert_eq!(
            probe_input(&chain, &tx, flag, 0),
            expected,
            "operand={operand:?}"
        );
    }
}

#[test]
fn phase167_locktime_csv_funded_interpreter_masks_and_checker_order() {
    // Arrange: opcode comparisons use transaction fields, independently of BIP68 age.
    let (chain, funding) = funded(&[0xb2, 0x75, 0x51], 1);
    let flag = ScriptVerifyFlags::CHECKSEQUENCEVERIFY;
    for (operand, version, sequence, expected) in [
        (vec![100], 2, 100, Ok(())),
        (vec![101], 2, 100, Err(ScriptError::UnsatisfiedLockTime)),
        (vec![0], 1, 0, Err(ScriptError::UnsatisfiedLockTime)),
        (
            vec![0],
            2,
            0x8000_0000,
            Err(ScriptError::UnsatisfiedLockTime),
        ),
        (
            vec![0, 0, 0x40],
            2,
            0,
            Err(ScriptError::UnsatisfiedLockTime),
        ),
        (
            vec![0],
            2,
            0x0040_0000,
            Err(ScriptError::UnsatisfiedLockTime),
        ),
        (vec![1, 0, 0x40], 2, 0x0040_0001, Ok(())),
        (vec![100, 0, 1], 2, 0x0002_0064, Ok(())),
        (vec![100, 0, 0, 0, 1], 2, 100, Ok(())),
        (vec![0xff, 0xff, 0xff, 0xff, 0], 1, u32::MAX, Ok(())),
        (vec![0, 0, 0, 0x80, 0], 1, u32::MAX, Ok(())),
        (
            vec![0, 0, 0, 0x80, 0x80],
            1,
            u32::MAX,
            Err(ScriptError::NegativeLockTime),
        ),
    ] {
        // Arrange
        let mut tx = spending(&funding);
        tx.version = version;
        tx.inputs[0].sequence = sequence;
        tx.inputs[0].script_sig = push_number_bytes(&operand);
        // Act / Assert
        assert_eq!(
            probe_input(&chain, &tx, flag, 0),
            expected,
            "operand={operand:?}"
        );
    }
    let mut disabled = spending(&funding);
    disabled.version = 1;
    disabled.inputs[0].script_sig = push_number_bytes(&[0, 0, 0, 0x80, 0]);
    assert_eq!(
        probe_input(&chain, &disabled, flag, 99),
        Ok(()),
        "operand-disable precedes checker access"
    );
    disabled.inputs[0].script_sig = script(&[0]);
    assert_eq!(
        probe_input(&chain, &disabled, flag, 99),
        Err(ScriptError::UnsatisfiedLockTime)
    );
}

#[test]
fn phase167_locktime_lock_operands_signed_size_minimal_and_inactive_nops() {
    for (opcode, flag) in [
        (0xb1, ScriptVerifyFlags::CHECKLOCKTIMEVERIFY),
        (0xb2, ScriptVerifyFlags::CHECKSEQUENCEVERIFY),
    ] {
        let (chain, funding) = funded(&[opcode, 0x75, 0x51], 1);
        for (operand, expected) in [
            (vec![0x81], Err(ScriptError::NegativeLockTime)),
            (vec![0; 6], Err(ScriptError::NumOverflow(6))),
            (vec![0x80], Ok(())),
            (vec![0], Ok(())),
            (vec![0, 0], Ok(())),
        ] {
            // Arrange
            let mut tx = spending(&funding);
            tx.inputs[0].sequence = 0;
            tx.inputs[0].script_sig = push_number_bytes(&operand);
            // Act / Assert
            assert_eq!(probe_input(&chain, &tx, flag, 0), expected);
            if expected == Ok(()) {
                assert_eq!(
                    probe_input(&chain, &tx, flag | ScriptVerifyFlags::MINIMALDATA, 0),
                    Err(ScriptError::VerifyFailed)
                );
            }
        }
        let mut tx = spending(&funding);
        tx.inputs[0].script_sig = ScriptBuf::default();
        assert_eq!(
            probe_input(&chain, &tx, flag, 0),
            Err(ScriptError::InvalidStackOperation)
        );
        tx.inputs[0].script_sig = script(&[0]);
        tx.inputs[0].sequence = 0;
        assert_eq!(
            probe_input(&chain, &tx, flag | ScriptVerifyFlags::MINIMALDATA, 0),
            Ok(())
        );
        assert_eq!(
            probe_input(&chain, &tx, flag, 99),
            Err(ScriptError::UnsatisfiedLockTime)
        );
        for operand in [vec![0x81], vec![0; 6]] {
            tx.inputs[0].script_sig = push_number_bytes(&operand);
            assert_eq!(
                probe_input(
                    &chain,
                    &tx,
                    ScriptVerifyFlags::DISCOURAGE_UPGRADABLE_NOPS,
                    99
                ),
                Ok(())
            );
        }
        let (empty_chain, empty_funding) = funded(&[opcode, 0x51], 1);
        let mut empty_tx = spending(&empty_funding);
        empty_tx.inputs[0].script_sig = ScriptBuf::default();
        assert_eq!(
            probe_input(
                &empty_chain,
                &empty_tx,
                ScriptVerifyFlags::DISCOURAGE_UPGRADABLE_NOPS,
                99
            ),
            Ok(())
        );
    }
}

#[test]
fn phase167_locktime_minimal_five_byte_operands_preserve_the_exact_stack_value() {
    for (opcode, flag, operand, lock_time, sequence) in [
        (
            0xb1,
            ScriptVerifyFlags::CHECKLOCKTIMEVERIFY,
            vec![0xff, 0xff, 0xff, 0xff, 0],
            u32::MAX,
            0,
        ),
        (
            0xb2,
            ScriptVerifyFlags::CHECKSEQUENCEVERIFY,
            vec![100, 0, 0, 0, 1],
            0,
            100,
        ),
    ] {
        // Arrange: compare the retained bytes after the active opcode, not just stack depth.
        let mut locking = vec![opcode];
        locking.extend_from_slice(push_number_bytes(&operand).as_bytes());
        locking.extend_from_slice(&[0x88, 0x51]);
        let (chain, funding) = funded(&locking, 1);
        let mut tx = spending(&funding);
        tx.lock_time = lock_time;
        tx.inputs[0].sequence = sequence;
        tx.inputs[0].script_sig = push_number_bytes(&operand);
        // Act
        let result = probe_input(&chain, &tx, flag | ScriptVerifyFlags::MINIMALDATA, 0);
        // Assert
        assert_eq!(result, Ok(()));
    }
}

#[test]
fn phase167_locktime_generic_no_context_interfaces_keep_inactive_nop_semantics() {
    for opcode in [0xb1, 0xb2] {
        // Arrange: generic eval has no verify flags or transaction checker.
        let mut stack = vec![vec![0; 6]];
        let before = stack.clone();
        // Act
        let result = open_bitcoin_consensus::script::eval_script(&mut stack, &script(&[opcode]));
        // Assert
        assert_eq!(result, Ok(()));
        assert_eq!(stack, before);
        assert_eq!(
            open_bitcoin_consensus::script::verify_script(
                &ScriptBuf::default(),
                &script(&[opcode, 0x51])
            ),
            Ok(())
        );
    }
}

#[test]
fn phase167_locktime_lock_skipped_branches_and_four_byte_arithmetic_remain_distinct() {
    // Arrange
    let (chain, funding) = funded(&[0, 0x63, 0xb1, 0xb2, 0x68, 0x51], 1);
    // Act / Assert
    assert_eq!(
        probe_input(
            &chain,
            &spending(&funding),
            ScriptVerifyFlags::P2SH
                | ScriptVerifyFlags::DERSIG
                | ScriptVerifyFlags::CHECKLOCKTIMEVERIFY
                | ScriptVerifyFlags::CHECKSEQUENCEVERIFY
                | ScriptVerifyFlags::NULLDUMMY
                | ScriptVerifyFlags::WITNESS
                | ScriptVerifyFlags::TAPROOT,
            99
        ),
        Ok(())
    );
    let (chain, funding) = funded(&[0x8b, 0x75, 0x51], 1);
    let mut tx = spending(&funding);
    tx.inputs[0].script_sig = push_number_bytes(&[0xff, 0xff, 0xff, 0xff, 0]);
    assert_eq!(
        probe_input(&chain, &tx, ScriptVerifyFlags::NONE, 0),
        Err(ScriptError::NumOverflow(5))
    );
}
