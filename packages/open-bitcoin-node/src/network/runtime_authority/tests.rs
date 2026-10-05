// Parity breadcrumbs:
// - packages/bitcoin-knots/src/node/context.h

use std::{
    collections::HashMap,
    fs, io,
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

use open_bitcoin_core::{
    chainstate::{
        BlockUndo, ChainPosition, Chainstate, ChainstateSnapshot, Coin, FlushMode, FlushPolicyTime,
        PrunePlan,
    },
    consensus::{ConsensusParams, ScriptVerifyFlags, block_hash},
    primitives::{
        Amount, Block, BlockHash, BlockHeader, MerkleRoot, OutPoint, ScriptBuf, ScriptWitness,
        Transaction, TransactionInput, TransactionOutput, Txid,
    },
};
use open_bitcoin_mempool::{MempoolOutcome, PolicyConfig, PolicyTime, RelayIntent};
use open_bitcoin_network::LocalPeerConfig;

use crate::storage::{FjallNodeStore, PersistMode};
use crate::{FjallChainstateStore, ManagedPeerNetwork, MemoryChainstateStore};

use super::{ManagedNetworkAuthorityError, ManagedNetworkHandle};

fn test_handle() -> ManagedNetworkHandle {
    let network = ManagedPeerNetwork::new(
        MemoryChainstateStore::default(),
        LocalPeerConfig::default(),
        PolicyConfig::default(),
    );
    ManagedNetworkHandle::new(network)
}

fn orphan_transaction() -> Transaction {
    Transaction {
        version: 2,
        inputs: vec![TransactionInput {
            previous_output: OutPoint {
                txid: Txid::from_byte_array([7_u8; 32]),
                vout: 0,
            },
            script_sig: ScriptBuf::from_bytes(vec![0x51]).expect("valid script"),
            sequence: TransactionInput::SEQUENCE_FINAL,
            witness: ScriptWitness::default(),
        }],
        outputs: vec![TransactionOutput {
            value: Amount::from_sats(1_000).expect("valid amount"),
            script_pubkey: ScriptBuf::from_bytes(vec![0x51]).expect("valid script"),
        }],
        lock_time: 0,
    }
}

#[test]
fn cloned_handles_share_mutations() {
    // Arrange
    let mutating_handle = test_handle();
    let snapshot_handle = mutating_handle.clone();

    // Act
    mutating_handle
        .connect_outbound_peer(1, 1_777_225_210)
        .expect("shared authority should accept the peer");
    let snapshot = snapshot_handle
        .network_info()
        .expect("shared authority should return an owned snapshot");

    // Assert
    assert_eq!(snapshot.outbound_peers, 1);
}

#[test]
fn owned_snapshot_survives_authority_drop() {
    // Arrange
    let handle = test_handle();

    // Act
    let snapshot = handle
        .chainstate_snapshot()
        .expect("shared authority should return an owned snapshot");
    drop(handle);

    // Assert
    assert!(snapshot.active_chain.is_empty());
}

#[test]
fn poisoned_authority_returns_typed_error() {
    // Arrange
    let handle = test_handle();
    handle.poison_for_test();

    // Act
    let result = handle.operator_snapshot();

    // Assert
    assert!(matches!(
        result,
        Err(ManagedNetworkAuthorityError::Poisoned)
    ));
}

#[test]
fn explicit_local_admission_flows_through_the_shared_authority() {
    // Arrange
    let handle = test_handle();

    // Act
    let outcome = handle
        .submit_local_transaction_outcome_at(
            orphan_transaction(),
            ScriptVerifyFlags::NONE,
            ConsensusParams::default(),
            50,
            RelayIntent::Requested,
        )
        .expect("authority should return an admission outcome");

    // Assert
    assert!(matches!(outcome, MempoolOutcome::Orphaned { .. }));
}

#[test]
fn expire_mempool_flows_through_the_shared_authority() {
    // Arrange — empty authority; membership/serving age fixtures live in
    // mempool_lifecycle_cases (`expire_mempool_authority_hook_removes_aged_entry`).
    let handle = test_handle();

    // Act
    let delta = handle
        .expire_mempool(PolicyTime::new(1_000))
        .expect("expire through authority");

    // Assert
    assert!(delta.is_empty());
}

#[test]
fn flush_applying_prune_plan_drops_only_the_deleted_hash() {
    // Arrange
    let (path, store) = open_temp_store("cache-evict");
    let (eligible, eligible_block) = plant_payload(&store, 1, 11);
    let tip = ChainPosition::new(
        test_header(eligible.block_hash, 22),
        400,
        400,
        1_700_000_000,
    );
    let kept_header = test_header(BlockHash::from_byte_array([9_u8; 32]), 33);
    let kept_hash = block_hash(&kept_header);
    let kept_block = Block {
        header: kept_header,
        transactions: Vec::new(),
    };
    let mut undo_by_block = HashMap::new();
    undo_by_block.insert(eligible.block_hash, BlockUndo::default());
    let mut network = ManagedPeerNetwork::new(
        FjallChainstateStore::from_store(store),
        LocalPeerConfig::default(),
        PolicyConfig::default(),
    );
    network
        .chainstate_mut()
        .install_chainstate_for_test(Chainstate::from_snapshot(ChainstateSnapshot::new(
            vec![eligible.clone(), tip],
            HashMap::new(),
            undo_by_block,
        )));
    network
        .blocks_by_hash
        .insert(eligible.block_hash, eligible_block);
    network.blocks_by_hash.insert(kept_hash, kept_block);
    let handle = ManagedNetworkHandle::new(network);
    let plan = PrunePlan { heights: vec![1] };

    // Act
    let execution = handle
        .flush_applying_prune_plan(
            FlushMode::Always,
            FlushPolicyTime::from_unix_seconds(1_700_000_000),
            u64::MAX,
            &plan,
            &[],
        )
        .expect("prune flush should apply the plan");

    // Assert
    assert_eq!(execution.deleted_block_hashes, vec![eligible.block_hash]);
    let network = handle.authority.lock().expect("test authority should lock");
    assert!(!network.blocks_by_hash.contains_key(&eligible.block_hash));
    assert!(network.blocks_by_hash.contains_key(&kept_hash));
    let payload_present = network
        .chainstate()
        .store()
        .inner()
        .has_block(eligible.block_hash)
        .expect("payload probe");
    let undo_present = network
        .chainstate()
        .store()
        .inner()
        .has_undo(eligible.block_hash)
        .expect("undo probe");
    assert!(!payload_present);
    assert!(!undo_present);
    drop(network);
    drop(handle);
    remove_dir_if_exists(&path);
}

#[test]
fn error_after_unlink_drops_deleted_hash_and_retry_leaves_it_gone() {
    // Arrange
    let (path, store) = open_temp_store("cache-evict-error");
    let (eligible, eligible_block) = plant_payload(&store, 1, 11);
    let tip = ChainPosition::new(
        test_header(eligible.block_hash, 22),
        400,
        400,
        1_700_000_000,
    );
    let kept_header = test_header(BlockHash::from_byte_array([9_u8; 32]), 33);
    let kept_hash = block_hash(&kept_header);
    let kept_block = Block {
        header: kept_header,
        transactions: Vec::new(),
    };
    let mut undo_by_block = HashMap::new();
    undo_by_block.insert(eligible.block_hash, BlockUndo::default());
    let mut network = ManagedPeerNetwork::new(
        FjallChainstateStore::from_store(store),
        LocalPeerConfig::default(),
        PolicyConfig::default(),
    );
    network
        .chainstate_mut()
        .install_chainstate_for_test(Chainstate::from_snapshot(ChainstateSnapshot::new(
            vec![eligible.clone(), tip],
            HashMap::new(),
            undo_by_block,
        )));
    network
        .chainstate_mut()
        .insert_overlay_coin_for_test(
            OutPoint {
                txid: Txid::from_byte_array([0x11; 32]),
                vout: 0,
            },
            Coin {
                output: TransactionOutput {
                    value: Amount::from_sats(50).expect("valid amount"),
                    script_pubkey: ScriptBuf::from_bytes(vec![0x51]).expect("valid script"),
                },
                is_coinbase: false,
                created_height: 1,
                created_median_time_past: 1_700_000_001,
            },
        )
        .expect("overlay coin");
    assert_eq!(
        network
            .chainstate()
            .chainstate()
            .coins()
            .cache_entry_count(),
        1
    );
    network
        .blocks_by_hash
        .insert(eligible.block_hash, eligible_block);
    network.blocks_by_hash.insert(kept_hash, kept_block);
    let handle = ManagedNetworkHandle::new(network);
    let plan = PrunePlan { heights: vec![1] };

    // Act
    let refused = handle.flush_applying_prune_plan(
        FlushMode::Always,
        FlushPolicyTime::from_unix_seconds(1_700_000_000),
        0,
        &plan,
        &[],
    );

    // Assert
    let Err(ManagedNetworkAuthorityError::LifecycleEffect(message)) = refused else {
        panic!("expected disk-space refusal, got {refused:?}");
    };
    assert!(message.contains("refuse disk space"));
    {
        let network = handle.authority.lock().expect("test authority should lock");
        assert!(!network.blocks_by_hash.contains_key(&eligible.block_hash));
        assert!(network.blocks_by_hash.contains_key(&kept_hash));
        let payload_present = network
            .chainstate()
            .store()
            .inner()
            .has_block(eligible.block_hash)
            .expect("payload probe");
        let undo_present = network
            .chainstate()
            .store()
            .inner()
            .has_undo(eligible.block_hash)
            .expect("undo probe");
        assert!(!payload_present);
        assert!(!undo_present);
    }

    // Act
    let retry = handle
        .flush_applying_prune_plan(
            FlushMode::Always,
            FlushPolicyTime::from_unix_seconds(1_700_000_000),
            u64::MAX,
            &plan,
            &[],
        )
        .expect("retry of an already-absent height should succeed");

    // Assert
    assert!(retry.deleted_block_hashes.is_empty());
    let network = handle.authority.lock().expect("test authority should lock");
    assert!(!network.blocks_by_hash.contains_key(&eligible.block_hash));
    assert!(network.blocks_by_hash.contains_key(&kept_hash));
    drop(network);
    drop(handle);
    remove_dir_if_exists(&path);
}

#[test]
fn flush_coins_empty_plan_keeps_unrelated_cached_hash() {
    // Arrange
    let handle = test_handle();
    let kept_header = test_header(BlockHash::from_byte_array([4_u8; 32]), 8);
    let kept_hash = block_hash(&kept_header);
    let kept_block = Block {
        header: kept_header,
        transactions: Vec::new(),
    };
    handle
        .authority
        .lock()
        .expect("test authority should lock")
        .blocks_by_hash
        .insert(kept_hash, kept_block);

    // Act
    let execution = handle
        .flush_coins(
            FlushMode::Always,
            FlushPolicyTime::from_unix_seconds(1_700_000_000),
            u64::MAX,
        )
        .expect("empty plan flush should succeed");

    // Assert
    assert!(execution.deleted_block_hashes.is_empty());
    let network = handle.authority.lock().expect("test authority should lock");
    assert!(network.blocks_by_hash.contains_key(&kept_hash));
}

fn open_temp_store(test_name: &str) -> (PathBuf, FjallNodeStore) {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time after unix epoch")
        .as_nanos();
    let path = std::env::temp_dir().join(format!(
        "open-bitcoin-prune-cache-{test_name}-{}-{timestamp}",
        std::process::id()
    ));
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("open temp store");
    (path, store)
}

fn remove_dir_if_exists(path: &Path) {
    match fs::remove_dir_all(path) {
        Ok(()) => {}
        Err(error) if error.kind() == io::ErrorKind::NotFound => {}
        Err(error) => panic!("failed to remove {}: {error}", path.display()),
    }
}

fn test_header(previous_block_hash: BlockHash, nonce: u32) -> BlockHeader {
    BlockHeader {
        version: 1,
        previous_block_hash,
        merkle_root: MerkleRoot::from_byte_array([nonce as u8; 32]),
        time: 1_700_000_000 + nonce,
        bits: 0x207f_ffff,
        nonce,
    }
}

fn plant_payload(store: &FjallNodeStore, height: u32, nonce: u32) -> (ChainPosition, Block) {
    let header = test_header(BlockHash::from_byte_array([0_u8; 32]), nonce);
    let position = ChainPosition::new(header.clone(), height, u128::from(height), 1_700_000_000);
    let body = Block {
        header,
        transactions: Vec::new(),
    };
    let saved = store
        .save_block(&body, PersistMode::Sync)
        .expect("save payload");
    assert_eq!(saved, position.block_hash);
    store
        .save_undo(
            position.block_hash,
            &BlockUndo::default(),
            PersistMode::Sync,
        )
        .expect("save undo");
    (position, body)
}

#[test]
fn prune_reserved_handle_mutations_refuse_even_when_absent() {
    // Arrange
    let (path, store) = open_temp_store("reserved-crud");
    let network = ManagedPeerNetwork::new(
        FjallChainstateStore::from_store(store.clone()),
        Default::default(),
        Default::default(),
    );
    let handle = ManagedNetworkHandle::new(network);
    let reserved = open_bitcoin_core::chainstate::BASIC_INDEX_PRUNE_LOCK;

    // Act
    let replace = handle.replace_prune_lock(open_bitcoin_core::chainstate::PruneLockInfo {
        name: reserved.to_owned(),
        height_first: 0,
        height_last: 10,
    });
    let clear = handle.clear_prune_lock(reserved);

    // Assert
    for result in [replace, clear.map(|_| ())] {
        assert!(
            result
                .expect_err("reserved refusal")
                .to_string()
                .contains("internally owned")
        );
    }
    assert!(
        !handle
            .clear_prune_lock("missing")
            .expect("ordinary absence")
    );
    assert!(handle.list_prune_locks().expect("locks").is_empty());
    drop(handle);
    drop(store);
    let reopened = FjallNodeStore::open(&path).expect("reopen");
    assert!(reopened.load_prune_locks().expect("unchanged").is_empty());
    drop(reopened);
    remove_dir_if_exists(&path);
}

#[test]
fn prune_reserved_handle_refuses_every_saved_owner_and_preserves_ordinary_crud() {
    use crate::storage::StorageNamespace;
    use crate::storage::filter_index::ownership::{OWNER_KEY, encode_owner};
    use open_bitcoin_core::chainstate::filter_index::lifecycle::{IndexGeneration, IndexLifecycle};
    use open_bitcoin_core::chainstate::{
        BASIC_INDEX_PRUNE_LOCK, PruneLockInfo, VerifiedChainstateFence,
    };
    for mode in 0..4 {
        // Arrange: Active, Disabled released/retained, exhausted Active.
        let (path, store) = open_temp_store("reserved-owner-modes");
        let position = ChainPosition::new(test_header(BlockHash::default(), 0), 0, 1, 1);
        let snapshot =
            ChainstateSnapshot::new(vec![position], Default::default(), Default::default());
        store
            .seed_coins_from_snapshot(&snapshot)
            .expect("durable fence");
        let fence = VerifiedChainstateFence::new(
            Some(snapshot.active_chain[0].block_hash),
            Some(&snapshot.active_chain),
        )
        .expect("fence");
        store
            .initialize_basic_filter_state(&fence)
            .expect("internal initialization");
        match mode {
            1 => store.disable_basic_filter_index().expect("disable"),
            2 | 3 => store
                .write_raw_for_test(
                    StorageNamespace::BlockIndex,
                    OWNER_KEY,
                    encode_owner(if mode == 2 {
                        IndexLifecycle::Disabled {
                            generation: IndexGeneration::new(1),
                        }
                    } else {
                        IndexLifecycle::Active {
                            generation: IndexGeneration::new(u64::MAX),
                        }
                    })
                    .to_vec(),
                )
                .expect("explicit recovery fixture"),
            _ => {}
        }
        let original = store.load_prune_locks().expect("locks");
        let maybe_state = store.maybe_basic_filter_state().expect("state");
        let maybe_owner = store
            .maybe_basic_filter_lifecycle_for_test()
            .expect("owner");
        let network = ManagedPeerNetwork::new(
            FjallChainstateStore::from_store(store.clone()),
            Default::default(),
            Default::default(),
        );
        let handle = ManagedNetworkHandle::new(network);

        // Act / Assert
        assert!(
            handle
                .replace_prune_lock(PruneLockInfo {
                    name: BASIC_INDEX_PRUNE_LOCK.into(),
                    height_first: 0,
                    height_last: 10
                })
                .is_err()
        );
        assert!(handle.clear_prune_lock(BASIC_INDEX_PRUNE_LOCK).is_err());
        handle
            .replace_prune_lock(PruneLockInfo {
                name: "ordinary".into(),
                height_first: 20,
                height_last: 30,
            })
            .expect("ordinary set");
        handle
            .replace_prune_lock(PruneLockInfo {
                name: "ordinary".into(),
                height_first: 21,
                height_last: 31,
            })
            .expect("ordinary replace");
        assert!(!handle.clear_prune_lock("missing").expect("ordinary absent"));
        assert!(handle.clear_prune_lock("ordinary").expect("ordinary clear"));
        assert_eq!(handle.list_prune_locks().expect("owner"), original);
        assert_eq!(
            store.maybe_basic_filter_state().expect("state"),
            maybe_state
        );
        assert_eq!(
            store
                .maybe_basic_filter_lifecycle_for_test()
                .expect("owner"),
            maybe_owner
        );
        drop(handle);
        drop(store);
        let reopened = FjallNodeStore::open(&path).expect("reopen");
        assert_eq!(reopened.load_prune_locks().expect("owner"), original);
        assert_eq!(
            reopened.maybe_basic_filter_state().expect("state"),
            maybe_state
        );
        assert_eq!(
            reopened
                .maybe_basic_filter_lifecycle_for_test()
                .expect("owner"),
            maybe_owner
        );
        drop(reopened);
        remove_dir_if_exists(&path);
    }
}
