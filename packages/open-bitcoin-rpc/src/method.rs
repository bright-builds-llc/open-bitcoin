use serde::{Deserialize, Serialize, de::DeserializeOwned};
use serde_json::{Map, Value};

use crate::error::RpcFailure;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MethodOrigin {
    BaselineParity,
    OpenBitcoinExtension,
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
    GetWalletInfo(GetWalletInfoRequest),
    GetBalances(GetBalancesRequest),
    ListUnspent(ListUnspentRequest),
    ImportDescriptors(ImportDescriptorsRequest),
    RescanBlockchain(RescanBlockchainRequest),
    BuildTransaction(BuildTransactionRequest),
    BuildAndSignTransaction(BuildAndSignTransactionRequest),
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
            normalize_request::<GetBlockchainInfoRequest>(&[], params)
                .map(MethodCall::GetBlockchainInfo)
        }
        SupportedMethod::GetMempoolInfo => {
            normalize_request::<GetMempoolInfoRequest>(&[], params).map(MethodCall::GetMempoolInfo)
        }
        SupportedMethod::GetNetworkInfo => {
            normalize_request::<GetNetworkInfoRequest>(&[], params).map(MethodCall::GetNetworkInfo)
        }
        SupportedMethod::SendRawTransaction => normalize_request::<SendRawTransactionRequest>(
            &["hexstring", "maxfeerate", "maxburnamount", "ignore_rejects"],
            params,
        )
        .map(MethodCall::SendRawTransaction),
        SupportedMethod::DeriveAddresses => {
            let request =
                normalize_request::<DeriveAddressesRequest>(&["descriptor", "range"], params)?;
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
        SupportedMethod::GetWalletInfo => {
            normalize_request::<GetWalletInfoRequest>(&[], params).map(MethodCall::GetWalletInfo)
        }
        SupportedMethod::GetBalances => {
            normalize_request::<GetBalancesRequest>(&[], params).map(MethodCall::GetBalances)
        }
        SupportedMethod::ListUnspent => normalize_request::<ListUnspentRequest>(
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
            normalize_request::<ImportDescriptorsRequest>(&["requests"], params)
                .map(MethodCall::ImportDescriptors)
        }
        SupportedMethod::RescanBlockchain => {
            normalize_request::<RescanBlockchainRequest>(&["start_height", "stop_height"], params)
                .map(MethodCall::RescanBlockchain)
        }
        SupportedMethod::BuildTransaction => normalize_request::<BuildTransactionRequest>(
            &[
                "recipients",
                "fee_rate_sat_per_kvb",
                "change_descriptor_id",
                "lock_time",
                "replaceable",
            ],
            params,
        )
        .map(MethodCall::BuildTransaction),
        SupportedMethod::BuildAndSignTransaction => {
            normalize_request::<BuildAndSignTransactionRequest>(
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

fn normalize_request<T: DeserializeOwned>(
    ordered_fields: &[&str],
    params: RequestParameters,
) -> Result<T, RpcFailure> {
    let (positional, named) = match params {
        RequestParameters::None => (Vec::new(), Vec::new()),
        RequestParameters::Positional(values) => (values, Vec::new()),
        RequestParameters::Named(values) => (Vec::new(), values),
        RequestParameters::Mixed { positional, named } => (positional, named),
    };

    if positional.len() > ordered_fields.len() {
        return Err(RpcFailure::invalid_params(format!(
            "too many positional parameters: expected at most {}, got {}",
            ordered_fields.len(),
            positional.len()
        )));
    }

    let positional_fields = ordered_fields
        .iter()
        .take(positional.len())
        .copied()
        .collect::<Vec<_>>();
    let mut object = Map::new();
    for (index, value) in positional.into_iter().enumerate() {
        object.insert(ordered_fields[index].to_string(), value);
    }

    for (name, value) in named {
        if !ordered_fields.iter().any(|allowed| *allowed == name) {
            return Err(RpcFailure::invalid_params(format!(
                "unknown named parameter {name}"
            )));
        }
        if object.contains_key(&name) {
            if positional_fields.iter().any(|field| *field == name) {
                return Err(RpcFailure::invalid_params(format!(
                    "named parameter {name} collides with a positional argument"
                )));
            }
            return Err(RpcFailure::invalid_params(format!(
                "named parameter {name} was provided multiple times"
            )));
        }
        object.insert(name, value);
    }

    serde_json::from_value(Value::Object(object))
        .map_err(|error| RpcFailure::invalid_params(error.to_string()))
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GetBlockchainInfoRequest {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetBlockchainInfoResponse {
    pub chain: String,
    pub blocks: u32,
    pub headers: u32,
    #[serde(rename = "bestblockhash", skip_serializing_if = "Option::is_none")]
    pub maybe_best_block_hash: Option<String>,
    #[serde(rename = "mediantime", skip_serializing_if = "Option::is_none")]
    pub maybe_median_time_past: Option<i64>,
    pub verificationprogress: f64,
    pub initialblockdownload: bool,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GetMempoolInfoRequest {}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GetMempoolInfoResponse {
    pub size: usize,
    pub bytes: usize,
    pub usage: usize,
    pub total_fee_sats: i64,
    pub maxmempool: usize,
    pub mempoolminfee: i64,
    pub minrelaytxfee: i64,
    pub loaded: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GetNetworkInfoRequest {}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GetNetworkInfoResponse {
    pub version: i32,
    pub subversion: String,
    pub protocolversion: i32,
    pub localservices: String,
    pub localrelay: bool,
    pub connections: usize,
    #[serde(rename = "connections_in")]
    pub connections_in: usize,
    #[serde(rename = "connections_out")]
    pub connections_out: usize,
    pub relayfee: i64,
    pub incrementalfee: i64,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SendRawTransactionRequest {
    #[serde(rename = "hexstring")]
    pub transaction_hex: String,
    #[serde(rename = "maxfeerate", default)]
    pub maybe_max_fee_rate_sat_per_kvb: Option<i64>,
    #[serde(rename = "maxburnamount", default)]
    pub maybe_max_burn_amount_sats: Option<i64>,
    #[serde(default)]
    pub ignore_rejects: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SendRawTransactionResponse {
    pub txid_hex: String,
    pub replaced_txids: Vec<String>,
    pub evicted_txids: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DescriptorRange {
    pub start: u32,
    pub end: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DeriveAddressesRequest {
    pub descriptor: String,
    #[serde(rename = "range", default)]
    pub maybe_range: Option<DescriptorRange>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeriveAddressesResponse {
    pub addresses: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GetWalletInfoRequest {}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GetWalletInfoResponse {
    pub network: String,
    pub descriptor_count: usize,
    pub utxo_count: usize,
    pub maybe_tip_height: Option<u32>,
    pub maybe_tip_median_time_past: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GetBalancesRequest {}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WalletBalanceDetails {
    pub trusted_sats: i64,
    pub untrusted_pending_sats: i64,
    pub immature_sats: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GetBalancesResponse {
    pub mine: WalletBalanceDetails,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ListUnspentQuery {
    #[serde(rename = "minimumAmount", default)]
    pub maybe_minimum_amount_sats: Option<i64>,
    #[serde(rename = "maximumAmount", default)]
    pub maybe_maximum_amount_sats: Option<i64>,
    #[serde(rename = "maximumCount", default)]
    pub maybe_maximum_count: Option<usize>,
    #[serde(rename = "minimumSumAmount", default)]
    pub maybe_minimum_sum_amount_sats: Option<i64>,
    #[serde(rename = "include_immature_coinbase", default)]
    pub include_immature_coinbase: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ListUnspentRequest {
    #[serde(rename = "minconf", default)]
    pub maybe_min_confirmations: Option<u32>,
    #[serde(rename = "maxconf", default)]
    pub maybe_max_confirmations: Option<u32>,
    #[serde(default)]
    pub addresses: Vec<String>,
    #[serde(default)]
    pub include_unsafe: bool,
    #[serde(rename = "query_options", default)]
    pub query: ListUnspentQuery,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ListUnspentEntry {
    pub txid_hex: String,
    pub vout: u32,
    pub amount_sats: i64,
    pub descriptor_id: u32,
    pub is_coinbase: bool,
    pub confirmations: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ListUnspentResponse {
    pub entries: Vec<ListUnspentEntry>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DescriptorImportItem {
    #[serde(rename = "desc")]
    pub descriptor: String,
    #[serde(default)]
    pub label: String,
    #[serde(default)]
    pub internal: bool,
    #[serde(rename = "timestamp", default)]
    pub maybe_rescan_since_height: Option<u32>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ImportDescriptorsRequest {
    pub requests: Vec<DescriptorImportItem>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DescriptorImportResult {
    pub success: bool,
    pub maybe_descriptor_id: Option<u32>,
    pub maybe_message: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImportDescriptorsResponse {
    pub results: Vec<DescriptorImportResult>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RescanBlockchainRequest {
    #[serde(rename = "start_height", default)]
    pub maybe_start_height: Option<u32>,
    #[serde(rename = "stop_height", default)]
    pub maybe_stop_height: Option<u32>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RescanBlockchainResponse {
    pub start_height: u32,
    pub stop_height: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransactionRecipient {
    pub script_pubkey_hex: String,
    pub amount_sats: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SelectedInput {
    pub txid_hex: String,
    pub vout: u32,
    pub descriptor_id: u32,
    pub amount_sats: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BuildTransactionRequest {
    pub recipients: Vec<TransactionRecipient>,
    pub fee_rate_sat_per_kvb: i64,
    #[serde(rename = "change_descriptor_id", default)]
    pub maybe_change_descriptor_id: Option<u32>,
    #[serde(rename = "lock_time", default)]
    pub maybe_lock_time: Option<u32>,
    #[serde(rename = "replaceable", default)]
    pub enable_rbf: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BuildTransactionResponse {
    pub transaction_hex: String,
    pub fee_sats: i64,
    pub inputs: Vec<SelectedInput>,
    pub maybe_change_output_index: Option<usize>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BuildAndSignTransactionRequest {
    pub recipients: Vec<TransactionRecipient>,
    pub fee_rate_sat_per_kvb: i64,
    #[serde(rename = "change_descriptor_id", default)]
    pub maybe_change_descriptor_id: Option<u32>,
    #[serde(rename = "lock_time", default)]
    pub maybe_lock_time: Option<u32>,
    #[serde(rename = "replaceable", default)]
    pub enable_rbf: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BuildAndSignTransactionResponse {
    pub transaction_hex: String,
    pub fee_sats: i64,
    pub inputs: Vec<SelectedInput>,
    pub maybe_change_output_index: Option<usize>,
}

#[cfg(test)]
mod tests;
