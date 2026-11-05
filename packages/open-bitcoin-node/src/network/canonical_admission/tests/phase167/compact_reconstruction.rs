// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/test/functional/p2p_compactblocks.py

use super::*;

#[test]
fn phase167_compact_header_unknown_valid_header_admits_and_reconstructs() {
    with_owner(|network, store, params| {
        // Arrange
        let block = mined_child(&CanonicalChain::Regtest.spec().genesis_header(), 1);
        compact_peer(network, params);
        let hash = block_hash(&block.header);
        let before = accepted(network, store);
        assert!(
            !network
                .peer_manager
                .header_store()
                .is_canonically_admitted(hash)
        );

        // Act
        let result = network.receive_message(
            1,
            WireNetworkMessage::CompactBlock(compact(&block, false)),
            1,
            ScriptVerifyFlags::NONE,
            params,
        );

        // Assert
        let outcome = result.expect("valid unknown compact header admits and reconstructs");
        assert!(
            network
                .peer_manager
                .header_store()
                .is_canonically_admitted(hash)
        );
        assert!(matches!(
            outcome.maybe_block_disposition,
            Some(crate::network::BlockConnectDisposition::Connected(_))
        ));
        assert_ne!(accepted(network, store), before);
        assert_eq!(
            network
                .chainstate
                .chainstate()
                .tip()
                .expect("accepted")
                .block_hash,
            hash
        );
    });
}

#[test]
fn phase167_canonical_admission_compact_reconstruction_mismatch_never_absorbs() {
    with_mode(BasicFilterStartupMode::Enabled, |network, store, params| {
        // Arrange
        let block = mined_child(&CanonicalChain::Regtest.spec().genesis_header(), 1);
        compact_peer(network, params);
        admit(network, &block, params);
        let budget = crate::chainstate::basic_filter_turn_budget().expect("BASIC budget");
        network
            .chainstate
            .initialize_basic_index_owner(budget.normal())
            .expect("BASIC owner");
        let before = accepted(network, store);
        let before_progress = network.chainstate.maybe_basic_index_progress();
        assert!(before_progress.is_some(), "enabled BASIC progress");
        let facts = CompactBlockReceiveFacts {
            candidates: &[],
            extra: &[],
        };
        let refusal = network.peer_manager.handle_fixture_compact_block_download(
            1,
            compact(&block, true),
            facts,
            1,
            false,
        );
        assert!(
            matches!(refusal, Err(NetworkError::InvalidHeader { reject_reason, .. })
                if reject_reason == "canonical-owner-refuses-fixture-compact")
        );
        network
            .receive_message(
                1,
                WireNetworkMessage::CompactBlock(compact(&block, true)),
                1,
                ScriptVerifyFlags::NONE,
                params,
            )
            .expect("real pending compact download");
        let mut wrong = block.transactions.clone();
        wrong[0].outputs[0].value = Amount::from_sats(1).expect("different transaction");
        // Act
        let outcome = network
            .receive_message(
                1,
                WireNetworkMessage::BlockTxn(open_bitcoin_codec::BlockTransactions {
                    block_hash: block_hash(&block.header),
                    transactions: wrong,
                }),
                1,
                ScriptVerifyFlags::NONE,
                params,
            )
            .expect("mismatched reconstruction requests full block");
        // Assert
        assert!(outcome.maybe_block_disposition.is_none());
        assert!(
            matches!(outcome.outbound.as_slice(), [WireNetworkMessage::GetData(list)]
            if list.inventory.len() == 1
                && list.inventory[0].inventory_type == open_bitcoin_core::primitives::InventoryType::WitnessBlock
                && list.inventory[0].object_hash == block_hash(&block.header).into())
        );
        let after_progress = network.chainstate.maybe_basic_index_progress();
        assert_eq!(after_progress, before_progress);
        assert_unaccepted(network, store, &block, &before);
    });
}

#[test]
fn phase167_compact_header_guard_passing_unknown_peer_precedes_header_effects() {
    with_owner(|network, store, params| {
        // Arrange
        let block = mined_child(&CanonicalChain::Regtest.spec().genesis_header(), 1);
        let hash = block_hash(&block.header);
        compact_peer(network, params);
        let before = accepted(network, store);
        let snapshot_attempts = store.snapshot_attempts();

        // Act
        let result = network.receive_message(
            2,
            WireNetworkMessage::CompactBlock(compact(&block, false)),
            1,
            ScriptVerifyFlags::NONE,
            params,
        );

        // Assert
        assert!(matches!(
            &result,
            Err(crate::ManagedNetworkError::Network(
                NetworkError::UnknownPeer(2)
            ))
        ));
        assert_unaccepted(network, store, &block, &before);
        assert_eq!(store.snapshot_attempts(), snapshot_attempts);
        assert!(
            !network
                .peer_manager
                .header_store()
                .is_canonically_admitted(hash)
        );
        assert!(
            network
                .peer_manager
                .block_request_peer_order(hash)
                .is_empty()
        );
        assert!(
            store
                .maybe_validation_history_record(hash)
                .expect("KnownOnly row")
                .is_none()
        );
    });
}

#[test]
fn phase167_compact_header_predispatch_guard_keeps_malformed_and_owner_error_order() {
    for owner_mismatch in [false, true] {
        with_owner(|network, store, params| {
            // Arrange
            let mut block = mined_child(&CanonicalChain::Regtest.spec().genesis_header(), 1);
            compact_peer(network, params);
            if owner_mismatch {
                network
                    .peer_manager
                    .seed_header_store(HeaderStore::default());
            } else {
                block.header.previous_block_hash = BlockHash::from_byte_array([0; 32]);
            }
            let hash = block_hash(&block.header);
            let before = accepted(network, store);
            let snapshot_attempts = store.snapshot_attempts();

            // Act
            let result = network.receive_message(
                2,
                WireNetworkMessage::CompactBlock(compact(&block, false)),
                1,
                ScriptVerifyFlags::NONE,
                params,
            );

            // Assert
            assert!(
                matches!(
                    &result,
                    Err(crate::ManagedNetworkError::Network(
                        NetworkError::InvalidHeader { .. }
                    ))
                ),
                "guard must reject before UnknownPeer: {result:?}"
            );
            assert_unaccepted(network, store, &block, &before);
            assert_eq!(store.snapshot_attempts(), snapshot_attempts);
            assert!(
                network
                    .peer_manager
                    .block_request_peer_order(hash)
                    .is_empty()
            );
            assert!(
                store
                    .maybe_validation_history_record(hash)
                    .expect("KnownOnly row")
                    .is_none()
            );
        });
    }
}

#[test]
fn phase167_compact_header_disabled_and_incapable_peers_have_no_header_effect() {
    for disabled in [false, true] {
        with_owner(|network, store, params| {
            // Arrange
            let block = mined_child(&CanonicalChain::Regtest.spec().genesis_header(), 1);
            let hash = block_hash(&block.header);
            if disabled {
                compact_peer(network, params);
                network
                    .peer_manager
                    .set_block_relay_activation_policy(Default::default());
            } else {
                network.peer_manager.set_block_relay_activation_policy(
                    open_bitcoin_network::BlockRelayActivationPolicy {
                        compact_relay: open_bitcoin_network::CompactRelayActivationConfig {
                            enabled: true,
                        },
                        ..Default::default()
                    },
                );
            }
            let before = accepted(network, store);
            let snapshot_attempts = store.snapshot_attempts();

            // Act
            let result = network
                .receive_message(
                    1,
                    WireNetworkMessage::CompactBlock(compact(&block, false)),
                    1,
                    ScriptVerifyFlags::NONE,
                    params,
                )
                .expect("silent disabled or incapable receive");

            // Assert
            assert!(result.outbound.is_empty());
            assert!(result.maybe_block_disposition.is_none());
            assert_unaccepted(network, store, &block, &before);
            assert_eq!(store.snapshot_attempts(), snapshot_attempts);
            assert!(
                !network
                    .peer_manager
                    .header_store()
                    .is_canonically_admitted(hash)
            );
            assert!(
                network
                    .peer_manager
                    .block_request_peer_order(hash)
                    .is_empty()
            );
            assert!(
                store
                    .maybe_validation_history_record(hash)
                    .expect("KnownOnly row")
                    .is_none()
            );
        });
    }
}

#[test]
fn phase167_compact_header_invalid_context_refuses_before_request_enrollment() {
    for fault in 0..6 {
        with_mode(BasicFilterStartupMode::Enabled, |network, store, params| {
            // Arrange
            let genesis = CanonicalChain::Regtest.spec().genesis_header();
            let mut block = mined_child(&genesis, 1);
            compact_peer(network, params);
            let budget = crate::chainstate::basic_filter_turn_budget().expect("BASIC budget");
            network
                .chainstate
                .initialize_basic_index_owner(budget.normal())
                .expect("BASIC owner");
            let expected = match fault {
                0 => {
                    block.header.previous_block_hash = BlockHash::from_byte_array([77; 32]);
                    mine(&mut block.header);
                    "unproved-canonical-header-parent"
                }
                1 => {
                    block.header.bits -= 1;
                    mine(&mut block.header);
                    "bad-diffbits"
                }
                2 => {
                    block.header.nonce = (0..1_000)
                        .find(|nonce| {
                            block.header.nonce = *nonce;
                            open_bitcoin_core::consensus::check_block_header(&block.header).is_err()
                        })
                        .expect("genuine high-hash nonce");
                    "high-hash"
                }
                3 => {
                    block.header.version = 0;
                    mine(&mut block.header);
                    "bad-version(0x00000000)"
                }
                4 => {
                    block.header.time = genesis.time;
                    mine(&mut block.header);
                    "time-too-old"
                }
                _ => {
                    let now = std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .expect("shell clock")
                        .as_secs();
                    block.header.time = u32::try_from(now + 7_201).expect("future fits u32");
                    mine(&mut block.header);
                    "time-too-new"
                }
            };
            let hash = block_hash(&block.header);
            let before = accepted(network, store);
            let snapshot_attempts = store.snapshot_attempts();

            // Act
            let result = network.receive_message(
                1,
                WireNetworkMessage::CompactBlock(compact(&block, false)),
                1,
                ScriptVerifyFlags::NONE,
                params,
            );

            // Assert
            let Err(crate::ManagedNetworkError::Network(NetworkError::InvalidHeader {
                reject_reason,
                ..
            })) = result
            else {
                panic!("fault {fault} must be typed header refusal: {result:?}");
            };
            assert_eq!(reject_reason, expected, "fault {fault}");
            assert_unaccepted(network, store, &block, &before);
            assert_eq!(store.snapshot_attempts(), snapshot_attempts);
            assert!(
                !network
                    .peer_manager
                    .header_store()
                    .is_canonically_admitted(hash)
            );
            assert!(
                network
                    .peer_manager
                    .block_request_peer_order(hash)
                    .is_empty()
            );
            assert!(
                store
                    .maybe_validation_history_record(hash)
                    .expect("KnownOnly row")
                    .is_none()
            );
        });
    }
}
