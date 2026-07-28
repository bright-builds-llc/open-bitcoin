// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/node/mempool_persist.cpp
// - packages/bitcoin-knots/src/txmempool.cpp
// - packages/bitcoin-knots/src/validation.cpp

use super::*;
use crate::network::lifecycle_projection::{
    LifecyclePreparationError, LifecyclePreparationFailureGuard, LifecyclePreparationFailurePoint,
};

mod independent_model {
    use std::collections::{BTreeMap, BTreeSet};

    pub(super) const TARGET_LABELS: [&str; 7] = [
        "serving",
        "fanout",
        "peer",
        "compact",
        "unbroadcast",
        "persistence",
        "evidence",
    ];

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    struct Member(u16);

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub(super) enum Step {
        Admission(u16),
        Package([u16; 2]),
        Replacement {
            victim: u16,
            replacements: [u16; 2],
        },
        Pressure([u16; 2]),
        Expiry([u16; 2]),
        Block([u16; 2]),
        Reorg {
            removed: [u16; 2],
            admitted: [u16; 2],
        },
    }

    #[derive(Debug, Clone, Default, PartialEq, Eq)]
    pub(super) struct Model {
        members: BTreeMap<Member, bool>,
        compact: BTreeSet<Member>,
        generation: u64,
        evidence: u64,
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub(super) struct Projection {
        serving: BTreeSet<Member>,
        fanout: BTreeSet<Member>,
        peer: BTreeSet<Member>,
        compact: BTreeSet<Member>,
        unbroadcast: BTreeSet<Member>,
        persistence_generation: u64,
        evidence_transitions: u64,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub(super) enum Target {
        Serving,
        Fanout,
        Peer,
        Compact,
        Unbroadcast,
        Persistence,
        Evidence,
    }

    impl Target {
        pub(super) const ALL: [Self; 7] = [
            Self::Serving,
            Self::Fanout,
            Self::Peer,
            Self::Compact,
            Self::Unbroadcast,
            Self::Persistence,
            Self::Evidence,
        ];

        pub(super) const fn index(self) -> usize {
            match self {
                Self::Serving => 0,
                Self::Fanout => 1,
                Self::Peer => 2,
                Self::Compact => 3,
                Self::Unbroadcast => 4,
                Self::Persistence => 5,
                Self::Evidence => 6,
            }
        }
    }

    impl Model {
        pub(super) fn apply(&mut self, step: &Step) {
            match *step {
                Step::Admission(member) => self.admit(member),
                Step::Package(members) => {
                    for member in members {
                        self.admit(member);
                    }
                }
                Step::Replacement {
                    victim,
                    replacements,
                } => {
                    self.remove(victim);
                    for member in replacements {
                        self.admit(member);
                    }
                }
                Step::Pressure(members) | Step::Expiry(members) | Step::Block(members) => {
                    for member in members {
                        self.remove(member);
                    }
                }
                Step::Reorg { removed, admitted } => {
                    for member in removed {
                        self.remove(member);
                    }
                    for member in admitted {
                        self.admit(member);
                    }
                }
            }
            self.generation = self.generation.saturating_add(1);
            self.evidence = self.evidence.saturating_add(1);
        }

        pub(super) fn projection(&self) -> Projection {
            let canonical = self.members.keys().copied().collect::<BTreeSet<_>>();
            Projection {
                serving: canonical.clone(),
                fanout: canonical.clone(),
                peer: canonical,
                compact: self.compact.clone(),
                unbroadcast: self
                    .members
                    .iter()
                    .filter_map(|(member, local)| local.then_some(*member))
                    .collect(),
                persistence_generation: self.generation,
                evidence_transitions: self.evidence,
            }
        }

        fn admit(&mut self, member: u16) {
            let member = Member(member);
            self.members.insert(member, member.0.is_multiple_of(2));
            self.compact.remove(&member);
        }

        fn remove(&mut self, member: u16) {
            let member = Member(member);
            if self.members.remove(&member).is_some() {
                self.compact.insert(member);
            }
        }
    }

    impl Projection {
        pub(super) fn corrupt(&mut self, target: Target) {
            let marker = Member(u16::MAX);
            match target {
                Target::Serving => {
                    self.serving.insert(marker);
                }
                Target::Fanout => {
                    self.fanout.insert(marker);
                }
                Target::Peer => {
                    self.peer.insert(marker);
                }
                Target::Compact => {
                    self.compact.insert(marker);
                }
                Target::Unbroadcast => {
                    self.unbroadcast.insert(marker);
                }
                Target::Persistence => {
                    self.persistence_generation = self.persistence_generation.saturating_add(1);
                }
                Target::Evidence => {
                    self.evidence_transitions = self.evidence_transitions.saturating_add(1);
                }
            }
        }
    }

    pub(super) fn reconcile(model: &Model, actual: &Projection) -> [usize; 7] {
        let expected = model.projection();
        [
            mismatch_count(&expected.serving, &actual.serving),
            mismatch_count(&expected.fanout, &actual.fanout),
            mismatch_count(&expected.peer, &actual.peer),
            mismatch_count(&expected.compact, &actual.compact),
            mismatch_count(&expected.unbroadcast, &actual.unbroadcast),
            usize::from(expected.persistence_generation != actual.persistence_generation),
            usize::from(expected.evidence_transitions != actual.evidence_transitions),
        ]
    }

    pub(super) fn generated_steps(mut state: u64) -> Vec<Step> {
        let mut next = || {
            state = state
                .wrapping_mul(6_364_136_223_846_793_005)
                .wrapping_add(1_442_695_040_888_963_407);
            ((state >> 32) as u16 % 10_000).saturating_add(1)
        };
        let admission = next();
        let package = [next(), next()];
        let replacement = [next(), next()];
        let later = [next(), next(), next(), next(), next(), next()];
        vec![
            Step::Admission(admission),
            Step::Package(package),
            Step::Replacement {
                victim: admission,
                replacements: replacement,
            },
            Step::Pressure(package),
            Step::Expiry(replacement),
            Step::Block([later[0], later[1]]),
            Step::Reorg {
                removed: [later[2], later[3]],
                admitted: [later[4], later[5]],
            },
        ]
    }

    fn mismatch_count(expected: &BTreeSet<Member>, actual: &BTreeSet<Member>) -> usize {
        expected.symmetric_difference(actual).count()
    }
}

#[test]
fn fixed_seed_generated_oracle_detects_each_corrupted_target_exactly() {
    // Arrange
    let steps = independent_model::generated_steps(0x1341_1000_5eed);
    let mut model = independent_model::Model::default();
    for step in &steps {
        model.apply(step);
    }
    let clean = model.projection();

    // Act and Assert
    assert_eq!(steps.len(), 7);
    assert_eq!(
        independent_model::TARGET_LABELS,
        LifecycleReconciliationReport::FIXED_TARGET_LABELS
    );
    assert_eq!(independent_model::reconcile(&model, &clean), [0; 7]);
    for target in independent_model::Target::ALL {
        let mut corrupted = clean.clone();
        corrupted.corrupt(target);
        let counts = independent_model::reconcile(&model, &corrupted);
        let mut expected = [0; 7];
        expected[target.index()] = 1;
        assert_eq!(counts, expected, "{target:?}");
    }
}

#[test]
fn independent_target_vectors_match_production_reconciliation() {
    // Arrange
    let (mut network, coinbase_txid) = network_with_spendable_coinbase(PolicyConfig::default());
    let transaction = spend_transaction(coinbase_txid, 499_999_000);
    let member = MempoolMemberIdentity {
        txid: transaction_txid(&transaction).expect("txid"),
        wtxid: transaction_wtxid(&transaction).expect("wtxid"),
    };
    let core = network
        .mempool
        .prepare_transaction_with_context(
            &network.chainstate,
            transaction.clone(),
            verify_flags(),
            consensus_params(),
            AdmissionContext::local(PolicyTime::new(100), RelayIntent::Requested),
        )
        .expect("admission should prepare");
    apply_prepared(&mut network, core);

    // Act
    let mut corrupted = Vec::new();
    let mut serving = network.clone();
    serving.relay_serving.record_status(
        member.txid,
        Some(member.wtxid),
        open_bitcoin_network::TxServingRecordStatus::Stale,
    );
    corrupted.push(serving);

    let mut fanout = network.clone();
    fanout.relay_fanout.cleanup_transactions(
        &[member.txid],
        open_bitcoin_network::TxFanoutCleanupReason::Confirmed,
    );
    corrupted.push(fanout);

    let mut peer = network.clone();
    let teardown = peer
        .peer_manager
        .prepare_transaction_lifecycle(open_bitcoin_network::PeerTransactionLifecycleInput::new(
            Vec::new(),
            vec![open_bitcoin_network::PeerTransactionIdentity::new(
                member.txid,
                member.wtxid,
            )],
            Vec::new(),
        ))
        .expect("peer teardown should prepare");
    peer.peer_manager
        .apply_prepared_transaction_lifecycle(teardown);
    corrupted.push(peer);

    let mut compact = network.clone();
    compact.compact_extra_txn.push(member.wtxid, transaction);
    corrupted.push(compact);

    let mut unbroadcast = network.clone();
    unbroadcast
        .unbroadcast_members
        .insert(MempoolMemberIdentity {
            txid: Txid::from_byte_array([0x75; 32]),
            wtxid: open_bitcoin_core::primitives::Wtxid::from_byte_array([0x76; 32]),
        });
    corrupted.push(unbroadcast);

    let mut persistence = network.clone();
    persistence.dirty_generation = Some(LifecycleGeneration::INITIAL);
    corrupted.push(persistence);

    let mut evidence = network;
    evidence.lifecycle_evidence = LifecycleEvidenceSnapshot::default();
    corrupted.push(evidence);

    // Assert
    for (target, aggregate) in independent_model::Target::ALL.into_iter().zip(corrupted) {
        let counts = aggregate.reconcile_lifecycle_projection().counts();
        let mut expected = [0; 7];
        expected[target.index()] = counts[target.index()];
        assert!(expected[target.index()] > 0, "{target:?}");
        assert_eq!(counts, expected, "{target:?}");
    }
}

#[test]
fn every_injected_preflight_failure_preserves_the_complete_aggregate() {
    // Arrange, Act, Assert
    for (index, point) in LifecyclePreparationFailurePoint::ALL
        .into_iter()
        .enumerate()
    {
        let (network, coinbase_txid) = network_with_spendable_coinbase(PolicyConfig::default());
        let index = index as i64;
        let transaction = spend_transaction(coinbase_txid, 499_999_000 - index);
        let core = network
            .mempool
            .prepare_transaction_with_context(
                &network.chainstate,
                transaction,
                verify_flags(),
                consensus_params(),
                AdmissionContext::local(PolicyTime::new(1_000 + index), RelayIntent::Requested),
            )
            .expect("core admission should prepare");
        let baseline = format!("{network:#?}");
        let guard = LifecyclePreparationFailureGuard::inject(point);

        let Err(error) =
            LifecycleProjectionPlan::prepare(&network, network.authority_epoch(), core)
        else {
            panic!("injected lifecycle preparation must fail");
        };

        assert_eq!(error, LifecyclePreparationError::InjectedFailure(point));
        assert_eq!(format!("{network:#?}"), baseline, "{point:?}");
        drop(guard);
    }
}

#[test]
fn lifecycle_evidence_schema_is_bounded_and_identifier_free() {
    // Arrange
    let evidence = LifecycleEvidenceSnapshot {
        committed_transitions: 1,
        admitted_members: 2,
        removed_members: 3,
        retry_clears: 4,
        replacement_removals: 5,
        expiry_removals: 6,
        pressure_removals: 7,
        block_confirmation_removals: 8,
        block_conflict_removals: 9,
        reorg_removals: 10,
    };

    // Act
    let rendered = format!("{evidence:?}").to_ascii_lowercase();

    // Assert
    assert_eq!(
        std::mem::size_of_val(&evidence),
        10 * std::mem::size_of::<u64>()
    );
    assert!(rendered.len() < 512);
    for forbidden in ["txid", "wtxid", "package", "peer", "path", "string"] {
        assert!(!rendered.contains(forbidden), "{forbidden}: {rendered}");
    }
}
