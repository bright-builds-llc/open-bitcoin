// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

use super::{StorageError, inspection_error};
use open_bitcoin_core::consensus::crypto::Sha256;
#[cfg(unix)]
use std::os::unix::fs::{DirBuilderExt, OpenOptionsExt, PermissionsExt};
use std::{
    collections::BTreeMap,
    fs::{self, File, OpenOptions},
    io::{Read, Write},
    path::{Component, Path, PathBuf},
};

// Root counts as one entry at depth zero; only regular-file contents count as bytes.
pub(super) const MAX_ENTRIES: usize = 4096;
pub(super) const MAX_BYTES: u64 = 512 * 1024 * 1024;
pub(super) const MAX_DEPTH: usize = 16;
pub(super) const MAX_PATH_BYTES: usize = 4096;
const CHUNK_BYTES: usize = 64 * 1024;

#[derive(Debug)]
pub(super) enum CopyFailure {
    Changed(StorageError),
    Refused(StorageError),
}

impl CopyFailure {
    pub(super) fn original(self) -> StorageError {
        match self {
            Self::Changed(error) | Self::Refused(error) => error,
        }
    }
}

impl From<StorageError> for CopyFailure {
    fn from(error: StorageError) -> Self {
        Self::Refused(error)
    }
}

/// Cumulative entry visits across every attempt of one live observation.
#[derive(Debug, Default)]
pub(super) struct WorkLedger {
    pub(super) visits: u64,
}

impl WorkLedger {
    pub(super) fn reserve(counter: &mut u64, amount: u64, cap: u64) -> Result<(), CopyFailure> {
        let next = counter
            .checked_add(amount)
            .ok_or_else(|| inspection_error("inspection work arithmetic overflow"))?;
        if next > cap {
            return Err(inspection_error("inspection work limit exceeded").into());
        }
        *counter = next;
        Ok(())
    }

    pub(super) fn visit(&mut self) -> Result<(), CopyFailure> {
        Self::reserve(&mut self.visits, 1, 2 * MAX_ENTRIES as u64)
    }
}

fn source_io(error: std::io::Error, message: &'static str, known: bool) -> CopyFailure {
    let original = inspection_error(message);
    if known && error.kind() == std::io::ErrorKind::NotFound {
        CopyFailure::Changed(original)
    } else {
        CopyFailure::Refused(original)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct Inventory {
    entries: BTreeMap<PathBuf, Entry>,
    budget: Budget,
}

#[derive(Debug, Default, PartialEq, Eq)]
pub(super) struct Budget {
    entries: usize,
    bytes: u64,
}

impl Budget {
    pub(super) fn add(&mut self, length: u64, depth: usize) -> Result<(), StorageError> {
        let maybe_entries = self.entries.checked_add(1);
        let maybe_bytes = self.bytes.checked_add(length);
        let (Some(entries), Some(bytes)) = (maybe_entries, maybe_bytes) else {
            return Err(inspection_error("inspection inventory arithmetic overflow"));
        };
        if entries > MAX_ENTRIES || bytes > MAX_BYTES || depth > MAX_DEPTH {
            return Err(inspection_error("inspection inventory limit exceeded"));
        }
        self.entries = entries;
        self.bytes = bytes;
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Entry {
    identity: Identity,
    maybe_digest: Option<[u8; 32]>,
}

mod identity;
pub(super) use identity::Identity;

impl Inventory {
    pub(super) fn capture(source: &Path) -> Result<Self, StorageError> {
        Self::capture_typed(source, &mut None).map_err(CopyFailure::original)
    }

    /// With a ledger this is the live walk: it opens each file to prove it is
    /// readable and unchanged but never reads it, because every envelope limit
    /// is decided by metadata. Without one it also digests file content.
    pub(super) fn capture_typed(
        source: &Path,
        maybe_work: &mut Option<&mut WorkLedger>,
    ) -> Result<Self, CopyFailure> {
        validate_path(source)?;
        if let Some(work) = maybe_work.as_mut() {
            work.visit()?;
        }
        if !Identity::read(source)?.directory {
            return Err(inspection_error("unsafe inspection source root").into());
        }
        let mut inventory = Self {
            entries: BTreeMap::new(),
            budget: Budget::default(),
        };
        inventory.visit(source, Path::new(""), 0, maybe_work)?;
        #[cfg(test)]
        inventory.observe_walk(maybe_work, false);
        Ok(inventory)
    }

    fn visit(
        &mut self,
        root: &Path,
        relative: &Path,
        depth: usize,
        maybe_work: &mut Option<&mut WorkLedger>,
    ) -> Result<(), CopyFailure> {
        validate_path(relative)?;
        let path = root.join(relative);
        validate_path(&path)?;
        let descendant = !relative.as_os_str().is_empty();
        if descendant && let Some(work) = maybe_work.as_mut() {
            work.visit()?;
        }
        #[cfg(test)]
        super::live::observe(super::live::Point::BeforeMetadata, &path);
        let identity = Identity::read_typed(&path, descendant)?;
        self.budget.add(
            if identity.directory {
                0
            } else {
                identity.length
            },
            depth,
        )?;
        let maybe_digest = if identity.directory {
            #[cfg(test)]
            super::live::observe(super::live::Point::DirectoryRead, &path);
            for child in fs::read_dir(&path)
                .map_err(|error| source_io(error, "inspection directory unavailable", descendant))?
            {
                let child = child
                    .map_err(|_| inspection_error("inspection directory entry unavailable"))?;
                let child_relative = relative.join(child.file_name());
                self.visit(
                    root,
                    &child_relative,
                    depth
                        .checked_add(1)
                        .ok_or_else(|| inspection_error("inspection depth overflow"))?,
                    maybe_work,
                )?;
            }
            identity.check_path_typed(&path, descendant)?;
            None
        } else if maybe_work.is_some() {
            check_readable(&identity, &path)?;
            None
        } else {
            Some(file_digest(&identity, &path, None)?)
        };
        self.entries.insert(
            relative.to_path_buf(),
            Entry {
                identity,
                maybe_digest,
            },
        );
        Ok(())
    }

    pub(super) fn copy_to(&self, source: &Path, destination: &Path) -> Result<(), StorageError> {
        for (relative, entry) in &self.entries {
            let original = source.join(relative);
            let target = destination.join(relative);
            validate_path(&target)?;
            entry.identity.check_path(&original)?;
            if entry.identity.directory {
                create_private_directory(&target)?;
                continue;
            }
            let mut options = OpenOptions::new();
            options.write(true).create_new(true);
            #[cfg(unix)]
            options.mode(0o600);
            let mut file = options
                .open(&target)
                .map_err(|_| inspection_error("inspection private copy unavailable"))?;
            if Some(file_digest(&entry.identity, &original, Some(&mut file))?) != entry.maybe_digest
            {
                return Err(inspection_error("inspection source content changed"));
            }
        }
        Ok(())
    }

    pub(super) fn recheck(&self, source: &Path) -> Result<(), StorageError> {
        self.recheck_typed(source, &mut None)
            .map_err(CopyFailure::original)
    }

    pub(super) fn recheck_typed(
        &self,
        source: &Path,
        maybe_work: &mut Option<&mut WorkLedger>,
    ) -> Result<(), CopyFailure> {
        let actual = Self::capture_typed(source, maybe_work)?;
        if *self == actual {
            #[cfg(test)]
            actual.observe_walk(maybe_work, true);
            return Ok(());
        }
        let original = inspection_error("inspection source changed");
        if self.entries.iter().any(|(name, before)| {
            actual.entries.get(name).is_some_and(|after| {
                #[cfg(all(test, unix))]
                if name.as_os_str().is_empty() && before.identity.directory && after.identity.directory
                    && before.identity.links != after.identity.links {
                    super::live::record_root_links(before.identity.links, after.identity.links);
                }
                let refused = before.identity.safety_differs(&after.identity);
                #[cfg(test)]
                if refused {
                    eprintln!(
                        "LIVE_COMMON_ENTRY_REFUSED stage=whole_inventory path={:?} relative={name:?} typed=Refused before={:?} after={:?}",
                        source.join(name), before.identity, after.identity
                    );
                }
                refused
            })
        }) {
            return Err(CopyFailure::Refused(original));
        }
        Err(CopyFailure::Changed(original))
    }

    #[cfg(test)]
    fn observe_walk(&self, maybe_work: &Option<&mut WorkLedger>, equal: bool) {
        if let Some(work) = maybe_work {
            super::live::record_walk(
                (
                    self.budget.entries,
                    self.budget.bytes,
                    self.entries
                        .values()
                        .filter(|entry| !entry.identity.directory)
                        .count(),
                ),
                work.visits,
                equal,
            );
        }
    }

    #[cfg(test)]
    pub(super) fn size(&self) -> (usize, u64) {
        (self.budget.entries, self.budget.bytes)
    }

    #[cfg(all(test, unix))]
    pub(super) fn private_modes(&self) -> bool {
        self.entries.values().all(|entry| {
            entry.identity.mode & 0o777
                == if entry.identity.directory {
                    0o700
                } else {
                    0o600
                }
        })
    }
}

pub(super) fn secure_recovered_tree(root: &Path) -> Result<(), StorageError> {
    fn visit(path: &Path, depth: usize, budget: &mut Budget) -> Result<(), StorageError> {
        validate_path(path)?;
        let identity = Identity::read(path)?;
        budget.add(
            if identity.directory {
                0
            } else {
                identity.length
            },
            depth,
        )?;
        #[cfg(unix)]
        fs::set_permissions(
            path,
            fs::Permissions::from_mode(if identity.directory { 0o700 } else { 0o600 }),
        )
        .map_err(|_| inspection_error("inspection recovered permissions unavailable"))?;
        if !identity.directory {
            return Ok(());
        }
        for child in fs::read_dir(path)
            .map_err(|_| inspection_error("inspection recovered directory unavailable"))?
        {
            let child =
                child.map_err(|_| inspection_error("inspection recovered entry unavailable"))?;
            let next = depth
                .checked_add(1)
                .ok_or_else(|| inspection_error("inspection depth overflow"))?;
            visit(&child.path(), next, budget)?;
        }
        Ok(())
    }
    visit(root, 0, &mut Budget::default())
}

pub(super) fn validate_path(path: &Path) -> Result<(), StorageError> {
    if path.as_os_str().as_encoded_bytes().len() > MAX_PATH_BYTES
        || path
            .components()
            .any(|part| matches!(part, Component::ParentDir | Component::Prefix(_)))
    {
        return Err(inspection_error("unsafe inspection path"));
    }
    Ok(())
}

pub(super) fn create_private_directory(path: &Path) -> Result<(), StorageError> {
    let mut builder = fs::DirBuilder::new();
    #[cfg(unix)]
    builder.mode(0o700);
    builder
        .create(path)
        .map_err(|_| inspection_error("inspection private directory unavailable"))
}

fn check_readable(identity: &Identity, path: &Path) -> Result<(), CopyFailure> {
    let file = identity.open_file_typed(path)?;
    #[cfg(test)]
    super::live::observe(super::live::Point::Opened, path);
    identity.check_opened_typed(&file, path)
}

fn file_digest(
    identity: &Identity,
    path: &Path,
    mut maybe_copy: Option<&mut File>,
) -> Result<[u8; 32], StorageError> {
    let mut file = identity.open_file(path)?;
    #[cfg(test)]
    super::live::observe(super::live::Point::Opened, path);
    let mut remaining = identity.length;
    let mut digest = [0_u8; 32];
    let mut buffer = [0_u8; CHUNK_BYTES];
    while remaining != 0 {
        let count = usize::try_from(remaining.min(CHUNK_BYTES as u64))
            .map_err(|_| inspection_error("inspection copy arithmetic overflow"))?;
        #[cfg(test)]
        let maybe_fault = if maybe_copy.is_some() {
            COPY_IO_FAULT.take()
        } else {
            None
        };
        #[cfg(test)]
        if maybe_fault == Some(CopyIoFault::Read) {
            return Err(inspection_error("inspection source read failed"));
        }
        file.read_exact(&mut buffer[..count])
            .map_err(|_| inspection_error("inspection source read failed"))?;
        if let Some(copy) = maybe_copy.as_mut() {
            #[cfg(test)]
            if maybe_fault == Some(CopyIoFault::Write) {
                return Err(inspection_error("inspection private copy failed"));
            }
            copy.write_all(&buffer[..count])
                .map_err(|_| inspection_error("inspection private copy failed"))?;
        }
        // A fixed-chunk hash chain bounds memory without adding a streaming-hash dependency.
        let mut pair = [0_u8; 64];
        pair[..32].copy_from_slice(&digest);
        pair[32..].copy_from_slice(&Sha256::digest(&buffer[..count]));
        digest = Sha256::digest(&pair);
        remaining = remaining
            .checked_sub(count as u64)
            .ok_or_else(|| inspection_error("inspection copy arithmetic overflow"))?;
    }
    let tail = file
        .read(&mut buffer[..1])
        .map_err(|_| inspection_error("inspection source read failed"))?;
    if tail != 0 {
        return Err(inspection_error("inspection source content changed"));
    }
    identity.check_opened(&file, path)?;
    Ok(digest)
}

#[cfg(test)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum CopyIoFault {
    Read,
    Write,
}

#[cfg(test)]
thread_local! {
    pub(super) static COPY_IO_FAULT: std::cell::Cell<Option<CopyIoFault>> = const { std::cell::Cell::new(None) };
}
