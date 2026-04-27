// Parity breadcrumbs:
// - packages/bitcoin-knots/src/wallet/rpc/spend.cpp
// - packages/bitcoin-knots/src/wallet/rpc/backup.cpp
// - packages/bitcoin-knots/src/wallet/rpc/util.cpp
// - packages/bitcoin-knots/test/functional/wallet_backup.py

//! Operator-owned wallet workflows layered on top of the wallet-scoped RPC path.

use std::{
    fs,
    path::{Path, PathBuf},
};

use open_bitcoin_node::{
    FjallNodeStore, WalletRegistry,
    core::{
        mempool::FeeRate,
        wallet::wallet::{
            ChangePolicy, FeeEstimateMode, FeeEstimateRequest, FeeSelection, SendIntent,
        },
        wallet::{AddressNetwork, Recipient},
    },
};
use open_bitcoin_rpc::method::{
    BuildAndSignTransactionRequest, BuildAndSignTransactionResponse, GetBalancesResponse,
    SendToAddressRequest, TransactionRecipient,
};
use serde_json::{Value, json};

use crate::{args::CliStartupArgs, startup::resolve_startup_config};

use super::{
    NetworkSelection, OperatorCli, OperatorOutputFormat, WalletArgs, WalletBackupArgs,
    WalletCommand, WalletEstimateMode, WalletSendArgs,
    config::OperatorConfigResolution,
    detect::DetectedInstallation,
    runtime::{OperatorCommandOutcome, OperatorExitCode, OperatorRuntimeError},
    wallet_support::{
        HttpWalletOperatorRpcClient, WalletOperatorError, WalletOperatorRpcClient,
        script_pubkey_from_address,
    },
};

const BACKUP_EXPORT_FORMAT: &str = "open-bitcoin-wallet-backup";
const BACKUP_EXPORT_VERSION: u32 = 1;
const CONFIRMATION_REQUIRED_MESSAGE: &str =
    "confirmation required: rerun with --confirm to submit this transaction";
const MULTI_WALLET_SELECTION_MESSAGE: &str =
    "Multiple wallets are loaded. Please pass --wallet <name> to select the managed wallet.";
const NO_MANAGED_WALLET_MESSAGE: &str = "No managed wallet is available in the configured datadir.";

pub(crate) fn execute_wallet_command(
    args: &WalletArgs,
    cli: &OperatorCli,
    config_resolution: &OperatorConfigResolution,
    detections: &[DetectedInstallation],
    default_data_dir: &Path,
) -> Result<OperatorCommandOutcome, OperatorRuntimeError> {
    let maybe_data_dir = config_resolution.maybe_data_dir.as_deref();
    let network = config_resolution
        .maybe_network
        .map(map_network_selection)
        .unwrap_or(AddressNetwork::Mainnet);
    let maybe_wallet_name =
        resolve_managed_wallet_name(maybe_data_dir, args.maybe_wallet_name.as_deref())
            .map_err(to_invalid_request)?;

    match &args.command {
        WalletCommand::Send(send) => {
            let startup = wallet_startup_config(config_resolution, default_data_dir)?;
            execute_send_command(
                cli.format,
                maybe_wallet_name.as_deref(),
                network,
                send,
                &HttpWalletOperatorRpcClient::from_config(&startup.rpc)?,
            )
        }
        WalletCommand::Backup(backup) => execute_backup_command(
            cli.format,
            maybe_wallet_name.as_deref(),
            backup,
            maybe_data_dir,
            detections,
        ),
    }
}

fn wallet_startup_config(
    config_resolution: &OperatorConfigResolution,
    default_data_dir: &Path,
) -> Result<crate::startup::CliStartupConfig, OperatorRuntimeError> {
    let startup = CliStartupArgs {
        maybe_conf_path: config_resolution.maybe_bitcoin_conf_path.clone(),
        maybe_data_dir: config_resolution.maybe_data_dir.clone(),
        ..CliStartupArgs::default()
    };
    resolve_startup_config(&startup, default_data_dir).map_err(|error| {
        OperatorRuntimeError::InvalidRequest {
            message: error.to_string(),
        }
    })
}

fn execute_send_command(
    format: OperatorOutputFormat,
    maybe_wallet_name: Option<&str>,
    network: AddressNetwork,
    args: &WalletSendArgs,
    client: &dyn WalletOperatorRpcClient,
) -> Result<OperatorCommandOutcome, OperatorRuntimeError> {
    let intent = build_send_intent(network, args)?;
    let preview_request =
        build_preview_request(network, &intent, &args.address).map_err(to_invalid_request)?;
    let preview_response = client
        .build_and_sign_transaction(maybe_wallet_name, preview_request)
        .map_err(to_invalid_request)?;
    let wallet_info = client
        .get_wallet_info(maybe_wallet_name)
        .map_err(to_invalid_request)?;
    let balances = client
        .get_balances(maybe_wallet_name)
        .map_err(to_invalid_request)?;
    let preview_rendered = render_send_preview(
        format,
        maybe_wallet_name,
        args,
        &intent,
        &preview_response,
        &wallet_info,
        &balances,
    )
    .map_err(to_invalid_request)?;

    if !args.confirm {
        return Ok(OperatorCommandOutcome::new(
            preview_rendered,
            format!("{CONFIRMATION_REQUIRED_MESSAGE}\n"),
            OperatorExitCode::Failure(1),
        ));
    }

    let txid = client
        .send_to_address(maybe_wallet_name, build_send_commit_request(args))
        .map_err(to_invalid_request)?;
    let rendered = render_send_submission(format, maybe_wallet_name, &txid)?;
    Ok(OperatorCommandOutcome::success(rendered))
}

fn execute_backup_command(
    format: OperatorOutputFormat,
    maybe_wallet_name: Option<&str>,
    args: &WalletBackupArgs,
    maybe_data_dir: Option<&Path>,
    detections: &[DetectedInstallation],
) -> Result<OperatorCommandOutcome, OperatorRuntimeError> {
    let wallet_name = maybe_wallet_name.ok_or_else(|| OperatorRuntimeError::InvalidRequest {
        message: NO_MANAGED_WALLET_MESSAGE.to_string(),
    })?;
    let data_dir = maybe_data_dir.ok_or_else(|| OperatorRuntimeError::InvalidRequest {
        message: "wallet backup requires a configured datadir".to_string(),
    })?;
    ensure_safe_backup_destination(&args.destination, detections).map_err(to_invalid_request)?;
    if args.destination.exists() {
        return Err(OperatorRuntimeError::InvalidRequest {
            message: format!(
                "backup destination already exists: {}",
                args.destination.display()
            ),
        });
    }
    let Some(parent) = args.destination.parent() else {
        return Err(OperatorRuntimeError::InvalidRequest {
            message: format!(
                "backup destination must include a parent directory: {}",
                args.destination.display()
            ),
        });
    };
    if !parent.is_dir() {
        return Err(OperatorRuntimeError::InvalidRequest {
            message: format!(
                "backup destination parent directory does not exist: {}",
                parent.display()
            ),
        });
    }

    let store =
        FjallNodeStore::open(data_dir).map_err(|error| OperatorRuntimeError::InvalidRequest {
            message: error.to_string(),
        })?;
    let registry =
        WalletRegistry::load(&store).map_err(|error| OperatorRuntimeError::InvalidRequest {
            message: error.to_string(),
        })?;
    let snapshot = registry.wallet_snapshot(wallet_name).map_err(|error| {
        OperatorRuntimeError::InvalidRequest {
            message: error.to_string(),
        }
    })?;
    let maybe_rescan_job = registry.rescan_job(wallet_name);
    let export = json!({
        "format": BACKUP_EXPORT_FORMAT,
        "version": BACKUP_EXPORT_VERSION,
        "wallet_name": wallet_name,
        "snapshot": {
            "network": snapshot.network.to_string(),
            "descriptor_count": snapshot.descriptors.len(),
            "utxo_count": snapshot.utxos.len(),
            "maybe_tip_height": snapshot.maybe_tip_height,
            "maybe_tip_median_time_past": snapshot.maybe_tip_median_time_past,
            "descriptors": snapshot.descriptors.iter().map(|descriptor| {
                json!({
                    "id": descriptor.id,
                    "label": descriptor.label,
                    "role": format!("{:?}", descriptor.role).to_lowercase(),
                    "original_text": descriptor.original_text,
                    "display_text": descriptor.descriptor.display_text(),
                    "is_ranged": descriptor.descriptor.is_ranged(),
                    "maybe_range_start": descriptor.descriptor.range_start(),
                    "maybe_range_end": descriptor.descriptor.range_end(),
                    "maybe_next_index": descriptor.descriptor.next_index(),
                })
            }).collect::<Vec<_>>(),
            "utxos": snapshot.utxos.iter().map(|utxo| {
                json!({
                    "txid_hex": encode_hex(utxo.outpoint.txid.as_bytes()),
                    "vout": utxo.outpoint.vout,
                    "descriptor_id": utxo.descriptor_id,
                    "amount_sats": utxo.output.value.to_sats(),
                    "script_pubkey_hex": encode_hex(utxo.output.script_pubkey.as_bytes()),
                    "created_height": utxo.created_height,
                    "created_median_time_past": utxo.created_median_time_past,
                    "is_coinbase": utxo.is_coinbase,
                })
            }).collect::<Vec<_>>(),
        },
        "maybe_rescan_job": maybe_rescan_job.map(|job| {
            json!({
                "target_tip_hash_hex": encode_hex(job.target_tip_hash.as_bytes()),
                "target_tip_height": job.target_tip_height,
                "next_height": job.next_height,
                "maybe_scanned_through_height": job.maybe_scanned_through_height,
                "maybe_tip_median_time_past": job.maybe_tip_median_time_past,
                "freshness": format!("{:?}", job.freshness).to_lowercase(),
                "state": format!("{:?}", job.state).to_lowercase(),
                "maybe_error": job.maybe_error,
            })
        }),
    });
    let encoded = serde_json::to_string_pretty(&export).map_err(to_invalid_request)?;
    fs::write(&args.destination, format!("{encoded}\n")).map_err(|error| {
        OperatorRuntimeError::InvalidRequest {
            message: error.to_string(),
        }
    })?;

    let rendered = render_backup_success(format, wallet_name, &args.destination)?;
    Ok(OperatorCommandOutcome::success(rendered))
}

fn build_send_intent(
    network: AddressNetwork,
    args: &WalletSendArgs,
) -> Result<SendIntent, OperatorRuntimeError> {
    let fee_selection = if let Some(fee_rate_sat_per_kvb) = args.maybe_fee_rate_sat_per_kvb {
        FeeSelection::Explicit(FeeRate::from_sats_per_kvb(fee_rate_sat_per_kvb))
    } else {
        FeeSelection::Estimate(FeeEstimateRequest {
            conf_target: args.maybe_conf_target.unwrap_or(6),
            mode: map_estimate_mode(
                args.maybe_estimate_mode
                    .unwrap_or(WalletEstimateMode::Unset),
            ),
        })
    };
    let recipient = Recipient {
        script_pubkey: script_pubkey_from_address(network, &args.address)
            .map_err(to_invalid_request)?,
        value: open_bitcoin_node::core::primitives::Amount::from_sats(args.amount_sats).map_err(
            |error: open_bitcoin_node::core::primitives::AmountError| {
                OperatorRuntimeError::InvalidRequest {
                    message: error.to_string(),
                }
            },
        )?,
    };
    let change_policy = args
        .maybe_change_descriptor_id
        .map_or(ChangePolicy::Automatic, ChangePolicy::FixedDescriptor);

    SendIntent::new(
        vec![recipient],
        fee_selection,
        change_policy,
        args.maybe_lock_time,
        args.enable_rbf,
        args.maybe_max_tx_fee_sats,
    )
    .map_err(|error| OperatorRuntimeError::InvalidRequest {
        message: error.to_string(),
    })
}

fn build_preview_request(
    network: AddressNetwork,
    intent: &SendIntent,
    address: &str,
) -> Result<BuildAndSignTransactionRequest, WalletOperatorError> {
    let maybe_resolved_estimate = match intent.fee_selection {
        FeeSelection::Explicit(_) => None,
        FeeSelection::Estimate(request) => {
            Some(resolve_fee_estimate(request.conf_target, request.mode))
        }
    };
    let build_request = intent.into_build_request(maybe_resolved_estimate)?;
    Ok(BuildAndSignTransactionRequest {
        recipients: vec![TransactionRecipient {
            script_pubkey_hex: encode_hex(script_pubkey_from_address(network, address)?.as_bytes()),
            amount_sats: build_request.recipients[0].value.to_sats(),
        }],
        fee_rate_sat_per_kvb: build_request.fee_rate.sats_per_kvb(),
        maybe_change_descriptor_id: build_request.maybe_change_descriptor_id,
        maybe_lock_time: build_request.maybe_lock_time,
        enable_rbf: build_request.enable_rbf,
    })
}

fn build_send_commit_request(args: &WalletSendArgs) -> SendToAddressRequest {
    SendToAddressRequest {
        address: args.address.clone(),
        amount_sats: args.amount_sats,
        maybe_fee_rate_sat_per_kvb: args.maybe_fee_rate_sat_per_kvb,
        maybe_conf_target: args.maybe_conf_target,
        maybe_estimate_mode: args.maybe_estimate_mode.map(map_rpc_estimate_mode),
        maybe_change_descriptor_id: args.maybe_change_descriptor_id,
        maybe_lock_time: args.maybe_lock_time,
        enable_rbf: args.enable_rbf,
        maybe_max_tx_fee_sats: args.maybe_max_tx_fee_sats,
    }
}

fn render_send_preview(
    format: OperatorOutputFormat,
    maybe_wallet_name: Option<&str>,
    args: &WalletSendArgs,
    intent: &SendIntent,
    preview: &BuildAndSignTransactionResponse,
    wallet_info: &Value,
    balances: &GetBalancesResponse,
) -> Result<String, WalletOperatorError> {
    let maybe_wallet_name = wallet_info
        .get("walletname")
        .and_then(Value::as_str)
        .or(maybe_wallet_name);
    let summary = json!({
        "wallet": maybe_wallet_name,
        "address": args.address,
        "amount_sats": args.amount_sats,
        "fee_sats": preview.fee_sats,
        "fee_rate_sat_per_kvb": preview.fee_sats,
        "inputs_selected": preview.inputs.len(),
        "maybe_change_output_index": preview.maybe_change_output_index,
        "transaction_hex": preview.transaction_hex,
        "trusted_balance_sats": balances.mine.trusted_sats,
        "freshness": wallet_info.get("freshness").cloned().unwrap_or(Value::Null),
        "scanning": wallet_info.get("scanning").cloned().unwrap_or(Value::Bool(false)),
        "maybe_tip_height": wallet_info.get("maybe_tip_height").cloned().unwrap_or(Value::Null),
        "fee_selection": render_fee_selection(intent),
        "change_policy": render_change_policy(intent),
        "replaceable": intent.enable_rbf,
        "maybe_fee_ceiling_sats": intent.maybe_fee_ceiling_sats,
    });
    if format == OperatorOutputFormat::Json {
        return serde_json::to_string_pretty(&json!({
            "mode": "preview",
            "summary": summary,
            "confirmation_required": true,
        }))
        .map(|value| format!("{value}\n"))
        .map_err(WalletOperatorError::from);
    }

    Ok(format!(
        "Wallet: {}\nFreshness: {}\nScanning: {}\nTrusted balance (sats): {}\nDestination: {}\nAmount (sats): {}\nFee selection: {}\nChange policy: {}\nReplaceable: {}\nFee (sats): {}\nSelected inputs: {}\nChange output index: {}\nTransaction hex: {}\n",
        maybe_wallet_name.unwrap_or("Unavailable"),
        summary["freshness"].as_str().unwrap_or("unavailable"),
        summary["scanning"].as_bool().unwrap_or(false),
        balances.mine.trusted_sats,
        args.address,
        args.amount_sats,
        render_fee_selection(intent),
        render_change_policy(intent),
        intent.enable_rbf,
        preview.fee_sats,
        preview.inputs.len(),
        preview
            .maybe_change_output_index
            .map_or_else(|| "none".to_string(), |index| index.to_string()),
        preview.transaction_hex
    ))
}

fn render_send_submission(
    format: OperatorOutputFormat,
    maybe_wallet_name: Option<&str>,
    txid: &str,
) -> Result<String, OperatorRuntimeError> {
    if format == OperatorOutputFormat::Json {
        return serde_json::to_string_pretty(&json!({
            "wallet": maybe_wallet_name,
            "txid": txid,
        }))
        .map(|value| format!("{value}\n"))
        .map_err(to_invalid_request);
    }
    Ok(format!(
        "Submitted transaction {txid} for wallet {}\n",
        maybe_wallet_name.unwrap_or("Unavailable")
    ))
}

fn render_backup_success(
    format: OperatorOutputFormat,
    wallet_name: &str,
    destination: &Path,
) -> Result<String, OperatorRuntimeError> {
    if format == OperatorOutputFormat::Json {
        return serde_json::to_string_pretty(&json!({
            "wallet": wallet_name,
            "destination": destination.display().to_string(),
            "format": BACKUP_EXPORT_FORMAT,
            "version": BACKUP_EXPORT_VERSION,
        }))
        .map(|value| format!("{value}\n"))
        .map_err(to_invalid_request);
    }
    Ok(format!(
        "Wrote Open Bitcoin wallet backup for {wallet_name} to {}\n",
        destination.display()
    ))
}

fn render_fee_selection(intent: &SendIntent) -> String {
    match intent.fee_selection {
        FeeSelection::Explicit(fee_rate) => format!("explicit {} sat/kvB", fee_rate.sats_per_kvb()),
        FeeSelection::Estimate(request) => format!(
            "estimate conf_target={} mode={}",
            request.conf_target,
            render_fee_estimate_mode(request.mode)
        ),
    }
}

fn render_change_policy(intent: &SendIntent) -> String {
    match intent.change_policy {
        ChangePolicy::Automatic => "automatic".to_string(),
        ChangePolicy::ChangeForbidden => "forbidden".to_string(),
        ChangePolicy::FixedDescriptor(descriptor_id) => format!("fixed descriptor {descriptor_id}"),
    }
}

const fn render_fee_estimate_mode(mode: FeeEstimateMode) -> &'static str {
    match mode {
        FeeEstimateMode::Unset => "unset",
        FeeEstimateMode::Economical => "economical",
        FeeEstimateMode::Conservative => "conservative",
    }
}

fn resolve_managed_wallet_name(
    maybe_data_dir: Option<&Path>,
    maybe_requested_wallet_name: Option<&str>,
) -> Result<Option<String>, WalletOperatorError> {
    let Some(data_dir) = maybe_data_dir else {
        return Ok(maybe_requested_wallet_name.map(str::to_owned));
    };
    let Ok(store) = FjallNodeStore::open(data_dir) else {
        return Ok(maybe_requested_wallet_name.map(str::to_owned));
    };
    let Ok(registry) = WalletRegistry::load(&store) else {
        return Ok(maybe_requested_wallet_name.map(str::to_owned));
    };

    if let Some(wallet_name) = maybe_requested_wallet_name {
        if registry
            .wallet_names()
            .iter()
            .any(|candidate| candidate == wallet_name)
        {
            return Ok(Some(wallet_name.to_string()));
        }
        return Err(WalletOperatorError::new(
            "Requested wallet does not exist or is not loaded",
        ));
    }
    if let Some(selected_wallet_name) = registry.selected_wallet_name() {
        return Ok(Some(selected_wallet_name.to_string()));
    }
    match registry.wallet_names() {
        [] => Ok(None),
        [wallet_name] => Ok(Some(wallet_name.clone())),
        _ => Err(WalletOperatorError::new(MULTI_WALLET_SELECTION_MESSAGE)),
    }
}

fn ensure_safe_backup_destination(
    destination: &Path,
    detections: &[DetectedInstallation],
) -> Result<(), WalletOperatorError> {
    let destination = absolutize_path(destination);
    for detection in detections {
        for candidate in &detection.wallet_candidates {
            let candidate_path = absolutize_path(&candidate.path);
            if path_overlaps_wallet_candidate(&destination, &candidate_path) {
                return Err(WalletOperatorError::new(format!(
                    "backup destination overlaps detected external wallet candidate {}",
                    candidate.path.display()
                )));
            }
        }
    }
    Ok(())
}

fn path_overlaps_wallet_candidate(destination: &Path, candidate_path: &Path) -> bool {
    destination.starts_with(candidate_path)
        || candidate_path.starts_with(destination)
        || destination
            .parent()
            .is_some_and(|parent| parent.starts_with(candidate_path))
        || candidate_path
            .parent()
            .is_some_and(|parent| destination.starts_with(parent))
}

fn absolutize_path(path: &Path) -> PathBuf {
    if path.exists() {
        return fs::canonicalize(path).unwrap_or_else(|_| path.to_path_buf());
    }
    if let Some(parent) = path.parent()
        && parent.exists()
    {
        let maybe_name = path.file_name().map(|name| name.to_os_string());
        let canonical_parent = fs::canonicalize(parent).unwrap_or_else(|_| parent.to_path_buf());
        if let Some(name) = maybe_name {
            return canonical_parent.join(name);
        }
        return canonical_parent;
    }
    if path.is_absolute() {
        return path.to_path_buf();
    }
    std::env::current_dir()
        .unwrap_or_else(|_| PathBuf::from("."))
        .join(path)
}

const fn map_estimate_mode(mode: WalletEstimateMode) -> FeeEstimateMode {
    match mode {
        WalletEstimateMode::Unset => FeeEstimateMode::Unset,
        WalletEstimateMode::Economical => FeeEstimateMode::Economical,
        WalletEstimateMode::Conservative => FeeEstimateMode::Conservative,
    }
}

fn map_rpc_estimate_mode(mode: WalletEstimateMode) -> open_bitcoin_rpc::method::EstimateMode {
    match mode {
        WalletEstimateMode::Unset => open_bitcoin_rpc::method::EstimateMode::Unset,
        WalletEstimateMode::Economical => open_bitcoin_rpc::method::EstimateMode::Economical,
        WalletEstimateMode::Conservative => open_bitcoin_rpc::method::EstimateMode::Conservative,
    }
}

const fn map_network_selection(network: NetworkSelection) -> AddressNetwork {
    match network {
        NetworkSelection::Mainnet => AddressNetwork::Mainnet,
        NetworkSelection::Testnet => AddressNetwork::Testnet,
        NetworkSelection::Signet => AddressNetwork::Signet,
        NetworkSelection::Regtest => AddressNetwork::Regtest,
    }
}

fn resolve_fee_estimate(conf_target: u16, mode: FeeEstimateMode) -> FeeRate {
    let base_rate: i64 = match conf_target {
        0..=2 => 2_500,
        3..=6 => 2_000,
        7..=12 => 1_500,
        _ => 1_000,
    };
    let resolved_rate = match mode {
        FeeEstimateMode::Unset => base_rate,
        FeeEstimateMode::Economical => base_rate.saturating_sub(250).max(1_000),
        FeeEstimateMode::Conservative => base_rate.saturating_add(250),
    };
    FeeRate::from_sats_per_kvb(resolved_rate)
}

fn to_invalid_request(error: impl std::fmt::Display) -> OperatorRuntimeError {
    OperatorRuntimeError::InvalidRequest {
        message: error.to_string(),
    }
}

fn encode_hex(bytes: &[u8]) -> String {
    let mut encoded = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        encoded.push(nibble_to_hex(byte >> 4));
        encoded.push(nibble_to_hex(byte & 0x0f));
    }
    encoded
}

const fn nibble_to_hex(value: u8) -> char {
    match value {
        0..=9 => (b'0' + value) as char,
        10..=15 => (b'a' + (value - 10)) as char,
        _ => '?',
    }
}

#[cfg(test)]
mod tests;
