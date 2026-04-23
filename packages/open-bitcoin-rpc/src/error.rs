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
    WalletRequired,
    AuthenticationFailed,
    ClientNotConnected,
    InternalError,
}

impl RpcFailureKind {
    pub const fn maybe_error_code(self) -> Option<RpcErrorCode> {
        match self {
            Self::ParseError => Some(RpcErrorCode::ParseError),
            Self::InvalidRequest => Some(RpcErrorCode::InvalidRequest),
            Self::MethodNotFound => Some(RpcErrorCode::MethodNotFound),
            Self::InvalidParams => Some(RpcErrorCode::InvalidParams),
            Self::WalletRequired => Some(RpcErrorCode::WalletNotSpecified),
            // Knots rejects missing or invalid HTTP auth at the transport layer with 401.
            Self::AuthenticationFailed => None,
            Self::ClientNotConnected => Some(RpcErrorCode::ClientNotConnected),
            Self::InternalError => Some(RpcErrorCode::InternalError),
        }
    }

    pub const fn http_status(self) -> u16 {
        match self {
            Self::ParseError | Self::InvalidRequest | Self::InvalidParams => HTTP_BAD_REQUEST,
            Self::MethodNotFound => HTTP_NOT_FOUND,
            Self::WalletRequired | Self::InternalError => HTTP_INTERNAL_SERVER_ERROR,
            Self::AuthenticationFailed => HTTP_UNAUTHORIZED,
            Self::ClientNotConnected => HTTP_SERVICE_UNAVAILABLE,
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

    pub fn maybe_from_kind(kind: RpcFailureKind, message: impl Into<String>) -> Option<Self> {
        let maybe_code = kind.maybe_error_code()?;
        Some(Self::new(maybe_code, message))
    }
}
