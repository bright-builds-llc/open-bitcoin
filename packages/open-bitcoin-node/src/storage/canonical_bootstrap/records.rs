// Parity breadcrumbs:
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/chain.cpp
// - packages/bitcoin-knots/src/txdb.cpp

use super::CanonicalBootstrapRefusal as Refusal;
use crate::storage::fjall_store::inspection::InspectedRecords;
use crate::storage::{
    coins_codec, filter_index as filters, snapshot_codec as snapshots,
    validation_history as history,
};
use open_bitcoin_core::{
    chainstate::ChainPosition,
    codec::{encode_block, parse_block},
    consensus::chain_spec::CanonicalChain,
    primitives::{Block, BlockHash},
};
use open_bitcoin_network::HeaderEntry;
use std::collections::BTreeMap;

pub(super) type Rows = BTreeMap<Vec<u8>, Vec<u8>>;

/// Version byte, selected-chain byte, then raw 32-byte genesis hash.
pub(in crate::storage) const IDENTITY_KEY: &str = "canonical_chain:v1";
const MAX_AUTHORITY_RECORD_BYTES: usize = 64 * 1024;

pub(in crate::storage) fn encode_identity(chain: CanonicalChain) -> Vec<u8> {
    let mut bytes = vec![
        1,
        match chain {
            CanonicalChain::Mainnet => 0,
            CanonicalChain::Testnet3 => 1,
            CanonicalChain::Signet => 2,
            CanonicalChain::Regtest => 3,
        },
    ];
    bytes.extend_from_slice(chain.spec().genesis_hash().as_bytes());
    bytes
}

/// Parsed data only: none of these fields grants a live owner or store capability.
pub(in crate::storage) struct CanonicalGenesisRecords {
    pub(super) chain: CanonicalChain,
    pub(super) identity_hash: BlockHash,
    pub(super) body: Block,
    pub(super) body_key: String,
    pub(super) headers: Vec<HeaderEntry>,
    pub(super) index: Vec<HeaderEntry>,
    pub(super) active: Vec<ChainPosition>,
    pub(super) best_block: BlockHash,
    pub(super) validation: history::ValidationHistoryRecord,
    pub(super) coverage: history::ValidationCoverage,
    pub(super) basic: Rows,
}

impl CanonicalGenesisRecords {
    pub(in crate::storage) fn decode(records: &InspectedRecords) -> Result<Self, Refusal> {
        let mut namespaces = BTreeMap::new();
        for name in records.namespaces() {
            let rows = records.maybe_records(name).ok_or(Refusal::MissingRecord)?;
            namespaces.insert(name, rows);
        }
        Self::parse_rows(namespaces)
    }

    pub(super) fn parse_rows(namespaces: BTreeMap<&str, &Rows>) -> Result<Self, Refusal> {
        inventory(&namespaces)?;
        let schema = required(&namespaces, "schema", "schema_version")?;
        if schema != b"2" {
            return Err(Refusal::MalformedRecord);
        }
        let (chain, identity_hash) =
            decode_identity(required(&namespaces, "schema", IDENTITY_KEY)?)?;
        let header_bytes = required(&namespaces, "headers", "snapshot")?;
        let headers = snapshots::decode_header_entries(header_bytes)
            .map_err(|_| Refusal::MalformedRecord)?
            .entries;
        strict_json(header_bytes, snapshots::encode_header_entries(&headers))?;
        let index_bytes = required(&namespaces, "block_index", "snapshot")?;
        let index = snapshots::decode_block_index_entries(index_bytes)
            .map_err(|_| Refusal::MalformedRecord)?
            .entries;
        strict_json(index_bytes, snapshots::encode_block_index_entries(&index))?;
        let meta_bytes = required(&namespaces, "chainstate", "chain_meta")?;
        let (active, maybe_counts) =
            snapshots::decode_chain_meta(meta_bytes).map_err(|_| Refusal::MalformedRecord)?;
        // Canonical genesis writes explicit empty confirmed-tx counts; legacy
        // absent counts would otherwise trigger hydration during recovery.
        let counts = maybe_counts.ok_or(Refusal::MissingRecord)?;
        if !counts.is_empty() {
            return Err(Refusal::UnsupportedHistory);
        }
        strict_json(
            meta_bytes,
            snapshots::encode_chain_meta(&active, Some(&counts)),
        )?;
        let best_block = coins_codec::decode_best_block_value(required(&namespaces, "coins", "B")?)
            .map_err(|_| Refusal::MalformedRecord)?;
        let index_rows = namespaces
            .get("block_index")
            .ok_or(Refusal::MissingRecord)?;
        let (body_key, body_bytes) = single_prefixed(index_rows, "block:")?;
        let body = parse_block(body_bytes).map_err(|_| Refusal::MalformedRecord)?;
        if encode_block(&body).map_err(|_| Refusal::MalformedRecord)? != body_bytes {
            return Err(Refusal::MalformedRecord);
        }
        let (validation_key, validation_bytes) =
            single_prefixed(index_rows, history::RECORD_PREFIX)?;
        let validation = history::decode_record(&validation_key, validation_bytes)
            .map_err(|_| Refusal::MalformedRecord)?;
        let coverage =
            history::decode_coverage(required(&namespaces, "block_index", history::COVERAGE_KEY)?)
                .map_err(|_| Refusal::MalformedRecord)?;
        let basic = index_rows
            .iter()
            .filter(|(key, _)| {
                key.starts_with(b"basic_filter:") || key.as_slice() == b"prune_locks"
            })
            .map(|(key, value)| (key.clone(), value.clone()))
            .collect();
        Ok(Self {
            chain,
            identity_hash,
            body,
            body_key,
            headers,
            index,
            active,
            best_block,
            validation,
            coverage,
            basic,
        })
    }
}

fn required<'a>(
    namespaces: &BTreeMap<&str, &'a Rows>,
    namespace: &str,
    key: &str,
) -> Result<&'a [u8], Refusal> {
    let bytes = namespaces
        .get(namespace)
        .and_then(|rows| rows.get(key.as_bytes()))
        .ok_or(Refusal::MissingRecord)?;
    if bytes.is_empty() || bytes.len() > MAX_AUTHORITY_RECORD_BYTES {
        return Err(Refusal::MalformedRecord);
    }
    Ok(bytes)
}

fn single_prefixed<'a>(rows: &'a Rows, prefix: &str) -> Result<(String, &'a [u8]), Refusal> {
    let mut matches = rows
        .iter()
        .filter(|(key, _)| key.starts_with(prefix.as_bytes()));
    let (key, bytes) = matches.next().ok_or(Refusal::MissingRecord)?;
    if matches.next().is_some() {
        return Err(Refusal::UnsupportedHistory);
    }
    if bytes.is_empty() || bytes.len() > MAX_AUTHORITY_RECORD_BYTES {
        return Err(Refusal::MalformedRecord);
    }
    Ok((
        std::str::from_utf8(key)
            .map_err(|_| Refusal::MalformedRecord)?
            .to_owned(),
        bytes,
    ))
}

fn decode_identity(bytes: &[u8]) -> Result<(CanonicalChain, BlockHash), Refusal> {
    if bytes.len() != 34 || bytes[0] != 1 {
        return Err(Refusal::MalformedRecord);
    }
    let chain = match bytes[1] {
        0 => CanonicalChain::Mainnet,
        1 => CanonicalChain::Testnet3,
        2 => CanonicalChain::Signet,
        3 => CanonicalChain::Regtest,
        _ => return Err(Refusal::MalformedRecord),
    };
    Ok((
        chain,
        BlockHash::from_byte_array(
            bytes[2..]
                .try_into()
                .map_err(|_| Refusal::MalformedRecord)?,
        ),
    ))
}

// Existing snapshot codecs intentionally read older permissive blobs. This
// boundary accepts only the complete current shape without changing those APIs.
fn strict_json(bytes: &[u8], encoded: Result<Vec<u8>, crate::StorageError>) -> Result<(), Refusal> {
    let actual: serde_json::Value =
        serde_json::from_slice(bytes).map_err(|_| Refusal::MalformedRecord)?;
    let expected: serde_json::Value =
        serde_json::from_slice(&encoded.map_err(|_| Refusal::MalformedRecord)?)
            .map_err(|_| Refusal::MalformedRecord)?;
    if actual != expected {
        return Err(Refusal::MalformedRecord);
    }
    Ok(())
}

fn inventory(namespaces: &BTreeMap<&str, &Rows>) -> Result<(), Refusal> {
    for (&namespace, rows) in namespaces {
        for (key, value) in *rows {
            let key = std::str::from_utf8(key).map_err(|_| Refusal::UnsupportedRecord)?;
            let supported = match namespace {
                "schema" => matches!(key, "schema_version" | IDENTITY_KEY),
                "headers" => key == "snapshot",
                "chainstate" => key == "chain_meta",
                "coins" => key == "B",
                "block_index" => {
                    key == "snapshot"
                        || key == history::COVERAGE_KEY
                        || key.starts_with("block:")
                        || key.starts_with(history::RECORD_PREFIX)
                        || key == filters::STATE_KEY
                        || key == filters::ownership::OWNER_KEY
                        || key.starts_with(filters::RECORD_PREFIX)
                        || key.starts_with(filters::ACTIVE_PREFIX)
                        || key == "prune_locks"
                }
                "wallet" => {
                    matches!(key, "snapshot" | "wallet_registry" | "selected_wallet")
                        || key
                            .strip_prefix("wallet_snapshot:")
                            .is_some_and(|name| !name.is_empty())
                        || key
                            .strip_prefix("wallet_rescan_job:")
                            .is_some_and(|name| !name.is_empty())
                }
                "metrics" | "mempool" => key == "snapshot",
                "runtime" => key == "snapshot",
                _ => false,
            };
            if !supported {
                return Err(Refusal::UnsupportedRecord);
            }
            if matches!(
                namespace,
                "schema" | "headers" | "chainstate" | "coins" | "block_index"
            ) && (value.is_empty() || value.len() > MAX_AUTHORITY_RECORD_BYTES)
            {
                return Err(Refusal::MalformedRecord);
            }
        }
        if !matches!(
            namespace,
            "schema"
                | "headers"
                | "chainstate"
                | "coins"
                | "block_index"
                | "wallet"
                | "metrics"
                | "mempool"
                | "runtime"
        ) {
            return Err(Refusal::UnsupportedRecord);
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::super::tests::{fixture, parse};
    use super::*;

    #[test]
    fn phase166_records_authority_record_bound_keeps_current_genesis_shape() {
        // Arrange
        let mut rows = fixture(CanonicalChain::Regtest);
        let bytes = rows
            .get_mut("headers")
            .expect("headers")
            .get_mut(b"snapshot".as_slice())
            .expect("snapshot");
        bytes.resize(MAX_AUTHORITY_RECORD_BYTES, b' ');
        // Act / Assert
        assert!(parse(&rows).is_ok());
        rows.get_mut("headers")
            .expect("headers")
            .get_mut(b"snapshot".as_slice())
            .expect("snapshot")
            .push(b' ');
        assert!(matches!(parse(&rows), Err(Refusal::MalformedRecord)));
    }

    #[test]
    fn phase166_records_absent_namespace_never_becomes_empty_authority() {
        // Arrange
        let rows = fixture(CanonicalChain::Regtest);
        // Act / Assert
        for name in ["schema", "headers", "block_index", "chainstate", "coins"] {
            let mut changed = rows.clone();
            changed.remove(name);
            assert!(matches!(parse(&changed), Err(Refusal::MissingRecord)));
            changed.insert(name.to_owned(), Rows::new());
            assert!(matches!(parse(&changed), Err(Refusal::MissingRecord)));
        }
    }
}
