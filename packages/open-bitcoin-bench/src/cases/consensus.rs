// Parity breadcrumbs:
// - packages/bitcoin-knots/src/bench/verify_script.cpp

use open_bitcoin_consensus::{count_legacy_sigops, verify_script};

use crate::{
    error::BenchError,
    fixtures::BenchFixtures,
    registry::{
        BenchCase, BenchDurability, BenchGroupId, BenchMeasurement, CONSENSUS_SCRIPT_MAPPING,
    },
};

const CASE_ID: &str = "consensus-script.legacy-script-validation";

pub const CASES: [BenchCase; 1] = [BenchCase {
    id: CASE_ID,
    group: BenchGroupId::ConsensusScript,
    description: "Counts legacy sigops and verifies a deterministic standard script pair.",
    measurement: BenchMeasurement {
        focus: "script_validation",
        fixture: "shared_static_fixtures",
        durability: BenchDurability::Pure,
    },
    knots_mapping: &CONSENSUS_SCRIPT_MAPPING,
    run_once,
}];

fn run_once() -> Result<(), BenchError> {
    let fixtures = BenchFixtures::shared()?;
    let sigops = count_legacy_sigops(&fixtures.consensus.sigops_script)
        .map_err(|error| BenchError::case_failed(CASE_ID, error.to_string()))?;
    if sigops == 0 {
        return Err(BenchError::case_failed(
            CASE_ID,
            "legacy sigops fixture produced zero sigops",
        ));
    }

    verify_script(
        &fixtures.consensus.script_sig,
        &fixtures.consensus.script_pubkey,
    )
    .map_err(|error| BenchError::case_failed(CASE_ID, error.to_string()))
}
