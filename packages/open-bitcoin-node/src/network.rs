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
mod tests;
