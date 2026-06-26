// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

use open_bitcoin_node::{
    BuildProvenance, LogStatus, MetricsStatus,
    status::{
        BestKnownTipSource, BestKnownTipStatus, ConfigStatus, FieldAvailability,
        InboundAddressDecisionEvent, InboundAddressEvidenceEntry, InboundAdmissionEvent,
        InboundHandshakeStatusCounts, InboundPeerServingStatus, InboundPermissionDecisionEvent,
        MempoolStatus, NoProgressDiagnosis, NoProgressThresholdEvidence, NoProgressThresholdState,
        NodeRuntimeState, NodeStatus, OpenBitcoinStatusSnapshot, PeerContributionEvidence,
        PeerContributionKind, PeerCounts, PeerStatus, PeerTelemetry, PeerTipAgreement,
        PeerTipAgreementStatus, ProgressCreditEvidence, ProgressCreditKind, ProgressWindowEvidence,
        RejectedProgressActivity, RejectedProgressActivityKind, ServiceLifecycleStatus,
        ServicePriorShutdownStatus, ServiceRestartResumeStatus, ServiceResumeProgressStatus,
        ServiceStaleInflightStatus, ServiceStatus, StallDiagnosisConfidence,
        StallDiagnosisEvidence, StalledSubsystem, StayCurrentStatus, SyncAttemptCounters,
        SyncConfiguredTargets, SyncLagStatus, SyncLifecycleState, SyncProgress, SyncProgressSignal,
        SyncReconcileProgressStatus, SyncRecoveryCategory, SyncReorgEvidence, SyncResourcePressure,
        SyncStatus, SyncStopReasonStatus, TipFreshnessStatus, WalletStatus,
    },
};

use super::{StatusRenderMode, render_status};

#[test]
fn status_render_includes_sync_progress_and_peer_evidence() {
    // Arrange
    let snapshot = shared_sync_truth_snapshot();

    // Act
    let rendered = render_status(&snapshot, StatusRenderMode::Human).expect("human status");

    // Assert
    assert!(rendered.contains("headers=840100 downloaded_blocks=840006 connected_blocks=840004"));
    for expected in [
        "Sync configured targets: outbound_peers=4 target_header_height=840200",
        "Sync attempts: attempted_peers=3 connected_peers=2 failed_peers=1 max_sync_rounds=8",
        "Sync latest stop reason: target_header_reached",
        "awaiting_blocks",
        "Sync recovery category: invalid_peer_data",
        "Sync recovery: Retry sync after peer backoff",
        "peer stalled before block connect",
        "failed:seed.bitcoin.sipa.be:8333 via dns_seed",
    ] {
        assert!(rendered.contains(expected));
    }

    let mut snapshot = shared_sync_truth_snapshot();
    snapshot.sync.configured_targets =
        FieldAvailability::unavailable("operator target unavailable");
    snapshot.sync.attempt_counters = FieldAvailability::unavailable("attempt counters unavailable");
    snapshot.sync.latest_stop_reason = FieldAvailability::unavailable("stop reason unavailable");

    let rendered = render_status(&snapshot, StatusRenderMode::Human).expect("human status");

    for expected in [
        "Sync configured targets: Unavailable: operator target unavailable",
        "Sync attempts: Unavailable: attempt counters unavailable",
        "Sync latest stop reason: Unavailable: stop reason unavailable",
    ] {
        assert!(rendered.contains(expected));
    }
    for unexpected in [
        "Sync configured targets: outbound_peers=0",
        "Sync attempts: attempted_peers=0",
        "Sync latest stop reason: ok",
    ] {
        assert!(!rendered.contains(unexpected));
    }
}

#[test]
fn inbound_status_render_includes_listener_and_admission_evidence() {
    // Arrange
    let snapshot = shared_sync_truth_snapshot();

    // Act
    let rendered = render_status(&snapshot, StatusRenderMode::Human).expect("human status");

    // Assert
    assert!(rendered.contains("Peers: in=0 out=2"));
    assert!(rendered.contains("Inbound serving:"));
    for expected in [
        "listener_state=listening",
        "bound_endpoints=127.0.0.1:18444,[::1]:18444",
        "preflight_reason=ready",
        "admitted_inbound_peers=2",
        "rejected_inbound_peers=5",
        "handshake=awaiting_version=1 awaiting_verack=2 established=3 disconnected=4",
        "duplicate_rejects=1",
        "self_connection_rejects=1",
        "cap_rejects=2",
        "reserved_slot_rejects=1",
        "permission_class=protected_inbound",
        "permissioned_inbound_peers=1",
        "protected_inbound_peers=1",
        "active_permission_effects=admission_protected,eviction_policy_protected,download_serving_policy_input",
        "inactive_permission_effects=inactive_relay,inactive_mempool,inactive_blockfilters",
        "latest_permission_decision=outcome=admitted reason=admitted permission_class=protected_inbound active_permission_effects=admission_protected,download_serving_policy_input inactive_permission_effects=inactive_relay message=inbound permission decision admitted as protected_inbound",
        "latest_admission_event=outcome=rejected reason=cap_reached slot_class=ordinary message=inbound cap reached",
    ] {
        assert!(rendered.contains(expected), "missing {expected}");
    }
}

#[test]
fn inbound_status_render_includes_phase92_address_boundary_evidence() {
    // Arrange
    let snapshot = shared_sync_truth_snapshot();

    // Act
    let rendered = render_status(&snapshot, StatusRenderMode::Human).expect("human status");

    // Assert
    for expected in [
        "local advertisement candidates: 1",
        "source=source_local_listener",
        "routability=publicly_routable",
        "persistence_eligible=true",
        "suppressed advertisements: 2",
        "label=not_publicly_routable",
        "bounded getaddr responses served: 3",
        "bounded getaddr requests suppressed: 2",
        "learned address entries: 5",
        "learned address rejections: 1",
        "latest address decision=outcome=suppressed reason=already_served label=getaddr_suppressed source=source_inbound_addr message=bounded getaddr request already served",
    ] {
        assert!(rendered.contains(expected), "missing {expected}");
    }
}

#[test]
fn inbound_status_render_preserves_unavailable_address_decision_reason() {
    // Arrange
    let mut snapshot = shared_sync_truth_snapshot();
    let FieldAvailability::Available(inbound) = &mut snapshot.peers.inbound else {
        panic!("inbound status fixture should be available");
    };
    inbound.latest_address_decision =
        FieldAvailability::unavailable("inbound address boundary evidence unavailable");

    // Act
    let rendered = render_status(&snapshot, StatusRenderMode::Human).expect("human status");

    // Assert
    assert!(rendered.contains(
        "latest address decision=Unavailable: inbound address boundary evidence unavailable"
    ));
}

#[test]
fn inbound_status_render_uses_none_for_empty_permission_effects() {
    // Arrange
    let mut snapshot = shared_sync_truth_snapshot();
    let FieldAvailability::Available(inbound) = &mut snapshot.peers.inbound else {
        panic!("inbound status fixture should be available");
    };
    inbound.active_permission_effects = Vec::new();
    inbound.inactive_permission_effects = Vec::new();
    inbound.latest_permission_decision =
        FieldAvailability::available(InboundPermissionDecisionEvent {
            outcome: "admitted".to_string(),
            reason: "admitted".to_string(),
            permission_class: "ordinary_inbound".to_string(),
            active_permission_effects: Vec::new(),
            inactive_permission_effects: Vec::new(),
            message: "inbound permission decision admitted as ordinary_inbound".to_string(),
        });

    // Act
    let rendered = render_status(&snapshot, StatusRenderMode::Human).expect("human status");

    // Assert
    assert!(rendered.contains("active_permission_effects=none"));
    assert!(rendered.contains("inactive_permission_effects=none"));
    assert!(rendered.contains(
        "latest_permission_decision=outcome=admitted reason=admitted permission_class=ordinary_inbound active_permission_effects=none inactive_permission_effects=none message=inbound permission decision admitted as ordinary_inbound"
    ));
}

#[test]
fn inbound_status_render_preserves_unavailable_reason() {
    // Arrange
    let mut snapshot = shared_sync_truth_snapshot();
    snapshot.peers.inbound = FieldAvailability::unavailable("legacy daemon");

    // Act
    let rendered = render_status(&snapshot, StatusRenderMode::Human).expect("human status");

    // Assert
    assert!(rendered.contains("Inbound serving: Unavailable: legacy daemon"));
}

#[test]
fn status_render_uses_shared_no_progress_fields() {
    // Arrange
    let mut snapshot = shared_sync_truth_snapshot();
    snapshot.sync.no_progress_diagnosis =
        FieldAvailability::available(NoProgressDiagnosis::PeerBackoff);
    snapshot.sync.no_progress_next_action = FieldAvailability::available(
        "Wait for retry backoff or try another configured peer.".to_string(),
    );

    // Act
    let rendered = render_status(&snapshot, StatusRenderMode::Human).expect("human status");

    // Assert
    assert!(rendered.contains("Sync no-progress diagnosis: peer_backoff"));
    assert!(rendered.contains(
        "Sync no-progress action: Wait for retry backoff or try another configured peer."
    ));

    // Arrange
    snapshot.sync.no_progress_diagnosis = FieldAvailability::unavailable("diagnosis withheld");
    snapshot.sync.no_progress_next_action = FieldAvailability::unavailable("guidance withheld");

    // Act
    let unavailable = render_status(&snapshot, StatusRenderMode::Human).expect("human status");

    // Assert
    assert!(unavailable.contains("Sync no-progress diagnosis: Unavailable: diagnosis withheld"));
    assert!(unavailable.contains("Sync no-progress action: Unavailable: guidance withheld"));
}

#[test]
fn status_render_includes_phase78_progress_guarantee_fields() {
    // Arrange
    let mut snapshot = shared_sync_truth_snapshot();
    apply_phase78_available_sync_fields(&mut snapshot.sync);

    // Act
    let rendered = render_status(&snapshot, StatusRenderMode::Human).expect("human status");

    // Assert
    for expected in [
        "Sync progress credit: kind=validated_durable_active_chain height=840004 hash=1111111111111111111111111111111111111111111111111111111111111111 work=840005 source_unix_seconds=1717000020 rejected_activity_count=1",
        "Sync expected progress window: expected_progress_window_seconds=300 retry_backoff_seconds=30 max_sync_rounds=8 tip_freshness_threshold_seconds=600",
        "Sync no-progress threshold: state=within_window threshold_seconds=300 elapsed_since_last_useful_work_seconds=12 evaluated_at_unix_seconds=1717000032",
        "Sync last useful work: kind=current_at_best_known_tip height=840004 hash=1111111111111111111111111111111111111111111111111111111111111111 work=840005 source_unix_seconds=1717000025 rejected_activity_count=0",
        "Sync last peer contribution: peer=peer-1 endpoint=203.0.113.10:8333 kind=headers_and_blocks messages=7 headers=3 blocks=1 last_activity_unix_seconds=1717000028 failure=Unavailable: no peer failure recorded",
        "Sync stalled subsystem: stalled_subsystem=at_tip_waiting confidence=high basis=stay_current,current_tip next_action=No operator action required. no_progress_diagnosis=current_at_best_known_tip recovery_category=Unavailable: no recovery category latest_stop_reason=best_known_tip_reached",
    ] {
        assert!(rendered.contains(expected), "missing {expected}");
    }
}

#[test]
fn status_render_phase78_unavailable_reasons() {
    // Arrange
    let snapshot = shared_sync_truth_snapshot();

    // Act
    let rendered = render_status(&snapshot, StatusRenderMode::Human).expect("human status");

    // Assert
    for expected in [
        "Sync progress credit: Unavailable: progress credit evidence unavailable",
        "Sync expected progress window: Unavailable: expected progress window unavailable",
        "Sync no-progress threshold: Unavailable: no-progress threshold evidence unavailable",
        "Sync last useful work: Unavailable: last useful work unavailable",
        "Sync last peer contribution: Unavailable: last peer contribution unavailable",
        "Sync stalled subsystem: Unavailable: stall diagnosis unavailable",
    ] {
        assert!(rendered.contains(expected), "missing {expected}");
    }
}

#[test]
fn phase72_cli_status_renders_full_sync_truth_contract() {
    // Arrange
    let mut snapshot = shared_sync_truth_snapshot();
    snapshot.sync.best_known_tip = FieldAvailability::available(BestKnownTipStatus {
        source: BestKnownTipSource::HeaderStore,
        height: 840_004,
        block_hash: "11".repeat(32),
        work: "840005".to_string(),
        block_time_unix_seconds: 1_717_000_010,
        observed_at_unix_seconds: 1_717_000_020,
        freshness: TipFreshnessStatus::Fresh,
        peer_agreement: vec![PeerTipAgreement {
            peer: "peer-1".to_string(),
            maybe_resolved_endpoint: Some("203.0.113.10:8333".to_string()),
            status: PeerTipAgreementStatus::Agrees,
            maybe_height: Some(840_004),
            maybe_hash: Some("11".repeat(32)),
            maybe_work: Some("840005".to_string()),
            maybe_last_activity_unix_seconds: Some(1_717_000_020),
        }],
    });
    snapshot.sync.stay_current =
        FieldAvailability::available(StayCurrentStatus::CurrentAtBestKnownTip);
    snapshot.sync.stay_current_next_action =
        FieldAvailability::available("Continue monitoring best-known tip freshness.".to_string());
    snapshot.sync.no_progress_diagnosis =
        FieldAvailability::available(NoProgressDiagnosis::CurrentAtBestKnownTip);
    snapshot.sync.no_progress_next_action =
        FieldAvailability::available("No operator action required.".to_string());
    snapshot.sync.latest_reorg = FieldAvailability::available(SyncReorgEvidence {
        common_ancestor_height: 840_000,
        common_ancestor_hash: "00".repeat(32),
        disconnected_count: 2,
        connected_count: 4,
        final_active_height: 840_004,
        final_active_hash: "11".repeat(32),
        fully_persisted: true,
    });
    snapshot.sync.reconcile_progress =
        FieldAvailability::available(SyncReconcileProgressStatus::ExtendedActiveChain {
            connected_count: 4,
            final_active_height: 840_004,
            final_active_hash: "11".repeat(32),
        });

    // Act
    let rendered = render_status(&snapshot, StatusRenderMode::Human).expect("human status");

    // Assert
    for prefix in [
        "Sync best-known tip:",
        "Sync stay-current:",
        "Sync stay-current action:",
        "Sync latest reorg:",
        "Sync reconcile:",
        "Sync no-progress diagnosis:",
        "Sync no-progress action:",
        "Sync pressure:",
        "Sync:",
    ] {
        assert!(
            rendered.lines().any(|line| line.starts_with(prefix)),
            "missing line prefix {prefix}"
        );
    }
    let sync_line = rendered
        .lines()
        .find(|line| line.starts_with("Sync:"))
        .expect("sync progress line");
    for expected in [
        "headers=840100",
        "downloaded_blocks=840006",
        "connected_blocks=840004",
        "validated_active_chain_height=840004",
        "validated_active_chain_hash=1111111111111111111111111111111111111111111111111111111111111111",
        "validated_active_chain_work=840005",
    ] {
        assert!(sync_line.contains(expected), "missing {expected}");
    }

    // Arrange
    snapshot.sync.best_known_tip =
        FieldAvailability::unavailable("best-known tip evidence unavailable");
    snapshot.sync.stay_current = FieldAvailability::unavailable("stay-current state unavailable");
    snapshot.sync.latest_reorg = FieldAvailability::unavailable("no reorg evidence recorded");
    snapshot.sync.reconcile_progress =
        FieldAvailability::unavailable("reconcile progress unavailable");

    // Act
    let unavailable = render_status(&snapshot, StatusRenderMode::Human).expect("human status");

    // Assert
    for expected in [
        "Sync best-known tip: Unavailable: best-known tip evidence unavailable",
        "Sync stay-current: Unavailable: stay-current state unavailable",
        "Sync latest reorg: Unavailable: no reorg evidence recorded",
        "Sync reconcile: Unavailable: reconcile progress unavailable",
    ] {
        assert!(unavailable.contains(expected), "missing {expected}");
    }
}

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

fn apply_phase78_available_sync_fields(sync: &mut SyncStatus) {
    sync.progress_credit = FieldAvailability::available(ProgressCreditEvidence {
        kind: ProgressCreditKind::ValidatedDurableActiveChain,
        credited_validated_active_chain_height: 840_004,
        credited_validated_active_chain_hash: "11".repeat(32),
        credited_validated_active_chain_work: "840005".to_string(),
        source_unix_seconds: 1_717_000_020,
        rejected_activity: vec![RejectedProgressActivity {
            kind: RejectedProgressActivityKind::HeaderDownload,
            observed_count: 3,
            reason: "headers do not prove durable active-chain progress".to_string(),
        }],
    });
    sync.expected_progress_window = FieldAvailability::available(ProgressWindowEvidence {
        retry_backoff_seconds: 30,
        max_sync_rounds: 8,
        expected_progress_window_seconds: 300,
        tip_freshness_threshold_seconds: 600,
    });
    sync.no_progress_threshold = FieldAvailability::available(NoProgressThresholdEvidence {
        threshold_seconds: 300,
        elapsed_since_last_useful_work_seconds: 12,
        state: NoProgressThresholdState::WithinWindow,
        evaluated_at_unix_seconds: 1_717_000_032,
    });
    sync.last_useful_work = FieldAvailability::available(ProgressCreditEvidence {
        kind: ProgressCreditKind::CurrentAtBestKnownTip,
        credited_validated_active_chain_height: 840_004,
        credited_validated_active_chain_hash: "11".repeat(32),
        credited_validated_active_chain_work: "840005".to_string(),
        source_unix_seconds: 1_717_000_025,
        rejected_activity: Vec::new(),
    });
    sync.last_peer_contribution = FieldAvailability::available(PeerContributionEvidence {
        peer: "peer-1".to_string(),
        maybe_resolved_endpoint: Some("203.0.113.10:8333".to_string()),
        kind: PeerContributionKind::HeadersAndBlocks,
        messages_processed: 7,
        headers_received: 3,
        blocks_received: 1,
        maybe_last_activity_unix_seconds: Some(1_717_000_028),
        maybe_failure_reason_label: None,
    });
    sync.stall_diagnosis = FieldAvailability::available(StallDiagnosisEvidence {
        stalled_subsystem: StalledSubsystem::AtTipWaiting,
        confidence: StallDiagnosisConfidence::High,
        evidence_basis: vec!["stay_current".to_string(), "current_tip".to_string()],
        next_action: "No operator action required.".to_string(),
        maybe_no_progress_diagnosis: Some(NoProgressDiagnosis::CurrentAtBestKnownTip),
        maybe_recovery_category: None,
        maybe_latest_stop_reason_label: Some("best_known_tip_reached".to_string()),
        source_unix_seconds: 1_717_000_032,
    });
}

fn shared_sync_truth_snapshot() -> OpenBitcoinStatusSnapshot {
    OpenBitcoinStatusSnapshot {
        node: NodeStatus {
            state: NodeRuntimeState::Running,
            version: "0.1.0".to_string(),
        },
        config: ConfigStatus {
            datadir: FieldAvailability::available("/tmp/open-bitcoin".to_string()),
            config_paths: vec!["/tmp/open-bitcoin/open-bitcoin.jsonc".to_string()],
        },
        service: ServiceStatus {
            manager: FieldAvailability::available("launchd".to_string()),
            lifecycle: FieldAvailability::available(ServiceLifecycleStatus::Running),
            installed: FieldAvailability::available(true),
            enabled: FieldAvailability::available(true),
            running: FieldAvailability::available(true),
            service_file_path: FieldAvailability::available(
                "/tmp/open-bitcoin-node.service".to_string(),
            ),
            log_path: FieldAvailability::available("/tmp/logs/open-bitcoin.log".to_string()),
            diagnostics: FieldAvailability::unavailable("service diagnostics unavailable"),
            restart_resume: FieldAvailability::available(ServiceRestartResumeStatus {
                datadir: FieldAvailability::available("/tmp/open-bitcoin".to_string()),
                same_datadir: FieldAvailability::available(true),
                prior_shutdown: FieldAvailability::available(ServicePriorShutdownStatus::Clean),
                durable_progress: FieldAvailability::available(ServiceResumeProgressStatus {
                    downloaded_block_height: 840_006,
                    connected_block_height: 840_004,
                    maybe_downloaded_block_hash: Some("22".repeat(32)),
                    maybe_connected_block_hash: Some("11".repeat(32)),
                }),
                stale_inflight: FieldAvailability::available(ServiceStaleInflightStatus::Cleared),
                recovery_category: FieldAvailability::available(
                    SyncRecoveryCategory::CleanShutdown,
                ),
                next_action: FieldAvailability::available(
                    "Resume service sync review from preserved durable progress.".to_string(),
                ),
            }),
        },
        sync: SyncStatus {
            network: FieldAvailability::available("mainnet".to_string()),
            chain_tip: FieldAvailability::unavailable("chain tip unavailable"),
            sync_progress: FieldAvailability::available(SyncProgress {
                header_height: 840_100,
                block_height: 840_004,
                downloaded_block_height: 840_006,
                connected_block_height: 840_004,
                validated_active_chain_height: 840_004,
                maybe_downloaded_block_hash: Some("22".repeat(32)),
                maybe_connected_block_hash: Some("11".repeat(32)),
                maybe_validated_active_chain_hash: Some("11".repeat(32)),
                maybe_validated_active_chain_work: Some("840005".to_string()),
                progress_ratio: 840_004.0 / 840_100.0,
                messages_processed: 7,
                headers_received: 3,
                blocks_received: 1,
            }),
            lifecycle: FieldAvailability::available(SyncLifecycleState::Active),
            phase: FieldAvailability::available("block_download".to_string()),
            configured_targets: FieldAvailability::available(SyncConfiguredTargets {
                target_outbound_peers: 4,
                maybe_target_header_height: Some(840_200),
            }),
            attempt_counters: FieldAvailability::available(SyncAttemptCounters {
                attempted_peers: 3,
                connected_peers: 2,
                failed_peers: 1,
                max_sync_rounds: 8,
            }),
            progress_signal: FieldAvailability::available(SyncProgressSignal::AwaitingBlocks),
            lag: FieldAvailability::available(SyncLagStatus {
                headers_remaining: 0,
                blocks_remaining: 96,
            }),
            last_successful_progress_unix_seconds: FieldAvailability::available(1_717_000_000),
            progress_credit: FieldAvailability::unavailable(
                "progress credit evidence unavailable",
            ),
            expected_progress_window: FieldAvailability::unavailable(
                "expected progress window unavailable",
            ),
            no_progress_threshold: FieldAvailability::unavailable(
                "no-progress threshold evidence unavailable",
            ),
            last_useful_work: FieldAvailability::unavailable("last useful work unavailable"),
            last_peer_contribution: FieldAvailability::unavailable(
                "last peer contribution unavailable",
            ),
            stall_diagnosis: FieldAvailability::unavailable("stall diagnosis unavailable"),
            latest_stop_reason: FieldAvailability::available(SyncStopReasonStatus {
                label: "target_header_reached".to_string(),
                message:
                    "sync header target reached: target_header_height=840200 best_header_height=840200"
                        .to_string(),
            }),
            last_error: FieldAvailability::available("peer stalled before block connect".to_string()),
            recovery_category: FieldAvailability::available(SyncRecoveryCategory::InvalidPeerData),
            recovery_action: FieldAvailability::available(
                "Retry sync after peer backoff or choose a different peer.".to_string(),
            ),
            resource_pressure: FieldAvailability::available(SyncResourcePressure {
                blocks_in_flight: 0,
                max_header_requests_in_flight_per_peer: 1,
                max_headers_per_message: 2_000,
                max_blocks_in_flight_per_peer: 16,
                max_blocks_in_flight_total: 64,
                max_messages_per_peer: 64,
                max_sync_rounds: 8,
                outbound_peers: 2,
                target_outbound_peers: 4,
            }),
            best_known_tip: FieldAvailability::<BestKnownTipStatus>::unavailable(
                "best-known tip evidence unavailable",
            ),
            stay_current: FieldAvailability::<StayCurrentStatus>::unavailable(
                "stay-current state unavailable",
            ),
            stay_current_next_action: FieldAvailability::unavailable(
                "stay-current next action unavailable",
            ),
            no_progress_diagnosis: FieldAvailability::unavailable(
                "no-progress diagnosis unavailable",
            ),
            no_progress_next_action: FieldAvailability::unavailable(
                "no-progress next action unavailable",
            ),
            latest_reorg: FieldAvailability::unavailable("no reorg evidence recorded"),
            reconcile_progress: FieldAvailability::unavailable("reconcile progress unavailable"),
        },
        peers: PeerStatus {
            peer_counts: FieldAvailability::available(PeerCounts {
                inbound: 0,
                outbound: 2,
            }),
            recent_peers: FieldAvailability::available(vec![PeerTelemetry {
                peer: "seed.bitcoin.sipa.be:8333".to_string(),
                source: "dns_seed".to_string(),
                state: "failed".to_string(),
                network: "mainnet".to_string(),
                attempts: 1,
                maybe_resolved_endpoint: FieldAvailability::available(
                    "203.0.113.10:8333".to_string(),
                ),
                capabilities: FieldAvailability::unavailable("peer capabilities unavailable"),
                headers_received: 3,
                blocks_received: 0,
                maybe_last_activity_unix_seconds: FieldAvailability::available(1_717_000_000),
                failure_reason: FieldAvailability::available("compatibility".to_string()),
                error: FieldAvailability::available(
                    "failed:seed.bitcoin.sipa.be:8333 via dns_seed".to_string(),
                ),
            }]),
            inbound: FieldAvailability::available(inbound_peer_serving_status()),
        },
        mempool: MempoolStatus {
            transactions: FieldAvailability::unavailable("mempool unavailable"),
        },
        wallet: WalletStatus {
            trusted_balance_sats: FieldAvailability::unavailable("wallet unavailable"),
            freshness: FieldAvailability::unavailable("wallet unavailable"),
            scan_progress: FieldAvailability::unavailable("wallet unavailable"),
        },
        logs: LogStatus::default(),
        metrics: MetricsStatus::default(),
        recovery_evidence: FieldAvailability::default(),
        resource_bounds: FieldAvailability::unavailable("resource bounds unavailable"),
        health_signals: Vec::new(),
        build: BuildProvenance::unavailable(),
    }
}

fn inbound_peer_serving_status() -> InboundPeerServingStatus {
    InboundPeerServingStatus {
        listener_state: "listening".to_string(),
        bound_endpoints: vec!["127.0.0.1:18444".to_string(), "[::1]:18444".to_string()],
        preflight_reason: "ready".to_string(),
        admitted_inbound_peers: 2,
        rejected_inbound_peers: 5,
        handshake: InboundHandshakeStatusCounts {
            awaiting_version: 1,
            awaiting_verack: 2,
            established: 3,
            disconnected: 4,
        },
        duplicate_rejects: 1,
        self_connection_rejects: 1,
        cap_rejects: 2,
        reserved_slot_rejects: 1,
        latest_admission_event: FieldAvailability::available(InboundAdmissionEvent {
            outcome: "rejected".to_string(),
            reason: "cap_reached".to_string(),
            slot_class: "ordinary".to_string(),
            message: "inbound cap reached".to_string(),
        }),
        permissioned_inbound_peers: 1,
        protected_inbound_peers: 1,
        permission_class: "protected_inbound".to_string(),
        active_permission_effects: vec![
            "admission_protected".to_string(),
            "eviction_policy_protected".to_string(),
            "download_serving_policy_input".to_string(),
        ],
        inactive_permission_effects: vec![
            "inactive_relay".to_string(),
            "inactive_mempool".to_string(),
            "inactive_blockfilters".to_string(),
        ],
        latest_permission_decision: FieldAvailability::available(InboundPermissionDecisionEvent {
            outcome: "admitted".to_string(),
            reason: "admitted".to_string(),
            permission_class: "protected_inbound".to_string(),
            active_permission_effects: vec![
                "admission_protected".to_string(),
                "download_serving_policy_input".to_string(),
            ],
            inactive_permission_effects: vec!["inactive_relay".to_string()],
            message: "inbound permission decision admitted as protected_inbound".to_string(),
        }),
        local_advertisement_candidates: vec![InboundAddressEvidenceEntry {
            source: "source_local_listener".to_string(),
            network_kind: "ipv4".to_string(),
            routability: "publicly_routable".to_string(),
            freshness: "fresh".to_string(),
            services_bits: 1,
            port: 8333,
            persistence_eligible: true,
        }],
        suppressed_advertisements: vec![
            InboundAddressDecisionEvent {
                outcome: "suppressed".to_string(),
                reason: "not_publicly_routable".to_string(),
                label: "not_publicly_routable".to_string(),
                source: "source_local_listener".to_string(),
                message: "local evidence only".to_string(),
            },
            InboundAddressDecisionEvent {
                outcome: "suppressed".to_string(),
                reason: "not_publicly_routable".to_string(),
                label: "not_publicly_routable".to_string(),
                source: "source_local_listener".to_string(),
                message: "local evidence only".to_string(),
            },
        ],
        getaddr_responses_served: 3,
        getaddr_requests_suppressed: 2,
        learned_address_entries: 5,
        learned_address_rejections: 1,
        latest_address_decision: FieldAvailability::available(InboundAddressDecisionEvent {
            outcome: "suppressed".to_string(),
            reason: "already_served".to_string(),
            label: "getaddr_suppressed".to_string(),
            source: "source_inbound_addr".to_string(),
            message: "bounded getaddr request already served".to_string(),
        }),
    }
}
