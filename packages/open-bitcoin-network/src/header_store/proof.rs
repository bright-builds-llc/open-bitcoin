// Parity breadcrumbs:
// - packages/bitcoin-knots/src/chain.cpp
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp
// - packages/bitcoin-knots/src/pow.cpp

use super::*;
use open_bitcoin_consensus::chain_proof::{ProofWork, block_proof};

/// Derived header arithmetic only; this never asserts scripts validation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HeaderProofFacts {
    pub block_hash: BlockHash,
    pub height: u32,
    pub timestamp: u32,
    pub cumulative: ProofWork,
    pub block_proof: ProofWork,
}

impl HeaderStore {
    /// Projection of accepted header arithmetic; imports have no earned facts.
    pub fn maybe_proof_facts(&self, hash: BlockHash) -> Option<HeaderProofFacts> {
        if !self.accepted_headers.contains(&hash) {
            return None;
        }
        let entry = self.entries.get(&hash)?;
        Some(HeaderProofFacts {
            block_hash: hash,
            height: entry.height,
            timestamp: entry.header.time,
            cumulative: entry.chain_work,
            block_proof: block_proof(entry.header.bits).ok()?,
        })
    }

    /// Read-only projection of the single selected tip, absent for unproved imports.
    pub fn maybe_best_proof_header(&self) -> Option<HeaderProofFacts> {
        self.best_tip()
            .and_then(|entry| self.maybe_proof_facts(entry.block_hash))
    }
}

#[cfg(test)]
mod tests;
