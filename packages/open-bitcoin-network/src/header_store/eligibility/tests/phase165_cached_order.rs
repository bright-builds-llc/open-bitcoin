// Parity breadcrumbs:
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/chain.h
// - packages/bitcoin-knots/src/node/blockstorage.cpp

use super::*;

#[test]
fn phase165_cached_order_requests_require_full_selected_params_not_generic_proof_facts() {
    // Arrange
    let params = ConsensusParams::default();
    let root = body(BlockHash::default(), 0);
    let hash = block_hash(&root.header);
    let mut unbound = HeaderStore::default();
    unbound
        .insert_header(root.header.clone())
        .expect("generic accepted proof facts");
    assert!(unbound.maybe_proof_facts(hash).is_some());
    let mut bound = HeaderStore::default();
    bound
        .insert_header_with_params(root.header.clone(), &params)
        .expect("selected proof facts");
    let wrong = ConsensusParams {
        coinbase_maturity: 1,
        ..params
    };
    // Act / Assert
    assert!(unbound.requestable_header_entries(&params).is_empty());
    assert!(bound.requestable_header_entries(&wrong).is_empty());
    assert_eq!(
        bound
            .requestable_header_entries(&params)
            .iter()
            .map(|entry| entry.block_hash)
            .collect::<Vec<_>>(),
        vec![hash]
    );
    let imported =
        HeaderStore::from_entries(bound.entries().cloned()).expect("raw imported numbers");
    assert!(imported.requestable_header_entries(&params).is_empty());
}

#[test]
fn phase165_cached_order_request_projection_keeps_best_priority_selected_side_and_raw_exclusion() {
    // Arrange
    let params = ConsensusParams::default();
    let root = body(BlockHash::default(), 0);
    let first = body(block_hash(&root.header), 1);
    let best = body(block_hash(&first.header), 2);
    let mut side = first.clone();
    side.header.time += 10;
    while open_bitcoin_consensus::block::check_block_header(&side.header).is_err() {
        side.header.nonce += 1;
    }
    let mut raw = first.clone();
    raw.header.time += 20;
    while open_bitcoin_consensus::block::check_block_header(&raw.header).is_err() {
        raw.header.nonce += 1;
    }
    let mut store = HeaderStore::default();
    for block in [&root, &first, &best, &side] {
        store
            .insert_header_with_params(block.header.clone(), &params)
            .expect("actual selected header");
    }
    store.record_position(&ChainPosition::new(
        raw.header.clone(),
        1,
        ProofWork::from_le_bytes([255; 32]),
        i64::from(raw.header.time),
    ));
    // Act
    let projected: Vec<_> = store
        .requestable_header_entries(&params)
        .iter()
        .map(|entry| entry.block_hash)
        .collect();
    // Assert
    assert_eq!(
        projected,
        [&root, &first, &best, &side].map(|block| block_hash(&block.header))
    );
    assert!(
        store.entry(&block_hash(&raw.header)).is_some(),
        "raw projection remains readable"
    );
    assert!(store.maybe_proof_facts(block_hash(&raw.header)).is_none());
}

#[test]
fn phase165_cached_order_malformed_ancestry_never_invents_known_order() {
    // Arrange
    for malformed in 0..3 {
        let mut store = linked_store();
        match malformed {
            0 => {
                store.body_eligibility.bodies.remove(&hash(1));
            }
            1 => {
                store.parents.remove(&hash(1));
            }
            _ => {
                store.parents.insert(hash(1), Some(hash(2)));
            }
        }
        let epoch = store.body_eligibility_epoch();
        // Act
        let known = store.has_known_body_order(hash(2));
        // Assert
        assert!(!known);
        assert!(
            epoch.is_current(),
            "an order query never repairs malformed retained facts"
        );
    }
}

#[test]
fn phase165_cached_order_detailed_check_preserves_actual_consensus_cause_and_compatibility() {
    // Arrange
    let mut invalid = body(BlockHash::default(), 0);
    invalid.transactions.clear();
    let mut detailed = HeaderStore::default();
    let mut compatible = HeaderStore::default();
    // Act
    let error = detailed
        .check_cached_body_eligibility_detailed(&invalid, ConsensusParams::default(), 10_000)
        .expect_err("actual invalid body");
    let old_error = compatible
        .check_cached_body_eligibility(&invalid, ConsensusParams::default(), 10_000)
        .expect_err("compatible public refusal");
    // Assert
    let BodyEligibilityError::Validation(source) = error else {
        panic!("actual validator cause required");
    };
    assert!(
        matches!(old_error, NetworkError::InvalidHeader { reject_reason, .. } if reject_reason == source.reject_reason)
    );
    assert!(detailed.eligible_body_candidates(&[]).is_empty());
}

#[test]
fn phase165_cached_order_detailed_params_refusal_is_not_consensus_invalidity() {
    // Arrange
    let params = ConsensusParams::default();
    let root = body(BlockHash::default(), 0);
    let mut store = HeaderStore::default();
    store
        .check_body_eligibility(&root, params, 10_000)
        .expect("selected actual source");
    let epoch = store.body_eligibility_epoch();
    // Act
    let result = store.check_cached_body_eligibility_detailed(
        &root,
        ConsensusParams {
            coinbase_maturity: 1,
            ..params
        },
        10_000,
    );
    // Assert
    assert!(
        matches!(result, Err(BodyEligibilityError::Network(NetworkError::InvalidHeader { ref reject_reason, .. })) if reject_reason == "body-consensus-params-mismatch")
    );
    assert!(epoch.is_current());
}

#[test]
fn phase165_cached_order_detailed_capacity_refusal_is_not_consensus_invalidity() {
    // Arrange
    let mut store = HeaderStore::default();
    store.body_eligibility.capacity = 0;
    let root = body(BlockHash::default(), 0);
    // Act
    let result =
        store.check_cached_body_eligibility_detailed(&root, ConsensusParams::default(), 10_000);
    // Assert
    assert!(
        matches!(result, Err(BodyEligibilityError::Network(NetworkError::InvalidHeader { ref reject_reason, .. })) if reject_reason == "body-eligibility-capacity")
    );
    assert!(
        store
            .maybe_body_eligibility_binding(block_hash(&root.header))
            .is_none()
    );
}

#[test]
fn phase165_cached_order_is_unknown_in_both_import_orders_but_has_actual_linkage() {
    // Arrange
    let params = ConsensusParams::default();
    let root = body(BlockHash::default(), 0);
    let left = body(block_hash(&root.header), 1);
    let mut right = left.clone();
    right.header.time += 1;
    while open_bitcoin_consensus::block::check_block_header(&right.header).is_err() {
        right.header.nonce += 1;
    }
    for siblings in [[&left, &right], [&right, &left]] {
        let mut store = HeaderStore::default();
        store
            .check_body_eligibility(&root, params, 10_000)
            .expect("known root");
        // Act
        for block in siblings {
            store
                .check_cached_body_eligibility(block, params, 10_000)
                .expect("checked raw bytes");
        }
        // Assert
        for block in [&left, &right] {
            let hash = block_hash(&block.header);
            let binding = store
                .maybe_body_eligibility_binding(hash)
                .expect("genuine linkage");
            assert_eq!(binding.facts().maybe_sequence, None);
            assert!(
                store
                    .eligible_body_candidates(&[])
                    .iter()
                    .any(|facts| facts.block_hash == hash)
            );
            store
                .check_body_eligibility(block, params, 10_000)
                .expect("ordinary duplicate");
            let copied = store.clone();
            let current = copied
                .maybe_body_eligibility_binding(hash)
                .expect("copied actual linkage");
            assert!(binding.same_linkage(&current));
            assert_eq!(current.facts().maybe_sequence, None);
        }
    }
}

#[test]
fn phase165_cached_order_pending_and_later_descendants_inherit_unknown_ancestry() {
    // Arrange
    let params = ConsensusParams::default();
    let root = body(BlockHash::default(), 0);
    let parent = body(block_hash(&root.header), 1);
    let child = body(block_hash(&parent.header), 2);
    let later = body(block_hash(&child.header), 3);
    let mut store = HeaderStore::default();
    store
        .check_body_eligibility(&root, params, 10_000)
        .expect("known root");
    store
        .insert_header_with_params(parent.header.clone(), &params)
        .expect("accepted parent header");
    store
        .check_body_eligibility(&child, params, 10_000)
        .expect("checked pending child");
    assert!(
        store
            .maybe_body_eligibility_binding(block_hash(&child.header))
            .is_none()
    );
    let epoch = store.body_eligibility_epoch();
    // Act
    store
        .check_cached_body_eligibility(&parent, params, 10_000)
        .expect("unknown parent receipt");
    store
        .check_body_eligibility(&later, params, 10_000)
        .expect("real later child receipt");
    // Assert
    assert!(!epoch.is_current());
    for block in [&parent, &child, &later] {
        let binding = store
            .maybe_body_eligibility_binding(block_hash(&block.header))
            .expect("fully linked actual bytes");
        assert_eq!(binding.facts().maybe_sequence, None);
    }
    assert_eq!(
        store
            .eligible_body_candidates(&[block_hash(&parent.header)])
            .len(),
        4
    );
}

#[test]
fn phase165_cached_order_duplicate_never_downgrades_known_fifo_or_revokes_epoch() {
    // Arrange
    let mut store = HeaderStore::default();
    let root = body(BlockHash::default(), 0);
    store
        .check_body_eligibility(&root, ConsensusParams::default(), 10_000)
        .expect("actual root arrival");
    let hash = block_hash(&root.header);
    let before = store
        .maybe_body_eligibility_binding(hash)
        .expect("known binding");
    let epoch = store.body_eligibility_epoch();
    // Act
    store
        .check_cached_body_eligibility(&root, ConsensusParams::default(), 10_000)
        .expect("cached known duplicate");
    // Assert
    let after = store
        .maybe_body_eligibility_binding(hash)
        .expect("unchanged binding");
    assert_eq!(before.facts(), after.facts());
    assert!(before.facts().maybe_sequence.is_some());
    assert!(before.same_linkage(&after));
    assert!(epoch.is_current());
}

#[test]
fn phase165_cached_order_invalid_bytes_do_not_create_linkage_or_expire_epoch() {
    // Arrange
    let mut store = HeaderStore::default();
    let mut root = body(BlockHash::default(), 0);
    root.transactions.clear();
    let epoch = store.body_eligibility_epoch();
    // Act
    let result = store.check_cached_body_eligibility(&root, ConsensusParams::default(), 10_000);
    // Assert
    assert!(result.is_err());
    assert!(store.eligible_body_candidates(&[]).is_empty());
    assert!(
        store
            .maybe_body_eligibility_binding(block_hash(&root.header))
            .is_none()
    );
    assert!(epoch.is_current());
}
