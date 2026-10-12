// Shared std-only test subprocess supervision; excluded from production builds.
use std::{
    fmt,
    io::{self, Read, Write},
    process::{Child, Command, ExitStatus, Stdio},
    sync::mpsc::{self, Receiver, Sender},
    thread::{self, JoinHandle},
    time::{Duration, Instant},
};

#[derive(Clone, Copy)]
pub struct Limits {
    pub input_bytes: usize,
    pub stdout_bytes: usize,
    pub stderr_bytes: usize,
    pub deadline: Duration,
}

// Five raw batches, five wire batches and bounded Git probes fit this total cap.
pub const ORACLE_LIMITS: Limits = Limits {
    input_bytes: 8 * 1024 * 1024,
    stdout_bytes: 16 * 1024 * 1024,
    stderr_bytes: 64 * 1024,
    deadline: Duration::from_secs(150),
};

pub struct Captured {
    pub stdout: Vec<u8>,
    pub stderr: Vec<u8>,
    pub pid: u32,
    pub status: ExitStatus,
    pub reaped: bool,
}

pub struct Failure {
    pub reason: String,
    pub stdout: Vec<u8>,
    pub stderr: Vec<u8>,
    pub maybe_pid: Option<u32>,
    pub reaped: bool,
}

impl fmt::Display for Failure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{} (stdout={} bytes, stderr={} bytes, reaped={}): {}",
            self.reason,
            self.stdout.len(),
            self.stderr.len(),
            self.reaped,
            String::from_utf8_lossy(&self.stderr[..self.stderr.len().min(2048)])
        )
    }
}

impl fmt::Debug for Failure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, formatter)
    }
}

struct ChildGuard {
    child: Child,
    group_cleaned: bool,
    reaped: bool,
}

#[cfg(unix)]
fn configure_group(command: &mut Command) -> io::Result<()> {
    use std::os::unix::process::CommandExt;
    // A fresh PGID equal to the new child's PID cannot contain this parent.
    command.process_group(0);
    Ok(())
}

#[cfg(not(unix))]
fn configure_group(_: &mut Command) -> io::Result<()> {
    Err(io::Error::new(
        io::ErrorKind::Unsupported,
        "test oracle supervision requires Unix process groups",
    ))
}

struct SignalGuard(Child);

impl Drop for SignalGuard {
    fn drop(&mut self) {
        if self
            .0
            .try_wait()
            .is_ok_and(|maybe_status| maybe_status.is_some())
        {
            return;
        }
        if let Err(error) = self.0.kill() {
            eprintln!("native signal child kill failed: {error}");
        }
        if let Err(error) = self.0.wait() {
            eprintln!("native signal child reap failed: {error}");
        }
    }
}

fn kill_owned_group(pid: u32) -> io::Result<()> {
    if pid <= 1 || pid == std::process::id() {
        return Err(io::Error::other("refusing parent/system process group"));
    }
    // Only a PGID derived from our freshly process_group(0)-spawned child enters.
    // /bin/kill is the native Unix signal adapter; no shell or external library.
    let mut killer = SignalGuard(
        Command::new("/bin/kill")
            .args(["-KILL", "--", &format!("-{pid}")])
            .env("LC_ALL", "C")
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::piped())
            .spawn()?,
    );
    let started = Instant::now();
    let status = loop {
        if let Some(status) = killer.0.try_wait()? {
            break status;
        }
        if started.elapsed() >= Duration::from_secs(1) {
            let killed = killer.0.kill();
            let reaped = killer.0.wait();
            killed?;
            reaped?;
            return Err(io::Error::new(
                io::ErrorKind::TimedOut,
                "owned-group signal command deadline exceeded",
            ));
        }
        thread::sleep(Duration::from_millis(2));
    };
    let mut error = Vec::new();
    killer
        .0
        .stderr
        .take()
        .expect("signal stderr")
        .take(2048)
        .read_to_end(&mut error)?;
    if status.success() || String::from_utf8_lossy(&error).contains("No such process") {
        return Ok(());
    }
    Err(io::Error::other(format!(
        "owned-group signal failed: {}",
        String::from_utf8_lossy(&error)
    )))
}

impl ChildGuard {
    fn stop(&mut self) -> io::Result<ExitStatus> {
        let group = if self.group_cleaned {
            Ok(())
        } else {
            kill_owned_group(self.child.id())
        };
        self.group_cleaned |= group.is_ok();
        if self.child.try_wait()?.is_none()
            && let Err(error) = self.child.kill()
        {
            // A child may exit between try_wait and kill; require proof of exit.
            if self.child.try_wait()?.is_none() {
                return Err(error);
            }
        }
        let status = self.child.wait();
        self.reaped |= status.is_ok();
        group.and(status)
    }
}

impl Drop for ChildGuard {
    fn drop(&mut self) {
        if let Err(error) = self.stop() {
            eprintln!("test child cleanup failed: {error}");
        }
    }
}

enum Event {
    Input(io::Result<()>),
    Read {
        stderr: bool,
        bytes: Vec<u8>,
        maybe_error: Option<String>,
    },
}

fn drain(
    reader: impl Read + Send + 'static,
    limit: usize,
    stderr: bool,
    sender: Sender<Event>,
) -> JoinHandle<()> {
    thread::spawn(move || {
        let mut reader = reader;
        let mut captured = Vec::new();
        let mut buffer = [0_u8; 8192];
        loop {
            match reader.read(&mut buffer) {
                Ok(0) => {
                    sender
                        .send(Event::Read {
                            stderr,
                            bytes: captured,
                            maybe_error: None,
                        })
                        .expect("supervisor remains until readers finish");
                    return;
                }
                Ok(count) => {
                    let keep = count.min(limit.saturating_sub(captured.len()));
                    captured.extend_from_slice(&buffer[..keep]);
                    if keep == count {
                        continue;
                    }
                    sender
                        .send(Event::Read {
                            stderr,
                            bytes: captured,
                            maybe_error: Some(format!(
                                "{} output budget exceeded",
                                if stderr { "stderr" } else { "stdout" }
                            )),
                        })
                        .expect("supervisor remains until readers finish");
                    // Signal immediately, then drain without retaining any more bytes.
                    // The supervisor kills/reaps before joining this worker.
                    if let Err(error) = io::copy(&mut reader, &mut io::sink()) {
                        eprintln!("test child capped drain failed: {error}");
                    }
                    return;
                }
                Err(error) if error.kind() == io::ErrorKind::Interrupted => continue,
                Err(error) => {
                    sender
                        .send(Event::Read {
                            stderr,
                            bytes: captured,
                            maybe_error: Some(format!("pipe read failed: {error}")),
                        })
                        .expect("supervisor remains until readers finish");
                    return;
                }
            }
        }
    })
}

#[derive(Default)]
struct Observation {
    stdout: Vec<u8>,
    stderr: Vec<u8>,
    completed: usize,
    maybe_failure: Option<String>,
}

impl Observation {
    fn receive(&mut self, event: Event) {
        self.completed += 1;
        match event {
            Event::Input(Ok(())) => {}
            Event::Input(Err(error)) => {
                self.maybe_failure
                    .get_or_insert(format!("stdin write failed: {error}"));
            }
            Event::Read {
                stderr,
                bytes,
                maybe_error,
            } => {
                if stderr {
                    self.stderr = bytes;
                } else {
                    self.stdout = bytes;
                }
                if let Some(error) = maybe_error {
                    self.maybe_failure.get_or_insert(error);
                }
            }
        }
    }
}

fn supervise(
    child: &mut ChildGuard,
    receiver: &Receiver<Event>,
    limits: Limits,
    started: Instant,
    observation: &mut Observation,
) -> io::Result<ExitStatus> {
    loop {
        while let Ok(event) = receiver.try_recv() {
            observation.receive(event);
        }
        if observation.maybe_failure.is_some() {
            return child.stop();
        }
        let maybe_status = child.child.try_wait()?;
        if let Some(status) = maybe_status {
            if !status.success() {
                observation.maybe_failure = Some(format!("child exited with {status}"));
                return child.stop();
            }
            if observation.completed == 3 {
                return Ok(status);
            }
        }
        if started.elapsed() >= limits.deadline {
            observation.maybe_failure = Some("total child deadline exceeded".to_owned());
            return child.stop();
        }
        match receiver.recv_timeout(Duration::from_millis(5)) {
            Ok(event) => observation.receive(event),
            Err(mpsc::RecvTimeoutError::Timeout) => {}
            Err(mpsc::RecvTimeoutError::Disconnected) if observation.completed == 3 => {
                thread::sleep(Duration::from_millis(5))
            }
            Err(error) => {
                observation.maybe_failure = Some(format!("child worker channel failed: {error}"));
                return child.stop();
            }
        }
    }
}

/// Bound the outer process from before stdin writing through reaping and draining.
pub fn run(command: &mut Command, input: Vec<u8>, limits: Limits) -> Result<Captured, Failure> {
    let failure = |reason: String| Failure {
        reason,
        stdout: Vec::new(),
        stderr: Vec::new(),
        maybe_pid: None,
        reaped: false,
    };
    if input.len() > limits.input_bytes {
        return Err(failure(
            "stdin input budget exceeded before spawn".to_owned(),
        ));
    }
    let started = Instant::now();
    configure_group(command).map_err(|error| failure(error.to_string()))?;
    let mut child = ChildGuard {
        child: command
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|error| failure(format!("child spawn failed: {error}")))?,
        group_cleaned: false,
        reaped: false,
    };
    let pid = child.child.id();
    let mut stdin = child.child.stdin.take().expect("piped stdin");
    let stdout = child.child.stdout.take().expect("piped stdout");
    let stderr = child.child.stderr.take().expect("piped stderr");
    let (sender, receiver) = mpsc::channel();
    let output_worker = drain(stdout, limits.stdout_bytes, false, sender.clone());
    let error_worker = drain(stderr, limits.stderr_bytes, true, sender.clone());
    let input_worker = thread::spawn(move || {
        let result = stdin.write_all(&input);
        drop(stdin);
        sender
            .send(Event::Input(result))
            .expect("supervisor remains until writer finishes");
    });
    let mut observation = Observation::default();
    let status = supervise(&mut child, &receiver, limits, started, &mut observation);
    // Reap before joins on every path, including polling/worker errors.
    let stopped = child.stop();
    let drain_started = Instant::now();
    for worker in [input_worker, output_worker, error_worker] {
        while !worker.is_finished() && drain_started.elapsed() < Duration::from_secs(2) {
            thread::sleep(Duration::from_millis(2));
        }
        if !worker.is_finished() {
            return Err(Failure {
                reason: "incomplete cleanup: pipe drain deadline exceeded".to_owned(),
                stdout: observation.stdout,
                stderr: observation.stderr,
                maybe_pid: Some(pid),
                reaped: child.reaped,
            });
        }
        if worker.join().is_err() {
            observation
                .maybe_failure
                .get_or_insert("child worker panicked".to_owned());
        }
    }
    while let Ok(event) = receiver.try_recv() {
        observation.receive(event);
    }
    let reaped = child.reaped;
    let status = status
        .and(stopped)
        .map_err(|error| format!("child reap failed: {error}"));
    let maybe_error = observation
        .maybe_failure
        .or_else(|| status.as_ref().err().cloned());
    if let Some(reason) = maybe_error {
        return Err(Failure {
            reason,
            stdout: observation.stdout,
            stderr: observation.stderr,
            maybe_pid: Some(pid),
            reaped,
        });
    }
    Ok(Captured {
        stdout: observation.stdout,
        stderr: observation.stderr,
        pid,
        status: status.expect("successful supervision status"),
        reaped,
    })
}

#[cfg(all(test, unix))]
mod tests {
    use super::*;

    fn control(mode: &str) -> Command {
        let mut command = Command::new(std::env::current_exe().expect("test executable"));
        command.args([
            "--exact",
            &format!(
                "{}::phase162_process_child",
                module_path!().split_once("::").expect("crate prefix").1
            ),
            "--ignored",
            "--nocapture",
            "--test-threads=1",
        ]);
        command.env("OPEN_BITCOIN_PHASE162_CHILD_CONTROL", mode);
        command
    }

    fn limits() -> Limits {
        Limits {
            input_bytes: 1024 * 1024,
            stdout_bytes: 4096,
            stderr_bytes: 4096,
            deadline: Duration::from_millis(750),
        }
    }

    fn refused(mode: &str, input: Vec<u8>, reason: &str) {
        // Arrange
        let started = Instant::now();
        // Act
        let result = run(&mut control(mode), input, limits());
        let error = match result {
            Ok(_) => panic!("control child unexpectedly succeeded"),
            Err(error) => error,
        };
        // Assert
        assert!(error.reason.contains(reason), "{error}");
        assert!(started.elapsed() < Duration::from_secs(5));
        assert!(error.reaped && error.maybe_pid.is_some());
        assert!(error.stdout.len() <= limits().stdout_bytes);
        assert!(error.stderr.len() <= limits().stderr_bytes);
    }

    #[test]
    fn phase162_process_bounds_unread_stdin() {
        refused("ignore-input", vec![0; 1024 * 1024], "deadline");
    }

    #[test]
    fn phase162_process_bounds_sleeping_child() {
        refused("sleep", Vec::new(), "deadline");
    }

    #[test]
    fn phase162_process_kills_inherited_pipe_descendant() {
        refused("descendant", Vec::new(), "deadline");
    }

    #[test]
    fn phase162_process_bounds_stdout() {
        refused("stdout", Vec::new(), "stdout output budget");
    }

    #[test]
    fn phase162_process_bounds_stderr() {
        refused("stderr", Vec::new(), "stderr output budget");
    }

    #[test]
    fn phase162_process_propagates_nonzero_exit() {
        refused("nonzero", Vec::new(), "exited");
    }

    #[test]
    fn phase162_process_reaps_after_early_stdin_close() {
        refused("close-input", vec![0; 1024 * 1024], "stdin write failed");
    }

    #[test]
    fn phase162_process_rejects_input_before_spawn() {
        // Arrange / Act
        let result = run(
            &mut control("sleep"),
            vec![0; limits().input_bytes + 1],
            limits(),
        );
        // Assert
        let Err(error) = result else {
            panic!("oversized input accepted");
        };
        assert!(error.reason.contains("before spawn"));
        assert!(error.maybe_pid.is_none());
    }

    #[test]
    fn phase162_process_captures_success_and_reaps() {
        // Arrange / Act
        let result = run(
            &mut control("echo"),
            b"bounded-input-marker".to_vec(),
            limits(),
        )
        .expect("echo success");
        // Assert
        assert!(result.status.success() && result.reaped && result.pid > 0);
        assert!(
            result
                .stdout
                .windows(20)
                .any(|bytes| bytes == b"bounded-input-marker")
        );
        assert!(result.stderr.is_empty());
    }

    #[test]
    #[ignore = "invoked only as a bounded subprocess control"]
    fn phase162_process_child() {
        let mode = std::env::var("OPEN_BITCOIN_PHASE162_CHILD_CONTROL").expect("control mode");
        match mode.as_str() {
            "sleep" | "ignore-input" => loop {
                thread::sleep(Duration::from_secs(1));
            },
            "stdout" => loop {
                io::stdout()
                    .write_all(&[b'x'; 8192])
                    .expect("control stdout");
            },
            "stderr" => loop {
                io::stderr()
                    .write_all(&[b'x'; 8192])
                    .expect("control stderr");
            },
            "nonzero" => {
                eprintln!("controlled nonzero exit");
                std::process::exit(7);
            }
            "close-input" => std::process::exit(0),
            "descendant" => {
                let mut descendant = control("sleep");
                let _child = descendant
                    .stdin(Stdio::null())
                    .stdout(Stdio::inherit())
                    .stderr(Stdio::inherit())
                    .spawn()
                    .expect("inherited-pipe descendant");
                std::process::exit(0);
            }
            "echo" => {
                io::copy(&mut io::stdin(), &mut io::stdout()).expect("control echo");
            }
            _ => panic!("unknown child control"),
        }
    }
}
