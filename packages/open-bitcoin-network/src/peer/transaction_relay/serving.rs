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
// - packages/bitcoin-knots/test/functional/p2p_tx_download.py
// - packages/bitcoin-knots/test/functional/p2p_getdata.py

use open_bitcoin_primitives::InventoryVector;

use crate::RelayEligibilityDecision;

use super::{TxRelayId, TxRelayIdentityError, TxRelayPeerMode};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TxServingRecordStatus {
    Accepted,
    Stale,
    Confirmed,
    Rejected,
    Replaced,
    Evicted,
    Expired,
}

impl TxServingRecordStatus {
    pub const fn outcome_label(self) -> TxServeOutcomeLabel {
        match self {
            Self::Accepted => TxServeOutcomeLabel::Served,
            Self::Stale => TxServeOutcomeLabel::Stale,
            Self::Confirmed => TxServeOutcomeLabel::Confirmed,
            Self::Rejected => TxServeOutcomeLabel::Rejected,
            Self::Replaced => TxServeOutcomeLabel::Replaced,
            Self::Evicted => TxServeOutcomeLabel::Evicted,
            Self::Expired => TxServeOutcomeLabel::Expired,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TxServeOutcomeLabel {
    Served,
    Unknown,
    Stale,
    Confirmed,
    Rejected,
    Replaced,
    Evicted,
    Expired,
    IdentityMismatch,
    NotRelayEligible,
    NotTransactionInventory,
}

impl TxServeOutcomeLabel {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Served => "served",
            Self::Unknown => "unknown",
            Self::Stale => "stale",
            Self::Confirmed => "confirmed",
            Self::Rejected => "rejected",
            Self::Replaced => "replaced",
            Self::Evicted => "evicted",
            Self::Expired => "expired",
            Self::IdentityMismatch => "identity_mismatch",
            Self::NotRelayEligible => "not_relay_eligible",
            Self::NotTransactionInventory => "not_transaction_inventory",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TxServeDecision {
    pub maybe_relay_id: Option<TxRelayId>,
    pub outcome: TxServeOutcomeLabel,
}

pub fn classify_tx_serve_request(
    vector: &InventoryVector,
    peer_mode: TxRelayPeerMode,
    relay_eligibility: &RelayEligibilityDecision,
    maybe_record_status: Option<TxServingRecordStatus>,
) -> TxServeDecision {
    let relay_id = match TxRelayId::from_inventory_vector_for_peer(vector, peer_mode) {
        Ok(relay_id) => relay_id,
        Err(TxRelayIdentityError::NegotiationMismatch { .. }) => {
            return TxServeDecision {
                maybe_relay_id: None,
                outcome: TxServeOutcomeLabel::IdentityMismatch,
            };
        }
        Err(TxRelayIdentityError::NotTransactionInventory { .. }) => {
            return TxServeDecision {
                maybe_relay_id: None,
                outcome: TxServeOutcomeLabel::NotTransactionInventory,
            };
        }
    };

    if !relay_eligibility.eligible {
        return TxServeDecision {
            maybe_relay_id: Some(relay_id),
            outcome: TxServeOutcomeLabel::NotRelayEligible,
        };
    }

    let Some(record_status) = maybe_record_status else {
        return TxServeDecision {
            maybe_relay_id: Some(relay_id),
            outcome: TxServeOutcomeLabel::Unknown,
        };
    };

    TxServeDecision {
        maybe_relay_id: Some(relay_id),
        outcome: record_status.outcome_label(),
    }
}
