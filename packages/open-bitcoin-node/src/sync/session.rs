// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net.cpp
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/headerssync.cpp
// - packages/bitcoin-knots/src/sync.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp

use open_bitcoin_network::{PeerId, WireNetworkMessage};

use super::{
    DurableSyncRuntime, ResolvedSyncPeerAddress, SyncPeerSession, SyncRuntimeError, SyncTransport,
    progress::{self, PeerFailure, PeerProgress},
};

impl DurableSyncRuntime {
    pub(super) fn sync_peer_with_retries<T: SyncTransport, C: FnMut() -> i64>(
        &mut self,
        transport: &mut T,
        peer: &ResolvedSyncPeerAddress,
        peer_id: PeerId,
        timestamp: i64,
        clock: &mut C,
    ) -> Result<PeerProgress, Box<PeerFailure>> {
        let mut attempts = 0_u8;
        let max_attempts = self.config.max_peer_retries.saturating_add(1);
        loop {
            attempts = attempts.saturating_add(1);
            match transport.connect(peer, &self.config) {
                Ok(session) => {
                    return self
                        .sync_connected_peer(session, peer, peer_id, attempts, timestamp, clock);
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
}
