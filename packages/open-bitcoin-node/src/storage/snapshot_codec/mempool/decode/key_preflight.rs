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
    let mut containers = Vec::new();
    let mut cursor = 0_usize;
    let mut root_complete = false;

    while cursor < encoded.len() {
        if encoded[cursor].is_ascii_whitespace() {
            cursor += 1;
            continue;
        }

        let Some(container) = containers.last().copied() else {
            if root_complete {
                return Err(structural_failure());
            }
            root_complete = true;
            begin_value(encoded, &mut cursor, &mut containers)?;
            continue;
        };

        match container {
            ContainerState::Object(ObjectState::KeyOrEnd) => {
                if encoded[cursor] == b'}' {
                    containers.pop();
                    cursor += 1;
                    continue;
                }
                scan_object_key(encoded, &mut cursor, &mut containers)?;
            }
            ContainerState::Object(ObjectState::Key) => {
                scan_object_key(encoded, &mut cursor, &mut containers)?;
            }
            ContainerState::Object(ObjectState::Colon) => {
                if encoded[cursor] != b':' {
                    return Err(structural_failure());
                }
                containers.pop();
                push_container(&mut containers, ContainerState::Object(ObjectState::Value))?;
                cursor += 1;
            }
            ContainerState::Object(ObjectState::Value) => {
                containers.pop();
                push_container(
                    &mut containers,
                    ContainerState::Object(ObjectState::CommaOrEnd),
                )?;
                begin_value(encoded, &mut cursor, &mut containers)?;
            }
            ContainerState::Object(ObjectState::CommaOrEnd) => {
                scan_object_delimiter(encoded, &mut cursor, &mut containers)?;
            }
            ContainerState::Array(ArrayState::ValueOrEnd) => {
                if encoded[cursor] == b']' {
                    containers.pop();
                    cursor += 1;
                    continue;
                }
                containers.pop();
                push_container(
                    &mut containers,
                    ContainerState::Array(ArrayState::CommaOrEnd),
                )?;
                begin_value(encoded, &mut cursor, &mut containers)?;
            }
            ContainerState::Array(ArrayState::Value) => {
                containers.pop();
                push_container(
                    &mut containers,
                    ContainerState::Array(ArrayState::CommaOrEnd),
                )?;
                begin_value(encoded, &mut cursor, &mut containers)?;
            }
            ContainerState::Array(ArrayState::CommaOrEnd) => {
                scan_array_delimiter(encoded, &mut cursor, &mut containers)?;
            }
        }
    }

    if !root_complete || !containers.is_empty() {
        return Err(structural_failure());
    }
    Ok(())
}

#[derive(Clone, Copy)]
enum ContainerState {
    Object(ObjectState),
    Array(ArrayState),
}

#[derive(Clone, Copy)]
enum ObjectState {
    KeyOrEnd,
    Key,
    Colon,
    Value,
    CommaOrEnd,
}

#[derive(Clone, Copy)]
enum ArrayState {
    ValueOrEnd,
    Value,
    CommaOrEnd,
}

fn scan_object_key(
    encoded: &[u8],
    cursor: &mut usize,
    containers: &mut Vec<ContainerState>,
) -> Result<(), StorageError> {
    if encoded[*cursor] != b'"' {
        return Err(structural_failure());
    }
    *cursor = scan_json_string(encoded, *cursor, Some(MAX_MEMPOOL_FIELD_TOKEN_BYTES))?;
    containers.pop();
    push_container(containers, ContainerState::Object(ObjectState::Colon))
}

fn scan_object_delimiter(
    encoded: &[u8],
    cursor: &mut usize,
    containers: &mut Vec<ContainerState>,
) -> Result<(), StorageError> {
    match encoded[*cursor] {
        b',' => {
            containers.pop();
            push_container(containers, ContainerState::Object(ObjectState::Key))?;
            *cursor += 1;
        }
        b'}' => {
            containers.pop();
            *cursor += 1;
        }
        _ => return Err(structural_failure()),
    }
    Ok(())
}

fn scan_array_delimiter(
    encoded: &[u8],
    cursor: &mut usize,
    containers: &mut Vec<ContainerState>,
) -> Result<(), StorageError> {
    match encoded[*cursor] {
        b',' => {
            containers.pop();
            push_container(containers, ContainerState::Array(ArrayState::Value))?;
            *cursor += 1;
        }
        b']' => {
            containers.pop();
            *cursor += 1;
        }
        _ => return Err(structural_failure()),
    }
    Ok(())
}

fn begin_value(
    encoded: &[u8],
    cursor: &mut usize,
    containers: &mut Vec<ContainerState>,
) -> Result<(), StorageError> {
    match encoded[*cursor] {
        b'"' => *cursor = scan_json_string(encoded, *cursor, None)?,
        b'{' => {
            push_container(containers, ContainerState::Object(ObjectState::KeyOrEnd))?;
            *cursor += 1;
        }
        b'[' => {
            push_container(containers, ContainerState::Array(ArrayState::ValueOrEnd))?;
            *cursor += 1;
        }
        b'}' | b']' | b',' | b':' => return Err(structural_failure()),
        _ => scan_scalar(encoded, cursor)?,
    }
    Ok(())
}

fn scan_scalar(encoded: &[u8], cursor: &mut usize) -> Result<(), StorageError> {
    let start = *cursor;
    while *cursor < encoded.len()
        && !encoded[*cursor].is_ascii_whitespace()
        && !matches!(encoded[*cursor], b',' | b'}' | b']')
    {
        if matches!(encoded[*cursor], b'"' | b'{' | b'[' | b':') {
            return Err(structural_failure());
        }
        *cursor += 1;
    }
    if *cursor == start {
        return Err(structural_failure());
    }
    Ok(())
}

fn scan_json_string(
    encoded: &[u8],
    opening_quote: usize,
    maybe_max_raw_bytes: Option<usize>,
) -> Result<usize, StorageError> {
    let mut cursor = opening_quote
        .checked_add(1)
        .ok_or_else(structural_failure)?;
    let mut raw_bytes = 0_usize;

    while cursor < encoded.len() {
        let byte = encoded[cursor];
        if byte == b'"' {
            return cursor.checked_add(1).ok_or_else(structural_failure);
        }
        consume_raw_bytes(&mut raw_bytes, 1, maybe_max_raw_bytes)?;
        if byte == b'\\' {
            cursor = scan_json_escape(encoded, cursor, &mut raw_bytes, maybe_max_raw_bytes)?;
            continue;
        }
        if byte < 0x20 {
            return Err(structural_failure());
        }
        cursor = cursor.checked_add(1).ok_or_else(structural_failure)?;
    }

    Err(structural_failure())
}

fn scan_json_escape(
    encoded: &[u8],
    backslash: usize,
    raw_bytes: &mut usize,
    maybe_max_raw_bytes: Option<usize>,
) -> Result<usize, StorageError> {
    let escaped = backslash.checked_add(1).ok_or_else(structural_failure)?;
    let Some(escape) = encoded.get(escaped).copied() else {
        return Err(structural_failure());
    };
    consume_raw_bytes(raw_bytes, 1, maybe_max_raw_bytes)?;

    if matches!(
        escape,
        b'"' | b'\\' | b'/' | b'b' | b'f' | b'n' | b'r' | b't'
    ) {
        return escaped.checked_add(1).ok_or_else(structural_failure);
    }
    if escape != b'u' {
        return Err(structural_failure());
    }

    let hex_start = escaped.checked_add(1).ok_or_else(structural_failure)?;
    let hex_end = hex_start.checked_add(4).ok_or_else(structural_failure)?;
    let Some(hex_digits) = encoded.get(hex_start..hex_end) else {
        return Err(structural_failure());
    };
    consume_raw_bytes(raw_bytes, 4, maybe_max_raw_bytes)?;
    if !hex_digits.iter().all(u8::is_ascii_hexdigit) {
        return Err(structural_failure());
    }
    Ok(hex_end)
}

fn consume_raw_bytes(
    raw_bytes: &mut usize,
    consumed: usize,
    maybe_max_raw_bytes: Option<usize>,
) -> Result<(), StorageError> {
    *raw_bytes = raw_bytes
        .checked_add(consumed)
        .ok_or_else(resource_failure)?;
    if maybe_max_raw_bytes.is_some_and(|maximum| *raw_bytes > maximum) {
        return Err(resource_failure());
    }
    Ok(())
}

fn push_container(
    containers: &mut Vec<ContainerState>,
    state: ContainerState,
) -> Result<(), StorageError> {
    containers.try_reserve(1).map_err(|_| resource_failure())?;
    containers.push(state);
    Ok(())
}

fn resource_failure() -> StorageError {
    snapshot_failure(MempoolSnapshotError::ResourceBoundExceeded)
}

fn structural_failure() -> StorageError {
    snapshot_failure(MempoolSnapshotError::StructuralCorruption)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::StorageNamespace;

    fn assert_structural_corruption(encoded: &[u8]) {
        // Act
        let error = validate_raw_object_keys(encoded).expect_err("malformed JSON must fail");

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

    #[test]
    fn raw_key_preflight_rejects_trailing_simple_escape() {
        // Arrange
        let encoded = br#"{"key\"#;

        assert_structural_corruption(encoded);
    }

    #[test]
    fn raw_key_preflight_rejects_incomplete_unicode_escapes() {
        // Arrange / Act / Assert
        for encoded in [
            br#"{"key\u"#.as_slice(),
            br#"{"key\u0"#,
            br#"{"key\u00"#,
            br#"{"key\u000"#,
        ] {
            assert_structural_corruption(encoded);
        }
    }

    #[test]
    fn raw_key_preflight_rejects_unterminated_key() {
        // Arrange
        let encoded = br#"{"key"#;

        assert_structural_corruption(encoded);
    }

    #[test]
    fn raw_key_preflight_rejects_unterminated_value_string() {
        // Arrange
        let encoded = br#"{"key":"value"#;

        assert_structural_corruption(encoded);
    }
}
