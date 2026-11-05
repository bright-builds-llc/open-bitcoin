// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net.cpp
// - packages/bitcoin-knots/src/net_processing.cpp

//! Bounded announcement queues and registration identity; no transport under a queue lock.

use super::super::{SyncRuntimeError, block_reconcile::PeerRequests};
use crate::network::{AnnouncementPreparationOutcome, PeerEmission, PeerOutboxSnapshot};
use open_bitcoin_network::{
    PHASE94_MAX_AGGREGATE_QUEUED_MESSAGES, PHASE94_MAX_PEER_QUEUED_MESSAGES, PeerId,
};
use std::{
    collections::{BTreeMap, VecDeque},
    future::poll_fn,
    sync::{
        Arc, Mutex,
        atomic::{AtomicU64, Ordering},
    },
    task::{Poll, Waker},
};

pub(super) struct AnnouncementOutbox {
    pub(super) emissions: VecDeque<PeerEmission>,
    pub(super) readiness: Arc<AnnouncementOutboxReadiness>,
}

impl Default for AnnouncementOutbox {
    fn default() -> Self {
        Self {
            emissions: VecDeque::new(),
            readiness: Arc::new(AnnouncementOutboxReadiness::default()),
        }
    }
}

#[derive(Default)]
pub(super) struct AnnouncementOutboxReadiness {
    pub(super) generation: AtomicU64,
    pub(super) maybe_waker: Mutex<Option<Waker>>,
}

impl AnnouncementOutboxReadiness {
    pub(super) fn notify(&self) {
        self.generation.fetch_add(1, Ordering::Release);
        let maybe_waker = self
            .maybe_waker
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .take();
        if let Some(waker) = maybe_waker {
            waker.wake();
        }
    }
}

/// A cancellation-safe readiness cursor for one live peer announcement outbox.
pub struct AnnouncementOutboxNotification {
    pub(super) readiness: Arc<AnnouncementOutboxReadiness>,
    pub(super) observed_generation: u64,
}

impl AnnouncementOutboxNotification {
    /// Waits until at least one enqueue occurred after the last observed generation.
    pub async fn notified(&mut self) {
        poll_fn(|context| {
            let generation = self.readiness.generation.load(Ordering::Acquire);
            if generation != self.observed_generation {
                self.observed_generation = generation;
                return Poll::Ready(());
            }

            let mut maybe_waker = self
                .readiness
                .maybe_waker
                .lock()
                .unwrap_or_else(|poisoned| poisoned.into_inner());
            let generation = self.readiness.generation.load(Ordering::Acquire);
            if generation != self.observed_generation {
                self.observed_generation = generation;
                return Poll::Ready(());
            }
            *maybe_waker = Some(context.waker().clone());
            Poll::Pending
        })
        .await
    }
}

#[derive(Clone, Default)]
pub struct AnnouncementOutboxRegistry {
    pub(super) outboxes: Arc<Mutex<BTreeMap<PeerId, AnnouncementOutbox>>>,
}

impl AnnouncementOutboxRegistry {
    /// Makes one live peer eligible to receive prepared announcement emissions.
    pub fn register_peer(
        &self,
        peer_id: PeerId,
    ) -> Result<AnnouncementOutboxNotification, SyncRuntimeError> {
        let mut outboxes = self.lock_outboxes()?;
        if outboxes.contains_key(&peer_id) {
            return Err(SyncRuntimeError::Network {
                message: format!("announcement outbox peer {peer_id} is already registered"),
            });
        }
        let outbox = outboxes.entry(peer_id).or_default();
        Ok(AnnouncementOutboxNotification {
            observed_generation: outbox.readiness.generation.load(Ordering::Acquire),
            readiness: Arc::clone(&outbox.readiness),
        })
    }

    /// Removes one peer and aborts its bounded volatile announcement queue.
    pub fn unregister_peer<S, V>(
        &self,
        network: &crate::ManagedNetworkHandle<S, V>,
        peer_id: PeerId,
    ) -> Result<(), SyncRuntimeError>
    where
        S: crate::ChainstateStore,
        V: open_bitcoin_core::chainstate::CoinsView,
    {
        self.unregister_registration(network, peer_id, None)
    }

    /// Captures queue pressure without retaining the registry lock during preparation.
    pub fn snapshots(&self) -> Result<Vec<PeerOutboxSnapshot>, SyncRuntimeError> {
        Ok(self
            .lock_outboxes()?
            .iter()
            .map(|(peer_id, outbox)| {
                PeerOutboxSnapshot::new(
                    *peer_id,
                    outbox.emissions.len(),
                    PHASE94_MAX_PEER_QUEUED_MESSAGES,
                )
            })
            .collect())
    }

    /// Enqueues prepared emissions while preserving per-peer and aggregate bounds.
    pub fn enqueue_prepared<S, V>(
        &self,
        network: &crate::ManagedNetworkHandle<S, V>,
        outcomes: Vec<AnnouncementPreparationOutcome>,
    ) -> Result<(), SyncRuntimeError>
    where
        S: crate::ChainstateStore,
        V: open_bitcoin_core::chainstate::CoinsView,
    {
        let mut outboxes = match self.lock_outboxes() {
            Ok(outboxes) => outboxes,
            Err(error) => {
                return super::emission_terminal::abort_outcomes(network, outcomes).and(Err(error));
            }
        };
        let mut aggregate_queued = outboxes
            .values()
            .map(|outbox| outbox.emissions.len())
            .sum::<usize>();
        let mut readiness_notifications = Vec::new();
        let mut aborted_emissions = Vec::new();
        for outcome in outcomes {
            let AnnouncementPreparationOutcome::Ready(emission) = outcome else {
                continue;
            };
            if aggregate_queued >= PHASE94_MAX_AGGREGATE_QUEUED_MESSAGES {
                aborted_emissions.push(*emission);
                continue;
            }
            let Some(outbox) = outboxes.get_mut(&emission.peer_id()) else {
                aborted_emissions.push(*emission);
                continue;
            };
            if outbox.emissions.len() >= PHASE94_MAX_PEER_QUEUED_MESSAGES {
                aborted_emissions.push(*emission);
                continue;
            }
            outbox.emissions.push_back(*emission);
            readiness_notifications.push(Arc::clone(&outbox.readiness));
            aggregate_queued = aggregate_queued.saturating_add(1);
        }
        drop(outboxes);
        for readiness in readiness_notifications {
            readiness.notify();
        }
        super::emission_terminal::abort_emissions(network, aborted_emissions)
    }

    pub(super) fn lock_outboxes(
        &self,
    ) -> Result<std::sync::MutexGuard<'_, BTreeMap<PeerId, AnnouncementOutbox>>, SyncRuntimeError>
    {
        self.outboxes
            .lock()
            .map_err(|_error| SyncRuntimeError::Network {
                message: "announcement outbox registry is unavailable".to_string(),
            })
    }
}

impl AnnouncementOutboxRegistry {
    pub(super) fn unregister_registration<
        S: crate::ChainstateStore,
        V: open_bitcoin_core::chainstate::CoinsView,
    >(
        &self,
        network: &crate::ManagedNetworkHandle<S, V>,
        peer_id: PeerId,
        maybe_registration: Option<&AnnouncementOutboxNotification>,
    ) -> Result<(), SyncRuntimeError> {
        let maybe_outbox = {
            let mut outboxes = self.lock_outboxes()?;
            if let Some(registration) = maybe_registration
                && let Some(current) = outboxes.get(&peer_id)
                && !Arc::ptr_eq(&registration.readiness, &current.readiness)
            {
                return Err(SyncRuntimeError::Network {
                    message: "announcement outbox registration was replaced".to_owned(),
                });
            }
            outboxes.remove(&peer_id)
        };
        let emissions = maybe_outbox
            .map(|outbox| outbox.emissions)
            .unwrap_or_default();
        super::emission_terminal::abort_emissions(network, emissions)
    }

    pub(super) fn finish_registration(
        &self,
        network: &crate::DurableNetworkHandle,
        peer_id: PeerId,
        registration: &AnnouncementOutboxNotification,
        maybe_requests: Option<&PeerRequests>,
    ) -> Result<(), SyncRuntimeError> {
        let emissions = match maybe_requests {
            Some(requests) => requests.with_outbox_cleanup(network, |owned_peer| {
                self.take_registration(owned_peer, registration, true, false)
            })??,
            // Failed admission earned no lease: only this matching empty queue is removable.
            None => self.take_registration(peer_id, registration, true, true)?,
        };
        super::emission_terminal::abort_emissions(network, emissions)
    }

    /// Takes the current FIFO batch for exactly one peer.
    pub fn take_peer_emissions(
        &self,
        peer_id: PeerId,
    ) -> Result<Vec<PeerEmission>, SyncRuntimeError> {
        let mut outboxes = self.lock_outboxes()?;
        let Some(outbox) = outboxes.get_mut(&peer_id) else {
            return Ok(Vec::new());
        };
        Ok(outbox.emissions.drain(..).collect())
    }

    pub(super) fn take_registration(
        &self,
        peer_id: PeerId,
        registration: &AnnouncementOutboxNotification,
        remove: bool,
        empty_only: bool,
    ) -> Result<Vec<PeerEmission>, SyncRuntimeError> {
        let mut outboxes = self.lock_outboxes()?;
        let Some(outbox) = outboxes.get_mut(&peer_id) else {
            return Ok(Vec::new());
        };
        if !Arc::ptr_eq(&outbox.readiness, &registration.readiness) {
            return Err(SyncRuntimeError::Network {
                message: "announcement outbox registration was replaced".to_owned(),
            });
        }
        if empty_only && !outbox.emissions.is_empty() {
            return Err(SyncRuntimeError::Network {
                message: "announcement cleanup has no earned Sync lifetime ownership".to_owned(),
            });
        }
        if remove {
            return Ok(outboxes
                .remove(&peer_id)
                .map(|outbox| outbox.emissions)
                .unwrap_or_default()
                .into());
        }
        Ok(outbox.emissions.drain(..).collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sync::{DurableSyncRuntime, SyncNetwork, SyncRuntimeConfig};
    use crate::{FjallNodeStore, chainstate::BasicFilterStartupMode};
    use open_bitcoin_core::{consensus::CanonicalChain, primitives::Block};
    use open_bitcoin_network::{VersionMessage, WireNetworkMessage};
    use std::path::PathBuf;

    fn fixture() -> (PathBuf, DurableSyncRuntime, Block) {
        let mut nonce = [0; 16];
        getrandom::fill(&mut nonce).expect("owned random path");
        let path = std::fs::canonicalize(std::env::temp_dir())
            .expect("physical parent")
            .join(format!(
                "phase167-outbox-proof-{:x}",
                u128::from_le_bytes(nonce)
            ));
        let config = SyncRuntimeConfig {
            network: SyncNetwork::Regtest,
            ..Default::default()
        };
        let opened = FjallNodeStore::open_canonical_genesis(&path, CanonicalChain::Regtest)
            .expect("real public proved genesis");
        let runtime = DurableSyncRuntime::open_canonical_genesis_configured(
            opened,
            config,
            BasicFilterStartupMode::Disabled,
        )
        .expect("actual durable runtime");
        (
            path,
            runtime,
            CanonicalChain::Regtest.spec().genesis_block().clone(),
        )
    }

    fn enqueue_two(runtime: &DurableSyncRuntime, block: &Block) -> Vec<WireNetworkMessage> {
        for message in [
            WireNetworkMessage::Version(VersionMessage {
                nonce: 1,
                ..VersionMessage::default()
            }),
            WireNetworkMessage::Verack,
        ] {
            runtime
                .network
                .receive_sync_message(
                    1,
                    message,
                    0,
                    runtime.verify_flags,
                    runtime.consensus_params,
                )
                .expect("actual handshake");
        }
        let mut expected = Vec::new();
        for headers in [false, true] {
            if headers {
                runtime
                    .network
                    .receive_sync_message(
                        1,
                        WireNetworkMessage::SendHeaders,
                        0,
                        runtime.verify_flags,
                        runtime.consensus_params,
                    )
                    .expect("real preference");
            }
            let outcomes = runtime
                .network
                .prepare_block_announcements(
                    block,
                    &runtime
                        .announcement_outboxes
                        .snapshots()
                        .expect("real queue pressure"),
                )
                .expect("genuine producer");
            expected.extend(outcomes.iter().filter_map(|outcome| match outcome {
                AnnouncementPreparationOutcome::Ready(emission) => Some(emission.message().clone()),
                _ => None,
            }));
            runtime
                .announcement_outboxes
                .enqueue_prepared(&runtime.network, outcomes)
                .expect("genuine queue");
        }
        assert_eq!(expected.len(), 2);
        assert_ne!(expected[0], expected[1]);
        expected
    }

    fn queued(runtime: &DurableSyncRuntime) -> Vec<WireNetworkMessage> {
        runtime
            .announcement_outboxes
            .lock_outboxes()
            .expect("real queue")
            .get(&1)
            .expect("still registered")
            .emissions
            .iter()
            .map(|emission| emission.message().clone())
            .collect()
    }

    #[test]
    fn phase167_peer_session_owned_absent_nonempty_queue_aborts_without_intervening_admission() {
        // Arrange
        let (path, runtime, block) = fixture();
        let registration = runtime
            .announcement_outboxes
            .register_peer(1)
            .expect("real registration");
        let (token, outbound) = runtime
            .network
            .connect_sync_peer(1, 0)
            .expect("real A factory");
        outbound.expect("version");
        let requests = PeerRequests::new(token);
        let expected = enqueue_two(&runtime, &block);
        assert_eq!(queued(&runtime), expected);
        let generation = registration.readiness.generation.load(Ordering::Acquire);
        runtime
            .network
            .disconnect_peer(1)
            .expect("actual A removal, no B");
        // Act
        runtime
            .announcement_outboxes
            .finish_registration(&runtime.network, 1, &registration, Some(&requests))
            .expect("earned A absent cleanup");
        // Assert
        assert!(
            runtime
                .announcement_outboxes
                .snapshots()
                .expect("removed")
                .is_empty()
        );
        assert_eq!(
            registration.readiness.generation.load(Ordering::Acquire),
            generation
        );
        assert!(
            runtime
                .network
                .peer_manager_snapshot()
                .expect("actual absence")
                .peer_state(1)
                .is_none()
        );
        drop(runtime);
        std::fs::remove_dir_all(path).expect("owned cleanup");
    }

    #[test]
    fn phase167_peer_session_same_registration_later_network_lifetime_never_grants_old_cleanup() {
        for route in 0..3 {
            for remove_b in [false, true] {
                // Arrange
                let (path, runtime, block) = fixture();
                let registration = runtime
                    .announcement_outboxes
                    .register_peer(1)
                    .expect("same real Arc");
                let (token, outbound) = runtime
                    .network
                    .connect_sync_peer(1, 0)
                    .expect("actual A factory");
                outbound.expect("version");
                let requests = PeerRequests::new(token);
                runtime.network.disconnect_peer(1).expect("A absent");
                match route {
                    0 => {
                        runtime
                            .network
                            .connect_outbound_peer(1, 1)
                            .expect("actual B outbound");
                    }
                    1 => runtime
                        .network
                        .add_inbound_peer(1)
                        .expect("actual B inbound"),
                    _ => assert!(matches!(
                        runtime
                            .network
                            .admit_inbound_peer(
                                open_bitcoin_network::InboundAdmissionRequest::ordinary(
                                    1,
                                    "127.0.0.1:18449"
                                )
                            )
                            .expect("actual B policy"),
                        open_bitcoin_network::InboundAdmissionDecision::Admit(_)
                    )),
                }
                let expected = enqueue_two(&runtime, &block);
                if remove_b {
                    runtime.network.disconnect_peer(1).expect("B absent again");
                }
                let before = format!(
                    "{:?}",
                    runtime.network.peer_manager_snapshot().expect("real state")
                );
                let generation = registration.readiness.generation.load(Ordering::Acquire);
                // Act
                let error = runtime
                    .announcement_outboxes
                    .finish_registration(&runtime.network, 1, &registration, Some(&requests))
                    .expect_err("original A permanently revoked");
                // Assert
                assert!(error.to_string().contains("sync peer lifetime"));
                assert_eq!(queued(&runtime), expected);
                assert_eq!(
                    registration.readiness.generation.load(Ordering::Acquire),
                    generation
                );
                assert_eq!(
                    format!(
                        "{:?}",
                        runtime.network.peer_manager_snapshot().expect("unchanged")
                    ),
                    before
                );
                runtime
                    .announcement_outboxes
                    .unregister_peer(&runtime.network, 1)
                    .expect("actual current generic owner still aborts both genuine capabilities");
                drop(runtime);
                std::fs::remove_dir_all(path).expect("owned cleanup");
            }
        }
    }

    #[test]
    fn phase167_peer_session_failed_admission_only_removes_matching_empty_registration() {
        for nonempty in [false, true] {
            // Arrange: preserve production preregistration-before-admission order.
            let (path, runtime, block) = fixture();
            runtime
                .network
                .connect_outbound_peer(1, 0)
                .expect("genuine existing peer");
            let registration = runtime
                .announcement_outboxes
                .register_peer(1)
                .expect("new real registration");
            let expected = if nonempty {
                enqueue_two(&runtime, &block)
            } else {
                Vec::new()
            };
            let primary = match runtime.network.connect_sync_peer(1, 1) {
                Err(error) => error,
                Ok(_) => panic!("duplicate actual membership cannot earn factory token"),
            };
            let primary_text = primary.to_string();
            let generation = registration.readiness.generation.load(Ordering::Acquire);
            // Act
            let cleanup = runtime.announcement_outboxes.finish_registration(
                &runtime.network,
                1,
                &registration,
                None,
            );
            let result: Result<(), SyncRuntimeError> =
                super::super::request_cleanup_result(Err(primary.into()), cleanup);
            // Assert
            let error = result.expect_err("genuine duplicate admission stays visible");
            assert!(error.to_string().contains(&primary_text));
            if nonempty {
                assert!(
                    error
                        .to_string()
                        .contains("no earned Sync lifetime ownership")
                );
                assert_eq!(queued(&runtime), expected);
                assert_eq!(
                    registration.readiness.generation.load(Ordering::Acquire),
                    generation
                );
                runtime
                    .announcement_outboxes
                    .unregister_peer(&runtime.network, 1)
                    .expect("known generic owner still owns both real capabilities");
            } else {
                assert!(
                    runtime
                        .announcement_outboxes
                        .snapshots()
                        .expect("empty removed")
                        .is_empty()
                );
                let current = runtime
                    .announcement_outboxes
                    .register_peer(1)
                    .expect("actual replacement empty Arc");
                let error = runtime
                    .announcement_outboxes
                    .finish_registration(&runtime.network, 1, &registration, None)
                    .expect_err("old no-token Arc cannot remove even empty B");
                assert!(error.to_string().contains("registration was replaced"));
                assert!(Arc::ptr_eq(
                    &runtime
                        .announcement_outboxes
                        .lock_outboxes()
                        .expect("current queue")
                        .get(&1)
                        .expect("still registered")
                        .readiness,
                    &current.readiness
                ));
                runtime
                    .announcement_outboxes
                    .unregister_peer(&runtime.network, 1)
                    .expect("current owner");
            }
            assert!(
                runtime
                    .network
                    .peer_manager_snapshot()
                    .expect("existing peer")
                    .peer_state(1)
                    .is_some()
            );
            drop(runtime);
            std::fs::remove_dir_all(path).expect("owned cleanup");
        }
    }
}
