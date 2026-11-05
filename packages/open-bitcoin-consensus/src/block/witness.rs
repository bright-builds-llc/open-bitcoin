// Parity breadcrumbs:
// - packages/bitcoin-knots/src/primitives/block.h
// - packages/bitcoin-knots/src/consensus/merkle.cpp
// - packages/bitcoin-knots/src/pow.cpp
// - packages/bitcoin-knots/src/validation.cpp

use open_bitcoin_codec::CodecError;
use open_bitcoin_codec::block::wire::BlockView;
use open_bitcoin_codec::transaction::wire::TransactionView;
use open_bitcoin_primitives::{Block, MerkleRoot};

use super::{consensus_error, map_codec_error};
use crate::context::BlockValidationContext;
use crate::crypto::double_sha256;
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

pub(super) fn update_uncommitted_block_structures(
    block: &mut Block,
    rules: &crate::CandidateRules,
) {
    if !rules.enforce_segwit_commitment() || witness_commitment_index(block).is_none() {
        return;
    }
    for coinbase in block.transactions.iter_mut().take(1) {
        if coinbase.has_witness() {
            return;
        }
        let Some(input) = coinbase.inputs.first_mut() else {
            return;
        };
        input.witness = open_bitcoin_primitives::ScriptWitness::new(vec![vec![0; 32]]);
    }
}

pub(super) enum WitnessMalleationError {
    Encoding(CodecError),
    Invalid(BlockValidationError),
}

impl From<CodecError> for WitnessMalleationError {
    fn from(error: CodecError) -> Self {
        Self::Encoding(error)
    }
}

impl From<BlockValidationError> for WitnessMalleationError {
    fn from(error: BlockValidationError) -> Self {
        Self::Invalid(error)
    }
}

pub(super) fn check_witness_commitment(
    block: &Block,
    context: &BlockValidationContext,
) -> Result<(), BlockValidationError> {
    check_witness_malleation(block, context.consensus_params.enforce_segwit).map_err(|error| {
        match error {
            WitnessMalleationError::Encoding(error) => map_codec_error(error),
            WitnessMalleationError::Invalid(error) => error,
        }
    })
}

pub(super) fn check_witness_malleation(
    block: &impl BlockView,
    expect_witness_commitment: bool,
) -> Result<(), WitnessMalleationError> {
    let maybe_commitment_index = if expect_witness_commitment {
        witness_commitment_index(block)
    } else {
        None
    };
    let Some(commitment_index) = maybe_commitment_index else {
        if block.transactions().iter().any(|transaction| {
            transaction
                .inputs()
                .iter()
                .any(|input| !input.witness.is_empty())
        }) {
            return Err(block_error(
                BlockValidationResult::Mutated,
                "unexpected-witness",
                Some("unexpected witness data found".to_string()),
            )
            .into());
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
        )
        .into());
    }

    let witness_root = block_witness_merkle_root(block)?;
    let mut commitment_preimage = [0_u8; WITNESS_COMMITMENT_PREIMAGE_SIZE];
    commitment_preimage[..WITNESS_RESERVED_VALUE_SIZE].copy_from_slice(witness_root.as_bytes());
    commitment_preimage[WITNESS_RESERVED_VALUE_SIZE..].copy_from_slice(reserved_value);
    let expected_commitment = double_sha256(&commitment_preimage);

    let commitment_output = required_commitment_output(coinbase_transaction, commitment_index)?;
    let commitment_script = commitment_output.as_bytes();
    if commitment_script[WITNESS_COMMITMENT_START..WITNESS_COMMITMENT_END] != expected_commitment {
        return Err(block_error(
            BlockValidationResult::Mutated,
            "bad-witness-merkle-match",
            Some("witness merkle commitment mismatch".to_string()),
        )
        .into());
    }
    Ok(())
}

pub(super) fn witness_commitment_index(block: &impl BlockView) -> Option<usize> {
    let coinbase_outputs = block.transactions().first()?.outputs().enumerate();
    let mut maybe_commitment_index = None;
    for (index, output) in coinbase_outputs {
        let bytes = output.script_pubkey.as_bytes();
        if has_witness_commitment_header(bytes) {
            maybe_commitment_index = Some(index);
        }
    }
    maybe_commitment_index
}

pub(super) fn block_witness_merkle_root(block: &impl BlockView) -> Result<MerkleRoot, CodecError> {
    super::mutation::witness_merkle_root(block)
}

fn has_witness_commitment_header(bytes: &[u8]) -> bool {
    bytes.len() >= WITNESS_COMMITMENT_SCRIPT_LEN && bytes.starts_with(&WITNESS_COMMITMENT_HEADER)
}

fn required_coinbase_transaction<B: BlockView>(
    block: &B,
) -> Result<&B::Transaction, BlockValidationError> {
    let Some(coinbase_transaction) = block.transactions().first() else {
        return Err(consensus_error(
            "bad-cb-missing",
            Some("first tx is not coinbase".to_string()),
        ));
    };
    Ok(coinbase_transaction)
}

fn required_coinbase_input(
    coinbase_transaction: &impl TransactionView,
) -> Result<&open_bitcoin_primitives::TransactionInput, BlockValidationError> {
    let Some(coinbase_input) = coinbase_transaction.inputs().first() else {
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
    coinbase_transaction: &impl TransactionView,
    commitment_index: usize,
) -> Result<&open_bitcoin_primitives::ScriptBuf, BlockValidationError> {
    let Some(commitment_output) = coinbase_transaction.outputs().nth(commitment_index) else {
        return Err(block_error(
            BlockValidationResult::Mutated,
            "bad-witness-merkle-match",
            Some("witness commitment output missing".to_string()),
        ));
    };
    Ok(commitment_output.script_pubkey)
}

#[cfg(test)]
mod tests {
    use open_bitcoin_primitives::{
        Amount, Block, BlockHeader, OutPoint, ScriptBuf, ScriptWitness, Transaction,
        TransactionInput, TransactionOutput,
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

    fn submission_rules(block: &Block, chain: crate::CanonicalChain) -> crate::CandidateRules {
        let history = crate::CandidateHistory::check(
            chain.spec().rules(),
            block.header.clone(),
            crate::HistoryHeader::new(chain, 0, chain.spec().genesis_header().clone()),
            vec![],
        )
        .expect("checked parent facts");
        crate::CandidateRules::derive(&history)
    }

    fn mine_after_invalid_nonce(header: &mut BlockHeader) {
        let params = crate::ConsensusParams::default();
        let mut skipped = 0;
        while crate::block::check_block_header_with_params(header, &params).is_ok() {
            assert!(skipped < 4_096, "bounded search for invalid starting nonce");
            skipped += 1;
            header.nonce = header.nonce.wrapping_add(1);
        }
        let mut repairs = 0;
        while crate::block::check_block_header_with_params(header, &params).is_err() {
            assert!(repairs < 4_096, "bounded real proof-of-work search");
            repairs += 1;
            header.nonce = header.nonce.wrapping_add(1);
        }
        assert!(repairs > 0);
        assert!(crate::block::check_block_header_with_params(header, &params).is_ok());
    }

    fn submission_body(wide: bool) -> Block {
        let mut block = crate::CanonicalChain::Regtest.spec().genesis_block();
        block.header.previous_block_hash = crate::block_hash(&block.header);
        block.header.version = 4;
        block.header.time += 1;
        let commitment = crate::crypto::double_sha256(&[0; 64]);
        block.transactions[0].outputs.push(TransactionOutput {
            value: Amount::ZERO,
            script_pubkey: ScriptBuf::from_serialized_bytes(
                [
                    vec![0x6a, 0x24, 0xaa, 0x21, 0xa9, 0xed],
                    commitment.to_vec(),
                ]
                .concat(),
            ),
        });
        if wide {
            block.transactions[0].outputs[0].script_pubkey =
                ScriptBuf::from_serialized_bytes(vec![0; 10001]);
        }
        block.header.merkle_root = crate::crypto::block_merkle_root(&block.transactions)
            .expect("base merkle")
            .0;
        mine_after_invalid_nonce(&mut block.header);
        block
    }

    fn wire(block: &Block) -> open_bitcoin_codec::block::wire::WireBlock {
        open_bitcoin_codec::block::wire::WireBlock {
            header: block.header.clone(),
            transactions: block
                .transactions
                .clone()
                .into_iter()
                .map(Into::into)
                .collect(),
        }
    }

    #[test]
    fn phase167_submission_pure_reserve_commutes_with_every_intrinsic_fact() {
        for wide in [false, true] {
            // Arrange
            let mut block = submission_body(wide);
            let original = block.clone();
            let rules = submission_rules(&block, crate::CanonicalChain::Regtest);
            let params = rules.consensus_params();
            let before = wire(&block);
            let intrinsic = crate::block::intrinsic::check_submission_wire_block(&before, &params);
            assert!(intrinsic.is_ok());
            // Act
            crate::block::update_uncommitted_block_structures(&mut block, &rules);
            let after = wire(&block);
            // Assert
            assert_eq!(
                block.transactions[0].inputs[0].witness.stack(),
                &[vec![0; 32]]
            );
            assert_eq!(
                open_bitcoin_codec::encode_block_header(&original.header),
                open_bitcoin_codec::encode_block_header(&block.header)
            );
            assert_eq!(
                crate::block_hash(&original.header),
                crate::block_hash(&block.header)
            );
            assert_eq!(
                crate::crypto::block_merkle_root_view(&before.transactions),
                crate::crypto::block_merkle_root_view(&after.transactions)
            );
            for (left, right) in before.transactions.iter().zip(&after.transactions) {
                let encoding = open_bitcoin_codec::TransactionEncoding::WithoutWitness;
                assert_eq!(
                    open_bitcoin_codec::transaction::encode_transaction_view(left, encoding),
                    open_bitcoin_codec::transaction::encode_transaction_view(right, encoding)
                );
                assert_eq!(
                    crate::crypto::transaction_txid_view(left),
                    crate::crypto::transaction_txid_view(right)
                );
                assert_eq!(left.outputs, right.outputs);
                assert_eq!(
                    left.inputs[0].previous_output,
                    right.inputs[0].previous_output
                );
                assert_eq!(
                    crate::transaction::intrinsic::is_coinbase(left),
                    crate::transaction::intrinsic::is_coinbase(right)
                );
            }
            assert_eq!(
                crate::block::sigops::legacy_block_sigop_cost(&before),
                crate::block::sigops::legacy_block_sigop_cost(&after)
            );
            assert_eq!(
                intrinsic,
                crate::block::intrinsic::check_submission_wire_block(&after, &params)
            );
            assert_eq!(wire(&block).try_into_block().expect("valid money"), block);
            assert_eq!(
                crate::block::intrinsic::serialized_block_size(&before, false),
                crate::block::intrinsic::serialized_block_size(&after, false)
            );
        }
    }

    #[test]
    fn phase167_submission_pure_existing_witness_absent_commitment_and_inactive_are_unchanged() {
        for case in 0..4 {
            // Arrange
            let mut block = submission_body(false);
            let chain = if case == 0 {
                crate::CanonicalChain::Mainnet
            } else {
                crate::CanonicalChain::Regtest
            };
            if case == 0 {
                block.header.previous_block_hash = chain.spec().genesis_hash();
            }
            if case == 1 {
                block.transactions[0].outputs.pop();
            }
            if case == 2 {
                block.transactions[0].inputs[0].witness = ScriptWitness::new(vec![vec![7]]);
            }
            if case == 3 {
                let mut second = block.transactions[0].inputs[0].clone();
                second.witness = ScriptWitness::new(vec![vec![8]]);
                block.transactions[0].inputs.push(second);
            }
            let rules = submission_rules(&block, chain);
            let before = block.clone();
            // Act
            crate::block::update_uncommitted_block_structures(&mut block, &rules);
            // Assert
            assert_eq!(block, before);
        }
    }

    #[test]
    fn phase167_submission_pure_extreme_amounts_fail_before_conversion_or_normalization() {
        for value in [i64::MIN, -1, i64::MAX] {
            // Arrange
            let body = submission_body(true);
            let mut raw = wire(&body);
            raw.transactions[0].outputs[0].value_sats = value;
            raw.header.merkle_root = crate::crypto::block_merkle_root_view(&raw.transactions)
                .expect("raw merkle")
                .0;
            mine_after_invalid_nonce(&mut raw.header);
            let witness = raw.transactions[0].inputs[0].witness.clone();
            // Act / Assert
            assert!(
                crate::block::intrinsic::check_submission_wire_block(
                    &raw,
                    &crate::ConsensusParams::default()
                )
                .is_err()
            );
            assert_eq!(raw.transactions[0].inputs[0].witness, witness);
            assert!(raw.try_into_block().is_err());
        }
    }

    #[test]
    fn phase167_submission_pure_missing_transaction_or_input_is_safe() {
        // Arrange
        let original = submission_body(false);
        let rules = submission_rules(&original, crate::CanonicalChain::Regtest);
        for no_transaction in [false, true] {
            let mut block = original.clone();
            if no_transaction {
                block.transactions.clear();
            } else {
                block.transactions[0].inputs.clear();
            }
            let before = block.clone();
            // Act
            crate::block::update_uncommitted_block_structures(&mut block, &rules);
            // Assert
            assert_eq!(block, before);
        }
    }

    fn reseal_submission_body(block: &mut Block) {
        block.header.merkle_root = crate::crypto::block_merkle_root(&block.transactions)
            .expect("base merkle")
            .0;
        while crate::block::check_block_header_with_params(
            &block.header,
            &crate::ConsensusParams::default(),
        )
        .is_err()
        {
            block.header.nonce += 1;
        }
    }

    #[test]
    fn phase167_submission_pure_preserves_every_transaction_and_input_field() {
        // Arrange
        let mut body = submission_body(true);
        let inputs = (0..2)
            .map(|index| TransactionInput {
                previous_output: OutPoint {
                    txid: open_bitcoin_primitives::Txid::from_byte_array([index + 1; 32]),
                    vout: u32::from(index),
                },
                script_sig: script(&[0x51, 0x52]),
                sequence: u32::from(index) + 9,
                witness: ScriptWitness::new(vec![vec![index + 1], vec![0x52, 0x53]]),
            })
            .collect();
        let mut spend = transaction(
            inputs,
            vec![TransactionOutput {
                value: Amount::from_sats(1).expect("valid amount"),
                script_pubkey: script(&[0x51, 0xac]),
            }],
        );
        spend.version = 2;
        spend.lock_time = 17;
        body.transactions.push(spend.clone());
        spend.inputs[0].previous_output.vout = 2;
        spend.lock_time = 19;
        body.transactions.push(spend);
        reseal_submission_body(&mut body);
        let before = wire(&body);
        let mut block = before.clone().try_into_block().expect("valid amounts");
        let rules = submission_rules(&block, crate::CanonicalChain::Regtest);
        let params = rules.consensus_params();
        let intrinsic = crate::block::intrinsic::check_submission_wire_block(&before, &params);
        assert!(intrinsic.is_ok());
        let mut expected = body;
        expected.transactions[0].inputs[0].witness = ScriptWitness::new(vec![vec![0; 32]]);

        // Act
        crate::block::update_uncommitted_block_structures(&mut block, &rules);

        // Assert
        assert_eq!(block, expected);
        let after = wire(&block);
        assert_eq!(
            open_bitcoin_codec::encode_block_header(&before.header),
            open_bitcoin_codec::encode_block_header(&after.header)
        );
        assert_eq!(
            crate::block_hash(&before.header),
            crate::block_hash(&after.header)
        );
        assert_eq!(
            crate::crypto::block_merkle_root_view(&before.transactions),
            crate::crypto::block_merkle_root_view(&after.transactions)
        );
        assert_eq!(
            crate::block::intrinsic::serialized_block_size(&before, false),
            crate::block::intrinsic::serialized_block_size(&after, false)
        );
        assert_eq!(
            crate::block::sigops::legacy_block_sigop_cost(&before),
            crate::block::sigops::legacy_block_sigop_cost(&after)
        );
        for (left, right) in before.transactions.iter().zip(&after.transactions) {
            let encoding = open_bitcoin_codec::TransactionEncoding::WithoutWitness;
            assert_eq!(
                open_bitcoin_codec::transaction::encode_transaction_view(left, encoding),
                open_bitcoin_codec::transaction::encode_transaction_view(right, encoding)
            );
            assert_eq!(
                crate::crypto::transaction_txid_view(left),
                crate::crypto::transaction_txid_view(right)
            );
        }
        assert_eq!(
            intrinsic,
            crate::block::intrinsic::check_submission_wire_block(&after, &params)
        );
    }

    #[test]
    fn phase167_submission_pure_preserves_domain_representable_intrinsic_failures() {
        for (case, reason) in [
            (0, "bad-txns-txouttotal-toolarge"),
            (1, "bad-cb-length"),
            (2, "bad-cb-missing"),
        ] {
            // Arrange
            let mut body = submission_body(false);
            match case {
                0 => {
                    body.transactions[0].outputs[0].value =
                        Amount::from_sats(open_bitcoin_primitives::MAX_MONEY)
                            .expect("valid amount");
                    body.transactions[0].outputs[1].value =
                        Amount::from_sats(1).expect("valid amount");
                }
                1 => body.transactions[0].inputs[0].script_sig = script(&[0x51]),
                _ => {
                    let input = body.transactions[0].inputs[0].clone();
                    body.transactions[0].inputs.push(input);
                }
            }
            reseal_submission_body(&mut body);
            let before = wire(&body);
            let mut block = before
                .clone()
                .try_into_block()
                .expect("valid individual amounts");
            let rules = submission_rules(&block, crate::CanonicalChain::Regtest);
            let params = rules.consensus_params();
            let intrinsic = crate::block::intrinsic::check_submission_wire_block(&before, &params);
            let error = intrinsic
                .as_ref()
                .expect_err("intrinsic failure before normalization");
            assert_eq!(error.result, crate::BlockValidationResult::Consensus);
            assert_eq!(error.reject_reason, reason);
            let mut expected = body;
            expected.transactions[0].inputs[0].witness = ScriptWitness::new(vec![vec![0; 32]]);

            // Act
            crate::block::update_uncommitted_block_structures(&mut block, &rules);

            // Assert
            assert_eq!(block, expected);
            assert_eq!(
                intrinsic,
                crate::block::intrinsic::check_submission_wire_block(&wire(&block), &params)
            );
        }
    }
}
