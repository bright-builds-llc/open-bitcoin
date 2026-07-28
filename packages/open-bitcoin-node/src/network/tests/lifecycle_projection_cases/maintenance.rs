// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/node/mempool_persist.cpp
// - packages/bitcoin-knots/src/txmempool.cpp
// - packages/bitcoin-knots/src/validation.cpp

use open_bitcoin_core::{
    chainstate::AnchoredBlock,
    consensus::block_merkle_root,
    primitives::{Block, BlockHash, BlockHeader, Transaction},
};
use open_bitcoin_mempool::{
    FinalMempoolMembership, MempoolCapacityStatus, MempoolOutcome, MempoolRemovalCause,
    MempoolRemovalRole, ReorgLifecycleContext,
};

use super::*;
use crate::network::mempool_lifecycle;

fn generation_after(count: usize) -> LifecycleGeneration {
    (0..count).fold(LifecycleGeneration::INITIAL, |generation, _| {
        generation.checked_next().expect("test generation advances")
    })
}

fn empty_projection(
    network: &ManagedPeerNetwork<MemoryChainstateStore>,
    generation_count: usize,
    evidence: LifecycleEvidenceSnapshot,
) -> ExpectedProjection {
    let generation = generation_after(generation_count);
    ExpectedProjection {
        canonical_members: BTreeSet::new(),
        serving_members: BTreeSet::new(),
        fanout_members: BTreeSet::new(),
        peer_known_members: BTreeSet::new(),
        peer: network.peer_manager.mempool_lifecycle_snapshot(),
        compact_members: BTreeSet::new(),
        unbroadcast_members: BTreeSet::new(),
        authority_epoch: AuthorityEpoch::INITIAL,
        lifecycle_generation: generation,
        dirty_generation: (generation_count != 0).then_some(generation),
        evidence,
        reconciliation_counts: [0; 7],
    }
}

fn present_projection(
    network: &ManagedPeerNetwork<MemoryChainstateStore>,
    members: BTreeSet<MempoolMemberIdentity>,
    generation_count: usize,
) -> ExpectedProjection {
    let generation = generation_after(generation_count);
    ExpectedProjection {
        canonical_members: members.clone(),
        serving_members: members.clone(),
        fanout_members: members.clone(),
        peer_known_members: members,
        peer: network.peer_manager.mempool_lifecycle_snapshot(),
        compact_members: BTreeSet::new(),
        unbroadcast_members: BTreeSet::new(),
        authority_epoch: AuthorityEpoch::INITIAL,
        lifecycle_generation: generation,
        dirty_generation: Some(generation),
        evidence: LifecycleEvidenceSnapshot {
            committed_transitions: generation_count as u64,
            admitted_members: generation_count as u64,
            ..LifecycleEvidenceSnapshot::default()
        },
        reconciliation_counts: [0; 7],
    }
}

fn identity(transaction: &Transaction) -> MempoolMemberIdentity {
    MempoolMemberIdentity {
        txid: transaction_txid(transaction).expect("txid"),
        wtxid: transaction_wtxid(transaction).expect("wtxid"),
    }
}

fn block_with_transactions(
    previous_block_hash: BlockHash,
    height: u32,
    extra_transactions: Vec<Transaction>,
) -> Block {
    let mut transactions = vec![coinbase_transaction(height, 500_000_000)];
    transactions.extend(extra_transactions);
    let (merkle_root, maybe_mutated) = block_merkle_root(&transactions).expect("merkle root");
    assert!(!maybe_mutated);
    let mut block = Block {
        header: BlockHeader {
            version: 1,
            previous_block_hash,
            merkle_root,
            time: 1_231_006_500 + height,
            bits: EASY_BITS,
            nonce: 0,
        },
        transactions,
    };
    mine_header(&mut block);
    block
}

#[test]
fn expiry_removes_descendants_from_every_projection_and_advances_once() {
    // Arrange
    let (mut network, coinbase_txid) = network_with_spendable_coinbase(PolicyConfig {
        mempool_expiry_hours: 1,
        ..PolicyConfig::default()
    });
    let parent = spend_transaction(coinbase_txid, 499_999_000);
    let parent_identity = identity(&parent);
    let child = spend_transaction(parent_identity.txid, 499_998_000);
    let child_identity = identity(&child);
    network
        .submit_local_transaction_outcome_at(
            parent,
            verify_flags(),
            consensus_params(),
            0,
            RelayIntent::Requested,
        )
        .expect("parent admission");
    network
        .submit_local_transaction_outcome_at(
            child,
            verify_flags(),
            consensus_params(),
            3_700,
            RelayIntent::Requested,
        )
        .expect("child admission");
    let prepared = network
        .mempool()
        .prepare_expiry(PolicyTime::new(3_601))
        .expect("expiry should prepare");
    assert_prepared_orders(&prepared, &[], &[child_identity, parent_identity]);

    // Act
    let delta = network
        .expire_mempool(PolicyTime::new(3_601))
        .expect("expiry should apply");

    // Assert
    assert!(delta.removed.iter().any(|removal| {
        removal.member == parent_identity
            && removal.cause == MempoolRemovalCause::Expiry
            && removal.role == MempoolRemovalRole::Direct
    }));
    assert!(delta.removed.iter().any(|removal| {
        removal.member == child_identity
            && removal.cause == MempoolRemovalCause::Expiry
            && removal.role == MempoolRemovalRole::Descendant
    }));
    assert!(
        delta
            .final_membership
            .iter()
            .all(|state| { state.membership == FinalMempoolMembership::Absent })
    );
    assert_complete_projection(
        &network,
        &empty_projection(
            &network,
            3,
            LifecycleEvidenceSnapshot {
                committed_transitions: 3,
                admitted_members: 2,
                removed_members: 2,
                retry_clears: 2,
                expiry_removals: 2,
                ..LifecycleEvidenceSnapshot::default()
            },
        ),
    );
}

#[test]
fn connected_block_conflict_removes_descendants_from_every_projection() {
    // Arrange
    let (mut network, coinbase_txid) = network_with_spendable_coinbase(PolicyConfig::default());
    let parent = spend_transaction(coinbase_txid, 499_999_000);
    let parent_identity = identity(&parent);
    let child = spend_transaction(parent_identity.txid, 499_998_000);
    let child_identity = identity(&child);
    network
        .submit_local_transaction_outcome_at(
            parent,
            verify_flags(),
            consensus_params(),
            100,
            RelayIntent::Requested,
        )
        .expect("parent admission");
    network
        .submit_local_transaction_outcome_at(
            child,
            verify_flags(),
            consensus_params(),
            101,
            RelayIntent::Requested,
        )
        .expect("child admission");
    let conflict = spend_transaction(coinbase_txid, 499_997_000);
    let tip = network
        .chainstate
        .chainstate()
        .tip()
        .expect("chain tip")
        .clone();
    let block = block_with_transactions(tip.block_hash, tip.height + 1, vec![conflict]);
    let context = mempool_lifecycle::block_lifecycle_context(200, tip.height + 1);
    let prepared = network
        .mempool()
        .prepare_connected_block_transition(&block, context)
        .expect("connected-block transition should prepare");
    assert_prepared_orders(&prepared, &[], &[child_identity, parent_identity]);

    // Act
    let lifecycle = network
        .apply_connected_block_mempool_lifecycle(&block, context)
        .expect("connected-block lifecycle should apply");

    // Assert
    assert!(lifecycle.delta.removed.iter().any(|removal| {
        removal.member == parent_identity
            && removal.cause == MempoolRemovalCause::BlockConflict
            && removal.role == MempoolRemovalRole::Direct
    }));
    assert!(lifecycle.delta.removed.iter().any(|removal| {
        removal.member == child_identity
            && removal.cause == MempoolRemovalCause::BlockConflict
            && removal.role == MempoolRemovalRole::Descendant
    }));
    assert_complete_projection(
        &network,
        &empty_projection(
            &network,
            3,
            LifecycleEvidenceSnapshot {
                committed_transitions: 3,
                admitted_members: 2,
                removed_members: 2,
                retry_clears: 2,
                block_conflict_removals: 2,
                ..LifecycleEvidenceSnapshot::default()
            },
        ),
    );
}

#[test]
fn empty_maintenance_preserves_generation_evidence_and_pressure_summary() {
    // Arrange
    let (mut network, _coinbase_txid) = network_with_spendable_coinbase(PolicyConfig::default());
    let baseline = empty_projection(&network, 0, LifecycleEvidenceSnapshot::default());

    // Act
    let delta = network
        .expire_mempool(PolicyTime::new(10_000))
        .expect("empty expiry should apply");

    // Assert
    assert!(delta.is_empty());
    assert_eq!(
        network.mempool_info().capacity_status,
        MempoolCapacityStatus::Empty
    );
    assert_complete_projection(&network, &baseline);
}

#[test]
fn reorg_steps_apply_sequentially_and_reconcile_each_generation() {
    // Arrange
    let (mut network, coinbase_txid) = network_with_spendable_coinbase(PolicyConfig::default());
    let old_parent = spend_transaction(coinbase_txid, 499_999_000);
    let parent_identity = identity(&old_parent);
    let old_child = spend_transaction(parent_identity.txid, 499_998_000);
    let child_identity = identity(&old_child);
    let fork = network
        .chainstate
        .chainstate()
        .tip()
        .expect("fork position")
        .clone();
    let old_parent_tip =
        block_with_transactions(fork.block_hash, fork.height + 1, vec![old_parent.clone()]);
    network
        .connect_local_block(&old_parent_tip, verify_flags(), consensus_params())
        .expect("old parent tip");
    let old_child_tip = block_with_transactions(
        block_hash(&old_parent_tip.header),
        fork.height + 2,
        vec![old_child.clone()],
    );
    network
        .connect_local_block(&old_child_tip, verify_flags(), consensus_params())
        .expect("old child tip");
    let replacement_parent_tip =
        block_with_transactions(fork.block_hash, fork.height + 1, Vec::new());
    let replacement_child_tip = block_with_transactions(
        block_hash(&replacement_parent_tip.header),
        fork.height + 2,
        Vec::new(),
    );
    let replacement = [
        AnchoredBlock {
            block: replacement_parent_tip.clone(),
            chain_work: fork.chain_work + 1,
        },
        AnchoredBlock {
            block: replacement_child_tip.clone(),
            chain_work: fork.chain_work + 2,
        },
    ];
    let disconnected = [old_child_tip, old_parent_tip];
    let transition = network
        .chainstate
        .reorg(
            &disconnected,
            &replacement,
            verify_flags(),
            consensus_params(),
        )
        .expect("chainstate reorg");
    let context = ReorgLifecycleContext::new(PolicyTime::new(500));

    // Act — every replacement-block state commits before disconnected transactions.
    for (anchored, position) in replacement.iter().zip(&transition.connected) {
        let block_context =
            mempool_lifecycle::block_lifecycle_context_from_reorg(context, position.height);
        let connected = network
            .apply_reorg_connected_block_mempool_lifecycle(&anchored.block, block_context)
            .expect("replacement-block step");

        // Assert
        assert!(connected.delta.is_empty());
        assert_complete_projection(
            &network,
            &empty_projection(&network, 0, LifecycleEvidenceSnapshot::default()),
        );
    }

    // Act — the parent commits before the child is prepared.
    let parent = network
        .apply_reorg_transaction_mempool_lifecycle(
            old_parent,
            verify_flags(),
            consensus_params(),
            context,
        )
        .expect("parent reconsideration");

    // Assert
    assert!(matches!(
        parent.outcome,
        MempoolOutcome::Accepted { txid, .. } if txid == parent_identity.txid
    ));
    assert_complete_projection(
        &network,
        &present_projection(&network, BTreeSet::from([parent_identity]), 1),
    );

    // Act — child preparation observes the parent's committed membership.
    let child = network
        .apply_reorg_transaction_mempool_lifecycle(
            old_child,
            verify_flags(),
            consensus_params(),
            context,
        )
        .expect("child reconsideration");

    // Assert
    assert!(matches!(
        child.outcome,
        MempoolOutcome::Accepted { txid, .. } if txid == child_identity.txid
    ));
    assert_complete_projection(
        &network,
        &present_projection(
            &network,
            BTreeSet::from([parent_identity, child_identity]),
            2,
        ),
    );
}
