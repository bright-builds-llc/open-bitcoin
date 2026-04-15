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
mod tests {
    use open_bitcoin_chainstate::ChainPosition;
    use open_bitcoin_consensus::check_block_header;
    use open_bitcoin_primitives::{
        Block, BlockHash, BlockHeader, Hash32, MerkleRoot, NetworkMagic,
    };

    use crate::{
        ConnectionRole, HeadersMessage, InventoryList, LocalPeerConfig, PeerAction, PeerManager,
        ServiceFlags, WireNetworkMessage,
    };
    use open_bitcoin_primitives::{InventoryType, InventoryVector};

    fn local_config() -> LocalPeerConfig {
        LocalPeerConfig {
            magic: NetworkMagic::MAINNET,
            services: ServiceFlags::NETWORK | ServiceFlags::WITNESS,
            address: super::super::message::zero_address(),
            nonce: 7,
            relay: true,
            user_agent: "/open-bitcoin:test/".to_string(),
        }
    }

    fn header(previous_block_hash: BlockHash, nonce: u32) -> BlockHeader {
        BlockHeader {
            version: 1,
            previous_block_hash,
            merkle_root: MerkleRoot::from_byte_array([nonce as u8; 32]),
            time: 1_231_006_500 + nonce,
            bits: 0x207f_ffff,
            nonce,
        }
    }

    fn mined_header(previous_block_hash: BlockHash, seed: u32) -> BlockHeader {
        let mut header = header(previous_block_hash, seed);
        let nonce = (0..=u32::MAX)
            .find(|nonce| {
                header.nonce = *nonce;
                check_block_header(&header).is_ok()
            })
            .expect("expected nonce at easy target");
        header.nonce = nonce;
        header
    }

    #[test]
    fn outbound_handshake_negotiates_verack_sendheaders_and_wtxidrelay() {
        let mut manager = PeerManager::new(local_config());
        let outbound = manager
            .add_outbound_peer(11, 10)
            .expect("peer should be added");
        assert!(matches!(
            outbound.as_slice(),
            [PeerAction::Send(WireNetworkMessage::Version(_))]
        ));

        let version_actions = manager
            .handle_message(
                11,
                WireNetworkMessage::Version(crate::VersionMessage {
                    start_height: 3,
                    ..crate::VersionMessage::default()
                }),
                11,
            )
            .expect("version should process");
        assert_eq!(
            version_actions,
            vec![
                PeerAction::Send(WireNetworkMessage::WtxidRelay),
                PeerAction::Send(WireNetworkMessage::Verack),
                PeerAction::Send(WireNetworkMessage::SendHeaders),
            ],
        );

        let verack_actions = manager
            .handle_message(11, WireNetworkMessage::Verack, 12)
            .expect("verack should process");
        assert!(matches!(
            verack_actions.as_slice(),
            [PeerAction::Send(WireNetworkMessage::GetHeaders { .. })]
        ));

        let ping_actions = manager
            .handle_message(11, WireNetworkMessage::Ping { nonce: 99 }, 13)
            .expect("ping should process");
        assert_eq!(
            ping_actions,
            vec![PeerAction::Send(WireNetworkMessage::Pong { nonce: 99 })],
        );
        assert_eq!(
            manager.peer_state(11).expect("state").role,
            ConnectionRole::Outbound,
        );
    }

    #[test]
    fn block_inventory_triggers_getheaders_then_getdata_for_missing_blocks() {
        let mut manager = PeerManager::new(local_config());
        let genesis_header = mined_header(BlockHash::from_byte_array([0_u8; 32]), 1);
        let genesis_hash = open_bitcoin_consensus::block_hash(&genesis_header);
        manager.seed_local_chain(&[ChainPosition::new(genesis_header.clone(), 0, 1, 0)]);
        manager.add_outbound_peer(2, 10).expect("peer");
        manager
            .handle_message(
                2,
                WireNetworkMessage::Version(crate::VersionMessage {
                    start_height: 0,
                    ..crate::VersionMessage::default()
                }),
                11,
            )
            .expect("version");
        manager
            .handle_message(2, WireNetworkMessage::Verack, 12)
            .expect("verack");

        let next_header = mined_header(genesis_hash, 2);
        let block_inventory = InventoryList::new(vec![InventoryVector {
            inventory_type: InventoryType::Block,
            object_hash: open_bitcoin_consensus::block_hash(&next_header).into(),
        }]);
        let inventory_actions = manager
            .handle_message(2, WireNetworkMessage::Inv(block_inventory), 13)
            .expect("inventory");
        assert!(inventory_actions.iter().any(|action| matches!(
            action,
            PeerAction::Send(WireNetworkMessage::GetHeaders { .. })
        )));

        let header_actions = manager
            .handle_message(
                2,
                WireNetworkMessage::Headers(crate::HeadersMessage {
                    headers: vec![next_header.clone()],
                }),
                14,
            )
            .expect("headers");
        assert!(
            header_actions
                .iter()
                .any(|action| matches!(action, PeerAction::Send(WireNetworkMessage::GetData(_))))
        );
        assert!(
            manager
                .peer_state(2)
                .expect("peer")
                .requested_blocks
                .contains(&open_bitcoin_consensus::block_hash(&next_header))
        );
    }

    #[test]
    fn announce_transaction_uses_wtxidrelay_when_peer_negotiates_it() {
        let mut manager = PeerManager::new(local_config());
        manager.add_inbound_peer(4).expect("peer");
        manager
            .handle_message(
                4,
                WireNetworkMessage::Version(crate::VersionMessage::default()),
                20,
            )
            .expect("version");
        manager
            .handle_message(4, WireNetworkMessage::WtxidRelay, 20)
            .expect("wtxidrelay");

        let transaction = open_bitcoin_primitives::Transaction::default();
        let announcement = manager
            .announce_transaction(4, &transaction)
            .expect("announce")
            .expect("message");

        assert!(matches!(
            announcement,
            WireNetworkMessage::Inv(InventoryList { inventory })
            if inventory[0].inventory_type == InventoryType::WitnessTransaction
        ));
    }

    #[test]
    fn helper_methods_and_unknown_peer_errors_are_covered() {
        let mut manager = PeerManager::new(local_config());
        assert!(manager.peer_state(99).is_none());
        assert_eq!(
            manager
                .handle_message(99, WireNetworkMessage::Version(Default::default()), 1)
                .expect_err("unknown peer")
                .to_string(),
            "unknown peer: 99",
        );
        assert_eq!(
            manager
                .request_ping(99, 1)
                .expect_err("unknown peer")
                .to_string(),
            "unknown peer: 99",
        );
        let block = Block {
            header: mined_header(BlockHash::from_byte_array([0_u8; 32]), 1),
            transactions: Vec::new(),
        };
        assert_eq!(
            manager
                .announce_block(99, &block)
                .expect_err("unknown peer")
                .to_string(),
            "unknown peer: 99",
        );
        assert_eq!(
            manager
                .announce_transaction(99, &open_bitcoin_primitives::Transaction::default())
                .expect_err("unknown peer")
                .to_string(),
            "unknown peer: 99",
        );
        assert_eq!(
            manager
                .handle_message(99, WireNetworkMessage::Verack, 1)
                .expect_err("unknown peer")
                .to_string(),
            "unknown peer: 99",
        );

        let genesis = mined_header(BlockHash::from_byte_array([0_u8; 32]), 2);
        let position = ChainPosition::new(genesis, 0, 1, 0);
        manager.seed_local_chain(std::slice::from_ref(&position));
        manager.note_local_position(&position);
        manager
            .note_local_transaction(&open_bitcoin_primitives::Transaction::default())
            .expect("local transaction");
        assert_eq!(manager.header_store().best_height(), 0);
    }

    #[test]
    fn ping_block_announcement_and_duplicate_add_paths_are_exercised() {
        let mut manager = PeerManager::new(local_config());
        manager.add_inbound_peer(5).expect("peer");
        assert_eq!(
            manager
                .add_inbound_peer(5)
                .expect_err("duplicate peer")
                .to_string(),
            "peer already exists: 5",
        );
        assert_eq!(
            manager
                .add_outbound_peer(5, 1)
                .expect_err("duplicate peer")
                .to_string(),
            "peer already exists: 5",
        );

        let ping = manager.request_ping(5, 123).expect("ping");
        assert_eq!(ping, WireNetworkMessage::Ping { nonce: 123 });
        manager
            .handle_message(5, WireNetworkMessage::Pong { nonce: 123 }, 1)
            .expect("pong");
        assert!(
            manager
                .peer_state(5)
                .expect("state")
                .last_ping_nonce
                .is_none()
        );

        let block = Block {
            header: mined_header(BlockHash::from_byte_array([0_u8; 32]), 4),
            transactions: Vec::new(),
        };
        let inv_message = manager
            .announce_block(5, &block)
            .expect("announce")
            .expect("inv");
        assert!(matches!(
            inv_message,
            WireNetworkMessage::Inv(InventoryList { inventory })
            if inventory[0].inventory_type == InventoryType::Block
        ));

        manager
            .handle_message(5, WireNetworkMessage::SendHeaders, 2)
            .expect("sendheaders");
        let headers_message = manager
            .announce_block(5, &block)
            .expect("announce")
            .expect("headers");
        assert!(matches!(
            headers_message,
            WireNetworkMessage::Headers(HeadersMessage { headers }) if headers.len() == 1
        ));

        let transaction = open_bitcoin_primitives::Transaction::default();
        let announcement = manager
            .announce_transaction(5, &transaction)
            .expect("announce")
            .expect("message");
        assert!(matches!(
            announcement,
            WireNetworkMessage::Inv(InventoryList { inventory })
            if inventory[0].inventory_type == InventoryType::Transaction
        ));
    }

    #[test]
    fn inventory_requests_and_notfound_paths_cover_tx_and_block_modes() {
        let mut manager = PeerManager::new(local_config());
        manager.add_inbound_peer(6).expect("peer");
        assert_eq!(
            manager
                .handle_message(99, WireNetworkMessage::Inv(InventoryList::default()), 1)
                .expect_err("unknown peer")
                .to_string(),
            "unknown peer: 99",
        );

        let txid_inv = InventoryList::new(vec![InventoryVector {
            inventory_type: InventoryType::Transaction,
            object_hash: Hash32::from_byte_array([2_u8; 32]),
        }]);
        let txid_actions = manager
            .handle_message(6, WireNetworkMessage::Inv(txid_inv), 1)
            .expect("txid inventory");
        assert!(matches!(
            txid_actions.as_slice(),
            [PeerAction::Send(WireNetworkMessage::GetData(_))]
        ));

        manager
            .handle_message(6, WireNetworkMessage::WtxidRelay, 1)
            .expect("wtxidrelay");
        assert_eq!(
            manager
                .handle_message(99, WireNetworkMessage::WtxidRelay, 1)
                .expect_err("unknown peer")
                .to_string(),
            "unknown peer: 99",
        );
        assert_eq!(
            manager
                .handle_message(99, WireNetworkMessage::SendHeaders, 1)
                .expect_err("unknown peer")
                .to_string(),
            "unknown peer: 99",
        );
        assert_eq!(
            manager
                .handle_message(99, WireNetworkMessage::Pong { nonce: 1 }, 1)
                .expect_err("unknown peer")
                .to_string(),
            "unknown peer: 99",
        );
        let wtxid_inv = InventoryList::new(vec![InventoryVector {
            inventory_type: InventoryType::WitnessTransaction,
            object_hash: Hash32::from_byte_array([3_u8; 32]),
        }]);
        let wtxid_actions = manager
            .handle_message(6, WireNetworkMessage::Inv(wtxid_inv), 2)
            .expect("wtxid inventory");
        assert!(matches!(
            wtxid_actions.as_slice(),
            [PeerAction::Send(WireNetworkMessage::GetData(_))]
        ));
        let ignored_inventory = manager
            .handle_message(
                6,
                WireNetworkMessage::Inv(InventoryList::new(vec![InventoryVector {
                    inventory_type: InventoryType::CompactBlock,
                    object_hash: Hash32::from_byte_array([4_u8; 32]),
                }])),
                2,
            )
            .expect("ignored inventory");
        assert!(ignored_inventory.is_empty());

        let genesis = mined_header(BlockHash::from_byte_array([0_u8; 32]), 5);
        manager.seed_local_chain(&[ChainPosition::new(genesis.clone(), 0, 1, 0)]);
        let next = mined_header(open_bitcoin_consensus::block_hash(&genesis), 6);
        manager
            .handle_message(
                6,
                WireNetworkMessage::Headers(crate::HeadersMessage {
                    headers: vec![next.clone()],
                }),
                3,
            )
            .expect("headers");

        let not_found = InventoryList::new(vec![
            InventoryVector {
                inventory_type: InventoryType::Transaction,
                object_hash: Hash32::from_byte_array([2_u8; 32]),
            },
            InventoryVector {
                inventory_type: InventoryType::WitnessTransaction,
                object_hash: Hash32::from_byte_array([3_u8; 32]),
            },
            InventoryVector {
                inventory_type: InventoryType::Block,
                object_hash: open_bitcoin_consensus::block_hash(&next).into(),
            },
            InventoryVector {
                inventory_type: InventoryType::CompactBlock,
                object_hash: Hash32::from_byte_array([4_u8; 32]),
            },
        ]);
        manager
            .handle_message(6, WireNetworkMessage::NotFound(not_found), 4)
            .expect("notfound");
        let peer = manager.peer_state(6).expect("peer");
        assert!(peer.requested_txids.is_empty());
        assert!(peer.requested_wtxids.is_empty());
        assert!(peer.requested_blocks.is_empty());
    }

    #[test]
    fn getheaders_headers_tx_and_block_paths_are_explicit() {
        let mut manager = PeerManager::new(local_config());
        manager.add_inbound_peer(7).expect("peer");

        let genesis = mined_header(BlockHash::from_byte_array([0_u8; 32]), 7);
        let genesis_position = ChainPosition::new(genesis.clone(), 0, 1, 0);
        manager.seed_local_chain(std::slice::from_ref(&genesis_position));

        let getheaders_actions = manager
            .handle_message(
                7,
                WireNetworkMessage::GetHeaders {
                    locator: open_bitcoin_primitives::BlockLocator::default(),
                    stop_hash: BlockHash::from_byte_array([0_u8; 32]),
                },
                1,
            )
            .expect("getheaders");
        assert!(matches!(
            getheaders_actions.as_slice(),
            [PeerAction::Send(WireNetworkMessage::Headers(HeadersMessage { headers }))]
            if headers.len() == 1
        ));
        assert_eq!(
            manager
                .handle_message(
                    99,
                    WireNetworkMessage::Headers(crate::HeadersMessage {
                        headers: vec![genesis.clone()],
                    }),
                    1,
                )
                .expect_err("unknown peer")
                .to_string(),
            "unknown peer: 99",
        );

        let missing_parent = mined_header(BlockHash::from_byte_array([8_u8; 32]), 8);
        assert_eq!(
            manager
                .handle_message(
                    7,
                    WireNetworkMessage::Headers(crate::HeadersMessage {
                        headers: vec![missing_parent],
                    }),
                    2,
                )
                .expect_err("missing ancestor")
                .to_string(),
            format!(
                "missing header ancestor: {:?}",
                BlockHash::from_byte_array([8_u8; 32]).to_byte_array()
            ),
        );
        let invalid_pow_header = header(genesis_position.block_hash, 99);
        assert_eq!(
            manager
                .handle_message(
                    7,
                    WireNetworkMessage::Headers(crate::HeadersMessage {
                        headers: vec![invalid_pow_header],
                    }),
                    2,
                )
                .expect_err("invalid pow")
                .to_string(),
            "high-hash length out of range: 0",
        );
        let empty_headers = manager
            .handle_message(
                7,
                WireNetworkMessage::Headers(crate::HeadersMessage { headers: vec![] }),
                3,
            )
            .expect("empty headers");
        assert!(empty_headers.is_empty());

        let served = manager
            .handle_message(
                7,
                WireNetworkMessage::GetData(InventoryList::new(vec![InventoryVector {
                    inventory_type: InventoryType::Block,
                    object_hash: genesis_position.block_hash.into(),
                }])),
                3,
            )
            .expect("getdata");
        assert!(matches!(served.as_slice(), [PeerAction::ServeInventory(_)]));
        assert_eq!(
            manager
                .handle_message(
                    99,
                    WireNetworkMessage::NotFound(InventoryList::default()),
                    3
                )
                .expect_err("unknown peer")
                .to_string(),
            "unknown peer: 99",
        );

        let transaction = open_bitcoin_primitives::Transaction::default();
        let txid = open_bitcoin_consensus::transaction_txid(&transaction).expect("txid");
        let wtxid = open_bitcoin_consensus::transaction_wtxid(&transaction).expect("wtxid");
        let tx_actions = manager
            .handle_message(7, WireNetworkMessage::Tx(transaction), 4)
            .expect("tx");
        assert!(matches!(
            tx_actions.as_slice(),
            [PeerAction::ReceivedTransaction(_)]
        ));
        assert_eq!(
            manager
                .handle_message(
                    99,
                    WireNetworkMessage::Tx(open_bitcoin_primitives::Transaction::default()),
                    4,
                )
                .expect_err("unknown peer")
                .to_string(),
            "unknown peer: 99",
        );
        let block = Block {
            header: genesis,
            transactions: Vec::new(),
        };
        let block_hash = open_bitcoin_consensus::block_hash(&block.header);
        let block_actions = manager
            .handle_message(7, WireNetworkMessage::Block(block), 5)
            .expect("block");
        assert!(matches!(
            block_actions.as_slice(),
            [PeerAction::ReceivedBlock(_)]
        ));
        assert_eq!(
            manager
                .handle_message(
                    99,
                    WireNetworkMessage::Block(Block {
                        header: mined_header(BlockHash::from_byte_array([0_u8; 32]), 10),
                        transactions: Vec::new(),
                    }),
                    5,
                )
                .expect_err("unknown peer")
                .to_string(),
            "unknown peer: 99",
        );
        let peer = manager.peer_state(7).expect("peer");
        assert!(!peer.requested_txids.contains(&txid));
        assert!(!peer.requested_wtxids.contains(&wtxid));
        assert!(!peer.requested_blocks.contains(&block_hash));
    }
}
