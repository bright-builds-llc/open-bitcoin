// Parity breadcrumbs:
// - packages/bitcoin-knots/src/txdb.cpp

//! Data integrity only: neither manifests nor digests establish chain authority.

use super::super::{StorageError, StorageNamespace, corruption};
use open_bitcoin_core::consensus::crypto::Sha256;
use std::collections::BTreeMap;

pub(super) const ROW_BYTES: usize = 65536;
pub(super) const MAX_ENTRIES: usize = 4096;
pub(super) const MAX_BYTES: usize = 536870912;
const MAGIC: &[u8; 8] = b"OBROWS\0\x01";
const MANIFEST_BYTES: usize = 70;

#[derive(Clone, Copy, Debug)]
pub(in crate::storage) enum RowFamily {
    Headers,
    BlockIndex,
    ChainMeta,
}

impl RowFamily {
    pub(super) fn namespace(self) -> StorageNamespace {
        match self {
            Self::Headers => StorageNamespace::Headers,
            Self::BlockIndex => StorageNamespace::BlockIndex,
            Self::ChainMeta => StorageNamespace::Chainstate,
        }
    }

    pub(super) fn root(self) -> &'static str {
        match self {
            Self::ChainMeta => "chain_meta",
            _ => "snapshot",
        }
    }

    fn tag(self) -> [u8; 2] {
        match self {
            Self::Headers => [1, 1],
            Self::BlockIndex => [2, 1],
            Self::ChainMeta => [3, 2],
        }
    }

    pub(super) fn owns(self, key: &[u8]) -> bool {
        key == self.root().as_bytes() || key.starts_with(format!("{}:", self.root()).as_bytes())
    }

    pub(super) fn manifest_key(self) -> String {
        format!("{}:segments:v1", self.root())
    }

    fn segment_key(self, generation: &[u8], ordinal: usize) -> Vec<u8> {
        let generation: String = generation.iter().map(|b| format!("{b:02x}")).collect();
        format!("{}:segment:v1:{generation}:{ordinal:08x}", self.root()).into_bytes()
    }

    fn error(self, detail: &str) -> StorageError {
        corruption(self.namespace(), detail)
    }
}

// Hash the whole stream once, then bind that digest to a fixed domain and family.
// This avoids a second stream-sized allocation solely to prepend the domain.
fn digest(family: RowFamily, bytes: &[u8]) -> [u8; 32] {
    let mut binding = Vec::from(b"open-bitcoin:bounded-rows:v1:".as_slice());
    binding.extend(family.tag());
    binding.extend((bytes.len() as u64).to_be_bytes());
    binding.extend(Sha256::digest(bytes));
    Sha256::digest(&binding)
}

pub(in crate::storage::fjall_store) struct PreparedRows {
    pub(super) family: RowFamily,
    pub(super) rows: BTreeMap<Vec<u8>, Vec<u8>>,
}

pub(super) fn prepare(family: RowFamily, bytes: Vec<u8>) -> Result<PreparedRows, StorageError> {
    let count = bytes.len().div_ceil(ROW_BYTES);
    if bytes.len() > MAX_BYTES || count + 1 > MAX_ENTRIES {
        return Err(family.error("bounded row family exceeds inspection limits"));
    }
    let mut rows = BTreeMap::new();
    if bytes.len() <= ROW_BYTES {
        rows.insert(family.root().as_bytes().to_vec(), bytes);
        return Ok(PreparedRows { family, rows });
    }
    let hash = digest(family, &bytes);
    let generation = &hash[..16];
    let mut manifest = Vec::from(MAGIC.as_slice());
    manifest.extend(family.tag());
    manifest.extend((count as u32).to_be_bytes());
    manifest.extend((bytes.len() as u64).to_be_bytes());
    manifest.extend(generation);
    manifest.extend(hash);
    rows.insert(family.manifest_key().into_bytes(), manifest);
    for (ordinal, segment) in bytes.chunks(ROW_BYTES).enumerate() {
        rows.insert(family.segment_key(generation, ordinal), segment.to_vec());
    }
    Ok(PreparedRows { family, rows })
}

/// Decode an already inspected complete map. The caller retains the global
/// all-namespace inspection budget; this function never opens or repairs data.
pub(in crate::storage) fn decode_detached_rows(
    family: RowFamily,
    records: &BTreeMap<Vec<u8>, Vec<u8>>,
    canonical: bool,
) -> Result<Option<Vec<u8>>, StorageError> {
    let mut total = 0_usize;
    let mut count = 0_usize;
    for (key, value) in records.iter().filter(|(key, _)| family.owns(key)) {
        count = count
            .checked_add(1)
            .ok_or_else(|| family.error("row count overflow"))?;
        total = total
            .checked_add(key.len())
            .and_then(|n| n.checked_add(value.len()))
            .ok_or_else(|| family.error("row byte overflow"))?;
        if count > MAX_ENTRIES
            || total > MAX_BYTES
            || key.len() > 4096
            || (canonical && value.len() > ROW_BYTES)
        {
            return Err(family.error("bounded row inspection limits exceeded"));
        }
    }
    if count == 0 {
        return Ok(None);
    }
    if let Some(root) = records.get(family.root().as_bytes()) {
        if count != 1 {
            return Err(family.error("mixed aggregate and segmented row family"));
        }
        return Ok(Some(root.clone()));
    }
    let manifest = records
        .get(family.manifest_key().as_bytes())
        .ok_or_else(|| family.error("orphan segmented row family"))?;
    if manifest.len() != MANIFEST_BYTES
        || &manifest[..8] != MAGIC
        || manifest[8..10] != family.tag()
    {
        return Err(family.error("invalid bounded row manifest shape/version/namespace/kind"));
    }
    let segments = u32::from_be_bytes(
        manifest[10..14]
            .try_into()
            .map_err(|_| family.error("invalid bounded row count width"))?,
    ) as usize;
    let length = usize::try_from(u64::from_be_bytes(
        manifest[14..22]
            .try_into()
            .map_err(|_| family.error("invalid bounded row length width"))?,
    ))
    .map_err(|_| family.error("bounded row length overflow"))?;
    if length <= ROW_BYTES
        || length > MAX_BYTES
        || segments == 0
        || segments + 1 != count
        || segments != length.div_ceil(ROW_BYTES)
    {
        return Err(family.error("invalid bounded row count/length"));
    }
    let generation = &manifest[22..38];
    if generation != &manifest[38..54] {
        return Err(family.error("invalid bounded row generation"));
    }
    // Validate the entire exact key set and lengths before reconstruction allocation.
    for ordinal in 0..segments {
        let segment = records
            .get(&family.segment_key(generation, ordinal))
            .ok_or_else(|| {
                family.error("missing/noncanonical bounded row ordinal or generation")
            })?;
        let expected = if ordinal + 1 == segments {
            length - ordinal * ROW_BYTES
        } else {
            ROW_BYTES
        };
        if segment.len() != expected || segment.len() > ROW_BYTES {
            return Err(family.error("invalid bounded row segment length"));
        }
    }
    let mut bytes = Vec::with_capacity(length);
    for ordinal in 0..segments {
        bytes.extend(&records[&family.segment_key(generation, ordinal)]);
    }
    if digest(family, &bytes).as_slice() != &manifest[38..70] {
        return Err(family.error("bounded row stream digest mismatch"));
    }
    Ok(Some(bytes))
}
