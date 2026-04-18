use std::collections::{BTreeMap, BTreeSet};

use open_bitcoin_chainstate::ChainPosition;
use open_bitcoin_consensus::{block_hash, check_block_header, transaction_txid, transaction_wtxid};
use open_bitcoin_primitives::{
    Block, BlockHash, InventoryType, InventoryVector, Transaction, Txid, Wtxid,
};

use crate::error::{DisconnectReason, NetworkError, PeerId};
use crate::header_store::HeaderStore;
use crate::message::{HeadersMessage, InventoryList, LocalPeerConfig, WireNetworkMessage};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionRole {
    Inbound,
    Outbound,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PeerAction {
    Send(WireNetworkMessage),
    ServeInventory(Vec<InventoryVector>),
    ReceivedTransaction(Transaction),
    ReceivedBlock(Block),
    Disconnect(DisconnectReason),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PeerState {
    pub role: ConnectionRole,
    pub remote_start_height: i32,
    pub remote_wtxidrelay: bool,
    pub remote_prefers_headers: bool,
    pub remote_version_received: bool,
    pub remote_verack_received: bool,
    pub local_version_sent: bool,
    pub local_verack_sent: bool,
    pub sync_started: bool,
    pub getheaders_in_flight: bool,
    pub requested_blocks: BTreeSet<BlockHash>,
    pub requested_txids: BTreeSet<Txid>,
    pub requested_wtxids: BTreeSet<Wtxid>,
    pub last_ping_nonce: Option<u64>,
}

impl PeerState {
    fn new(role: ConnectionRole) -> Self {
        Self {
            role,
            remote_start_height: -1,
            remote_wtxidrelay: false,
            remote_prefers_headers: false,
            remote_version_received: false,
            remote_verack_received: false,
            local_version_sent: false,
            local_verack_sent: false,
            sync_started: false,
            getheaders_in_flight: false,
            requested_blocks: BTreeSet::new(),
            requested_txids: BTreeSet::new(),
            requested_wtxids: BTreeSet::new(),
            last_ping_nonce: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PeerManager {
    local_config: LocalPeerConfig,
    headers: HeaderStore,
    peers: BTreeMap<PeerId, PeerState>,
    known_blocks: BTreeSet<BlockHash>,
    known_txids: BTreeSet<Txid>,
    known_wtxids: BTreeSet<Wtxid>,
}

impl PeerManager {
    pub fn new(local_config: LocalPeerConfig) -> Self {
        Self {
            local_config,
            headers: HeaderStore::default(),
            peers: BTreeMap::new(),
            known_blocks: BTreeSet::new(),
            known_txids: BTreeSet::new(),
            known_wtxids: BTreeSet::new(),
        }
    }

    pub fn seed_local_chain(&mut self, active_chain: &[ChainPosition]) {
        self.headers.seed_from_chain(active_chain);
        self.known_blocks.clear();
        for position in active_chain {
            self.known_blocks.insert(position.block_hash);
        }
    }

    pub fn note_local_position(&mut self, position: &ChainPosition) {
        self.headers.record_position(position);
        self.known_blocks.insert(position.block_hash);
    }

    pub fn note_local_transaction(
        &mut self,
        transaction: &Transaction,
    ) -> Result<(), NetworkError> {
        self.known_txids.insert(transaction_txid(transaction)?);
        self.known_wtxids.insert(transaction_wtxid(transaction)?);
        Ok(())
    }

    pub fn header_store(&self) -> &HeaderStore {
        &self.headers
    }

    pub fn peer_state(&self, peer_id: PeerId) -> Option<&PeerState> {
        self.peers.get(&peer_id)
    }

    pub fn add_inbound_peer(&mut self, peer_id: PeerId) -> Result<(), NetworkError> {
        if self.peers.contains_key(&peer_id) {
            return Err(NetworkError::PeerAlreadyExists(peer_id));
        }
        self.peers
            .insert(peer_id, PeerState::new(ConnectionRole::Inbound));
        Ok(())
    }

    pub fn add_outbound_peer(
        &mut self,
        peer_id: PeerId,
        timestamp: i64,
    ) -> Result<Vec<PeerAction>, NetworkError> {
        if self.peers.contains_key(&peer_id) {
            return Err(NetworkError::PeerAlreadyExists(peer_id));
        }
        let mut state = PeerState::new(ConnectionRole::Outbound);
        state.local_version_sent = true;
        self.peers.insert(peer_id, state);
        Ok(vec![PeerAction::Send(WireNetworkMessage::Version(
            self.local_config
                .version_message(timestamp, self.headers.best_height()),
        ))])
    }

    pub fn request_ping(
        &mut self,
        peer_id: PeerId,
        nonce: u64,
    ) -> Result<WireNetworkMessage, NetworkError> {
        let Some(peer) = self.peers.get_mut(&peer_id) else {
            return Err(NetworkError::UnknownPeer(peer_id));
        };
        peer.last_ping_nonce = Some(nonce);
        Ok(WireNetworkMessage::Ping { nonce })
    }

    pub fn announce_block(
        &self,
        peer_id: PeerId,
        block: &Block,
    ) -> Result<Option<WireNetworkMessage>, NetworkError> {
        let Some(peer) = self.peers.get(&peer_id) else {
            return Err(NetworkError::UnknownPeer(peer_id));
        };
        let block_hash = block_hash(&block.header);
        if peer.remote_prefers_headers {
            return Ok(Some(WireNetworkMessage::Headers(HeadersMessage {
                headers: vec![block.header.clone()],
            })));
        }
        Ok(Some(WireNetworkMessage::Inv(InventoryList::new(vec![
            InventoryVector {
                inventory_type: InventoryType::Block,
                object_hash: block_hash.into(),
            },
        ]))))
    }

    pub fn announce_transaction(
        &self,
        peer_id: PeerId,
        transaction: &Transaction,
    ) -> Result<Option<WireNetworkMessage>, NetworkError> {
        let Some(peer) = self.peers.get(&peer_id) else {
            return Err(NetworkError::UnknownPeer(peer_id));
        };
        let txid = transaction_txid(transaction)?;
        let wtxid = transaction_wtxid(transaction)?;
        let inventory = if peer.remote_wtxidrelay {
            InventoryVector {
                inventory_type: InventoryType::WitnessTransaction,
                object_hash: wtxid.into(),
            }
        } else {
            InventoryVector {
                inventory_type: InventoryType::Transaction,
                object_hash: txid.into(),
            }
        };
        Ok(Some(WireNetworkMessage::Inv(InventoryList::new(vec![
            inventory,
        ]))))
    }

    pub fn handle_message(
        &mut self,
        peer_id: PeerId,
        message: WireNetworkMessage,
        timestamp: i64,
    ) -> Result<Vec<PeerAction>, NetworkError> {
        match message {
            WireNetworkMessage::Version(version) => {
                self.handle_version(peer_id, version, timestamp)
            }
            WireNetworkMessage::Verack => self.handle_verack(peer_id),
            WireNetworkMessage::WtxidRelay => {
                let Some(peer) = self.peers.get_mut(&peer_id) else {
                    return Err(NetworkError::UnknownPeer(peer_id));
                };
                peer.remote_wtxidrelay = true;
                Ok(Vec::new())
            }
            WireNetworkMessage::SendHeaders => {
                let Some(peer) = self.peers.get_mut(&peer_id) else {
                    return Err(NetworkError::UnknownPeer(peer_id));
                };
                peer.remote_prefers_headers = true;
                Ok(Vec::new())
            }
            WireNetworkMessage::Ping { nonce } => {
                Ok(vec![PeerAction::Send(WireNetworkMessage::Pong { nonce })])
            }
            WireNetworkMessage::Pong { nonce } => {
                let Some(peer) = self.peers.get_mut(&peer_id) else {
                    return Err(NetworkError::UnknownPeer(peer_id));
                };
                if peer.last_ping_nonce == Some(nonce) {
                    peer.last_ping_nonce = None;
                }
                Ok(Vec::new())
            }
            WireNetworkMessage::Inv(inventory) => self.handle_inventory(peer_id, inventory),
            WireNetworkMessage::GetHeaders { locator, stop_hash } => {
                let headers = self.headers.headers_after_locator(
                    &locator,
                    stop_hash,
                    crate::MAX_HEADERS_RESULTS,
                );
                Ok(vec![PeerAction::Send(WireNetworkMessage::Headers(
                    HeadersMessage { headers },
                ))])
            }
            WireNetworkMessage::Headers(message) => self.handle_headers(peer_id, message),
            WireNetworkMessage::GetData(inventory) => {
                Ok(vec![PeerAction::ServeInventory(inventory.inventory)])
            }
            WireNetworkMessage::NotFound(inventory) => {
                let Some(peer) = self.peers.get_mut(&peer_id) else {
                    return Err(NetworkError::UnknownPeer(peer_id));
                };
                for item in inventory.inventory {
                    match item.inventory_type {
                        InventoryType::Block | InventoryType::WitnessBlock => {
                            peer.requested_blocks
                                .remove(&BlockHash::from(item.object_hash));
                        }
                        InventoryType::Transaction => {
                            peer.requested_txids.remove(&Txid::from(item.object_hash));
                        }
                        InventoryType::WitnessTransaction => {
                            peer.requested_wtxids.remove(&Wtxid::from(item.object_hash));
                        }
                        _ => {}
                    }
                }
                Ok(Vec::new())
            }
            WireNetworkMessage::Tx(transaction) => self.handle_transaction(peer_id, transaction),
            WireNetworkMessage::Block(block) => self.handle_block(peer_id, block),
        }
    }

    fn handle_version(
        &mut self,
        peer_id: PeerId,
        version: crate::VersionMessage,
        timestamp: i64,
    ) -> Result<Vec<PeerAction>, NetworkError> {
        let best_height = self.headers.best_height();
        let Some(peer) = self.peers.get_mut(&peer_id) else {
            return Err(NetworkError::UnknownPeer(peer_id));
        };
        if peer.remote_version_received {
            return Ok(vec![PeerAction::Disconnect(
                DisconnectReason::DuplicateVersion,
            )]);
        }

        peer.remote_version_received = true;
        peer.remote_start_height = version.start_height;

        let mut actions = Vec::new();
        if !peer.local_version_sent {
            peer.local_version_sent = true;
            actions.push(PeerAction::Send(WireNetworkMessage::Version(
                self.local_config.version_message(timestamp, best_height),
            )));
        }
        if !peer.local_verack_sent {
            peer.local_verack_sent = true;
            actions.push(PeerAction::Send(WireNetworkMessage::WtxidRelay));
            actions.push(PeerAction::Send(WireNetworkMessage::Verack));
            actions.push(PeerAction::Send(WireNetworkMessage::SendHeaders));
        }
        Ok(actions)
    }

    fn handle_verack(&mut self, peer_id: PeerId) -> Result<Vec<PeerAction>, NetworkError> {
        let locator = self.headers.locator();
        let best_height = self.headers.best_height();
        let Some(peer) = self.peers.get_mut(&peer_id) else {
            return Err(NetworkError::UnknownPeer(peer_id));
        };
        peer.remote_verack_received = true;

        if peer.remote_start_height > best_height && !peer.getheaders_in_flight {
            peer.getheaders_in_flight = true;
            peer.sync_started = true;
            return Ok(vec![PeerAction::Send(WireNetworkMessage::GetHeaders {
                locator,
                stop_hash: BlockHash::from_byte_array([0_u8; 32]),
            })]);
        }
        Ok(Vec::new())
    }

    fn handle_inventory(
        &mut self,
        peer_id: PeerId,
        inventory: InventoryList,
    ) -> Result<Vec<PeerAction>, NetworkError> {
        let locator = self.headers.locator();
        let mut tx_requests = Vec::new();
        let mut request_headers = false;

        let Some(peer) = self.peers.get_mut(&peer_id) else {
            return Err(NetworkError::UnknownPeer(peer_id));
        };

        for item in inventory.inventory {
            match item.inventory_type {
                InventoryType::Block | InventoryType::WitnessBlock => {
                    let block_hash = BlockHash::from(item.object_hash);
                    if !self.known_blocks.contains(&block_hash) {
                        request_headers = true;
                    }
                }
                InventoryType::Transaction => {
                    let txid = Txid::from(item.object_hash);
                    if !peer.remote_wtxidrelay
                        && !self.known_txids.contains(&txid)
                        && !peer.requested_txids.contains(&txid)
                    {
                        peer.requested_txids.insert(txid);
                        tx_requests.push(item);
                    }
                }
                InventoryType::WitnessTransaction => {
                    let wtxid = Wtxid::from(item.object_hash);
                    if peer.remote_wtxidrelay
                        && !self.known_wtxids.contains(&wtxid)
                        && !peer.requested_wtxids.contains(&wtxid)
                    {
                        peer.requested_wtxids.insert(wtxid);
                        tx_requests.push(item);
                    }
                }
                _ => {}
            }
        }

        let mut actions = Vec::new();
        if request_headers && !peer.getheaders_in_flight {
            peer.getheaders_in_flight = true;
            peer.sync_started = true;
            actions.push(PeerAction::Send(WireNetworkMessage::GetHeaders {
                locator,
                stop_hash: BlockHash::from_byte_array([0_u8; 32]),
            }));
        }
        if !tx_requests.is_empty() {
            actions.push(PeerAction::Send(WireNetworkMessage::GetData(
                InventoryList::new(tx_requests),
            )));
        }
        Ok(actions)
    }

    fn handle_headers(
        &mut self,
        peer_id: PeerId,
        headers_message: HeadersMessage,
    ) -> Result<Vec<PeerAction>, NetworkError> {
        let mut requested_inventory = Vec::new();
        for header in headers_message.headers {
            check_block_header(&header).map_err(|error| {
                NetworkError::Codec(open_bitcoin_codec::CodecError::LengthOutOfRange {
                    field: error.reject_reason,
                    value: 0,
                })
            })?;
            let inserted = self.headers.insert_header(header)?;
            if !self.known_blocks.contains(&inserted.block_hash) {
                requested_inventory.push(InventoryVector {
                    inventory_type: InventoryType::Block,
                    object_hash: inserted.block_hash.into(),
                });
            }
        }

        let Some(peer) = self.peers.get_mut(&peer_id) else {
            return Err(NetworkError::UnknownPeer(peer_id));
        };
        peer.getheaders_in_flight = false;
        for item in &requested_inventory {
            peer.requested_blocks
                .insert(BlockHash::from(item.object_hash));
        }
        if requested_inventory.is_empty() {
            return Ok(Vec::new());
        }
        Ok(vec![PeerAction::Send(WireNetworkMessage::GetData(
            InventoryList::new(requested_inventory),
        ))])
    }

    fn handle_transaction(
        &mut self,
        peer_id: PeerId,
        transaction: Transaction,
    ) -> Result<Vec<PeerAction>, NetworkError> {
        let txid = transaction_txid(&transaction)?;
        let wtxid = transaction_wtxid(&transaction)?;
        self.known_txids.insert(txid);
        self.known_wtxids.insert(wtxid);

        let Some(peer) = self.peers.get_mut(&peer_id) else {
            return Err(NetworkError::UnknownPeer(peer_id));
        };
        peer.requested_txids.remove(&txid);
        peer.requested_wtxids.remove(&wtxid);

        Ok(vec![PeerAction::ReceivedTransaction(transaction)])
    }

    fn handle_block(
        &mut self,
        peer_id: PeerId,
        block: Block,
    ) -> Result<Vec<PeerAction>, NetworkError> {
        let hash = block_hash(&block.header);
        self.known_blocks.insert(hash);

        let Some(peer) = self.peers.get_mut(&peer_id) else {
            return Err(NetworkError::UnknownPeer(peer_id));
        };
        peer.requested_blocks.remove(&hash);

        Ok(vec![PeerAction::ReceivedBlock(block)])
    }
}

#[cfg(test)]
mod tests;
