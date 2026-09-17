// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/node/txdownloadman_impl.cpp
// - packages/bitcoin-knots/src/node/txdownloadman.h
// - packages/bitcoin-knots/src/node/mempool_persist.cpp
// - packages/bitcoin-knots/src/protocol.h
// - packages/bitcoin-knots/src/txorphanage.cpp
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/test/functional/mempool_persist.py
// - packages/bitcoin-knots/test/functional/p2p_getdata.py
// - packages/bitcoin-knots/test/functional/p2p_orphan_handling.py
// - packages/bitcoin-knots/test/functional/p2p_tx_download.py
// - packages/bitcoin-knots/test/functional/mempool_accept.py

use std::fmt;

use open_bitcoin_core::chainstate::{CoinsView, MemoryCoinsView};

use super::ManagedPeerNetwork;

impl<S, V: CoinsView> fmt::Debug for ManagedPeerNetwork<S, V> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.debug_struct("ManagedPeerNetwork").finish()
    }
}

impl<S: Clone> Clone for ManagedPeerNetwork<S, MemoryCoinsView> {
    fn clone(&self) -> Self {
        Self {
            chainstate: self.chainstate.clone(),
            mempool: self.mempool.clone(),
            peer_manager: self.peer_manager.clone(),
            known_peers: self.known_peers.clone(),
            inbound_admission_policy: self.inbound_admission_policy,
            inbound_admission_info: self.inbound_admission_info.clone(),
            resource_governance_info: self.resource_governance_info.clone(),
            relay_activation: self.relay_activation,
            block_relay_activation: self.block_relay_activation,
            inbound_serving_enabled: self.inbound_serving_enabled,
            block_relay_evidence: self.block_relay_evidence.clone(),
            have_bytes_accumulator: self.have_bytes_accumulator.clone(),
            relay_fanout: self.relay_fanout.clone(),
            relay_serving: self.relay_serving.clone(),
            compact_extra_txn: self.compact_extra_txn.clone(),
            authority_epoch: self.authority_epoch,
            lifecycle_generation: self.lifecycle_generation,
            dirty_generation: self.dirty_generation,
            unbroadcast_members: self.unbroadcast_members.clone(),
            maybe_retry_due_at_unix_seconds: self.maybe_retry_due_at_unix_seconds,
            maybe_unbroadcast_walk_cursor: self.maybe_unbroadcast_walk_cursor,
            maybe_last_transport_written_clear: self.maybe_last_transport_written_clear,
            lifecycle_evidence: self.lifecycle_evidence,
            checkpoint_evidence: self.checkpoint_evidence.clone(),
            peer_session_generations: self.peer_session_generations.clone(),
            peer_effect_ledger: self.peer_effect_ledger.clone(),
            snapshot_effect_ledger: self.snapshot_effect_ledger.clone(),
            latest_mempool_recovery: self.latest_mempool_recovery.clone(),
            latest_mempool_recovery_storage_error: self.latest_mempool_recovery_storage_error,
            local_config: self.local_config.clone(),
            blocks_by_hash: self.blocks_by_hash.clone(),
            transactions_by_txid: self.transactions_by_txid.clone(),
            transactions_by_wtxid: self.transactions_by_wtxid.clone(),
        }
    }
}
