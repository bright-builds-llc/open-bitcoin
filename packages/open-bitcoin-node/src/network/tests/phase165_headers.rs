// Parity breadcrumbs:
// - packages/bitcoin-knots/src/pow.cpp
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp

use super::*;
use crate::ManagedNetworkHandle;
use open_bitcoin_core::consensus::chain_proof::block_proof;
use open_bitcoin_network::{HeaderEntry, HeaderStore, HeadersMessage};

fn params() -> ConsensusParams {
    ConsensusParams {
        no_pow_retargeting: false,
        pow_target_timespan_seconds: 1_200,
        ..consensus_params()
    }
}

fn header(parent: BlockHash, time: u32, bits: u32, marker: u8) -> BlockHeader {
    let mut candidate = BlockHeader {
        previous_block_hash: parent,
        time,
        bits,
        merkle_root: Hash32::from_byte_array([marker; 32]).into(),
        ..BlockHeader::default()
    };
    candidate.nonce = (0..=u32::MAX)
        .find(|nonce| {
            candidate.nonce = *nonce;
            check_block_header(&candidate).is_ok()
        })
        .expect("cheap fixture mining");
    candidate
}

fn fixture() -> ManagedPeerNetwork<MemoryChainstateStore> {
    let mut network = ManagedPeerNetwork::new(
        MemoryChainstateStore::default(),
        local_config(42),
        PolicyConfig::default(),
    );
    network.add_inbound_peer(1).expect("fixture peer");
    network
        .receive_message(
            1,
            WireNetworkMessage::Version(open_bitcoin_network::VersionMessage {
                start_height: 100,
                nonce: 99,
                ..Default::default()
            }),
            100_000,
            verify_flags(),
            params(),
        )
        .expect("fixture version");
    network
}

fn dispatch(
    network: &mut ManagedPeerNetwork<MemoryChainstateStore>,
    headers: Vec<BlockHeader>,
    params: ConsensusParams,
    route: usize,
) -> Result<crate::network::ManagedSyncMessageResult, crate::ManagedNetworkError> {
    let message = WireNetworkMessage::Headers(HeadersMessage { headers });
    match route {
        0 => network.receive_sync_message(1, message, 100_000, verify_flags(), params),
        1 => network.receive_message(1, message, 100_000, verify_flags(), params),
        _ => network.receive_message_for_durable_serving(
            1,
            message,
            100_000,
            verify_flags(),
            params,
            |_| false,
        ),
    }
}

fn initialize_root(
    network: &mut ManagedPeerNetwork<MemoryChainstateStore>,
    root: &BlockHeader,
    params: ConsensusParams,
) {
    let mut headers = HeaderStore::default();
    headers
        .insert_header_with_params(root.clone(), &params)
        .expect("actual fixture root bytes");
    network
        .try_admit_fixture_headers(headers, 100_000, params)
        .expect("controlled fixture initialization");
}

#[test]
fn phase165_headers_numerical_body_helper_uses_target_and_propagates_invalid_compact() {
    // Arrange
    let network = fixture();
    let mut incoming = header(BlockHash::default(), 1_000, EASY_BITS, 98);
    let before = snapshot(&network);
    // Act
    let value = network
        .next_chain_work(&incoming)
        .expect("numerical own proof");
    incoming.bits = 0x201f_ffff;
    let harder = network
        .next_chain_work(&incoming)
        .expect("harder numerical target");
    incoming.bits = 0x0100_3456;
    let invalid = network.next_chain_work(&incoming);
    // Assert
    assert_eq!(value.to_le_bytes()[0], 2);
    assert!(value.to_le_bytes()[1..].iter().all(|byte| *byte == 0));
    assert_eq!(harder.to_le_bytes()[0], 8);
    assert!(invalid.is_err());
    assert_eq!(snapshot(&network), before);
    assert!(network.best_chain_entries().is_empty());
}

#[test]
fn phase165_headers_all_dispatch_routes_enforce_selected_limit_unchanged() {
    for route in 0..3 {
        // Arrange
        let mut network = fixture();
        let selected = ConsensusParams {
            pow_limit_bits: 0x201f_ffff,
            ..params()
        };
        let root = header(BlockHash::default(), 1_000, selected.pow_limit_bits, 0);
        initialize_root(&mut network, &root, selected);
        let candidate = header(block_hash(&root), 1_001, EASY_BITS, 1);
        let before = snapshot(&network);
        // Act
        let result = dispatch(&mut network, vec![candidate], selected, route);
        // Assert
        assert!(result.is_err(), "route {route} must apply selected limit");
        assert_eq!(snapshot(&network), before);
    }
}

#[test]
fn phase165_headers_shorter_stronger_branch_drives_owner_and_handle_locator() {
    // Arrange
    let mut network = fixture();
    let root = header(BlockHash::default(), 1_000, EASY_BITS, 2);
    initialize_root(&mut network, &root, params());
    let hard_parent = header(block_hash(&root), 1_001, EASY_BITS, 8);
    let hard_base = header(block_hash(&hard_parent), 1_002, 0x201f_ffff, 9);
    dispatch(
        &mut network,
        vec![hard_parent.clone(), hard_base.clone()],
        params(),
        0,
    )
    .expect("legal hard retarget base");
    let mut easy = hard_base.clone();
    for index in 0..3 {
        easy = header(block_hash(&easy), easy.time + 1_201, EASY_BITS, index + 3);
        dispatch(&mut network, vec![easy.clone()], params(), 0).expect("delayed easy branch");
    }
    let hard = header(block_hash(&hard_base), 1_003, hard_base.bits, 10);
    // Act
    let result = dispatch(&mut network, vec![hard.clone()], params(), 0)
        .expect("legal min-difficulty recovery branch");
    let locator = network.peer_manager().header_store().locator();
    let handle = ManagedNetworkHandle::from_network_fixture(network);
    let best = handle.best_chain_entries().expect("owner query");
    // Assert
    assert_eq!(best.len(), 4);
    assert_eq!(best.last().expect("best").block_hash, block_hash(&hard));
    assert_eq!(locator.block_hashes[0], block_hash(&hard).into());
    assert!(result.outbound.iter().any(|message| matches!(message,
        WireNetworkMessage::GetHeaders { locator, .. } if locator.block_hashes[0] == block_hash(&hard).into()
    )));
    assert_eq!(
        best[3].chain_work,
        block_proof(root.bits)
            .expect("proof")
            .wrapping_add(block_proof(hard_parent.bits).expect("proof"))
            .wrapping_add(block_proof(hard_base.bits).expect("proof"))
            .wrapping_add(block_proof(hard.bits).expect("proof"))
    );
}

#[test]
fn phase165_headers_equal_work_retains_first_shorter_opposing_hash_tip() {
    // Arrange
    let mut network = fixture();
    let root = header(BlockHash::default(), 1_000, EASY_BITS, 20);
    initialize_root(&mut network, &root, params());
    let parent = header(block_hash(&root), 1_001, EASY_BITS, 21);
    let hard = header(block_hash(&parent), 1_002, 0x201f_ffff, 22);
    dispatch(&mut network, vec![parent, hard.clone()], params(), 0).expect("first branch");
    let mut easy = root;
    for marker in 23..27 {
        easy = header(block_hash(&easy), easy.time + 1_201, EASY_BITS, marker);
        dispatch(&mut network, vec![easy.clone()], params(), 0).expect("easy prefix");
    }
    let tied = (27..=255)
        .map(|marker| header(block_hash(&easy), easy.time + 1_201, EASY_BITS, marker))
        .find(|candidate| block_hash(candidate) < block_hash(&hard))
        .expect("opposing hash order");
    // Act
    dispatch(&mut network, vec![tied.clone()], params(), 0).expect("equal work branch");
    // Assert
    let store = network.peer_manager().header_store();
    assert_eq!(
        store
            .entry(&block_hash(&tied))
            .expect("tall entry")
            .chain_work,
        store.best_tip().expect("first best").chain_work
    );
    assert_eq!(
        store.best_tip().expect("first best").block_hash,
        block_hash(&hard)
    );
    assert!(
        store.entry(&block_hash(&tied)).expect("tall entry").height
            > store.best_tip().expect("short best").height
    );
}

#[test]
fn phase165_headers_hash_failure_and_invalid_suffix_preserve_complete_state() {
    for route in 0..3 {
        // Arrange
        let mut network = fixture();
        let root = header(BlockHash::default(), 1_000, EASY_BITS, 40);
        initialize_root(&mut network, &root, params());
        let mut invalid = header(block_hash(&root), 1_001, EASY_BITS, 41);
        invalid.nonce = (0..=u32::MAX)
            .find(|nonce| {
                invalid.nonce = *nonce;
                check_block_header(&invalid).is_err()
            })
            .expect("unsatisfied hash");
        let before = snapshot(&network);
        // Act
        let result = dispatch(&mut network, vec![root, invalid], params(), route);
        // Assert
        assert!(result.is_err());
        assert_eq!(snapshot(&network), before);
    }
}

#[test]
fn phase165_headers_normalized_zero_target_preserves_complete_state() {
    for route in 0..3 {
        // Arrange
        let mut network = fixture();
        let mut candidate = header(BlockHash::default(), 1_000, EASY_BITS, 45);
        candidate.bits = 0x0100_3456;
        let before = snapshot(&network);
        // Act
        let result = dispatch(&mut network, vec![candidate], params(), route);
        // Assert
        assert!(result.is_err());
        assert_eq!(snapshot(&network), before);
    }
}

#[test]
fn phase165_headers_contextual_suffix_retains_prefix_and_first_equal_work_arrival() {
    // Arrange
    let mut network = fixture();
    let root = header(BlockHash::default(), 1_000, EASY_BITS, 90);
    initialize_root(&mut network, &root, params());
    let prefix = header(block_hash(&root), 1_001, EASY_BITS, 91);
    let invalid_context = header(block_hash(&prefix), 1_002, EASY_BITS, 92);
    assert!(check_block_header(&invalid_context).is_ok());
    // Act
    let result = dispatch(
        &mut network,
        vec![prefix.clone(), invalid_context.clone()],
        params(),
        0,
    );
    // Assert
    assert!(result.is_err());
    assert_eq!(
        network
            .best_chain_entries()
            .last()
            .expect("retained prefix")
            .block_hash,
        block_hash(&prefix)
    );
    assert!(
        !network
            .peer_manager()
            .header_store()
            .contains(&block_hash(&invalid_context))
    );
    // Act
    let competitor = header(block_hash(&root), 1_001, EASY_BITS, 93);
    dispatch(&mut network, vec![competitor], params(), 0).expect("later equal work branch");
    // Assert
    assert_eq!(
        network
            .best_chain_entries()
            .last()
            .expect("first accepted best")
            .block_hash,
        block_hash(&prefix)
    );
}

#[test]
fn phase165_headers_discontinuous_batch_preserves_complete_state() {
    // Arrange
    let mut network = fixture();
    let root = header(BlockHash::default(), 1_000, EASY_BITS, 94);
    initialize_root(&mut network, &root, params());
    let first = header(block_hash(&root), 1_001, EASY_BITS, 95);
    let discontinuous = header(block_hash(&root), 1_002, EASY_BITS, 96);
    let before = snapshot(&network);
    // Act
    let result = dispatch(&mut network, vec![first, discontinuous], params(), 0);
    // Assert
    assert!(result.is_err());
    assert_eq!(snapshot(&network), before);
}

#[test]
fn phase165_headers_p2p_cannot_initialize_an_unknown_root() {
    for route in 0..3 {
        for seeded in [false, true] {
            // Arrange
            let mut network = fixture();
            let root = header(BlockHash::default(), 1_000, EASY_BITS, 97);
            if seeded {
                initialize_root(&mut network, &root, params());
            }
            let before = snapshot(&network);
            // Act
            let result = dispatch(&mut network, vec![root], params(), route);
            // Assert
            assert!(result.is_err());
            assert_eq!(snapshot(&network), before);
            assert_eq!(network.best_chain_entries().len(), usize::from(seeded));
        }
    }
}

#[test]
fn phase165_headers_explicit_reseed_checks_height_and_cannot_replace_bound_owner() {
    // Arrange
    let mut network = fixture();
    let root = header(BlockHash::default(), 1_000, EASY_BITS, 46);
    let child = header(block_hash(&root), 1_001, EASY_BITS, 47);
    let mut actual = HeaderStore::default();
    actual
        .insert_header(root.clone())
        .expect("unbound root bytes");
    actual
        .insert_header(child.clone())
        .expect("unbound child bytes");
    let mut entries: Vec<_> = actual.entries().cloned().collect();
    entries
        .iter_mut()
        .find(|entry| entry.block_hash == block_hash(&child))
        .expect("child")
        .height = u32::MAX;
    let forged = HeaderStore::from_entries(entries).expect("untrusted metadata");
    let before = snapshot(&network);
    // Act
    let result = network.try_admit_fixture_headers(forged, 100_000, params());
    // Assert
    assert!(result.is_err());
    assert_eq!(snapshot(&network), before);
    // Act
    network
        .try_admit_fixture_headers(actual.clone(), 100_000, params())
        .expect("explicit actual admission");
    let before = snapshot(&network);
    let result = network.try_admit_fixture_headers(actual, 100_000, params());
    // Assert
    assert!(result.is_err());
    assert_eq!(snapshot(&network), before);
    assert_eq!(network.best_chain_entries().len(), 2);
}

#[test]
fn phase165_headers_binding_clones_duplicate_and_missing_parent_preserve_authority() {
    // Arrange
    let mut network = fixture();
    let root = header(BlockHash::default(), 1_000, EASY_BITS, 50);
    initialize_root(&mut network, &root, params());
    let child = header(block_hash(&root), 1_001, EASY_BITS, 52);
    dispatch(&mut network, vec![child.clone()], params(), 0).expect("accepted non-root");
    let mut cloned = network.clone();
    let before = snapshot(&cloned);
    let changed = ConsensusParams {
        allow_min_difficulty_blocks: false,
        ..params()
    };
    // Act
    let result = dispatch(&mut cloned, vec![child.clone()], changed, 0);
    // Assert
    assert!(result.is_err());
    assert_eq!(snapshot(&cloned), before);
    // Act
    let orphan = header(BlockHash::from_byte_array([42; 32]), 1_001, EASY_BITS, 51);
    let result = dispatch(&mut cloned, vec![orphan], params(), 0);
    // Assert
    assert!(result.is_err());
    assert_eq!(snapshot(&cloned), before);
    // Act
    dispatch(&mut cloned, vec![child], params(), 0).expect("non-root duplicate");
    // Assert
    assert_eq!(
        cloned.peer_manager().header_store(),
        network.peer_manager().header_store()
    );
}

#[test]
fn phase165_headers_valid_raw_numbers_are_recomputed_from_actual_admission() {
    // Arrange
    let mut network = fixture();
    let root = header(BlockHash::default(), 1_000, EASY_BITS, 60);
    let raw = HeaderStore::from_entries([HeaderEntry {
        block_hash: block_hash(&root),
        header: root.clone(),
        height: 0,
        chain_work: open_bitcoin_core::consensus::chain_proof::ProofWork::from_le_bytes([255; 32]),
    }])
    .expect("raw numbers");
    network.seed_header_store(raw.clone());
    assert!(network.best_chain_entries().is_empty());
    // Act
    network
        .try_admit_fixture_headers(raw.clone(), 100_000, params())
        .expect("explicit actual root admission");
    let child = header(block_hash(&root), 1_001, EASY_BITS, 61);
    dispatch(&mut network, vec![child], params(), 0).expect("actual re-admission");
    let before = snapshot(&network);
    network.seed_header_store(raw);
    // Assert
    assert_eq!(snapshot(&network), before);
    assert_eq!(
        network.best_chain_entries()[0].chain_work,
        block_proof(root.bits).expect("own proof")
    );
}

#[test]
fn phase165_headers_raw_inflated_parent_cannot_supply_live_ancestry() {
    // Arrange
    let mut network = fixture();
    let root = header(BlockHash::default(), 1_000, EASY_BITS, 10);
    let raw = HeaderStore::from_entries([HeaderEntry {
        block_hash: BlockHash::from_byte_array([99; 32]),
        header: root.clone(),
        height: 0,
        chain_work: open_bitcoin_core::consensus::chain_proof::ProofWork::from_le_bytes([255; 32]),
    }])
    .expect("unproved import");
    network.seed_header_store(raw);
    let before = snapshot(&network);
    let child = header(BlockHash::from_byte_array([99; 32]), 1_001, EASY_BITS, 11);
    // Act
    let result = dispatch(&mut network, vec![child], params(), 0);
    // Assert
    assert!(result.is_err());
    assert_eq!(snapshot(&network), before);
    assert!(network.best_chain_entries().is_empty());
}

fn snapshot(network: &ManagedPeerNetwork<MemoryChainstateStore>) -> String {
    let admission = format!(
        "{:?}",
        (
            network.admission_scope,
            network.maybe_header_consensus_params,
            &network.known_peers,
            network.inbound_admission_policy,
            &network.inbound_admission_info,
            &network.resource_governance_info,
            network.relay_activation,
            network.block_relay_activation,
            network.inbound_serving_enabled,
            &network.relay_fanout,
        )
    );
    let lifecycle = format!(
        "{:?}",
        (
            network.authority_epoch,
            network.lifecycle_generation,
            network.dirty_generation,
            &network.unbroadcast_members,
            network.maybe_retry_due_at_unix_seconds,
            network.maybe_unbroadcast_walk_cursor,
            network.maybe_last_transport_written_clear,
            &network.peer_session_generations,
            &network.latest_mempool_recovery,
            &network.latest_mempool_recovery_storage_error,
        )
    );
    let configuration = format!(
        "{:?}",
        (
            &network.local_config,
            network.maybe_configured_basic_indexing,
            network.configured_basic_peer_serving,
            network.prune_mode,
            network.serving_have_pruned,
        )
    );
    format!(
        "{admission}{lifecycle}{configuration}{:?}{:?}{:?}{:?}{:?}{:?}{:?}{:?}{:?}{:?}{:?}{:?}{:?}{:?}",
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
        network.block_relay_evidence,
        network.relay_serving,
        network.compact_extra_txn
    )
}

#[test]
fn phase165_headers_all_canonical_selectors_refuse_before_selected_arithmetic() {
    use open_bitcoin_core::consensus::CanonicalChain;
    for chain in [
        CanonicalChain::Mainnet,
        CanonicalChain::Testnet3,
        CanonicalChain::Signet,
        CanonicalChain::Regtest,
    ] {
        for route in 0..3 {
            // Arrange
            let mut network = fixture().into_canonical_scope(chain);
            let selected = ConsensusParams {
                pow_limit_bits: match chain {
                    CanonicalChain::Mainnet | CanonicalChain::Testnet3 => 0x1d00_ffff,
                    CanonicalChain::Signet => 0x1e03_77ae,
                    CanonicalChain::Regtest => EASY_BITS,
                },
                ..params()
            };
            let before = snapshot(&network);
            // Act
            let result = dispatch(
                &mut network,
                vec![chain.spec().genesis_header()],
                selected,
                route,
            );
            // Assert
            assert!(
                matches!(result, Err(crate::ManagedNetworkError::Network(NetworkError::InvalidHeader { ref reject_reason, .. })) if reject_reason == "unsupported-validation-scope")
            );
            assert_eq!(snapshot(&network), before);
            // Controlled fixture API also preserves the product canonical support guard.
            let mut headers = HeaderStore::default();
            headers
                .insert_header(chain.spec().genesis_header())
                .expect("actual raw root bytes");
            let result = network.try_admit_fixture_headers(headers, i64::MAX, selected);
            assert!(
                matches!(result, Err(crate::ManagedNetworkError::Network(NetworkError::InvalidHeader { ref reject_reason, .. })) if reject_reason == "unsupported-validation-scope")
            );
            assert_eq!(snapshot(&network), before);
        }
    }
}

#[test]
fn phase165_headers_same_pow_limit_cannot_transfer_fixture_or_foreign_root_binding() {
    use open_bitcoin_core::consensus::CanonicalChain;
    for (selected, foreign) in [
        (CanonicalChain::Mainnet, CanonicalChain::Testnet3),
        (CanonicalChain::Testnet3, CanonicalChain::Mainnet),
    ] {
        // Arrange
        let mut network = fixture();
        let root = header(BlockHash::default(), 1_000, EASY_BITS, 80);
        initialize_root(&mut network, &root, params());
        let mut network = network.into_canonical_scope(selected);
        assert!(network.best_chain_entries().is_empty());
        let before = snapshot(&network);
        // Act
        let result = dispatch(
            &mut network,
            vec![foreign.spec().genesis_header()],
            params(),
            0,
        );
        let rebound = ManagedNetworkHandle::from_network_fixture(network.clone())
            .maybe_canonical_chain()
            .expect("immutable scope");
        // Assert
        assert!(
            matches!(result, Err(crate::ManagedNetworkError::Network(NetworkError::InvalidHeader { ref reject_reason, .. })) if reject_reason == "noncanonical-genesis-header")
        );
        assert_eq!(snapshot(&network), before);
        assert_eq!(rebound, Some(selected));
    }
}
