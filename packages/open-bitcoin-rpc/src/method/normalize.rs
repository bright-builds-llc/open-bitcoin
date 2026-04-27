use serde::de::DeserializeOwned;
use serde_json::{Map, Value};

use crate::error::RpcFailure;

use super::RequestParameters;

pub(super) fn normalize_request<T: DeserializeOwned>(
    ordered_fields: &[&str],
    params: RequestParameters,
) -> Result<T, RpcFailure> {
    let (positional, named) = match params {
        RequestParameters::None => (Vec::new(), Vec::new()),
        RequestParameters::Positional(values) => (values, Vec::new()),
        RequestParameters::Named(values) => (Vec::new(), values),
        RequestParameters::Mixed { positional, named } => (positional, named),
    };

    if positional.len() > ordered_fields.len() {
        return Err(RpcFailure::invalid_params(format!(
            "too many positional parameters: expected at most {}, got {}",
            ordered_fields.len(),
            positional.len()
        )));
    }

    let positional_fields = ordered_fields
        .iter()
        .take(positional.len())
        .copied()
        .collect::<Vec<_>>();
    let mut object = Map::new();
    for (index, value) in positional.into_iter().enumerate() {
        object.insert(ordered_fields[index].to_string(), value);
    }

    for (name, value) in named {
        if !ordered_fields.iter().any(|allowed| *allowed == name) {
            return Err(RpcFailure::invalid_params(format!(
                "unknown named parameter {name}"
            )));
        }
        if object.contains_key(&name) {
            if positional_fields.iter().any(|field| *field == name) {
                return Err(RpcFailure::invalid_params(format!(
                    "named parameter {name} collides with a positional argument"
                )));
            }
            return Err(RpcFailure::invalid_params(format!(
                "named parameter {name} was provided multiple times"
            )));
        }
        object.insert(name, value);
    }

    serde_json::from_value(Value::Object(object))
        .map_err(|error| RpcFailure::invalid_params(error.to_string()))
}
