---
phase: 52-operator-evidence-cleanup
status: passed
verified_at: 2026-06-01T01:54:14Z
requirements:
  - OBS-02
  - OBS-03
scope: deterministic cleanup only
public_network_checks: not_run
---

# Phase 52 Verification

## Verdict

**status: passed**

Phase 52 closes deterministic operator-evidence debt D-02 and D-04 without
refreshing live public-network evidence or changing the Phase 53 boundary.

## Requirement Verdicts

| Requirement | Verdict | Evidence |
| --- | --- | --- |
| `OBS-02` | `passed` | `open-bitcoind` preflight wording now says durable-store preflight opened and enabled startup runs the explicit opt-in bounded mainnet sync worker while preserving non-claims for unattended production-node operation and packaged-service readiness. |
| `OBS-03` | `passed` | Support bundles now summarize schema v2 nested live-smoke `result` fields, preserve top-level fallback behavior, and keep raw live-smoke inputs, daemon tails, raw snapshots, options, and endpoint tables out of JSON and Markdown. |

## Audit Debt Closure

| Debt | Status | Evidence |
| --- | --- | --- |
| D-02 Support bundle schema v2 summary shallow | Closed | `LIVE_SMOKE_RESULT_SUMMARY_KEYS`, nested-result-first extraction, named Markdown fields, deterministic schema v2/fallback/missing-report support tests. |
| D-04 `open-bitcoind` preflight message stale | Closed | `daemon_sync_preflight_message` helper, exact unit assertions, stale phrase absent from current source. |
| D-01 Phase 44 optional public-network UAT skipped | Still pending | Owned by Phase 53; no public-network UAT was rerun in Phase 52. |
| D-03 Historical Phase 50 selected report caveat | Still pending | Owned by Phase 53; Phase 52 adds amendments but does not replace historical live artifacts. |

## Commands Run

| Command | Result | Notes |
| --- | --- | --- |
| `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli open_bitcoin_support_bundle --test operator_binary` | passed | Proved schema v2 support summary, redaction, fallback, and missing-report behavior. |
| `rg -n "LIVE_SMOKE_RESULT_SUMMARY_KEYS\|live_smoke_summary_from_result\|live_smoke_summary_from_top_level" packages/open-bitcoin-cli/src/operator/support.rs` | passed | Confirmed support extraction helpers and result key allowlist. |
| `rg -n "Progress detected\|No-progress cause\|Header delta\|Block delta" packages/open-bitcoin-cli/src/operator/support/render.rs` | passed | Confirmed named Markdown summary labels. |
| `rg -n "open_bitcoin_support_bundle_summarizes_schema_v2_live_smoke_result\|open_bitcoin_support_bundle_preserves_top_level_live_smoke_fallback\|open_bitcoin_support_bundle_keeps_missing_live_smoke_report_unavailable" packages/open-bitcoin-cli/tests/operator_binary.rs` | passed | Confirmed required support-bundle test names. |
| `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --bin open-bitcoind enabled_sync_preflight_message_describes_opt_in_worker_without_production_claim` | passed | Proved exact preflight message contract. |
| `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --bin open-bitcoind enabled_sync_preflight` | passed | Proved enabled preflight behavior and message test together. |
| `rg -n "daemon_sync_preflight_message\|opened durable store\|explicit opt-in bounded mainnet sync worker\|not unattended production-node operation\|not a packaged-service guarantee" packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs` | passed | Confirmed helper and required wording. |
| `bash -lc '! rg -n "peer transport and unattended full IBD are not started by this phase" packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs'` | passed | Confirmed stale phrase absent from current source. |
| `rg -n "result\.status\|result\.progressDetected\|result\.maybeNoProgressCause\|result\.nextAction\|result\.headerDelta\|result\.blockDelta\|raw live-smoke input\|daemon stdout/stderr tails\|endpoint tables" docs/operator/runtime-guide.md docs/parity/release-readiness.md` | passed | Confirmed docs name schema v2 fields and raw-evidence exclusions. |
| `rg -n "Phase 52 Support Summary Amendment\|Phase 52 Preflight Wording Amendment" .planning/phases/50-public-mainnet-progress-evidence-closeout/50-UAT.md` | passed | Confirmed historical Phase 50 amendment sections. |
| `rg -n "D-02.*Complete\|D-04.*Complete\|D-01.*Pending\|D-03.*Pending" .planning/REQUIREMENTS.md` | passed | Confirmed D-02/D-04 complete and D-01/D-03 pending. |
| `cargo fmt --manifest-path packages/Cargo.toml --all` | passed | Rust formatting completed. |
| `bun run scripts/check-v1.3-release-boundaries.ts` | passed | Release-boundary parity roots remain valid. |
| `bash -lc '! rg -n "run-live-mainnet-smoke" scripts/verify.sh'` | passed | Default verification remains public-network-free. |
| `bun run scripts/generate-loc-report.ts --source=worktree --output=docs/metrics/lines-of-code.md` | passed | Refreshed tracked LOC report after the first verification attempt found it stale. |
| `bash scripts/verify.sh` | passed | Completed in 17m 50.874s after LOC refresh. |

## Residual Risk

Phase 52 is deterministic cleanup only. It does not rerun live public-network
evidence, does not claim new header/block progress, and does not close Phase 53
D-01 or D-03.
