// Parity breadcrumbs:
// - packages/bitcoin-knots/src/blockencodings.h
// - packages/bitcoin-knots/src/blockencodings.cpp
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/net_processing.h
// - packages/bitcoin-knots/test/functional/p2p_compactblocks.py

use open_bitcoin_codec::expand_block_transaction_indexes;
use open_bitcoin_consensus::check_block_header;
use open_bitcoin_primitives::{BlockHash, BlockHeader, InventoryType, InventoryVector};

use crate::address::maybe_version_sender_address;
use crate::error::{DisconnectReason, NetworkError, PeerId};
use crate::header_store::{HeaderStore, InsertedHeader};
use crate::inbound::InboundHandshakeState;
use crate::message::{HeadersMessage, InventoryList, WireNetworkMessage};

use super::inbound_state::reject_self_connection;
use super::inventory_state::{request_pressure_input, resource_limit_disconnect_actions};
use super::{
    CompactBlockTransactionsRequest, ConnectionRole, HeaderSyncPolicy, PeerAction, PeerManager,
};

impl PeerManager {
    pub fn handle_message(
        &mut self,
        peer_id: PeerId,
        message: WireNetworkMessage,
        timestamp: i64,
    ) -> Result<Vec<PeerAction>, NetworkError> {
        match message {
            WireNetworkMessage::GetCFilters(request) => {
                if !Self::peer_mut(&mut self.peers, peer_id)?.handshake_established() {
                    return Ok(Vec::new());
                }
                Ok(vec![PeerAction::ServeBasicFilter(
                    crate::policy::FilterRequest::Filters {
                        filter_type: request.filter_type,
                        start_height: request.start_height,
                        stop_hash: request.stop_hash,
                    },
                )])
            }
            WireNetworkMessage::GetCFHeaders(request) => {
                if !Self::peer_mut(&mut self.peers, peer_id)?.handshake_established() {
                    return Ok(Vec::new());
                }
                Ok(vec![PeerAction::ServeBasicFilter(
                    crate::policy::FilterRequest::Headers {
                        filter_type: request.filter_type,
                        start_height: request.start_height,
                        stop_hash: request.stop_hash,
                    },
                )])
            }
            WireNetworkMessage::GetCFCheckpt(request) => {
                if !Self::peer_mut(&mut self.peers, peer_id)?.handshake_established() {
                    return Ok(Vec::new());
                }
                Ok(vec![PeerAction::ServeBasicFilter(
                    crate::policy::FilterRequest::Checkpoints {
                        filter_type: request.filter_type,
                        stop_hash: request.stop_hash,
                    },
                )])
            }
            WireNetworkMessage::MalformedCFRequest(_)
            | WireNetworkMessage::CFilter(_)
            | WireNetworkMessage::CFHeaders(_)
            | WireNetworkMessage::CFCheckpt(_) => {
                Self::peer_mut(&mut self.peers, peer_id)?;
                Ok(Vec::new())
            }
            WireNetworkMessage::Version(version) => {
                self.handle_version(peer_id, version, timestamp)
            }
            WireNetworkMessage::Verack => self.handle_verack(peer_id),
            WireNetworkMessage::WtxidRelay => {
                let peer = Self::peer_mut(&mut self.peers, peer_id)?;
                peer.remote_wtxidrelay = true;
                Ok(Vec::new())
            }
            WireNetworkMessage::SendHeaders => {
                let peer = Self::peer_mut(&mut self.peers, peer_id)?;
                peer.remote_prefers_headers = true;
                Ok(Vec::new())
            }
            WireNetworkMessage::SendCompact(message) => {
                let peer = Self::peer_mut(&mut self.peers, peer_id)?;
                let _outcome = peer.compact_relay.apply_send_compact(message);
                Ok(Vec::new())
            }
            WireNetworkMessage::CompactBlock(_) => {
                Err(NetworkError::CompactBlockReceiveFactsRequired)
            }
            WireNetworkMessage::GetBlockTxn(request) => {
                self.handle_get_block_transactions(peer_id, request)
            }
            WireNetworkMessage::BlockTxn(response) => {
                self.handle_block_transactions_message(peer_id, response)
            }
            WireNetworkMessage::Ping { nonce } => {
                Ok(vec![PeerAction::Send(WireNetworkMessage::Pong { nonce })])
            }
            WireNetworkMessage::Pong { nonce } => {
                let peer = Self::peer_mut(&mut self.peers, peer_id)?;
                if peer.last_ping_nonce == Some(nonce) {
                    peer.last_ping_nonce = None;
                }
                Ok(Vec::new())
            }
            WireNetworkMessage::Inv(inventory) => {
                self.handle_inventory(peer_id, inventory, timestamp)
            }
            WireNetworkMessage::GetHeaders { locator, stop_hash } => {
                self.handle_getheaders(peer_id, locator, stop_hash)
            }
            WireNetworkMessage::Headers(message) => self.handle_headers(peer_id, message),
            WireNetworkMessage::GetAddr => self.handle_getaddr(peer_id, timestamp),
            WireNetworkMessage::Addr(addresses) => self.handle_addr(peer_id, addresses, timestamp),
            WireNetworkMessage::GetData(inventory) => self.handle_getdata(peer_id, inventory),
            WireNetworkMessage::NotFound(inventory) => {
                self.handle_notfound(peer_id, inventory, timestamp)
            }
            WireNetworkMessage::Tx(transaction) => self.handle_transaction(peer_id, transaction),
            WireNetworkMessage::Block(block) => self.handle_block(peer_id, block),
        }
    }

    fn handle_get_block_transactions(
        &self,
        peer_id: PeerId,
        request: open_bitcoin_codec::BlockTransactionsRequest,
    ) -> Result<Vec<PeerAction>, NetworkError> {
        let peer = self
            .peers
            .get(&peer_id)
            .ok_or(NetworkError::UnknownPeer(peer_id))?;
        let pressure = request_pressure_input(
            peer,
            0,
            request.index_deltas.len(),
            0,
            peer.requested_blocks.len(),
            self.tx_download.peer_snapshot(peer_id).in_flight_count,
            0,
        );
        if let Some(actions) = resource_limit_disconnect_actions(pressure) {
            return Ok(actions);
        }
        if !peer.compact_announcements.contains(&request.block_hash) {
            return Ok(Vec::new());
        }

        let Ok(indexes) = expand_block_transaction_indexes(&request) else {
            return Ok(vec![PeerAction::Disconnect(
                DisconnectReason::CompactBlockMisbehavior,
            )]);
        };

        Ok(vec![PeerAction::ServeCompactBlockTransactions(
            CompactBlockTransactionsRequest {
                block_hash: request.block_hash,
                indexes,
            },
        )])
    }

    fn handle_version(
        &mut self,
        peer_id: PeerId,
        version: crate::VersionMessage,
        timestamp: i64,
    ) -> Result<Vec<PeerAction>, NetworkError> {
        let best_height = self.headers.best_height();
        let local_nonce = self.local_config.nonce;
        let maybe_sender = maybe_version_sender_address(&self.local_address_decisions);
        let peer = Self::peer_mut(&mut self.peers, peer_id)?;
        let old = peer.basic_scope;
        if peer.remote_version_received {
            peer.basic_scope = super::BasicPeerScope::default();
            self.transition_basic_peer_scope(old, super::BasicPeerScope::default());
            return Ok(vec![PeerAction::Disconnect(
                DisconnectReason::DuplicateVersion,
            )]);
        }

        if peer.role == ConnectionRole::Inbound && version.nonce == local_nonce {
            let action = reject_self_connection(peer, version.nonce);
            peer.basic_scope = super::BasicPeerScope::default();
            self.transition_basic_peer_scope(old, super::BasicPeerScope::default());
            return Ok(vec![action]);
        }

        peer.remote_version_received = true;
        peer.maybe_remote_protocol_version = Some(version.version);
        peer.remote_start_height = version.start_height;
        peer.remote_services_bits = version.services.bits();
        peer.remote_user_agent = version.user_agent.clone();
        if let Some(record) = peer.maybe_inbound_record.as_mut() {
            record.maybe_remote_nonce = Some(version.nonce);
            record.handshake_state = InboundHandshakeState::Handshaking;
        }

        let mut actions = Vec::new();
        if !peer.local_version_sent {
            peer.local_version_sent = true;
            let mut local_version = self.local_config.version_message_with_sender_policy(
                timestamp,
                best_height,
                maybe_sender,
            );
            local_version.services = peer.snapshot_own_services(self.local_config.services);
            actions.push(PeerAction::Send(WireNetworkMessage::Version(local_version)));
        }
        if !peer.local_verack_sent {
            peer.local_verack_sent = true;
            actions.push(PeerAction::Send(WireNetworkMessage::WtxidRelay));
            actions.push(PeerAction::Send(WireNetworkMessage::Verack));
            actions.push(PeerAction::Send(WireNetworkMessage::SendHeaders));
        }
        let new = peer.basic_scope;
        self.transition_basic_peer_scope(old, new);
        Ok(actions)
    }

    fn handle_verack(&mut self, peer_id: PeerId) -> Result<Vec<PeerAction>, NetworkError> {
        let locator = self.headers.locator();
        let best_height = self.headers.best_height();
        let mut actions = {
            let peer = Self::peer_mut(&mut self.peers, peer_id)?;
            peer.remote_verack_received = true;
            if let Some(record) = peer.maybe_inbound_record.as_mut()
                && peer.remote_version_received
                && peer.local_verack_sent
            {
                record.handshake_state = InboundHandshakeState::Established;
            }

            let mut actions = Vec::new();
            if peer.remote_start_height > best_height && !peer.getheaders_in_flight {
                peer.getheaders_in_flight = true;
                peer.sync_started = true;
                actions.push(PeerAction::Send(WireNetworkMessage::GetHeaders {
                    locator,
                    stop_hash: BlockHash::from_byte_array([0_u8; 32]),
                }));
            }
            actions
        };

        if let Some(message) = self.maybe_schedule_local_compact_offer(peer_id)? {
            actions.push(PeerAction::Send(WireNetworkMessage::SendCompact(message)));
        }
        Ok(actions)
    }

    fn handle_headers(
        &mut self,
        peer_id: PeerId,
        headers_message: HeadersMessage,
    ) -> Result<Vec<PeerAction>, NetworkError> {
        self.handle_headers_with_policy(
            peer_id,
            headers_message,
            HeaderSyncPolicy::HeadersAndBlocks,
            |headers: &mut HeaderStore, header: &BlockHeader| {
                check_block_header(header).map_err(|error| NetworkError::InvalidHeader {
                    reject_reason: error.reject_reason.to_string(),
                    maybe_debug_message: error.debug_message.clone(),
                })?;
                headers.insert_header(header.clone())
            },
        )
    }

    pub fn handle_headers_with_policy<F>(
        &mut self,
        peer_id: PeerId,
        headers_message: HeadersMessage,
        policy: HeaderSyncPolicy,
        mut validate_and_insert: F,
    ) -> Result<Vec<PeerAction>, NetworkError>
    where
        F: FnMut(&mut HeaderStore, &BlockHeader) -> Result<InsertedHeader, NetworkError>,
    {
        let maybe_previous_best_work = self.headers.best_tip().map(|tip| tip.chain_work);
        let header_count = headers_message.headers.len();
        let mut requested_inventory = Vec::new();
        for header in headers_message.headers {
            let inserted = validate_and_insert(&mut self.headers, &header)?;
            if !self.known_blocks.contains(&inserted.block_hash) {
                requested_inventory.push(InventoryVector {
                    inventory_type: InventoryType::Block,
                    object_hash: inserted.block_hash.into(),
                });
            }
        }
        let best_height = self.headers.best_height();
        let locator = self.headers.locator();
        let max_blocks_in_flight_per_peer = self
            .max_blocks_in_flight_per_peer
            .min(crate::PHASE94_MAX_INBOUND_BLOCK_REQUESTS_PER_PEER);
        let peer = Self::peer_mut(&mut self.peers, peer_id)?;
        peer.getheaders_in_flight = false;

        let header_progressed = self
            .headers
            .best_tip()
            .is_some_and(|tip| maybe_previous_best_work.is_none_or(|work| tip.chain_work > work));
        let should_request_more_headers = header_count > 0
            && header_progressed
            && (header_count == crate::MAX_HEADERS_RESULTS
                || peer.remote_start_height > best_height);

        let mut actions = Vec::new();
        if should_request_more_headers {
            peer.getheaders_in_flight = true;
            peer.sync_started = true;
            actions.push(PeerAction::Send(WireNetworkMessage::GetHeaders {
                locator,
                stop_hash: BlockHash::from_byte_array([0_u8; 32]),
            }));
        }

        if policy == HeaderSyncPolicy::HeadersAndBlocks {
            let available_slots =
                max_blocks_in_flight_per_peer.saturating_sub(peer.requested_blocks.len());
            requested_inventory.truncate(available_slots);
            for item in &requested_inventory {
                peer.requested_blocks
                    .insert(BlockHash::from(item.object_hash));
            }
            if !requested_inventory.is_empty() {
                actions.push(PeerAction::Send(WireNetworkMessage::GetData(
                    InventoryList::new(requested_inventory),
                )));
            }
        }

        Ok(actions)
    }
}

#[cfg(test)]
mod filter_contract_tests {
    use super::*;
    use crate::{
        CFCheckpt, CFHeaders, CFRequestKind, CFilter, GetCFCheckpt, GetCFHeaders, GetCFilters,
        LocalPeerConfig, policy::FilterRequest,
    };

    fn requests(valid_fields: bool) -> [(WireNetworkMessage, FilterRequest); 3] {
        let filter_types = if valid_fields { [0; 3] } else { [255, 2, 1] };
        let starts = if valid_fields { [0; 2] } else { [u32::MAX, 77] };
        let stop_hash = BlockHash::from_byte_array([73; 32]);
        [
            (
                WireNetworkMessage::GetCFilters(GetCFilters {
                    filter_type: filter_types[0],
                    start_height: starts[0],
                    stop_hash,
                }),
                FilterRequest::Filters {
                    filter_type: filter_types[0],
                    start_height: starts[0],
                    stop_hash,
                },
            ),
            (
                WireNetworkMessage::GetCFHeaders(GetCFHeaders {
                    filter_type: filter_types[1],
                    start_height: starts[1],
                    stop_hash,
                }),
                FilterRequest::Headers {
                    filter_type: filter_types[1],
                    start_height: starts[1],
                    stop_hash,
                },
            ),
            (
                WireNetworkMessage::GetCFCheckpt(GetCFCheckpt {
                    filter_type: filter_types[2],
                    stop_hash,
                }),
                FilterRequest::Checkpoints {
                    filter_type: filter_types[2],
                    stop_hash,
                },
            ),
        ]
    }

    #[test]
    fn phase160_dispatch_contract_preserves_all_raw_request_identities() {
        // Arrange
        let mut manager = PeerManager::new(LocalPeerConfig::default());
        manager.add_outbound_peer(1, 0).expect("peer");
        manager
            .handle_message(1, WireNetworkMessage::Version(Default::default()), 0)
            .expect("version");
        manager
            .handle_message(1, WireNetworkMessage::Verack, 0)
            .expect("verack");
        for (request, expected) in requests(false) {
            // Act
            let actions = manager
                .handle_message(1, request, 1)
                .expect("parsed request");
            // Assert
            assert_eq!(actions, vec![PeerAction::ServeBasicFilter(expected)]);
        }
    }

    #[test]
    fn phase160_dispatch_contract_malformed_and_unsolicited_responses_are_inert() {
        // Arrange
        let mut manager = PeerManager::new(LocalPeerConfig::default());
        manager.add_outbound_peer(1, 0).expect("peer");
        let messages = [
            WireNetworkMessage::MalformedCFRequest(CFRequestKind::Filters),
            WireNetworkMessage::MalformedCFRequest(CFRequestKind::Headers),
            WireNetworkMessage::MalformedCFRequest(CFRequestKind::Checkpoints),
            WireNetworkMessage::CFilter(CFilter {
                filter_type: 0,
                block_hash: BlockHash::default(),
                filter: vec![0],
            }),
            WireNetworkMessage::CFHeaders(CFHeaders {
                filter_type: 0,
                stop_hash: BlockHash::default(),
                previous_filter_header: Default::default(),
                filter_hashes: Vec::new(),
            }),
            WireNetworkMessage::CFCheckpt(CFCheckpt {
                filter_type: 0,
                stop_hash: BlockHash::default(),
                filter_headers: Vec::new(),
            }),
        ];
        for message in messages {
            // Act
            let actions = manager
                .handle_message(1, message, 1)
                .expect("inert receive");
            // Assert
            assert!(actions.is_empty());
        }
    }

    #[test]
    fn phase160_dispatch_contract_unknown_session_does_not_authorize_intent() {
        // Arrange
        let mut manager = PeerManager::new(LocalPeerConfig::default());
        for (request, _) in requests(false) {
            // Act
            let result = manager.handle_message(99, request, 1);
            // Assert
            assert_eq!(result, Err(NetworkError::UnknownPeer(99)));
        }
    }

    #[test]
    fn phase160_dispatch_contract_requests_before_handshake_are_inert() {
        for after_version in [false, true] {
            for valid_fields in [false, true] {
                for (request, _) in requests(valid_fields) {
                    // Arrange: parsed valid/invalid fields cannot bypass handshake.
                    let mut manager = PeerManager::new(LocalPeerConfig::default());
                    manager.add_outbound_peer(1, 0).expect("peer");
                    if after_version {
                        manager
                            .handle_message(1, WireNetworkMessage::Version(Default::default()), 0)
                            .expect("version");
                    }

                    // Act
                    let actions = manager
                        .handle_message(1, request, 1)
                        .expect("inert receive");

                    // Assert
                    assert!(
                        actions.is_empty(),
                        "version={after_version}, valid={valid_fields}"
                    );
                }
            }
        }
    }
}
