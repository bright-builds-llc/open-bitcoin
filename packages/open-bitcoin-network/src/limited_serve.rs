// Parity breadcrumbs:
// - packages/bitcoin-knots/src/protocol.h
// - packages/bitcoin-knots/src/init.cpp
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/validation.h

//! Pure advertised-service bits and the BIP 159 limited-serve window.

use open_bitcoin_chainstate::{MIN_BLOCKS_TO_KEEP, PruneMode};

use crate::message::ServiceFlags;

/// Extra heights beyond the delete keep window for historical block requests.
///
/// Knots adds this race buffer when deciding whether a pruned node may serve
/// a block body. Deletion still keeps [`MIN_BLOCKS_TO_KEEP`] heights.
pub const LIMITED_SERVE_RACE_BUFFER: u32 = 2;

/// Service flags a node advertises for `mode`.
///
/// Disabled keeps full-history `NETWORK | WITNESS`. Manual and automatic
/// prune advertise `NETWORK_LIMITED | WITNESS` and omit `NETWORK`.
pub fn advertised_service_flags(mode: PruneMode) -> ServiceFlags {
    if matches!(mode, PruneMode::Disabled) {
        return ServiceFlags::NETWORK | ServiceFlags::WITNESS;
    }
    ServiceFlags::NETWORK_LIMITED | ServiceFlags::WITNESS
}

/// Returns true when an active-chain block body sits outside the BIP 159 window.
///
/// Outside means tip distance is greater than the delete keep window plus the
/// historical-request race buffer.
pub fn block_request_exceeds_limited_serve_window(tip_height: u32, block_height: u32) -> bool {
    tip_height.saturating_sub(block_height) > MIN_BLOCKS_TO_KEEP + LIMITED_SERVE_RACE_BUFFER
}

#[cfg(test)]
mod tests {
    use open_bitcoin_chainstate::{MIN_BLOCKS_TO_KEEP, PruneMode};

    use crate::limited_serve::{
        LIMITED_SERVE_RACE_BUFFER, advertised_service_flags,
        block_request_exceeds_limited_serve_window,
    };
    use crate::message::{LocalPeerConfig, ServiceFlags, VersionMessage};

    const BLOOM_SERVICE_BITS: u64 = 1 << 2;
    const COMPACT_FILTERS_SERVICE_BITS: u64 = 1 << 6;

    #[test]
    fn disabled_mode_advertises_network_and_witness() {
        // Arrange
        let mode = PruneMode::Disabled;

        // Act
        let flags = advertised_service_flags(mode);

        // Assert
        assert!(flags.contains(ServiceFlags::NETWORK));
        assert!(flags.contains(ServiceFlags::WITNESS));
        assert!(!flags.contains(ServiceFlags::NETWORK_LIMITED));
    }

    #[test]
    fn manual_only_advertises_limited_without_network() {
        // Arrange
        let mode = PruneMode::ManualOnly;

        // Act
        let flags = advertised_service_flags(mode);

        // Assert
        assert!(flags.contains(ServiceFlags::NETWORK_LIMITED));
        assert!(flags.contains(ServiceFlags::WITNESS));
        assert!(!flags.contains(ServiceFlags::NETWORK));
    }

    #[test]
    fn automatic_mode_advertises_limited_without_network() {
        // Arrange
        let mode = PruneMode::Automatic { target_mib: 550 };

        // Act
        let flags = advertised_service_flags(mode);

        // Assert
        assert!(flags.contains(ServiceFlags::NETWORK_LIMITED));
        assert!(flags.contains(ServiceFlags::WITNESS));
        assert!(!flags.contains(ServiceFlags::NETWORK));
    }

    #[test]
    fn prune_advertisement_omits_bloom_and_compact_filter_bits() {
        // Arrange
        let bloom = ServiceFlags::from_bits(BLOOM_SERVICE_BITS);
        let compact_filters = ServiceFlags::from_bits(COMPACT_FILTERS_SERVICE_BITS);
        let manual = PruneMode::ManualOnly;
        let automatic = PruneMode::Automatic { target_mib: 550 };

        // Act
        let manual_flags = advertised_service_flags(manual);
        let automatic_flags = advertised_service_flags(automatic);

        // Assert
        assert!(!manual_flags.contains(bloom));
        assert!(!manual_flags.contains(compact_filters));
        assert!(!automatic_flags.contains(bloom));
        assert!(!automatic_flags.contains(compact_filters));
        assert_eq!(ServiceFlags::NETWORK_LIMITED.bits(), 1 << 10);
    }

    #[test]
    fn distance_290_is_inside_the_limited_serve_window() {
        // Arrange
        let tip_height = 1_000;
        let block_height = 710;

        // Act
        let exceeds = block_request_exceeds_limited_serve_window(tip_height, block_height);

        // Assert
        assert!(!exceeds);
    }

    #[test]
    fn distance_291_is_outside_the_limited_serve_window() {
        // Arrange
        let tip_height = 1_000;
        let block_height = 709;

        // Act
        let exceeds = block_request_exceeds_limited_serve_window(tip_height, block_height);

        // Assert
        assert!(exceeds);
    }

    #[test]
    fn tip_block_is_inside_the_limited_serve_window() {
        // Arrange
        let tip_height = 1_000;
        let block_height = 1_000;

        // Act
        let exceeds = block_request_exceeds_limited_serve_window(tip_height, block_height);

        // Assert
        assert!(!exceeds);
    }

    #[test]
    fn block_above_tip_is_inside_the_limited_serve_window() {
        // Arrange
        let tip_height = 1_000;
        let block_height = 1_001;

        // Act
        let exceeds = block_request_exceeds_limited_serve_window(tip_height, block_height);

        // Assert
        assert!(!exceeds);
    }

    #[test]
    fn keep_window_stays_288_and_race_buffer_is_two() {
        // Arrange / Act / Assert
        assert_eq!(MIN_BLOCKS_TO_KEEP, 288);
        assert_eq!(LIMITED_SERVE_RACE_BUFFER, 2);
    }

    #[test]
    fn default_services_stay_network_and_witness() {
        // Arrange
        let expected = ServiceFlags::NETWORK | ServiceFlags::WITNESS;

        // Act
        let version_services = VersionMessage::default().services;
        let local_services = LocalPeerConfig::default().services;

        // Assert
        assert_eq!(version_services, expected);
        assert_eq!(local_services, expected);
        assert!(!version_services.contains(ServiceFlags::NETWORK_LIMITED));
        assert!(!local_services.contains(ServiceFlags::NETWORK_LIMITED));
    }
}
