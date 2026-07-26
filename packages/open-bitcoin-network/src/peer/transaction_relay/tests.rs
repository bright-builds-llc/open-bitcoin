// Parity breadcrumbs:
// - packages/bitcoin-knots/src/protocol.h
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/node/txdownloadman.h
// - packages/bitcoin-knots/src/node/txdownloadman_impl.cpp
// - packages/bitcoin-knots/src/txorphanage.h
// - packages/bitcoin-knots/src/txorphanage.cpp
// - packages/bitcoin-knots/src/txrequest.h
// - packages/bitcoin-knots/src/txrequest.cpp
// - packages/bitcoin-knots/test/functional/p2p_orphan_handling.py
// - packages/bitcoin-knots/test/functional/p2p_tx_download.py
// - packages/bitcoin-knots/test/functional/p2p_getdata.py

use open_bitcoin_primitives::{Hash32, InventoryType, InventoryVector, Txid, Wtxid};

use super::*;

mod fanout_cases;
mod orphanage_cases;
mod reject_evidence_cases;
mod scheduler_cases;
mod serving_cases;

fn txid(byte: u8) -> Txid {
    Txid::from_byte_array([byte; 32])
}

fn wtxid(byte: u8) -> Wtxid {
    Wtxid::from_byte_array([byte; 32])
}

#[test]
fn tx_relay_id_round_trips_txid_inventory() {
    // Arrange
    let txid = txid(1);
    let relay_id = TxRelayId::Txid(txid);

    // Act
    let inventory = relay_id.to_inventory_vector();

    // Assert
    assert_eq!(relay_id.inventory_type(), InventoryType::Transaction);
    assert_eq!(relay_id.object_hash(), Hash32::from(txid));
    assert_eq!(
        inventory,
        InventoryVector {
            inventory_type: InventoryType::Transaction,
            object_hash: Hash32::from(txid),
        },
    );
}

#[test]
fn tx_relay_id_round_trips_wtxid_inventory() {
    // Arrange
    let wtxid = wtxid(2);
    let relay_id = TxRelayId::Wtxid(wtxid);

    // Act
    let inventory = relay_id.to_inventory_vector();

    // Assert
    assert_eq!(relay_id.inventory_type(), InventoryType::WitnessTransaction,);
    assert_eq!(relay_id.object_hash(), Hash32::from(wtxid));
    assert_eq!(
        inventory,
        InventoryVector {
            inventory_type: InventoryType::WitnessTransaction,
            object_hash: Hash32::from(wtxid),
        },
    );
}

#[test]
fn inventory_identity_mismatch_is_typed() {
    // Arrange
    let tx_inventory = InventoryVector {
        inventory_type: InventoryType::Transaction,
        object_hash: Hash32::from(txid(3)),
    };
    let wtxid_inventory = InventoryVector {
        inventory_type: InventoryType::WitnessTransaction,
        object_hash: Hash32::from(wtxid(4)),
    };

    // Act
    let txid_for_wtxid_peer =
        TxRelayId::from_inventory_vector_for_peer(&tx_inventory, TxRelayPeerMode::WtxidRelay);
    let wtxid_for_txid_peer =
        TxRelayId::from_inventory_vector_for_peer(&wtxid_inventory, TxRelayPeerMode::TxidOnly);

    // Assert
    assert_eq!(
        txid_for_wtxid_peer,
        Err(TxRelayIdentityError::NegotiationMismatch {
            inventory_type: InventoryType::Transaction,
            peer_mode: TxRelayPeerMode::WtxidRelay,
        }),
    );
    assert_eq!(
        wtxid_for_txid_peer,
        Err(TxRelayIdentityError::NegotiationMismatch {
            inventory_type: InventoryType::WitnessTransaction,
            peer_mode: TxRelayPeerMode::TxidOnly,
        }),
    );
}

#[test]
fn peer_modes_and_inventory_errors_are_typed() {
    // Arrange
    let non_transaction_inventory = InventoryVector {
        inventory_type: InventoryType::Block,
        object_hash: Hash32::from_byte_array([9; 32]),
    };

    // Act
    let txid_mode = TxRelayPeerMode::from_remote_wtxidrelay(false);
    let wtxid_mode = TxRelayPeerMode::from_remote_wtxidrelay(true);
    let txid_inventory = TxRelayId::from_inventory_vector_for_peer(
        &InventoryVector {
            inventory_type: InventoryType::Transaction,
            object_hash: Hash32::from(txid(10)),
        },
        txid_mode,
    );
    let wtxid_inventory = TxRelayId::from_inventory_vector_for_peer(
        &InventoryVector {
            inventory_type: InventoryType::WitnessTransaction,
            object_hash: Hash32::from(wtxid(11)),
        },
        wtxid_mode,
    );
    let non_transaction =
        TxRelayId::from_inventory_vector_for_peer(&non_transaction_inventory, txid_mode);

    // Assert
    assert_eq!(
        txid_mode.expected_inventory_type(),
        InventoryType::Transaction
    );
    assert_eq!(
        wtxid_mode.expected_inventory_type(),
        InventoryType::WitnessTransaction,
    );
    assert_eq!(txid_inventory, Ok(TxRelayId::Txid(txid(10))));
    assert_eq!(wtxid_inventory, Ok(TxRelayId::Wtxid(wtxid(11))));
    assert_eq!(
        non_transaction,
        Err(TxRelayIdentityError::NotTransactionInventory {
            inventory_type: InventoryType::Block,
        }),
    );
}

#[test]
fn transaction_relay_action_labels_are_fixed() {
    // Arrange
    let txid = txid(5);
    let wtxid = wtxid(6);
    let relay_id = TxRelayId::Txid(txid);
    let actions = [
        TxDownloadAction::RequestGetData {
            peer_id: 1,
            relay_id,
        },
        TxDownloadAction::SuppressDuplicate {
            peer_id: 1,
            relay_id,
        },
        TxDownloadAction::SuppressAlreadyHave {
            peer_id: 1,
            relay_id,
        },
        TxDownloadAction::SuppressRecentReject {
            peer_id: 1,
            relay_id,
        },
        TxDownloadAction::Suppress {
            peer_id: 1,
            relay_id,
            reason: TxDownloadSuppressionReason::MempoolKnown,
        },
        TxDownloadAction::Suppress {
            peer_id: 1,
            relay_id,
            reason: TxDownloadSuppressionReason::RelayDisabled,
        },
        TxDownloadAction::SuppressIdentityMismatch {
            peer_id: 1,
            reason: TxDownloadSuppressionReason::IdentityMismatch,
        },
        TxDownloadAction::SuppressRequestCap {
            peer_id: 1,
            relay_id,
        },
        TxDownloadAction::FallbackRequest {
            peer_id: 1,
            relay_id,
        },
        TxDownloadAction::RequestExpired {
            peer_id: 1,
            relay_id,
        },
        TxDownloadAction::NotFoundCleanup {
            peer_id: 1,
            relay_id,
        },
        TxDownloadAction::ReceivedTxCleanup {
            peer_id: 1,
            txid,
            wtxid,
        },
        TxDownloadAction::PeerCleanup { peer_id: 1 },
    ];

    // Act
    let labels = actions.map(|action| action.as_str());
    let peer_ids = actions.map(|action| action.peer_id());
    let mempool_reason = actions[4]
        .suppression_reason()
        .expect("mempool-known suppression reason");

    // Assert
    assert_eq!(
        labels,
        [
            "request_getdata",
            "suppress_duplicate",
            "suppress_already_have",
            "suppress_recent_reject",
            "suppress_mempool_known",
            "suppress_relay_disabled",
            "suppress_identity_mismatch",
            "suppress_request_cap",
            "fallback_request",
            "request_expired",
            "notfound_cleanup",
            "received_tx_cleanup",
            "peer_cleanup",
        ],
    );
    assert_eq!(peer_ids, [1; 13]);
    assert_eq!(mempool_reason, TxDownloadSuppressionReason::MempoolKnown);
    assert_eq!(mempool_reason.as_str(), "mempool_known");
}

#[test]
fn generic_suppression_labels_cover_all_reasons() {
    // Arrange
    let relay_id = TxRelayId::Txid(txid(12));
    let actions = [
        TxDownloadAction::Suppress {
            peer_id: 1,
            relay_id,
            reason: TxDownloadSuppressionReason::AlreadyHave,
        },
        TxDownloadAction::Suppress {
            peer_id: 1,
            relay_id,
            reason: TxDownloadSuppressionReason::RecentReject,
        },
        TxDownloadAction::Suppress {
            peer_id: 1,
            relay_id,
            reason: TxDownloadSuppressionReason::Duplicate,
        },
        TxDownloadAction::Suppress {
            peer_id: 1,
            relay_id,
            reason: TxDownloadSuppressionReason::InFlight,
        },
        TxDownloadAction::Suppress {
            peer_id: 1,
            relay_id,
            reason: TxDownloadSuppressionReason::RequestCapReached,
        },
        TxDownloadAction::Suppress {
            peer_id: 1,
            relay_id,
            reason: TxDownloadSuppressionReason::IdentityMismatch,
        },
        TxDownloadAction::Suppress {
            peer_id: 1,
            relay_id,
            reason: TxDownloadSuppressionReason::NotTransactionInventory,
        },
        TxDownloadAction::Suppress {
            peer_id: 1,
            relay_id,
            reason: TxDownloadSuppressionReason::RelayDisabled,
        },
        TxDownloadAction::Suppress {
            peer_id: 1,
            relay_id,
            reason: TxDownloadSuppressionReason::NotRelayEligible,
        },
        TxDownloadAction::Suppress {
            peer_id: 1,
            relay_id,
            reason: TxDownloadSuppressionReason::InboundServingRequired,
        },
        TxDownloadAction::Suppress {
            peer_id: 1,
            relay_id,
            reason: TxDownloadSuppressionReason::PermissionRequired,
        },
        TxDownloadAction::Suppress {
            peer_id: 1,
            relay_id,
            reason: TxDownloadSuppressionReason::ProtectedNotRelay,
        },
    ];

    // Act
    let labels = actions.map(|action| action.as_str());
    let reasons = [
        TxDownloadSuppressionReason::Duplicate,
        TxDownloadSuppressionReason::AlreadyHave,
        TxDownloadSuppressionReason::RecentReject,
        TxDownloadSuppressionReason::InFlight,
        TxDownloadSuppressionReason::RequestCapReached,
        TxDownloadSuppressionReason::IdentityMismatch,
        TxDownloadSuppressionReason::NotTransactionInventory,
        TxDownloadSuppressionReason::RelayDisabled,
        TxDownloadSuppressionReason::NotRelayEligible,
        TxDownloadSuppressionReason::InboundServingRequired,
        TxDownloadSuppressionReason::PermissionRequired,
        TxDownloadSuppressionReason::ProtectedNotRelay,
    ]
    .map(TxDownloadSuppressionReason::as_str);
    let request_inventory = TxDownloadAction::FallbackRequest {
        peer_id: 2,
        relay_id,
    }
    .maybe_request_inventory();
    let non_request_inventory =
        TxDownloadAction::PeerCleanup { peer_id: 2 }.maybe_request_inventory();
    let non_suppression = TxDownloadAction::RequestGetData {
        peer_id: 2,
        relay_id,
    }
    .suppression_reason();
    let direct_duplicate_reason = TxDownloadAction::SuppressDuplicate {
        peer_id: 2,
        relay_id,
    }
    .suppression_reason();
    let direct_already_have_reason = TxDownloadAction::SuppressAlreadyHave {
        peer_id: 2,
        relay_id,
    }
    .suppression_reason();
    let direct_recent_reject_reason = TxDownloadAction::SuppressRecentReject {
        peer_id: 2,
        relay_id,
    }
    .suppression_reason();
    let request_cap_reason = TxDownloadAction::SuppressRequestCap {
        peer_id: 2,
        relay_id,
    }
    .suppression_reason();

    // Assert
    assert_eq!(
        labels,
        [
            "suppress_already_have",
            "suppress_recent_reject",
            "suppress_duplicate",
            "suppress_duplicate",
            "suppress_request_cap",
            "suppress_identity_mismatch",
            "suppress_identity_mismatch",
            "suppress_relay_disabled",
            "suppress_not_relay_eligible",
            "suppress_inbound_serving_required",
            "suppress_permission_required",
            "suppress_protected_not_relay",
        ],
    );
    assert_eq!(
        reasons,
        [
            "duplicate",
            "already_have",
            "recent_reject",
            "in_flight",
            "request_cap_reached",
            "identity_mismatch",
            "not_transaction_inventory",
            "relay_disabled",
            "not_relay_eligible",
            "inbound_serving_required",
            "permission_required",
            "protected_not_relay",
        ],
    );
    assert_eq!(request_inventory, Some(relay_id.to_inventory_vector()));
    assert_eq!(non_request_inventory, None);
    assert_eq!(non_suppression, None);
    assert_eq!(
        direct_duplicate_reason,
        Some(TxDownloadSuppressionReason::Duplicate),
    );
    assert_eq!(
        direct_already_have_reason,
        Some(TxDownloadSuppressionReason::AlreadyHave),
    );
    assert_eq!(
        direct_recent_reject_reason,
        Some(TxDownloadSuppressionReason::RecentReject),
    );
    assert_eq!(
        request_cap_reason,
        Some(TxDownloadSuppressionReason::RequestCapReached),
    );
}

#[test]
fn download_policy_defaults_use_knots_inspired_names_and_values() {
    // Arrange / Act
    let policy = TxDownloadPolicy::default();

    // Assert
    assert_eq!(policy.max_announcements_per_peer, 5_000);
    assert_eq!(policy.max_in_flight_per_peer, 100);
    assert_eq!(policy.txid_relay_delay_seconds, 2);
    assert_eq!(policy.non_preferred_peer_delay_seconds, 2);
    assert_eq!(policy.overloaded_peer_delay_seconds, 2);
    assert_eq!(policy.getdata_tx_interval_seconds, 60);
}

#[test]
fn txid_announcement_requests_transaction_inventory() {
    scheduler_cases::txid_announcement_requests_transaction_inventory();
}

#[test]
fn wtxid_announcement_requests_witness_transaction_inventory() {
    scheduler_cases::wtxid_announcement_requests_witness_transaction_inventory();
}

#[test]
fn identity_mismatch_suppresses_without_candidate_or_inflight_state() {
    scheduler_cases::identity_mismatch_suppresses_without_candidate_or_inflight_state();
}

#[test]
fn disabled_relay_suppresses_announcement_without_request_state() {
    scheduler_cases::disabled_relay_suppresses_announcement_without_request_state();
}

#[test]
fn ineligible_relay_suppressions_are_typed_without_request_state() {
    scheduler_cases::ineligible_relay_suppressions_are_typed_without_request_state();
}

#[test]
fn ineligible_eligible_reason_maps_to_not_relay_eligible() {
    scheduler_cases::ineligible_eligible_reason_maps_to_not_relay_eligible();
}

#[test]
fn disabled_parent_request_suppresses_without_request_state() {
    scheduler_cases::disabled_parent_request_suppresses_without_request_state();
}

#[test]
fn ineligible_first_announcement_does_not_block_eligible_second_announcer() {
    scheduler_cases::ineligible_first_announcement_does_not_block_eligible_second_announcer();
}

#[test]
fn duplicate_announcement_retains_fallback_candidate_without_second_request() {
    scheduler_cases::duplicate_announcement_retains_fallback_candidate_without_second_request();
}

#[test]
fn orphan_parent_request_suppresses_duplicate_pending_parent_with_fallback() {
    scheduler_cases::orphan_parent_request_suppresses_duplicate_pending_parent_with_fallback();
}

#[test]
fn already_have_recent_reject_and_mempool_known_suppress_requests() {
    scheduler_cases::semantic_reject_facts_suppress_inventory_while_parent_bypasses_reconsiderable(
    );
}

#[test]
fn inflight_cap_suppresses_additional_ready_requests() {
    scheduler_cases::inflight_cap_suppresses_additional_ready_requests();
}

#[test]
fn txid_delay_waits_until_fake_clock_reaches_ready_time() {
    scheduler_cases::txid_delay_waits_until_fake_clock_reaches_ready_time();
}

#[test]
fn non_preferred_peer_delay_waits_until_fake_clock_reaches_ready_time() {
    scheduler_cases::non_preferred_peer_delay_waits_until_fake_clock_reaches_ready_time();
}

#[test]
fn overloaded_peer_delay_waits_until_fake_clock_reaches_ready_time() {
    scheduler_cases::overloaded_peer_delay_waits_until_fake_clock_reaches_ready_time();
}

#[test]
fn expiry_fallback_waits_until_fake_clock_reaches_getdata_interval() {
    scheduler_cases::expiry_fallback_waits_until_fake_clock_reaches_getdata_interval();
}

#[test]
fn timeout_expires_request_and_falls_back_to_duplicate_announcer() {
    scheduler_cases::timeout_expires_request_and_falls_back_to_duplicate_announcer();
}

#[test]
fn notfound_clears_matching_request_and_falls_back() {
    scheduler_cases::notfound_clears_matching_request_and_falls_back();
}

#[test]
fn disconnect_cleanup_removes_peer_state_and_falls_back() {
    scheduler_cases::disconnect_cleanup_removes_peer_state_and_falls_back();
}

#[test]
fn received_transaction_cleanup_waits_for_admission_before_already_have() {
    scheduler_cases::received_transaction_cleanup_waits_for_admission_before_already_have();
}

#[test]
fn tx_serving_policy_reports_low_cardinality_outcomes() {
    serving_cases::tx_serving_policy_reports_low_cardinality_outcomes();
}

#[test]
fn tx_serving_policy_rejects_identity_mismatch_and_non_transaction_inventory() {
    serving_cases::tx_serving_policy_rejects_identity_mismatch_and_non_transaction_inventory();
}

#[test]
fn tx_fanout_policy_honors_identity_and_limits() {
    fanout_cases::tx_fanout_policy_honors_identity_and_limits();
}

#[test]
fn tx_fanout_policy_suppresses_origin_and_ineligible_peers() {
    fanout_cases::tx_fanout_policy_suppresses_origin_and_ineligible_peers();
}

#[test]
fn tx_fanout_policy_reports_rebroadcast_deferred_without_timer() {
    fanout_cases::tx_fanout_policy_reports_rebroadcast_deferred_without_timer();
}

#[test]
fn tx_fanout_policy_covers_fixed_labels_and_suppression_edges() {
    fanout_cases::tx_fanout_policy_covers_fixed_labels_and_suppression_edges();
}

#[test]
fn tx_fanout_policy_cleans_up_duplicates_and_empty_state() {
    fanout_cases::tx_fanout_policy_cleans_up_duplicates_and_empty_state();
}
