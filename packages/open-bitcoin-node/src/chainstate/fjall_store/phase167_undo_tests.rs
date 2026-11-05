// Parity breadcrumbs:
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/coins.cpp
// - packages/bitcoin-knots/src/undo.h

use super::canonical_genesis::historical_fixture::{Case, DATA, load};
use super::*;
use crate::chainstate::{FlushLifecycle, FlushMode};
use crate::storage::snapshot_codec::{
    decode_block_undo, decode_chainstate_snapshot, encode_block_undo, encode_chainstate_snapshot,
};
use open_bitcoin_core::chainstate::{CoinsCache, FlushPolicyTime};
use std::collections::HashMap;

fn initial(case: &Case) -> ChainstateSnapshot {
    // A finite suffix cannot prove confirmation counts for the entire ancestry.
    ChainstateSnapshot::new(case.positions.clone(), case.coins.clone(), HashMap::new())
}

fn path() -> std::path::PathBuf {
    let mut nonce = [0u8; 16];
    getrandom::fill(&mut nonce).expect("nonce");
    std::fs::canonicalize(std::env::temp_dir())
        .expect("canonical physical parent")
        .join(format!(
            "phase167-durable-undo-{:x}",
            u128::from_le_bytes(nonce)
        ))
}

fn manager(
    store: FjallNodeStore,
    snapshot: ChainstateSnapshot,
) -> ManagedChainstate<FjallChainstateStore, FjallCoinsView> {
    let chain = Chainstate::from_coins_cache_for_mechanical_fixture(
        CoinsCache::from_parent(store.coins_view()),
        snapshot.active_chain,
        snapshot.undo_by_block,
        snapshot.maybe_confirmed_txid_counts,
    );
    ManagedChainstate::from_chainstate(
        FjallChainstateStore::from_store(store),
        chain,
        FlushLifecycle::ready(FlushPolicyTime::new(0), FlushPolicyTime::new(0), 0, false),
    )
}

fn connect(managed: &mut ManagedChainstate<FjallChainstateStore, FjallCoinsView>, case: &Case) {
    let stage = managed
        .chainstate
        .stage_seeded_historical_block(&case.block, &case.history, case.now)
        .expect("exact seeded repeat body");
    assert!(!stage.has_earned_work());
    managed
        .chainstate
        .absorb_staged_connect(stage)
        .expect("actual absorption");
}

fn checkpoint(managed: &mut ManagedChainstate<FjallChainstateStore, FjallCoinsView>, case: &Case) {
    managed
        .store
        .inner()
        .save_block(&case.block, PersistMode::Sync)
        .expect("body");
    managed
        .flush_with_mode(FlushMode::Always, FlushPolicyTime::new(10), u64::MAX)
        .expect("actual managed checkpoint");
}

fn connected(case: &Case) -> ChainstateSnapshot {
    let mut chain = Chainstate::from_snapshot_for_mechanical_fixture(initial(case));
    let stage = chain
        .stage_seeded_historical_block(&case.block, &case.history, case.now)
        .expect("repeat");
    chain.absorb_staged_connect(stage).expect("absorb");
    chain.snapshot()
}

#[test]
fn phase167_undo_codec_both_complete_real_pairs_roundtrip_per_block_and_snapshot() {
    // Arrange
    let (cases, _) = load(DATA).expect("complete source-bound projection");
    for case in cases {
        assert_eq!(
            case.original.transactions.len(),
            if case.history.height() == 91842 { 2 } else { 1 }
        );
        assert_eq!(
            case.block.transactions.len(),
            if case.history.height() == 91880 { 2 } else { 1 }
        );
        let snapshot = connected(&case);
        let undo = &snapshot.undo_by_block[&case.history.candidate_hash()];
        // Act
        let bytes = encode_block_undo(undo).expect("encode complete undo");
        let decoded = decode_block_undo(&bytes).expect("decode complete undo");
        let roundtrip = decode_chainstate_snapshot(
            &encode_chainstate_snapshot(&snapshot).expect("encode snapshot"),
        )
        .expect("decode complete snapshot");
        // Assert
        assert_eq!(decoded, *undo);
        assert_eq!(roundtrip, snapshot);
        assert_eq!(
            decoded.overwrites[0].overwritten[0].previous_coin,
            case.old.1
        );
        decoded
            .validate_overwrite_binding(&case.block, snapshot.tip().expect("tip"))
            .expect("structural binding");
        let imported = Chainstate::from_snapshot(roundtrip);
        assert!(!imported.has_earned_work());
        assert!(imported.maybe_canonical_chain().is_none());
    }
}

#[test]
fn phase167_undo_codec_captured_empty_scan_survives_both_carriers() {
    // Arrange: make the original fully spent, retaining each complete repeat body.
    let (cases, _) = load(DATA).expect("envelopes");
    for case in cases {
        let mut prior = initial(&case);
        assert_eq!(prior.utxos.remove(&case.old.0), Some(case.old.1.clone()));
        let mut chain = Chainstate::from_snapshot_for_mechanical_fixture(prior.clone());
        let stage = chain
            .stage_seeded_historical_block(&case.block, &case.history, case.now)
            .expect("fully spent duplicate");
        chain.absorb_staged_connect(stage).expect("captured scan");
        let snapshot = chain.snapshot();
        let undo = &snapshot.undo_by_block[&case.history.candidate_hash()];
        // Act
        let decoded = decode_block_undo(&encode_block_undo(undo).expect("encode")).expect("decode");
        let recovered =
            decode_chainstate_snapshot(&encode_chainstate_snapshot(&snapshot).expect("snapshot"))
                .expect("decode snapshot");
        // Assert: exactly one complete batch with zero entries differs from unknown.
        assert_eq!(decoded, *undo);
        assert_eq!(decoded.overwrites.len(), 1);
        assert!(decoded.overwrites[0].overwritten.is_empty());
        assert_eq!(recovered, snapshot);
        let mut recovered = Chainstate::from_snapshot_for_mechanical_fixture(recovered);
        recovered
            .disconnect_tip(&case.block)
            .expect("captured no-overwrite");
        assert_eq!(recovered.snapshot(), prior);
    }
}

fn undo_key(hash: BlockHash) -> String {
    let hex: String = hash
        .as_bytes()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect();
    format!("undo:{hex}")
}

#[test]
fn phase167_undo_codec_complete_records_refuse_mutation_in_both_directions() {
    // Arrange
    let (cases, _) = load(DATA).expect("envelopes");
    for case in cases {
        let snapshot = connected(&case);
        let undo = &snapshot.undo_by_block[&case.history.candidate_hash()];
        let bytes = encode_block_undo(undo).expect("complete captured undo");
        for mutation in 0..18 {
            let mut json: serde_json::Value = serde_json::from_slice(&bytes).expect("JSON");
            let payload = &mut json["payload"];
            let entry = &mut payload["overwrites"][0]["overwritten"][0];
            match mutation {
                0 => entry["transaction_index"] = serde_json::json!(1),
                1 => entry["outpoint"]["vout"] = serde_json::json!(4294967296u64),
                2 => entry["outpoint"]["txid"] = serde_json::json!([1, 2]),
                3 => entry["previous_coin"]["created_height"] = serde_json::json!(-1),
                4 => entry["previous_coin"]["created_median_time_past"] = serde_json::json!(-1),
                5 => entry["previous_coin"]["is_coinbase"] = serde_json::json!(false),
                6 => entry["previous_coin"]["output"]["value_sats"] = serde_json::json!(-1),
                7 => {
                    entry["previous_coin"]["output"]["script_pubkey"] =
                        serde_json::json!(vec![0; 10001])
                }
                8 => entry["new_coin"]["created_height"] = serde_json::json!(0),
                9 => entry["new_coin"]["created_median_time_past"] = serde_json::json!(0),
                10 => entry["new_coin"]["is_coinbase"] = serde_json::json!(false),
                11 => {
                    entry
                        .as_object_mut()
                        .expect("entry")
                        .remove("previous_coin");
                }
                12 => {
                    let duplicate = entry.clone();
                    payload["overwrites"][0]["overwritten"]
                        .as_array_mut()
                        .expect("entries")
                        .push(duplicate);
                }
                13 => {
                    let duplicate = payload["overwrites"][0].clone();
                    payload["overwrites"]
                        .as_array_mut()
                        .expect("batches")
                        .push(duplicate);
                }
                14 => payload["overwrites"][0]["integrity"][0] = serde_json::json!(256),
                15 => payload["overwrites"][0]["height"] = serde_json::json!(4294967296u64),
                16 => {
                    payload["overwrites"][0]["creating_median_time_past"] =
                        serde_json::json!(9223372036854775808u64)
                }
                17 => {
                    payload["overwrites"][0]
                        .as_object_mut()
                        .expect("batch")
                        .remove("integrity");
                }
                _ => unreachable!("bounded mutation"),
            }
            // Act / Assert: malformed data never becomes an empty scan or repaired digest.
            assert!(
                matches!(
                    decode_block_undo(&serde_json::to_vec(&json).expect("JSON")),
                    Err(StorageError::Corruption { .. })
                ),
                "mutation {mutation}"
            );
        }
        let mut altered = undo.clone();
        altered.overwrites[0].overwritten[0]
            .previous_coin
            .created_median_time_past += 1;
        assert!(
            encode_block_undo(&altered).is_err(),
            "encoder must not re-seal changed facts"
        );
        let mut altered_snapshot = snapshot.clone();
        altered_snapshot
            .undo_by_block
            .insert(case.history.candidate_hash(), altered);
        assert!(encode_chainstate_snapshot(&altered_snapshot).is_err());
    }
}

#[test]
fn phase167_undo_codec_snapshot_rejects_duplicate_and_mismatched_context_with_valid_digest() {
    // Arrange
    let (cases, _) = load(DATA).expect("envelopes");
    for case in cases {
        let snapshot = connected(&case);
        let bytes = encode_chainstate_snapshot(&snapshot).expect("snapshot");
        for mutation in 0..5 {
            let mut json: serde_json::Value = serde_json::from_slice(&bytes).expect("JSON");
            let payload = &mut json["payload"];
            match mutation {
                0 => {
                    let duplicate = payload["undo_by_block"][0].clone();
                    payload["undo_by_block"]
                        .as_array_mut()
                        .expect("undo")
                        .push(duplicate);
                }
                1 => payload["undo_by_block"][0]["block_hash"] = serde_json::json!(vec![0; 32]),
                2 => {
                    let len = payload["active_chain"].as_array().expect("positions").len();
                    payload["active_chain"][len - 1]["height"] = serde_json::json!(42);
                }
                3 => {
                    let len = payload["active_chain"].as_array().expect("positions").len();
                    payload["active_chain"][len - 2]["median_time_past"] = serde_json::json!(0);
                }
                4 => {
                    let duplicate = payload["utxos"][0].clone();
                    payload["utxos"]
                        .as_array_mut()
                        .expect("utxos")
                        .push(duplicate);
                }
                _ => unreachable!("bounded mutation"),
            }
            // Act / Assert: these changes leave the actual captured batch/digest intact.
            assert!(
                matches!(
                    decode_chainstate_snapshot(&serde_json::to_vec(&json).expect("JSON")),
                    Err(StorageError::Corruption { .. })
                ),
                "mutation {mutation}"
            );
        }
    }
}

#[test]
fn phase167_durable_undo_corrupt_missing_batches_and_record_refuse_without_endpoint_change() {
    // Arrange
    let (cases, _) = load(DATA).expect("source-bound full envelopes");
    for case in &cases {
        for mutation in 0..7 {
            let source = path();
            let expected;
            {
                let store = FjallNodeStore::open(&source).expect("fixture");
                let prior = initial(case);
                store
                    .seed_coins_from_snapshot(&prior)
                    .expect("explicit seed");
                let mut managed = manager(store, prior);
                connect(&mut managed, case);
                expected = managed.chainstate.admission_snapshot().expect("snapshot");
                checkpoint(&mut managed, case);
                let store = managed.store.inner();
                if mutation == 5 {
                    store
                        .commit_paired_delete(case.history.height(), case.history.candidate_hash())
                        .expect("remove the actual durable body/undo pair");
                } else {
                    let undo = &expected.undo_by_block[&case.history.candidate_hash()];
                    let mut json: serde_json::Value =
                        serde_json::from_slice(&encode_block_undo(undo).expect("encode"))
                            .expect("JSON");
                    let payload = &mut json["payload"];
                    match mutation {
                        0 => {
                            payload
                                .as_object_mut()
                                .expect("payload")
                                .remove("overwrites");
                        }
                        1 => payload["overwrites"] = serde_json::json!([]),
                        2 => payload["overwrites"][0]["overwritten"] = serde_json::json!([]),
                        3 => payload["overwrites"][0]["integrity"][0] = serde_json::json!(999),
                        4 => {
                            payload["overwrites"][0]["overwritten"][0]["outpoint"]["vout"] =
                                serde_json::json!(1)
                        }
                        6 => {
                            let other = cases
                                .iter()
                                .find(|other| other.history.height() != case.history.height())
                                .expect("other genuine pair");
                            let other_snapshot = connected(other);
                            let other_undo =
                                &other_snapshot.undo_by_block[&other.history.candidate_hash()];
                            let other_json: serde_json::Value = serde_json::from_slice(
                                &encode_block_undo(other_undo).expect("other complete undo"),
                            )
                            .expect("other JSON");
                            payload["overwrites"] = other_json["payload"]["overwrites"].clone();
                        }
                        _ => unreachable!("bounded mutation"),
                    }
                    store
                        .write_raw_for_test(
                            StorageNamespace::Chainstate,
                            &undo_key(case.history.candidate_hash()),
                            serde_json::to_vec(&json).expect("damaged bytes"),
                        )
                        .expect("physical corruption");
                }
            }
            // Act: every manager/store/view/lease was released before reopening.
            let store = FjallNodeStore::open(&source).expect("reopen tainted fixture");
            let meta_before = store.load_chain_meta_for_open().expect("metadata");
            let coins_before = store
                .coins_view()
                .complete_snapshot()
                .expect("complete coins");
            let basic_before = store.maybe_basic_filter_state().expect("BASIC state");
            let checkpoint_before = store
                .maybe_basic_filter_checkpoint()
                .expect("BASIC checkpoint");
            let hydrated = store.hydrate_chainstate_for_open();
            // Assert: unknown explicit data can decode, but cannot authorize disconnect.
            if matches!(mutation, 1 | 5 | 6) {
                let mut managed = manager(
                    store.clone(),
                    hydrated.expect("unknown carrier").expect("fixture"),
                );
                let before = managed.chainstate.admission_snapshot().expect("before");
                assert!(managed.chainstate.disconnect_tip(&case.block).is_err());
                assert_eq!(
                    managed.chainstate.admission_snapshot().expect("after"),
                    before
                );
                assert!(managed.chainstate.validate_live_origin().is_err());
                drop(managed);
            } else {
                assert!(
                    matches!(hydrated, Err(StorageError::Corruption { .. })),
                    "mutation {mutation}"
                );
            }
            assert_eq!(
                store
                    .coins_view()
                    .complete_snapshot()
                    .expect("coins unchanged"),
                coins_before
            );
            assert_eq!(coins_before.coins, expected.utxos);
            assert_eq!(
                store
                    .load_chain_meta_for_open()
                    .expect("metadata unchanged"),
                meta_before
            );
            assert_eq!(meta_before.0, expected.active_chain);
            assert_eq!(
                store.maybe_basic_filter_state().expect("BASIC unchanged"),
                basic_before
            );
            assert_eq!(
                store
                    .maybe_basic_filter_checkpoint()
                    .expect("checkpoint unchanged"),
                checkpoint_before
            );
            for chain in [
                open_bitcoin_core::consensus::CanonicalChain::Mainnet,
                open_bitcoin_core::consensus::CanonicalChain::Testnet3,
                open_bitcoin_core::consensus::CanonicalChain::Signet,
            ] {
                assert!(FjallNodeStore::open_ordinary(&source, chain).is_err());
            }
            drop(store);
            std::fs::remove_dir_all(source).expect("cleanup");
        }
    }
}

#[test]
fn phase167_durable_undo_both_real_repeats_checkpoint_drop_reopen_restore_reconnect() {
    // Arrange: one actual store owner at a time; every view/lease dies with manager.
    let (cases, _) = load(DATA).expect("complete envelopes");
    for case in cases {
        let source = path();
        let prior = initial(&case);
        let expected;
        {
            let store = FjallNodeStore::open(&source).expect("seeded fixture store");
            store
                .seed_coins_from_snapshot(&prior)
                .expect("explicit seed");
            let mut managed = manager(store, prior.clone());
            connect(&mut managed, &case);
            expected = managed
                .chainstate
                .admission_snapshot()
                .expect("complete connected state");
            checkpoint(&mut managed, &case);
            assert_eq!(
                managed.chainstate.coins_best_block().expect("coins B"),
                Some(case.history.candidate_hash())
            );
        }
        // Act: a new physical open must reacquire the released application lease.
        let store = FjallNodeStore::open(&source).expect("physical all-handle reopen");
        let restored = store
            .hydrate_chainstate_for_open()
            .expect("read/decode")
            .expect("stored fixture");
        assert_eq!(restored, expected);
        assert_eq!(
            store
                .load_block(case.history.candidate_hash())
                .expect("body"),
            Some(case.block.clone())
        );
        let mut managed = manager(store, restored);
        managed
            .chainstate
            .disconnect_tip(&case.block)
            .expect("restore full old Coin");
        // Assert: includes exact output/script,value,height,coinbase,MTP and full work endpoints.
        assert_eq!(
            managed.chainstate.admission_snapshot().expect("snapshot"),
            prior
        );
        assert_eq!(
            managed
                .chainstate
                .coins()
                .get_coin(&case.old.0)
                .expect("old coin"),
            Some(case.old.1.clone())
        );
        connect(&mut managed, &case);
        assert_eq!(
            managed
                .chainstate
                .admission_snapshot()
                .expect("reconnected"),
            expected
        );
        assert!(!managed.chainstate.has_earned_work());
        assert_eq!(managed.chainstate.maybe_canonical_chain(), None);
        assert!(managed.maybe_validated_lineage.is_none());
        drop(managed);
        std::fs::remove_dir_all(source).expect("cleanup released store");
    }
}
