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

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::error::RpcFailure;

mod node;
mod normalize;
#[cfg(test)]
mod tests;
mod wallet;

pub use node::*;
pub use wallet::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MethodOrigin {
    BaselineParity,
    OpenBitcoinExtension,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MethodScope {
    Node,
    Wallet,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SupportedMethod {
    #[serde(rename = "getblockchaininfo")]
    GetBlockchainInfo,
    #[serde(rename = "getmempoolinfo")]
    GetMempoolInfo,
    #[serde(rename = "getnetworkinfo")]
    GetNetworkInfo,
    #[serde(rename = "sendrawtransaction")]
    SendRawTransaction,
    #[serde(rename = "deriveaddresses")]
    DeriveAddresses,
    #[serde(rename = "sendtoaddress")]
    SendToAddress,
    #[serde(rename = "getnewaddress")]
    GetNewAddress,
    #[serde(rename = "getrawchangeaddress")]
    GetRawChangeAddress,
    #[serde(rename = "listdescriptors")]
    ListDescriptors,
    #[serde(rename = "getwalletinfo")]
    GetWalletInfo,
    #[serde(rename = "getbalances")]
    GetBalances,
    #[serde(rename = "listunspent")]
    ListUnspent,
    #[serde(rename = "importdescriptors")]
    ImportDescriptors,
    #[serde(rename = "rescanblockchain")]
    RescanBlockchain,
    #[serde(rename = "buildtransaction")]
    BuildTransaction,
    #[serde(rename = "buildandsigntransaction")]
    BuildAndSignTransaction,
}

impl SupportedMethod {
    pub const fn all() -> &'static [Self] {
        &[
            Self::GetBlockchainInfo,
            Self::GetMempoolInfo,
            Self::GetNetworkInfo,
            Self::SendRawTransaction,
            Self::DeriveAddresses,
            Self::SendToAddress,
            Self::GetNewAddress,
            Self::GetRawChangeAddress,
            Self::ListDescriptors,
            Self::GetWalletInfo,
            Self::GetBalances,
            Self::ListUnspent,
            Self::ImportDescriptors,
            Self::RescanBlockchain,
            Self::BuildTransaction,
            Self::BuildAndSignTransaction,
        ]
    }

    pub const fn name(self) -> &'static str {
        match self {
            Self::GetBlockchainInfo => "getblockchaininfo",
            Self::GetMempoolInfo => "getmempoolinfo",
            Self::GetNetworkInfo => "getnetworkinfo",
            Self::SendRawTransaction => "sendrawtransaction",
            Self::DeriveAddresses => "deriveaddresses",
            Self::SendToAddress => "sendtoaddress",
            Self::GetNewAddress => "getnewaddress",
            Self::GetRawChangeAddress => "getrawchangeaddress",
            Self::ListDescriptors => "listdescriptors",
            Self::GetWalletInfo => "getwalletinfo",
            Self::GetBalances => "getbalances",
            Self::ListUnspent => "listunspent",
            Self::ImportDescriptors => "importdescriptors",
            Self::RescanBlockchain => "rescanblockchain",
            Self::BuildTransaction => "buildtransaction",
            Self::BuildAndSignTransaction => "buildandsigntransaction",
        }
    }

    pub const fn origin(self) -> MethodOrigin {
        match self {
            Self::BuildTransaction | Self::BuildAndSignTransaction => {
                MethodOrigin::OpenBitcoinExtension
            }
            _ => MethodOrigin::BaselineParity,
        }
    }

    pub const fn scope(self) -> MethodScope {
        match self {
            Self::GetWalletInfo
            | Self::SendToAddress
            | Self::GetNewAddress
            | Self::GetRawChangeAddress
            | Self::ListDescriptors
            | Self::GetBalances
            | Self::ListUnspent
            | Self::ImportDescriptors
            | Self::RescanBlockchain
            | Self::BuildTransaction
            | Self::BuildAndSignTransaction => MethodScope::Wallet,
            Self::GetBlockchainInfo
            | Self::GetMempoolInfo
            | Self::GetNetworkInfo
            | Self::SendRawTransaction
            | Self::DeriveAddresses => MethodScope::Node,
        }
    }

    pub fn from_name(name: &str) -> Option<Self> {
        Self::all()
            .iter()
            .copied()
            .find(|method| method.name() == name)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum RequestParameters {
    None,
    Positional(Vec<Value>),
    Named(Vec<(String, Value)>),
    Mixed {
        positional: Vec<Value>,
        named: Vec<(String, Value)>,
    },
}

impl RequestParameters {
    pub fn from_json(params: Value) -> Self {
        match params {
            Value::Null => Self::None,
            Value::Array(values) => Self::Positional(values),
            Value::Object(values) => Self::Named(values.into_iter().collect()),
            value => Self::Positional(vec![value]),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum MethodCall {
    GetBlockchainInfo(GetBlockchainInfoRequest),
    GetMempoolInfo(GetMempoolInfoRequest),
    GetNetworkInfo(GetNetworkInfoRequest),
    SendRawTransaction(SendRawTransactionRequest),
    DeriveAddresses(DeriveAddressesRequest),
    SendToAddress(SendToAddressRequest),
    GetNewAddress(GetNewAddressRequest),
    GetRawChangeAddress(GetRawChangeAddressRequest),
    ListDescriptors(ListDescriptorsRequest),
    GetWalletInfo(GetWalletInfoRequest),
    GetBalances(GetBalancesRequest),
    ListUnspent(ListUnspentRequest),
    ImportDescriptors(ImportDescriptorsRequest),
    RescanBlockchain(RescanBlockchainRequest),
    BuildTransaction(BuildTransactionRequest),
    BuildAndSignTransaction(BuildAndSignTransactionRequest),
}

impl MethodCall {
    pub const fn scope(&self) -> MethodScope {
        match self {
            Self::GetWalletInfo(_)
            | Self::SendToAddress(_)
            | Self::GetNewAddress(_)
            | Self::GetRawChangeAddress(_)
            | Self::ListDescriptors(_)
            | Self::GetBalances(_)
            | Self::ListUnspent(_)
            | Self::ImportDescriptors(_)
            | Self::RescanBlockchain(_)
            | Self::BuildTransaction(_)
            | Self::BuildAndSignTransaction(_) => MethodScope::Wallet,
            Self::GetBlockchainInfo(_)
            | Self::GetMempoolInfo(_)
            | Self::GetNetworkInfo(_)
            | Self::SendRawTransaction(_)
            | Self::DeriveAddresses(_) => MethodScope::Node,
        }
    }
}

pub fn normalize_method_call(
    method_name: &str,
    params: RequestParameters,
) -> Result<MethodCall, RpcFailure> {
    let Some(method) = SupportedMethod::from_name(method_name) else {
        return Err(RpcFailure::method_not_found(method_name));
    };

    match method {
        SupportedMethod::GetBlockchainInfo => {
            normalize::normalize_request::<GetBlockchainInfoRequest>(&[], params)
                .map(MethodCall::GetBlockchainInfo)
        }
        SupportedMethod::GetMempoolInfo => {
            normalize::normalize_request::<GetMempoolInfoRequest>(&[], params)
                .map(MethodCall::GetMempoolInfo)
        }
        SupportedMethod::GetNetworkInfo => {
            normalize::normalize_request::<GetNetworkInfoRequest>(&[], params)
                .map(MethodCall::GetNetworkInfo)
        }
        SupportedMethod::SendRawTransaction => {
            normalize::normalize_request::<SendRawTransactionRequest>(
                &["hexstring", "maxfeerate", "maxburnamount", "ignore_rejects"],
                params,
            )
            .map(MethodCall::SendRawTransaction)
        }
        SupportedMethod::DeriveAddresses => {
            let request = normalize::normalize_request::<DeriveAddressesRequest>(
                &["descriptor", "range"],
                params,
            )?;
            if !request.descriptor.contains('#') {
                return Err(RpcFailure::invalid_params(
                    "deriveaddresses requires a checksum-qualified descriptor",
                ));
            }
            if request.maybe_range.is_some() {
                return Err(RpcFailure::invalid_params(
                    "ranged descriptors are deferred to later wallet phases",
                ));
            }
            Ok(MethodCall::DeriveAddresses(request))
        }
        SupportedMethod::SendToAddress => normalize::normalize_request::<SendToAddressRequest>(
            &[
                "address",
                "amount_sats",
                "fee_rate_sat_per_kvb",
                "conf_target",
                "estimate_mode",
                "change_descriptor_id",
                "lock_time",
                "replaceable",
                "max_tx_fee_sats",
            ],
            params,
        )
        .map(MethodCall::SendToAddress),
        SupportedMethod::GetNewAddress => {
            normalize::normalize_request::<GetNewAddressRequest>(&[], params)
                .map(MethodCall::GetNewAddress)
        }
        SupportedMethod::GetRawChangeAddress => {
            normalize::normalize_request::<GetRawChangeAddressRequest>(&[], params)
                .map(MethodCall::GetRawChangeAddress)
        }
        SupportedMethod::ListDescriptors => {
            normalize::normalize_request::<ListDescriptorsRequest>(&[], params)
                .map(MethodCall::ListDescriptors)
        }
        SupportedMethod::GetWalletInfo => {
            normalize::normalize_request::<GetWalletInfoRequest>(&[], params)
                .map(MethodCall::GetWalletInfo)
        }
        SupportedMethod::GetBalances => {
            normalize::normalize_request::<GetBalancesRequest>(&[], params)
                .map(MethodCall::GetBalances)
        }
        SupportedMethod::ListUnspent => normalize::normalize_request::<ListUnspentRequest>(
            &[
                "minconf",
                "maxconf",
                "addresses",
                "include_unsafe",
                "query_options",
            ],
            params,
        )
        .map(MethodCall::ListUnspent),
        SupportedMethod::ImportDescriptors => {
            normalize::normalize_request::<ImportDescriptorsRequest>(&["requests"], params)
                .map(MethodCall::ImportDescriptors)
        }
        SupportedMethod::RescanBlockchain => {
            normalize::normalize_request::<RescanBlockchainRequest>(
                &["start_height", "stop_height"],
                params,
            )
            .map(MethodCall::RescanBlockchain)
        }
        SupportedMethod::BuildTransaction => {
            normalize::normalize_request::<BuildTransactionRequest>(
                &[
                    "recipients",
                    "fee_rate_sat_per_kvb",
                    "change_descriptor_id",
                    "lock_time",
                    "replaceable",
                ],
                params,
            )
            .map(MethodCall::BuildTransaction)
        }
        SupportedMethod::BuildAndSignTransaction => {
            normalize::normalize_request::<BuildAndSignTransactionRequest>(
                &[
                    "recipients",
                    "fee_rate_sat_per_kvb",
                    "change_descriptor_id",
                    "lock_time",
                    "replaceable",
                ],
                params,
            )
            .map(MethodCall::BuildAndSignTransaction)
        }
    }
}
