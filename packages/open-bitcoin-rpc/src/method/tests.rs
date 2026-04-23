use super::{MethodOrigin, SupportedMethod};

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
