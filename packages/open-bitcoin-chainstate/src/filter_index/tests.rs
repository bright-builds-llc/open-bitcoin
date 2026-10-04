// Parity breadcrumbs:
// - packages/bitcoin-knots/src/index/base.cpp
// - packages/bitcoin-knots/src/index/blockfilterindex.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp

use super::*;
use open_bitcoin_primitives::BlockHeader;

mod commitments;

fn positions() -> Vec<ChainPosition> {
    let header = BlockHeader {
        version: 1,
        previous_block_hash: BlockHash::default(),
        merkle_root: Default::default(),
        time: 1,
        bits: 0x207f_ffff,
        nonce: 0,
    };
    let genesis = ChainPosition::new(header.clone(), 0, 1, 1);
    let child = ChainPosition::new(
        BlockHeader {
            previous_block_hash: genesis.block_hash,
            time: 2,
            ..header
        },
        1,
        2,
        2,
    );
    vec![genesis, child]
}

fn identity(
    position: &ChainPosition,
    maybe_previous: Option<&FilterRecordIdentity>,
) -> FilterRecordIdentity {
    let previous_header =
        maybe_previous.map_or(FilterHeader::default(), |previous| previous.filter_header());
    let hash = FilterHash::from_byte_array([position.height as u8; 32]);
    FilterRecordIdentity::new(
        position.height,
        position.block_hash,
        position.previous_block_hash(),
        hash,
        compute_filter_header(hash, previous_header),
        previous_header,
        maybe_previous,
    )
    .expect("compatible identity")
}

#[test]
fn distinguishes_empty_and_committed_genesis_next_input() {
    // Arrange
    let genesis = identity(&positions()[0], None);
    // Act / Assert
    assert_eq!(
        IndexPrefix::Empty.input_protection(),
        IndexInputProtection::FromHeight(0)
    );
    assert_eq!(
        IndexPrefix::Committed(genesis).input_protection(),
        IndexInputProtection::FromHeight(1)
    );
}

#[test]
fn rejects_missing_or_incompatible_predecessor() {
    // Arrange
    let chain = positions();
    let genesis = identity(&chain[0], None);
    let child = identity(&chain[1], Some(&genesis));
    // Act / Assert
    assert_eq!(
        FilterRecordIdentity::new(
            1,
            child.block_hash(),
            child.parent_hash(),
            child.filter_hash(),
            child.filter_header(),
            child.previous_header(),
            None
        ),
        Err(FilterIndexError::Predecessor)
    );
    assert_eq!(
        FilterRecordIdentity::new(
            2,
            child.block_hash(),
            child.parent_hash(),
            child.filter_hash(),
            child.filter_header(),
            child.previous_header(),
            Some(&genesis)
        ),
        Err(FilterIndexError::Predecessor)
    );
}

#[test]
fn verifies_recovered_coins_and_compatible_metadata() {
    // Arrange
    let chain = positions();
    // Act
    let fence = VerifiedChainstateFence::new(Some(chain[1].block_hash), Some(&chain))
        .expect("fenced chain");
    // Assert
    assert_eq!(fence.tip(), &chain[1]);
    assert_eq!(fence.maybe_position(0), Some(&chain[0]));
    assert_eq!(fence.maybe_position(2), None);
}

#[test]
fn refuses_missing_authority() {
    // Arrange
    let chain = positions();
    // Act / Assert
    assert_eq!(
        VerifiedChainstateFence::new(None, Some(&chain)),
        Err(FilterIndexError::MissingCoins)
    );
    assert_eq!(
        VerifiedChainstateFence::new(Some(chain[1].block_hash), None),
        Err(FilterIndexError::MissingMetadata)
    );
    assert_eq!(
        VerifiedChainstateFence::new(Some(chain[1].block_hash), Some(&[])),
        Err(FilterIndexError::MissingMetadata)
    );
    assert_eq!(
        VerifiedChainstateFence::new(Some(chain[0].block_hash), Some(&chain)),
        Err(FilterIndexError::CoinsMetadataMismatch)
    );
}

#[test]
fn protects_early_and_upper_height_inputs_directly() {
    // Arrange
    for earliest in [0, 1, 2, u32::MAX] {
        let protection = IndexInputProtection::FromHeight(earliest);
        // Act / Assert
        assert_eq!(
            protection.check_prune_intent(earliest),
            Err(FilterIndexError::UnsafePruneIntent)
        );
        if earliest > 0 {
            assert_eq!(protection.check_prune_intent(earliest - 1), Ok(()));
        }
        let lock = protection.maybe_prune_lock().expect("required lock");
        assert!(lock.height_last <= u32::MAX - PRUNE_LOCK_BUFFER);
        assert_eq!(
            IndexInputProtection::from_saved_lock(&lock),
            Ok(IndexInputProtection::FromHeight(lock.height_first))
        );
    }
}

#[test]
fn absent_state_requires_every_index_artifact_to_be_absent() {
    // Arrange / Act / Assert
    assert_eq!(
        FilterRecoveryPlan::for_absent_state(false, false, None),
        FilterRecoveryPlan::LegacyAbsent
    );
    for (rows, projection, protection) in [
        (true, false, None),
        (false, true, None),
        (false, false, Some(IndexInputProtection::FromHeight(0))),
    ] {
        assert_eq!(
            FilterRecoveryPlan::for_absent_state(rows, projection, protection),
            FilterRecoveryPlan::Refuse(FilterIndexError::PartialState)
        );
    }
}

#[test]
fn keeps_proven_saved_prefix_and_stronger_protection() {
    // Arrange
    let chain = positions();
    let fence =
        VerifiedChainstateFence::new(Some(chain[1].block_hash), Some(&chain)).expect("fence");
    let genesis = identity(&chain[0], None);
    let child = identity(&chain[1], Some(&genesis));
    let saved = FilterCheckpoint::new(IndexPrefix::Committed(child));
    let protection = IndexInputProtection::FromHeight(0);
    let mut scan = FilterRecoveryScan::new(saved, Some(protection), &fence).expect("scan");
    // Act
    scan.push(genesis, genesis.block_hash())
        .expect("genesis projection");
    scan.push(child, child.block_hash())
        .expect("child projection");
    // Assert
    assert_eq!(
        scan.finish(),
        FilterRecoveryPlan::Keep {
            checkpoint: saved,
            protection
        }
    );
}

#[test]
fn reconciles_ahead_checkpoint_to_recovered_fence() {
    // Arrange
    let chain = positions();
    let fence =
        VerifiedChainstateFence::new(Some(chain[0].block_hash), Some(&chain[..1])).expect("fence");
    let genesis = identity(&chain[0], None);
    let child = identity(&chain[1], Some(&genesis));
    let saved = FilterCheckpoint::new(IndexPrefix::Committed(child));
    let mut scan =
        FilterRecoveryScan::new(saved, Some(saved.input_protection()), &fence).expect("scan");
    // Act
    scan.push(genesis, genesis.block_hash())
        .expect("genesis projection");
    scan.push(child, child.block_hash())
        .expect("ahead projection");
    // Assert
    assert_eq!(
        scan.finish(),
        FilterRecoveryPlan::Reconcile {
            checkpoint: FilterCheckpoint::new(IndexPrefix::Committed(genesis)),
            protection: IndexInputProtection::FromHeight(1)
        }
    );
}

#[test]
fn identity_refuses_invalid_genesis_facts() {
    // Arrange
    let genesis = identity(&positions()[0], None);
    // Act / Assert
    for (parent, previous_header, maybe_previous) in [
        (
            BlockHash::from_byte_array([1; 32]),
            FilterHeader::default(),
            None,
        ),
        (
            BlockHash::default(),
            FilterHeader::from_byte_array([1; 32]),
            None,
        ),
        (
            BlockHash::default(),
            FilterHeader::default(),
            Some(&genesis),
        ),
    ] {
        assert_eq!(
            FilterRecordIdentity::new(
                0,
                genesis.block_hash(),
                parent,
                genesis.filter_hash(),
                genesis.filter_header(),
                previous_header,
                maybe_previous
            ),
            Err(FilterIndexError::Genesis)
        );
    }
}

#[test]
fn identity_refuses_incompatible_parent_and_predecessor_header() {
    // Arrange
    let chain = positions();
    let genesis = identity(&chain[0], None);
    let child = identity(&chain[1], Some(&genesis));
    // Act / Assert
    for (parent, header) in [
        (BlockHash::default(), child.previous_header()),
        (child.parent_hash(), FilterHeader::default()),
    ] {
        assert_eq!(
            FilterRecordIdentity::new(
                1,
                child.block_hash(),
                parent,
                child.filter_hash(),
                child.filter_header(),
                header,
                Some(&genesis)
            ),
            Err(FilterIndexError::Predecessor)
        );
    }
}

#[test]
fn identity_refuses_invalid_composed_header() {
    // Arrange
    let genesis = identity(&positions()[0], None);
    // Act / Assert
    assert_eq!(
        FilterRecordIdentity::new(
            0,
            genesis.block_hash(),
            genesis.parent_hash(),
            genesis.filter_hash(),
            FilterHeader::default(),
            genesis.previous_header(),
            None
        ),
        Err(FilterIndexError::HeaderCommitment)
    );
}

#[test]
fn authority_rejects_incompatible_position_facts() {
    // Arrange
    let chain = positions();
    let mut cases = vec![chain.clone(); 5];
    cases[0][0].height = 1;
    cases[1][1].block_hash = BlockHash::default();
    cases[2][1] = ChainPosition::new(chain[0].header.clone(), 1, 2, 2);
    cases[3][0] = ChainPosition::new(chain[1].header.clone(), 0, 1, 1);
    cases[4][1].height = 2;
    // Act / Assert
    for invalid in cases {
        assert_eq!(
            VerifiedChainstateFence::new(Some(chain[1].block_hash), Some(&invalid)),
            Err(FilterIndexError::MetadataAncestry)
        );
    }
}

#[test]
fn terminal_prefix_has_no_wrapped_required_height() {
    // Arrange
    let mut endpoint = identity(&positions()[0], None);
    endpoint.height = u32::MAX;
    // Act
    let protection = IndexPrefix::Committed(endpoint).input_protection();
    // Assert
    assert_eq!(protection, IndexInputProtection::HeightSpaceExhausted);
    assert_eq!(protection.maybe_prune_lock(), None);
    assert_eq!(protection.check_prune_intent(u32::MAX), Ok(()));
    assert!(IndexInputProtection::FromHeight(0).covers(protection));
    assert!(!protection.covers(IndexInputProtection::FromHeight(u32::MAX)));
}

#[test]
fn malformed_saved_reserved_ranges_refuse() {
    // Arrange
    let lock = IndexInputProtection::FromHeight(1)
        .maybe_prune_lock()
        .expect("lock");
    let mut cases = vec![lock; 3];
    cases[0].name = "operator".to_owned();
    cases[1].height_first = u32::MAX;
    cases[2].height_last = u32::MAX;
    // Act / Assert
    for invalid in cases {
        assert_eq!(
            IndexInputProtection::from_saved_lock(&invalid),
            Err(FilterIndexError::MalformedProtection)
        );
    }
}

#[test]
fn refuses_missing_or_weak_saved_protection_before_rewind() {
    // Arrange
    let chain = positions();
    let fence =
        VerifiedChainstateFence::new(Some(chain[1].block_hash), Some(&chain)).expect("fence");
    let saved = FilterCheckpoint::new(IndexPrefix::Empty);
    // Act / Assert
    for maybe_protection in [
        None,
        Some(IndexInputProtection::FromHeight(1)),
        Some(IndexInputProtection::HeightSpaceExhausted),
    ] {
        assert_eq!(
            FilterRecoveryScan::new(saved, maybe_protection, &fence).err(),
            Some(FilterIndexError::WeakProtection)
        );
    }
}

#[test]
fn keeps_empty_checkpoint_without_claiming_genesis() {
    // Arrange
    let chain = positions();
    let fence =
        VerifiedChainstateFence::new(Some(chain[1].block_hash), Some(&chain)).expect("fence");
    let saved = FilterCheckpoint::new(IndexPrefix::Empty);
    // Act
    let plan = FilterRecoveryScan::new(saved, Some(IndexInputProtection::FromHeight(0)), &fence)
        .expect("empty scan")
        .finish();
    // Assert
    assert_eq!(
        plan,
        FilterRecoveryPlan::Keep {
            checkpoint: saved,
            protection: IndexInputProtection::FromHeight(0)
        }
    );
}

#[test]
fn reconciles_equal_height_wrong_branch_to_common_genesis() {
    // Arrange
    let chain = positions();
    let fence =
        VerifiedChainstateFence::new(Some(chain[1].block_hash), Some(&chain)).expect("fence");
    let genesis = identity(&chain[0], None);
    let fork = ChainPosition::new(
        BlockHeader {
            nonce: 4,
            ..chain[1].header.clone()
        },
        1,
        2,
        2,
    );
    let child = identity(&fork, Some(&genesis));
    let saved = FilterCheckpoint::new(IndexPrefix::Committed(child));
    let mut scan =
        FilterRecoveryScan::new(saved, Some(IndexInputProtection::FromHeight(0)), &fence)
            .expect("scan");
    // Act
    scan.push(genesis, genesis.block_hash()).expect("genesis");
    scan.push(child, child.block_hash()).expect("fork");
    // Assert
    assert_eq!(
        scan.finish(),
        FilterRecoveryPlan::Reconcile {
            checkpoint: FilterCheckpoint::new(IndexPrefix::Committed(genesis)),
            protection: IndexInputProtection::FromHeight(0)
        }
    );
}

#[test]
fn refuses_incomplete_checkpoint_projection() {
    // Arrange
    let chain = positions();
    let fence =
        VerifiedChainstateFence::new(Some(chain[1].block_hash), Some(&chain)).expect("fence");
    let genesis = identity(&chain[0], None);
    let child = identity(&chain[1], Some(&genesis));
    let saved = FilterCheckpoint::new(IndexPrefix::Committed(child));
    let mut scan =
        FilterRecoveryScan::new(saved, Some(saved.input_protection()), &fence).expect("scan");
    // Act
    scan.push(genesis, genesis.block_hash())
        .expect("genesis only");
    // Assert
    assert_eq!(
        scan.finish(),
        FilterRecoveryPlan::Refuse(FilterIndexError::Checkpoint)
    );
}

#[test]
fn refuses_unrelated_genesis() {
    // Arrange
    let chain = positions();
    let fence =
        VerifiedChainstateFence::new(Some(chain[1].block_hash), Some(&chain)).expect("fence");
    let fork = ChainPosition::new(
        BlockHeader {
            nonce: 1,
            ..chain[0].header.clone()
        },
        0,
        1,
        1,
    );
    let genesis = identity(&fork, None);
    let saved = FilterCheckpoint::new(IndexPrefix::Committed(genesis));
    let mut scan =
        FilterRecoveryScan::new(saved, Some(saved.input_protection()), &fence).expect("scan");
    // Act
    scan.push(genesis, genesis.block_hash())
        .expect("well-formed foreign genesis");
    // Assert
    assert_eq!(
        scan.finish(),
        FilterRecoveryPlan::Refuse(FilterIndexError::NoCommonGenesis)
    );
}

#[test]
fn projection_refusals_remain_fail_closed_after_error() {
    // Arrange
    let chain = positions();
    let fence =
        VerifiedChainstateFence::new(Some(chain[1].block_hash), Some(&chain)).expect("fence");
    let genesis = identity(&chain[0], None);
    let child = identity(&chain[1], Some(&genesis));
    let checkpoints = [
        IndexPrefix::Empty,
        IndexPrefix::Committed(genesis),
        IndexPrefix::Committed(child),
    ];
    // Act / Assert
    for prefix in checkpoints {
        let saved = FilterCheckpoint::new(prefix);
        let mut scan =
            FilterRecoveryScan::new(saved, Some(saved.input_protection()), &fence).expect("scan");
        assert_eq!(
            scan.push(child, child.block_hash()),
            Err(FilterIndexError::Projection)
        );
        assert_eq!(
            scan.push(genesis, genesis.block_hash()),
            Err(FilterIndexError::Projection)
        );
        assert_eq!(
            scan.finish(),
            FilterRecoveryPlan::Refuse(FilterIndexError::Projection)
        );
    }
}

#[test]
fn projection_refuses_wrong_hash_and_ancestry() {
    // Arrange
    let chain = positions();
    let fence =
        VerifiedChainstateFence::new(Some(chain[1].block_hash), Some(&chain)).expect("fence");
    let genesis = identity(&chain[0], None);
    let child = identity(&chain[1], Some(&genesis));
    let saved = FilterCheckpoint::new(IndexPrefix::Committed(child));
    // Act / Assert
    let mut scan =
        FilterRecoveryScan::new(saved, Some(saved.input_protection()), &fence).expect("scan");
    assert_eq!(
        scan.push(genesis, BlockHash::default()),
        Err(FilterIndexError::Projection)
    );
    let mut cases = [child; 3];
    cases[0].parent_hash = BlockHash::default();
    cases[1].previous_header = FilterHeader::default();
    cases[2].height = 0;
    for invalid in cases {
        let mut scan =
            FilterRecoveryScan::new(saved, Some(saved.input_protection()), &fence).expect("scan");
        scan.push(genesis, genesis.block_hash()).expect("genesis");
        assert_eq!(
            scan.push(invalid, invalid.block_hash()),
            Err(FilterIndexError::Projection)
        );
    }
}

#[test]
fn checkpoint_refuses_different_endpoint_commitments() {
    // Arrange
    let chain = positions();
    let fence =
        VerifiedChainstateFence::new(Some(chain[1].block_hash), Some(&chain)).expect("fence");
    let genesis = identity(&chain[0], None);
    let child = identity(&chain[1], Some(&genesis));
    let mut corrupt_endpoint = child;
    corrupt_endpoint.filter_hash = FilterHash::default();
    let saved = FilterCheckpoint::new(IndexPrefix::Committed(corrupt_endpoint));
    let mut scan =
        FilterRecoveryScan::new(saved, Some(saved.input_protection()), &fence).expect("scan");
    // Act
    scan.push(genesis, genesis.block_hash()).expect("genesis");
    scan.push(child, child.block_hash()).expect("child");
    // Assert
    assert_eq!(
        scan.finish(),
        FilterRecoveryPlan::Refuse(FilterIndexError::Checkpoint)
    );
}

#[test]
fn error_messages_describe_each_refusal() {
    // Arrange
    let errors = [
        FilterIndexError::Genesis,
        FilterIndexError::Predecessor,
        FilterIndexError::HeaderCommitment,
        FilterIndexError::MissingCoins,
        FilterIndexError::MissingMetadata,
        FilterIndexError::MetadataAncestry,
        FilterIndexError::CoinsMetadataMismatch,
        FilterIndexError::MalformedProtection,
        FilterIndexError::WeakProtection,
        FilterIndexError::PartialState,
        FilterIndexError::Projection,
        FilterIndexError::Checkpoint,
        FilterIndexError::NoCommonGenesis,
        FilterIndexError::UnsafePruneIntent,
    ];
    // Act / Assert
    for error in errors {
        assert!(!error.to_string().is_empty());
    }
}
