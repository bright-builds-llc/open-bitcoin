// Parity breadcrumbs:
// - packages/bitcoin-knots/src/rpc/blockchain.cpp
// - packages/bitcoin-knots/src/node/chainstate.cpp

use std::{env, ffi::OsString, fs::OpenOptions, io};

use open_bitcoin_node::{
    chainstate::BasicFilterStartupMode,
    core::consensus::CanonicalChain,
    storage::FjallNodeStore,
    sync::{DurableSyncRuntime, SyncNetwork, SyncRuntimeConfig},
};
use serde::Serialize;

open_bitcoin_node::register_inspection_worker!();

#[derive(Serialize)]
struct CoinRow {
    txid: String,
    vout: u32,
    sats: String,
    script: String,
    coinbase: bool,
    height: u32,
}

fn required_path(key: &str) -> io::Result<OsString> {
    env::var_os(key).ok_or_else(|| io::Error::other(format!("missing {key}")))
}

fn hex(bytes: impl IntoIterator<Item = u8>) -> String {
    bytes
        .into_iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

#[test]
#[ignore = "invoked by the CREC-02 runner after shutting down its isolated Regtest daemon"]
fn observe_closed_regtest_state() -> Result<(), Box<dyn std::error::Error>> {
    let path = required_path("OPEN_BITCOIN_CREC02_DATADIR")?;
    let output = required_path("OPEN_BITCOIN_CREC02_OUTPUT")?;
    let opened = FjallNodeStore::reopen_canonical_history_existing(path, CanonicalChain::Regtest)
        .map_err(|error| {
        io::Error::other(format!("closed Regtest proof refused: {error:?}"))
    })?;
    let runtime = DurableSyncRuntime::open_canonical_history_configured(
        opened,
        SyncRuntimeConfig {
            network: SyncNetwork::Regtest,
            ..Default::default()
        },
        BasicFilterStartupMode::PreserveSaved,
    )?;
    let snapshot = runtime.network_handle().chainstate_snapshot()?;
    let tip = snapshot
        .tip()
        .ok_or_else(|| io::Error::other("proved active tip missing"))?;
    if !(101..=103).contains(&tip.height) || snapshot.utxos.len() > 4096 {
        return Err(io::Error::other("closed Regtest state outside CREC-02 bounds").into());
    }
    let utxos: Vec<_> = snapshot
        .utxos
        .iter()
        .map(|(outpoint, coin)| CoinRow {
            txid: hex(outpoint.txid.to_byte_array().into_iter().rev()),
            vout: outpoint.vout,
            sats: coin.output.value.to_sats().to_string(),
            script: hex(coin.output.script_pubkey.as_bytes().iter().copied()),
            coinbase: coin.is_coinbase,
            height: coin.created_height,
        })
        .collect();
    let report = serde_json::json!({
        "tip": {
            "height": tip.height,
            "hash": hex(tip.block_hash.to_byte_array().into_iter().rev()),
        },
        "chainwork": tip.chain_work.to_hex(),
        "utxos": utxos,
    });
    let file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(output)?;
    serde_json::to_writer(file, &report)?;
    Ok(())
}
