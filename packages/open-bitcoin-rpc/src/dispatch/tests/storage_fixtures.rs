// Parity breadcrumbs:
// - packages/bitcoin-knots/src/bitcoind.cpp
// - packages/bitcoin-knots/src/rpc/protocol.h
// - packages/bitcoin-knots/src/rpc/request.cpp
// - packages/bitcoin-knots/src/rpc/server.cpp
// - packages/bitcoin-knots/src/rpc/blockchain.cpp
// - packages/bitcoin-knots/src/rpc/mempool.cpp
// - packages/bitcoin-knots/src/rpc/net.cpp
// - packages/bitcoin-knots/src/rpc/rawtransaction.cpp
// - packages/bitcoin-knots/test/functional/interface_rpc.py

use super::*;

pub(super) fn temp_store_path(test_name: &str) -> PathBuf {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time after unix epoch")
        .as_nanos();
    std::env::temp_dir().join(format!(
        "open-bitcoin-rpc-dispatch-{test_name}-{}-{timestamp}",
        std::process::id()
    ))
}

pub(super) fn remove_dir_if_exists(path: &Path) {
    match fs::remove_dir_all(path) {
        Ok(()) => {}
        Err(error) if error.kind() == io::ErrorKind::NotFound => {}
        Err(error) => panic!("failed to remove {}: {error}", path.display()),
    }
}

pub(super) fn durable_wallet_context(test_name: &str, wallet_name: &str) -> ManagedRpcContext {
    let path = temp_store_path(test_name);
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("store");
    let mut registry = WalletRegistry::default();
    let mut wallet = Wallet::new(AddressNetwork::Regtest);
    wallet
        .import_descriptor(
            "receive-ranged",
            DescriptorRole::External,
            &format!("wpkh({RANGED_TPRV}/1/1/*)"),
        )
        .expect("receive descriptor");
    wallet
        .import_descriptor(
            "change-ranged",
            DescriptorRole::Internal,
            &format!("sh(wpkh({RANGED_TPUB}/1/*))"),
        )
        .expect("change descriptor");
    registry
        .create_wallet(&store, wallet_name.to_string(), wallet, PersistMode::Sync)
        .expect("create wallet");
    drop(store);

    let mut context = ManagedRpcContext::from_runtime_config(&RuntimeConfig {
        chain: AddressNetwork::Regtest,
        maybe_data_dir: Some(path),
        wallet: WalletRuntimeConfig {
            coinbase_maturity: 1,
            ..WalletRuntimeConfig::default()
        },
        ..RuntimeConfig::default()
    });
    context.set_request_wallet_name(Some(wallet_name.to_string()));
    context
}
