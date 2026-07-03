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

use open_bitcoin_network::TxFanoutAction;

use super::ManagedRelayFanoutActionInfo;

impl From<&TxFanoutAction> for ManagedRelayFanoutActionInfo {
    fn from(action: &TxFanoutAction) -> Self {
        Self {
            label: action.as_str(),
            reason: fanout_action_reason(action),
        }
    }
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
