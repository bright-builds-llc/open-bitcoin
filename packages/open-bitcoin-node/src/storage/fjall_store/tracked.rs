// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

//! Private same-open logical mutation boundary around native storage.

use super::inspection::InspectedRecords;
use crate::storage::StorageError;
use fjall::{KeyspaceCreateOptions, PersistMode, Readable, UserKey, UserValue};
use std::sync::{Arc, Mutex, MutexGuard};

#[derive(Default)]
struct Generation {
    value: u64,
    active_writers: u64,
    exhausted: bool,
}

#[derive(Default)]
struct Shared {
    generation: Mutex<Generation>,
    catalog: Mutex<()>,
}

/// Opaque observation bound to one actual leased open, never a physical counter.
pub(in crate::storage) struct MutationStamp {
    shared: Arc<Shared>,
    generation: u64,
}

#[derive(Debug)]
pub(in crate::storage) struct MutationError;

impl Shared {
    fn generation(&self) -> Result<MutexGuard<'_, Generation>, MutationError> {
        if self.catalog.is_poisoned() {
            return Err(MutationError);
        }
        let guard = self.generation.lock().map_err(|_| MutationError)?;
        if guard.exhausted {
            return Err(MutationError);
        }
        Ok(guard)
    }

    fn begin(self: &Arc<Self>) -> Result<MutationGuard, MutationError> {
        let mut generation = self.generation()?;
        let (Some(next), Some(active)) = (
            generation.value.checked_add(1),
            generation.active_writers.checked_add(1),
        ) else {
            generation.exhausted = true;
            return Err(MutationError);
        };
        generation.value = next;
        generation.active_writers = active;
        Ok(MutationGuard(Arc::clone(self)))
    }
}

struct MutationGuard(Arc<Shared>);

impl Drop for MutationGuard {
    fn drop(&mut self) {
        // A poisoned lock stays poisoned; cleanup must never panic during unwind.
        if let Ok(mut generation) = self.0.generation.lock() {
            if let Some(active) = generation.active_writers.checked_sub(1) {
                generation.active_writers = active;
            } else {
                generation.exhausted = true;
            }
        }
    }
}

fn tracking_error(_: MutationError) -> fjall::Error {
    std::io::Error::other("logical mutation observation unavailable").into()
}

#[derive(Clone)]
pub(crate) struct Database {
    raw: fjall::Database,
    shared: Arc<Shared>,
}

#[derive(Clone)]
pub(crate) struct Keyspace {
    raw: fjall::Keyspace,
    shared: Arc<Shared>,
}

pub(crate) struct Snapshot {
    raw: fjall::Snapshot,
}

pub(crate) struct OwnedWriteBatch {
    raw: fjall::OwnedWriteBatch,
    shared: Arc<Shared>,
    foreign: bool,
}

impl Database {
    pub(super) fn from_raw_opened(raw: fjall::Database) -> Self {
        Self {
            raw,
            shared: Arc::default(),
        }
    }

    pub(in crate::storage) fn stamp(&self) -> Result<MutationStamp, MutationError> {
        let generation = self.shared.generation()?;
        if generation.active_writers != 0 {
            return Err(MutationError);
        }
        Ok(MutationStamp {
            shared: Arc::clone(&self.shared),
            generation: generation.value,
        })
    }

    pub(in crate::storage) fn check_stamp(
        &self,
        stamp: &MutationStamp,
    ) -> Result<(), MutationError> {
        if !Arc::ptr_eq(&self.shared, &stamp.shared) {
            return Err(MutationError);
        }
        let generation = self.shared.generation()?;
        if generation.active_writers != 0 || generation.value != stamp.generation {
            return Err(MutationError);
        }
        Ok(())
    }

    pub(crate) fn keyspace(
        &self,
        name: &str,
        options: impl FnOnce() -> KeyspaceCreateOptions,
    ) -> fjall::Result<Keyspace> {
        let _catalog = self
            .shared
            .catalog
            .lock()
            .map_err(|_| tracking_error(MutationError))?;
        // Catalog serialization makes this native create-capable lookup read-only.
        let _mutation = if self.raw.keyspace_exists(name) {
            None
        } else {
            Some(self.shared.begin().map_err(tracking_error)?)
        };
        let raw = self.raw.keyspace(name, options)?;
        Ok(Keyspace {
            raw,
            shared: Arc::clone(&self.shared),
        })
    }

    pub(crate) fn batch(&self) -> OwnedWriteBatch {
        OwnedWriteBatch {
            raw: self.raw.batch(),
            shared: Arc::clone(&self.shared),
            foreign: false,
        }
    }

    pub(crate) fn snapshot(&self) -> Snapshot {
        Snapshot {
            raw: self.raw.snapshot(),
        }
    }

    pub(crate) fn persist(&self, mode: PersistMode) -> fjall::Result<()> {
        self.raw.persist(mode)
    }

    pub(crate) fn list_keyspace_names(&self) -> Vec<String> {
        self.raw
            .list_keyspace_names()
            .iter()
            .map(ToString::to_string)
            .collect()
    }

    #[cfg(test)]
    pub(crate) fn keyspace_count(&self) -> usize {
        self.raw.keyspace_count()
    }

    #[cfg(test)]
    pub(crate) fn keyspace_exists(&self, name: &str) -> bool {
        self.raw.keyspace_exists(name)
    }

    #[cfg(test)]
    pub(crate) fn visible_seqno(&self) -> u64 {
        self.raw.visible_seqno()
    }

    #[cfg(test)]
    pub(crate) fn seqno(&self) -> u64 {
        self.raw.seqno()
    }

    #[cfg(test)]
    pub(crate) fn delete_keyspace(&self, keyspace: Keyspace) -> fjall::Result<()> {
        if !Arc::ptr_eq(&self.shared, &keyspace.shared) {
            return Err(tracking_error(MutationError));
        }
        let _catalog = self
            .shared
            .catalog
            .lock()
            .map_err(|_| tracking_error(MutationError))?;
        let _mutation = self.shared.begin().map_err(tracking_error)?;
        self.raw.delete_keyspace(keyspace.raw)
    }
}

impl Keyspace {
    pub(crate) fn insert<K: Into<UserKey>, V: Into<UserValue>>(
        &self,
        key: K,
        value: V,
    ) -> fjall::Result<()> {
        let _mutation = self.shared.begin().map_err(tracking_error)?;
        self.raw.insert(key, value)
    }

    pub(crate) fn remove<K: Into<UserKey>>(&self, key: K) -> fjall::Result<()> {
        let _mutation = self.shared.begin().map_err(tracking_error)?;
        self.raw.remove(key)
    }

    pub(crate) fn get<K: AsRef<[u8]>>(&self, key: K) -> fjall::Result<Option<UserValue>> {
        self.raw.get(key)
    }

    pub(crate) fn contains_key<K: AsRef<[u8]>>(&self, key: K) -> fjall::Result<bool> {
        self.raw.contains_key(key)
    }

    pub(crate) fn size_of<K: AsRef<[u8]>>(&self, key: K) -> fjall::Result<Option<u32>> {
        self.raw.size_of(key)
    }

    pub(crate) fn iter(&self) -> fjall::Iter {
        self.raw.iter()
    }

    pub(crate) fn prefix<K: AsRef<[u8]>>(&self, prefix: K) -> fjall::Iter {
        self.raw.prefix(prefix)
    }

    #[cfg(test)]
    pub(crate) fn clear(&self) -> fjall::Result<()> {
        let _mutation = self.shared.begin().map_err(tracking_error)?;
        self.raw.clear()
    }

    #[cfg(test)]
    fn remove_weak<K: Into<UserKey>>(&self, key: K) -> fjall::Result<()> {
        let _mutation = self.shared.begin().map_err(tracking_error)?;
        self.raw.remove_weak(key)
    }

    #[cfg(test)]
    pub(crate) fn rotate_memtable_and_wait(&self) -> fjall::Result<()> {
        self.raw.rotate_memtable_and_wait()
    }

    #[cfg(test)]
    fn major_compact(&self) -> fjall::Result<()> {
        self.raw.major_compact()
    }
}

impl Snapshot {
    #[cfg(test)]
    pub(crate) fn iter(&self, keyspace: &Keyspace) -> fjall::Iter {
        self.raw.iter(&keyspace.raw)
    }

    pub(crate) fn prefix<K: AsRef<[u8]>>(&self, keyspace: &Keyspace, prefix: K) -> fjall::Iter {
        self.raw.prefix(&keyspace.raw, prefix)
    }

    pub(crate) fn get<K: AsRef<[u8]>>(
        &self,
        keyspace: &Keyspace,
        key: K,
    ) -> fjall::Result<Option<UserValue>> {
        self.raw.get(&keyspace.raw, key)
    }

    pub(crate) fn size_of<K: AsRef<[u8]>>(
        &self,
        keyspace: &Keyspace,
        key: K,
    ) -> fjall::Result<Option<u32>> {
        self.raw.size_of(&keyspace.raw, key)
    }
}

impl OwnedWriteBatch {
    pub(crate) fn insert<K: Into<UserKey>, V: Into<UserValue>>(
        &mut self,
        keyspace: &Keyspace,
        key: K,
        value: V,
    ) {
        if !Arc::ptr_eq(&self.shared, &keyspace.shared) {
            self.foreign = true;
            return;
        }
        self.raw.insert(&keyspace.raw, key, value);
    }

    pub(crate) fn remove<K: Into<UserKey>>(&mut self, keyspace: &Keyspace, key: K) {
        if !Arc::ptr_eq(&self.shared, &keyspace.shared) {
            self.foreign = true;
            return;
        }
        self.raw.remove(&keyspace.raw, key);
    }

    pub(crate) fn durability(mut self, maybe_mode: Option<PersistMode>) -> Self {
        self.raw = self.raw.durability(maybe_mode);
        self
    }

    pub(crate) fn commit(self) -> fjall::Result<()> {
        if self.foreign {
            return Err(tracking_error(MutationError));
        }
        if self.raw.is_empty() {
            return self.raw.commit();
        }
        let _mutation = self.shared.begin().map_err(tracking_error)?;
        self.raw.commit()
    }
}

pub(super) mod sealed {
    pub trait Sealed {}
}

/// A private source yielding only detached records through the bounded reader.
pub(in crate::storage) trait RecordSource: sealed::Sealed {
    fn read_existing_records(&self) -> Result<InspectedRecords, StorageError>;
}

impl sealed::Sealed for Database {}

impl RecordSource for Database {
    fn read_existing_records(&self) -> Result<InspectedRecords, StorageError> {
        let _catalog = self.shared.catalog.lock().map_err(|_| {
            super::corruption(super::StorageNamespace::Runtime, "catalog lock poisoned")
        })?;
        self.raw.read_existing_records()
    }
}

#[cfg(test)]
mod tests;
