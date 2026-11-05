// Parity breadcrumbs:
// - packages/bitcoin-knots/src/bitcoind.cpp
// - packages/bitcoin-knots/src/init.cpp
// - packages/bitcoin-knots/src/rpc/mining.cpp

use super::*;
use open_bitcoin_node::{
    SyncNetwork,
    core::{consensus::CanonicalChain, wallet::AddressNetwork},
};
use open_bitcoin_rpc::{
    config::BasicFilterIndexSetting,
    dispatch::dispatch,
    method::{RequestParameters, normalize_method_call},
};
use std::{fs, path::PathBuf};

struct Source(PathBuf);
impl Source {
    fn new() -> Self {
        let mut nonce = [0; 16];
        getrandom::fill(&mut nonce).expect("owned scenario");
        Self(
            std::env::temp_dir()
                .canonicalize()
                .expect("physical temp")
                .join(format!("phase167-bin-{:x}", u128::from_le_bytes(nonce))),
        )
    }
}
impl Drop for Source {
    fn drop(&mut self) {
        if self.0.exists() {
            fs::remove_dir_all(&self.0).expect("all owned handles dropped");
        }
    }
}
fn configured(source: &Source, setting: BasicFilterIndexSetting) -> RuntimeConfig {
    let mut config = RuntimeConfig {
        chain: AddressNetwork::Regtest,
        maybe_data_dir: Some(source.0.clone()),
        block_filter_index: setting,
        ..Default::default()
    };
    config.sync.runtime.network = SyncNetwork::Regtest;
    config
}

#[test]
fn phase167_daemon_history_route_uses_one_sealed_runtime_and_same_rpc_context() {
    // Arrange
    let source = Source::new();
    let config = configured(&source, BasicFilterIndexSetting::Basic);
    // Act
    let opened = open_configured_history_runtime(&config)
        .expect("safe route")
        .expect("explicit durable trigger");
    let OpenedAuthoritativeRuntime::Durable(mut owner) = opened else {
        panic!("genuine durable owner required");
    };
    let mut context = owner.maybe_prebuilt_context.take().expect("paired context");
    let genesis = CanonicalChain::Regtest.spec().genesis_block();
    let encoded: String = open_bitcoin_codec::encode_block(&genesis)
        .expect("genesis bytes")
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect();
    let call = normalize_method_call(
        "submitblock",
        RequestParameters::Positional(vec![serde_json::json!(encoded)]),
    )
    .expect("registry");
    // Assert
    assert_eq!(
        dispatch(&mut context, call).expect("genuine root duplicate"),
        serde_json::json!("duplicate")
    );
    assert!(owner.maybe_sync_runtime.is_some());
    drop(context);
    drop(owner);
    let reopened = open_configured_history_runtime(&config)
        .expect("full original reproof")
        .expect("same route");
    drop(reopened);
    assert!(!source.0.join("wallet.dat").exists());
}

#[test]
fn phase167_daemon_old_genesis_signature_and_activation_refusal_remain_exact() {
    // Arrange
    let _: fn(
        &RuntimeConfig,
        Option<open_bitcoin_node::storage::OpenedCanonicalGenesis>,
    ) -> Result<OpenedAuthoritativeRuntime, DaemonSyncPreflightError> =
        open_authoritative_network_runtime;
    let source = Source::new();
    let mut config = configured(&source, BasicFilterIndexSetting::Basic);
    config.inbound.enabled = true;
    // Act / Assert
    assert!(open_configured_history_runtime(&config).is_err());
    assert!(!source.0.exists());
    config.inbound.enabled = false;
    config.chain = AddressNetwork::Signet;
    config.sync.runtime.network = SyncNetwork::Signet;
    assert!(
        open_configured_history_runtime(&config)
            .expect("strict selector route")
            .is_none()
    );
    assert!(!source.0.exists());
}
