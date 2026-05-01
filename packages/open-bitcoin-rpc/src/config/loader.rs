// Parity breadcrumbs:
// - packages/bitcoin-knots/src/common/args.cpp
// - packages/bitcoin-knots/src/common/config.cpp
// - packages/bitcoin-knots/src/httprpc.cpp
// - packages/bitcoin-knots/src/rpc/server.cpp

use std::{
    collections::HashSet,
    env,
    ffi::OsString,
    fs::File,
    io::{BufRead, BufReader},
    net::{IpAddr, SocketAddr},
    path::{Path, PathBuf},
};

use open_bitcoin_node::core::wallet::AddressNetwork;

use super::{
    ConfigError, DEFAULT_COOKIE_FILE_NAME, DEFAULT_RPC_HOST, DaemonSyncMode, RpcAuthConfig,
    RpcClientConfig, RpcServerConfig, RuntimeConfig, WalletRuntimeConfig, default_rpc_port,
};

mod chain;
mod open_bitcoin_runtime;
mod rpc_address;

use chain::{
    config_section_name, determine_chain, parse_chain_key, parse_chain_name, supported_chain_key,
};
use open_bitcoin_runtime::{load_open_bitcoin_config, resolve_daemon_sync_config};
use rpc_address::parse_rpc_client_address;

const BITCOIN_CONF_FILE_NAME: &str = "bitcoin.conf";
#[derive(Debug, Clone, Default)]
struct CliSettings {
    maybe_chain: Option<AddressNetwork>,
    maybe_conf_path: Option<PathBuf>,
    maybe_data_dir: Option<PathBuf>,
    maybe_server: Option<bool>,
    maybe_rpc_bind: Option<String>,
    maybe_rpc_port: Option<u16>,
    maybe_rpc_connect: Option<String>,
    maybe_rpc_user: Option<String>,
    maybe_rpc_password: Option<String>,
    maybe_cookie_file: Option<PathBuf>,
    maybe_open_bitcoin_config_path: Option<PathBuf>,
    maybe_daemon_sync_mode: Option<DaemonSyncMode>,
}

#[derive(Debug, Clone)]
struct ConfigEntry {
    maybe_section: Option<String>,
    key: String,
    value: String,
}

#[derive(Debug, Clone, Default)]
struct FileSettings {
    maybe_data_dir: Option<PathBuf>,
    maybe_server: Option<bool>,
    maybe_rpc_bind: Option<String>,
    maybe_rpc_port: Option<u16>,
    maybe_rpc_connect: Option<String>,
    maybe_rpc_user: Option<String>,
    maybe_rpc_password: Option<String>,
    maybe_cookie_file: Option<PathBuf>,
}

pub(super) fn load_runtime_config() -> Result<RuntimeConfig, ConfigError> {
    let cli_args = env::args_os().skip(1).collect::<Vec<_>>();
    let default_data_dir = default_data_dir();
    load_runtime_config_for_args(&cli_args, &default_data_dir)
}

pub(super) fn load_runtime_config_for_args(
    cli_args: &[OsString],
    default_data_dir: &Path,
) -> Result<RuntimeConfig, ConfigError> {
    let cli = parse_cli_args(cli_args)?;
    let initial_data_dir = cli
        .maybe_data_dir
        .clone()
        .unwrap_or_else(|| default_data_dir.to_path_buf());
    let conf_path = cli
        .maybe_conf_path
        .clone()
        .map(|path| resolve_path(&path, &initial_data_dir))
        .unwrap_or_else(|| initial_data_dir.join(BITCOIN_CONF_FILE_NAME));
    let config_entries =
        parse_config_entries(&conf_path, cli.maybe_conf_path.is_some(), &initial_data_dir)?;
    let chain = determine_chain(cli.maybe_chain, &config_entries)?;
    let file_settings = collect_file_settings(&config_entries, chain)?;
    let maybe_data_dir = cli
        .maybe_data_dir
        .clone()
        .or_else(|| {
            file_settings
                .maybe_data_dir
                .clone()
                .map(|path| resolve_path(&path, &initial_data_dir))
        })
        .or_else(|| {
            if default_data_dir.exists() {
                Some(default_data_dir.to_path_buf())
            } else {
                None
            }
        });
    let maybe_data_dir = maybe_data_dir.filter(|path| !path.as_os_str().is_empty());
    if let Some(data_dir) = maybe_data_dir.as_ref()
        && !data_dir.exists()
    {
        return Err(ConfigError::new(format!(
            "Error reading configuration file: specified data directory \"{}\" does not exist.",
            data_dir.display()
        )));
    }
    let open_bitcoin_base_dir = maybe_data_dir.as_deref().unwrap_or(&initial_data_dir);
    let maybe_open_bitcoin_config = load_open_bitcoin_config(&cli, open_bitcoin_base_dir)?;
    let sync = resolve_daemon_sync_config(
        chain,
        cli.maybe_daemon_sync_mode,
        maybe_open_bitcoin_config.as_ref(),
    )?;

    let rpc_bind = cli
        .maybe_rpc_bind
        .clone()
        .or(file_settings.maybe_rpc_bind)
        .unwrap_or_else(|| DEFAULT_RPC_HOST.to_string());
    let rpc_connect = cli
        .maybe_rpc_connect
        .clone()
        .or(file_settings.maybe_rpc_connect)
        .unwrap_or_else(|| DEFAULT_RPC_HOST.to_string());
    let maybe_explicit_rpc_port = cli.maybe_rpc_port.or(file_settings.maybe_rpc_port);
    let rpc_port = maybe_explicit_rpc_port.unwrap_or_else(|| default_rpc_port(chain));
    let auth = resolve_auth(
        cli.maybe_rpc_user.clone().or(file_settings.maybe_rpc_user),
        cli.maybe_rpc_password
            .clone()
            .or(file_settings.maybe_rpc_password),
        cli.maybe_cookie_file
            .clone()
            .or(file_settings.maybe_cookie_file),
        maybe_data_dir.as_deref(),
    )?;
    let server_enabled = cli
        .maybe_server
        .or(file_settings.maybe_server)
        .unwrap_or(true);

    Ok(RuntimeConfig {
        chain,
        maybe_data_dir,
        rpc_server: RpcServerConfig {
            enabled: server_enabled,
            bind_address: parse_socket_address(&rpc_bind, rpc_port)?,
            auth: auth.clone(),
        },
        rpc_client: RpcClientConfig {
            endpoint: parse_rpc_client_address(
                &rpc_connect,
                maybe_explicit_rpc_port,
                default_rpc_port(chain),
            )?,
            auth,
        },
        wallet: WalletRuntimeConfig::default(),
        sync,
    })
}

fn parse_cli_args(cli_args: &[OsString]) -> Result<CliSettings, ConfigError> {
    let mut settings = CliSettings::default();

    for cli_arg in cli_args {
        let arg = cli_arg.to_string_lossy();
        if !arg.starts_with('-') {
            return Err(ConfigError::new(format!(
                "Error parsing command line arguments: Invalid parameter {arg}"
            )));
        }

        let normalized = arg.trim_start_matches('-');
        let (raw_key, maybe_value) = match normalized.split_once('=') {
            Some((key, value)) => (key, Some(value)),
            None => (normalized, None),
        };
        let (key, negated) = raw_key
            .strip_prefix("no")
            .map_or((raw_key, false), |stripped| (stripped, true));

        match key {
            "server" => {
                settings.maybe_server = Some(parse_bool(maybe_value, negated)?);
            }
            "regtest" | "signet" | "test" | "testnet" | "main" => {
                if parse_bool(maybe_value, negated)? {
                    settings.maybe_chain = Some(parse_chain_key(key)?);
                }
            }
            "chain" => {
                let Some(value) = maybe_value else {
                    return Err(ConfigError::new(
                        "Error parsing command line arguments: Can not set -chain with no value. Please specify value with -chain=value.",
                    ));
                };
                settings.maybe_chain = Some(parse_chain_name(value)?);
            }
            "conf" => {
                let Some(value) = maybe_value else {
                    return Err(ConfigError::new(
                        "Error parsing command line arguments: Can not set -conf with no value. Please specify value with -conf=value.",
                    ));
                };
                settings.maybe_conf_path = Some(PathBuf::from(value));
            }
            "datadir" => {
                let Some(value) = maybe_value else {
                    return Err(ConfigError::new(
                        "Error parsing command line arguments: Can not set -datadir with no value. Please specify value with -datadir=value.",
                    ));
                };
                settings.maybe_data_dir = Some(PathBuf::from(value));
            }
            "rpcbind" => {
                let Some(value) = maybe_value else {
                    return Err(ConfigError::new(
                        "Error parsing command line arguments: Can not set -rpcbind with no value. Please specify value with -rpcbind=value.",
                    ));
                };
                settings.maybe_rpc_bind = Some(value.to_string());
            }
            "rpcport" => {
                let Some(value) = maybe_value else {
                    return Err(ConfigError::new(
                        "Error parsing command line arguments: Can not set -rpcport with no value. Please specify value with -rpcport=value.",
                    ));
                };
                settings.maybe_rpc_port = Some(parse_port(value)?);
            }
            "rpcconnect" => {
                let Some(value) = maybe_value else {
                    return Err(ConfigError::new(
                        "Error parsing command line arguments: Can not set -rpcconnect with no value. Please specify value with -rpcconnect=value.",
                    ));
                };
                settings.maybe_rpc_connect = Some(value.to_string());
            }
            "rpcuser" => {
                settings.maybe_rpc_user = Some(maybe_value.unwrap_or_default().to_string());
            }
            "rpcpassword" => {
                settings.maybe_rpc_password = Some(maybe_value.unwrap_or_default().to_string());
            }
            "rpccookiefile" => {
                settings.maybe_cookie_file = Some(PathBuf::from(maybe_value.unwrap_or_default()));
            }
            "openbitcoinconf" | "openbitcoinconfig" => {
                let Some(value) = maybe_value else {
                    return Err(ConfigError::new(
                        "Error parsing command line arguments: Can not set -openbitcoinconf with no value. Please specify value with -openbitcoinconf=value.",
                    ));
                };
                settings.maybe_open_bitcoin_config_path = Some(PathBuf::from(value));
            }
            "openbitcoinsync" => {
                let Some(value) = maybe_value else {
                    return Err(ConfigError::new(
                        "Error parsing command line arguments: Can not set -openbitcoinsync with no value. Please specify value with -openbitcoinsync=value.",
                    ));
                };
                settings.maybe_daemon_sync_mode = Some(DaemonSyncMode::parse(value)?);
            }
            "includeconf" | "rpcauth" | "rpcwhitelist" => {
                return Err(ConfigError::new(format!(
                    "Error parsing command line arguments: Invalid parameter {arg}"
                )));
            }
            _ => {
                return Err(ConfigError::new(format!(
                    "Error parsing command line arguments: Invalid parameter {arg}"
                )));
            }
        }
    }

    Ok(settings)
}

fn parse_bool(maybe_value: Option<&str>, negated: bool) -> Result<bool, ConfigError> {
    let value = maybe_value.unwrap_or("1");
    let parsed = match value {
        "" => true,
        "0" | "false" => false,
        "1" | "true" => true,
        _ => value
            .parse::<i64>()
            .map(|number| number != 0)
            .map_err(|_| ConfigError::new(format!("invalid boolean value: {value}")))?,
    };
    if negated { Ok(!parsed) } else { Ok(parsed) }
}

fn parse_port(value: &str) -> Result<u16, ConfigError> {
    value
        .parse::<u16>()
        .map_err(|_| ConfigError::new(format!("invalid rpc port: {value}")))
}

fn parse_config_entries(
    conf_path: &Path,
    explicit_conf: bool,
    base_data_dir: &Path,
) -> Result<Vec<ConfigEntry>, ConfigError> {
    if conf_path.is_dir() {
        return Err(ConfigError::new(format!(
            "Error reading configuration file: Config file \"{}\" is a directory.",
            conf_path.display()
        )));
    }
    if !conf_path.exists() {
        if explicit_conf {
            return Err(ConfigError::new(format!(
                "Error reading configuration file: specified config file \"{}\" could not be opened.",
                conf_path.display()
            )));
        }
        return Ok(Vec::new());
    }

    let mut entries = Vec::new();
    let mut visited = HashSet::new();
    parse_config_file(conf_path, &mut entries, &mut visited, base_data_dir)?;
    Ok(entries)
}

fn parse_config_file(
    path: &Path,
    entries: &mut Vec<ConfigEntry>,
    visited: &mut HashSet<PathBuf>,
    base_data_dir: &Path,
) -> Result<(), ConfigError> {
    let resolved_path = resolve_path(path, base_data_dir);
    if !visited.insert(resolved_path.clone()) {
        return Ok(());
    }
    if resolved_path.is_dir() {
        return Err(ConfigError::new(format!(
            "Error reading configuration file: Included config file \"{}\" is a directory.",
            resolved_path.display()
        )));
    }

    let file = File::open(&resolved_path).map_err(|_| {
        ConfigError::new(format!(
            "Error reading configuration file: Failed to include configuration file {}",
            resolved_path.display()
        ))
    })?;
    let reader = BufReader::new(file);
    let mut maybe_current_section: Option<String> = None;
    let mut include_paths = Vec::new();

    for (index, line_result) in reader.lines().enumerate() {
        let line_number = index + 1;
        let line = line_result.map_err(|error| {
            ConfigError::new(format!("Error reading configuration file: {error}"))
        })?;
        let (trimmed, used_hash) = strip_comments(&line);
        if trimmed.is_empty() {
            continue;
        }
        if trimmed.starts_with('[') && trimmed.ends_with(']') {
            let section = trimmed
                .trim_start_matches('[')
                .trim_end_matches(']')
                .trim()
                .to_string();
            maybe_current_section = Some(section);
            continue;
        }
        if trimmed.starts_with('-') {
            return Err(ConfigError::new(format!(
                "Error reading configuration file: parse error on line {line_number}: {trimmed}, options in configuration file must be specified without leading -"
            )));
        }

        let Some((raw_name, raw_value)) = trimmed.split_once('=') else {
            let mut message = format!(
                "Error reading configuration file: parse error on line {line_number}: {trimmed}"
            );
            if trimmed.starts_with("no") {
                message.push_str(&format!(
                    ", if you intended to specify a negated option, use {trimmed}=1 instead"
                ));
            }
            return Err(ConfigError::new(message));
        };
        let raw_name = raw_name.trim();
        let raw_value = raw_value.trim().to_string();
        let (maybe_section, key) = interpret_config_key(maybe_current_section.as_deref(), raw_name);
        if used_hash && key == "rpcpassword" {
            return Err(ConfigError::new(format!(
                "Error reading configuration file: parse error on line {line_number}, using # in rpcpassword can be ambiguous and should be avoided"
            )));
        }
        if key == "conf" {
            return Err(ConfigError::new(
                "Error reading configuration file: conf cannot be set in the configuration file; use includeconf= if you want to include additional config files",
            ));
        }
        if !supported_config_key(&key) && !supported_chain_key(&key) {
            let full_key = maybe_section
                .as_ref()
                .map_or_else(|| key.clone(), |section| format!("{section}.{key}"));
            return Err(ConfigError::new(format!(
                "Error reading configuration file: Invalid configuration value {full_key}"
            )));
        }
        if key == "includeconf" {
            include_paths.push(resolve_path(Path::new(&raw_value), base_data_dir));
        }
        entries.push(ConfigEntry {
            maybe_section,
            key,
            value: raw_value,
        });
    }

    for include_path in include_paths {
        parse_config_file(&include_path, entries, visited, base_data_dir)?;
    }

    Ok(())
}

fn strip_comments(line: &str) -> (String, bool) {
    let Some(index) = line.find('#') else {
        return (line.trim().to_string(), false);
    };
    (line[..index].trim().to_string(), true)
}

fn interpret_config_key(
    maybe_current_section: Option<&str>,
    raw_name: &str,
) -> (Option<String>, String) {
    let Some((section, key)) = raw_name.split_once('.') else {
        return (
            maybe_current_section.map(str::to_string),
            raw_name.to_string(),
        );
    };
    (Some(section.to_string()), key.to_string())
}

fn supported_config_key(key: &str) -> bool {
    matches!(
        key,
        "server"
            | "rpcbind"
            | "rpcport"
            | "rpcconnect"
            | "rpcuser"
            | "rpcpassword"
            | "rpccookiefile"
            | "includeconf"
            | "datadir"
    )
}

fn collect_file_settings(
    entries: &[ConfigEntry],
    chain: AddressNetwork,
) -> Result<FileSettings, ConfigError> {
    let mut settings = FileSettings::default();
    let active_section = config_section_name(chain);

    for entry in entries {
        if entry
            .maybe_section
            .as_deref()
            .is_some_and(|section| section != active_section)
        {
            continue;
        }

        match entry.key.as_str() {
            "server" => settings.maybe_server = Some(parse_bool(Some(&entry.value), false)?),
            "rpcbind" => settings.maybe_rpc_bind = Some(entry.value.clone()),
            "rpcport" => settings.maybe_rpc_port = Some(parse_port(&entry.value)?),
            "rpcconnect" => settings.maybe_rpc_connect = Some(entry.value.clone()),
            "rpcuser" => settings.maybe_rpc_user = Some(entry.value.clone()),
            "rpcpassword" => settings.maybe_rpc_password = Some(entry.value.clone()),
            "rpccookiefile" => settings.maybe_cookie_file = Some(PathBuf::from(&entry.value)),
            "datadir" => settings.maybe_data_dir = Some(PathBuf::from(&entry.value)),
            "main" | "test" | "testnet" | "signet" | "regtest" | "includeconf" => {}
            _ => {
                return Err(ConfigError::new(format!(
                    "Error reading configuration file: Invalid configuration value {}",
                    entry.key
                )));
            }
        }
    }

    Ok(settings)
}

fn resolve_auth(
    maybe_username: Option<String>,
    maybe_password: Option<String>,
    maybe_cookie_file: Option<PathBuf>,
    maybe_data_dir: Option<&Path>,
) -> Result<RpcAuthConfig, ConfigError> {
    let password = maybe_password.unwrap_or_default();
    if password.is_empty() {
        let maybe_cookie_file = if let Some(cookie_file) = maybe_cookie_file {
            if cookie_file.as_os_str().is_empty() {
                None
            } else if let Some(data_dir) = maybe_data_dir {
                Some(resolve_path(&cookie_file, data_dir))
            } else {
                Some(cookie_file)
            }
        } else {
            maybe_data_dir.map(|data_dir| data_dir.join(DEFAULT_COOKIE_FILE_NAME))
        };
        return Ok(RpcAuthConfig::cookie(maybe_cookie_file));
    }

    let Some(username) = maybe_username.filter(|value| !value.is_empty()) else {
        return Err(ConfigError::new(
            "Error reading configuration file: rpcuser must be set when rpcpassword is set",
        ));
    };
    Ok(RpcAuthConfig::user_password(username, password))
}

fn resolve_path(path: &Path, base: &Path) -> PathBuf {
    if path.is_absolute() || path.as_os_str().is_empty() {
        path.to_path_buf()
    } else {
        base.join(path)
    }
}

fn parse_socket_address(host: &str, port: u16) -> Result<SocketAddr, ConfigError> {
    if let Ok(address) = host.parse::<SocketAddr>() {
        return Ok(address);
    }
    let ip = host
        .parse::<IpAddr>()
        .map_err(|_| ConfigError::new(format!("invalid rpc address: {host}")))?;
    Ok(SocketAddr::new(ip, port))
}

fn default_data_dir() -> PathBuf {
    #[cfg(target_os = "windows")]
    {
        if let Some(appdata) = env::var_os("APPDATA") {
            return PathBuf::from(appdata).join("Bitcoin");
        }
    }
    #[cfg(target_os = "macos")]
    {
        if let Some(home) = env::var_os("HOME") {
            return PathBuf::from(home)
                .join("Library")
                .join("Application Support")
                .join("Bitcoin");
        }
    }
    if let Some(home) = env::var_os("HOME") {
        return PathBuf::from(home).join(".bitcoin");
    }
    env::current_dir()
        .unwrap_or_else(|_| PathBuf::from("."))
        .join(".bitcoin")
}
