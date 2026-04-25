// Parity breadcrumbs:
// - packages/bitcoin-knots/src/bitcoind.cpp
// - packages/bitcoin-knots/src/rpc/protocol.h
// - packages/bitcoin-knots/src/rpc/request.cpp
// - packages/bitcoin-knots/src/rpc/server.cpp
// - packages/bitcoin-knots/src/rpc/blockchain.cpp
// - packages/bitcoin-knots/src/rpc/mempool.cpp
// - packages/bitcoin-knots/src/rpc/net.cpp
// - packages/bitcoin-knots/src/rpc/rawtransaction.cpp
// - packages/bitcoin-knots/test/functional/interface_rpc.py

use serde::{Deserialize, Serialize};

pub const HTTP_OK: u16 = 200;
pub const HTTP_BAD_REQUEST: u16 = 400;
pub const HTTP_UNAUTHORIZED: u16 = 401;
pub const HTTP_NOT_FOUND: u16 = 404;
pub const HTTP_INTERNAL_SERVER_ERROR: u16 = 500;
pub const HTTP_SERVICE_UNAVAILABLE: u16 = 503;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(into = "i32", try_from = "i32")]
pub enum RpcErrorCode {
    InvalidRequest,
    MethodNotFound,
    InvalidParams,
    InternalError,
    ParseError,
    MiscError,
    ForbiddenBySafeMode,
    WalletError,
    ClientNotConnected,
    WalletNotFound,
    WalletNotSpecified,
    VerifyRejected,
}

impl RpcErrorCode {
    pub const fn as_i32(self) -> i32 {
        match self {
            Self::InvalidRequest => -32600,
            Self::MethodNotFound => -32601,
            Self::InvalidParams => -32602,
            Self::InternalError => -32603,
            Self::ParseError => -32700,
            Self::MiscError => -1,
            Self::ForbiddenBySafeMode => -2,
            Self::WalletError => -4,
            Self::ClientNotConnected => -9,
            Self::WalletNotFound => -18,
            Self::WalletNotSpecified => -19,
            Self::VerifyRejected => -26,
        }
    }
}

impl From<RpcErrorCode> for i32 {
    fn from(value: RpcErrorCode) -> Self {
        value.as_i32()
    }
}

impl TryFrom<i32> for RpcErrorCode {
    type Error = i32;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            -32700 => Ok(Self::ParseError),
            -32603 => Ok(Self::InternalError),
            -32602 => Ok(Self::InvalidParams),
            -32601 => Ok(Self::MethodNotFound),
            -32600 => Ok(Self::InvalidRequest),
            -26 => Ok(Self::VerifyRejected),
            -19 => Ok(Self::WalletNotSpecified),
            -18 => Ok(Self::WalletNotFound),
            -9 => Ok(Self::ClientNotConnected),
            -4 => Ok(Self::WalletError),
            -2 => Ok(Self::ForbiddenBySafeMode),
            -1 => Ok(Self::MiscError),
            _ => Err(value),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RpcFailureKind {
    ParseError,
    InvalidRequest,
    MethodNotFound,
    InvalidParams,
    WalletError,
    ClientNotConnected,
    AuthenticationFailed,
    InternalError,
}

impl RpcFailureKind {
    pub const fn http_status(self) -> u16 {
        match self {
            Self::ParseError | Self::InvalidRequest | Self::InvalidParams => HTTP_BAD_REQUEST,
            Self::MethodNotFound => HTTP_NOT_FOUND,
            Self::WalletError | Self::InternalError => HTTP_INTERNAL_SERVER_ERROR,
            Self::AuthenticationFailed => HTTP_UNAUTHORIZED,
            Self::ClientNotConnected => HTTP_SERVICE_UNAVAILABLE,
        }
    }

    pub const fn default_error_code(self) -> Option<RpcErrorCode> {
        match self {
            Self::ParseError => Some(RpcErrorCode::ParseError),
            Self::InvalidRequest => Some(RpcErrorCode::InvalidRequest),
            Self::MethodNotFound => Some(RpcErrorCode::MethodNotFound),
            Self::InvalidParams => Some(RpcErrorCode::InvalidParams),
            Self::WalletError => Some(RpcErrorCode::WalletError),
            Self::ClientNotConnected => Some(RpcErrorCode::ClientNotConnected),
            Self::AuthenticationFailed => None,
            Self::InternalError => Some(RpcErrorCode::InternalError),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RpcErrorDetail {
    pub code: RpcErrorCode,
    pub message: String,
}

impl RpcErrorDetail {
    pub fn new(code: RpcErrorCode, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RpcFailure {
    pub kind: RpcFailureKind,
    pub maybe_detail: Option<RpcErrorDetail>,
}

impl RpcFailure {
    pub fn new(kind: RpcFailureKind, maybe_detail: Option<RpcErrorDetail>) -> Self {
        Self { kind, maybe_detail }
    }

    pub fn from_kind(kind: RpcFailureKind, message: impl Into<String>) -> Self {
        let maybe_detail = kind
            .default_error_code()
            .map(|code| RpcErrorDetail::new(code, message));
        Self::new(kind, maybe_detail)
    }

    pub fn parse_error(message: impl Into<String>) -> Self {
        Self::from_kind(RpcFailureKind::ParseError, message)
    }

    pub fn invalid_request(message: impl Into<String>) -> Self {
        Self::from_kind(RpcFailureKind::InvalidRequest, message)
    }

    pub fn method_not_found(name: &str) -> Self {
        Self::from_kind(
            RpcFailureKind::MethodNotFound,
            format!("method {name} is not supported in Phase 8"),
        )
    }

    pub fn invalid_params(message: impl Into<String>) -> Self {
        Self::from_kind(RpcFailureKind::InvalidParams, message)
    }

    pub fn wallet_error(message: impl Into<String>) -> Self {
        Self::from_kind(RpcFailureKind::WalletError, message)
    }

    pub fn client_not_connected(message: impl Into<String>) -> Self {
        Self::from_kind(RpcFailureKind::ClientNotConnected, message)
    }

    pub fn authentication_failed() -> Self {
        Self::new(RpcFailureKind::AuthenticationFailed, None)
    }

    pub fn internal_error(message: impl Into<String>) -> Self {
        Self::from_kind(RpcFailureKind::InternalError, message)
    }

    pub fn verify_rejected(message: impl Into<String>) -> Self {
        Self::new(
            RpcFailureKind::InvalidParams,
            Some(RpcErrorDetail::new(RpcErrorCode::VerifyRejected, message)),
        )
    }

    pub fn http_status(&self) -> u16 {
        self.kind.http_status()
    }
}
