---
phase: 40
phase_name: "Live Mainnet Smoke, Docs, and Parity Closeout"
generated_by: gsd-verify-work
lifecycle_mode: yolo
phase_lifecycle_id: "40-2026-05-02T13-22-45"
generated_at: "2026-05-02T13:48:15.604Z"
status: passed
lifecycle_validated: true
---

# Phase 40 Verification

## Result

Passed. Phase 40 now ships an explicit opt-in live-mainnet smoke command, refreshed operator and parity closeout docs, and machine-readable release-readiness updates while keeping the default local verification contract hermetic. The live smoke command was exercised locally and wrote a no-progress report with explicit zero-outbound-peer guidance, which validated the failure-path prerequisite messaging for an environment that could not establish mainnet peers during the sampled window.

## Commands

| Command | Result | Notes |
| --- | --- | --- |
| `bash scripts/test-run-live-mainnet-smoke.sh` | Passed | Offline regression proved successful report generation and clear preflight failure output without requiring public-network access. |
| `cargo fmt --manifest-path packages/Cargo.toml --all` | Passed | Workspace formatting stayed clean on the final tree. |
| `cargo clippy --manifest-path packages/Cargo.toml --all-targets --all-features -- -D warnings` | Passed | Workspace linting stayed clean. |
| `cargo build --manifest-path packages/Cargo.toml --all-targets --all-features` | Passed | Full workspace build succeeded. |
| `cargo test --manifest-path packages/Cargo.toml --all-features` | Passed | Full workspace tests and doctests passed, including the existing opt-in live sync guard. |
| `bun run scripts/run-live-mainnet-smoke.ts --datadir=/tmp/open-bitcoin-mainnet-smoke --timeout-seconds=60 --poll-seconds=5` | Passed with environment-limited evidence | The command completed end to end and wrote JSON/Markdown reports. The sampled environment never established outbound peers, so the report recorded a clear `no_progress` outcome with actionable DNS/TCP or peer-config guidance instead of failing opaquely. |
| `bash scripts/verify.sh` | Passed | Final repo-native verification passed on the finished tree. |
| `git diff --check` | Passed | No whitespace or conflict-marker issues remain. |

## Evidence

- [`scripts/run-live-mainnet-smoke.ts`](/Users/peterryszkiewicz/Repos/open-bitcoin/scripts/run-live-mainnet-smoke.ts) is the shipped opt-in live-mainnet smoke runner.
- [`scripts/test-run-live-mainnet-smoke.sh`](/Users/peterryszkiewicz/Repos/open-bitcoin/scripts/test-run-live-mainnet-smoke.sh) protects the smoke runner's success and failure-path behavior offline.
- [`packages/target/live-mainnet-smoke-reports/open-bitcoin-live-mainnet-smoke.json`](/Users/peterryszkiewicz/Repos/open-bitcoin/packages/target/live-mainnet-smoke-reports/open-bitcoin-live-mainnet-smoke.json) and [`packages/target/live-mainnet-smoke-reports/open-bitcoin-live-mainnet-smoke.md`](/Users/peterryszkiewicz/Repos/open-bitcoin/packages/target/live-mainnet-smoke-reports/open-bitcoin-live-mainnet-smoke.md) record the sampled local no-progress run and the explicit zero-outbound-peer guidance.
- [`README.md`](/Users/peterryszkiewicz/Repos/open-bitcoin/README.md), [`docs/operator/runtime-guide.md`](/Users/peterryszkiewicz/Repos/open-bitcoin/docs/operator/runtime-guide.md), and [`docs/parity/release-readiness.md`](/Users/peterryszkiewicz/Repos/open-bitcoin/docs/parity/release-readiness.md) now describe the shipped live-smoke workflow and its bounded non-claims.
- [`docs/parity/checklist.md`](/Users/peterryszkiewicz/Repos/open-bitcoin/docs/parity/checklist.md) and [`docs/parity/index.json`](/Users/peterryszkiewicz/Repos/open-bitcoin/docs/parity/index.json) now track `live-mainnet-smoke-closeout` as a completed, auditable surface.

## Residual Risks

- The local smoke evidence in this session validated the explicit failure path, not a successful public-mainnet peer connection, because the sampled environment never established outbound peers within the 60-second window.
- The live smoke command remains intentionally opt-in and outside `bash scripts/verify.sh`.
- Production-node, production-funds, packaged-service, and stronger release-gate claims remain out of scope after this phase.
