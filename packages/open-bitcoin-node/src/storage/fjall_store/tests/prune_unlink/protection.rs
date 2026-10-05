// Parity breadcrumbs:
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp
// - packages/bitcoin-knots/src/node/blockstorage.h
// - packages/bitcoin-knots/src/validation.h

//! Dense metadata with sparse payloads proves deletion gates, not consensus sync.

use super::*;
use crate::storage::filter_index::{self as codec, StoredFilterRecord};
use crate::storage::fjall_store::filters::FilterPublicationFault;
use crate::storage::fjall_store::resume_prune_intent;
use open_bitcoin_core::chainstate::{
    BasicFilterInputs, ChainPosition, FilterCheckpoint, HistoricalBlockUndo, IndexPrefix,
    VerifiedChainstateFence,
};
use std::sync::mpsc;

fn history_block(parent: BlockHash, nonce: u32) -> Block {
    let mut body = block(parent, nonce);
    body.transactions = vec![Transaction {
        version: 1,
        inputs: vec![TransactionInput {
            previous_output: OutPoint::null(),
            script_sig: ScriptBuf::from_bytes(vec![1, nonce as u8]).expect("script"),
            sequence: u32::MAX,
            witness: ScriptWitness::default(),
        }],
        outputs: vec![TransactionOutput {
            value: Amount::from_sats(5_000_000_000).expect("amount"),
            script_pubkey: ScriptBuf::from_bytes(vec![0x51]).expect("script"),
        }],
        lock_time: 0,
    }];
    body.header.merkle_root = open_bitcoin_core::consensus::block_merkle_root(&body.transactions)
        .expect("merkle")
        .0;
    body
}

struct History {
    path: PathBuf,
    store: FjallNodeStore,
    positions: Vec<ChainPosition>,
}

impl History {
    fn new(name: &str) -> Self {
        let (path, store) = open_store(name);
        let mut positions = Vec::new();
        let mut records: Vec<StoredFilterRecord> = Vec::new();
        let mut parent = BlockHash::default();
        for height in 0..=400 {
            let body = history_block(parent, height);
            let position =
                ChainPosition::new(body.header.clone(), height, u128::from(height) + 1, 1);
            let undo = BlockUndo::default();
            if height <= 16 {
                let maybe_history = (height != 0).then_some(HistoricalBlockUndo {
                    block_hash: position.block_hash,
                    undo: &undo,
                });
                let inputs = BasicFilterInputs::from_historical(&body, &position, maybe_history)
                    .expect("inputs");
                let maybe_previous = records.last().map(StoredFilterRecord::identity);
                records.push(
                    StoredFilterRecord::generate(&inputs, &position, maybe_previous.as_ref())
                        .expect("record"),
                );
            }
            if [0, 1, 20].contains(&height) {
                store.save_block(&body, PersistMode::Sync).expect("body");
                if height != 0 {
                    plant_undo(&store, position.block_hash);
                }
            }
            parent = position.block_hash;
            positions.push(position);
        }
        save_authority(&store, &positions);
        let fence = VerifiedChainstateFence::new(Some(parent), Some(&positions)).expect("fence");
        store.initialize_basic_filter_state(&fence).expect("owner");
        let checkpoint = FilterCheckpoint::new(IndexPrefix::Committed(records[16].identity()));
        let work = store
            .maybe_basic_filter_work(&fence)
            .expect("work")
            .expect("active");
        store
            .publish_basic_filter_checkpoint(
                &work,
                &fence,
                checkpoint,
                checkpoint.input_protection(),
                &records,
            )
            .expect("safe checkpoint");
        Self {
            path,
            store,
            positions,
        }
    }

    fn intent(&self, height: u32) -> PruneIntent {
        PruneIntent {
            height,
            block_hash: self.positions[height as usize].block_hash,
        }
    }

    fn close_and_check(self, intent: PruneIntent, maybe_expected_intent: Option<PruneIntent>) {
        drop(self.store);
        let store = FjallNodeStore::open(&self.path).expect("actual reopen");
        assert!(store.has_block(intent.block_hash).expect("durable body"));
        assert_eq!(
            store.has_undo(intent.block_hash).expect("durable undo"),
            intent.height != 0
        );
        assert_eq!(
            store.maybe_prune_intent().expect("durable intent"),
            maybe_expected_intent
        );
        assert!(!store.load_have_pruned().expect("durable pruned"));
        assert_eq!(
            store
                .load_operator_support_counts()
                .expect("support")
                .pruned_height_count,
            0
        );
        drop(store);
        remove_dir_if_exists(&self.path);
    }
}

fn save_authority(store: &FjallNodeStore, positions: &[ChainPosition]) {
    store
        .save_chain_meta(positions, PersistMode::Sync)
        .expect("metadata");
    store
        .coins_view()
        .write_raw_bytes(
            &encode_best_block_key(),
            encode_best_block_value(positions.last().expect("tip").block_hash),
        )
        .expect("coins B");
}

fn assert_refused_without_effects(history: &History, intent: PruneIntent, standalone: bool) {
    let usage = history
        .store
        .retained_payload_usage(&history.positions)
        .expect("measurement");
    let before_intent = history.store.maybe_prune_intent().expect("old intent");
    let before_rows = snapshot(&history.store);
    let result = if standalone {
        history.store.sync_prune_intent(intent)
    } else {
        history
            .store
            .commit_paired_delete(intent.height, intent.block_hash)
            .map(|_| ())
    };
    let error = result.expect_err("current protected authority must refuse");
    assert_eq!(
        error.recovery_action(),
        Some(StorageRecoveryAction::Repair),
        "{error}"
    );
    assert_eq!(
        history.store.payload_usage_revision().expect("revision"),
        usage.revision
    );
    assert_eq!(
        history.store.maybe_prune_intent().expect("intent"),
        before_intent
    );
    assert_eq!(snapshot(&history.store), before_rows);
    assert_eq!(
        history
            .store
            .retained_payload_usage(&history.positions)
            .expect("exact usage")
            .current_usage_bytes,
        usage.current_usage_bytes
    );
}

fn snapshot(store: &FjallNodeStore) -> Vec<(Vec<u8>, Vec<u8>)> {
    store
        .block_index
        .iter()
        .map(|row| {
            let (key, value) = row.into_inner().expect("row");
            (key.to_vec(), value.to_vec())
        })
        .collect()
}

fn raw_intent(store: &FjallNodeStore, intent: PruneIntent) {
    let mut bytes = intent.height.to_le_bytes().to_vec();
    bytes.extend_from_slice(intent.block_hash.as_bytes());
    store
        .write_raw_for_test(StorageNamespace::BlockIndex, PRUNE_INTENT_KEY, bytes)
        .expect("deliberately unsafe recovered intent");
}

#[test]
fn prune_owned_forged_height_refuses_direct_and_standalone_intent_in_both_modes() {
    for disabled in [false, true] {
        for standalone in [false, true] {
            // Arrange
            let history = History::new("forged-height");
            if disabled {
                let owner = history
                    .store
                    .maybe_basic_filter_lifecycle_for_test()
                    .expect("owner")
                    .expect("active")
                    .disable()
                    .expect("generation");
                history
                    .store
                    .write_raw_for_test(
                        StorageNamespace::BlockIndex,
                        codec::ownership::OWNER_KEY,
                        codec::ownership::encode_owner(owner).to_vec(),
                    )
                    .expect("Disabled retained recovery fixture");
            }
            let mut forged = history.intent(20);
            forged.height = 1;

            // Act / Assert
            assert_refused_without_effects(&history, forged, standalone);
            history.close_and_check(
                PruneIntent {
                    height: 20,
                    ..forged
                },
                None,
            );
        }
    }
}

#[test]
fn prune_owned_unknown_hash_refuses_even_below_released_cursor() {
    for standalone in [false, true] {
        // Arrange
        let history = History::new("unknown-ancestry");
        let intent = PruneIntent {
            height: 1,
            block_hash: BlockHash::from_byte_array([0xab; 32]),
        };

        // Act / Assert
        assert_refused_without_effects(&history, intent, standalone);
        let retained = history.intent(1);
        history.close_and_check(retained, None);
    }
}

#[test]
fn prune_owned_live_replacement_branch_cannot_use_old_released_cursor() {
    for standalone in [false, true] {
        // Arrange
        let mut history = History::new("old-branch");
        let mut replacement = history.positions[..1].to_vec();
        for height in 1..=400 {
            let body = history_block(
                replacement.last().expect("parent").block_hash,
                height + 1000,
            );
            let position =
                ChainPosition::new(body.header.clone(), height, u128::from(height) + 1, 1);
            if height == 1 {
                history
                    .store
                    .save_block(&body, PersistMode::Sync)
                    .expect("replacement body");
                plant_undo(&history.store, position.block_hash);
            }
            replacement.push(position);
        }
        save_authority(&history.store, &replacement);
        history.positions = replacement;
        let intent = history.intent(1);

        // Act / Assert
        assert_refused_without_effects(&history, intent, standalone);
        history.close_and_check(intent, None);
    }
}

#[test]
fn prune_owned_ahead_saved_fence_refuses_until_genuine_publication() {
    for standalone in [false, true] {
        // Arrange
        let history = History::new("ahead-fence");
        save_authority(&history.store, &history.positions[..=10]);
        let intent = history.intent(1);

        // Act / Assert
        assert_refused_without_effects(&history, intent, standalone);
        history.close_and_check(intent, None);
    }
}

#[test]
fn prune_owned_current_coins_must_be_recovered_and_match_metadata() {
    use crate::storage::coins_codec::{encode_head_blocks_key, encode_head_blocks_value};
    for defect in ["heads", "missing-B", "wrong-B", "metadata"] {
        // Arrange
        let history = History::new("current-fence");
        let view = history.store.coins_view();
        match defect {
            "heads" => view
                .write_raw_bytes(
                    &encode_head_blocks_key(),
                    encode_head_blocks_value(&[
                        history.positions[400].block_hash,
                        history.positions[399].block_hash,
                    ])
                    .expect("heads"),
                )
                .expect("unrecovered heads"),
            "missing-B" => history
                .store
                .coins_keyspace()
                .remove(encode_best_block_key())
                .expect("missing B"),
            "wrong-B" => view
                .write_raw_bytes(
                    &encode_best_block_key(),
                    encode_best_block_value(history.positions[399].block_hash),
                )
                .expect("older B"),
            _ => history
                .store
                .write_raw_for_test(StorageNamespace::Chainstate, "chain_meta", vec![1])
                .expect("corrupt metadata"),
        }
        drop(view);
        let intent = history.intent(1);

        // Act / Assert
        assert_refused_without_effects(&history, intent, false);
        assert_refused_without_effects(&history, intent, true);
        if defect == "heads" {
            // Normal open rejects this deliberately malformed H+B fixture itself.
            let before_rows = snapshot(&history.store);
            drop(history.store);
            let Err(error) = FjallNodeStore::open(&history.path) else {
                panic!("unrecovered coins must refuse normal open");
            };
            assert_eq!(error.recovery_action(), Some(StorageRecoveryAction::Repair));
            let db = fjall::Database::builder(&history.path)
                .open()
                .expect("actual raw reopen");
            let index = db
                .keyspace("block_index", fjall::KeyspaceCreateOptions::default)
                .expect("index");
            let rows: Vec<_> = index
                .iter()
                .map(|row| {
                    let (key, value) = row.into_inner().expect("row");
                    (key.to_vec(), value.to_vec())
                })
                .collect();
            assert_eq!(rows, before_rows);
            let chainstate = db
                .keyspace("chainstate", fjall::KeyspaceCreateOptions::default)
                .expect("undo namespace");
            assert!(
                chainstate
                    .contains_key(crate::storage::fjall_store::coins::undo_key(
                        intent.block_hash
                    ))
                    .expect("durable undo survives")
            );
            drop(chainstate);
            drop(index);
            drop(db);
            remove_dir_if_exists(&history.path);
            continue;
        }
        history.close_and_check(intent, None);
    }
}

#[test]
fn prune_owned_same_branch_extension_preserves_safe_prefix_delete() {
    // Arrange
    let mut history = History::new("safe-extension");
    let body = history_block(history.positions.last().expect("tip").block_hash, 401);
    history
        .positions
        .push(ChainPosition::new(body.header, 401, 402, 1));
    save_authority(&history.store, &history.positions);
    let intent = history.intent(1);

    // Act
    history
        .store
        .sync_prune_intent(intent)
        .expect("safe standalone intent");
    let result = history
        .store
        .commit_paired_delete(intent.height, intent.block_hash)
        .expect("safe pair");

    // Assert
    assert_eq!(result, PairedDeleteOutcome::DeletedLiveMate);
    assert!(!history.store.has_block(intent.block_hash).expect("body"));
    assert!(!history.store.has_undo(intent.block_hash).expect("undo"));
    assert!(history.store.load_have_pruned().expect("pruned"));
    assert_eq!(history.store.maybe_prune_intent().expect("intent"), None);
    drop(history.store);
    let store = FjallNodeStore::open(&history.path).expect("actual reopen");
    assert!(
        !store
            .has_block(intent.block_hash)
            .expect("durable deletion")
    );
    assert!(store.load_have_pruned().expect("durable receipt"));
    drop(store);
    remove_dir_if_exists(&history.path);
}

#[test]
fn prune_owned_corruption_refuses_even_absent_mates_before_accounting() {
    for defect in ["owner", "missing-lock", "weak-lock", "state"] {
        // Arrange
        let history = History::new("corrupt-owner");
        match defect {
            "owner" => history
                .store
                .write_raw_for_test(
                    StorageNamespace::BlockIndex,
                    codec::ownership::OWNER_KEY,
                    vec![1],
                )
                .expect("corrupt owner"),
            "state" => history
                .store
                .write_raw_for_test(StorageNamespace::BlockIndex, codec::STATE_KEY, vec![1])
                .expect("corrupt state"),
            "missing-lock" => history
                .store
                .block_index
                .remove("prune_locks")
                .expect("missing protection"),
            _ => {
                let mut locks = history.store.load_prune_locks().expect("locks");
                locks[0].height_first = 18;
                history
                    .store
                    .write_raw_for_test(
                        StorageNamespace::BlockIndex,
                        "prune_locks",
                        super::super::super::prune::encode_prune_locks(&locks)
                            .expect("lock encoding"),
                    )
                    .expect("weak protection");
            }
        }
        let absent = history.intent(2);

        // Act / Assert
        assert_refused_without_effects(&history, absent, false);
        assert_refused_without_effects(&history, absent, true);
        let retained = history.intent(1);
        history.close_and_check(retained, None);
    }
}

#[test]
fn prune_owned_poisoned_publication_refuses_before_payload_revision() {
    // Arrange
    let history = History::new("poisoned-delete");
    history
        .store
        .set_basic_filter_fault(FilterPublicationFault::BeforeDisable);
    history
        .store
        .disable_basic_filter_index()
        .expect_err("injected failure");
    let usage = history
        .store
        .retained_payload_usage(&history.positions)
        .expect("usage");
    let intent = history.intent(1);

    // Act
    assert!(
        history
            .store
            .commit_paired_delete(intent.height, intent.block_hash)
            .is_err()
    );
    assert!(history.store.sync_prune_intent(intent).is_err());

    // Assert
    assert_eq!(
        history.store.payload_usage_revision().expect("revision"),
        usage.revision
    );
    history.close_and_check(intent, None);
}

#[test]
fn prune_owned_resume_reloads_stronger_protection_before_absent_intent_clear() {
    // Arrange
    let history = History::new("resume-stronger");
    let intent = history.intent(2);
    raw_intent(&history.store, intent);
    let state = history
        .store
        .maybe_basic_filter_state()
        .expect("state")
        .expect("saved");
    history
        .store
        .write_raw_for_test(
            StorageNamespace::BlockIndex,
            codec::STATE_KEY,
            codec::encode_state(codec::StoredFilterState {
                protection: open_bitcoin_core::chainstate::IndexInputProtection::FromHeight(0),
                ..state
            }),
        )
        .expect("conservative fixture");
    let mut locks = history.store.load_prune_locks().expect("locks");
    locks[0].height_first = 0;
    history
        .store
        .write_raw_for_test(
            StorageNamespace::BlockIndex,
            "prune_locks",
            super::super::super::prune::encode_prune_locks(&locks).expect("locks"),
        )
        .expect("stronger protection");

    // Act
    let result = resume_prune_intent(&history.store, &[]);

    // Assert
    assert_eq!(
        result.expect_err("protected absent pair").recovery_action(),
        Some(StorageRecoveryAction::Repair)
    );
    assert_eq!(
        history.store.maybe_prune_intent().expect("retained intent"),
        Some(intent)
    );
    assert!(!history.store.load_have_pruned().expect("pruned"));
    let retained = history.intent(1);
    history.close_and_check(retained, Some(intent));
}

#[test]
fn prune_owned_channel_ordered_clone_cannot_use_stale_release_after_authority_change() {
    // Arrange
    let history = History::new("clone-stale-ancestry");
    let intent = history.intent(1);
    let clone = history.store.clone();
    let (ready_tx, ready_rx) = mpsc::channel();
    let (go_tx, go_rx) = mpsc::channel();
    let worker = std::thread::spawn(move || {
        ready_tx.send(()).expect("prepared candidate");
        go_rx.recv().expect("new authority published");
        clone.commit_paired_delete(intent.height, intent.block_hash)
    });
    ready_rx.recv().expect("prepared");
    save_authority(&history.store, &history.positions[..=10]);
    let usage = history
        .store
        .retained_payload_usage(&history.positions)
        .expect("usage");

    // Act
    go_tx.send(()).expect("apply stale candidate");
    let result = worker.join().expect("clone thread");

    // Assert
    assert_eq!(
        result.expect_err("fresh proof required").recovery_action(),
        Some(StorageRecoveryAction::Repair)
    );
    assert_eq!(
        history.store.payload_usage_revision().expect("revision"),
        usage.revision
    );
    history.close_and_check(intent, None);
}
