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
