// Parity breadcrumbs:
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/txdb.cpp
// - packages/bitcoin-knots/src/coins.cpp

use super::Refusal;
use crate::storage::{
    canonical_bootstrap::records::{IDENTITY_KEY, encode_identity},
    coins_codec, filter_index as filters,
    fjall_store::{
        self,
        bounded_rows::{RowFamily, decode_detached_rows},
        inspection::InspectedRecords,
    },
    snapshot_codec as snapshots, validation_history as history,
};
use open_bitcoin_core::{
    chainstate::ChainstateSnapshot,
    codec::{block::wire::parse_serialized_block, encode_block},
    consensus::{CanonicalChain, block_hash, crypto::Sha256},
    primitives::{Block, BlockHash},
};
use open_bitcoin_network::HeaderEntry;
use std::collections::{BTreeMap, HashMap};

mod projection;

pub(super) type Rows = BTreeMap<Vec<u8>, Vec<u8>>;
pub(super) type Namespaces = BTreeMap<String, Rows>;
/// Total bounded graph, including the retained replacement and its next block.
pub(in crate::storage) const MAX_HISTORY_BLOCKS: usize = 306;

impl crate::FjallNodeStore {
    /// Existing total retained graph envelope of safe canonical-history reopen.
    pub(crate) const fn canonical_history_reopen_limit() -> usize {
        MAX_HISTORY_BLOCKS
    }
}

/// Detached untrusted data. This parser never creates an accepted owner.
pub(in crate::storage) struct CanonicalHistoryRecords {
    pub(super) chain: CanonicalChain,
    pub(super) bodies: HashMap<BlockHash, Block>,
    pub(super) headers: Vec<HeaderEntry>,
    pub(super) index: Vec<HeaderEntry>,
    pub(super) snapshot: ChainstateSnapshot,
    pub(super) best_block: BlockHash,
    pub(super) validation: HashMap<BlockHash, history::ValidationHistoryRecord>,
    pub(super) coverage: history::ValidationCoverage,
    pub(super) basic: Rows,
    source: [u8; 32],
}

impl CanonicalHistoryRecords {
    pub(in crate::storage) fn decode(records: &InspectedRecords) -> Result<Self, Refusal> {
        let rows = records
            .namespaces()
            .map(|name| {
                Ok((
                    name.to_owned(),
                    records
                        .maybe_records(name)
                        .ok_or(Refusal::MissingRecord)?
                        .clone(),
                ))
            })
            .collect::<Result<Namespaces, Refusal>>()?;
        Self::parse_rows(&rows)
    }

    pub(super) fn parse_rows(namespaces: &Namespaces) -> Result<Self, Refusal> {
        inventory(namespaces)?;
        let schema = namespace(namespaces, "schema")?;
        if required(schema, "schema_version")? != b"2" {
            return Err(Refusal::MalformedRecord);
        }
        let identity = required(schema, IDENTITY_KEY)?;
        let chain = [
            CanonicalChain::Mainnet,
            CanonicalChain::Testnet3,
            CanonicalChain::Signet,
            CanonicalChain::Regtest,
        ]
        .into_iter()
        .find(|chain| encode_identity(*chain) == identity)
        .ok_or(Refusal::InconsistentGenesis)?;
        let bytes = family(namespaces, "headers", RowFamily::Headers)?;
        let headers = snapshots::decode_header_entries(&bytes)
            .map_err(malformed)?
            .entries;
        strict_json(&bytes, snapshots::encode_header_entries(&headers))?;
        let bytes = family(namespaces, "block_index", RowFamily::BlockIndex)?;
        let index = snapshots::decode_block_index_entries(&bytes)
            .map_err(malformed)?
            .entries;
        strict_json(&bytes, snapshots::encode_block_index_entries(&index))?;
        let bytes = family(namespaces, "chainstate", RowFamily::ChainMeta)?;
        let (active_chain, maybe_counts) =
            snapshots::decode_chain_meta(&bytes).map_err(malformed)?;
        let counts = maybe_counts.ok_or(Refusal::MissingRecord)?;
        strict_json(
            &bytes,
            snapshots::encode_chain_meta(&active_chain, Some(&counts)),
        )?;
        if active_chain.is_empty()
            || active_chain.len() > MAX_HISTORY_BLOCKS
            || headers.len() > MAX_HISTORY_BLOCKS
            || index.len() > MAX_HISTORY_BLOCKS
        {
            return Err(Refusal::UnsupportedHistory);
        }
        let coins = namespace(namespaces, "coins")?;
        let best_block =
            coins_codec::decode_best_block_value(required(coins, "B")?).map_err(malformed)?;
        // An unfinished flush is never a complete checkpoint, even with empty bytes.
        if coins.contains_key(b"H".as_slice()) {
            return Err(Refusal::UnsupportedHistory);
        }
        let mut utxos = HashMap::new();
        for (key, value) in coins.iter().filter(|(key, _)| key.starts_with(b"C")) {
            let outpoint = coins_codec::decode_coin_key(key).map_err(malformed)?;
            let coin = coins_codec::decode_coin_value(value).map_err(malformed)?;
            if coins_codec::encode_coin_value(&coin).map_err(malformed)? != *value {
                return Err(Refusal::MalformedRecord);
            }
            utxos.insert(outpoint, coin);
        }
        let mut undo_by_block = HashMap::new();
        for (key, bytes) in namespace(namespaces, "chainstate")?
            .iter()
            .filter(|(key, _)| key.starts_with(b"undo:"))
        {
            let hash = keyed_hash(key, "undo:")?;
            let undo = snapshots::decode_block_undo(bytes).map_err(malformed)?;
            strict_json(bytes, snapshots::encode_block_undo(&undo))?;
            undo_by_block.insert(hash, undo);
        }
        let index_rows = namespace(namespaces, "block_index")?;
        let mut bodies = HashMap::new();
        let mut validation = HashMap::new();
        let mut basic = Rows::new();
        for (key, bytes) in index_rows {
            let name = std::str::from_utf8(key).map_err(malformed)?;
            if name.starts_with("block:") {
                let block = parse_serialized_block(bytes).map_err(malformed)?;
                let hash = block_hash(&block.header);
                if fjall_store::block_key(hash) != name
                    || encode_block(&block).map_err(malformed)? != *bytes
                {
                    return Err(Refusal::MalformedRecord);
                }
                bodies.insert(hash, block);
            } else if name.starts_with(history::RECORD_PREFIX) {
                let record = history::decode_record(name, bytes).map_err(malformed)?;
                validation.insert(record.identity().hash(), record);
            } else if name.starts_with("basic_filter:") || name == "prune_locks" {
                basic.insert(key.clone(), bytes.clone());
            }
        }
        let coverage = history::decode_coverage(required(index_rows, history::COVERAGE_KEY)?)
            .map_err(malformed)?;
        Ok(Self {
            chain,
            bodies,
            headers,
            index,
            snapshot: ChainstateSnapshot {
                active_chain,
                utxos,
                undo_by_block,
                maybe_confirmed_txid_counts: Some(counts),
            },
            best_block,
            validation,
            coverage,
            basic,
            source: source_digest(namespaces),
        })
    }

    pub(in crate::storage) fn source_digest(&self) -> [u8; 32] {
        self.source
    }
}

fn malformed<T>(_: T) -> Refusal {
    Refusal::MalformedRecord
}
fn namespace<'a>(all: &'a Namespaces, name: &str) -> Result<&'a Rows, Refusal> {
    all.get(name).ok_or(Refusal::MissingRecord)
}
fn required<'a>(rows: &'a Rows, key: &str) -> Result<&'a [u8], Refusal> {
    rows.get(key.as_bytes())
        .map(Vec::as_slice)
        .ok_or(Refusal::MissingRecord)
}
fn family(all: &Namespaces, name: &str, kind: RowFamily) -> Result<Vec<u8>, Refusal> {
    decode_detached_rows(kind, namespace(all, name)?, true)
        .map_err(malformed)?
        .ok_or(Refusal::MissingRecord)
}

fn keyed_hash(key: &[u8], prefix: &str) -> Result<BlockHash, Refusal> {
    let text = std::str::from_utf8(key).map_err(malformed)?;
    let suffix = text.strip_prefix(prefix).ok_or(Refusal::MalformedRecord)?;
    if suffix.len() != 64
        || !suffix
            .bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
    {
        return Err(Refusal::MalformedRecord);
    }
    let mut bytes = [0; 32];
    for (index, byte) in bytes.iter_mut().enumerate() {
        *byte = u8::from_str_radix(&suffix[index * 2..index * 2 + 2], 16).map_err(malformed)?;
    }
    Ok(BlockHash::from_byte_array(bytes))
}

fn strict_json(bytes: &[u8], encoded: Result<Vec<u8>, crate::StorageError>) -> Result<(), Refusal> {
    let actual: serde_json::Value = serde_json::from_slice(bytes).map_err(malformed)?;
    let expected: serde_json::Value =
        serde_json::from_slice(&encoded.map_err(malformed)?).map_err(malformed)?;
    if actual != expected {
        return Err(Refusal::MalformedRecord);
    }
    Ok(())
}

fn inventory(all: &Namespaces) -> Result<(), Refusal> {
    let mut entries = 0usize;
    let mut bytes = 0usize;
    for (namespace, rows) in all {
        entries = entries.checked_add(1).ok_or(Refusal::MalformedRecord)?;
        bytes = bytes
            .checked_add(namespace.len())
            .ok_or(Refusal::MalformedRecord)?;
        if !matches!(
            namespace.as_str(),
            "schema"
                | "headers"
                | "block_index"
                | "chainstate"
                | "coins"
                | "wallet"
                | "metrics"
                | "mempool"
                | "runtime"
        ) {
            return Err(Refusal::UnsupportedRecord);
        }
        for (key, value) in rows {
            entries = entries.checked_add(1).ok_or(Refusal::MalformedRecord)?;
            bytes = bytes
                .checked_add(key.len())
                .and_then(|n| n.checked_add(value.len()))
                .ok_or(Refusal::MalformedRecord)?;
            if entries > 4096 || bytes > 536870912 || key.len() > 4096 {
                return Err(Refusal::MalformedRecord);
            }
            let authority = matches!(
                namespace.as_str(),
                "schema" | "headers" | "block_index" | "chainstate" | "coins"
            );
            if authority && (value.is_empty() || value.len() > 65536) {
                return Err(Refusal::MalformedRecord);
            }
            if namespace == "coins" {
                if !matches!(key.as_slice(), b"B" | b"H") && !key.starts_with(b"C") {
                    return Err(Refusal::UnsupportedRecord);
                }
                continue;
            }
            let key = std::str::from_utf8(key).map_err(malformed)?;
            let supported = match namespace.as_str() {
                "schema" => matches!(key, "schema_version" | IDENTITY_KEY),
                "headers" => key == "snapshot" || key.starts_with("snapshot:"),
                "chainstate" => {
                    key == "chain_meta"
                        || key.starts_with("chain_meta:")
                        || key.starts_with("undo:")
                }
                "block_index" => {
                    key == "snapshot"
                        || key.starts_with("snapshot:")
                        || key.starts_with("block:")
                        || key.starts_with(history::RECORD_PREFIX)
                        || key == history::COVERAGE_KEY
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
                            .is_some_and(|s| !s.is_empty())
                        || key
                            .strip_prefix("wallet_rescan_job:")
                            .is_some_and(|s| !s.is_empty())
                }
                "metrics" | "mempool" | "runtime" => key == "snapshot",
                _ => false,
            };
            if !supported {
                return Err(Refusal::UnsupportedRecord);
            }
        }
    }
    Ok(())
}

fn source_digest(all: &Namespaces) -> [u8; 32] {
    let mut manifest = Vec::from(b"open-bitcoin:complete-history-source:v1".as_slice());
    for (name, rows) in all {
        manifest.extend((name.len() as u64).to_le_bytes());
        manifest.extend(name.as_bytes());
        manifest.extend((rows.len() as u64).to_le_bytes());
        for (key, value) in rows {
            manifest.extend((key.len() as u64).to_le_bytes());
            manifest.extend(key);
            manifest.extend((value.len() as u64).to_le_bytes());
            manifest.extend(Sha256::digest(value));
        }
    }
    Sha256::digest(&manifest)
}
