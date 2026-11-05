// Parity breadcrumbs:
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/pow.cpp
// - packages/bitcoin-knots/test/functional/p2p_compactblocks.py

use super::*;
use crate::chainstate::BasicFilterStartupMode;
use crate::storage::validation_history::ValidationProvenance as Provenance;
use crate::{FjallChainstateStore, FjallCoinsView, FjallNodeStore, ManagedChainstate};
use open_bitcoin_core::{
    consensus::block_merkle_root,
    primitives::{
        Amount, Block, BlockHeader, OutPoint, ScriptBuf, ScriptWitness, Transaction,
        TransactionInput, TransactionOutput,
    },
};
use open_bitcoin_network::{CompactBlockReceiveFacts, NetworkError, WireNetworkMessage};

mod compact_reconstruction;
mod compact_stored_body;
mod fixtures;
use fixtures::*;

#[test]
fn phase167_canonical_admission_genuine_header_and_derived_import_work() {
    with_owner(|network, _, params| {
        // Arrange
        let block = mined_child(&CanonicalChain::Regtest.spec().genesis_header(), 1);
        let before = network.chainstate_snapshot().expect("coins snapshot");
        // Act
        admit(network, &block, params);
        // Assert
        let headers = network.peer_manager.header_store();
        assert!(headers.is_canonically_admitted(block_hash(&block.header)));
        let held = headers
            .canonical_header_validation_context(&block.header)
            .expect("held clock");
        assert!(held.current_time > i64::from(block.header.time));
        assert_eq!(
            network.chainstate_snapshot().expect("header-only coins"),
            before
        );

        let genesis = CanonicalChain::Regtest.spec().genesis_header();
        let imported = HeaderStore::from_entries([
            HeaderEntry {
                block_hash: block_hash(&genesis),
                header: genesis,
                height: 0,
                chain_work: open_bitcoin_core::chainstate::ChainPosition::untrusted_fixture_work(
                    u128::MAX,
                ),
            },
            HeaderEntry {
                block_hash: block_hash(&block.header),
                header: block.header.clone(),
                height: 999,
                chain_work: open_bitcoin_core::chainstate::ChainPosition::untrusted_fixture_work(
                    u128::MAX,
                ),
            },
        ])
        .expect("untrusted arithmetic carrier");
        network
            .try_seed_canonical_headers(imported)
            .expect("contextual re-admission");
        let restored = network
            .peer_manager
            .header_store()
            .canonical_header_validation_context(&block.header);
        assert_eq!(restored.expect("original imported clock"), held);
        let entry = network
            .peer_manager
            .header_store()
            .entry(&block_hash(&block.header))
            .expect("child");
        assert_eq!(entry.height, 1);
        assert_eq!(
            entry.chain_work,
            before.active_chain[0].chain_work.wrapping_add(
                open_bitcoin_core::consensus::chain_proof::block_proof(block.header.bits)
                    .expect("proof")
            )
        );
    });
}

#[test]
fn phase167_canonical_admission_actual_stored_body_reaches_coins_stage() {
    with_owner(|network, store, params| {
        // Arrange
        let block = mined_child(&CanonicalChain::Regtest.spec().genesis_header(), 1);
        admit(network, &block, params);
        // Act
        let result = network.connect_stored_block(
            &block,
            open_bitcoin_core::consensus::chain_proof::ProofWork::ZERO,
            i64::MAX,
            ScriptVerifyFlags::NONE,
            params,
        );
        // Assert
        assert!(result.is_ok(), "real managed body route: {result:?}");
        let state = network.chainstate.chainstate();
        assert_eq!(
            state.tip().expect("accepted child").block_hash,
            block_hash(&block.header)
        );
        assert_eq!(
            state.coins_best_block().expect("coins endpoint"),
            Some(block_hash(&block.header))
        );
        assert_eq!(
            network
                .chainstate
                .validation_provenance(block_hash(&block.header))
                .expect("receipt"),
            crate::storage::validation_history::ValidationProvenance::ScriptsValid
        );
        assert_eq!(
            store
                .validation_provenance(block_hash(&block.header))
                .expect("published receipt"),
            crate::storage::validation_history::ValidationProvenance::ScriptsValid
        );
    });
}

#[test]
fn phase167_canonical_admission_peer_routes_reach_genuine_absorption() {
    for route in 0..6 {
        with_owner(|network, store, params| {
            // Arrange
            let block = mined_child(&CanonicalChain::Regtest.spec().genesis_header(), 1);
            compact_peer(network, params);
            admit(network, &block, params);
            let before = accepted(network, store);
            // Act
            let result = match route {
                0 => network.receive_message(
                    1,
                    WireNetworkMessage::Block(block.clone()),
                    1,
                    ScriptVerifyFlags::NONE,
                    params,
                ),
                1 => network.receive_sync_message(
                    1,
                    WireNetworkMessage::Block(block.clone()),
                    1,
                    ScriptVerifyFlags::NONE,
                    params,
                ),
                2 => network.receive_message_for_durable_serving(
                    1,
                    WireNetworkMessage::Block(block.clone()),
                    1,
                    ScriptVerifyFlags::NONE,
                    params,
                    |_| true,
                ),
                3 => network.receive_message(
                    1,
                    WireNetworkMessage::CompactBlock(compact(&block, false)),
                    1,
                    ScriptVerifyFlags::NONE,
                    params,
                ),
                4 => {
                    let partial = network
                        .receive_message(
                            1,
                            WireNetworkMessage::CompactBlock(compact(&block, true)),
                            1,
                            ScriptVerifyFlags::NONE,
                            params,
                        )
                        .expect("partial reconstruction");
                    assert!(partial.maybe_block_disposition.is_none());
                    assert_unaccepted(network, store, &block, &before);
                    assert!(
                        partial
                            .outbound
                            .iter()
                            .any(|message| matches!(message, WireNetworkMessage::GetBlockTxn(_)))
                    );
                    network.receive_message(
                        1,
                        WireNetworkMessage::BlockTxn(open_bitcoin_codec::BlockTransactions {
                            block_hash: block_hash(&block.header),
                            transactions: block.transactions.clone(),
                        }),
                        1,
                        ScriptVerifyFlags::NONE,
                        params,
                    )
                }
                _ => network.process_actions(
                    1,
                    vec![open_bitcoin_network::PeerAction::ReceivedBlock(
                        block.clone(),
                    )],
                    1,
                    ScriptVerifyFlags::NONE,
                    params,
                    super::super::super::action_translation::immediate_inventory_serving(),
                ),
            };
            // Assert
            let result = result.unwrap_or_else(|error| panic!("route {route}: {error}"));
            let hash = block_hash(&block.header);
            assert!(
                matches!(
                    result.maybe_block_disposition,
                    Some(crate::network::BlockConnectDisposition::Connected(_))
                ),
                "route {route}: {result:?}"
            );
            let state = network.chainstate.chainstate();
            assert_eq!(state.tip().expect("accepted").block_hash, hash);
            let provenance = network.chainstate.validation_provenance(hash);
            assert_eq!(provenance.expect("receipt"), Provenance::ScriptsValid);
            assert_ne!(accepted(network, store), before);
        });
    }
}

#[test]
fn phase167_canonical_admission_caller_clock_cannot_spoof_or_replace_observation() {
    with_owner(|network, store, params| {
        // Arrange
        let mut future = mined_child(&CanonicalChain::Regtest.spec().genesis_header(), 1);
        future.header.time = u32::try_from(
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("clock")
                .as_secs(),
        )
        .expect("time")
            + 10_000;
        mine(&mut future.header);
        let before = accepted(network, store);
        // Act
        let result = network.handle_validated_headers(
            1,
            HeadersMessage {
                headers: vec![future.header.clone()],
            },
            i64::MAX,
            params,
        );
        // Assert
        assert_eq!(
            error_code(result.expect_err("caller future clock rejected")),
            "time-too-new"
        );
        assert_unaccepted(network, store, &future, &before);
        let block = mined_child(&CanonicalChain::Regtest.spec().genesis_header(), 1);
        admit(network, &block, params);
        let held = network
            .peer_manager
            .header_store()
            .canonical_header_validation_context(&block.header)
            .expect("original clock");
        admit(network, &block, params);
        let duplicate = network
            .peer_manager
            .header_store()
            .canonical_header_validation_context(&block.header);
        assert_eq!(duplicate.expect("duplicate clock"), held);
        network
            .connect_stored_block(
                &block,
                open_bitcoin_core::consensus::chain_proof::ProofWork::ZERO,
                i64::MIN,
                ScriptVerifyFlags::NONE,
                params,
            )
            .expect("replay uses held clock");
        let post_body = network
            .peer_manager
            .header_store()
            .canonical_header_validation_context(&block.header);
        assert_eq!(post_body.expect("body clock"), held);
    });
}

#[test]
fn phase167_canonical_admission_wrong_context_imports_preserve_accepted_state() {
    with_owner(|network, store, _| {
        for fault in 0..3 {
            // Arrange
            let genesis = CanonicalChain::Regtest.spec().genesis_header();
            let mut block = mined_child(&genesis, 1);
            match fault {
                0 => block.header.version = 1,
                1 => block.header.time = genesis.time,
                _ => block.header.bits = 0x2070_ffff,
            }
            mine(&mut block.header);
            let rows = [
                HeaderEntry {
                    block_hash: block_hash(&genesis),
                    header: genesis,
                    height: 0,
                    chain_work: open_bitcoin_core::consensus::chain_proof::ProofWork::ZERO,
                },
                HeaderEntry {
                    block_hash: block_hash(&block.header),
                    header: block.header.clone(),
                    height: 999,
                    chain_work:
                        open_bitcoin_core::chainstate::ChainPosition::untrusted_fixture_work(
                            u128::MAX,
                        ),
                },
            ];
            let before = accepted(network, store);
            let before_headers = network.peer_manager.header_store().clone();
            // Act
            let result = network.try_seed_canonical_headers(
                HeaderStore::from_entries(rows).expect("raw linked import"),
            );
            // Assert
            assert!(result.is_err(), "wrong context {fault}");
            assert_eq!(*network.peer_manager.header_store(), before_headers);
            assert_unaccepted(network, store, &block, &before);
        }
    });
}

#[test]
fn phase167_canonical_admission_fixture_seed_never_overwrites_proved_owner() {
    with_owner(|network, store, params| {
        // Arrange
        let block = mined_child(&CanonicalChain::Regtest.spec().genesis_header(), 1);
        let before = accepted(network, store);
        let before_headers = network.peer_manager.header_store().clone();
        // Act
        network.seed_header_store(HeaderStore::default());
        let result = network.try_admit_fixture_headers(HeaderStore::default(), i64::MAX, params);
        // Assert
        assert!(result.is_err());
        let mut foreign = HeaderStore::default();
        foreign
            .insert_header(CanonicalChain::Mainnet.spec().genesis_header())
            .expect("foreign root");
        let error = network
            .try_admit_fixture_headers(foreign, i64::MAX, params)
            .expect_err("foreign fixture root");
        assert_eq!(error_code(error), "noncanonical-genesis-header");
        assert_eq!(*network.peer_manager.header_store(), before_headers);
        assert_unaccepted(network, store, &block, &before);
    });
}

#[test]
fn phase167_canonical_admission_header_only_and_wrong_body_cannot_publish_acceptance() {
    for mode in [
        crate::chainstate::BasicFilterStartupMode::Disabled,
        crate::chainstate::BasicFilterStartupMode::Enabled,
    ] {
        with_mode(mode, |network, store, params| {
            // Arrange
            let block = mined_child(&CanonicalChain::Regtest.spec().genesis_header(), 1);
            let before = accepted(network, store);
            admit(network, &block, params);
            assert_unaccepted(network, store, &block, &before);
            let mut wrong = block.clone();
            wrong.transactions[0].outputs[0].script_pubkey =
                ScriptBuf::from_bytes(vec![0]).expect("different body");
            // Act
            let result = network.connect_stored_block(
                &wrong,
                open_bitcoin_core::chainstate::ChainPosition::untrusted_fixture_work(u128::MAX),
                i64::MAX,
                ScriptVerifyFlags::NONE,
                params,
            );
            // Assert
            assert!(result.is_err());
            assert!(
                network
                    .peer_manager
                    .header_store()
                    .is_canonically_admitted(block_hash(&block.header))
            );
            assert_unaccepted(network, store, &block, &before);
        });
    }
}

#[test]
fn phase167_canonical_admission_actual_coins_stage_rejects_reward_despite_caller_flags() {
    with_owner(|network, store, params| {
        // Arrange
        let mut block = mined_child(&CanonicalChain::Regtest.spec().genesis_header(), 1);
        block.transactions[0].outputs[0].value =
            Amount::from_sats(5_000_000_001).expect("overreward");
        block.header.merkle_root = block_merkle_root(&block.transactions)
            .expect("actual body root")
            .0;
        mine(&mut block.header);
        admit(network, &block, params);
        let before = accepted(network, store);
        // Act
        let result = network.connect_stored_block(
            &block,
            open_bitcoin_core::chainstate::ChainPosition::untrusted_fixture_work(u128::MAX),
            i64::MAX,
            ScriptVerifyFlags::NONE,
            params,
        );
        // Assert
        let error = result.expect_err("actual reward validation");
        assert!(error.to_string().contains("bad-cb-amount"), "{error}");
        assert!(
            network
                .peer_manager
                .header_store()
                .maybe_body_eligibility_binding(block_hash(&block.header))
                .is_some(),
            "body linkage alone is insufficient"
        );
        assert_unaccepted(network, store, &block, &before);
    });
}

#[test]
fn phase167_canonical_admission_caller_params_cannot_replace_selection() {
    with_owner(|network, store, params| {
        // Arrange
        let block = mined_child(&CanonicalChain::Regtest.spec().genesis_header(), 1);
        admit(network, &block, params);
        let before = accepted(network, store);
        let spoofed = ConsensusParams {
            coinbase_maturity: 0,
            ..params
        };
        // Act
        let result = network.connect_stored_block(
            &block,
            open_bitcoin_core::consensus::chain_proof::ProofWork::ZERO,
            i64::MAX,
            ScriptVerifyFlags::NONE,
            spoofed,
        );
        // Assert
        assert!(result.is_err());
        assert_unaccepted(network, store, &block, &before);
    });
}

#[test]
fn phase167_canonical_admission_unadmitted_header_and_foreign_owner_refuse() {
    for foreign in [false, true] {
        with_owner(|network, store, params| {
            // Arrange
            let block = mined_child(&CanonicalChain::Regtest.spec().genesis_header(), 1);
            if foreign {
                admit(network, &block, params);
                let entries: Vec<_> = network
                    .peer_manager
                    .header_store()
                    .entries()
                    .cloned()
                    .collect();
                network.peer_manager.seed_header_store(
                    HeaderStore::from_entries(entries).expect("same scalar rows, no lineage"),
                );
            }
            let before = accepted(network, store);
            // Act
            let result = network.connect_stored_block(
                &block,
                open_bitcoin_core::chainstate::ChainPosition::untrusted_fixture_work(u128::MAX),
                i64::MAX,
                ScriptVerifyFlags::NONE,
                params,
            );
            // Assert
            assert!(result.is_err());
            assert_unaccepted(network, store, &block, &before);
        });
    }
}

#[test]
fn phase167_canonical_admission_caller_none_cannot_disable_selected_p2sh() {
    with_owner(|network, store, params| {
        // Arrange: mature an actual accepted P2SH coin; no seeded coins or owner labels.
        let mut first = mined_child(&CanonicalChain::Regtest.spec().genesis_header(), 1);
        let mut p2sh = vec![0xa9, 20];
        p2sh.extend(open_bitcoin_core::consensus::crypto::hash160(&[0]));
        p2sh.push(0x87);
        first.transactions[0].outputs[0].script_pubkey =
            ScriptBuf::from_bytes(p2sh).expect("P2SH false redeem");
        first.header.merkle_root = block_merkle_root(&first.transactions).expect("merkle").0;
        mine(&mut first.header);
        let outpoint = OutPoint {
            txid: open_bitcoin_core::consensus::transaction_txid(&first.transactions[0])
                .expect("actual coinbase txid"),
            vout: 0,
        };
        let mut block = first;
        for height in 1..=100 {
            admit(network, &block, params);
            network
                .connect_stored_block(
                    &block,
                    open_bitcoin_core::consensus::chain_proof::ProofWork::ZERO,
                    1,
                    ScriptVerifyFlags::NONE,
                    params,
                )
                .expect("genuine maturation chain");
            block = mined_child(&block.header, height + 1);
        }
        block.transactions.push(Transaction {
            version: 1,
            lock_time: 0,
            inputs: vec![TransactionInput {
                previous_output: outpoint,
                script_sig: ScriptBuf::from_bytes(vec![1, 0]).expect("hash matches, redeem fails"),
                sequence: u32::MAX,
                witness: ScriptWitness::default(),
            }],
            outputs: vec![TransactionOutput {
                value: Amount::from_sats(4_999_999_999).expect("fee"),
                script_pubkey: ScriptBuf::from_bytes(vec![0x51]).expect("true"),
            }],
        });
        block.header.merkle_root = block_merkle_root(&block.transactions)
            .expect("real merkle")
            .0;
        mine(&mut block.header);
        admit(network, &block, params);
        let before = accepted(network, store);
        // Act
        let result = network.connect_stored_block(
            &block,
            open_bitcoin_core::consensus::chain_proof::ProofWork::ZERO,
            i64::MAX,
            ScriptVerifyFlags::NONE,
            params,
        );
        // Assert
        let error = result.expect_err("selected P2SH remains enforced");
        let crate::ManagedNetworkError::Chainstate(
            open_bitcoin_core::chainstate::ChainstateError::TransactionValidation { source },
        ) = error
        else {
            panic!("expected actual transaction script rejection: {error}")
        };
        assert_eq!(source.reject_reason, "mandatory-script-verify-flag-failed");
        assert_unaccepted(network, store, &block, &before);
    });
}
