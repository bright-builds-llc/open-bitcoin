// Parity breadcrumbs:
// - packages/bitcoin-knots/src/index/blockfilterindex.cpp
// - packages/bitcoin-knots/src/blockfilter.cpp

//! Concrete immutable BASIC reads. Scans never hydrate a full encoded index.

use super::{FjallNodeStore, StorageError, StorageNamespace, backend_failure};
use crate::storage::filter_index::{
    self as codec, StoredFilterRecord, StoredFilterState, index_corruption,
};
use open_bitcoin_core::{
    chainstate::{
        FilterCheckpoint, FilterRecoveryPlan, FilterRecoveryScan, IndexInputProtection,
        IndexPrefix, VerifiedChainstateFence,
    },
    primitives::BlockHash,
};

mod publication;
mod startup;
#[cfg(test)]
pub(crate) use publication::FilterPublicationFault;
pub(super) use publication::PublicationControl;

impl FjallNodeStore {
    pub(crate) fn maybe_basic_filter_state(
        &self,
    ) -> Result<Option<StoredFilterState>, StorageError> {
        self.get_bytes(StorageNamespace::BlockIndex, codec::STATE_KEY)?
            .map(|bytes| codec::decode_state(&bytes))
            .transpose()
    }

    /// Check explicit artifact presence; saved rows cannot create a cursor.
    pub(crate) fn basic_filter_artifacts(&self) -> Result<(bool, bool), StorageError> {
        Ok((
            self.filter_prefix_present(codec::RECORD_PREFIX)?,
            self.filter_prefix_present(codec::ACTIVE_PREFIX)?,
        ))
    }

    fn filter_prefix_present(&self, prefix: &str) -> Result<bool, StorageError> {
        match self.block_index.prefix(prefix).next() {
            None => Ok(false),
            Some(guard) => {
                guard
                    .into_inner()
                    .map_err(|error| backend_failure(StorageNamespace::BlockIndex, error))?;
                Ok(true)
            }
        }
    }

    /// Follow parent hashes cheaply, then prove commitments from genesis upward.
    /// Only hashes of ancestors and one record's bytes are retained.
    pub(crate) fn load_basic_filter_record(
        &self,
        hash: BlockHash,
    ) -> Result<Option<StoredFilterRecord>, StorageError> {
        let Some(bytes) = self.get_bytes(StorageNamespace::BlockIndex, &codec::record_key(hash))?
        else {
            return Ok(None);
        };
        let (mut height, mut parent) = codec::record_parent(&codec::record_key(hash), &bytes)?;
        let mut ancestors = vec![hash];
        while height != 0 {
            let key = codec::record_key(parent);
            let Some(bytes) = self.get_bytes(StorageNamespace::BlockIndex, &key)? else {
                return Err(index_corruption("missing BASIC predecessor record"));
            };
            let (parent_height, next_parent) = codec::record_parent(&key, &bytes)?;
            if parent_height != height - 1 {
                return Err(index_corruption("BASIC predecessor height"));
            }
            ancestors.push(parent);
            height = parent_height;
            parent = next_parent;
        }
        let mut maybe_previous = None;
        let mut maybe_record = None;
        for ancestor in ancestors.into_iter().rev() {
            let key = codec::record_key(ancestor);
            let Some(bytes) = self.get_bytes(StorageNamespace::BlockIndex, &key)? else {
                return Err(index_corruption("missing BASIC ancestor record"));
            };
            let record = codec::decode_record(&key, &bytes, maybe_previous.as_ref())?;
            maybe_previous = Some(record.identity());
            maybe_record = Some(record);
        }
        Ok(maybe_record)
    }

    pub(crate) fn maybe_basic_filter_checkpoint(
        &self,
    ) -> Result<Option<FilterCheckpoint>, StorageError> {
        let Some(state) = self.maybe_basic_filter_state()? else {
            return Ok(None);
        };
        let checkpoint = self.checkpoint_from_state(state)?;
        self.validate_basic_filter_projection(checkpoint)?;
        Ok(Some(checkpoint))
    }

    fn checkpoint_from_state(
        &self,
        state: StoredFilterState,
    ) -> Result<FilterCheckpoint, StorageError> {
        let prefix = match state.maybe_endpoint {
            None => IndexPrefix::Empty,
            Some((height, hash)) => {
                if height > state.fence_height {
                    return Err(index_corruption("BASIC checkpoint exceeds saved fence"));
                }
                if height == state.fence_height && hash != state.fence_hash {
                    return Err(index_corruption(
                        "BASIC checkpoint differs from same-height saved fence",
                    ));
                }
                let Some(record) = self.load_basic_filter_record(hash)? else {
                    return Err(index_corruption("missing BASIC checkpoint record"));
                };
                if record.identity().height() != height {
                    return Err(index_corruption("BASIC checkpoint height"));
                }
                IndexPrefix::Committed(record.identity())
            }
        };
        let checkpoint = FilterCheckpoint::new(prefix);
        if !state.protection.covers(checkpoint.input_protection()) {
            return Err(index_corruption("weak BASIC state protection"));
        }
        Ok(checkpoint)
    }

    fn validate_basic_filter_projection(
        &self,
        checkpoint: FilterCheckpoint,
    ) -> Result<(), StorageError> {
        let IndexPrefix::Committed(endpoint) = checkpoint.prefix() else {
            return Ok(());
        };
        let mut maybe_previous = None;
        for height in 0..=endpoint.height() {
            let hash = self.basic_filter_projection(height)?;
            let key = codec::record_key(hash);
            let Some(bytes) = self.get_bytes(StorageNamespace::BlockIndex, &key)? else {
                return Err(index_corruption("missing BASIC projected row"));
            };
            let record = codec::decode_record(&key, &bytes, maybe_previous.as_ref())?;
            if record.identity().height() != height {
                return Err(index_corruption("BASIC projection height sequence"));
            }
            maybe_previous = Some(record.identity());
        }
        if maybe_previous != Some(endpoint) {
            return Err(index_corruption("BASIC projection differs from checkpoint"));
        }
        Ok(())
    }

    #[cfg_attr(not(test), allow(dead_code))] // Serving is a later phase.
    pub(crate) fn maybe_active_basic_filter_record(
        &self,
        height: u32,
    ) -> Result<Option<StoredFilterRecord>, StorageError> {
        let Some(checkpoint) = self.maybe_basic_filter_checkpoint()? else {
            return Ok(None);
        };
        let IndexPrefix::Committed(endpoint) = checkpoint.prefix() else {
            return Ok(None);
        };
        if height > endpoint.height() {
            return Ok(None);
        }
        let hash = self.basic_filter_projection(height)?;
        let Some(record) = self.load_basic_filter_record(hash)? else {
            return Err(index_corruption("missing BASIC projected record"));
        };
        if record.identity().height() != height {
            return Err(index_corruption("BASIC projection record height"));
        }
        Ok(Some(record))
    }

    pub(crate) fn basic_filter_projection(&self, height: u32) -> Result<BlockHash, StorageError> {
        let key = codec::active_key(height);
        let Some(bytes) = self.get_bytes(StorageNamespace::BlockIndex, &key)? else {
            return Err(index_corruption("missing BASIC projection"));
        };
        codec::decode_projection(&key, &bytes)
    }

    /// Validate all immutable rows, including unclaimed suffix/fork rows.
    pub(crate) fn validate_basic_filter_records(&self) -> Result<(), StorageError> {
        #[cfg(test)]
        self.filter_integrity_reads
            .store(0, std::sync::atomic::Ordering::Relaxed);
        // Every row proves its own commitments and its single decreasing-height
        // edge. A complete scan therefore proves all ancestry without a cache.
        for guard in self.block_index.prefix(codec::RECORD_PREFIX) {
            self.count_filter_integrity_read();
            let (key, bytes) = guard
                .into_inner()
                .map_err(|error| backend_failure(StorageNamespace::BlockIndex, error))?;
            let key = std::str::from_utf8(&key)
                .map_err(|_| index_corruption("BASIC record key encoding"))?;
            let parsed = codec::parse_record(key, &bytes)?;
            if parsed.height == 0 {
                parsed.verify_parent(None)?;
                continue;
            }
            let parent_key = codec::record_key(parsed.parent);
            let Some(parent_bytes) = self.get_bytes(StorageNamespace::BlockIndex, &parent_key)?
            else {
                return Err(index_corruption("missing BASIC predecessor record"));
            };
            let parent = codec::parse_record_fields(&parent_key, &parent_bytes)?;
            parsed.verify_parent(Some(&parent))?;
        }
        // Validate even hidden projection suffix. Immutable rows were globally
        // proven above; each projection now needs only a bounded direct read.
        for guard in self.block_index.prefix(codec::ACTIVE_PREFIX) {
            self.count_filter_integrity_read();
            let (key, bytes) = guard
                .into_inner()
                .map_err(|error| backend_failure(StorageNamespace::BlockIndex, error))?;
            let key = std::str::from_utf8(&key)
                .map_err(|_| index_corruption("BASIC projection key encoding"))?;
            let hash = codec::decode_projection(key, &bytes)?;
            let record_key = codec::record_key(hash);
            let Some(record_bytes) = self.get_bytes(StorageNamespace::BlockIndex, &record_key)?
            else {
                return Err(index_corruption("missing BASIC suffix projection record"));
            };
            let record = codec::parse_record_fields(&record_key, &record_bytes)?;
            if codec::active_key(record.height) != key {
                return Err(index_corruption("BASIC suffix projection record height"));
            }
        }
        Ok(())
    }

    fn count_filter_integrity_read(&self) {
        #[cfg(test)]
        self.filter_integrity_reads
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    }

    #[cfg(test)]
    pub(in crate::storage::fjall_store) fn count_filter_index_point_read(
        &self,
        namespace: StorageNamespace,
        key: &str,
    ) {
        if namespace == StorageNamespace::BlockIndex && key.starts_with("basic_filter:v1:") {
            self.count_filter_integrity_read();
        }
    }

    pub(crate) fn scan_basic_filter_checkpoint(
        &self,
        checkpoint: FilterCheckpoint,
        protection: IndexInputProtection,
        fence: &VerifiedChainstateFence<'_>,
    ) -> Result<FilterRecoveryPlan, StorageError> {
        let mut scan = FilterRecoveryScan::new(checkpoint, Some(protection), fence)
            .map_err(index_corruption)?;
        if let IndexPrefix::Committed(endpoint) = checkpoint.prefix() {
            let mut maybe_previous = None;
            for height in 0..=endpoint.height() {
                let hash = self.basic_filter_projection(height)?;
                let key = codec::record_key(hash);
                let Some(bytes) = self.get_bytes(StorageNamespace::BlockIndex, &key)? else {
                    return Err(index_corruption("missing BASIC projected record"));
                };
                let record = codec::decode_record(&key, &bytes, maybe_previous.as_ref())?;
                scan.push(record.identity(), hash)
                    .map_err(index_corruption)?;
                maybe_previous = Some(record.identity());
            }
        }
        Ok(scan.finish())
    }
}

#[cfg(test)]
mod tests;
