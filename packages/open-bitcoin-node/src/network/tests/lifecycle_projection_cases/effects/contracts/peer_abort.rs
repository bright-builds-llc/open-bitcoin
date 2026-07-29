// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/node/mempool_persist.cpp
// - packages/bitcoin-knots/src/txmempool.cpp
// - packages/bitcoin-knots/src/validation.cpp

use super::*;
use crate::network::EffectAbort;

fn capability(
    authority_epoch: AuthorityEpoch,
    lifecycle_generation: LifecycleGeneration,
    effect_id: u64,
    peer_id: PeerId,
    peer_session_generation: PeerSessionGeneration,
) -> PeerEffectCapability {
    PeerEffectCapability::new(
        authority_epoch,
        lifecycle_generation,
        PeerEffectId::new(effect_id),
        peer_id,
        peer_session_generation,
    )
}

#[test]
fn exact_peer_abort_releases_pending_without_recording_completion() {
    // Arrange
    let mut ledger = PeerEffectLedger::default();
    let authority_epoch = AuthorityEpoch::INITIAL
        .checked_next()
        .expect("test authority epoch should advance");
    let exact = capability(
        authority_epoch,
        LifecycleGeneration::INITIAL,
        16,
        134_160,
        PeerSessionGeneration::new(3),
    );
    ledger
        .try_reserve_for_test(capability(
            authority_epoch,
            LifecycleGeneration::INITIAL,
            16,
            134_160,
            PeerSessionGeneration::new(3),
        ))
        .expect("exact peer binding should reserve");

    // Act
    let abort = ledger.abort_exact(&exact);

    // Assert
    assert_eq!(abort, EffectAbort::Aborted);
    assert_eq!(ledger.pending_len(), 0);
    assert_eq!(ledger.completed_len(), 0);
}

#[test]
fn exact_peer_abort_releases_after_lifecycle_and_target_session_advance() {
    // Arrange
    let mut network = network_fixture();
    let target_peer = 134_161;
    network
        .connect_outbound_peer(target_peer, 1)
        .expect("target peer should connect");
    let exact = match apply_lifecycle_command(
        &mut network,
        LifecycleCommand::PrepareRelay(PeerRelayPreparationRequest::new(target_peer)),
    )
    .expect("peer effect should prepare")
    {
        crate::network::runtime_authority::LifecycleCommandResult::RelayPrepared(capability) => {
            capability
        }
        _ => panic!("relay preparation returned the wrong command result"),
    };
    network.lifecycle_generation = network
        .lifecycle_generation
        .checked_next()
        .expect("test lifecycle generation should advance");
    network
        .disconnect_peer(target_peer)
        .expect("target peer should disconnect");
    network
        .connect_outbound_peer(target_peer, 2)
        .expect("target peer should reconnect");
    let lifecycle_generation_after = network.lifecycle_generation;
    let peer_session_after = network.peer_session_generation(target_peer);

    // Act
    let abort = apply_lifecycle_command(&mut network, LifecycleCommand::AbortPeerEffect(exact))
        .expect("exact pre-achievement abort should dispatch");

    // Assert
    assert!(matches!(
        abort,
        crate::network::runtime_authority::LifecycleCommandResult::PeerEffectAborted(
            EffectAbort::Aborted
        )
    ));
    assert_eq!(network.lifecycle_generation, lifecycle_generation_after);
    assert_eq!(
        network.peer_session_generation(target_peer),
        peer_session_after
    );
    assert_eq!(network.peer_effect_ledger.pending_len(), 0);
    assert_eq!(network.peer_effect_ledger.completed_len(), 0);
}

#[test]
fn foreign_and_immutable_mismatched_peer_aborts_leave_accounting_unchanged() {
    // Arrange
    let mut network = network_fixture();
    let authority_epoch = network.authority_epoch;
    let lifecycle_generation = network.lifecycle_generation;
    let effect_id = 17;
    let peer_id = 134_162;
    let session_generation = PeerSessionGeneration::new(4);
    network
        .peer_effect_ledger
        .try_reserve_for_test(capability(
            authority_epoch,
            lifecycle_generation,
            effect_id,
            peer_id,
            session_generation,
        ))
        .expect("exact peer binding should reserve");
    let mismatches = [
        capability(
            authority_epoch
                .checked_next()
                .expect("test authority epoch should advance"),
            lifecycle_generation,
            effect_id,
            peer_id,
            session_generation,
        ),
        capability(
            authority_epoch,
            lifecycle_generation
                .checked_next()
                .expect("test lifecycle generation should advance"),
            effect_id,
            peer_id,
            session_generation,
        ),
        capability(
            authority_epoch,
            lifecycle_generation,
            effect_id + 1,
            peer_id,
            session_generation,
        ),
        capability(
            authority_epoch,
            lifecycle_generation,
            effect_id,
            peer_id + 1,
            session_generation,
        ),
        capability(
            authority_epoch,
            lifecycle_generation,
            effect_id,
            peer_id,
            session_generation
                .checked_next()
                .expect("test session generation should advance"),
        ),
    ];
    let state_before = format!("{network:?}");

    // Act
    for mismatch in mismatches {
        let abort =
            apply_lifecycle_command(&mut network, LifecycleCommand::AbortPeerEffect(mismatch))
                .expect("binding mismatch should return a typed no-op");

        // Assert
        assert!(matches!(
            abort,
            crate::network::runtime_authority::LifecycleCommandResult::PeerEffectAborted(
                EffectAbort::NotPending
            )
        ));
        assert_eq!(format!("{network:?}"), state_before);
    }
}

#[test]
fn replayed_peer_abort_is_a_typed_no_op() {
    // Arrange
    let mut network = network_fixture();
    let authority_epoch = network.authority_epoch;
    let lifecycle_generation = network.lifecycle_generation;
    let peer_id = 134_163;
    let session_generation = PeerSessionGeneration::new(5);
    network
        .peer_effect_ledger
        .try_reserve_for_test(capability(
            authority_epoch,
            lifecycle_generation,
            18,
            peer_id,
            session_generation,
        ))
        .expect("exact peer binding should reserve");
    let first = capability(
        authority_epoch,
        lifecycle_generation,
        18,
        peer_id,
        session_generation,
    );
    let replay = capability(
        authority_epoch,
        lifecycle_generation,
        18,
        peer_id,
        session_generation,
    );

    // Act
    let first_abort =
        apply_lifecycle_command(&mut network, LifecycleCommand::AbortPeerEffect(first))
            .expect("first abort should dispatch");
    let replay_abort =
        apply_lifecycle_command(&mut network, LifecycleCommand::AbortPeerEffect(replay))
            .expect("replayed abort should dispatch");

    // Assert
    assert!(matches!(
        first_abort,
        crate::network::runtime_authority::LifecycleCommandResult::PeerEffectAborted(
            EffectAbort::Aborted
        )
    ));
    assert!(matches!(
        replay_abort,
        crate::network::runtime_authority::LifecycleCommandResult::PeerEffectAborted(
            EffectAbort::NotPending
        )
    ));
    assert_eq!(network.peer_effect_ledger.pending_len(), 0);
    assert_eq!(network.peer_effect_ledger.completed_len(), 0);
}

#[test]
fn exact_peer_abort_restores_bounded_pending_capacity() {
    // Arrange
    let mut network = network_fixture();
    let mut capabilities = Vec::with_capacity(MAX_PENDING_PEER_EFFECTS);
    for peer_id in 0..MAX_PENDING_PEER_EFFECTS {
        let prepared = apply_lifecycle_command(
            &mut network,
            LifecycleCommand::PrepareRelay(PeerRelayPreparationRequest::new(peer_id as PeerId)),
        )
        .expect("peer effects up to the cap should prepare");
        let crate::network::runtime_authority::LifecycleCommandResult::RelayPrepared(capability) =
            prepared
        else {
            panic!("relay preparation returned the wrong command result");
        };
        capabilities.push(capability);
    }
    assert!(
        apply_lifecycle_command(
            &mut network,
            LifecycleCommand::PrepareRelay(PeerRelayPreparationRequest::new(134_164)),
        )
        .is_err()
    );
    let exact = capabilities
        .pop()
        .expect("the bounded capability set should be non-empty");

    // Act
    let abort = apply_lifecycle_command(&mut network, LifecycleCommand::AbortPeerEffect(exact))
        .expect("exact abort should dispatch");
    let replacement = apply_lifecycle_command(
        &mut network,
        LifecycleCommand::PrepareRelay(PeerRelayPreparationRequest::new(134_164)),
    );

    // Assert
    assert!(matches!(
        abort,
        crate::network::runtime_authority::LifecycleCommandResult::PeerEffectAborted(
            EffectAbort::Aborted
        )
    ));
    assert!(matches!(
        replacement,
        Ok(crate::network::runtime_authority::LifecycleCommandResult::RelayPrepared(_))
    ));
    assert_eq!(
        network.peer_effect_ledger.pending_len(),
        MAX_PENDING_PEER_EFFECTS
    );
    assert_eq!(network.peer_effect_ledger.completed_len(), 0);
}

#[test]
fn public_peer_abort_facade_consumes_the_exact_pre_achievement_capability() {
    // Arrange
    let handle = ManagedNetworkHandle::from_network_fixture(network_fixture());
    let capability = handle
        .prepare_peer_relay_effect(134_165)
        .expect("peer effect should prepare");

    // Act
    let abort = handle
        .abort_peer_effect(capability)
        .expect("peer abort should dispatch");

    // Assert
    assert_eq!(abort, EffectAbort::Aborted);
    assert!(
        handle.prepare_peer_relay_effect(134_166).is_ok(),
        "exact abort should release pending capacity"
    );
}
