// Parity breadcrumbs:
// - packages/bitcoin-knots/src/protocol.h
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/node/txdownloadman.h
// - packages/bitcoin-knots/src/node/txdownloadman_impl.cpp
// - packages/bitcoin-knots/src/txrequest.h
// - packages/bitcoin-knots/src/txrequest.cpp
// - packages/bitcoin-knots/test/functional/p2p_tx_download.py

use open_bitcoin_primitives::{Hash32, InventoryType, InventoryVector, Txid, Wtxid};

use crate::error::PeerId;

pub const PHASE101_MAX_TX_ANNOUNCEMENTS_PER_PEER: usize = 5_000;
pub const PHASE101_MAX_TX_REQUESTS_IN_FLIGHT_PER_PEER: usize = 100;
pub const PHASE101_TXID_RELAY_DELAY_SECONDS: i64 = 2;
pub const PHASE101_NONPREF_PEER_TX_DELAY_SECONDS: i64 = 2;
pub const PHASE101_OVERLOADED_PEER_TX_DELAY_SECONDS: i64 = 2;
pub const PHASE101_GETDATA_TX_INTERVAL_SECONDS: i64 = 60;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TxRelayPeerMode {
    TxidOnly,
    WtxidRelay,
}

impl TxRelayPeerMode {
    pub const fn from_remote_wtxidrelay(remote_wtxidrelay: bool) -> Self {
        if remote_wtxidrelay {
            return Self::WtxidRelay;
        }
        Self::TxidOnly
    }

    pub const fn expected_inventory_type(self) -> InventoryType {
        match self {
            Self::TxidOnly => InventoryType::Transaction,
            Self::WtxidRelay => InventoryType::WitnessTransaction,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TxRelayId {
    Txid(Txid),
    Wtxid(Wtxid),
}

impl TxRelayId {
    pub const fn inventory_type(self) -> InventoryType {
        match self {
            Self::Txid(_) => InventoryType::Transaction,
            Self::Wtxid(_) => InventoryType::WitnessTransaction,
        }
    }

    pub fn object_hash(self) -> Hash32 {
        match self {
            Self::Txid(txid) => txid.into(),
            Self::Wtxid(wtxid) => wtxid.into(),
        }
    }

    pub fn to_inventory_vector(self) -> InventoryVector {
        InventoryVector {
            inventory_type: self.inventory_type(),
            object_hash: self.object_hash(),
        }
    }

    pub fn from_inventory_vector_for_peer(
        vector: &InventoryVector,
        mode: TxRelayPeerMode,
    ) -> Result<Self, TxRelayIdentityError> {
        match vector.inventory_type {
            InventoryType::Transaction if mode == TxRelayPeerMode::TxidOnly => {
                Ok(Self::Txid(Txid::from(vector.object_hash)))
            }
            InventoryType::WitnessTransaction if mode == TxRelayPeerMode::WtxidRelay => {
                Ok(Self::Wtxid(Wtxid::from(vector.object_hash)))
            }
            InventoryType::Transaction | InventoryType::WitnessTransaction => {
                Err(TxRelayIdentityError::NegotiationMismatch {
                    inventory_type: vector.inventory_type,
                    peer_mode: mode,
                })
            }
            inventory_type => Err(TxRelayIdentityError::NotTransactionInventory { inventory_type }),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TxRelayIdentityError {
    NotTransactionInventory {
        inventory_type: InventoryType,
    },
    NegotiationMismatch {
        inventory_type: InventoryType,
        peer_mode: TxRelayPeerMode,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TxDownloadSuppressionReason {
    Duplicate,
    AlreadyHave,
    RecentReject,
    InFlight,
    RequestCapReached,
    IdentityMismatch,
    NotTransactionInventory,
    MempoolKnown,
}

impl TxDownloadSuppressionReason {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Duplicate => "duplicate",
            Self::AlreadyHave => "already_have",
            Self::RecentReject => "recent_reject",
            Self::InFlight => "in_flight",
            Self::RequestCapReached => "request_cap_reached",
            Self::IdentityMismatch => "identity_mismatch",
            Self::NotTransactionInventory => "not_transaction_inventory",
            Self::MempoolKnown => "mempool_known",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TxDownloadAction {
    RequestGetData {
        peer_id: PeerId,
        relay_id: TxRelayId,
    },
    SuppressDuplicate {
        peer_id: PeerId,
        relay_id: TxRelayId,
    },
    SuppressAlreadyHave {
        peer_id: PeerId,
        relay_id: TxRelayId,
    },
    SuppressRecentReject {
        peer_id: PeerId,
        relay_id: TxRelayId,
    },
    Suppress {
        peer_id: PeerId,
        relay_id: TxRelayId,
        reason: TxDownloadSuppressionReason,
    },
    SuppressIdentityMismatch {
        peer_id: PeerId,
        reason: TxDownloadSuppressionReason,
    },
    SuppressRequestCap {
        peer_id: PeerId,
        relay_id: TxRelayId,
    },
    FallbackRequest {
        peer_id: PeerId,
        relay_id: TxRelayId,
    },
    RequestExpired {
        peer_id: PeerId,
        relay_id: TxRelayId,
    },
    NotFoundCleanup {
        peer_id: PeerId,
        relay_id: TxRelayId,
    },
    ReceivedTxCleanup {
        peer_id: PeerId,
        txid: Txid,
        wtxid: Wtxid,
    },
    PeerCleanup {
        peer_id: PeerId,
    },
}

impl TxDownloadAction {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::RequestGetData { .. } => "request_getdata",
            Self::SuppressDuplicate { .. } => "suppress_duplicate",
            Self::SuppressAlreadyHave { .. } => "suppress_already_have",
            Self::SuppressRecentReject { .. } => "suppress_recent_reject",
            Self::Suppress {
                reason: TxDownloadSuppressionReason::MempoolKnown,
                ..
            } => "suppress_mempool_known",
            Self::Suppress {
                reason: TxDownloadSuppressionReason::RequestCapReached,
                ..
            }
            | Self::SuppressRequestCap { .. } => "suppress_request_cap",
            Self::Suppress {
                reason: TxDownloadSuppressionReason::AlreadyHave,
                ..
            } => "suppress_already_have",
            Self::Suppress {
                reason: TxDownloadSuppressionReason::RecentReject,
                ..
            } => "suppress_recent_reject",
            Self::Suppress {
                reason:
                    TxDownloadSuppressionReason::Duplicate | TxDownloadSuppressionReason::InFlight,
                ..
            } => "suppress_duplicate",
            Self::Suppress {
                reason:
                    TxDownloadSuppressionReason::IdentityMismatch
                    | TxDownloadSuppressionReason::NotTransactionInventory,
                ..
            }
            | Self::SuppressIdentityMismatch { .. } => "suppress_identity_mismatch",
            Self::FallbackRequest { .. } => "fallback_request",
            Self::RequestExpired { .. } => "request_expired",
            Self::NotFoundCleanup { .. } => "notfound_cleanup",
            Self::ReceivedTxCleanup { .. } => "received_tx_cleanup",
            Self::PeerCleanup { .. } => "peer_cleanup",
        }
    }

    pub const fn suppression_reason(&self) -> Option<TxDownloadSuppressionReason> {
        match self {
            Self::SuppressDuplicate { .. } => Some(TxDownloadSuppressionReason::Duplicate),
            Self::SuppressAlreadyHave { .. } => Some(TxDownloadSuppressionReason::AlreadyHave),
            Self::SuppressRecentReject { .. } => Some(TxDownloadSuppressionReason::RecentReject),
            Self::Suppress { reason, .. } | Self::SuppressIdentityMismatch { reason, .. } => {
                Some(*reason)
            }
            Self::SuppressRequestCap { .. } => Some(TxDownloadSuppressionReason::RequestCapReached),
            Self::RequestGetData { .. }
            | Self::FallbackRequest { .. }
            | Self::RequestExpired { .. }
            | Self::NotFoundCleanup { .. }
            | Self::ReceivedTxCleanup { .. }
            | Self::PeerCleanup { .. } => None,
        }
    }

    pub fn maybe_request_inventory(&self) -> Option<InventoryVector> {
        match self {
            Self::RequestGetData { relay_id, .. } | Self::FallbackRequest { relay_id, .. } => {
                Some(relay_id.to_inventory_vector())
            }
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TxDownloadPolicy {
    pub max_announcements_per_peer: usize,
    pub max_in_flight_per_peer: usize,
    pub txid_relay_delay_seconds: i64,
    pub non_preferred_peer_delay_seconds: i64,
    pub overloaded_peer_delay_seconds: i64,
    pub getdata_tx_interval_seconds: i64,
}

impl Default for TxDownloadPolicy {
    fn default() -> Self {
        Self {
            max_announcements_per_peer: PHASE101_MAX_TX_ANNOUNCEMENTS_PER_PEER,
            max_in_flight_per_peer: PHASE101_MAX_TX_REQUESTS_IN_FLIGHT_PER_PEER,
            txid_relay_delay_seconds: PHASE101_TXID_RELAY_DELAY_SECONDS,
            non_preferred_peer_delay_seconds: PHASE101_NONPREF_PEER_TX_DELAY_SECONDS,
            overloaded_peer_delay_seconds: PHASE101_OVERLOADED_PEER_TX_DELAY_SECONDS,
            getdata_tx_interval_seconds: PHASE101_GETDATA_TX_INTERVAL_SECONDS,
        }
    }
}

#[cfg(test)]
mod tests;
