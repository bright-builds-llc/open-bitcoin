// Parity breadcrumbs:
// - packages/bitcoin-knots/src/blockencodings.h
// - packages/bitcoin-knots/src/blockencodings.cpp
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/net_processing.h
// - packages/bitcoin-knots/test/functional/p2p_compactblocks.py

use super::*;

#[test]
fn schedule_missing_transaction_request_requires_activation_and_in_flight_state() {
    let (payload, _, _) = compact_payload_with_missing_short_id();
    let block_hash = block_hash(&payload.header);
    let mut peer_state = CompactDownloadPeerState::new();
    let disabled = BlockRelayActivationPolicy::default();

    assert_eq!(
        schedule_missing_transaction_request(disabled, true, &mut peer_state, block_hash),
        ScheduleMissingTransactionOutcome::Suppressed(
            ScheduleMissingTransactionSuppressionReason::CompactRelayDisabled
        )
    );

    peer_state.in_flight.insert(
        block_hash,
        super::super::CompactDownloadInFlight {
            partial: PartialCompactBlock::new(),
            getblocktxn_in_flight: false,
            requested_indexes: Vec::new(),
            started_at_unix: 0,
        },
    );

    assert_eq!(
        schedule_missing_transaction_request(
            activated_policy(),
            false,
            &mut peer_state,
            block_hash
        ),
        ScheduleMissingTransactionOutcome::Suppressed(
            ScheduleMissingTransactionSuppressionReason::PeerNotCompactCapable
        )
    );
}

#[test]
fn init_compact_block_download_schedules_getblocktxn_for_missing_indexes() {
    let (payload, _, _) = compact_payload_with_missing_short_id();
    let block_hash = block_hash(&payload.header);
    let mut peer_state = CompactDownloadPeerState::new();

    let outcome = init_compact_block_download(
        activated_policy(),
        true,
        &mut peer_state,
        &payload,
        eligible_input(&payload.header),
        1_000,
        std::iter::empty::<(&Wtxid, &Transaction)>(),
        std::iter::empty::<(&Wtxid, &Transaction)>(),
    );

    let CompactBlockInitOutcome::Ready { actions, .. } = outcome else {
        panic!("expected ready outcome");
    };

    assert_eq!(actions.len(), 1);
    assert!(peer_state.in_flight.contains_key(&block_hash));
    let in_flight = peer_state.in_flight.get(&block_hash).expect("in flight");
    assert!(in_flight.getblocktxn_in_flight);
    assert_eq!(in_flight.requested_indexes, vec![1_u16]);
}

#[test]
fn evaluate_compact_block_download_eligibility_accepts_tip_extension() {
    let header = sample_header();
    let input = CompactBlockDownloadEligibilityInput {
        local_best_height: 5,
        best_tip_hash: header.previous_block_hash,
        maybe_known_block_height: None,
    };

    assert_eq!(
        evaluate_compact_block_download_eligibility(&header, input),
        CompactBlockDownloadEligibility::Eligible
    );
}

#[test]
fn peer_supports_compact_download_reflects_peer_capability() {
    assert!(peer_supports_compact_download(true));
    assert!(!peer_supports_compact_download(false));
}

#[test]
fn evaluate_compact_block_download_eligibility_rejects_null_and_unknown_headers() {
    let null_header = BlockHeader {
        version: 0,
        previous_block_hash: BlockHash::from_byte_array([0_u8; 32]),
        merkle_root: MerkleRoot::from_byte_array([0_u8; 32]),
        time: 0,
        bits: 0,
        nonce: 0,
    };
    let input = CompactBlockDownloadEligibilityInput {
        local_best_height: 5,
        best_tip_hash: BlockHash::from_byte_array([1_u8; 32]),
        maybe_known_block_height: Some(1),
    };

    assert_eq!(
        evaluate_compact_block_download_eligibility(&null_header, input),
        CompactBlockDownloadEligibility::TooFarFromTip
    );

    let header = sample_header();
    let unknown_input = CompactBlockDownloadEligibilityInput {
        local_best_height: 5,
        best_tip_hash: BlockHash::from_byte_array([9_u8; 32]),
        maybe_known_block_height: None,
    };
    assert_eq!(
        evaluate_compact_block_download_eligibility(&header, unknown_input),
        CompactBlockDownloadEligibility::UnknownHeaderNotExtendingTip
    );
}

#[test]
fn evaluate_compact_block_download_eligibility_rejects_future_height() {
    let header = sample_header();
    let input = CompactBlockDownloadEligibilityInput {
        local_best_height: 5,
        best_tip_hash: header.previous_block_hash,
        maybe_known_block_height: Some(100),
    };

    assert_eq!(
        evaluate_compact_block_download_eligibility(&header, input),
        CompactBlockDownloadEligibility::TooFarFromTip
    );
}

#[test]
fn schedule_missing_transaction_request_handles_missing_in_flight_and_indexes() {
    let block_hash = BlockHash::from_byte_array([4_u8; 32]);
    let mut peer_state = CompactDownloadPeerState::new();

    assert_eq!(
        schedule_missing_transaction_request(activated_policy(), true, &mut peer_state, block_hash),
        ScheduleMissingTransactionOutcome::Suppressed(
            ScheduleMissingTransactionSuppressionReason::NoInFlightBlock
        )
    );

    peer_state.in_flight.insert(
        block_hash,
        super::super::CompactDownloadInFlight {
            partial: PartialCompactBlock::new(),
            getblocktxn_in_flight: false,
            requested_indexes: Vec::new(),
            started_at_unix: 0,
        },
    );

    assert_eq!(
        schedule_missing_transaction_request(activated_policy(), true, &mut peer_state, block_hash),
        ScheduleMissingTransactionOutcome::Suppressed(
            ScheduleMissingTransactionSuppressionReason::NoMissingIndexes
        )
    );
}

#[test]
fn scheduled_init_request_removes_in_flight_when_schedule_is_suppressed() {
    let block_hash = BlockHash::from_byte_array([0xce; 32]);
    let mut peer_state = CompactDownloadPeerState::new();
    peer_state.in_flight.insert(
        block_hash,
        super::super::CompactDownloadInFlight {
            partial: PartialCompactBlock::new(),
            getblocktxn_in_flight: false,
            requested_indexes: Vec::new(),
            started_at_unix: 0,
        },
    );

    assert!(matches!(
        scheduled_init_request(
            block_hash,
            &mut peer_state,
            ScheduleMissingTransactionOutcome::Suppressed(
                ScheduleMissingTransactionSuppressionReason::CompactRelayDisabled
            ),
        ),
        Err(CompactBlockInitOutcome::Fallback(FullBlockFetch { block_hash: hash }))
            if hash == block_hash
    ));
    assert!(!peer_state.in_flight.contains_key(&block_hash));
}

#[test]
fn finalize_ready_init_returns_fallback_when_schedule_is_suppressed() {
    let block_hash = BlockHash::from_byte_array([0xd0; 32]);
    let mut peer_state = CompactDownloadPeerState::new();
    peer_state.in_flight.insert(
        block_hash,
        super::super::CompactDownloadInFlight {
            partial: PartialCompactBlock::new(),
            getblocktxn_in_flight: false,
            requested_indexes: vec![1],
            started_at_unix: 0,
        },
    );

    assert!(matches!(
        finalize_ready_init(
            block_hash,
            &mut peer_state,
            ScheduleMissingTransactionOutcome::Suppressed(
                ScheduleMissingTransactionSuppressionReason::CompactRelayDisabled
            ),
        ),
        CompactBlockInitOutcome::Fallback(FullBlockFetch { block_hash: expected })
            if expected == block_hash
    ));
}
