// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net.cpp
// - packages/bitcoin-knots/src/net_processing.cpp

use super::*;
use std::{collections::BTreeMap, sync::Weak};

#[derive(Default)]
pub(in crate::network::runtime_authority) struct PeerSessions {
    next_nonce: u64,
    current: BTreeMap<PeerId, u64>,
    cleanup: BTreeMap<PeerId, Weak<CleanupLease>>,
}

/// Only the Sync factory creates and retains the strong identity.
pub(super) struct CleanupLease;

#[derive(Debug)]
pub(super) enum LifetimeFailure {
    RegistryUnavailable,
    Exhausted,
    Foreign,
    EpochChanged,
    Replaced,
    Absent,
    CleanupRevoked,
    StillPresent,
}

impl From<LifetimeFailure> for ManagedNetworkAuthorityError {
    fn from(error: LifetimeFailure) -> Self {
        Self::LifecycleEffect(format!("sync peer lifetime: {error:?}"))
    }
}

impl PeerSessions {
    fn reserve(&mut self) -> Result<u64, ManagedNetworkAuthorityError> {
        self.next_nonce = self
            .next_nonce
            .checked_add(1)
            .ok_or(LifetimeFailure::Exhausted)?;
        Ok(self.next_nonce)
    }

    pub(super) fn is_current(&self, peer: PeerId, nonce: u64) -> bool {
        self.current.get(&peer) == Some(&nonce)
    }

    pub(super) fn prune<S: ChainstateStore, V: CoinsView>(
        &mut self,
        network: &ManagedPeerNetwork<S, V>,
    ) {
        self.current
            .retain(|peer, _| network.peer_manager().peer_state(*peer).is_some());
        // Absence is not revocation; only actual later admission or dead identity prunes.
        self.cleanup
            .retain(|_, identity| identity.strong_count() != 0);
    }

    pub(super) fn install_cleanup(&mut self, peer: PeerId) -> Arc<CleanupLease> {
        let identity = Arc::new(CleanupLease);
        self.cleanup.insert(peer, Arc::downgrade(&identity));
        identity
    }

    pub(super) fn require_cleanup(
        &self,
        peer: PeerId,
        identity: &Arc<CleanupLease>,
    ) -> Result<(), ManagedNetworkAuthorityError> {
        let maybe_current = self.cleanup.get(&peer).and_then(Weak::upgrade);
        if !maybe_current
            .as_ref()
            .is_some_and(|current| Arc::ptr_eq(current, identity))
        {
            return Err(LifetimeFailure::CleanupRevoked.into());
        }
        Ok(())
    }

    pub(super) fn admit<S: ChainstateStore, V: CoinsView, T>(
        &mut self,
        network: &mut ManagedPeerNetwork<S, V>,
        peer: PeerId,
        command: impl FnOnce(&mut ManagedPeerNetwork<S, V>) -> Result<T, ManagedNetworkError>,
    ) -> Result<(Result<T, ManagedNetworkAuthorityError>, Option<u64>), ManagedNetworkAuthorityError>
    {
        let existed = network.peer_manager().peer_state(peer).is_some();
        let nonce = self.reserve()?;
        let result = command(network).map_err(Into::into);
        let maybe_created = if !existed && network.peer_manager().peer_state(peer).is_some() {
            // This also runs when the real command failed AFTER creating membership.
            self.cleanup.remove(&peer);
            self.current.insert(peer, nonce);
            Some(nonce)
        } else {
            None
        };
        Ok((result, maybe_created))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;

    #[test]
    fn phase167_peer_session_all_admission_routes_refuse_old_lifetime() {
        for route in 0..3 {
            // Arrange
            let handle = memory_handle();
            let (token, outbound) = handle.connect_sync_peer(7, 0).expect("real admission");
            outbound.expect("real version");
            handle.disconnect_peer(7).expect("actual removal");
            match route {
                0 => {
                    handle
                        .connect_outbound_peer(7, 1)
                        .expect("outbound replacement");
                }
                1 => handle.add_inbound_peer(7).expect("inbound replacement"),
                _ => assert!(matches!(
                    handle
                        .admit_inbound_peer(InboundAdmissionRequest::ordinary(7, "127.0.0.1:18444"))
                        .expect("policy admission"),
                    InboundAdmissionDecision::Admit(_)
                )),
            }
            let before = format!("{:?}", handle.peer_manager_snapshot().expect("replacement"));
            let called = Cell::new(false);
            // Act
            let result = handle.with_sync_peer_requests(&token, |_owner| called.set(true));
            let finished = handle.finish_sync_peer(&token);
            // Assert
            assert!(result.is_err());
            assert!(finished.is_err());
            assert!(!called.get());
            assert_eq!(
                format!("{:?}", handle.peer_manager_snapshot().expect("unchanged")),
                before
            );
        }
    }

    #[test]
    fn phase167_peer_session_nonce_exhaustion_precedes_all_admission_effects() {
        // Arrange
        let handle = memory_handle();
        let (token, outbound) = handle.connect_sync_peer(1, 0).expect("original");
        outbound.expect("version");
        handle
            .with_peer_sessions(|_network, registry| {
                registry.next_nonce = u64::MAX;
                Ok(())
            })
            .expect("private exhaustion setup");
        let before = format!("{:?}", handle.peer_manager_snapshot().expect("before"));
        // Act
        let outbound = handle.connect_outbound_peer(2, 0);
        let inbound = handle.add_inbound_peer(3);
        let policy =
            handle.admit_inbound_peer(InboundAdmissionRequest::ordinary(4, "127.0.0.1:18445"));
        // Assert
        assert!(outbound.is_err() && inbound.is_err() && policy.is_err());
        assert_eq!(
            format!("{:?}", handle.peer_manager_snapshot().expect("after")),
            before
        );
        assert!(matches!(
            handle.observe_sync_peer(&token),
            Ok(SyncPeerObservation::Present { .. })
        ));
        handle
            .with_sync_peer_outbox_cleanup(&token, |_| ())
            .expect("exhausted attempted admissions preserve original cleanup identity");
    }

    #[test]
    fn phase167_peer_session_registry_is_current_bounded_and_nonces_never_reuse() {
        // Arrange
        let handle = memory_handle();
        let mut previous = 0;
        // Act / Assert
        for _ in 0..128 {
            let (token, outbound) = handle
                .connect_sync_peer(1, 0)
                .expect("actual renewed admission");
            outbound.expect("version");
            assert!(token.nonce > previous);
            previous = token.nonce;
            handle.finish_sync_peer(&token).expect("actual removal");
            handle
                .with_peer_sessions(|network, registry| {
                    assert!(registry.current.is_empty());
                    assert_eq!(registry.cleanup.len(), 1);
                    registry.require_cleanup(token.peer, &token.cleanup)?;
                    assert!(network.peer_manager().peer_ids().is_empty());
                    Ok(())
                })
                .expect("bounded registry");
            drop(token);
            handle
                .with_peer_sessions(|_, registry| {
                    assert!(registry.cleanup.is_empty());
                    Ok(())
                })
                .expect("dead Weak pruned in each actual round");
        }
    }
    #[test]
    fn phase167_peer_session_cleanup_lease_survives_only_absence_without_intervening_admission() {
        for route in 0..3 {
            // Arrange
            let handle = memory_handle();
            let (token, outbound) = handle.connect_sync_peer(1, 0).expect("actual factory");
            outbound.expect("real version");
            let called = Cell::new(0);
            // Act / Assert: duplicate operations create no new lifetime.
            assert!(handle.connect_outbound_peer(1, 0).is_err());
            assert!(handle.add_inbound_peer(1).is_err());
            assert!(matches!(
                handle
                    .admit_inbound_peer(InboundAdmissionRequest::ordinary(1, "127.0.0.1:18444"))
                    .expect("duplicate policy"),
                InboundAdmissionDecision::Reject(_)
            ));
            handle
                .with_sync_peer_outbox_cleanup(&token, |_| called.set(called.get() + 1))
                .expect("original current cleanup lease");
            handle.disconnect_peer(1).expect("actual original removed");
            handle
                .with_sync_peer_outbox_cleanup(&token, |_| {
                    assert!(handle.authority.try_lock().is_err());
                    assert!(handle.peer_sessions.try_lock().is_ok());
                    called.set(called.get() + 1);
                })
                .expect("absence preserves factory cleanup; short registry already released");
            match route {
                0 => {
                    handle
                        .connect_outbound_peer(1, 1)
                        .expect("actual B outbound");
                }
                1 => handle.add_inbound_peer(1).expect("actual B inbound"),
                _ => assert!(matches!(
                    handle
                        .admit_inbound_peer(InboundAdmissionRequest::ordinary(1, "127.0.0.1:18445"))
                        .expect("actual B policy"),
                    InboundAdmissionDecision::Admit(_)
                )),
            }
            assert!(
                handle
                    .with_sync_peer_outbox_cleanup(&token, |_| called.set(99))
                    .is_err()
            );
            handle.disconnect_peer(1).expect("actual B removed");
            assert!(
                handle
                    .with_sync_peer_outbox_cleanup(&token, |_| called.set(99))
                    .is_err()
            );
            assert_eq!(called.get(), 2);
        }
    }

    #[test]
    fn phase167_peer_session_error_after_real_created_membership_revokes_cleanup() {
        for route in 0..3 {
            // Arrange
            let handle = memory_handle();
            let (token, outbound) = handle.connect_sync_peer(1, 0).expect("original factory");
            outbound.expect("version");
            handle.disconnect_peer(1).expect("original absent");
            // Act: real raw creation followed by a genuine duplicate error, no copied cause.
            let (result, maybe_nonce) = handle
                .with_peer_sessions(|network, registry| {
                    registry.admit(network, 1, |network| {
                        match route {
                            0 => {
                                network.connect_outbound_peer(1, 1)?;
                            }
                            1 => network.add_inbound_peer(1)?,
                            _ => assert!(matches!(
                                network.admit_inbound_peer(InboundAdmissionRequest::ordinary(
                                    1,
                                    "127.0.0.1:18446"
                                )),
                                InboundAdmissionDecision::Admit(_)
                            )),
                        }
                        network.add_inbound_peer(1)
                    })
                })
                .expect("actual private admission kernel boundary");
            // Assert: this covers kernel error-after-membership, not a fabricated public failure.
            assert!(result.is_err());
            assert!(maybe_nonce.is_some());
            assert!(
                handle
                    .peer_manager_snapshot()
                    .expect("real membership")
                    .peer_state(1)
                    .is_some()
            );
            let called = Cell::new(false);
            assert!(
                handle
                    .with_sync_peer_outbox_cleanup(&token, |_| called.set(true))
                    .is_err()
            );
            handle
                .disconnect_peer(1)
                .expect("actual created member removed");
            assert!(
                handle
                    .with_sync_peer_outbox_cleanup(&token, |_| called.set(true))
                    .is_err()
            );
            assert!(!called.get());
        }
    }

    #[test]
    fn phase167_peer_session_live_absent_cleanup_bounds_are_not_admission_history() {
        // Arrange
        let handle = memory_handle();
        let mut tokens = Vec::new();
        // Act / Assert: 128 actual absent owners stay eligible until a later admission.
        for peer in 1..=128 {
            let (token, outbound) = handle.connect_sync_peer(peer, 0).expect("actual factory");
            outbound.expect("version");
            handle.finish_sync_peer(&token).expect("actual removal");
            tokens.push(token);
            handle
                .with_peer_sessions(|_, registry| {
                    assert!(registry.current.is_empty());
                    assert_eq!(registry.cleanup.len(), tokens.len());
                    Ok(())
                })
                .expect("one Weak for each still-live original token");
        }
        for token in &tokens {
            handle
                .with_sync_peer_outbox_cleanup(token, |_| ())
                .expect("live absent cleanup");
        }
        drop(tokens);
        handle
            .with_peer_sessions(|_, registry| {
                assert!(registry.current.is_empty());
                assert!(registry.cleanup.is_empty());
                Ok(())
            })
            .expect("all dead Weak entries pruned without tombstones");
    }

    #[test]
    fn phase167_peer_session_cleanup_rejects_foreign_epoch_and_actual_poison() {
        // Arrange
        let handle = memory_handle();
        let foreign = memory_handle();
        let (token, outbound) = handle.connect_sync_peer(1, 0).expect("actual original");
        outbound.expect("version");
        let called = Cell::new(false);
        // Act / Assert
        assert!(
            foreign
                .with_sync_peer_outbox_cleanup(&token, |_| called.set(true))
                .is_err()
        );
        handle
            .mutate(|network| network.install_authority_incarnation())
            .expect("actual epoch");
        assert!(
            handle
                .with_sync_peer_outbox_cleanup(&token, |_| called.set(true))
                .is_err()
        );
        assert!(!called.get());
        let handle = memory_handle();
        let (token, outbound) = handle
            .connect_sync_peer(2, 0)
            .expect("actual other original");
        outbound.expect("version");
        let registry = Arc::clone(&handle.peer_sessions);
        assert!(
            std::thread::spawn(move || {
                let _guard = registry.lock().expect("real registry lock");
                panic!("intentional actual registry poison");
            })
            .join()
            .is_err()
        );
        assert!(
            handle
                .with_sync_peer_outbox_cleanup(&token, |_| called.set(true))
                .is_err()
        );
        assert!(!called.get());
        let handle = memory_handle();
        let (token, outbound) = handle
            .connect_sync_peer(3, 0)
            .expect("actual third original");
        outbound.expect("version");
        handle.poison_for_test();
        assert!(
            handle
                .with_sync_peer_outbox_cleanup(&token, |_| called.set(true))
                .is_err()
        );
        assert!(!called.get());
    }
    #[test]
    fn phase167_peer_session_new_factory_identity_never_resurrects_older_live_token() {
        // Arrange
        let handle = memory_handle();
        let (old, outbound) = handle.connect_sync_peer(1, 0).expect("actual A factory");
        outbound.expect("version");
        handle.finish_sync_peer(&old).expect("actual A removed");
        let (current, outbound) = handle.connect_sync_peer(1, 1).expect("actual B factory");
        outbound.expect("version");
        // Act / Assert
        assert!(current.nonce > old.nonce);
        assert!(handle.with_sync_peer_outbox_cleanup(&old, |_| ()).is_err());
        handle
            .with_sync_peer_outbox_cleanup(&current, |_| ())
            .expect("only B is current");
        handle.finish_sync_peer(&current).expect("B removed");
        handle
            .with_sync_peer_outbox_cleanup(&current, |_| ())
            .expect("B owns absent cleanup");
        assert!(handle.with_sync_peer_outbox_cleanup(&old, |_| ()).is_err());
        drop(current);
        handle
            .with_peer_sessions(|_, registry| {
                assert!(registry.current.is_empty());
                assert!(registry.cleanup.is_empty());
                Ok(())
            })
            .expect("dead latest Weak pruned, old token still actually alive");
        assert!(handle.with_sync_peer_outbox_cleanup(&old, |_| ()).is_err());
    }
    #[test]
    fn phase167_peer_session_absence_requirement_is_direct_and_preserves_real_failures() {
        // Arrange
        let handle = memory_handle();
        let foreign = memory_handle();
        let (token, outbound) = handle.connect_sync_peer(1, 0).expect("actual factory");
        outbound.expect("version");
        let before = format!(
            "{:?}",
            handle.peer_manager_snapshot().expect("actual present")
        );
        // Act / Assert: no request capture/disconnect is granted by this direct requirement.
        assert!(
            handle
                .require_absent_sync_peer(&token)
                .expect_err("still present")
                .to_string()
                .contains("StillPresent")
        );
        assert!(
            foreign
                .require_absent_sync_peer(&token)
                .expect_err("foreign")
                .to_string()
                .contains("Foreign")
        );
        assert_eq!(
            format!("{:?}", handle.peer_manager_snapshot().expect("unchanged")),
            before
        );
        handle.disconnect_peer(1).expect("actual absence");
        handle
            .require_absent_sync_peer(&token)
            .expect("only actual absence succeeds");
        handle
            .mutate(|network| network.install_authority_incarnation())
            .expect("real epoch change");
        assert!(
            handle
                .require_absent_sync_peer(&token)
                .expect_err("epoch changed")
                .to_string()
                .contains("EpochChanged")
        );
        let handle = memory_handle();
        let (token, outbound) = handle.connect_sync_peer(2, 0).expect("real second owner");
        outbound.expect("version");
        assert!(
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                let _result = handle.with_sync_peer_requests(&token, |_owner| {
                    panic!("actual request-scope poison")
                });
            }))
            .is_err()
        );
        assert!(handle.require_absent_sync_peer(&token).is_err());
        let actual = match handle.authority.lock() {
            Err(poison) => poison.into_inner(),
            Ok(_) => panic!("real authority remains poisoned, no recovery API"),
        };
        assert!(actual.peer_manager().peer_state(2).is_some());
    }
}
