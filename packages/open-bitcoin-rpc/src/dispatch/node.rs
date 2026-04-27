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

use open_bitcoin_node::core::{codec::parse_transaction, wallet::SingleKeyDescriptor};

use crate::{
    ManagedRpcContext,
    error::RpcFailure,
    method::{
        DeriveAddressesRequest, DeriveAddressesResponse, GetBlockchainInfoResponse,
        GetMempoolInfoResponse, GetNetworkInfoResponse, SendRawTransactionRequest,
        SendRawTransactionResponse,
    },
};

use super::{decode, network_error_to_failure, version_number, wallet_error_to_failure};

const UNSUPPORTED_MAX_FEE_RATE_MESSAGE: &str =
    "sendrawtransaction maxfeerate enforcement is not supported in Phase 8; omit maxfeerate";
const UNSUPPORTED_MAX_BURN_AMOUNT_MESSAGE: &str =
    "sendrawtransaction maxburnamount enforcement is not supported in Phase 8; omit maxburnamount";

pub(super) fn get_blockchain_info(context: &ManagedRpcContext) -> GetBlockchainInfoResponse {
    let maybe_tip = context.maybe_chain_tip();
    GetBlockchainInfoResponse {
        chain: context.chain_name().to_string(),
        blocks: maybe_tip.as_ref().map_or(0, |tip| tip.height),
        headers: maybe_tip.as_ref().map_or(0, |tip| tip.height),
        maybe_best_block_hash: maybe_tip
            .as_ref()
            .map(|tip| decode::encode_hex(tip.block_hash.as_bytes())),
        maybe_median_time_past: maybe_tip.as_ref().map(|tip| tip.median_time_past),
        verificationprogress: if maybe_tip.is_some() { 1.0 } else { 0.0 },
        initialblockdownload: false,
        warnings: Vec::new(),
    }
}

pub(super) fn get_mempool_info(context: &ManagedRpcContext) -> GetMempoolInfoResponse {
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

pub(super) fn get_network_info(context: &ManagedRpcContext) -> GetNetworkInfoResponse {
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

pub(super) fn derive_addresses(
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

pub(super) fn send_raw_transaction(
    context: &mut ManagedRpcContext,
    request: SendRawTransactionRequest,
) -> Result<SendRawTransactionResponse, RpcFailure> {
    if request.maybe_max_fee_rate_sat_per_kvb.is_some() {
        return Err(RpcFailure::invalid_params(UNSUPPORTED_MAX_FEE_RATE_MESSAGE));
    }
    if request.maybe_max_burn_amount_sats.is_some() {
        return Err(RpcFailure::invalid_params(
            UNSUPPORTED_MAX_BURN_AMOUNT_MESSAGE,
        ));
    }

    let transaction_bytes = decode::decode_hex(&request.transaction_hex)
        .map_err(|error| RpcFailure::invalid_params(error.to_string()))?;
    let transaction = parse_transaction(&transaction_bytes)
        .map_err(|error| RpcFailure::invalid_params(error.to_string()))?;
    let result = context
        .submit_local_transaction(transaction)
        .map_err(network_error_to_failure)?;

    Ok(SendRawTransactionResponse {
        txid_hex: decode::encode_hex(result.accepted.as_bytes()),
        replaced_txids: result
            .replaced
            .into_iter()
            .map(|txid| decode::encode_hex(txid.as_bytes()))
            .collect(),
        evicted_txids: result
            .evicted
            .into_iter()
            .map(|txid| decode::encode_hex(txid.as_bytes()))
            .collect(),
    })
}
