// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

//! Probe-only lock evidence for Fjall-backed datadirs.

use std::{
    fs::{self, File, TryLockError},
    path::Path,
};

use crate::{
    recovery::{LockEvidence, LockEvidenceKind},
    status::FieldAvailability,
};

/// Fjall database lock artifact file name.
pub const FJALL_LOCK_FILE_NAME: &str = "lock";

const MISSING_DATADIR_REASON: &str = "lock probe unavailable: datadir does not exist";
const NO_LOCK_ARTIFACT_DETAIL: &str = "no Fjall lock artifact found";
const STALE_LOCK_DETAIL: &str = "Fjall lock artifact is present but not currently held";
const ACTIVE_CONTENTION_DETAIL: &str = "Fjall lock is currently held by another opener";
const DETAIL_LIMIT_BYTES: usize = 240;

/// Collect non-mutating Fjall lock evidence for a datadir.
pub fn probe_fjall_lock(datadir: impl AsRef<Path>) -> FieldAvailability<LockEvidence> {
    let datadir = datadir.as_ref();
    let Ok(metadata) = fs::metadata(datadir) else {
        return FieldAvailability::unavailable(MISSING_DATADIR_REASON);
    };
    if !metadata.is_dir() {
        return FieldAvailability::unavailable(MISSING_DATADIR_REASON);
    }

    let lock_path = datadir.join(FJALL_LOCK_FILE_NAME);
    if !lock_path.exists() {
        return FieldAvailability::available(lock_evidence(
            LockEvidenceKind::NoLockArtifact,
            &lock_path,
            NO_LOCK_ARTIFACT_DETAIL.to_string(),
        ));
    }

    let file = match File::open(&lock_path) {
        Ok(file) => file,
        Err(error) => {
            return FieldAvailability::available(lock_evidence(
                LockEvidenceKind::ProbeUnavailable,
                &lock_path,
                lock_probe_unavailable_detail("lock file could not be opened", &error),
            ));
        }
    };

    match file.try_lock() {
        Ok(()) => match file.unlock() {
            Ok(()) => FieldAvailability::available(lock_evidence(
                LockEvidenceKind::StaleLockEvidence,
                &lock_path,
                STALE_LOCK_DETAIL.to_string(),
            )),
            Err(error) => FieldAvailability::available(lock_evidence(
                LockEvidenceKind::ProbeUnavailable,
                &lock_path,
                lock_probe_unavailable_detail("advisory lock could not be released", &error),
            )),
        },
        Err(TryLockError::WouldBlock) => FieldAvailability::available(lock_evidence(
            LockEvidenceKind::ActiveContention,
            &lock_path,
            ACTIVE_CONTENTION_DETAIL.to_string(),
        )),
        Err(TryLockError::Error(error)) => FieldAvailability::available(lock_evidence(
            LockEvidenceKind::ProbeUnavailable,
            &lock_path,
            lock_probe_unavailable_detail("advisory lock failed", &error),
        )),
    }
}

fn lock_evidence(kind: LockEvidenceKind, lock_path: &Path, detail: String) -> LockEvidence {
    LockEvidence {
        kind,
        lock_path: lock_path.display().to_string(),
        detail,
    }
}

fn lock_probe_unavailable_detail(context: &str, error: &std::io::Error) -> String {
    bounded_detail(format!("lock probe unavailable: {context}: {error}"))
}

fn bounded_detail(detail: String) -> String {
    if detail.len() <= DETAIL_LIMIT_BYTES {
        return detail;
    }

    let mut end = DETAIL_LIMIT_BYTES;
    while !detail.is_char_boundary(end) {
        end -= 1;
    }
    format!("{}...", &detail[..end])
}
