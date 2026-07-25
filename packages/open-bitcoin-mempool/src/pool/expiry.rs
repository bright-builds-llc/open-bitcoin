// Parity breadcrumbs:
// - packages/bitcoin-knots/doc/policy/packages.md
// - packages/bitcoin-knots/src/kernel/mempool_options.h
// - packages/bitcoin-knots/src/kernel/mempool_removal_reason.h
// - packages/bitcoin-knots/src/policy/packages.cpp
// - packages/bitcoin-knots/src/policy/policy.h
// - packages/bitcoin-knots/src/policy/rbf.cpp
// - packages/bitcoin-knots/src/rpc/mempool.cpp
// - packages/bitcoin-knots/src/txmempool.cpp
// - packages/bitcoin-knots/src/txmempool.h

//! Pure PolicyTime-driven mempool expiry (Knots `Expire`).

use std::collections::{BTreeMap, BTreeSet};

use open_bitcoin_primitives::Txid;

use crate::{
    FinalMempoolMembership, MempoolAcceptanceTime, MempoolError, MempoolLifecycleDelta,
    MempoolLifecycleRemoval, MempoolMemberIdentity, MempoolMemberState, MempoolRemovalCause,
    MempoolRemovalRole, MempoolRetryClear, MempoolRetryClearCause, PolicyConfig, PolicyTime,
};

use super::admission::lifecycle_invariant_error;
use super::topology::collect_descendants;
use super::{Mempool, MempoolState, recompute_state, resource_invariant_error};

const SECONDS_PER_HOUR: i64 = 3_600;

impl Mempool {
    /// Removes entries with `Known(accepted_at) < now - expiry` and their descendants.
    ///
    /// Skips `MempoolAcceptanceTime::LegacyUnknown` without inventing times. Pure core never
    /// samples wall-clock time — callers inject `PolicyTime`.
    pub fn expire(&mut self, now: PolicyTime) -> Result<MempoolLifecycleDelta, MempoolError> {
        let state = MempoolState {
            entries: std::mem::take(&mut self.entries),
            spent_outpoints: std::mem::take(&mut self.spent_outpoints),
            resource_ledger: self.resource_ledger,
        };
        let (new_state, removed) = expire_entries(state, &self.config, now)?;
        self.entries = new_state.entries;
        self.spent_outpoints = new_state.spent_outpoints;
        self.resource_ledger = new_state.resource_ledger;

        let mut delta_builder = MempoolLifecycleDelta::builder();
        for (member, role) in removed {
            delta_builder
                .record_removal(MempoolLifecycleRemoval {
                    member,
                    cause: MempoolRemovalCause::Expiry,
                    role,
                })
                .map_err(lifecycle_invariant_error)?;
            delta_builder
                .record_final_membership(MempoolMemberState {
                    member,
                    membership: FinalMempoolMembership::Absent,
                })
                .map_err(lifecycle_invariant_error)?;
            delta_builder
                .record_retry_clear(MempoolRetryClear {
                    member,
                    cause: MempoolRetryClearCause::LifecycleRemoval,
                })
                .map_err(lifecycle_invariant_error)?;
        }
        delta_builder.build().map_err(lifecycle_invariant_error)
    }
}

fn expire_entries(
    mut state: MempoolState,
    config: &PolicyConfig,
    now: PolicyTime,
) -> Result<
    (
        MempoolState,
        BTreeMap<MempoolMemberIdentity, MempoolRemovalRole>,
    ),
    MempoolError,
> {
    // u64::MAX cannot convert to i64; clamp to i64::MAX so cutoff math stays defined.
    let expiry_hours = i64::try_from(config.mempool_expiry_hours).unwrap_or(i64::MAX);
    let expiry_seconds = expiry_hours.saturating_mul(SECONDS_PER_HOUR);
    let cutoff = now.unix_seconds().saturating_sub(expiry_seconds);

    let aged_roots = state
        .entries
        .iter()
        .filter_map(|(txid, entry)| match entry.metadata.accepted_at {
            MempoolAcceptanceTime::Known(accepted_at) if accepted_at.unix_seconds() < cutoff => {
                Some(*txid)
            }
            MempoolAcceptanceTime::Known(_) | MempoolAcceptanceTime::LegacyUnknown => None,
        })
        .collect::<BTreeSet<Txid>>();

    if aged_roots.is_empty() {
        return Ok((state, BTreeMap::new()));
    }

    let mut remove_set = aged_roots.clone();
    for root_txid in &aged_roots {
        remove_set.extend(collect_descendants(&state.entries, *root_txid));
    }

    let mut removed = BTreeMap::new();
    let removed_members = state
        .entries
        .iter()
        .filter(|(txid, _entry)| remove_set.contains(txid))
        .map(|(txid, entry)| MempoolMemberIdentity {
            txid: *txid,
            wtxid: entry.wtxid,
        })
        .collect::<Vec<_>>();
    for member in removed_members {
        state.entries.remove(&member.txid);
        let role = if aged_roots.contains(&member.txid) {
            MempoolRemovalRole::Direct
        } else {
            MempoolRemovalRole::Descendant
        };
        removed.insert(member, role);
    }
    state = recompute_state(state.entries).map_err(resource_invariant_error)?;

    Ok((state, removed))
}

#[cfg(test)]
mod default_constant_tests {
    use crate::DEFAULT_MEMPOOL_EXPIRY_HOURS;

    #[test]
    fn default_mempool_expiry_hours_matches_knots() {
        assert_eq!(DEFAULT_MEMPOOL_EXPIRY_HOURS, 336);
    }
}
