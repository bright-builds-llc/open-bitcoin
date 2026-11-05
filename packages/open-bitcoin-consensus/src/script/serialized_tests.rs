// Parity breadcrumbs:
// - packages/bitcoin-knots/src/script/interpreter.cpp
// - packages/bitcoin-knots/src/script/script.h
// - packages/bitcoin-knots/src/script/script_error.cpp

use open_bitcoin_primitives::{
    Amount, OutPoint, ScriptBuf, Transaction, TransactionInput, TransactionOutput, Txid,
};
use secp256k1::Secp256k1;

use super::legacy::{LegacyExecutionContext, eval_script_internal};
use super::{ScriptError, eval_script};
use crate::context::{
    ConsensusParams, PrecomputedTransactionData, ScriptExecutionData, ScriptVerifyFlags,
    SpentOutput, TransactionInputContext, TransactionValidationContext,
};
use crate::sighash::SigVersion;
use crate::signature::TransactionSignatureChecker;

pub(super) fn transaction() -> Transaction {
    Transaction {
        version: 2,
        inputs: vec![TransactionInput {
            previous_output: OutPoint {
                txid: Txid::from_byte_array([31; 32]),
                vout: 0,
            },
            script_sig: ScriptBuf::default(),
            sequence: TransactionInput::SEQUENCE_FINAL,
            witness: Default::default(),
        }],
        outputs: vec![TransactionOutput {
            value: Amount::from_sats(40).expect("valid amount"),
            script_pubkey: ScriptBuf::from_bytes(vec![0x51]).expect("small script"),
        }],
        lock_time: 0,
    }
}

pub(super) fn context(
    transaction: &Transaction,
) -> (
    TransactionInputContext,
    TransactionValidationContext,
    PrecomputedTransactionData,
) {
    let spent = TransactionInputContext {
        spent_output: SpentOutput {
            value: Amount::from_sats(50).expect("valid amount"),
            script_pubkey: ScriptBuf::default(),
            is_coinbase: false,
        },
        created_height: 0,
        created_median_time_past: 0,
    };
    let validation = TransactionValidationContext {
        inputs: vec![spent.clone()],
        spend_height: 1,
        block_time: 0,
        median_time_past: 0,
        verify_flags: ScriptVerifyFlags::NONE,
        consensus_params: ConsensusParams::default(),
    };
    let precomputed = validation.precompute(transaction).expect("precomputed");
    (spent, validation, precomputed)
}

fn evaluate(bytes: Vec<u8>, version: SigVersion) -> Result<Vec<Vec<u8>>, ScriptError> {
    let transaction = transaction();
    let (spent, validation, precomputed) = context(&transaction);
    let secp = Secp256k1::verification_only();
    let context = LegacyExecutionContext {
        checker: TransactionSignatureChecker::new(&secp, &validation, &precomputed),
        transaction: &transaction,
        input_index: 0,
        spent_input: &spent,
        verify_flags: ScriptVerifyFlags::NONE,
        sig_version: version,
    };
    let mut stack = Vec::new();
    eval_script_internal(
        &mut stack,
        &ScriptBuf::from_serialized_bytes(bytes),
        Some(&context),
        Some(&mut ScriptExecutionData::default()),
    )?;
    Ok(stack)
}

// Literal boundaries from the pinned interpreter, independent of Rust limit getters.
#[test]
fn phase167_serialized_script_version_raw_storage_retains_bytes() {
    // Arrange
    let bytes = vec![0xff; 10_001];
    // Act
    let raw = ScriptBuf::from_serialized_bytes(bytes.clone());
    // Assert
    assert_eq!(raw.into_bytes(), bytes);
    assert!(matches!(
        ScriptBuf::from_bytes(bytes),
        Err(open_bitcoin_primitives::ScriptError::TooLarge(10_001))
    ));
}

#[test]
fn phase167_serialized_script_version_default_size_precedes_opcode() {
    // Arrange
    let script = ScriptBuf::from_serialized_bytes(vec![0x6a; 10_001]);
    // Act
    let result = eval_script(&mut Vec::new(), &script);
    // Assert
    assert_eq!(result, Err(ScriptError::ScriptSize));
    assert_eq!(ScriptError::ScriptSize.to_string(), "Script is too big");
}

#[test]
fn phase167_serialized_script_version_base_and_v0_size_boundaries() {
    // Arrange
    let mut boundary = Vec::new();
    for _ in 0..19 {
        boundary.extend_from_slice(&[0x4d, 0x08, 0x02]);
        boundary.extend(vec![0; 520]);
        boundary.push(0x75);
    }
    boundary.extend_from_slice(&[0x4c, 0x29]);
    boundary.extend(vec![0; 41]);
    boundary.push(0x75);
    assert_eq!(boundary.len(), 10_000);
    for version in [SigVersion::Base, SigVersion::WitnessV0] {
        // Act
        let accepted = evaluate(boundary.clone(), version);
        let rejected = evaluate(vec![0xff; 10_001], version);
        // Assert
        assert_eq!(accepted, Ok(Vec::new()));
        assert_eq!(rejected, Err(ScriptError::ScriptSize));
    }
}

#[test]
fn phase167_serialized_script_version_opcode_limit_is_version_specific() {
    // Arrange
    for version in [SigVersion::Base, SigVersion::WitnessV0] {
        // Act / Assert
        assert_eq!(evaluate(vec![0x61; 201], version), Ok(Vec::new()));
        assert_eq!(
            evaluate(vec![0x61; 202], version),
            Err(ScriptError::OpCount)
        );
    }
    // Act / Assert
    assert_eq!(
        evaluate(vec![0x61; 10_001], SigVersion::Tapscript),
        Ok(Vec::new())
    );
}

#[test]
fn phase167_serialized_script_version_tapscript_keeps_element_limit() {
    // Arrange
    let mut bytes = vec![0x4d, 0x09, 0x02];
    bytes.extend(vec![0; 521]);
    // Act
    let result = evaluate(bytes, SigVersion::Tapscript);
    // Assert
    assert_eq!(result, Err(ScriptError::PushSize(521)));
}

#[test]
fn phase167_serialized_script_version_tapscript_keeps_stack_limit() {
    // Arrange
    let bytes = vec![0x51; 1001];
    // Act
    let result = evaluate(bytes, SigVersion::Tapscript);
    // Assert
    assert_eq!(result, Err(ScriptError::StackOverflow(1001)));
}
