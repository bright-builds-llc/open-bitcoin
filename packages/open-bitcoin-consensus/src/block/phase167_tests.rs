// Parity breadcrumbs:
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/script/script.h
// - packages/bitcoin-knots/src/consensus/tx_verify.cpp

use super::*;
use crate::context::{calculate_sequence_locks, sequence_locks};
use crate::{
    CandidateHistory, CanonicalChain, HistoryHeader, ScriptVerifyFlags, TransactionInputContext,
};
use open_bitcoin_primitives::{
    OutPoint, ScriptBuf, ScriptWitness, Transaction, TransactionInput, TransactionOutput,
};

fn script(bytes: &[u8]) -> ScriptBuf {
    ScriptBuf::from_bytes(bytes.to_vec()).expect("bounded fixture script")
}

fn coinbase(bytes: &[u8]) -> Transaction {
    Transaction {
        version: 2,
        inputs: vec![TransactionInput {
            previous_output: OutPoint::null(),
            script_sig: script(bytes),
            sequence: TransactionInput::SEQUENCE_FINAL,
            witness: ScriptWitness::default(),
        }],
        outputs: vec![TransactionOutput {
            value: Amount::ZERO,
            script_pubkey: script(&[0x51]),
        }],
        lock_time: 0,
    }
}

#[test]
fn phase167_block_context_height_one_through_sixteen_use_op_n() {
    for height in 1..=16 {
        // Arrange
        let block = Block {
            header: Default::default(),
            transactions: vec![coinbase(&[0x50 + height as u8, 0])],
        };
        // Act / Assert
        assert!(
            coinbase_has_height_prefix(&block, height),
            "height {height}"
        );
        let nonminimal = Block {
            transactions: vec![coinbase(&[1, height as u8])],
            ..block
        };
        assert!(!coinbase_has_height_prefix(&nonminimal, height));
    }
}

#[test]
fn phase167_block_context_signed_height_edges() {
    for (height, bytes) in [
        (17, vec![1, 17]),
        (127, vec![1, 127]),
        (128, vec![2, 128, 0]),
        (32767, vec![2, 255, 127]),
        (32768, vec![3, 0, 128, 0]),
    ] {
        // Arrange
        let block = Block {
            header: Default::default(),
            transactions: vec![coinbase(&bytes)],
        };
        // Act / Assert
        assert!(coinbase_has_height_prefix(&block, height));
        assert!(!coinbase_has_height_prefix(&block, height + 1));
    }
}

fn history(chain: CanonicalChain, height: u32, offset: u32, version: i32) -> CandidateHistory {
    let mut rows = Vec::new();
    let mut previous = Default::default();
    let window = if chain == CanonicalChain::Mainnet {
        2200
    } else {
        160
    };
    for at in height.saturating_sub(window)..height {
        let header = if at == 0 {
            chain.spec().genesis_header().clone()
        } else {
            BlockHeader {
                version: 4,
                previous_block_hash: previous,
                time: 1_600_000_000 + at * 600 + offset,
                bits: chain.spec().rules().pow_limit_bits(),
                ..Default::default()
            }
        };
        previous = block_hash(&header);
        rows.push(HistoryHeader::new(chain, at, header));
    }
    rows.reverse();
    let parent = rows.remove(0);
    let candidate = BlockHeader {
        version,
        previous_block_hash: parent.block_hash,
        time: parent.header.time + 600,
        bits: parent.header.bits,
        ..Default::default()
    };
    let maybe_seed = (chain == CanonicalChain::Mainnet && height > 227931 && height < 1983702)
        .then(|| HistoryHeader::new(chain, 227931, BlockHeader::default()));
    CandidateHistory::check_seeded(chain.spec().rules(), candidate, parent, rows, maybe_seed)
        .expect("complete synthetic branch; conditional evidence only")
}

fn spend(sequence: u32) -> Transaction {
    let mut transaction = coinbase(&[0x51, 0]);
    transaction.inputs[0].previous_output.vout = 0;
    transaction.inputs[0].sequence = sequence;
    transaction
}

fn inputs(height: u32) -> Vec<TransactionInputContext> {
    vec![TransactionInputContext {
        spent_output: SpentOutput {
            value: Amount::ZERO,
            script_pubkey: script(&[0x51]),
            is_coinbase: false,
        },
        created_height: height,
        created_median_time_past: -123,
    }]
}

#[test]
fn phase167_block_context_finality_uses_candidate_time_before_csv_and_parent_mtp_after() {
    for height in [419327, 419328, 419329] {
        // Arrange
        let history = history(CanonicalChain::Mainnet, height, 0, 4);
        let mut transaction = spend(0);
        transaction.lock_time = history.candidate().time - 1;
        // Act
        let context = TransactionValidationContext::from_candidate_history(
            &transaction,
            inputs(height - 1),
            &history,
        )
        .expect("derived context");
        // Assert
        assert_eq!(
            is_final_transaction(
                &transaction,
                height,
                context.block_time,
                &context.consensus_params
            ),
            height < 419328
        );
        assert!(!context.verify_flags.contains(ScriptVerifyFlags::CLEANSTACK));
        assert!(
            !context
                .verify_flags
                .contains(ScriptVerifyFlags::MINIMALDATA)
        );
    }
}

#[test]
fn phase167_block_context_finality_equality_requires_final_sequences() {
    // Arrange
    let history = history(CanonicalChain::Regtest, 20, 0, 4);
    let mut transaction = spend(0);
    transaction.lock_time = history.parent_median_time_past() as u32;
    let context =
        TransactionValidationContext::from_candidate_history(&transaction, inputs(19), &history)
            .expect("context");
    // Act / Assert
    assert!(!is_final_transaction(
        &transaction,
        20,
        context.block_time,
        &context.consensus_params
    ));
    transaction.inputs[0].sequence = TransactionInput::SEQUENCE_FINAL;
    assert!(is_final_transaction(
        &transaction,
        20,
        context.block_time,
        &context.consensus_params
    ));
    transaction.inputs[0].sequence = 0;
    transaction.lock_time = 20;
    assert!(!is_final_transaction(
        &transaction,
        20,
        context.block_time,
        &context.consensus_params
    ));
    transaction.lock_time = 19;
    assert!(is_final_transaction(
        &transaction,
        20,
        context.block_time,
        &context.consensus_params
    ));
}

#[test]
fn phase167_block_context_finality_follows_candidate_branch_not_another_tip() {
    // Arrange
    let history_a = history(CanonicalChain::Regtest, 20, 0, 4);
    let history_b = history(CanonicalChain::Regtest, 20, 100_000, 4);
    let mut transaction = spend(0);
    transaction.lock_time = history_a.parent_median_time_past() as u32;
    // Act / Assert
    for (history, expected) in [(&history_a, false), (&history_b, true)] {
        let context =
            TransactionValidationContext::from_candidate_history(&transaction, inputs(19), history)
                .expect("branch context");
        assert_eq!(
            is_final_transaction(
                &transaction,
                20,
                context.block_time,
                &context.consensus_params
            ),
            expected
        );
    }
}

#[test]
fn phase167_block_context_sequence_height_equality_version_flags_disable_and_mask() {
    // Arrange
    let history = history(CanonicalChain::Regtest, 20, 0, 4);
    let mut transaction = spend(2 | (1 << 20));
    let mut context =
        TransactionValidationContext::from_candidate_history(&transaction, inputs(19), &history)
            .expect("context");
    // Act / Assert
    assert_eq!(
        calculate_sequence_locks(&transaction, &context),
        Ok((20, -1))
    );
    assert_eq!(sequence_locks(&transaction, &context), Ok(false));
    transaction.inputs[0].sequence = 1;
    assert_eq!(sequence_locks(&transaction, &context), Ok(true));
    transaction.inputs[0].sequence = 2 | TransactionInput::SEQUENCE_LOCKTIME_DISABLE_FLAG;
    assert_eq!(
        calculate_sequence_locks(&transaction, &context),
        Ok((-1, -1))
    );
    transaction.inputs[0].sequence = 2;
    transaction.version = 1;
    assert_eq!(sequence_locks(&transaction, &context), Ok(true));
    transaction.version = 2;
    context.verify_flags = ScriptVerifyFlags::NONE;
    assert_eq!(sequence_locks(&transaction, &context), Ok(true));
}

#[test]
fn phase167_block_context_creating_time_comes_from_exact_branch_ancestor() {
    // Arrange
    let history_a = history(CanonicalChain::Regtest, 20, 0, 4);
    let history_b = history(CanonicalChain::Regtest, 20, 100_000, 4);
    let transaction = spend(TransactionInput::SEQUENCE_LOCKTIME_TYPE_FLAG | 1);
    // Act
    let context_a =
        TransactionValidationContext::from_candidate_history(&transaction, inputs(15), &history_a)
            .expect("a");
    let context_b =
        TransactionValidationContext::from_candidate_history(&transaction, inputs(15), &history_b)
            .expect("b");
    // Assert: MTP of height14 is height9's timestamp, not stored metadata or height15's MTP.
    assert_eq!(context_a.inputs[0].created_median_time_past, 1_600_005_400);
    assert_eq!(context_b.inputs[0].created_median_time_past, 1_600_105_400);
    assert_eq!(
        calculate_sequence_locks(&transaction, &context_a),
        Ok((-1, 1_600_005_911))
    );
    let mut equality = context_a;
    equality.median_time_past = 1_600_005_911;
    assert_eq!(sequence_locks(&transaction, &equality), Ok(false));
    equality.median_time_past += 1;
    assert_eq!(sequence_locks(&transaction, &equality), Ok(true));
}

#[test]
fn phase167_block_context_same_block_and_genesis_creating_times() {
    // Arrange
    let history = history(CanonicalChain::Regtest, 20, 0, 4);
    let transaction = spend(TransactionInput::SEQUENCE_LOCKTIME_TYPE_FLAG | 1);
    // Act
    let same_block =
        TransactionValidationContext::from_candidate_history(&transaction, inputs(20), &history)
            .expect("same block");
    let genesis =
        TransactionValidationContext::from_candidate_history(&transaction, inputs(0), &history)
            .expect("genesis");
    // Assert
    assert_eq!(
        same_block.inputs[0].created_median_time_past,
        history.parent_median_time_past()
    );
    assert_eq!(sequence_locks(&transaction, &same_block), Ok(false));
    assert_eq!(genesis.inputs[0].created_median_time_past, 1296688602);
}

#[test]
fn phase167_block_context_missing_creating_ancestor_refuses_without_stored_fallback() {
    // Arrange
    let history = history(CanonicalChain::Regtest, 301, 0, 4);
    let transaction = spend(TransactionInput::SEQUENCE_LOCKTIME_TYPE_FLAG | 1);
    // Act / Assert
    assert_eq!(
        TransactionValidationContext::from_candidate_history(&transaction, inputs(1), &history)
            .expect_err("missing old window")
            .reject_reason,
        "bad-txns-input-ancestor"
    );
    assert_eq!(
        TransactionValidationContext::from_candidate_history(&transaction, inputs(302), &history)
            .expect_err("future coin")
            .reject_reason,
        "bad-txns-input-height"
    );
    assert_eq!(
        TransactionValidationContext::from_candidate_history(&transaction, vec![], &history)
            .expect_err("missing coin")
            .reject_reason,
        "bad-txns-inputs-missingorspent"
    );
}

#[test]
fn phase167_block_context_time_arithmetic_is_checked() {
    // Arrange
    let history = history(CanonicalChain::Regtest, 20, 0, 4);
    let transaction = spend(TransactionInput::SEQUENCE_LOCKTIME_TYPE_FLAG | 1);
    let mut context =
        TransactionValidationContext::from_candidate_history(&transaction, inputs(15), &history)
            .expect("context");
    context.inputs[0].created_median_time_past = i64::MAX;
    // Act / Assert
    assert_eq!(
        calculate_sequence_locks(&transaction, &context)
            .expect_err("overflow")
            .reject_reason,
        "non-BIP68-final"
    );
}

fn selected_block(version: i32, bytes: &[u8]) -> (Block, CandidateHistory) {
    let chain = CanonicalChain::Regtest;
    let parent = HistoryHeader::new(chain, 0, chain.spec().genesis_header().clone());
    let mut block = Block {
        header: BlockHeader {
            version,
            previous_block_hash: parent.block_hash,
            time: parent.header.time + 1,
            bits: parent.header.bits,
            ..Default::default()
        },
        transactions: vec![coinbase(bytes)],
    };
    block.header.merkle_root = block_merkle_root(&block.transactions).expect("merkle").0;
    for nonce in 0..u32::MAX {
        block.header.nonce = nonce;
        if check_block_header(&block.header).is_ok() {
            break;
        }
    }
    let history =
        CandidateHistory::check(chain.spec().rules(), block.header.clone(), parent, vec![])
            .expect("history");
    (block, history)
}

#[test]
fn phase167_block_context_selected_body_uses_exact_versions_and_strict_height() {
    for (version, bytes, maybe_reason) in [
        (4, vec![0x51, 0], None),
        (3, vec![0x51, 0], Some("bad-version")),
        (4, vec![1, 1], Some("bad-cb-height")),
        (4, vec![0x52, 0], Some("bad-cb-height")),
        (4, vec![0x51], Some("bad-cb-length")),
    ] {
        // Arrange
        let (block, history) = selected_block(version, &bytes);
        // Act
        let result =
            check_block_with_candidate_history(&block, &history, i64::from(block.header.time));
        // Assert
        assert_eq!(result.err().map(|error| error.reject_reason), maybe_reason);
    }
}

#[test]
fn phase167_block_context_selected_body_refuses_foreign_candidate() {
    // Arrange
    let (mut block, history) = selected_block(4, &[0x51, 0]);
    block.header.time += 1;
    // Act / Assert
    assert_eq!(
        check_block_with_candidate_history(&block, &history, i64::from(block.header.time))
            .expect_err("different candidate")
            .reject_reason,
        "bad-candidate-context"
    );
}

#[test]
fn phase167_block_context_coinbase_script_length_retains_two_to_one_hundred() {
    for (length, maybe_reason) in [(2, None), (100, None), (101, Some("bad-cb-length"))] {
        // Arrange
        let mut bytes = vec![0; length];
        bytes[0] = 0x51;
        let (block, history) = selected_block(4, &bytes);
        // Act
        let result =
            check_block_with_candidate_history(&block, &history, i64::from(block.header.time));
        // Assert
        assert_eq!(result.err().map(|error| error.reject_reason), maybe_reason);
    }
}

#[test]
fn phase167_block_context_unused_creating_ancestor_is_not_required() {
    // Arrange
    let history = history(CanonicalChain::Regtest, 301, 0, 4);
    // Act / Assert
    for (version, sequence) in [
        (1, TransactionInput::SEQUENCE_LOCKTIME_TYPE_FLAG | 1),
        (
            2,
            TransactionInput::SEQUENCE_LOCKTIME_DISABLE_FLAG
                | TransactionInput::SEQUENCE_LOCKTIME_TYPE_FLAG
                | 1,
        ),
        (2, 1),
    ] {
        let mut transaction = spend(sequence);
        transaction.version = version;
        assert!(
            TransactionValidationContext::from_candidate_history(&transaction, inputs(1), &history)
                .is_ok()
        );
    }
}

fn committed_block() -> (Block, BlockValidationContext) {
    let (mut block, history) = selected_block(4, &[0x51, 0]);
    block.transactions[0].inputs[0].witness = ScriptWitness::new(vec![vec![9; 32]]);
    // A one-transaction block has a zero witness leaf, independently of the coinbase wtxid.
    let mut preimage = [0; 64];
    preimage[32..].fill(9);
    let mut commitment = vec![0x6a, 0x24, 0xaa, 0x21, 0xa9, 0xed];
    commitment.extend_from_slice(&crate::crypto::double_sha256(&preimage));
    block.transactions[0].outputs.push(TransactionOutput {
        value: Amount::ZERO,
        script_pubkey: script(&commitment),
    });
    (
        block,
        history.block_validation_context(i64::from(history.candidate().time)),
    )
}

#[test]
fn phase167_block_context_witness_coinbase_leaf_is_zero() {
    // Arrange
    let (block, context) = committed_block();
    // Act / Assert
    assert_eq!(
        block_witness_merkle_root(&block)
            .expect("root")
            .to_byte_array(),
        [0; 32]
    );
    assert_eq!(check_witness_commitment(&block, &context), Ok(()));
}

#[test]
fn phase167_block_context_witness_last_matching_output_wins_with_trailing_data() {
    // Arrange
    let (mut block, context) = committed_block();
    let valid = block.transactions[0].outputs[1].clone();
    let mut bytes = valid.script_pubkey.as_bytes().to_vec();
    bytes[6] ^= 1;
    block.transactions[0].outputs.push(TransactionOutput {
        value: Amount::ZERO,
        script_pubkey: script(&bytes),
    });
    // Act / Assert
    assert_eq!(
        check_witness_commitment(&block, &context)
            .expect_err("last mismatch")
            .reject_reason,
        "bad-witness-merkle-match"
    );
    bytes = valid.script_pubkey.as_bytes().to_vec();
    bytes.push(0xff);
    block.transactions[0].outputs.push(TransactionOutput {
        value: Amount::ZERO,
        script_pubkey: script(&bytes),
    });
    assert_eq!(witness_commitment_index(&block), Some(3));
    assert_eq!(check_witness_commitment(&block, &context), Ok(()));
}

#[test]
fn phase167_block_context_witness_truncated_commitment_is_not_a_match() {
    // Arrange
    let (mut block, context) = committed_block();
    let mut bytes = block.transactions[0].outputs[1]
        .script_pubkey
        .as_bytes()
        .to_vec();
    bytes.pop();
    block.transactions[0].outputs[1].script_pubkey = script(&bytes);
    // Act / Assert
    assert_eq!(witness_commitment_index(&block), None);
    assert_eq!(
        check_witness_commitment(&block, &context)
            .expect_err("no complete commitment")
            .reject_reason,
        "unexpected-witness"
    );
}

#[test]
fn phase167_block_context_witness_requires_exactly_one_32_byte_nonce() {
    for stack in [
        vec![],
        vec![vec![9; 31]],
        vec![vec![9; 33]],
        vec![vec![9; 32], vec![]],
    ] {
        // Arrange
        let (mut block, context) = committed_block();
        block.transactions[0].inputs[0].witness = ScriptWitness::new(stack);
        // Act / Assert
        assert_eq!(
            check_witness_commitment(&block, &context)
                .expect_err("nonce shape")
                .reject_reason,
            "bad-witness-nonce-size"
        );
    }
}

#[test]
fn phase167_block_context_witness_before_activation_is_unexpected() {
    // Arrange
    let (block, _) = committed_block();
    let history = history(CanonicalChain::Mainnet, 481823, 0, 4);
    let context = history.block_validation_context(i64::from(history.candidate().time));
    // Act / Assert: baseline WITNESS flag does not activate the commitment rules.
    assert!(
        history
            .rules()
            .script_flags()
            .contains(ScriptVerifyFlags::WITNESS)
    );
    assert_eq!(
        check_witness_commitment(&block, &context)
            .expect_err("preactivation witness")
            .reject_reason,
        "unexpected-witness"
    );
}

#[test]
fn phase167_block_context_witness_at_activation_uses_commitment_rules() {
    // Arrange
    let (block, _) = committed_block();
    let history = history(CanonicalChain::Mainnet, 481824, 0, 4);
    let context = history.block_validation_context(i64::from(history.candidate().time));
    // Act / Assert
    assert_eq!(check_witness_commitment(&block, &context), Ok(()));
}
