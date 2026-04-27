// Parity breadcrumbs:
// - packages/bitcoin-knots/src/bitcoind.cpp
// - packages/bitcoin-knots/src/rpc/protocol.h
// - packages/bitcoin-knots/src/rpc/request.cpp
// - packages/bitcoin-knots/src/rpc/server.cpp
// - packages/bitcoin-knots/src/rpc/blockchain.cpp
// - packages/bitcoin-knots/src/rpc/mempool.cpp
// - packages/bitcoin-knots/src/rpc/net.cpp
// - packages/bitcoin-knots/src/rpc/rawtransaction.cpp
// - packages/bitcoin-knots/test/functional/interface_rpc.py

use serde_json::Value;

use open_bitcoin_node::core::{
    mempool::FeeRate,
    primitives::{Amount, ScriptBuf},
    wallet::{DescriptorRole, Recipient, WalletError},
};

use crate::{
    ManagedRpcContext,
    error::RpcFailure,
    method::{
        BuildAndSignTransactionRequest, BuildAndSignTransactionResponse, BuildTransactionRequest,
        BuildTransactionResponse, DescriptorImportResult, EstimateMode, GetBalancesResponse,
        GetWalletInfoResponse, ListDescriptorEntry, ListDescriptorsResponse, ListUnspentEntry,
        ListUnspentRequest, ListUnspentResponse, RescanBlockchainRequest, RescanBlockchainResponse,
        SendToAddressRequest, TransactionRecipient, WalletBalanceDetails,
    },
};

use super::{
    decode, map_built_transaction, map_wallet_freshness, network_error_to_failure,
    rpc_failure_message, wallet_error_to_failure,
};

pub(super) fn send_to_address(
    context: &mut ManagedRpcContext,
    request: SendToAddressRequest,
) -> Result<String, RpcFailure> {
    let recipient = Recipient {
        script_pubkey: decode::script_pubkey_from_address(context.chain(), &request.address)?,
        value: Amount::from_sats(request.amount_sats)
            .map_err(|error| RpcFailure::invalid_params(error.to_string()))?,
    };
    let build_request = open_bitcoin_node::core::wallet::BuildRequest {
        recipients: vec![recipient],
        fee_rate: resolve_fee_rate_from_request(&request)?,
        maybe_change_descriptor_id: request.maybe_change_descriptor_id,
        maybe_lock_time: request.maybe_lock_time,
        enable_rbf: request.enable_rbf,
    };
    let built = context.build_and_sign_transaction(&build_request, context.coinbase_maturity())?;
    if let Some(max_tx_fee_sats) = request.maybe_max_tx_fee_sats
        && built.fee.to_sats() > max_tx_fee_sats
    {
        return Err(wallet_error_to_failure(WalletError::FeeCeilingExceeded {
            fee_sats: built.fee.to_sats(),
            ceiling_sats: max_tx_fee_sats,
        }));
    }

    let submitted = context
        .submit_local_transaction(built.transaction)
        .map_err(network_error_to_failure)?;
    Ok(decode::encode_hex(submitted.accepted.as_bytes()))
}

pub(super) fn get_new_address(context: &mut ManagedRpcContext) -> Result<String, RpcFailure> {
    context
        .allocate_receive_address()
        .map(|address| address.to_string())
}

pub(super) fn get_raw_change_address(
    context: &mut ManagedRpcContext,
) -> Result<String, RpcFailure> {
    context
        .allocate_change_address()
        .map(|address| address.to_string())
}

pub(super) fn list_descriptors(
    context: &ManagedRpcContext,
) -> Result<ListDescriptorsResponse, RpcFailure> {
    let snapshot = context.wallet_snapshot()?;
    let descriptors = snapshot
        .descriptors
        .iter()
        .map(|record| ListDescriptorEntry {
            descriptor: record.descriptor.display_text(),
            active: true,
            internal: record.role == DescriptorRole::Internal,
            maybe_range: match (
                record.descriptor.range_start(),
                record.descriptor.range_end(),
            ) {
                (Some(start), Some(end)) => Some(crate::method::DescriptorRange { start, end }),
                _ => None,
            },
            maybe_next_index: record.descriptor.next_index(),
        })
        .collect();

    Ok(ListDescriptorsResponse {
        maybe_wallet_name: context.selected_wallet_name()?,
        descriptors,
    })
}

pub(super) fn get_wallet_info(context: &ManagedRpcContext) -> Result<Value, RpcFailure> {
    let wallet_info = context.selected_wallet_info()?;
    let freshness = context.wallet_freshness()?;
    let base = GetWalletInfoResponse {
        network: wallet_info.network.to_string(),
        descriptor_count: wallet_info.descriptor_count,
        utxo_count: wallet_info.utxo_count,
        maybe_tip_height: wallet_info.maybe_tip_height,
        maybe_tip_median_time_past: wallet_info.maybe_tip_median_time_past,
    };
    let mut value = serde_json::to_value(base)
        .map_err(|error| RpcFailure::internal_error(error.to_string()))?;
    let Value::Object(ref mut object) = value else {
        return Err(RpcFailure::internal_error(
            "getwalletinfo response must serialize to an object",
        ));
    };
    object.insert(
        "walletname".to_string(),
        context
            .selected_wallet_name()?
            .map_or(Value::Null, Value::String),
    );
    object.insert("scanning".to_string(), Value::Bool(freshness.scanning));
    object.insert(
        "freshness".to_string(),
        serde_json::to_value(map_wallet_freshness(freshness.freshness))
            .map_err(|error| RpcFailure::internal_error(error.to_string()))?,
    );
    if let Some(scanned_through_height) = freshness.maybe_scanned_through_height {
        object.insert(
            "maybe_scanned_through_height".to_string(),
            Value::from(scanned_through_height),
        );
    }
    if let Some(target_height) = freshness.maybe_target_height {
        object.insert(
            "maybe_rescan_target_height".to_string(),
            Value::from(target_height),
        );
    }
    if let Some(next_height) = freshness.maybe_next_height {
        object.insert(
            "maybe_rescan_next_height".to_string(),
            Value::from(next_height),
        );
    }

    Ok(value)
}

pub(super) fn get_balances(context: &ManagedRpcContext) -> Result<GetBalancesResponse, RpcFailure> {
    let balance = context.wallet_balance(context.coinbase_maturity())?;
    Ok(GetBalancesResponse {
        mine: WalletBalanceDetails {
            trusted_sats: balance.spendable.to_sats(),
            untrusted_pending_sats: 0,
            immature_sats: balance.immature.to_sats(),
        },
    })
}

pub(super) fn list_unspent(
    context: &ManagedRpcContext,
    request: ListUnspentRequest,
) -> Result<ListUnspentResponse, RpcFailure> {
    let tip_height = context.maybe_chain_tip().map_or(0, |tip| tip.height);
    let mut entries = Vec::new();
    let mut total_amount_sats = 0_i64;

    for utxo in context.wallet_utxos()? {
        let confirmations = if tip_height >= utxo.created_height {
            tip_height - utxo.created_height + 1
        } else {
            0
        };
        if let Some(min_confirmations) = request.maybe_min_confirmations
            && confirmations < min_confirmations
        {
            continue;
        }
        if let Some(max_confirmations) = request.maybe_max_confirmations
            && confirmations > max_confirmations
        {
            continue;
        }
        if !request.query.include_immature_coinbase
            && utxo.is_coinbase
            && confirmations < context.coinbase_maturity()
        {
            continue;
        }

        let amount_sats = utxo.output.value.to_sats();
        if let Some(minimum_amount_sats) = request.query.maybe_minimum_amount_sats
            && amount_sats < minimum_amount_sats
        {
            continue;
        }
        if let Some(maximum_amount_sats) = request.query.maybe_maximum_amount_sats
            && amount_sats > maximum_amount_sats
        {
            continue;
        }
        if !request.addresses.is_empty() {
            let address = context.descriptor_address(utxo.descriptor_id)?;
            if !request
                .addresses
                .iter()
                .any(|candidate| candidate == address.as_str())
            {
                continue;
            }
        }

        total_amount_sats += amount_sats;
        entries.push(ListUnspentEntry {
            txid_hex: decode::encode_hex(utxo.outpoint.txid.as_bytes()),
            vout: utxo.outpoint.vout,
            amount_sats,
            descriptor_id: utxo.descriptor_id,
            is_coinbase: utxo.is_coinbase,
            confirmations,
        });
    }

    if let Some(minimum_sum_amount_sats) = request.query.maybe_minimum_sum_amount_sats
        && total_amount_sats < minimum_sum_amount_sats
    {
        entries.clear();
    }
    if let Some(maximum_count) = request.query.maybe_maximum_count {
        entries.truncate(maximum_count);
    }

    Ok(ListUnspentResponse { entries })
}

pub(super) fn import_descriptors(
    context: &mut ManagedRpcContext,
    request: crate::method::ImportDescriptorsRequest,
) -> Result<crate::method::ImportDescriptorsResponse, RpcFailure> {
    let mut results = Vec::with_capacity(request.requests.len());

    for item in request.requests {
        let role = if item.internal {
            DescriptorRole::Internal
        } else {
            DescriptorRole::External
        };
        match context.import_descriptor(item.label, role, &item.descriptor) {
            Ok(descriptor_id) => results.push(DescriptorImportResult {
                success: true,
                maybe_descriptor_id: Some(descriptor_id),
                maybe_message: None,
            }),
            Err(error) => results.push(DescriptorImportResult {
                success: false,
                maybe_descriptor_id: None,
                maybe_message: Some(rpc_failure_message(&error)),
            }),
        }
    }

    Ok(crate::method::ImportDescriptorsResponse { results })
}

pub(super) fn rescan_blockchain(
    context: &mut ManagedRpcContext,
    request: RescanBlockchainRequest,
) -> Result<RescanBlockchainResponse, RpcFailure> {
    let execution =
        context.rescan_wallet_range(request.maybe_start_height, request.maybe_stop_height)?;
    Ok(RescanBlockchainResponse {
        start_height: execution.start_height,
        stop_height: execution.stop_height,
        scanning: execution.freshness.scanning,
        freshness: map_wallet_freshness(execution.freshness.freshness),
        maybe_scanned_through_height: execution.freshness.maybe_scanned_through_height,
        maybe_rescan_next_height: execution.freshness.maybe_next_height,
    })
}

pub(super) fn build_transaction(
    context: &ManagedRpcContext,
    request: BuildTransactionRequest,
) -> Result<BuildTransactionResponse, RpcFailure> {
    let build_request = build_wallet_request(
        request.recipients,
        request.fee_rate_sat_per_kvb,
        request.maybe_change_descriptor_id,
        request.maybe_lock_time,
        request.enable_rbf,
    )?;
    let built = context.build_transaction(&build_request, context.coinbase_maturity())?;
    map_built_transaction(&built)
}

pub(super) fn build_and_sign_transaction(
    context: &ManagedRpcContext,
    request: BuildAndSignTransactionRequest,
) -> Result<BuildAndSignTransactionResponse, RpcFailure> {
    let build_request = build_wallet_request(
        request.recipients,
        request.fee_rate_sat_per_kvb,
        request.maybe_change_descriptor_id,
        request.maybe_lock_time,
        request.enable_rbf,
    )?;
    let built = context.build_and_sign_transaction(&build_request, context.coinbase_maturity())?;
    map_built_transaction(&built).map(|response| BuildAndSignTransactionResponse {
        transaction_hex: response.transaction_hex,
        fee_sats: response.fee_sats,
        inputs: response.inputs,
        maybe_change_output_index: response.maybe_change_output_index,
    })
}

fn build_wallet_request(
    recipients: Vec<TransactionRecipient>,
    fee_rate_sat_per_kvb: i64,
    maybe_change_descriptor_id: Option<u32>,
    maybe_lock_time: Option<u32>,
    enable_rbf: bool,
) -> Result<open_bitcoin_node::core::wallet::BuildRequest, RpcFailure> {
    let recipients = recipients
        .into_iter()
        .map(|recipient| {
            let script_bytes = decode::decode_hex(&recipient.script_pubkey_hex)
                .map_err(|error| RpcFailure::invalid_params(error.to_string()))?;
            let script_pubkey = ScriptBuf::from_bytes(script_bytes)
                .map_err(|error| RpcFailure::invalid_params(error.to_string()))?;
            let value = Amount::from_sats(recipient.amount_sats)
                .map_err(|error| RpcFailure::invalid_params(error.to_string()))?;
            Ok(Recipient {
                script_pubkey,
                value,
            })
        })
        .collect::<Result<Vec<_>, RpcFailure>>()?;

    Ok(open_bitcoin_node::core::wallet::BuildRequest {
        recipients,
        fee_rate: FeeRate::from_sats_per_kvb(fee_rate_sat_per_kvb),
        maybe_change_descriptor_id,
        maybe_lock_time,
        enable_rbf,
    })
}

fn resolve_fee_rate_from_request(request: &SendToAddressRequest) -> Result<FeeRate, RpcFailure> {
    if request.maybe_fee_rate_sat_per_kvb.is_some()
        && (request.maybe_conf_target.is_some() || request.maybe_estimate_mode.is_some())
    {
        return Err(RpcFailure::invalid_params(
            "sendtoaddress accepts either fee_rate_sat_per_kvb or conf_target/estimate_mode, but not both",
        ));
    }

    if let Some(fee_rate_sat_per_kvb) = request.maybe_fee_rate_sat_per_kvb {
        return Ok(FeeRate::from_sats_per_kvb(fee_rate_sat_per_kvb));
    }

    Ok(resolve_fee_estimate(
        request.maybe_conf_target.unwrap_or(6),
        request.maybe_estimate_mode.unwrap_or(EstimateMode::Unset),
    ))
}

fn resolve_fee_estimate(conf_target: u16, mode: EstimateMode) -> FeeRate {
    let base_rate: i64 = match conf_target {
        0..=2 => 2_500,
        3..=6 => 2_000,
        7..=12 => 1_500,
        _ => 1_000,
    };
    let resolved_rate = match mode {
        EstimateMode::Unset => base_rate,
        EstimateMode::Economical => base_rate.saturating_sub(250).max(1_000),
        EstimateMode::Conservative => base_rate.saturating_add(250),
    };

    FeeRate::from_sats_per_kvb(resolved_rate)
}
