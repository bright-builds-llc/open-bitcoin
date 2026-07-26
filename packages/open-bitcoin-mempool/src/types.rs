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

use std::collections::BTreeSet;

use open_bitcoin_primitives::{Amount, Transaction, Txid, Wtxid};

use crate::context::MempoolEntryMetadata;
use crate::fee::{DustRelayFeeRate, FeeRate, IncrementalRelayFeeRate, StaticRelayFeeRate};
use crate::resource::{MempoolCapacity, TransactionVirtualSize};

const DEFAULT_STATIC_RELAY_FEE_RATE_SATS_PER_KVB: i64 = 1_000;
const DEFAULT_INCREMENTAL_RELAY_FEE_RATE_SATS_PER_KVB: i64 = 1_000;
const DEFAULT_MAX_STANDARD_TX_WEIGHT: usize = 400_000;
const DEFAULT_MAX_STANDARD_SIGOPS_COST: usize = 20_000;
const DEFAULT_MAX_SCRIPT_SIG_SIZE: usize = 1_650;
const DEFAULT_MAX_DATACARRIER_BYTES: usize = 83;
const DEFAULT_MAX_ANCESTOR_COUNT: usize = 25;
const DEFAULT_MAX_ANCESTOR_VIRTUAL_SIZE: usize = 101_000;
const DEFAULT_MAX_DESCENDANT_COUNT: usize = 25;
const DEFAULT_MAX_DESCENDANT_VIRTUAL_SIZE: usize = 101_000;
const DEFAULT_MEMPOOL_CAPACITY: usize = 300_000_000;
/// Knots `DEFAULT_MEMPOOL_EXPIRY_HOURS` — entries older than this leave via Expire.
pub const DEFAULT_MEMPOOL_EXPIRY_HOURS: u64 = 336;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RbfPolicy {
    Never,
    OptIn,
    Always,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrucPolicy {
    Reject,
    Accept,
    Enforce,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EphemeralPolicy {
    pub anchor: bool,
    pub send: bool,
    pub dust: bool,
}

impl Default for EphemeralPolicy {
    fn default() -> Self {
        Self {
            anchor: true,
            send: false,
            dust: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PolicyConfig {
    pub static_relay_fee_rate: StaticRelayFeeRate,
    pub dust_relay_fee_rate: DustRelayFeeRate,
    pub incremental_relay_fee_rate: IncrementalRelayFeeRate,
    pub rbf_policy: RbfPolicy,
    pub truc_policy: TrucPolicy,
    pub ephemeral_policy: EphemeralPolicy,
    pub max_standard_tx_weight: usize,
    pub max_standard_sigops_cost: usize,
    pub max_script_sig_size: usize,
    pub max_datacarrier_bytes: usize,
    pub accept_datacarrier: bool,
    pub permit_bare_datacarrier: bool,
    pub permit_bare_anchor: bool,
    pub permit_bare_multisig: bool,
    pub max_ancestor_count: usize,
    pub max_ancestor_virtual_size: usize,
    pub max_descendant_count: usize,
    pub max_descendant_virtual_size: usize,
    pub mempool_capacity: MempoolCapacity,
    /// Max age in hours for `Known` acceptance times before expiry (Knots default 336).
    pub mempool_expiry_hours: u64,
}

impl Default for PolicyConfig {
    fn default() -> Self {
        Self {
            static_relay_fee_rate: StaticRelayFeeRate::new(FeeRate::from_sats_per_kvb(
                DEFAULT_STATIC_RELAY_FEE_RATE_SATS_PER_KVB,
            )),
            dust_relay_fee_rate: DustRelayFeeRate::default(),
            incremental_relay_fee_rate: IncrementalRelayFeeRate::new(FeeRate::from_sats_per_kvb(
                DEFAULT_INCREMENTAL_RELAY_FEE_RATE_SATS_PER_KVB,
            )),
            rbf_policy: RbfPolicy::Always,
            truc_policy: TrucPolicy::Accept,
            ephemeral_policy: EphemeralPolicy::default(),
            max_standard_tx_weight: DEFAULT_MAX_STANDARD_TX_WEIGHT,
            max_standard_sigops_cost: DEFAULT_MAX_STANDARD_SIGOPS_COST,
            max_script_sig_size: DEFAULT_MAX_SCRIPT_SIG_SIZE,
            max_datacarrier_bytes: DEFAULT_MAX_DATACARRIER_BYTES,
            accept_datacarrier: true,
            permit_bare_datacarrier: false,
            permit_bare_anchor: false,
            permit_bare_multisig: false,
            max_ancestor_count: DEFAULT_MAX_ANCESTOR_COUNT,
            max_ancestor_virtual_size: DEFAULT_MAX_ANCESTOR_VIRTUAL_SIZE,
            max_descendant_count: DEFAULT_MAX_DESCENDANT_COUNT,
            max_descendant_virtual_size: DEFAULT_MAX_DESCENDANT_VIRTUAL_SIZE,
            mempool_capacity: MempoolCapacity::new(DEFAULT_MEMPOOL_CAPACITY),
            mempool_expiry_hours: DEFAULT_MEMPOOL_EXPIRY_HOURS,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AggregateStats {
    pub count: usize,
    pub virtual_size: TransactionVirtualSize,
    pub total_fee_sats: i64,
}

impl AggregateStats {
    pub const fn new(
        count: usize,
        virtual_size: TransactionVirtualSize,
        total_fee_sats: i64,
    ) -> Self {
        Self {
            count,
            virtual_size,
            total_fee_sats,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MempoolEntry {
    pub transaction: Transaction,
    pub txid: Txid,
    pub wtxid: Wtxid,
    pub metadata: MempoolEntryMetadata,
    pub fee: Amount,
    pub virtual_size: TransactionVirtualSize,
    pub weight: usize,
    pub sigops_cost: usize,
    pub parents: BTreeSet<Txid>,
    pub children: BTreeSet<Txid>,
    pub ancestor_stats: AggregateStats,
    pub descendant_stats: AggregateStats,
}

impl MempoolEntry {
    /// Creates a canonical entry from validated transaction and admission facts.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        transaction: Transaction,
        txid: Txid,
        wtxid: Wtxid,
        fee: Amount,
        virtual_size: TransactionVirtualSize,
        weight: usize,
        sigops_cost: usize,
        metadata: MempoolEntryMetadata,
    ) -> Self {
        let stats = AggregateStats::new(1, virtual_size, fee.to_sats());
        Self {
            transaction,
            txid,
            wtxid,
            metadata,
            fee,
            virtual_size,
            weight,
            sigops_cost,
            parents: BTreeSet::new(),
            children: BTreeSet::new(),
            ancestor_stats: stats,
            descendant_stats: stats,
        }
    }

    pub fn fee_sats(&self) -> i64 {
        self.fee.to_sats()
    }

    pub fn fee_rate(&self) -> FeeRate {
        FeeRate::from_fee_sats_and_vbytes(self.fee_sats(), self.virtual_size)
    }

    pub fn descendant_score(&self) -> FeeRate {
        let self_rate = self.fee_rate();
        let descendant_rate = FeeRate::from_fee_sats_and_vbytes(
            self.descendant_stats.total_fee_sats,
            self.descendant_stats.virtual_size,
        );
        if descendant_rate > self_rate {
            descendant_rate
        } else {
            self_rate
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdmissionResult {
    pub accepted: Txid,
    pub replaced: Vec<Txid>,
    pub evicted: Vec<Txid>,
}

#[cfg(test)]
mod tests {
    use open_bitcoin_primitives::{
        Amount, OutPoint, ScriptBuf, ScriptWitness, Transaction, TransactionInput,
        TransactionOutput, Txid, Wtxid,
    };

    use super::{AggregateStats, FeeRate, MempoolEntry, PolicyConfig, RbfPolicy};
    use crate::{StaticRelayFeeRate, TransactionVirtualSize};

    fn sample_transaction() -> Transaction {
        Transaction {
            version: 2,
            inputs: vec![TransactionInput {
                previous_output: OutPoint {
                    txid: Txid::from_byte_array([1_u8; 32]),
                    vout: 0,
                },
                script_sig: ScriptBuf::from_bytes(vec![0x01, 0x51]).expect("valid script"),
                sequence: TransactionInput::SEQUENCE_FINAL,
                witness: ScriptWitness::default(),
            }],
            outputs: vec![TransactionOutput {
                value: Amount::from_sats(1000).expect("valid amount"),
                script_pubkey: ScriptBuf::from_bytes(vec![0x51]).expect("valid script"),
            }],
            lock_time: 0,
        }
    }

    #[test]
    fn fee_rate_round_trips_expected_values() {
        let rate = FeeRate::from_fee_sats_and_vbytes(250, TransactionVirtualSize::new(125));

        assert_eq!(rate, FeeRate::from_sats_per_kvb(2000));
        assert_eq!(
            rate.fee_for_virtual_size(TransactionVirtualSize::new(125)),
            250
        );
    }

    #[test]
    fn default_policy_matches_the_targeted_phase_defaults() {
        let config = PolicyConfig::default();

        assert_eq!(config.rbf_policy, RbfPolicy::Always);
        assert_eq!(
            config.static_relay_fee_rate,
            StaticRelayFeeRate::new(FeeRate::from_sats_per_kvb(1000))
        );
        assert_eq!(config.max_ancestor_count, 25);
        assert_eq!(config.max_descendant_virtual_size, 101_000);
    }

    #[test]
    fn mempool_entry_starts_with_self_only_metrics() {
        let fee = Amount::from_sats(200).expect("valid amount");
        let entry = MempoolEntry::new(
            sample_transaction(),
            Txid::from_byte_array([2_u8; 32]),
            Wtxid::from_byte_array([3_u8; 32]),
            fee,
            TransactionVirtualSize::new(100),
            400,
            4,
            crate::MempoolEntryMetadata::legacy_unknown(),
        );

        assert_eq!(entry.ancestor_stats.count, 1);
        assert_eq!(
            entry.descendant_stats.virtual_size,
            TransactionVirtualSize::new(100)
        );
        assert_eq!(entry.descendant_score(), FeeRate::from_sats_per_kvb(2000));
    }

    #[test]
    fn fee_rate_handles_zero_virtual_size_and_formats_cleanly() {
        let zero = FeeRate::from_fee_sats_and_vbytes(25, TransactionVirtualSize::ZERO);

        assert_eq!(zero, FeeRate::ZERO);
        assert_eq!(zero.sats_per_kvb(), 0);
        assert_eq!(zero.fee_for_virtual_size(TransactionVirtualSize::ZERO), 0);
        assert_eq!(zero.to_string(), "0 sat/kvB");
    }

    #[test]
    fn fee_rate_arithmetic_clamps_extremes_without_panicking_or_wrapping() {
        // Arrange
        let maximum_size = TransactionVirtualSize::new(usize::MAX);
        let maximum_rate = FeeRate::from_sats_per_kvb(i64::MAX);
        let minimum_rate = FeeRate::from_sats_per_kvb(i64::MIN);

        // Act
        let maximum_fee = maximum_rate.fee_for_virtual_size(maximum_size);
        let minimum_fee = minimum_rate.fee_for_virtual_size(maximum_size);
        let maximum_derived =
            FeeRate::from_fee_sats_and_vbytes(i64::MAX, TransactionVirtualSize::new(1));
        let minimum_derived =
            FeeRate::from_fee_sats_and_vbytes(i64::MIN, TransactionVirtualSize::new(1));

        // Assert
        assert_eq!(maximum_fee, i64::MAX);
        assert_eq!(minimum_fee, i64::MIN);
        assert_eq!(maximum_derived.sats_per_kvb(), i64::MAX);
        assert_eq!(minimum_derived.sats_per_kvb(), i64::MIN);
    }

    #[test]
    fn fee_rate_arithmetic_preserves_signed_minimum_and_zero_size_rules() {
        // Arrange
        let negative_rate = FeeRate::from_sats_per_kvb(-1);
        let zero_rate = FeeRate::ZERO;
        let one_vbyte = TransactionVirtualSize::new(1);
        let maximum_size = TransactionVirtualSize::new(usize::MAX);

        // Act / Assert
        assert_eq!(negative_rate.fee_for_virtual_size(one_vbyte), -1);
        assert_eq!(zero_rate.fee_for_virtual_size(maximum_size), 0);
        assert_eq!(
            FeeRate::from_sats_per_kvb(i64::MAX).fee_for_virtual_size(TransactionVirtualSize::ZERO),
            0
        );
    }

    #[test]
    fn descendant_score_prefers_the_descendant_package_rate_when_higher() {
        let fee = Amount::from_sats(100).expect("valid amount");
        let mut entry = MempoolEntry::new(
            sample_transaction(),
            Txid::from_byte_array([4_u8; 32]),
            Wtxid::from_byte_array([5_u8; 32]),
            fee,
            TransactionVirtualSize::new(100),
            400,
            4,
            crate::MempoolEntryMetadata::legacy_unknown(),
        );
        entry.descendant_stats = AggregateStats::new(2, TransactionVirtualSize::new(150), 600);

        assert_eq!(
            entry.descendant_score(),
            FeeRate::from_fee_sats_and_vbytes(600, TransactionVirtualSize::new(150))
        );
    }
}
