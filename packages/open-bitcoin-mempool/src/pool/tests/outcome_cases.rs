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

use open_bitcoin_consensus::{ConsensusParams, ScriptVerifyFlags};
use open_bitcoin_primitives::{
    Amount, OutPoint, ScriptWitness, Transaction, TransactionInput, Txid, Wtxid,
};

use super::{non_standard_spend, sample_chainstate_snapshot, script, spend_transaction, submit};
use crate::{
    LimitDirection, LimitKind, Mempool, MempoolCapacity, MempoolError, MempoolOutcome,
    MempoolOutcomeLabel, MempoolRejectionCategory, PolicyConfig, RbfPolicy,
};

#[derive(Debug, Clone, PartialEq, Eq)]
struct MempoolAdmissionSnapshot {
    accepted_txids: Vec<Txid>,
    parents: Vec<(Txid, Vec<Txid>)>,
    children: Vec<(Txid, Vec<Txid>)>,
    spent_outpoints: Vec<(OutPoint, Txid)>,
    total_virtual_size: usize,
}

impl MempoolAdmissionSnapshot {
    fn capture(mempool: &Mempool) -> Self {
        let mut accepted_txids = mempool.entries().keys().copied().collect::<Vec<_>>();
        accepted_txids.sort();

        let mut parents = mempool
            .entries()
            .iter()
            .map(|(txid, entry)| (*txid, entry.parents.iter().copied().collect::<Vec<_>>()))
            .collect::<Vec<_>>();
        parents.sort_by(|(left_txid, _), (right_txid, _)| left_txid.cmp(right_txid));

        let mut children = mempool
            .entries()
            .iter()
            .map(|(txid, entry)| (*txid, entry.children.iter().copied().collect::<Vec<_>>()))
            .collect::<Vec<_>>();
        children.sort_by(|(left_txid, _), (right_txid, _)| left_txid.cmp(right_txid));

        let mut spent_outpoints = mempool
            .spent_outpoints
            .iter()
            .map(|(outpoint, spender_txid)| (outpoint.clone(), *spender_txid))
            .collect::<Vec<_>>();
        spent_outpoints.sort_by(
            |(left_outpoint, left_spender), (right_outpoint, right_spender)| {
                left_outpoint
                    .txid
                    .cmp(&right_outpoint.txid)
                    .then_with(|| left_outpoint.vout.cmp(&right_outpoint.vout))
                    .then_with(|| left_spender.cmp(right_spender))
            },
        );

        Self {
            accepted_txids,
            parents,
            children,
            spent_outpoints,
            total_virtual_size: mempool.total_virtual_size().as_usize(),
        }
    }
}

fn submit_outcome(
    mempool: &mut Mempool,
    snapshot: &open_bitcoin_chainstate::ChainstateSnapshot,
    transaction: Transaction,
) -> MempoolOutcome {
    mempool
        .accept_transaction_outcome_with_context(
            transaction,
            snapshot,
            ScriptVerifyFlags::P2SH
                | ScriptVerifyFlags::CHECKLOCKTIMEVERIFY
                | ScriptVerifyFlags::CHECKSEQUENCEVERIFY,
            ConsensusParams {
                coinbase_maturity: 1,
                ..ConsensusParams::default()
            },
            crate::AdmissionContext::legacy_unknown(),
        )
        .expect("outcome")
}

#[allow(dead_code)]
fn amount(value: i64) -> Amount {
    Amount::from_sats(value).expect("valid amount")
}

mod missing_parent_outcome_collects_unique_parent_txids;
mod outcome_labels_are_fixed_low_cardinality_values;
