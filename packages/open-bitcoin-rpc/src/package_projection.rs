// Parity breadcrumbs:
// - packages/bitcoin-knots/src/rpc/mempool.cpp

//! Pure Knots JSON projection for an authoritative [`PackageReport`].
//!
//! This module must not emit extra Open Bitcoin keys on Knots JSON. Comments
//! that name fingerprint, admission, or effective_fee_groups exist only to
//! forbid those keys on the BaselineParity result trees.

use open_bitcoin_mempool::PackageReport;
use open_bitcoin_node::core::primitives::{Txid, Wtxid};
use serde_json::Value;

#[cfg(test)]
mod tests;

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
}

/// Projects Knots `testmempoolaccept` array JSON from one package report.
pub fn project_testmempoolaccept(
    _report: &PackageReport,
    _member_facts: &[PackageMemberProjectionFacts],
) -> Result<Value, PackageProjectionError> {
    Ok(Value::Array(Vec::new()))
}
