// Parity breadcrumbs:
// - packages/bitcoin-knots/src/kernel/mempool_entry.h
// - packages/bitcoin-knots/src/node/mempool_persist.cpp

use open_bitcoin_consensus::{ConsensusParams, ScriptVerifyFlags, transaction_txid};
use open_bitcoin_primitives::{Transaction, TransactionInput};

use super::{sample_chainstate_snapshot, spend_transaction};
use crate::{
    AccountedMempoolMemory, AdmissionContext, BlockLifecycleContext, Mempool,
    MempoolAcceptanceTime, MempoolCapacity, MempoolEntryMetadata, MempoolOrigin, PolicyTime,
    PressureDecisionContext, RelayIntent, ReorgLifecycleContext,
};

fn metadata(
    accepted_at: i64,
    origin: MempoolOrigin,
    relay_intent: RelayIntent,
) -> MempoolEntryMetadata {
    MempoolEntryMetadata::new(
        MempoolAcceptanceTime::Known(PolicyTime::new(accepted_at)),
        origin,
        relay_intent,
    )
}

fn admit_with_metadata(
    mempool: &mut Mempool,
    snapshot: &open_bitcoin_chainstate::ChainstateSnapshot,
    transaction: Transaction,
    metadata: MempoolEntryMetadata,
) -> crate::AdmissionResult {
    mempool
        .accept_transaction_with_context(
            transaction,
            snapshot,
            ScriptVerifyFlags::P2SH
                | ScriptVerifyFlags::CHECKLOCKTIMEVERIFY
                | ScriptVerifyFlags::CHECKSEQUENCEVERIFY,
            ConsensusParams {
                coinbase_maturity: 1,
                ..ConsensusParams::default()
            },
            AdmissionContext::new(metadata),
        )
        .expect("context-aware admission")
}

#[test]
fn local_admission_preserves_supplied_metadata() {
    // Arrange
    let (snapshot, coinbase_txids) = sample_chainstate_snapshot(2);
    let expected = metadata(1_750_000_001, MempoolOrigin::Local, RelayIntent::Requested);
    let transaction = spend_transaction(
        coinbase_txids[0],
        0,
        499_999_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    let mut mempool = Mempool::default();

    // Act
    let result = admit_with_metadata(&mut mempool, &snapshot, transaction, expected);

    // Assert
    assert_eq!(
        mempool.entry(&result.accepted).expect("entry").metadata,
        expected
    );
}

#[test]
fn peer_admission_preserves_supplied_metadata() {
    // Arrange
    let (snapshot, coinbase_txids) = sample_chainstate_snapshot(2);
    let expected = metadata(
        1_750_000_002,
        MempoolOrigin::Peer,
        RelayIntent::NotRequested,
    );
    let transaction = spend_transaction(
        coinbase_txids[0],
        0,
        499_999_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    let mut mempool = Mempool::default();

    // Act
    let result = admit_with_metadata(&mut mempool, &snapshot, transaction, expected);

    // Assert
    assert_eq!(
        mempool.entry(&result.accepted).expect("entry").metadata,
        expected
    );
}

#[test]
fn reorg_admission_preserves_supplied_metadata() {
    // Arrange
    let (snapshot, coinbase_txids) = sample_chainstate_snapshot(2);
    let expected = metadata(
        1_750_000_003,
        MempoolOrigin::Reorg,
        RelayIntent::NotRequested,
    );
    let transaction = spend_transaction(
        coinbase_txids[0],
        0,
        499_999_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    let mut mempool = Mempool::default();

    // Act
    let result = admit_with_metadata(&mut mempool, &snapshot, transaction, expected);

    // Assert
    assert_eq!(
        mempool.entry(&result.accepted).expect("entry").metadata,
        expected
    );
}

#[test]
fn duplicate_attempt_does_not_rewrite_canonical_metadata() {
    // Arrange
    let (snapshot, coinbase_txids) = sample_chainstate_snapshot(2);
    let transaction = spend_transaction(
        coinbase_txids[0],
        0,
        499_999_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    let original = metadata(1_750_000_004, MempoolOrigin::Local, RelayIntent::Requested);
    let replacement = metadata(
        1_750_000_099,
        MempoolOrigin::Peer,
        RelayIntent::NotRequested,
    );
    let mut mempool = Mempool::default();
    let accepted = admit_with_metadata(&mut mempool, &snapshot, transaction.clone(), original);

    // Act
    let duplicate = mempool.accept_transaction_with_context(
        transaction,
        &snapshot,
        ScriptVerifyFlags::P2SH,
        ConsensusParams::default(),
        AdmissionContext::new(replacement),
    );

    // Assert
    assert!(duplicate.is_err());
    assert_eq!(
        mempool.entry(&accepted.accepted).expect("entry").metadata,
        original
    );
}

#[test]
fn known_recovery_preserves_original_metadata() {
    // Arrange
    let (snapshot, coinbase_txids) = sample_chainstate_snapshot(2);
    let recovered = metadata(1_650_000_005, MempoolOrigin::Local, RelayIntent::Requested);
    let transaction = spend_transaction(
        coinbase_txids[0],
        0,
        499_999_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    let mut mempool = Mempool::default();

    // Act
    let result = admit_with_metadata(&mut mempool, &snapshot, transaction, recovered);

    // Assert
    assert_eq!(
        mempool.entry(&result.accepted).expect("entry").metadata,
        recovered
    );
}

#[test]
#[allow(deprecated)]
fn legacy_admission_adapter_assigns_fail_closed_metadata() {
    // Arrange
    let (snapshot, coinbase_txids) = sample_chainstate_snapshot(2);
    let transaction = spend_transaction(
        coinbase_txids[0],
        0,
        499_999_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    let mut mempool = Mempool::default();

    // Act
    let result = mempool
        .accept_transaction(
            transaction,
            &snapshot,
            ScriptVerifyFlags::P2SH
                | ScriptVerifyFlags::CHECKLOCKTIMEVERIFY
                | ScriptVerifyFlags::CHECKSEQUENCEVERIFY,
            ConsensusParams {
                coinbase_maturity: 1,
                ..ConsensusParams::default()
            },
        )
        .expect("legacy adapter admission");

    // Assert
    assert_eq!(
        mempool.entry(&result.accepted).expect("entry").metadata,
        MempoolEntryMetadata::legacy_unknown()
    );
}

#[test]
#[allow(deprecated)]
fn legacy_outcome_adapter_assigns_fail_closed_metadata() {
    // Arrange
    let (snapshot, coinbase_txids) = sample_chainstate_snapshot(2);
    let transaction = spend_transaction(
        coinbase_txids[0],
        0,
        499_999_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    let txid = transaction_txid(&transaction).expect("txid");
    let mut mempool = Mempool::default();

    // Act
    mempool
        .accept_transaction_outcome(
            transaction,
            &snapshot,
            ScriptVerifyFlags::P2SH
                | ScriptVerifyFlags::CHECKLOCKTIMEVERIFY
                | ScriptVerifyFlags::CHECKSEQUENCEVERIFY,
            ConsensusParams {
                coinbase_maturity: 1,
                ..ConsensusParams::default()
            },
        )
        .expect("legacy adapter outcome");

    // Assert
    assert_eq!(
        mempool.entry(&txid).expect("entry").metadata,
        MempoolEntryMetadata::legacy_unknown()
    );
}

#[test]
fn legacy_unknown_metadata_is_not_retry_eligible() {
    // Arrange
    let metadata = MempoolEntryMetadata::legacy_unknown();

    // Act
    let is_retry_eligible = metadata.is_retry_eligible(true);

    // Assert
    assert!(!is_retry_eligible);
    assert_eq!(metadata.accepted_at, MempoolAcceptanceTime::LegacyUnknown);
    assert_eq!(metadata.origin, MempoolOrigin::RecoveryUnknown);
    assert_eq!(metadata.relay_intent, RelayIntent::NotRequested);
}

#[test]
fn retry_eligibility_requires_local_requested_current_membership() {
    // Arrange
    let local_requested = metadata(1, MempoolOrigin::Local, RelayIntent::Requested);
    let peer_requested = metadata(1, MempoolOrigin::Peer, RelayIntent::Requested);
    let local_not_requested = metadata(1, MempoolOrigin::Local, RelayIntent::NotRequested);

    // Act
    let current_local_requested = local_requested.is_retry_eligible(true);
    let absent_local_requested = local_requested.is_retry_eligible(false);

    // Assert
    assert!(current_local_requested);
    assert!(!absent_local_requested);
    assert!(!peer_requested.is_retry_eligible(true));
    assert!(!local_not_requested.is_retry_eligible(true));
}

#[test]
fn operation_contexts_preserve_all_explicit_inputs() {
    // Arrange
    let pressure_time = PolicyTime::new(1_750_000_010);
    let block_time = PolicyTime::new(1_750_000_011);
    let reorg_time = PolicyTime::new(1_750_000_012);

    // Act
    let pressure = PressureDecisionContext::new(
        pressure_time,
        AccountedMempoolMemory::new(12_345),
        MempoolCapacity::new(300_000_000),
    );
    let block = BlockLifecycleContext::new(block_time, 900_000);
    let reorg = ReorgLifecycleContext::new(reorg_time);

    // Assert
    assert_eq!(pressure.observed_at, pressure_time);
    assert_eq!(pressure.usage, AccountedMempoolMemory::new(12_345));
    assert_eq!(pressure.capacity, MempoolCapacity::new(300_000_000));
    assert_eq!(pressure.observed_at.unix_seconds(), 1_750_000_010);
    assert_eq!(block.connected_at, block_time);
    assert_eq!(block.height, 900_000);
    assert_eq!(reorg.occurred_at, reorg_time);
}

#[test]
fn admission_context_constructors_map_trusted_source_facts() {
    // Arrange
    let peer_time = PolicyTime::from_unix_seconds(std::hint::black_box(42));
    let local_time = PolicyTime::from_unix_seconds(std::hint::black_box(50));
    let reorg_time = PolicyTime::from_unix_seconds(std::hint::black_box(80));
    let recovery_metadata = MempoolEntryMetadata::new(
        MempoolAcceptanceTime::Known(local_time),
        MempoolOrigin::Local,
        RelayIntent::Requested,
    );

    // Act
    let peer = AdmissionContext::peer(peer_time);
    let local_requested = AdmissionContext::local(local_time, RelayIntent::Requested);
    let local_not_requested = AdmissionContext::local(local_time, RelayIntent::NotRequested);
    let reorg = AdmissionContext::reorg(reorg_time);
    let recovery = AdmissionContext::recovery(std::hint::black_box(recovery_metadata));

    // Assert
    assert_eq!(
        peer.metadata.accepted_at,
        MempoolAcceptanceTime::Known(peer_time)
    );
    assert_eq!(peer.metadata.origin, MempoolOrigin::Peer);
    assert_eq!(peer.metadata.relay_intent, RelayIntent::NotRequested);
    assert_eq!(
        local_requested.metadata.accepted_at,
        MempoolAcceptanceTime::Known(local_time)
    );
    assert_eq!(local_requested.metadata.origin, MempoolOrigin::Local);
    assert_eq!(
        local_requested.metadata.relay_intent,
        RelayIntent::Requested
    );
    assert_eq!(
        local_not_requested.metadata.relay_intent,
        RelayIntent::NotRequested
    );
    assert_eq!(
        reorg.metadata.accepted_at,
        MempoolAcceptanceTime::Known(reorg_time)
    );
    assert_eq!(reorg.metadata.origin, MempoolOrigin::Reorg);
    assert_eq!(reorg.metadata.relay_intent, RelayIntent::NotRequested);
    assert_eq!(recovery.metadata, recovery_metadata);
    assert_eq!(peer_time.unix_seconds(), 42);
    assert_eq!(local_time.unix_seconds(), 50);
}
