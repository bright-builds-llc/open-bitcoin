---
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: 64-2026-06-08T03-22-46
generated_at: 2026-06-08T03:22:46.768Z
---

# Phase 64: Service Restart and Same-Datadir Resume Evidence - Context

**Gathered:** 2026-06-07
**Status:** Ready for planning
**Mode:** Yolo

<domain>

## Phase Boundary

Phase 64 proves that the Phase 63 launchd/systemd supervision workflow can
restart the explicit opt-in `open-bitcoind` unattended mainnet review daemon
against the same Open Bitcoin datadir and expose truthful resume evidence. It
extends Phase 58 same-datadir restart proof from script-managed daemon relaunch
into the service-supervised operator path.

This phase owns deterministic service-restart fixtures, restart/resume status
projection, clean versus unclean prior-shutdown evidence, stale in-flight work
reset behavior, service restart UAT reporting, and operator documentation for
reviewing same-datadir service restarts. It does not own production service
claims, packaging, Windows service integration, inbound serving, transaction
relay, production-funds wallet use, migration apply mode, support-bundle
expansion, compatibility harness wrapping, or making public-network or real
service-manager checks part of default verification.

</domain>

<decisions>

## Implementation Decisions

### Service Restart Evidence Contract

- **D-01:** Add a service-scoped restart/resume evidence contract that reuses
  the Phase 58 same-datadir concepts and the Phase 63 service lifecycle labels.
  The operator-visible evidence should identify the service manager, service
  lifecycle state, datadir identity, prior shutdown classification, persisted
  progress before and after restart, restart action outcome, stale in-flight
  cleanup verdict, and next-action guidance.
- **D-02:** Keep the contract additive and explicit. Existing status and
  restart fields must not be removed or renamed; unavailable manager, missing
  service state, and absent durable sync state should render as explicit
  `Unavailable` evidence rather than false success.
- **D-03:** Treat clean versus unclean prior shutdown as a first-class status
  field for service restart review. Clean service stop/restart should be
  distinguishable from interrupted or unclean daemon exit, and both paths must
  preserve durable resume evidence.
- **D-04:** Service restart evidence should not require fresh public-network
  progress after restart. Preserved same-datadir durable state plus bounded,
  typed next-action guidance is acceptable; fresh progress is stronger optional
  UAT evidence when available.

### Same-Datadir Resume Safety

- **D-05:** Deterministic tests must prove that service restart uses the same
  configured Open Bitcoin datadir and reopens the same durable sync state. Do
  not infer same-datadir safety from service file text alone; status or fixture
  evidence must tie the restart path to persisted runtime metadata.
- **D-06:** Same-datadir restart tests should cover extended-loop resume
  behavior without duplicate block requests, duplicate block connects, corrupted
  active chainstate, or lost progress counters. Reuse Phase 58 durable reopen
  fixtures where they already prove core behavior, and add service-level tests
  for the service-supervised path.
- **D-07:** Restart must clear or ignore stale in-flight requests from the prior
  daemon session while preserving durable downloaded and connected block
  progress. The evidence should distinguish "no stale in-flight requests
  resumed" from "no progress existed."
- **D-08:** Storage recovery categories from Phase 61 keep precedence over peer
  retry guidance. Store corruption, incompatible schema, lock contention, and
  resource exhaustion should drive next-action guidance before peer-level
  retry advice.

### Operator Surfaces

- **D-09:** `open-bitcoin service restart` and service status output should
  preserve Phase 63 lifecycle truth while adding restart/resume evidence where
  it exists. The dashboard and JSON status surfaces should agree with CLI
  status on service lifecycle, datadir, prior shutdown, recovery category, and
  resume guidance.
- **D-10:** Real launchd/systemd start, stop, and restart operations remain
  operator-initiated. Default verification should use fake managers, pure
  renderers, deterministic Rust fixtures, and Bun checkers; it must not run
  `systemctl --user restart`, `launchctl kickstart`, or public-mainnet live
  smoke.
- **D-11:** UAT reports may document opt-in launchd/systemd restart review
  commands and the Phase 58 `--restart-after-progress` live-smoke path, but
  they must clearly label those commands as optional operator evidence outside
  `bash scripts/verify.sh`.
- **D-12:** Operator guidance should be concrete and copy-pasteable with
  repo-local Cargo and Bazel command forms, including service status, restart,
  JSON status review, and pass/fail interpretation.

### Documentation And Release Boundary

- **D-13:** Docs and parity notes should describe Phase 64 as service-supervised
  same-datadir restart evidence for extended operator review. Avoid broad
  production-node, packaged-service, uptime, or unattended production claims.
- **D-14:** If deterministic checker coverage is added, wire it into
  `scripts/verify.sh` and keep it focused on source/docs contracts and default
  verification boundaries. Generated LOC evidence should be refreshed through
  the repo-owned generator if verification updates change tracked metrics.

### the agent's Discretion

- The planner may split work across durable runtime tests, service/status
  rendering, operator docs/checkers, and parity evidence if each plan remains
  reviewable.
- The executor may add small pure helpers for restart/resume evidence mapping
  when that keeps service adapters thin and status surfaces consistent.
- If new first-party Rust source or test files are added under
  `packages/open-bitcoin-*/src` or `packages/open-bitcoin-*/tests`, the executor
  must update parity breadcrumbs before committing.

</decisions>

<canonical_refs>

## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Phase Scope

- `.planning/ROADMAP.md` - Phase 64 goal, dependency on Phase 63, SVC-03 and
  RR-03 success criteria, and deferred Phase 65 through Phase 67 boundaries.
- `.planning/REQUIREMENTS.md` - SVC-03, RR-03, OBS-04, REL-03, and v1.5
  out-of-scope boundaries.
- `.planning/PROJECT.md` - v1.5 milestone goal, parity baseline, functional-core
  boundary, and production-claim limits.
- `.planning/STATE.md` - Active milestone state and prior decisions about
  deterministic verification and opt-in public-network evidence.

### Prior Phase Decisions And Evidence

- `.planning/phases/63-service-supervision-lifecycle/63-CONTEXT.md` - Service
  command surface, lifecycle labels, launchd/systemd behavior, and docs
  boundaries that Phase 64 extends.
- `.planning/phases/63-service-supervision-lifecycle/63-VERIFICATION.md` -
  Passed evidence for Phase 63 service lifecycle management.
- `.planning/phases/58-same-datadir-restart-and-resume-evidence/58-CONTEXT.md`
  - Prior same-datadir restart/resume decisions, evidence schema, and recovery
  diagnosis boundaries.
- `.planning/phases/58-same-datadir-restart-and-resume-evidence/58-VERIFICATION.md`
  - Passed evidence for durable resume, no duplicate connected block work, and
  opt-in restart live-smoke reporting.
- `.planning/phases/62-long-run-sync-truth-surfaces/62-CONTEXT.md` - Shared
  status/dashboard/RPC/log truth fields that restart evidence should preserve.
- `.planning/phases/61-resource-bounds-and-recovery-taxonomy/61-CONTEXT.md` -
  Recovery category and next-action guidance precedence.

### Implementation Surfaces

- `packages/open-bitcoin-node/src/sync.rs` - `DurableSyncRuntime` reopen, sync
  loop state, progress persistence, and durable status writes.
- `packages/open-bitcoin-node/src/sync/tests.rs` - Existing same-datadir,
  restart, block reconnect, and peer-failure fixtures.
- `packages/open-bitcoin-node/src/status.rs` - Shared operator sync status and
  durable sync state exposed to CLI/RPC surfaces.
- `packages/open-bitcoin-node/src/storage.rs` - Runtime metadata, storage
  recovery actions, schema errors, and shutdown/recovery markers.
- `packages/open-bitcoin-cli/src/operator/service.rs` - Service command
  dispatch, lifecycle state, command outcomes, and status rendering.
- `packages/open-bitcoin-cli/src/operator/service/fake.rs` - Deterministic fake
  service manager for service command tests.
- `packages/open-bitcoin-cli/src/operator/service/launchd.rs` - launchd user
  service generation, restart command integration, and status parsing.
- `packages/open-bitcoin-cli/src/operator/service/systemd.rs` - systemd user
  unit generation, restart command integration, and status parsing.
- `packages/open-bitcoin-cli/src/operator/service/tests.rs` - Service lifecycle
  command and rendering tests.
- `packages/open-bitcoin-cli/src/operator/status.rs` - Operator status
  collection and service/sync projection.
- `packages/open-bitcoin-cli/src/operator/status/render.rs` - Human and JSON
  status rendering.
- `packages/open-bitcoin-cli/src/operator/dashboard/action.rs` - Dashboard
  service action dispatch.
- `scripts/run-live-mainnet-smoke.ts` - Existing opt-in two-session restart
  evidence path and report schema.
- `scripts/test-run-live-mainnet-smoke.sh` - Deterministic live-smoke restart
  fixture checks.
- `docs/operator/runtime-guide.md` - Operator service and restart/resume
  runbook.
- `docs/parity/catalog/p2p.md` - Public-network and restart/resume parity
  evidence boundaries.

</canonical_refs>

<code_context>

## Existing Code Insights

- Phase 63 established `ServiceManager`, fake service managers, service command
  outcomes, lifecycle labels, dashboard service actions, and deterministic
  service lifecycle verification. Phase 64 should extend these surfaces instead
  of adding a second service abstraction.
- Phase 58 already proves core durable same-datadir restart safety in
  `DurableSyncRuntime` fixtures and exposes compact opt-in restart evidence in
  the live-smoke script. Phase 64 should reuse that core evidence and add the
  service-supervised operator path.
- Phase 61 recovery categories and Phase 62 truth surfaces should remain the
  canonical recovery/status vocabulary. Restart-specific code should map into
  those contracts rather than inventing new labels.
- Default verification must stay deterministic. Real service-manager commands
  and public-mainnet smoke runs belong in optional UAT docs, not in
  `scripts/verify.sh`.

</code_context>

<deferred>

## Deferred Ideas

- Support bundle expansion for service restart evidence belongs to Phase 65.
- Compatibility harness operator wrapping belongs to Phase 66.
- v1.5 release-boundary closeout belongs to Phase 67.
- Windows service integration, signed packages, inbound serving, transaction
  relay, compact block relay, production-funds wallet use, migration apply mode,
  hosted dashboards, GUI work, and broad production-node claims remain out of
  scope for v1.5.

</deferred>

---

*Phase: 64-service-restart-and-same-datadir-resume-evidence*
*Context gathered: 2026-06-07 via yolo discussion*
