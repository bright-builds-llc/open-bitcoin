// Parity breadcrumbs:
// - packages/bitcoin-knots/src/blockencodings.h
// - packages/bitcoin-knots/src/blockencodings.cpp
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/net_processing.h
// - packages/bitcoin-knots/test/functional/p2p_compactblocks.py

use super::*;

#[test]
fn try_complete_compact_download_reports_suppressed_and_fallback_outcomes() {
    let block_hash = BlockHash::from_byte_array([5_u8; 32]);
    let (payload, _, _) = compact_payload_with_missing_short_id();
    let mut partial = PartialCompactBlock::new();
    let _ = init_partial_compact_block(
        &mut partial,
        &payload,
        std::iter::empty::<(&Wtxid, &Transaction)>(),
        std::iter::empty(),
    );
    let in_flight = super::super::CompactDownloadInFlight {
        partial,
        getblocktxn_in_flight: false,
        requested_indexes: Vec::new(),
        started_at_unix: 0,
    };

    assert_eq!(
        try_complete_compact_download(block_hash, &in_flight),
        CompactDownloadCompletionOutcome::Suppressed(
            CompactDownloadSuppressionReason::CompactReconstructionFailed
        )
    );

    let in_flight = super::super::CompactDownloadInFlight {
        partial: PartialCompactBlock::new(),
        getblocktxn_in_flight: false,
        requested_indexes: Vec::new(),
        started_at_unix: 0,
    };
    assert_eq!(
        try_complete_compact_download(block_hash, &in_flight),
        CompactDownloadCompletionOutcome::Fallback(FullBlockFetch { block_hash })
    );
}

#[test]
fn init_compact_block_download_completes_prefilled_only_blocks() {
    let header = sample_header();
    let payload = CompactBlockPayload {
        header: header.clone(),
        nonce: 5,
        short_ids: Vec::new(),
        prefilled_transactions: vec![PrefilledTransaction {
            index_delta: 0,
            transaction: coinbase_transaction(),
        }],
    };
    let mut peer_state = CompactDownloadPeerState::new();

    let outcome = init_compact_block_download(
        activated_policy(),
        true,
        &mut peer_state,
        &payload,
        eligible_input(&payload.header),
        1_000,
        std::iter::empty::<(&Wtxid, &Transaction)>(),
        std::iter::empty(),
    );

    let CompactBlockInitOutcome::Completed { block } = outcome else {
        panic!("expected completed outcome");
    };
    assert_eq!(block.header, header);
    assert_eq!(block.transactions.len(), 1);
    assert!(peer_state.in_flight.is_empty());
}

#[test]
fn completed_or_fallback_init_falls_back_for_uninitialized_partial() {
    let block_hash = BlockHash::from_byte_array([0xcd; 32]);

    assert!(matches!(
        completed_or_fallback_init(block_hash, &PartialCompactBlock::new()),
        CompactBlockInitOutcome::Fallback(FullBlockFetch { block_hash: hash }) if hash == block_hash
    ));
}

#[test]
fn completion_actions_from_outcome_maps_all_completion_variants() {
    let block_hash = BlockHash::from_byte_array([0xcf; 32]);
    let block = open_bitcoin_primitives::Block {
        header: sample_header(),
        transactions: vec![coinbase_transaction()],
    };

    let actions = completion_actions_from_outcome(
        block_hash,
        CompactDownloadCompletionOutcome::Completed(block.clone()),
    );
    assert!(matches!(
        &actions[0],
        CompactDownloadAction::ReceivedBlock(received) if received == &block
    ));
    assert!(matches!(
        completion_actions_from_outcome(
            block_hash,
            CompactDownloadCompletionOutcome::Fallback(FullBlockFetch { block_hash }),
        )[0],
        CompactDownloadAction::RequestFullBlock(FullBlockFetch { block_hash: expected })
            if expected == block_hash
    ));
    assert!(matches!(
        completion_actions_from_outcome(
            block_hash,
            CompactDownloadCompletionOutcome::Suppressed(
                CompactDownloadSuppressionReason::CompactReconstructionFailed
            ),
        )[0],
        CompactDownloadAction::RequestFullBlock(FullBlockFetch { block_hash: expected })
            if expected == block_hash
    ));
}

#[test]
fn take_in_flight_download_reports_missing_state() {
    let block_hash = BlockHash::from_byte_array([0xdf; 32]);
    let mut peer_state = CompactDownloadPeerState::new();

    assert_eq!(
        take_in_flight_download(&mut peer_state, block_hash),
        Err(CompactBlockTxnHandleOutcome::NoMatchingInFlight)
    );
}

#[test]
fn complete_applied_download_reports_missing_in_flight_state() {
    let block_hash = BlockHash::from_byte_array([0xd1; 32]);

    assert_eq!(
        complete_applied_download(&mut CompactDownloadPeerState::new(), block_hash, block_hash,),
        Err(CompactBlockTxnHandleOutcome::NoMatchingInFlight)
    );

    assert_eq!(
        finish_applied_handle(&mut CompactDownloadPeerState::new(), block_hash, block_hash),
        CompactBlockTxnHandleOutcome::NoMatchingInFlight
    );
}
