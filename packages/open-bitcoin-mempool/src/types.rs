// Parity breadcrumbs:
// - packages/bitcoin-knots/src/txmempool.h
// - packages/bitcoin-knots/src/txmempool.cpp
// - packages/bitcoin-knots/src/policy/policy.h
// - packages/bitcoin-knots/src/policy/rbf.cpp
// - packages/bitcoin-knots/src/policy/packages.cpp

use std::collections::BTreeSet;

use open_bitcoin_primitives::{Amount, Transaction, Txid, Wtxid};

const SATOSHIS_PER_KILOVBYTE: i64 = 1_000;
const FEE_RATE_ROUNDING_ADJUSTMENT: i64 = SATOSHIS_PER_KILOVBYTE - 1;

const DEFAULT_MIN_RELAY_FEERATE_SATS_PER_KVB: i64 = SATOSHIS_PER_KILOVBYTE;
const DEFAULT_INCREMENTAL_RELAY_FEERATE_SATS_PER_KVB: i64 = SATOSHIS_PER_KILOVBYTE;
const DEFAULT_MAX_STANDARD_TX_WEIGHT: usize = 400_000;
const DEFAULT_MAX_STANDARD_SIGOPS_COST: usize = 20_000;
const DEFAULT_MAX_SCRIPT_SIG_SIZE: usize = 1_650;
const DEFAULT_MAX_DATACARRIER_BYTES: usize = 83;
const DEFAULT_MAX_ANCESTOR_COUNT: usize = 25;
const DEFAULT_MAX_ANCESTOR_VIRTUAL_SIZE: usize = 101_000;
const DEFAULT_MAX_DESCENDANT_COUNT: usize = 25;
const DEFAULT_MAX_DESCENDANT_VIRTUAL_SIZE: usize = 101_000;
const DEFAULT_MAX_MEMPOOL_VIRTUAL_SIZE: usize = 300_000_000;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct FeeRate {
    sats_per_kvb: i64,
}

impl FeeRate {
    pub const ZERO: Self = Self { sats_per_kvb: 0 };

    pub const fn from_sats_per_kvb(sats_per_kvb: i64) -> Self {
        Self { sats_per_kvb }
    }

    pub fn from_fee_sats_and_vbytes(fee_sats: i64, virtual_size: usize) -> Self {
        if virtual_size == 0 {
            return Self::ZERO;
        }

        let virtual_size = i64::try_from(virtual_size).unwrap_or(i64::MAX);
        let sats_per_kvb =
            (fee_sats.saturating_mul(SATOSHIS_PER_KILOVBYTE) + virtual_size - 1) / virtual_size;
        Self { sats_per_kvb }
    }

    pub const fn sats_per_kvb(self) -> i64 {
        self.sats_per_kvb
    }

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
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{} sat/kvB", self.sats_per_kvb)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RbfPolicy {
    Never,
    OptIn,
    Always,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PolicyConfig {
    pub min_relay_feerate: FeeRate,
    pub incremental_relay_feerate: FeeRate,
    pub rbf_policy: RbfPolicy,
    pub max_standard_tx_weight: usize,
    pub max_standard_sigops_cost: usize,
    pub max_script_sig_size: usize,
    pub max_datacarrier_bytes: usize,
    pub accept_datacarrier: bool,
    pub permit_bare_multisig: bool,
    pub max_ancestor_count: usize,
    pub max_ancestor_virtual_size: usize,
    pub max_descendant_count: usize,
    pub max_descendant_virtual_size: usize,
    pub max_mempool_virtual_size: usize,
}

impl Default for PolicyConfig {
    fn default() -> Self {
        Self {
            min_relay_feerate: FeeRate::from_sats_per_kvb(DEFAULT_MIN_RELAY_FEERATE_SATS_PER_KVB),
            incremental_relay_feerate: FeeRate::from_sats_per_kvb(
                DEFAULT_INCREMENTAL_RELAY_FEERATE_SATS_PER_KVB,
            ),
            rbf_policy: RbfPolicy::Always,
            max_standard_tx_weight: DEFAULT_MAX_STANDARD_TX_WEIGHT,
            max_standard_sigops_cost: DEFAULT_MAX_STANDARD_SIGOPS_COST,
            max_script_sig_size: DEFAULT_MAX_SCRIPT_SIG_SIZE,
            max_datacarrier_bytes: DEFAULT_MAX_DATACARRIER_BYTES,
            accept_datacarrier: true,
            permit_bare_multisig: false,
            max_ancestor_count: DEFAULT_MAX_ANCESTOR_COUNT,
            max_ancestor_virtual_size: DEFAULT_MAX_ANCESTOR_VIRTUAL_SIZE,
            max_descendant_count: DEFAULT_MAX_DESCENDANT_COUNT,
            max_descendant_virtual_size: DEFAULT_MAX_DESCENDANT_VIRTUAL_SIZE,
            max_mempool_virtual_size: DEFAULT_MAX_MEMPOOL_VIRTUAL_SIZE,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AggregateStats {
    pub count: usize,
    pub virtual_size: usize,
    pub total_fee_sats: i64,
}

impl AggregateStats {
    pub const fn new(count: usize, virtual_size: usize, total_fee_sats: i64) -> Self {
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
    pub fee: Amount,
    pub virtual_size: usize,
    pub weight: usize,
    pub sigops_cost: usize,
    pub parents: BTreeSet<Txid>,
    pub children: BTreeSet<Txid>,
    pub ancestor_stats: AggregateStats,
    pub descendant_stats: AggregateStats,
}

impl MempoolEntry {
    pub fn new(
        transaction: Transaction,
        txid: Txid,
        wtxid: Wtxid,
        fee: Amount,
        virtual_size: usize,
        weight: usize,
        sigops_cost: usize,
    ) -> Self {
        let stats = AggregateStats::new(1, virtual_size, fee.to_sats());
        Self {
            transaction,
            txid,
            wtxid,
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
        let rate = FeeRate::from_fee_sats_and_vbytes(250, 125);

        assert_eq!(rate, FeeRate::from_sats_per_kvb(2000));
        assert_eq!(rate.fee_for_virtual_size(125), 250);
    }

    #[test]
    fn default_policy_matches_the_targeted_phase_defaults() {
        let config = PolicyConfig::default();

        assert_eq!(config.rbf_policy, RbfPolicy::Always);
        assert_eq!(config.min_relay_feerate, FeeRate::from_sats_per_kvb(1000));
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
            100,
            400,
            4,
        );

        assert_eq!(entry.ancestor_stats.count, 1);
        assert_eq!(entry.descendant_stats.virtual_size, 100);
        assert_eq!(entry.descendant_score(), FeeRate::from_sats_per_kvb(2000));
    }

    #[test]
    fn fee_rate_handles_zero_virtual_size_and_formats_cleanly() {
        let zero = FeeRate::from_fee_sats_and_vbytes(25, 0);

        assert_eq!(zero, FeeRate::ZERO);
        assert_eq!(zero.sats_per_kvb(), 0);
        assert_eq!(zero.fee_for_virtual_size(0), 0);
        assert_eq!(zero.to_string(), "0 sat/kvB");
    }

    #[test]
    fn descendant_score_prefers_the_descendant_package_rate_when_higher() {
        let fee = Amount::from_sats(100).expect("valid amount");
        let mut entry = MempoolEntry::new(
            sample_transaction(),
            Txid::from_byte_array([4_u8; 32]),
            Wtxid::from_byte_array([5_u8; 32]),
            fee,
            100,
            400,
            4,
        );
        entry.descendant_stats = AggregateStats::new(2, 150, 600);

        assert_eq!(
            entry.descendant_score(),
            FeeRate::from_fee_sats_and_vbytes(600, 150)
        );
    }
}
