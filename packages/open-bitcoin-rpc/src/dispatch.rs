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
    codec::{TransactionEncoding, encode_transaction},
    wallet::WalletError,
};

use crate::{
    ManagedRpcContext,
    error::{RpcErrorCode, RpcErrorDetail, RpcFailure},
    method::{BuildTransactionResponse, MethodCall, SelectedInput, WalletFreshness},
};

mod decode;
mod node;
#[cfg(test)]
mod tests;
mod wallet;

pub fn dispatch(context: &mut ManagedRpcContext, call: MethodCall) -> Result<Value, RpcFailure> {
    match call {
        MethodCall::GetBlockchainInfo(_request) => {
            serde_json::to_value(node::get_blockchain_info(context))
                .map_err(|error| RpcFailure::internal_error(error.to_string()))
        }
        MethodCall::GetMempoolInfo(_request) => {
            serde_json::to_value(node::get_mempool_info(context))
                .map_err(|error| RpcFailure::internal_error(error.to_string()))
        }
        MethodCall::GetNetworkInfo(_request) => {
            serde_json::to_value(node::get_network_info(context))
                .map_err(|error| RpcFailure::internal_error(error.to_string()))
        }
        MethodCall::SendRawTransaction(request) => {
            serde_json::to_value(node::send_raw_transaction(context, request)?)
                .map_err(|error| RpcFailure::internal_error(error.to_string()))
        }
        MethodCall::DeriveAddresses(request) => {
            serde_json::to_value(node::derive_addresses(context, request)?)
                .map_err(|error| RpcFailure::internal_error(error.to_string()))
        }
        MethodCall::SendToAddress(request) => {
            serde_json::to_value(wallet::send_to_address(context, request)?)
                .map_err(|error| RpcFailure::internal_error(error.to_string()))
        }
        MethodCall::GetNewAddress(_request) => {
            serde_json::to_value(wallet::get_new_address(context)?)
                .map_err(|error| RpcFailure::internal_error(error.to_string()))
        }
        MethodCall::GetRawChangeAddress(_request) => {
            serde_json::to_value(wallet::get_raw_change_address(context)?)
                .map_err(|error| RpcFailure::internal_error(error.to_string()))
        }
        MethodCall::ListDescriptors(_request) => {
            serde_json::to_value(wallet::list_descriptors(context)?)
                .map_err(|error| RpcFailure::internal_error(error.to_string()))
        }
        MethodCall::GetWalletInfo(_request) => wallet::get_wallet_info(context),
        MethodCall::GetBalances(_request) => serde_json::to_value(wallet::get_balances(context)?)
            .map_err(|error| RpcFailure::internal_error(error.to_string())),
        MethodCall::ListUnspent(request) => {
            serde_json::to_value(wallet::list_unspent(context, request)?)
                .map_err(|error| RpcFailure::internal_error(error.to_string()))
        }
        MethodCall::ImportDescriptors(request) => {
            serde_json::to_value(wallet::import_descriptors(context, request)?)
                .map_err(|error| RpcFailure::internal_error(error.to_string()))
        }
        MethodCall::RescanBlockchain(request) => {
            serde_json::to_value(wallet::rescan_blockchain(context, request)?)
                .map_err(|error| RpcFailure::internal_error(error.to_string()))
        }
        MethodCall::BuildTransaction(request) => {
            serde_json::to_value(wallet::build_transaction(context, request)?)
                .map_err(|error| RpcFailure::internal_error(error.to_string()))
        }
        MethodCall::BuildAndSignTransaction(request) => {
            serde_json::to_value(wallet::build_and_sign_transaction(context, request)?)
                .map_err(|error| RpcFailure::internal_error(error.to_string()))
        }
    }
}

pub(super) fn map_wallet_freshness(
    freshness: crate::context::WalletFreshnessKind,
) -> WalletFreshness {
    match freshness {
        crate::context::WalletFreshnessKind::Fresh => WalletFreshness::Fresh,
        crate::context::WalletFreshnessKind::Partial => WalletFreshness::Partial,
        crate::context::WalletFreshnessKind::Scanning => WalletFreshness::Scanning,
    }
}

pub(super) fn map_built_transaction(
    built: &open_bitcoin_node::core::wallet::BuiltTransaction,
) -> Result<BuildTransactionResponse, RpcFailure> {
    let transaction_hex = decode::encode_hex(
        &encode_transaction(&built.transaction, TransactionEncoding::WithWitness)
            .map_err(|error| RpcFailure::internal_error(error.to_string()))?,
    );
    let inputs = built
        .selected_inputs
        .iter()
        .map(|utxo| SelectedInput {
            txid_hex: decode::encode_hex(utxo.outpoint.txid.as_bytes()),
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

pub(super) fn wallet_error_to_failure(error: WalletError) -> RpcFailure {
    RpcFailure::wallet_error(error.to_string())
}

pub(super) fn rpc_failure_message(failure: &RpcFailure) -> String {
    failure.maybe_detail.as_ref().map_or_else(
        || "Internal error".to_string(),
        |detail| detail.message.clone(),
    )
}

pub(super) fn network_error_to_failure(error: ManagedNetworkError) -> RpcFailure {
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

pub(super) fn version_number() -> i32 {
    let mut parts = env!("CARGO_PKG_VERSION")
        .split('.')
        .map(|part| part.parse::<i32>().unwrap_or(0));
    let major = parts.next().unwrap_or(0);
    let minor = parts.next().unwrap_or(0);
    let patch = parts.next().unwrap_or(0);
    major * 10_000 + minor * 100 + patch
}
