// Parity breadcrumbs:
// - packages/bitcoin-knots/src/rpc/mempool.cpp

//! Projector family for an authoritative [`PackageReport`].
//!
//! Knots JSON must not include fingerprint, admission, or dual-state keys.
//! [`project_open_bitcoin_package`] is the D-05 typed sibling and may emit
//! fingerprint, still-present, and relay_disabled on the extension path.

#![allow(dead_code)]

use open_bitcoin_mempool::{
    EffectiveFeeGroup, EffectiveFeeGroupId, HardMemberFailure, MempoolMemberIdentity,
    PackageMemberResult, PackageReport, PackageStatus, ReconsiderableMemberFailure,
};
use open_bitcoin_node::core::primitives::{Txid, Wtxid};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value, json};

mod report_view;
#[cfg(test)]
mod tests;

const SATOSHIS_PER_BITCOIN: u64 = 100_000_000;

/// Caller-supplied per-member sizes and fees used by Knots result bodies.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackageMemberProjectionFacts {
    pub txid: Txid,
    pub wtxid: Wtxid,
    pub virtual_size: u64,
    pub base_fee_sats: i64,
}

/// Failures while looking up caller-supplied member facts.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PackageProjectionError {
    MissingMemberFacts { index: usize },
    MemberFactsWtxidMismatch { index: usize },
    DualStateLengthMismatch { expected: usize, actual: usize },
}

/// Admission tokens on the typed Open Bitcoin package report.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PackageAdmissionState {
    Accepted,
    StillPresent,
    Cleared,
}

/// Relay/fanout tokens on the typed Open Bitcoin package report.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PackageRelayState {
    Eligible,
    Queued,
    Attempted,
    Emitted,
    Requested,
    Served,
    Suppressed,
    RelayDisabled,
}

/// Caller-supplied dual-state for one input-ordered member.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct PackageMemberDualState {
    pub admission: PackageAdmissionState,
    pub relay: PackageRelayState,
}

/// Identity-bearing submit facts used to walk the D-15 relay ladder.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct PackageSubmitRelayFacts {
    pub queued: bool,
    pub leftover_unattempted: bool,
    pub emitted: bool,
    pub requested: bool,
    pub served: bool,
    pub suppressed: bool,
    pub unbroadcast: bool,
    pub admitted: bool,
}

/// Projects Knots `testmempoolaccept` array JSON from one package report.
pub fn project_testmempoolaccept(
    report: &PackageReport,
    member_facts: &[PackageMemberProjectionFacts],
) -> Result<Value, PackageProjectionError> {
    let maybe_package_error = maybe_failed_package_error(report);
    let mut elements = Vec::with_capacity(report.members().len());
    for (index, member) in report.members().iter().enumerate() {
        elements.push(project_testmempoolaccept_member(
            index,
            member,
            report,
            member_facts,
            maybe_package_error.as_deref(),
        )?);
    }
    Ok(Value::Array(elements))
}

/// Projects Knots `submitpackage` object JSON from the same package report.
pub fn project_submitpackage(
    report: &PackageReport,
    member_facts: &[PackageMemberProjectionFacts],
    replaced_txids: &[Txid],
) -> Result<Value, PackageProjectionError> {
    let mut tx_results = Map::new();
    let mut earlier_member_failed = false;
    for (index, member) in report.members().iter().enumerate() {
        let (key, value) = project_submitpackage_member(
            index,
            member,
            report,
            member_facts,
            earlier_member_failed,
        )?;
        tx_results.insert(key, value);
        earlier_member_failed |= member_failed(member);
    }

    let replaced = replaced_txids
        .iter()
        .map(|txid| Value::String(encode_hex(txid.as_bytes())))
        .collect();

    let mut object = Map::new();
    object.insert(
        "package_msg".to_string(),
        Value::String(package_msg(report)),
    );
    object.insert("tx-results".to_string(), Value::Object(tx_results));
    object.insert("replaced-transactions".to_string(), Value::Array(replaced));
    Ok(Value::Object(object))
}

/// Projects the typed Open Bitcoin package report from the same PackageReport family.
pub fn project_open_bitcoin_package(
    report: &PackageReport,
    member_facts: &[PackageMemberProjectionFacts],
    member_dual_state: &[PackageMemberDualState],
) -> Result<Value, PackageProjectionError> {
    if member_dual_state.len() != report.members().len() {
        return Err(PackageProjectionError::DualStateLengthMismatch {
            expected: report.members().len(),
            actual: member_dual_state.len(),
        });
    }

    let mut members = Vec::with_capacity(report.members().len());
    for (index, (member, dual_state)) in report
        .members()
        .iter()
        .zip(member_dual_state.iter())
        .enumerate()
    {
        members.push(project_open_bitcoin_member(
            index,
            member,
            member_facts,
            *dual_state,
        )?);
    }

    let mut object = Map::new();
    object.insert(
        "fingerprint".to_string(),
        Value::String(encode_hex(report.fingerprint().as_bytes())),
    );
    object.insert(
        "status".to_string(),
        Value::String(package_status_token(*report.status()).to_string()),
    );
    object.insert("members".to_string(), Value::Array(members));
    object.insert(
        "effective_fee_groups".to_string(),
        project_effective_fee_groups(report),
    );
    Ok(Value::Object(object))
}

/// Maps PackageMemberResult onto the locked admission tokens for both modes.
pub fn admission_state_for_member(member: &PackageMemberResult) -> PackageAdmissionState {
    match member {
        PackageMemberResult::FinallyPresent(_) => PackageAdmissionState::Accepted,
        PackageMemberResult::AlreadyPresent(_) => PackageAdmissionState::StillPresent,
        PackageMemberResult::HardRejected(_)
        | PackageMemberResult::Reconsiderable(_)
        | PackageMemberResult::SameTxidDifferentWitness(_)
        | PackageMemberResult::PostTrimAbsent(_) => PackageAdmissionState::Cleared,
    }
}

/// Dry-run relay uses only RelayIntent / network.relay plus presence.
pub fn dry_run_relay_state_for_member(
    member: &PackageMemberResult,
    relay_disabled: bool,
) -> PackageRelayState {
    if relay_disabled {
        return PackageRelayState::RelayDisabled;
    }
    match member {
        PackageMemberResult::FinallyPresent(_) | PackageMemberResult::AlreadyPresent(_) => {
            PackageRelayState::Eligible
        }
        PackageMemberResult::HardRejected(_)
        | PackageMemberResult::Reconsiderable(_)
        | PackageMemberResult::SameTxidDifferentWitness(_)
        | PackageMemberResult::PostTrimAbsent(_) => PackageRelayState::Suppressed,
    }
}

/// Submit relay walks the D-15 transport ladder; first match wins.
pub fn submit_relay_state_for_member(
    member: &PackageMemberResult,
    relay_disabled: bool,
    facts: PackageSubmitRelayFacts,
) -> PackageRelayState {
    if relay_disabled {
        return PackageRelayState::RelayDisabled;
    }
    if facts.queued {
        return PackageRelayState::Queued;
    }
    if facts.leftover_unattempted {
        return PackageRelayState::Attempted;
    }
    if facts.emitted {
        return PackageRelayState::Emitted;
    }
    if facts.requested {
        return PackageRelayState::Requested;
    }
    if facts.served {
        return PackageRelayState::Served;
    }
    if facts.suppressed {
        return PackageRelayState::Suppressed;
    }
    if matches!(
        member,
        PackageMemberResult::FinallyPresent(_) | PackageMemberResult::AlreadyPresent(_)
    ) || facts.admitted
        || facts.unbroadcast
    {
        return PackageRelayState::Eligible;
    }
    PackageRelayState::Suppressed
}

fn project_open_bitcoin_member(
    index: usize,
    member: &PackageMemberResult,
    member_facts: &[PackageMemberProjectionFacts],
    dual_state: PackageMemberDualState,
) -> Result<Value, PackageProjectionError> {
    let identity = member.requested_identity();
    let facts = member_facts
        .iter()
        .find(|facts| facts.wtxid == identity.wtxid)
        .or_else(|| member_facts.get(index));
    let txid_hex = facts.map_or_else(
        || encode_hex(identity.txid.as_bytes()),
        |facts| encode_hex(facts.txid.as_bytes()),
    );
    let wtxid_hex = facts.map_or_else(
        || encode_hex(identity.wtxid.as_bytes()),
        |facts| encode_hex(facts.wtxid.as_bytes()),
    );
    Ok(json!({
        "txid": txid_hex,
        "wtxid": wtxid_hex,
        "result": member_result_name(member),
        "admission": admission_token(dual_state.admission),
        "relay": relay_token(dual_state.relay),
    }))
}

fn admission_token(state: PackageAdmissionState) -> &'static str {
    match state {
        PackageAdmissionState::Accepted => "accepted",
        PackageAdmissionState::StillPresent => "still-present",
        PackageAdmissionState::Cleared => "cleared",
    }
}

fn relay_token(state: PackageRelayState) -> &'static str {
    match state {
        PackageRelayState::Eligible => "eligible",
        PackageRelayState::Queued => "queued",
        PackageRelayState::Attempted => "attempted",
        PackageRelayState::Emitted => "emitted",
        PackageRelayState::Requested => "requested",
        PackageRelayState::Served => "served",
        PackageRelayState::Suppressed => "suppressed",
        PackageRelayState::RelayDisabled => "relay_disabled",
    }
}

fn member_result_name(member: &PackageMemberResult) -> &'static str {
    match member {
        PackageMemberResult::FinallyPresent(_) => "FinallyPresent",
        PackageMemberResult::AlreadyPresent(_) => "AlreadyPresent",
        PackageMemberResult::SameTxidDifferentWitness(_) => "SameTxidDifferentWitness",
        PackageMemberResult::HardRejected(_) => "HardRejected",
        PackageMemberResult::Reconsiderable(_) => "Reconsiderable",
        PackageMemberResult::PostTrimAbsent(_) => "PostTrimAbsent",
    }
}

fn package_status_token(status: PackageStatus) -> &'static str {
    match status {
        PackageStatus::Complete => "complete",
        PackageStatus::Partial => "partial",
        PackageStatus::Failed => "failed",
    }
}

fn project_effective_fee_groups(report: &PackageReport) -> Value {
    let groups = report_view::fee_groups(report)
        .iter()
        .map(|group| {
            json!({
                "id": group.id().as_u64(),
                "ordered_wtxids": group
                    .ordered_wtxids()
                    .iter()
                    .map(|wtxid| encode_hex(wtxid.as_bytes()))
                    .collect::<Vec<_>>(),
                "base_fee_sats": group.base_fee_sats().to_sats(),
                "modified_fee_sats": group.modified_fee_sats().to_sats(),
                "virtual_size": group.virtual_size().as_usize(),
                "effective_fee_rate_sats_per_kvb": group.effective_fee_rate().sats_per_kvb(),
            })
        })
        .collect();
    Value::Array(groups)
}

fn project_submitpackage_member(
    index: usize,
    member: &PackageMemberResult,
    report: &PackageReport,
    member_facts: &[PackageMemberProjectionFacts],
    earlier_member_failed: bool,
) -> Result<(String, Value), PackageProjectionError> {
    let identity = member.requested_identity();
    let key = encode_hex(identity.wtxid.as_bytes());
    let mut object = Map::new();
    object.insert(
        "txid".to_string(),
        Value::String(encode_hex(identity.txid.as_bytes())),
    );

    match member {
        PackageMemberResult::FinallyPresent(present) => {
            let facts = require_facts(identity, member_facts, index)?;
            let maybe_group = maybe_fee_group(report, present.effective_fee_group_id);
            object.insert("vsize".to_string(), json!(facts.virtual_size));
            object.insert("fees".to_string(), finally_present_fees(facts, maybe_group));
        }
        PackageMemberResult::AlreadyPresent(_) => {
            let facts = require_facts(identity, member_facts, index)?;
            object.insert("vsize".to_string(), json!(facts.virtual_size));
            object.insert("fees".to_string(), already_present_fees(facts));
        }
        PackageMemberResult::SameTxidDifferentWitness(alias) => {
            object.insert(
                "other-wtxid".to_string(),
                Value::String(encode_hex(alias.existing_wtxid.as_bytes())),
            );
        }
        PackageMemberResult::HardRejected(failure) => {
            insert_error(&mut object, hard_reject_reason(failure));
        }
        PackageMemberResult::Reconsiderable(failure) => {
            if earlier_member_failed
                && matches!(failure, ReconsiderableMemberFailure::MissingInputs { .. })
            {
                insert_error(&mut object, "unevaluated");
            } else {
                insert_error(&mut object, &reconsiderable_reject_reason(failure));
            }
        }
        PackageMemberResult::PostTrimAbsent(_) => {
            insert_error(&mut object, "mempool full");
        }
    }

    Ok((key, Value::Object(object)))
}

fn package_msg(report: &PackageReport) -> String {
    if *report.status() == PackageStatus::Complete {
        return "success".to_string();
    }

    report
        .members()
        .iter()
        .find_map(|member| match member {
            PackageMemberResult::HardRejected(failure) => {
                Some(hard_reject_reason(failure).to_string())
            }
            _ => None,
        })
        .unwrap_or_else(|| "transaction failed".to_string())
}

fn member_failed(member: &PackageMemberResult) -> bool {
    matches!(
        member,
        PackageMemberResult::HardRejected(_)
            | PackageMemberResult::Reconsiderable(_)
            | PackageMemberResult::PostTrimAbsent(_)
    )
}

fn already_present_fees(facts: &PackageMemberProjectionFacts) -> Value {
    let mut fees = Map::new();
    fees.insert(
        "base".to_string(),
        amount_sats_to_btc_json(facts.base_fee_sats),
    );
    Value::Object(fees)
}

fn insert_error(object: &mut Map<String, Value>, message: &str) {
    object.insert("error".to_string(), Value::String(message.to_string()));
}

fn project_testmempoolaccept_member(
    index: usize,
    member: &PackageMemberResult,
    report: &PackageReport,
    member_facts: &[PackageMemberProjectionFacts],
    maybe_package_error: Option<&str>,
) -> Result<Value, PackageProjectionError> {
    let identity = member.requested_identity();
    let mut object = identity_object(identity);
    if let Some(package_error) = maybe_package_error {
        object.insert(
            "package-error".to_string(),
            Value::String(package_error.to_string()),
        );
    }

    match member {
        PackageMemberResult::FinallyPresent(present) => {
            let facts = require_facts(identity, member_facts, index)?;
            let maybe_group = maybe_fee_group(report, present.effective_fee_group_id);
            object.insert("allowed".to_string(), json!(true));
            object.insert("vsize".to_string(), json!(facts.virtual_size));
            object.insert("fees".to_string(), finally_present_fees(facts, maybe_group));
        }
        PackageMemberResult::AlreadyPresent(_) => {
            insert_rejected(&mut object, "txn-already-in-mempool");
        }
        PackageMemberResult::SameTxidDifferentWitness(_) => {
            insert_rejected(&mut object, "txn-same-nonwitness-data-already-in-mempool");
        }
        PackageMemberResult::HardRejected(failure) => {
            insert_rejected(&mut object, hard_reject_reason(failure));
        }
        PackageMemberResult::Reconsiderable(failure) => {
            insert_rejected(&mut object, &reconsiderable_reject_reason(failure));
        }
        PackageMemberResult::PostTrimAbsent(_) => {
            insert_rejected(&mut object, "mempool full");
        }
    }

    Ok(Value::Object(object))
}

fn identity_object(identity: MempoolMemberIdentity) -> Map<String, Value> {
    let mut object = Map::new();
    object.insert(
        "txid".to_string(),
        Value::String(encode_hex(identity.txid.as_bytes())),
    );
    object.insert(
        "wtxid".to_string(),
        Value::String(encode_hex(identity.wtxid.as_bytes())),
    );
    object
}

fn insert_rejected(object: &mut Map<String, Value>, reason: &str) {
    object.insert("allowed".to_string(), json!(false));
    object.insert(
        "reject-reason".to_string(),
        Value::String(reason.to_string()),
    );
}

fn finally_present_fees(
    facts: &PackageMemberProjectionFacts,
    maybe_group: Option<&EffectiveFeeGroup>,
) -> Value {
    let mut fees = Map::new();
    fees.insert(
        "base".to_string(),
        amount_sats_to_btc_json(facts.base_fee_sats),
    );
    let (feerate_sats, includes) = match maybe_group {
        Some(group) => (
            group.effective_fee_rate().sats_per_kvb(),
            group
                .ordered_wtxids()
                .iter()
                .map(|wtxid| Value::String(encode_hex(wtxid.as_bytes())))
                .collect(),
        ),
        None => (0, Vec::new()),
    };
    fees.insert(
        "effective-feerate".to_string(),
        amount_sats_to_btc_json(feerate_sats),
    );
    fees.insert("effective-includes".to_string(), Value::Array(includes));
    Value::Object(fees)
}

fn maybe_failed_package_error(report: &PackageReport) -> Option<String> {
    if *report.status() != PackageStatus::Failed {
        return None;
    }

    report
        .members()
        .iter()
        .find_map(maybe_reject_reason)
        .or_else(|| Some("transaction failed".to_string()))
}

fn maybe_reject_reason(member: &PackageMemberResult) -> Option<String> {
    match member {
        PackageMemberResult::AlreadyPresent(_) => Some("txn-already-in-mempool".to_string()),
        PackageMemberResult::SameTxidDifferentWitness(_) => {
            Some("txn-same-nonwitness-data-already-in-mempool".to_string())
        }
        PackageMemberResult::HardRejected(failure) => Some(hard_reject_reason(failure).to_string()),
        PackageMemberResult::Reconsiderable(failure) => Some(reconsiderable_reject_reason(failure)),
        PackageMemberResult::PostTrimAbsent(_) => Some("mempool full".to_string()),
        PackageMemberResult::FinallyPresent(_) => None,
    }
}

fn hard_reject_reason(failure: &HardMemberFailure) -> &str {
    let reason = match failure {
        HardMemberFailure::Policy { reason, .. }
        | HardMemberFailure::TrucPolicy { reason, .. }
        | HardMemberFailure::EphemeralPolicy { reason, .. }
        | HardMemberFailure::PackageReplacement { reason, .. } => reason.as_str(),
    };
    if reason.is_empty() {
        "rejected"
    } else {
        reason
    }
}

fn reconsiderable_reject_reason(failure: &ReconsiderableMemberFailure) -> String {
    match failure {
        ReconsiderableMemberFailure::MissingInputs { .. } => "missing-inputs".to_string(),
        ReconsiderableMemberFailure::PackageReplacement { .. } => {
            "package rbf rejected".to_string()
        }
        ReconsiderableMemberFailure::PackageFee { .. } => "package-fee".to_string(),
    }
}

fn maybe_fee_group(report: &PackageReport, id: EffectiveFeeGroupId) -> Option<&EffectiveFeeGroup> {
    report_view::fee_groups(report)
        .iter()
        .find(|group| group.id() == id)
}

fn require_facts(
    identity: MempoolMemberIdentity,
    member_facts: &[PackageMemberProjectionFacts],
    index: usize,
) -> Result<&PackageMemberProjectionFacts, PackageProjectionError> {
    let Some(facts) = member_facts
        .iter()
        .find(|facts| facts.wtxid == identity.wtxid)
    else {
        return Err(PackageProjectionError::MissingMemberFacts { index });
    };
    if facts.txid != identity.txid {
        return Err(PackageProjectionError::MemberFactsWtxidMismatch { index });
    }
    Ok(facts)
}

fn encode_hex(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut output = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        output.push(HEX[(byte >> 4) as usize] as char);
        output.push(HEX[(byte & 0x0f) as usize] as char);
    }
    output
}

fn amount_sats_to_btc_json(sats: i64) -> Value {
    let negative = sats < 0;
    let abs = sats.unsigned_abs();
    let whole = abs / SATOSHIS_PER_BITCOIN;
    let frac = abs % SATOSHIS_PER_BITCOIN;
    let formatted = if negative {
        format!("-{whole}.{frac:08}")
    } else {
        format!("{whole}.{frac:08}")
    };
    formatted
        .parse::<serde_json::Number>()
        .map(Value::Number)
        .unwrap_or(Value::String(formatted))
}
