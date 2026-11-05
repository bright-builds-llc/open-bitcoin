// Parity breadcrumbs:
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/chain.cpp

use std::sync::Arc;

use super::candidate_rules::PendingAdmission;
use open_bitcoin_consensus::chain_proof::{ProofWork, block_proof};
use open_bitcoin_consensus::chain_spec::NetworkRules;
use open_bitcoin_consensus::{ConsensusParams, block_hash};
use open_bitcoin_primitives::BlockHash;

use super::Chainstate;
use crate::coins::{CoinsCache, CoinsView, MemoryCoinsView};
use crate::{BlockUndo, ChainPosition, ChainstateError, ChainstateSnapshot};
use open_bitcoin_primitives::Txid;
use std::collections::HashMap;

/// Numeric derivation only; the owner must separately prove the parent lineage.
pub(super) fn derive_child_work(
    parent_work: ProofWork,
    bits: u32,
) -> Result<ProofWork, ChainstateError> {
    let proof = block_proof(bits).map_err(|error| ChainstateError::Serialization {
        context: "block proof",
        reason: error.to_string(),
    })?;
    Ok(parent_work.wrapping_add(proof))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum WorkOrigin {
    Live,
    Untrusted,
    MechanicalFixture,
}

#[derive(Debug)]
pub(super) struct WorkAuthority {
    owner: Arc<()>,
    generation: Arc<()>,
    origin: WorkOrigin,
    maybe_params: Option<ConsensusParams>,
    maybe_preview: Option<Arc<()>>,
    pub(super) maybe_canonical_rules: Option<NetworkRules>,
    pub(super) maybe_admission: Option<PendingAdmission>,
}

#[derive(Debug)]
pub(super) struct StageBinding {
    owner: Arc<()>,
    generation: Arc<()>,
    maybe_tip: Option<ChainPosition>,
    seal: Arc<()>,
    params: ConsensusParams,
    origin: WorkOrigin,
}

impl WorkAuthority {
    pub(super) fn maybe_bound_params(&self) -> Option<ConsensusParams> {
        self.maybe_params
    }
    pub(super) fn is_mechanical_fixture(&self) -> bool {
        self.origin == WorkOrigin::MechanicalFixture
    }
    pub(super) fn new(origin: WorkOrigin) -> Self {
        Self {
            owner: Arc::new(()),
            generation: Arc::new(()),
            origin,
            maybe_params: None,
            maybe_preview: None,
            maybe_canonical_rules: None,
            maybe_admission: None,
        }
    }

    pub(super) fn invalidate_stages(&mut self) {
        self.generation = Arc::new(());
        self.maybe_admission = None;
        if self.maybe_preview.take().is_some() {
            self.origin = WorkOrigin::Untrusted;
            self.maybe_canonical_rules = None;
        }
    }

    pub(super) fn invalidate_coins_authority(&mut self) {
        self.invalidate_stages();
        self.origin = WorkOrigin::Untrusted;
        self.maybe_canonical_rules = None;
    }

    pub(super) fn bind(
        &self,
        maybe_tip: Option<&ChainPosition>,
        params: ConsensusParams,
    ) -> StageBinding {
        StageBinding {
            owner: self.owner.clone(),
            generation: self.generation.clone(),
            maybe_tip: maybe_tip.cloned(),
            seal: Arc::new(()),
            params,
            origin: self.origin,
        }
    }

    pub(super) fn check(
        &self,
        binding: &StageBinding,
        maybe_tip: Option<&ChainPosition>,
        allow_preview: bool,
    ) -> Result<(), ChainstateError> {
        if !Arc::ptr_eq(&self.owner, &binding.owner) {
            return Err(ChainstateError::StageOwnerMismatch);
        }
        if allow_preview
            && self
                .maybe_preview
                .as_ref()
                .is_some_and(|seal| Arc::ptr_eq(seal, &binding.seal))
        {
            return Ok(());
        }
        if !Arc::ptr_eq(&self.generation, &binding.generation)
            || maybe_tip != binding.maybe_tip.as_ref()
        {
            return Err(ChainstateError::StaleStage);
        }
        Ok(())
    }

    pub(super) fn accept(&mut self, binding: &StageBinding) {
        let maybe_canonical_rules = self.maybe_canonical_rules;
        self.maybe_params = Some(binding.params);
        self.invalidate_stages();
        self.origin = binding.origin;
        self.maybe_canonical_rules = maybe_canonical_rules;
    }

    pub(super) fn preview(&mut self, binding: &StageBinding) {
        self.invalidate_stages();
        self.maybe_preview = Some(binding.seal.clone());
    }

    pub(super) fn check_parent(
        &self,
        active_chain: &[ChainPosition],
        params: ConsensusParams,
    ) -> Result<bool, ChainstateError> {
        if self.maybe_preview.is_some() {
            return Err(ChainstateError::StaleStage);
        }
        if self.origin == WorkOrigin::Untrusted {
            return Err(ChainstateError::UntrustedWorkLineage);
        }
        if self.maybe_params.is_some_and(|selected| selected != params) {
            return Err(ChainstateError::WorkParametersMismatch);
        }
        if self.origin == WorkOrigin::MechanicalFixture {
            return Ok(false);
        }
        for (index, position) in active_chain.iter().enumerate() {
            let previous = index
                .checked_sub(1)
                .map_or(BlockHash::default(), |i| active_chain[i].block_hash);
            if usize::try_from(position.height).ok() != Some(index)
                || position.block_hash != block_hash(&position.header)
                || position.previous_block_hash() != previous
            {
                return Err(ChainstateError::UntrustedWorkLineage);
            }
        }
        Ok(true)
    }
}

impl StageBinding {
    pub(super) fn matches_source(&self, source: &crate::LiveWorkSourceBinding) -> bool {
        Arc::ptr_eq(&self.owner, &source.owner)
            && Arc::ptr_eq(&self.generation, &source.generation)
            && self.maybe_tip == source.maybe_tip
            && self.params == source.params
    }

    pub(super) fn has_earned_work(&self) -> bool {
        self.origin == WorkOrigin::Live
    }
}

impl Clone for Chainstate<MemoryCoinsView> {
    fn clone(&self) -> Self {
        let mut independent = Self::from_snapshot(self.snapshot());
        let origin = if self.work_authority.maybe_preview.is_some() {
            WorkOrigin::Untrusted
        } else {
            self.work_authority.origin
        };
        independent.work_authority = WorkAuthority::new(origin);
        independent.work_authority.maybe_params = self.work_authority.maybe_params;
        if origin == WorkOrigin::Live {
            independent.work_authority.maybe_canonical_rules =
                self.work_authority.maybe_canonical_rules;
        }
        independent
    }
}

impl Chainstate<MemoryCoinsView> {
    /// Empty mechanical root for legacy score/transaction fixtures, never earned authority.
    pub fn new_for_mechanical_fixture() -> Self {
        let mut chain = Self::new();
        chain.work_authority = WorkAuthority::new(WorkOrigin::MechanicalFixture);
        chain
    }

    /// Explicit transaction-engine fixture import. This never earns live work authority.
    pub fn from_snapshot_for_mechanical_fixture(snapshot: ChainstateSnapshot) -> Self {
        let mut chain = Self::from_snapshot(snapshot);
        chain.work_authority = WorkAuthority::new(WorkOrigin::MechanicalFixture);
        chain
    }
}

impl<V: CoinsView> Chainstate<V> {
    /// Capture the actual private live source without preparing any block or effect.
    pub fn bind_live_work_source(
        &self,
        params: ConsensusParams,
    ) -> Result<crate::LiveWorkSourceBinding, ChainstateError> {
        self.validate_live_origin()?;
        self.work_authority
            .check_parent(&self.active_chain, params)?;
        Ok(crate::LiveWorkSourceBinding {
            owner: self.work_authority.owner.clone(),
            generation: self.work_authority.generation.clone(),
            maybe_tip: self.tip().cloned(),
            params,
        })
    }

    /// Recheck source identity before preparation; imported equal-looking state refuses.
    pub fn check_live_work_source(
        &self,
        source: &crate::LiveWorkSourceBinding,
    ) -> Result<(), ChainstateError> {
        self.validate_live_origin()?;
        self.work_authority
            .check_parent(&self.active_chain, source.params)?;
        if !Arc::ptr_eq(&self.work_authority.owner, &source.owner) {
            return Err(ChainstateError::StageOwnerMismatch);
        }
        if !Arc::ptr_eq(&self.work_authority.generation, &source.generation)
            || self.tip() != source.maybe_tip.as_ref()
        {
            return Err(ChainstateError::StaleStage);
        }
        Ok(())
    }

    /// Consume genuine live authority when replacing an exactly matching coins view.
    pub fn rebind_into<W: CoinsView>(
        self,
        target: &mut Chainstate<W>,
    ) -> Result<(), ChainstateError> {
        self.validate_live_origin()?;
        if target.work_authority.maybe_preview.is_some() {
            return Err(ChainstateError::StaleStage);
        }
        let source = self.coins.complete_snapshot()?;
        let destination = target.coins.complete_snapshot()?;
        let pristine_empty = source.coins.is_empty()
            && source.maybe_best_block.is_none()
            && self.active_chain.is_empty()
            && self.undo_by_block.is_empty()
            && target.active_chain.is_empty()
            && target.undo_by_block.is_empty()
            && self
                .maybe_confirmed_txid_counts
                .as_ref()
                .is_none_or(HashMap::is_empty)
            && target
                .maybe_confirmed_txid_counts
                .as_ref()
                .is_none_or(HashMap::is_empty);
        if source != destination
            || !source.head_blocks.is_empty()
            || source.maybe_best_block != self.tip().map(|tip| tip.block_hash)
            || self.active_chain != target.active_chain
            || self.undo_by_block != target.undo_by_block
            || (!pristine_empty
                && self.maybe_confirmed_txid_counts != target.maybe_confirmed_txid_counts)
        {
            return Err(ChainstateError::WorkRebindMismatch);
        }
        target.active_chain = self.active_chain;
        target.undo_by_block = self.undo_by_block;
        target.maybe_confirmed_txid_counts = self.maybe_confirmed_txid_counts;
        target.work_authority = WorkAuthority::new(WorkOrigin::Live);
        target.work_authority.maybe_params = self.work_authority.maybe_params;
        target.work_authority.maybe_canonical_rules = self.work_authority.maybe_canonical_rules;
        Ok(())
    }

    /// Whether this owner has live validated ancestry rather than imported fixture metadata.
    pub fn has_earned_work(&self) -> bool {
        self.validate_live_origin().is_ok() && !self.active_chain.is_empty()
    }

    /// Inspect the actual private origin before effects. Genuine owned-empty
    /// state may start live validation; raw empty imports and previews cannot.
    pub fn validate_live_origin(&self) -> Result<(), ChainstateError> {
        if self.work_authority.origin != WorkOrigin::Live
            || self.work_authority.maybe_preview.is_some()
        {
            return Err(ChainstateError::UntrustedWorkLineage);
        }
        Ok(())
    }

    /// Explicit mechanical parent boundary; never a canonical or earned-work import.
    pub fn from_parent_for_mechanical_fixture(
        parent: V,
        active_chain: Vec<ChainPosition>,
        undo_by_block: HashMap<BlockHash, BlockUndo>,
        maybe_confirmed_txid_counts: Option<HashMap<Txid, u32>>,
    ) -> Self {
        let mut chain = Self::from_parent(
            parent,
            active_chain,
            undo_by_block,
            maybe_confirmed_txid_counts,
        );
        chain.work_authority = WorkAuthority::new(WorkOrigin::MechanicalFixture);
        chain
    }

    /// Explicit mechanical cache boundary; coins B alone cannot earn work authority.
    pub fn from_coins_cache_for_mechanical_fixture(
        coins: CoinsCache<V>,
        active_chain: Vec<ChainPosition>,
        undo_by_block: HashMap<BlockHash, BlockUndo>,
        maybe_confirmed_txid_counts: Option<HashMap<Txid, u32>>,
    ) -> Self {
        let mut chain = Self::from_coins_cache(
            coins,
            active_chain,
            undo_by_block,
            maybe_confirmed_txid_counts,
        );
        chain.work_authority = WorkAuthority::new(WorkOrigin::MechanicalFixture);
        chain
    }
}

impl<V: CoinsView> crate::coins::CoinsFlush for Chainstate<V> {
    fn maybe_canonical_confirmed_txid_counts(
        &self,
    ) -> Result<Option<&HashMap<Txid, u32>>, ChainstateError> {
        if self.work_authority.maybe_canonical_rules.is_none() {
            return Ok(None);
        }
        self.validate_live_origin()?;
        self.maybe_confirmed_txid_counts
            .as_ref()
            .map(Some)
            .ok_or(ChainstateError::WorkRebindMismatch)
    }
    fn estimated_cache_bytes(&self) -> u64 {
        self.coins.estimated_cache_bytes()
    }
    fn cache_entry_count(&self) -> u64 {
        self.coins.cache_entry_count()
    }
    fn prepare_current_best_block(&mut self) -> Result<(), ChainstateError> {
        if self.work_authority.maybe_preview.is_some() {
            return Err(ChainstateError::StaleStage);
        }
        crate::coins::CoinsFlush::prepare_current_best_block(&mut self.coins)
    }
    fn flush(&mut self) -> Result<(), ChainstateError> {
        if self.work_authority.maybe_preview.is_some() {
            return Err(ChainstateError::StaleStage);
        }
        self.work_authority.invalidate_stages();
        self.coins.flush()
    }
    fn sync(&mut self) -> Result<(), ChainstateError> {
        if self.work_authority.maybe_preview.is_some() {
            return Err(ChainstateError::StaleStage);
        }
        self.work_authority.invalidate_stages();
        self.coins.sync()
    }
}

#[cfg(test)]
mod phase167_canonical_count_observation {
    use super::*;
    use crate::coins::CoinsFlush;
    use open_bitcoin_consensus::{CanonicalChain, chain_spec::CanonicalGenesisProof};

    fn canonical() -> Chainstate {
        let spec = CanonicalChain::Regtest.spec();
        let proof = CanonicalGenesisProof::prove(spec, &spec.genesis_block()).expect("proof");
        Chainstate::from_canonical_genesis(
            &proof,
            CoinsCache::from_parent(MemoryCoinsView::from_coins(
                HashMap::new(),
                Some(proof.hash()),
            )),
        )
        .expect("genuine canonical owner")
    }

    #[test]
    fn generic_live_and_explicit_seeded_owners_remain_unknown() {
        // Arrange
        let generic = Chainstate::new();
        let seeded = Chainstate::from_snapshot_for_mechanical_fixture(canonical().snapshot());
        // Act / Assert
        for state in [generic, seeded] {
            let before = state.snapshot();
            assert_eq!(state.maybe_canonical_confirmed_txid_counts(), Ok(None));
            assert_eq!(state.snapshot(), before);
        }
    }

    #[test]
    fn proved_genesis_has_explicit_empty_map() {
        // Arrange
        let state = canonical();
        let before = state.snapshot();
        // Act
        let observed = state.maybe_canonical_confirmed_txid_counts();
        // Assert
        assert_eq!(observed, Ok(Some(&HashMap::new())));
        assert_eq!(state.snapshot(), before);
    }

    #[test]
    fn admitted_connection_observes_the_exact_existing_nonempty_map() {
        // Arrange
        let mut state = canonical();
        let mut block = CanonicalChain::Regtest.spec().genesis_block();
        block.header.previous_block_hash = state.tip().expect("tip").block_hash;
        block.header.version = 4;
        block.header.time += 1;
        block.transactions[0].inputs[0].script_sig =
            open_bitcoin_primitives::ScriptBuf::from_bytes(vec![0x51, 0]).expect("height one");
        block.transactions[0].outputs[0].script_pubkey =
            open_bitcoin_primitives::ScriptBuf::from_bytes(vec![0x51]).expect("true");
        block.header.merkle_root = open_bitcoin_consensus::block_merkle_root(&block.transactions)
            .expect("merkle")
            .0;
        block.header.nonce = 0;
        while open_bitcoin_consensus::check_block_header(&block.header).is_err() {
            block.header.nonce += 1;
        }
        state
            .admit_canonical_header(&block.header, i64::from(block.header.time))
            .expect("admission");
        let work = derive_child_work(state.tip().expect("parent").chain_work, block.header.bits)
            .expect("work");
        state
            .connect_block_with_current_time(
                &block,
                work,
                i64::MIN,
                open_bitcoin_consensus::ScriptVerifyFlags::NONE,
                ConsensusParams::default(),
            )
            .expect("actual admitted connect");
        let before = state.snapshot();
        let expected = HashMap::from([(
            open_bitcoin_consensus::transaction_txid(&block.transactions[0]).expect("txid"),
            1,
        )]);
        // Act
        let observed = state
            .maybe_canonical_confirmed_txid_counts()
            .expect("known")
            .expect("canonical");
        // Assert
        assert_eq!(observed, &expected);
        assert!(std::ptr::eq(
            observed,
            state
                .maybe_confirmed_txid_counts
                .as_ref()
                .expect("same owner")
        ));
        assert_eq!(state.snapshot(), before);
    }

    #[test]
    fn canonical_marker_with_invalid_origin_refuses_without_mutation() {
        // Arrange
        let mut state = canonical();
        state.work_authority.origin = WorkOrigin::Untrusted;
        let before = state.snapshot();
        // Act
        let observed = state.maybe_canonical_confirmed_txid_counts();
        // Assert
        assert_eq!(observed, Err(ChainstateError::UntrustedWorkLineage));
        assert_eq!(state.snapshot(), before);
    }

    #[test]
    fn canonical_marker_with_preview_refuses_without_mutation() {
        // Arrange
        let mut state = canonical();
        state.work_authority.maybe_preview = Some(Arc::new(()));
        let before = state.snapshot();
        // Act
        let observed = state.maybe_canonical_confirmed_txid_counts();
        // Assert
        assert_eq!(observed, Err(ChainstateError::UntrustedWorkLineage));
        assert_eq!(state.snapshot(), before);
    }

    #[test]
    fn canonical_marker_with_missing_complete_map_refuses_without_mutation() {
        // Arrange
        let mut state = canonical();
        state.maybe_confirmed_txid_counts = None;
        let before = state.snapshot();
        // Act
        let observed = state.maybe_canonical_confirmed_txid_counts();
        // Assert
        assert_eq!(observed, Err(ChainstateError::WorkRebindMismatch));
        assert_eq!(state.snapshot(), before);
    }
}
