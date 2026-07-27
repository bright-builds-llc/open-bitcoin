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
fn get_mempool_info_exposes_truthful_resource_and_fee_evidence() {
    // Arrange
    let (mut context, expected_virtual_size) = resource_fee_evidence_context();

    // Act
    let mempool = dispatch(
        &mut context,
        MethodCall::GetMempoolInfo(GetMempoolInfoRequest::default()),
    )
    .expect("mempool");
    let serialized = mempool.to_string();

    // Assert
    assert_eq!(mempool["size"], json!(1));
    assert_eq!(mempool["bytes"], json!(expected_virtual_size));
    let usage = mempool["usage"].as_u64().expect("usage") as usize;
    let bytes = mempool["bytes"].as_u64().expect("bytes") as usize;
    let maxmempool = mempool["maxmempool"].as_u64().expect("maxmempool") as usize;
    assert!(usage > bytes);
    assert_eq!(maxmempool, 12_345_678);
    assert_ne!(bytes, usage);
    assert_ne!(usage, maxmempool);
    assert_ne!(bytes, maxmempool);
    assert_eq!(mempool["minrelaytxfee"], json!(1_000));
    assert_eq!(mempool["incrementalrelayfee"], json!(7_000));
    assert_eq!(mempool["rollingmempoolfee"], json!(3_000));
    assert_eq!(mempool["mempoolminfee"], json!(3_000));
    assert_eq!(mempool["effectiveadmissionfee"], json!(3_000));
    assert_eq!(mempool["mempoolminfee"], mempool["effectiveadmissionfee"]);
    let static_fee = mempool["minrelaytxfee"].as_i64().expect("static");
    let rolling_fee = mempool["rollingmempoolfee"].as_i64().expect("rolling");
    let effective = mempool["mempoolminfee"].as_i64().expect("effective");
    assert_eq!(effective, static_fee.max(rolling_fee));
    assert_ne!(
        effective,
        mempool["incrementalrelayfee"]
            .as_i64()
            .expect("incremental")
    );
    assert_eq!(mempool["capacityenforcement"], json!("accounted_memory"));
    assert_eq!(mempool["loaded"], json!(true));
    for forbidden in [
        "txid",
        "wtxid",
        "peer_id",
        "peerid",
        "127.0.0.1",
        "script_sig",
        "transaction_hex",
        "hexstring",
    ] {
        assert!(
            !serialized.to_lowercase().contains(forbidden),
            "shared evidence leaked {forbidden}: {serialized}"
        );
    }
}

#[test]
fn node_info_methods_return_documented_phase_8_fields() {
    // Arrange
    let mut context = node_context_with_chain_and_mempool();

    // Act
    let network = dispatch(
        &mut context,
        MethodCall::GetNetworkInfo(GetNetworkInfoRequest::default()),
    )
    .expect("network");
    let blockchain = dispatch(
        &mut context,
        MethodCall::GetBlockchainInfo(GetBlockchainInfoRequest::default()),
    )
    .expect("blockchain");
    let mempool = dispatch(
        &mut context,
        MethodCall::GetMempoolInfo(GetMempoolInfoRequest::default()),
    )
    .expect("mempool");

    // Assert
    assert_eq!(network["connections"], json!(2));
    assert_eq!(network["connections_in"], json!(1));
    assert_eq!(network["connections_out"], json!(1));
    assert_eq!(network["localrelay"], json!(false));
    assert_eq!(blockchain["chain"], json!("regtest"));
    assert_eq!(blockchain["blocks"], json!(1));
    assert_eq!(blockchain["headers"], json!(1));
    assert_eq!(blockchain["mediantime"], json!(1231006501));
    assert_eq!(mempool["size"], json!(1));
    assert_eq!(mempool["total_fee_sats"], json!(1000));
    assert_eq!(mempool["loaded"], json!(true));
    assert!(!network.to_string().contains("outcome_counters"));
    assert!(!mempool.to_string().contains("outcome_counters"));
}
