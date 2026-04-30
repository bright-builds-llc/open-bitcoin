---
phase: 33-operator-surface-truth-and-coverage-cleanup
verified: 2026-04-30T05:12:28.878Z
status: passed
score: 4/4 truths verified
generated_by: gsd-execute-phase
lifecycle_mode: yolo
phase_lifecycle_id: 33-2026-04-30T05-02-20
generated_at: 2026-04-30T05:12:28.878Z
lifecycle_validated: true
overrides_applied: 0
---

# Phase 33: Operator Surface Truth and Coverage Cleanup Verification Report

**Phase Goal:** Resolve the remaining operator-surface truthfulness and
verification debt around `status --watch`, unmanaged service preview hints,
interactive dashboard coverage, and nearby flaky operator-binary status checks
before v1.1 archive.
**Requirements:** none (optional cleanup)
**Verified:** 2026-04-30T05:12:28.878Z
**Status:** PASS

## Goal Achievement

### Observable Truths

| # | Truth | Status | Evidence |
|---|-------|--------|----------|
| 1 | `open-bitcoin status --watch` no longer misrepresents the shipped runtime behavior. | VERIFIED | `packages/open-bitcoin-cli/src/operator.rs` no longer defines a `watch` field on `StatusArgs`, `packages/open-bitcoin-cli/src/operator/runtime.rs` remains a one-shot status renderer, and `packages/open-bitcoin-cli/src/operator/tests.rs` now proves `open-bitcoin status --watch` is rejected instead of silently doing nothing. |
| 2 | Unmanaged service/install preview guidance now points operators at the real preview flow across shared CLI errors, platform-backed status diagnostics, dashboard reuse, and operator docs. | VERIFIED | `packages/open-bitcoin-cli/src/operator/service.rs`, `packages/open-bitcoin-cli/src/operator/service/launchd.rs`, and `packages/open-bitcoin-cli/src/operator/service/systemd.rs` all now say `open-bitcoin service install` previews what would be created and reserve `--apply` for mutation; `docs/operator/runtime-guide.md` already documented that contract and required no wording change. |
| 3 | Higher-level verification now covers the relevant interactive dashboard action paths. | VERIFIED | `packages/open-bitcoin-cli/src/operator/dashboard/app.rs` now includes hermetic tests for pending, confirmed, cancelled, and shared-status action handling through `handle_action()`, raising coverage above the existing lower-level `dashboard/action.rs` unit tests and the existing non-TTY snapshot binary coverage. |
| 4 | Nearby operator-binary status coverage is deterministic enough that the normal verification path no longer depends on a lucky rerun. | VERIFIED | `packages/open-bitcoin-cli/tests/operator_binary.rs` now waits for complete HTTP request bodies before the fake RPC server responds, `open_bitcoin_status_json_uses_fake_running_rpc` passed in the full `open-bitcoin-cli` package suite plus two extra focused reruns, and the final `bash scripts/verify.sh` run completed cleanly end to end. |

**Score:** 4/4 truths verified

## Verification Evidence

- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli
  operator::service::tests` passed after adding the new preview-hint coverage.
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli
  operator::tests` passed after removing the dead `--watch` surface and adding
  the rejection test.
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli
  dashboard::app` passed with the new app-level confirmation-loop coverage.
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli` passed,
  including the full `operator_binary` integration suite.
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli --test
  operator_binary open_bitcoin_status_json_uses_fake_running_rpc` passed two
  additional times after the package-wide pass.
- `bun run scripts/generate-loc-report.ts --source=worktree
  --output=docs/metrics/lines-of-code.md` refreshed the tracked LOC report after
  the Phase 33 edits and formatting.
- `bash scripts/verify.sh` passed end to end in `2m 0.608s` after one fast
  first-run failure that only required a post-format LOC refresh.

## Human Verification Required

None. Phase 33 closes deterministic operator-surface cleanup through hermetic
tests, repeated binary reruns, and the repo-native verification contract.

## Residual Risks

- The dashboard still does not have a full pseudoterminal end-to-end binary
  harness. Phase 33 closes the reported gap with app-level confirmation-loop
  coverage, but true terminal-paint integration remains manual usage territory.
- Phase 34 remains open as the final optional v1.1 cleanup. Phase 33 does not
  address the migration detection ownership-shape debt.

---

_Verified: 2026-04-30T05:12:28.878Z_  
_Verifier: GPT-5.4 (GSD yolo wrapper)_
