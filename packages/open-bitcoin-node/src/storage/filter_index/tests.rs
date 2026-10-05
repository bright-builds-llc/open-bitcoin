// Parity breadcrumbs:
// - packages/bitcoin-knots/src/index/blockfilterindex.cpp
// - packages/bitcoin-knots/src/blockfilter.cpp

use super::ownership::*;
use super::*;
use open_bitcoin_core::chainstate::filter_index::lifecycle::{
    EffectiveIndexOwnership, IndexGeneration, IndexLifecycle,
};

fn genesis() -> StoredFilterRecord {
    let bytes = vec![0];
    let hash = FilterHash::from_byte_array(double_sha256(&bytes));
    let header = open_bitcoin_core::consensus::compute_filter_header(hash, FilterHeader::default());
    StoredFilterRecord {
        identity: FilterRecordIdentity::new(
            0,
            BlockHash::from_byte_array([8; 32]),
            BlockHash::default(),
            hash,
            header,
            FilterHeader::default(),
            None,
        )
        .expect("genesis"),
        bytes,
    }
}

#[test]
fn record_roundtrip_has_frozen_exact_field_order() {
    // Arrange
    let record = genesis();
    let mut expected = vec![1, 0, 0, 0, 0, 0];
    expected.extend_from_slice(&[8; 32]);
    expected.extend_from_slice(&[0; 64]);
    expected.extend_from_slice(record.identity().filter_hash().as_bytes());
    expected.extend_from_slice(record.identity().filter_header().as_bytes());
    expected.extend_from_slice(&1_u32.to_le_bytes());
    expected.push(0);
    // Act
    let bytes = encode_record(&record);
    // Assert
    assert_eq!(bytes, expected);
    assert_eq!(bytes.len(), RECORD_OVERHEAD + 1);
    assert_eq!(
        decode_record(&record_key(record.identity().block_hash()), &bytes, None).expect("record"),
        record
    );
}

#[test]
fn record_refuses_truncation_trailing_unknown_type_and_each_commitment() {
    // Arrange
    let record = genesis();
    let bytes = encode_record(&record);
    let key = record_key(record.identity().block_hash());
    // Act / Assert
    for end in 0..bytes.len() {
        assert!(
            decode_record(&key, &bytes[..end], None).is_err(),
            "truncation {end}"
        );
    }
    let mut trailing = bytes.clone();
    trailing.push(0);
    assert!(decode_record(&key, &trailing, None).is_err());
    for offset in [0, 1, 2, 6, 38, 70, 102, 134, 166, 170] {
        let mut invalid = bytes.clone();
        invalid[offset] ^= 1;
        let error = decode_record(&key, &invalid, None).expect_err("refuse");
        assert_eq!(error.recovery_action(), Some(StorageRecoveryAction::Repair));
    }
    let mut oversized = bytes.clone();
    oversized[166..170].copy_from_slice(&(MAX_SIZE as u32 + 1).to_le_bytes());
    assert!(decode_record(&key, &oversized, None).is_err());
    assert!(decode_record("basic_filter:v1:record:invalid", &bytes, None).is_err());
}

#[test]
fn noncanonical_basic_bytes_refuse_before_accepting_a_consistent_hash() {
    // Arrange
    let mut record = genesis();
    record.bytes = vec![0, 0];
    let hash = FilterHash::from_byte_array(double_sha256(&record.bytes));
    record.identity = FilterRecordIdentity::new(
        0,
        record.identity.block_hash(),
        BlockHash::default(),
        hash,
        open_bitcoin_core::consensus::compute_filter_header(hash, FilterHeader::default()),
        FilterHeader::default(),
        None,
    )
    .expect("identity");
    // Act
    let result = decode_record(
        &record_key(record.identity.block_hash()),
        &encode_record(&record),
        None,
    );
    // Assert
    assert!(result.is_err());
}

#[test]
fn child_record_requires_the_exact_immutable_predecessor() {
    // Arrange
    let previous = genesis();
    let mut child = previous.clone();
    child.identity = FilterRecordIdentity::new(
        1,
        BlockHash::from_byte_array([9; 32]),
        previous.identity.block_hash(),
        previous.identity.filter_hash(),
        open_bitcoin_core::consensus::compute_filter_header(
            previous.identity.filter_hash(),
            previous.identity.filter_header(),
        ),
        previous.identity.filter_header(),
        Some(&previous.identity),
    )
    .expect("child");
    let key = record_key(child.identity.block_hash());
    let bytes = encode_record(&child);
    // Act / Assert
    assert_eq!(
        decode_record(&key, &bytes, Some(&previous.identity)).expect("child"),
        child
    );
    assert!(decode_record(&key, &bytes, None).is_err());
    assert!(decode_record(&key, &bytes, Some(&child.identity)).is_err());
}

#[test]
fn state_retains_genesis_endpoint_and_explicit_height_exhaustion() {
    // Arrange
    let state = StoredFilterState {
        maybe_endpoint: Some((0, genesis().identity.block_hash())),
        fence_height: 0,
        fence_hash: genesis().identity.block_hash(),
        protection: IndexInputProtection::HeightSpaceExhausted,
    };
    // Act
    let decoded = decode_state(&encode_state(state)).expect("state");
    // Assert
    assert_eq!(decoded, state);
    assert!(decoded.maybe_endpoint.is_some());
}

#[test]
fn empty_state_has_a_frozen_envelope_and_is_distinct_from_genesis() {
    // Arrange
    let state = StoredFilterState {
        maybe_endpoint: None,
        fence_height: 0,
        fence_hash: BlockHash::from_byte_array([7; 32]),
        protection: IndexInputProtection::FromHeight(0),
    };
    // Act
    let bytes = encode_state(state);
    // Assert
    let mut expected = vec![1, 0, 0];
    expected.extend_from_slice(&0_u32.to_le_bytes());
    expected.extend_from_slice(&[7; 32]);
    expected.push(0);
    expected.extend_from_slice(&0_u32.to_le_bytes());
    assert_eq!(bytes, expected);
    assert_eq!(decode_state(&bytes).expect("state"), state);
}

#[test]
fn malformed_state_refuses_every_truncation_and_trailing_data() {
    // Arrange
    let valid = encode_state(StoredFilterState {
        maybe_endpoint: None,
        fence_height: 0,
        fence_hash: BlockHash::default(),
        protection: IndexInputProtection::FromHeight(0),
    });
    // Act / Assert
    for end in 0..valid.len() {
        assert!(decode_state(&valid[..end]).is_err());
    }
    let mut trailing = valid.clone();
    trailing.push(0);
    assert!(decode_state(&trailing).is_err());
    for (offset, value) in [(0, 2), (1, 1), (2, 2), (39, 2)] {
        let mut invalid = valid.clone();
        invalid[offset] = value;
        assert!(decode_state(&invalid).is_err());
    }
}

#[test]
fn projection_binds_key_height_and_hash() {
    // Arrange
    let hash = BlockHash::from_byte_array([9; 32]);
    // Act
    let bytes = encode_projection(12, hash);
    // Assert
    assert_eq!(
        decode_projection(&active_key(12), &bytes).expect("projection"),
        hash
    );
    assert!(decode_projection(&active_key(13), &bytes).is_err());
    assert!(decode_projection("basic_filter:v1:active:0000000C", &bytes).is_err());
    assert!(decode_projection(&active_key(12), &bytes[..37]).is_err());
}

#[test]
fn owner_has_frozen_exact_bytes_for_each_mode_and_generation_boundary() {
    // Arrange
    for value in [0, 42, u64::MAX] {
        for (mode, owner) in [
            (
                0,
                IndexLifecycle::Active {
                    generation: IndexGeneration::new(value),
                },
            ),
            (
                1,
                IndexLifecycle::Disabled {
                    generation: IndexGeneration::new(value),
                },
            ),
        ] {
            let mut expected = vec![1, 0, mode];
            expected.extend_from_slice(&value.to_le_bytes());
            // Act
            let bytes = encode_owner(owner);
            // Assert
            assert_eq!(bytes.as_slice(), expected);
            assert_eq!(bytes.len(), 11);
            assert_eq!(decode_owner(&bytes).expect("owner"), owner);
        }
    }
}

#[test]
fn owner_refuses_every_truncation_and_trailing_bytes_as_corruption() {
    // Arrange
    let bytes = encode_owner(IndexLifecycle::Active {
        generation: IndexGeneration::new(0),
    });
    let mut malformed: Vec<Vec<u8>> = (0..bytes.len()).map(|end| bytes[..end].to_vec()).collect();
    let mut trailing = bytes.to_vec();
    trailing.push(0);
    malformed.push(trailing);
    // Act / Assert
    for invalid in malformed {
        let error = decode_owner(&invalid).expect_err("corruption is not absence");
        assert_eq!(error.recovery_action(), Some(StorageRecoveryAction::Repair));
        assert!(matches!(
            error,
            StorageError::Corruption {
                namespace: StorageNamespace::BlockIndex,
                ..
            }
        ));
    }
}

#[test]
fn owner_refuses_unknown_version_type_and_mode() {
    // Arrange
    let bytes = encode_owner(IndexLifecycle::Active {
        generation: IndexGeneration::new(0),
    });
    // Act / Assert
    for (offset, value) in [(0, 2), (1, 1), (2, 2)] {
        let mut invalid = bytes;
        invalid[offset] = value;
        let error = decode_owner(&invalid).expect_err("unsupported owner");
        assert_eq!(error.recovery_action(), Some(StorageRecoveryAction::Repair));
    }
}

#[test]
fn owner_decoding_cannot_make_missing_state_valid() {
    // Arrange
    let bytes = encode_owner(IndexLifecycle::Disabled {
        generation: IndexGeneration::new(1),
    });
    // Act
    let owner = decode_owner(&bytes).expect("owner envelope");
    let result =
        EffectiveIndexOwnership::maybe_from_artifacts(Some(owner), None, None, None, false, false);
    // Assert
    assert!(result.is_err());
}

#[test]
fn projection_has_frozen_v1_bytes_after_additive_owner_codec() {
    // Arrange
    let mut expected = vec![1, 0, 12, 0, 0, 0];
    expected.extend_from_slice(&[9; 32]);
    // Act
    let bytes = encode_projection(12, BlockHash::from_byte_array([9; 32]));
    // Assert
    assert_eq!(bytes, expected);
}
