// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/headerssync.cpp
// - packages/bitcoin-knots/src/sync.cpp
// - packages/bitcoin-knots/src/protocol.h

use open_bitcoin_core::consensus::block::check_block_header_with_params;
use open_bitcoin_core::{
    consensus::{BlockValidationError, ConsensusParams, block_hash, check_block_header_contextual},
    primitives::BlockHeader,
};
use open_bitcoin_network::{HeaderStore, NetworkError};

/// Contextual fixture validation only; the managed owner checks canonical scope first.
pub(super) fn validate_header_for_sync(
    header_store: &HeaderStore,
    header: &BlockHeader,
    current_time: i64,
    consensus_params: ConsensusParams,
) -> Result<(), NetworkError> {
    check_block_header_with_params(header, &consensus_params).map_err(map_invalid_header)?;
    if header_store.maybe_proof_facts(block_hash(header)).is_some() {
        return Ok(());
    }
    let context = header_store.header_validation_context(header, current_time, consensus_params)?;
    check_block_header_contextual(header, &context).map_err(map_invalid_header)?;
    Ok(())
}

/// The pin rejects whole-message proof/continuity faults before contextual prefix admission.
pub(super) fn validate_header_batch_proof(
    headers: &[BlockHeader],
    params: ConsensusParams,
) -> Result<(), NetworkError> {
    for header in headers {
        check_block_header_with_params(header, &params).map_err(map_invalid_header)?;
    }
    for pair in headers.windows(2) {
        if pair[1].previous_block_hash != block_hash(&pair[0]) {
            return Err(invalid_import("non-continuous-headers"));
        }
    }
    Ok(())
}

/// Re-admit actual fixture headers under explicit parameters; imported numbers earn nothing.
pub(super) fn readmit_fixture_headers(
    imported: &HeaderStore,
    current_time: i64,
    params: ConsensusParams,
) -> Result<HeaderStore, NetworkError> {
    let mut headers = HeaderStore::default();
    let mut pending: Vec<_> = imported.entries().collect();
    pending.sort_by_key(|entry| entry.height);
    for entry in pending {
        if entry.block_hash != block_hash(&entry.header) {
            return Err(invalid_import("unproved-header-identity"));
        }
        validate_header_for_sync(&headers, &entry.header, current_time, params)?;
        let inserted = headers.insert_header_with_params(entry.header.clone(), &params)?;
        if inserted.height != entry.height {
            return Err(invalid_import("unproved-header-height"));
        }
    }
    Ok(headers)
}

fn invalid_import(reason: &str) -> NetworkError {
    NetworkError::InvalidHeader {
        reject_reason: reason.to_owned(),
        maybe_debug_message: None,
    }
}

fn map_invalid_header(error: BlockValidationError) -> NetworkError {
    NetworkError::InvalidHeader {
        reject_reason: error.reject_reason.to_string(),
        maybe_debug_message: error.debug_message.clone(),
    }
}
