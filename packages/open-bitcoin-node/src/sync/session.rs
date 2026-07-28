// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net.cpp
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/headerssync.cpp
// - packages/bitcoin-knots/src/sync.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp

use std::{
    collections::{BTreeMap, VecDeque},
    future::poll_fn,
    sync::{
        Arc, Mutex,
        atomic::{AtomicU64, Ordering},
    },
    task::{Poll, Waker},
};

use open_bitcoin_core::{
    consensus::block_hash,
    primitives::{BlockHash, InventoryType},
};
use open_bitcoin_network::{
    PHASE94_MAX_AGGREGATE_QUEUED_MESSAGES, PHASE94_MAX_PEER_QUEUED_MESSAGES, PeerId,
    WireNetworkMessage,
};

use super::{
    BlockConnectDisposition, DurableSyncRuntime, PeerFailureReason, PeerSyncState,
    ResolvedSyncPeerAddress, SyncPeerReceiveOutcome, SyncPeerSession, SyncRuntimeError,
    SyncTransport, block_reconcile,
    progress::{self, PeerFailure, PeerProgress},
    tip,
};
use crate::network::{AnnouncementPreparationOutcome, PeerEmission, PeerOutboxSnapshot};

struct AnnouncementOutbox {
    emissions: VecDeque<PeerEmission>,
    readiness: Arc<AnnouncementOutboxReadiness>,
}

impl Default for AnnouncementOutbox {
    fn default() -> Self {
        Self {
            emissions: VecDeque::new(),
            readiness: Arc::new(AnnouncementOutboxReadiness::default()),
        }
    }
}

#[derive(Default)]
struct AnnouncementOutboxReadiness {
    generation: AtomicU64,
    maybe_waker: Mutex<Option<Waker>>,
}

impl AnnouncementOutboxReadiness {
    fn notify(&self) {
        self.generation.fetch_add(1, Ordering::Release);
        let maybe_waker = self
            .maybe_waker
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .take();
        if let Some(waker) = maybe_waker {
            waker.wake();
        }
    }
}

/// A cancellation-safe readiness cursor for one live peer announcement outbox.
pub struct AnnouncementOutboxNotification {
    readiness: Arc<AnnouncementOutboxReadiness>,
    observed_generation: u64,
}

impl AnnouncementOutboxNotification {
    /// Waits until at least one enqueue occurred after the last observed generation.
    pub async fn notified(&mut self) {
        poll_fn(|context| {
            let generation = self.readiness.generation.load(Ordering::Acquire);
            if generation != self.observed_generation {
                self.observed_generation = generation;
                return Poll::Ready(());
            }

            let mut maybe_waker = self
                .readiness
                .maybe_waker
                .lock()
                .unwrap_or_else(|poisoned| poisoned.into_inner());
            let generation = self.readiness.generation.load(Ordering::Acquire);
            if generation != self.observed_generation {
                self.observed_generation = generation;
                return Poll::Ready(());
            }
            *maybe_waker = Some(context.waker().clone());
            Poll::Pending
        })
        .await
    }
}

#[derive(Clone, Default)]
pub struct AnnouncementOutboxRegistry {
    outboxes: Arc<Mutex<BTreeMap<PeerId, AnnouncementOutbox>>>,
}

impl AnnouncementOutboxRegistry {
    /// Makes one live peer eligible to receive prepared announcement emissions.
    pub fn register_peer(
        &self,
        peer_id: PeerId,
    ) -> Result<AnnouncementOutboxNotification, SyncRuntimeError> {
        let mut outboxes = self.lock_outboxes()?;
        if outboxes.contains_key(&peer_id) {
            return Err(SyncRuntimeError::Network {
                message: format!("announcement outbox peer {peer_id} is already registered"),
            });
        }
        let outbox = outboxes.entry(peer_id).or_default();
        Ok(AnnouncementOutboxNotification {
            observed_generation: outbox.readiness.generation.load(Ordering::Acquire),
            readiness: Arc::clone(&outbox.readiness),
        })
    }

    /// Removes one peer and discards only its bounded volatile announcement queue.
    pub fn unregister_peer(&self, peer_id: PeerId) -> Result<(), SyncRuntimeError> {
        self.lock_outboxes()?.remove(&peer_id);
        Ok(())
    }

    /// Captures queue pressure without retaining the registry lock during preparation.
    pub fn snapshots(&self) -> Result<Vec<PeerOutboxSnapshot>, SyncRuntimeError> {
        Ok(self
            .lock_outboxes()?
            .iter()
            .map(|(peer_id, outbox)| {
                PeerOutboxSnapshot::new(
                    *peer_id,
                    outbox.emissions.len(),
                    PHASE94_MAX_PEER_QUEUED_MESSAGES,
                )
            })
            .collect())
    }

    /// Enqueues prepared emissions while preserving per-peer and aggregate bounds.
    pub fn enqueue_prepared(
        &self,
        outcomes: Vec<AnnouncementPreparationOutcome>,
    ) -> Result<(), SyncRuntimeError> {
        let mut outboxes = self.lock_outboxes()?;
        let mut aggregate_queued = outboxes
            .values()
            .map(|outbox| outbox.emissions.len())
            .sum::<usize>();
        let mut readiness_notifications = Vec::new();
        for outcome in outcomes {
            let AnnouncementPreparationOutcome::Ready(emission) = outcome else {
                continue;
            };
            if aggregate_queued >= PHASE94_MAX_AGGREGATE_QUEUED_MESSAGES {
                break;
            }
            let Some(outbox) = outboxes.get_mut(&emission.peer_id()) else {
                continue;
            };
            if outbox.emissions.len() >= PHASE94_MAX_PEER_QUEUED_MESSAGES {
                continue;
            }
            outbox.emissions.push_back(*emission);
            readiness_notifications.push(Arc::clone(&outbox.readiness));
            aggregate_queued = aggregate_queued.saturating_add(1);
        }
        drop(outboxes);
        for readiness in readiness_notifications {
            readiness.notify();
        }
        Ok(())
    }

    /// Takes the current FIFO batch for exactly one peer.
    pub fn take_peer_emissions(
        &self,
        peer_id: PeerId,
    ) -> Result<Vec<PeerEmission>, SyncRuntimeError> {
        let mut outboxes = self.lock_outboxes()?;
        let Some(outbox) = outboxes.get_mut(&peer_id) else {
            return Ok(Vec::new());
        };
        Ok(outbox.emissions.drain(..).collect())
    }

    fn lock_outboxes(
        &self,
    ) -> Result<std::sync::MutexGuard<'_, BTreeMap<PeerId, AnnouncementOutbox>>, SyncRuntimeError>
    {
        self.outboxes
            .lock()
            .map_err(|_error| SyncRuntimeError::Network {
                message: "announcement outbox registry is unavailable".to_string(),
            })
    }
}

impl DurableSyncRuntime {
    #[cfg(test)]
    pub(super) fn sync_connected_peer<S: SyncPeerSession, C: FnMut() -> i64>(
        &mut self,
        session: S,
        peer: &ResolvedSyncPeerAddress,
        peer_id: PeerId,
        attempts: u8,
        timestamp: i64,
        clock: &mut C,
    ) -> Result<PeerProgress, Box<PeerFailure>> {
        let mut never_cancel = || false;
        self.sync_connected_peer_with_cancel(
            session,
            peer,
            peer_id,
            attempts,
            timestamp,
            (clock, &mut never_cancel),
        )
    }

    fn sync_connected_peer_with_cancel<
        S: SyncPeerSession,
        C: FnMut() -> i64,
        K: FnMut() -> bool,
    >(
        &mut self,
        mut session: S,
        peer: &ResolvedSyncPeerAddress,
        peer_id: PeerId,
        attempts: u8,
        timestamp: i64,
        controls: (&mut C, &mut K),
    ) -> Result<PeerProgress, Box<PeerFailure>> {
        let mut progress = PeerProgress::new(peer.clone(), self.config.network, attempts);
        let mut maybe_failure_reason_override = None;
        let mut outbox_registered = false;
        let mut network_connected = false;
        let result = (|| -> Result<(), SyncRuntimeError> {
            self.announcement_outboxes.register_peer(peer_id)?;
            outbox_registered = true;
            let mut outbound = self.network.connect_outbound_peer(peer_id, timestamp)?;
            network_connected = true;
            outbound.extend(block_reconcile::request_missing_blocks(self, peer_id)?);
            self.send_all_for_peer(&mut session, peer_id, &outbound)?;

            let mut messages_received = 0_usize;
            while messages_received < self.config.max_messages_per_peer {
                if (controls.1)() {
                    self.complete_peer_session_progress(&mut progress, peer_id);
                    return Ok(());
                }
                let receive_outcome = match session.receive(self.config.network.magic()) {
                    Ok(receive_outcome) => receive_outcome,
                    Err(error) => {
                        let reason = progress::peer_failure_reason_for_error(&error);
                        if reason == PeerFailureReason::MalformedBlock {
                            progress.record_malformed_block();
                            maybe_failure_reason_override = Some(reason);
                        }
                        return Err(error);
                    }
                };
                let (message, current_timestamp) = match receive_outcome {
                    SyncPeerReceiveOutcome::Message(message) => {
                        let current_timestamp = (controls.0)();
                        messages_received = messages_received.saturating_add(1);
                        (message, current_timestamp)
                    }
                    SyncPeerReceiveOutcome::Idle => {
                        let current_timestamp = (controls.0)();
                        if (controls.1)() {
                            self.complete_peer_session_progress(&mut progress, peer_id);
                            return Ok(());
                        }
                        let targeted = self
                            .network
                            .expire_compact_download_timeouts(current_timestamp)?;
                        if targeted
                            .iter()
                            .any(|(target_peer_id, _message)| *target_peer_id != peer_id)
                        {
                            return Err(SyncRuntimeError::Network {
                                message:
                                    "compact timeout action target does not match connected session"
                                        .to_string(),
                            });
                        }
                        let fallback_block_hashes = targeted
                            .iter()
                            .filter_map(|(_target_peer_id, message)| match message {
                                WireNetworkMessage::GetData(inventory) => {
                                    Some(&inventory.inventory)
                                }
                                _ => None,
                            })
                            .flatten()
                            .filter(|item| {
                                matches!(
                                    item.inventory_type,
                                    InventoryType::Block | InventoryType::WitnessBlock
                                )
                            })
                            .map(|item| BlockHash::from(item.object_hash))
                            .collect::<Vec<_>>();
                        let outbound = block_reconcile::request_tracked_blocks(
                            self,
                            peer_id,
                            &fallback_block_hashes,
                        )?;
                        self.send_all_for_peer(&mut session, peer_id, &outbound)?;
                        if !self.peer_has_pending_download_work(peer_id) {
                            self.complete_peer_session_progress(&mut progress, peer_id);
                            return Ok(());
                        }
                        continue;
                    }
                    SyncPeerReceiveOutcome::Closed => {
                        self.complete_peer_session_progress(&mut progress, peer_id);
                        return Ok(());
                    }
                };
                progress.record_activity(current_timestamp);
                let maybe_header_count = match &message {
                    WireNetworkMessage::Headers(headers) => Some(headers.headers.len()),
                    _ => None,
                };
                let maybe_terminal_header_hash = match &message {
                    WireNetworkMessage::Headers(headers) => headers.headers.last().map(block_hash),
                    _ => None,
                };
                let maybe_block = match &message {
                    WireNetworkMessage::Block(block) => Some(block.clone()),
                    _ => None,
                };
                let maybe_block_hash = maybe_block.as_ref().map(|block| block_hash(&block.header));
                let block_response_was_requested = match maybe_block_hash {
                    Some(block_hash) => self.peer_requested_block(peer_id, block_hash)?,
                    None => false,
                };
                let block_response_is_best_chain = match maybe_block.as_ref() {
                    Some(block) => {
                        self.block_has_best_chain_header(block_hash(&block.header))?
                            || self.block_extends_active_tip(block)?
                    }
                    None => false,
                };
                let notfound_was_requested =
                    self.message_reports_requested_block_notfound(peer_id, &message)?;
                block_reconcile::release_inflight_for_message(self, &message);

                if let Some(block) = maybe_block.as_ref()
                    && !block_response_was_requested
                {
                    self.record_unrequested_block_response(
                        &mut progress,
                        block,
                        block_response_is_best_chain,
                    )?;
                    let outbound = block_reconcile::request_missing_blocks(self, peer_id)?;
                    self.send_all_for_peer(&mut session, peer_id, &outbound)?;
                    continue;
                }

                let sync_result = match self.network.receive_sync_message(
                    peer_id,
                    message,
                    current_timestamp,
                    self.verify_flags,
                    self.consensus_params,
                ) {
                    Ok(result) => result,
                    Err(error) => {
                        if maybe_block_hash.is_some() {
                            progress.record_invalid_block();
                            maybe_failure_reason_override = Some(PeerFailureReason::InvalidBlock);
                        }
                        return Err(error.into());
                    }
                };
                let mut outbound = sync_result.outbound;
                if let Some(header_count) = maybe_header_count {
                    progress.record_validated_headers(header_count);
                    let peer_manager = self.network.peer_manager_snapshot()?;
                    tip::record_peer_terminal_tip(
                        &mut progress,
                        peer_manager.header_store(),
                        maybe_terminal_header_hash,
                    );
                }
                if notfound_was_requested {
                    progress.record_block_notfound();
                }
                let mut should_persist_progress = maybe_header_count.is_some_and(|count| count > 0);
                if let Some(disposition) = sync_result.maybe_block_disposition {
                    should_persist_progress =
                        matches!(disposition, BlockConnectDisposition::Connected(_));
                    self.record_block_disposition(
                        &mut progress,
                        maybe_block.as_ref(),
                        disposition,
                        block_response_was_requested,
                        block_response_is_best_chain,
                    )?;
                }
                let reconcile_progress = block_reconcile::reconcile_best_chain_for_live_session(
                    self,
                    current_timestamp,
                )?;
                should_persist_progress |= reconcile_progress.should_persist_progress();
                self.record_reconcile_progress(reconcile_progress);
                if should_persist_progress {
                    self.persist_progress_and_dispatch_tip()?;
                }
                outbound.extend(block_reconcile::request_missing_blocks(self, peer_id)?);
                self.send_all_for_peer(&mut session, peer_id, &outbound)?;
            }
            progress.state = PeerSyncState::Connected;
            progress.maybe_capabilities = self.peer_capabilities(peer_id);

            Ok(())
        })();
        let outstanding_blocks = if network_connected {
            self.network
                .peer_requested_blocks(peer_id)
                .unwrap_or_default()
        } else {
            Vec::new()
        };
        for block_hash in outstanding_blocks {
            self.inflight_blocks.remove(&block_hash);
        }
        let outbox_cleanup_result = if outbox_registered {
            self.announcement_outboxes.unregister_peer(peer_id)
        } else {
            Ok(())
        };
        let disconnect_result = if network_connected {
            self.network.disconnect_peer(peer_id)
        } else {
            Ok(())
        };
        let disconnect_result = disconnect_result.map_err(SyncRuntimeError::from);
        match (result, outbox_cleanup_result, disconnect_result) {
            (Ok(()), Ok(()), Ok(())) => Ok(progress),
            (Ok(()), Err(error), _) | (Ok(()), Ok(()), Err(error)) => {
                if progress.maybe_capabilities.is_none() {
                    progress.maybe_capabilities = self.peer_capabilities(peer_id);
                }
                Err(Box::new(PeerFailure {
                    peer: peer.clone(),
                    reason: progress::peer_failure_reason_for_error(&error),
                    error,
                    attempts,
                    maybe_progress: Some(progress),
                }))
            }
            (Err(error), _, _) => {
                if progress.maybe_capabilities.is_none() {
                    progress.maybe_capabilities = self.peer_capabilities(peer_id);
                }
                Err(Box::new(PeerFailure {
                    peer: peer.clone(),
                    reason: maybe_failure_reason_override
                        .clone()
                        .unwrap_or_else(|| progress::peer_failure_reason_for_error(&error)),
                    error,
                    attempts,
                    maybe_progress: Some(progress),
                }))
            }
        }
    }

    pub(super) fn sync_peer_with_retries<
        T: SyncTransport,
        C: FnMut() -> i64,
        K: FnMut() -> bool,
    >(
        &mut self,
        transport: &mut T,
        peer: &ResolvedSyncPeerAddress,
        peer_id: PeerId,
        timestamp: i64,
        clock: &mut C,
        should_cancel: &mut K,
    ) -> Result<PeerProgress, Box<PeerFailure>> {
        let mut attempts = 0_u8;
        let max_attempts = self.config.max_peer_retries.saturating_add(1);
        loop {
            attempts = attempts.saturating_add(1);
            match transport.connect(peer, &self.config) {
                Ok(session) => {
                    return self.sync_connected_peer_with_cancel(
                        session,
                        peer,
                        peer_id,
                        attempts,
                        timestamp,
                        (clock, should_cancel),
                    );
                }
                Err(error) if attempts < max_attempts => {
                    let _ = error;
                }
                Err(error) => {
                    return Err(Box::new(PeerFailure {
                        peer: peer.clone(),
                        reason: progress::peer_failure_reason_for_error(&error),
                        error,
                        attempts,
                        maybe_progress: None,
                    }));
                }
            }
        }
    }

    pub(super) fn send_all<S: SyncPeerSession>(
        &mut self,
        session: &mut S,
        messages: &[WireNetworkMessage],
    ) -> Result<(), SyncRuntimeError> {
        for message in messages {
            session.send(message, self.config.network.magic())?;
            self.network.acknowledge_wire_message_written(message)?;
        }
        Ok(())
    }

    pub(super) fn send_all_for_peer<S: SyncPeerSession>(
        &mut self,
        session: &mut S,
        peer_id: PeerId,
        messages: &[WireNetworkMessage],
    ) -> Result<(), SyncRuntimeError> {
        self.send_all(session, messages)?;
        let emissions = self.announcement_outboxes.take_peer_emissions(peer_id)?;
        for emission in emissions {
            let (target_peer_id, message, capability) = emission.into_parts();
            if target_peer_id != peer_id {
                return Err(SyncRuntimeError::Network {
                    message: "announcement outbox target does not match connected session"
                        .to_string(),
                });
            }
            session.send(&message, self.config.network.magic())?;
            self.network
                .complete_peer_emission(capability.acknowledge_write())?;
        }
        Ok(())
    }

    pub(super) fn peer_handshake_complete(&self, peer_id: open_bitcoin_network::PeerId) -> bool {
        self.network
            .peer_manager_snapshot()
            .ok()
            .as_ref()
            .and_then(|peer_manager| peer_manager.peer_state(peer_id))
            .is_some_and(|peer| {
                peer.local_version_sent
                    && peer.remote_version_received
                    && peer.local_verack_sent
                    && peer.remote_verack_received
            })
    }

    fn peer_has_pending_download_work(&self, peer_id: PeerId) -> bool {
        let compact_download_in_flight = self
            .network
            .peer_manager_snapshot()
            .ok()
            .and_then(|peer_manager| peer_manager.compact_download_peer_state(peer_id).cloned())
            .is_some_and(|state| !state.in_flight.is_empty());
        let full_block_response_pending =
            self.network
                .peer_requested_blocks(peer_id)
                .is_ok_and(|blocks| {
                    blocks
                        .iter()
                        .any(|block_hash| self.inflight_blocks.contains(block_hash))
                });
        compact_download_in_flight || full_block_response_pending
    }

    pub(super) fn complete_peer_session_progress(
        &self,
        progress: &mut PeerProgress,
        peer_id: PeerId,
    ) {
        progress.maybe_capabilities = self.peer_capabilities(peer_id);
        if !self.peer_handshake_complete(peer_id) {
            progress.state = super::PeerSyncState::Stalled;
            progress.maybe_failure_reason = Some(super::PeerFailureReason::Stall);
        }
    }
}
