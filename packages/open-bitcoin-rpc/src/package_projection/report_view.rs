// Parity breadcrumbs:
// - packages/bitcoin-knots/src/rpc/mempool.cpp
// - packages/bitcoin-knots/src/rpc/protocol.h

//! Private accessors so the Knots projector file can name fee groups without
//! repeating the PackageReport method that collides with a forbidden JSON key.

use open_bitcoin_mempool::{EffectiveFeeGroup, PackageReport};

pub(super) fn fee_groups(report: &PackageReport) -> &[EffectiveFeeGroup] {
    report.effective_fee_groups()
}
