// Parity breadcrumbs:
// - packages/bitcoin-knots/src/bitcoin-cli.cpp
// - packages/bitcoin-knots/src/rpc/client.cpp
// - packages/bitcoin-knots/test/functional/interface_bitcoin_cli.py

use super::*;

pub(super) fn run_raw_cli(sandbox: &TestSandbox, args: &[String]) -> Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_open-bitcoin-cli"));
    command.env("HOME", &sandbox.home);
    for arg in args {
        command.arg(arg);
    }
    command.output().expect("cli output")
}

pub(super) fn stdout_text(output: &Output) -> String {
    String::from_utf8(output.stdout.clone()).expect("stdout")
}

pub(super) fn stderr_text(output: &Output) -> String {
    String::from_utf8(output.stderr.clone()).expect("stderr")
}

pub(super) fn assert_success_json(output: &Output) -> Value {
    assert!(
        output.status.success(),
        "stdout: {}\nstderr: {}",
        stdout_text(output),
        stderr_text(output),
    );
    serde_json::from_slice(&output.stdout).expect("stdout json")
}
