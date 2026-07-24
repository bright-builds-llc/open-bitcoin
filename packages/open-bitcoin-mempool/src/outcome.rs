// Parity breadcrumbs:
// - packages/bitcoin-knots/src/txmempool.h
// - packages/bitcoin-knots/src/txmempool.cpp
// - packages/bitcoin-knots/src/policy/policy.h
// - packages/bitcoin-knots/src/policy/rbf.cpp
// - packages/bitcoin-knots/src/policy/packages.cpp
// - packages/bitcoin-knots/src/rpc/mempool.cpp
// - packages/bitcoin-knots/doc/policy/packages.md

use open_bitcoin_primitives::{Txid, Wtxid};

use crate::MempoolError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MempoolOutcomeLabel {
    Accepted,
    Rejected,
    Duplicate,
    Replaced,
    Orphaned,
    Evicted,
    Expired,
}

impl MempoolOutcomeLabel {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Accepted => "accepted",
            Self::Rejected => "rejected",
            Self::Duplicate => "duplicate",
            Self::Replaced => "replaced",
            Self::Orphaned => "orphaned",
            Self::Evicted => "evicted",
            Self::Expired => "expired",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MempoolRejectionCategory {
    Validation,
    NonStandard,
    RelayFeeTooLow,
    ConflictNotAllowed,
    ReplacementRejected,
    LimitExceeded,
    InternalInvariant,
}

impl MempoolRejectionCategory {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Validation => "validation",
            Self::NonStandard => "non_standard",
            Self::RelayFeeTooLow => "relay_fee_too_low",
            Self::ConflictNotAllowed => "conflict_not_allowed",
            Self::ReplacementRejected => "replacement_rejected",
            Self::LimitExceeded => "limit_exceeded",
            Self::InternalInvariant => "internal_invariant",
        }
    }

    pub const fn from_error(error: &MempoolError) -> Option<Self> {
        match error {
            MempoolError::DuplicateTransaction { .. }
            | MempoolError::MissingInput { .. }
            | MempoolError::CandidateEvicted { .. } => None,
            MempoolError::Validation { .. } => Some(Self::Validation),
            MempoolError::NonStandard { .. } => Some(Self::NonStandard),
            MempoolError::RelayFeeTooLow { .. } => Some(Self::RelayFeeTooLow),
            MempoolError::ConflictNotAllowed { .. } => Some(Self::ConflictNotAllowed),
            MempoolError::ReplacementRejected { .. } => Some(Self::ReplacementRejected),
            MempoolError::LimitExceeded { .. } => Some(Self::LimitExceeded),
            MempoolError::InternalInvariant { .. } => Some(Self::InternalInvariant),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MempoolOutcome {
    Accepted {
        txid: Txid,
        wtxid: Wtxid,
        evicted: Vec<Txid>,
    },
    Rejected {
        txid: Txid,
        wtxid: Wtxid,
        category: MempoolRejectionCategory,
    },
    Duplicate {
        txid: Txid,
    },
    Replaced {
        txid: Txid,
        wtxid: Wtxid,
        replaced: Vec<Txid>,
        evicted: Vec<Txid>,
    },
    Orphaned {
        txid: Txid,
        wtxid: Wtxid,
        missing_parents: Vec<Txid>,
    },
    Evicted {
        txid: Txid,
        wtxid: Wtxid,
    },
    Expired {
        txid: Txid,
        wtxid: Wtxid,
    },
}

impl MempoolOutcome {
    pub const fn label(&self) -> MempoolOutcomeLabel {
        match self {
            Self::Accepted { .. } => MempoolOutcomeLabel::Accepted,
            Self::Rejected { .. } => MempoolOutcomeLabel::Rejected,
            Self::Duplicate { .. } => MempoolOutcomeLabel::Duplicate,
            Self::Replaced { .. } => MempoolOutcomeLabel::Replaced,
            Self::Orphaned { .. } => MempoolOutcomeLabel::Orphaned,
            Self::Evicted { .. } => MempoolOutcomeLabel::Evicted,
            Self::Expired { .. } => MempoolOutcomeLabel::Expired,
        }
    }

    pub const fn txid(&self) -> Txid {
        match self {
            Self::Accepted { txid, .. }
            | Self::Rejected { txid, .. }
            | Self::Duplicate { txid }
            | Self::Replaced { txid, .. }
            | Self::Orphaned { txid, .. }
            | Self::Evicted { txid, .. }
            | Self::Expired { txid, .. } => *txid,
        }
    }

    pub const fn maybe_wtxid(&self) -> Option<Wtxid> {
        match self {
            Self::Accepted { wtxid, .. }
            | Self::Rejected { wtxid, .. }
            | Self::Replaced { wtxid, .. }
            | Self::Orphaned { wtxid, .. }
            | Self::Evicted { wtxid, .. }
            | Self::Expired { wtxid, .. } => Some(*wtxid),
            Self::Duplicate { .. } => None,
        }
    }

    pub fn missing_parents(&self) -> &[Txid] {
        match self {
            Self::Orphaned {
                missing_parents, ..
            } => missing_parents,
            Self::Accepted { .. }
            | Self::Rejected { .. }
            | Self::Duplicate { .. }
            | Self::Replaced { .. }
            | Self::Evicted { .. }
            | Self::Expired { .. } => &[],
        }
    }

    pub fn replaced(&self) -> &[Txid] {
        match self {
            Self::Replaced { replaced, .. } => replaced,
            Self::Accepted { .. }
            | Self::Rejected { .. }
            | Self::Duplicate { .. }
            | Self::Orphaned { .. }
            | Self::Evicted { .. }
            | Self::Expired { .. } => &[],
        }
    }

    pub fn evicted(&self) -> &[Txid] {
        match self {
            Self::Accepted { evicted, .. } | Self::Replaced { evicted, .. } => evicted,
            Self::Rejected { .. }
            | Self::Duplicate { .. }
            | Self::Orphaned { .. }
            | Self::Evicted { .. }
            | Self::Expired { .. } => &[],
        }
    }

    pub const fn maybe_rejection_category(&self) -> Option<MempoolRejectionCategory> {
        match self {
            Self::Rejected { category, .. } => Some(*category),
            Self::Accepted { .. }
            | Self::Duplicate { .. }
            | Self::Replaced { .. }
            | Self::Orphaned { .. }
            | Self::Evicted { .. }
            | Self::Expired { .. } => None,
        }
    }
}
