---
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: 22-2026-04-28T01-24-15
generated_at: 2026-04-28T01:24:15Z
---

# Phase 22 Research

## Summary

Phase 22 is not starting from zero. The repository already has a strong offline verification contract, deterministic runtime tests for the new v1.1 surfaces, and a benchmark wrapper. The main gaps are:

- the benchmark harness is still Phase 10 microbenchmark and mapping oriented
- operator-facing docs do not yet read like a v1.1 release guide
- parity and release-readiness artifacts do not yet model the Phase 22 runtime closeout explicitly

## Verification Baseline

### What already exists

- `bash scripts/verify.sh` is already the repo-native verification contract.
- The current verify path is offline by default and already runs:
  - LOC report freshness
  - parity-breadcrumb checks
  - pure-core and file-length guards
  - panic-site checks
  - workspace `cargo fmt`, `cargo clippy`, `cargo build`, and `cargo test`
  - bounded benchmark smoke generation
  - Bazel smoke builds
  - missing-line coverage checks
- Existing Rust tests already cover most of the new v1.1 surfaces indirectly:
  - service lifecycle
  - dashboard rendering and projection
  - migration planning
  - status snapshot rendering
  - durable storage reopen and recovery
  - sync persistence and restart behavior
  - logging retention and status

### Gap to close

- The benchmark smoke output is generated but not yet validated as a Phase 22 runtime-hardening artifact.
- The parity ledger and release-readiness docs do not yet explain that the verify contract already covers these operator/runtime surfaces without public-network access.

## Benchmark Baseline

### What already exists

- `packages/open-bitcoin-bench` emits JSON and Markdown reports.
- `scripts/run-benchmarks.sh` is the contributor-facing wrapper.
- The current registry has seven groups:
  - `consensus-script`
  - `block-transaction-codec`
  - `chainstate`
  - `mempool-policy`
  - `network-wire-sync`
  - `wallet`
  - `rpc-cli`
- Reports are stored under `packages/target/benchmark-reports`.
- The current contract is intentionally threshold-free and mapping-first.

### Gap to close

The current harness does not yet measure the concrete Phase 22 scenarios:

- headers sync
- block download and connect
- storage write and read
- restart recovery
- dashboard and status overhead
- wallet rescan cost

The current report schema is also thin for reproducible release-hardening evidence. It does not yet make scenario intent or benchmark profile explicit enough.

## Documentation Baseline

### What already exists

- `README.md` gives a high-level project summary and command previews.
- `docs/architecture/status-snapshot.md` documents status semantics well.
- `docs/architecture/config-precedence.md` documents config layering well.
- `docs/parity/catalog/drop-in-audit-and-migration.md` is already strong on migration tradeoffs and non-claims.
- `docs/parity/index.json` and `docs/parity/checklist.md` are already the repo's auditable parity roots.

### Gap to close

Phase 22 still needs a practical operator-facing guide that explains:

- how to install and bootstrap the current source-built runtime
- how onboarding fits with config ownership and migration
- service lifecycle expectations on supported platforms
- how to interpret `status` and `dashboard`
- how to run real-sync benchmark and verification flows locally
- which limitations and non-claims still apply

There is also stale wording in `docs/architecture/cli-command-architecture.md` that still describes `service` and `dashboard` as deferred boundaries.

## Parity and Readiness Baseline

### What already exists

- `docs/parity/index.json` is the machine-readable source of truth.
- `docs/parity/checklist.md` mirrors the ledger in a human-readable format.
- `docs/parity/deviations-and-unknowns.md` already records deferred and risky areas.
- Migration deviations are already modeled and surfaced cleanly.

### Gap to close

- `docs/parity/release-readiness.md` is still framed as a headless-v1 handoff instead of a v1.1 operator-runtime closeout.
- The machine-readable parity ledger does not yet model the new Phase 22 operator/runtime and release-hardening claims as explicit surfaces.
- Deferred or out-of-scope runtime surfaces should be separated more clearly from shipped v1.1 claims.

## Recommended Phase Breakdown

### Plan 22-01

Extend the benchmark harness and report schema with deterministic runtime-backed scenarios for Phase 22.

### Plan 22-02

Add an operator-facing v1.1 guide and refresh README plus contract docs so the release story is discoverable and accurate.

### Plan 22-03

Update parity, release-readiness, and verification hardening so v1.1 shipped claims and deferrals are explicit and auditable.

## Recommended Verification

- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-bench -- --nocapture`
- `bash scripts/run-benchmarks.sh --smoke --output-dir packages/target/benchmark-reports`
- `bash scripts/verify.sh`

Use narrower loops during implementation, then finish with the repo-native `bash scripts/verify.sh` contract before commit.
