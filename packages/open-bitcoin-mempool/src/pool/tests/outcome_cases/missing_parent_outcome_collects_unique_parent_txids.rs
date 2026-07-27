use super::*;

#[test]
fn missing_parent_outcome_collects_unique_parent_txids() {
    // Arrange
    let (snapshot, coinbase_txids) = sample_chainstate_snapshot(2);
    let mut mempool = Mempool::default();
    let mempool_parent_txid = submit(
        &mut mempool,
        &snapshot,
        spend_transaction(
            coinbase_txids[1],
            0,
            499_999_000,
            TransactionInput::SEQUENCE_FINAL,
        ),
    )
    .expect("mempool parent")
    .accepted;
    let first_missing = Txid::from_byte_array([9_u8; 32]);
    let second_missing = Txid::from_byte_array([10_u8; 32]);
    let mut transaction = spend_transaction(
        first_missing,
        0,
        499_999_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    transaction.inputs.push(TransactionInput {
        previous_output: OutPoint {
            txid: second_missing,
            vout: 0,
        },
        script_sig: script(&[0x01, 0x51]),
        sequence: TransactionInput::SEQUENCE_FINAL,
        witness: ScriptWitness::default(),
    });
    transaction.inputs.push(TransactionInput {
        previous_output: OutPoint {
            txid: first_missing,
            vout: 1,
        },
        script_sig: script(&[0x01, 0x51]),
        sequence: TransactionInput::SEQUENCE_FINAL,
        witness: ScriptWitness::default(),
    });
    transaction.inputs.push(TransactionInput {
        previous_output: OutPoint {
            txid: coinbase_txids[0],
            vout: 0,
        },
        script_sig: script(&[0x01, 0x51]),
        sequence: TransactionInput::SEQUENCE_FINAL,
        witness: ScriptWitness::default(),
    });
    transaction.inputs.push(TransactionInput {
        previous_output: OutPoint {
            txid: mempool_parent_txid,
            vout: 0,
        },
        script_sig: script(&[0x01, 0x51]),
        sequence: TransactionInput::SEQUENCE_FINAL,
        witness: ScriptWitness::default(),
    });
    transaction.inputs.push(TransactionInput {
        previous_output: OutPoint {
            txid: mempool_parent_txid,
            vout: 1,
        },
        script_sig: script(&[0x01, 0x51]),
        sequence: TransactionInput::SEQUENCE_FINAL,
        witness: ScriptWitness::default(),
    });

    // Act
    let outcome = mempool
        .accept_transaction_outcome_with_context(
            transaction,
            &snapshot,
            ScriptVerifyFlags::P2SH
                | ScriptVerifyFlags::CHECKLOCKTIMEVERIFY
                | ScriptVerifyFlags::CHECKSEQUENCEVERIFY,
            ConsensusParams {
                coinbase_maturity: 1,
                ..ConsensusParams::default()
            },
            crate::AdmissionContext::legacy_unknown(),
        )
        .expect("orphan outcome");

    // Assert
    assert_eq!(outcome.label(), MempoolOutcomeLabel::Orphaned);
    assert_eq!(
        outcome.missing_parents(),
        &[first_missing, second_missing, mempool_parent_txid]
    );
}

#[test]
fn replacement_outcome_distinguishes_replaced_and_evicted_transactions() {
    // Arrange
    let (snapshot, coinbase_txids) = sample_chainstate_snapshot(4);
    let original = spend_transaction(
        coinbase_txids[0],
        0,
        499_999_000,
        TransactionInput::MAX_SEQUENCE_NONFINAL - 1,
    );
    let unrelated_low_fee = spend_transaction(
        coinbase_txids[1],
        0,
        499_999_300,
        TransactionInput::SEQUENCE_FINAL,
    );
    let mut staging = Mempool::default();
    submit(&mut staging, &snapshot, original.clone()).expect("stage original");
    submit(&mut staging, &snapshot, unrelated_low_fee.clone()).expect("stage unrelated");
    let two_entry_usage = staging.accounted_memory().as_usize();
    let mut mempool = Mempool::new(PolicyConfig {
        rbf_policy: RbfPolicy::Always,
        mempool_capacity: MempoolCapacity::new(two_entry_usage),
        ..PolicyConfig::default()
    });
    let original_txid = submit(&mut mempool, &snapshot, original)
        .expect("original")
        .accepted;
    let evicted_txid = submit(&mut mempool, &snapshot, unrelated_low_fee)
        .expect("unrelated")
        .accepted;
    let mut replacement = spend_transaction(
        coinbase_txids[0],
        0,
        499_996_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    replacement.inputs.push(TransactionInput {
        previous_output: OutPoint {
            txid: coinbase_txids[2],
            vout: 0,
        },
        script_sig: script(&[0x01, 0x51]),
        sequence: TransactionInput::SEQUENCE_FINAL,
        witness: ScriptWitness::default(),
    });

    // Act
    let outcome = mempool
        .accept_transaction_outcome_with_context(
            replacement,
            &snapshot,
            ScriptVerifyFlags::P2SH
                | ScriptVerifyFlags::CHECKLOCKTIMEVERIFY
                | ScriptVerifyFlags::CHECKSEQUENCEVERIFY,
            ConsensusParams {
                coinbase_maturity: 1,
                ..ConsensusParams::default()
            },
            crate::AdmissionContext::legacy_unknown(),
        )
        .expect("replacement outcome");

    // Assert
    assert_eq!(outcome.label(), MempoolOutcomeLabel::Replaced);
    assert_eq!(outcome.replaced(), &[original_txid]);
    assert_eq!(outcome.evicted(), &[evicted_txid]);
}

#[test]
fn no_partial_mutation_for_non_standard_rejection() {
    // Arrange
    let (snapshot, coinbase_txids) = sample_chainstate_snapshot(3);
    let mut mempool = Mempool::default();
    submit(
        &mut mempool,
        &snapshot,
        spend_transaction(
            coinbase_txids[0],
            0,
            499_999_000,
            TransactionInput::SEQUENCE_FINAL,
        ),
    )
    .expect("baseline admission");
    let before = MempoolAdmissionSnapshot::capture(&mempool);

    // Act
    let outcome = submit_outcome(
        &mut mempool,
        &snapshot,
        non_standard_spend(coinbase_txids[1]),
    );

    // Assert
    assert_eq!(outcome.label(), MempoolOutcomeLabel::Rejected);
    assert_eq!(
        outcome.maybe_rejection_category(),
        Some(MempoolRejectionCategory::NonStandard)
    );
    assert_eq!(MempoolAdmissionSnapshot::capture(&mempool), before);
}

#[test]
fn no_partial_mutation_for_low_fee_rejection() {
    // Arrange
    let (snapshot, coinbase_txids) = sample_chainstate_snapshot(3);
    let mut mempool = Mempool::default();
    submit(
        &mut mempool,
        &snapshot,
        spend_transaction(
            coinbase_txids[0],
            0,
            499_999_000,
            TransactionInput::SEQUENCE_FINAL,
        ),
    )
    .expect("baseline admission");
    let low_fee = spend_transaction(
        coinbase_txids[1],
        0,
        499_999_999,
        TransactionInput::SEQUENCE_FINAL,
    );
    let before = MempoolAdmissionSnapshot::capture(&mempool);

    // Act
    let outcome = submit_outcome(&mut mempool, &snapshot, low_fee);

    // Assert
    assert_eq!(outcome.label(), MempoolOutcomeLabel::Rejected);
    assert_eq!(
        outcome.maybe_rejection_category(),
        Some(MempoolRejectionCategory::RelayFeeTooLow)
    );
    assert_eq!(MempoolAdmissionSnapshot::capture(&mempool), before);
}

#[test]
fn no_partial_mutation_for_failed_replacement() {
    // Arrange
    let (snapshot, coinbase_txids) = sample_chainstate_snapshot(2);
    let mut mempool = Mempool::new(PolicyConfig {
        rbf_policy: RbfPolicy::Always,
        ..PolicyConfig::default()
    });
    let original = spend_transaction(
        coinbase_txids[0],
        0,
        499_999_000,
        TransactionInput::MAX_SEQUENCE_NONFINAL - 1,
    );
    submit(&mut mempool, &snapshot, original).expect("original admission");
    let equal_fee_replacement = spend_transaction(
        coinbase_txids[0],
        0,
        499_999_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    let before = MempoolAdmissionSnapshot::capture(&mempool);

    // Act
    let outcome = submit_outcome(&mut mempool, &snapshot, equal_fee_replacement);

    // Assert
    assert_eq!(outcome.label(), MempoolOutcomeLabel::Rejected);
    assert_eq!(
        outcome.maybe_rejection_category(),
        Some(MempoolRejectionCategory::ReplacementRejected)
    );
    assert_eq!(MempoolAdmissionSnapshot::capture(&mempool), before);
}

#[test]
fn no_partial_mutation_for_ancestor_limit_rejection() {
    // Arrange
    let (snapshot, coinbase_txids) = sample_chainstate_snapshot(1);
    let mut mempool = Mempool::new(PolicyConfig {
        max_ancestor_count: 1,
        ..PolicyConfig::default()
    });
    let parent_txid = submit(
        &mut mempool,
        &snapshot,
        spend_transaction(
            coinbase_txids[0],
            0,
            499_999_000,
            TransactionInput::SEQUENCE_FINAL,
        ),
    )
    .expect("parent admission")
    .accepted;
    let child = spend_transaction(
        parent_txid,
        0,
        499_998_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    let before = MempoolAdmissionSnapshot::capture(&mempool);

    // Act
    let outcome = submit_outcome(&mut mempool, &snapshot, child);

    // Assert
    assert_eq!(outcome.label(), MempoolOutcomeLabel::Rejected);
    assert_eq!(
        outcome.maybe_rejection_category(),
        Some(MempoolRejectionCategory::LimitExceeded)
    );
    assert_eq!(MempoolAdmissionSnapshot::capture(&mempool), before);
}

#[test]
fn no_partial_mutation_for_descendant_limit_rejection() {
    // Arrange
    let (snapshot, coinbase_txids) = sample_chainstate_snapshot(1);
    let mut mempool = Mempool::new(PolicyConfig {
        max_descendant_count: 1,
        ..PolicyConfig::default()
    });
    let parent_txid = submit(
        &mut mempool,
        &snapshot,
        spend_transaction(
            coinbase_txids[0],
            0,
            499_999_000,
            TransactionInput::SEQUENCE_FINAL,
        ),
    )
    .expect("parent admission")
    .accepted;
    let child = spend_transaction(
        parent_txid,
        0,
        499_998_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    let before = MempoolAdmissionSnapshot::capture(&mempool);

    // Act
    let outcome = submit_outcome(&mut mempool, &snapshot, child);

    // Assert
    assert_eq!(outcome.label(), MempoolOutcomeLabel::Rejected);
    assert_eq!(
        outcome.maybe_rejection_category(),
        Some(MempoolRejectionCategory::LimitExceeded)
    );
    assert_eq!(MempoolAdmissionSnapshot::capture(&mempool), before);
}

#[test]
fn no_partial_mutation_for_candidate_evicted() {
    // Arrange
    let (snapshot, coinbase_txids) = sample_chainstate_snapshot(1);
    let mut mempool = Mempool::new(PolicyConfig {
        mempool_capacity: MempoolCapacity::new(0),
        ..PolicyConfig::default()
    });
    let candidate = spend_transaction(
        coinbase_txids[0],
        0,
        499_999_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    let before = MempoolAdmissionSnapshot::capture(&mempool);

    // Act
    let outcome = submit_outcome(&mut mempool, &snapshot, candidate);

    // Assert
    assert_eq!(outcome.label(), MempoolOutcomeLabel::Evicted);
    assert_eq!(MempoolAdmissionSnapshot::capture(&mempool), before);
}
