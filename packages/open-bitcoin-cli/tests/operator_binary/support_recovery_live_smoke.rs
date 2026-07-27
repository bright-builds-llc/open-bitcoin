// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

use super::*;

#[test]
fn open_bitcoin_support_bundle_summarizes_phase61_resource_recovery_evidence() {
    // Arrange
    let sandbox = TestSandbox::new("support-live-smoke-v2");
    let data_dir = sandbox.child("open-data");
    let output_dir = sandbox.child("support");
    let report_path = sandbox.child("live-smoke.json");
    let cookie_like_text = "__cookie__:live-smoke-secret";
    let wallet_like_text = "seed phrase live-smoke-secret";
    fs::create_dir_all(&data_dir).expect("open datadir");
    fs::write(
        &report_path,
        json!({
            "schema_version": 2,
            "result": {
                "status": "no_progress",
                "progressDetected": false,
                "maybeNoProgressCause": "handshake_failure",
                "nextAction": "Retry after rotating manual peers; rpcpassword=fixture-secret",
                "headerDelta": 0,
                "blockDelta": 0,
                "firstHeaderProgress": {
                    "observedAtUnixSeconds": 1_780_600_100,
                    "before": {
                        "headerHeight": 840_000,
                        "downloadedBlockHeight": 830_000,
                        "secret": "raw before header live-smoke-secret"
                    },
                    "after": {
                        "headerHeight": 840_100,
                        "connectedBlockHeight": 830_001,
                        "secret": "raw after header live-smoke-secret"
                    },
                    "maybePeer": "peer-accepted-headers",
                    "maybeSource": "manual_peer",
                    "maybeResolvedEndpoint": "redacted-endpoint",
                    "rawEndpoint": "192.0.2.10:8333"
                },
                "firstBlockProgress": {
                    "kind": "connected",
                    "height": 840_004,
                    "blockHash": "0000000000000000000b10c400000000000000000000000000000000000000",
                    "observedAtUnixSeconds": 1_780_600_120,
                    "before": {
                        "downloadedBlockHeight": 840_005,
                        "connectedBlockHeight": 840_003,
                        "headerHeight": 840_100
                    },
                    "after": {
                        "downloadedBlockHeight": 840_006,
                        "connectedBlockHeight": 840_004,
                        "headerHeight": 840_100
                    },
                    "maybePeer": "peer-accepted-block",
                    "maybeSource": "manual_peer",
                    "maybeResolvedEndpoint": "redacted-endpoint",
                    "rawEndpoint": "192.0.2.10:8333"
                },
                "restartResumeEvidence": {
                    "restartStatus": "completed",
                    "sameDatadir": {
                        "requestedPathMatched": true,
                        "resolvedPathMatched": true,
                        "rawPath": "/tmp/live-smoke-secret"
                    },
                    "duplicateConnectVerdict": "no_duplicate_connect_observed",
                    "maybePostRestartProgressDelta": {
                        "headerDelta": 0,
                        "downloadedBlockDelta": 1,
                        "connectedBlockDelta": 1,
                        "rawDeltaNote": "live-smoke-secret"
                    },
                    "peerOutcomeSummary": {
                        "connected": 1,
                        "failed": 1,
                        "failureCauses": ["handshake_failure"],
                        "handshook": 1,
                        "skipped": 0,
                        "rawEndpoint": "192.0.2.10:8333"
                    },
                    "recoveryDiagnosis": {
                        "category": "storage_lock_contention",
                        "maybeLastError": null,
                        "maybeNoProgressCause": "handshake_failure",
                        "maybePeerFailureReason": "peer did not provide useful blocks",
                        "maybeStorageRecoveryAction": null,
                        "rawCookie": cookie_like_text
                    },
                    "beforeRestart": {
                        "headerHeight": 840_100,
                        "downloadedBlockHeight": 840_005,
                        "connectedBlockHeight": 840_003,
                        "phase": "blocks",
                        "lifecycle": "active",
                        "maybeLastError": null,
                        "maybeLastSuccessfulProgressUnixSeconds": 1_780_600_100,
                        "maybeDownloadedBlockHash": "0000000000000000000b10c500000000000000000000000000000000000000",
                        "maybeConnectedBlockHash": "0000000000000000000b10c300000000000000000000000000000000000000",
                        "snapshots": ["live-smoke-secret"]
                    },
                    "afterRestart": {
                        "headerHeight": 840_100,
                        "downloadedBlockHeight": 840_006,
                        "connectedBlockHeight": 840_004,
                        "phase": "blocks",
                        "lifecycle": "active",
                        "maybeLastError": null,
                        "maybeLastSuccessfulProgressUnixSeconds": 1_780_600_120,
                        "maybeDownloadedBlockHash": "0000000000000000000b10c600000000000000000000000000000000000000",
                        "maybeConnectedBlockHash": "0000000000000000000b10c400000000000000000000000000000000000000",
                        "walletMaterial": wallet_like_text
                    }
                },
                "message": "raw result message should not be copied"
            },
            "final_status": {
                "headerHeight": 840_100,
                "downloadedBlockHeight": 840_006,
                "connectedBlockHeight": 840_004,
                "blockHeight": 840_004,
                "phase": "blocks",
                "lifecycle": "active",
                "outboundPeers": 2,
                "messagesProcessed": 128,
                "recoveryCategory": "invalid_peer_data",
                "resourcePressure": {
                    "blocksInFlight": 4,
                    "maxHeaderRequestsInFlightPerPeer": 2,
                    "maxHeadersPerMessage": 2_000,
                    "maxBlocksInFlightPerPeer": 16,
                    "maxBlocksInFlightTotal": 64,
                    "maxMessagesPerPeer": 4_096,
                    "maxSyncRounds": 16,
                    "outboundPeers": 2,
                    "targetOutboundPeers": 3,
                    "rawEndpoint": "192.0.2.10:8333",
                    "rawCookie": cookie_like_text
                },
                "maybeLastError": null,
                "maybeLastSuccessfulProgressUnixSeconds": 1_780_600_120,
                "recentPeers": [
                    {
                        "peer": "192.0.2.10:8333",
                        "error": "live-smoke-secret"
                    }
                ]
            },
            "daemon": {
                "stderrTail": "raw daemon stderr tail live-smoke-secret",
                "stdoutTail": "raw daemon stdout tail live-smoke-secret"
            },
            "options": {
                "manualPeers": ["192.0.2.10:8333"],
                "rpcpassword": "live-smoke-secret"
            },
            "snapshots": [
                {
                    "phase": "header_sync",
                    "secret": "raw snapshot live-smoke-secret",
                    "cookie": cookie_like_text,
                    "wallet": wallet_like_text
                }
            ],
            "preflight": {
                "checks": [
                    {
                        "name": "fixture",
                        "ok": false,
                        "message": "raw preflight check live-smoke-secret"
                    }
                ]
            },
            "network_preflight": {
                "endpoint_outcomes": [
                    {
                        "address": "192.0.2.10:8333",
                        "maybeError": "endpoint-table-detail live-smoke-secret",
                        "state": "failed"
                    }
                ]
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
    let json_text =
        fs::read_to_string(output_dir.join("support-evidence.json")).expect("support json");
    let markdown =
        fs::read_to_string(output_dir.join("support-evidence.md")).expect("support markdown");
    let decoded: Value = serde_json::from_str(&json_text).expect("support json");
    assert_eq!(decoded["live_smoke"]["state"], "available");
    assert_eq!(decoded["live_smoke"]["summary"]["status"], "no_progress");
    assert_eq!(decoded["live_smoke"]["summary"]["progressDetected"], false);
    assert_eq!(
        decoded["live_smoke"]["summary"]["maybeNoProgressCause"],
        "handshake_failure"
    );
    assert_eq!(decoded["live_smoke"]["summary"]["nextAction"], "[redacted]");
    assert_eq!(decoded["live_smoke"]["summary"]["headerDelta"], 0);
    assert_eq!(decoded["live_smoke"]["summary"]["blockDelta"], 0);
    assert_eq!(
        decoded["live_smoke"]["summary"]["firstHeaderProgress"]["observedAtUnixSeconds"],
        1_780_600_100
    );
    assert_eq!(
        decoded["live_smoke"]["summary"]["firstHeaderProgress"]["before"]["headerHeight"],
        840_000
    );
    assert_eq!(
        decoded["live_smoke"]["summary"]["firstHeaderProgress"]["after"]["headerHeight"],
        840_100
    );
    assert_eq!(
        decoded["live_smoke"]["summary"]["firstBlockProgress"]["kind"],
        "connected"
    );
    assert_eq!(
        decoded["live_smoke"]["summary"]["firstBlockProgress"]["height"],
        840_004
    );
    assert_eq!(
        decoded["live_smoke"]["summary"]["firstBlockProgress"]["before"]["downloadedBlockHeight"],
        840_005
    );
    assert_eq!(
        decoded["live_smoke"]["summary"]["firstBlockProgress"]["after"]["connectedBlockHeight"],
        840_004
    );
    assert_eq!(
        decoded["live_smoke"]["summary"]["restartResumeEvidence"]["restartStatus"],
        "completed"
    );
    assert_eq!(
        decoded["live_smoke"]["summary"]["restartResumeEvidence"]["sameDatadir"]["requestedPathMatched"],
        true
    );
    assert_eq!(
        decoded["live_smoke"]["summary"]["restartResumeEvidence"]["duplicateConnectVerdict"],
        "no_duplicate_connect_observed"
    );
    assert_eq!(
        decoded["live_smoke"]["summary"]["restartResumeEvidence"]["recoveryDiagnosis"]["category"],
        "storage_lock_contention"
    );
    assert_eq!(
        decoded["live_smoke"]["summary"]["restartResumeEvidence"]["afterRestart"]["connectedBlockHeight"],
        840_004
    );
    assert_eq!(
        decoded["live_smoke"]["summary"]["finalStatus"]["headerHeight"],
        840_100
    );
    assert_eq!(
        decoded["live_smoke"]["summary"]["finalStatus"]["downloadedBlockHeight"],
        840_006
    );
    assert_eq!(
        decoded["live_smoke"]["summary"]["finalStatus"]["connectedBlockHeight"],
        840_004
    );
    assert_eq!(
        decoded["live_smoke"]["summary"]["finalStatus"]["recoveryCategory"],
        "invalid_peer_data"
    );
    assert_eq!(
        decoded["live_smoke"]["summary"]["finalStatus"]["resourcePressure"]["maxBlocksInFlightTotal"],
        64
    );
    assert_eq!(
        decoded["live_smoke"]["summary"]["finalStatus"]["resourcePressure"]["targetOutboundPeers"],
        3
    );
    assert_eq!(decoded["store_health"]["state"], "unavailable");
    assert!(decoded["status"]["service"].is_object());
    let restart_resume = &decoded["status"]["service"]["restart_resume"];
    if restart_resume["state"] == json!("available") {
        assert_eq!(
            restart_resume["value"]["prior_shutdown"]["state"],
            "unavailable"
        );
        assert_eq!(
            restart_resume["value"]["stale_inflight"]["state"],
            "unavailable"
        );
        assert_eq!(
            restart_resume["value"]["next_action"]["state"],
            "unavailable"
        );
    } else {
        assert_eq!(restart_resume["state"], "unavailable");
    }
    assert_eq!(decoded["status"]["logs"]["path"]["state"], "unavailable");
    assert_eq!(
        decoded["status"]["metrics"]["availability"]["state"],
        "unavailable"
    );
    assert!(markdown.contains("Progress detected"));
    assert!(markdown.contains("No-progress cause"));
    assert!(markdown.contains("Header delta"));
    assert!(markdown.contains("Block delta"));
    assert!(markdown.contains("First header progress"));
    assert!(markdown.contains("First block progress"));
    assert!(markdown.contains("Restart/resume evidence"));
    assert!(markdown.contains("Recovery diagnosis"));
    assert!(markdown.contains("Recovery category"));
    assert!(markdown.contains("Resource pressure"));
    assert!(markdown.contains("Final status"));
    assert!(markdown.contains("Service lifecycle"));
    assert!(markdown.contains("Service restart/resume"));
    assert!(markdown.contains("Logs path"));
    assert!(markdown.contains("Metrics availability"));
    assert!(markdown.contains("Metrics samples"));
    assert!(markdown.contains("handshake_failure"));
    for rendered in [&json_text, &markdown] {
        assert_absent(rendered, "live-smoke-secret");
        assert_absent(rendered, "fixture-secret");
        assert_absent(rendered, "stdoutTail");
        assert_absent(rendered, "stderrTail");
        assert_absent(rendered, "endpoint_outcomes");
        assert_absent(rendered, "manualPeers");
        assert_absent(rendered, "rpcpassword");
        assert_absent(rendered, "raw daemon stderr tail");
        assert_absent(rendered, "raw daemon stdout tail");
        assert_absent(rendered, "raw snapshot");
        assert_absent(rendered, "raw preflight check");
        assert_absent(rendered, "endpoint-table-detail");
        assert_absent(rendered, "192.0.2.10:8333");
        assert_absent(rendered, cookie_like_text);
        assert_absent(rendered, wallet_like_text);
        assert_absent(rendered, "snapshots");
    }
}

#[test]
fn open_bitcoin_support_bundle_preserves_top_level_live_smoke_fallback() {
    // Arrange
    let sandbox = TestSandbox::new("support-live-smoke-fallback");
    let data_dir = sandbox.child("open-data");
    let output_dir = sandbox.child("support");
    let report_path = sandbox.child("live-smoke.json");
    fs::create_dir_all(&data_dir).expect("open datadir");
    fs::write(
        &report_path,
        json!({
            "status": "timeout",
            "maybeNoProgressCause": "handshake_failure",
            "nextAction": "Retry with --manual-peer=HOST[:PORT]",
            "manualPeers": ["192.0.2.10:8333"],
            "manual_peers": ["198.51.100.20:8333"],
            "daemonStderrTail": "live-smoke-secret"
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
    let json_text =
        fs::read_to_string(output_dir.join("support-evidence.json")).expect("support json");
    let markdown =
        fs::read_to_string(output_dir.join("support-evidence.md")).expect("support markdown");
    let decoded: Value = serde_json::from_str(&json_text).expect("support json");
    assert_eq!(decoded["live_smoke"]["state"], "available");
    assert_eq!(decoded["live_smoke"]["summary"]["status"], "timeout");
    assert_eq!(
        decoded["live_smoke"]["summary"]["maybeNoProgressCause"],
        "handshake_failure"
    );
    assert_eq!(
        decoded["live_smoke"]["summary"]["nextAction"],
        "Retry with --manual-peer=HOST[:PORT]"
    );
    assert!(markdown.contains("handshake_failure"));
    assert_absent(&json_text, "live-smoke-secret");
    assert_absent(&markdown, "live-smoke-secret");
    for rendered in [&json_text, &markdown] {
        assert_absent(rendered, "manualPeers");
        assert_absent(rendered, "manual_peers");
        assert_absent(rendered, "192.0.2.10:8333");
        assert_absent(rendered, "198.51.100.20:8333");
    }
}

#[test]
fn open_bitcoin_support_bundle_keeps_missing_live_smoke_report_unavailable() {
    // Arrange
    let sandbox = TestSandbox::new("support-live-smoke-missing");
    let data_dir = sandbox.child("open-data");
    let output_dir = sandbox.child("support");
    let report_path = sandbox.child("missing-live-smoke.json");
    fs::create_dir_all(&data_dir).expect("open datadir");

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
    let json_text =
        fs::read_to_string(output_dir.join("support-evidence.json")).expect("support json");
    let decoded: Value = serde_json::from_str(&json_text).expect("support json");
    assert_eq!(decoded["live_smoke"]["state"], "unavailable");
    assert!(
        decoded["live_smoke"]["reason"]
            .as_str()
            .expect("live smoke reason")
            .contains("does not exist")
    );
}
