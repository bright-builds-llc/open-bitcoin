// Parity breadcrumbs:
// - packages/bitcoin-knots/src/blockencodings.h
// - packages/bitcoin-knots/src/blockencodings.cpp
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/net_processing.h
// - packages/bitcoin-knots/test/functional/p2p_compactblocks.py

use super::*;

#[test]
fn cleanup_matrix_clears_in_flight_state_without_touching_unrelated_blocks() {
    let hash_a = BlockHash::from_byte_array([10_u8; 32]);
    let hash_b = BlockHash::from_byte_array([11_u8; 32]);
    let mut peer_state = CompactDownloadPeerState::new();
    peer_state.in_flight.insert(
        hash_a,
        super::super::CompactDownloadInFlight {
            partial: PartialCompactBlock::new(),
            getblocktxn_in_flight: true,
            requested_indexes: vec![1],
            started_at_unix: 0,
        },
    );
    peer_state.in_flight.insert(
        hash_b,
        super::super::CompactDownloadInFlight {
            partial: PartialCompactBlock::new(),
            getblocktxn_in_flight: false,
            requested_indexes: Vec::new(),
            started_at_unix: 0,
        },
    );

    assert!(cleanup_compact_download_on_block_connected(
        &mut peer_state,
        hash_a
    ));
    assert!(!peer_state.in_flight.contains_key(&hash_a));
    assert!(peer_state.in_flight.contains_key(&hash_b));

    assert_eq!(
        cleanup_compact_download_peer(&mut peer_state, CompactDownloadCleanupCause::Timeout),
        1
    );
    assert!(peer_state.in_flight.is_empty());

    for cause in [
        CompactDownloadCleanupCause::PeerDisconnect,
        CompactDownloadCleanupCause::Reorg,
        CompactDownloadCleanupCause::RuntimeRestart,
        CompactDownloadCleanupCause::BlockConnected,
    ] {
        peer_state.in_flight.insert(
            hash_a,
            super::super::CompactDownloadInFlight {
                partial: PartialCompactBlock::new(),
                getblocktxn_in_flight: false,
                requested_indexes: Vec::new(),
                started_at_unix: 0,
            },
        );
        assert_eq!(cleanup_compact_download_peer(&mut peer_state, cause), 1);
    }
}

#[test]
fn expire_stale_compact_downloads_removes_timed_out_in_flight_state() {
    let block_hash = BlockHash::from_byte_array([7_u8; 32]);
    let mut peer_state = CompactDownloadPeerState::new();
    peer_state.in_flight.insert(
        block_hash,
        super::super::CompactDownloadInFlight {
            partial: PartialCompactBlock::new(),
            getblocktxn_in_flight: true,
            requested_indexes: vec![1],
            started_at_unix: 100,
        },
    );

    let expired = expire_stale_compact_downloads(
        &mut peer_state,
        100 + COMPACT_BLOCK_DOWNLOAD_TIMEOUT_SECONDS + 1,
        COMPACT_BLOCK_DOWNLOAD_TIMEOUT_SECONDS,
    );

    assert_eq!(expired.len(), 1);
    assert_eq!(expired[0].block_hash, block_hash);
    assert!(peer_state.in_flight.is_empty());
}

#[test]
fn compact_download_cleanup_and_suppression_labels_are_stable() {
    assert_eq!(
        CompactDownloadCleanupCause::PeerDisconnect.as_str(),
        "compact_download_peer_disconnect"
    );
    assert_eq!(
        CompactDownloadCleanupCause::Timeout.as_str(),
        "compact_download_timeout"
    );
    assert_eq!(
        CompactDownloadCleanupCause::Reorg.as_str(),
        "compact_download_reorg"
    );
    assert_eq!(
        CompactDownloadCleanupCause::RuntimeRestart.as_str(),
        "compact_download_restart"
    );
    assert_eq!(
        CompactDownloadCleanupCause::BlockConnected.as_str(),
        "compact_download_block_connected"
    );
    assert_eq!(
        CompactDownloadSuppressionReason::CompactReconstructionFailed.as_str(),
        "compact_reconstruction_failed"
    );
    assert_eq!(
        CompactDownloadSuppressionReason::CompactDownloadTimeout.as_str(),
        "compact_download_timeout"
    );
    assert_eq!(
        CompactDownloadSuppressionReason::CompactPeerIneligible.as_str(),
        "compact_peer_ineligible"
    );
    assert_eq!(
        CompactDownloadSuppressionReason::CompactReconstructionInvalid.as_str(),
        "compact_reconstruction_invalid"
    );
    assert_eq!(
        CompactDownloadSuppressionReason::CompactBlockAlreadyInFlight.as_str(),
        "compact_block_already_in_flight"
    );
}

#[test]
fn expire_stale_compact_downloads_keeps_fresh_in_flight_state() {
    let block_hash = BlockHash::from_byte_array([8_u8; 32]);
    let mut peer_state = CompactDownloadPeerState::new();
    peer_state.in_flight.insert(
        block_hash,
        super::super::CompactDownloadInFlight {
            partial: PartialCompactBlock::new(),
            getblocktxn_in_flight: false,
            requested_indexes: Vec::new(),
            started_at_unix: 1_000,
        },
    );

    let expired = expire_stale_compact_downloads(
        &mut peer_state,
        1_000 + COMPACT_BLOCK_DOWNLOAD_TIMEOUT_SECONDS,
        COMPACT_BLOCK_DOWNLOAD_TIMEOUT_SECONDS,
    );

    assert!(expired.is_empty());
    assert!(peer_state.in_flight.contains_key(&block_hash));
}
