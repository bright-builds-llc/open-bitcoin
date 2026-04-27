#![cfg_attr(
    not(test),
    deny(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::unreachable,
        clippy::todo,
        clippy::unimplemented,
        clippy::panic_in_result_fn,
    )
)]
// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

use std::process::ExitCode;

use clap::Parser;
use open_bitcoin_cli::operator::{OperatorCli, runtime::execute_operator_cli};

fn main() -> ExitCode {
    let cli = OperatorCli::parse();
    let outcome = execute_operator_cli(cli);
    if !outcome.stdout.text.is_empty() {
        print!("{}", outcome.stdout.text);
    }
    if !outcome.stderr.text.is_empty() {
        eprintln!("{}", outcome.stderr.text);
    }
    ExitCode::from(outcome.exit_code.code())
}
