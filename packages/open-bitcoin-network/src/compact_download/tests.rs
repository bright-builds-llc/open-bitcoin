// Parity breadcrumbs:
// - packages/bitcoin-knots/src/blockencodings.h
// - packages/bitcoin-knots/src/blockencodings.cpp
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/net_processing.h
// - packages/bitcoin-knots/test/functional/p2p_compactblocks.py

use open_bitcoin_codec::{
    BlockTransactions, CompactBlockPayload, PrefilledTransaction, expand_block_transaction_indexes,
    short_id_from_masked_u64,
};
use open_bitcoin_consensus::{block_hash, transaction_wtxid};
use open_bitcoin_primitives::{
    Amount, BlockHash, BlockHeader, MerkleRoot, OutPoint, ScriptBuf, ScriptWitness, Transaction,
    TransactionInput, TransactionOutput, Txid, Wtxid,
};

use crate::block_serving::{BlockRelayActivationPolicy, CompactRelayActivationConfig};
use crate::compact_reconstruction::{
    CompactBlockTxnMisbehavior, CompactBlockTxnOutcome, CompactReconstructionOutcome,
    PartialCompactBlock, apply_block_transactions, fill_block, init_partial_compact_block,
};

use super::{
    COMPACT_BLOCK_DOWNLOAD_TIMEOUT_SECONDS, CompactBlockDownloadEligibility,
    CompactBlockDownloadEligibilityInput, CompactBlockInitOutcome, CompactBlockTxnHandleOutcome,
    CompactDownloadAction, CompactDownloadCleanupCause, CompactDownloadCompletionOutcome,
    CompactDownloadPeerState, CompactDownloadSuppressionReason, FullBlockFetch,
    ScheduleMissingTransactionOutcome, ScheduleMissingTransactionSuppressionReason,
    build_get_block_transactions_request, cleanup_compact_download_on_block_connected,
    cleanup_compact_download_peer, compact_download_actions_to_peer_actions,
    complete_applied_download, completed_or_fallback_init, completion_actions_from_outcome,
    evaluate_compact_block_download_eligibility, expire_stale_compact_downloads,
    finalize_ready_init, finish_applied_handle, handle_block_transactions,
    init_compact_block_download, peer_supports_compact_download,
    schedule_missing_transaction_request, scheduled_init_request, take_in_flight_download,
    try_complete_compact_download,
};

fn eligible_input(header: &BlockHeader) -> CompactBlockDownloadEligibilityInput {
    CompactBlockDownloadEligibilityInput {
        local_best_height: 0,
        best_tip_hash: header.previous_block_hash,
        maybe_known_block_height: Some(1),
    }
}

fn activated_policy() -> BlockRelayActivationPolicy {
    BlockRelayActivationPolicy {
        compact_relay: CompactRelayActivationConfig { enabled: true },
        ..BlockRelayActivationPolicy::default()
    }
}

fn sample_header() -> BlockHeader {
    BlockHeader {
        version: 2,
        previous_block_hash: BlockHash::from_byte_array([1_u8; 32]),
        merkle_root: MerkleRoot::from_byte_array([2_u8; 32]),
        time: 1_234,
        bits: 0x207f_ffff,
        nonce: 99,
    }
}

fn coinbase_transaction() -> Transaction {
    Transaction {
        version: 2,
        inputs: vec![TransactionInput {
            previous_output: OutPoint::null(),
            script_sig: ScriptBuf::from_bytes(vec![0x01, 0x02]).expect("valid script"),
            sequence: TransactionInput::SEQUENCE_FINAL,
            witness: ScriptWitness::default(),
        }],
        outputs: vec![TransactionOutput {
            value: Amount::from_sats(50_000_000_000).expect("valid amount"),
            script_pubkey: ScriptBuf::from_bytes(vec![0x51]).expect("valid script"),
        }],
        lock_time: 0,
    }
}

fn sample_transaction(previous_txid_byte: u8) -> Transaction {
    Transaction {
        version: 2,
        inputs: vec![TransactionInput {
            previous_output: OutPoint {
                txid: Txid::from_byte_array([previous_txid_byte; 32]),
                vout: 0,
            },
            script_sig: ScriptBuf::from_bytes(Vec::new()).expect("valid script"),
            sequence: TransactionInput::SEQUENCE_FINAL,
            witness: ScriptWitness::new(vec![vec![0x01, 0x02]]),
        }],
        outputs: vec![TransactionOutput {
            value: Amount::from_sats(5_000).expect("valid amount"),
            script_pubkey: ScriptBuf::from_bytes(vec![0x51, 0xac]).expect("valid script"),
        }],
        lock_time: 0,
    }
}

fn compact_payload_with_missing_short_id() -> (CompactBlockPayload, Transaction, Wtxid) {
    let header = sample_header();
    let coinbase = coinbase_transaction();
    let missing = sample_transaction(0x22);
    let wtxid = transaction_wtxid(&missing).expect("wtxid");
    let selector = open_bitcoin_codec::short_id_selector_from_header_and_nonce(&header, 42);
    let short_id = open_bitcoin_consensus::compact_short_id_for_wtxid(selector, &wtxid);

    let payload = CompactBlockPayload {
        header,
        nonce: 42,
        short_ids: vec![short_id],
        prefilled_transactions: vec![PrefilledTransaction {
            index_delta: 0,
            transaction: coinbase,
        }],
    };

    (payload, missing, wtxid)
}

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
        super::CompactDownloadInFlight {
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
        super::CompactDownloadInFlight {
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
        super::CompactDownloadAction::ReceivedBlock(_)
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
fn cleanup_matrix_clears_in_flight_state_without_touching_unrelated_blocks() {
    let hash_a = BlockHash::from_byte_array([10_u8; 32]);
    let hash_b = BlockHash::from_byte_array([11_u8; 32]);
    let mut peer_state = CompactDownloadPeerState::new();
    peer_state.in_flight.insert(
        hash_a,
        super::CompactDownloadInFlight {
            partial: PartialCompactBlock::new(),
            getblocktxn_in_flight: true,
            requested_indexes: vec![1],
            started_at_unix: 0,
        },
    );
    peer_state.in_flight.insert(
        hash_b,
        super::CompactDownloadInFlight {
            partial: PartialCompactBlock::new(),
            getblocktxn_in_flight: false,
            requested_indexes: Vec::new(),
            started_at_unix: 0,
        },
    );

    assert!(cleanup_compact_download_on_block_connected(
        &mut peer_state,
        hash_a
    ));
    assert!(!peer_state.in_flight.contains_key(&hash_a));
    assert!(peer_state.in_flight.contains_key(&hash_b));

    assert_eq!(
        cleanup_compact_download_peer(&mut peer_state, CompactDownloadCleanupCause::Timeout),
        1
    );
    assert!(peer_state.in_flight.is_empty());

    for cause in [
        CompactDownloadCleanupCause::PeerDisconnect,
        CompactDownloadCleanupCause::Reorg,
        CompactDownloadCleanupCause::RuntimeRestart,
        CompactDownloadCleanupCause::BlockConnected,
    ] {
        peer_state.in_flight.insert(
            hash_a,
            super::CompactDownloadInFlight {
                partial: PartialCompactBlock::new(),
                getblocktxn_in_flight: false,
                requested_indexes: Vec::new(),
                started_at_unix: 0,
            },
        );
        assert_eq!(cleanup_compact_download_peer(&mut peer_state, cause), 1);
    }
}

#[test]
fn duplicate_in_flight_getblocktxn_request_is_suppressed() {
    let (payload, _, _) = compact_payload_with_missing_short_id();
    let block_hash = block_hash(&payload.header);
    let mut peer_state = CompactDownloadPeerState::new();
    let _ = init_compact_block_download(
        activated_policy(),
        true,
        &mut peer_state,
        &payload,
        eligible_input(&payload.header),
        1_000,
        std::iter::empty::<(&Wtxid, &Transaction)>(),
        std::iter::empty(),
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
        activated_policy(),
        true,
        &mut peer_state,
        &payload,
        eligibility,
        1_000,
        std::iter::empty::<(&Wtxid, &Transaction)>(),
        std::iter::empty(),
    );

    assert!(matches!(
        outcome,
        CompactBlockInitOutcome::Fallback(fetch) if fetch.block_hash == block_hash
    ));
    assert!(peer_state.in_flight.is_empty());
}

#[test]
fn expire_stale_compact_downloads_removes_timed_out_in_flight_state() {
    let block_hash = BlockHash::from_byte_array([7_u8; 32]);
    let mut peer_state = CompactDownloadPeerState::new();
    peer_state.in_flight.insert(
        block_hash,
        super::CompactDownloadInFlight {
            partial: PartialCompactBlock::new(),
            getblocktxn_in_flight: true,
            requested_indexes: vec![1],
            started_at_unix: 100,
        },
    );

    let expired = expire_stale_compact_downloads(
        &mut peer_state,
        100 + COMPACT_BLOCK_DOWNLOAD_TIMEOUT_SECONDS + 1,
        COMPACT_BLOCK_DOWNLOAD_TIMEOUT_SECONDS,
    );

    assert_eq!(expired.len(), 1);
    assert_eq!(expired[0].block_hash, block_hash);
    assert!(peer_state.in_flight.is_empty());
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
fn compact_download_cleanup_and_suppression_labels_are_stable() {
    assert_eq!(
        CompactDownloadCleanupCause::PeerDisconnect.as_str(),
        "compact_download_peer_disconnect"
    );
    assert_eq!(
        CompactDownloadCleanupCause::Timeout.as_str(),
        "compact_download_timeout"
    );
    assert_eq!(
        CompactDownloadCleanupCause::Reorg.as_str(),
        "compact_download_reorg"
    );
    assert_eq!(
        CompactDownloadCleanupCause::RuntimeRestart.as_str(),
        "compact_download_restart"
    );
    assert_eq!(
        CompactDownloadCleanupCause::BlockConnected.as_str(),
        "compact_download_block_connected"
    );
    assert_eq!(
        CompactDownloadSuppressionReason::CompactReconstructionFailed.as_str(),
        "compact_reconstruction_failed"
    );
    assert_eq!(
        CompactDownloadSuppressionReason::CompactDownloadTimeout.as_str(),
        "compact_download_timeout"
    );
    assert_eq!(
        CompactDownloadSuppressionReason::CompactPeerIneligible.as_str(),
        "compact_peer_ineligible"
    );
    assert_eq!(
        CompactDownloadSuppressionReason::CompactReconstructionInvalid.as_str(),
        "compact_reconstruction_invalid"
    );
    assert_eq!(
        CompactDownloadSuppressionReason::CompactBlockAlreadyInFlight.as_str(),
        "compact_block_already_in_flight"
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
fn expire_stale_compact_downloads_keeps_fresh_in_flight_state() {
    let block_hash = BlockHash::from_byte_array([8_u8; 32]);
    let mut peer_state = CompactDownloadPeerState::new();
    peer_state.in_flight.insert(
        block_hash,
        super::CompactDownloadInFlight {
            partial: PartialCompactBlock::new(),
            getblocktxn_in_flight: false,
            requested_indexes: Vec::new(),
            started_at_unix: 1_000,
        },
    );

    let expired = expire_stale_compact_downloads(
        &mut peer_state,
        1_000 + COMPACT_BLOCK_DOWNLOAD_TIMEOUT_SECONDS,
        COMPACT_BLOCK_DOWNLOAD_TIMEOUT_SECONDS,
    );

    assert!(expired.is_empty());
    assert!(peer_state.in_flight.contains_key(&block_hash));
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
        super::CompactDownloadInFlight {
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
    let in_flight = super::CompactDownloadInFlight {
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

    let in_flight = super::CompactDownloadInFlight {
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
fn compact_download_actions_to_peer_actions_maps_all_variants() {
    let block_hash = BlockHash::from_byte_array([6_u8; 32]);
    let request = build_get_block_transactions_request(block_hash, &[1_u16]);
    let header = sample_header();
    let block = open_bitcoin_primitives::Block {
        header,
        transactions: vec![coinbase_transaction()],
    };

    let actions = compact_download_actions_to_peer_actions(vec![
        CompactDownloadAction::SendGetBlockTxn(request.clone()),
        CompactDownloadAction::RequestFullBlock(FullBlockFetch { block_hash }),
        CompactDownloadAction::ReceivedBlock(block.clone()),
    ]);

    assert_eq!(actions.len(), 3);
    assert!(matches!(
        &actions[0],
        crate::PeerAction::Send(crate::WireNetworkMessage::GetBlockTxn(_))
    ));
    assert!(matches!(
        &actions[1],
        crate::PeerAction::Send(crate::WireNetworkMessage::GetData(_))
    ));
    assert!(matches!(
        &actions[2],
        crate::PeerAction::ReceivedBlock(received) if received == &block
    ));
}

#[test]
fn init_compact_block_download_suppressed_when_ineligible_or_duplicate() {
    let (payload, _, _) = compact_payload_with_missing_short_id();
    let block_hash = block_hash(&payload.header);
    let mut peer_state = CompactDownloadPeerState::new();

    let disabled_outcome = init_compact_block_download(
        BlockRelayActivationPolicy::default(),
        true,
        &mut peer_state,
        &payload,
        eligible_input(&payload.header),
        1_000,
        std::iter::empty::<(&Wtxid, &Transaction)>(),
        std::iter::empty(),
    );
    assert!(matches!(
        disabled_outcome,
        CompactBlockInitOutcome::Suppressed(
            CompactDownloadSuppressionReason::CompactPeerIneligible
        )
    ));

    let peer_ineligible = init_compact_block_download(
        activated_policy(),
        false,
        &mut peer_state,
        &payload,
        eligible_input(&payload.header),
        1_000,
        std::iter::empty::<(&Wtxid, &Transaction)>(),
        std::iter::empty(),
    );
    assert!(matches!(
        peer_ineligible,
        CompactBlockInitOutcome::Suppressed(
            CompactDownloadSuppressionReason::CompactPeerIneligible
        )
    ));

    let _ = init_compact_block_download(
        activated_policy(),
        true,
        &mut peer_state,
        &payload,
        eligible_input(&payload.header),
        1_000,
        std::iter::empty::<(&Wtxid, &Transaction)>(),
        std::iter::empty(),
    );
    let duplicate = init_compact_block_download(
        activated_policy(),
        true,
        &mut peer_state,
        &payload,
        eligible_input(&payload.header),
        1_001,
        std::iter::empty::<(&Wtxid, &Transaction)>(),
        std::iter::empty(),
    );
    assert!(matches!(
        duplicate,
        CompactBlockInitOutcome::Suppressed(
            CompactDownloadSuppressionReason::CompactBlockAlreadyInFlight
        )
    ));
    assert!(peer_state.in_flight.contains_key(&block_hash));
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
fn init_compact_block_download_falls_back_on_invalid_or_failed_reconstruction() {
    let header = sample_header();
    let expected_block_hash = block_hash(&header);
    let mut peer_state = CompactDownloadPeerState::new();

    let empty_payload = CompactBlockPayload {
        header: header.clone(),
        nonce: 1,
        short_ids: Vec::new(),
        prefilled_transactions: Vec::new(),
    };
    let invalid = init_compact_block_download(
        activated_policy(),
        true,
        &mut peer_state,
        &empty_payload,
        eligible_input(&header),
        1_000,
        std::iter::empty::<(&Wtxid, &Transaction)>(),
        std::iter::empty(),
    );
    assert!(matches!(
        invalid,
        CompactBlockInitOutcome::Fallback(fetch) if fetch.block_hash == expected_block_hash
    ));

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
        activated_policy(),
        true,
        &mut peer_state,
        &collision_payload,
        eligible_input(&collision_payload.header),
        1_000,
        std::iter::empty::<(&Wtxid, &Transaction)>(),
        std::iter::empty(),
    );
    assert!(matches!(
        failed,
        CompactBlockInitOutcome::Fallback(fetch) if fetch.block_hash == expected_block_hash
    ));
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
        super::CompactDownloadInFlight {
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
        super::CompactDownloadInFlight {
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
        super::CompactDownloadInFlight {
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
        super::CompactDownloadInFlight {
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
        super::CompactDownloadInFlight {
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
fn completed_or_fallback_init_falls_back_for_uninitialized_partial() {
    let block_hash = BlockHash::from_byte_array([0xcd; 32]);

    assert!(matches!(
        completed_or_fallback_init(block_hash, &PartialCompactBlock::new()),
        CompactBlockInitOutcome::Fallback(FullBlockFetch { block_hash: hash }) if hash == block_hash
    ));
}

#[test]
fn scheduled_init_request_removes_in_flight_when_schedule_is_suppressed() {
    let block_hash = BlockHash::from_byte_array([0xce; 32]);
    let mut peer_state = CompactDownloadPeerState::new();
    peer_state.in_flight.insert(
        block_hash,
        super::CompactDownloadInFlight {
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
        super::CompactDownloadInFlight {
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
fn finalize_ready_init_returns_fallback_when_schedule_is_suppressed() {
    let block_hash = BlockHash::from_byte_array([0xd0; 32]);
    let mut peer_state = CompactDownloadPeerState::new();
    peer_state.in_flight.insert(
        block_hash,
        super::CompactDownloadInFlight {
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
