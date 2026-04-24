use open_bitcoin_mempool::{
    Mempool, transaction_sigops_cost, transaction_weight_and_virtual_size,
    validate_standard_transaction,
};

use crate::{
    error::BenchError,
    fixtures::{BenchFixtures, consensus_params, verify_flags},
    registry::{BenchCase, BenchGroupId, MEMPOOL_POLICY_MAPPING},
};

const CASE_ID: &str = "mempool-policy.standard-admission";

pub const CASES: [BenchCase; 1] = [BenchCase {
    id: CASE_ID,
    group: BenchGroupId::MempoolPolicy,
    description: "Runs standard policy accounting and accepts a deterministic confirmed spend.",
    knots_mapping: &MEMPOOL_POLICY_MAPPING,
    run_once,
}];

fn run_once() -> Result<(), BenchError> {
    let fixtures = BenchFixtures::shared()?;
    let transaction = fixtures.mempool.standard_spend.clone();
    let (weight, virtual_size) = transaction_weight_and_virtual_size(&transaction);
    let sigops_cost = transaction_sigops_cost(&transaction, &fixtures.mempool.input_contexts)
        .map_err(|error| BenchError::case_failed(CASE_ID, error.to_string()))?;
    let mut mempool = Mempool::default();

    validate_standard_transaction(
        &transaction,
        &fixtures.mempool.input_contexts,
        mempool.config(),
        weight,
        sigops_cost,
    )
    .map_err(|error| BenchError::case_failed(CASE_ID, error.to_string()))?;
    let result = mempool
        .accept_transaction(
            transaction,
            &fixtures.mempool.snapshot,
            verify_flags(),
            consensus_params(),
        )
        .map_err(|error| BenchError::case_failed(CASE_ID, error.to_string()))?;

    if mempool.entry(&result.accepted).is_none() {
        return Err(BenchError::case_failed(
            CASE_ID,
            "accepted transaction was not stored in the mempool",
        ));
    }
    if mempool.total_virtual_size() != virtual_size {
        return Err(BenchError::case_failed(
            CASE_ID,
            "mempool virtual size did not match policy calculation",
        ));
    }

    Ok(())
}
