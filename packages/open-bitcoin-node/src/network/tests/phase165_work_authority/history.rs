// Parity breadcrumbs:
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/chain.h

use super::*;

#[test]
fn phase165_work_authority_genuine_earlier_body_tie_survives_post_active_clone() {
    for cloned in [false, true] {
        // Arrange
        let (mut network, root, _) = fork(1, 0);
        let side = build_block(block_hash(&root[0].header), 1, 4_000_000_000);
        let active = build_block(block_hash(&root[0].header), 1, 5_000_000_000);
        network
            .retain_checked_candidate(&side, consensus_params(), 2_000_000_000)
            .expect("actual earlier checked retained side");
        network
            .connect_local_block(&active, verify_flags(), consensus_params())
            .expect("later active acceptance");
        let mut owner = if cloned { network.clone() } else { network };
        // Act
        owner
            .reorg_to_branch(
                &[active],
                &anchored(std::slice::from_ref(&side)),
                context(),
                verify_flags(),
                consensus_params(),
            )
            .expect("same immutable history and genuinely earlier candidate");
        // Assert
        assert_eq!(
            owner.maybe_chain_tip().expect("new active").block_hash,
            block_hash(&side.header)
        );
        assert_eq!(
            owner
                .maybe_chain_tip()
                .expect("exact work")
                .chain_work
                .to_le_bytes(),
            [
                4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0
            ]
        );
    }
}

#[test]
fn phase165_work_authority_unknown_accepted_body_order_cannot_authorize_an_equal_tie() {
    // Arrange
    let (mut network, root, _) = fork(1, 0);
    let side = build_block(block_hash(&root[0].header), 1, 4_000_000_000);
    let active = build_block(block_hash(&root[0].header), 1, 5_000_000_000);
    network
        .retain_checked_candidate(&side, consensus_params(), 2_000_000_000)
        .expect("earlier checked side");
    network
        .retain_checked_candidate(&active, consensus_params(), 2_000_000_000)
        .expect("later checked active");
    network
        .chainstate
        .connect_block(&active, ProofWork::ZERO, verify_flags(), consensus_params())
        .expect("genuine direct extension without arrival capability");
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
        "fresh checked numeric registry facts cannot reconstruct accepted activation order"
    );
    assert_eq!(state(&network), before);
}

#[test]
fn phase165_work_authority_actual_lower_public_owner_refuses_known_later_and_weaker_validated_bodies()
 {
    for active_len in [2, 3] {
        // Arrange
        let (mut network, active, side) = fork(active_len, 1);
        let replacements = anchored(&side);
        let disconnect = active[1..].iter().rev().cloned().collect::<Vec<_>>();
        assert!(
            network
                .chainstate
                .chainstate()
                .stage_reorg(
                    &disconnect,
                    &replacements,
                    verify_flags(),
                    consensus_params()
                )
                .expect("actual genuinely validated suffix")
                .has_earned_work()
        );
        let candidate = network
            .maybe_body_eligibility(block_hash(&side[0].header))
            .expect("actual later candidate history");
        let accepted = network
            .maybe_body_eligibility(block_hash(&active.last().expect("tip").header))
            .expect("actual earlier accepted history");
        assert!(candidate.maybe_sequence > accepted.maybe_sequence);
        let before = state(&network);
        // Act
        let result = network.chainstate.reorg(
            &disconnect,
            &replacements,
            verify_flags(),
            consensus_params(),
        );
        // Assert
        assert!(result.is_err());
        assert_eq!(state(&network), before);
    }
}

#[test]
fn phase165_work_authority_forged_imported_parent_metadata_cannot_authorize_a_valid_child() {
    for mutation in 0..4 {
        // Arrange
        let (mut network, active, _) = fork(1, 0);
        let child = build_block(block_hash(&active[0].header), 1, 5_000_000_000);
        let mut snapshot = network.chainstate_snapshot().expect("actual source facts");
        let position = snapshot.active_chain.last_mut().expect("parent");
        match mutation {
            0 => position.chain_work = ProofWork::from_le_bytes([255; 32]),
            1 => position.height += 1,
            2 => position.block_hash = BlockHash::from_byte_array([44; 32]),
            _ => position.header.previous_block_hash = BlockHash::from_byte_array([33; 32]),
        }
        network.chainstate.install_chainstate_for_test(
            open_bitcoin_core::chainstate::Chainstate::from_snapshot(snapshot),
        );
        let before = state(&network);
        // Act
        let result = network.connect_stored_block(
            &child,
            ProofWork::from_le_bytes([255; 32]),
            2_000_000_000,
            verify_flags(),
            consensus_params(),
        );
        // Assert
        assert!(result.is_err());
        assert_eq!(state(&network), before);
    }
}

#[test]
fn phase165_work_authority_disconnect_bytes_and_common_ancestor_are_bound_before_preparation() {
    for mutation in 0..3 {
        // Arrange
        let (mut network, active, side) = fork(2, 2);
        let mut disconnect = [active[1].clone()];
        let mut replacements = anchored(&side);
        match mutation {
            0 => disconnect[0].transactions[0].lock_time += 1,
            1 => disconnect[0].header.nonce = disconnect[0].header.nonce.wrapping_add(1),
            _ => {
                replacements[0].block.header.previous_block_hash = block_hash(&active[1].header);
                mine_header(&mut replacements[0].block);
            }
        }
        let before = state(&network);
        // Act
        let result = network.reorg_to_branch(
            &disconnect,
            &replacements,
            context(),
            verify_flags(),
            consensus_params(),
        );
        // Assert
        assert!(result.is_err());
        assert_eq!(state(&network), before);
    }
}

#[test]
fn phase165_work_authority_stored_extension_earns_fixed_target_proof_despite_max_score() {
    // Arrange
    let (mut network, active, _) = fork(1, 0);
    let child = build_block(block_hash(&active[0].header), 1, 5_000_000_000);
    // Act
    let disposition = network
        .connect_stored_block(
            &child,
            ProofWork::from_le_bytes([255; 32]),
            2_000_000_000,
            verify_flags(),
            consensus_params(),
        )
        .expect("actual stored extension");
    // Assert
    let BlockConnectDisposition::Connected(position) = disposition else {
        panic!("expected a real connection");
    };
    assert_eq!(
        position.chain_work.to_le_bytes(),
        [
            4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0
        ]
    );
}

#[test]
fn phase165_work_authority_local_wrong_parent_refuses_before_candidate_retention() {
    // Arrange
    let (mut network, active, _) = fork(2, 0);
    let side = build_block(block_hash(&active[0].header), 1, 4_000_000_000);
    let before = state(&network);
    // Act
    let result = network.connect_local_block(&side, verify_flags(), consensus_params());
    // Assert
    assert!(result.is_err());
    assert_eq!(state(&network), before);
}

#[test]
fn phase165_work_authority_connect_capability_binds_actual_body_and_derived_position() {
    // Arrange
    let (mut network, active, _) = fork(1, 0);
    let first = build_block(block_hash(&active[0].header), 1, 4_000_000_000);
    let other = build_block(block_hash(&active[0].header), 1, 3_000_000_000);
    network
        .retain_checked_candidate(&first, consensus_params(), 2_000_000_000)
        .expect("checked first body");
    let authorization = network
        .authorize_connect(&first, consensus_params())
        .expect("actual source and body");
    let before = state(&network);
    // Act
    let result = network.chainstate.prepare_authorized_connect(
        authorization,
        &other,
        2_000_000_000,
        verify_flags(),
        consensus_params(),
    );
    let authorization = network
        .authorize_connect(&first, consensus_params())
        .expect("fresh capability");
    let position_result = authorization.check_position(
        network
            .chainstate
            .chainstate()
            .tip()
            .expect("actual different position"),
    );
    // Assert
    assert!(result.is_err());
    assert!(position_result.is_err());
    assert_eq!(state(&network), before);
}

#[test]
fn phase165_work_authority_matching_connect_grant_does_not_skip_context_validation() {
    // Arrange
    let (mut network, active, _) = fork(1, 0);
    let child = build_block(block_hash(&active[0].header), 1, 5_000_000_000);
    network
        .retain_checked_candidate(&child, consensus_params(), 2_000_000_000)
        .expect("checked body at legitimate time");
    let authorization = network
        .authorize_connect(&child, consensus_params())
        .expect("actual checked-body grant");
    let before = network.chainstate_snapshot().expect("actual active source");
    // Act
    let result = network.chainstate.prepare_authorized_connect(
        authorization,
        &child,
        0,
        verify_flags(),
        consensus_params(),
    );
    // Assert
    let Err(error) = result else {
        panic!("caller time still undergoes full validation");
    };
    assert!(error.to_string().contains("time-too-new"));
    assert_eq!(
        network.chainstate_snapshot().expect("no accepted mutation"),
        before
    );
}

#[test]
fn phase165_work_authority_reorg_capability_refuses_a_genuine_different_candidate_stage() {
    // Arrange
    let (network, active, side) = fork(2, 2);
    let disconnect = [active[1].clone()];
    let authorization = network
        .authorize_reorg(&disconnect, &anchored(&side), consensus_params())
        .expect("preferred actual candidate");
    let other_first = build_block(block_hash(&active[0].header), 1, 3_000_000_000);
    let other_second = build_block(block_hash(&other_first.header), 2, 3_000_000_000);
    let other = network
        .chainstate
        .chainstate()
        .stage_reorg(
            &disconnect,
            &anchored(&[other_first, other_second]),
            verify_flags(),
            consensus_params(),
        )
        .expect("genuine differently checked stage on same source");
    let before = state(&network);
    // Act
    let result = authorization.check_stage(network.chainstate.chainstate(), &other);
    // Assert
    assert!(result.is_err());
    assert_eq!(state(&network), before);
}

#[test]
fn phase165_work_authority_invalid_target_or_body_cannot_earn_reorg_preference() {
    for invalid_target in [false, true] {
        // Arrange
        let (mut network, active, side) = fork(2, 2);
        let mut replacements = anchored(&side);
        if invalid_target {
            replacements[0].block.header.bits = 0;
        } else {
            replacements[0].block.transactions.clear();
        }
        let before = state(&network);
        // Act
        let result = network.reorg_to_branch(
            &[active[1].clone()],
            &replacements,
            context(),
            verify_flags(),
            consensus_params(),
        );
        // Assert
        assert!(result.is_err());
        assert_eq!(state(&network), before);
    }
}

#[test]
fn phase165_work_authority_attribution_does_not_reclassify_non_consensus_refusals() {
    for cause in [
        open_bitcoin_core::chainstate::ChainstateError::CoinsStorage {
            detail: "actual backend refusal".to_owned(),
        },
        open_bitcoin_core::chainstate::ChainstateError::StageOwnerMismatch,
    ] {
        // Arrange
        let original =
            crate::SyncRuntimeError::from(crate::ManagedNetworkError::Chainstate(cause.clone()));
        let wrapped = crate::ManagedNetworkError::Chainstate(
            open_bitcoin_core::chainstate::ChainstateError::ReplacementBlock {
                block_hash: BlockHash::from_byte_array([42; 32]),
                source: Box::new(cause),
            },
        );
        // Act
        let converted = crate::SyncRuntimeError::from(wrapped);
        // Assert
        assert_eq!(converted, original);
        assert!(matches!(converted, crate::SyncRuntimeError::Network { .. }));
    }
}
