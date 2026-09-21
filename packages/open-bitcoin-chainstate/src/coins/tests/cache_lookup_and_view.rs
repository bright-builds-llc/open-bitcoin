// Parity breadcrumbs:
// - packages/bitcoin-knots/src/coins.h
// - packages/bitcoin-knots/src/coins.cpp
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp
// - packages/bitcoin-knots/src/node/chainstate.cpp

use super::*;

#[test]
fn memory_coins_view_round_trips_unspent_coins_and_best_block() {
    // Arrange
    let outpoint = fixture_outpoint();
    let coin = fixture_coin();
    let maybe_best_block = Some(fixture_best_block());
    let mut coins = HashMap::new();
    coins.insert(outpoint.clone(), coin.clone());
    let mut view = MemoryCoinsView::from_coins(coins, maybe_best_block);
    let missing = OutPoint {
        txid: Txid::from_byte_array([9_u8; 32]),
        vout: 1,
    };

    // Act
    let maybe_found = view.get_coin(&outpoint);
    let has_coin = view.have_coin(&outpoint);
    let best_block = view.best_block();
    let maybe_miss = view.get_coin(&missing);
    let mut spent_writes = HashMap::new();
    spent_writes.insert(outpoint.clone(), CoinsCacheEntry::spent_dirty());
    let write_result = view.batch_write(
        CoinsBatch {
            entries: spent_writes,
        },
        None,
    );
    let maybe_after_spend = view.get_coin(&outpoint);

    // Assert
    assert_eq!(maybe_found, Ok(Some(coin)));
    assert_eq!(has_coin, Ok(true));
    assert_eq!(best_block, Ok(maybe_best_block));
    assert_eq!(maybe_miss, Ok(None));
    assert!(!matches!(
        maybe_miss,
        Err(ChainstateError::MissingCoin { .. })
    ));
    assert_eq!(write_result, Ok(()));
    assert_eq!(maybe_after_spend, Ok(None));
    assert!(!view.contains_outpoint(&outpoint));
}

#[test]
fn coins_cache_entry_constructs_exactly_the_five_valid_knots_states() {
    // Arrange
    let coin = fixture_coin();

    // Act
    let unspent_clean = CoinsCacheEntry::unspent_clean(coin.clone());
    let unspent_dirty = CoinsCacheEntry::unspent_dirty(coin.clone());
    let unspent_fresh_dirty = CoinsCacheEntry::unspent_fresh_dirty(coin.clone());
    let spent_dirty = CoinsCacheEntry::spent_dirty();
    let spent_fresh = CoinsCacheEntry::spent_fresh();

    // Assert
    assert_eq!(unspent_clean.flags(), CoinsCacheFlags::UnspentClean);
    assert_eq!(unspent_clean.maybe_coin(), Some(&coin));
    assert_eq!(unspent_dirty.flags(), CoinsCacheFlags::UnspentDirty);
    assert_eq!(unspent_dirty.maybe_coin(), Some(&coin));
    assert_eq!(
        unspent_fresh_dirty.flags(),
        CoinsCacheFlags::UnspentFreshDirty
    );
    assert_eq!(unspent_fresh_dirty.maybe_coin(), Some(&coin));
    assert_eq!(spent_dirty.flags(), CoinsCacheFlags::SpentDirty);
    assert_eq!(spent_dirty.maybe_coin(), None);
    assert_eq!(spent_fresh.flags(), CoinsCacheFlags::SpentFresh);
    assert_eq!(spent_fresh.maybe_coin(), None);
}

#[test]
fn coins_cache_entry_rejects_fresh_only_unspent_spent_clean_and_spent_fresh_dirty() {
    // Arrange
    let coin = fixture_coin();

    // Act
    let fresh_only_unspent = CoinsCacheEntry::try_from_flags(Some(coin), false, true);
    let spent_clean = CoinsCacheEntry::try_from_flags(None, false, false);
    let spent_fresh_dirty = CoinsCacheEntry::try_from_flags(None, true, true);

    // Assert
    assert_eq!(
        fresh_only_unspent,
        Err(ChainstateError::InvalidCacheEntry {
            dirty: false,
            fresh: true,
            spent: false,
        })
    );
    assert_eq!(
        spent_clean,
        Err(ChainstateError::InvalidCacheEntry {
            dirty: false,
            fresh: false,
            spent: true,
        })
    );
    assert_eq!(
        spent_fresh_dirty,
        Err(ChainstateError::InvalidCacheEntry {
            dirty: true,
            fresh: true,
            spent: true,
        })
    );
}

#[test]
fn four_lookup_facts_are_distinct_for_spent_dirty_tombstone() {
    // Arrange
    let outpoint = fixture_outpoint();
    let coin = fixture_coin();
    let mut parent_coins = HashMap::new();
    parent_coins.insert(outpoint.clone(), coin.clone());
    let parent = MemoryCoinsView::from_coins(parent_coins, None);
    let mut cache = CoinsCache::from_parent(parent);
    cache.insert_entry_for_test(outpoint.clone(), CoinsCacheEntry::spent_dirty());

    // Act
    let occupancy = cache.contains_in_cache(&outpoint);
    let in_cache_unspent = cache.have_coin_in_cache(&outpoint);
    let have_coin = cache.have_coin(&outpoint);
    let maybe_coin = cache.get_coin(&outpoint);
    let parent_has_outpoint = cache.parent().contains_outpoint(&outpoint);
    let parent_truth = cache.parent().get_coin(&outpoint);

    // Assert
    assert!(occupancy);
    assert!(!in_cache_unspent);
    assert_eq!(have_coin, Ok(false));
    assert_eq!(maybe_coin, Ok(None));
    assert!(parent_has_outpoint);
    assert_eq!(parent_truth, Ok(Some(coin)));
}

#[test]
fn spent_dirty_tombstone_is_occupancy_hit_with_have_coin_false() {
    // Arrange
    let outpoint = fixture_outpoint();
    let parent = MemoryCoinsView::from_coins(HashMap::new(), None);
    let mut cache = CoinsCache::from_parent(parent);
    cache.insert_entry_for_test(outpoint.clone(), CoinsCacheEntry::spent_dirty());

    // Act
    let occupancy = cache.contains_in_cache(&outpoint);
    let in_cache_unspent = cache.have_coin_in_cache(&outpoint);
    let have_coin = cache.have_coin(&outpoint);
    let maybe_coin = cache.get_coin(&outpoint);
    let parent_gained_outpoint = cache.parent().contains_outpoint(&outpoint);

    // Assert
    assert!(occupancy);
    assert!(!in_cache_unspent);
    assert_eq!(have_coin, Ok(false));
    assert_eq!(maybe_coin, Ok(None));
    assert!(!parent_gained_outpoint);
}

#[test]
fn fetch_coin_populates_self_only_from_parent_peek() {
    // Arrange
    let outpoint = fixture_outpoint();
    let coin = fixture_coin();
    let mut parent_coins = HashMap::new();
    parent_coins.insert(outpoint.clone(), coin.clone());
    let parent = MemoryCoinsView::from_coins(parent_coins, None);
    let mut cache = CoinsCache::from_parent(parent);

    // Act
    let maybe_fetched = cache.fetch_coin(&outpoint);
    let (maybe_fetched_coin, fetched_flags) = match maybe_fetched {
        Ok(Some(entry)) => (entry.maybe_coin().cloned(), entry.flags()),
        Ok(None) => (None, CoinsCacheFlags::UnspentClean),
        Err(error) => panic!("fetch should succeed: {error}"),
    };
    let occupancy = cache.contains_in_cache(&outpoint);
    let in_cache_unspent = cache.have_coin_in_cache(&outpoint);
    let parent_still_has_outpoint = cache.parent().contains_outpoint(&outpoint);

    // Assert
    assert_eq!(maybe_fetched_coin.as_ref(), Some(&coin));
    assert_eq!(fetched_flags, CoinsCacheFlags::UnspentClean);
    assert!(occupancy);
    assert!(in_cache_unspent);
    assert!(parent_still_has_outpoint);
}

#[test]
fn coins_view_get_coin_does_not_insert_on_parent_miss() {
    // Arrange
    let outpoint = fixture_outpoint();
    let parent = MemoryCoinsView::from_coins(HashMap::new(), None);
    let cache = CoinsCache::from_parent(parent);

    // Act
    let maybe_coin = cache.get_coin(&outpoint);
    let occupancy = cache.contains_in_cache(&outpoint);
    let best_block = cache.best_block();

    // Assert
    assert_eq!(maybe_coin, Ok(None));
    assert!(!occupancy);
    assert_eq!(best_block, Ok(None));
}

#[test]
fn coin_type_has_no_spent_flag() {
    // Arrange / Act
    let coin = Coin {
        output: TransactionOutput {
            value: Amount::from_sats(50).expect("valid amount"),
            script_pubkey: ScriptBuf::from_bytes(vec![0x51]).expect("valid script"),
        },
        is_coinbase: false,
        created_height: 1,
        created_median_time_past: 0,
    };

    // Assert
    assert_eq!(
        coin.output.value,
        Amount::from_sats(50).expect("valid amount")
    );
    assert!(!coin.is_coinbase);
    assert_eq!(coin.created_height, 1);
    assert_eq!(coin.created_median_time_past, 0);
}

#[test]
fn coins_cache_entry_try_from_flags_accepts_the_five_valid_knots_states() {
    // Arrange
    let coin = fixture_coin();

    // Act
    let unspent_clean = CoinsCacheEntry::try_from_flags(Some(coin.clone()), false, false);
    let unspent_dirty = CoinsCacheEntry::try_from_flags(Some(coin.clone()), true, false);
    let unspent_fresh_dirty = CoinsCacheEntry::try_from_flags(Some(coin.clone()), true, true);
    let spent_dirty = CoinsCacheEntry::try_from_flags(None, true, false);
    let spent_fresh = CoinsCacheEntry::try_from_flags(None, false, true);

    // Assert
    assert_eq!(
        unspent_clean,
        Ok(CoinsCacheEntry::unspent_clean(coin.clone()))
    );
    assert_eq!(
        unspent_dirty,
        Ok(CoinsCacheEntry::unspent_dirty(coin.clone()))
    );
    assert_eq!(
        unspent_fresh_dirty,
        Ok(CoinsCacheEntry::unspent_fresh_dirty(coin.clone()))
    );
    assert_eq!(spent_dirty, Ok(CoinsCacheEntry::spent_dirty()));
    assert_eq!(spent_fresh, Ok(CoinsCacheEntry::spent_fresh()));
}

#[test]
fn fetch_coin_returns_existing_overlay_entry_and_none_on_parent_miss() {
    // Arrange
    let outpoint = fixture_outpoint();
    let missing = OutPoint {
        txid: Txid::from_byte_array([8_u8; 32]),
        vout: 2,
    };
    let coin = fixture_coin();
    let mut parent_coins = HashMap::new();
    parent_coins.insert(outpoint.clone(), coin.clone());
    let parent = MemoryCoinsView::from_coins(parent_coins, None);
    let mut cache = CoinsCache::from_parent(parent);

    // Act
    let first_flags = cache
        .fetch_coin(&outpoint)
        .expect("first fetch should succeed")
        .expect("parent held an unspent coin")
        .flags();
    let second_flags = cache
        .fetch_coin(&outpoint)
        .expect("overlay fetch should succeed")
        .expect("overlay still holds the coin")
        .flags();
    let missing_was_absent = matches!(cache.fetch_coin(&missing), Ok(None));
    let missing_occupancy = cache.contains_in_cache(&missing);

    // Assert
    assert_eq!(first_flags, CoinsCacheFlags::UnspentClean);
    assert_eq!(second_flags, CoinsCacheFlags::UnspentClean);
    assert!(missing_was_absent);
    assert!(!missing_occupancy);
}

#[test]
fn coins_cache_have_coin_and_best_block_consult_parent_on_overlay_miss() {
    // Arrange
    let outpoint = fixture_outpoint();
    let coin = fixture_coin();
    let maybe_best_block = Some(fixture_best_block());
    let mut parent_coins = HashMap::new();
    parent_coins.insert(outpoint.clone(), coin);
    let parent = MemoryCoinsView::from_coins(parent_coins, maybe_best_block);
    let cache = CoinsCache::from_parent(parent);

    // Act
    let have_coin = cache.have_coin(&outpoint);
    let best_block = cache.best_block();

    // Assert
    assert_eq!(have_coin, Ok(true));
    assert_eq!(best_block, Ok(maybe_best_block));
    assert!(!cache.contains_in_cache(&outpoint));
}

#[test]
fn coins_cache_batch_write_installs_overlay_and_best_block() {
    // Arrange
    let outpoint = fixture_outpoint();
    let coin = fixture_coin();
    let maybe_best_block = Some(fixture_best_block());
    let parent = MemoryCoinsView::from_coins(HashMap::new(), None);
    let mut cache = CoinsCache::from_parent(parent);
    let mut writes = HashMap::new();
    writes.insert(
        outpoint.clone(),
        CoinsCacheEntry::unspent_dirty(coin.clone()),
    );

    // Act
    let write_result = cache.batch_write(CoinsBatch { entries: writes }, maybe_best_block);

    // Assert
    assert_eq!(write_result, Ok(()));
    assert!(cache.contains_in_cache(&outpoint));
    assert!(cache.have_coin_in_cache(&outpoint));
    assert_eq!(cache.get_coin(&outpoint), Ok(Some(coin)));
    assert_eq!(cache.best_block(), Ok(maybe_best_block));
}

#[test]
fn memory_coins_view_batch_write_inserts_unspent_and_updates_best_block() {
    // Arrange
    let outpoint = fixture_outpoint();
    let coin = fixture_coin();
    let maybe_best_block = Some(fixture_best_block());
    let mut view = MemoryCoinsView::from_coins(HashMap::new(), None);
    let mut writes = HashMap::new();
    writes.insert(
        outpoint.clone(),
        CoinsCacheEntry::unspent_clean(coin.clone()),
    );

    // Act
    let write_result = view.batch_write(CoinsBatch { entries: writes }, maybe_best_block);

    // Assert
    assert_eq!(write_result, Ok(()));
    assert_eq!(view.get_coin(&outpoint), Ok(Some(coin)));
    assert!(view.contains_outpoint(&outpoint));
    assert_eq!(view.best_block(), Ok(maybe_best_block));
}
