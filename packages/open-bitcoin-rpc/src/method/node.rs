use serde::{Deserialize, Serialize};

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
