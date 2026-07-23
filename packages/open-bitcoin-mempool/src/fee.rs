// Parity breadcrumbs:
// - packages/bitcoin-knots/src/txmempool.h
// - packages/bitcoin-knots/src/txmempool.cpp
// - packages/bitcoin-knots/src/policy/policy.h
// - packages/bitcoin-knots/src/policy/rbf.cpp
// - packages/bitcoin-knots/src/policy/packages.cpp

//! Fee-rate arithmetic and compile-time-distinct mempool policy roles.

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
    pub fn from_fee_sats_and_vbytes(fee_sats: i64, virtual_size: usize) -> Self {
        if virtual_size == 0 {
            return Self::ZERO;
        }

        let virtual_size = i64::try_from(virtual_size).unwrap_or(i64::MAX);
        let sats_per_kvb =
            (fee_sats.saturating_mul(SATOSHIS_PER_KILOVBYTE) + virtual_size - 1) / virtual_size;
        Self { sats_per_kvb }
    }

    /// Returns satoshis per 1,000 virtual bytes.
    pub const fn sats_per_kvb(self) -> i64 {
        self.sats_per_kvb
    }

    /// Calculates the rounded-up fee for a virtual size.
    pub fn fee_for_virtual_size(self, virtual_size: usize) -> i64 {
        if virtual_size == 0 {
            return 0;
        }

        let virtual_size = i64::try_from(virtual_size).unwrap_or(i64::MAX);
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
