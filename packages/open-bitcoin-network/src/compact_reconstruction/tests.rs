// Parity breadcrumbs:
// - packages/bitcoin-knots/src/blockencodings.cpp
// - packages/bitcoin-knots/src/blockencodings.h
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/test/functional/p2p_compactblocks.py

use open_bitcoin_codec::{
    CompactBlockPayload, PrefilledTransaction, ShortId, short_id_from_masked_u64,
    short_id_match_key, short_id_selector_from_header_and_nonce,
};
use open_bitcoin_consensus::{compact_short_id_for_wtxid, transaction_wtxid};
use open_bitcoin_primitives::{
    Amount, BlockHash, BlockHeader, MerkleRoot, OutPoint, ScriptBuf, ScriptWitness, Transaction,
    TransactionInput, TransactionOutput, Txid, Wtxid,
};

use super::{
    CompactBlockTxnMisbehavior, CompactBlockTxnOutcome, CompactReconstructionFailureReason,
    CompactReconstructionInvalidReason, CompactReconstructionOutcome,
    MAX_COMPACT_BLOCK_TRANSACTION_COUNT, PartialCompactBlock, apply_block_transactions, fill_block,
    init_partial_compact_block,
};

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

fn sample_transaction(previous_txid_byte: u8, vout: u32, output_value: i64) -> Transaction {
    Transaction {
        version: 2,
        inputs: vec![TransactionInput {
            previous_output: OutPoint {
                txid: Txid::from_byte_array([previous_txid_byte; 32]),
                vout,
            },
            script_sig: ScriptBuf::from_bytes(Vec::new()).expect("valid script"),
            sequence: TransactionInput::SEQUENCE_FINAL,
            witness: ScriptWitness::new(vec![vec![0x01, 0x02]]),
        }],
        outputs: vec![TransactionOutput {
            value: Amount::from_sats(output_value).expect("valid amount"),
            script_pubkey: ScriptBuf::from_bytes(vec![0x51, 0xac]).expect("valid script"),
        }],
        lock_time: 0,
    }
}

fn transaction_pair(transaction: &Transaction) -> (Wtxid, Transaction) {
    let wtxid = transaction_wtxid(transaction).expect("wtxid");
    (wtxid, transaction.clone())
}

fn compact_payload_with_short_ids(
    header: BlockHeader,
    nonce: u64,
    prefilled: Vec<PrefilledTransaction>,
    transactions_for_short_ids: &[Transaction],
) -> CompactBlockPayload {
    let selector = short_id_selector_from_header_and_nonce(&header, nonce);
    let short_ids = transactions_for_short_ids
        .iter()
        .map(|transaction| {
            let wtxid = transaction_wtxid(transaction).expect("wtxid");
            compact_short_id_for_wtxid(selector, &wtxid)
        })
        .collect();

    CompactBlockPayload {
        header,
        nonce,
        short_ids,
        prefilled_transactions: prefilled,
    }
}

#[test]
fn happy_path_reconstructs_prefilled_and_mempool_matches() {
    let header = sample_header();
    let coinbase = coinbase_transaction();
    let matched = sample_transaction(0x11, 0, 5_000);
    let missing = sample_transaction(0x22, 1, 7_000);
    let short_id_transactions = [matched, missing];
    let (matched_wtxid, matched_tx) = transaction_pair(&short_id_transactions[0]);

    let payload = compact_payload_with_short_ids(
        header.clone(),
        42,
        vec![PrefilledTransaction {
            index_delta: 0,
            transaction: coinbase.clone(),
        }],
        &short_id_transactions,
    );

    let mut state = PartialCompactBlock::new();
    let outcome = init_partial_compact_block(
        &mut state,
        &payload,
        [(&matched_wtxid, &matched_tx)],
        std::iter::empty::<(&Wtxid, &Transaction)>(),
    );

    assert_eq!(
        outcome,
        CompactReconstructionOutcome::Ready {
            missing_indexes: vec![2],
        }
    );
    assert!(state.is_transaction_available(0));
    assert!(state.is_transaction_available(1));
    assert!(!state.is_transaction_available(2));
    assert_eq!(state.header(), Some(&header));
}

#[test]
fn missing_transactions_reports_all_unfilled_short_id_slots() {
    let header = sample_header();
    let coinbase = coinbase_transaction();
    let missing_a = sample_transaction(0x31, 0, 1_000);
    let missing_b = sample_transaction(0x32, 0, 2_000);

    let payload = compact_payload_with_short_ids(
        header,
        7,
        vec![PrefilledTransaction {
            index_delta: 0,
            transaction: coinbase,
        }],
        &[missing_a, missing_b],
    );

    let mut state = PartialCompactBlock::new();
    let outcome = init_partial_compact_block(
        &mut state,
        &payload,
        std::iter::empty::<(&Wtxid, &Transaction)>(),
        std::iter::empty::<(&Wtxid, &Transaction)>(),
    );

    assert_eq!(
        outcome,
        CompactReconstructionOutcome::Ready {
            missing_indexes: vec![1, 2],
        }
    );
}

#[test]
fn short_id_collision_fails_initialization() {
    let header = sample_header();
    let coinbase = coinbase_transaction();
    let colliding_short_id = short_id_from_masked_u64(0x00aa_bbcc_dd11);

    let payload = CompactBlockPayload {
        header,
        nonce: 1,
        short_ids: vec![colliding_short_id, colliding_short_id],
        prefilled_transactions: vec![PrefilledTransaction {
            index_delta: 0,
            transaction: coinbase,
        }],
    };

    let mut state = PartialCompactBlock::new();
    let outcome = init_partial_compact_block(
        &mut state,
        &payload,
        std::iter::empty::<(&Wtxid, &Transaction)>(),
        std::iter::empty::<(&Wtxid, &Transaction)>(),
    );

    assert_eq!(
        outcome,
        CompactReconstructionOutcome::Failed(CompactReconstructionFailureReason::ShortIdCollision)
    );
    assert!(!state.is_initialized());
}

#[test]
fn duplicate_mempool_match_clears_slot() {
    let header = sample_header();
    let coinbase = coinbase_transaction();
    let matched = sample_transaction(0x41, 0, 3_000);
    let (matched_wtxid, matched_tx) = transaction_pair(&matched);

    let payload = compact_payload_with_short_ids(
        header,
        99,
        vec![PrefilledTransaction {
            index_delta: 0,
            transaction: coinbase.clone(),
        }],
        std::slice::from_ref(&matched),
    );

    let selector = short_id_selector_from_header_and_nonce(&payload.header, payload.nonce);
    let slot_one_short_id = compact_short_id_for_wtxid(selector, &matched_wtxid);
    let mut short_id_map = std::collections::HashMap::new();
    short_id_map.insert(short_id_match_key(slot_one_short_id), 1_u16);

    let mut state = PartialCompactBlock::new();
    state.header = Some(payload.header);
    state.txn_available = vec![Some(coinbase), None];
    state.slot_wtxids = vec![None, None];
    state.prefilled_count = 1;
    state.short_id_slots_remaining = 2;

    let mut matched_short_ids = 0_usize;
    super::scan_candidate_transactions(
        &mut state,
        &selector,
        &short_id_map,
        [(&matched_wtxid, &matched_tx), (&matched_wtxid, &matched_tx)],
        &mut matched_short_ids,
        false,
    );

    assert!(!state.is_transaction_available(1));
}

#[test]
fn prefilled_only_compact_block_initializes() {
    let header = sample_header();
    let coinbase = coinbase_transaction();
    let payload = CompactBlockPayload {
        header: header.clone(),
        nonce: 5,
        short_ids: Vec::new(),
        prefilled_transactions: vec![PrefilledTransaction {
            index_delta: 0,
            transaction: coinbase,
        }],
    };

    let mut state = PartialCompactBlock::new();
    let outcome = init_partial_compact_block(
        &mut state,
        &payload,
        std::iter::empty::<(&Wtxid, &Transaction)>(),
        std::iter::empty::<(&Wtxid, &Transaction)>(),
    );

    assert_eq!(
        outcome,
        CompactReconstructionOutcome::Ready {
            missing_indexes: Vec::new(),
        }
    );
    assert!(state.is_transaction_available(0));
    assert_eq!(state.transaction_count(), 1);
}

#[test]
fn invalid_prefilled_index_is_rejected() {
    let header = sample_header();
    let coinbase = coinbase_transaction();
    let payload = CompactBlockPayload {
        header,
        nonce: 3,
        short_ids: Vec::new(),
        prefilled_transactions: vec![
            PrefilledTransaction {
                index_delta: 0,
                transaction: coinbase.clone(),
            },
            PrefilledTransaction {
                index_delta: 5,
                transaction: coinbase,
            },
        ],
    };

    let mut state = PartialCompactBlock::new();
    let outcome = init_partial_compact_block(
        &mut state,
        &payload,
        std::iter::empty::<(&Wtxid, &Transaction)>(),
        std::iter::empty::<(&Wtxid, &Transaction)>(),
    );

    assert_eq!(
        outcome,
        CompactReconstructionOutcome::Invalid(
            CompactReconstructionInvalidReason::PrefilledIndexOutOfBounds
        )
    );
}

#[test]
fn bucket_overload_fails_initialization() {
    let header = sample_header();
    let coinbase = coinbase_transaction();
    let bucket_count = 13_u64;
    let short_ids = (0..bucket_count)
        .map(|index| short_id_from_masked_u64(index * bucket_count))
        .collect::<Vec<_>>();

    let mut bucket_sizes = std::collections::HashMap::new();
    for short_id in &short_ids {
        let bucket = short_id_match_key(*short_id) % bucket_count;
        let entry = bucket_sizes.entry(bucket).or_insert(0_u16);
        *entry = entry.saturating_add(1);
    }
    assert!(
        bucket_sizes.values().any(|count| *count > 12),
        "fixture must overload a bucket"
    );

    let payload = CompactBlockPayload {
        header,
        nonce: 11,
        short_ids,
        prefilled_transactions: vec![PrefilledTransaction {
            index_delta: 0,
            transaction: coinbase,
        }],
    };

    let mut state = PartialCompactBlock::new();
    let outcome = init_partial_compact_block(
        &mut state,
        &payload,
        std::iter::empty::<(&Wtxid, &Transaction)>(),
        std::iter::empty::<(&Wtxid, &Transaction)>(),
    );

    assert_eq!(
        outcome,
        CompactReconstructionOutcome::Failed(
            CompactReconstructionFailureReason::ShortIdBucketOverload
        )
    );
}

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
fn mempool_removal_clears_matching_slot() {
    let header = sample_header();
    let coinbase = coinbase_transaction();
    let matched = sample_transaction(0x51, 0, 9_000);
    let (matched_wtxid, matched_tx) = transaction_pair(&matched);

    let payload = compact_payload_with_short_ids(
        header,
        12,
        vec![PrefilledTransaction {
            index_delta: 0,
            transaction: coinbase,
        }],
        std::slice::from_ref(&matched),
    );

    let mut state = PartialCompactBlock::new();
    let outcome = init_partial_compact_block(
        &mut state,
        &payload,
        [(&matched_wtxid, &matched_tx)],
        std::iter::empty::<(&Wtxid, &Transaction)>(),
    );
    assert!(state.is_transaction_available(1));
    assert!(matches!(
        outcome,
        CompactReconstructionOutcome::Ready { missing_indexes: _ }
    ));

    state.on_mempool_transaction_removed(&matched_wtxid);
    assert!(!state.is_transaction_available(1));
    assert_eq!(state.missing_transaction_indexes(), vec![1]);
}

#[test]
fn extra_transactions_can_fill_remaining_slots() {
    let header = sample_header();
    let coinbase = coinbase_transaction();
    let mempool_tx = sample_transaction(0x61, 0, 2_500);
    let extra_tx = sample_transaction(0x62, 0, 3_500);
    let short_id_transactions = [mempool_tx, extra_tx];
    let (mempool_wtxid, mempool_tx_clone) = transaction_pair(&short_id_transactions[0]);
    let (extra_wtxid, extra_tx_clone) = transaction_pair(&short_id_transactions[1]);

    let payload = compact_payload_with_short_ids(
        header,
        15,
        vec![PrefilledTransaction {
            index_delta: 0,
            transaction: coinbase,
        }],
        &short_id_transactions,
    );

    let mut state = PartialCompactBlock::new();
    let outcome = init_partial_compact_block(
        &mut state,
        &payload,
        [(&mempool_wtxid, &mempool_tx_clone)],
        [(&extra_wtxid, &extra_tx_clone)],
    );

    assert_eq!(
        outcome,
        CompactReconstructionOutcome::Ready {
            missing_indexes: Vec::new(),
        }
    );
    assert!(state.is_transaction_available(1));
    assert!(state.is_transaction_available(2));
}

#[test]
fn transaction_count_bound_matches_knots_limit() {
    assert_eq!(MAX_COMPACT_BLOCK_TRANSACTION_COUNT, 100_000);
}

#[test]
fn already_initialized_state_is_rejected() {
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
    let first = init_partial_compact_block(
        &mut state,
        &payload,
        std::iter::empty::<(&Wtxid, &Transaction)>(),
        std::iter::empty::<(&Wtxid, &Transaction)>(),
    );
    assert!(matches!(first, CompactReconstructionOutcome::Ready { .. }));

    let second = init_partial_compact_block(
        &mut state,
        &payload,
        std::iter::empty::<(&Wtxid, &Transaction)>(),
        std::iter::empty::<(&Wtxid, &Transaction)>(),
    );
    assert_eq!(
        second,
        CompactReconstructionOutcome::Invalid(
            CompactReconstructionInvalidReason::AlreadyInitialized
        )
    );
}

#[test]
fn partial_compact_block_default_is_uninitialized() {
    let state = PartialCompactBlock::default();

    assert!(!state.is_initialized());
    assert!(!state.is_transaction_available(0));
}

#[test]
fn null_header_is_rejected() {
    let header = BlockHeader {
        version: 0,
        previous_block_hash: BlockHash::from_byte_array([0_u8; 32]),
        merkle_root: MerkleRoot::from_byte_array([0_u8; 32]),
        time: 0,
        bits: 0,
        nonce: 0,
    };
    assert!(header.is_null());
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

    assert_eq!(
        outcome,
        CompactReconstructionOutcome::Invalid(CompactReconstructionInvalidReason::NullHeader)
    );
}

#[test]
fn empty_compact_block_is_rejected() {
    let payload = CompactBlockPayload {
        header: sample_header(),
        nonce: 1,
        short_ids: Vec::new(),
        prefilled_transactions: Vec::new(),
    };

    let mut state = PartialCompactBlock::new();
    let outcome = init_partial_compact_block(
        &mut state,
        &payload,
        std::iter::empty::<(&Wtxid, &Transaction)>(),
        std::iter::empty::<(&Wtxid, &Transaction)>(),
    );

    assert_eq!(
        outcome,
        CompactReconstructionOutcome::Invalid(
            CompactReconstructionInvalidReason::EmptyCompactBlock
        )
    );
}

#[test]
fn null_prefilled_transaction_is_rejected() {
    let header = sample_header();
    let null_transaction = Transaction {
        version: 2,
        inputs: Vec::new(),
        outputs: vec![TransactionOutput {
            value: Amount::from_sats(1).expect("valid amount"),
            script_pubkey: ScriptBuf::from_bytes(vec![0x51]).expect("valid script"),
        }],
        lock_time: 0,
    };

    let payload = CompactBlockPayload {
        header,
        nonce: 1,
        short_ids: Vec::new(),
        prefilled_transactions: vec![PrefilledTransaction {
            index_delta: 0,
            transaction: null_transaction,
        }],
    };

    let mut state = PartialCompactBlock::new();
    let outcome = init_partial_compact_block(
        &mut state,
        &payload,
        std::iter::empty::<(&Wtxid, &Transaction)>(),
        std::iter::empty::<(&Wtxid, &Transaction)>(),
    );

    assert_eq!(
        outcome,
        CompactReconstructionOutcome::Invalid(
            CompactReconstructionInvalidReason::NullPrefilledTransaction
        )
    );
}

#[test]
fn mempool_removal_on_uninitialized_state_is_noop() {
    let mut state = PartialCompactBlock::new();
    let wtxid = Wtxid::from_byte_array([0x77; 32]);

    state.on_mempool_transaction_removed(&wtxid);

    assert!(!state.is_initialized());
}

#[test]
fn extra_duplicate_with_same_witness_hash_does_not_clear_slot() {
    let header = sample_header();
    let coinbase = coinbase_transaction();
    let matched = sample_transaction(0x81, 0, 4_000);
    let (matched_wtxid, matched_tx) = transaction_pair(&matched);

    let payload = compact_payload_with_short_ids(
        header,
        21,
        vec![PrefilledTransaction {
            index_delta: 0,
            transaction: coinbase,
        }],
        std::slice::from_ref(&matched),
    );

    let selector = short_id_selector_from_header_and_nonce(&payload.header, payload.nonce);
    let slot_one_short_id = compact_short_id_for_wtxid(selector, &matched_wtxid);
    let mut short_id_map = std::collections::HashMap::new();
    short_id_map.insert(short_id_match_key(slot_one_short_id), 1_u16);

    let mut state = PartialCompactBlock::new();
    state.header = Some(payload.header);
    state.txn_available = vec![Some(coinbase_transaction()), None];
    state.slot_wtxids = vec![None, None];
    state.prefilled_count = 1;
    state.short_id_slots_remaining = 2;

    let mut matched_short_ids = 0_usize;
    super::scan_candidate_transactions(
        &mut state,
        &selector,
        &short_id_map,
        [(&matched_wtxid, &matched_tx), (&matched_wtxid, &matched_tx)],
        &mut matched_short_ids,
        true,
    );

    assert!(state.is_transaction_available(1));
}

#[test]
fn transaction_count_above_limit_is_rejected() {
    let header = sample_header();
    let coinbase = coinbase_transaction();
    let short_ids = vec![ShortId::from_wire_bytes([0_u8; 6]); MAX_COMPACT_BLOCK_TRANSACTION_COUNT];

    let payload = CompactBlockPayload {
        header,
        nonce: 1,
        short_ids,
        prefilled_transactions: vec![PrefilledTransaction {
            index_delta: 0,
            transaction: coinbase,
        }],
    };

    let mut state = PartialCompactBlock::new();
    let outcome = init_partial_compact_block(
        &mut state,
        &payload,
        std::iter::empty::<(&Wtxid, &Transaction)>(),
        std::iter::empty::<(&Wtxid, &Transaction)>(),
    );

    assert_eq!(
        outcome,
        CompactReconstructionOutcome::Invalid(
            CompactReconstructionInvalidReason::TransactionCountOutOfRange
        )
    );
}

#[test]
fn malformed_prefilled_delta_overflow_is_rejected() {
    let header = sample_header();
    let coinbase = coinbase_transaction();
    let payload = CompactBlockPayload {
        header,
        nonce: 1,
        short_ids: Vec::new(),
        prefilled_transactions: vec![PrefilledTransaction {
            index_delta: u64::MAX,
            transaction: coinbase,
        }],
    };

    let mut state = PartialCompactBlock::new();
    let outcome = init_partial_compact_block(
        &mut state,
        &payload,
        std::iter::empty::<(&Wtxid, &Transaction)>(),
        std::iter::empty::<(&Wtxid, &Transaction)>(),
    );

    assert_eq!(
        outcome,
        CompactReconstructionOutcome::Invalid(
            CompactReconstructionInvalidReason::MalformedPrefilledIndex
        )
    );
}

#[test]
fn scan_candidate_skips_unknown_short_ids_and_out_of_range_slots() {
    let header = sample_header();
    let coinbase = coinbase_transaction();
    let matched = sample_transaction(0x91, 0, 4_000);
    let (matched_wtxid, matched_tx) = transaction_pair(&matched);
    let unknown = sample_transaction(0x92, 0, 5_000);
    let (unknown_wtxid, unknown_tx) = transaction_pair(&unknown);

    let payload = compact_payload_with_short_ids(
        header,
        31,
        vec![PrefilledTransaction {
            index_delta: 0,
            transaction: coinbase,
        }],
        std::slice::from_ref(&matched),
    );

    let selector = short_id_selector_from_header_and_nonce(&payload.header, payload.nonce);
    let slot_one_short_id = compact_short_id_for_wtxid(selector, &matched_wtxid);
    let mut short_id_map = std::collections::HashMap::new();
    short_id_map.insert(short_id_match_key(slot_one_short_id), 1_u16);
    short_id_map.insert(0xdead_beef_cafe, 99_u16);

    let mut state = PartialCompactBlock::new();
    state.header = Some(payload.header);
    state.txn_available = vec![Some(coinbase_transaction()), None];
    state.slot_wtxids = vec![None, None];
    state.prefilled_count = 1;
    state.short_id_slots_remaining = 2;

    let mut matched_short_ids = 0_usize;
    super::scan_candidate_transactions(
        &mut state,
        &selector,
        &short_id_map,
        [(&unknown_wtxid, &unknown_tx), (&matched_wtxid, &matched_tx)],
        &mut matched_short_ids,
        false,
    );

    assert!(state.is_transaction_available(1));
}

#[test]
fn short_id_count_alone_above_limit_is_rejected() {
    let payload = CompactBlockPayload {
        header: sample_header(),
        nonce: 1,
        short_ids: vec![
            ShortId::from_wire_bytes([0_u8; 6]);
            MAX_COMPACT_BLOCK_TRANSACTION_COUNT + 1
        ],
        prefilled_transactions: Vec::new(),
    };

    let mut state = PartialCompactBlock::new();
    let outcome = init_partial_compact_block(
        &mut state,
        &payload,
        std::iter::empty::<(&Wtxid, &Transaction)>(),
        std::iter::empty::<(&Wtxid, &Transaction)>(),
    );

    assert_eq!(
        outcome,
        CompactReconstructionOutcome::Invalid(
            CompactReconstructionInvalidReason::TransactionCountOutOfRange
        )
    );
}

#[test]
fn prefilled_delta_overflow_is_rejected_before_placement() {
    let header = sample_header();
    let coinbase = coinbase_transaction();
    let payload = CompactBlockPayload {
        header,
        nonce: 1,
        short_ids: Vec::new(),
        prefilled_transactions: vec![
            PrefilledTransaction {
                index_delta: 0,
                transaction: coinbase.clone(),
            },
            PrefilledTransaction {
                index_delta: i64::from(i32::MAX) as u64,
                transaction: coinbase,
            },
        ],
    };

    let mut state = PartialCompactBlock::new();
    let outcome = init_partial_compact_block(
        &mut state,
        &payload,
        std::iter::empty::<(&Wtxid, &Transaction)>(),
        std::iter::empty::<(&Wtxid, &Transaction)>(),
    );

    assert_eq!(
        outcome,
        CompactReconstructionOutcome::Invalid(
            CompactReconstructionInvalidReason::MalformedPrefilledIndex
        )
    );
}

#[test]
fn scan_skips_out_of_range_slot_indexes() {
    let header = sample_header();
    let coinbase = coinbase_transaction();
    let matched = sample_transaction(0xa1, 0, 4_000);
    let (matched_wtxid, matched_tx) = transaction_pair(&matched);

    let payload = compact_payload_with_short_ids(
        header,
        41,
        vec![PrefilledTransaction {
            index_delta: 0,
            transaction: coinbase,
        }],
        std::slice::from_ref(&matched),
    );

    let selector = short_id_selector_from_header_and_nonce(&payload.header, payload.nonce);
    let slot_one_short_id = compact_short_id_for_wtxid(selector, &matched_wtxid);
    let mut short_id_map = std::collections::HashMap::new();
    short_id_map.insert(short_id_match_key(slot_one_short_id), 9_u16);

    let mut state = PartialCompactBlock::new();
    state.header = Some(payload.header);
    state.txn_available = vec![Some(coinbase_transaction()), None];
    state.slot_wtxids = vec![None, None];
    state.prefilled_count = 1;
    state.short_id_slots_remaining = 2;

    let mut matched_short_ids = 0_usize;
    super::scan_candidate_transactions(
        &mut state,
        &selector,
        &short_id_map,
        [(&matched_wtxid, &matched_tx)],
        &mut matched_short_ids,
        false,
    );

    assert!(!state.is_transaction_available(1));
}

#[test]
fn scan_stops_after_matching_all_short_id_slots() {
    let header = sample_header();
    let coinbase = coinbase_transaction();
    let first = sample_transaction(0xb1, 0, 4_000);
    let second = sample_transaction(0xb2, 0, 5_000);
    let (first_wtxid, first_tx) = transaction_pair(&first);
    let (second_wtxid, second_tx) = transaction_pair(&second);

    let payload = compact_payload_with_short_ids(
        header,
        51,
        vec![PrefilledTransaction {
            index_delta: 0,
            transaction: coinbase,
        }],
        &[first, second],
    );

    let selector = short_id_selector_from_header_and_nonce(&payload.header, payload.nonce);
    let first_short_id = compact_short_id_for_wtxid(selector, &first_wtxid);
    let second_short_id = compact_short_id_for_wtxid(selector, &second_wtxid);
    let mut short_id_map = std::collections::HashMap::new();
    short_id_map.insert(short_id_match_key(first_short_id), 1_u16);
    short_id_map.insert(short_id_match_key(second_short_id), 2_u16);

    let mut state = PartialCompactBlock::new();
    state.header = Some(payload.header);
    state.txn_available = vec![Some(coinbase_transaction()), None, None];
    state.slot_wtxids = vec![None, None, None];
    state.prefilled_count = 1;
    state.short_id_slots_remaining = 1;

    let mut matched_short_ids = 0_usize;
    super::scan_candidate_transactions(
        &mut state,
        &selector,
        &short_id_map,
        [(&first_wtxid, &first_tx), (&second_wtxid, &second_tx)],
        &mut matched_short_ids,
        false,
    );

    assert!(state.is_transaction_available(1));
    assert!(!state.is_transaction_available(2));
}

#[test]
fn prefilled_wtxid_encode_failure_is_rejected() {
    let header = sample_header();
    let mut coinbase = coinbase_transaction();
    coinbase.lock_time = super::TEST_PREFILLED_WTXID_FAILURE_LOCK_TIME;

    let payload = CompactBlockPayload {
        header,
        nonce: 1,
        short_ids: Vec::new(),
        prefilled_transactions: vec![PrefilledTransaction {
            index_delta: 0,
            transaction: coinbase,
        }],
    };

    let mut state = PartialCompactBlock::new();
    let outcome = init_partial_compact_block(
        &mut state,
        &payload,
        std::iter::empty::<(&Wtxid, &Transaction)>(),
        std::iter::empty::<(&Wtxid, &Transaction)>(),
    );

    assert_eq!(
        outcome,
        CompactReconstructionOutcome::Invalid(
            CompactReconstructionInvalidReason::NullPrefilledTransaction
        )
    );
}

#[test]
fn duplicate_extra_candidate_with_same_witness_keeps_prefilled_slot() {
    let header = sample_header();
    let coinbase = coinbase_transaction();
    let matched = sample_transaction(0xc1, 0, 4_000);
    let (matched_wtxid, matched_tx) = transaction_pair(&matched);

    let payload = compact_payload_with_short_ids(
        header,
        61,
        vec![PrefilledTransaction {
            index_delta: 0,
            transaction: coinbase,
        }],
        std::slice::from_ref(&matched),
    );

    let selector = short_id_selector_from_header_and_nonce(&payload.header, payload.nonce);
    let slot_one_short_id = compact_short_id_for_wtxid(selector, &matched_wtxid);
    let mut short_id_map = std::collections::HashMap::new();
    short_id_map.insert(short_id_match_key(slot_one_short_id), 1_u16);

    let mut state = PartialCompactBlock::new();
    state.header = Some(payload.header);
    state.txn_available = vec![Some(coinbase_transaction()), Some(matched_tx.clone())];
    state.slot_wtxids = vec![None, Some(matched_wtxid)];
    state.prefilled_count = 1;
    state.short_id_slots_remaining = 2;

    let mut matched_short_ids = 1_usize;
    super::scan_candidate_transactions(
        &mut state,
        &selector,
        &short_id_map,
        [(&matched_wtxid, &matched_tx)],
        &mut matched_short_ids,
        true,
    );

    assert!(state.is_transaction_available(1));
    assert_eq!(matched_short_ids, 1);
}

#[test]
fn should_clear_duplicate_slot_respects_witness_hash_comparison_mode() {
    let wtxid = Wtxid::from_byte_array([0x44; 32]);
    let other = Wtxid::from_byte_array([0x55; 32]);

    assert!(!super::should_clear_duplicate_slot(
        false, false, None, &wtxid
    ));
    assert!(super::should_clear_duplicate_slot(
        true,
        false,
        Some(&wtxid),
        &wtxid
    ));
    assert!(!super::should_clear_duplicate_slot(
        true,
        true,
        Some(&wtxid),
        &wtxid
    ));
    assert!(super::should_clear_duplicate_slot(
        true,
        true,
        Some(&wtxid),
        &other
    ));
}

#[test]
fn fill_block_returns_complete_block_when_all_slots_are_available() {
    let header = sample_header();
    let coinbase = coinbase_transaction();
    let matched = sample_transaction(0x11, 0, 5_000);
    let extra = sample_transaction(0x22, 1, 7_000);
    let short_id_transactions = [matched.clone(), extra.clone()];
    let (matched_wtxid, matched_tx) = transaction_pair(&short_id_transactions[0]);
    let (extra_wtxid, extra_tx) = transaction_pair(&short_id_transactions[1]);

    let payload = compact_payload_with_short_ids(
        header.clone(),
        42,
        vec![PrefilledTransaction {
            index_delta: 0,
            transaction: coinbase.clone(),
        }],
        &short_id_transactions,
    );

    let mut state = PartialCompactBlock::new();
    let _ = init_partial_compact_block(
        &mut state,
        &payload,
        [(&matched_wtxid, &matched_tx)],
        [(&extra_wtxid, &extra_tx)],
    );

    let block = super::fill_block(&state).expect("filled block");

    assert_eq!(block.header, header);
    assert_eq!(block.transactions.len(), 3);
}

#[test]
fn fill_block_rejects_incomplete_partial_state() {
    let (payload, _, _) = {
        let header = sample_header();
        let coinbase = coinbase_transaction();
        let missing = sample_transaction(0x22, 1, 7_000);
        let payload = compact_payload_with_short_ids(
            header,
            42,
            vec![PrefilledTransaction {
                index_delta: 0,
                transaction: coinbase,
            }],
            &[missing],
        );
        (payload, (), ())
    };

    let mut state = PartialCompactBlock::new();
    let _ = init_partial_compact_block(
        &mut state,
        &payload,
        std::iter::empty::<(&Wtxid, &Transaction)>(),
        std::iter::empty(),
    );

    assert_eq!(
        super::fill_block(&state),
        Err(super::CompactReconstructionInvalidReason::IncompleteTransactions)
    );
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

#[test]
fn fill_block_rejects_uninitialized_partial_state() {
    assert_eq!(
        fill_block(&PartialCompactBlock::new()),
        Err(CompactReconstructionInvalidReason::NullHeader)
    );
}

#[test]
fn apply_block_transactions_rejects_uninitialized_and_duplicate_responses() {
    let block_hash = BlockHash::from_byte_array([0xab; 32]);
    let transaction = sample_transaction(0x11, 0, 5_000);

    assert_eq!(
        apply_block_transactions(
            &mut PartialCompactBlock::new(),
            &open_bitcoin_codec::BlockTransactions {
                block_hash,
                transactions: vec![transaction.clone()],
            },
            block_hash,
        ),
        CompactBlockTxnOutcome::Misbehavior(CompactBlockTxnMisbehavior::NotInitialized)
    );

    let header = sample_header();
    let payload = compact_payload_with_short_ids(
        header,
        42,
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
    let expected_hash = open_bitcoin_consensus::block_hash(state.header().expect("header"));

    assert_eq!(
        apply_block_transactions(
            &mut state,
            &open_bitcoin_codec::BlockTransactions {
                block_hash: expected_hash,
                transactions: vec![transaction],
            },
            expected_hash,
        ),
        CompactBlockTxnOutcome::Misbehavior(CompactBlockTxnMisbehavior::DuplicateResponse)
    );
}

#[test]
fn apply_block_transactions_rejects_out_of_bounds_and_invalid_transactions() {
    let header = sample_header();
    let coinbase = coinbase_transaction();
    let missing = sample_transaction(0x22, 1, 7_000);
    let payload = compact_payload_with_short_ids(
        header.clone(),
        42,
        vec![PrefilledTransaction {
            index_delta: 0,
            transaction: coinbase.clone(),
        }],
        std::slice::from_ref(&missing),
    );
    let block_hash = open_bitcoin_consensus::block_hash(&header);
    let mut state = PartialCompactBlock::new();
    let _ = init_partial_compact_block(
        &mut state,
        &payload,
        std::iter::empty::<(&Wtxid, &Transaction)>(),
        std::iter::empty(),
    );

    state.txn_available[1] = Some(missing.clone());
    assert_eq!(
        apply_block_transactions(
            &mut state,
            &open_bitcoin_codec::BlockTransactions {
                block_hash,
                transactions: vec![missing.clone()],
            },
            block_hash,
        ),
        CompactBlockTxnOutcome::Misbehavior(CompactBlockTxnMisbehavior::DuplicateResponse)
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
    let mut invalid_state = PartialCompactBlock::new();
    let _ = init_partial_compact_block(
        &mut invalid_state,
        &payload,
        std::iter::empty::<(&Wtxid, &Transaction)>(),
        std::iter::empty(),
    );
    assert_eq!(
        apply_block_transactions(
            &mut invalid_state,
            &open_bitcoin_codec::BlockTransactions {
                block_hash,
                transactions: vec![null_transaction],
            },
            block_hash,
        ),
        CompactBlockTxnOutcome::Misbehavior(CompactBlockTxnMisbehavior::OutOfBoundsIndex)
    );
}

#[test]
fn apply_downloaded_transaction_at_index_rejects_invalid_slots_and_transactions() {
    let header = sample_header();
    let coinbase = coinbase_transaction();
    let missing = sample_transaction(0x22, 1, 7_000);
    let payload = compact_payload_with_short_ids(
        header,
        42,
        vec![PrefilledTransaction {
            index_delta: 0,
            transaction: coinbase.clone(),
        }],
        std::slice::from_ref(&missing),
    );
    let mut state = PartialCompactBlock::new();
    let _ = init_partial_compact_block(
        &mut state,
        &payload,
        std::iter::empty::<(&Wtxid, &Transaction)>(),
        std::iter::empty(),
    );

    assert_eq!(
        super::apply_downloaded_transaction_at_index(&mut state, 9, &missing),
        Err(CompactBlockTxnMisbehavior::OutOfBoundsIndex)
    );

    state.txn_available[1] = Some(missing.clone());
    assert_eq!(
        super::apply_downloaded_transaction_at_index(&mut state, 1, &missing),
        Err(CompactBlockTxnMisbehavior::DuplicateResponse)
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
    let mut fresh_state = PartialCompactBlock::new();
    let _ = init_partial_compact_block(
        &mut fresh_state,
        &payload,
        std::iter::empty::<(&Wtxid, &Transaction)>(),
        std::iter::empty(),
    );
    assert_eq!(
        super::apply_downloaded_transaction_at_index(&mut fresh_state, 1, &null_transaction),
        Err(CompactBlockTxnMisbehavior::OutOfBoundsIndex)
    );

    let mut wtxid_failure_tx = missing.clone();
    wtxid_failure_tx.lock_time = super::TEST_APPLY_WTXID_FAILURE_LOCK_TIME;
    assert_eq!(
        super::apply_downloaded_transaction_at_index(&mut fresh_state, 1, &wtxid_failure_tx),
        Err(CompactBlockTxnMisbehavior::OutOfBoundsIndex)
    );
}

#[test]
fn decoded_wtxid_for_apply_maps_codec_failures_to_misbehavior() {
    let mut transaction = sample_transaction(0x33, 1, 7_000);
    transaction.lock_time = super::TEST_APPLY_WTXID_FAILURE_LOCK_TIME;

    assert_eq!(
        super::decoded_wtxid_for_apply(&transaction),
        Err(CompactBlockTxnMisbehavior::OutOfBoundsIndex)
    );
}
