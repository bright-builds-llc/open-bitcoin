// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net.cpp
// - packages/bitcoin-knots/src/net.h
// - packages/bitcoin-knots/src/net_processing.cpp

//! Generic encoded buffers share the filter transport registry.

use super::{
    FilterTransportError, FilterTransportOwner, Registry,
    encoded_size::{WireSize, wire_size},
};
use open_bitcoin_core::primitives::NetworkMagic;
use open_bitcoin_network::WireNetworkMessage;
use open_bitcoin_network::{PeerId, basic_peer_resource::BASIC_PEER_MAX_AGGREGATE_FRAME_BYTES};
use std::sync::{Arc, Mutex};

#[derive(Debug)]
pub(super) struct PendingEntry {
    pub(super) peer: PeerId,
    pub(super) bytes: u64,
}

/// One immutable encoded non-CF frame. Retain this owner through queueing and writes.
/// Fields drop in declaration order: the actual Vec is freed before its charge.
/// No extraction or separate guard permits bytes to outlive their reservation.
///
/// ```compile_fail
/// fn clone_buffer(buffer: open_bitcoin_node::network::filter_transport::GenericPendingBuffer) { let _ = buffer.clone(); }
/// ```
///
/// ```compile_fail
/// fn extract_buffer(buffer: open_bitcoin_node::network::filter_transport::GenericPendingBuffer) -> Vec<u8> { buffer.bytes }
/// ```
#[derive(Debug)]
pub struct GenericPendingBuffer {
    bytes: Vec<u8>,
    _reservation: PendingReservation,
}

impl GenericPendingBuffer {
    /// Borrow the same encoded frame for the complete write; never encode it again.
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }
}

/// Private pre-allocation reservation; its owning buffer must drop first.
#[derive(Debug)]
pub(super) struct PendingReservation {
    id: u64,
    registry: Arc<Mutex<Registry>>,
}

impl PendingReservation {
    fn shrink(&mut self, bytes: u64) -> Result<(), FilterTransportError> {
        let mut registry = self
            .registry
            .lock()
            .map_err(|_| FilterTransportError::Poisoned)?;
        if registry.failed {
            return Err(FilterTransportError::Poisoned);
        }
        let entry = registry
            .generic
            .get_mut(&self.id)
            .ok_or(FilterTransportError::Foreign)?;
        if bytes > entry.bytes {
            return Err(FilterTransportError::Length);
        }
        entry.bytes = bytes;
        Ok(())
    }
}

impl Drop for PendingReservation {
    fn drop(&mut self) {
        let mut registry = match self.registry.lock() {
            Ok(registry) => registry,
            Err(poison) => {
                let mut registry = poison.into_inner();
                registry.failed = true;
                registry
            }
        };
        if registry.generic.remove(&self.id).is_none() {
            registry.failed = true;
        }
    }
}

impl FilterTransportOwner {
    pub(in crate::network) fn encode_pending(
        &self,
        peer: PeerId,
        message: &WireNetworkMessage,
        magic: NetworkMagic,
    ) -> Result<GenericPendingBuffer, FilterTransportError> {
        if matches!(
            message,
            WireNetworkMessage::CFilter(_)
                | WireNetworkMessage::CFHeaders(_)
                | WireNetworkMessage::CFCheckpt(_)
        ) {
            return Err(FilterTransportError::FilterResponseRequiresLease);
        }
        let size = wire_size(message)?;
        self.encode_reserved(peer, size, || {
            message
                .encode_wire(magic)
                .map_err(|_| FilterTransportError::Encoding)
        })
    }

    fn encode_reserved(
        &self,
        peer: PeerId,
        size: WireSize,
        encode: impl FnOnce() -> Result<Vec<u8>, FilterTransportError>,
    ) -> Result<GenericPendingBuffer, FilterTransportError> {
        let mut reservation = self.reserve_pending(peer, size.peak)?;
        let bytes = encode()?;
        if bytes.len() as u64 != size.frame {
            drop(bytes);
            return Err(FilterTransportError::Length);
        }
        // encode_wire has returned, so payload/nested/hash scratch is actually gone.
        if let Err(error) = reservation.shrink(size.frame) {
            drop(bytes);
            return Err(error);
        }
        Ok(GenericPendingBuffer {
            bytes,
            _reservation: reservation,
        })
    }

    pub(super) fn reserve_pending(
        &self,
        peer: PeerId,
        bytes: u64,
    ) -> Result<PendingReservation, FilterTransportError> {
        let mut registry = self
            .registry
            .lock()
            .map_err(|_| FilterTransportError::Poisoned)?;
        if registry.failed {
            return Err(FilterTransportError::Poisoned);
        }
        if registry
            .generic
            .values()
            .filter(|entry| entry.peer == peer)
            .count()
            >= 128
        {
            return Err(FilterTransportError::PeerPendingDescriptors);
        }
        if registry.generic.len() >= 1024 {
            return Err(FilterTransportError::AggregatePendingDescriptors);
        }
        if registry
            .pending_bytes()?
            .checked_add(bytes)
            .is_none_or(|total| total > BASIC_PEER_MAX_AGGREGATE_FRAME_BYTES)
        {
            return Err(FilterTransportError::Capacity(
                open_bitcoin_network::basic_peer_resource::BasicPeerRetentionRefusal::AggregateFrame,
            ));
        }
        let id = registry
            .next
            .checked_add(1)
            .ok_or(FilterTransportError::Exhausted)?;
        registry.next = id;
        registry.generic.insert(id, PendingEntry { peer, bytes });
        Ok(PendingReservation {
            id,
            registry: Arc::clone(&self.registry),
        })
    }
}

#[cfg(test)]
mod tests;
