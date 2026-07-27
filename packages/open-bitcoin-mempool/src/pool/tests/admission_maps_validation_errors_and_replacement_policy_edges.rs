use super::*;

#[test]
fn admission_maps_validation_errors_and_replacement_policy_edges() {
    let (snapshot, coinbase_txids) = sample_chainstate_snapshot(3);
    let invalid = spend_transaction(
        coinbase_txids[0],
        0,
        500_000_001,
        TransactionInput::SEQUENCE_FINAL,
    );
    let validation_error =
        submit(&mut Mempool::default(), &snapshot, invalid).expect_err("invalid spend");
    assert!(matches!(validation_error, MempoolError::Validation { .. }));

    let original = spend_transaction(
        coinbase_txids[0],
        0,
        499_999_000,
        TransactionInput::MAX_SEQUENCE_NONFINAL - 1,
    );
    let mut mempool = Mempool::new(PolicyConfig {
        incremental_relay_fee_rate: crate::IncrementalRelayFeeRate::new(
            crate::FeeRate::from_sats_per_kvb(10_000),
        ),
        ..PolicyConfig::default()
    });
    submit(&mut mempool, &snapshot, original).expect("original");
    let conflict_txid = *mempool.entries().keys().next().expect("conflict txid");

    let absolute_fee_error = mempool
        .validate_replacement(
            &spend_transaction(
                coinbase_txids[0],
                0,
                499_998_000,
                TransactionInput::SEQUENCE_FINAL,
            ),
            &BTreeSet::from([conflict_txid]),
            1_000,
            TransactionVirtualSize::new(1),
        )
        .expect_err("absolute fee should fail");
    assert!(matches!(
        absolute_fee_error,
        MempoolError::ReplacementRejected { ref reason }
        if reason.contains("must exceed conflicting fee")
    ));

    let low_feerate_error = mempool
        .validate_replacement(
            &spend_transaction(
                coinbase_txids[0],
                0,
                499_998_000,
                TransactionInput::SEQUENCE_FINAL,
            ),
            &BTreeSet::from([conflict_txid]),
            2_000,
            TransactionVirtualSize::new(2_000),
        )
        .expect_err("feerate should fail");
    assert!(matches!(
        low_feerate_error,
        MempoolError::ReplacementRejected { ref reason }
        if reason.contains("replacement feerate")
    ));

    let incremental_error = mempool
        .validate_replacement(
            &spend_transaction(
                coinbase_txids[0],
                0,
                499_998_000,
                TransactionInput::SEQUENCE_FINAL,
            ),
            &BTreeSet::from([conflict_txid]),
            1_001,
            TransactionVirtualSize::new(10),
        )
        .expect_err("incremental relay bump should fail");
    assert!(matches!(
        incremental_error,
        MempoolError::ReplacementRejected { ref reason }
        if reason.contains("replacement fee bump")
    ));

    let stale_conflict = mempool.validate_replacement(
        &spend_transaction(
            coinbase_txids[0],
            0,
            499_998_000,
            TransactionInput::SEQUENCE_FINAL,
        ),
        &BTreeSet::from([Txid::from_byte_array([42_u8; 32])]),
        2_000,
        TransactionVirtualSize::new(100),
    );
    assert!(stale_conflict.is_ok());
}

#[test]
fn helper_functions_cover_missing_vout_and_limit_branches() {
    let (snapshot, coinbase_txids) = sample_chainstate_snapshot(2);
    let parent = spend_transaction(
        coinbase_txids[0],
        0,
        499_999_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    let parent_txid = open_bitcoin_consensus::transaction_txid(&parent).expect("txid");
    let parent_wtxid = open_bitcoin_consensus::transaction_wtxid(&parent).expect("wtxid");
    let entries = HashMap::from([(
        parent_txid,
        MempoolEntry::new(
            parent,
            parent_txid,
            parent_wtxid,
            Amount::from_sats(1000).expect("amount"),
            TransactionVirtualSize::new(100),
            400,
            0,
            crate::MempoolEntryMetadata::legacy_unknown(),
        ),
    )]);
    let mempool = Mempool {
        entries,
        ..Mempool::default()
    };
    let missing_vout = super::derive_input_contexts(
        &spend_transaction(
            parent_txid,
            9,
            499_998_000,
            TransactionInput::SEQUENCE_FINAL,
        ),
        &snapshot,
        &mempool,
    )
    .expect_err("missing vout should fail");
    assert!(matches!(missing_vout, MempoolError::MissingInput { .. }));

    let candidate_txid = Txid::from_byte_array([11_u8; 32]);
    let candidate = MempoolEntry::new(
        spend_transaction(
            coinbase_txids[1],
            0,
            499_999_000,
            TransactionInput::SEQUENCE_FINAL,
        ),
        candidate_txid,
        open_bitcoin_consensus::transaction_wtxid(&spend_transaction(
            coinbase_txids[1],
            0,
            499_999_000,
            TransactionInput::SEQUENCE_FINAL,
        ))
        .expect("wtxid"),
        Amount::from_sats(1000).expect("amount"),
        TransactionVirtualSize::new(100),
        400,
        0,
        crate::MempoolEntryMetadata::legacy_unknown(),
    );
    let mut descendant_parent = candidate.clone();
    descendant_parent.descendant_stats =
        crate::AggregateStats::new(2, TransactionVirtualSize::new(200), 2_000);
    let oversized_ancestor = super::validate_limits(
        &HashMap::from([(candidate_txid, candidate.clone())]),
        &PolicyConfig {
            max_ancestor_virtual_size: 50,
            ..PolicyConfig::default()
        },
        candidate_txid,
    )
    .expect_err("ancestor vsize should fail");
    assert!(matches!(
        oversized_ancestor,
        MempoolError::LimitExceeded { .. }
    ));

    let descendant_limit = super::validate_limits(
        &HashMap::from([(candidate_txid, descendant_parent)]),
        &PolicyConfig {
            max_descendant_count: 1,
            ..PolicyConfig::default()
        },
        candidate_txid,
    )
    .expect_err("descendant count should fail");
    assert!(matches!(
        descendant_limit,
        MempoolError::LimitExceeded { .. }
    ));

    let mut descendant_size_parent = candidate;
    descendant_size_parent.descendant_stats =
        crate::AggregateStats::new(1, TransactionVirtualSize::new(200), 1_000);
    let descendant_size = super::validate_limits(
        &HashMap::from([(candidate_txid, descendant_size_parent)]),
        &PolicyConfig {
            max_descendant_virtual_size: 50,
            ..PolicyConfig::default()
        },
        candidate_txid,
    )
    .expect_err("descendant size should fail");
    assert!(matches!(
        descendant_size,
        MempoolError::LimitExceeded { .. }
    ));
}

#[test]
fn trim_and_graph_helpers_cover_remaining_internal_branches() {
    let empty_mempool = Mempool::default();
    let mut prospective = super::prospective::ProspectiveMempool::new(&empty_mempool);
    let empty_trimmed = super::pressure::trim_prospective_to_capacity(
        &mut prospective,
        &PolicyConfig {
            mempool_capacity: crate::MempoolCapacity::new(0),
            ..PolicyConfig::default()
        },
    );
    assert!(empty_trimmed.expect("empty trim").is_empty());

    let base = spend_transaction(
        Txid::from_byte_array([1_u8; 32]),
        0,
        499_999_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    let base_txid = open_bitcoin_consensus::transaction_txid(&base).expect("txid");
    let base_wtxid = open_bitcoin_consensus::transaction_wtxid(&base).expect("wtxid");
    let left = spend_transaction(base_txid, 0, 499_998_000, TransactionInput::SEQUENCE_FINAL);
    let left_txid = open_bitcoin_consensus::transaction_txid(&left).expect("txid");
    let left_wtxid = open_bitcoin_consensus::transaction_wtxid(&left).expect("wtxid");
    let right = spend_transaction(base_txid, 0, 499_997_000, TransactionInput::SEQUENCE_FINAL);
    let right_txid = open_bitcoin_consensus::transaction_txid(&right).expect("txid");
    let right_wtxid = open_bitcoin_consensus::transaction_wtxid(&right).expect("wtxid");
    let mut leaf = spend_transaction(left_txid, 0, 499_996_000, TransactionInput::SEQUENCE_FINAL);
    leaf.inputs.push(TransactionInput {
        previous_output: OutPoint {
            txid: right_txid,
            vout: 0,
        },
        script_sig: script(&[0x01, 0x51]),
        sequence: TransactionInput::SEQUENCE_FINAL,
        witness: ScriptWitness::default(),
    });
    let leaf_txid = open_bitcoin_consensus::transaction_txid(&leaf).expect("txid");
    let leaf_wtxid = open_bitcoin_consensus::transaction_wtxid(&leaf).expect("wtxid");

    let entries = HashMap::from([
        (
            base_txid,
            MempoolEntry::new(
                base,
                base_txid,
                base_wtxid,
                Amount::from_sats(1000).expect("amount"),
                TransactionVirtualSize::new(100),
                400,
                0,
                crate::MempoolEntryMetadata::legacy_unknown(),
            ),
        ),
        (
            left_txid,
            MempoolEntry::new(
                left,
                left_txid,
                left_wtxid,
                Amount::from_sats(1000).expect("amount"),
                TransactionVirtualSize::new(100),
                400,
                0,
                crate::MempoolEntryMetadata::legacy_unknown(),
            ),
        ),
        (
            right_txid,
            MempoolEntry::new(
                right,
                right_txid,
                right_wtxid,
                Amount::from_sats(1000).expect("amount"),
                TransactionVirtualSize::new(100),
                400,
                0,
                crate::MempoolEntryMetadata::legacy_unknown(),
            ),
        ),
        (
            leaf_txid,
            MempoolEntry::new(
                leaf,
                leaf_txid,
                leaf_wtxid,
                Amount::from_sats(1000).expect("amount"),
                TransactionVirtualSize::new(100),
                400,
                0,
                crate::MempoolEntryMetadata::legacy_unknown(),
            ),
        ),
    ]);
    let recomputed = super::recompute_state(entries).expect("recompute");
    let ancestors = super::collect_ancestors(&recomputed.entries, leaf_txid);
    let descendants = super::collect_descendants(&recomputed.entries, base_txid);
    assert!(ancestors.contains(&base_txid));
    assert!(descendants.contains(&leaf_txid));
}

#[test]
fn recompute_state_skips_invalid_parent_links_and_candidate_eviction_is_reported() {
    let txid = Txid::from_byte_array([4_u8; 32]);
    let invalid_parent = MempoolEntry::new(
        spend_transaction(
            Txid::from_byte_array([1_u8; 32]),
            1,
            499_999_000,
            TransactionInput::SEQUENCE_FINAL,
        ),
        txid,
        open_bitcoin_consensus::transaction_wtxid(&spend_transaction(
            Txid::from_byte_array([1_u8; 32]),
            1,
            499_999_000,
            TransactionInput::SEQUENCE_FINAL,
        ))
        .expect("wtxid"),
        Amount::from_sats(100).expect("amount"),
        TransactionVirtualSize::new(100),
        400,
        0,
        crate::MempoolEntryMetadata::legacy_unknown(),
    );
    let state = super::recompute_state(HashMap::from([(txid, invalid_parent)])).expect("recompute");
    assert!(state.entries.get(&txid).expect("entry").parents.is_empty());

    let (snapshot, coinbase_txids) = sample_chainstate_snapshot(2);
    let mut mempool = Mempool::new(PolicyConfig {
        mempool_capacity: MempoolCapacity::new(0),
        ..PolicyConfig::default()
    });
    let error = submit(
        &mut mempool,
        &snapshot,
        spend_transaction(
            coinbase_txids[0],
            0,
            499_999_000,
            TransactionInput::SEQUENCE_FINAL,
        ),
    )
    .expect_err("tiny mempool should evict candidate");

    assert!(matches!(error, MempoolError::CandidateEvicted { .. }));
}

#[test]
fn validate_limits_reports_missing_candidate_as_internal_invariant() {
    // Arrange
    let entries = HashMap::new();
    let config = PolicyConfig::default();
    let candidate_txid = Txid::from_byte_array([9_u8; 32]);

    // Act
    let error = super::validate_limits(&entries, &config, candidate_txid)
        .expect_err("missing candidate should be reported without panicking");

    // Assert
    assert!(matches!(error, MempoolError::InternalInvariant { .. }));
    assert!(error.to_string().contains("candidate"));
}

#[test]
fn validate_limits_reports_missing_ancestor_as_internal_invariant() {
    // Arrange
    let candidate_txid = Txid::from_byte_array([9_u8; 32]);
    let missing_ancestor_txid = Txid::from_byte_array([8_u8; 32]);
    let transaction = spend_transaction(
        Txid::from_byte_array([7_u8; 32]),
        0,
        499_999_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    let mut entry = MempoolEntry::new(
        transaction.clone(),
        candidate_txid,
        open_bitcoin_consensus::transaction_wtxid(&transaction).expect("wtxid"),
        Amount::from_sats(100).expect("amount"),
        TransactionVirtualSize::new(100),
        400,
        0,
        crate::MempoolEntryMetadata::legacy_unknown(),
    );
    entry.parents.insert(missing_ancestor_txid);
    let entries = HashMap::from([(candidate_txid, entry)]);

    // Act
    let error = super::validate_limits(&entries, &PolicyConfig::default(), candidate_txid)
        .expect_err("missing ancestor should be reported without panicking");

    // Assert
    assert!(matches!(error, MempoolError::InternalInvariant { .. }));
    assert!(error.to_string().contains("ancestor"));
}
