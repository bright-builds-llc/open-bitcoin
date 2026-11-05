// Parity breadcrumbs:
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/coins.cpp

use super::super::phase167_support::physical_manifest;
use super::source::load_cases;
use crate::{
    FjallChainstateStore, FjallNodeStore, ManagedChainstate, chainstate::FlushLifecycle,
    storage::validation_history::ValidationProvenance,
};
use open_bitcoin_core::{
    chainstate::{Chainstate, Coin, CoinsCache, CoinsView, FlushMode, FlushPolicyTime},
    consensus::{CanonicalChain, block_hash, transaction_txid},
    primitives::OutPoint,
};

#[test]
fn phase167_historical_bip30_both_repeats_restore_across_physical_reopen() {
    // Arrange: one source pass binds each exact repeat to its earlier complete Coin.
    let cases = load_cases(&[
        "bip30-repeat-91842",
        "bip30-original-91812",
        "bip30-repeat-91880",
        "bip30-original-91722",
    ]);
    assert_eq!(cases.len(), 4);
    for (repeat_height, original_height) in [(91842, 91812), (91880, 91722)] {
        let repeat = cases
            .iter()
            .find(|case| case.history.height() == repeat_height)
            .expect("repeat");
        let original = cases
            .iter()
            .find(|case| case.history.height() == original_height)
            .expect("original");
        let txid = transaction_txid(&original.block.transactions[0]).expect("old txid");
        assert_eq!(
            txid,
            transaction_txid(&repeat.block.transactions[0]).expect("same txid")
        );
        let outpoint = OutPoint { txid, vout: 0 };
        let old = Coin {
            output: original.block.transactions[0].outputs[0].clone(),
            is_coinbase: true,
            created_height: original_height,
            created_median_time_past: original
                .snapshot
                .tip()
                .expect("old parent")
                .median_time_past,
        };
        let mut prior = repeat.snapshot.clone();
        assert!(prior.utxos.insert(outpoint.clone(), old.clone()).is_none());
        // The selected live old Coin gives a mechanical count inference of one.
        // No canonical whole-chain count is imported into this tainted owner.
        assert_eq!(prior.utxos.keys().filter(|key| key.txid == txid).count(), 1);
        prior.maybe_confirmed_txid_counts = None;
        let hash = block_hash(&repeat.block.header);
        let mut nonce = [0; 16];
        getrandom::fill(&mut nonce).expect("test nonce");
        let path = std::env::temp_dir()
            .canonicalize()
            .expect("temp root")
            .join(format!(
                "phase167-bip30-restore-{:x}",
                u128::from_le_bytes(nonce)
            ));
        let connected = {
            let store = FjallNodeStore::open(&path).expect("fresh mechanical store");
            store
                .seed_coins_from_snapshot(&prior)
                .expect("complete mechanical prior seed");
            assert_eq!(
                store
                    .hydrate_chainstate_for_open()
                    .expect("prior hydration"),
                Some(prior.clone())
            );
            let chain = Chainstate::from_coins_cache_for_mechanical_fixture(
                CoinsCache::from_parent(store.coins_view()),
                prior.active_chain.clone(),
                prior.undo_by_block.clone(),
                prior.maybe_confirmed_txid_counts.clone(),
            );
            let mut manager = ManagedChainstate::from_chainstate(
                FjallChainstateStore::from_store(store),
                chain,
                FlushLifecycle::ready(FlushPolicyTime::new(0), FlushPolicyTime::new(0), 0, false),
            );
            manager
                .store()
                .inner()
                .save_block(&repeat.block, crate::PersistMode::Sync)
                .expect("exact body retained");
            manager
                .connect_seeded_historical_fixture(&repeat.block, &repeat.history, repeat.now)
                .expect("exact managed connect");
            let connected = manager
                .chainstate()
                .admission_snapshot()
                .expect("connected admission");
            assert_eq!(
                connected.undo_by_block.get(&hash).expect("undo").overwrites[0].overwritten[0]
                    .previous_coin,
                old
            );
            if repeat_height == 91880 {
                let external = repeat
                    .snapshot
                    .utxos
                    .keys()
                    .next()
                    .expect("source external outpoint");
                assert!(prior.utxos.contains_key(external));
                assert!(!connected.utxos.contains_key(external));
            }
            connected
        };
        // Act: release every handle, mechanically reopen and disconnect through manager.
        assert!(
            FjallNodeStore::open_ordinary(&path, CanonicalChain::Mainnet).is_err(),
            "seeded mechanical history cannot become ordinary product state"
        );
        let before_product_probe = physical_manifest(&path);
        assert!(
            FjallNodeStore::reopen_canonical_history_existing(&path, CanonicalChain::Regtest)
                .is_err(),
            "seeded Mainnet history cannot pass the real canonical Regtest history proof"
        );
        assert_eq!(physical_manifest(&path), before_product_probe);
        {
            let store = FjallNodeStore::open(&path).expect("physical connected reopen");
            let hydrated = store
                .hydrate_chainstate_for_open()
                .expect("connected hydration")
                .expect("connected rows");
            assert!(
                hydrated == connected,
                "connected hydration mismatch: positions={}, coins={} {:?} vs {:?}, undo={}, counts={:?} vs {:?}",
                hydrated.active_chain == connected.active_chain,
                hydrated.utxos == connected.utxos,
                hydrated.utxos,
                connected.utxos,
                hydrated.undo_by_block == connected.undo_by_block,
                hydrated.maybe_confirmed_txid_counts,
                connected.maybe_confirmed_txid_counts
            );
            assert_eq!(
                store.load_block(hash).expect("body read"),
                Some(repeat.block.clone())
            );
            assert_eq!(
                store.coins_view().best_block().expect("connected best"),
                Some(hash)
            );
            assert!(
                store
                    .maybe_basic_filter_state()
                    .expect("BASIC state")
                    .is_none()
            );
            assert_ne!(
                store.validation_provenance(hash).expect("provenance"),
                ValidationProvenance::ScriptsValid
            );
            assert_eq!(
                store.load_undo(hash).expect("undo read"),
                connected.undo_by_block.get(&hash).cloned()
            );
            let chain = Chainstate::from_coins_cache_for_mechanical_fixture(
                CoinsCache::from_parent(store.coins_view()),
                hydrated.active_chain,
                hydrated.undo_by_block,
                hydrated.maybe_confirmed_txid_counts,
            );
            let mut manager = ManagedChainstate::from_chainstate(
                FjallChainstateStore::from_store(store),
                chain,
                FlushLifecycle::ready(FlushPolicyTime::new(0), FlushPolicyTime::new(0), 0, false),
            );
            manager
                .disconnect_tip(&repeat.block)
                .expect("managed durable disconnect");
            manager
                .flush_with_mode(FlushMode::Always, FlushPolicyTime::new(11), u64::MAX)
                .expect("durable disconnected checkpoint");
            assert_eq!(
                manager
                    .chainstate()
                    .admission_snapshot()
                    .expect("restored admission"),
                prior
            );
        }
        // Assert: old complete Coin and whole prior endpoint survived physical reopen.
        {
            let store = FjallNodeStore::open(&path).expect("physical restored reopen");
            let hydrated = store
                .hydrate_chainstate_for_open()
                .expect("restored hydration")
                .expect("restored rows");
            assert_eq!(hydrated.active_chain, prior.active_chain);
            assert_eq!(hydrated.utxos, prior.utxos);
            assert_eq!(
                hydrated.maybe_confirmed_txid_counts,
                prior.maybe_confirmed_txid_counts
            );
            assert_eq!(
                store.coins_view().best_block().expect("restored best"),
                prior.tip().map(|position| position.block_hash)
            );
            assert!(
                store
                    .maybe_basic_filter_state()
                    .expect("BASIC state")
                    .is_none()
            );
            assert_ne!(
                store.validation_provenance(hash).expect("provenance"),
                ValidationProvenance::ScriptsValid
            );
            let retained = hydrated
                .undo_by_block
                .get(&hash)
                .expect("retained historical undo");
            assert_eq!(
                Some(retained.clone()),
                store.load_undo(hash).expect("retained undo read")
            );
            assert_eq!(
                store.load_block(hash).expect("retained body read"),
                Some(repeat.block.clone())
            );
            assert_eq!(hydrated.undo_by_block.len(), prior.undo_by_block.len() + 1);
            assert_eq!(hydrated.utxos.get(&outpoint), Some(&old));
            let active_undo = hydrated
                .undo_by_block
                .into_iter()
                .filter(|(undo_hash, _)| {
                    hydrated
                        .active_chain
                        .iter()
                        .any(|position| position.block_hash == *undo_hash)
                })
                .collect();
            let chain = Chainstate::from_coins_cache_for_mechanical_fixture(
                CoinsCache::from_parent(store.coins_view()),
                hydrated.active_chain,
                active_undo,
                hydrated.maybe_confirmed_txid_counts,
            );
            let mut manager = ManagedChainstate::from_chainstate(
                FjallChainstateStore::from_store(store),
                chain,
                FlushLifecycle::ready(FlushPolicyTime::new(0), FlushPolicyTime::new(0), 0, false),
            );
            manager
                .connect_seeded_historical_fixture(&repeat.block, &repeat.history, repeat.now)
                .expect("same seeded managed reconnect");
            assert_eq!(
                manager
                    .chainstate()
                    .admission_snapshot()
                    .expect("reconnected admission"),
                connected
            );
        }
        {
            let store = FjallNodeStore::open(&path).expect("physical reconnected reopen");
            assert_eq!(
                store.hydrate_chainstate_for_open().expect("hydration"),
                Some(connected)
            );
        }
        eprintln!("CASE RG16-BIP30-{repeat_height}-RESTORE PASS");
        if repeat_height == 91880 {
            eprintln!("CASE RG16-BIP30-91880-EXTERNAL-SPEND PASS");
        }
        std::fs::remove_dir_all(path).expect("all physical handles released");
    }
}

#[test]
fn phase167_historical_bip30_missing_and_corrupt_undo_refuse_without_accepted_change() {
    use crate::StorageNamespace;

    // Arrange: independent stores for each fault preserve exact source body and connected endpoint.
    let cases = load_cases(&[
        "bip30-repeat-91842",
        "bip30-original-91812",
        "bip30-repeat-91880",
        "bip30-original-91722",
    ]);
    for (repeat_height, original_height) in [(91842, 91812), (91880, 91722)] {
        let repeat = cases
            .iter()
            .find(|case| case.history.height() == repeat_height)
            .expect("repeat");
        let original = cases
            .iter()
            .find(|case| case.history.height() == original_height)
            .expect("original");
        let txid = transaction_txid(&original.block.transactions[0]).expect("txid");
        let mut prior = repeat.snapshot.clone();
        assert!(
            prior
                .utxos
                .insert(
                    OutPoint { txid, vout: 0 },
                    Coin {
                        output: original.block.transactions[0].outputs[0].clone(),
                        is_coinbase: true,
                        created_height: original_height,
                        created_median_time_past: original
                            .snapshot
                            .tip()
                            .expect("original parent")
                            .median_time_past,
                    }
                )
                .is_none()
        );
        let hash = block_hash(&repeat.block.header);
        for fault in ["missing", "decode", "binding"] {
            let mut nonce = [0; 16];
            getrandom::fill(&mut nonce).expect("nonce");
            let path = std::env::temp_dir()
                .canonicalize()
                .expect("temp root")
                .join(format!(
                    "phase167-bip30-undo-{fault}-{:x}",
                    u128::from_le_bytes(nonce)
                ));
            let connected = {
                let store = FjallNodeStore::open(&path).expect("fresh store");
                store
                    .seed_coins_from_snapshot(&prior)
                    .expect("prior physical seed");
                let chain = Chainstate::from_coins_cache_for_mechanical_fixture(
                    CoinsCache::from_parent(store.coins_view()),
                    prior.active_chain.clone(),
                    prior.undo_by_block.clone(),
                    None,
                );
                let mut manager = ManagedChainstate::from_chainstate(
                    FjallChainstateStore::from_store(store),
                    chain,
                    FlushLifecycle::ready(
                        FlushPolicyTime::new(0),
                        FlushPolicyTime::new(0),
                        0,
                        false,
                    ),
                );
                manager
                    .store()
                    .inner()
                    .save_block(&repeat.block, crate::PersistMode::Sync)
                    .expect("retained exact body");
                manager
                    .connect_seeded_historical_fixture(&repeat.block, &repeat.history, repeat.now)
                    .expect("managed connect");
                manager
                    .chainstate()
                    .admission_snapshot()
                    .expect("connected")
            };
            let pre_injection = physical_manifest(&path);
            {
                let store = FjallNodeStore::open(&path).expect("injection store");
                let undo = store
                    .load_undo(hash)
                    .expect("good undo")
                    .expect("present undo");
                let (key, _) =
                    FjallNodeStore::submission_undo_row(hash, &undo).expect("exact undo key");
                assert_eq!(
                    store.load_block(hash).expect("body"),
                    Some(repeat.block.clone())
                );
                let space = store
                    .database()
                    .keyspace(
                        StorageNamespace::Chainstate.as_str(),
                        fjall::KeyspaceCreateOptions::default,
                    )
                    .expect("existing chainstate keyspace");
                let logical_rows = || {
                    space
                        .iter()
                        .map(|entry| {
                            let (row_key, value) = entry.into_inner().expect("logical row");
                            (row_key.to_vec(), value.to_vec())
                        })
                        .collect::<std::collections::BTreeMap<_, _>>()
                };
                let before_rows = logical_rows();
                assert!(before_rows.contains_key(key.as_bytes()));
                match fault {
                    "missing" => {
                        assert!(space.contains_key(&key).expect("present logical row"));
                        space.remove(&key).expect("remove only undo row");
                        store
                            .database()
                            .persist(fjall::PersistMode::SyncAll)
                            .expect("sync row removal");
                        assert!(!space.contains_key(&key).expect("absent logical row"));
                        assert!(!store.has_undo(hash).expect("missing undo"));
                    }
                    "decode" => {
                        store
                            .write_raw_for_test(StorageNamespace::Chainstate, &key, vec![0xff])
                            .expect("malformed serialized undo");
                        assert!(store.load_undo(hash).is_err());
                    }
                    "binding" => {
                        let mut wrong = undo;
                        wrong.transactions.push(Default::default());
                        store
                            .save_undo(hash, &wrong, crate::PersistMode::Sync)
                            .expect("structurally decoded wrong binding");
                        assert_eq!(store.load_undo(hash).expect("decodable"), Some(wrong));
                    }
                    _ => unreachable!("enumerated fault"),
                }
                let after_rows = logical_rows();
                let mut unchanged_rows = before_rows.clone();
                unchanged_rows.remove(key.as_bytes());
                let mut after_without_injected_key = after_rows.clone();
                after_without_injected_key.remove(key.as_bytes());
                assert_eq!(
                    after_without_injected_key, unchanged_rows,
                    "only the intended logical undo row changes"
                );
                assert_ne!(
                    after_rows.get(key.as_bytes()),
                    before_rows.get(key.as_bytes())
                );
                assert_eq!(
                    store.load_block(hash).expect("body retained"),
                    Some(repeat.block.clone())
                );
            }
            let post_injection = physical_manifest(&path);
            assert_ne!(
                pre_injection, post_injection,
                "injection has a physical effect"
            );
            // Act: read the injected state, then attempt managed disconnect when decode permits it.
            {
                let store = FjallNodeStore::open(&path).expect("post-injection reopen");
                match fault {
                    "decode" => assert!(store.hydrate_chainstate_for_open().is_err()),
                    _ => {
                        let hydrated = store
                            .hydrate_chainstate_for_open()
                            .expect("hydration")
                            .expect("connected rows");
                        assert_eq!(hydrated.active_chain, connected.active_chain);
                        assert_eq!(hydrated.utxos, connected.utxos);
                        let chain = Chainstate::from_coins_cache_for_mechanical_fixture(
                            CoinsCache::from_parent(store.coins_view()),
                            hydrated.active_chain,
                            hydrated.undo_by_block,
                            hydrated.maybe_confirmed_txid_counts,
                        );
                        let mut manager = ManagedChainstate::from_chainstate(
                            FjallChainstateStore::from_store(store),
                            chain,
                            FlushLifecycle::ready(
                                FlushPolicyTime::new(0),
                                FlushPolicyTime::new(0),
                                0,
                                false,
                            ),
                        );
                        assert!(
                            manager.disconnect_tip(&repeat.block).is_err(),
                            "{fault} refuses"
                        );
                        let unchanged = manager
                            .chainstate()
                            .admission_snapshot()
                            .expect("unchanged");
                        assert_eq!(unchanged.active_chain, connected.active_chain);
                        assert_eq!(unchanged.utxos, connected.utxos);
                    }
                }
            }
            // Assert: the failed attempt cannot mutate the post-injection physical endpoint.
            assert_eq!(physical_manifest(&path), post_injection);
            if fault == "missing" {
                eprintln!("CASE RG16-BIP30-{repeat_height}-MISSING-UNDO PASS");
            }
            std::fs::remove_dir_all(path).expect("all fault-store handles dropped");
        }
        eprintln!("CASE RG16-BIP30-{repeat_height}-MALFORMED-UNDO PASS");
    }
}
