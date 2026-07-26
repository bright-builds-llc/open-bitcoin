// Parity breadcrumbs:
// - packages/bitcoin-knots/src/policy/ephemeral_policy.h
// - packages/bitcoin-knots/src/policy/ephemeral_policy.cpp
// - packages/bitcoin-knots/src/policy/policy.h
// - packages/bitcoin-knots/src/policy/policy.cpp
// - packages/bitcoin-knots/src/test/txvalidation_tests.cpp
// - packages/bitcoin-knots/test/functional/mempool_ephemeral_dust.py

//! Pure zero-fee and complete-spend policy for permitted ephemeral dust.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use open_bitcoin_primitives::{OutPoint, Txid};

use super::output::is_permitted_ephemeral_dust;
use super::replacement::MempoolView;
use crate::pool::candidate::PreparedCandidate;
use crate::{DustRelayFeeRate, EphemeralPolicy, MempoolEntry};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum EphemeralPolicyError {
    DustyTransactionHasFee {
        txid: Txid,
        base_fee_sats: i64,
        modified_fee_sats: i64,
    },
    MissingEphemeralSpends {
        child: Txid,
        missing: BTreeSet<OutPoint>,
    },
}

impl fmt::Display for EphemeralPolicyError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DustyTransactionHasFee {
                txid,
                base_fee_sats,
                modified_fee_sats,
            } => write!(
                formatter,
                "transaction {txid:?} with dust output must have zero base and modified fee, got {base_fee_sats} and {modified_fee_sats}"
            ),
            Self::MissingEphemeralSpends { child, missing } => write!(
                formatter,
                "transaction {child:?} did not spend all parent ephemeral dust: {missing:?}"
            ),
        }
    }
}

impl std::error::Error for EphemeralPolicyError {}

pub(crate) fn validate_ephemeral_spends<V: MempoolView>(
    view: &V,
    members: &[PreparedCandidate],
    permissions: EphemeralPolicy,
    dust_relay_fee_rate: DustRelayFeeRate,
) -> Result<(), EphemeralPolicyError> {
    for member in members {
        let has_permitted_dust =
            member.entry.transaction.outputs.iter().any(|output| {
                is_permitted_ephemeral_dust(output, permissions, dust_relay_fee_rate)
            });
        if has_permitted_dust
            && (member.fees.base.to_sats() != 0 || member.fees.modified.to_sats() != 0)
        {
            return Err(EphemeralPolicyError::DustyTransactionHasFee {
                txid: member.entry.txid,
                base_fee_sats: member.fees.base.to_sats(),
                modified_fee_sats: member.fees.modified.to_sats(),
            });
        }
    }

    let candidates = members
        .iter()
        .map(|member| (member.entry.txid, &member.entry))
        .collect::<BTreeMap<_, _>>();
    for member in members {
        validate_child_spends(
            view,
            &member.entry,
            &candidates,
            permissions,
            dust_relay_fee_rate,
        )?;
    }
    Ok(())
}

fn validate_child_spends(
    view: &impl MempoolView,
    child: &MempoolEntry,
    candidates: &BTreeMap<Txid, &MempoolEntry>,
    permissions: EphemeralPolicy,
    dust_relay_fee_rate: DustRelayFeeRate,
) -> Result<(), EphemeralPolicyError> {
    let parent_txids = child
        .transaction
        .inputs
        .iter()
        .map(|input| input.previous_output.txid)
        .collect::<BTreeSet<_>>();
    let mut missing = BTreeSet::new();
    for parent_txid in parent_txids {
        let maybe_parent = candidates
            .get(&parent_txid)
            .copied()
            .or_else(|| view.maybe_entry(&parent_txid));
        let Some(parent) = maybe_parent else {
            continue;
        };
        for (vout, output) in parent.transaction.outputs.iter().enumerate() {
            if is_permitted_ephemeral_dust(output, permissions, dust_relay_fee_rate) {
                missing.insert(OutPoint {
                    txid: parent_txid,
                    vout: u32::try_from(vout).unwrap_or(u32::MAX),
                });
            }
        }
    }
    for input in &child.transaction.inputs {
        missing.remove(&input.previous_output);
    }
    if missing.is_empty() {
        return Ok(());
    }
    Err(EphemeralPolicyError::MissingEphemeralSpends {
        child: child.txid,
        missing,
    })
}

#[cfg(test)]
mod tests;
