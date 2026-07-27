// Parity breadcrumbs:
// - packages/bitcoin-knots/src/kernel/mempool_removal_reason.h
// - packages/bitcoin-knots/src/txmempool.h
// - packages/bitcoin-knots/src/txmempool.cpp
// - packages/bitcoin-knots/src/validation.cpp

use super::*;

#[test]
fn replacement_transition_preserves_direct_and_descendant_roles() {
    // Arrange
    let (snapshot, coinbase_txids) = sample_chainstate_snapshot(2);
    let original = spend_transaction(
        coinbase_txids[0],
        0,
        499_999_000,
        TransactionInput::MAX_SEQUENCE_NONFINAL - 1,
    );
    let original_txid = transaction_txid(&original).expect("original txid");
    let descendant = spend_transaction(
        original_txid,
        0,
        499_998_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    let descendant_txid = transaction_txid(&descendant).expect("descendant txid");
    let replacement = spend_transaction(
        coinbase_txids[0],
        0,
        499_996_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    let replacement_txid = transaction_txid(&replacement).expect("replacement txid");
    let mut mempool = Mempool::default();
    submit(&mut mempool, &snapshot, original).expect("original admission");
    submit(&mut mempool, &snapshot, descendant).expect("descendant admission");

    // Act
    let transition = submit_transition(
        &mut mempool,
        &snapshot,
        replacement,
        AdmissionContext::legacy_unknown(),
    );

    // Assert
    assert!(matches!(
        transition.outcome,
        MempoolOutcome::Replaced { .. }
    ));
    assert!(transition.delta.removed.iter().any(|removal| {
        removal.member.txid == original_txid
            && removal.cause == MempoolRemovalCause::Replacement
            && removal.role == MempoolRemovalRole::Direct
    }));
    assert!(transition.delta.removed.iter().any(|removal| {
        removal.member.txid == descendant_txid
            && removal.cause == MempoolRemovalCause::Replacement
            && removal.role == MempoolRemovalRole::Descendant
    }));
    assert!(transition.delta.final_membership.iter().any(|state| {
        state.member.txid == replacement_txid && state.membership == FinalMempoolMembership::Present
    }));
}

#[test]
fn pressure_transition_reports_roles_and_bumps_rolling_fee() {
    // Arrange
    let (snapshot, coinbase_txids) = sample_chainstate_snapshot(3);
    let low_fee_parent = spend_transaction(
        coinbase_txids[0],
        0,
        499_999_900,
        TransactionInput::SEQUENCE_FINAL,
    );
    let low_fee_parent_txid = transaction_txid(&low_fee_parent).expect("low fee parent txid");
    let low_fee_child = spend_transaction(
        low_fee_parent_txid,
        0,
        499_998_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    let low_fee_child_txid = transaction_txid(&low_fee_child).expect("low fee child txid");
    let high_fee = spend_transaction(
        coinbase_txids[1],
        0,
        499_997_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    let mut staging = Mempool::default();
    submit(&mut staging, &snapshot, low_fee_parent.clone()).expect("stage parent");
    submit(&mut staging, &snapshot, low_fee_child.clone()).expect("stage child");
    let package_usage = staging.accounted_memory().as_usize();
    let mut mempool = Mempool::new(PolicyConfig {
        mempool_capacity: MempoolCapacity::new(package_usage),
        ..PolicyConfig::default()
    });
    submit(&mut mempool, &snapshot, low_fee_parent).expect("low fee parent admission");
    submit(&mut mempool, &snapshot, low_fee_child).expect("low fee child admission");

    // Act
    let transition = submit_transition(
        &mut mempool,
        &snapshot,
        high_fee,
        AdmissionContext::legacy_unknown(),
    );

    // Assert
    assert!(transition.delta.removed.iter().any(|removal| {
        removal.member.txid == low_fee_parent_txid
            && removal.cause == MempoolRemovalCause::Pressure
            && removal.role == MempoolRemovalRole::Direct
    }));
    assert!(transition.delta.removed.iter().any(|removal| {
        removal.member.txid == low_fee_child_txid
            && removal.cause == MempoolRemovalCause::Pressure
            && removal.role == MempoolRemovalRole::Descendant
    }));
    assert_ne!(
        mempool.rolling_mempool_fee_rate(),
        RollingMempoolFeeRate::ZERO
    );
}

#[test]
fn connected_block_transition_distinguishes_confirmation_from_conflict_descendants() {
    // Arrange
    let (snapshot, coinbase_txids) = sample_chainstate_snapshot(2);
    let confirmed = spend_transaction(
        coinbase_txids[0],
        0,
        499_999_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    let confirmed_txid = transaction_txid(&confirmed).expect("confirmed txid");
    let conflict = spend_transaction(
        coinbase_txids[1],
        0,
        499_999_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    let conflict_txid = transaction_txid(&conflict).expect("conflict txid");
    let descendant = spend_transaction(
        conflict_txid,
        0,
        499_998_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    let descendant_txid = transaction_txid(&descendant).expect("descendant txid");
    let in_block_conflict = spend_transaction(
        coinbase_txids[1],
        0,
        499_997_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    let mut block = build_block(BlockHash::from_byte_array([0; 32]), 3, 499_999_000);
    block.transactions.push(confirmed.clone());
    block.transactions.push(in_block_conflict);
    let mut mempool = Mempool::default();
    submit(&mut mempool, &snapshot, confirmed).expect("confirmed admission");
    submit(&mut mempool, &snapshot, conflict).expect("conflict admission");
    submit(&mut mempool, &snapshot, descendant).expect("descendant admission");

    // Act
    let delta = mempool
        .remove_for_connected_block_transition(
            &block,
            BlockLifecycleContext::new(PolicyTime::new(70), 3),
        )
        .expect("block transition");

    // Assert
    assert!(delta.removed.iter().any(|removal| {
        removal.member.txid == confirmed_txid
            && removal.cause == MempoolRemovalCause::BlockConfirmation
            && removal.role == MempoolRemovalRole::Direct
    }));
    assert!(delta.removed.iter().any(|removal| {
        removal.member.txid == conflict_txid
            && removal.cause == MempoolRemovalCause::BlockConflict
            && removal.role == MempoolRemovalRole::Direct
    }));
    assert!(delta.removed.iter().any(|removal| {
        removal.member.txid == descendant_txid
            && removal.cause == MempoolRemovalCause::BlockConflict
            && removal.role == MempoolRemovalRole::Descendant
    }));
}

#[test]
fn noncommitting_attempts_return_empty_delta_and_preserve_metadata() {
    // Arrange
    let (snapshot, coinbase_txids) = sample_chainstate_snapshot(2);
    let transaction = spend_transaction(
        coinbase_txids[0],
        0,
        499_999_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    let txid = transaction_txid(&transaction).expect("txid");
    let initial_metadata = MempoolEntryMetadata::legacy_unknown();
    let mut mempool = Mempool::default();
    submit_transition(
        &mut mempool,
        &snapshot,
        transaction.clone(),
        AdmissionContext::new(initial_metadata),
    );
    let changed_metadata = MempoolEntryMetadata::new(
        crate::MempoolAcceptanceTime::Known(PolicyTime::new(123)),
        MempoolOrigin::Local,
        RelayIntent::Requested,
    );

    // Act
    let duplicate = submit_transition(
        &mut mempool,
        &snapshot,
        transaction,
        AdmissionContext::new(changed_metadata),
    );
    let orphan = submit_transition(
        &mut mempool,
        &snapshot,
        spend_transaction(
            Txid::from_byte_array([99; 32]),
            0,
            499_999_000,
            TransactionInput::SEQUENCE_FINAL,
        ),
        AdmissionContext::new(changed_metadata),
    );
    let rejected = submit_transition(
        &mut Mempool::default(),
        &snapshot,
        non_standard_spend(coinbase_txids[1]),
        AdmissionContext::new(changed_metadata),
    );

    // Assert
    assert!(matches!(
        duplicate.outcome,
        MempoolOutcome::Duplicate { .. }
    ));
    assert!(duplicate.delta.is_empty());
    assert!(matches!(orphan.outcome, MempoolOutcome::Orphaned { .. }));
    assert!(orphan.delta.is_empty());
    assert!(matches!(rejected.outcome, MempoolOutcome::Rejected { .. }));
    assert!(rejected.delta.is_empty());
    assert_eq!(
        mempool.entry(&txid).expect("original entry").metadata,
        initial_metadata
    );
}
