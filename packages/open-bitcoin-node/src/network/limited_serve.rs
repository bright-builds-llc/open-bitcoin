// Parity breadcrumbs:
// - packages/bitcoin-knots/src/init.cpp
// - packages/bitcoin-knots/src/protocol.h
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp

//! Version-message services composed from prune mode and explicit BASIC serving.

use open_bitcoin_core::chainstate::{CoinsView, PruneMode};
use open_bitcoin_network::{
    BasicPeerServingCapability, PermissionEffectLabel, ServiceFlags, advertised_service_flags,
};

use super::ManagedPeerNetwork;
use crate::ChainstateStore;

impl<S: ChainstateStore, V: CoinsView> ManagedPeerNetwork<S, V> {
    /// Configured prune mode, independent of whether any body has been deleted.
    pub fn prune_mode(&self) -> PruneMode {
        self.prune_mode
    }

    /// Updates full/limited service classification, preserving configured BASIC serving.
    pub fn set_prune_mode(&mut self, mode: PruneMode) {
        self.prune_mode = mode;
        self.apply_configured_services(
            self.local_config
                .services
                .contains(ServiceFlags::COMPACT_FILTERS),
        );
    }

    /// Applies explicit general BASIC peer serving, independently of catch-up.
    /// Startup must validate the BASIC prerequisite before calling this setter.
    pub fn set_basic_filter_serving(&mut self, general_configured: bool) {
        self.apply_configured_services(general_configured);
    }

    fn apply_configured_services(&mut self, general_configured: bool) {
        let capability = BasicPeerServingCapability {
            general_configured,
            ..Default::default()
        };
        let services = capability.own_services(advertised_service_flags(self.prune_mode));
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
