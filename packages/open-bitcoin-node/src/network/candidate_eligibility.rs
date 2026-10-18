// Parity breadcrumbs:
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/chain.h
// - packages/bitcoin-knots/src/node/blockstorage.cpp

use super::*;
use open_bitcoin_network::BodyEligibilityFacts;

impl<S: ChainstateStore, V: CoinsView> ManagedPeerNetwork<S, V> {
    pub(crate) fn requestable_header_entries(
        &self,
        params: ConsensusParams,
    ) -> Vec<open_bitcoin_network::HeaderEntry> {
        self.peer_manager
            .header_store()
            .requestable_header_entries(&params)
    }

    /// Re-admit actual cached bytes without inventing an arrival-order receipt.
    pub(crate) fn retain_cached_candidate(
        &mut self,
        block: &Block,
        params: ConsensusParams,
        current_time: i64,
    ) -> ManagedResult<()> {
        self.guard_body(block)?;
        let mut headers = self.peer_manager.header_store().clone();
        headers
            .check_cached_body_eligibility_detailed(block, params, current_time)
            .map_err(|error| match error {
                open_bitcoin_network::BodyEligibilityError::Validation(source) => {
                    ManagedNetworkError::Chainstate(
                        open_bitcoin_core::chainstate::ChainstateError::BlockValidation { source },
                    )
                }
                open_bitcoin_network::BodyEligibilityError::Network(error) => {
                    ManagedNetworkError::Network(error)
                }
            })?;
        self.peer_manager.seed_header_store(headers);
        self.maybe_header_consensus_params = Some(params);
        self.blocks_by_hash
            .insert(block_hash(&block.header), block.clone());
        Ok(())
    }

    pub(crate) fn retain_checked_candidate(
        &mut self,
        block: &Block,
        params: ConsensusParams,
        current_time: i64,
    ) -> ManagedResult<()> {
        self.guard_body(block)?;
        if block.header.previous_block_hash != BlockHash::default()
            && self
                .peer_manager
                .header_store()
                .maybe_proof_facts(block.header.previous_block_hash)
                .is_none()
        {
            // Existing raw disconnected/nonextending cache receipts have no linkage authority.
            return Ok(());
        }
        if self
            .maybe_header_consensus_params
            .is_some_and(|selected| selected != params)
        {
            return Err(open_bitcoin_network::NetworkError::InvalidHeader {
                reject_reason: "body-consensus-params-mismatch".to_owned(),
                maybe_debug_message: None,
            }
            .into());
        }
        self.peer_manager
            .check_body_eligibility(block, &params, current_time)?;
        self.maybe_header_consensus_params = Some(params);
        self.blocks_by_hash
            .insert(block_hash(&block.header), block.clone());
        Ok(())
    }

    pub(super) fn note_candidate_connection_failure(
        &mut self,
        hash: BlockHash,
        error: &open_bitcoin_core::chainstate::ChainstateError,
    ) {
        use open_bitcoin_core::chainstate::ChainstateError;
        if matches!(
            error,
            ChainstateError::BlockValidation { .. }
                | ChainstateError::TransactionValidation { .. }
                | ChainstateError::MissingCoin { .. }
                | ChainstateError::OutputOverwrite { .. }
        ) {
            self.peer_manager.mark_body_failed(hash);
        }
    }

    /// Transaction-linked candidates only; later connection still validates scripts and coins.
    pub fn eligible_body_candidates(&self) -> Vec<BodyEligibilityFacts> {
        let active: Vec<_> = self
            .chainstate
            .chainstate()
            .active_chain()
            .iter()
            .map(|position| position.block_hash)
            .collect();
        self.peer_manager
            .header_store()
            .eligible_body_candidates(&active)
            .into_iter()
            .filter(|facts| self.has_retained_candidate_suffix(facts.block_hash, &active))
            .collect()
    }

    fn has_retained_candidate_suffix(&self, hash: BlockHash, active: &[BlockHash]) -> bool {
        let headers = self.peer_manager.header_store();
        let mut current = hash;
        for _ in 0..headers.entries().count() {
            if active.contains(&current) {
                return true;
            }
            if !self.blocks_by_hash.contains_key(&current) {
                return false;
            }
            let Some(entry) = headers.entry(&current) else {
                return false;
            };
            if entry.header.previous_block_hash == BlockHash::default() {
                return true;
            }
            current = entry.header.previous_block_hash;
        }
        false
    }

    /// Historical linkage is separate from present availability and accepted coins lineage.
    pub fn maybe_body_eligibility(&self, hash: BlockHash) -> Option<BodyEligibilityFacts> {
        let mut facts = self
            .peer_manager
            .header_store()
            .maybe_body_eligibility(hash)?;
        facts.available &= self.blocks_by_hash.contains_key(&hash);
        Some(facts)
    }
}
