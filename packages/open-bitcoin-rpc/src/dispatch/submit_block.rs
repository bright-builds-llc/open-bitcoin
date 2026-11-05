// Parity breadcrumbs:
// - packages/bitcoin-knots/src/rpc/mining.cpp
// - packages/bitcoin-knots/src/rpc/server.cpp
// - packages/bitcoin-knots/src/rpc/util.cpp
// - packages/bitcoin-knots/src/core_read.cpp
// - packages/bitcoin-knots/src/util/strencodings.cpp

use crate::ManagedRpcContext;
use crate::error::{RpcErrorCode, RpcErrorDetail, RpcFailure, RpcFailureKind};
use crate::method::{RequestParameters, SubmitBlockRequest};
use serde_json::Value;
use std::collections::HashMap;

const HELP: &str = concat!(
    "submitblock \"hexdata\" ( \"dummy\" )\n",
    "\n",
    "Attempts to submit new block to network.\n",
    "See https://en.bitcoin.it/wiki/BIP_0022 for full specification.\n",
    "\n",
    "Arguments:\n",
    "1. hexdata    (string, required) the hex-encoded block data to submit\n",
    "2. dummy      (string, optional, default=ignored) dummy value, for compatibility with BIP22. This value is ignored.\n",
    "\n",
    "Result (If the block was accepted):\n",
    "null    (json null)\n",
    "\n",
    "Result (Otherwise):\n",
    "\"str\"    (string) According to BIP22\n",
    "\n",
    "Examples:\n",
    "> bitcoin-cli submitblock \"mydata\"\n",
    "> curl --user myusername --data-binary '{\"jsonrpc\": \"2.0\", \"id\": \"curltest\", \"method\": \"submitblock\", \"params\": [\"mydata\"]}' -H 'content-type: application/json' http://127.0.0.1:8332/\n",
);

impl SubmitBlockRequest {
    /// Preserve pinned count/type/named semantics without consulting a backend.
    pub fn normalize_pinned(params: RequestParameters) -> Result<Self, RpcFailure> {
        let values = match params {
            RequestParameters::None => Vec::new(),
            RequestParameters::Positional(values) => values,
            RequestParameters::Named(named) => named_arguments(named)?,
            RequestParameters::Mixed {
                positional,
                mut named,
            } => {
                named.insert(0, ("args".to_owned(), Value::Array(positional)));
                named_arguments(named)?
            }
        };
        if !(1..=2).contains(&values.len()) {
            return Err(failure(RpcErrorCode::MiscError, HELP));
        }
        let mismatches = ["hexdata", "dummy"].iter().enumerate().filter_map(|(index, name)| {
            let value = values.get(index).unwrap_or(&Value::Null);
            if value.is_string() || (index == 1 && value.is_null()) { return None; }
            Some(format!("    \"Position {} ({name})\": \"JSON value of type {} is not of expected type string\"", index + 1, json_type(value)))
        }).collect::<Vec<_>>();
        if !mismatches.is_empty() {
            return Err(failure(
                RpcErrorCode::TypeError,
                format!("Wrong type passed:\n{{\n{}\n}}", mismatches.join(",\n")),
            ));
        }
        let Some(hexdata) = values[0].as_str() else {
            return Err(RpcFailure::internal_error(
                "normalized submitblock input unavailable",
            ));
        };
        Ok(Self {
            hexdata: hexdata.to_owned(),
        })
    }
}

fn failure(code: RpcErrorCode, message: impl Into<String>) -> RpcFailure {
    RpcFailure::new(
        RpcFailureKind::InvalidParams,
        Some(RpcErrorDetail::new(code, message)),
    )
}

fn json_type(value: &Value) -> &'static str {
    match value {
        Value::Null => "null",
        Value::Bool(_) => "bool",
        Value::Number(_) => "number",
        Value::String(_) => "string",
        Value::Array(_) => "array",
        Value::Object(_) => "object",
    }
}

fn named_arguments(named: Vec<(String, Value)>) -> Result<Vec<Value>, RpcFailure> {
    let mut arguments = HashMap::new();
    for (name, value) in named {
        if arguments.insert(name.clone(), value).is_some() {
            return Err(RpcFailure::invalid_parameter(format!(
                "Parameter {name} specified multiple times"
            )));
        }
    }
    let maybe_positional = match arguments.remove("args") {
        Some(Value::Array(values)) => Some(values),
        Some(value) => {
            arguments.insert("args".to_owned(), value);
            None
        }
        None => None,
    };
    let mut values = Vec::new();
    let mut maybe_first_named = None;
    for (index, name) in ["hexdata", "dummy"].iter().enumerate() {
        if let Some(value) = arguments.remove(*name) {
            maybe_first_named.get_or_insert((index, *name));
            values.resize(index, Value::Null);
            values.push(value);
        }
    }
    if let Some(positional) = maybe_positional {
        if let Some((index, name)) = maybe_first_named
            && positional.len() > index
        {
            return Err(RpcFailure::invalid_parameter(format!(
                "Parameter {name} specified twice both as positional and named argument"
            )));
        }
        let mut combined = positional;
        combined.extend(values.into_iter().skip(combined.len()));
        values = combined;
    }
    if let Some(name) = arguments.keys().next() {
        return Err(RpcFailure::invalid_parameter(format!(
            "Unknown named parameter {name}"
        )));
    }
    Ok(values)
}

fn decode(hexdata: &str) -> Result<open_bitcoin_codec::block::wire::WireBlock, RpcFailure> {
    let malformed = || RpcFailure::deserialization_error("Block decode failed");
    if hexdata.is_empty()
        || !hexdata.len().is_multiple_of(2)
        || !hexdata.bytes().all(|byte| byte.is_ascii_hexdigit())
    {
        return Err(malformed());
    }
    let bytes = super::decode::decode_hex(hexdata).map_err(|_| malformed())?;
    let (block, _) = open_bitcoin_codec::block::wire::parse_submission_block_prefix(&bytes)
        .map_err(|_| malformed())?;
    Ok(block)
}

pub(super) fn submit<S, V>(
    context: &ManagedRpcContext<S, V>,
    request: SubmitBlockRequest,
) -> Result<Value, RpcFailure>
where
    S: open_bitcoin_node::ChainstateStore,
    V: open_bitcoin_node::core::chainstate::CoinsView,
{
    context.submit_canonical_block(decode(&request.hexdata)?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::method::{MethodCall, normalize_method_call};
    use serde_json::json;

    fn code(result: Result<MethodCall, RpcFailure>) -> i32 {
        result
            .expect_err("pinned refusal")
            .maybe_detail
            .expect("detail")
            .code
            .as_i32()
    }

    #[test]
    fn phase167_submitblock_positional_named_mixed_dummy_are_identical() {
        for params in [
            RequestParameters::Positional(vec![json!("00")]),
            RequestParameters::Positional(vec![json!("00"), Value::Null]),
            RequestParameters::Positional(vec![json!("00"), json!("ignored")]),
            RequestParameters::Named(vec![("hexdata".to_owned(), json!("00"))]),
            RequestParameters::Named(vec![("args".to_owned(), json!(["00"]))]),
            RequestParameters::Mixed {
                positional: vec![json!("00")],
                named: vec![("dummy".to_owned(), Value::Null)],
            },
        ] {
            // Arrange / Act
            let call = normalize_method_call("submitblock", params).expect("pinned parameters");
            // Assert
            assert_eq!(
                call,
                MethodCall::SubmitBlock(SubmitBlockRequest {
                    hexdata: "00".to_owned()
                })
            );
        }
    }

    #[test]
    fn phase167_submitblock_arity_is_misc_minus_one_before_type_checks() {
        for params in [
            RequestParameters::None,
            RequestParameters::Positional(vec![]),
            RequestParameters::Positional(vec![Value::Null; 3]),
        ] {
            // Arrange / Act / Assert
            assert_eq!(code(normalize_method_call("submitblock", params)), -1);
        }
    }

    #[test]
    fn phase167_submitblock_required_and_optional_type_errors_are_minus_three() {
        for params in [
            RequestParameters::Positional(vec![Value::Null]),
            RequestParameters::Positional(vec![json!(7)]),
            RequestParameters::Positional(vec![json!("00"), json!(false)]),
            RequestParameters::Named(vec![("dummy".to_owned(), json!("ignored"))]),
        ] {
            // Arrange / Act / Assert
            assert_eq!(code(normalize_method_call("submitblock", params)), -3);
        }
        let error = normalize_method_call(
            "submitblock",
            RequestParameters::Positional(vec![json!(7), json!(false)]),
        )
        .expect_err("both types");
        let message = error.maybe_detail.expect("detail").message;
        assert!(message.contains("Position 1 (hexdata)"));
        assert!(message.contains("Position 2 (dummy)"));
    }

    #[test]
    fn phase167_submitblock_unknown_duplicate_and_colliding_named_are_minus_eight() {
        for params in [
            RequestParameters::Named(vec![("unknown".to_owned(), json!("00"))]),
            RequestParameters::Named(vec![("args".to_owned(), json!("00"))]),
            RequestParameters::Named(vec![
                ("hexdata".to_owned(), json!("00")),
                ("hexdata".to_owned(), json!("11")),
            ]),
            RequestParameters::Mixed {
                positional: vec![json!("00")],
                named: vec![("hexdata".to_owned(), json!("00"))],
            },
            RequestParameters::Mixed {
                positional: vec![json!("00")],
                named: vec![("args".to_owned(), json!([]))],
            },
        ] {
            // Arrange / Act / Assert
            assert_eq!(code(normalize_method_call("submitblock", params)), -8);
        }
    }

    #[test]
    fn phase167_submitblock_strict_ascii_hex_and_complete_wire_decode() {
        for input in [
            "", "0", "gg", " 00", "00 ", "00\n", "0x00", "００", "00", "0000",
        ] {
            // Arrange / Act
            let error = decode(input).expect_err("strict malformed block");
            // Assert
            let detail = error.maybe_detail.expect("detail");
            assert_eq!(detail.code, RpcErrorCode::DeserializationError);
            assert_eq!(detail.message, "Block decode failed");
        }
    }

    #[test]
    fn phase167_submitblock_complete_prefix_accepts_only_post_body_suffix() {
        // Arrange
        let block = open_bitcoin_node::core::consensus::CanonicalChain::Regtest
            .spec()
            .genesis_block();
        let bytes = open_bitcoin_codec::encode_block(&block).expect("genesis bytes");
        let mut suffixed = bytes.clone();
        suffixed.extend_from_slice(&[0xff, 0x00]);
        // Act
        let decoded =
            decode(&super::super::decode::encode_hex(&suffixed)).expect("complete prefix");
        // Assert
        assert_eq!(decoded.try_into_block().expect("amounts"), block);
        assert!(decode(&super::super::decode::encode_hex(&bytes[..bytes.len() - 1])).is_err());
        assert!(open_bitcoin_codec::parse_block(&suffixed).is_err());
    }

    #[test]
    fn phase167_submitblock_fixture_dispatch_has_no_canonical_submission_bridge() {
        // Arrange
        let mut context = crate::ManagedRpcContext::from_fixture_runtime_config(
            &crate::config::RuntimeConfig::default(),
        );
        let block = open_bitcoin_node::core::consensus::CanonicalChain::Regtest
            .spec()
            .genesis_block();
        let hex = super::super::decode::encode_hex(
            &open_bitcoin_codec::encode_block(&block).expect("bytes"),
        );
        let call = normalize_method_call(
            "submitblock",
            RequestParameters::Positional(vec![json!(hex)]),
        )
        .expect("registry");
        // Act
        let error = crate::dispatch::dispatch(&mut context, call).expect_err("unproved fixture");
        // Assert
        assert_eq!(
            error.maybe_detail.expect("detail").code,
            RpcErrorCode::VerifyError
        );
    }
}
