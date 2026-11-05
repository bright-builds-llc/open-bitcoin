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
    CompactBlockTxnMisbehavior, CompactBlockTxnOutcome, CompactReconstructionInvalidReason,
    CompactReconstructionOutcome, PartialCompactBlock, apply_block_transactions, fill_block,
    init_partial_compact_block,
};

use super::{
    COMPACT_BLOCK_DOWNLOAD_TIMEOUT_SECONDS, CompactBlockDownloadEligibility,
    CompactBlockDownloadEligibilityInput, CompactBlockInitOutcome, CompactBlockTxnHandleOutcome,
    CompactDownloadAction, CompactDownloadCleanupCause, CompactDownloadCompletionOutcome,
    CompactDownloadPeerState, CompactDownloadSuppressionReason, FullBlockFetch,
    ScheduleMissingTransactionOutcome, ScheduleMissingTransactionSuppressionReason,
    build_get_block_transactions_request, cleanup_compact_download_on_block_connected,
    cleanup_compact_download_peer, compact_download_actions_to_peer_actions,
    complete_applied_download_selected, completed_or_fallback_init_selected,
    completion_actions_from_outcome, evaluate_compact_block_download_eligibility,
    expire_stale_compact_downloads, finalize_ready_init, finish_applied_handle_selected,
    handle_block_transactions_selected, init_compact_block_download,
    init_compact_block_download_selected, peer_supports_compact_download,
    schedule_missing_transaction_request, scheduled_init_request, take_in_flight_download,
    try_complete_compact_download_selected,
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
    let missing = sample_transaction(0x22);
    let body = crate::test_block_commitments::coherent_body(
        sample_header(),
        vec![coinbase_transaction(), missing.clone()],
        false,
    );
    let header = body.header;
    let coinbase = body.transactions[0].clone();
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

mod blocktxn_cases;
mod completion_cases;
mod initialization_cases;
mod lifecycle_cases;
mod scheduling_cases;

#[test]
fn phase167_compact_fixture_complete_missing_body_is_coherent() {
    // Arrange
    let (payload, missing, wtxid) = compact_payload_with_missing_short_id();
    let mut partial = PartialCompactBlock::new();
    let _ = init_partial_compact_block(
        &mut partial,
        &payload,
        std::iter::empty::<(&Wtxid, &Transaction)>(),
        [(&wtxid, &missing)],
    );
    let reconstructed = fill_block(&partial).expect("complete committed fixture body");
    let body = open_bitcoin_primitives::Block {
        header: payload.header,
        transactions: vec![
            payload.prefilled_transactions[0].transaction.clone(),
            missing,
        ],
    };
    // Act / Assert
    assert_eq!(reconstructed, body);
    crate::test_block_commitments::assert_coherent(&body);
    assert!(body.transactions[1].has_witness());
}

#[test]
fn phase167_compact_fixture_commitment_controls_reject_each_incoherence() {
    // Arrange
    let (payload, missing, _) = compact_payload_with_missing_short_id();
    let original = open_bitcoin_primitives::Block {
        header: payload.header,
        transactions: vec![
            payload.prefilled_transactions[0].transaction.clone(),
            missing,
        ],
    };
    for case in 0..6 {
        let mut body = original.clone();
        match case {
            0 => body.transactions[0].inputs[0].witness = ScriptWitness::default(),
            1 => body.transactions[0].inputs[0].witness = ScriptWitness::new(vec![vec![1; 32]]),
            2 => {
                body.transactions[0].outputs.pop();
            }
            3 => {
                body.transactions.pop();
            }
            4 => body.transactions[1].lock_time += 1,
            _ => body.header.merkle_root = Default::default(),
        }
        // Act
        let mutated = open_bitcoin_consensus::block::is_block_mutated(&body, true);
        // Assert
        assert_eq!(mutated, Ok(true), "coherence control {case}");
    }
}
