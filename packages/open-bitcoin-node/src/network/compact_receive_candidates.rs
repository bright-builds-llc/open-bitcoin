// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/net_processing.h
// - packages/bitcoin-knots/src/blockencodings.cpp
// - packages/bitcoin-knots/test/functional/p2p_compactblocks.py

use open_bitcoin_codec::CompactBlockPayload;
use open_bitcoin_core::primitives::{Transaction, Wtxid};
use open_bitcoin_mempool::{
    Mempool, MempoolError, MempoolRemovalCause, PreparedLifecycleFacts,
    transaction_weight_and_virtual_size,
};
use open_bitcoin_network::{CompactBlockReceiveFacts, PeerAction, PeerId};

use super::lifecycle_projection::PreparedCompactProjection;
use super::{ManagedNetworkError, ManagedPeerNetwork};
use crate::ChainstateStore;

/// Knots `DEFAULT_BLOCK_RECONSTRUCTION_EXTRA_TXN` (`net_processing.h`).
pub const DEFAULT_BLOCK_RECONSTRUCTION_EXTRA_TXN: usize = 32_768;
/// Knots `DEFAULT_BLOCK_RECONSTRUCTION_EXTRA_TXN_SIZE` (`net_processing.h`).
pub const DEFAULT_BLOCK_RECONSTRUCTION_EXTRA_TXN_SIZE: usize = 10_000_000;
/// Knots `BLOCK_RECONSTRUCTION_EXTRA_TXN_PER_TXN_SIZE_LIMIT` (`net_processing.h`).
pub const BLOCK_RECONSTRUCTION_EXTRA_TXN_PER_TXN_SIZE_LIMIT: usize = 100_000;

/// Knots-shaped ring buffer for compact-block reconstruction extras (`vExtraTxnForCompact`).
///
/// Byte accounting uses virtual size as a Knots-aligned bound approximation; it is not
/// required to match `RecursiveDynamicUsage` byte-for-byte.
#[derive(Debug, Clone)]
pub struct CompactExtraTxnBuffer {
    slots: Vec<Option<(Wtxid, Transaction)>>,
    write_cursor: usize,
    approximate_bytes: usize,
    max_slots: usize,
    max_bytes: usize,
    per_tx_size_limit: usize,
}

impl CompactExtraTxnBuffer {
    pub fn new(max_slots: usize, max_bytes: usize, per_tx_size_limit: usize) -> Self {
        Self {
            slots: Vec::new(),
            write_cursor: 0,
            approximate_bytes: 0,
            max_slots,
            max_bytes,
            per_tx_size_limit,
        }
    }

    pub fn with_defaults() -> Self {
        Self::new(
            DEFAULT_BLOCK_RECONSTRUCTION_EXTRA_TXN,
            DEFAULT_BLOCK_RECONSTRUCTION_EXTRA_TXN_SIZE,
            BLOCK_RECONSTRUCTION_EXTRA_TXN_PER_TXN_SIZE_LIMIT,
        )
    }

    /// Insert like Knots `AddToCompactExtraTransactions` (ring overwrite + byte-budget clear).
    pub fn push(&mut self, wtxid: Wtxid, transaction: Transaction) {
        let tx_bytes = approximate_tx_bytes(&transaction);
        self.push_with_size(wtxid, transaction, tx_bytes);
    }

    /// Reject bodies whose approximate size exceeds the per-tx limit without inserting.
    pub fn push_gated(&mut self, wtxid: Wtxid, transaction: Transaction) -> bool {
        let tx_bytes = approximate_tx_bytes(&transaction);
        if tx_bytes > self.per_tx_size_limit {
            return false;
        }
        self.push(wtxid, transaction);
        true
    }

    pub fn iter_available(&self) -> impl Iterator<Item = &(Wtxid, Transaction)> {
        self.slots.iter().filter_map(Option::as_ref)
    }

    pub fn to_owned_pairs(&self) -> Vec<(Wtxid, Transaction)> {
        self.iter_available().cloned().collect()
    }

    #[cfg(test)]
    pub fn approximate_bytes(&self) -> usize {
        self.approximate_bytes
    }

    #[cfg(test)]
    pub fn max_slots(&self) -> usize {
        self.max_slots
    }

    #[cfg(test)]
    pub fn max_bytes(&self) -> usize {
        self.max_bytes
    }

    #[cfg(test)]
    pub fn per_tx_size_limit(&self) -> usize {
        self.per_tx_size_limit
    }

    fn clear_slot_at(&mut self, index: usize) {
        let Some(slot) = self.slots.get_mut(index) else {
            return;
        };
        let Some((_, transaction)) = slot.take() else {
            return;
        };
        let cleared_bytes = approximate_tx_bytes(&transaction);
        self.approximate_bytes = self.approximate_bytes.saturating_sub(cleared_bytes);
    }

    fn clear_wtxid_checked(&mut self, wtxid: Wtxid) -> Result<(), MempoolError> {
        for slot in &mut self.slots {
            let Some((slot_wtxid, transaction)) = slot.as_ref() else {
                continue;
            };
            if *slot_wtxid != wtxid {
                continue;
            }
            let tx_bytes = checked_approximate_tx_bytes(transaction)?;
            slot.take();
            self.approximate_bytes = self.approximate_bytes.saturating_sub(tx_bytes);
        }
        Ok(())
    }

    fn push_checked(&mut self, wtxid: Wtxid, transaction: Transaction) -> Result<(), MempoolError> {
        let tx_bytes = checked_approximate_tx_bytes(&transaction)?;
        self.push_with_size(wtxid, transaction, tx_bytes);
        Ok(())
    }

    fn push_with_size(&mut self, wtxid: Wtxid, transaction: Transaction, tx_bytes: usize) {
        if self.max_slots == 0 {
            return;
        }

        if self.slots.is_empty() {
            self.slots.resize_with(self.max_slots, || None);
        }

        self.clear_slot_at(self.write_cursor);
        self.slots[self.write_cursor] = Some((wtxid, transaction));
        self.approximate_bytes = self.approximate_bytes.saturating_add(tx_bytes);
        self.write_cursor = (self.write_cursor + 1) % self.max_slots;

        while self.approximate_bytes > self.max_bytes {
            self.clear_slot_at(self.write_cursor);
            self.write_cursor = (self.write_cursor + 1) % self.max_slots;
        }
    }
}

/// Snapshot mempool entries as owned `(Wtxid, Transaction)` pairs for compact receive facts.
pub fn mempool_compact_candidate_owned(mempool: &Mempool) -> Vec<(Wtxid, Transaction)> {
    mempool
        .entries()
        .values()
        .map(|entry| (entry.wtxid, entry.transaction.clone()))
        .collect()
}

/// Snapshot non-null extra-buffer slots as owned pairs for facts adaptation.
pub fn compact_extra_owned(buffer: &CompactExtraTxnBuffer) -> Vec<(Wtxid, Transaction)> {
    buffer.to_owned_pairs()
}

fn approximate_tx_bytes(transaction: &Transaction) -> usize {
    match transaction_weight_and_virtual_size(transaction) {
        Ok((_, virtual_size)) => virtual_size,
        Err(_) => 0,
    }
}

fn checked_approximate_tx_bytes(transaction: &Transaction) -> Result<usize, MempoolError> {
    transaction_weight_and_virtual_size(transaction).map(|(_, virtual_size)| virtual_size)
}

type OwnedCompactTxnPairs = Vec<(Wtxid, Transaction)>;

impl<S: ChainstateStore> ManagedPeerNetwork<S> {
    pub(super) fn prepare_compact_projection(
        &self,
        facts: &PreparedLifecycleFacts,
    ) -> Result<PreparedCompactProjection, MempoolError> {
        let mut replacement = self.compact_extra_txn.clone();
        for member in facts.teardown_order() {
            replacement.clear_wtxid_checked(member.wtxid)?;
        }
        for member in facts.final_present() {
            replacement.clear_wtxid_checked(member.member.wtxid)?;
        }
        for removed in facts.removed() {
            if removed.removal.cause != MempoolRemovalCause::Replacement {
                continue;
            }
            replacement.push_checked(removed.removal.member.wtxid, removed.transaction.clone())?;
        }
        Ok(PreparedCompactProjection { replacement })
    }

    #[allow(dead_code)] // Plan 134-05 invokes the closed aggregate apply.
    pub(super) fn apply_prepared_compact(&mut self, prepared: PreparedCompactProjection) {
        self.compact_extra_txn = prepared.replacement;
    }

    /// Snapshot mempool + extra buffer as owned pairs before borrowing PeerManager (D-02/D-04).
    pub(super) fn collect_compact_receive_owned(
        &self,
    ) -> (OwnedCompactTxnPairs, OwnedCompactTxnPairs) {
        let candidates = mempool_compact_candidate_owned(self.mempool.mempool());
        let extras = compact_extra_owned(&self.compact_extra_txn);
        (candidates, extras)
    }

    /// Live CompactBlock receive: inject mempool + extras into `handle_compact_block_download`.
    pub(super) fn handle_compact_block_receive(
        &mut self,
        peer_id: PeerId,
        payload: CompactBlockPayload,
        timestamp: i64,
    ) -> Result<Vec<PeerAction>, ManagedNetworkError> {
        let (candidate_owned, extra_owned) = self.collect_compact_receive_owned();
        let candidate_refs: Vec<(&Wtxid, &Transaction)> = candidate_owned
            .iter()
            .map(|(wtxid, tx)| (wtxid, tx))
            .collect();
        let extra_refs: Vec<(&Wtxid, &Transaction)> =
            extra_owned.iter().map(|(wtxid, tx)| (wtxid, tx)).collect();
        let facts = CompactBlockReceiveFacts {
            candidates: &candidate_refs,
            extra: &extra_refs,
        };
        Ok(self
            .peer_manager
            .handle_compact_block_download(peer_id, payload, facts, timestamp)?)
    }

    #[cfg(test)]
    pub(super) fn compact_extra_txn_len(&self) -> usize {
        self.compact_extra_txn.to_owned_pairs().len()
    }
}

#[cfg(test)]
mod tests {
    use open_bitcoin_core::chainstate::Chainstate;
    use open_bitcoin_core::consensus::{
        ConsensusParams, ScriptVerifyFlags, block_merkle_root, check_block_header,
        transaction_txid, transaction_wtxid,
    };
    use open_bitcoin_core::primitives::{
        Amount, Block, BlockHash, BlockHeader, OutPoint, ScriptBuf, ScriptWitness, Transaction,
        TransactionInput, TransactionOutput, Txid, Wtxid,
    };
    use open_bitcoin_mempool::Mempool;

    use super::{
        BLOCK_RECONSTRUCTION_EXTRA_TXN_PER_TXN_SIZE_LIMIT, CompactExtraTxnBuffer,
        DEFAULT_BLOCK_RECONSTRUCTION_EXTRA_TXN, DEFAULT_BLOCK_RECONSTRUCTION_EXTRA_TXN_SIZE,
        compact_extra_owned, mempool_compact_candidate_owned,
    };

    const EASY_BITS: u32 = 0x207f_ffff;

    fn sample_tx(tag: u8) -> Transaction {
        Transaction {
            version: 2,
            inputs: vec![TransactionInput {
                previous_output: OutPoint {
                    txid: Txid::from_byte_array([tag; 32]),
                    vout: 0,
                },
                script_sig: ScriptBuf::from_bytes(Vec::new()).expect("script"),
                sequence: TransactionInput::SEQUENCE_FINAL,
                witness: ScriptWitness::new(vec![vec![tag]]),
            }],
            outputs: vec![TransactionOutput {
                value: Amount::from_sats(1_000).expect("amount"),
                script_pubkey: ScriptBuf::from_bytes(vec![0x51]).expect("script"),
            }],
            lock_time: 0,
        }
    }

    fn wtxid_for(tag: u8) -> Wtxid {
        Wtxid::from_byte_array([tag; 32])
    }

    fn script(bytes: &[u8]) -> ScriptBuf {
        ScriptBuf::from_bytes(bytes.to_vec()).expect("valid script")
    }

    fn redeem_script() -> ScriptBuf {
        script(&[0x51])
    }

    fn p2sh_script() -> ScriptBuf {
        let redeem_hash = open_bitcoin_core::consensus::crypto::hash160(redeem_script().as_bytes());
        let mut bytes = vec![0xa9, 20];
        bytes.extend_from_slice(&redeem_hash);
        bytes.push(0x87);
        script(&bytes)
    }

    fn serialized_script_num(value: i64) -> Vec<u8> {
        if value == 0 {
            return vec![0x00];
        }
        let mut magnitude = value as u64;
        let mut encoded = Vec::new();
        while magnitude > 0 {
            encoded.push((magnitude & 0xff) as u8);
            magnitude >>= 8;
        }
        let mut out = Vec::with_capacity(encoded.len() + 2);
        out.push(encoded.len() as u8);
        out.extend(encoded);
        out.push(0x51);
        out
    }

    fn coinbase_transaction(height: u32, value: i64) -> Transaction {
        let mut script_sig = serialized_script_num(i64::from(height));
        script_sig.push(0x51);
        Transaction {
            version: 1,
            inputs: vec![TransactionInput {
                previous_output: OutPoint::null(),
                script_sig: script(&script_sig),
                sequence: TransactionInput::SEQUENCE_FINAL,
                witness: ScriptWitness::default(),
            }],
            outputs: vec![TransactionOutput {
                value: Amount::from_sats(value).expect("valid amount"),
                script_pubkey: p2sh_script(),
            }],
            lock_time: 0,
        }
    }

    fn spend_transaction(previous_txid: Txid, output_value: i64) -> Transaction {
        Transaction {
            version: 2,
            inputs: vec![TransactionInput {
                previous_output: OutPoint {
                    txid: previous_txid,
                    vout: 0,
                },
                script_sig: script(&[0x01, 0x51]),
                sequence: TransactionInput::SEQUENCE_FINAL,
                witness: ScriptWitness::default(),
            }],
            outputs: vec![TransactionOutput {
                value: Amount::from_sats(output_value).expect("valid amount"),
                script_pubkey: p2sh_script(),
            }],
            lock_time: 0,
        }
    }

    fn build_block(previous_block_hash: BlockHash, height: u32, value: i64) -> Block {
        let transactions = vec![coinbase_transaction(height, value)];
        let (merkle_root, maybe_mutated) = block_merkle_root(&transactions).expect("merkle root");
        assert!(!maybe_mutated);
        let mut block = Block {
            header: BlockHeader {
                version: 1,
                previous_block_hash,
                merkle_root,
                time: 1_231_006_500 + height,
                bits: EASY_BITS,
                nonce: 0,
            },
            transactions,
        };
        block.header.nonce = (0..=u32::MAX)
            .find(|nonce| {
                block.header.nonce = *nonce;
                check_block_header(&block.header).is_ok()
            })
            .expect("nonce");
        block
    }

    #[test]
    fn with_defaults_uses_knots_bound_constants() {
        // Arrange / Act
        let buffer = CompactExtraTxnBuffer::with_defaults();

        // Assert
        assert_eq!(buffer.max_slots(), DEFAULT_BLOCK_RECONSTRUCTION_EXTRA_TXN);
        assert_eq!(
            buffer.max_bytes(),
            DEFAULT_BLOCK_RECONSTRUCTION_EXTRA_TXN_SIZE
        );
        assert_eq!(
            buffer.per_tx_size_limit(),
            BLOCK_RECONSTRUCTION_EXTRA_TXN_PER_TXN_SIZE_LIMIT
        );
        assert_eq!(DEFAULT_BLOCK_RECONSTRUCTION_EXTRA_TXN, 32_768);
        assert_eq!(DEFAULT_BLOCK_RECONSTRUCTION_EXTRA_TXN_SIZE, 10_000_000);
        assert_eq!(BLOCK_RECONSTRUCTION_EXTRA_TXN_PER_TXN_SIZE_LIMIT, 100_000);
    }

    #[test]
    fn push_overwrites_in_ring_order_when_full() {
        // Arrange
        let mut buffer = CompactExtraTxnBuffer::new(2, 10_000_000, 100_000);
        let tx_a = sample_tx(0x01);
        let tx_b = sample_tx(0x02);
        let tx_c = sample_tx(0x03);

        // Act
        buffer.push(wtxid_for(0x01), tx_a);
        buffer.push(wtxid_for(0x02), tx_b.clone());
        buffer.push(wtxid_for(0x03), tx_c.clone());

        // Assert — FIFO overwrite: slot 0 (A) replaced by C; B remains
        let pairs = buffer.to_owned_pairs();
        assert_eq!(pairs.len(), 2);
        assert!(pairs.iter().any(|(wtxid, _)| *wtxid == wtxid_for(0x02)));
        assert!(pairs.iter().any(|(wtxid, _)| *wtxid == wtxid_for(0x03)));
        assert!(!pairs.iter().any(|(wtxid, _)| *wtxid == wtxid_for(0x01)));
    }

    #[test]
    fn push_evicts_next_slots_when_aggregate_byte_budget_exceeded() {
        // Arrange — tiny byte budget forces eviction after inserts
        let mut buffer = CompactExtraTxnBuffer::new(4, 120, 100_000);
        let tx_a = sample_tx(0x11);
        let tx_b = sample_tx(0x12);
        let size_a = open_bitcoin_mempool::transaction_weight_and_virtual_size(&tx_a)
            .expect("size")
            .1;
        let size_b = open_bitcoin_mempool::transaction_weight_and_virtual_size(&tx_b)
            .expect("size")
            .1;
        assert!(size_a + size_b > 120);

        // Act
        buffer.push(wtxid_for(0x11), tx_a);
        buffer.push(wtxid_for(0x12), tx_b);

        // Assert — Knots clears next slots until under budget
        assert!(buffer.approximate_bytes() <= 120);
        assert!(buffer.to_owned_pairs().len() < 2);
    }

    #[test]
    fn push_gated_rejects_oversized_transaction_without_inserting() {
        // Arrange
        let mut buffer = CompactExtraTxnBuffer::new(4, 10_000_000, 50);
        let tx = sample_tx(0x21);
        let size = open_bitcoin_mempool::transaction_weight_and_virtual_size(&tx)
            .expect("size")
            .1;
        assert!(size > 50);

        // Act
        let accepted = buffer.push_gated(wtxid_for(0x21), tx);

        // Assert
        assert!(!accepted);
        assert!(buffer.to_owned_pairs().is_empty());
        assert_eq!(buffer.approximate_bytes(), 0);
    }

    #[test]
    fn mempool_compact_candidate_owned_maps_entry_wtxid_and_transaction() {
        // Arrange
        let mut chainstate = Chainstate::new();
        let mut previous = BlockHash::from_byte_array([0_u8; 32]);
        let mut coinbase_txids = Vec::new();
        for height in 0..2 {
            let block = build_block(previous, height, 500_000_000);
            coinbase_txids.push(transaction_txid(&block.transactions[0]).expect("txid"));
            chainstate
                .connect_block(
                    &block,
                    u128::from(height + 1),
                    ScriptVerifyFlags::P2SH
                        | ScriptVerifyFlags::CHECKLOCKTIMEVERIFY
                        | ScriptVerifyFlags::CHECKSEQUENCEVERIFY,
                    ConsensusParams {
                        coinbase_maturity: 1,
                        ..ConsensusParams::default()
                    },
                )
                .expect("connect");
            previous = open_bitcoin_core::consensus::block_hash(&block.header);
        }
        let snapshot = chainstate.snapshot();
        let transaction = spend_transaction(coinbase_txids[0], 499_999_000);
        let expected_wtxid = transaction_wtxid(&transaction).expect("wtxid");
        let expected_tx = transaction.clone();
        let mut mempool = Mempool::default();
        mempool
            .accept_transaction_with_context(
                transaction,
                &snapshot,
                ScriptVerifyFlags::P2SH
                    | ScriptVerifyFlags::CHECKLOCKTIMEVERIFY
                    | ScriptVerifyFlags::CHECKSEQUENCEVERIFY,
                ConsensusParams {
                    coinbase_maturity: 1,
                    ..ConsensusParams::default()
                },
                open_bitcoin_mempool::AdmissionContext::legacy_unknown(),
            )
            .expect("admit");

        // Act
        let candidates = mempool_compact_candidate_owned(&mempool);

        // Assert
        assert_eq!(candidates.len(), 1);
        assert_eq!(candidates[0].0, expected_wtxid);
        assert_eq!(candidates[0].1, expected_tx);
    }

    #[test]
    fn compact_extra_owned_returns_non_null_slots() {
        // Arrange
        let mut buffer = CompactExtraTxnBuffer::new(4, 10_000_000, 100_000);
        buffer.push(wtxid_for(0x31), sample_tx(0x31));

        // Act
        let owned = compact_extra_owned(&buffer);

        // Assert
        assert_eq!(owned.len(), 1);
        assert_eq!(owned[0].0, wtxid_for(0x31));
    }
}
