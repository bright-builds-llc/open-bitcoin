// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net.cpp
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/banman.h
// - packages/bitcoin-knots/src/banman.cpp
// - packages/bitcoin-knots/src/net_permissions.cpp

use core::net::IpAddr;
use std::collections::BTreeMap;

use crate::inbound::{InboundHandshakeState, PermissionEffectLabel};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum EvictionReason {
    CapPressure,
    HandshakeStalled,
    LowActivity,
    DiversityPressure,
    ProtectedPeer,
    NoCandidate,
}

impl EvictionReason {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CapPressure => "cap_pressure",
            Self::HandshakeStalled => "handshake_stalled",
            Self::LowActivity => "low_activity",
            Self::DiversityPressure => "diversity_pressure",
            Self::ProtectedPeer => "protected_peer",
            Self::NoCandidate => "no_eviction_candidate",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvictionScoreComponent {
    pub label: &'static str,
    pub points: i32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvictionCandidateInput {
    pub peer_label: String,
    pub handshake_state: InboundHandshakeState,
    pub connection_class: &'static str,
    pub slot_class: &'static str,
    pub requested_inventory_count: usize,
    pub active_permission_effects: Vec<PermissionEffectLabel>,
    pub diversity_group: String,
}

impl EvictionCandidateInput {
    pub fn is_protected(&self) -> bool {
        self.active_permission_effects
            .contains(&PermissionEffectLabel::EvictionPolicyProtected)
            || self.connection_class == "protected_inbound"
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvictionCandidate {
    pub peer_label: String,
    pub reason: EvictionReason,
    pub score: i32,
    pub components: Vec<EvictionScoreComponent>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EvictionDecision {
    Select(EvictionCandidate),
    Suppress {
        reason: EvictionReason,
        protected_peer_count: usize,
    },
}

impl EvictionDecision {
    pub fn outcome_label(&self) -> &'static str {
        match self {
            Self::Select(_) => "eviction_candidate_selected",
            Self::Suppress { .. } => "eviction_suppressed",
        }
    }
}

pub fn select_eviction_candidate(inputs: &[EvictionCandidateInput]) -> EvictionDecision {
    let mut candidates = Vec::new();
    let mut protected_peer_count = 0;
    let mut diversity_counts: BTreeMap<&str, usize> = BTreeMap::new();

    for input in inputs {
        *diversity_counts
            .entry(input.diversity_group.as_str())
            .or_default() += 1;
    }

    for input in inputs {
        if input.is_protected() {
            protected_peer_count += 1;
            continue;
        }
        candidates.push(score_eviction_candidate(input, &diversity_counts));
    }

    let maybe_candidate = candidates.into_iter().max_by(|left, right| {
        left.score
            .cmp(&right.score)
            .then(left.peer_label.cmp(&right.peer_label))
    });

    match maybe_candidate {
        Some(candidate) if candidate.score > 0 => EvictionDecision::Select(candidate),
        _ => EvictionDecision::Suppress {
            reason: EvictionReason::NoCandidate,
            protected_peer_count,
        },
    }
}

fn score_eviction_candidate(
    input: &EvictionCandidateInput,
    diversity_counts: &BTreeMap<&str, usize>,
) -> EvictionCandidate {
    let mut components = Vec::new();

    if input.handshake_state != InboundHandshakeState::Established {
        components.push(EvictionScoreComponent {
            label: EvictionReason::HandshakeStalled.as_str(),
            points: 40,
        });
    }
    if input.requested_inventory_count == 0 {
        components.push(EvictionScoreComponent {
            label: EvictionReason::LowActivity.as_str(),
            points: 10,
        });
    }
    if diversity_counts
        .get(input.diversity_group.as_str())
        .copied()
        .unwrap_or_default()
        > 1
    {
        components.push(EvictionScoreComponent {
            label: EvictionReason::DiversityPressure.as_str(),
            points: 20,
        });
    }
    if input.slot_class == "ordinary" {
        components.push(EvictionScoreComponent {
            label: EvictionReason::CapPressure.as_str(),
            points: 15,
        });
    }

    let score = components.iter().map(|component| component.points).sum();
    let reason = components
        .first()
        .map(|component| match component.label {
            "handshake_stalled" => EvictionReason::HandshakeStalled,
            "low_activity" => EvictionReason::LowActivity,
            "diversity_pressure" => EvictionReason::DiversityPressure,
            _ => EvictionReason::CapPressure,
        })
        .unwrap_or(EvictionReason::NoCandidate);

    EvictionCandidate {
        peer_label: input.peer_label.clone(),
        reason,
        score,
        components,
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum BanScope {
    Address(IpAddr),
    Subnet { network: IpAddr, prefix_bits: u8 },
}

impl BanScope {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Address(_) => "address",
            Self::Subnet { .. } => "subnet",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BanReason {
    MisbehaviorThreshold,
    Manual,
    InvalidAddressAbuse,
}

impl BanReason {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MisbehaviorThreshold => "misbehavior_threshold_reached",
            Self::Manual => "manual_ban",
            Self::InvalidAddressAbuse => "invalid_address_abuse",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PeerBanEntry {
    pub scope: BanScope,
    pub reason: BanReason,
    pub created_at_unix_seconds: i64,
    pub expires_at_unix_seconds: i64,
    pub source: &'static str,
}

impl PeerBanEntry {
    pub fn is_expired(&self, now_unix_seconds: i64) -> bool {
        self.expires_at_unix_seconds <= now_unix_seconds
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BanDecision {
    Active(PeerBanEntry),
    Expired(PeerBanEntry),
}

impl BanDecision {
    pub const fn outcome_label(&self) -> &'static str {
        match self {
            Self::Active(_) => "ban_active",
            Self::Expired(_) => "ban_expired",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UnbanDecision {
    Unbanned(PeerBanEntry),
    NotFound(BanScope),
    AlreadyExpired(PeerBanEntry),
}

impl UnbanDecision {
    pub const fn outcome_label(&self) -> &'static str {
        match self {
            Self::Unbanned(_) => "unbanned",
            Self::NotFound(_) => "unban_not_found",
            Self::AlreadyExpired(_) => "unban_already_expired",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct PeerBanBook {
    entries: BTreeMap<BanScope, PeerBanEntry>,
}

impl PeerBanBook {
    pub fn ban(&mut self, entry: PeerBanEntry, now_unix_seconds: i64) -> BanDecision {
        self.entries.insert(entry.scope.clone(), entry.clone());
        if entry.is_expired(now_unix_seconds) {
            return BanDecision::Expired(entry);
        }
        BanDecision::Active(entry)
    }

    pub fn maybe_ban_decision(
        &self,
        scope: &BanScope,
        now_unix_seconds: i64,
    ) -> Option<BanDecision> {
        let entry = self.entries.get(scope)?.clone();
        if entry.is_expired(now_unix_seconds) {
            return Some(BanDecision::Expired(entry));
        }
        Some(BanDecision::Active(entry))
    }

    pub fn unban(&mut self, scope: &BanScope, now_unix_seconds: i64) -> UnbanDecision {
        let Some(entry) = self.entries.remove(scope) else {
            return UnbanDecision::NotFound(scope.clone());
        };
        if entry.is_expired(now_unix_seconds) {
            return UnbanDecision::AlreadyExpired(entry);
        }
        UnbanDecision::Unbanned(entry)
    }

    pub fn active_count(&self, now_unix_seconds: i64) -> usize {
        self.entries
            .values()
            .filter(|entry| !entry.is_expired(now_unix_seconds))
            .count()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MisbehaviorKind {
    MalformedMessage,
    DuplicateVersion,
    InvalidAddress,
    UnsupportedCommandAbuse,
    HeaderViolation,
}

impl MisbehaviorKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MalformedMessage => "malformed_message",
            Self::DuplicateVersion => "duplicate_version",
            Self::InvalidAddress => "invalid_address",
            Self::UnsupportedCommandAbuse => "unsupported_command_abuse",
            Self::HeaderViolation => "header_violation",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MisbehaviorResponse {
    ObserveOnly,
    Disconnect,
    Discourage,
    Ban,
    ProtectedNoAction,
}

impl MisbehaviorResponse {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ObserveOnly => "misbehavior_observed",
            Self::Disconnect => "disconnect_requested",
            Self::Discourage => "discouraged",
            Self::Ban => "ban_active",
            Self::ProtectedNoAction => "protected_no_action",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MisbehaviorPolicy {
    pub disconnect_threshold: u32,
    pub discourage_threshold: u32,
    pub ban_threshold: u32,
}

impl Default for MisbehaviorPolicy {
    fn default() -> Self {
        Self {
            disconnect_threshold: 10,
            discourage_threshold: 50,
            ban_threshold: 100,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MisbehaviorObservation {
    pub peer_label: String,
    pub kind: MisbehaviorKind,
    pub points: u32,
    pub prior_score: u32,
    pub protected: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MisbehaviorDecision {
    pub peer_label: String,
    pub kind: MisbehaviorKind,
    pub score: u32,
    pub response: MisbehaviorResponse,
}

impl MisbehaviorPolicy {
    pub fn decide(self, observation: MisbehaviorObservation) -> MisbehaviorDecision {
        let score = observation.prior_score.saturating_add(observation.points);
        let response = if observation.protected {
            MisbehaviorResponse::ProtectedNoAction
        } else if score >= self.ban_threshold {
            MisbehaviorResponse::Ban
        } else if score >= self.discourage_threshold {
            MisbehaviorResponse::Discourage
        } else if score >= self.disconnect_threshold {
            MisbehaviorResponse::Disconnect
        } else {
            MisbehaviorResponse::ObserveOnly
        };

        MisbehaviorDecision {
            peer_label: observation.peer_label,
            kind: observation.kind,
            score,
            response,
        }
    }
}

#[cfg(test)]
mod tests;
