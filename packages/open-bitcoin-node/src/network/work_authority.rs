// Parity breadcrumbs:
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/chain.h

use super::*;
use open_bitcoin_core::chainstate::{
    AnchoredBlock, ChainstateError, LiveWorkSourceBinding, StagedChainstateReorg,
};
use open_bitcoin_core::consensus::chain_proof::{ProofWork, block_proof};
use open_bitcoin_network::BodyEligibilityFacts;

pub(crate) struct ConnectAuthorization {
    source: LiveWorkSourceBinding,
    block: Block,
    binding: open_bitcoin_network::BodyEligibilityBinding,
    epoch: open_bitcoin_network::BodyEligibilityEpoch,
}

impl ConnectAuthorization {
    pub(crate) fn check_owner<V: CoinsView>(
        &self,
        owner: &open_bitcoin_core::chainstate::Chainstate<V>,
    ) -> Result<(), ChainstateError> {
        owner.check_live_work_source(&self.source)?;
        if !self.epoch.is_current() {
            return Err(ChainstateError::StaleStage);
        }
        Ok(())
    }
    pub(crate) fn check<V: CoinsView>(
        &self,
        owner: &open_bitcoin_core::chainstate::Chainstate<V>,
        block: &Block,
    ) -> Result<(), ChainstateError> {
        self.check_owner(owner)?;
        if self.block != *block {
            return Err(ChainstateError::PreferenceAuthorizationRequired);
        }
        Ok(())
    }

    pub(crate) fn check_position(&self, position: &ChainPosition) -> Result<(), ChainstateError> {
        if !self.epoch.is_current() {
            return Err(ChainstateError::StaleStage);
        }
        let facts = self.binding.facts();
        if position.block_hash != facts.block_hash
            || position.header != self.block.header
            || position.chain_work != facts.chain_work
        {
            return Err(ChainstateError::PreferenceAuthorizationRequired);
        }
        Ok(())
    }

    pub(crate) fn into_binding(self) -> open_bitcoin_network::BodyEligibilityBinding {
        self.binding
    }
}

/// Minted only by the actual network owner after checked body preference.
/// Fields and constructor are private; moving it into a prepared transition consumes it.
pub(crate) struct ReorgAuthorization {
    source: LiveWorkSourceBinding,
    disconnect: Vec<Block>,
    replacements: Vec<Block>,
    work: Vec<ProofWork>,
    params: ConsensusParams,
    active: BodyEligibilityFacts,
    candidate: BodyEligibilityFacts,
    candidate_binding: open_bitcoin_network::BodyEligibilityBinding,
    epoch: open_bitcoin_network::BodyEligibilityEpoch,
    maybe_checked_headers: Option<HeaderStore>,
}

impl ReorgAuthorization {
    pub(crate) fn into_candidate_binding(self) -> open_bitcoin_network::BodyEligibilityBinding {
        self.candidate_binding
    }
    pub(crate) fn check_before_prepare<V: CoinsView>(
        &self,
        owner: &open_bitcoin_core::chainstate::Chainstate<V>,
        disconnect: &[Block],
        replacements: &[AnchoredBlock],
        params: ConsensusParams,
    ) -> Result<(), ChainstateError> {
        owner.check_live_work_source(&self.source)?;
        if self.params != params
            || self.disconnect != disconnect
            || self.replacements.len() != replacements.len()
            || !self
                .replacements
                .iter()
                .zip(replacements)
                .all(|(a, b)| a == &b.block)
        {
            return Err(ChainstateError::PreferenceAuthorizationRequired);
        }
        self.check_preference()
    }

    fn check_preference(&self) -> Result<(), ChainstateError> {
        if !self.epoch.is_current() {
            return Err(ChainstateError::StaleStage);
        }
        let preferred = self.candidate.chain_work > self.active.chain_work
            || (self.candidate.chain_work == self.active.chain_work
                && self
                    .candidate
                    .maybe_sequence
                    .zip(self.active.maybe_sequence)
                    .is_some_and(|(candidate, active)| candidate < active));
        if !preferred || self.candidate.failed || !self.candidate.available {
            return Err(ChainstateError::BranchNotPreferred);
        }
        Ok(())
    }

    pub(crate) fn check_stage<V: CoinsView>(
        &self,
        owner: &open_bitcoin_core::chainstate::Chainstate<V>,
        staged: &StagedChainstateReorg,
    ) -> Result<(), ChainstateError> {
        self.check_preference()?;
        owner.check_reorg_work_source(&self.source, staged)?;
        if staged.transition().connected.len() != self.work.len()
            || !staged
                .transition()
                .connected
                .iter()
                .zip(self.replacements.iter().zip(&self.work))
                .all(|(position, (block, work))| {
                    position.block_hash == block_hash(&block.header)
                        && position.header == block.header
                        && position.chain_work == *work
                })
        {
            return Err(ChainstateError::PreferenceAuthorizationRequired);
        }
        Ok(())
    }
}

impl<S: ChainstateStore, V: CoinsView> ManagedPeerNetwork<S, V> {
    #[cfg(test)]
    pub(crate) fn into_fixture_chainstate(self) -> ManagedChainstate<S, V> {
        self.chainstate
    }
    pub(super) fn retain_authorized_reorg(
        &mut self,
        authorization: &mut ReorgAuthorization,
    ) -> Result<(), ChainstateError> {
        self.chainstate
            .chainstate()
            .check_live_work_source(&authorization.source)?;
        authorization.check_preference()?;
        let headers = authorization
            .maybe_checked_headers
            .take()
            .ok_or(ChainstateError::PreferenceAuthorizationRequired)?;
        // Publish the exact checked history, retaining its immutable linkage identities.
        self.peer_manager.seed_header_store(headers);
        authorization.epoch = self.peer_manager.header_store().body_eligibility_epoch();
        self.maybe_header_consensus_params = Some(authorization.params);
        for block in &authorization.replacements {
            self.blocks_by_hash
                .insert(block_hash(&block.header), block.clone());
        }
        Ok(())
    }
    pub(crate) fn authorize_connect(
        &self,
        block: &Block,
        params: ConsensusParams,
    ) -> Result<ConnectAuthorization, ChainstateError> {
        let source = self.chainstate.chainstate().bind_live_work_source(params)?;
        let binding = self
            .peer_manager
            .header_store()
            .maybe_body_eligibility_binding(block_hash(&block.header))
            .filter(|binding| {
                let facts = binding.facts();
                facts.available
                    && !facts.failed
                    && self.blocks_by_hash.get(&facts.block_hash) == Some(block)
            })
            .ok_or(ChainstateError::PreferenceAuthorizationRequired)?;
        Ok(ConnectAuthorization {
            source,
            block: block.clone(),
            binding,
            epoch: self.peer_manager.header_store().body_eligibility_epoch(),
        })
    }
    /// Pure preparation against checked copies; refusal cannot change peer or retained state.
    pub(crate) fn authorize_reorg(
        &self,
        disconnect: &[Block],
        replacements: &[AnchoredBlock],
        params: ConsensusParams,
    ) -> Result<ReorgAuthorization, ChainstateError> {
        let owner = self.chainstate.chainstate();
        let source = owner.bind_live_work_source(params)?;
        let active_tip = owner.tip().ok_or(ChainstateError::MissingTip)?;
        let retained_len = owner
            .active_chain()
            .len()
            .checked_sub(disconnect.len())
            .ok_or(ChainstateError::DisconnectPastGenesis {
                requested: disconnect.len(),
                available: owner.active_chain().len(),
            })?;
        // A replacement must retain an actually accepted ancestor, including the selected root.
        let ancestor = retained_len
            .checked_sub(1)
            .and_then(|index| owner.active_chain().get(index))
            .ok_or(ChainstateError::PreferenceAuthorizationRequired)?;
        for (block, position) in disconnect.iter().zip(owner.active_chain().iter().rev()) {
            if block_hash(&block.header) != position.block_hash || block.header != position.header {
                return Err(ChainstateError::DisconnectBlockMismatch {
                    expected_tip: position.block_hash,
                    actual_block: block_hash(&block.header),
                });
            }
            if self.blocks_by_hash.get(&position.block_hash) != Some(block) {
                return Err(ChainstateError::PreferenceAuthorizationRequired);
            }
            if owner.undo_by_block().get(&position.block_hash).is_none() {
                return Err(ChainstateError::MissingUndo {
                    block_hash: position.block_hash,
                });
            }
        }
        let mut headers = self.peer_manager.header_store().clone();
        let active = headers
            .maybe_body_eligibility_binding(active_tip.block_hash)
            .map(|binding| binding.facts())
            .filter(|facts| !facts.failed && facts.chain_work == active_tip.chain_work)
            .ok_or(ChainstateError::PreferenceAuthorizationRequired)?;
        let active = match self.chainstate.maybe_accepted_body_preference() {
            Some(accepted)
                if headers
                    .maybe_body_eligibility_binding(active.block_hash)
                    .is_some_and(|current| accepted.same_linkage(&current)) =>
            {
                accepted.facts()
            }
            // Unknown/restart linkage is bounded: strictly greater work can qualify,
            // but mutable peer history cannot manufacture a known activation tie.
            _ => BodyEligibilityFacts {
                maybe_sequence: None,
                ..active
            },
        };
        let mut previous = ancestor.block_hash;
        let mut work = ancestor.chain_work;
        let mut works = Vec::new();
        for candidate in replacements {
            let block = &candidate.block;
            if block.header.previous_block_hash != previous {
                return Err(ChainstateError::InvalidTipExtension {
                    expected_previous: previous,
                    actual_previous: block.header.previous_block_hash,
                });
            }
            let own_proof = block_proof(block.header.bits)
                .map_err(|_| ChainstateError::PreferenceAuthorizationRequired)?;
            headers
                .check_cached_body_eligibility(block, params, i64::from(block.header.time))
                .map_err(|_| ChainstateError::PreferenceAuthorizationRequired)?;
            previous = block_hash(&block.header);
            work = work.wrapping_add(own_proof);
            let facts = headers
                .maybe_body_eligibility_binding(previous)
                .map(|binding| binding.facts())
                .filter(|facts| !facts.failed && facts.available && facts.chain_work == work)
                .ok_or(ChainstateError::PreferenceAuthorizationRequired)?;
            works.push(facts.chain_work);
        }
        let candidate = headers
            .maybe_body_eligibility(previous)
            .ok_or(ChainstateError::PreferenceAuthorizationRequired)?;
        let candidate_binding = headers
            .maybe_body_eligibility_binding(previous)
            .ok_or(ChainstateError::PreferenceAuthorizationRequired)?;
        let authorization = ReorgAuthorization {
            source,
            disconnect: disconnect.to_vec(),
            replacements: replacements
                .iter()
                .map(|candidate| candidate.block.clone())
                .collect(),
            work: works,
            params,
            active,
            candidate,
            candidate_binding,
            epoch: self.peer_manager.header_store().body_eligibility_epoch(),
            maybe_checked_headers: Some(headers),
        };
        authorization.check_preference()?;
        Ok(authorization)
    }
}
