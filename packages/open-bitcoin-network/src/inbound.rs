// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net.cpp
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/test/functional/p2p_handshake.py

use core::net::SocketAddr;
use std::collections::BTreeSet;

use crate::error::PeerId;

mod permissions;

pub use permissions::{
    INBOUND_PERMISSION_ADDRESSES_FIELD, INBOUND_PERMISSION_CLASS_NAME_FIELD,
    INBOUND_PERMISSION_TOKENS_FIELD, InactivePermissionEffectLabel, InboundPermissionDecision,
    ParsedPeerPermissionClass, PeerConnectionClass, PeerPermissionClassRegistry,
    PeerPermissionDirection, PeerPermissionParseError, PeerPermissionSet, PeerPermissionToken,
    PermissionClassName, PermissionEffectLabel,
};

pub const INBOUND_LISTEN_ADDRESSES_FIELD: &str = "inbound.listen_addresses";
pub const INBOUND_ENABLED_FIELD: &str = "inbound.enabled";

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct InboundListenerConfig {
    pub enabled: bool,
    pub listen_addresses: Vec<String>,
    pub max_peers: usize,
    pub reserved_slots: usize,
    pub allow_public: bool,
    pub permission_classes: PeerPermissionClassRegistry,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InboundPreflightReason {
    Disabled,
    NoListenAddresses,
    InvalidEndpoint,
    UnsafeEndpoint,
    BindUnavailable,
    AlreadyBound,
    Ready,
}

impl InboundPreflightReason {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Disabled => "disabled",
            Self::NoListenAddresses => "no_listen_addresses",
            Self::InvalidEndpoint => "invalid_endpoint",
            Self::UnsafeEndpoint => "unsafe_endpoint",
            Self::BindUnavailable => "bind_unavailable",
            Self::AlreadyBound => "already_bound",
            Self::Ready => "ready",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InboundPreflightDiagnostic {
    pub reason: InboundPreflightReason,
    pub maybe_endpoint: Option<String>,
    pub field: &'static str,
    pub message: String,
    pub next_action: String,
}

impl InboundPreflightDiagnostic {
    fn new(
        reason: InboundPreflightReason,
        maybe_endpoint: Option<String>,
        field: &'static str,
        message: impl Into<String>,
        next_action: impl Into<String>,
    ) -> Self {
        Self {
            reason,
            maybe_endpoint,
            field,
            message: message.into(),
            next_action: next_action.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InboundListenerEndpoint {
    pub raw: String,
    pub normalized: String,
    pub address: SocketAddr,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InboundPreflightPlan {
    reason: InboundPreflightReason,
    diagnostics: Vec<InboundPreflightDiagnostic>,
    ready_endpoints: Vec<InboundListenerEndpoint>,
}

impl InboundPreflightPlan {
    pub fn blocked(diagnostic: InboundPreflightDiagnostic) -> Self {
        Self {
            reason: diagnostic.reason,
            diagnostics: vec![diagnostic],
            ready_endpoints: Vec::new(),
        }
    }

    pub fn ready(endpoints: Vec<InboundListenerEndpoint>) -> Self {
        Self {
            reason: InboundPreflightReason::Ready,
            diagnostics: vec![InboundPreflightDiagnostic::new(
                InboundPreflightReason::Ready,
                None,
                INBOUND_LISTEN_ADDRESSES_FIELD,
                "inbound listener preflight is ready",
                "bind the normalized listener endpoints in the runtime activation stage",
            )],
            ready_endpoints: endpoints,
        }
    }

    pub const fn reason(&self) -> InboundPreflightReason {
        self.reason
    }

    pub fn diagnostics(&self) -> &[InboundPreflightDiagnostic] {
        &self.diagnostics
    }

    pub fn ready_endpoints(&self) -> &[InboundListenerEndpoint] {
        &self.ready_endpoints
    }

    pub fn should_attempt_bind(&self) -> bool {
        self.reason == InboundPreflightReason::Ready && !self.ready_endpoints.is_empty()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InboundListenerActivationDiagnostic {
    pub reason: InboundPreflightReason,
    pub maybe_endpoint: Option<String>,
    pub field: &'static str,
    pub message: String,
    pub next_action: String,
}

impl InboundListenerActivationDiagnostic {
    pub fn bind_unavailable(endpoint: &InboundListenerEndpoint, detail: impl Into<String>) -> Self {
        let detail = detail.into();
        Self {
            reason: InboundPreflightReason::BindUnavailable,
            maybe_endpoint: Some(endpoint.normalized.clone()),
            field: INBOUND_LISTEN_ADDRESSES_FIELD,
            message: format!(
                "inbound listener endpoint {} could not be bound: {detail}",
                endpoint.normalized
            ),
            next_action: "choose a local address that is available to this process".to_string(),
        }
    }

    pub fn already_bound(endpoint: &InboundListenerEndpoint, detail: impl Into<String>) -> Self {
        let detail = detail.into();
        Self {
            reason: InboundPreflightReason::AlreadyBound,
            maybe_endpoint: Some(endpoint.normalized.clone()),
            field: INBOUND_LISTEN_ADDRESSES_FIELD,
            message: format!(
                "inbound listener endpoint {} is already bound: {detail}",
                endpoint.normalized
            ),
            next_action: "stop the conflicting process or choose a different loopback port"
                .to_string(),
        }
    }

    pub fn into_preflight_diagnostic(self) -> InboundPreflightDiagnostic {
        InboundPreflightDiagnostic {
            reason: self.reason,
            maybe_endpoint: self.maybe_endpoint,
            field: self.field,
            message: self.message,
            next_action: self.next_action,
        }
    }
}

pub fn classify_inbound_preflight(config: &InboundListenerConfig) -> InboundPreflightPlan {
    if !config.enabled {
        return InboundPreflightPlan::blocked(InboundPreflightDiagnostic::new(
            InboundPreflightReason::Disabled,
            None,
            INBOUND_ENABLED_FIELD,
            "inbound serving is disabled",
            "set inbound.enabled to true and configure inbound.listen_addresses to enable listener preflight",
        ));
    }

    if config.listen_addresses.is_empty() {
        return InboundPreflightPlan::blocked(InboundPreflightDiagnostic::new(
            InboundPreflightReason::NoListenAddresses,
            None,
            INBOUND_LISTEN_ADDRESSES_FIELD,
            "inbound serving is enabled but no listener endpoints are configured",
            "add at least one loopback host:port value to inbound.listen_addresses",
        ));
    }

    let mut endpoints = Vec::with_capacity(config.listen_addresses.len());
    for raw_endpoint in &config.listen_addresses {
        let endpoint = match parse_listener_endpoint(raw_endpoint) {
            Ok(endpoint) => endpoint,
            Err(diagnostic) => return InboundPreflightPlan::blocked(diagnostic),
        };
        if !config.allow_public && !endpoint.address.ip().is_loopback() {
            return InboundPreflightPlan::blocked(InboundPreflightDiagnostic::new(
                InboundPreflightReason::UnsafeEndpoint,
                Some(endpoint.normalized),
                INBOUND_LISTEN_ADDRESSES_FIELD,
                "inbound listener endpoint is not loopback",
                "use a loopback endpoint or set inbound.allow_public to true after reviewing exposure",
            ));
        }
        endpoints.push(endpoint);
    }

    InboundPreflightPlan::ready(endpoints)
}

fn parse_listener_endpoint(
    raw_endpoint: &str,
) -> Result<InboundListenerEndpoint, InboundPreflightDiagnostic> {
    let trimmed = raw_endpoint.trim();
    let address = trimmed
        .parse::<SocketAddr>()
        .map_err(|_error| invalid_endpoint_diagnostic(raw_endpoint))?;

    Ok(InboundListenerEndpoint {
        raw: raw_endpoint.to_string(),
        normalized: address.to_string(),
        address,
    })
}

fn invalid_endpoint_diagnostic(raw_endpoint: &str) -> InboundPreflightDiagnostic {
    InboundPreflightDiagnostic::new(
        InboundPreflightReason::InvalidEndpoint,
        Some(raw_endpoint.to_string()),
        INBOUND_LISTEN_ADDRESSES_FIELD,
        "inbound listener endpoint must be a literal host:port endpoint",
        "set inbound.listen_addresses entries to literal host:port endpoints such as 127.0.0.1:8333 or [::1]:8333",
    )
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InboundAdmissionSlotClass {
    Ordinary,
    Reserved,
}

impl InboundAdmissionSlotClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Ordinary => "ordinary",
            Self::Reserved => "reserved",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InboundHandshakeState {
    Accepted,
    Handshaking,
    Established,
    Disconnected,
}

impl InboundHandshakeState {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Accepted => "accepted",
            Self::Handshaking => "handshaking",
            Self::Established => "established",
            Self::Disconnected => "disconnected",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct InboundAdmissionCounters {
    pub current_inbound_peers: usize,
    pub current_outbound_peers: usize,
    pub current_reserved_inbound_peers: usize,
}

impl InboundAdmissionCounters {
    pub const fn after_admitted(self, slot_class: InboundAdmissionSlotClass) -> Self {
        let current_reserved_inbound_peers = match slot_class {
            InboundAdmissionSlotClass::Ordinary => self.current_reserved_inbound_peers,
            InboundAdmissionSlotClass::Reserved => self.current_reserved_inbound_peers + 1,
        };
        Self {
            current_inbound_peers: self.current_inbound_peers + 1,
            current_outbound_peers: self.current_outbound_peers,
            current_reserved_inbound_peers,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InboundAdmissionRequest {
    pub peer_id: PeerId,
    pub remote_endpoint: String,
    pub slot_class: InboundAdmissionSlotClass,
    pub connection_class: PeerConnectionClass,
    pub permission_decision: InboundPermissionDecision,
    pub counters: InboundAdmissionCounters,
    pub existing_endpoint_keys: BTreeSet<String>,
    pub existing_peer_ids: BTreeSet<PeerId>,
    pub local_nonce: u64,
    pub maybe_remote_nonce: Option<u64>,
    pub is_shutdown_requested: bool,
}

impl InboundAdmissionRequest {
    pub fn ordinary(peer_id: PeerId, remote_endpoint: impl Into<String>) -> Self {
        Self::from_permission_decision(
            peer_id,
            remote_endpoint,
            InboundPermissionDecision::ordinary(),
        )
    }

    pub fn from_permission_decision(
        peer_id: PeerId,
        remote_endpoint: impl Into<String>,
        permission_decision: InboundPermissionDecision,
    ) -> Self {
        let connection_class = permission_decision.connection_class();
        let slot_class = permission_decision.slot_class();
        Self {
            peer_id,
            remote_endpoint: remote_endpoint.into(),
            slot_class,
            connection_class,
            permission_decision,
            counters: InboundAdmissionCounters::default(),
            existing_endpoint_keys: BTreeSet::new(),
            existing_peer_ids: BTreeSet::new(),
            local_nonce: 0,
            maybe_remote_nonce: None,
            is_shutdown_requested: false,
        }
    }

    pub fn set_permission_decision(&mut self, permission_decision: InboundPermissionDecision) {
        self.connection_class = permission_decision.connection_class();
        self.slot_class = permission_decision.slot_class();
        self.permission_decision = permission_decision;
    }

    pub const fn effective_slot_class(&self) -> InboundAdmissionSlotClass {
        self.permission_decision.slot_class()
    }

    pub fn set_existing_identities(&mut self, identities: BTreeSet<PeerId>) {
        self.existing_peer_ids = identities;
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InboundPeerRecord {
    pub peer_id: PeerId,
    pub remote_endpoint: String,
    pub slot_class: InboundAdmissionSlotClass,
    pub connection_class: PeerConnectionClass,
    pub permission_decision: InboundPermissionDecision,
    pub handshake_state: InboundHandshakeState,
    pub maybe_remote_nonce: Option<u64>,
    pub observed_inbound_peers: usize,
    pub observed_outbound_peers: usize,
}

impl InboundPeerRecord {
    pub const fn identity(&self) -> PeerId {
        self.peer_id
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InboundAdmissionRejectionReason {
    CapReached,
    ReservedSlotUnavailable,
    DuplicateEndpoint,
    DuplicatePeerId,
    SelfConnection,
    Shutdown,
}

impl InboundAdmissionRejectionReason {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CapReached => "cap_reached",
            Self::ReservedSlotUnavailable => "reserved_slot_unavailable",
            Self::DuplicateEndpoint => "duplicate_endpoint",
            Self::DuplicatePeerId => "duplicate_peer_id",
            Self::SelfConnection => "self_connection",
            Self::Shutdown => "shutdown",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InboundAdmissionRejection {
    pub reason: InboundAdmissionRejectionReason,
    pub peer_id: PeerId,
    pub slot_class: InboundAdmissionSlotClass,
    pub maybe_endpoint: Option<String>,
    pub message: String,
    pub next_action: String,
}

impl InboundAdmissionRejection {
    pub fn runtime_self_connection(record: &InboundPeerRecord) -> Self {
        Self {
            reason: InboundAdmissionRejectionReason::SelfConnection,
            peer_id: record.peer_id,
            slot_class: record.slot_class,
            maybe_endpoint: Some(record.remote_endpoint.clone()),
            message: "remote peer nonce matches the local peer nonce".to_string(),
            next_action: "disconnect the self-connection candidate before continuing".to_string(),
        }
    }

    pub fn duplicate_identity(record: &InboundPeerRecord) -> Self {
        Self {
            reason: InboundAdmissionRejectionReason::DuplicatePeerId,
            peer_id: record.peer_id,
            slot_class: record.slot_class,
            maybe_endpoint: Some(record.remote_endpoint.clone()),
            message: "inbound peer id already has an admitted peer record".to_string(),
            next_action: "allocate a fresh peer id before retrying admission".to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InboundAdmissionDecision {
    Admit(InboundPeerRecord),
    Reject(InboundAdmissionRejection),
}

impl InboundAdmissionDecision {
    pub const fn is_admitted(&self) -> bool {
        matches!(self, Self::Admit(_))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InboundAdmissionPolicy {
    pub max_inbound_peers: usize,
    pub reserved_slots: usize,
}

impl InboundAdmissionPolicy {
    pub const fn new(max_inbound_peers: usize, reserved_slots: usize) -> Self {
        Self {
            max_inbound_peers,
            reserved_slots,
        }
    }

    pub const fn effective_reserved_slots(self) -> usize {
        if self.reserved_slots > self.max_inbound_peers {
            self.max_inbound_peers
        } else {
            self.reserved_slots
        }
    }

    pub const fn ordinary_capacity(self) -> usize {
        self.max_inbound_peers - self.effective_reserved_slots()
    }

    pub fn decide(self, request: InboundAdmissionRequest) -> InboundAdmissionDecision {
        if request.is_shutdown_requested {
            return reject_admission(&request, InboundAdmissionRejectionReason::Shutdown);
        }

        if request
            .existing_endpoint_keys
            .contains(&request.remote_endpoint)
        {
            return reject_admission(&request, InboundAdmissionRejectionReason::DuplicateEndpoint);
        }

        if request.existing_peer_ids.contains(&request.peer_id) {
            return reject_admission(&request, InboundAdmissionRejectionReason::DuplicatePeerId);
        }

        if request.maybe_remote_nonce == Some(request.local_nonce) {
            return reject_admission(&request, InboundAdmissionRejectionReason::SelfConnection);
        }

        match request.effective_slot_class() {
            InboundAdmissionSlotClass::Ordinary => self.decide_ordinary(request),
            InboundAdmissionSlotClass::Reserved => self.decide_reserved(request),
        }
    }

    fn decide_ordinary(self, request: InboundAdmissionRequest) -> InboundAdmissionDecision {
        if request.counters.current_inbound_peers >= self.max_inbound_peers {
            return reject_admission(&request, InboundAdmissionRejectionReason::CapReached);
        }

        let ordinary_in_use = request
            .counters
            .current_inbound_peers
            .saturating_sub(reserved_inbound_in_use(request.counters));
        if ordinary_in_use >= self.ordinary_capacity() {
            return reject_admission(
                &request,
                InboundAdmissionRejectionReason::ReservedSlotUnavailable,
            );
        }

        admit_request(request)
    }

    fn decide_reserved(self, request: InboundAdmissionRequest) -> InboundAdmissionDecision {
        let reserved_capacity = self.effective_reserved_slots();
        let reserved_in_use = reserved_inbound_in_use(request.counters);
        if reserved_capacity == 0
            || reserved_in_use >= reserved_capacity
            || request.counters.current_inbound_peers >= self.max_inbound_peers
        {
            return reject_admission(
                &request,
                InboundAdmissionRejectionReason::ReservedSlotUnavailable,
            );
        }

        admit_request(request)
    }
}

fn reserved_inbound_in_use(counters: InboundAdmissionCounters) -> usize {
    counters
        .current_reserved_inbound_peers
        .min(counters.current_inbound_peers)
}

fn admit_request(request: InboundAdmissionRequest) -> InboundAdmissionDecision {
    let connection_class = request.permission_decision.connection_class();
    let slot_class = request.permission_decision.slot_class();
    InboundAdmissionDecision::Admit(InboundPeerRecord {
        peer_id: request.peer_id,
        remote_endpoint: request.remote_endpoint,
        slot_class,
        connection_class,
        permission_decision: request.permission_decision,
        handshake_state: InboundHandshakeState::Accepted,
        maybe_remote_nonce: request.maybe_remote_nonce,
        observed_inbound_peers: request.counters.current_inbound_peers,
        observed_outbound_peers: request.counters.current_outbound_peers,
    })
}

fn reject_admission(
    request: &InboundAdmissionRequest,
    reason: InboundAdmissionRejectionReason,
) -> InboundAdmissionDecision {
    let (message, next_action) = rejection_text(reason);
    InboundAdmissionDecision::Reject(InboundAdmissionRejection {
        reason,
        peer_id: request.peer_id,
        slot_class: request.effective_slot_class(),
        maybe_endpoint: Some(request.remote_endpoint.clone()),
        message,
        next_action,
    })
}

fn rejection_text(reason: InboundAdmissionRejectionReason) -> (String, String) {
    match reason {
        InboundAdmissionRejectionReason::CapReached => (
            "inbound peer cap has been reached".to_string(),
            "increase inbound.max_peers or wait for an inbound peer to disconnect".to_string(),
        ),
        InboundAdmissionRejectionReason::ReservedSlotUnavailable => (
            "reserved inbound admission capacity is unavailable".to_string(),
            "retry as an ordinary inbound candidate or wait for reserved capacity".to_string(),
        ),
        InboundAdmissionRejectionReason::DuplicateEndpoint => (
            "inbound endpoint already has an admitted peer record".to_string(),
            "close the existing connection before admitting this endpoint again".to_string(),
        ),
        InboundAdmissionRejectionReason::DuplicatePeerId => (
            "inbound peer id already has an admitted peer record".to_string(),
            "allocate a fresh peer id before retrying admission".to_string(),
        ),
        InboundAdmissionRejectionReason::SelfConnection => (
            "remote peer nonce matches the local peer nonce".to_string(),
            "disconnect the self-connection candidate before peer insertion".to_string(),
        ),
        InboundAdmissionRejectionReason::Shutdown => (
            "inbound admission is closed for shutdown".to_string(),
            "wait for shutdown to complete before opening new inbound connections".to_string(),
        ),
    }
}

#[cfg(test)]
mod tests;
