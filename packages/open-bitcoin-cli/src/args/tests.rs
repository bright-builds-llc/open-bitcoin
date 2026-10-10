// Parity breadcrumbs:
// - packages/bitcoin-knots/src/bitcoin-cli.cpp
// - packages/bitcoin-knots/src/common/args.cpp
// - packages/bitcoin-knots/src/common/config.cpp
// - packages/bitcoin-knots/test/functional/interface_bitcoin_cli.py

use std::ffi::OsString;

use serde_json::json;

use super::{CliCommand, parse_cli_args, parse_named_parameters, stdin_required_for_args};
use open_bitcoin_rpc::method::RequestParameters;

fn os(value: &str) -> OsString {
    OsString::from(value)
}

#[test]
fn stdinrpcpass_is_consumed_before_stdin_arguments() {
    // Arrange
    let cli_args = vec![
        os("-stdin"),
        os("-stdinrpcpass"),
        os("sendrawtransaction"),
        os("deadbeef"),
    ];

    // Act
    let parsed = parse_cli_args(&cli_args, "secret\n0\n0\n[\"ignore\"]\n").expect("stdin parsing");

    // Assert
    assert_eq!(parsed.startup.maybe_rpc_password.as_deref(), Some("secret"));
    assert_eq!(
        parsed.command,
        CliCommand::RpcMethod(super::RpcMethodCommand {
            method: "sendrawtransaction".to_string(),
            params: RequestParameters::Positional(vec![
                json!("deadbeef"),
                json!(0),
                json!(0),
                json!(["ignore"]),
            ]),
        })
    );
}

#[test]
fn invalid_rpc_ports_fail_before_request_dispatch() {
    // Arrange
    let invalid_rpcconnect_args = vec![os("-rpcconnect=127.0.0.1:notaport"), os("getnetworkinfo")];
    let invalid_rpcport_args = vec![os("-rpcport=notaport"), os("getnetworkinfo")];

    // Act
    let rpcconnect_error =
        parse_cli_args(&invalid_rpcconnect_args, "").expect_err("invalid rpcconnect must fail");
    let rpcport_error =
        parse_cli_args(&invalid_rpcport_args, "").expect_err("invalid rpcport must fail");

    // Assert
    assert_eq!(
        rpcconnect_error.to_string(),
        "Invalid port provided in -rpcconnect: 127.0.0.1:notaport",
    );
    assert_eq!(
        rpcport_error.to_string(),
        "Invalid port provided in -rpcport: notaport",
    );
}

#[test]
fn stdin_requirement_detection_matches_stdin_flags() {
    // Arrange
    let no_stdin_args = vec![os("getnetworkinfo")];
    let stdin_args = vec![os("-stdin"), os("sendrawtransaction")];
    let disabled_stdin_args = vec![os("-stdin=0"), os("getnetworkinfo")];
    let stdinrpcpass_args = vec![os("-stdinrpcpass"), os("getnetworkinfo")];
    let negated_then_enabled_args = vec![os("-nostdin"), os("-stdin=1"), os("getnetworkinfo")];

    // Act
    let no_stdin_required = stdin_required_for_args(&no_stdin_args);
    let stdin_required = stdin_required_for_args(&stdin_args);
    let disabled_stdin_required = stdin_required_for_args(&disabled_stdin_args);
    let stdinrpcpass_required = stdin_required_for_args(&stdinrpcpass_args);
    let negated_then_enabled_required = stdin_required_for_args(&negated_then_enabled_args);

    // Assert
    assert!(!no_stdin_required);
    assert!(stdin_required);
    assert!(!disabled_stdin_required);
    assert!(stdinrpcpass_required);
    assert!(negated_then_enabled_required);
}

#[test]
fn named_arguments_reject_duplicate_keys_before_transport() {
    // Arrange
    let descriptor_a = "wpkh(cMec2DGaTXkYJYfi7x3ZGjRXkeqmAvYAoWzMAcWj5fdLaqudWsNi)#8fhd9pwu";
    let descriptor_b = "wpkh(cTe1f5rdT8A8DFgVWTjyPwACsDPJM9ff4QngFxUixCSvvbg1x6sh)#8fhd9pwu";
    let duplicate_args = vec![
        os("-named"),
        os("deriveaddresses"),
        os(&format!("descriptor={descriptor_a:?}")),
        os(&format!("descriptor={descriptor_b:?}")),
    ];
    let raw_params = vec![
        format!("descriptor={descriptor_a:?}"),
        format!("descriptor={descriptor_b:?}"),
    ];

    // Act
    let preserved_params = parse_named_parameters(&raw_params).expect("preserved parameters");
    let duplicate_error =
        parse_cli_args(&duplicate_args, "").expect_err("duplicate named key must fail");

    // Assert
    assert_eq!(
        preserved_params,
        RequestParameters::Named(vec![
            ("descriptor".to_string(), json!(descriptor_a)),
            ("descriptor".to_string(), json!(descriptor_b)),
        ]),
    );
    assert_eq!(
        duplicate_error.to_string(),
        "Parameter descriptor specified multiple times",
    );
}

#[test]
fn named_arguments_reject_positional_collisions() {
    // Arrange
    let descriptor = "wpkh(cMec2DGaTXkYJYfi7x3ZGjRXkeqmAvYAoWzMAcWj5fdLaqudWsNi)#8fhd9pwu";
    let collision_args = vec![
        os("-named"),
        os("deriveaddresses"),
        os(descriptor),
        os(&format!("descriptor={descriptor}")),
    ];
    let repeated_args_payload = vec![
        os("-named"),
        os("sendrawtransaction"),
        os("args=[\"deadbeef\"]"),
        os("0"),
    ];

    // Act
    let collision_error =
        parse_cli_args(&collision_args, "").expect_err("mixed positional and named must fail");
    let repeated_args_error =
        parse_cli_args(&repeated_args_payload, "").expect_err("args= collision must fail");

    // Assert
    assert_eq!(
        collision_error.to_string(),
        "Parameter descriptor specified twice both as positional and named argument",
    );
    assert_eq!(
        repeated_args_error.to_string(),
        "Parameter args specified multiple times",
    );
}

#[test]
fn deferred_netinfo_and_supported_rpcwallet_behave_as_documented() {
    // Arrange
    let netinfo_args = vec![os("-netinfo")];
    let rpcwallet_args = vec![os("-rpcwallet=wallet.dat"), os("getwalletinfo")];

    // Act
    let netinfo_error = parse_cli_args(&netinfo_args, "").expect_err("netinfo is deferred");
    let rpcwallet = parse_cli_args(&rpcwallet_args, "").expect("rpcwallet is supported");

    // Assert
    assert_eq!(
        netinfo_error.to_string(),
        "-netinfo is deferred until the getpeerinfo-backed network dashboard lands in a later Phase 8 plan.",
    );
    assert_eq!(
        rpcwallet.startup.maybe_rpc_wallet.as_deref(),
        Some("wallet.dat"),
    );
    assert_eq!(
        rpcwallet.command,
        CliCommand::RpcMethod(super::RpcMethodCommand {
            method: "getwalletinfo".to_string(),
            params: RequestParameters::None,
        }),
    );
}

#[test]
fn phase159_filter_rpc_cli_args_accept_positional_named_and_v0() {
    // Arrange
    use open_bitcoin_rpc::method::{BlockFilterSelection, MethodCall, normalize_method_call};
    let hash = "00112233445566778899aabbccddeeff00112233445566778899aabbccddeeff";
    let cases = [
        (
            vec![os("getblockfilter"), os(hash)],
            BlockFilterSelection::Basic,
        ),
        (
            vec![os("getblockfilter"), os(hash), os("null")],
            BlockFilterSelection::Basic,
        ),
        (
            vec![
                os("-named"),
                os("getblockfilter"),
                os(&format!("blockhash={hash}")),
                os("filtertype=v0"),
            ],
            BlockFilterSelection::V0,
        ),
        (
            vec![
                os("-named"),
                os("getblockfilter"),
                os(hash),
                os("filtertype=basic"),
            ],
            BlockFilterSelection::Basic,
        ),
    ];
    for (args, expected_filter) in cases {
        // Act
        let parsed = parse_cli_args(&args, "").expect("valid filter arguments");
        let CliCommand::RpcMethod(command) = parsed.command else {
            panic!("RPC method expected")
        };
        let MethodCall::GetBlockFilter(request) =
            normalize_method_call(&command.method, command.params).expect("typed filter call")
        else {
            panic!("getblockfilter expected")
        };
        // Assert
        assert_eq!(request.filter_type, expected_filter);
        assert_eq!(request.block_hash.as_bytes()[0], 0xff);
        assert_eq!(request.block_hash.as_bytes()[31], 0);
    }
}

#[test]
fn phase159_filter_rpc_cli_args_preserve_index_selectors() {
    // Arrange
    let mut cases = vec![(vec![os("getindexinfo")], RequestParameters::None)];
    for name in [
        "",
        "basic block filter index",
        "txindex",
        "v0 block filter index",
    ] {
        cases.push((
            vec![os("getindexinfo"), os(name)],
            RequestParameters::Positional(vec![json!(name)]),
        ));
        cases.push((
            vec![
                os("-named"),
                os("getindexinfo"),
                os(&format!("index_name={name}")),
            ],
            RequestParameters::Named(vec![("index_name".to_owned(), json!(name))]),
        ));
    }
    for (args, expected_params) in cases {
        // Act
        let parsed = parse_cli_args(&args, "").expect("valid index selector");
        // Assert
        assert_eq!(
            parsed.command,
            CliCommand::RpcMethod(super::RpcMethodCommand {
                method: "getindexinfo".to_owned(),
                params: expected_params
            })
        );
    }
}

#[test]
fn phase159_filter_rpc_cli_args_reject_filter_parameter_errors() {
    // Arrange
    let hash = "00112233445566778899aabbccddeeff00112233445566778899aabbccddeeff";
    let cases = [
        (
            vec![
                os("-named"),
                os("getblockfilter"),
                os(&format!("blockhash={hash}")),
                os(&format!("blockhash={hash}")),
            ],
            "Parameter blockhash specified multiple times",
        ),
        (
            vec![
                os("-named"),
                os("getblockfilter"),
                os(hash),
                os(&format!("blockhash={hash}")),
            ],
            "Parameter blockhash specified twice both as positional and named argument",
        ),
        (
            vec![os("getblockfilter"), os("bad")],
            "blockhash must be of length 64 (not 3, for 'bad')",
        ),
        (
            vec![os("getblockfilter"), os(hash), os("BASIC")],
            "Unknown filtertype",
        ),
        (
            vec![os("-named"), os("getindexinfo"), os("name=basic")],
            "Unknown named parameter name",
        ),
    ];
    for (args, expected_error) in cases {
        // Act / Assert
        assert_eq!(
            parse_cli_args(&args, "")
                .expect_err("invalid arguments")
                .to_string(),
            expected_error
        );
    }
}
