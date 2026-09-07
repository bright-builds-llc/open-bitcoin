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

use std::{
    fs,
    io::Write,
    path::{Path, PathBuf},
    sync::Arc,
};

#[cfg(unix)]
use std::os::unix::fs::OpenOptionsExt;

use axum::{
    Router,
    body::{Body, Bytes},
    extract::{OriginalUri, State},
    http::{HeaderMap, HeaderValue, Method, StatusCode},
    response::Response,
    routing::any,
};
use base64::{Engine as _, engine::general_purpose::STANDARD};
use tokio::sync::Mutex;

use open_bitcoin_node::core::chainstate::CoinsView;
use open_bitcoin_node::{ChainstateStore, MemoryChainstateStore};

use crate::{
    JsonRpcId, ManagedRpcContext, RpcAuthConfig, RpcFailure,
    config::DEFAULT_COOKIE_AUTH_USER,
    dispatch::dispatch,
    method::{MethodScope, RequestParameters, normalize_method_call},
};

mod request;
use request::{
    ParsedRequest, error_body, legacy_error_body, legacy_status_for_failure, parse_request,
    rpc_error_object, status_for_single, success_body,
};

const WWW_AUTH_HEADER_DATA: &str = "Basic realm=\"jsonrpc\"";

pub struct RpcHttpState<
    S = MemoryChainstateStore,
    V: CoinsView = open_bitcoin_node::core::chainstate::MemoryCoinsView,
> {
    auth: ResolvedHttpAuth,
    context: Arc<Mutex<ManagedRpcContext<S, V>>>,
}

impl<S, V: CoinsView> Clone for RpcHttpState<S, V> {
    fn clone(&self) -> Self {
        Self {
            auth: self.auth.clone(),
            context: Arc::clone(&self.context),
        }
    }
}

impl<S, V: CoinsView> core::fmt::Debug for RpcHttpState<S, V> {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter
            .debug_struct("RpcHttpState")
            .field("auth", &self.auth)
            .finish()
    }
}

#[derive(Debug, Clone)]
struct ResolvedHttpAuth {
    username: String,
    password: String,
}

pub fn build_http_state<S, V>(
    auth: RpcAuthConfig,
    context: ManagedRpcContext<S, V>,
) -> std::io::Result<RpcHttpState<S, V>>
where
    S: ChainstateStore + Send + 'static,
    V: CoinsView + Send + 'static,
{
    build_http_state_with_shared_context(auth, Arc::new(Mutex::new(context)))
}

pub fn build_http_state_with_shared_context<S, V>(
    auth: RpcAuthConfig,
    context: Arc<Mutex<ManagedRpcContext<S, V>>>,
) -> std::io::Result<RpcHttpState<S, V>>
where
    S: ChainstateStore + Send + 'static,
    V: CoinsView + Send + 'static,
{
    Ok(RpcHttpState {
        auth: resolve_auth(auth)?,
        context,
    })
}

pub fn router<S, V>(state: RpcHttpState<S, V>) -> Router
where
    S: ChainstateStore + Send + 'static,
    V: CoinsView + Send + 'static,
{
    Router::new()
        .route("/", any(rpc_handler))
        .route("/wallet/{*wallet_name}", any(rpc_handler))
        .with_state(state)
}

pub async fn handle_http_request<S, V>(
    state: &RpcHttpState<S, V>,
    path: &str,
    method: Method,
    headers: &HeaderMap,
    body: &[u8],
) -> Response
where
    S: ChainstateStore + Send + 'static,
    V: CoinsView + Send + 'static,
{
    if method != Method::POST {
        return plain_response(
            StatusCode::METHOD_NOT_ALLOWED,
            "JSONRPC server handles only POST requests",
        );
    }
    if !authorized(headers, &state.auth) {
        return unauthorized_response();
    }

    let value = match serde_json::from_slice::<serde_json::Value>(body) {
        Ok(value) => value,
        Err(_) => {
            return json_response(
                StatusCode::INTERNAL_SERVER_ERROR,
                legacy_error_body(
                    Some(JsonRpcId::Null),
                    rpc_error_object("Parse error", -32700),
                ),
            );
        }
    };

    match value {
        serde_json::Value::Object(_) => handle_single_request(state, path, value).await,
        serde_json::Value::Array(batch) => handle_batch_request(state, path, batch).await,
        _ => json_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            legacy_error_body(
                Some(JsonRpcId::Null),
                rpc_error_object("Top-level object parse error", -32700),
            ),
        ),
    }
}

async fn rpc_handler<S, V>(
    State(state): State<RpcHttpState<S, V>>,
    OriginalUri(uri): OriginalUri,
    method: Method,
    headers: HeaderMap,
    body: Bytes,
) -> Response
where
    S: ChainstateStore + Send + 'static,
    V: CoinsView + Send + 'static,
{
    handle_http_request(&state, uri.path(), method, &headers, &body).await
}

async fn handle_single_request<S, V>(
    state: &RpcHttpState<S, V>,
    path: &str,
    value: serde_json::Value,
) -> Response
where
    S: ChainstateStore + Send + 'static,
    V: CoinsView + Send + 'static,
{
    let parsed = match parse_request(value) {
        Ok(parsed) => parsed,
        Err(error) => {
            let status = legacy_status_for_failure(error.failure.kind);
            return json_response(
                status,
                error_body(error.version, error.maybe_id, error.failure),
            );
        }
    };

    match execute_request(state, path, parsed).await {
        Some((status, body)) => json_response(status, body),
        None => empty_response(StatusCode::NO_CONTENT),
    }
}

async fn handle_batch_request<S, V>(
    state: &RpcHttpState<S, V>,
    path: &str,
    batch: Vec<serde_json::Value>,
) -> Response
where
    S: ChainstateStore + Send + 'static,
    V: CoinsView + Send + 'static,
{
    if batch.is_empty() {
        return json_response(StatusCode::OK, serde_json::Value::Array(Vec::new()));
    }

    let mut responses = Vec::new();

    for value in batch {
        match parse_request(value) {
            Ok(parsed) => {
                if let Some((_status, body)) = execute_request(state, path, parsed).await {
                    responses.push(body);
                }
            }
            Err(error) => responses.push(error_body(error.version, error.maybe_id, error.failure)),
        }
    }

    if responses.is_empty() {
        return empty_response(StatusCode::NO_CONTENT);
    }

    json_response(StatusCode::OK, serde_json::Value::Array(responses))
}

async fn execute_request<S, V>(
    state: &RpcHttpState<S, V>,
    path: &str,
    parsed: ParsedRequest,
) -> Option<(StatusCode, serde_json::Value)>
where
    S: ChainstateStore + Send + 'static,
    V: CoinsView + Send + 'static,
{
    let call = match normalize_method_call(
        &parsed.method,
        RequestParameters::from_json(parsed.params.clone()),
    ) {
        Ok(call) => call,
        Err(failure) => {
            if parsed.is_notification {
                return None;
            }
            return Some((
                status_for_single(parsed.version, failure.kind),
                error_body(parsed.version, parsed.maybe_id, failure),
            ));
        }
    };

    let maybe_wallet_name = match parse_request_wallet_name(path) {
        Ok(maybe_wallet_name) => maybe_wallet_name,
        Err(failure) => {
            if parsed.is_notification {
                return None;
            }
            return Some((
                status_for_single(parsed.version, failure.kind),
                error_body(parsed.version, parsed.maybe_id, failure),
            ));
        }
    };

    let mut context = state.context.lock().await;
    context.set_request_wallet_name(maybe_wallet_name.clone());
    if let Err(failure) = validate_scope(&mut context, call.scope(), maybe_wallet_name.as_deref()) {
        context.clear_request_wallet_name();
        if parsed.is_notification {
            return None;
        }
        return Some((
            status_for_single(parsed.version, failure.kind),
            error_body(parsed.version, parsed.maybe_id, failure),
        ));
    }
    let result = dispatch(&mut context, call);
    context.clear_request_wallet_name();
    match result {
        Ok(result) => {
            if parsed.is_notification {
                None
            } else {
                Some((
                    StatusCode::OK,
                    success_body(parsed.version, parsed.maybe_id, result),
                ))
            }
        }
        Err(failure) => {
            if parsed.is_notification {
                None
            } else {
                Some((
                    status_for_single(parsed.version, failure.kind),
                    error_body(parsed.version, parsed.maybe_id, failure),
                ))
            }
        }
    }
}

fn validate_scope<S, V>(
    context: &mut ManagedRpcContext<S, V>,
    scope: MethodScope,
    maybe_wallet_name: Option<&str>,
) -> Result<(), RpcFailure>
where
    S: open_bitcoin_node::ChainstateStore,
    V: open_bitcoin_node::core::chainstate::CoinsView,
{
    match scope {
        MethodScope::Node => {
            if maybe_wallet_name.is_some() {
                return Err(RpcFailure::invalid_request(
                    "Node-scoped RPC methods must be requested through the root URI path.",
                ));
            }
            Ok(())
        }
        MethodScope::Wallet => context.require_wallet_selection(),
    }
}

fn parse_request_wallet_name(path: &str) -> Result<Option<String>, RpcFailure> {
    if path == "/" {
        return Ok(None);
    }

    let Some(wallet_name) = path.strip_prefix("/wallet/") else {
        return Err(RpcFailure::invalid_request(
            "RPC requests must target / or /wallet/<walletname>.",
        ));
    };
    if wallet_name.is_empty() {
        return Err(RpcFailure::invalid_request(
            "Wallet RPC requests must include a wallet name in the URI path.",
        ));
    }

    percent_decode_path_segment(wallet_name).map(Some)
}

fn percent_decode_path_segment(value: &str) -> Result<String, RpcFailure> {
    let mut decoded = String::with_capacity(value.len());
    let bytes = value.as_bytes();
    let mut index = 0;

    while index < bytes.len() {
        if bytes[index] != b'%' {
            decoded.push(char::from(bytes[index]));
            index += 1;
            continue;
        }

        if index + 2 >= bytes.len() {
            return Err(RpcFailure::invalid_request(
                "Wallet URI path contains an invalid percent-encoding sequence.",
            ));
        }

        let high = decode_hex_nibble(bytes[index + 1])?;
        let low = decode_hex_nibble(bytes[index + 2])?;
        decoded.push(char::from((high << 4) | low));
        index += 3;
    }

    Ok(decoded)
}

fn decode_hex_nibble(value: u8) -> Result<u8, RpcFailure> {
    match value {
        b'0'..=b'9' => Ok(value - b'0'),
        b'a'..=b'f' => Ok(value - b'a' + 10),
        b'A'..=b'F' => Ok(value - b'A' + 10),
        _ => Err(RpcFailure::invalid_request(
            "Wallet URI path contains an invalid percent-encoding sequence.",
        )),
    }
}

fn authorized(headers: &HeaderMap, auth: &ResolvedHttpAuth) -> bool {
    let Some(header) = headers.get("authorization") else {
        return false;
    };
    let Ok(header) = header.to_str() else {
        return false;
    };
    let Some(encoded) = header.strip_prefix("Basic ") else {
        return false;
    };
    let Ok(decoded) = STANDARD.decode(encoded.trim()) else {
        return false;
    };
    let Ok(decoded) = String::from_utf8(decoded) else {
        return false;
    };
    decoded == format!("{}:{}", auth.username, auth.password)
}

fn resolve_auth(auth: RpcAuthConfig) -> std::io::Result<ResolvedHttpAuth> {
    match auth {
        RpcAuthConfig::UserPassword { username, password } => {
            Ok(ResolvedHttpAuth { username, password })
        }
        RpcAuthConfig::Cookie { maybe_cookie_file } => {
            let cookie_file = maybe_cookie_file.unwrap_or_else(|| PathBuf::from(".cookie"));
            let password = read_or_create_cookie_password(&cookie_file)?;
            Ok(ResolvedHttpAuth {
                username: DEFAULT_COOKIE_AUTH_USER.to_string(),
                password,
            })
        }
    }
}

fn read_or_create_cookie_password(path: &Path) -> std::io::Result<String> {
    if let Ok(contents) = fs::read_to_string(path)
        && let Some((_username, password)) = contents.trim().split_once(':')
    {
        return Ok(password.to_string());
    }

    let password = random_hex_secret()?;
    if let Some(parent) = path.parent()
        && !parent.as_os_str().is_empty()
    {
        fs::create_dir_all(parent)?;
    }
    let mut open_options = fs::OpenOptions::new();
    open_options.write(true).create(true).truncate(true);
    #[cfg(unix)]
    open_options.mode(0o600);
    let mut file = open_options.open(path)?;
    file.write_all(format!("{DEFAULT_COOKIE_AUTH_USER}:{password}\n").as_bytes())?;
    Ok(password)
}

fn random_hex_secret() -> std::io::Result<String> {
    let mut bytes = [0_u8; 32];
    getrandom::fill(&mut bytes).map_err(|error| std::io::Error::other(error.to_string()))?;
    Ok(hex_encode(&bytes))
}

fn hex_encode(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut output = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        output.push(HEX[(byte >> 4) as usize] as char);
        output.push(HEX[(byte & 0x0f) as usize] as char);
    }
    output
}

fn json_response(status: StatusCode, body: serde_json::Value) -> Response {
    let mut response = Response::new(Body::from(body.to_string()));
    *response.status_mut() = status;
    response
        .headers_mut()
        .insert("content-type", HeaderValue::from_static("application/json"));
    response
}

fn plain_response(status: StatusCode, body: &'static str) -> Response {
    let mut response = Response::new(Body::from(body));
    *response.status_mut() = status;
    response
}

fn unauthorized_response() -> Response {
    let mut response = empty_response(StatusCode::UNAUTHORIZED);
    response.headers_mut().insert(
        "www-authenticate",
        HeaderValue::from_static(WWW_AUTH_HEADER_DATA),
    );
    response
}

fn empty_response(status: StatusCode) -> Response {
    let mut response = Response::new(Body::empty());
    *response.status_mut() = status;
    response
}

#[cfg(test)]
mod tests;
