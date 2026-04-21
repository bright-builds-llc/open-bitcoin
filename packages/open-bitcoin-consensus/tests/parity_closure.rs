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

struct LegacySighashVector {
    raw_tx: String,
    script: String,
    input_index: usize,
    hash_type: u32,
    expected_hash: String,
}

struct ScriptVector {
    comment: &'static str,
    witness_stack: &'static [&'static str],
    amount_sats: i64,
    script_sig: &'static str,
    script_pubkey: &'static str,
    flags: &'static str,
    expected: &'static str,
}

#[derive(Clone, Debug)]
enum JsonValue {
    String(String),
    Number(String),
    Array(Vec<JsonValue>),
}

const SCRIPT_VECTORS: &[ScriptVector] = &[
    ScriptVector {
        comment: "Basic P2WSH",
        witness_stack: &[
            "304402200d461c140cfdfcf36b94961db57ae8c18d1cb80e9d95a9e47ac22470c1bf125502201c8dc1cbfef6a3ef90acbbb992ca22fe9466ee6f9d4898eda277a7ac3ab4b25101",
            "410479be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8ac",
        ],
        amount_sats: 1,
        script_sig: "",
        script_pubkey: "0 0x20 0xb95237b48faaa69eb078e1170be3b5cbb3fddf16d0a991e14ad274f7b33a4f64",
        flags: "P2SH,WITNESS",
        expected: "OK",
    },
    ScriptVector {
        comment: "Basic P2WPKH",
        witness_stack: &[
            "304402201e7216e5ccb3b61d46946ec6cc7e8c4e0117d13ac2fd4b152197e4805191c74202203e9903e33e84d9ee1dd13fb057afb7ccfb47006c23f6a067185efbc9dd780fc501",
            "0479be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8",
        ],
        amount_sats: 1,
        script_sig: "",
        script_pubkey: "0 0x14 0x91b24bf9f5288532960ac687abb035127b1d28a5",
        flags: "P2SH,WITNESS",
        expected: "OK",
    },
    ScriptVector {
        comment: "Basic P2SH(P2WSH)",
        witness_stack: &[
            "3044022066e02c19a513049d49349cf5311a1b012b7c4fae023795a18ab1d91c23496c22022025e216342c8e07ce8ef51e8daee88f84306a9de66236cab230bb63067ded1ad301",
            "410479be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8ac",
        ],
        amount_sats: 1,
        script_sig: "0x22 0x0020b95237b48faaa69eb078e1170be3b5cbb3fddf16d0a991e14ad274f7b33a4f64",
        script_pubkey: "HASH160 0x14 0xf386c2ba255cc56d20cfa6ea8b062f8b59945518 EQUAL",
        flags: "P2SH,WITNESS",
        expected: "OK",
    },
    ScriptVector {
        comment: "Basic P2SH(P2WPKH)",
        witness_stack: &[
            "304402200929d11561cd958460371200f82e9cae64c727a495715a31828e27a7ad57b36d0220361732ced04a6f97351ecca21a56d0b8cd4932c1da1f8f569a2b68e5e48aed7801",
            "0479be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8",
        ],
        amount_sats: 1,
        script_sig: "0x16 0x001491b24bf9f5288532960ac687abb035127b1d28a5",
        script_pubkey: "HASH160 0x14 0x17743beb429c55c942d2ec703b98c4d57c2df5c6 EQUAL",
        flags: "P2SH,WITNESS",
        expected: "OK",
    },
    ScriptVector {
        comment: "P2WPKH with future witness version",
        witness_stack: &[
            "304402205ae57ae0534c05ca9981c8a6cdf353b505eaacb7375f96681a2d1a4ba6f02f84022056248e68643b7d8ce7c7d128c9f1f348bcab8be15d094ad5cadd24251a28df8001",
            "0479be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8",
        ],
        amount_sats: 0,
        script_sig: "",
        script_pubkey: "1 0x14 0x91b24bf9f5288532960ac687abb035127b1d28a5",
        flags: "DISCOURAGE_UPGRADABLE_WITNESS_PROGRAM,P2SH,WITNESS",
        expected: "DISCOURAGE_UPGRADABLE_WITNESS_PROGRAM",
    },
    ScriptVector {
        comment: "P2WPKH with wrong witness program length",
        witness_stack: &[
            "3044022064100ca0e2a33332136775a86cd83d0230e58b9aebb889c5ac952abff79a46ef02205f1bf900e022039ad3091bdaf27ac2aef3eae9ed9f190d821d3e508405b9513101",
            "0479be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8",
        ],
        amount_sats: 0,
        script_sig: "",
        script_pubkey: "0 0x1f 0xb34b78da162751647974d5cb7410aa428ad339dbf7d1e16e833f68a0cbf1c3",
        flags: "P2SH,WITNESS",
        expected: "WITNESS_PROGRAM_WRONG_LENGTH",
    },
    ScriptVector {
        comment: "P2WSH with witness program mismatch",
        witness_stack: &[
            "3044022039105b995a5f448639a997a5c90fda06f50b49df30c3bdb6663217bf79323db002206fecd54269dec569fcc517178880eb58bb40f381a282bb75766ff3637d5f4b4301",
            "400479be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8ac",
        ],
        amount_sats: 0,
        script_sig: "",
        script_pubkey: "0 0x20 0xb95237b48faaa69eb078e1170be3b5cbb3fddf16d0a991e14ad274f7b33a4f64",
        flags: "P2SH,WITNESS",
        expected: "WITNESS_PROGRAM_MISMATCH",
    },
    ScriptVector {
        comment: "P2WPKH with non-empty scriptSig",
        witness_stack: &[
            "304402201a96950593cb0af32d080b0f193517f4559241a8ebd1e95e414533ad64a3f423022047f4f6d3095c23235bdff3aeff480d0529c027a3f093cb265b7cbf148553b85101",
            "0479be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8",
        ],
        amount_sats: 0,
        script_sig: "11",
        script_pubkey: "0 0x14 0x91b24bf9f5288532960ac687abb035127b1d28a5",
        flags: "P2SH,WITNESS",
        expected: "WITNESS_MALLEATED",
    },
    ScriptVector {
        comment: "P2SH(P2WPKH) with superfluous push in scriptSig",
        witness_stack: &[
            "304402204209e49457c2358f80d0256bc24535b8754c14d08840fc4be762d6f5a0aed80b02202eaf7d8fc8d62f60c67adcd99295528d0e491ae93c195cec5a67e7a09532a88001",
            "048282263212c609d9ea2a6e3e172de238d8c39cabd5ac1ca10646e23fd5f5150811f8a8098557dfe45e8256e830b60ace62d613ac2f7b17bed31b6eaff6e26caf",
        ],
        amount_sats: 0,
        script_sig: "11 0x16 0x00147cf9c846cd4882efec4bf07e44ebdad495c94f4b",
        script_pubkey: "HASH160 0x14 0x4e0c2aed91315303fc6a1dc4c7bc21c88f75402e EQUAL",
        flags: "P2SH,WITNESS",
        expected: "WITNESS_MALLEATED_P2SH",
    },
    ScriptVector {
        comment: "P2PK with witness",
        witness_stack: &[""],
        amount_sats: 0,
        script_sig: "0x47 0x304402200a5c6163f07b8d3b013c4d1d6dba25e780b39658d79ba37af7057a3b7f15ffa102201fd9b4eaa9943f734928b99a83592c2e7bf342ea2680f6a2bb705167966b742001",
        script_pubkey: "0x41 0x0479be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8 CHECKSIG",
        flags: "P2SH,WITNESS",
        expected: "WITNESS_UNEXPECTED",
    },
    ScriptVector {
        comment: "Basic P2WSH with compressed key",
        witness_stack: &[
            "304402204256146fcf8e73b0fd817ffa2a4e408ff0418ff987dd08a4f485b62546f6c43c02203f3c8c3e2febc051e1222867f5f9d0eaf039d6792911c10940aa3cc74123378e01",
            "210279be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798ac",
        ],
        amount_sats: 1,
        script_sig: "",
        script_pubkey: "0 0x20 0x1863143c14c5166804bd19203356da136c985678cd4d27a1b8c6329604903262",
        flags: "P2SH,WITNESS,WITNESS_PUBKEYTYPE",
        expected: "OK",
    },
    ScriptVector {
        comment: "Basic P2WPKH with compressed key",
        witness_stack: &[
            "304402204edf27486f11432466b744df533e1acac727e0c83e5f912eb289a3df5bf8035f022075809fdd876ede40ad21667eba8b7e96394938f9c9c50f11b6a1280cce2cea8601",
            "0279be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798",
        ],
        amount_sats: 1,
        script_sig: "",
        script_pubkey: "0 0x14 0x751e76e8199196d454941c45d1b3a323f1433bd6",
        flags: "P2SH,WITNESS,WITNESS_PUBKEYTYPE",
        expected: "OK",
    },
    ScriptVector {
        comment: "P2WSH CHECKMULTISIG with compressed keys",
        witness_stack: &[
            "",
            "304402207eb8a59b5c65fc3f6aeef77066556ed5c541948a53a3ba7f7c375b8eed76ee7502201e036a7a9a98ff919ff94dc905d67a1ec006f79ef7cff0708485c8bb79dce38e01",
            "5121038282263212c609d9ea2a6e3e172de238d8c39cabd5ac1ca10646e23fd5f51508210279be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f8179852ae",
        ],
        amount_sats: 1,
        script_sig: "",
        script_pubkey: "0 0x20 0x06c24420938f0fa3c1cb2707d867154220dca365cdbfa0dd2a83854730221460",
        flags: "P2SH,WITNESS,WITNESS_PUBKEYTYPE",
        expected: "OK",
    },
    ScriptVector {
        comment: "P2WSH CHECKMULTISIG with first key uncompressed and signing with the first key",
        witness_stack: &[
            "",
            "304402202d092ededd1f060609dbf8cb76950634ff42b3e62cf4adb69ab92397b07d742302204ff886f8d0817491a96d1daccdcc820f6feb122ee6230143303100db37dfa79f01",
            "5121038282263212c609d9ea2a6e3e172de238d8c39cabd5ac1ca10646e23fd5f51508410479be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b852ae",
        ],
        amount_sats: 1,
        script_sig: "",
        script_pubkey: "0 0x20 0x08a6665ebfd43b02323423e764e185d98d1587f903b81507dbb69bfc41005efa",
        flags: "P2SH,WITNESS",
        expected: "OK",
    },
    ScriptVector {
        comment: "P2WSH CHECKMULTISIG with second key uncompressed and signing with the first key should pass as the uncompressed key is not used",
        witness_stack: &[
            "",
            "3044022046f5367a261fd8f8d7de6eb390491344f8ec2501638fb9a1095a0599a21d3f4c02205c1b3b51d20091c5f1020841bbca87b44ebe25405c64e4acf758f2eae8665f8401",
            "5141048282263212c609d9ea2a6e3e172de238d8c39cabd5ac1ca10646e23fd5f5150811f8a8098557dfe45e8256e830b60ace62d613ac2f7b17bed31b6eaff6e26caf210279be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f8179852ae",
        ],
        amount_sats: 1,
        script_sig: "",
        script_pubkey: "0 0x20 0x230828ed48871f0f362ce9432aa52f620f442cc8d9ce7a8b5e798365595a38bb",
        flags: "P2SH,WITNESS,WITNESS_PUBKEYTYPE",
        expected: "OK",
    },
];

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

#[test]
fn imported_sighash_vectors_match_upstream() {
    for vector in load_sighash_vectors() {
        let transaction = parse_transaction(&decode_hex(&vector.raw_tx)).expect("vector tx");
        let script_code = script(&decode_hex(&vector.script));
        let mut expected_hash = decode_hex(&vector.expected_hash);
        expected_hash.reverse();
        let digest = legacy_sighash(
            &script_code,
            &transaction,
            vector.input_index,
            SigHashType::from_u32(vector.hash_type),
        );
        assert_eq!(
            digest.to_byte_array().as_slice(),
            expected_hash.as_slice(),
            "legacy sighash mismatch for hash type {}",
            vector.hash_type
        );
    }
}

#[test]
fn imported_script_vectors_match_supported_consensus_surface() {
    for vector in SCRIPT_VECTORS {
        let witness = ScriptWitness::new(
            vector
                .witness_stack
                .iter()
                .map(|item| decode_hex(item))
                .collect(),
        );
        let script_sig = parse_script_expr(vector.script_sig);
        let script_pubkey = parse_script_expr(vector.script_pubkey);
        let verify_flags = parse_flags(vector.flags);
        let credit_tx = build_crediting_transaction(&script_pubkey, vector.amount_sats);
        let transaction = build_spending_transaction(&script_sig, &witness, &credit_tx);
        let (spent_input, context) = build_context(
            &transaction,
            &script_pubkey,
            vector.amount_sats,
            verify_flags,
        );
        let precomputed = context.precompute(&transaction).expect("precompute");
        let mut execution_data = ScriptExecutionData::default();

        let result = verify_input_script(ScriptInputVerificationContext {
            script_sig: &script_sig,
            script_pubkey: &script_pubkey,
            witness: &witness,
            transaction: &transaction,
            input_index: 0,
            spent_input: &spent_input,
            validation_context: &context,
            spent_amount: spent_input.spent_output.value,
            verify_flags,
            precomputed: &precomputed,
            execution_data: &mut execution_data,
        });

        match vector.expected {
            "OK" => assert!(result.is_ok(), "{} should pass", vector.comment),
            expected => {
                let error = result.expect_err(vector.comment);
                assert_eq!(
                    core_error_name(&error),
                    expected,
                    "unexpected script error for {}",
                    vector.comment
                );
            }
        }
    }
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

#[test]
fn repo_owned_contextual_consensus_regressions_are_covered() {
    let coinbase = coinbase_transaction_with_height(1);
    let coinbase_txid = open_bitcoin_consensus::transaction_txid(&coinbase).expect("coinbase txid");

    let immature_spend = spend_transaction(coinbase_txid, 0, TransactionInput::SEQUENCE_FINAL);
    let immature_context = TransactionValidationContext {
        inputs: vec![TransactionInputContext {
            spent_output: SpentOutput {
                value: coinbase.outputs[0].value,
                script_pubkey: coinbase.outputs[0].script_pubkey.clone(),
                is_coinbase: true,
            },
            created_height: 1,
            created_median_time_past: 0,
        }],
        spend_height: 10,
        block_time: 0,
        median_time_past: 0,
        verify_flags: ScriptVerifyFlags::NONE,
        consensus_params: ConsensusParams::default(),
    };
    let maturity_error = validate_transaction_with_context(&immature_spend, &immature_context)
        .expect_err("immature coinbase spend must fail");
    assert_eq!(
        maturity_error.reject_reason,
        "bad-txns-premature-spend-of-coinbase"
    );

    let nonfinal_tx = spend_transaction(coinbase_txid, 2, 0);
    let nonfinal_context = TransactionValidationContext {
        inputs: vec![TransactionInputContext {
            spent_output: SpentOutput {
                value: Amount::from_sats(50).expect("valid amount"),
                script_pubkey: script(&[0x51]),
                is_coinbase: false,
            },
            created_height: 0,
            created_median_time_past: 0,
        }],
        spend_height: 1,
        block_time: 0,
        median_time_past: 0,
        verify_flags: ScriptVerifyFlags::CHECKSEQUENCEVERIFY,
        consensus_params: ConsensusParams::default(),
    };
    let nonfinal_error = validate_transaction_with_context(&nonfinal_tx, &nonfinal_context)
        .expect_err("non-final transaction must fail");
    assert_eq!(nonfinal_error.reject_reason, "bad-txns-nonfinal");

    let sequence_locked_tx = spend_transaction(coinbase_txid, 0, 5);
    let sequence_locked_context = TransactionValidationContext {
        inputs: vec![TransactionInputContext {
            spent_output: SpentOutput {
                value: Amount::from_sats(50).expect("valid amount"),
                script_pubkey: script(&[0x51]),
                is_coinbase: false,
            },
            created_height: 0,
            created_median_time_past: 0,
        }],
        spend_height: 1,
        block_time: 0,
        median_time_past: 0,
        verify_flags: ScriptVerifyFlags::CHECKSEQUENCEVERIFY,
        consensus_params: ConsensusParams::default(),
    };
    let sequence_error =
        validate_transaction_with_context(&sequence_locked_tx, &sequence_locked_context)
            .expect_err("sequence-locked transaction must fail");
    assert_eq!(sequence_error.reject_reason, "non-BIP68-final");

    let mut coinbase_with_witness = coinbase_transaction_with_height(1);
    coinbase_with_witness.inputs[0].witness = ScriptWitness::new(vec![vec![9_u8; 32]]);
    let witness_spend_txid =
        open_bitcoin_consensus::transaction_txid(&coinbase_with_witness).expect("coinbase txid");
    let mut witness_spend =
        spend_transaction(witness_spend_txid, 0, TransactionInput::SEQUENCE_FINAL);
    witness_spend.inputs[0].witness = ScriptWitness::new(vec![vec![1_u8]]);
    let mut block = Block {
        header: BlockHeader {
            version: 1,
            previous_block_hash: BlockHash::from_byte_array([0_u8; 32]),
            merkle_root: MerkleRoot::from_byte_array([0_u8; 32]),
            time: 1_231_006_505,
            bits: EASY_BITS,
            nonce: 0,
        },
        transactions: vec![coinbase_with_witness.clone(), witness_spend],
    };
    let witness_root = witness_merkle_root(&block);
    let mut commitment_preimage = [0_u8; 64];
    commitment_preimage[..32].copy_from_slice(witness_root.as_bytes());
    commitment_preimage[32..].copy_from_slice(&coinbase_with_witness.inputs[0].witness.stack()[0]);
    let commitment = open_bitcoin_consensus::crypto::double_sha256(&commitment_preimage);
    block.transactions[0].outputs.push(TransactionOutput {
        value: Amount::from_sats(0).expect("zero amount"),
        script_pubkey: script(
            &[&[0x6a, 0x24, 0xaa, 0x21, 0xa9, 0xed][..], &commitment[..]].concat(),
        ),
    });
    let (merkle_root, _) =
        open_bitcoin_consensus::block_merkle_root(&block.transactions).expect("merkle root");
    block.header.merkle_root = merkle_root;
    mine_header(&mut block);

    let block_context = BlockValidationContext {
        height: 1,
        previous_header: BlockHeader {
            bits: block.header.bits,
            time: block.header.time - 1,
            ..BlockHeader::default()
        },
        maybe_retarget_anchor: None,
        previous_median_time_past: i64::from(block.header.time) - 1,
        current_time: i64::from(block.header.time),
        consensus_params: ConsensusParams::default(),
    };
    assert!(check_block_contextual(&block, &block_context).is_ok());

    block.transactions[0].outputs[1].script_pubkey =
        script(&[&[0x6a, 0x24, 0xaa, 0x21, 0xa9, 0xed][..], &[0_u8; 32][..]].concat());
    let (bad_merkle_root, _) =
        open_bitcoin_consensus::block_merkle_root(&block.transactions).expect("merkle root");
    block.header.merkle_root = bad_merkle_root;
    mine_header(&mut block);
    let witness_commitment_error = check_block_contextual(&block, &block_context)
        .expect_err("bad witness commitment must fail");
    assert_eq!(
        witness_commitment_error.reject_reason,
        "bad-witness-merkle-match"
    );

    let unexpected_witness_tx = Transaction {
        version: 1,
        inputs: vec![TransactionInput {
            previous_output: OutPoint {
                txid: Txid::from_byte_array([8_u8; 32]),
                vout: 0,
            },
            script_sig: script(&[0x51]),
            sequence: TransactionInput::SEQUENCE_FINAL,
            witness: ScriptWitness::new(vec![vec![1_u8]]),
        }],
        outputs: vec![TransactionOutput {
            value: Amount::from_sats(1).expect("valid amount"),
            script_pubkey: script(&[0x51]),
        }],
        lock_time: 0,
    };
    let unexpected_witness_context = TransactionValidationContext {
        inputs: vec![TransactionInputContext {
            spent_output: SpentOutput {
                value: Amount::from_sats(1).expect("valid amount"),
                script_pubkey: script(&[0x51, 0xac]),
                is_coinbase: false,
            },
            created_height: 0,
            created_median_time_past: 0,
        }],
        spend_height: 1,
        block_time: 0,
        median_time_past: 0,
        verify_flags: ScriptVerifyFlags::P2SH | ScriptVerifyFlags::WITNESS,
        consensus_params: ConsensusParams::default(),
    };
    let unexpected_witness_error =
        validate_transaction_with_context(&unexpected_witness_tx, &unexpected_witness_context)
            .expect_err("unexpected witness must fail");
    assert_eq!(
        unexpected_witness_error.reject_reason,
        "mandatory-script-verify-flag-failed"
    );
}
