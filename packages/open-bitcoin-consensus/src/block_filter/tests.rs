// Parity breadcrumbs:
// - packages/bitcoin-knots/src/blockfilter.cpp
// - packages/bitcoin-knots/src/blockfilter.h
// - packages/bitcoin-knots/src/crypto/siphash.cpp

use super::*;

fn decode_hex(hex: &str) -> Vec<u8> {
    hex.as_bytes()
        .chunks_exact(2)
        .map(|pair| {
            let text = core::str::from_utf8(pair).expect("ASCII hex");
            u8::from_str_radix(text, 16).expect("hex byte")
        })
        .collect()
}

fn empty_filter(block: u8) -> BasicFilter {
    BasicFilter::from_script_facts(BlockHash::from_byte_array([block; 32]), &[], &[])
        .expect("empty filter")
}

#[test]
fn mapped_value_allocation_refuses_unrepresentable_count() {
    assert_eq!(
        reserve_mapped_values(usize::MAX),
        Err(BasicFilterEncodingError::AllocationFailed)
    );
}

#[test]
fn genesis_filter_matches_pinned_bytes_and_header() {
    // Arrange: first vector in Knots src/test/data/blockfilters.json (testnet genesis).
    let mut raw_hash =
        decode_hex("000000000933ea01ad0ee984209779baaec3ced90fa3f408719526f8d77f4943");
    raw_hash.reverse();
    let hash = BlockHash::from_slice(&raw_hash).expect("block hash");
    let script = decode_hex(
        "4104678afdb0fe5548271967f1a67130b7105cd6a828e03909a67962e0ea1f61deb649f6bc3f4cef38c4f35504e51ec112de5c384df7ba0b8d578a4c702b6bf11d5fac",
    );
    // Act
    let filter = BasicFilter::from_script_facts(hash, &[&script], &[]).expect("genesis filter");
    let header = filter
        .header(BlockHash::default(), 0, FilterHeaderPredecessor::Genesis)
        .expect("genesis header");
    // Assert
    assert_eq!(filter.block_hash(), hash);
    assert_eq!(filter.encoded_bytes(), decode_hex("019dfca8"));
    assert_eq!(
        filter_header_display_hex(header),
        "21584579b7eb08997773e5aeff3a7f932700042d0ed2a6129012b7d7ae81b750"
    );
}

#[test]
fn empty_filter_hashes_the_canonical_zero_byte() {
    // Arrange
    let filter = empty_filter(1);
    // Act
    let actual = filter_hash_display_hex(filter.filter_hash());
    // Assert: independent Python hashlib SHA256d(b'\0'), reversed for display.
    assert_eq!(filter.encoded_bytes(), &[0]);
    assert_eq!(
        actual,
        "9a538906e6466ebd2617d321f71bc94e56056ce213d366773699e28158e00614"
    );
}

#[test]
fn selection_retains_raw_spent_op_return_and_malformed_scripts() {
    // Arrange
    let hash = BlockHash::from_byte_array([1; 32]);
    let outputs: &[&[u8]] = &[&[], &[0x6a, 1], &[0x4c], &[0x51, 0x6a]];
    let spent: &[&[u8]] = &[&[], &[0x6a, 1], &[0x4c]];
    // Act
    let actual = BasicFilter::from_script_facts(hash, outputs, spent).expect("selected filter");
    let expected = BasicFilter::from_script_facts(hash, &[&[0x4c], &[0x51, 0x6a]], &[&[0x6a, 1]])
        .expect("explicit selected scripts");
    // Assert
    assert_eq!(actual, expected);
    assert_eq!(actual.encoded_bytes()[0], 3);
}

#[test]
fn raw_duplicates_do_not_increase_element_count() {
    // Arrange
    let hash = BlockHash::from_byte_array([2; 32]);
    // Act
    let actual =
        BasicFilter::from_script_facts(hash, &[&[0x51], &[0x51]], &[&[0x51]]).expect("duplicates");
    let expected = BasicFilter::from_script_facts(hash, &[&[0x51]], &[]).expect("one script");
    // Assert
    assert_eq!(actual, expected);
    assert_eq!(actual.encoded_bytes()[0], 1);
}

#[test]
fn genesis_refuses_nonzero_height() {
    assert_eq!(
        empty_filter(1).header(BlockHash::default(), 1, FilterHeaderPredecessor::Genesis),
        Err(FilterHeaderError::GenesisHeight)
    );
}

#[test]
fn genesis_refuses_nonzero_parent() {
    assert_eq!(
        empty_filter(1).header(
            BlockHash::from_byte_array([1; 32]),
            0,
            FilterHeaderPredecessor::Genesis
        ),
        Err(FilterHeaderError::GenesisParent)
    );
}

#[test]
fn previous_refuses_genesis_height() {
    let previous = FilterHeaderPredecessor::Previous {
        block_hash: BlockHash::default(),
        height: 0,
        header: FilterHeader::default(),
    };
    assert_eq!(
        empty_filter(1).header(BlockHash::default(), 0, previous),
        Err(FilterHeaderError::PreviousForGenesis)
    );
}

#[test]
fn previous_refuses_wrong_parent() {
    let previous = FilterHeaderPredecessor::Previous {
        block_hash: BlockHash::default(),
        height: 0,
        header: FilterHeader::default(),
    };
    assert_eq!(
        empty_filter(1).header(BlockHash::from_byte_array([2; 32]), 1, previous),
        Err(FilterHeaderError::PreviousHash)
    );
}

#[test]
fn previous_refuses_wrong_height_without_overflow() {
    // Arrange
    let filter = empty_filter(1);
    for height in [1, u32::MAX] {
        let previous = FilterHeaderPredecessor::Previous {
            block_hash: BlockHash::default(),
            height,
            header: FilterHeader::default(),
        };
        // Act / Assert
        assert_eq!(
            filter.header(BlockHash::default(), 1, previous),
            Err(FilterHeaderError::PreviousHeight)
        );
    }
}

#[test]
fn replacement_descendants_use_their_own_ordered_commitments() {
    // Arrange
    let ancestor = empty_filter(1);
    let ancestor_header = ancestor
        .header(BlockHash::default(), 0, FilterHeaderPredecessor::Genesis)
        .expect("genesis");
    let original =
        BasicFilter::from_script_facts(BlockHash::from_byte_array([2; 32]), &[&[0x51]], &[])
            .expect("original");
    let replacement =
        BasicFilter::from_script_facts(BlockHash::from_byte_array([3; 32]), &[&[0x52]], &[])
            .expect("replacement");
    let previous = FilterHeaderPredecessor::Previous {
        block_hash: ancestor.block_hash(),
        height: 0,
        header: ancestor_header,
    };
    // Act
    let original_header = original
        .header(ancestor.block_hash(), 1, previous)
        .expect("original header");
    let replacement_header = replacement
        .header(ancestor.block_hash(), 1, previous)
        .expect("replacement header");
    let descendant = empty_filter(4);
    let descendant_header = descendant
        .header(
            replacement.block_hash(),
            2,
            FilterHeaderPredecessor::Previous {
                block_hash: replacement.block_hash(),
                height: 1,
                header: replacement_header,
            },
        )
        .expect("descendant header");
    // Assert
    assert_ne!(original_header, replacement_header);
    assert_eq!(
        descendant_header,
        compute_filter_header(descendant.filter_hash(), replacement_header)
    );
    assert_ne!(
        descendant_header,
        compute_filter_header(descendant.filter_hash(), original_header)
    );
}

#[test]
fn display_hex_reverses_only_at_the_formatting_boundary() {
    // Arrange
    let raw = core::array::from_fn(|index| index as u8);
    // Act
    let hash = FilterHash::from_byte_array(raw);
    let header = FilterHeader::from_byte_array(raw);
    // Assert
    let expected = "1f1e1d1c1b1a191817161514131211100f0e0d0c0b0a09080706050403020100";
    assert_eq!(filter_hash_display_hex(hash), expected);
    assert_eq!(filter_header_display_hex(header), expected);
    assert_eq!(hash.to_byte_array(), raw);
    assert_eq!(header.to_byte_array(), raw);
}

#[test]
fn header_error_messages_explain_identity_refusals() {
    let cases = [
        (
            FilterHeaderError::GenesisHeight,
            "genesis predecessor requires height zero",
        ),
        (
            FilterHeaderError::GenesisParent,
            "genesis predecessor requires a null parent",
        ),
        (
            FilterHeaderError::PreviousForGenesis,
            "height zero requires a genesis predecessor",
        ),
        (
            FilterHeaderError::PreviousHash,
            "predecessor block hash does not match parent",
        ),
        (
            FilterHeaderError::PreviousHeight,
            "predecessor height is not the preceding height",
        ),
    ];
    for (error, expected) in cases {
        assert_eq!(error.to_string(), expected);
    }
}
