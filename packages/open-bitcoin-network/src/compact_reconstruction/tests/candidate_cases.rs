// Parity breadcrumbs:
// - packages/bitcoin-knots/src/blockencodings.cpp
// - packages/bitcoin-knots/src/blockencodings.h
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/test/functional/p2p_compactblocks.py

use super::*;

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
    super::super::scan_candidate_transactions(
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
    super::super::scan_candidate_transactions(
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
    super::super::scan_candidate_transactions(
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
    super::super::scan_candidate_transactions(
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
    super::super::scan_candidate_transactions(
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
    super::super::scan_candidate_transactions(
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

    assert!(!super::super::should_clear_duplicate_slot(
        false, false, None, &wtxid
    ));
    assert!(super::super::should_clear_duplicate_slot(
        true,
        false,
        Some(&wtxid),
        &wtxid
    ));
    assert!(!super::super::should_clear_duplicate_slot(
        true,
        true,
        Some(&wtxid),
        &wtxid
    ));
    assert!(super::super::should_clear_duplicate_slot(
        true,
        true,
        Some(&wtxid),
        &other
    ));
}
