---
phase: 30-configless-live-status-and-dashboard-bootstrap
verified: 2026-04-29T16:31:06.038Z
status: passed
score: 3/3 truths verified
generated_by: gsd-execute-phase
lifecycle_mode: yolo
phase_lifecycle_id: 30-2026-04-29T16-19-20
generated_at: 2026-04-29T16:31:06.038Z
lifecycle_validated: true
overrides_applied: 0
---

# Phase 30: Configless Live Status and Dashboard Bootstrap Verification Report

**Phase Goal:** Restore truthful live status and dashboard behavior for the
documented flag-only local operator workflow when no implicit `bitcoin.conf`
file exists.
**Requirements:** OBS-01, DASH-01, VER-07
**Verified:** 2026-04-29T16:31:06.038Z
**Status:** PASS

## Goal Achievement

### Observable Truths

| # | Truth | Status | Evidence |
|---|-------|--------|----------|
| 1 | The shared operator runtime can now build live RPC startup inputs from the selected datadir and chain defaults even when the implicit `bitcoin.conf` path is absent. | VERIFIED | `packages/open-bitcoin-cli/src/args.rs` now carries `maybe_chain_name` into the normal runtime config loader, and `packages/open-bitcoin-cli/src/operator/runtime.rs` now builds shared status/dashboard startup args that pass `-conf` only when the resolved `bitcoin.conf` actually exists. |
| 2 | `open-bitcoin status` and `open-bitcoin dashboard` both keep the repaired shared bootstrap path while preserving the stopped fallback when credentials cannot be bootstrapped. | VERIFIED | `packages/open-bitcoin-cli/src/operator/tests.rs` now covers three public-operator cases: configless `status` with cookie auth, configless `dashboard` with cookie auth, and the no-credentials fallback path. The first two now render `node.state = "unreachable"` instead of silently collapsing to `stopped`, while the last case stays `stopped`. |
| 3 | Operator-facing docs and repo-native verification now keep the flag-only workflow truthful and evidence-backed. | VERIFIED | `docs/operator/runtime-guide.md` now states that an implicit datadir-local `bitcoin.conf` is optional for live `status`/`dashboard` bootstrap, `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli -- --nocapture` passed, and `bash scripts/verify.sh` completed cleanly after the expected LOC refreshes. |

**Score:** 3/3 truths verified

## Requirements Coverage

| Requirement | Status | Evidence |
|---|---|---|
| OBS-01 | SATISFIED | The shared operator runtime no longer mistakes a configless live node for a stopped node solely because the implicit `bitcoin.conf` file is absent, and it preserves explicit unreachable/stopped distinctions through the public `status` flow. |
| DASH-01 | SATISFIED | `open-bitcoin dashboard` still reuses the same shared status snapshot bootstrap, and the new public-operator regression test proves the configless dashboard path follows the same truthful live-RPC behavior as `status`. |
| VER-07 | SATISFIED | `docs/operator/runtime-guide.md` now documents that the flag-only status/dashboard workflow does not require an implicit datadir-local `bitcoin.conf`, and the passing verification evidence is captured in this phase report. |

## Verification Evidence

- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli -- --nocapture` passed.
- `bun run scripts/generate-loc-report.ts --source=worktree --output=docs/metrics/lines-of-code.md` refreshed the stale LOC report required by the repo-native gate.
- `cargo fmt --manifest-path packages/Cargo.toml --all` normalized the import ordering expected by the verification stack.
- `bash scripts/verify.sh` passed end-to-end in `1m 57.707s` after the final LOC refresh.

## Human Verification Required

None. Phase 30 closes a code and documentation truthfulness gap through public
operator regression tests plus the repo-native verification contract.

## Residual Risks

- The configless bootstrap still depends on normal RPC auth evidence under the
  selected datadir. If neither a cookie file nor other bootstrappable auth
  inputs exist, the operator surfaces will truthfully stay in the stopped
  fallback path.
- The Phase 30 fix carries the selected chain through internal startup args but
  does not add new operator-facing RPC override flags; broader operator RPC
  override work remains outside this phase.
- Future refactors around `CliStartupArgs` and runtime config loading should
  preserve the shared operator/bootstrap path so `status` and `dashboard` do
  not drift again.

---

_Verified: 2026-04-29T16:31:06.038Z_  
_Verifier: GPT-5.4 (GSD yolo wrapper)_
