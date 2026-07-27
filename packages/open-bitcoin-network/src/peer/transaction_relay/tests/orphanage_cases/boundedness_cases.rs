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
fn total_cap_eviction_is_deterministic() {
    // Arrange
    let mut orphanage = TxOrphanage::new(policy(2, 10, 120, 10));
    let _ = stage_singleton(&mut orphanage, 2, orphan_input(2, 20, 2, [1], 0));
    let _ = stage_singleton(&mut orphanage, 1, orphan_input(1, 21, 3, [1], 0));

    // Act
    let actions = stage_singleton(&mut orphanage, 1, orphan_input(1, 22, 1, [1], 0));

    // Assert
    assert!(actions.contains(&evicted(1, 22, 1)));
    assert_eq!(orphanage.len(), 2);
    assert_eq!(orphanage.peer_len(1), 1);
    assert_eq!(orphanage.peer_len(2), 1);
}

#[test]
fn per_peer_cap_eviction_is_deterministic() {
    // Arrange
    let mut orphanage = TxOrphanage::new(policy(10, 2, 120, 10));
    let _ = stage_singleton(&mut orphanage, 7, orphan_input(7, 30, 3, [1], 0));
    let _ = stage_singleton(&mut orphanage, 7, orphan_input(7, 31, 2, [1], 0));

    // Act
    let actions = stage_singleton(&mut orphanage, 7, orphan_input(7, 32, 1, [1], 0));

    // Assert
    assert!(actions.contains(&evicted(7, 32, 1)));
    assert_eq!(orphanage.len(), 2);
    assert_eq!(orphanage.peer_len(7), 2);
}

#[test]
fn announcer_cap_keeps_one_shared_body_under_adversarial_peer_churn() {
    // Arrange
    let configured_cap = PHASE133_MAX_ANNOUNCERS_PER_ORPHAN;
    let mut orphanage = TxOrphanage::new(OrphanPolicy::default());
    let retained_wtxid = wtxid(102);
    let announced_peers = 1..=(configured_cap as u64 + 4);

    // Act
    let _ = orphanage.stage_missing_parent_with_provenance(
        orphan_input(99, 101, 102, [7], 0),
        provenance(99, announced_peers.clone()),
    );
    for peer_id in announced_peers {
        let _ = orphanage.add_announcer(retained_wtxid, peer_id);
    }

    // Assert
    let retained_associations: usize = (1..=99).map(|peer_id| orphanage.peer_len(peer_id)).sum();
    assert_eq!(orphanage.len(), 1, "announcers must share one body");
    assert_eq!(retained_associations, configured_cap);
    assert!(orphanage.contains(retained_wtxid));
    assert!(orphanage.debug_indexes_match_oracle());
}

#[test]
fn retained_byte_budget_evicts_large_orphan_bodies_before_count_cap() {
    // Arrange
    let body_bytes = 4_096;
    let mut orphanage = TxOrphanage::new(OrphanPolicy {
        max_retained_bytes: body_bytes * 2,
        ..policy(10, 10, 120, 10)
    });
    let _ = orphanage.stage_missing_parent_with_provenance(
        OrphanStageInput {
            transaction: transaction_with_body_bytes(1, body_bytes),
            ..orphan_input(1, 101, 102, [7], 0)
        },
        provenance(1, [1]),
    );

    // Act
    let actions = orphanage.stage_missing_parent_with_provenance(
        OrphanStageInput {
            transaction: transaction_with_body_bytes(2, body_bytes),
            ..orphan_input(2, 103, 104, [7], 1)
        },
        provenance(2, [2]),
    );

    // Assert
    assert_eq!(orphanage.len(), 1, "byte budget must precede count cap");
    assert_eq!(actions, [evicted(1, 101, 102), parent_request(2, 7)]);
    assert!(orphanage.debug_retained_bytes() <= body_bytes * 2);
    assert!(orphanage.debug_indexes_match_oracle());
}

#[test]
fn retained_byte_budget_rejects_late_announcer_state_growth() {
    // Arrange
    let input = OrphanStageInput {
        transaction: transaction_with_body_bytes(1, 4_096),
        ..orphan_input(1, 105, 106, [7], 0)
    };
    let mut probe = TxOrphanage::new(policy(10, 10, 120, 10));
    let _ = probe.stage_missing_parent_with_provenance(input.clone(), provenance(1, [1]));
    let retained_cap = probe.debug_retained_bytes();
    let mut orphanage = TxOrphanage::new(OrphanPolicy {
        max_retained_bytes: retained_cap,
        ..policy(10, 10, 120, 10)
    });
    let _ = orphanage.stage_missing_parent_with_provenance(input, provenance(1, [1]));

    // Act
    let added = orphanage.add_announcer(wtxid(106), 2);

    // Assert
    assert!(!added);
    assert_eq!(orphanage.peer_len(2), 0);
    assert_eq!(orphanage.debug_retained_bytes(), retained_cap);
}
