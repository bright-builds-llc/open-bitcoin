// Parity breadcrumbs:
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/chain.h
// - packages/bitcoin-knots/src/node/blockstorage.cpp

use super::*;

fn fixture() -> ManagedPeerNetwork<MemoryChainstateStore> {
    ManagedPeerNetwork::new(
        MemoryChainstateStore::default(),
        local_config(42),
        PolicyConfig::default(),
    )
}

#[test]
fn phase165_eligibility_local_checked_body_earns_sequence_before_coins_authority() {
    // Arrange
    let mut network = fixture();
    let root = build_block(BlockHash::default(), 0, 5_000_000_000);
    // Act
    network
        .connect_local_block(&root, verify_flags(), consensus_params())
        .expect("local checked root");
    // Assert
    assert_eq!(
        network
            .maybe_body_eligibility(block_hash(&root.header))
            .expect("earned body sequence")
            .maybe_sequence,
        Some(2)
    );
}

fn with_headers(blocks: &[&Block]) -> ManagedPeerNetwork<MemoryChainstateStore> {
    let mut network = fixture();
    let mut headers = open_bitcoin_network::HeaderStore::default();
    for block in blocks {
        headers
            .insert_header_with_params(block.header.clone(), &consensus_params())
            .expect("selected header");
    }
    network
        .try_admit_fixture_headers(headers, 2_000_000_000, consensus_params())
        .expect("explicit model admission");
    network
}

fn retain(
    network: &mut ManagedPeerNetwork<MemoryChainstateStore>,
    block: &Block,
) -> Result<BlockConnectDisposition, crate::ManagedNetworkError> {
    let work = network
        .peer_manager()
        .header_store()
        .maybe_proof_facts(block_hash(&block.header))
        .map_or(
            open_bitcoin_core::consensus::chain_proof::ProofWork::ZERO,
            |facts| facts.cumulative,
        );
    network.connect_stored_block(
        block,
        work,
        2_000_000_000,
        verify_flags(),
        consensus_params(),
    )
}

#[test]
fn phase165_eligibility_opposing_header_and_body_order_retains_earlier_active_tie() {
    // Arrange
    let root = build_block(BlockHash::default(), 0, 5_000_000_000);
    let a = build_block(block_hash(&root.header), 1, 5_000_000_000);
    let b = build_block(block_hash(&root.header), 1, 4_000_000_000);
    let mut network = with_headers(&[&root]);
    network.add_inbound_peer(1).expect("peer");
    network
        .receive_message(
            1,
            WireNetworkMessage::Version(open_bitcoin_network::VersionMessage {
                nonce: 99,
                ..Default::default()
            }),
            2_000_000_000,
            verify_flags(),
            consensus_params(),
        )
        .expect("version");
    for block in [&a, &b] {
        network
            .receive_sync_message(
                1,
                WireNetworkMessage::Headers(open_bitcoin_network::HeadersMessage {
                    headers: vec![block.header.clone()],
                }),
                2_000_000_000,
                verify_flags(),
                consensus_params(),
            )
            .expect("actual ordered headers");
    }
    assert_eq!(
        network
            .peer_manager()
            .header_store()
            .best_tip()
            .expect("header A first")
            .block_hash,
        block_hash(&a.header)
    );
    network
        .connect_local_block(&root, verify_flags(), consensus_params())
        .expect("root");
    // Act
    retain(&mut network, &b).expect("B body first");
    retain(&mut network, &a).expect("A side body later");
    let before = network.eligible_body_candidates();
    retain(&mut network, &a).expect("side duplicate");
    retain(&mut network, &b).expect("active duplicate");
    // Assert
    assert_eq!(network.eligible_body_candidates(), before);
    assert_eq!(before[0].block_hash, block_hash(&b.header));
    assert_eq!(before[1].block_hash, block_hash(&a.header));
    assert!(before[0].maybe_sequence < before[1].maybe_sequence);
    assert_eq!(
        network.maybe_chain_tip().expect("active").block_hash,
        block_hash(&b.header)
    );
}

#[test]
fn phase165_eligibility_child_body_waits_for_transaction_linked_parent() {
    // Arrange
    let root = build_block(BlockHash::default(), 0, 5_000_000_000);
    let parent = build_block(block_hash(&root.header), 1, 5_000_000_000);
    let child = build_block(block_hash(&parent.header), 2, 5_000_000_000);
    let mut network = with_headers(&[&root, &parent, &child]);
    network
        .connect_local_block(&root, verify_flags(), consensus_params())
        .expect("root");
    // Act
    retain(&mut network, &child).expect("pending child");
    let pending = network
        .maybe_body_eligibility(block_hash(&child.header))
        .expect("checked body");
    retain(&mut network, &parent).expect("parent links");
    // Assert
    assert_eq!(pending.maybe_sequence, None);
    assert_eq!(
        network
            .maybe_body_eligibility(block_hash(&child.header))
            .expect("released")
            .maybe_sequence,
        Some(4)
    );
    assert_eq!(
        network.eligible_body_candidates()[0].block_hash,
        block_hash(&child.header)
    );
    assert_eq!(
        network
            .maybe_chain_tip()
            .expect("only parent connected")
            .block_hash,
        block_hash(&parent.header)
    );
}

#[test]
fn phase165_eligibility_raw_cache_and_known_hash_do_not_earn_linkage() {
    // Arrange
    let mut network = fixture();
    let root = build_block(BlockHash::default(), 0, 5_000_000_000);
    let hash = block_hash(&root.header);
    // Act
    network.blocks_by_hash.insert(hash, root);
    network.note_local_block_hash(hash);
    // Assert
    assert_eq!(network.maybe_body_eligibility(hash), None);
    assert!(network.eligible_body_candidates().is_empty());
}

#[test]
fn phase165_eligibility_mutated_body_cannot_change_sequence_cache_or_chainstate() {
    // Arrange
    let root = build_block(BlockHash::default(), 0, 5_000_000_000);
    let side = build_block(block_hash(&root.header), 1, 5_000_000_000);
    let mut network = with_headers(&[&root, &side]);
    network
        .connect_local_block(&root, verify_flags(), consensus_params())
        .expect("root");
    let before = network.chainstate_snapshot().expect("snapshot");
    let headers_before = network.peer_manager().header_store().clone();
    let bytes_before = network.blocks_by_hash.clone();
    let mut mutated = side;
    mutated.transactions.clear();
    // Act
    let result = retain(&mut network, &mutated);
    // Assert
    assert!(result.is_err());
    assert_eq!(network.chainstate_snapshot().expect("unchanged"), before);
    assert_eq!(*network.peer_manager().header_store(), headers_before);
    assert_eq!(network.blocks_by_hash, bytes_before);
}

#[test]
fn phase165_eligibility_contextual_body_failure_does_not_earn_sequence() {
    // Arrange
    let root = build_block(BlockHash::default(), 0, 5_000_000_000);
    let wrong_height = build_block(block_hash(&root.header), 99, 5_000_000_000);
    let mut network = with_headers(&[&root, &wrong_height]);
    network
        .connect_local_block(&root, verify_flags(), consensus_params())
        .expect("root");
    let before = network.peer_manager().header_store().clone();
    // Act
    let result = retain(&mut network, &wrong_height);
    // Assert
    assert!(
        result
            .expect_err("BIP34 height check")
            .to_string()
            .contains("bad-cb-height")
    );
    assert_eq!(*network.peer_manager().header_store(), before);
}

#[test]
fn phase165_eligibility_unknown_header_parent_cannot_earn_sequence() {
    // Arrange
    let mut network = fixture();
    let orphan = build_block(BlockHash::from_byte_array([99; 32]), 1, 5_000_000_000);
    // Act
    let result = retain(&mut network, &orphan);
    // Assert
    assert!(matches!(
        result,
        Ok(BlockConnectDisposition::Disconnected { .. })
    ));
    assert!(network.header_entries().is_empty());
    assert!(network.eligible_body_candidates().is_empty());
    assert_eq!(
        network.maybe_body_eligibility(block_hash(&orphan.header)),
        None
    );
}

#[test]
fn phase165_eligibility_full_selected_params_cannot_be_changed_for_bodies() {
    // Arrange
    let root = build_block(BlockHash::default(), 0, 5_000_000_000);
    let mut network = with_headers(&[&root]);
    let before = network.peer_manager().header_store().clone();
    let changed = ConsensusParams {
        enforce_bip34_height_in_coinbase: false,
        ..consensus_params()
    };
    // Act
    let result = network.connect_local_block(&root, verify_flags(), changed);
    // Assert
    assert!(result.is_err());
    assert_eq!(*network.peer_manager().header_store(), before);
    assert!(network.blocks_by_hash.is_empty());
}

#[test]
fn phase165_eligibility_script_or_coins_failure_revokes_candidate_without_connecting() {
    // Arrange
    let root = build_block(BlockHash::default(), 0, 5_000_000_000);
    let mut invalid_spend = build_block(block_hash(&root.header), 1, 5_000_000_000);
    invalid_spend
        .transactions
        .push(spend_transaction(Txid::from_byte_array([44; 32]), 1_000));
    invalid_spend.header.merkle_root = block_merkle_root(&invalid_spend.transactions)
        .expect("merkle")
        .0;
    mine_header(&mut invalid_spend);
    let mut network = with_headers(&[&root, &invalid_spend]);
    network
        .connect_local_block(&root, verify_flags(), consensus_params())
        .expect("root");
    let before = network.chainstate_snapshot().expect("before");
    // Act
    let result = retain(&mut network, &invalid_spend);
    // Assert
    assert!(result.is_err());
    let facts = network
        .maybe_body_eligibility(block_hash(&invalid_spend.header))
        .expect("TRANSACTIONS linkage before UTXO check");
    assert_eq!(facts.maybe_sequence, Some(3));
    assert!(facts.failed);
    assert!(
        !network
            .eligible_body_candidates()
            .iter()
            .any(|candidate| candidate.block_hash == facts.block_hash)
    );
    assert_eq!(
        network.chainstate_snapshot().expect("unchanged coins"),
        before
    );
}

#[test]
fn phase165_eligibility_handle_projects_all_sides_and_pruning_keeps_history() {
    // Arrange
    let root = build_block(BlockHash::default(), 0, 5_000_000_000);
    let a = build_block(block_hash(&root.header), 1, 5_000_000_000);
    let b = build_block(block_hash(&root.header), 1, 4_000_000_000);
    let child = build_block(block_hash(&a.header), 2, 5_000_000_000);
    let mut network = with_headers(&[&root, &a, &b, &child]);
    network
        .connect_local_block(&root, verify_flags(), consensus_params())
        .expect("root");
    retain(&mut network, &b).expect("active B");
    retain(&mut network, &a).expect("side A");
    retain(&mut network, &child).expect("side child");
    let handle = crate::ManagedNetworkHandle::from_network_fixture(network);
    let before = handle
        .maybe_body_eligibility(block_hash(&a.header))
        .expect("handle")
        .expect("A");
    assert_eq!(
        handle.eligible_body_candidates().expect("all sides").len(),
        4
    );
    // Act
    handle
        .forget_block_payload_and_undo_for_test(block_hash(&a.header))
        .expect("drop side payload");
    // Assert
    let after = handle
        .maybe_body_eligibility(block_hash(&a.header))
        .expect("handle")
        .expect("historical A");
    assert_eq!(after.maybe_sequence, before.maybe_sequence);
    assert!(!after.available);
    assert_eq!(
        handle
            .eligible_body_candidates()
            .expect("present branches")
            .iter()
            .map(|facts| facts.block_hash)
            .collect::<Vec<_>>(),
        [block_hash(&b.header), block_hash(&root.header)]
    );
}

#[test]
fn phase165_eligibility_public_delegate_rechecks_body_and_full_selected_parameters() {
    // Arrange
    let root = build_block(BlockHash::default(), 0, 5_000_000_000);
    let mut network = with_headers(&[&root]);
    let before = network.peer_manager().header_store().clone();
    let changed = ConsensusParams {
        enforce_segwit: false,
        ..consensus_params()
    };
    let mut mutated = root.clone();
    mutated.transactions.clear();
    // Act / Assert
    assert!(
        network
            .peer_manager_mut()
            .check_body_eligibility(&root, &changed, 2_000_000_000)
            .is_err()
    );
    assert!(
        network
            .peer_manager_mut()
            .check_body_eligibility(&mutated, &consensus_params(), 2_000_000_000)
            .is_err()
    );
    assert_eq!(*network.peer_manager().header_store(), before);
    network
        .peer_manager_mut()
        .check_body_eligibility(&root, &consensus_params(), 2_000_000_000)
        .expect("actual checked bytes");
    assert_eq!(
        network
            .maybe_body_eligibility(block_hash(&root.header))
            .expect("checked")
            .maybe_sequence,
        Some(2)
    );
}

#[test]
fn phase165_eligibility_raw_numeric_header_import_is_never_body_authority() {
    // Arrange
    let root = build_block(BlockHash::default(), 0, 5_000_000_000);
    let mut admitted = open_bitcoin_network::HeaderStore::default();
    admitted
        .insert_header_with_params(root.header.clone(), &consensus_params())
        .expect("header");
    let entries: Vec<_> = admitted.entries().cloned().collect();
    let imported =
        open_bitcoin_network::HeaderStore::from_entries(entries).expect("numeric import");
    let mut network = fixture();
    network.peer_manager_mut().seed_header_store(imported);
    let before = network.peer_manager().header_store().clone();
    // Act
    let result = network.peer_manager_mut().check_body_eligibility(
        &root,
        &consensus_params(),
        2_000_000_000,
    );
    // Assert
    assert!(result.is_err());
    assert_eq!(*network.peer_manager().header_store(), before);
    assert_eq!(
        network.maybe_body_eligibility(block_hash(&root.header)),
        None
    );
}

#[test]
fn phase165_eligibility_public_delegate_unknown_parent_refuses_without_effects() {
    // Arrange
    let mut network = fixture();
    let orphan = build_block(BlockHash::from_byte_array([99; 32]), 1, 5_000_000_000);
    let before = network.peer_manager().header_store().clone();
    // Act
    let result = network.peer_manager_mut().check_body_eligibility(
        &orphan,
        &consensus_params(),
        2_000_000_000,
    );
    // Assert
    assert!(result.is_err());
    assert_eq!(*network.peer_manager().header_store(), before);
    assert!(network.eligible_body_candidates().is_empty());
}

#[test]
fn phase165_eligibility_peer_linkage_without_node_retained_suffix_cannot_rank() {
    // Arrange
    let root = build_block(BlockHash::default(), 0, 5_000_000_000);
    let parent = build_block(block_hash(&root.header), 1, 5_000_000_000);
    let child = build_block(block_hash(&parent.header), 2, 5_000_000_000);
    let mut network = with_headers(&[&root, &parent, &child]);
    for block in [&root, &parent] {
        network
            .peer_manager_mut()
            .check_body_eligibility(block, &consensus_params(), 2_000_000_000)
            .expect("real bytes checked outside node retention");
    }
    // Act
    retain(&mut network, &child).expect("child retained but required suffix absent");
    // Assert
    assert!(network.eligible_body_candidates().is_empty());
    assert!(
        !network
            .maybe_body_eligibility(block_hash(&parent.header))
            .expect("historical checked linkage")
            .available
    );
    assert!(
        network
            .maybe_body_eligibility(block_hash(&child.header))
            .expect("retained child")
            .available
    );
    network
        .connect_local_block(&root, verify_flags(), consensus_params())
        .expect("managed root retention");
    assert_eq!(network.eligible_body_candidates().len(), 1);
    retain(&mut network, &parent).expect("fresh managed checked parent retention");
    assert_eq!(
        network.eligible_body_candidates()[0].block_hash,
        block_hash(&child.header)
    );
    assert_eq!(
        network
            .maybe_body_eligibility(block_hash(&parent.header))
            .expect("managed retained parent")
            .maybe_sequence,
        Some(3)
    );
}
