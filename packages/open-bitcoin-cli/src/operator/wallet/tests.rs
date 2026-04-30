// Parity breadcrumbs:
// - packages/bitcoin-knots/src/wallet/rpc/spend.cpp
// - packages/bitcoin-knots/src/wallet/rpc/backup.cpp
// - packages/bitcoin-knots/src/wallet/rpc/util.cpp
// - packages/bitcoin-knots/test/functional/wallet_backup.py

use std::{
    cell::RefCell,
    fs, io,
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

use open_bitcoin_node::{
    FjallNodeStore, PersistMode, WalletRegistry,
    core::wallet::{AddressNetwork, DescriptorRole, Wallet},
};
use open_bitcoin_rpc::method::{
    BuildAndSignTransactionRequest, BuildAndSignTransactionResponse, GetBalancesResponse,
    SelectedInput, SendToAddressRequest, WalletBalanceDetails,
};
use serde_json::json;

use super::*;

const RECEIVE_DESCRIPTOR: &str = "wpkh(cMec2DGaTXkYJYfi7x3ZGjRXkeqmAvYAoWzMAcWj5fdLaqudWsNi)";
const CHANGE_DESCRIPTOR: &str = "sh(wpkh(cMec2DGaTXkYJYfi7x3ZGjRXkeqmAvYAoWzMAcWj5fdLaqudWsNi))";

fn temp_path(test_name: &str) -> PathBuf {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time after unix epoch")
        .as_nanos();
    std::env::temp_dir().join(format!(
        "open-bitcoin-operator-wallet-{test_name}-{}-{timestamp}",
        std::process::id()
    ))
}

fn remove_dir_if_exists(path: &Path) {
    match fs::remove_dir_all(path) {
        Ok(()) => {}
        Err(error) if error.kind() == io::ErrorKind::NotFound => {}
        Err(error) => panic!("failed to remove {}: {error}", path.display()),
    }
}

fn managed_wallet_store(wallet_names: &[&str]) -> PathBuf {
    let path = temp_path("managed-wallet-store");
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("store");
    let mut registry = WalletRegistry::default();
    for wallet_name in wallet_names {
        let mut wallet = Wallet::new(AddressNetwork::Regtest);
        wallet
            .import_descriptor("receive", DescriptorRole::External, RECEIVE_DESCRIPTOR)
            .expect("receive descriptor");
        wallet
            .import_descriptor("change", DescriptorRole::Internal, CHANGE_DESCRIPTOR)
            .expect("change descriptor");
        registry
            .create_wallet(&store, *wallet_name, wallet, PersistMode::Sync)
            .expect("create wallet");
    }
    if let Some(wallet_name) = wallet_names.first() {
        registry
            .set_selected_wallet(&store, wallet_name, PersistMode::Sync)
            .expect("select wallet");
    }
    path
}

fn sample_send_args() -> WalletSendArgs {
    WalletSendArgs {
        address: "mipcBbFg9gMiCh81Kj8tqqdgoZub1ZJRfn".to_string(),
        amount_sats: 12_000,
        maybe_fee_rate_sat_per_kvb: Some(2_000),
        maybe_conf_target: None,
        maybe_estimate_mode: None,
        maybe_change_descriptor_id: Some(1),
        maybe_lock_time: Some(44),
        enable_rbf: true,
        maybe_max_tx_fee_sats: Some(5_000),
        confirm: false,
    }
}

#[derive(Default)]
struct FakeWalletOperatorRpcClient {
    build_calls: RefCell<Vec<(Option<String>, BuildAndSignTransactionRequest)>>,
    send_calls: RefCell<Vec<(Option<String>, SendToAddressRequest)>>,
}

impl WalletOperatorRpcClient for FakeWalletOperatorRpcClient {
    fn build_and_sign_transaction(
        &self,
        maybe_wallet_name: Option<&str>,
        request: BuildAndSignTransactionRequest,
    ) -> Result<BuildAndSignTransactionResponse, WalletOperatorError> {
        self.build_calls
            .borrow_mut()
            .push((maybe_wallet_name.map(str::to_owned), request));
        Ok(BuildAndSignTransactionResponse {
            transaction_hex: "001122".to_string(),
            fee_sats: 220,
            inputs: vec![SelectedInput {
                txid_hex: "aa".repeat(32),
                vout: 0,
                descriptor_id: 1,
                amount_sats: 75_000,
            }],
            maybe_change_output_index: Some(1),
        })
    }

    fn send_to_address(
        &self,
        maybe_wallet_name: Option<&str>,
        request: SendToAddressRequest,
    ) -> Result<String, WalletOperatorError> {
        self.send_calls
            .borrow_mut()
            .push((maybe_wallet_name.map(str::to_owned), request));
        Ok("bb".repeat(32))
    }

    fn get_wallet_info(
        &self,
        maybe_wallet_name: Option<&str>,
    ) -> Result<serde_json::Value, WalletOperatorError> {
        Ok(json!({
            "walletname": maybe_wallet_name,
            "freshness": "fresh",
            "scanning": false,
            "maybe_tip_height": 144,
        }))
    }

    fn get_balances(
        &self,
        _maybe_wallet_name: Option<&str>,
    ) -> Result<GetBalancesResponse, WalletOperatorError> {
        Ok(GetBalancesResponse {
            mine: WalletBalanceDetails {
                trusted_sats: 50_000,
                untrusted_pending_sats: 0,
                immature_sats: 0,
            },
        })
    }
}

#[test]
fn send_preview_requires_confirm_before_commit() {
    // Arrange
    let rpc = FakeWalletOperatorRpcClient::default();
    let args = sample_send_args();

    // Act
    let outcome = execute_send_command(
        OperatorOutputFormat::Human,
        Some("alpha"),
        AddressNetwork::Regtest,
        &args,
        &rpc,
    )
    .expect("preview outcome");

    // Assert
    assert_eq!(outcome.exit_code, OperatorExitCode::Failure(1));
    assert!(outcome.stdout.text.contains("Wallet: alpha"));
    assert!(outcome.stderr.text.contains(CONFIRMATION_REQUIRED_MESSAGE));
    assert_eq!(rpc.build_calls.borrow().len(), 1);
    assert!(rpc.send_calls.borrow().is_empty());
}

#[test]
fn confirmed_send_delegates_to_sendtoaddress_commit_path() {
    // Arrange
    let rpc = FakeWalletOperatorRpcClient::default();
    let mut args = sample_send_args();
    args.confirm = true;

    // Act
    let outcome = execute_send_command(
        OperatorOutputFormat::Json,
        Some("alpha"),
        AddressNetwork::Regtest,
        &args,
        &rpc,
    )
    .expect("confirmed outcome");

    // Assert
    assert_eq!(outcome.exit_code, OperatorExitCode::Success);
    assert!(outcome.stdout.text.contains("\"txid\""));
    let send_calls = rpc.send_calls.borrow();
    assert_eq!(send_calls.len(), 1);
    assert_eq!(send_calls[0].0.as_deref(), Some("alpha"));
    assert_eq!(send_calls[0].1.address, args.address);
    assert_eq!(send_calls[0].1.amount_sats, args.amount_sats);
}

#[test]
fn managed_wallet_resolution_rejects_unknown_wallet_names() {
    // Arrange
    let store_path = managed_wallet_store(&["alpha", "beta"]);

    // Act
    let error = resolve_managed_wallet_name(Some(&store_path), Some("missing"))
        .expect_err("unknown wallet");

    // Assert
    assert_eq!(
        error.to_string(),
        "Requested wallet does not exist or is not loaded"
    );

    remove_dir_if_exists(&store_path);
}

#[test]
fn backup_rejects_unsafe_external_wallet_destinations() {
    // Arrange
    let root = temp_path("unsafe-backup");
    fs::create_dir_all(root.join(".bitcoin/wallets/external")).expect("wallet dir");
    let detections = vec![DetectedInstallation {
        product_family: super::super::detect::ProductFamily::BitcoinCore,
        confidence: super::super::detect::DetectionConfidence::High,
        uncertainty: Vec::new(),
        source_paths: Vec::new(),
        maybe_data_dir: Some(root.join(".bitcoin")),
        maybe_config_file: None,
        maybe_cookie_file: None,
        wallet_candidates: vec![super::super::detect::WalletCandidate {
            kind: super::super::detect::WalletCandidateKind::DescriptorWalletDirectory,
            path: root.join(".bitcoin/wallets/external"),
            maybe_name: Some("external".to_string()),
            present: true,
            product_family: super::super::detect::ProductFamily::BitcoinCore,
            product_confidence: super::super::detect::DetectionConfidence::High,
            chain_scope: super::super::detect::WalletChainScope::Regtest,
        }],
    }];

    // Act
    let error = ensure_safe_backup_destination(
        &root.join(".bitcoin/wallets/external/backup.json"),
        &detections,
    )
    .expect_err("unsafe destination");

    // Assert
    assert!(
        error
            .to_string()
            .contains("backup destination overlaps detected external wallet candidate")
    );

    remove_dir_if_exists(&root);
}
