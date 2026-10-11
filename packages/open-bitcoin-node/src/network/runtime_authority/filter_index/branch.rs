// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/chain.h
// - packages/bitcoin-knots/src/validation.cpp

//! One shared authority snapshot; peer fields never mint validation provenance.

use super::BasicFilterQueryError;
use crate::network::lifecycle_effects::PeerSessionGeneration;
use crate::storage::validation_history::{MAX_VALIDATION_IDENTITIES, ValidationProvenance};
use crate::{ChainstateStore, ManagedNetworkHandle, ManagedPeerNetwork};
use open_bitcoin_core::{
    chainstate::{CoinsView, filter_index::lifecycle::IndexGeneration},
    consensus::ConsensusParams,
};
use open_bitcoin_network::{
    PeerId, ServiceFlags,
    policy::{
        self, AuthorizedFilterBranch, BranchResourceRefusal, BranchWorkLimit, FilterRequest,
        FilterRequestInvalid, StopProvenance,
    },
};

/// Local ancestry/storage metadata ceilings, independent of pinned semantic limits.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BasicFilterBranchLimit {
    pub branch: BranchWorkLimit,
    pub metadata_units: u64,
}

/// Captured identity is read authority only; it never authorizes store mutation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BasicFilterBranchCapture {
    branch: AuthorizedFilterBranch,
    metadata_units: u64,
    authority_incarnation: u64,
    peer_id: PeerId,
    peer_session_generation: PeerSessionGeneration,
    own_services: ServiceFlags,
    maybe_index_generation: Option<IndexGeneration>,
    maybe_index_branch_identity: Option<open_bitcoin_core::primitives::BlockHash>,
}

impl BasicFilterBranchCapture {
    /// Immutable requested-branch identities, captured with the private authority facts.
    pub fn branch(&self) -> &AuthorizedFilterBranch {
        &self.branch
    }
    /// Conservative finite semantic/lifecycle metadata charge for later reservation.
    pub const fn metadata_units(&self) -> u64 {
        self.metadata_units
    }
    /// Identity for later stale-completion rejection, captured under the same lock.
    pub const fn authority_incarnation(&self) -> u64 {
        self.authority_incarnation
    }
    /// Session identity for later complete preparation.
    pub const fn peer_id(&self) -> PeerId {
        self.peer_id
    }
    /// Session incarnation, never inferred from the remote version.
    pub const fn peer_session_generation(&self) -> PeerSessionGeneration {
        self.peer_session_generation
    }
    /// The local version's capability snapshot, unaffected by later configuration.
    pub const fn own_services(&self) -> ServiceFlags {
        self.own_services
    }
    /// In-memory lifecycle identity; backend absence is deliberately checked later.
    pub const fn maybe_index_generation(&self) -> Option<IndexGeneration> {
        self.maybe_index_generation
    }
    /// Captured lifecycle branch, distinct from the requested stop branch.
    pub const fn maybe_index_branch_identity(
        &self,
    ) -> Option<open_bitcoin_core::primitives::BlockHash> {
        self.maybe_index_branch_identity
    }
}

/// Protocol rejection and bounded local refusal have separate transport dispositions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BasicFilterBranchOutcome {
    Invalid(FilterRequestInvalid),
    ResourceRefused(BranchResourceRefusal),
    Authorized(Box<BasicFilterBranchCapture>),
}

impl<S: ChainstateStore, V: CoinsView> ManagedNetworkHandle<S, V> {
    /// Capture peer capability, active identity, genuine provenance and exact ancestry once.
    pub fn capture_basic_filter_branch(
        &self,
        peer_id: PeerId,
        request: FilterRequest,
        params: ConsensusParams,
        limit: BasicFilterBranchLimit,
    ) -> Result<BasicFilterBranchOutcome, BasicFilterQueryError> {
        self.read(|network| capture_network(network, peer_id, request, params, limit))
            .map_err(BasicFilterQueryError::Authority)?
    }
}

/// Test observations exclude mutex acquisition from hold time, without latency promises.
#[cfg(test)]
pub(crate) struct BasicPeerReadMeasurement {
    pub result: Result<
        Option<Vec<crate::BasicFilterRecordView>>,
        crate::storage::fjall_store::filters::query::peer::BasicPeerReadError,
    >,
    pub work: open_bitcoin_network::basic_peer_resource::BasicPeerServeWork,
    pub hold: std::time::Duration,
    pub total: std::time::Duration,
}

#[cfg(test)]
impl<S: ChainstateStore, V: CoinsView> ManagedNetworkHandle<S, V> {
    pub(crate) fn measure_basic_peer_reads_for_test(
        &self,
        request: FilterRequest,
        budget: &mut open_bitcoin_network::basic_peer_resource::BasicPeerServeBudget,
    ) -> Result<BasicPeerReadMeasurement, crate::ManagedNetworkAuthorityError> {
        use crate::storage::fjall_store::filters::query::peer::BasicPeerReadError;
        use open_bitcoin_network::basic_peer_resource::{
            BasicPeerResourceRefusal, BasicPeerServeAxis as Axis, cfcheckpt_frame_bytes,
            cfheaders_frame_bytes,
        };
        let total = std::time::Instant::now();
        let (result, work, hold) = self.read(|network| {
            let hold = std::time::Instant::now();
            let result = (|| {
                let capture = capture_network(
                    network,
                    1,
                    request,
                    ConsensusParams::default(),
                    BasicFilterBranchLimit {
                        branch: BranchWorkLimit {
                            ancestry_steps: budget.remaining(Axis::AncestryLinks),
                            identity_bytes: budget.remaining(Axis::LogicalCopyBytes),
                        },
                        metadata_units: budget.remaining(Axis::MetadataReads),
                    },
                )
                .map_err(|error| match error {
                    BasicFilterQueryError::Storage(error) => BasicPeerReadError::Storage(error),
                    _ => panic!("fixture authority must be available"),
                })?;
                let capture = match capture {
                    BasicFilterBranchOutcome::Authorized(capture) => capture,
                    BasicFilterBranchOutcome::ResourceRefused(refusal) => {
                        return Err(BasicPeerResourceRefusal(match refusal {
                            BranchResourceRefusal::MetadataWork => Axis::MetadataReads,
                            BranchResourceRefusal::IdentityBytes => Axis::LogicalCopyBytes,
                            _ => Axis::AncestryLinks,
                        })
                        .into());
                    }
                    BasicFilterBranchOutcome::Invalid(error) => {
                        panic!("invalid fixture: {error:?}")
                    }
                };
                let branch = capture.branch();
                budget.charge(Axis::MetadataReads, capture.metadata_units())?;
                budget.charge(Axis::AncestryLinks, branch.ancestry_steps)?;
                budget.charge(Axis::HeaderOperations, branch.ancestry_steps)?;
                budget.charge(Axis::LogicalCopyBytes, branch.identity_bytes)?;
                let maybe_frame = match request {
                    FilterRequest::Filters { .. } => None,
                    FilterRequest::Headers { .. } => {
                        cfheaders_frame_bytes(branch.identities.len() as u64)
                    }
                    FilterRequest::Checkpoints { .. } => cfcheckpt_frame_bytes(branch.stop.height),
                };
                if let Some(frame) = maybe_frame {
                    budget.charge(Axis::EncodedSuffixBytes, frame)?;
                }
                let Some(store) = network
                    .chainstate()
                    .store()
                    .maybe_basic_filter_query_store()
                else {
                    return Err(BasicPeerReadError::Disabled);
                };
                store.with_basic_peer_query(budget, |reader, meter| {
                    if capture
                        .maybe_index_generation()
                        .is_some_and(|generation| !reader.matches_generation(generation))
                    {
                        return Err(crate::storage::filter_index::index_corruption(
                            "BASIC measured epoch invalidated",
                        )
                        .into());
                    }
                    if let Some(previous) = branch.maybe_predecessor {
                        let Some(record) = reader.maybe_peer_record(previous.hash, meter, false)?
                        else {
                            return Ok(None);
                        };
                        if record.identity().height() != previous.height
                            || record.identity().parent_hash() != previous.parent
                        {
                            return Err(crate::storage::filter_index::index_corruption(
                                "BASIC measured predecessor differs",
                            )
                            .into());
                        }
                    }
                    reader.maybe_peer_records(
                        &branch.identities,
                        meter,
                        matches!(request, FilterRequest::Filters { .. }),
                    )
                })
            })();
            (result, budget.work(), hold.elapsed())
        })?;
        Ok(BasicPeerReadMeasurement {
            result,
            work,
            hold,
            total: total.elapsed(),
        })
    }
}

pub(in crate::network) fn capture_network<S: ChainstateStore, V: CoinsView>(
    network: &ManagedPeerNetwork<S, V>,
    peer_id: PeerId,
    request: FilterRequest,
    params: ConsensusParams,
    limit: BasicFilterBranchLimit,
) -> Result<BasicFilterBranchOutcome, BasicFilterQueryError> {
    let (validated, metadata_units) = match validate_network(network, peer_id, request, params)? {
        Ok(facts) => facts,
        Err(reason) => return Ok(BasicFilterBranchOutcome::Invalid(reason)),
    };
    Ok(resolve_network(
        network,
        peer_id,
        validated,
        metadata_units,
        limit,
    ))
}

pub(super) fn validate_network<S: ChainstateStore, V: CoinsView>(
    network: &ManagedPeerNetwork<S, V>,
    peer_id: PeerId,
    request: FilterRequest,
    params: ConsensusParams,
) -> Result<
    Result<(policy::ValidatedFilterRequest, u64), FilterRequestInvalid>,
    BasicFilterQueryError,
> {
    let own_services = network
        .peer_manager()
        .peer_state(peer_id)
        .map_or(ServiceFlags::NONE, |peer| peer.own_advertised_services);
    // These first two gates perform no provenance/backend read, even when malformed
    // requests arrive during local saturation or persisted-store failure.
    if request.filter_type() != 0 || !own_services.contains(ServiceFlags::COMPACT_FILTERS) {
        return Ok(Err(FilterRequestInvalid::UnsupportedTypeOrCapability));
    }
    let headers = network.peer_manager().header_store();
    let Some(stop) = headers.entry(&request.stop_hash()) else {
        return Ok(Err(FilterRequestInvalid::UnknownOrDisallowedStop));
    };
    let chainstate = network.chainstate();
    let active = chainstate.chainstate().active_chain();
    // Checked offset supports a retained active window without scanning it.
    let is_active = active
        .first()
        .and_then(|first| stop.height.checked_sub(first.height))
        .and_then(|offset| usize::try_from(offset).ok())
        .and_then(|offset| active.get(offset))
        .is_some_and(|position| {
            position.height == stop.height && position.block_hash == stop.block_hash
        });
    let (provenance, metadata_units) = if is_active {
        (StopProvenance::Active, 6)
    } else {
        let provenance = chainstate
            .validation_provenance(stop.block_hash)
            .map_err(BasicFilterQueryError::Storage)?;
        // Conservatively charge the sealed pending batch's finite scan ceiling,
        // coverage/point read, two proof lookups and ordinary snapshot metadata.
        let units = MAX_VALIDATION_IDENTITIES as u64 + 10;
        (
            if provenance == ValidationProvenance::ScriptsValid {
                StopProvenance::ScriptsValid
            } else {
                StopProvenance::Insufficient
            },
            units,
        )
    };
    let validated = match policy::validate_filter_request(
        request,
        own_services,
        headers,
        provenance,
        params.pow_target_spacing_seconds,
    ) {
        Ok(validated) => validated,
        Err(reason) => return Ok(Err(reason)),
    };
    Ok(Ok((validated, metadata_units)))
}

pub(super) fn resolve_network<S: ChainstateStore, V: CoinsView>(
    network: &ManagedPeerNetwork<S, V>,
    peer_id: PeerId,
    validated: policy::ValidatedFilterRequest,
    metadata_units: u64,
    limit: BasicFilterBranchLimit,
) -> BasicFilterBranchOutcome {
    // Fixed, bounded semantic reads precede local refusal to preserve disconnect
    // precedence. Their conservative cost remains included in admission accounting.
    if metadata_units > limit.metadata_units {
        return BasicFilterBranchOutcome::ResourceRefused(BranchResourceRefusal::MetadataWork);
    }
    let branch = match policy::resolve_filter_branch(
        validated,
        network.peer_manager().header_store(),
        limit.branch,
    ) {
        Ok(branch) => branch,
        Err(reason) => return BasicFilterBranchOutcome::ResourceRefused(reason),
    };
    let maybe_progress = network.chainstate().maybe_basic_index_progress();
    BasicFilterBranchOutcome::Authorized(Box::new(BasicFilterBranchCapture {
        branch,
        metadata_units,
        authority_incarnation: network.authority_epoch.raw(),
        peer_id,
        peer_session_generation: network.peer_session_generation(peer_id),
        own_services: network
            .peer_manager()
            .peer_state(peer_id)
            .map_or(ServiceFlags::NONE, |peer| peer.own_advertised_services),
        maybe_index_generation: maybe_progress.map(|progress| progress.generation()),
        maybe_index_branch_identity: maybe_progress.map(|progress| progress.branch_identity()),
    }))
}

#[cfg(test)]
mod tests;
