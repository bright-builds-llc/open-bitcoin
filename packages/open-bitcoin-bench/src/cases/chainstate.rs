// Parity breadcrumbs:
// - packages/bitcoin-knots/src/bench/checkblock.cpp
// - packages/bitcoin-knots/src/bench/readwriteblock.cpp
// - packages/bitcoin-knots/src/bench/ccoins_caching.cpp
// - packages/bitcoin-knots/src/coins.cpp

use open_bitcoin_chainstate::{AnchoredBlock, Chainstate};

use crate::{
    error::BenchError,
    fixtures::{BenchFixtures, consensus_params, verify_flags},
    registry::{BenchCase, BenchDurability, BenchGroupId, BenchMeasurement, CHAINSTATE_MAPPING},
};

const CASE_ID: &str = "chainstate.connect-disconnect-reorg";

pub const CASES: [BenchCase; 1] = [BenchCase {
    id: CASE_ID,
    group: BenchGroupId::Chainstate,
    description: "Connects, disconnects, and reorgs cloned deterministic chainstate snapshots.",
    measurement: BenchMeasurement {
        focus: "chainstate_connect_disconnect_reorg",
        fixture: "shared_chainstate_snapshots",
        durability: BenchDurability::Pure,
    },
    knots_mapping: &CHAINSTATE_MAPPING,
    run_once,
}];

fn run_once() -> Result<(), BenchError> {
    let fixtures = BenchFixtures::shared()?;
    let mut connect_disconnect =
        Chainstate::from_snapshot(fixtures.chainstate.genesis_snapshot.clone());
    connect_disconnect
        .connect_block(
            &fixtures.chainstate.branch_a,
            2,
            verify_flags(),
            consensus_params(),
        )
        .map_err(|error| BenchError::case_failed(CASE_ID, error.to_string()))?;
    connect_disconnect
        .disconnect_tip(&fixtures.chainstate.branch_a)
        .map_err(|error| BenchError::case_failed(CASE_ID, error.to_string()))?;

    let mut reorg = Chainstate::from_snapshot(fixtures.chainstate.branch_a_snapshot.clone());
    reorg
        .reorg(
            std::slice::from_ref(&fixtures.chainstate.branch_a),
            &[AnchoredBlock {
                block: fixtures.chainstate.branch_b.clone(),
                chain_work: 3,
            }],
            verify_flags(),
            consensus_params(),
        )
        .map_err(|error| BenchError::case_failed(CASE_ID, error.to_string()))?;

    Ok(())
}
