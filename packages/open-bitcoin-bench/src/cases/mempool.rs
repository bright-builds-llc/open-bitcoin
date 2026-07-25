// Parity breadcrumbs:
// - packages/bitcoin-knots/src/bench/mempool_stress.cpp
// - packages/bitcoin-knots/src/bench/mempool_eviction.cpp
// - packages/bitcoin-knots/src/txmempool.cpp
// - packages/bitcoin-knots/src/policy/policy.cpp

use std::time::{Duration, Instant};

use open_bitcoin_mempool::{
    AdmissionContext, Mempool, MempoolCapacity, PolicyConfig, transaction_sigops_cost,
    transaction_weight_and_virtual_size, validate_standard_transaction,
};
use open_bitcoin_primitives::TransactionInput;

use crate::{
    error::BenchError,
    fixtures::{
        BenchFixtures, consensus_params, p2sh_script, sample_chainstate_snapshot, script,
        spend_transaction, verify_flags,
    },
    registry::{
        BenchCase, BenchDurability, BenchGroupId, BenchMeasurement, MEMPOOL_POLICY_MAPPING,
    },
};

const STANDARD_ADMISSION_CASE_ID: &str = "mempool-policy.standard-admission";
const SUSTAINED_PRESSURE_CASE_ID: &str = "mempool-policy.sustained-pressure-trim";

/// Number of admit+trim cycles in the hermetic PRESS-05 pressure loop.
///
/// Chosen large enough that a quadratic clone/recompute path is obvious, and
/// small enough that the loop stays a default-verifier smoke check.
const SUSTAINED_PRESSURE_TRIM_CYCLES: usize = 24;

/// Maximum wall time for one sustained-pressure case invocation.
///
/// Measured locally under debug/test profile (~tens of ms for 24 cycles on Apple
/// Silicon). The 2s budget is loose enough for CI noise but fails on unbounded
/// blowups. Pure durability — no network, no sleep.
const SUSTAINED_PRESSURE_MAX_ELAPSED: Duration = Duration::from_millis(2_000);

pub const CASES: [BenchCase; 2] = [
    BenchCase {
        id: STANDARD_ADMISSION_CASE_ID,
        group: BenchGroupId::MempoolPolicy,
        description: "Runs standard policy accounting and accepts a deterministic confirmed spend.",
        measurement: BenchMeasurement {
            focus: "mempool_policy_admission",
            fixture: "shared_mempool_snapshots",
            durability: BenchDurability::Pure,
        },
        knots_mapping: &MEMPOOL_POLICY_MAPPING,
        run_once: run_standard_admission,
    },
    BenchCase {
        id: SUSTAINED_PRESSURE_CASE_ID,
        group: BenchGroupId::MempoolPolicy,
        description: "Hermetic accounted-capacity fill/trim loop with a wall-time blowup threshold.",
        measurement: BenchMeasurement {
            focus: "mempool_pressure_trim",
            fixture: "sustained_pressure_coinbase_chain",
            durability: BenchDurability::Pure,
        },
        knots_mapping: &MEMPOOL_POLICY_MAPPING,
        run_once: run_sustained_pressure_trim,
    },
];

fn run_standard_admission() -> Result<(), BenchError> {
    let fixtures = BenchFixtures::shared()?;
    let transaction = fixtures.mempool.standard_spend.clone();
    let (weight, virtual_size) = transaction_weight_and_virtual_size(&transaction)
        .map_err(|error| BenchError::case_failed(STANDARD_ADMISSION_CASE_ID, error.to_string()))?;
    let sigops_cost = transaction_sigops_cost(&transaction, &fixtures.mempool.input_contexts)
        .map_err(|error| BenchError::case_failed(STANDARD_ADMISSION_CASE_ID, error.to_string()))?;
    let mut mempool = Mempool::default();

    validate_standard_transaction(
        &transaction,
        &fixtures.mempool.input_contexts,
        mempool.config(),
        weight,
        sigops_cost,
    )
    .map_err(|error| BenchError::case_failed(STANDARD_ADMISSION_CASE_ID, error.to_string()))?;
    let result = mempool
        .accept_transaction_with_context(
            transaction,
            &fixtures.mempool.snapshot,
            verify_flags(),
            consensus_params(),
            AdmissionContext::legacy_unknown(),
        )
        .map_err(|error| BenchError::case_failed(STANDARD_ADMISSION_CASE_ID, error.to_string()))?;

    if mempool.entry(&result.accepted).is_none() {
        return Err(BenchError::case_failed(
            STANDARD_ADMISSION_CASE_ID,
            "accepted transaction was not stored in the mempool",
        ));
    }
    if mempool.total_virtual_size().as_usize() != virtual_size {
        return Err(BenchError::case_failed(
            STANDARD_ADMISSION_CASE_ID,
            "mempool virtual size did not match policy calculation",
        ));
    }

    Ok(())
}

fn run_sustained_pressure_trim() -> Result<(), BenchError> {
    let output_script = p2sh_script()?;
    let script_sig = script(&[0x01, 0x51])?;
    // One probe coinbase + one cycle coinbase per trim iteration.
    let block_count = (SUSTAINED_PRESSURE_TRIM_CYCLES + 1) as u32;
    let (snapshot, coinbase_txids) =
        sample_chainstate_snapshot(block_count, output_script.clone())?;
    let Some(probe_txid) = coinbase_txids.first().copied() else {
        return Err(BenchError::case_failed(
            SUSTAINED_PRESSURE_CASE_ID,
            "pressure fixture produced no coinbase txids",
        ));
    };

    let mut probe = Mempool::default();
    let probe_spend = spend_transaction(
        probe_txid,
        0,
        499_999_000,
        output_script.clone(),
        script_sig.clone(),
        TransactionInput::SEQUENCE_FINAL,
    )?;
    probe
        .accept_transaction_with_context(
            probe_spend,
            &snapshot,
            verify_flags(),
            consensus_params(),
            AdmissionContext::legacy_unknown(),
        )
        .map_err(|error| BenchError::case_failed(SUSTAINED_PRESSURE_CASE_ID, error.to_string()))?;
    let one_entry_capacity = probe.accounted_memory().as_usize();

    let mut mempool = Mempool::new(PolicyConfig {
        mempool_capacity: MempoolCapacity::new(one_entry_capacity),
        ..PolicyConfig::default()
    });

    let started = Instant::now();
    for (index, coinbase_txid) in coinbase_txids.iter().skip(1).enumerate() {
        // Lower output value => higher fee so the newest admission survives trim.
        let output_value = 499_999_000 - (index as i64) * 1_000;
        let spend = spend_transaction(
            *coinbase_txid,
            0,
            output_value,
            output_script.clone(),
            script_sig.clone(),
            TransactionInput::SEQUENCE_FINAL,
        )?;
        mempool
            .accept_transaction_with_context(
                spend,
                &snapshot,
                verify_flags(),
                consensus_params(),
                AdmissionContext::legacy_unknown(),
            )
            .map_err(|error| {
                BenchError::case_failed(SUSTAINED_PRESSURE_CASE_ID, error.to_string())
            })?;
        if mempool.accounted_memory().as_usize() > mempool.config().mempool_capacity.as_usize() {
            return Err(BenchError::case_failed(
                SUSTAINED_PRESSURE_CASE_ID,
                "accounted trim left mempool over MempoolCapacity",
            ));
        }
    }
    let elapsed = started.elapsed();

    if mempool.entries().len() != 1 {
        return Err(BenchError::case_failed(
            SUSTAINED_PRESSURE_CASE_ID,
            format!(
                "expected one retained entry after sustained trim, got {}",
                mempool.entries().len()
            ),
        ));
    }
    if mempool.rolling_mempool_fee_rate().fee_rate().sats_per_kvb() == 0 {
        return Err(BenchError::case_failed(
            SUSTAINED_PRESSURE_CASE_ID,
            "expected rolling fee bump after pressure package removals",
        ));
    }
    if elapsed > SUSTAINED_PRESSURE_MAX_ELAPSED {
        return Err(BenchError::case_failed(
            SUSTAINED_PRESSURE_CASE_ID,
            format!(
                "sustained-pressure trim exceeded hermetic threshold: {:?} > {:?} (N={})",
                elapsed, SUSTAINED_PRESSURE_MAX_ELAPSED, SUSTAINED_PRESSURE_TRIM_CYCLES
            ),
        ));
    }

    Ok(())
}
