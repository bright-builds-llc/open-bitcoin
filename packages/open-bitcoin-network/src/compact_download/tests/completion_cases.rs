// Parity breadcrumbs:
// - packages/bitcoin-knots/src/blockencodings.h
// - packages/bitcoin-knots/src/blockencodings.cpp
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/net_processing.h
// - packages/bitcoin-knots/test/functional/p2p_compactblocks.py

use super::*;

pub(super) fn oversized_candidate_partial(
    with_missing: bool,
) -> (CompactBlockPayload, PartialCompactBlock, Transaction) {
    // A synthetic short ID selects an owned transaction whose script cannot be encoded.
    let mut oversized = sample_transaction(0x73);
    oversized.outputs[0].script_pubkey =
        ScriptBuf::from_serialized_bytes(vec![
            0;
            usize::try_from(open_bitcoin_codec::MAX_SIZE + 1)
                .expect("bounded codec oversize")
        ]);
    let missing = sample_transaction(0x74);
    let header = sample_header();
    let nonce = 42;
    let selector = open_bitcoin_codec::short_id_selector_from_header_and_nonce(&header, nonce);
    let synthetic_wtxid = Wtxid::from_byte_array([0x73; 32]);
    let mut short_ids = vec![open_bitcoin_consensus::compact_short_id_for_wtxid(
        selector,
        &synthetic_wtxid,
    )];
    if with_missing {
        let missing_wtxid = transaction_wtxid(&missing).expect("ordinary missing transaction");
        short_ids.push(open_bitcoin_consensus::compact_short_id_for_wtxid(
            selector,
            &missing_wtxid,
        ));
    }
    let payload = CompactBlockPayload {
        header,
        nonce,
        short_ids,
        prefilled_transactions: vec![PrefilledTransaction {
            index_delta: 0,
            transaction: coinbase_transaction(),
        }],
    };
    let mut partial = PartialCompactBlock::new();
    assert_eq!(
        init_partial_compact_block(
            &mut partial,
            &payload,
            [(&synthetic_wtxid, &oversized)],
            std::iter::empty::<(&Wtxid, &Transaction)>(),
        ),
        CompactReconstructionOutcome::Ready {
            missing_indexes: if with_missing { vec![2] } else { Vec::new() },
        }
    );
    (payload, partial, missing)
}

#[test]
fn selected_complete_merkle_mutation_falls_back_with_partial_retained() {
    // Arrange
    let body = crate::test_block_commitments::coherent_body(
        sample_header(),
        vec![coinbase_transaction()],
        false,
    );
    let mut changed_coinbase = body.transactions[0].clone();
    changed_coinbase.lock_time += 1;
    let payload = CompactBlockPayload {
        header: body.header.clone(),
        nonce: 5,
        short_ids: Vec::new(),
        prefilled_transactions: vec![PrefilledTransaction {
            index_delta: 0,
            transaction: changed_coinbase,
        }],
    };
    let block_hash = block_hash(&payload.header);
    let mut partial = PartialCompactBlock::new();
    assert_eq!(
        init_partial_compact_block(
            &mut partial,
            &payload,
            std::iter::empty::<(&Wtxid, &Transaction)>(),
            std::iter::empty(),
        ),
        CompactReconstructionOutcome::Ready {
            missing_indexes: Vec::new()
        }
    );
    let original_partial = partial.clone();
    let in_flight = super::super::CompactDownloadInFlight {
        partial,
        getblocktxn_in_flight: false,
        requested_indexes: Vec::new(),
        started_at_unix: 0,
    };

    // Act
    let result = try_complete_compact_download_selected(block_hash, &in_flight, true);

    // Assert
    assert_eq!(
        result,
        Ok(CompactDownloadCompletionOutcome::Fallback(FullBlockFetch {
            block_hash
        }))
    );
    assert_eq!(in_flight.partial, original_partial);
    assert_eq!(in_flight.partial.block_hash(), Some(block_hash));
}

#[test]
fn public_complete_matched_body_is_completed() {
    // Arrange
    let body = crate::test_block_commitments::coherent_body(
        sample_header(),
        vec![coinbase_transaction()],
        false,
    );
    let payload = CompactBlockPayload {
        header: body.header.clone(),
        nonce: 5,
        short_ids: Vec::new(),
        prefilled_transactions: vec![PrefilledTransaction {
            index_delta: 0,
            transaction: body.transactions[0].clone(),
        }],
    };
    let block_hash = block_hash(&payload.header);
    let mut partial = PartialCompactBlock::new();
    assert_eq!(
        init_partial_compact_block(
            &mut partial,
            &payload,
            std::iter::empty::<(&Wtxid, &Transaction)>(),
            std::iter::empty(),
        ),
        CompactReconstructionOutcome::Ready {
            missing_indexes: Vec::new()
        }
    );
    let in_flight = super::super::CompactDownloadInFlight {
        partial,
        getblocktxn_in_flight: false,
        requested_indexes: Vec::new(),
        started_at_unix: 0,
    };

    // Act
    let result = super::super::try_complete_compact_download(block_hash, &in_flight, true);

    // Assert
    assert_eq!(
        result,
        Ok(CompactDownloadCompletionOutcome::Completed(body))
    );
}

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
        try_complete_compact_download_selected(block_hash, &in_flight, true),
        Ok(CompactDownloadCompletionOutcome::Suppressed(
            CompactDownloadSuppressionReason::CompactReconstructionFailed
        ))
    );

    let in_flight = super::super::CompactDownloadInFlight {
        partial: PartialCompactBlock::new(),
        getblocktxn_in_flight: false,
        requested_indexes: Vec::new(),
        started_at_unix: 0,
    };
    assert_eq!(
        try_complete_compact_download_selected(block_hash, &in_flight, true),
        Ok(CompactDownloadCompletionOutcome::Fallback(FullBlockFetch {
            block_hash
        }))
    );
}

#[test]
fn init_compact_block_download_completes_prefilled_only_blocks() {
    let body = crate::test_block_commitments::coherent_body(
        sample_header(),
        vec![coinbase_transaction()],
        false,
    );
    let header = body.header.clone();
    let payload = CompactBlockPayload {
        header: header.clone(),
        nonce: 5,
        short_ids: Vec::new(),
        prefilled_transactions: vec![PrefilledTransaction {
            index_delta: 0,
            transaction: body.transactions[0].clone(),
        }],
    };
    let mut peer_state = CompactDownloadPeerState::new();

    let outcome = init_compact_block_download_selected(
        (activated_policy(), true),
        &mut peer_state,
        &payload,
        (eligible_input(&payload.header), 1_000, true),
        std::iter::empty::<(&Wtxid, &Transaction)>(),
        std::iter::empty(),
    );

    let Ok(CompactBlockInitOutcome::Completed { block }) = outcome else {
        panic!("expected selected completed outcome: {outcome:?}");
    };
    assert_eq!(block, body);
    crate::test_block_commitments::assert_coherent(&block);
    assert_eq!(block.header, header);
    assert_eq!(block.transactions.len(), 1);
    assert!(peer_state.in_flight.is_empty());
}

#[test]
fn completed_or_fallback_init_falls_back_for_uninitialized_partial() {
    let block_hash = BlockHash::from_byte_array([0xcd; 32]);

    assert_eq!(
        completed_or_fallback_init_selected(block_hash, &PartialCompactBlock::new(), true),
        Ok(CompactBlockInitOutcome::Fallback(FullBlockFetch {
            block_hash
        }))
    );
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
        complete_applied_download_selected(
            &mut CompactDownloadPeerState::new(),
            block_hash,
            block_hash,
            true,
        ),
        Ok(Err(CompactBlockTxnHandleOutcome::NoMatchingInFlight))
    );

    assert_eq!(
        finish_applied_handle_selected(
            &mut CompactDownloadPeerState::new(),
            block_hash,
            block_hash,
            true,
        ),
        Ok(CompactBlockTxnHandleOutcome::NoMatchingInFlight)
    );
}

#[test]
fn public_completion_refuses_owned_oversized_candidate_without_publishing_block() {
    // Arrange
    let (payload, partial, _) = oversized_candidate_partial(false);
    let in_flight = super::super::CompactDownloadInFlight {
        partial,
        getblocktxn_in_flight: false,
        requested_indexes: Vec::new(),
        started_at_unix: 0,
    };

    // Act
    let result =
        super::super::try_complete_compact_download(block_hash(&payload.header), &in_flight, true);

    // Assert
    assert_eq!(
        result,
        Err(open_bitcoin_codec::CodecError::CompactSizeTooLarge(
            open_bitcoin_codec::MAX_SIZE + 1,
        ))
    );
    assert!(in_flight.partial.is_transaction_available(1));
    assert!(in_flight.partial.missing_transaction_indexes().is_empty());
}
