// Parity breadcrumbs:
// - packages/bitcoin-knots/src/validation.h
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/test/txpackage_tests.cpp

use std::collections::{HashMap, HashSet};
use std::fmt;

use open_bitcoin_primitives::{Amount, Wtxid};

use super::{PackageFingerprint, WellFormedPackage};
use crate::{FeeRate, MempoolMemberIdentity, TransactionVirtualSize};

/// Stable identity for one effective-fee calculation group.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EffectiveFeeGroupId(u64);

impl EffectiveFeeGroupId {
    /// Creates an identifier from its stable scalar representation.
    pub const fn from_u64(value: u64) -> Self {
        Self(value)
    }

    /// Returns the stable scalar representation.
    pub const fn as_u64(self) -> u64 {
        self.0
    }
}

/// Failure to construct a non-empty, internally consistent fee group.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EffectiveFeeGroupError {
    EmptyMembership,
    DuplicateMembership { wtxid: Wtxid },
    ZeroVirtualSize,
    VirtualSizeOutOfRange { virtual_size: usize },
    InconsistentEffectiveRate { expected: FeeRate, actual: FeeRate },
}

impl fmt::Display for EffectiveFeeGroupError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyMembership => write!(formatter, "effective fee group must not be empty"),
            Self::DuplicateMembership { wtxid } => {
                write!(formatter, "effective fee group repeats wtxid {wtxid:?}")
            }
            Self::ZeroVirtualSize => {
                write!(
                    formatter,
                    "effective fee group virtual size must be non-zero"
                )
            }
            Self::VirtualSizeOutOfRange { virtual_size } => write!(
                formatter,
                "effective fee group virtual size {virtual_size} exceeds the checked fee range"
            ),
            Self::InconsistentEffectiveRate { expected, actual } => write!(
                formatter,
                "effective fee rate {actual} does not match checked rate {expected}"
            ),
        }
    }
}

impl std::error::Error for EffectiveFeeGroupError {}

/// A checked, non-empty set of request-ordered members used for one fee calculation.
///
/// `Amount` makes negative or out-of-range Bitcoin fee aggregates unrepresentable,
/// while `try_new` checks the remaining cross-field arithmetic.
///
/// ```compile_fail,E0451
/// use open_bitcoin_mempool::{
///     EffectiveFeeGroup, EffectiveFeeGroupId, FeeRate, TransactionVirtualSize,
/// };
/// use open_bitcoin_primitives::{Amount, Wtxid};
///
/// fn forge(
///     id: EffectiveFeeGroupId,
///     ordered_wtxids: Vec<Wtxid>,
///     base_fee_sats: Amount,
///     modified_fee_sats: Amount,
///     virtual_size: TransactionVirtualSize,
///     effective_fee_rate: FeeRate,
/// ) -> EffectiveFeeGroup {
///     EffectiveFeeGroup { id, ordered_wtxids, base_fee_sats, modified_fee_sats, virtual_size, effective_fee_rate }
/// }
/// ```
///
/// ```compile_fail,E0616
/// use open_bitcoin_mempool::EffectiveFeeGroup;
/// use open_bitcoin_primitives::Wtxid;
///
/// fn mutate(group: &mut EffectiveFeeGroup, wtxid: Wtxid) {
///     group.ordered_wtxids.push(wtxid);
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EffectiveFeeGroup {
    id: EffectiveFeeGroupId,
    ordered_wtxids: Vec<Wtxid>,
    base_fee_sats: Amount,
    modified_fee_sats: Amount,
    virtual_size: TransactionVirtualSize,
    effective_fee_rate: FeeRate,
}

impl EffectiveFeeGroup {
    /// Checks non-empty membership, uniqueness, size, and effective-rate consistency.
    pub fn try_new(
        id: EffectiveFeeGroupId,
        ordered_wtxids: Vec<Wtxid>,
        base_fee_sats: Amount,
        modified_fee_sats: Amount,
        virtual_size: TransactionVirtualSize,
        effective_fee_rate: FeeRate,
    ) -> Result<Self, EffectiveFeeGroupError> {
        if ordered_wtxids.is_empty() {
            return Err(EffectiveFeeGroupError::EmptyMembership);
        }

        let mut unique_wtxids = HashSet::with_capacity(ordered_wtxids.len());
        for wtxid in &ordered_wtxids {
            if !unique_wtxids.insert(*wtxid) {
                return Err(EffectiveFeeGroupError::DuplicateMembership { wtxid: *wtxid });
            }
        }

        if virtual_size == TransactionVirtualSize::ZERO {
            return Err(EffectiveFeeGroupError::ZeroVirtualSize);
        }
        if i64::try_from(virtual_size.as_usize()).is_err() {
            return Err(EffectiveFeeGroupError::VirtualSizeOutOfRange {
                virtual_size: virtual_size.as_usize(),
            });
        }

        let expected = FeeRate::from_fee_sats_and_vbytes(modified_fee_sats.to_sats(), virtual_size);
        if effective_fee_rate != expected {
            return Err(EffectiveFeeGroupError::InconsistentEffectiveRate {
                expected,
                actual: effective_fee_rate,
            });
        }

        Ok(Self {
            id,
            ordered_wtxids,
            base_fee_sats,
            modified_fee_sats,
            virtual_size,
            effective_fee_rate,
        })
    }

    /// Returns the stable group identifier.
    pub const fn id(&self) -> EffectiveFeeGroupId {
        self.id
    }

    /// Borrows request-ordered fee-calculation membership.
    pub fn ordered_wtxids(&self) -> &[Wtxid] {
        &self.ordered_wtxids
    }

    /// Returns the checked aggregate base fee.
    pub const fn base_fee_sats(&self) -> Amount {
        self.base_fee_sats
    }

    /// Returns the checked aggregate modified fee.
    pub const fn modified_fee_sats(&self) -> Amount {
        self.modified_fee_sats
    }

    /// Returns the checked aggregate virtual size.
    pub const fn virtual_size(&self) -> TransactionVirtualSize {
        self.virtual_size
    }

    /// Returns the effective rate checked against modified fee and virtual size.
    pub const fn effective_fee_rate(&self) -> FeeRate {
        self.effective_fee_rate
    }
}

/// Prospective outcome of the entire ordered package request.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PackageStatus {
    Complete,
    Partial,
    Failed,
}

/// A newly evaluated member that reached final mempool presence.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NewlyPresent {
    pub requested: MempoolMemberIdentity,
    pub effective_fee_group_id: EffectiveFeeGroupId,
}

/// An exact requested member that was already present.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExistingMember {
    pub requested: MempoolMemberIdentity,
}

/// A request whose txid exists with another witness serialization.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WitnessAlias {
    pub requested: MempoolMemberIdentity,
    pub existing_wtxid: Wtxid,
}

/// A non-reconsiderable member failure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HardMemberFailure {
    Policy {
        requested: MempoolMemberIdentity,
        reason: String,
    },
    PackageReplacement {
        requested: MempoolMemberIdentity,
        reason: String,
    },
}

/// A member failure that a later package or input arrival may reconsider.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReconsiderableMemberFailure {
    MissingInputs {
        requested: MempoolMemberIdentity,
    },
    PackageFee {
        requested: MempoolMemberIdentity,
        effective_fee_group_id: EffectiveFeeGroupId,
    },
}

/// The typed successful state a later mempool trim removed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PriorMemberSuccess {
    FinallyPresent {
        effective_fee_group_id: EffectiveFeeGroupId,
    },
    AlreadyPresent,
    SameTxidDifferentWitness {
        existing_wtxid: Wtxid,
    },
}

/// A member absent after trimming, retaining the successful state it previously reached.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PostTrimAbsence {
    pub requested: MempoolMemberIdentity,
    pub prior: PriorMemberSuccess,
}

/// Explicit result vocabulary for one request-ordered member.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PackageMemberResult {
    FinallyPresent(NewlyPresent),
    AlreadyPresent(ExistingMember),
    SameTxidDifferentWitness(WitnessAlias),
    HardRejected(HardMemberFailure),
    Reconsiderable(ReconsiderableMemberFailure),
    PostTrimAbsent(PostTrimAbsence),
}

impl PackageMemberResult {
    /// Returns the identity supplied by the original package request.
    pub const fn requested_identity(&self) -> MempoolMemberIdentity {
        match self {
            Self::FinallyPresent(result) => result.requested,
            Self::AlreadyPresent(result) => result.requested,
            Self::SameTxidDifferentWitness(result) => result.requested,
            Self::HardRejected(
                HardMemberFailure::Policy { requested, .. }
                | HardMemberFailure::PackageReplacement { requested, .. },
            ) => *requested,
            Self::Reconsiderable(ReconsiderableMemberFailure::MissingInputs { requested }) => {
                *requested
            }
            Self::Reconsiderable(ReconsiderableMemberFailure::PackageFee { requested, .. }) => {
                *requested
            }
            Self::PostTrimAbsent(result) => result.requested,
        }
    }

    const fn maybe_effective_fee_group_id(&self) -> Option<EffectiveFeeGroupId> {
        match self {
            Self::FinallyPresent(result) => Some(result.effective_fee_group_id),
            Self::Reconsiderable(ReconsiderableMemberFailure::PackageFee {
                effective_fee_group_id,
                ..
            }) => Some(*effective_fee_group_id),
            Self::PostTrimAbsent(PostTrimAbsence {
                prior:
                    PriorMemberSuccess::FinallyPresent {
                        effective_fee_group_id,
                    },
                ..
            }) => Some(*effective_fee_group_id),
            Self::AlreadyPresent(_)
            | Self::SameTxidDifferentWitness(_)
            | Self::HardRejected(_)
            | Self::Reconsiderable(ReconsiderableMemberFailure::MissingInputs { .. })
            | Self::PostTrimAbsent(PostTrimAbsence {
                prior:
                    PriorMemberSuccess::AlreadyPresent
                    | PriorMemberSuccess::SameTxidDifferentWitness { .. },
                ..
            }) => None,
        }
    }

    const fn is_present(&self) -> bool {
        matches!(
            self,
            Self::FinallyPresent(_) | Self::AlreadyPresent(_) | Self::SameTxidDifferentWitness(_)
        )
    }
}

/// Failure to align an ordered package report with its checked package.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PackageReportError {
    MemberCountMismatch {
        expected: usize,
        actual: usize,
    },
    IdentityMismatch {
        index: usize,
    },
    StatusMismatch {
        supplied: PackageStatus,
        derived: PackageStatus,
    },
    DuplicateFeeGroupId {
        id: EffectiveFeeGroupId,
    },
    DuplicateCrossGroupMembership {
        wtxid: Wtxid,
    },
    UnexpectedFeeGroup {
        id: EffectiveFeeGroupId,
    },
    FeeGroupMembersMismatch {
        id: EffectiveFeeGroupId,
    },
    MissingFeeGroup {
        id: EffectiveFeeGroupId,
    },
}

impl fmt::Display for PackageReportError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MemberCountMismatch { expected, actual } => write!(
                formatter,
                "package report has {actual} members; expected {expected}"
            ),
            Self::IdentityMismatch { index } => {
                write!(
                    formatter,
                    "package report member {index} identity is misaligned"
                )
            }
            Self::StatusMismatch { supplied, derived } => write!(
                formatter,
                "package report status {supplied:?} does not match derived {derived:?}"
            ),
            Self::DuplicateFeeGroupId { id } => {
                write!(formatter, "effective fee group id {id:?} is duplicated")
            }
            Self::DuplicateCrossGroupMembership { wtxid } => write!(
                formatter,
                "wtxid {wtxid:?} appears in more than one effective fee group"
            ),
            Self::UnexpectedFeeGroup { id } => {
                write!(
                    formatter,
                    "effective fee group {id:?} has no eligible members"
                )
            }
            Self::FeeGroupMembersMismatch { id } => write!(
                formatter,
                "effective fee group {id:?} does not match eligible request order"
            ),
            Self::MissingFeeGroup { id } => {
                write!(
                    formatter,
                    "eligible members reference missing fee group {id:?}"
                )
            }
        }
    }
}

impl std::error::Error for PackageReportError {}

/// Checked request-ordered package results and effective-fee evidence.
///
/// ```compile_fail,E0451
/// use open_bitcoin_mempool::{
///     EffectiveFeeGroup, PackageFingerprint, PackageMemberResult, PackageReport, PackageStatus,
/// };
///
/// fn forge(
///     fingerprint: PackageFingerprint,
///     status: PackageStatus,
///     members: Vec<PackageMemberResult>,
///     effective_fee_groups: Vec<EffectiveFeeGroup>,
/// ) -> PackageReport {
///     PackageReport { fingerprint, status, members, effective_fee_groups }
/// }
/// ```
///
/// ```compile_fail,E0616
/// use open_bitcoin_mempool::{PackageReport, PackageStatus};
///
/// fn mutate(report: &mut PackageReport, status: PackageStatus) {
///     report.status = status;
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackageReport {
    fingerprint: PackageFingerprint,
    status: PackageStatus,
    members: Vec<PackageMemberResult>,
    effective_fee_groups: Vec<EffectiveFeeGroup>,
}

impl PackageReport {
    /// Checks cardinality, request order, status, and fee-group eligibility.
    pub fn try_new(
        package: &WellFormedPackage,
        status: PackageStatus,
        members: Vec<PackageMemberResult>,
        effective_fee_groups: Vec<EffectiveFeeGroup>,
    ) -> Result<Self, PackageReportError> {
        if members.len() != package.len() {
            return Err(PackageReportError::MemberCountMismatch {
                expected: package.len(),
                actual: members.len(),
            });
        }

        for (index, (package_member, member)) in package.members.iter().zip(&members).enumerate() {
            let expected = package_member.identity;
            let actual = member.requested_identity();
            if actual != expected {
                return Err(PackageReportError::IdentityMismatch { index });
            }
        }

        let present_count = members.iter().filter(|member| member.is_present()).count();
        let derived = if present_count == members.len() {
            PackageStatus::Complete
        } else if present_count == 0 {
            PackageStatus::Failed
        } else {
            PackageStatus::Partial
        };
        if status != derived {
            return Err(PackageReportError::StatusMismatch {
                supplied: status,
                derived,
            });
        }

        validate_effective_fee_groups(&members, &effective_fee_groups)?;
        Ok(Self {
            fingerprint: *package.fingerprint(),
            status,
            members,
            effective_fee_groups,
        })
    }

    /// Borrows the package fingerprint.
    pub fn fingerprint(&self) -> &PackageFingerprint {
        &self.fingerprint
    }

    /// Borrows the deterministic package status.
    pub fn status(&self) -> &PackageStatus {
        &self.status
    }

    /// Borrows request-ordered member results.
    pub fn members(&self) -> &[PackageMemberResult] {
        &self.members
    }

    /// Borrows checked effective-fee groups.
    pub fn effective_fee_groups(&self) -> &[EffectiveFeeGroup] {
        &self.effective_fee_groups
    }
}

fn validate_effective_fee_groups(
    members: &[PackageMemberResult],
    groups: &[EffectiveFeeGroup],
) -> Result<(), PackageReportError> {
    let mut expected_by_id: HashMap<EffectiveFeeGroupId, Vec<Wtxid>> = HashMap::new();
    for member in members {
        if let Some(id) = member.maybe_effective_fee_group_id() {
            expected_by_id
                .entry(id)
                .or_default()
                .push(member.requested_identity().wtxid);
        }
    }

    let mut seen_ids = HashSet::with_capacity(groups.len());
    let mut seen_wtxids = HashSet::new();
    for group in groups {
        if !seen_ids.insert(group.id()) {
            return Err(PackageReportError::DuplicateFeeGroupId { id: group.id() });
        }
        for wtxid in group.ordered_wtxids() {
            if !seen_wtxids.insert(*wtxid) {
                return Err(PackageReportError::DuplicateCrossGroupMembership { wtxid: *wtxid });
            }
        }

        let Some(expected) = expected_by_id.remove(&group.id()) else {
            return Err(PackageReportError::UnexpectedFeeGroup { id: group.id() });
        };
        if group.ordered_wtxids() != expected {
            return Err(PackageReportError::FeeGroupMembersMismatch { id: group.id() });
        }
    }

    if let Some(id) = expected_by_id.keys().copied().min() {
        return Err(PackageReportError::MissingFeeGroup { id });
    }
    Ok(())
}
