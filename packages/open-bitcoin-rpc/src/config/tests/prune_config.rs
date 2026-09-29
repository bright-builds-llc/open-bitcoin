// Parity breadcrumbs:
// - packages/bitcoin-knots/src/node/blockmanager_args.cpp

use open_bitcoin_network::advertised_service_flags;
use open_bitcoin_node::core::chainstate::PruneMode;
use open_bitcoin_node::core::mempool::PolicyConfig;
use open_bitcoin_node::core::network::{LocalPeerConfig, ServiceFlags};
use open_bitcoin_node::core::primitives::{NetworkAddress, NetworkMagic};
use open_bitcoin_node::{ManagedNetworkHandle, ManagedPeerNetwork, MemoryChainstateStore};

use crate::ManagedRpcContext;

use super::{
    OpenBitcoinConfig, RuntimeConfig, TestDirectory, cli_arg, fs, load_runtime_config_for_args,
    parse_open_bitcoin_jsonc_config,
};

fn load_jsonc(label: &str, text: &str) -> RuntimeConfig {
    let sandbox = TestDirectory::new(label);
    fs::write(sandbox.child("open-bitcoin.jsonc"), text).expect("open bitcoin config");
    load_runtime_config_for_args(&[cli_arg("datadir", &sandbox.path)], &sandbox.path)
        .expect("runtime config")
}

fn rejected_prune_message(label: &str, text: &str) -> String {
    let sandbox = TestDirectory::new(label);
    fs::write(sandbox.child("open-bitcoin.jsonc"), text).expect("open bitcoin config");
    let error = load_runtime_config_for_args(&[cli_arg("datadir", &sandbox.path)], &sandbox.path)
        .expect_err("invalid prune");
    let message = error.to_string();
    let datadir = sandbox.path.display().to_string();
    assert!(
        !message.contains(&datadir),
        "config error must name the integer, not the datadir"
    );
    message
}

#[test]
fn default_prune_is_disabled() {
    // Arrange
    let config = OpenBitcoinConfig::default();
    let runtime = RuntimeConfig::default();

    // Act / Assert
    assert_eq!(config.prune, 0);
    assert_eq!(runtime.prune_mode, PruneMode::Disabled);
}

#[test]
fn missing_jsonc_file_loads_disabled() {
    // Arrange
    let sandbox = TestDirectory::new("prune-missing-file");

    // Act
    let runtime = load_runtime_config_for_args(&[cli_arg("datadir", &sandbox.path)], &sandbox.path)
        .expect("runtime config");

    // Assert
    assert_eq!(runtime.prune_mode, PruneMode::Disabled);
}

#[test]
fn jsonc_without_prune_field_loads_disabled() {
    // Arrange / Act
    let parsed = parse_open_bitcoin_jsonc_config("{ \"schema_version\": 1 }").expect("jsonc");
    let runtime = load_jsonc("prune-field-absent", "{ \"schema_version\": 1 }");

    // Assert
    assert_eq!(parsed.prune, 0);
    assert_eq!(runtime.prune_mode, PruneMode::Disabled);
}

#[test]
fn prune_zero_loads_disabled() {
    // Arrange / Act
    let runtime = load_jsonc("prune-zero", "{ \"prune\": 0 }");

    // Assert
    assert_eq!(runtime.prune_mode, PruneMode::Disabled);
}

#[test]
fn prune_one_loads_manual_only() {
    // Arrange / Act
    let runtime = load_jsonc("prune-one", "{ \"prune\": 1 }");

    // Assert
    assert_eq!(runtime.prune_mode, PruneMode::ManualOnly);
}

#[test]
fn prune_550_loads_automatic_target() {
    // Arrange / Act
    let runtime = load_jsonc("prune-550", "{ \"prune\": 550 }");

    // Assert
    assert_eq!(runtime.prune_mode, PruneMode::Automatic { target_mib: 550 });
}

#[test]
fn prune_two_fails_closed() {
    // Arrange / Act
    let message = rejected_prune_message("prune-two", "{ \"prune\": 2 }");

    // Assert
    assert!(message.contains('2'));
}

#[test]
fn prune_549_fails_closed() {
    // Arrange / Act
    let message = rejected_prune_message("prune-549", "{ \"prune\": 549 }");

    // Assert
    assert!(message.contains("549"));
}

#[test]
fn prune_negative_one_fails_closed() {
    // Arrange / Act
    let message = rejected_prune_message("prune-negative", "{ \"prune\": -1 }");

    // Assert
    assert!(message.contains("-1"));
}

#[test]
fn unknown_jsonc_key_still_fails() {
    // Arrange
    let text = r#"{ "prune_bytes": 550 }"#;

    // Act
    let error = parse_open_bitcoin_jsonc_config(text).expect_err("unknown field should fail");

    // Assert
    assert!(error.to_string().contains("unknown field"));
}

fn startup_service_bits(context: &ManagedRpcContext) -> u64 {
    context
        .network_info()
        .expect("network info")
        .local_services_bits
}

fn fresh_network_handle() -> ManagedNetworkHandle {
    let local_config = LocalPeerConfig {
        magic: NetworkMagic::MAINNET,
        services: ServiceFlags::NETWORK | ServiceFlags::WITNESS,
        address: NetworkAddress {
            services: 0,
            address_bytes: [0_u8; 16],
            port: 18_444,
        },
        nonce: 0,
        relay: true,
        user_agent: "/open-bitcoin:prune-config/".to_string(),
    };
    ManagedNetworkHandle::from_network_fixture(ManagedPeerNetwork::new(
        MemoryChainstateStore::default(),
        local_config,
        PolicyConfig::default(),
    ))
}

#[test]
fn startup_store_applies_manual_only_prune_mode() {
    // Arrange
    let config = RuntimeConfig {
        prune_mode: PruneMode::ManualOnly,
        ..RuntimeConfig::default()
    };

    // Act
    let context =
        ManagedRpcContext::from_runtime_config_with_store(&config, None).expect("startup context");

    // Assert
    assert_eq!(
        startup_service_bits(&context),
        advertised_service_flags(PruneMode::ManualOnly).bits()
    );
}

#[test]
fn startup_store_default_stays_disabled() {
    // Arrange
    let config = RuntimeConfig::default();

    // Act
    let context =
        ManagedRpcContext::from_runtime_config_with_store(&config, None).expect("startup context");

    // Assert
    assert_eq!(
        startup_service_bits(&context),
        advertised_service_flags(PruneMode::Disabled).bits()
    );
}

#[test]
fn startup_network_handle_applies_manual_only_prune_mode() {
    // Arrange
    let config = RuntimeConfig {
        prune_mode: PruneMode::ManualOnly,
        ..RuntimeConfig::default()
    };

    // Act
    let context = ManagedRpcContext::from_runtime_config_with_network_handle(
        &config,
        fresh_network_handle(),
        None,
    )
    .expect("startup context");

    // Assert
    assert_eq!(
        startup_service_bits(&context),
        advertised_service_flags(PruneMode::ManualOnly).bits()
    );
}

#[test]
fn later_set_prune_mode_overrides_startup_mode() {
    // Arrange
    let config = RuntimeConfig {
        prune_mode: PruneMode::ManualOnly,
        ..RuntimeConfig::default()
    };
    let mut context =
        ManagedRpcContext::from_runtime_config_with_store(&config, None).expect("startup context");

    // Act
    context
        .set_prune_mode(PruneMode::Disabled)
        .expect("override startup mode");

    // Assert
    assert_eq!(
        startup_service_bits(&context),
        advertised_service_flags(PruneMode::Disabled).bits()
    );
}
