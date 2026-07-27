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
fn open_bitcoin_network_status_preserves_unavailable_reason() {
    // Arrange
    let mut context = empty_context();

    // Act
    let status = dispatch(
        &mut context,
        MethodCall::OpenBitcoinNetworkStatus(OpenBitcoinNetworkStatusRequest::default()),
    )
    .expect("network status");

    // Assert
    assert_eq!(status["inbound"]["state"], json!("unavailable"));
    assert_eq!(
        status["inbound"]["value"]["reason"],
        json!(INBOUND_STATUS_UNAVAILABLE_REASON)
    );
}

#[test]
fn open_bitcoin_network_status_reports_permission_evidence_without_raw_class_names() {
    // Arrange
    let mut context = permission_context(vec![
        parsed_permission_class(
            "operator-loopback-relay-like",
            "127.0.0.1",
            &[
                "in",
                "download",
                "addr",
                "relay",
                "forcerelay",
                "mempool",
                "bloomfilter",
                "blockfilters",
            ],
        ),
        parsed_permission_class(
            "operator-loopback-protected",
            "127.0.0.2",
            &["in", "noban", "forceinbound"],
        ),
    ]);
    context
        .record_inbound_admission_for_remote_addr(
            31,
            "127.0.0.1:50031".parse().expect("permissioned remote"),
            false,
        )
        .expect("authoritative inbound admission");
    context
        .record_inbound_admission_for_remote_addr(
            32,
            "127.0.0.2:50032".parse().expect("protected remote"),
            false,
        )
        .expect("authoritative inbound admission");

    // Act
    let status = dispatch(
        &mut context,
        MethodCall::OpenBitcoinNetworkStatus(OpenBitcoinNetworkStatusRequest::default()),
    )
    .expect("network status");
    let serialized = serde_json::to_string(&status).expect("status json");

    // Assert
    let inbound = &status["inbound"]["value"];
    assert_eq!(inbound["permissioned_inbound_peers"], json!(1));
    assert_eq!(inbound["protected_inbound_peers"], json!(1));
    assert_eq!(inbound["permission_class"], json!("protected_inbound"));
    assert_eq!(
        inbound["active_permission_effects"],
        json!([
            "admission_protected",
            "eviction_policy_protected",
            "misbehavior_policy_protected",
            "address_response_policy_input",
            "download_serving_policy_input"
        ])
    );
    assert_eq!(
        inbound["inactive_permission_effects"],
        json!(["inactive_bloomfilter", "inactive_blockfilters"])
    );
    assert_eq!(
        inbound["latest_permission_decision"]["value"]["permission_class"],
        json!("protected_inbound")
    );
    assert_eq!(
        inbound["latest_admission_event"]["value"]["slot_class"],
        json!("reserved")
    );
    assert_eq!(
        inbound["latest_permission_decision"]["value"]["active_permission_effects"],
        json!([
            "admission_protected",
            "eviction_policy_protected",
            "misbehavior_policy_protected",
            "download_serving_policy_input"
        ])
    );
    assert!(!serialized.contains("operator-loopback-relay-like"));
    assert!(!serialized.contains("operator-loopback-protected"));
}

#[test]
fn open_bitcoin_network_status_reports_cap_and_reserved_slot_rejections() {
    // Arrange
    let mut cap_context = inbound_context(1, 0);
    cap_context
        .record_inbound_admission(11, "127.0.0.1:18444".to_string(), false)
        .expect("authoritative inbound admission");
    cap_context
        .record_inbound_admission(12, "127.0.0.1:18445".to_string(), false)
        .expect("authoritative inbound admission");
    let mut reserved_context = inbound_context(2, 1);
    reserved_context
        .record_inbound_admission(21, "127.0.0.1:18444".to_string(), false)
        .expect("authoritative inbound admission");
    reserved_context
        .record_inbound_admission(22, "127.0.0.1:18445".to_string(), false)
        .expect("authoritative inbound admission");
    let mut protected_reserved_context = permission_context_with_limits(
        vec![parsed_permission_class(
            "operator-loopback-protected",
            "127.0.0.1",
            &["in", "noban", "forceinbound"],
        )],
        1,
        1,
    );
    protected_reserved_context
        .record_inbound_admission_for_remote_addr(
            31,
            "127.0.0.1:50031".parse().expect("first protected peer"),
            false,
        )
        .expect("authoritative inbound admission");
    protected_reserved_context
        .record_inbound_admission_for_remote_addr(
            32,
            "127.0.0.1:50032".parse().expect("second protected peer"),
            false,
        )
        .expect("authoritative inbound admission");

    // Act
    let cap_status = dispatch(
        &mut cap_context,
        MethodCall::OpenBitcoinNetworkStatus(OpenBitcoinNetworkStatusRequest::default()),
    )
    .expect("cap status");
    let reserved_status = dispatch(
        &mut reserved_context,
        MethodCall::OpenBitcoinNetworkStatus(OpenBitcoinNetworkStatusRequest::default()),
    )
    .expect("reserved status");
    let protected_reserved_status = dispatch(
        &mut protected_reserved_context,
        MethodCall::OpenBitcoinNetworkStatus(OpenBitcoinNetworkStatusRequest::default()),
    )
    .expect("protected reserved status");

    // Assert
    assert_eq!(cap_status["inbound"]["value"]["cap_rejects"], json!(1));
    assert_eq!(
        cap_status["inbound"]["value"]["latest_admission_event"]["value"]["reason"],
        json!("cap_reached")
    );
    assert_eq!(
        reserved_status["inbound"]["value"]["reserved_slot_rejects"],
        json!(1)
    );
    assert_eq!(
        reserved_status["inbound"]["value"]["latest_admission_event"]["value"]["reason"],
        json!("reserved_slot_unavailable")
    );
    assert_eq!(
        reserved_status["inbound"]["value"]["latest_admission_event"]["value"]["slot_class"],
        json!("ordinary")
    );
    assert_eq!(
        protected_reserved_status["inbound"]["value"]["latest_admission_event"]["value"]["reason"],
        json!("reserved_slot_unavailable")
    );
    assert_eq!(
        protected_reserved_status["inbound"]["value"]["latest_admission_event"]["value"]["slot_class"],
        json!("reserved")
    );
    assert_eq!(
        protected_reserved_status["inbound"]["value"]["latest_permission_decision"]["state"],
        json!("unavailable")
    );
}

#[test]
fn open_bitcoin_network_status_latest_event_updates_after_rejection_then_admission() {
    // Arrange
    let mut context = inbound_context(2, 0);
    context
        .record_inbound_admission(41, "127.0.0.1:18444".to_string(), false)
        .expect("authoritative inbound admission");
    context
        .record_inbound_admission(42, "127.0.0.1:18444".to_string(), false)
        .expect("authoritative inbound admission");
    context
        .record_inbound_admission(43, "127.0.0.1:18445".to_string(), false)
        .expect("authoritative inbound admission");

    // Act
    let status = dispatch(
        &mut context,
        MethodCall::OpenBitcoinNetworkStatus(OpenBitcoinNetworkStatusRequest::default()),
    )
    .expect("network status");

    // Assert
    let inbound = &status["inbound"]["value"];
    assert_eq!(inbound["admitted_inbound_peers"], json!(2));
    assert_eq!(inbound["rejected_inbound_peers"], json!(1));
    assert_eq!(
        inbound["latest_admission_event"]["value"]["outcome"],
        json!("admitted")
    );
    assert_eq!(
        inbound["latest_admission_event"]["value"]["reason"],
        json!("admitted")
    );
}

#[test]
fn open_bitcoin_network_status_records_runtime_self_connection_rejection() {
    // Arrange
    let mut context = inbound_context(2, 0);
    context
        .record_inbound_admission(51, "127.0.0.1:18451".to_string(), false)
        .expect("authoritative inbound admission");

    // Act
    let error = context
        .receive_network_message(
            51,
            WireNetworkMessage::Version(VersionMessage {
                nonce: 0,
                ..VersionMessage::default()
            }),
            1,
        )
        .expect_err("self-connection should disconnect admitted inbound peer");
    let status = dispatch(
        &mut context,
        MethodCall::OpenBitcoinNetworkStatus(OpenBitcoinNetworkStatusRequest::default()),
    )
    .expect("network status");

    // Assert
    assert_eq!(error.to_string(), "peer 51 connected to self");
    let inbound = &status["inbound"]["value"];
    assert_eq!(inbound["rejected_inbound_peers"], json!(1));
    assert_eq!(inbound["self_connection_rejects"], json!(1));
    assert_eq!(
        inbound["latest_admission_event"]["value"]["outcome"],
        json!("rejected")
    );
    assert_eq!(
        inbound["latest_admission_event"]["value"]["reason"],
        json!("self_connection")
    );
    assert_eq!(
        inbound["latest_admission_event"]["value"]["slot_class"],
        json!("ordinary")
    );
    assert_eq!(
        inbound["latest_permission_decision"]["state"],
        json!("unavailable")
    );
}

#[test]
fn open_bitcoin_network_status_get_network_info_omits_open_bitcoin_inbound_status_details() {
    // Arrange
    let mut context = node_context_with_chain_and_mempool();
    context
        .record_inbound_admission(17, "127.0.0.1:18447".to_string(), false)
        .expect("authoritative inbound admission");
    let regression_scope =
        "getnetworkinfo local_advertisement_candidates latest_address_decision regression";

    // Act
    let network = dispatch(
        &mut context,
        MethodCall::GetNetworkInfo(GetNetworkInfoRequest::default()),
    )
    .expect("network");
    let serialized = serde_json::to_string(&network).expect("serialize network info");

    // Assert
    assert_eq!(network["connections_in"], json!(2));
    for forbidden in [
        "listener_state",
        "preflight_reason",
        "admission",
        "duplicate_rejects",
        "self_connection_rejects",
        "reserved_slot_rejects",
        "cap_rejects",
        "permission_class",
        "permissioned_inbound_peers",
        "protected_inbound_peers",
        "active_permission_effects",
        "inactive_permission_effects",
        "latest_permission_decision",
        "local_advertisement_candidates",
        "suppressed_advertisements",
        "getaddr_responses_served",
        "getaddr_requests_suppressed",
        "learned_address_entries",
        "learned_address_rejections",
        "latest_address_decision",
        "eviction_candidates_evaluated",
        "disconnects_requested",
        "discouraged_peers",
        "active_bans",
        "expired_bans",
        "manual_unbans",
        "misbehavior_observations",
        "protected_no_actions",
        "latest_peer_policy_decision",
        "outcome_counters",
        "accepted_count",
        "rebroadcast_deferred_count",
        "public_relay",
        "mempool_admission",
    ] {
        assert!(
            !serialized.contains(forbidden),
            "{regression_scope}: baseline method exposed {forbidden}"
        );
    }
}
