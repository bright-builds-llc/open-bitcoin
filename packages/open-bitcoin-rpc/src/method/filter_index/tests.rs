// Parity breadcrumbs:
// - packages/bitcoin-knots/src/rpc/blockchain.cpp
// - packages/bitcoin-knots/src/rpc/node.cpp
// - packages/bitcoin-knots/src/rpc/protocol.h
// - packages/bitcoin-knots/src/rpc/server.cpp
// - packages/bitcoin-knots/src/rpc/util.cpp
// - packages/bitcoin-knots/src/univalue/lib/univalue.cpp
// - packages/bitcoin-knots/src/univalue/lib/univalue_write.cpp
// - packages/bitcoin-knots/src/uint256.h

use serde_json::json;

use super::*;
use crate::error::RpcErrorCode;
use crate::method::RequestParameters;

#[test]
fn phase159_filter_rpc_contract_numeric_errors_roundtrip() {
    for (code, number) in [
        (RpcErrorCode::TypeError, -3),
        (RpcErrorCode::InvalidAddressOrKey, -5),
    ] {
        assert_eq!(code.as_i32(), number);
        assert_eq!(
            serde_json::to_value(code).expect("numeric code"),
            json!(number)
        );
        assert_eq!(
            serde_json::from_value::<RpcErrorCode>(json!(number)).expect("known code"),
            code
        );
    }
}

#[test]
fn phase159_filter_rpc_contract_asymmetric_header_and_filter_projection() {
    // Arrange
    let raw_header = std::array::from_fn(|index| index as u8);
    // Act
    let result = GetBlockFilterResult::from_encoded(&[0xab, 0x01, 0xfe], raw_header);
    // Assert
    assert_eq!(
        serde_json::to_value(result).expect("result JSON"),
        json!({
            "filter": "ab01fe",
            "header": "1f1e1d1c1b1a191817161514131211100f0e0d0c0b0a09080706050403020100"
        })
    );
}

#[test]
fn phase159_filter_rpc_contract_index_projection_has_exact_basic_fields() {
    // Arrange
    let summary = BasicFilterIndexSummary {
        synced: false,
        best_block_height: 17,
    };
    // Act / Assert
    for maybe_index_name in [None, Some(""), Some(BASIC_FILTER_INDEX_NAME)] {
        let request = GetIndexInfoRequest {
            maybe_index_name: maybe_index_name.map(str::to_owned),
        };
        assert_eq!(
            serde_json::to_value(request.project_basic(Some(summary))).expect("index JSON"),
            json!({"basic block filter index": {"synced": false, "best_block_height": 17}})
        );
    }
}

#[test]
fn phase159_filter_rpc_contract_unmatched_or_disabled_index_is_empty_object() {
    for name in [
        "basic",
        "BASIC block filter index",
        "v0 block filter index",
        "txindex",
    ] {
        let request = GetIndexInfoRequest {
            maybe_index_name: Some(name.to_owned()),
        };
        assert_eq!(
            serde_json::to_value(request.project_basic(Some(BasicFilterIndexSummary {
                synced: true,
                best_block_height: 0,
            })))
            .expect("index JSON"),
            json!({})
        );
    }
    assert_eq!(
        serde_json::to_value(
            GetIndexInfoRequest {
                maybe_index_name: None
            }
            .project_basic(None)
        )
        .expect("disabled JSON"),
        json!({})
    );
}

#[test]
fn phase159_filter_rpc_contract_filter_selection_and_disabled_diagnostics() {
    for (name, selected) in [
        ("basic", BlockFilterSelection::Basic),
        ("v0", BlockFilterSelection::V0),
    ] {
        assert_eq!(
            BlockFilterSelection::parse(name).expect("recognized filter"),
            selected
        );
        let detail = selected
            .disabled_failure()
            .maybe_detail
            .expect("disabled diagnostic");
        assert_eq!(detail.code.as_i32(), -1);
        assert_eq!(
            detail.message,
            format!("Index is not enabled for filtertype {name}")
        );
    }
    for name in ["BASIC", "0", "1", "", "V0", "unknown"] {
        let detail = BlockFilterSelection::parse(name)
            .expect_err("unknown filter")
            .maybe_detail
            .expect("diagnostic");
        assert_eq!(detail.code.as_i32(), -5);
        assert_eq!(detail.message, "Unknown filtertype");
    }
}

fn valid_hash() -> String {
    "00112233445566778899aabbccddeeff00112233445566778899aabbccddeeff".to_owned()
}

fn named(values: &[(&str, serde_json::Value)]) -> RequestParameters {
    RequestParameters::Named(
        values
            .iter()
            .map(|(name, value)| ((*name).to_owned(), value.clone()))
            .collect(),
    )
}

fn assert_failure<T: std::fmt::Debug>(
    result: Result<T, crate::error::RpcFailure>,
    code: i32,
    message: &str,
) {
    let detail = result
        .expect_err("request must fail")
        .maybe_detail
        .expect("wire diagnostic");
    assert_eq!(detail.code.as_i32(), code);
    assert_eq!(detail.message, message);
}

#[test]
fn phase159_filter_rpc_normalize_positional_named_mixed_and_args_agree() {
    // Arrange
    let hash = valid_hash();
    let expected = GetBlockFilterRequest {
        block_hash: open_bitcoin_node::core::primitives::BlockHash::from_byte_array([
            0xff, 0xee, 0xdd, 0xcc, 0xbb, 0xaa, 0x99, 0x88, 0x77, 0x66, 0x55, 0x44, 0x33, 0x22,
            0x11, 0, 0xff, 0xee, 0xdd, 0xcc, 0xbb, 0xaa, 0x99, 0x88, 0x77, 0x66, 0x55, 0x44, 0x33,
            0x22, 0x11, 0,
        ]),
        filter_type: BlockFilterSelection::Basic,
    };
    let cases = [
        RequestParameters::Positional(vec![json!(hash)]),
        RequestParameters::Positional(vec![json!(hash.to_uppercase()), json!(null)]),
        named(&[("blockhash", json!(hash)), ("filtertype", json!("basic"))]),
        named(&[("filtertype", json!(null)), ("blockhash", json!(hash))]),
        named(&[("args", json!([hash])), ("filtertype", json!("basic"))]),
        RequestParameters::Mixed {
            positional: vec![json!(hash)],
            named: vec![("filtertype".to_owned(), json!("basic"))],
        },
    ];
    for params in cases {
        // Act / Assert
        assert_eq!(
            normalize_getblockfilter(params).expect("valid request"),
            expected
        );
    }
}

#[test]
fn phase159_filter_rpc_normalize_optional_index_defaults_and_names() {
    for params in [
        RequestParameters::None,
        RequestParameters::Positional(vec![]),
        named(&[]),
        RequestParameters::Positional(vec![json!(null)]),
        named(&[("index_name", json!(null))]),
    ] {
        assert_eq!(
            normalize_getindexinfo(params).expect("default index"),
            GetIndexInfoRequest {
                maybe_index_name: None
            }
        );
    }
    for name in ["", "basic block filter index", "txindex", "BASIC"] {
        for params in [
            RequestParameters::Positional(vec![json!(name)]),
            named(&[("index_name", json!(name))]),
            named(&[("args", json!([name]))]),
        ] {
            assert_eq!(
                normalize_getindexinfo(params).expect("index selector"),
                GetIndexInfoRequest {
                    maybe_index_name: Some(name.to_owned())
                }
            );
        }
    }
}

#[test]
fn phase159_filter_rpc_normalize_duplicate_names_win_before_unknown_and_arity() {
    for name in ["blockhash", "filtertype", "args", "unknown"] {
        assert_failure(
            normalize_getblockfilter(named(&[(name, json!(null)), (name, json!(null))])),
            -8,
            &format!("Parameter {name} specified multiple times"),
        );
    }
    assert_failure(
        normalize_getindexinfo(named(&[("index_name", json!(1)), ("index_name", json!(2))])),
        -8,
        "Parameter index_name specified multiple times",
    );
}

#[test]
fn phase159_filter_rpc_normalize_unknown_names_and_nonarray_args() {
    for value in [
        json!(null),
        json!(1),
        json!(true),
        json!("string"),
        json!({}),
    ] {
        assert_failure(
            normalize_getblockfilter(named(&[("args", value)])),
            -8,
            "Unknown named parameter args",
        );
    }
    assert_failure(
        normalize_getblockfilter(named(&[("BLOCKHASH", json!(valid_hash()))])),
        -8,
        "Unknown named parameter BLOCKHASH",
    );
    assert_failure(
        normalize_getindexinfo(named(&[("index", json!("basic"))])),
        -8,
        "Unknown named parameter index",
    );
}

#[test]
fn phase159_filter_rpc_normalize_mixed_collisions_precede_unknown_and_arity() {
    let hash = valid_hash();
    for params in [
        named(&[
            ("args", json!([hash])),
            ("blockhash", json!(null)),
            ("unknown", json!(1)),
        ]),
        RequestParameters::Mixed {
            positional: vec![json!(hash)],
            named: vec![("blockhash".to_owned(), json!(null))],
        },
    ] {
        assert_failure(
            normalize_getblockfilter(params),
            -8,
            "Parameter blockhash specified twice both as positional and named argument",
        );
    }
    assert_failure(
        normalize_getblockfilter(named(&[
            ("args", json!([hash, "basic", 1])),
            ("filtertype", json!(null)),
        ])),
        -8,
        "Parameter filtertype specified twice both as positional and named argument",
    );
    assert_failure(
        normalize_getindexinfo(named(&[
            ("args", json!(["basic"])),
            ("index_name", json!(null)),
        ])),
        -8,
        "Parameter index_name specified twice both as positional and named argument",
    );
}

#[test]
fn phase159_filter_rpc_normalize_named_hole_becomes_required_null_type_error() {
    assert_failure(
        normalize_getblockfilter(named(&[("filtertype", json!("basic"))])),
        -3,
        "Wrong type passed:\n{\n    \"Position 1 (blockhash)\": \"JSON value of type null is not of expected type string\"\n}",
    );
}

#[test]
fn phase159_filter_rpc_normalize_aggregate_types_in_declaration_order() {
    // Arrange
    let params = named(&[("filtertype", json!([])), ("blockhash", json!(false))]);
    // Act / Assert
    assert_failure(
        normalize_getblockfilter(params),
        -3,
        "Wrong type passed:\n{\n    \"Position 1 (blockhash)\": \"JSON value of type bool is not of expected type string\",\n    \"Position 2 (filtertype)\": \"JSON value of type array is not of expected type string\"\n}",
    );
}

#[test]
fn phase159_filter_rpc_normalize_type_errors_precede_malformed_hash() {
    for (value, type_name) in [
        (json!(1), "number"),
        (json!(true), "bool"),
        (json!([]), "array"),
        (json!({}), "object"),
    ] {
        assert_failure(
            normalize_getblockfilter(RequestParameters::Positional(vec![
                json!("bad"),
                value.clone(),
            ])),
            -3,
            &format!(
                "Wrong type passed:\n{{\n    \"Position 2 (filtertype)\": \"JSON value of type {type_name} is not of expected type string\"\n}}"
            ),
        );
        assert_failure(
            normalize_getindexinfo(RequestParameters::Positional(vec![value])),
            -3,
            &format!(
                "Wrong type passed:\n{{\n    \"Position 1 (index_name)\": \"JSON value of type {type_name} is not of expected type string\"\n}}"
            ),
        );
    }
}

#[test]
fn phase159_filter_rpc_normalize_required_null_is_type_error() {
    assert_failure(
        normalize_getblockfilter(RequestParameters::Positional(vec![json!(null)])),
        -3,
        "Wrong type passed:\n{\n    \"Position 1 (blockhash)\": \"JSON value of type null is not of expected type string\"\n}",
    );
}

#[test]
fn phase159_filter_rpc_normalize_strict_hash_length_before_filter_selection() {
    for hash in [
        "".to_owned(),
        "00".to_owned(),
        "0".repeat(63),
        "0".repeat(65),
        format!(" {}", valid_hash()),
        format!("{} ", valid_hash()),
        format!("0x{}", valid_hash()),
        "é".repeat(32),
    ] {
        assert_failure(
            normalize_getblockfilter(RequestParameters::Positional(vec![
                json!(hash),
                json!("UNKNOWN"),
            ])),
            -8,
            &if hash.len() == 64 {
                format!("blockhash must be hexadecimal string (not '{hash}')")
            } else {
                format!(
                    "blockhash must be of length 64 (not {}, for '{hash}')",
                    hash.len()
                )
            },
        );
    }
}

#[test]
fn phase159_filter_rpc_normalize_strict_hash_nonhex_and_no_trim() {
    for hash in [
        format!("{}g", "0".repeat(63)),
        format!(" {}", "0".repeat(63)),
        format!("0x{}", "0".repeat(62)),
    ] {
        assert_failure(
            normalize_getblockfilter(RequestParameters::Positional(vec![json!(hash)])),
            -8,
            &format!("blockhash must be hexadecimal string (not '{hash}')"),
        );
    }
}

#[test]
fn phase159_filter_rpc_normalize_recognized_v0_and_unknown_filter_names() {
    let request = normalize_getblockfilter(RequestParameters::Positional(vec![
        json!(valid_hash()),
        json!("v0"),
    ]))
    .expect("recognized V0");
    assert_eq!(request.filter_type, BlockFilterSelection::V0);
    for name in ["", "BASIC", "0", "V0"] {
        assert_failure(
            normalize_getblockfilter(RequestParameters::Positional(vec![
                json!(valid_hash()),
                json!(name),
            ])),
            -5,
            "Unknown filtertype",
        );
    }
}

// Source-derived fixture, not output captured from a running Knots node:
// RPCHelpMan::ToString, Sections::ToString and RPCResult::ToSections in util.cpp.
fn help_sections(sections: &[(&str, &str)], removed_max_comma: usize) -> String {
    // Sections retains its maximum width when ToSections pops the final comma.
    let pad = sections
        .iter()
        .map(|(left, _)| left.len())
        .max()
        .expect("nonempty sections")
        + removed_max_comma
        + 4;
    sections
        .iter()
        .map(|(left, right)| {
            if right.is_empty() {
                format!("{left}\n")
            } else {
                format!("{left:pad$}{right}\n")
            }
        })
        .collect()
}

fn help_example(method: &str, args: &str, rpc_args: &str) -> String {
    format!(
        "> bitcoin-cli {method} {args}\n> curl --user myusername --data-binary '{{\"jsonrpc\": \"2.0\", \"id\": \"curltest\", \"method\": \"{method}\", \"params\": [{rpc_args}]}}' -H 'content-type: application/json' http://127.0.0.1:8332/\n"
    )
}

fn source_derived_blockfilter_help() -> String {
    let hash = "00000000c937983704a73af28acdec37b049d214adbda81d7e2a3dd146f6ed09";
    format!(
        "getblockfilter \"blockhash\" ( \"filtertype\" )\n\nRetrieve a BIP 157 content filter for a particular block.\n\nArguments:\n{}\nResult:\n{}\nExamples:\n{}",
        help_sections(
            &[
                ("1. blockhash", "(string, required) The hash of the block"),
                (
                    "2. filtertype",
                    "(string, optional, default=\"basic\") The type name of the filter, values: basic, v0"
                )
            ],
            0
        ),
        help_sections(
            &[
                ("{", "(json object)"),
                (
                    "  \"filter\" : \"hex\",",
                    "(string) the hex-encoded filter data"
                ),
                (
                    "  \"header\" : \"hex\"",
                    "(string) the hex-encoded filter header"
                ),
                ("}", "")
            ],
            0
        ),
        help_example(
            "getblockfilter",
            &format!("\"{hash}\" \"basic\""),
            &format!("\"{hash}\", \"basic\"")
        )
    )
}

fn source_derived_indexinfo_help() -> String {
    format!(
        "getindexinfo ( \"index_name\" )\n\nReturns the status of one or all available indices currently running in the node.\n\nArguments:\n{}\nResult:\n{}\nExamples:\n{}{}",
        help_sections(
            &[(
                "1. index_name",
                "(string, optional) Filter results for an index with a specific name."
            )],
            0
        ),
        help_sections(
            &[
                ("{", "(json object)"),
                ("  \"name\" : {", "(json object) The name of the index"),
                (
                    "    \"synced\" : true|false,",
                    "(boolean) Whether the index is synced or not"
                ),
                (
                    "    \"best_block_height\" : n",
                    "(numeric) The block height to which the index is synced"
                ),
                ("  },", ""),
                ("  ...", ""),
                ("}", "")
            ],
            1
        ),
        help_example("getindexinfo", "", ""),
        help_example("getindexinfo", "txindex", "txindex")
    )
}

#[test]
fn phase159_filter_rpc_normalize_wrong_arity_returns_complete_source_derived_help() {
    let block_help = source_derived_blockfilter_help();
    for params in [
        RequestParameters::None,
        named(&[]),
        RequestParameters::Positional(vec![]),
        RequestParameters::Positional(vec![json!(1), json!(2), json!(3)]),
        named(&[("args", json!([1, 2, 3]))]),
    ] {
        assert_failure(normalize_getblockfilter(params), -1, &block_help);
    }
    let index_help = source_derived_indexinfo_help();
    assert_failure(
        normalize_getindexinfo(RequestParameters::Positional(vec![json!(1), json!(2)])),
        -1,
        &index_help,
    );
    assert_failure(
        normalize_getindexinfo(named(&[("args", json!([1, 2]))])),
        -1,
        &index_help,
    );
}
