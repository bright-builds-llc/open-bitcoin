// Parity breadcrumbs:
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/chain.h

use super::*;
use crate::network::work_authority::{ConnectAuthorization, ReorgAuthorization};

#[cfg(test)]
thread_local! {
    static OBSERVATIONS: std::cell::Cell<[u64; 5]> = const { std::cell::Cell::new([0; 5]) };
}

#[cfg(test)]
pub(super) fn observe_preparation(index: usize) {
    OBSERVATIONS.with(|observations| {
        let mut value = observations.get();
        value[index] += 1;
        observations.set(value);
    });
}

impl<S: ChainstateStore, V: CoinsView> ManagedChainstate<S, V> {
    pub(crate) fn authorize_direct_connect(
        &self,
        block: &Block,
        params: ConsensusParams,
    ) -> Result<open_bitcoin_core::consensus::chain_proof::ProofWork, ChainstateError> {
        use open_bitcoin_core::consensus::chain_proof::{ProofWork, block_proof};
        let _source = self.chainstate.bind_live_work_source(params)?;
        let previous = self
            .chainstate
            .tip()
            .map_or(BlockHash::default(), |tip| tip.block_hash);
        if block.header.previous_block_hash != previous {
            return Err(ChainstateError::InvalidTipExtension {
                expected_previous: previous,
                actual_previous: block.header.previous_block_hash,
            });
        }
        let proof =
            block_proof(block.header.bits).map_err(|error| ChainstateError::Serialization {
                context: "managed block proof",
                reason: error.to_string(),
            })?;
        open_bitcoin_core::consensus::block::check_block_header_with_params(&block.header, &params)
            .map_err(|source| ChainstateError::BlockValidation { source })?;
        Ok(self
            .chainstate
            .tip()
            .map_or(ProofWork::ZERO, |tip| tip.chain_work)
            .wrapping_add(proof))
    }
    #[cfg(test)]
    pub(crate) fn work_preparation_observations(&self) -> [u64; 5] {
        OBSERVATIONS.with(std::cell::Cell::get)
    }
    pub(crate) fn maybe_accepted_body_preference(
        &self,
    ) -> Option<&open_bitcoin_network::BodyEligibilityBinding> {
        self.maybe_body_preference.as_ref()
    }

    pub(crate) fn prepare_authorized_connect(
        &self,
        authorization: ConnectAuthorization,
        block: &Block,
        current_time: i64,
        flags: ScriptVerifyFlags,
        params: ConsensusParams,
    ) -> Result<PreparedChainstateConnect, ChainstateError> {
        authorization.check(&self.chainstate, block)?;
        let mut prepared = self.prepare_connect_block_with_current_time(
            block,
            open_bitcoin_core::consensus::chain_proof::ProofWork::ZERO,
            current_time,
            flags,
            params,
        )?;
        authorization.check_position(prepared.position())?;
        prepared.maybe_body_authorization = Some(authorization);
        Ok(prepared)
    }
    pub(crate) fn prepare_reorg(
        &self,
        _disconnect_blocks: &[Block],
        _replacement_branch: &[AnchoredBlock],
        _verify_flags: ScriptVerifyFlags,
        _consensus_params: ConsensusParams,
    ) -> Result<PreparedChainstateReorg, open_bitcoin_core::chainstate::ChainstateError> {
        Err(ChainstateError::PreferenceAuthorizationRequired)
    }

    pub(crate) fn prepare_authorized_reorg(
        &self,
        authorization: ReorgAuthorization,
        disconnect_blocks: &[Block],
        replacement_branch: &[AnchoredBlock],
        verify_flags: ScriptVerifyFlags,
        consensus_params: ConsensusParams,
    ) -> Result<PreparedChainstateReorg, ChainstateError> {
        authorization.check_before_prepare(
            &self.chainstate,
            disconnect_blocks,
            replacement_branch,
            consensus_params,
        )?;
        #[cfg(test)]
        observe_preparation(0);
        let staged = self.chainstate.stage_reorg(
            disconnect_blocks,
            replacement_branch,
            verify_flags,
            consensus_params,
        )?;
        authorization.check_stage(&self.chainstate, &staged)?;
        let transition = staged.transition().clone();
        #[cfg(test)]
        observe_preparation(1);
        let maybe_history = self.prepare_validation_reorg(&staged)?;
        #[cfg(test)]
        observe_preparation(2);
        let maybe_index = self.prepare_basic_index_reorg(&staged, replacement_branch)?;
        Ok(PreparedChainstateReorg {
            authorization,
            staged,
            maybe_history,
            transition,
            maybe_index,
        })
    }
}
