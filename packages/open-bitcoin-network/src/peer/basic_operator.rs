// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_permissions.h
// - packages/bitcoin-knots/src/net_permissions.cpp
// - packages/bitcoin-knots/src/init.cpp
// - packages/bitcoin-knots/src/net_processing.cpp

//! Constant-size session scope; inspecting it never traverses the peer table.

use super::PeerManager;
use crate::BasicPeerServingCapability;

impl super::PeerState {
    pub(super) fn from_inbound_record(mut record: crate::InboundPeerRecord) -> Self {
        record.handshake_state = crate::InboundHandshakeState::Handshaking;
        let basic_scope = BasicPeerScope::from_capability(
            record.permission_decision.basic_filter_capability(),
            false,
        );
        let mut state = Self::new(super::ConnectionRole::Inbound);
        state.maybe_inbound_record = Some(record);
        state.basic_scope = basic_scope;
        state
    }
}

/// Current modeled sessions, with overlapping permission and offer categories.
/// `implicit_sessions` counts only proven implicit-origin sessions. The current
/// permission grammar has no implicit whitelist resolver, so this count is zero;
/// ordinary and explicitly permissioned non-filter sessions are not implicit.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct BasicPeerScope {
    pub general_sessions: u64,
    pub explicit_sessions: u64,
    pub all_sessions: u64,
    pub implicit_sessions: u64,
    pub offered_sessions: u64,
}

impl BasicPeerScope {
    pub(super) fn from_capability(capability: BasicPeerServingCapability, offered: bool) -> Self {
        Self {
            general_sessions: u64::from(capability.general_configured),
            explicit_sessions: u64::from(capability.explicit_permission_requested),
            all_sessions: u64::from(capability.all_permission_requested),
            implicit_sessions: 0,
            offered_sessions: u64::from(offered),
        }
    }

    fn maybe_transition(self, old: Self, new: Self) -> Option<Self> {
        Some(Self {
            general_sessions: self
                .general_sessions
                .checked_sub(old.general_sessions)?
                .checked_add(new.general_sessions)?,
            explicit_sessions: self
                .explicit_sessions
                .checked_sub(old.explicit_sessions)?
                .checked_add(new.explicit_sessions)?,
            all_sessions: self
                .all_sessions
                .checked_sub(old.all_sessions)?
                .checked_add(new.all_sessions)?,
            implicit_sessions: self
                .implicit_sessions
                .checked_sub(old.implicit_sessions)?
                .checked_add(new.implicit_sessions)?,
            offered_sessions: self
                .offered_sessions
                .checked_sub(old.offered_sessions)?
                .checked_add(new.offered_sessions)?,
        })
    }
}

impl PeerManager {
    /// Copies five counters; arithmetic exhaustion stays unavailable until a new manager.
    pub fn maybe_basic_peer_scope(&self) -> Option<BasicPeerScope> {
        self.maybe_basic_scope
    }

    pub(super) fn transition_basic_peer_scope(&mut self, old: BasicPeerScope, new: BasicPeerScope) {
        self.maybe_basic_scope = self
            .maybe_basic_scope
            .and_then(|scope| scope.maybe_transition(old, new));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{LocalPeerConfig, NetworkError, ServiceFlags};

    fn inbound(peer_id: crate::PeerId, tokens: &[&str]) -> crate::InboundPeerRecord {
        let class = crate::ParsedPeerPermissionClass::parse("scope", ["127.0.0.1"], tokens)
            .expect("permission class");
        let permission_decision = crate::PeerPermissionClassRegistry::new([class])
            .resolve_inbound("127.0.0.1".parse().expect("literal address"));
        crate::InboundPeerRecord {
            peer_id,
            remote_endpoint: "test".into(),
            slot_class: permission_decision.slot_class(),
            connection_class: permission_decision.connection_class(),
            permission_decision,
            handshake_state: crate::InboundHandshakeState::Accepted,
            maybe_remote_nonce: None,
            observed_inbound_peers: 0,
            observed_outbound_peers: 0,
        }
    }

    fn version(
        manager: &mut PeerManager,
        peer_id: crate::PeerId,
        nonce: u64,
    ) -> Vec<super::super::PeerAction> {
        manager
            .handle_message(
                peer_id,
                crate::WireNetworkMessage::Version(crate::VersionMessage {
                    nonce,
                    ..Default::default()
                }),
                0,
            )
            .expect("version dispatch")
    }

    #[test]
    fn phase161_peer_scope_permission_provenance_and_offer_matrix() {
        let cases: &[(&[&str], u64, u64)] = &[
            (&["in", "blockfilters"], 1, 0),
            (&["in", "all"], 0, 1),
            (&["in", "all", "blockfilters"], 1, 1),
            (&["in", "mempool", "noban", "addr", "relay"], 0, 0),
            (&["in", "download"], 0, 0),
        ];
        for &(tokens, explicit, all) in cases {
            for general in [false, true] {
                // Arrange
                let mut manager = PeerManager::new(LocalPeerConfig {
                    services: if general {
                        ServiceFlags::COMPACT_FILTERS
                    } else {
                        ServiceFlags::from_bits(0)
                    },
                    nonce: 5,
                    ..Default::default()
                });
                let record = inbound(1, tokens);
                manager
                    .add_inbound_peer_record(record.clone())
                    .expect("admitted");
                let admitted = manager.maybe_basic_peer_scope().expect("admission scope");
                // Act
                let duplicate = manager.add_inbound_peer_record(record);
                let unchanged = manager.maybe_basic_peer_scope();
                let actions = version(&mut manager, 1, 6);
                let offered = manager.maybe_basic_peer_scope().expect("offered scope");
                manager.remove_peer(1).expect("remove");
                // Assert
                assert_eq!(
                    admitted,
                    BasicPeerScope {
                        explicit_sessions: explicit,
                        all_sessions: all,
                        ..Default::default()
                    }
                );
                assert_eq!(duplicate, Err(NetworkError::PeerAlreadyExists(1)));
                assert_eq!(unchanged, Some(admitted));
                assert_eq!(
                    offered,
                    BasicPeerScope {
                        general_sessions: u64::from(general),
                        explicit_sessions: explicit,
                        all_sessions: all,
                        implicit_sessions: 0,
                        offered_sessions: u64::from(general || explicit != 0 || all != 0),
                    }
                );
                assert!(actions.iter().any(|action| matches!(action, super::super::PeerAction::Send(crate::WireNetworkMessage::Version(v)) if v.services.contains(ServiceFlags::COMPACT_FILTERS) == (offered.offered_sessions == 1))));
                assert_eq!(
                    manager.maybe_basic_peer_scope(),
                    Some(BasicPeerScope::default())
                );
            }
        }
    }

    #[test]
    fn phase161_peer_scope_failed_handshake_disconnects_once() {
        // Arrange
        let mut manager = PeerManager::new(LocalPeerConfig {
            nonce: 5,
            ..Default::default()
        });
        manager
            .add_inbound_peer_record(inbound(1, &["in", "all"]))
            .expect("admitted");
        // Act
        let actions = version(&mut manager, 1, 5);
        let disconnected = manager.maybe_basic_peer_scope();
        manager
            .remove_peer(1)
            .expect("cleanup disconnected session");
        manager
            .add_inbound_peer_record(inbound(1, &["in", "blockfilters"]))
            .expect("reconnect");
        version(&mut manager, 1, 6);
        // Assert
        assert!(matches!(
            actions.as_slice(),
            [super::super::PeerAction::Disconnect(
                crate::DisconnectReason::SelfConnection
            )]
        ));
        assert_eq!(disconnected, Some(BasicPeerScope::default()));
        assert_eq!(
            manager.maybe_basic_peer_scope(),
            Some(BasicPeerScope {
                explicit_sessions: 1,
                offered_sessions: 1,
                ..Default::default()
            })
        );
    }

    #[test]
    fn phase161_peer_scope_duplicate_version_revokes_then_cleanup_keeps_zero() {
        // Arrange
        let mut manager = PeerManager::new(LocalPeerConfig {
            nonce: 5,
            ..Default::default()
        });
        manager
            .add_inbound_peer_record(inbound(1, &["in", "all"]))
            .expect("admitted");
        version(&mut manager, 1, 6);
        // Act
        version(&mut manager, 1, 6);
        let disconnected = manager.maybe_basic_peer_scope();
        manager.remove_peer(1).expect("cleanup");
        let unknown =
            manager.handle_message(9, crate::WireNetworkMessage::Version(Default::default()), 0);
        let missing_remove = manager.remove_peer(9);
        // Assert
        assert_eq!(disconnected, Some(BasicPeerScope::default()));
        assert_eq!(manager.maybe_basic_peer_scope(), disconnected);
        assert_eq!(unknown, Err(NetworkError::UnknownPeer(9)));
        assert_eq!(missing_remove, Err(NetworkError::UnknownPeer(9)));
    }

    fn scalar(index: usize, value: u64) -> BasicPeerScope {
        let mut fields = [0; 5];
        fields[index] = value;
        BasicPeerScope {
            general_sessions: fields[0],
            explicit_sessions: fields[1],
            all_sessions: fields[2],
            implicit_sessions: fields[3],
            offered_sessions: fields[4],
        }
    }

    #[test]
    fn phase161_peer_scope_every_counter_underflow_and_overflow_is_sticky_unavailable() {
        for index in 0..5 {
            for overflow in [false, true] {
                // Arrange
                let mut manager = PeerManager::new(LocalPeerConfig::default());
                let zero = BasicPeerScope::default();
                manager.maybe_basic_scope = Some(if overflow {
                    scalar(index, u64::MAX)
                } else {
                    zero
                });
                // Act
                manager.transition_basic_peer_scope(
                    if overflow { zero } else { scalar(index, 1) },
                    if overflow { scalar(index, 1) } else { zero },
                );
                let exhausted = manager.maybe_basic_peer_scope();
                manager.transition_basic_peer_scope(zero, zero);
                // Assert
                assert_eq!(exhausted, None, "field={index} overflow={overflow}");
                assert_eq!(manager.maybe_basic_peer_scope(), None);
            }
        }
    }

    #[test]
    fn phase161_peer_scope_copy_and_clone_do_not_inspect_sessions() {
        // Arrange
        let mut manager = PeerManager::new(LocalPeerConfig {
            services: ServiceFlags::COMPACT_FILTERS,
            ..Default::default()
        });
        manager.add_outbound_peer(1, 0).expect("offered");
        let expected = manager.maybe_basic_peer_scope();
        // Act: deliberately remove table contents without lifecycle bookkeeping in
        // this pure test, so any table-derived getter would return a different value.
        manager.peers.clear();
        let copied = manager.maybe_basic_peer_scope();
        let cloned = manager.clone().maybe_basic_peer_scope();
        // Assert
        assert_eq!(copied, expected);
        assert_eq!(cloned, expected);
    }

    #[test]
    fn phase161_peer_scope_duplicate_remove_and_reconnect() {
        // Arrange
        let mut manager = PeerManager::new(LocalPeerConfig {
            services: ServiceFlags::COMPACT_FILTERS,
            ..Default::default()
        });
        manager.add_outbound_peer(1, 0).expect("first session");
        let first = manager.maybe_basic_peer_scope().expect("known counters");
        // Act
        let duplicate = manager.add_outbound_peer(1, 0);
        manager.remove_peer(1).expect("remove session");
        let empty = manager.maybe_basic_peer_scope();
        manager.set_local_services(ServiceFlags::from_bits(0));
        manager
            .add_outbound_peer(1, 0)
            .expect("replacement session");
        // Assert
        assert_eq!(first.general_sessions, 1);
        assert_eq!(first.offered_sessions, 1);
        assert_eq!(duplicate, Err(NetworkError::PeerAlreadyExists(1)));
        assert_eq!(empty, Some(BasicPeerScope::default()));
        assert_eq!(manager.maybe_basic_peer_scope(), empty);
    }

    #[test]
    fn phase161_peer_scope_general_toggle_preserves_existing_offers() {
        // Arrange
        let mut manager = PeerManager::new(LocalPeerConfig::default());
        manager.add_outbound_peer(1, 0).expect("ordinary session");
        manager.set_local_services(ServiceFlags::COMPACT_FILTERS);
        manager.add_outbound_peer(2, 0).expect("offered session");
        // Act
        manager.set_local_services(ServiceFlags::from_bits(0));
        let before = manager.maybe_basic_peer_scope();
        for _ in 0..64 {
            assert_eq!(manager.maybe_basic_peer_scope(), before);
        }
        manager.remove_peer(2).expect("remove offered session");
        // Assert
        assert_eq!(before.expect("scope").general_sessions, 1);
        assert_eq!(before.expect("scope").offered_sessions, 1);
        assert_eq!(
            manager.maybe_basic_peer_scope(),
            Some(BasicPeerScope::default())
        );
    }
}
