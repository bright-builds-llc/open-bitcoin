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

use open_bitcoin_network::{
    BanReason, BanScope, InboundResourceEvent, MisbehaviorDecision, MisbehaviorKind,
    MisbehaviorResponse, PeerBanEntry,
};
use open_bitcoin_node::{
    core::wallet::AddressNetwork,
    logging::{INBOUND_PEER_POLICY_LOG_SOURCE, StructuredLogLevel, StructuredLogRecord},
    status::{FieldAvailability, InboundPeerPolicyEvent},
};
use std::{
    fs,
    path::PathBuf,
    process,
    sync::atomic::{AtomicU64, Ordering},
};

use crate::config::RuntimeConfig;

use super::ManagedRpcContext;

static NEXT_TEMP_DIR: AtomicU64 = AtomicU64::new(0);

#[test]
fn managed_rpc_context_builds_from_runtime_config() {
    // Arrange
    let runtime = RuntimeConfig {
        chain: AddressNetwork::Regtest,
        ..RuntimeConfig::default()
    };

    // Act
    let context = ManagedRpcContext::from_runtime_config(&runtime);
    let network_info = context.network_info();
    let wallet_info = context.wallet_info();
    let snapshot = context.blockchain_snapshot();

    // Assert
    assert_eq!(context.chain(), AddressNetwork::Regtest);
    assert_eq!(network_info.connected_peers, 0);
    assert_eq!(wallet_info.network, AddressNetwork::Regtest);
    assert!(snapshot.active_chain.is_empty());
}

#[test]
fn record_inbound_resource_event_appends_inbound_resource_governance_log_record() {
    // Arrange
    let data_dir = test_data_dir("resource-governance-log");
    let runtime = RuntimeConfig {
        chain: AddressNetwork::Regtest,
        maybe_data_dir: Some(data_dir.clone()),
        ..RuntimeConfig::default()
    };
    let mut context = ManagedRpcContext::from_runtime_config(&runtime);
    let event = InboundResourceEvent {
        outcome: "rejected".to_string(),
        reason: "payload checksum did not match message header".to_string(),
        label: "invalid_checksum".to_string(),
        source: "source_envelope_gate".to_string(),
        message: "inbound_message_resource_governance".to_string(),
        next_action: "payload_rejected".to_string(),
    };

    // Act
    context
        .record_inbound_resource_event_at(event, 1_777_225_022)
        .expect("append resource governance log record");

    // Assert
    let records = read_structured_log_records(&data_dir.join("logs"));
    assert_eq!(records.len(), 1);
    let record = &records[0];
    assert_eq!(record.source, "inbound_resource_governance");
    assert_eq!(record.level, StructuredLogLevel::Warn);
    assert!(record.message.contains("outcome=rejected"));
    assert!(
        record
            .message
            .contains("reason=payload checksum did not match message header")
    );
    assert!(record.message.contains("label=invalid_checksum"));
    assert!(record.message.contains("source=source_envelope_gate"));
    assert!(
        record
            .message
            .contains("message=inbound_message_resource_governance")
    );
    assert!(record.message.contains("next_action=payload_rejected"));
}

#[test]
fn record_inbound_resource_event_projects_current_inbound_status() {
    // Arrange
    let runtime = RuntimeConfig {
        chain: AddressNetwork::Regtest,
        ..RuntimeConfig::default()
    };
    let mut context = ManagedRpcContext::from_runtime_config(&runtime);
    let event = InboundResourceEvent {
        outcome: "rejected".to_string(),
        reason: "payload checksum did not match message header".to_string(),
        label: "invalid_checksum".to_string(),
        source: "source_envelope_gate".to_string(),
        message: "inbound_message_resource_governance".to_string(),
        next_action: "payload_rejected".to_string(),
    };

    // Act
    context.record_inbound_resource_event(event);
    let status = context.current_inbound_status();

    // Assert
    let FieldAvailability::Available(inbound) = status else {
        panic!("resource governance event should make inbound status available");
    };
    assert_eq!(inbound.payload_rejections, 1);
    assert_eq!(inbound.resource_pressure_events, 0);
    let FieldAvailability::Available(decision) = inbound.latest_resource_governance_decision else {
        panic!("latest resource decision should be available");
    };
    assert_eq!(decision.next_action, "payload_rejected");
}

#[test]
fn record_inbound_peer_policy_event_appends_inbound_peer_policy_log_record() {
    // Arrange
    let data_dir = test_data_dir("peer-policy-log");
    let runtime = RuntimeConfig {
        chain: AddressNetwork::Regtest,
        maybe_data_dir: Some(data_dir.clone()),
        ..RuntimeConfig::default()
    };
    let mut context = ManagedRpcContext::from_runtime_config(&runtime);
    let event = InboundPeerPolicyEvent {
        outcome: "ban_active".to_string(),
        reason: "127.0.0.1:18444".to_string(),
        label: "peer_id=42".to_string(),
        source: "source_peer_policy_runtime_bridge".to_string(),
        message: "raw_endpoint=192.0.2.1:8333 credential=fixture".to_string(),
    };

    // Act
    context
        .record_inbound_peer_policy_event_at(event, 1_777_225_024)
        .expect("append peer-policy log record");

    // Assert
    let records = read_structured_log_records(&data_dir.join("logs"));
    assert_eq!(records.len(), 1);
    let record = &records[0];
    assert_eq!(record.source, INBOUND_PEER_POLICY_LOG_SOURCE);
    assert_eq!(record.level, StructuredLogLevel::Warn);
    assert!(record.message.contains("outcome=ban_active"));
    assert!(
        record
            .message
            .contains("source=source_peer_policy_runtime_bridge")
    );
    assert!(record.message.contains("redacted_peer_policy_field"));
    for raw in [
        "127.0.0.1:18444",
        "peer_id=42",
        "192.0.2.1:8333",
        "credential=fixture",
    ] {
        assert!(!record.message.contains(raw));
    }
}

#[test]
fn record_peer_policy_runtime_decisions_append_sanitized_logs_automatically() {
    // Arrange
    let data_dir = test_data_dir("peer-policy-runtime-auto-log");
    let runtime = RuntimeConfig {
        chain: AddressNetwork::Regtest,
        maybe_data_dir: Some(data_dir.clone()),
        ..RuntimeConfig::default()
    };
    let mut context = ManagedRpcContext::from_runtime_config(&runtime);
    let ban_scope = BanScope::Address(std::net::IpAddr::from([203, 0, 113, 32]));
    let discouragement_scope = BanScope::Address(std::net::IpAddr::from([203, 0, 113, 33]));
    let misbehavior = MisbehaviorDecision {
        peer_label: "peer-raw-42 credential=cookie".to_string(),
        kind: MisbehaviorKind::MalformedMessage,
        score: 500,
        response: MisbehaviorResponse::Discourage,
    };

    // Act
    context.record_peer_policy_ban(peer_policy_entry(ban_scope.clone(), 300), 150);
    context.record_peer_policy_discouragement(peer_policy_entry(discouragement_scope, 300), 150);
    context.record_peer_policy_misbehavior(misbehavior);
    context.record_peer_policy_unban(&ban_scope, 160);

    // Assert
    let records = read_structured_log_records(&data_dir.join("logs"));
    assert_eq!(records.len(), 4);
    for record in &records {
        assert_eq!(record.source, INBOUND_PEER_POLICY_LOG_SOURCE);
        assert_eq!(record.level, StructuredLogLevel::Warn);
    }
    let messages = records
        .iter()
        .map(|record| record.message.as_str())
        .collect::<Vec<_>>()
        .join("\n");
    for required in [
        "outcome=ban_active",
        "outcome=discouragement_active",
        "outcome=discouraged",
        "outcome=unbanned",
        "source=source_peer_policy_runtime_bridge",
        "source=source_misbehavior_policy",
        "source=source_unban_policy",
    ] {
        assert!(
            messages.contains(required),
            "missing peer-policy log field: {required}"
        );
    }
    for raw in [
        "203.0.113.32",
        "203.0.113.33",
        "peer-raw-42",
        "credential=cookie",
        "peer_id=",
        "raw_endpoint",
        "cookie=",
    ] {
        assert!(
            !messages.contains(raw),
            "raw peer-policy data leaked: {raw}"
        );
    }
}

#[test]
fn record_inbound_peer_policy_runtime_decision_projects_status_and_log() {
    // Arrange
    let data_dir = test_data_dir("peer-policy-runtime-log");
    let runtime = RuntimeConfig {
        chain: AddressNetwork::Regtest,
        maybe_data_dir: Some(data_dir.clone()),
        ..RuntimeConfig::default()
    };
    let mut context = ManagedRpcContext::from_runtime_config(&runtime);
    context.record_peer_policy_ban(
        peer_policy_entry(
            BanScope::Address(std::net::IpAddr::from([203, 0, 113, 31])),
            300,
        ),
        150,
    );

    // Act
    let recorded = context
        .record_latest_inbound_peer_policy_event_at(1_777_225_025)
        .expect("append latest peer-policy log record");
    let status = context.current_inbound_status();

    // Assert
    assert!(recorded);
    let FieldAvailability::Available(inbound) = status else {
        panic!("peer-policy bridge evidence should make inbound status available");
    };
    assert_eq!(inbound.active_bans, 1);
    let FieldAvailability::Available(decision) = inbound.latest_peer_policy_decision else {
        panic!("latest peer-policy decision should be available");
    };
    assert_eq!(decision.label, "ban_active");
    assert_eq!(decision.source, "source_peer_policy_runtime_bridge");

    let records = read_structured_log_records(&data_dir.join("logs"));
    assert_eq!(records.len(), 2);
    let record = &records[0];
    assert_eq!(record.source, "inbound_peer_policy");
    assert!(record.message.contains("outcome=ban_active"));
    assert!(
        record
            .message
            .contains("source=source_peer_policy_runtime_bridge")
    );
    assert!(!record.message.contains("203.0.113.31"));
    assert!(!record.message.contains("peer_id="));
}

#[test]
fn current_inbound_status_projects_runtime_peer_policy_bridge() {
    // Arrange
    let runtime = RuntimeConfig {
        chain: AddressNetwork::Regtest,
        ..RuntimeConfig::default()
    };
    let mut context = ManagedRpcContext::from_runtime_config(&runtime);
    let scope = BanScope::Address(std::net::IpAddr::from([203, 0, 113, 30]));
    let decision = MisbehaviorDecision {
        peer_label: "peer-protected".to_string(),
        kind: MisbehaviorKind::MalformedMessage,
        score: 500,
        response: MisbehaviorResponse::ProtectedNoAction,
    };

    // Act
    context.record_peer_policy_ban(peer_policy_entry(scope.clone(), 300), 150);
    context.record_peer_policy_unban(&scope, 160);
    context.record_peer_policy_misbehavior(decision);
    let status = context.current_inbound_status();

    // Assert
    let FieldAvailability::Available(inbound) = status else {
        panic!("peer-policy bridge evidence should make inbound status available");
    };
    assert_eq!(inbound.active_bans, 1);
    assert_eq!(inbound.manual_unbans, 1);
    assert_eq!(inbound.misbehavior_observations, 1);
    assert_eq!(inbound.protected_no_actions, 1);
    let FieldAvailability::Available(decision) = inbound.latest_peer_policy_decision else {
        panic!("latest peer-policy decision should be available");
    };
    assert_eq!(decision.outcome, "unbanned");
    assert_eq!(decision.label, "unbanned");
    assert_eq!(decision.source, "source_unban_policy");
}

#[test]
fn current_inbound_status_remains_unavailable_without_peer_policy_bridge_evidence() {
    // Arrange
    let context = ManagedRpcContext::for_local_operator(AddressNetwork::Regtest);

    // Act
    let status = context.current_inbound_status();

    // Assert
    assert!(matches!(status, FieldAvailability::Unavailable { .. }));
}

fn test_data_dir(name: &str) -> PathBuf {
    let mut path = std::env::temp_dir();
    path.push(format!(
        "open-bitcoin-context-{name}-{}-{}",
        process::id(),
        NEXT_TEMP_DIR.fetch_add(1, Ordering::SeqCst)
    ));
    let _ = fs::remove_dir_all(&path);
    fs::create_dir_all(&path).expect("create context test data dir");
    path
}

fn peer_policy_entry(scope: BanScope, expires_at_unix_seconds: i64) -> PeerBanEntry {
    PeerBanEntry {
        scope,
        reason: BanReason::Manual,
        created_at_unix_seconds: 100,
        expires_at_unix_seconds,
        source: "peer_policy_runtime_bridge",
    }
}

fn read_structured_log_records(log_dir: &std::path::Path) -> Vec<StructuredLogRecord> {
    let mut records = Vec::new();
    for entry in fs::read_dir(log_dir).expect("read log directory") {
        let path = entry.expect("read log entry").path();
        let Some(file_name) = path.file_name().and_then(|name| name.to_str()) else {
            continue;
        };
        if !file_name.starts_with("open-bitcoin-runtime-") {
            continue;
        }
        let contents = fs::read_to_string(&path).expect("read structured log file");
        for line in contents.lines() {
            records.push(serde_json::from_str(line).expect("decode structured log record"));
        }
    }
    records
}
