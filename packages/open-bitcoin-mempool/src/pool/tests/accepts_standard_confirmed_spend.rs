// Parity breadcrumbs:
// - packages/bitcoin-knots/doc/policy/packages.md
// - packages/bitcoin-knots/src/kernel/mempool_options.h
// - packages/bitcoin-knots/src/kernel/mempool_removal_reason.h
// - packages/bitcoin-knots/src/policy/packages.cpp
// - packages/bitcoin-knots/src/policy/policy.h
// - packages/bitcoin-knots/src/policy/rbf.cpp
// - packages/bitcoin-knots/src/rpc/mempool.cpp
// - packages/bitcoin-knots/src/txmempool.cpp
// - packages/bitcoin-knots/src/txmempool.h

use super::*;

#[test]
fn accepts_standard_confirmed_spend() {
    let (snapshot, coinbase_txids) = sample_chainstate_snapshot(2);
    let transaction = spend_transaction(
        coinbase_txids[0],
        0,
        499_999_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    let mut mempool = Mempool::default();

    let result = submit(&mut mempool, &snapshot, transaction).expect("admission");
    let entry = mempool.entry(&result.accepted).expect("entry");

    assert!(result.replaced.is_empty());
    assert!(result.evicted.is_empty());
    assert_eq!(entry.ancestor_stats.count, 1);
}

#[test]
fn getters_expose_config_entries_and_total_virtual_size() {
    let (snapshot, coinbase_txids) = sample_chainstate_snapshot(2);
    let mut mempool = Mempool::default();
    let result = submit(
        &mut mempool,
        &snapshot,
        spend_transaction(
            coinbase_txids[0],
            0,
            499_999_000,
            TransactionInput::SEQUENCE_FINAL,
        ),
    )
    .expect("admission");

    assert_eq!(mempool.config().rbf_policy, RbfPolicy::Always);
    assert_eq!(mempool.entries().len(), 1);
    assert_eq!(
        mempool.total_virtual_size(),
        mempool.entry(&result.accepted).expect("entry").virtual_size
    );
}

#[test]
fn duplicate_transactions_and_missing_inputs_are_rejected() {
    let (snapshot, coinbase_txids) = sample_chainstate_snapshot(2);
    let transaction = spend_transaction(
        coinbase_txids[0],
        0,
        499_999_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    let mut mempool = Mempool::default();
    submit(&mut mempool, &snapshot, transaction.clone()).expect("first admission");

    let duplicate = submit(&mut mempool, &snapshot, transaction).expect_err("duplicate");
    assert!(matches!(
        duplicate,
        MempoolError::DuplicateTransaction { .. }
    ));

    let missing = submit(
        &mut Mempool::default(),
        &snapshot,
        spend_transaction(
            Txid::from_byte_array([8_u8; 32]),
            0,
            499_999_000,
            TransactionInput::SEQUENCE_FINAL,
        ),
    )
    .expect_err("missing input");
    assert!(matches!(missing, MempoolError::MissingInput { .. }));
}

#[test]
fn rejects_non_standard_output_scripts() {
    let (snapshot, coinbase_txids) = sample_chainstate_snapshot(2);
    let error = submit(
        &mut Mempool::default(),
        &snapshot,
        non_standard_spend(coinbase_txids[0]),
    )
    .expect_err("non-standard output should fail");

    assert!(matches!(error, MempoolError::NonStandard { .. }));
}

#[test]
fn rejects_entries_that_exceed_ancestor_limits() {
    let (snapshot, coinbase_txids) = sample_chainstate_snapshot(2);
    let parent = spend_transaction(
        coinbase_txids[0],
        0,
        499_999_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    let parent_txid = open_bitcoin_consensus::transaction_txid(&parent).expect("txid");
    let child = spend_transaction(
        parent_txid,
        0,
        499_998_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    let mut mempool = Mempool::new(PolicyConfig {
        max_ancestor_count: 1,
        ..PolicyConfig::default()
    });

    submit(&mut mempool, &snapshot, parent).expect("parent");
    let error = submit(&mut mempool, &snapshot, child).expect_err("limit should fail");

    assert!(matches!(
        error,
        MempoolError::LimitExceeded {
            direction: LimitDirection::Ancestor,
            kind: LimitKind::Count,
            ..
        }
    ));
}

#[test]
fn tracks_parent_child_and_ancestor_descendant_metrics() {
    let (snapshot, coinbase_txids) = sample_chainstate_snapshot(2);
    let parent = spend_transaction(
        coinbase_txids[0],
        0,
        499_999_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    let parent_txid = open_bitcoin_consensus::transaction_txid(&parent).expect("txid");
    let child = spend_transaction(
        parent_txid,
        0,
        499_998_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    let mut mempool = Mempool::default();

    submit(&mut mempool, &snapshot, parent).expect("parent");
    let child_result = submit(&mut mempool, &snapshot, child).expect("child");
    let child_entry = mempool.entry(&child_result.accepted).expect("child entry");
    let parent_entry = mempool.entry(&parent_txid).expect("parent entry");

    assert_eq!(child_entry.parents, BTreeSet::from([parent_txid]));
    assert_eq!(child_entry.ancestor_stats.count, 2);
    assert_eq!(
        parent_entry.children,
        BTreeSet::from([child_result.accepted])
    );
    assert_eq!(parent_entry.descendant_stats.count, 2);
}

#[test]
fn replacement_requires_a_fee_bump() {
    let (snapshot, coinbase_txids) = sample_chainstate_snapshot(3);
    let original = spend_transaction(
        coinbase_txids[0],
        0,
        499_999_000,
        TransactionInput::MAX_SEQUENCE_NONFINAL - 1,
    );
    let lower_fee_replacement = spend_transaction(
        coinbase_txids[0],
        0,
        499_999_100,
        TransactionInput::SEQUENCE_FINAL,
    );
    let higher_fee_replacement = spend_transaction(
        coinbase_txids[0],
        0,
        499_998_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    let mut mempool = Mempool::new(PolicyConfig {
        rbf_policy: RbfPolicy::OptIn,
        ..PolicyConfig::default()
    });

    let original_result = submit(&mut mempool, &snapshot, original).expect("original");
    let lower_error = submit(&mut mempool, &snapshot, lower_fee_replacement)
        .expect_err("lower fee replacement should fail");
    let higher_result =
        submit(&mut mempool, &snapshot, higher_fee_replacement).expect("replacement");

    assert!(matches!(
        lower_error,
        MempoolError::ReplacementRejected { .. }
    ));
    assert_eq!(higher_result.replaced, vec![original_result.accepted]);
}

#[test]
fn replacement_requires_opt_in_signal_when_policy_demands_it() {
    let (snapshot, coinbase_txids) = sample_chainstate_snapshot(2);
    let original = spend_transaction(
        coinbase_txids[0],
        0,
        499_999_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    let replacement = spend_transaction(
        coinbase_txids[0],
        0,
        499_998_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    let mut mempool = Mempool::new(PolicyConfig {
        rbf_policy: RbfPolicy::OptIn,
        ..PolicyConfig::default()
    });

    submit(&mut mempool, &snapshot, original).expect("original");
    let error = submit(&mut mempool, &snapshot, replacement).expect_err("opt-in required");

    assert!(matches!(error, MempoolError::ConflictNotAllowed { .. }));
}

#[test]
fn replacement_rejects_new_unconfirmed_inputs() {
    let (snapshot, coinbase_txids) = sample_chainstate_snapshot(3);
    let parent = spend_transaction(
        coinbase_txids[1],
        0,
        499_999_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    let parent_txid = open_bitcoin_consensus::transaction_txid(&parent).expect("txid");
    let original = spend_transaction(
        coinbase_txids[0],
        0,
        499_999_000,
        TransactionInput::MAX_SEQUENCE_NONFINAL - 1,
    );
    let original_txid = open_bitcoin_consensus::transaction_txid(&original).expect("txid");
    let mut replacement = spend_transaction(
        coinbase_txids[0],
        0,
        499_997_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    replacement.inputs.push(TransactionInput {
        previous_output: OutPoint {
            txid: parent_txid,
            vout: 0,
        },
        script_sig: script(&[0x01, 0x51]),
        sequence: TransactionInput::SEQUENCE_FINAL,
        witness: ScriptWitness::default(),
    });
    let mut mempool = Mempool::new(PolicyConfig {
        rbf_policy: RbfPolicy::Always,
        ..PolicyConfig::default()
    });

    submit(&mut mempool, &snapshot, parent).expect("parent");
    submit(&mut mempool, &snapshot, original).expect("original");
    assert!(mempool.entry(&original_txid).is_some());

    let error = submit(&mut mempool, &snapshot, replacement)
        .expect_err("replacement with new unconfirmed input should fail");

    assert!(matches!(error, MempoolError::ReplacementRejected { .. }));
}

#[test]
fn evicts_lowest_descendant_score_package_when_size_limit_is_exceeded() {
    let (snapshot, coinbase_txids) = sample_chainstate_snapshot(3);
    let low_fee = spend_transaction(
        coinbase_txids[0],
        0,
        499_999_200,
        TransactionInput::SEQUENCE_FINAL,
    );
    let high_fee = spend_transaction(
        coinbase_txids[1],
        0,
        499_998_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    let mut probe = Mempool::default();
    submit(
        &mut probe,
        &snapshot,
        spend_transaction(
            coinbase_txids[2],
            0,
            499_999_000,
            TransactionInput::SEQUENCE_FINAL,
        ),
    )
    .expect("probe");
    let mut mempool = Mempool::new(PolicyConfig {
        mempool_capacity: MempoolCapacity::new(probe.accounted_memory().as_usize()),
        ..PolicyConfig::default()
    });

    let low_fee_result = submit(&mut mempool, &snapshot, low_fee).expect("low fee");
    let high_fee_result = submit(&mut mempool, &snapshot, high_fee).expect("high fee");

    assert_eq!(high_fee_result.evicted, vec![low_fee_result.accepted]);
    assert!(mempool.entry(&low_fee_result.accepted).is_none());
    assert!(mempool.entry(&high_fee_result.accepted).is_some());
}

#[test]
fn replacements_respect_disabled_policy() {
    let (snapshot, coinbase_txids) = sample_chainstate_snapshot(2);
    let original = spend_transaction(
        coinbase_txids[0],
        0,
        499_999_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    let replacement = spend_transaction(
        coinbase_txids[0],
        0,
        499_998_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    let mut mempool = Mempool::new(PolicyConfig {
        rbf_policy: RbfPolicy::Never,
        ..PolicyConfig::default()
    });

    submit(&mut mempool, &snapshot, original).expect("original");
    let error = submit(&mut mempool, &snapshot, replacement).expect_err("rbf disabled");

    assert!(matches!(error, MempoolError::ConflictNotAllowed { .. }));
}

#[test]
fn direct_helper_paths_cover_internal_edge_branches() {
    let empty_snapshot = ChainstateSnapshot::new(Vec::new(), HashMap::new(), HashMap::new());
    let context = super::build_validation_context(
        &empty_snapshot,
        Vec::new(),
        ScriptVerifyFlags::NONE,
        ConsensusParams::default(),
    );
    assert_eq!(context.spend_height, 0);
    assert_eq!(context.block_time, 0);

    let config = PolicyConfig::default();
    let effective_fee_rate = crate::effective_admission_fee_rate(
        config.static_relay_fee_rate,
        crate::RollingMempoolFeeRate::ZERO,
    );
    let relay_error =
        super::enforce_min_relay_fee(effective_fee_rate, 0, TransactionVirtualSize::new(100))
            .expect_err("fee floor should fail");
    assert!(matches!(relay_error, MempoolError::RelayFeeTooLow { .. }));

    let invalid_fee = super::amount_from_fee_sats(-1).expect_err("negative fee should fail");
    assert!(matches!(invalid_fee, MempoolError::Validation { .. }));
    let serialization_error = super::serialization_validation_error(
        "transaction txid",
        open_bitcoin_codec::CodecError::CompactSizeTooLarge(33_554_433),
    );
    assert!(matches!(
        serialization_error,
        MempoolError::Validation { .. }
    ));

    let candidate_txid = Txid::from_byte_array([7_u8; 32]);
    let candidate_transaction = spend_transaction(
        Txid::from_byte_array([6_u8; 32]),
        0,
        499_999_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    let candidate_entry = MempoolEntry::new(
        candidate_transaction.clone(),
        candidate_txid,
        open_bitcoin_consensus::transaction_wtxid(&candidate_transaction).expect("wtxid"),
        Amount::from_sats(100).expect("amount"),
        TransactionVirtualSize::new(100),
        400,
        0,
        crate::MempoolEntryMetadata::legacy_unknown(),
    );
    let missing_candidate = super::validate_limits(
        &HashMap::from([(candidate_txid, candidate_entry)]),
        &PolicyConfig {
            max_ancestor_count: 0,
            ..PolicyConfig::default()
        },
        candidate_txid,
    );
    assert!(missing_candidate.is_err());

    assert!(super::pressure::select_eviction_candidate(&HashMap::new()).is_none());
    let missing_ancestors =
        super::collect_ancestors(&HashMap::new(), Txid::from_byte_array([1_u8; 32]));
    let missing_descendants =
        super::collect_descendants(&HashMap::new(), Txid::from_byte_array([1_u8; 32]));
    assert!(missing_ancestors.is_empty());
    assert!(missing_descendants.is_empty());
}
