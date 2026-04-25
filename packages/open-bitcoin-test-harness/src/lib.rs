#![forbid(unsafe_code)]
// Parity breadcrumbs:
// - packages/bitcoin-knots/test/functional/test_framework
// - packages/bitcoin-knots/test/functional/interface_rpc.py
// - packages/bitcoin-knots/test/functional/interface_bitcoin_cli.py

//! Test-only parity harness utilities for Open Bitcoin.

pub mod case;
pub mod isolation;
pub mod report;
pub mod target;

pub use case::{
    CaseOutcome, ExpectedOutcome, FunctionalCase, SuiteReport, run_suite, skipped_suite,
};
pub use isolation::{PortReservation, ProcessGuard, Sandbox};
pub use report::{ReportError, write_reports_from_env};
pub use target::{HarnessError, HarnessTarget, RpcHttpTarget};
