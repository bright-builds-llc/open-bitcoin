use std::{collections::BTreeMap, ffi::OsString, fs, path::Path};

use open_bitcoin_cli::{
    args::{CliCommand, ParsedCli, parse_cli_args},
    getinfo::{GetInfoBatch, GetInfoSnapshot, build_getinfo_batch, render_getinfo},
    startup::{CliRpcConfig, CliStartupConfig, resolve_startup_config},
};
use open_bitcoin_rpc::{
    JsonRpcId, JsonRpcVersion, RpcAuthConfig, RpcErrorDetail, RpcRequestEnvelope,
    method::{MethodCall, RequestParameters, SupportedMethod, normalize_method_call},
};
use serde::Serialize;
use serde_json::Value;
use ureq::Agent;

use crate::output::CliCommandFailure;

pub fn run_cli(
    cli_args: &[OsString],
    stdin: &str,
    default_data_dir: &Path,
) -> Result<String, CliCommandFailure> {
    let parsed = parse_cli_args(cli_args, stdin).map_err(CliCommandFailure::from_cli_error)?;
    let startup = resolve_startup_config(&parsed.startup, default_data_dir)
        .map_err(CliCommandFailure::from_cli_error)?;
    execute_parsed_cli(&parsed, &startup)
}

pub fn execute_parsed_cli(
    parsed: &ParsedCli,
    startup: &CliStartupConfig,
) -> Result<String, CliCommandFailure> {
    let client = RpcHttpClient::from_config(&startup.rpc)?;

    match &parsed.command {
        CliCommand::GetInfo(command) => {
            let batch = build_getinfo_batch(command).map_err(CliCommandFailure::from_cli_error)?;
            let snapshot = client.getinfo_snapshot(&batch)?;
            render_getinfo(&snapshot, batch.output_mode, batch.color)
                .map_err(CliCommandFailure::from_cli_error)
        }
        CliCommand::RpcMethod(command) => {
            let result = client.call_method(&command.method, command.params.clone())?;
            crate::output::render_rpc_result(&result)
        }
    }
}

struct RpcHttpClient {
    agent: Agent,
    endpoint_url: String,
    endpoint_display: String,
    authorization_header: String,
}

impl RpcHttpClient {
    fn from_config(config: &CliRpcConfig) -> Result<Self, CliCommandFailure> {
        Ok(Self {
            agent: Agent::new_with_config(
                Agent::config_builder().http_status_as_error(false).build(),
            ),
            endpoint_url: rpc_endpoint_url(config),
            endpoint_display: rpc_endpoint_display(config),
            authorization_header: authorization_header(&config.auth)?,
        })
    }

    fn call_method(
        &self,
        method_name: &str,
        params: RequestParameters,
    ) -> Result<Value, CliCommandFailure> {
        let request = build_request_envelope(method_name, params, 1)?;
        let response = self.post_json(&request)?;
        extract_result(response)
    }

    fn getinfo_snapshot(&self, batch: &GetInfoBatch) -> Result<GetInfoSnapshot, CliCommandFailure> {
        let requests = batch
            .calls
            .iter()
            .enumerate()
            .map(|(index, call)| {
                build_request_envelope(call.method.name(), call.params.clone(), index as i64)
            })
            .collect::<Result<Vec<_>, _>>()?;
        let response = self.post_json(&requests)?;
        let results_by_id = parse_batch_results(response)?;

        Ok(GetInfoSnapshot {
            network: decode_batch_result(&results_by_id, 0)?,
            blockchain: decode_batch_result(&results_by_id, 1)?,
            maybe_wallet: Some(decode_batch_result(&results_by_id, 2)?),
            maybe_balances: Some(decode_batch_result(&results_by_id, 3)?),
        })
    }

    fn post_json<T: Serialize>(&self, body: &T) -> Result<Value, CliCommandFailure> {
        let response = self
            .agent
            .post(&self.endpoint_url)
            .header("Authorization", &self.authorization_header)
            .send_json(body)
            .map_err(|error| {
                CliCommandFailure::connection_failure(&self.endpoint_display, &error)
            })?;
        let status = response.status().as_u16();
        if status == 401 {
            return Err(CliCommandFailure::authentication_failed());
        }
        if status != 200 {
            return Err(CliCommandFailure::http_status_failure(
                &self.endpoint_display,
                status,
            ));
        }

        response
            .into_body()
            .read_json()
            .map_err(|error| CliCommandFailure::invalid_response(error.to_string()))
    }
}

fn build_request_envelope(
    method_name: &str,
    params: RequestParameters,
    id: i64,
) -> Result<RpcRequestEnvelope<Value>, CliCommandFailure> {
    Ok(RpcRequestEnvelope {
        jsonrpc: Some(JsonRpcVersion::V2),
        method: method_name.to_string(),
        params: canonical_params(method_name, params)?,
        id: Some(JsonRpcId::Number(id)),
    })
}

fn canonical_params(
    method_name: &str,
    params: RequestParameters,
) -> Result<Value, CliCommandFailure> {
    if SupportedMethod::from_name(method_name).is_none() {
        return request_parameters_to_json(method_name, params);
    }

    let call =
        normalize_method_call(method_name, params).map_err(CliCommandFailure::from_rpc_failure)?;
    method_call_to_json(call)
}

fn method_call_to_json(call: MethodCall) -> Result<Value, CliCommandFailure> {
    match call {
        MethodCall::GetBlockchainInfo(request) => to_json_value(request),
        MethodCall::GetMempoolInfo(request) => to_json_value(request),
        MethodCall::GetNetworkInfo(request) => to_json_value(request),
        MethodCall::SendRawTransaction(request) => to_json_value(request),
        MethodCall::DeriveAddresses(request) => to_json_value(request),
        MethodCall::GetWalletInfo(request) => to_json_value(request),
        MethodCall::GetBalances(request) => to_json_value(request),
        MethodCall::ListUnspent(request) => to_json_value(request),
        MethodCall::ImportDescriptors(request) => to_json_value(request),
        MethodCall::RescanBlockchain(request) => Ok(serde_json::json!({
            "start_height": request.maybe_start_height,
            "stop_height": request.maybe_stop_height,
        })),
        MethodCall::BuildTransaction(request) => to_json_value(request),
        MethodCall::BuildAndSignTransaction(request) => to_json_value(request),
    }
}

fn to_json_value<T: Serialize>(value: T) -> Result<Value, CliCommandFailure> {
    serde_json::to_value(value)
        .map_err(|error| CliCommandFailure::invalid_response(error.to_string()))
}

fn request_parameters_to_json(
    method_name: &str,
    params: RequestParameters,
) -> Result<Value, CliCommandFailure> {
    match params {
        RequestParameters::None => Ok(Value::Array(Vec::new())),
        RequestParameters::Positional(values) => Ok(Value::Array(values)),
        RequestParameters::Named(values) => {
            let object = values
                .into_iter()
                .collect::<serde_json::Map<String, Value>>();
            Ok(Value::Object(object))
        }
        RequestParameters::Mixed { .. } => Err(CliCommandFailure::new(format!(
            "Method {method_name} requires shared Phase 8 metadata before mixed named and positional parameters can be encoded over HTTP",
        ))),
    }
}

fn parse_batch_results(response: Value) -> Result<BTreeMap<i64, Value>, CliCommandFailure> {
    let Value::Array(items) = response else {
        return Err(CliCommandFailure::invalid_response(
            "batch RPC response must be a JSON array".to_string(),
        ));
    };

    let mut results = BTreeMap::new();
    for item in items {
        let id = extract_numeric_id(&item)?;
        let result = extract_result(item)?;
        if results.insert(id, result).is_some() {
            return Err(CliCommandFailure::invalid_response(format!(
                "batch RPC response repeated id {id}",
            )));
        }
    }

    Ok(results)
}

fn extract_numeric_id(response: &Value) -> Result<i64, CliCommandFailure> {
    let Some(id) = response.get("id") else {
        return Err(CliCommandFailure::invalid_response(
            "RPC response is missing id".to_string(),
        ));
    };
    let Some(id) = id.as_i64() else {
        return Err(CliCommandFailure::invalid_response(
            "RPC response id must be a number".to_string(),
        ));
    };
    Ok(id)
}

fn decode_batch_result<T: serde::de::DeserializeOwned>(
    results_by_id: &BTreeMap<i64, Value>,
    id: i64,
) -> Result<T, CliCommandFailure> {
    let Some(value) = results_by_id.get(&id) else {
        return Err(CliCommandFailure::invalid_response(format!(
            "batch RPC response is missing id {id}",
        )));
    };

    serde_json::from_value(value.clone())
        .map_err(|error| CliCommandFailure::invalid_response(error.to_string()))
}

fn extract_result(response: Value) -> Result<Value, CliCommandFailure> {
    let Value::Object(object) = response else {
        return Err(CliCommandFailure::invalid_response(
            "RPC response must be an object".to_string(),
        ));
    };

    if let Some(error) = object.get("error") {
        if error.is_null() {
            return Ok(object.get("result").cloned().unwrap_or(Value::Null));
        }

        let detail: RpcErrorDetail =
            serde_json::from_value(error.clone()).map_err(|decode_error| {
                CliCommandFailure::invalid_response(decode_error.to_string())
            })?;
        return Err(CliCommandFailure::from_rpc_error_detail(detail));
    }

    object.get("result").cloned().ok_or_else(|| {
        CliCommandFailure::invalid_response("RPC response is missing result".to_string())
    })
}

fn authorization_header(auth: &RpcAuthConfig) -> Result<String, CliCommandFailure> {
    let credentials = match auth {
        RpcAuthConfig::UserPassword { username, password } => {
            format!("{username}:{password}")
        }
        RpcAuthConfig::Cookie { maybe_cookie_file } => {
            let cookie_file = maybe_cookie_file
                .clone()
                .unwrap_or_else(|| std::path::PathBuf::from(".cookie"));
            let contents = fs::read_to_string(&cookie_file).map_err(|_| {
                CliCommandFailure::new(format!(
                    "Could not locate RPC credentials. No authentication cookie was found at {}",
                    cookie_file.display(),
                ))
            })?;
            let Some((username, password)) = contents.trim().split_once(':') else {
                return Err(CliCommandFailure::new(format!(
                    "Could not parse RPC credentials from {}",
                    cookie_file.display(),
                )));
            };
            format!("{username}:{password}")
        }
    };

    Ok(format!("Basic {}", base64_encode(credentials.as_bytes())))
}

fn rpc_endpoint_url(config: &CliRpcConfig) -> String {
    format!("http://{}/", format_host_for_url(&config.host, config.port),)
}

fn rpc_endpoint_display(config: &CliRpcConfig) -> String {
    format_host_for_url(&config.host, config.port)
}

fn format_host_for_url(host: &str, port: u16) -> String {
    if host.contains(':') && !host.starts_with('[') {
        format!("[{host}]:{port}")
    } else {
        format!("{host}:{port}")
    }
}

fn base64_encode(bytes: &[u8]) -> String {
    const TABLE: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut output = String::with_capacity(bytes.len().div_ceil(3) * 4);

    for chunk in bytes.chunks(3) {
        let b0 = chunk[0];
        let b1 = *chunk.get(1).unwrap_or(&0);
        let b2 = *chunk.get(2).unwrap_or(&0);
        let triple = ((b0 as u32) << 16) | ((b1 as u32) << 8) | (b2 as u32);

        output.push(TABLE[((triple >> 18) & 0x3f) as usize] as char);
        output.push(TABLE[((triple >> 12) & 0x3f) as usize] as char);
        if chunk.len() > 1 {
            output.push(TABLE[((triple >> 6) & 0x3f) as usize] as char);
        } else {
            output.push('=');
        }
        if chunk.len() > 2 {
            output.push(TABLE[(triple & 0x3f) as usize] as char);
        } else {
            output.push('=');
        }
    }

    output
}

#[cfg(test)]
mod tests;
