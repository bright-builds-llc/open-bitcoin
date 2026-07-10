// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/net_processing.h
// - packages/bitcoin-knots/src/net.h
// - packages/bitcoin-knots/src/protocol.h
// - packages/bitcoin-knots/test/functional/p2p_compactblocks.py

use open_bitcoin_codec::{BIP152_COMPACT_BLOCKS_VERSION, SendCompactMessage};

use crate::block_serving::{
    BlockRelayActivationPolicy, BlockServingResourceGateDecision, BlockServingStatusDecision,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CompactRelayPeerState {
    pub capability: CompactRelayCapability,
    pub high_bandwidth_preference: CompactRelayPreference,
    pub low_bandwidth_preference: CompactRelayPreference,
    pub announcement_eligibility: CompactAnnouncementEligibility,
    pub maybe_unsupported_version: Option<u64>,
}

impl CompactRelayPeerState {
    pub fn apply_send_compact(
        &mut self,
        message: SendCompactMessage,
    ) -> CompactRelayNegotiationOutcome {
        if message.version != BIP152_COMPACT_BLOCKS_VERSION {
            return self.record_unsupported_version(message.version);
        }

        self.capability = CompactRelayCapability::Supported {
            version: BIP152_COMPACT_BLOCKS_VERSION,
        };

        if message.announce {
            self.high_bandwidth_preference = CompactRelayPreference::Requested;
            self.low_bandwidth_preference = CompactRelayPreference::NotRequested;
            return self.negotiation_outcome(CompactRelayNegotiationReason::Version2HighBandwidth);
        }

        self.low_bandwidth_preference = CompactRelayPreference::Requested;
        self.high_bandwidth_preference = CompactRelayPreference::NotRequested;
        self.negotiation_outcome(CompactRelayNegotiationReason::Version2LowBandwidth)
    }

    pub fn record_announcement_decision(&mut self, decision: &CompactAnnouncementDecision) {
        self.announcement_eligibility = decision.eligibility;
    }

    fn record_unsupported_version(&mut self, version: u64) -> CompactRelayNegotiationOutcome {
        self.maybe_unsupported_version = Some(version);
        if !matches!(self.capability, CompactRelayCapability::Supported { .. }) {
            self.capability = CompactRelayCapability::Unsupported { version };
        }
        self.negotiation_outcome(CompactRelayNegotiationReason::UnsupportedVersion)
    }

    fn negotiation_outcome(
        &self,
        reason: CompactRelayNegotiationReason,
    ) -> CompactRelayNegotiationOutcome {
        CompactRelayNegotiationOutcome {
            capability: self.capability,
            high_bandwidth_preference: self.high_bandwidth_preference,
            low_bandwidth_preference: self.low_bandwidth_preference,
            announcement_eligibility: self.announcement_eligibility,
            maybe_unsupported_version: self.maybe_unsupported_version,
            reason,
        }
    }
}

impl Default for CompactRelayPeerState {
    fn default() -> Self {
        Self {
            capability: CompactRelayCapability::Unknown,
            high_bandwidth_preference: CompactRelayPreference::Unknown,
            low_bandwidth_preference: CompactRelayPreference::Unknown,
            announcement_eligibility: CompactAnnouncementEligibility::Unknown,
            maybe_unsupported_version: None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompactRelayCapability {
    Unknown,
    Supported { version: u64 },
    Unsupported { version: u64 },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompactRelayPreference {
    Unknown,
    Requested,
    NotRequested,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompactAnnouncementEligibility {
    Unknown,
    Eligible,
    Ineligible {
        reason: CompactAnnouncementEligibilityReason,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompactAnnouncementEligibilityReason {
    LocalActivationDisabled,
    PeerNotNegotiated,
    UnsupportedVersion,
    HighBandwidthNotRequested,
    HeaderContinuityMissing,
    PeerAlreadyHasHeader,
    BlockUnavailable,
    ResourceLimited,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompactAnnouncementInput {
    pub activation: BlockRelayActivationPolicy,
    pub peer_state: CompactRelayPeerState,
    pub peer_prefers_headers: bool,
    pub peer_has_previous_header: bool,
    pub peer_has_current_header: bool,
    pub status: BlockServingStatusDecision,
    pub resource_gate: BlockServingResourceGateDecision,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PeerCompactAnnouncementInput {
    pub activation: BlockRelayActivationPolicy,
    pub peer_has_previous_header: bool,
    pub peer_has_current_header: bool,
    pub status: BlockServingStatusDecision,
    pub resource_gate: BlockServingResourceGateDecision,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CompactAnnouncementDecision {
    pub action: CompactAnnouncementAction,
    pub reason: CompactAnnouncementReason,
    pub eligibility: CompactAnnouncementEligibility,
}

impl CompactAnnouncementDecision {
    const fn new(action: CompactAnnouncementAction, reason: CompactAnnouncementReason) -> Self {
        Self {
            action,
            reason,
            eligibility: reason.eligibility(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompactAnnouncementAction {
    AnnounceCompactBlock,
    AnnounceHeaders,
    AnnounceInventory,
    Suppress,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompactAnnouncementReason {
    CompactAnnounced,
    CompactRelayDisabled,
    CompactPeerNotNegotiated,
    CompactUnsupportedVersion,
    CompactHighBandwidthNotRequested,
    CompactHeaderContinuityMissing,
    CompactPeerAlreadyHasHeader,
    CompactBlockUnavailable,
    CompactResourceLimited,
    CompactHeadersFallback,
    CompactInventoryFallback,
}

impl CompactAnnouncementReason {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CompactAnnounced => "compact_announced",
            Self::CompactRelayDisabled => "compact_relay_disabled",
            Self::CompactPeerNotNegotiated => "compact_peer_not_negotiated",
            Self::CompactUnsupportedVersion => "compact_unsupported_version",
            Self::CompactHighBandwidthNotRequested => "compact_high_bandwidth_not_requested",
            Self::CompactHeaderContinuityMissing => "compact_header_continuity_missing",
            Self::CompactPeerAlreadyHasHeader => "compact_peer_already_has_header",
            Self::CompactBlockUnavailable => "compact_block_unavailable",
            Self::CompactResourceLimited => "compact_resource_limited",
            Self::CompactHeadersFallback => "compact_headers_fallback",
            Self::CompactInventoryFallback => "compact_inventory_fallback",
        }
    }

    const fn eligibility(self) -> CompactAnnouncementEligibility {
        match self {
            Self::CompactAnnounced => CompactAnnouncementEligibility::Eligible,
            Self::CompactRelayDisabled => {
                Self::ineligible(CompactAnnouncementEligibilityReason::LocalActivationDisabled)
            }
            Self::CompactPeerNotNegotiated => {
                Self::ineligible(CompactAnnouncementEligibilityReason::PeerNotNegotiated)
            }
            Self::CompactUnsupportedVersion => {
                Self::ineligible(CompactAnnouncementEligibilityReason::UnsupportedVersion)
            }
            Self::CompactHighBandwidthNotRequested => {
                Self::ineligible(CompactAnnouncementEligibilityReason::HighBandwidthNotRequested)
            }
            Self::CompactHeaderContinuityMissing => {
                Self::ineligible(CompactAnnouncementEligibilityReason::HeaderContinuityMissing)
            }
            Self::CompactPeerAlreadyHasHeader => {
                Self::ineligible(CompactAnnouncementEligibilityReason::PeerAlreadyHasHeader)
            }
            Self::CompactBlockUnavailable => {
                Self::ineligible(CompactAnnouncementEligibilityReason::BlockUnavailable)
            }
            Self::CompactResourceLimited => {
                Self::ineligible(CompactAnnouncementEligibilityReason::ResourceLimited)
            }
            Self::CompactHeadersFallback | Self::CompactInventoryFallback => {
                Self::ineligible(CompactAnnouncementEligibilityReason::PeerNotNegotiated)
            }
        }
    }

    const fn ineligible(
        reason: CompactAnnouncementEligibilityReason,
    ) -> CompactAnnouncementEligibility {
        CompactAnnouncementEligibility::Ineligible { reason }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CompactRelayNegotiationOutcome {
    pub capability: CompactRelayCapability,
    pub high_bandwidth_preference: CompactRelayPreference,
    pub low_bandwidth_preference: CompactRelayPreference,
    pub announcement_eligibility: CompactAnnouncementEligibility,
    pub maybe_unsupported_version: Option<u64>,
    pub reason: CompactRelayNegotiationReason,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompactRelayNegotiationReason {
    Version2HighBandwidth,
    Version2LowBandwidth,
    UnsupportedVersion,
}

pub fn decide_compact_announcement(input: CompactAnnouncementInput) -> CompactAnnouncementDecision {
    if !input.activation.compact_relay.enabled {
        return fallback_decision(
            input.peer_prefers_headers,
            CompactAnnouncementReason::CompactRelayDisabled,
        );
    }

    match input.peer_state.capability {
        CompactRelayCapability::Supported {
            version: BIP152_COMPACT_BLOCKS_VERSION,
        } => {}
        CompactRelayCapability::Unknown if input.peer_state.maybe_unsupported_version.is_none() => {
            return fallback_decision(
                input.peer_prefers_headers,
                CompactAnnouncementReason::CompactPeerNotNegotiated,
            );
        }
        CompactRelayCapability::Unknown
        | CompactRelayCapability::Unsupported { .. }
        | CompactRelayCapability::Supported { .. } => {
            return fallback_decision(
                input.peer_prefers_headers,
                CompactAnnouncementReason::CompactUnsupportedVersion,
            );
        }
    }

    if input.peer_state.high_bandwidth_preference != CompactRelayPreference::Requested {
        return fallback_decision(
            input.peer_prefers_headers,
            CompactAnnouncementReason::CompactHighBandwidthNotRequested,
        );
    }

    if !input.peer_has_previous_header {
        return CompactAnnouncementDecision::new(
            CompactAnnouncementAction::AnnounceHeaders,
            CompactAnnouncementReason::CompactHeaderContinuityMissing,
        );
    }

    if input.peer_has_current_header {
        return CompactAnnouncementDecision::new(
            CompactAnnouncementAction::AnnounceHeaders,
            CompactAnnouncementReason::CompactPeerAlreadyHasHeader,
        );
    }

    if !input.status.may_serve_block {
        return CompactAnnouncementDecision::new(
            CompactAnnouncementAction::Suppress,
            CompactAnnouncementReason::CompactBlockUnavailable,
        );
    }

    if !input.resource_gate.may_serve_block {
        return CompactAnnouncementDecision::new(
            CompactAnnouncementAction::Suppress,
            CompactAnnouncementReason::CompactResourceLimited,
        );
    }

    CompactAnnouncementDecision::new(
        CompactAnnouncementAction::AnnounceCompactBlock,
        CompactAnnouncementReason::CompactAnnounced,
    )
}

fn fallback_decision(
    peer_prefers_headers: bool,
    reason: CompactAnnouncementReason,
) -> CompactAnnouncementDecision {
    let action = if peer_prefers_headers {
        CompactAnnouncementAction::AnnounceHeaders
    } else {
        CompactAnnouncementAction::AnnounceInventory
    };

    CompactAnnouncementDecision::new(action, reason)
}

#[cfg(test)]
mod tests;
