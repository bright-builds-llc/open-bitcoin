// Parity breadcrumbs:
// - packages/bitcoin-knots/src/node/blockstorage.cpp
// - packages/bitcoin-knots/src/validation.cpp

use super::super::ManagedNetworkHandle;
use crate::chainstate::FlushPersistSink;
use crate::storage::{
    StorageError, StorageNamespace, StorageRecoveryAction,
    fjall_store::{PairedDeleteOutcome, PayloadUsageRevision, RetainedPayloadUsage},
};
use crate::{ChainstateStore, ManagedPeerNetwork, MemoryChainstateStore};
use open_bitcoin_core::{
    chainstate::{
        BlockUndo, ChainPosition, ChainstateError, ChainstateSnapshot, Coin, CoinsBatch, FlushMode,
        FlushPolicyTime, PruneLockInfo, PruneMode,
    },
    primitives::{Block, BlockHash, BlockHeader, OutPoint},
};
use open_bitcoin_network::HeaderEntry;
use std::{
    collections::{BTreeMap, HashMap},
    sync::{Arc, Mutex},
};

mod fixtures;
mod protection;
mod writers;
use fixtures::*;

#[test]
fn automatic_prune_current_tip_threshold_is_a_cheap_gate() {
    // Arrange
    let (handle, facts) = fixture(PruneMode::Automatic { target_mib: 550 }, TARGET + 500);
    handle
        .set_prune_network(crate::SyncNetwork::Mainnet)
        .expect("mainnet threshold");
    // Act
    tick(&handle, FlushMode::Always, 100).expect("short chain");
    // Assert
    assert_eq!(facts.lock().expect("facts").scans, 0);
}

#[test]
fn automatic_prune_at_or_below_network_threshold_does_not_measure() {
    // Arrange / Act
    for height in [999, 1_000] {
        let (handle, facts) = fixture(PruneMode::Automatic { target_mib: 550 }, TARGET + 500);
        handle
            .mutate(|network| {
                network.chainstate_mut().install_chainstate_for_test(
                    open_bitcoin_core::chainstate::Chainstate::from_snapshot(
                        ChainstateSnapshot::new(
                            vec![position(10), position(height)],
                            HashMap::new(),
                            HashMap::new(),
                        ),
                    ),
                )
            })
            .expect("threshold tip");
        tick(&handle, FlushMode::Periodic, 100).expect("gated flush");
        // Assert
        assert_eq!(facts.lock().expect("facts").scans, 0);
        assert!(facts.lock().expect("facts").deletes.is_empty());
    }
}

#[test]
fn automatic_prune_keeps_exact_trailing_window_above_target() {
    // Arrange
    let (handle, facts) = fixture(PruneMode::Automatic { target_mib: 550 }, TARGET + 500);
    // Act
    tick(&handle, FlushMode::Periodic, 100).expect("flush");
    // Assert
    assert_eq!(facts.lock().expect("facts").deletes, vec![10, 712, 713]);
    assert!(facts.lock().expect("facts").usage > TARGET);
}

#[test]
fn automatic_prune_fully_protected_retains_requested_mode_above_target() {
    // Arrange
    let (handle, facts) = fixture(PruneMode::Automatic { target_mib: 550 }, TARGET + 500);
    handle
        .replace_prune_lock(PruneLockInfo {
            name: "wallet".into(),
            height_first: 20,
            height_last: 713,
        })
        .expect("lock");
    // Act
    let execution = tick(&handle, FlushMode::Periodic, 100).expect("protected flush");
    // Assert
    assert!(!execution.wrote_coins);
    assert!(facts.lock().expect("facts").deletes.is_empty());
}

#[test]
fn automatic_prune_unchanged_periodic_activity_skips_exact_scan() {
    // Arrange
    let (handle, facts) = fixture(PruneMode::Automatic { target_mib: 550 }, TARGET);
    tick(&handle, FlushMode::Periodic, 100).expect("first");
    // Act
    tick(&handle.clone(), FlushMode::Periodic, 200).expect("idle clone");
    // Assert
    assert_eq!(facts.lock().expect("facts").scans, 1);
}

#[test]
fn automatic_prune_changed_periodic_activity_coalesces_without_cached_deletes() {
    // Arrange
    let (handle, facts) = fixture(PruneMode::Automatic { target_mib: 550 }, TARGET);
    tick(&handle, FlushMode::Periodic, 100).expect("first");
    {
        let mut facts = facts.lock().expect("facts");
        facts.usage = TARGET + 50;
        facts.generation += 1;
    }
    // Act
    tick(&handle, FlushMode::Periodic, 159).expect("deferred");
    let deferred_deletes = facts.lock().expect("facts").deletes.clone();
    tick(&handle, FlushMode::Periodic, 160).expect("eligible");
    // Assert
    assert!(deferred_deletes.is_empty());
    assert_eq!(facts.lock().expect("facts").scans, 2);
    assert_eq!(facts.lock().expect("facts").deletes, vec![10]);
}

#[test]
fn automatic_prune_always_bypasses_changed_periodic_coalescing() {
    // Arrange
    let (handle, facts) = fixture(PruneMode::Automatic { target_mib: 550 }, TARGET);
    tick(&handle, FlushMode::Periodic, 100).expect("first");
    {
        let mut facts = facts.lock().expect("facts");
        facts.usage = TARGET + 50;
        facts.generation += 1;
    }
    // Act
    tick(&handle, FlushMode::Always, 101).expect("always");
    // Assert
    assert_eq!(facts.lock().expect("facts").scans, 2);
    assert_eq!(facts.lock().expect("facts").deletes, vec![10]);
}

#[test]
fn automatic_prune_tip_mode_and_lock_changes_require_new_facts() {
    // Arrange / Act
    for changed in ["tip", "tip_hash", "mode", "locks"] {
        let (mut handle, facts) = fixture(PruneMode::Automatic { target_mib: 550 }, TARGET);
        tick(&handle, FlushMode::Periodic, 100).expect("first");
        match changed {
            "tip" => {
                handle
                    .mutate(|network| {
                        network.chainstate_mut().install_chainstate_for_test(
                            open_bitcoin_core::chainstate::Chainstate::from_snapshot(
                                ChainstateSnapshot::new(
                                    vec![position(10), position(1_002)],
                                    HashMap::new(),
                                    HashMap::new(),
                                ),
                            ),
                        )
                    })
                    .expect("tip change");
            }
            "mode" => handle
                .set_prune_mode(PruneMode::Automatic { target_mib: 551 })
                .expect("mode"),
            "tip_hash" => {
                let mut replacement = position(1_001);
                replacement.header.nonce += 1;
                replacement.block_hash =
                    open_bitcoin_core::consensus::block_hash(&replacement.header);
                handle
                    .mutate(|network| {
                        network.chainstate_mut().install_chainstate_for_test(
                            open_bitcoin_core::chainstate::Chainstate::from_snapshot(
                                ChainstateSnapshot::new(
                                    vec![position(10), replacement],
                                    HashMap::new(),
                                    HashMap::new(),
                                ),
                            ),
                        )
                    })
                    .expect("same-height reorg");
            }
            "locks" => handle
                .replace_prune_lock(PruneLockInfo {
                    name: "wallet".into(),
                    height_first: 20,
                    height_last: 30,
                })
                .expect("lock"),
            _ => unreachable!("table case"),
        }
        tick(&handle, FlushMode::Periodic, 160).expect("changed flush");
        // Assert
        assert_eq!(facts.lock().expect("facts").scans, 2, "{changed}");
    }
}

#[test]
fn automatic_prune_changed_revision_during_measurement_cannot_authorize_delete() {
    // Arrange
    let (handle, facts) = fixture(PruneMode::Automatic { target_mib: 550 }, TARGET + 500);
    facts.lock().expect("facts").change_during_scan = true;
    // Act
    let execution = tick(&handle, FlushMode::Periodic, 100).expect("unstable measurement");
    // Assert
    assert!(!execution.wrote_coins);
    assert!(facts.lock().expect("facts").deletes.is_empty());
}

#[test]
fn automatic_prune_invalid_revision_never_authorizes_idle_reuse() {
    // Arrange
    let (handle, facts) = fixture(PruneMode::Automatic { target_mib: 550 }, TARGET);
    facts.lock().expect("facts").invalid_revision = true;
    tick(&handle, FlushMode::Periodic, 100).expect("first");
    // Act
    tick(&handle, FlushMode::Periodic, 160).expect("unreusable");
    // Assert
    assert_eq!(facts.lock().expect("facts").scans, 2);
}

#[test]
fn automatic_prune_accounting_failure_refuses_before_unlink() {
    // Arrange
    let (handle, facts) = fixture(PruneMode::Automatic { target_mib: 550 }, TARGET + 500);
    facts.lock().expect("facts").fail_accounting = true;
    // Act
    let result = tick(&handle, FlushMode::Periodic, 100);
    // Assert
    assert!(result.is_err());
    assert!(facts.lock().expect("facts").deletes.is_empty());
}

#[test]
fn automatic_prune_receipts_evict_payload_and_undo_after_later_metadata_error() {
    // Arrange
    let (handle, facts) = fixture(PruneMode::Automatic { target_mib: 550 }, TARGET + 50);
    let eligible = position(10);
    handle
        .mutate(|network| {
            let mut snapshot = network
                .chainstate()
                .export_chainstate_snapshot()
                .expect("snapshot");
            snapshot
                .undo_by_block
                .insert(eligible.block_hash, BlockUndo::default());
            network.chainstate_mut().install_chainstate_for_test(
                open_bitcoin_core::chainstate::Chainstate::from_snapshot(snapshot),
            );
            network.blocks_by_hash.insert(
                eligible.block_hash,
                Block {
                    header: eligible.header,
                    transactions: Vec::new(),
                },
            );
        })
        .expect("cache seed");
    facts.lock().expect("facts").fail_metadata = true;
    // Act
    let result = tick(&handle, FlushMode::Periodic, 100);
    // Assert
    assert!(result.is_err());
    assert_eq!(facts.lock().expect("facts").deletes, vec![10]);
    assert!(
        !handle
            .cached_block_present(eligible.block_hash)
            .expect("cache")
    );
    assert!(
        !handle
            .chainstate_snapshot()
            .expect("snapshot")
            .undo_by_block
            .contains_key(&eligible.block_hash)
    );
}

#[test]
fn automatic_prune_ordinary_periodic_forces_full_checkpoint() {
    // Arrange
    let (handle, facts) = fixture(PruneMode::Automatic { target_mib: 550 }, TARGET + 150);
    // Act
    let execution = handle
        .flush_coins(FlushMode::Periodic, FlushPolicyTime::new(100), u64::MAX)
        .expect("ordinary flush");
    // Assert
    assert!(execution.wrote_coins);
    assert_eq!(facts.lock().expect("facts").deletes, vec![10, 712]);
}

#[test]
fn automatic_prune_under_target_retains_requested_periodic_policy() {
    // Arrange
    let (handle, facts) = fixture(PruneMode::Automatic { target_mib: 550 }, TARGET);
    // Act
    let execution = handle
        .flush_coins(FlushMode::Periodic, FlushPolicyTime::new(100), u64::MAX)
        .expect("flush");
    // Assert
    assert!(!execution.wrote_coins);
    assert!(facts.lock().expect("facts").deletes.is_empty());
}

#[test]
fn automatic_prune_cheap_mode_gates_do_not_measure() {
    // Arrange / Act
    for mode in [
        PruneMode::Disabled,
        PruneMode::ManualOnly,
        PruneMode::Automatic {
            target_mib: u64::MAX,
        },
    ] {
        let (handle, facts) = fixture(mode, TARGET + 100);
        handle
            .flush_coins(FlushMode::Periodic, FlushPolicyTime::new(100), u64::MAX)
            .expect("gated flush");
        // Assert
        let facts = facts.lock().expect("facts");
        assert_eq!(facts.scans, 0);
        assert!(facts.deletes.is_empty());
    }
}

#[test]
fn automatic_prune_none_does_not_measure_or_delete() {
    // Arrange
    let (handle, facts) = fixture(PruneMode::Automatic { target_mib: 550 }, TARGET + 100);
    // Act
    handle
        .flush_coins(FlushMode::None, FlushPolicyTime::new(100), u64::MAX)
        .expect("none flush");
    // Assert
    let facts = facts.lock().expect("facts");
    assert_eq!(facts.scans, 0);
    assert!(facts.deletes.is_empty());
}

#[test]
fn automatic_prune_completed_lock_protects_manual_stale_slice_and_buffer() {
    // Arrange
    let (handle, facts) = fixture(PruneMode::Automatic { target_mib: 550 }, TARGET + 250);
    handle
        .replace_prune_lock(PruneLockInfo {
            name: "wallet".into(),
            height_first: 20,
            height_last: 702,
        })
        .expect("publish lock");
    // Act
    handle
        .flush_applying_prune_plan(
            FlushMode::Always,
            FlushPolicyTime::new(100),
            u64::MAX,
            &open_bitcoin_core::chainstate::PrunePlan {
                heights: vec![10, 712, 713],
            },
            &[],
        )
        .expect("manual flush");
    // Assert
    assert_eq!(facts.lock().expect("facts").deletes, vec![713]);
}

#[test]
fn automatic_prune_lock_replace_preserves_other_names_and_missing_clear_does_not_write() {
    // Arrange
    let (handle, facts) = fixture(PruneMode::ManualOnly, TARGET);
    for (name, first) in [("wallet", 20), ("other", 30), ("wallet", 40)] {
        handle
            .replace_prune_lock(PruneLockInfo {
                name: name.into(),
                height_first: first,
                height_last: first,
            })
            .expect("publish lock");
    }
    // Act
    let cleared = handle.clear_prune_lock("absent").expect("absent lock");
    let listed = handle.list_prune_locks().expect("list locks");
    // Assert
    assert!(!cleared);
    assert_eq!(facts.lock().expect("facts").lock_writes, 3);
    assert_eq!(
        listed
            .iter()
            .map(|lock| (lock.name.as_str(), lock.height_first))
            .collect::<Vec<_>>(),
        vec![("other", 30), ("wallet", 40)]
    );
}

#[test]
fn automatic_prune_lock_publication_waits_for_existing_owner() {
    use std::sync::mpsc;
    // Arrange
    let (handle, _) = fixture(PruneMode::ManualOnly, TARGET);
    let clone = handle.clone();
    let guard = handle.authority.lock().expect("owner");
    let (started_tx, started_rx) = mpsc::channel();
    let (published_tx, published_rx) = mpsc::channel();
    // Act
    std::thread::scope(|scope| {
        scope.spawn(move || {
            started_tx.send(()).expect("started");
            clone
                .replace_prune_lock(PruneLockInfo {
                    name: "wallet".into(),
                    height_first: 20,
                    height_last: 30,
                })
                .expect("publish");
            published_tx.send(()).expect("published");
        });
        started_rx.recv().expect("writer starts");
        let blocked = published_rx.try_recv().is_err();
        drop(guard);
        published_rx.recv().expect("writer completes");
        // Assert
        assert!(blocked);
    });
    assert_eq!(handle.list_prune_locks().expect("locks").len(), 1);
}

#[test]
fn automatic_prune_recovery_finishes_or_refuses_current_lock() {
    // Arrange: inject only the existing intent record, not usage or delete receipts.
    for locked in [false, true] {
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("clock")
            .as_nanos();
        let temp = std::env::temp_dir().join(format!(
            "automatic-prune-recovery-{}-{nanos}",
            std::process::id()
        ));
        let store = crate::FjallNodeStore::open(&temp).expect("store");
        let eligible = position(10);
        let truth = ChainstateSnapshot::new(
            vec![eligible.clone(), position(1_001)],
            HashMap::new(),
            HashMap::new(),
        );
        store
            .seed_coins_from_snapshot(&truth)
            .expect("coins and chain");
        store
            .save_block(
                &Block {
                    header: eligible.header.clone(),
                    transactions: Vec::new(),
                },
                crate::PersistMode::Sync,
            )
            .expect("block");
        store
            .save_undo(
                eligible.block_hash,
                &BlockUndo::default(),
                crate::PersistMode::Sync,
            )
            .expect("undo");
        let mut intent = 10_u32.to_le_bytes().to_vec();
        intent.extend_from_slice(eligible.block_hash.as_bytes());
        store
            .write_raw_for_test(StorageNamespace::BlockIndex, "prune_intent", intent)
            .expect("interrupted intent");
        if locked {
            store
                .sync_prune_locks(&[PruneLockInfo {
                    name: "new-current-lock".into(),
                    height_first: 20,
                    height_last: 20,
                }])
                .expect("current protection");
        }
        drop(store);

        // Act
        let store = crate::FjallNodeStore::open(&temp).expect("reopen");
        let opened =
            crate::DurableSyncRuntime::open(store.clone(), crate::SyncRuntimeConfig::default());

        // Assert
        if locked {
            let error = match opened {
                Err(error) => error,
                Ok(_) => panic!("current lock must refuse readiness"),
            };
            assert!(error.to_string().contains("prune lock"));
            assert!(store.has_block(eligible.block_hash).expect("refused body"));
            assert!(store.has_undo(eligible.block_hash).expect("refused undo"));
            assert!(!store.load_have_pruned().expect("unearned marker"));
        } else {
            let opened = opened.expect("recovery finishes before readiness");
            assert!(!store.has_block(eligible.block_hash).expect("finished body"));
            assert!(!store.has_undo(eligible.block_hash).expect("finished undo"));
            assert!(store.load_have_pruned().expect("earned marker"));
            drop(opened);
        }
        drop(store);
        std::fs::remove_dir_all(temp).expect("cleanup");
    }
}

#[test]
fn automatic_prune_prune_first_then_lock_cannot_resurrect_deleted_payload() {
    // Arrange
    let (handle, facts) = fixture(PruneMode::Automatic { target_mib: 550 }, TARGET + 50);
    tick(&handle, FlushMode::Periodic, 100).expect("prune owns first schedule");

    // Act
    handle
        .replace_prune_lock(PruneLockInfo {
            name: "wallet".into(),
            height_first: 20,
            height_last: 20,
        })
        .expect("later lock publication");
    tick(&handle, FlushMode::Always, 101).expect("locked retry");

    // Assert
    let facts = facts.lock().expect("facts");
    assert_eq!(facts.deletes, vec![10]);
    assert!(!facts.sizes.contains_key(&10));
    assert_eq!(facts.locks[0].name, "wallet");
}
