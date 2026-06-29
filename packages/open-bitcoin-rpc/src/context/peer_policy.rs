// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net.cpp
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/banman.cpp
// - packages/bitcoin-knots/src/net_permissions.cpp

use open_bitcoin_network::{
    BanDecision, BanScope, MisbehaviorDecision, PeerBanEntry, UnbanDecision,
};
use open_bitcoin_node::{
    logging::{
        StructuredLogError, inbound_peer_policy_log_record, writer::append_structured_log_record,
    },
    status::{FieldAvailability, InboundPeerPolicyEvent},
};

use super::{ManagedRpcContext, resource_governance::current_unix_seconds};

impl ManagedRpcContext {
    pub fn record_inbound_peer_policy_event(&mut self, event: InboundPeerPolicyEvent) {
        if self
            .record_inbound_peer_policy_event_at(event, current_unix_seconds())
            .is_err()
        {
            self.resource_governance_log_write_failures = self
                .resource_governance_log_write_failures
                .saturating_add(1);
        }
    }

    pub fn record_inbound_peer_policy_event_at(
        &mut self,
        event: InboundPeerPolicyEvent,
        timestamp_unix_seconds: u64,
    ) -> Result<(), StructuredLogError> {
        let Some(log_dir) = &self.maybe_resource_governance_log_dir else {
            return Ok(());
        };
        let record = inbound_peer_policy_log_record(&event, timestamp_unix_seconds);
        append_structured_log_record(log_dir, &record, self.resource_governance_log_retention)?;
        Ok(())
    }

    pub fn record_latest_inbound_peer_policy_event_at(
        &mut self,
        timestamp_unix_seconds: u64,
    ) -> Result<bool, StructuredLogError> {
        let FieldAvailability::Available(inbound) = self.current_inbound_status() else {
            return Ok(false);
        };
        let FieldAvailability::Available(event) = inbound.latest_peer_policy_decision else {
            return Ok(false);
        };
        self.record_inbound_peer_policy_event_at(event, timestamp_unix_seconds)?;
        Ok(true)
    }

    pub fn record_peer_policy_ban(
        &mut self,
        entry: PeerBanEntry,
        now_unix_seconds: i64,
    ) -> BanDecision {
        let decision = self.network.record_peer_policy_ban(entry, now_unix_seconds);
        self.record_inbound_peer_policy_event(peer_policy_event_from_ban_decision(&decision));
        decision
    }

    pub fn record_peer_policy_discouragement(
        &mut self,
        entry: PeerBanEntry,
        now_unix_seconds: i64,
    ) -> BanDecision {
        let decision = self
            .network
            .record_peer_policy_discouragement(entry, now_unix_seconds);
        self.record_inbound_peer_policy_event(peer_policy_event_from_discouragement_decision(
            &decision,
        ));
        decision
    }

    pub fn record_peer_policy_unban(
        &mut self,
        scope: &BanScope,
        now_unix_seconds: i64,
    ) -> UnbanDecision {
        let decision = self
            .network
            .record_peer_policy_unban(scope, now_unix_seconds);
        self.record_inbound_peer_policy_event(peer_policy_event_from_unban_decision(&decision));
        decision
    }

    pub fn record_peer_policy_misbehavior(&mut self, decision: MisbehaviorDecision) {
        self.record_inbound_peer_policy_event(peer_policy_event_from_misbehavior_decision(
            &decision,
        ));
        self.network.record_peer_policy_misbehavior(decision);
    }
}

fn peer_policy_event_from_ban_decision(decision: &BanDecision) -> InboundPeerPolicyEvent {
    let (reason, source) = match decision {
        BanDecision::Active(entry) | BanDecision::Expired(entry) => {
            (entry.reason.as_str(), peer_policy_source(entry.source))
        }
    };
    InboundPeerPolicyEvent {
        outcome: decision.outcome_label().to_string(),
        reason: reason.to_string(),
        label: decision.outcome_label().to_string(),
        source: source.to_string(),
        message: format!("ban policy decision {}: {reason}", decision.outcome_label()),
    }
}

fn peer_policy_event_from_discouragement_decision(
    decision: &BanDecision,
) -> InboundPeerPolicyEvent {
    let (outcome, reason, source) = match decision {
        BanDecision::Active(entry) => (
            "discouragement_active",
            entry.reason.as_str(),
            peer_policy_source(entry.source),
        ),
        BanDecision::Expired(entry) => (
            "discouragement_expired",
            entry.reason.as_str(),
            peer_policy_source(entry.source),
        ),
    };
    InboundPeerPolicyEvent {
        outcome: outcome.to_string(),
        reason: reason.to_string(),
        label: outcome.to_string(),
        source: source.to_string(),
        message: format!("discouragement policy decision {outcome}: {reason}"),
    }
}

fn peer_policy_event_from_unban_decision(decision: &UnbanDecision) -> InboundPeerPolicyEvent {
    let reason = unban_decision_reason(decision);
    InboundPeerPolicyEvent {
        outcome: decision.outcome_label().to_string(),
        reason: reason.to_string(),
        label: decision.outcome_label().to_string(),
        source: "source_unban_policy".to_string(),
        message: format!(
            "unban policy decision {}: {reason}",
            decision.outcome_label()
        ),
    }
}

fn peer_policy_event_from_misbehavior_decision(
    decision: &MisbehaviorDecision,
) -> InboundPeerPolicyEvent {
    InboundPeerPolicyEvent {
        outcome: decision.response.as_str().to_string(),
        reason: decision.kind.as_str().to_string(),
        label: "misbehavior_policy_decision".to_string(),
        source: "source_misbehavior_policy".to_string(),
        message: format!(
            "misbehavior policy decision {}: {}",
            decision.response.as_str(),
            decision.kind.as_str()
        ),
    }
}

fn peer_policy_source(source: &str) -> &'static str {
    match source {
        "misbehavior_policy" => "source_misbehavior_policy",
        "manual" | "manual_ban" => "source_manual_ban",
        "peer_policy_runtime_bridge" => "source_peer_policy_runtime_bridge",
        _ => "source_ban_policy",
    }
}

fn unban_decision_reason(decision: &UnbanDecision) -> &'static str {
    match decision {
        UnbanDecision::Unbanned(_) => "manual_unban",
        UnbanDecision::NotFound(_) => "unban_not_found",
        UnbanDecision::AlreadyExpired(_) => "ban_already_expired",
    }
}
