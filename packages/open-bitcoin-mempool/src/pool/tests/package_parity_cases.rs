// Parity breadcrumbs:
// - packages/bitcoin-knots/src/policy/packages.cpp
// - packages/bitcoin-knots/src/validation.h
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/test/txpackage_tests.cpp
// - packages/bitcoin-knots/src/test/txvalidation_tests.cpp
// - packages/bitcoin-knots/test/functional/mempool_package_rbf.py
// - packages/bitcoin-knots/test/functional/mempool_truc.py
// - packages/bitcoin-knots/test/functional/mempool_ephemeral_dust.py

//! Integrated Phase 132 package-policy closure against pinned Knots behavior.

use std::cell::Cell;
use std::collections::{BTreeMap, BTreeSet, HashMap};

use open_bitcoin_chainstate::ChainstateSnapshot;
use open_bitcoin_consensus::{
    ConsensusParams, ScriptVerifyFlags, TransactionInputContext, transaction_txid,
};
use open_bitcoin_primitives::{
    Amount, OutPoint, ScriptBuf, ScriptWitness, Transaction, TransactionInput, TransactionOutput,
    Txid, Wtxid,
};

use super::{sample_chainstate_snapshot, script, spend_transaction, submit};
use crate::policy::replacement::{
    MempoolView, PackageReplacementError, evaluate_limited_package_replacement,
};
use crate::policy::truc::{TrucPolicyError, evaluate_truc_package};
use crate::pool::candidate::{
    PreparedCandidate, fail_missing_parent_report_on_call_for_test, prepare_candidate,
};
use crate::pool::package_admission::{
    PackagePolicyStage, evaluate_package_for_test, package_policy_probe_for_test,
    package_trim_count_for_test, reset_package_trim_count_for_test,
};
use crate::pool::pressure::trim_prospective_to_capacity;
use crate::pool::prospective::ProspectiveMempool;
use crate::{
    AdmissionContext, CandidateFees, DryRunPackageCommand, EffectiveFeeGroup,
    EffectiveFeeGroupError, EffectiveFeeGroupId, EphemeralPolicy, FeeRate, HardMemberFailure,
    IncrementalRelayFeeRate, MAX_PACKAGE_COUNT, MAX_PACKAGE_WEIGHT, Mempool, MempoolCapacity,
    MempoolEntry, MempoolEntryMetadata, MempoolError, MempoolLifecycleDelta, MempoolMemberIdentity,
    MempoolRejectionCategory, PackageMemberResult, PackageReport, PackageReportError,
    PackageShapeError, PackageStatus, PolicyConfig, ReconsiderableMemberFailure,
    StaticRelayFeeRate, SubmissionPackage, SubmissionPackageKind, SubmitPackageCommand,
    TransactionVirtualSize, TrucPolicy, WellFormedPackage, recompute_resource_ledger,
    validate_standard_transaction,
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

fn empty_snapshot() -> ChainstateSnapshot {
    ChainstateSnapshot::new(Vec::new(), HashMap::new(), HashMap::new())
}

fn identity(byte: u8) -> MempoolMemberIdentity {
    MempoolMemberIdentity {
        txid: Txid::from_byte_array([byte; 32]),
        wtxid: Wtxid::from_byte_array([byte.wrapping_add(100); 32]),
    }
}

fn policy_transaction(id: u8, version: i32, inputs: Vec<OutPoint>) -> Transaction {
    Transaction {
        version,
        inputs: inputs
            .into_iter()
            .map(|previous_output| TransactionInput {
                previous_output,
                script_sig: ScriptBuf::default(),
                sequence: TransactionInput::MAX_SEQUENCE_NONFINAL,
                witness: ScriptWitness::default(),
            })
            .collect::<Vec<_>>(),
        outputs: vec![TransactionOutput {
            value: Amount::from_sats(i64::from(id) + 1).expect("valid output"),
            script_pubkey: ScriptBuf::default(),
        }],
        lock_time: u32::from(id),
    }
}

fn policy_entry(
    id: u8,
    version: i32,
    inputs: Vec<OutPoint>,
    fee_sats: i64,
    virtual_size: usize,
    descendant_count: usize,
) -> MempoolEntry {
    let fee = Amount::from_sats(fee_sats).expect("valid fee");
    let mut entry = MempoolEntry::new(
        policy_transaction(id, version, inputs),
        Txid::from_byte_array([id; 32]),
        Wtxid::from_byte_array([id.wrapping_add(100); 32]),
        fee,
        TransactionVirtualSize::new(virtual_size),
        virtual_size.saturating_mul(4),
        0,
        MempoolEntryMetadata::legacy_unknown(),
    );
    entry.descendant_stats.count = descendant_count;
    entry
}

fn policy_candidate(
    id: u8,
    version: i32,
    inputs: Vec<OutPoint>,
    fee_sats: i64,
    virtual_size: usize,
) -> PreparedCandidate {
    let fee = Amount::from_sats(fee_sats).expect("valid candidate fee");
    PreparedCandidate::for_policy_test(
        policy_entry(id, version, inputs, fee_sats, virtual_size, 1),
        CandidateFees {
            base: fee,
            modified: fee,
        },
    )
}

#[derive(Default)]
struct PolicyView {
    entries: BTreeMap<Txid, MempoolEntry>,
    spenders: BTreeMap<OutPoint, Txid>,
    descendant_calls: Cell<usize>,
}

impl MempoolView for PolicyView {
    fn maybe_entry(&self, txid: &Txid) -> Option<&MempoolEntry> {
        self.entries.get(txid)
    }

    fn maybe_spender(&self, outpoint: &OutPoint) -> Option<Txid> {
        self.spenders.get(outpoint).copied()
    }

    fn collect_descendants(&self, txid: Txid) -> BTreeSet<Txid> {
        self.descendant_calls
            .set(self.descendant_calls.get().saturating_add(1));
        let mut descendants = BTreeSet::new();
        let mut pending = self
            .entries
            .get(&txid)
            .map(|entry| entry.children.iter().copied().collect::<Vec<_>>())
            .unwrap_or_default();
        while let Some(descendant) = pending.pop() {
            if !descendants.insert(descendant) {
                continue;
            }
            if let Some(entry) = self.entries.get(&descendant) {
                pending.extend(entry.children.iter().copied());
            }
        }
        descendants
    }
}

fn checked_fee_group(
    id: EffectiveFeeGroupId,
    ordered_wtxids: Vec<Wtxid>,
) -> Result<EffectiveFeeGroup, EffectiveFeeGroupError> {
    let virtual_size = TransactionVirtualSize::new(100);
    EffectiveFeeGroup::try_new(
        id,
        ordered_wtxids,
        Amount::from_sats(200).expect("base fee"),
        Amount::from_sats(300).expect("modified fee"),
        virtual_size,
        FeeRate::from_fee_sats_and_vbytes(300, virtual_size),
    )
}

fn output_policy_result(
    script_pubkey: ScriptBuf,
    value_sats: i64,
    permissions: EphemeralPolicy,
) -> Result<(), MempoolError> {
    let transaction = Transaction {
        version: 3,
        inputs: vec![TransactionInput {
            previous_output: OutPoint {
                txid: Txid::from_byte_array([0x31; 32]),
                vout: 0,
            },
            script_sig: script(&[0x01, 0x51]),
            sequence: TransactionInput::SEQUENCE_FINAL,
            witness: ScriptWitness::default(),
        }],
        outputs: vec![TransactionOutput {
            value: Amount::from_sats(value_sats).expect("valid output value"),
            script_pubkey,
        }],
        lock_time: 0,
    };
    let input_context = TransactionInputContext {
        spent_output: open_bitcoin_consensus::SpentOutput {
            value: Amount::from_sats(10_000).expect("valid spent value"),
            script_pubkey: script(&[
                0xa9, 0x14, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0x87,
            ]),
            is_coinbase: false,
        },
        created_height: 1,
        created_median_time_past: 1,
    };
    validate_standard_transaction(
        &transaction,
        &[input_context],
        &PolicyConfig {
            ephemeral_policy: permissions,
            permit_bare_anchor: true,
            ..PolicyConfig::default()
        },
        100,
        0,
    )
}

mod dry_run_submit_valid_parent_invalid_child_partial_acceptance_and_lifecyc;
mod max_bound_shape_fingerprint_order_and_try_from_package_refinement_are_pi;
