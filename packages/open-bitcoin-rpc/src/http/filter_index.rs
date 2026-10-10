// Parity breadcrumbs:
// - packages/bitcoin-knots/src/rpc/server.cpp
// - packages/bitcoin-knots/src/rpc/blockchain.cpp
// - packages/bitcoin-knots/src/index/base.cpp

use super::request::{ParsedRequest, ParsedRequestError, parse_request};
use crate::dispatch::filter_index::{self as dispatch, PreparedDispatch};
use crate::{ManagedRpcContext, RpcFailure, method::RequestParameters};
use open_bitcoin_node::{ChainstateStore, core::chainstate::CoinsView};
use serde::{
    Deserialize, Deserializer,
    de::{MapAccess, SeqAccess, Visitor},
};
use serde_json::{Map, Value};
use std::fmt;

/// Preserve only request parameter object pairs; all other maps retain Value semantics.
pub(super) struct ScopedRequest {
    pub(super) value: Value,
    pub(super) maybe_named: Option<Vec<(String, Value)>>,
    pub(super) batch: Vec<Self>,
}

impl ScopedRequest {
    pub(super) fn parse(self) -> Result<ParsedRequest, ParsedRequestError> {
        let mut parsed = parse_request(self.value)?;
        if matches!(parsed.method.as_str(), "getblockfilter" | "getindexinfo")
            && let Some(named) = self.maybe_named
        {
            parsed.params = RequestParameters::Named(named);
        }
        Ok(parsed)
    }
}

impl<'de> Deserialize<'de> for ScopedRequest {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct RequestVisitor;
        impl<'de> Visitor<'de> for RequestVisitor {
            type Value = ScopedRequest;
            fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str("JSON request")
            }
            fn visit_map<M: MapAccess<'de>>(self, mut map: M) -> Result<Self::Value, M::Error> {
                let mut object = Map::new();
                let mut maybe_named = None;
                while let Some(key) = map.next_key::<String>()? {
                    let value = if key == "params" {
                        let params = map.next_value::<ParameterValue>()?;
                        maybe_named = params.maybe_named;
                        params.value
                    } else {
                        map.next_value::<Value>()?
                    };
                    object.insert(key, value);
                }
                Ok(ScopedRequest {
                    value: Value::Object(object),
                    maybe_named,
                    batch: Vec::new(),
                })
            }
            fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
                let mut batch = Vec::new();
                while let Some(request) = seq.next_element::<ScopedRequest>()? {
                    batch.push(request);
                }
                Ok(ScopedRequest {
                    value: Value::Array(Vec::new()),
                    maybe_named: None,
                    batch,
                })
            }
            fn visit_bool<E: serde::de::Error>(self, value: bool) -> Result<Self::Value, E> {
                Ok(scalar(Value::Bool(value)))
            }
            fn visit_i64<E: serde::de::Error>(self, value: i64) -> Result<Self::Value, E> {
                Ok(scalar(value.into()))
            }
            fn visit_u64<E: serde::de::Error>(self, value: u64) -> Result<Self::Value, E> {
                Ok(scalar(value.into()))
            }
            fn visit_f64<E: serde::de::Error>(self, value: f64) -> Result<Self::Value, E> {
                Ok(scalar(serde_json::json!(value)))
            }
            fn visit_str<E: serde::de::Error>(self, value: &str) -> Result<Self::Value, E> {
                Ok(scalar(value.into()))
            }
            fn visit_unit<E: serde::de::Error>(self) -> Result<Self::Value, E> {
                Ok(scalar(Value::Null))
            }
        }
        deserializer.deserialize_any(RequestVisitor)
    }
}

fn scalar(value: Value) -> ScopedRequest {
    ScopedRequest {
        value,
        maybe_named: None,
        batch: Vec::new(),
    }
}

struct ParameterValue {
    value: Value,
    maybe_named: Option<Vec<(String, Value)>>,
}

impl<'de> Deserialize<'de> for ParameterValue {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct ParameterVisitor;
        impl<'de> Visitor<'de> for ParameterVisitor {
            type Value = ParameterValue;
            fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str("JSON parameters")
            }
            fn visit_map<M: MapAccess<'de>>(self, mut map: M) -> Result<Self::Value, M::Error> {
                let mut pairs = Vec::new();
                let mut object = Map::new();
                while let Some((key, value)) = map.next_entry::<String, Value>()? {
                    object.insert(key.clone(), value.clone());
                    pairs.push((key, value));
                }
                Ok(ParameterValue {
                    value: Value::Object(object),
                    maybe_named: Some(pairs),
                })
            }
            fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
                let mut values = Vec::new();
                while let Some(value) = seq.next_element::<Value>()? {
                    values.push(value);
                }
                Ok(parameter(Value::Array(values)))
            }
            fn visit_bool<E: serde::de::Error>(self, value: bool) -> Result<Self::Value, E> {
                Ok(parameter(value.into()))
            }
            fn visit_i64<E: serde::de::Error>(self, value: i64) -> Result<Self::Value, E> {
                Ok(parameter(value.into()))
            }
            fn visit_u64<E: serde::de::Error>(self, value: u64) -> Result<Self::Value, E> {
                Ok(parameter(value.into()))
            }
            fn visit_f64<E: serde::de::Error>(self, value: f64) -> Result<Self::Value, E> {
                Ok(parameter(serde_json::json!(value)))
            }
            fn visit_str<E: serde::de::Error>(self, value: &str) -> Result<Self::Value, E> {
                Ok(parameter(value.into()))
            }
            fn visit_unit<E: serde::de::Error>(self) -> Result<Self::Value, E> {
                Ok(parameter(Value::Null))
            }
        }
        deserializer.deserialize_any(ParameterVisitor)
    }
}

fn parameter(value: Value) -> ParameterValue {
    ParameterValue {
        value,
        maybe_named: None,
    }
}

pub(super) async fn finish<S: ChainstateStore, V: CoinsView>(
    context: &tokio::sync::Mutex<ManagedRpcContext<S, V>>,
    network: open_bitcoin_node::ManagedNetworkHandle<S, V>,
    prepared: PreparedDispatch,
) -> Result<Value, RpcFailure> {
    match prepared {
        PreparedDispatch::Complete(value) => Ok(value),
        PreparedDispatch::Pending { request, barrier } => {
            // Retain the original authority across await; completion never captures a newer tip.
            let completion = barrier.await.map_err(dispatch::query_failure)?;
            let mut guard = context.lock().await;
            super::validate_scope(&mut guard, crate::method::MethodScope::Node, None)?;
            let query = network
                .complete_basic_filter_read(request.block_hash, completion)
                .map_err(dispatch::query_failure)?;
            dispatch::completed(dispatch::project_query(request, query)?)
        }
    }
}

#[cfg(test)]
mod initial_tests {
    use crate::http::{build_http_state, handle_http_request};
    use crate::{ManagedRpcContext, RpcAuthConfig, RuntimeConfig};
    use axum::{
        body::to_bytes,
        http::{HeaderMap, HeaderValue, Method},
    };
    use base64::Engine as _;

    #[tokio::test]
    async fn phase159_filter_rpc_http_raw_duplicates() {
        // Arrange
        let state = build_http_state(
            RpcAuthConfig::UserPassword {
                username: "alice".into(),
                password: "secret".into(),
            },
            ManagedRpcContext::from_runtime_config(&RuntimeConfig::default()),
        )
        .expect("state");
        let mut headers = HeaderMap::new();
        headers.insert(
            "authorization",
            HeaderValue::from_str(&format!(
                "Basic {}",
                base64::engine::general_purpose::STANDARD.encode("alice:secret")
            ))
            .expect("header"),
        );
        // Act
        let response = handle_http_request(&state, "/", Method::POST, &headers, br#"{"jsonrpc":"2.0","id":7,"method":"getindexinfo","params":{"index_name":"a","index_name":"b"}}"#).await;
        let bytes = to_bytes(response.into_body(), usize::MAX)
            .await
            .expect("body");
        let body: serde_json::Value = serde_json::from_slice(&bytes).expect("json");
        // Assert
        assert_eq!(body["error"]["code"], -8);
        assert_eq!(
            body["error"]["message"],
            "Parameter index_name specified multiple times"
        );
    }
}
