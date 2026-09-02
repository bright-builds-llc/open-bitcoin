// Parity breadcrumbs:
// - packages/bitcoin-knots/src/coins.h
// - packages/bitcoin-knots/src/coins.cpp
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp
// - packages/bitcoin-knots/src/node/chainstate.cpp

/// Knots `GetCoinsCacheSizeState` LARGE numerator (`9` of `9/10`).
pub const LARGE_CACHE_NUMERATOR: u64 = 9;
/// Knots `GetCoinsCacheSizeState` LARGE denominator (`10` of `9/10`).
pub const LARGE_CACHE_DENOMINATOR: u64 = 10;
/// Knots LARGE headroom: `total - 10 MiB`.
pub const LARGE_CACHE_HEADROOM_BYTES: u64 = 10 * 1024 * 1024;
/// Knots coins-write disk guard: `48 * 2 * 2` bytes per cache entry.
pub const COIN_WRITE_GUARD_BYTES_PER_ENTRY: u64 = 48 * 2 * 2;

/// Injected Unix-seconds clock fact. Adapters sample time; this crate does not.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct FlushPolicyTime(u64);

impl FlushPolicyTime {
    /// Creates a flush-policy timestamp from Unix seconds supplied by an adapter.
    pub const fn new(unix_seconds: u64) -> Self {
        Self(unix_seconds)
    }

    /// Creates a flush-policy timestamp from explicit Unix seconds.
    pub const fn from_unix_seconds(unix_seconds: u64) -> Self {
        Self::new(unix_seconds)
    }

    /// Returns the Unix-seconds representation.
    pub const fn unix_seconds(self) -> u64 {
        self.0
    }
}

/// Knots `FlushStateMode` values the later manager will request.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlushMode {
    None,
    IfNeeded,
    Periodic,
    Always,
}

/// Knots `CoinsCacheSizeState` occupancy classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum CoinsCacheSizeState {
    Ok = 0,
    Large = 1,
    Critical = 2,
}

/// Last flush-policy reason carried on every decision for later operator evidence.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LastFlushReason {
    None,
    Needed,
    Periodic,
    Always,
    FailedDisk,
}

/// Shared cache-size and reason payload on every `FlushDecision` variant.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FlushDecisionFacts {
    pub cache_size: CoinsCacheSizeState,
    pub reason: LastFlushReason,
}

/// Exclusive flush-policy outcome. A write and a disk refusal cannot coexist.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlushDecision {
    None(FlushDecisionFacts),
    Flush(FlushDecisionFacts),
    Sync(FlushDecisionFacts),
    RefuseDiskSpace(FlushDecisionFacts),
}

impl FlushDecision {
    /// Classified cache occupancy on this decision.
    pub const fn cache_size(self) -> CoinsCacheSizeState {
        self.facts().cache_size
    }

    /// Last-flush reason on this decision.
    pub const fn reason(self) -> LastFlushReason {
        self.facts().reason
    }

    const fn facts(self) -> FlushDecisionFacts {
        match self {
            Self::None(facts)
            | Self::Flush(facts)
            | Self::Sync(facts)
            | Self::RefuseDiskSpace(facts) => facts,
        }
    }
}

/// Injected occupancy, time, pressure, and disk facts for `decide_flush`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FlushPolicyInput {
    pub mode: FlushMode,
    pub cache_bytes: u64,
    pub cache_byte_limit: u64,
    pub mempool_leftover_bytes: u64,
    pub now: FlushPolicyTime,
    pub next_write: FlushPolicyTime,
    pub memory_pressure: bool,
    pub disk_free_bytes: u64,
    pub cache_entry_count: u64,
}

impl FlushPolicyInput {
    /// Knots periodic-due predicate: `now >= next_write`.
    pub const fn periodic_due(&self) -> bool {
        self.now.unix_seconds() >= self.next_write.unix_seconds()
    }
}

/// Knots `GetCoinsCacheSizeState`: CRITICAL over total, LARGE over `max(90%, total - 10 MiB)`.
pub(crate) fn classify_cache_size(
    cache_bytes: u64,
    cache_byte_limit: u64,
    mempool_leftover_bytes: u64,
) -> CoinsCacheSizeState {
    let total = cache_byte_limit.saturating_add(mempool_leftover_bytes);
    if cache_bytes > total {
        return CoinsCacheSizeState::Critical;
    }

    let ninety = u64::try_from(
        u128::from(total) * u128::from(LARGE_CACHE_NUMERATOR) / u128::from(LARGE_CACHE_DENOMINATOR),
    )
    .unwrap_or(u64::MAX);
    let headroom = total.saturating_sub(LARGE_CACHE_HEADROOM_BYTES);
    if cache_bytes > ninety.max(headroom) {
        return CoinsCacheSizeState::Large;
    }

    CoinsCacheSizeState::Ok
}

enum FlushWriteKind {
    Flush,
    Sync,
}

fn maybe_intended_write(
    mode: FlushMode,
    cache_size: CoinsCacheSizeState,
    periodic_due: bool,
    memory_pressure: bool,
) -> Option<(FlushWriteKind, LastFlushReason)> {
    match mode {
        FlushMode::None => None,
        FlushMode::Always => Some((FlushWriteKind::Flush, LastFlushReason::Always)),
        FlushMode::Periodic => {
            if cache_size >= CoinsCacheSizeState::Large {
                Some((FlushWriteKind::Flush, LastFlushReason::Periodic))
            } else if periodic_due {
                Some((FlushWriteKind::Sync, LastFlushReason::Periodic))
            } else {
                None
            }
        }
        FlushMode::IfNeeded => {
            if cache_size == CoinsCacheSizeState::Critical || memory_pressure {
                Some((FlushWriteKind::Flush, LastFlushReason::Needed))
            } else {
                None
            }
        }
    }
}

fn disk_guard_fails(disk_free_bytes: u64, cache_entry_count: u64) -> bool {
    let required = COIN_WRITE_GUARD_BYTES_PER_ENTRY.saturating_mul(cache_entry_count);
    disk_free_bytes < required
}

/// Knots `FlushStateToDisk` boolean split with first-class disk-space refusal.
pub fn decide_flush(input: FlushPolicyInput) -> FlushDecision {
    let cache_size = classify_cache_size(
        input.cache_bytes,
        input.cache_byte_limit,
        input.mempool_leftover_bytes,
    );
    let Some((kind, reason)) = maybe_intended_write(
        input.mode,
        cache_size,
        input.periodic_due(),
        input.memory_pressure,
    ) else {
        return FlushDecision::None(FlushDecisionFacts {
            cache_size,
            reason: LastFlushReason::None,
        });
    };
    if disk_guard_fails(input.disk_free_bytes, input.cache_entry_count) {
        return FlushDecision::RefuseDiskSpace(FlushDecisionFacts {
            cache_size,
            reason: LastFlushReason::FailedDisk,
        });
    }
    match kind {
        FlushWriteKind::Flush => FlushDecision::Flush(FlushDecisionFacts { cache_size, reason }),
        FlushWriteKind::Sync => FlushDecision::Sync(FlushDecisionFacts { cache_size, reason }),
    }
}
