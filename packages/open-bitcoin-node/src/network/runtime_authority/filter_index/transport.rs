// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net.cpp
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/test/functional/p2p_blockfilters.py

//! Authority checks inspect bounded memory only; transport owns every socket wait.

use super::BasicFilterBranchCapture;
use crate::network::filter_transport::{
    FilterFrameReceipt, FilterRequestLease, FilterTransportCompletion, FilterTransportError,
    FilterTransportIdentity, FilterTransportSnapshot, GenericPendingBuffer, PreparedFilterResponse,
};
use crate::{ChainstateStore, ManagedNetworkHandle, ManagedPeerNetwork};
use open_bitcoin_core::chainstate::CoinsView;
use open_bitcoin_network::PeerId;

impl<S: ChainstateStore, V: CoinsView> ManagedNetworkHandle<S, V> {
    /// Encode one generic outbound message under the shared 8MiB pending budget.
    /// The new local payload envelope is 4,000,000 bytes plus the 24-byte frame.
    /// Retain the returned owner through queueing/backpressure and the full write.
    /// CF responses require prepared request leases and cannot use this factory.
    pub fn encode_pending_network_message(
        &self,
        peer: PeerId,
        message: &open_bitcoin_network::WireNetworkMessage,
    ) -> Result<GenericPendingBuffer, FilterTransportError> {
        let magic = self
            .lock_authority()
            .map_err(|_| FilterTransportError::AuthorityUnavailable)?
            .local_config
            .magic;
        self.basic_filter_transport
            .encode_pending(peer, message, magic)
    }

    /// Acquire one shared peer slot before any response output collection.
    #[cfg(test)]
    pub(crate) fn reserve_basic_filter_response(
        &self,
        peer: PeerId,
    ) -> Result<FilterRequestLease, FilterTransportError> {
        let network = self
            .lock_authority()
            .map_err(|_| FilterTransportError::AuthorityUnavailable)?;
        self.reserve_basic_filter_response_locked(&network, peer)
    }
    pub(super) fn reserve_basic_filter_response_locked(
        &self,
        network: &ManagedPeerNetwork<S, V>,
        peer: PeerId,
    ) -> Result<FilterRequestLease, FilterTransportError> {
        if !current_filter_grant(network, peer) {
            return Err(FilterTransportError::Invalidated);
        }
        self.basic_filter_transport.reserve(peer, 0)
    }
    /// Bind a fully authorized and guarded read to its authority and integrity epoch.
    #[cfg(test)]
    pub(in crate::network) fn bind_basic_filter_response(
        &self,
        lease: &FilterRequestLease,
        capture: &BasicFilterBranchCapture,
        integrity_epoch: crate::storage::fjall_store::filters::query::BasicFilterReadEpoch,
    ) -> Result<(), FilterTransportError> {
        let network = self
            .lock_authority()
            .map_err(|_| FilterTransportError::AuthorityUnavailable)?;
        self.bind_basic_filter_response_locked(&network, lease, capture, integrity_epoch)
    }
    pub(super) fn bind_basic_filter_response_locked(
        &self,
        network: &ManagedPeerNetwork<S, V>,
        lease: &FilterRequestLease,
        capture: &BasicFilterBranchCapture,
        integrity_epoch: crate::storage::fjall_store::filters::query::BasicFilterReadEpoch,
    ) -> Result<(), FilterTransportError> {
        let identity = FilterTransportIdentity {
            authority: capture.authority_incarnation(),
            session: capture.peer_session_generation(),
            generation: capture
                .maybe_index_generation()
                .ok_or(FilterTransportError::Invalidated)?,
            branch_anchor: capture
                .maybe_index_branch_identity()
                .ok_or(FilterTransportError::Invalidated)?,
            stop: capture.branch().stop.hash,
            integrity_epoch,
        };
        if !self.basic_filter_transport.owns(lease) || lease.peer_id() != capture.peer_id() {
            return Err(FilterTransportError::Foreign);
        }
        if !self.filter_current_or_fail(network, capture.peer_id(), &identity)? {
            return Err(FilterTransportError::Invalidated);
        }
        lease.bind(identity)
    }
    /// Must run immediately before each socket write; preparation is never write evidence.
    pub fn basic_filter_response_is_current(
        &self,
        response: &PreparedFilterResponse,
    ) -> Result<bool, FilterTransportError> {
        let network = self
            .lock_authority()
            .map_err(|_| FilterTransportError::AuthorityUnavailable)?;
        let maybe_identity = response
            .lease()
            .identity()
            .inspect_err(|_| self.basic_filter_transport.mark_failed())?;
        let current = match maybe_identity {
            Some(identity) => {
                self.filter_current_or_fail(&network, response.peer_id(), &identity)?
            }
            None => false,
        };
        if !current {
            response.lease().abort()?;
        }
        Ok(current)
    }
    /// Consume one private full-write proof; stale physical bytes cannot earn live success.
    pub fn complete_basic_filter_frame(
        &self,
        receipt: FilterFrameReceipt,
    ) -> Result<FilterTransportCompletion, FilterTransportError> {
        let network = match self.lock_authority() {
            Ok(network) => network,
            Err(_) => {
                return match self.basic_filter_transport.complete(receipt, false) {
                    Ok(_) | Err(FilterTransportError::Poisoned) => {
                        Err(FilterTransportError::AuthorityUnavailable)
                    }
                    Err(error) => Err(error),
                };
            }
        };
        let maybe_identity = match receipt.identity() {
            Ok(identity) => identity,
            Err(error) => {
                self.basic_filter_transport.mark_failed();
                return match self.basic_filter_transport.complete(receipt, false) {
                    Ok(_) | Err(FilterTransportError::Poisoned) => Err(error),
                    Err(cleanup) => Err(cleanup),
                };
            }
        };
        let current = match maybe_identity {
            Some(identity) => match is_current(&network, receipt.peer_id(), &identity) {
                Ok(current) => current,
                Err(error) => {
                    let completed = self.basic_filter_transport.complete(receipt, false);
                    self.basic_filter_transport.mark_failed();
                    return match completed {
                        Ok(_) | Err(FilterTransportError::Poisoned) => Err(error),
                        Err(cleanup) => Err(cleanup),
                    };
                }
            },
            None => false,
        };
        self.basic_filter_transport.complete(receipt, current)
    }
    /// Read live reservation charges; failed registry state is a typed error.
    pub fn basic_filter_transport_snapshot(
        &self,
    ) -> Result<FilterTransportSnapshot, FilterTransportError> {
        self.basic_filter_transport.snapshot()
    }
    fn filter_current_or_fail(
        &self,
        network: &ManagedPeerNetwork<S, V>,
        peer: PeerId,
        identity: &FilterTransportIdentity,
    ) -> Result<bool, FilterTransportError> {
        is_current(network, peer, identity)
            .inspect_err(|_| self.basic_filter_transport.mark_failed())
    }
    pub(in crate::network) fn invalidate_basic_filter_transports(
        &self,
        network: &ManagedPeerNetwork<S, V>,
    ) -> Result<(), FilterTransportError> {
        self.basic_filter_transport
            .invalidate(|peer, maybe_identity| match maybe_identity {
                Some(identity) => is_current(network, peer, &identity),
                None => Ok(current_filter_grant(network, peer)),
            })
    }
}

fn is_current<S: ChainstateStore, V: CoinsView>(
    network: &ManagedPeerNetwork<S, V>,
    peer: PeerId,
    identity: &FilterTransportIdentity,
) -> Result<bool, FilterTransportError> {
    if identity.authority != network.authority_epoch.raw()
        || identity.session != network.peer_session_generation(peer)
        || !current_filter_grant(network, peer)
    {
        return Ok(false);
    }
    let Some(progress) = network.chainstate().maybe_basic_index_progress() else {
        return Ok(false);
    };
    if progress.generation() != identity.generation {
        return Ok(false);
    }
    // Accepted tip extension preserves the old anchor. Reorg replacement does not.
    let active = network.chainstate().chainstate().active_chain();
    let anchor_active = network
        .peer_manager()
        .header_store()
        .entry(&identity.branch_anchor)
        .and_then(|header| {
            active
                .first()
                .and_then(|first| header.height.checked_sub(first.height))
        })
        .and_then(|offset| active.get(offset as usize))
        .is_some_and(|position| position.block_hash == identity.branch_anchor);
    let branch_current = progress.branch_identity() == identity.branch_anchor || anchor_active;
    if !branch_current
        || network
            .peer_manager()
            .header_store()
            .entry(&identity.stop)
            .is_none()
    {
        return Ok(false);
    }
    let Some(store) = network
        .chainstate()
        .store()
        .maybe_basic_filter_query_store()
    else {
        return Ok(false);
    };
    store
        .basic_peer_epoch_is_current(&identity.integrity_epoch)
        .map_err(|_| FilterTransportError::PublicationUnavailable)
}

fn current_filter_grant<S: ChainstateStore, V: CoinsView>(
    network: &ManagedPeerNetwork<S, V>,
    peer: PeerId,
) -> bool {
    use open_bitcoin_network::ServiceFlags;
    let Some(state) = network.peer_manager().peer_state(peer) else {
        return false;
    };
    state
        .own_advertised_services
        .contains(ServiceFlags::COMPACT_FILTERS)
        && (network
            .local_services()
            .contains(ServiceFlags::COMPACT_FILTERS)
            || state.maybe_inbound_record.as_ref().is_some_and(|record| {
                record
                    .permission_decision
                    .basic_filter_capability()
                    .effective_permission
            }))
}

#[cfg(test)]
mod tests;
