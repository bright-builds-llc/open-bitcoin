// Parity breadcrumbs:
// - packages/bitcoin-knots/src/headerssync.cpp
// - packages/bitcoin-knots/src/sync.cpp
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/node/txdownloadman_impl.cpp
// - packages/bitcoin-knots/src/node/txdownloadman.h
// - packages/bitcoin-knots/test/functional/p2p_handshake.py
// - packages/bitcoin-knots/test/functional/p2p_initial_headers_sync.py

use super::*;

#[test]
fn inbound_addr_messages_update_learned_address_evidence_without_actions() {
    // Arrange
    let mut manager = PeerManager::new(local_config());
    manager.add_inbound_peer(93).expect("peer should be added");
    let now_unix_seconds = 1_700_000_000;
    let addresses = AddressList {
        addresses: vec![
            address_announcement(
                now_unix_seconds,
                public_ipv4_network_address(8, 8, 8, 8, 8333),
            ),
            address_announcement(
                now_unix_seconds,
                public_ipv6_network_address("2606:4700:4700::1111", 8333),
            ),
        ],
    };

    // Act
    let addr_actions = manager
        .handle_message(
            93,
            WireNetworkMessage::Addr(addresses),
            now_unix_seconds as i64,
        )
        .expect("addr should be learned");
    let evidence = manager.address_boundary_evidence();

    // Assert
    assert!(addr_actions.is_empty());
    assert_eq!(evidence.learned_address_entries.len(), 2);
    assert_eq!(evidence.learned_address_rejection_count, 0);
    assert!(evidence.learned_address_entries.iter().all(|entry| {
        entry.source == AddressSourceKind::InboundAddr
            && entry.routability == RoutabilityClass::PubliclyRoutable
            && entry.persistence_eligible
    }));
    assert!(evidence.learned_address_rejections.is_empty());
    assert_eq!(
        evidence
            .maybe_latest_address_decision
            .expect("latest decision")
            .label
            .as_str(),
        "learned_accepted",
    );
}

#[test]
fn inbound_addr_rejections_are_recorded_without_disconnect_actions() {
    // Arrange
    let mut manager = PeerManager::new(local_config());
    manager.add_inbound_peer(94).expect("peer should be added");
    let now_unix_seconds = 1_700_000_000;
    let accepted = address_announcement(
        now_unix_seconds,
        public_ipv4_network_address(8, 8, 4, 4, 8333),
    );
    manager
        .handle_message(
            94,
            WireNetworkMessage::Addr(AddressList {
                addresses: vec![accepted.clone()],
            }),
            now_unix_seconds as i64,
        )
        .expect("seed address should be learned");
    let rejected_addresses = AddressList {
        addresses: vec![
            address_announcement(now_unix_seconds, public_ipv4_network_address(8, 8, 8, 8, 0)),
            address_announcement(
                now_unix_seconds - crate::PHASE92_MAX_ADDR_AGE_SECONDS - 1,
                public_ipv4_network_address(8, 8, 8, 8, 8333),
            ),
            address_announcement(
                now_unix_seconds + crate::PHASE92_MAX_FUTURE_SKEW_SECONDS + 1,
                public_ipv4_network_address(1, 1, 1, 1, 8333),
            ),
            accepted,
            address_announcement(
                now_unix_seconds,
                public_ipv4_network_address(127, 0, 0, 1, 8333),
            ),
            address_announcement(
                now_unix_seconds,
                public_ipv4_network_address(10, 0, 0, 1, 8333),
            ),
            address_announcement(
                now_unix_seconds,
                public_ipv4_network_address(192, 0, 2, 1, 8333),
            ),
        ],
    };

    // Act
    let actions = manager
        .handle_message(
            94,
            WireNetworkMessage::Addr(rejected_addresses),
            now_unix_seconds as i64,
        )
        .expect("addr rejections should be evidence only");
    let evidence = manager.address_boundary_evidence();

    // Assert
    assert!(actions.is_empty());
    assert_eq!(evidence.learned_address_entries.len(), 1);
    assert_eq!(evidence.learned_address_rejection_count, 7);
    assert_eq!(
        evidence
            .learned_address_rejections
            .iter()
            .map(|decision| decision.label.as_str())
            .collect::<Vec<_>>(),
        vec![
            "learned_rejected",
            "learned_rejected",
            "learned_rejected",
            "learned_rejected",
            "learned_rejected",
            "learned_rejected",
            "learned_rejected",
        ],
    );
    assert_eq!(
        evidence
            .learned_address_rejections
            .iter()
            .map(|decision| decision.reason.as_str())
            .collect::<Vec<_>>(),
        vec![
            "invalid_port",
            "stale_or_future",
            "stale_or_future",
            "duplicate_address",
            "not_publicly_routable",
            "not_publicly_routable",
            "not_publicly_routable",
        ],
    );
}

#[test]
fn over_cap_addr_batch_records_batch_rejection_without_partial_inserts() {
    // Arrange
    let mut manager = PeerManager::new(local_config());
    manager.add_inbound_peer(95).expect("peer should be added");
    let now_unix_seconds = 1_700_000_000;
    let addresses = (0..=PHASE92_LEARNED_ADDR_BATCH_LIMIT)
        .map(|index| {
            address_announcement(
                now_unix_seconds,
                public_ipv4_network_address(8, 8, 8, index as u8, 8333),
            )
        })
        .collect();

    // Act
    let actions = manager
        .handle_message(
            95,
            WireNetworkMessage::Addr(AddressList { addresses }),
            now_unix_seconds as i64,
        )
        .expect("over-cap addr should be rejected as evidence");
    let evidence = manager.address_boundary_evidence();
    let latest = evidence
        .maybe_latest_address_decision
        .expect("batch rejection should be latest decision");

    // Assert
    assert!(actions.is_empty());
    assert!(evidence.learned_address_entries.is_empty());
    assert_eq!(
        evidence.learned_address_rejection_count,
        PHASE92_LEARNED_ADDR_BATCH_LIMIT + 1,
    );
    assert!(evidence.learned_address_rejections.is_empty());
    assert_eq!(latest.label, AddressDecisionLabel::LearnedRejected);
    assert_eq!(latest.reason, AddressDecisionReason::OverCapBatch);
    assert_eq!(latest.label.as_str(), "learned_rejected");
    assert_eq!(latest.reason.as_str(), "over_cap_batch");
}

#[test]
fn outbound_addr_messages_parse_without_response_or_relay_actions() {
    // Arrange
    let mut manager = PeerManager::new(local_config());
    manager
        .add_outbound_peer(96, 1)
        .expect("outbound peer should be added");
    let now_unix_seconds = 1_700_000_000;
    let addresses = AddressList {
        addresses: vec![address_announcement(
            now_unix_seconds,
            public_ipv4_network_address(8, 8, 8, 8, 8333),
        )],
    };

    // Act
    let actions = manager
        .handle_message(
            96,
            WireNetworkMessage::Addr(addresses),
            now_unix_seconds as i64,
        )
        .expect("outbound addr should parse");
    let evidence = manager.address_boundary_evidence();

    // Assert
    assert!(actions.is_empty());
    assert_eq!(evidence.learned_address_entries.len(), 1);
    assert!(evidence.getaddr_responses_served.is_empty());
    assert!(evidence.getaddr_requests_suppressed.is_empty());
}

#[test]
fn addr_unknown_peer_empty_and_local_duplicate_paths_are_evidence_only() {
    // Arrange
    let mut manager = PeerManager::new(local_config());
    manager.add_inbound_peer(97).expect("peer should be added");
    let local_address = public_ipv4_network_address(8, 8, 8, 8, 8333);
    manager.local_address_decisions = vec![LocalAdvertisementDecision {
        label: AddressDecisionLabel::AdvertiseCandidate,
        reason: AddressDecisionReason::PolicyAccepted,
        source: AddressSourceKind::LocalListener,
        network_kind: AddressNetworkKind::Ipv4,
        routability: RoutabilityClass::PubliclyRoutable,
        services_bits: (ServiceFlags::NETWORK | ServiceFlags::WITNESS).bits(),
        port: local_address.port,
        maybe_wire_address: Some(local_address.clone()),
    }];

    // Act
    let unknown_peer_error = manager
        .handle_message(404, WireNetworkMessage::Addr(AddressList::default()), -1)
        .expect_err("unknown peer should fail");
    let empty_actions = manager
        .handle_message(97, WireNetworkMessage::Addr(AddressList::default()), -1)
        .expect("empty addr should be evidence only");
    let duplicate_actions = manager
        .handle_message(
            97,
            WireNetworkMessage::Addr(AddressList {
                addresses: vec![address_announcement(0, local_address)],
            }),
            -1,
        )
        .expect("duplicate local addr should be evidence only");
    let evidence = manager.address_boundary_evidence();

    // Assert
    assert_eq!(unknown_peer_error.to_string(), "unknown peer: 404");
    assert!(empty_actions.is_empty());
    assert!(duplicate_actions.is_empty());
    assert!(evidence.learned_address_entries.is_empty());
    assert_eq!(evidence.learned_address_rejections.len(), 1);
    assert_eq!(
        evidence.learned_address_rejections[0].reason,
        AddressDecisionReason::DuplicateAddress,
    );
    assert_eq!(
        evidence
            .maybe_latest_address_decision
            .expect("local duplicate latest decision")
            .reason
            .as_str(),
        "duplicate_address",
    );
}

#[test]
fn permissioned_inbound_getaddr_serves_once_and_records_repeated_suppression() {
    // Arrange
    let mut manager = PeerManager::new(local_config());
    let now_unix_seconds = 1_700_000_000;
    let local_address = public_ipv4_network_address(9, 9, 9, 9, 8333);
    manager.set_local_address_decisions(vec![local_advertisement_candidate(local_address.clone())]);
    manager.add_inbound_peer(98).expect("seed peer");
    let learned_addresses = (0..(PHASE92_GETADDR_RESPONSE_LIMIT + 2))
        .map(|index| {
            address_announcement(
                now_unix_seconds,
                public_ipv4_network_address(11, 0, 0, (index + 1) as u8, 8333),
            )
        })
        .collect();
    manager
        .handle_message(
            98,
            WireNetworkMessage::Addr(AddressList {
                addresses: learned_addresses,
            }),
            now_unix_seconds as i64,
        )
        .expect("seed learned addresses");
    manager
        .add_inbound_peer_record(permissioned_inbound_record(
            99,
            permission_decision(["in", "addr"]),
        ))
        .expect("permissioned addr peer");

    // Act
    let first_actions = manager
        .handle_message(99, WireNetworkMessage::GetAddr, now_unix_seconds as i64)
        .expect("first getaddr should be served");
    let second_actions = manager
        .handle_message(99, WireNetworkMessage::GetAddr, now_unix_seconds as i64)
        .expect("second getaddr should be suppressed");
    let evidence = manager.address_boundary_evidence();

    // Assert
    let [PeerAction::Send(WireNetworkMessage::Addr(response))] = first_actions.as_slice() else {
        panic!("expected getaddr addr response");
    };
    assert_eq!(response.addresses.len(), PHASE92_GETADDR_RESPONSE_LIMIT);
    assert_eq!(response.addresses[0].address, local_address);
    assert!(second_actions.is_empty());
    let GetAddrResponseDecision::Served {
        label,
        reason,
        entries,
    } = &evidence.getaddr_responses_served[0]
    else {
        panic!("expected getaddr served evidence");
    };
    assert_eq!(label.as_str(), "getaddr_served");
    assert_eq!(*reason, AddressDecisionReason::PolicyAccepted);
    assert_eq!(entries.len(), PHASE92_GETADDR_RESPONSE_LIMIT);
    let GetAddrResponseDecision::Suppressed { label, reason } =
        &evidence.getaddr_requests_suppressed[0]
    else {
        panic!("expected getaddr suppressed evidence");
    };
    assert_eq!(label.as_str(), "getaddr_suppressed");
    assert_eq!(reason.as_str(), "already_served");
}

#[test]
fn getaddr_suppression_records_permission_and_outbound_reasons() {
    // Arrange
    let mut manager = PeerManager::new(local_config());
    manager
        .add_inbound_peer(100)
        .expect("ordinary inbound peer");
    manager
        .add_outbound_peer(101, 1)
        .expect("outbound peer should be added");

    // Act
    let ordinary_actions = manager
        .handle_message(100, WireNetworkMessage::GetAddr, 2)
        .expect("ordinary inbound getaddr should be suppressed");
    let outbound_actions = manager
        .handle_message(101, WireNetworkMessage::GetAddr, 3)
        .expect("outbound getaddr should be suppressed");
    let evidence = manager.address_boundary_evidence();

    // Assert
    assert!(ordinary_actions.is_empty());
    assert!(outbound_actions.is_empty());
    assert_eq!(
        evidence
            .getaddr_requests_suppressed
            .iter()
            .map(|decision| match decision {
                GetAddrResponseDecision::Suppressed { reason, .. } => reason.as_str(),
                GetAddrResponseDecision::Served { .. } => "unexpected_served",
            })
            .collect::<Vec<_>>(),
        vec!["permission_policy_denied", "not_inbound"],
    );
}

#[test]
fn ordinary_peer_flows_do_not_send_unsolicited_addr_messages() {
    // Arrange
    let mut manager = PeerManager::new(local_config());
    manager.set_local_address_decisions(vec![local_advertisement_candidate(
        public_ipv4_network_address(13, 0, 0, 1, 8333),
    )]);
    manager.add_inbound_peer(103).expect("inbound peer");

    // Act
    let version_actions = manager
        .handle_message(
            103,
            WireNetworkMessage::Version(crate::VersionMessage {
                start_height: 0,
                ..crate::VersionMessage::default()
            }),
            10,
        )
        .expect("version should process");
    let verack_actions = manager
        .handle_message(103, WireNetworkMessage::Verack, 11)
        .expect("verack should process");
    let ping_actions = manager
        .handle_message(103, WireNetworkMessage::Ping { nonce: 7 }, 12)
        .expect("ping should process");
    let inv_actions = manager
        .handle_message(
            103,
            WireNetworkMessage::Inv(InventoryList::new(Vec::new())),
            13,
        )
        .expect("inventory should process");
    let headers_actions = manager
        .handle_message(
            103,
            WireNetworkMessage::Headers(HeadersMessage {
                headers: Vec::new(),
            }),
            14,
        )
        .expect("headers should process");

    // Assert
    for actions in [
        version_actions,
        verack_actions,
        ping_actions,
        inv_actions,
        headers_actions,
    ] {
        assert_no_addr_actions(&actions);
    }
}
