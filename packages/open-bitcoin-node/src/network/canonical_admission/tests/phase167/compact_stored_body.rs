// Parity breadcrumbs:
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/test/functional/p2p_compactblocks.py

use super::*;

fn request_order(network: &super::fixtures::Network, hash: BlockHash) -> Vec<u64> {
    network.peer_manager().block_request_peer_order(hash)
}

#[test]
fn phase167_compact_stored_body_canonical_connected_completion_uses_earned_history() {
    with_owner(|network, store, params| {
        // Arrange
        let genesis = CanonicalChain::Regtest.spec().genesis_header();
        let block = mined_child(&genesis, 1);
        let hash = block_hash(&block.header);
        let unrelated = mined_child(&CanonicalChain::Regtest.spec().genesis_header(), 2);
        let unrelated_hash = block_hash(&unrelated.header);
        compact_peer(network, params);
        network.connect_outbound_peer(2, 0).expect("second peer");
        network.connect_outbound_peer(3, 0).expect("unrelated peer");
        for peer in [2, 3] {
            for message in [
                WireNetworkMessage::Version(open_bitcoin_network::VersionMessage::default()),
                WireNetworkMessage::Verack,
                WireNetworkMessage::SendCompact(open_bitcoin_codec::SendCompactMessage {
                    announce: true,
                    version: open_bitcoin_codec::BIP152_COMPACT_BLOCKS_VERSION,
                }),
            ] {
                network
                    .receive_message(peer, message, 1, ScriptVerifyFlags::NONE, params)
                    .expect("genuine peer handshake");
            }
        }
        admit(network, &block, params);
        for peer in [1, 2] {
            network
                .receive_message(
                    peer,
                    WireNetworkMessage::CompactBlock(compact(&block, true)),
                    1,
                    ScriptVerifyFlags::NONE,
                    params,
                )
                .expect("matching compact request");
        }
        network
            .receive_message(
                3,
                WireNetworkMessage::CompactBlock(compact(&unrelated, true)),
                1,
                ScriptVerifyFlags::NONE,
                params,
            )
            .expect("unrelated compact request");
        assert_eq!(request_order(network, hash), vec![1, 2]);
        assert_eq!(request_order(network, unrelated_hash), vec![3]);

        // Act
        let outcome = network.connect_stored_block(
            &block,
            open_bitcoin_core::consensus::chain_proof::ProofWork::ZERO,
            i64::MAX,
            ScriptVerifyFlags::NONE,
            params,
        );

        // Assert
        assert!(
            matches!(
                outcome,
                Ok(crate::network::BlockConnectDisposition::Connected(_))
            ),
            "{outcome:?}"
        );
        let manager = network.chainstate();
        let core = manager.chainstate();
        let position = core.tip().expect("accepted tip");
        let header = network
            .peer_manager()
            .header_store()
            .entry(&hash)
            .expect("earned header");
        assert_eq!(position.block_hash, hash);
        assert_eq!(position.chain_work, header.chain_work);
        assert_eq!(core.coins_best_block().expect("Coins endpoint"), Some(hash));
        let undo = &core.undo_by_block()[&hash];
        assert!(undo.transactions.is_empty());
        assert_eq!(undo.overwrites.len(), 1);
        assert!(undo.overwrites[0].overwritten.is_empty());
        undo.validate_overwrite_binding(&block, position)
            .expect("exact undo binding");
        assert!(!store.has_undo(hash).expect("pre-flush durable undo"));
        assert_eq!(
            manager
                .validation_provenance(hash)
                .expect("live provenance"),
            Provenance::ScriptsValid
        );
        let snapshot = network.chainstate_snapshot().expect("complete typed UTXOs");
        let txid = open_bitcoin_core::consensus::transaction_txid(&block.transactions[0])
            .expect("coinbase txid");
        let outpoint = OutPoint { txid, vout: 0 };
        let expected = open_bitcoin_core::chainstate::Coin {
            output: block.transactions[0].outputs[0].clone(),
            is_coinbase: true,
            created_height: 1,
            created_median_time_past: i64::from(genesis.time),
        };
        assert_eq!(
            snapshot.utxos,
            std::collections::HashMap::from([(outpoint, expected)])
        );
        assert_eq!(
            store
                .validation_provenance(hash)
                .expect("durable provenance"),
            Provenance::ScriptsValid
        );
        assert_eq!(store.maybe_basic_filter_state().expect("BASIC state"), None);
        assert!(request_order(network, hash).is_empty());
        assert_eq!(request_order(network, unrelated_hash), vec![3]);
    });
}

#[test]
fn phase167_compact_stored_body_failed_ancestor_preserves_descendant_request() {
    with_owner(|network, store, params| {
        // Arrange
        let genesis = CanonicalChain::Regtest.spec().genesis_header();
        let ancestor = mined_child(&genesis, 1);
        let descendant = mined_child(&ancestor.header, 2);
        let descendant_hash = block_hash(&descendant.header);
        let unrelated = mined_child(&genesis, 3);
        let unrelated_hash = block_hash(&unrelated.header);
        compact_peer(network, params);
        admit(network, &ancestor, params);
        admit(network, &descendant, params);
        network
            .retain_checked_candidate(&ancestor, params, i64::MAX)
            .expect("real retained ancestor body");
        for body in [&descendant, &unrelated] {
            network
                .receive_message(
                    1,
                    WireNetworkMessage::CompactBlock(compact(body, true)),
                    1,
                    ScriptVerifyFlags::NONE,
                    params,
                )
                .expect("real compact request");
        }
        assert_eq!(request_order(network, descendant_hash), vec![1]);
        assert_eq!(request_order(network, unrelated_hash), vec![1]);
        network
            .peer_manager_mut()
            .mark_body_failed(block_hash(&ancestor.header));
        let before = accepted(network, store);

        // Act
        let outcome = network.connect_stored_block(
            &descendant,
            open_bitcoin_core::consensus::chain_proof::ProofWork::ZERO,
            i64::MAX,
            ScriptVerifyFlags::NONE,
            params,
        );

        // Assert
        assert!(
            matches!(
                outcome,
                Ok(crate::network::BlockConnectDisposition::NonExtending { .. })
            ),
            "{outcome:?}"
        );
        assert_eq!(accepted(network, store), before);
        assert!(
            network
                .eligible_body_candidates()
                .iter()
                .all(|facts| facts.block_hash != descendant_hash)
        );
        assert_eq!(request_order(network, descendant_hash), vec![1]);
        assert_eq!(request_order(network, unrelated_hash), vec![1]);
    });
}

#[test]
fn phase167_compact_stored_body_canonical_retained_sidebranch_finishes_matching_only() {
    with_owner(|network, store, params| {
        // Arrange
        let genesis = CanonicalChain::Regtest.spec().genesis_header();
        let active = mined_child(&genesis, 1);
        admit(network, &active, params);
        network
            .connect_stored_block(
                &active,
                open_bitcoin_core::consensus::chain_proof::ProofWork::ZERO,
                i64::MAX,
                ScriptVerifyFlags::NONE,
                params,
            )
            .expect("real accepted active child");
        let mut side = mined_child(&genesis, 1);
        side.transactions[0].lock_time += 1;
        side.header.merkle_root = block_merkle_root(&side.transactions)
            .expect("side merkle")
            .0;
        mine(&mut side.header);
        let hash = block_hash(&side.header);
        compact_peer(network, params);
        admit(network, &side, params);
        network
            .receive_message(
                1,
                WireNetworkMessage::CompactBlock(compact(&side, true)),
                1,
                ScriptVerifyFlags::NONE,
                params,
            )
            .expect("real side compact request");
        let unrelated = BlockHash::from_byte_array([0x55; 32]);
        assert!(
            network
                .peer_manager_mut()
                .request_missing_blocks(1, &[unrelated])
                .expect("unrelated request")
                .is_some()
        );
        assert_eq!(request_order(network, hash), vec![1]);
        let before = accepted(network, store);

        // Act
        let outcome = network.connect_stored_block(
            &side,
            open_bitcoin_core::consensus::chain_proof::ProofWork::ZERO,
            i64::MAX,
            ScriptVerifyFlags::NONE,
            params,
        );

        // Assert
        assert!(
            matches!(
                outcome,
                Ok(crate::network::BlockConnectDisposition::NonExtending { .. })
            ),
            "{outcome:?}"
        );
        assert_eq!(accepted(network, store), before);
        assert!(
            network
                .eligible_body_candidates()
                .iter()
                .any(|facts| facts.block_hash == hash)
        );
        assert!(request_order(network, hash).is_empty());
        assert_eq!(request_order(network, unrelated), vec![1]);
    });
}

#[test]
fn phase167_compact_stored_body_canonical_rejection_preserves_requests_and_accepted_state() {
    with_owner(|network, store, params| {
        // Arrange
        let mut bad = mined_child(&CanonicalChain::Regtest.spec().genesis_header(), 1);
        bad.transactions[0].outputs[0].value =
            Amount::from_sats(5_000_000_001).expect("actual overreward");
        bad.header.merkle_root = block_merkle_root(&bad.transactions).expect("bad merkle").0;
        mine(&mut bad.header);
        let hash = block_hash(&bad.header);
        compact_peer(network, params);
        admit(network, &bad, params);
        network
            .receive_message(
                1,
                WireNetworkMessage::CompactBlock(compact(&bad, true)),
                1,
                ScriptVerifyFlags::NONE,
                params,
            )
            .expect("real pending bad body request");
        let unrelated = BlockHash::from_byte_array([0x56; 32]);
        assert!(
            network
                .peer_manager_mut()
                .request_missing_blocks(1, &[unrelated])
                .expect("unrelated request")
                .is_some()
        );
        let before = accepted(network, store);

        // Act
        let result = network.connect_stored_block(
            &bad,
            open_bitcoin_core::consensus::chain_proof::ProofWork::ZERO,
            i64::MAX,
            ScriptVerifyFlags::NONE,
            params,
        );

        // Assert
        assert!(result.is_err(), "overreward cannot connect");
        assert_eq!(accepted(network, store), before);
        assert_eq!(request_order(network, hash), vec![1]);
        assert_eq!(request_order(network, unrelated), vec![1]);
        assert!(
            network
                .peer_manager()
                .header_store()
                .maybe_body_eligibility(hash)
                .is_some_and(|facts| facts.failed)
        );
    });
}

#[test]
fn phase167_compact_stored_body_header_only_cannot_finish_request() {
    with_owner(|network, store, params| {
        // Arrange
        let block = mined_child(&CanonicalChain::Regtest.spec().genesis_header(), 1);
        let hash = block_hash(&block.header);
        compact_peer(network, params);
        admit(network, &block, params);
        network
            .receive_message(
                1,
                WireNetworkMessage::CompactBlock(compact(&block, true)),
                1,
                ScriptVerifyFlags::NONE,
                params,
            )
            .expect("real compact request");
        let before = accepted(network, store);

        // Act
        let finished = network.finish_bookkeeping(&block);

        // Assert
        assert!(!finished);
        assert_eq!(accepted(network, store), before);
        assert_eq!(request_order(network, hash), vec![1]);
    });
}

#[test]
fn phase167_compact_stored_body_duplicate_uses_actual_retained_body() {
    with_owner(|network, store, params| {
        // Arrange
        let block = mined_child(&CanonicalChain::Regtest.spec().genesis_header(), 1);
        let hash = block_hash(&block.header);
        admit(network, &block, params);
        network
            .connect_stored_block(
                &block,
                open_bitcoin_core::consensus::chain_proof::ProofWork::ZERO,
                i64::MAX,
                ScriptVerifyFlags::NONE,
                params,
            )
            .expect("genuine first acceptance");
        compact_peer(network, params);
        network
            .receive_message(
                1,
                WireNetworkMessage::CompactBlock(compact(&block, true)),
                1,
                ScriptVerifyFlags::NONE,
                params,
            )
            .expect("real known-body compact request");
        let unrelated = BlockHash::from_byte_array([0x57; 32]);
        assert!(
            network
                .peer_manager_mut()
                .request_missing_blocks(1, &[unrelated])
                .expect("unrelated request")
                .is_some()
        );
        assert_eq!(request_order(network, hash), vec![1]);
        let before = accepted(network, store);

        // Act
        let outcome = network.connect_stored_block(
            &block,
            open_bitcoin_core::consensus::chain_proof::ProofWork::ZERO,
            i64::MAX,
            ScriptVerifyFlags::NONE,
            params,
        );

        // Assert
        assert!(
            matches!(outcome, Ok(crate::network::BlockConnectDisposition::Duplicate(actual)) if actual == hash),
            "{outcome:?}"
        );
        assert_eq!(accepted(network, store), before);
        assert!(request_order(network, hash).is_empty());
        assert_eq!(request_order(network, unrelated), vec![1]);
        assert_eq!(network.blocks_by_hash.get(&hash), Some(&block));
    });
}

#[test]
fn phase167_compact_stored_body_stale_epoch_and_unavailable_body_keep_request() {
    with_owner(|network, store, params| {
        // Arrange
        let block = mined_child(&CanonicalChain::Regtest.spec().genesis_header(), 1);
        let hash = block_hash(&block.header);
        compact_peer(network, params);
        admit(network, &block, params);
        network
            .retain_checked_candidate(&block, params, i64::MAX)
            .expect("real retained candidate");
        network
            .receive_message(
                1,
                WireNetworkMessage::CompactBlock(compact(&block, true)),
                1,
                ScriptVerifyFlags::NONE,
                params,
            )
            .expect("real compact request");
        let prior_epoch = network
            .peer_manager()
            .header_store()
            .body_eligibility_epoch();
        network.peer_manager_mut().mark_body_unavailable(hash);
        assert!(!prior_epoch.is_current());
        let before = accepted(network, store);

        // Act
        let finished = network.finish_bookkeeping(&block);

        // Assert
        assert!(!finished);
        assert_eq!(accepted(network, store), before);
        assert_eq!(request_order(network, hash), vec![1]);
        assert!(
            network
                .eligible_body_candidates()
                .iter()
                .all(|facts| facts.block_hash != hash)
        );
    });
}

#[test]
fn phase167_compact_stored_body_fixture_owner_refusal_keeps_request_and_state() {
    with_owner(|network, store, params| {
        // Arrange
        let block = mined_child(&CanonicalChain::Regtest.spec().genesis_header(), 1);
        let hash = block_hash(&block.header);
        compact_peer(network, params);
        admit(network, &block, params);
        network
            .receive_message(
                1,
                WireNetworkMessage::CompactBlock(compact(&block, true)),
                1,
                ScriptVerifyFlags::NONE,
                params,
            )
            .expect("real compact request");
        let before = accepted(network, store);
        let headers = network.peer_manager().header_store().clone();

        // Act
        let result = network.try_admit_fixture_headers(HeaderStore::default(), i64::MAX, params);
        let mut rows = headers.entries().cloned().collect::<Vec<_>>();
        rows.iter_mut()
            .find(|entry| entry.block_hash == hash)
            .expect("admitted child")
            .block_hash = BlockHash::from_byte_array([0x58; 32]);
        let imported = HeaderStore::from_entries(rows).expect("raw linked import");
        let import_result = network.try_seed_canonical_headers(imported);

        // Assert
        assert!(
            result.is_err(),
            "canonical owner cannot accept fixture reset"
        );
        assert!(
            import_result.is_err(),
            "forged imported identity cannot replace owner"
        );
        assert_eq!(accepted(network, store), before);
        assert_eq!(*network.peer_manager().header_store(), headers);
        assert_eq!(request_order(network, hash), vec![1]);
    });
}

#[test]
fn phase167_compact_stored_body_real_rejected_ancestor_keeps_descendant_request() {
    with_owner(|network, store, params| {
        // Arrange
        let mut bad = mined_child(&CanonicalChain::Regtest.spec().genesis_header(), 1);
        bad.transactions[0].outputs[0].value =
            Amount::from_sats(5_000_000_001).expect("overreward");
        bad.header.merkle_root = block_merkle_root(&bad.transactions).expect("merkle").0;
        mine(&mut bad.header);
        compact_peer(network, params);
        admit(network, &bad, params);
        let bad_result = network.connect_stored_block(
            &bad,
            open_bitcoin_core::consensus::chain_proof::ProofWork::ZERO,
            i64::MAX,
            ScriptVerifyFlags::NONE,
            params,
        );
        assert!(bad_result.is_err(), "actual Coins rejection");
        assert!(
            network
                .peer_manager()
                .header_store()
                .maybe_body_eligibility(block_hash(&bad.header))
                .is_some_and(|facts| facts.failed)
        );
        let child = mined_child(&bad.header, 2);
        let hash = block_hash(&child.header);
        admit(network, &child, params);
        network
            .receive_message(
                1,
                WireNetworkMessage::CompactBlock(compact(&child, true)),
                1,
                ScriptVerifyFlags::NONE,
                params,
            )
            .expect("real descendant compact request");
        let before = accepted(network, store);

        // Act
        let outcome = network.connect_stored_block(
            &child,
            open_bitcoin_core::consensus::chain_proof::ProofWork::ZERO,
            i64::MAX,
            ScriptVerifyFlags::NONE,
            params,
        );

        // Assert
        assert!(!matches!(
            outcome,
            Ok(crate::network::BlockConnectDisposition::Connected(_))
        ));
        assert_eq!(accepted(network, store), before);
        assert_eq!(request_order(network, hash), vec![1]);
        assert!(
            network
                .eligible_body_candidates()
                .iter()
                .all(|facts| facts.block_hash != hash)
        );
    });
}
