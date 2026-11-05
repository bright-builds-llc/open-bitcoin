// Parity breadcrumbs:
// - packages/bitcoin-knots/src/blockencodings.h
// - packages/bitcoin-knots/src/blockencodings.cpp
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/net_processing.h
// - packages/bitcoin-knots/test/functional/p2p_compactblocks.py

use open_bitcoin_codec::{BIP152_COMPACT_BLOCKS_VERSION, BlockTransactions, CompactBlockPayload};
use open_bitcoin_consensus::block_hash;
use open_bitcoin_primitives::{BlockHash, BlockHeader, InventoryType, Transaction, Wtxid};

use crate::PeerAction;
use crate::block_serving::BlockRelayActivationPolicy;
use crate::compact_download::{
    COMPACT_BLOCK_DOWNLOAD_TIMEOUT_SECONDS, CompactBlockDownloadEligibilityInput,
    CompactBlockInitOutcome, CompactBlockTxnHandleOutcome, CompactDownloadAction,
    CompactDownloadCleanupCause, CompactDownloadPeerState, FullBlockFetch,
    cleanup_compact_download_on_block_connected, cleanup_compact_download_peer,
    compact_download_actions_to_peer_actions_selected, expire_stale_compact_downloads,
    handle_block_transactions_selected, init_compact_block_download_selected,
};
use crate::error::{DisconnectReason, NetworkError, PeerId};

use super::block_requests::{CompactAttempt, CompactRequestRules, RequestPosition};
use super::{CompactRelayCapability, PeerManager, PeerState, ServiceFlags};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CompactBlockReceiveFacts<'a> {
    pub candidates: &'a [(&'a Wtxid, &'a Transaction)],
    pub extra: &'a [(&'a Wtxid, &'a Transaction)],
}

impl PeerManager {
    /// Factual preflight for the managed shell before a compact header can gain authority.
    pub fn compact_download_enabled_for_peer(&self, peer_id: PeerId) -> Result<bool, NetworkError> {
        let peer = self
            .peers
            .get(&peer_id)
            .ok_or(NetworkError::UnknownPeer(peer_id))?;
        Ok(self.block_relay_activation.compact_relay.enabled && peer_compact_capable(peer))
    }

    fn compact_fetch_inventory_type(&self, peer_id: PeerId) -> InventoryType {
        if self.peers.get(&peer_id).is_some_and(|peer| {
            ServiceFlags::from_bits(peer.remote_services_bits).contains(ServiceFlags::WITNESS)
        }) {
            InventoryType::WitnessBlock
        } else {
            InventoryType::Block
        }
    }

    fn compact_failure_actions(&mut self, peer_id: PeerId, hash: BlockHash) -> Vec<PeerAction> {
        match self.request_position(peer_id, hash) {
            RequestPosition::First => {
                if matches!(
                    self.maybe_compact_attempt(peer_id, hash),
                    Some(CompactAttempt::Consumed { .. })
                ) {
                    return Vec::new();
                }
                self.consume_compact_attempt(peer_id, hash);
                compact_download_actions_to_peer_actions_selected(
                    vec![CompactDownloadAction::RequestFullBlock(FullBlockFetch {
                        block_hash: hash,
                    })],
                    self.compact_fetch_inventory_type(peer_id),
                )
            }
            RequestPosition::Later => {
                self.release_block_request(peer_id, hash);
                Vec::new()
            }
            RequestPosition::Absent => Vec::new(),
        }
    }

    pub fn set_block_relay_activation_policy(&mut self, activation: BlockRelayActivationPolicy) {
        self.block_relay_activation = activation;
    }

    pub fn block_relay_activation_policy(&self) -> BlockRelayActivationPolicy {
        self.block_relay_activation
    }

    pub fn compact_download_peer_state(
        &self,
        peer_id: PeerId,
    ) -> Option<&CompactDownloadPeerState> {
        self.compact_download_states.get(&peer_id)
    }

    pub fn compact_download_peer_state_mut(
        &mut self,
        peer_id: PeerId,
    ) -> Option<&mut CompactDownloadPeerState> {
        self.compact_download_states.get_mut(&peer_id)
    }

    pub fn cleanup_compact_download_for_peer(
        &mut self,
        peer_id: PeerId,
        cause: CompactDownloadCleanupCause,
    ) -> Result<usize, NetworkError> {
        let cleared = self
            .compact_download_states
            .get_mut(&peer_id)
            .map(|state| cleanup_compact_download_peer(state, cause))
            .unwrap_or(0);
        for hash in self.compact_request_hashes(peer_id) {
            self.detach_compact_block_request(peer_id, hash);
        }
        Ok(cleared)
    }

    pub fn cleanup_all_compact_downloads(&mut self, cause: CompactDownloadCleanupCause) {
        for peer_id in self.peers.keys().copied().collect::<Vec<_>>() {
            if let Some(state) = self.compact_download_states.get_mut(&peer_id) {
                cleanup_compact_download_peer(state, cause);
            }
            for hash in self.compact_request_hashes(peer_id) {
                self.detach_compact_block_request(peer_id, hash);
            }
        }
    }

    /// Clear matching volatile compact in-flight slots across all peers.
    ///
    /// Returns the number of peers that had an in-flight entry removed for `block_hash`.
    pub fn on_compact_download_block_connected(&mut self, block_hash: BlockHash) -> usize {
        let mut removed_count = 0;
        for peer_id in self.peers.keys().copied().collect::<Vec<_>>() {
            if self
                .compact_download_states
                .get_mut(&peer_id)
                .is_some_and(|state| cleanup_compact_download_on_block_connected(state, block_hash))
            {
                removed_count += 1;
            }
            self.detach_compact_block_request(peer_id, block_hash);
        }
        removed_count
    }

    /// Clear volatile partial-compact slots that matched a mempool transaction by wtxid.
    ///
    /// Walks every peer's in-flight compact downloads. Does not schedule timeouts or
    /// mutate chainstate. The node shell supplies removed wtxids; this crate stays free
    /// of mempool types.
    pub fn on_mempool_transaction_removed(&mut self, removed_wtxid: &Wtxid) {
        for state in self.compact_download_states.values_mut() {
            for in_flight in state.in_flight.values_mut() {
                in_flight
                    .partial
                    .on_mempool_transaction_removed(removed_wtxid);
            }
        }
    }

    pub fn handle_compact_block_download(
        &mut self,
        peer_id: PeerId,
        payload: CompactBlockPayload,
        facts: CompactBlockReceiveFacts<'_>,
        now_unix_seconds: i64,
    ) -> Result<Vec<PeerAction>, NetworkError> {
        if !self.compact_download_enabled_for_peer(peer_id)? {
            return Ok(Vec::new());
        }
        let rules = self.canonical_compact_rules(&payload.header)?;
        self.handle_selected_compact_download(peer_id, payload, facts, now_unix_seconds, rules)
    }

    fn canonical_compact_rules(
        &self,
        header: &BlockHeader,
    ) -> Result<CompactRequestRules, NetworkError> {
        let hash = block_hash(header);
        if !self.headers.is_canonically_admitted(hash)
            || self
                .headers
                .entry(&hash)
                .is_none_or(|entry| entry.header != *header)
        {
            return Err(compact_context_error("missing-canonical-compact-context"));
        }
        let rules = self.headers.canonical_header_history(hash)?.rules();
        let context = self.headers.canonical_header_validation_context(header)?;
        Ok(CompactRequestRules::Canonical(Box::new((rules, context))))
    }

    pub fn expire_compact_download_timeouts(
        &mut self,
        now_unix_seconds: i64,
    ) -> Vec<(PeerId, PeerAction)> {
        let mut actions = Vec::new();
        for peer_id in self
            .compact_download_states
            .keys()
            .copied()
            .collect::<Vec<_>>()
        {
            let expired = self
                .compact_download_states
                .get_mut(&peer_id)
                .map(|state| {
                    expire_stale_compact_downloads(
                        state,
                        now_unix_seconds,
                        COMPACT_BLOCK_DOWNLOAD_TIMEOUT_SECONDS,
                    )
                })
                .unwrap_or_default();
            for item in expired {
                // Only a real, retained request can be resent without a new capacity slot.
                if self
                    .maybe_compact_request_rules(peer_id, item.block_hash)
                    .is_none()
                    || !self
                        .peers
                        .get(&peer_id)
                        .is_some_and(|peer| peer.requested_blocks.contains(&item.block_hash))
                {
                    continue;
                }
                for action in self.compact_failure_actions(peer_id, item.block_hash) {
                    actions.push((peer_id, action));
                }
            }
        }
        actions
    }

    pub(super) fn handle_block_transactions_message(
        &mut self,
        peer_id: PeerId,
        response: BlockTransactions,
    ) -> Result<Vec<PeerAction>, NetworkError> {
        let peer = Self::peer_mut(&mut self.peers, peer_id)?;
        let peer_compact_capable = peer_compact_capable(peer);
        let activation = self.block_relay_activation;
        let Some(partial) = self
            .compact_download_states
            .get(&peer_id)
            .and_then(|state| state.in_flight.get(&response.block_hash))
        else {
            if matches!(
                self.maybe_compact_attempt(peer_id, response.block_hash),
                Some(CompactAttempt::Consumed { .. })
            ) {
                self.release_block_request(peer_id, response.block_hash);
                return Ok(vec![PeerAction::Disconnect(
                    DisconnectReason::CompactBlockMisbehavior,
                )]);
            }
            return Ok(Vec::new());
        };
        let Some(header) = partial.partial.header() else {
            return Err(compact_context_error("missing-compact-request-binding"));
        };
        // Malformed embedded hashes keep their wire misbehavior precedence over rule lookup.
        if partial.partial.block_hash() != Some(response.block_hash) {
            if !partial.getblocktxn_in_flight {
                return Ok(vec![PeerAction::Disconnect(
                    DisconnectReason::CompactBlockMisbehavior,
                )]);
            }
            let state = self
                .compact_download_states
                .get_mut(&peer_id)
                .ok_or_else(|| compact_context_error("missing-compact-request-binding"))?;
            state.in_flight.remove(&response.block_hash);
            self.release_block_request(peer_id, response.block_hash);
            return Ok(compact_block_txn_actions(
                CompactBlockTxnHandleOutcome::UnexpectedBlockHash,
                self.compact_fetch_inventory_type(peer_id),
            ));
        }
        if !self
            .peers
            .get(&peer_id)
            .is_some_and(|peer| peer.requested_blocks.contains(&response.block_hash))
        {
            return Err(compact_context_error("missing-compact-request-binding"));
        }
        let Some(selected) = self.maybe_compact_request_rules(peer_id, response.block_hash) else {
            return Err(compact_context_error("missing-compact-request-binding"));
        };
        match selected {
            CompactRequestRules::Fixture { .. }
                if self.headers.maybe_canonical_chain().is_none() => {}
            CompactRequestRules::Canonical(_)
                if selected == &self.canonical_compact_rules(header)? => {}
            _ => return Err(compact_context_error("compact-request-context-mismatch")),
        }
        let check_witness_root = selected.enforce_segwit();
        let outcome = self
            .compact_download_states
            .get_mut(&peer_id)
            .map(|download_state| {
                handle_block_transactions_selected(
                    activation,
                    peer_compact_capable,
                    download_state,
                    &response,
                    check_witness_root,
                )
            })
            .transpose()?
            .unwrap_or(CompactBlockTxnHandleOutcome::NoMatchingInFlight);

        match outcome {
            CompactBlockTxnHandleOutcome::Progress { actions } => {
                let mut translated = Vec::new();
                for action in actions {
                    match action {
                        CompactDownloadAction::RequestFullBlock(fetch) => {
                            translated
                                .extend(self.compact_failure_actions(peer_id, fetch.block_hash));
                        }
                        other => {
                            translated.extend(compact_download_actions_to_peer_actions_selected(
                                vec![other],
                                self.compact_fetch_inventory_type(peer_id),
                            ))
                        }
                    }
                }
                Ok(translated)
            }
            other => {
                self.release_block_request(peer_id, response.block_hash);
                Ok(compact_block_txn_actions(
                    other,
                    self.compact_fetch_inventory_type(peer_id),
                ))
            }
        }
    }

    pub(super) fn compact_download_disconnect_cleanup(&mut self, peer_id: PeerId) {
        self.compact_download_states.remove(&peer_id);
    }

    /// Explicit fixture rule data grants no canonical acceptance or storage authority.
    ///
    /// ```no_run
    /// use open_bitcoin_network::{PeerManager, CompactBlockReceiveFacts};
    /// use open_bitcoin_codec::CompactBlockPayload;
    /// fn fixture_compact(manager: &mut PeerManager, payload: CompactBlockPayload) {
    ///     let hash = open_bitcoin_consensus::block_hash(&payload.header);
    ///     let fixture_enforce_segwit = false;
    ///     let result = manager.handle_fixture_compact_block_download(1, payload,
    ///         CompactBlockReceiveFacts { candidates: &[], extra: &[] }, 0, fixture_enforce_segwit);
    ///     let peers: Vec<open_bitcoin_network::PeerId> = manager.block_request_peer_order(hash);
    ///     let _observed = (result, peers);
    /// }
    /// ```
    pub fn handle_fixture_compact_block_download(
        &mut self,
        peer_id: PeerId,
        payload: CompactBlockPayload,
        facts: CompactBlockReceiveFacts<'_>,
        now_unix_seconds: i64,
        enforce_segwit: bool,
    ) -> Result<Vec<PeerAction>, NetworkError> {
        if self.headers.maybe_canonical_chain().is_some() {
            return Err(compact_context_error(
                "canonical-owner-refuses-fixture-compact",
            ));
        }
        self.handle_selected_compact_download(
            peer_id,
            payload,
            facts,
            now_unix_seconds,
            CompactRequestRules::Fixture { enforce_segwit },
        )
    }

    fn handle_selected_compact_download(
        &mut self,
        peer_id: PeerId,
        payload: CompactBlockPayload,
        facts: CompactBlockReceiveFacts<'_>,
        now_unix_seconds: i64,
        rules: CompactRequestRules,
    ) -> Result<Vec<PeerAction>, NetworkError> {
        let capable = peer_compact_capable(Self::peer_mut(&mut self.peers, peer_id)?);
        let hash = block_hash(&payload.header);
        if matches!(
            self.maybe_compact_attempt(peer_id, hash),
            Some(CompactAttempt::Consumed { .. })
        ) {
            return Ok(Vec::new());
        }
        let eligibility = CompactBlockDownloadEligibilityInput {
            local_best_height: self.headers.best_height(),
            best_tip_hash: self
                .headers
                .best_tip()
                .map(|entry| entry.block_hash)
                .unwrap_or(payload.header.previous_block_hash),
            maybe_known_block_height: self.headers.entry(&hash).map(|entry| entry.height),
        };
        // Stage only this hash; capacity refusal cannot leave a newly installed partial.
        let mut staged = CompactDownloadPeerState::default();
        let state = self.compact_download_states.entry(peer_id).or_default();
        if let Some(existing) = state.in_flight.get(&hash) {
            staged.in_flight.insert(hash, existing.clone());
        }
        let outcome = init_compact_block_download_selected(
            (self.block_relay_activation, capable),
            &mut staged,
            &payload,
            (eligibility, now_unix_seconds, rules.enforce_segwit()),
            facts.candidates.iter().copied(),
            facts.extra.iter().copied(),
        )?;
        if matches!(&outcome, CompactBlockInitOutcome::Ready { actions, .. } if !actions.is_empty())
            || matches!(&outcome, CompactBlockInitOutcome::Fallback(_))
        {
            if !self.register_block_request(
                peer_id,
                hash,
                Some(rules),
                crate::PHASE94_MAX_INBOUND_BLOCK_REQUESTS_PER_PEER,
            )? {
                return Ok(Vec::new());
            }
            self.start_compact_attempt(peer_id, hash, now_unix_seconds);
            if let Some(partial) = staged.in_flight.remove(&hash) {
                self.compact_download_states
                    .entry(peer_id)
                    .or_default()
                    .in_flight
                    .insert(hash, partial);
            }
        }
        match outcome {
            CompactBlockInitOutcome::Fallback(fetch) => {
                Ok(self.compact_failure_actions(peer_id, fetch.block_hash))
            }
            other => Ok(compact_block_init_actions(
                other,
                self.compact_fetch_inventory_type(peer_id),
            )),
        }
    }
}

fn compact_context_error(reason: &str) -> NetworkError {
    NetworkError::InvalidHeader {
        reject_reason: reason.to_owned(),
        maybe_debug_message: None,
    }
}

fn peer_compact_capable(peer: &PeerState) -> bool {
    matches!(
        peer.compact_relay.capability,
        CompactRelayCapability::Supported {
            version: BIP152_COMPACT_BLOCKS_VERSION,
        }
    )
}

fn compact_block_init_actions(
    outcome: CompactBlockInitOutcome,
    inventory_type: InventoryType,
) -> Vec<PeerAction> {
    match outcome {
        CompactBlockInitOutcome::Ready { actions, .. } => {
            compact_download_actions_to_peer_actions_selected(actions, inventory_type)
        }
        CompactBlockInitOutcome::Completed { block } => vec![PeerAction::ReceivedBlock(block)],
        CompactBlockInitOutcome::Fallback(fetch) => {
            compact_download_actions_to_peer_actions_selected(
                vec![crate::compact_download::CompactDownloadAction::RequestFullBlock(fetch)],
                inventory_type,
            )
        }
        CompactBlockInitOutcome::Misbehavior(_) => {
            vec![PeerAction::Disconnect(
                DisconnectReason::CompactBlockHeaderViolation,
            )]
        }
        CompactBlockInitOutcome::Suppressed(_) => Vec::new(),
    }
}

fn compact_block_txn_actions(
    outcome: CompactBlockTxnHandleOutcome,
    inventory_type: InventoryType,
) -> Vec<PeerAction> {
    match outcome {
        CompactBlockTxnHandleOutcome::Progress { actions } => {
            compact_download_actions_to_peer_actions_selected(actions, inventory_type)
        }
        CompactBlockTxnHandleOutcome::Misbehavior(_)
        | CompactBlockTxnHandleOutcome::UnexpectedBlockHash => {
            vec![PeerAction::Disconnect(
                DisconnectReason::CompactBlockMisbehavior,
            )]
        }
        CompactBlockTxnHandleOutcome::NoMatchingInFlight => Vec::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn consumed_and_absent_compact_fallbacks_never_send_a_second_request() {
        // Arrange
        let peer_id = 17;
        let hash = BlockHash::from_byte_array([17; 32]);
        let absent = BlockHash::from_byte_array([18; 32]);
        let mut manager = PeerManager::new(Default::default());
        manager.add_outbound_peer(peer_id, 0).expect("peer");
        assert!(
            manager
                .register_block_request(
                    peer_id,
                    hash,
                    Some(CompactRequestRules::Fixture {
                        enforce_segwit: true,
                    }),
                    2,
                )
                .expect("initial compact request")
        );
        manager.start_compact_attempt(peer_id, hash, 10);
        manager.consume_compact_attempt(peer_id, hash);

        // Act
        let consumed_actions = manager.compact_failure_actions(peer_id, hash);
        let absent_actions = manager.compact_failure_actions(peer_id, absent);

        // Assert
        assert!(consumed_actions.is_empty());
        assert!(absent_actions.is_empty());
        assert_eq!(manager.block_request_peer_order(hash), vec![peer_id]);
        assert!(manager.block_request_peer_order(absent).is_empty());
    }

    #[test]
    fn compact_outcome_translation_keeps_full_fetch_and_absence_distinct() {
        // Arrange
        let hash = BlockHash::from_byte_array([19; 32]);
        let fetch = FullBlockFetch { block_hash: hash };

        // Act
        let init = compact_block_init_actions(
            CompactBlockInitOutcome::Fallback(fetch.clone()),
            InventoryType::WitnessBlock,
        );
        let progress = compact_block_txn_actions(
            CompactBlockTxnHandleOutcome::Progress {
                actions: vec![CompactDownloadAction::RequestFullBlock(fetch)],
            },
            InventoryType::Block,
        );
        let absent = compact_block_txn_actions(
            CompactBlockTxnHandleOutcome::NoMatchingInFlight,
            InventoryType::Block,
        );

        // Assert
        assert!(
            matches!(init.as_slice(), [PeerAction::Send(crate::WireNetworkMessage::GetData(list))]
            if list.inventory.len() == 1 && list.inventory[0].inventory_type == InventoryType::WitnessBlock
                && list.inventory[0].object_hash == hash.into())
        );
        assert!(
            matches!(progress.as_slice(), [PeerAction::Send(crate::WireNetworkMessage::GetData(list))]
            if list.inventory.len() == 1 && list.inventory[0].inventory_type == InventoryType::Block
                && list.inventory[0].object_hash == hash.into())
        );
        assert!(absent.is_empty());
    }
}
