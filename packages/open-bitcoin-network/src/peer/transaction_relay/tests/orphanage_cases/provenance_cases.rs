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
fn different_deliverer_provenance_is_bounded_deduplicated_and_retains_delivered_by() {
    // Arrange
    let mut orphanage = TxOrphanage::new(OrphanPolicy {
        max_announcers_per_orphan: 3,
        ..policy(10, 10, 120, 10)
    });
    let input = orphan_input(9, 100, 101, [7], 0);

    // Act
    let _ = orphanage.stage_missing_parent_with_provenance(input, provenance(9, [5, 3, 5, 1, 7]));

    // Assert
    assert_eq!(orphanage.peer_len(9), 1);
    assert_eq!(orphanage.peer_len(1), 1);
    assert_eq!(orphanage.peer_len(3), 1);
    assert_eq!(orphanage.peer_len(5), 0);
    assert!(orphanage.debug_indexes_match_oracle());
}

#[test]
fn late_announcer_respects_per_peer_orphan_cap() {
    // Arrange
    let mut orphanage = TxOrphanage::new(policy(10, 1, 120, 10));
    let first_wtxid = wtxid(107);
    let second_wtxid = wtxid(109);
    let _ = stage_singleton(&mut orphanage, 1, orphan_input(1, 106, 107, [7], 0));
    let _ = stage_singleton(&mut orphanage, 2, orphan_input(2, 108, 109, [8], 0));

    // Act
    let first_added = orphanage.add_announcer(first_wtxid, 3);
    let second_added = orphanage.add_announcer(second_wtxid, 3);

    // Assert
    assert!(first_added);
    assert!(!second_added);
    assert_eq!(orphanage.peer_len(3), 1);
    assert!(orphanage.contains(first_wtxid));
    assert!(orphanage.contains(second_wtxid));
    assert!(orphanage.debug_indexes_match_oracle());
}

#[test]
fn late_announcer_missing_body_is_noop_and_existing_body_does_not_refresh_ttl() {
    // Arrange
    let mut orphanage = TxOrphanage::new(OrphanPolicy {
        max_announcers_per_orphan: 2,
        ..policy(10, 10, 5, 10)
    });
    let _ = stage_singleton(&mut orphanage, 1, orphan_input(1, 102, 103, [7], 10));

    // Act
    let missing_body_added = orphanage.add_announcer(wtxid(200), 2);
    let existing_body_added = orphanage.add_announcer(wtxid(103), 2);
    let over_cap_added = orphanage.add_announcer(wtxid(103), 3);
    let expired_actions = orphanage.expire(15);

    // Assert
    assert!(!missing_body_added);
    assert!(existing_body_added);
    assert!(!over_cap_added);
    assert_eq!(expired_actions, [expired(1, 102, 103)]);
    assert!(orphanage.is_empty());
    assert!(orphanage.debug_indexes_match_oracle());
}

#[test]
fn disconnect_removes_only_one_announcer_and_deletes_body_at_zero() {
    // Arrange
    let mut orphanage = TxOrphanage::new(policy(10, 10, 120, 10));
    let _ = orphanage.stage_missing_parent_with_provenance(
        orphan_input(1, 104, 105, [7], 0),
        provenance(1, [1, 2]),
    );

    // Act
    let first_cleanup = orphanage.cleanup_peer(1);
    let second_cleanup = orphanage.cleanup_peer(2);

    // Assert
    assert!(matches!(
        &first_cleanup[..],
        [OrphanAction::PeerCleanup {
            peer_id: 1,
            removed: 0,
            ..
        }]
    ));
    assert_eq!(second_cleanup.len(), 1);
    assert!(orphanage.is_empty());
    assert!(orphanage.debug_indexes_match_oracle());
}

#[test]
fn coherent_disconnect_expiry_eviction_cleanup_preserves_index_oracle() {
    // Arrange
    let mut orphanage = TxOrphanage::new(policy(2, 10, 2, 10));
    let _ = stage_singleton(&mut orphanage, 1, orphan_input(1, 160, 161, [7], 0));
    let _ = stage_singleton(&mut orphanage, 2, orphan_input(2, 160, 161, [8], 1));
    let _ = stage_singleton(&mut orphanage, 3, orphan_input(3, 162, 163, [8], 1));
    let _ = stage_singleton(&mut orphanage, 4, orphan_input(4, 164, 165, [9], 1));

    // Act / Assert
    assert_eq!(orphanage.len(), 2, "the total cap must evict one body");
    assert!(orphanage.debug_indexes_match_oracle());
    let _ =
        orphanage.record_reconsideration_outcome(wtxid(161), OrphanReconsiderationStatus::Rejected);
    assert!(orphanage.debug_indexes_match_oracle());
    let _ = orphanage.expire(3);
    assert!(orphanage.debug_indexes_match_oracle());
    let _ = orphanage.cleanup_peer(3);
    assert!(orphanage.debug_indexes_match_oracle());
}
