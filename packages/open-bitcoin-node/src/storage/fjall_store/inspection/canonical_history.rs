// Parity breadcrumbs:
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/txdb.cpp

//! Full copied replay occurs only inside the original supervised worker.

use super::*;
use crate::storage::{canonical_bootstrap, canonical_history::records::CanonicalHistoryRecords};
use open_bitcoin_core::consensus::CanonicalChain;

pub(super) fn validate_in_child(
    selected: CanonicalChain,
    records: &InspectedRecords,
) -> Result<(), StorageError> {
    require_existing_namespaces(records)?;
    let records = CanonicalHistoryRecords::decode(records).map_err(refused)?;
    let proof =
        canonical_bootstrap::validate_history_records(selected, &records).map_err(refused)?;
    // No earned owner or serialized authority escapes the child.
    drop(proof.into_parts(&records).map_err(refused)?);
    Ok(())
}

pub(in crate::storage::fjall_store) fn inspect_history(
    lease: &OperationLease,
    selected: CanonicalChain,
) -> Result<InspectedRecords, StorageError> {
    #[cfg(test)]
    let started = std::time::Instant::now();
    let result = inspect_owned_then(
        lease,
        |scratch| worker::recover_history(scratch, selected),
        |_, _, _| Ok(()),
        |records| {
            // The copied full proof has completed, exited and reaped. This parent
            // callback parses bounded data only, after all source/cleanup gates.
            require_existing_namespaces(&records)?;
            CanonicalHistoryRecords::decode(&records).map_err(refused)?;
            Ok(records)
        },
    );
    #[cfg(test)]
    if let Ok(records) = &result {
        let mut units = 0;
        let mut bytes = 0;
        let mut authority_max = 0;
        for name in records.namespaces() {
            units += 1;
            bytes += name.len();
            for (key, value) in records.maybe_records(name).expect("enumerated namespace") {
                units += 1;
                bytes += key.len() + value.len();
                if matches!(
                    name,
                    "schema" | "headers" | "block_index" | "chainstate" | "coins"
                ) {
                    authority_max = authority_max.max(value.len());
                }
            }
        }
        eprintln!(
            "HISTORY_INSPECTION units={units} bytes={bytes} authority_max={authority_max} child={:?} total={:?} source_gates=true",
            worker::HISTORY_OBSERVATION.get(),
            started.elapsed()
        );
    }
    result
}

fn refused<T>(_: T) -> StorageError {
    inspection_error("canonical history proof refused")
}

#[cfg(test)]
pub(in crate::storage::fjall_store) fn observation_for_test() -> (Option<u8>, std::time::Duration) {
    worker::HISTORY_OBSERVATION.get()
}
