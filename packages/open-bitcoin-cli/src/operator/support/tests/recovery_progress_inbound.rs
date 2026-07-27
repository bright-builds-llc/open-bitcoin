// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

use super::*;

#[test]
fn support_bundle_redacts_sensitive_block_relay_reasons_in_json_and_markdown() {
    // Arrange
    let temp = TestDirectory::new("phase116-block-relay-redaction");
    let status = phase116_status_with_sensitive_block_relay_reasons();
    let bundle = phase77_support_bundle_with_status(temp.path(), status);

    // Act
    let serialized = serde_json::to_value(&bundle).expect("support bundle json");
    let json_text = serde_json::to_string_pretty(&bundle).expect("support json");
    let markdown = render::render_support_markdown(&bundle);
    let block_relay_json = serde_json::to_string_pretty(&serialized["status"]["block_relay"])
        .expect("block relay support json");
    let block_relay_markdown = markdown
        .split("## Block Relay Evidence")
        .nth(1)
        .and_then(|section| section.split("## Inbound Serving").next())
        .expect("block relay markdown section");

    // Assert
    assert_eq!(
        serialized["status"]["block_relay"]["block_serving"]["activation"]["value"]["reason"],
        json!("redacted_block_relay_evidence")
    );
    assert_eq!(
        serialized["status"]["block_relay"]["negotiation"]["value"]["reason"],
        json!("redacted_block_relay_evidence")
    );
    for rendered in [&block_relay_json, block_relay_markdown] {
        assert!(rendered.contains("redacted_block_relay_evidence"));
        for forbidden in [
            "cmpctblock",
            "blocktxn",
            "getblocktxn",
            "0000000000000000000000000000000000000000000000000000000000000000",
            "127.0.0.1:",
            "198.51.100.116:8333",
            "peer_id=",
            "permission_string",
            "credential=phase116",
            "secret=phase116",
            "cookie=phase116",
            "dynamic_label",
        ] {
            assert_absent(rendered, forbidden);
        }
    }
    assert!(json_text.contains("block relay evidence bounded/redacted"));
}

#[test]
fn authoritative_rpc_status_support_bundle_redacts_every_forbidden_material_class_in_json_and_markdown()
 {
    // Arrange
    let temp = TestDirectory::new("phase127-authoritative-rpc-redaction");
    let output_dir = temp.path().join("bundle");
    let status = phase127_authoritative_status_with_sensitive_operator_evidence();
    let network_status = OpenBitcoinNetworkStatusResponse {
        inbound: status.peers.inbound,
        relay: status.mempool.relay,
        block_relay: status.block_relay,
        metrics: status.metrics,
    };
    let rpc = Phase127SensitiveStatusRpc { network_status };
    let input = phase127_status_collector_input(temp.path());
    let args = SupportArgs {
        command: SupportCommand::Bundle(SupportBundleArgs {
            maybe_output_dir: Some(output_dir.clone()),
            maybe_live_smoke_report: None,
        }),
    };
    let forbidden = [
        "127.0.0.1:18444",
        "198.51.100.127:8333",
        "permission_string=in,noban",
        "rpcpassword=phase127-secret",
        "020000000001",
        "txid=aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        "peer-127-dynamic-label",
    ];
    let raw_rpc_json =
        serde_json::to_string_pretty(&rpc.network_status).expect("raw network status RPC json");
    for marker in forbidden {
        assert!(
            raw_rpc_json.contains(marker),
            "phase 127 raw RPC fixture must seed {marker}"
        );
    }

    // Act
    let collected = collect_status_snapshot(&input, Some(&rpc));
    let outcome = execute_support_command(
        &args,
        OperatorOutputFormat::Json,
        &input.config_resolution,
        collected,
    )
    .expect("production support command should write redacted evidence");
    let json_text =
        fs::read_to_string(output_dir.join("support-evidence.json")).expect("support JSON output");
    let markdown = fs::read_to_string(output_dir.join("support-evidence.md"))
        .expect("support Markdown output");

    // Assert
    assert_eq!(outcome.exit_code.code(), 0);
    for rendered in [&json_text, &markdown] {
        for marker in forbidden {
            assert_absent(rendered, marker);
        }
    }
    for expected in [
        "redacted_relay_mempool_evidence",
        "redacted_block_relay_evidence",
        "redacted_permission_class",
        "redacted_permission_effect",
        "redacted_address_evidence",
        "redacted_peer_policy_label",
        "redacted_resource_governance_evidence",
    ] {
        assert!(json_text.contains(expected), "JSON missing {expected}");
        assert!(markdown.contains(expected), "Markdown missing {expected}");
    }
}

#[test]
fn support_recovery_evidence_unavailable_status_preserves_reason() {
    // Arrange
    let temp = TestDirectory::new("recovery-unavailable");
    let mut status = phase72_status();
    status.recovery_evidence =
        FieldAvailability::unavailable("status recovery evidence probe disabled");
    let bundle = phase77_support_bundle_with_status(temp.path(), status);

    // Act
    let serialized = serde_json::to_value(&bundle).expect("support bundle json");
    let markdown = render::render_support_markdown(&bundle);

    // Assert
    assert_eq!(
        serialized["recovery_evidence"]["state"],
        json!("unavailable")
    );
    assert_eq!(
        serialized["recovery_evidence"]["maybe_unavailable_reason"],
        json!("status recovery evidence probe disabled")
    );
    assert!(markdown.contains("Status: Unavailable: status recovery evidence probe disabled"));
}

#[test]
fn support_recovery_evidence_full_sync_prefers_top_level_status_evidence() {
    // Arrange
    let status = phase77_status_with_available_recovery();

    // Act
    let evidence = derive_full_sync_evidence(&status, &missing_live_smoke());

    // Assert
    assert_eq!(evidence.recovery.state, EvidenceState::Available);
    assert_eq!(
        evidence.recovery.summary.as_deref(),
        Some(
            "category=storage_lock_contention cause=stale_lock_evidence action_class=read_only_inspection next_action=Inspect the datadir read-only and avoid deleting lock artifacts automatically."
        )
    );
}

#[test]
fn support_phase78_progress_guarantee_json_projects_shared_status() {
    // Arrange
    let mut status = phase72_status();
    apply_phase78_available_sync_fields(&mut status.sync);

    // Act
    let evidence = derive_full_sync_evidence(&status, &missing_live_smoke());
    let serialized = serde_json::to_value(&evidence).expect("evidence json");

    // Assert
    assert_eq!(
        serialized["progress_guarantee"]["summary"],
        json!(
            "credit=kind=validated_durable_active_chain height=840004 source_unix_seconds=1717000020 rejected_activity_count=1 last_useful_work=kind=current_at_best_known_tip height=840004 source_unix_seconds=1717000025 rejected_activity_count=0 expected_window=seconds=300 retry_backoff_seconds=30 max_sync_rounds=8 threshold=state=within_window seconds=300 elapsed_seconds=12"
        )
    );
    assert_eq!(
        serialized["stall_diagnosis"]["summary"],
        json!(
            "stalled_subsystem=at_tip_waiting confidence=high basis=stay_current,current_tip next_action=No operator action required."
        )
    );
}

#[test]
fn support_phase78_progress_guarantee_markdown_renders_operator_fields() {
    // Arrange
    let temp = TestDirectory::new("phase78-progress-markdown");
    let mut status = phase72_status();
    apply_phase78_available_sync_fields(&mut status.sync);
    let bundle = phase77_support_bundle_with_status(temp.path(), status);

    // Act
    let markdown = render::render_support_markdown(&bundle);

    // Assert
    for expected in [
        "- Progress guarantee: credit=kind=validated_durable_active_chain",
        "last_useful_work=kind=current_at_best_known_tip",
        "expected_window=seconds=300",
        "threshold=state=within_window",
        "- Stall diagnosis: stalled_subsystem=at_tip_waiting confidence=high",
        "next_action=No operator action required.",
    ] {
        assert!(markdown.contains(expected), "missing {expected}");
    }
}

#[test]
fn support_phase78_progress_guarantee_excludes_raw_status_body() {
    // Arrange
    let temp = TestDirectory::new("phase78-progress-redaction");
    let mut status = phase72_status();
    apply_phase78_available_sync_fields(&mut status.sync);
    status.sync.stall_diagnosis = FieldAvailability::available(StallDiagnosisEvidence {
        stalled_subsystem: StalledSubsystem::StorageOrResourcePressure,
        confidence: StallDiagnosisConfidence::High,
        evidence_basis: vec!["compact evidence only".to_string()],
        next_action: "Inspect bounded resource evidence.".to_string(),
        maybe_no_progress_diagnosis: Some(NoProgressDiagnosis::StorageOrResourceBlocked),
        maybe_recovery_category: Some(SyncRecoveryCategory::ResourceExhaustion),
        maybe_latest_stop_reason_label: Some("resource_stop".to_string()),
        source_unix_seconds: 1_717_000_032,
    });
    let bundle = phase77_support_bundle_with_status(temp.path(), status);

    // Act
    let json_text = serde_json::to_string_pretty(&bundle).expect("support json");
    let markdown = render::render_support_markdown(&bundle);

    // Assert
    for rendered in [&json_text, &markdown] {
        for forbidden in [
            "raw status snapshot phase78-secret",
            "raw live-smoke input phase78-secret",
            "raw daemon log phase78-secret",
            "credential phase78-secret",
        ] {
            assert_absent(rendered, forbidden);
        }
    }
}

#[test]
fn inbound_support_json_projects_shared_status_evidence_with_redacted_endpoints() {
    // Arrange
    let temp = TestDirectory::new("inbound-support-json");
    let status = phase90_status_with_available_inbound();
    let bundle = phase77_support_bundle_with_status(temp.path(), status);

    // Act
    let serialized = serde_json::to_value(&bundle).expect("support bundle json");
    let json_text = serde_json::to_string_pretty(&bundle).expect("support json");
    let inbound = &serialized["status"]["peers"]["inbound"]["value"];

    // Assert
    assert_eq!(
        serialized["status"]["peers"]["inbound"]["state"],
        json!("available")
    );
    assert_eq!(inbound["listener_state"], json!("listening"));
    assert_eq!(inbound["preflight_reason"], json!("ready"));
    assert_eq!(inbound["admitted_inbound_peers"], json!(3));
    assert_eq!(inbound["rejected_inbound_peers"], json!(4));
    assert_eq!(inbound["duplicate_rejects"], json!(1));
    assert_eq!(inbound["self_connection_rejects"], json!(1));
    assert_eq!(inbound["cap_rejects"], json!(1));
    assert_eq!(inbound["reserved_slot_rejects"], json!(1));
    assert_eq!(
        inbound["bound_endpoints"],
        json!([
            "1 loopback endpoint redacted",
            "2 non-loopback endpoints redacted",
            "1 wildcard endpoint redacted"
        ])
    );
    for forbidden in phase90_raw_inbound_endpoints() {
        assert_absent(&json_text, forbidden);
    }
}

#[test]
fn inbound_support_markdown_renders_bounded_admission_labels_and_next_action() {
    // Arrange
    let temp = TestDirectory::new("inbound-support-markdown");
    let status = phase90_status_with_available_inbound();
    let bundle = phase77_support_bundle_with_status(temp.path(), status);

    // Act
    let markdown = render::render_support_markdown(&bundle);

    // Assert
    for expected in [
        "## Inbound Serving",
        "listener_state: listening",
        "preflight_reason: ready",
        "bound_endpoints: 1 loopback endpoint redacted, 2 non-loopback endpoints redacted, 1 wildcard endpoint redacted",
        "admitted_inbound_peers: 3",
        "rejected_inbound_peers: 4",
        "handshake.awaiting_version: 1",
        "handshake.awaiting_verack: 2",
        "handshake.established: 3",
        "handshake.disconnected: 4",
        "duplicate_rejects: 1",
        "self_connection_rejects: 1",
        "cap_rejects: 1",
        "reserved_slot_rejects: 1",
        "latest_admission_event: outcome=rejected reason=cap_reject slot_class=ordinary message=inbound cap reached",
        "Next action: Review configured inbound caps and reserved slots before increasing listener exposure.",
    ] {
        assert!(markdown.contains(expected), "missing {expected}");
    }
    for forbidden in phase90_raw_inbound_endpoints() {
        assert_absent(&markdown, forbidden);
    }
}

#[test]
fn inbound_support_markdown_renders_permission_labels_and_inactive_relay_guidance() {
    // Arrange
    let temp = TestDirectory::new("inbound-support-permissions");
    let status = phase91_status_with_permissioned_inbound();
    let bundle = phase77_support_bundle_with_status(temp.path(), status);

    // Act
    let markdown = render::render_support_markdown(&bundle);

    // Assert
    for expected in [
        "permission_class: protected_inbound",
        "permissioned_inbound_peers: 2",
        "protected_inbound_peers: 1",
        "active_permission_effects: admission_protected, download_serving_policy_input",
        "inactive_permission_effects: inactive_relay, inactive_mempool",
        "latest_permission_decision: outcome=admitted reason=admitted permission_class=protected_inbound active_permission_effects=admission_protected inactive_permission_effects=inactive_relay message=inbound permission decision admitted as protected_inbound",
        "Next action: Relay, mempool, bloom, and blockfilter permissions are recorded as inactive Phase 91 evidence; do not treat them as relay support.",
    ] {
        assert!(markdown.contains(expected), "missing {expected}");
    }
}

#[test]
fn inbound_support_markdown_renders_phase92_address_boundary_evidence() {
    // Arrange
    let temp = TestDirectory::new("inbound-support-address-boundary");
    let status = phase92_status_with_address_boundary_evidence();
    let bundle = phase77_support_bundle_with_status(temp.path(), status);

    // Act
    let markdown = render::render_support_markdown(&bundle);

    // Assert
    for expected in [
        "## Inbound Address Boundary Evidence",
        "Local advertisement candidates: 1",
        "source=source_local_listener",
        "routability=publicly_routable",
        "persistence_eligible=true",
        "Suppressed advertisements: 2",
        "not_publicly_routable",
        "local evidence only",
        "Bounded getaddr responses served: 3",
        "Bounded getaddr requests suppressed: 2",
        "Learned address entries: 5",
        "Learned address rejections: 1",
        "Latest address decision: outcome=suppressed reason=already_served label=getaddr_suppressed source=source_inbound_addr message=bounded getaddr already served",
        "Next action: Treat Phase 92 as bounded local advertisement and direct getaddr evidence only; peer discovery, unsolicited address relay, DNS seed discovery, UPnP/NAT-PMP discovery, and public-network readiness remain outside this surface.",
    ] {
        assert!(markdown.contains(expected), "missing {expected}");
    }
    for forbidden in [
        "full address relay support",
        "peer discovery support",
        "addr gossip support",
    ] {
        assert_absent(&markdown, forbidden);
    }
}

#[test]
fn inbound_support_markdown_renders_phase93_peer_policy_evidence() {
    // Arrange
    let temp = TestDirectory::new("inbound-support-peer-policy");
    let status = phase93_status_with_peer_policy_evidence();
    let bundle = phase77_support_bundle_with_status(temp.path(), status);

    // Act
    let markdown = render::render_support_markdown(&bundle);

    // Assert
    for expected in [
        "## Inbound Peer Policy Evidence",
        "Eviction candidates evaluated: 2",
        "Disconnects requested: 1",
        "Discouraged peers: 1",
        "Active bans: 1",
        "Expired bans: 0",
        "Manual unbans: 0",
        "Misbehavior observations: 2",
        "Protected no-actions: 1",
        "Latest peer policy decision: outcome=selected reason=low_activity label=eviction_candidate_selected source=source_eviction_policy message=peer policy decision selected: low_activity",
        PHASE96_PEER_POLICY_RUNTIME_BRIDGE_NEXT_ACTION,
    ] {
        assert!(markdown.contains(expected), "missing {expected}");
    }
}

#[test]
fn inbound_support_markdown_renders_phase94_resource_governance_evidence() {
    // Arrange
    let temp = TestDirectory::new("inbound-support-resource-governance");
    let status = phase94_status_with_resource_governance_evidence();
    let bundle = phase77_support_bundle_with_status(temp.path(), status);

    // Act
    let markdown = render::render_support_markdown(&bundle);

    // Assert
    for expected in [
        "## Inbound Resource Governance Evidence",
        "Resource pressure events: 8",
        "Read queue pressure events: 7",
        "Write queue pressure events: 6",
        "Request cap events: 5",
        "Payload rejections: 1",
        "Timeout disconnects: 3",
        "Churn rejections: 2",
        "Reconnect suppressions: 4",
        "Latest resource governance decision: outcome=rejected reason=invalid_checksum label=payload_rejected source=source_inbound_resource_governance message=bounded payload rejected next_action=payload_rejected",
        "Next action: Treat Phase 94 as bounded inbound resource-governance evidence only; inspect resource labels before raising listener exposure, queue caps, request caps, or timeout thresholds.",
    ] {
        assert!(markdown.contains(expected), "missing {expected}");
    }
}
