// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

use super::*;
use std::os::unix::fs::{MetadataExt, PermissionsExt};

#[test]
fn phase166_inspection_inventory_accepts_regular_tree() {
    // Arrange
    let source = fixture();
    fs::write(source.join("lock"), b"").expect("lock");
    fs::write(source.join("payload"), b"private bytes").expect("payload");
    // Act
    let result = inventory::Inventory::capture(&source);
    fs::remove_dir_all(&source).expect("fixture cleanup");
    // Assert
    assert!(result.is_ok());
}

#[test]
fn phase166_inspection_inventory_rejects_symlinks() {
    // Arrange
    let source = fixture();
    std::os::unix::fs::symlink("missing", source.join("link")).expect("fixture link");
    // Act
    let result = inventory::Inventory::capture(&source);
    fs::remove_dir_all(&source).expect("fixture cleanup");
    // Assert
    assert!(result.is_err());
}

#[test]
fn phase166_inspection_exact_budget_boundaries_and_overflow() {
    // Arrange / Act / Assert
    let mut budget = inventory::Budget::default();
    budget
        .add(inventory::MAX_BYTES, inventory::MAX_DEPTH)
        .expect("exact byte/depth limits");
    assert!(budget.add(1, 0).is_err());
    assert!(
        inventory::Budget::default()
            .add(0, inventory::MAX_DEPTH + 1)
            .is_err()
    );
    let mut entries = inventory::Budget::default();
    for _ in 0..inventory::MAX_ENTRIES {
        entries.add(0, 0).expect("at entry bound");
    }
    assert!(entries.add(0, 0).is_err());
    assert!(budget.add(u64::MAX, 0).is_err());
}

#[test]
fn phase166_inspection_rejects_oversized_sparse_source_before_copy() {
    // Arrange
    let source = fixture();
    fs::File::create(source.join("huge"))
        .expect("sparse file")
        .set_len(inventory::MAX_BYTES + 1)
        .expect("sparse length");
    // Act
    let result = inventory::Inventory::capture(&source);
    fs::remove_dir_all(&source).expect("fixture cleanup");
    // Assert
    assert!(result.is_err());
}

#[test]
fn phase166_inspection_rejects_special_and_hardlinked_source_entries() {
    // Arrange
    let source = fixture();
    let socket = std::os::unix::net::UnixListener::bind(source.join("socket")).expect("socket");
    // Act / Assert
    assert!(inventory::Inventory::capture(&source).is_err());
    drop(socket);
    fs::remove_file(source.join("socket")).expect("socket cleanup");
    fs::write(source.join("original"), b"bytes").expect("file");
    fs::hard_link(source.join("original"), source.join("linked")).expect("link");
    assert!(inventory::Inventory::capture(&source).is_err());
    fs::remove_dir_all(&source).expect("fixture cleanup");
}

#[test]
fn phase166_inspection_private_independent_copy_preserves_source() {
    // Arrange
    let source = fixture();
    fs::write(source.join("lock"), b"").expect("lock");
    fs::create_dir(source.join("nested")).expect("nested");
    fs::write(source.join("nested/payload"), b"private bytes").expect("payload");
    let before = inventory::Inventory::capture(&source).expect("before");
    let lease = OperationLease::acquire(&source).expect("lease");
    let mut maybe_scratch = None;
    // Act
    let result = inspect_path(
        &lease,
        |scratch| {
            assert_eq!(fs::metadata(scratch).expect("dir").mode() & 0o777, 0o700);
            let copied = scratch.join("nested/payload");
            let metadata = fs::metadata(&copied).expect("copied file");
            assert_eq!(metadata.mode() & 0o777, 0o600);
            assert_eq!(metadata.nlink(), 1);
            assert_ne!(
                metadata.ino(),
                fs::metadata(source.join("nested/payload"))
                    .expect("source")
                    .ino()
            );
            fs::write(copied, b"independent").expect("scratch change");
            Ok(())
        },
        |stage, _, scratch| {
            if stage == Stage::Copied {
                maybe_scratch = Some(scratch.to_path_buf());
            }
            Ok(())
        },
    );
    // Assert
    result.expect("private copy");
    assert_eq!(
        before,
        inventory::Inventory::capture(&source).expect("after")
    );
    assert!(!maybe_scratch.expect("scratch recorded").exists());
    drop(lease);
    fs::remove_dir_all(&source).expect("fixture cleanup");
}

#[test]
fn phase166_inspection_existing_lock_required_and_contention_refuses() {
    // Arrange
    let source = fixture();
    let lease = OperationLease::acquire(&source).expect("lease");
    // Act / Assert
    assert!(inspect_path(&lease, |_| Ok(()), |_, _, _| Ok(())).is_err());
    assert!(!source.join("lock").exists());
    fs::write(source.join("lock"), b"").expect("existing lock");
    let lock = File::open(source.join("lock")).expect("lock");
    lock.try_lock().expect("hold engine lock");
    assert!(inspect_path(&lease, |_| Ok(()), |_, _, _| Ok(())).is_err());
    drop(lock);
    inspect_path(&lease, |_| Ok(()), |_, _, _| Ok(())).expect("available lock");
    drop(lease);
    fs::remove_dir_all(&source).expect("fixture cleanup");
}

#[test]
fn phase166_inspection_detects_source_content_identity_and_mode_changes() {
    for change in ["content", "inode", "mode", "mtime"] {
        // Arrange
        let source = fixture();
        fs::write(source.join("lock"), b"").expect("lock");
        fs::write(source.join("payload"), b"initial").expect("payload");
        let lease = OperationLease::acquire(&source).expect("lease");
        let mut maybe_scratch = None;
        // Act
        let result = inspect_path(
            &lease,
            |_| Ok(()),
            |stage, source, scratch| {
                if stage != Stage::Copied {
                    return Ok(());
                }
                maybe_scratch = Some(scratch.to_path_buf());
                let path = source.join("payload");
                let times = fs::FileTimes::new().set_modified(
                    fs::metadata(&path)
                        .expect("metadata")
                        .modified()
                        .expect("mtime"),
                );
                match change {
                    "content" => {
                        fs::write(&path, b"changed").expect("change");
                        File::open(&path)
                            .expect("file")
                            .set_times(times)
                            .expect("restore mtime");
                    }
                    "inode" => {
                        fs::rename(&path, source.join("saved")).expect("move");
                        fs::write(&path, b"initial").expect("replacement");
                    }
                    "mode" => {
                        fs::set_permissions(&path, fs::Permissions::from_mode(0o400)).expect("mode")
                    }
                    "mtime" => File::open(&path)
                        .expect("file")
                        .set_times(fs::FileTimes::new().set_modified(std::time::UNIX_EPOCH))
                        .expect("mtime"),
                    _ => unreachable!("fixed change list"),
                }
                Ok(())
            },
        );
        // Assert
        assert!(result.is_err(), "{change}");
        assert!(!maybe_scratch.expect("scratch").exists());
        drop(lease);
        fs::remove_dir_all(&source).expect("fixture cleanup");
    }
}

#[test]
fn phase166_inspection_path_depth_and_entry_limits_are_enforced() {
    // Arrange
    assert!(inventory::validate_path(std::path::Path::new("safe/../escape")).is_err());
    assert!(
        inventory::validate_path(std::path::Path::new(&"x".repeat(inventory::MAX_PATH_BYTES)))
            .is_ok()
    );
    assert!(
        inventory::validate_path(std::path::Path::new(
            &"x".repeat(inventory::MAX_PATH_BYTES + 1)
        ))
        .is_err()
    );
    let source = fixture();
    let mut nested = source.clone();
    for _ in 0..inventory::MAX_DEPTH {
        nested.push("d");
        fs::create_dir(&nested).expect("depth");
    }
    // Act / Assert
    inventory::Inventory::capture(&source).expect("exact depth");
    fs::create_dir(nested.join("d")).expect("one over depth");
    assert!(inventory::Inventory::capture(&source).is_err());
    fs::remove_dir_all(&source).expect("cleanup depth");
    let source = fixture();
    for index in 1..inventory::MAX_ENTRIES {
        fs::write(source.join(index.to_string()), b"").expect("entry");
    }
    inventory::Inventory::capture(&source).expect("exact entry limit including root");
    fs::write(source.join("one-over"), b"").expect("excess entry");
    assert!(inventory::Inventory::capture(&source).is_err());
    fs::remove_dir_all(&source).expect("fixture cleanup");
}
