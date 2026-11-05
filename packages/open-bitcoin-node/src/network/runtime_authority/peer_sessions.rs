// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/net.cpp

use super::*;
use open_bitcoin_network::{InboundAdmissionDecision, InboundAdmissionRequest, PeerState};

mod registry;
use registry::LifetimeFailure;
pub(super) use registry::PeerSessions;

/// Earned only by an actual atomic admission on this shared authority.
/// The registry identity and authority epoch bind its private admission nonce.
pub(crate) struct SyncPeerLifetime {
    registry: Arc<Mutex<PeerSessions>>,
    epoch: u64,
    peer: PeerId,
    nonce: u64,
    cleanup: Arc<registry::CleanupLease>,
}

#[derive(Debug)]
pub(crate) enum SyncPeerObservation {
    Present {
        requests: Vec<BlockHash>,
        peer: Box<PeerState>,
        compact_pending: bool,
    },
    Absent,
}

/// An actual raw dispatch and every required BASIC continuation were authorized.
#[derive(Debug)]
pub(crate) struct SyncPeerAuthorized {
    observed: SyncPeerObservation,
    result: Result<ManagedSyncMessageResult, ManagedNetworkAuthorityError>,
}

impl SyncPeerAuthorized {
    pub(crate) fn into_parts(
        self,
    ) -> (
        SyncPeerObservation,
        Result<ManagedSyncMessageResult, ManagedNetworkAuthorityError>,
    ) {
        (self.observed, self.result)
    }
}

/// An earned raw observation survives denial; this carries no cleanup grant.
#[derive(Debug)]
pub(crate) struct ContinuationDenied {
    observed: SyncPeerObservation,
    denial: Box<ManagedNetworkAuthorityError>,
    maybe_primary: Option<Box<ManagedNetworkAuthorityError>>,
}

impl ContinuationDenied {
    pub(crate) fn into_parts(
        self,
    ) -> (
        SyncPeerObservation,
        ManagedNetworkAuthorityError,
        Option<ManagedNetworkAuthorityError>,
    ) {
        (
            self.observed,
            *self.denial,
            self.maybe_primary.map(|primary| *primary),
        )
    }
}

#[derive(Debug)]
pub(crate) struct SyncPeerDispatchOutcome {
    disposition: Result<SyncPeerAuthorized, ContinuationDenied>,
}

impl SyncPeerDispatchOutcome {
    pub(crate) fn into_parts(self) -> Result<SyncPeerAuthorized, ContinuationDenied> {
        self.disposition
    }
}

struct RawSyncPeerDispatch {
    observed: SyncPeerObservation,
    result: Result<ManagedSyncMessageResult, ManagedNetworkAuthorityError>,
}

/// The real raw dispatch is resolved only with its captured origin and parameters.
/// Affine ownership is not a linear guarantee; ordinary consumers resolve immediately.
#[must_use = "resolve the actual raw dispatch before discarding its earned observation"]
pub(crate) struct SyncPeerContinuation<'origin, S, V: CoinsView> {
    handle: &'origin ManagedNetworkHandle<S, V>,
    token: &'origin SyncPeerLifetime,
    raw: RawSyncPeerDispatch,
    params: ConsensusParams,
}

impl<S: ChainstateStore, V: CoinsView> SyncPeerContinuation<'_, S, V> {
    pub(crate) fn resolve(self) -> SyncPeerDispatchOutcome {
        self.handle
            .continue_sync_peer_dispatch(self.token, self.raw, self.params)
    }
}

pub(in crate::network::runtime_authority) enum SyncContinuationFailure {
    Denied {
        denial: Box<ManagedNetworkAuthorityError>,
        maybe_primary: Option<Box<ManagedNetworkAuthorityError>>,
    },
    Operation(ManagedNetworkAuthorityError),
}

/// Synchronous request-only view of the already held original peer authority.
/// The private fields and higher-ranked callback prevent retaining its raw guard.
pub(crate) struct SyncPeerRequestAuthority<'guard, S, V: CoinsView> {
    network: &'guard mut ManagedPeerNetwork<S, V>,
    peer: PeerId,
}

impl<S: ChainstateStore, V: CoinsView> SyncPeerRequestAuthority<'_, S, V> {
    pub(crate) fn peer(&self) -> PeerId {
        self.peer
    }

    pub(crate) fn note_local(&mut self, hash: BlockHash) {
        self.network.note_local_block_hash(hash);
    }

    pub(crate) fn requested(&self) -> Result<Vec<BlockHash>, ManagedNetworkError> {
        self.network.peer_requested_blocks(self.peer)
    }

    pub(crate) fn request(
        &mut self,
        hashes: &[BlockHash],
    ) -> Result<Vec<WireNetworkMessage>, ManagedNetworkError> {
        self.network.request_missing_blocks(self.peer, hashes)
    }

    pub(crate) fn expire(
        &mut self,
        timestamp: i64,
    ) -> Result<Vec<(PeerId, WireNetworkMessage)>, ManagedNetworkError> {
        self.network.expire_compact_download_timeouts(timestamp)
    }
}

impl<S: ChainstateStore, V: CoinsView> ManagedNetworkHandle<S, V> {
    fn with_peer_sessions<T>(
        &self,
        command: impl FnOnce(
            &mut ManagedPeerNetwork<S, V>,
            &mut PeerSessions,
        ) -> Result<T, ManagedNetworkAuthorityError>,
    ) -> Result<T, ManagedNetworkAuthorityError> {
        self.mutate(|network| {
            let mut registry = self
                .peer_sessions
                .lock()
                .map_err(|_| LifetimeFailure::RegistryUnavailable)?;
            registry.prune(network);
            let result = command(network, &mut registry);
            registry.prune(network);
            result
        })?
    }

    fn check_sync_lifetime(
        &self,
        network: &ManagedPeerNetwork<S, V>,
        registry: &PeerSessions,
        token: &SyncPeerLifetime,
    ) -> Result<bool, ManagedNetworkAuthorityError> {
        if !Arc::ptr_eq(&self.peer_sessions, &token.registry) {
            return Err(LifetimeFailure::Foreign.into());
        }
        if network.authority_epoch.raw() != token.epoch {
            return Err(LifetimeFailure::EpochChanged.into());
        }
        if network.peer_manager().peer_state(token.peer).is_none() {
            return Ok(false);
        }
        if !registry.is_current(token.peer, token.nonce) {
            return Err(LifetimeFailure::Replaced.into());
        }
        Ok(true)
    }

    pub(in crate::network::runtime_authority) fn require_sync_peer_locked(
        &self,
        network: &ManagedPeerNetwork<S, V>,
        token: &SyncPeerLifetime,
    ) -> Result<PeerId, ManagedNetworkAuthorityError> {
        let registry = self
            .peer_sessions
            .lock()
            .map_err(|_| LifetimeFailure::RegistryUnavailable)?;
        if !self.check_sync_lifetime(network, &registry, token)? {
            return Err(LifetimeFailure::Absent.into());
        }
        Ok(token.peer)
    }

    fn observe_sync_lifetime(
        &self,
        network: &ManagedPeerNetwork<S, V>,
        registry: &PeerSessions,
        token: &SyncPeerLifetime,
    ) -> Result<SyncPeerObservation, ManagedNetworkAuthorityError> {
        if !self.check_sync_lifetime(network, registry, token)? {
            return Ok(SyncPeerObservation::Absent);
        }
        let manager = network.peer_manager();
        let peer = manager
            .peer_state(token.peer)
            .ok_or(LifetimeFailure::Absent)?;
        Ok(SyncPeerObservation::Present {
            requests: peer.requested_blocks.iter().copied().collect(),
            peer: Box::new(peer.clone()),
            compact_pending: manager
                .compact_download_peer_state(token.peer)
                .is_some_and(|state| !state.in_flight.is_empty()),
        })
    }

    pub(in crate::network::runtime_authority) fn prune_sync_peers_locked(
        &self,
        network: &ManagedPeerNetwork<S, V>,
    ) -> Result<(), ManagedNetworkAuthorityError> {
        let mut registry = self
            .peer_sessions
            .lock()
            .map_err(|_| LifetimeFailure::RegistryUnavailable)?;
        registry.prune(network);
        Ok(())
    }

    /// No request capture, disconnect or outbox grant: only actual absence succeeds.
    pub(crate) fn require_absent_sync_peer(
        &self,
        token: &SyncPeerLifetime,
    ) -> Result<(), ManagedNetworkAuthorityError> {
        self.with_peer_sessions(|network, registry| {
            if self.check_sync_lifetime(network, registry, token)? {
                return Err(LifetimeFailure::StillPresent.into());
            }
            Ok(())
        })
    }

    /// Present membership permits an active TAKE; legitimate original absence permits completion.
    pub(crate) fn with_sync_peer_outbox<T>(
        &self,
        token: &SyncPeerLifetime,
        command: impl FnOnce(PeerId) -> T,
    ) -> Result<Option<T>, ManagedNetworkAuthorityError> {
        let (network, present) = self.lock_sync_peer_queue(token)?;
        if !present {
            return Ok(None);
        }
        let result = command(token.peer);
        drop(network);
        Ok(Some(result))
    }

    pub(crate) fn with_sync_peer_outbox_cleanup<T>(
        &self,
        token: &SyncPeerLifetime,
        command: impl FnOnce(PeerId) -> T,
    ) -> Result<T, ManagedNetworkAuthorityError> {
        let (network, _present) = self.lock_sync_peer_queue(token)?;
        let result = command(token.peer);
        drop(network);
        Ok(result)
    }

    /// This raw guard remains private to the authority module, never a Sync projection.
    fn lock_sync_peer_queue(
        &self,
        token: &SyncPeerLifetime,
    ) -> Result<
        (std::sync::MutexGuard<'_, ManagedPeerNetwork<S, V>>, bool),
        ManagedNetworkAuthorityError,
    > {
        let network = self.lock_authority()?;
        let present = {
            let mut registry = self
                .peer_sessions
                .lock()
                .map_err(|_| LifetimeFailure::RegistryUnavailable)?;
            registry.prune(&network);
            let present = self.check_sync_lifetime(&network, &registry, token)?;
            registry.require_cleanup(token.peer, &token.cleanup)?;
            present
        };
        Ok((network, present))
    }

    pub fn admit_inbound_peer(
        &self,
        request: InboundAdmissionRequest,
    ) -> Result<InboundAdmissionDecision, ManagedNetworkAuthorityError> {
        self.with_peer_sessions(|network, registry| {
            registry
                .admit(network, request.peer_id, |network| {
                    Ok(network.admit_inbound_peer(request))
                })?
                .0
        })
    }

    pub fn add_inbound_peer(&self, peer_id: PeerId) -> Result<(), ManagedNetworkAuthorityError> {
        self.with_peer_sessions(|network, registry| {
            registry
                .admit(network, peer_id, |network| {
                    network.add_inbound_peer(peer_id)
                })?
                .0
        })
    }

    pub fn connect_outbound_peer(
        &self,
        peer_id: PeerId,
        timestamp: i64,
    ) -> Result<Vec<WireNetworkMessage>, ManagedNetworkAuthorityError> {
        self.with_peer_sessions(|network, registry| {
            registry
                .admit(network, peer_id, |network| {
                    network.connect_outbound_peer(peer_id, timestamp)
                })?
                .0
        })
    }

    pub(crate) fn connect_sync_peer(
        &self,
        peer: PeerId,
        timestamp: i64,
    ) -> Result<
        (
            SyncPeerLifetime,
            Result<Vec<WireNetworkMessage>, ManagedNetworkAuthorityError>,
        ),
        ManagedNetworkAuthorityError,
    > {
        self.with_peer_sessions(|network, registry| {
            let (result, maybe_nonce) = registry.admit(network, peer, |network| {
                network.connect_outbound_peer(peer, timestamp)
            })?;
            let Some(nonce) = maybe_nonce else {
                return Err(result
                    .err()
                    .unwrap_or_else(|| LifetimeFailure::Absent.into()));
            };
            let token = SyncPeerLifetime {
                registry: Arc::clone(&self.peer_sessions),
                epoch: network.authority_epoch.raw(),
                peer,
                nonce,
                cleanup: registry.install_cleanup(peer),
            };
            Ok((token, result))
        })
    }

    pub(crate) fn observe_sync_peer(
        &self,
        token: &SyncPeerLifetime,
    ) -> Result<SyncPeerObservation, ManagedNetworkAuthorityError> {
        self.with_peer_sessions(|network, registry| {
            self.observe_sync_lifetime(network, registry, token)
        })
    }

    pub(crate) fn finish_sync_peer(
        &self,
        token: &SyncPeerLifetime,
    ) -> Result<SyncPeerObservation, ManagedNetworkAuthorityError> {
        self.with_peer_sessions(|network, registry| {
            let observed = self.observe_sync_lifetime(network, registry, token)?;
            if matches!(observed, SyncPeerObservation::Present { .. }) {
                network.disconnect_peer(token.peer)?;
            }
            Ok(observed)
        })
    }

    fn produce_sync_peer_dispatch(
        &self,
        token: &SyncPeerLifetime,
        message: WireNetworkMessage,
        timestamp: i64,
        verify_flags: ScriptVerifyFlags,
        consensus_params: ConsensusParams,
    ) -> Result<RawSyncPeerDispatch, ManagedNetworkAuthorityError> {
        self.with_peer_sessions(|network, registry| {
            let observed = self.observe_sync_lifetime(network, registry, token)?;
            if matches!(observed, SyncPeerObservation::Absent) {
                return Err(LifetimeFailure::Absent.into());
            }
            let result = network
                .receive_sync_message(
                    token.peer,
                    message,
                    timestamp,
                    verify_flags,
                    consensus_params,
                )
                .map_err(Into::into);
            Ok(RawSyncPeerDispatch { observed, result })
        })
    }

    fn continue_sync_peer_dispatch(
        &self,
        token: &SyncPeerLifetime,
        raw: RawSyncPeerDispatch,
        params: ConsensusParams,
    ) -> SyncPeerDispatchOutcome {
        let RawSyncPeerDispatch { observed, result } = raw;
        let result = match result {
            // No BASIC continuation exists on a genuine raw protocol error.
            Err(primary) => Err(primary),
            Ok(result) => match self.resolve_basic_filter_intents_for_sync(token, result, params) {
                Ok(result) => Ok(result),
                Err(SyncContinuationFailure::Operation(error)) => Err(error),
                Err(SyncContinuationFailure::Denied {
                    denial,
                    maybe_primary,
                }) => {
                    return SyncPeerDispatchOutcome {
                        disposition: Err(ContinuationDenied {
                            observed,
                            denial,
                            maybe_primary,
                        }),
                    };
                }
            },
        };
        SyncPeerDispatchOutcome {
            disposition: Ok(SyncPeerAuthorized { observed, result }),
        }
    }

    pub(crate) fn begin_sync_peer_dispatch<'origin>(
        &'origin self,
        token: &'origin SyncPeerLifetime,
        message: WireNetworkMessage,
        timestamp: i64,
        verify_flags: ScriptVerifyFlags,
        consensus_params: ConsensusParams,
    ) -> Result<SyncPeerContinuation<'origin, S, V>, ManagedNetworkAuthorityError> {
        let raw = self.produce_sync_peer_dispatch(
            token,
            message,
            timestamp,
            verify_flags,
            consensus_params,
        )?;
        Ok(SyncPeerContinuation {
            handle: self,
            token,
            raw,
            params: consensus_params,
        })
    }

    pub(crate) fn with_sync_peer_requests<T>(
        &self,
        token: &SyncPeerLifetime,
        command: impl for<'guard> FnOnce(SyncPeerRequestAuthority<'guard, S, V>) -> T,
    ) -> Result<T, ManagedNetworkAuthorityError> {
        self.with_peer_sessions(|network, registry| {
            if matches!(
                self.observe_sync_lifetime(network, registry, token)?,
                SyncPeerObservation::Absent
            ) {
                return Err(LifetimeFailure::Absent.into());
            }
            Ok(command(SyncPeerRequestAuthority {
                network,
                peer: token.peer,
            }))
        })
    }

    pub fn receive_sync_message(
        &self,
        peer_id: PeerId,
        message: WireNetworkMessage,
        timestamp: i64,
        verify_flags: ScriptVerifyFlags,
        consensus_params: ConsensusParams,
    ) -> Result<ManagedSyncMessageResult, ManagedNetworkAuthorityError> {
        let result = self.try_mutate(|network| {
            network.receive_sync_message(
                peer_id,
                message,
                timestamp,
                verify_flags,
                consensus_params,
            )
        })?;
        self.resolve_basic_filter_intents(peer_id, result, consensus_params)
    }
}

impl<S: ChainstateStore, V: CoinsView> ManagedPeerNetwork<S, V> {
    pub fn connect_outbound_peer(
        &mut self,
        peer_id: PeerId,
        timestamp: i64,
    ) -> Result<Vec<WireNetworkMessage>, ManagedNetworkError> {
        let next_session_generation = self.next_peer_session_generation(peer_id)?;
        let actions = self.peer_manager.add_outbound_peer(peer_id, timestamp)?;
        self.peer_session_generations
            .insert(peer_id, next_session_generation);
        self.known_peers.insert(peer_id);
        self.collect_outbound(actions)
    }
}

#[cfg(test)]
fn memory_handle() -> MemoryNetworkHandle {
    ManagedNetworkHandle::new(ManagedPeerNetwork::new(
        MemoryChainstateStore::default(),
        open_bitcoin_network::LocalPeerConfig::default(),
        open_bitcoin_mempool::PolicyConfig::default(),
    ))
}

#[cfg(test)]
mod basic_tests;
