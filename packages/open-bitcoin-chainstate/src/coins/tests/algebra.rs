// Parity breadcrumbs:
// - packages/bitcoin-knots/src/coins.h
// - packages/bitcoin-knots/src/coins.cpp
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp
// - packages/bitcoin-knots/src/node/chainstate.cpp

use std::collections::HashMap;

use super::{
    CoinsBatch, CoinsCache, CoinsCacheEntry, CoinsCacheFlags, CoinsView, MemoryCoinsView,
    fixture_best_block, fixture_coin, fixture_outpoint, fixture_outpoint_two,
};
use crate::coins::CoinsOverlay;
use crate::error::ChainstateError;

#[test]
fn reorg_readd_of_dirty_spent_coin_is_not_fresh() {
    // Arrange
    let outpoint = fixture_outpoint();
    let coin = fixture_coin();
    let parent = MemoryCoinsView::from_coins(HashMap::new(), None);
    let mut cache = CoinsCache::from_parent(parent);
    cache.insert_entry_for_test(outpoint.clone(), CoinsCacheEntry::spent_dirty());

    // Act
    let add_result = cache.add_coin(outpoint.clone(), coin.clone(), true);
    let maybe_readd_flags = cache.cached_entry(&outpoint).map(CoinsCacheEntry::flags);
    let spend_result = cache.spend_coin(&outpoint);
    let maybe_spent_flags = cache.cached_entry(&outpoint).map(CoinsCacheEntry::flags);

    // Assert
    assert_eq!(add_result, Ok(()));
    assert_eq!(maybe_readd_flags, Some(CoinsCacheFlags::UnspentDirty));
    assert_ne!(maybe_readd_flags, Some(CoinsCacheFlags::UnspentFreshDirty));
    assert_eq!(spend_result, Ok(coin));
    assert!(cache.contains_in_cache(&outpoint));
    assert_eq!(maybe_spent_flags, Some(CoinsCacheFlags::SpentDirty));
}

#[test]
fn spend_coin_erases_fresh_entry() {
    // Arrange
    let outpoint = fixture_outpoint();
    let coin = fixture_coin();
    let parent = MemoryCoinsView::from_coins(HashMap::new(), None);
    let mut cache = CoinsCache::from_parent(parent);

    // Act
    let add_result = cache.add_coin(outpoint.clone(), coin.clone(), false);
    let maybe_add_flags = cache.cached_entry(&outpoint).map(CoinsCacheEntry::flags);
    let spend_result = cache.spend_coin(&outpoint);

    // Assert
    assert_eq!(add_result, Ok(()));
    assert_eq!(maybe_add_flags, Some(CoinsCacheFlags::UnspentFreshDirty));
    assert_eq!(spend_result, Ok(coin));
    assert!(!cache.contains_in_cache(&outpoint));
}

#[test]
fn spend_coin_writes_dirty_tombstone_for_clean_parent_hit() {
    // Arrange
    let outpoint = fixture_outpoint();
    let coin = fixture_coin();
    let mut parent_coins = HashMap::new();
    parent_coins.insert(outpoint.clone(), coin.clone());
    let parent = MemoryCoinsView::from_coins(parent_coins, None);
    let mut cache = CoinsCache::from_parent(parent);

    // Act
    let spend_result = cache.spend_coin(&outpoint);
    let maybe_flags = cache.cached_entry(&outpoint).map(CoinsCacheEntry::flags);

    // Assert
    assert_eq!(spend_result, Ok(coin));
    assert!(cache.contains_in_cache(&outpoint));
    assert!(!cache.have_coin_in_cache(&outpoint));
    assert_eq!(maybe_flags, Some(CoinsCacheFlags::SpentDirty));
}

#[test]
fn spend_coin_missing_unspent_returns_missing_coin() {
    // Arrange
    let outpoint = fixture_outpoint();
    let parent = MemoryCoinsView::from_coins(HashMap::new(), None);
    let mut cache = CoinsCache::from_parent(parent);

    // Act
    let spend_result = cache.spend_coin(&outpoint);

    // Assert
    assert_eq!(
        spend_result,
        Err(ChainstateError::MissingCoin {
            outpoint: outpoint.clone(),
        })
    );
}

#[test]
fn add_coin_possible_overwrite_false_rejects_live_unspent() {
    // Arrange
    let outpoint = fixture_outpoint();
    let coin = fixture_coin();
    let mut parent_coins = HashMap::new();
    parent_coins.insert(outpoint.clone(), coin.clone());
    let parent = MemoryCoinsView::from_coins(parent_coins, None);
    let mut cache = CoinsCache::from_parent(parent);

    // Act
    let add_result = cache.add_coin(outpoint.clone(), coin, false);

    // Assert
    assert_eq!(
        add_result,
        Err(ChainstateError::OutputOverwrite {
            outpoint: outpoint.clone(),
        })
    );
}

#[test]
fn batch_write_fresh_misapplied_to_unspent_parent() {
    // Arrange
    let outpoint = fixture_outpoint();
    let coin = fixture_coin();
    let parent = MemoryCoinsView::from_coins(HashMap::new(), None);
    let mut cache = CoinsCache::from_parent(parent);
    cache.insert_entry_for_test(
        outpoint.clone(),
        CoinsCacheEntry::unspent_clean(coin.clone()),
    );
    let mut writes = HashMap::new();
    writes.insert(outpoint.clone(), CoinsCacheEntry::unspent_fresh_dirty(coin));

    // Act
    let write_result = cache.batch_write(CoinsBatch { entries: writes }, None);

    // Assert
    assert_eq!(
        write_result,
        Err(ChainstateError::FreshFlagMisapplied {
            outpoint: outpoint.clone(),
        })
    );
}

#[test]
fn batch_write_fresh_spent_against_fresh_parent_deletes_parent() {
    // Arrange
    let outpoint = fixture_outpoint();
    let coin = fixture_coin();
    let parent = MemoryCoinsView::from_coins(HashMap::new(), None);
    let mut cache = CoinsCache::from_parent(parent);
    cache.insert_entry_for_test(outpoint.clone(), CoinsCacheEntry::unspent_fresh_dirty(coin));
    let mut writes = HashMap::new();
    writes.insert(outpoint.clone(), CoinsCacheEntry::spent_dirty());

    // Act
    let write_result = cache.batch_write(CoinsBatch { entries: writes }, None);

    // Assert
    assert_eq!(write_result, Ok(()));
    assert!(!cache.contains_in_cache(&outpoint));
}

#[test]
fn batch_write_skips_non_dirty_and_ignores_parent_miss_fresh_spent() {
    // Arrange
    let clean_outpoint = fixture_outpoint();
    let fresh_spent_outpoint = fixture_outpoint_two();
    let coin = fixture_coin();
    let parent = MemoryCoinsView::from_coins(HashMap::new(), None);
    let mut cache = CoinsCache::from_parent(parent);
    let mut writes = HashMap::new();
    writes.insert(clean_outpoint.clone(), CoinsCacheEntry::unspent_clean(coin));
    writes.insert(fresh_spent_outpoint.clone(), CoinsCacheEntry::spent_fresh());

    // Act
    let write_result = cache.batch_write(CoinsBatch { entries: writes }, None);

    // Assert
    assert_eq!(write_result, Ok(()));
    assert!(!cache.contains_in_cache(&clean_outpoint));
    assert!(!cache.contains_in_cache(&fresh_spent_outpoint));
}

#[test]
fn child_overlay_peek_does_not_populate_parent() {
    // Arrange
    let outpoint = fixture_outpoint();
    let memory = MemoryCoinsView::from_coins(HashMap::new(), None);
    let parent = CoinsCache::from_parent(memory);
    let overlay = CoinsOverlay::default();

    // Act
    let maybe_coin = overlay.peek_coin(&parent, &outpoint);

    // Assert
    assert_eq!(maybe_coin, Ok(None));
    assert!(!parent.contains_in_cache(&outpoint));
    assert!(!parent.have_coin_in_cache(&outpoint));
}

#[test]
fn flush_empties_child_and_writes_spent_tombstones() {
    // Arrange
    let outpoint = fixture_outpoint();
    let coin = fixture_coin();
    let mut parent_coins = HashMap::new();
    parent_coins.insert(outpoint.clone(), coin.clone());
    let memory = MemoryCoinsView::from_coins(parent_coins, None);
    let parent = CoinsCache::from_parent(memory);
    let mut child = CoinsCache::from_parent(parent);

    // Act
    let spend_result = child.spend_coin(&outpoint);
    let flush_result = child.flush();
    let maybe_parent_flags = child
        .parent()
        .cached_entry(&outpoint)
        .map(CoinsCacheEntry::flags);

    // Assert
    assert_eq!(spend_result, Ok(coin));
    assert_eq!(flush_result, Ok(()));
    assert!(!child.contains_in_cache(&outpoint));
    assert!(child.parent().contains_in_cache(&outpoint));
    assert!(!child.parent().have_coin_in_cache(&outpoint));
    assert_eq!(maybe_parent_flags, Some(CoinsCacheFlags::SpentDirty));
}

#[test]
fn sync_retains_unspent_clean_and_drops_spent() {
    // Arrange
    let unspent_outpoint = fixture_outpoint();
    let spent_outpoint = fixture_outpoint_two();
    let unspent_coin = fixture_coin();
    let spent_coin = fixture_coin();
    let mut parent_coins = HashMap::new();
    parent_coins.insert(spent_outpoint.clone(), spent_coin);
    let memory = MemoryCoinsView::from_coins(parent_coins, None);
    let mut cache = CoinsCache::from_parent(memory);

    // Act
    let add_result = cache.add_coin(unspent_outpoint.clone(), unspent_coin.clone(), false);
    let spend_result = cache.spend_coin(&spent_outpoint);
    let sync_result = cache.sync();
    let maybe_retained_flags = cache
        .cached_entry(&unspent_outpoint)
        .map(CoinsCacheEntry::flags);

    // Assert
    assert_eq!(add_result, Ok(()));
    assert_eq!(spend_result, Ok(fixture_coin()));
    assert_eq!(sync_result, Ok(()));
    assert!(cache.have_coin_in_cache(&unspent_outpoint));
    assert_eq!(maybe_retained_flags, Some(CoinsCacheFlags::UnspentClean));
    assert!(!cache.contains_in_cache(&spent_outpoint));
    assert_eq!(
        cache.parent().get_coin(&unspent_outpoint),
        Ok(Some(unspent_coin))
    );
    assert_eq!(cache.parent().get_coin(&spent_outpoint), Ok(None));
}

#[test]
fn overlay_add_coin_writes_dirty_batch_without_warming_parent() {
    // Arrange
    let outpoint = fixture_outpoint();
    let coin = fixture_coin();
    let memory = MemoryCoinsView::from_coins(HashMap::new(), None);
    let parent = CoinsCache::from_parent(memory);
    let mut overlay = CoinsOverlay::new();
    overlay.set_best_block(fixture_best_block());

    // Act
    let add_result = overlay.add_coin(&parent, outpoint.clone(), coin.clone(), false);
    let maybe_peeked = overlay.peek_coin(&parent, &outpoint);
    let dirty_batch = overlay.into_dirty_batch();

    // Assert
    assert_eq!(add_result, Ok(()));
    assert_eq!(maybe_peeked, Ok(Some(coin)));
    assert_eq!(dirty_batch.entries.len(), 1);
    assert!(dirty_batch.entries.values().all(CoinsCacheEntry::is_dirty));
    assert!(!parent.contains_in_cache(&outpoint));
}

#[test]
fn overlay_spend_coin_erases_fresh_without_warming_parent() {
    // Arrange
    let outpoint = fixture_outpoint();
    let coin = fixture_coin();
    let memory = MemoryCoinsView::from_coins(HashMap::new(), None);
    let parent = CoinsCache::from_parent(memory);
    let mut overlay = CoinsOverlay::new();
    overlay
        .add_coin(&parent, outpoint.clone(), coin.clone(), false)
        .expect("fresh add on empty overlay");

    // Act
    let spend_result = overlay.spend_coin(&parent, &outpoint);
    let maybe_peeked = overlay.peek_coin(&parent, &outpoint);

    // Assert
    assert_eq!(spend_result, Ok(coin));
    assert_eq!(maybe_peeked, Ok(None));
    assert!(!parent.contains_in_cache(&outpoint));
}

#[test]
fn batch_write_parent_miss_copies_fresh_and_overwrites_without_fresh() {
    // Arrange
    let fresh_outpoint = fixture_outpoint();
    let overwrite_outpoint = fixture_outpoint_two();
    let coin = fixture_coin();
    let parent = MemoryCoinsView::from_coins(HashMap::new(), None);
    let mut cache = CoinsCache::from_parent(parent);
    cache.insert_entry_for_test(
        overwrite_outpoint.clone(),
        CoinsCacheEntry::unspent_clean(coin.clone()),
    );
    let mut writes = HashMap::new();
    writes.insert(
        fresh_outpoint.clone(),
        CoinsCacheEntry::unspent_fresh_dirty(coin.clone()),
    );
    writes.insert(overwrite_outpoint.clone(), CoinsCacheEntry::spent_dirty());

    // Act
    let write_result = cache.batch_write(CoinsBatch { entries: writes }, None);

    // Assert
    assert_eq!(write_result, Ok(()));
    assert_eq!(
        cache
            .cached_entry(&fresh_outpoint)
            .map(CoinsCacheEntry::flags),
        Some(CoinsCacheFlags::UnspentFreshDirty)
    );
    assert_eq!(
        cache
            .cached_entry(&overwrite_outpoint)
            .map(CoinsCacheEntry::flags),
        Some(CoinsCacheFlags::SpentDirty)
    );
}

#[test]
fn add_coin_rejects_or_replaces_existing_overlay_unspent() {
    // Arrange
    let reject_outpoint = fixture_outpoint();
    let replace_outpoint = fixture_outpoint_two();
    let coin = fixture_coin();
    let parent = MemoryCoinsView::from_coins(HashMap::new(), None);
    let mut cache = CoinsCache::from_parent(parent);
    cache.insert_entry_for_test(
        reject_outpoint.clone(),
        CoinsCacheEntry::unspent_clean(coin.clone()),
    );
    cache.insert_entry_for_test(
        replace_outpoint.clone(),
        CoinsCacheEntry::unspent_clean(coin.clone()),
    );

    // Act
    let reject_result = cache.add_coin(reject_outpoint.clone(), coin.clone(), false);
    let replace_result = cache.add_coin(replace_outpoint.clone(), coin, true);

    // Assert
    assert_eq!(
        reject_result,
        Err(ChainstateError::OutputOverwrite {
            outpoint: reject_outpoint,
        })
    );
    assert_eq!(replace_result, Ok(()));
    assert_eq!(
        cache
            .cached_entry(&replace_outpoint)
            .map(CoinsCacheEntry::flags),
        Some(CoinsCacheFlags::UnspentDirty)
    );
}

#[test]
fn spend_coin_spent_overlay_occupancy_returns_missing_coin() {
    // Arrange
    let outpoint = fixture_outpoint();
    let parent = MemoryCoinsView::from_coins(HashMap::new(), None);
    let mut cache = CoinsCache::from_parent(parent);
    cache.insert_entry_for_test(outpoint.clone(), CoinsCacheEntry::spent_dirty());

    // Act
    let spend_result = cache.spend_coin(&outpoint);

    // Assert
    assert_eq!(spend_result, Err(ChainstateError::MissingCoin { outpoint }));
}

#[test]
fn batch_write_overwrites_parent_unspent_with_dirty_unspent() {
    // Arrange
    let outpoint = fixture_outpoint();
    let coin = fixture_coin();
    let parent = MemoryCoinsView::from_coins(HashMap::new(), None);
    let mut cache = CoinsCache::from_parent(parent);
    cache.insert_entry_for_test(
        outpoint.clone(),
        CoinsCacheEntry::unspent_clean(coin.clone()),
    );
    let mut writes = HashMap::new();
    writes.insert(outpoint.clone(), CoinsCacheEntry::unspent_dirty(coin));

    // Act
    let write_result = cache.batch_write(CoinsBatch { entries: writes }, None);

    // Assert
    assert_eq!(write_result, Ok(()));
    assert_eq!(
        cache.cached_entry(&outpoint).map(CoinsCacheEntry::flags),
        Some(CoinsCacheFlags::UnspentDirty)
    );
}
