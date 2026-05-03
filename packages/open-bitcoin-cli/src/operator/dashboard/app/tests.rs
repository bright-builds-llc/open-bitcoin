// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

use std::{cell::RefCell, path::PathBuf, rc::Rc};

use super::{
    DashboardAction, DashboardAppError, DashboardState, handle_action, handle_dashboard_action,
    is_interactive_dashboard_height_sufficient, render_action_line,
};
use crate::operator::{
    config::OperatorConfigResolution,
    dashboard::model::ActionEntry,
    dashboard::{DashboardRuntimeContext, DashboardServiceRuntime, collect_dashboard_snapshot},
    service::{
        ServiceCommandOutcome, ServiceDisableRequest, ServiceEnableRequest, ServiceError,
        ServiceInstallRequest, ServiceLifecycleState, ServiceManager, ServiceStateSnapshot,
        ServiceUninstallRequest, fake::FakeServiceCall,
    },
    status::{
        StatusCollectorInput, StatusDetectionEvidence, StatusRenderMode, StatusRequest,
        StatusWalletRpcAccess,
    },
};

struct TestServiceManager {
    calls: Rc<RefCell<Vec<FakeServiceCall>>>,
    snapshot: ServiceStateSnapshot,
}

impl TestServiceManager {
    fn new(calls: Rc<RefCell<Vec<FakeServiceCall>>>, snapshot: ServiceStateSnapshot) -> Self {
        Self { calls, snapshot }
    }
}

impl ServiceManager for TestServiceManager {
    fn install(
        &self,
        request: &ServiceInstallRequest,
    ) -> Result<ServiceCommandOutcome, ServiceError> {
        self.calls.borrow_mut().push(FakeServiceCall::Install {
            apply: request.apply,
        });
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
        self.calls.borrow_mut().push(FakeServiceCall::Uninstall {
            apply: request.apply,
        });
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
        self.calls.borrow_mut().push(FakeServiceCall::Enable);
        Ok(ServiceCommandOutcome {
            dry_run: false,
            description: "fake enable".to_string(),
            maybe_file_path: None,
            maybe_file_content: None,
            commands_that_would_run: vec![],
        })
    }

    fn disable(
        &self,
        _request: &ServiceDisableRequest,
    ) -> Result<ServiceCommandOutcome, ServiceError> {
        self.calls.borrow_mut().push(FakeServiceCall::Disable);
        Ok(ServiceCommandOutcome {
            dry_run: false,
            description: "fake disable".to_string(),
            maybe_file_path: None,
            maybe_file_content: None,
            commands_that_would_run: vec![],
        })
    }

    fn status(&self) -> Result<ServiceStateSnapshot, ServiceError> {
        self.calls.borrow_mut().push(FakeServiceCall::Status);
        Ok(self.snapshot.clone())
    }
}

#[test]
fn service_install_action_requires_confirmation_then_executes() -> Result<(), DashboardAppError> {
    // Arrange
    let calls = Rc::new(RefCell::new(Vec::new()));
    let context = test_context(
        Rc::clone(&calls),
        ServiceStateSnapshot {
            state: ServiceLifecycleState::Unmanaged,
            maybe_enabled: Some(false),
            maybe_service_file_path: None,
            maybe_manager_diagnostics: None,
            maybe_log_path: None,
            maybe_log_path_unavailable_reason: Some("service not installed".to_string()),
        },
    );
    let mut state = test_state(&context);
    let mut maybe_pending_action = None;
    let mut message = String::new();

    // Act
    let should_exit = handle_action(
        DashboardAction::InstallService,
        &context,
        &mut state,
        &mut maybe_pending_action,
        &mut message,
    )?;
    let should_exit_after_confirm = handle_action(
        DashboardAction::Confirm,
        &context,
        &mut state,
        &mut maybe_pending_action,
        &mut message,
    )?;

    // Assert
    assert!(!should_exit);
    assert!(!should_exit_after_confirm);
    assert_eq!(
        calls.borrow().as_slice(),
        &[FakeServiceCall::Install { apply: true }]
    );
    assert!(message.contains("fake install"));
    Ok(())
}

#[test]
fn render_action_line_inserts_visual_separators_between_actions() {
    // Arrange
    let actions = vec![
        ActionEntry {
            key: "r".to_string(),
            label: "refresh".to_string(),
            destructive: false,
        },
        ActionEntry {
            key: "i".to_string(),
            label: "install service".to_string(),
            destructive: true,
        },
        ActionEntry {
            key: "q".to_string(),
            label: "quit".to_string(),
            destructive: false,
        },
    ];

    // Act
    let rendered = render_action_line(&actions);

    // Assert
    let text = rendered
        .iter()
        .map(|span| span.content.as_ref())
        .collect::<String>();
    assert_eq!(text, "r refresh | i install service | q quit");
}

#[test]
fn interactive_dashboard_height_requires_at_least_twenty_five_rows() {
    // Arrange
    let too_short = 24;
    let just_right = 25;

    // Act / Assert
    assert!(!is_interactive_dashboard_height_sufficient(too_short));
    assert!(is_interactive_dashboard_height_sufficient(just_right));
}

#[test]
fn blocked_dashboard_ignores_destructive_actions_but_allows_exit() -> Result<(), DashboardAppError>
{
    // Arrange
    let calls = Rc::new(RefCell::new(Vec::new()));
    let context = test_context(
        Rc::clone(&calls),
        ServiceStateSnapshot {
            state: ServiceLifecycleState::Unmanaged,
            maybe_enabled: Some(false),
            maybe_service_file_path: None,
            maybe_manager_diagnostics: None,
            maybe_log_path: None,
            maybe_log_path_unavailable_reason: Some("service not installed".to_string()),
        },
    );
    let mut state = test_state(&context);
    let mut maybe_pending_action = None;
    let mut message = String::from("unchanged");

    // Act
    let blocked_install_exit = handle_dashboard_action(
        DashboardAction::InstallService,
        true,
        &context,
        &mut state,
        &mut maybe_pending_action,
        &mut message,
    )?;
    let blocked_exit = handle_dashboard_action(
        DashboardAction::Exit,
        true,
        &context,
        &mut state,
        &mut maybe_pending_action,
        &mut message,
    )?;

    // Assert
    assert!(!blocked_install_exit);
    assert!(blocked_exit);
    assert!(calls.borrow().is_empty());
    assert!(maybe_pending_action.is_none());
    assert_eq!(message, "unchanged");
    Ok(())
}

#[test]
fn service_install_action_can_be_cancelled_without_side_effects() -> Result<(), DashboardAppError> {
    // Arrange
    let calls = Rc::new(RefCell::new(Vec::new()));
    let context = test_context(
        Rc::clone(&calls),
        ServiceStateSnapshot {
            state: ServiceLifecycleState::Unmanaged,
            maybe_enabled: Some(false),
            maybe_service_file_path: None,
            maybe_manager_diagnostics: None,
            maybe_log_path: None,
            maybe_log_path_unavailable_reason: Some("service not installed".to_string()),
        },
    );
    let mut state = test_state(&context);
    let mut maybe_pending_action = None;
    let mut message = String::new();

    // Act
    handle_action(
        DashboardAction::InstallService,
        &context,
        &mut state,
        &mut maybe_pending_action,
        &mut message,
    )?;
    let should_exit = handle_action(
        DashboardAction::Cancel,
        &context,
        &mut state,
        &mut maybe_pending_action,
        &mut message,
    )?;

    // Assert
    assert!(!should_exit);
    assert!(calls.borrow().is_empty());
    assert_eq!(message, "confirmation cancelled");
    Ok(())
}

#[test]
fn show_status_action_reuses_shared_service_command_path() -> Result<(), DashboardAppError> {
    // Arrange
    let calls = Rc::new(RefCell::new(Vec::new()));
    let context = test_context(
        Rc::clone(&calls),
        ServiceStateSnapshot {
            state: ServiceLifecycleState::Running,
            maybe_enabled: Some(true),
            maybe_service_file_path: Some(PathBuf::from("/tmp/open-bitcoin.service")),
            maybe_manager_diagnostics: Some("manager healthy".to_string()),
            maybe_log_path: Some(PathBuf::from("/tmp/open-bitcoin.log")),
            maybe_log_path_unavailable_reason: None,
        },
    );
    let mut state = test_state(&context);
    let mut maybe_pending_action = None;
    let mut message = String::new();

    // Act
    let should_exit = handle_action(
        DashboardAction::ShowStatus,
        &context,
        &mut state,
        &mut maybe_pending_action,
        &mut message,
    )?;

    // Assert
    assert!(!should_exit);
    assert_eq!(calls.borrow().as_slice(), &[FakeServiceCall::Status]);
    assert!(message.contains("service: running"));
    assert!(message.contains("logs: /tmp/open-bitcoin.log"));
    Ok(())
}

fn test_context(
    calls: Rc<RefCell<Vec<FakeServiceCall>>>,
    snapshot: ServiceStateSnapshot,
) -> DashboardRuntimeContext {
    DashboardRuntimeContext {
        render_mode: StatusRenderMode::Human,
        status_input: StatusCollectorInput {
            request: StatusRequest {
                render_mode: StatusRenderMode::Human,
                maybe_config_path: None,
                maybe_data_dir: Some(PathBuf::from("/tmp/open-bitcoin")),
                maybe_network: None,
                include_live_rpc: false,
                no_color: true,
            },
            config_resolution: OperatorConfigResolution {
                maybe_data_dir: Some(PathBuf::from("/tmp/open-bitcoin")),
                ..OperatorConfigResolution::default()
            },
            detection_evidence: StatusDetectionEvidence {
                detected_installations: Vec::new(),
                service_candidates: Vec::new(),
            },
            maybe_live_rpc: None,
            maybe_service_manager: None,
            wallet_rpc_access: StatusWalletRpcAccess::Root,
        },
        maybe_rpc_client: None,
        service: DashboardServiceRuntime {
            binary_path: PathBuf::from("open-bitcoin"),
            data_dir: PathBuf::from("/tmp/open-bitcoin"),
            maybe_config_path: None,
            maybe_log_path: None,
            manager: Box::new(TestServiceManager::new(calls, snapshot)),
        },
    }
}

fn test_state(context: &DashboardRuntimeContext) -> DashboardState {
    DashboardState::from_snapshot(&collect_dashboard_snapshot(context))
}
