// Parity breadcrumbs:
// - packages/bitcoin-knots/src/node/mempool_persist.cpp
// - packages/bitcoin-knots/src/node/mempool_persist.h
// - packages/bitcoin-knots/test/functional/mempool_persist.py

use super::*;

#[test]
fn mempool_snapshot_v2_decode_rejects_terminal_captured_generation() {
    // Arrange
    let snapshot = mempool_snapshot();
    let encoded = encode_mempool_snapshot(&snapshot).expect("encode valid current-v2");
    let mut value: serde_json::Value = serde_json::from_slice(&encoded).expect("snapshot JSON");
    value["payload"]["captured_generation"] = serde_json::json!(18_446_744_073_709_551_615_u64);
    let crafted = serde_json::to_vec(&value).expect("re-serialize crafted current-v2");

    // Act
    let error = decode_mempool_snapshot(&crafted)
        .expect_err("terminal captured generation is structural corruption");

    // Assert
    assert!(matches!(
        error,
        StorageError::Corruption {
            namespace: StorageNamespace::Mempool,
            ref detail,
            ..
        } if detail == "mempool snapshot structure is corrupt"
    ));
}
