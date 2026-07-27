// Parity breadcrumbs:
// - packages/bitcoin-knots/src/blockencodings.cpp
// - packages/bitcoin-knots/src/blockencodings.h
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/test/functional/p2p_compactblocks.py

use super::*;

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
fn prefilled_wtxid_encode_failure_is_rejected() {
    let header = sample_header();
    let mut coinbase = coinbase_transaction();
    coinbase.lock_time = super::super::TEST_PREFILLED_WTXID_FAILURE_LOCK_TIME;

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
