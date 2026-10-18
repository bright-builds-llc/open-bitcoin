// Parity breadcrumbs:
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/chain.h
// - packages/bitcoin-knots/src/node/blockstorage.cpp

use super::PeerManager;
use open_bitcoin_consensus::ConsensusParams;
use open_bitcoin_primitives::{Block, BlockHash};

impl PeerManager {
    /// Check actual selected-context body bytes before recording transaction linkage.
    pub fn check_body_eligibility(
        &mut self,
        block: &Block,
        params: &ConsensusParams,
        current_time: i64,
    ) -> Result<(), crate::NetworkError> {
        self.headers
            .check_body_eligibility(block, *params, current_time)
    }

    /// Revoke activation availability while retaining historical transaction linkage.
    pub fn mark_body_unavailable(&mut self, hash: BlockHash) {
        self.headers.mark_body_unavailable(hash);
    }

    /// Mark a failed checked body; descendants cannot activate through it.
    pub fn mark_body_failed(&mut self, hash: BlockHash) {
        self.headers.mark_body_failed(hash);
    }
}
