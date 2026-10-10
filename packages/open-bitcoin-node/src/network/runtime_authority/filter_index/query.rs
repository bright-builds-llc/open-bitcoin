// Parity breadcrumbs:
// - packages/bitcoin-knots/src/index/base.cpp
// - packages/bitcoin-knots/src/index/blockfilterindex.cpp
// - packages/bitcoin-knots/src/rpc/blockchain.cpp

//! One authority snapshot; no independent reader, history cache or request walker.

pub use crate::storage::fjall_store::filters::query::{
    BASIC_FILTER_QUERY_MAX_HEX_BYTES, BASIC_FILTER_QUERY_MAX_READ_BYTES,
    BASIC_FILTER_QUERY_MAX_RECORD_BYTES, BasicFilterQueryWork, BasicFilterRecordView,
};
use crate::storage::validation_history::ValidationProvenance;
use crate::{ChainstateStore, ManagedNetworkAuthorityError, ManagedNetworkHandle};
use open_bitcoin_core::{
    chainstate::{CoinsView, filter_index::lifecycle::IndexGeneration},
    primitives::BlockHash,
};

/// Validation history is independent of active membership and filter presence.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BasicBlockValidationProvenance {
    NeverConnected,
    ScriptsValid,
    UnknownLegacy,
}

impl From<ValidationProvenance> for BasicBlockValidationProvenance {
    fn from(provenance: ValidationProvenance) -> Self {
        match provenance {
            ValidationProvenance::NeverConnected => Self::NeverConnected,
            ValidationProvenance::ScriptsValid => Self::ScriptsValid,
            ValidationProvenance::UnknownLegacy => Self::UnknownLegacy,
        }
    }
}

/// A query reports owned immutable data or an explicitly captured work frontier.
#[derive(Debug, PartialEq, Eq)]
pub enum BasicFilterQuery {
    Disabled,
    UnknownBlock,
    Missing {
        provenance: BasicBlockValidationProvenance,
        initially_synchronized: bool,
    },
    Found(BasicFilterRecordView),
    Pending(super::readiness::BasicFilterReadBarrier),
}

/// Authority availability and persisted-data failures stay distinct from absence.
#[derive(Debug)]
pub enum BasicFilterQueryError {
    Authority(ManagedNetworkAuthorityError),
    Storage(crate::StorageError),
    Readiness(super::readiness::BasicFilterReadFailure),
}

impl std::fmt::Display for BasicFilterQueryError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Authority(error) => error.fmt(formatter),
            Self::Storage(error) => error.fmt(formatter),
            Self::Readiness(error) => error.fmt(formatter),
        }
    }
}

impl std::error::Error for BasicFilterQueryError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Authority(error) => Some(error),
            Self::Storage(error) => Some(error),
            Self::Readiness(error) => Some(error),
        }
    }
}

/// Opaque branch-local captured work, not a readiness proof.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BasicFilterReadFrontier {
    authority_incarnation: u64,
    generation: IndexGeneration,
    branch_identity: BlockHash,
    accepted_height: u32,
    accepted_hash: BlockHash,
}

impl BasicFilterReadFrontier {
    /// Return the captured accepted height on this branch; this is not a global sequence.
    pub const fn accepted_height(self) -> u32 {
        self.accepted_height
    }
    /// Return the exact captured accepted hash, unaffected by later acceptance.
    pub const fn accepted_hash(self) -> BlockHash {
        self.accepted_hash
    }
    /// Return the index lifecycle generation at capture.
    pub const fn generation(self) -> IndexGeneration {
        self.generation
    }
    /// Return the stable branch identity at capture.
    pub const fn branch_identity(self) -> BlockHash {
        self.branch_identity
    }
    /// Return the authority incarnation at capture, for identity inspection only.
    pub const fn authority_incarnation(self) -> u64 {
        self.authority_incarnation
    }
}

/// Knots-shaped progress: completion is a latch; height is processed, not durable.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BasicIndexSummary {
    pub synced: bool,
    pub best_block_height: u32,
}

#[derive(Debug, Clone, Copy)]
pub(super) struct ReadRequest {
    pub(super) hash: BlockHash,
    pub(super) height: u32,
    pub(super) parent: BlockHash,
    pub(super) provenance: BasicBlockValidationProvenance,
    pub(super) initially_synchronized: bool,
}

enum QueryStage {
    Immediate(BasicFilterQuery),
    Pending(BasicFilterReadFrontier, ReadRequest),
}

impl<S: ChainstateStore, V: CoinsView> ManagedNetworkHandle<S, V> {
    /// Inspect BASIC availability while holding the existing network authority once.
    pub fn basic_filter_query(
        &self,
        hash: BlockHash,
    ) -> Result<BasicFilterQuery, BasicFilterQueryError> {
        self.read(|network| query_network(network, hash, &self.basic_filter_readiness))
            .map_err(BasicFilterQueryError::Authority)?
    }

    /// Measure the actual shared query path; the hold excludes mutex acquisition.
    #[cfg(test)]
    pub(crate) fn measure_basic_filter_query_for_test(
        &self,
        hash: BlockHash,
    ) -> Result<(BasicFilterQuery, std::time::Duration, std::time::Duration), BasicFilterQueryError>
    {
        let elapsed = std::time::Instant::now();
        let (query, hold) = self
            .read(|network| {
                let start = std::time::Instant::now();
                query_network(network, hash, &self.basic_filter_readiness)
                    .map(|query| (query, start.elapsed()))
            })
            .map_err(BasicFilterQueryError::Authority)??;
        Ok((query, hold, elapsed.elapsed()))
    }

    /// Return no summary when the configured backend/lifecycle is disabled.
    pub fn maybe_basic_index_summary(
        &self,
    ) -> Result<Option<BasicIndexSummary>, BasicFilterQueryError> {
        self.read(|network| {
            let chainstate = network.chainstate();
            let Some(store) = chainstate.store().maybe_basic_filter_query_store() else {
                return Ok(None);
            };
            store.with_basic_filter_query(|reader| {
                if !reader.enabled {
                    return Ok(None);
                }
                let maybe_progress = chainstate.maybe_basic_index_progress();
                Ok(Some(BasicIndexSummary {
                    synced: maybe_progress
                        .is_some_and(|progress| progress.initially_synchronized()),
                    best_block_height: maybe_progress
                        .and_then(|progress| progress.maybe_processed_endpoint())
                        .map_or(0, |record| record.height()),
                }))
            })
        })
        .map_err(BasicFilterQueryError::Authority)?
        .map_err(BasicFilterQueryError::Storage)
    }
}

pub(super) fn query_network<S: ChainstateStore, V: CoinsView>(
    network: &crate::ManagedPeerNetwork<S, V>,
    hash: BlockHash,
    readiness: &super::readiness::ReadinessOwner,
) -> Result<BasicFilterQuery, BasicFilterQueryError> {
    let chainstate = network.chainstate();
    let Some(store) = chainstate.store().maybe_basic_filter_query_store() else {
        return Ok(BasicFilterQuery::Disabled);
    };
    let stage = store
        .with_basic_filter_query(|reader| {
            if !reader.enabled {
                return Ok(QueryStage::Immediate(BasicFilterQuery::Disabled));
            }
            let maybe_identity = chainstate
                .maybe_pending_validation_identity(hash)
                .map(|identity| (identity.height(), identity.parent_hash()))
                .or_else(|| {
                    network
                        .peer_manager()
                        .header_store()
                        .entry(&hash)
                        .map(|entry| (entry.height, entry.header.previous_block_hash))
                });
            let maybe_identity = match maybe_identity {
                Some(identity) => Some(identity),
                None => store
                    .maybe_validation_history_record(hash)?
                    .map(|record| (record.identity().height(), record.identity().parent_hash())),
            };
            let Some((height, parent)) = maybe_identity else {
                return Ok(QueryStage::Immediate(BasicFilterQuery::UnknownBlock));
            };
            let maybe_progress = chainstate.maybe_basic_index_progress();
            let initially_synchronized =
                maybe_progress.is_some_and(|progress| progress.initially_synchronized());
            if initially_synchronized
                && let Some(progress) = maybe_progress
                && let Some(target) = chainstate.maybe_basic_index_accepted_target()
                && progress.maybe_processed_endpoint().is_none_or(|record| {
                    record.height() < target.height() || record.block_hash() != target.block_hash()
                })
            {
                if !reader.matches_generation(progress.generation()) {
                    return Err(crate::storage::filter_index::index_corruption(
                        "BASIC captured read authority invalidated",
                    ));
                }
                let provenance = chainstate.validation_provenance(hash)?.into();
                return Ok(QueryStage::Pending(
                    BasicFilterReadFrontier {
                        authority_incarnation: network.authority_epoch.raw(),
                        generation: progress.generation(),
                        branch_identity: progress.branch_identity(),
                        accepted_height: target.height(),
                        accepted_hash: target.block_hash(),
                    },
                    ReadRequest {
                        hash,
                        height,
                        parent,
                        provenance,
                        initially_synchronized,
                    },
                ));
            }
            // Stored data wins through initial catch-up, stale branches and pruning.
            if let Some(record) = reader.maybe_record(hash)? {
                if record.identity().height() != height || record.identity().parent_hash() != parent
                {
                    return Err(crate::storage::filter_index::index_corruption(
                        "BASIC query differs from known block identity",
                    ));
                }
                return Ok(QueryStage::Immediate(BasicFilterQuery::Found(record)));
            }
            let provenance = chainstate.validation_provenance(hash)?;
            Ok(QueryStage::Immediate(BasicFilterQuery::Missing {
                provenance: provenance.into(),
                initially_synchronized,
            }))
        })
        .map_err(BasicFilterQueryError::Storage)?;
    match stage {
        QueryStage::Immediate(query) => Ok(query),
        QueryStage::Pending(frontier, request) => readiness
            .register(
                frontier,
                request,
                super::readiness::maybe_frontier_result(network, frontier),
            )
            .map(BasicFilterQuery::Pending)
            .map_err(BasicFilterQueryError::Readiness),
    }
}
