// Parity breadcrumbs:
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/txdb.cpp

use super::*;

type AchievedCheck<'a> = Box<dyn FnOnce(&FjallNodeStore) -> Result<(), crate::StorageError> + 'a>;

impl CanonicalSubmissionCommand {
    pub(super) fn execute<'a>(
        &'a self,
        network: &mut SubmissionNetwork,
        raw: WireBlock,
        maybe_achieved: &mut Option<AchievedCheck<'a>>,
    ) -> SubmissionResult<CanonicalSubmissionOutcome> {
        use crate::network::lifecycle_projection::LifecycleProjectionPlan;
        use crate::storage::validation_history::BlockValidationIdentity;
        use open_bitcoin_core::consensus::{CandidateRules, block_hash};
        self.check_owner(network)?;
        if let Err(source) =
            open_bitcoin_core::consensus::block::intrinsic::check_submission_wire_block(
                &raw,
                &self.params,
            )
        {
            return Ok(CanonicalSubmissionOutcome::Rejected(
                source.reject_reason.to_owned(),
            ));
        }
        let mut block = raw
            .try_into_block()
            .map_err(operational("domain conversion"))?;
        let hash = block_hash(&block.header);
        if self.accepted_identity(network, &block)? {
            return Ok(CanonicalSubmissionOutcome::Duplicate(hash));
        }
        self.check_retained_capacity(network, hash)?;
        if network
            .peer_manager
            .header_store()
            .maybe_body_eligibility(hash)
            .is_some_and(|facts| facts.failed)
        {
            return Ok(CanonicalSubmissionOutcome::Rejected(
                "duplicate-invalid".to_owned(),
            ));
        }
        let headers = network.peer_manager.header_store();
        if headers.entry(&hash).is_none()
            && headers.entry(&block.header.previous_block_hash).is_some()
            && headers
                .maybe_body_eligibility(block.header.previous_block_hash)
                .is_some_and(|facts| facts.failed)
        {
            return Ok(CanonicalSubmissionOutcome::Rejected(
                "bad-prevblk".to_owned(),
            ));
        }
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(operational("shell clock"))?
            .as_secs();
        let observed = i64::try_from(now).map_err(operational("clock range"))?;
        let mut headers = network.peer_manager.header_store().clone();
        let inserted =
            match headers.insert_canonical_header_detailed(block.header.clone(), observed) {
                Ok(inserted) => inserted,
                Err(error) => {
                    return error.fold(
                        |_, reason| Ok(CanonicalSubmissionOutcome::Rejected(reason)),
                        |_, _| {
                            Ok(CanonicalSubmissionOutcome::Rejected(
                                "prev-blk-not-found".to_owned(),
                            ))
                        },
                        |e| {
                            Err(CanonicalSubmissionError::operation(
                                "header owner/history",
                                e,
                            ))
                        },
                    );
                }
            };
        let identity =
            BlockValidationIdentity::new(hash, block.header.previous_block_hash, inserted.height)
                .map_err(operational("admitted identity"))?;
        let mut early = self
            .store
            .inspect_submission_inventory()
            .map_err(operational("early native inventory"))?;
        early
            .project_admitted_headers(&headers.entries().cloned().collect::<Vec<_>>(), identity)
            .map_err(operational("early native resource projection"))?;
        early
            .check_current_source()
            .map_err(operational("early inventory freshness"))?;
        self.check_owner(network)?;
        // Earned admission is volatile; these real durable writes are separately fallible.
        network.peer_manager.seed_header_store(headers);
        network
            .publish_admitted_header_identities(vec![identity])
            .map_err(operational("KnownOnly publication"))?;
        network
            .persist_validation_header_snapshot()
            .map_err(operational("admitted header persistence"))?;
        drop(early);
        // Own early writes require a new source observation before the full forecast.
        let mut inventory = self
            .store
            .inspect_submission_inventory()
            .map_err(operational("prepared native inventory"))?;
        let headers = network.peer_manager.header_store();
        let history = headers
            .canonical_header_history(hash)
            .map_err(operational("earned history"))?;
        let context = headers
            .canonical_header_validation_context(&block.header)
            .map_err(operational("held context"))?;
        let rules = CandidateRules::derive(&history);
        open_bitcoin_core::consensus::block::update_uncommitted_block_structures(
            &mut block, &rules,
        );
        let mut headers = headers.clone();
        if let Err(error) = headers.check_submission_body_eligibility_detailed(
            &block,
            self.params,
            context.current_time,
        ) {
            return match error {
                open_bitcoin_network::BodyEligibilityError::Validation(source) => {
                    if headers
                        .maybe_body_eligibility(hash)
                        .is_some_and(|facts| facts.failed)
                    {
                        network.peer_manager.seed_header_store(headers);
                    }
                    Ok(CanonicalSubmissionOutcome::Rejected(
                        source.reject_reason.to_owned(),
                    ))
                }
                open_bitcoin_network::BodyEligibilityError::Network(error) => Err(
                    CanonicalSubmissionError::operation("live body registry", error),
                ),
            };
        }
        network.peer_manager.seed_header_store(headers);
        network.blocks_by_hash.insert(hash, block.clone());
        if network
            .chainstate()
            .chainstate()
            .tip()
            .is_none_or(|tip| tip.block_hash != block.header.previous_block_hash)
        {
            return Err(CanonicalSubmissionError::support(
                "candidate does not extend actual accepted tip",
            ));
        }
        network
            .chainstate_mut()
            .admit_canonical_body_header(&block.header, context.current_time)
            .map_err(operational("Coins header owner"))?;
        let authorization = network
            .authorize_connect(&block, self.params)
            .map_err(operational("body preference"))?;
        let prepared = match network.chainstate().prepare_authorized_connect(
            authorization,
            &block,
            context.current_time,
            rules.script_flags(),
            self.params,
        ) {
            Ok(prepared) => prepared,
            Err(error) => return connection_failure(network, hash, error),
        };
        let position = prepared.position().clone();
        let lifecycle = network
            .mempool
            .mempool()
            .prepare_connected_block_transition(
                &block,
                crate::network::mempool_lifecycle::block_lifecycle_context(
                    observed,
                    position.height,
                ),
            )
            .map_err(operational("mempool preparation"))?;
        let plan = LifecycleProjectionPlan::prepare(network, network.authority_epoch(), lifecycle)
            .map_err(operational("lifecycle preparation"))?;
        let sealed = network
            .validate_prepared_lifecycle(plan)
            .map_err(operational("lifecycle validation"))?;
        let manager = network.chainstate();
        let projected = manager
            .project_prepared_submission(&prepared)
            .map_err(operational("prepared state projection"))?;
        let maybe_inputs = manager
            .maybe_prepared_submission_basic_inputs(&prepared)
            .map_err(operational("prepared BASIC inputs"))?;
        inventory
            .project_prepared_connect(&block, &projected, &network.header_entries())
            .map_err(operational("prepared native resource projection"))?;
        inventory
            .project_basic(
                &position,
                maybe_inputs.as_ref(),
                manager.maybe_basic_index_progress(),
            )
            .map_err(operational("prepared BASIC resource projection"))?;
        inventory
            .check_current_source()
            .map_err(operational("prepared inventory freshness"))?;
        self.check_owner(network)?;
        *maybe_achieved = Some(inventory.into_achieved_check());
        self.store
            .save_block(&block, crate::PersistMode::Sync)
            .map_err(operational("retained body"))?;
        #[cfg(test)]
        tests::before_absorb(&self.store);
        network
            .commit_connected_block_lifecycle_transaction(&block, &position, prepared, sealed)
            .map_err(operational("acceptance/publication"))?;
        if !self.accepted_identity(network, &block)? {
            return Err(CanonicalSubmissionError::support(
                "absorption receipt unavailable",
            ));
        }
        super::automatic_prune::flush(
            network,
            &self.network.automatic_prune,
            FlushMode::Always,
            FlushPolicyTime::from_unix_seconds(now),
            crate::chainstate::probe_disk_free_bytes(self.store.datadir()),
        )
        .map_err(operational("accepted full flush"))?;
        Ok(CanonicalSubmissionOutcome::Accepted(position))
    }

    pub(super) fn confirm_durable(
        &self,
        network: &SubmissionNetwork,
        position: &ChainPosition,
    ) -> SubmissionResult<()> {
        use open_bitcoin_core::chainstate::filter_index::catch_up::BasicIndexState;
        self.check_owner(network)?;
        self.check_retained_capacity(network, position.block_hash)?;
        let manager = network.chainstate();
        if manager.chainstate().tip() != Some(position) || manager.has_pending_validation_history()
        {
            return Err(CanonicalSubmissionError::support(
                "accepted target changed or publication pending",
            ));
        }
        let body = self
            .store
            .load_block(position.block_hash)
            .map_err(operational("durable body"))?
            .ok_or_else(|| CanonicalSubmissionError::support("durable body absent"))?;
        if !self.accepted_identity(network, &body)? {
            return Err(CanonicalSubmissionError::support(
                "durable acceptance absent",
            ));
        }
        if manager.maybe_basic_index_failure().is_some() {
            return Err(CanonicalSubmissionError::support(
                "accepted BASIC owner failed",
            ));
        }
        if let Some(progress) = manager.maybe_basic_index_progress()
            && progress.state() != BasicIndexState::Disabled
            && (progress
                .maybe_safe_durable_endpoint()
                .is_none_or(|id| id.block_hash() != position.block_hash)
                || progress
                    .maybe_processed_endpoint()
                    .is_none_or(|id| id.block_hash() != position.block_hash))
        {
            return Err(CanonicalSubmissionError::support(
                "accepted BASIC target is not safely durable",
            ));
        }
        Ok(())
    }

    pub(super) fn check_retained_capacity(
        &self,
        network: &SubmissionNetwork,
        hash: BlockHash,
    ) -> SubmissionResult<()> {
        let headers = network.peer_manager.header_store();
        let count = headers.entries().count();
        let limit = FjallNodeStore::canonical_history_reopen_limit();
        if count > limit || (count == limit && headers.entry(&hash).is_none()) {
            return Err(CanonicalSubmissionError::support(
                "canonical history reopen capacity unavailable",
            ));
        }
        Ok(())
    }
}

fn connection_failure(
    network: &mut SubmissionNetwork,
    hash: BlockHash,
    error: open_bitcoin_core::chainstate::ChainstateError,
) -> SubmissionResult<CanonicalSubmissionOutcome> {
    use open_bitcoin_core::chainstate::ChainstateError;
    use open_bitcoin_core::consensus::{BlockValidationResult, TxValidationResult};
    let (reason, permanent) = match &error {
        ChainstateError::BlockValidation { source } => (
            source.reject_reason,
            matches!(source.result, BlockValidationResult::Consensus),
        ),
        ChainstateError::TransactionValidation { source } => (
            source.reject_reason,
            matches!(
                source.result,
                TxValidationResult::Consensus
                    | TxValidationResult::MissingInputs
                    | TxValidationResult::PrematureSpend
            ),
        ),
        ChainstateError::MissingCoin { .. } => ("bad-txns-inputs-missingorspent", true),
        ChainstateError::OutputOverwrite { .. } => ("bad-txns-BIP30", true),
        _ => {
            return Err(CanonicalSubmissionError::operation(
                "Coins preparation",
                error,
            ));
        }
    };
    if permanent {
        network.note_candidate_connection_failure(hash, &error);
    }
    Ok(CanonicalSubmissionOutcome::Rejected(reason.to_owned()))
}
