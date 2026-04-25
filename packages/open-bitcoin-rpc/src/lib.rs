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

//! Shell-layer RPC crate for Phase 8 typed contracts and runtime composition.

pub mod config;
pub mod context;
pub mod dispatch;
pub mod envelope;
pub mod error;
pub mod http;
pub mod method;

pub use config::{
    RpcAuthConfig, RpcClientConfig, RpcClientEndpoint, RpcServerConfig, RuntimeConfig,
    WalletRuntimeScope,
};
pub use context::ManagedRpcContext;
pub use envelope::{
    JsonRpcId, JsonRpcVersion, RpcErrorEnvelope, RpcRequestEnvelope, RpcSuccessEnvelope,
};
pub use error::{RpcErrorCode, RpcErrorDetail, RpcFailure, RpcFailureKind};
pub use method::{MethodOrigin, SupportedMethod};
