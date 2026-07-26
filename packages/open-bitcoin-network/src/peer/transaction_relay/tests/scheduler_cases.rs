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

use open_bitcoin_primitives::{Hash32, InventoryType, InventoryVector};

use crate::error::PeerId;
use crate::{RelayEligibilityDecision, RelayEligibilityReason};

use super::*;

mod edge_cases;
mod received_cases;

fn test_policy() -> TxDownloadPolicy {
    TxDownloadPolicy {
        max_announcements_per_peer: 3,
        max_in_flight_per_peer: 1,
        txid_relay_delay_seconds: 2,
        non_preferred_peer_delay_seconds: 2,
        overloaded_peer_delay_seconds: 2,
        getdata_tx_interval_seconds: 60,
    }
}

fn scheduler() -> TxDownloadScheduler {
    TxDownloadScheduler::new(test_policy())
}

fn txid_relay(byte: u8) -> TxRelayId {
    TxRelayId::Txid(txid(byte))
}

fn wtxid_relay(byte: u8) -> TxRelayId {
    TxRelayId::Wtxid(wtxid(byte))
}

fn txid_inventory(byte: u8) -> InventoryVector {
    txid_relay(byte).to_inventory_vector()
}

fn wtxid_inventory(byte: u8) -> InventoryVector {
    wtxid_relay(byte).to_inventory_vector()
}

fn announcement(
    peer_id: PeerId,
    inventory: InventoryVector,
    peer_mode: TxRelayPeerMode,
    now_unix_seconds: i64,
) -> TxAnnouncementInput {
    TxAnnouncementInput {
        peer_id,
        inventory,
        peer_mode,
        now_unix_seconds,
        local_facts: TxDownloadLocalFacts::default(),
        relay_eligibility: eligible_relay(),
        preferred_peer: true,
        peer_overloaded: false,
    }
}

fn parent_request(
    peer_id: PeerId,
    relay_id: TxRelayId,
    now_unix_seconds: i64,
) -> TxParentRequestInput {
    TxParentRequestInput {
        peer_id,
        relay_id,
        now_unix_seconds,
        local_facts: TxDownloadLocalFacts::default(),
        relay_eligibility: eligible_relay(),
    }
}

fn with_relay_eligibility(
    mut input: TxAnnouncementInput,
    relay_eligibility: RelayEligibilityDecision,
) -> TxAnnouncementInput {
    input.relay_eligibility = relay_eligibility;
    input
}

fn parent_with_relay_eligibility(
    mut input: TxParentRequestInput,
    relay_eligibility: RelayEligibilityDecision,
) -> TxParentRequestInput {
    input.relay_eligibility = relay_eligibility;
    input
}

fn eligible_relay() -> RelayEligibilityDecision {
    relay_decision(true, RelayEligibilityReason::Eligible)
}

fn relay_disabled() -> RelayEligibilityDecision {
    relay_decision(false, RelayEligibilityReason::Disabled)
}

fn inbound_serving_required() -> RelayEligibilityDecision {
    relay_decision(false, RelayEligibilityReason::InboundServingRequired)
}

fn permission_required() -> RelayEligibilityDecision {
    relay_decision(false, RelayEligibilityReason::PermissionRequired)
}

fn protected_not_relay() -> RelayEligibilityDecision {
    relay_decision(false, RelayEligibilityReason::ProtectedNotRelay)
}

fn ineligible_with_eligible_reason() -> RelayEligibilityDecision {
    relay_decision(false, RelayEligibilityReason::Eligible)
}

fn relay_decision(eligible: bool, reason: RelayEligibilityReason) -> RelayEligibilityDecision {
    RelayEligibilityDecision {
        eligible,
        reason,
        relay_permission_effects: Vec::new(),
        version_message_relay: eligible,
    }
}

fn not_preferred(mut input: TxAnnouncementInput) -> TxAnnouncementInput {
    input.preferred_peer = false;
    input
}

fn overloaded(mut input: TxAnnouncementInput) -> TxAnnouncementInput {
    input.peer_overloaded = true;
    input
}

fn announce_with(
    scheduler: &mut TxDownloadScheduler,
    peer_id: PeerId,
    inventory: InventoryVector,
    peer_mode: TxRelayPeerMode,
    now_unix_seconds: i64,
) -> Vec<TxDownloadAction> {
    scheduler.record_announcement(announcement(
        peer_id,
        inventory,
        peer_mode,
        now_unix_seconds,
    ))
}

fn announce_txid(
    scheduler: &mut TxDownloadScheduler,
    peer_id: PeerId,
    byte: u8,
    now_unix_seconds: i64,
) -> Vec<TxDownloadAction> {
    announce_with(
        scheduler,
        peer_id,
        txid_inventory(byte),
        TxRelayPeerMode::TxidOnly,
        now_unix_seconds,
    )
}

fn announce_wtxid(
    scheduler: &mut TxDownloadScheduler,
    peer_id: PeerId,
    byte: u8,
    now_unix_seconds: i64,
) -> Vec<TxDownloadAction> {
    announce_with(
        scheduler,
        peer_id,
        wtxid_inventory(byte),
        TxRelayPeerMode::WtxidRelay,
        now_unix_seconds,
    )
}

fn request(peer_id: PeerId, relay_id: TxRelayId) -> TxDownloadAction {
    TxDownloadAction::RequestGetData { peer_id, relay_id }
}

fn duplicate(peer_id: PeerId, relay_id: TxRelayId) -> TxDownloadAction {
    TxDownloadAction::SuppressDuplicate { peer_id, relay_id }
}

fn expect_already_have(peer_id: PeerId, relay_id: TxRelayId) -> TxDownloadAction {
    TxDownloadAction::SuppressAlreadyHave { peer_id, relay_id }
}

fn expect_recent_reject(peer_id: PeerId, relay_id: TxRelayId) -> TxDownloadAction {
    TxDownloadAction::SuppressRecentReject { peer_id, relay_id }
}

fn request_cap(peer_id: PeerId, relay_id: TxRelayId) -> TxDownloadAction {
    TxDownloadAction::SuppressRequestCap { peer_id, relay_id }
}

fn suppress(
    peer_id: PeerId,
    relay_id: TxRelayId,
    reason: TxDownloadSuppressionReason,
) -> TxDownloadAction {
    TxDownloadAction::Suppress {
        peer_id,
        relay_id,
        reason,
    }
}

fn fallback(peer_id: PeerId, relay_id: TxRelayId) -> TxDownloadAction {
    TxDownloadAction::FallbackRequest { peer_id, relay_id }
}

fn expect_expired(peer_id: PeerId, relay_id: TxRelayId) -> TxDownloadAction {
    TxDownloadAction::RequestExpired { peer_id, relay_id }
}

pub(super) fn txid_announcement_requests_transaction_inventory() {
    // Arrange
    let mut scheduler = scheduler();
    let relay_id = txid_relay(1);

    // Act
    let actions = announce_txid(&mut scheduler, 1, 1, 0);

    // Assert
    assert_eq!(actions, [request(1, relay_id)]);
    assert_eq!(
        actions[0].maybe_request_inventory(),
        Some(txid_inventory(1))
    );
    assert_eq!(
        scheduler.snapshot(),
        TxDownloadSnapshot {
            candidate_count: 0,
            in_flight_count: 1,
            already_have_count: 0,
        },
    );
    assert_eq!(scheduler.peer_snapshot(1).in_flight_count, 1);
}

pub(super) fn wtxid_announcement_requests_witness_transaction_inventory() {
    // Arrange
    let mut scheduler = scheduler();
    let relay_id = wtxid_relay(2);

    // Act
    let actions = announce_wtxid(&mut scheduler, 2, 2, 0);

    // Assert
    assert_eq!(actions, [request(2, relay_id)]);
    assert_eq!(
        actions[0].maybe_request_inventory(),
        Some(wtxid_inventory(2)),
    );
    assert_eq!(scheduler.peer_snapshot(2).in_flight_count, 1);
}

pub(super) fn identity_mismatch_suppresses_without_candidate_or_inflight_state() {
    // Arrange
    let mut scheduler = scheduler();
    let non_transaction_inventory = InventoryVector {
        inventory_type: InventoryType::Block,
        object_hash: Hash32::from_byte_array([3; 32]),
    };

    // Act
    let mismatch_actions = scheduler.record_announcement(announcement(
        3,
        txid_inventory(3),
        TxRelayPeerMode::WtxidRelay,
        0,
    ));
    let non_transaction_actions = scheduler.record_announcement(announcement(
        3,
        non_transaction_inventory,
        TxRelayPeerMode::TxidOnly,
        0,
    ));

    // Assert
    assert_eq!(
        mismatch_actions,
        [TxDownloadAction::SuppressIdentityMismatch {
            peer_id: 3,
            reason: TxDownloadSuppressionReason::IdentityMismatch,
        }],
    );
    assert_eq!(
        non_transaction_actions,
        [TxDownloadAction::SuppressIdentityMismatch {
            peer_id: 3,
            reason: TxDownloadSuppressionReason::NotTransactionInventory,
        }],
    );
    assert_eq!(
        scheduler.snapshot(),
        TxDownloadSnapshot {
            candidate_count: 0,
            in_flight_count: 0,
            already_have_count: 0,
        },
    );
}

pub(super) fn disabled_relay_suppresses_announcement_without_request_state() {
    // Arrange
    let mut scheduler = scheduler();
    let relay_id = txid_relay(22);

    // Act
    let actions = scheduler.record_announcement(with_relay_eligibility(
        announcement(22, txid_inventory(22), TxRelayPeerMode::TxidOnly, 0),
        relay_disabled(),
    ));

    // Assert
    assert_eq!(
        actions,
        [suppress(
            22,
            relay_id,
            TxDownloadSuppressionReason::RelayDisabled,
        )],
    );
    assert_eq!(
        scheduler.snapshot(),
        TxDownloadSnapshot {
            candidate_count: 0,
            in_flight_count: 0,
            already_have_count: 0,
        },
    );
}

pub(super) fn ineligible_relay_suppressions_are_typed_without_request_state() {
    // Arrange
    let relay_id = txid_relay(23);
    let cases = [
        (
            inbound_serving_required(),
            TxDownloadSuppressionReason::InboundServingRequired,
        ),
        (
            permission_required(),
            TxDownloadSuppressionReason::PermissionRequired,
        ),
        (
            protected_not_relay(),
            TxDownloadSuppressionReason::ProtectedNotRelay,
        ),
    ];

    for (relay_eligibility, expected_reason) in cases {
        let mut scheduler = scheduler();

        // Act
        let actions = scheduler.record_announcement(with_relay_eligibility(
            announcement(23, txid_inventory(23), TxRelayPeerMode::TxidOnly, 0),
            relay_eligibility,
        ));

        // Assert
        assert_eq!(actions, [suppress(23, relay_id, expected_reason)]);
        assert_eq!(
            scheduler.snapshot(),
            TxDownloadSnapshot {
                candidate_count: 0,
                in_flight_count: 0,
                already_have_count: 0,
            },
        );
    }
}

pub(super) fn ineligible_eligible_reason_maps_to_not_relay_eligible() {
    // Arrange
    let mut scheduler = scheduler();
    let relay_id = txid_relay(26);

    // Act
    let actions = scheduler.record_announcement(with_relay_eligibility(
        announcement(26, txid_inventory(26), TxRelayPeerMode::TxidOnly, 0),
        ineligible_with_eligible_reason(),
    ));

    // Assert
    assert_eq!(
        actions,
        [suppress(
            26,
            relay_id,
            TxDownloadSuppressionReason::NotRelayEligible,
        )],
    );
    assert_eq!(
        scheduler.snapshot(),
        TxDownloadSnapshot {
            candidate_count: 0,
            in_flight_count: 0,
            already_have_count: 0,
        },
    );
}

pub(super) fn disabled_parent_request_suppresses_without_request_state() {
    // Arrange
    let mut scheduler = scheduler();
    let relay_id = txid_relay(24);

    // Act
    let actions = scheduler.request_parent(parent_with_relay_eligibility(
        parent_request(24, relay_id, 0),
        relay_disabled(),
    ));

    // Assert
    assert_eq!(
        actions,
        [suppress(
            24,
            relay_id,
            TxDownloadSuppressionReason::RelayDisabled,
        )],
    );
    assert_eq!(
        scheduler.snapshot(),
        TxDownloadSnapshot {
            candidate_count: 0,
            in_flight_count: 0,
            already_have_count: 0,
        },
    );
}

pub(super) fn ineligible_first_announcement_does_not_block_eligible_second_announcer() {
    // Arrange
    let mut scheduler = scheduler();
    let relay_id = txid_relay(25);

    // Act
    let first_actions = scheduler.record_announcement(with_relay_eligibility(
        announcement(25, txid_inventory(25), TxRelayPeerMode::TxidOnly, 0),
        permission_required(),
    ));
    let second_actions = announce_txid(&mut scheduler, 26, 25, 1);

    // Assert
    assert_eq!(
        first_actions,
        [suppress(
            25,
            relay_id,
            TxDownloadSuppressionReason::PermissionRequired,
        )],
    );
    assert_eq!(second_actions, [request(26, relay_id)]);
    assert_eq!(scheduler.peer_snapshot(25).candidate_count, 0);
    assert_eq!(scheduler.peer_snapshot(25).in_flight_count, 0);
    assert_eq!(scheduler.peer_snapshot(26).in_flight_count, 1);
}

pub(super) fn duplicate_announcement_retains_fallback_candidate_without_second_request() {
    // Arrange
    let mut scheduler = scheduler();
    let relay_id = txid_relay(4);

    // Act
    let first_actions = announce_txid(&mut scheduler, 4, 4, 0);
    let duplicate_actions = announce_txid(&mut scheduler, 5, 4, 1);

    // Assert
    assert_eq!(first_actions, [request(4, relay_id)]);
    assert_eq!(duplicate_actions, [duplicate(5, relay_id)]);
    assert_eq!(scheduler.snapshot().candidate_count, 1);
    assert_eq!(scheduler.peer_snapshot(5).candidate_count, 1);
}

pub(super) fn orphan_parent_request_suppresses_duplicate_pending_parent_with_fallback() {
    // Arrange
    let mut scheduler = scheduler();
    let relay_id = txid_relay(5);

    // Act
    let first_actions = scheduler.request_parent(parent_request(5, relay_id, 0));
    let duplicate_actions = scheduler.request_parent(parent_request(6, relay_id, 1));
    let fallback_actions = scheduler.expire_and_schedule(60);

    // Assert
    assert_eq!(first_actions, [request(5, relay_id)]);
    assert_eq!(duplicate_actions, [duplicate(6, relay_id)]);
    assert_eq!(scheduler.snapshot().in_flight_count, 1);
    assert_eq!(scheduler.peer_snapshot(6).candidate_count, 0);
    assert_eq!(scheduler.peer_snapshot(6).in_flight_count, 1);
    assert_eq!(
        fallback_actions,
        [expect_expired(5, relay_id), fallback(6, relay_id)]
    );
}

pub(super) fn semantic_reject_facts_suppress_inventory_while_parent_bypasses_reconsiderable() {
    // Arrange
    let mut scheduler = scheduler();

    // Act
    let already_have = scheduler.record_announcement(TxAnnouncementInput {
        local_facts: TxDownloadLocalFacts {
            already_have: true,
            ..TxDownloadLocalFacts::default()
        },
        ..announcement(6, txid_inventory(6), TxRelayPeerMode::TxidOnly, 0)
    });
    let hard_reject = scheduler.record_announcement(TxAnnouncementInput {
        local_facts: TxDownloadLocalFacts {
            hard_rejected: true,
            ..TxDownloadLocalFacts::default()
        },
        ..announcement(7, txid_inventory(7), TxRelayPeerMode::TxidOnly, 0)
    });
    let reconsiderable = scheduler.record_announcement(TxAnnouncementInput {
        local_facts: TxDownloadLocalFacts {
            reconsiderable: true,
            ..TxDownloadLocalFacts::default()
        },
        ..announcement(8, txid_inventory(8), TxRelayPeerMode::TxidOnly, 0)
    });
    let mempool_known = scheduler.record_announcement(TxAnnouncementInput {
        local_facts: TxDownloadLocalFacts {
            mempool_known: true,
            ..TxDownloadLocalFacts::default()
        },
        ..announcement(9, txid_inventory(9), TxRelayPeerMode::TxidOnly, 0)
    });
    let reconsiderable_parent = scheduler.request_parent(TxParentRequestInput {
        local_facts: TxDownloadLocalFacts {
            reconsiderable: true,
            ..TxDownloadLocalFacts::default()
        },
        ..parent_request(10, txid_relay(10), 0)
    });
    let hard_rejected_parent = scheduler.request_parent(TxParentRequestInput {
        local_facts: TxDownloadLocalFacts {
            hard_rejected: true,
            ..TxDownloadLocalFacts::default()
        },
        ..parent_request(11, txid_relay(11), 0)
    });

    // Assert
    assert_eq!(already_have, [expect_already_have(6, txid_relay(6))]);
    assert_eq!(hard_reject, [expect_recent_reject(7, txid_relay(7))]);
    assert_eq!(reconsiderable, [expect_recent_reject(8, txid_relay(8))]);
    assert_eq!(
        mempool_known,
        [TxDownloadAction::Suppress {
            peer_id: 9,
            relay_id: txid_relay(9),
            reason: TxDownloadSuppressionReason::MempoolKnown,
        }],
    );
    assert_eq!(
        mempool_known[0].suppression_reason(),
        Some(TxDownloadSuppressionReason::MempoolKnown),
    );
    assert_eq!(
        reconsiderable_parent,
        [request(10, txid_relay(10))],
        "parent requests must set include_reconsiderable=false",
    );
    assert_eq!(
        hard_rejected_parent,
        [expect_recent_reject(11, txid_relay(11))],
    );
}

pub(super) fn inflight_cap_suppresses_additional_ready_requests() {
    // Arrange
    let mut scheduler = scheduler();

    // Act
    let first_actions = announce_txid(&mut scheduler, 9, 9, 0);
    let capped_actions = announce_txid(&mut scheduler, 9, 10, 1);

    // Assert
    assert_eq!(first_actions.len(), 1);
    assert_eq!(capped_actions, [request_cap(9, txid_relay(10))]);
    assert_eq!(
        scheduler.peer_snapshot(9),
        TxPeerRequestSnapshot {
            peer_id: 9,
            candidate_count: 0,
            in_flight_count: 1,
        },
    );
}

pub(super) fn txid_delay_waits_until_fake_clock_reaches_ready_time() {
    // Arrange
    let mut scheduler = scheduler();
    let relay_id = txid_relay(11);
    let _ = announce_wtxid(&mut scheduler, 10, 11, 0);

    // Act
    let delayed = scheduler.record_announcement(announcement(
        11,
        txid_inventory(11),
        TxRelayPeerMode::TxidOnly,
        0,
    ));
    let too_early = scheduler.expire_and_schedule(1);
    let ready = scheduler.expire_and_schedule(2);

    // Assert
    assert!(delayed.is_empty());
    assert!(too_early.is_empty());
    assert_eq!(ready, [request(11, relay_id)]);
}

pub(super) fn non_preferred_peer_delay_waits_until_fake_clock_reaches_ready_time() {
    // Arrange
    let mut scheduler = scheduler();
    let relay_id = txid_relay(12);

    // Act
    let delayed = scheduler.record_announcement(not_preferred(announcement(
        12,
        txid_inventory(12),
        TxRelayPeerMode::TxidOnly,
        0,
    )));
    let too_early = scheduler.expire_and_schedule(1);
    let ready = scheduler.expire_and_schedule(2);

    // Assert
    assert!(delayed.is_empty());
    assert!(too_early.is_empty());
    assert_eq!(ready, [request(12, relay_id)]);
}

pub(super) fn overloaded_peer_delay_waits_until_fake_clock_reaches_ready_time() {
    // Arrange
    let mut scheduler = scheduler();
    let relay_id = txid_relay(13);

    // Act
    let delayed = scheduler.record_announcement(overloaded(announcement(
        13,
        txid_inventory(13),
        TxRelayPeerMode::TxidOnly,
        0,
    )));
    let too_early = scheduler.expire_and_schedule(1);
    let ready = scheduler.expire_and_schedule(2);

    // Assert
    assert!(delayed.is_empty());
    assert!(too_early.is_empty());
    assert_eq!(ready, [request(13, relay_id)]);
}

pub(super) fn expiry_fallback_waits_until_fake_clock_reaches_getdata_interval() {
    // Arrange
    let mut scheduler = scheduler();
    let relay_id = txid_relay(14);
    let _ = announce_txid(&mut scheduler, 14, 14, 0);
    let _ = announce_txid(&mut scheduler, 15, 14, 1);

    // Act
    let too_early = scheduler.expire_and_schedule(59);
    let expired = scheduler.expire_and_schedule(60);

    // Assert
    assert!(too_early.is_empty());
    assert_eq!(
        expired,
        [expect_expired(14, relay_id), fallback(15, relay_id)]
    );
}

pub(super) fn timeout_expires_request_and_falls_back_to_duplicate_announcer() {
    // Arrange
    let mut scheduler = scheduler();
    let relay_id = txid_relay(16);
    let _ = announce_txid(&mut scheduler, 16, 16, 0);
    let _ = announce_txid(&mut scheduler, 17, 16, 0);

    // Act
    let actions = scheduler.expire_and_schedule(60);

    // Assert
    assert_eq!(
        actions,
        [expect_expired(16, relay_id), fallback(17, relay_id)]
    );
    assert_eq!(scheduler.snapshot().in_flight_count, 1);
    assert_eq!(scheduler.peer_snapshot(17).in_flight_count, 1);
}

pub(super) fn notfound_clears_matching_request_and_falls_back() {
    // Arrange
    let mut scheduler = scheduler();
    let relay_id = txid_relay(18);
    let _ = announce_txid(&mut scheduler, 18, 18, 0);
    let _ = announce_txid(&mut scheduler, 19, 18, 1);

    // Act
    let actions = scheduler.record_notfound(18, relay_id, 5);

    // Assert
    assert_eq!(
        actions,
        [
            TxDownloadAction::NotFoundCleanup {
                peer_id: 18,
                relay_id,
            },
            fallback(19, relay_id),
        ],
    );
}

pub(super) fn disconnect_cleanup_removes_peer_state_and_falls_back() {
    // Arrange
    let mut scheduler = scheduler();
    let relay_id = txid_relay(20);
    let _ = announce_txid(&mut scheduler, 20, 20, 0);
    let _ = announce_txid(&mut scheduler, 21, 20, 1);

    // Act
    let actions = scheduler.cleanup_peer(20, 2);

    // Assert
    assert_eq!(
        actions,
        [
            TxDownloadAction::PeerCleanup { peer_id: 20 },
            fallback(21, relay_id),
        ],
    );
    assert_eq!(scheduler.peer_snapshot(20).in_flight_count, 0);
    assert_eq!(scheduler.peer_snapshot(21).in_flight_count, 1);
}

pub(super) fn received_transaction_cleanup_waits_for_admission_before_already_have() {
    received_cases::received_transaction_cleanup_waits_for_admission_before_already_have();
}
