// Parity breadcrumbs:
// - packages/bitcoin-knots/src/blockencodings.h
// - packages/bitcoin-knots/src/blockencodings.cpp
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/net_processing.h
// - packages/bitcoin-knots/test/functional/p2p_compactblocks.py

use super::*;

#[test]
fn absolute_indexes_to_differential_deltas_matches_codec_expansion() {
    let missing = vec![2_u16, 5, 7];
    let request =
        build_get_block_transactions_request(BlockHash::from_byte_array([3_u8; 32]), &missing);

    let expanded = expand_block_transaction_indexes(&request).expect("expand");

    assert_eq!(request.index_deltas, vec![2, 2, 1]);
    assert_eq!(expanded, missing);
}

#[test]
fn apply_block_transactions_rejects_unexpected_hash_and_too_many_transactions() {
    let (payload, missing_tx, _) = compact_payload_with_missing_short_id();
    let block_hash = block_hash(&payload.header);
    let mut partial = PartialCompactBlock::new();
    let _ = init_partial_compact_block(
        &mut partial,
        &payload,
        std::iter::empty(),
        std::iter::empty::<(&Wtxid, &Transaction)>(),
    );

    assert_eq!(
        apply_block_transactions(
            &mut partial,
            &BlockTransactions {
                block_hash: BlockHash::from_byte_array([9_u8; 32]),
                transactions: vec![missing_tx.clone()],
            },
            block_hash,
        ),
        CompactBlockTxnOutcome::Misbehavior(CompactBlockTxnMisbehavior::UnexpectedBlockHash)
    );

    assert_eq!(
        apply_block_transactions(
            &mut partial,
            &BlockTransactions {
                block_hash,
                transactions: vec![missing_tx.clone(), missing_tx],
            },
            block_hash,
        ),
        CompactBlockTxnOutcome::Misbehavior(CompactBlockTxnMisbehavior::TooManyTransactions)
    );
}

#[test]
fn duplicate_in_flight_getblocktxn_request_is_suppressed() {
    let (payload, _, _) = compact_payload_with_missing_short_id();
    let block_hash = block_hash(&payload.header);
    let mut peer_state = CompactDownloadPeerState::new();
    let initialized = init_compact_block_download(
        (activated_policy(), true),
        &mut peer_state,
        &payload,
        (eligible_input(&payload.header), 1_000, true),
        std::iter::empty::<(&Wtxid, &Transaction)>(),
        std::iter::empty(),
    );
    assert!(
        matches!(initialized, Ok(CompactBlockInitOutcome::Ready { block_hash: hash, .. }) if hash == block_hash)
    );

    assert_eq!(
        schedule_missing_transaction_request(activated_policy(), true, &mut peer_state, block_hash),
        ScheduleMissingTransactionOutcome::Suppressed(
            ScheduleMissingTransactionSuppressionReason::DuplicateInFlightRequest
        )
    );
}

#[test]
fn old_block_far_from_tip_falls_back_to_full_block_fetch() {
    let (payload, _, _) = compact_payload_with_missing_short_id();
    let block_hash = block_hash(&payload.header);
    let mut peer_state = CompactDownloadPeerState::new();
    let eligibility = CompactBlockDownloadEligibilityInput {
        local_best_height: 100,
        best_tip_hash: BlockHash::from_byte_array([9_u8; 32]),
        maybe_known_block_height: Some(1),
    };

    let outcome = init_compact_block_download(
        (activated_policy(), true),
        &mut peer_state,
        &payload,
        (eligibility, 1_000, true),
        std::iter::empty::<(&Wtxid, &Transaction)>(),
        std::iter::empty(),
    );

    assert!(matches!(
        outcome,
        Ok(CompactBlockInitOutcome::Fallback(fetch)) if fetch.block_hash == block_hash
    ));
    assert!(peer_state.in_flight.is_empty());
}

#[test]
fn phase167_compact_public_init_mutated_prefilled_falls_back() {
    // Arrange
    let body = crate::test_block_commitments::coherent_body(
        sample_header(),
        vec![coinbase_transaction()],
        false,
    );
    let mut changed_coinbase = body.transactions[0].clone();
    changed_coinbase.lock_time += 1;
    let payload = CompactBlockPayload {
        header: body.header,
        nonce: 5,
        short_ids: Vec::new(),
        prefilled_transactions: vec![PrefilledTransaction {
            index_delta: 0,
            transaction: changed_coinbase,
        }],
    };
    let original_payload = payload.clone();
    let block_hash = block_hash(&payload.header);
    let mut peer_state = CompactDownloadPeerState::new();

    // Act
    let result = init_compact_block_download(
        (activated_policy(), true),
        &mut peer_state,
        &payload,
        (eligible_input(&payload.header), 1_000, true),
        std::iter::empty::<(&Wtxid, &Transaction)>(),
        std::iter::empty(),
    );

    // Assert
    assert_eq!(
        result,
        Ok(CompactBlockInitOutcome::Fallback(FullBlockFetch {
            block_hash
        }))
    );
    assert!(peer_state.in_flight.is_empty());
    assert_eq!(payload, original_payload);
}

#[test]
fn phase167_compact_public_init_matched_prefilled_completes() {
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
    let mut peer_state = CompactDownloadPeerState::new();

    // Act
    let result = init_compact_block_download(
        (activated_policy(), true),
        &mut peer_state,
        &payload,
        (eligible_input(&payload.header), 1_000, true),
        std::iter::empty::<(&Wtxid, &Transaction)>(),
        std::iter::empty(),
    );

    // Assert
    assert_eq!(
        result,
        Ok(CompactBlockInitOutcome::Completed { block: body })
    );
    assert!(peer_state.in_flight.is_empty());
}

#[test]
fn compact_download_actions_to_peer_actions_maps_all_variants() {
    let block_hash = BlockHash::from_byte_array([6_u8; 32]);
    let request = build_get_block_transactions_request(block_hash, &[1_u16]);
    let header = sample_header();
    let block = open_bitcoin_primitives::Block {
        header,
        transactions: vec![coinbase_transaction()],
    };

    let actions = compact_download_actions_to_peer_actions(
        vec![
            CompactDownloadAction::SendGetBlockTxn(request.clone()),
            CompactDownloadAction::RequestFullBlock(FullBlockFetch { block_hash }),
            CompactDownloadAction::ReceivedBlock(block.clone()),
        ],
        open_bitcoin_primitives::InventoryType::Block,
    );

    assert_eq!(actions.len(), 3);
    assert!(matches!(
        &actions[0],
        crate::PeerAction::Send(crate::WireNetworkMessage::GetBlockTxn(_))
    ));
    assert!(matches!(
        &actions[1],
        crate::PeerAction::Send(crate::WireNetworkMessage::GetData(inventory))
            if inventory.inventory == vec![open_bitcoin_primitives::InventoryVector {
                inventory_type: open_bitcoin_primitives::InventoryType::Block,
                object_hash: block_hash.into(),
            }]
    ));
    assert!(matches!(
        &actions[2],
        crate::PeerAction::ReceivedBlock(received) if received == &block
    ));

    let witness_actions = compact_download_actions_to_peer_actions(
        vec![CompactDownloadAction::RequestFullBlock(FullBlockFetch {
            block_hash,
        })],
        open_bitcoin_primitives::InventoryType::WitnessBlock,
    );
    assert!(matches!(
        witness_actions.as_slice(),
        [crate::PeerAction::Send(crate::WireNetworkMessage::GetData(inventory))]
            if inventory.inventory == vec![open_bitcoin_primitives::InventoryVector {
                inventory_type: open_bitcoin_primitives::InventoryType::WitnessBlock,
                object_hash: block_hash.into(),
            }]
    ));
}

#[test]
fn init_compact_block_download_suppressed_when_ineligible_or_duplicate() {
    let (payload, _, _) = compact_payload_with_missing_short_id();
    let block_hash = block_hash(&payload.header);
    let mut peer_state = CompactDownloadPeerState::new();

    let disabled_outcome = init_compact_block_download(
        (BlockRelayActivationPolicy::default(), true),
        &mut peer_state,
        &payload,
        (eligible_input(&payload.header), 1_000, true),
        std::iter::empty::<(&Wtxid, &Transaction)>(),
        std::iter::empty(),
    );
    assert!(matches!(
        disabled_outcome,
        Ok(CompactBlockInitOutcome::Suppressed(
            CompactDownloadSuppressionReason::CompactPeerIneligible
        ))
    ));

    let peer_ineligible = init_compact_block_download(
        (activated_policy(), false),
        &mut peer_state,
        &payload,
        (eligible_input(&payload.header), 1_000, true),
        std::iter::empty::<(&Wtxid, &Transaction)>(),
        std::iter::empty(),
    );
    assert!(matches!(
        peer_ineligible,
        Ok(CompactBlockInitOutcome::Suppressed(
            CompactDownloadSuppressionReason::CompactPeerIneligible
        ))
    ));

    let initialized = init_compact_block_download(
        (activated_policy(), true),
        &mut peer_state,
        &payload,
        (eligible_input(&payload.header), 1_000, true),
        std::iter::empty::<(&Wtxid, &Transaction)>(),
        std::iter::empty(),
    );
    assert!(
        matches!(initialized, Ok(CompactBlockInitOutcome::Ready { block_hash: hash, .. }) if hash == block_hash)
    );
    let duplicate = init_compact_block_download(
        (activated_policy(), true),
        &mut peer_state,
        &payload,
        (eligible_input(&payload.header), 1_001, true),
        std::iter::empty::<(&Wtxid, &Transaction)>(),
        std::iter::empty(),
    );
    assert!(matches!(
        duplicate,
        Ok(CompactBlockInitOutcome::Suppressed(
            CompactDownloadSuppressionReason::CompactBlockAlreadyInFlight
        ))
    ));
    assert!(peer_state.in_flight.contains_key(&block_hash));
}

#[test]
fn init_compact_block_download_misbehaves_on_invalid_reconstruction() {
    let header = sample_header();
    let mut peer_state = CompactDownloadPeerState::new();

    let empty_payload = CompactBlockPayload {
        header: header.clone(),
        nonce: 1,
        short_ids: Vec::new(),
        prefilled_transactions: Vec::new(),
    };
    let invalid = init_compact_block_download(
        (activated_policy(), true),
        &mut peer_state,
        &empty_payload,
        (eligible_input(&header), 1_000, true),
        std::iter::empty::<(&Wtxid, &Transaction)>(),
        std::iter::empty(),
    );
    assert_eq!(
        invalid,
        Ok(CompactBlockInitOutcome::Misbehavior(
            CompactReconstructionInvalidReason::EmptyCompactBlock
        ))
    );
    assert!(peer_state.in_flight.is_empty());
}

#[test]
fn init_compact_block_download_falls_back_on_short_id_collision() {
    let header = sample_header();
    let expected_block_hash = block_hash(&header);
    let mut peer_state = CompactDownloadPeerState::new();

    let colliding_short_id = short_id_from_masked_u64(0x00aa_bbcc_dd11);
    let collision_payload = CompactBlockPayload {
        header,
        nonce: 1,
        short_ids: vec![colliding_short_id, colliding_short_id],
        prefilled_transactions: vec![PrefilledTransaction {
            index_delta: 0,
            transaction: coinbase_transaction(),
        }],
    };
    let failed = init_compact_block_download(
        (activated_policy(), true),
        &mut peer_state,
        &collision_payload,
        (eligible_input(&collision_payload.header), 1_000, true),
        std::iter::empty::<(&Wtxid, &Transaction)>(),
        std::iter::empty(),
    );
    assert!(matches!(
        failed,
        Ok(CompactBlockInitOutcome::Fallback(fetch)) if fetch.block_hash == expected_block_hash
    ));
}
