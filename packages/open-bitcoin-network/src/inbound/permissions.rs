// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_permissions.h
// - packages/bitcoin-knots/src/net_permissions.cpp
// - packages/bitcoin-knots/test/functional/p2p_permissions.py

mod error;

use std::collections::BTreeSet;
use std::net::IpAddr;

use super::InboundAdmissionSlotClass;
pub use error::PeerPermissionParseError;

pub const INBOUND_PERMISSION_TOKENS_FIELD: &str = "inbound.permission_classes[].permissions[]";
pub const INBOUND_PERMISSION_ADDRESSES_FIELD: &str = "inbound.permission_classes[].addresses[]";
pub const INBOUND_PERMISSION_CLASS_NAME_FIELD: &str = "inbound.permission_classes[].name";

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PeerPermissionToken {
    BloomFilter,
    BlockFilters,
    NoBan,
    ForceRelay,
    Relay,
    Mempool,
    Download,
    Addr,
    ForceInbound,
    All,
}

impl PeerPermissionToken {
    pub fn parse(
        field: &'static str,
        token: impl AsRef<str>,
    ) -> Result<Self, PeerPermissionParseError> {
        let token = token.as_ref();
        match token {
            "bloomfilter" => Ok(Self::BloomFilter),
            "blockfilters" => Ok(Self::BlockFilters),
            "noban" => Ok(Self::NoBan),
            "forcerelay" => Ok(Self::ForceRelay),
            "relay" => Ok(Self::Relay),
            "mempool" => Ok(Self::Mempool),
            "download" => Ok(Self::Download),
            "addr" => Ok(Self::Addr),
            "forceinbound" => Ok(Self::ForceInbound),
            "all" => Ok(Self::All),
            _ => Err(PeerPermissionParseError::unsupported_token(field, token)),
        }
    }

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BloomFilter => "bloomfilter",
            Self::BlockFilters => "blockfilters",
            Self::NoBan => "noban",
            Self::ForceRelay => "forcerelay",
            Self::Relay => "relay",
            Self::Mempool => "mempool",
            Self::Download => "download",
            Self::Addr => "addr",
            Self::ForceInbound => "forceinbound",
            Self::All => "all",
        }
    }

    const fn all_expansion() -> [Self; 9] {
        [
            Self::BloomFilter,
            Self::BlockFilters,
            Self::NoBan,
            Self::ForceRelay,
            Self::Relay,
            Self::Mempool,
            Self::Download,
            Self::Addr,
            Self::ForceInbound,
        ]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PeerPermissionDirection {
    Inbound,
    Outbound,
}

impl PeerPermissionDirection {
    pub fn parse(
        field: &'static str,
        token: impl AsRef<str>,
    ) -> Result<Self, PeerPermissionParseError> {
        let token = token.as_ref();
        match token {
            "in" => Ok(Self::Inbound),
            "out" => Ok(Self::Outbound),
            _ => Err(PeerPermissionParseError::unsupported_token(field, token)),
        }
    }

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Inbound => "in",
            Self::Outbound => "out",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PermissionEffectLabel {
    AdmissionProtected,
    EvictionPolicyProtected,
    MisbehaviorPolicyProtected,
    AddressResponsePolicyInput,
    DownloadServingPolicyInput,
}

impl PermissionEffectLabel {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AdmissionProtected => "admission_protected",
            Self::EvictionPolicyProtected => "eviction_policy_protected",
            Self::MisbehaviorPolicyProtected => "misbehavior_policy_protected",
            Self::AddressResponsePolicyInput => "address_response_policy_input",
            Self::DownloadServingPolicyInput => "download_serving_policy_input",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InactivePermissionEffectLabel {
    Relay,
    ForceRelay,
    Mempool,
    BloomFilter,
    BlockFilters,
}

impl InactivePermissionEffectLabel {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Relay => "inactive_relay",
            Self::ForceRelay => "inactive_forcerelay",
            Self::Mempool => "inactive_mempool",
            Self::BloomFilter => "inactive_bloomfilter",
            Self::BlockFilters => "inactive_blockfilters",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct PeerPermissionSet {
    requested_tokens: BTreeSet<PeerPermissionToken>,
    permissions: BTreeSet<PeerPermissionToken>,
    directions: BTreeSet<PeerPermissionDirection>,
}

impl PeerPermissionSet {
    pub fn parse<I, S>(field: &'static str, tokens: I) -> Result<Self, PeerPermissionParseError>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let mut set = Self::default();
        for token in tokens {
            set.insert_raw_token(field, token.as_ref())?;
        }
        Ok(set)
    }

    pub fn contains_token(&self, token: PeerPermissionToken) -> bool {
        if token == PeerPermissionToken::All {
            return self.requested_tokens.contains(&PeerPermissionToken::All);
        }
        self.permissions.contains(&token)
    }

    pub fn has_direction(&self, direction: PeerPermissionDirection) -> bool {
        self.directions.contains(&direction)
    }

    pub fn has_any_permission(&self) -> bool {
        !self.permissions.is_empty()
    }

    pub fn is_admission_protected(&self) -> bool {
        self.permissions
            .contains(&PeerPermissionToken::ForceInbound)
    }

    pub fn active_effects(&self) -> Vec<PermissionEffectLabel> {
        let mut effects = Vec::new();
        if self
            .permissions
            .contains(&PeerPermissionToken::ForceInbound)
        {
            effects.push(PermissionEffectLabel::AdmissionProtected);
        }
        if self.permissions.contains(&PeerPermissionToken::NoBan) {
            effects.push(PermissionEffectLabel::EvictionPolicyProtected);
            effects.push(PermissionEffectLabel::MisbehaviorPolicyProtected);
        }
        if self.permissions.contains(&PeerPermissionToken::Addr) {
            effects.push(PermissionEffectLabel::AddressResponsePolicyInput);
        }
        if self.permissions.contains(&PeerPermissionToken::Download) {
            effects.push(PermissionEffectLabel::DownloadServingPolicyInput);
        }
        effects
    }

    pub fn inactive_effects(&self) -> Vec<InactivePermissionEffectLabel> {
        let mut effects = Vec::new();
        if self.permissions.contains(&PeerPermissionToken::Relay) {
            effects.push(InactivePermissionEffectLabel::Relay);
        }
        if self.permissions.contains(&PeerPermissionToken::ForceRelay) {
            effects.push(InactivePermissionEffectLabel::ForceRelay);
        }
        if self.permissions.contains(&PeerPermissionToken::Mempool) {
            effects.push(InactivePermissionEffectLabel::Mempool);
        }
        if self.permissions.contains(&PeerPermissionToken::BloomFilter) {
            effects.push(InactivePermissionEffectLabel::BloomFilter);
        }
        if self
            .permissions
            .contains(&PeerPermissionToken::BlockFilters)
        {
            effects.push(InactivePermissionEffectLabel::BlockFilters);
        }
        effects
    }

    fn insert_raw_token(
        &mut self,
        field: &'static str,
        token: &str,
    ) -> Result<(), PeerPermissionParseError> {
        if let Ok(direction) = PeerPermissionDirection::parse(field, token) {
            self.directions.insert(direction);
            return Ok(());
        }

        let permission = PeerPermissionToken::parse(field, token)?;
        self.insert_permission(permission);
        Ok(())
    }

    fn insert_permission(&mut self, permission: PeerPermissionToken) {
        self.requested_tokens.insert(permission);
        match permission {
            PeerPermissionToken::All => {
                for expanded in PeerPermissionToken::all_expansion() {
                    self.insert_expanded_permission(expanded);
                }
            }
            _ => self.insert_expanded_permission(permission),
        }
    }

    fn insert_expanded_permission(&mut self, permission: PeerPermissionToken) {
        match permission {
            PeerPermissionToken::ForceRelay => {
                self.permissions.insert(PeerPermissionToken::ForceRelay);
                self.permissions.insert(PeerPermissionToken::Relay);
            }
            PeerPermissionToken::NoBan => {
                self.permissions.insert(PeerPermissionToken::NoBan);
                self.permissions.insert(PeerPermissionToken::Download);
            }
            PeerPermissionToken::ForceInbound => {
                self.permissions.insert(PeerPermissionToken::ForceInbound);
                self.permissions.insert(PeerPermissionToken::NoBan);
                self.permissions.insert(PeerPermissionToken::Download);
            }
            PeerPermissionToken::All => {}
            _ => {
                self.permissions.insert(permission);
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PermissionClassName(String);

impl PermissionClassName {
    pub fn parse(
        field: &'static str,
        name: impl AsRef<str>,
    ) -> Result<Self, PeerPermissionParseError> {
        let name = name.as_ref().trim();
        if name.is_empty() {
            return Err(PeerPermissionParseError::empty_class_name(field));
        }

        Ok(Self(name.to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsedPeerPermissionClass {
    name: PermissionClassName,
    addresses: Vec<IpAddr>,
    permissions: PeerPermissionSet,
}

impl ParsedPeerPermissionClass {
    pub fn parse<A, P, AS, PS>(
        name: impl AsRef<str>,
        addresses: A,
        permissions: P,
    ) -> Result<Self, PeerPermissionParseError>
    where
        A: IntoIterator<Item = AS>,
        AS: AsRef<str>,
        P: IntoIterator<Item = PS>,
        PS: AsRef<str>,
    {
        let name = PermissionClassName::parse(INBOUND_PERMISSION_CLASS_NAME_FIELD, name)?;
        let permissions = PeerPermissionSet::parse(INBOUND_PERMISSION_TOKENS_FIELD, permissions)?;
        validate_inbound_class_permissions(&permissions)?;
        let addresses = parse_literal_ip_addresses(addresses)?;

        Ok(Self {
            name,
            addresses,
            permissions,
        })
    }

    pub fn name(&self) -> &PermissionClassName {
        &self.name
    }

    pub fn permissions(&self) -> &PeerPermissionSet {
        &self.permissions
    }

    pub fn addresses(&self) -> &[IpAddr] {
        &self.addresses
    }

    fn matches_inbound(&self, remote_address: IpAddr) -> bool {
        self.addresses.contains(&remote_address)
    }

    fn inbound_decision(&self) -> InboundPermissionDecision {
        let connection_class = if self.permissions.is_admission_protected() {
            PeerConnectionClass::ProtectedInbound
        } else {
            PeerConnectionClass::PermissionedInbound
        };
        InboundPermissionDecision {
            connection_class,
            active_effects: self.permissions.active_effects(),
            inactive_effects: self.permissions.inactive_effects(),
        }
    }
}

fn validate_inbound_class_permissions(
    permissions: &PeerPermissionSet,
) -> Result<(), PeerPermissionParseError> {
    if permissions.has_direction(PeerPermissionDirection::Outbound) {
        return Err(PeerPermissionParseError::outbound_direction_unsupported(
            INBOUND_PERMISSION_TOKENS_FIELD,
        ));
    }

    if !permissions.has_any_permission() {
        return Err(PeerPermissionParseError::direction_only(
            INBOUND_PERMISSION_TOKENS_FIELD,
            "in",
        ));
    }

    if !permissions.has_direction(PeerPermissionDirection::Inbound) {
        return Err(PeerPermissionParseError::missing_inbound_direction(
            INBOUND_PERMISSION_TOKENS_FIELD,
        ));
    }

    Ok(())
}

fn parse_literal_ip_addresses<A, AS>(addresses: A) -> Result<Vec<IpAddr>, PeerPermissionParseError>
where
    A: IntoIterator<Item = AS>,
    AS: AsRef<str>,
{
    let mut parsed = Vec::new();
    for address in addresses {
        let raw_address = address.as_ref();
        let parsed_address = raw_address
            .trim()
            .parse::<IpAddr>()
            .map_err(|_error| PeerPermissionParseError::invalid_address(raw_address))?;
        parsed.push(parsed_address);
    }

    if parsed.is_empty() {
        return Err(PeerPermissionParseError::empty_address_list(
            INBOUND_PERMISSION_ADDRESSES_FIELD,
        ));
    }

    Ok(parsed)
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct PeerPermissionClassRegistry {
    classes: Vec<ParsedPeerPermissionClass>,
}

impl PeerPermissionClassRegistry {
    pub fn new<I>(classes: I) -> Self
    where
        I: IntoIterator<Item = ParsedPeerPermissionClass>,
    {
        Self {
            classes: classes.into_iter().collect(),
        }
    }

    pub fn resolve_inbound(&self, remote_address: IpAddr) -> InboundPermissionDecision {
        for permission_class in &self.classes {
            if permission_class.matches_inbound(remote_address) {
                return permission_class.inbound_decision();
            }
        }

        InboundPermissionDecision::ordinary()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PeerConnectionClass {
    OrdinaryInbound,
    PermissionedInbound,
    ProtectedInbound,
    Outbound,
    ManualConfigured,
}

impl PeerConnectionClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OrdinaryInbound => "ordinary_inbound",
            Self::PermissionedInbound => "permissioned_inbound",
            Self::ProtectedInbound => "protected_inbound",
            Self::Outbound => "outbound",
            Self::ManualConfigured => "manual_configured",
        }
    }

    pub const fn slot_class(self) -> InboundAdmissionSlotClass {
        match self {
            Self::ProtectedInbound => InboundAdmissionSlotClass::Reserved,
            Self::OrdinaryInbound
            | Self::PermissionedInbound
            | Self::Outbound
            | Self::ManualConfigured => InboundAdmissionSlotClass::Ordinary,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InboundPermissionDecision {
    connection_class: PeerConnectionClass,
    active_effects: Vec<PermissionEffectLabel>,
    inactive_effects: Vec<InactivePermissionEffectLabel>,
}

impl InboundPermissionDecision {
    pub fn ordinary() -> Self {
        Self {
            connection_class: PeerConnectionClass::OrdinaryInbound,
            active_effects: Vec::new(),
            inactive_effects: Vec::new(),
        }
    }

    pub const fn connection_class(&self) -> PeerConnectionClass {
        self.connection_class
    }

    pub const fn slot_class(&self) -> InboundAdmissionSlotClass {
        self.connection_class.slot_class()
    }

    pub fn active_effects(&self) -> &[PermissionEffectLabel] {
        &self.active_effects
    }

    pub fn inactive_effects(&self) -> &[InactivePermissionEffectLabel] {
        &self.inactive_effects
    }
}
