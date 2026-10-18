// Parity breadcrumbs:
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/chain.h

use super::*;
use open_bitcoin_core::chainstate::AnchoredBlock;
use open_bitcoin_core::consensus::chain_proof::ProofWork;
use open_bitcoin_mempool::{PolicyTime, ReorgLifecycleContext};

mod fixtures;
mod freshness;
mod history;
use fixtures::*;

#[test]
fn phase165_work_authority_weaker_max_score_refuses_before_preview() {
    // Arrange
    let (mut network, active, side) = fork(3, 1);
    let before = state(&network);
    // Act
    let result = network.reorg_to_branch(
        &active[1..].iter().rev().cloned().collect::<Vec<_>>(),
        &anchored(&side),
        context(),
        verify_flags(),
        consensus_params(),
    );
    // Assert
    assert!(
        result.is_err(),
        "weaker suffix must not replace the active branch"
    );
    assert_eq!(state(&network), before);
}

#[test]
fn phase165_work_authority_later_equal_body_refuses_before_preview() {
    // Arrange
    let (mut network, active, side) = fork(2, 1);
    let before = state(&network);
    // Act
    let result = network.reorg_to_branch(
        &[active[1].clone()],
        &anchored(&side),
        context(),
        verify_flags(),
        consensus_params(),
    );
    // Assert
    assert!(
        result.is_err(),
        "later body eligibility must lose the known tie"
    );
    assert_eq!(state(&network), before);
}

#[test]
fn phase165_work_authority_stronger_suffix_ignores_all_supplied_scores() {
    // Arrange
    let (mut network, active, side) = fork(2, 2);
    let mut replacements = anchored(&side);
    replacements[0].chain_work = ProofWork::ZERO;
    // Act
    let transition = network
        .reorg_to_branch(
            &[active[1].clone()],
            &replacements,
            context(),
            verify_flags(),
            consensus_params(),
        )
        .expect("matching preferred path");
    // Assert
    let expected = ProofWork::from_le_bytes([
        6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0,
    ]);
    assert_eq!(
        transition.connected.last().expect("tip").chain_work,
        expected
    );
    assert_eq!(
        network.maybe_chain_tip().expect("tip").block_hash,
        block_hash(&side[1].header)
    );
    assert_eq!(
        network
            .peer_manager
            .header_store()
            .maybe_proof_facts(block_hash(&side[1].header))
            .expect("accepted header")
            .cumulative,
        expected
    );
}

#[test]
fn phase165_work_authority_cross_owner_authorization_refuses_before_preparation() {
    // Arrange
    let (network, active, side) = fork(2, 2);
    let foreign = network.clone();
    let replacements = anchored(&side);
    let disconnect = [active[1].clone()];
    let authorization = network
        .authorize_reorg(&disconnect, &replacements, consensus_params())
        .expect("actual preference");
    let before = state(&foreign);
    // Act
    let result = foreign.chainstate.prepare_authorized_reorg(
        authorization,
        &disconnect,
        &replacements,
        verify_flags(),
        consensus_params(),
    );
    // Assert
    assert!(result.is_err());
    assert_eq!(state(&foreign), before);
}

#[test]
fn phase165_work_authority_candidate_mismatch_refuses_before_preparation() {
    // Arrange
    let (network, active, side) = fork(2, 2);
    let mut replacements = anchored(&side);
    let disconnect = [active[1].clone()];
    let authorization = network
        .authorize_reorg(&disconnect, &replacements, consensus_params())
        .expect("actual preference");
    replacements[1].block.transactions[0].lock_time += 1;
    let before = state(&network);
    // Act
    let result = network.chainstate.prepare_authorized_reorg(
        authorization,
        &disconnect,
        &replacements,
        verify_flags(),
        consensus_params(),
    );
    // Assert
    assert!(result.is_err());
    assert_eq!(state(&network), before);
}

#[test]
fn phase165_work_authority_stale_source_refuses_before_preparation() {
    // Arrange
    let (mut network, active, side) = fork(2, 2);
    let replacements = anchored(&side);
    let disconnect = [active[1].clone()];
    let authorization = network
        .authorize_reorg(&disconnect, &replacements, consensus_params())
        .expect("actual preference");
    network
        .chainstate
        .flush_with_mode(
            open_bitcoin_core::chainstate::FlushMode::Always,
            open_bitcoin_core::chainstate::FlushPolicyTime::from_unix_seconds(3_000),
            u64::MAX,
        )
        .expect("benign generation change");
    let before = state(&network);
    // Act
    let result = network.chainstate.prepare_authorized_reorg(
        authorization,
        &disconnect,
        &replacements,
        verify_flags(),
        consensus_params(),
    );
    // Assert
    assert!(result.is_err());
    assert_eq!(state(&network), before);
}

#[test]
fn phase165_work_authority_exact_own_preview_allows_one_matching_commit() {
    // Arrange
    let (mut network, active, side) = fork(2, 2);
    let replacements = anchored(&side);
    let disconnect = [active[1].clone()];
    let authorization = network
        .authorize_reorg(&disconnect, &replacements, consensus_params())
        .expect("actual preference");
    let prepared = network
        .chainstate
        .prepare_authorized_reorg(
            authorization,
            &disconnect,
            &replacements,
            verify_flags(),
            consensus_params(),
        )
        .expect("authorized preparation");
    // Act
    network
        .chainstate
        .install_prepared_reorg_preview(&prepared)
        .expect("one preview");
    let before = state(&network);
    let duplicate = network.chainstate.install_prepared_reorg_preview(&prepared);
    // Assert
    assert!(duplicate.is_err());
    assert_eq!(state(&network), before);
    network
        .chainstate
        .commit_prepared_reorg(prepared)
        .expect("only exact own continuation");
    assert!(network.chainstate.chainstate().has_earned_work());
}

#[test]
fn phase165_work_authority_foreign_prepared_stage_cannot_preview_or_commit() {
    for preview in [false, true] {
        // Arrange
        let (network, active, side) = fork(2, 2);
        let mut foreign = network.clone();
        let replacements = anchored(&side);
        let disconnect = [active[1].clone()];
        let authorization = network
            .authorize_reorg(&disconnect, &replacements, consensus_params())
            .expect("preference");
        let prepared = network
            .chainstate
            .prepare_authorized_reorg(
                authorization,
                &disconnect,
                &replacements,
                verify_flags(),
                consensus_params(),
            )
            .expect("actual source preparation");
        let before = state(&foreign);
        // Act
        let result = if preview {
            foreign.chainstate.install_prepared_reorg_preview(&prepared)
        } else {
            foreign
                .chainstate
                .commit_prepared_reorg(prepared)
                .map(|_| ())
        };
        // Assert
        assert!(result.is_err());
        assert_eq!(state(&foreign), before);
    }
}

#[test]
fn phase165_work_authority_consensus_failure_marks_exact_block_and_descendants_only() {
    // Arrange
    let (mut network, active, side) = fork(2, 1);
    let prefix = side[0].clone();
    let mut invalid = build_block(block_hash(&prefix.header), 2, 5_000_000_000);
    invalid
        .transactions
        .push(spend_transaction(Txid::from_byte_array([44; 32]), 1_000));
    invalid.header.merkle_root = block_merkle_root(&invalid.transactions).expect("merkle").0;
    mine_header(&mut invalid);
    let descendant = build_block(block_hash(&invalid.header), 3, 5_000_000_000);
    let sibling = build_block(block_hash(&prefix.header), 2, 4_000_000_000);
    for block in [&invalid, &descendant, &sibling] {
        network
            .connect_stored_block(
                block,
                ProofWork::ZERO,
                2_000_000_000,
                verify_flags(),
                consensus_params(),
            )
            .expect("transaction checked side");
    }
    let before = network.chainstate_snapshot().expect("active state");
    // Act
    let result = network.reorg_to_branch(
        &[active[1].clone()],
        &anchored(&[prefix.clone(), invalid.clone(), descendant.clone()]),
        context(),
        verify_flags(),
        consensus_params(),
    );
    // Assert
    assert!(
        matches!(result, Err(crate::ManagedNetworkError::Chainstate(open_bitcoin_core::chainstate::ChainstateError::ReplacementBlock { block_hash: hash, .. })) if hash == block_hash(&invalid.header))
    );
    assert_eq!(
        network.chainstate_snapshot().expect("unchanged active"),
        before
    );
    assert!(
        network
            .maybe_body_eligibility(block_hash(&invalid.header))
            .expect("failed")
            .failed
    );
    let eligible: Vec<_> = network
        .eligible_body_candidates()
        .iter()
        .map(|facts| facts.block_hash)
        .collect();
    assert!(eligible.contains(&block_hash(&prefix.header)));
    assert!(eligible.contains(&block_hash(&sibling.header)));
    assert!(!eligible.contains(&block_hash(&invalid.header)));
    assert!(!eligible.contains(&block_hash(&descendant.header)));
    let error = result.expect_err("exact consensus cause remains observable");
    let crate::ManagedNetworkError::Chainstate(
        open_bitcoin_core::chainstate::ChainstateError::ReplacementBlock { ref source, .. },
    ) = error
    else {
        panic!("actual failed replacement identity");
    };
    let original_message = source.to_string();
    assert!(
        matches!(crate::SyncRuntimeError::from(error), crate::SyncRuntimeError::InvalidData { message } if message == original_message)
    );
}

#[test]
fn phase165_work_authority_reseeded_peer_body_order_cannot_rewrite_accepted_tie() {
    // Arrange
    let (mut network, active, side) = fork(2, 1);
    let mut replacement_registry = open_bitcoin_network::HeaderStore::default();
    for block in [&active[0], &side[0], &active[1]] {
        replacement_registry
            .check_body_eligibility(block, consensus_params(), 2_000_000_000)
            .expect("real but differently ordered body checks");
    }
    network
        .peer_manager_mut()
        .seed_header_store(replacement_registry);
    let before = state(&network);
    // Act
    let result = network.reorg_to_branch(
        &[active[1].clone()],
        &anchored(&side),
        context(),
        verify_flags(),
        consensus_params(),
    );
    // Assert
    assert!(
        result.is_err(),
        "a replaced peer registry cannot overwrite accepted body order"
    );
    assert_eq!(state(&network), before);
}

#[test]
fn phase165_work_authority_padded_registry_reseed_cannot_reuse_numeric_active_sequence() {
    for cloned in [false, true] {
        // Arrange
        let (mut network, root, _) = fork(1, 0);
        let filler = build_block(block_hash(&root[0].header), 1, 3_000_000_000);
        let active = build_block(block_hash(&root[0].header), 1, 5_000_000_000);
        let side = build_block(block_hash(&root[0].header), 1, 4_000_000_000);
        let ancestor_registry = network.peer_manager.header_store().clone();
        network
            .peer_manager_mut()
            .check_body_eligibility(&filler, &consensus_params(), 2_000_000_000)
            .expect("real prior unrelated body");
        network
            .connect_local_block(&active, verify_flags(), consensus_params())
            .expect("actual earlier active body");
        network
            .connect_stored_block(
                &side,
                ProofWork::ZERO,
                2_000_000_000,
                verify_flags(),
                consensus_params(),
            )
            .expect("later retained side");
        let accepted_sequence = network
            .maybe_body_eligibility(block_hash(&active.header))
            .expect("original")
            .maybe_sequence;
        let mut replacement = if cloned {
            ancestor_registry
        } else {
            open_bitcoin_network::HeaderStore::default()
        };
        for block in [&root[0], &side, &active] {
            replacement
                .check_body_eligibility(block, consensus_params(), 2_000_000_000)
                .expect("independent body history");
        }
        assert_eq!(
            replacement
                .maybe_body_eligibility(block_hash(&active.header))
                .expect("same numeric sequence")
                .maybe_sequence,
            accepted_sequence
        );
        network.peer_manager_mut().seed_header_store(replacement);
        let before = state(&network);
        // Act
        let result = network.reorg_to_branch(
            &[active],
            &anchored(&[side]),
            context(),
            verify_flags(),
            consensus_params(),
        );
        // Assert
        assert!(
            result.is_err(),
            "numeric order equality cannot rebind independent history"
        );
        assert_eq!(state(&network), before);
    }
}

#[test]
fn phase165_work_authority_missing_undo_or_retained_body_does_not_mark_valid_candidates_failed() {
    for missing_undo in [false, true] {
        // Arrange
        let (mut network, active, side) = fork(2, 2);
        let active_hash = block_hash(&active[1].header);
        if missing_undo {
            network.chainstate.forget_undo_for_test(active_hash);
        } else {
            network.blocks_by_hash.remove(&active_hash);
        }
        let before = state(&network);
        // Act
        let result = network.reorg_to_branch(
            &[active[1].clone()],
            &anchored(&side),
            context(),
            verify_flags(),
            consensus_params(),
        );
        // Assert
        assert!(result.is_err());
        assert_eq!(state(&network), before);
        for block in &side {
            assert!(
                !network
                    .maybe_body_eligibility(block_hash(&block.header))
                    .expect("valid side history")
                    .failed
            );
        }
    }
}

#[test]
fn phase165_work_authority_raw_and_mechanical_managed_connects_refuse_without_effects() {
    for mechanical in [false, true] {
        // Arrange
        let (mut network, _, _) = fork(1, 0);
        let source = network.chainstate.chainstate().snapshot();
        let raw = if mechanical {
            open_bitcoin_core::chainstate::Chainstate::from_snapshot_for_mechanical_fixture(source)
        } else {
            open_bitcoin_core::chainstate::Chainstate::from_snapshot(source)
        };
        network.chainstate.install_chainstate_for_test(raw);
        let child = build_block(
            network.maybe_chain_tip().expect("tip").block_hash,
            1,
            5_000_000_000,
        );
        let before = state(&network);
        // Act / Assert
        assert!(
            network
                .connect_local_block(&child, verify_flags(), consensus_params())
                .is_err()
        );
        assert_eq!(state(&network), before);
        assert!(
            network
                .connect_stored_block(
                    &child,
                    ProofWork::from_le_bytes([255; 32]),
                    2_000_000_000,
                    verify_flags(),
                    consensus_params()
                )
                .is_err()
        );
        assert_eq!(state(&network), before);
    }
}

#[test]
fn phase165_work_authority_stale_prepared_transition_refuses_preview_and_commit() {
    for preview in [false, true] {
        // Arrange
        let (mut network, active, side) = fork(2, 2);
        let replacements = anchored(&side);
        let disconnect = [active[1].clone()];
        let authorization = network
            .authorize_reorg(&disconnect, &replacements, consensus_params())
            .expect("preference");
        let prepared = network
            .chainstate
            .prepare_authorized_reorg(
                authorization,
                &disconnect,
                &replacements,
                verify_flags(),
                consensus_params(),
            )
            .expect("stage");
        network
            .chainstate
            .forget_undo_for_test(block_hash(&active[1].header));
        let before = state(&network);
        // Act
        let result = if preview {
            network.chainstate.install_prepared_reorg_preview(&prepared)
        } else {
            network
                .chainstate
                .commit_prepared_reorg(prepared)
                .map(|_| ())
        };
        // Assert
        assert!(result.is_err());
        assert_eq!(state(&network), before);
    }
}

#[test]
fn phase165_work_authority_actual_handle_refuses_weaker_and_later_equal_suffixes() {
    for active_len in [2, 3] {
        // Arrange
        let (network, active, side) = fork(active_len, 1);
        let handle = crate::ManagedNetworkHandle::from_network_fixture(network);
        let before = format!(
            "{:?}{:?}{:?}{:?}",
            handle.chainstate_snapshot().expect("actual coins/undo"),
            handle.peer_manager_snapshot().expect("peer queues"),
            handle.operator_snapshot().expect("actual projections"),
            handle.mempool_info().expect("mempool")
        );
        // Act
        let result = handle.reorg_to_branch(
            &active[1..].iter().rev().cloned().collect::<Vec<_>>(),
            &anchored(&side),
            context(),
            verify_flags(),
            consensus_params(),
        );
        // Assert
        assert!(result.is_err());
        assert_eq!(
            format!(
                "{:?}{:?}{:?}{:?}",
                handle.chainstate_snapshot().expect("actual coins/undo"),
                handle.peer_manager_snapshot().expect("peer queues"),
                handle.operator_snapshot().expect("actual projections"),
                handle.mempool_info().expect("mempool")
            ),
            before
        );
    }
}
