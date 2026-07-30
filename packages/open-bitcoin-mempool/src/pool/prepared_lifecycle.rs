// Parity breadcrumbs:
// - packages/bitcoin-knots/src/txmempool.cpp
// - packages/bitcoin-knots/src/validation.cpp

//! Sealed, revision-bound mempool transition preparation.

#[cfg(test)]
use std::cell::Cell;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use open_bitcoin_chainstate::ChainstateSnapshot;
use open_bitcoin_consensus::{
    ConsensusParams, ScriptVerifyFlags, transaction_txid, transaction_wtxid,
};
use open_bitcoin_primitives::{Transaction, Txid, Wtxid};

use crate::{
    AdmissionContext, AdmissionResult, FinalMempoolMembership, MempoolError, MempoolLifecycleDelta,
    MempoolLifecycleRemoval, MempoolMemberIdentity, PackageReport, SubmitPackageCommand,
};

use super::admission::prepare_admission_patch;
use super::candidate::{check_candidate_scripts, prepare_candidate};
use super::package_admission::evaluate_package;
use super::{Mempool, MempoolPatch, MempoolRevision};

#[cfg(test)]
thread_local! {
    static SINGLETON_PREPARE_COUNT: Cell<usize> = const { Cell::new(0) };
    static PACKAGE_PREPARE_COUNT: Cell<usize> = const { Cell::new(0) };
    static APPLY_COUNT: Cell<usize> = const { Cell::new(0) };
}

/// A canonical identity paired with its fully validated transaction body.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PreparedMempoolMember {
    pub member: MempoolMemberIdentity,
    pub transaction: Transaction,
    pub metadata: crate::MempoolEntryMetadata,
}

/// A semantic removal paired with the body present before the transition.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PreparedMempoolRemoval {
    pub removal: MempoolLifecycleRemoval,
    pub transaction: Transaction,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum PreparedTransitionResult {
    Admission(AdmissionResult),
    Package(PackageReport),
    Maintenance,
}

/// Immutable, fully preflighted facts for one mempool transition.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PreparedLifecycleFacts {
    delta: MempoolLifecycleDelta,
    final_present: Vec<PreparedMempoolMember>,
    removed: Vec<PreparedMempoolRemoval>,
    admitted_order: Vec<MempoolMemberIdentity>,
    teardown_order: Vec<MempoolMemberIdentity>,
    result: PreparedTransitionResult,
}

impl PreparedLifecycleFacts {
    /// Returns the exact committed lifecycle vocabulary.
    pub const fn delta(&self) -> &MempoolLifecycleDelta {
        &self.delta
    }

    /// Returns final-present bodies in admitted topological order.
    pub fn final_present(&self) -> &[PreparedMempoolMember] {
        &self.final_present
    }

    /// Returns removed bodies with their semantic lifecycle facts.
    pub fn removed(&self) -> &[PreparedMempoolRemoval] {
        &self.removed
    }

    /// Returns final-present admission identities in transition order.
    pub fn admitted_order(&self) -> &[MempoolMemberIdentity] {
        &self.admitted_order
    }

    /// Returns the preflighted teardown order.
    pub fn teardown_order(&self) -> &[MempoolMemberIdentity] {
        &self.teardown_order
    }

    /// Returns the singleton attempt result when this is an admission transition.
    pub fn maybe_admission_result(&self) -> Option<&AdmissionResult> {
        let PreparedTransitionResult::Admission(result) = &self.result else {
            return None;
        };
        Some(result)
    }

    /// Returns the package report when this is a package transition.
    pub fn maybe_package_report(&self) -> Option<&PackageReport> {
        let PreparedTransitionResult::Package(report) = &self.result else {
            return None;
        };
        Some(report)
    }
}

enum PreparedCoreTransition {
    Patch(Box<MempoolPatch>),
    Noop { base_revision: MempoolRevision },
}

impl PreparedCoreTransition {
    const fn base_revision(&self) -> MempoolRevision {
        match self {
            Self::Patch(patch) => patch.base_revision,
            Self::Noop { base_revision } => *base_revision,
        }
    }
}

/// Opaque, non-`Clone` capability for one revision-bound transition.
///
/// A capability cannot be consumed twice because validation takes ownership:
///
/// ```compile_fail,E0382
/// use open_bitcoin_mempool::{Mempool, PreparedMempoolTransition};
///
/// fn consume_twice(mempool: &mut Mempool, prepared: PreparedMempoolTransition) {
///     let _first = mempool.commit_prepared_mempool_transition(prepared);
///     let _second = mempool.commit_prepared_mempool_transition(prepared);
/// }
/// ```
pub struct PreparedMempoolTransition {
    core: PreparedCoreTransition,
    facts: PreparedLifecycleFacts,
}

/// Opaque proof that a prepared transition still matches the current mempool revision.
pub struct SealedMempoolTransition {
    core: PreparedCoreTransition,
    facts: PreparedLifecycleFacts,
}

impl fmt::Debug for PreparedMempoolTransition {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("PreparedMempoolTransition")
            .field("base_revision", &self.core.base_revision())
            .field("facts", &self.facts)
            .finish_non_exhaustive()
    }
}

impl PreparedMempoolTransition {
    /// Borrows the immutable facts needed by dependent projection preparation.
    pub const fn facts(&self) -> &PreparedLifecycleFacts {
        &self.facts
    }
}

impl Mempool {
    /// Fully prepares singleton admission without mutating the mempool.
    pub fn prepare_transaction_with_context(
        &self,
        transaction: Transaction,
        chainstate: &ChainstateSnapshot,
        verify_flags: ScriptVerifyFlags,
        consensus_params: ConsensusParams,
        context: AdmissionContext,
    ) -> Result<PreparedMempoolTransition, MempoolError> {
        #[cfg(test)]
        SINGLETON_PREPARE_COUNT.with(|count| count.set(count.get() + 1));
        let prepared = prepare_candidate(self, transaction, chainstate, consensus_params, context)?;
        let (patch, result) = prepare_admission_patch(self, &prepared)?;
        check_candidate_scripts(&prepared, verify_flags)?;
        PreparedMempoolTransition::from_patch(
            self,
            patch,
            PreparedTransitionResult::Admission(result),
        )
    }

    /// Fully evaluates a checked package without mutating the mempool.
    pub fn prepare_package(
        &self,
        command: SubmitPackageCommand,
        chainstate: &ChainstateSnapshot,
        verify_flags: ScriptVerifyFlags,
        consensus_params: ConsensusParams,
    ) -> Result<PreparedMempoolTransition, MempoolError> {
        #[cfg(test)]
        PACKAGE_PREPARE_COUNT.with(|count| count.set(count.get() + 1));
        let _submission_kind = command.package.kind();
        let evaluation = evaluate_package(
            self,
            command.package.package(),
            command.context,
            chainstate,
            verify_flags,
            consensus_params,
        )?;
        let result = PreparedTransitionResult::Package(evaluation.report);
        let Some(patch) = evaluation.patch else {
            return Ok(PreparedMempoolTransition::noop(self, result));
        };
        PreparedMempoolTransition::from_patch(self, patch, result)
    }

    /// Atomically checks and commits one prepared transition.
    pub fn commit_prepared_mempool_transition(
        &mut self,
        prepared: PreparedMempoolTransition,
    ) -> Result<MempoolLifecycleDelta, MempoolError> {
        let sealed = self.seal_prepared_mempool_transition(prepared)?;
        Ok(self.commit_sealed_mempool_transition(sealed))
    }

    /// Validates the revision guard without mutating the mempool.
    pub fn seal_prepared_mempool_transition(
        &self,
        prepared: PreparedMempoolTransition,
    ) -> Result<SealedMempoolTransition, MempoolError> {
        let expected_revision = prepared.core.base_revision();
        if self.revision != expected_revision {
            return Err(MempoolError::StalePreparedTransition {
                expected_revision: expected_revision.0,
                actual_revision: self.revision.0,
            });
        }
        Ok(SealedMempoolTransition {
            core: prepared.core,
            facts: prepared.facts,
        })
    }

    /// Commits a revision-sealed transition without a remaining failure path.
    pub fn commit_sealed_mempool_transition(
        &mut self,
        sealed: SealedMempoolTransition,
    ) -> MempoolLifecycleDelta {
        #[cfg(test)]
        APPLY_COUNT.with(|count| count.set(count.get() + 1));
        match sealed.core {
            PreparedCoreTransition::Patch(patch) => self.apply_validated_patch(*patch),
            PreparedCoreTransition::Noop { .. } => sealed.facts.delta,
        }
    }
}

impl PreparedMempoolTransition {
    pub(super) fn admission_result_for_facade(&self) -> Result<AdmissionResult, MempoolError> {
        self.facts.maybe_admission_result().cloned().ok_or_else(|| {
            prepared_invariant_error("singleton facade received a package transition".to_string())
        })
    }

    pub(super) fn package_report_for_facade(&self) -> Result<PackageReport, MempoolError> {
        self.facts.maybe_package_report().cloned().ok_or_else(|| {
            prepared_invariant_error("package facade received a singleton transition".to_string())
        })
    }

    pub(super) fn maintenance_from_patch(
        mempool: &Mempool,
        patch: MempoolPatch,
    ) -> Result<Self, MempoolError> {
        Self::from_patch(mempool, patch, PreparedTransitionResult::Maintenance)
    }

    pub(super) fn maintenance_noop(mempool: &Mempool) -> Self {
        Self::noop(mempool, PreparedTransitionResult::Maintenance)
    }

    fn from_patch(
        mempool: &Mempool,
        patch: MempoolPatch,
        result: PreparedTransitionResult,
    ) -> Result<Self, MempoolError> {
        let facts = PreparedLifecycleFacts::from_patch(mempool, &patch, result)?;
        Ok(Self {
            core: PreparedCoreTransition::Patch(Box::new(patch)),
            facts,
        })
    }

    fn noop(mempool: &Mempool, result: PreparedTransitionResult) -> Self {
        Self {
            core: PreparedCoreTransition::Noop {
                base_revision: mempool.revision,
            },
            facts: PreparedLifecycleFacts {
                delta: MempoolLifecycleDelta::empty(),
                final_present: Vec::new(),
                removed: Vec::new(),
                admitted_order: Vec::new(),
                teardown_order: Vec::new(),
                result,
            },
        }
    }
}

impl PreparedLifecycleFacts {
    fn from_patch(
        mempool: &Mempool,
        patch: &MempoolPatch,
        result: PreparedTransitionResult,
    ) -> Result<Self, MempoolError> {
        let final_membership = final_membership_by_identity(&patch.delta)?;
        let admitted_order = patch
            .delta
            .admitted
            .iter()
            .copied()
            .filter(|member| final_membership.get(member) == Some(&FinalMempoolMembership::Present))
            .collect::<Vec<_>>();
        let final_present = admitted_order
            .iter()
            .copied()
            .map(|member| prepared_member(mempool, patch, member))
            .collect::<Result<Vec<_>, _>>()?;
        let removed = patch
            .delta
            .removed
            .iter()
            .cloned()
            .map(|removal| prepared_removal(mempool, patch, removal))
            .collect::<Result<Vec<_>, _>>()?;
        let teardown_order = graph_teardown_order(&removed);

        Ok(Self {
            delta: patch.delta.clone(),
            final_present,
            removed,
            admitted_order,
            teardown_order,
            result,
        })
    }
}

fn graph_teardown_order(removals: &[PreparedMempoolRemoval]) -> Vec<MempoolMemberIdentity> {
    let removal_members = removals
        .iter()
        .map(|removed| (removed.removal.member.txid, removed.removal.member))
        .collect::<BTreeMap<_, _>>();
    let mut children = BTreeMap::<Txid, BTreeSet<Txid>>::new();
    for removed in removals {
        let child_txid = removed.removal.member.txid;
        for input in &removed.transaction.inputs {
            let parent_txid = input.previous_output.txid;
            if removal_members.contains_key(&parent_txid) {
                children.entry(parent_txid).or_default().insert(child_txid);
            }
        }
    }
    let mut visited = BTreeSet::new();
    let mut ordered = Vec::with_capacity(removal_members.len());

    for txid in removal_members.keys().copied() {
        visit_removed_descendants(
            &removal_members,
            &children,
            txid,
            &mut visited,
            &mut ordered,
        );
    }
    ordered
}

fn visit_removed_descendants(
    removal_members: &BTreeMap<Txid, MempoolMemberIdentity>,
    children: &BTreeMap<Txid, BTreeSet<Txid>>,
    txid: Txid,
    visited: &mut BTreeSet<Txid>,
    ordered: &mut Vec<MempoolMemberIdentity>,
) {
    if !visited.insert(txid) {
        return;
    }
    for child_txid in children.get(&txid).into_iter().flatten().copied() {
        visit_removed_descendants(removal_members, children, child_txid, visited, ordered);
    }
    ordered.push(removal_members[&txid]);
}

fn final_membership_by_identity(
    delta: &MempoolLifecycleDelta,
) -> Result<BTreeMap<MempoolMemberIdentity, FinalMempoolMembership>, MempoolError> {
    let mut by_txid = BTreeMap::<Txid, MempoolMemberIdentity>::new();
    let mut by_wtxid = BTreeMap::<Wtxid, MempoolMemberIdentity>::new();
    let mut membership = BTreeMap::new();
    for state in &delta.final_membership {
        validate_identity_maps(&mut by_txid, &mut by_wtxid, state.member)?;
        if membership.insert(state.member, state.membership).is_some() {
            return Err(prepared_invariant_error(format!(
                "duplicate final membership for {:?}",
                state.member
            )));
        }
    }
    for member in delta
        .admitted
        .iter()
        .copied()
        .chain(delta.removed.iter().map(|removal| removal.member))
        .chain(delta.retry_clears.iter().map(|clear| clear.member))
    {
        validate_identity_maps(&mut by_txid, &mut by_wtxid, member)?;
        if !membership.contains_key(&member) {
            return Err(prepared_invariant_error(format!(
                "missing final membership for {member:?}"
            )));
        }
    }
    Ok(membership)
}

fn validate_identity_maps(
    by_txid: &mut BTreeMap<Txid, MempoolMemberIdentity>,
    by_wtxid: &mut BTreeMap<Wtxid, MempoolMemberIdentity>,
    member: MempoolMemberIdentity,
) -> Result<(), MempoolError> {
    if by_txid
        .insert(member.txid, member)
        .is_some_and(|existing| existing != member)
        || by_wtxid
            .insert(member.wtxid, member)
            .is_some_and(|existing| existing != member)
    {
        return Err(prepared_invariant_error(format!(
            "conflicting prepared identity {member:?}"
        )));
    }
    Ok(())
}

fn prepared_member(
    mempool: &Mempool,
    patch: &MempoolPatch,
    member: MempoolMemberIdentity,
) -> Result<PreparedMempoolMember, MempoolError> {
    let entry = patch
        .entry_upserts
        .get(&member.txid)
        .or_else(|| mempool.entries.get(&member.txid))
        .ok_or_else(|| prepared_invariant_error(format!("missing body for {member:?}")))?;
    validate_body_identity(&entry.transaction, member)?;
    Ok(PreparedMempoolMember {
        member,
        transaction: entry.transaction.clone(),
        metadata: entry.metadata,
    })
}

fn prepared_removal(
    mempool: &Mempool,
    patch: &MempoolPatch,
    removal: MempoolLifecycleRemoval,
) -> Result<PreparedMempoolRemoval, MempoolError> {
    let member = removal.member;
    let entry = mempool
        .entries
        .get(&member.txid)
        .or_else(|| patch.entry_upserts.get(&member.txid))
        .ok_or_else(|| prepared_invariant_error(format!("missing removal body for {member:?}")))?;
    validate_body_identity(&entry.transaction, member)?;
    Ok(PreparedMempoolRemoval {
        removal,
        transaction: entry.transaction.clone(),
    })
}

fn validate_body_identity(
    transaction: &Transaction,
    member: MempoolMemberIdentity,
) -> Result<(), MempoolError> {
    let txid = transaction_txid(transaction)
        .map_err(|source| super::serialization_validation_error("transaction txid", source))?;
    let wtxid = transaction_wtxid(transaction)
        .map_err(|source| super::serialization_validation_error("transaction wtxid", source))?;
    if txid != member.txid || wtxid != member.wtxid {
        return Err(prepared_invariant_error(format!(
            "body identity txid={txid:?}, wtxid={wtxid:?} does not match {member:?}"
        )));
    }
    Ok(())
}

fn prepared_invariant_error(reason: String) -> MempoolError {
    MempoolError::InternalInvariant { reason }
}

#[cfg(test)]
pub(super) fn reset_transition_counts_for_test() {
    SINGLETON_PREPARE_COUNT.with(|count| count.set(0));
    PACKAGE_PREPARE_COUNT.with(|count| count.set(0));
    APPLY_COUNT.with(|count| count.set(0));
}

#[cfg(test)]
pub(super) fn transition_counts_for_test() -> (usize, usize, usize) {
    (
        SINGLETON_PREPARE_COUNT.with(Cell::get),
        PACKAGE_PREPARE_COUNT.with(Cell::get),
        APPLY_COUNT.with(Cell::get),
    )
}

#[cfg(test)]
mod tests {
    use open_bitcoin_primitives::{Transaction, Txid, Wtxid};

    use super::{final_membership_by_identity, validate_body_identity, validate_identity_maps};
    use crate::{
        FinalMempoolMembership, MempoolLifecycleDelta, MempoolMemberIdentity, MempoolMemberState,
    };

    fn identity(txid_byte: u8, wtxid_byte: u8) -> MempoolMemberIdentity {
        MempoolMemberIdentity {
            txid: Txid::from_byte_array([txid_byte; 32]),
            wtxid: Wtxid::from_byte_array([wtxid_byte; 32]),
        }
    }

    #[test]
    fn prepared_membership_rejects_duplicate_and_missing_states() {
        // Arrange
        let member = identity(1, 2);
        let duplicate = MempoolLifecycleDelta {
            final_membership: vec![
                MempoolMemberState {
                    member,
                    membership: FinalMempoolMembership::Present,
                },
                MempoolMemberState {
                    member,
                    membership: FinalMempoolMembership::Absent,
                },
            ],
            ..MempoolLifecycleDelta::empty()
        };
        let missing = MempoolLifecycleDelta {
            admitted: vec![member],
            ..MempoolLifecycleDelta::empty()
        };

        // Act
        let duplicate_error =
            final_membership_by_identity(&duplicate).expect_err("duplicate must fail");
        let missing_error = final_membership_by_identity(&missing).expect_err("missing must fail");

        // Assert
        assert!(
            duplicate_error
                .to_string()
                .contains("duplicate final membership")
        );
        assert!(
            missing_error
                .to_string()
                .contains("missing final membership")
        );
    }

    #[test]
    fn prepared_identity_maps_reject_cross_identity_collisions() {
        // Arrange
        let mut by_txid = std::collections::BTreeMap::new();
        let mut by_wtxid = std::collections::BTreeMap::new();
        let first = identity(1, 2);
        let conflicting = identity(1, 3);
        validate_identity_maps(&mut by_txid, &mut by_wtxid, first).expect("first identity");

        // Act
        let error = validate_identity_maps(&mut by_txid, &mut by_wtxid, conflicting)
            .expect_err("collision must fail");

        // Assert
        assert!(error.to_string().contains("conflicting prepared identity"));
    }

    #[test]
    fn prepared_body_validation_rejects_identity_mismatch() {
        // Arrange
        let transaction = Transaction::default();
        let mismatched = identity(1, 2);

        // Act
        let error =
            validate_body_identity(&transaction, mismatched).expect_err("mismatch must fail");

        // Assert
        assert!(error.to_string().contains("body identity"));
    }
}
