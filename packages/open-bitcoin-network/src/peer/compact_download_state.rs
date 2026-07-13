// Parity breadcrumbs:
// - packages/bitcoin-knots/src/blockencodings.h
// - packages/bitcoin-knots/src/blockencodings.cpp
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/net_processing.h
// - packages/bitcoin-knots/test/functional/p2p_compactblocks.py

use open_bitcoin_codec::{BIP152_COMPACT_BLOCKS_VERSION, BlockTransactions, CompactBlockPayload};
use open_bitcoin_consensus::block_hash;
use open_bitcoin_primitives::{BlockHash, Transaction, Wtxid};

use crate::PeerAction;
use crate::block_serving::BlockRelayActivationPolicy;
use crate::compact_download::{
    COMPACT_BLOCK_DOWNLOAD_TIMEOUT_SECONDS, CompactBlockDownloadEligibilityInput,
    CompactBlockInitOutcome, CompactBlockTxnHandleOutcome, CompactDownloadAction,
    CompactDownloadCleanupCause, CompactDownloadPeerState, FullBlockFetch,
    cleanup_compact_download_on_block_connected, cleanup_compact_download_peer,
    compact_download_actions_to_peer_actions, expire_stale_compact_downloads,
    handle_block_transactions, init_compact_block_download,
};
use crate::error::{NetworkError, PeerId};

use super::{CompactRelayCapability, PeerManager, PeerState};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct CompactBlockReceiveFacts<'a> {
    pub candidates: &'a [(&'a Wtxid, &'a Transaction)],
    pub extra: &'a [(&'a Wtxid, &'a Transaction)],
}

impl PeerManager {
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

    pub fn cleanup_compact_download_for_peer(
        &mut self,
        peer_id: PeerId,
        cause: CompactDownloadCleanupCause,
    ) -> Result<usize, NetworkError> {
        let Some(state) = self.compact_download_states.get_mut(&peer_id) else {
            return Ok(0);
        };
        Ok(cleanup_compact_download_peer(state, cause))
    }

    pub fn cleanup_all_compact_downloads(&mut self, cause: CompactDownloadCleanupCause) {
        for state in self.compact_download_states.values_mut() {
            let _ = cleanup_compact_download_peer(state, cause);
        }
    }

    pub fn on_compact_download_block_connected(&mut self, block_hash: BlockHash) {
        for state in self.compact_download_states.values_mut() {
            let _ = cleanup_compact_download_on_block_connected(state, block_hash);
        }
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
        let peer = Self::peer_mut(&mut self.peers, peer_id)?;
        let peer_compact_capable = peer_compact_capable(peer);
        let activation = self.block_relay_activation;
        let download_state = self.compact_download_states.entry(peer_id).or_default();
        let block_hash = block_hash(&payload.header);
        let eligibility = CompactBlockDownloadEligibilityInput {
            local_best_height: self.headers.best_height(),
            best_tip_hash: self
                .headers
                .best_tip()
                .map(|entry| entry.block_hash)
                .unwrap_or(payload.header.previous_block_hash),
            maybe_known_block_height: self.headers.entry(&block_hash).map(|entry| entry.height),
        };

        let outcome = init_compact_block_download(
            activation,
            peer_compact_capable,
            download_state,
            &payload,
            eligibility,
            now_unix_seconds,
            facts.candidates.iter().copied(),
            facts.extra.iter().copied(),
        );

        Ok(compact_block_init_actions(outcome))
    }

    pub fn expire_compact_download_timeouts(&mut self, now_unix_seconds: i64) -> Vec<PeerAction> {
        let mut actions = Vec::new();
        for state in self.compact_download_states.values_mut() {
            let expired = expire_stale_compact_downloads(
                state,
                now_unix_seconds,
                COMPACT_BLOCK_DOWNLOAD_TIMEOUT_SECONDS,
            );
            for item in expired {
                actions.extend(compact_download_actions_to_peer_actions(vec![
                    CompactDownloadAction::RequestFullBlock(FullBlockFetch {
                        block_hash: item.block_hash,
                    }),
                ]));
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
        let Some(download_state) = self.compact_download_states.get_mut(&peer_id) else {
            return Ok(Vec::new());
        };

        let outcome =
            handle_block_transactions(activation, peer_compact_capable, download_state, &response);

        Ok(compact_block_txn_actions(outcome))
    }

    pub(super) fn compact_download_disconnect_cleanup(&mut self, peer_id: PeerId) {
        self.compact_download_states.remove(&peer_id);
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

fn compact_block_init_actions(outcome: CompactBlockInitOutcome) -> Vec<PeerAction> {
    match outcome {
        CompactBlockInitOutcome::Ready { actions, .. } => {
            compact_download_actions_to_peer_actions(actions)
        }
        CompactBlockInitOutcome::Completed { block } => vec![PeerAction::ReceivedBlock(block)],
        CompactBlockInitOutcome::Fallback(fetch) => compact_download_actions_to_peer_actions(vec![
            crate::compact_download::CompactDownloadAction::RequestFullBlock(fetch),
        ]),
        CompactBlockInitOutcome::Suppressed(_) => Vec::new(),
    }
}

fn compact_block_txn_actions(outcome: CompactBlockTxnHandleOutcome) -> Vec<PeerAction> {
    match outcome {
        CompactBlockTxnHandleOutcome::Progress { actions } => {
            compact_download_actions_to_peer_actions(actions)
        }
        CompactBlockTxnHandleOutcome::Misbehavior(_)
        | CompactBlockTxnHandleOutcome::UnexpectedBlockHash
        | CompactBlockTxnHandleOutcome::NoMatchingInFlight => Vec::new(),
    }
}
