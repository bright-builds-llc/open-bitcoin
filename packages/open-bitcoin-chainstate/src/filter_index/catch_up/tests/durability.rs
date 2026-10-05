// Parity breadcrumbs:
// - packages/bitcoin-knots/src/index/base.cpp
// - packages/bitcoin-knots/src/index/blockfilterindex.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp

use super::*;

#[test]
fn phase157_owner_achieved_bridge_preserves_initial_completion_during_later_lag() {
    // Arrange
    let (positions, genesis) = durable_genesis();
    let mut progress = BasicIndexProgress::new(
        IndexGeneration::new(4),
        hash(99),
        AcceptedIndexTarget::new(0, genesis.block_hash()),
        Some(genesis),
        Some(genesis),
        IndexInputProtection::FromHeight(1),
    )
    .expect("synced");
    progress
        .observe_validated_connect(
            AcceptedIndexTarget::new(1, positions[1].block_hash),
            genesis.block_hash(),
        )
        .expect("accepted child");
    // Act
    progress
        .confirm_achieved_checkpoint(
            IndexGeneration::new(4),
            hash(99),
            FilterCheckpoint::new(IndexPrefix::Committed(genesis)),
            IndexInputProtection::FromHeight(0),
        )
        .expect("achieved stronger covering lock");
    // Assert
    assert!(progress.initially_synchronized());
    assert_eq!(progress.current_lag(), 1);
    assert_eq!(progress.accepted_target().height(), 1);
    assert_eq!(progress.protection(), IndexInputProtection::FromHeight(0));
}

#[test]
fn phase157_owner_achieved_bridge_refuses_foreign_generation_atomically() {
    // Arrange
    let (_, genesis) = durable_genesis();
    let mut progress = BasicIndexProgress::new(
        IndexGeneration::new(4),
        hash(99),
        AcceptedIndexTarget::new(0, genesis.block_hash()),
        Some(genesis),
        Some(genesis),
        IndexInputProtection::FromHeight(1),
    )
    .expect("synced");
    let before = progress;
    // Act
    let result = progress.confirm_achieved_checkpoint(
        IndexGeneration::new(5),
        hash(99),
        FilterCheckpoint::new(IndexPrefix::Committed(genesis)),
        IndexInputProtection::FromHeight(1),
    );
    // Assert
    assert_eq!(result, Err(BasicIndexCatchUpError::StaleWork));
    assert_eq!(progress, before);
}

#[test]
fn phase157_owner_achieved_bridge_refuses_unprocessed_endpoint_atomically() {
    // Arrange
    let (positions, genesis) = durable_genesis();
    let child = position_record(&positions[1], Some(genesis));
    let mut progress = BasicIndexProgress::new(
        IndexGeneration::new(4),
        hash(99),
        AcceptedIndexTarget::new(1, child.block_hash()),
        Some(genesis),
        Some(genesis),
        IndexInputProtection::FromHeight(1),
    )
    .expect("behind");
    let before = progress;
    // Act
    let result = progress.confirm_achieved_checkpoint(
        IndexGeneration::new(4),
        hash(99),
        FilterCheckpoint::new(IndexPrefix::Committed(child)),
        IndexInputProtection::FromHeight(2),
    );
    // Assert
    assert!(result.is_err());
    assert_eq!(progress, before);
}

fn durable_genesis() -> (Vec<crate::ChainPosition>, FilterRecordIdentity) {
    let header = open_bitcoin_primitives::BlockHeader {
        version: 1,
        previous_block_hash: BlockHash::default(),
        merkle_root: Default::default(),
        time: 1,
        bits: 0x207f_ffff,
        nonce: 0,
    };
    let genesis_position = crate::ChainPosition::new(header.clone(), 0, 1, 1);
    let child = crate::ChainPosition::new(
        open_bitcoin_primitives::BlockHeader {
            previous_block_hash: genesis_position.block_hash,
            time: 2,
            ..header
        },
        1,
        2,
        2,
    );
    let positions = vec![genesis_position, child];
    let genesis = position_record(&positions[0], None);
    (positions, genesis)
}

fn position_record(
    position: &crate::ChainPosition,
    maybe_previous: Option<FilterRecordIdentity>,
) -> FilterRecordIdentity {
    let previous = maybe_previous.map_or(FilterHeader::default(), |r| r.filter_header());
    FilterRecordIdentity::new(
        position.height,
        position.block_hash,
        position.previous_block_hash(),
        FilterHash::default(),
        compute_filter_header(FilterHash::default(), previous),
        previous,
        maybe_previous.as_ref(),
    )
    .expect("validated position record")
}

#[test]
fn phase157_owner_only_verified_durable_checkpoint_releases_protection() {
    // Arrange
    let (positions, genesis) = durable_genesis();
    let fence = VerifiedChainstateFence::new(Some(positions[1].block_hash), Some(&positions))
        .expect("verified fence");
    let mut progress = BasicIndexProgress::new(
        IndexGeneration::new(4),
        hash(99),
        AcceptedIndexTarget::new(0, genesis.block_hash()),
        Some(genesis),
        None,
        IndexInputProtection::FromHeight(0),
    )
    .expect("processed only");
    let durability = ValidatedIndexDurability::new(
        IndexGeneration::new(4),
        hash(99),
        FilterCheckpoint::new(IndexPrefix::Committed(genesis)),
        &fence,
    )
    .expect("fenced endpoint");
    // Act
    progress
        .confirm_safe_durable(durability, IndexInputProtection::FromHeight(1))
        .expect("confirmed publication");
    // Assert
    assert_eq!(progress.maybe_safe_durable_endpoint(), Some(genesis));
    assert_eq!(progress.protection(), IndexInputProtection::FromHeight(1));
    assert!(progress.initially_synchronized());
    assert_eq!(progress.maybe_next_height(), Ok(None));
}

#[test]
fn phase157_owner_wrong_durable_fence_refuses() {
    // Arrange
    let (positions, _) = durable_genesis();
    let fence = VerifiedChainstateFence::new(Some(positions[1].block_hash), Some(&positions))
        .expect("fence");
    // Act
    let result = ValidatedIndexDurability::new(
        IndexGeneration::new(4),
        hash(99),
        FilterCheckpoint::new(IndexPrefix::Committed(record(0, None))),
        &fence,
    );
    // Assert
    assert_eq!(result, Err(BasicIndexCatchUpError::InvalidDurability));
}

#[test]
fn phase157_owner_durable_failure_combinations_preserve_state() {
    // Arrange
    let (positions, genesis) = durable_genesis();
    let child = position_record(&positions[1], Some(genesis));
    let fence = VerifiedChainstateFence::new(Some(positions[1].block_hash), Some(&positions))
        .expect("fence");
    let mut progress = BasicIndexProgress::new(
        IndexGeneration::new(4),
        hash(99),
        AcceptedIndexTarget::new(1, child.block_hash()),
        Some(genesis),
        Some(genesis),
        IndexInputProtection::FromHeight(1),
    )
    .expect("recovered");
    // Act / Assert
    for (generation, branch, checkpoint, protection, error) in [
        (
            5,
            hash(99),
            IndexPrefix::Committed(genesis),
            IndexInputProtection::FromHeight(1),
            BasicIndexCatchUpError::StaleWork,
        ),
        (
            4,
            hash(98),
            IndexPrefix::Committed(genesis),
            IndexInputProtection::FromHeight(1),
            BasicIndexCatchUpError::StaleWork,
        ),
        (
            4,
            hash(99),
            IndexPrefix::Committed(child),
            IndexInputProtection::FromHeight(2),
            BasicIndexCatchUpError::InvalidProgress,
        ),
        (
            4,
            hash(99),
            IndexPrefix::Empty,
            IndexInputProtection::FromHeight(0),
            BasicIndexCatchUpError::InvalidProgress,
        ),
        (
            4,
            hash(99),
            IndexPrefix::Committed(genesis),
            IndexInputProtection::FromHeight(2),
            BasicIndexCatchUpError::WeakProtection,
        ),
    ] {
        let before = progress;
        let validated = ValidatedIndexDurability::new(
            IndexGeneration::new(generation),
            branch,
            FilterCheckpoint::new(checkpoint),
            &fence,
        )
        .expect("corresponds to fence");
        assert_eq!(
            progress.confirm_safe_durable(validated, protection),
            Err(error)
        );
        assert_eq!(progress, before);
    }
}

#[test]
fn phase157_owner_paused_progress_cannot_confirm_durability() {
    // Arrange
    let (positions, genesis) = durable_genesis();
    let fence = VerifiedChainstateFence::new(Some(positions[1].block_hash), Some(&positions))
        .expect("fence");
    let validated = ValidatedIndexDurability::new(
        IndexGeneration::new(4),
        hash(99),
        FilterCheckpoint::new(IndexPrefix::Committed(genesis)),
        &fence,
    )
    .expect("validated endpoint");
    let mut progress = BasicIndexProgress::new(
        IndexGeneration::new(4),
        hash(99),
        AcceptedIndexTarget::new(0, genesis.block_hash()),
        Some(genesis),
        None,
        IndexInputProtection::FromHeight(0),
    )
    .expect("processed");
    progress.pause(BasicIndexPause::Persistence);
    let before = progress;
    // Act
    let result = progress.confirm_safe_durable(validated, IndexInputProtection::FromHeight(1));
    // Assert
    assert_eq!(result, Err(BasicIndexCatchUpError::StaleWork));
    assert_eq!(progress, before);
}

#[test]
fn phase157_owner_changed_durability_and_protection_invalidate_prepared_turn() {
    // Arrange
    let (positions, genesis) = durable_genesis();
    let fence = VerifiedChainstateFence::new(Some(positions[1].block_hash), Some(&positions))
        .expect("fence");
    let mut progress = BasicIndexProgress::new(
        IndexGeneration::new(4),
        hash(99),
        AcceptedIndexTarget::new(1, positions[1].block_hash),
        Some(genesis),
        None,
        IndexInputProtection::FromHeight(0),
    )
    .expect("processed");
    let prepared = progress.prepare_turn().expect("prepared");
    let validated = ValidatedIndexDurability::new(
        IndexGeneration::new(4),
        hash(99),
        FilterCheckpoint::new(IndexPrefix::Committed(genesis)),
        &fence,
    )
    .expect("validated endpoint");
    progress
        .confirm_safe_durable(validated, IndexInputProtection::FromHeight(1))
        .expect("durable");
    let before = progress;
    // Act
    let result = progress.complete_turn(prepared, &[position_record(&positions[1], Some(genesis))]);
    // Assert
    assert_eq!(result, Err(BasicIndexCatchUpError::StaleWork));
    assert_eq!(progress, before);
}

#[test]
fn phase157_owner_fence_rejects_wrong_parent_and_above_tip_endpoints() {
    // Arrange
    let (positions, genesis) = durable_genesis();
    let fence = VerifiedChainstateFence::new(Some(positions[1].block_hash), Some(&positions))
        .expect("fence");
    let wrong_parent = FilterRecordIdentity::new_with_predecessor_facts(
        1,
        positions[1].block_hash,
        hash(7),
        FilterHash::default(),
        compute_filter_header(FilterHash::default(), genesis.filter_header()),
        genesis.filter_header(),
        Some((0, hash(7), genesis.filter_header())),
    )
    .expect("local foreign edge");
    let child = position_record(&positions[1], Some(genesis));
    let above_tip = record(2, Some(child));
    // Act / Assert
    for endpoint in [wrong_parent, above_tip] {
        assert_eq!(
            ValidatedIndexDurability::new(
                IndexGeneration::new(4),
                hash(99),
                FilterCheckpoint::new(IndexPrefix::Committed(endpoint)),
                &fence
            ),
            Err(BasicIndexCatchUpError::InvalidDurability)
        );
    }
}

#[test]
fn phase157_owner_compatible_durability_advances_without_resetting_initial_completion() {
    // Arrange
    let (positions, genesis) = durable_genesis();
    let child = position_record(&positions[1], Some(genesis));
    let fence = VerifiedChainstateFence::new(Some(positions[1].block_hash), Some(&positions))
        .expect("fence");
    let mut progress = BasicIndexProgress::new(
        IndexGeneration::new(4),
        hash(99),
        AcceptedIndexTarget::new(1, child.block_hash()),
        Some(child),
        Some(genesis),
        IndexInputProtection::FromHeight(1),
    )
    .expect("processed ahead of safe checkpoint");
    let validated = ValidatedIndexDurability::new(
        IndexGeneration::new(4),
        hash(99),
        FilterCheckpoint::new(IndexPrefix::Committed(child)),
        &fence,
    )
    .expect("validated child");
    // Act
    progress
        .confirm_safe_durable(validated, IndexInputProtection::FromHeight(2))
        .expect("safe publication");
    // Assert
    assert_eq!(progress.maybe_safe_durable_endpoint(), Some(child));
    assert_eq!(progress.protection(), IndexInputProtection::FromHeight(2));
    assert!(progress.initially_synchronized());
}

#[test]
fn phase157_owner_protection_change_alone_invalidates_prepared_work() {
    // Arrange
    let (positions, genesis) = durable_genesis();
    let fence = VerifiedChainstateFence::new(Some(positions[1].block_hash), Some(&positions))
        .expect("fence");
    let validated = ValidatedIndexDurability::new(
        IndexGeneration::new(4),
        hash(99),
        FilterCheckpoint::new(IndexPrefix::Committed(genesis)),
        &fence,
    )
    .expect("validated endpoint");
    let mut progress = BasicIndexProgress::new(
        IndexGeneration::new(4),
        hash(99),
        AcceptedIndexTarget::new(1, positions[1].block_hash),
        Some(genesis),
        Some(genesis),
        IndexInputProtection::FromHeight(1),
    )
    .expect("recovered");
    let prepared = progress.prepare_turn().expect("prepared");
    progress
        .confirm_safe_durable(validated, IndexInputProtection::FromHeight(0))
        .expect("stronger retained protection");
    let before = progress;
    // Act
    let result = progress.complete_turn(prepared, &[position_record(&positions[1], Some(genesis))]);
    // Assert
    assert_eq!(result, Err(BasicIndexCatchUpError::StaleWork));
    assert_eq!(progress, before);
}
