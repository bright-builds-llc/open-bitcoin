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
        DescriptorImportResult, GetBalancesResponse, GetBlockchainInfoResponse,
        GetMempoolInfoResponse, GetNetworkInfoResponse, GetWalletInfoResponse, ListUnspentEntry,
        ListUnspentRequest, ListUnspentResponse, MethodCall, RescanBlockchainRequest,
        RescanBlockchainResponse, SelectedInput, SendRawTransactionResponse, WalletBalanceDetails,
    },
};

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
        MethodCall::GetWalletInfo(_request) => serde_json::to_value(get_wallet_info(context))
            .map_err(|error| RpcFailure::internal_error(error.to_string())),
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

fn get_wallet_info(context: &ManagedRpcContext) -> GetWalletInfoResponse {
    let wallet_info = context.wallet_info();
    GetWalletInfoResponse {
        network: wallet_info.network.to_string(),
        descriptor_count: wallet_info.descriptor_count,
        utxo_count: wallet_info.utxo_count,
        maybe_tip_height: wallet_info.maybe_tip_height,
        maybe_tip_median_time_past: wallet_info.maybe_tip_median_time_past,
    }
}

fn get_balances(context: &ManagedRpcContext) -> Result<GetBalancesResponse, RpcFailure> {
    let balance = context
        .wallet_balance(context.coinbase_maturity())
        .map_err(wallet_error_to_failure)?;
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

    for utxo in context.wallet_utxos() {
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
            let address = context
                .descriptor_address(utxo.descriptor_id)
                .map_err(wallet_error_to_failure)?;
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
                maybe_message: Some(error.to_string()),
            }),
        }
    }

    Ok(crate::method::ImportDescriptorsResponse { results })
}

fn rescan_blockchain(
    context: &mut ManagedRpcContext,
    request: RescanBlockchainRequest,
) -> Result<RescanBlockchainResponse, RpcFailure> {
    let snapshot = context.blockchain_snapshot();
    let tip_height = snapshot.tip().map_or(0, |tip| tip.height);
    let start_height = request.maybe_start_height.unwrap_or(0);
    let stop_height = request.maybe_stop_height.unwrap_or(tip_height);
    if start_height > tip_height {
        return Err(RpcFailure::invalid_params("Invalid start_height"));
    }
    if stop_height > tip_height {
        return Err(RpcFailure::invalid_params("Invalid stop_height"));
    }
    if stop_height < start_height {
        return Err(RpcFailure::invalid_params(
            "stop_height must be greater than start_height",
        ));
    }

    context
        .rescan_wallet(&snapshot)
        .map_err(wallet_error_to_failure)?;
    Ok(RescanBlockchainResponse {
        start_height,
        stop_height,
    })
}

fn send_raw_transaction(
    context: &mut ManagedRpcContext,
    request: crate::method::SendRawTransactionRequest,
) -> Result<SendRawTransactionResponse, RpcFailure> {
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
    let built = context
        .build_transaction(&build_request, context.coinbase_maturity())
        .map_err(wallet_error_to_failure)?;
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
    let built = context
        .build_and_sign_transaction(&build_request, context.coinbase_maturity())
        .map_err(wallet_error_to_failure)?;
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
