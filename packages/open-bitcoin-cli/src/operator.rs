// Parity breadcrumbs:
// - packages/bitcoin-knots/src/bitcoin-cli.cpp
// - packages/bitcoin-knots/src/common/args.cpp
// - packages/bitcoin-knots/test/functional/interface_bitcoin_cli.py

//! Clap contracts for the Open Bitcoin operator command path.

use std::{ffi::OsString, path::PathBuf};

use clap::{Args, Parser, Subcommand, ValueEnum};

use crate::CliError;

pub mod config;
pub mod dashboard;
pub mod detect;
pub mod onboarding;
pub mod runtime;
pub mod service;
pub mod status;
pub mod wallet;

/// First-party Open Bitcoin operator CLI contract.
#[derive(Debug, Clone, PartialEq, Eq, Parser)]
#[command(name = "open-bitcoin")]
#[command(about = "Manage an Open Bitcoin node")]
pub struct OperatorCli {
    #[arg(long = "config", global = true)]
    pub maybe_config_path: Option<PathBuf>,
    #[arg(long = "datadir", global = true)]
    pub maybe_data_dir: Option<PathBuf>,
    #[arg(long = "network", global = true, value_enum)]
    pub maybe_network: Option<NetworkSelection>,
    #[arg(long = "format", global = true, value_enum, default_value = "human")]
    pub format: OperatorOutputFormat,
    #[arg(long = "no-color", global = true)]
    pub no_color: bool,
    #[command(subcommand)]
    pub command: OperatorCommand,
}

/// Operator-owned subcommands. Phase 13 defines shape only.
#[derive(Debug, Clone, PartialEq, Eq, Subcommand)]
pub enum OperatorCommand {
    Status(StatusArgs),
    Config(ConfigArgs),
    Service(ServiceArgs),
    Dashboard(DashboardArgs),
    Onboard(OnboardArgs),
    Wallet(WalletArgs),
}

#[derive(Debug, Clone, PartialEq, Eq, Args)]
pub struct StatusArgs {
    #[arg(long = "watch")]
    pub watch: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Args)]
pub struct ConfigArgs {
    #[command(subcommand)]
    pub command: ConfigCommand,
}

#[derive(Debug, Clone, PartialEq, Eq, Subcommand)]
pub enum ConfigCommand {
    Paths,
}

#[derive(Debug, Clone, PartialEq, Eq, Args)]
pub struct ServiceArgs {
    #[command(subcommand)]
    pub command: ServiceCommand,
    /// Apply changes (default: dry-run only). Pass --apply to write files or invoke
    /// service manager commands. Without this flag, install and uninstall show a
    /// preview of what would happen with no side effects.
    #[arg(long = "apply", global = true)]
    pub apply: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Subcommand)]
pub enum ServiceCommand {
    Status,
    Install,
    Uninstall,
    Enable,
    Disable,
}

#[derive(Debug, Clone, PartialEq, Eq, Args)]
pub struct DashboardArgs {
    #[arg(long = "tick-ms", default_value_t = 1_000)]
    pub tick_ms: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Args)]
pub struct OnboardArgs {
    #[arg(long = "non-interactive")]
    pub non_interactive: bool,
    #[arg(long = "approve-write")]
    pub approve_write: bool,
    #[arg(long = "force-overwrite")]
    pub force_overwrite: bool,
    #[arg(long = "disable-metrics")]
    pub disable_metrics: bool,
    #[arg(long = "disable-logs")]
    pub disable_logs: bool,
    #[arg(long = "detect-existing")]
    pub detect_existing: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Args)]
pub struct WalletArgs {
    #[arg(long = "wallet")]
    pub maybe_wallet_name: Option<String>,
    #[command(subcommand)]
    pub command: WalletCommand,
}

#[derive(Debug, Clone, PartialEq, Eq, Subcommand)]
pub enum WalletCommand {
    Send(WalletSendArgs),
    Backup(WalletBackupArgs),
}

#[derive(Debug, Clone, PartialEq, Eq, Args)]
pub struct WalletSendArgs {
    pub address: String,
    pub amount_sats: i64,
    #[arg(long = "fee-rate-sat-per-kvb")]
    pub maybe_fee_rate_sat_per_kvb: Option<i64>,
    #[arg(long = "conf-target")]
    pub maybe_conf_target: Option<u16>,
    #[arg(long = "estimate-mode", value_enum)]
    pub maybe_estimate_mode: Option<WalletEstimateMode>,
    #[arg(long = "change-descriptor-id")]
    pub maybe_change_descriptor_id: Option<u32>,
    #[arg(long = "lock-time")]
    pub maybe_lock_time: Option<u32>,
    #[arg(long = "replaceable")]
    pub enable_rbf: bool,
    #[arg(long = "max-tx-fee-sats")]
    pub maybe_max_tx_fee_sats: Option<i64>,
    #[arg(long = "confirm")]
    pub confirm: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Args)]
pub struct WalletBackupArgs {
    pub destination: PathBuf,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum WalletEstimateMode {
    Unset,
    Economical,
    Conservative,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, ValueEnum)]
pub enum OperatorOutputFormat {
    #[default]
    Human,
    Json,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum NetworkSelection {
    Mainnet,
    Testnet,
    Signet,
    Regtest,
}

/// Top-level route selected before command execution.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CliRoute {
    Operator(OperatorCli),
    BitcoinCliCompat(Vec<OsString>),
}

/// Route a shell invocation to the operator parser or compatibility parser.
pub fn route_cli_invocation(binary_name: &str, args: &[OsString]) -> Result<CliRoute, CliError> {
    if binary_name.ends_with("open-bitcoin-cli") {
        return Ok(CliRoute::BitcoinCliCompat(args.to_vec()));
    }

    let mut argv = Vec::with_capacity(args.len() + 1);
    argv.push(OsString::from(binary_name));
    argv.extend(args.iter().cloned());
    let parsed =
        OperatorCli::try_parse_from(argv).map_err(|error| CliError::new(error.to_string()))?;
    Ok(CliRoute::Operator(parsed))
}

pub(crate) mod wallet_support {
    use open_bitcoin_node::core::{primitives::ScriptBuf, wallet::AddressNetwork};
    use open_bitcoin_rpc::{
        JsonRpcId, JsonRpcVersion, RpcErrorDetail, RpcRequestEnvelope,
        method::{
            BuildAndSignTransactionRequest, BuildAndSignTransactionResponse, GetBalancesResponse,
            SendToAddressRequest,
        },
    };
    use serde_json::{Value, json};
    use ureq::Agent;

    use crate::startup::CliRpcConfig;

    pub(crate) trait WalletOperatorRpcClient {
        fn build_and_sign_transaction(
            &self,
            maybe_wallet_name: Option<&str>,
            request: BuildAndSignTransactionRequest,
        ) -> Result<BuildAndSignTransactionResponse, WalletOperatorError>;

        fn send_to_address(
            &self,
            maybe_wallet_name: Option<&str>,
            request: SendToAddressRequest,
        ) -> Result<String, WalletOperatorError>;

        fn get_wallet_info(
            &self,
            maybe_wallet_name: Option<&str>,
        ) -> Result<Value, WalletOperatorError>;

        fn get_balances(
            &self,
            maybe_wallet_name: Option<&str>,
        ) -> Result<GetBalancesResponse, WalletOperatorError>;
    }

    pub(crate) struct HttpWalletOperatorRpcClient {
        agent: Agent,
        root_endpoint_url: String,
        root_endpoint_display: String,
        authorization_header: String,
    }

    impl HttpWalletOperatorRpcClient {
        pub(crate) fn from_config(
            config: &CliRpcConfig,
        ) -> Result<Self, super::runtime::OperatorRuntimeError> {
            Ok(Self {
                agent: Agent::new_with_config(
                    Agent::config_builder().http_status_as_error(false).build(),
                ),
                root_endpoint_url: format!(
                    "http://{}/",
                    super::runtime::format_host_for_url(&config.host, config.port)
                ),
                root_endpoint_display: super::runtime::format_host_for_url(
                    &config.host,
                    config.port,
                ),
                authorization_header: super::runtime::authorization_header(&config.auth)?,
            })
        }

        fn wallet_endpoint_url(&self, maybe_wallet_name: Option<&str>) -> String {
            maybe_wallet_name.map_or_else(
                || self.root_endpoint_url.clone(),
                |wallet_name| {
                    format!(
                        "http://{}/wallet/{}",
                        self.root_endpoint_display,
                        percent_encode_path_segment(wallet_name)
                    )
                },
            )
        }

        fn post_json(
            &self,
            maybe_wallet_name: Option<&str>,
            method: &str,
            params: Value,
        ) -> Result<Value, WalletOperatorError> {
            let endpoint = self.wallet_endpoint_url(maybe_wallet_name);
            let response = self
                .agent
                .post(&endpoint)
                .header("Authorization", &self.authorization_header)
                .send_json(RpcRequestEnvelope {
                    jsonrpc: Some(JsonRpcVersion::V2),
                    method: method.to_string(),
                    params,
                    id: Some(JsonRpcId::Number(1)),
                })
                .map_err(|error| WalletOperatorError::new(error.to_string()))?;
            let status = response.status().as_u16();
            if status == 401 {
                return Err(WalletOperatorError::new(
                    "RPC authentication failed for operator wallet command",
                ));
            }
            if status != 200 {
                return Err(WalletOperatorError::new(format!(
                    "RPC endpoint {} returned HTTP status {}",
                    endpoint, status
                )));
            }
            let value: Value = response
                .into_body()
                .read_json()
                .map_err(|error| WalletOperatorError::new(error.to_string()))?;
            extract_result(value)
        }
    }

    impl WalletOperatorRpcClient for HttpWalletOperatorRpcClient {
        fn build_and_sign_transaction(
            &self,
            maybe_wallet_name: Option<&str>,
            request: BuildAndSignTransactionRequest,
        ) -> Result<BuildAndSignTransactionResponse, WalletOperatorError> {
            serde_json::from_value(self.post_json(
                maybe_wallet_name,
                "buildandsigntransaction",
                serde_json::to_value(request)?,
            )?)
            .map_err(WalletOperatorError::from)
        }

        fn send_to_address(
            &self,
            maybe_wallet_name: Option<&str>,
            request: SendToAddressRequest,
        ) -> Result<String, WalletOperatorError> {
            serde_json::from_value(self.post_json(
                maybe_wallet_name,
                "sendtoaddress",
                serde_json::to_value(request)?,
            )?)
            .map_err(WalletOperatorError::from)
        }

        fn get_wallet_info(
            &self,
            maybe_wallet_name: Option<&str>,
        ) -> Result<Value, WalletOperatorError> {
            self.post_json(maybe_wallet_name, "getwalletinfo", json!({}))
        }

        fn get_balances(
            &self,
            maybe_wallet_name: Option<&str>,
        ) -> Result<GetBalancesResponse, WalletOperatorError> {
            serde_json::from_value(self.post_json(maybe_wallet_name, "getbalances", json!({}))?)
                .map_err(WalletOperatorError::from)
        }
    }

    fn extract_result(response: Value) -> Result<Value, WalletOperatorError> {
        let Value::Object(object) = response else {
            return Err(WalletOperatorError::new("RPC response must be an object"));
        };
        if let Some(error) = object.get("error") {
            if error.is_null() {
                return Ok(object.get("result").cloned().unwrap_or(Value::Null));
            }
            let detail: RpcErrorDetail = serde_json::from_value(error.clone())?;
            return Err(WalletOperatorError::new(detail.message));
        }
        object
            .get("result")
            .cloned()
            .ok_or_else(|| WalletOperatorError::new("RPC response is missing result"))
    }

    fn percent_encode_path_segment(value: &str) -> String {
        let mut encoded = String::with_capacity(value.len());
        for byte in value.bytes() {
            if byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.' | b'~') {
                encoded.push(char::from(byte));
            } else {
                encoded.push('%');
                encoded.push(nibble_to_hex(byte >> 4));
                encoded.push(nibble_to_hex(byte & 0x0f));
            }
        }
        encoded
    }

    pub(crate) fn script_pubkey_from_address(
        network: AddressNetwork,
        address: &str,
    ) -> Result<ScriptBuf, WalletOperatorError> {
        if address.starts_with(network.hrp()) {
            return decode_segwit_script(network, address);
        }
        decode_base58_script(network, address)
    }

    fn decode_base58_script(
        network: AddressNetwork,
        address: &str,
    ) -> Result<ScriptBuf, WalletOperatorError> {
        let decoded = base58_decode(address)?;
        if decoded.len() < 5 {
            return Err(WalletOperatorError::new("invalid destination address"));
        }
        let payload_len = decoded.len().saturating_sub(4);
        let (payload, checksum) = decoded.split_at(payload_len);
        let expected_checksum = open_bitcoin_node::core::consensus::crypto::double_sha256(payload);
        if checksum != &expected_checksum[..4] {
            return Err(WalletOperatorError::new("invalid destination address"));
        }
        let Some((prefix, body)) = payload.split_first() else {
            return Err(WalletOperatorError::new("invalid destination address"));
        };
        let expected_prefix = match network {
            AddressNetwork::Mainnet => [0x00_u8, 0x05_u8],
            AddressNetwork::Testnet | AddressNetwork::Signet | AddressNetwork::Regtest => {
                [0x6f_u8, 0xc4_u8]
            }
        };
        if *prefix == expected_prefix[0] && body.len() == 20 {
            let mut script = vec![0x76, 0xa9, 0x14];
            script.extend_from_slice(body);
            script.extend_from_slice(&[0x88, 0xac]);
            return ScriptBuf::from_bytes(script).map_err(WalletOperatorError::from);
        }
        if *prefix == expected_prefix[1] && body.len() == 20 {
            let mut script = vec![0xa9, 0x14];
            script.extend_from_slice(body);
            script.push(0x87);
            return ScriptBuf::from_bytes(script).map_err(WalletOperatorError::from);
        }
        Err(WalletOperatorError::new("invalid destination address"))
    }

    fn decode_segwit_script(
        network: AddressNetwork,
        address: &str,
    ) -> Result<ScriptBuf, WalletOperatorError> {
        let lower = address.to_ascii_lowercase();
        if lower != address && address.to_ascii_uppercase() != address {
            return Err(WalletOperatorError::new("invalid destination address"));
        }
        let Some(separator_index) = lower.rfind('1') else {
            return Err(WalletOperatorError::new("invalid destination address"));
        };
        let hrp = &lower[..separator_index];
        let data_part = &lower[separator_index + 1..];
        if hrp != network.hrp() || data_part.len() < 7 {
            return Err(WalletOperatorError::new("invalid destination address"));
        }
        let data = data_part
            .bytes()
            .map(bech32_value)
            .collect::<Result<Vec<_>, WalletOperatorError>>()?;
        if bech32_polymod(&[expand_hrp(hrp), data.clone()].concat()) != 1 {
            return Err(WalletOperatorError::new("invalid destination address"));
        }
        let (witness_version, program_with_checksum) = data
            .split_first()
            .ok_or_else(|| WalletOperatorError::new("invalid destination address"))?;
        let program = convert_bits(
            &program_with_checksum[..program_with_checksum.len().saturating_sub(6)],
            5,
            8,
            false,
        )?;
        if program.len() < 2 || program.len() > 40 {
            return Err(WalletOperatorError::new("invalid destination address"));
        }
        let opcode = if *witness_version == 0 {
            0x00
        } else {
            0x50 + *witness_version
        };
        let mut script = Vec::with_capacity(program.len() + 2);
        script.push(opcode);
        script.push(program.len() as u8);
        script.extend_from_slice(&program);
        ScriptBuf::from_bytes(script).map_err(WalletOperatorError::from)
    }

    fn base58_decode(input: &str) -> Result<Vec<u8>, WalletOperatorError> {
        const ALPHABET: &str = "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";
        let mut bytes = vec![0_u8];
        for character in input.bytes() {
            let value = ALPHABET
                .bytes()
                .position(|candidate| candidate == character)
                .ok_or_else(|| WalletOperatorError::new("invalid destination address"))?
                as u32;
            let mut carry = value;
            for byte in bytes.iter_mut().rev() {
                let total = (*byte as u32) * 58 + carry;
                *byte = (total & 0xff) as u8;
                carry = total >> 8;
            }
            while carry > 0 {
                bytes.insert(0, (carry & 0xff) as u8);
                carry >>= 8;
            }
        }
        for _ in input.bytes().take_while(|character| *character == b'1') {
            bytes.insert(0, 0);
        }
        Ok(bytes)
    }

    fn bech32_value(byte: u8) -> Result<u8, WalletOperatorError> {
        const CHARSET: &[u8; 32] = b"qpzry9x8gf2tvdw0s3jn54khce6mua7l";
        CHARSET
            .iter()
            .position(|candidate| *candidate == byte)
            .map(|value| value as u8)
            .ok_or_else(|| WalletOperatorError::new("invalid destination address"))
    }

    fn expand_hrp(hrp: &str) -> Vec<u8> {
        let mut expanded = Vec::with_capacity(hrp.len() * 2 + 1);
        expanded.extend(hrp.bytes().map(|byte| byte >> 5));
        expanded.push(0);
        expanded.extend(hrp.bytes().map(|byte| byte & 0x1f));
        expanded
    }

    fn bech32_polymod(values: &[u8]) -> u32 {
        let mut checksum = 1_u32;
        for value in values {
            let top = checksum >> 25;
            checksum = ((checksum & 0x01ff_ffff) << 5) ^ u32::from(*value);
            for (bit, generator) in [
                0x3b6a_57b2_u32,
                0x2650_8e6d_u32,
                0x1ea1_19fa_u32,
                0x3d42_33dd_u32,
                0x2a14_62b3_u32,
            ]
            .into_iter()
            .enumerate()
            {
                if ((top >> bit) & 1) == 1 {
                    checksum ^= generator;
                }
            }
        }
        checksum
    }

    fn convert_bits(
        data: &[u8],
        from_bits: u8,
        to_bits: u8,
        pad: bool,
    ) -> Result<Vec<u8>, WalletOperatorError> {
        let mut acc = 0_u32;
        let mut bits = 0_u8;
        let mut output = Vec::new();
        let maxv = (1_u32 << to_bits) - 1;
        for value in data {
            if (u32::from(*value) >> from_bits) != 0 {
                return Err(WalletOperatorError::new("invalid destination address"));
            }
            acc = (acc << from_bits) | u32::from(*value);
            bits += from_bits;
            while bits >= to_bits {
                bits -= to_bits;
                output.push(((acc >> bits) & maxv) as u8);
            }
        }
        if pad {
            if bits > 0 {
                output.push(((acc << (to_bits - bits)) & maxv) as u8);
            }
        } else if bits >= from_bits || ((acc << (to_bits - bits)) & maxv) != 0 {
            return Err(WalletOperatorError::new("invalid destination address"));
        }
        Ok(output)
    }

    const fn nibble_to_hex(value: u8) -> char {
        match value {
            0..=9 => (b'0' + value) as char,
            10..=15 => (b'a' + (value - 10)) as char,
            _ => '?',
        }
    }

    #[derive(Debug)]
    pub(crate) struct WalletOperatorError {
        message: String,
    }

    impl WalletOperatorError {
        pub(crate) fn new(message: impl Into<String>) -> Self {
            Self {
                message: message.into(),
            }
        }
    }

    impl std::fmt::Display for WalletOperatorError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(&self.message)
        }
    }

    impl std::error::Error for WalletOperatorError {}

    impl From<serde_json::Error> for WalletOperatorError {
        fn from(value: serde_json::Error) -> Self {
            Self::new(value.to_string())
        }
    }

    impl From<open_bitcoin_node::core::wallet::WalletError> for WalletOperatorError {
        fn from(value: open_bitcoin_node::core::wallet::WalletError) -> Self {
            Self::new(value.to_string())
        }
    }

    impl From<open_bitcoin_node::core::primitives::ScriptError> for WalletOperatorError {
        fn from(value: open_bitcoin_node::core::primitives::ScriptError) -> Self {
            Self::new(value.to_string())
        }
    }
}

#[cfg(test)]
mod tests;
