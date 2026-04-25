// Parity breadcrumbs:
// - packages/bitcoin-knots/src/headerssync.cpp
// - packages/bitcoin-knots/src/sync.cpp
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/node/txdownloadman_impl.cpp
// - packages/bitcoin-knots/src/node/txdownloadman.h
// - packages/bitcoin-knots/test/functional/p2p_handshake.py
// - packages/bitcoin-knots/test/functional/p2p_initial_headers_sync.py

use std::collections::BTreeMap;

use open_bitcoin_chainstate::ChainPosition;
use open_bitcoin_consensus::{block_hash, block_merkle_root, check_block_header};
use open_bitcoin_network::{
    ConnectionRole, DisconnectReason, LocalPeerConfig, ParsedNetworkMessage, PeerAction,
    PeerManager, ServiceFlags, WireNetworkMessage,
};
use open_bitcoin_primitives::{
    Amount, Block, BlockHash, BlockHeader, InventoryType, NetworkAddress, NetworkMagic, OutPoint,
    ScriptBuf, ScriptWitness, Transaction, TransactionInput, TransactionOutput,
};

const EASY_BITS: u32 = 0x207f_ffff;

fn script(bytes: &[u8]) -> ScriptBuf {
    ScriptBuf::from_bytes(bytes.to_vec()).expect("valid script")
}

fn coinbase_transaction(height: u32, value: i64) -> Transaction {
    let mut script_sig = vec![0x01, height as u8, 0x51];
    if height > 255 {
        script_sig = vec![0x02, (height & 0xff) as u8, (height >> 8) as u8, 0x51];
    }

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
            script_pubkey: script(&[0x51]),
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

fn deliver(
    receiver: &mut PeerManager,
    messages: Vec<WireNetworkMessage>,
    timestamp: i64,
) -> Vec<PeerAction> {
    let mut actions = Vec::new();
    for message in messages {
        let encoded = message
            .encode_wire(NetworkMagic::MAINNET)
            .expect("wire encode");
        let parsed = ParsedNetworkMessage::decode_wire(&encoded).expect("wire decode");
        actions.extend(
            receiver
                .handle_message(7, parsed.message, timestamp)
                .expect("message should process"),
        );
    }
    actions
}

fn messages_from_actions(
    actions: Vec<PeerAction>,
    block_store: &BTreeMap<BlockHash, Block>,
) -> (Vec<WireNetworkMessage>, Vec<BlockHash>) {
    let mut messages = Vec::new();
    let mut received_blocks = Vec::new();
    let mut not_found = Vec::new();

    for action in actions {
        match action {
            PeerAction::Send(message) => messages.push(message),
            PeerAction::ServeInventory(requests) => {
                for request in requests {
                    match request.inventory_type {
                        InventoryType::Block | InventoryType::WitnessBlock => {
                            let hash = BlockHash::from(request.object_hash);
                            if let Some(block) = block_store.get(&hash) {
                                messages.push(WireNetworkMessage::Block(block.clone()));
                            } else {
                                not_found.push(request);
                            }
                        }
                        _ => not_found.push(request),
                    }
                }
            }
            PeerAction::ReceivedBlock(block) => received_blocks.push(block_hash(&block.header)),
            PeerAction::ReceivedTransaction(_) => {}
            PeerAction::Disconnect(reason) => panic!("unexpected disconnect: {reason}"),
        }
    }

    if !not_found.is_empty() {
        messages.push(WireNetworkMessage::NotFound(
            open_bitcoin_network::InventoryList::new(not_found),
        ));
    }

    (messages, received_blocks)
}

#[test]
fn wire_encoded_peer_managers_sync_headers_and_blocks() {
    let genesis = build_block(BlockHash::from_byte_array([0_u8; 32]), 0, 50);
    let child = build_block(block_hash(&genesis.header), 1, 50);
    let genesis_position = ChainPosition::new(genesis.header.clone(), 0, 1, 0);
    let child_position = ChainPosition::new(child.header.clone(), 1, 2, 0);

    let source_blocks = BTreeMap::from([
        (genesis_position.block_hash, genesis.clone()),
        (child_position.block_hash, child.clone()),
    ]);

    let mut source = PeerManager::new(local_config(1));
    source.seed_local_chain(&[genesis_position.clone(), child_position.clone()]);
    source.add_inbound_peer(7).expect("source peer");

    let mut sink = PeerManager::new(local_config(2));
    let outbound = sink.add_outbound_peer(7, 1).expect("outbound peer");
    let source_actions = deliver(
        &mut source,
        outbound
            .into_iter()
            .filter_map(|action| match action {
                PeerAction::Send(message) => Some(message),
                _ => None,
            })
            .collect(),
        2,
    );

    let (to_sink, _) = messages_from_actions(source_actions, &source_blocks);
    let sink_actions = deliver(&mut sink, to_sink, 3);
    let (to_source, _) = messages_from_actions(sink_actions, &source_blocks);
    let source_actions = deliver(&mut source, to_source, 4);
    let (to_sink, _) = messages_from_actions(source_actions, &source_blocks);
    let sink_actions = deliver(&mut sink, to_sink, 5);
    let (to_source, _) = messages_from_actions(sink_actions, &source_blocks);
    let source_actions = deliver(&mut source, to_source, 6);
    let (to_sink, _) = messages_from_actions(source_actions, &source_blocks);
    let sink_actions = deliver(&mut sink, to_sink, 7);
    let (_, received_blocks) = messages_from_actions(sink_actions, &source_blocks);

    assert_eq!(
        sink.peer_state(7).expect("sink state").role,
        ConnectionRole::Outbound,
    );
    assert_eq!(sink.header_store().best_height(), 1);
    assert_eq!(
        received_blocks,
        vec![genesis_position.block_hash, child_position.block_hash]
    );
    assert!(
        source
            .peer_state(7)
            .expect("source state")
            .remote_prefers_headers
    );
    assert!(
        source
            .peer_state(7)
            .expect("source state")
            .remote_verack_received
    );
}

#[test]
fn duplicate_version_requests_disconnect() {
    let mut manager = PeerManager::new(local_config(9));
    manager.add_inbound_peer(7).expect("peer");
    manager
        .handle_message(7, WireNetworkMessage::Version(Default::default()), 1)
        .expect("first version");

    let actions = manager
        .handle_message(7, WireNetworkMessage::Version(Default::default()), 2)
        .expect("duplicate version should not hard-fail");
    assert_eq!(
        actions,
        vec![PeerAction::Disconnect(DisconnectReason::DuplicateVersion)],
    );
}
