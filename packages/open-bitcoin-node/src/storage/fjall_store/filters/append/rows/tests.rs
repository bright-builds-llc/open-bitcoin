// Parity breadcrumbs:
// - packages/bitcoin-knots/src/index/blockfilterindex.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp

use super::*;
use crate::chainstate::{BasicFilterStartupMode, validation_history::tests::phase167 as genuine};
use open_bitcoin_core::consensus::CanonicalChain;

#[test]
fn phase167_submission_basic_rows_changed_checkpoint_moves_only_basic_lock() {
    genuine::with_mode(BasicFilterStartupMode::Enabled, |manager, store, _| {
        // Arrange
        let network = crate::ManagedNetworkHandle::new(genuine::network(manager));
        network
            .initialize_basic_filter_index_owner()
            .expect("real BASIC owner");
        network
            .drive_basic_filter_index_turn()
            .expect("real genesis generation");
        let hash = CanonicalChain::Regtest.spec().genesis_hash();
        let record = store
            .load_basic_filter_record(hash)
            .expect("actual record")
            .expect("genesis BASIC");
        let checkpoint = FilterCheckpoint::new(IndexPrefix::Committed(record.identity()));
        let mut operator = IndexInputProtection::FromHeight(7)
            .maybe_prune_lock()
            .expect("lock");
        operator.name = "operator".to_owned();
        // Act
        let rows = checkpoint_rows(
            FilterCheckpoint::new(IndexPrefix::Empty),
            checkpoint,
            IndexInputProtection::FromHeight(0),
            0,
            hash,
            vec![operator.clone()],
        )
        .expect("data construction");
        // Assert
        assert_eq!(rows.protection, IndexInputProtection::FromHeight(1));
        assert!(rows.locks.contains(&operator));
        assert_eq!(rows.locks.len(), 2);
        assert_eq!(rows.state.maybe_endpoint, Some((0, hash)));
        let (state, locks) = rows.encode().expect("existing codecs");
        assert_eq!(codec::decode_state(&state).expect("decode"), rows.state);
        assert_eq!(
            crate::storage::fjall_store::decode_prune_locks(&locks)
                .expect("locks")
                .len(),
            2
        );
        assert!(rows.lock_size >= locks.len() as u64);
        assert_eq!(rows.lock_count, 2);
        drop(network);
    });
}

#[test]
fn phase167_submission_basic_rows_identical_checkpoint_preserves_saved_protection() {
    // Arrange
    let empty = FilterCheckpoint::new(IndexPrefix::Empty);
    let saved = IndexInputProtection::FromHeight(7);
    // Act
    let rows =
        checkpoint_rows(empty, empty, saved, 0, BlockHash::default(), vec![]).expect("retry data");
    // Assert
    assert_eq!(rows.protection, saved);
    assert_eq!(rows.state.maybe_endpoint, None);
    assert_eq!(rows.locks.len(), 1);
}
