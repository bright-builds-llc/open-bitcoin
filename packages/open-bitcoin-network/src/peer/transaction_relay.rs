// Parity breadcrumbs:
// - packages/bitcoin-knots/src/protocol.h
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/node/txdownloadman.h
// - packages/bitcoin-knots/src/node/txdownloadman_impl.cpp
// - packages/bitcoin-knots/src/txorphanage.h
// - packages/bitcoin-knots/src/txorphanage.cpp
// - packages/bitcoin-knots/src/txrequest.h
// - packages/bitcoin-knots/src/txrequest.cpp
// - packages/bitcoin-knots/test/functional/p2p_orphan_handling.py
// - packages/bitcoin-knots/test/functional/p2p_opportunistic_1p1c.py
// - packages/bitcoin-knots/test/functional/p2p_tx_download.py
// - packages/bitcoin-knots/test/functional/p2p_getdata.py

use open_bitcoin_primitives::{Hash32, InventoryType, InventoryVector, Txid, Wtxid};

use crate::error::PeerId;
use crate::{RelayEligibilityDecision, RelayEligibilityReason};

mod fanout;
mod reject_evidence;
mod retry;
mod scheduler;
mod serving;

pub mod orphanage;

pub use fanout::{
    PHASE104_MAX_TX_FANOUT_DRAIN_PER_PEER, PHASE104_MAX_TX_FANOUT_QUEUE_PER_PEER,
    PHASE104_TX_FANOUT_MIN_INTERVAL_SECONDS, TxFanoutAction, TxFanoutAdmission,
    TxFanoutAdmissionOutcome, TxFanoutCleanupReason, TxFanoutPeerInput, TxFanoutPolicy,
    TxFanoutQueue, TxFanoutSnapshot, TxFanoutSuppressionReason, defer_local_rebroadcast,
};
pub use orphanage::{
    BoundedOrphanAnnouncers, OrphanAction, OrphanEvidenceLabel, OrphanPolicy,
    OrphanReconsiderationCandidate, OrphanReconsiderationStatus, OrphanStageInput,
    PHASE102_MAX_ORPHAN_TRANSACTIONS, PHASE102_MAX_ORPHANS_PER_PEER,
    PHASE102_MAX_RECONSIDERATIONS_PER_PARENT, PHASE102_ORPHAN_TTL_SECONDS,
    PHASE133_MAX_ANNOUNCERS_PER_ORPHAN, PHASE133_MAX_ORPHAN_RETAINED_BYTES,
    SamePeerOneParentOneChildCandidate, TxOrphanage,
};
pub use reject_evidence::{
    HardRejectEvidence, PHASE133_REJECT_FILTER_CAPACITY,
    PHASE133_REJECT_FILTER_FALSE_POSITIVE_RATE, ReconsiderableEvidenceKey,
    ReconsiderableRejectEvidence, RejectEvidenceConfigError, RejectEvidenceTweak,
};
pub use retry::{RetryDecisionContext, RetryJitterRangeError, RetryJitterSeconds};
pub use scheduler::{
    TxAnnouncementInput, TxDownloadLocalFacts, TxDownloadScheduler, TxDownloadSnapshot,
    TxParentRequestInput, TxPeerRequestSnapshot,
};
pub use serving::{
    TxServeDecision, TxServeOutcomeLabel, TxServingRecordStatus, classify_tx_serve_request,
};

pub const PHASE101_MAX_TX_ANNOUNCEMENTS_PER_PEER: usize = 5_000;
pub const PHASE101_MAX_TX_REQUESTS_IN_FLIGHT_PER_PEER: usize = 100;
pub const PHASE101_TXID_RELAY_DELAY_SECONDS: i64 = 2;
pub const PHASE101_NONPREF_PEER_TX_DELAY_SECONDS: i64 = 2;
pub const PHASE101_OVERLOADED_PEER_TX_DELAY_SECONDS: i64 = 2;
pub const PHASE101_GETDATA_TX_INTERVAL_SECONDS: i64 = 60;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReceivedTransactionProvenance {
    pub delivered_by: PeerId,
    pub announcers: Vec<PeerId>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReceivedTransactionResult {
    pub actions: Vec<TxDownloadAction>,
    pub maybe_provenance: Option<ReceivedTransactionProvenance>,
}

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
    RelayDisabled,
    NotRelayEligible,
    InboundServingRequired,
    PermissionRequired,
    ProtectedNotRelay,
}

fn relay_eligibility_suppression(
    peer_id: PeerId,
    relay_id: TxRelayId,
    relay_eligibility: &RelayEligibilityDecision,
) -> Option<TxDownloadAction> {
    if relay_eligibility.eligible {
        return None;
    }

    Some(TxDownloadAction::Suppress {
        peer_id,
        relay_id,
        reason: match relay_eligibility.reason {
            RelayEligibilityReason::Disabled | RelayEligibilityReason::ActivationRequired => {
                TxDownloadSuppressionReason::RelayDisabled
            }
            RelayEligibilityReason::InboundServingRequired => {
                TxDownloadSuppressionReason::InboundServingRequired
            }
            RelayEligibilityReason::PermissionRequired
            | RelayEligibilityReason::PermissionEffectInactive => {
                TxDownloadSuppressionReason::PermissionRequired
            }
            RelayEligibilityReason::ProtectedNotRelay => {
                TxDownloadSuppressionReason::ProtectedNotRelay
            }
            RelayEligibilityReason::Eligible => TxDownloadSuppressionReason::NotRelayEligible,
        },
    })
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
            Self::RelayDisabled => "relay_disabled",
            Self::NotRelayEligible => "not_relay_eligible",
            Self::InboundServingRequired => "inbound_serving_required",
            Self::PermissionRequired => "permission_required",
            Self::ProtectedNotRelay => "protected_not_relay",
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
    pub const fn peer_id(&self) -> PeerId {
        match self {
            Self::RequestGetData { peer_id, .. }
            | Self::SuppressDuplicate { peer_id, .. }
            | Self::SuppressAlreadyHave { peer_id, .. }
            | Self::SuppressRecentReject { peer_id, .. }
            | Self::Suppress { peer_id, .. }
            | Self::SuppressIdentityMismatch { peer_id, .. }
            | Self::SuppressRequestCap { peer_id, .. }
            | Self::FallbackRequest { peer_id, .. }
            | Self::RequestExpired { peer_id, .. }
            | Self::NotFoundCleanup { peer_id, .. }
            | Self::ReceivedTxCleanup { peer_id, .. }
            | Self::PeerCleanup { peer_id } => *peer_id,
        }
    }

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
                reason: TxDownloadSuppressionReason::RelayDisabled,
                ..
            } => "suppress_relay_disabled",
            Self::Suppress {
                reason: TxDownloadSuppressionReason::NotRelayEligible,
                ..
            } => "suppress_not_relay_eligible",
            Self::Suppress {
                reason: TxDownloadSuppressionReason::InboundServingRequired,
                ..
            } => "suppress_inbound_serving_required",
            Self::Suppress {
                reason: TxDownloadSuppressionReason::PermissionRequired,
                ..
            } => "suppress_permission_required",
            Self::Suppress {
                reason: TxDownloadSuppressionReason::ProtectedNotRelay,
                ..
            } => "suppress_protected_not_relay",
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
