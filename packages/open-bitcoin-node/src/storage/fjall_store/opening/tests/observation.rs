// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

use super::*;

#[derive(Debug, Default)]
pub(super) struct Physical {
    bytes: u64,
    entries: usize,
    depth: usize,
    pub(super) journals: Vec<JournalIdentity>,
    maybe_root: Option<fs::Metadata>,
}

#[derive(Debug, Default, Clone, Copy)]
pub(super) struct ObservationWork {
    pub(super) visits: usize,
    pub(super) bytes: u64,
    closing: usize,
}

#[derive(Debug)]
pub(super) enum Observation {
    Complete(Physical),
    Changed(io::Error, ObservationWork),
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum ObservationPoint {
    BeforeMetadata,
    BeforeDirectoryRead,
}

enum ObservationFailure {
    Changed(io::Error),
    Refused(io::Error),
}

impl From<io::Error> for ObservationFailure {
    fn from(error: io::Error) -> Self {
        Self::Refused(error)
    }
}

fn observation_io(error: io::Error, known: bool, stage: &str, path: &Path) -> ObservationFailure {
    let bytes = path.as_os_str().as_encoded_bytes();
    eprintln!(
        "FIXTURE_OBSERVATION_IO stage={stage} known={known} kind={:?} os={:?} path_prefix={:?}",
        error.kind(),
        error.raw_os_error(),
        &bytes[..bytes.len().min(256)]
    );
    if known && error.kind() == io::ErrorKind::NotFound {
        ObservationFailure::Changed(error)
    } else {
        ObservationFailure::Refused(error)
    }
}

#[derive(Default)]
pub(super) struct ObservationSlots {
    used: usize,
}

impl ObservationSlots {
    pub(super) fn observe(
        &mut self,
        root: &Path,
        hook: &mut impl FnMut(ObservationPoint, &Path),
    ) -> io::Result<Observation> {
        self.charge()?;
        Physical::observe_with(root, &mut ObservationWork::default(), hook)
    }

    fn limit() -> usize {
        usize::try_from(1 + OBSERVATION.as_millis() / Duration::from_millis(5).as_millis())
            .expect("fixed observation slots fit")
    }

    fn charge(&mut self) -> io::Result<()> {
        if self.used >= Self::limit() {
            return Err(io::Error::other("fixture observation slots exhausted"));
        }
        self.used += 1;
        Ok(())
    }
}

impl Observation {
    pub(super) fn maybe_finished(
        &self,
        original: &JournalIdentity,
        elapsed: impl FnOnce() -> Duration,
    ) -> Option<bool> {
        if matches!(self, Self::Complete(physical) if !physical.journals.contains(original)) {
            return Some(true);
        }
        (elapsed() >= OBSERVATION).then_some(false)
    }

    pub(super) fn report(&self, stage: &str, writes: usize, started: Instant) {
        match self {
            Self::Complete(physical) => physical.report(stage, writes, started),
            Self::Changed(error, work) => eprintln!(
                "JOURNAL_PRESSURE stage={stage}-changed writes={writes} payload_bytes={} elapsed_ms={} complete=false spent={work:?} error={error:?}",
                writes * VALUE_BYTES,
                started.elapsed().as_millis()
            ),
        }
    }
}

impl ObservationWork {
    fn visit(&mut self, target: &Path, depth: usize) -> io::Result<()> {
        if self.visits >= 4096 || depth > 16 || target.as_os_str().as_encoded_bytes().len() > 4096 {
            return Err(io::Error::other(
                "fixture crossed unchanged physical bounds",
            ));
        }
        self.visits += 1;
        Ok(())
    }
}

impl Physical {
    pub(super) fn read(root: &Path) -> io::Result<Self> {
        match Self::observe(root)? {
            Observation::Complete(physical) => Ok(physical),
            Observation::Changed(error, _) => Err(error),
        }
    }

    pub(super) fn observe(root: &Path) -> io::Result<Observation> {
        ObservationSlots::default().observe(root, &mut |_, _| {})
    }

    pub(super) fn observe_with(
        root: &Path,
        work: &mut ObservationWork,
        hook: &mut impl FnMut(ObservationPoint, &Path),
    ) -> io::Result<Observation> {
        let mut result = Self::default();
        let maybe_changed = match result.visit(root, Path::new(""), 0, work, hook) {
            Ok(()) => None,
            Err(ObservationFailure::Changed(error)) => Some(error),
            Err(ObservationFailure::Refused(error)) => return Err(error),
        };
        result.close_root(root, work)?;
        if let Some(error) = maybe_changed {
            return Ok(Observation::Changed(error, *work));
        }
        result.journals.sort_by(|a, b| a.relative.cmp(&b.relative));
        Ok(Observation::Complete(result))
    }

    fn close_root(&self, root: &Path, work: &mut ObservationWork) -> io::Result<()> {
        work.closing += 1;
        let original = self
            .maybe_root
            .as_ref()
            .ok_or_else(|| io::Error::other("fixture root not observed"))?;
        let current = fs::symlink_metadata(root)?;
        let changed = !current.is_dir() || current.file_type().is_symlink();
        #[cfg(unix)]
        let changed = changed
            || (original.dev(), original.ino(), original.mode())
                != (current.dev(), current.ino(), current.mode());
        #[cfg(not(unix))]
        let changed = changed || !original.is_dir();
        if changed {
            return Err(io::Error::other("fixture root identity changed"));
        }
        Ok(())
    }

    fn visit(
        &mut self,
        root: &Path,
        relative: &Path,
        depth: usize,
        work: &mut ObservationWork,
        hook: &mut impl FnMut(ObservationPoint, &Path),
    ) -> Result<(), ObservationFailure> {
        let target = root.join(relative);
        work.visit(&target, depth)?;
        hook(ObservationPoint::BeforeMetadata, &target);
        let known = !relative.as_os_str().is_empty();
        let metadata = fs::symlink_metadata(&target)
            .map_err(|error| observation_io(error, known, "metadata", relative))?;
        if metadata.file_type().is_symlink() || (!metadata.is_dir() && !metadata.is_file()) {
            return Err(io::Error::other("unsafe fixture tree entry").into());
        }
        if !known {
            if !metadata.is_dir() {
                return Err(io::Error::other("unsafe fixture root").into());
            }
            self.maybe_root = Some(metadata.clone());
        }
        self.entries += 1;
        self.depth = self.depth.max(depth);
        if metadata.is_file() {
            let next = work
                .bytes
                .checked_add(metadata.len())
                .ok_or_else(|| io::Error::other("fixture byte overflow"))?;
            if next > 512 * 1024 * 1024 {
                return Err(io::Error::other("fixture crossed unchanged physical bounds").into());
            }
            work.bytes = next;
            self.bytes = next;
            if relative.extension().is_some_and(|suffix| suffix == "jnl") {
                self.journals.push(JournalIdentity {
                    relative: relative.to_owned(),
                    #[cfg(unix)]
                    device: metadata.dev(),
                    #[cfg(unix)]
                    inode: metadata.ino(),
                });
            }
        } else {
            hook(ObservationPoint::BeforeDirectoryRead, &target);
            for entry in fs::read_dir(&target)
                .map_err(|error| observation_io(error, known, "directory", relative))?
            {
                self.visit(
                    root,
                    &relative.join(entry?.file_name()),
                    depth + 1,
                    work,
                    hook,
                )?;
            }
        }
        Ok(())
    }

    pub(super) fn report(&self, stage: &str, writes: usize, started: Instant) {
        eprintln!(
            "JOURNAL_PRESSURE stage={stage} writes={writes} payload_bytes={} elapsed_ms={} total_bytes={} entries={} depth={} journal_ids={:?}",
            writes * VALUE_BYTES,
            started.elapsed().as_millis(),
            self.bytes,
            self.entries,
            self.depth,
            self.journals
        );
    }
}

fn with_observer_tree(test: impl FnOnce(&Path)) {
    let root = source();
    fs::create_dir(&root).expect("owned observer tree");
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| test(&root)));
    if let Err(error) = fs::remove_dir_all(&root) {
        assert_eq!(error.kind(), io::ErrorKind::NotFound);
    }
    if let Err(error) = result {
        std::panic::resume_unwind(error);
    }
}

pub(super) fn known_directory_notfound_changed() {
    with_observer_tree(|root| {
        // Arrange
        let child = root.join("directory");
        fs::create_dir(&child).expect("known directory");
        let mut work = ObservationWork::default();
        // Act
        let observed = Physical::observe_with(root, &mut work, &mut |point, path| {
            if point == ObservationPoint::BeforeDirectoryRead && path == child {
                fs::remove_dir(path).expect("real known directory retirement");
            }
        });
        // Assert
        assert!(
            matches!(observed, Ok(Observation::Changed(error, _)) if error.kind() == io::ErrorKind::NotFound)
        );
        assert_eq!((work.visits, work.closing), (2, 1));
        fs::write(root.join("fresh"), [3; 4]).expect("independent fresh file");
        let complete = Physical::read(root).expect("new complete observation");
        assert_eq!((complete.entries, complete.bytes), (2, 4));
    });
}

#[cfg(unix)]
pub(super) fn root_unsafe_and_permanent_refused() {
    use std::os::unix::{
        fs::{PermissionsExt, symlink},
        net::UnixListener,
    };
    for case in [
        "missing",
        "removed",
        "replaced",
        "mode",
        "symlink",
        "socket",
        "permission",
    ] {
        with_observer_tree(|root| {
            // Arrange
            let child = root.join("x");
            let held = fs::File::open(root).expect("retain real original root inode");
            let original_mode = held
                .metadata()
                .expect("observed original readable mode")
                .permissions()
                .mode();
            match case {
                "missing" => fs::remove_dir(root).expect("actual absent root"),
                "symlink" => symlink("absent", &child).expect("unsafe actual symlink"),
                "socket" => {
                    let alias = fs::canonicalize("/tmp")
                        .expect("short parent")
                        .join(root.file_name().expect("unique owned alias"));
                    symlink(root, &alias).expect("unique alias");
                    let socket = UnixListener::bind(alias.join("x"));
                    fs::remove_file(alias).expect("remove alias before assertion");
                    drop(socket.expect("actual socket through short address"));
                }
                "permission" => {
                    fs::create_dir(&child).expect("denied directory");
                    fs::set_permissions(&child, fs::Permissions::from_mode(0o0))
                        .expect("real denied mode");
                }
                _ => {}
            }
            // Act
            let result = Physical::observe_with(
                root,
                &mut ObservationWork::default(),
                &mut |point, path| {
                    if point != ObservationPoint::BeforeDirectoryRead || path != root {
                        return;
                    }
                    if case == "removed" || case == "replaced" {
                        fs::remove_dir(root).expect("remove observed original root");
                    }
                    if case == "replaced" {
                        fs::create_dir(root).expect("different root inode");
                        assert_ne!(
                            held.metadata().expect("held inode").ino(),
                            fs::metadata(root).expect("new inode").ino()
                        );
                    }
                    if case == "mode" {
                        fs::set_permissions(
                            root,
                            fs::Permissions::from_mode((original_mode & 0o7777) ^ 0o010),
                        )
                        .expect("different readable root mode");
                    }
                },
            );
            if case == "permission" {
                fs::set_permissions(&child, fs::Permissions::from_mode(0o700))
                    .expect("restore denied directory before assertions");
            }
            // Assert
            let error = result.expect_err("root/unsafe/permanent errors never become Changed");
            if case == "permission" {
                assert_eq!(
                    error.kind(),
                    io::ErrorKind::PermissionDenied,
                    "actual permanent OS error required"
                );
            }
            eprintln!("FIXTURE_REFUSED case={case} actual_kind={:?}", error.kind());
        });
    }
}

pub(super) fn limits_precede_walk_io() {
    with_observer_tree(|root| {
        // Arrange / Act / Assert: denied entry/depth/path charges precede all walker hooks.
        let long = "x".repeat(4097);
        for (relative, depth, visits) in [
            ("missing", 0, 4096),
            ("missing", 17, 0),
            (long.as_str(), 0, 0),
        ] {
            let mut work = ObservationWork {
                visits,
                ..Default::default()
            };
            let mut hooks = 0;
            let result = Physical::default().visit(
                root,
                Path::new(relative),
                depth,
                &mut work,
                &mut |_, _| hooks += 1,
            );
            assert!(matches!(result, Err(ObservationFailure::Refused(_))));
            assert_eq!((hooks, work.visits, work.closing), (0, visits, 0));
        }
        let mut exact = ObservationWork::default();
        for _ in 0..4096 {
            exact
                .visit(Path::new("x"), 16)
                .expect("exact entry/depth allowance");
        }
        assert!(exact.visit(Path::new("x"), 16).is_err());
        ObservationWork::default()
            .visit(Path::new(&"x".repeat(4096)), 16)
            .expect("exact path allowance");
        let file = fs::File::create(root.join("sparse")).expect("sparse metadata fixture");
        file.set_len(512 * 1024 * 1024)
            .expect("exact sparse byte allowance");
        assert_eq!(
            Physical::read(root)
                .expect("exact metadata byte limit")
                .bytes,
            512 * 1024 * 1024
        );
        file.set_len(512 * 1024 * 1024 + 1)
            .expect("one-over sparse metadata");
        assert!(
            Physical::observe(root).is_err(),
            "no file contents were read"
        );
    });
}

pub(super) fn funded_default_churn_and_denied_edge() {
    with_observer_tree(|root| {
        // Arrange
        fs::write(root.join("0.jnl"), [1]).expect("real original journal identity");
        let original = Physical::read(root)
            .expect("real original complete walk")
            .journals
            .remove(0);
        fs::remove_file(root.join("0.jnl")).expect("remove only owned original journal");
        let mut slots = ObservationSlots::default();
        let mut total = 0;
        let mut maybe_last = None;
        // Act
        assert_eq!(ObservationSlots::limit(), 6001);
        for _ in 0..ObservationSlots::limit() {
            let result = slots
                .observe(root, &mut |point, path| {
                    if point == ObservationPoint::BeforeMetadata && path == root {
                        fs::write(root.join("x"), [1; 4]).expect("owned churn only");
                    }
                    if point == ObservationPoint::BeforeMetadata && path != root {
                        fs::remove_file(path).expect("real enumerated file unlink");
                    }
                })
                .expect("funded actual observation");
            let Observation::Changed(ref error, spent) = result else {
                panic!("no partial complete result")
            };
            assert_eq!(error.kind(), io::ErrorKind::NotFound);
            assert_eq!((spent.visits, spent.bytes, spent.closing), (2, 0, 1));
            total += spent.visits;
            maybe_last = Some(result);
        }
        // Assert
        let changed = maybe_last.expect("real final incomplete observation");
        assert_eq!(
            changed.maybe_finished(&original, || OBSERVATION),
            Some(false)
        );
        assert_eq!(total, 12002);
        let mut hooks = 0;
        assert!(slots.observe(root, &mut |_, _| hooks += 1).is_err());
        assert_eq!(hooks, 0, "denied slot performs no walk");
        let mut final_slot = ObservationSlots { used: 6000 };
        let complete = final_slot
            .observe(root, &mut |_, _| {})
            .expect("final funded complete observation");
        assert_eq!(
            complete.maybe_finished(&original, || panic!(
                "complete retirement precedes deadline check"
            )),
            Some(true)
        );
        assert!(final_slot.charge().is_err());
        eprintln!(
            "FIXTURE_DEFAULT_CHURN slots={} visits={total} denied_hooks={hooks} complete_final_slot_retired=true",
            slots.used
        );
    });
}
