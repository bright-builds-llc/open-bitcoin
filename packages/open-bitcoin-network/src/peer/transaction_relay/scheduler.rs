// Parity breadcrumbs:
// - packages/bitcoin-knots/src/protocol.h
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/node/txdownloadman.h
// - packages/bitcoin-knots/src/node/txdownloadman_impl.cpp
// - packages/bitcoin-knots/src/txrequest.h
// - packages/bitcoin-knots/src/txrequest.cpp
// - packages/bitcoin-knots/test/functional/p2p_tx_download.py

use std::collections::{BTreeMap, BTreeSet};

use open_bitcoin_primitives::{Hash32, InventoryVector, Txid, Wtxid};

use crate::error::PeerId;

use super::{
    TxDownloadAction, TxDownloadPolicy, TxDownloadSuppressionReason, TxRelayId,
    TxRelayIdentityError, TxRelayPeerMode,
};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct TxDownloadLocalFacts {
    pub already_have: BTreeSet<TxRelayId>,
    pub recent_rejects: BTreeSet<TxRelayId>,
    pub mempool_known: BTreeSet<TxRelayId>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TxAnnouncementInput {
    pub peer_id: PeerId,
    pub inventory: InventoryVector,
    pub peer_mode: TxRelayPeerMode,
    pub now_unix_seconds: i64,
    pub local_facts: TxDownloadLocalFacts,
    pub preferred_peer: bool,
    pub peer_overloaded: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TxPeerRequestSnapshot {
    pub peer_id: PeerId,
    pub candidate_count: usize,
    pub in_flight_count: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TxDownloadSnapshot {
    pub candidate_count: usize,
    pub in_flight_count: usize,
    pub already_have_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TxDownloadScheduler {
    policy: TxDownloadPolicy,
    announcements: BTreeMap<TxRelayId, BTreeMap<PeerId, TxAnnouncement>>,
    in_flight: BTreeMap<TxRelayId, InFlightRequest>,
    already_have: BTreeSet<TxRelayId>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct TxAnnouncement {
    peer_id: PeerId,
    ready_at_unix_seconds: i64,
    preferred_peer: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct InFlightRequest {
    peer_id: PeerId,
    expires_at_unix_seconds: i64,
}

impl TxDownloadScheduler {
    pub fn new(policy: TxDownloadPolicy) -> Self {
        Self {
            policy,
            announcements: BTreeMap::new(),
            in_flight: BTreeMap::new(),
            already_have: BTreeSet::new(),
        }
    }

    pub fn record_announcement(&mut self, input: TxAnnouncementInput) -> Vec<TxDownloadAction> {
        let relay_id =
            match TxRelayId::from_inventory_vector_for_peer(&input.inventory, input.peer_mode) {
                Ok(relay_id) => relay_id,
                Err(TxRelayIdentityError::NegotiationMismatch { .. }) => {
                    return vec![TxDownloadAction::SuppressIdentityMismatch {
                        peer_id: input.peer_id,
                        reason: TxDownloadSuppressionReason::IdentityMismatch,
                    }];
                }
                Err(TxRelayIdentityError::NotTransactionInventory { .. }) => {
                    return vec![TxDownloadAction::SuppressIdentityMismatch {
                        peer_id: input.peer_id,
                        reason: TxDownloadSuppressionReason::NotTransactionInventory,
                    }];
                }
            };

        if self.already_have.contains(&relay_id)
            || input.local_facts.already_have.contains(&relay_id)
        {
            self.mark_already_have(relay_id);
            return vec![TxDownloadAction::SuppressAlreadyHave {
                peer_id: input.peer_id,
                relay_id,
            }];
        }

        if input.local_facts.recent_rejects.contains(&relay_id) {
            return vec![TxDownloadAction::SuppressRecentReject {
                peer_id: input.peer_id,
                relay_id,
            }];
        }

        if input.local_facts.mempool_known.contains(&relay_id) {
            return vec![TxDownloadAction::Suppress {
                peer_id: input.peer_id,
                relay_id,
                reason: TxDownloadSuppressionReason::MempoolKnown,
            }];
        }

        if self.has_pending_relay(relay_id) {
            self.record_fallback_candidate_if_allowed(relay_id, &input);
            return vec![TxDownloadAction::SuppressDuplicate {
                peer_id: input.peer_id,
                relay_id,
            }];
        }

        if self.peer_is_at_request_cap(input.peer_id) {
            return vec![TxDownloadAction::SuppressRequestCap {
                peer_id: input.peer_id,
                relay_id,
            }];
        }

        let ready_at_unix_seconds = self.ready_at_unix_seconds(relay_id, &input);
        if ready_at_unix_seconds <= input.now_unix_seconds {
            self.insert_in_flight(relay_id, input.peer_id, input.now_unix_seconds);
            return vec![TxDownloadAction::RequestGetData {
                peer_id: input.peer_id,
                relay_id,
            }];
        }

        self.insert_candidate(
            relay_id,
            input.peer_id,
            ready_at_unix_seconds,
            input.preferred_peer,
        );
        Vec::new()
    }

    pub fn expire_and_schedule(&mut self, now_unix_seconds: i64) -> Vec<TxDownloadAction> {
        let expired: Vec<_> = self
            .in_flight
            .iter()
            .filter(|(_, request)| request.expires_at_unix_seconds <= now_unix_seconds)
            .map(|(relay_id, request)| (*relay_id, *request))
            .collect();
        let mut actions = Vec::new();

        for (relay_id, request) in expired {
            self.in_flight.remove(&relay_id);
            actions.push(TxDownloadAction::RequestExpired {
                peer_id: request.peer_id,
                relay_id,
            });
            if let Some(action) = self.schedule_relay(relay_id, now_unix_seconds, true) {
                actions.push(action);
            }
        }

        actions.extend(self.schedule_ready_candidates(now_unix_seconds));
        actions
    }

    pub fn record_notfound(
        &mut self,
        peer_id: PeerId,
        relay_id: TxRelayId,
        now_unix_seconds: i64,
    ) -> Vec<TxDownloadAction> {
        let Some(request) = self.in_flight.get(&relay_id).copied() else {
            return Vec::new();
        };
        if request.peer_id != peer_id {
            return Vec::new();
        }

        self.in_flight.remove(&relay_id);
        let mut actions = vec![TxDownloadAction::NotFoundCleanup { peer_id, relay_id }];
        if let Some(action) = self.schedule_relay(relay_id, now_unix_seconds, true) {
            actions.push(action);
        }
        actions
    }

    pub fn record_received_transaction(
        &mut self,
        peer_id: PeerId,
        txid: Txid,
        wtxid: Wtxid,
    ) -> Vec<TxDownloadAction> {
        let txid_relay_id = TxRelayId::Txid(txid);
        let wtxid_relay_id = TxRelayId::Wtxid(wtxid);
        let peer_has_in_flight = self
            .in_flight
            .values()
            .any(|request| request.peer_id == peer_id);
        let matches_peer_in_flight = self.in_flight.iter().any(|(relay_id, request)| {
            request.peer_id == peer_id
                && (*relay_id == txid_relay_id || *relay_id == wtxid_relay_id)
        });

        if peer_has_in_flight && !matches_peer_in_flight {
            return vec![TxDownloadAction::SuppressIdentityMismatch {
                peer_id,
                reason: TxDownloadSuppressionReason::IdentityMismatch,
            }];
        }

        self.mark_already_have(txid_relay_id);
        self.mark_already_have(wtxid_relay_id);
        vec![TxDownloadAction::ReceivedTxCleanup {
            peer_id,
            txid,
            wtxid,
        }]
    }

    pub fn cleanup_peer(
        &mut self,
        peer_id: PeerId,
        now_unix_seconds: i64,
    ) -> Vec<TxDownloadAction> {
        let removed_candidates = self.remove_peer_candidates(peer_id);
        let removed_in_flight: Vec<_> = self
            .in_flight
            .iter()
            .filter(|(_, request)| request.peer_id == peer_id)
            .map(|(relay_id, _)| *relay_id)
            .collect();

        for relay_id in &removed_in_flight {
            self.in_flight.remove(relay_id);
        }

        if !removed_candidates && removed_in_flight.is_empty() {
            return Vec::new();
        }

        let mut actions = vec![TxDownloadAction::PeerCleanup { peer_id }];
        for relay_id in removed_in_flight {
            if let Some(action) = self.schedule_relay(relay_id, now_unix_seconds, true) {
                actions.push(action);
            }
        }
        actions
    }

    pub fn mark_already_have(&mut self, relay_id: TxRelayId) {
        self.already_have.insert(relay_id);
        self.announcements.remove(&relay_id);
        self.in_flight.remove(&relay_id);
    }

    pub fn snapshot(&self) -> TxDownloadSnapshot {
        TxDownloadSnapshot {
            candidate_count: self.candidate_count(),
            in_flight_count: self.in_flight.len(),
            already_have_count: self.already_have.len(),
        }
    }

    pub fn peer_snapshot(&self, peer_id: PeerId) -> TxPeerRequestSnapshot {
        TxPeerRequestSnapshot {
            peer_id,
            candidate_count: self.peer_candidate_count(peer_id),
            in_flight_count: self.peer_in_flight_count(peer_id),
        }
    }

    fn ready_at_unix_seconds(&self, relay_id: TxRelayId, input: &TxAnnouncementInput) -> i64 {
        let mut delay_seconds = 0;
        if matches!(relay_id, TxRelayId::Txid(_)) && self.has_wtxid_for_hash(relay_id.object_hash())
        {
            delay_seconds += self.policy.txid_relay_delay_seconds;
        }
        if !input.preferred_peer {
            delay_seconds += self.policy.non_preferred_peer_delay_seconds;
        }
        if input.peer_overloaded {
            delay_seconds += self.policy.overloaded_peer_delay_seconds;
        }
        input.now_unix_seconds.saturating_add(delay_seconds)
    }

    fn has_wtxid_for_hash(&self, hash: Hash32) -> bool {
        self.announcements.keys().chain(self.in_flight.keys()).any(
            |relay_id| matches!(relay_id, TxRelayId::Wtxid(wtxid) if Hash32::from(*wtxid) == hash),
        )
    }

    fn has_pending_relay(&self, relay_id: TxRelayId) -> bool {
        self.in_flight.contains_key(&relay_id)
            || self
                .announcements
                .get(&relay_id)
                .is_some_and(|announcements| !announcements.is_empty())
    }

    fn record_fallback_candidate_if_allowed(
        &mut self,
        relay_id: TxRelayId,
        input: &TxAnnouncementInput,
    ) {
        if self.peer_total_count(input.peer_id) >= self.policy.max_announcements_per_peer {
            return;
        }

        let ready_at_unix_seconds = self.ready_at_unix_seconds(relay_id, input);
        self.insert_candidate(
            relay_id,
            input.peer_id,
            ready_at_unix_seconds,
            input.preferred_peer,
        );
    }

    fn peer_is_at_request_cap(&self, peer_id: PeerId) -> bool {
        self.peer_in_flight_count(peer_id) >= self.policy.max_in_flight_per_peer
            || self.peer_total_count(peer_id) >= self.policy.max_announcements_per_peer
    }

    fn insert_candidate(
        &mut self,
        relay_id: TxRelayId,
        peer_id: PeerId,
        ready_at_unix_seconds: i64,
        preferred_peer: bool,
    ) {
        self.announcements
            .entry(relay_id)
            .or_default()
            .entry(peer_id)
            .or_insert(TxAnnouncement {
                peer_id,
                ready_at_unix_seconds,
                preferred_peer,
            });
    }

    fn insert_in_flight(&mut self, relay_id: TxRelayId, peer_id: PeerId, now_unix_seconds: i64) {
        self.in_flight.insert(
            relay_id,
            InFlightRequest {
                peer_id,
                expires_at_unix_seconds: now_unix_seconds
                    .saturating_add(self.policy.getdata_tx_interval_seconds),
            },
        );
    }

    fn schedule_ready_candidates(&mut self, now_unix_seconds: i64) -> Vec<TxDownloadAction> {
        let relay_ids: Vec<_> = self.announcements.keys().copied().collect();
        let mut actions = Vec::new();

        for relay_id in relay_ids {
            if let Some(action) = self.schedule_relay(relay_id, now_unix_seconds, false) {
                actions.push(action);
            }
        }

        actions
    }

    fn schedule_relay(
        &mut self,
        relay_id: TxRelayId,
        now_unix_seconds: i64,
        fallback: bool,
    ) -> Option<TxDownloadAction> {
        if self.in_flight.contains_key(&relay_id) {
            return None;
        }

        let maybe_candidate = self.select_ready_candidate(relay_id, now_unix_seconds)?;
        self.remove_candidate(relay_id, maybe_candidate.peer_id);
        self.insert_in_flight(relay_id, maybe_candidate.peer_id, now_unix_seconds);

        if fallback {
            return Some(TxDownloadAction::FallbackRequest {
                peer_id: maybe_candidate.peer_id,
                relay_id,
            });
        }

        Some(TxDownloadAction::RequestGetData {
            peer_id: maybe_candidate.peer_id,
            relay_id,
        })
    }

    fn select_ready_candidate(
        &self,
        relay_id: TxRelayId,
        now_unix_seconds: i64,
    ) -> Option<TxAnnouncement> {
        self.announcements
            .get(&relay_id)?
            .values()
            .filter(|announcement| announcement.ready_at_unix_seconds <= now_unix_seconds)
            .filter(|announcement| {
                self.peer_in_flight_count(announcement.peer_id) < self.policy.max_in_flight_per_peer
            })
            .min_by_key(|announcement| (!announcement.preferred_peer, announcement.peer_id))
            .copied()
    }

    fn remove_candidate(&mut self, relay_id: TxRelayId, peer_id: PeerId) {
        let mut should_remove_relay = false;
        if let Some(announcements) = self.announcements.get_mut(&relay_id) {
            announcements.remove(&peer_id);
            should_remove_relay = announcements.is_empty();
        }
        if should_remove_relay {
            self.announcements.remove(&relay_id);
        }
    }

    fn remove_peer_candidates(&mut self, peer_id: PeerId) -> bool {
        let mut removed_any = false;
        let relay_ids: Vec<_> = self.announcements.keys().copied().collect();

        for relay_id in relay_ids {
            let mut should_remove_relay = false;
            if let Some(announcements) = self.announcements.get_mut(&relay_id) {
                removed_any |= announcements.remove(&peer_id).is_some();
                should_remove_relay = announcements.is_empty();
            }
            if should_remove_relay {
                self.announcements.remove(&relay_id);
            }
        }

        removed_any
    }

    fn candidate_count(&self) -> usize {
        self.announcements
            .values()
            .map(BTreeMap::len)
            .sum::<usize>()
    }

    fn peer_candidate_count(&self, peer_id: PeerId) -> usize {
        self.announcements
            .values()
            .filter(|announcements| announcements.contains_key(&peer_id))
            .count()
    }

    fn peer_in_flight_count(&self, peer_id: PeerId) -> usize {
        self.in_flight
            .values()
            .filter(|request| request.peer_id == peer_id)
            .count()
    }

    fn peer_total_count(&self, peer_id: PeerId) -> usize {
        self.peer_candidate_count(peer_id) + self.peer_in_flight_count(peer_id)
    }
}
