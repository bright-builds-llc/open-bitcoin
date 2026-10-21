// Parity breadcrumbs:
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/chain.cpp
// - packages/bitcoin-knots/src/txdb.cpp

//! Pure decoding and proof of detached canonical genesis records.

mod proof;
pub(super) mod records;
pub(in crate::storage) type ProvedGenesisRecords = proof::ProvedGenesisRecords;

/// Validate detached data without opening or authorizing any live store.
pub(in crate::storage) fn validate_genesis_records(
    selected: open_bitcoin_core::consensus::chain_spec::CanonicalChain,
    records: &records::CanonicalGenesisRecords,
) -> Result<ProvedGenesisRecords, CanonicalBootstrapRefusal> {
    proof::validate_genesis_records(selected, records)
}

/// Fixed, payload-free refusal categories for untrusted durable records.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CanonicalBootstrapRefusal {
    MissingRecord,
    MalformedRecord,
    UnsupportedRecord,
    InconsistentGenesis,
    UnsupportedHistory,
    InconsistentIndex,
    StorageUnavailable,
    CommitUncertain,
    ChangedStore,
}

impl core::fmt::Display for CanonicalBootstrapRefusal {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter.write_str(match self {
            Self::MissingRecord => "canonical bootstrap required record missing",
            Self::MalformedRecord => "canonical bootstrap record malformed",
            Self::UnsupportedRecord => "canonical bootstrap record unsupported",
            Self::InconsistentGenesis => "canonical bootstrap genesis binding mismatch",
            Self::UnsupportedHistory => "canonical bootstrap ordinary history unsupported",
            Self::InconsistentIndex => "canonical bootstrap index binding mismatch",
            Self::StorageUnavailable => "canonical bootstrap storage unavailable",
            Self::CommitUncertain => "canonical bootstrap commit uncertain; reopen required",
            Self::ChangedStore => "canonical bootstrap store changed; reopen required",
        })
    }
}

impl std::error::Error for CanonicalBootstrapRefusal {}

type PreparedRows = std::collections::BTreeMap<&'static str, records::Rows>;

/// Encode and independently validate the entire batch before any publication.
pub(in crate::storage) fn prepare_genesis_rows(
    selected: open_bitcoin_core::consensus::chain_spec::CanonicalChain,
) -> Result<PreparedRows, CanonicalBootstrapRefusal> {
    use crate::storage::{snapshot_codec as snapshots, validation_history as history};
    use open_bitcoin_core::{
        chainstate::ChainPosition, consensus::chain_spec::CanonicalGenesisProof,
    };
    let block = selected.spec().genesis_block();
    let genesis = CanonicalGenesisProof::prove(selected.spec(), &block)
        .map_err(|_| CanonicalBootstrapRefusal::InconsistentGenesis)?;
    let position = ChainPosition::genesis(
        block.header.clone(),
        genesis.work(),
        i64::from(block.header.time),
    )
    .map_err(|_| CanonicalBootstrapRefusal::InconsistentGenesis)?;
    let entries = [open_bitcoin_network::HeaderEntry {
        block_hash: genesis.hash(),
        header: block.header.clone(),
        height: 0,
        chain_work: genesis.work(),
    }];
    let identity =
        history::BlockValidationIdentity::new(genesis.hash(), block.header.previous_block_hash, 0)
            .map_err(|_| CanonicalBootstrapRefusal::InconsistentGenesis)?;
    let mut rows = PreparedRows::new();
    let mut put = |namespace, key: &str, bytes: Vec<u8>| {
        rows.entry(namespace)
            .or_default()
            .insert(key.as_bytes().to_vec(), bytes);
    };
    let malformed = |_| CanonicalBootstrapRefusal::MalformedRecord;
    put("schema", "schema_version", b"2".to_vec());
    put(
        "schema",
        records::IDENTITY_KEY,
        records::encode_identity(selected),
    );
    put(
        "headers",
        "snapshot",
        snapshots::encode_header_entries(&entries).map_err(malformed)?,
    );
    put(
        "block_index",
        "snapshot",
        snapshots::encode_block_index_entries(&entries).map_err(malformed)?,
    );
    put(
        "block_index",
        &crate::storage::fjall_store::block_key(genesis.hash()),
        open_bitcoin_core::codec::encode_block(&block)
            .map_err(|_| CanonicalBootstrapRefusal::MalformedRecord)?,
    );
    put(
        "chainstate",
        "chain_meta",
        snapshots::encode_chain_meta(&[position], Some(&std::collections::HashMap::new()))
            .map_err(malformed)?,
    );
    put("coins", "B", genesis.hash().as_bytes().to_vec());
    put(
        "block_index",
        history::COVERAGE_KEY,
        history::encode_coverage(history::ValidationCoverage::Complete).to_vec(),
    );
    put(
        "block_index",
        &history::record_key(genesis.hash()),
        history::encode_record(history::ValidationHistoryRecord::new(
            identity,
            history::StoredValidationStatus::ScriptsValid,
        )),
    );
    let parsed = records::CanonicalGenesisRecords::parse_rows(
        rows.iter()
            .map(|(name, records)| (*name, records))
            .collect(),
    )?;
    validate_genesis_records(selected, &parsed)?;
    Ok(rows)
}

#[cfg(test)]
mod tests;
