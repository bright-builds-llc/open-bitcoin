// Parity breadcrumbs:
// - packages/bitcoin-knots/src/bitcoind.cpp
// - packages/bitcoin-knots/src/rpc/protocol.h
// - packages/bitcoin-knots/src/rpc/request.cpp
// - packages/bitcoin-knots/src/rpc/server.cpp
// - packages/bitcoin-knots/src/rpc/blockchain.cpp
// - packages/bitcoin-knots/src/rpc/mempool.cpp
// - packages/bitcoin-knots/src/rpc/net.cpp
// - packages/bitcoin-knots/src/rpc/rawtransaction.cpp
// - packages/bitcoin-knots/test/functional/interface_rpc.py

use super::chain_fixtures::empty_context;
use super::network_fixtures::*;
use super::*;

#[test]
fn open_bitcoin_network_status_returns_available_inbound_evidence() {
    // Arrange
    let mut context = inbound_context(4, 0);
    context
        .record_inbound_admission(7, "127.0.0.1:18444".to_string(), false)
        .expect("authoritative inbound admission");
    context
        .record_inbound_admission(8, "127.0.0.1:18444".to_string(), false)
        .expect("authoritative inbound admission");
    context
        .record_inbound_admission(7, "127.0.0.1:18445".to_string(), false)
        .expect("authoritative inbound admission");

    // Act
    let status = dispatch(
        &mut context,
        MethodCall::OpenBitcoinNetworkStatus(OpenBitcoinNetworkStatusRequest::default()),
    )
    .expect("network status");

    // Assert
    let inbound = &status["inbound"];
    assert_eq!(inbound["state"], json!("available"));
    assert_eq!(inbound["value"]["listener_state"], json!("listening"));
    assert_eq!(inbound["value"]["preflight_reason"], json!("ready"));
    assert_eq!(inbound["value"]["admitted_inbound_peers"], json!(1));
    assert_eq!(inbound["value"]["rejected_inbound_peers"], json!(2));
    assert_eq!(inbound["value"]["handshake"]["established"], json!(1));
    assert_eq!(inbound["value"]["duplicate_rejects"], json!(2));
    assert_eq!(inbound["value"]["self_connection_rejects"], json!(0));
    assert_eq!(inbound["value"]["cap_rejects"], json!(0));
    assert_eq!(inbound["value"]["reserved_slot_rejects"], json!(0));
    assert_eq!(
        inbound["value"]["latest_admission_event"]["value"]["reason"],
        json!("duplicate_peer_id")
    );
    assert_eq!(inbound["value"]["permissioned_inbound_peers"], json!(0));
    assert_eq!(inbound["value"]["protected_inbound_peers"], json!(0));
    assert_eq!(
        inbound["value"]["permission_class"],
        json!("ordinary_inbound")
    );
    assert_eq!(inbound["value"]["active_permission_effects"], json!([]));
    assert_eq!(inbound["value"]["inactive_permission_effects"], json!([]));
    assert_eq!(
        inbound["value"]["latest_permission_decision"]["state"],
        json!("unavailable")
    );
    assert_eq!(inbound["value"]["eviction_candidates_evaluated"], json!(1));
    assert_eq!(inbound["value"]["disconnects_requested"], json!(1));
    assert_eq!(
        inbound["value"]["latest_peer_policy_decision"]["state"],
        json!("available")
    );
    assert_eq!(
        inbound["value"]["latest_peer_policy_decision"]["value"]["label"],
        json!("eviction_candidate_selected")
    );
    assert_eq!(
        status["relay"]["outcome_counters"]["state"],
        json!("implemented")
    );
    assert_eq!(
        status["relay"]["outcome_counters"]["value"]["accepted_count"],
        json!(0)
    );
    assert_eq!(
        status["relay"]["mempool_admission"]["state"],
        json!("unavailable")
    );
    assert_eq!(
        status["relay"]["public_relay"]["state"],
        json!("intentionally_different")
    );
    assert_eq!(
        status["block_relay"]["block_serving"]["activation"]["state"],
        json!("unavailable")
    );
    assert_eq!(
        status["block_relay"]["negotiation"]["value"]["version2_high_bandwidth_count"],
        json!(0)
    );
}

#[test]
fn authoritative_operator_snapshot_preserves_network_status_schema_and_provenance() {
    // Arrange
    let mut context = inbound_context(4, 0);
    context
        .record_inbound_admission(7, "127.0.0.1:18444".to_string(), false)
        .expect("authoritative inbound admission");

    // Act
    let snapshot = context
        .authoritative_operator_snapshot()
        .expect("owned authoritative operator snapshot");
    let status = dispatch(
        &mut context,
        MethodCall::OpenBitcoinNetworkStatus(OpenBitcoinNetworkStatusRequest::default()),
    )
    .expect("network status");

    // Assert
    assert_eq!(
        status
            .as_object()
            .expect("status object")
            .keys()
            .collect::<Vec<_>>(),
        vec!["block_relay", "inbound", "metrics", "relay"]
    );
    assert_eq!(
        status["inbound"],
        serde_json::to_value(snapshot.inbound()).expect("inbound snapshot")
    );
    assert_eq!(
        status["relay"],
        serde_json::to_value(snapshot.relay()).expect("relay snapshot")
    );
    assert_eq!(
        status["block_relay"],
        serde_json::to_value(snapshot.block_relay()).expect("block-relay snapshot")
    );
}

#[test]
fn open_bitcoin_network_status_includes_block_relay_projection() {
    let mut context = empty_context();

    let status = dispatch(
        &mut context,
        MethodCall::OpenBitcoinNetworkStatus(OpenBitcoinNetworkStatusRequest::default()),
    )
    .expect("network status");

    assert_eq!(
        status["block_relay"]["block_serving"]["activation"]["state"],
        json!("unavailable")
    );
    assert_eq!(
        status["block_relay"]["cleanup"]["value"]["compact_cleanup_count"],
        json!(0)
    );
}

#[test]
fn open_bitcoin_network_status_projects_listener_activation_before_admissions() {
    // Arrange
    let mut context = empty_context();
    context
        .set_inbound_listener_evidence(InboundListenerEvidence {
            listener_state: "listening".to_string(),
            preflight_reason: "ready".to_string(),
            bound_endpoints: vec!["127.0.0.1:18444".to_string()],
            admitted_inbound_peers: 0,
            rejected_inbound_peers: 0,
            resource_rejections: 0,
            timeout_disconnects: 0,
            churn_rejections: 0,
            reconnect_suppressions: 0,
            maybe_admission_reject_reason: None,
            maybe_latest_admission_event: Some("ready".to_string()),
            maybe_latest_resource_event: None,
        })
        .expect("authoritative listener evidence");

    // Act
    let status = dispatch(
        &mut context,
        MethodCall::OpenBitcoinNetworkStatus(OpenBitcoinNetworkStatusRequest::default()),
    )
    .expect("network status");

    // Assert
    let inbound = &status["inbound"];
    assert_eq!(inbound["state"], json!("available"));
    assert_eq!(inbound["value"]["listener_state"], json!("listening"));
    assert_eq!(inbound["value"]["preflight_reason"], json!("ready"));
    assert_eq!(
        inbound["value"]["bound_endpoints"],
        json!(["127.0.0.1:18444"])
    );
    assert_eq!(inbound["value"]["admitted_inbound_peers"], json!(0));
    assert_eq!(inbound["value"]["rejected_inbound_peers"], json!(0));
    assert_eq!(
        inbound["value"]["latest_admission_event"]["value"]["reason"],
        json!("ready")
    );
}

#[test]
fn open_bitcoin_network_status_projects_address_boundary_evidence_without_raw_details() {
    // Arrange
    let mut context = address_boundary_context();
    let peer_id = 9_206_101;
    let now_unix_seconds = 1_700_000_000;
    context
        .set_inbound_listener_evidence(InboundListenerEvidence {
            listener_state: "listening".to_string(),
            preflight_reason: "ready".to_string(),
            bound_endpoints: vec!["8.8.8.8:18444".to_string(), "127.0.0.1:18445".to_string()],
            admitted_inbound_peers: 0,
            rejected_inbound_peers: 0,
            resource_rejections: 0,
            timeout_disconnects: 0,
            churn_rejections: 0,
            reconnect_suppressions: 0,
            maybe_admission_reject_reason: None,
            maybe_latest_admission_event: Some("ready".to_string()),
            maybe_latest_resource_event: None,
        })
        .expect("authoritative listener evidence");
    context
        .record_inbound_admission_for_remote_addr(
            peer_id,
            "127.0.0.1:52061".parse().expect("permissioned remote"),
            false,
        )
        .expect("authoritative inbound admission");
    context
        .receive_network_message(
            peer_id,
            WireNetworkMessage::Addr(AddressList {
                addresses: vec![
                    address_announcement(
                        now_unix_seconds,
                        public_ipv4_network_address(9, 9, 9, 9, 8333),
                    ),
                    address_announcement(
                        now_unix_seconds,
                        public_ipv4_network_address(10, 0, 0, 1, 8333),
                    ),
                ],
            }),
            now_unix_seconds as i64,
        )
        .expect("addr evidence should be recorded");
    let first_getaddr_response = context
        .receive_network_message(
            peer_id,
            WireNetworkMessage::GetAddr,
            now_unix_seconds as i64 + 1,
        )
        .expect("first getaddr should be served");
    let second_getaddr_response = context
        .receive_network_message(
            peer_id,
            WireNetworkMessage::GetAddr,
            now_unix_seconds as i64 + 2,
        )
        .expect("second getaddr should be suppressed");

    // Act
    let status = dispatch(
        &mut context,
        MethodCall::OpenBitcoinNetworkStatus(OpenBitcoinNetworkStatusRequest::default()),
    )
    .expect("network status");

    // Assert
    assert!(matches!(
        first_getaddr_response.as_slice(),
        [WireNetworkMessage::Addr(addresses)] if !addresses.addresses.is_empty()
    ));
    assert!(second_getaddr_response.is_empty());
    let inbound = &status["inbound"]["value"];
    assert_eq!(
        inbound["local_advertisement_candidates"],
        json!([{
            "source": "source_local_listener",
            "network_kind": "ipv4",
            "routability": "publicly_routable",
            "freshness": "fresh",
            "services_bits": 9,
            "port": 18444,
            "persistence_eligible": false
        }])
    );
    assert_eq!(
        inbound["suppressed_advertisements"][0]["label"],
        json!("advertise_suppressed")
    );
    assert_eq!(
        inbound["suppressed_advertisements"][0]["reason"],
        json!("not_publicly_routable")
    );
    assert_eq!(inbound["getaddr_responses_served"], json!(1));
    assert_eq!(inbound["getaddr_requests_suppressed"], json!(1));
    assert_eq!(inbound["learned_address_entries"], json!(1));
    assert_eq!(inbound["learned_address_rejections"], json!(1));
    assert_eq!(
        inbound["latest_address_decision"]["value"]["label"],
        json!("getaddr_suppressed")
    );
    assert_eq!(
        inbound["latest_address_decision"]["value"]["reason"],
        json!("already_served")
    );
    let address_evidence = json!({
        "local_advertisement_candidates": inbound["local_advertisement_candidates"],
        "suppressed_advertisements": inbound["suppressed_advertisements"],
        "getaddr_responses_served": inbound["getaddr_responses_served"],
        "getaddr_requests_suppressed": inbound["getaddr_requests_suppressed"],
        "learned_address_entries": inbound["learned_address_entries"],
        "learned_address_rejections": inbound["learned_address_rejections"],
        "latest_address_decision": inbound["latest_address_decision"],
    });
    let serialized_address_evidence =
        serde_json::to_string(&address_evidence).expect("serialize address evidence");
    for forbidden in [
        "operator-private-addr-secret",
        "8.8.8.8:18444",
        "127.0.0.1:18445",
        "127.0.0.1",
        "8.8.8.8",
        "9.9.9.9",
        "10.0.0.1",
        "9206101",
        "address_bytes",
        "raw_permission",
        "raw_config",
        "class_name",
        "00000000000000000000ffff08080808",
    ] {
        assert!(
            !serialized_address_evidence.contains(forbidden),
            "address evidence exposed raw detail {forbidden}"
        );
    }
}
