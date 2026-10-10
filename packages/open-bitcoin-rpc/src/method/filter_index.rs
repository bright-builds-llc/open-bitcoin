// Parity breadcrumbs:
// - packages/bitcoin-knots/src/rpc/blockchain.cpp
// - packages/bitcoin-knots/src/rpc/node.cpp
// - packages/bitcoin-knots/src/rpc/util.cpp
// - packages/bitcoin-knots/src/blockfilter.cpp
// - packages/bitcoin-knots/src/index/blockfilterindex.cpp
// - packages/bitcoin-knots/src/uint256.h

use std::collections::BTreeMap;

use open_bitcoin_node::core::primitives::BlockHash;
use serde::Serialize;

use crate::error::{RpcErrorCode, RpcErrorDetail, RpcFailure, RpcFailureKind};

mod normalize;
pub use normalize::{normalize_getblockfilter, normalize_getindexinfo};

#[cfg(test)]
mod tests;

/// Exact pinned name exposed by the BASIC block filter index.
pub const BASIC_FILTER_INDEX_NAME: &str = "basic block filter index";

/// Complete source-derived help from the pinned RPCHelpMan contract in rpc/util.cpp.
pub const GETBLOCKFILTER_HELP: &str = concat!(
    "getblockfilter \"blockhash\" ( \"filtertype\" )\n\n",
    "Retrieve a BIP 157 content filter for a particular block.\n\nArguments:\n",
    "1. blockhash     (string, required) The hash of the block\n",
    "2. filtertype    (string, optional, default=\"basic\") The type name of the filter, values: basic, v0\n",
    "\nResult:\n",
    "{                      (json object)\n",
    "  \"filter\" : \"hex\",    (string) the hex-encoded filter data\n",
    "  \"header\" : \"hex\"     (string) the hex-encoded filter header\n",
    "}\n\nExamples:\n",
    "> bitcoin-cli getblockfilter \"00000000c937983704a73af28acdec37b049d214adbda81d7e2a3dd146f6ed09\" \"basic\"\n",
    "> curl --user myusername --data-binary '{\"jsonrpc\": \"2.0\", \"id\": \"curltest\", \"method\": \"getblockfilter\", \"params\": [\"00000000c937983704a73af28acdec37b049d214adbda81d7e2a3dd146f6ed09\", \"basic\"]}' -H 'content-type: application/json' http://127.0.0.1:8332/\n",
);

/// Complete help, including the pinned unquoted txindex RPC example.
pub const GETINDEXINFO_HELP: &str = concat!(
    "getindexinfo ( \"index_name\" )\n\n",
    "Returns the status of one or all available indices currently running in the node.\n\nArguments:\n",
    "1. index_name    (string, optional) Filter results for an index with a specific name.\n",
    "\nResult:\n",
    "{                               (json object)\n",
    "  \"name\" : {                    (json object) The name of the index\n",
    "    \"synced\" : true|false,      (boolean) Whether the index is synced or not\n",
    "    \"best_block_height\" : n     (numeric) The block height to which the index is synced\n",
    "  },\n  ...\n}\n\nExamples:\n",
    "> bitcoin-cli getindexinfo \n",
    "> curl --user myusername --data-binary '{\"jsonrpc\": \"2.0\", \"id\": \"curltest\", \"method\": \"getindexinfo\", \"params\": []}' -H 'content-type: application/json' http://127.0.0.1:8332/\n",
    "> bitcoin-cli getindexinfo txindex\n",
    "> curl --user myusername --data-binary '{\"jsonrpc\": \"2.0\", \"id\": \"curltest\", \"method\": \"getindexinfo\", \"params\": [txindex]}' -H 'content-type: application/json' http://127.0.0.1:8332/\n",
);

/// Recognized pinned filter names; V0 selection never grants serving support.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlockFilterSelection {
    Basic,
    V0,
}

impl BlockFilterSelection {
    /// Resolve the case-sensitive Knots filter name after hash validation.
    pub fn parse(name: &str) -> Result<Self, RpcFailure> {
        match name {
            "basic" => Ok(Self::Basic),
            "v0" => Ok(Self::V0),
            _ => Err(filter_failure(
                RpcErrorCode::InvalidAddressOrKey,
                "Unknown filtertype",
            )),
        }
    }

    /// Return the pinned error for a recognized index that is not enabled.
    pub fn disabled_failure(self) -> RpcFailure {
        let name = match self {
            Self::Basic => "basic",
            Self::V0 => "v0",
        };
        filter_failure(
            RpcErrorCode::MiscError,
            format!("Index is not enabled for filtertype {name}"),
        )
    }
}

/// Fully parsed getblockfilter arguments, independent of index availability.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetBlockFilterRequest {
    pub block_hash: BlockHash,
    pub filter_type: BlockFilterSelection,
}

/// getindexinfo's optional, case-sensitive index selector.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetIndexInfoRequest {
    pub maybe_index_name: Option<String>,
}

/// The two allowlisted fields of a successful getblockfilter response.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct GetBlockFilterResult {
    pub filter: String,
    pub header: String,
}

impl GetBlockFilterResult {
    /// Encode filter bytes directly and raw uint256 header bytes in display order.
    pub fn from_encoded(encoded_filter: &[u8], raw_header: [u8; 32]) -> Self {
        Self {
            filter: lowercase_hex(encoded_filter.iter().copied()),
            header: lowercase_hex(raw_header.into_iter().rev()),
        }
    }
}

/// Pinned summary fields; readiness is supplied by the shared node authority.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct BasicFilterIndexSummary {
    pub synced: bool,
    pub best_block_height: u32,
}

impl GetIndexInfoRequest {
    /// Project only an enabled BASIC index matching the pinned selector rules.
    pub fn project_basic(
        &self,
        maybe_summary: Option<BasicFilterIndexSummary>,
    ) -> BTreeMap<&'static str, BasicFilterIndexSummary> {
        let Some(summary) = maybe_summary else {
            return BTreeMap::new();
        };
        if self
            .maybe_index_name
            .as_deref()
            .is_some_and(|name| !name.is_empty() && name != BASIC_FILTER_INDEX_NAME)
        {
            return BTreeMap::new();
        }
        BTreeMap::from([(BASIC_FILTER_INDEX_NAME, summary)])
    }
}

fn lowercase_hex(bytes: impl Iterator<Item = u8>) -> String {
    const DIGITS: &[u8; 16] = b"0123456789abcdef";
    bytes
        .flat_map(|byte| {
            [
                char::from(DIGITS[usize::from(byte >> 4)]),
                char::from(DIGITS[usize::from(byte & 15)]),
            ]
        })
        .collect()
}

fn filter_failure(code: RpcErrorCode, message: impl Into<String>) -> RpcFailure {
    RpcFailure::new(
        RpcFailureKind::InvalidParams,
        Some(RpcErrorDetail::new(code, message)),
    )
}
