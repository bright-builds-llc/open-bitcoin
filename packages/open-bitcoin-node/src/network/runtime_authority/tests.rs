// Parity breadcrumbs:
// - packages/bitcoin-knots/src/node/context.h

use open_bitcoin_core::{
    consensus::{ConsensusParams, ScriptVerifyFlags},
    primitives::{
        Amount, OutPoint, ScriptBuf, ScriptWitness, Transaction, TransactionInput,
        TransactionOutput, Txid,
    },
};
use open_bitcoin_mempool::{MempoolOutcome, PolicyConfig, PolicyTime, RelayIntent};
use open_bitcoin_network::LocalPeerConfig;

use crate::{ManagedPeerNetwork, MemoryChainstateStore};

use super::{ManagedNetworkAuthorityError, ManagedNetworkHandle};

fn test_handle() -> ManagedNetworkHandle {
    let network = ManagedPeerNetwork::new(
        MemoryChainstateStore::default(),
        LocalPeerConfig::default(),
        PolicyConfig::default(),
    );
    ManagedNetworkHandle::new(network)
}

fn orphan_transaction() -> Transaction {
    Transaction {
        version: 2,
        inputs: vec![TransactionInput {
            previous_output: OutPoint {
                txid: Txid::from_byte_array([7_u8; 32]),
                vout: 0,
            },
            script_sig: ScriptBuf::from_bytes(vec![0x51]).expect("valid script"),
            sequence: TransactionInput::SEQUENCE_FINAL,
            witness: ScriptWitness::default(),
        }],
        outputs: vec![TransactionOutput {
            value: Amount::from_sats(1_000).expect("valid amount"),
            script_pubkey: ScriptBuf::from_bytes(vec![0x51]).expect("valid script"),
        }],
        lock_time: 0,
    }
}

#[test]
fn cloned_handles_share_mutations() {
    // Arrange
    let mutating_handle = test_handle();
    let snapshot_handle = mutating_handle.clone();

    // Act
    mutating_handle
        .connect_outbound_peer(1, 1_777_225_210)
        .expect("shared authority should accept the peer");
    let snapshot = snapshot_handle
        .network_info()
        .expect("shared authority should return an owned snapshot");

    // Assert
    assert_eq!(snapshot.outbound_peers, 1);
}

#[test]
fn owned_snapshot_survives_authority_drop() {
    // Arrange
    let handle = test_handle();

    // Act
    let snapshot = handle
        .chainstate_snapshot()
        .expect("shared authority should return an owned snapshot");
    drop(handle);

    // Assert
    assert!(snapshot.active_chain.is_empty());
}

#[test]
fn poisoned_authority_returns_typed_error() {
    // Arrange
    let handle = test_handle();
    handle.poison_for_test();

    // Act
    let result = handle.operator_snapshot();

    // Assert
    assert!(matches!(
        result,
        Err(ManagedNetworkAuthorityError::Poisoned)
    ));
}

#[test]
fn explicit_local_admission_flows_through_the_shared_authority() {
    // Arrange
    let handle = test_handle();

    // Act
    let outcome = handle
        .submit_local_transaction_outcome_at(
            orphan_transaction(),
            ScriptVerifyFlags::NONE,
            ConsensusParams::default(),
            50,
            RelayIntent::Requested,
        )
        .expect("authority should return an admission outcome");

    // Assert
    assert!(matches!(outcome, MempoolOutcome::Orphaned { .. }));
}

#[test]
fn expire_mempool_flows_through_the_shared_authority() {
    // Arrange — empty authority; membership/serving age fixtures live in
    // mempool_lifecycle_cases (`expire_mempool_authority_hook_removes_aged_entry`).
    let handle = test_handle();

    // Act
    let delta = handle
        .expire_mempool(PolicyTime::new(1_000))
        .expect("expire through authority");

    // Assert
    assert!(delta.is_empty());
}
