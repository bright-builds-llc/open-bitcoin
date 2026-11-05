// Parity breadcrumbs:
// - packages/bitcoin-knots/src/script/interpreter.cpp
// - packages/bitcoin-knots/src/script/script.h
// - packages/bitcoin-knots/src/primitives/transaction.h

use open_bitcoin_primitives::TransactionInput;

use crate::context::ScriptVerifyFlags;

use super::ScriptError;
use super::legacy::LegacyExecutionContext;
use super::stack::decode_bounded_script_num;

fn operand(stack: &[Vec<u8>], flags: ScriptVerifyFlags) -> Result<i64, ScriptError> {
    let bytes = stack.last().ok_or(ScriptError::InvalidStackOperation)?;
    let value = decode_bounded_script_num(bytes, 5)?;
    if flags.contains(ScriptVerifyFlags::MINIMALDATA)
        && bytes.last().is_some_and(|last| last & 0x7f == 0)
        && (bytes.len() == 1 || bytes[bytes.len() - 2] & 0x80 == 0)
    {
        return Err(ScriptError::VerifyFailed);
    }
    if value < 0 {
        return Err(ScriptError::NegativeLockTime);
    }
    Ok(value)
}

pub(super) fn check_lock_time(
    stack: &[Vec<u8>],
    maybe_context: Option<&LegacyExecutionContext<'_>>,
) -> Result<(), ScriptError> {
    let Some(context) = maybe_context else {
        return Ok(());
    };
    if !context
        .verify_flags
        .contains(ScriptVerifyFlags::CHECKLOCKTIMEVERIFY)
    {
        return Ok(());
    }
    let lock_time = operand(stack, context.verify_flags)?;
    let input = context
        .transaction
        .inputs
        .get(context.input_index)
        .ok_or(ScriptError::UnsatisfiedLockTime)?;
    let transaction_lock_time = i64::from(context.transaction.lock_time);
    if (lock_time < 500_000_000) != (transaction_lock_time < 500_000_000)
        || lock_time > transaction_lock_time
        || input.sequence == TransactionInput::SEQUENCE_FINAL
    {
        return Err(ScriptError::UnsatisfiedLockTime);
    }
    Ok(())
}

pub(super) fn check_sequence(
    stack: &[Vec<u8>],
    maybe_context: Option<&LegacyExecutionContext<'_>>,
) -> Result<(), ScriptError> {
    let Some(context) = maybe_context else {
        return Ok(());
    };
    if !context
        .verify_flags
        .contains(ScriptVerifyFlags::CHECKSEQUENCEVERIFY)
    {
        return Ok(());
    }
    let sequence = operand(stack, context.verify_flags)?;
    if sequence & i64::from(TransactionInput::SEQUENCE_LOCKTIME_DISABLE_FLAG) != 0 {
        return Ok(());
    }
    let input = context
        .transaction
        .inputs
        .get(context.input_index)
        .ok_or(ScriptError::UnsatisfiedLockTime)?;
    if context.transaction.version < 2
        || input.sequence & TransactionInput::SEQUENCE_LOCKTIME_DISABLE_FLAG != 0
    {
        return Err(ScriptError::UnsatisfiedLockTime);
    }
    let mask = i64::from(
        TransactionInput::SEQUENCE_LOCKTIME_TYPE_FLAG | TransactionInput::SEQUENCE_LOCKTIME_MASK,
    );
    let expected = sequence & mask;
    let actual = i64::from(input.sequence) & mask;
    let type_flag = i64::from(TransactionInput::SEQUENCE_LOCKTIME_TYPE_FLAG);
    if (expected & type_flag) != (actual & type_flag) || expected > actual {
        return Err(ScriptError::UnsatisfiedLockTime);
    }
    Ok(())
}
