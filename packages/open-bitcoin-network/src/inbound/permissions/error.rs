// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_permissions.h
// - packages/bitcoin-knots/src/net_permissions.cpp
// - packages/bitcoin-knots/test/functional/p2p_permissions.py

use std::fmt;

use super::INBOUND_PERMISSION_ADDRESSES_FIELD;

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

    pub fn direction_only(field: &'static str, token: impl Into<String>) -> Self {
        let token = token.into();
        Self {
            field,
            token: token.clone(),
            reason: PeerPermissionParseErrorReason::DirectionOnly,
            message: format!("peer permission class sets only direction token: {token}"),
        }
    }

    pub fn missing_inbound_direction(field: &'static str) -> Self {
        Self {
            field,
            token: "in".to_string(),
            reason: PeerPermissionParseErrorReason::MissingInboundDirection,
            message: "peer permission class must include the in direction".to_string(),
        }
    }

    pub fn outbound_direction_unsupported(field: &'static str) -> Self {
        Self {
            field,
            token: "out".to_string(),
            reason: PeerPermissionParseErrorReason::OutboundDirectionUnsupported,
            message: "peer permission class cannot include the out direction in Phase 91"
                .to_string(),
        }
    }

    pub fn invalid_address(token: impl Into<String>) -> Self {
        let token = token.into();
        Self {
            field: INBOUND_PERMISSION_ADDRESSES_FIELD,
            token: token.clone(),
            reason: PeerPermissionParseErrorReason::InvalidLiteralIpAddress,
            message: format!("peer permission class address must be a literal IP address: {token}"),
        }
    }

    pub fn empty_class_name(field: &'static str) -> Self {
        Self {
            field,
            token: String::new(),
            reason: PeerPermissionParseErrorReason::EmptyClassName,
            message: "peer permission class name must not be empty".to_string(),
        }
    }

    pub fn empty_address_list(field: &'static str) -> Self {
        Self {
            field,
            token: String::new(),
            reason: PeerPermissionParseErrorReason::EmptyAddressList,
            message: "peer permission class must include at least one literal IP address"
                .to_string(),
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
    DirectionOnly,
    MissingInboundDirection,
    OutboundDirectionUnsupported,
    InvalidLiteralIpAddress,
    EmptyClassName,
    EmptyAddressList,
}

impl PeerPermissionParseErrorReason {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UnsupportedToken => "unsupported_token",
            Self::DirectionOnly => "direction_only",
            Self::MissingInboundDirection => "missing_inbound_direction",
            Self::OutboundDirectionUnsupported => "outbound_direction_unsupported",
            Self::InvalidLiteralIpAddress => "invalid_literal_ip_address",
            Self::EmptyClassName => "empty_class_name",
            Self::EmptyAddressList => "empty_address_list",
        }
    }
}
