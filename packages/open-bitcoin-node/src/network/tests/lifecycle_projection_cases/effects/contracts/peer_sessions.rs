// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/node/mempool_persist.cpp
// - packages/bitcoin-knots/src/txmempool.cpp
// - packages/bitcoin-knots/src/validation.cpp

use super::*;

#[test]
fn unrelated_peer_churn_does_not_stale_an_exact_peer_receipt() {
    // Arrange
    let mut network = network_fixture();
    let target_peer = 134_150;
    let unrelated_peer = 134_151;
    network
        .connect_outbound_peer(target_peer, 1)
        .expect("target peer should connect");
    network
        .connect_outbound_peer(unrelated_peer, 1)
        .expect("unrelated peer should connect");
    let receipt = match apply_lifecycle_command(
        &mut network,
        LifecycleCommand::PrepareRelay(PeerRelayPreparationRequest::new(target_peer)),
    )
    .expect("target peer effect should prepare")
    {
        crate::network::runtime_authority::LifecycleCommandResult::RelayPrepared(capability) => {
            capability.acknowledge_write()
        }
        _ => panic!("relay preparation returned the wrong command result"),
    };
    network
        .disconnect_peer(unrelated_peer)
        .expect("unrelated peer should disconnect");
    network
        .connect_outbound_peer(unrelated_peer, 2)
        .expect("unrelated peer should reconnect");

    // Act
    let completion =
        apply_lifecycle_command(&mut network, LifecycleCommand::CompletePeerEffect(receipt))
            .expect("target receipt should remain valid");

    // Assert
    assert!(matches!(
        completion,
        crate::network::runtime_authority::LifecycleCommandResult::PeerEffectCompleted(
            EffectCompletion::Applied
        )
    ));
    assert_eq!(network.peer_effect_ledger.pending_len(), 0);
    assert_eq!(network.peer_effect_ledger.completed_len(), 1);
}

#[test]
fn same_peer_replacement_stales_only_the_exact_achieved_receipt() {
    // Arrange
    let mut network = network_fixture();
    let target_peer = 134_152;
    network
        .connect_outbound_peer(target_peer, 1)
        .expect("target peer should connect");
    let receipt = match apply_lifecycle_command(
        &mut network,
        LifecycleCommand::PrepareRelay(PeerRelayPreparationRequest::new(target_peer)),
    )
    .expect("target peer effect should prepare")
    {
        crate::network::runtime_authority::LifecycleCommandResult::RelayPrepared(capability) => {
            capability.acknowledge_write()
        }
        _ => panic!("relay preparation returned the wrong command result"),
    };
    network
        .disconnect_peer(target_peer)
        .expect("target peer should disconnect");
    network
        .connect_outbound_peer(target_peer, 2)
        .expect("target peer should reconnect");
    let replacement_peer_before = format!("{:?}", network.peer_manager);
    let replacement_evidence_before =
        serde_json::to_value(network.relay_evidence_status()).expect("relay evidence");

    // Act
    let completion =
        apply_lifecycle_command(&mut network, LifecycleCommand::CompletePeerEffect(receipt))
            .expect("achieved receipt should be classified");

    // Assert
    assert!(matches!(
        completion,
        crate::network::runtime_authority::LifecycleCommandResult::PeerEffectCompleted(
            EffectCompletion::AchievedButStale
        )
    ));
    assert_eq!(network.peer_effect_ledger.pending_len(), 0);
    assert_eq!(network.peer_effect_ledger.completed_len(), 1);
    assert_eq!(
        format!("{:?}", network.peer_manager),
        replacement_peer_before
    );
    assert_eq!(
        serde_json::to_value(network.relay_evidence_status()).expect("relay evidence"),
        replacement_evidence_before
    );
}

#[test]
fn replacement_peer_rejects_foreign_target_binding_without_ledger_mutation() {
    // Arrange
    let mut network = network_fixture();
    let target_peer = 134_153;
    network
        .connect_outbound_peer(target_peer, 1)
        .expect("target peer should connect");
    let exact = match apply_lifecycle_command(
        &mut network,
        LifecycleCommand::PrepareRelay(PeerRelayPreparationRequest::new(target_peer)),
    )
    .expect("target peer effect should prepare")
    {
        crate::network::runtime_authority::LifecycleCommandResult::RelayPrepared(capability) => {
            capability.acknowledge_write()
        }
        _ => panic!("relay preparation returned the wrong command result"),
    };
    network
        .disconnect_peer(target_peer)
        .expect("target peer should disconnect");
    network
        .connect_outbound_peer(target_peer, 2)
        .expect("target peer should reconnect");
    let foreign = PeerEffectCapability::new(
        exact.authority_epoch(),
        exact.lifecycle_generation(),
        exact.effect_id(),
        134_154,
        exact.peer_session_generation(),
    )
    .acknowledge_write();
    let state_before = format!("{network:?}");

    // Act
    let result =
        apply_lifecycle_command(&mut network, LifecycleCommand::CompletePeerEffect(foreign));

    // Assert
    assert!(result.is_err());
    assert_eq!(format!("{network:?}"), state_before);
    assert_eq!(network.peer_effect_ledger.pending_len(), 1);
    assert_eq!(network.peer_effect_ledger.completed_len(), 0);
}
