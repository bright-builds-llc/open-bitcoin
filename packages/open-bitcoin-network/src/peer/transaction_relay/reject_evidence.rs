// Parity breadcrumbs:
// - packages/bitcoin-knots/src/common/bloom.h
// - packages/bitcoin-knots/src/common/bloom.cpp
// - packages/bitcoin-knots/src/node/txdownloadman_impl.h
// - packages/bitcoin-knots/src/node/txdownloadman_impl.cpp

//! Fixed-memory hard-reject and package-reconsideration evidence.

use open_bitcoin_primitives::Wtxid;

/// Number of recent insertions retained without false negatives.
pub const PHASE133_REJECT_FILTER_CAPACITY: usize = 120_000;
/// Target false-positive rate for each reject-evidence filter.
pub const PHASE133_REJECT_FILTER_FALSE_POSITIVE_RATE: f64 = 0.000_001;

const MAX_PROBE_COUNT: usize = 50;
const WORD_BITS: usize = u64::BITS as usize;
const GENERATION_COUNT: usize = 3;
const PHASE133_ENTRIES_PER_GENERATION: usize = 60_000;
const PHASE133_REJECT_FILTER_PROBE_COUNT: usize = 20;
const PHASE133_REJECT_FILTER_WORD_COUNT: usize = 161_750;

/// Injected entropy for one reject-evidence filter generation sequence.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RejectEvidenceTweak(u64);

impl RejectEvidenceTweak {
    /// Creates explicit deterministic or shell-derived tweak material.
    pub const fn new(value: u64) -> Self {
        Self(value)
    }
}

/// Invalid rolling-filter configuration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RejectEvidenceConfigError {
    /// A rolling filter cannot retain zero entries.
    ZeroCapacity,
    /// The false-positive rate must be finite and strictly between zero and one.
    InvalidFalsePositiveRate,
    /// Capacity arithmetic exceeded the platform representation.
    ArithmeticOverflow,
    /// The derived bit allocation exceeded the Knots-compatible 32-bit range.
    FilterTooLarge,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct RollingGenerationFilter {
    entries_per_generation: usize,
    entries_this_generation: usize,
    generation: u8,
    words: Vec<u64>,
    tweak: RejectEvidenceTweak,
    probe_count: usize,
}

impl RollingGenerationFilter {
    fn new_locked(tweak: RejectEvidenceTweak) -> Self {
        Self {
            entries_per_generation: PHASE133_ENTRIES_PER_GENERATION,
            entries_this_generation: 0,
            generation: 1,
            words: vec![0; PHASE133_REJECT_FILTER_WORD_COUNT],
            tweak,
            probe_count: PHASE133_REJECT_FILTER_PROBE_COUNT,
        }
    }

    fn try_new(
        capacity: usize,
        false_positive_rate: f64,
        tweak: RejectEvidenceTweak,
    ) -> Result<Self, RejectEvidenceConfigError> {
        if capacity == 0 {
            return Err(RejectEvidenceConfigError::ZeroCapacity);
        }
        if !false_positive_rate.is_finite()
            || false_positive_rate <= 0.0
            || false_positive_rate >= 1.0
        {
            return Err(RejectEvidenceConfigError::InvalidFalsePositiveRate);
        }

        let log_false_positive_rate = false_positive_rate.ln();
        let probe_count = (log_false_positive_rate / 0.5_f64.ln())
            .round()
            .clamp(1.0, MAX_PROBE_COUNT as f64) as usize;
        let entries_per_generation = capacity
            .checked_add(1)
            .ok_or(RejectEvidenceConfigError::ArithmeticOverflow)?
            / 2;
        let max_elements = entries_per_generation
            .checked_mul(GENERATION_COUNT)
            .ok_or(RejectEvidenceConfigError::ArithmeticOverflow)?;
        let exponent = (log_false_positive_rate / probe_count as f64).exp();
        let denominator = (1.0 - exponent).ln();
        let filter_bits = (-(probe_count as f64) * max_elements as f64 / denominator).ceil();
        if !filter_bits.is_finite() || filter_bits <= 0.0 || filter_bits > f64::from(u32::MAX) {
            return Err(RejectEvidenceConfigError::FilterTooLarge);
        }

        let filter_bits = filter_bits as usize;
        let word_pair_count = filter_bits
            .checked_add(WORD_BITS - 1)
            .ok_or(RejectEvidenceConfigError::ArithmeticOverflow)?
            / WORD_BITS;
        let word_count = word_pair_count
            .checked_mul(2)
            .ok_or(RejectEvidenceConfigError::ArithmeticOverflow)?;

        Ok(Self {
            entries_per_generation,
            entries_this_generation: 0,
            generation: 1,
            words: vec![0; word_count],
            tweak,
            probe_count,
        })
    }

    fn record(&mut self, domain: u8, bytes: &[u8]) {
        if self.entries_this_generation == self.entries_per_generation {
            self.rotate_generation();
        }
        self.entries_this_generation += 1;

        let pair_count = self.words.len() / 2;
        for probe in 0..self.probe_count {
            let hash = hash_evidence(self.tweak, probe, domain, bytes);
            let bit = (hash as usize) & (WORD_BITS - 1);
            let pair = fast_range(hash, pair_count);
            let first = pair * 2;
            let second = first + 1;
            let bit_mask = 1_u64 << bit;
            self.words[first] =
                (self.words[first] & !bit_mask) | (u64::from(self.generation & 1) << bit);
            self.words[second] =
                (self.words[second] & !bit_mask) | (u64::from(self.generation >> 1) << bit);
        }
    }

    fn contains(&self, domain: u8, bytes: &[u8]) -> bool {
        let pair_count = self.words.len() / 2;
        for probe in 0..self.probe_count {
            let hash = hash_evidence(self.tweak, probe, domain, bytes);
            let bit = (hash as usize) & (WORD_BITS - 1);
            let pair = fast_range(hash, pair_count);
            let first = pair * 2;
            if ((self.words[first] | self.words[first + 1]) >> bit) & 1 == 0 {
                return false;
            }
        }
        true
    }

    fn reset(&mut self, tweak: RejectEvidenceTweak) {
        self.words.fill(0);
        self.entries_this_generation = 0;
        self.generation = 1;
        self.tweak = tweak;
    }

    fn rotate_generation(&mut self) {
        self.entries_this_generation = 0;
        self.generation += 1;
        if self.generation > GENERATION_COUNT as u8 {
            self.generation = 1;
        }

        let generation_mask_1 = 0_u64.wrapping_sub(u64::from(self.generation & 1));
        let generation_mask_2 = 0_u64.wrapping_sub(u64::from(self.generation >> 1));
        for pair in self.words.chunks_exact_mut(2) {
            let mask = (pair[0] ^ generation_mask_1) | (pair[1] ^ generation_mask_2);
            pair[0] &= mask;
            pair[1] &= mask;
        }
    }
}

fn fast_range(hash: u64, range: usize) -> usize {
    (((hash as u128) * (range as u128)) >> u64::BITS) as usize
}

fn hash_evidence(tweak: RejectEvidenceTweak, probe: usize, domain: u8, bytes: &[u8]) -> u64 {
    let mut hash = 0xcbf2_9ce4_8422_2325_u64;
    for byte in tweak.0.to_le_bytes() {
        hash = mix_byte(hash, byte);
    }
    for byte in (probe as u64).to_le_bytes() {
        hash = mix_byte(hash, byte);
    }
    hash = mix_byte(hash, domain);
    for &byte in bytes {
        hash = mix_byte(hash, byte);
    }
    avalanche(hash)
}

fn mix_byte(hash: u64, byte: u8) -> u64 {
    (hash ^ u64::from(byte)).wrapping_mul(0x0000_0100_0000_01b3)
}

fn avalanche(mut hash: u64) -> u64 {
    hash ^= hash >> 33;
    hash = hash.wrapping_mul(0xff51_afd7_ed55_8ccd);
    hash ^= hash >> 33;
    hash = hash.wrapping_mul(0xc4ce_b9fe_1a85_ec53);
    hash ^ (hash >> 33)
}

/// Node-global evidence that a transaction failed hard policy.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HardRejectEvidence {
    filter: RollingGenerationFilter,
}

impl HardRejectEvidence {
    /// Constructs the locked Phase 133 fixed-memory filter.
    pub fn new(tweak: RejectEvidenceTweak) -> Self {
        Self {
            filter: RollingGenerationFilter::new_locked(tweak),
        }
    }

    /// Constructs a filter from checked parameters.
    pub fn try_with_parameters(
        capacity: usize,
        false_positive_rate: f64,
        tweak: RejectEvidenceTweak,
    ) -> Result<Self, RejectEvidenceConfigError> {
        Ok(Self {
            filter: RollingGenerationFilter::try_new(capacity, false_positive_rate, tweak)?,
        })
    }

    /// Records one transaction wtxid as hard-rejected.
    pub fn record(&mut self, wtxid: Wtxid) {
        self.filter.record(1, wtxid.as_bytes());
    }

    /// Tests whether a transaction wtxid may be hard-rejected.
    pub fn contains(&self, wtxid: Wtxid) -> bool {
        self.filter.contains(1, wtxid.as_bytes())
    }

    /// Clears membership and starts again with injected tweak material.
    pub fn reset(&mut self, tweak: RejectEvidenceTweak) {
        self.filter.reset(tweak);
    }

    #[cfg(test)]
    pub(crate) fn debug_entries_per_generation(&self) -> usize {
        self.filter.entries_per_generation
    }

    #[cfg(test)]
    pub(crate) fn debug_probe_count(&self) -> usize {
        self.filter.probe_count
    }

    #[cfg(test)]
    pub(crate) fn debug_storage_len(&self) -> usize {
        self.filter.words.len()
    }

    #[cfg(test)]
    pub(crate) fn debug_storage_capacity(&self) -> usize {
        self.filter.words.capacity()
    }

    #[cfg(test)]
    pub(crate) fn debug_generation(&self) -> u8 {
        self.filter.generation
    }

    #[cfg(test)]
    pub(crate) fn debug_entries_this_generation(&self) -> usize {
        self.filter.entries_this_generation
    }

    #[cfg(test)]
    pub(crate) fn debug_tweak(&self) -> RejectEvidenceTweak {
        self.filter.tweak
    }

    #[cfg(test)]
    pub(crate) fn debug_storage_checksum(&self) -> u64 {
        self.filter
            .words
            .iter()
            .fold(0xcbf2_9ce4_8422_2325, |checksum, word| {
                word.to_le_bytes().into_iter().fold(checksum, mix_byte)
            })
    }
}

/// Typed key domain for package-reconsiderable evidence.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReconsiderableEvidenceKey {
    /// A transaction that may succeed with package context.
    Transaction(Wtxid),
    /// A previously attempted content-derived package fingerprint.
    Package([u8; 32]),
}

/// Node-global evidence for reconsiderable transactions and failed packages.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReconsiderableRejectEvidence {
    filter: RollingGenerationFilter,
}

impl ReconsiderableRejectEvidence {
    /// Constructs the locked Phase 133 fixed-memory filter.
    pub fn new(tweak: RejectEvidenceTweak) -> Self {
        Self {
            filter: RollingGenerationFilter::new_locked(tweak),
        }
    }

    /// Records typed transaction or package evidence.
    pub fn record(&mut self, key: ReconsiderableEvidenceKey) {
        match key {
            ReconsiderableEvidenceKey::Transaction(wtxid) => {
                self.filter.record(2, wtxid.as_bytes());
            }
            ReconsiderableEvidenceKey::Package(fingerprint) => {
                self.filter.record(3, &fingerprint);
            }
        }
    }

    /// Tests typed transaction or package evidence.
    pub fn contains(&self, key: ReconsiderableEvidenceKey) -> bool {
        match key {
            ReconsiderableEvidenceKey::Transaction(wtxid) => {
                self.filter.contains(2, wtxid.as_bytes())
            }
            ReconsiderableEvidenceKey::Package(fingerprint) => {
                self.filter.contains(3, &fingerprint)
            }
        }
    }

    /// Clears membership and starts again with injected tweak material.
    pub fn reset(&mut self, tweak: RejectEvidenceTweak) {
        self.filter.reset(tweak);
    }
}
