// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net.cpp
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/test/functional/p2p_compactblocks.py

use std::{
    cell::RefCell,
    collections::VecDeque,
    io::{self, Read},
    rc::Rc,
};

use open_bitcoin_codec::{
    BIP152_COMPACT_BLOCKS_VERSION, CompactBlockPayload, PrefilledTransaction, SendCompactMessage,
};
use open_bitcoin_core::consensus::{
    block_hash, block_merkle_root, transaction_txid, transaction_wtxid,
};
use open_bitcoin_network::{
    BlockRelayActivationPolicy, BlockServingActivationConfig,
    COMPACT_BLOCK_DOWNLOAD_TIMEOUT_SECONDS, CompactRelayActivationConfig,
};

use super::*;
use crate::sync::tcp::{ReadStageOutcome, read_stage};

#[derive(Debug)]
enum ReadAction {
    Bytes(Vec<u8>),
    Error(io::ErrorKind),
    Eof,
}

#[derive(Debug)]
struct ScriptedReader {
    actions: VecDeque<ReadAction>,
}

impl ScriptedReader {
    fn new(actions: Vec<ReadAction>) -> Self {
        Self {
            actions: actions.into(),
        }
    }
}

impl Read for ScriptedReader {
    fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
        match self.actions.pop_front().unwrap_or(ReadAction::Eof) {
            ReadAction::Bytes(bytes) => {
                let copied = bytes.len().min(buffer.len());
                buffer[..copied].copy_from_slice(&bytes[..copied]);
                Ok(copied)
            }
            ReadAction::Error(kind) => Err(io::Error::from(kind)),
            ReadAction::Eof => Ok(0),
        }
    }
}

#[derive(Debug)]
struct TimingTransport {
    maybe_outcomes: Option<VecDeque<SyncPeerReceiveOutcome>>,
    sent: Rc<RefCell<Vec<WireNetworkMessage>>>,
}

impl TimingTransport {
    fn new(outcomes: Vec<SyncPeerReceiveOutcome>) -> Self {
        Self {
            maybe_outcomes: Some(outcomes.into()),
            sent: Rc::new(RefCell::new(Vec::new())),
        }
    }

    fn sent_messages(&self) -> Vec<WireNetworkMessage> {
        self.sent.borrow().clone()
    }
}

#[derive(Debug)]
struct TimingSession {
    outcomes: VecDeque<SyncPeerReceiveOutcome>,
    sent: Rc<RefCell<Vec<WireNetworkMessage>>>,
}

#[derive(Debug)]
struct PerpetualIdleSession {
    receive_calls: Rc<RefCell<usize>>,
}

impl SyncTransport for TimingTransport {
    type Session = TimingSession;

    fn connect(
        &mut self,
        _peer: &ResolvedSyncPeerAddress,
        _config: &SyncRuntimeConfig,
    ) -> Result<Self::Session, SyncRuntimeError> {
        Ok(TimingSession {
            outcomes: self.maybe_outcomes.take().unwrap_or_default(),
            sent: Rc::clone(&self.sent),
        })
    }
}

impl SyncPeerSession for TimingSession {
    fn send(
        &mut self,
        message: &WireNetworkMessage,
        _magic: open_bitcoin_core::primitives::NetworkMagic,
    ) -> Result<(), SyncRuntimeError> {
        self.sent.borrow_mut().push(message.clone());
        Ok(())
    }

    fn receive(
        &mut self,
        _magic: open_bitcoin_core::primitives::NetworkMagic,
    ) -> Result<SyncPeerReceiveOutcome, SyncRuntimeError> {
        Ok(self
            .outcomes
            .pop_front()
            .unwrap_or(SyncPeerReceiveOutcome::Closed))
    }
}

impl SyncPeerSession for PerpetualIdleSession {
    fn send(
        &mut self,
        _message: &WireNetworkMessage,
        _magic: open_bitcoin_core::primitives::NetworkMagic,
    ) -> Result<(), SyncRuntimeError> {
        Ok(())
    }

    fn receive(
        &mut self,
        _magic: open_bitcoin_core::primitives::NetworkMagic,
    ) -> Result<SyncPeerReceiveOutcome, SyncRuntimeError> {
        let call = *self.receive_calls.borrow();
        *self.receive_calls.borrow_mut() = call.saturating_add(1);
        Ok(match call {
            0 => SyncPeerReceiveOutcome::Message(version_message()),
            1 => SyncPeerReceiveOutcome::Message(WireNetworkMessage::Verack),
            _ => SyncPeerReceiveOutcome::Idle,
        })
    }
}

fn timing_runtime(path: &std::path::Path, max_messages_per_peer: usize) -> DurableSyncRuntime {
    let store = FjallNodeStore::open(path).expect("store");
    DurableSyncRuntime::open_with_block_relay_activation(
        store,
        SyncRuntimeConfig {
            max_messages_per_peer,
            max_peer_retries: 0,
            ..sync_config()
        },
        enabled_block_relay_activation(),
    )
    .expect("runtime")
}

fn timing_resolver() -> ScriptedResolver {
    ScriptedResolver::new(vec![Ok(vec![resolved_manual_peer("127.0.0.1", 18_444)])])
}

fn version_message() -> WireNetworkMessage {
    WireNetworkMessage::Version(VersionMessage::default())
}

fn enabled_block_relay_activation() -> BlockRelayActivationPolicy {
    BlockRelayActivationPolicy {
        block_serving: BlockServingActivationConfig { enabled: true },
        compact_relay: CompactRelayActivationConfig { enabled: true },
    }
}

fn compact_block_fixture(runtime: &mut DurableSyncRuntime) -> Block {
    let genesis = build_block(BlockHash::from_byte_array([0_u8; 32]), 0);
    let spendable = build_block(block_hash(&genesis.header), 1);
    runtime
        .network
        .connect_local_block(&genesis, runtime.verify_flags, runtime.consensus_params)
        .expect("connect genesis");
    runtime
        .network
        .connect_local_block(&spendable, runtime.verify_flags, runtime.consensus_params)
        .expect("connect spendable");

    let mut announced = build_block(block_hash(&spendable.header), 2);
    announced.transactions.push(coinbase_transaction(3, 25));
    let (merkle_root, maybe_mutated) =
        block_merkle_root(&announced.transactions).expect("merkle root");
    assert!(!maybe_mutated);
    announced.header.merkle_root = merkle_root;
    mine_header(&mut announced);
    announced
}

fn connectable_compact_block_fixture(runtime: &mut DurableSyncRuntime) -> Block {
    runtime.consensus_params.coinbase_maturity = 1;
    let genesis = build_block(BlockHash::from_byte_array([0_u8; 32]), 0);
    let spendable = build_block(block_hash(&genesis.header), 1);
    runtime
        .network
        .connect_local_block(&genesis, runtime.verify_flags, runtime.consensus_params)
        .expect("connect genesis");
    runtime
        .network
        .connect_local_block(&spendable, runtime.verify_flags, runtime.consensus_params)
        .expect("connect spendable");

    let previous_txid = transaction_txid(&spendable.transactions[0]).expect("coinbase txid");
    let spend = Transaction {
        version: 1,
        inputs: vec![TransactionInput {
            previous_output: OutPoint {
                txid: previous_txid,
                vout: 0,
            },
            script_sig: script(&[]),
            sequence: TransactionInput::SEQUENCE_FINAL,
            witness: ScriptWitness::default(),
        }],
        outputs: vec![TransactionOutput {
            value: Amount::from_sats(25).expect("valid amount"),
            script_pubkey: script(&[0x51]),
        }],
        lock_time: 0,
    };
    let mut announced = build_block(block_hash(&spendable.header), 2);
    announced.transactions.push(spend);
    let (merkle_root, maybe_mutated) =
        block_merkle_root(&announced.transactions).expect("merkle root");
    assert!(!maybe_mutated);
    announced.header.merkle_root = merkle_root;
    mine_header(&mut announced);
    announced
}

fn compact_download_script(block: &Block) -> Vec<SyncPeerReceiveOutcome> {
    vec![
        SyncPeerReceiveOutcome::Message(version_message()),
        SyncPeerReceiveOutcome::Message(WireNetworkMessage::Verack),
        SyncPeerReceiveOutcome::Message(send_compact_message()),
        SyncPeerReceiveOutcome::Message(WireNetworkMessage::CompactBlock(compact_payload(block))),
        SyncPeerReceiveOutcome::Idle,
        SyncPeerReceiveOutcome::Closed,
    ]
}

fn send_compact_message() -> WireNetworkMessage {
    WireNetworkMessage::SendCompact(SendCompactMessage {
        announce: true,
        version: BIP152_COMPACT_BLOCKS_VERSION,
    })
}

fn compact_payload(block: &Block) -> CompactBlockPayload {
    let nonce = 17;
    let selector =
        open_bitcoin_codec::short_id_selector_from_header_and_nonce(&block.header, nonce);
    let wtxid = transaction_wtxid(&block.transactions[1]).expect("wtxid");
    CompactBlockPayload {
        header: block.header.clone(),
        nonce,
        short_ids: vec![open_bitcoin_core::consensus::compact_short_id_for_wtxid(
            selector, &wtxid,
        )],
        prefilled_transactions: vec![PrefilledTransaction {
            index_delta: 0,
            transaction: block.transactions[0].clone(),
        }],
    }
}

fn start_other_peer_compact_download(
    runtime: &mut DurableSyncRuntime,
    peer_id: PeerId,
    block: &Block,
    timestamp: i64,
) {
    runtime
        .network
        .connect_outbound_peer(peer_id, timestamp)
        .expect("connect other peer");
    for message in [
        version_message(),
        WireNetworkMessage::Verack,
        send_compact_message(),
    ] {
        runtime
            .network
            .receive_sync_message(
                peer_id,
                message,
                timestamp,
                runtime.verify_flags,
                runtime.consensus_params,
            )
            .expect("other peer handshake");
    }
    let outbound = runtime
        .network
        .receive_sync_message(
            peer_id,
            WireNetworkMessage::CompactBlock(compact_payload(block)),
            timestamp,
            runtime.verify_flags,
            runtime.consensus_params,
        )
        .expect("other peer compact download")
        .outbound;
    assert!(
        outbound
            .iter()
            .any(|message| { matches!(message, WireNetworkMessage::GetBlockTxn(_)) })
    );
}

fn is_full_block_getdata(message: &WireNetworkMessage) -> bool {
    matches!(
        message,
        WireNetworkMessage::GetData(inventory)
            if inventory.inventory.iter().any(|item| item.inventory_type == InventoryType::Block)
    )
}

fn is_full_block_getdata_for_hash(message: &WireNetworkMessage, expected_hash: BlockHash) -> bool {
    matches!(
        message,
        WireNetworkMessage::GetData(inventory)
            if inventory.inventory.iter().any(|item| {
                item.inventory_type == InventoryType::Block
                    && BlockHash::from(item.object_hash) == expected_hash
            })
    )
}

mod compact_timeout;
mod idle_sessions;
mod session_boundaries;
mod tcp_read_semantics;
