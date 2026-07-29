// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/node/mempool_persist.cpp
// - packages/bitcoin-knots/src/txmempool.cpp
// - packages/bitcoin-knots/src/validation.cpp

use super::*;
use open_bitcoin_core::primitives::BlockHash;
use open_bitcoin_network::{HeadersMessage, WireNetworkMessage};

use crate::network::PeerEmission;

fn emission_receipt(
    peer_id: PeerId,
    block_hash: BlockHash,
    capability: PeerEffectCapability,
) -> crate::network::PeerEmissionReceipt {
    PeerEmission::new(
        peer_id,
        WireNetworkMessage::Headers(HeadersMessage {
            headers: Vec::new(),
        }),
        block_hash,
        capability,
    )
    .expect("headers emission should match the prepared peer")
    .into_parts()
    .2
    .acknowledge_write()
}

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
        serde_json::to_value(network.block_relay_evidence_status()).expect("relay evidence");

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
        serde_json::to_value(network.block_relay_evidence_status()).expect("relay evidence"),
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

#[test]
fn emission_command_records_current_evidence_once_and_classifies_replay() {
    // Arrange
    let mut network = network_fixture();
    let target_peer = 134_155;
    network
        .connect_outbound_peer(target_peer, 1)
        .expect("target peer should connect");
    let capability = match apply_lifecycle_command(
        &mut network,
        LifecycleCommand::PrepareRelay(PeerRelayPreparationRequest::new(target_peer)),
    )
    .expect("target peer effect should prepare")
    {
        crate::network::runtime_authority::LifecycleCommandResult::RelayPrepared(capability) => {
            capability
        }
        _ => panic!("relay preparation returned the wrong command result"),
    };
    let receipt = emission_receipt(
        target_peer,
        BlockHash::from_byte_array([0x31; 32]),
        capability,
    );
    let replay = receipt.duplicate_for_test();
    let evidence_before =
        serde_json::to_value(network.block_relay_evidence_status()).expect("relay evidence");

    // Act
    let completion = apply_lifecycle_command(
        &mut network,
        LifecycleCommand::CompletePeerEmission(receipt),
    )
    .expect("current emission should complete");
    let evidence_after =
        serde_json::to_value(network.block_relay_evidence_status()).expect("relay evidence");
    let peer_after = format!("{:?}", network.peer_manager);
    let replay_completion =
        apply_lifecycle_command(&mut network, LifecycleCommand::CompletePeerEmission(replay))
            .expect("replayed emission should be classified");

    // Assert
    assert!(matches!(
        completion,
        crate::network::runtime_authority::LifecycleCommandResult::PeerEffectCompleted(
            EffectCompletion::Applied
        )
    ));
    assert!(matches!(
        replay_completion,
        crate::network::runtime_authority::LifecycleCommandResult::PeerEffectCompleted(
            EffectCompletion::AlreadyApplied
        )
    ));
    assert_ne!(evidence_after, evidence_before);
    assert_eq!(
        serde_json::to_value(network.block_relay_evidence_status()).expect("relay evidence"),
        evidence_after
    );
    assert_eq!(format!("{:?}", network.peer_manager), peer_after);
    assert_eq!(network.peer_effect_ledger.pending_len(), 0);
    assert_eq!(network.peer_effect_ledger.completed_len(), 1);
}

#[test]
fn stale_emission_command_changes_only_exact_achieved_accounting() {
    // Arrange
    let mut network = network_fixture();
    let target_peer = 134_156;
    network
        .connect_outbound_peer(target_peer, 1)
        .expect("target peer should connect");
    let capability = match apply_lifecycle_command(
        &mut network,
        LifecycleCommand::PrepareRelay(PeerRelayPreparationRequest::new(target_peer)),
    )
    .expect("target peer effect should prepare")
    {
        crate::network::runtime_authority::LifecycleCommandResult::RelayPrepared(capability) => {
            capability
        }
        _ => panic!("relay preparation returned the wrong command result"),
    };
    let receipt = emission_receipt(
        target_peer,
        BlockHash::from_byte_array([0x32; 32]),
        capability,
    );
    network
        .disconnect_peer(target_peer)
        .expect("target peer should disconnect");
    network
        .connect_outbound_peer(target_peer, 2)
        .expect("target peer should reconnect");
    let replacement_peer_before = format!("{:?}", network.peer_manager);
    let evidence_before =
        serde_json::to_value(network.block_relay_evidence_status()).expect("relay evidence");

    // Act
    let completion = apply_lifecycle_command(
        &mut network,
        LifecycleCommand::CompletePeerEmission(receipt),
    )
    .expect("stale achieved emission should be classified");

    // Assert
    assert!(matches!(
        completion,
        crate::network::runtime_authority::LifecycleCommandResult::PeerEffectCompleted(
            EffectCompletion::AchievedButStale
        )
    ));
    assert_eq!(
        serde_json::to_value(network.block_relay_evidence_status()).expect("relay evidence"),
        evidence_before
    );
    assert_eq!(
        format!("{:?}", network.peer_manager),
        replacement_peer_before
    );
    assert_eq!(network.peer_effect_ledger.pending_len(), 0);
    assert_eq!(network.peer_effect_ledger.completed_len(), 1);
}

#[test]
fn foreign_emission_binding_changes_neither_accounting_nor_evidence() {
    // Arrange
    let mut network = network_fixture();
    let target_peer = 134_157;
    let foreign_peer = 134_158;
    network
        .connect_outbound_peer(target_peer, 1)
        .expect("target peer should connect");
    let capability = match apply_lifecycle_command(
        &mut network,
        LifecycleCommand::PrepareRelay(PeerRelayPreparationRequest::new(target_peer)),
    )
    .expect("target peer effect should prepare")
    {
        crate::network::runtime_authority::LifecycleCommandResult::RelayPrepared(capability) => {
            capability
        }
        _ => panic!("relay preparation returned the wrong command result"),
    };
    let exact = capability.acknowledge_write();
    let foreign_capability = PeerEffectCapability::new(
        exact.authority_epoch(),
        exact.lifecycle_generation(),
        exact.effect_id(),
        foreign_peer,
        exact.peer_session_generation(),
    );
    let foreign_receipt = emission_receipt(
        foreign_peer,
        BlockHash::from_byte_array([0x33; 32]),
        foreign_capability,
    );
    let state_before = format!("{network:?}");

    // Act
    let result = apply_lifecycle_command(
        &mut network,
        LifecycleCommand::CompletePeerEmission(foreign_receipt),
    );

    // Assert
    assert!(result.is_err());
    assert_eq!(format!("{network:?}"), state_before);
    assert_eq!(network.peer_effect_ledger.pending_len(), 1);
    assert_eq!(network.peer_effect_ledger.completed_len(), 0);
}
