// Parity breadcrumbs:
// - packages/bitcoin-knots/src/blockencodings.cpp
// - packages/bitcoin-knots/src/blockencodings.h
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/test/functional/p2p_compactblocks.py

use super::*;

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

    let block = super::super::fill_block(&state).expect("filled block");

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
        super::super::fill_block(&state),
        Err(super::super::CompactReconstructionInvalidReason::IncompleteTransactions)
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
        super::super::apply_downloaded_transaction_at_index(&mut state, 9, &missing),
        Err(CompactBlockTxnMisbehavior::OutOfBoundsIndex)
    );

    state.txn_available[1] = Some(missing.clone());
    assert_eq!(
        super::super::apply_downloaded_transaction_at_index(&mut state, 1, &missing),
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
        super::super::apply_downloaded_transaction_at_index(&mut fresh_state, 1, &null_transaction),
        Err(CompactBlockTxnMisbehavior::OutOfBoundsIndex)
    );

    let mut wtxid_failure_tx = missing.clone();
    wtxid_failure_tx.lock_time = super::super::TEST_APPLY_WTXID_FAILURE_LOCK_TIME;
    assert_eq!(
        super::super::apply_downloaded_transaction_at_index(&mut fresh_state, 1, &wtxid_failure_tx),
        Err(CompactBlockTxnMisbehavior::OutOfBoundsIndex)
    );
}

#[test]
fn decoded_wtxid_for_apply_maps_codec_failures_to_misbehavior() {
    let mut transaction = sample_transaction(0x33, 1, 7_000);
    transaction.lock_time = super::super::TEST_APPLY_WTXID_FAILURE_LOCK_TIME;

    assert_eq!(
        super::super::decoded_wtxid_for_apply(&transaction),
        Err(CompactBlockTxnMisbehavior::OutOfBoundsIndex)
    );
}
