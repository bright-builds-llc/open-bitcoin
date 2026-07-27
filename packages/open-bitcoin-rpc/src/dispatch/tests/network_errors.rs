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

#[test]
fn package_shape_network_error_maps_to_internal_failure() {
    // Arrange
    let error = ManagedNetworkError::PackageShape(PackageShapeError::Empty);

    // Act
    let failure = super::network_error_to_failure(error);

    // Assert
    assert_eq!(failure.kind, RpcFailureKind::InternalError);
}
