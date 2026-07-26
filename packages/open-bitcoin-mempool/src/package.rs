// Parity breadcrumbs:
// - packages/bitcoin-knots/src/policy/packages.h
// - packages/bitcoin-knots/src/policy/packages.cpp
// - packages/bitcoin-knots/src/validation.h
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/test/txpackage_tests.cpp

//! Opaque package identities and submission refinements.

mod report;
mod shape;

use open_bitcoin_consensus::crypto::Sha256;
use open_bitcoin_primitives::{Transaction, Txid, Wtxid};

pub use report::{
    EffectiveFeeGroup, EffectiveFeeGroupError, EffectiveFeeGroupId, ExistingMember,
    HardMemberFailure, NewlyPresent, PackageMemberResult, PackageReport, PackageReportError,
    PackageStatus, PostTrimAbsence, PriorMemberSuccess, ReconsiderableMemberFailure, WitnessAlias,
};
pub use shape::PackageShapeError;

use crate::{AdmissionContext, MempoolLifecycleDelta, MempoolMemberIdentity};

/// Maximum number of transactions admitted through one package boundary.
pub const MAX_PACKAGE_COUNT: usize = 25;
/// Maximum combined BIP141 weight admitted through one package boundary.
pub const MAX_PACKAGE_WEIGHT: usize = 404_000;

#[derive(Debug, Clone, PartialEq, Eq)]
struct PackageMember {
    transaction: Transaction,
    identity: MempoolMemberIdentity,
    weight: usize,
    input_index: usize,
}

/// A Knots-compatible, permutation-independent package fingerprint.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PackageFingerprint([u8; 32]);

impl PackageFingerprint {
    fn from_members(members: &[PackageMember]) -> Self {
        let wtxids: Vec<Wtxid> = members.iter().map(|member| member.identity.wtxid).collect();
        Self::from_wtxids(&wtxids)
    }

    fn from_wtxids(wtxids: &[Wtxid]) -> Self {
        let mut sorted_wtxids = wtxids.to_vec();
        sorted_wtxids.sort_by(|left, right| {
            left.as_bytes()
                .iter()
                .rev()
                .cmp(right.as_bytes().iter().rev())
        });

        let mut preimage = Vec::with_capacity(sorted_wtxids.len() * 32);
        for wtxid in sorted_wtxids {
            preimage.extend_from_slice(wtxid.as_bytes());
        }
        Self(Sha256::digest(&preimage))
    }

    /// Borrows the canonical raw digest bytes.
    pub const fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}

/// A context-free checked package whose private storage remains request ordered.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WellFormedPackage {
    members: Vec<PackageMember>,
    fingerprint: PackageFingerprint,
}

impl WellFormedPackage {
    /// Returns the number of request-ordered members.
    pub fn len(&self) -> usize {
        self.members.len()
    }

    /// Returns whether this checked package contains no members.
    ///
    /// A successfully constructed package always returns `false`.
    pub fn is_empty(&self) -> bool {
        self.members.is_empty()
    }

    /// Iterates over transactions without exposing mutable package storage.
    pub fn members(&self) -> impl DoubleEndedIterator<Item = &Transaction> + ExactSizeIterator {
        self.members.iter().map(|member| &member.transaction)
    }

    pub(crate) fn members_with_identities(
        &self,
    ) -> impl ExactSizeIterator<Item = (MempoolMemberIdentity, &Transaction)> {
        self.members
            .iter()
            .map(|member| (member.identity, &member.transaction))
    }

    /// Returns the cached identity at an input index, if that index exists.
    pub fn maybe_identity_at(&self, index: usize) -> Option<MempoolMemberIdentity> {
        self.members.get(index).map(|member| member.identity)
    }

    /// Looks up a cached member identity by txid.
    pub fn maybe_identity_for_txid(&self, txid: Txid) -> Option<MempoolMemberIdentity> {
        self.members
            .iter()
            .find(|member| member.identity.txid == txid)
            .map(|member| member.identity)
    }

    /// Looks up a cached member identity by wtxid.
    pub fn maybe_identity_for_wtxid(&self, wtxid: Wtxid) -> Option<MempoolMemberIdentity> {
        self.members
            .iter()
            .find(|member| member.identity.wtxid == wtxid)
            .map(|member| member.identity)
    }

    /// Returns the original request index stored for one member.
    pub fn maybe_input_index_at(&self, index: usize) -> Option<usize> {
        self.members.get(index).map(|member| member.input_index)
    }

    /// Returns the checked total package weight.
    pub fn total_weight(&self) -> usize {
        self.members.iter().map(|member| member.weight).sum()
    }

    /// Borrows the permutation-independent package fingerprint.
    pub const fn fingerprint(&self) -> &PackageFingerprint {
        &self.fingerprint
    }
}

/// The checked submission capability represented by a package.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SubmissionPackageKind {
    Single,
    ChildWithUnconfirmedParents,
}

/// A package refined for the selected local submission surface.
///
/// Direct construction is forbidden; callers must use
/// [`SubmissionPackage::try_from_package`].
///
/// ```compile_fail,E0451
/// use open_bitcoin_mempool::{
///     SubmissionPackage, SubmissionPackageKind, WellFormedPackage,
/// };
///
/// fn forge(package: WellFormedPackage) -> SubmissionPackage {
///     let kind = SubmissionPackageKind::ChildWithUnconfirmedParents;
///     SubmissionPackage { package, kind: kind }
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubmissionPackage {
    package: WellFormedPackage,
    kind: SubmissionPackageKind,
}

impl SubmissionPackage {
    /// Borrows the checked request-ordered package.
    pub fn package(&self) -> &WellFormedPackage {
        &self.package
    }

    /// Returns the capability kind proven by the checked refinement.
    pub fn kind(&self) -> SubmissionPackageKind {
        self.kind
    }
}

/// A non-mutating package evaluation request.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DryRunPackageCommand {
    pub package: WellFormedPackage,
    pub context: AdmissionContext,
}

/// A checked child-with-unconfirmed-parents submission request.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubmitPackageCommand {
    pub package: SubmissionPackage,
    pub context: AdmissionContext,
}

/// Prospective package facts with no committed-state capability.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DryRunPackageResult {
    pub report: PackageReport,
}

/// Package facts paired with the lifecycle delta committed by submission.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubmittedPackageResult {
    pub report: PackageReport,
    pub delta: MempoolLifecycleDelta,
}

#[cfg(test)]
mod tests;
