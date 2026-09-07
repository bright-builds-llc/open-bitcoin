// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net.cpp
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/headerssync.cpp
// - packages/bitcoin-knots/src/sync.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp

use open_bitcoin_core::primitives::BlockHash;

use crate::status::{FieldAvailability, ProgressCreditEvidence};

use super::super::tip;

pub(super) fn maybe_available_ref<T>(field: &FieldAvailability<T>) -> Option<&T> {
    match field {
        FieldAvailability::Available(value) => Some(value),
        FieldAvailability::Unavailable { .. } => None,
    }
}

pub(super) fn progress_ratio(block_height: u64, header_height: u64) -> f64 {
    if header_height == 0 {
        return 1.0;
    }

    (block_height as f64 / header_height as f64).min(1.0)
}

pub(super) fn gate_progress_credit_on_coins_best(
    credit: FieldAvailability<ProgressCreditEvidence>,
    maybe_coins_best: Option<BlockHash>,
) -> FieldAvailability<ProgressCreditEvidence> {
    let FieldAvailability::Available(evidence) = credit else {
        return credit;
    };
    let claimed = evidence.credited_validated_active_chain_hash.as_str();
    let matches_b = maybe_coins_best.is_some_and(|best| tip::block_hash_hex(best) == claimed);
    if matches_b {
        return FieldAvailability::available(evidence);
    }
    FieldAvailability::unavailable("coins best-block does not match claimed tip")
}
