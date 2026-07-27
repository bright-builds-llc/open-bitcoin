use super::*;

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
fn inbound_status_render_includes_phase93_peer_policy_evidence() {
    // Arrange
    let snapshot = shared_sync_truth_snapshot();

    // Act
    let rendered = render_status(&snapshot, StatusRenderMode::Human).expect("human status");

    // Assert
    for expected in [
        "peer policy evidence: eviction_candidates_evaluated=2",
        "disconnects_requested=1",
        "discouraged_peers=1",
        "active_bans=1",
        "expired_bans=0",
        "manual_unbans=0",
        "misbehavior_observations=2",
        "protected_no_actions=1",
        "latest peer policy decision=outcome=selected reason=low_activity label=eviction_candidate_selected source=source_eviction_policy message=peer eviction decision eviction_candidate_selected: low_activity",
    ] {
        assert!(rendered.contains(expected), "missing {expected}");
    }
}

#[test]
fn inbound_status_render_includes_phase96_peer_policy_runtime_bridge_labels() {
    // Arrange
    let mut snapshot = shared_sync_truth_snapshot();
    let FieldAvailability::Available(inbound) = &mut snapshot.peers.inbound else {
        panic!("inbound status fixture should be available");
    };
    inbound.active_bans = 1;
    inbound.manual_unbans = 1;
    inbound.misbehavior_observations = 1;
    inbound.protected_no_actions = 1;
    inbound.latest_peer_policy_decision = FieldAvailability::available(InboundPeerPolicyEvent {
        outcome: "protected_no_action".to_string(),
        reason: "unbanned".to_string(),
        label: "ban_active".to_string(),
        source: "source_peer_policy_runtime_bridge".to_string(),
        message: "protected_no_action ban_active unbanned source_peer_policy_runtime_bridge"
            .to_string(),
    });

    // Act
    let rendered = render_status(&snapshot, StatusRenderMode::Human).expect("human status");

    // Assert
    for expected in [
        "peer policy evidence:",
        "active_bans=1",
        "manual_unbans=1",
        "misbehavior_observations=1",
        "protected_no_actions=1",
        "protected_no_action",
        "unbanned",
        "ban_active",
        "source_peer_policy_runtime_bridge",
    ] {
        assert!(rendered.contains(expected), "missing {expected}");
    }
}

#[test]
fn inbound_status_render_includes_phase94_resource_governance_evidence() {
    // Arrange
    let mut snapshot = shared_sync_truth_snapshot();
    let FieldAvailability::Available(inbound) = &mut snapshot.peers.inbound else {
        panic!("inbound status fixture should be available");
    };
    inbound.resource_pressure_events = 8;
    inbound.read_queue_pressure_events = 7;
    inbound.write_queue_pressure_events = 6;
    inbound.request_cap_events = 5;
    inbound.payload_rejections = 1;
    inbound.timeout_disconnects = 3;
    inbound.churn_rejections = 2;
    inbound.reconnect_suppressions = 4;
    inbound.latest_resource_governance_decision =
        FieldAvailability::available(InboundResourceGovernanceEvent {
            outcome: "rejected".to_string(),
            reason: "invalid_checksum".to_string(),
            label: "payload_rejected".to_string(),
            source: "source_inbound_resource_governance".to_string(),
            message: "bounded payload rejected".to_string(),
            next_action: "payload_rejected".to_string(),
        });

    // Act
    let rendered = render_status(&snapshot, StatusRenderMode::Human).expect("human status");

    // Assert
    for expected in [
        "resource evidence:",
        "resource_pressure_events=8",
        "read_queue_pressure_events=7",
        "write_queue_pressure_events=6",
        "request_cap_events=5",
        "payload_rejections=1",
        "timeout_disconnects=3",
        "churn_rejections=2",
        "reconnect_suppressions=4",
        "latest resource governance decision=outcome=rejected reason=invalid_checksum label=payload_rejected source=source_inbound_resource_governance message=bounded payload rejected next_action=payload_rejected",
    ] {
        assert!(rendered.contains(expected), "missing {expected}");
    }
}

#[test]
fn inbound_status_render_preserves_unavailable_resource_decision_reason() {
    // Arrange
    let snapshot = shared_sync_truth_snapshot();

    // Act
    let rendered = render_status(&snapshot, StatusRenderMode::Human).expect("human status");

    // Assert
    assert!(rendered.contains(
        "latest resource governance decision=Unavailable: inbound resource governance evidence unavailable"
    ));
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
