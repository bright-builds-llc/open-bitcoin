use super::*;

impl StatusRpcClient for Phase127SensitiveStatusRpc {
    fn get_network_info(&self) -> Result<GetNetworkInfoResponse, StatusRpcError> {
        Ok(GetNetworkInfoResponse {
            version: 29_300,
            subversion: "/Satoshi:29.3.0/".to_string(),
            protocolversion: 70_016,
            localservices: "0000000000000409".to_string(),
            localrelay: true,
            connections: 7,
            connections_in: 2,
            connections_out: 5,
            relayfee: 1_000,
            incrementalfee: 1_000,
            warnings: Vec::new(),
        })
    }

    fn get_open_bitcoin_network_status(
        &self,
    ) -> Result<OpenBitcoinNetworkStatusResponse, StatusRpcError> {
        Ok(self.network_status.clone())
    }

    fn get_blockchain_info(&self) -> Result<GetBlockchainInfoResponse, StatusRpcError> {
        Ok(GetBlockchainInfoResponse {
            chain: "regtest".to_string(),
            blocks: 144,
            headers: 144,
            maybe_best_block_hash: Some("00aabb".to_string()),
            maybe_median_time_past: Some(1_777_225_000),
            verificationprogress: 1.0,
            initialblockdownload: false,
            warnings: Vec::new(),
        })
    }

    fn get_mempool_info(&self) -> Result<GetMempoolInfoResponse, StatusRpcError> {
        Ok(GetMempoolInfoResponse {
            size: 12,
            bytes: 2_048,
            usage: 4_096,
            total_fee_sats: 320,
            maxmempool: 300_000_000,
            mempoolminfee: 1_000,
            minrelaytxfee: 1_000,
            incrementalrelayfee: 1_000,
            rollingmempoolfee: 0,
            effectiveadmissionfee: 1_000,
            capacityenforcement: "accounted_memory".to_string(),
            loaded: true,
        })
    }

    fn get_wallet_info(&self) -> Result<GetWalletInfoResponse, StatusRpcError> {
        Err(StatusRpcError::new("wallet unavailable"))
    }

    fn get_balances(&self) -> Result<GetBalancesResponse, StatusRpcError> {
        Err(StatusRpcError::new("wallet unavailable"))
    }
}

pub(super) fn phase127_status_collector_input(data_dir: &Path) -> StatusCollectorInput {
    StatusCollectorInput {
        request: StatusRequest {
            render_mode: StatusRenderMode::Json,
            maybe_config_path: None,
            maybe_data_dir: Some(data_dir.to_path_buf()),
            maybe_network: None,
            include_live_rpc: true,
            no_color: true,
        },
        config_resolution: phase75_config_resolution(data_dir),
        detection_evidence: StatusDetectionEvidence {
            detected_installations: Vec::new(),
            service_candidates: Vec::new(),
        },
        maybe_live_rpc: None,
        maybe_service_manager: None,
        wallet_rpc_access: StatusWalletRpcAccess::Root,
    }
}

pub(super) fn normal_resource_pressure() -> SyncResourcePressure {
    SyncResourcePressure {
        blocks_in_flight: 1,
        max_header_requests_in_flight_per_peer: 1,
        max_headers_per_message: 2_000,
        max_blocks_in_flight_per_peer: 16,
        max_blocks_in_flight_total: 64,
        max_messages_per_peer: 64,
        max_sync_rounds: 8,
        outbound_peers: 4,
        target_outbound_peers: 4,
    }
}

pub(super) fn missing_live_smoke() -> LiveSmokeEvidence {
    LiveSmokeEvidence {
        state: super::EvidenceState::Unavailable,
        report_path: None,
        summary: None,
        reason: Some("live smoke report not provided".to_string()),
    }
}

pub(super) fn phase75_config_resolution(data_dir: &Path) -> OperatorConfigResolution {
    OperatorConfigResolution {
        maybe_data_dir: Some(data_dir.to_path_buf()),
        ..OperatorConfigResolution::default()
    }
}

pub(super) fn phase75_support_bundle_for_test(data_dir: &Path) -> SupportEvidenceBundle {
    let resolution = phase75_config_resolution(data_dir);
    let status = phase72_status();
    let live_smoke = missing_live_smoke();
    let full_sync_evidence = derive_full_sync_evidence(&status, &live_smoke);
    let output_dir = data_dir.join("support");
    let redaction = redaction_summary();
    let soak_collection = collect_soak_support_evidence(&resolution, &redaction);
    SupportEvidenceBundle {
        generated_at_unix_seconds: 1_781_485_562,
        generated_by: "phase75 test".to_string(),
        output: SupportEvidenceOutput {
            directory: output_dir.display().to_string(),
            json_path: output_dir
                .join("support-evidence.json")
                .display()
                .to_string(),
            markdown_path: output_dir.join("support-evidence.md").display().to_string(),
        },
        redaction,
        config: super::ConfigEvidence::from_resolution(&resolution),
        status: status.clone(),
        recovery_evidence: RecoverySupportEvidence::from_status(&status.recovery_evidence),
        store_health: unavailable_store_health(),
        live_smoke,
        full_sync_evidence,
        soak_evidence: soak_collection.soak_evidence,
        support_forensics: soak_collection.support_forensics,
        resource_bound_evidence: collect_resource_bound_support_evidence(&status, &output_dir),
    }
}

pub(super) fn phase77_support_bundle_with_status(
    data_dir: &Path,
    status: OpenBitcoinStatusSnapshot,
) -> SupportEvidenceBundle {
    let mut bundle = phase75_support_bundle_for_test(data_dir);
    bundle.status = support_status_for_bundle(status);
    bundle.recovery_evidence =
        RecoverySupportEvidence::from_status(&bundle.status.recovery_evidence);
    bundle.full_sync_evidence = derive_full_sync_evidence(&bundle.status, &bundle.live_smoke);
    bundle.resource_bound_evidence =
        collect_resource_bound_support_evidence(&bundle.status, &data_dir.join("support"));
    bundle
}

pub(super) fn phase90_status_with_available_inbound() -> OpenBitcoinStatusSnapshot {
    let mut status = phase72_status();
    status.peers.inbound = FieldAvailability::available(InboundPeerServingStatus {
        listener_state: "listening".to_string(),
        bound_endpoints: phase90_raw_inbound_endpoints()
            .into_iter()
            .map(str::to_string)
            .collect(),
        preflight_reason: "ready".to_string(),
        admitted_inbound_peers: 3,
        rejected_inbound_peers: 4,
        handshake: InboundHandshakeStatusCounts {
            awaiting_version: 1,
            awaiting_verack: 2,
            established: 3,
            disconnected: 4,
        },
        duplicate_rejects: 1,
        self_connection_rejects: 1,
        cap_rejects: 1,
        reserved_slot_rejects: 1,
        latest_admission_event: FieldAvailability::available(InboundAdmissionEvent {
            outcome: "rejected".to_string(),
            reason: "cap_reject".to_string(),
            slot_class: "ordinary".to_string(),
            message: "inbound cap reached".to_string(),
        }),
        permissioned_inbound_peers: 0,
        protected_inbound_peers: 0,
        permission_class: "ordinary_inbound".to_string(),
        active_permission_effects: Vec::new(),
        inactive_permission_effects: Vec::new(),
        inactive_permission_effect_observations: 0,
        permission_validation_failures: 0,
        latest_permission_decision: FieldAvailability::unavailable(
            "inbound permission decision evidence unavailable",
        ),
        local_advertisement_candidates: Vec::new(),
        suppressed_advertisements: Vec::new(),
        getaddr_responses_served: 0,
        getaddr_requests_suppressed: 0,
        learned_address_entries: 0,
        learned_address_rejections: 0,
        latest_address_decision: FieldAvailability::unavailable(
            "inbound address boundary evidence unavailable",
        ),
        eviction_candidates_evaluated: 0,
        disconnects_requested: 0,
        discouraged_peers: 0,
        active_bans: 0,
        expired_bans: 0,
        manual_unbans: 0,
        misbehavior_observations: 0,
        protected_no_actions: 0,
        latest_peer_policy_decision: FieldAvailability::unavailable(
            "inbound peer policy evidence unavailable",
        ),
        resource_pressure_events: 0,
        read_queue_pressure_events: 0,
        write_queue_pressure_events: 0,
        request_cap_events: 0,
        payload_rejections: 0,
        timeout_disconnects: 0,
        churn_rejections: 0,
        reconnect_suppressions: 0,
        latest_resource_governance_decision: FieldAvailability::unavailable(
            "inbound resource governance evidence unavailable",
        ),
    });
    status
}

pub(super) fn phase91_status_with_permissioned_inbound() -> OpenBitcoinStatusSnapshot {
    let mut status = phase90_status_with_available_inbound();
    let FieldAvailability::Available(inbound) = &mut status.peers.inbound else {
        panic!("inbound status fixture should be available");
    };
    inbound.duplicate_rejects = 0;
    inbound.self_connection_rejects = 0;
    inbound.cap_rejects = 0;
    inbound.reserved_slot_rejects = 0;
    inbound.permissioned_inbound_peers = 2;
    inbound.protected_inbound_peers = 1;
    inbound.permission_class = "protected_inbound".to_string();
    inbound.active_permission_effects = vec![
        "admission_protected".to_string(),
        "download_serving_policy_input".to_string(),
    ];
    inbound.inactive_permission_effects =
        vec!["inactive_relay".to_string(), "inactive_mempool".to_string()];
    inbound.latest_permission_decision =
        FieldAvailability::available(InboundPermissionDecisionEvent {
            outcome: "admitted".to_string(),
            reason: "admitted".to_string(),
            permission_class: "protected_inbound".to_string(),
            active_permission_effects: vec!["admission_protected".to_string()],
            inactive_permission_effects: vec!["inactive_relay".to_string()],
            message: "inbound permission decision admitted as protected_inbound".to_string(),
        });
    status
}

pub(super) fn phase92_status_with_address_boundary_evidence() -> OpenBitcoinStatusSnapshot {
    let mut status = phase90_status_with_available_inbound();
    let FieldAvailability::Available(inbound) = &mut status.peers.inbound else {
        panic!("inbound status fixture should be available");
    };
    inbound.local_advertisement_candidates = vec![InboundAddressEvidenceEntry {
        source: "source_local_listener".to_string(),
        network_kind: "ipv4".to_string(),
        routability: "publicly_routable".to_string(),
        freshness: "fresh".to_string(),
        services_bits: 1,
        port: 8333,
        persistence_eligible: true,
    }];
    inbound.suppressed_advertisements = vec![
        InboundAddressDecisionEvent {
            outcome: "suppressed".to_string(),
            reason: "not_publicly_routable".to_string(),
            label: "not_publicly_routable".to_string(),
            source: "source_local_listener".to_string(),
            message: "local evidence only".to_string(),
        },
        InboundAddressDecisionEvent {
            outcome: "suppressed".to_string(),
            reason: "permission_policy_denied".to_string(),
            label: "permission_policy_denied".to_string(),
            source: "source_inbound_addr".to_string(),
            message: "bounded getaddr permission policy denied".to_string(),
        },
    ];
    inbound.getaddr_responses_served = 3;
    inbound.getaddr_requests_suppressed = 2;
    inbound.learned_address_entries = 5;
    inbound.learned_address_rejections = 1;
    inbound.latest_address_decision = FieldAvailability::available(InboundAddressDecisionEvent {
        outcome: "suppressed".to_string(),
        reason: "already_served".to_string(),
        label: "getaddr_suppressed".to_string(),
        source: "source_inbound_addr".to_string(),
        message: "bounded getaddr already served".to_string(),
    });
    status
}

pub(super) fn phase93_status_with_peer_policy_evidence() -> OpenBitcoinStatusSnapshot {
    let mut status = phase92_status_with_address_boundary_evidence();
    let FieldAvailability::Available(inbound) = &mut status.peers.inbound else {
        panic!("inbound status fixture should be available");
    };
    inbound.eviction_candidates_evaluated = 2;
    inbound.disconnects_requested = 1;
    inbound.discouraged_peers = 1;
    inbound.active_bans = 1;
    inbound.expired_bans = 0;
    inbound.manual_unbans = 0;
    inbound.misbehavior_observations = 2;
    inbound.protected_no_actions = 1;
    inbound.latest_peer_policy_decision = FieldAvailability::available(InboundPeerPolicyEvent {
        outcome: "selected".to_string(),
        reason: "low_activity".to_string(),
        label: "eviction_candidate_selected".to_string(),
        source: "source_eviction_policy".to_string(),
        message: "peer eviction decision eviction_candidate_selected: low_activity".to_string(),
    });
    status
}

pub(super) fn phase94_status_with_resource_governance_evidence() -> OpenBitcoinStatusSnapshot {
    let mut status = phase93_status_with_peer_policy_evidence();
    let FieldAvailability::Available(inbound) = &mut status.peers.inbound else {
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
    status
}

pub(super) fn phase90_raw_inbound_endpoints() -> [&'static str; 4] {
    [
        "127.0.0.1:18444",
        "203.0.113.11:8333",
        "198.51.100.21:8333",
        "0.0.0.0:8333",
    ]
}

pub(super) fn phase77_status_with_available_recovery() -> OpenBitcoinStatusSnapshot {
    let mut status = phase72_status();
    status.sync.recovery_category =
        FieldAvailability::unavailable("legacy recovery category unavailable");
    status.sync.recovery_action =
        FieldAvailability::unavailable("legacy recovery action unavailable");
    status.recovery_evidence = FieldAvailability::available(phase77_recovery_evidence());
    status
}

pub(super) fn phase77_recovery_evidence() -> RecoveryEvidenceSnapshot {
    RecoveryEvidenceSnapshot {
        category: SyncRecoveryCategory::StorageLockContention,
        action_class: RecoveryActionClass::ReadOnlyInspection,
        cause: RecoveryCause::StaleLockEvidence,
        evidence_basis: vec![RecoveryEvidenceBasis::LockProbe],
        maybe_affected_namespace: None,
        maybe_affected_path: Some("/tmp/open-bitcoin/LOCK".to_string()),
        next_action:
            "Inspect the datadir read-only and avoid deleting lock artifacts automatically."
                .to_string(),
        compatibility_action: FieldAvailability::unavailable(
            "no compatibility recovery action recorded",
        ),
    }
}

pub(super) fn phase79_shared_contract_status() -> OpenBitcoinStatusSnapshot {
    let mut status = phase72_status();
    status.recovery_evidence = FieldAvailability::available(phase77_recovery_evidence());
    status.sync.recovery_category =
        FieldAvailability::available(SyncRecoveryCategory::StorageLockContention);
    status.sync.progress_credit = FieldAvailability::available(ProgressCreditEvidence {
        kind: ProgressCreditKind::ValidatedDurableActiveChain,
        credited_validated_active_chain_height: 840_004,
        credited_validated_active_chain_hash: "11".repeat(32),
        credited_validated_active_chain_work: "840005".to_string(),
        source_unix_seconds: 1_717_000_020,
        rejected_activity: vec![RejectedProgressActivity {
            kind: RejectedProgressActivityKind::HeaderDownload,
            observed_count: 2,
            reason: "headers without durable active-chain update".to_string(),
        }],
    });
    status.sync.last_peer_contribution = FieldAvailability::available(PeerContributionEvidence {
        peer: "peer-79".to_string(),
        maybe_resolved_endpoint: None,
        kind: PeerContributionKind::HeadersAndBlocks,
        messages_processed: 9,
        headers_received: 4,
        blocks_received: 2,
        maybe_last_activity_unix_seconds: Some(1_717_000_030),
        maybe_failure_reason_label: None,
    });
    status.sync.stall_diagnosis = FieldAvailability::available(StallDiagnosisEvidence {
        stalled_subsystem: StalledSubsystem::StorageOrResourcePressure,
        confidence: StallDiagnosisConfidence::Medium,
        evidence_basis: vec![
            "resource_bounds".to_string(),
            "support_bundle_size".to_string(),
        ],
        next_action: "Inspect support bundle resource bounds.".to_string(),
        maybe_no_progress_diagnosis: Some(NoProgressDiagnosis::StorageOrResourceBlocked),
        maybe_recovery_category: Some(SyncRecoveryCategory::StorageLockContention),
        maybe_latest_stop_reason_label: Some("resource_pressure".to_string()),
        source_unix_seconds: 1_717_000_032,
    });
    status.sync.no_progress_diagnosis =
        FieldAvailability::available(NoProgressDiagnosis::StorageOrResourceBlocked);
    status.resource_bounds = FieldAvailability::available(phase79_resource_bound_snapshot());
    status
}

pub(super) fn phase79_shared_checkpoint_status() -> SoakCheckpointStatus {
    SoakCheckpointStatus {
        maybe_network: Some("mainnet".to_string()),
        maybe_lifecycle: Some("active".to_string()),
        maybe_latest_stop_reason_label: Some("resource_pressure".to_string()),
        maybe_recovery_category_label: Some("storage_lock_contention".to_string()),
        maybe_recovery_action_class_label: Some("read_only_inspection".to_string()),
        maybe_recovery_cause_label: Some("stale_lock_evidence".to_string()),
        maybe_recovery_next_action: Some(
            "Inspect the datadir read-only and avoid deleting lock artifacts automatically."
                .to_string(),
        ),
        maybe_no_progress_diagnosis_label: Some("storage_or_resource_blocked".to_string()),
        maybe_progress_credit_kind_label: Some("validated_durable_active_chain".to_string()),
        maybe_progress_credit_height: Some(840_004),
        maybe_progress_credit_hash: Some("11".repeat(32)),
        maybe_progress_credit_work: Some("840005".to_string()),
        maybe_progress_credit_source_unix_seconds: Some(1_717_000_020),
        progress_credit_rejected_activity_labels: vec![
            "kind=header_download observed_count=2 reason=headers without durable active-chain update"
                .to_string(),
        ],
        maybe_expected_progress_window_seconds: Some(300),
        maybe_no_progress_threshold_state_label: Some("within_window".to_string()),
        maybe_no_progress_threshold_seconds: Some(300),
        maybe_last_useful_work_kind_label: Some("current_at_best_known_tip".to_string()),
        maybe_last_useful_work_height: Some(840_004),
        maybe_last_peer_contribution_label: Some(
            "peer=peer-79 kind=headers_and_blocks messages=9 headers=4 blocks=2 failure=unavailable"
                .to_string(),
        ),
        maybe_stalled_subsystem_label: Some("storage_or_resource_pressure".to_string()),
        maybe_stall_confidence_label: Some("medium".to_string()),
        stall_evidence_basis: vec![
            "resource_bounds".to_string(),
            "support_bundle_size".to_string(),
        ],
        maybe_stall_next_action: Some("Inspect support bundle resource bounds.".to_string()),
        maybe_resource_bound_state_label: Some("warning".to_string()),
        resource_bound_labels: vec!["support_bundle=warning".to_string()],
        maybe_resource_bound_next_action: Some("Archive or rotate large support bundles.".to_string()),
        maybe_validated_active_chain_height: Some(840_004),
        maybe_best_known_tip_height: Some(840_004),
        maybe_source_status_path: Some(PathBuf::from("/tmp/open-bitcoin-mainnet/status-snapshot.json")),
    }
}
