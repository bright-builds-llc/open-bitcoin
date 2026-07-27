// Parity breadcrumbs:
// - packages/bitcoin-knots/src/test/txpackage_tests.cpp
// - packages/bitcoin-knots/src/txmempool.h
// - packages/bitcoin-knots/src/validation.cpp

use open_bitcoin_consensus::{
    ConsensusParams, ScriptVerifyFlags, transaction_txid, transaction_wtxid,
};
use open_bitcoin_primitives::{Amount, Transaction, TransactionInput, Txid};

use super::{non_standard_spend, sample_chainstate_snapshot, script, spend_transaction, submit};
use crate::{
    AdmissionContext, DryRunPackageCommand, FeeRate, HardMemberFailure, Mempool, MempoolEntry,
    MempoolError, PackageMemberResult, PackageReportError, PackageStatus, PolicyConfig,
    ReconsiderableMemberFailure, RollingMempoolFeeRate, StaticRelayFeeRate, SubmissionPackage,
    SubmitPackageCommand, TransactionVirtualSize, TrucPolicy, WellFormedPackage,
};

fn verify_flags() -> ScriptVerifyFlags {
    ScriptVerifyFlags::P2SH
        | ScriptVerifyFlags::CHECKLOCKTIMEVERIFY
        | ScriptVerifyFlags::CHECKSEQUENCEVERIFY
}

fn consensus_params() -> ConsensusParams {
    ConsensusParams {
        coinbase_maturity: 1,
        ..ConsensusParams::default()
    }
}

fn package(transactions: Vec<Transaction>) -> WellFormedPackage {
    WellFormedPackage::try_from(transactions).expect("well-formed package")
}

fn invalid_script_transaction(mut transaction: Transaction) -> Transaction {
    transaction.inputs[0].script_sig = script(&[0x01, 0x52]);
    transaction
}

fn rolling_only_mempool(mut config: PolicyConfig) -> Mempool {
    config.static_relay_fee_rate = StaticRelayFeeRate::new(FeeRate::ZERO);
    let mut mempool = Mempool::new(config);
    mempool
        .set_rolling_mempool_fee_rate(RollingMempoolFeeRate::new(FeeRate::from_sats_per_kvb(
            1_000,
        )))
        .expect("rolling fixture");
    mempool
}

mod missing_input_stays_reconsiderable_after_residual_attempt;
mod reconsiderable_parent_and_child_succeed_as_residual_group;
mod singleton_and_residual_truc_failures_are_typed_and_atomic;
