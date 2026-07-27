// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

use super::*;

#[test]
fn open_bitcoin_support_bundle_includes_phase75_soak_summary() {
    // Arrange
    let sandbox = TestSandbox::new("support-phase75-soak");
    let data_dir = sandbox.child("open-data");
    let output_dir = sandbox.child("support");
    fs::create_dir_all(&data_dir).expect("open datadir");
    fs::write(
        data_dir.join("bitcoin.conf"),
        "regtest=1\nrpcuser=alice\nrpcpassword=phase75-rpc-secret\n",
    )
    .expect("bitcoin.conf");
    fs::write(
        data_dir.join(".cookie"),
        "__cookie__:phase75-cookie-secret\n",
    )
    .expect("cookie");
    seed_phase72_runtime_metadata(&data_dir, false);
    let start = run_open_bitcoin_vec(&sandbox, soak_start_args(&data_dir, "soak-1700000000-0001"));
    assert_success(&start);
    let run_dir = data_dir.join("soak/runs/soak-1700000000-0001");
    let events_path = run_dir.join("events.jsonl");
    let report_json_path = run_dir.join("report.json");
    let report_markdown_path = run_dir.join("report.md");
    fs::write(
        &report_json_path,
        r#"{"raw reports phase75-secret":"wallet material phase75-secret"}"#,
    )
    .expect("overwrite report json with raw report sentinel");
    fs::write(
        &report_markdown_path,
        "raw daemon logs phase75-secret\nRPC credentials phase75-secret\nunbounded peer tables phase75-secret\n",
    )
    .expect("overwrite report markdown with raw report sentinel");

    // Act
    let output = run_open_bitcoin(
        &sandbox,
        [
            "--network",
            "regtest",
            "--datadir",
            data_dir.to_str().expect("datadir"),
            "support",
            "bundle",
            "--output-dir",
            output_dir.to_str().expect("output dir"),
        ],
    );

    // Assert
    assert_success(&output);
    let json_text =
        fs::read_to_string(output_dir.join("support-evidence.json")).expect("support json");
    let markdown =
        fs::read_to_string(output_dir.join("support-evidence.md")).expect("support markdown");
    let decoded: Value = serde_json::from_str(&json_text).expect("support json");
    assert_eq!(decoded["soak_evidence"]["state"], "available");
    assert_eq!(
        decoded["soak_evidence"]["maybe_run_id"],
        json!("soak-1700000000-0001")
    );
    assert_eq!(
        decoded["soak_evidence"]["maybe_final_outcome"],
        json!("clean_completion")
    );
    let latest_sequence = decoded["soak_evidence"]["maybe_latest_sequence"]
        .as_u64()
        .expect("latest sequence");
    assert!(latest_sequence > 0);
    assert_eq!(
        decoded["soak_evidence"]["maybe_source_ledger_path"],
        json!(events_path.display().to_string())
    );
    assert_eq!(
        decoded["soak_evidence"]["maybe_json_report_path"],
        json!(report_json_path.display().to_string())
    );
    assert_eq!(
        decoded["soak_evidence"]["maybe_markdown_report_path"],
        json!(report_markdown_path.display().to_string())
    );
    for expected in [
        "## Soak Evidence",
        "State: available",
        "Run: soak-1700000000-0001",
        "Final outcome: clean_completion",
        "Source ledger:",
        "JSON report:",
        "Markdown report:",
    ] {
        assert!(markdown.contains(expected), "missing {expected}");
    }
    assert!(markdown.contains(&format!("Latest sequence: {latest_sequence}")));
    for rendered in [&json_text, &markdown] {
        for forbidden in [
            "raw ledger",
            "raw daemon logs",
            "raw reports",
            "wallet material",
            "RPC credentials",
            "unbounded peer tables",
            "phase75-rpc-secret",
            "phase75-cookie-secret",
            "phase75-secret",
            "\"kind\":\"started\"",
            "\"kind\":\"checkpoint\"",
        ] {
            assert_absent(rendered, forbidden);
        }
    }
}

#[test]
fn open_bitcoin_support_bundle_includes_phase72_full_sync_evidence_and_typed_verdict() {
    // Arrange
    let sandbox = TestSandbox::new("support-phase72-evidence");
    let data_dir = sandbox.child("open-data");
    let unavailable_data_dir = sandbox.child("open-data-unavailable");
    let output_dir = sandbox.child("support");
    let unavailable_output_dir = sandbox.child("support-unavailable");
    let report_path = sandbox.child("phase72-live-smoke.json");
    let server = FakeRpcServer::start();
    seed_phase72_runtime_metadata(&data_dir, false);
    seed_phase72_runtime_metadata(&unavailable_data_dir, true);
    write_rpc_conf(&data_dir, server.address.port());
    write_rpc_conf(&unavailable_data_dir, server.address.port());
    fs::write(
        &report_path,
        json!({
            "schema_version": 2,
            "result": {
                "status": "progress",
                "rawPeerTable": "rawPeerTable phase72-live-smoke-secret",
                "walletMaterial": "seed phrase phase72-live-smoke-secret"
            },
            "final_status": {
                "validatedActiveChainHeight": 840_004,
                "maybeValidatedActiveChainHash": "1111111111111111111111111111111111111111111111111111111111111111",
                "maybeValidatedActiveChainWork": "840005",
                "bestKnownTip": {
                    "height": 840_004,
                    "blockHash": "1111111111111111111111111111111111111111111111111111111111111111",
                    "work": "840005",
                    "freshness": "fresh",
                    "rawPeerTable": "raw phase72-live-smoke-secret"
                },
                "stayCurrent": "current_at_best_known_tip",
                "stayCurrentNextAction": "Continue monitoring best-known tip freshness.",
                "noProgressDiagnosis": "current_at_best_known_tip",
                "noProgressNextAction": "No operator action required.",
                "latestReorg": {
                    "finalActiveHeight": 840_004,
                    "finalActiveHash": "1111111111111111111111111111111111111111111111111111111111111111",
                    "fullyPersisted": true,
                    "rawLogTail": "raw phase72-live-smoke-secret"
                },
                "reconcileProgress": {
                    "state": "extended_active_chain",
                    "connectedCount": 4,
                    "finalActiveHeight": 840_004,
                    "finalActiveHash": "1111111111111111111111111111111111111111111111111111111111111111"
                },
                "resourcePressure": {
                    "blocksInFlight": 1,
                    "targetOutboundPeers": 4
                },
                "peerContribution": {
                    "connected": 3,
                    "failed": 1,
                    "rawPeerTable": "peer phase72-live-smoke-secret"
                },
                "rpcpassword": "super-secret-password",
                "__cookie__": "super-secret-cookie"
            },
            "daemon": {
                "stdoutTail": "stdoutTail phase72-live-smoke-secret",
                "stderrTail": "stderrTail phase72-live-smoke-secret"
            }
        })
        .to_string(),
    )
    .expect("live smoke report");

    // Act
    let output = run_open_bitcoin(
        &sandbox,
        [
            "--network",
            "regtest",
            "--datadir",
            data_dir.to_str().expect("datadir"),
            "support",
            "bundle",
            "--output-dir",
            output_dir.to_str().expect("output dir"),
            "--include-live-smoke-report",
            report_path.to_str().expect("report path"),
        ],
    );

    // Assert
    assert_success(&output);
    let stdout = String::from_utf8(output.stdout).expect("stdout utf8");
    let json_text =
        fs::read_to_string(output_dir.join("support-evidence.json")).expect("support json");
    let markdown =
        fs::read_to_string(output_dir.join("support-evidence.md")).expect("support markdown");
    let decoded: Value = serde_json::from_str(&json_text).expect("support json");
    assert_eq!(
        decoded["full_sync_evidence"]["verdict"]["label"],
        json!("sync_to_tip_proven")
    );
    assert_eq!(
        decoded["full_sync_evidence"]["connected_active_chain"]["height"],
        json!(840_004)
    );
    assert_eq!(
        decoded["full_sync_evidence"]["connected_active_chain"]["hash"],
        json!("1111111111111111111111111111111111111111111111111111111111111111")
    );
    assert_eq!(
        decoded["full_sync_evidence"]["connected_active_chain"]["work"],
        json!("840005")
    );
    assert_eq!(
        decoded["full_sync_evidence"]["validated_active_chain"]["height"],
        json!(840_004)
    );
    assert_eq!(
        decoded["full_sync_evidence"]["validated_active_chain"]["hash"],
        json!("1111111111111111111111111111111111111111111111111111111111111111")
    );
    assert_eq!(
        decoded["full_sync_evidence"]["validated_active_chain"]["work"],
        json!("840005")
    );
    for expected in [
        "## Full Sync Evidence",
        "Evidence verdict: sync_to_tip_proven",
        "validated_active_chain_matches_best_known_tip",
        "Connected active chain: height=840004 hash=1111111111111111111111111111111111111111111111111111111111111111 work=840005",
        "Validated active chain: height=840004 hash=1111111111111111111111111111111111111111111111111111111111111111 work=840005",
        "Stay-current window:",
        "Peer contribution:",
        "No-progress or reorg events:",
        "Resource pressure:",
        "Recovery:",
    ] {
        assert!(markdown.contains(expected), "missing {expected}");
    }
    for rendered in [&stdout, &json_text, &markdown] {
        for forbidden in [
            "super-secret-password",
            "super-secret-cookie",
            "stdoutTail",
            "stderrTail",
            "rawPeerTable",
            "rawLogTail",
            "seed phrase",
            "walletMaterial",
            "phase72-live-smoke-secret",
        ] {
            assert_absent(rendered, forbidden);
        }
    }

    // Act
    let unavailable_output = run_open_bitcoin(
        &sandbox,
        [
            "--network",
            "regtest",
            "--datadir",
            unavailable_data_dir.to_str().expect("datadir"),
            "support",
            "bundle",
            "--output-dir",
            unavailable_output_dir.to_str().expect("output dir"),
        ],
    );

    // Assert
    assert_success(&unavailable_output);
    let unavailable_json_text =
        fs::read_to_string(unavailable_output_dir.join("support-evidence.json"))
            .expect("support json");
    let unavailable_markdown =
        fs::read_to_string(unavailable_output_dir.join("support-evidence.md"))
            .expect("support markdown");
    let unavailable: Value = serde_json::from_str(&unavailable_json_text).expect("support json");
    assert_eq!(
        unavailable["full_sync_evidence"]["connected_active_chain"]["maybe_unavailable_reason"],
        json!("connected active-chain work unavailable")
    );
    assert_eq!(
        unavailable["full_sync_evidence"]["validated_active_chain"]["maybe_unavailable_reason"],
        json!("validated active-chain work unavailable")
    );
    assert!(unavailable_markdown.contains("Unavailable: connected active-chain work unavailable"));
    assert!(unavailable_markdown.contains("Unavailable: validated active-chain work unavailable"));
}
