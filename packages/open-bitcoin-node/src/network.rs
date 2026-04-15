use std::collections::BTreeMap;

use open_bitcoin_core::{
    chainstate::{ChainPosition, ChainstateError},
    codec::CodecError,
    consensus::{
        ConsensusParams, ScriptVerifyFlags, block_hash, transaction_txid, transaction_wtxid,
    },
    primitives::{Block, BlockHash, InventoryType, Transaction, Txid, Wtxid},
};
use open_bitcoin_mempool::{AdmissionResult, MempoolError, PolicyConfig};
use open_bitcoin_network::{
    InventoryList, LocalPeerConfig, NetworkError, ParsedNetworkMessage, PeerAction, PeerId,
    PeerManager, WireNetworkMessage,
};

use crate::{ChainstateStore, ManagedChainstate, ManagedMempool};

#[derive(Debug)]
pub enum ManagedNetworkError {
    Network(NetworkError),
    Chainstate(ChainstateError),
    Mempool(MempoolError),
}

impl core::fmt::Display for ManagedNetworkError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Network(error) => error.fmt(f),
            Self::Chainstate(error) => error.fmt(f),
            Self::Mempool(error) => error.fmt(f),
        }
    }
}

impl std::error::Error for ManagedNetworkError {}

impl From<NetworkError> for ManagedNetworkError {
    fn from(value: NetworkError) -> Self {
        Self::Network(value)
    }
}

impl From<ChainstateError> for ManagedNetworkError {
    fn from(value: ChainstateError) -> Self {
        Self::Chainstate(value)
    }
}

impl From<MempoolError> for ManagedNetworkError {
    fn from(value: MempoolError) -> Self {
        Self::Mempool(value)
    }
}

impl From<CodecError> for ManagedNetworkError {
    fn from(value: CodecError) -> Self {
        Self::Network(NetworkError::from(value))
    }
}

#[derive(Debug, Clone)]
pub struct ManagedPeerNetwork<S> {
    chainstate: ManagedChainstate<S>,
    mempool: ManagedMempool,
    peer_manager: PeerManager,
    local_config: LocalPeerConfig,
    blocks_by_hash: BTreeMap<BlockHash, Block>,
    transactions_by_txid: BTreeMap<Txid, Transaction>,
    transactions_by_wtxid: BTreeMap<Wtxid, Transaction>,
}

impl<S: ChainstateStore> ManagedPeerNetwork<S> {
    pub fn new(store: S, local_config: LocalPeerConfig, mempool_config: PolicyConfig) -> Self {
        let chainstate = ManagedChainstate::from_store(store);
        let mut peer_manager = PeerManager::new(local_config.clone());
        peer_manager.seed_local_chain(&chainstate.chainstate().snapshot().active_chain);

        Self {
            chainstate,
            mempool: ManagedMempool::new(mempool_config),
            peer_manager,
            local_config,
            blocks_by_hash: BTreeMap::new(),
            transactions_by_txid: BTreeMap::new(),
            transactions_by_wtxid: BTreeMap::new(),
        }
    }

    pub fn chainstate(&self) -> &ManagedChainstate<S> {
        &self.chainstate
    }

    pub fn mempool(&self) -> &ManagedMempool {
        &self.mempool
    }

    pub fn peer_manager(&self) -> &PeerManager {
        &self.peer_manager
    }

    pub fn add_inbound_peer(&mut self, peer_id: PeerId) -> Result<(), ManagedNetworkError> {
        self.peer_manager.add_inbound_peer(peer_id)?;
        Ok(())
    }

    pub fn connect_outbound_peer(
        &mut self,
        peer_id: PeerId,
        timestamp: i64,
    ) -> Result<Vec<WireNetworkMessage>, ManagedNetworkError> {
        let actions = self.peer_manager.add_outbound_peer(peer_id, timestamp)?;
        self.collect_outbound(actions)
    }

    pub fn receive_message(
        &mut self,
        peer_id: PeerId,
        message: WireNetworkMessage,
        timestamp: i64,
        verify_flags: ScriptVerifyFlags,
        consensus_params: ConsensusParams,
    ) -> Result<Vec<WireNetworkMessage>, ManagedNetworkError> {
        let actions = self
            .peer_manager
            .handle_message(peer_id, message, timestamp)?;
        self.process_actions(actions, verify_flags, consensus_params)
    }

    pub fn receive_wire_message(
        &mut self,
        peer_id: PeerId,
        bytes: &[u8],
        timestamp: i64,
        verify_flags: ScriptVerifyFlags,
        consensus_params: ConsensusParams,
    ) -> Result<Vec<WireNetworkMessage>, ManagedNetworkError> {
        let parsed = ParsedNetworkMessage::decode_wire(bytes)?;
        self.receive_message(
            peer_id,
            parsed.message,
            timestamp,
            verify_flags,
            consensus_params,
        )
    }

    pub fn encode_messages(
        &self,
        messages: &[WireNetworkMessage],
    ) -> Result<Vec<Vec<u8>>, ManagedNetworkError> {
        messages
            .iter()
            .map(|message| message.encode_wire(self.local_config.magic))
            .collect::<Result<Vec<_>, _>>()
            .map_err(ManagedNetworkError::from)
    }

    pub fn connect_local_block(
        &mut self,
        block: &Block,
        verify_flags: ScriptVerifyFlags,
        consensus_params: ConsensusParams,
    ) -> Result<ChainPosition, ManagedNetworkError> {
        let position = self.chainstate.connect_block(
            block,
            self.next_chain_work(),
            verify_flags,
            consensus_params,
        )?;
        self.blocks_by_hash
            .insert(position.block_hash, block.clone());
        self.peer_manager.note_local_position(&position);
        Ok(position)
    }

    pub fn submit_local_transaction(
        &mut self,
        transaction: Transaction,
        verify_flags: ScriptVerifyFlags,
        consensus_params: ConsensusParams,
    ) -> Result<AdmissionResult, ManagedNetworkError> {
        let result = self.mempool.submit_transaction(
            &self.chainstate,
            transaction.clone(),
            verify_flags,
            consensus_params,
        )?;
        self.store_transaction(transaction)?;
        Ok(result)
    }

    pub fn announce_block(
        &self,
        peer_id: PeerId,
        block: &Block,
    ) -> Result<Option<WireNetworkMessage>, ManagedNetworkError> {
        self.peer_manager
            .announce_block(peer_id, block)
            .map_err(ManagedNetworkError::from)
    }

    pub fn announce_transaction(
        &self,
        peer_id: PeerId,
        transaction: &Transaction,
    ) -> Result<Option<WireNetworkMessage>, ManagedNetworkError> {
        self.peer_manager
            .announce_transaction(peer_id, transaction)
            .map_err(ManagedNetworkError::from)
    }

    fn collect_outbound(
        &mut self,
        actions: Vec<PeerAction>,
    ) -> Result<Vec<WireNetworkMessage>, ManagedNetworkError> {
        let mut outbound = Vec::new();
        for action in actions {
            if let PeerAction::Send(message) = action {
                outbound.push(message);
            }
        }
        Ok(outbound)
    }

    fn process_actions(
        &mut self,
        actions: Vec<PeerAction>,
        verify_flags: ScriptVerifyFlags,
        consensus_params: ConsensusParams,
    ) -> Result<Vec<WireNetworkMessage>, ManagedNetworkError> {
        let mut outbound = Vec::new();

        for action in actions {
            match action {
                PeerAction::Send(message) => outbound.push(message),
                PeerAction::ServeInventory(requests) => {
                    let (messages, missing) = self.serve_inventory(requests);
                    outbound.extend(messages);
                    if !missing.is_empty() {
                        outbound.push(WireNetworkMessage::NotFound(InventoryList::new(missing)));
                    }
                }
                PeerAction::ReceivedTransaction(transaction) => {
                    let txid = transaction_txid(&transaction)?;
                    if !self.transactions_by_txid.contains_key(&txid) {
                        self.mempool.submit_transaction(
                            &self.chainstate,
                            transaction.clone(),
                            verify_flags,
                            consensus_params,
                        )?;
                        self.store_transaction(transaction)?;
                    }
                }
                PeerAction::ReceivedBlock(block) => {
                    let block_hash = block_hash(&block.header);
                    if !self.blocks_by_hash.contains_key(&block_hash) {
                        let position = self.chainstate.connect_block(
                            &block,
                            self.next_chain_work(),
                            verify_flags,
                            consensus_params,
                        )?;
                        self.blocks_by_hash.insert(block_hash, block);
                        self.peer_manager.note_local_position(&position);
                    }
                }
                PeerAction::Disconnect(_) => {}
            }
        }

        Ok(outbound)
    }

    fn serve_inventory(
        &self,
        requests: Vec<open_bitcoin_core::primitives::InventoryVector>,
    ) -> (
        Vec<WireNetworkMessage>,
        Vec<open_bitcoin_core::primitives::InventoryVector>,
    ) {
        let mut messages = Vec::new();
        let mut missing = Vec::new();

        for request in requests {
            match request.inventory_type {
                InventoryType::Block | InventoryType::WitnessBlock => {
                    let block_hash = BlockHash::from(request.object_hash);
                    if let Some(block) = self.blocks_by_hash.get(&block_hash) {
                        messages.push(WireNetworkMessage::Block(block.clone()));
                    } else {
                        missing.push(request);
                    }
                }
                InventoryType::Transaction => {
                    let txid = Txid::from(request.object_hash);
                    if let Some(transaction) = self.transactions_by_txid.get(&txid) {
                        messages.push(WireNetworkMessage::Tx(transaction.clone()));
                    } else {
                        missing.push(request);
                    }
                }
                InventoryType::WitnessTransaction => {
                    let wtxid = Wtxid::from(request.object_hash);
                    if let Some(transaction) = self.transactions_by_wtxid.get(&wtxid) {
                        messages.push(WireNetworkMessage::Tx(transaction.clone()));
                    } else {
                        missing.push(request);
                    }
                }
                _ => missing.push(request),
            }
        }

        (messages, missing)
    }

    fn store_transaction(
        &mut self,
        transaction: Transaction,
    ) -> Result<(Txid, Wtxid), ManagedNetworkError> {
        let txid = transaction_txid(&transaction)?;
        let wtxid = transaction_wtxid(&transaction)?;
        self.transactions_by_txid.insert(txid, transaction.clone());
        self.transactions_by_wtxid
            .insert(wtxid, transaction.clone());
        self.peer_manager.note_local_transaction(&transaction)?;
        Ok((txid, wtxid))
    }

    fn next_chain_work(&self) -> u128 {
        self.chainstate
            .chainstate()
            .tip()
            .map_or(1, |tip| tip.chain_work.saturating_add(1))
    }
}

#[cfg(test)]
mod tests {
    use open_bitcoin_core::consensus::crypto::hash160;
    use open_bitcoin_core::{
        consensus::{
            ConsensusParams, ScriptVerifyFlags, block_merkle_root, check_block_header,
            transaction_txid,
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
        let mut to_source = sink.connect_outbound_peer(7, 1).expect("connect");
        let mut to_sink = deliver(&sink, &mut source, 7, to_source, 2);
        to_source = deliver(&source, &mut sink, 7, to_sink, 3);
        to_sink = deliver(&sink, &mut source, 7, to_source, 4);
        to_source = deliver(&source, &mut sink, 7, to_sink, 5);
        to_sink = deliver(&sink, &mut source, 7, to_source, 6);
        let final_outbound = deliver(&source, &mut sink, 7, to_sink, 7);
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
}
