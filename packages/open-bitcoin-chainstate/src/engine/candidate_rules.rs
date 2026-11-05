// Parity breadcrumbs:
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/chain.h
// - packages/bitcoin-knots/src/consensus/tx_verify.cpp

use super::*;
use open_bitcoin_consensus::chain_spec::NetworkRules;
use open_bitcoin_consensus::{
    CandidateHistory, CandidateRules, CanonicalChain, HistoryHeader, HistoryOrigin,
};
use open_bitcoin_primitives::BlockHeader;

const MAX_ADMISSION_HEADERS: usize = 4096;
const HEADER_HISTORY_WINDOW: usize = 155;

pub(super) fn transaction_context(
    transaction: &open_bitcoin_primitives::Transaction,
    inputs: Vec<open_bitcoin_consensus::TransactionInputContext>,
    history: &CandidateHistory,
    active_chain: &[ChainPosition],
) -> Result<open_bitcoin_consensus::TransactionValidationContext, ChainstateError> {
    let rules = history.rules();
    let required_creating_heights: Vec<_> = transaction
        .inputs
        .iter()
        .zip(&inputs)
        .filter(|(input, _)| {
            transaction.version >= 2
                && rules.enforce_sequence_locks()
                && input.sequence
                    & open_bitcoin_primitives::TransactionInput::SEQUENCE_LOCKTIME_DISABLE_FLAG
                    == 0
                && input.sequence
                    & open_bitcoin_primitives::TransactionInput::SEQUENCE_LOCKTIME_TYPE_FLAG
                    != 0
        })
        .map(|(_, coin)| coin.created_height.saturating_sub(1))
        .collect();
    let mut extended;
    let mut selected = history;
    if let Some(oldest) = required_creating_heights.iter().min()
        && history.median_time_past_at(*oldest).is_err()
        && history.origin() != HistoryOrigin::IndependentlySeeded
    {
        let lookback = history
            .parent()
            .height
            .checked_sub(oldest.saturating_sub(10))
            .and_then(|count| count.checked_add(1))
            .ok_or_else(|| context_error("creating-ancestor-height"))?
            as usize;
        if lookback > MAX_ADMISSION_HEADERS {
            return Err(context_error("creating-ancestor-window-bound"));
        }
        extended = history_from_chain(
            history.network_rules(),
            history.candidate(),
            active_chain,
            lookback.max(HEADER_HISTORY_WINDOW),
        )?;
        selected = &mut extended;
    }
    for (input, coin) in transaction.inputs.iter().zip(&inputs) {
        if transaction.version >= 2
            && rules.enforce_sequence_locks()
            && input.sequence
                & open_bitcoin_primitives::TransactionInput::SEQUENCE_LOCKTIME_DISABLE_FLAG
                == 0
            && input.sequence
                & open_bitcoin_primitives::TransactionInput::SEQUENCE_LOCKTIME_TYPE_FLAG
                != 0
        {
            let expected = selected
                .median_time_past_at(coin.created_height.saturating_sub(1))
                .map_err(context_error)?;
            if coin.created_median_time_past != expected {
                return Err(context_error("creating-ancestor-mtp-mismatch"));
            }
        }
    }
    open_bitcoin_consensus::TransactionValidationContext::from_candidate_history(
        transaction,
        inputs,
        selected,
    )
    .map_err(|source| ChainstateError::TransactionValidation { source })
}

#[derive(Debug)]
pub(super) struct PendingAdmission {
    disconnect: Vec<BlockHeader>,
    candidates: Vec<HeaderAdmission>,
}

#[derive(Debug)]
struct HeaderAdmission {
    header: BlockHeader,
    rules: CandidateRules,
    original_shell_now: i64,
}

pub(super) fn context_error(reason: impl fmt::Display) -> ChainstateError {
    ChainstateError::Serialization {
        context: "candidate history",
        reason: reason.to_string(),
    }
}

pub(super) fn history_from_chain(
    rules: NetworkRules,
    candidate: &BlockHeader,
    active_chain: &[ChainPosition],
    lookback: usize,
) -> Result<CandidateHistory, ChainstateError> {
    let mut rows = active_chain
        .iter()
        .rev()
        .take(lookback)
        .map(|position| HistoryHeader {
            chain: rules.chain(),
            height: position.height,
            block_hash: position.block_hash,
            header: position.header.clone(),
        });
    let parent = rows.next().ok_or(ChainstateError::MissingTip)?;
    CandidateHistory::check(rules, candidate.clone(), parent, rows.collect()).map_err(context_error)
}

impl WorkAuthority {
    pub(super) fn bound_params(&self, fallback: ConsensusParams) -> ConsensusParams {
        if self.maybe_canonical_rules.is_some() {
            self.maybe_bound_params().unwrap_or(fallback)
        } else {
            fallback
        }
    }
}

impl<V: CoinsView> Chainstate<V> {
    /// Selected identity exists only on a proved-root owner with usable live ancestry.
    pub fn maybe_canonical_chain(&self) -> Option<CanonicalChain> {
        self.validate_live_origin().ok()?;
        self.work_authority
            .maybe_canonical_rules
            .map(|rules| rules.chain())
    }

    /// Validate one exact header with an explicitly trusted shell clock on this owner.
    /// This is pure validation of supplied time, not proof of another header store's membership.
    pub fn admit_canonical_header(
        &mut self,
        header: &BlockHeader,
        shell_observed_now: i64,
    ) -> Result<(), ChainstateError> {
        self.admit_canonical_reorg_headers(&[], &[(header.clone(), shell_observed_now)])
    }

    /// Retain a single bounded batch for this generation and exact disconnect branch.
    /// Repeating an identical pending batch preserves its first successful observations.
    pub fn admit_canonical_reorg_headers(
        &mut self,
        disconnect: &[BlockHeader],
        candidates: &[(BlockHeader, i64)],
    ) -> Result<(), ChainstateError> {
        let rules = self.canonical_selection()?;
        if candidates.is_empty()
            || candidates.len() > MAX_ADMISSION_HEADERS
            || disconnect.len() > MAX_ADMISSION_HEADERS
        {
            return Err(context_error("canonical-admission-batch-bound"));
        }
        let mut branch = self.active_chain.clone();
        for header in disconnect {
            let tip = branch.last().ok_or(ChainstateError::MissingTip)?;
            if tip.height == 0 || &tip.header != header {
                return Err(context_error("canonical-admission-disconnect-mismatch"));
            }
            branch.pop();
        }
        if self
            .work_authority
            .maybe_admission
            .as_ref()
            .is_some_and(|pending| {
                pending.disconnect == disconnect
                    && pending.candidates.len() == candidates.len()
                    && pending
                        .candidates
                        .iter()
                        .zip(candidates)
                        .all(|(held, (header, _))| held.header == *header)
            })
        {
            return Ok(());
        }
        let mut admitted = Vec::with_capacity(candidates.len());
        for (header, now) in candidates {
            let history = history_from_chain(rules, header, &branch, HEADER_HISTORY_WINDOW)?;
            history
                .check_header_rules(*now)
                .map_err(|source| ChainstateError::BlockValidation { source })?;
            let parent = branch.last().ok_or(ChainstateError::MissingTip)?;
            let chain_work =
                super::work_authority::derive_child_work(parent.chain_work, header.bits)?;
            let position = ChainPosition::new(
                header.clone(),
                history.height(),
                chain_work,
                compute_median_time_past(&branch, Some(header.time)),
            );
            admitted.push(HeaderAdmission {
                header: header.clone(),
                rules: history.rules(),
                original_shell_now: *now,
            });
            branch.push(position);
        }
        self.work_authority.invalidate_stages();
        self.work_authority.maybe_admission = Some(PendingAdmission {
            disconnect: disconnect.to_vec(),
            candidates: admitted,
        });
        Ok(())
    }

    fn canonical_selection(&self) -> Result<NetworkRules, ChainstateError> {
        self.validate_live_origin()?;
        let rules = self
            .work_authority
            .maybe_canonical_rules
            .ok_or_else(|| context_error("missing-canonical-root"))?;
        match rules.chain() {
            CanonicalChain::Regtest => Ok(rules),
            CanonicalChain::Signet => Err(context_error("signet-challenge-validation-unavailable")),
            _ => Err(context_error(
                "canonical-checkpoint-prerequisites-unavailable",
            )),
        }
    }

    pub(super) fn check_admission_batch(
        &self,
        disconnect: &[BlockHeader],
        headers: &[BlockHeader],
    ) -> Result<(), ChainstateError> {
        if self.work_authority.maybe_canonical_rules.is_none() {
            return Ok(());
        }
        self.canonical_selection()?;
        let pending = self
            .work_authority
            .maybe_admission
            .as_ref()
            .ok_or_else(|| context_error("missing-canonical-header-admission"))?;
        if pending.disconnect != disconnect
            || pending.candidates.len() != headers.len()
            || !pending
                .candidates
                .iter()
                .zip(headers)
                .all(|(held, header)| held.header == *header)
        {
            return Err(context_error("canonical-admission-branch-mismatch"));
        }
        Ok(())
    }

    pub(super) fn selected_candidate_context(
        &self,
        branch: &[ChainPosition],
        header: &BlockHeader,
        disconnect: &[BlockHeader],
        index: usize,
    ) -> Result<Option<(CandidateHistory, i64)>, ChainstateError> {
        if self.work_authority.maybe_canonical_rules.is_none() {
            return Ok(None);
        }
        let rules = self.canonical_selection()?;
        let pending = self
            .work_authority
            .maybe_admission
            .as_ref()
            .ok_or_else(|| context_error("missing-canonical-header-admission"))?;
        let held = pending
            .candidates
            .get(index)
            .ok_or_else(|| context_error("missing-canonical-header-admission"))?;
        if pending.disconnect != disconnect || held.header != *header {
            return Err(context_error("canonical-admission-branch-mismatch"));
        }
        let history = history_from_chain(rules, header, branch, HEADER_HISTORY_WINDOW)?;
        if history.rules() != held.rules {
            return Err(context_error("canonical-admission-context-mismatch"));
        }
        Ok(Some((history, held.original_shell_now)))
    }

    /// Conditional historical execution only. Imported fixtures never acquire canonical
    /// selection, live work, or product proof, even when all scalar rules match.
    pub fn stage_seeded_historical_block(
        &self,
        block: &Block,
        history: &CandidateHistory,
        original_shell_now: i64,
    ) -> Result<StagedChainstateConnect, ChainstateError> {
        if !self.work_authority.is_mechanical_fixture()
            || history.origin() != HistoryOrigin::IndependentlySeeded
            || history.branch().len() > MAX_ADMISSION_HEADERS
            || history.branch().is_empty()
            || self.tip().is_none_or(|tip| {
                tip.header != history.parent().header || tip.height != history.parent().height
            })
            || history.branch().iter().any(|row| {
                !self.active_chain.iter().any(|position| {
                    position.height == row.height
                        && position.block_hash == row.block_hash
                        && position.header == row.header
                })
            })
        {
            return Err(context_error("seeded-historical-envelope-mismatch"));
        }
        let params = history.rules().consensus_params();
        let binding = self.work_authority.bind(self.tip(), params);
        let mut overlay = CoinsOverlay::new();
        let parent = self.tip().ok_or(ChainstateError::MissingTip)?;
        let expected_work =
            super::work_authority::derive_child_work(parent.chain_work, block.header.bits)?;
        let (position, undo, next_confirmed_txid_counts) = apply_connect_on_overlay(
            &mut overlay,
            &self.coins,
            &self.active_chain,
            &self.maybe_confirmed_txid_counts,
            block,
            expected_work,
            true,
            original_shell_now,
            history.rules().script_flags(),
            params,
            Some((history, original_shell_now)),
        )?;
        overlay.set_best_block(position.block_hash);
        let mut next_active_chain = self.active_chain.clone();
        next_active_chain.push(position.clone());
        Ok(StagedChainstateConnect {
            binding,
            overlay,
            undo,
            position,
            next_active_chain,
            next_confirmed_txid_counts,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use open_bitcoin_consensus::chain_proof::ProofWork;
    use open_bitcoin_consensus::chain_spec::CanonicalGenesisProof;
    use open_bitcoin_primitives::{
        Amount, OutPoint, ScriptBuf, Transaction, TransactionInput, TransactionOutput, Txid,
    };

    #[test]
    fn linked_regtest_creating_mtp_extends_past_short_header_window() {
        // Arrange
        let rules = CanonicalChain::Regtest.spec().rules();
        let mut active = Vec::new();
        let mut parent_hash = BlockHash::default();
        let first_time = CanonicalChain::Regtest.spec().genesis_header().time;
        for height in 0..=160_u32 {
            let header = if height == 0 {
                CanonicalChain::Regtest.spec().genesis_header().clone()
            } else {
                BlockHeader {
                    version: 4,
                    previous_block_hash: parent_hash,
                    time: first_time + height,
                    bits: rules.pow_limit_bits(),
                    nonce: height,
                    ..Default::default()
                }
            };
            let position = ChainPosition::new(header, height, ProofWork::ZERO, 0);
            parent_hash = position.block_hash;
            active.push(position);
        }
        let candidate = BlockHeader {
            version: 4,
            previous_block_hash: parent_hash,
            time: first_time + 161,
            bits: rules.pow_limit_bits(),
            ..Default::default()
        };
        let history = history_from_chain(rules, &candidate, &active, HEADER_HISTORY_WINDOW)
            .expect("checked short linked window");
        assert!(history.median_time_past_at(1).is_err());
        let transaction = Transaction {
            version: 2,
            inputs: vec![TransactionInput {
                previous_output: OutPoint {
                    txid: Txid::from_byte_array([7; 32]),
                    vout: 0,
                },
                script_sig: ScriptBuf::default(),
                sequence: TransactionInput::SEQUENCE_LOCKTIME_TYPE_FLAG | 1,
                witness: Default::default(),
            }],
            outputs: vec![TransactionOutput {
                value: Amount::from_sats(1).expect("value"),
                script_pubkey: ScriptBuf::from_serialized_bytes(vec![0x51]),
            }],
            lock_time: 0,
        };
        let input = open_bitcoin_consensus::TransactionInputContext {
            spent_output: open_bitcoin_consensus::SpentOutput {
                value: Amount::from_sats(2).expect("value"),
                script_pubkey: ScriptBuf::from_serialized_bytes(vec![0x51]),
                is_coinbase: false,
            },
            created_height: 2,
            created_median_time_past: i64::from(active[1].header.time),
        };

        // Act
        let context = transaction_context(&transaction, vec![input.clone()], &history, &active)
            .expect("extended linked creating MTP");

        // Assert
        assert_eq!(
            context.inputs[0].created_median_time_past,
            i64::from(active[1].header.time)
        );
        assert_eq!(context.spend_height, 161);

        // Arrange: the held short history is sound; only an older extended row is altered.
        let mut altered_active = active.clone();
        let mut altered_header = altered_active[1].header.clone();
        altered_header.nonce ^= 1;
        altered_active[1] = ChainPosition::new(altered_header, 1, ProofWork::ZERO, 0);
        assert_ne!(altered_active[1].block_hash, active[1].block_hash);
        assert_eq!(altered_active.last(), active.last());

        // Act / Assert: extension cannot accept MTP from the broken older link.
        assert!(matches!(
            transaction_context(&transaction, vec![input], &history, &altered_active),
            Err(ChainstateError::Serialization { context: "candidate history", reason })
                if reason == "candidate history: ParentMismatch { height: 1 }"
        ));
    }

    fn canonical_child() -> (Chainstate, Block) {
        let spec = CanonicalChain::Regtest.spec();
        let proof = CanonicalGenesisProof::prove(spec, &spec.genesis_block()).expect("proved root");
        let chain = Chainstate::from_canonical_genesis(
            &proof,
            CoinsCache::from_parent(MemoryCoinsView::from_coins(
                HashMap::new(),
                Some(proof.hash()),
            )),
        )
        .expect("canonical owner");
        let coinbase = Transaction {
            version: 2,
            inputs: vec![TransactionInput {
                previous_output: OutPoint::null(),
                script_sig: ScriptBuf::from_serialized_bytes(vec![0x51, 0x51]),
                sequence: TransactionInput::SEQUENCE_FINAL,
                witness: Default::default(),
            }],
            outputs: vec![TransactionOutput {
                value: Amount::from_sats(1).expect("valid reward"),
                script_pubkey: ScriptBuf::from_serialized_bytes(vec![0x51]),
            }],
            lock_time: 0,
        };
        let mut block = Block {
            header: BlockHeader {
                version: 4,
                previous_block_hash: proof.hash(),
                time: proof.header().time + 600,
                bits: proof.header().bits,
                ..Default::default()
            },
            transactions: vec![coinbase],
        };
        block.header.merkle_root = open_bitcoin_consensus::block_merkle_root(&block.transactions)
            .expect("merkle")
            .0;
        for _ in 0..4096 {
            if open_bitcoin_consensus::check_block_header(&block.header).is_err() {
                break;
            }
            block.header.nonce = block.header.nonce.wrapping_add(1);
        }
        assert!(open_bitcoin_consensus::check_block_header(&block.header).is_err());
        for _ in 0..4096 {
            if open_bitcoin_consensus::check_block_header(&block.header).is_ok() {
                break;
            }
            block.header.nonce = block.header.nonce.wrapping_add(1);
        }
        assert!(open_bitcoin_consensus::check_block_header(&block.header).is_ok());
        (chain, block)
    }

    #[test]
    fn wrong_held_header_refuses_without_publishing_selected_context() {
        // Arrange
        let (mut chain, block) = canonical_child();
        chain
            .admit_canonical_header(&block.header, i64::from(block.header.time))
            .expect("held header");
        let before = chain.snapshot();
        chain
            .work_authority
            .maybe_admission
            .as_mut()
            .expect("pending")
            .candidates[0]
            .header
            .nonce ^= 1;

        // Act
        let error = chain
            .selected_candidate_context(&chain.active_chain, &block.header, &[], 0)
            .expect_err("wrong held header");

        // Assert
        assert!(
            matches!(error, ChainstateError::Serialization { reason, .. }
            if reason == "canonical-admission-branch-mismatch")
        );
        assert_eq!(chain.snapshot(), before);
    }

    #[test]
    fn seeded_rules_for_same_candidate_refuse_selected_context_and_reorg_stage() {
        // Arrange
        let (mut chain, block) = canonical_child();
        chain
            .admit_canonical_header(&block.header, i64::from(block.header.time))
            .expect("held header");
        let rules = chain.canonical_selection().expect("Regtest owner");
        let linked = history_from_chain(
            rules,
            &block.header,
            &chain.active_chain,
            HEADER_HISTORY_WINDOW,
        )
        .expect("linked candidate");
        let seeded = CandidateHistory::check_seeded(
            rules,
            block.header.clone(),
            linked.parent().clone(),
            linked.branch()[1..].to_vec(),
            None,
        )
        .expect("structural seed remains unproved");
        assert_eq!(seeded.origin(), HistoryOrigin::IndependentlySeeded);
        chain
            .work_authority
            .maybe_admission
            .as_mut()
            .expect("pending")
            .candidates[0]
            .rules = seeded.rules();
        let before = chain.snapshot();
        let anchored = AnchoredBlock {
            block: block.clone(),
            chain_work: chain.tip().expect("tip").chain_work.wrapping_add(
                open_bitcoin_consensus::chain_proof::block_proof(block.header.bits).expect("work"),
            ),
        };

        // Act
        let context_error = chain
            .selected_candidate_context(&chain.active_chain, &block.header, &[], 0)
            .expect_err("seeded held rules cannot authorize context");
        let stage_error = chain
            .stage_reorg(
                &[],
                &[anchored],
                ScriptVerifyFlags::NONE,
                ConsensusParams::default(),
            )
            .expect_err("seeded held rules cannot authorize stage");

        // Assert
        assert!(
            matches!(context_error, ChainstateError::Serialization { reason, .. }
            if reason == "canonical-admission-context-mismatch")
        );
        assert!(
            matches!(stage_error, ChainstateError::Serialization { reason, .. }
            if reason == "canonical-admission-context-mismatch")
        );
        assert_eq!(chain.snapshot(), before);
    }
}
