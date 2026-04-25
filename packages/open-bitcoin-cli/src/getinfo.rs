// Parity breadcrumbs:
// - packages/bitcoin-knots/src/bitcoin-cli.cpp
// - packages/bitcoin-knots/src/rpc/client.cpp
// - packages/bitcoin-knots/test/functional/interface_bitcoin_cli.py

use open_bitcoin_rpc::method::{
    GetBalancesResponse, GetBlockchainInfoResponse, GetNetworkInfoResponse, GetWalletInfoResponse,
    RequestParameters, SupportedMethod,
};
use serde::Serialize;

use crate::{
    CliError,
    args::{ColorSetting, GetInfoCommand},
};

/// Thin `-getinfo` batch contract over the supported Phase 8 RPC methods.
#[derive(Debug, Clone, PartialEq)]
pub struct GetInfoBatch {
    pub output_mode: GetInfoOutputMode,
    pub color: ColorSetting,
    pub calls: Vec<GetInfoBatchCall>,
}

/// One batch entry for the `-getinfo` helper.
#[derive(Debug, Clone, PartialEq)]
pub struct GetInfoBatchCall {
    pub method: SupportedMethod,
    pub params: RequestParameters,
}

/// Output mode requested for `-getinfo`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GetInfoOutputMode {
    Human,
    Json,
}

/// Transport-free `-getinfo` snapshot built from real RPC responses.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct GetInfoSnapshot {
    pub network: GetNetworkInfoResponse,
    pub blockchain: GetBlockchainInfoResponse,
    #[serde(rename = "wallet", skip_serializing_if = "Option::is_none")]
    pub maybe_wallet: Option<GetWalletInfoResponse>,
    #[serde(rename = "balances", skip_serializing_if = "Option::is_none")]
    pub maybe_balances: Option<GetBalancesResponse>,
}

/// Build the strict four-call `-getinfo` batch for the supported Phase 8 slice.
pub fn build_getinfo_batch(command: &GetInfoCommand) -> Result<GetInfoBatch, CliError> {
    let output_mode = match command.helper_args.as_slice() {
        [] => GetInfoOutputMode::Human,
        [flag] if flag == "--json" => GetInfoOutputMode::Json,
        _ => {
            return Err(CliError::new("-getinfo takes no arguments except --json"));
        }
    };

    Ok(GetInfoBatch {
        output_mode,
        color: command.color,
        calls: vec![
            GetInfoBatchCall {
                method: SupportedMethod::GetNetworkInfo,
                params: RequestParameters::None,
            },
            GetInfoBatchCall {
                method: SupportedMethod::GetBlockchainInfo,
                params: RequestParameters::None,
            },
            GetInfoBatchCall {
                method: SupportedMethod::GetWalletInfo,
                params: RequestParameters::None,
            },
            GetInfoBatchCall {
                method: SupportedMethod::GetBalances,
                params: RequestParameters::None,
            },
        ],
    })
}

/// Render `-getinfo` output for either automation or human operators.
pub fn render_getinfo(
    snapshot: &GetInfoSnapshot,
    output_mode: GetInfoOutputMode,
    color: ColorSetting,
) -> Result<String, CliError> {
    match output_mode {
        GetInfoOutputMode::Json => {
            serde_json::to_string_pretty(snapshot).map_err(|error| CliError::new(error.to_string()))
        }
        GetInfoOutputMode::Human => Ok(render_human_getinfo(snapshot, color)),
    }
}

fn render_human_getinfo(snapshot: &GetInfoSnapshot, color: ColorSetting) -> String {
    let styles = ColorStyles::for_setting(color);
    let mut lines = vec![
        format!(
            "{}Chain:{} {}",
            styles.header, styles.reset, snapshot.blockchain.chain
        ),
        format!("Blocks: {}", snapshot.blockchain.blocks),
        format!("Headers: {}", snapshot.blockchain.headers),
        format!(
            "Verification progress: {:.4}%",
            snapshot.blockchain.verificationprogress * 100.0
        ),
        String::new(),
        format!(
            "{}Network:{} in {}, out {}, total {}",
            styles.network,
            styles.reset,
            snapshot.network.connections_in,
            snapshot.network.connections_out,
            snapshot.network.connections
        ),
        format!("Version: {}", snapshot.network.version),
        format!("Relay fee (sats/kvB): {}", snapshot.network.relayfee),
    ];

    if let Some(wallet) = snapshot.maybe_wallet.as_ref() {
        lines.push(String::new());
        lines.push(format!(
            "{}Wallet network:{} {}",
            styles.wallet, styles.reset, wallet.network
        ));
        lines.push(format!("Descriptor count: {}", wallet.descriptor_count));
        lines.push(format!("Tracked UTXOs: {}", wallet.utxo_count));
        if let Some(tip_height) = wallet.maybe_tip_height {
            lines.push(format!("Wallet tip height: {tip_height}"));
        }
        if let Some(median_time_past) = wallet.maybe_tip_median_time_past {
            lines.push(format!("Wallet tip median time past: {median_time_past}"));
        }
    }

    if let Some(balances) = snapshot.maybe_balances.as_ref() {
        lines.push(String::new());
        lines.push(format!(
            "{}Trusted balance (sats):{} {}",
            styles.balance, styles.reset, balances.mine.trusted_sats
        ));
    }

    lines.push(String::new());
    lines.push(format!(
        "{}Warnings:{} {}",
        styles.warning,
        styles.reset,
        render_warnings(&snapshot.network.warnings)
    ));

    lines.join("\n")
}

fn render_warnings(warnings: &[String]) -> String {
    if warnings.is_empty() {
        return "(none)".to_string();
    }

    warnings.join("; ")
}

#[derive(Debug, Clone, Copy)]
struct ColorStyles {
    header: &'static str,
    network: &'static str,
    wallet: &'static str,
    balance: &'static str,
    warning: &'static str,
    reset: &'static str,
}

impl ColorStyles {
    fn for_setting(setting: ColorSetting) -> Self {
        match setting {
            ColorSetting::Always => Self {
                header: "\u{001b}[34m",
                network: "\u{001b}[32m",
                wallet: "\u{001b}[35m",
                balance: "\u{001b}[36m",
                warning: "\u{001b}[33m",
                reset: "\u{001b}[0m",
            },
            ColorSetting::Auto | ColorSetting::Never => Self {
                header: "",
                network: "",
                wallet: "",
                balance: "",
                warning: "",
                reset: "",
            },
        }
    }
}

#[cfg(test)]
mod tests;
