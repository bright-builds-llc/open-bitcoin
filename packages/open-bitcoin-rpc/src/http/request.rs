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

use axum::http::StatusCode;

use crate::{JsonRpcId, RpcFailure, RpcFailureKind};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum RequestVersion {
    Legacy,
    V2,
}

#[derive(Debug, Clone)]
pub(super) struct ParsedRequest {
    pub(super) version: RequestVersion,
    pub(super) maybe_id: Option<JsonRpcId>,
    pub(super) method: String,
    pub(super) params: serde_json::Value,
    pub(super) is_notification: bool,
}

#[derive(Debug, Clone)]
pub(super) struct ParsedRequestError {
    pub(super) version: RequestVersion,
    pub(super) maybe_id: Option<JsonRpcId>,
    pub(super) failure: RpcFailure,
}

pub(super) fn parse_request(value: serde_json::Value) -> Result<ParsedRequest, ParsedRequestError> {
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

pub(super) fn status_for_single(version: RequestVersion, kind: RpcFailureKind) -> StatusCode {
    match version {
        RequestVersion::V2 => StatusCode::OK,
        RequestVersion::Legacy => legacy_status_for_failure(kind),
    }
}

pub(super) fn legacy_status_for_failure(kind: RpcFailureKind) -> StatusCode {
    match kind {
        RpcFailureKind::MethodNotFound => StatusCode::NOT_FOUND,
        _ => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

pub(super) fn success_body(
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

pub(super) fn error_body(
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

pub(super) fn legacy_error_body(
    maybe_id: Option<JsonRpcId>,
    error: serde_json::Value,
) -> serde_json::Value {
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

pub(super) fn rpc_error_object(message: &str, code: i32) -> serde_json::Value {
    serde_json::json!({
        "code": code,
        "message": message,
    })
}
