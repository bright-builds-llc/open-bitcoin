// Parity breadcrumbs:
// - packages/bitcoin-knots/src/consensus/tx_check.cpp
// - packages/bitcoin-knots/src/consensus/tx_verify.cpp
// - packages/bitcoin-knots/src/consensus/validation.h
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/test/data/tx_valid.json
// - packages/bitcoin-knots/src/test/data/tx_invalid.json

use open_bitcoin_codec::parse_transaction;
use open_bitcoin_consensus::{
    BlockValidationContext, ConsensusParams, ScriptError, ScriptExecutionData,
    ScriptInputVerificationContext, ScriptVerifyFlags, SigHashType, SpentOutput,
    TransactionInputContext, TransactionValidationContext, check_block_contextual, legacy_sighash,
    validate_transaction_with_context, verify_input_script,
};
use open_bitcoin_primitives::{
    Amount, Block, BlockHash, BlockHeader, MerkleRoot, OutPoint, ScriptBuf, ScriptWitness,
    Transaction, TransactionInput, TransactionOutput, Txid,
};

const EASY_BITS: u32 = 0x207f_ffff;

#[path = "parity_closure/vector_data.rs"]
mod vector_data;
use vector_data::*;

fn decode_hex(input: &str) -> Vec<u8> {
    let trimmed = input.trim();
    assert_eq!(trimmed.len() % 2, 0, "hex fixtures must use full bytes");
    let mut bytes = Vec::with_capacity(trimmed.len() / 2);
    let chars: Vec<char> = trimmed.chars().collect();
    for pair in chars.chunks(2) {
        let high = pair[0].to_digit(16).expect("fixture should be hex");
        let low = pair[1].to_digit(16).expect("fixture should be hex");
        bytes.push(((high << 4) | low) as u8);
    }
    bytes
}

fn parse_json(input: &str) -> JsonValue {
    struct Parser<'a> {
        bytes: &'a [u8],
        pos: usize,
    }

    impl<'a> Parser<'a> {
        fn peek(&self) -> Option<u8> {
            self.bytes.get(self.pos).copied()
        }

        fn bump(&mut self) -> Option<u8> {
            let byte = self.peek()?;
            self.pos += 1;
            Some(byte)
        }

        fn skip_ws(&mut self) {
            while self.peek().is_some_and(|byte| byte.is_ascii_whitespace()) {
                self.pos += 1;
            }
        }

        fn expect(&mut self, expected: u8) {
            assert_eq!(self.bump(), Some(expected), "unexpected JSON token");
        }

        fn parse_string(&mut self) -> String {
            self.expect(b'"');
            let mut out = String::new();
            while let Some(byte) = self.bump() {
                match byte {
                    b'"' => return out,
                    b'\\' => {
                        let escaped = self.bump().expect("unterminated escape");
                        out.push(match escaped {
                            b'"' => '"',
                            b'\\' => '\\',
                            b'/' => '/',
                            b'b' => '\u{0008}',
                            b'f' => '\u{000c}',
                            b'n' => '\n',
                            b'r' => '\r',
                            b't' => '\t',
                            other => other as char,
                        });
                    }
                    other => out.push(other as char),
                }
            }
            panic!("unterminated JSON string");
        }

        fn parse_number(&mut self) -> String {
            let start = self.pos;
            while self
                .peek()
                .is_some_and(|byte| matches!(byte, b'-' | b'+' | b'.' | b'e' | b'E' | b'0'..=b'9'))
            {
                self.pos += 1;
            }
            String::from_utf8(self.bytes[start..self.pos].to_vec()).expect("valid number")
        }

        fn parse_array(&mut self) -> Vec<JsonValue> {
            self.expect(b'[');
            self.skip_ws();
            let mut values = Vec::new();
            if self.peek() == Some(b']') {
                self.pos += 1;
                return values;
            }
            loop {
                values.push(self.parse_value());
                self.skip_ws();
                match self.bump() {
                    Some(b',') => {
                        self.skip_ws();
                    }
                    Some(b']') => return values,
                    other => panic!("unexpected array delimiter: {other:?}"),
                }
            }
        }

        fn parse_value(&mut self) -> JsonValue {
            self.skip_ws();
            match self.peek().expect("expected JSON value") {
                b'"' => JsonValue::String(self.parse_string()),
                b'[' => JsonValue::Array(self.parse_array()),
                b'-' | b'0'..=b'9' => JsonValue::Number(self.parse_number()),
                other => panic!("unsupported JSON token: {other}"),
            }
        }
    }

    let mut parser = Parser {
        bytes: input.as_bytes(),
        pos: 0,
    };
    parser.parse_value()
}

fn load_sighash_vectors() -> Vec<LegacySighashVector> {
    let data = include_str!("../../bitcoin-knots/src/test/data/sighash.json");
    let JsonValue::Array(entries) = parse_json(data) else {
        panic!("sighash.json must be a top-level array");
    };

    entries
        .into_iter()
        .filter_map(|entry| {
            let JsonValue::Array(fields) = entry else {
                return None;
            };
            if fields.len() != 5 {
                return None;
            }
            let (
                JsonValue::String(raw_tx),
                JsonValue::String(script),
                JsonValue::Number(input_index),
                JsonValue::Number(hash_type),
                JsonValue::String(expected_hash),
            ) = (&fields[0], &fields[1], &fields[2], &fields[3], &fields[4])
            else {
                return None;
            };
            if raw_tx.starts_with("raw_transaction") {
                return None;
            }
            Some(LegacySighashVector {
                raw_tx: raw_tx.clone(),
                script: script.clone(),
                input_index: input_index.parse::<usize>().expect("input index"),
                hash_type: hash_type.parse::<i64>().expect("hash type") as i32 as u32,
                expected_hash: expected_hash.clone(),
            })
        })
        .collect()
}

fn script(bytes: &[u8]) -> ScriptBuf {
    ScriptBuf::from_bytes(bytes.to_vec()).expect("valid script")
}

fn encode_script_num(value: i64) -> Vec<u8> {
    match value {
        -1 => vec![0x4f],
        0 => vec![0x00],
        1..=16 => vec![0x50 + value as u8],
        _ => {
            let negative = value < 0;
            let mut abs = value.unsigned_abs();
            let mut out = Vec::new();
            while abs > 0 {
                out.push((abs & 0xff) as u8);
                abs >>= 8;
            }
            if out.last().is_some_and(|byte| (byte & 0x80) != 0) {
                out.push(if negative { 0x80 } else { 0x00 });
            } else if negative {
                let last = out.last_mut().expect("non-empty number bytes");
                *last |= 0x80;
            }
            let mut encoded = Vec::with_capacity(out.len() + 1);
            encoded.push(out.len() as u8);
            encoded.extend_from_slice(&out);
            encoded
        }
    }
}

fn opcode_byte(token: &str) -> Option<u8> {
    match token {
        "0" | "OP_0" | "FALSE" | "OP_FALSE" => Some(0x00),
        "1" | "OP_1" | "TRUE" | "OP_TRUE" => Some(0x51),
        "2" | "OP_2" => Some(0x52),
        "16" | "OP_16" => Some(0x60),
        "EQUAL" | "OP_EQUAL" => Some(0x87),
        "EQUALVERIFY" | "OP_EQUALVERIFY" => Some(0x88),
        "HASH160" | "OP_HASH160" => Some(0xa9),
        "CHECKSIG" | "OP_CHECKSIG" => Some(0xac),
        "CHECKMULTISIG" | "OP_CHECKMULTISIG" => Some(0xae),
        "DUP" | "OP_DUP" => Some(0x76),
        "IF" | "OP_IF" => Some(0x63),
        "ELSE" | "OP_ELSE" => Some(0x67),
        "ENDIF" | "OP_ENDIF" => Some(0x68),
        "NOP" | "OP_NOP" => Some(0x61),
        _ => None,
    }
}

fn parse_script_expr(expr: &str) -> ScriptBuf {
    let mut out = Vec::new();
    for token in expr.split_whitespace() {
        if token.is_empty() {
            continue;
        }
        if let Some(opcode) = opcode_byte(token) {
            out.push(opcode);
            continue;
        }
        if let Some(hex) = token.strip_prefix("0x") {
            out.extend_from_slice(&decode_hex(hex));
            continue;
        }
        if token.chars().all(|ch| ch == '-' || ch.is_ascii_digit()) {
            let encoded = encode_script_num(token.parse::<i64>().expect("script number"));
            out.extend_from_slice(&encoded);
            continue;
        }
        if token.starts_with('\'') && token.ends_with('\'') && token.len() >= 2 {
            let bytes = &token.as_bytes()[1..token.len() - 1];
            out.push(bytes.len() as u8);
            out.extend_from_slice(bytes);
            continue;
        }
        panic!("unsupported script token: {token}");
    }
    script(&out)
}

fn parse_flags(flags: &str) -> ScriptVerifyFlags {
    let mut parsed = ScriptVerifyFlags::NONE;
    if flags.trim().is_empty() {
        return parsed;
    }
    for token in flags
        .split(',')
        .map(str::trim)
        .filter(|token| !token.is_empty())
    {
        parsed |= match token {
            "P2SH" => ScriptVerifyFlags::P2SH,
            "STRICTENC" => ScriptVerifyFlags::STRICTENC,
            "DERSIG" => ScriptVerifyFlags::DERSIG,
            "LOW_S" => ScriptVerifyFlags::LOW_S,
            "NULLDUMMY" => ScriptVerifyFlags::NULLDUMMY,
            "SIGPUSHONLY" => ScriptVerifyFlags::SIGPUSHONLY,
            "MINIMALDATA" => ScriptVerifyFlags::MINIMALDATA,
            "CLEANSTACK" => ScriptVerifyFlags::CLEANSTACK,
            "WITNESS" => ScriptVerifyFlags::WITNESS,
            "DISCOURAGE_UPGRADABLE_WITNESS_PROGRAM" => {
                ScriptVerifyFlags::DISCOURAGE_UPGRADABLE_WITNESS_PROGRAM
            }
            "MINIMALIF" => ScriptVerifyFlags::MINIMALIF,
            "NULLFAIL" => ScriptVerifyFlags::NULLFAIL,
            "WITNESS_PUBKEYTYPE" => ScriptVerifyFlags::WITNESS_PUBKEYTYPE,
            other => panic!("unsupported flag token: {other}"),
        };
    }
    parsed
}

fn build_crediting_transaction(script_pubkey: &ScriptBuf, amount_sats: i64) -> Transaction {
    Transaction {
        version: 1,
        inputs: vec![TransactionInput {
            previous_output: OutPoint::null(),
            script_sig: script(&[0x00, 0x00]),
            sequence: TransactionInput::SEQUENCE_FINAL,
            witness: ScriptWitness::default(),
        }],
        outputs: vec![TransactionOutput {
            value: Amount::from_sats(amount_sats).expect("valid amount"),
            script_pubkey: script_pubkey.clone(),
        }],
        lock_time: 0,
    }
}

fn build_spending_transaction(
    script_sig: &ScriptBuf,
    witness: &ScriptWitness,
    credit_tx: &Transaction,
) -> Transaction {
    let credit_txid = open_bitcoin_consensus::transaction_txid(credit_tx).expect("credit txid");
    Transaction {
        version: 1,
        inputs: vec![TransactionInput {
            previous_output: OutPoint {
                txid: credit_txid,
                vout: 0,
            },
            script_sig: script_sig.clone(),
            sequence: TransactionInput::SEQUENCE_FINAL,
            witness: witness.clone(),
        }],
        outputs: vec![TransactionOutput {
            value: credit_tx.outputs[0].value,
            script_pubkey: ScriptBuf::default(),
        }],
        lock_time: 0,
    }
}

fn core_error_name(error: &ScriptError) -> &'static str {
    match error {
        ScriptError::EvalFalse => "EVAL_FALSE",
        ScriptError::OpReturn => "OP_RETURN",
        ScriptError::OpCount => "OP_COUNT",
        ScriptError::StackOverflow(_) => "STACK_SIZE",
        ScriptError::SigCount => "SIG_COUNT",
        ScriptError::PubKeyCount => "PUBKEY_COUNT",
        ScriptError::VerifyFailed => "VERIFY",
        ScriptError::DisabledOpcode(_) => "DISABLED_OPCODE",
        ScriptError::UnsupportedOpcode(0x92) => "DISCOURAGE_UPGRADABLE_WITNESS_PROGRAM",
        ScriptError::UnsupportedOpcode(_) | ScriptError::BadOpcode => "BAD_OPCODE",
        ScriptError::InvalidStackOperation => "INVALID_STACK_OPERATION",
        ScriptError::UnbalancedConditional => "UNBALANCED_CONDITIONAL",
        ScriptError::SigHashType => "SIG_HASHTYPE",
        ScriptError::SigDer => "SIG_DER",
        ScriptError::SigPushOnly => "SIG_PUSHONLY",
        ScriptError::SigHighS => "SIG_HIGH_S",
        ScriptError::SigNullDummy => "SIG_NULLDUMMY",
        ScriptError::PubKeyType => "PUBKEYTYPE",
        ScriptError::WitnessCleanStack => "CLEANSTACK",
        ScriptError::SigNullFail => "NULLFAIL",
        ScriptError::WitnessProgramWrongLength => "WITNESS_PROGRAM_WRONG_LENGTH",
        ScriptError::WitnessProgramWitnessEmpty => "WITNESS_PROGRAM_WITNESS_EMPTY",
        ScriptError::WitnessProgramMismatch => "WITNESS_PROGRAM_MISMATCH",
        ScriptError::WitnessMalleated => "WITNESS_MALLEATED",
        ScriptError::WitnessMalleatedP2sh => "WITNESS_MALLEATED_P2SH",
        ScriptError::WitnessUnexpected => "WITNESS_UNEXPECTED",
        ScriptError::WitnessPubKeyType => "WITNESS_PUBKEYTYPE",
        _ => panic!("unsupported script error mapping: {error:?}"),
    }
}

fn build_context(
    transaction: &Transaction,
    script_pubkey: &ScriptBuf,
    amount_sats: i64,
    verify_flags: ScriptVerifyFlags,
) -> (TransactionInputContext, TransactionValidationContext) {
    let spent_input = TransactionInputContext {
        spent_output: SpentOutput {
            value: Amount::from_sats(amount_sats).expect("valid amount"),
            script_pubkey: script_pubkey.clone(),
            is_coinbase: false,
        },
        created_height: 0,
        created_median_time_past: 0,
    };
    let context = TransactionValidationContext {
        inputs: vec![spent_input.clone()],
        spend_height: 1,
        block_time: 0,
        median_time_past: 0,
        verify_flags,
        consensus_params: ConsensusParams::default(),
    };
    let _ = context.precompute(transaction).expect("precompute");
    (spent_input, context)
}

fn coinbase_transaction_with_height(height: u32) -> Transaction {
    let height_bytes = if height == 0 {
        vec![0x00]
    } else if height <= 0x7f {
        vec![0x01, height as u8]
    } else {
        panic!("test fixture only supports small heights");
    };
    Transaction {
        version: 1,
        inputs: vec![TransactionInput {
            previous_output: OutPoint::null(),
            script_sig: script(&height_bytes),
            sequence: TransactionInput::SEQUENCE_FINAL,
            witness: ScriptWitness::default(),
        }],
        outputs: vec![TransactionOutput {
            value: Amount::from_sats(50).expect("valid amount"),
            script_pubkey: script(&[0x51]),
        }],
        lock_time: 0,
    }
}

fn spend_transaction(previous_txid: Txid, lock_time: u32, sequence: u32) -> Transaction {
    Transaction {
        version: 2,
        inputs: vec![TransactionInput {
            previous_output: OutPoint {
                txid: previous_txid,
                vout: 0,
            },
            script_sig: script(&[0x51]),
            sequence,
            witness: ScriptWitness::default(),
        }],
        outputs: vec![TransactionOutput {
            value: Amount::from_sats(40).expect("valid amount"),
            script_pubkey: script(&[0x51]),
        }],
        lock_time,
    }
}

fn mine_header(block: &mut Block) {
    block.header.nonce = (0..=u32::MAX)
        .find(|nonce| {
            block.header.nonce = *nonce;
            open_bitcoin_consensus::check_block_header(&block.header).is_ok()
        })
        .expect("easy target should mine");
}

fn witness_merkle_root(block: &Block) -> MerkleRoot {
    if block.transactions.is_empty() {
        return MerkleRoot::from_byte_array([0_u8; 32]);
    }

    let mut level = Vec::with_capacity(block.transactions.len());
    level.push([0_u8; 32]);
    for transaction in block.transactions.iter().skip(1) {
        level.push(
            open_bitcoin_consensus::transaction_wtxid(transaction)
                .expect("wtxid")
                .to_byte_array(),
        );
    }

    while level.len() > 1 {
        if level.len() % 2 == 1 {
            let last = *level.last().expect("non-empty merkle level");
            level.push(last);
        }
        let mut next = Vec::with_capacity(level.len() / 2);
        for pair in level.chunks_exact(2) {
            let mut concatenated = [0_u8; 64];
            concatenated[..32].copy_from_slice(&pair[0]);
            concatenated[32..].copy_from_slice(&pair[1]);
            next.push(open_bitcoin_consensus::crypto::double_sha256(&concatenated));
        }
        level = next;
    }

    MerkleRoot::from_byte_array(level[0])
}

#[path = "parity_closure/contextual_consensus_regressions.rs"]
mod contextual_consensus_regressions;
#[path = "parity_closure/imported_sighash_vectors_match_upstream.rs"]
mod imported_sighash_vectors_match_upstream;
