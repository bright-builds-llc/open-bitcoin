---
phase: 40
phase_name: "Live Mainnet Smoke, Docs, and Parity Closeout"
plan_id: "40-01"
generated_by: gsd-execute-phase
lifecycle_mode: yolo
phase_lifecycle_id: "40-2026-05-02T13-22-45"
generated_at: "2026-05-02T13:48:15.604Z"
status: completed
---

# Summary 40-01: Opt-In Live Mainnet Evidence And Closeout Truth

## Completed

- Added [`scripts/run-live-mainnet-smoke.ts`](/Users/peterryszkiewicz/Repos/open-bitcoin/scripts/run-live-mainnet-smoke.ts), a repo-owned Bun runner that builds the daemon and CLI binaries, launches `open-bitcoind` in explicit `mainnet-ibd` mode, probes `getblockchaininfo`, captures the post-run durable sync snapshot, and writes JSON plus Markdown evidence reports under `packages/target/live-mainnet-smoke-reports`.
- Added [`scripts/test-run-live-mainnet-smoke.sh`](/Users/peterryszkiewicz/Repos/open-bitcoin/scripts/test-run-live-mainnet-smoke.sh) so the smoke runner keeps offline regression coverage for successful report generation and clear preflight failure messaging.
- Refreshed [`README.md`](/Users/peterryszkiewicz/Repos/open-bitcoin/README.md), [`docs/operator/runtime-guide.md`](/Users/peterryszkiewicz/Repos/open-bitcoin/docs/operator/runtime-guide.md), [`docs/parity/checklist.md`](/Users/peterryszkiewicz/Repos/open-bitcoin/docs/parity/checklist.md), [`docs/parity/index.json`](/Users/peterryszkiewicz/Repos/open-bitcoin/docs/parity/index.json), [`docs/parity/release-readiness.md`](/Users/peterryszkiewicz/Repos/open-bitcoin/docs/parity/release-readiness.md), and [`docs/parity/deviations-and-unknowns.md`](/Users/peterryszkiewicz/Repos/open-bitcoin/docs/parity/deviations-and-unknowns.md) so the shipped v1.2 surface now records the explicit live-smoke flow and its still-deferred production claims.
- Ran the new live smoke command against `/tmp/open-bitcoin-mainnet-smoke`, which generated a local no-progress report with explicit zero-outbound-peer guidance instead of failing opaquely.

## Tests Added

- `bash scripts/test-run-live-mainnet-smoke.sh`

## Residual Risks

- The live smoke command remains environment-dependent: the local scratch run in this session produced a report with `0` outbound peers and no observed header or block movement, so successful live progress still depends on reachable mainnet DNS/TCP paths.
- The new live evidence path is intentionally opt-in and remains outside `bash scripts/verify.sh`.
- This closeout phase does not widen Open Bitcoin into a production-node, production-funds, or packaged-service claim.
