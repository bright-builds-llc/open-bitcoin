// Parity breadcrumbs:
// - packages/bitcoin-knots/src/txmempool.h
// - packages/bitcoin-knots/src/txmempool.cpp
// - packages/bitcoin-knots/src/policy/policy.h
// - packages/bitcoin-knots/src/policy/rbf.cpp
// - packages/bitcoin-knots/src/policy/packages.cpp

use open_bitcoin_chainstate::ChainstateSnapshot;
use open_bitcoin_consensus::{
    ConsensusParams, ScriptVerifyFlags, transaction_txid, transaction_wtxid,
};
use open_bitcoin_primitives::{Transaction, TransactionInput, Txid, Wtxid};

use crate::{
    AdmissionContext, MempoolError, MempoolLifecycleDelta, MempoolOutcome,
    MempoolRejectionCategory, MempoolTransition,
};

use super::{Mempool, serialization_validation_error};

pub(super) fn accept(
    mempool: &mut Mempool,
    transaction: Transaction,
    chainstate: &ChainstateSnapshot,
    verify_flags: ScriptVerifyFlags,
    consensus_params: ConsensusParams,
    context: AdmissionContext,
) -> Result<MempoolTransition, MempoolError> {
    let txid = transaction_txid(&transaction)
        .map_err(|source| serialization_validation_error("transaction txid", source))?;
    let wtxid = transaction_wtxid(&transaction)
        .map_err(|source| serialization_validation_error("transaction wtxid", source))?;
    let missing_parents = missing_parent_txids(mempool, &transaction, chainstate);

    let admission = mempool.commit_transaction_with_context(
        transaction,
        chainstate,
        verify_flags,
        consensus_params,
        context,
    );
    let transition = match admission {
        Ok(committed) if committed.result.replaced.is_empty() => MempoolTransition {
            outcome: MempoolOutcome::Accepted {
                txid: committed.result.accepted,
                wtxid,
                evicted: committed.result.evicted,
            },
            delta: committed.delta,
        },
        Ok(committed) => MempoolTransition {
            outcome: MempoolOutcome::Replaced {
                txid: committed.result.accepted,
                wtxid,
                replaced: committed.result.replaced,
                evicted: committed.result.evicted,
            },
            delta: committed.delta,
        },
        Err(MempoolError::DuplicateTransaction { txid }) => MempoolTransition {
            outcome: MempoolOutcome::Duplicate { txid },
            delta: MempoolLifecycleDelta::empty(),
        },
        Err(MempoolError::MissingInput { .. }) => MempoolTransition {
            outcome: MempoolOutcome::Orphaned {
                txid,
                wtxid,
                missing_parents,
            },
            delta: MempoolLifecycleDelta::empty(),
        },
        Err(MempoolError::CandidateEvicted { txid }) => MempoolTransition {
            outcome: MempoolOutcome::Evicted { txid, wtxid },
            delta: MempoolLifecycleDelta::empty(),
        },
        Err(error) => MempoolTransition {
            outcome: rejected_outcome(txid, wtxid, &error),
            delta: MempoolLifecycleDelta::empty(),
        },
    };
    Ok(transition)
}
fn missing_parent_txids(
    mempool: &Mempool,
    transaction: &Transaction,
    chainstate: &ChainstateSnapshot,
) -> Vec<Txid> {
    transaction
        .inputs
        .iter()
        .filter_map(|input| missing_parent_txid(mempool, input, chainstate))
        .fold(Vec::new(), push_unique_txid)
}
fn missing_parent_txid(
    mempool: &Mempool,
    input: &TransactionInput,
    chainstate: &ChainstateSnapshot,
) -> Option<Txid> {
    let parent_txid = input.previous_output.txid;
    let parent_in_mempool = mempool.entries.get(&parent_txid).is_some_and(|entry| {
        (input.previous_output.vout as usize) < entry.transaction.outputs.len()
    });
    let parent_available =
        chainstate.utxos.contains_key(&input.previous_output) || parent_in_mempool;
    (!parent_available).then_some(parent_txid)
}
fn push_unique_txid(mut txids: Vec<Txid>, txid: Txid) -> Vec<Txid> {
    if !txids.contains(&txid) {
        txids.push(txid);
    }
    txids
}
fn rejected_outcome(txid: Txid, wtxid: Wtxid, error: &MempoolError) -> MempoolOutcome {
    let category = MempoolRejectionCategory::from_error(error)
        .unwrap_or(MempoolRejectionCategory::InternalInvariant);
    MempoolOutcome::Rejected {
        txid,
        wtxid,
        category,
    }
}
