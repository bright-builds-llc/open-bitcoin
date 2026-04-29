---
phase: 28-service-log-path-truth-and-operator-docs-alignment
verified: 2026-04-29T13:50:24.398Z
status: passed
score: 4/4 truths verified
generated_by: gsd-execute-phase
lifecycle_mode: yolo
phase_lifecycle_id: 28-2026-04-29T13-37-12
generated_at: 2026-04-29T13:50:24.398Z
lifecycle_validated: true
overrides_applied: 0
---

# Phase 28: Service Log-Path Truth and Operator Docs Alignment Verification Report

**Phase Goal:** Preserve configured service log-path truth across
launchd/systemd preview, apply, status, and operator docs.
**Requirements:** SVC-03, SVC-04, VER-07
**Verified:** 2026-04-29T13:50:24.398Z
**Status:** PASS

## Goal Achievement

### Observable Truths

| # | Truth | Status | Evidence |
|---|-------|--------|----------|
| 1 | The shared operator runtime now derives one concrete service log file from the selected operator log directory instead of passing a raw directory as a log path. | VERIFIED | `packages/open-bitcoin-cli/src/operator/service.rs` now exposes `service_log_path_from_log_dir()`, and `packages/open-bitcoin-cli/src/operator/runtime.rs` routes that derived file through both the CLI service path and the dashboard service runtime. |
| 2 | Launchd and systemd generated service definitions now preserve the concrete log file path in dry-run and apply behavior. | VERIFIED | `packages/open-bitcoin-cli/src/operator/service/launchd.rs` continues to emit `StandardOutPath` or `StandardErrorPath`, and `packages/open-bitcoin-cli/src/operator/service/systemd.rs` now emits `StandardOutput=append:...` and `StandardError=append:...` when a managed log path is available. |
| 3 | `open-bitcoin service status` now surfaces the effective service log path or an explicit unavailable reason through `ServiceStateSnapshot`. | VERIFIED | `packages/open-bitcoin-cli/src/operator/service.rs` now renders a `logs:` line unconditionally, while `launchd.rs` and `systemd.rs` recover the installed file-backed path or preserve a platform-backed unavailable reason. |
| 4 | Operator-facing docs now describe the shipped service log-path behavior truthfully. | VERIFIED | `docs/operator/runtime-guide.md` now explains that service previews derive `<log_dir>/open-bitcoin.log` and that `open-bitcoin service status` reports the effective installed service log path. |

**Score:** 4/4 truths verified

## Requirements Coverage

| Requirement | Status | Evidence |
|---|---|---|
| SVC-03 | SATISFIED | The shared runtime now derives a concrete service log file from the selected operator log directory, and both launchd and systemd dry-run artifacts preserve that path in generated content and previews. |
| SVC-04 | SATISFIED | `ServiceStateSnapshot` now preserves either the recovered effective log path or an explicit unavailable reason, and the service-status renderer always prints the `logs:` line. |
| VER-07 | SATISFIED | `docs/operator/runtime-guide.md` now describes the concrete service-managed log file behavior and the truthful `service status` surface. |

## Verification Evidence

- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli service::tests -- --nocapture` passed.
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli status::tests -- --nocapture` passed.
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-bench -- --nocapture` passed.
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli -- --nocapture` passed.
- `bun run scripts/generate-loc-report.ts --source=worktree --output=docs/metrics/lines-of-code.md` refreshed the stale LOC report required by the repo-native gate.
- `bash scripts/verify.sh` passed end-to-end after the LOC refresh.

## Human Verification Required

None. Phase 28 closes a code and documentation truthfulness gap through hermetic
tests and the repo-native verification contract.

## Residual Risks

- Older installed plist or unit files created before the repair may still omit a
  file-backed service log path, so `service status` will surface an explicit
  unavailable reason until the service is reinstalled.
- The current service snapshot still exposes one combined service-managed log
  file path; separate stdout/stderr surfacing remains outside Phase 28.
- The service-managed `open-bitcoin.log` file sits alongside the existing
  structured runtime log directory and does not replace the structured JSONL log
  files used by the broader observability surface.

---

_Verified: 2026-04-29T13:50:24.398Z_  
_Verifier: GPT-5.4 (GSD yolo wrapper)_
