#![forbid(unsafe_code)]

//! Pure-core consensus and validation checks for Open Bitcoin.

pub mod block;
pub mod classify;
pub mod context;
pub mod crypto;
pub mod script;
pub mod sighash;
pub mod signature;
pub mod transaction;
pub mod validation;

pub use block::{
    check_block, check_block_contextual, check_block_header, check_block_header_contextual,
    validate_block, validate_block_with_context,
};
pub use classify::{
    ScriptPubKeyType, classify_script_pubkey, extract_redeem_script, extract_script_sig_pushes,
    is_push_only,
};
pub use context::{
    BlockValidationContext, ConsensusParams, PrecomputedTransactionData, ScriptExecutionData,
    ScriptVerifyFlags, SpentOutput, TransactionInputContext, TransactionValidationContext,
    calculate_sequence_locks, check_tx_inputs, evaluate_sequence_locks, is_final_transaction,
    sequence_locks,
};
pub use crypto::{
    CompactTargetError, block_hash, block_merkle_root, check_proof_of_work, transaction_txid,
    transaction_wtxid,
};
pub use script::{
    ScriptError, ScriptInputVerificationContext, count_legacy_sigops, eval_script,
    verify_input_script, verify_script,
};
pub use sighash::{
    SigHashType, SigVersion, legacy_sighash, segwit_v0_sighash, taproot_sighash,
    taproot_tagged_hash,
};
pub use signature::{
    EcdsaVerificationRequest, ParsedEcdsaSignature, ParsedSchnorrSignature, SignatureError,
    TransactionSignatureChecker, parse_ecdsa_signature, parse_public_key, parse_schnorr_signature,
    parse_xonly_public_key,
};
pub use transaction::{check_transaction, validate_transaction, validate_transaction_with_context};
pub use validation::{
    BlockValidationError, BlockValidationResult, TxValidationError, TxValidationResult,
    ValidationError,
};

pub const WITNESS_SCALE_FACTOR: usize = 4;
pub const MAX_BLOCK_WEIGHT: usize = 4_000_000;
pub const MAX_BLOCK_SIGOPS_COST: usize = 80_000;

pub const fn crate_ready() -> bool {
    true
}

#[cfg(test)]
mod tests {
    use super::crate_ready;

    #[test]
    fn crate_ready_reports_true() {
        assert!(crate_ready());
    }
}
