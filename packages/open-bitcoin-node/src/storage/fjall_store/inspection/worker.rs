// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

//! Same-executable scratch recovery. The parent alone owns source and cleanup.

use super::{
    InspectedRecords, OperationLease, ScratchGuard, StorageError, copy, inspection_error,
    protocol::{self, OwnedScratchJob},
};
#[cfg(unix)]
use std::os::unix::fs::{MetadataExt, OpenOptionsExt};
use std::{
    ffi::OsString,
    fs::{self, OpenOptions},
    io::Write,
    path::Path,
    process::{Child, Command, ExitStatus, Stdio},
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};

const ARGS: [&str; 4] = [
    "--exact",
    "__open_bitcoin_storage_inspection_worker",
    "--nocapture",
    "--test-threads=1",
];
const REFUSAL: i32 = 78;
const DEADLINE: Duration = Duration::from_secs(30);

pub(super) enum Completion<T> {
    Reaped(Result<T, StorageError>),
    Uncertain(StorageError),
}

fn refused() -> StorageError {
    inspection_error("inspection isolated recovery refused")
}
fn route(args: impl Iterator<Item = OsString>) -> bool {
    args.eq(ARGS.iter().map(OsString::from))
}

pub(in crate::storage::fjall_store) fn maybe_run() -> Option<i32> {
    if !route(std::env::args_os().skip(1)) {
        return None;
    }
    // Installed solely in an exact worker child; never format untrusted payloads
    // or unwind through poisoned engine destructors.
    std::panic::set_hook(Box::new(|_| std::process::exit(REFUSAL)));
    Some(if run_child().is_ok() { 0 } else { REFUSAL })
}

fn run_child() -> Result<(), StorageError> {
    let job = OwnedScratchJob::decode(std::io::stdin().lock())?;
    job.check_root()?;
    let mut names = std::collections::BTreeSet::new();
    for entry in fs::read_dir(&job.root).map_err(|_| refused())? {
        names.insert(entry.map_err(|_| refused())?.file_name());
        if names.len() > 2 {
            return Err(refused());
        }
    }
    if names
        != std::collections::BTreeSet::from([OsString::from("database"), OsString::from("result")])
    {
        return Err(refused());
    }
    #[cfg(test)]
    tests::child_before_read(&job)?;
    let database = job.root.join("database");
    let inventory = copy::Inventory::capture(&database)?;
    #[cfg(unix)]
    check_private_layout(&database)?;
    drop(inventory);
    let records = {
        let _scratch_lease = OperationLease::acquire(&database)?;
        raw_recover(&database)?
    };
    // DatabaseInner::drop stops and joins Fjall workers before returning. No
    // database/keyspace/lease owner escapes the scope above, even on refusal.
    let mut output = job.result_file(true)?;
    #[cfg(test)]
    if tests::child_output(&job, &mut output)? {
        return Ok(());
    }
    protocol::write_result(&mut output, &job, &records)?;
    output.sync_all().map_err(|_| refused())?;
    drop(output);
    Ok(())
}

#[cfg(unix)]
fn check_private_layout(root: &Path) -> Result<(), StorageError> {
    let metadata = fs::symlink_metadata(root).map_err(|_| refused())?;
    protocol::check_metadata(&metadata, metadata.is_dir(), metadata.dev(), metadata.ino())?;
    if metadata.is_dir() {
        for entry in fs::read_dir(root).map_err(|_| refused())? {
            check_private_layout(&entry.map_err(|_| refused())?.path())?;
        }
    }
    Ok(())
}

// Only run_child and the separately controlled raw-pin characterization call
// this adapter. The live inspecting parent never opens a scratch backend.
pub(super) fn raw_recover(database_path: &Path) -> Result<InspectedRecords, StorageError> {
    let database = fjall::Database::builder(database_path)
        .worker_threads(1)
        .cache_size(8 * 1024 * 1024)
        .open()
        .map_err(|_| refused())?;
    InspectedRecords::read_existing(&database)
}

fn job(scratch: &ScratchGuard) -> Result<OwnedScratchJob, StorageError> {
    let mut options = OpenOptions::new();
    options.write(true).create_new(true);
    #[cfg(unix)]
    options.mode(0o600);
    let file = options
        .open(scratch.root.join("result"))
        .map_err(|_| refused())?;
    let metadata = file.metadata().map_err(|_| refused())?;
    let mut nonce = [0; 16];
    getrandom::fill(&mut nonce).map_err(|_| refused())?;
    #[cfg(unix)]
    let (device, inode, result_device, result_inode) = (
        scratch.device,
        scratch.inode,
        metadata.dev(),
        metadata.ino(),
    );
    #[cfg(not(unix))]
    let (device, inode, result_device, result_inode) = (0, 0, 0, 0);
    Ok(OwnedScratchJob {
        root: scratch.root.clone(),
        device,
        inode,
        result_device,
        result_inode,
        nonce,
        #[cfg(test)]
        fault: tests::FAULT.get(),
    })
}

pub(super) fn recover(scratch: &ScratchGuard) -> Completion<InspectedRecords> {
    let result = (|| {
        let job = job(scratch)?;
        let bytes = job.encode()?;
        let executable = std::env::current_exe().map_err(|_| refused())?;
        let mut command = Command::new(executable);
        command
            .args(ARGS)
            .stdin(Stdio::piped())
            .stdout(Stdio::null())
            .stderr(Stdio::null());
        #[cfg(test)]
        tests::configure_command(&mut command, job.fault);
        let started = Instant::now();
        let child = command.spawn().map_err(|_| refused())?;
        Ok((job, bytes, child, started))
    })();
    let (job, bytes, mut child, started) = match result {
        Ok(value) => value,
        Err(error) => return Completion::Reaped(Err(error)),
    };
    let Some(mut stdin) = child.stdin.take() else {
        return terminate(&mut child, refused());
    };
    let (sender, receiver) = mpsc::sync_channel(1);
    #[cfg(test)]
    let (cancel_send, cancel_receive) = mpsc::sync_channel::<()>(0);
    #[cfg(test)]
    let fault = job.fault;
    let writer = thread::spawn(move || {
        #[cfg(test)]
        if tests::input_stalled(fault) {
            // Test a child waiting for its still-unsent stdin job. Dropping the
            // parent sender cancels this seam after the monitored child reaps.
            let _canceled = cancel_receive.recv();
            return;
        }
        let result = stdin.write_all(&bytes);
        drop(stdin);
        // Receiver may have left only after an uncertain child-lifecycle error.
        let _maybe_delivery = sender.send(result);
    });
    let outcome = monitor(&mut child, &job, started, &receiver);
    #[cfg(test)]
    drop(cancel_send);
    if matches!(outcome, Completion::Uncertain(_)) {
        return outcome;
    }
    if writer.join().is_err() {
        return Completion::Reaped(Err(refused()));
    }
    outcome
}

fn monitor(
    child: &mut Child,
    job: &OwnedScratchJob,
    started: Instant,
    receiver: &mpsc::Receiver<std::io::Result<()>>,
) -> Completion<InspectedRecords> {
    let mut delivered = false;
    loop {
        match receiver.try_recv() {
            Ok(Ok(())) => delivered = true,
            Ok(Err(_)) => return terminate(child, refused()),
            Err(mpsc::TryRecvError::Disconnected) if !delivered => {
                return terminate(child, refused());
            }
            Err(_) => {}
        }
        let deadline = DEADLINE;
        #[cfg(test)]
        let deadline = tests::deadline(job.fault, deadline);
        if started.elapsed() >= deadline {
            return terminate(
                child,
                inspection_error("inspection worker deadline exceeded"),
            );
        }
        if job.result_file(false).is_err() {
            return terminate(child, refused());
        }
        #[cfg(test)]
        if tests::is_uncertain(job.fault) {
            return tests::uncertain(child, job.fault);
        }
        match child.try_wait() {
            Ok(Some(status)) if delivered || !status.success() => {
                return Completion::Reaped(finish(status, delivered, job));
            }
            Ok(Some(_)) => thread::sleep(Duration::from_millis(5)),
            Ok(None) => thread::sleep(Duration::from_millis(5)),
            Err(_) => return terminate(child, inspection_error("inspection worker wait failed")),
        }
    }
}

fn finish(
    status: ExitStatus,
    delivered: bool,
    job: &OwnedScratchJob,
) -> Result<InspectedRecords, StorageError> {
    if !status.success() || !delivered {
        return Err(refused());
    }
    let file = job.result_file(false)?;
    let length = file.metadata().map_err(|_| refused())?.len();
    let records = protocol::read_result(&file, length, job)?;
    // Recheck both opened and selected metadata after bounded decoding.
    let selected = job.result_file(false)?;
    if selected.metadata().map_err(|_| refused())?.len() != length {
        return Err(refused());
    }
    Ok(records)
}

fn terminate<T>(child: &mut Child, failure: StorageError) -> Completion<T> {
    match child.try_wait() {
        Ok(Some(_)) => return Completion::Reaped(Err(failure)),
        Ok(None) => {}
        Err(_) => {
            return Completion::Uncertain(inspection_error(
                "inspection worker wait failed; completion uncertain",
            ));
        }
    }
    if child.kill().is_err() {
        return Completion::Uncertain(inspection_error(
            "inspection worker kill failed; completion uncertain",
        ));
    }
    match child.wait() {
        Ok(_) => Completion::Reaped(Err(failure)),
        Err(_) => Completion::Uncertain(inspection_error(
            "inspection worker reap failed; completion uncertain",
        )),
    }
}

#[cfg(all(test, unix))]
mod tests;
