// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

//! Dashboard action state machine and service-command adapter.

use std::path::PathBuf;

use super::super::{
    ServiceArgs, ServiceCommand,
    runtime::OperatorCommandOutcome,
    service::{ServiceManager, execute_service_command},
};

/// Keyboard actions supported by the dashboard.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DashboardAction {
    Refresh,
    Exit,
    ShowStatus,
    InstallService,
    UninstallService,
    EnableService,
    DisableService,
    Help,
    Confirm,
    Cancel,
    None,
}

impl DashboardAction {
    pub const fn requires_confirmation(self) -> bool {
        matches!(
            self,
            Self::InstallService
                | Self::UninstallService
                | Self::EnableService
                | Self::DisableService
        )
    }
}

/// Operator confirmation decision for a dashboard action.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActionDecision {
    Pending,
    Confirmed,
    Cancelled,
}

/// Current action and confirmation state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DashboardActionState {
    pub action: DashboardAction,
    pub decision: ActionDecision,
}

impl DashboardActionState {
    pub const fn pending(action: DashboardAction) -> Self {
        Self {
            action,
            decision: ActionDecision::Pending,
        }
    }

    pub const fn confirmed(action: DashboardAction) -> Self {
        Self {
            action,
            decision: ActionDecision::Confirmed,
        }
    }

    pub const fn cancelled(action: DashboardAction) -> Self {
        Self {
            action,
            decision: ActionDecision::Cancelled,
        }
    }
}

/// Service execution context supplied by the operator runtime.
pub struct DashboardServiceContext<'a> {
    pub binary_path: PathBuf,
    pub data_dir: PathBuf,
    pub maybe_config_path: Option<PathBuf>,
    pub maybe_log_path: Option<PathBuf>,
    pub manager: &'a dyn ServiceManager,
}

/// Execute an action only when its confirmation state permits side effects.
pub fn confirm_and_execute(
    action_state: &DashboardActionState,
    context: &DashboardServiceContext<'_>,
) -> OperatorCommandOutcome {
    match action_state.decision {
        ActionDecision::Cancelled => {
            return OperatorCommandOutcome::success("dashboard action cancelled\n");
        }
        ActionDecision::Pending if action_state.action.requires_confirmation() => {
            return OperatorCommandOutcome::failure(action_confirm_text(action_state.action, true));
        }
        ActionDecision::Pending | ActionDecision::Confirmed => {}
    }

    let Some(args) = service_args_for_action(action_state.action) else {
        return OperatorCommandOutcome::success("dashboard action completed\n");
    };

    execute_service_command(
        &args,
        context.binary_path.clone(),
        context.data_dir.clone(),
        context.maybe_config_path.clone(),
        context.maybe_log_path.clone(),
        context.manager,
    )
}

/// Human-readable confirmation text shown before service-affecting actions.
pub fn action_confirm_text(action: DashboardAction, apply_mode: bool) -> String {
    let effect = match action {
        DashboardAction::InstallService => "install the user-level Open Bitcoin service",
        DashboardAction::UninstallService => "remove the user-level Open Bitcoin service",
        DashboardAction::EnableService => "enable automatic service startup",
        DashboardAction::DisableService => "disable automatic service startup",
        DashboardAction::ShowStatus => "inspect service status",
        DashboardAction::Refresh => "refresh dashboard data",
        DashboardAction::Exit => "exit the dashboard",
        DashboardAction::Help => "show dashboard help",
        DashboardAction::Confirm => "confirm the pending action",
        DashboardAction::Cancel => "cancel the pending action",
        DashboardAction::None => "no action",
    };
    let mode = if apply_mode { "apply" } else { "preview" };
    format!("Confirm {mode}: {effect}. Press y to confirm or n to cancel.")
}

fn service_args_for_action(action: DashboardAction) -> Option<ServiceArgs> {
    let command = match action {
        DashboardAction::ShowStatus => ServiceCommand::Status,
        DashboardAction::InstallService => ServiceCommand::Install,
        DashboardAction::UninstallService => ServiceCommand::Uninstall,
        DashboardAction::EnableService => ServiceCommand::Enable,
        DashboardAction::DisableService => ServiceCommand::Disable,
        DashboardAction::Refresh
        | DashboardAction::Exit
        | DashboardAction::Help
        | DashboardAction::Confirm
        | DashboardAction::Cancel
        | DashboardAction::None => return None,
    };

    Some(ServiceArgs {
        command,
        apply: action.requires_confirmation(),
    })
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::super::super::service::fake::{FakeServiceCall, FakeServiceManager};
    use super::{
        DashboardAction, DashboardActionState, DashboardServiceContext, confirm_and_execute,
    };

    #[test]
    fn pending_service_action_does_not_call_manager() {
        // Arrange
        let manager = FakeServiceManager::unmanaged();
        let context = context(&manager);
        let state = DashboardActionState::pending(DashboardAction::InstallService);

        // Act
        let outcome = confirm_and_execute(&state, &context);

        // Assert
        assert_eq!(outcome.exit_code.code(), 1);
        assert!(manager.recorded_calls.borrow().is_empty());
        assert!(outcome.stderr.text.contains("Confirm apply"));
    }

    #[test]
    fn confirmed_service_action_uses_service_command_path() {
        // Arrange
        let manager = FakeServiceManager::unmanaged();
        let context = context(&manager);
        let state = DashboardActionState::confirmed(DashboardAction::InstallService);

        // Act
        let outcome = confirm_and_execute(&state, &context);

        // Assert
        assert_eq!(outcome.exit_code.code(), 0);
        assert_eq!(
            manager.recorded_calls.borrow().as_slice(),
            &[FakeServiceCall::Install { apply: true }]
        );
    }

    #[test]
    fn cancelled_service_action_does_not_call_manager() {
        // Arrange
        let manager = FakeServiceManager::unmanaged();
        let context = context(&manager);
        let state = DashboardActionState::cancelled(DashboardAction::DisableService);

        // Act
        let outcome = confirm_and_execute(&state, &context);

        // Assert
        assert_eq!(outcome.exit_code.code(), 0);
        assert!(manager.recorded_calls.borrow().is_empty());
    }

    fn context<'a>(manager: &'a FakeServiceManager) -> DashboardServiceContext<'a> {
        DashboardServiceContext {
            binary_path: PathBuf::from("open-bitcoin"),
            data_dir: PathBuf::from("/tmp/open-bitcoin"),
            maybe_config_path: None,
            maybe_log_path: None,
            manager,
        }
    }
}
