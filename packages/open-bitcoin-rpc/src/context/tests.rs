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

use open_bitcoin_network::InboundResourceEvent;
use open_bitcoin_node::{
    core::wallet::AddressNetwork,
    logging::{StructuredLogLevel, StructuredLogRecord},
    status::FieldAvailability,
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
