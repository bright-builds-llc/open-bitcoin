use super::*;

#[test]
fn phase79_support_forensics_json_includes_sidecar_contract() {
    // Arrange
    let temp = TestDirectory::new("phase79-json-contract");
    seed_phase75_soak_run(
        temp.path(),
        "soak-1781485562-0081",
        SoakOutcomeLabel::CleanCompletion,
    );
    let bundle = phase75_support_bundle_for_test(temp.path());

    // Act
    let serialized = serde_json::to_value(&bundle).expect("support bundle json");

    // Assert
    for field in [
        "timeline",
        "checkpoint_chain",
        "narrative",
        "source",
        "redaction",
    ] {
        assert!(
            !serialized["support_forensics"][field].is_null(),
            "missing {field}"
        );
    }
    assert!(
        !serialized["support_forensics"]["narrative"]["evidence_basis"]
            .as_array()
            .expect("evidence basis")
            .is_empty()
    );
}

#[test]
fn phase79_support_forensics_json_excludes_sensitive_seed_material() {
    // Arrange
    let temp = TestDirectory::new("phase79-sensitive-json");
    seed_phase79_sensitive_soak_run(
        temp.path(),
        "soak-1781485562-0082",
        SoakOutcomeLabel::ResourceStop,
    );
    let bundle = phase75_support_bundle_for_test(temp.path());

    // Act
    let json_text = serde_json::to_string_pretty(&bundle).expect("support json");

    // Assert
    for forbidden in phase79_sensitive_literals() {
        assert_absent(&json_text, forbidden);
    }
}

#[test]
fn phase79_support_forensics_markdown_renders_timeline_chain_and_failure_narrative() {
    // Arrange
    let temp = TestDirectory::new("phase79-markdown");
    seed_phase75_soak_run(
        temp.path(),
        "soak-1781485562-0083",
        SoakOutcomeLabel::CleanCompletion,
    );
    let bundle = phase75_support_bundle_for_test(temp.path());

    // Act
    let markdown = render::render_support_markdown(&bundle);

    // Assert
    for expected in [
        "## Forensic Timeline",
        "## Checkpoint Chain",
        "## Failure Narrative",
        "Verdict: soak_stable",
        "Likely cause: soak completed with validated progress evidence",
        "Evidence basis: outcome=clean_completion",
        "Next action: Archive the support bundle as soak stability evidence.",
        "Confidence: high",
        "Algorithm: sha256-json-v1",
        "Event count: 4",
        "Missing sequence count: 0",
        "Truncated: false",
        "Sequence 1: kind=run_start recorded_at=1781485562",
    ] {
        assert!(markdown.contains(expected), "missing {expected}");
    }
}

#[test]
fn phase79_support_forensics_cross_surface_agreement_uses_shared_status_contract() {
    // Arrange
    let temp = TestDirectory::new("phase79-cross-surface");
    let status = phase79_shared_contract_status();
    seed_soak_run_with_checkpoint(
        temp.path(),
        "soak-1781485562-0084",
        SoakOutcomeLabel::RecoveryStop,
        phase79_shared_checkpoint_status(),
    );
    let bundle = phase77_support_bundle_with_status(temp.path(), status);

    // Act
    let serialized = serde_json::to_value(&bundle).expect("support bundle json");
    let forensics_text =
        serde_json::to_string(&serialized["support_forensics"]).expect("forensics json");
    let markdown = render::render_support_markdown(&bundle);

    // Assert
    assert_eq!(
        serialized["support_forensics"]["source"]["event_count"],
        json!(4)
    );
    assert_eq!(
        serialized["support_forensics"]["narrative"]["evidence_basis"],
        json!(["recovery=stale_lock_evidence"])
    );
    assert!(serialized["resource_bound_evidence"]["maybe_projected_bundle_size_bytes"].is_number());
    assert!(serialized["support_forensics"]["maybe_projected_bundle_size_bytes"].is_null());
    for topic in [
        "RPC cookie contents",
        "RPC password and RPC auth values",
        "wallet private material and raw wallet files",
        "raw unbounded log contents",
    ] {
        assert!(
            serialized["support_forensics"]["redaction"]["omitted"]
                .as_array()
                .expect("redaction omitted")
                .iter()
                .any(|value| value.as_str() == Some(topic)),
            "missing redaction topic {topic}"
        );
    }
    for expected in [
        "recovery=stale_lock_evidence",
        "resource_bound=warning",
        "resource_bound_label=support_bundle=warning",
        "progress_credit=validated_durable_active_chain",
        "last_peer_contribution=peer=peer-79 kind=headers_and_blocks messages=9 headers=4 blocks=2 failure=unavailable",
        "stall=storage_or_resource_pressure",
        "stall_confidence=medium",
    ] {
        assert!(
            forensics_text.contains(expected),
            "missing JSON label {expected}"
        );
        assert!(
            markdown.contains(expected),
            "missing Markdown label {expected}"
        );
    }
}

#[test]
fn phase79_support_forensics_markdown_and_json_exclude_forbidden_sensitive_material() {
    // Arrange
    let temp = TestDirectory::new("phase79-sensitive-render");
    seed_phase79_sensitive_soak_run(
        temp.path(),
        "soak-1781485562-0085",
        SoakOutcomeLabel::ResourceStop,
    );
    let bundle = phase75_support_bundle_for_test(temp.path());

    // Act
    let json_text = serde_json::to_string_pretty(&bundle).expect("support json");
    let markdown = render::render_support_markdown(&bundle);

    // Assert
    for rendered in [&json_text, &markdown] {
        for forbidden in phase79_sensitive_literals() {
            assert_absent(rendered, forbidden);
        }
    }
}

#[test]
fn phase75_soak_support_summary_excludes_raw_local_evidence() {
    // Arrange
    let temp = TestDirectory::new("soak-redaction");
    seed_phase75_soak_run(
        temp.path(),
        "soak-1781485562-0003",
        SoakOutcomeLabel::ResourceStop,
    );
    let bundle = phase75_support_bundle_for_test(temp.path());

    // Act
    let json_text = serde_json::to_string_pretty(&bundle).expect("support json");
    let markdown = render::render_support_markdown(&bundle);

    // Assert
    for rendered in [&json_text, &markdown] {
        for forbidden in [
            "raw ledger line phase75-secret",
            "raw daemon logs phase75-secret",
            "raw reports phase75-secret",
            "wallet material phase75-secret",
            "RPC credentials phase75-secret",
            "unbounded peer tables phase75-secret",
            "\"kind\":\"started\"",
            "\"kind\":\"checkpoint\"",
        ] {
            assert_absent(rendered, forbidden);
        }
    }
}

#[test]
fn support_recovery_evidence_json_projects_shared_status_evidence() {
    // Arrange
    let temp = TestDirectory::new("recovery-json");
    let status = phase77_status_with_available_recovery();
    let bundle = phase77_support_bundle_with_status(temp.path(), status);

    // Act
    let serialized = serde_json::to_value(&bundle).expect("support bundle json");

    // Assert
    assert_eq!(serialized["recovery_evidence"]["state"], json!("available"));
    assert_eq!(
        serialized["recovery_evidence"]["category"],
        json!("storage_lock_contention")
    );
    assert_eq!(
        serialized["recovery_evidence"]["cause"],
        json!("stale_lock_evidence")
    );
    assert_eq!(
        serialized["recovery_evidence"]["action_class"],
        json!("read_only_inspection")
    );
    assert_eq!(
        serialized["recovery_evidence"]["evidence_basis"],
        json!(["lock_probe"])
    );
    assert_eq!(
        serialized["recovery_evidence"]["next_action"],
        json!("Inspect the datadir read-only and avoid deleting lock artifacts automatically.")
    );
    assert_eq!(
        serialized["recovery_evidence"]["maybe_unavailable_reason"],
        json!(null)
    );
    assert_eq!(
        serialized["recovery_evidence"]["source"],
        json!("status.recovery_evidence")
    );
}

#[test]
fn support_recovery_evidence_markdown_renders_operator_fields() {
    // Arrange
    let temp = TestDirectory::new("recovery-markdown");
    let status = phase77_status_with_available_recovery();
    let bundle = phase77_support_bundle_with_status(temp.path(), status);

    // Act
    let markdown = render::render_support_markdown(&bundle);

    // Assert
    for expected in [
        "## Recovery Evidence",
        "Category: storage_lock_contention",
        "Cause: stale_lock_evidence",
        "Action class: read_only_inspection",
        "Next action: Inspect the datadir read-only and avoid deleting lock artifacts automatically.",
    ] {
        assert!(markdown.contains(expected), "missing {expected}");
    }
}

#[test]
fn support_recovery_evidence_collection_preserves_probe_only_store_health() {
    // Arrange
    let status = phase72_status();

    // Act
    let health = collect_store_health(&status);

    // Assert
    assert_eq!(health.state, EvidenceState::Unavailable);
    assert_eq!(
        health.runtime_metadata.availability.reason,
        Some(
            "runtime metadata unavailable: probe-only support bundle does not open Fjall stores"
                .to_string()
        )
    );
    assert_eq!(
        health.metrics_history.availability.reason,
        Some(
            "metrics history unavailable: probe-only support bundle does not open Fjall stores"
                .to_string()
        )
    );
}

#[test]
fn support_bundle_preserves_retained_inbound_metric_samples() {
    // Arrange
    let mut status = phase72_status();
    status.metrics = MetricsStatus::available_with_samples(
        MetricRetentionPolicy::default(),
        vec![MetricSample::new(
            MetricKind::InboundResourcePressureActiveCount,
            16.0,
            1_777_225_022,
        )],
    );

    // Act
    let health = collect_store_health(&status);
    let serialized = serde_json::to_value(&health).expect("store health json");
    let sample = &serialized["metrics_history"]["status"]["samples"][0];

    // Assert
    assert_eq!(health.state, EvidenceState::Available);
    assert_eq!(health.metrics_history.samples, 1);
    assert_eq!(sample["kind"], "inbound_resource_pressure_active_count");
    assert_eq!(sample["value"], 16.0);
    assert_eq!(sample["timestamp_unix_seconds"], 1_777_225_022);
    assert!(sample.get("peer_id").is_none());
    assert!(sample.get("endpoint").is_none());
    assert!(sample.get("permission_class").is_none());
}

#[test]
fn support_bundle_renders_relay_and_mempool_evidence_from_shared_projection() {
    // Arrange
    let temp = TestDirectory::new("phase105-relay-support");
    let status = phase105_status_with_relay_evidence();
    let bundle = phase77_support_bundle_with_status(temp.path(), status);

    // Act
    let serialized = serde_json::to_value(&bundle).expect("support bundle json");
    let markdown = render::render_support_markdown(&bundle);

    // Assert
    assert_eq!(
        serialized["status"]["mempool"]["relay"]["outcome_counters"]["state"],
        json!("implemented")
    );
    assert_eq!(
        serialized["status"]["mempool"]["relay"]["outcome_counters"]["value"]["accepted_count"],
        json!(11)
    );
    assert_eq!(
        serialized["status"]["mempool"]["relay"]["recovery_counters"]["value"]["recovered_count"],
        json!(21)
    );
    assert_eq!(
        serialized["status"]["mempool"]["relay"]["recovery_counters"]["value"]["dropped_evicted_count"],
        json!(26)
    );
    assert_eq!(
        serialized["status"]["mempool"]["relay"]["activation"]["value"]["enabled"],
        json!(true)
    );
    assert_eq!(
        serialized["status"]["mempool"]["relay"]["download_eligibility"]["value"]["eligible_peer_count"],
        json!(2)
    );
    assert_eq!(
        serialized["status"]["mempool"]["relay"]["download_eligibility"]["value"]["protected_not_relay_count"],
        json!(1)
    );
    for expected in [
        "## Relay and Mempool Evidence",
        "Mempool: transactions=7",
        "Relay evidence: accepted_count=11 rejected_count=2 orphaned_count=3 requested_count=5 served_count=4 announced_count=13 suppressed_count=8 evicted_count=1 expired_count=6 rebroadcast_deferred_count=9",
        "Relay recovery: recovered_count=21 dropped_confirmed_count=22 dropped_duplicate_count=23 dropped_missing_parent_count=24 dropped_policy_incompatible_count=25 dropped_evicted_count=26",
        "Mempool evidence: Implemented: mempool_admission",
        "Relay local submission: Implemented: local_submission_relay",
        "Relay fanout: Implemented: relay_fanout",
        "Relay serving: Implemented: relay_serving",
        "Rebroadcast: deferred: Deferred: rebroadcast relay evidence not projected",
        "Public relay: Intentionally different: public relay readiness is intentionally not claimed",
        "bounded local status",
        "local troubleshooting/parity-review evidence only",
        "public propagation",
        "compact-block relay",
        "release validator",
        "public-network proof",
        "production-service proof",
        "production full-node readiness proof",
        "production-funds wallet safety proof",
        "authorization for destructive repair",
    ] {
        assert!(markdown.contains(expected), "missing {expected}");
    }
}

#[test]
fn support_bundle_redacts_sensitive_relay_reasons_in_json_and_markdown() {
    // Arrange
    let temp = TestDirectory::new("phase105-relay-redaction");
    let status = phase105_status_with_sensitive_relay_reasons();
    let bundle = phase77_support_bundle_with_status(temp.path(), status);

    // Act
    let serialized = serde_json::to_value(&bundle).expect("support bundle json");
    let json_text = serde_json::to_string_pretty(&bundle).expect("support json");
    let markdown = render::render_support_markdown(&bundle);

    // Assert
    assert_eq!(
        serialized["status"]["mempool"]["relay"]["mempool_admission"]["value"]["reason"],
        json!("redacted_relay_mempool_evidence")
    );
    assert_eq!(
        serialized["status"]["mempool"]["relay"]["activation"]["value"]["reason"],
        json!("redacted_relay_mempool_evidence")
    );
    assert_eq!(
        serialized["status"]["mempool"]["relay"]["download_eligibility"]["value"]["reason"],
        json!("redacted_relay_mempool_evidence")
    );
    assert_eq!(
        serialized["status"]["mempool"]["relay"]["recovery_counters"]["value"]["reason"],
        json!("redacted_relay_mempool_evidence")
    );
    assert!(markdown.contains("redacted_relay_mempool_evidence"));
    for rendered in [&json_text, &markdown] {
        for forbidden in [
            "raw tx hex",
            "020000000001",
            "txid=aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
            "wtxid=bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb",
            "127.0.0.1:",
            "198.51.100.105:8333",
            "peer_id=",
            "permission_string",
            "credential=phase105",
            "secret=phase105",
            "cookie=phase105",
            "dynamic_label",
        ] {
            assert_absent(rendered, forbidden);
        }
    }
}

#[test]
fn support_bundle_renders_block_relay_evidence_from_shared_projection() {
    // Arrange
    let temp = TestDirectory::new("phase116-block-relay-support");
    let status = phase116_status_with_block_relay_evidence();
    let bundle = phase77_support_bundle_with_status(temp.path(), status);

    // Act
    let serialized = serde_json::to_value(&bundle).expect("support bundle json");
    let markdown = render::render_support_markdown(&bundle);

    // Assert
    assert_eq!(
        serialized["status"]["block_relay"]["block_serving"]["activation"]["state"],
        json!("available")
    );
    assert_eq!(
        serialized["status"]["block_relay"]["block_serving"]["activation"]["value"]["block_serving_enabled"],
        json!(true)
    );
    assert_eq!(
        serialized["status"]["block_relay"]["announcement"]["value"]["compact_announced_count"],
        json!(6)
    );
    assert_eq!(
        serialized["status"]["block_relay"]["cleanup"]["value"]["compact_cleanup_count"],
        json!(3)
    );
    for expected in [
        "## Block Relay Evidence",
        "Block relay activation: block_serving_enabled=true compact_relay_enabled=true",
        "Block relay eligibility: eligible_peer_count=2",
        "Compact announcement: compact_announced_count=6",
        "Compact cleanup: compact_cleanup_count=3",
        "bounded local troubleshooting/parity-review evidence only",
        "public block serving by default",
        "production full-node readiness proof",
    ] {
        assert!(markdown.contains(expected), "missing {expected}");
    }
}
