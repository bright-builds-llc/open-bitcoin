// Parity breadcrumbs:
// - packages/bitcoin-knots/src/blockencodings.cpp
// - packages/bitcoin-knots/src/blockencodings.h
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/test/functional/p2p_compactblocks.py

use std::collections::HashMap;

use open_bitcoin_codec::{
    CompactBlockPayload, expand_prefilled_positions, short_id_match_key,
    short_id_selector_from_header_and_nonce,
};
use open_bitcoin_consensus::{
    MAX_BLOCK_WEIGHT, block_hash, compact_short_id_for_wtxid, transaction_wtxid,
};
use open_bitcoin_primitives::{Block, BlockHash, BlockHeader, Transaction, Wtxid};

#[cfg(test)]
mod tests;

const MIN_SERIALIZABLE_TRANSACTION_WEIGHT: usize = 40;
pub(crate) const MAX_COMPACT_BLOCK_TRANSACTION_COUNT: usize =
    MAX_BLOCK_WEIGHT / MIN_SERIALIZABLE_TRANSACTION_WEIGHT;
const MAX_SHORT_ID_BUCKET_SIZE: u16 = 12;
#[cfg(test)]
const TEST_PREFILLED_WTXID_FAILURE_LOCK_TIME: u32 = 0x0114_1144;
#[cfg(test)]
const TEST_APPLY_WTXID_FAILURE_LOCK_TIME: u32 = 0x0115_1155;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PartialCompactBlock {
    header: Option<BlockHeader>,
    txn_available: Vec<Option<Transaction>>,
    slot_wtxids: Vec<Option<Wtxid>>,
    prefilled_count: usize,
    short_id_slots_remaining: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CompactReconstructionOutcome {
    Ready { missing_indexes: Vec<u16> },
    Invalid(CompactReconstructionInvalidReason),
    Failed(CompactReconstructionFailureReason),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompactReconstructionInvalidReason {
    NullHeader,
    EmptyCompactBlock,
    AlreadyInitialized,
    TransactionCountOutOfRange,
    NullPrefilledTransaction,
    PrefilledIndexOutOfBounds,
    MalformedPrefilledIndex,
    IncompleteTransactions,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompactBlockTxnMisbehavior {
    UnexpectedBlockHash,
    DuplicateResponse,
    OutOfBoundsIndex,
    TooManyTransactions,
    NotInitialized,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CompactBlockTxnOutcome {
    Applied { still_missing: Vec<u16> },
    Misbehavior(CompactBlockTxnMisbehavior),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompactReconstructionFailureReason {
    ShortIdCollision,
    ShortIdBucketOverload,
}

impl PartialCompactBlock {
    pub fn new() -> Self {
        Self {
            header: None,
            txn_available: Vec::new(),
            slot_wtxids: Vec::new(),
            prefilled_count: 0,
            short_id_slots_remaining: 0,
        }
    }

    pub fn is_initialized(&self) -> bool {
        self.header.is_some()
    }

    pub fn header(&self) -> Option<&BlockHeader> {
        self.header.as_ref()
    }

    pub fn transaction_count(&self) -> usize {
        self.txn_available.len()
    }

    pub fn is_transaction_available(&self, index: usize) -> bool {
        if self.header.is_none() {
            return false;
        }

        self.txn_available
            .get(index)
            .is_some_and(|maybe_tx| maybe_tx.is_some())
    }

    pub fn missing_transaction_indexes(&self) -> Vec<u16> {
        if self.header.is_none() {
            return Vec::new();
        }

        self.txn_available
            .iter()
            .enumerate()
            .filter_map(|(index, maybe_tx)| {
                if maybe_tx.is_none() {
                    u16::try_from(index).ok()
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn on_mempool_transaction_removed(&mut self, removed_wtxid: &Wtxid) {
        let Some(_) = self.header else {
            return;
        };

        for (index, maybe_slot_wtxid) in self.slot_wtxids.iter_mut().enumerate() {
            if maybe_slot_wtxid.as_ref() == Some(removed_wtxid) {
                self.txn_available[index] = None;
                *maybe_slot_wtxid = None;
            }
        }
    }

    pub fn on_block_connected(&mut self) {
        self.clear();
    }

    pub fn clear(&mut self) {
        self.header = None;
        self.txn_available.clear();
        self.slot_wtxids.clear();
        self.prefilled_count = 0;
        self.short_id_slots_remaining = 0;
    }

    pub fn block_hash(&self) -> Option<BlockHash> {
        self.header.as_ref().map(block_hash)
    }
}

impl Default for PartialCompactBlock {
    fn default() -> Self {
        Self::new()
    }
}

pub fn init_partial_compact_block<'a>(
    state: &mut PartialCompactBlock,
    payload: &CompactBlockPayload,
    candidates: impl IntoIterator<Item = (&'a Wtxid, &'a Transaction)>,
    extra_transactions: impl IntoIterator<Item = (&'a Wtxid, &'a Transaction)>,
) -> CompactReconstructionOutcome {
    if payload.header.is_null() {
        return CompactReconstructionOutcome::Invalid(
            CompactReconstructionInvalidReason::NullHeader,
        );
    }

    if payload.short_ids.is_empty() && payload.prefilled_transactions.is_empty() {
        return CompactReconstructionOutcome::Invalid(
            CompactReconstructionInvalidReason::EmptyCompactBlock,
        );
    }

    let short_id_count = payload.short_ids.len();
    let prefilled_count = payload.prefilled_transactions.len();
    if short_id_count > MAX_COMPACT_BLOCK_TRANSACTION_COUNT
        || prefilled_count > MAX_COMPACT_BLOCK_TRANSACTION_COUNT
    {
        return CompactReconstructionOutcome::Invalid(
            CompactReconstructionInvalidReason::TransactionCountOutOfRange,
        );
    }

    let transaction_count = short_id_count + prefilled_count;
    if transaction_count > MAX_COMPACT_BLOCK_TRANSACTION_COUNT {
        return CompactReconstructionOutcome::Invalid(
            CompactReconstructionInvalidReason::TransactionCountOutOfRange,
        );
    }

    if state.is_initialized() {
        return CompactReconstructionOutcome::Invalid(
            CompactReconstructionInvalidReason::AlreadyInitialized,
        );
    }

    let prefilled_positions = match expand_prefilled_positions(payload) {
        Ok(positions) => positions,
        Err(_) => {
            return CompactReconstructionOutcome::Invalid(
                CompactReconstructionInvalidReason::MalformedPrefilledIndex,
            );
        }
    };

    state.header = Some(payload.header.clone());
    state.txn_available = vec![None; transaction_count];
    state.slot_wtxids = vec![None; transaction_count];
    state.prefilled_count = payload.prefilled_transactions.len();
    state.short_id_slots_remaining = payload.short_ids.len();

    for (prefilled_index, position) in prefilled_positions.iter().copied().enumerate() {
        let transaction = &payload.prefilled_transactions[prefilled_index].transaction;
        if transaction.inputs.is_empty() || transaction.outputs.is_empty() {
            state.clear();
            return CompactReconstructionOutcome::Invalid(
                CompactReconstructionInvalidReason::NullPrefilledTransaction,
            );
        }

        let max_position = payload.short_ids.len() + prefilled_index;
        if usize::from(position) > max_position {
            state.clear();
            return CompactReconstructionOutcome::Invalid(
                CompactReconstructionInvalidReason::PrefilledIndexOutOfBounds,
            );
        }

        let slot = usize::from(position);
        let wtxid = match prefilled_wtxid(transaction) {
            Ok(wtxid) => wtxid,
            Err(_) => {
                state.clear();
                return CompactReconstructionOutcome::Invalid(
                    CompactReconstructionInvalidReason::NullPrefilledTransaction,
                );
            }
        };
        state.txn_available[slot] = Some(transaction.clone());
        state.slot_wtxids[slot] = Some(wtxid);
    }

    let selector = short_id_selector_from_header_and_nonce(&payload.header, payload.nonce);
    let short_id_map = match build_short_id_map(payload, &state.txn_available) {
        Ok(map) => map,
        Err(reason) => {
            state.clear();
            return CompactReconstructionOutcome::Failed(reason);
        }
    };

    let mut matched_short_ids = 0_usize;
    scan_candidate_transactions(
        state,
        &selector,
        &short_id_map,
        candidates,
        &mut matched_short_ids,
        false,
    );
    scan_candidate_transactions(
        state,
        &selector,
        &short_id_map,
        extra_transactions,
        &mut matched_short_ids,
        true,
    );

    let missing_indexes = state.missing_transaction_indexes();
    CompactReconstructionOutcome::Ready { missing_indexes }
}

pub fn apply_block_transactions(
    state: &mut PartialCompactBlock,
    response: &open_bitcoin_codec::BlockTransactions,
    expected_block_hash: BlockHash,
) -> CompactBlockTxnOutcome {
    if !state.is_initialized() {
        return CompactBlockTxnOutcome::Misbehavior(CompactBlockTxnMisbehavior::NotInitialized);
    }

    if response.block_hash != expected_block_hash {
        return CompactBlockTxnOutcome::Misbehavior(
            CompactBlockTxnMisbehavior::UnexpectedBlockHash,
        );
    }

    let missing_indexes = state.missing_transaction_indexes();
    if missing_indexes.is_empty() && !response.transactions.is_empty() {
        return CompactBlockTxnOutcome::Misbehavior(CompactBlockTxnMisbehavior::DuplicateResponse);
    }

    if response.transactions.len() > missing_indexes.len() {
        return CompactBlockTxnOutcome::Misbehavior(
            CompactBlockTxnMisbehavior::TooManyTransactions,
        );
    }

    for (index, transaction) in missing_indexes
        .iter()
        .copied()
        .zip(response.transactions.iter())
    {
        if let Err(reason) = apply_downloaded_transaction_at_index(state, index, transaction) {
            return CompactBlockTxnOutcome::Misbehavior(reason);
        }
    }

    CompactBlockTxnOutcome::Applied {
        still_missing: state.missing_transaction_indexes(),
    }
}

fn apply_downloaded_transaction_at_index(
    state: &mut PartialCompactBlock,
    index: u16,
    transaction: &Transaction,
) -> Result<(), CompactBlockTxnMisbehavior> {
    let slot = usize::from(index);
    if slot >= state.txn_available.len() {
        return Err(CompactBlockTxnMisbehavior::OutOfBoundsIndex);
    }

    if state.txn_available[slot].is_some() {
        return Err(CompactBlockTxnMisbehavior::DuplicateResponse);
    }

    if transaction.inputs.is_empty() || transaction.outputs.is_empty() {
        return Err(CompactBlockTxnMisbehavior::OutOfBoundsIndex);
    }

    let wtxid = decoded_wtxid_for_apply(transaction)?;

    state.txn_available[slot] = Some(transaction.clone());
    state.slot_wtxids[slot] = Some(wtxid);
    Ok(())
}

fn decoded_wtxid_for_apply(transaction: &Transaction) -> Result<Wtxid, CompactBlockTxnMisbehavior> {
    #[cfg(test)]
    if transaction.lock_time == TEST_APPLY_WTXID_FAILURE_LOCK_TIME {
        return Err(CompactBlockTxnMisbehavior::OutOfBoundsIndex);
    }

    transaction_wtxid(transaction).map_err(|_| CompactBlockTxnMisbehavior::OutOfBoundsIndex)
}

pub fn fill_block(
    state: &PartialCompactBlock,
) -> Result<Block, CompactReconstructionInvalidReason> {
    let header = state
        .header
        .as_ref()
        .ok_or(CompactReconstructionInvalidReason::NullHeader)?
        .clone();

    if state.missing_transaction_indexes().is_empty() {
        let transactions = state
            .txn_available
            .iter()
            .map(|maybe_tx| {
                maybe_tx
                    .clone()
                    .ok_or(CompactReconstructionInvalidReason::IncompleteTransactions)
            })
            .collect::<Result<Vec<_>, _>>()?;
        return Ok(Block {
            header,
            transactions,
        });
    }

    Err(CompactReconstructionInvalidReason::IncompleteTransactions)
}

fn build_short_id_map(
    payload: &CompactBlockPayload,
    txn_available: &[Option<Transaction>],
) -> Result<HashMap<u64, u16>, CompactReconstructionFailureReason> {
    let mut short_id_map = HashMap::with_capacity(payload.short_ids.len());
    let bucket_count = u64::try_from(payload.short_ids.len().max(1))
        .map_err(|_| CompactReconstructionFailureReason::ShortIdBucketOverload)?;
    let mut bucket_sizes: HashMap<u64, u16> = HashMap::new();

    let mut index_offset = 0_u16;
    for short_index in 0..payload.short_ids.len() {
        while txn_available
            .get(short_index + usize::from(index_offset))
            .is_some_and(Option::is_some)
        {
            index_offset = index_offset.saturating_add(1);
        }

        let absolute_index = u16::try_from(short_index)
            .ok()
            .and_then(|base| base.checked_add(index_offset))
            .ok_or(CompactReconstructionFailureReason::ShortIdCollision)?;

        let short_id = payload.short_ids[short_index];
        let match_key = short_id_match_key(short_id);
        let bucket = match_key % bucket_count;
        let bucket_size = bucket_sizes.entry(bucket).or_insert(0);
        *bucket_size = bucket_size.saturating_add(1);
        if *bucket_size > MAX_SHORT_ID_BUCKET_SIZE {
            return Err(CompactReconstructionFailureReason::ShortIdBucketOverload);
        }

        if short_id_map.insert(match_key, absolute_index).is_some() {
            return Err(CompactReconstructionFailureReason::ShortIdCollision);
        }
    }

    Ok(short_id_map)
}

fn prefilled_wtxid(transaction: &Transaction) -> Result<Wtxid, open_bitcoin_codec::CodecError> {
    #[cfg(test)]
    if transaction.lock_time == TEST_PREFILLED_WTXID_FAILURE_LOCK_TIME {
        return Err(open_bitcoin_codec::CodecError::CompactBlockEmpty);
    }

    transaction_wtxid(transaction)
}

fn should_clear_duplicate_slot(
    slot_is_filled: bool,
    compare_witness_hash_on_duplicate: bool,
    maybe_slot_wtxid: Option<&Wtxid>,
    candidate_wtxid: &Wtxid,
) -> bool {
    if !slot_is_filled {
        return false;
    }

    if compare_witness_hash_on_duplicate {
        maybe_slot_wtxid != Some(candidate_wtxid)
    } else {
        true
    }
}

pub(crate) fn scan_candidate_transactions<'a>(
    state: &mut PartialCompactBlock,
    selector: &open_bitcoin_codec::ShortIdSelector,
    short_id_map: &HashMap<u64, u16>,
    candidates: impl IntoIterator<Item = (&'a Wtxid, &'a Transaction)>,
    matched_short_ids: &mut usize,
    compare_witness_hash_on_duplicate: bool,
) {
    for (wtxid, transaction) in candidates {
        let short_id = compact_short_id_for_wtxid(*selector, wtxid);
        let match_key = short_id_match_key(short_id);
        let Some(&slot) = short_id_map.get(&match_key) else {
            continue;
        };

        let slot_index = usize::from(slot);
        let Some(_) = state.txn_available.get(slot_index) else {
            continue;
        };

        if state.txn_available[slot_index].is_none() {
            state.txn_available[slot_index] = Some(transaction.clone());
            state.slot_wtxids[slot_index] = Some(*wtxid);
            *matched_short_ids = matched_short_ids.saturating_add(1);
        } else if should_clear_duplicate_slot(
            state.txn_available[slot_index].is_some(),
            compare_witness_hash_on_duplicate,
            state.slot_wtxids[slot_index].as_ref(),
            wtxid,
        ) {
            state.txn_available[slot_index] = None;
            state.slot_wtxids[slot_index] = None;
            *matched_short_ids = matched_short_ids.saturating_sub(1);
        }

        if *matched_short_ids >= state.short_id_slots_remaining {
            break;
        }
    }
}
