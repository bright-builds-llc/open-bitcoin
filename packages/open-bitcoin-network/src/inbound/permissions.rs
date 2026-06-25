// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_permissions.h
// - packages/bitcoin-knots/src/net_permissions.cpp
// - packages/bitcoin-knots/test/functional/p2p_permissions.py

use std::collections::BTreeSet;
use std::fmt;

pub const INBOUND_PERMISSION_TOKENS_FIELD: &str = "inbound.permission_classes[].permissions[]";

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
pub struct PeerPermissionParseError {
    field: &'static str,
    token: String,
    reason: PeerPermissionParseErrorReason,
    message: String,
}

impl PeerPermissionParseError {
    pub fn unsupported_token(field: &'static str, token: impl Into<String>) -> Self {
        let token = token.into();
        Self {
            field,
            token: token.clone(),
            reason: PeerPermissionParseErrorReason::UnsupportedToken,
            message: format!("unsupported peer permission token: {token}"),
        }
    }

    pub const fn field(&self) -> &'static str {
        self.field
    }

    pub fn token(&self) -> &str {
        &self.token
    }

    pub const fn reason(&self) -> &'static str {
        self.reason.as_str()
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for PeerPermissionParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{} contains invalid token {}: {}",
            self.field, self.token, self.message
        )
    }
}

impl std::error::Error for PeerPermissionParseError {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PeerPermissionParseErrorReason {
    UnsupportedToken,
}

impl PeerPermissionParseErrorReason {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UnsupportedToken => "unsupported_token",
        }
    }
}
