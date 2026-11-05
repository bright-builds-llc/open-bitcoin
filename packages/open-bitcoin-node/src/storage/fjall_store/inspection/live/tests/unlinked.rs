// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

use super::*;
use std::{
    fs::File,
    os::unix::{
        fs::{FileTypeExt, MetadataExt, PermissionsExt, symlink},
        net::UnixListener,
    },
    path::Path,
};

fn install_unlink_hook() -> Arc<AtomicUsize> {
    let calls = Arc::new(AtomicUsize::new(0));
    let observed = Arc::clone(&calls);
    MAYBE_HOOK.set(Some(Box::new(move |point, path| {
        if point != Point::Opened {
            return;
        }
        let held = File::open(path).expect("independent held descriptor");
        fs::remove_file(path).expect("real last unlink after initial descriptor validation");
        let metadata = held.metadata().expect("actual unlinked held metadata");
        assert!(metadata.is_file());
        assert_eq!(metadata.nlink(), 0);
        observed.fetch_add(1, Ordering::SeqCst);
        eprintln!(
            "OPENED_UNLINK_REAL_FD regular=true links={} inode={}",
            metadata.nlink(),
            metadata.ino()
        );
    })));
    calls
}

fn isolated_source() -> PathBuf {
    let root = source();
    fs::create_dir(&root).expect("owned root");
    fs::write(root.join("payload"), [1, 2, 3, 4]).expect("four byte payload");
    root
}

#[test]
fn phase167_live_physical_opened_unlink_changed_and_charged() {
    // Arrange
    let root = isolated_source();
    let calls = install_unlink_hook();
    let mut work = copy::WorkLedger::default();
    // Act
    let result = copy::Inventory::capture_typed(&root, &mut Some(&mut work));
    MAYBE_HOOK.set(None);
    fs::remove_dir_all(root).expect("cleanup before intended RED assertion");
    // Assert
    assert_eq!(calls.load(Ordering::SeqCst), 1);
    assert_eq!((work.visits, work.content, work.probes), (2, 4, 1));
    assert!(
        matches!(&result, Err(copy::CopyFailure::Changed(_))),
        "actual opened-unlink category: {result:?}"
    );
    assert_eq!(
        result
            .expect_err("no Inventory for unlinked file")
            .original(),
        inspection_error("unsafe inspection source hardlink")
    );
}

#[test]
fn phase167_live_physical_opened_unlink_strict_original_once() {
    // Arrange
    let root = isolated_source();
    let calls = install_unlink_hook();
    // Act
    let result = copy::Inventory::capture(&root);
    MAYBE_HOOK.set(None);
    fs::remove_dir_all(root).expect("strict cleanup");
    // Assert
    assert_eq!(calls.load(Ordering::SeqCst), 1);
    assert_eq!(
        result.expect_err("strict original refusal"),
        inspection_error("unsafe inspection source hardlink")
    );
}

#[derive(Clone, Copy, Debug)]
enum Replacement {
    Missing,
    HeldMode,
    Symlink,
    Directory,
    Socket,
    Hardlink,
    WrongMode,
    Safe,
    OpenedHardlink,
}

fn socket_replacement(path: &Path) {
    let mut nonce = [0; 16];
    getrandom::fill(&mut nonce).expect("short owned alias nonce");
    let alias = fs::canonicalize("/tmp")
        .expect("physical short parent")
        .join(format!("p167-unlink-{:x}", u128::from_le_bytes(nonce)));
    symlink(path.parent().expect("owned parent"), &alias).expect("unique owned short alias");
    let address = alias.join(path.file_name().expect("owned filename"));
    let result = UnixListener::bind(&address);
    let maybe_pair = result
        .is_ok()
        .then(|| (fs::symlink_metadata(path), fs::symlink_metadata(&address)));
    fs::remove_file(alias).expect("remove only owned alias before bind assertion");
    drop(result.expect("real socket through short address"));
    let (selected, short) = maybe_pair.expect("successful bind metadata");
    let selected = selected.expect("physical selected socket metadata");
    let short = short.expect("short alias selected metadata");
    assert!(selected.file_type().is_socket());
    assert_eq!((selected.dev(), selected.ino()), (short.dev(), short.ino()));
}

fn replace_opened(path: &Path, replacement: Replacement) {
    let held = File::open(path).expect("held original inode");
    let before = held.metadata().expect("original safe metadata");
    assert_eq!(before.nlink(), 1);
    if matches!(replacement, Replacement::OpenedHardlink) {
        fs::hard_link(path, path.with_extension("anchor")).expect("actual opened second link");
        assert_eq!(
            held.metadata().expect("opened hardlink metadata").nlink(),
            2
        );
        return;
    }
    if matches!(replacement, Replacement::HeldMode) {
        fs::set_permissions(path, fs::Permissions::from_mode(0o640)).expect("readable held chmod");
    }
    fs::remove_file(path).expect("last unlink of actual validated file");
    match replacement {
        Replacement::Missing | Replacement::HeldMode => {}
        Replacement::Symlink => symlink(path.with_extension("absent"), path).expect("symlink"),
        Replacement::Directory => fs::create_dir(path).expect("directory replacement"),
        Replacement::Socket => socket_replacement(path),
        Replacement::Hardlink => {
            let anchor = path.with_extension("anchor");
            fs::write(&anchor, [9; 4]).expect("replacement anchor");
            fs::hard_link(anchor, path).expect("real two-link pathname");
            assert_eq!(
                fs::symlink_metadata(path)
                    .expect("two-link path metadata")
                    .nlink(),
                2
            );
        }
        Replacement::Safe | Replacement::WrongMode => {
            fs::write(path, [9; 4]).expect("different inode regular replacement");
            let mode = if matches!(replacement, Replacement::WrongMode) {
                0o640
            } else {
                before.mode() & 0o777
            };
            fs::set_permissions(path, fs::Permissions::from_mode(mode)).expect("replacement mode");
            assert_ne!(
                fs::symlink_metadata(path)
                    .expect("different path inode")
                    .ino(),
                before.ino()
            );
        }
        Replacement::OpenedHardlink => {}
    }
    let after = held
        .metadata()
        .expect("actual unlinked descriptor metadata");
    assert!(after.is_file());
    assert_eq!(
        (after.dev(), after.ino(), after.nlink()),
        (before.dev(), before.ino(), 0)
    );
    eprintln!(
        "OPENED_UNLINK_CASE case={replacement:?} held_regular=true links={} inode={}",
        after.nlink(),
        after.ino()
    );
}

fn case_hook(selected: PathBuf, replacement: Replacement) -> Arc<AtomicUsize> {
    let calls = Arc::new(AtomicUsize::new(0));
    let observed = Arc::clone(&calls);
    MAYBE_HOOK.set(Some(Box::new(move |point, path| {
        if point == Point::Opened && path == selected {
            observed.fetch_add(1, Ordering::SeqCst);
            replace_opened(path, replacement);
        }
    })));
    calls
}

fn assert_original(error: StorageError) {
    assert_eq!(error, inspection_error("unsafe inspection source hardlink"));
}

fn isolated_case(replacement: Replacement, changed: bool) {
    // Each strict/typed observation has a fresh inode, hook, and owned root.
    for strict in [false, true] {
        let root = isolated_source();
        fs::set_permissions(root.join("payload"), fs::Permissions::from_mode(0o600))
            .expect("known original mode");
        let calls = case_hook(root.join("payload"), replacement);
        let mut work = copy::WorkLedger::default();
        let result = if strict {
            copy::Inventory::capture(&root).map_err(copy::CopyFailure::Refused)
        } else {
            copy::Inventory::capture_typed(&root, &mut Some(&mut work))
        };
        MAYBE_HOOK.set(None);
        fs::remove_dir_all(root).expect("cleanup before result assertions");
        assert_eq!(
            calls.load(Ordering::SeqCst),
            1,
            "case={replacement:?} strict={strict}"
        );
        let error = result.expect_err("no accepted Inventory after replacement");
        if !strict {
            assert_eq!(
                matches!(&error, copy::CopyFailure::Changed(_)),
                changed,
                "case={replacement:?} result={error:?}"
            );
            assert_eq!((work.visits, work.content, work.probes), (2, 4, 1));
        }
        assert_original(error.original());
    }
}

fn live_refusal(replacement: Replacement) {
    with_store(|store| {
        let path = store.datadir().join("!phase167-opened-unlink");
        fs::write(&path, [1; 4]).expect("owned non-native sidecar");
        fs::set_permissions(&path, fs::Permissions::from_mode(0o600))
            .expect("original sidecar mode");
        let before = rows(store);
        let stamp = store
            .database()
            .stamp()
            .expect("original stamp before live operation");
        let calls = case_hook(path, replacement);
        let result = store.check_submission_physical_inventory();
        MAYBE_HOOK.set(None);
        assert_eq!(calls.load(Ordering::SeqCst), 1);
        assert_original(result.expect_err("unsafe zero must not continue"));
        TRACE.with_borrow(|trace| {
            assert_eq!(trace.starts.len(), 1);
            assert_eq!(trace.changed, 0);
            assert!(trace.captures.is_empty() && trace.equals.is_empty());
        });
        assert_eq!(rows(store), before);
        store
            .database()
            .check_stamp(&stamp)
            .expect("original logical authority intact");
    });
}

#[test]
fn phase167_live_physical_opened_unlink_hardlinks_refused() {
    // Arrange: actual two-name file before any path constructor.
    let root = isolated_source();
    fs::hard_link(root.join("payload"), root.join("anchor")).expect("actual path hardlink");
    assert_eq!(
        fs::metadata(root.join("payload"))
            .expect("path metadata")
            .nlink(),
        2
    );
    let mut work = copy::WorkLedger::default();
    // Act
    let typed = copy::Inventory::capture_typed(&root, &mut Some(&mut work));
    let strict = copy::Inventory::capture(&root);
    fs::remove_dir_all(root).expect("hardlink cleanup");
    // Assert
    assert!(matches!(&typed, Err(copy::CopyFailure::Refused(_))));
    assert_original(typed.expect_err("path hardlinks refuse").original());
    assert_original(strict.expect_err("strict path hardlinks refuse"));
    isolated_case(Replacement::OpenedHardlink, false);
    live_refusal(Replacement::OpenedHardlink);
}

#[test]
fn phase167_live_physical_opened_unlink_mode_refused() {
    // Arrange / Act / Assert: changed readable mode on the same unlinked held inode.
    isolated_case(Replacement::HeldMode, false);
    live_refusal(Replacement::HeldMode);
}

#[test]
fn phase167_live_physical_opened_unlink_unsafe_replacements_refused() {
    // Arrange / Act / Assert: fresh typed, strict and live fixtures for each unsafe pathname.
    for replacement in [
        Replacement::Symlink,
        Replacement::Directory,
        Replacement::Socket,
        Replacement::Hardlink,
        Replacement::WrongMode,
    ] {
        isolated_case(replacement, false);
        live_refusal(replacement);
    }
}

#[test]
fn phase167_live_physical_opened_unlink_safe_replacement_changed() {
    // Arrange / Act / Assert: a safe different inode aborts this complete capture.
    isolated_case(Replacement::Safe, true);
    isolated_case(Replacement::Missing, true);
}

#[test]
fn phase167_live_physical_opened_unlink_once_fresh_pair_converges() {
    with_store(|store| {
        // Arrange
        let path = store.datadir().join("!phase167-opened-unlink");
        fs::write(&path, [1; 4]).expect("owned sidecar before original stamp");
        let expected = rows(store);
        let stamp = store.database().stamp().expect("one original stamp");
        let calls = Arc::new(AtomicUsize::new(0));
        let observed = Arc::clone(&calls);
        MAYBE_HOOK.set(Some(Box::new(move |point, candidate| {
            if point == Point::Opened
                && candidate == path
                && observed.fetch_add(1, Ordering::SeqCst) == 0
            {
                replace_opened(candidate, Replacement::Missing);
                record_action();
            }
        })));
        // Act
        let result = store.check_submission_physical_inventory();
        MAYBE_HOOK.set(None);
        // Assert
        result.expect("fresh fully hashed equal pair after actual unlink");
        assert_eq!(calls.load(Ordering::SeqCst), 1);
        assert_eq!(rows(store), expected);
        store
            .database()
            .check_stamp(&stamp)
            .expect("original stamp valid");
        assert_actions_changed(1);
        assert_successful_pair();
    });
}

#[test]
fn phase167_live_physical_opened_unlink_repeated_exhausts() {
    with_store(|store| {
        // Arrange
        let root = store.datadir().to_path_buf();
        let path = root.join("!phase167-opened-unlink");
        fs::write(&path, [1; 4]).expect("owned sidecar before stamp");
        let expected = rows(store);
        let stamp = store.database().stamp().expect("one original stamp");
        MAYBE_HOOK.set(Some(Box::new(move |point, candidate| {
            if point == Point::BeforeMetadata && candidate == root {
                fs::write(&path, [1; 4]).expect("recreate only the owned sidecar");
            }
            if point == Point::Opened && candidate == path {
                replace_opened(candidate, Replacement::Missing);
                record_action();
            }
        })));
        // Act
        let result = store.check_submission_physical_inventory();
        MAYBE_HOOK.set(None);
        // Assert
        assert_eq!(
            result,
            Err(inspection_error("inspection work limit exceeded"))
        );
        assert_eq!(rows(store), expected);
        store
            .database()
            .check_stamp(&stamp)
            .expect("original stamp and all nine maps intact");
        assert_churn_exhausted(OBSERVED_WORK.get());
    });
}
