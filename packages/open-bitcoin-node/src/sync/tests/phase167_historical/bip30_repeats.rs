// Parity breadcrumbs:
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/coins.cpp

use super::source::load_cases;
use crate::{FjallChainstateStore, FjallNodeStore, ManagedChainstate, chainstate::FlushLifecycle};
use open_bitcoin_core::{
    chainstate::{Chainstate, ChainstateError, Coin, CoinsCache, FlushPolicyTime},
    consensus::{
        Bip34Context, CandidateHistory, CanonicalChain, HistoryError, HistoryHeader, HistoryOrigin,
        block_hash, transaction_txid,
    },
    primitives::OutPoint,
};

#[test]
fn phase167_historical_bip30_exact_repeats_preserve_complete_old_coin() {
    // Arrange: one bounded corpus pass selects both repeats and their original bodies.
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
        assert_eq!(
            repeat.overwrite_original,
            vec![super::source::display(
                block_hash(&original.block.header).as_bytes()
            )]
        );
        let txid = transaction_txid(&original.block.transactions[0]).expect("original txid");
        assert_eq!(
            txid,
            transaction_txid(&repeat.block.transactions[0]).expect("repeated txid")
        );
        let outpoint = OutPoint { txid, vout: 0 };
        let old = Coin {
            output: original.block.transactions[0].outputs[0].clone(),
            is_coinbase: true,
            created_height: original_height,
            created_median_time_past: original
                .snapshot
                .tip()
                .expect("original parent")
                .median_time_past,
        };
        let mut snapshot = repeat.snapshot.clone();
        assert!(
            snapshot
                .utxos
                .insert(outpoint.clone(), old.clone())
                .is_none()
        );
        snapshot.maybe_confirmed_txid_counts = Some(std::collections::HashMap::from([(txid, 1)]));
        let mut nonce = [0; 16];
        getrandom::fill(&mut nonce).expect("test nonce");
        let path = std::env::temp_dir()
            .canonicalize()
            .expect("temp root")
            .join(format!("phase167-bip30-{:x}", u128::from_le_bytes(nonce)));
        {
            let store = FjallNodeStore::open(&path).expect("mechanical durable store");
            store
                .seed_coins_from_snapshot(&snapshot)
                .expect("complete mechanical prior seed");
            assert_eq!(
                store
                    .hydrate_chainstate_for_open()
                    .expect("prior hydration"),
                Some(snapshot.clone())
            );
            let chain = Chainstate::from_snapshot_for_mechanical_fixture(snapshot.clone());
            let mut manager = ManagedChainstate::from_chainstate(
                FjallChainstateStore::from_store(store),
                chain,
                FlushLifecycle::ready(FlushPolicyTime::new(0), FlushPolicyTime::new(0), 0, false),
            );
            assert_eq!(repeat.history.origin(), HistoryOrigin::IndependentlySeeded);
            // Act: the selected historical body reaches the managed durable apply kernel.
            let accepted = manager.connect_seeded_historical_fixture(
                &repeat.block,
                &repeat.history,
                repeat.now,
            );
            // Assert: the full old Coin is retained in the single overwrite undo entry.
            assert!(accepted.is_ok(), "repeat {repeat_height}: {accepted:?}");
            let connected = manager.chainstate().snapshot();
            let undo = connected
                .undo_by_block
                .get(&block_hash(&repeat.block.header))
                .expect("undo");
            assert_eq!(undo.overwrites.len(), 1);
            assert_eq!(undo.overwrites[0].overwritten.len(), 1);
            assert_eq!(undo.overwrites[0].overwritten[0].previous_coin, old);
            assert_eq!(undo.overwrites[0].overwritten[0].outpoint, outpoint);
            assert_ne!(connected.utxos.get(&outpoint), Some(&old));
            assert!(!manager.chainstate().has_earned_work());
            assert!(manager.chainstate().maybe_canonical_chain().is_none());
            manager
                .store()
                .inner()
                .save_block(&repeat.block, crate::PersistMode::Sync)
                .expect("exact body");
            eprintln!("CASE RG16-BIP30-{repeat_height}-MANAGED PASS");
        }
        std::fs::remove_dir_all(path).expect("all store handles dropped");

        for fully_spent in [true, false] {
            let mut controlled = snapshot.clone();
            if fully_spent {
                assert_eq!(controlled.utxos.remove(&outpoint), Some(old.clone()));
            } else {
                controlled
                    .utxos
                    .get_mut(&outpoint)
                    .expect("old live coin")
                    .is_coinbase = false;
            }
            controlled.maybe_confirmed_txid_counts = None;
            let mut nonce = [0; 16];
            getrandom::fill(&mut nonce).expect("control nonce");
            let path = std::env::temp_dir()
                .canonicalize()
                .expect("temp root")
                .join(format!(
                    "phase167-bip30-control-{:x}",
                    u128::from_le_bytes(nonce)
                ));
            {
                let store = FjallNodeStore::open(&path).expect("independent control store");
                store
                    .seed_coins_from_snapshot(&controlled)
                    .expect("complete control seed");
                let chain = Chainstate::from_coins_cache_for_mechanical_fixture(
                    CoinsCache::from_parent(store.coins_view()),
                    controlled.active_chain.clone(),
                    controlled.undo_by_block.clone(),
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
                let before = manager.chainstate().admission_snapshot().expect("before");
                let result = manager.connect_seeded_historical_fixture(
                    &repeat.block,
                    &repeat.history,
                    repeat.now,
                );
                if fully_spent {
                    assert!(result.is_ok(), "fully spent duplicate: {result:?}");
                    let after = manager.chainstate().admission_snapshot().expect("after");
                    let undo = after
                        .undo_by_block
                        .get(&block_hash(&repeat.block.header))
                        .expect("captured scan");
                    assert_eq!(undo.overwrites.len(), 1);
                    assert!(undo.overwrites[0].overwritten.is_empty());
                    eprintln!("CASE RG16-BIP30-{repeat_height}-FULLY-SPENT PASS");
                } else {
                    assert!(
                        matches!(result, Err(ChainstateError::OutputOverwrite { .. })),
                        "{result:?}"
                    );
                    assert_eq!(
                        manager
                            .chainstate()
                            .admission_snapshot()
                            .expect("unchanged"),
                        before
                    );
                    assert_eq!(
                        manager
                            .store()
                            .inner()
                            .hydrate_chainstate_for_open()
                            .expect("durable unchanged"),
                        Some(controlled)
                    );
                }
            }
            std::fs::remove_dir_all(path).expect("all control handles dropped");
        }
        if repeat_height == 91880 {
            // A distinct unrelated live transaction output must still fail BIP30.
            let spend = &repeat.block.transactions[1];
            let extra = OutPoint {
                txid: transaction_txid(spend).expect("spend txid"),
                vout: 0,
            };
            let mut prior = snapshot.clone();
            prior.maybe_confirmed_txid_counts = None;
            assert!(prior.utxos.insert(extra.clone(), old.clone()).is_none());
            let mut nonce = [0; 16];
            getrandom::fill(&mut nonce).expect("unrelated nonce");
            let path = std::env::temp_dir()
                .canonicalize()
                .expect("temp root")
                .join(format!(
                    "phase167-bip30-unrelated-{:x}",
                    u128::from_le_bytes(nonce)
                ));
            {
                let store = FjallNodeStore::open(&path).expect("unrelated store");
                store.seed_coins_from_snapshot(&prior).expect("prior seed");
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
                assert!(matches!(manager.connect_seeded_historical_fixture(
                    &repeat.block, &repeat.history, repeat.now,
                ), Err(ChainstateError::OutputOverwrite { outpoint }) if outpoint == extra));
                assert_eq!(
                    manager
                        .chainstate()
                        .admission_snapshot()
                        .expect("unchanged"),
                    prior
                );
                assert_eq!(
                    manager
                        .store()
                        .inner()
                        .hydrate_chainstate_for_open()
                        .expect("durable unchanged"),
                    Some(prior)
                );
                eprintln!("CASE RG16-BIP30-UNRELATED-LIVE-DUPLICATE PASS");
            }
            std::fs::remove_dir_all(path).expect("unrelated handles dropped");
        }
    }
}

#[test]
fn phase167_historical_bip30_wrong_network_height_hash_and_ancestor_refuse_first() {
    // Arrange: mutate checked branch facts from the exact two pinned repeat bodies.
    let cases = load_cases(&["bip30-repeat-91842", "bip30-repeat-91880"]);
    assert_eq!(cases.len(), 2);
    for case in cases {
        let candidate = case.block.header.clone();
        let parent = case.history.parent().clone();
        let ancestors = case.history.branch()[1..].to_vec();
        let rules = CanonicalChain::Mainnet.spec().rules();
        let mut wrong_network = parent.clone();
        wrong_network.chain = CanonicalChain::Testnet3;
        assert_eq!(
            CandidateHistory::check_seeded(
                rules,
                candidate.clone(),
                wrong_network,
                ancestors.clone(),
                None
            ),
            Err(HistoryError::WrongNetwork {
                height: parent.height
            })
        );
        eprintln!(
            "CASE RG16-BIP30-{}-WRONG-NETWORK PASS",
            case.history.height()
        );
        let mut wrong_height = parent.clone();
        wrong_height.height -= 1;
        assert!(matches!(
            CandidateHistory::check_seeded(
                rules,
                candidate.clone(),
                wrong_height,
                ancestors.clone(),
                None
            ),
            Err(HistoryError::HeightMismatch { .. })
        ));
        eprintln!(
            "CASE RG16-BIP30-{}-WRONG-HEIGHT PASS",
            case.history.height()
        );
        let mut wrong_hash = parent.clone();
        wrong_hash.block_hash = Default::default();
        assert_eq!(
            CandidateHistory::check_seeded(
                rules,
                candidate.clone(),
                wrong_hash,
                ancestors.clone(),
                None
            ),
            Err(HistoryError::HashMismatch {
                height: parent.height
            })
        );
        eprintln!("CASE RG16-BIP30-{}-WRONG-HASH PASS", case.history.height());
        let mut wrong_ancestors = ancestors;
        let mut altered = wrong_ancestors[0].header.clone();
        altered.nonce ^= 1;
        wrong_ancestors[0] =
            HistoryHeader::new(CanonicalChain::Mainnet, wrong_ancestors[0].height, altered);
        assert_eq!(
            CandidateHistory::check_seeded(rules, candidate, parent, wrong_ancestors, None),
            Err(HistoryError::ParentMismatch {
                height: case.history.height() - 2
            })
        );
        eprintln!(
            "CASE RG16-BIP30-{}-WRONG-ANCESTOR PASS",
            case.history.height()
        );
    }
}

#[test]
fn phase167_historical_bip30_bip34_checked_synthetic_recheck_boundary() {
    // Arrange: source-selected anchor headers are authenticated by the Knots hash.
    // Header hex: blockstream.info mainnet/testnet block header endpoints.
    let anchors = [
        (
            CanonicalChain::Mainnet,
            "02000000d629b6d62e07d09960f1c093dbfea0fa1b049c5b8d4c83b35102000000000000bad87c7cd313c982787729637b1ff4651df74ee34803fe84af74b667f010a2b501fd4f516e81021a160a0594",
        ),
        (
            CanonicalChain::Testnet3,
            "020000006189e027d2c6178ff9a3cfeffe6b595ae12223df9c0cb6fcf85f3916000000001b60c87bc41844b8098833329d4b680b762c53ce590218736cfc78acc3e53a6f13d03a5056174d1cc8e703e5",
        ),
    ];
    for (network, encoded) in anchors {
        let label = match network {
            CanonicalChain::Mainnet => "MAINNET",
            CanonicalChain::Testnet3 => "TESTNET3",
            _ => unreachable!("enumerated selected networks"),
        };
        let selected = network.spec().rules();
        let anchor_header = open_bitcoin_codec::parse_block_header(&super::source::bytes(encoded))
            .expect("selected 80-byte anchor");
        assert_eq!(
            block_hash(&anchor_header),
            selected.maybe_bip34_hash().expect("selected commitment")
        );
        let matching = HistoryHeader::new(network, selected.bip34_height(), anchor_header.clone());
        let mut different = anchor_header;
        different.nonce ^= 1;
        let mismatching = HistoryHeader::new(network, selected.bip34_height(), different);
        assert_ne!(matching.block_hash, mismatching.block_hash);
        for candidate_height in [1_983_701_u32, 1_983_702] {
            // A 156-row suffix covers parent MTP and expiry without pretending to earn work.
            let mut previous = Default::default();
            let mut ascending = Vec::new();
            for height in candidate_height - 156..candidate_height {
                let header = open_bitcoin_core::primitives::BlockHeader {
                    version: 4,
                    previous_block_hash: previous,
                    time: 1_600_000_000 + height,
                    bits: 0x1c0ffff0,
                    nonce: height,
                    ..Default::default()
                };
                previous = block_hash(&header);
                ascending.push(HistoryHeader::new(network, height, header));
            }
            ascending.reverse();
            let parent = ascending.remove(0);
            let candidate = open_bitcoin_core::primitives::BlockHeader {
                version: 4,
                previous_block_hash: parent.block_hash,
                time: parent.header.time + 1,
                bits: parent.header.bits,
                ..Default::default()
            };
            let checked = |maybe_seed| {
                CandidateHistory::check_seeded(
                    selected,
                    candidate.clone(),
                    parent.clone(),
                    ascending.clone(),
                    maybe_seed,
                )
            };
            if candidate_height == 1_983_701 {
                let matched = checked(Some(matching.clone())).expect("matching selected seed");
                assert_eq!(matched.origin(), HistoryOrigin::IndependentlySeeded);
                assert_eq!(
                    matched.bip34_context(),
                    Bip34Context::IndependentlySeeded(matching.block_hash)
                );
                assert!(!matched.rules().requires_bip30_scan());
                eprintln!("CASE RG16-BIP34-{label}-1983701-MATCH PASS");
                let mismatch = checked(Some(mismatching.clone())).expect("different selected seed");
                assert_eq!(mismatch.origin(), HistoryOrigin::IndependentlySeeded);
                assert!(mismatch.rules().requires_bip30_scan());
                eprintln!("CASE RG16-BIP34-{label}-1983701-MISMATCH PASS");
                assert_eq!(
                    checked(None),
                    Err(HistoryError::MissingAncestor {
                        height: selected.bip34_height()
                    })
                );
                eprintln!("CASE RG16-BIP34-{label}-1983701-ABSENT PASS");
            } else {
                let absent = checked(None).expect("seed unneeded at recheck");
                assert_eq!(absent.bip34_context(), Bip34Context::UnneededAtRecheck);
                assert!(absent.rules().requires_bip30_scan());
                eprintln!("CASE RG16-BIP34-{label}-1983702-ABSENT PASS");
                assert_eq!(
                    checked(Some(matching.clone())),
                    Err(HistoryError::UnexpectedBip34Seed)
                );
                eprintln!("CASE RG16-BIP34-{label}-1983702-MATCH PASS");
                assert_eq!(
                    checked(Some(mismatching.clone())),
                    Err(HistoryError::UnexpectedBip34Seed)
                );
                eprintln!("CASE RG16-BIP34-{label}-1983702-MISMATCH PASS");
            }
        }
    }
}
