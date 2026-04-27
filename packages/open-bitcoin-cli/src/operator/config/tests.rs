// Parity breadcrumbs:
// - packages/bitcoin-knots/src/common/config.cpp
// - packages/bitcoin-knots/src/common/args.cpp

use std::{
    collections::BTreeMap,
    fs,
    path::PathBuf,
    sync::atomic::{AtomicU64, Ordering},
};

use super::{
    OPEN_BITCOIN_CONFIG_ENV, OPEN_BITCOIN_DATADIR_ENV, OPEN_BITCOIN_NETWORK_ENV,
    OperatorConfigPathKind, OperatorConfigPathReport, OperatorConfigRequest,
    OperatorConfigResolution, OperatorConfigRoots, OperatorConfigSource, OperatorCredentialSource,
    resolve_operator_config,
};
use crate::operator::NetworkSelection;

static NEXT_TEST_DIRECTORY_ID: AtomicU64 = AtomicU64::new(0);

#[derive(Debug)]
struct TestDirectory {
    path: PathBuf,
}

impl TestDirectory {
    fn new(label: &str) -> Self {
        let directory = std::env::temp_dir().join(format!(
            "open-bitcoin-operator-config-tests-{label}-{}",
            NEXT_TEST_DIRECTORY_ID.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir_all(&directory).expect("test directory");
        Self { path: directory }
    }

    fn child(&self, relative: &str) -> PathBuf {
        self.path.join(relative)
    }
}

impl Drop for TestDirectory {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.path);
    }
}

#[test]
fn config_sources_keep_documented_precedence_names() {
    // Arrange
    let sources = OperatorConfigSource::ordered();

    // Act
    let names: Vec<_> = sources.iter().map(|source| source.as_str()).collect();

    // Assert
    assert_eq!(
        names,
        vec![
            "cli_flags",
            "environment",
            "open_bitcoin_jsonc",
            "bitcoin_conf",
            "cookies",
            "defaults",
        ]
    );
}

#[test]
fn config_resolution_compares_deterministically() {
    // Arrange
    let report = OperatorConfigPathReport {
        source: OperatorConfigSource::OpenBitcoinJsonc,
        kind: OperatorConfigPathKind::ConfigFile,
        path: PathBuf::from("/tmp/open-bitcoin.jsonc"),
        present: true,
    };

    // Act
    let left = OperatorConfigResolution {
        ordered_sources: OperatorConfigSource::ordered().to_vec(),
        sources_considered: OperatorConfigSource::ordered().to_vec(),
        path_reports: vec![report.clone()],
        maybe_config_path: Some(report.path.clone()),
        maybe_bitcoin_conf_path: Some(PathBuf::from("/tmp/bitcoin.conf")),
        maybe_data_dir: Some(PathBuf::from("/tmp/open-bitcoin")),
        maybe_network: Some(NetworkSelection::Regtest),
        maybe_log_dir: Some(PathBuf::from("/tmp/open-bitcoin/logs")),
        maybe_metrics_store_path: Some(PathBuf::from("/tmp/open-bitcoin/metrics")),
        credential_source: OperatorCredentialSource::None,
        maybe_open_bitcoin_config: None,
    };
    let right = left.clone();

    // Assert
    assert_eq!(left, right);
}

#[test]
fn cli_flags_override_environment() {
    // Arrange
    let sandbox = TestDirectory::new("cli-over-env");
    let roots = OperatorConfigRoots::new(sandbox.child("default"));
    let cli_config = sandbox.child("cli-open-bitcoin.jsonc");
    let env_config = sandbox.child("env-open-bitcoin.jsonc");
    let cli_data_dir = sandbox.child("cli-data");
    let env_data_dir = sandbox.child("env-data");
    fs::create_dir_all(&cli_data_dir).expect("cli datadir");
    fs::create_dir_all(&env_data_dir).expect("env datadir");
    fs::write(&cli_config, "{}").expect("cli config");
    fs::write(&env_config, "{}").expect("env config");
    let request = OperatorConfigRequest {
        maybe_config_path: Some(cli_config.clone()),
        maybe_data_dir: Some(cli_data_dir.clone()),
        maybe_network: Some(NetworkSelection::Regtest),
    };
    let environment = BTreeMap::from([
        (
            OPEN_BITCOIN_CONFIG_ENV.to_string(),
            env_config.display().to_string(),
        ),
        (
            OPEN_BITCOIN_DATADIR_ENV.to_string(),
            env_data_dir.display().to_string(),
        ),
        (OPEN_BITCOIN_NETWORK_ENV.to_string(), "signet".to_string()),
    ]);

    // Act
    let resolution =
        resolve_operator_config(&request, &environment, &roots).expect("config resolution");

    // Assert
    assert_eq!(resolution.maybe_config_path, Some(cli_config));
    assert_eq!(resolution.maybe_data_dir, Some(cli_data_dir));
    assert_eq!(resolution.maybe_network, Some(NetworkSelection::Regtest));
    assert_eq!(
        resolution.source_names(),
        vec![
            "cli_flags",
            "environment",
            "open_bitcoin_jsonc",
            "bitcoin_conf",
            "cookies",
            "defaults",
        ]
    );
}

#[test]
fn environment_overrides_open_bitcoin_jsonc() {
    // Arrange
    let sandbox = TestDirectory::new("env-over-jsonc");
    let roots = OperatorConfigRoots::new(sandbox.child("default"));
    let config_path = sandbox.child("open-bitcoin.jsonc");
    let env_data_dir = sandbox.child("env-data");
    let jsonc_data_dir = sandbox.child("jsonc-data");
    fs::create_dir_all(&env_data_dir).expect("env datadir");
    fs::create_dir_all(&jsonc_data_dir).expect("jsonc datadir");
    fs::write(
        &config_path,
        format!(
            r#"{{
              "onboarding": {{
                "wizard_answers": {{
                  "datadir": "{}",
                  "network": "regtest"
                }}
              }}
            }}"#,
            jsonc_data_dir.display()
        ),
    )
    .expect("jsonc config");
    let environment = BTreeMap::from([
        (
            OPEN_BITCOIN_CONFIG_ENV.to_string(),
            config_path.display().to_string(),
        ),
        (
            OPEN_BITCOIN_DATADIR_ENV.to_string(),
            env_data_dir.display().to_string(),
        ),
        (OPEN_BITCOIN_NETWORK_ENV.to_string(), "signet".to_string()),
    ]);

    // Act
    let resolution =
        resolve_operator_config(&OperatorConfigRequest::default(), &environment, &roots)
            .expect("config resolution");

    // Assert
    assert_eq!(resolution.maybe_data_dir, Some(env_data_dir));
    assert_eq!(resolution.maybe_network, Some(NetworkSelection::Signet));
    assert!(resolution.maybe_open_bitcoin_config.is_some());
}

#[test]
fn jsonc_overrides_bitcoin_conf_for_open_bitcoin_owned_settings() {
    // Arrange
    let sandbox = TestDirectory::new("jsonc-over-conf");
    let default_data_dir = sandbox.child("default");
    let jsonc_data_dir = sandbox.child("jsonc-data");
    let bitcoin_conf_data_dir = sandbox.child("bitcoin-conf-data");
    fs::create_dir_all(&default_data_dir).expect("default datadir");
    fs::create_dir_all(&jsonc_data_dir).expect("jsonc datadir");
    fs::create_dir_all(&bitcoin_conf_data_dir).expect("bitcoin.conf datadir");
    fs::write(
        default_data_dir.join("open-bitcoin.jsonc"),
        format!(
            r#"{{
              "onboarding": {{
                "wizard_answers": {{
                  "datadir": "{}",
                  "network": "regtest"
                }}
              }}
            }}"#,
            jsonc_data_dir.display()
        ),
    )
    .expect("jsonc config");
    fs::write(
        jsonc_data_dir.join("bitcoin.conf"),
        format!("datadir={}\n", bitcoin_conf_data_dir.display()),
    )
    .expect("bitcoin.conf");

    // Act
    let resolution = resolve_operator_config(
        &OperatorConfigRequest::default(),
        &BTreeMap::new(),
        &OperatorConfigRoots::new(&default_data_dir),
    )
    .expect("config resolution");

    // Assert
    assert_eq!(resolution.maybe_data_dir, Some(jsonc_data_dir.clone()));
    assert_eq!(resolution.maybe_network, Some(NetworkSelection::Regtest));
    assert_eq!(
        resolution.maybe_bitcoin_conf_path,
        Some(jsonc_data_dir.join("bitcoin.conf"))
    );
}

#[test]
fn cookie_source_reports_path_without_secret() {
    // Arrange
    let sandbox = TestDirectory::new("cookie-report");
    let data_dir = sandbox.child("data");
    fs::create_dir_all(&data_dir).expect("datadir");
    fs::write(data_dir.join(".cookie"), "__cookie__:super-secret-token").expect("cookie");
    let request = OperatorConfigRequest {
        maybe_data_dir: Some(data_dir.clone()),
        ..OperatorConfigRequest::default()
    };

    // Act
    let resolution = resolve_operator_config(
        &request,
        &BTreeMap::new(),
        &OperatorConfigRoots::new(sandbox.child("default")),
    )
    .expect("config resolution");

    // Assert
    assert_eq!(
        resolution.credential_source,
        OperatorCredentialSource::CookieFile {
            path: data_dir.join(".cookie"),
            present: true,
        }
    );
    let rendered = format!("{resolution:?}");
    assert!(!rendered.contains("super-secret-token"));
    assert!(!rendered.contains("Authorization"));
    assert!(!rendered.contains("Basic "));
}

#[test]
fn invalid_jsonc_returns_open_bitcoin_jsonc_error() {
    // Arrange
    let sandbox = TestDirectory::new("invalid-jsonc");
    let roots = OperatorConfigRoots::new(sandbox.child("default"));
    let config_path = sandbox.child("open-bitcoin.jsonc");
    fs::write(&config_path, "{ invalid").expect("jsonc config");
    let request = OperatorConfigRequest {
        maybe_config_path: Some(config_path),
        ..OperatorConfigRequest::default()
    };

    // Act
    let error = resolve_operator_config(&request, &BTreeMap::new(), &roots)
        .expect_err("invalid JSONC must fail");

    // Assert
    assert!(
        error
            .to_string()
            .contains("Error reading open-bitcoin.jsonc")
    );
}

#[test]
fn bitcoin_conf_rejects_open_bitcoin_only_keys() {
    // Arrange
    let sandbox = TestDirectory::new("open-bitcoin-only-conf");
    let data_dir = sandbox.child("data");
    fs::create_dir_all(&data_dir).expect("datadir");
    fs::write(data_dir.join("bitcoin.conf"), "dashboard=1\nservice=1\n").expect("bitcoin.conf");
    let request = OperatorConfigRequest {
        maybe_data_dir: Some(data_dir),
        ..OperatorConfigRequest::default()
    };

    // Act
    let error = resolve_operator_config(
        &request,
        &BTreeMap::new(),
        &OperatorConfigRoots::new(sandbox.child("default")),
    )
    .expect_err("open bitcoin keys must fail");

    // Assert
    assert!(error.to_string().contains("Invalid configuration value"));
}
