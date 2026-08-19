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

use super::network_fixtures::*;
use super::*;

#[test]
fn openbitcoinnetworkstatus_mempool_resources_use_open_bitcoin_names() {
    // Arrange
    let (mut context, expected_virtual_size) = resource_fee_evidence_context();

    // Act
    let status = dispatch(
        &mut context,
        MethodCall::OpenBitcoinNetworkStatus(OpenBitcoinNetworkStatusRequest::default()),
    )
    .expect("network status");
    let resources = &status["mempool"]["resources"]["value"];
    let object = resources
        .as_object()
        .expect("mempool.resources.value is an object");

    // Assert
    assert_eq!(status["mempool"]["resources"]["state"], json!("available"));
    assert_eq!(resources["virtual_size"], json!(expected_virtual_size));
    assert_eq!(resources["transaction_count"], json!(1));
    assert_eq!(resources["accounted_capacity"], json!(12_345_678));
    assert_ne!(resources["accounted_usage"], resources["virtual_size"]);
    assert_eq!(object.get("bytes"), None);
    assert_eq!(object.get("usage"), None);
    assert_eq!(object.get("maxmempool"), None);
    assert_eq!(status["mempool"]["transactions"]["value"], json!(1));
}

#[test]
fn openbitcoinnetworkstatus_fee_floors_are_distinct_from_getmempoolinfo_aliases() {
    // Arrange
    let (mut context, _) = resource_fee_evidence_context();

    // Act
    let status = dispatch(
        &mut context,
        MethodCall::OpenBitcoinNetworkStatus(OpenBitcoinNetworkStatusRequest::default()),
    )
    .expect("network status");
    let floors = &status["mempool"]["fee_floors"]["value"];
    let object = floors
        .as_object()
        .expect("mempool.fee_floors.value is an object");

    // Assert
    assert_eq!(status["mempool"]["fee_floors"]["state"], json!("available"));
    assert_eq!(floors["static_relay_floor"], json!(1_000));
    assert_eq!(floors["rolling_mempool_floor"], json!(3_000));
    assert_eq!(floors["effective_admission_floor"], json!(3_000));
    assert_eq!(floors["incremental_relay_fee"], json!(7_000));
    assert_ne!(
        floors["static_relay_floor"],
        floors["rolling_mempool_floor"]
    );
    assert_ne!(
        floors["incremental_relay_fee"],
        floors["effective_admission_floor"]
    );
    assert_eq!(object.get("mempoolminfee"), None);
    assert_eq!(object.get("minrelaytxfee"), None);
    assert_eq!(object.get("incrementalrelayfee"), None);
}

#[test]
fn getmempoolinfo_aliases_remain_bytes_usage_maxmempool_mempoolminfee() {
    // Arrange
    let (mut context, expected_virtual_size) = resource_fee_evidence_context();

    // Act
    let mempool = dispatch(
        &mut context,
        MethodCall::GetMempoolInfo(GetMempoolInfoRequest::default()),
    )
    .expect("mempool info");

    // Assert
    assert_eq!(mempool["bytes"], json!(expected_virtual_size));
    assert!(mempool.get("usage").is_some());
    assert_eq!(mempool["maxmempool"], json!(12_345_678));
    assert_eq!(mempool["mempoolminfee"], json!(3_000));
    assert_eq!(mempool.get("virtual_size"), None);
    assert_eq!(mempool.get("effective_admission_floor"), None);
}
