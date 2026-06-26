#![forbid(unsafe_code)]
#![cfg_attr(
    not(test),
    deny(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::unreachable,
        clippy::todo,
        clippy::unimplemented,
        clippy::panic_in_result_fn,
    )
)]
// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

//! Pure-core peer lifecycle, wire-message, and sync state for Open Bitcoin.

mod address;
mod compatibility;
mod error;
mod header_store;
mod inbound;
mod message;
mod peer;
mod peer_policy;

pub use address::{
    AddressAnnouncement, AddressClassification, AddressDecisionLabel, AddressDecisionReason,
    AddressList, AddressNetworkKind, AddressResponseCache, AddressResponseEntryEvidence,
    AddressSourceKind, GetAddrPeerEligibility, GetAddrRequestState, GetAddrResponseDecision,
    LearnedAddressBatchDecision, LearnedAddressBook, LearnedAddressDecision, LearnedAddressEntry,
    LocalAdvertisementDecision, LocalAdvertisementInput, PHASE92_GETADDR_RESPONSE_LIMIT,
    PHASE92_LEARNED_ADDR_BATCH_LIMIT, PHASE92_MAX_ADDR_AGE_SECONDS,
    PHASE92_MAX_FUTURE_SKEW_SECONDS, RoutabilityClass, classify_network_address,
    maybe_version_sender_address, privacy_network_deferred_classification, select_getaddr_response,
    select_local_advertisement_candidates, unsupported_future_network_classification,
};
pub use compatibility::{
    CompatibilityDiagnosis, CompatibilityReport, TranscriptEvent, TranscriptStep,
    evaluate_transcript,
};
pub use error::PeerId;
pub use error::{DisconnectReason, NetworkError};
pub use header_store::{HeaderEntry, HeaderStore, InsertedHeader};
pub use inbound::{
    INBOUND_ENABLED_FIELD, INBOUND_LISTEN_ADDRESSES_FIELD, INBOUND_PERMISSION_ADDRESSES_FIELD,
    INBOUND_PERMISSION_CLASS_NAME_FIELD, INBOUND_PERMISSION_TOKENS_FIELD,
    InactivePermissionEffectLabel, InboundAdmissionCounters, InboundAdmissionDecision,
    InboundAdmissionPolicy, InboundAdmissionRejection, InboundAdmissionRejectionReason,
    InboundAdmissionRequest, InboundAdmissionSlotClass, InboundHandshakeState,
    InboundListenerActivationDiagnostic, InboundListenerConfig, InboundListenerEndpoint,
    InboundPeerRecord, InboundPermissionDecision, InboundPreflightDiagnostic, InboundPreflightPlan,
    InboundPreflightReason, ParsedPeerPermissionClass, PeerConnectionClass,
    PeerPermissionClassRegistry, PeerPermissionDirection, PeerPermissionParseError,
    PeerPermissionSet, PeerPermissionToken, PermissionClassName, PermissionEffectLabel,
    classify_inbound_preflight,
};
pub use message::{
    HeadersMessage, InventoryList, LocalPeerConfig, MAX_HEADERS_RESULTS, MAX_INV_SIZE,
    PROTOCOL_VERSION, ParsedNetworkMessage, ServiceFlags, USER_AGENT, VersionMessage,
    WireNetworkMessage,
};
pub use peer::{
    ConnectionRole, HeaderSyncPolicy, PeerAction, PeerAddressBoundaryDecision,
    PeerAddressBoundaryEvidence, PeerManager, PeerState,
};
pub use peer_policy::{
    BanDecision, BanReason, BanScope, EvictionCandidate, EvictionCandidateInput, EvictionDecision,
    EvictionReason, EvictionScoreComponent, MisbehaviorDecision, MisbehaviorKind,
    MisbehaviorObservation, MisbehaviorPolicy, MisbehaviorResponse, PeerBanBook, PeerBanEntry,
    UnbanDecision, select_eviction_candidate,
};

pub const fn crate_ready() -> bool {
    true
}

#[cfg(test)]
mod tests {
    use super::crate_ready;

    #[test]
    fn crate_ready_reports_true() {
        assert!(crate_ready());
    }
}
