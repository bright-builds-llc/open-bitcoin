// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/node/txdownloadman_impl.cpp
// - packages/bitcoin-knots/src/node/txdownloadman.h
// - packages/bitcoin-knots/src/protocol.h
// - packages/bitcoin-knots/src/txorphanage.cpp
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/test/functional/p2p_orphan_handling.py
// - packages/bitcoin-knots/test/functional/mempool_accept.py

use std::collections::BTreeMap;

use open_bitcoin_core::{
    consensus::{transaction_txid, transaction_wtxid},
    primitives::{InventoryVector, Transaction, Txid, Wtxid},
};
use open_bitcoin_mempool::PolicyConfig;
use open_bitcoin_network::{
    ConnectionRole, InboundAdmissionPolicy, LocalPeerConfig, OrphanPolicy, PeerConnectionClass,
    PeerId, PeerManager, RelayActivationConfig, RelayEligibilityDecision, RelayEligibilityInput,
    TxOrphanage, TxRelayId, TxRelayPeerMode, TxServeOutcomeLabel, TxServingRecordStatus,
    classify_relay_eligibility, classify_tx_serve_request,
};

use super::{
    ManagedInboundAdmissionInfo, ManagedNetworkError, ManagedPeerNetwork,
    ManagedResourceGovernanceInfo,
};
use crate::{ChainstateStore, ManagedChainstate, ManagedMempool};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct RelayServingRecord {
    txid: Txid,
    wtxid: Wtxid,
    transaction: Transaction,
    status: TxServingRecordStatus,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct ManagedTxServeDecision {
    pub label: TxServeOutcomeLabel,
    pub maybe_transaction: Option<Transaction>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManagedTxServeOutcome {
    pub label: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ManagedRelayServingInfo {
    pub serveable_transactions: usize,
    pub known_status_transactions: usize,
    pub latest_outcomes: Vec<ManagedTxServeOutcome>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(super) struct RelayServingCache {
    records_by_txid: BTreeMap<Txid, RelayServingRecord>,
    txid_by_wtxid: BTreeMap<Wtxid, Txid>,
    status_by_txid: BTreeMap<Txid, TxServingRecordStatus>,
    status_by_wtxid: BTreeMap<Wtxid, TxServingRecordStatus>,
    latest_outcomes: Vec<ManagedTxServeOutcome>,
}

impl RelayServingCache {
    pub(super) fn record_accepted(
        &mut self,
        transaction: Transaction,
    ) -> Result<(Txid, Wtxid), ManagedNetworkError> {
        let txid = transaction_txid(&transaction)?;
        let wtxid = transaction_wtxid(&transaction)?;
        self.txid_by_wtxid.insert(wtxid, txid);
        self.status_by_txid
            .insert(txid, TxServingRecordStatus::Accepted);
        self.status_by_wtxid
            .insert(wtxid, TxServingRecordStatus::Accepted);
        self.records_by_txid.insert(
            txid,
            RelayServingRecord {
                txid,
                wtxid,
                transaction,
                status: TxServingRecordStatus::Accepted,
            },
        );
        Ok((txid, wtxid))
    }

    pub(super) fn record_replaced(
        &mut self,
        transaction: Transaction,
        replaced: &[Txid],
    ) -> Result<(Txid, Wtxid), ManagedNetworkError> {
        self.remove_transactions(replaced, TxServingRecordStatus::Replaced)?;
        self.record_accepted(transaction)
    }

    pub(super) fn record_status(
        &mut self,
        txid: Txid,
        maybe_wtxid: Option<Wtxid>,
        status: TxServingRecordStatus,
    ) {
        let maybe_record = self.records_by_txid.remove(&txid);
        let maybe_record_wtxid = maybe_record.as_ref().map(|record| record.wtxid);
        let maybe_status_wtxid = maybe_wtxid.or(maybe_record_wtxid);

        if let Some(wtxid) = maybe_record_wtxid {
            self.txid_by_wtxid.remove(&wtxid);
        }
        self.status_by_txid.insert(txid, status);
        if let Some(wtxid) = maybe_status_wtxid {
            self.status_by_wtxid.insert(wtxid, status);
        }
    }

    pub(super) fn remove_transactions(
        &mut self,
        txids: &[Txid],
        status: TxServingRecordStatus,
    ) -> Result<(), ManagedNetworkError> {
        for txid in txids {
            let maybe_wtxid = self
                .records_by_txid
                .get(txid)
                .map(|record| record.wtxid)
                .or_else(|| self.status_wtxid_for_txid(*txid));
            self.record_status(*txid, maybe_wtxid, status);
        }
        Ok(())
    }

    pub(super) fn clear_latest_outcomes(&mut self) {
        self.latest_outcomes.clear();
    }

    pub(super) fn classify_request(
        &mut self,
        request: &InventoryVector,
        peer_mode: TxRelayPeerMode,
        relay_eligibility: &RelayEligibilityDecision,
    ) -> ManagedTxServeDecision {
        let maybe_status = self.status_for_request(request);
        let decision =
            classify_tx_serve_request(request, peer_mode, relay_eligibility, maybe_status);
        let maybe_transaction = match decision.outcome {
            TxServeOutcomeLabel::Served => decision
                .maybe_relay_id
                .and_then(|relay_id| self.transaction_for_relay_id(relay_id)),
            TxServeOutcomeLabel::Unknown
            | TxServeOutcomeLabel::Stale
            | TxServeOutcomeLabel::Confirmed
            | TxServeOutcomeLabel::Rejected
            | TxServeOutcomeLabel::Replaced
            | TxServeOutcomeLabel::Evicted
            | TxServeOutcomeLabel::Expired
            | TxServeOutcomeLabel::IdentityMismatch
            | TxServeOutcomeLabel::NotRelayEligible
            | TxServeOutcomeLabel::NotTransactionInventory => None,
        };
        let label =
            if decision.outcome == TxServeOutcomeLabel::Served && maybe_transaction.is_none() {
                TxServeOutcomeLabel::Stale
            } else {
                decision.outcome
            };
        self.latest_outcomes.push(ManagedTxServeOutcome {
            label: label.as_str(),
        });
        ManagedTxServeDecision {
            label,
            maybe_transaction,
        }
    }

    pub(super) fn info(&self) -> ManagedRelayServingInfo {
        ManagedRelayServingInfo {
            serveable_transactions: self.records_by_txid.len(),
            known_status_transactions: self.status_by_txid.len(),
            latest_outcomes: self.latest_outcomes.clone(),
        }
    }

    fn status_for_request(&self, request: &InventoryVector) -> Option<TxServingRecordStatus> {
        match TxRelayId::from_inventory_vector_for_peer(request, TxRelayPeerMode::TxidOnly) {
            Ok(TxRelayId::Txid(txid)) => self.status_by_txid.get(&txid).copied(),
            Ok(TxRelayId::Wtxid(_)) => None,
            Err(_) => match TxRelayId::from_inventory_vector_for_peer(
                request,
                TxRelayPeerMode::WtxidRelay,
            ) {
                Ok(TxRelayId::Wtxid(wtxid)) => self.status_by_wtxid.get(&wtxid).copied(),
                Ok(TxRelayId::Txid(_)) | Err(_) => None,
            },
        }
    }

    fn transaction_for_relay_id(&self, relay_id: TxRelayId) -> Option<Transaction> {
        match relay_id {
            TxRelayId::Txid(txid) => self
                .records_by_txid
                .get(&txid)
                .filter(|record| record.status == TxServingRecordStatus::Accepted)
                .map(|record| record.transaction.clone()),
            TxRelayId::Wtxid(wtxid) => self
                .txid_by_wtxid
                .get(&wtxid)
                .and_then(|txid| self.records_by_txid.get(txid))
                .filter(|record| record.status == TxServingRecordStatus::Accepted)
                .map(|record| record.transaction.clone()),
        }
    }

    fn status_wtxid_for_txid(&self, txid: Txid) -> Option<Wtxid> {
        self.txid_by_wtxid
            .iter()
            .find_map(|(wtxid, mapped_txid)| (*mapped_txid == txid).then_some(*wtxid))
    }
}

impl<S: ChainstateStore> ManagedPeerNetwork<S> {
    pub fn new(store: S, local_config: LocalPeerConfig, mempool_config: PolicyConfig) -> Self {
        Self::new_with_relay_activation(
            store,
            local_config,
            mempool_config,
            RelayActivationConfig::default(),
            false,
        )
    }

    pub fn new_with_relay_activation(
        store: S,
        local_config: LocalPeerConfig,
        mempool_config: PolicyConfig,
        relay_activation: RelayActivationConfig,
        inbound_serving_enabled: bool,
    ) -> Self {
        Self::from_peer_manager(
            store,
            local_config.clone(),
            mempool_config,
            PeerManager::new(local_config),
            relay_activation,
            inbound_serving_enabled,
        )
    }

    pub fn with_sync_limits(
        store: S,
        local_config: LocalPeerConfig,
        mempool_config: PolicyConfig,
        max_blocks_in_flight_per_peer: usize,
    ) -> Self {
        Self::with_sync_limits_and_relay_activation(
            store,
            local_config,
            mempool_config,
            max_blocks_in_flight_per_peer,
            RelayActivationConfig::default(),
            false,
        )
    }

    pub fn with_sync_limits_and_relay_activation(
        store: S,
        local_config: LocalPeerConfig,
        mempool_config: PolicyConfig,
        max_blocks_in_flight_per_peer: usize,
        relay_activation: RelayActivationConfig,
        inbound_serving_enabled: bool,
    ) -> Self {
        let peer_manager = PeerManager::with_max_blocks_in_flight(
            local_config.clone(),
            max_blocks_in_flight_per_peer,
        );
        Self::from_peer_manager(
            store,
            local_config,
            mempool_config,
            peer_manager,
            relay_activation,
            inbound_serving_enabled,
        )
    }

    pub fn relay_serving_info(&self) -> ManagedRelayServingInfo {
        self.relay_serving.info()
    }

    pub(super) fn relay_serving_context_for_peer(
        &self,
        peer_id: PeerId,
    ) -> (TxRelayPeerMode, RelayEligibilityDecision) {
        let Some(peer) = self.peer_manager.peer_state(peer_id) else {
            return (
                TxRelayPeerMode::TxidOnly,
                classify_relay_eligibility(&RelayEligibilityInput {
                    activation: RelayActivationConfig::default(),
                    inbound_serving_enabled: false,
                    connection_class: PeerConnectionClass::OrdinaryInbound,
                    relay_permission_effects: Vec::new(),
                    inactive_permission_effects: Vec::new(),
                }),
            );
        };
        let peer_mode = TxRelayPeerMode::from_remote_wtxidrelay(peer.remote_wtxidrelay);
        let (connection_class, relay_permission_effects, inactive_permission_effects) =
            match peer.role {
                ConnectionRole::Outbound => (PeerConnectionClass::Outbound, Vec::new(), Vec::new()),
                ConnectionRole::Inbound => {
                    let Some(record) = peer.maybe_inbound_record.as_ref() else {
                        return (
                            peer_mode,
                            classify_relay_eligibility(&RelayEligibilityInput {
                                activation: self.relay_activation,
                                inbound_serving_enabled: self.inbound_serving_enabled,
                                connection_class: PeerConnectionClass::OrdinaryInbound,
                                relay_permission_effects: Vec::new(),
                                inactive_permission_effects: Vec::new(),
                            }),
                        );
                    };
                    (
                        record.connection_class,
                        record
                            .permission_decision
                            .relay_permission_effects()
                            .to_vec(),
                        record.permission_decision.inactive_effects().to_vec(),
                    )
                }
            };

        (
            peer_mode,
            classify_relay_eligibility(&RelayEligibilityInput {
                activation: self.relay_activation,
                inbound_serving_enabled: self.inbound_serving_enabled,
                connection_class,
                relay_permission_effects,
                inactive_permission_effects,
            }),
        )
    }

    fn from_peer_manager(
        store: S,
        local_config: LocalPeerConfig,
        mempool_config: PolicyConfig,
        mut peer_manager: PeerManager,
        relay_activation: RelayActivationConfig,
        inbound_serving_enabled: bool,
    ) -> Self {
        let chainstate = ManagedChainstate::from_store(store);
        peer_manager.seed_local_chain(&chainstate.chainstate().snapshot().active_chain);

        Self {
            chainstate,
            mempool: ManagedMempool::new(mempool_config),
            peer_manager,
            orphanage: TxOrphanage::new(OrphanPolicy::default()),
            known_peers: Default::default(),
            inbound_admission_policy: InboundAdmissionPolicy::new(usize::MAX, 0),
            inbound_admission_info: ManagedInboundAdmissionInfo::default(),
            resource_governance_info: ManagedResourceGovernanceInfo::default(),
            relay_activation,
            inbound_serving_enabled,
            relay_fanout: super::relay_fanout::ManagedRelayFanoutState::default(),
            relay_serving: RelayServingCache::default(),
            local_config,
            blocks_by_hash: BTreeMap::new(),
            transactions_by_txid: BTreeMap::new(),
            transactions_by_wtxid: BTreeMap::new(),
        }
    }
}
