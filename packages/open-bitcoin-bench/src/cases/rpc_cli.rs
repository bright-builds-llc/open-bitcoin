// Parity breadcrumbs:
// - packages/bitcoin-knots/src/bench/rpc_mempool.cpp
// - packages/bitcoin-knots/src/bitcoin-cli.cpp
// - packages/bitcoin-knots/src/rpc/server.cpp

use std::ffi::OsString;

use open_bitcoin_cli::args::{CliCommand, parse_cli_args};
use open_bitcoin_rpc::{
    ManagedRpcContext, RpcFailure,
    dispatch::dispatch,
    method::{RequestParameters, normalize_method_call},
};
use open_bitcoin_wallet::AddressNetwork;

use crate::{
    error::BenchError,
    registry::{BenchCase, BenchDurability, BenchGroupId, BenchMeasurement, RPC_CLI_MAPPING},
};

pub const CASES: [BenchCase; 1] = [BenchCase {
    id: "rpc-cli.parse-normalize-dispatch",
    group: BenchGroupId::RpcCli,
    description: "Parse a CLI RPC command, normalize it, and dispatch against the in-memory RPC context.",
    measurement: BenchMeasurement {
        focus: "rpc_cli_parse_normalize_dispatch",
        fixture: "in_memory_rpc_context",
        durability: BenchDurability::Ephemeral,
    },
    knots_mapping: &RPC_CLI_MAPPING,
    run_once: run_rpc_cli_case,
}];

fn run_rpc_cli_case() -> Result<(), BenchError> {
    let cli_args = [OsString::from("getnetworkinfo")];
    let parsed = parse_cli_args(&cli_args, "").map_err(|error| {
        BenchError::case_failed("rpc-cli.parse-normalize-dispatch", error.to_string())
    })?;
    let CliCommand::RpcMethod(command) = parsed.command else {
        return Err(BenchError::case_failed(
            "rpc-cli.parse-normalize-dispatch",
            "CLI parser did not produce an RPC method command",
        ));
    };

    let call = normalize_method_call(&command.method, command.params).map_err(|error| {
        BenchError::case_failed(
            "rpc-cli.parse-normalize-dispatch",
            rpc_failure_reason(error),
        )
    })?;
    let mut context = ManagedRpcContext::for_local_operator(AddressNetwork::Regtest);
    let network_info = dispatch(&mut context, call).map_err(|error| {
        BenchError::case_failed(
            "rpc-cli.parse-normalize-dispatch",
            rpc_failure_reason(error),
        )
    })?;
    if network_info
        .get("protocolversion")
        .and_then(serde_json::Value::as_u64)
        .is_none()
    {
        return Err(BenchError::case_failed(
            "rpc-cli.parse-normalize-dispatch",
            "getnetworkinfo response did not include protocolversion",
        ));
    }

    let mempool_call =
        normalize_method_call("getmempoolinfo", RequestParameters::None).map_err(|error| {
            BenchError::case_failed(
                "rpc-cli.parse-normalize-dispatch",
                rpc_failure_reason(error),
            )
        })?;
    let mempool_info = dispatch(&mut context, mempool_call).map_err(|error| {
        BenchError::case_failed(
            "rpc-cli.parse-normalize-dispatch",
            rpc_failure_reason(error),
        )
    })?;
    if mempool_info
        .get("loaded")
        .and_then(serde_json::Value::as_bool)
        != Some(true)
    {
        return Err(BenchError::case_failed(
            "rpc-cli.parse-normalize-dispatch",
            "getmempoolinfo response did not report loaded mempool",
        ));
    }

    Ok(())
}

fn rpc_failure_reason(failure: RpcFailure) -> String {
    match failure.maybe_detail {
        Some(detail) => detail.message,
        None => format!("{:?}", failure.kind),
    }
}
