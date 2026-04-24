# Quick Task 260424-jnn: Partial Clippy Panic-Lint Enforcement

## Goal

Lock in the Phase 11 production panic cleanup by adding production-only Clippy deny attributes to runtime and domain crate roots that already pass the panic lint set.

## Tasks

1. Add `cfg_attr(not(test), deny(...))` panic-related Clippy lint blocks to selected runtime/domain `lib.rs` roots and production binary roots.
2. Leave benchmark and test-harness crates out of the Clippy deny layer while retaining coverage through `scripts/check-panic-sites.sh`.
3. Verify with targeted Clippy preflight, the panic-site guard, full repo verification, and the repo-required Rust pre-commit sequence.

## Done Criteria

- Future production `unwrap`, `expect`, `panic!`, `unreachable!`, `todo!`, `unimplemented!`, and panics in `Result` functions fail Clippy in the selected crates.
- No public API, RPC/CLI behavior, wire behavior, or crate exports change.
- All requested verification commands pass.
