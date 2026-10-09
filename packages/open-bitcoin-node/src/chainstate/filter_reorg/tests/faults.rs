// Parity breadcrumbs:
// - packages/bitcoin-knots/src/index/base.cpp
// - packages/bitcoin-knots/src/index/blockfilterindex.cpp
// - packages/bitcoin-knots/src/validation.cpp

use super::*;
use crate::storage::fjall_store::filters::FilterPublicationFault;

#[test]
fn phase158_manager_reorg_owned_pending_flush_is_revoked_by_preview() {
    // Arrange
    let mut fixture = ReorgFixture::new("manager-pending-preview", 3);
    let path = fixture.path().to_owned();
    fixture
        .manager
        .initialize_basic_index_owner(ReorgFixture::budget())
        .expect("owner");
    let pending = fixture
        .manager
        .maybe_validated_lineage
        .as_ref()
        .expect("tracked")
        .maybe_prepare(fixture.manager.chainstate.tip())
        .expect("prepare")
        .expect("owned");
    let prepared = prepared(&fixture, 2);
    // Act
    fixture
        .manager
        .install_prepared_reorg_preview(&prepared)
        .expect("preview");
    let result = pending.complete(
        fixture.manager.maybe_validated_lineage.as_ref(),
        fixture.manager.chainstate.tip(),
    );
    // Assert
    assert!(result.is_err());
    assert!(fixture.store.maybe_basic_filter_append_proof().is_err());
    drop(prepared);
    drop(fixture);
    std::fs::remove_dir_all(path).expect("cleanup");
}

#[test]
fn phase158_manager_reorg_next_genuine_connect_advances_visible_accepted_target() {
    // Arrange
    let mut fixture = ReorgFixture::new("manager-next-connect", 3);
    let path = fixture.path().to_owned();
    fixture
        .manager
        .initialize_basic_index_owner(ReorgFixture::budget())
        .expect("owner");
    let prepared = prepared(&fixture, 2);
    let (longer, records) = fixture.stage(3);
    let hash = records[2].identity().block_hash();
    let block = fixture
        .store
        .load_block(hash)
        .expect("body")
        .expect("retained");
    drop(longer);
    fixture
        .manager
        .commit_prepared_reorg(prepared)
        .expect("replacement");
    // Act
    fixture
        .manager
        .connect_block_with_current_time(
            &block,
            20,
            i64::from(block.header.time) + 1,
            ScriptVerifyFlags::P2SH,
            ConsensusParams {
                coinbase_maturity: 1,
                ..Default::default()
            },
        )
        .expect("genuine next connect");
    // Assert
    assert_eq!(
        fixture.manager.maybe_basic_index_accepted_target(),
        Some(AcceptedIndexTarget::new(3, hash))
    );
    assert_eq!(
        fixture
            .manager
            .maybe_basic_index_progress()
            .expect("progress")
            .accepted_target(),
        AcceptedIndexTarget::new(3, hash)
    );
    drop(fixture);
    std::fs::remove_dir_all(path).expect("cleanup");
}

#[test]
fn phase158_manager_reorg_public_direct_path_uses_same_owner_transition() {
    // Arrange
    let mut fixture = ReorgFixture::new("manager-direct", 3);
    let path = fixture.path().to_owned();
    fixture
        .manager
        .initialize_basic_index_owner(ReorgFixture::budget())
        .expect("owner");
    let (stage, records) = fixture.stage(2);
    let disconnect = stage
        .transition()
        .disconnected
        .iter()
        .map(|p| {
            fixture
                .store
                .load_block(p.block_hash)
                .expect("body")
                .expect("retained")
        })
        .collect::<Vec<_>>();
    let replacement = stage
        .transition()
        .connected
        .iter()
        .map(|p| AnchoredBlock {
            block: fixture
                .store
                .load_block(p.block_hash)
                .expect("body")
                .expect("retained"),
            chain_work: p.chain_work,
        })
        .collect::<Vec<_>>();
    let old = fixture.manager.maybe_basic_index_progress().expect("owner");
    drop(stage);
    // Act
    fixture
        .manager
        .reorg(
            &disconnect,
            &replacement,
            ScriptVerifyFlags::P2SH,
            ConsensusParams {
                coinbase_maturity: 1,
                ..Default::default()
            },
        )
        .expect("direct reorg");
    // Assert
    let progress = fixture
        .manager
        .maybe_basic_index_progress()
        .expect("same owner");
    assert_eq!(progress.generation().value(), old.generation().value() + 1);
    assert_eq!(
        progress.accepted_target().block_hash(),
        records[1].identity().block_hash()
    );
    assert_eq!(
        progress.maybe_safe_durable_endpoint(),
        Some(fixture.records[0].identity())
    );
    assert!(progress.prepare_turn().is_ok());
    drop(fixture);
    std::fs::remove_dir_all(path).expect("cleanup");
}

#[test]
fn phase158_manager_reorg_commit_persist_failure_keeps_achieved_accepted_target() {
    for fault in [
        FilterPublicationFault::BeforeUndo,
        FilterPublicationFault::BeforeCoins,
        FilterPublicationFault::BeforeChainMeta,
    ] {
        // Arrange: genuine ordinary pressure causes the existing IfNeeded cadence to flush.
        let mut fixture = ReorgFixture::new("manager-commit-persist-fault", 3);
        let path = fixture.path().to_owned();
        fixture
            .manager
            .initialize_basic_index_owner(ReorgFixture::budget())
            .expect("owner");
        let prepared = prepared(&fixture, 2);
        let target = prepared
            .transition()
            .connected
            .last()
            .map(|p| (p.height, p.block_hash));
        fixture.manager.flush_lifecycle =
            FlushLifecycle::ready_for_test(0, 0, FlushPolicyTime::from_unix_seconds(0), true);
        fixture.store.set_basic_filter_fault(fault);
        // Act
        let result = fixture.manager.commit_prepared_reorg(prepared);
        // Assert: actual absorption and guarded publication preceded this persistence error.
        assert!(result.is_err());
        let owner = fixture
            .manager
            .maybe_basic_index_owner
            .as_ref()
            .expect("owner");
        assert_eq!(
            owner.reorg_state,
            BasicIndexReorgState::AcceptedReplacement {
                maybe_target: target
            }
        );
        let (height, hash) = target.expect("nonempty replacement");
        assert_eq!(
            owner.progress.accepted_target(),
            AcceptedIndexTarget::new(height, hash)
        );
        assert_eq!(
            owner.progress.maybe_safe_durable_endpoint(),
            Some(fixture.records[0].identity())
        );
        assert_eq!(
            owner.maybe_failure,
            Some(super::super::super::filter_index::AcceptedBasicIndexFailure::Persistence)
        );
        assert!(owner.progress.prepare_turn().is_err());
        assert_eq!(
            fixture
                .manager
                .chainstate
                .tip()
                .map(|p| (p.height, p.block_hash)),
            target
        );
        drop(fixture);
        std::fs::remove_dir_all(path).expect("cleanup");
    }
}

#[test]
fn phase158_manager_reorg_body_adapter_error_preserves_accepted_shared_safe() {
    // Arrange: bodies are the concrete runtime adapter's responsibility, outside flush_window.
    let mut fixture = ReorgFixture::new("manager-body-adapter-fault", 3);
    let path = fixture.path().to_owned();
    fixture
        .manager
        .initialize_basic_index_owner(ReorgFixture::budget())
        .expect("owner");
    let prepared = prepared(&fixture, 2);
    let replacement_hash = prepared.transition().connected[0].block_hash;
    let body = fixture
        .store
        .load_block(replacement_hash)
        .expect("body")
        .expect("retained");
    fixture
        .manager
        .commit_prepared_reorg(prepared)
        .expect("accepted unflushed");
    let before = fixture
        .manager
        .maybe_basic_index_progress()
        .expect("progress");
    fixture
        .store
        .set_basic_filter_fault(FilterPublicationFault::BeforeBody);
    // Act
    let body_result = fixture
        .store
        .save_block(&body, crate::storage::PersistMode::Sync);
    let flush_result = fixture.manager.flush_with_mode(
        FlushMode::Always,
        FlushPolicyTime::from_unix_seconds(10),
        u64::MAX,
    );
    // Assert
    assert!(body_result.is_err());
    assert!(flush_result.is_err());
    let owner = fixture
        .manager
        .maybe_basic_index_owner
        .as_ref()
        .expect("owner");
    assert_eq!(owner.progress.accepted_target(), before.accepted_target());
    assert_eq!(
        owner.progress.maybe_safe_durable_endpoint(),
        before.maybe_safe_durable_endpoint()
    );
    assert_eq!(owner.progress.protection(), before.protection());
    assert_eq!(
        owner.maybe_failure,
        Some(super::super::super::filter_index::AcceptedBasicIndexFailure::Persistence)
    );
    drop(fixture);
    std::fs::remove_dir_all(path).expect("cleanup");
}
