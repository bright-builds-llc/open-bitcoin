// Parity breadcrumbs:
// - packages/bitcoin-knots/src/index/base.cpp
// - packages/bitcoin-knots/src/index/blockfilterindex.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp

use super::*;

#[test]
fn standalone_commitment_proof_refuses_genesis_and_header_corruption_without_minting_identity() {
    // Arrange
    let genesis = identity(&positions()[0], None);
    // Act / Assert
    assert_eq!(
        verify_filter_record_commitment(
            0,
            genesis.parent_hash(),
            genesis.filter_hash(),
            genesis.filter_header(),
            genesis.previous_header()
        ),
        Ok(())
    );
    assert_eq!(
        verify_filter_record_commitment(
            0,
            BlockHash::from_byte_array([1; 32]),
            genesis.filter_hash(),
            genesis.filter_header(),
            genesis.previous_header()
        ),
        Err(FilterIndexError::Genesis)
    );
    assert_eq!(
        verify_filter_record_commitment(
            0,
            genesis.parent_hash(),
            genesis.filter_hash(),
            genesis.filter_header(),
            FilterHeader::from_byte_array([1; 32])
        ),
        Err(FilterIndexError::Genesis)
    );
    assert_eq!(
        verify_filter_record_commitment(
            1,
            genesis.block_hash(),
            genesis.filter_hash(),
            FilterHeader::default(),
            genesis.filter_header()
        ),
        Err(FilterIndexError::HeaderCommitment)
    );
}

#[test]
fn direct_predecessor_proof_requires_strict_decreasing_height_hash_and_header() {
    // Arrange
    let genesis = identity(&positions()[0], None);
    let parent = (
        genesis.height(),
        genesis.block_hash(),
        genesis.filter_header(),
    );
    // Act / Assert
    assert_eq!(
        verify_filter_record_predecessor(0, BlockHash::default(), FilterHeader::default(), None),
        Ok(())
    );
    assert_eq!(
        verify_filter_record_predecessor(
            0,
            BlockHash::default(),
            FilterHeader::default(),
            Some(parent)
        ),
        Err(FilterIndexError::Genesis)
    );
    assert_eq!(
        verify_filter_record_predecessor(
            1,
            genesis.block_hash(),
            genesis.filter_header(),
            Some(parent)
        ),
        Ok(())
    );
    for maybe_parent in [
        None,
        Some((1, parent.1, parent.2)),
        Some((0, BlockHash::default(), parent.2)),
        Some((0, parent.1, FilterHeader::default())),
    ] {
        assert_eq!(
            verify_filter_record_predecessor(
                1,
                genesis.block_hash(),
                genesis.filter_header(),
                maybe_parent
            ),
            Err(FilterIndexError::Predecessor)
        );
    }
}
