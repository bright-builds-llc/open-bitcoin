// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/node/mempool_persist.cpp
// - packages/bitcoin-knots/src/txmempool.cpp
// - packages/bitcoin-knots/src/validation.cpp

use std::collections::{BTreeMap, BTreeSet};

use open_bitcoin_core::consensus::{block_hash, transaction_txid, transaction_wtxid};
use open_bitcoin_core::primitives::{BlockHash, Transaction, Txid, Wtxid};
use open_bitcoin_mempool::{
    AdmissionContext, FeeRate, MempoolAcceptanceTime, MempoolEntryMetadata, MempoolMemberIdentity,
    MempoolOrigin, PolicyConfig, PolicyTime, PreparedMempoolTransition, RelayIntent,
    RollingMempoolFeeRate,
};
use open_bitcoin_network::PeerMempoolLifecycleSnapshot;

use super::*;
use crate::network::lifecycle_projection::{
    AuthorityEpoch, LifecycleCommand, LifecycleEvidenceSnapshot, LifecycleGeneration,
    LifecycleProjectionError, LifecycleProjectionPlan, LifecycleReconciliationReport,
};
use crate::network::runtime_authority::{LifecycleCommandResult, apply_lifecycle_command};

mod admission;
mod effects;
mod maintenance;
mod oracle;
mod reconciliation;

#[derive(Debug)]
struct ExpectedProjection {
    canonical_members: BTreeSet<MempoolMemberIdentity>,
    serving_members: BTreeSet<MempoolMemberIdentity>,
    fanout_members: BTreeSet<MempoolMemberIdentity>,
    peer_known_members: BTreeSet<MempoolMemberIdentity>,
    peer: PeerMempoolLifecycleSnapshot,
    compact_members: BTreeSet<MempoolMemberIdentity>,
    unbroadcast_members: BTreeSet<MempoolMemberIdentity>,
    authority_epoch: AuthorityEpoch,
    lifecycle_generation: LifecycleGeneration,
    dirty_generation: Option<LifecycleGeneration>,
    evidence: LifecycleEvidenceSnapshot,
    reconciliation_counts: [usize; 7],
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct CompleteAggregateSnapshot {
    canonical_members: BTreeSet<MempoolMemberIdentity>,
    rolling_mempool_fee_rate: RollingMempoolFeeRate,
    serving_members: BTreeSet<MempoolMemberIdentity>,
    transactions_by_txid: BTreeMap<Txid, Transaction>,
    transactions_by_wtxid: BTreeMap<Wtxid, Transaction>,
    fanout_members: BTreeSet<MempoolMemberIdentity>,
    peer: PeerMempoolLifecycleSnapshot,
    compact_members: BTreeSet<MempoolMemberIdentity>,
    unbroadcast_members: BTreeSet<MempoolMemberIdentity>,
    authority_epoch: AuthorityEpoch,
    lifecycle_generation: LifecycleGeneration,
    dirty_generation: Option<LifecycleGeneration>,
    evidence: LifecycleEvidenceSnapshot,
}

fn canonical_members(
    network: &ManagedPeerNetwork<MemoryChainstateStore>,
) -> BTreeSet<MempoolMemberIdentity> {
    network
        .mempool()
        .mempool()
        .entries()
        .iter()
        .map(|(txid, entry)| MempoolMemberIdentity {
            txid: *txid,
            wtxid: entry.wtxid,
        })
        .collect()
}

fn complete_aggregate_snapshot(
    network: &ManagedPeerNetwork<MemoryChainstateStore>,
) -> CompleteAggregateSnapshot {
    CompleteAggregateSnapshot {
        canonical_members: canonical_members(network),
        rolling_mempool_fee_rate: network.mempool().mempool().rolling_mempool_fee_rate(),
        serving_members:
            crate::network::relay_serving::RelayServingCache::lifecycle_members_for_test(
                &network.relay_serving,
            ),
        transactions_by_txid: network.transactions_by_txid.clone(),
        transactions_by_wtxid: network.transactions_by_wtxid.clone(),
        fanout_members:
            crate::network::relay_fanout::ManagedRelayFanoutState::lifecycle_members_for_test(
                &network.relay_fanout,
            ),
        peer: open_bitcoin_network::PeerManager::mempool_lifecycle_snapshot(&network.peer_manager),
        compact_members: network
            .compact_extra_txn
            .iter_available()
            .map(|(wtxid, transaction)| MempoolMemberIdentity {
                txid: transaction_txid(transaction).expect("compact extra txid"),
                wtxid: *wtxid,
            })
            .collect(),
        unbroadcast_members: ManagedPeerNetwork::unbroadcast_members(network).clone(),
        authority_epoch: network.authority_epoch(),
        lifecycle_generation: network.lifecycle_generation(),
        dirty_generation: ManagedPeerNetwork::dirty_generation(network),
        evidence: ManagedPeerNetwork::lifecycle_evidence_snapshot(network),
    }
}

fn advance_only_core_revision_for_test(network: &mut ManagedPeerNetwork<MemoryChainstateStore>) {
    network
        .set_rolling_mempool_fee_rate(RollingMempoolFeeRate::new(FeeRate::from_sats_per_kvb(1)))
        .expect("core-only revision advance should succeed");
}

fn compact_members(
    network: &ManagedPeerNetwork<MemoryChainstateStore>,
) -> BTreeSet<MempoolMemberIdentity> {
    network
        .compact_extra_txn
        .iter_available()
        .map(|(wtxid, transaction)| MempoolMemberIdentity {
            txid: transaction_txid(transaction).expect("compact extra txid"),
            wtxid: *wtxid,
        })
        .collect()
}

fn assert_complete_projection(
    network: &ManagedPeerNetwork<MemoryChainstateStore>,
    expected: &ExpectedProjection,
) {
    assert_eq!(canonical_members(network), expected.canonical_members);
    assert_eq!(
        network.relay_serving.lifecycle_members_for_test(),
        expected.serving_members
    );
    assert_eq!(
        network.relay_fanout.lifecycle_members_for_test(),
        expected.fanout_members
    );
    assert_eq!(
        network
            .transactions_by_txid
            .keys()
            .copied()
            .collect::<BTreeSet<_>>(),
        expected
            .serving_members
            .iter()
            .map(|member| member.txid)
            .collect::<BTreeSet<_>>()
    );
    assert_eq!(
        network
            .transactions_by_wtxid
            .keys()
            .copied()
            .collect::<BTreeSet<_>>(),
        expected
            .serving_members
            .iter()
            .map(|member| member.wtxid)
            .collect::<BTreeSet<_>>()
    );
    assert_eq!(
        network.peer_manager.mempool_lifecycle_snapshot(),
        expected.peer
    );
    for member in &expected.peer_known_members {
        assert!(network.peer_manager.mempool_identity_known(
            open_bitcoin_network::PeerTransactionIdentity::new(member.txid, member.wtxid)
        ));
    }
    assert_eq!(compact_members(network), expected.compact_members);
    assert_eq!(network.unbroadcast_members(), &expected.unbroadcast_members);
    assert_eq!(network.authority_epoch(), expected.authority_epoch);
    assert_eq!(
        network.lifecycle_generation(),
        expected.lifecycle_generation
    );
    assert_eq!(network.dirty_generation(), expected.dirty_generation);
    assert_eq!(network.lifecycle_evidence_snapshot(), expected.evidence);

    let report = network.reconcile_lifecycle_projection();
    assert_eq!(
        report.counts(),
        expected.reconciliation_counts,
        "{report:?}"
    );
    assert_eq!(
        report.labels(),
        LifecycleReconciliationReport::FIXED_TARGET_LABELS
    );
}

fn assert_prepared_orders(
    prepared: &PreparedMempoolTransition,
    admitted: &[MempoolMemberIdentity],
    teardowns: &[MempoolMemberIdentity],
) {
    assert_eq!(prepared.facts().admitted_order(), admitted);
    assert_eq!(prepared.facts().teardown_order(), teardowns);
}

fn network_with_spendable_coinbase(
    config: PolicyConfig,
) -> (ManagedPeerNetwork<MemoryChainstateStore>, Txid) {
    let mut network = ManagedPeerNetwork::new(
        MemoryChainstateStore::default(),
        local_config(134_051),
        config,
    );
    let genesis = build_block(BlockHash::from_byte_array([0; 32]), 0, 500_000_000);
    let spendable = build_block(block_hash(&genesis.header), 1, 500_000_000);
    network
        .connect_local_block(&genesis, verify_flags(), consensus_params())
        .expect("connect genesis");
    network
        .connect_local_block(&spendable, verify_flags(), consensus_params())
        .expect("connect spendable block");
    (
        network,
        transaction_txid(&spendable.transactions[0]).expect("coinbase txid"),
    )
}

fn apply_prepared(
    network: &mut ManagedPeerNetwork<MemoryChainstateStore>,
    core: open_bitcoin_mempool::PreparedMempoolTransition,
) {
    let plan = LifecycleProjectionPlan::prepare(network, network.authority_epoch(), core)
        .expect("projection should prepare");
    let sealed = network
        .validate_prepared_lifecycle(plan)
        .expect("current projection should validate");
    network.commit_sealed_lifecycle(sealed);
}

mod authority {
    use super::*;

    #[test]
    fn stale_authority_epoch_preserves_the_complete_aggregate() {
        // Arrange
        let network = ManagedPeerNetwork::new(
            MemoryChainstateStore::default(),
            local_config(134_052),
            PolicyConfig::default(),
        );
        let core = network
            .mempool()
            .prepare_expiry(PolicyTime::new(0))
            .expect("empty expiry should prepare");
        let stale_epoch = AuthorityEpoch::INITIAL
            .checked_next()
            .expect("test epoch should advance");
        let plan = LifecycleProjectionPlan::prepare(&network, stale_epoch, core)
            .expect("projection should prepare");
        let baseline = format!("{network:?}");

        // Act
        let Err(error) = network.validate_prepared_lifecycle(plan) else {
            panic!("stale epoch must fail");
        };

        // Assert
        assert!(matches!(
            error,
            LifecycleProjectionError::StaleAuthorityEpoch { .. }
        ));
        assert_eq!(format!("{network:?}"), baseline);
    }

    #[test]
    fn stale_aggregate_failure_preserves_all_eight_domains_then_fresh_plan_succeeds() {
        // Arrange
        let (mut network, coinbase_txid) = network_with_spendable_coinbase(PolicyConfig::default());
        let transaction = spend_transaction(coinbase_txid, 499_999_000);
        let member = MempoolMemberIdentity {
            txid: transaction_txid(&transaction).expect("txid"),
            wtxid: transaction_wtxid(&transaction).expect("wtxid"),
        };
        let stale_core = network
            .mempool
            .prepare_transaction_with_context(
                &network.chainstate,
                transaction.clone(),
                verify_flags(),
                consensus_params(),
                AdmissionContext::local(PolicyTime::new(100), RelayIntent::Requested),
            )
            .expect("stale admission should prepare");
        let stale_plan =
            LifecycleProjectionPlan::prepare(&network, AuthorityEpoch::INITIAL, stale_core)
                .expect("projection should prepare");
        advance_only_core_revision_for_test(&mut network);
        let after_newer_core_revision = complete_aggregate_snapshot(&network);

        // Act
        let error = match apply_lifecycle_command(
            &mut network,
            LifecycleCommand::SingletonAdmission(stale_plan),
        ) {
            Ok(_) => panic!("stale core revision must fail"),
            Err(error) => error,
        };

        // Assert
        assert!(matches!(error, LifecycleProjectionError::Mempool(_)));
        assert_eq!(
            complete_aggregate_snapshot(&network),
            after_newer_core_revision
        );

        // Arrange
        let fresh_core = network
            .mempool
            .prepare_transaction_with_context(
                &network.chainstate,
                transaction,
                verify_flags(),
                consensus_params(),
                AdmissionContext::local(PolicyTime::new(100), RelayIntent::Requested),
            )
            .expect("fresh admission should prepare");
        let fresh_plan =
            LifecycleProjectionPlan::prepare(&network, AuthorityEpoch::INITIAL, fresh_core)
                .expect("fresh projection should prepare");

        // Act
        let LifecycleCommandResult::Lifecycle(delta) = apply_lifecycle_command(
            &mut network,
            LifecycleCommand::SingletonAdmission(fresh_plan),
        )
        .expect("fresh aggregate commit should succeed") else {
            panic!("fresh admission must return a lifecycle delta");
        };

        // Assert
        assert_eq!(delta.admitted, [member]);
        assert_complete_projection(
            &network,
            &ExpectedProjection {
                canonical_members: BTreeSet::from([member]),
                serving_members: BTreeSet::from([member]),
                fanout_members: BTreeSet::from([member]),
                peer_known_members: BTreeSet::from([member]),
                peer: network.peer_manager.mempool_lifecycle_snapshot(),
                compact_members: BTreeSet::new(),
                unbroadcast_members: BTreeSet::from([member]),
                authority_epoch: AuthorityEpoch::INITIAL,
                lifecycle_generation: LifecycleGeneration::INITIAL
                    .checked_next()
                    .expect("test generation should advance"),
                dirty_generation: Some(
                    LifecycleGeneration::INITIAL
                        .checked_next()
                        .expect("test generation should advance"),
                ),
                evidence: LifecycleEvidenceSnapshot {
                    committed_transitions: 1,
                    admitted_members: 1,
                    ..LifecycleEvidenceSnapshot::default()
                },
                reconciliation_counts: [0; 7],
            },
        );
        assert_eq!(
            network.mempool().mempool().rolling_mempool_fee_rate(),
            RollingMempoolFeeRate::new(FeeRate::from_sats_per_kvb(1))
        );
    }

    #[test]
    fn local_requested_admission_advances_once_and_enters_unbroadcast() {
        // Arrange
        let (mut network, coinbase_txid) = network_with_spendable_coinbase(PolicyConfig::default());
        let transaction = spend_transaction(coinbase_txid, 499_999_000);
        let txid = transaction_txid(&transaction).expect("txid");
        let wtxid = transaction_wtxid(&transaction).expect("wtxid");
        let core = network
            .mempool
            .prepare_transaction_with_context(
                &network.chainstate,
                transaction,
                verify_flags(),
                consensus_params(),
                AdmissionContext::local(PolicyTime::new(100), RelayIntent::Requested),
            )
            .expect("admission should prepare");

        // Act
        apply_prepared(&mut network, core);

        // Assert
        assert_eq!(network.lifecycle_generation().raw(), 1);
        assert_eq!(
            network.dirty_generation(),
            Some(network.lifecycle_generation())
        );
        assert!(
            network
                .unbroadcast_members()
                .contains(&open_bitcoin_mempool::MempoolMemberIdentity { txid, wtxid })
        );
    }

    #[test]
    fn empty_delta_advances_no_generation_or_evidence() {
        // Arrange
        let mut network = ManagedPeerNetwork::new(
            MemoryChainstateStore::default(),
            local_config(134_054),
            PolicyConfig::default(),
        );
        let core = network
            .mempool()
            .prepare_expiry(PolicyTime::new(0))
            .expect("empty expiry should prepare");
        let baseline_evidence = network.lifecycle_evidence_snapshot();

        // Act
        apply_prepared(&mut network, core);

        // Assert
        assert_eq!(network.lifecycle_generation().raw(), 0);
        assert_eq!(network.dirty_generation(), None);
        assert_eq!(network.lifecycle_evidence_snapshot(), baseline_evidence);
    }

    #[test]
    fn only_local_requested_members_enter_unbroadcast() {
        // Arrange
        let contexts = [
            AdmissionContext::local(PolicyTime::new(100), RelayIntent::NotRequested),
            AdmissionContext::new(MempoolEntryMetadata::new(
                MempoolAcceptanceTime::Known(PolicyTime::new(100)),
                MempoolOrigin::Peer,
                RelayIntent::Requested,
            )),
        ];

        // Act
        let unbroadcast_counts = contexts.map(|context| {
            let (mut network, coinbase_txid) =
                network_with_spendable_coinbase(PolicyConfig::default());
            let core = network
                .mempool
                .prepare_transaction_with_context(
                    &network.chainstate,
                    spend_transaction(coinbase_txid, 499_999_000),
                    verify_flags(),
                    consensus_params(),
                    context,
                )
                .expect("admission should prepare");
            apply_prepared(&mut network, core);
            network.unbroadcast_members().len()
        });

        // Assert
        assert_eq!(unbroadcast_counts, [0, 0]);
    }

    #[test]
    fn lifecycle_removal_clears_the_exact_unbroadcast_identity() {
        // Arrange
        let (mut network, coinbase_txid) = network_with_spendable_coinbase(PolicyConfig {
            mempool_expiry_hours: 1,
            ..PolicyConfig::default()
        });
        let transaction = spend_transaction(coinbase_txid, 499_999_000);
        let txid = transaction_txid(&transaction).expect("txid");
        let wtxid = transaction_wtxid(&transaction).expect("wtxid");
        let admission = network
            .mempool
            .prepare_transaction_with_context(
                &network.chainstate,
                transaction,
                verify_flags(),
                consensus_params(),
                AdmissionContext::local(PolicyTime::new(100), RelayIntent::Requested),
            )
            .expect("admission should prepare");
        apply_prepared(&mut network, admission);
        let removal = network
            .mempool
            .prepare_expiry(PolicyTime::new(3_701))
            .expect("expiry should prepare");

        // Act
        apply_prepared(&mut network, removal);

        // Assert
        assert!(
            !network
                .unbroadcast_members()
                .contains(&open_bitcoin_mempool::MempoolMemberIdentity { txid, wtxid })
        );
        assert_eq!(network.lifecycle_generation().raw(), 2);
    }
}
