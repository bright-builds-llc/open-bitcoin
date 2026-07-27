// Parity breadcrumbs:
// - packages/bitcoin-knots/src/protocol.h
// - packages/bitcoin-knots/src/protocol.cpp
// - packages/bitcoin-knots/src/netaddress.h
// - packages/bitcoin-knots/src/netaddress.cpp
// - packages/bitcoin-knots/src/net.h
// - packages/bitcoin-knots/src/net.cpp
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/addrman.h
// - packages/bitcoin-knots/src/addrman.cpp
// - packages/bitcoin-knots/src/addrdb.h
// - packages/bitcoin-knots/src/addrdb.cpp

use super::*;

#[test]
fn learned_addresses_reject_invalid_freshness_duplicate_and_unroutable_inputs() {
    // Arrange
    let mut book = LearnedAddressBook::default();
    let now_unix_seconds = 1_700_000_000;
    let first = address_announcement(
        now_unix_seconds,
        public_ipv4_network_address(8, 8, 4, 4, 8333),
    );
    let stale = address_announcement(
        now_unix_seconds - PHASE92_MAX_ADDR_AGE_SECONDS - 1,
        public_ipv4_network_address(8, 8, 8, 8, 8333),
    );
    let future = address_announcement(
        now_unix_seconds + PHASE92_MAX_FUTURE_SKEW_SECONDS + 1,
        public_ipv4_network_address(1, 1, 1, 1, 8333),
    );
    let invalid_port =
        address_announcement(now_unix_seconds, public_ipv4_network_address(8, 8, 8, 8, 0));
    let loopback = address_announcement(
        now_unix_seconds,
        public_ipv4_network_address(127, 0, 0, 1, 8333),
    );
    let private = address_announcement(
        now_unix_seconds,
        public_ipv4_network_address(10, 0, 0, 1, 8333),
    );
    let documentation = address_announcement(
        now_unix_seconds,
        public_ipv4_network_address(192, 0, 2, 1, 8333),
    );

    // Act
    let accepted = book.learn_batch(
        core::slice::from_ref(&first),
        AddressSourceKind::InboundAddr,
        now_unix_seconds,
    );
    let rejected = book.learn_batch(
        &[
            invalid_port,
            stale,
            future,
            first,
            loopback,
            private,
            documentation,
        ],
        AddressSourceKind::InboundAddr,
        now_unix_seconds,
    );

    // Assert
    assert_eq!(accepted.accepted_count, 1);
    assert_eq!(rejected.accepted_count, 0);
    assert_eq!(
        rejected
            .decisions
            .iter()
            .map(|decision| decision.label)
            .collect::<Vec<_>>(),
        vec![
            AddressDecisionLabel::LearnedRejected,
            AddressDecisionLabel::LearnedRejected,
            AddressDecisionLabel::LearnedRejected,
            AddressDecisionLabel::LearnedRejected,
            AddressDecisionLabel::LearnedRejected,
            AddressDecisionLabel::LearnedRejected,
            AddressDecisionLabel::LearnedRejected,
        ],
    );
    assert_eq!(
        rejected
            .decisions
            .iter()
            .map(|decision| decision.reason)
            .collect::<Vec<_>>(),
        vec![
            AddressDecisionReason::InvalidPort,
            AddressDecisionReason::StaleOrFuture,
            AddressDecisionReason::StaleOrFuture,
            AddressDecisionReason::DuplicateAddress,
            AddressDecisionReason::NotPubliclyRoutable,
            AddressDecisionReason::NotPubliclyRoutable,
            AddressDecisionReason::NotPubliclyRoutable,
        ],
    );
    assert!(
        rejected
            .decisions
            .iter()
            .all(|decision| decision.maybe_entry.is_none())
    );
    assert_eq!(book.entries().len(), 1);
}

#[test]
fn learned_address_batches_above_phase92_limit_are_rejected_without_partial_inserts() {
    // Arrange
    let mut book = LearnedAddressBook::default();
    let now_unix_seconds = 1_700_000_000;
    let announcements: Vec<_> = (0..=PHASE92_LEARNED_ADDR_BATCH_LIMIT)
        .map(|index| {
            address_announcement(
                now_unix_seconds,
                public_ipv4_network_address(8, 8, 8, index as u8, 8333),
            )
        })
        .collect();

    // Act
    let batch = book.learn_batch(
        &announcements,
        AddressSourceKind::InboundAddr,
        now_unix_seconds,
    );

    // Assert
    assert_eq!(batch.label, AddressDecisionLabel::LearnedRejected);
    assert_eq!(batch.reason, AddressDecisionReason::OverCapBatch);
    assert_eq!(batch.accepted_count, 0);
    assert_eq!(batch.rejected_count, announcements.len());
    assert!(batch.decisions.is_empty());
    assert!(book.entries().is_empty());
}
