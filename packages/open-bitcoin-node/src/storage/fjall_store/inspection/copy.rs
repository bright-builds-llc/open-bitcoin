// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

use super::{StorageError, inspection_error};
use open_bitcoin_core::consensus::crypto::Sha256;
#[cfg(unix)]
use std::os::unix::fs::{DirBuilderExt, MetadataExt, OpenOptionsExt, PermissionsExt};
use std::{
    collections::BTreeMap,
    fs::{self, File, OpenOptions},
    io::{Read, Write},
    path::{Component, Path, PathBuf},
    time::SystemTime,
};

// Root counts as one entry at depth zero; only regular-file contents count as bytes.
pub(super) const MAX_ENTRIES: usize = 4096;
pub(super) const MAX_BYTES: u64 = 512 * 1024 * 1024;
pub(super) const MAX_DEPTH: usize = 16;
pub(super) const MAX_PATH_BYTES: usize = 4096;
const CHUNK_BYTES: usize = 64 * 1024;

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

#[derive(Debug, PartialEq, Eq)]
pub(super) struct Identity {
    directory: bool,
    length: u64,
    modified: SystemTime,
    #[cfg(unix)]
    device: u64,
    #[cfg(unix)]
    inode: u64,
    #[cfg(unix)]
    mode: u32,
    #[cfg(unix)]
    links: u64,
}

impl Identity {
    pub(super) fn read(path: &Path) -> Result<Self, StorageError> {
        Self::from_metadata(
            &fs::symlink_metadata(path)
                .map_err(|_| inspection_error("inspection source metadata unavailable"))?,
        )
    }

    fn from_metadata(metadata: &fs::Metadata) -> Result<Self, StorageError> {
        if (!metadata.is_dir() && !metadata.is_file()) || metadata.file_type().is_symlink() {
            return Err(inspection_error("unsafe inspection source entry"));
        }
        #[cfg(unix)]
        if metadata.is_file() && metadata.nlink() != 1 {
            return Err(inspection_error("unsafe inspection source hardlink"));
        }
        Ok(Self {
            directory: metadata.is_dir(),
            length: metadata.len(),
            modified: metadata
                .modified()
                .map_err(|_| inspection_error("inspection source timestamp unavailable"))?,
            #[cfg(unix)]
            device: metadata.dev(),
            #[cfg(unix)]
            inode: metadata.ino(),
            #[cfg(unix)]
            mode: metadata.mode(),
            #[cfg(unix)]
            links: metadata.nlink(),
        })
    }

    pub(super) fn check_path(&self, path: &Path) -> Result<(), StorageError> {
        if *self != Self::read(path)? {
            return Err(inspection_error("inspection source changed"));
        }
        Ok(())
    }

    pub(super) fn open_file(&self, path: &Path) -> Result<File, StorageError> {
        if self.directory {
            return Err(inspection_error("unsafe inspection source file"));
        }
        self.check_path(path)?;
        let file = OpenOptions::new()
            .read(true)
            .write(false)
            .open(path)
            .map_err(|_| inspection_error("inspection source file unavailable"))?;
        self.check_opened(&file, path)?;
        Ok(file)
    }

    pub(super) fn check_opened(&self, file: &File, path: &Path) -> Result<(), StorageError> {
        let opened = Self::from_metadata(
            &file
                .metadata()
                .map_err(|_| inspection_error("inspection source file metadata unavailable"))?,
        )?;
        if *self != opened {
            return Err(inspection_error("inspection source changed"));
        }
        self.check_path(path)
    }
}

impl Inventory {
    pub(super) fn capture(source: &Path) -> Result<Self, StorageError> {
        validate_path(source)?;
        if !Identity::read(source)?.directory {
            return Err(inspection_error("unsafe inspection source root"));
        }
        let mut inventory = Self {
            entries: BTreeMap::new(),
            budget: Budget::default(),
        };
        inventory.visit(source, Path::new(""), 0)?;
        Ok(inventory)
    }

    fn visit(&mut self, root: &Path, relative: &Path, depth: usize) -> Result<(), StorageError> {
        validate_path(relative)?;
        let path = root.join(relative);
        validate_path(&path)?;
        let identity = Identity::read(&path)?;
        self.budget.add(
            if identity.directory {
                0
            } else {
                identity.length
            },
            depth,
        )?;
        let maybe_digest = if identity.directory {
            for child in fs::read_dir(&path)
                .map_err(|_| inspection_error("inspection directory unavailable"))?
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
                )?;
            }
            identity.check_path(&path)?;
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
        if *self != Self::capture(source)? {
            return Err(inspection_error("inspection source changed"));
        }
        Ok(())
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

fn file_digest(
    identity: &Identity,
    path: &Path,
    mut maybe_copy: Option<&mut File>,
) -> Result<[u8; 32], StorageError> {
    let mut file = identity.open_file(path)?;
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
    if file
        .read(&mut buffer[..1])
        .map_err(|_| inspection_error("inspection source read failed"))?
        != 0
    {
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
