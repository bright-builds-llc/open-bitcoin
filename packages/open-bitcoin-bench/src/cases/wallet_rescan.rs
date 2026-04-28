// Parity breadcrumbs:
// - packages/bitcoin-knots/src/wallet/wallet.cpp
// - packages/bitcoin-knots/src/wallet/spend.cpp
// - packages/bitcoin-knots/doc/managing-wallets.md

use open_bitcoin_node::{
    FjallNodeStore, PersistMode, WalletRegistry, WalletRescanFreshness, WalletRescanJobState,
    WalletRescanRuntime,
};

use crate::{
    error::BenchError,
    registry::{BenchCase, BenchDurability, BenchGroupId, BenchMeasurement, WALLET_RESCAN_MAPPING},
    runtime_fixtures::{TempStoreDir, funded_chainstate, wallet_with_ranged_descriptor},
};

const CASE_ID: &str = "wallet-rescan.runtime-rescan";

pub const CASES: [BenchCase; 1] = [BenchCase {
    id: CASE_ID,
    group: BenchGroupId::WalletRescan,
    description: "Runs the durable wallet rescan runtime against a funded chainstate snapshot.",
    measurement: BenchMeasurement {
        focus: "wallet_rescan",
        fixture: "wallet_registry_and_chainstate_snapshot",
        durability: BenchDurability::Durable,
    },
    knots_mapping: &WALLET_RESCAN_MAPPING,
    run_once: run_wallet_rescan_case,
}];

fn run_wallet_rescan_case() -> Result<(), BenchError> {
    let temp_dir = TempStoreDir::new("wallet-rescan-runtime")?;
    let store = FjallNodeStore::open(temp_dir.path())
        .map_err(|error| BenchError::case_failed(CASE_ID, error.to_string()))?;
    let wallet = wallet_with_ranged_descriptor()?;
    store
        .save_chainstate_snapshot(&funded_chainstate(&wallet)?, PersistMode::Sync)
        .map_err(|error| BenchError::case_failed(CASE_ID, error.to_string()))?;
    let mut registry = WalletRegistry::default();
    registry
        .create_wallet(&store, "alpha", wallet, PersistMode::Sync)
        .map_err(|error| BenchError::case_failed(CASE_ID, error.to_string()))?;

    let runtime = WalletRescanRuntime::open(store, PersistMode::Sync)
        .map_err(|error| BenchError::case_failed(CASE_ID, error.to_string()))?;
    let job = runtime
        .enqueue_rescan("alpha")
        .map_err(|error| BenchError::case_failed(CASE_ID, error.to_string()))?;
    let reopened_registry = WalletRegistry::load(runtime.store())
        .map_err(|error| BenchError::case_failed(CASE_ID, error.to_string()))?;
    let wallet_snapshot = reopened_registry
        .wallet_snapshot("alpha")
        .map_err(|error| BenchError::case_failed(CASE_ID, error.to_string()))?;

    if job.state != WalletRescanJobState::Complete || job.freshness != WalletRescanFreshness::Fresh
    {
        return Err(BenchError::case_failed(
            CASE_ID,
            "wallet rescan runtime did not complete with a fresh result",
        ));
    }
    if wallet_snapshot.maybe_tip_height != Some(3) || wallet_snapshot.utxos.len() != 2 {
        return Err(BenchError::case_failed(
            CASE_ID,
            "wallet rescan runtime did not update the managed wallet snapshot",
        ));
    }

    Ok(())
}
