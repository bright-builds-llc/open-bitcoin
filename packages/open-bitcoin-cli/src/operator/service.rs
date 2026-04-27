// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

//! Service lifecycle trait, error types, and platform factory for Open Bitcoin.

use std::path::PathBuf;

pub mod fake;
pub mod launchd;
pub mod systemd;
#[cfg(test)]
mod tests;

/// Lifecycle state of the Open Bitcoin node service.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServiceLifecycleState {
    /// No service definition found; not installed.
    Unmanaged,
    /// Service file is present but not yet registered with the service manager.
    Installed,
    /// Service is registered to start at login/boot.
    Enabled,
    /// Service is currently running.
    Running,
    /// Service is installed and registered but not currently running.
    Stopped,
    /// Service exited with a non-zero status.
    Failed,
}

/// Current state snapshot returned by `ServiceManager::status`.
#[derive(Debug, Clone)]
pub struct ServiceStateSnapshot {
    /// Classified lifecycle state.
    pub state: ServiceLifecycleState,
    /// Path to the service definition file, if present.
    pub maybe_service_file_path: Option<PathBuf>,
    /// Raw diagnostic output from the service manager, if available.
    pub maybe_manager_diagnostics: Option<String>,
    /// Log file path associated with this service, if configured.
    pub maybe_log_path: Option<PathBuf>,
}

/// Request to install the Open Bitcoin node as a managed service.
#[derive(Debug, Clone)]
pub struct ServiceInstallRequest {
    /// Path to the open-bitcoin binary.
    pub binary_path: PathBuf,
    /// Node data directory.
    pub data_dir: PathBuf,
    /// Optional config file path.
    pub maybe_config_path: Option<PathBuf>,
    /// Optional log file path for stdout/stderr redirection.
    pub maybe_log_path: Option<PathBuf>,
    /// Whether to actually write files and run commands (false = dry-run).
    pub apply: bool,
}

/// Request to uninstall the Open Bitcoin node service.
#[derive(Debug, Clone)]
pub struct ServiceUninstallRequest {
    /// Whether to actually remove files and run commands (false = dry-run).
    pub apply: bool,
}

/// Request to enable the Open Bitcoin node service (start at login/boot).
#[derive(Debug, Clone)]
pub struct ServiceEnableRequest;

/// Request to disable the Open Bitcoin node service.
#[derive(Debug, Clone)]
pub struct ServiceDisableRequest;

/// Outcome of a service lifecycle command.
#[derive(Debug, Clone)]
pub struct ServiceCommandOutcome {
    /// Whether this outcome reflects a dry-run (no state was mutated).
    pub dry_run: bool,
    /// Human-readable description of what was or would be done.
    pub description: String,
    /// Service definition file path involved in this command, if any.
    pub maybe_file_path: Option<PathBuf>,
    /// Generated file content shown in dry-run output.
    pub maybe_file_content: Option<String>,
    /// Shell commands that ran or would run on `--apply`.
    pub commands_that_would_run: Vec<String>,
}

/// Typed error for service lifecycle operations.
#[derive(Debug, Clone, thiserror::Error)]
pub enum ServiceError {
    /// Requested operation is not supported on this platform.
    #[error("unsupported platform: {reason}")]
    UnsupportedPlatform { reason: String },
    /// Service is already installed at the given path.
    #[error("service already installed at {path}; use --force to reinstall")]
    AlreadyInstalled { path: PathBuf },
    /// Service is not installed; install it first.
    #[error("service not installed — run `open-bitcoin service install --dry-run` to preview")]
    NotInstalled,
    /// File write failed.
    #[error("write failed at {path}: {cause}")]
    WriteFailure { path: PathBuf, cause: String },
    /// Service manager command (launchctl/systemctl) returned a non-zero exit code.
    #[error("service manager command failed (exit {exit_code}): {stderr}")]
    ManagerCommandFailed { exit_code: i32, stderr: String },
}

/// Platform-agnostic service lifecycle interface.
pub trait ServiceManager {
    /// Install the node as a managed service, optionally writing files and running commands.
    fn install(
        &self,
        request: &ServiceInstallRequest,
    ) -> Result<ServiceCommandOutcome, ServiceError>;

    /// Uninstall the managed service, optionally removing files and running commands.
    fn uninstall(
        &self,
        request: &ServiceUninstallRequest,
    ) -> Result<ServiceCommandOutcome, ServiceError>;

    /// Enable the service to start at login/boot.
    fn enable(&self, request: &ServiceEnableRequest)
    -> Result<ServiceCommandOutcome, ServiceError>;

    /// Disable the service from starting at login/boot.
    fn disable(
        &self,
        request: &ServiceDisableRequest,
    ) -> Result<ServiceCommandOutcome, ServiceError>;

    /// Return the current state of the managed service.
    fn status(&self) -> Result<ServiceStateSnapshot, ServiceError>;
}

/// Returns an `Err(ServiceError::UnsupportedPlatform)` for all operations.
///
/// Used as the fallback adapter when neither macOS nor Linux is the compile target.
#[cfg(not(any(target_os = "macos", target_os = "linux")))]
struct UnsupportedPlatformAdapter;

#[cfg(not(any(target_os = "macos", target_os = "linux")))]
impl ServiceManager for UnsupportedPlatformAdapter {
    fn install(
        &self,
        _request: &ServiceInstallRequest,
    ) -> Result<ServiceCommandOutcome, ServiceError> {
        Err(ServiceError::UnsupportedPlatform {
            reason: "service management is only supported on macOS and Linux".to_string(),
        })
    }

    fn uninstall(
        &self,
        _request: &ServiceUninstallRequest,
    ) -> Result<ServiceCommandOutcome, ServiceError> {
        Err(ServiceError::UnsupportedPlatform {
            reason: "service management is only supported on macOS and Linux".to_string(),
        })
    }

    fn enable(
        &self,
        _request: &ServiceEnableRequest,
    ) -> Result<ServiceCommandOutcome, ServiceError> {
        Err(ServiceError::UnsupportedPlatform {
            reason: "service management is only supported on macOS and Linux".to_string(),
        })
    }

    fn disable(
        &self,
        _request: &ServiceDisableRequest,
    ) -> Result<ServiceCommandOutcome, ServiceError> {
        Err(ServiceError::UnsupportedPlatform {
            reason: "service management is only supported on macOS and Linux".to_string(),
        })
    }

    fn status(&self) -> Result<ServiceStateSnapshot, ServiceError> {
        Err(ServiceError::UnsupportedPlatform {
            reason: "service management is only supported on macOS and Linux".to_string(),
        })
    }
}

/// Render a `ServiceCommandOutcome` as human-readable text.
fn render_service_outcome(outcome: &ServiceCommandOutcome) -> String {
    let mut lines = Vec::new();
    if outcome.dry_run {
        lines.push("Dry run (pass --apply to make changes):".to_string());
    }
    lines.push(outcome.description.clone());
    if let Some(path) = &outcome.maybe_file_path {
        lines.push(format!("  File: {}", path.display()));
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

/// Render a `ServiceStateSnapshot` as human-readable text.
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
/// Routes each `ServiceCommand` variant to the corresponding `ServiceManager` method and
/// renders the outcome as an `OperatorCommandOutcome`. Returns success on `Ok`, failure on `Err`.
pub fn execute_service_command(
    args: &super::ServiceArgs,
    binary_path: PathBuf,
    data_dir: PathBuf,
    maybe_config_path: Option<PathBuf>,
    maybe_log_path: Option<PathBuf>,
    manager: &dyn ServiceManager,
) -> crate::operator::runtime::OperatorCommandOutcome {
    use super::ServiceCommand;
    use crate::operator::runtime::OperatorCommandOutcome;

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

/// Construct the platform-appropriate `ServiceManager` for the current OS.
///
/// On macOS, returns a `LaunchdAdapter`. On Linux, returns a `SystemdAdapter`.
/// On other platforms, returns an adapter that reports `UnsupportedPlatform` for all operations.
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
