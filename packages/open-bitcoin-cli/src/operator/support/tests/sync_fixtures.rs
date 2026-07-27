// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

use super::*;

pub(super) const PHASE96_PEER_POLICY_RUNTIME_BRIDGE_NEXT_ACTION: &str = "Treat Phase 96 as scoped runtime peer policy bridge evidence only; review ban, discourage, unban, and misbehavior labels before changing listener exposure or peer policy.";

#[derive(Debug)]
pub(super) struct TestDirectory {
    pub(super) path: PathBuf,
}

impl TestDirectory {
    pub(super) fn new(label: &str) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time after epoch")
            .as_nanos();
        let path = std::env::temp_dir().join(format!(
            "open-bitcoin-support-{label}-{}-{timestamp}",
            std::process::id()
        ));
        fs::create_dir_all(&path).expect("test directory");
        Self { path }
    }

    pub(super) fn path(&self) -> &Path {
        &self.path
    }
}

impl Drop for TestDirectory {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.path);
    }
}

pub(super) fn apply_phase78_available_sync_fields(sync: &mut SyncStatus) {
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

pub(super) fn phase72_status_missing_tip_match() -> OpenBitcoinStatusSnapshot {
    let mut status = phase72_status();
    status.sync.best_known_tip =
        FieldAvailability::unavailable("best-known tip evidence unavailable");
    status
}

pub(super) fn phase72_status() -> OpenBitcoinStatusSnapshot {
    OpenBitcoinStatusSnapshot {
        node: NodeStatus {
            state: NodeRuntimeState::Running,
            version: "0.1.0".to_string(),
        },
        config: ConfigStatus {
            datadir: FieldAvailability::available("/tmp/open-bitcoin-mainnet".to_string()),
            config_paths: vec![],
        },
        service: ServiceStatus {
            manager: FieldAvailability::unavailable("service manager unavailable"),
            lifecycle: FieldAvailability::available(ServiceLifecycleStatus::Unmanaged),
            installed: FieldAvailability::unavailable("service install state unavailable"),
            enabled: FieldAvailability::unavailable("service enablement unavailable"),
            running: FieldAvailability::unavailable("service runtime unavailable"),
            service_file_path: FieldAvailability::unavailable("service file path unavailable"),
            log_path: FieldAvailability::unavailable("service log path unavailable"),
            diagnostics: FieldAvailability::unavailable("service diagnostics unavailable"),
            restart_resume: FieldAvailability::unavailable(
                "service restart/resume evidence unavailable",
            ),
        },
        sync: phase72_sync_status(),
        peers: PeerStatus {
            peer_counts: FieldAvailability::available(PeerCounts {
                inbound: 0,
                outbound: 3,
            }),
            recent_peers: FieldAvailability::unavailable("peer telemetry unavailable"),
            inbound: inbound_status_unavailable(),
        },
        mempool: MempoolStatus {
            transactions: FieldAvailability::unavailable("mempool unavailable"),
            relay: RelayEvidenceStatus::default(),
        },
        block_relay: BlockRelayEvidenceStatus::default_unavailable(),
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

pub(super) fn phase72_sync_status() -> SyncStatus {
    SyncStatus {
        network: FieldAvailability::available("mainnet".to_string()),
        chain_tip: FieldAvailability::available(ChainTipStatus {
            height: 840_004,
            block_hash: "1111111111111111111111111111111111111111111111111111111111111111"
                .to_string(),
        }),
        sync_progress: FieldAvailability::available(SyncProgress {
            header_height: 840_004,
            block_height: 840_004,
            downloaded_block_height: 840_004,
            connected_block_height: 840_004,
            validated_active_chain_height: 840_004,
            maybe_downloaded_block_hash: Some("11".repeat(32)),
            maybe_connected_block_hash: Some("11".repeat(32)),
            maybe_validated_active_chain_hash: Some("11".repeat(32)),
            maybe_validated_active_chain_work: Some("840005".to_string()),
            progress_ratio: 1.0,
            messages_processed: 128,
            headers_received: 4,
            blocks_received: 4,
        }),
        lifecycle: FieldAvailability::available(SyncLifecycleState::Active),
        phase: FieldAvailability::available("blocks".to_string()),
        configured_targets: FieldAvailability::available(SyncConfiguredTargets {
            target_outbound_peers: 4,
            maybe_target_header_height: Some(840_004),
        }),
        attempt_counters: FieldAvailability::available(SyncAttemptCounters {
            attempted_peers: 4,
            connected_peers: 3,
            failed_peers: 1,
            max_sync_rounds: 8,
        }),
        progress_signal: FieldAvailability::available(SyncProgressSignal::Steady),
        lag: FieldAvailability::available(SyncLagStatus {
            headers_remaining: 0,
            blocks_remaining: 0,
        }),
        last_successful_progress_unix_seconds: FieldAvailability::available(1_717_000_020),
        progress_credit: FieldAvailability::unavailable("progress credit evidence unavailable"),
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
            label: "best_known_tip_reached".to_string(),
            message: "best known tip reached".to_string(),
        }),
        last_error: FieldAvailability::unavailable("sync error unavailable"),
        recovery_category: FieldAvailability::unavailable("no recovery category recorded"),
        recovery_action: FieldAvailability::unavailable(
            "daemon sync recovery guidance unavailable",
        ),
        resource_pressure: FieldAvailability::available(normal_resource_pressure()),
        best_known_tip: FieldAvailability::available(BestKnownTipStatus {
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
        }),
        stay_current: FieldAvailability::available(StayCurrentStatus::InitialCatchUp),
        stay_current_next_action: FieldAvailability::available(
            "Wait for best-known tip catch-up evidence.".to_string(),
        ),
        no_progress_diagnosis: FieldAvailability::available(
            NoProgressDiagnosis::CurrentAtBestKnownTip,
        ),
        no_progress_next_action: FieldAvailability::available(
            "No operator action required.".to_string(),
        ),
        latest_reorg: FieldAvailability::unavailable("no reorg evidence recorded"),
        reconcile_progress: FieldAvailability::unavailable("reconcile progress unavailable"),
    }
}

pub(super) fn phase105_status_with_relay_evidence() -> OpenBitcoinStatusSnapshot {
    let mut status = phase72_status();
    status.mempool.transactions = FieldAvailability::available(7);
    status.mempool.relay = RelayEvidenceStatus {
        activation: RelayEvidenceField::implemented(RelayActivationEvidence { enabled: true }),
        download_eligibility: RelayEvidenceField::implemented(RelayDownloadEligibilityCounters {
            eligible_peer_count: 2,
            ineligible_peer_count: 4,
            relay_disabled_count: 1,
            not_relay_eligible_count: 1,
            inbound_serving_required_count: 0,
            permission_required_count: 1,
            protected_not_relay_count: 1,
        }),
        outcome_counters: RelayEvidenceField::implemented(RelayEvidenceCounters {
            accepted_count: 11,
            rejected_count: 2,
            orphaned_count: 3,
            requested_count: 5,
            served_count: 4,
            announced_count: 13,
            suppressed_count: 8,
            evicted_count: 1,
            expired_count: 6,
            rebroadcast_deferred_count: 9,
        }),
        recovery_counters: RelayEvidenceField::implemented(RelayRecoveryCounters {
            recovered_count: 21,
            dropped_confirmed_count: 22,
            dropped_duplicate_count: 23,
            dropped_missing_parent_count: 24,
            dropped_policy_incompatible_count: 25,
            dropped_evicted_count: 26,
        }),
        mempool_admission: RelayEvidenceField::implemented(RelayCapabilityEvidence::new(
            RelayEvidenceCapability::MempoolAdmission,
        )),
        local_submission: RelayEvidenceField::implemented(RelayCapabilityEvidence::new(
            RelayEvidenceCapability::LocalSubmissionRelay,
        )),
        fanout: RelayEvidenceField::implemented(RelayCapabilityEvidence::new(
            RelayEvidenceCapability::RelayFanout,
        )),
        serving: RelayEvidenceField::implemented(RelayCapabilityEvidence::new(
            RelayEvidenceCapability::RelayServing,
        )),
        ..RelayEvidenceStatus::default()
    };
    status
}

pub(super) fn phase105_status_with_sensitive_relay_reasons() -> OpenBitcoinStatusSnapshot {
    let mut status = phase105_status_with_relay_evidence();
    let sensitive = "raw tx hex 020000000001 txid=aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa wtxid=bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb 127.0.0.1:18444 198.51.100.105:8333 peer_id=105 permission_string=in,noban credential=phase105 secret=phase105 cookie=phase105 dynamic_label=peer";
    status.mempool.relay.outcome_counters = RelayEvidenceField::unavailable(sensitive);
    status.mempool.relay.recovery_counters = RelayEvidenceField::unavailable(sensitive);
    status.mempool.relay.activation = RelayEvidenceField::unavailable(sensitive);
    status.mempool.relay.download_eligibility = RelayEvidenceField::unavailable(sensitive);
    status.mempool.relay.mempool_admission = RelayEvidenceField::unavailable(sensitive);
    status.mempool.relay.local_submission = RelayEvidenceField::deferred(sensitive);
    status.mempool.relay.fanout = RelayEvidenceField::deferred(sensitive);
    status.mempool.relay.serving = RelayEvidenceField::deferred(sensitive);
    status.mempool.relay.rebroadcast = RelayEvidenceField::deferred(sensitive);
    status.mempool.relay.public_relay = RelayEvidenceField::intentionally_different(sensitive);
    status
}

pub(super) fn phase116_status_with_block_relay_evidence() -> OpenBitcoinStatusSnapshot {
    let mut status = phase72_status();
    status.block_relay = BlockRelayEvidenceStatus::with_components(
        open_bitcoin_node::status::BlockServingEvidenceStatus::with_activation_eligibility_and_status(
            open_bitcoin_node::status::BlockServingActivationEvidence {
                block_serving_enabled: true,
                compact_relay_enabled: true,
            },
            open_bitcoin_node::status::BlockServingEligibilityCounters {
                eligible_peer_count: 2,
                ineligible_peer_count: 3,
                disabled_count: 1,
                activation_required_count: 0,
                inbound_serving_required_count: 1,
                permission_required_count: 1,
                protected_not_serving_count: 0,
                status_unavailable_count: 0,
                permission_effect_inactive_count: 1,
            },
            open_bitcoin_node::status::BlockServingStatusCounters {
                validated_count: 5,
                available_count: 4,
                stale_count: 1,
                side_chain_count: 2,
                pruned_count: 1,
                unavailable_count: 3,
                unvalidated_count: 0,
                unknown_count: 1,
                suppressed_count: 2,
            },
        ),
        open_bitcoin_node::status::CompactRelayNegotiationCounters {
            version2_high_bandwidth_count: 3,
            version2_low_bandwidth_count: 1,
            unsupported_version_count: 1,
        },
        open_bitcoin_node::status::CompactRelayAnnouncementCounters {
            compact_announced_count: 6,
            compact_headers_fallback_count: 2,
            compact_inventory_fallback_count: 1,
            compact_suppressed_count: 2,
        },
        open_bitcoin_node::status::CompactRelayReconstructionCounters {
            compact_reconstructed_count: 4,
            compact_reconstruction_failed_count: 1,
            compact_malformed_count: 1,
        },
        open_bitcoin_node::status::CompactRelayMissingTransactionCounters {
            compact_missing_tx_requested_count: 2,
            compact_missing_tx_suppressed_count: 1,
        },
        open_bitcoin_node::status::CompactRelayFallbackCounters {
            compact_fallback_count: 2,
            compact_timeout_count: 1,
        },
        open_bitcoin_node::status::CompactRelayInFlightCounters {
            in_flight_count: 3,
            getblocktxn_in_flight_count: 2,
            peers_with_in_flight_count: 2,
        },
        open_bitcoin_node::status::CompactRelayCleanupCounters {
            compact_cleanup_count: 3,
            compact_download_peer_disconnect_count: 1,
            compact_download_timeout_count: 1,
            compact_download_reorg_count: 0,
            compact_download_restart_count: 0,
            compact_download_block_connected_count: 1,
        },
    );
    status
}

pub(super) fn phase116_status_with_sensitive_block_relay_reasons() -> OpenBitcoinStatusSnapshot {
    let mut status = phase116_status_with_block_relay_evidence();
    let sensitive = "cmpctblock blocktxn getblocktxn block_hash=0000000000000000000000000000000000000000000000000000000000000000 127.0.0.1:18444 198.51.100.116:8333 peer_id=116 permission_string=in,noban credential=phase116 secret=phase116 cookie=phase116 dynamic_label=peer";
    status.block_relay.block_serving.activation = FieldAvailability::unavailable(sensitive);
    status.block_relay.negotiation = FieldAvailability::unavailable(sensitive);
    status.block_relay.announcement = FieldAvailability::unavailable(sensitive);
    status.block_relay.reconstruction = FieldAvailability::unavailable(sensitive);
    status.block_relay.missing_transaction = FieldAvailability::unavailable(sensitive);
    status.block_relay.fallback = FieldAvailability::unavailable(sensitive);
    status.block_relay.in_flight = FieldAvailability::unavailable(sensitive);
    status.block_relay.cleanup = FieldAvailability::unavailable(sensitive);
    status
}

pub(super) fn phase127_authoritative_status_with_sensitive_operator_evidence()
-> OpenBitcoinStatusSnapshot {
    let mut status = phase116_status_with_sensitive_block_relay_reasons();
    status.mempool.relay = phase105_status_with_sensitive_relay_reasons().mempool.relay;
    let sensitive = "raw tx hex 020000000001 txid=aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa 127.0.0.1:18444 198.51.100.127:8333 permission_string=in,noban rpcpassword=phase127-secret peer-127-dynamic-label";
    status.mempool.relay.activation = RelayEvidenceField::unavailable(sensitive);
    status.block_relay.negotiation = FieldAvailability::unavailable(sensitive);

    status.peers.inbound = phase94_status_with_resource_governance_evidence()
        .peers
        .inbound;
    let FieldAvailability::Available(inbound) = &mut status.peers.inbound else {
        panic!("phase127 authoritative inbound fixture should be available");
    };
    inbound.bound_endpoints = vec![
        "127.0.0.1:18444".to_string(),
        "198.51.100.127:8333".to_string(),
    ];
    inbound.permission_class = "permission_string=in,noban".to_string();
    inbound.active_permission_effects = vec!["peer-127-dynamic-label".to_string()];
    inbound.inactive_permission_effects = vec!["rpcpassword=phase127-secret".to_string()];
    inbound.latest_permission_decision =
        FieldAvailability::available(InboundPermissionDecisionEvent {
            outcome: "admitted".to_string(),
            reason: "admitted".to_string(),
            permission_class: "permission_string=in,noban".to_string(),
            active_permission_effects: vec!["peer-127-dynamic-label".to_string()],
            inactive_permission_effects: vec!["rpcpassword=phase127-secret".to_string()],
            message: sensitive.to_string(),
        });
    inbound.local_advertisement_candidates[0].source = sensitive.to_string();
    inbound.suppressed_advertisements[0].message = sensitive.to_string();
    inbound.latest_address_decision = FieldAvailability::available(InboundAddressDecisionEvent {
        outcome: "suppressed".to_string(),
        reason: "permission_policy_denied".to_string(),
        label: "getaddr_suppressed".to_string(),
        source: "source_inbound_addr".to_string(),
        message: sensitive.to_string(),
    });
    inbound.latest_peer_policy_decision = FieldAvailability::available(InboundPeerPolicyEvent {
        outcome: "peer-127-dynamic-label".to_string(),
        reason: "permission_string=in,noban".to_string(),
        label: "rpcpassword=phase127-secret".to_string(),
        source: "198.51.100.127:8333".to_string(),
        message: sensitive.to_string(),
    });
    inbound.latest_resource_governance_decision =
        FieldAvailability::available(InboundResourceGovernanceEvent {
            outcome: "rejected".to_string(),
            reason: "peer-127-dynamic-label".to_string(),
            label: "permission_string=in,noban".to_string(),
            source: "198.51.100.127:8333".to_string(),
            message: "rpcpassword=phase127-secret".to_string(),
            next_action: sensitive.to_string(),
        });
    status
}

#[derive(Debug)]
pub(super) struct Phase127SensitiveStatusRpc {
    pub(super) network_status: OpenBitcoinNetworkStatusResponse,
}
