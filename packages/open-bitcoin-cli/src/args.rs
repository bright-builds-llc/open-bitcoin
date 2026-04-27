// Parity breadcrumbs:
// - packages/bitcoin-knots/src/bitcoin-cli.cpp
// - packages/bitcoin-knots/src/common/args.cpp
// - packages/bitcoin-knots/src/common/config.cpp
// - packages/bitcoin-knots/test/functional/interface_bitcoin_cli.py

use std::{ffi::OsString, path::PathBuf};

use open_bitcoin_rpc::{
    RpcFailure,
    method::{RequestParameters, SupportedMethod, normalize_method_call},
};
use serde_json::Value;

use crate::CliError;

/// Parsed `bitcoin-cli` command line input for the supported Phase 8 slice.
#[derive(Debug, Clone, PartialEq)]
pub struct ParsedCli {
    pub startup: CliStartupArgs,
    pub command: CliCommand,
}

/// Startup-related CLI flags that need config or auth resolution.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CliStartupArgs {
    pub maybe_conf_path: Option<PathBuf>,
    pub maybe_data_dir: Option<PathBuf>,
    pub maybe_rpc_connect: Option<String>,
    pub maybe_rpc_port: Option<u16>,
    pub maybe_rpc_user: Option<String>,
    pub maybe_rpc_password: Option<String>,
    pub maybe_rpc_cookie_file: Option<PathBuf>,
    pub maybe_rpc_wallet: Option<String>,
}

impl CliStartupArgs {
    pub(crate) fn to_runtime_config_args(&self) -> Vec<OsString> {
        let mut cli_args = Vec::new();

        if let Some(path) = self.maybe_conf_path.as_ref() {
            cli_args.push(OsString::from(format!("-conf={}", path.display())));
        }
        if let Some(path) = self.maybe_data_dir.as_ref() {
            cli_args.push(OsString::from(format!("-datadir={}", path.display())));
        }
        if let Some(rpc_connect) = self.maybe_rpc_connect.as_ref() {
            cli_args.push(OsString::from(format!("-rpcconnect={rpc_connect}")));
        }
        if let Some(rpc_port) = self.maybe_rpc_port {
            cli_args.push(OsString::from(format!("-rpcport={rpc_port}")));
        }
        if let Some(rpc_user) = self.maybe_rpc_user.as_ref() {
            cli_args.push(OsString::from(format!("-rpcuser={rpc_user}")));
        }
        if let Some(rpc_password) = self.maybe_rpc_password.as_ref() {
            cli_args.push(OsString::from(format!("-rpcpassword={rpc_password}")));
        }
        if let Some(cookie_file) = self.maybe_rpc_cookie_file.as_ref() {
            cli_args.push(OsString::from(format!(
                "-rpccookiefile={}",
                cookie_file.display()
            )));
        }

        cli_args
    }
}

/// Supported top-level command shapes before HTTP transport exists.
#[derive(Debug, Clone, PartialEq)]
pub enum CliCommand {
    GetInfo(GetInfoCommand),
    RpcMethod(RpcMethodCommand),
}

/// Deferred `-getinfo` rendering or transport options.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetInfoCommand {
    pub color: ColorSetting,
    pub helper_args: Vec<String>,
}

/// Parsed Phase 8 RPC method invocation.
#[derive(Debug, Clone, PartialEq)]
pub struct RpcMethodCommand {
    pub method: String,
    pub params: RequestParameters,
}

/// Supported `-color` values for `-getinfo`.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ColorSetting {
    Always,
    #[default]
    Auto,
    Never,
}

impl ColorSetting {
    fn parse(value: &str) -> Result<Self, CliError> {
        match value {
            "always" => Ok(Self::Always),
            "auto" => Ok(Self::Auto),
            "never" => Ok(Self::Never),
            _ => Err(CliError::new(
                "Invalid value for -color option. Valid values: always, auto, never.",
            )),
        }
    }
}

/// Parse the supported Phase 8 `bitcoin-cli` flags and command shape.
pub fn parse_cli_args(cli_args: &[OsString], stdin: &str) -> Result<ParsedCli, CliError> {
    let mut startup = CliStartupArgs::default();
    let mut maybe_color = None;
    let mut named = false;
    let mut stdin_enabled = false;
    let mut stdinrpcpass_enabled = false;
    let mut getinfo_enabled = false;
    let mut free_args = Vec::new();
    let mut command_started = false;

    for cli_arg in cli_args {
        let token = cli_arg.to_string_lossy().into_owned();
        if token == "--json" {
            free_args.push(token);
            command_started = true;
            continue;
        }

        if !token.starts_with('-') || token == "-" {
            free_args.push(token);
            command_started = true;
            continue;
        }

        if command_started && !is_known_flag(&token) {
            free_args.push(token);
            continue;
        }

        let normalized = token.trim_start_matches('-');
        let (raw_key, maybe_value) = match normalized.split_once('=') {
            Some((key, value)) => (key, Some(value)),
            None => (normalized, None),
        };
        let (key, negated) = raw_key
            .strip_prefix("no")
            .map_or((raw_key, false), |stripped| (stripped, true));

        match key {
            "named" => named = parse_bool_flag(maybe_value, negated, &token)?,
            "stdin" => stdin_enabled = parse_bool_flag(maybe_value, negated, &token)?,
            "stdinrpcpass" => {
                stdinrpcpass_enabled = parse_bool_flag(maybe_value, negated, &token)?;
            }
            "getinfo" => getinfo_enabled = parse_bool_flag(maybe_value, negated, &token)?,
            "color" => {
                let Some(value) = maybe_value else {
                    return Err(CliError::new(
                        "Error parsing command line arguments: Can not set -color with no value. Please specify value with -color=value.",
                    ));
                };
                maybe_color = Some(ColorSetting::parse(value)?);
            }
            "conf" => {
                let Some(value) = maybe_value else {
                    return Err(CliError::new(
                        "Error parsing command line arguments: Can not set -conf with no value. Please specify value with -conf=value.",
                    ));
                };
                startup.maybe_conf_path = Some(PathBuf::from(value));
            }
            "datadir" => {
                let Some(value) = maybe_value else {
                    return Err(CliError::new(
                        "Error parsing command line arguments: Can not set -datadir with no value. Please specify value with -datadir=value.",
                    ));
                };
                startup.maybe_data_dir = Some(PathBuf::from(value));
            }
            "rpcconnect" => {
                let Some(value) = maybe_value else {
                    return Err(CliError::new(
                        "Error parsing command line arguments: Can not set -rpcconnect with no value. Please specify value with -rpcconnect=value.",
                    ));
                };
                startup.maybe_rpc_connect = Some(validate_rpc_connect(value)?);
            }
            "rpcport" => {
                let Some(value) = maybe_value else {
                    return Err(CliError::new(
                        "Error parsing command line arguments: Can not set -rpcport with no value. Please specify value with -rpcport=value.",
                    ));
                };
                startup.maybe_rpc_port = Some(parse_rpc_port(value)?);
            }
            "rpcuser" => {
                startup.maybe_rpc_user = Some(maybe_value.unwrap_or_default().to_string());
            }
            "rpcpassword" => {
                startup.maybe_rpc_password = Some(maybe_value.unwrap_or_default().to_string());
            }
            "rpccookiefile" => {
                startup.maybe_rpc_cookie_file =
                    Some(PathBuf::from(maybe_value.unwrap_or_default()));
            }
            "rpcwallet" => {
                let Some(value) = maybe_value else {
                    return Err(CliError::new(
                        "Error parsing command line arguments: Can not set -rpcwallet with no value. Please specify value with -rpcwallet=value.",
                    ));
                };
                startup.maybe_rpc_wallet = Some(value.to_string());
            }
            "netinfo" => {
                return Err(CliError::new(
                    "-netinfo is deferred until the getpeerinfo-backed network dashboard lands in a later Phase 8 plan.",
                ));
            }
            _ => {
                return Err(CliError::new(format!(
                    "Error parsing command line arguments: Invalid parameter {token}"
                )));
            }
        }
    }

    let mut stdin_lines = stdin.lines();
    if stdinrpcpass_enabled {
        let Some(password) = stdin_lines.next() else {
            return Err(CliError::new(
                "-stdinrpcpass specified but failed to read from standard input",
            ));
        };
        startup.maybe_rpc_password = Some(password.to_string());
    }
    if stdin_enabled {
        free_args.extend(stdin_lines.map(str::to_owned));
    }

    if getinfo_enabled {
        return Ok(ParsedCli {
            startup,
            command: CliCommand::GetInfo(GetInfoCommand {
                color: maybe_color.unwrap_or_default(),
                helper_args: free_args,
            }),
        });
    }

    let Some((method, raw_params)) = free_args.split_first() else {
        return Err(CliError::new(
            "error: too few parameters (need at least command)",
        ));
    };
    let params = if named {
        parse_named_parameters(raw_params)?
    } else {
        request_parameters_from_positional(raw_params)
    };
    validate_supported_method(method, &params)?;

    Ok(ParsedCli {
        startup,
        command: CliCommand::RpcMethod(RpcMethodCommand {
            method: method.clone(),
            params,
        }),
    })
}

pub fn stdin_required_for_args(cli_args: &[OsString]) -> bool {
    cli_args
        .iter()
        .filter_map(|cli_arg| maybe_stdin_flag_enabled(&cli_arg.to_string_lossy()))
        .any(|enabled| enabled)
}

fn maybe_stdin_flag_enabled(token: &str) -> Option<bool> {
    if !token.starts_with('-') || token == "-" {
        return None;
    }

    let normalized = token.trim_start_matches('-');
    let (raw_key, maybe_value) = match normalized.split_once('=') {
        Some((key, value)) => (key, Some(value)),
        None => (normalized, None),
    };
    let (key, negated) = raw_key
        .strip_prefix("no")
        .map_or((raw_key, false), |stripped| (stripped, true));
    if key != "stdin" && key != "stdinrpcpass" {
        return None;
    }

    parse_bool_flag(maybe_value, negated, token).ok()
}

fn is_known_flag(token: &str) -> bool {
    let normalized = token.trim_start_matches('-');
    let raw_key = normalized
        .split_once('=')
        .map_or(normalized, |(key, _)| key);
    let key = raw_key.strip_prefix("no").unwrap_or(raw_key);
    matches!(
        key,
        "conf"
            | "datadir"
            | "named"
            | "stdin"
            | "stdinrpcpass"
            | "rpcconnect"
            | "rpcport"
            | "rpcuser"
            | "rpcpassword"
            | "rpccookiefile"
            | "getinfo"
            | "color"
            | "netinfo"
            | "rpcwallet"
    )
}

fn parse_bool_flag(
    maybe_value: Option<&str>,
    negated: bool,
    raw_token: &str,
) -> Result<bool, CliError> {
    let value = maybe_value.unwrap_or("1");
    let parsed = match value {
        "" => true,
        "0" | "false" => false,
        "1" | "true" => true,
        _ => value
            .parse::<i64>()
            .map(|number| number != 0)
            .map_err(|_| {
                CliError::new(format!(
                    "Error parsing command line arguments: Invalid parameter {raw_token}"
                ))
            })?,
    };

    if negated { Ok(!parsed) } else { Ok(parsed) }
}

fn parse_rpc_port(value: &str) -> Result<u16, CliError> {
    parse_nonzero_port(value)
        .map_err(|_| CliError::new(format!("Invalid port provided in -rpcport: {value}")))
}

fn validate_rpc_connect(value: &str) -> Result<String, CliError> {
    parse_rpc_connect_target(value)
        .map(|_| value.to_string())
        .map_err(|_| CliError::new(format!("Invalid port provided in -rpcconnect: {value}")))
}

fn parse_nonzero_port(value: &str) -> Result<u16, ()> {
    let port = value.parse::<u16>().map_err(|_| ())?;
    if port == 0 {
        return Err(());
    }
    Ok(port)
}

fn parse_rpc_connect_target(value: &str) -> Result<(), ()> {
    if let Some(stripped) = value.strip_prefix('[') {
        let Some(end_index) = stripped.find(']') else {
            return Ok(());
        };
        let remainder = &stripped[end_index + 1..];
        if remainder.is_empty() {
            return Ok(());
        }
        let Some(port) = remainder.strip_prefix(':') else {
            return Err(());
        };
        return parse_nonzero_port(port).map(|_| ());
    }

    if value.matches(':').count() == 1 {
        let Some((_, port)) = value.rsplit_once(':') else {
            return Ok(());
        };
        return parse_nonzero_port(port).map(|_| ());
    }

    Ok(())
}

fn parse_named_parameters(raw_params: &[String]) -> Result<RequestParameters, CliError> {
    let mut positional = Vec::new();
    let mut named = Vec::new();
    let mut maybe_explicit_args = None;

    for raw_param in raw_params {
        let Some((name, value)) = raw_param.split_once('=') else {
            if maybe_explicit_args.is_some() {
                return Err(CliError::new("Parameter args specified multiple times"));
            }
            positional.push(parse_cli_value(raw_param));
            continue;
        };

        if name == "args" {
            if maybe_explicit_args.is_some() || !positional.is_empty() {
                return Err(CliError::new("Parameter args specified multiple times"));
            }
            let parsed_args = parse_cli_value(value);
            let Value::Array(values) = parsed_args else {
                return Err(CliError::new("Parameter args must be a JSON array"));
            };
            maybe_explicit_args = Some(values);
            continue;
        }

        named.push((name.to_string(), parse_cli_value(value)));
    }

    let positional = maybe_explicit_args.unwrap_or(positional);
    match (positional.is_empty(), named.is_empty()) {
        (true, true) => Ok(RequestParameters::None),
        (false, true) => Ok(RequestParameters::Positional(positional)),
        (true, false) => Ok(RequestParameters::Named(named)),
        (false, false) => Ok(RequestParameters::Mixed { positional, named }),
    }
}

fn request_parameters_from_positional(raw_params: &[String]) -> RequestParameters {
    if raw_params.is_empty() {
        return RequestParameters::None;
    }

    RequestParameters::Positional(
        raw_params
            .iter()
            .map(|param| parse_cli_value(param))
            .collect(),
    )
}

fn parse_cli_value(raw_value: &str) -> Value {
    serde_json::from_str::<Value>(raw_value)
        .unwrap_or_else(|_| Value::String(raw_value.to_string()))
}

fn validate_supported_method(method: &str, params: &RequestParameters) -> Result<(), CliError> {
    if SupportedMethod::from_name(method).is_none() {
        return Ok(());
    }

    normalize_method_call(method, params.clone())
        .map(|_| ())
        .map_err(map_rpc_failure)
}

fn map_rpc_failure(failure: RpcFailure) -> CliError {
    let Some(detail) = failure.maybe_detail else {
        return CliError::new("Authentication failed");
    };

    if let Some(parameter_message) = parameter_error_message(&detail.message) {
        return CliError::new(parameter_message);
    }

    CliError::new(detail.message)
}

fn parameter_error_message(message: &str) -> Option<String> {
    let maybe_collision_name = message
        .strip_prefix("named parameter ")?
        .strip_suffix(" collides with a positional argument");
    if let Some(name) = maybe_collision_name {
        return Some(format!(
            "Parameter {name} specified twice both as positional and named argument"
        ));
    }

    let name = message
        .strip_prefix("named parameter ")?
        .strip_suffix(" was provided multiple times")?;
    Some(format!("Parameter {name} specified multiple times"))
}

#[cfg(test)]
mod transport_tests {
    use std::ffi::OsString;

    use super::{CliCommand, RequestParameters, parse_cli_args};

    fn os(value: &str) -> OsString {
        OsString::from(value)
    }

    #[test]
    fn rpcwallet_is_preserved_as_transport_metadata() {
        // Arrange
        let cli_args = [os("-rpcwallet=alpha"), os("getwalletinfo")];

        // Act
        let parsed = parse_cli_args(&cli_args, "").expect("parsed cli");

        // Assert
        assert_eq!(parsed.startup.maybe_rpc_wallet.as_deref(), Some("alpha"));
        assert_eq!(
            parsed.startup.to_runtime_config_args(),
            Vec::<OsString>::new()
        );
        assert_eq!(
            parsed.command,
            CliCommand::RpcMethod(super::RpcMethodCommand {
                method: "getwalletinfo".to_string(),
                params: RequestParameters::None,
            }),
        );
    }
}

#[cfg(test)]
mod tests;
