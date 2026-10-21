// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

//! Backend construction behind the required application lease.

use super::*;

impl FjallNodeStore {
    /// Lower-level storage/fixture infrastructure: opens, recovers and may migrate.
    /// This generic API provides no canonical identity or ordinary-admission guarantee.
    /// Ordinary product paths must use [`Self::open_ordinary`].
    pub fn open(path: impl AsRef<Path>) -> Result<Self, StorageError> {
        let store = Self::open_unrecovered(path)?;
        store.recover_validation_history()?;
        store.ensure_schema()?;
        Ok(store)
    }

    pub(super) fn open_unrecovered(path: impl AsRef<Path>) -> Result<Self, StorageError> {
        Self::open_with_lease(open_lease::OperationLease::acquire(path.as_ref())?)
    }

    pub(super) fn open_with_lease(lease: open_lease::OperationLease) -> Result<Self, StorageError> {
        #[cfg(test)]
        super::super::chain_compatibility::record_backend_builder_call();
        #[cfg(test)]
        assert!(
            !super::super::chain_compatibility::ORDINARY_BACKEND_OPEN_SPY.get(),
            "backend reached during ordinary refusal"
        );
        let db = Database::builder(lease.source())
            .open()
            .map_err(|error| backend_failure(StorageNamespace::Runtime, error))?;

        Ok(Self {
            payload_usage: Arc::new(Mutex::new(payload_usage::PayloadUsageState::new())),
            filter_publication: Arc::new(Mutex::new(filters::PublicationControl::default())),
            validation_history: Arc::default(),
            #[cfg(test)]
            filter_integrity_reads: Arc::default(),
            path: lease.source().to_path_buf(),
            headers: open_keyspace(&db, StorageNamespace::Headers)?,
            block_index: open_keyspace(&db, StorageNamespace::BlockIndex)?,
            chainstate: open_keyspace(&db, StorageNamespace::Chainstate)?,
            coins: open_keyspace(&db, StorageNamespace::Coins)?,
            wallet: open_keyspace(&db, StorageNamespace::Wallet)?,
            metrics: open_keyspace(&db, StorageNamespace::Metrics)?,
            mempool: open_keyspace(&db, StorageNamespace::Mempool)?,
            runtime: open_keyspace(&db, StorageNamespace::Runtime)?,
            schema: open_keyspace(&db, StorageNamespace::Schema)?,
            db,
            _lease: Arc::new(lease),
        })
    }
}
