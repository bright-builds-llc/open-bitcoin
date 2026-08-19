// Parity breadcrumbs:
// - packages/bitcoin-knots/src/bitcoind.cpp
// - packages/bitcoin-knots/src/rpc/protocol.h
// - packages/bitcoin-knots/src/rpc/request.cpp
// - packages/bitcoin-knots/src/rpc/server.cpp
// - packages/bitcoin-knots/src/rpc/blockchain.cpp
// - packages/bitcoin-knots/src/rpc/mempool.cpp
// - packages/bitcoin-knots/src/rpc/net.cpp
// - packages/bitcoin-knots/src/rpc/rawtransaction.cpp
// - packages/bitcoin-knots/test/functional/interface_rpc.py

use std::collections::HashMap;

use open_bitcoin_mempool::{
    MAX_PACKAGE_COUNT, MempoolLifecycleRemoval, MempoolRemovalCause, PackageMemberResult,
    PackageReport, transaction_weight_and_virtual_size,
};
use open_bitcoin_node::core::{
    chainstate::ChainstateSnapshot,
    codec::parse_transaction,
    consensus::{transaction_txid, transaction_wtxid},
    primitives::{OutPoint, Transaction, Txid, Wtxid},
};
use open_bitcoin_node::{ManagedNetworkAuthorityError, ManagedNetworkError};
use serde_json::Value;

use crate::{
    ManagedRpcContext,
    error::RpcFailure,
    method::{SubmitPackageRequest, TestMempoolAcceptRequest},
    package_projection::{
        PackageMemberProjectionFacts, project_submitpackage, project_testmempoolaccept,
    },
};

use super::{decode, network_authority_error_to_failure, node::current_timestamp_unix_seconds};

const PACKAGE_COUNT_MESSAGE: &str = "Array must contain between 1 and 25 transactions.";
const UNSUPPORTED_MAX_FEE_RATE_MESSAGE: &str =
    "package RPC maxfeerate enforcement is not supported; omit maxfeerate";
const UNSUPPORTED_MAX_BURN_AMOUNT_MESSAGE: &str =
    "package RPC maxburnamount enforcement is not supported; omit maxburnamount";
const UNSUPPORTED_IGNORE_REJECTS_MESSAGE: &str =
    "package RPC ignore_rejects is not supported; omit ignore_rejects or pass an empty array";

pub(super) fn test_mempool_accept(
    context: &ManagedRpcContext,
    request: TestMempoolAcceptRequest,
) -> Result<Value, RpcFailure> {
    reject_unsupported_options(
        request.maybe_max_fee_rate,
        request.maybe_max_burn_amount,
        &request.ignore_rejects,
    )?;
    reject_package_count(request.raw_txs.len())?;
    let transactions = decode_package_transactions(&request.raw_txs)?;
    let now_unix_seconds = current_timestamp_unix_seconds()?;
    let report = context
        .dry_run_local_package(transactions.clone(), now_unix_seconds)
        .map_err(package_authority_error_to_failure)?;
    let facts = member_projection_facts(context, &transactions, &report)?;
    project_testmempoolaccept(&report, &facts)
        .map_err(|error| RpcFailure::internal_error(format!("{error:?}")))
}

pub(super) fn submit_package(
    context: &ManagedRpcContext,
    request: SubmitPackageRequest,
) -> Result<Value, RpcFailure> {
    reject_unsupported_options(
        request.maybe_max_fee_rate,
        request.maybe_max_burn_amount,
        &request.ignore_rejects,
    )?;
    reject_package_count(request.package.len())?;
    let transactions = decode_package_transactions(&request.package)?;
    let now_unix_seconds = current_timestamp_unix_seconds()?;
    let submitted = context
        .submit_local_package(transactions.clone(), now_unix_seconds)
        .map_err(package_authority_error_to_failure)?;
    let facts = member_projection_facts(context, &transactions, &submitted.report)?;
    let replaced_txids = replacement_txids(&submitted.delta.removed);
    project_submitpackage(&submitted.report, &facts, &replaced_txids)
        .map_err(|error| RpcFailure::internal_error(format!("{error:?}")))
}

fn reject_unsupported_options(
    maybe_max_fee_rate: Option<i64>,
    maybe_max_burn_amount: Option<i64>,
    ignore_rejects: &[String],
) -> Result<(), RpcFailure> {
    if maybe_max_fee_rate.is_some() {
        return Err(RpcFailure::invalid_parameter(
            UNSUPPORTED_MAX_FEE_RATE_MESSAGE,
        ));
    }
    if maybe_max_burn_amount.is_some() {
        return Err(RpcFailure::invalid_parameter(
            UNSUPPORTED_MAX_BURN_AMOUNT_MESSAGE,
        ));
    }
    if !ignore_rejects.is_empty() {
        return Err(RpcFailure::invalid_parameter(
            UNSUPPORTED_IGNORE_REJECTS_MESSAGE,
        ));
    }
    Ok(())
}

fn reject_package_count(count: usize) -> Result<(), RpcFailure> {
    if count == 0 || count > MAX_PACKAGE_COUNT {
        return Err(RpcFailure::invalid_parameter(PACKAGE_COUNT_MESSAGE));
    }
    Ok(())
}

fn decode_package_transactions(hexes: &[String]) -> Result<Vec<Transaction>, RpcFailure> {
    let mut transactions = Vec::with_capacity(hexes.len());
    for hex in hexes {
        let bytes = decode::decode_hex(hex).map_err(|error| {
            RpcFailure::deserialization_error(format!("TX decode failed: {error}"))
        })?;
        let transaction = parse_transaction(&bytes).map_err(|error| {
            RpcFailure::deserialization_error(format!(
                "TX decode failed: {error}. Make sure the tx has at least one input."
            ))
        })?;
        transactions.push(transaction);
    }
    Ok(transactions)
}

fn package_authority_error_to_failure(error: ManagedNetworkAuthorityError) -> RpcFailure {
    match error {
        ManagedNetworkAuthorityError::Operation(ManagedNetworkError::PackageShape(shape)) => {
            RpcFailure::verify_error(shape.to_string())
        }
        other => network_authority_error_to_failure(other),
    }
}

fn member_projection_facts(
    context: &ManagedRpcContext,
    transactions: &[Transaction],
    report: &PackageReport,
) -> Result<Vec<PackageMemberProjectionFacts>, RpcFailure> {
    let snapshot = context
        .blockchain_snapshot()
        .map_err(network_authority_error_to_failure)?;
    let known_values = known_input_values(transactions, &snapshot);
    let mut facts = Vec::with_capacity(transactions.len());
    for (index, transaction) in transactions.iter().enumerate() {
        let txid = transaction_txid(transaction)
            .map_err(|error| RpcFailure::internal_error(error.to_string()))?;
        let wtxid = transaction_wtxid(transaction)
            .map_err(|error| RpcFailure::internal_error(error.to_string()))?;
        let (_, virtual_size) = transaction_weight_and_virtual_size(transaction)
            .map_err(|error| RpcFailure::internal_error(error.to_string()))?;
        let maybe_member = report.members().get(index);
        let maybe_base_fee = maybe_base_fee_sats(transaction, &wtxid, report, &known_values);
        let Some(base_fee_sats) = maybe_base_fee else {
            if member_requires_facts(maybe_member) {
                return Err(RpcFailure::internal_error(
                    "package member projection facts are unavailable for a present member",
                ));
            }
            facts.push(PackageMemberProjectionFacts {
                txid,
                wtxid,
                virtual_size: virtual_size as u64,
                base_fee_sats: 0,
            });
            continue;
        };
        facts.push(PackageMemberProjectionFacts {
            txid,
            wtxid,
            virtual_size: virtual_size as u64,
            base_fee_sats,
        });
    }
    Ok(facts)
}

fn member_requires_facts(maybe_member: Option<&PackageMemberResult>) -> bool {
    matches!(
        maybe_member,
        Some(PackageMemberResult::FinallyPresent(_) | PackageMemberResult::AlreadyPresent(_))
    )
}

fn known_input_values(
    transactions: &[Transaction],
    snapshot: &ChainstateSnapshot,
) -> HashMap<OutPoint, i64> {
    let mut values = HashMap::new();
    for (outpoint, coin) in &snapshot.utxos {
        values.insert(outpoint.clone(), coin.output.value.to_sats());
    }
    for transaction in transactions {
        let Ok(txid) = transaction_txid(transaction) else {
            continue;
        };
        for (vout, output) in transaction.outputs.iter().enumerate() {
            let Ok(vout) = u32::try_from(vout) else {
                continue;
            };
            values.insert(
                OutPoint {
                    txid: txid.clone(),
                    vout,
                },
                output.value.to_sats(),
            );
        }
    }
    values
}

fn maybe_base_fee_sats(
    transaction: &Transaction,
    wtxid: &Wtxid,
    report: &PackageReport,
    known_values: &HashMap<OutPoint, i64>,
) -> Option<i64> {
    if let Some(fee) = maybe_input_minus_output_fee(transaction, known_values) {
        return Some(fee);
    }
    maybe_singleton_group_fee(report, wtxid)
}

fn maybe_input_minus_output_fee(
    transaction: &Transaction,
    known_values: &HashMap<OutPoint, i64>,
) -> Option<i64> {
    let mut input_sum = 0_i64;
    for input in &transaction.inputs {
        input_sum = input_sum.checked_add(*known_values.get(&input.previous_output)?)?;
    }
    let mut output_sum = 0_i64;
    for output in &transaction.outputs {
        output_sum = output_sum.checked_add(output.value.to_sats())?;
    }
    input_sum.checked_sub(output_sum)
}

fn maybe_singleton_group_fee(
    report: &PackageReport,
    wtxid: &Wtxid,
) -> Option<i64> {
    report.effective_fee_groups().iter().find_map(|group| {
        let wtxids = group.ordered_wtxids();
        if wtxids.len() == 1 && wtxids[0] == *wtxid {
            Some(group.base_fee_sats().to_sats())
        } else {
            None
        }
    })
}

fn replacement_txids(removals: &[MempoolLifecycleRemoval]) -> Vec<Txid> {
    removals
        .iter()
        .filter(|removal| removal.cause == MempoolRemovalCause::Replacement)
        .map(|removal| removal.member.txid.clone())
        .collect()
}
