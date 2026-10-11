// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

use std::cell::RefCell;

use open_bitcoin_node::status::{
    BasicIndexFailureCategory, BasicIndexNextAction, BasicIndexOperationalState,
    BasicIndexOperatorStatus, BasicIndexProtection, BasicIndexProtectionRange,
    BasicIndexUnavailableReason, BasicPeerScopeStatus, BasicRetainedAvailability,
    BasicRetentionBasis, BasicRetentionMeasurement,
};
use serde_json::{Value, json};

use super::*;

struct RecordingClient {
    inner: FakeStatusRpcClient,
    calls: RefCell<Vec<&'static str>>,
    maybe_wire: Option<Value>,
}

impl RecordingClient {
    fn new(inner: FakeStatusRpcClient) -> Self {
        Self {
            inner,
            calls: RefCell::default(),
            maybe_wire: None,
        }
    }
}

impl StatusRpcClient for RecordingClient {
    fn get_network_info(&self) -> Result<GetNetworkInfoResponse, StatusRpcError> {
        self.calls.borrow_mut().push("getnetworkinfo");
        self.inner.get_network_info()
    }

    fn get_open_bitcoin_network_status(
        &self,
    ) -> Result<OpenBitcoinNetworkStatusResponse, StatusRpcError> {
        self.calls.borrow_mut().push("openbitcoinnetworkstatus");
        if let Some(wire) = &self.maybe_wire {
            return serde_json::from_value(wire.clone())
                .map_err(|_| StatusRpcError::new("malformed network response"));
        }
        self.inner.get_open_bitcoin_network_status()
    }

    fn get_blockchain_info(&self) -> Result<GetBlockchainInfoResponse, StatusRpcError> {
        self.calls.borrow_mut().push("getblockchaininfo");
        self.inner.get_blockchain_info()
    }

    fn get_mempool_info(&self) -> Result<GetMempoolInfoResponse, StatusRpcError> {
        self.calls.borrow_mut().push("getmempoolinfo");
        self.inner.get_mempool_info()
    }

    fn get_wallet_info(&self) -> Result<GetWalletInfoResponse, StatusRpcError> {
        self.calls.borrow_mut().push("getwalletinfo");
        self.inner.get_wallet_info()
    }

    fn get_balances(&self) -> Result<GetBalancesResponse, StatusRpcError> {
        self.calls.borrow_mut().push("getbalances");
        self.inner.get_balances()
    }
}

fn collected_evidence() -> BasicIndexOperatorStatus {
    BasicIndexOperatorStatus {
        configured_indexing: FieldAvailability::available(true),
        configured_peer_serving: FieldAvailability::available(false),
        peer_scope: FieldAvailability::available(BasicPeerScopeStatus {
            general_sessions: 0,
            explicit_sessions: 2,
            all_sessions: 1,
            implicit_sessions: 0,
            offered_sessions: 3,
        }),
        operational_state: FieldAvailability::available(BasicIndexOperationalState::Lagged),
        initial_sync_complete: FieldAvailability::available(true),
        maybe_accepted_height: FieldAvailability::available(Some(23)),
        maybe_processed_height: FieldAvailability::available(Some(21)),
        maybe_safe_durable_height: FieldAvailability::available(Some(18)),
        maybe_lag_blocks: FieldAvailability::available(Some(2)),
        maybe_failure: FieldAvailability::available(None),
        retained_availability: FieldAvailability::available(BasicRetainedAvailability::Observed),
        retention: FieldAvailability::available(BasicRetentionMeasurement {
            retained_record_count: 25,
            encoded_filter_payload_bytes: 157,
            retained_record_key_value_bytes: 2_817,
            basis: BasicRetentionBasis::ImmutableRecordKeysAndValuesIncludingStaleBranches,
        }),
        protection: FieldAvailability::available(BasicIndexProtection::InclusiveRange(
            BasicIndexProtectionRange::new(19, u32::MAX).expect("valid protection"),
        )),
        next_action: FieldAvailability::available(BasicIndexNextAction::ProcessedAheadOfDurable),
    }
}

fn client_with_evidence(basic_index: BasicIndexOperatorStatus) -> RecordingClient {
    let mut inner = FakeStatusRpcClient::running();
    inner
        .maybe_network_status
        .as_mut()
        .expect("network response")
        .basic_index = basic_index;
    RecordingClient::new(inner)
}

fn assert_single_collection(client: &RecordingClient) {
    assert_eq!(
        *client.calls.borrow(),
        vec![
            "getnetworkinfo",
            "getblockchaininfo",
            "getmempoolinfo",
            "getwalletinfo",
            "getbalances",
            "openbitcoinnetworkstatus",
        ]
    );
}

fn assert_all_unavailable(status: &BasicIndexOperatorStatus) {
    let encoded = serde_json::to_value(status).expect("BASIC JSON");
    for (name, value) in encoded.as_object().expect("BASIC section") {
        assert_eq!(value["state"], "unavailable", "{name} invented evidence");
    }
}

#[test]
fn phase161_cli_collection_preserves_earned_lag_processed_durable_and_retention() {
    // Arrange
    let evidence = collected_evidence();
    let client = client_with_evidence(evidence.clone());

    // Act
    let snapshot = collect_status_snapshot(&status_input(Vec::new()), Some(&client));

    // Assert
    assert_eq!(snapshot.basic_index, evidence);
    assert_single_collection(&client);
}

#[test]
fn phase161_cli_collection_legacy_omission_stays_unavailable() {
    // Arrange
    let mut client = client_with_evidence(collected_evidence());
    let mut wire = serde_json::to_value(
        client
            .inner
            .maybe_network_status
            .as_ref()
            .expect("response"),
    )
    .expect("wire");
    wire.as_object_mut().expect("object").remove("basic_index");
    client.maybe_wire = Some(wire);

    // Act
    let snapshot = collect_status_snapshot(&status_input(Vec::new()), Some(&client));

    // Assert
    assert_eq!(snapshot.basic_index, BasicIndexOperatorStatus::default());
    assert_all_unavailable(&snapshot.basic_index);
    assert_single_collection(&client);
}

#[test]
fn phase161_cli_collection_method_unavailable_stays_unavailable() {
    // Arrange
    let client = RecordingClient::new(FakeStatusRpcClient::network_status_failing(
        StatusRpcError::from_rpc_detail(RpcErrorDetail {
            code: RpcErrorCode::MethodNotFound,
            message: "unknown method".to_string(),
        }),
    ));

    // Act
    let snapshot = collect_status_snapshot(&status_input(Vec::new()), Some(&client));

    // Assert
    assert_all_unavailable(&snapshot.basic_index);
    assert_single_collection(&client);
}

#[test]
fn phase161_cli_collection_stopped_cannot_guess_configuration() {
    // Arrange
    let mut input = status_input(Vec::new());
    input.request.include_live_rpc = false;
    let client = client_with_evidence(collected_evidence());

    // Act
    let snapshot = collect_status_snapshot(&input, Some(&client));

    // Assert
    assert_eq!(snapshot.node.state, NodeRuntimeState::Stopped);
    assert_all_unavailable(&snapshot.basic_index);
    assert!(client.calls.borrow().is_empty());
}

#[test]
fn phase161_cli_collection_unreachable_auth_does_not_request_index() {
    // Arrange
    let client = RecordingClient::new(FakeStatusRpcClient::failing("401 unauthorized"));

    // Act
    let snapshot = collect_status_snapshot(&status_input(Vec::new()), Some(&client));

    // Assert
    assert_eq!(snapshot.node.state, NodeRuntimeState::Unreachable);
    assert_all_unavailable(&snapshot.basic_index);
    assert_eq!(*client.calls.borrow(), vec!["getnetworkinfo"]);
}

#[test]
fn phase161_cli_collection_network_failure_has_bounded_private_reason() {
    // Arrange
    let client = RecordingClient::new(FakeStatusRpcClient::network_status_failing(
        StatusRpcError::new("backend /private/wallet key=secret peer=192.0.2.1"),
    ));

    // Act
    let snapshot = collect_status_snapshot(&status_input(Vec::new()), Some(&client));

    // Assert
    assert_eq!(
        snapshot.basic_index,
        BasicIndexOperatorStatus::unavailable(BasicIndexUnavailableReason::LiveUnavailable,)
    );
    assert_all_unavailable(&snapshot.basic_index);
    assert_single_collection(&client);
}

#[test]
fn phase161_cli_collection_malformed_response_never_becomes_healthy() {
    // Arrange
    let mut client = client_with_evidence(collected_evidence());
    let mut wire = serde_json::to_value(
        client
            .inner
            .maybe_network_status
            .as_ref()
            .expect("response"),
    )
    .expect("wire");
    wire["basic_index"]["configured_indexing"] = json!({"state": "available", "value": "false"});
    client.maybe_wire = Some(wire);

    // Act
    let snapshot = collect_status_snapshot(&status_input(Vec::new()), Some(&client));

    // Assert
    assert_all_unavailable(&snapshot.basic_index);
    assert_single_collection(&client);
}

#[test]
fn phase161_cli_collection_failure_does_not_erase_retained_facts() {
    // Arrange
    let mut evidence = collected_evidence();
    evidence.operational_state = FieldAvailability::available(BasicIndexOperationalState::Failed);
    evidence.maybe_failure =
        FieldAvailability::available(Some(BasicIndexFailureCategory::RequiredUndo));
    evidence.next_action = FieldAvailability::available(BasicIndexNextAction::RequiredUndo);
    let client = client_with_evidence(evidence.clone());

    // Act
    let snapshot = collect_status_snapshot(&status_input(Vec::new()), Some(&client));

    // Assert
    assert_eq!(snapshot.basic_index, evidence);
    assert_single_collection(&client);
}

#[test]
fn phase161_cli_collection_wire_round_trip_preserves_exact_facts() {
    // Arrange
    let expected = collected_evidence();
    let mut client = client_with_evidence(expected.clone());
    client.maybe_wire = Some(
        serde_json::to_value(
            client
                .inner
                .maybe_network_status
                .as_ref()
                .expect("response"),
        )
        .expect("wire"),
    );

    // Act
    let snapshot = collect_status_snapshot(&status_input(Vec::new()), Some(&client));
    let wire = render_status(&snapshot, StatusRenderMode::Json).expect("status JSON");
    let decoded: OpenBitcoinStatusSnapshot =
        serde_json::from_str(&wire).expect("status round trip");

    // Assert
    assert_eq!(decoded.basic_index, expected);
    assert_single_collection(&client);
}
