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

use std::sync::Arc;

use open_bitcoin_network::{InboundPreflightDiagnostic, InboundPreflightReason};
use open_bitcoin_node::core::chainstate::CoinsView;
use open_bitcoin_node::{
    ChainstateStore, ManagedNetworkHandle, PeerIdentityAuthority, sync::AnnouncementOutboxRegistry,
};
use open_bitcoin_rpc::{
    ManagedRpcContext,
    config::RuntimeConfig,
    inbound_listener::{
        InboundListenerState, activate_inbound_listener, start_inbound_accept_loop,
        start_inbound_accept_loop_with_announcements,
    },
};

use super::InboundDaemonListener;

pub(super) fn report_inbound_listener_startup(listener: &InboundDaemonListener) {
    eprintln!("{}", inbound_listener_startup_message(listener));
}

#[cfg(test)]
pub(super) async fn start_inbound_listener_for_runtime(
    runtime: &RuntimeConfig,
) -> InboundDaemonListener {
    let context = Arc::new(tokio::sync::Mutex::new(
        ManagedRpcContext::from_runtime_config(runtime),
    ));
    start_inbound_listener_for_runtime_with_context(runtime, context).await
}

#[cfg(test)]
pub(super) async fn start_inbound_listener_for_runtime_with_context<S, V>(
    runtime: &RuntimeConfig,
    context: Arc<tokio::sync::Mutex<ManagedRpcContext<S, V>>>,
) -> InboundDaemonListener
where
    S: ChainstateStore + Send + 'static,
    V: CoinsView + Send + 'static,
{
    start_inbound_listener_for_runtime_with_context_inner(runtime, context, None).await
}

pub(super) async fn start_inbound_listener_for_runtime_with_context_and_announcements<S, V>(
    runtime: &RuntimeConfig,
    context: Arc<tokio::sync::Mutex<ManagedRpcContext<S, V>>>,
    peer_identity_authority: PeerIdentityAuthority,
    outboxes: AnnouncementOutboxRegistry,
    network: ManagedNetworkHandle<S, V>,
) -> InboundDaemonListener
where
    S: ChainstateStore + Send + 'static,
    V: CoinsView + Send + 'static,
{
    start_inbound_listener_for_runtime_with_context_inner(
        runtime,
        context,
        Some((peer_identity_authority, outboxes, network)),
    )
    .await
}

async fn start_inbound_listener_for_runtime_with_context_inner<S, V>(
    runtime: &RuntimeConfig,
    context: Arc<tokio::sync::Mutex<ManagedRpcContext<S, V>>>,
    maybe_announcement_transport: Option<(
        PeerIdentityAuthority,
        AnnouncementOutboxRegistry,
        ManagedNetworkHandle<S, V>,
    )>,
) -> InboundDaemonListener
where
    S: ChainstateStore + Send + 'static,
    V: CoinsView + Send + 'static,
{
    let activation = activate_inbound_listener(&runtime.inbound).await;
    let mut state = activation.state();
    let mut preflight_reason = activation.preflight_reason();
    let bound_endpoints = activation
        .bound_endpoints()
        .iter()
        .map(|endpoint| endpoint.bound_endpoint.clone())
        .collect::<Vec<_>>();
    let mut diagnostics = activation.diagnostics().to_vec();
    let authority_available = {
        let mut context = context.lock().await;
        context
            .set_inbound_listener_evidence(activation.evidence().clone())
            .is_ok()
    };
    if !authority_available {
        state = InboundListenerState::Blocked;
        preflight_reason = InboundPreflightReason::BindUnavailable;
        diagnostics.push(InboundPreflightDiagnostic {
            reason: InboundPreflightReason::BindUnavailable,
            maybe_endpoint: None,
            field: "authoritative_network",
            message: "authoritative network state is unavailable".to_string(),
            next_action: "restart open-bitcoind and inspect runtime health".to_string(),
        });
    }
    let maybe_worker = if state == InboundListenerState::Listening && authority_available {
        match maybe_announcement_transport {
            Some((peer_identity_authority, outboxes, network)) => {
                start_inbound_accept_loop_with_announcements(
                    activation,
                    context,
                    peer_identity_authority,
                    outboxes,
                    network,
                )
            }
            None => start_inbound_accept_loop(activation, context),
        }
    } else {
        None
    };

    InboundDaemonListener {
        state,
        preflight_reason,
        bound_endpoints,
        diagnostics,
        maybe_worker,
    }
}

pub(super) fn inbound_listener_startup_message(listener: &InboundDaemonListener) -> String {
    let bound_endpoint = listener
        .bound_endpoints
        .first()
        .cloned()
        .unwrap_or_else(|| "unavailable".to_string());
    let next_action = listener
        .diagnostics
        .first()
        .map(|diagnostic| diagnostic.next_action.as_str())
        .unwrap_or("no listener action needed");
    format!(
        "open-bitcoind inbound listener startup: inbound_listener_state={} inbound_preflight_reason={} bound_endpoint={} admission_reject_reason=unavailable; opt-in inbound listener/admission {}; next_action=\"{}\"; deferred network participation remains out of scope.",
        listener.state.as_str(),
        listener.preflight_reason.as_str(),
        bound_endpoint,
        inbound_listener_state_description(listener.state),
        next_action
    )
}

fn inbound_listener_state_description(state: InboundListenerState) -> &'static str {
    match state {
        InboundListenerState::Disabled => "is disabled by configuration",
        InboundListenerState::Blocked => "is blocked before socket serving",
        InboundListenerState::Listening => "is active on configured endpoints",
    }
}
