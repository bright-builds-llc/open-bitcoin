use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use super::{RpcAuthConfig, RuntimeConfig, WalletRuntimeScope};

#[test]
fn runtime_config_defaults_to_local_single_wallet_auth() {
    // Arrange
    let expected_bind = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 8332);

    // Act
    let runtime = RuntimeConfig::default();

    // Assert
    assert_eq!(runtime.server.bind_address, expected_bind);
    assert_eq!(runtime.client.connect_address, expected_bind);
    assert_eq!(
        runtime.wallet_scope,
        WalletRuntimeScope::LocalOperatorSingleWallet
    );
    assert!(matches!(
        runtime.server.auth,
        RpcAuthConfig::CookieFile { .. }
    ));
    assert!(matches!(
        runtime.client.auth,
        RpcAuthConfig::CookieFile { .. }
    ));
}
