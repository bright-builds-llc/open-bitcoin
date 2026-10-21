// Parity breadcrumbs:
// - packages/bitcoin-knots/src/kernel/chainparams.cpp
// - packages/bitcoin-knots/src/validation.cpp

use super::*;
use crate::operator::{OperatorCli, runtime::execute_operator_cli_with_default_data_dir};
use clap::Parser;
use open_bitcoin_node::{
    DurableSyncRuntime, SyncNetwork, SyncRuntimeConfig, chainstate::BasicFilterStartupMode,
};

static PROCESS_SCENARIO: std::sync::Mutex<()> = std::sync::Mutex::new(());

#[test]
fn phase166_operator_observes_existing_genesis_through_real_cli_parsing() {
    let _scenario = PROCESS_SCENARIO.lock().expect("canonical process scenario");
    let vectors: serde_json::Value = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../docs/parity/fixtures/phase166-genesis-vectors.json"
    )))
    .expect("independent vectors");
    for ((chain, network), selector) in CHAINS
        .into_iter()
        .zip([
            SyncNetwork::Mainnet,
            SyncNetwork::Testnet,
            SyncNetwork::Signet,
            SyncNetwork::Regtest,
        ])
        .zip(["mainnet", "testnet", "signet", "regtest"])
    {
        // Arrange
        let fixture = SourceFixture::new("missing");
        let runtime = DurableSyncRuntime::open_canonical_genesis_configured(
            FjallNodeStore::open_canonical_genesis(&fixture.source, chain)
                .expect("proved fresh store"),
            SyncRuntimeConfig {
                network,
                ..Default::default()
            },
            BasicFilterStartupMode::Enabled,
        )
        .expect("earned genesis runtime");
        drop(runtime);
        let cli = OperatorCli::try_parse_from([
            "open-bitcoin",
            "--datadir",
            fixture.source.to_str().expect("owned path"),
            "--network",
            selector,
            "--format",
            "json",
            "sync",
            "status",
        ])
        .expect("actual operator parser");
        // Act
        let outcome = execute_operator_cli_with_default_data_dir(cli, fixture.root.join("unused"));
        // Assert
        assert!(outcome.stderr.text.is_empty(), "{}", outcome.stderr.text);
        let output: serde_json::Value =
            serde_json::from_str(&outcome.stdout.text).expect("status JSON");
        let vector = vectors["genesis"]
            .as_array()
            .expect("vectors")
            .iter()
            .find(|vector| vector["chain"] == chain.to_string())
            .expect("selected vector");
        let progress = &output["maybe_sync_state"]["sync"]["sync_progress"]["value"];
        assert_eq!(output["selected_network"], chain.to_string());
        assert_eq!(
            progress["maybe_validated_active_chain_hash"],
            vector["block_hash_raw"]
        );
        assert_eq!(
            progress["maybe_validated_active_chain_work"],
            vector["proof_hex"]
        );
        assert_eq!(progress["validated_active_chain_height"], 0);
        assert_eq!(output["basic_index"]["configured_indexing"]["value"], true);
        assert_eq!(output["basic_index"]["safe_durable_height"]["value"], 0);
        assert_eq!(
            output["basic_index"]["retention"]["value"]["retained_record_count"],
            1
        );
    }
}

#[test]
fn phase166_operator_absent_and_empty_status_never_bootstrap() {
    let _scenario = PROCESS_SCENARIO.lock().expect("canonical process scenario");
    for shape in ["missing", "empty"] {
        let fixture = SourceFixture::new(shape);
        let before = fixture.snapshot();
        for chain in CHAINS {
            // Act
            let result = execute_offline_sync_command(
                &fixture.source,
                chain,
                &SyncArgs {
                    command: SyncCommand::Status,
                },
                OperatorOutputFormat::Json,
            );
            // Assert
            assert!(result.is_err());
            assert_eq!(fixture.snapshot(), before);
        }
    }
}

#[test]
fn phase166_operator_genesis_cannot_authorize_sync_mutation_or_wallet_lookup() {
    let _scenario = PROCESS_SCENARIO.lock().expect("canonical process scenario");
    let fixture = SourceFixture::new("missing");
    drop(
        FjallNodeStore::open_canonical_genesis(&fixture.source, CanonicalChain::Regtest)
            .expect("canonical fixture"),
    );
    let before = fixture.snapshot();
    for command in [SyncCommand::Pause, SyncCommand::Resume] {
        assert!(
            execute_offline_sync_command(
                &fixture.source,
                CanonicalChain::Regtest,
                &SyncArgs { command },
                OperatorOutputFormat::Json
            )
            .is_err()
        );
        assert_eq!(fixture.snapshot(), before);
    }
    let destination = fixture.root.join("uncreated-backup");
    let cli = OperatorCli::try_parse_from([
        "open-bitcoin",
        "--datadir",
        fixture.source.to_str().expect("path"),
        "--network",
        "regtest",
        "wallet",
        "backup",
        destination.to_str().expect("destination"),
    ])
    .expect("wallet parser");
    let outcome = execute_operator_cli_with_default_data_dir(cli, fixture.root.join("unused"));
    assert_ne!(outcome.exit_code.code(), 0);
    assert!(outcome.stderr.text.contains("ordinary_history=false"));
    assert!(!destination.exists());
    assert_eq!(fixture.snapshot(), before);
}
