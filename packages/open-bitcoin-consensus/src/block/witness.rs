// Parity breadcrumbs:
// - packages/bitcoin-knots/src/primitives/block.h
// - packages/bitcoin-knots/src/consensus/merkle.cpp
// - packages/bitcoin-knots/src/pow.cpp
// - packages/bitcoin-knots/src/validation.cpp

use open_bitcoin_codec::CodecError;
use open_bitcoin_primitives::{Block, MerkleRoot};

use super::{consensus_error, map_codec_error};
use crate::context::BlockValidationContext;
use crate::crypto::{double_sha256, transaction_wtxid};
use crate::validation::{BlockValidationError, BlockValidationResult, block_error};

const WITNESS_RESERVED_VALUE_STACK_ITEMS: usize = 1;
const WITNESS_RESERVED_VALUE_SIZE: usize = 32;
const WITNESS_COMMITMENT_PREIMAGE_SIZE: usize = 64;
const WITNESS_COMMITMENT_HEADER: [u8; 6] = [0x6a, 0x24, 0xaa, 0x21, 0xa9, 0xed];
const WITNESS_COMMITMENT_HEADER_LEN: usize = WITNESS_COMMITMENT_HEADER.len();
const WITNESS_COMMITMENT_SCRIPT_LEN: usize =
    WITNESS_COMMITMENT_HEADER_LEN + WITNESS_RESERVED_VALUE_SIZE;
const WITNESS_COMMITMENT_START: usize = WITNESS_COMMITMENT_HEADER_LEN;
const WITNESS_COMMITMENT_END: usize = WITNESS_COMMITMENT_START + WITNESS_RESERVED_VALUE_SIZE;
const HASH_CONCATENATION_SIZE: usize = 64;

pub(super) fn check_witness_commitment(
    block: &Block,
    context: &BlockValidationContext,
) -> Result<(), BlockValidationError> {
    let witness_present = block
        .transactions
        .iter()
        .any(|transaction| transaction.has_witness());
    if !context.consensus_params.enforce_segwit {
        if witness_present {
            return Err(block_error(
                BlockValidationResult::Mutated,
                "unexpected-witness",
                Some("unexpected witness data found".to_string()),
            ));
        }

        return Ok(());
    }

    let Some(commitment_index) = witness_commitment_index(block) else {
        if witness_present {
            return Err(block_error(
                BlockValidationResult::Mutated,
                "unexpected-witness",
                Some("unexpected witness data found".to_string()),
            ));
        }

        return Ok(());
    };
    let coinbase_transaction = required_coinbase_transaction(block)?;
    let coinbase_input = required_coinbase_input(coinbase_transaction)?;
    let reserved_value = required_witness_reserved_value(coinbase_input)?;
    if coinbase_input.witness.stack().len() != WITNESS_RESERVED_VALUE_STACK_ITEMS
        || reserved_value.len() != WITNESS_RESERVED_VALUE_SIZE
    {
        return Err(block_error(
            BlockValidationResult::Mutated,
            "bad-witness-nonce-size",
            Some("invalid witness reserved value size".to_string()),
        ));
    }

    let witness_root = block_witness_merkle_root(block).map_err(map_codec_error)?;
    let mut commitment_preimage = [0_u8; WITNESS_COMMITMENT_PREIMAGE_SIZE];
    commitment_preimage[..WITNESS_RESERVED_VALUE_SIZE].copy_from_slice(witness_root.as_bytes());
    commitment_preimage[WITNESS_RESERVED_VALUE_SIZE..].copy_from_slice(reserved_value);
    let expected_commitment = double_sha256(&commitment_preimage);

    let commitment_output = required_commitment_output(coinbase_transaction, commitment_index)?;
    let commitment_script = commitment_output.script_pubkey.as_bytes();
    if commitment_script[WITNESS_COMMITMENT_START..WITNESS_COMMITMENT_END] != expected_commitment {
        return Err(block_error(
            BlockValidationResult::Mutated,
            "bad-witness-merkle-match",
            Some("witness merkle commitment mismatch".to_string()),
        ));
    }

    Ok(())
}

pub(super) fn witness_commitment_index(block: &Block) -> Option<usize> {
    let coinbase_outputs = block.transactions.first()?.outputs.iter().enumerate();
    let mut maybe_commitment_index = None;
    for (index, output) in coinbase_outputs {
        let bytes = output.script_pubkey.as_bytes();
        if has_witness_commitment_header(bytes) {
            maybe_commitment_index = Some(index);
        }
    }
    maybe_commitment_index
}

pub(super) fn block_witness_merkle_root(block: &Block) -> Result<MerkleRoot, CodecError> {
    if block.transactions.is_empty() {
        return Ok(MerkleRoot::from_byte_array([0_u8; 32]));
    }

    let mut level = Vec::with_capacity(block.transactions.len());
    level.push([0_u8; 32]);
    for transaction in block.transactions.iter().skip(1) {
        level.push(transaction_wtxid(transaction)?.to_byte_array());
    }

    while level.len() > 1 {
        if level.len() % 2 == 1
            && let Some(last_hash) = level.last().copied()
        {
            level.push(last_hash);
        }

        let mut next_level = Vec::with_capacity(level.len() / 2);
        for pair in level.chunks_exact(2) {
            let mut concatenated = [0_u8; HASH_CONCATENATION_SIZE];
            concatenated[..WITNESS_RESERVED_VALUE_SIZE].copy_from_slice(&pair[0]);
            concatenated[WITNESS_RESERVED_VALUE_SIZE..].copy_from_slice(&pair[1]);
            next_level.push(double_sha256(&concatenated));
        }
        level = next_level;
    }

    Ok(MerkleRoot::from_byte_array(
        level.first().copied().unwrap_or([0_u8; 32]),
    ))
}

fn has_witness_commitment_header(bytes: &[u8]) -> bool {
    bytes.len() >= WITNESS_COMMITMENT_SCRIPT_LEN && bytes.starts_with(&WITNESS_COMMITMENT_HEADER)
}

fn required_coinbase_transaction(
    block: &Block,
) -> Result<&open_bitcoin_primitives::Transaction, BlockValidationError> {
    let Some(coinbase_transaction) = block.transactions.first() else {
        return Err(consensus_error(
            "bad-cb-missing",
            Some("first tx is not coinbase".to_string()),
        ));
    };
    Ok(coinbase_transaction)
}

fn required_coinbase_input(
    coinbase_transaction: &open_bitcoin_primitives::Transaction,
) -> Result<&open_bitcoin_primitives::TransactionInput, BlockValidationError> {
    let Some(coinbase_input) = coinbase_transaction.inputs.first() else {
        return Err(consensus_error(
            "bad-cb-missing",
            Some("coinbase transaction has no inputs".to_string()),
        ));
    };
    Ok(coinbase_input)
}

fn required_witness_reserved_value(
    coinbase_input: &open_bitcoin_primitives::TransactionInput,
) -> Result<&Vec<u8>, BlockValidationError> {
    let Some(reserved_value) = coinbase_input.witness.stack().first() else {
        return Err(block_error(
            BlockValidationResult::Mutated,
            "bad-witness-nonce-size",
            Some("invalid witness reserved value size".to_string()),
        ));
    };
    Ok(reserved_value)
}

fn required_commitment_output(
    coinbase_transaction: &open_bitcoin_primitives::Transaction,
    commitment_index: usize,
) -> Result<&open_bitcoin_primitives::TransactionOutput, BlockValidationError> {
    let Some(commitment_output) = coinbase_transaction.outputs.get(commitment_index) else {
        return Err(block_error(
            BlockValidationResult::Mutated,
            "bad-witness-merkle-match",
            Some("witness commitment output missing".to_string()),
        ));
    };
    Ok(commitment_output)
}

#[cfg(test)]
mod tests {
    use open_bitcoin_primitives::{
        Amount, Block, OutPoint, ScriptBuf, ScriptWitness, Transaction, TransactionInput,
        TransactionOutput,
    };

    use super::{
        required_coinbase_input, required_coinbase_transaction, required_commitment_output,
        required_witness_reserved_value,
    };

    fn script(bytes: &[u8]) -> ScriptBuf {
        ScriptBuf::from_bytes(bytes.to_vec()).expect("valid script")
    }

    fn transaction(inputs: Vec<TransactionInput>, outputs: Vec<TransactionOutput>) -> Transaction {
        Transaction {
            version: 1,
            inputs,
            outputs,
            lock_time: 0,
        }
    }

    #[test]
    fn required_witness_helpers_report_missing_state() {
        // Arrange
        let empty_block = Block {
            header: Default::default(),
            transactions: vec![],
        };
        let no_input_transaction = transaction(
            vec![],
            vec![TransactionOutput {
                value: Amount::ZERO,
                script_pubkey: script(&[0x51]),
            }],
        );
        let no_reserved_input = TransactionInput {
            previous_output: OutPoint::null(),
            script_sig: ScriptBuf::default(),
            sequence: TransactionInput::SEQUENCE_FINAL,
            witness: ScriptWitness::default(),
        };

        // Act
        let missing_transaction =
            required_coinbase_transaction(&empty_block).expect_err("empty block should fail");
        let missing_input =
            required_coinbase_input(&no_input_transaction).expect_err("missing input should fail");
        let missing_reserved = required_witness_reserved_value(&no_reserved_input)
            .expect_err("missing reserved value should fail");
        let missing_output = required_commitment_output(&no_input_transaction, 1)
            .expect_err("missing commitment output should fail");

        // Assert
        assert_eq!(missing_transaction.reject_reason, "bad-cb-missing");
        assert_eq!(missing_input.reject_reason, "bad-cb-missing");
        assert_eq!(missing_reserved.reject_reason, "bad-witness-nonce-size");
        assert_eq!(missing_output.reject_reason, "bad-witness-merkle-match");
    }
}
