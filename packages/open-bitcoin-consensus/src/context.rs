use core::fmt;
use core::ops::{BitOr, BitOrAssign};

use open_bitcoin_codec::CodecError;
use open_bitcoin_primitives::{
    Amount, BlockHeader, Hash32, MAX_MONEY, ScriptBuf, Transaction, TransactionOutput,
};

use crate::crypto::double_sha256;
use crate::validation::{TxValidationError, TxValidationResult, tx_error};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConsensusParams {
    pub coinbase_maturity: u32,
    pub locktime_threshold: u32,
    pub sequence_locktime_granularity: u32,
    pub enforce_bip34_height_in_coinbase: bool,
    pub enforce_bip113_median_time_past: bool,
    pub enforce_segwit: bool,
}

impl Default for ConsensusParams {
    fn default() -> Self {
        Self {
            coinbase_maturity: 100,
            locktime_threshold: 500_000_000,
            sequence_locktime_granularity: 9,
            enforce_bip34_height_in_coinbase: true,
            enforce_bip113_median_time_past: true,
            enforce_segwit: true,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ScriptVerifyFlags(u32);

impl ScriptVerifyFlags {
    pub const NONE: Self = Self(0);
    pub const P2SH: Self = Self(1 << 0);
    pub const STRICTENC: Self = Self(1 << 1);
    pub const DERSIG: Self = Self(1 << 2);
    pub const LOW_S: Self = Self(1 << 3);
    pub const NULLDUMMY: Self = Self(1 << 4);
    pub const SIGPUSHONLY: Self = Self(1 << 5);
    pub const MINIMALDATA: Self = Self(1 << 6);
    pub const DISCOURAGE_UPGRADABLE_NOPS: Self = Self(1 << 7);
    pub const CLEANSTACK: Self = Self(1 << 8);
    pub const CHECKLOCKTIMEVERIFY: Self = Self(1 << 9);
    pub const CHECKSEQUENCEVERIFY: Self = Self(1 << 10);
    pub const WITNESS: Self = Self(1 << 11);
    pub const DISCOURAGE_UPGRADABLE_WITNESS_PROGRAM: Self = Self(1 << 12);
    pub const MINIMALIF: Self = Self(1 << 13);
    pub const NULLFAIL: Self = Self(1 << 14);
    pub const WITNESS_PUBKEYTYPE: Self = Self(1 << 15);
    pub const CONST_SCRIPTCODE: Self = Self(1 << 16);
    pub const TAPROOT: Self = Self(1 << 17);
    pub const DISCOURAGE_UPGRADABLE_TAPROOT_VERSION: Self = Self(1 << 18);
    pub const DISCOURAGE_OP_SUCCESS: Self = Self(1 << 19);
    pub const DISCOURAGE_UPGRADABLE_PUBKEYTYPE: Self = Self(1 << 20);

    pub const fn bits(self) -> u32 {
        self.0
    }

    pub const fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }
}

impl BitOr for ScriptVerifyFlags {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl BitOrAssign for ScriptVerifyFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl fmt::Display for ScriptVerifyFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "0x{:x}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpentOutput {
    pub value: Amount,
    pub script_pubkey: ScriptBuf,
    pub is_coinbase: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransactionInputContext {
    pub spent_output: SpentOutput,
    pub created_height: u32,
    pub created_median_time_past: i64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransactionValidationContext {
    pub inputs: Vec<TransactionInputContext>,
    pub spend_height: u32,
    pub block_time: i64,
    pub median_time_past: i64,
    pub verify_flags: ScriptVerifyFlags,
    pub consensus_params: ConsensusParams,
}

impl TransactionValidationContext {
    pub fn precompute(
        &self,
        transaction: &Transaction,
    ) -> Result<PrecomputedTransactionData, CodecError> {
        PrecomputedTransactionData::new(transaction, &self.inputs)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlockValidationContext {
    pub height: u32,
    pub previous_header: BlockHeader,
    pub previous_median_time_past: i64,
    pub consensus_params: ConsensusParams,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ScriptExecutionData {
    pub maybe_tapleaf_hash: Option<Hash32>,
    pub maybe_codeseparator_position: Option<u32>,
    pub maybe_annex: Option<Vec<u8>>,
    pub maybe_validation_weight_left: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrecomputedTransactionData {
    pub hash_prevouts: Hash32,
    pub hash_sequence: Hash32,
    pub hash_outputs: Hash32,
    pub maybe_spent_amounts_hash: Option<Hash32>,
    pub maybe_spent_scripts_hash: Option<Hash32>,
}

impl PrecomputedTransactionData {
    pub fn new(
        transaction: &Transaction,
        inputs: &[TransactionInputContext],
    ) -> Result<Self, CodecError> {
        if !inputs.is_empty() && inputs.len() != transaction.inputs.len() {
            return Err(CodecError::LengthOutOfRange {
                field: "spent_outputs",
                value: inputs.len() as u64,
            });
        }

        let mut prevouts = Vec::with_capacity(transaction.inputs.len() * 36);
        let mut sequences = Vec::with_capacity(transaction.inputs.len() * 4);
        for input in &transaction.inputs {
            prevouts.extend_from_slice(input.previous_output.txid.as_bytes());
            prevouts.extend_from_slice(&input.previous_output.vout.to_le_bytes());
            sequences.extend_from_slice(&input.sequence.to_le_bytes());
        }

        let mut outputs = Vec::new();
        for output in &transaction.outputs {
            encode_transaction_output(&mut outputs, output);
        }

        let (maybe_spent_amounts_hash, maybe_spent_scripts_hash) = if inputs.is_empty() {
            (None, None)
        } else {
            let mut spent_amounts = Vec::with_capacity(inputs.len() * 8);
            let mut spent_scripts = Vec::new();
            for input in inputs {
                spent_amounts.extend_from_slice(&input.spent_output.value.to_sats().to_le_bytes());
                encode_script_buf(&mut spent_scripts, &input.spent_output.script_pubkey);
            }

            (
                Some(Hash32::from_byte_array(double_sha256(&spent_amounts))),
                Some(Hash32::from_byte_array(double_sha256(&spent_scripts))),
            )
        };

        Ok(Self {
            hash_prevouts: Hash32::from_byte_array(double_sha256(&prevouts)),
            hash_sequence: Hash32::from_byte_array(double_sha256(&sequences)),
            hash_outputs: Hash32::from_byte_array(double_sha256(&outputs)),
            maybe_spent_amounts_hash,
            maybe_spent_scripts_hash,
        })
    }
}

pub fn is_final_transaction(
    transaction: &Transaction,
    spend_height: u32,
    block_time: i64,
    consensus_params: &ConsensusParams,
) -> bool {
    if transaction.lock_time == 0 {
        return true;
    }

    let lock_time = i64::from(transaction.lock_time);
    let comparison_target = if lock_time < i64::from(consensus_params.locktime_threshold) {
        i64::from(spend_height)
    } else {
        block_time
    };
    if lock_time < comparison_target {
        return true;
    }

    !transaction
        .inputs
        .iter()
        .any(|input| input.sequence != open_bitcoin_primitives::TransactionInput::SEQUENCE_FINAL)
}

pub fn calculate_sequence_locks(
    transaction: &Transaction,
    context: &TransactionValidationContext,
) -> Result<(i64, i64), TxValidationError> {
    if context.inputs.len() != transaction.inputs.len() {
        return Err(tx_error(
            TxValidationResult::MissingInputs,
            "bad-txns-inputs-missingorspent",
            None,
        ));
    }
    if transaction.version < 2
        || !context
            .verify_flags
            .contains(ScriptVerifyFlags::CHECKSEQUENCEVERIFY)
    {
        return Ok((-1, -1));
    }

    let mut min_height = -1_i64;
    let mut min_time = -1_i64;
    for (input, input_context) in transaction.inputs.iter().zip(&context.inputs) {
        if (input.sequence
            & open_bitcoin_primitives::TransactionInput::SEQUENCE_LOCKTIME_DISABLE_FLAG)
            != 0
        {
            continue;
        }

        if (input.sequence & open_bitcoin_primitives::TransactionInput::SEQUENCE_LOCKTIME_TYPE_FLAG)
            != 0
        {
            let relative = i64::from(
                (input.sequence
                    & open_bitcoin_primitives::TransactionInput::SEQUENCE_LOCKTIME_MASK)
                    << open_bitcoin_primitives::TransactionInput::SEQUENCE_LOCKTIME_GRANULARITY,
            );
            min_time = min_time.max(input_context.created_median_time_past + relative - 1);
        } else {
            min_height = min_height.max(
                i64::from(input_context.created_height)
                    + i64::from(
                        input.sequence
                            & open_bitcoin_primitives::TransactionInput::SEQUENCE_LOCKTIME_MASK,
                    )
                    - 1,
            );
        }
    }

    Ok((min_height, min_time))
}

pub fn evaluate_sequence_locks(
    context: &TransactionValidationContext,
    lock_pair: (i64, i64),
) -> bool {
    lock_pair.0 < i64::from(context.spend_height) && lock_pair.1 < context.median_time_past
}

pub fn sequence_locks(
    transaction: &Transaction,
    context: &TransactionValidationContext,
) -> Result<bool, TxValidationError> {
    Ok(evaluate_sequence_locks(
        context,
        calculate_sequence_locks(transaction, context)?,
    ))
}

pub fn check_tx_inputs(
    transaction: &Transaction,
    context: &TransactionValidationContext,
) -> Result<Amount, TxValidationError> {
    if transaction.is_coinbase() {
        return Ok(Amount::from_sats(0).expect("zero fee is always in range"));
    }

    if context.inputs.len() != transaction.inputs.len() {
        return Err(tx_error(
            TxValidationResult::MissingInputs,
            "bad-txns-inputs-missingorspent",
            None,
        ));
    }

    let mut total_input_value = 0_i64;
    for input_context in &context.inputs {
        if input_context.spent_output.is_coinbase
            && context
                .spend_height
                .saturating_sub(input_context.created_height)
                < context.consensus_params.coinbase_maturity
        {
            return Err(tx_error(
                TxValidationResult::PrematureSpend,
                "bad-txns-premature-spend-of-coinbase",
                Some(format!(
                    "tried to spend coinbase at depth {}",
                    context
                        .spend_height
                        .saturating_sub(input_context.created_height)
                )),
            ));
        }

        let value = input_context.spent_output.value.to_sats();
        total_input_value += value;
        if !(0..=MAX_MONEY).contains(&value) || !(0..=MAX_MONEY).contains(&total_input_value) {
            return Err(tx_error(
                TxValidationResult::Consensus,
                "bad-txns-inputvalues-outofrange",
                None,
            ));
        }
    }

    let total_output_value: i64 = transaction
        .outputs
        .iter()
        .map(|output| output.value.to_sats())
        .sum();
    if total_input_value < total_output_value {
        return Err(tx_error(
            TxValidationResult::Consensus,
            "bad-txns-in-belowout",
            Some(format!(
                "value in ({total_input_value}) < value out ({total_output_value})"
            )),
        ));
    }

    let fee = total_input_value - total_output_value;
    Ok(Amount::from_sats(fee).expect("checked fee must stay within range"))
}

fn encode_transaction_output(out: &mut Vec<u8>, output: &TransactionOutput) {
    out.extend_from_slice(&output.value.to_sats().to_le_bytes());
    encode_script_buf(out, &output.script_pubkey);
}

fn encode_script_buf(out: &mut Vec<u8>, script: &ScriptBuf) {
    write_compact_size(out, script.as_bytes().len() as u64);
    out.extend_from_slice(script.as_bytes());
}

fn write_compact_size(out: &mut Vec<u8>, value: u64) {
    match value {
        0..=252 => out.push(value as u8),
        253..=0xffff => {
            out.push(0xfd);
            out.extend_from_slice(&(value as u16).to_le_bytes());
        }
        0x1_0000..=0xffff_ffff => {
            out.push(0xfe);
            out.extend_from_slice(&(value as u32).to_le_bytes());
        }
        _ => {
            out.push(0xff);
            out.extend_from_slice(&value.to_le_bytes());
        }
    }
}

#[cfg(test)]
mod tests {
    use open_bitcoin_codec::CodecError;
    use open_bitcoin_primitives::{Amount, BlockHeader, ScriptBuf, Transaction};
    use open_bitcoin_primitives::{MAX_MONEY, TransactionInput, TransactionOutput, Txid};

    use super::{
        BlockValidationContext, ConsensusParams, PrecomputedTransactionData, ScriptVerifyFlags,
        SpentOutput, TransactionInputContext, TransactionValidationContext,
        calculate_sequence_locks, check_tx_inputs, evaluate_sequence_locks, is_final_transaction,
        sequence_locks, write_compact_size,
    };

    fn script(bytes: &[u8]) -> ScriptBuf {
        ScriptBuf::from_bytes(bytes.to_vec()).expect("valid script")
    }

    fn sample_transaction() -> Transaction {
        Transaction {
            version: 2,
            inputs: vec![TransactionInput {
                previous_output: open_bitcoin_primitives::OutPoint {
                    txid: Txid::from_byte_array([7_u8; 32]),
                    vout: 0,
                },
                script_sig: script(&[0x51]),
                sequence: 5,
                witness: Default::default(),
            }],
            outputs: vec![TransactionOutput {
                value: Amount::from_sats(40).expect("valid amount"),
                script_pubkey: script(&[0x51]),
            }],
            lock_time: 10,
        }
    }

    fn sample_context() -> TransactionValidationContext {
        TransactionValidationContext {
            inputs: vec![TransactionInputContext {
                spent_output: SpentOutput {
                    value: Amount::from_sats(50).expect("valid amount"),
                    script_pubkey: script(&[0x52, 0x87]),
                    is_coinbase: false,
                },
                created_height: 3,
                created_median_time_past: 1_000,
            }],
            spend_height: 10,
            block_time: 2_000,
            median_time_past: 2_000,
            verify_flags: ScriptVerifyFlags::CHECKSEQUENCEVERIFY,
            consensus_params: ConsensusParams::default(),
        }
    }

    #[test]
    fn finality_uses_locktime_threshold_and_sequences() {
        let mut transaction = sample_transaction();
        let params = ConsensusParams::default();

        assert!(!is_final_transaction(&transaction, 5, 2_000, &params));

        transaction.inputs[0].sequence = TransactionInput::SEQUENCE_FINAL;
        assert!(is_final_transaction(&transaction, 5, 2_000, &params));

        transaction.lock_time = params.locktime_threshold + 1;
        transaction.inputs[0].sequence = 1;
        assert!(!is_final_transaction(
            &transaction,
            10,
            i64::from(params.locktime_threshold),
            &params,
        ));

        transaction.lock_time = 1;
        assert!(is_final_transaction(&transaction, 10, 0, &params));
    }

    #[test]
    fn sequence_lock_helpers_match_height_and_time_modes() {
        let mut transaction = sample_transaction();
        let context = sample_context();

        let locks = calculate_sequence_locks(&transaction, &context).expect("locks");
        assert_eq!(locks, (7, -1));
        assert!(evaluate_sequence_locks(&context, locks));
        assert!(sequence_locks(&transaction, &context).expect("sequence locks"));

        transaction.inputs[0].sequence = TransactionInput::SEQUENCE_LOCKTIME_TYPE_FLAG | 1;
        let locks = calculate_sequence_locks(&transaction, &context).expect("locks");
        assert_eq!(
            locks,
            (
                -1,
                context.inputs[0].created_median_time_past
                    + (1_i64 << TransactionInput::SEQUENCE_LOCKTIME_GRANULARITY)
                    - 1,
            ),
        );
    }

    #[test]
    fn check_tx_inputs_returns_fee_and_rejects_premature_coinbase() {
        let transaction = sample_transaction();
        let fee = check_tx_inputs(&transaction, &sample_context()).expect("fee");
        assert_eq!(fee.to_sats(), 10);

        let mut context = sample_context();
        context.inputs[0].spent_output.is_coinbase = true;
        context.inputs[0].created_height = 9;
        let error = check_tx_inputs(&transaction, &context)
            .expect_err("premature coinbase spend must fail");

        assert_eq!(error.reject_reason, "bad-txns-premature-spend-of-coinbase");
    }

    #[test]
    fn precomputed_transaction_data_hashes_change_with_inputs() {
        let transaction = sample_transaction();
        let data = PrecomputedTransactionData::new(&transaction, &sample_context().inputs)
            .expect("precomputed data");

        assert_ne!(data.hash_prevouts.to_byte_array(), [0_u8; 32]);
        assert_ne!(data.hash_sequence.to_byte_array(), [0_u8; 32]);
        assert_ne!(data.hash_outputs.to_byte_array(), [0_u8; 32]);
        assert!(data.maybe_spent_amounts_hash.is_some());
        assert!(data.maybe_spent_scripts_hash.is_some());
    }

    #[test]
    fn flag_helpers_and_precompute_edge_cases_are_exercised() {
        let mut flags = ScriptVerifyFlags::P2SH | ScriptVerifyFlags::WITNESS;
        flags |= ScriptVerifyFlags::CHECKSEQUENCEVERIFY;

        assert!(flags.contains(ScriptVerifyFlags::P2SH));
        assert!(flags.contains(ScriptVerifyFlags::WITNESS));
        assert!(flags.contains(ScriptVerifyFlags::CHECKSEQUENCEVERIFY));
        assert_eq!(flags.bits(), 0xc01);
        assert_eq!(flags.to_string(), "0xc01");

        let transaction = sample_transaction();
        let empty_data =
            PrecomputedTransactionData::new(&transaction, &[]).expect("empty inputs are allowed");
        assert!(empty_data.maybe_spent_amounts_hash.is_none());
        assert!(empty_data.maybe_spent_scripts_hash.is_none());

        let error = PrecomputedTransactionData::new(
            &transaction,
            &[
                sample_context().inputs[0].clone(),
                sample_context().inputs[0].clone(),
            ],
        )
        .expect_err("mismatched inputs must fail");
        assert!(matches!(
            error,
            CodecError::LengthOutOfRange {
                field: "spent_outputs",
                value: 2
            }
        ));
    }

    #[test]
    fn finality_short_circuit_and_sequence_lock_edge_cases_are_covered() {
        let mut transaction = sample_transaction();
        let params = ConsensusParams::default();

        transaction.lock_time = 0;
        assert!(is_final_transaction(&transaction, 1, 0, &params));

        let mut context = sample_context();
        let error = calculate_sequence_locks(
            &transaction,
            &TransactionValidationContext {
                inputs: vec![],
                ..context.clone()
            },
        )
        .expect_err("missing inputs must fail");
        assert_eq!(error.reject_reason, "bad-txns-inputs-missingorspent");

        transaction.inputs[0].sequence = TransactionInput::SEQUENCE_LOCKTIME_DISABLE_FLAG | 1;
        let locks = calculate_sequence_locks(&transaction, &context).expect("locks");
        assert_eq!(locks, (-1, -1));

        context.spend_height = 7;
        assert!(!evaluate_sequence_locks(&context, (7, -1)));
        assert!(!evaluate_sequence_locks(
            &context,
            (-1, context.median_time_past)
        ));
    }

    #[test]
    fn check_tx_inputs_covers_mismatch_and_value_failures() {
        let transaction = sample_transaction();
        let error = check_tx_inputs(
            &transaction,
            &TransactionValidationContext {
                inputs: vec![],
                ..sample_context()
            },
        )
        .expect_err("missing inputs must fail");
        assert_eq!(error.reject_reason, "bad-txns-inputs-missingorspent");

        let coinbase_fee = check_tx_inputs(
            &Transaction {
                version: 1,
                inputs: vec![TransactionInput {
                    previous_output: open_bitcoin_primitives::OutPoint::null(),
                    script_sig: script(&[0x01, 0x01]),
                    sequence: TransactionInput::SEQUENCE_FINAL,
                    witness: Default::default(),
                }],
                outputs: vec![TransactionOutput {
                    value: Amount::from_sats(1).expect("valid amount"),
                    script_pubkey: script(&[0x51]),
                }],
                lock_time: 0,
            },
            &sample_context(),
        )
        .expect("coinbase fee should be zero");
        assert_eq!(coinbase_fee.to_sats(), 0);

        let overflow_context = TransactionValidationContext {
            inputs: vec![
                TransactionInputContext {
                    spent_output: SpentOutput {
                        value: Amount::from_sats(MAX_MONEY).expect("max money"),
                        script_pubkey: script(&[0x51]),
                        is_coinbase: false,
                    },
                    created_height: 1,
                    created_median_time_past: 0,
                },
                TransactionInputContext {
                    spent_output: SpentOutput {
                        value: Amount::from_sats(MAX_MONEY).expect("max money"),
                        script_pubkey: script(&[0x51]),
                        is_coinbase: false,
                    },
                    created_height: 1,
                    created_median_time_past: 0,
                },
            ],
            ..sample_context()
        };
        let overflow_transaction = Transaction {
            version: 2,
            inputs: vec![
                sample_transaction().inputs[0].clone(),
                TransactionInput {
                    previous_output: open_bitcoin_primitives::OutPoint {
                        txid: Txid::from_byte_array([8_u8; 32]),
                        vout: 1,
                    },
                    script_sig: script(&[0x51]),
                    sequence: 0,
                    witness: Default::default(),
                },
            ],
            outputs: vec![TransactionOutput {
                value: Amount::from_sats(1).expect("valid amount"),
                script_pubkey: script(&[0x51]),
            }],
            lock_time: 0,
        };
        assert_eq!(
            check_tx_inputs(&overflow_transaction, &overflow_context)
                .expect_err("overflowing inputs must fail")
                .reject_reason,
            "bad-txns-inputvalues-outofrange",
        );

        let overspend_context = sample_context();
        let overspend_transaction = Transaction {
            outputs: vec![TransactionOutput {
                value: Amount::from_sats(60).expect("valid amount"),
                script_pubkey: script(&[0x51]),
            }],
            ..sample_transaction()
        };
        assert_eq!(
            check_tx_inputs(&overspend_transaction, &overspend_context)
                .expect_err("overspend must fail")
                .reject_reason,
            "bad-txns-in-belowout",
        );

        let mut compact = Vec::new();
        write_compact_size(&mut compact, 253);
        write_compact_size(&mut compact, 65_536);
        write_compact_size(&mut compact, u64::MAX);
        assert_eq!(compact[0], 0xfd);
        assert_eq!(compact[3], 0xfe);
        assert_eq!(compact[8], 0xff);
    }

    #[test]
    fn block_context_carries_expected_defaults() {
        let context = BlockValidationContext {
            height: 11,
            previous_header: BlockHeader::default(),
            previous_median_time_past: 1_000,
            consensus_params: ConsensusParams::default(),
        };

        assert_eq!(context.height, 11);
        assert!(context.consensus_params.enforce_segwit);
    }
}
