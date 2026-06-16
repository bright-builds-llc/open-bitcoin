// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

use serde_json::json;

use super::summary;

#[test]
fn phase71_live_smoke_summary_is_allowlisted_and_bounded() {
    // Arrange
    let report = json!({
        "result": {
            "status": "progress",
            "rawPeerTable": "raw peer table must not be copied",
            "daemonStdout": "daemon stdout must not be copied",
            "daemonStderr": "daemon stderr must not be copied",
            "rawLogTail": "raw log tail must not be copied",
            "rpcpassword": "rpcpassword=super-secret",
            "cookie": "__cookie__:super-secret",
        },
        "final_status": {
            "headerHeight": 840_200,
            "rawPeerTable": "final raw peer table must not be copied",
            "daemonStdout": "final daemon stdout must not be copied",
            "daemonStderr": "final daemon stderr must not be copied",
            "rawLogTail": "final raw log tail must not be copied",
            "rpcpassword": "final rpcpassword=super-secret",
            "cookie": "final __cookie__:super-secret",
            "resourcePressure": {
                "blocksInFlight": 8,
                "maxHeaderRequestsInFlightPerPeer": 1,
                "maxHeadersPerMessage": 2000,
                "maxBlocksInFlightPerPeer": 16,
                "maxBlocksInFlightTotal": 64,
                "maxMessagesPerPeer": 64,
                "maxSyncRounds": 8,
                "outboundPeers": 2,
                "targetOutboundPeers": 4,
                "rawPeerTable": "nested raw peer table must not be copied",
                "daemonStdout": "nested daemon stdout must not be copied",
                "daemonStderr": "nested daemon stderr must not be copied",
                "rawLogTail": "nested raw log tail must not be copied",
                "rpcpassword": "nested rpcpassword=super-secret",
                "cookie": "nested __cookie__:super-secret",
            }
        },
        "rawPeerTable": "top raw peer table must not be copied",
        "daemonStdout": "top daemon stdout must not be copied",
        "daemonStderr": "top daemon stderr must not be copied",
        "rawLogTail": "top raw log tail must not be copied",
        "rpcpassword": "top rpcpassword=super-secret",
        "cookie": "top __cookie__:super-secret",
    });

    // Act
    let summarized = summary(&report).expect("summary");
    let text = summarized.to_string();

    // Assert
    assert!(text.contains("resourcePressure"));
    assert!(text.contains("blocksInFlight"));
    assert!(text.contains("maxBlocksInFlightTotal"));
    for forbidden in [
        "rawPeerTable",
        "daemonStdout",
        "daemonStderr",
        "rawLogTail",
        "rpcpassword",
        "cookie",
        "raw peer table must not be copied",
        "daemon stdout must not be copied",
        "daemon stderr must not be copied",
        "raw log tail must not be copied",
        "super-secret",
    ] {
        assert!(
            !text.contains(forbidden),
            "summary copied forbidden live-smoke material: {forbidden}"
        );
    }
}

#[test]
fn phase72_live_smoke_summary_preserves_full_sync_evidence_without_raw_report() {
    // Arrange
    let report = json!({
        "schema_version": 2,
        "result": {
            "status": "progress",
            "rawPeerTable": "raw peer table phase72-live-smoke-secret",
            "daemonStdout": "daemon stdout phase72-live-smoke-secret",
            "daemonStderr": "daemon stderr phase72-live-smoke-secret",
            "rawLogTail": "raw log phase72-live-smoke-secret",
            "walletMaterial": "seed phrase phase72-live-smoke-secret",
        },
        "final_status": {
            "headerHeight": 840_004,
            "downloadedBlockHeight": 840_004,
            "connectedBlockHeight": 840_004,
            "validatedActiveChainHeight": 840_004,
            "maybeValidatedActiveChainHeightUnavailableReason": null,
            "maybeValidatedActiveChainHash": "1111111111111111111111111111111111111111111111111111111111111111",
            "maybeValidatedActiveChainWork": "840005",
            "bestKnownTip": {
                "source": "header_store",
                "height": 840_004,
                "blockHash": "1111111111111111111111111111111111111111111111111111111111111111",
                "work": "840005",
                "blockTimeUnixSeconds": 1_717_000_010,
                "observedAtUnixSeconds": 1_717_000_020,
                "freshness": "fresh",
                "rawPeerTable": "best tip raw phase72-live-smoke-secret"
            },
            "stayCurrent": "current_at_best_known_tip",
            "stayCurrentNextAction": "Continue monitoring best-known tip freshness.",
            "noProgressDiagnosis": "current_at_best_known_tip",
            "noProgressNextAction": "No operator action required.",
            "latestReorg": {
                "commonAncestorHeight": 840_000,
                "commonAncestorHash": "0000000000000000000000000000000000000000000000000000000000000000",
                "disconnectedCount": 0,
                "connectedCount": 4,
                "finalActiveHeight": 840_004,
                "finalActiveHash": "1111111111111111111111111111111111111111111111111111111111111111",
                "fullyPersisted": true,
                "rawLogTail": "reorg raw phase72-live-smoke-secret"
            },
            "reconcileProgress": {
                "state": "extended_active_chain",
                "connectedCount": 4,
                "finalActiveHeight": 840_004,
                "finalActiveHash": "1111111111111111111111111111111111111111111111111111111111111111",
                "rawPeerTable": "reconcile raw phase72-live-smoke-secret"
            },
            "resourcePressure": {
                "blocksInFlight": 1,
                "targetOutboundPeers": 4,
                "rpcpassword": "rpcpassword=phase72-live-smoke-secret"
            },
            "peerContribution": {
                "connected": 3,
                "failed": 1,
                "attempted": 4,
                "rawPeerTable": "peer raw phase72-live-smoke-secret"
            },
            "rpcpassword": "rpcpassword=phase72-live-smoke-secret",
            "rpcauth": "rpcauth=phase72-live-smoke-secret",
            "__cookie__": "__cookie__:phase72-live-smoke-secret"
        },
        "daemon": {
            "daemonStdout": "raw stdout phase72-live-smoke-secret",
            "daemonStderr": "raw stderr phase72-live-smoke-secret"
        },
        "wallet": {
            "walletMaterial": "seed phrase phase72-live-smoke-secret"
        }
    });

    // Act
    let summarized = summary(&report).expect("summary");
    let final_status = summarized.get("finalStatus").expect("final status summary");
    let text = summarized.to_string();

    // Assert
    for key in [
        "validatedActiveChainHeight",
        "maybeValidatedActiveChainHeightUnavailableReason",
        "maybeValidatedActiveChainHash",
        "maybeValidatedActiveChainWork",
        "bestKnownTip",
        "stayCurrent",
        "stayCurrentNextAction",
        "noProgressDiagnosis",
        "noProgressNextAction",
        "latestReorg",
        "reconcileProgress",
        "resourcePressure",
        "peerContribution",
    ] {
        assert!(
            final_status.get(key).is_some(),
            "summary missing Phase 72 key {key}"
        );
    }
    for forbidden in [
        "rawPeerTable",
        "daemonStdout",
        "daemonStderr",
        "rawLogTail",
        "rpcpassword",
        "rpcauth",
        "__cookie__",
        "walletMaterial",
        "phase72-live-smoke-secret",
    ] {
        assert!(
            !text.contains(forbidden),
            "summary copied forbidden live-smoke material: {forbidden}"
        );
    }
}

#[test]
fn live_smoke_recovery_evidence_phase77_live_smoke_summary_preserves_recovery_evidence() {
    // Arrange
    let report = json!({
        "schema_version": 2,
        "result": {
            "status": "progress",
            "rawPeerTable": "raw peer table phase77-live-smoke-secret",
            "daemonStdout": "daemon stdout phase77-live-smoke-secret",
            "daemonStderr": "daemon stderr phase77-live-smoke-secret",
            "rawLogTail": "raw log phase77-live-smoke-secret",
            "walletMaterial": "seed phrase phase77-live-smoke-secret",
        },
        "final_status": {
            "recoveryEvidence": {
                "state": "available",
                "category": "storage_lock_contention",
                "cause": "stale_lock_evidence",
                "actionClass": "read_only_inspection",
                "evidenceBasis": ["lock_probe"],
                "affectedNamespace": null,
                "affectedPath": "/tmp/open-bitcoin/LOCK",
                "nextAction": "Inspect the datadir read-only and avoid deleting lock artifacts automatically.",
                "compatibilityAction": null,
                "maybeUnavailableReason": null,
                "source": "status.recovery_evidence",
                "rawLogTail": "nested raw log phase77-live-smoke-secret",
                "rpcpassword": "rpcpassword=phase77-live-smoke-secret",
                "walletMaterial": "seed phrase phase77-live-smoke-secret"
            },
            "recoveryActionClass": "read_only_inspection",
            "recoveryCause": "stale_lock_evidence",
            "recoveryNextAction": "Inspect the datadir read-only and avoid deleting lock artifacts automatically.",
            "maybeRecoveryEvidenceUnavailableReason": null,
            "rawPeerTable": "final raw peer table phase77-live-smoke-secret",
            "daemonStdout": "final daemon stdout phase77-live-smoke-secret",
            "daemonStderr": "final daemon stderr phase77-live-smoke-secret",
            "rawLogTail": "final raw log phase77-live-smoke-secret",
            "rpcpassword": "rpcpassword=phase77-live-smoke-secret",
            "rpcauth": "rpcauth=phase77-live-smoke-secret",
            "__cookie__": "__cookie__:phase77-live-smoke-secret",
            "walletMaterial": "seed phrase phase77-live-smoke-secret"
        }
    });

    // Act
    let summarized = summary(&report).expect("summary");
    let final_status = summarized.get("finalStatus").expect("final status summary");
    let text = summarized.to_string();

    // Assert
    assert_eq!(
        final_status["recoveryEvidence"]["category"],
        "storage_lock_contention"
    );
    assert_eq!(
        final_status["recoveryEvidence"]["cause"],
        "stale_lock_evidence"
    );
    assert_eq!(
        final_status["recoveryEvidence"]["actionClass"],
        "read_only_inspection"
    );
    assert_eq!(final_status["recoveryActionClass"], "read_only_inspection");
    assert_eq!(final_status["recoveryCause"], "stale_lock_evidence");
    assert_eq!(
        final_status["recoveryNextAction"],
        "Inspect the datadir read-only and avoid deleting lock artifacts automatically."
    );
    for forbidden in [
        "rawPeerTable",
        "daemonStdout",
        "daemonStderr",
        "rawLogTail",
        "rpcpassword",
        "rpcauth",
        "__cookie__",
        "walletMaterial",
        "phase77-live-smoke-secret",
    ] {
        assert!(
            !text.contains(forbidden),
            "summary copied forbidden live-smoke material: {forbidden}"
        );
    }
}

#[test]
fn live_smoke_recovery_evidence_preserves_unavailable_reason() {
    // Arrange
    let report = json!({
        "schema_version": 2,
        "final_status": {
            "maybeRecoveryEvidenceUnavailableReason": "recovery evidence unavailable",
            "recoveryActionClass": null,
            "recoveryCause": null,
            "recoveryNextAction": null
        }
    });

    // Act
    let summarized = summary(&report).expect("summary");
    let final_status = summarized.get("finalStatus").expect("final status summary");

    // Assert
    assert_eq!(
        final_status["maybeRecoveryEvidenceUnavailableReason"],
        "recovery evidence unavailable"
    );
    assert!(final_status.get("recoveryEvidence").is_none());
}

#[test]
fn live_smoke_recovery_evidence_redacts_authorization_from_allowlisted_fields() {
    // Arrange
    let report = json!({
        "schema_version": 2,
        "final_status": {
            "recoveryEvidence": {
                "state": "available",
                "category": "storage_lock_contention",
                "cause": "stale_lock_evidence",
                "actionClass": "read_only_inspection",
                "evidenceBasis": ["lock_probe"],
                "nextAction": "Authorization: Bearer phase77-secret",
                "maybeUnavailableReason": null
            },
            "recoveryNextAction": "Authorization: Bearer phase77-secret"
        }
    });

    // Act
    let summarized = summary(&report).expect("summary");
    let text = summarized.to_string();

    // Assert
    assert!(text.contains("[redacted]"));
    for forbidden in ["Authorization", "Bearer", "phase77-secret"] {
        assert!(
            !text.contains(forbidden),
            "summary copied authorization material: {forbidden}"
        );
    }
}
