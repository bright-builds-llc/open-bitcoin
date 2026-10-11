// Parity breadcrumbs:
// - packages/bitcoin-knots/src/index/blockfilterindex.cpp
// - packages/bitcoin-knots/src/blockfilter.cpp

//! Checked in-memory freshness, independent of append revision and persisted schema.

use super::{PublicationControl, StorageError, publication_failure};

impl PublicationControl {
    /// Raw/recovery effects revoke the trusted forest before advancing its lifetime.
    pub(crate) fn invalidate_read_integrity(&mut self) -> Result<(), StorageError> {
        self.read_integrity = false;
        self.advance_integrity_epoch()
    }

    /// Disable revokes old captures without changing the existing RPC integrity contract.
    pub(crate) fn advance_integrity_epoch(&mut self) -> Result<(), StorageError> {
        let Some(epoch) = self.integrity_epoch.checked_add(1) else {
            self.read_integrity = false;
            self.poisoned = true;
            return Err(publication_failure("BASIC read integrity epoch exhausted"));
        };
        self.integrity_epoch = epoch;
        Ok(())
    }

    pub(in crate::storage::fjall_store::filters) fn read_integrity_epoch(&self) -> u64 {
        self.integrity_epoch
    }

    #[cfg(test)]
    pub(in crate::storage::fjall_store::filters) fn exhaust_integrity_epoch_for_test(&mut self) {
        self.integrity_epoch = u64::MAX;
    }
}
