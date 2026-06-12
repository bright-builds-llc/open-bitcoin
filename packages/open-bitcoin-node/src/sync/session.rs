// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net.cpp
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/headerssync.cpp
// - packages/bitcoin-knots/src/sync.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp

use open_bitcoin_network::WireNetworkMessage;

use super::{DurableSyncRuntime, SyncPeerSession, SyncRuntimeError};

impl DurableSyncRuntime {
    pub(super) fn send_all<S: SyncPeerSession>(
        &self,
        session: &mut S,
        messages: &[WireNetworkMessage],
    ) -> Result<(), SyncRuntimeError> {
        for message in messages {
            session.send(message, self.config.network.magic())?;
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
