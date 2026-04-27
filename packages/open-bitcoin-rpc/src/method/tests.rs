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

use serde_json::json;

use crate::RpcFailure;

use super::{MethodOrigin, RequestParameters, SupportedMethod, normalize_method_call};

#[test]
fn supported_http_methods_match_phase_20_wallet_surface() {
    // Arrange
    let expected = [
        "getblockchaininfo",
        "getmempoolinfo",
        "getnetworkinfo",
        "sendrawtransaction",
        "deriveaddresses",
        "sendtoaddress",
        "getnewaddress",
        "getrawchangeaddress",
        "listdescriptors",
        "getwalletinfo",
        "getbalances",
        "listunspent",
        "importdescriptors",
        "rescanblockchain",
        "buildtransaction",
        "buildandsigntransaction",
    ];

    // Act
    let names: Vec<_> = SupportedMethod::all()
        .iter()
        .map(|method| method.name())
        .collect();

    // Assert
    assert_eq!(names, expected);
}

#[test]
fn build_transaction_methods_are_marked_as_open_bitcoin_extensions() {
    // Arrange
    let expected = ["buildtransaction", "buildandsigntransaction"];

    // Act
    let names: Vec<_> = SupportedMethod::all()
        .iter()
        .filter(|method| method.origin() == MethodOrigin::OpenBitcoinExtension)
        .map(|method| method.name())
        .collect();

    // Assert
    assert_eq!(names, expected);
}

#[test]
fn ranged_descriptors_and_deferred_lifecycle_methods_fail_explicitly() {
    // Arrange
    let ranged_request = RequestParameters::Positional(vec![
        json!("wpkh(cMec2DGaTXkYJYfi7x3ZGjRXkeqmAvYAoWzMAcWj5fdLaqudWsNi)#8fhd9pwu"),
        json!({"start": 0, "end": 1}),
    ]);

    // Act
    let ranged_error =
        normalize_method_call("deriveaddresses", ranged_request).expect_err("range is deferred");
    let deferred_error =
        normalize_method_call("loadwallet", RequestParameters::None).expect_err("deferred");

    // Assert
    assert_eq!(
        ranged_error,
        RpcFailure::invalid_params("ranged descriptors are deferred to later wallet phases"),
    );
    assert_eq!(deferred_error, RpcFailure::method_not_found("loadwallet"),);
}

#[test]
fn sendtoaddress_accepts_estimate_inputs_and_wallet_address_methods() {
    // Arrange
    let send_request = RequestParameters::Named(vec![
        (
            "address".to_string(),
            json!("bcrt1qa0qwuze2h85zw7nqpsj3ga0z9geyrgwpf2m8je"),
        ),
        ("amount_sats".to_string(), json!(25_000)),
        ("conf_target".to_string(), json!(3)),
        ("estimate_mode".to_string(), json!("economical")),
    ]);

    // Act
    let send = normalize_method_call("sendtoaddress", send_request).expect("sendtoaddress");
    let get_new_address =
        normalize_method_call("getnewaddress", RequestParameters::None).expect("getnewaddress");
    let get_raw_change_address =
        normalize_method_call("getrawchangeaddress", RequestParameters::None)
            .expect("getrawchangeaddress");
    let list_descriptors =
        normalize_method_call("listdescriptors", RequestParameters::None).expect("listdescriptors");

    // Assert
    match send {
        super::MethodCall::SendToAddress(request) => {
            assert_eq!(request.amount_sats, 25_000);
            assert_eq!(request.maybe_conf_target, Some(3));
            assert_eq!(
                request.maybe_estimate_mode,
                Some(super::EstimateMode::Economical)
            );
            assert!(request.maybe_fee_rate_sat_per_kvb.is_none());
        }
        other => panic!("expected sendtoaddress, got {other:?}"),
    }
    assert!(matches!(
        get_new_address,
        super::MethodCall::GetNewAddress(_)
    ));
    assert!(matches!(
        get_raw_change_address,
        super::MethodCall::GetRawChangeAddress(_)
    ));
    assert!(matches!(
        list_descriptors,
        super::MethodCall::ListDescriptors(_)
    ));
}

#[test]
fn named_params_distinguish_duplicate_keys_from_positional_collisions() {
    // Arrange
    let named_request = RequestParameters::Named(vec![(
        "descriptor".to_string(),
        json!("wpkh(cMec2DGaTXkYJYfi7x3ZGjRXkeqmAvYAoWzMAcWj5fdLaqudWsNi)#8fhd9pwu"),
    )]);
    let duplicate_request = RequestParameters::Named(vec![
        (
            "descriptor".to_string(),
            json!("wpkh(cMec2DGaTXkYJYfi7x3ZGjRXkeqmAvYAoWzMAcWj5fdLaqudWsNi)#8fhd9pwu"),
        ),
        (
            "descriptor".to_string(),
            json!("wpkh(cTe1f5rdT8A8DFgVWTjyPwACsDPJM9ff4QngFxUixCSvvbg1x6sh)#8fhd9pwu"),
        ),
    ]);
    let collision_request = RequestParameters::Mixed {
        positional: vec![json!(
            "wpkh(cMec2DGaTXkYJYfi7x3ZGjRXkeqmAvYAoWzMAcWj5fdLaqudWsNi)#8fhd9pwu"
        )],
        named: vec![(
            "descriptor".to_string(),
            json!("wpkh(cTe1f5rdT8A8DFgVWTjyPwACsDPJM9ff4QngFxUixCSvvbg1x6sh)#8fhd9pwu"),
        )],
    };

    // Act
    let normalized = normalize_method_call("deriveaddresses", named_request);
    let duplicate_error =
        normalize_method_call("deriveaddresses", duplicate_request).expect_err("duplicate");
    let collision_error =
        normalize_method_call("deriveaddresses", collision_request).expect_err("collision");

    // Assert
    assert!(normalized.is_ok());
    assert_eq!(
        duplicate_error,
        RpcFailure::invalid_params("named parameter descriptor was provided multiple times"),
    );
    assert_eq!(
        collision_error,
        RpcFailure::invalid_params(
            "named parameter descriptor collides with a positional argument"
        ),
    );
}
