// Parity breadcrumbs:
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/chain.h
// - packages/bitcoin-knots/src/node/blockstorage.cpp

use super::*;

#[test]
fn phase165_eligibility_epoch_changes_only_for_actual_eligibility_changes() {
    // Arrange
    let mut store = HeaderStore::default();
    let root = body(BlockHash::default(), 0);
    let hash = block_hash(&root.header);
    let empty = store.body_eligibility_epoch();
    // Act / Assert
    store
        .check_body_eligibility(&root, ConsensusParams::default(), 2_000_000_000)
        .expect("new body");
    assert!(!empty.is_current());
    let linked = store.body_eligibility_epoch();
    store
        .check_body_eligibility(&root, ConsensusParams::default(), 2_000_000_000)
        .expect("unchanged duplicate");
    let _facts = store.maybe_body_eligibility(hash);
    let _candidates = store.eligible_body_candidates(&[]);
    store.mark_body_failed(BlockHash::default());
    store.mark_body_unavailable(BlockHash::default());
    assert!(linked.is_current());
    store.mark_body_unavailable(hash);
    assert!(!linked.is_current());
    let unavailable = store.body_eligibility_epoch();
    store.mark_body_unavailable(hash);
    assert!(unavailable.is_current());
    store.mark_body_failed(hash);
    assert!(!unavailable.is_current());
    let failed = store.body_eligibility_epoch();
    store.mark_body_failed(hash);
    assert!(failed.is_current());
}

#[test]
fn phase165_eligibility_clone_shares_history_but_never_mutable_epoch_ownership() {
    // Arrange
    let mut source = HeaderStore::default();
    let root = body(BlockHash::default(), 0);
    let hash = block_hash(&root.header);
    source
        .check_body_eligibility(&root, ConsensusParams::default(), 2_000_000_000)
        .expect("actual source event");
    let source_epoch = source.body_eligibility_epoch();
    let mut independent = source.clone();
    let clone_epoch = independent.body_eligibility_epoch();
    // Act
    independent.mark_body_failed(hash);
    // Assert
    assert!(source_epoch.is_current());
    assert!(!clone_epoch.is_current());
    assert!(
        source
            .maybe_body_eligibility_binding(hash)
            .expect("source event")
            .same_linkage(
                &independent
                    .maybe_body_eligibility_binding(hash)
                    .expect("same immutable clone event")
            )
    );
    source.mark_body_unavailable(hash);
    assert!(
        !source_epoch.is_current(),
        "clone cannot keep the old source epoch alive"
    );
}

#[test]
fn phase165_eligibility_epoch_expires_when_owner_is_dropped_or_reseeded() {
    // Arrange
    let mut source = HeaderStore::default();
    let original = source.body_eligibility_epoch();
    // Act / Assert
    source.seed_from_chain(&[]);
    assert!(!original.is_current());
    let current = source.body_eligibility_epoch();
    drop(source);
    assert!(!current.is_current());
}

#[test]
fn phase165_eligibility_linkage_seal_preserves_only_the_actual_history_event() {
    // Arrange
    let mut first = HeaderStore::default();
    let root = body(BlockHash::default(), 0);
    let hash = block_hash(&root.header);
    let mut before_linkage = first.clone();
    assert!(first.maybe_body_eligibility_binding(hash).is_none());
    first
        .check_body_eligibility(&root, ConsensusParams::default(), 2_000_000_000)
        .expect("first actual body");
    let binding = first
        .maybe_body_eligibility_binding(hash)
        .expect("actual event");
    let after_linkage = first.clone();
    // Act
    before_linkage
        .check_body_eligibility(&root, ConsensusParams::default(), 2_000_000_000)
        .expect("independent same body");
    first
        .check_body_eligibility(&root, ConsensusParams::default(), 2_000_000_000)
        .expect("duplicate same event");
    // Assert
    assert!(
        binding.same_linkage(
            &after_linkage
                .maybe_body_eligibility_binding(hash)
                .expect("immutable cloned event")
        )
    );
    assert!(
        binding.same_linkage(
            &first
                .maybe_body_eligibility_binding(hash)
                .expect("duplicate event")
        )
    );
    assert!(
        !binding.same_linkage(
            &before_linkage
                .maybe_body_eligibility_binding(hash)
                .expect("equal numeric independent event")
        )
    );
    assert_eq!(
        binding.facts(),
        before_linkage
            .maybe_body_eligibility(hash)
            .expect("equal numeric facts")
    );
    assert!(
        first
            .maybe_body_eligibility_binding(BlockHash::default())
            .is_none()
    );
}
