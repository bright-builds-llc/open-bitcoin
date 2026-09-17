// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

use serde_json::{Value, json};

use super::{
    CHAINSTATE_DURABILITY_UNAVAILABLE_REASON, CacheSizeLabel, ChainstateDurabilityEvidence,
    CoinsRecoveryOutcome, LastFlushReasonLabel, ReadinessLabel, ServingStatusLabel, WriteKindLabel,
};
use crate::status::FieldAvailability;

fn available_fixture() -> ChainstateDurabilityEvidence {
    ChainstateDurabilityEvidence {
        cache_size: CacheSizeLabel::Ok,
        last_flush_reason: LastFlushReasonLabel::Periodic,
        write_kind: WriteKindLabel::Sync,
        readiness: ReadinessLabel::ReadyToFlush,
        cache_bytes: 1_024,
        cache_byte_limit: 8_192,
        recovery_outcome: CoinsRecoveryOutcome::Replayed,
        maybe_coins_best_block_height: Some(840_004),
        maybe_coins_best_block_hash: Some("11".repeat(32)),
        last_serving_status: ServingStatusLabel::Unavailable,
        last_payload_present: false,
        last_index_known: true,
        last_validated_on_active_chain: true,
        available_count: 0,
        unavailable_count: 1,
        index_known_without_payload_count: 1,
    }
}

fn fail_closed_without_tip() -> ChainstateDurabilityEvidence {
    ChainstateDurabilityEvidence {
        recovery_outcome: CoinsRecoveryOutcome::FailClosed,
        maybe_coins_best_block_height: None,
        maybe_coins_best_block_hash: None,
        ..available_fixture()
    }
}

fn interrupted_without_tip() -> ChainstateDurabilityEvidence {
    ChainstateDurabilityEvidence {
        recovery_outcome: CoinsRecoveryOutcome::Interrupted,
        maybe_coins_best_block_height: None,
        maybe_coins_best_block_hash: None,
        ..available_fixture()
    }
}

fn collect_json_keys(value: &Value) -> Vec<String> {
    match value {
        Value::Object(map) => map.keys().cloned().collect(),
        _ => Vec::new(),
    }
}

#[test]
fn chainstate_durability_default_unavailable_uses_stable_reason() {
    // Arrange
    let expected = FieldAvailability::unavailable(CHAINSTATE_DURABILITY_UNAVAILABLE_REASON);

    // Act
    let via_helper = ChainstateDurabilityEvidence::default_unavailable();
    let via_default = FieldAvailability::<ChainstateDurabilityEvidence>::default();

    // Assert
    assert_eq!(via_helper, expected);
    assert_eq!(via_default, expected);
    match via_helper {
        FieldAvailability::Unavailable { reason } => {
            assert_eq!(reason, CHAINSTATE_DURABILITY_UNAVAILABLE_REASON);
        }
        FieldAvailability::Available(_) => panic!("default must stay unavailable"),
    }
}

#[test]
fn chainstate_durability_available_serializes_locked_snake_case_keys() {
    // Arrange
    let evidence = available_fixture();

    // Act
    let encoded = serde_json::to_value(&evidence).expect("available evidence json");
    let mut keys = collect_json_keys(&encoded);
    keys.sort();

    // Assert
    assert_eq!(encoded["cache_size"], "ok");
    assert_eq!(encoded["last_flush_reason"], "periodic");
    assert_eq!(encoded["write_kind"], "sync");
    assert_eq!(encoded["readiness"], "ready_to_flush");
    assert_eq!(encoded["recovery_outcome"], "replayed");
    assert_eq!(encoded["last_serving_status"], "unavailable");
    assert_eq!(encoded["available_count"], 0);
    assert_eq!(encoded["unavailable_count"], 1);
    assert_eq!(encoded["index_known_without_payload_count"], 1);
    assert_eq!(
        keys,
        vec![
            "available_count".to_string(),
            "cache_byte_limit".to_string(),
            "cache_bytes".to_string(),
            "cache_size".to_string(),
            "index_known_without_payload_count".to_string(),
            "last_flush_reason".to_string(),
            "last_index_known".to_string(),
            "last_payload_present".to_string(),
            "last_serving_status".to_string(),
            "last_validated_on_active_chain".to_string(),
            "maybe_coins_best_block_hash".to_string(),
            "maybe_coins_best_block_height".to_string(),
            "readiness".to_string(),
            "recovery_outcome".to_string(),
            "unavailable_count".to_string(),
            "write_kind".to_string(),
        ]
    );
}

#[test]
fn chainstate_durability_json_omits_peer_coin_and_pruned_keys() {
    // Arrange
    let encoded = serde_json::to_string(&available_fixture()).expect("available evidence text");
    let forbidden = [
        "pruned",
        "block_status_pruned",
        "peer_id",
        "getblock",
        "txid:vout",
        "cmpctblock",
    ];

    // Act
    let maybe_hits: Vec<&str> = forbidden
        .into_iter()
        .filter(|token| encoded.contains(token))
        .collect();

    // Assert
    assert!(
        maybe_hits.is_empty(),
        "serialized evidence leaked forbidden tokens: {maybe_hits:?}"
    );
}

#[test]
fn chainstate_durability_fail_closed_omits_coins_best_block() {
    // Arrange
    let fail_closed = fail_closed_without_tip();
    let interrupted = interrupted_without_tip();

    // Act
    let fail_closed_json = serde_json::to_value(&fail_closed).expect("fail_closed json");
    let interrupted_json = serde_json::to_value(&interrupted).expect("interrupted json");

    // Assert
    assert_eq!(fail_closed_json["recovery_outcome"], "fail_closed");
    assert_eq!(
        fail_closed_json["maybe_coins_best_block_height"],
        json!(null)
    );
    assert_eq!(fail_closed_json["maybe_coins_best_block_hash"], json!(null));
    assert_eq!(interrupted_json["recovery_outcome"], "interrupted");
    assert_eq!(
        interrupted_json["maybe_coins_best_block_height"],
        json!(null)
    );
    assert_eq!(interrupted_json["maybe_coins_best_block_hash"], json!(null));
}

#[test]
fn as_str_cache_size_and_last_flush_reason() {
    // Arrange
    let cache_sizes = [
        (CacheSizeLabel::Ok, "ok"),
        (CacheSizeLabel::Large, "large"),
        (CacheSizeLabel::Critical, "critical"),
    ];
    let flush_reasons = [
        (LastFlushReasonLabel::None, "none"),
        (LastFlushReasonLabel::Needed, "needed"),
        (LastFlushReasonLabel::Periodic, "periodic"),
        (LastFlushReasonLabel::Always, "always"),
        (LastFlushReasonLabel::FailedDisk, "failed_disk"),
    ];

    // Act / Assert
    for (label, expected) in cache_sizes {
        assert_eq!(label.as_str(), expected);
    }
    for (label, expected) in flush_reasons {
        assert_eq!(label.as_str(), expected);
    }
    assert_eq!(
        WriteKindLabel::RefuseDiskSpace.as_str(),
        "refuse_disk_space"
    );
    assert_eq!(ReadinessLabel::ReadyToFlush.as_str(), "ready_to_flush");
    assert_eq!(CoinsRecoveryOutcome::FailClosed.as_str(), "fail_closed");
    assert_eq!(ServingStatusLabel::Unavailable.as_str(), "unavailable");
}
