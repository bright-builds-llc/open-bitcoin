// Parity breadcrumbs:
// - packages/bitcoin-knots/src/kernel/chainparams.cpp
// - packages/bitcoin-knots/src/validation.cpp

use super::*;
use crate::operator::{OperatorCli, runtime::execute_operator_cli_with_default_data_dir};
use clap::Parser;
use open_bitcoin_node::storage::chain_compatibility::{
    WorkCompatibilityDecision, WorkProfileObservation,
};

pub(crate) fn forged_source(shape: &str) -> SourceFixture {
    let fixture = SourceFixture::new(shape);
    if fixture.source.is_dir() && !fixture.source.is_symlink() {
        for (name, bytes) in [
            ("work-profile", "canonical-exact-work-v1"),
            ("exact-work", "canonical-exact-work-v1"),
            (
                "snapshot.json",
                r#"{"version":1,"chain_work":340282366920938463463374607431768211455,"Exact":"ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff"}"#,
            ),
            ("wallet-secret", "PRIVATE-wallet-secret"),
            ("prune-intent", "block undo PRIVATE-backend-payload"),
            ("basic-index", "malformed BASIC index"),
            ("recovery", "malformed recovery payload"),
        ] {
            fs::write(fixture.source.join(name), bytes).expect("forged closed source");
        }
    }
    fs::write(
        fixture.root.join("bitcoin.conf"),
        b"# PRIVATE rpcpassword=wallet-secret\n",
    )
    .expect("valid config sentinel");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(
            fixture.root.join("bitcoin.conf"),
            fs::Permissions::from_mode(0o600),
        )
        .expect("private config permissions");
        std::os::unix::fs::symlink("bitcoin.conf", fixture.root.join("config-link"))
            .expect("owned sentinel link");
    }
    fixture
}

pub(crate) fn assert_work_message(message: &str, chain: CanonicalChain, decision: &str) {
    assert!(
        message.contains(&format!("selected_network={chain}")),
        "{message}"
    );
    assert!(message.contains("observed_work=unavailable"), "{message}");
    assert!(
        message.contains("scope=identity_only; ordinary_history=false"),
        "{message}"
    );
    assert!(
        message.contains(&format!("work_decision={decision}")),
        "{message}"
    );
    assert!(message.len() <= 1100);
    for secret in [
        "PRIVATE",
        "rpcpassword",
        "wallet-secret",
        "backend-payload",
        "snapshot.json",
    ] {
        assert!(!message.contains(secret), "diagnostic leaked {secret}");
    }
}

#[test]
fn phase165_offline_controls_use_typed_work_policy_and_preserve_forged_sources() {
    for shape in SHAPES {
        // Arrange
        let fixture = if shape == "empty" {
            SourceFixture::new(shape)
        } else {
            forged_source(shape)
        };
        let before = fixture.snapshot();
        let (decision, code) = match shape {
            "missing" | "empty" => (
                WorkCompatibilityDecision::BootstrapUnavailable,
                "bootstrap_unavailable",
            ),
            "file" | "symlink" => (WorkCompatibilityDecision::UnsafePath, "unsafe_path"),
            _ => (
                WorkCompatibilityDecision::ExistingUnproved,
                "existing_unproved",
            ),
        };
        for chain in CHAINS {
            for command in [SyncCommand::Status, SyncCommand::Pause, SyncCommand::Resume] {
                // Act
                let error = execute_offline_sync_command(
                    &fixture.source,
                    chain,
                    &SyncArgs { command },
                    OperatorOutputFormat::Json,
                )
                .expect_err("ordinary refusal");
                // Assert
                let OperatorRuntimeError::CompatibilityRefused { refusal } = error else {
                    panic!("typed work refusal required");
                };
                assert_eq!(refusal.work_decision(), decision);
                assert_eq!(refusal.selected_genesis_hash(), chain.spec().genesis_hash());
                assert!(!refusal.selected_scope().ordinary_history());
                assert_eq!(refusal.work_profile(), WorkProfileObservation::Unavailable);
                assert_work_message(&refusal.to_string(), chain, code);
                assert!(
                    !refusal
                        .to_string()
                        .contains(&fixture.root.display().to_string())
                );
                assert_eq!(fixture.snapshot(), before, "{shape} {chain}");
            }
        }
    }
}

#[test]
fn phase165_parsed_operator_sync_and_wallet_routes_render_shared_work_refusal() {
    for shape in ["durable", "malformed", "canonical_looking"] {
        // Arrange
        let fixture = forged_source(shape);
        let config = fixture.root.join("operator.jsonc");
        for (selector, chain) in ["mainnet", "testnet", "signet", "regtest"]
            .into_iter()
            .zip(CHAINS)
        {
            fs::write(
                &config,
                serde_json::json!({"onboarding":{"wizard_answers":{"network":selector}}})
                    .to_string(),
            )
            .expect("valid selected config");
            let before = fixture.snapshot();
            for format in ["human", "json"] {
                for command in [
                    vec!["sync", "status"],
                    vec!["sync", "pause"],
                    vec!["sync", "resume"],
                    vec!["wallet", "backup", "new-backup.json"],
                ] {
                    let destination = fixture.root.join("new-backup.json");
                    let mut args = vec![
                        "open-bitcoin".to_owned(),
                        "--config".to_owned(),
                        config.display().to_string(),
                        "--datadir".to_owned(),
                        fixture.source.display().to_string(),
                        "--format".to_owned(),
                        format.to_owned(),
                    ];
                    args.extend(command.into_iter().map(|part| {
                        if part == "new-backup.json" {
                            destination.display().to_string()
                        } else {
                            part.to_owned()
                        }
                    }));
                    let cli = OperatorCli::try_parse_from(args).expect("valid operator options");
                    // Act
                    let outcome =
                        execute_operator_cli_with_default_data_dir(cli, fixture.root.clone());
                    // Assert
                    assert_eq!(outcome.exit_code.code(), 1);
                    assert!(outcome.stdout.text.is_empty());
                    assert_work_message(&outcome.stderr.text, chain, "existing_unproved");
                    assert!(
                        !outcome
                            .stderr
                            .text
                            .contains(&fixture.root.display().to_string())
                    );
                    if format == "json" {
                        let json: serde_json::Value =
                            serde_json::from_str(&outcome.stderr.text).expect("JSON refusal");
                        assert_eq!(json["error"], "chain_compatibility_refused");
                    }
                    assert!(!destination.exists());
                    assert_eq!(fixture.snapshot(), before, "{shape} {selector}");
                }
            }
        }
    }
}

#[test]
fn phase165_parsed_soak_start_refuses_work_before_ledger_creation() {
    // Arrange
    let fixture = forged_source("durable");
    let config = fixture.root.join("operator.jsonc");
    for (selector, chain) in ["mainnet", "testnet", "signet", "regtest"]
        .into_iter()
        .zip(CHAINS)
    {
        fs::write(
            &config,
            serde_json::json!({"onboarding":{"wizard_answers":{"network":selector}}}).to_string(),
        )
        .expect("valid selected config");
        let before = fixture.snapshot();
        let args = vec![
            "open-bitcoin".to_owned(),
            "--config".to_owned(),
            config.display().to_string(),
            "--datadir".to_owned(),
            fixture.source.display().to_string(),
            "--format".to_owned(),
            "json".to_owned(),
            "soak".to_owned(),
            "start".to_owned(),
            "--elapsed-time-seconds".to_owned(),
            "60".to_owned(),
            "--checkpoint-interval-seconds".to_owned(),
            "15".to_owned(),
            "--peer-policy".to_owned(),
            "no-dns-seeds".to_owned(),
            "--disk-budget-bytes".to_owned(),
            "1048576".to_owned(),
            "--stop-condition".to_owned(),
            "elapsed-time".to_owned(),
            "--run-id".to_owned(),
            "phase165-owned-refusal".to_owned(),
        ];
        let cli = OperatorCli::try_parse_from(args).expect("valid bounded soak options");
        // Act
        let outcome = execute_operator_cli_with_default_data_dir(cli, fixture.root.clone());
        // Assert
        assert_eq!(outcome.exit_code.code(), 1);
        assert!(outcome.stdout.text.is_empty());
        assert_work_message(&outcome.stderr.text, chain, "existing_unproved");
        assert!(
            !outcome
                .stderr
                .text
                .contains(&fixture.root.display().to_string())
        );
        assert!(!fixture.source.join("soak").exists());
        assert_eq!(fixture.snapshot(), before);
    }
}
