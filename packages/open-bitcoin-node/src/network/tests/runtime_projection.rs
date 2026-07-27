// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/node/txdownloadman_impl.cpp
// - packages/bitcoin-knots/src/node/txdownloadman.h
// - packages/bitcoin-knots/src/node/mempool_persist.cpp
// - packages/bitcoin-knots/src/protocol.h
// - packages/bitcoin-knots/src/txorphanage.cpp
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/test/functional/mempool_persist.py
// - packages/bitcoin-knots/test/functional/p2p_getdata.py
// - packages/bitcoin-knots/test/functional/p2p_orphan_handling.py
// - packages/bitcoin-knots/test/functional/p2p_tx_download.py
// - packages/bitcoin-knots/test/functional/mempool_accept.py

use super::*;

#[test]
fn managed_network_requests_transactions_using_wtxidrelay_when_negotiated() {
    let mut network = ManagedPeerNetwork::new(
        MemoryChainstateStore::default(),
        local_config(1),
        PolicyConfig::default(),
    );
    network.add_inbound_peer(1).expect("peer");
    network
        .receive_message(
            1,
            WireNetworkMessage::Version(open_bitcoin_network::VersionMessage::default()),
            1,
            verify_flags(),
            consensus_params(),
        )
        .expect("version");
    network
        .receive_message(
            1,
            WireNetworkMessage::WtxidRelay,
            1,
            verify_flags(),
            consensus_params(),
        )
        .expect("wtxidrelay");

    let genesis = build_block(BlockHash::from_byte_array([0_u8; 32]), 0, 500_000_000);
    let spendable = build_block(
        open_bitcoin_core::consensus::block_hash(&genesis.header),
        1,
        500_000_000,
    );
    network
        .connect_local_block(&genesis, verify_flags(), consensus_params())
        .expect("genesis");
    network
        .connect_local_block(&spendable, verify_flags(), consensus_params())
        .expect("spendable");

    let transaction = spend_transaction(
        transaction_txid(&genesis.transactions[0]).expect("txid"),
        499_999_000,
    );
    network
        .submit_local_transaction_outcome_at(
            transaction.clone(),
            verify_flags(),
            consensus_params(),
            2,
            RelayIntent::Requested,
        )
        .expect("admit");

    let message = network
        .announce_transaction(1, &transaction)
        .expect("announce")
        .expect("message");
    assert!(matches!(
        message,
        WireNetworkMessage::Inv(InventoryList { inventory })
        if inventory[0].inventory_type == InventoryType::WitnessTransaction
    ));
}

#[test]
fn managed_network_info_exposes_rpc_projection_helpers() {
    // Arrange
    let mut network = ManagedPeerNetwork::new(
        MemoryChainstateStore::default(),
        local_config(100),
        PolicyConfig::default(),
    );
    let genesis = build_block(BlockHash::from_byte_array([0_u8; 32]), 0, 500_000_000);
    let spendable = build_block(
        open_bitcoin_core::consensus::block_hash(&genesis.header),
        1,
        500_000_000,
    );
    network
        .connect_local_block(&genesis, verify_flags(), consensus_params())
        .expect("genesis");
    network
        .connect_local_block(&spendable, verify_flags(), consensus_params())
        .expect("spendable");
    network.add_inbound_peer(7).expect("inbound peer");
    network
        .receive_message(
            7,
            WireNetworkMessage::WtxidRelay,
            1,
            verify_flags(),
            consensus_params(),
        )
        .expect("wtxidrelay");
    network
        .receive_message(
            7,
            WireNetworkMessage::SendHeaders,
            1,
            verify_flags(),
            consensus_params(),
        )
        .expect("sendheaders");
    network.connect_outbound_peer(8, 2).expect("outbound peer");
    let transaction = spend_transaction(
        transaction_txid(&genesis.transactions[0]).expect("txid"),
        499_999_000,
    );
    let expected_virtual_size =
        open_bitcoin_mempool::transaction_weight_and_virtual_size(&transaction)
            .expect("weight")
            .1;
    network
        .submit_local_transaction_outcome_at(
            transaction,
            verify_flags(),
            consensus_params(),
            3,
            RelayIntent::NotRequested,
        )
        .expect("submit");

    // Act
    let snapshot = network.chainstate_snapshot();
    let maybe_tip = network.maybe_chain_tip();
    let mempool_info = network.mempool_info();
    let network_info = network.network_info();

    // Assert
    assert_eq!(snapshot.active_chain.len(), 2);
    assert_eq!(maybe_tip.expect("tip").height, 1);
    assert_eq!(mempool_info.transaction_count, 1);
    assert_eq!(mempool_info.total_virtual_size, expected_virtual_size);
    assert!(mempool_info.accounted_memory > mempool_info.total_virtual_size);
    assert_eq!(mempool_info.mempool_capacity, 300_000_000);
    assert_eq!(mempool_info.total_fee_sats, 1_000);
    assert_eq!(
        mempool_info.capacity_enforcement,
        MempoolCapacityEnforcement::AccountedMemory
    );
    assert_eq!(network_info.connected_peers, 2);
    assert_eq!(network_info.inbound_peers, 1);
    assert_eq!(network_info.outbound_peers, 1);
    assert_eq!(network_info.wtxidrelay_peers, 1);
    assert_eq!(network_info.header_preferring_peers, 1);
}

#[test]
fn mempool_info_exposes_truthful_resource_and_fee_roles() {
    // Arrange
    let policy = PolicyConfig {
        mempool_capacity: MempoolCapacity::new(12_345_678),
        static_relay_fee_rate: StaticRelayFeeRate::new(FeeRate::from_sats_per_kvb(1_000)),
        incremental_relay_fee_rate: IncrementalRelayFeeRate::new(FeeRate::from_sats_per_kvb(7_000)),
        ..PolicyConfig::default()
    };
    let mut network =
        ManagedPeerNetwork::new(MemoryChainstateStore::default(), local_config(91), policy);
    let genesis = build_block(BlockHash::from_byte_array([0_u8; 32]), 0, 500_000_000);
    let spendable = build_block(block_hash(&genesis.header), 1, 500_000_000);
    network
        .connect_local_block(&genesis, verify_flags(), consensus_params())
        .expect("genesis");
    network
        .connect_local_block(&spendable, verify_flags(), consensus_params())
        .expect("spendable");
    let transaction = script_heavy_spend_transaction(
        transaction_txid(&genesis.transactions[0]).expect("txid"),
        499_998_000,
    );
    let expected_virtual_size =
        open_bitcoin_mempool::transaction_weight_and_virtual_size(&transaction)
            .expect("weight")
            .1;
    network
        .submit_local_transaction_outcome_at(
            transaction,
            verify_flags(),
            consensus_params(),
            3,
            RelayIntent::NotRequested,
        )
        .expect("submit");
    network
        .set_rolling_mempool_fee_rate(RollingMempoolFeeRate::new(FeeRate::from_sats_per_kvb(
            3_000,
        )))
        .expect("revision remains available");

    // Act
    let info = network.mempool_info();
    let serialized = format!("{info:?}");

    // Assert
    assert_eq!(info.transaction_count, 1);
    assert_eq!(info.total_virtual_size, expected_virtual_size);
    assert!(info.accounted_memory > info.total_virtual_size);
    assert_eq!(info.mempool_capacity, 12_345_678);
    assert_ne!(info.total_virtual_size, info.accounted_memory);
    assert_ne!(info.accounted_memory, info.mempool_capacity);
    assert_ne!(info.total_virtual_size, info.mempool_capacity);
    assert_eq!(info.static_relay_fee_rate_sats_per_kvb, 1_000);
    assert_eq!(info.incremental_relay_fee_rate_sats_per_kvb, 7_000);
    assert_eq!(info.rolling_mempool_fee_rate_sats_per_kvb, 3_000);
    assert_eq!(info.effective_admission_fee_rate_sats_per_kvb, 3_000);
    assert_eq!(
        info.effective_admission_fee_rate_sats_per_kvb,
        info.static_relay_fee_rate_sats_per_kvb
            .max(info.rolling_mempool_fee_rate_sats_per_kvb)
    );
    assert_ne!(
        info.effective_admission_fee_rate_sats_per_kvb,
        info.incremental_relay_fee_rate_sats_per_kvb
    );
    assert_eq!(
        info.capacity_enforcement,
        MempoolCapacityEnforcement::AccountedMemory
    );
    assert_eq!(info.capacity_enforcement.as_str(), "accounted_memory");
    for forbidden in [
        "txid",
        "wtxid",
        "peer_id",
        "127.0.0.1",
        "script_sig",
        "transaction_hex",
    ] {
        assert!(
            !serialized.to_lowercase().contains(forbidden),
            "shared evidence leaked {forbidden}: {serialized}"
        );
    }
}

#[test]
fn managed_nodes_sync_blocks_and_relay_transactions_in_memory() {
    let mut source = block_serving_enabled_managed_network(10);
    let mut sink = relay_enabled_managed_network(20);

    let genesis = build_block(BlockHash::from_byte_array([0_u8; 32]), 0, 500_000_000);
    let spendable = build_block(
        open_bitcoin_core::consensus::block_hash(&genesis.header),
        1,
        500_000_000,
    );
    source
        .connect_local_block(&genesis, verify_flags(), consensus_params())
        .expect("genesis");
    source
        .connect_local_block(&spendable, verify_flags(), consensus_params())
        .expect("spendable");

    let source_admission = source.admit_inbound_peer(permissioned_inbound_request(
        7,
        "127.0.0.1:18447",
        &["in", "download", "relay", "mempool"],
    ));
    assert!(matches!(
        source_admission,
        InboundAdmissionDecision::Admit(_)
    ));
    let sync_timestamp = i64::from(spendable.header.time);
    let mut to_source = sink.connect_outbound_peer(7, 1).expect("connect");
    let mut to_sink = deliver(&sink, &mut source, 7, to_source, sync_timestamp);
    to_source = deliver(&source, &mut sink, 7, to_sink, sync_timestamp);
    to_sink = deliver(&sink, &mut source, 7, to_source, sync_timestamp);
    to_source = deliver(&source, &mut sink, 7, to_sink, sync_timestamp);
    to_sink = deliver(&sink, &mut source, 7, to_source, sync_timestamp);
    let final_outbound = deliver(&source, &mut sink, 7, to_sink, sync_timestamp);
    assert!(final_outbound.is_empty());
    assert_eq!(
        sink.chainstate().chainstate().tip().map(|tip| tip.height),
        Some(1)
    );

    let transaction = spend_transaction(
        transaction_txid(&genesis.transactions[0]).expect("txid"),
        499_999_000,
    );
    source
        .submit_local_transaction_outcome_at(
            transaction.clone(),
            verify_flags(),
            consensus_params(),
            7,
            RelayIntent::Requested,
        )
        .expect("source admit");

    let announced = source
        .announce_transaction(7, &transaction)
        .expect("announce")
        .expect("inv");
    let to_source = deliver(&source, &mut sink, 7, vec![announced], 8);
    let to_sink = deliver(&sink, &mut source, 7, to_source, 9);
    let final_messages = deliver(&source, &mut sink, 7, to_sink, 10);
    assert!(final_messages.is_empty());

    let txid = transaction_txid(&transaction).expect("txid");
    assert!(sink.mempool().mempool().entry(&txid).is_some());
}

#[test]
fn managed_network_rejects_future_block_using_message_timestamp() {
    let mut network = ManagedPeerNetwork::new(
        MemoryChainstateStore::default(),
        local_config(30),
        PolicyConfig::default(),
    );
    let genesis = build_block(BlockHash::from_byte_array([0_u8; 32]), 0, 500_000_000);
    network
        .connect_local_block(&genesis, verify_flags(), consensus_params())
        .expect("genesis");
    network.add_inbound_peer(9).expect("peer");

    let future_block = build_block(
        open_bitcoin_core::consensus::block_hash(&genesis.header),
        10_000,
        500_000_000,
    );
    let error = network
        .receive_message(
            9,
            WireNetworkMessage::Block(future_block.clone()),
            i64::from(future_block.header.time) - 7_201,
            verify_flags(),
            consensus_params(),
        )
        .expect_err("future block must use the message timestamp");

    assert!(matches!(
        error,
        crate::network::ManagedNetworkError::Chainstate(
            open_bitcoin_core::chainstate::ChainstateError::BlockValidation { source }
        ) if source.reject_reason == "time-too-new"
    ));
}
