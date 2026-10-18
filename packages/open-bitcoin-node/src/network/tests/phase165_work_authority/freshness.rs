// Parity breadcrumbs:
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/chain.h

use super::*;

#[test]
fn phase165_work_authority_connect_grant_checks_epoch_before_preparation_and_commit() {
    for prepared in [false, true] {
        // Arrange
        let (mut network, active, _) = fork(1, 0);
        let child = build_block(block_hash(&active[0].header), 1, 5_000_000_000);
        network
            .retain_checked_candidate(&child, consensus_params(), 2_000_000_000)
            .expect("actual retained candidate");
        let mut maybe_authorization = Some(
            network
                .authorize_connect(&child, consensus_params())
                .expect("current actual eligibility"),
        );
        let maybe_prepared = prepared.then(|| {
            network
                .chainstate
                .prepare_authorized_connect(
                    maybe_authorization.take().expect("one grant"),
                    &child,
                    2_000_000_000,
                    verify_flags(),
                    consensus_params(),
                )
                .expect("actual prepared child")
        });
        let source = network
            .chainstate
            .chainstate()
            .bind_live_work_source(consensus_params())
            .expect("actual private source");
        network
            .peer_manager
            .mark_body_unavailable(block_hash(&child.header));
        let before = state(&network);
        // Act
        if let Some(authorization) = &maybe_authorization {
            assert!(
                authorization
                    .check_position(
                        network
                            .chainstate
                            .chainstate()
                            .tip()
                            .expect("actual position")
                    )
                    .is_err()
            );
        }
        let result = if let Some(prepared) = maybe_prepared {
            network
                .chainstate
                .commit_prepared_connect(prepared)
                .map(|_| ())
        } else {
            network
                .chainstate
                .prepare_authorized_connect(
                    maybe_authorization.expect("one grant"),
                    &child,
                    2_000_000_000,
                    verify_flags(),
                    consensus_params(),
                )
                .map(|_| ())
        };
        // Assert
        assert!(result.is_err());
        assert_eq!(state(&network), before);
        assert_eq!(
            network
                .chainstate
                .chainstate()
                .check_live_work_source(&source),
            Ok(())
        );
    }
}

#[test]
fn phase165_work_authority_replaced_registry_expires_old_grant_but_preserves_genuine_history() {
    // Arrange
    let (mut network, active, side) = fork(2, 2);
    let disconnect = [active[1].clone()];
    let replacements = anchored(&side);
    let authorization = network
        .authorize_reorg(&disconnect, &replacements, consensus_params())
        .expect("original registry grant");
    let replacement_registry = network.peer_manager.header_store().clone();
    network.peer_manager.seed_header_store(replacement_registry);
    let before = state(&network);
    // Act
    let old = network.chainstate.prepare_authorized_reorg(
        authorization,
        &disconnect,
        &replacements,
        verify_flags(),
        consensus_params(),
    );
    // Assert
    assert!(old.is_err());
    assert_eq!(state(&network), before);
    let fresh = network
        .authorize_reorg(&disconnect, &replacements, consensus_params())
        .expect("same immutable body events in fresh owner");
    let prepared = network
        .chainstate
        .prepare_authorized_reorg(
            fresh,
            &disconnect,
            &replacements,
            verify_flags(),
            consensus_params(),
        )
        .expect("new current grant");
    network
        .chainstate
        .commit_prepared_reorg(prepared)
        .expect("actual stronger acceptance");
}

#[test]
fn phase165_work_authority_own_checked_intake_is_one_consumed_continuation() {
    // Arrange
    let (mut network, active, side) = fork(2, 2);
    let disconnect = [active[1].clone()];
    let replacements = anchored(&side);
    let mut authorization = network
        .authorize_reorg(&disconnect, &replacements, consensus_params())
        .expect("matching current decision");
    // Act
    network
        .retain_authorized_reorg(&mut authorization)
        .expect("move the exact checked registry/payload");
    let before = state(&network);
    let duplicate = network.retain_authorized_reorg(&mut authorization);
    // Assert
    assert!(duplicate.is_err());
    assert_eq!(state(&network), before);
    let prepared = network
        .chainstate
        .prepare_authorized_reorg(
            authorization,
            &disconnect,
            &replacements,
            verify_flags(),
            consensus_params(),
        )
        .expect("only own moved intake continuation");
    network
        .chainstate
        .install_prepared_reorg_preview(&prepared)
        .expect("own preview");
    network
        .chainstate
        .commit_prepared_reorg(prepared)
        .expect("single matching acceptance");
}

#[test]
fn phase165_work_authority_stale_intake_cannot_refresh_itself_after_revocation() {
    // Arrange
    let (mut network, active, side) = fork(2, 2);
    let mut authorization = network
        .authorize_reorg(&[active[1].clone()], &anchored(&side), consensus_params())
        .expect("current decision");
    network
        .peer_manager
        .mark_body_failed(block_hash(&side[1].header));
    let before = state(&network);
    // Act
    let result = network.retain_authorized_reorg(&mut authorization);
    // Assert
    assert!(result.is_err());
    assert_eq!(state(&network), before);
}

#[test]
fn phase165_work_authority_registry_revocation_stales_prepare_preview_and_commit() {
    for point in 0..4 {
        for change in 0..3 {
            // Arrange
            let (mut network, active, side) = fork(2, 2);
            let disconnect = [active[1].clone()];
            let replacements = anchored(&side);
            let mut maybe_authorization = Some(
                network
                    .authorize_reorg(&disconnect, &replacements, consensus_params())
                    .expect("actual source eligibility"),
            );
            let maybe_prepared = (point != 0).then(|| {
                network
                    .chainstate
                    .prepare_authorized_reorg(
                        maybe_authorization.take().expect("one grant"),
                        &disconnect,
                        &replacements,
                        verify_flags(),
                        consensus_params(),
                    )
                    .expect("initially matching prepared stage")
            });
            if point == 3 {
                network
                    .chainstate
                    .install_prepared_reorg_preview(
                        maybe_prepared.as_ref().expect("own preview stage"),
                    )
                    .expect("own preview before eligibility change");
            }
            let maybe_source = (point != 3).then(|| {
                network
                    .chainstate
                    .chainstate()
                    .bind_live_work_source(consensus_params())
                    .expect("unchanged core source")
            });
            match change {
                0 => network
                    .peer_manager
                    .mark_body_failed(block_hash(&side[1].header)),
                1 => network
                    .peer_manager
                    .mark_body_unavailable(block_hash(&side[1].header)),
                _ => {
                    let other = build_block(block_hash(&active[0].header), 1, 2_000_000_000);
                    network
                        .retain_checked_candidate(&other, consensus_params(), 2_000_000_000)
                        .expect("new unrelated checked eligibility");
                }
            }
            let before = state(&network);
            // Act
            let result = match point {
                0 => network
                    .chainstate
                    .prepare_authorized_reorg(
                        maybe_authorization.expect("unconsumed grant"),
                        &disconnect,
                        &replacements,
                        verify_flags(),
                        consensus_params(),
                    )
                    .map(|_| ()),
                1 => network
                    .chainstate
                    .install_prepared_reorg_preview(maybe_prepared.as_ref().expect("prepared")),
                _ => network
                    .chainstate
                    .commit_prepared_reorg(maybe_prepared.expect("prepared"))
                    .map(|_| ()),
            };
            // Assert
            assert!(
                result.is_err(),
                "issuing eligibility changed without a core generation change"
            );
            assert_eq!(state(&network), before);
            if let Some(source) = maybe_source {
                assert_eq!(
                    network
                        .chainstate
                        .chainstate()
                        .check_live_work_source(&source),
                    Ok(())
                );
            }
        }
    }
}

#[test]
fn phase165_work_authority_read_duplicate_and_independent_clone_keep_issuing_grant_current() {
    // Arrange
    let (mut network, active, side) = fork(2, 2);
    let disconnect = [active[1].clone()];
    let replacements = anchored(&side);
    let authorization = network
        .authorize_reorg(&disconnect, &replacements, consensus_params())
        .expect("issuing preference");
    let mut independent = network.clone();
    independent
        .peer_manager
        .mark_body_failed(block_hash(&side[1].header));
    network
        .peer_manager
        .check_body_eligibility(&side[1], &consensus_params(), 2_000_000_000)
        .expect("unchanged duplicate");
    let _candidates = network.eligible_body_candidates();
    let _history = network.maybe_body_eligibility(block_hash(&side[1].header));
    // Act
    let prepared = network
        .chainstate
        .prepare_authorized_reorg(
            authorization,
            &disconnect,
            &replacements,
            verify_flags(),
            consensus_params(),
        )
        .expect("reads, duplicates and independent clone do not expire source");
    network
        .chainstate
        .install_prepared_reorg_preview(&prepared)
        .expect("preview");
    network
        .chainstate
        .commit_prepared_reorg(prepared)
        .expect("same authorized continuation");
    // Assert
    assert_eq!(
        network
            .maybe_chain_tip()
            .expect("genuine source acceptance")
            .block_hash,
        block_hash(&side[1].header)
    );
    assert!(
        !network
            .maybe_body_eligibility(block_hash(&side[1].header))
            .expect("source not revoked")
            .failed
    );
    assert!(
        independent
            .maybe_body_eligibility(block_hash(&side[1].header))
            .expect("clone alone revoked")
            .failed
    );
}
