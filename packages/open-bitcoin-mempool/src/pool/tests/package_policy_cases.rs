// Parity breadcrumbs:
// - packages/bitcoin-knots/doc/policy/packages.md
// - packages/bitcoin-knots/src/kernel/mempool_options.h
// - packages/bitcoin-knots/src/policy/policy.h
// - packages/bitcoin-knots/src/policy/policy.cpp
// - packages/bitcoin-knots/src/script/script.cpp
// - packages/bitcoin-knots/src/test/txpackage_tests.cpp
// - packages/bitcoin-knots/src/test/txvalidation_tests.cpp
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/test/functional/mempool_ephemeral_dust.py
// - packages/bitcoin-knots/test/functional/mempool_truc.py

use open_bitcoin_consensus::{
    ConsensusParams, ScriptVerifyFlags, transaction_txid, transaction_wtxid,
};
use open_bitcoin_primitives::{Amount, ScriptWitness, TransactionInput, Txid, Wtxid};

use crate::{
    AdmissionContext, CandidateFees, DryRunPackageCommand, DustRelayFeeRate, EffectiveFeeGroupId,
    EphemeralPolicy, FeeRate, IncrementalRelayFeeRate, Mempool, MempoolCapacity,
    MempoolMemberIdentity, MempoolRemovalCause, MempoolRemovalRole, MempoolRetryClearCause,
    PackageFeeError, PackageFeeMember, PackageMemberResult, PolicyConfig, PriorMemberSuccess,
    ResourceAccountingError, RollingMempoolFeeRate, StaticRelayFeeRate, SubmissionPackage,
    SubmitPackageCommand, TransactionVirtualSize, TrucPolicy, WellFormedPackage,
    dust_threshold_sats_at_rate, evaluate_package_fee_group, validate_standard_transaction,
};

use super::{sample_chainstate_snapshot, script, spend_transaction, submit};

fn identity(byte: u8) -> MempoolMemberIdentity {
    MempoolMemberIdentity {
        txid: Txid::from_byte_array([byte; 32]),
        wtxid: Wtxid::from_byte_array([byte.wrapping_add(1); 32]),
    }
}

fn member(byte: u8, version: i32, base: i64, modified: i64, vsize: usize) -> PackageFeeMember {
    PackageFeeMember {
        identity: identity(byte),
        version,
        fees: CandidateFees {
            base: Amount::from_sats(base).expect("valid base fee"),
            modified: Amount::from_sats(modified).expect("valid modified fee"),
        },
        virtual_size: TransactionVirtualSize::new(vsize),
    }
}

fn static_floor(sats_per_kvb: i64) -> StaticRelayFeeRate {
    StaticRelayFeeRate::new(FeeRate::from_sats_per_kvb(sats_per_kvb))
}

fn rolling_floor(sats_per_kvb: i64) -> RollingMempoolFeeRate {
    RollingMempoolFeeRate::new(FeeRate::from_sats_per_kvb(sats_per_kvb))
}

fn p2a_script() -> open_bitcoin_primitives::ScriptBuf {
    script(&[0x51, 0x02, 0x4e, 0x73])
}

fn p2sh_script() -> open_bitcoin_primitives::ScriptBuf {
    script(&[
        0xa9, 0x14, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0x87,
    ])
}

fn ephemeral_parent(
    confirmed_txid: Txid,
    ordinary_value_sats: i64,
) -> open_bitcoin_primitives::Transaction {
    open_bitcoin_primitives::Transaction {
        version: 3,
        inputs: vec![TransactionInput {
            previous_output: open_bitcoin_primitives::OutPoint {
                txid: confirmed_txid,
                vout: 0,
            },
            script_sig: script(&[0x01, 0x51]),
            sequence: TransactionInput::SEQUENCE_FINAL,
            witness: ScriptWitness::default(),
        }],
        outputs: vec![
            open_bitcoin_primitives::TransactionOutput {
                value: Amount::from_sats(ordinary_value_sats).expect("valid ordinary value"),
                script_pubkey: super::p2sh_script(),
            },
            open_bitcoin_primitives::TransactionOutput {
                value: Amount::ZERO,
                script_pubkey: p2a_script(),
            },
        ],
        lock_time: 0,
    }
}

fn ephemeral_child(
    parent_txid: Txid,
    spend_dust: bool,
    output_value_sats: i64,
) -> open_bitcoin_primitives::Transaction {
    let mut inputs = vec![TransactionInput {
        previous_output: open_bitcoin_primitives::OutPoint {
            txid: parent_txid,
            vout: 0,
        },
        script_sig: script(&[0x01, 0x51]),
        sequence: TransactionInput::SEQUENCE_FINAL,
        witness: ScriptWitness::default(),
    }];
    if spend_dust {
        inputs.push(TransactionInput {
            previous_output: open_bitcoin_primitives::OutPoint {
                txid: parent_txid,
                vout: 1,
            },
            script_sig: script(&[]),
            sequence: TransactionInput::SEQUENCE_FINAL,
            witness: ScriptWitness::default(),
        });
    }
    open_bitcoin_primitives::Transaction {
        version: 3,
        inputs,
        outputs: vec![open_bitcoin_primitives::TransactionOutput {
            value: Amount::from_sats(output_value_sats).expect("valid child value"),
            script_pubkey: super::p2sh_script(),
        }],
        lock_time: 0,
    }
}

fn output_policy_result(
    script_pubkey: open_bitcoin_primitives::ScriptBuf,
    value_sats: i64,
    permissions: EphemeralPolicy,
) -> Result<(), crate::MempoolError> {
    transaction_output_policy_result(
        vec![open_bitcoin_primitives::TransactionOutput {
            value: Amount::from_sats(value_sats).expect("valid output value"),
            script_pubkey,
        }],
        PolicyConfig {
            ephemeral_policy: permissions,
            permit_bare_datacarrier: true,
            permit_bare_anchor: true,
            ..PolicyConfig::default()
        },
    )
}

fn transaction_output_policy_result(
    outputs: Vec<open_bitcoin_primitives::TransactionOutput>,
    config: PolicyConfig,
) -> Result<(), crate::MempoolError> {
    let transaction = open_bitcoin_primitives::Transaction {
        version: 2,
        inputs: vec![TransactionInput {
            previous_output: open_bitcoin_primitives::OutPoint {
                txid: Txid::from_byte_array([0x31; 32]),
                vout: 0,
            },
            script_sig: script(&[0x01, 0x51]),
            sequence: TransactionInput::SEQUENCE_FINAL,
            witness: ScriptWitness::default(),
        }],
        outputs,
        lock_time: 0,
    };
    let input_context = open_bitcoin_consensus::TransactionInputContext {
        spent_output: open_bitcoin_consensus::SpentOutput {
            value: Amount::from_sats(10_000).expect("valid spent value"),
            script_pubkey: p2sh_script(),
            is_coinbase: false,
        },
        created_height: 1,
        created_median_time_past: 1,
    };

    validate_standard_transaction(&transaction, &[input_context], &config, 100, 0)
}

fn package_rbf_transactions(
    confirmed_txid: Txid,
) -> (
    open_bitcoin_primitives::Transaction,
    open_bitcoin_primitives::Transaction,
    open_bitcoin_primitives::Transaction,
    open_bitcoin_primitives::Transaction,
) {
    let original_parent = spend_transaction(
        confirmed_txid,
        0,
        499_999_000,
        TransactionInput::MAX_SEQUENCE_NONFINAL - 1,
    );
    let original_parent_txid = transaction_txid(&original_parent).expect("original parent txid");
    let original_child = spend_transaction(
        original_parent_txid,
        0,
        499_998_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    let replacement_parent = spend_transaction(
        confirmed_txid,
        0,
        499_996_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    let replacement_parent_txid =
        transaction_txid(&replacement_parent).expect("replacement parent txid");
    let replacement_child = spend_transaction(
        replacement_parent_txid,
        0,
        499_990_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    (
        original_parent,
        original_child,
        replacement_parent,
        replacement_child,
    )
}

mod incremental_relay_fee_does_not_change_static_or_rolling_assessment;
mod newly_present_becomes_post_trim_absent_and_dry_run_submit_reports_are_eq;
mod pay_to_anchor_defaults_and_dust_relay_thresholds_are_pinned;
