// Parity breadcrumbs:
// - packages/bitcoin-knots/src/index/base.cpp
// - packages/bitcoin-knots/src/index/blockfilterindex.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp

//! Genuine retained inputs and configured recovery at actual writer boundaries.

use super::*;
use crate::storage::fjall_store::filters::FilterPublicationFault;

fn retained(fixture: &ForkFixture, records: &[StoredFilterRecord]) {
    for record in records {
        assert_eq!(
            fixture
                .runtime
                .store()
                .load_basic_filter_record(record.identity().block_hash())
                .expect("immutable read"),
            Some(record.clone()),
        );
    }
}

fn snapshot_basic(path: &Path) -> Vec<(Vec<u8>, Vec<u8>)> {
    snapshot_index(path)
        .into_iter()
        .filter(|(key, _)| key.starts_with(b"basic_filter:"))
        .collect()
}

fn save_branch(fixture: &ForkFixture, branch: &ForkBranch) {
    for anchored in &branch.connected {
        fixture
            .runtime
            .store()
            .save_block(&anchored.block, PersistMode::Sync)
            .expect("received genuine body");
    }
}

fn reorg(
    fixture: &ForkFixture,
    branch: &ForkBranch,
) -> Result<
    open_bitcoin_core::chainstate::ChainTransition,
    crate::network::ManagedNetworkAuthorityError,
> {
    fixture.runtime.network.reorg_to_branch(
        &branch.disconnect,
        &branch.connected,
        ReorgLifecycleContext::new(PolicyTime::from_unix_seconds(50_000)),
        fixture.runtime.verify_flags,
        fixture.runtime.consensus_params,
    )
}

fn finish_recovered(
    fixture: ForkFixture,
    old: &[StoredFilterRecord],
    new: &[StoredFilterRecord],
    expected: &[StoredFilterRecord],
    maybe_physical: Option<&StoredFilterRecord>,
) {
    let fixture = fixture.reopen_inspecting(|path| {
        let rows = snapshot_index(path);
        for record in old.iter().chain(new) {
            let key = codec::record_key(record.identity().block_hash());
            assert_eq!(
                rows.iter()
                    .find(|(stored, _)| stored == key.as_bytes())
                    .map(|(_, bytes)| bytes),
                Some(&codec::encode_record(record))
            );
        }
        if let Some(physical) = maybe_physical {
            let key = codec::active_key(physical.identity().height());
            assert_eq!(
                rows.iter()
                    .find(|(stored, _)| stored == key.as_bytes())
                    .map(|(_, bytes)| codec::decode_projection(&key, bytes)
                        .expect("physical projection")),
                Some(physical.identity().block_hash())
            );
        }
    });
    for turn in 0..256 {
        let outcome = fixture
            .runtime
            .network
            .drive_basic_filter_index_turn()
            .expect("ordinary recovered bounded turn");
        let progress = outcome.maybe_progress.expect("recovered owner");
        let protection = progress.maybe_safe_durable_endpoint().map_or(
            IndexInputProtection::FromHeight(0),
            |id| {
                checkpoint(
                    expected
                        .iter()
                        .find(|record| record.identity() == id)
                        .expect("safe on recovered canonical branch"),
                )
                .input_protection()
            },
        );
        assert!(progress.protection().covers(protection));
        assert!(
            fixture
                .runtime
                .store()
                .load_prune_locks()
                .expect("conservative locks")
                .contains(
                    &progress
                        .protection()
                        .maybe_prune_lock()
                        .expect("finite required frontier")
                )
        );
        if outcome.maybe_accepted_lag == Some(0) {
            break;
        }
        assert!(turn < 255, "finite recovery driver");
    }
    for record in expected {
        assert_eq!(
            fixture
                .runtime
                .store()
                .maybe_active_basic_filter_record(record.identity().height())
                .expect("canonical active"),
            Some(record.clone())
        );
    }
    let progress = fixture
        .runtime
        .network
        .maybe_basic_index_progress()
        .expect("owner")
        .expect("recovered owner");
    assert_eq!(progress.current_lag(), 0);
    assert_eq!(
        progress.maybe_safe_durable_endpoint(),
        expected.last().map(StoredFilterRecord::identity)
    );
    retained(&fixture, old);
    retained(&fixture, new);
    fixture.cleanup();
}

#[test]
fn phase158_reorg_failure_configured_reopen_index_ahead_of_old_coins_rebinds_conflicting_rows() {
    // Arrange
    let fixture = ForkFixture::compact(16, 24, 40);
    let old = fixture.history.records.clone();
    let branch = fixture.fork(10, 29, 0x71);
    fixture.apply(&branch);
    fixture.finish();
    assert_eq!(
        fixture
            .runtime
            .store()
            .coins_view()
            .best_block()
            .expect("old coins"),
        Some(old[39].identity().block_hash())
    );
    retained(&fixture, &branch.records);
    // Act / Assert: no retained stage, manual enable, or source repair.
    finish_recovered(
        fixture,
        &old,
        &branch.records,
        &old,
        Some(&branch.records[0]),
    );
}

#[test]
fn phase158_reorg_failure_configured_reopen_rewinds_checkpoint_with_conflicting_suffix() {
    // Arrange: A -> B own flush, then B -> A index-only; coins still B.
    let fixture = ForkFixture::compact(16, 24, 40);
    let old = fixture.history.records.clone();
    let branch = fixture.fork(10, 29, 0x72);
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
        records: old[11..].to_vec(),
    };
    fixture.apply(&original);
    fixture.finish();
    assert_eq!(
        fixture
            .runtime
            .store()
            .coins_view()
            .best_block()
            .expect("durable B"),
        Some(branch.records[28].identity().block_hash())
    );
    let canonical: Vec<_> = old[..=10].iter().chain(&branch.records).cloned().collect();
    // Act / Assert
    finish_recovered(fixture, &old, &branch.records, &canonical, Some(&old[11]));
}

#[test]
fn phase158_reorg_failure_configured_reopen_new_coins_partial_replacement_projection() {
    // Arrange: only first replacement turn overwrites A, while B earns own coins.
    let fixture = ForkFixture::compact(16, 24, 40);
    let old = fixture.history.records.clone();
    let branch = fixture.fork(10, 29, 0x73);
    fixture.apply(&branch);
    let turn = fixture
        .runtime
        .network
        .drive_basic_filter_index_turn()
        .expect("first bounded turn");
    assert_eq!(turn.work.blocks, 8);
    fixture.flush();
    assert!(
        fixture
            .runtime
            .network
            .maybe_basic_index_progress()
            .expect("owner")
            .expect("live")
            .current_lag()
            > 0
    );
    let written: Vec<_> = branch.records.iter().take(16).cloned().collect();
    retained(&fixture, &written);
    let canonical: Vec<_> = old[..=10].iter().chain(&branch.records).cloned().collect();
    // Act / Assert
    finish_recovered(fixture, &old, &written, &canonical, Some(&old[27]));
}

#[test]
fn phase158_reorg_failure_deep_required_body_and_undo_loss_precedes_preview_and_mempool() {
    for undo in [false, true] {
        // Arrange: build the genuine branch before deleting one actual durable mate.
        let fixture = ForkFixture::compact(128, 32, 160);
        let mut before_rows = Vec::new();
        let fixture = fixture.reopen_inspecting(|path| before_rows = snapshot_basic(path));
        let old = fixture.history.records.clone();
        let branch = fixture.fork(127, 32, 0x76);
        save_branch(&fixture, &branch);
        let state = fixture
            .runtime
            .network
            .chainstate_snapshot()
            .expect("before");
        let maybe_progress = fixture
            .runtime
            .network
            .maybe_basic_index_progress()
            .expect("before");
        let pool = fixture.runtime.network.mempool_info().expect("before");
        let locks = fixture.runtime.store().load_prune_locks().expect("before");
        let hash = old[140].identity().block_hash();
        fixture
            .runtime
            .store()
            .delete_basic_input_for_test(hash, undo)
            .expect("one missing mate");
        assert_eq!(fixture.runtime.store().has_block(hash).expect("body"), undo);
        assert_eq!(fixture.runtime.store().has_undo(hash).expect("undo"), !undo);
        // Act
        let error = reorg(&fixture, &branch)
            .expect_err("physical source required")
            .to_string();
        // Assert
        assert!(
            error.contains(if undo {
                "required undo"
            } else {
                "required body"
            }),
            "{error}"
        );
        assert_eq!(
            fixture
                .runtime
                .network
                .chainstate_snapshot()
                .expect("after"),
            state
        );
        assert_eq!(
            fixture
                .runtime
                .network
                .maybe_basic_index_progress()
                .expect("after"),
            maybe_progress
        );
        assert_eq!(fixture.runtime.network.mempool_info().expect("after"), pool);
        assert_eq!(
            fixture.runtime.store().load_prune_locks().expect("after"),
            locks
        );
        assert!(
            fixture
                .runtime
                .store()
                .maybe_basic_filter_append_proof()
                .expect("not preview frozen")
                .is_some()
        );
        retained(&fixture, &old);
        // Startup restores no deleted source; the full indexed prefix remains valid.
        let fixture =
            fixture.reopen_inspecting(|path| assert_eq!(snapshot_basic(path), before_rows));
        let error = reorg(&fixture, &branch)
            .expect_err("reopened cache cannot conceal loss")
            .to_string();
        assert!(
            error.contains(if undo {
                "missing undo data"
            } else {
                "required body"
            }),
            "{error}"
        );
        retained(&fixture, &old);
        fixture.cleanup();
    }
}

#[test]
fn phase158_reorg_failure_preview_mempool_refusal_has_no_accepted_receipt() {
    // Arrange
    let fixture = ForkFixture::compact(16, 8, 24);
    let branch = fixture.fork(10, 13, 0x77);
    save_branch(&fixture, &branch);
    let old = fixture.history.records.clone();
    let progress = fixture
        .runtime
        .network
        .maybe_basic_index_progress()
        .expect("before")
        .expect("owner");
    let pool = fixture.runtime.network.mempool_info().expect("before");
    let maybe_checkpoint = fixture
        .runtime
        .store()
        .maybe_basic_filter_checkpoint()
        .expect("before");
    let locks = fixture.runtime.store().load_prune_locks().expect("before");
    // Act
    let error = fixture
        .runtime
        .network
        .reorg_with_mempool_failure_for_test(
            &branch.disconnect,
            &branch.connected,
            fixture.runtime.verify_flags,
            fixture.runtime.consensus_params,
        )
        .expect_err("actual mempool seam")
        .to_string();
    // Assert: preview tip is visible, but accepted BASIC target remains displaced.
    assert!(error.contains("Serving"), "{error}");
    let after = fixture
        .runtime
        .network
        .maybe_basic_index_progress()
        .expect("after")
        .expect("owner");
    assert_eq!(after.accepted_target(), progress.accepted_target());
    assert_eq!(
        fixture
            .runtime
            .network
            .maybe_basic_index_accepted_target_for_test()
            .expect("explicit accepted identity"),
        Some(progress.accepted_target())
    );
    assert_eq!(after.generation(), progress.generation());
    assert_eq!(fixture.runtime.network.mempool_info().expect("after"), pool);
    assert_eq!(
        fixture
            .runtime
            .store()
            .maybe_basic_filter_checkpoint()
            .expect("after"),
        maybe_checkpoint
    );
    assert_eq!(
        fixture.runtime.store().load_prune_locks().expect("after"),
        locks
    );
    assert!(
        fixture
            .runtime
            .network
            .drive_basic_filter_index_turn()
            .is_err()
    );
    retained(&fixture, &old);
    finish_recovered(fixture, &old, &[], &old, None);
}

#[test]
fn phase158_reorg_failure_accepted_body_undo_coins_metadata_boundaries_reopen_truthfully() {
    for fault in [
        FilterPublicationFault::BeforeBody,
        FilterPublicationFault::BeforeUndo,
        FilterPublicationFault::BeforeCoins,
        FilterPublicationFault::BeforeChainMeta,
    ] {
        // Arrange
        let fixture = ForkFixture::compact(16, 8, 24);
        let old = fixture.history.records.clone();
        let branch = fixture.fork(10, 16, 0x78);
        fixture.apply(&branch);
        let before = fixture
            .runtime
            .network
            .maybe_basic_index_progress()
            .expect("before")
            .expect("accepted owner");
        fixture.runtime.store().set_basic_filter_fault(fault);
        // Act
        if fault == FilterPublicationFault::BeforeBody {
            assert!(
                fixture
                    .runtime
                    .store()
                    .save_block(&branch.connected[0].block, PersistMode::Sync)
                    .is_err()
            );
            fixture
                .runtime
                .network
                .note_basic_index_failure(
                    crate::chainstate::filter_index::AcceptedBasicIndexFailure::Persistence,
                )
                .expect("host error projection");
        } else {
            assert!(
                fixture
                    .runtime
                    .network
                    .flush_coins(
                        FlushMode::Always,
                        FlushPolicyTime::from_unix_seconds(50_001),
                        u64::MAX
                    )
                    .is_err()
            );
        }
        // Assert
        let after = fixture
            .runtime
            .network
            .maybe_basic_index_progress()
            .expect("after")
            .expect("accepted owner");
        assert_eq!(after.accepted_target(), before.accepted_target());
        assert_eq!(
            after.maybe_safe_durable_endpoint(),
            before.maybe_safe_durable_endpoint()
        );
        assert_eq!(after.protection(), before.protection());
        assert!(
            fixture
                .runtime
                .network
                .drive_basic_filter_index_turn()
                .is_err()
        );
        retained(&fixture, &old);
        if fault != FilterPublicationFault::BeforeChainMeta {
            finish_recovered(fixture, &old, &[], &old, None);
            continue;
        }
        // Coins changed before metadata: this is explicit H/B failure, never success.
        let ForkFixture {
            history, runtime, ..
        } = fixture;
        drop(runtime);
        let before = snapshot_index(&history.path);
        let result = DurableSyncRuntime::open_configured(
            FjallNodeStore::open(&history.path).expect("actual reopen"),
            sync_config(),
            BasicFilterStartupMode::Enabled,
        );
        assert_filter_refusal(result, "coins best block differs from durable metadata");
        assert_eq!(snapshot_index(&history.path), before);
        history.cleanup();
    }
}

#[test]
fn phase158_reorg_failure_rewind_publication_faults_preserve_actual_accepted_tip_and_reopen() {
    for fault in [
        FilterPublicationFault::BeforeRecords,
        FilterPublicationFault::BeforeCheckpoint,
        FilterPublicationFault::BeforeProtection,
        FilterPublicationFault::AfterCommit,
    ] {
        // Arrange
        let fixture = ForkFixture::compact(16, 8, 24);
        let old = fixture.history.records.clone();
        let branch = fixture.fork(10, 13, 0x74);
        save_branch(&fixture, &branch);
        fixture.runtime.store().set_basic_filter_fault(fault);
        // Act
        assert!(reorg(&fixture, &branch).is_err(), "{fault:?}");
        // Assert: absorption precedes the index disposition.
        assert_eq!(
            fixture
                .runtime
                .network
                .maybe_basic_index_accepted_target_for_test()
                .expect("explicit absorption identity")
                .expect("accepted branch")
                .block_hash(),
            branch.records[12].identity().block_hash()
        );
        assert_eq!(
            fixture
                .runtime
                .network
                .maybe_chain_tip()
                .expect("accepted tip")
                .expect("nonempty")
                .block_hash,
            branch.records[12].identity().block_hash()
        );
        assert!(
            fixture
                .runtime
                .network
                .drive_basic_filter_index_turn()
                .is_err()
        );
        retained(&fixture, &old);
        finish_recovered(fixture, &old, &[], &old, None);
    }
}

#[test]
fn phase158_reorg_failure_first_and_subsequent_append_faults_reopen_old_canonical_branch() {
    for subsequent in [false, true] {
        for fault in [
            FilterPublicationFault::BeforeRecords,
            FilterPublicationFault::BeforeCheckpoint,
            FilterPublicationFault::BeforeProtection,
            FilterPublicationFault::AfterCommit,
        ] {
            // Arrange
            let fixture = ForkFixture::compact(16, 24, 40);
            let old = fixture.history.records.clone();
            let branch = fixture.fork(10, 29, 0x75);
            fixture.apply(&branch);
            if subsequent {
                fixture
                    .runtime
                    .network
                    .drive_basic_filter_index_turn()
                    .expect("first turn");
            }
            fixture.runtime.store().set_basic_filter_fault(fault);
            // Act
            assert!(
                fixture
                    .runtime
                    .network
                    .drive_basic_filter_index_turn()
                    .is_err(),
                "{fault:?}"
            );
            // Assert
            assert_eq!(
                fixture
                    .runtime
                    .network
                    .maybe_chain_tip()
                    .expect("tip")
                    .expect("accepted")
                    .block_hash,
                branch.records[28].identity().block_hash()
            );
            let present: Vec<_> = branch
                .records
                .iter()
                .filter(|record| {
                    fixture
                        .runtime
                        .store()
                        .load_basic_filter_record(record.identity().block_hash())
                        .expect("actual row")
                        .is_some()
                })
                .cloned()
                .collect();
            retained(&fixture, &old);
            finish_recovered(fixture, &old, &present, &old, None);
        }
    }
}
