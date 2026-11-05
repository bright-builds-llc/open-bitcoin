// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp

use super::*;
use open_bitcoin_consensus::{BlockValidationContext, CandidateRules};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum CompactRequestRules {
    Fixture { enforce_segwit: bool },
    Canonical(Box<(CandidateRules, BlockValidationContext)>),
}

impl CompactRequestRules {
    pub(super) fn enforce_segwit(&self) -> bool {
        match self {
            Self::Fixture { enforce_segwit } => *enforce_segwit,
            Self::Canonical(selected) => selected.0.enforce_segwit_commitment(),
        }
    }
}

#[derive(Debug, Clone)]
struct BlockRequest {
    peer_id: PeerId,
    ordinary: bool,
    maybe_compact_rules: Option<CompactRequestRules>,
    maybe_compact_attempt: Option<CompactAttempt>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum CompactAttempt {
    Active { original_start_unix: i64 },
    Consumed { original_start_unix: i64 },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum RequestPosition {
    First,
    Later,
    Absent,
}

/// Each vector records committed request arrival, independent of peer IDs or clocks.
#[derive(Debug, Clone, Default)]
pub(super) struct BlockRequestOrder {
    records: BTreeMap<BlockHash, Vec<BlockRequest>>,
}

impl PeerManager {
    pub(super) fn request_position(&self, peer_id: PeerId, hash: BlockHash) -> RequestPosition {
        let Some(records) = self.block_requests.records.get(&hash) else {
            return RequestPosition::Absent;
        };
        match records.iter().position(|record| record.peer_id == peer_id) {
            Some(0) => RequestPosition::First,
            Some(_) => RequestPosition::Later,
            None => RequestPosition::Absent,
        }
    }

    pub(super) fn maybe_compact_attempt(
        &self,
        peer_id: PeerId,
        hash: BlockHash,
    ) -> Option<CompactAttempt> {
        self.block_requests
            .records
            .get(&hash)?
            .iter()
            .find(|record| record.peer_id == peer_id)?
            .maybe_compact_attempt
    }

    pub(super) fn consume_compact_attempt(&mut self, peer_id: PeerId, hash: BlockHash) {
        let maybe_record = self
            .block_requests
            .records
            .get_mut(&hash)
            .and_then(|records| records.iter_mut().find(|record| record.peer_id == peer_id));
        if let Some(record) = maybe_record
            && let Some(CompactAttempt::Active {
                original_start_unix,
            }) = record.maybe_compact_attempt
        {
            record.maybe_compact_attempt = Some(CompactAttempt::Consumed {
                original_start_unix,
            });
        }
    }

    pub(super) fn start_compact_attempt(
        &mut self,
        peer_id: PeerId,
        hash: BlockHash,
        original_start_unix: i64,
    ) {
        let maybe_record = self
            .block_requests
            .records
            .get_mut(&hash)
            .and_then(|records| records.iter_mut().find(|record| record.peer_id == peer_id));
        if let Some(record) = maybe_record {
            record
                .maybe_compact_attempt
                .get_or_insert(CompactAttempt::Active {
                    original_start_unix,
                });
        }
    }

    /// Observe actual registration order; this snapshot grants no validation authority.
    pub fn block_request_peer_order(&self, hash: BlockHash) -> Vec<PeerId> {
        self.block_requests
            .records
            .get(&hash)
            .into_iter()
            .flatten()
            .map(|record| record.peer_id)
            .collect()
    }

    /// Observe the explicit selected compact rule for an existing request.
    pub fn maybe_block_request_enforce_segwit(
        &self,
        peer_id: PeerId,
        hash: BlockHash,
    ) -> Option<bool> {
        self.maybe_compact_request_rules(peer_id, hash)
            .map(CompactRequestRules::enforce_segwit)
    }

    pub(super) fn maybe_compact_request_rules(
        &self,
        peer_id: PeerId,
        hash: BlockHash,
    ) -> Option<&CompactRequestRules> {
        self.block_requests
            .records
            .get(&hash)?
            .iter()
            .find(|record| record.peer_id == peer_id)?
            .maybe_compact_rules
            .as_ref()
    }

    pub(super) fn register_block_request(
        &mut self,
        peer_id: PeerId,
        hash: BlockHash,
        maybe_compact_rules: Option<CompactRequestRules>,
        cap: usize,
    ) -> Result<bool, NetworkError> {
        let peer = Self::peer_mut(&mut self.peers, peer_id)?;
        if peer.requested_blocks.contains(&hash) {
            let Some(rules) = maybe_compact_rules else {
                return Ok(false);
            };
            let Some(record) = self
                .block_requests
                .records
                .get_mut(&hash)
                .and_then(|records| records.iter_mut().find(|record| record.peer_id == peer_id))
            else {
                return Ok(false);
            };
            if record
                .maybe_compact_rules
                .as_ref()
                .is_some_and(|selected| selected != &rules)
            {
                return Err(NetworkError::InvalidHeader {
                    reject_reason: "compact-request-context-mismatch".to_owned(),
                    maybe_debug_message: None,
                });
            }
            record.maybe_compact_rules = Some(rules);
            return Ok(true);
        }
        if peer.requested_blocks.len() >= self.max_blocks_in_flight_per_peer.min(cap) {
            return Ok(false);
        }
        peer.requested_blocks.insert(hash);
        self.block_requests
            .records
            .entry(hash)
            .or_default()
            .push(BlockRequest {
                peer_id,
                ordinary: maybe_compact_rules.is_none(),
                maybe_compact_rules,
                maybe_compact_attempt: None,
            });
        Ok(true)
    }

    pub(super) fn release_block_request(&mut self, peer_id: PeerId, hash: BlockHash) {
        if let Some(peer) = self.peers.get_mut(&peer_id) {
            peer.requested_blocks.remove(&hash);
        }
        if let Some(records) = self.block_requests.records.get_mut(&hash) {
            records.retain(|record| record.peer_id != peer_id);
            if records.is_empty() {
                self.block_requests.records.remove(&hash);
            }
        }
        if let Some(state) = self.compact_download_states.get_mut(&peer_id) {
            state.in_flight.remove(&hash);
        }
    }

    /// Finish every request for one retained block, preserving unrelated request order.
    pub fn finish_block_requests(&mut self, hash: BlockHash) -> usize {
        self.block_requests.records.remove(&hash);
        for peer in self.peers.values_mut() {
            peer.requested_blocks.remove(&hash);
        }
        self.compact_download_states
            .values_mut()
            .map(|state| state.in_flight.remove(&hash).is_some())
            .filter(|removed| *removed)
            .count()
    }

    pub(super) fn release_peer_block_requests(&mut self, peer_id: PeerId) {
        let hashes: Vec<_> = self
            .block_requests
            .records
            .iter()
            .filter_map(|(hash, records)| {
                records
                    .iter()
                    .any(|record| record.peer_id == peer_id)
                    .then_some(*hash)
            })
            .collect();
        for hash in hashes {
            self.release_block_request(peer_id, hash);
        }
    }

    pub(super) fn compact_request_hashes(&self, peer_id: PeerId) -> Vec<BlockHash> {
        self.block_requests
            .records
            .iter()
            .filter_map(|(hash, records)| {
                records
                    .iter()
                    .any(|record| record.peer_id == peer_id && record.maybe_compact_rules.is_some())
                    .then_some(*hash)
            })
            .collect()
    }

    pub(super) fn detach_compact_block_request(&mut self, peer_id: PeerId, hash: BlockHash) {
        let maybe_record = self
            .block_requests
            .records
            .get_mut(&hash)
            .and_then(|records| records.iter_mut().find(|record| record.peer_id == peer_id));
        let Some(record) = maybe_record else {
            return;
        };
        record.maybe_compact_rules = None;
        record.maybe_compact_attempt = None;
        if !record.ordinary {
            self.release_block_request(peer_id, hash);
        }
    }
}
