// Parity breadcrumbs:
// - packages/bitcoin-knots/src/headerssync.cpp
// - packages/bitcoin-knots/src/sync.cpp
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/node/txdownloadman_impl.cpp
// - packages/bitcoin-knots/src/node/txdownloadman.h
// - packages/bitcoin-knots/test/functional/p2p_handshake.py
// - packages/bitcoin-knots/test/functional/p2p_initial_headers_sync.py

use open_bitcoin_primitives::{BlockHash, Hash32, InventoryType, Txid, Wtxid};

use super::PeerState;

pub(super) fn forget_requested_inventory(
    peer: &mut PeerState,
    inventory_type: InventoryType,
    object_hash: Hash32,
) {
    match inventory_type {
        InventoryType::Block | InventoryType::WitnessBlock => {
            peer.requested_blocks.remove(&BlockHash::from(object_hash));
        }
        InventoryType::Transaction => {
            peer.requested_txids.remove(&Txid::from(object_hash));
        }
        InventoryType::WitnessTransaction => {
            peer.requested_wtxids.remove(&Wtxid::from(object_hash));
        }
        _ => {}
    }
}
