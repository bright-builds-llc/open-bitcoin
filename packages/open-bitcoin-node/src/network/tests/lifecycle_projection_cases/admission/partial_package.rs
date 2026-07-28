// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/node/mempool_persist.cpp
// - packages/bitcoin-knots/src/txmempool.cpp
// - packages/bitcoin-knots/src/validation.cpp

use open_bitcoin_core::chainstate::ChainstateSnapshot;
use open_bitcoin_mempool::{
    AdmissionContext, Mempool, MempoolCapacity, MempoolRemovalCause, MempoolRemovalRole,
    PackageMemberResult, PackageStatus, PolicyTime, SubmissionPackage, SubmitPackageCommand,
    WellFormedPackage,
};

use super::*;

fn generation_after(count: usize) -> LifecycleGeneration {
    (0..count).fold(LifecycleGeneration::INITIAL, |generation, _| {
        generation.checked_next().expect("test generation advances")
    })
}

fn expected_projection(
    network: &ManagedPeerNetwork<MemoryChainstateStore>,
    canonical_members: BTreeSet<MempoolMemberIdentity>,
    compact_members: BTreeSet<MempoolMemberIdentity>,
    generation_count: usize,
    evidence: LifecycleEvidenceSnapshot,
) -> ExpectedProjection {
    let generation = generation_after(generation_count);
    ExpectedProjection {
        canonical_members: canonical_members.clone(),
        serving_members: canonical_members.clone(),
        fanout_members: canonical_members.clone(),
        peer_known_members: canonical_members,
        peer: network.peer_manager.mempool_lifecycle_snapshot(),
        compact_members,
        unbroadcast_members: BTreeSet::new(),
        authority_epoch: AuthorityEpoch::INITIAL,
        lifecycle_generation: generation,
        dirty_generation: (generation_count != 0).then_some(generation),
        evidence,
        reconciliation_counts: [0; 7],
    }
}

fn submit_package(
    network: &mut ManagedPeerNetwork<MemoryChainstateStore>,
    members: [Transaction; 2],
    peer_id: u64,
) -> open_bitcoin_mempool::SubmittedPackageResult {
    network
        .submit_package_admission_for_test(
            members,
            [peer_id; 2],
            100,
            verify_flags(),
            consensus_params(),
        )
        .expect("package admission should return its exact report")
}

fn identity(transaction: &Transaction) -> MempoolMemberIdentity {
    MempoolMemberIdentity {
        txid: transaction_txid(transaction).expect("txid"),
        wtxid: transaction_wtxid(transaction).expect("wtxid"),
    }
}

#[test]
fn partial_package_projects_only_the_parent_survivor() {
    // Arrange
    let peer_id = 134_063;
    let (mut network, coinbase_txid) = network_with_spendable_coinbase(PolicyConfig::default());
    network.add_inbound_peer(peer_id).expect("peer");
    let parent = spend_transaction(coinbase_txid, 499_999_000);
    let parent_identity = identity(&parent);
    let mut invalid_child = spend_transaction(parent_identity.txid, 499_998_000);
    invalid_child.inputs[0].script_sig = script(&[0x01, 0x52]);

    // Act
    let submitted = submit_package(&mut network, [parent, invalid_child], peer_id);

    // Assert
    assert_eq!(submitted.report.status(), &PackageStatus::Partial);
    assert!(matches!(
        submitted.report.members(),
        [
            PackageMemberResult::FinallyPresent(_),
            PackageMemberResult::HardRejected(_)
        ]
    ));
    assert_eq!(submitted.delta.admitted, vec![parent_identity]);
    assert_complete_projection(
        &network,
        &expected_projection(
            &network,
            BTreeSet::from([parent_identity]),
            BTreeSet::new(),
            1,
            LifecycleEvidenceSnapshot {
                committed_transitions: 1,
                admitted_members: 1,
                ..LifecycleEvidenceSnapshot::default()
            },
        ),
    );
}

#[test]
fn post_trim_package_absences_never_enter_accepted_targets() {
    // Arrange
    let peer_id = 134_064;
    let (mut network, coinbase_txid) = network_with_spendable_coinbase(PolicyConfig {
        mempool_capacity: MempoolCapacity::new(0),
        ..PolicyConfig::default()
    });
    network.add_inbound_peer(peer_id).expect("peer");
    let parent = spend_transaction(coinbase_txid, 499_999_000);
    let child = spend_transaction(identity(&parent).txid, 499_998_000);
    let baseline = expected_projection(
        &network,
        BTreeSet::new(),
        BTreeSet::new(),
        0,
        LifecycleEvidenceSnapshot::default(),
    );

    // Act
    let submitted = submit_package(&mut network, [parent, child], peer_id);

    // Assert
    assert_eq!(submitted.report.status(), &PackageStatus::Failed);
    assert!(
        submitted
            .report
            .members()
            .iter()
            .all(|member| matches!(member, PackageMemberResult::PostTrimAbsent(_)))
    );
    assert!(submitted.delta.is_empty());
    assert_complete_projection(&network, &baseline);
}

#[test]
fn failed_package_admission_is_an_all_projection_noop() {
    // Arrange
    let peer_id = 134_065;
    let (mut network, coinbase_txid) = network_with_spendable_coinbase(PolicyConfig::default());
    network.add_inbound_peer(peer_id).expect("peer");
    let mut invalid_parent = spend_transaction(coinbase_txid, 499_999_000);
    invalid_parent.inputs[0].script_sig = script(&[0x01, 0x52]);
    let child = spend_transaction(identity(&invalid_parent).txid, 499_998_000);
    let baseline = expected_projection(
        &network,
        BTreeSet::new(),
        BTreeSet::new(),
        0,
        LifecycleEvidenceSnapshot::default(),
    );

    // Act
    let submitted = submit_package(&mut network, [invalid_parent, child], peer_id);

    // Assert
    assert_eq!(submitted.report.status(), &PackageStatus::Failed);
    assert!(submitted.delta.is_empty());
    assert_complete_projection(&network, &baseline);
}

#[test]
fn replacement_package_tears_down_both_victim_aliases_and_fingerprint() {
    // Arrange
    let peer_id = 134_066;
    let (mut network, coinbase_txid) = network_with_spendable_coinbase(PolicyConfig::default());
    network.add_inbound_peer(peer_id).expect("peer");
    let mut original_parent = spend_transaction(coinbase_txid, 499_999_000);
    original_parent.inputs[0].sequence = TransactionInput::MAX_SEQUENCE_NONFINAL - 1;
    let original_parent_identity = identity(&original_parent);
    let original_child = spend_transaction(original_parent_identity.txid, 499_998_000);
    let original_child_identity = identity(&original_child);
    for (timestamp, transaction) in [(10, original_parent.clone()), (11, original_child.clone())] {
        network
            .submit_local_transaction_outcome_at(
                transaction,
                verify_flags(),
                consensus_params(),
                timestamp,
                RelayIntent::NotRequested,
            )
            .expect("original member admission");
    }
    let replacement_parent = spend_transaction(coinbase_txid, 499_996_000);
    let replacement_parent_identity = identity(&replacement_parent);
    let replacement_child = spend_transaction(replacement_parent_identity.txid, 499_990_000);
    let replacement_child_identity = identity(&replacement_child);

    // Act
    let submitted = submit_package(
        &mut network,
        [replacement_parent, replacement_child],
        peer_id,
    );

    // Assert
    assert_eq!(submitted.report.status(), &PackageStatus::Complete);
    assert_eq!(submitted.delta.removed.len(), 2);
    assert!(submitted.delta.removed.iter().any(|removal| {
        removal.member == original_parent_identity
            && removal.cause == MempoolRemovalCause::Replacement
            && removal.role == MempoolRemovalRole::Direct
    }));
    assert!(submitted.delta.removed.iter().any(|removal| {
        removal.member == original_child_identity
            && removal.cause == MempoolRemovalCause::Replacement
            && removal.role == MempoolRemovalRole::Descendant
    }));
    let expected_members =
        BTreeSet::from([replacement_parent_identity, replacement_child_identity]);
    let expected_compact = BTreeSet::from([original_parent_identity, original_child_identity]);
    assert_complete_projection(
        &network,
        &expected_projection(
            &network,
            expected_members,
            expected_compact,
            3,
            LifecycleEvidenceSnapshot {
                committed_transitions: 3,
                admitted_members: 4,
                removed_members: 2,
                retry_clears: 2,
                replacement_removals: 2,
                ..LifecycleEvidenceSnapshot::default()
            },
        ),
    );
    for victim in [original_parent_identity, original_child_identity] {
        assert!(!network.peer_manager.mempool_identity_known(
            open_bitcoin_network::PeerTransactionIdentity::new(victim.txid, victim.wtxid)
        ));
    }
}

fn package_memory(snapshot: &ChainstateSnapshot, members: [Transaction; 2]) -> usize {
    let checked = WellFormedPackage::try_from(Vec::from(members)).expect("checked package");
    let package =
        SubmissionPackage::try_from_package(checked, snapshot).expect("submission package");
    let mut mempool = Mempool::default();
    mempool
        .submit_package(
            SubmitPackageCommand {
                package,
                context: AdmissionContext::peer(PolicyTime::from_unix_seconds(100)),
            },
            snapshot,
            verify_flags(),
            consensus_params(),
        )
        .expect("probe package");
    mempool.accounted_memory().as_usize()
}

#[test]
fn pressure_package_removes_the_lower_fee_member_identity_completely() {
    // Arrange
    let peer_id = 134_067;
    let mut probe = ManagedPeerNetwork::new(
        MemoryChainstateStore::default(),
        local_config(peer_id),
        PolicyConfig::default(),
    );
    let genesis = build_block(BlockHash::from_byte_array([0; 32]), 0, 500_000_000);
    let second = build_block(block_hash(&genesis.header), 1, 500_000_000);
    let third = build_block(block_hash(&second.header), 2, 500_000_000);
    for block in [&genesis, &second, &third] {
        probe
            .connect_local_block(block, verify_flags(), consensus_params())
            .expect("probe chain block");
    }
    let low = spend_transaction(
        transaction_txid(&genesis.transactions[0]).expect("genesis coinbase"),
        499_999_900,
    );
    let package_parent = spend_transaction(
        transaction_txid(&second.transactions[0]).expect("second coinbase"),
        499_996_000,
    );
    let package_child = spend_transaction(identity(&package_parent).txid, 499_990_000);
    let capacity = package_memory(
        &probe.chainstate.chainstate().snapshot(),
        [package_parent.clone(), package_child.clone()],
    );
    let mut network = ManagedPeerNetwork::new(
        MemoryChainstateStore::default(),
        local_config(peer_id),
        PolicyConfig {
            mempool_capacity: MempoolCapacity::new(capacity),
            ..PolicyConfig::default()
        },
    );
    for block in [&genesis, &second, &third] {
        network
            .connect_local_block(block, verify_flags(), consensus_params())
            .expect("chain block");
    }
    network.add_inbound_peer(peer_id).expect("peer");
    let low_identity = identity(&low);
    network
        .submit_local_transaction_outcome_at(
            low,
            verify_flags(),
            consensus_params(),
            90,
            RelayIntent::NotRequested,
        )
        .expect("low-fee admission");
    let parent_identity = identity(&package_parent);
    let child_identity = identity(&package_child);

    // Act
    let submitted = submit_package(&mut network, [package_parent, package_child], peer_id);

    // Assert
    assert!(submitted.delta.removed.iter().any(|removal| {
        removal.member == low_identity && removal.cause == MempoolRemovalCause::Pressure
    }));
    assert_complete_projection(
        &network,
        &expected_projection(
            &network,
            BTreeSet::from([parent_identity, child_identity]),
            BTreeSet::new(),
            2,
            LifecycleEvidenceSnapshot {
                committed_transitions: 2,
                admitted_members: 3,
                removed_members: 1,
                retry_clears: 1,
                pressure_removals: 1,
                ..LifecycleEvidenceSnapshot::default()
            },
        ),
    );
    assert!(!network.peer_manager.mempool_identity_known(
        open_bitcoin_network::PeerTransactionIdentity::new(low_identity.txid, low_identity.wtxid)
    ));
}
