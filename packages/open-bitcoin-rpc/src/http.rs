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
    extract::State,
    http::{HeaderMap, HeaderValue, Method, StatusCode},
    response::Response,
    routing::any,
};
use base64::{Engine as _, engine::general_purpose::STANDARD};
use tokio::sync::Mutex;

use crate::{
    JsonRpcId, ManagedRpcContext, RpcAuthConfig, RpcFailure, RpcFailureKind,
    config::DEFAULT_COOKIE_AUTH_USER,
    dispatch::dispatch,
    method::{RequestParameters, normalize_method_call},
};

const WWW_AUTH_HEADER_DATA: &str = "Basic realm=\"jsonrpc\"";

#[derive(Debug, Clone)]
pub struct RpcHttpState {
    auth: ResolvedHttpAuth,
    context: Arc<Mutex<ManagedRpcContext>>,
}

#[derive(Debug, Clone)]
struct ResolvedHttpAuth {
    username: String,
    password: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RequestVersion {
    Legacy,
    V2,
}

#[derive(Debug, Clone)]
struct ParsedRequest {
    version: RequestVersion,
    maybe_id: Option<JsonRpcId>,
    method: String,
    params: serde_json::Value,
    is_notification: bool,
}

#[derive(Debug, Clone)]
struct ParsedRequestError {
    version: RequestVersion,
    maybe_id: Option<JsonRpcId>,
    failure: RpcFailure,
}

pub fn build_http_state(
    auth: RpcAuthConfig,
    context: ManagedRpcContext,
) -> std::io::Result<RpcHttpState> {
    Ok(RpcHttpState {
        auth: resolve_auth(auth)?,
        context: Arc::new(Mutex::new(context)),
    })
}

pub fn router(state: RpcHttpState) -> Router {
    Router::new().route("/", any(rpc_handler)).with_state(state)
}

pub async fn handle_http_request(
    state: &RpcHttpState,
    method: Method,
    headers: &HeaderMap,
    body: &[u8],
) -> Response {
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
        serde_json::Value::Object(_) => handle_single_request(state, value).await,
        serde_json::Value::Array(batch) => handle_batch_request(state, batch).await,
        _ => json_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            legacy_error_body(
                Some(JsonRpcId::Null),
                rpc_error_object("Top-level object parse error", -32700),
            ),
        ),
    }
}

async fn rpc_handler(
    State(state): State<RpcHttpState>,
    method: Method,
    headers: HeaderMap,
    body: Bytes,
) -> Response {
    handle_http_request(&state, method, &headers, &body).await
}

async fn handle_single_request(state: &RpcHttpState, value: serde_json::Value) -> Response {
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

    match execute_request(state, parsed).await {
        Some((status, body)) => json_response(status, body),
        None => empty_response(StatusCode::NO_CONTENT),
    }
}

async fn handle_batch_request(state: &RpcHttpState, batch: Vec<serde_json::Value>) -> Response {
    if batch.is_empty() {
        return json_response(StatusCode::OK, serde_json::Value::Array(Vec::new()));
    }

    let mut responses = Vec::new();

    for value in batch {
        match parse_request(value) {
            Ok(parsed) => {
                if let Some((_status, body)) = execute_request(state, parsed).await {
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

async fn execute_request(
    state: &RpcHttpState,
    parsed: ParsedRequest,
) -> Option<(StatusCode, serde_json::Value)> {
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

    let mut context = state.context.lock().await;
    let result = dispatch(&mut context, call);
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

fn parse_request(value: serde_json::Value) -> Result<ParsedRequest, ParsedRequestError> {
    let serde_json::Value::Object(object) = value else {
        return Err(ParsedRequestError {
            version: RequestVersion::Legacy,
            maybe_id: Some(JsonRpcId::Null),
            failure: RpcFailure::invalid_request("Invalid Request object"),
        });
    };

    let version = match object.get("jsonrpc") {
        Some(serde_json::Value::String(marker)) if marker == "2.0" => RequestVersion::V2,
        _ => RequestVersion::Legacy,
    };
    let maybe_id = object.get("id").and_then(parse_id);
    let is_notification = version == RequestVersion::V2 && !object.contains_key("id");

    let Some(method) = object.get("method") else {
        return Err(ParsedRequestError {
            version,
            maybe_id,
            failure: RpcFailure::invalid_request("Missing method"),
        });
    };
    let serde_json::Value::String(method) = method else {
        return Err(ParsedRequestError {
            version,
            maybe_id,
            failure: RpcFailure::invalid_request("Method must be a string"),
        });
    };

    let params = match object.get("params") {
        None | Some(serde_json::Value::Null) => serde_json::Value::Null,
        Some(serde_json::Value::Array(values)) => serde_json::Value::Array(values.clone()),
        Some(serde_json::Value::Object(values)) => serde_json::Value::Object(values.clone()),
        Some(_) => {
            return Err(ParsedRequestError {
                version,
                maybe_id,
                failure: RpcFailure::invalid_request("Params must be an array or object"),
            });
        }
    };

    Ok(ParsedRequest {
        version,
        maybe_id,
        method: method.clone(),
        params,
        is_notification,
    })
}

fn parse_id(value: &serde_json::Value) -> Option<JsonRpcId> {
    match value {
        serde_json::Value::Null => Some(JsonRpcId::Null),
        serde_json::Value::Number(number) => number.as_i64().map(JsonRpcId::Number),
        serde_json::Value::String(string) => Some(JsonRpcId::String(string.clone())),
        _ => None,
    }
}

fn status_for_single(version: RequestVersion, kind: RpcFailureKind) -> StatusCode {
    match version {
        RequestVersion::V2 => StatusCode::OK,
        RequestVersion::Legacy => legacy_status_for_failure(kind),
    }
}

fn legacy_status_for_failure(kind: RpcFailureKind) -> StatusCode {
    match kind {
        RpcFailureKind::MethodNotFound => StatusCode::NOT_FOUND,
        _ => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

fn success_body(
    version: RequestVersion,
    maybe_id: Option<JsonRpcId>,
    result: serde_json::Value,
) -> serde_json::Value {
    match version {
        RequestVersion::V2 => {
            let mut object = serde_json::Map::new();
            object.insert("jsonrpc".to_string(), serde_json::json!("2.0"));
            object.insert("result".to_string(), result);
            if let Some(id) = maybe_id {
                object.insert("id".to_string(), json_id_value(id));
            }
            serde_json::Value::Object(object)
        }
        RequestVersion::Legacy => legacy_success_body(maybe_id, result),
    }
}

fn error_body(
    version: RequestVersion,
    maybe_id: Option<JsonRpcId>,
    failure: RpcFailure,
) -> serde_json::Value {
    let detail = failure.maybe_detail.unwrap_or_else(|| {
        crate::RpcErrorDetail::new(crate::RpcErrorCode::InternalError, "Internal error")
    });
    let error = rpc_error_object(&detail.message, detail.code.as_i32());
    match version {
        RequestVersion::V2 => {
            let mut object = serde_json::Map::new();
            object.insert("jsonrpc".to_string(), serde_json::json!("2.0"));
            object.insert("error".to_string(), error);
            object.insert(
                "id".to_string(),
                maybe_id.map_or(serde_json::Value::Null, json_id_value),
            );
            serde_json::Value::Object(object)
        }
        RequestVersion::Legacy => legacy_error_body(maybe_id, error),
    }
}

fn legacy_success_body(
    maybe_id: Option<JsonRpcId>,
    result: serde_json::Value,
) -> serde_json::Value {
    let mut object = serde_json::Map::new();
    object.insert("result".to_string(), result);
    object.insert("error".to_string(), serde_json::Value::Null);
    if let Some(id) = maybe_id {
        object.insert("id".to_string(), json_id_value(id));
    }
    serde_json::Value::Object(object)
}

fn legacy_error_body(maybe_id: Option<JsonRpcId>, error: serde_json::Value) -> serde_json::Value {
    let mut object = serde_json::Map::new();
    object.insert("result".to_string(), serde_json::Value::Null);
    object.insert("error".to_string(), error);
    if let Some(id) = maybe_id {
        object.insert("id".to_string(), json_id_value(id));
    }
    serde_json::Value::Object(object)
}

fn json_id_value(id: JsonRpcId) -> serde_json::Value {
    match id {
        JsonRpcId::Null => serde_json::Value::Null,
        JsonRpcId::Number(number) => serde_json::json!(number),
        JsonRpcId::String(string) => serde_json::json!(string),
    }
}

fn rpc_error_object(message: &str, code: i32) -> serde_json::Value {
    serde_json::json!({
        "code": code,
        "message": message,
    })
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
