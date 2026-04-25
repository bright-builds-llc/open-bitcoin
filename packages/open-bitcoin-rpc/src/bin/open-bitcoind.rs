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
// - packages/bitcoin-knots/src/bitcoind.cpp
// - packages/bitcoin-knots/src/rpc/protocol.h
// - packages/bitcoin-knots/src/rpc/request.cpp
// - packages/bitcoin-knots/src/rpc/server.cpp
// - packages/bitcoin-knots/src/rpc/blockchain.cpp
// - packages/bitcoin-knots/src/rpc/mempool.cpp
// - packages/bitcoin-knots/src/rpc/net.cpp
// - packages/bitcoin-knots/src/rpc/rawtransaction.cpp
// - packages/bitcoin-knots/test/functional/interface_rpc.py

use open_bitcoin_rpc::{ManagedRpcContext, config::load_runtime_config, http};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let runtime = load_runtime_config()?;
    if !runtime.rpc_server.enabled {
        return Err("RPC server is disabled by configuration".into());
    }

    let bind_address = runtime.rpc_server.bind_address;
    let auth = runtime.rpc_server.auth.clone();
    let context = ManagedRpcContext::from_runtime_config(&runtime);
    let state = http::build_http_state(auth, context)?;
    let listener = tokio::net::TcpListener::bind(bind_address).await?;

    axum::serve(listener, http::router(state)).await?;
    Ok(())
}
