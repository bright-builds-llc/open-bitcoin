// Parity breadcrumbs:
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/chain.h
// - packages/bitcoin-knots/src/node/blockstorage.cpp

use super::*;
use open_bitcoin_consensus::context::{MinDifficultyRecoveryTarget, RetargetAnchor};
use open_bitcoin_consensus::{BlockValidationContext, chain_proof::ProofWork};
use open_bitcoin_primitives::Block;
use std::collections::VecDeque;
use std::sync::Arc;

/// Actual body consensus rejection is distinct from registry/context/local refusal.
#[derive(Debug)]
pub enum BodyEligibilityError {
    Validation(open_bitcoin_consensus::BlockValidationError),
    Network(NetworkError),
}

impl From<NetworkError> for BodyEligibilityError {
    fn from(error: NetworkError) -> Self {
        Self::Network(error)
    }
}

impl BodyEligibilityError {
    /// Preserve the established public peer-validation error spelling.
    pub fn into_network_error(self) -> NetworkError {
        match self {
            Self::Validation(error) => invalid_header(error.reject_reason),
            Self::Network(error) => error,
        }
    }
}

/// Read-only transaction-linkage evidence. This does not certify script/UTXO acceptance.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BodyEligibilityFacts {
    pub block_hash: BlockHash,
    pub chain_work: ProofWork,
    pub maybe_sequence: Option<u64>,
    pub available: bool,
    pub failed: bool,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
struct BodyState {
    maybe_linkage_seal: Option<std::sync::Arc<()>>,
    maybe_sequence: Option<u64>,
    available: bool,
    failed: bool,
    order_unknown: bool,
}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct BodyEligibilityRegistry {
    epoch: Arc<()>,
    bodies: BTreeMap<BlockHash, BodyState>,
    pending: BTreeMap<BlockHash, Vec<BlockHash>>,
    next_sequence: u64,
    capacity: usize,
}

impl Clone for BodyEligibilityRegistry {
    fn clone(&self) -> Self {
        Self {
            epoch: Arc::new(()),
            bodies: self.bodies.clone(),
            pending: self.pending.clone(),
            next_sequence: self.next_sequence,
            capacity: self.capacity,
        }
    }
}

impl Default for BodyEligibilityRegistry {
    fn default() -> Self {
        Self {
            epoch: Arc::new(()),
            bodies: BTreeMap::new(),
            pending: BTreeMap::new(),
            next_sequence: 2,
            capacity: crate::message::MAX_INV_SIZE,
        }
    }
}

impl HeaderStore {
    /// Bind current facts to this mutable registry, independently of history seals.
    pub fn body_eligibility_epoch(&self) -> crate::BodyEligibilityEpoch {
        crate::BodyEligibilityEpoch {
            epoch: Arc::downgrade(&self.body_eligibility.epoch),
        }
    }
    /// Return actual linkage identity, never a constructor from copied numerical facts.
    pub fn maybe_body_eligibility_binding(
        &self,
        hash: BlockHash,
    ) -> Option<crate::BodyEligibilityBinding> {
        Some(crate::BodyEligibilityBinding {
            seal: self
                .body_eligibility
                .bodies
                .get(&hash)?
                .maybe_linkage_seal
                .clone()?,
            facts: self.maybe_body_eligibility(hash)?,
        })
    }
    /// Check full transaction validity before recording body linkage, independently of coins.
    pub fn check_body_eligibility(
        &mut self,
        block: &Block,
        params: ConsensusParams,
        current_time: i64,
    ) -> Result<(), NetworkError> {
        self.check_body_eligibility_inner(block, params, current_time, false)
            .map_err(BodyEligibilityError::into_network_error)
    }

    fn check_body_eligibility_inner(
        &mut self,
        block: &Block,
        params: ConsensusParams,
        current_time: i64,
        order_unknown: bool,
    ) -> Result<(), BodyEligibilityError> {
        let hash = block_hash(&block.header);
        if self
            .maybe_consensus_params
            .is_some_and(|selected| selected != params)
            || (self.maybe_consensus_params.is_none() && !self.entries.is_empty())
        {
            return Err(invalid_header("body-consensus-params-mismatch").into());
        }
        if self.entries.contains_key(&hash) && self.maybe_proof_facts(hash).is_none() {
            return Err(invalid_header("unproved-body-header").into());
        }
        let context = self.header_validation_context(&block.header, current_time, params)?;
        open_bitcoin_consensus::block::check_block_header_with_params(&block.header, &params)
            .map_err(BodyEligibilityError::Validation)?;
        open_bitcoin_consensus::block::check_block_contextual(block, &context)
            .map_err(BodyEligibilityError::Validation)?;
        self.insert_header_with_params(block.header.clone(), &params)?;
        self.body_eligibility
            .record_with_order(hash, self.parents[&hash], order_unknown)?;
        Ok(())
    }

    /// Revalidate imported bytes. Their iteration order cannot establish arrival parity.
    /// Already checked receipts, including pending children, retain their provenance.
    pub fn check_cached_body_eligibility(
        &mut self,
        block: &Block,
        params: ConsensusParams,
        current_time: i64,
    ) -> Result<(), NetworkError> {
        self.check_cached_body_eligibility_detailed(block, params, current_time)
            .map_err(BodyEligibilityError::into_network_error)
    }

    /// Run the same check once while retaining its actual consensus failure cause.
    pub fn check_cached_body_eligibility_detailed(
        &mut self,
        block: &Block,
        params: ConsensusParams,
        current_time: i64,
    ) -> Result<(), BodyEligibilityError> {
        self.check_body_eligibility_inner(block, params, current_time, true)
    }

    pub fn maybe_body_eligibility(&self, hash: BlockHash) -> Option<BodyEligibilityFacts> {
        let state = self.body_eligibility.bodies.get(&hash)?;
        Some(BodyEligibilityFacts {
            block_hash: hash,
            chain_work: self.maybe_proof_facts(hash)?.cumulative,
            maybe_sequence: state
                .maybe_sequence
                .filter(|_| self.has_known_body_order(hash)),
            available: state.available,
            failed: state.failed,
        })
    }

    fn has_known_body_order(&self, hash: BlockHash) -> bool {
        let mut current = hash;
        for _ in 0..self.body_eligibility.bodies.len() {
            let Some(state) = self.body_eligibility.bodies.get(&current) else {
                return false;
            };
            if state.order_unknown {
                return false;
            }
            match self.parents.get(&current) {
                Some(Some(parent)) => current = *parent,
                Some(None) => return true,
                None => return false,
            }
        }
        false
    }

    /// Enumerate all retained eligible bodies in activation preference order.
    pub fn eligible_body_candidates(
        &self,
        active_chain: &[BlockHash],
    ) -> Vec<BodyEligibilityFacts> {
        let active: BTreeSet<_> = active_chain.iter().copied().collect();
        let mut candidates: Vec<_> = self
            .body_eligibility
            .bodies
            .keys()
            .copied()
            .filter(|hash| {
                self.body_eligibility
                    .activatable(*hash, &self.parents, &active)
            })
            .filter_map(|hash| self.maybe_body_eligibility(hash))
            .collect();
        candidates.sort_by(|left, right| {
            right.chain_work.cmp(&left.chain_work).then_with(|| {
                left.maybe_sequence
                    .unwrap_or(u64::MAX)
                    .cmp(&right.maybe_sequence.unwrap_or(u64::MAX))
            })
        });
        candidates
    }

    /// Revoke current availability without erasing historical transaction linkage.
    pub fn mark_body_unavailable(&mut self, hash: BlockHash) {
        if let Some(state) = self.body_eligibility.bodies.get_mut(&hash)
            && state.available
        {
            state.available = false;
            self.body_eligibility.epoch = Arc::new(());
        }
    }

    /// Revoke a failed branch. Descendants are excluded by ancestry traversal.
    pub fn mark_body_failed(&mut self, hash: BlockHash) {
        if let Some(state) = self.body_eligibility.bodies.get_mut(&hash)
            && !state.failed
        {
            state.failed = true;
            self.body_eligibility.epoch = Arc::new(());
        }
    }
}

impl BodyEligibilityRegistry {
    #[cfg(test)]
    fn record(
        &mut self,
        hash: BlockHash,
        maybe_parent: Option<BlockHash>,
    ) -> Result<(), NetworkError> {
        self.record_with_order(hash, maybe_parent, false)
    }

    fn record_with_order(
        &mut self,
        hash: BlockHash,
        maybe_parent: Option<BlockHash>,
        order_unknown: bool,
    ) -> Result<(), NetworkError> {
        if let Some(existing) = self.bodies.get(&hash) {
            if !existing.available {
                // Requested pruned-body redownload and preciousblock are separate surfaces.
                return Err(invalid_header("body-redownload-eligibility-unavailable"));
            }
            return Ok(());
        }
        // Open Bitcoin resource policy; the pin has no corresponding registry cap.
        if self.bodies.len() >= self.capacity {
            return Err(invalid_header("body-eligibility-capacity"));
        }
        let parent_linked = maybe_parent.is_none_or(|parent| {
            self.bodies
                .get(&parent)
                .is_some_and(|state| state.maybe_sequence.is_some())
        });
        let mut release = Vec::new();
        if parent_linked {
            let mut queue = VecDeque::from([hash]);
            while let Some(current) = queue.pop_front() {
                release.push(current);
                if let Some(children) = self.pending.get(&current) {
                    queue.extend(children.iter().copied());
                }
            }
        }
        // Reserve the whole release before mutation, including counter exhaustion.
        let next = self
            .next_sequence
            .checked_add(release.len() as u64)
            .ok_or_else(|| invalid_header("body-eligibility-sequence-exhausted"))?;
        self.bodies.insert(
            hash,
            BodyState {
                available: true,
                order_unknown,
                ..Default::default()
            },
        );
        if let Some(parent) = maybe_parent.filter(|_| !parent_linked) {
            self.pending.entry(parent).or_default().push(hash);
        }
        for current in release {
            self.bodies.entry(current).and_modify(|state| {
                state.maybe_sequence = Some(self.next_sequence);
                state.maybe_linkage_seal = Some(std::sync::Arc::new(()));
            });
            self.next_sequence += 1;
            self.pending.remove(&current);
        }
        self.next_sequence = next;
        self.epoch = Arc::new(());
        Ok(())
    }

    fn activatable(
        &self,
        hash: BlockHash,
        parents: &BTreeMap<BlockHash, Option<BlockHash>>,
        active: &BTreeSet<BlockHash>,
    ) -> bool {
        let mut current = hash;
        for _ in 0..self.bodies.len() {
            let Some(state) = self.bodies.get(&current) else {
                return false;
            };
            if state.failed || state.maybe_sequence.is_none() {
                return false;
            }
            if active.contains(&current) {
                return true;
            }
            if !state.available {
                return false;
            }
            match parents.get(&current) {
                Some(Some(parent)) => current = *parent,
                Some(None) => return true,
                None => return false,
            }
        }
        false
    }
}

impl HeaderStore {
    /// Derive contextual inputs solely from accepted ancestry and selected parameters.
    pub fn header_validation_context(
        &self,
        header: &BlockHeader,
        current_time: i64,
        consensus_params: ConsensusParams,
    ) -> Result<BlockValidationContext, NetworkError> {
        let header_store = self;
        let has_known_headers = header_store.best_tip().is_some();
        let previous_block_hash = header.previous_block_hash;
        let has_parent = previous_block_hash.to_byte_array() != [0_u8; 32];

        if !has_parent {
            if has_known_headers
                && header_store
                    .maybe_proof_facts(block_hash(header))
                    .is_none_or(|facts| facts.height != 0)
            {
                return Err(NetworkError::InvalidHeader {
                    reject_reason: "bad-prevblk".to_string(),
                    maybe_debug_message: Some(
                        "unexpected additional genesis header in non-empty header store"
                            .to_string(),
                    ),
                });
            }
            return Ok(BlockValidationContext {
                height: 0,
                previous_header: BlockHeader::default(),
                maybe_retarget_anchor: None,
                maybe_min_difficulty_recovery_target: None,
                previous_median_time_past: 0,
                current_time,
                consensus_params,
            });
        }

        let parent = header_store
            .entry(&previous_block_hash)
            .ok_or(NetworkError::MissingHeaderAncestor(previous_block_hash))?;
        if header_store.maybe_proof_facts(parent.block_hash).is_none() {
            return Err(invalid_header("unproved-header-parent"));
        }
        let height = parent.height.saturating_add(1);
        let previous_median_time_past = header_store
            .median_time_past(parent.block_hash)
            .ok_or(NetworkError::MissingHeaderAncestor(parent.block_hash))?;
        let interval = difficulty_adjustment_interval(&consensus_params);
        let maybe_retarget_anchor =
            retarget_anchor_for_height(header_store, parent, height, interval, consensus_params)?;
        let maybe_min_difficulty_recovery_target = min_difficulty_recovery_target(
            header_store,
            parent,
            header,
            height,
            interval,
            consensus_params,
        )?;

        Ok(BlockValidationContext {
            height,
            previous_header: parent.header.clone(),
            maybe_retarget_anchor,
            maybe_min_difficulty_recovery_target,
            previous_median_time_past,
            current_time,
            consensus_params,
        })
    }
}

fn difficulty_adjustment_interval(consensus_params: &ConsensusParams) -> u32 {
    if consensus_params.pow_target_spacing_seconds <= 0 {
        return 1;
    }

    let interval =
        consensus_params.pow_target_timespan_seconds / consensus_params.pow_target_spacing_seconds;
    interval.max(1) as u32
}

fn retarget_anchor_for_height(
    header_store: &HeaderStore,
    parent: &HeaderEntry,
    height: u32,
    interval: u32,
    consensus_params: ConsensusParams,
) -> Result<Option<RetargetAnchor>, NetworkError> {
    if consensus_params.no_pow_retargeting || height == 0 || !height.is_multiple_of(interval) {
        return Ok(None);
    }

    let anchor_height = height.saturating_sub(interval);
    let anchor = header_store
        .ancestor_at_height(parent.block_hash, anchor_height)
        .ok_or(NetworkError::MissingHeaderAncestor(parent.block_hash))?;
    Ok(Some(RetargetAnchor {
        first_block_time: i64::from(anchor.header.time),
    }))
}

fn min_difficulty_recovery_target(
    header_store: &HeaderStore,
    parent: &HeaderEntry,
    header: &BlockHeader,
    height: u32,
    interval: u32,
    consensus_params: ConsensusParams,
) -> Result<Option<MinDifficultyRecoveryTarget>, NetworkError> {
    if !consensus_params.allow_min_difficulty_blocks
        || height == 0
        || height.is_multiple_of(interval)
    {
        return Ok(None);
    }

    let delayed_enough = i64::from(header.time)
        > i64::from(parent.header.time)
            + consensus_params
                .pow_target_spacing_seconds
                .saturating_mul(2);
    if delayed_enough {
        return Ok(None);
    }

    let mut current_hash = parent.block_hash;
    loop {
        let current = header_store
            .entry(&current_hash)
            .ok_or(NetworkError::MissingHeaderAncestor(current_hash))?;
        if current.height % interval == 0 || current.header.bits != consensus_params.pow_limit_bits
        {
            return Ok(Some(MinDifficultyRecoveryTarget {
                bits: current.header.bits,
            }));
        }
        current_hash = current.header.previous_block_hash;
    }
}

#[cfg(test)]
mod tests;
