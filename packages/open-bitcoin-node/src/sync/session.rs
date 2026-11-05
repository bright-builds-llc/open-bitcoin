// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net.cpp
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/headerssync.cpp
// - packages/bitcoin-knots/src/sync.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp

use open_bitcoin_core::consensus::block_hash;
use open_bitcoin_network::{PeerId, WireNetworkMessage};

use super::{
    BlockConnectDisposition, DurableSyncRuntime, PeerFailureReason, PeerSyncState,
    ResolvedSyncPeerAddress, SyncPeerReceiveOutcome, SyncPeerSession, SyncRuntimeError,
    SyncTransport, block_reconcile,
    progress::{self, PeerFailure, PeerProgress},
    tip,
};

mod emission_terminal;
mod outbox;
pub use outbox::{AnnouncementOutboxNotification, AnnouncementOutboxRegistry};
pub(super) mod filter_serving;
use block_reconcile::PeerRequests;

struct PeerTurn<'turn> {
    peer_id: PeerId,
    progress: &'turn mut PeerProgress,
    requests: &'turn mut PeerRequests,
    registration: &'turn AnnouncementOutboxNotification,
    maybe_failure_reason_override: &'turn mut Option<PeerFailureReason>,
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
        let mut maybe_outbox_registration = None;
        let mut maybe_requests = None;
        let result = match self.announcement_outboxes.register_peer(peer_id) {
            Err(error) => Err(error),
            Ok(registration) => {
                let result = (|| -> Result<(), SyncRuntimeError> {
                    let (token, outbound) = self.network.connect_sync_peer(peer_id, timestamp)?;
                    let mut requests = PeerRequests::new(token);
                    let active_result = (|| {
                        let outbound = outbound?;
                        let mut turn = PeerTurn {
                            peer_id,
                            progress: &mut progress,
                            requests: &mut requests,
                            registration: &registration,
                            maybe_failure_reason_override: &mut maybe_failure_reason_override,
                        };
                        self.run_peer_messages(&mut session, &mut turn, outbound, controls)
                    })();
                    maybe_requests = Some(requests);
                    active_result
                })();
                maybe_outbox_registration = Some(registration);
                result
            }
        };
        let outbox_cleanup_result = match maybe_outbox_registration.as_ref() {
            Some(registration) => self.announcement_outboxes.finish_registration(
                &self.network,
                peer_id,
                registration,
                maybe_requests.as_ref(),
            ),
            None => Ok(()),
        };
        let (result, outbox_cleanup_result) =
            emission_terminal::surface_abort_cleanup_error(result, outbox_cleanup_result);
        let disconnect_result = match maybe_requests.as_mut() {
            Some(requests) => requests.finish(self),
            None => Ok(()),
        };
        let result = request_cleanup_result(
            request_cleanup_result(result, outbox_cleanup_result),
            disconnect_result,
        );
        match result {
            Ok(()) => Ok(progress),
            Err(error) => {
                if progress.maybe_capabilities.is_none() {
                    progress.maybe_capabilities =
                        maybe_requests.as_ref().and_then(PeerRequests::capabilities);
                }
                Err(Box::new(PeerFailure {
                    peer: peer.clone(),
                    reason: maybe_failure_reason_override
                        .unwrap_or_else(|| progress::peer_failure_reason_for_error(&error)),
                    error,
                    attempts,
                    maybe_progress: Some(progress),
                }))
            }
        }
    }

    fn run_peer_messages<S: SyncPeerSession, C: FnMut() -> i64, K: FnMut() -> bool>(
        &mut self,
        session: &mut S,
        turn: &mut PeerTurn<'_>,
        mut outbound: Vec<WireNetworkMessage>,
        controls: (&mut C, &mut K),
    ) -> Result<(), SyncRuntimeError> {
        outbound.extend(turn.requests.request_missing(self)?);
        if !turn.requests.renew(self, &outbound)? {
            return turn.requests.complete(self, turn.progress);
        }
        if !self.send_all_for_registered_peer_with_cancel(
            session,
            turn.peer_id,
            &outbound,
            controls.1,
            turn.registration,
            turn.requests,
        )? {
            return turn.requests.complete(self, turn.progress);
        }
        let mut messages_received = 0_usize;
        while messages_received < self.config.max_messages_per_peer {
            if (controls.1)() {
                return turn.requests.complete(self, turn.progress);
            }
            let received = match session.receive(self.config.network.magic()) {
                Ok(received) => received,
                Err(error) => {
                    let reason = progress::peer_failure_reason_for_error(&error);
                    if reason == PeerFailureReason::MalformedBlock {
                        turn.progress.record_malformed_block();
                        *turn.maybe_failure_reason_override = Some(reason);
                    }
                    return Err(error);
                }
            };
            match received {
                SyncPeerReceiveOutcome::Message(message) => {
                    let timestamp = (controls.0)();
                    messages_received = messages_received.saturating_add(1);
                    if !self.process_peer_message(session, turn, message, timestamp, controls.1)? {
                        return turn.requests.complete(self, turn.progress);
                    }
                }
                SyncPeerReceiveOutcome::Idle => {
                    let timestamp = (controls.0)();
                    if (controls.1)() {
                        return turn.requests.complete(self, turn.progress);
                    }
                    let outbound = turn.requests.expire(self, timestamp)?;
                    if !turn.requests.renew(self, &outbound)? {
                        return turn.requests.complete(self, turn.progress);
                    }
                    if !self.send_all_for_registered_peer_with_cancel(
                        session,
                        turn.peer_id,
                        &outbound,
                        controls.1,
                        turn.registration,
                        turn.requests,
                    )? {
                        return turn.requests.complete(self, turn.progress);
                    }
                    if !turn.requests.pending(self) {
                        return turn.requests.complete(self, turn.progress);
                    }
                }
                SyncPeerReceiveOutcome::Closed => {
                    return turn.requests.complete(self, turn.progress);
                }
            }
        }
        turn.progress.state = PeerSyncState::Connected;
        turn.requests.observe_capabilities(self, turn.progress)
    }

    fn process_peer_message<S: SyncPeerSession, K: FnMut() -> bool>(
        &mut self,
        session: &mut S,
        turn: &mut PeerTurn<'_>,
        message: WireNetworkMessage,
        current_timestamp: i64,
        should_cancel: &mut K,
    ) -> Result<bool, SyncRuntimeError> {
        let PeerTurn {
            peer_id,
            progress,
            requests,
            maybe_failure_reason_override,
            registration,
        } = turn;
        let peer_id = *peer_id;
        if !requests.renew(self, &[])? {
            return Ok(false);
        }
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
        let maybe_response_hash = super::block_response::maybe_block_response_hash(&message);
        let block_response_was_requested = match maybe_response_hash {
            Some(block_hash) => requests.block_was_requested(self, block_hash)?,
            None => false,
        };
        let block_response_is_best_chain = match maybe_block.as_ref() {
            Some(block) => {
                self.block_has_best_chain_header(block_hash(&block.header))?
                    || self.block_extends_active_tip(block)?
            }
            None => false,
        };
        let notfound_was_requested = requests.notfound_was_requested(self, &message)?;
        let receipt_hashes =
            super::block_response::inflight_receipt_hashes(&message, &self.inflight_blocks);

        if let Some(block) = maybe_block.as_ref()
            && !block_response_was_requested
        {
            requests.record_ignored(self, current_timestamp, progress, &receipt_hashes)?;
            self.record_unrequested_block_response(progress, block, block_response_is_best_chain)?;
            let outbound = requests.request_missing(self)?;
            if !requests.renew(self, &outbound)? {
                return Ok(false);
            }
            return self.send_all_for_registered_peer_with_cancel(
                session,
                peer_id,
                &outbound,
                should_cancel,
                registration,
                requests,
            );
        }

        let received =
            requests.dispatch(self, message, current_timestamp, &receipt_hashes, progress)?;
        let sync_result = match received {
            Ok(result) => result,
            Err(error) => {
                if maybe_block_hash.is_some() {
                    progress.record_invalid_block();
                    **maybe_failure_reason_override = Some(PeerFailureReason::InvalidBlock);
                }
                return Err(error);
            }
        };
        let mut outbound = requests.handoff(
            self,
            sync_result
                .outbound
                .into_iter()
                .map(|message| (peer_id, message))
                .collect(),
        )?;
        if !requests.renew(self, &outbound)? {
            return Ok(false);
        }
        filter_serving::send_filter_plans(
            &self.network,
            session,
            peer_id,
            self.config.network.magic(),
            sync_result.inbound_response_plan,
            should_cancel,
        )?;
        if let Some(header_count) = maybe_header_count {
            progress.record_validated_headers(header_count);
            let peer_manager = self.network.peer_manager_snapshot()?;
            tip::record_peer_terminal_tip(
                progress,
                peer_manager.header_store(),
                maybe_terminal_header_hash,
            );
        }
        if notfound_was_requested {
            progress.record_block_notfound();
        }
        let mut should_persist_progress = maybe_header_count.is_some_and(|count| count > 0);
        if let Some(disposition) = sync_result.maybe_block_disposition {
            should_persist_progress = matches!(disposition, BlockConnectDisposition::Connected(_));
            self.record_block_disposition(
                progress,
                sync_result
                    .maybe_completed_block
                    .as_ref()
                    .or(maybe_block.as_ref()),
                disposition,
                block_response_was_requested,
                block_response_is_best_chain,
            )?;
        }
        let reconcile_progress =
            block_reconcile::reconcile_best_chain_for_live_session(self, current_timestamp)?;
        should_persist_progress |= reconcile_progress.should_persist_progress();
        self.record_reconcile_progress(reconcile_progress);
        if should_persist_progress {
            self.persist_progress_and_dispatch_tip()?;
        }
        outbound.extend(requests.request_missing(self)?);
        if !requests.renew(self, &outbound)? {
            return Ok(false);
        }
        self.send_all_for_registered_peer_with_cancel(
            session,
            peer_id,
            &outbound,
            should_cancel,
            registration,
            requests,
        )
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
}

pub(super) fn request_cleanup_result<T>(
    result: Result<T, SyncRuntimeError>,
    cleanup: Result<(), SyncRuntimeError>,
) -> Result<T, SyncRuntimeError> {
    match (result, cleanup) {
        (result, Ok(())) => result,
        (Ok(_), Err(error)) => Err(error),
        (Err(error), Err(cleanup)) => Err(SyncRuntimeError::Network {
            message: format!("{error}; request accounting cleanup also failed: {cleanup}"),
        }),
    }
}

#[cfg(test)]
mod owned_queue_tests {
    use super::*;
    use crate::network::AnnouncementPreparationOutcome;
    use crate::sync::{SyncNetwork, SyncPeerAddress, SyncRuntimeConfig, TcpPeerTransport};
    use crate::{FjallNodeStore, chainstate::BasicFilterStartupMode};
    use open_bitcoin_core::consensus::CanonicalChain;
    use open_bitcoin_network::VersionMessage;
    use std::{io::Read, net::TcpListener, sync::atomic::Ordering};

    #[test]
    fn phase167_peer_session_active_absence_keeps_owned_nonempty_queue_for_final_cleanup() {
        // Arrange: genuine public proved provider and real factory/registration.
        let mut nonce = [0; 16];
        getrandom::fill(&mut nonce).expect("owned path");
        let path = std::fs::canonicalize(std::env::temp_dir())
            .expect("physical temp parent")
            .join(format!(
                "phase167-active-absent-{:x}",
                u128::from_le_bytes(nonce)
            ));
        let opened = FjallNodeStore::open_canonical_genesis(&path, CanonicalChain::Regtest)
            .expect("proved genesis");
        let mut runtime = DurableSyncRuntime::open_canonical_genesis_configured(
            opened,
            SyncRuntimeConfig {
                network: SyncNetwork::Regtest,
                ..Default::default()
            },
            BasicFilterStartupMode::Disabled,
        )
        .expect("actual runtime");
        let registration = runtime
            .announcement_outboxes
            .register_peer(1)
            .expect("actual Arc");
        let (token, outbound) = runtime
            .network
            .connect_sync_peer(1, 0)
            .expect("actual A factory");
        outbound.expect("real version");
        let requests = PeerRequests::new(token);
        for message in [
            WireNetworkMessage::Version(VersionMessage::default()),
            WireNetworkMessage::Verack,
        ] {
            runtime
                .network
                .receive_sync_message(
                    1,
                    message,
                    0,
                    runtime.verify_flags,
                    runtime.consensus_params,
                )
                .expect("actual handshake");
        }
        let block = CanonicalChain::Regtest.spec().genesis_block();
        let mut expected = Vec::new();
        for headers in [false, true] {
            if headers {
                runtime
                    .network
                    .receive_sync_message(
                        1,
                        WireNetworkMessage::SendHeaders,
                        0,
                        runtime.verify_flags,
                        runtime.consensus_params,
                    )
                    .expect("actual preference");
            }
            let outcomes = runtime
                .network
                .prepare_block_announcements(
                    &block,
                    &runtime
                        .announcement_outboxes
                        .snapshots()
                        .expect("actual queue"),
                )
                .expect("genuine producer");
            expected.extend(outcomes.iter().filter_map(|outcome| match outcome {
                AnnouncementPreparationOutcome::Ready(emission) => Some(emission.message().clone()),
                _ => None,
            }));
            runtime
                .announcement_outboxes
                .enqueue_prepared(&runtime.network, outcomes)
                .expect("two actual queued capabilities");
        }
        assert_eq!(expected.len(), 2);
        assert_ne!(expected[0], expected[1]);
        let generation = registration.readiness.generation.load(Ordering::Acquire);
        let listener = TcpListener::bind("127.0.0.1:0").expect("owned loopback");
        let endpoint = listener.local_addr().expect("endpoint");
        let peer = ResolvedSyncPeerAddress::new(
            SyncPeerAddress::manual("127.0.0.1", endpoint.port()),
            endpoint,
        );
        let mut session = TcpPeerTransport
            .connect(&peer, &runtime.config)
            .expect("real socket");
        let (mut reader, _) = listener.accept().expect("accepted socket");
        runtime
            .network
            .disconnect_peer(1)
            .expect("A removed without any B admission");
        // Act: absent completion runs no active TAKE, even with a valid final cleanup lease.
        let present = runtime
            .send_all_for_registered_peer_with_cancel(
                &mut session,
                1,
                &[],
                &mut || false,
                &registration,
                &requests,
            )
            .expect("legitimate original absence explicitly completes without queue grant");
        // Assert
        assert!(!present);
        let actual = runtime
            .announcement_outboxes
            .lock_outboxes()
            .expect("actual retained queue")
            .get(&1)
            .expect("same registration")
            .emissions
            .iter()
            .map(|emission| emission.message().clone())
            .collect::<Vec<_>>();
        assert_eq!(actual, expected);
        assert_eq!(
            registration.readiness.generation.load(Ordering::Acquire),
            generation
        );
        runtime
            .announcement_outboxes
            .finish_registration(&runtime.network, 1, &registration, Some(&requests))
            .expect("final earned absent cleanup may abort own nonempty queue");
        assert!(
            runtime
                .announcement_outboxes
                .snapshots()
                .expect("removed")
                .is_empty()
        );
        drop(session);
        let mut wire = Vec::new();
        reader
            .read_to_end(&mut wire)
            .expect("actual physical bytes");
        assert!(wire.is_empty());
        drop(runtime);
        std::fs::remove_dir_all(path).expect("owned cleanup");
    }
}
