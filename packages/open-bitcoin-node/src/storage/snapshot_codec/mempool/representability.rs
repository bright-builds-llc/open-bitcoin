// Parity breadcrumbs:
// - packages/bitcoin-knots/src/node/mempool_persist.cpp
// - packages/bitcoin-knots/src/node/mempool_persist.h
// - packages/bitcoin-knots/test/functional/mempool_persist.py

use open_bitcoin_core::codec::{TransactionEncoding, encode_transaction};

use super::{MempoolSnapshotPersistedInputLimits, persisted_mempool_input_limits};
use crate::storage::MempoolSnapshot;
use crate::storage::mempool_snapshot::MempoolSnapshotError;

pub(crate) fn persisted_record_count_is_representable(
    record_count: usize,
    max_records: usize,
) -> bool {
    record_count <= max_records
}

pub(crate) fn assert_mempool_snapshot_representable(
    snapshot: &MempoolSnapshot,
) -> Result<MempoolSnapshotPersistedInputLimits, MempoolSnapshotError> {
    let Some(limits) = persisted_mempool_input_limits() else {
        return Err(MempoolSnapshotError::ResourceBoundExceeded);
    };
    if !persisted_record_count_is_representable(snapshot.records.len(), limits.max_records) {
        return Err(MempoolSnapshotError::ResourceBoundExceeded);
    }
    if snapshot.unbroadcast_members().len() > limits.max_unbroadcast_members {
        return Err(MempoolSnapshotError::ResourceBoundExceeded);
    }

    let mut total_transaction_bytes = 0_usize;
    let mut total_input_edges = 0_usize;
    for record in &snapshot.records {
        let encoded = encode_transaction(&record.transaction, TransactionEncoding::WithWitness)
            .map_err(|_| MempoolSnapshotError::StructuralCorruption)?;
        if encoded.len() > limits.max_transaction_bytes {
            return Err(MempoolSnapshotError::ResourceBoundExceeded);
        }
        total_transaction_bytes = total_transaction_bytes
            .checked_add(encoded.len())
            .ok_or(MempoolSnapshotError::ResourceBoundExceeded)?;
        if total_transaction_bytes > limits.max_total_transaction_bytes {
            return Err(MempoolSnapshotError::ResourceBoundExceeded);
        }

        let input_count = record.transaction.inputs.len();
        if input_count > limits.max_input_edges_per_record {
            return Err(MempoolSnapshotError::ResourceBoundExceeded);
        }
        total_input_edges = total_input_edges
            .checked_add(input_count)
            .ok_or(MempoolSnapshotError::ResourceBoundExceeded)?;
        if total_input_edges > limits.max_input_edges {
            return Err(MempoolSnapshotError::ResourceBoundExceeded);
        }
    }

    Ok(limits)
}
