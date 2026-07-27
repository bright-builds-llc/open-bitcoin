// Parity breadcrumbs:
// - packages/bitcoin-knots/src/blockencodings.cpp
// - packages/bitcoin-knots/src/blockencodings.h
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/test/functional/p2p_compactblocks.py

use super::*;

#[test]
fn lifecycle_cleanup_clears_state_on_block_connect() {
    let header = sample_header();
    let coinbase = coinbase_transaction();
    let payload = compact_payload_with_short_ids(
        header,
        1,
        vec![PrefilledTransaction {
            index_delta: 0,
            transaction: coinbase,
        }],
        &[],
    );

    let mut state = PartialCompactBlock::new();
    let outcome = init_partial_compact_block(
        &mut state,
        &payload,
        std::iter::empty::<(&Wtxid, &Transaction)>(),
        std::iter::empty::<(&Wtxid, &Transaction)>(),
    );
    assert!(matches!(
        outcome,
        CompactReconstructionOutcome::Ready { .. }
    ));

    state.on_block_connected();
    assert!(!state.is_initialized());
    assert_eq!(state.missing_transaction_indexes(), Vec::<u16>::new());
}

#[test]
fn partial_compact_block_default_is_uninitialized() {
    let state = PartialCompactBlock::default();

    assert!(!state.is_initialized());
    assert!(!state.is_transaction_available(0));
}

#[test]
fn block_hash_returns_none_until_initialized() {
    let state = PartialCompactBlock::new();
    assert_eq!(state.block_hash(), None);

    let header = sample_header();
    let payload = compact_payload_with_short_ids(
        header,
        1,
        vec![PrefilledTransaction {
            index_delta: 0,
            transaction: coinbase_transaction(),
        }],
        &[],
    );
    let mut state = PartialCompactBlock::new();
    let _ = init_partial_compact_block(
        &mut state,
        &payload,
        std::iter::empty::<(&Wtxid, &Transaction)>(),
        std::iter::empty(),
    );

    assert_eq!(
        state.block_hash(),
        Some(open_bitcoin_consensus::block_hash(
            state.header().expect("header")
        ))
    );
}
