// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

//! Service lifecycle management contracts for Open Bitcoin node.
//!
//! This module defines the `ServiceManager` trait, typed request/response structs, error
//! variants, and a `platform_service_manager()` factory for selecting the active platform
//! adapter at compile time.

use std::path::PathBuf;

pub mod fake;
pub mod launchd;
pub mod systemd;

#[cfg(test)]
mod tests;

/// The current lifecycle state of the Open Bitcoin service.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServiceLifecycleState {
    /// No service definition found on this machine.
    Unmanaged,
    /// Service file is installed but not necessarily enabled.
    Installed,
    /// Service is installed and registered to start at login/boot.
    Enabled,
    /// Service is currently running.
    Running,
    /// Service exited with a non-zero status (failed).
    Failed,
    /// Service is installed and enabled but not currently running.
    Stopped,
}

/// A snapshot of the current service state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServiceStateSnapshot {
    /// Current lifecycle state.
    pub state: ServiceLifecycleState,
    /// Path to the installed service file, if known.
    pub maybe_service_file_path: Option<PathBuf>,
    /// Diagnostic output from the service manager, if available.
    pub maybe_manager_diagnostics: Option<String>,
    /// Path to the service log file, if configured.
    pub maybe_log_path: Option<PathBuf>,
}

/// Request to install the Open Bitcoin node as a managed service.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServiceInstallRequest {
    /// Path to the Open Bitcoin node binary.
    pub binary_path: PathBuf,
    /// Data directory for the node.
    pub data_dir: PathBuf,
    /// Optional path to the Open Bitcoin config file.
    pub maybe_config_path: Option<PathBuf>,
    /// Optional path for service log output.
    pub maybe_log_path: Option<PathBuf>,
    /// If `false`, perform a dry run and return a preview without writing anything.
    /// If `true`, write the service file and register with the platform manager.
    pub apply: bool,
}

/// Request to uninstall the Open Bitcoin node service.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServiceUninstallRequest {
    /// If `false`, perform a dry run. If `true`, remove the service file.
    pub apply: bool,
}

/// Request to enable the service to start at login/boot.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServiceEnableRequest;

/// Request to disable automatic service startup.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServiceDisableRequest;

/// Outcome of a service lifecycle command (install, uninstall, enable, disable).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServiceCommandOutcome {
    /// `true` if this was a dry run (no filesystem or subprocess side effects).
    pub dry_run: bool,
    /// Human-readable description of what was done or would be done.
    pub description: String,
    /// Target service file path, if applicable.
    pub maybe_file_path: Option<PathBuf>,
    /// Generated file content shown in dry-run preview.
    pub maybe_file_content: Option<String>,
    /// Platform commands that were or would be run.
    pub commands_that_would_run: Vec<String>,
}

/// Typed errors for service lifecycle operations.
#[derive(Debug, Clone, thiserror::Error)]
pub enum ServiceError {
    #[error("unsupported platform: {reason}")]
    UnsupportedPlatform { reason: String },

    #[error("service already installed at {path}; use --force to reinstall")]
    AlreadyInstalled { path: PathBuf },

    #[error("service not installed — run `open-bitcoin service install --dry-run` to preview")]
    NotInstalled,

    #[error("write failed at {path}: {cause}")]
    WriteFailure { path: PathBuf, cause: String },

    #[error("service manager command failed (exit {exit_code}): {stderr}")]
    ManagerCommandFailed { exit_code: i32, stderr: String },
}

/// Service lifecycle management: install, uninstall, enable, disable, and status.
///
/// Implementations handle platform-specific service manager invocations. Tests
/// use `FakeServiceManager` to avoid real filesystem or subprocess side effects.
pub trait ServiceManager {
    fn install(
        &self,
        request: &ServiceInstallRequest,
    ) -> Result<ServiceCommandOutcome, ServiceError>;

    fn uninstall(
        &self,
        request: &ServiceUninstallRequest,
    ) -> Result<ServiceCommandOutcome, ServiceError>;

    fn enable(&self, request: &ServiceEnableRequest)
    -> Result<ServiceCommandOutcome, ServiceError>;

    fn disable(
        &self,
        request: &ServiceDisableRequest,
    ) -> Result<ServiceCommandOutcome, ServiceError>;

    fn status(&self) -> Result<ServiceStateSnapshot, ServiceError>;
}

/// A platform adapter that always returns `ServiceError::UnsupportedPlatform`.
///
/// This type is never cfg-gated so it compiles on all platforms; the cfg gates
/// are only on the arms of `platform_service_manager()`. The `dead_code` allow
/// is needed because on macOS and Linux the fallback arm is never reached.
#[allow(dead_code)]
struct UnsupportedPlatformAdapter;

impl ServiceManager for UnsupportedPlatformAdapter {
    fn install(
        &self,
        _request: &ServiceInstallRequest,
    ) -> Result<ServiceCommandOutcome, ServiceError> {
        Err(ServiceError::UnsupportedPlatform {
            reason: "this platform does not support automated service installation".to_string(),
        })
    }

    fn uninstall(
        &self,
        _request: &ServiceUninstallRequest,
    ) -> Result<ServiceCommandOutcome, ServiceError> {
        Err(ServiceError::UnsupportedPlatform {
            reason: "this platform does not support automated service uninstallation".to_string(),
        })
    }

    fn enable(
        &self,
        _request: &ServiceEnableRequest,
    ) -> Result<ServiceCommandOutcome, ServiceError> {
        Err(ServiceError::UnsupportedPlatform {
            reason: "this platform does not support service enable".to_string(),
        })
    }

    fn disable(
        &self,
        _request: &ServiceDisableRequest,
    ) -> Result<ServiceCommandOutcome, ServiceError> {
        Err(ServiceError::UnsupportedPlatform {
            reason: "this platform does not support service disable".to_string(),
        })
    }

    fn status(&self) -> Result<ServiceStateSnapshot, ServiceError> {
        Err(ServiceError::UnsupportedPlatform {
            reason: "this platform does not support service status inspection".to_string(),
        })
    }
}

/// Construct the platform-appropriate `ServiceManager` for the current OS.
///
/// On macOS, returns a `LaunchdAdapter`. On Linux, returns a `SystemdAdapter`.
/// On other platforms, returns an adapter that always returns `UnsupportedPlatform`.
pub fn platform_service_manager(home_dir: PathBuf) -> Box<dyn ServiceManager> {
    #[cfg(target_os = "macos")]
    {
        Box::new(launchd::LaunchdAdapter::new(home_dir))
    }

    #[cfg(target_os = "linux")]
    {
        Box::new(systemd::SystemdAdapter::new(home_dir))
    }

    #[cfg(not(any(target_os = "macos", target_os = "linux")))]
    {
        let _ = home_dir;
        Box::new(UnsupportedPlatformAdapter)
    }
}

/// Render a `ServiceCommandOutcome` as human-readable output.
fn render_service_outcome(outcome: &ServiceCommandOutcome) -> String {
    let mut lines = Vec::new();
    if outcome.dry_run {
        lines.push("Dry run (pass --apply to make changes):".to_string());
    }
    lines.push(outcome.description.clone());
    if let Some(path) = &outcome.maybe_file_path {
        lines.push(format!("  Would write: {}", path.display()));
    }
    if !outcome.commands_that_would_run.is_empty() {
        lines.push("  Commands:".to_string());
        for cmd in &outcome.commands_that_would_run {
            lines.push(format!("    {cmd}"));
        }
    }
    if outcome.dry_run {
        lines.push("Scope: user-level (no sudo required).".to_string());
    }
    if let Some(content) = &outcome.maybe_file_content {
        lines.push("  Generated content:".to_string());
        lines.push("  ---".to_string());
        for line in content.lines() {
            lines.push(format!("  {line}"));
        }
        lines.push("  ---".to_string());
    }
    lines.join("\n")
}

/// Render a `ServiceStateSnapshot` as human-readable output.
fn render_service_state_snapshot(snapshot: &ServiceStateSnapshot) -> String {
    let state = match snapshot.state {
        ServiceLifecycleState::Unmanaged => {
            "unmanaged — run `open-bitcoin service install` to see what would be created"
        }
        ServiceLifecycleState::Installed => "installed (not enabled)",
        ServiceLifecycleState::Enabled => "enabled (not running)",
        ServiceLifecycleState::Running => "running",
        ServiceLifecycleState::Failed => "failed",
        ServiceLifecycleState::Stopped => "stopped",
    };
    let mut lines = vec![format!("service: {state}")];
    if let Some(path) = &snapshot.maybe_service_file_path {
        lines.push(format!("  file: {}", path.display()));
    }
    if let Some(log_path) = &snapshot.maybe_log_path {
        lines.push(format!("  logs: {}", log_path.display()));
    }
    if let Some(diag) = &snapshot.maybe_manager_diagnostics {
        lines.push(format!("  diagnostics: {diag}"));
    }
    lines.join("\n")
}

/// Execute a service subcommand using the injected manager.
///
/// Returns an `OperatorCommandOutcome` with dry-run preview output (when `args.apply` is
/// false) or the result of the applied action. Enable and disable always execute without
/// requiring `--apply` per D-12.
pub fn execute_service_command(
    args: &super::ServiceArgs,
    binary_path: PathBuf,
    data_dir: PathBuf,
    maybe_config_path: Option<PathBuf>,
    maybe_log_path: Option<PathBuf>,
    manager: &dyn ServiceManager,
) -> super::runtime::OperatorCommandOutcome {
    use super::ServiceCommand;
    use super::runtime::OperatorCommandOutcome;

    match &args.command {
        ServiceCommand::Install => {
            let request = ServiceInstallRequest {
                binary_path,
                data_dir,
                maybe_config_path,
                maybe_log_path,
                apply: args.apply,
            };
            match manager.install(&request) {
                Ok(outcome) => OperatorCommandOutcome::success(format!(
                    "{}\n",
                    render_service_outcome(&outcome)
                )),
                Err(error) => OperatorCommandOutcome::failure(error.to_string()),
            }
        }
        ServiceCommand::Uninstall => {
            let request = ServiceUninstallRequest { apply: args.apply };
            match manager.uninstall(&request) {
                Ok(outcome) => OperatorCommandOutcome::success(format!(
                    "{}\n",
                    render_service_outcome(&outcome)
                )),
                Err(error) => OperatorCommandOutcome::failure(error.to_string()),
            }
        }
        ServiceCommand::Enable => {
            let request = ServiceEnableRequest;
            match manager.enable(&request) {
                Ok(outcome) => OperatorCommandOutcome::success(format!(
                    "{}\n",
                    render_service_outcome(&outcome)
                )),
                Err(error) => OperatorCommandOutcome::failure(error.to_string()),
            }
        }
        ServiceCommand::Disable => {
            let request = ServiceDisableRequest;
            match manager.disable(&request) {
                Ok(outcome) => OperatorCommandOutcome::success(format!(
                    "{}\n",
                    render_service_outcome(&outcome)
                )),
                Err(error) => OperatorCommandOutcome::failure(error.to_string()),
            }
        }
        ServiceCommand::Status => match manager.status() {
            Ok(snapshot) => OperatorCommandOutcome::success(format!(
                "{}\n",
                render_service_state_snapshot(&snapshot)
            )),
            Err(error) => OperatorCommandOutcome::failure(error.to_string()),
        },
    }
}
