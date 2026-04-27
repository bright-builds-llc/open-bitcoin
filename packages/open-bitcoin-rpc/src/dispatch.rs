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

use open_bitcoin_node::ManagedNetworkError;
use open_bitcoin_node::core::{
    codec::{TransactionEncoding, encode_transaction, parse_transaction},
    mempool::FeeRate,
    primitives::{Amount, ScriptBuf},
    wallet::{DescriptorRole, Recipient, SingleKeyDescriptor, WalletError},
};

use crate::{
    ManagedRpcContext,
    error::{RpcErrorCode, RpcErrorDetail, RpcFailure},
    method::{
        BuildAndSignTransactionRequest, BuildAndSignTransactionResponse, BuildTransactionRequest,
        BuildTransactionResponse, DeriveAddressesRequest, DeriveAddressesResponse,
        DescriptorImportResult, EstimateMode, GetBalancesResponse, GetBlockchainInfoResponse,
        GetMempoolInfoResponse, GetNetworkInfoResponse, GetWalletInfoResponse, ListDescriptorEntry,
        ListDescriptorsResponse, ListUnspentEntry, ListUnspentRequest, ListUnspentResponse,
        MethodCall, RescanBlockchainRequest, RescanBlockchainResponse, SelectedInput,
        SendRawTransactionResponse, SendToAddressRequest, WalletBalanceDetails, WalletFreshness,
    },
};
const UNSUPPORTED_MAX_FEE_RATE_MESSAGE: &str =
    "sendrawtransaction maxfeerate enforcement is not supported in Phase 8; omit maxfeerate";
const UNSUPPORTED_MAX_BURN_AMOUNT_MESSAGE: &str =
    "sendrawtransaction maxburnamount enforcement is not supported in Phase 8; omit maxburnamount";
const BASE58_ALPHABET: &[u8; 58] = b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";
const BECH32_ALPHABET: &[u8; 32] = b"qpzry9x8gf2tvdw0s3jn54khce6mua7l";

pub fn dispatch(context: &mut ManagedRpcContext, call: MethodCall) -> Result<Value, RpcFailure> {
    match call {
        MethodCall::GetBlockchainInfo(_request) => {
            serde_json::to_value(get_blockchain_info(context))
                .map_err(|error| RpcFailure::internal_error(error.to_string()))
        }
        MethodCall::GetMempoolInfo(_request) => serde_json::to_value(get_mempool_info(context))
            .map_err(|error| RpcFailure::internal_error(error.to_string())),
        MethodCall::GetNetworkInfo(_request) => serde_json::to_value(get_network_info(context))
            .map_err(|error| RpcFailure::internal_error(error.to_string())),
        MethodCall::SendRawTransaction(request) => {
            serde_json::to_value(send_raw_transaction(context, request)?)
                .map_err(|error| RpcFailure::internal_error(error.to_string()))
        }
        MethodCall::DeriveAddresses(request) => {
            serde_json::to_value(derive_addresses(context, request)?)
                .map_err(|error| RpcFailure::internal_error(error.to_string()))
        }
        MethodCall::SendToAddress(request) => {
            serde_json::to_value(send_to_address(context, request)?)
                .map_err(|error| RpcFailure::internal_error(error.to_string()))
        }
        MethodCall::GetNewAddress(_request) => serde_json::to_value(get_new_address(context)?)
            .map_err(|error| RpcFailure::internal_error(error.to_string())),
        MethodCall::GetRawChangeAddress(_request) => {
            serde_json::to_value(get_raw_change_address(context)?)
                .map_err(|error| RpcFailure::internal_error(error.to_string()))
        }
        MethodCall::ListDescriptors(_request) => serde_json::to_value(list_descriptors(context)?)
            .map_err(|error| RpcFailure::internal_error(error.to_string())),
        MethodCall::GetWalletInfo(_request) => get_wallet_info(context),
        MethodCall::GetBalances(_request) => serde_json::to_value(get_balances(context)?)
            .map_err(|error| RpcFailure::internal_error(error.to_string())),
        MethodCall::ListUnspent(request) => serde_json::to_value(list_unspent(context, request)?)
            .map_err(|error| RpcFailure::internal_error(error.to_string())),
        MethodCall::ImportDescriptors(request) => {
            serde_json::to_value(import_descriptors(context, request)?)
                .map_err(|error| RpcFailure::internal_error(error.to_string()))
        }
        MethodCall::RescanBlockchain(request) => {
            serde_json::to_value(rescan_blockchain(context, request)?)
                .map_err(|error| RpcFailure::internal_error(error.to_string()))
        }
        MethodCall::BuildTransaction(request) => {
            serde_json::to_value(build_transaction(context, request)?)
                .map_err(|error| RpcFailure::internal_error(error.to_string()))
        }
        MethodCall::BuildAndSignTransaction(request) => {
            serde_json::to_value(build_and_sign_transaction(context, request)?)
                .map_err(|error| RpcFailure::internal_error(error.to_string()))
        }
    }
}

fn get_blockchain_info(context: &ManagedRpcContext) -> GetBlockchainInfoResponse {
    let maybe_tip = context.maybe_chain_tip();
    GetBlockchainInfoResponse {
        chain: context.chain_name().to_string(),
        blocks: maybe_tip.as_ref().map_or(0, |tip| tip.height),
        headers: maybe_tip.as_ref().map_or(0, |tip| tip.height),
        maybe_best_block_hash: maybe_tip
            .as_ref()
            .map(|tip| encode_hex(tip.block_hash.as_bytes())),
        maybe_median_time_past: maybe_tip.as_ref().map(|tip| tip.median_time_past),
        verificationprogress: if maybe_tip.is_some() { 1.0 } else { 0.0 },
        initialblockdownload: false,
        warnings: Vec::new(),
    }
}

fn get_mempool_info(context: &ManagedRpcContext) -> GetMempoolInfoResponse {
    let info = context.mempool_info();
    GetMempoolInfoResponse {
        size: info.transaction_count,
        bytes: info.total_virtual_size,
        usage: info.total_virtual_size,
        total_fee_sats: info.total_fee_sats,
        maxmempool: info.max_mempool_virtual_size,
        mempoolminfee: info.min_relay_feerate_sats_per_kvb,
        minrelaytxfee: info.min_relay_feerate_sats_per_kvb,
        loaded: true,
    }
}

fn get_network_info(context: &ManagedRpcContext) -> GetNetworkInfoResponse {
    let network_info = context.network_info();
    let mempool_info = context.mempool_info();
    GetNetworkInfoResponse {
        version: version_number(),
        subversion: network_info.user_agent,
        protocolversion: network_info.protocol_version,
        localservices: format!("{:016x}", network_info.local_services_bits),
        localrelay: network_info.relay,
        connections: network_info.connected_peers,
        connections_in: network_info.inbound_peers,
        connections_out: network_info.outbound_peers,
        relayfee: mempool_info.min_relay_feerate_sats_per_kvb,
        incrementalfee: mempool_info.incremental_relay_feerate_sats_per_kvb,
        warnings: Vec::new(),
    }
}

fn derive_addresses(
    context: &ManagedRpcContext,
    request: DeriveAddressesRequest,
) -> Result<DeriveAddressesResponse, RpcFailure> {
    let descriptor = SingleKeyDescriptor::parse(&request.descriptor, context.chain())
        .map_err(wallet_error_to_failure)?;
    let address = descriptor
        .address(context.chain())
        .map_err(wallet_error_to_failure)?;
    Ok(DeriveAddressesResponse {
        addresses: vec![address.to_string()],
    })
}

fn send_to_address(
    context: &mut ManagedRpcContext,
    request: SendToAddressRequest,
) -> Result<String, RpcFailure> {
    let recipient = Recipient {
        script_pubkey: script_pubkey_from_address(context.chain(), &request.address)?,
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
    Ok(encode_hex(submitted.accepted.as_bytes()))
}

fn get_new_address(context: &mut ManagedRpcContext) -> Result<String, RpcFailure> {
    context
        .allocate_receive_address()
        .map(|address| address.to_string())
}

fn get_raw_change_address(context: &mut ManagedRpcContext) -> Result<String, RpcFailure> {
    context
        .allocate_change_address()
        .map(|address| address.to_string())
}

fn list_descriptors(context: &ManagedRpcContext) -> Result<ListDescriptorsResponse, RpcFailure> {
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

fn get_wallet_info(context: &ManagedRpcContext) -> Result<Value, RpcFailure> {
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

fn get_balances(context: &ManagedRpcContext) -> Result<GetBalancesResponse, RpcFailure> {
    let balance = context.wallet_balance(context.coinbase_maturity())?;
    Ok(GetBalancesResponse {
        mine: WalletBalanceDetails {
            trusted_sats: balance.spendable.to_sats(),
            untrusted_pending_sats: 0,
            immature_sats: balance.immature.to_sats(),
        },
    })
}

fn list_unspent(
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
            txid_hex: encode_hex(utxo.outpoint.txid.as_bytes()),
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

fn import_descriptors(
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

fn rescan_blockchain(
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

fn send_raw_transaction(
    context: &mut ManagedRpcContext,
    request: crate::method::SendRawTransactionRequest,
) -> Result<SendRawTransactionResponse, RpcFailure> {
    if request.maybe_max_fee_rate_sat_per_kvb.is_some() {
        return Err(RpcFailure::invalid_params(UNSUPPORTED_MAX_FEE_RATE_MESSAGE));
    }
    if request.maybe_max_burn_amount_sats.is_some() {
        return Err(RpcFailure::invalid_params(
            UNSUPPORTED_MAX_BURN_AMOUNT_MESSAGE,
        ));
    }

    let transaction_bytes = decode_hex(&request.transaction_hex)
        .map_err(|error| RpcFailure::invalid_params(error.to_string()))?;
    let transaction = parse_transaction(&transaction_bytes)
        .map_err(|error| RpcFailure::invalid_params(error.to_string()))?;
    let result = context
        .submit_local_transaction(transaction)
        .map_err(network_error_to_failure)?;

    Ok(SendRawTransactionResponse {
        txid_hex: encode_hex(result.accepted.as_bytes()),
        replaced_txids: result
            .replaced
            .into_iter()
            .map(|txid| encode_hex(txid.as_bytes()))
            .collect(),
        evicted_txids: result
            .evicted
            .into_iter()
            .map(|txid| encode_hex(txid.as_bytes()))
            .collect(),
    })
}

fn build_transaction(
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

fn build_and_sign_transaction(
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
    recipients: Vec<crate::method::TransactionRecipient>,
    fee_rate_sat_per_kvb: i64,
    maybe_change_descriptor_id: Option<u32>,
    maybe_lock_time: Option<u32>,
    enable_rbf: bool,
) -> Result<open_bitcoin_node::core::wallet::BuildRequest, RpcFailure> {
    let recipients = recipients
        .into_iter()
        .map(|recipient| {
            let script_bytes = decode_hex(&recipient.script_pubkey_hex)
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

fn map_wallet_freshness(freshness: crate::context::WalletFreshnessKind) -> WalletFreshness {
    match freshness {
        crate::context::WalletFreshnessKind::Fresh => WalletFreshness::Fresh,
        crate::context::WalletFreshnessKind::Partial => WalletFreshness::Partial,
        crate::context::WalletFreshnessKind::Scanning => WalletFreshness::Scanning,
    }
}

fn map_built_transaction(
    built: &open_bitcoin_node::core::wallet::BuiltTransaction,
) -> Result<BuildTransactionResponse, RpcFailure> {
    let transaction_hex = encode_hex(
        &encode_transaction(&built.transaction, TransactionEncoding::WithWitness)
            .map_err(|error| RpcFailure::internal_error(error.to_string()))?,
    );
    let inputs = built
        .selected_inputs
        .iter()
        .map(|utxo| SelectedInput {
            txid_hex: encode_hex(utxo.outpoint.txid.as_bytes()),
            vout: utxo.outpoint.vout,
            descriptor_id: utxo.descriptor_id,
            amount_sats: utxo.output.value.to_sats(),
        })
        .collect();

    Ok(BuildTransactionResponse {
        transaction_hex,
        fee_sats: built.fee.to_sats(),
        inputs,
        maybe_change_output_index: built.change_output_index,
    })
}

fn wallet_error_to_failure(error: WalletError) -> RpcFailure {
    RpcFailure::wallet_error(error.to_string())
}

fn rpc_failure_message(failure: &RpcFailure) -> String {
    failure.maybe_detail.as_ref().map_or_else(
        || "Internal error".to_string(),
        |detail| detail.message.clone(),
    )
}

fn script_pubkey_from_address(
    network: open_bitcoin_node::core::wallet::AddressNetwork,
    address: &str,
) -> Result<ScriptBuf, RpcFailure> {
    if address.starts_with(network.hrp()) {
        return decode_segwit_script(network, address);
    }

    decode_base58_script(network, address)
}

fn decode_base58_script(
    network: open_bitcoin_node::core::wallet::AddressNetwork,
    address: &str,
) -> Result<ScriptBuf, RpcFailure> {
    let decoded = base58_decode(address)?;
    if decoded.len() < 5 {
        return Err(RpcFailure::invalid_params("invalid destination address"));
    }

    let payload_len = decoded.len().saturating_sub(4);
    let (payload, checksum) = decoded.split_at(payload_len);
    let expected_checksum = open_bitcoin_node::core::consensus::crypto::double_sha256(payload);
    if checksum != &expected_checksum[..4] {
        return Err(RpcFailure::invalid_params("invalid destination address"));
    }
    let Some((prefix, body)) = payload.split_first() else {
        return Err(RpcFailure::invalid_params("invalid destination address"));
    };
    match (*prefix, body.len()) {
        (prefix, 20) if prefix == network.p2pkh_prefix() => {
            let mut script = vec![0x76, 0xa9, 0x14];
            script.extend_from_slice(body);
            script.extend_from_slice(&[0x88, 0xac]);
            ScriptBuf::from_bytes(script)
                .map_err(|error| RpcFailure::invalid_params(error.to_string()))
        }
        (prefix, 20) if prefix == network.p2sh_prefix() => {
            let mut script = vec![0xa9, 0x14];
            script.extend_from_slice(body);
            script.push(0x87);
            ScriptBuf::from_bytes(script)
                .map_err(|error| RpcFailure::invalid_params(error.to_string()))
        }
        _ => Err(RpcFailure::invalid_params("invalid destination address")),
    }
}

fn decode_segwit_script(
    network: open_bitcoin_node::core::wallet::AddressNetwork,
    address: &str,
) -> Result<ScriptBuf, RpcFailure> {
    let (hrp, data, bech32m) = bech32_decode(address)?;
    if hrp != network.hrp() || data.is_empty() {
        return Err(RpcFailure::invalid_params("invalid destination address"));
    }
    let version = data[0];
    let program = convert_bits(&data[1..], 5, 8, false)?;
    if version == 0 && bech32m {
        return Err(RpcFailure::invalid_params("invalid destination address"));
    }
    if version != 0 && !bech32m {
        return Err(RpcFailure::invalid_params("invalid destination address"));
    }

    let opcode = if version == 0 { 0x00 } else { 0x50 + version };
    let mut script = vec![opcode, program.len() as u8];
    script.extend_from_slice(&program);
    ScriptBuf::from_bytes(script).map_err(|error| RpcFailure::invalid_params(error.to_string()))
}

fn base58_decode(input: &str) -> Result<Vec<u8>, RpcFailure> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Err(RpcFailure::invalid_params("invalid destination address"));
    }

    let mut bytes = vec![0_u8];
    for ch in trimmed.bytes() {
        let Some(mut carry) = BASE58_ALPHABET
            .iter()
            .position(|candidate| *candidate == ch)
        else {
            return Err(RpcFailure::invalid_params("invalid destination address"));
        };
        for byte in bytes.iter_mut().rev() {
            let value = usize::from(*byte) * 58 + carry;
            *byte = (value & 0xff) as u8;
            carry = value >> 8;
        }
        while carry > 0 {
            bytes.insert(0, (carry & 0xff) as u8);
            carry >>= 8;
        }
    }

    let leading_zeros = trimmed.bytes().take_while(|byte| *byte == b'1').count();
    let mut decoded = vec![0_u8; leading_zeros];
    decoded.extend(bytes.into_iter().skip_while(|byte| *byte == 0));
    Ok(decoded)
}

fn bech32_decode(input: &str) -> Result<(String, Vec<u8>, bool), RpcFailure> {
    let trimmed = input.trim();
    let Some(separator) = trimmed.rfind('1') else {
        return Err(RpcFailure::invalid_params("invalid destination address"));
    };
    let hrp = trimmed[..separator].to_ascii_lowercase();
    let payload = &trimmed[separator + 1..];
    if hrp.is_empty() || payload.len() < 6 {
        return Err(RpcFailure::invalid_params("invalid destination address"));
    }

    let values = payload
        .bytes()
        .map(|byte| {
            BECH32_ALPHABET
                .iter()
                .position(|candidate| *candidate == byte.to_ascii_lowercase())
                .map(|index| index as u8)
                .ok_or_else(|| RpcFailure::invalid_params("invalid destination address"))
        })
        .collect::<Result<Vec<_>, _>>()?;
    let polymod = bech32_polymod(&[expand_hrp(&hrp), values.clone()].concat());
    let bech32m = match polymod {
        1 => false,
        0x2bc8_30a3 => true,
        _ => return Err(RpcFailure::invalid_params("invalid destination address")),
    };
    Ok((hrp, values[..values.len() - 6].to_vec(), bech32m))
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
        for (index, generator) in [
            0x3b6a_57b2_u32,
            0x2650_8e6d,
            0x1ea1_19fa,
            0x3d42_33dd,
            0x2a14_62b3,
        ]
        .iter()
        .enumerate()
        {
            if ((top >> index) & 1) == 1 {
                checksum ^= generator;
            }
        }
    }
    checksum
}

fn convert_bits(data: &[u8], from: u32, to: u32, pad: bool) -> Result<Vec<u8>, RpcFailure> {
    let max_value = (1_u32 << to) - 1;
    let max_accumulator = (1_u32 << (from + to - 1)) - 1;
    let mut accumulator = 0_u32;
    let mut bits = 0_u32;
    let mut output = Vec::new();

    for value in data {
        if (u32::from(*value) >> from) != 0 {
            return Err(RpcFailure::invalid_params("invalid destination address"));
        }
        accumulator = ((accumulator << from) | u32::from(*value)) & max_accumulator;
        bits += from;
        while bits >= to {
            bits -= to;
            output.push(((accumulator >> bits) & max_value) as u8);
        }
    }

    if pad {
        if bits > 0 {
            output.push(((accumulator << (to - bits)) & max_value) as u8);
        }
    } else if bits >= from || ((accumulator << (to - bits)) & max_value) != 0 {
        return Err(RpcFailure::invalid_params("invalid destination address"));
    }

    Ok(output)
}

fn network_error_to_failure(error: ManagedNetworkError) -> RpcFailure {
    match error {
        ManagedNetworkError::Mempool(error) => RpcFailure::verify_rejected(error.to_string()),
        ManagedNetworkError::Chainstate(error) => RpcFailure::new(
            crate::error::RpcFailureKind::InvalidParams,
            Some(RpcErrorDetail::new(
                RpcErrorCode::VerifyRejected,
                error.to_string(),
            )),
        ),
        ManagedNetworkError::Network(error) => RpcFailure::internal_error(error.to_string()),
    }
}

fn version_number() -> i32 {
    let mut parts = env!("CARGO_PKG_VERSION")
        .split('.')
        .map(|part| part.parse::<i32>().unwrap_or(0));
    let major = parts.next().unwrap_or(0);
    let minor = parts.next().unwrap_or(0);
    let patch = parts.next().unwrap_or(0);
    major * 10_000 + minor * 100 + patch
}

fn encode_hex(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut output = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        output.push(HEX[(byte >> 4) as usize] as char);
        output.push(HEX[(byte & 0x0f) as usize] as char);
    }
    output
}

fn decode_hex(text: &str) -> Result<Vec<u8>, &'static str> {
    let trimmed = text.trim();
    if !trimmed.len().is_multiple_of(2) {
        return Err("hex strings must have even length");
    }

    let mut bytes = Vec::with_capacity(trimmed.len() / 2);
    for pair in trimmed.as_bytes().chunks_exact(2) {
        let high = decode_nibble(pair[0])?;
        let low = decode_nibble(pair[1])?;
        bytes.push((high << 4) | low);
    }
    Ok(bytes)
}

fn decode_nibble(byte: u8) -> Result<u8, &'static str> {
    match byte {
        b'0'..=b'9' => Ok(byte - b'0'),
        b'a'..=b'f' => Ok(byte - b'a' + 10),
        b'A'..=b'F' => Ok(byte - b'A' + 10),
        _ => Err("hex strings may only contain ASCII hex digits"),
    }
}

#[cfg(test)]
mod tests;
