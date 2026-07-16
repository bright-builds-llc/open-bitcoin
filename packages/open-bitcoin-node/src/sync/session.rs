// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net.cpp
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/headerssync.cpp
// - packages/bitcoin-knots/src/sync.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp

use open_bitcoin_core::{
    consensus::block_hash,
    primitives::{BlockHash, InventoryType},
};
use open_bitcoin_network::{PeerId, WireNetworkMessage};

use super::{
    BlockConnectDisposition, DurableSyncRuntime, PeerFailureReason, PeerSyncState,
    ResolvedSyncPeerAddress, SyncPeerReceiveOutcome, SyncPeerSession, SyncRuntimeError,
    SyncTransport, block_reconcile,
    progress::{self, PeerFailure, PeerProgress},
    tip,
};

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
        let result = (|| -> Result<(), SyncRuntimeError> {
            let mut outbound = self.network.connect_outbound_peer(peer_id, timestamp)?;
            outbound.extend(block_reconcile::request_missing_blocks(self, peer_id)?);
            self.send_all(&mut session, &outbound)?;

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
                        self.send_all(&mut session, &outbound)?;
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
                let block_response_was_requested = maybe_block_hash
                    .as_ref()
                    .is_some_and(|hash| self.peer_requested_block(peer_id, *hash));
                let block_response_is_best_chain = maybe_block.as_ref().is_some_and(|block| {
                    self.block_has_best_chain_header(block_hash(&block.header))
                        || self.block_extends_active_tip(block)
                });
                let notfound_was_requested =
                    self.message_reports_requested_block_notfound(peer_id, &message);
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
                    self.send_all(&mut session, &outbound)?;
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
                    tip::record_peer_terminal_tip(
                        &mut progress,
                        self.network.peer_manager().header_store(),
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
                let reconcile_progress =
                    block_reconcile::reconcile_best_chain(self, current_timestamp)?;
                should_persist_progress |= reconcile_progress.should_persist_progress();
                self.record_reconcile_progress(reconcile_progress);
                if should_persist_progress {
                    self.persist_progress()?;
                }
                outbound.extend(block_reconcile::request_missing_blocks(self, peer_id)?);
                self.send_all(&mut session, &outbound)?;
            }
            progress.state = PeerSyncState::Connected;
            progress.maybe_capabilities = self.peer_capabilities(peer_id);

            Ok(())
        })();
        let outstanding_blocks = self
            .network
            .peer_requested_blocks(peer_id)
            .unwrap_or_default();
        for block_hash in outstanding_blocks {
            self.inflight_blocks.remove(&block_hash);
        }
        let disconnect_result = self.network.disconnect_peer(peer_id);
        match (result, disconnect_result) {
            (Ok(()), Ok(())) => Ok(progress),
            (Ok(()), Err(error)) => {
                let error = SyncRuntimeError::from(error);
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
            (Err(error), _) => {
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
            self.network.acknowledge_wire_message_written(message);
        }
        Ok(())
    }

    pub(super) fn peer_handshake_complete(&self, peer_id: open_bitcoin_network::PeerId) -> bool {
        self.network
            .peer_manager()
            .peer_state(peer_id)
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
            .peer_manager()
            .compact_download_peer_state(peer_id)
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
