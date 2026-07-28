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

use std::collections::BTreeSet;

use open_bitcoin_primitives::Txid;

use crate::{
    FinalMempoolMembership, MempoolAcceptanceTime, MempoolError, MempoolLifecycleDelta,
    MempoolLifecycleRemoval, MempoolMemberIdentity, MempoolMemberState, MempoolRemovalCause,
    MempoolRemovalRole, MempoolRetryClear, MempoolRetryClearCause, PolicyTime,
};

use super::admission::lifecycle_invariant_error;
use super::patch::prepare_removal_patch;
use super::topology::collect_descendants;
use super::{Mempool, PreparedMempoolTransition};

const SECONDS_PER_HOUR: i64 = 3_600;

impl Mempool {
    /// Prepares removal of entries with `Known(accepted_at) < now - expiry`.
    ///
    /// Skips `MempoolAcceptanceTime::LegacyUnknown` without inventing times. Pure core never
    /// samples wall-clock time — callers inject `PolicyTime`.
    pub fn prepare_expiry(
        &self,
        now: PolicyTime,
    ) -> Result<PreparedMempoolTransition, MempoolError> {
        let expiry_hours = i64::try_from(self.config.mempool_expiry_hours).unwrap_or(i64::MAX);
        let expiry_seconds = expiry_hours.saturating_mul(SECONDS_PER_HOUR);
        let cutoff = now.unix_seconds().saturating_sub(expiry_seconds);
        let aged_roots = self
            .entries
            .iter()
            .filter_map(|(txid, entry)| match entry.metadata.accepted_at {
                MempoolAcceptanceTime::Known(accepted_at)
                    if accepted_at.unix_seconds() < cutoff =>
                {
                    Some(*txid)
                }
                MempoolAcceptanceTime::Known(_) | MempoolAcceptanceTime::LegacyUnknown => None,
            })
            .collect::<BTreeSet<Txid>>();
        if aged_roots.is_empty() {
            return Ok(PreparedMempoolTransition::maintenance_noop(self));
        }

        let mut remove_set = aged_roots.clone();
        for root_txid in &aged_roots {
            remove_set.extend(collect_descendants(&self.entries, *root_txid));
        }

        let mut delta_builder = MempoolLifecycleDelta::builder();
        for (txid, entry) in self
            .entries
            .iter()
            .filter(|(txid, _entry)| remove_set.contains(txid))
        {
            let member = MempoolMemberIdentity {
                txid: *txid,
                wtxid: entry.wtxid,
            };
            let role = if aged_roots.contains(txid) {
                MempoolRemovalRole::Direct
            } else {
                MempoolRemovalRole::Descendant
            };
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
        let delta = delta_builder.build().map_err(lifecycle_invariant_error)?;
        let patch = prepare_removal_patch(self, remove_set, self.rolling_fee_state.clone(), delta)?;
        PreparedMempoolTransition::maintenance_from_patch(self, patch)
    }

    /// Removes expired entries through the prepared transition compatibility facade.
    pub fn expire(&mut self, now: PolicyTime) -> Result<MempoolLifecycleDelta, MempoolError> {
        let prepared = self.prepare_expiry(now)?;
        let validated = self.validate_prepared_mempool_transition(prepared)?;
        Ok(self.apply_validated_mempool_transition(validated))
    }
}

#[cfg(test)]
mod default_constant_tests {
    use crate::DEFAULT_MEMPOOL_EXPIRY_HOURS;

    #[test]
    fn default_mempool_expiry_hours_matches_knots() {
        assert_eq!(DEFAULT_MEMPOOL_EXPIRY_HOURS, 336);
    }
}
