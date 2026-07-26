// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/node/txdownloadman_impl.cpp
// - packages/bitcoin-knots/src/txorphanage.cpp
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/test/functional/p2p_opportunistic_1p1c.py

use open_bitcoin_core::{
    consensus::{ConsensusParams, ScriptVerifyFlags, transaction_txid, transaction_wtxid},
    primitives::{Transaction, Txid, Wtxid},
};
use open_bitcoin_mempool::{
    AdmissionContext, HardMemberFailure, MempoolError, MempoolLifecycleDelta, MempoolOutcome,
    MempoolRejectionCategory, MempoolRemovalCause, MempoolTransition, PackageMemberResult,
    PolicyTime, ReconsiderableMemberFailure, SubmissionPackage, SubmitPackageCommand,
    SubmittedPackageResult, WellFormedPackage,
};
use open_bitcoin_network::{OrphanStageInput, PeerId, PeerManager, ReceivedTransactionProvenance};

use super::{ManagedAdmissionBridgeResult, ManagedNetworkError, ManagedPeerNetwork};
use crate::ChainstateStore;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::network) struct ManagedPeerPackageAdmission {
    pub origins: [PeerId; 2],
    pub submitted: SubmittedPackageResult,
}

pub(in crate::network) enum ManagedPeerAdmissionResult {
    Singleton(ManagedAdmissionBridgeResult),
    Package(ManagedPeerPackageAdmission),
    Suppressed,
}

#[derive(Debug, Clone, Copy)]
struct PeerAdmissionOptions {
    timestamp: i64,
    verify_flags: ScriptVerifyFlags,
    consensus_params: ConsensusParams,
}

impl<S: ChainstateStore> ManagedPeerNetwork<S> {
    pub(in crate::network) fn process_peer_transaction_admission_with_provenance(
        &mut self,
        transaction: Transaction,
        provenance: ReceivedTransactionProvenance,
        timestamp: i64,
        verify_flags: ScriptVerifyFlags,
        consensus_params: ConsensusParams,
    ) -> Result<ManagedPeerAdmissionResult, ManagedNetworkError> {
        let txid = transaction_txid(&transaction)?;
        let wtxid = transaction_wtxid(&transaction)?;
        let options = PeerAdmissionOptions {
            timestamp,
            verify_flags,
            consensus_params,
        };
        if self.peer_manager.reconsiderable_transaction_contains(wtxid) {
            return self
                .submit_same_peer_candidate(
                    transaction,
                    txid,
                    wtxid,
                    provenance.delivered_by,
                    options,
                )
                .map(|maybe_package| {
                    maybe_package.map_or(
                        ManagedPeerAdmissionResult::Suppressed,
                        ManagedPeerAdmissionResult::Package,
                    )
                });
        }

        let submitted = self.submit_peer_singleton(
            transaction.clone(),
            timestamp,
            verify_flags,
            consensus_params,
        )?;
        let member =
            submitted
                .report
                .members()
                .first()
                .ok_or_else(|| MempoolError::InternalInvariant {
                    reason: "singleton package report omitted its member".to_string(),
                })?;
        let reconsiderable = matches!(
            member,
            PackageMemberResult::Reconsiderable(
                ReconsiderableMemberFailure::PackageFee { .. }
                    | ReconsiderableMemberFailure::PackageReplacement { .. }
            )
        );
        let singleton_package_replacement = matches!(
            member,
            PackageMemberResult::HardRejected(HardMemberFailure::PackageReplacement { .. })
                | PackageMemberResult::Reconsiderable(
                    ReconsiderableMemberFailure::PackageReplacement { .. }
                )
        );
        record_singleton_reject_evidence(&mut self.peer_manager, member);
        if reconsiderable
            && let Some(package_admission) = self.submit_same_peer_candidate(
                transaction.clone(),
                txid,
                wtxid,
                provenance.delivered_by,
                options,
            )?
        {
            return Ok(ManagedPeerAdmissionResult::Package(package_admission));
        }

        let transition = if singleton_package_replacement {
            self.mempool.submit_transaction_transition_with_context(
                &self.chainstate,
                transaction.clone(),
                verify_flags,
                consensus_params,
                AdmissionContext::peer(PolicyTime::from_unix_seconds(timestamp)),
            )?
        } else {
            singleton_transition_from_package(submitted)?
        };
        let mut result = ManagedAdmissionBridgeResult::new(transition.clone());

        match &transition.outcome {
            MempoolOutcome::Accepted { txid, .. } | MempoolOutcome::Replaced { txid, .. } => {
                self.apply_admitted_transition(&transition, transaction)?;
                let child_result = self.reconsider_orphans_after_acceptance(
                    *txid,
                    timestamp,
                    verify_flags,
                    consensus_params,
                )?;
                result
                    .targeted_outbound
                    .extend(child_result.targeted_outbound);
                result.reconsidered.extend(child_result.reconsidered);
            }
            MempoolOutcome::Orphaned {
                txid,
                wtxid,
                missing_parents,
            } => {
                self.compact_extra_txn.push(*wtxid, transaction.clone());
                let actions = self.peer_manager.stage_missing_parent_with_provenance(
                    OrphanStageInput {
                        transaction,
                        txid: *txid,
                        wtxid: *wtxid,
                        missing_parents: missing_parents.clone(),
                        now_unix_seconds: timestamp,
                    },
                    provenance,
                );
                self.apply_orphan_actions(actions, timestamp, &mut result)?;
            }
            MempoolOutcome::Rejected { wtxid, .. } => {
                let _ = self
                    .compact_extra_txn
                    .push_gated(*wtxid, transaction.clone());
            }
            MempoolOutcome::Duplicate { .. } | MempoolOutcome::Evicted { .. } => {}
            MempoolOutcome::Expired { .. } => {}
        }

        Ok(ManagedPeerAdmissionResult::Singleton(result))
    }

    pub(super) fn submit_peer_singleton(
        &mut self,
        transaction: Transaction,
        timestamp: i64,
        verify_flags: ScriptVerifyFlags,
        consensus_params: ConsensusParams,
    ) -> Result<SubmittedPackageResult, ManagedNetworkError> {
        let chainstate = self.chainstate.chainstate().snapshot();
        let package = WellFormedPackage::try_from(vec![transaction])?;
        let submission = SubmissionPackage::try_from_package(package, &chainstate)?;
        self.mempool
            .submit_package(
                SubmitPackageCommand {
                    package: submission,
                    context: AdmissionContext::peer(PolicyTime::from_unix_seconds(timestamp)),
                },
                &chainstate,
                verify_flags,
                consensus_params,
            )
            .map_err(ManagedNetworkError::from)
    }

    fn submit_same_peer_candidate(
        &mut self,
        parent: Transaction,
        parent_txid: Txid,
        parent_wtxid: Wtxid,
        parent_peer: PeerId,
        options: PeerAdmissionOptions,
    ) -> Result<Option<ManagedPeerPackageAdmission>, ManagedNetworkError> {
        let mut maybe_candidate = self.peer_manager.begin_same_peer_candidate(
            parent,
            parent_txid,
            parent_wtxid,
            parent_peer,
        );
        while let Some(candidate) = maybe_candidate {
            let (members, origins, provenances) = candidate.into_ordered_parts_with_provenance();
            let chainstate = self.chainstate.chainstate().snapshot();
            let checked = WellFormedPackage::try_from(Vec::from(members.clone()))?;
            let fingerprint = *checked.fingerprint().as_bytes();
            if self
                .peer_manager
                .reconsiderable_package_contains(fingerprint)
            {
                maybe_candidate = self
                    .peer_manager
                    .advance_same_peer_candidate(parent_wtxid, parent_peer);
                continue;
            }
            let package = SubmissionPackage::try_from_package(checked, &chainstate)?;
            let submitted = self.mempool.submit_package(
                SubmitPackageCommand {
                    package,
                    context: AdmissionContext::peer(PolicyTime::from_unix_seconds(
                        options.timestamp,
                    )),
                },
                &chainstate,
                options.verify_flags,
                options.consensus_params,
            )?;
            debug_assert_eq!(submitted.report.fingerprint().as_bytes(), &fingerprint);
            self.apply_package_feedback(&members, &provenances, &submitted, options.timestamp);
            return Ok(Some(ManagedPeerPackageAdmission { origins, submitted }));
        }
        Ok(None)
    }
}

pub(super) fn record_singleton_reject_evidence(
    peer_manager: &mut PeerManager,
    member: &PackageMemberResult,
) {
    match member {
        PackageMemberResult::HardRejected(HardMemberFailure::Policy { .. })
        | PackageMemberResult::HardRejected(HardMemberFailure::TrucPolicy { .. })
        | PackageMemberResult::HardRejected(HardMemberFailure::EphemeralPolicy { .. }) => {
            peer_manager.record_hard_reject(member.requested_identity().wtxid);
        }
        PackageMemberResult::HardRejected(HardMemberFailure::PackageReplacement { .. }) => {}
        PackageMemberResult::Reconsiderable(
            ReconsiderableMemberFailure::PackageFee { .. }
            | ReconsiderableMemberFailure::PackageReplacement { .. },
        ) => {
            peer_manager.record_reconsiderable_transaction(member.requested_identity().wtxid);
        }
        PackageMemberResult::FinallyPresent(_)
        | PackageMemberResult::AlreadyPresent(_)
        | PackageMemberResult::SameTxidDifferentWitness(_)
        | PackageMemberResult::Reconsiderable(ReconsiderableMemberFailure::MissingInputs {
            ..
        })
        | PackageMemberResult::PostTrimAbsent(_) => {}
    }
}

pub(super) fn singleton_transition_from_package(
    submitted: SubmittedPackageResult,
) -> Result<MempoolTransition, ManagedNetworkError> {
    let SubmittedPackageResult { report, delta } = submitted;
    let member = report
        .members()
        .first()
        .ok_or_else(|| MempoolError::InternalInvariant {
            reason: "singleton package report omitted its member".to_string(),
        })?;
    singleton_transition_from_member(member, delta)
}

#[cfg(test)]
pub(super) fn singleton_transition_from_package_member(
    submitted: &SubmittedPackageResult,
    index: usize,
    delta: MempoolLifecycleDelta,
) -> Result<MempoolTransition, ManagedNetworkError> {
    let member =
        submitted
            .report
            .members()
            .get(index)
            .ok_or_else(|| MempoolError::InternalInvariant {
                reason: format!("package report omitted member {index}"),
            })?;
    singleton_transition_from_member(member, delta)
}

fn singleton_transition_from_member(
    member: &PackageMemberResult,
    delta: MempoolLifecycleDelta,
) -> Result<MempoolTransition, ManagedNetworkError> {
    let requested = member.requested_identity();
    let outcome = match member {
        PackageMemberResult::FinallyPresent(_) => {
            let replaced = removed_txids(&delta, MempoolRemovalCause::Replacement);
            let evicted = removed_txids(&delta, MempoolRemovalCause::Pressure);
            if replaced.is_empty() {
                MempoolOutcome::Accepted {
                    txid: requested.txid,
                    wtxid: requested.wtxid,
                    evicted,
                }
            } else {
                MempoolOutcome::Replaced {
                    txid: requested.txid,
                    wtxid: requested.wtxid,
                    replaced,
                    evicted,
                }
            }
        }
        PackageMemberResult::AlreadyPresent(_)
        | PackageMemberResult::SameTxidDifferentWitness(_) => MempoolOutcome::Duplicate {
            txid: requested.txid,
        },
        PackageMemberResult::HardRejected(failure) => MempoolOutcome::Rejected {
            txid: requested.txid,
            wtxid: requested.wtxid,
            category: hard_rejection_category(failure),
        },
        PackageMemberResult::Reconsiderable(ReconsiderableMemberFailure::MissingInputs {
            missing_parents,
            ..
        }) => MempoolOutcome::Orphaned {
            txid: requested.txid,
            wtxid: requested.wtxid,
            missing_parents: missing_parents.clone(),
        },
        PackageMemberResult::Reconsiderable(ReconsiderableMemberFailure::PackageFee { .. }) => {
            MempoolOutcome::Rejected {
                txid: requested.txid,
                wtxid: requested.wtxid,
                category: MempoolRejectionCategory::RelayFeeTooLow,
            }
        }
        PackageMemberResult::Reconsiderable(ReconsiderableMemberFailure::PackageReplacement {
            ..
        }) => MempoolOutcome::Rejected {
            txid: requested.txid,
            wtxid: requested.wtxid,
            category: MempoolRejectionCategory::ReplacementRejected,
        },
        PackageMemberResult::PostTrimAbsent(_) => MempoolOutcome::Evicted {
            txid: requested.txid,
            wtxid: requested.wtxid,
        },
    };
    Ok(MempoolTransition { outcome, delta })
}

fn hard_rejection_category(failure: &HardMemberFailure) -> MempoolRejectionCategory {
    match failure {
        HardMemberFailure::Policy { .. } => MempoolRejectionCategory::InternalInvariant,
        HardMemberFailure::TrucPolicy { .. } | HardMemberFailure::EphemeralPolicy { .. } => {
            MempoolRejectionCategory::NonStandard
        }
        HardMemberFailure::PackageReplacement { .. } => {
            MempoolRejectionCategory::ReplacementRejected
        }
    }
}

fn removed_txids(delta: &MempoolLifecycleDelta, cause: MempoolRemovalCause) -> Vec<Txid> {
    delta
        .removed
        .iter()
        .filter(|removal| removal.cause == cause)
        .map(|removal| removal.member.txid)
        .collect()
}
