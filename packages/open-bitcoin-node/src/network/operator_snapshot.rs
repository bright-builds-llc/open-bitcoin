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

//! Identifier-free operator snapshot assembly for shared mempool evidence.

#[cfg(test)]
use open_bitcoin_mempool::MempoolMemberIdentity;

use super::{ManagedNetworkOperatorSnapshot, ManagedPeerNetwork};
use crate::ChainstateStore;
use crate::status::relay_evidence::{RelayEvidenceCounters, RelayEvidenceField};
use crate::status::{
    MempoolEvictionGroup, MempoolPressureGroup, admission_group_from_counts,
    checkpoint_group_from_evidence, decay_half_life_label, recovery_group_from_summary,
    retry_group_from_relay,
};

/// Periodic checkpoint interval used to derive overdue and loss-bound facts.
const OPERATOR_CHECKPOINT_INTERVAL_SECONDS: u64 = 300;

impl<S: ChainstateStore> ManagedPeerNetwork<S> {
    pub fn operator_snapshot(&self) -> ManagedNetworkOperatorSnapshot {
        let block_relay = self.block_relay_runtime_evidence_snapshot();
        let mempool = self.mempool_info();
        let network = self.network_info();
        let relay_enabled = network.relay;
        let relay = self.relay_evidence_status();
        let evidence = self.lifecycle_evidence;
        let pressure_removal_count = evidence.pressure_removals;
        let outcome_counters = match &relay.outcome_counters {
            RelayEvidenceField::Implemented(counters) => *counters,
            RelayEvidenceField::Unavailable { .. }
            | RelayEvidenceField::Deferred { .. }
            | RelayEvidenceField::IntentionallyDifferent { .. } => RelayEvidenceCounters::default(),
        };
        let checkpoint = checkpoint_group_from_evidence(&self.checkpoint_evidence.snapshot(
            self.lifecycle_generation,
            self.dirty_generation,
            self.mempool.mempool().rolling_fee_last_update(),
            OPERATOR_CHECKPOINT_INTERVAL_SECONDS,
        ));
        let recovery = self
            .latest_mempool_recovery
            .as_ref()
            .map(recovery_group_from_summary)
            .unwrap_or_default();
        ManagedNetworkOperatorSnapshot {
            network,
            mempool: mempool.clone(),
            relay,
            block_relay: block_relay.status,
            block_served_count: block_relay.served_count,
            inbound_admission: self.inbound_admission_info().clone(),
            address_boundary: self.address_boundary_info(),
            peer_policy: self.peer_policy_info(),
            resource_governance: self.resource_governance_info(),
            pressure: MempoolPressureGroup {
                pressure_removal_count,
                decay_half_life_label: decay_half_life_label(
                    mempool.accounted_memory,
                    mempool.mempool_capacity,
                    self.mempool.mempool().rolling_fee_decay_gate_open(),
                )
                .to_string(),
            },
            eviction: MempoolEvictionGroup {
                pressure_removal_count,
            },
            checkpoint,
            recovery,
            retry: retry_group_from_relay(
                self.unbroadcast_members.len() as u64,
                self.relay_fanout_info().queued_transactions as u64,
                0,
                outcome_counters.announced_count,
                &outcome_counters,
                relay_enabled,
                evidence.retry_clears,
            ),
            admission: admission_group_from_counts(
                evidence.admitted_members,
                mempool.transaction_count as u64,
                evidence.removed_members,
            ),
        }
    }

    pub fn unbroadcast_member_count(&self) -> u64 {
        self.unbroadcast_members.len() as u64
    }

    #[cfg(test)]
    pub(crate) fn insert_unbroadcast_member_for_test(&mut self, identity: MempoolMemberIdentity) {
        self.unbroadcast_members.insert(identity);
    }
}
