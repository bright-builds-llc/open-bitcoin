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

use crate::{MempoolError, MempoolOutcome, MempoolRejectionCategory};

use super::{Mempool, serialization_validation_error};

pub(super) fn accept(
    mempool: &mut Mempool,
    transaction: Transaction,
    chainstate: &ChainstateSnapshot,
    verify_flags: ScriptVerifyFlags,
    consensus_params: ConsensusParams,
) -> Result<MempoolOutcome, MempoolError> {
    let txid = transaction_txid(&transaction)
        .map_err(|source| serialization_validation_error("transaction txid", source))?;
    let wtxid = transaction_wtxid(&transaction)
        .map_err(|source| serialization_validation_error("transaction wtxid", source))?;
    let missing_parents = missing_parent_txids(mempool, &transaction, chainstate);

    let admission =
        mempool.accept_transaction(transaction, chainstate, verify_flags, consensus_params);
    let outcome = match admission {
        Ok(result) if result.replaced.is_empty() => MempoolOutcome::Accepted {
            txid: result.accepted,
            wtxid,
            evicted: result.evicted,
        },
        Ok(result) => MempoolOutcome::Replaced {
            txid: result.accepted,
            wtxid,
            replaced: result.replaced,
            evicted: result.evicted,
        },
        Err(MempoolError::DuplicateTransaction { txid }) => MempoolOutcome::Duplicate { txid },
        Err(MempoolError::MissingInput { .. }) => MempoolOutcome::Orphaned {
            txid,
            wtxid,
            missing_parents,
        },
        Err(MempoolError::CandidateEvicted { txid }) => MempoolOutcome::Evicted { txid, wtxid },
        Err(error) => rejected_outcome(txid, wtxid, &error),
    };
    Ok(outcome)
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
