// Parity breadcrumbs:
// - packages/bitcoin-knots/src/coins.h
// - packages/bitcoin-knots/src/coins.cpp
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp
// - packages/bitcoin-knots/src/node/chainstate.cpp

use super::*;

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
fn coins_mut_exposes_the_same_live_cache() {
    // Arrange
    let parent_tip = BlockHash::from_byte_array([9_u8; 32]);
    let parent = MemoryCoinsView::from_coins(HashMap::new(), Some(parent_tip));
    let mut chainstate =
        Chainstate::from_parent(parent, Vec::new(), HashMap::new(), Some(HashMap::new()));

    // Act
    let count = chainstate.coins_mut().cache_entry_count();

    // Assert
    assert_eq!(count, 0);
    assert_eq!(chainstate.coins_best_block(), Ok(Some(parent_tip)));
}

#[test]
fn into_dirty_parent_write_returns_parent_and_dirty_overlay() {
    // Arrange
    let parent_tip = BlockHash::from_byte_array([3_u8; 32]);
    let new_tip = BlockHash::from_byte_array([4_u8; 32]);
    let parent = MemoryCoinsView::from_coins(HashMap::new(), Some(parent_tip));
    let mut cache = CoinsCache::from_parent(parent);
    let outpoint = fixture_outpoint();
    let coin = fixture_coin();
    cache
        .add_coin(outpoint.clone(), coin.clone(), true)
        .expect("add dirty coin");
    cache.set_best_block(new_tip);

    // Act
    let (parent, writes) = cache.into_dirty_parent_write();

    // Assert
    assert_eq!(parent.best_block().expect("parent tip"), Some(parent_tip));
    assert_eq!(writes.entries.len(), 1);
    let entry = writes.entries.get(&outpoint).expect("dirty outpoint");
    assert_eq!(entry.maybe_coin(), Some(&coin));
    assert!(entry.is_dirty());
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

#[test]
fn default_collect_unspent_hint_is_empty() {
    // Arrange
    let view = HintlessView;

    // Act
    let hint = CoinsView::collect_unspent_hint(&view).expect("default hint");

    // Assert
    assert!(hint.is_empty());
}

#[test]
fn memory_collect_unspent_hint_returns_unspent_coins() {
    // Arrange
    let outpoint = fixture_outpoint();
    let coin = fixture_coin();
    let mut coins = HashMap::new();
    coins.insert(outpoint.clone(), coin.clone());
    let view = MemoryCoinsView::from_coins(coins, None);

    // Act
    let hint = CoinsView::collect_unspent_hint(&view).expect("memory hint");

    // Assert
    assert_eq!(hint.get(&outpoint), Some(&coin));
}

#[test]
fn cache_admission_unspent_merges_parent_hint_with_overlay() {
    // Arrange
    let parent_outpoint = fixture_outpoint();
    let overlay_outpoint = fixture_outpoint_two();
    let parent_coin = fixture_coin();
    let overlay_coin = fixture_coin();
    let mut parent_coins = HashMap::new();
    parent_coins.insert(parent_outpoint.clone(), parent_coin);
    let mut cache = CoinsCache::from_parent(MemoryCoinsView::from_coins(parent_coins, None));
    cache.insert_entry_for_test(
        overlay_outpoint.clone(),
        CoinsCacheEntry::unspent_dirty(overlay_coin.clone()),
    );
    cache.insert_entry_for_test(parent_outpoint.clone(), CoinsCacheEntry::spent_dirty());

    // Act
    let admission = cache.collect_admission_unspent().expect("admission hint");
    let overlay = cache.collect_overlay_unspent();

    // Assert
    assert!(!admission.contains_key(&parent_outpoint));
    assert_eq!(admission.get(&overlay_outpoint), Some(&overlay_coin));
    assert!(!overlay.contains_key(&parent_outpoint));
    assert_eq!(overlay.get(&overlay_outpoint), Some(&overlay_coin));
}

#[test]
fn from_coins_cache_exposes_admission_and_overlay_snapshots() {
    // Arrange
    let outpoint = fixture_outpoint();
    let coin = fixture_coin();
    let tip = fixture_best_block();
    let mut parent_coins = HashMap::new();
    parent_coins.insert(outpoint.clone(), coin.clone());
    let cache = CoinsCache::from_parent(MemoryCoinsView::from_coins(parent_coins, Some(tip)));
    let mut undo_by_block = HashMap::new();
    undo_by_block.insert(tip, BlockUndo::default());

    // Act
    let mut chainstate = Chainstate::from_coins_cache(
        cache,
        Vec::new(),
        undo_by_block.clone(),
        Some(HashMap::new()),
    );
    let maybe_found = chainstate.get_coin(&outpoint).expect("lookup");
    let admission = chainstate.admission_snapshot().expect("admission snapshot");
    let overlay = chainstate.overlay_snapshot();
    let overlay_count = chainstate.coins().cache_entry_count();
    let live_count = chainstate.coins_mut().cache_entry_count();

    // Assert
    assert!(chainstate.active_chain().is_empty());
    assert_eq!(chainstate.undo_by_block(), &undo_by_block);
    assert_eq!(maybe_found, Some(coin.clone()));
    assert_eq!(admission.utxos.get(&outpoint), Some(&coin));
    assert!(overlay.utxos.is_empty());
    assert_eq!(overlay_count, 0);
    assert_eq!(live_count, 0);
}
