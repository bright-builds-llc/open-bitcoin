use serde::{Deserialize, Serialize};

use super::node::DescriptorRange;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EstimateMode {
    Unset,
    Economical,
    Conservative,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SendToAddressRequest {
    pub address: String,
    pub amount_sats: i64,
    #[serde(rename = "fee_rate_sat_per_kvb", default)]
    pub maybe_fee_rate_sat_per_kvb: Option<i64>,
    #[serde(rename = "conf_target", default)]
    pub maybe_conf_target: Option<u16>,
    #[serde(rename = "estimate_mode", default)]
    pub maybe_estimate_mode: Option<EstimateMode>,
    #[serde(rename = "change_descriptor_id", default)]
    pub maybe_change_descriptor_id: Option<u32>,
    #[serde(rename = "lock_time", default)]
    pub maybe_lock_time: Option<u32>,
    #[serde(rename = "replaceable", default)]
    pub enable_rbf: bool,
    #[serde(rename = "max_tx_fee_sats", default)]
    pub maybe_max_tx_fee_sats: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GetNewAddressRequest {}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GetRawChangeAddressRequest {}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ListDescriptorsRequest {}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ListDescriptorEntry {
    #[serde(rename = "desc")]
    pub descriptor: String,
    pub active: bool,
    pub internal: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maybe_range: Option<DescriptorRange>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maybe_next_index: Option<u32>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ListDescriptorsResponse {
    #[serde(rename = "walletname", skip_serializing_if = "Option::is_none")]
    pub maybe_wallet_name: Option<String>,
    pub descriptors: Vec<ListDescriptorEntry>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum WalletFreshness {
    Fresh,
    Partial,
    Scanning,
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
    pub scanning: bool,
    pub freshness: WalletFreshness,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maybe_scanned_through_height: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maybe_rescan_next_height: Option<u32>,
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
