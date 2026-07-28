// Parity breadcrumbs:
// - packages/bitcoin-knots/src/node/txdownloadman_impl.cpp
// - packages/bitcoin-knots/src/txorphanage.cpp

use super::*;
use crate::{
    AcceptedPeerPackageFingerprint, PeerTransactionIdentity, PeerTransactionLifecycleInput,
};

#[derive(Clone, Copy)]
enum RemovalCause {
    BlockConnected,
    Conflict,
    Replacement,
}

#[derive(Clone, Copy)]
enum TestPeerRole {
    Inbound,
    Outbound,
}

fn identity(txid_byte: u8, wtxid_byte: u8) -> PeerTransactionIdentity {
    PeerTransactionIdentity::new(txid_from_byte(txid_byte), wtxid_from_byte(wtxid_byte))
}

fn manager_with_role(role: TestPeerRole, peer_id: PeerId) -> PeerManager {
    let mut manager = relay_download_manager(true);
    match role {
        TestPeerRole::Inbound => {
            add_relay_permissioned_inbound_peer(&mut manager, peer_id);
        }
        TestPeerRole::Outbound => {
            add_relay_outbound_peer(&mut manager, peer_id);
        }
    }
    manager
}

fn apply_cause_agnostic_teardown(
    manager: &mut PeerManager,
    target: PeerTransactionIdentity,
    cause: RemovalCause,
) {
    let teardowns = match cause {
        RemovalCause::BlockConnected | RemovalCause::Conflict | RemovalCause::Replacement => {
            vec![target]
        }
    };
    let prepared = manager
        .prepare_transaction_lifecycle(PeerTransactionLifecycleInput::new(
            Vec::new(),
            teardowns,
            Vec::new(),
        ))
        .expect("cause-independent teardown should prepare");
    manager.apply_prepared_transaction_lifecycle(prepared);
}

#[test]
fn prepared_lifecycle_preserves_parent_first_admission_and_descendant_first_teardown() {
    // Arrange
    let admission_parent = identity(1, 2);
    let admission_child = identity(3, 4);
    let teardown_parent = identity(5, 6);
    let teardown_child = identity(7, 8);
    let manager = PeerManager::new(local_config());

    // Act
    let prepared = manager
        .prepare_transaction_lifecycle(PeerTransactionLifecycleInput::new(
            vec![admission_parent, admission_child],
            vec![teardown_child, teardown_parent],
            Vec::new(),
        ))
        .expect("explicit ordering should prepare");

    // Assert
    assert_eq!(
        prepared.admission_order(),
        &[admission_parent, admission_child]
    );
    assert_eq!(
        prepared.teardown_order(),
        &[teardown_child, teardown_parent]
    );
}

#[test]
fn lifecycle_cleanup_is_independent_of_peer_role_and_removal_cause() {
    // Arrange
    let target = identity(20, 21);
    let fingerprint = [22; 32];
    let roles = [TestPeerRole::Inbound, TestPeerRole::Outbound];
    let causes = [
        RemovalCause::BlockConnected,
        RemovalCause::Conflict,
        RemovalCause::Replacement,
    ];

    // Act
    let outcomes = roles
        .into_iter()
        .flat_map(|role| causes.into_iter().map(move |cause| (role, cause)))
        .enumerate()
        .map(|(index, (role, cause))| {
            let mut manager = manager_with_role(role, 20_000 + index as u64);
            let admission = manager
                .prepare_transaction_lifecycle(PeerTransactionLifecycleInput::new(
                    vec![target],
                    Vec::new(),
                    vec![AcceptedPeerPackageFingerprint::new(
                        fingerprint,
                        vec![target],
                    )],
                ))
                .expect("admission should prepare");
            manager.apply_prepared_transaction_lifecycle(admission);
            apply_cause_agnostic_teardown(&mut manager, target, cause);
            (
                manager.debug_mempool_identity_known(target),
                manager.debug_accepted_package_fingerprint_contains(fingerprint),
            )
        })
        .collect::<Vec<_>>();

    // Assert
    assert_eq!(outcomes, vec![(false, false); 6]);
}
