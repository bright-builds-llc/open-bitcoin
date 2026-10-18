// Parity breadcrumbs:
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/chain.h
// - packages/bitcoin-knots/src/node/blockstorage.cpp

use super::*;
use open_bitcoin_primitives::{
    Amount, OutPoint, ScriptBuf, ScriptWitness, Transaction, TransactionInput, TransactionOutput,
};

fn hash(marker: u8) -> BlockHash {
    Hash32::from_byte_array([marker; 32]).into()
}

mod phase165_authority;
mod phase165_cached_order;

#[test]
fn phase165_eligibility_orphan_waits_then_breadth_first_release_preserves_parent_fifo() {
    // Arrange
    let mut registry = BodyEligibilityRegistry::default();
    registry
        .record(hash(3), Some(hash(1)))
        .expect("first child");
    registry
        .record(hash(4), Some(hash(3)))
        .expect("early grandchild");
    registry
        .record(hash(2), Some(hash(1)))
        .expect("later sibling");
    assert_eq!(registry.bodies[&hash(3)].maybe_sequence, None);
    // Act
    registry.record(hash(1), None).expect("root linkage");
    // Assert
    let ordered: Vec<_> = [1, 3, 2, 4]
        .map(hash)
        .into_iter()
        .map(|hash| registry.bodies[&hash].maybe_sequence)
        .collect();
    assert_eq!(ordered, vec![Some(2), Some(3), Some(4), Some(5)]);
    assert!(registry.pending.is_empty());
}

#[test]
fn phase165_eligibility_present_duplicate_preserves_sequence_and_pending_order() {
    // Arrange
    let mut registry = BodyEligibilityRegistry::default();
    registry.record(hash(2), Some(hash(1))).expect("pending");
    let pending = registry.clone();
    // Act / Assert
    registry
        .record(hash(2), Some(hash(1)))
        .expect("duplicate pending");
    assert_eq!(registry, pending);
    registry.record(hash(1), None).expect("link root");
    let linked = registry.clone();
    registry
        .record(hash(2), Some(hash(1)))
        .expect("duplicate linked");
    assert_eq!(registry, linked);
}

#[test]
fn phase165_eligibility_capacity_refuses_without_partial_mutation() {
    // Arrange
    let mut registry = BodyEligibilityRegistry {
        capacity: 1,
        ..Default::default()
    };
    registry
        .record(hash(2), Some(hash(1)))
        .expect("bounded pending");
    let before = registry.clone();
    // Act
    let result = registry.record(hash(1), None);
    // Assert
    assert!(result.is_err());
    assert_eq!(registry, before);
}

#[test]
fn phase165_eligibility_exhaustion_reserves_entire_descendant_release() {
    // Arrange
    let mut registry = BodyEligibilityRegistry::default();
    registry.record(hash(2), Some(hash(1))).expect("pending");
    registry.next_sequence = u64::MAX - 1;
    let before = registry.clone();
    // Act
    let result = registry.record(hash(1), None);
    // Assert
    assert!(result.is_err());
    assert_eq!(registry, before);
}

fn linked_store() -> HeaderStore {
    let mut store = HeaderStore::default();
    for (marker, parent, height) in [
        (1, None, 0),
        (2, Some(1), 1),
        (3, Some(1), 1),
        (4, Some(2), 2),
    ] {
        let hash = hash(marker);
        store.parents.insert(hash, parent.map(self::hash));
        store.accepted_headers.insert(hash);
        store.entries.insert(
            hash,
            HeaderEntry {
                block_hash: hash,
                header: BlockHeader {
                    bits: 0x207f_ffff,
                    ..Default::default()
                },
                height,
                chain_work: ProofWork::from_hex(&format!("{:064x}", height + 1)).expect("work"),
            },
        );
    }
    for marker in [1, 3, 2, 4] {
        let hash = hash(marker);
        store
            .body_eligibility
            .record(hash, store.parents[&hash])
            .expect("checked linkage");
    }
    store
}

#[test]
fn phase165_eligibility_equal_work_uses_body_order_including_active_tip() {
    // Arrange
    let mut store = linked_store();
    store.mark_body_unavailable(hash(4));
    // Act
    let candidates = store.eligible_body_candidates(&[hash(1), hash(3)]);
    // Assert
    assert_eq!(
        candidates
            .iter()
            .map(|facts| facts.block_hash)
            .collect::<Vec<_>>(),
        [hash(3), hash(2), hash(1)]
    );
    assert!(candidates[0].maybe_sequence < candidates[1].maybe_sequence);
}

#[test]
fn phase165_eligibility_greater_exact_work_precedes_sequence_and_enumerates_sides() {
    // Arrange
    let store = linked_store();
    // Act
    let candidates = store.eligible_body_candidates(&[]);
    // Assert
    assert_eq!(
        candidates
            .iter()
            .map(|facts| facts.block_hash)
            .collect::<Vec<_>>(),
        [hash(4), hash(3), hash(2), hash(1)]
    );
}

#[test]
fn phase165_eligibility_pruning_preserves_linkage_but_excludes_nonactive_suffix() {
    // Arrange
    let mut store = linked_store();
    let before = store
        .maybe_body_eligibility(hash(2))
        .expect("linked")
        .maybe_sequence;
    // Act
    store.mark_body_unavailable(hash(2));
    let without_active = store.eligible_body_candidates(&[]);
    let with_active = store.eligible_body_candidates(&[hash(1), hash(2)]);
    // Assert
    assert_eq!(
        store
            .maybe_body_eligibility(hash(2))
            .expect("historical")
            .maybe_sequence,
        before
    );
    assert!(
        !without_active
            .iter()
            .any(|facts| facts.block_hash == hash(4))
    );
    assert_eq!(with_active[0].block_hash, hash(4));
    assert!(
        store
            .body_eligibility
            .record(hash(2), Some(hash(1)))
            .is_err()
    );
}

#[test]
fn phase165_eligibility_failed_ancestor_excludes_every_descendant() {
    // Arrange
    let mut store = linked_store();
    // Act
    store.mark_body_failed(hash(2));
    store.mark_body_failed(hash(99));
    store.mark_body_unavailable(hash(99));
    let candidates = store.eligible_body_candidates(&[hash(1), hash(2)]);
    // Assert
    assert_eq!(
        candidates
            .iter()
            .map(|facts| facts.block_hash)
            .collect::<Vec<_>>(),
        [hash(3), hash(1)]
    );
}

#[test]
fn phase165_eligibility_unlinked_missing_or_cyclic_ancestry_cannot_rank() {
    // Arrange
    let mut store = linked_store();
    store
        .body_eligibility
        .bodies
        .get_mut(&hash(2))
        .expect("body")
        .maybe_sequence = None;
    store.parents.insert(hash(3), Some(hash(99)));
    // Act / Assert
    assert_eq!(store.eligible_body_candidates(&[]).len(), 1);
    store.parents.insert(hash(1), Some(hash(1)));
    assert!(store.eligible_body_candidates(&[]).is_empty());
    store.parents.remove(&hash(1));
    assert!(store.eligible_body_candidates(&[]).is_empty());
    assert_eq!(store.maybe_body_eligibility(hash(99)), None);
}

fn body(parent: BlockHash, height: u8) -> Block {
    let transactions = vec![Transaction {
        version: 1,
        inputs: vec![TransactionInput {
            previous_output: OutPoint::null(),
            script_sig: ScriptBuf::from_bytes(if height == 0 {
                vec![0, 0x51]
            } else {
                vec![1, height, 0x51]
            })
            .expect("coinbase script"),
            sequence: TransactionInput::SEQUENCE_FINAL,
            witness: ScriptWitness::default(),
        }],
        outputs: vec![TransactionOutput {
            value: Amount::from_sats(5_000_000_000).expect("subsidy"),
            script_pubkey: ScriptBuf::from_bytes(vec![0x51]).expect("true"),
        }],
        lock_time: 0,
    }];
    let mut block = Block {
        header: BlockHeader {
            version: 1,
            previous_block_hash: parent,
            merkle_root: open_bitcoin_consensus::block_merkle_root(&transactions)
                .expect("merkle")
                .0,
            time: 1_000 + u32::from(height),
            bits: 0x207f_ffff,
            nonce: 0,
        },
        transactions,
    };
    while open_bitcoin_consensus::block::check_block_header(&block.header).is_err() {
        block.header.nonce = block.header.nonce.checked_add(1).expect("cheap mining");
    }
    block
}

#[test]
fn phase165_eligibility_checked_bytes_bind_params_and_preserve_duplicate_sequence() {
    // Arrange
    let params = ConsensusParams::default();
    let root = body(BlockHash::default(), 0);
    let child = body(block_hash(&root.header), 1);
    let mut store = HeaderStore::default();
    // Act
    store
        .check_body_eligibility(&root, params, 10_000)
        .expect("checked root");
    store
        .check_body_eligibility(&child, params, 10_000)
        .expect("checked child");
    let before = store.clone();
    store
        .check_body_eligibility(&child, params, 10_000)
        .expect("duplicate");
    // Assert
    assert_eq!(store, before);
    assert_eq!(
        store.eligible_body_candidates(&[])[0].maybe_sequence,
        Some(3)
    );
    assert!(
        store
            .check_body_eligibility(
                &child,
                ConsensusParams {
                    enforce_segwit: false,
                    ..params
                },
                10_000
            )
            .is_err()
    );
    store.seed_from_chain(&[]);
    assert!(store.eligible_body_candidates(&[]).is_empty());
}

#[test]
fn phase165_eligibility_invalid_body_or_unproved_import_cannot_mint_sequence() {
    // Arrange
    let params = ConsensusParams::default();
    let root = body(BlockHash::default(), 0);
    let mut store = HeaderStore::default();
    let mut mutated = root.clone();
    mutated.transactions.clear();
    // Act / Assert
    assert!(
        store
            .check_body_eligibility(&mutated, params, 10_000)
            .is_err()
    );
    assert_eq!(store, HeaderStore::default());
    let mut high_hash = root.clone();
    high_hash.header.bits = 0x1d00_ffff;
    assert!(
        store
            .check_body_eligibility(&high_hash, params, 10_000)
            .is_err()
    );
    store
        .insert_header_with_params(root.header.clone(), &params)
        .expect("header");
    let imported = HeaderStore::from_entries(store.entries().cloned()).expect("raw import");
    assert!(
        imported
            .clone()
            .check_body_eligibility(&root, params, 10_000)
            .is_err()
    );
    store.accepted_headers.clear();
    assert!(store.check_body_eligibility(&root, params, 10_000).is_err());
    assert_eq!(store.maybe_body_eligibility(block_hash(&root.header)), None);
}

#[test]
fn phase165_eligibility_context_uses_accepted_parent_and_rejects_other_roots() {
    // Arrange
    let params = ConsensusParams::default();
    let root = body(BlockHash::default(), 0);
    let child = body(block_hash(&root.header), 1);
    let mut store = HeaderStore::default();
    // Act / Assert
    assert!(
        store
            .header_validation_context(&child.header, 10_000, params)
            .is_err()
    );
    store
        .check_body_eligibility(&root, params, 10_000)
        .expect("root");
    let other_root = body(BlockHash::default(), 1);
    assert!(
        store
            .header_validation_context(&other_root.header, 10_000, params)
            .is_err()
    );
    let context = store
        .header_validation_context(&child.header, 10_000, params)
        .expect("derived context");
    assert_eq!(context.height, 1);
    assert_eq!(context.previous_header, root.header);
    assert_eq!(context.previous_median_time_past, 1_000);
    store.accepted_headers.clear();
    assert!(
        store
            .header_validation_context(&child.header, 10_000, params)
            .is_err()
    );
}

#[test]
fn phase165_eligibility_pruned_ancestor_can_release_historical_pending_linkage() {
    // Arrange
    let mut registry = BodyEligibilityRegistry::default();
    registry
        .record(hash(3), Some(hash(2)))
        .expect("pending child");
    registry
        .record(hash(2), Some(hash(1)))
        .expect("pending parent");
    registry.bodies.get_mut(&hash(2)).expect("parent").available = false;
    // Act
    registry.record(hash(1), None).expect("release root");
    // Assert
    assert_eq!(registry.bodies[&hash(3)].maybe_sequence, Some(4));
    assert!(!registry.bodies[&hash(2)].available);
}

#[test]
fn phase165_eligibility_peer_delegate_checks_bytes_and_revokes_current_facts() {
    // Arrange
    let params = ConsensusParams::default();
    let root = body(BlockHash::default(), 0);
    let hash = block_hash(&root.header);
    let mut peer = crate::PeerManager::new(crate::LocalPeerConfig::default());
    // Act
    peer.check_body_eligibility(&root, &params, 10_000)
        .expect("checked peer body");
    peer.mark_body_failed(hash);
    peer.mark_body_unavailable(hash);
    // Assert
    let facts = peer
        .header_store()
        .maybe_body_eligibility(hash)
        .expect("historical linkage");
    assert_eq!(facts.maybe_sequence, Some(2));
    assert!(facts.failed);
    assert!(!facts.available);
    assert!(peer.header_store().eligible_body_candidates(&[]).is_empty());
}

#[test]
fn phase165_eligibility_context_retarget_anchor_and_minimum_recovery_follow_ancestry() {
    // Arrange
    let params = ConsensusParams {
        no_pow_retargeting: false,
        pow_target_spacing_seconds: 100,
        pow_target_timespan_seconds: 200,
        ..Default::default()
    };
    let root = body(BlockHash::default(), 0);
    let first = body(block_hash(&root.header), 1);
    let second = body(block_hash(&first.header), 2);
    let mut store = HeaderStore::default();
    store
        .insert_header_with_params(root.header.clone(), &params)
        .expect("root");
    store
        .insert_header_with_params(first.header.clone(), &params)
        .expect("first");
    // Act
    let retarget = store
        .header_validation_context(&second.header, 10_000, params)
        .expect("retarget boundary");
    let recovery = store
        .header_validation_context(&first.header, 10_000, params)
        .expect("root recovery");
    let mut delayed = first.header.clone();
    delayed.time = 1_201;
    let delayed_context = store
        .header_validation_context(&delayed, 10_000, params)
        .expect("delayed minimum");
    // Assert
    assert_eq!(
        retarget.maybe_retarget_anchor,
        Some(RetargetAnchor {
            first_block_time: 1_000
        })
    );
    assert_eq!(
        recovery.maybe_min_difficulty_recovery_target,
        Some(MinDifficultyRecoveryTarget {
            bits: root.header.bits
        })
    );
    assert_eq!(delayed_context.maybe_min_difficulty_recovery_target, None);
    assert_eq!(
        difficulty_adjustment_interval(&ConsensusParams {
            pow_target_spacing_seconds: 0,
            ..params
        }),
        1
    );
    assert_eq!(
        min_difficulty_recovery_target(
            &store,
            store.entry(&block_hash(&root.header)).expect("root"),
            &first.header,
            1,
            2,
            ConsensusParams {
                allow_min_difficulty_blocks: false,
                ..params
            }
        )
        .expect("disabled minimum"),
        None
    );
}

#[test]
fn phase165_eligibility_minimum_recovery_walks_past_easy_parent() {
    // Arrange
    let params = ConsensusParams {
        no_pow_retargeting: false,
        pow_target_spacing_seconds: 100,
        pow_target_timespan_seconds: 300,
        ..Default::default()
    };
    let root = body(BlockHash::default(), 0);
    let first = body(block_hash(&root.header), 1);
    let second = body(block_hash(&first.header), 2);
    let mut store = HeaderStore::default();
    store
        .insert_header_with_params(root.header.clone(), &params)
        .expect("root");
    store
        .insert_header_with_params(first.header.clone(), &params)
        .expect("first");
    // Act
    let context = store
        .header_validation_context(&second.header, 10_000, params)
        .expect("walk back past easy parent");
    // Assert
    assert_eq!(
        context.maybe_min_difficulty_recovery_target,
        Some(MinDifficultyRecoveryTarget {
            bits: root.header.bits
        })
    );
}

#[test]
fn phase165_eligibility_context_refuses_missing_recovery_ancestor_beyond_mtp_window() {
    // Arrange: malformed internal ancestry retains eleven MTP headers but loses the recovery root.
    let params = ConsensusParams {
        no_pow_retargeting: false,
        pow_target_spacing_seconds: 100,
        pow_target_timespan_seconds: 100_000,
        ..Default::default()
    };
    let mut store = HeaderStore::default();
    for height in 1..=12_u8 {
        store.entries.insert(
            hash(height),
            HeaderEntry {
                block_hash: hash(height),
                height: u32::from(height),
                chain_work: ProofWork::ZERO,
                header: BlockHeader {
                    bits: params.pow_limit_bits,
                    time: 1_000 + u32::from(height),
                    previous_block_hash: hash(height - 1),
                    ..Default::default()
                },
            },
        );
        store.parents.insert(hash(height), Some(hash(height - 1)));
        store.accepted_headers.insert(hash(height));
    }
    let incoming = BlockHeader {
        previous_block_hash: hash(12),
        time: 1_013,
        ..Default::default()
    };
    // Act
    let result = store.header_validation_context(&incoming, 10_000, params);
    // Assert
    assert!(
        matches!(result, Err(NetworkError::MissingHeaderAncestor(missing)) if missing == hash(0))
    );
}
