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

//! Accounted-capacity pressure trim and descendant-package rolling bumps.

#[cfg(test)]
use std::collections::HashMap;

use open_bitcoin_primitives::Txid;

use crate::{
    FeeRate, MempoolEntry, MempoolError, MempoolRemovalCause, MempoolRemovalRole, PolicyConfig,
};

use super::lifecycle::MempoolRemovalFact;
use super::prospective::ProspectiveMempool;

pub(super) fn trim_prospective_to_capacity(
    prospective: &mut ProspectiveMempool<'_>,
    config: &PolicyConfig,
) -> Result<Vec<MempoolRemovalFact>, MempoolError> {
    trim_to_size(prospective, config)
}

fn trim_to_size(
    prospective: &mut ProspectiveMempool<'_>,
    config: &PolicyConfig,
) -> Result<Vec<MempoolRemovalFact>, MempoolError> {
    let mut working = prospective.clone();
    working.record_trim_invocation();
    let mut evicted = Vec::new();

    while working.accounted_memory().as_usize() > config.mempool_capacity.as_usize() {
        let Some((victim_txid, package_feerate)) = select_eviction_package(&working) else {
            return Err(MempoolError::InternalInvariant {
                reason: "over-capacity prospective mempool has no eviction victim".to_string(),
            });
        };
        let package_plus_incremental = FeeRate::from_sats_per_kvb(
            package_feerate
                .sats_per_kvb()
                .saturating_add(config.incremental_relay_fee_rate.fee_rate().sats_per_kvb()),
        );
        working
            .rolling_fee_state_mut()
            .track_package_removed(package_plus_incremental);
        let removed_members =
            working.stage_descendant_package_removal(victim_txid, MempoolRemovalCause::Pressure)?;
        for member in removed_members {
            evicted.push(MempoolRemovalFact {
                cause: MempoolRemovalCause::Pressure,
                role: if member.txid == victim_txid {
                    MempoolRemovalRole::Direct
                } else {
                    MempoolRemovalRole::Descendant
                },
            });
        }
    }

    *prospective = working;
    Ok(evicted)
}

#[cfg(test)]
pub(super) fn select_eviction_candidate(entries: &HashMap<Txid, MempoolEntry>) -> Option<Txid> {
    select_entry(entries.iter().map(|(txid, entry)| (*txid, entry)))
        .map(|(txid, _package_feerate)| txid)
}

fn select_eviction_package(prospective: &ProspectiveMempool<'_>) -> Option<(Txid, FeeRate)> {
    select_entry(
        prospective
            .visible_txids()
            .into_iter()
            .filter_map(|txid| prospective.maybe_entry(&txid).map(|entry| (txid, entry))),
    )
}

fn select_entry<'entry>(
    entries: impl Iterator<Item = (Txid, &'entry MempoolEntry)>,
) -> Option<(Txid, FeeRate)> {
    entries
        .min_by(|(left_txid, left_entry), (right_txid, right_entry)| {
            left_entry
                .descendant_score()
                .cmp(&right_entry.descendant_score())
                .then_with(|| left_txid.cmp(right_txid))
        })
        .map(|(txid, entry)| {
            (
                txid,
                FeeRate::from_fee_sats_and_vbytes(
                    entry.descendant_stats.total_fee_sats,
                    entry.descendant_stats.virtual_size,
                ),
            )
        })
}

#[cfg(test)]
#[path = "tests/pressure_internal_cases.rs"]
mod tests;
