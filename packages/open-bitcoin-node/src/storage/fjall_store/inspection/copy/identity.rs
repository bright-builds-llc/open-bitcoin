// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

use super::{CopyFailure, StorageError, inspection_error, source_io};
#[cfg(unix)]
use std::os::unix::fs::MetadataExt;
use std::{
    fs::{self, File, OpenOptions},
    path::Path,
    time::SystemTime,
};

#[derive(Debug, PartialEq, Eq)]
pub(in super::super) struct Identity {
    pub(super) directory: bool,
    pub(super) length: u64,
    pub(super) modified: SystemTime,
    #[cfg(unix)]
    pub(super) device: u64,
    #[cfg(unix)]
    pub(super) inode: u64,
    #[cfg(unix)]
    pub(super) mode: u32,
    #[cfg(unix)]
    pub(super) links: u64,
}

impl Identity {
    pub(in super::super) fn read(path: &Path) -> Result<Self, StorageError> {
        Self::read_typed(path, false).map_err(CopyFailure::original)
    }

    pub(super) fn read_typed(path: &Path, known: bool) -> Result<Self, CopyFailure> {
        let metadata = fs::symlink_metadata(path)
            .map_err(|error| source_io(error, "inspection source metadata unavailable", known))?;
        let result = Self::from_metadata_typed(&metadata, known);
        #[cfg(test)]
        if result.is_err() {
            Self::observe_refusal("path", path, known, &metadata);
        }
        result
    }

    fn from_metadata_typed(metadata: &fs::Metadata, known: bool) -> Result<Self, CopyFailure> {
        #[cfg(unix)]
        if known && metadata.is_file() && metadata.nlink() == 0 {
            return Err(CopyFailure::Changed(inspection_error(
                "unsafe inspection source hardlink",
            )));
        }
        #[cfg(not(unix))]
        let _ = known;
        Self::from_metadata(metadata).map_err(CopyFailure::Refused)
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

    pub(in super::super) fn check_path(&self, path: &Path) -> Result<(), StorageError> {
        self.check_path_typed(path, false)
            .map_err(CopyFailure::original)
    }

    pub(super) fn safety_differs(&self, other: &Self) -> bool {
        if self.directory != other.directory {
            return true;
        }
        #[cfg(unix)]
        if self.mode != other.mode || (!self.directory && self.links != other.links) {
            return true;
        }
        false
    }

    fn compare(&self, other: &Self) -> Result<(), CopyFailure> {
        if self == other {
            return Ok(());
        }
        let original = inspection_error("inspection source changed");
        #[cfg(test)]
        eprintln!(
            "LIVE_IDENTITY_DIFFERENCE safety={} before={self:?} after={other:?}",
            self.safety_differs(other)
        );
        Err(if self.safety_differs(other) {
            CopyFailure::Refused(original)
        } else {
            CopyFailure::Changed(original)
        })
    }

    pub(super) fn check_path_typed(&self, path: &Path, known: bool) -> Result<(), CopyFailure> {
        let result = self.compare(&Self::read_typed(path, known)?);
        #[cfg(test)]
        if let Err(error) = &result {
            eprintln!(
                "LIVE_CLASSIFICATION stage=check_path path={path:?} known={known} typed={error:?}"
            );
        }
        result
    }

    pub(in super::super) fn open_file(&self, path: &Path) -> Result<File, StorageError> {
        self.open_file_typed(path).map_err(CopyFailure::original)
    }

    pub(super) fn open_file_typed(&self, path: &Path) -> Result<File, CopyFailure> {
        if self.directory {
            return Err(inspection_error("unsafe inspection source file").into());
        }
        self.check_path_typed(path, true)?;
        let file = OpenOptions::new()
            .read(true)
            .write(false)
            .open(path)
            .map_err(|error| source_io(error, "inspection source file unavailable", true))?;
        self.check_opened_typed(&file, path)?;
        Ok(file)
    }

    pub(in super::super) fn check_opened(
        &self,
        file: &File,
        path: &Path,
    ) -> Result<(), StorageError> {
        self.check_opened_typed(file, path)
            .map_err(CopyFailure::original)
    }

    pub(super) fn check_opened_typed(&self, file: &File, path: &Path) -> Result<(), CopyFailure> {
        let metadata = file
            .metadata()
            .map_err(|_| inspection_error("inspection source file metadata unavailable"))?;
        let result = Self::from_metadata(&metadata);
        #[cfg(test)]
        if result.is_err() {
            Self::observe_refusal("opened", path, true, &metadata);
        }
        #[cfg(unix)]
        let result = match result {
            Err(original) if metadata.is_file() && metadata.nlink() == 0 => {
                return self.check_unlinked(path, &metadata, original);
            }
            result => result,
        };
        let opened = result?;
        let compared = self.compare(&opened);
        #[cfg(test)]
        if let Err(error) = &compared {
            eprintln!("LIVE_CLASSIFICATION stage=check_opened path={path:?} typed={error:?}");
        }
        compared?;
        self.check_path_typed(path, true)
    }

    #[cfg(unix)]
    fn check_unlinked(
        &self,
        path: &Path,
        metadata: &fs::Metadata,
        original: StorageError,
    ) -> Result<(), CopyFailure> {
        if self.directory
            || self.links != 1
            || self.device != metadata.dev()
            || self.inode != metadata.ino()
            || self.mode != metadata.mode()
        {
            return Err(CopyFailure::Refused(original));
        }
        // The first strict constructor error remains the offline result. Guards
        // determine only whether this whole live attempt may be discarded.
        let path_safety = super::validate_path(path)
            .map_err(CopyFailure::Refused)
            .and_then(|()| self.check_path_typed(path, true));
        match path_safety {
            Err(CopyFailure::Refused(_)) => Err(CopyFailure::Refused(original)),
            Ok(()) | Err(CopyFailure::Changed(_)) => Err(CopyFailure::Changed(original)),
        }
    }

    #[cfg(test)]
    fn observe_refusal(stage: &str, path: &Path, known: bool, metadata: &fs::Metadata) {
        let bytes = path.as_os_str().as_encoded_bytes();
        let prefix = &bytes[..bytes.len().min(256)];
        let kind = if metadata.is_file() {
            "regular"
        } else if metadata.is_dir() {
            "directory"
        } else {
            "other"
        };
        #[cfg(unix)]
        let links = metadata.nlink();
        #[cfg(not(unix))]
        let links = "unavailable";
        eprintln!(
            "LIVE_CONSTRUCTOR_REFUSAL stage={stage} known={known} kind={kind} links={links} path_encoded_prefix={prefix:?}"
        );
    }
}

#[cfg(all(test, unix))]
mod tests {
    use super::*;
    use std::{
        path::PathBuf,
        sync::atomic::{AtomicU64, Ordering},
    };

    static NEXT_FIXTURE: AtomicU64 = AtomicU64::new(0);

    fn fixture() -> (PathBuf, PathBuf, File) {
        let root = fs::canonicalize(std::env::temp_dir())
            .expect("canonical temporary parent")
            .join(format!(
                "phase167-known-unlink-{}-{}",
                std::process::id(),
                NEXT_FIXTURE.fetch_add(1, Ordering::Relaxed)
            ));
        fs::create_dir(&root).expect("new private fixture root");
        let source = root.join("source");
        let held = OpenOptions::new()
            .read(true)
            .write(true)
            .create_new(true)
            .open(&source)
            .expect("new regular source");
        (root, source, held)
    }

    #[test]
    fn known_unlinked_regular_metadata_is_changed_but_unknown_is_refused() {
        // Arrange: retain the real file handle after unlink so its nlink is zero.
        let (root, source, held) = fixture();
        fs::remove_file(&source).expect("unlink held regular source");
        let metadata = held.metadata().expect("held file metadata");
        assert!(metadata.is_file());
        assert_eq!(metadata.nlink(), 0);

        // Act: use the same metadata classifier as the live path reader.
        let known = Identity::from_metadata_typed(&metadata, true);
        let unknown = Identity::from_metadata_typed(&metadata, false);

        // Assert: the strict constructor and unknown path still refuse.
        assert!(matches!(known, Err(CopyFailure::Changed(_))));
        assert!(matches!(unknown, Err(CopyFailure::Refused(_))));
        assert!(Identity::from_metadata(&metadata).is_err());
        drop(held);
        fs::remove_dir(&root).expect("remove empty fixture root");
    }

    #[test]
    fn two_link_regular_metadata_is_refused_even_when_known() {
        // Arrange: create a real second link to the held regular file.
        let (root, source, held) = fixture();
        fs::hard_link(&source, root.join("second")).expect("second link");
        let metadata = held.metadata().expect("held file metadata");
        assert!(metadata.is_file());
        assert_eq!(metadata.nlink(), 2);

        // Act: classify the same metadata as known and unknown.
        let known = Identity::from_metadata_typed(&metadata, true);
        let unknown = Identity::from_metadata_typed(&metadata, false);

        // Assert: neither path treats an actual hardlink as transient unlink.
        assert!(matches!(known, Err(CopyFailure::Refused(_))));
        assert!(matches!(unknown, Err(CopyFailure::Refused(_))));
        assert!(Identity::from_metadata(&metadata).is_err());
        drop(held);
        fs::remove_dir_all(&root).expect("remove hardlink fixture root");
    }
}
