// Parity breadcrumbs:
// - packages/bitcoin-knots/src/index/base.cpp
// - packages/bitcoin-knots/src/index/blockfilterindex.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp

use super::*;

fn replacement_case(indexed: usize, count: usize) {
    // Arrange: startup itself performs its real eight-block turn.
    let fixture = ForkFixture::compact(16, 8, indexed);
    let before = fixture
        .runtime
        .network
        .maybe_basic_index_progress()
        .expect("read")
        .expect("owner");
    let observed = before
        .maybe_processed_endpoint()
        .expect("startup prefix")
        .height();
    let branch = fixture.fork(10, count, 0x61);
    let old_tip = fixture.history.records[23].identity();
    // Act
    fixture.apply(&branch);
    let replaced = fixture
        .runtime
        .network
        .maybe_basic_index_progress()
        .expect("read")
        .expect("restored owner");
    let expected = branch
        .records
        .last()
        .unwrap_or(&fixture.history.records[10])
        .identity();
    assert_eq!(
        replaced.accepted_target().block_hash(),
        expected.block_hash()
    );
    assert_eq!(replaced.accepted_target().height(), expected.height());
    assert_eq!(
        replaced.current_lag(),
        u64::from(expected.height() - observed.min(10))
    );
    assert_ne!(replaced.generation(), before.generation());
    assert_ne!(replaced.branch_identity(), before.branch_identity());
    assert_eq!(
        replaced
            .maybe_processed_endpoint()
            .expect("common prefix")
            .height(),
        observed.min(10)
    );
    assert_eq!(
        replaced.initially_synchronized(),
        before.initially_synchronized()
    );
    assert_eq!(
        fixture
            .runtime
            .store()
            .coins_view()
            .best_block()
            .expect("displaced durable coins unchanged"),
        Some(old_tip.block_hash())
    );
    fixture.finish();
    let caught_up = fixture
        .runtime
        .network
        .maybe_basic_index_progress()
        .expect("read")
        .expect("owner");
    assert!(
        caught_up
            .maybe_safe_durable_endpoint()
            .is_none_or(|endpoint| endpoint.height() <= 10)
    );
    fixture.flush();
    // Assert
    assert_active(&fixture, &branch, 10);
    if let Some(last) = branch.records.last() {
        assert_ne!(last.identity().filter_header(), old_tip.filter_header());
        assert_eq!(
            branch.records[0].identity().previous_header(),
            fixture.history.records[10].identity().filter_header()
        );
    }
    fixture.cleanup();
}

#[test]
fn phase158_validated_reorg_equal_height_index_before_ancestor() {
    replacement_case(0, 13);
}

#[test]
fn phase158_validated_reorg_equal_height_index_within_displaced_suffix() {
    replacement_case(8, 13);
}

#[test]
fn phase158_validated_reorg_equal_height_index_at_old_tip() {
    replacement_case(24, 13);
}

#[test]
fn phase158_validated_reorg_longer_replacement() {
    replacement_case(24, 16);
}

#[test]
fn phase158_validated_reorg_shorter_replacement() {
    replacement_case(24, 4);
}

#[test]
fn phase158_validated_reorg_disconnect_only_to_common_ancestor() {
    replacement_case(24, 0);
}

#[test]
fn phase158_validated_reorg_full_disconnect_has_empty_checkpoint_and_no_fabricated_target() {
    // Arrange
    let fixture = ForkFixture::compact(16, 8, 24);
    let branch = ForkBranch {
        disconnect: fixture.history.blocks.iter().rev().cloned().collect(),
        connected: Vec::new(),
        records: Vec::new(),
    };
    // Act
    fixture.apply(&branch);
    let turn = fixture
        .runtime
        .network
        .drive_basic_filter_index_turn()
        .expect("ordinary empty maintenance");
    // Assert
    assert_eq!(turn.maybe_accepted_target, None);
    assert_eq!(turn.maybe_accepted_lag, None);
    assert_eq!(turn.work.blocks, 0);
    assert_eq!(
        fixture
            .runtime
            .store()
            .maybe_basic_filter_checkpoint()
            .expect("empty checkpoint"),
        Some(FilterCheckpoint::new(IndexPrefix::Empty))
    );
    assert_eq!(
        fixture
            .runtime
            .store()
            .maybe_active_basic_filter_record(0)
            .expect("empty active projection"),
        None
    );
    for record in &fixture.history.records {
        assert_eq!(
            fixture
                .runtime
                .store()
                .load_basic_filter_record(record.identity().block_hash())
                .expect("immutable empty-branch retention"),
            Some(record.clone())
        );
    }
    fixture.cleanup();
}

pub(super) fn assert_active(fixture: &ForkFixture, branch: &ForkBranch, ancestor: usize) {
    for record in fixture.history.records[..=ancestor]
        .iter()
        .chain(&branch.records)
    {
        assert_eq!(
            fixture
                .runtime
                .store()
                .maybe_active_basic_filter_record(record.identity().height())
                .expect("active height"),
            Some(record.clone())
        );
        assert_eq!(
            fixture
                .runtime
                .store()
                .load_basic_filter_record(record.identity().block_hash())
                .expect("exact hash"),
            Some(record.clone())
        );
    }
    let endpoint = branch
        .records
        .last()
        .unwrap_or(&fixture.history.records[ancestor]);
    assert_eq!(
        fixture
            .runtime
            .store()
            .maybe_basic_filter_checkpoint()
            .expect("checkpoint"),
        Some(checkpoint(endpoint))
    );
    assert_eq!(
        fixture
            .runtime
            .store()
            .coins_view()
            .best_block()
            .expect("coins B"),
        Some(endpoint.identity().block_hash())
    );
    assert_eq!(
        fixture
            .runtime
            .store()
            .maybe_active_basic_filter_record(endpoint.identity().height() + 1)
            .expect("hidden old suffix"),
        None
    );
}

#[test]
fn phase158_validated_reorg_stale_owned_turn_cannot_publish() {
    // Arrange
    let fixture = ForkFixture::compact(16, 8, 24);
    let proof = fixture
        .runtime
        .store()
        .maybe_basic_filter_append_proof_with_budget(
            crate::chainstate::basic_filter_turn_budget()
                .expect("production budget")
                .normal(),
        )
        .expect("proof")
        .expect("live");
    let old_turn = fixture
        .runtime
        .store()
        .prepare_basic_filter_append(proof, &[])
        .expect("real owned turn");
    let branch = fixture.fork(10, 13, 0x62);
    // Act
    fixture.apply(&branch);
    let stale = fixture
        .runtime
        .store()
        .complete_basic_filter_append(old_turn);
    // Assert
    assert!(stale.is_err());
    fixture.finish();
    fixture.flush();
    assert_active(&fixture, &branch, 10);
    fixture.cleanup();
}

#[test]
fn phase158_validated_reorg_shorter_unflushed_return_preserves_coins_fence_and_reopens() {
    // Arrange: A23 -> B14 leaves the actual durable coins checkpoint at A23.
    let fixture = ForkFixture::compact(16, 8, 24);
    let branch = fixture.fork(10, 4, 0x6a);
    fixture.apply(&branch);
    fixture.finish();
    // Retain required sources independently; this does not flush coins or metadata.
    let accepted = fixture
        .runtime
        .network
        .chainstate_snapshot()
        .expect("accepted B snapshot");
    for anchored in &branch.connected {
        let hash = block_hash(&anchored.block.header);
        fixture
            .runtime
            .store()
            .save_undo(hash, &accepted.undo_by_block[&hash], PersistMode::Sync)
            .expect("retained genuine B undo");
    }
    let original = ForkBranch {
        disconnect: branch
            .connected
            .iter()
            .rev()
            .map(|a| a.block.clone())
            .collect(),
        connected: fixture.history.blocks[11..]
            .iter()
            .enumerate()
            .map(|(offset, block)| AnchoredBlock {
                block: block.clone(),
                chain_work: offset as u128 + 2_000,
            })
            .collect(),
        records: fixture.history.records[11..].to_vec(),
    };
    let original_tip = fixture.history.records[23].identity().block_hash();
    drop(accepted);
    assert_eq!(
        fixture
            .runtime
            .store()
            .coins_view()
            .best_block()
            .expect("actual coins A23"),
        Some(original_tip)
    );
    let before = fixture
        .runtime
        .network
        .maybe_basic_index_progress()
        .expect("read")
        .expect("owner");
    assert_eq!(before.accepted_target().height(), 14);
    assert_eq!(
        before
            .maybe_safe_durable_endpoint()
            .expect("shared safe prefix")
            .height(),
        10
    );

    // Act: genuine B14 -> A23 acceptance before any ordinary coins flush.
    fixture.apply(&original);
    let rewound = fixture
        .runtime
        .network
        .maybe_basic_index_progress()
        .expect("read")
        .expect("owner");
    assert_eq!(
        rewound
            .maybe_processed_endpoint()
            .expect("shared processed prefix")
            .height(),
        10
    );
    assert_eq!(
        rewound
            .maybe_safe_durable_endpoint()
            .expect("shared safe prefix")
            .height(),
        10
    );
    assert!(
        rewound
            .protection()
            .covers(IndexInputProtection::FromHeight(11))
    );
    fixture.finish();

    // Assert: exact A history earns the already durable A fence during ordinary catch-up.
    assert_eq!(
        fixture
            .runtime
            .store()
            .coins_view()
            .best_block()
            .expect("unchanged actual coins A23"),
        Some(original_tip)
    );
    let returned = fixture
        .runtime
        .network
        .maybe_basic_index_progress()
        .expect("read")
        .expect("owner");
    assert_eq!(returned.accepted_target().height(), 23);
    assert_eq!(returned.current_lag(), 0);
    assert_eq!(
        returned
            .maybe_safe_durable_endpoint()
            .expect("already durable A prefix")
            .height(),
        23
    );
    assert_eq!(
        fixture
            .runtime
            .store()
            .maybe_basic_filter_checkpoint()
            .expect("safe checkpoint"),
        Some(checkpoint(&fixture.history.records[23]))
    );
    for record in &fixture.history.records {
        assert_eq!(
            fixture
                .runtime
                .store()
                .maybe_active_basic_filter_record(record.identity().height())
                .expect("original active projection"),
            Some(record.clone())
        );
    }
    for record in &branch.records {
        assert_eq!(
            fixture
                .runtime
                .store()
                .load_basic_filter_record(record.identity().block_hash())
                .expect("displaced B immutable"),
            Some(record.clone())
        );
    }
    fixture.flush();
    assert_active(&fixture, &original, 10);
    let fixture = fixture.reopen();
    assert_active(&fixture, &original, 10);
    for record in &branch.records {
        assert_eq!(
            fixture
                .runtime
                .store()
                .load_basic_filter_record(record.identity().block_hash())
                .expect("reopened B immutable"),
            Some(record.clone())
        );
    }
    fixture.cleanup();
}

#[test]
fn phase158_validated_reorg_return_to_original_then_subsequent_stored_connect() {
    // Arrange
    let fixture = ForkFixture::compact(16, 8, 24);
    let branch = fixture.fork(10, 13, 0x63);
    fixture.apply(&branch);
    fixture.finish();
    fixture.flush();
    let original = ForkBranch {
        disconnect: branch
            .connected
            .iter()
            .rev()
            .map(|a| a.block.clone())
            .collect(),
        connected: fixture.history.blocks[11..]
            .iter()
            .enumerate()
            .map(|(offset, block)| AnchoredBlock {
                block: block.clone(),
                chain_work: offset as u128 + 2_000,
            })
            .collect(),
        records: fixture.history.records[11..].to_vec(),
    };
    // Act: A -> B -> A through ordinary authority, without reopen or re-enable.
    fixture.apply(&original);
    fixture.finish();
    fixture.flush();
    assert_active(&fixture, &original, 10);
    let child_branch = fixture.fork(23, 1, 0x64);
    let anchored = &child_branch.connected[0];
    fixture
        .runtime
        .store()
        .save_block(&anchored.block, PersistMode::Sync)
        .expect("actual later stored body");
    let connected = fixture
        .runtime
        .network
        .connect_stored_block(
            &anchored.block,
            anchored.chain_work,
            50_000,
            fixture.runtime.verify_flags,
            fixture.runtime.consensus_params,
        )
        .expect("ordinary subsequent connect");
    // Assert
    assert!(matches!(
        connected,
        crate::network::BlockConnectDisposition::Connected(_)
    ));
    fixture.finish();
    fixture.flush();
    assert_active(&fixture, &child_branch, 23);
    for record in &branch.records {
        assert_eq!(
            fixture
                .runtime
                .store()
                .load_basic_filter_record(record.identity().block_hash())
                .expect("B immutable"),
            Some(record.clone())
        );
    }
    fixture.cleanup();
}

#[test]
fn phase158_validated_reorg_actual_default_maturity_spends_flush_and_reopen() {
    // Arrange: heights 1..100 are rewards only; 101 spends reward 1 and its child.
    let fixture = ForkFixture::standard();
    assert_eq!(fixture.runtime.consensus_params.coinbase_maturity, 100);
    assert!(
        fixture.history.blocks[1..=100]
            .iter()
            .all(|block| block.transactions.len() == 1)
    );
    assert_eq!(fixture.history.blocks[101].transactions.len(), 3);
    let branch = fixture.fork(100, 6, 0x65);
    // Act
    fixture.apply(&branch);
    fixture.finish();
    fixture.flush();
    let fixture = fixture.reopen();
    // Assert: reopened runtime still supplies its untouched actual params/flags.
    assert_eq!(fixture.runtime.consensus_params.coinbase_maturity, 100);
    assert_eq!(fixture.runtime.verify_flags, ScriptVerifyFlags::P2SH);
    assert_active(&fixture, &branch, 100);
    for record in &fixture.history.records[101..] {
        assert_eq!(
            fixture
                .runtime
                .store()
                .load_basic_filter_record(record.identity().block_hash())
                .expect("standard displaced row"),
            Some(record.clone())
        );
    }
    fixture.cleanup();
}

#[test]
fn phase158_validated_reorg_sync_reconciliation_and_ordinary_live_resume() {
    // Arrange: actual live headers entry, with stored bodies as in sync reception.
    let mut fixture = ForkFixture::compact(16, 8, 24);
    let branch = fixture.fork(10, 16, 0x69);
    for anchored in &branch.connected {
        fixture
            .runtime
            .store()
            .save_block(&anchored.block, PersistMode::Sync)
            .expect("received stored body");
    }
    let peer = 158;
    fixture
        .runtime
        .network
        .add_inbound_peer(peer)
        .expect("peer");
    for message in [
        WireNetworkMessage::Version(VersionMessage {
            start_height: 26,
            nonce: 158_005,
            ..Default::default()
        }),
        WireNetworkMessage::Verack,
        WireNetworkMessage::Headers(HeadersMessage {
            headers: branch
                .connected
                .iter()
                .map(|a| a.block.header.clone())
                .collect(),
        }),
    ] {
        fixture
            .runtime
            .network
            .receive_sync_message(
                peer,
                message,
                50_000,
                fixture.runtime.verify_flags,
                fixture.runtime.consensus_params,
            )
            .expect("ordinary live header reception");
    }
    // Act
    let reconciled =
        crate::sync::block_reconcile::reconcile_best_chain(&mut fixture.runtime, 50_000)
            .expect("actual sync reconciliation");
    fixture.finish();
    fixture.flush();
    // Assert
    assert!(
        matches!(reconciled, SyncReconcileProgress::ReorgPersisted(_)),
        "{reconciled:?}"
    );
    assert_active(&fixture, &branch, 10);
    fixture.cleanup();
}
