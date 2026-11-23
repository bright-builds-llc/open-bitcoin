// Parity breadcrumbs:
// - packages/bitcoin-knots/src/node/blockstorage.cpp
// - packages/bitcoin-knots/src/validation.cpp

use super::workflow::AchievedCheck;
use super::*;
use crate::PersistMode;
use crate::storage::canonical_history::SubmissionInventory;
use open_bitcoin_core::{
    chainstate::{AnchoredBlock, ChainstateError},
    consensus::{ScriptVerifyFlags, block_hash},
};

struct BranchPaths {
    disconnect: Vec<Block>,
    replacements: Vec<AnchoredBlock>,
}

impl CanonicalSubmissionCommand {
    pub(super) fn retain_side_body(
        &self,
        network: &mut SubmissionNetwork,
        block: Block,
        mut inventory: SubmissionInventory<'_>,
    ) -> SubmissionResult<CanonicalSubmissionOutcome> {
        let needs_retention = inventory
            .project_retained_side_body(&block)
            .map_err(operational("side body resource projection"))?;
        inventory
            .check_current_source()
            .map_err(operational("side body inventory freshness"))?;
        self.check_owner(network)?;
        if needs_retention {
            self.store
                .save_block(&block, PersistMode::Sync)
                .map_err(operational("retained side body"))?;
            inventory.into_achieved_check()(&self.store)
                .map_err(operational("side body achieved inventory"))?;
            self.store
                .check_submission_physical_inventory()
                .map_err(operational("side body physical inventory"))?;
        }
        network
            .blocks_by_hash
            .insert(block_hash(&block.header), block);
        Ok(CanonicalSubmissionOutcome::Inconclusive)
    }

    pub(super) fn activate_stronger_branch<'a>(
        &'a self,
        network: &mut SubmissionNetwork,
        block: Block,
        observed: i64,
        mut inventory: SubmissionInventory<'a>,
        maybe_achieved: &mut Option<AchievedCheck<'a>>,
    ) -> SubmissionResult<CanonicalSubmissionOutcome> {
        let hash = block_hash(&block.header);
        let Some(BranchPaths {
            disconnect,
            replacements,
        }) = self.branch_paths(network, &block)?
        else {
            return self.retain_side_body(network, block, inventory);
        };
        let now = u64::try_from(observed).map_err(operational("branch clock range"))?;
        let flags = ScriptVerifyFlags::NONE;
        let prepared =
            match network.prepare_live_reorg(&disconnect, &replacements, flags, self.params) {
                Ok(prepared) => prepared,
                Err(ManagedNetworkError::Chainstate(ChainstateError::ReplacementBlock {
                    block_hash,
                    source,
                })) => {
                    let outcome =
                        super::workflow::connection_failure(network, block_hash, *source)?;
                    if block_hash != hash
                        && matches!(outcome, CanonicalSubmissionOutcome::Rejected(_))
                    {
                        return Ok(CanonicalSubmissionOutcome::Rejected(
                            "bad-prevblk".to_owned(),
                        ));
                    }
                    return Ok(outcome);
                }
                Err(error) => {
                    return Err(CanonicalSubmissionError::operation(
                        "stronger branch preparation",
                        error,
                    ));
                }
            };
        let manager = network.chainstate();
        let projected = manager
            .project_prepared_submission_reorg(&prepared)
            .map_err(operational("prepared branch state projection"))?;
        let position = projected
            .tip()
            .cloned()
            .ok_or_else(|| CanonicalSubmissionError::support("prepared branch tip missing"))?;
        inventory
            .project_prepared_reorg(&replacements, &projected, &network.header_entries())
            .map_err(operational("prepared branch native projection"))?;
        inventory
            .project_basic_reorg(
                &replacements,
                &projected,
                manager.maybe_basic_index_progress(),
            )
            .map_err(operational("prepared branch BASIC projection"))?;
        inventory
            .check_current_source()
            .map_err(operational("prepared branch inventory freshness"))?;
        self.check_owner(network)?;
        *maybe_achieved = Some(inventory.into_achieved_check());
        self.store
            .save_block(&block, PersistMode::Sync)
            .map_err(operational("retained winning body"))?;
        let transition = network
            .commit_live_reorg(
                prepared,
                &disconnect,
                &replacements,
                open_bitcoin_mempool::ReorgLifecycleContext::new(
                    open_bitcoin_mempool::PolicyTime::from_unix_seconds(observed),
                ),
                flags,
                self.params,
            )
            .map_err(operational("winning branch publication"))?;
        if transition.connected.last() != Some(&position)
            || !self.accepted_identity(network, &block)?
        {
            return Err(CanonicalSubmissionError::support(
                "branch absorption receipt unavailable",
            ));
        }
        super::super::automatic_prune::flush(
            network,
            &self.network.automatic_prune,
            FlushMode::Always,
            FlushPolicyTime::from_unix_seconds(now),
            crate::chainstate::probe_disk_free_bytes(self.store.datadir()),
        )
        .map_err(operational("winning branch full flush"))?;
        Ok(CanonicalSubmissionOutcome::Accepted(position))
    }

    fn branch_paths(
        &self,
        network: &SubmissionNetwork,
        candidate: &Block,
    ) -> SubmissionResult<Option<BranchPaths>> {
        let headers = network.peer_manager.header_store();
        let active = network.chainstate().chainstate().active_chain();
        let candidate_hash = block_hash(&candidate.header);
        let mut hash = candidate_hash;
        let mut path: Vec<&open_bitcoin_network::HeaderEntry> = Vec::new();
        for _ in 0..FjallNodeStore::canonical_history_reopen_limit() {
            if let Some(ancestor) = active
                .iter()
                .position(|position| position.block_hash == hash)
            {
                let mut replacements = Vec::with_capacity(path.len());
                for entry in path.into_iter().rev() {
                    let block = if entry.block_hash == candidate_hash {
                        candidate.clone()
                    } else {
                        let Some(body) = self
                            .store
                            .load_block(entry.block_hash)
                            .map_err(operational("side ancestor body read"))?
                        else {
                            return Ok(None);
                        };
                        body
                    };
                    if block.header != entry.header || block_hash(&block.header) != entry.block_hash
                    {
                        return Err(CanonicalSubmissionError::support(
                            "replacement body identity changed",
                        ));
                    }
                    replacements.push(AnchoredBlock {
                        block,
                        chain_work: entry.chain_work,
                    });
                }
                let disconnect = active[ancestor + 1..]
                    .iter()
                    .rev()
                    .map(|position| {
                        self.store
                            .load_block(position.block_hash)
                            .map_err(operational("displaced body read"))?
                            .filter(|block| block.header == position.header)
                            .ok_or_else(|| {
                                CanonicalSubmissionError::support("displaced body unavailable")
                            })
                    })
                    .collect::<SubmissionResult<Vec<_>>>()?;
                return Ok(Some(BranchPaths {
                    disconnect,
                    replacements,
                }));
            }
            // Follow the whole header path before testing payload availability. A
            // missing intermediate body must not conceal a known failed ancestor.
            if headers
                .maybe_body_eligibility(hash)
                .is_some_and(|facts| facts.failed)
            {
                return Err(CanonicalSubmissionError::support(
                    "replacement crosses failed branch ancestry",
                ));
            }
            let entry = headers.entry(&hash).ok_or_else(|| {
                CanonicalSubmissionError::support("replacement header unavailable")
            })?;
            path.push(entry);
            hash = entry.header.previous_block_hash;
        }
        Err(CanonicalSubmissionError::support(
            "replacement ancestry exceeds retained bound",
        ))
    }
}
