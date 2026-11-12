// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

use super::super::{FjallNodeStore, tracked::MutationStamp};
use super::{StorageError, copy, inspection_error};
#[cfg(unix)]
use std::{
    fs::{self, File, OpenOptions},
    os::unix::fs::MetadataExt,
    path::Path,
};

pub(super) fn check(store: &FjallNodeStore) -> Result<(), StorageError> {
    #[cfg(test)]
    TRACE.set(Trace::default());
    // Capture logical authority before even the first filesystem observation.
    let stamp = store.database().stamp().map_err(|_| stale())?;
    let result = check_original(store, &stamp);
    check_stamp(store, &stamp)?;
    result
}

fn stale() -> StorageError {
    inspection_error("inspection live logical source changed")
}

fn check_stamp(store: &FjallNodeStore, stamp: &MutationStamp) -> Result<(), StorageError> {
    store.database().check_stamp(stamp).map_err(|_| stale())
}

fn check_original(store: &FjallNodeStore, stamp: &MutationStamp) -> Result<(), StorageError> {
    check_stamp(store, stamp)?;
    #[cfg(not(unix))]
    {
        let mut work = copy::WorkLedger::default();
        let result = copy::Inventory::capture_typed(store.datadir(), &mut Some(&mut work))
            .and_then(|inventory| inventory.recheck_typed(store.datadir(), &mut Some(&mut work)))
            .map_err(copy::CopyFailure::original);
        check_stamp(store, stamp)?;
        result
    }
    #[cfg(unix)]
    {
        let root = Root::open(store.datadir())?;
        let mut work = copy::WorkLedger::default();
        #[cfg(test)]
        TRACE.with_borrow_mut(|trace| trace.active = true);
        let result = attempts(store, stamp, &root, &mut work);
        #[cfg(test)]
        TRACE.with_borrow_mut(|trace| trace.active = false);
        // These checks also override a Changed error with lost authority.
        root.check(store.datadir())?;
        check_stamp(store, stamp)?;
        #[cfg(test)]
        OBSERVED_WORK.set(work.visits);
        result
    }
}

#[cfg(unix)]
fn attempts(
    store: &FjallNodeStore,
    stamp: &MutationStamp,
    root: &Root,
    work: &mut copy::WorkLedger,
) -> Result<(), StorageError> {
    loop {
        match attempt_once(store, stamp, root, work) {
            Err(copy::CopyFailure::Changed(_)) => continue,
            result => return result.map_err(copy::CopyFailure::original),
        }
    }
}

#[cfg(unix)]
fn attempt_once(
    store: &FjallNodeStore,
    stamp: &MutationStamp,
    root: &Root,
    work: &mut copy::WorkLedger,
) -> Result<(), copy::CopyFailure> {
    #[cfg(test)]
    TRACE.with_borrow_mut(|trace| {
        trace.rechecking = false;
        trace.starts.push(Sample {
            ledger: work.visits,
            footprint: (0, 0, 0),
        });
    });
    check_stamp(store, stamp)?;
    root.check(store.datadir())?;
    let result = (|| {
        let inventory = copy::Inventory::capture_typed(store.datadir(), &mut Some(&mut *work))?;
        #[cfg(test)]
        {
            observe(Point::Captured, store.datadir());
            TRACE.with_borrow_mut(|trace| trace.rechecking = true);
        }
        inventory.recheck_typed(store.datadir(), &mut Some(&mut *work))
    })();
    root.check(store.datadir())?;
    check_stamp(store, stamp)?;
    #[cfg(test)]
    TRACE.with_borrow_mut(|trace| {
        if matches!(&result, Err(copy::CopyFailure::Changed(_))) {
            trace.changed += 1;
            trace.changed_attempts.push(trace.starts.len());
        }
        eprintln!(
            "LIVE_TYPED_RESULT attempt={} result={result:?}",
            trace.starts.len()
        );
    });
    result
}

// A distinct held anchor pins the inode. Inventory identities remain completely
// unchanged, including root length and mtime in every full capture and Eq.
#[cfg(unix)]
#[derive(PartialEq, Eq)]
struct Anchor {
    device: u64,
    inode: u64,
    mode: u32,
}

#[cfg(unix)]
impl Anchor {
    fn read(metadata: &fs::Metadata) -> Result<Self, StorageError> {
        if !metadata.is_dir() || metadata.file_type().is_symlink() {
            return Err(inspection_error("unsafe inspection live root"));
        }
        Ok(Self {
            device: metadata.dev(),
            inode: metadata.ino(),
            mode: metadata.mode(),
        })
    }
}

#[cfg(unix)]
struct Root {
    held: File,
    anchor: Anchor,
}

#[cfg(unix)]
impl Root {
    fn open(source: &Path) -> Result<Self, StorageError> {
        super::check_source_path(source)?;
        let anchor = Anchor::read(
            &fs::symlink_metadata(source)
                .map_err(|_| inspection_error("inspection live root unavailable"))?,
        )?;
        let held = OpenOptions::new()
            .read(true)
            .write(false)
            .open(source)
            .map_err(|_| inspection_error("inspection live root unavailable"))?;
        let root = Self { held, anchor };
        root.check(source)?;
        Ok(root)
    }

    fn check(&self, source: &Path) -> Result<(), StorageError> {
        super::check_source_path(source)?;
        let selected = fs::symlink_metadata(source)
            .map_err(|_| inspection_error("inspection live root unavailable"))?;
        let opened = self
            .held
            .metadata()
            .map_err(|_| inspection_error("inspection live root unavailable"))?;
        if self.anchor != Anchor::read(&selected)? || self.anchor != Anchor::read(&opened)? {
            return Err(inspection_error("inspection live root changed"));
        }
        Ok(())
    }
}

#[cfg(test)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum Point {
    BeforeMetadata,
    DirectoryRead,
    Opened,
    Captured,
}

#[cfg(test)]
type Hook = Box<dyn FnMut(Point, &std::path::Path)>;

#[cfg(test)]
thread_local! {
    static MAYBE_HOOK: std::cell::RefCell<Option<Hook>> = const { std::cell::RefCell::new(None) };
    static OBSERVED_WORK: std::cell::Cell<u64> = const { std::cell::Cell::new(0) };
    static TRACE: std::cell::RefCell<Trace> = std::cell::RefCell::new(Trace::default());
}

#[cfg(test)]
#[derive(Clone, Copy, Debug)]
struct Sample {
    ledger: u64,
    footprint: (usize, u64, usize),
}

#[cfg(test)]
#[derive(Default)]
struct Trace {
    active: bool,
    rechecking: bool,
    starts: Vec<Sample>,
    captures: Vec<Sample>,
    rechecks: Vec<Sample>,
    equals: Vec<Sample>,
    changed: usize,
    changed_attempts: Vec<usize>,
    actions: Vec<usize>,
    root_links: Vec<(u64, u64)>,
}

#[cfg(test)]
pub(super) fn record_walk(footprint: (usize, u64, usize), ledger: u64, equal: bool) {
    TRACE.with_borrow_mut(|trace| {
        if !trace.active {
            return;
        }
        let sample = Sample { ledger, footprint };
        if equal {
            trace.equals.push(sample);
        } else if trace.rechecking {
            trace.rechecks.push(sample);
        } else {
            trace.captures.push(sample);
        }
    });
}

#[cfg(test)]
pub(super) fn record_root_links(before: u64, after: u64) {
    TRACE.with_borrow_mut(|trace| {
        if trace.active {
            trace.root_links.push((before, after));
        }
    });
}

#[cfg(test)]
pub(super) fn observe(point: Point, path: &std::path::Path) {
    let maybe_hook = MAYBE_HOOK.take();
    if let Some(mut hook) = maybe_hook {
        let active = TRACE.with_borrow_mut(|trace| std::mem::replace(&mut trace.active, false));
        hook(point, path);
        TRACE.with_borrow_mut(|trace| trace.active = active);
        MAYBE_HOOK.set(Some(hook));
    }
}

#[cfg(all(test, unix))]
mod tests;
