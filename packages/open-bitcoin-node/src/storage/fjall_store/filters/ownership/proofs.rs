// Parity breadcrumbs:
// - packages/bitcoin-knots/src/index/base.cpp
// - packages/bitcoin-knots/src/index/blockfilterindex.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp

//! Guarded proof acquisition; reorg checks retain a separate suspended transition path.

use super::*;

impl FjallNodeStore {
    /// Prepare bounded work only after exclusive configured recovery installed authority.
    pub(crate) fn maybe_basic_filter_append_proof(
        &self,
    ) -> Result<Option<BasicFilterAppendProof>, StorageError> {
        self.maybe_basic_filter_append_proof_with_limits(None)
    }

    /// Scheduler must reserve metadata and all endpoint work before decoding.
    pub(crate) fn maybe_basic_filter_append_proof_with_budget(
        &self,
        maximum_work: TurnWork,
    ) -> Result<Option<BasicFilterAppendProof>, StorageError> {
        self.maybe_basic_filter_append_proof_with_limits(Some(maximum_work))
    }

    pub(super) fn maybe_basic_filter_append_proof_with_limits(
        &self,
        maybe_maximum_work: Option<TurnWork>,
    ) -> Result<Option<BasicFilterAppendProof>, StorageError> {
        let control = self.filter_publication_guard()?;
        if control.maybe_reorg_suspension.is_some() {
            return Err(index_corruption("BASIC work suspended for reorg preview"));
        }
        self.maybe_basic_filter_reorg_proof_guarded(&control, maybe_maximum_work)
    }

    pub(crate) fn maybe_basic_filter_reorg_proof_with_budget(
        &self,
        maximum_work: TurnWork,
    ) -> Result<Option<BasicFilterAppendProof>, StorageError> {
        let control = self.filter_publication_guard()?;
        self.maybe_basic_filter_reorg_proof_guarded(&control, Some(maximum_work))
    }

    fn maybe_basic_filter_reorg_proof_guarded(
        &self,
        control: &PublicationControl,
        maybe_maximum_work: Option<TurnWork>,
    ) -> Result<Option<BasicFilterAppendProof>, StorageError> {
        let Some(identity) = control.maybe_append_identity else {
            return Ok(None);
        };
        let mut proof = BasicFilterAppendProof {
            identity,
            publication: Arc::clone(&self.filter_publication),
            preparation_work: TurnWork::default(),
            maybe_maximum_work,
        };
        let mut work = TurnWork::default();
        self.check_basic_filter_append_proof_guarded_counted(&proof, control, &mut work)?;
        proof.preparation_work = work;
        Ok(Some(proof))
    }
}
