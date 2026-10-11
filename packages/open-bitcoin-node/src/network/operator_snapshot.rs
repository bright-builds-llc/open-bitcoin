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

//! Identifier-free operator snapshot assembly for shared mempool evidence.

#[cfg(test)]
use open_bitcoin_mempool::MempoolMemberIdentity;

use super::{ManagedNetworkOperatorSnapshot, ManagedPeerNetwork};
use crate::ChainstateStore;
use crate::chainstate::ManagerReadiness;
use crate::status::relay_evidence::{RelayEvidenceCounters, RelayEvidenceField};
use crate::status::{
    ChainstateDurabilityEvidence, FieldAvailability, MempoolEvictionGroup, MempoolPressureGroup,
    admission_group_from_counts, checkpoint_group_from_evidence, decay_half_life_label,
    recovery_group_from_summary, retry_group_from_relay,
};
use open_bitcoin_core::chainstate::{CoinsView, FlushPolicyTime};

/// Periodic checkpoint interval used to derive overdue and loss-bound facts.
const OPERATOR_CHECKPOINT_INTERVAL_SECONDS: u64 = 300;

use crate::chainstate::filter_index::AcceptedBasicIndexFailure;
use crate::status::{
    BasicIndexFailureCategory, BasicIndexNextAction, BasicIndexOperationalState,
    BasicIndexOperatorStatus, BasicIndexProtection, BasicIndexProtectionRange,
    BasicIndexUnavailableReason, BasicPeerScopeStatus,
};
use crate::storage::fjall_store::filters::retention::{
    BasicFilterOperatorAuthority, BasicOperatorObservationFailure,
};
use open_bitcoin_core::chainstate::{
    FilterCheckpoint, IndexInputProtection, IndexPrefix,
    filter_index::{
        catch_up::{AcceptedIndexTarget, BasicIndexProgress, BasicIndexState},
        lifecycle::IndexLifecycle,
    },
};

fn failure_category(failure: AcceptedBasicIndexFailure) -> BasicIndexFailureCategory {
    match failure {
        AcceptedBasicIndexFailure::RequiredBody => BasicIndexFailureCategory::RequiredBody,
        AcceptedBasicIndexFailure::RequiredUndo => BasicIndexFailureCategory::RequiredUndo,
        AcceptedBasicIndexFailure::Persistence => BasicIndexFailureCategory::Backend,
        AcceptedBasicIndexFailure::InvalidFacts => {
            BasicIndexFailureCategory::IntegrityOrInvalidFacts
        }
        AcceptedBasicIndexFailure::FactLimit => BasicIndexFailureCategory::ResourceBound,
        AcceptedBasicIndexFailure::Invalidated => BasicIndexFailureCategory::Invalidated,
    }
}

fn failure_action(failure: BasicIndexFailureCategory) -> BasicIndexNextAction {
    match failure {
        BasicIndexFailureCategory::RequiredBody => BasicIndexNextAction::RequiredBody,
        BasicIndexFailureCategory::RequiredUndo => BasicIndexNextAction::RequiredUndo,
        BasicIndexFailureCategory::Backend => BasicIndexNextAction::Backend,
        BasicIndexFailureCategory::IntegrityOrInvalidFacts => {
            BasicIndexNextAction::IntegrityOrInvalidFacts
        }
        BasicIndexFailureCategory::ResourceBound => BasicIndexNextAction::ResourceBound,
        BasicIndexFailureCategory::Invalidated => BasicIndexNextAction::Invalidated,
    }
}

fn project_read_fault(
    status: &mut BasicIndexOperatorStatus,
    fault: crate::storage::fjall_store::filters::retention::BasicFilterReadFault,
) {
    use crate::storage::fjall_store::filters::retention::BasicFilterReadFault;
    let (category, reason) = match fault {
        BasicFilterReadFault::Backend => (
            BasicIndexFailureCategory::Backend,
            BasicIndexUnavailableReason::Backend,
        ),
        BasicFilterReadFault::Integrity => (
            BasicIndexFailureCategory::IntegrityOrInvalidFacts,
            BasicIndexUnavailableReason::Integrity,
        ),
    };
    status.maybe_failure = FieldAvailability::available(Some(category));
    status.operational_state = FieldAvailability::available(BasicIndexOperationalState::Failed);
    status.next_action = FieldAvailability::available(failure_action(category));
    status.maybe_processed_height = FieldAvailability::unavailable(reason.as_str());
    status.maybe_safe_durable_height = FieldAvailability::unavailable(reason.as_str());
    status.maybe_lag_blocks = FieldAvailability::unavailable(reason.as_str());
    status.retention = FieldAvailability::unavailable(reason.as_str());
    status.retained_availability = FieldAvailability::unavailable(reason.as_str());
}

fn endpoint_checkpoint(
    progress: Option<open_bitcoin_core::chainstate::FilterRecordIdentity>,
) -> FilterCheckpoint {
    FilterCheckpoint::new(progress.map_or(IndexPrefix::Empty, IndexPrefix::Committed))
}

fn project_protection(protection: IndexInputProtection) -> BasicIndexProtection {
    match protection {
        IndexInputProtection::HeightSpaceExhausted => BasicIndexProtection::HeightSpaceExhausted,
        IndexInputProtection::FromHeight(first) => {
            let last = u32::MAX - open_bitcoin_core::chainstate::PRUNE_LOCK_BUFFER;
            BasicIndexProtectionRange::new(first, last).map_or(
                BasicIndexProtection::HeightSpaceExhausted,
                BasicIndexProtection::InclusiveRange,
            )
        }
    }
}

fn project_basic_owner(
    status: &mut BasicIndexOperatorStatus,
    progress: BasicIndexProgress,
    maybe_target: Option<AcceptedIndexTarget>,
    maybe_active_target: Option<AcceptedIndexTarget>,
    authority: &BasicFilterOperatorAuthority,
) {
    status.initial_sync_complete = FieldAvailability::available(progress.initially_synchronized());
    status.maybe_accepted_height =
        FieldAvailability::available(maybe_target.map(AcceptedIndexTarget::height));
    let coherent = authority.integrity
        && !authority.suspended
        && maybe_target == Some(progress.accepted_target())
        && maybe_target == maybe_active_target
        && status.maybe_failure
            != FieldAvailability::available(Some(BasicIndexFailureCategory::Invalidated))
        && authority.maybe_lifecycle
            == Some(IndexLifecycle::Active {
                generation: progress.generation(),
            })
        && authority
            .maybe_publication
            .is_some_and(|(generation, branch, processed, safe, _)| {
                generation == progress.generation()
                    && branch == progress.branch_identity()
                    && processed == endpoint_checkpoint(progress.maybe_processed_endpoint())
                    && safe == endpoint_checkpoint(progress.maybe_safe_durable_endpoint())
            });
    if !coherent {
        status.maybe_processed_height =
            FieldAvailability::unavailable(BasicIndexUnavailableReason::Invalidated.as_str());
        status.maybe_safe_durable_height =
            FieldAvailability::unavailable(BasicIndexUnavailableReason::Invalidated.as_str());
        status.maybe_lag_blocks =
            FieldAvailability::unavailable(BasicIndexUnavailableReason::Invalidated.as_str());
        status.operational_state =
            FieldAvailability::available(BasicIndexOperationalState::Invalidated);
        if matches!(status.maybe_failure, FieldAvailability::Available(None)) {
            status.maybe_failure =
                FieldAvailability::available(Some(BasicIndexFailureCategory::Invalidated));
        }
        status.next_action = FieldAvailability::available(match status.maybe_failure {
            FieldAvailability::Available(Some(failure)) => failure_action(failure),
            _ => BasicIndexNextAction::Invalidated,
        });
        return;
    }
    status.maybe_processed_height =
        FieldAvailability::available(progress.maybe_processed_endpoint().map(|id| id.height()));
    status.maybe_safe_durable_height =
        FieldAvailability::available(progress.maybe_safe_durable_endpoint().map(|id| id.height()));
    status.maybe_lag_blocks = FieldAvailability::available(Some(progress.current_lag()));
    let (state, action) = match status.maybe_failure {
        FieldAvailability::Available(Some(failure)) => (
            match failure {
                BasicIndexFailureCategory::RequiredBody
                | BasicIndexFailureCategory::RequiredUndo
                | BasicIndexFailureCategory::ResourceBound => BasicIndexOperationalState::Paused,
                BasicIndexFailureCategory::Invalidated => BasicIndexOperationalState::Invalidated,
                _ => BasicIndexOperationalState::Failed,
            },
            failure_action(failure),
        ),
        _ if progress.state() == BasicIndexState::Disabled => (
            BasicIndexOperationalState::Disabled,
            BasicIndexNextAction::Disabled,
        ),
        _ => {
            let state = if !progress.initially_synchronized() {
                BasicIndexOperationalState::CatchingUp
            } else if progress.current_lag() == 0 {
                BasicIndexOperationalState::Current
            } else {
                BasicIndexOperationalState::Lagged
            };
            let action = if progress.maybe_processed_endpoint().is_none() {
                BasicIndexNextAction::EmptyPrefix
            } else if progress.maybe_processed_endpoint() != progress.maybe_safe_durable_endpoint()
            {
                BasicIndexNextAction::ProcessedAheadOfDurable
            } else if state == BasicIndexOperationalState::Current {
                BasicIndexNextAction::Current
            } else if state == BasicIndexOperationalState::Lagged {
                BasicIndexNextAction::Lagged
            } else {
                BasicIndexNextAction::InitialCatchUp
            };
            (state, action)
        }
    };
    status.operational_state = FieldAvailability::available(state);
    status.next_action = FieldAvailability::available(action);
}

impl<S: ChainstateStore, V: CoinsView> ManagedPeerNetwork<S, V> {
    fn project_basic_index(&self) -> BasicIndexOperatorStatus {
        let mut status =
            BasicIndexOperatorStatus::unavailable(BasicIndexUnavailableReason::LiveUnavailable);
        status.configured_indexing = self.maybe_configured_basic_indexing().map_or_else(
            || FieldAvailability::unavailable(BasicIndexUnavailableReason::NotReported.as_str()),
            FieldAvailability::available,
        );
        status.configured_peer_serving =
            FieldAvailability::available(self.configured_basic_peer_serving());
        status.peer_scope = self.peer_manager.maybe_basic_peer_scope().map_or_else(
            || FieldAvailability::unavailable(BasicIndexUnavailableReason::Backend.as_str()),
            |scope| {
                FieldAvailability::available(BasicPeerScopeStatus {
                    general_sessions: scope.general_sessions,
                    explicit_sessions: scope.explicit_sessions,
                    all_sessions: scope.all_sessions,
                    implicit_sessions: scope.implicit_sessions,
                    offered_sessions: scope.offered_sessions,
                })
            },
        );
        let maybe_progress = self.chainstate.maybe_basic_index_progress();
        if let Some(progress) = maybe_progress {
            status.initial_sync_complete =
                FieldAvailability::available(progress.initially_synchronized());
            status.maybe_accepted_height = FieldAvailability::available(
                self.chainstate
                    .maybe_basic_index_accepted_target()
                    .map(AcceptedIndexTarget::height),
            );
            status.maybe_failure = FieldAvailability::available(
                self.chainstate
                    .maybe_basic_index_failure()
                    .map(failure_category),
            );
        }
        let Some(store) = self.chainstate.store().maybe_basic_filter_query_store() else {
            status.next_action =
                FieldAvailability::available(BasicIndexNextAction::CollectLiveEvidence);
            return status;
        };
        let observation = match store.basic_filter_operator_observation() {
            Ok(value) => value,
            Err(reason) => {
                status.retention = FieldAvailability::unavailable(reason.as_str());
                status.retained_availability = FieldAvailability::unavailable(reason.as_str());
                status.operational_state =
                    FieldAvailability::available(BasicIndexOperationalState::Failed);
                if matches!(status.maybe_failure, FieldAvailability::Available(None)) {
                    status.maybe_failure =
                        FieldAvailability::available(Some(BasicIndexFailureCategory::Backend));
                }
                status.next_action = FieldAvailability::available(BasicIndexNextAction::Backend);
                return status;
            }
        };
        debug_assert!(
            observation.work.metadata_reads <= 8 && observation.work.metadata_bytes <= 4096
        );
        if let Some(fault) = observation.maybe_read_fault {
            if let Some(protection) = observation.maybe_protection {
                status.protection = FieldAvailability::available(project_protection(protection));
            }
            project_read_fault(&mut status, fault);
            return status;
        }
        status.retained_availability = observation.retained_availability.map_or_else(
            |reason| FieldAvailability::unavailable(reason.as_str()),
            FieldAvailability::available,
        );
        match observation.retention {
            Ok(measurement) => {
                status.retention = FieldAvailability::available(measurement);
            }
            Err(reason) => {
                status.retention = FieldAvailability::unavailable(reason.as_str());
            }
        }
        let authority = match observation.authority {
            Ok(value) => value,
            Err(failure) => {
                let reason = if failure == BasicOperatorObservationFailure::Integrity {
                    BasicIndexUnavailableReason::Integrity
                } else {
                    BasicIndexUnavailableReason::Backend
                };
                if failure == BasicOperatorObservationFailure::Budget {
                    status.operational_state = FieldAvailability::unavailable(reason.as_str());
                    status.next_action =
                        FieldAvailability::available(BasicIndexNextAction::InspectSupportEvidence);
                    return status;
                }
                status.operational_state =
                    FieldAvailability::available(BasicIndexOperationalState::Failed);
                if matches!(status.maybe_failure, FieldAvailability::Available(None)) {
                    status.maybe_failure = FieldAvailability::available(Some(
                        if reason == BasicIndexUnavailableReason::Integrity {
                            BasicIndexFailureCategory::IntegrityOrInvalidFacts
                        } else {
                            BasicIndexFailureCategory::Backend
                        },
                    ));
                }
                status.next_action =
                    FieldAvailability::available(BasicIndexNextAction::InspectSupportEvidence);
                return status;
            }
        };
        if let Some((_, _, _, _, protection)) = authority.maybe_publication {
            status.protection = FieldAvailability::available(project_protection(protection));
        }
        if self.maybe_configured_basic_indexing() == Some(false)
            && authority.integrity
            && !matches!(
                authority.maybe_lifecycle,
                Some(IndexLifecycle::Active { .. })
            )
        {
            status.operational_state =
                FieldAvailability::available(BasicIndexOperationalState::Disabled);
            status.protection = FieldAvailability::available(BasicIndexProtection::None);
            status.next_action = FieldAvailability::available(BasicIndexNextAction::Disabled);
        } else if let Some(progress) = maybe_progress {
            project_basic_owner(
                &mut status,
                progress,
                self.chainstate.maybe_basic_index_accepted_target(),
                self.chainstate
                    .chainstate()
                    .tip()
                    .map(|tip| AcceptedIndexTarget::new(tip.height, tip.block_hash)),
                &authority,
            );
        } else if !authority.has_state {
            status.next_action = FieldAvailability::available(BasicIndexNextAction::EmptyPrefix);
        }
        status
    }

    pub fn operator_snapshot(&self) -> ManagedNetworkOperatorSnapshot {
        let block_relay = self.block_relay_runtime_evidence_snapshot();
        let mempool = self.mempool_info();
        let network = self.network_info();
        let relay_enabled = network.relay;
        let relay = self.relay_evidence_status();
        let evidence = self.lifecycle_evidence;
        let pressure_removal_count = evidence.pressure_removals;
        let outcome_counters = match &relay.outcome_counters {
            RelayEvidenceField::Implemented(counters) => *counters,
            RelayEvidenceField::Unavailable { .. }
            | RelayEvidenceField::Deferred { .. }
            | RelayEvidenceField::IntentionallyDifferent { .. } => RelayEvidenceCounters::default(),
        };
        let checkpoint = checkpoint_group_from_evidence(&self.checkpoint_evidence.snapshot(
            self.lifecycle_generation,
            self.dirty_generation,
            self.mempool.mempool().rolling_fee_last_update(),
            OPERATOR_CHECKPOINT_INTERVAL_SECONDS,
        ));
        let recovery = self
            .latest_mempool_recovery
            .as_ref()
            .map(recovery_group_from_summary)
            .unwrap_or_default();
        ManagedNetworkOperatorSnapshot {
            basic_index: self.project_basic_index(),
            network,
            mempool: mempool.clone(),
            relay,
            block_relay: block_relay.status,
            chainstate_durability: self.project_chainstate_durability(),
            block_served_count: block_relay.served_count,
            inbound_admission: self.inbound_admission_info().clone(),
            address_boundary: self.address_boundary_info(),
            peer_policy: self.peer_policy_info(),
            resource_governance: self.resource_governance_info(),
            pressure: MempoolPressureGroup {
                pressure_removal_count,
                decay_half_life_label: decay_half_life_label(
                    mempool.accounted_memory,
                    mempool.mempool_capacity,
                    self.mempool.mempool().rolling_fee_decay_gate_open(),
                )
                .to_string(),
            },
            eviction: MempoolEvictionGroup {
                pressure_removal_count,
            },
            checkpoint,
            recovery,
            retry: retry_group_from_relay(
                self.unbroadcast_members.len() as u64,
                self.relay_fanout_info().queued_transactions as u64,
                0,
                outcome_counters.announced_count,
                &outcome_counters,
                relay_enabled,
                evidence.retry_clears,
            ),
            admission: admission_group_from_counts(
                evidence.admitted_members,
                mempool.transaction_count as u64,
                evidence.removed_members,
            ),
        }
    }

    fn project_chainstate_durability(&self) -> FieldAvailability<ChainstateDurabilityEvidence> {
        let lifecycle = &self.chainstate.flush_lifecycle;
        if lifecycle.readiness() == ManagerReadiness::NotReady
            && lifecycle.maybe_recovery_outcome().is_none()
        {
            return ChainstateDurabilityEvidence::default_unavailable();
        }

        let coins = self.chainstate.chainstate().coins();
        let maybe_coins_best_block = self
            .chainstate
            .chainstate()
            .coins_best_block()
            .ok()
            .flatten();
        let maybe_coins_best_block_height = maybe_coins_best_block.and_then(|block_hash| {
            self.peer_manager
                .header_store()
                .entry(&block_hash)
                .map(|entry| u64::from(entry.height))
        });
        lifecycle.project_chainstate_durability(
            coins.estimated_cache_bytes(),
            coins.cache_entry_count(),
            FlushPolicyTime::from_unix_seconds(0),
            u64::MAX,
            maybe_coins_best_block,
            maybe_coins_best_block_height,
            self.have_bytes_accumulator.snapshot(),
        )
    }

    pub fn unbroadcast_member_count(&self) -> u64 {
        self.unbroadcast_members.len() as u64
    }

    #[cfg(test)]
    pub(crate) fn insert_unbroadcast_member_for_test(&mut self, identity: MempoolMemberIdentity) {
        self.unbroadcast_members.insert(identity);
    }
}

#[cfg(test)]
mod tests;

#[cfg(test)]
#[test]
fn phase161_read_fault_projection_dominates_existing_owner_pause() {
    use crate::storage::fjall_store::filters::retention::BasicFilterReadFault;
    for (fault, expected) in [
        (
            BasicFilterReadFault::Backend,
            BasicIndexFailureCategory::Backend,
        ),
        (
            BasicFilterReadFault::Integrity,
            BasicIndexFailureCategory::IntegrityOrInvalidFacts,
        ),
    ] {
        for previous in [
            BasicIndexFailureCategory::RequiredBody,
            BasicIndexFailureCategory::RequiredUndo,
            BasicIndexFailureCategory::ResourceBound,
        ] {
            // Arrange
            let mut status =
                BasicIndexOperatorStatus::unavailable(BasicIndexUnavailableReason::LiveUnavailable);
            status.maybe_failure = FieldAvailability::available(Some(previous));
            status.initial_sync_complete = FieldAvailability::available(true);
            status.configured_indexing = FieldAvailability::available(true);
            status.maybe_accepted_height = FieldAvailability::available(Some(42));
            // Act
            project_read_fault(&mut status, fault);
            // Assert
            assert_eq!(
                status.maybe_failure,
                FieldAvailability::available(Some(expected))
            );
            assert_eq!(
                status.operational_state,
                FieldAvailability::available(BasicIndexOperationalState::Failed)
            );
            assert_eq!(
                status.next_action,
                FieldAvailability::available(failure_action(expected))
            );
            assert_eq!(
                status.initial_sync_complete,
                FieldAvailability::available(true)
            );
            assert_eq!(
                status.configured_indexing,
                FieldAvailability::available(true)
            );
            assert_eq!(
                status.maybe_accepted_height,
                FieldAvailability::available(Some(42))
            );
            assert!(matches!(
                status.retention,
                FieldAvailability::Unavailable { .. }
            ));
            assert!(matches!(
                status.maybe_processed_height,
                FieldAvailability::Unavailable { .. }
            ));
        }
    }
}
