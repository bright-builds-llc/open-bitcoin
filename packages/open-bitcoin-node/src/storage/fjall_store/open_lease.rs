// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

//! Cooperative application lease, independent of Fjall's engine lock.

use super::{StorageError, StorageNamespace, StorageRecoveryAction};
use std::{
    fs::File,
    path::{Path, PathBuf},
};
#[cfg(unix)]
use std::{
    fs::{self, OpenOptions, TryLockError},
    path::Component,
};

pub(super) struct OperationLease {
    source: PathBuf,
    _file: File,
}

#[cfg(all(test, unix))]
impl Drop for OperationLease {
    fn drop(&mut self) {
        tests::assert_backend_closed_when_armed(&self.source);
    }
}

impl OperationLease {
    #[cfg(not(unix))]
    pub(super) fn acquire(_source: &Path) -> Result<Self, StorageError> {
        Err(lease_error(
            "application store lease unsupported on this platform",
        ))
    }

    #[cfg(unix)]
    pub(super) fn acquire(source: &Path) -> Result<Self, StorageError> {
        let source = normalize_source(source)?;
        let lock_path = external_lock_path(&source)?;
        if lock_path.starts_with(&source) {
            return Err(lease_error("application lease directory overlaps source"));
        }
        match fs::symlink_metadata(&lock_path) {
            Ok(metadata) => validate_lock_metadata(&metadata)?,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(_) => return Err(lease_error("application lease metadata unavailable")),
        }
        let mut options = OpenOptions::new();
        options.read(true).write(true).create(true).truncate(false);
        #[cfg(unix)]
        {
            use std::os::unix::fs::OpenOptionsExt;
            options.mode(0o600);
        }
        let file = options
            .open(&lock_path)
            .map_err(|_| lease_error("application lease file unavailable"))?;
        let opened = file
            .metadata()
            .map_err(|_| lease_error("application lease metadata unavailable"))?;
        let selected = fs::symlink_metadata(&lock_path)
            .map_err(|_| lease_error("application lease metadata unavailable"))?;
        validate_lock_metadata(&opened)?;
        validate_lock_metadata(&selected)?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::MetadataExt;
            if opened.dev() != selected.dev() || opened.ino() != selected.ino() {
                return Err(lease_error("application lease file changed"));
            }
        }
        file.try_lock().map_err(|error| match error {
            TryLockError::WouldBlock => lease_error("database locked by another process"),
            TryLockError::Error(_) => lease_error("application lease locking unavailable"),
        })?;
        // Recheck path shape after locking; cooperating openers keep this inode forever.
        if normalize_source(&source)? != source {
            return Err(lease_error("application lease source changed"));
        }
        Ok(Self {
            source,
            _file: file,
        })
    }

    pub(super) fn source(&self) -> &Path {
        &self.source
    }
}

#[cfg(unix)]
fn normalize_source(source: &Path) -> Result<PathBuf, StorageError> {
    if source.as_os_str().is_empty() {
        return Err(lease_error("unsafe application lease source"));
    }
    let absolute = if source.is_absolute() {
        source.to_path_buf()
    } else {
        std::env::current_dir()
            .map_err(|_| lease_error("application lease current directory unavailable"))?
            .join(source)
    };
    let mut normalized = PathBuf::new();
    let mut missing = false;
    for component in absolute.components() {
        match component {
            Component::ParentDir | Component::Prefix(_) => {
                return Err(lease_error("unsafe application lease source"));
            }
            Component::CurDir => continue,
            Component::RootDir | Component::Normal(_) => normalized.push(component.as_os_str()),
        }
        if missing {
            continue;
        }
        match fs::symlink_metadata(&normalized) {
            Ok(metadata) if metadata.is_dir() && !metadata.file_type().is_symlink() => {
                let canonical = fs::canonicalize(&normalized)
                    .map_err(|_| lease_error("application lease source unavailable"))?;
                if canonical != normalized {
                    return Err(lease_error("unsafe application lease source"));
                }
            }
            Ok(_) => return Err(lease_error("unsafe application lease source")),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => missing = true,
            Err(_) => return Err(lease_error("application lease source unavailable")),
        }
    }
    if normalized.parent().is_none() {
        return Err(lease_error("unsafe application lease source"));
    }
    Ok(normalized)
}

#[cfg(unix)]
fn external_lock_path(source: &Path) -> Result<PathBuf, StorageError> {
    // Fixed platform scratch root keeps identity independent of caller TMPDIR/HOME.
    let root = fs::canonicalize("/tmp")
        .map_err(|_| lease_error("application lease directory unavailable"))?;
    let hash =
        open_bitcoin_core::consensus::crypto::double_sha256(source.as_os_str().as_encoded_bytes());
    let digest: String = hash.iter().map(|byte| format!("{byte:02x}")).collect();
    Ok(root.join(format!("open-bitcoin-store-lease-v1-{digest}.lock")))
}

#[cfg(unix)]
fn validate_lock_metadata(metadata: &fs::Metadata) -> Result<(), StorageError> {
    if !metadata.is_file() || metadata.file_type().is_symlink() || metadata.len() != 0 {
        return Err(lease_error("unsafe application lease file"));
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::MetadataExt;
        if metadata.nlink() != 1 || metadata.mode() & 0o077 != 0 {
            return Err(lease_error("unsafe application lease file"));
        }
    }
    Ok(())
}

fn lease_error(message: &'static str) -> StorageError {
    StorageError::BackendFailure {
        namespace: StorageNamespace::Runtime,
        message: message.into(),
        action: StorageRecoveryAction::Restart,
    }
}

#[cfg(test)]
impl super::FjallNodeStore {
    pub(crate) fn open_without_ensure_schema_for_test(
        path: impl AsRef<Path>,
    ) -> Result<Self, StorageError> {
        Self::open_unrecovered(path)
    }
}

#[cfg(all(test, unix))]
mod tests;
