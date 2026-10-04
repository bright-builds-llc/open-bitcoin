// Parity breadcrumbs:
// - packages/bitcoin-knots/src/blockfilter.cpp
// - packages/bitcoin-knots/src/test/blockfilter_tests.cpp
// - packages/bitcoin-knots/src/test/data/blockfilters.json
// - packages/bitcoin-knots/src/crypto/siphash.cpp

use open_bitcoin_codec::{encode_block, parse_block};
use open_bitcoin_consensus::block_filter::{
    BasicFilter, FilterHeaderPredecessor, compute_filter_header, filter_hash_display_hex,
    filter_header_display_hex,
};
use open_bitcoin_consensus::block_hash;
use open_bitcoin_consensus::crypto::siphash_bytes;
use open_bitcoin_primitives::{BlockHash, FilterHash, FilterHeader};

mod vectors {
    include!("../testdata/basic_filter_vectors.rs");
}
use vectors::{BASIC_VECTORS, BasicVector, SIPHASH_LENGTH_VECTORS};

fn hex(value: &str) -> Vec<u8> {
    assert_eq!(value.len() % 2, 0);
    value
        .as_bytes()
        .chunks_exact(2)
        .map(|pair| {
            u8::from_str_radix(std::str::from_utf8(pair).expect("fixture ASCII"), 16)
                .expect("fixture hex")
        })
        .collect()
}
fn raw32(value: &str) -> [u8; 32] {
    hex(value).try_into().expect("32 fixture bytes")
}
fn vector(name: &str) -> &'static BasicVector {
    BASIC_VECTORS
        .iter()
        .find(|vector| vector.name == name)
        .expect("named fixture")
}
fn generate(vector: &BasicVector) -> BasicFilter {
    let outputs: Vec<_> = vector.outputs.iter().map(|script| hex(script)).collect();
    let spent: Vec<_> = vector.spent.iter().map(|script| hex(script)).collect();
    BasicFilter::from_script_facts(
        BlockHash::from_byte_array(raw32(vector.block_hash_raw)),
        &outputs.iter().map(Vec::as_slice).collect::<Vec<_>>(),
        &spent.iter().map(Vec::as_slice).collect::<Vec<_>>(),
    )
    .expect("bounded facts")
}

#[test]
fn every_pinned_raw_block_matches_independent_bytes_hash_and_header() {
    // Arrange
    let corpus: Vec<_> = BASIC_VECTORS
        .iter()
        .filter(|row| row.name.starts_with("corpus-"))
        .collect();
    assert_eq!(corpus.len(), 10);
    for row in corpus {
        let block = parse_block(&hex(row.raw_block)).expect("pinned raw block");
        let spent: Vec<_> = row.spent.iter().map(|script| hex(script)).collect();
        let outputs: Vec<_> = block
            .transactions
            .iter()
            .flat_map(|tx| tx.outputs.iter())
            .map(|output| output.script_pubkey.as_bytes())
            .collect();
        // Act
        let filter = BasicFilter::from_script_facts(
            block_hash(&block.header),
            &outputs,
            &spent.iter().map(Vec::as_slice).collect::<Vec<_>>(),
        )
        .expect("sparse script facts");
        let header = compute_filter_header(
            filter.filter_hash(),
            FilterHeader::from_byte_array(raw32(row.predecessor_raw)),
        );
        // Assert
        assert_eq!(
            encode_block(&block).expect("raw block round trip"),
            hex(row.raw_block)
        );
        assert_eq!(
            block_hash(&block.header).as_bytes(),
            &raw32(row.block_hash_raw),
            "{}",
            row.name
        );
        assert_eq!(filter.encoded_bytes(), hex(row.encoded), "{}", row.name);
        assert_eq!(
            filter.filter_hash().as_bytes(),
            &raw32(row.hash_raw),
            "{}",
            row.name
        );
        assert_eq!(header.as_bytes(), &raw32(row.header_raw), "{}", row.name);
        assert_eq!(
            filter_hash_display_hex(filter.filter_hash()),
            row.hash_display
        );
        assert_eq!(filter_header_display_hex(header), row.header_display);
        let display: String = block_hash(&block.header)
            .as_bytes()
            .iter()
            .rev()
            .map(|byte| format!("{byte:02x}"))
            .collect();
        assert_eq!(display, row.block_hash_display);
    }
}

#[test]
fn actual_block_recipes_match_independent_raw_serialization_and_identity() {
    // Arrange
    for row in BASIC_VECTORS
        .iter()
        .filter(|row| row.name.starts_with("actual-"))
    {
        // Act
        let block = parse_block(&hex(row.raw_block)).expect("independent actual raw block");
        let encoded = encode_block(&block).expect("actual block round trip");
        // Assert
        assert_eq!(encoded, hex(row.raw_block), "{}", row.name);
        assert_eq!(
            block_hash(&block.header).as_bytes(),
            &raw32(row.block_hash_raw),
            "{}",
            row.name
        );
        let outputs: Vec<_> = block
            .transactions
            .iter()
            .flat_map(|tx| tx.outputs.iter())
            .map(|output| {
                output
                    .script_pubkey
                    .as_bytes()
                    .iter()
                    .map(|byte| format!("{byte:02x}"))
                    .collect::<String>()
            })
            .collect();
        assert_eq!(outputs, row.outputs);
    }
}

#[test]
fn edge_selection_and_encoding_match_independent_expectations() {
    // Arrange
    for row in BASIC_VECTORS
        .iter()
        .filter(|row| !row.name.starts_with("corpus-"))
    {
        // Act
        let filter = generate(row);
        let header = compute_filter_header(
            filter.filter_hash(),
            FilterHeader::from_byte_array(raw32(row.predecessor_raw)),
        );
        // Assert
        assert_eq!(filter.encoded_bytes(), hex(row.encoded), "{}", row.name);
        assert_eq!(
            filter.filter_hash().as_bytes(),
            &raw32(row.hash_raw),
            "{}",
            row.name
        );
        assert_eq!(header.as_bytes(), &raw32(row.header_raw), "{}", row.name);
    }
}

#[test]
fn raw_duplicates_deduplicate_but_distinct_mapped_collisions_retain_zero_delta() {
    // Arrange
    let duplicate = vector("duplicate");
    let collision = vector("collision");
    // Act
    let duplicate_filter = generate(duplicate);
    let collision_filter = generate(collision);
    let mapped: Vec<_> = collision
        .outputs
        .iter()
        .map(|script| {
            ((u128::from(siphash_bytes(collision.k0, collision.k1, &hex(script)))
                * u128::from(collision.count * 784_931))
                >> 64) as u64
        })
        .collect();
    // Assert
    assert_eq!(duplicate_filter, generate(vector("single")));
    assert_ne!(collision.outputs[0], collision.outputs[1]);
    assert_eq!(mapped, collision.mapped);
    assert_eq!(mapped, [1_560_268, 1_560_268]);
    assert_eq!(collision_filter.encoded_bytes()[0], 2);
    assert_eq!(collision_filter.encoded_bytes(), hex(collision.encoded));
}

#[test]
fn output_op_return_exclusion_does_not_exclude_spent_or_noninitial_op_return() {
    // Arrange / Act
    let output = generate(vector("output-op-return"));
    let spent = generate(vector("spent-op-return"));
    let noninitial = generate(vector("noninitial-op-return"));
    // Assert
    assert_eq!(output.encoded_bytes(), [0]);
    assert_eq!(
        spent.encoded_bytes(),
        hex(vector("spent-op-return").encoded)
    );
    assert_eq!(spent.encoded_bytes()[0], 1);
    assert_eq!(noninitial.encoded_bytes()[0], 1);
}

#[test]
fn compact_size_count_boundary_is_independently_frozen() {
    // Arrange / Act
    let before = generate(vector("count-252"));
    let after = generate(vector("count-253"));
    // Assert
    assert_eq!(before.encoded_bytes()[0], 252);
    assert_eq!(&after.encoded_bytes()[..3], &[253, 253, 0]);
    assert_eq!(after.encoded_bytes(), hex(vector("count-253").encoded));
}

#[test]
fn ordered_and_replacement_commitments_bind_their_own_predecessors() {
    // Arrange
    let genesis = vector("actual-genesis");
    let funding = vector("actual-funding");
    let spending = vector("actual-spending");
    let replacement = vector("actual-replacement");
    // Act
    let genesis_header = generate(genesis)
        .header(BlockHash::default(), 0, FilterHeaderPredecessor::Genesis)
        .expect("genesis");
    let funding_header = generate(funding)
        .header(
            BlockHash::from_byte_array(raw32(genesis.block_hash_raw)),
            1,
            FilterHeaderPredecessor::Previous {
                block_hash: BlockHash::from_byte_array(raw32(genesis.block_hash_raw)),
                height: 0,
                header: genesis_header,
            },
        )
        .expect("funding");
    // Assert
    assert_eq!(funding_header.as_bytes(), &raw32(funding.header_raw));
    for (row, parent, height) in [
        (spending, funding, 2),
        (replacement, funding, 2),
        (vector("actual-successor"), spending, 3),
        (vector("actual-replacement-successor"), replacement, 3),
    ] {
        let parent_hash = BlockHash::from_byte_array(raw32(parent.block_hash_raw));
        let header = generate(row)
            .header(
                parent_hash,
                height,
                FilterHeaderPredecessor::Previous {
                    block_hash: parent_hash,
                    height: height - 1,
                    header: FilterHeader::from_byte_array(raw32(parent.header_raw)),
                },
            )
            .expect("branch predecessor");
        assert_eq!(header.as_bytes(), &raw32(row.header_raw), "{}", row.name);
    }
    assert_ne!(
        vector("actual-successor").header_raw,
        vector("actual-replacement-successor").header_raw
    );
    let reversed = compute_filter_header(
        FilterHash::from_byte_array(raw32(spending.predecessor_raw)),
        FilterHeader::from_byte_array(raw32(spending.hash_raw)),
    );
    assert_ne!(reversed.as_bytes(), &raw32(spending.header_raw));
}

#[test]
fn byte_siphash_length_boundaries_match_pinned_python_helper() {
    // Arrange
    for &(length, expected) in SIPHASH_LENGTH_VECTORS {
        let bytes: Vec<_> = (0..length).map(|index| (index & 255) as u8).collect();
        // Act
        let actual = siphash_bytes(0x0706_0504_0302_0100, 0x0f0e_0d0c_0b0a_0908, &bytes);
        // Assert
        assert_eq!(actual, expected, "length {length}");
    }
}
