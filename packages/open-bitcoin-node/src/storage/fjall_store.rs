// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

//! Fjall-backed durable storage adapter for node-owned runtime state.

use std::{path::Path, str};

use fjall::{Database, Keyspace, KeyspaceCreateOptions, PersistMode as FjallPersistMode};
use open_bitcoin_core::{
    chainstate::ChainstateSnapshot,
    codec::{encode_block, parse_block},
    consensus::block_hash,
    primitives::{Block, BlockHash},
    wallet::WalletSnapshot,
};
use open_bitcoin_network::{HeaderEntry, HeaderStore};

use crate::metrics::{
    MetricRetentionPolicy, MetricSample, MetricsStatus, append_and_prune_metric_samples,
};

use super::{
    MetricsStorageSnapshot, PersistMode, RecoveryMarker, RuntimeMetadata, SchemaVersion,
    StorageError, StorageNamespace, StorageRecoveryAction, StoredHeaderEntries,
    snapshot_codec::{
        decode_block_index_entries, decode_chainstate_snapshot, decode_header_entries,
        decode_metrics_snapshot, decode_recovery_marker, decode_runtime_metadata,
        decode_wallet_snapshot, encode_block_index_entries, encode_chainstate_snapshot,
        encode_header_entries, encode_metrics_snapshot, encode_recovery_marker,
        encode_runtime_metadata, encode_wallet_snapshot,
    },
};

const SNAPSHOT_KEY: &str = "snapshot";
const SCHEMA_VERSION_KEY: &str = "schema_version";
const RECOVERY_MARKER_KEY: &str = "recovery_marker";

/// Durable node storage backed by one fjall database and namespace keyspaces.
pub struct FjallNodeStore {
    db: Database,
    headers: Keyspace,
    block_index: Keyspace,
    chainstate: Keyspace,
    wallet: Keyspace,
    metrics: Keyspace,
    runtime: Keyspace,
    schema: Keyspace,
}

impl FjallNodeStore {
    /// Open or create the store rooted at `path` and verify schema metadata.
    pub fn open(path: impl AsRef<Path>) -> Result<Self, StorageError> {
        let db = Database::builder(path.as_ref()).open().map_err(|error| {
            backend_failure(
                StorageNamespace::Runtime,
                error,
                StorageRecoveryAction::Restart,
            )
        })?;

        let store = Self {
            headers: open_keyspace(&db, StorageNamespace::Headers)?,
            block_index: open_keyspace(&db, StorageNamespace::BlockIndex)?,
            chainstate: open_keyspace(&db, StorageNamespace::Chainstate)?,
            wallet: open_keyspace(&db, StorageNamespace::Wallet)?,
            metrics: open_keyspace(&db, StorageNamespace::Metrics)?,
            runtime: open_keyspace(&db, StorageNamespace::Runtime)?,
            schema: open_keyspace(&db, StorageNamespace::Schema)?,
            db,
        };
        store.ensure_schema()?;

        Ok(store)
    }

    /// Persist a complete chainstate snapshot.
    pub fn save_chainstate_snapshot(
        &self,
        snapshot: &ChainstateSnapshot,
        mode: PersistMode,
    ) -> Result<(), StorageError> {
        let bytes = encode_chainstate_snapshot(snapshot)?;
        self.put_bytes(StorageNamespace::Chainstate, SNAPSHOT_KEY, bytes, mode)
    }

    /// Load the latest persisted chainstate snapshot, if present.
    pub fn load_chainstate_snapshot(&self) -> Result<Option<ChainstateSnapshot>, StorageError> {
        self.get_bytes(StorageNamespace::Chainstate, SNAPSHOT_KEY)?
            .map(|bytes| decode_chainstate_snapshot(&bytes))
            .transpose()
    }

    /// Persist header metadata and the block-index projection atomically.
    pub fn save_header_entries(
        &self,
        entries: &[HeaderEntry],
        mode: PersistMode,
    ) -> Result<(), StorageError> {
        let header_bytes = encode_header_entries(entries)?;
        let block_index_bytes = encode_block_index_entries(entries)?;
        let mut batch = self.db.batch();
        if let Some(mode) = fjall_persist_mode(mode) {
            batch = batch.durability(Some(mode));
        }
        batch.insert(&self.headers, SNAPSHOT_KEY, header_bytes);
        batch.insert(&self.block_index, SNAPSHOT_KEY, block_index_bytes);
        batch.commit().map_err(|error| {
            backend_failure(
                StorageNamespace::Headers,
                error,
                StorageRecoveryAction::Restart,
            )
        })
    }

    /// Load persisted header entries.
    pub fn load_header_entries(&self) -> Result<Option<StoredHeaderEntries>, StorageError> {
        self.get_bytes(StorageNamespace::Headers, SNAPSHOT_KEY)?
            .map(|bytes| decode_header_entries(&bytes))
            .transpose()
    }

    /// Load persisted block-index entries.
    pub fn load_block_index_entries(&self) -> Result<Option<StoredHeaderEntries>, StorageError> {
        self.get_bytes(StorageNamespace::BlockIndex, SNAPSHOT_KEY)?
            .map(|bytes| decode_block_index_entries(&bytes))
            .transpose()
    }

    /// Persist a downloaded block under its canonical block hash.
    pub fn save_block(&self, block: &Block, mode: PersistMode) -> Result<BlockHash, StorageError> {
        let block_hash = block_hash(&block.header);
        let bytes =
            encode_block(block).map_err(|error| corruption(StorageNamespace::BlockIndex, error))?;
        self.put_bytes(
            StorageNamespace::BlockIndex,
            &block_key(block_hash),
            bytes,
            mode,
        )?;

        Ok(block_hash)
    }

    /// Load a downloaded block by canonical block hash, if present.
    pub fn load_block(&self, block_hash: BlockHash) -> Result<Option<Block>, StorageError> {
        self.get_bytes(StorageNamespace::BlockIndex, &block_key(block_hash))?
            .map(|bytes| {
                parse_block(&bytes).map_err(|error| corruption(StorageNamespace::BlockIndex, error))
            })
            .transpose()
    }

    /// Rebuild an in-memory header store from persisted header entries.
    pub fn load_header_store(&self) -> Result<Option<HeaderStore>, StorageError> {
        let Some(stored) = self.load_header_entries()? else {
            return Ok(None);
        };

        HeaderStore::from_entries(stored.entries)
            .map(Some)
            .map_err(|error| corruption(StorageNamespace::Headers, error))
    }

    /// Persist a complete wallet snapshot.
    pub fn save_wallet_snapshot(
        &self,
        snapshot: &WalletSnapshot,
        mode: PersistMode,
    ) -> Result<(), StorageError> {
        let bytes = encode_wallet_snapshot(snapshot)?;
        self.put_bytes(StorageNamespace::Wallet, SNAPSHOT_KEY, bytes, mode)
    }

    /// Load the latest persisted wallet snapshot, if present.
    pub fn load_wallet_snapshot(&self) -> Result<Option<WalletSnapshot>, StorageError> {
        self.get_bytes(StorageNamespace::Wallet, SNAPSHOT_KEY)?
            .map(|bytes| decode_wallet_snapshot(&bytes))
            .transpose()
    }

    /// Persist the metrics history snapshot for runtime collectors.
    pub fn save_metrics_snapshot(
        &self,
        snapshot: &MetricsStorageSnapshot,
        mode: PersistMode,
    ) -> Result<(), StorageError> {
        let bytes = encode_metrics_snapshot(snapshot)?;
        self.put_bytes(StorageNamespace::Metrics, SNAPSHOT_KEY, bytes, mode)
    }

    /// Load the metrics history snapshot, if present.
    pub fn load_metrics_snapshot(&self) -> Result<Option<MetricsStorageSnapshot>, StorageError> {
        self.get_bytes(StorageNamespace::Metrics, SNAPSHOT_KEY)?
            .map(|bytes| decode_metrics_snapshot(&bytes))
            .transpose()
    }

    /// Append metric samples to persisted history and enforce bounded retention before saving.
    pub fn append_metric_samples(
        &self,
        samples: &[MetricSample],
        retention: MetricRetentionPolicy,
        now_unix_seconds: u64,
        mode: PersistMode,
    ) -> Result<MetricsStorageSnapshot, StorageError> {
        let maybe_existing_snapshot = self.load_metrics_snapshot()?;
        let existing_samples = maybe_existing_snapshot
            .map(|snapshot| snapshot.samples)
            .unwrap_or_default();
        let retained_samples = append_and_prune_metric_samples(
            &existing_samples,
            samples,
            retention,
            now_unix_seconds,
        );
        let snapshot = MetricsStorageSnapshot {
            samples: retained_samples,
        };
        self.save_metrics_snapshot(&snapshot, mode)?;

        Ok(snapshot)
    }

    /// Load status-facing metrics availability metadata from the stored history snapshot.
    pub fn load_metrics_status(
        &self,
        retention: MetricRetentionPolicy,
    ) -> Result<MetricsStatus, StorageError> {
        let maybe_snapshot = self.load_metrics_snapshot()?;
        if let Some(snapshot) = maybe_snapshot {
            return Ok(MetricsStatus::available_with_samples(
                retention,
                snapshot.samples,
            ));
        }

        Ok(MetricsStatus::unavailable(
            retention,
            "metrics history unavailable: no metrics snapshot recorded",
        ))
    }

    /// Persist runtime metadata outside pure chainstate and wallet snapshots.
    pub fn save_runtime_metadata(
        &self,
        metadata: &RuntimeMetadata,
        mode: PersistMode,
    ) -> Result<(), StorageError> {
        let bytes = encode_runtime_metadata(metadata)?;
        self.put_bytes(StorageNamespace::Runtime, SNAPSHOT_KEY, bytes, mode)
    }

    /// Load persisted runtime metadata, if present.
    pub fn load_runtime_metadata(&self) -> Result<Option<RuntimeMetadata>, StorageError> {
        self.get_bytes(StorageNamespace::Runtime, SNAPSHOT_KEY)?
            .map(|bytes| decode_runtime_metadata(&bytes))
            .transpose()
    }

    /// Mark an interrupted namespace write that needs operator recovery.
    pub fn mark_interrupted_write(
        &self,
        namespace: StorageNamespace,
        action: StorageRecoveryAction,
        detail: impl Into<String>,
        mode: PersistMode,
    ) -> Result<RecoveryMarker, StorageError> {
        let marker = RecoveryMarker::new(namespace, action, detail);
        let bytes = encode_recovery_marker(&marker)?;
        self.put_bytes(StorageNamespace::Runtime, RECOVERY_MARKER_KEY, bytes, mode)?;

        let mut metadata = self.load_runtime_metadata()?.unwrap_or_default();
        metadata.last_clean_shutdown = false;
        metadata.maybe_last_recovery_action = Some(action);
        self.save_runtime_metadata(&metadata, mode)?;

        Ok(marker)
    }

    /// Load the last persisted recovery marker, if present.
    pub fn load_recovery_marker(&self) -> Result<Option<RecoveryMarker>, StorageError> {
        self.get_bytes(StorageNamespace::Runtime, RECOVERY_MARKER_KEY)?
            .map(|bytes| decode_recovery_marker(&bytes))
            .transpose()
    }

    /// Remove any recovery marker after a successful repair or clean shutdown.
    pub fn clear_recovery_marker(&self, mode: PersistMode) -> Result<(), StorageError> {
        self.runtime.remove(RECOVERY_MARKER_KEY).map_err(|error| {
            backend_failure(
                StorageNamespace::Runtime,
                error,
                StorageRecoveryAction::Restart,
            )
        })?;
        self.persist(StorageNamespace::Runtime, mode)
    }

    /// Record a clean shutdown and clear outstanding recovery markers.
    pub fn mark_clean_shutdown(&self, mode: PersistMode) -> Result<(), StorageError> {
        self.clear_recovery_marker(mode)?;
        let mut metadata = self.load_runtime_metadata()?.unwrap_or_default();
        metadata.last_clean_shutdown = true;
        self.save_runtime_metadata(&metadata, mode)
    }

    fn ensure_schema(&self) -> Result<(), StorageError> {
        let Some(bytes) = self.get_bytes(StorageNamespace::Schema, SCHEMA_VERSION_KEY)? else {
            let version = SchemaVersion::CURRENT.get().to_string().into_bytes();
            return self.put_bytes(
                StorageNamespace::Schema,
                SCHEMA_VERSION_KEY,
                version,
                PersistMode::Sync,
            );
        };

        validate_schema_version(&bytes)
    }

    fn put_bytes(
        &self,
        namespace: StorageNamespace,
        key: &str,
        bytes: Vec<u8>,
        mode: PersistMode,
    ) -> Result<(), StorageError> {
        self.keyspace(namespace)
            .insert(key, bytes)
            .map_err(|error| backend_failure(namespace, error, StorageRecoveryAction::Restart))?;
        self.persist(namespace, mode)
    }

    fn get_bytes(
        &self,
        namespace: StorageNamespace,
        key: &str,
    ) -> Result<Option<Vec<u8>>, StorageError> {
        self.keyspace(namespace)
            .get(key)
            .map(|maybe_bytes| maybe_bytes.map(|bytes| bytes.as_ref().to_vec()))
            .map_err(|error| backend_failure(namespace, error, StorageRecoveryAction::Restart))
    }

    fn persist(&self, namespace: StorageNamespace, mode: PersistMode) -> Result<(), StorageError> {
        let Some(mode) = fjall_persist_mode(mode) else {
            return Ok(());
        };

        self.db
            .persist(mode)
            .map_err(|error| backend_failure(namespace, error, StorageRecoveryAction::Restart))
    }

    fn keyspace(&self, namespace: StorageNamespace) -> &Keyspace {
        match namespace {
            StorageNamespace::Headers => &self.headers,
            StorageNamespace::BlockIndex => &self.block_index,
            StorageNamespace::Chainstate => &self.chainstate,
            StorageNamespace::Wallet => &self.wallet,
            StorageNamespace::Metrics => &self.metrics,
            StorageNamespace::Runtime => &self.runtime,
            StorageNamespace::Schema => &self.schema,
        }
    }

    #[cfg(test)]
    fn write_raw_for_test(
        &self,
        namespace: StorageNamespace,
        key: &str,
        bytes: Vec<u8>,
    ) -> Result<(), StorageError> {
        self.put_bytes(namespace, key, bytes, PersistMode::Sync)
    }

    #[cfg(test)]
    fn write_schema_version_for_test(&self, version: u32) -> Result<(), StorageError> {
        self.put_bytes(
            StorageNamespace::Schema,
            SCHEMA_VERSION_KEY,
            version.to_string().into_bytes(),
            PersistMode::Sync,
        )
    }
}

fn open_keyspace(db: &Database, namespace: StorageNamespace) -> Result<Keyspace, StorageError> {
    db.keyspace(namespace.as_str(), KeyspaceCreateOptions::default)
        .map_err(|error| backend_failure(namespace, error, StorageRecoveryAction::Restart))
}

fn fjall_persist_mode(mode: PersistMode) -> Option<FjallPersistMode> {
    match mode {
        PersistMode::Buffered => None,
        PersistMode::Flush => Some(FjallPersistMode::Buffer),
        PersistMode::Sync => Some(FjallPersistMode::SyncAll),
    }
}

fn validate_schema_version(bytes: &[u8]) -> Result<(), StorageError> {
    let text =
        str::from_utf8(bytes).map_err(|error| corruption(StorageNamespace::Schema, error))?;
    let version = text
        .parse::<u32>()
        .map_err(|error| corruption(StorageNamespace::Schema, error))?;
    let actual = SchemaVersion::new(version)?;
    if actual != SchemaVersion::CURRENT {
        return Err(StorageError::schema_mismatch(
            SchemaVersion::CURRENT,
            actual,
        ));
    }

    Ok(())
}

fn block_key(block_hash: BlockHash) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";

    let mut key = String::with_capacity("block:".len() + 64);
    key.push_str("block:");
    for byte in block_hash.as_bytes() {
        key.push(HEX[(byte >> 4) as usize] as char);
        key.push(HEX[(byte & 0x0f) as usize] as char);
    }
    key
}

fn backend_failure(
    namespace: StorageNamespace,
    error: fjall::Error,
    action: StorageRecoveryAction,
) -> StorageError {
    StorageError::BackendFailure {
        namespace,
        message: error.to_string(),
        action,
    }
}

fn corruption(namespace: StorageNamespace, detail: impl std::fmt::Display) -> StorageError {
    StorageError::Corruption {
        namespace,
        detail: detail.to_string(),
        action: StorageRecoveryAction::Repair,
    }
}

#[cfg(test)]
mod tests;
