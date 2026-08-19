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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TestMempoolAcceptRequest {
    #[serde(rename = "rawtxs")]
    pub raw_txs: Vec<String>,
    #[serde(rename = "maxfeerate", default)]
    pub maybe_max_fee_rate: Option<i64>,
    #[serde(rename = "maxburnamount", default)]
    pub maybe_max_burn_amount: Option<i64>,
    #[serde(default)]
    pub ignore_rejects: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SubmitPackageRequest {
    pub package: Vec<String>,
    #[serde(rename = "maxfeerate", default)]
    pub maybe_max_fee_rate: Option<i64>,
    #[serde(rename = "maxburnamount", default)]
    pub maybe_max_burn_amount: Option<i64>,
    #[serde(default)]
    pub ignore_rejects: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OpenBitcoinPackageMode {
    #[serde(rename = "dry-run")]
    DryRun,
    #[serde(rename = "submit")]
    Submit,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OpenBitcoinPackageRequest {
    pub mode: OpenBitcoinPackageMode,
    #[serde(rename = "rawtxs")]
    pub raw_txs: Vec<String>,
}
