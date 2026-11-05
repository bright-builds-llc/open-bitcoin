// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/net.cpp

use super::*;
use crate::network::{
    ContinuationDenied, SyncPeerAuthorized, SyncPeerDispatchOutcome, SyncPeerLifetime,
    SyncPeerObservation, SyncPeerRequestAuthority,
};
use crate::sync::{
    PeerCapabilitySummary, PeerFailureReason, PeerSyncState, progress::PeerProgress,
    session::request_cleanup_result,
};
use open_bitcoin_core::{chainstate::CoinsView, primitives::InventoryType};
use open_bitcoin_network::PeerState;

trait RequestOwner {
    fn peer(&self) -> PeerId;
    fn requested(&mut self) -> Result<Vec<BlockHash>, SyncRuntimeError>;
    fn request(
        &mut self,
        hashes: &[BlockHash],
    ) -> Result<Vec<WireNetworkMessage>, SyncRuntimeError>;
}

impl<S: crate::ChainstateStore, V: CoinsView> RequestOwner for SyncPeerRequestAuthority<'_, S, V> {
    fn peer(&self) -> PeerId {
        SyncPeerRequestAuthority::peer(self)
    }
    fn requested(&mut self) -> Result<Vec<BlockHash>, SyncRuntimeError> {
        SyncPeerRequestAuthority::requested(self)
            .map_err(|error| ManagedNetworkAuthorityError::from(error).into())
    }
    fn request(
        &mut self,
        hashes: &[BlockHash],
    ) -> Result<Vec<WireNetworkMessage>, SyncRuntimeError> {
        SyncPeerRequestAuthority::request(self, hashes)
            .map_err(|error| ManagedNetworkAuthorityError::from(error).into())
    }
}

fn select_missing(
    runtime: &DurableSyncRuntime,
    mut known: impl FnMut(BlockHash) -> Result<(), SyncRuntimeError>,
) -> Result<Vec<BlockHash>, SyncRuntimeError> {
    let available_global = runtime
        .config
        .max_blocks_in_flight_total
        .saturating_sub(runtime.inflight_blocks.len());
    if available_global == 0 {
        return Ok(Vec::new());
    }
    let active_chain_hashes = runtime
        .network
        .chainstate_snapshot()?
        .active_chain
        .into_iter()
        .map(|position| position.block_hash)
        .collect::<BTreeSet<_>>();
    let mut requested = Vec::new();
    for entry in runtime
        .network
        .requestable_header_entries(runtime.consensus_params)?
    {
        if requested.len() >= available_global
            || active_chain_hashes.contains(&entry.block_hash)
            || runtime.inflight_blocks.contains(&entry.block_hash)
        {
            continue;
        }
        if runtime.store.load_block(entry.block_hash)?.is_some() {
            known(entry.block_hash)?;
            continue;
        }
        requested.push(entry.block_hash);
    }
    Ok(requested)
}

fn request_core(
    owner: &mut impl RequestOwner,
    inflight: &mut BTreeSet<BlockHash>,
    cap: usize,
    hashes: &[BlockHash],
) -> Result<Vec<WireNetworkMessage>, SyncRuntimeError> {
    let requested = hashes
        .iter()
        .copied()
        .filter(|hash| !inflight.contains(hash))
        .take(cap.saturating_sub(inflight.len()))
        .collect::<Vec<_>>();
    let outbound = owner.request(&requested)?;
    let peer = owner.peer();
    handoff_core(
        owner,
        inflight,
        cap,
        outbound
            .into_iter()
            .map(|message| (peer, message))
            .collect(),
    )
}

fn handoff_core(
    owner: &mut impl RequestOwner,
    inflight: &mut BTreeSet<BlockHash>,
    cap: usize,
    targeted: Vec<(PeerId, WireNetworkMessage)>,
) -> Result<Vec<WireNetworkMessage>, SyncRuntimeError> {
    if targeted.is_empty() {
        return Ok(Vec::new());
    }
    if targeted.iter().any(|(target, _)| *target != owner.peer()) {
        return Err(SyncRuntimeError::Network {
            message: "compact timeout action target does not match connected session".to_owned(),
        });
    }
    let requested: BTreeSet<_> = owner.requested()?.into_iter().collect();
    let mut hashes = BTreeSet::new();
    for (_, message) in &targeted {
        if let WireNetworkMessage::GetData(inventory) = message {
            for item in &inventory.inventory {
                if matches!(
                    item.inventory_type,
                    InventoryType::Block | InventoryType::WitnessBlock
                ) {
                    let hash = BlockHash::from(item.object_hash);
                    if !requested.contains(&hash) {
                        return Err(SyncRuntimeError::Network {
                            message: "request batch lacks retained peer membership".to_owned(),
                        });
                    }
                    hashes.insert(hash);
                }
            }
        }
    }
    if hashes.difference(inflight).count() > cap.saturating_sub(inflight.len()) {
        return Err(SyncRuntimeError::ResourceLimit {
            message: "request batch exceeds global in-flight capacity".to_owned(),
        });
    }
    inflight.extend(hashes);
    Ok(targeted.into_iter().map(|(_, message)| message).collect())
}

struct PreparedRequests {
    known: Vec<BlockHash>,
    selected: Result<Vec<BlockHash>, SyncRuntimeError>,
}

pub(in crate::sync) struct PeerRequests {
    token: SyncPeerLifetime,
    retained: Vec<BlockHash>,
    maybe_peer: Option<PeerState>,
    compact_pending: bool,
    maybe_denied_requests: Option<Vec<BlockHash>>,
}

impl PeerRequests {
    pub(in crate::sync) fn new(token: SyncPeerLifetime) -> Self {
        Self {
            token,
            retained: Vec::new(),
            maybe_peer: None,
            compact_pending: false,
            maybe_denied_requests: None,
        }
    }

    pub(in crate::sync) fn with_outbox<T>(
        &self,
        network: &crate::DurableNetworkHandle,
        command: impl FnOnce(PeerId) -> T,
    ) -> Result<Option<T>, SyncRuntimeError> {
        network
            .with_sync_peer_outbox(&self.token, command)
            .map_err(Into::into)
    }

    pub(in crate::sync) fn with_outbox_cleanup<T>(
        &self,
        network: &crate::DurableNetworkHandle,
        command: impl FnOnce(PeerId) -> T,
    ) -> Result<T, SyncRuntimeError> {
        network
            .with_sync_peer_outbox_cleanup(&self.token, command)
            .map_err(Into::into)
    }

    fn require_active(&self) -> Result<(), SyncRuntimeError> {
        if self.maybe_denied_requests.is_some() {
            return Err(SyncRuntimeError::Network {
                message: "BASIC continuation requires authoritative termination".to_owned(),
            });
        }
        Ok(())
    }

    pub(in crate::sync) fn request_missing(
        &mut self,
        runtime: &mut DurableSyncRuntime,
    ) -> Result<Vec<WireNetworkMessage>, SyncRuntimeError> {
        self.require_active()?;
        let mut known = Vec::new();
        let selected = select_missing(runtime, |hash| {
            known.push(hash);
            Ok(())
        });
        let prepared = PreparedRequests { known, selected };
        let maybe_preparation_error = prepared.selected.as_ref().err().cloned();
        let network = runtime.network.clone();
        let result = network.with_sync_peer_requests(&self.token, |mut owner| {
            for hash in prepared.known {
                owner.note_local(hash);
            }
            let requested = prepared.selected?;
            if runtime
                .config
                .max_blocks_in_flight_total
                .saturating_sub(runtime.inflight_blocks.len())
                == 0
            {
                return Ok(Vec::new());
            }
            request_core(
                &mut owner,
                &mut runtime.inflight_blocks,
                runtime.config.max_blocks_in_flight_total,
                &requested,
            )
        });
        match result {
            Ok(result) => result,
            Err(denial) => match maybe_preparation_error {
                Some(error) => request_cleanup_result(Err(error), Err(denial.into())),
                None => Err(denial.into()),
            },
        }
    }

    pub(in crate::sync) fn expire(
        &mut self,
        runtime: &mut DurableSyncRuntime,
        timestamp: i64,
    ) -> Result<Vec<WireNetworkMessage>, SyncRuntimeError> {
        self.require_active()?;
        let network = runtime.network.clone();
        network.with_sync_peer_requests(&self.token, |mut owner| {
            let targeted = owner
                .expire(timestamp)
                .map_err(ManagedNetworkAuthorityError::from)?;
            handoff_core(
                &mut owner,
                &mut runtime.inflight_blocks,
                runtime.config.max_blocks_in_flight_total,
                targeted,
            )
        })?
    }

    pub(in crate::sync) fn handoff(
        &mut self,
        runtime: &mut DurableSyncRuntime,
        targeted: Vec<(PeerId, WireNetworkMessage)>,
    ) -> Result<Vec<WireNetworkMessage>, SyncRuntimeError> {
        self.require_active()?;
        let network = runtime.network.clone();
        network.with_sync_peer_requests(&self.token, |mut owner| {
            handoff_core(
                &mut owner,
                &mut runtime.inflight_blocks,
                runtime.config.max_blocks_in_flight_total,
                targeted,
            )
        })?
    }

    fn reconcile(
        &mut self,
        runtime: &mut DurableSyncRuntime,
        observed: SyncPeerObservation,
    ) -> Result<bool, SyncRuntimeError> {
        let previous = release_unrequested_hashes(runtime, &self.retained);
        let SyncPeerObservation::Present {
            requests,
            peer,
            compact_pending,
        } = observed
        else {
            previous?;
            self.maybe_peer = None;
            self.compact_pending = false;
            return Ok(false);
        };
        let current = release_unrequested_hashes(runtime, &requests);
        request_cleanup_result(previous, current)?;
        self.retained = requests;
        self.maybe_peer = Some(*peer);
        self.compact_pending = compact_pending;
        Ok(true)
    }

    pub(in crate::sync) fn renew(
        &mut self,
        runtime: &mut DurableSyncRuntime,
        issued: &[WireNetworkMessage],
    ) -> Result<bool, SyncRuntimeError> {
        self.require_active()?;
        let observed = runtime.network.observe_sync_peer(&self.token)?;
        if matches!(observed, SyncPeerObservation::Absent) {
            let batch: Vec<_> = issued
                .iter()
                .filter_map(|message| {
                    let WireNetworkMessage::GetData(inventory) = message else {
                        return None;
                    };
                    Some(
                        inventory
                            .inventory
                            .iter()
                            .filter(|item| {
                                matches!(
                                    item.inventory_type,
                                    InventoryType::Block | InventoryType::WitnessBlock
                                )
                            })
                            .map(|item| BlockHash::from(item.object_hash)),
                    )
                })
                .flatten()
                .collect();
            let previous = self.reconcile(runtime, observed);
            let batch_cleanup = release_unrequested_hashes(runtime, &batch);
            return request_cleanup_result(previous, batch_cleanup);
        }
        self.reconcile(runtime, observed)
    }

    pub(in crate::sync) fn record_ignored(
        &mut self,
        runtime: &mut DurableSyncRuntime,
        timestamp: i64,
        progress: &mut PeerProgress,
        receipt: &[BlockHash],
    ) -> Result<(), SyncRuntimeError> {
        self.require_active()?;
        let observed = runtime.network.observe_sync_peer(&self.token)?;
        // This is the old ignored-body policy outcome, never a manager receive.
        // Credit occurs inside a fresh validated scope, with no I/O or grant.
        runtime
            .network
            .with_sync_peer_requests(&self.token, |_owner| {
                progress.record_activity(timestamp);
            })?;
        let observation_cleanup = self.reconcile(runtime, observed).map(|_| ());
        request_cleanup_result(
            observation_cleanup,
            release_unrequested_hashes(runtime, receipt),
        )
    }

    pub(in crate::sync) fn dispatch(
        &mut self,
        runtime: &mut DurableSyncRuntime,
        message: WireNetworkMessage,
        timestamp: i64,
        receipt: &[BlockHash],
        progress: &mut PeerProgress,
    ) -> Result<Result<crate::network::ManagedSyncMessageResult, SyncRuntimeError>, SyncRuntimeError>
    {
        self.require_active()?;
        let continuation: crate::network::SyncPeerContinuation<'_, _, _> =
            runtime.network.begin_sync_peer_dispatch(
                &self.token,
                message,
                timestamp,
                runtime.verify_flags,
                runtime.consensus_params,
            )?;
        let outcome = continuation.resolve();
        progress.record_activity(timestamp);
        self.settle_dispatch(runtime, outcome, receipt)
    }

    fn settle_dispatch(
        &mut self,
        runtime: &mut DurableSyncRuntime,
        outcome: SyncPeerDispatchOutcome,
        receipt: &[BlockHash],
    ) -> Result<Result<crate::network::ManagedSyncMessageResult, SyncRuntimeError>, SyncRuntimeError>
    {
        match outcome.into_parts() {
            Ok(authorized) => Ok(self.settle_authorized(runtime, authorized, receipt)),
            Err(denied) => self.retain_denied(denied).map(Ok),
        }
    }

    fn settle_authorized(
        &mut self,
        runtime: &mut DurableSyncRuntime,
        authorized: SyncPeerAuthorized,
        receipt: &[BlockHash],
    ) -> Result<crate::network::ManagedSyncMessageResult, SyncRuntimeError> {
        let (observed, result) = authorized.into_parts();
        let observation_cleanup = self.reconcile(runtime, observed).map(|_| ());
        let receipt_cleanup = release_unrequested_hashes(runtime, receipt);
        request_cleanup_result(
            result.map_err(Into::into),
            request_cleanup_result(observation_cleanup, receipt_cleanup),
        )
    }

    fn retain_denied(
        &mut self,
        denied: ContinuationDenied,
    ) -> Result<crate::network::ManagedSyncMessageResult, SyncRuntimeError> {
        let (observed, denial, maybe_primary) = denied.into_parts();
        self.maybe_denied_requests = Some(match observed {
            SyncPeerObservation::Present { requests, .. } => requests,
            SyncPeerObservation::Absent => Vec::new(),
        });
        // Two bounded sets remain; this known denial authorizes no cleanup.
        let (result, cleanup) = match maybe_primary {
            Some(primary) => (Err(primary.into()), Err(denial.into())),
            None => (Err(denial.into()), Ok(())),
        };
        request_cleanup_result(result, cleanup)
    }

    pub(in crate::sync) fn finish(
        &mut self,
        runtime: &mut DurableSyncRuntime,
    ) -> Result<(), SyncRuntimeError> {
        if let Some(denied) = self.maybe_denied_requests.as_ref() {
            // Recheck ownership without materializing a third candidate set.
            runtime.network.require_absent_sync_peer(&self.token)?;
            let retained_cleanup = release_unrequested_hashes(runtime, &self.retained);
            let denied_cleanup = release_unrequested_hashes(runtime, denied);
            request_cleanup_result(retained_cleanup, denied_cleanup)?;
            self.retained.clear();
            self.maybe_denied_requests = None;
            return Ok(());
        }
        let observed = runtime.network.finish_sync_peer(&self.token)?;
        self.reconcile(runtime, observed).map(|_| ())
    }

    pub(in crate::sync) fn block_was_requested(
        &self,
        runtime: &DurableSyncRuntime,
        hash: BlockHash,
    ) -> Result<bool, SyncRuntimeError> {
        self.require_active()?;
        runtime
            .network
            .with_sync_peer_requests(&self.token, |owner| {
                if !runtime.inflight_blocks.contains(&hash) {
                    return Ok(false);
                }
                let requested = owner
                    .requested()
                    .map_err(ManagedNetworkAuthorityError::from)?;
                Ok(crate::sync::block_response::runtime_requested_block(
                    &runtime.inflight_blocks,
                    &requested,
                    hash,
                ))
            })?
    }

    pub(in crate::sync) fn notfound_was_requested(
        &self,
        runtime: &DurableSyncRuntime,
        message: &WireNetworkMessage,
    ) -> Result<bool, SyncRuntimeError> {
        let WireNetworkMessage::NotFound(inventory) = message else {
            return Ok(false);
        };
        for item in &inventory.inventory {
            if matches!(
                item.inventory_type,
                InventoryType::Block | InventoryType::WitnessBlock
            ) && self.block_was_requested(runtime, BlockHash::from(item.object_hash))?
            {
                return Ok(true);
            }
        }
        Ok(false)
    }

    pub(in crate::sync) fn pending(&self, runtime: &DurableSyncRuntime) -> bool {
        self.compact_pending
            || self
                .retained
                .iter()
                .any(|hash| runtime.inflight_blocks.contains(hash))
    }

    pub(in crate::sync) fn capabilities(&self) -> Option<PeerCapabilitySummary> {
        self.maybe_peer.as_ref().map(|peer| PeerCapabilitySummary {
            services_bits: peer.remote_services_bits,
            user_agent: peer.remote_user_agent.clone(),
            start_height: peer.remote_start_height,
            wtxidrelay: peer.remote_wtxidrelay,
            prefers_headers: peer.remote_prefers_headers,
        })
    }

    pub(in crate::sync) fn observe_capabilities(
        &mut self,
        runtime: &mut DurableSyncRuntime,
        progress: &mut PeerProgress,
    ) -> Result<(), SyncRuntimeError> {
        self.renew(runtime, &[])?;
        progress.maybe_capabilities = self.capabilities();
        Ok(())
    }

    pub(in crate::sync) fn complete(
        &mut self,
        runtime: &mut DurableSyncRuntime,
        progress: &mut PeerProgress,
    ) -> Result<(), SyncRuntimeError> {
        self.observe_capabilities(runtime, progress)?;
        if self.maybe_peer.as_ref().is_none_or(|peer| {
            !(peer.local_version_sent
                && peer.remote_version_received
                && peer.local_verack_sent
                && peer.remote_verack_received)
        }) {
            progress.state = PeerSyncState::Stalled;
            progress.maybe_failure_reason = Some(PeerFailureReason::Stall);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests;
#[cfg(test)]
pub(in crate::sync) use tests::{handoff_requested_messages, request_missing_blocks};
