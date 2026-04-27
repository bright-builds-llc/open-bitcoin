// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

//! Fake service manager for test isolation — records calls, returns deterministic outcomes.

use std::cell::RefCell;

use super::{
    ServiceCommandOutcome, ServiceDisableRequest, ServiceEnableRequest, ServiceError,
    ServiceInstallRequest, ServiceManager, ServiceStateSnapshot, ServiceUninstallRequest,
};

/// Records a single call made to `FakeServiceManager`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FakeServiceCall {
    /// `install` was called with the given `apply` flag.
    Install { apply: bool },
    /// `uninstall` was called with the given `apply` flag.
    Uninstall { apply: bool },
    /// `enable` was called.
    Enable,
    /// `disable` was called.
    Disable,
    /// `status` was called.
    Status,
}

/// A deterministic, in-process `ServiceManager` that records calls without performing I/O.
pub struct FakeServiceManager {
    /// All calls made to this manager in order.
    pub recorded_calls: RefCell<Vec<FakeServiceCall>>,
    /// State snapshot returned by `status()`.
    pub status_to_return: ServiceStateSnapshot,
    /// If `Some`, `install()` returns this error instead of success.
    pub install_error: Option<ServiceError>,
    /// If `Some`, `uninstall()` returns this error instead of success.
    pub uninstall_error: Option<ServiceError>,
    /// Commands surfaced in the outcome from `enable()`.
    pub enable_commands: Vec<String>,
}

impl FakeServiceManager {
    /// Create a `FakeServiceManager` that returns `status_to_return` from `status()`.
    pub fn new(status_to_return: ServiceStateSnapshot) -> Self {
        Self {
            recorded_calls: RefCell::new(Vec::new()),
            status_to_return,
            install_error: None,
            uninstall_error: None,
            enable_commands: Vec::new(),
        }
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

        Ok(ServiceStateSnapshot {
            state: self.status_to_return.state,
            maybe_service_file_path: self.status_to_return.maybe_service_file_path.clone(),
            maybe_manager_diagnostics: self.status_to_return.maybe_manager_diagnostics.clone(),
            maybe_log_path: self.status_to_return.maybe_log_path.clone(),
        })
    }
}
