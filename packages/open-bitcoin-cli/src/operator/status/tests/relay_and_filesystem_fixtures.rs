use super::*;

pub(super) fn relay_evidence_status_fixture() -> RelayEvidenceStatus {
    let mut status = RelayEvidenceStatus::with_activation_recovery_and_counters(
        RelayActivationEvidence { enabled: true },
        RelayDownloadEligibilityCounters {
            eligible_peer_count: 1,
            ineligible_peer_count: 5,
            relay_disabled_count: 2,
            not_relay_eligible_count: 3,
            inbound_serving_required_count: 0,
            permission_required_count: 4,
            protected_not_relay_count: 1,
        },
        RelayRecoveryCounters {
            recovered_count: 11,
            dropped_confirmed_count: 12,
            dropped_duplicate_count: 13,
            dropped_missing_parent_count: 14,
            dropped_policy_incompatible_count: 15,
            dropped_evicted_count: 16,
        },
        RelayEvidenceCounters {
            accepted_count: 1,
            rejected_count: 2,
            orphaned_count: 3,
            requested_count: 4,
            served_count: 5,
            announced_count: 6,
            suppressed_count: 7,
            evicted_count: 8,
            expired_count: 9,
            rebroadcast_deferred_count: 10,
        },
    );
    status.mempool_admission = RelayEvidenceField::implemented(RelayCapabilityEvidence::new(
        RelayEvidenceCapability::MempoolAdmission,
    ));
    status.local_submission = RelayEvidenceField::implemented(RelayCapabilityEvidence::new(
        RelayEvidenceCapability::LocalSubmissionRelay,
    ));
    status.fanout = RelayEvidenceField::implemented(RelayCapabilityEvidence::new(
        RelayEvidenceCapability::RelayFanout,
    ));
    status.serving = RelayEvidenceField::implemented(RelayCapabilityEvidence::new(
        RelayEvidenceCapability::RelayServing,
    ));
    status
}

pub(super) fn block_relay_evidence_status_fixture() -> BlockRelayEvidenceStatus {
    BlockRelayEvidenceStatus::with_components(
        open_bitcoin_node::status::BlockServingEvidenceStatus::with_activation_eligibility_and_status(
            open_bitcoin_node::status::BlockServingActivationEvidence {
                block_serving_enabled: true,
                compact_relay_enabled: true,
            },
            open_bitcoin_node::status::BlockServingEligibilityCounters {
                eligible_peer_count: 2,
                ineligible_peer_count: 3,
                disabled_count: 1,
                activation_required_count: 0,
                inbound_serving_required_count: 1,
                permission_required_count: 1,
                protected_not_serving_count: 0,
                status_unavailable_count: 0,
                permission_effect_inactive_count: 1,
            },
            open_bitcoin_node::status::BlockServingStatusCounters {
                validated_count: 5,
                available_count: 4,
                stale_count: 1,
                side_chain_count: 2,
                pruned_count: 1,
                unavailable_count: 3,
                unvalidated_count: 0,
                unknown_count: 1,
                suppressed_count: 2,
            },
        ),
        open_bitcoin_node::status::CompactRelayNegotiationCounters {
            version2_high_bandwidth_count: 3,
            version2_low_bandwidth_count: 1,
            unsupported_version_count: 1,
        },
        open_bitcoin_node::status::CompactRelayAnnouncementCounters {
            compact_announced_count: 6,
            compact_headers_fallback_count: 2,
            compact_inventory_fallback_count: 1,
            compact_suppressed_count: 2,
        },
        open_bitcoin_node::status::CompactRelayReconstructionCounters {
            compact_reconstructed_count: 4,
            compact_reconstruction_failed_count: 1,
            compact_malformed_count: 1,
        },
        open_bitcoin_node::status::CompactRelayMissingTransactionCounters {
            compact_missing_tx_requested_count: 2,
            compact_missing_tx_suppressed_count: 1,
        },
        open_bitcoin_node::status::CompactRelayFallbackCounters {
            compact_fallback_count: 2,
            compact_timeout_count: 1,
        },
        open_bitcoin_node::status::CompactRelayInFlightCounters {
            in_flight_count: 3,
            getblocktxn_in_flight_count: 2,
            peers_with_in_flight_count: 2,
        },
        open_bitcoin_node::status::CompactRelayCleanupCounters {
            compact_cleanup_count: 3,
            compact_download_peer_disconnect_count: 1,
            compact_download_timeout_count: 1,
            compact_download_reorg_count: 0,
            compact_download_restart_count: 0,
            compact_download_block_connected_count: 1,
        },
    )
}

pub(super) fn temp_path(test_name: &str) -> PathBuf {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time after unix epoch")
        .as_nanos();
    std::env::temp_dir().join(format!(
        "open-bitcoin-status-{test_name}-{}-{timestamp}",
        std::process::id()
    ))
}

pub(super) fn remove_dir_if_exists(path: &Path) {
    match fs::remove_dir_all(path) {
        Ok(()) => {}
        Err(error) if error.kind() == io::ErrorKind::NotFound => {}
        Err(error) => panic!("failed to remove {}: {error}", path.display()),
    }
}

pub(super) fn assert_empty_dir(path: &Path) {
    let entries = fs::read_dir(path)
        .expect("read datadir")
        .collect::<Result<Vec<_>, _>>()
        .expect("datadir entries");
    assert!(
        entries.is_empty(),
        "datadir should remain empty: {entries:?}"
    );
}

pub(super) struct TempDirGuard {
    pub(super) path: PathBuf,
}

impl Drop for TempDirGuard {
    fn drop(&mut self) {
        remove_dir_if_exists(&self.path);
    }
}
