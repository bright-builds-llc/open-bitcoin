// Parity breadcrumbs:
// - packages/bitcoin-knots/src/undo.h
// - packages/bitcoin-knots/src/validation.cpp

use super::*;

#[test]
fn phase167_undo_codec_missing_legacy_field_is_not_no_overwrite_proof() {
    // Arrange: this entry point receives no history or pre-block scan.
    let bytes = br#"{"schema_version":2,"payload":{"transactions":[]}}"#;
    // Act / Assert
    assert!(matches!(
        decode_block_undo(bytes),
        Err(StorageError::Corruption { .. })
    ));
}

#[test]
fn phase167_undo_codec_explicit_empty_collection_remains_unproved() {
    // Arrange
    let undo = BlockUndo::default();
    // Act
    let decoded = decode_block_undo(&encode_block_undo(&undo).expect("encode"))
        .expect("explicit new-format unknown data");
    // Assert
    assert_eq!(decoded, undo);
    assert!(decoded.overwrites.is_empty());
}

#[test]
fn phase167_undo_codec_snapshot_missing_legacy_field_refuses() {
    // Arrange
    let bytes = encode_chainstate_snapshot(&chainstate_snapshot()).expect("snapshot");
    let mut json: serde_json::Value = serde_json::from_slice(&bytes).expect("JSON");
    json["payload"]["undo_by_block"][0]["undo"]
        .as_object_mut()
        .expect("undo")
        .remove("overwrites");
    // Act / Assert
    assert!(decode_chainstate_snapshot(&serde_json::to_vec(&json).expect("encode")).is_err());
}

#[test]
fn phase167_undo_codec_oversized_carriers_refuse_before_json_allocation() {
    // Arrange: one lazy zero-filled input shared by both decoding entry points.
    let oversized = vec![0; 256 * 1024 * 1024 + 1];
    // Act / Assert
    for result in [
        decode_block_undo(&oversized).map(|_| ()),
        decode_chainstate_snapshot(&oversized).map(|_| ()),
    ] {
        let error = result.expect_err("bounded carrier");
        assert!(error.to_string().contains("byte bound exceeded"));
    }
}
