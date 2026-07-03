// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

//! Shared sanitized relay and mempool evidence status contracts.

use serde::{Deserialize, Serialize};

/// Default reason when mempool admission evidence has not been projected.
pub const MEMPOOL_ADMISSION_EVIDENCE_UNAVAILABLE_REASON: &str =
    "mempool admission evidence unavailable";

/// Default reason when local submission relay evidence has not been projected.
pub const LOCAL_SUBMISSION_RELAY_EVIDENCE_DEFERRED_REASON: &str =
    "local submission relay evidence not projected";

/// Default reason when relay fanout evidence has not been projected.
pub const RELAY_FANOUT_EVIDENCE_DEFERRED_REASON: &str = "relay fanout evidence not projected";

/// Default reason when relay serving evidence has not been projected.
pub const RELAY_SERVING_EVIDENCE_DEFERRED_REASON: &str = "relay serving evidence not projected";

/// Default reason when rebroadcast evidence has not been projected.
pub const REBROADCAST_EVIDENCE_DEFERRED_REASON: &str = "rebroadcast relay evidence not projected";

/// Default reason when durable mempool recovery evidence could not be loaded.
pub const RELAY_RECOVERY_EVIDENCE_UNAVAILABLE_REASON: &str = "relay recovery evidence unavailable";

/// Stable reason for Open Bitcoin's non-promissory public relay boundary.
pub const PUBLIC_RELAY_INTENTIONALLY_DIFFERENT_REASON: &str =
    "public relay readiness is intentionally not claimed";

/// Explicit state wrapper for relay evidence fields.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "state", content = "value")]
pub enum RelayEvidenceField<T> {
    Implemented(T),
    Unavailable { reason: String },
    Deferred { reason: String },
    IntentionallyDifferent { reason: String },
}

impl<T> RelayEvidenceField<T> {
    pub const fn implemented(value: T) -> Self {
        Self::Implemented(value)
    }

    pub fn unavailable(reason: impl Into<String>) -> Self {
        Self::Unavailable {
            reason: reason.into(),
        }
    }

    pub fn deferred(reason: impl Into<String>) -> Self {
        Self::Deferred {
            reason: reason.into(),
        }
    }

    pub fn intentionally_different(reason: impl Into<String>) -> Self {
        Self::IntentionallyDifferent {
            reason: reason.into(),
        }
    }
}

/// Fixed relay and mempool outcome counters safe for every operator surface.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct RelayEvidenceCounters {
    pub accepted_count: u64,
    pub rejected_count: u64,
    pub orphaned_count: u64,
    pub requested_count: u64,
    pub served_count: u64,
    pub announced_count: u64,
    pub suppressed_count: u64,
    pub evicted_count: u64,
    pub expired_count: u64,
    pub rebroadcast_deferred_count: u64,
}

/// Fixed durable mempool recovery counters safe for every operator surface.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct RelayRecoveryCounters {
    pub recovered_count: u64,
    pub dropped_confirmed_count: u64,
    pub dropped_duplicate_count: u64,
    pub dropped_missing_parent_count: u64,
    pub dropped_policy_incompatible_count: u64,
    pub dropped_evicted_count: u64,
}

/// Explicit relay activation evidence safe for operator status surfaces.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct RelayActivationEvidence {
    pub enabled: bool,
}

/// Aggregate transaction download eligibility counters safe for public status.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct RelayDownloadEligibilityCounters {
    pub eligible_peer_count: u64,
    pub ineligible_peer_count: u64,
    pub relay_disabled_count: u64,
    pub not_relay_eligible_count: u64,
    pub inbound_serving_required_count: u64,
    pub permission_required_count: u64,
    pub protected_not_relay_count: u64,
}

/// Low-cardinality implemented capability labels for relay evidence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RelayEvidenceCapability {
    MempoolAdmission,
    LocalSubmissionRelay,
    RelayFanout,
    RelayServing,
    Rebroadcast,
    PublicRelayReadiness,
}

/// Implemented evidence value for a relay or mempool capability.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct RelayCapabilityEvidence {
    pub capability: RelayEvidenceCapability,
}

impl RelayCapabilityEvidence {
    pub const fn new(capability: RelayEvidenceCapability) -> Self {
        Self { capability }
    }
}

/// Shared status contract for sanitized relay and mempool evidence.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RelayEvidenceStatus {
    #[serde(default = "activation_default")]
    pub activation: RelayEvidenceField<RelayActivationEvidence>,
    #[serde(default = "download_eligibility_default")]
    pub download_eligibility: RelayEvidenceField<RelayDownloadEligibilityCounters>,
    #[serde(default = "outcome_counters_default")]
    pub outcome_counters: RelayEvidenceField<RelayEvidenceCounters>,
    #[serde(default = "recovery_counters_default")]
    pub recovery_counters: RelayEvidenceField<RelayRecoveryCounters>,
    #[serde(default = "mempool_admission_unavailable")]
    pub mempool_admission: RelayEvidenceField<RelayCapabilityEvidence>,
    #[serde(default = "local_submission_deferred")]
    pub local_submission: RelayEvidenceField<RelayCapabilityEvidence>,
    #[serde(default = "fanout_deferred")]
    pub fanout: RelayEvidenceField<RelayCapabilityEvidence>,
    #[serde(default = "serving_deferred")]
    pub serving: RelayEvidenceField<RelayCapabilityEvidence>,
    #[serde(default = "rebroadcast_deferred")]
    pub rebroadcast: RelayEvidenceField<RelayCapabilityEvidence>,
    #[serde(default = "public_relay_intentionally_different")]
    pub public_relay: RelayEvidenceField<RelayCapabilityEvidence>,
}

impl RelayEvidenceStatus {
    pub fn default_unavailable() -> Self {
        Self {
            activation: activation_default(),
            download_eligibility: download_eligibility_default(),
            outcome_counters: outcome_counters_default(),
            recovery_counters: recovery_counters_default(),
            mempool_admission: mempool_admission_unavailable(),
            local_submission: local_submission_deferred(),
            fanout: fanout_deferred(),
            serving: serving_deferred(),
            rebroadcast: rebroadcast_deferred(),
            public_relay: public_relay_intentionally_different(),
        }
    }

    pub fn with_counters(counters: RelayEvidenceCounters) -> Self {
        Self::with_activation_and_counters(
            RelayActivationEvidence::default(),
            RelayDownloadEligibilityCounters::default(),
            counters,
        )
    }

    pub fn with_activation_and_counters(
        activation: RelayActivationEvidence,
        download_eligibility: RelayDownloadEligibilityCounters,
        counters: RelayEvidenceCounters,
    ) -> Self {
        Self::with_activation_recovery_and_counters(
            activation,
            download_eligibility,
            RelayRecoveryCounters::default(),
            counters,
        )
    }

    pub fn with_activation_recovery_and_counters(
        activation: RelayActivationEvidence,
        download_eligibility: RelayDownloadEligibilityCounters,
        recovery: RelayRecoveryCounters,
        counters: RelayEvidenceCounters,
    ) -> Self {
        Self {
            activation: RelayEvidenceField::implemented(activation),
            download_eligibility: RelayEvidenceField::implemented(download_eligibility),
            outcome_counters: RelayEvidenceField::implemented(counters),
            recovery_counters: RelayEvidenceField::implemented(recovery),
            ..Self::default_unavailable()
        }
    }
}

impl Default for RelayEvidenceStatus {
    fn default() -> Self {
        Self::default_unavailable()
    }
}

fn outcome_counters_default() -> RelayEvidenceField<RelayEvidenceCounters> {
    RelayEvidenceField::implemented(RelayEvidenceCounters::default())
}

fn recovery_counters_default() -> RelayEvidenceField<RelayRecoveryCounters> {
    RelayEvidenceField::implemented(RelayRecoveryCounters::default())
}

fn activation_default() -> RelayEvidenceField<RelayActivationEvidence> {
    RelayEvidenceField::implemented(RelayActivationEvidence::default())
}

fn download_eligibility_default() -> RelayEvidenceField<RelayDownloadEligibilityCounters> {
    RelayEvidenceField::implemented(RelayDownloadEligibilityCounters::default())
}

fn mempool_admission_unavailable() -> RelayEvidenceField<RelayCapabilityEvidence> {
    RelayEvidenceField::unavailable(MEMPOOL_ADMISSION_EVIDENCE_UNAVAILABLE_REASON)
}

fn local_submission_deferred() -> RelayEvidenceField<RelayCapabilityEvidence> {
    RelayEvidenceField::deferred(LOCAL_SUBMISSION_RELAY_EVIDENCE_DEFERRED_REASON)
}

fn fanout_deferred() -> RelayEvidenceField<RelayCapabilityEvidence> {
    RelayEvidenceField::deferred(RELAY_FANOUT_EVIDENCE_DEFERRED_REASON)
}

fn serving_deferred() -> RelayEvidenceField<RelayCapabilityEvidence> {
    RelayEvidenceField::deferred(RELAY_SERVING_EVIDENCE_DEFERRED_REASON)
}

fn rebroadcast_deferred() -> RelayEvidenceField<RelayCapabilityEvidence> {
    RelayEvidenceField::deferred(REBROADCAST_EVIDENCE_DEFERRED_REASON)
}

fn public_relay_intentionally_different() -> RelayEvidenceField<RelayCapabilityEvidence> {
    RelayEvidenceField::intentionally_different(PUBLIC_RELAY_INTENTIONALLY_DIFFERENT_REASON)
}
