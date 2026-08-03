// Parity breadcrumbs:
// - packages/bitcoin-knots/src/node/mempool_persist.cpp
// - packages/bitcoin-knots/src/node/mempool_persist.h
// - packages/bitcoin-knots/test/functional/mempool_persist.py

//! Raw JSON object-key bounds enforced before Serde can unescape into scratch storage.

use super::super::snapshot_failure;
use crate::StorageError;
use crate::storage::mempool_snapshot::MempoolSnapshotError;

const MAX_MEMPOOL_FIELD_TOKEN_BYTES: usize = 64;

pub(super) fn validate_raw_object_keys(encoded: &[u8]) -> Result<(), StorageError> {
    let mut cursor = 0_usize;
    while cursor < encoded.len() {
        if encoded[cursor] != b'"' {
            cursor += 1;
            continue;
        }

        let token_start = cursor + 1;
        cursor = token_start;
        while cursor < encoded.len() {
            match encoded[cursor] {
                b'\\' => cursor = cursor.saturating_add(2),
                b'"' => break,
                _ => cursor += 1,
            }
        }
        if cursor == encoded.len() {
            return Ok(());
        }

        let token_bytes = cursor - token_start;
        let mut after_token = cursor + 1;
        while after_token < encoded.len() && encoded[after_token].is_ascii_whitespace() {
            after_token += 1;
        }
        if encoded.get(after_token) == Some(&b':') && token_bytes > MAX_MEMPOOL_FIELD_TOKEN_BYTES {
            return Err(snapshot_failure(
                MempoolSnapshotError::ResourceBoundExceeded,
            ));
        }

        cursor += 1;
    }
    Ok(())
}
