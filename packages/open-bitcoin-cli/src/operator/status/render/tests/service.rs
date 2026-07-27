use super::*;

#[test]
fn phase63_service_lifecycle_rendering_human_status_contract() {
    // Arrange
    let snapshot = shared_sync_truth_snapshot();

    // Act
    let rendered = render_status(&snapshot, StatusRenderMode::Human).expect("human status");

    // Assert
    assert!(rendered.contains(
        "Service: lifecycle=running manager=launchd installed=true enabled=true running=true file=/tmp/open-bitcoin-node.service logs=/tmp/logs/open-bitcoin.log diagnostics=Unavailable: service diagnostics unavailable"
    ));

    let mut unavailable = shared_sync_truth_snapshot();
    unavailable.service = ServiceStatus {
        manager: FieldAvailability::unavailable(
            "service manager unavailable: unsupported platform: launchd unavailable",
        ),
        lifecycle: FieldAvailability::available(ServiceLifecycleStatus::UnavailableManager),
        installed: FieldAvailability::unavailable(
            "service manager unavailable: unsupported platform: launchd unavailable",
        ),
        enabled: FieldAvailability::unavailable(
            "service manager unavailable: unsupported platform: launchd unavailable",
        ),
        running: FieldAvailability::unavailable(
            "service manager unavailable: unsupported platform: launchd unavailable",
        ),
        service_file_path: FieldAvailability::unavailable(
            "service manager unavailable: unsupported platform: launchd unavailable",
        ),
        log_path: FieldAvailability::unavailable(
            "service manager unavailable: unsupported platform: launchd unavailable",
        ),
        diagnostics: FieldAvailability::available(
            "unsupported platform: launchd unavailable".to_string(),
        ),
        restart_resume: FieldAvailability::unavailable(
            "service restart/resume evidence unavailable",
        ),
    };

    let rendered = render_status(&unavailable, StatusRenderMode::Human).expect("human status");

    assert!(rendered.contains("Service: lifecycle=unavailable-manager manager=Unavailable: service manager unavailable: unsupported platform: launchd unavailable"));
    assert!(rendered.contains("file=Unavailable: service manager unavailable"));
    assert!(rendered.contains("logs=Unavailable: service manager unavailable"));
    assert!(rendered.contains("diagnostics=unsupported platform: launchd unavailable"));
}

#[test]
fn service_restart_resume_status_render_includes_phase64_evidence() {
    // Arrange
    let snapshot = shared_sync_truth_snapshot();

    // Act
    let human = render_status(&snapshot, StatusRenderMode::Human).expect("human status");
    let json = render_status(&snapshot, StatusRenderMode::Json).expect("json status");
    let decoded: serde_json::Value = serde_json::from_str(&json).expect("decode status json");

    // Assert
    assert!(human.contains("restart_resume=datadir=/tmp/open-bitcoin same_datadir=true prior_shutdown=clean downloaded=840006 connected=840004 stale_inflight=cleared recovery_category=clean_shutdown next_action=Resume service sync review from preserved durable progress."));
    assert_eq!(decoded["service"]["restart_resume"]["state"], "available");
    assert_eq!(
        decoded["service"]["restart_resume"]["value"]["prior_shutdown"]["value"],
        "clean"
    );
    assert_eq!(
        decoded["service"]["restart_resume"]["value"]["stale_inflight"]["value"],
        "cleared"
    );
    assert_eq!(
        decoded["service"]["restart_resume"]["value"]["durable_progress"]["value"]["downloaded_block_height"],
        840_006
    );
}
