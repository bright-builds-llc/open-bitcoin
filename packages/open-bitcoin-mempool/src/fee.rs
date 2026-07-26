// Parity breadcrumbs:
// - packages/bitcoin-knots/doc/policy/packages.md
// - packages/bitcoin-knots/src/kernel/mempool_options.h
// - packages/bitcoin-knots/src/kernel/mempool_removal_reason.h
// - packages/bitcoin-knots/src/policy/packages.cpp
// - packages/bitcoin-knots/src/policy/policy.h
// - packages/bitcoin-knots/src/policy/policy.cpp
// - packages/bitcoin-knots/src/policy/rbf.cpp
// - packages/bitcoin-knots/src/rpc/mempool.cpp
// - packages/bitcoin-knots/src/script/script.cpp
// - packages/bitcoin-knots/src/test/txvalidation_tests.cpp
// - packages/bitcoin-knots/src/txmempool.cpp
// - packages/bitcoin-knots/src/txmempool.h
// - packages/bitcoin-knots/test/functional/mempool_ephemeral_dust.py

//! Fee-rate arithmetic and compile-time-distinct mempool policy roles.

pub mod rolling;

use core::fmt;

use open_bitcoin_primitives::{Amount, Wtxid};

use crate::package::{EffectiveFeeGroup, EffectiveFeeGroupError, EffectiveFeeGroupId};
use crate::pool::MempoolMemberIdentity;
use crate::resource::{ResourceAccountingError, TransactionVirtualSize};
use crate::types::TrucPolicy;

const SATOSHIS_PER_KILOVBYTE: i64 = 1_000;
const FEE_RATE_ROUNDING_ADJUSTMENT: i64 = SATOSHIS_PER_KILOVBYTE - 1;

/// A role-neutral fee rate expressed in satoshis per 1,000 virtual bytes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct FeeRate {
    sats_per_kvb: i64,
}

impl FeeRate {
    /// A zero fee rate.
    pub const ZERO: Self = Self { sats_per_kvb: 0 };

    /// Creates a fee rate from satoshis per 1,000 virtual bytes.
    pub const fn from_sats_per_kvb(sats_per_kvb: i64) -> Self {
        Self { sats_per_kvb }
    }

    /// Derives a rounded-up rate from a fee and virtual size.
    pub fn from_fee_sats_and_vbytes(fee_sats: i64, virtual_size: TransactionVirtualSize) -> Self {
        if virtual_size == TransactionVirtualSize::ZERO {
            return Self::ZERO;
        }

        let virtual_size = i64::try_from(virtual_size.as_usize()).unwrap_or(i64::MAX);
        let sats_per_kvb =
            (fee_sats.saturating_mul(SATOSHIS_PER_KILOVBYTE) + virtual_size - 1) / virtual_size;
        Self { sats_per_kvb }
    }

    /// Returns satoshis per 1,000 virtual bytes.
    pub const fn sats_per_kvb(self) -> i64 {
        self.sats_per_kvb
    }

    /// Calculates the rounded-up fee for a virtual size.
    pub fn fee_for_virtual_size(self, virtual_size: TransactionVirtualSize) -> i64 {
        if virtual_size == TransactionVirtualSize::ZERO {
            return 0;
        }

        let virtual_size = i64::try_from(virtual_size.as_usize()).unwrap_or(i64::MAX);
        (self.sats_per_kvb.saturating_mul(virtual_size) + FEE_RATE_ROUNDING_ADJUSTMENT)
            / SATOSHIS_PER_KILOVBYTE
    }
}

impl core::fmt::Display for FeeRate {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(formatter, "{} sat/kvB", self.sats_per_kvb)
    }
}

/// The configured anti-free-relay floor applied to every ordinary transaction.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct StaticRelayFeeRate(FeeRate);

impl StaticRelayFeeRate {
    /// Creates a static relay floor.
    pub const fn new(fee_rate: FeeRate) -> Self {
        Self(fee_rate)
    }

    /// Returns the role-neutral arithmetic value.
    pub const fn fee_rate(self) -> FeeRate {
        self.0
    }
}

/// The independent fee rate used to classify transaction outputs as dust.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct DustRelayFeeRate(FeeRate);

impl DustRelayFeeRate {
    /// The pinned Bitcoin Knots dust relay rate.
    pub const DEFAULT: Self = Self(FeeRate::from_sats_per_kvb(3_000));

    /// Creates a dust relay rate.
    pub const fn new(fee_rate: FeeRate) -> Self {
        Self(fee_rate)
    }

    /// Returns the role-neutral arithmetic value.
    pub const fn fee_rate(self) -> FeeRate {
        self.0
    }
}

impl Default for DustRelayFeeRate {
    fn default() -> Self {
        Self::DEFAULT
    }
}

/// The configured fee increment used by replacement and pressure bumps.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct IncrementalRelayFeeRate(FeeRate);

impl IncrementalRelayFeeRate {
    /// Creates an incremental relay rate.
    pub const fn new(fee_rate: FeeRate) -> Self {
        Self(fee_rate)
    }

    /// Returns the role-neutral arithmetic value.
    pub const fn fee_rate(self) -> FeeRate {
        self.0
    }
}

/// The dynamic floor established by mempool pressure.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct RollingMempoolFeeRate(FeeRate);

impl RollingMempoolFeeRate {
    /// The restart and Phase-130 baseline before rolling behavior exists.
    pub const ZERO: Self = Self(FeeRate::ZERO);

    /// Creates a rolling mempool floor.
    pub const fn new(fee_rate: FeeRate) -> Self {
        Self(fee_rate)
    }

    /// Returns the role-neutral arithmetic value.
    pub const fn fee_rate(self) -> FeeRate {
        self.0
    }
}

/// The derived ordinary-admission floor.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct EffectiveAdmissionFeeRate(FeeRate);

impl EffectiveAdmissionFeeRate {
    const fn new_derived(fee_rate: FeeRate) -> Self {
        Self(fee_rate)
    }

    /// Returns the role-neutral arithmetic value.
    pub const fn fee_rate(self) -> FeeRate {
        self.0
    }
}

/// Derives the ordinary-admission floor from static and rolling policy only.
pub fn effective_admission_fee_rate(
    static_floor: StaticRelayFeeRate,
    rolling_floor: RollingMempoolFeeRate,
) -> EffectiveAdmissionFeeRate {
    EffectiveAdmissionFeeRate::new_derived(core::cmp::max(
        static_floor.fee_rate(),
        rolling_floor.fee_rate(),
    ))
}

/// Separate package member and eligible aggregate fee-floor decisions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PackageFeeFloorAssessment {
    member_meets_static_floor: bool,
    aggregate_meets_rolling_floor: bool,
}

impl PackageFeeFloorAssessment {
    /// Whether the ordinary package member independently meets the static floor.
    pub const fn member_meets_static_floor(self) -> bool {
        self.member_meets_static_floor
    }

    /// Whether the eligible package aggregate meets the rolling floor.
    pub const fn aggregate_meets_rolling_floor(self) -> bool {
        self.aggregate_meets_rolling_floor
    }
}

/// Evaluates static member and rolling aggregate obligations independently.
pub fn evaluate_package_fee_floors(
    member_fee_rate: FeeRate,
    aggregate_fee_rate: FeeRate,
    static_floor: StaticRelayFeeRate,
    rolling_floor: RollingMempoolFeeRate,
) -> PackageFeeFloorAssessment {
    PackageFeeFloorAssessment {
        member_meets_static_floor: member_fee_rate >= static_floor.fee_rate(),
        aggregate_meets_rolling_floor: aggregate_fee_rate >= rolling_floor.fee_rate(),
    }
}

/// Base and policy-modified fee facts retained throughout admission.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CandidateFees {
    pub base: Amount,
    pub modified: Amount,
}

/// One eligible new package member participating in effective-fee assessment.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PackageFeeMember {
    pub identity: MempoolMemberIdentity,
    pub version: i32,
    pub fees: CandidateFees,
    pub virtual_size: TransactionVirtualSize,
}

/// Checked arithmetic for one non-empty ordered effective-fee group.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackageFeeGroupAssessment {
    ordered_wtxids: Vec<Wtxid>,
    base_fee_sats: Amount,
    modified_fee_sats: Amount,
    virtual_size: TransactionVirtualSize,
    effective_fee_rate: FeeRate,
}

impl PackageFeeGroupAssessment {
    /// Constructs the opaque report group with a caller-owned request-order identifier.
    pub fn try_effective_fee_group(
        &self,
        id: EffectiveFeeGroupId,
    ) -> Result<EffectiveFeeGroup, EffectiveFeeGroupError> {
        EffectiveFeeGroup::try_new(
            id,
            self.ordered_wtxids.clone(),
            self.base_fee_sats,
            self.modified_fee_sats,
            self.virtual_size,
            self.effective_fee_rate,
        )
    }

    /// Borrows the exact request-ordered eligible membership.
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

    /// Returns the checked modified-fee rate.
    pub const fn effective_fee_rate(&self) -> FeeRate {
        self.effective_fee_rate
    }
}

/// Checked package fee failure with static and rolling obligations kept distinct.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PackageFeeError {
    EmptyGroup,
    TrucRejected {
        member: MempoolMemberIdentity,
    },
    StaticFloorNotMet {
        member: MempoolMemberIdentity,
        fee: Amount,
        required_fee_sats: i64,
    },
    RollingFloorNotMet {
        assessment: PackageFeeGroupAssessment,
        required_fee_sats: i64,
    },
    BaseFeeOverflow,
    ModifiedFeeOverflow,
    VirtualSize(ResourceAccountingError),
    InvalidBaseFeeTotal,
    InvalidModifiedFeeTotal,
}

impl fmt::Display for PackageFeeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyGroup => write!(formatter, "package fee group must not be empty"),
            Self::TrucRejected { member } => {
                write!(
                    formatter,
                    "version 3 transaction {:?} rejected by TRUC policy",
                    member.txid
                )
            }
            Self::StaticFloorNotMet {
                member,
                fee,
                required_fee_sats,
            } => write!(
                formatter,
                "package member {:?} fee {} is below static relay minimum {}",
                member.txid,
                fee.to_sats(),
                required_fee_sats
            ),
            Self::RollingFloorNotMet {
                assessment,
                required_fee_sats,
            } => write!(
                formatter,
                "package modified fee {} is below rolling minimum {} for {} vbytes",
                assessment.modified_fee_sats.to_sats(),
                required_fee_sats,
                assessment.virtual_size.as_usize()
            ),
            Self::BaseFeeOverflow => write!(formatter, "package base fee total overflowed"),
            Self::ModifiedFeeOverflow => write!(formatter, "package modified fee total overflowed"),
            Self::VirtualSize(error) => write!(formatter, "{error}"),
            Self::InvalidBaseFeeTotal => {
                write!(formatter, "package base fee total is out of range")
            }
            Self::InvalidModifiedFeeTotal => {
                write!(formatter, "package modified fee total is out of range")
            }
        }
    }
}

impl std::error::Error for PackageFeeError {}

/// Evaluates member-static and aggregate-rolling obligations for one eligible group.
pub fn evaluate_package_fee_group(
    members: &[PackageFeeMember],
    static_floor: StaticRelayFeeRate,
    rolling_floor: RollingMempoolFeeRate,
    truc_policy: TrucPolicy,
) -> Result<PackageFeeGroupAssessment, PackageFeeError> {
    if members.is_empty() {
        return Err(PackageFeeError::EmptyGroup);
    }

    let mut ordered_wtxids = Vec::with_capacity(members.len());
    let mut base_fee_sats = 0_i64;
    let mut modified_fee_sats = 0_i64;
    let mut virtual_size = TransactionVirtualSize::ZERO;
    for member in members {
        let enforced_truc = member.version == 3 && truc_policy == TrucPolicy::Enforce;
        if member.version == 3 && truc_policy == TrucPolicy::Reject {
            return Err(PackageFeeError::TrucRejected {
                member: member.identity,
            });
        }
        let required_static_fee = static_floor
            .fee_rate()
            .fee_for_virtual_size(member.virtual_size);
        if !enforced_truc && member.fees.modified.to_sats() < required_static_fee {
            return Err(PackageFeeError::StaticFloorNotMet {
                member: member.identity,
                fee: member.fees.modified,
                required_fee_sats: required_static_fee,
            });
        }

        ordered_wtxids.push(member.identity.wtxid);
        base_fee_sats = base_fee_sats
            .checked_add(member.fees.base.to_sats())
            .ok_or(PackageFeeError::BaseFeeOverflow)?;
        modified_fee_sats = modified_fee_sats
            .checked_add(member.fees.modified.to_sats())
            .ok_or(PackageFeeError::ModifiedFeeOverflow)?;
        virtual_size = virtual_size
            .checked_add(member.virtual_size, "package fee group virtual size")
            .map_err(PackageFeeError::VirtualSize)?;
    }

    let base_fee_sats =
        Amount::from_sats(base_fee_sats).map_err(|_| PackageFeeError::InvalidBaseFeeTotal)?;
    let modified_fee_sats = Amount::from_sats(modified_fee_sats)
        .map_err(|_| PackageFeeError::InvalidModifiedFeeTotal)?;
    let assessment = PackageFeeGroupAssessment {
        ordered_wtxids,
        base_fee_sats,
        modified_fee_sats,
        virtual_size,
        effective_fee_rate: FeeRate::from_fee_sats_and_vbytes(
            modified_fee_sats.to_sats(),
            virtual_size,
        ),
    };
    let required_rolling_fee = rolling_floor.fee_rate().fee_for_virtual_size(virtual_size);
    if modified_fee_sats.to_sats() < required_rolling_fee {
        return Err(PackageFeeError::RollingFloorNotMet {
            assessment,
            required_fee_sats: required_rolling_fee,
        });
    }
    Ok(assessment)
}
