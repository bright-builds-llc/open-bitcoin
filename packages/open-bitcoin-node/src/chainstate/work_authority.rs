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
    fn check_prepared_submission(
        &self,
        prepared: &PreparedChainstateConnect,
    ) -> Result<(), ChainstateError> {
        let authorization = prepared
            .maybe_body_authorization
            .as_ref()
            .ok_or(ChainstateError::PreferenceAuthorizationRequired)?;
        authorization.check_owner(&self.chainstate)?;
        authorization.check_position(&prepared.position)?;
        if prepared.staged.position() != &prepared.position {
            return Err(ChainstateError::PreferenceAuthorizationRequired);
        }
        Ok(())
    }

    /// Borrow exact prospective data from this owner's authorized connect.
    pub(crate) fn project_prepared_submission(
        &self,
        prepared: &PreparedChainstateConnect,
    ) -> Result<ChainstateSnapshot, ChainstateError> {
        self.check_prepared_submission(prepared)?;
        self.chainstate.project_staged_connect(&prepared.staged)
    }

    /// Borrow captured historical BASIC inputs; absence never means an enabled proof.
    pub(crate) fn maybe_prepared_submission_basic_inputs<'a>(
        &self,
        prepared: &'a PreparedChainstateConnect,
    ) -> Result<Option<open_bitcoin_core::chainstate::BasicFilterInputs<'a>>, ChainstateError> {
        self.check_prepared_submission(prepared)?;
        match &prepared.maybe_filter_facts {
            Some(Ok(facts)) => {
                facts
                    .inputs()
                    .map(Some)
                    .map_err(|error| ChainstateError::CoinsStorage {
                        detail: error.to_string(),
                    })
            }
            Some(Err(_)) => Err(ChainstateError::CoinsStorage {
                detail: "prepared BASIC facts unavailable".to_owned(),
            }),
            None => Ok(None),
        }
    }
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

#[cfg(test)]
mod submission_tests {
    use super::*;
    use crate::chainstate::validation_history::tests::phase167 as genuine;

    fn prepared(
        network: &mut genuine::Network,
        block: &Block,
        params: ConsensusParams,
    ) -> PreparedChainstateConnect {
        genuine::admit(network, std::slice::from_ref(block), params);
        network
            .retain_checked_candidate(block, params, 2_000_000_000)
            .expect("actual body");
        let authorization = network
            .authorize_connect(block, params)
            .expect("actual preference");
        network
            .chainstate()
            .prepare_authorized_connect(
                authorization,
                block,
                2_000_000_000,
                ScriptVerifyFlags::NONE,
                params,
            )
            .expect("actual stage")
    }

    #[test]
    fn phase167_submission_projection_api_inferred_inventory() {
        genuine::with_manager(|manager, store, _| {
            // Arrange
            let inventory = store
                .inspect_submission_inventory()
                .expect("inferred opaque data");
            // Act / Assert
            inventory.check_current_source().expect("actual source");
            inventory.into_achieved_check()(&store).expect("same-store achieved bytes");
            drop(manager);
        });
    }

    #[test]
    fn phase167_submission_projection_genuine_prepared_state_matches_absorption() {
        genuine::with_manager(|manager, store, params| {
            // Arrange
            let mut network = genuine::network(manager);
            let block = genuine::child(&network.maybe_chain_tip().expect("root").header, 1);
            let prepared = prepared(&mut network, &block, params);
            let before = network.chainstate_snapshot().expect("before");
            // Act
            let projected = network
                .chainstate()
                .project_prepared_submission(&prepared)
                .expect("read-only data");
            // Assert
            assert_eq!(network.chainstate_snapshot().expect("unchanged"), before);
            assert_eq!(projected.utxos.len(), 1);
            assert_eq!(
                projected
                    .maybe_confirmed_txid_counts
                    .as_ref()
                    .expect("complete counts")
                    .values()
                    .sum::<u32>(),
                1
            );
            assert!(
                network
                    .chainstate()
                    .maybe_prepared_submission_basic_inputs(&prepared)
                    .expect("disabled")
                    .is_none()
            );
            genuine::with_manager(|foreign, _, _| {
                assert!(foreign.project_prepared_submission(&prepared).is_err());
            });
            network
                .chainstate_mut()
                .commit_prepared_connect(prepared)
                .expect("actual absorption");
            assert_eq!(network.chainstate_snapshot().expect("achieved"), projected);
            drop(network);
            drop(store);
        });
    }

    #[test]
    fn phase167_submission_projection_complete_coins_spends_and_same_block_cancellation() {
        use open_bitcoin_core::primitives::{
            Amount, ScriptBuf, ScriptWitness, Transaction, TransactionInput, TransactionOutput,
        };
        genuine::with_manager(|manager, store, params| {
            // Arrange
            let mut network = genuine::network(manager);
            let mut maybe_funding = None;
            for height in 1..=100 {
                let block = genuine::child(&network.maybe_chain_tip().expect("tip").header, height);
                if height == 1 {
                    maybe_funding = Some(OutPoint {
                        txid: open_bitcoin_core::consensus::transaction_txid(
                            &block.transactions[0],
                        )
                        .expect("funding txid"),
                        vout: 0,
                    });
                }
                let stage = prepared(&mut network, &block, params);
                network
                    .chainstate_mut()
                    .commit_prepared_connect(stage)
                    .expect("genuine canonical prefix");
            }
            let funding = maybe_funding.expect("matured real output");
            let first = Transaction {
                version: 2,
                lock_time: 0,
                inputs: vec![TransactionInput {
                    previous_output: funding.clone(),
                    script_sig: ScriptBuf::default(),
                    sequence: u32::MAX,
                    witness: ScriptWitness::default(),
                }],
                outputs: vec![TransactionOutput {
                    value: Amount::from_sats(4_999_999_999).expect("fee"),
                    script_pubkey: ScriptBuf::from_bytes(vec![0x51]).expect("true"),
                }],
            };
            let intermediate = OutPoint {
                txid: open_bitcoin_core::consensus::transaction_txid(&first).expect("intermediate"),
                vout: 0,
            };
            let mut second = first.clone();
            second.inputs[0].previous_output = intermediate.clone();
            second.outputs[0].value = Amount::from_sats(4_999_999_998).expect("second fee");
            let final_output = OutPoint {
                txid: open_bitcoin_core::consensus::transaction_txid(&second).expect("final"),
                vout: 0,
            };
            let mut block = genuine::child(&network.maybe_chain_tip().expect("tip").header, 101);
            block.transactions.extend([first, second]);
            genuine::remine(&mut block);
            let stage = prepared(&mut network, &block, params);
            let before = network.chainstate_snapshot().expect("complete prior Coins");
            // Act
            let projected = network
                .chainstate()
                .project_prepared_submission(&stage)
                .expect("actual complete overlay");
            // Assert
            assert_eq!(network.chainstate_snapshot().expect("unmodified"), before);
            assert!(!projected.utxos.contains_key(&funding));
            assert!(!projected.utxos.contains_key(&intermediate));
            assert!(projected.utxos.contains_key(&final_output));
            assert_eq!(
                projected
                    .maybe_confirmed_txid_counts
                    .as_ref()
                    .expect("all counts")
                    .len(),
                103
            );
            assert!(
                projected
                    .maybe_confirmed_txid_counts
                    .as_ref()
                    .expect("all counts")
                    .contains_key(&intermediate.txid)
            );
            network
                .chainstate_mut()
                .commit_prepared_connect(stage)
                .expect("same actual stage");
            assert_eq!(
                network.chainstate_snapshot().expect("all achieved facts"),
                projected
            );
            drop(network);
            drop(store);
        });
    }
}
