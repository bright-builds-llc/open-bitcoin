// Parity breadcrumbs:
// - packages/bitcoin-knots/src/index/blockfilterindex.cpp
// - packages/bitcoin-knots/src/blockfilter.cpp

//! Checked in-memory freshness, independent of append revision and persisted schema.

use super::{PublicationControl, StorageError, publication_failure};

/// Only observations from actual backend/immutable validation sources earn this fact.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum BasicFilterReadFault {
    Backend,
    Integrity,
}

impl PublicationControl {
    pub(in crate::storage::fjall_store::filters) fn observe_read_fault(
        &mut self,
        maybe_fault: Option<BasicFilterReadFault>,
    ) -> Result<(), StorageError> {
        let Some(fault) = maybe_fault else {
            return Ok(());
        };
        if self.maybe_read_fault == Some(BasicFilterReadFault::Integrity)
            || self.maybe_read_fault == Some(fault)
        {
            return Ok(());
        }
        let first = self.maybe_read_fault.is_none();
        self.maybe_read_fault = Some(fault);
        if first {
            let maybe_protection = self.maybe_append_identity.map(|identity| {
                identity
                    .owner
                    .maybe_effective_protection()
                    .unwrap_or(identity.owner.saved_protection())
            });
            self.invalidate_read_integrity()?;
            self.invalidate_append()?;
            self.maybe_read_fault_protection = maybe_protection;
        }
        Ok(())
    }

    /// Caller must finish complete mandatory validation and all fallible effects first.
    pub(in crate::storage::fjall_store::filters) fn reset_verified_forest(
        &mut self,
        retained: super::super::retention::RetainedBasicFilterObservation,
    ) {
        self.maybe_read_fault = None;
        self.maybe_read_fault_protection = None;
        self.read_integrity = true;
        self.install_retention(retained);
    }

    /// Clear every clone's live authority before any potentially ambiguous write.
    /// Immutable observations survive ordinary coins/metadata revisions.
    pub(crate) fn invalidate_append(&mut self) -> Result<(), StorageError> {
        self.maybe_append_identity = None;
        self.maybe_pending_coins = None;
        self.maybe_completed_metadata = None;
        let Some(revision) = self.revision.checked_add(1) else {
            self.poisoned = true;
            return Err(publication_failure("BASIC durable revision exhausted"));
        };
        self.revision = revision;
        Ok(())
    }

    /// Raw/recovery effects revoke the trusted forest before advancing its lifetime.
    pub(crate) fn invalidate_read_integrity(&mut self) -> Result<(), StorageError> {
        self.maybe_read_fault_protection = None;
        self.read_integrity = false;
        self.retention = super::super::retention::RetainedBasicFilterObservation::Incomplete;
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

    #[cfg(test)]
    pub(in crate::storage::fjall_store::filters) fn reserve_last_integrity_epoch_for_test(
        &mut self,
    ) {
        self.integrity_epoch = u64::MAX - 1;
    }
}
