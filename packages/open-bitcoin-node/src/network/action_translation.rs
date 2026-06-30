// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/node/txdownloadman_impl.cpp
// - packages/bitcoin-knots/src/node/txdownloadman.h
// - packages/bitcoin-knots/src/protocol.h

use open_bitcoin_network::{
    InventoryList, PeerAction, PeerId, TxDownloadAction, WireNetworkMessage,
};

use crate::ChainstateStore;

use super::{ManagedNetworkError, ManagedPeerNetwork, ManagedResult};

pub(super) fn process_transaction_relay_action(
    action: TxDownloadAction,
) -> Option<(PeerId, WireNetworkMessage)> {
    action.maybe_request_inventory().map(|inventory| {
        (
            action.peer_id(),
            WireNetworkMessage::GetData(InventoryList::new(vec![inventory])),
        )
    })
}

impl<S: ChainstateStore> ManagedPeerNetwork<S> {
    pub fn disconnect_peer(&mut self, peer_id: PeerId) -> Result<(), ManagedNetworkError> {
        self.peer_manager
            .remove_peer_with_transaction_cleanup(peer_id, 0)?;
        self.known_peers.remove(&peer_id);
        Ok(())
    }

    pub fn disconnect_peer_with_transaction_cleanup(
        &mut self,
        peer_id: PeerId,
        now_unix_seconds: i64,
    ) -> ManagedResult<Vec<(PeerId, WireNetworkMessage)>> {
        let actions = self
            .peer_manager
            .remove_peer_with_transaction_cleanup(peer_id, now_unix_seconds)?;
        self.known_peers.remove(&peer_id);
        Ok(transaction_relay_targeted_messages(actions))
    }

    pub fn expire_transaction_requests(
        &mut self,
        now_unix_seconds: i64,
    ) -> ManagedResult<Vec<(PeerId, WireNetworkMessage)>> {
        Ok(self
            .peer_manager
            .expire_transaction_requests(now_unix_seconds)
            .into_iter()
            .filter_map(|(_peer_id, action)| match action {
                PeerAction::TransactionRelay(action) => process_transaction_relay_action(action),
                _ => None,
            })
            .collect())
    }

    pub(super) fn collect_outbound(
        &mut self,
        actions: Vec<PeerAction>,
    ) -> Result<Vec<WireNetworkMessage>, ManagedNetworkError> {
        Ok(actions
            .into_iter()
            .filter_map(|action| match action {
                PeerAction::Send(message) => Some(message),
                _ => None,
            })
            .collect())
    }
}

fn transaction_relay_targeted_messages(
    actions: Vec<PeerAction>,
) -> Vec<(PeerId, WireNetworkMessage)> {
    actions
        .into_iter()
        .filter_map(|action| match action {
            PeerAction::TransactionRelay(action) => process_transaction_relay_action(action),
            _ => None,
        })
        .collect()
}
