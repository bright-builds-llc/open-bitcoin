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

use crate::{
    RpcFailure, RpcFailureKind,
    error::RpcErrorCode,
    method::{
        OpenBitcoinPackageMode, OpenBitcoinPackageRequest, SubmitPackageRequest,
        TestMempoolAcceptRequest,
    },
};

use super::{
    MethodCall, MethodOrigin, MethodScope, RequestParameters, SupportedMethod,
    normalize_method_call,
};

#[test]
fn supported_http_methods_match_phase_20_wallet_surface() {
    // Arrange
    let expected = [
        "getblockchaininfo",
        "getmempoolinfo",
        "getnetworkinfo",
        "openbitcoinnetworkstatus",
        "openbitcoinsyncstatus",
        "openbitcoinsyncpause",
        "openbitcoinsyncresume",
        "sendrawtransaction",
        "testmempoolaccept",
        "submitpackage",
        "openbitcoinpackage",
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
    let expected = [
        "openbitcoinnetworkstatus",
        "openbitcoinsyncstatus",
        "openbitcoinsyncpause",
        "openbitcoinsyncresume",
        "openbitcoinpackage",
        "buildtransaction",
        "buildandsigntransaction",
    ];

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
fn network_status_method_is_open_bitcoin_node_extension() {
    // Arrange
    let method = SupportedMethod::OpenBitcoinNetworkStatus;

    // Act
    let call =
        normalize_method_call("openbitcoinnetworkstatus", RequestParameters::None).expect("status");
    let positional_error = normalize_method_call(
        "openbitcoinnetworkstatus",
        RequestParameters::Positional(vec![json!("unexpected")]),
    )
    .expect_err("positional parameters should fail");
    let named_error = normalize_method_call(
        "openbitcoinnetworkstatus",
        RequestParameters::Named(vec![("unexpected".to_string(), json!(true))]),
    )
    .expect_err("named parameters should fail");

    // Assert
    assert_eq!(method.origin(), MethodOrigin::OpenBitcoinExtension);
    assert_eq!(method.scope(), MethodScope::Node);
    assert!(matches!(call, MethodCall::OpenBitcoinNetworkStatus(_)));
    assert_eq!(call.scope(), MethodScope::Node);
    assert_eq!(
        positional_error,
        RpcFailure::invalid_params("too many positional parameters: expected at most 0, got 1"),
    );
    assert_eq!(
        named_error,
        RpcFailure::invalid_params("unknown named parameter unexpected"),
    );
}

#[test]
fn sync_control_methods_are_open_bitcoin_node_extensions() {
    // Arrange
    let methods = [
        (
            "openbitcoinsyncstatus",
            SupportedMethod::OpenBitcoinSyncStatus,
        ),
        (
            "openbitcoinsyncpause",
            SupportedMethod::OpenBitcoinSyncPause,
        ),
        (
            "openbitcoinsyncresume",
            SupportedMethod::OpenBitcoinSyncResume,
        ),
    ];

    // Act / Assert
    for (name, supported_method) in methods {
        assert_eq!(
            supported_method.origin(),
            MethodOrigin::OpenBitcoinExtension
        );
        assert_eq!(supported_method.scope(), MethodScope::Node);
        let call = normalize_method_call(name, RequestParameters::None).expect("sync method");
        assert_eq!(call.scope(), MethodScope::Node);
        match (name, call) {
            ("openbitcoinsyncstatus", MethodCall::OpenBitcoinSyncStatus(_))
            | ("openbitcoinsyncpause", MethodCall::OpenBitcoinSyncPause(_))
            | ("openbitcoinsyncresume", MethodCall::OpenBitcoinSyncResume(_)) => {}
            (other_name, other_call) => {
                panic!("unexpected normalized call for {other_name}: {other_call:?}")
            }
        }

        let error = normalize_method_call(
            name,
            RequestParameters::Positional(vec![json!("unexpected")]),
        )
        .expect_err("extra sync parameters should fail");
        assert_eq!(
            error,
            RpcFailure::invalid_params("too many positional parameters: expected at most 0, got 1"),
        );
    }
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

#[test]
fn rpc_error_codes_include_knots_minus_eight_twenty_two_twenty_five() {
    // Arrange
    let codes = [
        RpcErrorCode::InvalidParameter,
        RpcErrorCode::DeserializationError,
        RpcErrorCode::VerifyError,
    ];

    // Act
    let as_i32 = codes.map(RpcErrorCode::as_i32);
    let invalid_parameter = RpcFailure::invalid_parameter("count");
    let deserialization = RpcFailure::deserialization_error("hex");
    let verify = RpcFailure::verify_error("topology");

    // Assert
    assert_eq!(as_i32, [-8, -22, -25]);
    assert_eq!(
        RpcErrorCode::try_from(-8),
        Ok(RpcErrorCode::InvalidParameter)
    );
    assert_eq!(
        RpcErrorCode::try_from(-22),
        Ok(RpcErrorCode::DeserializationError)
    );
    assert_eq!(RpcErrorCode::try_from(-25), Ok(RpcErrorCode::VerifyError));
    assert_eq!(invalid_parameter.kind, RpcFailureKind::InvalidParams);
    assert_eq!(
        invalid_parameter
            .maybe_detail
            .as_ref()
            .map(|detail| detail.code.as_i32()),
        Some(-8)
    );
    assert_eq!(deserialization.kind, RpcFailureKind::InvalidParams);
    assert_eq!(
        deserialization
            .maybe_detail
            .as_ref()
            .map(|detail| detail.code.as_i32()),
        Some(-22)
    );
    assert_eq!(verify.kind, RpcFailureKind::InvalidParams);
    assert_eq!(
        verify
            .maybe_detail
            .as_ref()
            .map(|detail| detail.code.as_i32()),
        Some(-25)
    );
}

#[test]
fn testmempoolaccept_is_baseline_parity_node_method() {
    // Arrange
    let maybe_method = SupportedMethod::from_name("testmempoolaccept");

    // Act
    let method = maybe_method.expect("testmempoolaccept should be registered");
    let call = normalize_method_call(
        "testmempoolaccept",
        RequestParameters::Named(vec![("rawtxs".to_string(), json!(["00"]))]),
    )
    .expect("normalize testmempoolaccept");

    // Assert
    assert_eq!(method, SupportedMethod::TestMempoolAccept);
    assert_eq!(method.origin(), MethodOrigin::BaselineParity);
    assert_eq!(method.scope(), MethodScope::Node);
    assert_eq!(method.name(), "testmempoolaccept");
    assert_eq!(call.scope(), MethodScope::Node);
    assert!(matches!(call, MethodCall::TestMempoolAccept(_)));
    assert!(
        SupportedMethod::all().contains(&SupportedMethod::TestMempoolAccept),
        "all() should include TestMempoolAccept"
    );
}

#[test]
fn submitpackage_is_baseline_parity_node_method() {
    // Arrange
    let maybe_method = SupportedMethod::from_name("submitpackage");

    // Act
    let method = maybe_method.expect("submitpackage should be registered");
    let call = normalize_method_call(
        "submitpackage",
        RequestParameters::Named(vec![("package".to_string(), json!(["00"]))]),
    )
    .expect("normalize submitpackage");

    // Assert
    assert_eq!(method, SupportedMethod::SubmitPackage);
    assert_eq!(method.origin(), MethodOrigin::BaselineParity);
    assert_eq!(method.scope(), MethodScope::Node);
    assert_eq!(method.name(), "submitpackage");
    assert_eq!(call.scope(), MethodScope::Node);
    assert!(matches!(call, MethodCall::SubmitPackage(_)));
    assert!(
        SupportedMethod::all().contains(&SupportedMethod::SubmitPackage),
        "all() should include SubmitPackage"
    );
}

#[test]
fn package_requests_deny_unknown_fields() {
    // Arrange
    let accept_json = json!({
        "rawtxs": ["00"],
        "maxfeerate": 1,
        "maxburnamount": 0,
        "ignore_rejects": ["txn-mempool-conflict"]
    });
    let submit_json = json!({
        "package": ["00"],
        "maxfeerate": 1,
        "maxburnamount": 0,
        "ignore_rejects": []
    });
    let unknown_accept = json!({
        "rawtxs": ["00"],
        "extra": true
    });
    let unknown_submit = json!({
        "package": ["00"],
        "extra": true
    });

    // Act
    let accept = serde_json::from_value::<TestMempoolAcceptRequest>(accept_json);
    let submit = serde_json::from_value::<SubmitPackageRequest>(submit_json);
    let unknown_accept = serde_json::from_value::<TestMempoolAcceptRequest>(unknown_accept);
    let unknown_submit = serde_json::from_value::<SubmitPackageRequest>(unknown_submit);

    // Assert
    let accept = accept.expect("known testmempoolaccept fields should deserialize");
    let submit = submit.expect("known submitpackage fields should deserialize");
    assert_eq!(accept.raw_txs, vec!["00".to_string()]);
    assert_eq!(accept.maybe_max_fee_rate, Some(1));
    assert_eq!(accept.maybe_max_burn_amount, Some(0));
    assert_eq!(
        accept.ignore_rejects,
        vec!["txn-mempool-conflict".to_string()]
    );
    assert_eq!(submit.package, vec!["00".to_string()]);
    assert_eq!(submit.maybe_max_fee_rate, Some(1));
    assert_eq!(submit.maybe_max_burn_amount, Some(0));
    assert!(submit.ignore_rejects.is_empty());
    assert!(unknown_accept.is_err(), "unknown testmempoolaccept field");
    assert!(unknown_submit.is_err(), "unknown submitpackage field");
}

#[test]
fn openbitcoinpackage_is_open_bitcoin_extension() {
    // Arrange
    let maybe_method = SupportedMethod::from_name("openbitcoinpackage");

    // Act
    let method = maybe_method.expect("openbitcoinpackage should be registered");
    let call = normalize_method_call(
        "openbitcoinpackage",
        RequestParameters::Named(vec![
            ("mode".to_string(), json!("dry-run")),
            ("rawtxs".to_string(), json!(["00"])),
        ]),
    )
    .expect("normalize openbitcoinpackage");
    let missing_mode = serde_json::from_value::<OpenBitcoinPackageRequest>(json!({
        "rawtxs": ["00"]
    }));

    // Assert
    assert_eq!(method, SupportedMethod::OpenBitcoinPackage);
    assert_eq!(method.origin(), MethodOrigin::OpenBitcoinExtension);
    assert_eq!(method.scope(), MethodScope::Node);
    assert_eq!(method.name(), "openbitcoinpackage");
    assert_eq!(call.scope(), MethodScope::Node);
    assert!(matches!(call, MethodCall::OpenBitcoinPackage(_)));
    assert!(
        SupportedMethod::all().contains(&SupportedMethod::OpenBitcoinPackage),
        "all() should include OpenBitcoinPackage"
    );
    assert!(missing_mode.is_err(), "mode is required");
}

#[test]
fn openbitcoinpackage_unknown_field_is_rejected() {
    // Arrange
    let known = json!({
        "mode": "submit",
        "rawtxs": ["00"]
    });
    let unknown = json!({
        "mode": "dry-run",
        "rawtxs": ["00"],
        "extra": true
    });

    // Act
    let accepted = serde_json::from_value::<OpenBitcoinPackageRequest>(known);
    let rejected = serde_json::from_value::<OpenBitcoinPackageRequest>(unknown);

    // Assert
    let accepted = accepted.expect("known openbitcoinpackage fields should deserialize");
    assert_eq!(accepted.mode, OpenBitcoinPackageMode::Submit);
    assert_eq!(accepted.raw_txs, vec!["00".to_string()]);
    assert!(rejected.is_err(), "unknown openbitcoinpackage field");
}
