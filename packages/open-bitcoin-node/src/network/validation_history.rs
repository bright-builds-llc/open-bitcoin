// Parity breadcrumbs:
// - packages/bitcoin-knots/src/chain.h
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/headerssync.cpp

//! Trusted header observations retain coverage but grant no scripts-valid facts.

use super::*;
use crate::storage::validation_history::{
    AdmittedValidationHeaders, BlockValidationIdentity, ValidationProvenance,
};
use open_bitcoin_network::NetworkError;

pub(crate) struct TrustedHeaderAdmission {
    store: crate::FjallNodeStore,
    identities: Vec<BlockValidationIdentity>,
}

impl TrustedHeaderAdmission {
    pub(crate) fn into_parts(self) -> (crate::FjallNodeStore, Vec<BlockValidationIdentity>) {
        (self.store, self.identities)
    }
}

pub(crate) struct TrustedHeaderSnapshot {
    store: crate::FjallNodeStore,
    entries: Vec<HeaderEntry>,
}

impl TrustedHeaderSnapshot {
    pub(crate) fn maybe_capture(
        store: crate::FjallNodeStore,
        headers: &HeaderStore,
    ) -> Option<Self> {
        #[cfg(test)]
        store.note_snapshot_attempt(1);
        let entries: Vec<_> = headers.entries().cloned().collect();
        if entries.is_empty() {
            return None;
        }
        Some(Self { store, entries })
    }

    pub(crate) fn persist(&self) -> Result<(), crate::StorageError> {
        self.store.save_trusted_header_snapshot(self)
    }

    pub(crate) fn belongs_to(&self, store: &crate::FjallNodeStore) -> bool {
        self.store.shares_validation_history_store(store)
    }
    pub(crate) fn entries(&self) -> &[HeaderEntry] {
        &self.entries
    }
}

impl<S: ChainstateStore, V: CoinsView> ManagedPeerNetwork<S, V> {
    pub(super) fn handle_validated_headers(
        &mut self,
        peer_id: PeerId,
        message: HeadersMessage,
        timestamp: i64,
        params: ConsensusParams,
    ) -> Result<Vec<PeerAction>, ManagedNetworkError> {
        self.handle_validated_headers_with_policy(
            peer_id,
            message,
            timestamp,
            params,
            HeaderSyncPolicy::HeadersOnly,
        )
    }

    pub(super) fn handle_validated_headers_with_policy(
        &mut self,
        peer_id: PeerId,
        message: HeadersMessage,
        timestamp: i64,
        params: ConsensusParams,
        policy: HeaderSyncPolicy,
    ) -> Result<Vec<PeerAction>, ManagedNetworkError> {
        if message.headers.len() > open_bitcoin_network::MAX_HEADERS_RESULTS {
            return Err(open_bitcoin_core::codec::CodecError::LengthOutOfRange {
                field: "headers count",
                value: message.headers.len() as u64,
            }
            .into());
        }
        // Refuse the entire ordinary batch before peer bookkeeping or prefix publication.
        self.guard_header_batch(&message.headers)?;
        if self
            .maybe_header_consensus_params
            .is_some_and(|selected| selected != params)
        {
            return Err(NetworkError::InvalidHeader {
                reject_reason: "header-consensus-params-mismatch".to_owned(),
                maybe_debug_message: None,
            }
            .into());
        }
        super::header_sync::validate_header_batch_proof(&message.headers, params)?;
        // Initialization is an explicit fixture/qualified canonical operation, never P2P.
        if self.maybe_header_consensus_params.is_none() {
            return Err(NetworkError::InvalidHeader {
                reject_reason: "unbound-header-owner".to_owned(),
                maybe_debug_message: None,
            }
            .into());
        }
        if let Some(first) = message.headers.first()
            && self
                .peer_manager
                .header_store()
                .maybe_proof_facts(first.previous_block_hash)
                .is_none()
        {
            return Err(NetworkError::MissingHeaderAncestor(first.previous_block_hash).into());
        }
        self.peer_manager
            .peer_state(peer_id)
            .ok_or(NetworkError::UnknownPeer(peer_id))?;
        let admission_scope = self.admission_scope;
        let maybe_store = self.chainstate.store().maybe_validation_history_store();
        let mut identities = Vec::new();
        let result = self.peer_manager.handle_headers_with_policy(
            peer_id,
            message,
            policy,
            |headers, header| {
                admission_scope
                    .check_header(header)
                    .map_err(|error| error.network_error())?;
                validate_header_for_sync(headers, header, timestamp, params)?;
                let was_known = headers.entry(&block_hash(header)).is_some();
                let inserted = headers.insert_header_with_params(header.clone(), &params)?;
                if !was_known && maybe_store.is_some() {
                    let entry = headers.entry(&inserted.block_hash).ok_or(
                        open_bitcoin_network::NetworkError::MissingHeaderAncestor(
                            inserted.block_hash,
                        ),
                    )?;
                    identities.push(
                        BlockValidationIdentity::new(
                            entry.block_hash,
                            entry.header.previous_block_hash,
                            entry.height,
                        )
                        .map_err(|_| {
                            open_bitcoin_network::NetworkError::InvalidHeader {
                                reject_reason: "invalid-validation-identity".to_owned(),
                                maybe_debug_message: None,
                            }
                        })?,
                    );
                }
                Ok(inserted)
            },
        );
        // A continuous PoW-valid contextual failure retains the accepted prefix and its order.
        self.publish_admitted_header_identities(identities)?;
        result.map_err(Into::into)
    }

    pub(super) fn publish_admitted_header_identities(
        &self,
        identities: Vec<BlockValidationIdentity>,
    ) -> ManagedResult<()> {
        let maybe_store = self.chainstate.store().maybe_validation_history_store();
        if let Some(store) = maybe_store {
            let recovered = store
                .recovered_validation_history()
                .map_err(history_network_error)?;
            if recovered.belongs_to(&store)
                && recovered.is_complete().map_err(history_network_error)?
            {
                let mut known_only = Vec::with_capacity(identities.len());
                for identity in identities {
                    if self
                        .chainstate
                        .validation_provenance(identity.hash())
                        .map_err(history_network_error)?
                        == ValidationProvenance::ScriptsValid
                    {
                        let maybe_existing = store
                            .maybe_validation_history_record(identity.hash())
                            .map_err(history_network_error)?;
                        if maybe_existing.is_some_and(|record| record.identity() != identity) {
                            return Err(history_network_error(
                                crate::storage::validation_history::history_corruption(
                                    "conflicting admitted accepted header",
                                ),
                            ));
                        }
                        continue;
                    }
                    known_only.push(identity);
                }
                let admission = TrustedHeaderAdmission {
                    store: store.clone(),
                    identities: known_only,
                };
                for batch in AdmittedValidationHeaders::from_admitted(admission) {
                    store
                        .publish_admitted_validation_headers(&batch)
                        .map_err(history_network_error)?;
                }
            }
        }
        Ok(())
    }

    pub(crate) fn persist_validation_header_snapshot(&self) -> Result<(), ManagedNetworkError> {
        let Some(store) = self.chainstate.store().maybe_validation_history_store() else {
            return Ok(());
        };
        let Some(snapshot) =
            TrustedHeaderSnapshot::maybe_capture(store, self.peer_manager.header_store())
        else {
            return Ok(());
        };
        snapshot.persist().map_err(history_network_error)
    }
}

fn history_network_error(error: crate::StorageError) -> ManagedNetworkError {
    ManagedNetworkError::Chainstate(
        open_bitcoin_core::chainstate::ChainstateError::CoinsStorage {
            detail: error.to_string(),
        },
    )
}
