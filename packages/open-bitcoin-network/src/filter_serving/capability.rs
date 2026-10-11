// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_permissions.h
// - packages/bitcoin-knots/src/net_permissions.cpp
// - packages/bitcoin-knots/src/init.cpp

use crate::{PeerPermissionSet, PeerPermissionToken, ServiceFlags};

/// Configured capability is independent of index availability and catch-up.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct BasicPeerServingCapability {
    pub general_configured: bool,
    pub effective_permission: bool,
    pub explicit_permission_requested: bool,
    pub all_permission_requested: bool,
}

/// Startup prerequisite, including the pinned `all`-without-index exception.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BasicPeerServingPrerequisite {
    Accepted,
    BasicIndexRequired,
}

impl BasicPeerServingCapability {
    /// Retains requested provenance separately from expanded permission flags.
    pub fn from_permissions(general_configured: bool, permissions: &PeerPermissionSet) -> Self {
        Self {
            general_configured,
            effective_permission: permissions.contains_token(PeerPermissionToken::BlockFilters),
            explicit_permission_requested: permissions
                .requested_token(PeerPermissionToken::BlockFilters),
            all_permission_requested: permissions.requested_token(PeerPermissionToken::All),
        }
    }

    /// General serving and explicitly named blockfilters require BASIC enabled.
    pub const fn validate_basic_index(self, basic_enabled: bool) -> BasicPeerServingPrerequisite {
        if !basic_enabled && (self.general_configured || self.explicit_permission_requested) {
            return BasicPeerServingPrerequisite::BasicIndexRequired;
        }
        BasicPeerServingPrerequisite::Accepted
    }

    /// Capability offered before querying any backend or readiness state.
    pub const fn advertised(self) -> bool {
        self.general_configured || self.effective_permission
    }

    /// Overlays only this session; it never changes the global service base.
    pub fn own_services(self, base: ServiceFlags) -> ServiceFlags {
        if self.advertised() {
            return base | ServiceFlags::COMPACT_FILTERS;
        }
        base
    }
}

#[cfg(test)]
mod tests;
