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

use super::chain_fixtures::*;
use super::*;

fn spendable_package_context() -> (ManagedRpcContext, String, String) {
    let mut context = empty_context();
    let genesis = build_block(
        BlockHash::from_byte_array([0_u8; 32]),
        0,
        500_000_000,
        p2sh_script(),
    );
    let spendable = build_block(block_hash(&genesis.header), 1, 500_000_000, p2sh_script());
    context.connect_local_block(&genesis).expect("genesis");
    context.connect_local_block(&spendable).expect("spendable");
    let first = spend_transaction(
        transaction_txid(&genesis.transactions[0]).expect("genesis txid"),
        499_999_000,
    );
    let second = spend_transaction(
        transaction_txid(&spendable.transactions[0]).expect("spendable txid"),
        499_999_000,
    );
    let first_hex = encode_hex(
        &encode_transaction(&first, TransactionEncoding::WithWitness).expect("encode first"),
    );
    let second_hex = encode_hex(
        &encode_transaction(&second, TransactionEncoding::WithWitness).expect("encode second"),
    );
    (context, first_hex, second_hex)
}

fn accept_request(raw_txs: Vec<String>) -> TestMempoolAcceptRequest {
    TestMempoolAcceptRequest {
        raw_txs,
        maybe_max_fee_rate: None,
        maybe_max_burn_amount: None,
        ignore_rejects: Vec::new(),
    }
}

fn submit_request(package: Vec<String>) -> SubmitPackageRequest {
    SubmitPackageRequest {
        package,
        maybe_max_fee_rate: None,
        maybe_max_burn_amount: None,
        ignore_rejects: Vec::new(),
    }
}

#[test]
fn testmempoolaccept_empty_array_is_invalid_parameter_minus_eight() {
    // Arrange
    let mut context = empty_context();

    // Act
    let failure = dispatch(
        &mut context,
        MethodCall::TestMempoolAccept(accept_request(Vec::new())),
    )
    .expect_err("empty rawtxs");

    // Assert
    let detail = failure.maybe_detail.expect("invalid parameter detail");
    assert_eq!(detail.code, RpcErrorCode::InvalidParameter);
    assert_eq!(detail.code.as_i32(), -8);
    assert!(
        detail
            .message
            .contains("Array must contain between 1 and 25 transactions"),
        "unexpected message: {}",
        detail.message
    );
}

#[test]
fn testmempoolaccept_twenty_six_is_invalid_parameter_minus_eight() {
    // Arrange
    let mut context = empty_context();
    let raw_txs = vec!["00".to_string(); 26];

    // Act
    let failure = dispatch(
        &mut context,
        MethodCall::TestMempoolAccept(accept_request(raw_txs)),
    )
    .expect_err("26 rawtxs");

    // Assert
    let detail = failure.maybe_detail.expect("invalid parameter detail");
    assert_eq!(detail.code, RpcErrorCode::InvalidParameter);
    assert_eq!(detail.code.as_i32(), -8);
    assert!(
        detail
            .message
            .contains("Array must contain between 1 and 25 transactions"),
        "unexpected message: {}",
        detail.message
    );
}

#[test]
fn testmempoolaccept_bad_hex_is_deserialization_minus_twenty_two() {
    // Arrange
    let mut context = empty_context();

    // Act
    let failure = dispatch(
        &mut context,
        MethodCall::TestMempoolAccept(accept_request(vec!["zz".to_string()])),
    )
    .expect_err("bad hex");

    // Assert
    let detail = failure.maybe_detail.expect("deserialization detail");
    assert_eq!(detail.code, RpcErrorCode::DeserializationError);
    assert_eq!(detail.code.as_i32(), -22);
}

#[test]
fn testmempoolaccept_dry_run_does_not_change_getmempoolinfo_size() {
    // Arrange
    let (mut context, transaction_hex, _) = spendable_package_context();
    let before = dispatch(
        &mut context,
        MethodCall::GetMempoolInfo(GetMempoolInfoRequest::default()),
    )
    .expect("mempool before");
    let size_before = before["size"].as_u64().expect("size before");

    // Act
    let accepted = dispatch(
        &mut context,
        MethodCall::TestMempoolAccept(accept_request(vec![transaction_hex])),
    )
    .expect("dry-run");
    let after = dispatch(
        &mut context,
        MethodCall::GetMempoolInfo(GetMempoolInfoRequest::default()),
    )
    .expect("mempool after");

    // Assert
    assert_eq!(after["size"], json!(size_before));
    let first = accepted
        .as_array()
        .and_then(|rows| rows.first())
        .expect("testmempoolaccept array");
    assert_eq!(first["allowed"], json!(true));
}

#[test]
fn submitpackage_non_child_with_parents_is_verify_error_minus_twenty_five() {
    // Arrange
    let (mut context, first_hex, second_hex) = spendable_package_context();

    // Act
    let failure = dispatch(
        &mut context,
        MethodCall::SubmitPackage(submit_request(vec![first_hex, second_hex])),
    )
    .expect_err("unrelated parents are not child-with-unconfirmed-parents");

    // Assert
    let detail = failure.maybe_detail.expect("verify error detail");
    assert_eq!(detail.code, RpcErrorCode::VerifyError);
    assert_eq!(detail.code.as_i32(), -25);
}

#[test]
fn submitpackage_success_is_knots_object_without_open_bitcoin_keys() {
    // Arrange
    let (mut context, transaction_hex, _) = spendable_package_context();

    // Act
    let success = dispatch(
        &mut context,
        MethodCall::SubmitPackage(submit_request(vec![transaction_hex])),
    )
    .expect("submitpackage");

    // Assert
    let object = success.as_object().expect("submitpackage object");
    assert!(object.contains_key("package_msg"));
    assert!(object.contains_key("tx-results"));
    for forbidden in [
        "fingerprint",
        "admission",
        "relay_disabled",
        "effective_fee_groups",
        "openbitcoin",
    ] {
        assert!(
            !object.contains_key(forbidden),
            "Knots submitpackage must omit {forbidden}"
        );
    }
}

#[test]
fn testmempoolaccept_policy_reject_is_result_body_not_rpc_error() {
    // Arrange
    let mut context = empty_context();
    let missing = spend_transaction(Txid::from_byte_array([9_u8; 32]), 1_000);
    let missing_hex = encode_hex(
        &encode_transaction(&missing, TransactionEncoding::WithWitness).expect("encode missing"),
    );

    // Act
    let result = dispatch(
        &mut context,
        MethodCall::TestMempoolAccept(accept_request(vec![missing_hex])),
    );

    // Assert
    let accepted = result.expect("policy reject stays in the result body");
    let first = accepted
        .as_array()
        .and_then(|rows| rows.first())
        .expect("testmempoolaccept array");
    assert_eq!(first["allowed"], json!(false));
    assert!(first.get("reject-reason").is_some() || first.get("error").is_some());
}

#[test]
fn package_methods_fail_closed_on_explicit_maxfeerate() {
    // Arrange
    let mut context = empty_context();
    let accept = TestMempoolAcceptRequest {
        raw_txs: vec!["00".to_string()],
        maybe_max_fee_rate: Some(1),
        maybe_max_burn_amount: None,
        ignore_rejects: Vec::new(),
    };
    let submit = SubmitPackageRequest {
        package: vec!["00".to_string()],
        maybe_max_fee_rate: Some(1),
        maybe_max_burn_amount: None,
        ignore_rejects: Vec::new(),
    };

    // Act
    let accept_failure = dispatch(&mut context, MethodCall::TestMempoolAccept(accept))
        .expect_err("explicit maxfeerate");
    let submit_failure =
        dispatch(&mut context, MethodCall::SubmitPackage(submit)).expect_err("explicit maxfeerate");

    // Assert
    for failure in [accept_failure, submit_failure] {
        let detail = failure.maybe_detail.expect("unsupported option detail");
        let code = detail.code.as_i32();
        assert!(
            code == -8 || code == -32602,
            "expected -8 or -32602, got {code}"
        );
        assert!(
            detail.message.contains("maxfeerate"),
            "unexpected message: {}",
            detail.message
        );
    }
}
