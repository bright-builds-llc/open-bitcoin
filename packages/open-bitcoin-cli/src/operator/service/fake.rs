// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

//! Fake service manager for use in tests.
//!
//! `FakeServiceManager` records all calls and returns deterministic outcomes configured at
//! construction time. No subprocess invocations or filesystem writes occur.

use std::cell::RefCell;

use super::{
    ServiceCommandOutcome, ServiceDisableRequest, ServiceEnableRequest, ServiceError,
    ServiceInstallRequest, ServiceLifecycleState, ServiceManager, ServiceRestartRequest,
    ServiceStartRequest, ServiceStateSnapshot, ServiceStopRequest, ServiceUninstallRequest,
};

/// A recorded call to `FakeServiceManager`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FakeServiceCall {
    Install { apply: bool },
    Uninstall { apply: bool },
    Start,
    Stop,
    Restart,
    Enable,
    Disable,
    Status,
}

/// A `ServiceManager` that records calls without performing any real I/O.
///
/// Configure `status_to_return` and error fields at construction time to drive
/// specific test scenarios.
pub struct FakeServiceManager {
    /// All calls made to this manager in order.
    pub recorded_calls: RefCell<Vec<FakeServiceCall>>,
    /// The state snapshot returned from `status()`.
    pub status_to_return: ServiceStateSnapshot,
    /// If `Some`, `install()` returns this error instead of success.
    pub install_error: Option<ServiceError>,
    /// If `Some`, `install()` returns this success outcome instead of the default fake outcome.
    pub install_outcome: Option<ServiceCommandOutcome>,
    /// If `Some`, `uninstall()` returns this error instead of success.
    pub uninstall_error: Option<ServiceError>,
    /// Commands surfaced in the outcome from `start()`.
    pub start_commands: Vec<String>,
    /// Commands surfaced in the outcome from `stop()`.
    pub stop_commands: Vec<String>,
    /// Commands surfaced in the outcome from `restart()`.
    pub restart_commands: Vec<String>,
    /// Commands surfaced in the outcome from `enable()`.
    pub enable_commands: Vec<String>,
}

impl FakeServiceManager {
    /// Create a new `FakeServiceManager` that returns the given status snapshot from `status()`.
    pub fn new(status_to_return: ServiceStateSnapshot) -> Self {
        Self {
            recorded_calls: RefCell::new(Vec::new()),
            status_to_return,
            install_error: None,
            install_outcome: None,
            uninstall_error: None,
            start_commands: Vec::new(),
            stop_commands: Vec::new(),
            restart_commands: Vec::new(),
            enable_commands: Vec::new(),
        }
    }

    /// Create an unmanaged fake manager (convenient default for tests).
    pub fn unmanaged() -> Self {
        Self::new(ServiceStateSnapshot {
            state: ServiceLifecycleState::Unmanaged,
            maybe_enabled: Some(false),
            maybe_service_file_path: None,
            maybe_manager_diagnostics: None,
            maybe_log_path: None,
            maybe_log_path_unavailable_reason: Some("service not installed".to_string()),
        })
    }
}

impl ServiceManager for FakeServiceManager {
    fn install(
        &self,
        request: &ServiceInstallRequest,
    ) -> Result<ServiceCommandOutcome, ServiceError> {
        self.recorded_calls
            .borrow_mut()
            .push(FakeServiceCall::Install {
                apply: request.apply,
            });

        if let Some(ref error) = self.install_error {
            return Err(error.clone());
        }
        if let Some(ref outcome) = self.install_outcome {
            return Ok(outcome.clone());
        }

        Ok(ServiceCommandOutcome {
            dry_run: !request.apply,
            description: "fake install".to_string(),
            maybe_file_path: None,
            maybe_file_content: None,
            commands_that_would_run: vec![],
        })
    }

    fn uninstall(
        &self,
        request: &ServiceUninstallRequest,
    ) -> Result<ServiceCommandOutcome, ServiceError> {
        self.recorded_calls
            .borrow_mut()
            .push(FakeServiceCall::Uninstall {
                apply: request.apply,
            });

        if let Some(ref error) = self.uninstall_error {
            return Err(error.clone());
        }

        Ok(ServiceCommandOutcome {
            dry_run: !request.apply,
            description: "fake uninstall".to_string(),
            maybe_file_path: None,
            maybe_file_content: None,
            commands_that_would_run: vec![],
        })
    }

    fn start(&self, _request: &ServiceStartRequest) -> Result<ServiceCommandOutcome, ServiceError> {
        self.recorded_calls
            .borrow_mut()
            .push(FakeServiceCall::Start);

        Ok(ServiceCommandOutcome {
            dry_run: false,
            description: "fake start".to_string(),
            maybe_file_path: None,
            maybe_file_content: None,
            commands_that_would_run: self.start_commands.clone(),
        })
    }

    fn stop(&self, _request: &ServiceStopRequest) -> Result<ServiceCommandOutcome, ServiceError> {
        self.recorded_calls.borrow_mut().push(FakeServiceCall::Stop);

        Ok(ServiceCommandOutcome {
            dry_run: false,
            description: "fake stop".to_string(),
            maybe_file_path: None,
            maybe_file_content: None,
            commands_that_would_run: self.stop_commands.clone(),
        })
    }

    fn restart(
        &self,
        _request: &ServiceRestartRequest,
    ) -> Result<ServiceCommandOutcome, ServiceError> {
        self.recorded_calls
            .borrow_mut()
            .push(FakeServiceCall::Restart);

        Ok(ServiceCommandOutcome {
            dry_run: false,
            description: "fake restart".to_string(),
            maybe_file_path: None,
            maybe_file_content: None,
            commands_that_would_run: self.restart_commands.clone(),
        })
    }

    fn enable(
        &self,
        _request: &ServiceEnableRequest,
    ) -> Result<ServiceCommandOutcome, ServiceError> {
        self.recorded_calls
            .borrow_mut()
            .push(FakeServiceCall::Enable);

        Ok(ServiceCommandOutcome {
            dry_run: false,
            description: "fake enable".to_string(),
            maybe_file_path: None,
            maybe_file_content: None,
            commands_that_would_run: self.enable_commands.clone(),
        })
    }

    fn disable(
        &self,
        _request: &ServiceDisableRequest,
    ) -> Result<ServiceCommandOutcome, ServiceError> {
        self.recorded_calls
            .borrow_mut()
            .push(FakeServiceCall::Disable);

        Ok(ServiceCommandOutcome {
            dry_run: false,
            description: "fake disable".to_string(),
            maybe_file_path: None,
            maybe_file_content: None,
            commands_that_would_run: vec![],
        })
    }

    fn status(&self) -> Result<ServiceStateSnapshot, ServiceError> {
        self.recorded_calls
            .borrow_mut()
            .push(FakeServiceCall::Status);

        Ok(self.status_to_return.clone())
    }
}
