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

use crate::{RelayEligibilityDecision, RelayEligibilityReason};

use super::*;

fn eligible_relay() -> RelayEligibilityDecision {
    RelayEligibilityDecision {
        eligible: true,
        reason: RelayEligibilityReason::Eligible,
        relay_permission_effects: Vec::new(),
        version_message_relay: true,
    }
}

fn disabled_relay() -> RelayEligibilityDecision {
    RelayEligibilityDecision {
        eligible: false,
        reason: RelayEligibilityReason::Disabled,
        relay_permission_effects: Vec::new(),
        version_message_relay: false,
    }
}

fn txid_inventory(byte: u8) -> InventoryVector {
    TxRelayId::Txid(txid(byte)).to_inventory_vector()
}

fn wtxid_inventory(byte: u8) -> InventoryVector {
    TxRelayId::Wtxid(wtxid(byte)).to_inventory_vector()
}

pub(super) fn tx_serving_policy_reports_low_cardinality_outcomes() {
    // Arrange
    let relay = eligible_relay();
    let cases = [
        (
            Some(TxServingRecordStatus::Accepted),
            TxServeOutcomeLabel::Served,
        ),
        (None, TxServeOutcomeLabel::Unknown),
        (
            Some(TxServingRecordStatus::Stale),
            TxServeOutcomeLabel::Stale,
        ),
        (
            Some(TxServingRecordStatus::Confirmed),
            TxServeOutcomeLabel::Confirmed,
        ),
        (
            Some(TxServingRecordStatus::Rejected),
            TxServeOutcomeLabel::Rejected,
        ),
        (
            Some(TxServingRecordStatus::Replaced),
            TxServeOutcomeLabel::Replaced,
        ),
        (
            Some(TxServingRecordStatus::Evicted),
            TxServeOutcomeLabel::Evicted,
        ),
        (
            Some(TxServingRecordStatus::Expired),
            TxServeOutcomeLabel::Expired,
        ),
    ];

    // Act
    let decisions: Vec<TxServeDecision> = cases
        .iter()
        .map(|(maybe_record_status, _outcome)| {
            classify_tx_serve_request(
                &txid_inventory(1),
                TxRelayPeerMode::TxidOnly,
                &relay,
                *maybe_record_status,
            )
        })
        .collect();
    let disabled = classify_tx_serve_request(
        &txid_inventory(1),
        TxRelayPeerMode::TxidOnly,
        &disabled_relay(),
        Some(TxServingRecordStatus::Accepted),
    );
    let labels: Vec<&'static str> = decisions
        .iter()
        .map(|decision| decision.outcome.as_str())
        .chain([disabled.outcome.as_str()])
        .collect();

    // Assert
    assert_eq!(
        decisions
            .iter()
            .map(|decision| decision.outcome)
            .collect::<Vec<_>>(),
        cases
            .iter()
            .map(|(_status, outcome)| *outcome)
            .collect::<Vec<_>>(),
    );
    assert_eq!(
        labels,
        [
            "served",
            "unknown",
            "stale",
            "confirmed",
            "rejected",
            "replaced",
            "evicted",
            "expired",
            "not_relay_eligible",
        ],
    );
    assert_eq!(decisions[0].maybe_relay_id, Some(TxRelayId::Txid(txid(1))));
    assert_eq!(disabled.outcome, TxServeOutcomeLabel::NotRelayEligible);
}

pub(super) fn tx_serving_policy_rejects_identity_mismatch_and_non_transaction_inventory() {
    // Arrange
    let relay = eligible_relay();
    let block_inventory = InventoryVector {
        inventory_type: InventoryType::Block,
        object_hash: Hash32::from_byte_array([9; 32]),
    };

    // Act
    let mismatch = classify_tx_serve_request(
        &wtxid_inventory(2),
        TxRelayPeerMode::TxidOnly,
        &relay,
        Some(TxServingRecordStatus::Accepted),
    );
    let non_transaction = classify_tx_serve_request(
        &block_inventory,
        TxRelayPeerMode::TxidOnly,
        &relay,
        Some(TxServingRecordStatus::Accepted),
    );

    // Assert
    assert_eq!(mismatch.outcome, TxServeOutcomeLabel::IdentityMismatch);
    assert_eq!(
        non_transaction.outcome,
        TxServeOutcomeLabel::NotTransactionInventory,
    );
    assert_eq!(mismatch.maybe_relay_id, None);
    assert_eq!(non_transaction.maybe_relay_id, None);
    assert_eq!(mismatch.outcome.as_str(), "identity_mismatch");
    assert_eq!(
        non_transaction.outcome.as_str(),
        "not_transaction_inventory"
    );
}
