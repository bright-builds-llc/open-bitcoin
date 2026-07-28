// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/node/mempool_persist.cpp
// - packages/bitcoin-knots/src/txmempool.cpp
// - packages/bitcoin-knots/src/validation.cpp

use open_bitcoin_network::{PHASE94_MAX_PEER_QUEUED_MESSAGES, PeerId};

use crate::MemoryChainstateStore;
use crate::network::lifecycle_effects::{
    MAX_COMPLETED_PEER_EFFECTS, MAX_COMPLETED_SNAPSHOT_EFFECTS, MAX_PENDING_PEER_EFFECTS,
    MAX_PENDING_SNAPSHOT_EFFECTS, PeerEffectCapability, PeerEffectId, PeerEffectLedger,
    PeerSessionGeneration, PreparedSnapshotWrite, SnapshotEffectId, SnapshotEffectLedger,
    SnapshotIdentity,
};
use crate::network::lifecycle_projection::{
    AuthorityEpoch, LifecycleCommand, LifecycleGeneration, PeerRelayPreparationRequest,
    SnapshotPreparationRequest,
};
use crate::network::runtime_authority::{ManagedNetworkHandle, apply_lifecycle_command};
use crate::network::{EffectCompletion, ManagedPeerNetwork};
use crate::storage::MempoolSnapshot;
use open_bitcoin_mempool::PolicyConfig;
use open_bitcoin_network::LocalPeerConfig;

fn network_fixture() -> ManagedPeerNetwork<MemoryChainstateStore> {
    ManagedPeerNetwork::new(
        MemoryChainstateStore::default(),
        LocalPeerConfig::default(),
        PolicyConfig::default(),
    )
}

mod contracts {
    use super::*;

    #[test]
    fn family_caps_match_the_resolved_peer_and_snapshot_bounds() {
        // Arrange
        let peer_cap = PHASE94_MAX_PEER_QUEUED_MESSAGES;

        // Act
        let actual = [
            MAX_PENDING_PEER_EFFECTS,
            MAX_COMPLETED_PEER_EFFECTS,
            MAX_PENDING_SNAPSHOT_EFFECTS,
            MAX_COMPLETED_SNAPSHOT_EFFECTS,
        ];

        // Assert
        assert_eq!(actual, [peer_cap, peer_cap, 1, 2]);
    }

    #[test]
    fn peer_capability_and_receipt_bind_every_peer_identity_dimension() {
        // Arrange
        let authority_epoch = AuthorityEpoch::INITIAL;
        let generation = LifecycleGeneration::INITIAL;
        let effect_id = PeerEffectId::new(41);
        let peer_id: PeerId = 134_081;
        let session_generation = PeerSessionGeneration::new(7);
        let capability = PeerEffectCapability::new(
            authority_epoch,
            generation,
            effect_id,
            peer_id,
            session_generation,
        );

        // Act
        let receipt = capability.acknowledge_write();

        // Assert
        assert_eq!(receipt.authority_epoch(), authority_epoch);
        assert_eq!(receipt.lifecycle_generation(), generation);
        assert_eq!(receipt.effect_id(), effect_id);
        assert_eq!(receipt.peer_id(), peer_id);
        assert_eq!(receipt.peer_session_generation(), session_generation);
    }

    #[test]
    fn snapshot_capability_and_receipt_bind_every_snapshot_identity_dimension() {
        // Arrange
        let authority_epoch = AuthorityEpoch::INITIAL;
        let generation = LifecycleGeneration::INITIAL;
        let effect_id = SnapshotEffectId::new(42);
        let snapshot_identity = SnapshotIdentity::new(9);
        let prepared = PreparedSnapshotWrite::new(
            authority_epoch,
            generation,
            effect_id,
            snapshot_identity,
            MempoolSnapshot::default(),
        );

        // Act
        let (snapshot, capability) = prepared.into_parts();
        let receipt = capability.acknowledge_write();

        // Assert
        assert!(snapshot.records.is_empty());
        assert_eq!(receipt.authority_epoch(), authority_epoch);
        assert_eq!(receipt.persistence_generation(), generation);
        assert_eq!(receipt.effect_id(), effect_id);
        assert_eq!(receipt.snapshot_identity(), snapshot_identity);
    }

    #[test]
    fn peer_completed_ledger_evicts_the_oldest_id_at_cap_plus_one() {
        // Arrange
        let mut ledger = PeerEffectLedger::default();
        for raw_id in 0..MAX_COMPLETED_PEER_EFFECTS {
            ledger.record_completed(PeerEffectId::new(raw_id as u64));
        }
        let oldest = PeerEffectId::new(0);
        let newest = PeerEffectId::new(MAX_COMPLETED_PEER_EFFECTS as u64);

        // Act
        ledger.record_completed(newest);

        // Assert
        assert_eq!(ledger.completed_len(), MAX_COMPLETED_PEER_EFFECTS);
        assert!(!ledger.is_completed(oldest));
        assert!(ledger.is_completed(newest));
    }

    #[test]
    fn snapshot_completed_ledger_evicts_the_oldest_id_at_cap_plus_one() {
        // Arrange
        let mut ledger = SnapshotEffectLedger::default();
        for raw_id in 0..MAX_COMPLETED_SNAPSHOT_EFFECTS {
            ledger.record_completed(SnapshotEffectId::new(raw_id as u64));
        }
        let oldest = SnapshotEffectId::new(0);
        let newest = SnapshotEffectId::new(MAX_COMPLETED_SNAPSHOT_EFFECTS as u64);

        // Act
        ledger.record_completed(newest);

        // Assert
        assert_eq!(ledger.completed_len(), MAX_COMPLETED_SNAPSHOT_EFFECTS);
        assert!(!ledger.is_completed(oldest));
        assert!(ledger.is_completed(newest));
    }

    #[test]
    fn pending_ledgers_fail_closed_at_exact_family_caps() {
        // Arrange
        let mut peer_ledger = PeerEffectLedger::default();
        for raw_id in 0..MAX_PENDING_PEER_EFFECTS {
            assert!(
                peer_ledger
                    .try_reserve(PeerEffectId::new(raw_id as u64))
                    .is_ok()
            );
        }
        let mut snapshot_ledger = SnapshotEffectLedger::default();
        snapshot_ledger
            .try_reserve(SnapshotEffectId::new(0))
            .expect("one snapshot may be pending");

        // Act
        let peer_overflow =
            peer_ledger.try_reserve(PeerEffectId::new(MAX_PENDING_PEER_EFFECTS as u64));
        let snapshot_overflow = snapshot_ledger.try_reserve(SnapshotEffectId::new(1));

        // Assert
        assert!(peer_overflow.is_err());
        assert!(snapshot_overflow.is_err());
        assert_eq!(peer_ledger.pending_len(), MAX_PENDING_PEER_EFFECTS);
        assert_eq!(snapshot_ledger.pending_len(), MAX_PENDING_SNAPSHOT_EFFECTS);
    }
}

mod completion {
    use super::*;

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
    fn stale_snapshot_completion_records_truth_without_clearing_newer_dirty_state() {
        // Arrange
        let mut network = network_fixture();
        let prepared = match apply_lifecycle_command(
            &mut network,
            LifecycleCommand::PrepareSnapshot(SnapshotPreparationRequest::new()),
        )
        .expect("snapshot should prepare")
        {
            crate::network::runtime_authority::LifecycleCommandResult::SnapshotPrepared(
                prepared,
            ) => prepared,
            _ => panic!("snapshot preparation returned the wrong command result"),
        };
        let receipt = prepared.into_parts().1.acknowledge_write();
        let newer_generation = network
            .lifecycle_generation
            .checked_next()
            .expect("test lifecycle generation should advance");
        network.lifecycle_generation = newer_generation;
        network.dirty_generation = Some(newer_generation);

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
        assert_eq!(network.dirty_generation, Some(newer_generation));
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
    }

    #[test]
    fn duplicate_snapshot_completion_precedes_stale_generation_detection() {
        // Arrange
        let mut network = network_fixture();
        let prepared = match apply_lifecycle_command(
            &mut network,
            LifecycleCommand::PrepareSnapshot(SnapshotPreparationRequest::new()),
        )
        .expect("snapshot should prepare")
        {
            crate::network::runtime_authority::LifecycleCommandResult::SnapshotPrepared(
                prepared,
            ) => prepared,
            _ => panic!("snapshot preparation returned the wrong command result"),
        };
        let receipt = prepared.into_parts().1.acknowledge_write();
        let duplicate = receipt.duplicate_for_test();
        apply_lifecycle_command(
            &mut network,
            LifecycleCommand::CompleteSnapshotEffect(receipt),
        )
        .expect("first snapshot completion should apply");
        network.lifecycle_generation = network
            .lifecycle_generation
            .checked_next()
            .expect("test lifecycle generation should advance");

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
    }
}
