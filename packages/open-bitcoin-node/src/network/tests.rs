// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/node/txdownloadman_impl.cpp
// - packages/bitcoin-knots/src/node/txdownloadman.h
// - packages/bitcoin-knots/src/protocol.h

use open_bitcoin_core::consensus::crypto::hash160;
use open_bitcoin_core::{
    consensus::{
        ConsensusParams, ScriptVerifyFlags, block_merkle_root, check_block_header, transaction_txid,
    },
    primitives::{
        Amount, Block, BlockHash, BlockHeader, InventoryType, NetworkAddress, NetworkMagic,
        OutPoint, ScriptBuf, ScriptWitness, Transaction, TransactionInput, TransactionOutput,
    },
};
use open_bitcoin_mempool::PolicyConfig;
use open_bitcoin_network::{InventoryList, LocalPeerConfig, ServiceFlags, WireNetworkMessage};

use crate::{ManagedPeerNetwork, MemoryChainstateStore};

const EASY_BITS: u32 = 0x207f_ffff;

fn script(bytes: &[u8]) -> ScriptBuf {
    ScriptBuf::from_bytes(bytes.to_vec()).expect("valid script")
}

fn redeem_script() -> ScriptBuf {
    script(&[0x51])
}

fn p2sh_script() -> ScriptBuf {
    let redeem_hash = hash160(redeem_script().as_bytes());
    let mut bytes = vec![0xa9, 20];
    bytes.extend_from_slice(&redeem_hash);
    bytes.push(0x87);
    script(&bytes)
}

fn serialized_script_num(value: i64) -> Vec<u8> {
    if value == 0 {
        return vec![0x00];
    }

    let mut magnitude = value as u64;
    let mut encoded = Vec::new();
    while magnitude > 0 {
        encoded.push((magnitude & 0xff) as u8);
        magnitude >>= 8;
    }

    let mut script = Vec::with_capacity(encoded.len() + 2);
    script.push(encoded.len() as u8);
    script.extend(encoded);
    script.push(0x51);
    script
}

fn coinbase_transaction(height: u32, value: i64) -> Transaction {
    let mut script_sig = serialized_script_num(i64::from(height));
    script_sig.push(0x51);
    Transaction {
        version: 1,
        inputs: vec![TransactionInput {
            previous_output: OutPoint::null(),
            script_sig: script(&script_sig),
            sequence: TransactionInput::SEQUENCE_FINAL,
            witness: ScriptWitness::default(),
        }],
        outputs: vec![TransactionOutput {
            value: Amount::from_sats(value).expect("valid amount"),
            script_pubkey: p2sh_script(),
        }],
        lock_time: 0,
    }
}

fn spend_transaction(
    previous_txid: open_bitcoin_core::primitives::Txid,
    value: i64,
) -> Transaction {
    Transaction {
        version: 2,
        inputs: vec![TransactionInput {
            previous_output: OutPoint {
                txid: previous_txid,
                vout: 0,
            },
            script_sig: script(&[0x01, 0x51]),
            sequence: TransactionInput::SEQUENCE_FINAL,
            witness: ScriptWitness::default(),
        }],
        outputs: vec![TransactionOutput {
            value: Amount::from_sats(value).expect("valid amount"),
            script_pubkey: p2sh_script(),
        }],
        lock_time: 0,
    }
}

fn mine_header(block: &mut Block) {
    block.header.nonce = (0..=u32::MAX)
        .find(|nonce| {
            block.header.nonce = *nonce;
            check_block_header(&block.header).is_ok()
        })
        .expect("expected nonce at easy target");
}

fn build_block(previous_block_hash: BlockHash, height: u32, value: i64) -> Block {
    let transactions = vec![coinbase_transaction(height, value)];
    let (merkle_root, maybe_mutated) = block_merkle_root(&transactions).expect("merkle root");
    assert!(!maybe_mutated);

    let mut block = Block {
        header: BlockHeader {
            version: 1,
            previous_block_hash,
            merkle_root,
            time: 1_231_006_500 + height,
            bits: EASY_BITS,
            nonce: 0,
        },
        transactions,
    };
    mine_header(&mut block);
    block
}

fn local_config(nonce: u64) -> LocalPeerConfig {
    LocalPeerConfig {
        magic: NetworkMagic::MAINNET,
        services: ServiceFlags::NETWORK | ServiceFlags::WITNESS,
        address: NetworkAddress {
            services: 0,
            address_bytes: [0_u8; 16],
            port: 8333,
        },
        nonce,
        relay: true,
        user_agent: "/open-bitcoin:test/".to_string(),
    }
}

fn verify_flags() -> ScriptVerifyFlags {
    ScriptVerifyFlags::P2SH
        | ScriptVerifyFlags::CHECKLOCKTIMEVERIFY
        | ScriptVerifyFlags::CHECKSEQUENCEVERIFY
}

fn consensus_params() -> ConsensusParams {
    ConsensusParams {
        coinbase_maturity: 1,
        ..ConsensusParams::default()
    }
}

fn deliver(
    sender: &ManagedPeerNetwork<MemoryChainstateStore>,
    receiver: &mut ManagedPeerNetwork<MemoryChainstateStore>,
    peer_id: u64,
    messages: Vec<WireNetworkMessage>,
    timestamp: i64,
) -> Vec<WireNetworkMessage> {
    let mut outbound = Vec::new();
    let encoded = sender.encode_messages(&messages).expect("encode");
    for bytes in encoded {
        outbound.extend(
            receiver
                .receive_wire_message(
                    peer_id,
                    &bytes,
                    timestamp,
                    verify_flags(),
                    consensus_params(),
                )
                .expect("receive"),
        );
    }
    outbound
}

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
        .submit_local_transaction(transaction.clone(), verify_flags(), consensus_params())
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
fn managed_network_exposes_rpc_projection_helpers() {
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
        .submit_local_transaction(transaction, verify_flags(), consensus_params())
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
    assert_eq!(mempool_info.total_fee_sats, 1_000);
    assert_eq!(network_info.connected_peers, 2);
    assert_eq!(network_info.inbound_peers, 1);
    assert_eq!(network_info.outbound_peers, 1);
    assert_eq!(network_info.wtxidrelay_peers, 1);
    assert_eq!(network_info.header_preferring_peers, 1);
}

#[test]
fn managed_nodes_sync_blocks_and_relay_transactions_in_memory() {
    let mut source = ManagedPeerNetwork::new(
        MemoryChainstateStore::default(),
        local_config(10),
        PolicyConfig::default(),
    );
    let mut sink = ManagedPeerNetwork::new(
        MemoryChainstateStore::default(),
        local_config(20),
        PolicyConfig::default(),
    );

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

    source.add_inbound_peer(7).expect("source peer");
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
        .submit_local_transaction(transaction.clone(), verify_flags(), consensus_params())
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
