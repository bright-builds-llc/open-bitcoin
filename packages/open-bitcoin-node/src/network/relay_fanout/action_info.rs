// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/node/txdownloadman_impl.cpp
// - packages/bitcoin-knots/src/node/txdownloadman.h
// - packages/bitcoin-knots/src/node/mempool_persist.cpp
// - packages/bitcoin-knots/src/protocol.h
// - packages/bitcoin-knots/src/txorphanage.cpp
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/test/functional/mempool_persist.py
// - packages/bitcoin-knots/test/functional/p2p_getdata.py
// - packages/bitcoin-knots/test/functional/p2p_orphan_handling.py
// - packages/bitcoin-knots/test/functional/p2p_tx_download.py
// - packages/bitcoin-knots/test/functional/mempool_accept.py

use open_bitcoin_network::{InventoryList, PeerId, TxFanoutAction, WireNetworkMessage};

use super::ManagedRelayFanoutActionInfo;
use crate::status::relay_evidence::{
    RelayCapabilityEvidence, RelayEvidenceCapability, RelayEvidenceField,
};

pub(super) fn implemented_capability(
    capability: RelayEvidenceCapability,
) -> RelayEvidenceField<RelayCapabilityEvidence> {
    RelayEvidenceField::implemented(RelayCapabilityEvidence::new(capability))
}

impl From<&TxFanoutAction> for ManagedRelayFanoutActionInfo {
    fn from(action: &TxFanoutAction) -> Self {
        Self {
            label: action.as_str(),
            reason: fanout_action_reason(action),
        }
    }
}

pub(super) fn translate_fanout_action(
    action: TxFanoutAction,
) -> Option<(PeerId, WireNetworkMessage)> {
    let TxFanoutAction::Announce { peer_id, relay_id } = action else {
        return None;
    };
    Some((
        peer_id,
        WireNetworkMessage::Inv(InventoryList::new(vec![relay_id.to_inventory_vector()])),
    ))
}

fn fanout_action_reason(action: &TxFanoutAction) -> Option<&'static str> {
    match action {
        TxFanoutAction::Suppress { reason, .. } => Some(reason.as_str()),
        TxFanoutAction::Cleanup { reason, .. } => Some(reason.as_str()),
        TxFanoutAction::QueueCap { .. } => Some("queue_cap_reached"),
        TxFanoutAction::RateLimit { .. } => Some("rate_limited"),
        TxFanoutAction::Announce { .. } | TxFanoutAction::RebroadcastDeferred { .. } => None,
    }
}
