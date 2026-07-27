use super::*;

#[test]
fn service_restart_resume_status_does_not_load_storage_recovery_action() {
    // Arrange
    let path = temp_path("service-restart-resume-storage-action");
    remove_dir_if_exists(&path);
    let _guard = TempDirGuard { path: path.clone() };

    let mut resolution = config_resolution();
    resolution.maybe_data_dir = Some(path.clone());
    let input = status_input_with_manager(
        Box::new(FakeServiceManager::new(service_snapshot(
            ServiceLifecycleState::Running,
            Some(true),
            &path,
        ))),
        resolution,
    );

    // Act
    let snapshot = collect_status_snapshot(&input, None);

    // Assert
    let FieldAvailability::Available(restart_resume) = snapshot.service.restart_resume else {
        panic!("restart/resume evidence should be available");
    };
    assert_eq!(
        restart_resume.next_action,
        FieldAvailability::unavailable(
            "service restart/resume evidence unavailable: probe-only status does not open Fjall stores"
        )
    );
    assert_eq!(
        restart_resume.recovery_category,
        FieldAvailability::unavailable("no recovery category recorded")
    );
}

#[test]
fn service_restart_resume_status_reports_probe_only_runtime_metadata_unavailable() {
    // Arrange
    let input = status_input_with_manager(
        Box::new(FakeServiceManager::new(service_snapshot(
            ServiceLifecycleState::Running,
            Some(true),
            Path::new("/tmp/nonexistent-open-bitcoin-status"),
        ))),
        OperatorConfigResolution {
            maybe_data_dir: Some(PathBuf::from("/tmp/nonexistent-open-bitcoin-status")),
            ..config_resolution()
        },
    );

    // Act
    let snapshot = collect_status_snapshot(&input, None);

    // Assert
    let FieldAvailability::Available(restart_resume) = snapshot.service.restart_resume else {
        panic!("restart/resume evidence should keep service same-datadir evidence");
    };
    assert_eq!(
        restart_resume.same_datadir,
        FieldAvailability::available(true)
    );
    assert_eq!(
        restart_resume.prior_shutdown,
        FieldAvailability::unavailable(
            "service restart/resume evidence unavailable: probe-only status does not open Fjall stores"
        )
    );
}
