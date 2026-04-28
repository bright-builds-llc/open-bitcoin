// Parity breadcrumbs:
// - packages/bitcoin-knots/src/node/blockstorage.cpp
// - packages/bitcoin-knots/src/validation.cpp

use open_bitcoin_node::{FjallNodeStore, PersistMode};

use crate::{
    error::BenchError,
    registry::{
        BenchCase, BenchDurability, BenchGroupId, BenchMeasurement, STORAGE_RECOVERY_MAPPING,
    },
    runtime_fixtures::{
        TempStoreDir, build_block, funded_chainstate, sample_header_entries,
        sample_metrics_storage_snapshot, sample_runtime_metadata, wallet_with_ranged_descriptor,
    },
};

const WRITE_READ_CASE_ID: &str = "storage-recovery.write-read";
const RESTART_CASE_ID: &str = "storage-recovery.restart-reopen";

pub const CASES: [BenchCase; 2] = [
    BenchCase {
        id: WRITE_READ_CASE_ID,
        group: BenchGroupId::StorageRecovery,
        description: "Writes and reads durable chainstate, wallet, header, block, metrics, and runtime metadata.",
        measurement: BenchMeasurement {
            focus: "storage_write_read",
            fixture: "temp_fjall_store",
            durability: BenchDurability::Durable,
        },
        knots_mapping: &STORAGE_RECOVERY_MAPPING,
        run_once: run_write_read_case,
    },
    BenchCase {
        id: RESTART_CASE_ID,
        group: BenchGroupId::StorageRecovery,
        description: "Reopens a populated durable store and reloads runtime state after restart.",
        measurement: BenchMeasurement {
            focus: "restart_recovery",
            fixture: "temp_fjall_store",
            durability: BenchDurability::Durable,
        },
        knots_mapping: &STORAGE_RECOVERY_MAPPING,
        run_once: run_restart_recovery_case,
    },
];

fn run_write_read_case() -> Result<(), BenchError> {
    let temp_dir = TempStoreDir::new("storage-recovery-write-read")?;
    let store = FjallNodeStore::open(temp_dir.path())
        .map_err(|error| BenchError::case_failed(WRITE_READ_CASE_ID, error.to_string()))?;
    let wallet = wallet_with_ranged_descriptor()?;
    let chainstate = funded_chainstate(&wallet)?;
    let wallet_snapshot = wallet.snapshot();
    let header_entries = sample_header_entries();
    let block = build_block(header_entries[0].block_hash, 3)?;
    let block_hash = open_bitcoin_node::core::consensus::block_hash(&block.header);
    let metrics = sample_metrics_storage_snapshot();
    let metadata = sample_runtime_metadata();

    store
        .save_chainstate_snapshot(&chainstate, PersistMode::Sync)
        .map_err(|error| BenchError::case_failed(WRITE_READ_CASE_ID, error.to_string()))?;
    store
        .save_wallet_snapshot(&wallet_snapshot, PersistMode::Sync)
        .map_err(|error| BenchError::case_failed(WRITE_READ_CASE_ID, error.to_string()))?;
    store
        .save_header_entries(&header_entries, PersistMode::Sync)
        .map_err(|error| BenchError::case_failed(WRITE_READ_CASE_ID, error.to_string()))?;
    store
        .save_block(&block, PersistMode::Sync)
        .map_err(|error| BenchError::case_failed(WRITE_READ_CASE_ID, error.to_string()))?;
    store
        .save_metrics_snapshot(&metrics, PersistMode::Sync)
        .map_err(|error| BenchError::case_failed(WRITE_READ_CASE_ID, error.to_string()))?;
    store
        .save_runtime_metadata(&metadata, PersistMode::Sync)
        .map_err(|error| BenchError::case_failed(WRITE_READ_CASE_ID, error.to_string()))?;

    if store
        .load_chainstate_snapshot()
        .map_err(|error| BenchError::case_failed(WRITE_READ_CASE_ID, error.to_string()))?
        != Some(chainstate)
    {
        return Err(BenchError::case_failed(
            WRITE_READ_CASE_ID,
            "stored chainstate snapshot did not round-trip",
        ));
    }
    if store
        .load_wallet_snapshot()
        .map_err(|error| BenchError::case_failed(WRITE_READ_CASE_ID, error.to_string()))?
        != Some(wallet_snapshot)
    {
        return Err(BenchError::case_failed(
            WRITE_READ_CASE_ID,
            "stored wallet snapshot did not round-trip",
        ));
    }
    if store
        .load_block(block_hash)
        .map_err(|error| BenchError::case_failed(WRITE_READ_CASE_ID, error.to_string()))?
        != Some(block)
    {
        return Err(BenchError::case_failed(
            WRITE_READ_CASE_ID,
            "stored block did not round-trip",
        ));
    }

    Ok(())
}

fn run_restart_recovery_case() -> Result<(), BenchError> {
    let temp_dir = TempStoreDir::new("storage-recovery-restart")?;
    let wallet = wallet_with_ranged_descriptor()?;
    let chainstate = funded_chainstate(&wallet)?;
    let wallet_snapshot = wallet.snapshot();
    let header_entries = sample_header_entries();
    let block = build_block(header_entries[1].block_hash, 4)?;
    let block_hash = open_bitcoin_node::core::consensus::block_hash(&block.header);
    let metrics = sample_metrics_storage_snapshot();
    let metadata = sample_runtime_metadata();

    {
        let store = FjallNodeStore::open(temp_dir.path())
            .map_err(|error| BenchError::case_failed(RESTART_CASE_ID, error.to_string()))?;
        store
            .save_chainstate_snapshot(&chainstate, PersistMode::Sync)
            .map_err(|error| BenchError::case_failed(RESTART_CASE_ID, error.to_string()))?;
        store
            .save_wallet_snapshot(&wallet_snapshot, PersistMode::Sync)
            .map_err(|error| BenchError::case_failed(RESTART_CASE_ID, error.to_string()))?;
        store
            .save_header_entries(&header_entries, PersistMode::Sync)
            .map_err(|error| BenchError::case_failed(RESTART_CASE_ID, error.to_string()))?;
        store
            .save_block(&block, PersistMode::Sync)
            .map_err(|error| BenchError::case_failed(RESTART_CASE_ID, error.to_string()))?;
        store
            .save_metrics_snapshot(&metrics, PersistMode::Sync)
            .map_err(|error| BenchError::case_failed(RESTART_CASE_ID, error.to_string()))?;
        store
            .save_runtime_metadata(&metadata, PersistMode::Sync)
            .map_err(|error| BenchError::case_failed(RESTART_CASE_ID, error.to_string()))?;
    }

    let reopened = FjallNodeStore::open(temp_dir.path())
        .map_err(|error| BenchError::case_failed(RESTART_CASE_ID, error.to_string()))?;
    let maybe_header_store = reopened
        .load_header_store()
        .map_err(|error| BenchError::case_failed(RESTART_CASE_ID, error.to_string()))?;
    let Some(header_store) = maybe_header_store else {
        return Err(BenchError::case_failed(
            RESTART_CASE_ID,
            "reopened store did not restore header state",
        ));
    };

    if header_store.best_height() != 1 {
        return Err(BenchError::case_failed(
            RESTART_CASE_ID,
            "reopened store restored an unexpected header height",
        ));
    }
    if reopened
        .load_runtime_metadata()
        .map_err(|error| BenchError::case_failed(RESTART_CASE_ID, error.to_string()))?
        != Some(metadata)
    {
        return Err(BenchError::case_failed(
            RESTART_CASE_ID,
            "reopened store did not restore runtime metadata",
        ));
    }
    if reopened
        .load_metrics_snapshot()
        .map_err(|error| BenchError::case_failed(RESTART_CASE_ID, error.to_string()))?
        != Some(metrics)
    {
        return Err(BenchError::case_failed(
            RESTART_CASE_ID,
            "reopened store did not restore metrics history",
        ));
    }
    if reopened
        .load_block(block_hash)
        .map_err(|error| BenchError::case_failed(RESTART_CASE_ID, error.to_string()))?
        != Some(block)
    {
        return Err(BenchError::case_failed(
            RESTART_CASE_ID,
            "reopened store did not restore persisted blocks",
        ));
    }

    Ok(())
}
