// Parity breadcrumbs:
// - packages/bitcoin-knots/src/bitcoin-cli.cpp
// - packages/bitcoin-knots/src/rpc/blockchain.cpp

use serde_json::{Value, json};

use super::*;
use crate::operator::{
    OperatorOutputFormat,
    prune::execute_prune_command_with_client,
    runtime::{OperatorCommandOutcome, OperatorExitCode, OperatorRuntimeError},
};

#[test]
fn prune_run_parses_height_or_timestamp() {
    // Arrange
    let command = parse_prune_command(&["prune", "run", "10"]);

    // Act
    let PruneCommand::Run {
        height_or_timestamp,
    } = command
    else {
        panic!("expected prune run");
    };

    // Assert
    assert_eq!(height_or_timestamp, 10);
}

#[test]
fn prune_lock_list_parses() {
    // Arrange
    let command = parse_prune_command(&["prune", "lock", "list"]);

    // Act
    let PruneCommand::Lock(lock) = command else {
        panic!("expected prune lock");
    };

    // Assert
    assert_eq!(lock.command, PruneLockCommand::List);
}

#[test]
fn prune_lock_set_parses_name_and_heights() {
    // Arrange
    let command = parse_prune_command(&[
        "prune",
        "lock",
        "set",
        "--name",
        "ibd",
        "--height-first",
        "1",
        "--height-last",
        "2",
    ]);

    // Act
    let PruneCommand::Lock(lock) = command else {
        panic!("expected prune lock");
    };

    // Assert
    assert_eq!(
        lock,
        PruneLockArgs {
            command: PruneLockCommand::Set {
                name: "ibd".to_string(),
                height_first: 1,
                height_last: 2,
            },
        }
    );
}

#[test]
fn prune_lock_clear_parses_name() {
    // Arrange
    let command = parse_prune_command(&["prune", "lock", "clear", "--name", "ibd"]);

    // Act
    let PruneCommand::Lock(lock) = command else {
        panic!("expected prune lock");
    };

    // Assert
    assert_eq!(
        lock.command,
        PruneLockCommand::Clear {
            name: "ibd".to_string(),
        }
    );
}

#[test]
fn prune_run_rejects_a_mode_flag() {
    // Arrange
    let args = vec![
        os("prune"),
        os("run"),
        os("--mode"),
        os("automatic"),
        os("10"),
    ];

    // Act
    let parsed = route_cli_invocation("open-bitcoin", &args);

    // Assert
    assert!(parsed.is_err(), "prune run must not accept a mode flag");
}

#[test]
fn prune_run_prints_the_rpc_height() {
    // Arrange
    let client = FakePruneRpc {
        expected_method: "pruneblockchain",
        expected_params: json!([10]),
        result: json!(-1),
    };
    let args = prune_args(PruneCommand::Run {
        height_or_timestamp: 10,
    });

    // Act
    let outcome = execute_prune_command_with_client(&args, OperatorOutputFormat::Human, &client)
        .expect("prune run");

    // Assert
    assert_eq!(
        outcome,
        OperatorCommandOutcome::success("Pruned height: -1\n")
    );
}

#[test]
fn prune_lock_list_prints_none_or_one_row() {
    // Arrange
    let empty = FakePruneRpc {
        expected_method: "listprunelocks",
        expected_params: json!([]),
        result: json!([]),
    };
    let listed = FakePruneRpc {
        expected_method: "listprunelocks",
        expected_params: json!([]),
        result: json!([{ "name": "ibd", "height_first": 1, "height_last": 2 }]),
    };
    let args = prune_args(PruneCommand::Lock(PruneLockArgs {
        command: PruneLockCommand::List,
    }));

    // Act
    let none = execute_prune_command_with_client(&args, OperatorOutputFormat::Human, &empty)
        .expect("empty locks");
    let row = execute_prune_command_with_client(&args, OperatorOutputFormat::Human, &listed)
        .expect("one lock");

    // Assert
    assert_eq!(none.stdout.text, "Prune locks: none\n");
    assert_eq!(
        row.stdout.text,
        "Prune lock: name=ibd height_first=1 height_last=2\n"
    );
}

#[test]
fn prune_lock_set_prints_the_rpc_row() {
    // Arrange
    let client = FakePruneRpc {
        expected_method: "setprunelock",
        expected_params: json!(["ibd", 1, 2]),
        result: json!({ "name": "ibd", "height_first": 1, "height_last": 2 }),
    };
    let args = prune_args(PruneCommand::Lock(PruneLockArgs {
        command: PruneLockCommand::Set {
            name: "ibd".to_string(),
            height_first: 1,
            height_last: 2,
        },
    }));

    // Act
    let outcome = execute_prune_command_with_client(&args, OperatorOutputFormat::Human, &client)
        .expect("set lock");

    // Assert
    assert_eq!(
        outcome.stdout.text,
        "Prune lock set: name=ibd height_first=1 height_last=2\n"
    );
    assert_eq!(outcome.exit_code, OperatorExitCode::Success);
}

#[test]
fn prune_lock_clear_prints_the_cleared_name() {
    // Arrange
    let client = FakePruneRpc {
        expected_method: "clearprunelock",
        expected_params: json!(["ibd"]),
        result: json!({ "success": true }),
    };
    let args = prune_args(PruneCommand::Lock(PruneLockArgs {
        command: PruneLockCommand::Clear {
            name: "ibd".to_string(),
        },
    }));

    // Act
    let outcome = execute_prune_command_with_client(&args, OperatorOutputFormat::Human, &client)
        .expect("clear lock");

    // Assert
    assert_eq!(outcome.stdout.text, "Prune lock cleared: name=ibd\n");
    assert_eq!(outcome.exit_code, OperatorExitCode::Success);
}

#[test]
fn prune_lock_clear_false_still_exits_success() {
    // Arrange
    let client = FakePruneRpc {
        expected_method: "clearprunelock",
        expected_params: json!(["missing"]),
        result: json!({ "success": false }),
    };
    let args = prune_args(PruneCommand::Lock(PruneLockArgs {
        command: PruneLockCommand::Clear {
            name: "missing".to_string(),
        },
    }));

    // Act
    let outcome = execute_prune_command_with_client(&args, OperatorOutputFormat::Human, &client)
        .expect("clear missing");

    // Assert
    assert_eq!(outcome.stdout.text, "Prune lock clear: success=false\n");
    assert_eq!(outcome.exit_code, OperatorExitCode::Success);
}

#[test]
fn prune_copy_forbids_out_of_scope_claims_and_storage() {
    // Arrange
    let source = include_str!("../../prune.rs");

    // Act
    let lowered = source.to_ascii_lowercase();

    // Assert
    assert!(!lowered.contains("fjall"));
    assert!(!source.contains("flush_applying_prune_plan"));
    assert!(!source.contains("sync_prune_locks"));
    for forbidden in [
        "archive-node",
        "assumeutxo",
        "assumevalid",
        "BIP37",
        "compact filter",
        "public default",
        "production ready",
        "production readiness",
        "production-funds",
    ] {
        assert!(!source.contains(forbidden), "forbidden copy {forbidden}");
    }
}

fn parse_prune_command(args: &[&str]) -> PruneCommand {
    let argv = args.iter().copied().map(os).collect::<Vec<_>>();
    let route = route_cli_invocation("open-bitcoin", &argv).expect("route");
    let CliRoute::Operator(cli) = route else {
        panic!("expected operator route");
    };
    let OperatorCommand::Prune(prune) = cli.command else {
        panic!("expected prune command");
    };
    prune.command
}

fn prune_args(command: PruneCommand) -> crate::operator::PruneArgs {
    crate::operator::PruneArgs { command }
}

struct FakePruneRpc {
    expected_method: &'static str,
    expected_params: Value,
    result: Value,
}

impl crate::operator::prune::PruneRpcClient for FakePruneRpc {
    fn call(&self, method: &str, params: Value) -> Result<Value, OperatorRuntimeError> {
        assert_eq!(method, self.expected_method);
        assert_eq!(params, self.expected_params);
        Ok(self.result.clone())
    }
}
