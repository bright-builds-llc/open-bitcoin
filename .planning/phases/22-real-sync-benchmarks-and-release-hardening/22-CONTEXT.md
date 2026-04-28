---
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: 22-2026-04-28T01-24-15
generated_at: 2026-04-28T01:24:15Z
---

# Phase 22: Real-Sync Benchmarks and Release Hardening - Context

## Phase Boundary

**Goal:** Close v1.1 with reproducible performance evidence, documentation, parity updates, and verification coverage.

**Success criteria:**
1. Repo-native verification covers new CLI, config, service, storage, sync, metrics, logging, dashboard, migration, and parity-breadcrumb rules without requiring public network access.
2. Real-sync benchmarks measure headers sync, block download/connect, storage read/write, restart recovery, dashboard/status overhead, and wallet rescan cost with reproducible local reports.
3. Documentation explains install, onboarding, service lifecycle, status, dashboard, config layering, migration, real-sync testing, and known limitations.
4. Parity docs and machine-readable indexes distinguish v1.1 shipped claims from deferred or out-of-scope surfaces.
5. The milestone is ready for `/gsd-verify-work`, `/gsd-secure-phase`, and `/gsd-audit-milestone`.

**Out of scope:**
- Public-network benchmark runs in the default local verify path.
- Release timing thresholds that pass or fail the milestone on elapsed numbers alone.
- New package-distribution or signed-release machinery.
- Mutation-capable migration apply flows beyond the existing dry-run planner.

## Requirements In Scope

- `SYNC-05`: reproducible real-sync benchmark evidence
- `MIG-05`: intentional migration differences stay visible in the parity ledger and operator docs
- `VER-05`: repo-native verification covers the new v1.1 surfaces without public-network dependency
- `VER-06`: real-sync benchmark coverage and reproducible local reports
- `VER-07`: operator-facing documentation
- `VER-08`: auditable machine-readable parity and release-readiness surfaces

## Canonical References

- `.planning/ROADMAP.md` — Phase 22 goal, success criteria, and milestone position
- `.planning/REQUIREMENTS.md` — `SYNC-05`, `MIG-05`, `VER-05`, `VER-06`, `VER-07`, `VER-08`
- `.planning/STATE.md` — current focus and ready-for-next-phase status
- `AGENTS.md` — repo-local verification contract and parity-breadcrumb requirements
- `AGENTS.bright-builds.md`
- `standards/index.md`
- `standards/core/architecture.md`
- `standards/core/code-shape.md`
- `standards/core/verification.md`
- `standards/core/testing.md`
- `standards/languages/rust.md`

## Repo Guidance That Materially Informs This Phase

- Use `bash scripts/verify.sh` as the repo-native verification contract.
- Keep parity claims auditable through `docs/parity/index.json` and companion docs.
- Treat new Rust files and tests as parity-breadcrumb-managed surfaces.
- Follow functional-core versus imperative-shell boundaries and keep the benchmark/report plumbing easy to test.
- Keep the default local gate deterministic, bounded, and free of required public-network access.

## Current State

### Existing benchmark and verification assets

- `scripts/verify.sh` already runs format, lint, build, tests, bounded benchmark smoke output, Bazel smoke builds, coverage, panic-site checks, file-length guards, and parity-breadcrumb checks.
- `packages/open-bitcoin-bench` already produces JSON and Markdown reports through `scripts/run-benchmarks.sh`, but the Phase 10 contract is still microbenchmark-oriented and mapping-first.
- `packages/open-bitcoin-node` and `packages/open-bitcoin-cli` already contain deterministic test fixtures for sync, storage reopen, logging, status, dashboard projection, and wallet-rescan behavior that can seed Phase 22 benchmark cases.

### Existing documentation and parity assets

- `README.md` already exposes status, onboarding, migration, and wallet preview commands, but it is still contributor-leaning rather than a full operator guide.
- `docs/architecture/status-snapshot.md`, `docs/architecture/config-precedence.md`, and `docs/architecture/operator-observability.md` document important contracts that Phase 22 can reference instead of duplicating.
- `docs/parity/index.json`, `docs/parity/checklist.md`, and `docs/parity/release-readiness.md` remain the source-of-truth parity/readiness surfaces, but the release-readiness doc is still scoped to an older headless-v1 handoff and the machine-readable ledger does not yet model the Phase 22 operator/runtime closeout explicitly.

## Decisions

1. **Extend the existing benchmark entrypoint instead of creating a second one.**
   Phase 22 builds on `open-bitcoin-bench` and `scripts/run-benchmarks.sh` so contributors still have one repo-owned benchmark surface.

2. **Keep the default verify path hermetic and deterministic.**
   `bash scripts/verify.sh` remains bounded and offline by default. Full or heavier runs stay opt-in, and public-network access remains outside the normal local gate.

3. **Use deterministic runtime-backed benchmark scenarios, not public-network timing.**
   The new benchmark cases should measure real sync, storage, dashboard, status, and wallet-rescan paths using local fixtures, temp dirs, and scripted transports that already exist in the repo.

4. **Make reproducibility explicit in the benchmark report contract.**
   Reports should capture enough scenario or profile metadata that reviewers can tell what was measured without inferring it from filenames alone.

5. **Add an operator-facing guide instead of overloading README.**
   Phase 22 should create a practical doc that covers install, onboarding, service lifecycle, status, dashboard, config layering, migration, real-sync testing, and known limitations, then link to it from `README.md`.

6. **Repair stale docs that still describe shipped v1.1 surfaces as boundaries.**
   Documentation that still frames `service` or `dashboard` as future-only work must be updated so the milestone story stays credible.

7. **Make shipped vs deferred runtime claims explicit in the parity ledger.**
   `docs/parity/index.json`, checklist views, and release-readiness docs must model the Phase 22 v1.1 runtime closeout as first-class evidence instead of burying it in free text.

## Key Files and Likely Change Surfaces

- `packages/open-bitcoin-bench/src/runner.rs`
- `packages/open-bitcoin-bench/src/report.rs`
- `packages/open-bitcoin-bench/src/registry.rs`
- `packages/open-bitcoin-bench/src/cases/*.rs`
- `packages/open-bitcoin-bench/src/main.rs`
- `packages/open-bitcoin-bench/Cargo.toml`
- `scripts/run-benchmarks.sh`
- `scripts/verify.sh`
- `README.md`
- `docs/architecture/cli-command-architecture.md`
- `docs/parity/benchmarks.md`
- `docs/parity/checklist.md`
- `docs/parity/index.json`
- `docs/parity/release-readiness.md`
- a new operator-facing guide under `docs/`

## Risks

- Benchmark scope can grow quickly if it tries to reproduce live-network behavior instead of using deterministic runtime fixtures.
- Report schema churn can create noisy diff surfaces if it is not defined carefully before implementation.
- Release docs can accidentally overclaim parity if deferred or out-of-scope surfaces are not echoed in both the operator guide and parity ledger.
- New benchmark code must respect existing file-length and pure-core guardrails as dependencies expand.

## Implementation Notes

- Reuse existing pure helpers and test-fixture builders where possible to keep benchmark adapters thin.
- Prefer new Rust modules or case files over growing existing files past repo guardrails.
- Add focused tests around benchmark-report schema and required group coverage instead of relying on manual inspection.
- Keep generated benchmark evidence under `packages/target/benchmark-reports`; do not check runtime timing artifacts into git.
