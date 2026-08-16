// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/bitcoind.cpp
// - packages/bitcoin-knots/test/functional/mempool_unbroadcast.py

//! Handle-owned initial-broadcast retry tick and first-hop TX INV drain.
//!
//! This is initial-broadcast retry for the local unbroadcast set; it is not
//! public/default relay and does not guarantee propagation.

use std::fmt;

use open_bitcoin_core::primitives::{InventoryType, Txid, Wtxid};
use open_bitcoin_mempool::MempoolMemberIdentity;
use open_bitcoin_network::{
    MaintenanceInspectBudget, MaintenancePrepareBudget, RetryDecisionContext, WireNetworkMessage,
    next_retry_due_unix_seconds, retry_cycle_is_due, select_maintenance_identities,
};

use super::{
    super::{
        PeerEmission,
        lifecycle_projection::{LifecycleCommand, PeerRelayPreparationRequest},
    },
    AuthoritativeNetwork, LifecycleCommandResult, ManagedNetworkAuthorityError,
    ManagedNetworkHandle, apply_lifecycle_command,
};

/// Outcome of one receive-independent maintenance tick.
#[derive(Debug)]
pub struct MaintenanceTickOutcome {
    pub inspected_count: usize,
    pub prepared_count: usize,
    pub leftover_unattempted_count: usize,
    pub maybe_next_due_unix_seconds: Option<i64>,
    pub emissions: Vec<PeerEmission>,
}

/// Typed maintenance-tick failure that does not walk the unbroadcast set.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MaintenanceTickError {
    DueTimeOverflow,
}

impl MaintenanceTickError {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DueTimeOverflow => "maintenance_due_time_overflow",
        }
    }
}

impl fmt::Display for MaintenanceTickError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl std::error::Error for MaintenanceTickError {}

impl ManagedNetworkHandle {
    /// Issues one process-global initial-broadcast retry tick.
    ///
    /// This is initial-broadcast retry for the local unbroadcast set; it is not
    /// public/default relay and does not guarantee propagation.
    pub fn maintenance_tick(
        &self,
        context: RetryDecisionContext,
    ) -> Result<MaintenanceTickOutcome, ManagedNetworkAuthorityError> {
        self.mutate(|network| apply_maintenance_tick(network, context))?
    }

    /// Drains queued TX INV into owned `PeerEmission` values after lock release.
    pub fn drain_tx_fanout_emissions(
        &self,
        now_unix_seconds: i64,
    ) -> Result<Vec<PeerEmission>, ManagedNetworkAuthorityError> {
        self.mutate(|network| drain_tx_fanout_emissions_locked(network, now_unix_seconds))?
    }

    #[cfg(test)]
    pub(in crate::network) fn insert_unbroadcast_identities_for_test(
        &self,
        identities: impl IntoIterator<Item = MempoolMemberIdentity>,
    ) -> Result<(), ManagedNetworkAuthorityError> {
        self.mutate(|network| {
            for identity in identities {
                network.unbroadcast_members.insert(identity);
                network
                    .relay_fanout
                    .seed_recovered_transaction(identity.txid, identity.wtxid);
            }
        })
    }

    #[cfg(test)]
    pub(in crate::network) fn set_retry_timer_for_test(
        &self,
        maybe_due: Option<i64>,
        maybe_cursor: Option<MempoolMemberIdentity>,
    ) -> Result<(), ManagedNetworkAuthorityError> {
        self.mutate(|network| {
            network.maybe_retry_due_at_unix_seconds = maybe_due;
            network.maybe_unbroadcast_walk_cursor = maybe_cursor;
        })
    }

    #[cfg(test)]
    pub(in crate::network) fn retry_timer_for_test(
        &self,
    ) -> Result<(Option<i64>, Option<MempoolMemberIdentity>), ManagedNetworkAuthorityError> {
        self.read(|network| {
            (
                network.maybe_retry_due_at_unix_seconds,
                network.maybe_unbroadcast_walk_cursor,
            )
        })
    }
}

fn apply_maintenance_tick(
    network: &mut AuthoritativeNetwork,
    context: RetryDecisionContext,
) -> Result<MaintenanceTickOutcome, ManagedNetworkAuthorityError> {
    let Some(next_due) = next_retry_due_unix_seconds(context) else {
        return Err(ManagedNetworkAuthorityError::MaintenanceTick(
            MaintenanceTickError::DueTimeOverflow,
        ));
    };

    if let Some(due_at) = network.maybe_retry_due_at_unix_seconds
        && !retry_cycle_is_due(context.observed_at_unix_seconds, due_at)
    {
        return Ok(MaintenanceTickOutcome {
            inspected_count: 0,
            prepared_count: 0,
            leftover_unattempted_count: 0,
            maybe_next_due_unix_seconds: Some(due_at),
            emissions: Vec::new(),
        });
    }

    let selection = select_maintenance_identities(
        &network.unbroadcast_members,
        network.maybe_unbroadcast_walk_cursor,
        MaintenanceInspectBudget::production(),
        MaintenancePrepareBudget::production(),
    );
    let _actions = network.enqueue_retry_admissions(&selection.prepare);
    // Plan 01's maybe_next_after is the last inspected identity. When the
    // unbroadcast set is smaller than the inspect budget, that cursor wraps to
    // the head and starves leftovers. Store the last prepared identity instead.
    network.maybe_unbroadcast_walk_cursor = selection
        .prepare
        .last()
        .copied()
        .or(selection.maybe_next_after);
    network.maybe_retry_due_at_unix_seconds = Some(next_due);
    let emissions = drain_tx_fanout_emissions_locked(network, context.observed_at_unix_seconds)?;
    Ok(MaintenanceTickOutcome {
        inspected_count: selection.inspected.len(),
        prepared_count: selection.prepare.len(),
        leftover_unattempted_count: selection.leftover_unattempted.len(),
        maybe_next_due_unix_seconds: Some(next_due),
        emissions,
    })
}

fn drain_tx_fanout_emissions_locked(
    network: &mut AuthoritativeNetwork,
    now_unix_seconds: i64,
) -> Result<Vec<PeerEmission>, ManagedNetworkAuthorityError> {
    let drained = network.drain_relay_fanout(now_unix_seconds);
    let mut emissions = Vec::new();
    for (peer_id, message) in drained {
        let WireNetworkMessage::Inv(_) = &message else {
            continue;
        };
        let Some(member) = member_for_inventory(network, &message) else {
            continue;
        };
        let capability = match apply_lifecycle_command(
            network,
            LifecycleCommand::PrepareRelay(PeerRelayPreparationRequest::new(peer_id)),
        ) {
            Ok(LifecycleCommandResult::RelayPrepared(capability)) => capability,
            Ok(_) => continue,
            Err(error) => return Err(ManagedNetworkAuthorityError::from(error)),
        };
        if capability.peer_id() != peer_id {
            apply_lifecycle_command(network, LifecycleCommand::AbortPeerEffect(capability))
                .map_err(ManagedNetworkAuthorityError::from)?;
            continue;
        }
        if let Some(emission) =
            PeerEmission::try_new_tx_inventory(peer_id, message, member, capability)
        {
            emissions.push(emission);
        }
    }
    Ok(emissions)
}

fn member_for_inventory(
    network: &AuthoritativeNetwork,
    message: &WireNetworkMessage,
) -> Option<MempoolMemberIdentity> {
    let WireNetworkMessage::Inv(list) = message else {
        return None;
    };
    let item = list.inventory.first()?;
    match item.inventory_type {
        InventoryType::Transaction => {
            let txid = Txid::from(item.object_hash);
            network
                .unbroadcast_members
                .iter()
                .find(|member| member.txid == txid)
                .copied()
                .or_else(|| {
                    network
                        .relay_fanout
                        .wtxid_for_txid(txid)
                        .map(|wtxid| MempoolMemberIdentity { txid, wtxid })
                })
        }
        InventoryType::WitnessTransaction => {
            let wtxid = Wtxid::from(item.object_hash);
            network
                .unbroadcast_members
                .iter()
                .find(|member| member.wtxid == wtxid)
                .copied()
                .or_else(|| {
                    network
                        .relay_fanout
                        .txid_for_wtxid(wtxid)
                        .map(|txid| MempoolMemberIdentity { txid, wtxid })
                })
        }
        _ => None,
    }
}
