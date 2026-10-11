// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/index/blockfilterindex.cpp

//! Complete immediate persisted preparation, with no transport or indexing effects.

use super::branch::{
    self, BasicFilterBranchCapture, BasicFilterBranchLimit, BasicFilterBranchOutcome,
};
use crate::network::filter_transport::{
    BasicFilterPrepareOutcome, BasicFilterServeUnavailable, FilterRequestLease,
    FilterTransportError,
};
use crate::storage::fjall_store::filters::query::{BasicFilterReadEpoch, peer::BasicPeerReadError};
use crate::{
    BasicFilterRecordView, ChainstateStore, ManagedNetworkAuthorityError, ManagedNetworkHandle,
    ManagedPeerNetwork,
};
use open_bitcoin_core::{
    chainstate::CoinsView,
    consensus::ConsensusParams,
    primitives::{Hash32, NetworkMagic},
};
use open_bitcoin_network::{
    CFCheckpt, CFHeaders, CFilter, PeerId, WireNetworkMessage,
    basic_peer_resource::{
        BASIC_PEER_MAX_SUFFIX_BYTES, BasicPeerResourceRefusal, BasicPeerServeAxis as Axis,
        BasicPeerServeBudget, cfcheckpt_frame_bytes, cfheaders_frame_bytes, cfilter_frame_bytes,
    },
    policy::{BranchWorkLimit, FilterRequest},
};

struct CompleteRead {
    lease: FilterRequestLease,
    capture: Box<BasicFilterBranchCapture>,
    epoch: BasicFilterReadEpoch,
    frames: Vec<Vec<u8>>,
}

impl<S: ChainstateStore, V: CoinsView> ManagedNetworkHandle<S, V> {
    pub(in crate::network) fn resolve_basic_filter_intents(
        &self,
        peer: PeerId,
        mut result: crate::network::ManagedSyncMessageResult,
        params: ConsensusParams,
    ) -> Result<crate::network::ManagedSyncMessageResult, ManagedNetworkAuthorityError> {
        use crate::network::ManagedInboundResponsePlanItem as Item;
        for item in &mut result.inbound_response_plan {
            let Item::FilterRequestIntent(request) = item else {
                continue;
            };
            *item = match self.prepare_basic_filter_response(peer, *request, params)? {
                BasicFilterPrepareOutcome::Prepared(response) => {
                    Item::PreparedFilterServe(response)
                }
                BasicFilterPrepareOutcome::Unavailable(reason) => {
                    Item::FilterServeUnavailable(reason)
                }
                BasicFilterPrepareOutcome::InvalidDisconnect(_reason) => {
                    self.disconnect_peer(peer)?;
                    return Err(ManagedNetworkAuthorityError::Operation(
                        crate::ManagedNetworkError::LifecycleEffect("invalid BASIC peer request"),
                    ));
                }
            };
        }
        Ok(result)
    }

    /// Prepare one complete immutable family immediately, without RPC readiness waits.
    pub fn prepare_basic_filter_response(
        &self,
        peer: PeerId,
        request: FilterRequest,
        params: ConsensusParams,
    ) -> Result<BasicFilterPrepareOutcome, ManagedNetworkAuthorityError> {
        let mut budget = BasicPeerServeBudget::default();
        self.prepare_basic_filter_response_with_budget(peer, request, params, &mut budget, |_| {})
    }

    fn prepare_basic_filter_response_with_budget(
        &self,
        peer: PeerId,
        request: FilterRequest,
        params: ConsensusParams,
        budget: &mut BasicPeerServeBudget,
        before_handoff: impl FnOnce(bool),
    ) -> Result<BasicFilterPrepareOutcome, ManagedNetworkAuthorityError> {
        self.read(|network| {
            let completed = self.prepare_filter_locked(network, peer, request, params, budget);
            let CompleteRead {
                lease,
                capture,
                epoch,
                frames,
            } = match completed {
                Ok(completed) => completed,
                Err(outcome) => return outcome,
            };
            #[cfg(test)]
            before_handoff(self.authority.try_lock().is_err());
            #[cfg(not(test))]
            before_handoff(true);
            // Publication is released; authority still serializes revocation and
            // admission until locally owned buffers transfer into the lease registry.
            if let Err(error) = lease.resize_suffix(budget.work().get(Axis::EncodedSuffixBytes)) {
                drop(frames);
                drop(lease);
                return unavailable(transport_failure(error));
            }
            if let Err(error) =
                self.bind_basic_filter_response_locked(network, &lease, &capture, epoch)
            {
                drop(frames);
                drop(lease);
                return unavailable(transport_failure(error));
            }
            match lease.prepare(frames) {
                Ok(response) => BasicFilterPrepareOutcome::Prepared(Box::new(response)),
                Err(error) => unavailable(transport_failure(error)),
            }
        })
    }

    #[cfg(test)]
    pub(crate) fn measure_basic_filter_preparation_for_test(
        &self,
        peer: PeerId,
        request: FilterRequest,
        params: ConsensusParams,
        mut budget: BasicPeerServeBudget,
    ) -> Result<
        (
            BasicFilterPrepareOutcome,
            open_bitcoin_network::basic_peer_resource::BasicPeerServeWork,
        ),
        ManagedNetworkAuthorityError,
    > {
        let outcome = self.prepare_basic_filter_response_with_budget(
            peer,
            request,
            params,
            &mut budget,
            |_| {},
        )?;
        Ok((outcome, budget.work()))
    }

    #[cfg(test)]
    pub(crate) fn inspect_basic_filter_handoff_for_test(
        &self,
        peer: PeerId,
        request: FilterRequest,
        params: ConsensusParams,
        inspect: impl FnOnce(bool),
    ) -> Result<BasicFilterPrepareOutcome, ManagedNetworkAuthorityError> {
        self.prepare_basic_filter_response_with_budget(
            peer,
            request,
            params,
            &mut BasicPeerServeBudget::default(),
            inspect,
        )
    }

    fn prepare_filter_locked(
        &self,
        network: &ManagedPeerNetwork<S, V>,
        peer: PeerId,
        request: FilterRequest,
        params: ConsensusParams,
        budget: &mut BasicPeerServeBudget,
    ) -> Result<CompleteRead, BasicFilterPrepareOutcome> {
        let (validated, metadata) = match branch::validate_network(network, peer, request, params) {
            Ok(Ok(facts)) => facts,
            Ok(Err(reason)) => return Err(BasicFilterPrepareOutcome::InvalidDisconnect(reason)),
            Err(_error) => return Err(unavailable(BasicFilterServeUnavailable::StorageFailure)),
        };
        let lease = self
            .reserve_basic_filter_response_locked(network, peer)
            .map_err(|error| unavailable(transport_failure(error)))?;
        // Reserve a finite worst-case suffix before ancestry/record/output allocations.
        lease
            .resize_suffix(BASIC_PEER_MAX_SUFFIX_BYTES)
            .map_err(|error| unavailable(transport_failure(error)))?;
        budget
            .charge(Axis::MetadataReads, metadata)
            .map_err(work_failure)?;
        let Some(store) = network
            .chainstate()
            .store()
            .maybe_basic_filter_query_store()
        else {
            return Err(unavailable(BasicFilterServeUnavailable::NoBackend));
        };
        let mut invalidated = false;
        let result = store.with_basic_peer_query(budget, |reader, budget| {
            let capture = match branch::resolve_network(
                network,
                peer,
                validated,
                metadata,
                BasicFilterBranchLimit {
                    branch: BranchWorkLimit {
                        ancestry_steps: budget
                            .remaining(Axis::AncestryLinks)
                            .min(budget.remaining(Axis::HeaderOperations)),
                        identity_bytes: budget.remaining(Axis::LogicalCopyBytes),
                    },
                    metadata_units: metadata,
                },
            ) {
                BasicFilterBranchOutcome::Authorized(capture) => capture,
                BasicFilterBranchOutcome::Invalid(_)
                | BasicFilterBranchOutcome::ResourceRefused(_) => {
                    return Err(BasicPeerResourceRefusal(Axis::AncestryLinks).into());
                }
            };
            let branch = capture.branch();
            budget.charge(Axis::AncestryLinks, branch.ancestry_steps)?;
            budget.charge(Axis::HeaderOperations, branch.ancestry_steps)?;
            budget.charge(Axis::LogicalCopyBytes, branch.identity_bytes)?;
            if let Some(frame) = match request {
                FilterRequest::Filters { .. } => None,
                FilterRequest::Headers { .. } => {
                    cfheaders_frame_bytes(branch.identities.len() as u64)
                }
                FilterRequest::Checkpoints { .. } => cfcheckpt_frame_bytes(branch.stop.height),
            } {
                budget.charge(Axis::EncodedSuffixBytes, frame)?;
            }
            if !capture
                .maybe_index_generation()
                .is_some_and(|generation| reader.matches_generation(generation))
            {
                invalidated = true;
                return Ok(None);
            }
            let epoch = reader.capture_basic_peer_epoch(budget)?;
            // Read the complete target set first: a missing predecessor of a present
            // target is corruption, while an absent target is ordinary absence.
            let Some(records) = reader.maybe_peer_records(
                &branch.identities,
                budget,
                matches!(request, FilterRequest::Filters { .. }),
            )?
            else {
                return Ok(None);
            };
            let previous_header = match branch.maybe_predecessor {
                None => Hash32::default(),
                Some(previous) => {
                    let record = reader
                        .maybe_peer_record(previous.hash, budget, false)?
                        .ok_or_else(|| {
                            crate::storage::filter_index::index_corruption(
                                "missing BASIC response predecessor",
                            )
                        })?;
                    if record.identity().block_hash() != previous.hash
                        || record.identity().height() != previous.height
                        || record.identity().parent_hash() != previous.parent
                    {
                        return Err(crate::storage::filter_index::index_corruption(
                            "BASIC response predecessor differs",
                        )
                        .into());
                    }
                    record.filter_header().into()
                }
            };
            let frames = encode_complete(
                request,
                branch.stop.height,
                records,
                previous_header,
                network.local_config.magic,
                budget,
            )?;
            Ok(Some((capture, epoch, frames)))
        });
        match result {
            Ok(Some((capture, epoch, frames))) => Ok(CompleteRead {
                lease,
                capture,
                epoch,
                frames,
            }),
            Ok(None) if invalidated => Err(unavailable(BasicFilterServeUnavailable::Invalidated)),
            Ok(None) => Err(unavailable(BasicFilterServeUnavailable::Missing)),
            Err(error) => Err(unavailable(read_failure(error))),
        }
    }
}

fn unavailable(reason: BasicFilterServeUnavailable) -> BasicFilterPrepareOutcome {
    BasicFilterPrepareOutcome::Unavailable(reason)
}
fn work_failure(_error: BasicPeerResourceRefusal) -> BasicFilterPrepareOutcome {
    unavailable(BasicFilterServeUnavailable::ResourceRefused)
}
fn transport_failure(error: FilterTransportError) -> BasicFilterServeUnavailable {
    match error {
        FilterTransportError::Capacity(_) | FilterTransportError::Exhausted => {
            BasicFilterServeUnavailable::ResourceRefused
        }
        FilterTransportError::Invalidated => BasicFilterServeUnavailable::Invalidated,
        _ => BasicFilterServeUnavailable::StorageFailure,
    }
}
fn read_failure(error: BasicPeerReadError) -> BasicFilterServeUnavailable {
    match error {
        BasicPeerReadError::Disabled => BasicFilterServeUnavailable::NoBackend,
        BasicPeerReadError::Resource(_) | BasicPeerReadError::WirePayload => {
            BasicFilterServeUnavailable::ResourceRefused
        }
        BasicPeerReadError::Storage(_error) => BasicFilterServeUnavailable::StorageFailure,
    }
}

fn encode_complete(
    request: FilterRequest,
    stop_height: u32,
    records: Vec<BasicFilterRecordView>,
    previous_header: Hash32,
    magic: NetworkMagic,
    budget: &mut BasicPeerServeBudget,
) -> Result<Vec<Vec<u8>>, BasicPeerReadError> {
    let refused =
        || BasicPeerReadError::Resource(BasicPeerResourceRefusal(Axis::EncodedSuffixBytes));
    let frame_count = if matches!(request, FilterRequest::Filters { .. }) {
        records.len()
    } else {
        1
    };
    // Both the Vec and eventual VecDeque containers retain ownership metadata.
    budget.charge(
        Axis::LogicalCopyBytes,
        frame_count as u64 * 2 * std::mem::size_of::<Vec<u8>>() as u64,
    )?;
    let mut frames = Vec::with_capacity(frame_count);
    for record in &records {
        if matches!(request, FilterRequest::Filters { .. }) {
            charge_encoding(
                budget,
                cfilter_frame_bytes(record.encoded_bytes().len() as u64).ok_or_else(refused)?,
            )?;
        }
    }
    match request {
        FilterRequest::Filters { filter_type, .. } => {
            for record in records {
                budget.charge(Axis::LogicalCopyBytes, record.encoded_bytes().len() as u64)?;
                let message = WireNetworkMessage::CFilter(CFilter {
                    filter_type,
                    block_hash: record.identity().block_hash(),
                    filter: record.encoded_bytes().to_vec(),
                });
                frames.push(
                    message
                        .encode_wire(magic)
                        .map_err(|_error| BasicPeerReadError::WirePayload)?,
                );
            }
        }
        FilterRequest::Headers {
            filter_type,
            stop_hash,
            ..
        } => {
            charge_encoding(
                budget,
                cfheaders_frame_bytes(records.len() as u64).ok_or_else(refused)?,
            )?;
            budget.charge(Axis::LogicalCopyBytes, records.len() as u64 * 32)?;
            let message = WireNetworkMessage::CFHeaders(CFHeaders {
                filter_type,
                stop_hash,
                previous_filter_header: previous_header,
                filter_hashes: records
                    .iter()
                    .map(|record| record.identity().filter_hash().into())
                    .collect(),
            });
            frames.push(
                message
                    .encode_wire(magic)
                    .map_err(|_error| BasicPeerReadError::WirePayload)?,
            );
        }
        FilterRequest::Checkpoints {
            filter_type,
            stop_hash,
        } => {
            charge_encoding(
                budget,
                cfcheckpt_frame_bytes(stop_height).ok_or_else(refused)?,
            )?;
            budget.charge(Axis::LogicalCopyBytes, records.len() as u64 * 32)?;
            let message = WireNetworkMessage::CFCheckpt(CFCheckpt {
                filter_type,
                stop_hash,
                filter_headers: records
                    .iter()
                    .map(|record| record.filter_header().into())
                    .collect(),
            });
            frames.push(
                message
                    .encode_wire(magic)
                    .map_err(|_error| BasicPeerReadError::WirePayload)?,
            );
        }
    }
    Ok(frames)
}

fn charge_encoding(
    budget: &mut BasicPeerServeBudget,
    frame: u64,
) -> Result<(), BasicPeerResourceRefusal> {
    let refusal = BasicPeerResourceRefusal(Axis::LogicalCopyBytes);
    let payload = frame.checked_sub(24).ok_or(refusal)?;
    let padded = payload
        .checked_add(9)
        .and_then(|n| n.checked_add(63))
        .map(|n| n / 64 * 64)
        .and_then(|n| n.checked_add(64))
        .ok_or(refusal)?;
    let copies = payload
        .checked_mul(3)
        .and_then(|n| n.checked_add(56))
        .ok_or(refusal)?;
    budget.charge(Axis::HashPaddedBytes, padded)?;
    budget.charge(Axis::LogicalCopyBytes, copies)
}

#[cfg(test)]
mod budget_tests {
    use super::*;
    #[test]
    fn phase160_complete_preparation_encoding_charges_payload_frame_and_sha_padding() {
        let mut budget = BasicPeerServeBudget::default();
        charge_encoding(&mut budget, 58).expect("bounded frame");
        assert_eq!(budget.work().get(Axis::HashPaddedBytes), 128);
        assert_eq!(budget.work().get(Axis::LogicalCopyBytes), 158);
    }
}
