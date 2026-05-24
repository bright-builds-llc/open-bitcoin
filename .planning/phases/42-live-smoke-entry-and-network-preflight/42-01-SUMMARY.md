---
phase: 42
phase_name: "Live Smoke Entry and Network Preflight"
plan_id: "42-01"
generated_by: gsd-execute-phase
lifecycle_mode: interactive
phase_lifecycle_id: "42-2026-05-24T13-40-48"
generated_at: "2026-05-24T14:18:50.544Z"
status: completed
---

# Summary 42-01: Live Smoke Network Preflight And Typed No-Progress Evidence

## Completed

- Extended [`scripts/run-live-mainnet-smoke.ts`](/Users/peterryszkiewicz/Repos/open-bitcoin/scripts/run-live-mainnet-smoke.ts) with repeatable `--manual-peer=HOST[:PORT]`, generated manual-peer JSONC config support, endpoint outcome reporting, typed no-progress causes, next-action guidance, final durable peer telemetry projection, and distinct `cancelled` report status.
- Expanded [`scripts/test-run-live-mainnet-smoke.sh`](/Users/peterryszkiewicz/Repos/open-bitcoin/scripts/test-run-live-mainnet-smoke.sh) to cover successful manual-peer report generation, generated config contents, local preflight failure, deterministic TCP no-progress classification, and SIGTERM cancellation evidence without public-network access.
- Updated [`docs/operator/runtime-guide.md`](/Users/peterryszkiewicz/Repos/open-bitcoin/docs/operator/runtime-guide.md) with the manual-peer command form, generated config behavior, endpoint outcomes, typed no-progress causes, and cancellation semantics.
- Refreshed v1.3 planning state in [`.planning/REQUIREMENTS.md`](/Users/peterryszkiewicz/Repos/open-bitcoin/.planning/REQUIREMENTS.md), [`.planning/ROADMAP.md`](/Users/peterryszkiewicz/Repos/open-bitcoin/.planning/ROADMAP.md), and [`.planning/STATE.md`](/Users/peterryszkiewicz/Repos/open-bitcoin/.planning/STATE.md), and regenerated [`docs/metrics/lines-of-code.md`](/Users/peterryszkiewicz/Repos/open-bitcoin/docs/metrics/lines-of-code.md).

## Tests Added

- `bash scripts/test-run-live-mainnet-smoke.sh`

## Residual Risks

- The endpoint preflight proves DNS/TCP reachability and projects daemon runtime peer telemetry when available; it does not implement a standalone TypeScript Bitcoin handshake. Runtime `handshook` evidence still comes from durable peer status after daemon sync attempts.
- Public-mainnet progress remains environment-dependent and opt-in. The deterministic tests intentionally avoid public-network access.
- `scripts/run-live-mainnet-smoke.ts` is now large because Phase 42 kept the existing entrypoint and report schema in one script; a later cleanup can split parser/preflight/report helpers if more live-smoke functionality lands.
