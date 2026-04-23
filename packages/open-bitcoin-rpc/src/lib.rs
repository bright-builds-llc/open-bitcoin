#![forbid(unsafe_code)]

//! Shell-layer RPC crate for Phase 8 typed contracts and runtime composition.

pub mod config;
pub mod context;
pub mod envelope;
pub mod error;
pub mod method;

pub use config::{
    RpcAuthConfig, RpcClientConfig, RpcServerConfig, RuntimeConfig, WalletRuntimeScope,
};
pub use context::ManagedRpcContext;
pub use envelope::{
    JsonRpcId, JsonRpcVersion, RpcErrorEnvelope, RpcRequestEnvelope, RpcSuccessEnvelope,
};
pub use error::{RpcErrorCode, RpcErrorDetail, RpcFailureKind};
pub use method::{MethodOrigin, SupportedMethod};
