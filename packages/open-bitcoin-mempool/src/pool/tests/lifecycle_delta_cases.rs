// Parity breadcrumbs:
// - packages/bitcoin-knots/src/kernel/mempool_removal_reason.h
// - packages/bitcoin-knots/src/txmempool.h
// - packages/bitcoin-knots/src/txmempool.cpp
// - packages/bitcoin-knots/src/validation.cpp

use open_bitcoin_consensus::{
    ConsensusParams, ScriptVerifyFlags, transaction_txid, transaction_wtxid,
};
use open_bitcoin_primitives::{BlockHash, Transaction, TransactionInput, Txid, Wtxid};

use crate::{
    AdmissionContext, BlockLifecycleContext, FinalMempoolMembership, Mempool, MempoolCapacity,
    MempoolEntryMetadata, MempoolError, MempoolLifecycleDelta, MempoolLifecycleInvariantError,
    MempoolLifecycleRemoval, MempoolMemberIdentity, MempoolMemberState, MempoolOrigin,
    MempoolOutcome, MempoolRemovalCause, MempoolRemovalRole, MempoolRetryClear,
    MempoolRetryClearCause, PolicyConfig, PolicyTime, RelayIntent, RollingMempoolFeeRate,
};

use super::{
    build_block, non_standard_spend, sample_chainstate_snapshot, spend_transaction, submit,
};

fn identity(value: u8) -> MempoolMemberIdentity {
    MempoolMemberIdentity {
        txid: Txid::from_byte_array([value; 32]),
        wtxid: Wtxid::from_byte_array([value.saturating_add(32); 32]),
    }
}

fn submit_transition(
    mempool: &mut Mempool,
    snapshot: &open_bitcoin_chainstate::ChainstateSnapshot,
    transaction: Transaction,
    context: AdmissionContext,
) -> crate::MempoolTransition {
    mempool
        .accept_transaction_transition_with_context(
            transaction,
            snapshot,
            ScriptVerifyFlags::P2SH
                | ScriptVerifyFlags::CHECKLOCKTIMEVERIFY
                | ScriptVerifyFlags::CHECKSEQUENCEVERIFY,
            ConsensusParams {
                coinbase_maturity: 1,
                ..ConsensusParams::default()
            },
            context,
        )
        .expect("transition outcome")
}

fn build_retry_delta(
    first: MempoolRetryClearCause,
    second: MempoolRetryClearCause,
) -> MempoolLifecycleDelta {
    let member = identity(1);
    let mut builder = MempoolLifecycleDelta::builder();
    builder
        .record_final_membership(MempoolMemberState {
            member,
            membership: FinalMempoolMembership::Present,
        })
        .expect("consistent final membership");
    builder
        .record_retry_clear(MempoolRetryClear {
            member,
            cause: first,
        })
        .expect("consistent first retry clear");
    builder
        .record_retry_clear(MempoolRetryClear {
            member,
            cause: second,
        })
        .expect("consistent second retry clear");
    builder.build().expect("complete lifecycle delta")
}

mod lifecycle_delta_labels_are_fixed_and_complete;
mod replacement_transition_preserves_direct_and_descendant_roles;
