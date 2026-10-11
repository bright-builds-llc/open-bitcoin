// Parity breadcrumbs:
// - packages/bitcoin-knots/src/chain.cpp
// - packages/bitcoin-knots/src/net_processing.cpp

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
    /// Exact admitted/recovered header work, absent for unusable legacy identities.
    pub fn maybe_proof_facts(&self, hash: BlockHash) -> Option<HeaderProofFacts> {
        self.proof_facts.get(&hash).copied()
    }

    /// Best exact-work header, independent of the existing synthetic fork choice.
    pub fn maybe_best_proof_header(&self) -> Option<HeaderProofFacts> {
        self.maybe_best_proof
            .and_then(|hash| self.maybe_proof_facts(hash))
    }

    pub(super) fn rebuild_proof_facts(&mut self) {
        let mut ordered: Vec<_> = self
            .entries
            .values()
            .map(|entry| (entry.height, entry.block_hash))
            .collect();
        ordered.sort_unstable();
        for (_, hash) in ordered {
            self.record_proof_facts(hash);
        }
    }

    pub(super) fn record_proof_facts(&mut self, hash: BlockHash) {
        let Some(entry) = self.entries.get(&hash) else {
            return;
        };
        if block_hash(&entry.header) != hash {
            return;
        }
        let Ok(proof) = block_proof(entry.header.bits) else {
            return;
        };
        let prior = if entry.height == 0 {
            if entry.header.previous_block_hash != BlockHash::default() {
                return;
            }
            ProofWork::ZERO
        } else {
            let Some(parent) = self.maybe_proof_facts(entry.header.previous_block_hash) else {
                return;
            };
            if parent.height.checked_add(1) != Some(entry.height) {
                return;
            }
            parent.cumulative
        };
        let facts = HeaderProofFacts {
            block_hash: hash,
            height: entry.height,
            timestamp: entry.header.time,
            cumulative: prior.wrapping_add(proof),
            block_proof: proof,
        };
        self.proof_facts.insert(hash, facts);
        let replace = self.maybe_best_proof_header().is_none_or(|current| {
            (facts.cumulative, facts.height, facts.block_hash)
                > (current.cumulative, current.height, current.block_hash)
        });
        if replace {
            self.maybe_best_proof = Some(hash);
        }
    }
}

#[cfg(test)]
mod tests;
