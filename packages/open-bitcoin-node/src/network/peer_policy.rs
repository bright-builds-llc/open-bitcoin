// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/banman.cpp
// - packages/bitcoin-knots/src/net_permissions.cpp

use std::net::IpAddr;

use open_bitcoin_network::{
    BanDecision, BanScope, MisbehaviorDecision, PeerBanEntry, ReconnectSuppressionInput,
    UnbanDecision,
};

use crate::ChainstateStore;

use super::{ManagedPeerNetwork, inbound::ManagedPeerPolicyInfo};

impl<S: ChainstateStore> ManagedPeerNetwork<S> {
    pub fn peer_policy_info(&self) -> ManagedPeerPolicyInfo {
        let eviction_candidate_count = self.peer_manager.eviction_candidate_inputs().len();
        let peer_policy_runtime_state = self.peer_manager.peer_policy_runtime_state();
        ManagedPeerPolicyInfo::from_policy_decisions(
            eviction_candidate_count,
            Some(self.peer_manager.eviction_decision()),
            peer_policy_runtime_state.misbehavior_decisions(),
            peer_policy_runtime_state.ban_decisions(),
            peer_policy_runtime_state.unban_decisions(),
        )
    }

    pub fn record_peer_policy_ban(
        &mut self,
        entry: PeerBanEntry,
        now_unix_seconds: i64,
    ) -> BanDecision {
        self.peer_manager
            .peer_policy_runtime_state_mut()
            .record_ban(entry, now_unix_seconds)
    }

    pub fn record_peer_policy_unban(
        &mut self,
        scope: &BanScope,
        now_unix_seconds: i64,
    ) -> UnbanDecision {
        self.peer_manager
            .peer_policy_runtime_state_mut()
            .record_unban(scope, now_unix_seconds)
    }

    pub fn record_peer_policy_discouragement(
        &mut self,
        entry: PeerBanEntry,
        now_unix_seconds: i64,
    ) -> BanDecision {
        self.peer_manager
            .peer_policy_runtime_state_mut()
            .record_discouragement(entry, now_unix_seconds)
    }

    pub fn record_peer_policy_misbehavior(&mut self, decision: MisbehaviorDecision) {
        self.peer_manager
            .peer_policy_runtime_state_mut()
            .record_misbehavior(decision);
    }

    pub fn reconnect_suppression_input_for_ip(
        &self,
        remote_ip: IpAddr,
        now_unix_seconds: i64,
    ) -> ReconnectSuppressionInput {
        self.peer_manager
            .peer_policy_runtime_state()
            .reconnect_suppression_input_for_ip(remote_ip, now_unix_seconds)
    }
}
