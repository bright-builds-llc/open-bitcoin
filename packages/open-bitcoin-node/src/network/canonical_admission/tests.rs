// Parity breadcrumbs:
// - packages/bitcoin-knots/src/kernel/chainparams.cpp
// - packages/bitcoin-knots/src/validation.cpp

use crate::{ManagedNetworkHandle, ManagedPeerNetwork, MemoryChainstateStore};
use open_bitcoin_core::consensus::CanonicalChain;
use open_bitcoin_mempool::PolicyConfig;

const CHAINS: [CanonicalChain; 4] = [
    CanonicalChain::Mainnet,
    CanonicalChain::Testnet3,
    CanonicalChain::Signet,
    CanonicalChain::Regtest,
];

fn fixture() -> ManagedPeerNetwork<MemoryChainstateStore> {
    ManagedPeerNetwork::new(
        MemoryChainstateStore::default(),
        Default::default(),
        PolicyConfig::default(),
    )
}

#[test]
fn canonical_chain_owner_fixture_has_no_binding() {
    // Arrange
    let handle = ManagedNetworkHandle::from_network_fixture(fixture());
    // Act
    let actual = handle.maybe_canonical_chain().expect("scope query");
    // Assert
    assert_eq!(actual, None);
}

#[test]
fn canonical_chain_owner_preserves_first_binding_across_clones_and_wrapping() {
    for chain in CHAINS {
        // Arrange
        let network = fixture().into_canonical_scope(chain);
        assert_eq!(
            ManagedNetworkHandle::from_network_fixture(network.clone())
                .maybe_canonical_chain()
                .expect("fixture wrapper preserves binding"),
            Some(chain)
        );
        let cloned = network
            .clone()
            .into_canonical_scope(CanonicalChain::Regtest);
        let handle = ManagedNetworkHandle::from_canonical_network(cloned, CanonicalChain::Regtest);
        // Act
        let actual = handle.clone().maybe_canonical_chain().expect("scope query");
        // Assert
        assert_eq!(actual, Some(chain));
    }
}

use crate::chainstate::validation_history::tests::block;
use open_bitcoin_core::{
    chainstate::AnchoredBlock,
    consensus::{ConsensusParams, ScriptVerifyFlags, block_hash},
    primitives::BlockHash,
};
use open_bitcoin_network::{HeaderEntry, HeaderStore, HeadersMessage};

fn snapshot<S: crate::ChainstateStore, V: open_bitcoin_core::chainstate::CoinsView>(
    network: &ManagedPeerNetwork<S, V>,
) -> String {
    format!(
        "{:?}|{:?}|{:?}|{:?}|{:?}|{:?}|{:?}|{:?}|{:?}|{:?}|{:?}|{:?}",
        network.chainstate_snapshot().expect("chain snapshot"),
        network.peer_manager,
        network.mempool.mempool(),
        network.blocks_by_hash,
        network.transactions_by_txid,
        network.transactions_by_wtxid,
        network.lifecycle_evidence,
        network.checkpoint_evidence,
        network.peer_effect_ledger,
        network.snapshot_effect_ledger,
        network.have_bytes_accumulator,
        network.block_relay_evidence
    )
}

fn error_code(error: crate::ManagedNetworkError) -> String {
    let crate::ManagedNetworkError::Network(open_bitcoin_network::NetworkError::InvalidHeader {
        reject_reason,
        ..
    }) = error
    else {
        panic!("expected canonical refusal, got {error}")
    };
    reject_reason
}

#[test]
fn canonical_chain_header_exact_and_foreign_roots_preserve_whole_owner() {
    for chain in CHAINS {
        for other in CHAINS {
            for seeded in [false, true] {
                // Arrange
                let mut network = fixture();
                if seeded {
                    let mut headers = HeaderStore::default();
                    headers
                        .insert_header(chain.spec().genesis_header())
                        .expect("raw carrier");
                    network.seed_header_store(headers);
                }
                let mut network = network.into_canonical_scope(chain);
                let before = snapshot(&network);
                // Act
                let result = network.handle_validated_headers(
                    1,
                    HeadersMessage {
                        headers: vec![other.spec().genesis_header()],
                    },
                    i64::MAX,
                    ConsensusParams::default(),
                );
                // Assert
                let expected = if chain == other {
                    "unsupported-validation-scope"
                } else {
                    "noncanonical-genesis-header"
                };
                assert_eq!(error_code(result.expect_err("canonical refusal")), expected);
                assert_eq!(snapshot(&network), before);
            }
        }
    }
}

#[test]
fn canonical_chain_header_batch_never_publishes_prefix() {
    for chain in CHAINS {
        // Arrange
        let mut network = fixture().into_canonical_scope(chain);
        let before = snapshot(&network);
        let child = block(chain.spec().genesis_hash(), 1, 0x51);
        // Act
        let result = network.handle_validated_headers(
            1,
            HeadersMessage {
                headers: vec![chain.spec().genesis_header(), child.header],
            },
            i64::MAX,
            ConsensusParams::default(),
        );
        // Assert
        assert_eq!(
            error_code(result.expect_err("scope refusal")),
            "unsupported-validation-scope"
        );
        assert_eq!(snapshot(&network), before);
    }
}

#[test]
fn canonical_chain_header_imports_and_fixture_seed_cannot_change_bound_store() {
    for chain in CHAINS {
        for other in CHAINS {
            // Arrange
            let header = other.spec().genesis_header();
            let entries = [HeaderEntry {
                block_hash: block_hash(&header),
                header,
                height: 0,
                chain_work: u128::MAX,
            }];
            let headers = HeaderStore::from_entries(entries).expect("forged raw work carrier");
            let mut network = fixture().into_canonical_scope(chain);
            let before = snapshot(&network);
            // Act
            let result = network.try_seed_canonical_headers(headers.clone());
            network.seed_header_store(headers);
            // Assert
            let expected = if chain == other {
                "unsupported-validation-scope"
            } else {
                "noncanonical-genesis-header"
            };
            assert_eq!(error_code(result.expect_err("import refusal")), expected);
            assert_eq!(snapshot(&network), before);
        }
    }
}

#[test]
fn canonical_chain_body_all_roots_and_altered_body_preserve_whole_owner() {
    for chain in CHAINS {
        for other in CHAINS {
            for altered in [false, true] {
                // Arrange
                let mut candidate = other.spec().genesis_block();
                if altered {
                    candidate.transactions.clear();
                }
                let mut network = fixture().into_canonical_scope(chain);
                let before = snapshot(&network);
                // Act
                let local = network.connect_local_block(
                    &candidate,
                    ScriptVerifyFlags::P2SH,
                    ConsensusParams::default(),
                );
                let stored = network.connect_stored_block(
                    &candidate,
                    u128::MAX,
                    i64::MAX,
                    ScriptVerifyFlags::P2SH,
                    ConsensusParams::default(),
                );
                let replacement = [AnchoredBlock {
                    block: candidate.clone(),
                    chain_work: u128::MAX,
                }];
                let reorg = network.reorg_to_branch(
                    &[],
                    &replacement,
                    open_bitcoin_mempool::ReorgLifecycleContext::new(
                        open_bitcoin_mempool::PolicyTime::from_unix_seconds(0),
                    ),
                    ScriptVerifyFlags::P2SH,
                    ConsensusParams::default(),
                );
                // Assert
                let expected = if chain != other {
                    "noncanonical-genesis-header"
                } else if altered {
                    "noncanonical-genesis-body"
                } else {
                    "unsupported-validation-scope"
                };
                assert_eq!(error_code(local.expect_err("local refusal")), expected);
                assert_eq!(error_code(stored.expect_err("stored refusal")), expected);
                assert_eq!(error_code(reorg.expect_err("reorg refusal")), expected);
                assert_eq!(snapshot(&network), before);
            }
        }
    }
}

#[test]
fn canonical_chain_body_cached_dispositions_and_reorg_refuse_before_effects() {
    for chain in CHAINS {
        for populated in [false, true] {
            // Arrange: lower-level synthetic fixture history only.
            let genesis = block(BlockHash::default(), 0, 0x51);
            let child = block(block_hash(&genesis.header), 1, 0x51);
            let foreign_child = block(BlockHash::from_byte_array([42; 32]), 1, 0x52);
            let mut network = fixture();
            if populated {
                network
                    .connect_local_block(
                        &genesis,
                        ScriptVerifyFlags::P2SH,
                        ConsensusParams::default(),
                    )
                    .expect("synthetic fixture root");
                network
                    .connect_local_block(
                        &child,
                        ScriptVerifyFlags::P2SH,
                        ConsensusParams::default(),
                    )
                    .expect("synthetic fixture child");
            }
            let mut network = network.into_canonical_scope(chain);
            let before = snapshot(&network);
            for candidate in [&child, &foreign_child] {
                // Act
                let local = network.connect_local_block(
                    candidate,
                    ScriptVerifyFlags::P2SH,
                    ConsensusParams::default(),
                );
                let result = network.connect_stored_block(
                    candidate,
                    u128::MAX,
                    i64::MAX,
                    ScriptVerifyFlags::P2SH,
                    ConsensusParams::default(),
                );
                // Assert: Duplicate, NonExtending and Disconnected cannot cache.
                assert_eq!(
                    error_code(local.expect_err("local scope refusal")),
                    "unsupported-validation-scope"
                );
                assert_eq!(
                    error_code(result.expect_err("scope refusal")),
                    "unsupported-validation-scope"
                );
                assert_eq!(snapshot(&network), before);
            }
            let replacement = [AnchoredBlock {
                block: child.clone(),
                chain_work: u128::MAX,
            }];
            let result = network.reorg_to_branch(
                &[child],
                &replacement,
                open_bitcoin_mempool::ReorgLifecycleContext::new(
                    open_bitcoin_mempool::PolicyTime::from_unix_seconds(0),
                ),
                ScriptVerifyFlags::P2SH,
                ConsensusParams::default(),
            );
            assert_eq!(
                error_code(result.expect_err("replacement refusal")),
                "unsupported-validation-scope"
            );
            assert_eq!(snapshot(&network), before);
        }
    }
}

mod fixtures;
use fixtures::DurableFixture;

#[test]
fn canonical_chain_owner_ordinary_startup_preserves_all_namespace_bytes() {
    use crate::{
        chainstate::BasicFilterStartupMode,
        sync::{DurableSyncRuntime, SyncNetwork, SyncRuntimeConfig},
    };
    for network in [
        SyncNetwork::Mainnet,
        SyncNetwork::Testnet,
        SyncNetwork::Signet,
        SyncNetwork::Regtest,
    ] {
        for mode in [
            BasicFilterStartupMode::Enabled,
            BasicFilterStartupMode::Disabled,
            BasicFilterStartupMode::PreserveSaved,
        ] {
            // Arrange: pending recovery/prune and malformed BASIC bytes must not be inspected.
            let fixture = DurableFixture::new();
            fixture
                .store
                .write_raw_for_test(crate::StorageNamespace::Coins, "H", vec![0xff])
                .expect("pending coins sentinel");
            fixture
                .store
                .write_raw_for_test(
                    crate::StorageNamespace::BlockIndex,
                    "prune_intent",
                    vec![0xff],
                )
                .expect("prune sentinel");
            fixture
                .store
                .write_raw_for_test(
                    crate::StorageNamespace::BlockIndex,
                    crate::storage::filter_index::STATE_KEY,
                    vec![0xff],
                )
                .expect("BASIC sentinel");
            let before = fixture.bytes();
            // Act
            let result = DurableSyncRuntime::open_ordinary_configured(
                fixture.store.clone(),
                SyncRuntimeConfig {
                    network,
                    ..Default::default()
                },
                mode,
            );
            // Assert
            let Err(error) = result else {
                panic!("ordinary startup unsupported")
            };
            assert!(
                error
                    .to_string()
                    .contains("ordinary history validation is unsupported")
            );
            if network == SyncNetwork::Signet {
                assert!(error.to_string().contains("signet_challenge_validation"));
            }
            assert_eq!(fixture.bytes(), before);
            fixture.cleanup();
        }
    }
}

#[test]
fn canonical_chain_header_empty_nonroot_and_multiple_root_imports_refuse() {
    for chain in CHAINS {
        let mut network = fixture().into_canonical_scope(chain);
        let child = block(chain.spec().genesis_hash(), 1, 0x51);
        let arbitrary = block(BlockHash::default(), 0, 0x52);
        let before = snapshot(&network);
        for (headers, expected) in [
            (vec![], "unsupported-validation-scope"),
            (vec![child.header], "unsupported-validation-scope"),
            (vec![arbitrary.header], "noncanonical-genesis-header"),
        ] {
            // Act
            let result = network.handle_validated_headers(
                1,
                HeadersMessage { headers },
                i64::MAX,
                ConsensusParams::default(),
            );
            // Assert
            assert_eq!(error_code(result.expect_err("refusal")), expected);
            assert_eq!(snapshot(&network), before);
        }
        for headers in [
            HeaderStore::default(),
            HeaderStore::from_entries(CHAINS.map(|other| {
                let header = other.spec().genesis_header();
                HeaderEntry {
                    block_hash: block_hash(&header),
                    header,
                    height: 0,
                    chain_work: u128::MAX,
                }
            }))
            .expect("multiple raw roots"),
        ] {
            assert!(network.try_seed_canonical_headers(headers).is_err());
            assert_eq!(snapshot(&network), before);
        }
    }
}

#[test]
fn canonical_chain_body_general_sync_and_durable_dispatch_preserve_peer_state() {
    use open_bitcoin_network::WireNetworkMessage;
    for chain in CHAINS {
        for message in [
            WireNetworkMessage::Headers(HeadersMessage {
                headers: vec![chain.spec().genesis_header()],
            }),
            WireNetworkMessage::Block(chain.spec().genesis_block()),
            WireNetworkMessage::CompactBlock(open_bitcoin_codec::CompactBlockPayload {
                header: chain.spec().genesis_header(),
                nonce: 0,
                short_ids: vec![],
                prefilled_transactions: vec![],
            }),
            WireNetworkMessage::BlockTxn(open_bitcoin_codec::BlockTransactions {
                block_hash: chain.spec().genesis_hash(),
                transactions: vec![],
            }),
        ] {
            // Arrange
            let mut network = fixture();
            network
                .connect_outbound_peer(1, 0)
                .expect("fixture session");
            fixtures::arm_compact_download(&mut network, &chain.spec().genesis_block());
            let mut network = network.into_canonical_scope(chain);
            let before = snapshot(&network);
            // Act
            let general = network.receive_message(
                1,
                message.clone(),
                i64::MAX,
                ScriptVerifyFlags::P2SH,
                ConsensusParams::default(),
            );
            let sync = network.receive_sync_message(
                1,
                message.clone(),
                i64::MAX,
                ScriptVerifyFlags::P2SH,
                ConsensusParams::default(),
            );
            let durable = network.receive_message_for_durable_serving(
                1,
                message,
                i64::MAX,
                ScriptVerifyFlags::P2SH,
                ConsensusParams::default(),
                |_| true,
            );
            // Assert
            for result in [general, sync, durable] {
                assert_eq!(
                    error_code(result.expect_err("dispatch refusal")),
                    "unsupported-validation-scope"
                );
            }
            assert_eq!(snapshot(&network), before);
        }
    }
}

#[test]
fn canonical_chain_body_received_action_preserves_pending_compact_slot() {
    for chain in CHAINS {
        // Arrange: owned lower-level volatile fixture, never ordinary authority.
        let candidate = chain.spec().genesis_block();
        let mut network = fixture();
        network
            .connect_outbound_peer(1, 0)
            .expect("fixture session");
        fixtures::arm_compact_download(&mut network, &candidate);
        let mut network = network.into_canonical_scope(chain);
        let before = snapshot(&network);
        // Act
        let result = network.process_actions(
            1,
            vec![open_bitcoin_network::PeerAction::ReceivedBlock(candidate)],
            i64::MAX,
            ScriptVerifyFlags::P2SH,
            ConsensusParams::default(),
            super::super::action_translation::immediate_inventory_serving(),
        );
        // Assert
        assert_eq!(
            error_code(result.expect_err("refusal before cleanup")),
            "unsupported-validation-scope"
        );
        assert_eq!(snapshot(&network), before);
        assert_eq!(
            network
                .peer_manager
                .compact_download_peer_state(1)
                .expect("slot map")
                .in_flight
                .len(),
            1
        );
    }
}

#[test]
fn canonical_chain_header_durable_receipts_and_complete_coverage_remain_unchanged() {
    for chain in CHAINS {
        // Arrange
        let fixture = DurableFixture::new();
        let mut network = fixture.network().into_canonical_scope(chain);
        let before = fixture.bytes();
        let before_owner = snapshot(&network);
        // Act
        let result = network.handle_validated_headers(
            1,
            HeadersMessage {
                headers: vec![chain.spec().genesis_header()],
            },
            i64::MAX,
            ConsensusParams::default(),
        );
        // Assert
        assert_eq!(
            error_code(result.expect_err("unsupported")),
            "unsupported-validation-scope"
        );
        assert_eq!(fixture.bytes(), before);
        assert_eq!(snapshot(&network), before_owner);
        assert!(
            fixture
                .store
                .recovered_validation_history()
                .expect("coverage")
                .is_complete()
                .expect("complete fixture coverage")
        );
        assert!(
            fixture
                .store
                .maybe_validation_history_record(chain.spec().genesis_hash())
                .expect("receipt")
                .is_none()
        );
        drop(network);
        fixture.cleanup();
    }
}

#[test]
fn canonical_chain_body_durable_receipts_coins_filters_and_prune_remain_unchanged() {
    for chain in CHAINS {
        // Arrange
        let fixture = DurableFixture::new();
        let mut network = fixture.network().into_canonical_scope(chain);
        let before = fixture.bytes();
        let before_owner = snapshot(&network);
        // Act
        let result = network.connect_stored_block(
            &chain.spec().genesis_block(),
            u128::MAX,
            i64::MAX,
            ScriptVerifyFlags::P2SH,
            ConsensusParams::default(),
        );
        // Assert
        assert_eq!(
            error_code(result.expect_err("unsupported")),
            "unsupported-validation-scope"
        );
        assert_eq!(fixture.bytes(), before);
        assert_eq!(snapshot(&network), before_owner);
        assert!(network.maybe_chain_tip().is_none());
        drop(network);
        fixture.cleanup();
    }
}
