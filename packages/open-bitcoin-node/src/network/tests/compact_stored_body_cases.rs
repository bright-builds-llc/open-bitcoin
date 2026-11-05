// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/test/functional/p2p_compactblocks.py

use open_bitcoin_codec::BlockTransactions;
use open_bitcoin_core::chainstate::AnchoredBlock;
use open_bitcoin_core::consensus::chain_proof::ProofWork;
use open_bitcoin_core::{consensus::block_hash, primitives::BlockHash};
use open_bitcoin_mempool::PolicyConfig;
use open_bitcoin_mempool::{PolicyTime, ReorgLifecycleContext};
use open_bitcoin_network::WireNetworkMessage;

use super::compact_cleanup_cases::{
    assert_in_flight, assert_no_in_flight_hash, build_block_with_transactions,
    compact_payload_from_block, handshake_and_sendcmpct, start_shared_compact_on_peer, tip_chain,
    txid,
};
use super::{
    BlockConnectDisposition, ManagedPeerNetwork, MemoryChainstateStore, build_block,
    compact_relay_enabled_managed_network, consensus_params, local_config, spend_transaction,
    verify_flags,
};

#[test]
fn phase167_compact_stored_body_invalid_full_receipt_preserves_other_requests() {
    // Arrange
    let sender = 167_511;
    let other_requester = 167_512;
    let unrelated_requester = 167_513;
    let mut network = compact_relay_enabled_managed_network(sender);
    let (_, spendable) = tip_chain(&mut network);
    let spend = spend_transaction(txid(&spendable.transactions[0]), 499_999_000);
    let block = build_block_with_transactions(block_hash(&spendable.header), 2, vec![spend]);
    let hash = block_hash(&block.header);
    start_shared_compact_on_peer(
        &mut network,
        sender,
        compact_payload_from_block(&block, 17),
        1_000,
    );
    start_shared_compact_on_peer(
        &mut network,
        other_requester,
        compact_payload_from_block(&block, 19),
        1_000,
    );
    let unrelated_spend = spend_transaction(txid(&spendable.transactions[0]), 499_998_000);
    let unrelated =
        build_block_with_transactions(block_hash(&spendable.header), 3, vec![unrelated_spend]);
    let unrelated_hash = block_hash(&unrelated.header);
    start_shared_compact_on_peer(
        &mut network,
        unrelated_requester,
        compact_payload_from_block(&unrelated, 23),
        1_000,
    );
    assert_in_flight(&network, other_requester, hash);
    assert_in_flight(&network, unrelated_requester, unrelated_hash);
    assert_eq!(
        network.peer_manager().block_request_peer_order(hash),
        vec![sender, other_requester]
    );

    // Act
    let mut invalid = block;
    invalid.transactions.pop();
    let result = network.receive_message(
        sender,
        WireNetworkMessage::Block(invalid),
        1_001,
        verify_flags(),
        consensus_params(),
    );

    // Assert
    assert!(result.is_err(), "invalid body must not connect");
    assert_eq!(
        network.peer_manager().block_request_peer_order(hash),
        vec![other_requester]
    );
    assert_in_flight(&network, other_requester, hash);
    assert_eq!(
        network
            .peer_manager()
            .block_request_peer_order(unrelated_hash),
        vec![unrelated_requester]
    );
    assert_in_flight(&network, unrelated_requester, unrelated_hash);
}

#[test]
fn phase167_compact_stored_body_connected_completion_clears_only_matching_requests() {
    // Arrange
    let sender = 167_521;
    let other_requester = 167_522;
    let unrelated_requester = 167_523;
    let mut network = compact_relay_enabled_managed_network(sender);
    let (_, spendable) = tip_chain(&mut network);
    let spend = spend_transaction(txid(&spendable.transactions[0]), 499_999_000);
    let block = build_block_with_transactions(block_hash(&spendable.header), 2, vec![spend]);
    let hash = block_hash(&block.header);
    let receipt_time = i64::from(block.header.time);
    let unrelated_spend = spend_transaction(txid(&spendable.transactions[0]), 499_998_000);
    let unrelated =
        build_block_with_transactions(block_hash(&spendable.header), 3, vec![unrelated_spend]);
    let unrelated_hash = block_hash(&unrelated.header);
    start_shared_compact_on_peer(
        &mut network,
        sender,
        compact_payload_from_block(&block, 17),
        receipt_time,
    );
    start_shared_compact_on_peer(
        &mut network,
        other_requester,
        compact_payload_from_block(&block, 19),
        receipt_time,
    );
    start_shared_compact_on_peer(
        &mut network,
        unrelated_requester,
        compact_payload_from_block(&unrelated, 23),
        receipt_time,
    );
    assert_eq!(
        network.peer_manager().block_request_peer_order(hash),
        vec![sender, other_requester]
    );

    // Act
    let result = network.receive_message(
        sender,
        WireNetworkMessage::BlockTxn(BlockTransactions {
            block_hash: hash,
            transactions: vec![block.transactions[1].clone()],
        }),
        receipt_time + 1,
        verify_flags(),
        consensus_params(),
    );

    // Assert
    assert!(result.is_ok(), "actual connected body: {result:?}");
    assert_eq!(
        network.peer_manager().block_request_peer_order(hash),
        Vec::<u64>::new()
    );
    assert_eq!(
        network
            .peer_manager()
            .block_request_peer_order(unrelated_hash),
        vec![unrelated_requester]
    );
    assert_in_flight(&network, unrelated_requester, unrelated_hash);
}

#[test]
fn phase167_compact_stored_body_local_success_and_repeat_finish_preserve_unrelated() {
    // Arrange
    let mut network = compact_relay_enabled_managed_network(167_571);
    let (_, spendable) = tip_chain(&mut network);
    let spend = spend_transaction(txid(&spendable.transactions[0]), 499_999_000);
    let block = build_block_with_transactions(block_hash(&spendable.header), 2, vec![spend]);
    let hash = block_hash(&block.header);
    let other = build_block_with_transactions(
        block_hash(&spendable.header),
        3,
        vec![spend_transaction(
            txid(&spendable.transactions[0]),
            499_998_000,
        )],
    );
    let other_hash = block_hash(&other.header);
    let now = i64::from(block.header.time);
    start_shared_compact_on_peer(
        &mut network,
        167_571,
        compact_payload_from_block(&block, 17),
        now,
    );
    start_shared_compact_on_peer(
        &mut network,
        167_572,
        compact_payload_from_block(&block, 19),
        now,
    );
    start_shared_compact_on_peer(
        &mut network,
        167_573,
        compact_payload_from_block(&other, 23),
        now,
    );
    assert_eq!(
        network.peer_manager().block_request_peer_order(hash),
        vec![167_571, 167_572]
    );

    // Act
    network
        .connect_local_block(&block, verify_flags(), consensus_params())
        .expect("actual local retained commit");
    let first = network.block_relay_evidence_status();
    assert!(network.finish_bookkeeping(&block));

    // Assert
    assert_eq!(network.block_relay_evidence_status(), first);
    for peer in [167_571, 167_572] {
        assert!(
            !network
                .peer_manager()
                .peer_state(peer)
                .expect("live peer")
                .requested_blocks
                .contains(&hash)
        );
        assert_no_in_flight_hash(&network, peer, hash);
    }
    assert!(
        network
            .peer_manager()
            .block_request_peer_order(hash)
            .is_empty()
    );
    assert_eq!(
        network.peer_manager().block_request_peer_order(other_hash),
        vec![167_573]
    );
    assert_in_flight(&network, 167_573, other_hash);
}

#[test]
fn phase167_compact_stored_body_unknown_parent_nonextending_keeps_requests() {
    // Arrange
    let mut network = ManagedPeerNetwork::new(
        MemoryChainstateStore::default(),
        local_config(167_581),
        PolicyConfig::default(),
    );
    let genesis = build_block(BlockHash::from_byte_array([0; 32]), 0, 500_000_000);
    network
        .connect_local_block(&genesis, verify_flags(), consensus_params())
        .expect("actual accepted genesis");
    let unknown = build_block(BlockHash::from_byte_array([42; 32]), 1, 500_000_000);
    let hash = block_hash(&unknown.header);
    let unrelated = BlockHash::from_byte_array([67; 32]);
    handshake_and_sendcmpct(&mut network, 167_581);
    for target in [hash, unrelated] {
        assert!(
            network
                .peer_manager_mut()
                .request_missing_blocks(167_581, &[target])
                .expect("actual ordinary request")
                .is_some()
        );
    }

    // Act
    let outcome = network.connect_stored_block(
        &unknown,
        open_bitcoin_core::chainstate::ChainPosition::untrusted_fixture_work(2),
        i64::from(unknown.header.time),
        verify_flags(),
        consensus_params(),
    );

    // Assert
    assert!(
        matches!(outcome, Ok(BlockConnectDisposition::NonExtending { .. })),
        "{outcome:?}"
    );
    assert_eq!(
        network.peer_manager().block_request_peer_order(hash),
        vec![167_581]
    );
    assert_eq!(
        network.peer_manager().block_request_peer_order(unrelated),
        vec![167_581]
    );
}

#[test]
fn phase167_compact_stored_body_unknown_parent_disconnected_keeps_requests() {
    // Arrange
    let mut network = ManagedPeerNetwork::new(
        MemoryChainstateStore::default(),
        local_config(167_591),
        PolicyConfig::default(),
    );
    let disconnected = build_block(BlockHash::from_byte_array([7; 32]), 1, 500_000_000);
    let hash = block_hash(&disconnected.header);
    handshake_and_sendcmpct(&mut network, 167_591);
    assert!(
        network
            .peer_manager_mut()
            .request_missing_blocks(167_591, &[hash])
            .expect("actual ordinary request")
            .is_some()
    );

    // Act
    let outcome = network.connect_stored_block(
        &disconnected,
        open_bitcoin_core::chainstate::ChainPosition::untrusted_fixture_work(1),
        i64::from(disconnected.header.time),
        verify_flags(),
        consensus_params(),
    );

    // Assert
    assert!(
        matches!(outcome, Ok(BlockConnectDisposition::Disconnected { .. })),
        "{outcome:?}"
    );
    assert_eq!(
        network.peer_manager().block_request_peer_order(hash),
        vec![167_591]
    );
}

fn retained_fork(
    side_len: u32,
) -> (
    ManagedPeerNetwork<MemoryChainstateStore>,
    Vec<open_bitcoin_core::primitives::Block>,
    Vec<open_bitcoin_core::primitives::Block>,
) {
    let mut network = ManagedPeerNetwork::new(
        MemoryChainstateStore::default(),
        local_config(167_601),
        PolicyConfig::default(),
    );
    let genesis = build_block(BlockHash::default(), 0, 5_000_000_000);
    let active_child = build_block(block_hash(&genesis.header), 1, 5_000_000_000);
    for block in [&genesis, &active_child] {
        network
            .connect_local_block(block, verify_flags(), consensus_params())
            .expect("genuine active prefix");
    }
    let mut parent = block_hash(&genesis.header);
    let mut side = Vec::new();
    for height in 1..=side_len {
        let block = build_block(parent, height, 4_000_000_000);
        network
            .connect_stored_block(
                &block,
                ProofWork::ZERO,
                2_000_000_000,
                verify_flags(),
                consensus_params(),
            )
            .expect("real retained side body");
        parent = block_hash(&block.header);
        side.push(block);
    }
    network
        .peer_manager_mut()
        .set_block_relay_activation_policy(open_bitcoin_network::BlockRelayActivationPolicy {
            compact_relay: open_bitcoin_network::CompactRelayActivationConfig { enabled: true },
            ..Default::default()
        });
    (network, vec![genesis, active_child], side)
}

fn coinbase_compact(
    block: &open_bitcoin_core::primitives::Block,
) -> open_bitcoin_codec::CompactBlockPayload {
    let selector = open_bitcoin_codec::short_id_selector_from_header_and_nonce(&block.header, 1);
    let wtxid = open_bitcoin_core::consensus::transaction_wtxid(&block.transactions[0])
        .expect("coinbase wtxid");
    open_bitcoin_codec::CompactBlockPayload {
        header: block.header.clone(),
        nonce: 1,
        short_ids: vec![open_bitcoin_core::consensus::compact_short_id_for_wtxid(
            selector, &wtxid,
        )],
        prefilled_transactions: vec![],
    }
}

#[test]
fn phase167_compact_stored_body_reorg_commit_clears_connected_replacements_only() {
    // Arrange
    let (mut network, active, side) = retained_fork(2);
    let hash = block_hash(&side[1].header);
    start_shared_compact_on_peer(
        &mut network,
        167_601,
        coinbase_compact(&side[1]),
        2_000_000_000,
    );
    let unrelated = BlockHash::from_byte_array([0x60; 32]);
    assert!(
        network
            .peer_manager_mut()
            .request_missing_blocks(167_601, &[unrelated])
            .expect("unrelated request")
            .is_some()
    );
    let replacements: Vec<_> = side
        .iter()
        .cloned()
        .map(|block| AnchoredBlock {
            block,
            chain_work: ProofWork::ZERO,
        })
        .collect();

    // Act
    let result = network.reorg_to_branch(
        &[active[1].clone()],
        &replacements,
        ReorgLifecycleContext::new(PolicyTime::from_unix_seconds(2_000_000_000)),
        verify_flags(),
        consensus_params(),
    );

    // Assert
    assert!(result.is_ok(), "genuine preferred reorg: {result:?}");
    assert_eq!(network.maybe_chain_tip().expect("new tip").block_hash, hash);
    assert!(
        !network
            .peer_manager()
            .peer_state(167_601)
            .expect("live peer")
            .requested_blocks
            .contains(&hash)
    );
    assert_no_in_flight_hash(&network, 167_601, hash);
    assert!(
        network
            .peer_manager()
            .block_request_peer_order(hash)
            .is_empty()
    );
    assert_eq!(
        network.peer_manager().block_request_peer_order(unrelated),
        vec![167_601]
    );
}

#[test]
fn phase167_compact_stored_body_reorg_refusal_preserves_matching_requests() {
    // Arrange
    let (mut network, active, side) = retained_fork(1);
    let hash = block_hash(&side[0].header);
    start_shared_compact_on_peer(
        &mut network,
        167_611,
        coinbase_compact(&side[0]),
        2_000_000_000,
    );
    let unrelated = BlockHash::from_byte_array([0x61; 32]);
    assert!(
        network
            .peer_manager_mut()
            .request_missing_blocks(167_611, &[unrelated])
            .expect("unrelated request")
            .is_some()
    );
    let before = network
        .chainstate_snapshot()
        .expect("complete pre-refusal state");
    let replacement = [AnchoredBlock {
        block: side[0].clone(),
        chain_work: ProofWork::ZERO,
    }];

    // Act
    let result = network.reorg_to_branch(
        &[active[1].clone()],
        &replacement,
        ReorgLifecycleContext::new(PolicyTime::from_unix_seconds(2_000_000_000)),
        verify_flags(),
        consensus_params(),
    );

    // Assert
    assert!(result.is_err(), "later equal-work side must not win");
    assert_eq!(
        network.chainstate_snapshot().expect("post-refusal state"),
        before
    );
    assert_eq!(
        network.peer_manager().block_request_peer_order(hash),
        vec![167_611]
    );
    assert_eq!(
        network.peer_manager().block_request_peer_order(unrelated),
        vec![167_611]
    );
}

#[test]
fn phase167_compact_stored_body_local_failure_preserves_other_requester() {
    // Arrange
    let mut network = compact_relay_enabled_managed_network(167_621);
    let (_, spendable) = tip_chain(&mut network);
    let spend = spend_transaction(txid(&spendable.transactions[0]), 499_999_000);
    let block = build_block_with_transactions(block_hash(&spendable.header), 2, vec![spend]);
    let hash = block_hash(&block.header);
    start_shared_compact_on_peer(
        &mut network,
        167_621,
        compact_payload_from_block(&block, 17),
        1_000,
    );
    let unrelated = BlockHash::from_byte_array([0x62; 32]);
    assert!(
        network
            .peer_manager_mut()
            .request_missing_blocks(167_621, &[unrelated])
            .expect("unrelated request")
            .is_some()
    );
    let before = network.chainstate_snapshot().expect("accepted before");
    let mut invalid = block;
    invalid.transactions.pop();

    // Act
    let result = network.connect_local_block(&invalid, verify_flags(), consensus_params());

    // Assert
    assert!(result.is_err(), "mutated local body cannot connect");
    assert_eq!(
        network.chainstate_snapshot().expect("accepted after"),
        before
    );
    assert_eq!(
        network.peer_manager().block_request_peer_order(hash),
        vec![167_621]
    );
    assert_eq!(
        network.peer_manager().block_request_peer_order(unrelated),
        vec![167_621]
    );
}

#[test]
fn phase167_compact_stored_body_witness_mutated_duplicate_refusal_retains_original() {
    // Arrange
    let mut network = compact_relay_enabled_managed_network(167_631);
    let (_, spendable) = tip_chain(&mut network);
    let spend = spend_transaction(txid(&spendable.transactions[0]), 499_999_000);
    let block = build_block_with_transactions(block_hash(&spendable.header), 2, vec![spend]);
    let hash = block_hash(&block.header);
    network
        .connect_local_block(&block, verify_flags(), consensus_params())
        .expect("genuine accepted body");
    start_shared_compact_on_peer(
        &mut network,
        167_631,
        compact_payload_from_block(&block, 17),
        i64::from(block.header.time),
    );
    let unrelated = BlockHash::from_byte_array([0x63; 32]);
    assert!(
        network
            .peer_manager_mut()
            .request_missing_blocks(167_631, &[unrelated])
            .expect("unrelated request")
            .is_some()
    );
    let mut altered = block.clone();
    altered.transactions[0].inputs[0].witness =
        open_bitcoin_core::primitives::ScriptWitness::new(vec![vec![7]]);
    let before = network
        .chainstate_snapshot()
        .expect("accepted before duplicate");

    // Act
    let result = network.connect_stored_block(
        &altered,
        ProofWork::ZERO,
        i64::from(altered.header.time),
        verify_flags(),
        consensus_params(),
    );

    // Assert
    assert!(
        matches!(&result, Err(crate::ManagedNetworkError::Network(
        open_bitcoin_network::NetworkError::InvalidHeader { reject_reason, .. }))
        if reject_reason == "unexpected-witness"),
        "{result:?}"
    );
    assert_eq!(
        network
            .chainstate_snapshot()
            .expect("accepted after duplicate"),
        before
    );
    assert_eq!(
        network.peer_manager().block_request_peer_order(hash),
        vec![167_631]
    );
    assert_eq!(
        network.peer_manager().block_request_peer_order(unrelated),
        vec![167_631]
    );
    assert_eq!(network.blocks_by_hash.get(&hash), Some(&block));
}
