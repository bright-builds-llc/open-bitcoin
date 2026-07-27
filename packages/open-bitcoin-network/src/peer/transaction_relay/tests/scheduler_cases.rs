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
// - packages/bitcoin-knots/test/functional/p2p_opportunistic_1p1c.py
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

mod eligibility_cases;
mod lifecycle_cases;
mod receipt_cases;
mod request_cases;
mod scheduling_cases;

pub(super) fn txid_announcement_requests_transaction_inventory() {
    request_cases::txid_announcement_requests_transaction_inventory();
}

pub(super) fn wtxid_announcement_requests_witness_transaction_inventory() {
    request_cases::wtxid_announcement_requests_witness_transaction_inventory();
}

pub(super) fn semantic_reject_facts_suppress_inventory_while_parent_bypasses_reconsiderable() {
    request_cases::semantic_reject_facts_suppress_inventory_while_parent_bypasses_reconsiderable();
}

pub(super) fn identity_mismatch_suppresses_without_candidate_or_inflight_state() {
    eligibility_cases::identity_mismatch_suppresses_without_candidate_or_inflight_state();
}

pub(super) fn disabled_relay_suppresses_announcement_without_request_state() {
    eligibility_cases::disabled_relay_suppresses_announcement_without_request_state();
}

pub(super) fn ineligible_relay_suppressions_are_typed_without_request_state() {
    eligibility_cases::ineligible_relay_suppressions_are_typed_without_request_state();
}

pub(super) fn ineligible_eligible_reason_maps_to_not_relay_eligible() {
    eligibility_cases::ineligible_eligible_reason_maps_to_not_relay_eligible();
}

pub(super) fn disabled_parent_request_suppresses_without_request_state() {
    eligibility_cases::disabled_parent_request_suppresses_without_request_state();
}

pub(super) fn ineligible_first_announcement_does_not_block_eligible_second_announcer() {
    eligibility_cases::ineligible_first_announcement_does_not_block_eligible_second_announcer();
}

pub(super) fn duplicate_announcement_retains_fallback_candidate_without_second_request() {
    scheduling_cases::duplicate_announcement_retains_fallback_candidate_without_second_request();
}

pub(super) fn orphan_parent_request_suppresses_duplicate_pending_parent_with_fallback() {
    scheduling_cases::orphan_parent_request_suppresses_duplicate_pending_parent_with_fallback();
}

pub(super) fn inflight_cap_suppresses_additional_ready_requests() {
    scheduling_cases::inflight_cap_suppresses_additional_ready_requests();
}

pub(super) fn txid_delay_waits_until_fake_clock_reaches_ready_time() {
    lifecycle_cases::txid_delay_waits_until_fake_clock_reaches_ready_time();
}

pub(super) fn non_preferred_peer_delay_waits_until_fake_clock_reaches_ready_time() {
    lifecycle_cases::non_preferred_peer_delay_waits_until_fake_clock_reaches_ready_time();
}

pub(super) fn overloaded_peer_delay_waits_until_fake_clock_reaches_ready_time() {
    lifecycle_cases::overloaded_peer_delay_waits_until_fake_clock_reaches_ready_time();
}

pub(super) fn expiry_fallback_waits_until_fake_clock_reaches_getdata_interval() {
    lifecycle_cases::expiry_fallback_waits_until_fake_clock_reaches_getdata_interval();
}

pub(super) fn timeout_expires_request_and_falls_back_to_duplicate_announcer() {
    lifecycle_cases::timeout_expires_request_and_falls_back_to_duplicate_announcer();
}

pub(super) fn notfound_clears_matching_request_and_falls_back() {
    lifecycle_cases::notfound_clears_matching_request_and_falls_back();
}

pub(super) fn disconnect_cleanup_removes_peer_state_and_falls_back() {
    lifecycle_cases::disconnect_cleanup_removes_peer_state_and_falls_back();
}

pub(super) fn received_transaction_cleanup_waits_for_admission_before_already_have() {
    receipt_cases::received_transaction_cleanup_waits_for_admission_before_already_have();
}

pub(super) fn different_deliverer_receipt_unions_and_orders_announcers() {
    receipt_cases::different_deliverer_receipt_unions_and_orders_announcers();
}

pub(super) fn receipt_provenance_deduplicates_delivered_by_announcer() {
    receipt_cases::receipt_provenance_deduplicates_delivered_by_announcer();
}
