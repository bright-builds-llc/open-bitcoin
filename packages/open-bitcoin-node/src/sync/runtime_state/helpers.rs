// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net.cpp
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/headerssync.cpp
// - packages/bitcoin-knots/src/sync.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp

use crate::status::FieldAvailability;

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
