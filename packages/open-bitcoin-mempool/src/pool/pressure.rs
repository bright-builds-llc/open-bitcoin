// Parity breadcrumbs:
// - packages/bitcoin-knots/src/txmempool.h
// - packages/bitcoin-knots/src/txmempool.cpp
// - packages/bitcoin-knots/src/policy/policy.h
// - packages/bitcoin-knots/src/policy/rbf.cpp
// - packages/bitcoin-knots/src/policy/packages.cpp
// - packages/bitcoin-knots/src/rpc/mempool.cpp
// - packages/bitcoin-knots/doc/policy/packages.md

//! Accounted-capacity pressure trim and descendant-package rolling bumps.

use std::collections::{BTreeMap, HashMap};

use open_bitcoin_primitives::Txid;

use crate::fee::rolling::RollingFeeState;
use crate::{
    FeeRate, MempoolEntry, MempoolError, MempoolMemberIdentity, MempoolRemovalRole, PolicyConfig,
};

use super::topology::collect_descendants;
use super::{MempoolState, recompute_state, resource_invariant_error};

pub(super) fn trim_to_size(
    mut state: MempoolState,
    config: &PolicyConfig,
    rolling: &mut RollingFeeState,
) -> Result<
    (
        MempoolState,
        BTreeMap<MempoolMemberIdentity, MempoolRemovalRole>,
    ),
    MempoolError,
> {
    let mut evicted = BTreeMap::new();

    while state.resource_ledger.accounted_memory().as_usize() > config.mempool_capacity.as_usize() {
        let Some((victim_txid, package_feerate)) = select_eviction_package(&state.entries) else {
            break;
        };
        let package_plus_incremental = FeeRate::from_sats_per_kvb(
            package_feerate
                .sats_per_kvb()
                .saturating_add(config.incremental_relay_fee_rate.fee_rate().sats_per_kvb()),
        );
        rolling.track_package_removed(package_plus_incremental);

        let mut remove_set = collect_descendants(&state.entries, victim_txid);
        remove_set.insert(victim_txid);
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
            let role = if member.txid == victim_txid {
                MempoolRemovalRole::Direct
            } else {
                MempoolRemovalRole::Descendant
            };
            evicted.insert(member, role);
        }
        state = recompute_state(state.entries).map_err(resource_invariant_error)?;
    }

    Ok((state, evicted))
}

#[cfg(test)]
pub(super) fn select_eviction_candidate(entries: &HashMap<Txid, MempoolEntry>) -> Option<Txid> {
    select_eviction_package(entries).map(|(txid, _package_feerate)| txid)
}

fn select_eviction_package(entries: &HashMap<Txid, MempoolEntry>) -> Option<(Txid, FeeRate)> {
    entries
        .iter()
        .min_by(|(left_txid, left_entry), (right_txid, right_entry)| {
            left_entry
                .descendant_score()
                .cmp(&right_entry.descendant_score())
                .then_with(|| left_txid.cmp(right_txid))
        })
        .map(|(txid, entry)| {
            (
                *txid,
                FeeRate::from_fee_sats_and_vbytes(
                    entry.descendant_stats.total_fee_sats,
                    entry.descendant_stats.virtual_size,
                ),
            )
        })
}
