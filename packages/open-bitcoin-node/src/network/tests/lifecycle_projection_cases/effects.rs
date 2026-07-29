// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/node/mempool_persist.cpp
// - packages/bitcoin-knots/src/txmempool.cpp
// - packages/bitcoin-knots/src/validation.cpp

use std::{
    fs, io,
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

use open_bitcoin_mempool::{
    AdmissionContext, MempoolAcceptanceTime, MempoolEntryMetadata, MempoolOrigin, PolicyConfig,
    PolicyTime, RelayIntent,
};
use open_bitcoin_network::{LocalPeerConfig, PHASE94_MAX_PEER_QUEUED_MESSAGES, PeerId};

use super::{apply_prepared, network_with_spendable_coinbase};
use crate::network::lifecycle_effects::{
    ExactEffectLedgerCompletion, MAX_COMPLETED_PEER_EFFECTS, MAX_COMPLETED_SNAPSHOT_EFFECTS,
    MAX_PENDING_PEER_EFFECTS, MAX_PENDING_SNAPSHOT_EFFECTS, PeerEffectCapability, PeerEffectId,
    PeerEffectLedger, PeerSessionGeneration, PreparedSnapshotWrite, SnapshotEffectId,
    SnapshotEffectLedger, SnapshotIdentity,
};
use crate::network::lifecycle_projection::{
    AuthorityEpoch, LifecycleCommand, LifecycleGeneration, PeerRelayPreparationRequest,
    SnapshotPreparationRequest,
};
use crate::network::runtime_authority::{ManagedNetworkHandle, apply_lifecycle_command};
use crate::network::tests::{consensus_params, spend_transaction, verify_flags};
use crate::network::{EffectCompletion, ManagedPeerNetwork};
use crate::storage::MempoolSnapshot;
use crate::{FjallNodeStore, MemoryChainstateStore, PersistMode, StorageNamespace};

fn temp_store_path(test_name: &str) -> PathBuf {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time after unix epoch")
        .as_nanos();
    std::env::temp_dir().join(format!(
        "open-bitcoin-phase134-snapshot-{test_name}-{}-{timestamp}",
        std::process::id()
    ))
}

fn remove_dir_if_exists(path: &Path) {
    match fs::remove_dir_all(path) {
        Ok(()) => {}
        Err(error) if error.kind() == io::ErrorKind::NotFound => {}
        Err(error) => panic!("failed to remove {}: {error}", path.display()),
    }
}

fn network_fixture() -> ManagedPeerNetwork<MemoryChainstateStore> {
    ManagedPeerNetwork::new(
        MemoryChainstateStore::default(),
        LocalPeerConfig::default(),
        PolicyConfig::default(),
    )
}

fn apply_local_spend(
    network: &mut ManagedPeerNetwork<MemoryChainstateStore>,
    coinbase_txid: open_bitcoin_core::primitives::Txid,
    accepted_at: i64,
) {
    let transaction = spend_transaction(coinbase_txid, 499_999_000);
    let core = network
        .mempool
        .prepare_transaction_with_context(
            &network.chainstate,
            transaction,
            verify_flags(),
            consensus_params(),
            AdmissionContext::local(PolicyTime::new(accepted_at), RelayIntent::Requested),
        )
        .expect("transaction should prepare");
    apply_prepared(network, core);
}

fn prepare_snapshot(
    network: &mut ManagedPeerNetwork<MemoryChainstateStore>,
) -> PreparedSnapshotWrite {
    match apply_lifecycle_command(
        network,
        LifecycleCommand::PrepareSnapshot(SnapshotPreparationRequest::new()),
    )
    .expect("snapshot should prepare")
    {
        crate::network::runtime_authority::LifecycleCommandResult::SnapshotPrepared(prepared) => {
            prepared
        }
        _ => panic!("snapshot preparation returned the wrong command result"),
    }
}

mod contracts;

mod completion {
    use super::*;

    #[test]
    fn exact_snapshot_completion_clears_matching_dirty_and_pending_state() {
        // Arrange
        let (mut network, coinbase_txid) = network_with_spendable_coinbase(PolicyConfig::default());
        apply_local_spend(&mut network, coinbase_txid, 134_100);
        let dirty_generation = network.lifecycle_generation;
        let prepared = prepare_snapshot(&mut network);
        let receipt = prepared.into_parts().1.acknowledge_write();
        // Act
        let completion = apply_lifecycle_command(
            &mut network,
            LifecycleCommand::CompleteSnapshotEffect(receipt),
        )
        .expect("snapshot completion should apply");
        // Assert
        assert_eq!(network.dirty_generation, None);
        assert_eq!(dirty_generation, network.lifecycle_generation);
        assert_eq!(network.snapshot_effect_ledger.pending_len(), 0);
        assert_eq!(network.snapshot_effect_ledger.completed_len(), 1);
        assert!(matches!(
            completion,
            crate::network::runtime_authority::LifecycleCommandResult::SnapshotEffectCompleted(
                EffectCompletion::Applied
            )
        ));
    }

    #[test]
    fn public_facades_prepare_and_complete_both_families() {
        // Arrange
        let handle = ManagedNetworkHandle::from_network_fixture(network_fixture());

        // Act
        let peer_receipt = handle
            .prepare_peer_relay_effect(134_082)
            .expect("peer effect should prepare")
            .acknowledge_write();
        let snapshot_receipt = handle
            .prepare_mempool_snapshot_write()
            .expect("snapshot should prepare")
            .into_parts()
            .1
            .acknowledge_write();
        let peer_completion = handle
            .complete_peer_effect(peer_receipt)
            .expect("peer completion should dispatch");
        let snapshot_completion = handle
            .complete_snapshot_write(snapshot_receipt)
            .expect("snapshot completion should dispatch");

        // Assert
        assert_eq!(peer_completion, EffectCompletion::Applied);
        assert_eq!(snapshot_completion, EffectCompletion::Applied);
    }

    #[test]
    fn duplicate_peer_completion_precedes_stale_session_detection() {
        // Arrange
        let mut network = network_fixture();
        let capability = match apply_lifecycle_command(
            &mut network,
            LifecycleCommand::PrepareRelay(PeerRelayPreparationRequest::new(134_083)),
        )
        .expect("peer effect should prepare")
        {
            crate::network::runtime_authority::LifecycleCommandResult::RelayPrepared(
                capability,
            ) => capability,
            _ => panic!("relay preparation returned the wrong command result"),
        };
        let receipt = capability.acknowledge_write();
        let duplicate = receipt.duplicate_for_test();
        let first =
            apply_lifecycle_command(&mut network, LifecycleCommand::CompletePeerEffect(receipt))
                .expect("first peer completion should apply");
        network.peer_session_generation = network
            .peer_session_generation
            .checked_next()
            .expect("test session generation should advance");

        // Act
        let replay = apply_lifecycle_command(
            &mut network,
            LifecycleCommand::CompletePeerEffect(duplicate),
        )
        .expect("duplicate peer completion should be classified");

        // Assert
        assert!(matches!(
            first,
            crate::network::runtime_authority::LifecycleCommandResult::PeerEffectCompleted(
                EffectCompletion::Applied
            )
        ));
        assert!(matches!(
            replay,
            crate::network::runtime_authority::LifecycleCommandResult::PeerEffectCompleted(
                EffectCompletion::AlreadyApplied
            )
        ));
    }

    #[test]
    fn stale_peer_epoch_completion_records_achieved_truth() {
        // Arrange
        let mut network = network_fixture();
        let capability = match apply_lifecycle_command(
            &mut network,
            LifecycleCommand::PrepareRelay(PeerRelayPreparationRequest::new(134_085)),
        )
        .expect("peer effect should prepare")
        {
            crate::network::runtime_authority::LifecycleCommandResult::RelayPrepared(
                capability,
            ) => capability,
            _ => panic!("relay preparation returned the wrong command result"),
        };
        let receipt = capability.acknowledge_write();
        network.authority_epoch = network
            .authority_epoch
            .checked_next()
            .expect("test authority epoch should advance");

        // Act
        let completion =
            apply_lifecycle_command(&mut network, LifecycleCommand::CompletePeerEffect(receipt))
                .expect("stale peer completion should be classified");

        // Assert
        assert!(matches!(
            completion,
            crate::network::runtime_authority::LifecycleCommandResult::PeerEffectCompleted(
                EffectCompletion::AchievedButStale
            )
        ));
    }

    #[test]
    fn stale_peer_session_completion_records_achieved_truth() {
        // Arrange
        let mut network = network_fixture();
        let capability = match apply_lifecycle_command(
            &mut network,
            LifecycleCommand::PrepareRelay(PeerRelayPreparationRequest::new(134_086)),
        )
        .expect("peer effect should prepare")
        {
            crate::network::runtime_authority::LifecycleCommandResult::RelayPrepared(
                capability,
            ) => capability,
            _ => panic!("relay preparation returned the wrong command result"),
        };
        let receipt = capability.acknowledge_write();
        network.peer_session_generation = network
            .peer_session_generation
            .checked_next()
            .expect("test peer session should advance");
        let session_before = network.peer_session_generation;
        let peer_provenance_before = format!("{:?}", network.peer_manager);
        let relay_evidence_before =
            serde_json::to_value(network.relay_evidence_status()).expect("relay evidence");

        // Act
        let completion =
            apply_lifecycle_command(&mut network, LifecycleCommand::CompletePeerEffect(receipt))
                .expect("stale peer completion should be classified");

        // Assert
        assert!(matches!(
            completion,
            crate::network::runtime_authority::LifecycleCommandResult::PeerEffectCompleted(
                EffectCompletion::AchievedButStale
            )
        ));
        assert_eq!(network.peer_session_generation, session_before);
        assert_eq!(
            format!("{:?}", network.peer_manager),
            peer_provenance_before
        );
        assert_eq!(
            serde_json::to_value(network.relay_evidence_status()).expect("relay evidence"),
            relay_evidence_before
        );
        assert_eq!(network.peer_effect_ledger.pending_len(), 0);
        assert_eq!(network.peer_effect_ledger.completed_len(), 1);
    }

    #[test]
    fn stale_peer_lifecycle_completion_records_truth_without_overwriting_newer_targets() {
        // Arrange
        let (mut network, coinbase_txid) = network_with_spendable_coinbase(PolicyConfig::default());
        let capability = match apply_lifecycle_command(
            &mut network,
            LifecycleCommand::PrepareRelay(PeerRelayPreparationRequest::new(134_147)),
        )
        .expect("peer effect should prepare")
        {
            crate::network::runtime_authority::LifecycleCommandResult::RelayPrepared(
                capability,
            ) => capability,
            _ => panic!("relay preparation returned the wrong command result"),
        };
        let receipt = capability.acknowledge_write();
        apply_local_spend(&mut network, coinbase_txid, 134_104);
        let authority_epoch_before = network.authority_epoch;
        let lifecycle_generation_before = network.lifecycle_generation;
        let dirty_generation_before = network.dirty_generation;
        let unbroadcast_before = network.unbroadcast_members().clone();
        let peer_session_before = network.peer_session_generation;
        let peer_provenance_before = format!("{:?}", network.peer_manager);
        let relay_evidence_before =
            serde_json::to_value(network.relay_evidence_status()).expect("relay evidence");

        // Act
        let completion =
            apply_lifecycle_command(&mut network, LifecycleCommand::CompletePeerEffect(receipt))
                .expect("stale peer completion should be classified");

        // Assert
        assert!(matches!(
            completion,
            crate::network::runtime_authority::LifecycleCommandResult::PeerEffectCompleted(
                EffectCompletion::AchievedButStale
            )
        ));
        assert_eq!(network.authority_epoch, authority_epoch_before);
        assert_eq!(network.lifecycle_generation, lifecycle_generation_before);
        assert_eq!(network.dirty_generation, dirty_generation_before);
        assert_eq!(network.unbroadcast_members(), &unbroadcast_before);
        assert_eq!(network.peer_session_generation, peer_session_before);
        assert_eq!(
            format!("{:?}", network.peer_manager),
            peer_provenance_before
        );
        assert_eq!(
            serde_json::to_value(network.relay_evidence_status()).expect("relay evidence"),
            relay_evidence_before
        );
        assert_eq!(network.peer_effect_ledger.pending_len(), 0);
        assert_eq!(network.peer_effect_ledger.completed_len(), 1);
    }

    #[test]
    fn stale_snapshot_completion_records_truth_without_clearing_newer_dirty_state() {
        // Arrange
        let (mut network, coinbase_txid) = network_with_spendable_coinbase(PolicyConfig::default());
        let prepared = prepare_snapshot(&mut network);
        let receipt = prepared.into_parts().1.acknowledge_write();
        apply_local_spend(&mut network, coinbase_txid, 134_101);
        let authority_epoch_before = network.authority_epoch;
        let lifecycle_generation_before = network.lifecycle_generation;
        let dirty_generation_before = network.dirty_generation;
        let unbroadcast_before = network.unbroadcast_members().clone();
        let peer_provenance_before = format!("{:?}", network.peer_manager);
        let relay_evidence_before =
            serde_json::to_value(network.relay_evidence_status()).expect("relay evidence");

        // Act
        let completion = apply_lifecycle_command(
            &mut network,
            LifecycleCommand::CompleteSnapshotEffect(receipt),
        )
        .expect("stale snapshot completion should be classified");

        // Assert
        assert!(matches!(
            completion,
            crate::network::runtime_authority::LifecycleCommandResult::SnapshotEffectCompleted(
                EffectCompletion::AchievedButStale
            )
        ));
        assert_eq!(network.authority_epoch, authority_epoch_before);
        assert_eq!(network.lifecycle_generation, lifecycle_generation_before);
        assert_eq!(network.dirty_generation, dirty_generation_before);
        assert_eq!(network.unbroadcast_members(), &unbroadcast_before);
        assert_eq!(
            format!("{:?}", network.peer_manager),
            peer_provenance_before
        );
        assert_eq!(
            serde_json::to_value(network.relay_evidence_status()).expect("relay evidence"),
            relay_evidence_before
        );
        assert_eq!(network.snapshot_effect_ledger.pending_len(), 0);
        assert_eq!(network.snapshot_effect_ledger.completed_len(), 1);
    }

    #[test]
    fn pending_caps_fail_closed_through_the_shared_dispatcher() {
        // Arrange
        let mut network = network_fixture();
        for peer_id in 0..MAX_PENDING_PEER_EFFECTS {
            apply_lifecycle_command(
                &mut network,
                LifecycleCommand::PrepareRelay(PeerRelayPreparationRequest::new(peer_id as PeerId)),
            )
            .expect("peer effects at the cap should prepare");
        }
        apply_lifecycle_command(
            &mut network,
            LifecycleCommand::PrepareSnapshot(SnapshotPreparationRequest::new()),
        )
        .expect("one snapshot should prepare");
        let snapshot_state_before_overflow = format!("{:?}", network.snapshot_effect_ledger);

        // Act
        let peer_overflow = apply_lifecycle_command(
            &mut network,
            LifecycleCommand::PrepareRelay(PeerRelayPreparationRequest::new(134_084)),
        );
        let snapshot_overflow = apply_lifecycle_command(
            &mut network,
            LifecycleCommand::PrepareSnapshot(SnapshotPreparationRequest::new()),
        );

        // Assert
        assert!(peer_overflow.is_err());
        assert!(snapshot_overflow.is_err());
        assert_eq!(
            format!("{:?}", network.snapshot_effect_ledger),
            snapshot_state_before_overflow
        );
    }

    #[test]
    fn duplicate_snapshot_completion_precedes_stale_generation_detection() {
        // Arrange
        let (mut network, coinbase_txid) = network_with_spendable_coinbase(PolicyConfig::default());
        let prepared = prepare_snapshot(&mut network);
        let receipt = prepared.into_parts().1.acknowledge_write();
        let duplicate = receipt.duplicate_for_test();
        apply_lifecycle_command(
            &mut network,
            LifecycleCommand::CompleteSnapshotEffect(receipt),
        )
        .expect("first snapshot completion should apply");
        apply_local_spend(&mut network, coinbase_txid, 134_102);
        let state_before_replay = format!("{network:?}");
        // Act
        let replay = apply_lifecycle_command(
            &mut network,
            LifecycleCommand::CompleteSnapshotEffect(duplicate),
        )
        .expect("duplicate snapshot completion should be classified");
        // Assert
        assert!(matches!(
            replay,
            crate::network::runtime_authority::LifecycleCommandResult::SnapshotEffectCompleted(
                EffectCompletion::AlreadyApplied
            )
        ));
        assert_eq!(format!("{network:?}"), state_before_replay);
    }

    #[test]
    fn public_snapshot_facades_preserve_newer_authority_after_stale_persistence() {
        // Arrange
        let path = temp_store_path("stale-public-facades");
        remove_dir_if_exists(&path);
        let store = FjallNodeStore::open(&path).expect("open store");
        let (network, coinbase_txid) = network_with_spendable_coinbase(PolicyConfig::default());
        let handle = ManagedNetworkHandle::from_network_fixture(network);
        let prepared_old = handle
            .prepare_mempool_snapshot_write()
            .expect("old snapshot should prepare");
        let old_receipt = store
            .execute_prepared_mempool_snapshot_write(prepared_old, PersistMode::Sync)
            .expect("old snapshot should persist");
        let old_duplicate = old_receipt.duplicate_for_test();
        let transaction = spend_transaction(coinbase_txid, 499_999_000);
        handle
            .submit_local_transaction_outcome_at(
                transaction,
                verify_flags(),
                consensus_params(),
                134_103,
                RelayIntent::Requested,
            )
            .expect("newer transaction should apply");
        // Act
        let stale_completion = handle
            .complete_snapshot_write(old_receipt)
            .expect("stale completion should dispatch");
        let prepared_current = handle
            .prepare_mempool_snapshot_write()
            .expect("current snapshot should prepare");
        let current_receipt = store
            .execute_prepared_mempool_snapshot_write(prepared_current, PersistMode::Sync)
            .expect("current snapshot should persist");
        let current_completion = handle
            .complete_snapshot_write(current_receipt)
            .expect("current completion should dispatch");
        let state_before_duplicate = handle.mempool_info().expect("mempool info");
        let duplicate_completion = handle
            .complete_snapshot_write(old_duplicate)
            .expect("duplicate completion should dispatch");
        // Assert
        assert_eq!(stale_completion, EffectCompletion::AchievedButStale);
        assert_eq!(current_completion, EffectCompletion::Applied);
        assert_eq!(duplicate_completion, EffectCompletion::AlreadyApplied);
        assert_eq!(
            handle.mempool_info().expect("mempool info after duplicate"),
            state_before_duplicate
        );
        assert_eq!(state_before_duplicate.transaction_count, 1);
        assert_eq!(
            store
                .load_mempool_snapshot()
                .expect("load current persisted snapshot")
                .expect("current snapshot should exist")
                .records
                .len(),
            1
        );
        assert!(
            handle.prepare_mempool_snapshot_write().is_ok(),
            "successful current completion should release the pending slot"
        );

        remove_dir_if_exists(&path);
    }

    #[test]
    fn snapshot_executor_encoding_failure_keeps_the_effect_pending() {
        // Arrange
        let path = temp_store_path("encode-failure");
        remove_dir_if_exists(&path);
        let store = FjallNodeStore::open(&path).expect("open store");
        let (mut network, coinbase_txid) = network_with_spendable_coinbase(PolicyConfig::default());
        let transaction = spend_transaction(coinbase_txid, 499_999_000);
        let invalid_metadata = MempoolEntryMetadata::new(
            MempoolAcceptanceTime::LegacyUnknown,
            MempoolOrigin::Local,
            RelayIntent::Requested,
        );
        let core = network
            .mempool
            .prepare_transaction_with_context(
                &network.chainstate,
                transaction,
                verify_flags(),
                consensus_params(),
                AdmissionContext::new(invalid_metadata),
            )
            .expect("invalid persistence metadata is still admissible in memory");
        apply_prepared(&mut network, core);
        let handle = ManagedNetworkHandle::from_network_fixture(network);
        let prepared = handle
            .prepare_mempool_snapshot_write()
            .expect("snapshot should prepare");

        // Act
        let result = store.execute_prepared_mempool_snapshot_write(prepared, PersistMode::Sync);

        // Assert
        assert!(matches!(
            result,
            Err(crate::StorageError::Corruption {
                namespace: StorageNamespace::Mempool,
                ..
            })
        ));
        assert!(
            handle.prepare_mempool_snapshot_write().is_err(),
            "encoding failure must not complete the pending snapshot"
        );
        assert_eq!(
            store
                .load_mempool_snapshot()
                .expect("load after encode failure"),
            None
        );

        remove_dir_if_exists(&path);
    }
}
