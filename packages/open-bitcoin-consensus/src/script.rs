use core::fmt;

use open_bitcoin_primitives::{Amount, ScriptBuf, ScriptWitness, Transaction};

use crate::context::{
    PrecomputedTransactionData, ScriptExecutionData, ScriptVerifyFlags, TransactionInputContext,
    TransactionValidationContext,
};

mod encoding;
mod legacy;
mod opcodes;
mod sigops;
mod stack;
mod taproot;
mod witness;

use self::legacy::{eval_script as eval_script_impl, verify_script as verify_script_impl};
use self::sigops::{
    count_legacy_sigops as count_legacy_sigops_impl, count_p2sh_sigops as count_p2sh_sigops_impl,
    count_witness_sigops as count_witness_sigops_impl,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScriptError {
    BadOpcode,
    DisabledOpcode(u8),
    EvalFalse,
    InvalidStackOperation,
    NumOverflow(usize),
    OpCount,
    OpReturn,
    PubKeyCount,
    PubKeyType,
    PushSize(usize),
    SigCount,
    SigDer,
    SigHashType,
    SigHighS,
    SigNullDummy,
    SigNullFail,
    SigPushOnly,
    StackOverflow(usize),
    TruncatedPushData,
    UnbalancedConditional,
    UnsupportedOpcode(u8),
    VerifyFailed,
    WitnessCleanStack,
    WitnessMalleated,
    WitnessMalleatedP2sh,
    WitnessProgramMismatch,
    WitnessProgramWitnessEmpty,
    WitnessProgramWrongLength,
    WitnessPubKeyType,
    WitnessUnexpected,
}

impl fmt::Display for ScriptError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BadOpcode => write!(f, "bad opcode"),
            Self::DisabledOpcode(opcode) => write!(f, "disabled opcode: 0x{opcode:02x}"),
            Self::EvalFalse => write!(f, "script evaluated to false"),
            Self::InvalidStackOperation => write!(f, "invalid stack operation"),
            Self::NumOverflow(size) => write!(f, "script number overflow: {size} bytes"),
            Self::OpCount => write!(f, "script exceeds opcode limit"),
            Self::OpReturn => write!(f, "OP_RETURN encountered"),
            Self::PubKeyCount => write!(f, "invalid public key count"),
            Self::PubKeyType => write!(f, "invalid public key encoding"),
            Self::PushSize(size) => write!(f, "push exceeds stack element limit: {size} bytes"),
            Self::SigCount => write!(f, "invalid signature count"),
            Self::SigDer => write!(f, "invalid DER signature"),
            Self::SigHashType => write!(f, "invalid signature hash type"),
            Self::SigHighS => write!(f, "non-low-S signature"),
            Self::SigNullDummy => write!(f, "non-null CHECKMULTISIG dummy argument"),
            Self::SigNullFail => write!(f, "non-null failing signature"),
            Self::SigPushOnly => write!(f, "scriptSig is not push-only"),
            Self::StackOverflow(size) => write!(f, "stack exceeds maximum size: {size}"),
            Self::TruncatedPushData => write!(f, "truncated pushdata"),
            Self::UnbalancedConditional => write!(f, "unbalanced conditional"),
            Self::UnsupportedOpcode(opcode) => write!(f, "unsupported opcode: 0x{opcode:02x}"),
            Self::VerifyFailed => write!(f, "VERIFY failed"),
            Self::WitnessCleanStack => write!(f, "witness script did not leave a clean stack"),
            Self::WitnessMalleated => write!(f, "witness program has unexpected scriptSig"),
            Self::WitnessMalleatedP2sh => {
                write!(f, "nested witness program scriptSig is malleated")
            }
            Self::WitnessProgramMismatch => write!(f, "witness program mismatch"),
            Self::WitnessProgramWitnessEmpty => write!(f, "witness program witness stack is empty"),
            Self::WitnessProgramWrongLength => write!(f, "witness program wrong length"),
            Self::WitnessPubKeyType => write!(f, "witness public key must be compressed"),
            Self::WitnessUnexpected => write!(f, "unexpected witness data"),
        }
    }
}

impl std::error::Error for ScriptError {}

pub struct ScriptInputVerificationContext<'a> {
    pub script_sig: &'a ScriptBuf,
    pub script_pubkey: &'a ScriptBuf,
    pub witness: &'a ScriptWitness,
    pub transaction: &'a Transaction,
    pub input_index: usize,
    pub spent_input: &'a TransactionInputContext,
    pub validation_context: &'a TransactionValidationContext,
    pub spent_amount: Amount,
    pub verify_flags: ScriptVerifyFlags,
    pub precomputed: &'a PrecomputedTransactionData,
    pub execution_data: &'a mut ScriptExecutionData,
}

pub fn eval_script(stack: &mut Vec<Vec<u8>>, script: &ScriptBuf) -> Result<(), ScriptError> {
    eval_script_impl(stack, script)
}

pub fn verify_script(script_sig: &ScriptBuf, script_pubkey: &ScriptBuf) -> Result<(), ScriptError> {
    verify_script_impl(script_sig, script_pubkey)
}

pub fn verify_input_script(context: ScriptInputVerificationContext<'_>) -> Result<(), ScriptError> {
    witness::verify_input_script(context)
}

pub fn count_legacy_sigops(script: &ScriptBuf) -> Result<usize, ScriptError> {
    count_legacy_sigops_impl(script)
}

pub fn count_p2sh_sigops(
    script_sig: &ScriptBuf,
    script_pubkey: &ScriptBuf,
) -> Result<usize, ScriptError> {
    count_p2sh_sigops_impl(script_sig, script_pubkey)
}

pub fn count_witness_sigops(
    script_sig: &ScriptBuf,
    script_pubkey: &ScriptBuf,
    witness: &ScriptWitness,
    verify_flags: ScriptVerifyFlags,
) -> Result<usize, ScriptError> {
    count_witness_sigops_impl(script_sig, script_pubkey, witness, verify_flags)
}

#[cfg(test)]
mod tests;
