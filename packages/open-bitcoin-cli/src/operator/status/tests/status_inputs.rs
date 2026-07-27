// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

use super::*;

pub(super) fn service_snapshot(
    state: ServiceLifecycleState,
    maybe_enabled: Option<bool>,
    data_dir: &Path,
) -> ServiceStateSnapshot {
    ServiceStateSnapshot {
        state,
        maybe_enabled,
        maybe_service_file_path: Some(PathBuf::from("/tmp/open-bitcoin-node.service")),
        maybe_manager_diagnostics: None,
        maybe_log_path: Some(PathBuf::from("/tmp/logs/open-bitcoin.log")),
        maybe_log_path_unavailable_reason: None,
        maybe_data_dir: Some(data_dir.to_path_buf()),
        maybe_data_dir_unavailable_reason: None,
    }
}

pub(super) fn status_input_with_manager(
    manager: Box<dyn crate::operator::service::ServiceManager>,
    config_resolution: OperatorConfigResolution,
) -> StatusCollectorInput {
    StatusCollectorInput {
        request: StatusRequest {
            render_mode: StatusRenderMode::Human,
            maybe_config_path: None,
            maybe_data_dir: None,
            maybe_network: None,
            include_live_rpc: false,
            no_color: false,
        },
        config_resolution,
        detection_evidence: StatusDetectionEvidence {
            detected_installations: Vec::new(),
            service_candidates: Vec::new(),
        },
        maybe_live_rpc: None,
        maybe_service_manager: Some(manager),
        wallet_rpc_access: StatusWalletRpcAccess::Root,
    }
}

pub(super) fn status_input(
    detected_installations: Vec<DetectedInstallation>,
) -> StatusCollectorInput {
    status_input_with_service_candidates(detected_installations, Vec::new())
}

pub(super) fn status_input_for_data_dir(data_dir: &Path) -> StatusCollectorInput {
    let mut resolution = config_resolution();
    resolution.maybe_data_dir = Some(data_dir.to_path_buf());
    resolution.maybe_log_dir = None;
    resolution.maybe_metrics_store_path = None;
    StatusCollectorInput {
        request: StatusRequest {
            render_mode: StatusRenderMode::Json,
            maybe_config_path: None,
            maybe_data_dir: Some(data_dir.to_path_buf()),
            maybe_network: Some(NetworkSelection::Regtest),
            include_live_rpc: false,
            no_color: true,
        },
        config_resolution: resolution,
        detection_evidence: StatusDetectionEvidence {
            detected_installations: Vec::new(),
            service_candidates: Vec::new(),
        },
        maybe_live_rpc: None,
        maybe_service_manager: None,
        wallet_rpc_access: StatusWalletRpcAccess::Root,
    }
}

pub(super) fn status_input_with_running_manager_and_live_rpc(
    data_dir: &Path,
) -> StatusCollectorInput {
    let mut input = status_input_with_manager(
        Box::new(FakeServiceManager::new(service_snapshot(
            ServiceLifecycleState::Running,
            Some(true),
            data_dir,
        ))),
        {
            let mut resolution = config_resolution();
            resolution.maybe_data_dir = Some(data_dir.to_path_buf());
            resolution.maybe_log_dir = None;
            resolution.maybe_metrics_store_path = None;
            resolution
        },
    );
    input.request.render_mode = StatusRenderMode::Json;
    input.request.include_live_rpc = true;
    input.maybe_live_rpc = Some(StatusLiveRpcAdapterInput {
        endpoint: "http://127.0.0.1:18443".to_string(),
        auth_source: StatusRpcAuthSource::CookieFile {
            path: data_dir.join(".cookie"),
        },
        timeout: Duration::from_secs(2),
    });
    input
}

pub(super) fn status_input_with_service_candidates(
    detected_installations: Vec<DetectedInstallation>,
    service_candidates: Vec<ServiceCandidate>,
) -> StatusCollectorInput {
    StatusCollectorInput {
        request: StatusRequest {
            render_mode: StatusRenderMode::Human,
            maybe_config_path: None,
            maybe_data_dir: None,
            maybe_network: Some(NetworkSelection::Regtest),
            include_live_rpc: true,
            no_color: false,
        },
        config_resolution: config_resolution(),
        detection_evidence: StatusDetectionEvidence {
            detected_installations,
            service_candidates,
        },
        maybe_live_rpc: Some(StatusLiveRpcAdapterInput {
            endpoint: "http://127.0.0.1:18443".to_string(),
            auth_source: StatusRpcAuthSource::CookieFile {
                path: PathBuf::from("/tmp/open-bitcoin/.cookie"),
            },
            timeout: Duration::from_secs(2),
        }),
        maybe_service_manager: None,
        wallet_rpc_access: StatusWalletRpcAccess::Root,
    }
}

pub(super) fn config_resolution() -> OperatorConfigResolution {
    OperatorConfigResolution {
        path_reports: vec![
            OperatorConfigPathReport {
                source: OperatorConfigSource::Defaults,
                kind: OperatorConfigPathKind::ConfigFile,
                path: PathBuf::from("/tmp/open-bitcoin/open-bitcoin.jsonc"),
                present: false,
            },
            OperatorConfigPathReport {
                source: OperatorConfigSource::BitcoinConf,
                kind: OperatorConfigPathKind::BitcoinConf,
                path: PathBuf::from("/tmp/open-bitcoin/bitcoin.conf"),
                present: false,
            },
        ],
        maybe_config_path: Some(PathBuf::from("/tmp/open-bitcoin/open-bitcoin.jsonc")),
        maybe_bitcoin_conf_path: Some(PathBuf::from("/tmp/open-bitcoin/bitcoin.conf")),
        maybe_data_dir: Some(PathBuf::from("/tmp/open-bitcoin")),
        maybe_network: Some(NetworkSelection::Regtest),
        maybe_log_dir: Some(PathBuf::from("/tmp/open-bitcoin/logs")),
        maybe_metrics_store_path: Some(PathBuf::from("/tmp/open-bitcoin/metrics")),
        ..OperatorConfigResolution::default()
    }
}

pub(super) fn detected_installation() -> DetectedInstallation {
    DetectedInstallation {
        product_family: ProductFamily::Unknown,
        confidence: DetectionConfidence::Low,
        uncertainty: vec![DetectionUncertainty::ProductAmbiguous],
        source_paths: vec![
            DetectionSourcePath {
                kind: DetectionSourcePathKind::DataDir,
                path: PathBuf::from("/tmp/core/.bitcoin"),
                present: true,
            },
            DetectionSourcePath {
                kind: DetectionSourcePathKind::ConfigFile,
                path: PathBuf::from("/tmp/core/.bitcoin/bitcoin.conf"),
                present: true,
            },
            DetectionSourcePath {
                kind: DetectionSourcePathKind::CookieFile,
                path: PathBuf::from("/tmp/core/.bitcoin/.cookie"),
                present: true,
            },
        ],
        maybe_data_dir: Some(PathBuf::from("/tmp/core/.bitcoin")),
        maybe_config_file: Some(PathBuf::from("/tmp/core/.bitcoin/bitcoin.conf")),
        maybe_cookie_file: Some(PathBuf::from("/tmp/core/.bitcoin/.cookie")),
        wallet_candidates: vec![WalletCandidate {
            kind: WalletCandidateKind::LegacyWalletFile,
            path: PathBuf::from("/tmp/core/.bitcoin/wallet.dat"),
            maybe_name: None,
            present: true,
            product_family: ProductFamily::Unknown,
            product_confidence: DetectionConfidence::Low,
            chain_scope: crate::operator::detect::WalletChainScope::Mainnet,
        }],
    }
}

pub(super) fn detected_service_candidate() -> ServiceCandidate {
    ServiceCandidate {
        product_family: ProductFamily::Unknown,
        manager: ServiceManager::Systemd,
        service_name: "bitcoind".to_string(),
        path: PathBuf::from("/tmp/systemd/bitcoind.service"),
        present: true,
    }
}

#[derive(Debug, Clone)]
pub(super) struct FakeStatusRpcClient {
    pub(super) maybe_node_error: Option<StatusRpcError>,
    pub(super) maybe_network_status_error: Option<StatusRpcError>,
    pub(super) maybe_network_status: Option<OpenBitcoinNetworkStatusResponse>,
    pub(super) maybe_wallet_error: Option<StatusRpcError>,
}

impl FakeStatusRpcClient {
    pub(super) fn running() -> Self {
        Self {
            maybe_node_error: None,
            maybe_network_status_error: None,
            maybe_network_status: Some(OpenBitcoinNetworkStatusResponse {
                inbound: FieldAvailability::<InboundPeerServingStatus>::unavailable(
                    INBOUND_STATUS_UNAVAILABLE_REASON,
                ),
                relay: RelayEvidenceStatus::default(),
                block_relay: BlockRelayEvidenceStatus::default_unavailable(),
                metrics: MetricsStatus::default(),
            }),
            maybe_wallet_error: None,
        }
    }

    pub(super) fn running_with_inbound_status() -> Self {
        Self {
            maybe_network_status: Some(inbound_status_response()),
            ..Self::running()
        }
    }

    pub(super) fn failing(message: &str) -> Self {
        Self {
            maybe_node_error: Some(StatusRpcError::new(message)),
            maybe_network_status_error: None,
            maybe_network_status: None,
            maybe_wallet_error: None,
        }
    }

    pub(super) fn network_status_failing(error: StatusRpcError) -> Self {
        Self {
            maybe_network_status_error: Some(error),
            maybe_network_status: None,
            ..Self::running()
        }
    }

    pub(super) fn wallet_failing(error: StatusRpcError) -> Self {
        Self {
            maybe_node_error: None,
            maybe_network_status_error: None,
            maybe_network_status: Some(OpenBitcoinNetworkStatusResponse {
                inbound: FieldAvailability::<InboundPeerServingStatus>::unavailable(
                    INBOUND_STATUS_UNAVAILABLE_REASON,
                ),
                relay: RelayEvidenceStatus::default(),
                block_relay: BlockRelayEvidenceStatus::default_unavailable(),
                metrics: MetricsStatus::default(),
            }),
            maybe_wallet_error: Some(error),
        }
    }

    pub(super) fn maybe_node_error(&self) -> Result<(), StatusRpcError> {
        match &self.maybe_node_error {
            Some(error) => Err(error.clone()),
            None => Ok(()),
        }
    }

    pub(super) fn maybe_wallet_error(&self) -> Result<(), StatusRpcError> {
        self.maybe_node_error()?;
        match &self.maybe_wallet_error {
            Some(error) => Err(error.clone()),
            None => Ok(()),
        }
    }
}

impl StatusRpcClient for FakeStatusRpcClient {
    fn get_network_info(&self) -> Result<GetNetworkInfoResponse, StatusRpcError> {
        self.maybe_node_error()?;
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
            warnings: vec!["network warning".to_string()],
        })
    }

    fn get_open_bitcoin_network_status(
        &self,
    ) -> Result<OpenBitcoinNetworkStatusResponse, StatusRpcError> {
        self.maybe_node_error()?;
        if let Some(error) = &self.maybe_network_status_error {
            return Err(error.clone());
        }

        Ok(self
            .maybe_network_status
            .clone()
            .unwrap_or_else(|| OpenBitcoinNetworkStatusResponse {
                inbound: FieldAvailability::<InboundPeerServingStatus>::unavailable(
                    INBOUND_STATUS_UNAVAILABLE_REASON,
                ),
                relay: RelayEvidenceStatus::default(),
                block_relay: BlockRelayEvidenceStatus::default_unavailable(),
                metrics: MetricsStatus::default(),
            }))
    }

    fn get_blockchain_info(&self) -> Result<GetBlockchainInfoResponse, StatusRpcError> {
        self.maybe_node_error()?;
        Ok(GetBlockchainInfoResponse {
            chain: "regtest".to_string(),
            blocks: 144,
            headers: 150,
            maybe_best_block_hash: Some("00aabb".to_string()),
            maybe_median_time_past: Some(1_777_225_000),
            verificationprogress: 0.96,
            initialblockdownload: false,
            warnings: vec!["chain warning".to_string()],
        })
    }

    fn get_mempool_info(&self) -> Result<GetMempoolInfoResponse, StatusRpcError> {
        self.maybe_node_error()?;
        Ok(GetMempoolInfoResponse {
            size: 12,
            bytes: 2048,
            usage: 4096,
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
        self.maybe_wallet_error()?;
        Ok(GetWalletInfoResponse {
            network: "regtest".to_string(),
            descriptor_count: 2,
            utxo_count: 1,
            maybe_tip_height: Some(144),
            maybe_tip_median_time_past: Some(1_777_225_000),
        })
    }

    fn get_balances(&self) -> Result<GetBalancesResponse, StatusRpcError> {
        self.maybe_wallet_error()?;
        Ok(GetBalancesResponse {
            mine: WalletBalanceDetails {
                trusted_sats: 50_000,
                untrusted_pending_sats: 0,
                immature_sats: 0,
            },
        })
    }
}

pub(super) fn inbound_status_response() -> OpenBitcoinNetworkStatusResponse {
    OpenBitcoinNetworkStatusResponse {
        inbound: FieldAvailability::available(InboundPeerServingStatus {
            listener_state: "listening".to_string(),
            bound_endpoints: vec!["127.0.0.1:18444".to_string()],
            preflight_reason: "ready".to_string(),
            admitted_inbound_peers: 2,
            rejected_inbound_peers: 3,
            handshake: InboundHandshakeStatusCounts {
                awaiting_version: 1,
                awaiting_verack: 0,
                established: 2,
                disconnected: 1,
            },
            duplicate_rejects: 1,
            self_connection_rejects: 1,
            cap_rejects: 1,
            reserved_slot_rejects: 0,
            latest_admission_event: FieldAvailability::available(InboundAdmissionEvent {
                outcome: "rejected".to_string(),
                reason: "duplicate_peer_id".to_string(),
                slot_class: "ordinary".to_string(),
                message: "duplicate inbound peer id rejected".to_string(),
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
            inactive_permission_effect_observations: 3,
            permission_validation_failures: 0,
            latest_permission_decision: FieldAvailability::available(
                InboundPermissionDecisionEvent {
                    outcome: "admitted".to_string(),
                    reason: "admitted".to_string(),
                    permission_class: "protected_inbound".to_string(),
                    active_permission_effects: vec![
                        "admission_protected".to_string(),
                        "download_serving_policy_input".to_string(),
                    ],
                    inactive_permission_effects: vec!["inactive_relay".to_string()],
                    message: "inbound permission decision admitted as protected_inbound"
                        .to_string(),
                },
            ),
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
                    reason: "permission_policy_denied".to_string(),
                    label: "permission_policy_denied".to_string(),
                    source: "source_inbound_addr".to_string(),
                    message: "bounded getaddr permission policy denied".to_string(),
                },
            ],
            getaddr_responses_served: 3,
            getaddr_requests_suppressed: 2,
            learned_address_entries: 5,
            learned_address_rejections: 1,
            latest_address_decision: FieldAvailability::available(InboundAddressDecisionEvent {
                outcome: "suppressed".to_string(),
                reason: "empty_response_cache".to_string(),
                label: "getaddr_suppressed".to_string(),
                source: "source_inbound_addr".to_string(),
                message: "bounded getaddr empty response cache".to_string(),
            }),
            eviction_candidates_evaluated: 2,
            disconnects_requested: 1,
            discouraged_peers: 0,
            active_bans: 0,
            expired_bans: 0,
            manual_unbans: 0,
            misbehavior_observations: 1,
            protected_no_actions: 1,
            latest_peer_policy_decision: FieldAvailability::available(InboundPeerPolicyEvent {
                outcome: "protected_no_action".to_string(),
                reason: "malformed_message".to_string(),
                label: "misbehavior_policy_decision".to_string(),
                source: "source_misbehavior_policy".to_string(),
                message: "misbehavior policy decision protected_no_action: malformed_message"
                    .to_string(),
            }),
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
        }),
        relay: RelayEvidenceStatus::default(),
        block_relay: BlockRelayEvidenceStatus::default_unavailable(),
        metrics: MetricsStatus::default(),
    }
}
