// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net.cpp
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/test/functional/p2p_blockfilters.py

//! One bounded request owns its suffix independently of the generic effect queue.

use super::lifecycle_effects::PeerSessionGeneration;
use open_bitcoin_core::{
    chainstate::filter_index::lifecycle::IndexGeneration, primitives::BlockHash,
};
use open_bitcoin_network::{
    PeerId,
    basic_peer_resource::{
        BasicPeerRetentionRefusal, admit_active_filter_frame, admit_response_reservation,
    },
};
use std::{
    collections::{BTreeMap, VecDeque},
    sync::{Arc, Mutex},
};

/// Resource and ownership failures contain no backend detail.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FilterTransportError {
    Capacity(BasicPeerRetentionRefusal),
    Poisoned,
    AuthorityUnavailable,
    PublicationUnavailable,
    Exhausted,
    InvalidFrames,
    Invalidated,
    Foreign,
    Sequence,
    Length,
    PeerPendingDescriptors,
    AggregatePendingDescriptors,
    LocalPayloadLimit,
    InvalidMessage,
    CompactSizeTooLarge,
    Encoding,
    FilterResponseRequiresLease,
}
impl std::fmt::Display for FilterTransportError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BASIC transport: {self:?}")
    }
}
impl std::error::Error for FilterTransportError {}

/// Physical completed frames remain factual after authority is invalidated.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct FilterWrittenPrefix {
    pub messages: u64,
    pub bytes: u64,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Terminal authority and factual complete-frame prefix have separate outcomes.
pub enum FilterTransportDisposition {
    WrittenPrefix,
    CurrentRangeComplete,
    PhysicalPrefixInvalidated,
    Aborted,
    DuplicateOrForeign,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Consumed transport evidence, including full framed bytes physically achieved.
pub struct FilterTransportCompletion {
    pub disposition: FilterTransportDisposition,
    pub progress: FilterWrittenPrefix,
}
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
/// Live charges include a cancelled request while transport still owns its handed frame.
pub struct FilterTransportSnapshot {
    pub requests: u64,
    pub suffix_bytes: u64,
    pub active_frame_bytes: u64,
    pub generic_pending_bytes: u64,
    pub generic_descriptors: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::network) struct FilterTransportIdentity {
    pub authority: u64,
    pub session: PeerSessionGeneration,
    pub generation: IndexGeneration,
    pub branch_anchor: BlockHash,
    pub stop: BlockHash,
    pub integrity_epoch: crate::storage::fjall_store::filters::query::BasicFilterReadEpoch,
}
#[derive(Debug, Default)]
struct Proof {
    progress: FilterWrittenPrefix,
    maybe_terminal: Option<FilterTransportDisposition>,
}
#[derive(Debug)]
struct Entry {
    peer: PeerId,
    suffix: u64,
    maybe_identity: Option<FilterTransportIdentity>,
    maybe_expected_frames: Option<u64>,
    next: u64,
    maybe_frame: Option<u64>,
    proof: Arc<Mutex<Proof>>,
    frames: Arc<Mutex<VecDeque<Vec<u8>>>>,
}
mod pending;
mod registry;
pub use pending::GenericPendingBuffer;
mod encoded_size;
pub(in crate::network) use registry::FilterTransportOwner;
use registry::Registry;
/// Sanitized local outcomes carry no backend details and authorize no wire message.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BasicFilterServeUnavailable {
    NoBackend,
    Missing,
    ResourceRefused,
    StorageFailure,
    Invalidated,
}

/// Parsing intent is distinct from a complete affine response and protocol rejection.
#[derive(Debug, PartialEq, Eq)]
pub enum BasicFilterPrepareOutcome {
    InvalidDisconnect(open_bitcoin_network::policy::FilterRequestInvalid),
    Unavailable(BasicFilterServeUnavailable),
    Prepared(Box<PreparedFilterResponse>),
}
#[derive(Debug)]
struct Lease {
    id: u64,
    peer: PeerId,
    registry: Arc<Mutex<Registry>>,
    proof: Arc<Mutex<Proof>>,
    frames: Arc<Mutex<VecDeque<Vec<u8>>>>,
}
impl Lease {
    fn maybe_identity(&self) -> Result<Option<FilterTransportIdentity>, FilterTransportError> {
        let registry = self
            .registry
            .lock()
            .map_err(|_| FilterTransportError::Poisoned)?;
        let Some(entry) = registry.active.get(&self.id) else {
            return Ok(None);
        };
        if entry
            .proof
            .lock()
            .map_err(|_| FilterTransportError::Poisoned)?
            .maybe_terminal
            .is_some()
        {
            return Ok(None);
        }
        Ok(entry.maybe_identity.clone())
    }
    fn abort(&self) -> Result<(), FilterTransportError> {
        match self.registry.lock() {
            Ok(mut registry) => {
                registry.retire(self.id, FilterTransportDisposition::Aborted);
                if registry.failed {
                    return Err(FilterTransportError::Poisoned);
                }
                Ok(())
            }
            Err(poison) => {
                let mut registry = poison.into_inner();
                registry.failed = true;
                registry.retire(self.id, FilterTransportDisposition::Aborted);
                Err(FilterTransportError::Poisoned)
            }
        }
    }
    fn progress(&self) -> Result<FilterWrittenPrefix, FilterTransportError> {
        Ok(self
            .proof
            .lock()
            .map_err(|_| FilterTransportError::Poisoned)?
            .progress)
    }
    fn release_frame(&self) {
        let mut registry = self
            .registry
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        if let Some(entry) = registry.active.get_mut(&self.id) {
            entry.maybe_frame = None;
        }
        registry.retire(self.id, FilterTransportDisposition::Aborted);
    }
    fn settle_dropped_receipt(&self, sequence: u64, bytes: u64) {
        let mut registry = self
            .registry
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        let valid = registry
            .active
            .get(&self.id)
            .is_some_and(|entry| entry.next == sequence && entry.maybe_frame == Some(bytes));
        if valid {
            let mut proof = match self.proof.lock() {
                Ok(proof) => proof,
                Err(poison) => {
                    registry.failed = true;
                    poison.into_inner()
                }
            };
            if proof.progress.messages == sequence {
                proof.progress.messages += 1;
                match proof.progress.bytes.checked_add(bytes) {
                    Some(total) => proof.progress.bytes = total,
                    None => registry.failed = true,
                }
            }
        }
        if let Some(entry) = registry.active.get_mut(&self.id) {
            entry.maybe_frame = None;
        }
        registry.retire(
            self.id,
            FilterTransportDisposition::PhysicalPrefixInvalidated,
        );
    }
}
impl Drop for Lease {
    fn drop(&mut self) {
        if let Err(_error) = self.abort() { /* Registry remains explicitly failed for the next observer. */
        }
    }
}

/// An affine lease: drop frees queued suffix; handed frames retain their charge until settlement.
///
/// ```compile_fail
/// fn clone_lease(lease: open_bitcoin_node::network::filter_transport::FilterRequestLease) { let _ = lease.clone(); }
/// ```
#[derive(Debug)]
pub struct FilterRequestLease {
    inner: Arc<Lease>,
}
impl FilterRequestLease {
    pub(in crate::network) fn peer_id(&self) -> PeerId {
        self.inner.peer
    }
    pub(in crate::network) fn abort(&self) -> Result<(), FilterTransportError> {
        self.inner.abort()
    }
    pub(in crate::network) fn bind(
        &self,
        identity: FilterTransportIdentity,
    ) -> Result<(), FilterTransportError> {
        let mut registry = self
            .inner
            .registry
            .lock()
            .map_err(|_| FilterTransportError::Poisoned)?;
        let entry = registry
            .active
            .get_mut(&self.inner.id)
            .ok_or(FilterTransportError::Invalidated)?;
        if entry
            .proof
            .lock()
            .map_err(|_| FilterTransportError::Poisoned)?
            .maybe_terminal
            .is_some()
        {
            return Err(FilterTransportError::Invalidated);
        }
        if entry.maybe_identity.is_some() {
            return Err(FilterTransportError::Foreign);
        }
        entry.maybe_identity = Some(identity);
        Ok(())
    }
    pub(in crate::network) fn identity(
        &self,
    ) -> Result<Option<FilterTransportIdentity>, FilterTransportError> {
        self.inner.maybe_identity()
    }
    /// Grow admission before allocating/encoding output; all bytes are full frame lengths.
    pub(in crate::network) fn resize_suffix(
        &self,
        suffix: u64,
    ) -> Result<(), FilterTransportError> {
        let mut registry = self
            .inner
            .registry
            .lock()
            .map_err(|_| FilterTransportError::Poisoned)?;
        let entry = registry
            .active
            .get(&self.inner.id)
            .ok_or(FilterTransportError::Invalidated)?;
        if entry.maybe_expected_frames.is_some() {
            return Err(FilterTransportError::InvalidFrames);
        }
        let other = registry.snapshot().suffix_bytes - entry.suffix;
        admit_response_reservation(0, 0, 0, other, suffix)
            .map_err(FilterTransportError::Capacity)?;
        registry
            .active
            .get_mut(&self.inner.id)
            .ok_or(FilterTransportError::Invalidated)?
            .suffix = suffix;
        Ok(())
    }
    pub(in crate::network) fn prepare(
        self,
        frames: Vec<Vec<u8>>,
    ) -> Result<PreparedFilterResponse, FilterTransportError> {
        let total = frames
            .iter()
            .try_fold(0u64, |total, f| total.checked_add(f.len() as u64))
            .ok_or(FilterTransportError::Exhausted)?;
        if frames.is_empty() || frames.len() > 1000 || frames.iter().any(|f| f.len() < 24) {
            return Err(FilterTransportError::InvalidFrames);
        }
        let mut registry = self
            .inner
            .registry
            .lock()
            .map_err(|_| FilterTransportError::Poisoned)?;
        let entry = registry
            .active
            .get_mut(&self.inner.id)
            .ok_or(FilterTransportError::Invalidated)?;
        if total != entry.suffix || entry.maybe_expected_frames.is_some() {
            return Err(FilterTransportError::Length);
        }
        entry.maybe_expected_frames = Some(frames.len() as u64);
        *self
            .inner
            .frames
            .lock()
            .map_err(|_| FilterTransportError::Poisoned)? = frames.into();
        drop(registry);
        Ok(PreparedFilterResponse { lease: self })
    }
    /// Revoke preparation and release queued buffers, preserving any handed frame's charge.
    pub fn cancel(self) -> Result<(), FilterTransportError> {
        self.inner.abort()
    }
}
impl Drop for FilterRequestLease {
    fn drop(&mut self) {
        if let Err(_error) = self.inner.abort() { /* Observable poisoned registry is retained. */ }
    }
}

/// Immutable bounded frames; no write has been achieved by preparation.
#[derive(Debug)]
pub struct PreparedFilterResponse {
    lease: FilterRequestLease,
}
impl PartialEq for PreparedFilterResponse {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.lease.inner, &other.lease.inner)
    }
}
impl Eq for PreparedFilterResponse {}
impl PreparedFilterResponse {
    /// The exact peer captured when this request acquired its admission slot.
    pub fn peer_id(&self) -> PeerId {
        self.lease.inner.peer
    }
    /// Factual complete-frame prefix, even after cancellation or authority invalidation.
    pub fn progress(&self) -> Result<FilterWrittenPrefix, FilterTransportError> {
        self.lease.inner.progress()
    }
    pub(in crate::network) fn lease(&self) -> &FilterRequestLease {
        &self.lease
    }
    /// Admit exactly one frame, including all other pending transport pressure.
    pub fn next_frame(&mut self) -> Result<Option<PreparedFilterFrame>, FilterTransportError> {
        let mut registry = self
            .lease
            .inner
            .registry
            .lock()
            .map_err(|_| FilterTransportError::Poisoned)?;
        if registry.failed {
            return Err(FilterTransportError::Poisoned);
        }
        match self
            .lease
            .inner
            .proof
            .lock()
            .map_err(|_| FilterTransportError::Poisoned)?
            .maybe_terminal
        {
            Some(FilterTransportDisposition::CurrentRangeComplete) => return Ok(None),
            Some(_) => return Err(FilterTransportError::Invalidated),
            None => {}
        }
        let pending = registry.pending_bytes()?;
        let peer_active = registry
            .active
            .values()
            .filter(|entry| entry.peer == self.peer_id())
            .filter_map(|entry| entry.maybe_frame)
            .try_fold(0u64, |total, bytes| total.checked_add(bytes))
            .ok_or(FilterTransportError::Exhausted)?;
        let entry = registry
            .active
            .get_mut(&self.lease.inner.id)
            .ok_or(FilterTransportError::Invalidated)?;
        let mut frames = self
            .lease
            .inner
            .frames
            .lock()
            .map_err(|_| FilterTransportError::Poisoned)?;
        let Some(bytes) = frames.front() else {
            return Ok(None);
        };
        if entry.maybe_frame.is_some() {
            return Err(FilterTransportError::Sequence);
        }
        admit_active_filter_frame(bytes.len() as u64, peer_active, pending)
            .map_err(FilterTransportError::Capacity)?;
        let frame_bytes = bytes.len() as u64;
        let bytes = frames
            .pop_front()
            .ok_or(FilterTransportError::InvalidFrames)?;
        entry.maybe_frame = Some(frame_bytes);
        let capability = FilterFrameWriteCapability {
            lease: Arc::clone(&self.lease.inner),
            sequence: entry.next,
            bytes: frame_bytes,
            consumed: false,
        };
        Ok(Some(PreparedFilterFrame { bytes, capability }))
    }
    /// Drop every queued frame; a handed frame remains charged until its owner settles.
    pub fn cancel(self) -> Result<(), FilterTransportError> {
        self.lease.inner.abort()
    }
}
#[derive(Debug)]
pub struct PreparedFilterFrame {
    bytes: Vec<u8>,
    capability: FilterFrameWriteCapability,
}
impl PreparedFilterFrame {
    /// Borrow the immutable full Bitcoin frame, including its 24-byte wire header.
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }
    #[cfg(test)]
    fn into_parts(self) -> (Vec<u8>, FilterFrameWriteCapability) {
        (self.bytes, self.capability)
    }
    /// Trusted transport consumes this only after successfully writing all `bytes()`.
    pub fn acknowledge_write(self) -> FilterFrameReceipt {
        self.capability.acknowledge_write()
    }
    #[cfg(test)]
    pub(in crate::network) fn into_written_for_test(self) -> FilterFrameReceipt {
        let (bytes, capability) = self.into_parts();
        capability.written(bytes.len() as u64).expect("exact write")
    }
}
/// Private request-bound capability; trusted first-party adapters acknowledge full writes.
///
/// ```compile_fail
/// fn clone_cap(cap: open_bitcoin_node::network::filter_transport::FilterFrameWriteCapability) { let _ = cap.clone(); }
/// ```
#[derive(Debug)]
pub struct FilterFrameWriteCapability {
    lease: Arc<Lease>,
    sequence: u64,
    bytes: u64,
    consumed: bool,
}
impl FilterFrameWriteCapability {
    /// Trusted first-party transport calls this only after its complete frame write.
    /// The bound length is private; partial/failed writes must drop the capability.
    pub fn acknowledge_write(mut self) -> FilterFrameReceipt {
        self.consumed = true;
        FilterFrameReceipt {
            lease: Arc::clone(&self.lease),
            sequence: self.sequence,
            bytes: self.bytes,
            consumed: false,
        }
    }
    #[cfg(test)]
    fn written(self, bytes: u64) -> Result<FilterFrameReceipt, FilterTransportError> {
        if bytes != self.bytes {
            return Err(FilterTransportError::Length);
        }
        Ok(self.acknowledge_write())
    }
}
impl Drop for FilterFrameWriteCapability {
    fn drop(&mut self) {
        if !self.consumed {
            self.lease.release_frame();
        }
    }
}
/// Non-Clone achieved-write proof, consumed by its originating shared authority.
///
/// ```compile_fail
/// fn forge() { let _ = open_bitcoin_node::network::filter_transport::FilterFrameReceipt {}; }
/// ```
///
/// ```compile_fail
/// fn clone_receipt(receipt: open_bitcoin_node::network::filter_transport::FilterFrameReceipt) { let _ = receipt.clone(); }
/// ```
///
/// ```compile_fail
/// fn deserialize_receipt() -> Result<open_bitcoin_node::network::filter_transport::FilterFrameReceipt, serde_json::Error> { serde_json::from_str("{}") }
/// ```
#[derive(Debug)]
pub struct FilterFrameReceipt {
    lease: Arc<Lease>,
    sequence: u64,
    bytes: u64,
    consumed: bool,
}
impl FilterFrameReceipt {
    pub(in crate::network) fn peer_id(&self) -> PeerId {
        self.lease.peer
    }
    pub(in crate::network) fn identity(
        &self,
    ) -> Result<Option<FilterTransportIdentity>, FilterTransportError> {
        self.lease.maybe_identity()
    }
}
impl Drop for FilterFrameReceipt {
    fn drop(&mut self) {
        if !self.consumed {
            self.lease.settle_dropped_receipt(self.sequence, self.bytes);
        }
    }
}

#[cfg(test)]
mod tests;
