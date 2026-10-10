// Parity breadcrumbs:
// - packages/bitcoin-knots/src/rpc/server.cpp
// - packages/bitcoin-knots/src/rpc/util.cpp
// - packages/bitcoin-knots/src/rpc/blockchain.cpp
// - packages/bitcoin-knots/src/rpc/node.cpp
// - packages/bitcoin-knots/src/univalue/lib/univalue.cpp
// - packages/bitcoin-knots/src/uint256.h

use std::collections::HashMap;

use open_bitcoin_node::core::primitives::BlockHash;
use serde_json::Value;

use super::{
    BlockFilterSelection, GETBLOCKFILTER_HELP, GETINDEXINFO_HELP, GetBlockFilterRequest,
    GetIndexInfoRequest, filter_failure,
};
use crate::{
    error::{RpcErrorCode, RpcFailure},
    method::RequestParameters,
};

/// Normalize pinned framework arguments before consulting any index/backend state.
pub fn normalize_getblockfilter(
    params: RequestParameters,
) -> Result<GetBlockFilterRequest, RpcFailure> {
    let values = framework_arguments(params, &["blockhash", "filtertype"], 1, GETBLOCKFILTER_HELP)?;
    let Some(hash) = values.first().and_then(Value::as_str) else {
        return Err(RpcFailure::internal_error("Invalid normalized blockhash"));
    };
    let block_hash = parse_hash(hash)?;
    let filter_name = values.get(1).and_then(Value::as_str).unwrap_or("basic");
    Ok(GetBlockFilterRequest {
        block_hash,
        filter_type: BlockFilterSelection::parse(filter_name)?,
    })
}

/// Normalize the optional getindexinfo selector without inventing enabled indexes.
pub fn normalize_getindexinfo(
    params: RequestParameters,
) -> Result<GetIndexInfoRequest, RpcFailure> {
    let values = framework_arguments(params, &["index_name"], 0, GETINDEXINFO_HELP)?;
    Ok(GetIndexInfoRequest {
        maybe_index_name: values.first().and_then(Value::as_str).map(str::to_owned),
    })
}

fn framework_arguments(
    params: RequestParameters,
    names: &[&str],
    required: usize,
    help: &str,
) -> Result<Vec<Value>, RpcFailure> {
    let values = match params {
        RequestParameters::None => Vec::new(),
        RequestParameters::Positional(values) => values,
        RequestParameters::Named(named) => transform_named(named, names)?,
        RequestParameters::Mixed {
            positional,
            mut named,
        } => {
            named.insert(0, ("args".to_owned(), Value::Array(positional)));
            transform_named(named, names)?
        }
    };
    if values.len() < required || values.len() > names.len() {
        return Err(filter_failure(RpcErrorCode::MiscError, help));
    }
    let mismatches = names.iter().enumerate().filter_map(|(index, name)| {
        let value = values.get(index).unwrap_or(&Value::Null);
        if value.is_string() || (index >= required && value.is_null()) {
            return None;
        }
        Some(format!(
            "    \"Position {} ({name})\": \"JSON value of type {} is not of expected type string\"",
            index + 1, json_type_name(value),
        ))
    }).collect::<Vec<_>>();
    if !mismatches.is_empty() {
        return Err(filter_failure(
            RpcErrorCode::TypeError,
            format!("Wrong type passed:\n{{\n{}\n}}", mismatches.join(",\n")),
        ));
    }
    Ok(values)
}

fn transform_named(named: Vec<(String, Value)>, names: &[&str]) -> Result<Vec<Value>, RpcFailure> {
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
    for (index, name) in names.iter().enumerate() {
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
    // Knots likewise selects one leftover unordered-map entry when several names are unknown.
    if let Some(name) = arguments.keys().next() {
        return Err(RpcFailure::invalid_parameter(format!(
            "Unknown named parameter {name}"
        )));
    }
    Ok(values)
}

fn json_type_name(value: &Value) -> &'static str {
    match value {
        Value::Null => "null",
        Value::Bool(_) => "bool",
        Value::Number(_) => "number",
        Value::String(_) => "string",
        Value::Array(_) => "array",
        Value::Object(_) => "object",
    }
}

fn parse_hash(hash: &str) -> Result<BlockHash, RpcFailure> {
    if hash.len() != 64 {
        return Err(RpcFailure::invalid_parameter(format!(
            "blockhash must be of length 64 (not {}, for '{hash}')",
            hash.len()
        )));
    }
    let invalid_hex = || {
        RpcFailure::invalid_parameter(format!(
            "blockhash must be hexadecimal string (not '{hash}')"
        ))
    };
    let mut raw = [0_u8; 32];
    for (output, pair) in raw.iter_mut().rev().zip(hash.as_bytes().chunks_exact(2)) {
        let high = maybe_hex_nibble(pair[0]).ok_or_else(invalid_hex)?;
        let low = maybe_hex_nibble(pair[1]).ok_or_else(invalid_hex)?;
        *output = high * 16 + low;
    }
    Ok(BlockHash::from_byte_array(raw))
}

fn maybe_hex_nibble(byte: u8) -> Option<u8> {
    match byte {
        b'0'..=b'9' => Some(byte - b'0'),
        b'a'..=b'f' => Some(byte - b'a' + 10),
        b'A'..=b'F' => Some(byte - b'A' + 10),
        _ => None,
    }
}
