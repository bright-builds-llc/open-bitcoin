// Parity breadcrumbs:
// - packages/bitcoin-knots/src/node/chainstate.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp
// - packages/bitcoin-knots/src/validation.cpp

use open_bitcoin_core::{
    chainstate::{
        CoinsCache, CoinsView, FlushDecision, FlushMode, FlushPolicyTime, RecoveryDecision,
    },
    consensus::block_hash,
    primitives::{Block, BlockHash, OutPoint, Txid},
};

use super::{
    RecordingCoinsView, SucceedingSink, coinbase, dirty_unspent_batch, expect_error, header,
    header_entry, initialize_ready, open_temp_store, plant_interrupted_heads, policy_now,
    remove_dir_if_exists, sample_coin,
};
use crate::chainstate::flush_lifecycle::{
    FlushLifecycle, coins_recovery_outcome_after_success, default_coins_cache_byte_limit,
    initialize,
};
use crate::status::{
    CacheSizeLabel, CoinsRecoveryOutcome, FieldAvailability, HaveBytesEvidence,
    LastFlushReasonLabel, WriteKindLabel,
};
use crate::storage::{
    PersistMode, coins_codec::encode_head_blocks_key, coins_view::FjallCoinsView,
};
use std::cell::Cell;

fn available_evidence(
    availability: FieldAvailability<crate::status::ChainstateDurabilityEvidence>,
) -> crate::status::ChainstateDurabilityEvidence {
    match availability {
        FieldAvailability::Available(evidence) => evidence,
        FieldAvailability::Unavailable { reason } => {
            panic!("expected available durability evidence, got {reason}")
        }
    }
}

#[test]
fn execute_flush_retains_last_write_decision_not_later_none_classification() {
    // Arrange
    let mut lifecycle = FlushLifecycle::ready_for_test(
        default_coins_cache_byte_limit(),
        0,
        FlushPolicyTime::from_unix_seconds(1),
        false,
    );
    let mut cache = CoinsCache::from_parent(RecordingCoinsView {
        writes: Cell::new(0),
    });
    let execution = lifecycle
        .execute_flush(
            &mut SucceedingSink,
            &mut cache,
            FlushMode::Periodic,
            FlushPolicyTime::from_unix_seconds(2),
            u64::MAX,
            &[],
            &[],
            &[],
            &[],
        )
        .expect("periodic sync");
    assert!(matches!(execution.decision, FlushDecision::Sync(_)));

    // Act
    let evidence = available_evidence(lifecycle.project_chainstate_durability(
        cache.estimated_cache_bytes(),
        cache.cache_entry_count(),
        FlushPolicyTime::from_unix_seconds(3),
        u64::MAX,
        None,
        None,
        HaveBytesEvidence::empty(),
    ));

    // Assert
    assert_eq!(evidence.cache_size, CacheSizeLabel::Ok);
    assert_eq!(evidence.last_flush_reason, LastFlushReasonLabel::Periodic);
    assert_eq!(evidence.write_kind, WriteKindLabel::Sync);
}

#[test]
fn project_chainstate_durability_uses_none_mode_for_current_cache_size() {
    // Arrange
    let mut lifecycle =
        FlushLifecycle::ready_for_test(100, 0, FlushPolicyTime::from_unix_seconds(1), false);
    let mut cache = CoinsCache::from_parent(RecordingCoinsView {
        writes: Cell::new(0),
    });
    lifecycle
        .execute_flush(
            &mut SucceedingSink,
            &mut cache,
            FlushMode::Periodic,
            FlushPolicyTime::from_unix_seconds(2),
            u64::MAX,
            &[],
            &[],
            &[],
            &[],
        )
        .expect("retain periodic write");
    cache
        .add_coin(
            OutPoint {
                txid: Txid::from_byte_array([0x21; 32]),
                vout: 0,
            },
            sample_coin(),
            true,
        )
        .expect("first overlay coin");
    cache
        .add_coin(
            OutPoint {
                txid: Txid::from_byte_array([0x22; 32]),
                vout: 0,
            },
            sample_coin(),
            true,
        )
        .expect("second overlay coin");

    // Act
    let evidence = available_evidence(lifecycle.project_chainstate_durability(
        cache.estimated_cache_bytes(),
        cache.cache_entry_count(),
        FlushPolicyTime::from_unix_seconds(3),
        u64::MAX,
        None,
        None,
        HaveBytesEvidence::empty(),
    ));

    // Assert
    assert!(
        matches!(
            evidence.cache_size,
            CacheSizeLabel::Large | CacheSizeLabel::Critical
        ),
        "current None-mode occupancy must be large or critical, got {:?}",
        evidence.cache_size
    );
    assert_eq!(evidence.last_flush_reason, LastFlushReasonLabel::Periodic);
    assert_eq!(evidence.write_kind, WriteKindLabel::Sync);
}

#[test]
fn initialize_consistent_with_best_block_is_consistent() {
    // Arrange
    let (path, store) = open_temp_store("consistent-best-block");
    let tip = BlockHash::from_byte_array([0x11; 32]);
    let outpoint = OutPoint {
        txid: Txid::from_byte_array([0x12; 32]),
        vout: 0,
    };
    let mut planted = FjallCoinsView::from_store(&store);
    planted
        .batch_write(dirty_unspent_batch(&[(outpoint, sample_coin())]), Some(tip))
        .expect("plant consistent tip");

    // Act
    let (lifecycle, view, cache) = initialize_ready(&store);
    let evidence = available_evidence(lifecycle.project_chainstate_durability(
        cache.estimated_cache_bytes(),
        cache.cache_entry_count(),
        policy_now(),
        u64::MAX,
        view.best_block().expect("best block"),
        Some(7),
        HaveBytesEvidence::empty(),
    ));

    // Assert
    assert_eq!(
        lifecycle.maybe_recovery_outcome(),
        Some(CoinsRecoveryOutcome::Consistent)
    );
    assert_eq!(evidence.recovery_outcome, CoinsRecoveryOutcome::Consistent);
    assert_eq!(evidence.maybe_coins_best_block_height, Some(7));
    assert_eq!(evidence.maybe_coins_best_block_hash, Some("11".repeat(32)));
    remove_dir_if_exists(&path);
}

#[test]
fn initialize_missing_b_is_fail_closed_without_tip() {
    // Arrange
    let (path, store) = open_temp_store("missing-b-fail-closed");

    // Act
    let (lifecycle, view, cache) = initialize_ready(&store);
    let evidence = available_evidence(lifecycle.project_chainstate_durability(
        cache.estimated_cache_bytes(),
        cache.cache_entry_count(),
        policy_now(),
        u64::MAX,
        view.best_block().expect("best block"),
        Some(1),
        HaveBytesEvidence::empty(),
    ));
    let one_head = coins_recovery_outcome_after_success(RecoveryDecision::OneHead, None)
        .expect("one-head without B");

    // Assert
    assert_eq!(
        lifecycle.maybe_recovery_outcome(),
        Some(CoinsRecoveryOutcome::FailClosed)
    );
    assert_eq!(one_head, CoinsRecoveryOutcome::FailClosed);
    assert_eq!(evidence.recovery_outcome, CoinsRecoveryOutcome::FailClosed);
    assert_eq!(evidence.maybe_coins_best_block_height, None);
    assert_eq!(evidence.maybe_coins_best_block_hash, None);
    remove_dir_if_exists(&path);
}

#[test]
fn interrupted_replay_success_is_replayed() {
    // Arrange
    let (path, store) = open_temp_store("interrupted-replayed");
    let view = FjallCoinsView::from_store(&store);
    let new_header = header(BlockHash::from_byte_array([0_u8; 32]), 8);
    let new_hash = block_hash(&new_header);
    let new_block = Block {
        header: new_header.clone(),
        transactions: vec![coinbase(0, 50)],
    };
    store
        .save_header_entries(&[header_entry(new_header, 0, 1)], PersistMode::Sync)
        .expect("save first-flush header");
    store
        .save_block(&new_block, PersistMode::Sync)
        .expect("save first-flush body");
    plant_interrupted_heads(&view, new_hash, BlockHash::from_byte_array([0_u8; 32]));

    // Act
    let (lifecycle, view, cache) = initialize_ready(&store);
    let evidence = available_evidence(lifecycle.project_chainstate_durability(
        cache.estimated_cache_bytes(),
        cache.cache_entry_count(),
        policy_now(),
        u64::MAX,
        view.best_block().expect("replayed best"),
        Some(0),
        HaveBytesEvidence::empty(),
    ));

    // Assert
    assert_eq!(
        lifecycle.maybe_recovery_outcome(),
        Some(CoinsRecoveryOutcome::Replayed)
    );
    assert_eq!(evidence.recovery_outcome, CoinsRecoveryOutcome::Replayed);
    let expected_hash = new_hash
        .as_bytes()
        .iter()
        .fold(String::new(), |mut hex, byte| {
            hex.push_str(&format!("{byte:02x}"));
            hex
        });
    assert_eq!(
        evidence.maybe_coins_best_block_hash.as_deref(),
        Some(expected_hash.as_str())
    );
    remove_dir_if_exists(&path);
}

#[test]
fn initialize_error_detail_includes_fail_closed() {
    // Arrange
    let (count_path, count_store) = open_temp_store("count-three-fail-closed");
    let count_view = FjallCoinsView::from_store(&count_store);
    let mut planted = vec![0x03];
    planted.extend_from_slice(&[0x11; 96]);
    count_view
        .write_raw_bytes(&encode_head_blocks_key(), planted)
        .expect("plant count-3 H");
    let (interrupted_path, interrupted_store) = open_temp_store("interrupted-fail-closed");
    let interrupted_view = FjallCoinsView::from_store(&interrupted_store);
    plant_interrupted_heads(
        &interrupted_view,
        BlockHash::from_byte_array([0xaa; 32]),
        BlockHash::from_byte_array([0xbb; 32]),
    );

    // Act
    let count_error = expect_error(
        initialize(&count_store, policy_now(), policy_now(), 0, false, u64::MAX),
        "count-3 heads fail-closed",
    );
    let interrupted_error = expect_error(
        initialize(
            &interrupted_store,
            policy_now(),
            policy_now(),
            0,
            false,
            u64::MAX,
        ),
        "missing undo/body fail-closed",
    );
    let inconsistent = coins_recovery_outcome_after_success(
        RecoveryDecision::InconsistentOtherCount { count: 4 },
        None,
    );

    // Assert
    assert!(
        count_error.to_string().contains("fail_closed"),
        "count-3 must include fail_closed, got {count_error}"
    );
    assert!(
        interrupted_error.to_string().contains("fail_closed")
            && interrupted_error.to_string().contains("interrupted"),
        "replay failure must include fail_closed and interrupted, got {interrupted_error}"
    );
    assert!(
        inconsistent
            .expect_err("inconsistent count is fail-closed")
            .to_string()
            .contains("fail_closed")
    );
    remove_dir_if_exists(&count_path);
    remove_dir_if_exists(&interrupted_path);
}

#[test]
fn occupancy_uses_estimated_cache_bytes_not_fjall_len() {
    // Arrange
    let projector = include_str!("../../../status/chainstate_durability.rs");
    let lifecycle = include_str!("../../flush_lifecycle.rs");

    // Act / Assert
    assert!(
        projector.contains("estimated_cache_bytes"),
        "projector must classify estimated_cache_bytes"
    );
    assert!(
        lifecycle.contains("estimated_cache_bytes"),
        "flush lifecycle must measure estimated_cache_bytes"
    );
    assert!(
        !projector.contains("fjall") && !lifecycle.contains(".len() as cache"),
        "occupancy must not use Fjall item counts"
    );
}
