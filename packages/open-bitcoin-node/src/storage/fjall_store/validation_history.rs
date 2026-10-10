// Parity breadcrumbs:
// - packages/bitcoin-knots/src/chain.h
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp

//! Retained scripts-valid history is independent of BASIC and payload retention.

use super::{
    FjallNodeStore, FjallPersistMode, StorageError, StorageNamespace, StorageRecoveryAction,
    backend_failure,
};
use crate::storage::validation_history as codec;
use open_bitcoin_core::primitives::BlockHash;
use std::sync::{Arc, MutexGuard};

use codec::{
    AcceptedValidationBatch, AdmittedValidationHeaders, BlockValidationIdentity,
    StoredValidationStatus, ValidationCoverage, ValidationHistoryRecord, ValidationProvenance,
    history_corruption,
};

#[derive(Default)]
pub(crate) struct HistoryControl {
    pub(super) maybe_coverage: Option<codec::ValidationCoverage>,
    pub(super) poisoned: bool,
    revision: u64,
    #[cfg(test)]
    maybe_fault: Option<HistoryPublicationFault>,
}

/// Store-bound recovery evidence; raw identities and snapshots cannot create it.
pub(crate) struct RecoveredValidationHistory {
    store: FjallNodeStore,
    revision: u64,
}

impl RecoveredValidationHistory {
    pub(crate) fn is_complete(&self) -> Result<bool, StorageError> {
        let control = self.store.history_guard()?;
        if control.revision != self.revision {
            return Err(history_corruption(
                "stale validation-history recovery capability",
            ));
        }
        Ok(control.maybe_coverage == Some(ValidationCoverage::Complete))
    }

    pub(crate) fn belongs_to(&self, store: &FjallNodeStore) -> bool {
        self.store.shares_validation_history_store(store)
    }
}

#[cfg(test)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum HistoryPublicationFault {
    BeforeCommit,
    AfterCommit,
}

impl FjallNodeStore {
    /// Public metadata sinks carry raw positions, not acceptance authority.
    /// Caller holds the existing publication guard; never reacquire it here.
    pub(super) fn check_validation_coverage_for_metadata(
        &self,
        positions: &[open_bitcoin_core::chainstate::ChainPosition],
    ) -> Result<(), StorageError> {
        let control = self.history_guard()?;
        if control.maybe_coverage != Some(ValidationCoverage::Complete) {
            return Ok(());
        }
        let mut authenticated = !positions.is_empty();
        for position in positions {
            // Sparse raw metadata is missing authority, not a corrupt ledger row.
            if self
                .maybe_validation_history_record(position.block_hash)?
                .is_none_or(|record| {
                    let identity = record.identity();
                    record.status() != StoredValidationStatus::ScriptsValid
                        || identity.hash() != position.block_hash
                        || identity.parent_hash() != position.previous_block_hash()
                        || identity.height() != position.height
                })
            {
                authenticated = false;
                break;
            }
        }
        drop(control);
        if !authenticated {
            self.invalidate_validation_coverage()?;
        }
        Ok(())
    }

    #[cfg(test)]
    pub(crate) fn remove_validation_filter_for_test(
        &self,
        hash: BlockHash,
    ) -> Result<(), StorageError> {
        self.remove_bytes(
            StorageNamespace::BlockIndex,
            &crate::storage::filter_index::record_key(hash),
            crate::PersistMode::Sync,
        )
    }
    /// Preserve coverage only for the serialized network owner's admitted headers.
    pub(crate) fn save_trusted_header_snapshot(
        &self,
        snapshot: &crate::network::validation_history::TrustedHeaderSnapshot,
    ) -> Result<(), StorageError> {
        if !snapshot.belongs_to(self) {
            return Err(history_corruption("foreign trusted header snapshot"));
        }
        let headers = super::encode_header_entries(snapshot.entries())?;
        let index = super::encode_block_index_entries(snapshot.entries())?;
        let _publication = self.filter_publication_guard()?;
        let control = self.history_guard()?;
        if control.maybe_coverage == Some(ValidationCoverage::Complete) {
            for entry in snapshot.entries() {
                let identity = BlockValidationIdentity::new(
                    entry.block_hash,
                    entry.header.previous_block_hash,
                    entry.height,
                )?;
                if self
                    .maybe_validation_history_record(entry.block_hash)?
                    .is_none_or(|record| record.identity() != identity)
                {
                    return Err(history_corruption("unadmitted trusted header snapshot"));
                }
            }
        }
        let mut batch = self.db.batch().durability(Some(FjallPersistMode::SyncAll));
        batch.insert(&self.headers, super::SNAPSHOT_KEY, headers);
        batch.insert(&self.block_index, super::SNAPSHOT_KEY, index);
        batch
            .commit()
            .map_err(|error| backend_failure(StorageNamespace::Headers, error))
    }

    pub(crate) fn shares_validation_history_store(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.validation_history, &other.validation_history)
    }

    pub(super) fn recover_validation_history(&self) -> Result<(), StorageError> {
        let mut control = self.history_guard_unrecovered()?;
        let mut records = false;
        for guard in self.block_index.prefix("validated_block:") {
            let (key, bytes) = guard
                .into_inner()
                .map_err(|error| backend_failure(StorageNamespace::BlockIndex, error))?;
            let key = std::str::from_utf8(&key).map_err(history_corruption)?;
            let record = codec::decode_record(key, &bytes)?;
            let id = record.identity();
            if let Some(parent) = self.maybe_validation_history_record(id.parent_hash())?
                && parent.identity().height().checked_add(1) != Some(id.height())
            {
                return Err(history_corruption(
                    "conflicting validation-history parent height",
                ));
            }
            records = true;
        }
        for guard in self.block_index.prefix("validated_coverage:") {
            let key = guard
                .key()
                .map_err(|error| backend_failure(StorageNamespace::BlockIndex, error))?;
            if key.as_ref() != codec::COVERAGE_KEY.as_bytes() {
                return Err(history_corruption(
                    "unsupported validation-history coverage key",
                ));
            }
        }
        let maybe_coverage = self
            .get_bytes(StorageNamespace::BlockIndex, codec::COVERAGE_KEY)?
            .map(|bytes| codec::decode_coverage(&bytes))
            .transpose()?;
        if records && maybe_coverage.is_none() {
            return Err(history_corruption("validation-history coverage missing"));
        }
        let coverage = match maybe_coverage {
            Some(coverage) => coverage,
            None if self.validation_store_is_genuinely_empty()? => {
                let coverage = ValidationCoverage::Complete;
                let mut batch = self.db.batch().durability(Some(FjallPersistMode::SyncAll));
                batch.insert(
                    &self.block_index,
                    codec::COVERAGE_KEY,
                    codec::encode_coverage(coverage),
                );
                batch
                    .commit()
                    .map_err(|error| backend_failure(StorageNamespace::BlockIndex, error))?;
                coverage
            }
            None => ValidationCoverage::UnknownLegacy,
        };
        control.maybe_coverage = Some(coverage);
        Ok(())
    }

    fn validation_store_is_genuinely_empty(&self) -> Result<bool, StorageError> {
        for namespace in [
            StorageNamespace::Headers,
            StorageNamespace::BlockIndex,
            StorageNamespace::Chainstate,
            StorageNamespace::Coins,
            StorageNamespace::Wallet,
            StorageNamespace::Metrics,
            StorageNamespace::Mempool,
            StorageNamespace::Runtime,
            StorageNamespace::Schema,
        ] {
            if let Some(guard) = self.keyspace(namespace).iter().next() {
                guard
                    .key()
                    .map_err(|error| backend_failure(namespace, error))?;
                return Ok(false);
            }
        }
        Ok(true)
    }

    pub(crate) fn recovered_validation_history(
        &self,
    ) -> Result<RecoveredValidationHistory, StorageError> {
        let control = self.history_guard()?;
        Ok(RecoveredValidationHistory {
            store: self.clone(),
            revision: control.revision,
        })
    }

    /// Two-state coverage never overwrites positive scripts-valid facts.
    /// Caller may already hold the BASIC publication guard; do not reacquire it.
    pub(crate) fn invalidate_validation_coverage(&self) -> Result<(), StorageError> {
        let mut control = self.history_guard_unrecovered()?;
        if control.maybe_coverage != Some(ValidationCoverage::Complete) {
            return Ok(());
        }
        let Some(revision) = control.revision.checked_add(1) else {
            control.poisoned = true;
            return Err(history_failure("validation-history revision exhausted"));
        };
        control.revision = revision;
        control.maybe_coverage = Some(ValidationCoverage::UnknownLegacy);
        let mut batch = self.db.batch().durability(Some(FjallPersistMode::SyncAll));
        batch.insert(
            &self.block_index,
            codec::COVERAGE_KEY,
            codec::encode_coverage(ValidationCoverage::UnknownLegacy),
        );
        if let Err(error) = batch.commit() {
            control.poisoned = true;
            return Err(backend_failure(StorageNamespace::BlockIndex, error));
        }
        Ok(())
    }

    /// Constant per-hash provenance read. Backend errors remain distinct from absence.
    pub(crate) fn validation_provenance(
        &self,
        hash: BlockHash,
    ) -> Result<ValidationProvenance, StorageError> {
        let control = self.history_guard()?;
        let coverage = control
            .maybe_coverage
            .ok_or_else(|| history_corruption("unrecovered validation history"))?;
        let Some(record) = self.maybe_validation_history_record(hash)? else {
            return Ok(coverage.absent_provenance());
        };
        if record.status() == StoredValidationStatus::KnownOnly
            && coverage != ValidationCoverage::Complete
        {
            return Ok(ValidationProvenance::UnknownLegacy);
        }
        Ok(record.provenance())
    }

    pub(crate) fn maybe_validation_history_record(
        &self,
        hash: BlockHash,
    ) -> Result<Option<ValidationHistoryRecord>, StorageError> {
        let key = codec::record_key(hash);
        self.block_index
            .get(&key)
            .map_err(|error| backend_failure(StorageNamespace::BlockIndex, error))?
            .map(|bytes| codec::decode_record(&key, &bytes))
            .transpose()
    }

    /// Borrowed capability remains available to the owner for bounded retry on error.
    pub(crate) fn publish_validation_history(
        &self,
        accepted: &AcceptedValidationBatch,
    ) -> Result<(), StorageError> {
        if !accepted.belongs_to(self) {
            return Err(history_corruption("foreign accepted validation batch"));
        }
        accepted.validate()?;
        self.publish_history_identities(accepted.identities(), StoredValidationStatus::ScriptsValid)
    }

    pub(crate) fn publish_admitted_validation_headers(
        &self,
        admitted: &AdmittedValidationHeaders,
    ) -> Result<(), StorageError> {
        if !admitted.belongs_to(self) {
            return Err(history_corruption("foreign admitted validation headers"));
        }
        codec::validate_batch_size(admitted.identities().len())?;
        self.publish_history_identities(admitted.identities(), StoredValidationStatus::KnownOnly)
    }

    fn publish_history_identities(
        &self,
        identities: &[BlockValidationIdentity],
        status: StoredValidationStatus,
    ) -> Result<(), StorageError> {
        let _publication = self.filter_publication_guard()?;
        let mut control = self.history_guard()?;
        let coverage = control
            .maybe_coverage
            .ok_or_else(|| history_corruption("unrecovered validation history"))?;
        if status == StoredValidationStatus::KnownOnly && coverage != ValidationCoverage::Complete {
            return Err(history_corruption(
                "legacy headers lack complete validation coverage",
            ));
        }
        let mut batch = self.db.batch().durability(Some(FjallPersistMode::SyncAll));
        batch.insert(
            &self.block_index,
            codec::COVERAGE_KEY,
            codec::encode_coverage(coverage),
        );
        for (index, identity) in identities.iter().enumerate() {
            if identities[..index]
                .iter()
                .any(|previous| previous.hash() == identity.hash())
            {
                return Err(history_corruption(
                    "duplicate validation-history publication identity",
                ));
            }
            let next = ValidationHistoryRecord::new(*identity, status);
            codec::validate_upgrade(self.maybe_validation_history_record(identity.hash())?, next)?;
            batch.insert(
                &self.block_index,
                codec::record_key(identity.hash()),
                codec::encode_record(next),
            );
        }
        #[cfg(test)]
        if control.maybe_fault == Some(HistoryPublicationFault::BeforeCommit) {
            control.maybe_fault = None;
            return Err(history_failure(
                "injected validation-history failure before commit",
            ));
        }
        if let Err(error) = batch.commit() {
            control.poisoned = true;
            return Err(backend_failure(StorageNamespace::BlockIndex, error));
        }
        #[cfg(test)]
        if control.maybe_fault == Some(HistoryPublicationFault::AfterCommit) {
            control.maybe_fault = None;
            control.poisoned = true;
            return Err(history_failure(
                "injected validation-history failure after commit",
            ));
        }
        Ok(())
    }

    fn history_guard(&self) -> Result<MutexGuard<'_, HistoryControl>, StorageError> {
        let control = self.history_guard_unrecovered()?;
        if control.maybe_coverage.is_none() {
            return Err(history_corruption("unrecovered validation history"));
        }
        Ok(control)
    }

    fn history_guard_unrecovered(&self) -> Result<MutexGuard<'_, HistoryControl>, StorageError> {
        let control = self
            .validation_history
            .lock()
            .map_err(|_| history_failure("validation-history mutex poisoned"))?;
        if control.poisoned {
            return Err(history_failure(
                "validation-history publication requires reopen",
            ));
        }
        Ok(control)
    }

    #[cfg(test)]
    pub(crate) fn set_validation_history_fault(&self, fault: HistoryPublicationFault) {
        self.validation_history
            .lock()
            .expect("history control")
            .maybe_fault = Some(fault);
    }
}

fn history_failure(message: &str) -> StorageError {
    StorageError::BackendFailure {
        namespace: StorageNamespace::BlockIndex,
        message: message.to_owned(),
        action: StorageRecoveryAction::Restart,
    }
}
