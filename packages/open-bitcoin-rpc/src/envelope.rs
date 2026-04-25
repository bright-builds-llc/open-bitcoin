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

use crate::error::RpcErrorDetail;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum JsonRpcVersion {
    #[serde(rename = "2.0")]
    V2,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum JsonRpcId {
    Null,
    Number(i64),
    String(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RpcRequestEnvelope<T> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jsonrpc: Option<JsonRpcVersion>,
    pub method: String,
    pub params: T,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<JsonRpcId>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RpcSuccessEnvelope<T> {
    pub jsonrpc: JsonRpcVersion,
    pub result: T,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<JsonRpcId>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RpcErrorEnvelope {
    pub jsonrpc: JsonRpcVersion,
    pub error: RpcErrorDetail,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<JsonRpcId>,
}

impl<T> RpcSuccessEnvelope<T> {
    pub fn new(result: T, id: Option<JsonRpcId>) -> Self {
        Self {
            jsonrpc: JsonRpcVersion::V2,
            result,
            id,
        }
    }
}

impl RpcErrorEnvelope {
    pub fn new(error: RpcErrorDetail, id: Option<JsonRpcId>) -> Self {
        Self {
            jsonrpc: JsonRpcVersion::V2,
            error,
            id,
        }
    }
}

#[cfg(test)]
mod tests;
