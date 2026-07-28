// Parity breadcrumbs:
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/txmempool.cpp
// - packages/bitcoin-knots/src/kernel/disconnected_transactions.cpp
// - packages/bitcoin-knots/src/blockencodings.cpp
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/test/functional/p2p_compactblocks.py

use super::*;
use crate::network::mempool_lifecycle;

#[test]
fn managed_reorg_reacceptance_uses_explicit_event_time() {
    // Arrange
    let (mut network, _genesis, spendable, coinbase_txids) = network_with_chain();
    let disconnected_transaction = spend_transaction(coinbase_txids[0], 499_999_000);
    let disconnected_txid = txid(&disconnected_transaction);
    let old_tip = build_block_with_transactions(
        block_hash(&spendable.header),
        2,
        vec![disconnected_transaction.clone()],
    );
    let replacement_tip = build_block_with_transactions(block_hash(&spendable.header), 2, vec![]);
    network
        .connect_local_block(&old_tip, verify_flags(), consensus_params())
        .expect("connect old tip");
    assert!(
        network
            .mempool()
            .mempool()
            .entry(&disconnected_txid)
            .is_none()
    );
    let reorg_context = ReorgLifecycleContext::new(PolicyTime::new(80));

    // Act
    network
        .reorg_to_branch(
            &[old_tip],
            &[AnchoredBlock {
                block: replacement_tip,
                chain_work: 3,
            }],
            reorg_context,
            verify_flags(),
            consensus_params(),
        )
        .expect("reorg to replacement tip");

    // Assert
    assert!(
        network
            .mempool()
            .mempool()
            .entry(&disconnected_txid)
            .is_some()
    );
    assert!(
        network
            .transactions_by_txid
            .contains_key(&disconnected_txid)
    );
    let metadata = network
        .mempool()
        .mempool()
        .entry(&disconnected_txid)
        .expect("reaccepted entry")
        .metadata;
    assert_eq!(
        metadata.accepted_at,
        MempoolAcceptanceTime::Known(PolicyTime::new(80))
    );
    assert_eq!(metadata.origin, MempoolOrigin::Reorg);
    assert_eq!(metadata.relay_intent, RelayIntent::NotRequested);
    assert_lifecycle_authority(&network, 1);
    assert_eq!(
        mempool_lifecycle::block_lifecycle_context_from_reorg(reorg_context, 2),
        open_bitcoin_mempool::BlockLifecycleContext::new(PolicyTime::new(80), 2)
    );
}

#[test]
fn local_active_tip_change_resets_both_reject_evidence_domains() {
    // Arrange
    let mut network = ManagedPeerNetwork::new(
        MemoryChainstateStore::default(),
        local_config(705),
        PolicyConfig::default(),
    );
    let (hard_reject, reconsiderable) = seed_reject_evidence(&mut network);
    let genesis = build_block_with_transactions(BlockHash::from_byte_array([0_u8; 32]), 0, vec![]);

    // Act
    network
        .connect_local_block(&genesis, verify_flags(), consensus_params())
        .expect("connect local genesis");

    // Assert
    assert_reject_evidence(&network, hard_reject, reconsiderable, false);
}

#[test]
fn stored_connected_active_tip_change_resets_both_reject_evidence_domains() {
    // Arrange
    let mut network = ManagedPeerNetwork::new(
        MemoryChainstateStore::default(),
        local_config(706),
        PolicyConfig::default(),
    );
    let (hard_reject, reconsiderable) = seed_reject_evidence(&mut network);
    let genesis = build_block_with_transactions(BlockHash::from_byte_array([0_u8; 32]), 0, vec![]);

    // Act
    let disposition = network
        .connect_stored_block(
            &genesis,
            1,
            1_231_006_500,
            verify_flags(),
            consensus_params(),
        )
        .expect("connect stored genesis");

    // Assert
    assert!(matches!(disposition, BlockConnectDisposition::Connected(_)));
    assert_reject_evidence(&network, hard_reject, reconsiderable, false);
}

#[test]
fn successful_reorg_resets_both_reject_evidence_domains() {
    // Arrange
    let (mut network, _genesis, spendable, _coinbase_txids) = network_with_chain();
    let old_tip = build_block_with_transactions(block_hash(&spendable.header), 2, vec![]);
    network
        .connect_local_block(&old_tip, verify_flags(), consensus_params())
        .expect("connect old tip");
    let replacement_tip = build_block_with_transactions(block_hash(&spendable.header), 2, vec![]);
    let (hard_reject, reconsiderable) = seed_reject_evidence(&mut network);

    // Act
    network
        .reorg_to_branch(
            &[old_tip],
            &[AnchoredBlock {
                block: replacement_tip,
                chain_work: 3,
            }],
            ReorgLifecycleContext::new(PolicyTime::new(90)),
            verify_flags(),
            consensus_params(),
        )
        .expect("reorg to replacement tip");

    // Assert
    assert_reject_evidence(&network, hard_reject, reconsiderable, false);
}

#[test]
fn duplicate_non_extending_and_disconnected_receipts_preserve_reject_evidence() {
    // Arrange
    let mut network = ManagedPeerNetwork::new(
        MemoryChainstateStore::default(),
        local_config(707),
        PolicyConfig::default(),
    );
    let genesis = build_block_with_transactions(BlockHash::from_byte_array([0_u8; 32]), 0, vec![]);
    network
        .connect_local_block(&genesis, verify_flags(), consensus_params())
        .expect("connect genesis");
    let (hard_reject, reconsiderable) = seed_reject_evidence(&mut network);
    let non_extending =
        build_block_with_transactions(BlockHash::from_byte_array([0x51; 32]), 1, vec![]);

    // Act
    let duplicate = network
        .connect_stored_block(
            &genesis,
            1,
            1_231_006_500,
            verify_flags(),
            consensus_params(),
        )
        .expect("classify duplicate");
    let side_branch = network
        .connect_stored_block(
            &non_extending,
            2,
            1_231_006_501,
            verify_flags(),
            consensus_params(),
        )
        .expect("classify non-extending");

    // Assert
    assert!(matches!(duplicate, BlockConnectDisposition::Duplicate(_)));
    assert!(matches!(
        side_branch,
        BlockConnectDisposition::NonExtending { .. }
    ));
    assert_reject_evidence(&network, hard_reject, reconsiderable, true);

    // Arrange
    let mut empty_network = ManagedPeerNetwork::new(
        MemoryChainstateStore::default(),
        local_config(708),
        PolicyConfig::default(),
    );
    let (hard_reject, reconsiderable) = seed_reject_evidence(&mut empty_network);

    // Act
    let disconnected = empty_network
        .connect_stored_block(
            &non_extending,
            1,
            1_231_006_501,
            verify_flags(),
            consensus_params(),
        )
        .expect("classify disconnected");

    // Assert
    assert!(matches!(
        disconnected,
        BlockConnectDisposition::Disconnected { .. }
    ));
    assert_reject_evidence(&empty_network, hard_reject, reconsiderable, true);
}

#[test]
fn failed_local_transition_preserves_reject_evidence() {
    // Arrange
    let mut network = ManagedPeerNetwork::new(
        MemoryChainstateStore::default(),
        local_config(709),
        PolicyConfig::default(),
    );
    let (hard_reject, reconsiderable) = seed_reject_evidence(&mut network);
    let mut invalid_genesis =
        build_block_with_transactions(BlockHash::from_byte_array([0_u8; 32]), 0, vec![]);
    invalid_genesis.header.merkle_root = MerkleRoot::from_byte_array([0x61; 32]);

    // Act
    let result = network.connect_local_block(&invalid_genesis, verify_flags(), consensus_params());

    // Assert
    assert!(result.is_err());
    assert_reject_evidence(&network, hard_reject, reconsiderable, true);
}

#[test]
fn failed_reorg_preserves_reject_evidence() {
    // Arrange
    let (mut network, _genesis, spendable, _coinbase_txids) = network_with_chain();
    let old_tip = build_block_with_transactions(block_hash(&spendable.header), 2, vec![]);
    network
        .connect_local_block(&old_tip, verify_flags(), consensus_params())
        .expect("connect old tip");
    let disconnected_replacement =
        build_block_with_transactions(BlockHash::from_byte_array([0x71; 32]), 2, vec![]);
    let (hard_reject, reconsiderable) = seed_reject_evidence(&mut network);

    // Act
    let result = network.reorg_to_branch(
        &[old_tip],
        &[AnchoredBlock {
            block: disconnected_replacement,
            chain_work: 3,
        }],
        ReorgLifecycleContext::new(PolicyTime::new(91)),
        verify_flags(),
        consensus_params(),
    );

    // Assert
    assert!(result.is_err());
    assert_reject_evidence(&network, hard_reject, reconsiderable, true);
}
