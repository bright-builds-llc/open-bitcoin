// Parity breadcrumbs:
// - packages/bitcoin-knots/src/blockencodings.h
// - packages/bitcoin-knots/src/blockencodings.cpp
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/net_processing.h
// - packages/bitcoin-knots/test/functional/p2p_compactblocks.py

use super::*;

#[test]
fn handle_block_transactions_completes_block_and_emits_received_block_action() {
    let (payload, missing_tx, _) = compact_payload_with_missing_short_id();
    let block_hash = block_hash(&payload.header);
    let mut peer_state = CompactDownloadPeerState::new();
    let mut partial = PartialCompactBlock::new();
    let outcome = init_partial_compact_block(
        &mut partial,
        &payload,
        std::iter::empty::<(&Wtxid, &Transaction)>(),
        std::iter::empty::<(&Wtxid, &Transaction)>(),
    );
    assert!(matches!(
        outcome,
        CompactReconstructionOutcome::Ready { .. }
    ));

    peer_state.in_flight.insert(
        block_hash,
        super::super::CompactDownloadInFlight {
            partial,
            getblocktxn_in_flight: true,
            requested_indexes: vec![1],
            started_at_unix: 0,
        },
    );

    let handle_outcome = handle_block_transactions(
        activated_policy(),
        true,
        &mut peer_state,
        &BlockTransactions {
            block_hash,
            transactions: vec![missing_tx.clone()],
        },
    );

    let CompactBlockTxnHandleOutcome::Progress { actions } = handle_outcome else {
        panic!("expected progress outcome");
    };

    assert_eq!(actions.len(), 1);
    assert!(matches!(
        actions[0],
        super::super::CompactDownloadAction::ReceivedBlock(_)
    ));
    assert!(!peer_state.in_flight.contains_key(&block_hash));

    let mut rebuilt = PartialCompactBlock::new();
    let _ = init_partial_compact_block(
        &mut rebuilt,
        &payload,
        std::iter::empty::<(&Wtxid, &Transaction)>(),
        [(&transaction_wtxid(&missing_tx).expect("wtxid"), &missing_tx)],
    );
    let completed = fill_block(&rebuilt).expect("filled block");
    assert_eq!(completed.transactions.len(), 2);
}

#[test]
fn handle_block_transactions_reports_missing_in_flight_and_duplicate_responses() {
    let (payload, missing_tx, _) = compact_payload_with_missing_short_id();
    let block_hash = block_hash(&payload.header);
    let mut peer_state = CompactDownloadPeerState::new();

    assert_eq!(
        handle_block_transactions(
            activated_policy(),
            true,
            &mut peer_state,
            &BlockTransactions {
                block_hash,
                transactions: vec![missing_tx.clone()],
            },
        ),
        CompactBlockTxnHandleOutcome::NoMatchingInFlight
    );

    let mut partial = PartialCompactBlock::new();
    let _ = init_partial_compact_block(
        &mut partial,
        &payload,
        std::iter::empty(),
        std::iter::empty::<(&Wtxid, &Transaction)>(),
    );
    peer_state.in_flight.insert(
        block_hash,
        super::super::CompactDownloadInFlight {
            partial,
            getblocktxn_in_flight: false,
            requested_indexes: Vec::new(),
            started_at_unix: 0,
        },
    );

    assert_eq!(
        handle_block_transactions(
            activated_policy(),
            true,
            &mut peer_state,
            &BlockTransactions {
                block_hash,
                transactions: vec![missing_tx],
            },
        ),
        CompactBlockTxnHandleOutcome::Misbehavior(CompactBlockTxnMisbehavior::DuplicateResponse)
    );
}

#[test]
fn handle_block_transactions_reschedules_or_falls_back_for_remaining_missing() {
    let header = sample_header();
    let coinbase = coinbase_transaction();
    let missing_a = sample_transaction(0x31);
    let missing_b = sample_transaction(0x32);
    let selector = open_bitcoin_codec::short_id_selector_from_header_and_nonce(&header, 7);
    let short_id_a = open_bitcoin_consensus::compact_short_id_for_wtxid(
        selector,
        &transaction_wtxid(&missing_a).expect("wtxid"),
    );
    let short_id_b = open_bitcoin_consensus::compact_short_id_for_wtxid(
        selector,
        &transaction_wtxid(&missing_b).expect("wtxid"),
    );
    let payload = CompactBlockPayload {
        header: header.clone(),
        nonce: 7,
        short_ids: vec![short_id_a, short_id_b],
        prefilled_transactions: vec![PrefilledTransaction {
            index_delta: 0,
            transaction: coinbase,
        }],
    };
    let block_hash = block_hash(&header);
    let mut peer_state = CompactDownloadPeerState::new();
    let mut partial = PartialCompactBlock::new();
    let _ = init_partial_compact_block(
        &mut partial,
        &payload,
        std::iter::empty(),
        std::iter::empty::<(&Wtxid, &Transaction)>(),
    );
    peer_state.in_flight.insert(
        block_hash,
        super::super::CompactDownloadInFlight {
            partial,
            getblocktxn_in_flight: true,
            requested_indexes: vec![1, 2],
            started_at_unix: 0,
        },
    );

    let partial_response = handle_block_transactions(
        activated_policy(),
        true,
        &mut peer_state,
        &BlockTransactions {
            block_hash,
            transactions: vec![missing_a.clone()],
        },
    );
    let CompactBlockTxnHandleOutcome::Progress { actions } = partial_response else {
        panic!("expected progress outcome");
    };
    assert!(matches!(
        actions[0],
        CompactDownloadAction::SendGetBlockTxn(_)
    ));
    assert!(peer_state.in_flight.contains_key(&block_hash));

    peer_state.in_flight.insert(
        block_hash,
        super::super::CompactDownloadInFlight {
            partial: {
                let mut partial = PartialCompactBlock::new();
                let _ = init_partial_compact_block(
                    &mut partial,
                    &payload,
                    std::iter::empty(),
                    std::iter::empty(),
                );
                partial
            },
            getblocktxn_in_flight: true,
            requested_indexes: vec![1, 2],
            started_at_unix: 0,
        },
    );
    let fallback = handle_block_transactions(
        BlockRelayActivationPolicy::default(),
        true,
        &mut peer_state,
        &BlockTransactions {
            block_hash,
            transactions: vec![missing_a],
        },
    );
    let CompactBlockTxnHandleOutcome::Progress { actions } = fallback else {
        panic!("expected progress outcome");
    };
    assert!(matches!(
        actions[0],
        CompactDownloadAction::RequestFullBlock(FullBlockFetch { block_hash: hash }) if hash == block_hash
    ));
    assert!(!peer_state.in_flight.contains_key(&block_hash));
}

#[test]
fn handle_block_transactions_surfaces_misbehavior_and_unexpected_hash() {
    let (payload, missing_tx, _) = compact_payload_with_missing_short_id();
    let block_hash = block_hash(&payload.header);
    let mut peer_state = CompactDownloadPeerState::new();
    let mut partial = PartialCompactBlock::new();
    let _ = init_partial_compact_block(
        &mut partial,
        &payload,
        std::iter::empty(),
        std::iter::empty::<(&Wtxid, &Transaction)>(),
    );
    peer_state.in_flight.insert(
        block_hash,
        super::super::CompactDownloadInFlight {
            partial,
            getblocktxn_in_flight: true,
            requested_indexes: vec![1],
            started_at_unix: 0,
        },
    );

    assert_eq!(
        handle_block_transactions(
            activated_policy(),
            true,
            &mut peer_state,
            &BlockTransactions {
                block_hash: BlockHash::from_byte_array([0xee; 32]),
                transactions: vec![missing_tx.clone()],
            },
        ),
        CompactBlockTxnHandleOutcome::NoMatchingInFlight
    );

    let mut partial = PartialCompactBlock::new();
    let _ = init_partial_compact_block(
        &mut partial,
        &payload,
        std::iter::empty(),
        std::iter::empty(),
    );
    peer_state.in_flight.insert(
        block_hash,
        super::super::CompactDownloadInFlight {
            partial,
            getblocktxn_in_flight: true,
            requested_indexes: vec![1],
            started_at_unix: 0,
        },
    );

    let null_transaction = Transaction {
        version: 2,
        inputs: Vec::new(),
        outputs: vec![TransactionOutput {
            value: Amount::from_sats(1).expect("valid amount"),
            script_pubkey: ScriptBuf::from_bytes(vec![0x51]).expect("valid script"),
        }],
        lock_time: 0,
    };
    assert_eq!(
        handle_block_transactions(
            activated_policy(),
            true,
            &mut peer_state,
            &BlockTransactions {
                block_hash,
                transactions: vec![null_transaction],
            },
        ),
        CompactBlockTxnHandleOutcome::Misbehavior(CompactBlockTxnMisbehavior::OutOfBoundsIndex)
    );
    assert!(!peer_state.in_flight.contains_key(&block_hash));
}

#[test]
fn handle_block_transactions_detects_partial_block_hash_mismatch() {
    let (payload, missing_tx, _) = compact_payload_with_missing_short_id();
    let partial_hash = block_hash(&payload.header);
    let lookup_hash = BlockHash::from_byte_array([0xde; 32]);
    let mut peer_state = CompactDownloadPeerState::new();
    let mut partial = PartialCompactBlock::new();
    let _ = init_partial_compact_block(
        &mut partial,
        &payload,
        std::iter::empty(),
        std::iter::empty::<(&Wtxid, &Transaction)>(),
    );
    peer_state.in_flight.insert(
        lookup_hash,
        super::super::CompactDownloadInFlight {
            partial,
            getblocktxn_in_flight: true,
            requested_indexes: vec![1],
            started_at_unix: 0,
        },
    );

    assert_eq!(
        handle_block_transactions(
            activated_policy(),
            true,
            &mut peer_state,
            &BlockTransactions {
                block_hash: lookup_hash,
                transactions: vec![missing_tx],
            },
        ),
        CompactBlockTxnHandleOutcome::UnexpectedBlockHash
    );
    assert!(!peer_state.in_flight.contains_key(&lookup_hash));
    assert_ne!(lookup_hash, partial_hash);
}
