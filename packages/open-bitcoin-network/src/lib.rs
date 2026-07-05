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
mod block_serving;
mod compact_reconstruction;
mod compatibility;
mod error;
mod header_store;
mod inbound;
mod message;
mod peer;
mod peer_policy;
mod relay;
mod resource;

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
pub use block_serving::{
    BlockInFlightCleanupCause, BlockInFlightCleanupDecision, BlockInFlightCleanupInput,
    BlockRelayActivationPolicy, BlockServingActivationConfig, BlockServingChainPosition,
    BlockServingDataAvailability, BlockServingEligibilityDecision, BlockServingEligibilityInput,
    BlockServingEligibilityReason, BlockServingOutcomeLabel, BlockServingResourceGateDecision,
    BlockServingResourceGateInput, BlockServingStatusDecision, BlockServingStatusFacts,
    BlockServingStatusLabel, BlockServingValidationState, CompactRelayActivationConfig,
    classify_block_inflight_cleanup, classify_block_serving_eligibility,
    classify_block_serving_status, evaluate_block_serving_resource_gate,
};
pub use compact_reconstruction::{
    CompactReconstructionFailureReason, CompactReconstructionInvalidReason,
    CompactReconstructionOutcome, PartialCompactBlock, init_partial_compact_block,
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
    RelayPermissionEffectLabel, classify_inbound_preflight,
};
pub use message::{
    HeadersMessage, InventoryList, LocalPeerConfig, MAX_HEADERS_RESULTS, MAX_INV_SIZE,
    PROTOCOL_VERSION, ParsedNetworkMessage, ServiceFlags, USER_AGENT, VersionMessage,
    WireNetworkMessage,
};
pub use peer::{
    CompactAnnouncementAction, CompactAnnouncementDecision, CompactAnnouncementEligibility,
    CompactAnnouncementEligibilityReason, CompactAnnouncementInput, CompactAnnouncementReason,
    CompactRelayCapability, CompactRelayNegotiationOutcome, CompactRelayNegotiationReason,
    CompactRelayPeerState, CompactRelayPreference, ConnectionRole, HeaderSyncPolicy, OrphanAction,
    OrphanEvidenceLabel, OrphanPolicy, OrphanReconsiderationCandidate, OrphanReconsiderationStatus,
    OrphanStageInput, PHASE101_GETDATA_TX_INTERVAL_SECONDS, PHASE101_MAX_TX_ANNOUNCEMENTS_PER_PEER,
    PHASE101_MAX_TX_REQUESTS_IN_FLIGHT_PER_PEER, PHASE101_NONPREF_PEER_TX_DELAY_SECONDS,
    PHASE101_OVERLOADED_PEER_TX_DELAY_SECONDS, PHASE101_TXID_RELAY_DELAY_SECONDS,
    PHASE102_MAX_ORPHAN_TRANSACTIONS, PHASE102_MAX_ORPHANS_PER_PEER,
    PHASE102_MAX_RECONSIDERATIONS_PER_PARENT, PHASE102_ORPHAN_TTL_SECONDS,
    PHASE104_MAX_TX_FANOUT_DRAIN_PER_PEER, PHASE104_MAX_TX_FANOUT_QUEUE_PER_PEER,
    PHASE104_TX_FANOUT_MIN_INTERVAL_SECONDS, PeerAction, PeerAddressBoundaryDecision,
    PeerAddressBoundaryEvidence, PeerCompactAnnouncementInput, PeerManager, PeerState,
    RelayDownloadPolicy, TxAnnouncementInput, TxDownloadAction, TxDownloadLocalFacts,
    TxDownloadPolicy, TxDownloadScheduler, TxDownloadSnapshot, TxDownloadSuppressionReason,
    TxFanoutAction, TxFanoutAdmission, TxFanoutAdmissionOutcome, TxFanoutCleanupReason,
    TxFanoutPeerInput, TxFanoutPolicy, TxFanoutQueue, TxFanoutSnapshot, TxFanoutSuppressionReason,
    TxOrphanage, TxParentRequestInput, TxPeerRequestSnapshot, TxRelayId, TxRelayIdentityError,
    TxRelayPeerMode, TxServeDecision, TxServeOutcomeLabel, TxServingRecordStatus,
    classify_tx_serve_request, decide_compact_announcement, defer_local_rebroadcast,
};
pub use peer_policy::{
    BanDecision, BanReason, BanScope, EvictionCandidate, EvictionCandidateInput, EvictionDecision,
    EvictionReason, EvictionScoreComponent, MAX_PEER_POLICY_RUNTIME_DECISIONS, MisbehaviorDecision,
    MisbehaviorKind, MisbehaviorObservation, MisbehaviorPolicy, MisbehaviorResponse, PeerBanBook,
    PeerBanEntry, PeerPolicyRuntimeState, UnbanDecision, select_eviction_candidate,
};
pub use relay::{
    RelayActivationConfig, RelayEligibilityDecision, RelayEligibilityInput, RelayEligibilityReason,
    classify_relay_eligibility,
};
pub use resource::{
    ConnectionChurnInput, INBOUND_MESSAGE_HEADER_LEN, InboundEnvelopeDecision,
    InboundEnvelopePolicy, InboundResourceEvent, PHASE94_CONNECTION_CHURN_WINDOW_SECONDS,
    PHASE94_IDLE_PEER_TIMEOUT_SECONDS, PHASE94_MAX_AGGREGATE_QUEUED_MESSAGES,
    PHASE94_MAX_AGGREGATE_READ_QUEUE_BYTES, PHASE94_MAX_AGGREGATE_WRITE_QUEUE_BYTES,
    PHASE94_MAX_CONNECTIONS_PER_CHURN_WINDOW, PHASE94_MAX_HEADER_LOCATOR_HASHES,
    PHASE94_MAX_INBOUND_BLOCK_REQUESTS_PER_PEER, PHASE94_MAX_INBOUND_REQUEST_INVENTORY_ITEMS,
    PHASE94_MAX_INBOUND_RUNTIME_PAYLOAD_BYTES, PHASE94_MAX_INBOUND_TX_REQUESTS_PER_PEER,
    PHASE94_MAX_PEER_QUEUED_MESSAGES, PHASE94_MAX_PEER_READ_QUEUE_BYTES,
    PHASE94_MAX_PEER_WRITE_QUEUE_BYTES, PHASE94_MAX_REPEATED_FAILURES_PER_WINDOW,
    PHASE94_REPEATED_FAILURE_WINDOW_SECONDS, PHASE94_SLOW_HANDSHAKE_TIMEOUT_SECONDS,
    QueuePressureInput, ReconnectSuppressionInput, RepeatedFailureInput, RequestPressureInput,
    ResourceGovernanceDecision, ResourceGovernancePolicy, ResourceGovernanceSource,
    ResourceLifecycleLabel, ResourcePressureLabel, ResourceTimeoutInput, ResourceViolationLabel,
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
