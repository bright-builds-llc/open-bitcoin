// Parity breadcrumbs:
// - packages/bitcoin-knots/src/init.cpp
// - packages/bitcoin-knots/src/protocol.h
// - packages/bitcoin-knots/src/net_processing.cpp

//! Version-message services chosen from prune mode.

use open_bitcoin_core::chainstate::{CoinsView, PruneMode};
use open_bitcoin_network::{PermissionEffectLabel, ServiceFlags, advertised_service_flags};

use super::ManagedPeerNetwork;
use crate::ChainstateStore;

impl<S: ChainstateStore, V: CoinsView> ManagedPeerNetwork<S, V> {
    /// Sets advertised services from `mode`.
    ///
    /// Manual and automatic mode advertise limited service even when no block
    /// data has been deleted. Disabled mode advertises full history.
    pub fn set_prune_mode(&mut self, mode: PruneMode) {
        self.prune_mode = mode;
        let services = advertised_service_flags(mode);
        self.local_config.services = services;
        self.peer_manager.set_local_services(services);
    }

    /// Stores whether a committed prune delete has removed block data.
    ///
    /// The flag does not choose version-message services.
    pub fn set_serving_have_pruned(&mut self, have_pruned: bool) {
        self.serving_have_pruned = have_pruned;
    }

    /// Services written onto the local version message.
    pub fn local_services(&self) -> ServiceFlags {
        self.local_config.services
    }
}

/// Ordinary peers that request a historical block body are removed.
///
/// The download permission is the only exemption. `noban` does not keep
/// the peer.
pub(super) fn disconnect_for_limited_window_request(
    limited_window_refused: bool,
    active_permission_effects: &[PermissionEffectLabel],
) -> bool {
    limited_window_refused
        && (!active_permission_effects.contains(&PermissionEffectLabel::DownloadServingPolicyInput)
            || active_permission_effects
                .contains(&PermissionEffectLabel::MisbehaviorPolicyProtected))
}
