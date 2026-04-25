// Parity breadcrumbs:
// - packages/bitcoin-knots/src/consensus/tx_check.cpp
// - packages/bitcoin-knots/src/consensus/tx_verify.cpp
// - packages/bitcoin-knots/src/consensus/validation.h
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/test/data/tx_valid.json
// - packages/bitcoin-knots/src/test/data/tx_invalid.json

use core::fmt;
use core::ops::{BitOr, BitOrAssign};

use open_bitcoin_codec::CodecError;
use open_bitcoin_primitives::{
    Amount, BlockHeader, Hash32, MAX_MONEY, OutPoint, ScriptBuf, Transaction, TransactionInput,
    TransactionOutput,
};

use crate::crypto::double_sha256;
use crate::validation::{TxValidationError, TxValidationResult, tx_error};

const DEFAULT_COINBASE_MATURITY: u32 = 100;
const DEFAULT_SUBSIDY_HALVING_INTERVAL: u32 = 210_000;
const LOCKTIME_TIMESTAMP_THRESHOLD: u32 = 500_000_000;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConsensusParams {
    pub coinbase_maturity: u32,
    /// Number of blocks between subsidy halvings.
    pub subsidy_halving_interval: u32,
    pub locktime_threshold: u32,
    pub sequence_locktime_granularity: u32,
    pub pow_limit_bits: u32,
    pub pow_target_spacing_seconds: i64,
    pub pow_target_timespan_seconds: i64,
    pub allow_min_difficulty_blocks: bool,
    pub no_pow_retargeting: bool,
    pub enforce_bip34_height_in_coinbase: bool,
    pub enforce_bip113_median_time_past: bool,
    pub enforce_segwit: bool,
}

impl Default for ConsensusParams {
    fn default() -> Self {
        Self {
            coinbase_maturity: DEFAULT_COINBASE_MATURITY,
            subsidy_halving_interval: DEFAULT_SUBSIDY_HALVING_INTERVAL,
            locktime_threshold: LOCKTIME_TIMESTAMP_THRESHOLD,
            sequence_locktime_granularity: TransactionInput::SEQUENCE_LOCKTIME_GRANULARITY as u32,
            pow_limit_bits: 0x207f_ffff,
            pow_target_spacing_seconds: 600,
            pow_target_timespan_seconds: 1_209_600,
            allow_min_difficulty_blocks: true,
            no_pow_retargeting: true,
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
    pub maybe_retarget_anchor: Option<RetargetAnchor>,
    pub maybe_min_difficulty_recovery_target: Option<MinDifficultyRecoveryTarget>,
    pub previous_median_time_past: i64,
    pub current_time: i64,
    pub consensus_params: ConsensusParams,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RetargetAnchor {
    pub first_block_time: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MinDifficultyRecoveryTarget {
    pub bits: u32,
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

        let mut prevouts = Vec::with_capacity(transaction.inputs.len() * OutPoint::SERIALIZED_LEN);
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
        .any(|input| input.sequence != TransactionInput::SEQUENCE_FINAL)
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
        return Ok(Amount::ZERO);
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
    amount_from_checked_fee(fee)
}

fn amount_from_checked_fee(fee: i64) -> Result<Amount, TxValidationError> {
    Amount::from_sats(fee).map_err(|_| {
        tx_error(
            TxValidationResult::Consensus,
            "bad-txns-inputvalues-outofrange",
            None,
        )
    })
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
mod tests;
