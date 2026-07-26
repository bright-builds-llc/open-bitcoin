// Parity breadcrumbs:
// - packages/bitcoin-knots/src/kernel/mempool_removal_reason.h
// - packages/bitcoin-knots/src/txmempool.h
// - packages/bitcoin-knots/src/txmempool.cpp
// - packages/bitcoin-knots/src/validation.cpp

use std::collections::{BTreeSet, HashMap};

use open_bitcoin_primitives::Txid;

use crate::MempoolEntry;

pub(super) fn collect_conflicts_and_descendants(
    entries: &HashMap<Txid, MempoolEntry>,
    direct_conflicts: &BTreeSet<Txid>,
) -> BTreeSet<Txid> {
    let mut txids = BTreeSet::new();
    for txid in direct_conflicts {
        txids.insert(*txid);
        txids.extend(collect_descendants(entries, *txid));
    }

    txids
}

#[cfg(test)]
pub(super) fn collect_ancestors(
    entries: &HashMap<Txid, MempoolEntry>,
    txid: Txid,
) -> BTreeSet<Txid> {
    let mut visited = BTreeSet::new();
    let Some(entry) = entries.get(&txid) else {
        return visited;
    };
    let mut stack = entry.parents.iter().copied().collect::<Vec<_>>();
    while let Some(next_txid) = stack.pop() {
        if !visited.insert(next_txid) {
            continue;
        }
        if let Some(next_entry) = entries.get(&next_txid) {
            stack.extend(next_entry.parents.iter().copied());
        }
    }

    visited
}

pub(super) fn collect_descendants(
    entries: &HashMap<Txid, MempoolEntry>,
    txid: Txid,
) -> BTreeSet<Txid> {
    let mut visited = BTreeSet::new();
    let Some(entry) = entries.get(&txid) else {
        return visited;
    };
    let mut stack = entry.children.iter().copied().collect::<Vec<_>>();
    while let Some(next_txid) = stack.pop() {
        if !visited.insert(next_txid) {
            continue;
        }
        if let Some(next_entry) = entries.get(&next_txid) {
            stack.extend(next_entry.children.iter().copied());
        }
    }

    visited
}
