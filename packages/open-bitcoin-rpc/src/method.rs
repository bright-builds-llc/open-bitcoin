use serde::{Deserialize, Serialize};

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
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GetBlockchainInfoRequest {}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GetBlockchainInfoResponse {
    pub blocks: u32,
    pub headers: u32,
    pub maybe_best_block_hash: Option<String>,
    pub maybe_median_time_past: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GetMempoolInfoRequest {}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GetMempoolInfoResponse {
    pub transaction_count: usize,
    pub total_virtual_size: usize,
    pub total_fee_sats: i64,
    pub min_relay_feerate_sats_per_kvb: i64,
    pub incremental_relay_feerate_sats_per_kvb: i64,
    pub max_mempool_virtual_size: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GetNetworkInfoRequest {}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GetNetworkInfoResponse {
    pub protocol_version: i32,
    pub subversion: String,
    pub local_services_bits: u64,
    pub relay: bool,
    pub connections: usize,
    pub inbound_connections: usize,
    pub outbound_connections: usize,
    pub wtxidrelay_connections: usize,
    pub header_preferring_connections: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SendRawTransactionRequest {
    pub transaction_hex: String,
    pub maybe_max_fee_rate_sat_per_kvb: Option<i64>,
    pub maybe_max_burn_amount_sats: Option<i64>,
    pub ignore_rejects: bool,
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
    pub maybe_range: Option<DescriptorRange>,
    pub require_checksum: bool,
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
    pub maybe_minimum_amount_sats: Option<i64>,
    pub maybe_maximum_amount_sats: Option<i64>,
    pub maybe_maximum_count: Option<usize>,
    pub maybe_minimum_sum_amount_sats: Option<i64>,
    pub include_immature_coinbase: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ListUnspentRequest {
    pub maybe_min_confirmations: Option<u32>,
    pub maybe_max_confirmations: Option<u32>,
    pub addresses: Vec<String>,
    pub include_unsafe: bool,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ImportDescriptorRole {
    External,
    Internal,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DescriptorImportItem {
    pub descriptor: String,
    pub label: String,
    pub role: ImportDescriptorRole,
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
    pub maybe_start_height: Option<u32>,
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
    pub maybe_change_descriptor_id: Option<u32>,
    pub maybe_lock_time: Option<u32>,
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
    pub maybe_change_descriptor_id: Option<u32>,
    pub maybe_lock_time: Option<u32>,
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
