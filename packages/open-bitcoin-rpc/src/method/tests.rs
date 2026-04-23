use serde_json::json;

use crate::RpcFailure;

use super::{MethodOrigin, RequestParameters, SupportedMethod, normalize_method_call};

#[test]
fn supported_http_methods_match_phase_8_surface() {
    // Arrange
    let expected = [
        "getblockchaininfo",
        "getmempoolinfo",
        "getnetworkinfo",
        "sendrawtransaction",
        "deriveaddresses",
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
fn ranged_descriptors_and_deferred_methods_fail_explicitly() {
    // Arrange
    let ranged_request = RequestParameters::Positional(vec![
        json!("wpkh(cMec2DGaTXkYJYfi7x3ZGjRXkeqmAvYAoWzMAcWj5fdLaqudWsNi)#8fhd9pwu"),
        json!({"start": 0, "end": 1}),
    ]);

    // Act
    let ranged_error =
        normalize_method_call("deriveaddresses", ranged_request).expect_err("range is deferred");
    let deferred_error =
        normalize_method_call("sendtoaddress", RequestParameters::None).expect_err("deferred");

    // Assert
    assert_eq!(
        ranged_error,
        RpcFailure::invalid_params("ranged descriptors are deferred to later wallet phases"),
    );
    assert_eq!(
        deferred_error,
        RpcFailure::method_not_found("sendtoaddress"),
    );
}

#[test]
fn named_params_normalize_and_reject_duplicate_or_colliding_keys() {
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
        RpcFailure::invalid_params(
            "named parameter descriptor was provided multiple times or collides with a positional argument",
        ),
    );
    assert_eq!(
        collision_error,
        RpcFailure::invalid_params(
            "named parameter descriptor was provided multiple times or collides with a positional argument",
        ),
    );
}
