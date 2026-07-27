// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/node/txdownloadman_impl.cpp
// - packages/bitcoin-knots/src/node/txdownloadman.h
// - packages/bitcoin-knots/src/node/mempool_persist.cpp
// - packages/bitcoin-knots/src/protocol.h
// - packages/bitcoin-knots/src/txorphanage.cpp
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/test/functional/mempool_persist.py
// - packages/bitcoin-knots/test/functional/p2p_getdata.py
// - packages/bitcoin-knots/test/functional/p2p_orphan_handling.py
// - packages/bitcoin-knots/test/functional/p2p_tx_download.py
// - packages/bitcoin-knots/test/functional/mempool_accept.py

use super::*;

#[test]
fn managed_address_boundary_info_projects_peer_manager_evidence() {
    // Arrange
    let services = ServiceFlags::NETWORK | ServiceFlags::WITNESS;
    let local_address = public_ipv4_network_address(8, 8, 8, 8, 18_444, services);
    let mut network = ManagedPeerNetwork::new(
        MemoryChainstateStore::default(),
        local_config(301),
        PolicyConfig::default(),
    );
    network.set_local_address_decisions(vec![
        local_advertisement_candidate(local_address),
        suppressed_local_advertisement(18_446, services),
    ]);
    let decision = network.admit_inbound_peer(permissioned_inbound_request(
        301,
        "127.0.0.1:18444",
        &["in", "addr"],
    ));
    assert!(matches!(decision, InboundAdmissionDecision::Admit(_)));
    let served = network
        .receive_message(
            301,
            WireNetworkMessage::GetAddr,
            101,
            verify_flags(),
            consensus_params(),
        )
        .expect("first getaddr should be served")
        .outbound;
    let suppressed = network
        .receive_message(
            301,
            WireNetworkMessage::GetAddr,
            101,
            verify_flags(),
            consensus_params(),
        )
        .expect("second getaddr should be suppressed")
        .outbound;

    // Act
    let info = network.address_boundary_info();

    // Assert
    assert!(matches!(
        served.as_slice(),
        [WireNetworkMessage::Addr(addresses)] if addresses.addresses.len() == 1
    ));
    assert!(suppressed.is_empty());
    assert_eq!(info.local_advertisement_candidates.len(), 1);
    assert_eq!(
        info.local_advertisement_candidates[0].source,
        "source_local_listener"
    );
    assert_eq!(info.local_advertisement_candidates[0].network_kind, "ipv4");
    assert_eq!(
        info.local_advertisement_candidates[0].routability,
        "publicly_routable"
    );
    assert_eq!(info.local_advertisement_candidates[0].freshness, "fresh");
    assert_eq!(
        info.local_advertisement_candidates[0].services_bits,
        services.bits()
    );
    assert_eq!(info.local_advertisement_candidates[0].port, 18_444);
    assert!(!info.local_advertisement_candidates[0].persistence_eligible);
    assert_eq!(info.suppressed_advertisements.len(), 1);
    assert_eq!(
        info.suppressed_advertisements[0].label,
        "advertise_suppressed"
    );
    assert_eq!(
        info.suppressed_advertisements[0].reason,
        "permission_policy_denied"
    );
    assert_eq!(info.getaddr_responses_served, 1);
    assert_eq!(info.getaddr_requests_suppressed, 1);
    assert_eq!(info.learned_address_entries, 0);
    assert_eq!(info.learned_address_rejections, 0);
    let latest = info
        .maybe_latest_address_decision
        .expect("latest address decision");
    assert_eq!(latest.label, "getaddr_suppressed");
    assert_eq!(latest.reason, "already_served");
}

#[test]
fn managed_peer_policy_info_projects_eviction_candidate_evidence() {
    // Arrange
    let mut network = ManagedPeerNetwork::new(
        MemoryChainstateStore::default(),
        local_config(401),
        PolicyConfig::default(),
    );
    network.add_inbound_peer(401).expect("peer should be added");

    // Act
    let info = network.peer_policy_info();

    // Assert
    assert_eq!(info.eviction_candidates_evaluated, 1);
    assert_eq!(info.disconnects_requested, 1);
    assert_eq!(info.protected_no_actions, 0);
    let latest = info
        .maybe_latest_peer_policy_decision
        .expect("latest policy decision");
    assert_eq!(latest.label, "eviction_candidate_selected");
    assert_eq!(latest.source, "source_eviction_policy");
    assert!(!latest.message.contains("peer-"));
}

#[test]
fn managed_peer_policy_info_projects_protected_eviction_suppression() {
    // Arrange
    let mut network = ManagedPeerNetwork::new(
        MemoryChainstateStore::default(),
        local_config(402),
        PolicyConfig::default(),
    );
    network.set_inbound_admission_policy(InboundAdmissionPolicy::new(2, 1));
    let decision = network.admit_inbound_peer(permissioned_inbound_request(
        402,
        "127.0.0.1:18444",
        &["in", "noban", "forceinbound"],
    ));
    assert!(matches!(decision, InboundAdmissionDecision::Admit(_)));

    // Act
    let info = network.peer_policy_info();

    // Assert
    assert_eq!(info.eviction_candidates_evaluated, 1);
    assert_eq!(info.disconnects_requested, 0);
    assert_eq!(info.protected_no_actions, 1);
    let latest = info
        .maybe_latest_peer_policy_decision
        .expect("latest policy decision");
    assert_eq!(latest.label, "eviction_suppressed");
    assert_eq!(latest.reason, "no_eviction_candidate");
}

#[test]
fn managed_peer_policy_info_projects_active_runtime_bans() {
    // Arrange
    let mut network = ManagedPeerNetwork::new(
        MemoryChainstateStore::default(),
        local_config(403),
        PolicyConfig::default(),
    );
    let entry = peer_policy_entry(
        BanScope::Address(IpAddr::from([203, 0, 113, 10])),
        300,
        "manual_ban",
    );

    // Act
    network.record_peer_policy_ban(entry, 150);
    let info = network.peer_policy_info();

    // Assert
    assert_eq!(info.active_bans, 1);
    let latest = info
        .maybe_latest_peer_policy_decision
        .expect("latest policy decision");
    assert_eq!(latest.label, "ban_active");
}

#[test]
fn managed_peer_policy_info_projects_manual_unbans() {
    // Arrange
    let mut network = ManagedPeerNetwork::new(
        MemoryChainstateStore::default(),
        local_config(404),
        PolicyConfig::default(),
    );
    let scope = BanScope::Address(IpAddr::from([203, 0, 113, 11]));
    network.record_peer_policy_ban(peer_policy_entry(scope.clone(), 300, "manual_ban"), 150);

    // Act
    network.record_peer_policy_unban(&scope, 160);
    let info = network.peer_policy_info();

    // Assert
    assert_eq!(info.manual_unbans, 1);
    let latest = info
        .maybe_latest_peer_policy_decision
        .expect("latest policy decision");
    assert_eq!(latest.label, "unbanned");
}

#[test]
fn managed_peer_policy_info_projects_runtime_misbehavior() {
    // Arrange
    let mut network = ManagedPeerNetwork::new(
        MemoryChainstateStore::default(),
        local_config(405),
        PolicyConfig::default(),
    );
    let decision = MisbehaviorDecision {
        peer_label: "peer-protected".to_string(),
        kind: MisbehaviorKind::MalformedMessage,
        score: 500,
        response: MisbehaviorResponse::ProtectedNoAction,
    };

    // Act
    network.record_peer_policy_misbehavior(decision);
    let info = network.peer_policy_info();

    // Assert
    assert_eq!(info.misbehavior_observations, 1);
    assert_eq!(info.protected_no_actions, 1);
    let latest = info
        .maybe_latest_peer_policy_decision
        .expect("latest policy decision");
    assert_eq!(latest.outcome, "protected_no_action");
}

#[test]
fn managed_address_boundary_info_projects_over_cap_addr_rejections() {
    // Arrange
    let services = ServiceFlags::NETWORK | ServiceFlags::WITNESS;
    let mut network = ManagedPeerNetwork::new(
        MemoryChainstateStore::default(),
        local_config(302),
        PolicyConfig::default(),
    );
    network.add_inbound_peer(302).expect("peer should be added");
    let now_unix_seconds = 1_700_000_000;
    let addresses = (0..=PHASE92_LEARNED_ADDR_BATCH_LIMIT)
        .map(|index| AddressAnnouncement {
            time_unix_seconds: now_unix_seconds,
            address: public_ipv4_network_address(9, 9, 9, index as u8, 18_444, services),
        })
        .collect();

    // Act
    let actions = network
        .receive_message(
            302,
            WireNetworkMessage::Addr(AddressList { addresses }),
            now_unix_seconds as i64,
            verify_flags(),
            consensus_params(),
        )
        .expect("over-cap addr batch should be evidence only")
        .outbound;
    let info = network.address_boundary_info();

    // Assert
    assert!(actions.is_empty());
    assert_eq!(info.learned_address_entries, 0);
    assert_eq!(
        info.learned_address_rejections,
        u32::try_from(PHASE92_LEARNED_ADDR_BATCH_LIMIT + 1).expect("phase limit fits"),
    );
    let latest = info
        .maybe_latest_address_decision
        .expect("latest address decision");
    assert_eq!(latest.label, "learned_rejected");
    assert_eq!(latest.reason, "over_cap_batch");
}

#[test]
fn managed_address_boundary_info_projects_learned_counts() {
    // Arrange
    let services = ServiceFlags::NETWORK | ServiceFlags::WITNESS;
    let learned_address = public_ipv4_network_address(9, 9, 9, 9, 18_445, services);
    let evidence = PeerAddressBoundaryEvidence {
        local_advertisement_candidates: Vec::new(),
        suppressed_advertisements: Vec::new(),
        getaddr_responses_served: Vec::new(),
        getaddr_requests_suppressed: Vec::new(),
        learned_address_entries: vec![learned_address_entry(learned_address)],
        learned_address_rejections: vec![learned_address_rejection(18_446, services)],
        learned_address_rejection_count: 1,
        maybe_latest_address_decision: Some(PeerAddressBoundaryDecision {
            label: AddressDecisionLabel::LearnedRejected,
            reason: AddressDecisionReason::DuplicateAddress,
        }),
    };

    // Act
    let info = ManagedAddressBoundaryInfo::from(evidence);

    // Assert
    assert_eq!(info.learned_address_entries, 1);
    assert_eq!(info.learned_address_rejections, 1);
    let latest = info
        .maybe_latest_address_decision
        .expect("latest learned decision");
    assert_eq!(latest.label, "learned_rejected");
    assert_eq!(latest.reason, "duplicate_address");
}

#[test]
fn managed_address_boundary_info_latest_decision_labels_are_stable() {
    // Arrange
    let cases = [
        (
            AddressDecisionLabel::AdvertiseCandidate,
            AddressDecisionReason::PolicyAccepted,
            "advertise_candidate",
            "source_local_listener",
        ),
        (
            AddressDecisionLabel::AdvertiseSuppressed,
            AddressDecisionReason::PermissionPolicyDenied,
            "advertise_suppressed",
            "source_local_listener",
        ),
        (
            AddressDecisionLabel::GetAddrServed,
            AddressDecisionReason::PolicyAccepted,
            "getaddr_served",
            "source_inbound_addr",
        ),
        (
            AddressDecisionLabel::GetAddrSuppressed,
            AddressDecisionReason::AlreadyServed,
            "getaddr_suppressed",
            "source_inbound_addr",
        ),
        (
            AddressDecisionLabel::LearnedAccepted,
            AddressDecisionReason::PolicyAccepted,
            "learned_accepted",
            "source_inbound_addr",
        ),
        (
            AddressDecisionLabel::LearnedRejected,
            AddressDecisionReason::DuplicateAddress,
            "learned_rejected",
            "source_inbound_addr",
        ),
    ];

    // Act
    let projected: Vec<_> = cases
        .into_iter()
        .map(|(label, reason, expected_label, expected_source)| {
            let info = ManagedAddressBoundaryInfo::from(PeerAddressBoundaryEvidence {
                local_advertisement_candidates: Vec::new(),
                suppressed_advertisements: Vec::new(),
                getaddr_responses_served: Vec::new(),
                getaddr_requests_suppressed: Vec::new(),
                learned_address_entries: Vec::new(),
                learned_address_rejections: Vec::new(),
                learned_address_rejection_count: 0,
                maybe_latest_address_decision: Some(PeerAddressBoundaryDecision { label, reason }),
            });
            let event = info
                .maybe_latest_address_decision
                .expect("latest decision should project");
            (
                event.label,
                event.reason,
                event.source,
                expected_label,
                expected_source,
            )
        })
        .collect();

    // Assert
    for (label, reason, source, expected_label, expected_source) in projected {
        assert_eq!(label, expected_label);
        assert!(!reason.is_empty());
        assert_eq!(source, expected_source);
    }
}
