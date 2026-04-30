---
phase: 24-wallet-aware-live-status-and-build-provenance
verified: 2026-04-28T19:20:00.000Z
status: passed
score: 4/4 truths verified
generated_by: gsd-execute-phase
lifecycle_mode: yolo
phase_lifecycle_id: 24-2026-04-28T17-43-00
generated_at: 2026-04-28T19:20:00.000Z
lifecycle_validated: true
overrides_applied: 0
---

# Phase 24: Wallet-Aware Live Status and Build Provenance Verification Report

**Phase Goal:** Keep live status and dashboard snapshots truthful when wallet
selection is missing or ambiguous, and surface real build provenance through the
shared operator status model.
**Requirements:** OBS-01, OBS-02, WAL-05, DASH-01
**Verified:** 2026-04-28T19:20:00.000Z
**Status:** PASS

## Goal Achievement

### Observable Truths

| # | Truth | Status | Evidence |
|---|-------|--------|----------|
| 1 | Live status and dashboard snapshots no longer collapse to `NodeRuntimeState::Unreachable` when wallet selection is missing or ambiguous. | VERIFIED | `packages/open-bitcoin-cli/src/operator/status.rs` now derives node reachability from node-scoped RPC calls only and degrades wallet-only failures through `collect_live_wallet_status()`. |
| 2 | Wallet selection issues are surfaced as wallet-specific unavailable data and health diagnostics while node reachability remains accurate. | VERIFIED | `packages/open-bitcoin-cli/src/operator/status/http.rs` now parses JSON-RPC error payloads, `packages/open-bitcoin-cli/src/operator/runtime.rs` resolves trusted selected-wallet routing, and `packages/open-bitcoin-cli/src/operator/status/tests.rs` covers wallet-only failure plus ambiguous multiwallet handling. |
| 3 | Build provenance is populated in shared status snapshots when available and rendered through operator-facing status and dashboard surfaces. | VERIFIED | `packages/open-bitcoin-cli/build.rs` emits compile-time build metadata, `packages/open-bitcoin-cli/src/operator/status.rs` assembles it into `BuildProvenance`, and `packages/open-bitcoin-cli/src/operator/status/render.rs` plus `packages/open-bitcoin-cli/src/operator/dashboard/model.rs` render the populated fields. |
| 4 | Phase 24 refreshed the roadmap, requirements ledger, summaries, and verification evidence around the repaired runtime behavior. | VERIFIED | `.planning/ROADMAP.md`, `.planning/REQUIREMENTS.md`, `24-01-SUMMARY.md`, `24-02-SUMMARY.md`, `24-03-SUMMARY.md`, and this report all reflect the completed gap-closure work. |

**Score:** 4/4 truths verified

## Requirements Coverage

| Requirement | Status | Evidence |
|---|---|---|
| OBS-01 | SATISFIED | `open-bitcoin status` now keeps node, sync, mempool, peers, and health truth live even when wallet selection is ambiguous, while also surfacing real build provenance when available. |
| OBS-02 | SATISFIED | The machine-readable status JSON remains stable and now reports wallet-scope unavailability separately from node reachability, with populated build metadata fields for Cargo builds. |
| WAL-05 | SATISFIED | The status runtime now honors the shipped wallet-selection model by using a selected or sole managed wallet for wallet-scoped RPC status calls when local registry evidence is trustworthy. |
| DASH-01 | SATISFIED | Dashboard snapshots continue to come from the repaired shared status collector and now display truthful wallet degradation plus shared build provenance. |

## Verification Evidence

- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli status::tests -- --nocapture` passed.
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli --test operator_binary -- --nocapture` passed.
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli -- --nocapture` passed.
- `bun run scripts/generate-loc-report.ts --source=worktree --output=docs/metrics/lines-of-code.md` refreshed the tracked LOC report required by the repo-native gate.
- `bash scripts/verify.sh` passed end-to-end, including:
  - LOC freshness
  - parity breadcrumb validation
  - pure-core dependency and import checks
  - production Rust file-length validation
  - panic-site validation
  - workspace format, lint, build, test, and coverage steps
  - benchmark smoke validation
  - Bazel smoke build

## Human Verification Required

None. The phase repairs operator runtime truthfulness through shared status code
and hermetic verification rather than a manual-only operational workflow.

## Residual Risks

- `open-bitcoin status` and `open-bitcoin dashboard` still do not expose an
  explicit wallet-selection flag; the phase improves truthfulness without
  widening the surface.
- Non-Cargo builds may still show unavailable build metadata when compile-time
  env vars are absent, though the unavailable reasons now remain explicit.
- Full multiwallet lifecycle parity and richer wallet-routing ergonomics remain
  outside the current shipped wallet slice.

---

_Verified: 2026-04-28T19:20:00.000Z_  
_Verifier: GPT-5.4 (GSD yolo wrapper)_
