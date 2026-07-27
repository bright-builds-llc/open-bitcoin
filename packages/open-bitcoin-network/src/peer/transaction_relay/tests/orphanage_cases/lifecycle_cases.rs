// Parity breadcrumbs:
// - packages/bitcoin-knots/src/protocol.h
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/node/txdownloadman.h
// - packages/bitcoin-knots/src/node/txdownloadman_impl.cpp
// - packages/bitcoin-knots/src/txorphanage.h
// - packages/bitcoin-knots/src/txorphanage.cpp
// - packages/bitcoin-knots/src/txrequest.h
// - packages/bitcoin-knots/src/txrequest.cpp
// - packages/bitcoin-knots/test/functional/p2p_orphan_handling.py
// - packages/bitcoin-knots/test/functional/p2p_opportunistic_1p1c.py
// - packages/bitcoin-knots/test/functional/p2p_tx_download.py
// - packages/bitcoin-knots/test/functional/p2p_getdata.py

use super::*;

#[test]
fn expiry_uses_injected_time_without_sleeping() {
    // Arrange
    let mut orphanage = TxOrphanage::new(policy(10, 10, 5, 10));
    let _ = stage_singleton(&mut orphanage, 8, orphan_input(8, 40, 41, [1], 10));

    // Act
    let too_early = orphanage.expire(14);
    let expired_at_deadline = orphanage.expire(15);
    let mut zero_ttl_orphanage = TxOrphanage::new(policy(10, 10, 0, 10));
    let immediately_expired =
        stage_singleton(&mut zero_ttl_orphanage, 9, orphan_input(9, 42, 43, [1], 20));

    // Assert
    assert!(too_early.is_empty());
    assert_eq!(expired_at_deadline, [expired(8, 40, 41)]);
    assert!(orphanage.is_empty());
    assert_eq!(immediately_expired, [expired(9, 42, 43)]);
    assert!(zero_ttl_orphanage.is_empty());
}

#[test]
fn peer_cleanup_removes_owned_orphans() {
    // Arrange
    let mut orphanage = TxOrphanage::new(policy(10, 10, 120, 10));
    let _ = stage_singleton(&mut orphanage, 1, orphan_input(1, 80, 81, [7], 0));
    let _ = stage_singleton(&mut orphanage, 1, orphan_input(1, 82, 83, [7], 0));
    let _ = stage_singleton(&mut orphanage, 2, orphan_input(2, 84, 85, [7], 0));

    // Act
    let actions = orphanage.cleanup_peer(1);

    // Assert
    assert_eq!(
        actions,
        [OrphanAction::PeerCleanup {
            peer_id: 1,
            removed: 2,
            label: OrphanEvidenceLabel::OrphanEvicted,
        }],
    );
    assert_eq!(orphanage.len(), 1);
    assert_eq!(orphanage.peer_len(1), 0);
    assert_eq!(orphanage.peer_len(2), 1);
}

#[test]
fn peer_cleanup_without_owned_orphans_is_noop() {
    // Arrange
    let mut orphanage = TxOrphanage::new(policy(10, 10, 120, 10));
    let _ = stage_singleton(&mut orphanage, 2, orphan_input(2, 90, 91, [7], 0));

    // Act
    let actions = orphanage.cleanup_peer(1);

    // Assert
    assert!(actions.is_empty());
    assert_eq!(orphanage.len(), 1);
    assert_eq!(orphanage.peer_len(2), 1);
}
