// Parity breadcrumbs:
// - packages/bitcoin-knots/src/coins.h
// - packages/bitcoin-knots/src/coins.cpp
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp
// - packages/bitcoin-knots/src/node/chainstate.cpp

use std::collections::HashMap;

use open_bitcoin_primitives::{Amount, BlockHash, OutPoint, ScriptBuf, TransactionOutput, Txid};

mod algebra;
mod flush;

use super::{
    CoinsBatch, CoinsCache, CoinsCacheEntry, CoinsCacheFlags, CoinsView,
    ESTIMATED_COIN_ENTRY_OVERHEAD_BYTES, MemoryCoinsView,
};
use crate::error::ChainstateError;
use crate::types::Coin;
use crate::{Chainstate, ChainstateSnapshot, MemoryBackedChainstate};

fn fixture_outpoint() -> OutPoint {
    OutPoint {
        txid: Txid::from_byte_array([1_u8; 32]),
        vout: 0,
    }
}

fn fixture_coin() -> Coin {
    Coin {
        output: TransactionOutput {
            value: Amount::from_sats(50).expect("valid amount"),
            script_pubkey: ScriptBuf::from_bytes(vec![0x51]).expect("valid script"),
        },
        is_coinbase: false,
        created_height: 1,
        created_median_time_past: 0,
    }
}

fn fixture_best_block() -> BlockHash {
    BlockHash::from_byte_array([2_u8; 32])
}

fn fixture_outpoint_two() -> OutPoint {
    OutPoint {
        txid: Txid::from_byte_array([3_u8; 32]),
        vout: 1,
    }
}

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

struct FailingCoinsView;

impl CoinsView for FailingCoinsView {
    fn get_coin(&self, _outpoint: &OutPoint) -> Result<Option<Coin>, ChainstateError> {
        Err(ChainstateError::CoinsStorage {
            detail: "injected-disk-failure".to_string(),
        })
    }

    fn have_coin(&self, _outpoint: &OutPoint) -> Result<bool, ChainstateError> {
        Err(ChainstateError::CoinsStorage {
            detail: "injected-disk-failure".to_string(),
        })
    }

    fn best_block(&self) -> Result<Option<BlockHash>, ChainstateError> {
        Err(ChainstateError::CoinsStorage {
            detail: "injected-disk-failure".to_string(),
        })
    }

    fn head_blocks(&self) -> Result<Vec<BlockHash>, ChainstateError> {
        Err(ChainstateError::CoinsStorage {
            detail: "injected-disk-failure".to_string(),
        })
    }

    fn batch_write(
        &mut self,
        _writes: CoinsBatch,
        _maybe_best_block: Option<BlockHash>,
    ) -> Result<(), ChainstateError> {
        Ok(())
    }
}

#[test]
fn memory_coins_view_best_block_and_head_blocks_are_ok() {
    // Arrange
    let maybe_best_block = Some(BlockHash::from_byte_array([2_u8; 32]));
    let view = MemoryCoinsView::from_coins(HashMap::new(), maybe_best_block);

    // Act
    let best_block = view.best_block();
    let heads = view.head_blocks();

    // Assert
    assert_eq!(best_block, Ok(maybe_best_block));
    assert_eq!(heads, Ok(Vec::new()));
}

#[test]
fn coins_cache_propagates_parent_coins_storage_on_get_have_best_and_heads() {
    // Arrange
    let failing = FailingCoinsView;
    let cache = CoinsCache::from_parent(failing);
    let outpoint = fixture_outpoint();
    let expected = ChainstateError::CoinsStorage {
        detail: "injected-disk-failure".to_string(),
    };

    // Act
    let maybe_coin = cache.get_coin(&outpoint);
    let have_coin = cache.have_coin(&outpoint);
    let best_block = cache.best_block();
    let heads = cache.head_blocks();

    // Assert
    assert_eq!(maybe_coin, Err(expected.clone()));
    assert_eq!(have_coin, Err(expected.clone()));
    assert_eq!(best_block, Err(expected.clone()));
    assert_eq!(heads, Err(expected));
    assert!(!matches!(maybe_coin, Ok(None)));
    assert!(!matches!(have_coin, Ok(false)));
    assert!(!matches!(
        maybe_coin,
        Err(ChainstateError::MissingCoin { .. })
    ));
    assert!(!matches!(
        have_coin,
        Err(ChainstateError::MissingCoin { .. })
    ));
    assert!(!matches!(
        best_block,
        Err(ChainstateError::MissingCoin { .. })
    ));
    assert!(!matches!(heads, Err(ChainstateError::MissingCoin { .. })));
}

#[test]
fn coins_storage_display_is_not_missing_coin() {
    // Arrange
    let error = ChainstateError::CoinsStorage {
        detail: "decode failed".into(),
    };

    // Act
    let message = error.to_string();

    // Assert
    assert_eq!(message, "coins storage error: decode failed");
    assert!(!message.contains("missing coin"));
}

#[test]
fn cache_entry_count_is_overlay_len() {
    // Arrange
    let mut cache = CoinsCache::from_parent(MemoryCoinsView::default());

    // Act
    let empty_count = cache.cache_entry_count();
    cache.insert_entry_for_test(fixture_outpoint(), CoinsCacheEntry::spent_dirty());
    cache.insert_entry_for_test(
        fixture_outpoint_two(),
        CoinsCacheEntry::unspent_dirty(fixture_coin()),
    );
    let occupied_count = cache.cache_entry_count();

    // Assert
    assert_eq!(empty_count, 0);
    assert_eq!(occupied_count, 2);
}

#[test]
fn estimated_cache_bytes_uses_first_party_overhead() {
    // Arrange
    let unspent = Coin {
        output: TransactionOutput {
            value: Amount::from_sats(50).expect("valid amount"),
            script_pubkey: ScriptBuf::from_bytes(vec![0x51]).expect("valid script"),
        },
        is_coinbase: true,
        created_height: 7,
        created_median_time_past: 1_000,
    };
    let mut cache = CoinsCache::from_parent(MemoryCoinsView::default());
    let expected_unspent = ESTIMATED_COIN_ENTRY_OVERHEAD_BYTES + 8 + 1 + 4 + 8 + 1;
    let expected_spent = ESTIMATED_COIN_ENTRY_OVERHEAD_BYTES;
    let cache_source = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/coins/cache.rs"));

    // Act
    cache.insert_entry_for_test(fixture_outpoint(), CoinsCacheEntry::unspent_dirty(unspent));
    let after_unspent = cache.estimated_cache_bytes();
    cache.insert_entry_for_test(fixture_outpoint_two(), CoinsCacheEntry::spent_dirty());
    let after_both = cache.estimated_cache_bytes();

    // Assert
    assert_eq!(after_unspent, expected_unspent);
    assert_eq!(after_both, expected_unspent + expected_spent);
    assert!(!cache_source.contains("SizeEstimate"));
    assert!(!cache_source.contains("fjall"));
}

#[test]
fn from_parent_does_not_probe_best_block() {
    // Arrange
    let parent_tip = BlockHash::from_byte_array([9_u8; 32]);
    let parent = MemoryCoinsView::from_coins(HashMap::new(), Some(parent_tip));

    // Act
    let chainstate =
        Chainstate::from_parent(parent, Vec::new(), HashMap::new(), Some(HashMap::new()));

    // Assert
    assert_eq!(chainstate.coins().cache_entry_count(), 0);
    assert_eq!(chainstate.coins_best_block(), Ok(Some(parent_tip)));
}

#[test]
fn memory_backed_alias_is_default_chainstate() {
    // Arrange
    let snapshot = ChainstateSnapshot::new(Vec::new(), HashMap::new(), HashMap::new());

    // Act
    let from_snapshot = Chainstate::from_snapshot(snapshot);
    let from_default: MemoryBackedChainstate = Chainstate::default();

    // Assert
    let aliased: MemoryBackedChainstate = from_snapshot;
    assert_eq!(from_default, Chainstate::new());
    assert_eq!(aliased.coins_best_block(), Ok(None));
}
