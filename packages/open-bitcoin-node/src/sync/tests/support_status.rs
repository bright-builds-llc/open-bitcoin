// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/test/functional/p2p_initial_headers_sync.py

use super::*;

pub(super) fn peer_outcome(
    peer: SyncPeerAddress,
    state: PeerSyncState,
    attempts: u8,
    maybe_failure_reason: Option<PeerFailureReason>,
    maybe_error: Option<String>,
) -> PeerSyncOutcome {
    PeerSyncOutcome {
        maybe_resolved_endpoint: Some(format!("127.0.0.1:{}", peer.port)),
        network: SyncNetwork::Regtest,
        contribution: PeerContribution {
            messages_processed: 0,
            headers_received: 0,
            blocks_received: 0,
        },
        maybe_tip_height: None,
        maybe_tip_hash: None,
        maybe_tip_work: None,
        maybe_last_activity_unix_seconds: None,
        maybe_capabilities: None,
        peer,
        state,
        attempts,
        maybe_failure_reason,
        maybe_error,
    }
}

pub(super) fn peer_outcome_with_contribution(
    peer: SyncPeerAddress,
    state: PeerSyncState,
    attempts: u8,
    maybe_failure_reason: Option<PeerFailureReason>,
    contribution: PeerContribution,
) -> PeerSyncOutcome {
    let mut outcome = peer_outcome(peer, state, attempts, maybe_failure_reason, None);
    outcome.contribution = contribution;
    outcome
}

pub(super) fn summary_with_peer_failure(reason: PeerFailureReason, error: &str) -> SyncRunSummary {
    let mut summary = SyncRunSummary::empty(0, 0, 1);
    summary.failed_peers = 1;
    summary.peer_outcomes.push(peer_outcome(
        SyncPeerAddress::manual("127.0.0.1", 18_444),
        PeerSyncState::Failed,
        1,
        Some(reason),
        Some(error.to_string()),
    ));
    summary
}

pub(super) fn assert_no_progress_status(
    state: &DurableSyncState,
    diagnosis: NoProgressDiagnosis,
    next_action: &str,
) {
    assert_eq!(
        state.sync.no_progress_diagnosis,
        FieldAvailability::available(diagnosis)
    );
    assert_eq!(
        state.sync.no_progress_next_action,
        FieldAvailability::available(next_action.to_string())
    );
}

pub(super) fn available_progress_credit(state: &DurableSyncState) -> &ProgressCreditEvidence {
    let FieldAvailability::Available(credit) = &state.sync.progress_credit else {
        panic!("progress credit should be available");
    };
    credit
}

pub(super) fn available_last_useful_work(state: &DurableSyncState) -> &ProgressCreditEvidence {
    let FieldAvailability::Available(credit) = &state.sync.last_useful_work else {
        panic!("last useful work should be available");
    };
    credit
}

pub(super) fn available_last_peer_contribution(
    state: &DurableSyncState,
) -> &PeerContributionEvidence {
    let FieldAvailability::Available(contribution) = &state.sync.last_peer_contribution else {
        panic!("last_peer_contribution should be available");
    };
    contribution
}

pub(super) fn available_stall_diagnosis(state: &DurableSyncState) -> &StallDiagnosisEvidence {
    let FieldAvailability::Available(diagnosis) = &state.sync.stall_diagnosis else {
        panic!("stall diagnosis should be available");
    };
    diagnosis
}

pub(super) fn assert_progress_credit_unavailable(state: &DurableSyncState) {
    assert!(matches!(
        state.sync.progress_credit,
        FieldAvailability::Unavailable { .. }
    ));
}

pub(super) fn assert_rejected_activity(
    credit: &ProgressCreditEvidence,
    kind: RejectedProgressActivityKind,
) {
    assert!(
        credit
            .rejected_activity
            .iter()
            .any(|activity| activity.kind == kind),
        "missing rejected activity {kind:?} in {credit:?}"
    );
}

pub(super) fn block_relay_status_for_metrics() -> BlockRelayEvidenceStatus {
    BlockRelayEvidenceStatus::with_components(
        BlockServingEvidenceStatus::with_activation_eligibility_and_status(
            BlockServingActivationEvidence {
                block_serving_enabled: true,
                compact_relay_enabled: true,
            },
            BlockServingEligibilityCounters {
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
            BlockServingStatusCounters {
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
        CompactRelayNegotiationCounters {
            version2_high_bandwidth_count: 3,
            version2_low_bandwidth_count: 1,
            unsupported_version_count: 1,
        },
        CompactRelayAnnouncementCounters {
            compact_announced_count: 6,
            compact_headers_fallback_count: 2,
            compact_inventory_fallback_count: 1,
            compact_suppressed_count: 2,
        },
        CompactRelayReconstructionCounters {
            compact_reconstructed_count: 4,
            compact_reconstruction_failed_count: 1,
            compact_malformed_count: 1,
        },
        CompactRelayMissingTransactionCounters {
            compact_missing_tx_requested_count: 2,
            compact_missing_tx_suppressed_count: 1,
        },
        CompactRelayFallbackCounters {
            compact_fallback_count: 2,
            compact_timeout_count: 1,
        },
        CompactRelayInFlightCounters {
            in_flight_count: 3,
            getblocktxn_in_flight_count: 2,
            peers_with_in_flight_count: 2,
        },
        CompactRelayCleanupCounters {
            compact_cleanup_count: 3,
            compact_download_peer_disconnect_count: 1,
            compact_download_timeout_count: 1,
            compact_download_reorg_count: 0,
            compact_download_restart_count: 0,
            compact_download_block_connected_count: 1,
        },
    )
}

pub(super) fn inbound_status_for_metrics() -> InboundPeerServingStatus {
    InboundPeerServingStatus {
        listener_state: "ready".to_string(),
        bound_endpoints: Vec::new(),
        preflight_reason: "ready".to_string(),
        admitted_inbound_peers: 1,
        rejected_inbound_peers: 2,
        handshake: InboundHandshakeStatusCounts::default(),
        duplicate_rejects: 5,
        self_connection_rejects: 6,
        cap_rejects: 3,
        reserved_slot_rejects: 4,
        latest_admission_event: FieldAvailability::unavailable("no admission event"),
        permissioned_inbound_peers: 7,
        protected_inbound_peers: 8,
        permission_class: "ordinary_inbound".to_string(),
        active_permission_effects: Vec::new(),
        inactive_permission_effects: Vec::new(),
        inactive_permission_effect_observations: 9,
        permission_validation_failures: 10,
        latest_permission_decision: FieldAvailability::unavailable("no permission decision"),
        local_advertisement_candidates: Vec::new(),
        suppressed_advertisements: Vec::new(),
        getaddr_responses_served: 0,
        getaddr_requests_suppressed: 0,
        learned_address_entries: 0,
        learned_address_rejections: 0,
        latest_address_decision: FieldAvailability::unavailable("no address decision"),
        eviction_candidates_evaluated: 11,
        disconnects_requested: 12,
        discouraged_peers: 0,
        active_bans: 13,
        expired_bans: 0,
        manual_unbans: 0,
        misbehavior_observations: 14,
        protected_no_actions: 15,
        latest_peer_policy_decision: FieldAvailability::unavailable("no peer policy decision"),
        resource_pressure_events: 16,
        read_queue_pressure_events: 17,
        write_queue_pressure_events: 18,
        request_cap_events: 19,
        payload_rejections: 20,
        timeout_disconnects: 21,
        churn_rejections: 22,
        reconnect_suppressions: 23,
        latest_resource_governance_decision: FieldAvailability::unavailable(
            "no resource governance decision",
        ),
    }
}
