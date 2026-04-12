use open_bitcoin_codec::{
    CodecError, TransactionEncoding, encode_block_header, encode_transaction,
};
use open_bitcoin_primitives::{Block, BlockHeader};

use crate::MAX_BLOCK_SIGOPS_COST;
use crate::MAX_BLOCK_WEIGHT;
use crate::WITNESS_SCALE_FACTOR;
use crate::context::{
    BlockValidationContext, SpentOutput, TransactionValidationContext, is_final_transaction,
};
use crate::crypto::{
    block_hash, block_merkle_root, check_proof_of_work, double_sha256, transaction_txid,
    transaction_wtxid,
};
use crate::script::{count_legacy_sigops, count_p2sh_sigops, count_witness_sigops};
use crate::transaction::{
    check_transaction, validate_transaction, validate_transaction_with_context,
};
use crate::validation::{BlockValidationError, BlockValidationResult, block_error};

pub fn check_block_header(header: &BlockHeader) -> Result<(), BlockValidationError> {
    let valid =
        check_proof_of_work(block_hash(header).to_byte_array(), header.bits).map_err(|error| {
            block_error(
                BlockValidationResult::InvalidHeader,
                "bad-diffbits",
                Some(error.to_string()),
            )
        })?;
    if !valid {
        return Err(block_error(
            BlockValidationResult::InvalidHeader,
            "high-hash",
            Some("proof of work failed".to_string()),
        ));
    }

    Ok(())
}

pub fn check_block(block: &Block) -> Result<(), BlockValidationError> {
    check_block_header(&block.header)?;

    let (merkle_root, maybe_mutated) =
        block_merkle_root(&block.transactions).map_err(map_codec_error)?;
    if block.header.merkle_root != merkle_root {
        return Err(block_error(
            BlockValidationResult::Mutated,
            "bad-txnmrklroot",
            Some("hashMerkleRoot mismatch".to_string()),
        ));
    }
    if maybe_mutated {
        return Err(block_error(
            BlockValidationResult::Mutated,
            "bad-txns-duplicate",
            Some("duplicate transaction".to_string()),
        ));
    }

    if block.transactions.is_empty() {
        return Err(block_error(
            BlockValidationResult::Consensus,
            "bad-blk-length",
            Some("size limits failed".to_string()),
        ));
    }

    let stripped_size = serialized_block_size(block, false).map_err(map_codec_error)?;
    if block
        .transactions
        .len()
        .saturating_mul(WITNESS_SCALE_FACTOR)
        > MAX_BLOCK_WEIGHT
        || stripped_size.saturating_mul(WITNESS_SCALE_FACTOR) > MAX_BLOCK_WEIGHT
    {
        return Err(block_error(
            BlockValidationResult::Consensus,
            "bad-blk-length",
            Some("size limits failed".to_string()),
        ));
    }

    if !block.transactions[0].is_coinbase() {
        return Err(block_error(
            BlockValidationResult::Consensus,
            "bad-cb-missing",
            Some("first tx is not coinbase".to_string()),
        ));
    }
    if block
        .transactions
        .iter()
        .skip(1)
        .any(|transaction| transaction.is_coinbase())
    {
        return Err(block_error(
            BlockValidationResult::Consensus,
            "bad-cb-multiple",
            Some("more than one coinbase".to_string()),
        ));
    }

    for transaction in &block.transactions {
        check_transaction(transaction).map_err(|error| {
            block_error(
                BlockValidationResult::Consensus,
                error.reject_reason,
                error.debug_message.clone(),
            )
        })?;
    }

    let mut sigops = 0_usize;
    for transaction in &block.transactions {
        for input in &transaction.inputs {
            sigops += count_legacy_sigops(&input.script_sig).map_err(map_script_error)?;
        }
        for output in &transaction.outputs {
            sigops += count_legacy_sigops(&output.script_pubkey).map_err(map_script_error)?;
        }
    }
    enforce_sigop_cost_limit(sigops.saturating_mul(WITNESS_SCALE_FACTOR))?;

    Ok(())
}

pub fn validate_block(
    block: &Block,
    spent_outputs: &[Vec<SpentOutput>],
) -> Result<(), BlockValidationError> {
    check_block(block)?;

    if spent_outputs.len() != block.transactions.len().saturating_sub(1) {
        return Err(block_error(
            BlockValidationResult::MissingPrev,
            "bad-txns-inputs-missingorspent",
            None,
        ));
    }

    for (transaction, transaction_spent_outputs) in
        block.transactions.iter().skip(1).zip(spent_outputs)
    {
        validate_transaction(transaction, transaction_spent_outputs).map_err(|error| {
            let txid = format!(
                "{:?}",
                transaction_txid(transaction)
                    .expect("phase-2 typed transactions should serialize for txid logging")
                    .to_byte_array()
            );
            let maybe_debug_message = error.debug_message.clone().map_or_else(
                || format!("transaction {txid} failed validation"),
                |debug_message| format!("transaction {txid} failed validation: {debug_message}"),
            );
            block_error(
                BlockValidationResult::Consensus,
                error.reject_reason,
                Some(maybe_debug_message),
            )
        })?;
    }

    Ok(())
}

pub fn check_block_header_contextual(
    header: &BlockHeader,
    context: &BlockValidationContext,
) -> Result<(), BlockValidationError> {
    if i64::from(header.time) <= context.previous_median_time_past {
        return Err(block_error(
            BlockValidationResult::InvalidHeader,
            "time-too-old",
            Some("block's timestamp is too early".to_string()),
        ));
    }

    Ok(())
}

pub fn check_block_contextual(
    block: &Block,
    context: &BlockValidationContext,
) -> Result<(), BlockValidationError> {
    check_block(block)?;
    check_block_header_contextual(&block.header, context)?;

    let lock_time_cutoff = if context.consensus_params.enforce_bip113_median_time_past {
        context.previous_median_time_past
    } else {
        i64::from(block.header.time)
    };
    for transaction in &block.transactions {
        if !is_final_transaction(
            transaction,
            context.height,
            lock_time_cutoff,
            &context.consensus_params,
        ) {
            return Err(block_error(
                BlockValidationResult::Consensus,
                "bad-txns-nonfinal",
                Some("non-final transaction".to_string()),
            ));
        }
    }

    if context.consensus_params.enforce_bip34_height_in_coinbase
        && !coinbase_has_height_prefix(block, context.height)
    {
        return Err(block_error(
            BlockValidationResult::Consensus,
            "bad-cb-height",
            Some("block height mismatch in coinbase".to_string()),
        ));
    }

    check_witness_commitment(block, context)?;

    let stripped_size = serialized_block_size(block, false).map_err(map_codec_error)?;
    let total_size = serialized_block_size(block, true).map_err(map_codec_error)?;
    let weight = stripped_size
        .saturating_mul(WITNESS_SCALE_FACTOR - 1)
        .saturating_add(total_size);
    if weight > MAX_BLOCK_WEIGHT {
        return Err(block_error(
            BlockValidationResult::Consensus,
            "bad-blk-weight",
            Some("weight limit failed".to_string()),
        ));
    }

    Ok(())
}

pub fn validate_block_with_context(
    block: &Block,
    transaction_contexts: &[TransactionValidationContext],
    block_context: &BlockValidationContext,
) -> Result<(), BlockValidationError> {
    check_block_contextual(block, block_context)?;

    if transaction_contexts.len() != block.transactions.len().saturating_sub(1) {
        return Err(block_error(
            BlockValidationResult::MissingPrev,
            "bad-txns-inputs-missingorspent",
            None,
        ));
    }

    for (transaction, transaction_context) in
        block.transactions.iter().skip(1).zip(transaction_contexts)
    {
        validate_transaction_with_context(transaction, transaction_context).map_err(|error| {
            let txid = format!(
                "{:?}",
                transaction_txid(transaction)
                    .expect("phase-2 typed transactions should serialize for txid logging")
                    .to_byte_array()
            );
            let maybe_debug_message = error.debug_message.clone().map_or_else(
                || format!("transaction {txid} failed validation"),
                |debug_message| format!("transaction {txid} failed validation: {debug_message}"),
            );
            block_error(
                BlockValidationResult::Consensus,
                error.reject_reason,
                Some(maybe_debug_message),
            )
        })?;
    }

    let mut sigop_cost = 0_usize;
    for transaction in &block.transactions {
        sigop_cost = sigop_cost.saturating_add(legacy_sigop_cost(transaction)?);
    }
    for (transaction, transaction_context) in
        block.transactions.iter().skip(1).zip(transaction_contexts)
    {
        sigop_cost = sigop_cost.saturating_add(split_sigop_cost(transaction, transaction_context)?);
    }
    enforce_sigop_cost_limit(sigop_cost)?;

    Ok(())
}

fn legacy_sigop_cost(
    transaction: &open_bitcoin_primitives::Transaction,
) -> Result<usize, BlockValidationError> {
    let mut sigops = 0_usize;
    for input in &transaction.inputs {
        sigops = sigops
            .saturating_add(count_legacy_sigops(&input.script_sig).map_err(map_script_error)?);
    }
    for output in &transaction.outputs {
        sigops = sigops
            .saturating_add(count_legacy_sigops(&output.script_pubkey).map_err(map_script_error)?);
    }
    Ok(sigops.saturating_mul(WITNESS_SCALE_FACTOR))
}

fn split_sigop_cost(
    transaction: &open_bitcoin_primitives::Transaction,
    transaction_context: &TransactionValidationContext,
) -> Result<usize, BlockValidationError> {
    let mut sigops = 0_usize;
    for (input, input_context) in transaction.inputs.iter().zip(&transaction_context.inputs) {
        sigops = sigops.saturating_add(
            count_p2sh_sigops(&input.script_sig, &input_context.spent_output.script_pubkey)
                .map_err(map_script_error)?
                .saturating_mul(WITNESS_SCALE_FACTOR),
        );
        sigops = sigops.saturating_add(
            count_witness_sigops(
                &input.script_sig,
                &input_context.spent_output.script_pubkey,
                &input.witness,
                transaction_context.verify_flags,
            )
            .map_err(map_script_error)?,
        );
    }
    Ok(sigops)
}

fn serialized_block_size(block: &Block, include_witness: bool) -> Result<usize, CodecError> {
    let mut size = encode_block_header(&block.header).len();
    size += compact_size_len(block.transactions.len() as u64);
    for transaction in &block.transactions {
        let encoding = if include_witness {
            TransactionEncoding::WithWitness
        } else {
            TransactionEncoding::WithoutWitness
        };
        size += encode_transaction(transaction, encoding)?.len();
    }
    Ok(size)
}

fn compact_size_len(value: u64) -> usize {
    match value {
        0..=252 => 1,
        253..=0xffff => 3,
        0x1_0000..=0xffff_ffff => 5,
        _ => 9,
    }
}

fn check_witness_commitment(
    block: &Block,
    context: &BlockValidationContext,
) -> Result<(), BlockValidationError> {
    let maybe_commitment_index = witness_commitment_index(block);
    if context.consensus_params.enforce_segwit
        && let Some(commitment_index) = maybe_commitment_index
    {
        let coinbase_input = block
            .transactions
            .first()
            .and_then(|transaction| transaction.inputs.first())
            .expect("coinbase input must exist after context-free block checks");
        if coinbase_input.witness.stack().len() != 1
            || coinbase_input.witness.stack()[0].len() != 32
        {
            return Err(block_error(
                BlockValidationResult::Mutated,
                "bad-witness-nonce-size",
                Some("invalid witness reserved value size".to_string()),
            ));
        }

        let witness_root = block_witness_merkle_root(block).map_err(map_codec_error)?;
        let reserved_value = &coinbase_input.witness.stack()[0];
        let mut commitment_preimage = [0_u8; 64];
        commitment_preimage[..32].copy_from_slice(witness_root.as_bytes());
        commitment_preimage[32..].copy_from_slice(reserved_value);
        let expected_commitment = double_sha256(&commitment_preimage);

        let commitment_script = block.transactions[0].outputs[commitment_index]
            .script_pubkey
            .as_bytes();
        if commitment_script[6..38] != expected_commitment {
            return Err(block_error(
                BlockValidationResult::Mutated,
                "bad-witness-merkle-match",
                Some("witness merkle commitment mismatch".to_string()),
            ));
        }

        return Ok(());
    }

    if block
        .transactions
        .iter()
        .any(|transaction| transaction.has_witness())
    {
        return Err(block_error(
            BlockValidationResult::Mutated,
            "unexpected-witness",
            Some("unexpected witness data found".to_string()),
        ));
    }

    Ok(())
}

fn witness_commitment_index(block: &Block) -> Option<usize> {
    let coinbase_outputs = block.transactions.first()?.outputs.iter().enumerate();
    let mut maybe_commitment_index = None;
    for (index, output) in coinbase_outputs {
        let bytes = output.script_pubkey.as_bytes();
        if bytes.len() >= 38
            && bytes[0] == 0x6a
            && bytes[1] == 0x24
            && bytes[2] == 0xaa
            && bytes[3] == 0x21
            && bytes[4] == 0xa9
            && bytes[5] == 0xed
        {
            maybe_commitment_index = Some(index);
        }
    }
    maybe_commitment_index
}

fn block_witness_merkle_root(
    block: &Block,
) -> Result<open_bitcoin_primitives::MerkleRoot, CodecError> {
    if block.transactions.is_empty() {
        return Ok(open_bitcoin_primitives::MerkleRoot::from_byte_array(
            [0_u8; 32],
        ));
    }

    let mut level = Vec::with_capacity(block.transactions.len());
    level.push([0_u8; 32]);
    for transaction in block.transactions.iter().skip(1) {
        level.push(transaction_wtxid(transaction)?.to_byte_array());
    }

    while level.len() > 1 {
        if level.len() % 2 == 1 {
            let last_hash = *level.last().expect("non-empty merkle level");
            level.push(last_hash);
        }

        let mut next_level = Vec::with_capacity(level.len() / 2);
        for pair in level.chunks_exact(2) {
            let mut concatenated = [0_u8; 64];
            concatenated[..32].copy_from_slice(&pair[0]);
            concatenated[32..].copy_from_slice(&pair[1]);
            next_level.push(double_sha256(&concatenated));
        }
        level = next_level;
    }

    Ok(open_bitcoin_primitives::MerkleRoot::from_byte_array(
        level[0],
    ))
}

fn coinbase_has_height_prefix(block: &Block, height: u32) -> bool {
    let Some(coinbase_input) = block
        .transactions
        .first()
        .and_then(|transaction| transaction.inputs.first())
    else {
        return false;
    };
    let expected_prefix = serialized_script_num(height as i64);
    let script_sig = coinbase_input.script_sig.as_bytes();

    script_sig.len() >= expected_prefix.len()
        && script_sig[..expected_prefix.len()] == expected_prefix
}

fn serialized_script_num(value: i64) -> Vec<u8> {
    if value == 0 {
        return vec![0x00];
    }

    let negative = value < 0;
    let mut magnitude = value.unsigned_abs();
    let mut encoded = Vec::new();
    while magnitude > 0 {
        encoded.push((magnitude & 0xff) as u8);
        magnitude >>= 8;
    }

    if encoded.last().is_some_and(|byte| (byte & 0x80) != 0) {
        encoded.push(if negative { 0x80 } else { 0x00 });
    } else if negative {
        let last = encoded.last_mut().expect("non-empty because value != 0");
        *last |= 0x80;
    }

    let mut script = Vec::with_capacity(encoded.len() + 1);
    script.push(encoded.len() as u8);
    script.extend(encoded);
    script
}

fn map_codec_error(error: CodecError) -> BlockValidationError {
    block_error(
        BlockValidationResult::Consensus,
        "bad-blk-serialization",
        Some(error.to_string()),
    )
}

fn map_script_error(error: crate::script::ScriptError) -> BlockValidationError {
    block_error(
        BlockValidationResult::Consensus,
        "bad-blk-script",
        Some(error.to_string()),
    )
}

fn block_sigop_overflow() -> BlockValidationError {
    block_error(
        BlockValidationResult::Consensus,
        "bad-blk-sigops",
        Some("out-of-bounds SigOpCount".to_string()),
    )
}

fn enforce_sigop_cost_limit(sigop_cost: usize) -> Result<(), BlockValidationError> {
    if sigop_cost > MAX_BLOCK_SIGOPS_COST {
        return Err(block_sigop_overflow());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use open_bitcoin_codec::parse_block_header;
    use open_bitcoin_primitives::{
        Amount, Block, BlockHash, BlockHeader, MerkleRoot, OutPoint, ScriptBuf, ScriptWitness,
        Transaction, TransactionInput, TransactionOutput, Txid,
    };

    use super::{
        block_sigop_overflow, block_witness_merkle_root, check_block, check_block_contextual,
        check_block_header, check_block_header_contextual, coinbase_has_height_prefix,
        compact_size_len, enforce_sigop_cost_limit, legacy_sigop_cost, map_codec_error,
        map_script_error, serialized_block_size, serialized_script_num, split_sigop_cost,
        validate_block, validate_block_with_context, witness_commitment_index,
    };
    use crate::MAX_BLOCK_SIGOPS_COST;
    use crate::context::{
        BlockValidationContext, ConsensusParams, ScriptVerifyFlags, SpentOutput,
        TransactionInputContext, TransactionValidationContext,
    };
    use crate::crypto::{block_hash, block_merkle_root};

    const EASY_BITS: u32 = 0x207f_ffff;
    const GENESIS_BLOCK_HEADER_HEX: &str =
        include_str!("../../open-bitcoin-codec/testdata/block_header.hex");

    fn decode_hex(input: &str) -> Vec<u8> {
        let trimmed = input.trim();
        let mut bytes = Vec::with_capacity(trimmed.len() / 2);
        let chars: Vec<char> = trimmed.chars().collect();
        for pair in chars.chunks(2) {
            let high = pair[0].to_digit(16).expect("hex fixture");
            let low = pair[1].to_digit(16).expect("hex fixture");
            bytes.push(((high << 4) | low) as u8);
        }
        bytes
    }

    fn script(bytes: &[u8]) -> ScriptBuf {
        ScriptBuf::from_bytes(bytes.to_vec()).expect("valid script")
    }

    fn coinbase_transaction() -> Transaction {
        Transaction {
            version: 1,
            inputs: vec![TransactionInput {
                previous_output: OutPoint::null(),
                script_sig: script(&[0x01, 0x01, 0x51]),
                sequence: TransactionInput::SEQUENCE_FINAL,
                witness: ScriptWitness::default(),
            }],
            outputs: vec![TransactionOutput {
                value: Amount::from_sats(50).expect("valid amount"),
                script_pubkey: script(&[0x52]),
            }],
            lock_time: 0,
        }
    }

    fn spend_transaction(previous_txid: Txid) -> Transaction {
        Transaction {
            version: 1,
            inputs: vec![TransactionInput {
                previous_output: OutPoint {
                    txid: previous_txid,
                    vout: 0,
                },
                script_sig: script(&[0x52]),
                sequence: TransactionInput::SEQUENCE_FINAL,
                witness: ScriptWitness::default(),
            }],
            outputs: vec![TransactionOutput {
                value: Amount::from_sats(40).expect("valid amount"),
                script_pubkey: script(&[0x51]),
            }],
            lock_time: 0,
        }
    }

    fn mine_header(block: &mut Block) {
        block.header.nonce = (0..=u32::MAX)
            .find(|nonce| {
                block.header.nonce = *nonce;
                check_block_header(&block.header).is_ok()
            })
            .expect("expected to find a nonce for easy regtest target");
    }

    fn valid_block() -> (Block, Vec<Vec<SpentOutput>>) {
        let coinbase = coinbase_transaction();
        let coinbase_txid = crate::crypto::transaction_txid(&coinbase).expect("coinbase txid");
        let spend = spend_transaction(coinbase_txid);
        let transactions = vec![coinbase.clone(), spend.clone()];
        let (merkle_root, maybe_mutated) = block_merkle_root(&transactions).expect("merkle root");
        assert!(!maybe_mutated);

        let mut block = Block {
            header: BlockHeader {
                version: 1,
                previous_block_hash: BlockHash::from_byte_array([0_u8; 32]),
                merkle_root,
                time: 1_231_006_505,
                bits: EASY_BITS,
                nonce: 0,
            },
            transactions,
        };
        mine_header(&mut block);

        let spent_outputs = vec![vec![SpentOutput {
            value: coinbase.outputs[0].value,
            script_pubkey: coinbase.outputs[0].script_pubkey.clone(),
            is_coinbase: true,
        }]];

        (block, spent_outputs)
    }

    fn p2sh_sigop_heavy_redeem_script(sigops: usize) -> ScriptBuf {
        let mut bytes = Vec::with_capacity(sigops + 4);
        bytes.push(0x00);
        bytes.push(0x63);
        bytes.extend(std::iter::repeat_n(0xac, sigops));
        bytes.push(0x68);
        bytes.push(0x51);
        script(&bytes)
    }

    fn p2sh_sigop_heavy_transaction(
        txid_byte: u8,
        sigops: usize,
    ) -> (Transaction, TransactionValidationContext) {
        let redeem_script = p2sh_sigop_heavy_redeem_script(sigops);
        let redeem_hash = crate::crypto::hash160(redeem_script.as_bytes());
        let script_pubkey = {
            let mut bytes = vec![0xa9, 20];
            bytes.extend_from_slice(&redeem_hash);
            bytes.push(0x87);
            script(&bytes)
        };
        let script_sig = {
            let mut bytes = vec![redeem_script.as_bytes().len() as u8];
            bytes.extend_from_slice(redeem_script.as_bytes());
            script(&bytes)
        };
        let transaction = Transaction {
            version: 1,
            inputs: vec![TransactionInput {
                previous_output: OutPoint {
                    txid: Txid::from_byte_array([txid_byte; 32]),
                    vout: 0,
                },
                script_sig,
                sequence: TransactionInput::SEQUENCE_FINAL,
                witness: ScriptWitness::default(),
            }],
            outputs: vec![TransactionOutput {
                value: Amount::from_sats(40).expect("valid amount"),
                script_pubkey: script(&[0x51]),
            }],
            lock_time: 0,
        };
        let context = TransactionValidationContext {
            inputs: vec![TransactionInputContext {
                spent_output: SpentOutput {
                    value: Amount::from_sats(50).expect("valid amount"),
                    script_pubkey,
                    is_coinbase: false,
                },
                created_height: 0,
                created_median_time_past: 0,
            }],
            spend_height: 1,
            block_time: 1,
            median_time_past: 1,
            verify_flags: ScriptVerifyFlags::P2SH,
            consensus_params: ConsensusParams::default(),
        };
        (transaction, context)
    }

    #[test]
    fn genesis_header_fixture_passes_pow_check() {
        let header = parse_block_header(&decode_hex(GENESIS_BLOCK_HEADER_HEX))
            .expect("genesis header fixture should parse");

        assert_eq!(check_block_header(&header), Ok(()));
    }

    #[test]
    fn check_block_accepts_mined_block() {
        let (block, _) = valid_block();

        assert_eq!(check_block(&block), Ok(()));
    }

    #[test]
    fn validate_block_accepts_matching_spent_outputs() {
        let (block, spent_outputs) = valid_block();

        assert_eq!(validate_block(&block, &spent_outputs), Ok(()));
    }

    #[test]
    fn check_block_rejects_bad_merkle_root() {
        let (mut block, _) = valid_block();
        block.header.merkle_root = MerkleRoot::from_byte_array([9_u8; 32]);
        mine_header(&mut block);

        let error = check_block(&block).expect_err("bad merkle root must fail");

        assert_eq!(error.reject_reason, "bad-txnmrklroot");
    }

    #[test]
    fn check_block_rejects_missing_coinbase() {
        let (mut block, spent_outputs) = valid_block();
        block.transactions.swap(0, 1);
        let (merkle_root, _) = block_merkle_root(&block.transactions).expect("merkle root");
        block.header.merkle_root = merkle_root;
        mine_header(&mut block);

        let error = check_block(&block).expect_err("missing coinbase must fail");

        assert_eq!(error.reject_reason, "bad-cb-missing");
        assert_eq!(spent_outputs.len(), 1);
    }

    #[test]
    fn check_block_rejects_multiple_coinbases() {
        let mut second_coinbase = coinbase_transaction();
        second_coinbase.inputs[0].script_sig = script(&[0x01, 0x02, 0x52]);
        let mut block = Block {
            header: BlockHeader {
                version: 1,
                previous_block_hash: BlockHash::from_byte_array([0_u8; 32]),
                merkle_root: MerkleRoot::from_byte_array([0_u8; 32]),
                time: 1_231_006_505,
                bits: EASY_BITS,
                nonce: 0,
            },
            transactions: vec![coinbase_transaction(), second_coinbase],
        };
        let (merkle_root, _) = block_merkle_root(&block.transactions).expect("merkle root");
        block.header.merkle_root = merkle_root;
        mine_header(&mut block);

        let error = check_block(&block).expect_err("multiple coinbases must fail");

        assert_eq!(error.reject_reason, "bad-cb-multiple");
    }

    #[test]
    fn check_block_rejects_duplicate_transactions_even_with_matching_root() {
        let coinbase = coinbase_transaction();
        let coinbase_txid = crate::crypto::transaction_txid(&coinbase).expect("coinbase txid");
        let spend = spend_transaction(coinbase_txid);
        let mut block = Block {
            header: BlockHeader {
                version: 1,
                previous_block_hash: BlockHash::from_byte_array([0_u8; 32]),
                merkle_root: MerkleRoot::from_byte_array([0_u8; 32]),
                time: 1_231_006_505,
                bits: EASY_BITS,
                nonce: 0,
            },
            transactions: vec![coinbase, spend.clone(), spend],
        };
        let (merkle_root, maybe_mutated) =
            block_merkle_root(&block.transactions).expect("merkle root");
        assert!(maybe_mutated);
        block.header.merkle_root = merkle_root;
        mine_header(&mut block);

        let error = check_block(&block).expect_err("mutated merkle tree must fail");

        assert_eq!(error.reject_reason, "bad-txns-duplicate");
    }

    #[test]
    fn validate_block_rejects_mismatched_spent_output_scripts() {
        let (block, mut spent_outputs) = valid_block();
        spent_outputs[0][0].script_pubkey = script(&[0x53, 0x87]);

        let error = validate_block(&block, &spent_outputs)
            .expect_err("mismatched prevout script must fail");

        assert_eq!(error.reject_reason, "mandatory-script-verify-flag-failed");
    }

    #[test]
    fn mined_block_hash_meets_easy_target() {
        let (block, _) = valid_block();
        let hash = block_hash(&block.header);

        assert_ne!(hash.to_byte_array(), [0_u8; 32]);
    }

    #[test]
    fn check_block_header_rejects_invalid_bits() {
        let mut header = parse_block_header(&decode_hex(GENESIS_BLOCK_HEADER_HEX))
            .expect("genesis header fixture should parse");
        header.bits = 0x0180_0000;

        assert_eq!(
            check_block_header(&header)
                .expect_err("invalid bits must fail")
                .reject_reason,
            "bad-diffbits",
        );
    }

    #[test]
    fn check_block_rejects_empty_blocks_and_oversized_blocks() {
        let mut empty_block = Block {
            header: BlockHeader {
                version: 1,
                previous_block_hash: BlockHash::from_byte_array([0_u8; 32]),
                merkle_root: MerkleRoot::from_byte_array([0_u8; 32]),
                time: 1_231_006_505,
                bits: EASY_BITS,
                nonce: 0,
            },
            transactions: vec![],
        };
        mine_header(&mut empty_block);
        assert_eq!(
            check_block(&empty_block)
                .expect_err("empty block must fail")
                .reject_reason,
            "bad-blk-length",
        );

        let big_script = script(&vec![0x51; 10_000]);
        let mut huge_block = Block {
            header: BlockHeader {
                version: 1,
                previous_block_hash: BlockHash::from_byte_array([0_u8; 32]),
                merkle_root: MerkleRoot::from_byte_array([0_u8; 32]),
                time: 1_231_006_505,
                bits: EASY_BITS,
                nonce: 0,
            },
            transactions: vec![Transaction {
                version: 1,
                inputs: vec![TransactionInput {
                    previous_output: OutPoint::null(),
                    script_sig: script(&[0x01, 0x01]),
                    sequence: TransactionInput::SEQUENCE_FINAL,
                    witness: ScriptWitness::default(),
                }],
                outputs: (0..101)
                    .map(|_| TransactionOutput {
                        value: Amount::from_sats(1).expect("valid amount"),
                        script_pubkey: big_script.clone(),
                    })
                    .collect(),
                lock_time: 0,
            }],
        };
        let (merkle_root, _) = block_merkle_root(&huge_block.transactions).expect("merkle root");
        huge_block.header.merkle_root = merkle_root;
        mine_header(&mut huge_block);

        assert_eq!(
            check_block(&huge_block)
                .expect_err("oversized block must fail")
                .reject_reason,
            "bad-blk-length",
        );
    }

    #[test]
    fn check_block_maps_transaction_and_sigop_failures() {
        let coinbase = coinbase_transaction();
        let mut invalid_tx =
            spend_transaction(crate::crypto::transaction_txid(&coinbase).expect("coinbase txid"));
        invalid_tx.inputs.push(invalid_tx.inputs[0].clone());
        let mut block = Block {
            header: BlockHeader {
                version: 1,
                previous_block_hash: BlockHash::from_byte_array([0_u8; 32]),
                merkle_root: MerkleRoot::from_byte_array([0_u8; 32]),
                time: 1_231_006_505,
                bits: EASY_BITS,
                nonce: 0,
            },
            transactions: vec![coinbase.clone(), invalid_tx],
        };
        let (merkle_root, _) = block_merkle_root(&block.transactions).expect("merkle root");
        block.header.merkle_root = merkle_root;
        mine_header(&mut block);

        assert_eq!(
            check_block(&block)
                .expect_err("invalid transaction must fail")
                .reject_reason,
            "bad-txns-inputs-duplicate",
        );

        let mut sigops_block = Block {
            header: BlockHeader {
                version: 1,
                previous_block_hash: BlockHash::from_byte_array([0_u8; 32]),
                merkle_root: MerkleRoot::from_byte_array([0_u8; 32]),
                time: 1_231_006_505,
                bits: EASY_BITS,
                nonce: 0,
            },
            transactions: vec![Transaction {
                version: 1,
                inputs: vec![TransactionInput {
                    previous_output: OutPoint::null(),
                    script_sig: script(&[0x01, 0x01]),
                    sequence: TransactionInput::SEQUENCE_FINAL,
                    witness: ScriptWitness::default(),
                }],
                outputs: vec![TransactionOutput {
                    value: Amount::from_sats(1).expect("valid amount"),
                    script_pubkey: script(&vec![0xae; 1_001]),
                }],
                lock_time: 0,
            }],
        };
        let (merkle_root, _) = block_merkle_root(&sigops_block.transactions).expect("merkle root");
        sigops_block.header.merkle_root = merkle_root;
        mine_header(&mut sigops_block);

        assert_eq!(
            check_block(&sigops_block)
                .expect_err("sigops overflow must fail")
                .reject_reason,
            "bad-blk-sigops",
        );
    }

    #[test]
    fn validate_block_rejects_missing_prev_groups_and_uses_default_debug_message() {
        let (block, _) = valid_block();
        assert_eq!(
            validate_block(&block, &[])
                .expect_err("missing prev groups must fail")
                .reject_reason,
            "bad-txns-inputs-missingorspent",
        );

        let coinbase = coinbase_transaction();
        let coinbase_txid = crate::crypto::transaction_txid(&coinbase).expect("coinbase txid");
        let mut spend = spend_transaction(coinbase_txid);
        spend.inputs.push(TransactionInput {
            previous_output: OutPoint {
                txid: Txid::from_byte_array([9_u8; 32]),
                vout: 0,
            },
            script_sig: script(&[0x52]),
            sequence: TransactionInput::SEQUENCE_FINAL,
            witness: ScriptWitness::default(),
        });
        let mut block = Block {
            header: BlockHeader {
                version: 1,
                previous_block_hash: BlockHash::from_byte_array([0_u8; 32]),
                merkle_root: MerkleRoot::from_byte_array([0_u8; 32]),
                time: 1_231_006_505,
                bits: EASY_BITS,
                nonce: 0,
            },
            transactions: vec![coinbase, spend],
        };
        let (merkle_root, _) = block_merkle_root(&block.transactions).expect("merkle root");
        block.header.merkle_root = merkle_root;
        mine_header(&mut block);

        let error = validate_block(
            &block,
            &[vec![SpentOutput {
                value: Amount::from_sats(50).expect("valid amount"),
                script_pubkey: script(&[0x52, 0x87]),
                is_coinbase: true,
            }]],
        )
        .expect_err("missing inputs inside transaction must fail");

        assert_eq!(error.reject_reason, "bad-txns-inputs-missingorspent");
        assert!(
            error
                .debug_message
                .expect("debug message")
                .contains("failed validation")
        );
    }

    #[test]
    fn helper_functions_cover_serialization_and_mapping_paths() {
        let (block, _) = valid_block();
        let without_witness = serialized_block_size(&block, false).expect("size without witness");
        let with_witness = serialized_block_size(&block, true).expect("size with witness");

        assert!(with_witness >= without_witness);
        assert_eq!(compact_size_len(252), 1);
        assert_eq!(compact_size_len(253), 3);
        assert_eq!(compact_size_len(65_536), 5);
        assert_eq!(compact_size_len(u64::MAX), 9);
        assert_eq!(
            map_codec_error(open_bitcoin_codec::CodecError::UnexpectedEof {
                needed: 1,
                remaining: 0,
            })
            .reject_reason,
            "bad-blk-serialization",
        );
        assert_eq!(
            map_script_error(crate::script::ScriptError::BadOpcode).reject_reason,
            "bad-blk-script",
        );
    }

    #[test]
    fn contextual_block_checks_cover_time_height_and_context_mapping() {
        let (block, spent_outputs) = valid_block();
        let context = BlockValidationContext {
            height: 1,
            previous_header: BlockHeader {
                time: block.header.time - 1,
                ..BlockHeader::default()
            },
            previous_median_time_past: i64::from(block.header.time) - 1,
            consensus_params: ConsensusParams {
                enforce_segwit: false,
                ..Default::default()
            },
        };

        assert_eq!(
            check_block_header_contextual(&block.header, &context),
            Ok(())
        );
        assert_eq!(check_block_contextual(&block, &context), Ok(()));

        let tx_contexts = vec![TransactionValidationContext {
            inputs: vec![TransactionInputContext {
                spent_output: SpentOutput {
                    is_coinbase: false,
                    ..spent_outputs[0][0].clone()
                },
                created_height: 0,
                created_median_time_past: 0,
            }],
            spend_height: 1,
            block_time: i64::from(block.header.time),
            median_time_past: i64::from(block.header.time) - 1,
            verify_flags: ScriptVerifyFlags::NONE,
            consensus_params: context.consensus_params,
        }];
        assert_eq!(
            validate_block_with_context(&block, &tx_contexts, &context),
            Ok(())
        );

        let stale_context = BlockValidationContext {
            previous_median_time_past: i64::from(block.header.time),
            ..context.clone()
        };
        assert_eq!(
            check_block_header_contextual(&block.header, &stale_context)
                .expect_err("time-too-old must fail")
                .reject_reason,
            "time-too-old",
        );

        assert_eq!(
            validate_block_with_context(&block, &[], &context)
                .expect_err("missing contexts must fail")
                .reject_reason,
            "bad-txns-inputs-missingorspent",
        );

        let mut nonfinal_block = block.clone();
        nonfinal_block.transactions[1].lock_time = 2;
        nonfinal_block.transactions[1].inputs[0].sequence = 0;
        let (merkle_root, _) =
            block_merkle_root(&nonfinal_block.transactions).expect("merkle root");
        nonfinal_block.header.merkle_root = merkle_root;
        mine_header(&mut nonfinal_block);

        assert_eq!(
            check_block_contextual(&nonfinal_block, &context)
                .expect_err("non-final tx must fail")
                .reject_reason,
            "bad-txns-nonfinal",
        );
    }

    #[test]
    fn witness_commitment_and_coinbase_height_paths_are_exercised() {
        let mut coinbase = coinbase_transaction();
        coinbase.inputs[0].witness = ScriptWitness::new(vec![vec![9_u8; 32]]);
        let coinbase_txid = crate::crypto::transaction_txid(&coinbase).expect("coinbase txid");
        let mut spend = spend_transaction(coinbase_txid);
        spend.inputs[0].witness = ScriptWitness::new(vec![vec![0x01]]);

        let mut block = Block {
            header: BlockHeader {
                version: 1,
                previous_block_hash: BlockHash::from_byte_array([0_u8; 32]),
                merkle_root: MerkleRoot::from_byte_array([0_u8; 32]),
                time: 1_231_006_505,
                bits: EASY_BITS,
                nonce: 0,
            },
            transactions: vec![coinbase.clone(), spend.clone()],
        };

        let witness_root = block_witness_merkle_root(&block).expect("witness root");
        let mut commitment_preimage = [0_u8; 64];
        commitment_preimage[..32].copy_from_slice(witness_root.as_bytes());
        commitment_preimage[32..].copy_from_slice(&coinbase.inputs[0].witness.stack()[0]);
        let commitment = crate::crypto::double_sha256(&commitment_preimage);
        block.transactions[0].outputs.push(TransactionOutput {
            value: Amount::from_sats(0).expect("zero amount"),
            script_pubkey: script(
                &[&[0x6a, 0x24, 0xaa, 0x21, 0xa9, 0xed][..], &commitment[..]].concat(),
            ),
        });
        let (merkle_root, _) = block_merkle_root(&block.transactions).expect("merkle root");
        block.header.merkle_root = merkle_root;
        mine_header(&mut block);

        let context = BlockValidationContext {
            height: 1,
            previous_header: BlockHeader {
                time: block.header.time - 1,
                ..BlockHeader::default()
            },
            previous_median_time_past: i64::from(block.header.time) - 1,
            consensus_params: ConsensusParams::default(),
        };

        assert_eq!(witness_commitment_index(&block), Some(1));
        assert_eq!(check_block_contextual(&block, &context), Ok(()));

        let bad_height_context = BlockValidationContext {
            height: 2,
            ..context.clone()
        };
        assert_eq!(
            check_block_contextual(&block, &bad_height_context)
                .expect_err("coinbase height mismatch must fail")
                .reject_reason,
            "bad-cb-height",
        );

        let mut bad_commitment_block = block.clone();
        bad_commitment_block.transactions[0].outputs[1].script_pubkey =
            script(&[&[0x6a, 0x24, 0xaa, 0x21, 0xa9, 0xed][..], &[7_u8; 32][..]].concat());
        let (bad_merkle_root, _) =
            block_merkle_root(&bad_commitment_block.transactions).expect("merkle root");
        bad_commitment_block.header.merkle_root = bad_merkle_root;
        mine_header(&mut bad_commitment_block);
        assert_eq!(
            check_block_contextual(&bad_commitment_block, &context)
                .expect_err("bad witness commitment must fail")
                .reject_reason,
            "bad-witness-merkle-match",
        );

        let no_witness_context = BlockValidationContext {
            consensus_params: ConsensusParams {
                enforce_segwit: false,
                ..Default::default()
            },
            ..context
        };
        assert_eq!(
            check_block_contextual(&block, &no_witness_context)
                .expect_err("unexpected witness must fail")
                .reject_reason,
            "unexpected-witness",
        );
    }

    #[test]
    fn contextual_helpers_cover_merkle_height_and_weight_edges() {
        let empty_block = Block {
            header: BlockHeader::default(),
            transactions: vec![],
        };
        assert_eq!(
            block_witness_merkle_root(&empty_block)
                .expect("empty witness merkle root")
                .to_byte_array(),
            [0_u8; 32],
        );
        assert!(!coinbase_has_height_prefix(&empty_block, 0));
        assert_eq!(serialized_script_num(0), vec![0x00]);
        assert_eq!(serialized_script_num(128), vec![0x02, 0x80, 0x00]);
        assert_eq!(serialized_script_num(-1), vec![0x01, 0x81]);

        let coinbase = coinbase_transaction();
        let coinbase_txid = crate::crypto::transaction_txid(&coinbase).expect("coinbase txid");
        let odd_block = Block {
            header: BlockHeader::default(),
            transactions: vec![
                coinbase.clone(),
                spend_transaction(coinbase_txid),
                spend_transaction(coinbase_txid),
            ],
        };
        let odd_root = block_witness_merkle_root(&odd_block).expect("odd witness merkle root");
        assert_ne!(odd_root.to_byte_array(), [0_u8; 32]);

        let mut witness_coinbase = coinbase_transaction();
        witness_coinbase.inputs[0].witness = ScriptWitness::new(vec![vec![0_u8; 1]]);
        let witness_coinbase_txid =
            crate::crypto::transaction_txid(&witness_coinbase).expect("coinbase txid");
        let mut witness_spend = spend_transaction(witness_coinbase_txid);
        witness_spend.inputs[0].witness = ScriptWitness::new(vec![vec![0_u8; 4_100_000]]);

        let mut heavy_block = Block {
            header: BlockHeader {
                version: 1,
                previous_block_hash: BlockHash::from_byte_array([0_u8; 32]),
                merkle_root: MerkleRoot::from_byte_array([0_u8; 32]),
                time: 1_231_006_505,
                bits: EASY_BITS,
                nonce: 0,
            },
            transactions: vec![witness_coinbase.clone(), witness_spend],
        };
        heavy_block.transactions[0].inputs[0].witness = ScriptWitness::new(vec![vec![9_u8; 32]]);
        let witness_root = block_witness_merkle_root(&heavy_block).expect("witness root");
        let mut commitment_preimage = [0_u8; 64];
        commitment_preimage[..32].copy_from_slice(witness_root.as_bytes());
        commitment_preimage[32..]
            .copy_from_slice(&heavy_block.transactions[0].inputs[0].witness.stack()[0]);
        let commitment = crate::crypto::double_sha256(&commitment_preimage);
        heavy_block.transactions[0].outputs.push(TransactionOutput {
            value: Amount::from_sats(0).expect("zero amount"),
            script_pubkey: script(
                &[&[0x6a, 0x24, 0xaa, 0x21, 0xa9, 0xed][..], &commitment[..]].concat(),
            ),
        });
        let (heavy_merkle_root, _) =
            block_merkle_root(&heavy_block.transactions).expect("merkle root");
        heavy_block.header.merkle_root = heavy_merkle_root;
        mine_header(&mut heavy_block);

        let no_mtp_context = BlockValidationContext {
            height: 1,
            previous_header: BlockHeader {
                time: heavy_block.header.time - 1,
                ..BlockHeader::default()
            },
            previous_median_time_past: i64::from(heavy_block.header.time) - 10,
            consensus_params: ConsensusParams {
                enforce_bip113_median_time_past: false,
                enforce_segwit: true,
                ..Default::default()
            },
        };
        assert_eq!(
            check_block_contextual(&heavy_block, &no_mtp_context)
                .expect_err("witness weight must fail")
                .reject_reason,
            "bad-blk-weight",
        );

        let mut bad_nonce_block = heavy_block.clone();
        bad_nonce_block.transactions[0].inputs[0].witness = ScriptWitness::new(vec![vec![0_u8; 1]]);
        assert_eq!(
            check_block_contextual(
                &bad_nonce_block,
                &BlockValidationContext {
                    consensus_params: ConsensusParams {
                        enforce_segwit: true,
                        ..Default::default()
                    },
                    ..no_mtp_context
                }
            )
            .expect_err("bad witness nonce size must fail")
            .reject_reason,
            "bad-witness-nonce-size",
        );
    }

    #[test]
    fn validate_block_with_context_maps_transaction_errors() {
        let (block, spent_outputs) = valid_block();
        let block_context = BlockValidationContext {
            height: 1,
            previous_header: BlockHeader {
                time: block.header.time - 1,
                ..BlockHeader::default()
            },
            previous_median_time_past: i64::from(block.header.time) - 1,
            consensus_params: ConsensusParams {
                enforce_segwit: false,
                ..Default::default()
            },
        };
        let tx_contexts = vec![TransactionValidationContext {
            inputs: vec![TransactionInputContext {
                spent_output: SpentOutput {
                    is_coinbase: true,
                    ..spent_outputs[0][0].clone()
                },
                created_height: 1,
                created_median_time_past: 0,
            }],
            spend_height: 1,
            block_time: i64::from(block.header.time),
            median_time_past: i64::from(block.header.time) - 1,
            verify_flags: ScriptVerifyFlags::NONE,
            consensus_params: block_context.consensus_params,
        }];

        let error = validate_block_with_context(&block, &tx_contexts, &block_context)
            .expect_err("transaction context error should map to block error");
        assert_eq!(error.reject_reason, "bad-txns-premature-spend-of-coinbase");
        assert!(
            error
                .debug_message
                .expect("debug message")
                .contains("failed validation")
        );

        let no_debug_tx_contexts = vec![TransactionValidationContext {
            inputs: vec![TransactionInputContext {
                spent_output: SpentOutput {
                    is_coinbase: false,
                    ..spent_outputs[0][0].clone()
                },
                created_height: 0,
                created_median_time_past: 0,
            }],
            spend_height: 0,
            block_time: i64::from(block.header.time),
            median_time_past: i64::from(block.header.time) - 1,
            verify_flags: ScriptVerifyFlags::CHECKSEQUENCEVERIFY,
            consensus_params: block_context.consensus_params,
        }];
        let mut sequence_locked_block = block.clone();
        sequence_locked_block.transactions[1].version = 2;
        sequence_locked_block.transactions[1].inputs[0].sequence = 2;
        let (sequence_merkle_root, _) =
            block_merkle_root(&sequence_locked_block.transactions).expect("merkle root");
        sequence_locked_block.header.merkle_root = sequence_merkle_root;
        mine_header(&mut sequence_locked_block);

        let error = validate_block_with_context(
            &sequence_locked_block,
            &no_debug_tx_contexts,
            &block_context,
        )
        .expect_err("sequence lock failure should map without source debug");
        assert_eq!(error.reject_reason, "non-BIP68-final");
        assert!(
            error
                .debug_message
                .expect("debug message")
                .contains("failed validation")
        );
    }

    #[test]
    fn validate_block_with_context_rejects_split_sigop_overflow() {
        let coinbase = coinbase_transaction();
        let mut transactions = vec![coinbase];
        let mut transaction_contexts = Vec::new();
        for index in 1..=127_u8 {
            let (transaction, context) = p2sh_sigop_heavy_transaction(index, 200);
            transactions.push(transaction);
            transaction_contexts.push(context);
        }

        let (merkle_root, maybe_mutated) = block_merkle_root(&transactions).expect("merkle root");
        assert!(!maybe_mutated);
        let mut block = Block {
            header: BlockHeader {
                version: 1,
                previous_block_hash: BlockHash::from_byte_array([0_u8; 32]),
                merkle_root,
                time: 1_231_006_505,
                bits: EASY_BITS,
                nonce: 0,
            },
            transactions,
        };
        mine_header(&mut block);
        let block_context = BlockValidationContext {
            height: 1,
            previous_header: BlockHeader {
                time: block.header.time - 1,
                ..BlockHeader::default()
            },
            previous_median_time_past: i64::from(block.header.time) - 1,
            consensus_params: ConsensusParams {
                enforce_segwit: false,
                ..ConsensusParams::default()
            },
        };

        let error = validate_block_with_context(&block, &transaction_contexts, &block_context)
            .expect_err("split sigop overflow must fail");

        assert_eq!(error.reject_reason, "bad-blk-sigops");
    }

    #[test]
    fn sigop_helper_functions_are_covered_directly() {
        let (transaction, context) = p2sh_sigop_heavy_transaction(200, 5);

        assert_eq!(legacy_sigop_cost(&transaction).expect("legacy cost"), 0);
        assert_eq!(
            split_sigop_cost(&transaction, &context).expect("split cost"),
            20
        );
        assert_eq!(block_sigop_overflow().reject_reason, "bad-blk-sigops");
        assert_eq!(enforce_sigop_cost_limit(0), Ok(()));
        assert_eq!(
            enforce_sigop_cost_limit(MAX_BLOCK_SIGOPS_COST + 1)
                .expect_err("overflow must fail")
                .reject_reason,
            "bad-blk-sigops"
        );
    }
}
