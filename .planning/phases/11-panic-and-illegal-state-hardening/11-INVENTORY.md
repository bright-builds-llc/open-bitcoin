# Phase 11 Production Panic-Site Inventory

## Scope

Included first-party Rust production code under `packages/open-bitcoin-*/src`.

Excluded:

- `packages/bitcoin-knots`
- `packages/target`
- `tests.rs`
- inline `#[cfg(test)]` sections

Searched for:

- `unwrap(`
- `expect(`
- `panic!(`
- `unreachable!(`
- `todo!(`
- `unimplemented!(`

## Classification Result

The initial production-only scan showed roughly 55 panic-like sites. The
reachable sites were treated as `fix now` instead of being allowlisted.

Closeout categories:

| Category | Result |
|----------|--------|
| Fix now | Replaced with typed errors, `let...else`, explicit fallbacks, or non-panicking helpers. |
| Proven invariant | No production entries kept. |
| Adapter-boundary I/O | Replaced with caller-visible errors where reachable. |
| Benchmark/tooling acceptable | Replaced with error propagation in first-party tooling. |
| False positive or test-only | Excluded by guard scope. |

## Main Clusters Addressed

- Mempool admission, replacement, eviction, and state recomputation.
- Consensus block validation, contextual transaction value accounting, taproot
  witness handling, script stack helpers, and Merkle helpers.
- Wallet transaction building, nested segwit script construction, taproot
  signing, and script push-data encoding.
- CLI stdin handling, network command and payload decoding, codec primitive
  decoding, benchmark argument handling, and JSON report writing.

## Guard State

`scripts/check-panic-sites.sh` is the reproducible scan. It is wired into
`bash scripts/verify.sh`.

`scripts/panic-sites.allowlist` is intentionally empty at close. Any future
entry must include a path, matching needle, and narrow invariant rationale.
