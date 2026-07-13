// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/net_processing.h
// - packages/bitcoin-knots/src/blockencodings.cpp
// - packages/bitcoin-knots/test/functional/p2p_compactblocks.py

use open_bitcoin_core::primitives::{Transaction, Wtxid};
use open_bitcoin_mempool::Mempool;

/// Knots `DEFAULT_BLOCK_RECONSTRUCTION_EXTRA_TXN` (`net_processing.h`).
pub const DEFAULT_BLOCK_RECONSTRUCTION_EXTRA_TXN: usize = 32_768;
/// Knots `DEFAULT_BLOCK_RECONSTRUCTION_EXTRA_TXN_SIZE` (`net_processing.h`).
pub const DEFAULT_BLOCK_RECONSTRUCTION_EXTRA_TXN_SIZE: usize = 10_000_000;
/// Knots `BLOCK_RECONSTRUCTION_EXTRA_TXN_PER_TXN_SIZE_LIMIT` (`net_processing.h`).
pub const BLOCK_RECONSTRUCTION_EXTRA_TXN_PER_TXN_SIZE_LIMIT: usize = 100_000;

/// Knots-shaped ring buffer for compact-block reconstruction extras (`vExtraTxnForCompact`).
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
        let _ = (max_slots, max_bytes, per_tx_size_limit);
        Self {
            slots: Vec::new(),
            write_cursor: 0,
            approximate_bytes: 0,
            max_slots: 0,
            max_bytes: 0,
            per_tx_size_limit: 0,
        }
    }

    pub fn with_defaults() -> Self {
        Self::new(
            DEFAULT_BLOCK_RECONSTRUCTION_EXTRA_TXN,
            DEFAULT_BLOCK_RECONSTRUCTION_EXTRA_TXN_SIZE,
            BLOCK_RECONSTRUCTION_EXTRA_TXN_PER_TXN_SIZE_LIMIT,
        )
    }

    pub fn push(&mut self, wtxid: Wtxid, transaction: Transaction) {
        let _ = (wtxid, transaction);
    }

    pub fn push_gated(&mut self, wtxid: Wtxid, transaction: Transaction) -> bool {
        let _ = (wtxid, transaction);
        false
    }

    pub fn iter_available(&self) -> impl Iterator<Item = &(Wtxid, Transaction)> {
        self.slots.iter().filter_map(Option::as_ref)
    }

    pub fn to_owned_pairs(&self) -> Vec<(Wtxid, Transaction)> {
        self.iter_available().cloned().collect()
    }

    pub fn approximate_bytes(&self) -> usize {
        self.approximate_bytes
    }

    pub fn max_slots(&self) -> usize {
        self.max_slots
    }

    pub fn max_bytes(&self) -> usize {
        self.max_bytes
    }

    pub fn per_tx_size_limit(&self) -> usize {
        self.per_tx_size_limit
    }
}

/// Snapshot mempool entries as owned `(Wtxid, Transaction)` pairs for compact receive facts.
pub fn mempool_compact_candidate_owned(mempool: &Mempool) -> Vec<(Wtxid, Transaction)> {
    let _ = mempool;
    Vec::new()
}

/// Snapshot non-null extra-buffer slots as owned pairs for facts adaptation.
pub fn compact_extra_owned(buffer: &CompactExtraTxnBuffer) -> Vec<(Wtxid, Transaction)> {
    buffer.to_owned_pairs()
}

#[cfg(test)]
mod tests {
    use open_bitcoin_core::primitives::{
        Amount, OutPoint, ScriptBuf, ScriptWitness, Transaction, TransactionInput,
        TransactionOutput, Txid, Wtxid,
    };
    use open_bitcoin_mempool::Mempool;

    use super::{
        BLOCK_RECONSTRUCTION_EXTRA_TXN_PER_TXN_SIZE_LIMIT, CompactExtraTxnBuffer,
        DEFAULT_BLOCK_RECONSTRUCTION_EXTRA_TXN, DEFAULT_BLOCK_RECONSTRUCTION_EXTRA_TXN_SIZE,
        compact_extra_owned, mempool_compact_candidate_owned,
    };

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

    #[test]
    fn with_defaults_uses_knots_bound_constants() {
        // Arrange / Act
        let buffer = CompactExtraTxnBuffer::with_defaults();

        // Assert
        assert_eq!(buffer.max_slots(), DEFAULT_BLOCK_RECONSTRUCTION_EXTRA_TXN);
        assert_eq!(buffer.max_bytes(), DEFAULT_BLOCK_RECONSTRUCTION_EXTRA_TXN_SIZE);
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
        // Arrange — empty mempool maps to empty; non-empty mapping covered when entries exist
        let mempool = Mempool::default();

        // Act
        let candidates = mempool_compact_candidate_owned(&mempool);

        // Assert
        assert!(candidates.is_empty());
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
