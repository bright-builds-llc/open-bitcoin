// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

use super::*;
use open_bitcoin_node::status::{
    BasicIndexFailureCategory, BasicIndexNextAction, BasicIndexOperationalState,
    BasicIndexOperatorStatus, BasicIndexProtection, BasicIndexProtectionRange,
    BasicIndexUnavailableReason, BasicPeerScopeStatus, BasicRetainedAvailability,
    BasicRetentionBasis, BasicRetentionMeasurement,
};

#[test]
fn phase161_basic_support_sanitizes_copy_before_rust_and_json_rendering() {
    // Arrange
    let temp = TestDirectory::new("phase161-basic-redaction");
    let mut status = phase72_status();
    status.basic_index.operational_state =
        FieldAvailability::unavailable("/private/phase161-secret\n\u{1b}[31m");
    let original = status.clone();
    // Act
    let bundle = phase77_support_bundle_with_status(temp.path(), status.clone());
    let json = serde_json::to_value(&bundle).expect("bundle json");
    let markdown = render::render_support_markdown(&bundle);
    // Assert
    assert_eq!(status, original);
    assert_eq!(
        bundle.status.basic_index.operational_state,
        FieldAvailability::unavailable(BasicIndexUnavailableReason::NotReported.as_str())
    );
    assert_eq!(
        json["status"]["basic_index"]["operational_state"]["value"]["reason"],
        BasicIndexUnavailableReason::NotReported.as_str()
    );
    assert!(
        markdown.contains("## BASIC index\n\n- State: Unavailable: BASIC evidence not reported")
    );
    assert!(!markdown.contains("phase161-secret"));
    assert!(!markdown.contains('\u{1b}'));
}

fn failed_retained_status() -> BasicIndexOperatorStatus {
    BasicIndexOperatorStatus {
        configured_indexing: FieldAvailability::available(true),
        configured_peer_serving: FieldAvailability::available(false),
        peer_scope: FieldAvailability::available(BasicPeerScopeStatus {
            general_sessions: 0,
            explicit_sessions: 2,
            all_sessions: 3,
            implicit_sessions: 0,
            offered_sessions: 7,
        }),
        operational_state: FieldAvailability::available(BasicIndexOperationalState::Failed),
        initial_sync_complete: FieldAvailability::available(true),
        maybe_accepted_height: FieldAvailability::available(Some(24)),
        maybe_processed_height: FieldAvailability::available(Some(21)),
        maybe_safe_durable_height: FieldAvailability::available(Some(18)),
        maybe_lag_blocks: FieldAvailability::available(Some(3)),
        maybe_failure: FieldAvailability::available(Some(BasicIndexFailureCategory::RequiredUndo)),
        retained_availability: FieldAvailability::available(BasicRetainedAvailability::Observed),
        retention: FieldAvailability::available(BasicRetentionMeasurement {
            retained_record_count: 7,
            encoded_filter_payload_bytes: 13,
            retained_record_key_value_bytes: 97,
            basis: BasicRetentionBasis::ImmutableRecordKeysAndValuesIncludingStaleBranches,
        }),
        protection: FieldAvailability::available(BasicIndexProtection::InclusiveRange(
            BasicIndexProtectionRange::new(18, 24).expect("valid bounds"),
        )),
        next_action: FieldAvailability::available(BasicIndexNextAction::RequiredUndo),
    }
}

#[test]
fn phase161_basic_support_preserves_captured_safe_facts_and_full_human_rows() {
    // Arrange
    let temp = TestDirectory::new("phase161-basic-agreement");
    let mut status = phase72_status();
    status.basic_index = failed_retained_status();
    let original = status.clone();
    // Act
    let bundle = phase77_support_bundle_with_status(temp.path(), status.clone());
    let encoded = serde_json::to_value(&bundle).expect("support json");
    let decoded: BasicIndexOperatorStatus =
        serde_json::from_value(encoded["status"]["basic_index"].clone())
            .expect("BASIC structured JSON");
    let markdown = render::render_support_markdown(&bundle);
    let section = markdown
        .split("## BASIC index\n\n")
        .nth(1)
        .expect("BASIC section")
        .split("\n## Prune")
        .next()
        .expect("BASIC rows");
    let rows = crate::operator::status::basic_index_entries(&original.basic_index);
    let human = crate::operator::status::render_status(&original, StatusRenderMode::Human)
        .expect("human status");
    // Assert
    assert_eq!(status, original);
    assert_eq!(bundle.status.basic_index, original.basic_index);
    assert_eq!(decoded, original.basic_index);
    assert_eq!(
        section.lines().collect::<Vec<_>>(),
        rows.iter()
            .map(|(label, value)| format!("- {label}: {value}"))
            .collect::<Vec<_>>()
    );
    for (label, value) in rows {
        assert!(human.contains(&format!("{label}: {value}\n")));
    }
    assert_eq!(
        encoded["status"]["basic_index"]["retention"]["value"]["retained_record_key_value_bytes"],
        97
    );
    for expected in [
        "Failure: missing undo",
        "Retained: observed",
        "Init sync: earned",
        "Lag: 3 blocks",
        "Processed: 21",
        "Durable: 18",
    ] {
        assert!(section.contains(expected), "{expected}");
    }
}

type ReasonSetter = fn(&mut BasicIndexOperatorStatus, &str);

fn reason_boundaries() -> [(&'static str, ReasonSetter); 14] {
    [
        ("configured_indexing", |s, r| {
            s.configured_indexing = FieldAvailability::unavailable(r)
        }),
        ("configured_peer_serving", |s, r| {
            s.configured_peer_serving = FieldAvailability::unavailable(r)
        }),
        ("peer_scope", |s, r| {
            s.peer_scope = FieldAvailability::unavailable(r)
        }),
        ("operational_state", |s, r| {
            s.operational_state = FieldAvailability::unavailable(r)
        }),
        ("initial_sync_complete", |s, r| {
            s.initial_sync_complete = FieldAvailability::unavailable(r)
        }),
        ("accepted_height", |s, r| {
            s.maybe_accepted_height = FieldAvailability::unavailable(r)
        }),
        ("processed_height", |s, r| {
            s.maybe_processed_height = FieldAvailability::unavailable(r)
        }),
        ("safe_durable_height", |s, r| {
            s.maybe_safe_durable_height = FieldAvailability::unavailable(r)
        }),
        ("lag_blocks", |s, r| {
            s.maybe_lag_blocks = FieldAvailability::unavailable(r)
        }),
        ("failure", |s, r| {
            s.maybe_failure = FieldAvailability::unavailable(r)
        }),
        ("retained_availability", |s, r| {
            s.retained_availability = FieldAvailability::unavailable(r)
        }),
        ("retention", |s, r| {
            s.retention = FieldAvailability::unavailable(r)
        }),
        ("protection", |s, r| {
            s.protection = FieldAvailability::unavailable(r)
        }),
        ("next_action", |s, r| {
            s.next_action = FieldAvailability::unavailable(r)
        }),
    ]
}

#[test]
fn phase161_basic_support_rejects_private_material_at_every_free_text_boundary() {
    // Arrange
    let temp = TestDirectory::new("phase161-basic-private");
    let markers = [
        "script=phase161-51ac",
        "undo=phase161-dead",
        "filter_bytes=phase161-0102",
        "txid=phase161-aaaaaaaa",
        "block_hash=phase161-bbbb",
        "peer_id=phase161-private",
        "198.51.100.161:8333",
        "rpcpassword=phase161-secret",
        "cookie=phase161-auth",
        "storage_key=phase161-private",
        "/private/phase161-wallet",
        "lock=phase161-reserved",
        "\u{1b}[31mphase161-ansi",
        "\n## phase161-injected",
        "BASIC evidence not reported private-suffix",
    ];
    // Act / Assert
    for (field, set_reason) in reason_boundaries() {
        for marker in markers {
            let mut status = phase72_status();
            status.basic_index = failed_retained_status();
            set_reason(&mut status.basic_index, marker);
            let original = status.clone();
            let mut expected = original.basic_index.clone();
            set_reason(
                &mut expected,
                BasicIndexUnavailableReason::NotReported.as_str(),
            );
            let bundle = phase77_support_bundle_with_status(temp.path(), status.clone());
            let json = serde_json::to_value(&bundle).expect("support json");
            let json_text = serde_json::to_string(&bundle).expect("support JSON text");
            let markdown = render::render_support_markdown(&bundle);
            let human = crate::operator::status::render_status(&original, StatusRenderMode::Human)
                .expect("direct human rendering");
            assert_eq!(status, original, "source {field}");
            assert_eq!(bundle.status.basic_index, expected, "safe scalars {field}");
            assert_eq!(
                json["status"]["basic_index"][field]["value"]["reason"],
                BasicIndexUnavailableReason::NotReported.as_str(),
                "{field}"
            );
            let decoded: BasicIndexOperatorStatus =
                serde_json::from_value(json["status"]["basic_index"].clone())
                    .expect("safe BASIC JSON");
            assert_eq!(
                decoded, bundle.status.basic_index,
                "direct Rust normalization {field}"
            );
            assert!(!json_text.contains(marker), "JSON {field}: {marker:?}");
            assert!(!markdown.contains(marker), "Markdown {field}: {marker:?}");
            assert!(!human.contains(marker), "Human {field}: {marker:?}");
            assert!(!markdown.contains('\u{1b}'));
            assert!(markdown.contains("Unavailable: BASIC evidence not reported"));
        }
    }
}

#[test]
fn phase161_basic_support_preserves_exact_allowlisted_causes_at_every_boundary() {
    // Arrange
    let temp = TestDirectory::new("phase161-basic-safe-reasons");
    use BasicIndexUnavailableReason::*;
    let reasons = [
        NotReported,
        LiveUnavailable,
        RetentionIncomplete,
        RetentionFailed,
        RetentionBudgetExceeded,
        Backend,
        Integrity,
        Invalidated,
    ];
    // Act / Assert
    for (field, set_reason) in reason_boundaries() {
        for reason in reasons {
            let mut status = phase72_status();
            status.basic_index = failed_retained_status();
            set_reason(&mut status.basic_index, reason.as_str());
            let bundle = phase77_support_bundle_with_status(temp.path(), status.clone());
            let json = serde_json::to_value(&bundle).expect("support JSON");
            let markdown = render::render_support_markdown(&bundle);
            assert_eq!(bundle.status.basic_index, status.basic_index);
            assert_eq!(
                json["status"]["basic_index"][field]["value"]["reason"],
                reason.as_str()
            );
            assert!(markdown.contains(&format!("Unavailable: {}", reason.as_str())));
        }
    }
}

struct CapturedRpc {
    inner: Phase127SensitiveStatusRpc,
    network_captures: std::cell::Cell<u32>,
}

impl StatusRpcClient for CapturedRpc {
    fn get_network_info(&self) -> Result<GetNetworkInfoResponse, StatusRpcError> {
        self.inner.get_network_info()
    }
    fn get_open_bitcoin_network_status(
        &self,
    ) -> Result<OpenBitcoinNetworkStatusResponse, StatusRpcError> {
        self.network_captures.set(self.network_captures.get() + 1);
        self.inner.get_open_bitcoin_network_status()
    }
    fn get_blockchain_info(&self) -> Result<GetBlockchainInfoResponse, StatusRpcError> {
        self.inner.get_blockchain_info()
    }
    fn get_mempool_info(&self) -> Result<GetMempoolInfoResponse, StatusRpcError> {
        self.inner.get_mempool_info()
    }
    fn get_wallet_info(&self) -> Result<GetWalletInfoResponse, StatusRpcError> {
        self.inner.get_wallet_info()
    }
    fn get_balances(&self) -> Result<GetBalancesResponse, StatusRpcError> {
        self.inner.get_balances()
    }
}

#[test]
fn phase161_basic_support_actual_exports_preserve_one_live_capture_without_recollection() {
    // Arrange
    let temp = TestDirectory::new("phase161-basic-live-export");
    let fixture = phase72_status();
    let live_basic = failed_retained_status();
    let rpc = CapturedRpc {
        inner: Phase127SensitiveStatusRpc {
            network_status: OpenBitcoinNetworkStatusResponse {
                basic_index: live_basic.clone(),
                inbound: fixture.peers.inbound,
                relay: fixture.mempool.relay.clone(),
                block_relay: fixture.block_relay,
                chainstate_durability: fixture.chainstate_durability,
                metrics: fixture.metrics,
                mempool: fixture.mempool,
                prune: fixture.prune,
            },
        },
        network_captures: std::cell::Cell::new(0),
    };
    let input = phase127_status_collector_input(temp.path());
    let output_dir = temp.path().join("bundle");
    let args = SupportArgs {
        command: SupportCommand::Bundle(SupportBundleArgs {
            maybe_output_dir: Some(output_dir.clone()),
            maybe_live_smoke_report: None,
        }),
    };
    // Act
    let captured = collect_status_snapshot(&input, Some(&rpc));
    let original = captured.clone();
    let outcome = execute_support_command(
        &args,
        OperatorOutputFormat::Json,
        &input.config_resolution,
        captured.clone(),
    )
    .expect("real support export");
    let encoded: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(output_dir.join("support-evidence.json")).expect("actual JSON file"),
    )
    .expect("structured JSON");
    let markdown =
        fs::read_to_string(output_dir.join("support-evidence.md")).expect("actual Markdown file");
    let exported: BasicIndexOperatorStatus =
        serde_json::from_value(encoded["status"]["basic_index"].clone()).expect("BASIC JSON");
    // Assert
    assert_eq!(outcome.exit_code.code(), 0);
    assert_eq!(rpc.network_captures.get(), 1);
    assert_eq!(captured, original);
    assert_eq!(captured.basic_index, live_basic);
    assert_eq!(exported, live_basic);
    for (label, value) in crate::operator::status::basic_index_entries(&live_basic) {
        assert!(markdown.contains(&format!("- {label}: {value}\n")));
    }
}

#[test]
fn phase161_basic_support_actual_exports_normalize_every_private_reason() {
    // Arrange
    let temp = TestDirectory::new("phase161-basic-private-export");
    let input = phase127_status_collector_input(temp.path());
    let output_dir = temp.path().join("bundle");
    let args = SupportArgs {
        command: SupportCommand::Bundle(SupportBundleArgs {
            maybe_output_dir: Some(output_dir.clone()),
            maybe_live_smoke_report: None,
        }),
    };
    let private = "phase161-private-script-undo-filter-txid-block-peer-address-credential-key-path-lock\u{1b}[31m\n## injected";
    let mut status = phase72_status();
    for (_, set_reason) in reason_boundaries() {
        set_reason(&mut status.basic_index, private);
    }
    let original = status.clone();
    // Act
    execute_support_command(
        &args,
        OperatorOutputFormat::Json,
        &input.config_resolution,
        status.clone(),
    )
    .expect("private support export");
    let json_text =
        fs::read_to_string(output_dir.join("support-evidence.json")).expect("actual JSON");
    let encoded: serde_json::Value = serde_json::from_str(&json_text).expect("structured JSON");
    let markdown =
        fs::read_to_string(output_dir.join("support-evidence.md")).expect("actual Markdown");
    // Assert
    assert_eq!(status, original);
    for (field, _) in reason_boundaries() {
        assert_eq!(
            encoded["status"]["basic_index"][field]["value"]["reason"],
            BasicIndexUnavailableReason::NotReported.as_str()
        );
    }
    for text in [&json_text, &markdown] {
        assert!(!text.contains("phase161-private"));
        assert!(!text.contains("## injected"));
    }
    assert!(!markdown.contains('\u{1b}'));
    assert!(markdown.contains("## BASIC index"));
}

#[test]
fn phase161_basic_support_permission_guidance_separates_capability_and_readiness() {
    // Arrange
    let temp = TestDirectory::new("phase161-basic-permission-copy");
    let mut status = phase90_status_with_available_inbound();
    let FieldAvailability::Available(inbound) = &mut status.peers.inbound else {
        panic!("available fixture")
    };
    inbound.inactive_permission_effects = vec!["inactive_blockfilters".into()];
    let bundle = phase77_support_bundle_with_status(temp.path(), status);
    // Act
    let markdown = render::render_support_markdown(&bundle);
    // Assert
    assert!(markdown.contains("Read effective BASIC capability separately from index readiness."));
    assert!(!markdown.contains("bloom, and blockfilter permissions are recorded as inactive"));
}
