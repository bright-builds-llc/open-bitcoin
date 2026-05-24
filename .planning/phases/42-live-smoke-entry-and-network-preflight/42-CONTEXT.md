---
generated_by: gsd-discuss-phase
lifecycle_mode: interactive
phase_lifecycle_id: 42-2026-05-24T13-40-48
generated_at: 2026-05-24T13:40:48.450Z
---

# Phase 42: Live Smoke Entry and Network Preflight - Context

**Gathered:** 2026-05-24
**Status:** Ready for planning
**Mode:** Recommended Review

<domain>
## Phase Boundary

Phase 42 makes the opt-in live-mainnet smoke entrypoint and network preflight
truthful enough that an operator can tell whether the run is blocked before
sync starts, blocked while trying to reach public peers, or making it far enough
to collect durable status evidence. This phase owns invocation, prerequisite
checks, endpoint outcome reporting, and no-progress diagnosis for the live-smoke
path.

This phase does not own later peer rotation/backoff policy, peer contribution
attribution, runtime resource-bound hardening, restart/resume proof, support
bundle generation, security closeout, or final public-mainnet progress evidence.
Those are covered by Phases 43 through 50.

</domain>

<decisions>
## Implementation Decisions

### Invocation Contract

- **D-01:** Keep `scripts/run-live-mainnet-smoke.ts` as the Phase 42 operator
  entrypoint. Extend the existing Bun script rather than creating a new binary
  or moving this first slice into `open-bitcoin`.
- **D-02:** The smoke command should continue to require an explicit datadir and
  remain opt-in. Manual-peer/config inputs should be explicit and visible in the
  report so a reviewer can reproduce the attempted run.
- **D-03:** Preserve the current local report output directory,
  `packages/target/live-mainnet-smoke-reports`, and extend the existing JSON and
  Markdown schema in place instead of introducing a second artifact format.

### Preflight Coverage

- **D-04:** Preflight should keep existing local checks for datadir, optional
  config path, system clock, disk floor, and command availability.
- **D-05:** Add network preflight outcome reporting for DNS seeds and manual
  peers. Operators need to see whether endpoints were resolved, connected,
  handshook, failed, or skipped.
- **D-06:** Manual peers should be treated as the preferred operator escape
  hatch when DNS or outbound access is blocked. The report should make the
  manual-peer path visible enough for UAT to retry without guessing where to put
  the peer list.

### No-Progress Diagnosis

- **D-07:** Replace the current broad `0 outbound peers` explanation with typed
  no-progress causes when evidence is available: DNS resolution failure, TCP
  connection failure, handshake failure, unsupported peer capability, validation
  failure, storage failure, timeout, and operator cancellation.
- **D-08:** Preserve operator cancellation as its own outcome rather than
  collapsing it into runtime failure or timeout. A cancelled smoke run should
  still write useful partial evidence.
- **D-09:** If the phase cannot yet prove header or block progress, the output
  should still tell the operator the next action: fix local prerequisites, fix
  outbound DNS/TCP, provide manual peers, increase timeout, or inspect daemon
  stderr/status evidence.

### Evidence Output

- **D-10:** The JSON report is the machine-readable contract. Markdown should
  remain a readable rendering of the same facts and avoid adding claims that are
  absent from JSON.
- **D-11:** Report tables should make endpoint attempts and final durable status
  easy to inspect. The existing snapshot table is not enough for Phase 42
  because it can show repeated zero-height rows without explaining the network
  path failure.
- **D-12:** The report must stay local-review oriented and threshold-free. Phase
  42 should not add elapsed-time pass/fail gates or require checked-in live
  report fixtures.

### Verification Posture

- **D-13:** Default verification should use deterministic tests around argument
  parsing, preflight classification, report serialization, and injected
  resolver/transport outcomes. Do not add public-network access to
  `bash scripts/verify.sh`.
- **D-14:** Live UAT remains copy-pasteable and repo-local. When commands are
  shown to the user, prefer explicit `bun run`, Cargo, or Bazel commands rather
  than assuming an installed `open-bitcoin` alias.

### the agent's Discretion

The planner may decide whether endpoint preflight is modeled as a separate
internal helper inside `scripts/run-live-mainnet-smoke.ts` or as a small
repo-owned TypeScript module under `scripts/` if that materially improves test
coverage and keeps the script readable. The planner may also choose exact JSON
field names, as long as they are stable, typed, and directly support the
requirements above.

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Phase Scope

- `.planning/ROADMAP.md` - Phase 42 goal, requirements, dependencies, and
  success criteria.
- `.planning/REQUIREMENTS.md` - Phase 42 requirement text for PROOF-01,
  PROOF-02, and PEER-01.
- `.planning/PROJECT.md` - v1.3 milestone goal, active scope, out-of-scope
  boundaries, and key decisions.

### Existing Live-Smoke Surface

- `scripts/run-live-mainnet-smoke.ts` - Current Bun live-mainnet smoke runner,
  preflight checks, daemon launch, status polling, JSON/Markdown report schema,
  and no-progress messages.
- `scripts/test-run-live-mainnet-smoke.sh` - Existing smoke-run regression
  wrapper for deterministic coverage.
- `packages/target/live-mainnet-smoke-reports/open-bitcoin-live-mainnet-smoke.md`
  - Latest local UAT report showing the current `no_progress` / `0 outbound
  peers` failure mode that Phase 42 is meant to make more actionable.

### Operator And Parity Boundaries

- `docs/operator/runtime-guide.md` - Mainnet sync activation, manual peers,
  target outbound peer setting, sync status controls, live-smoke behavior, and
  current limitations.
- `docs/parity/release-readiness.md` - Evidence-based readiness posture and
  public-network/live-smoke boundaries.
- `docs/parity/deviations-and-unknowns.md` - Deferred public-network,
  production-node, packaging, wallet, migration, and relay surfaces.
- `docs/parity/checklist.md` - Current live-mainnet smoke closeout and security
  closeout evidence boundaries.

### Runtime Integration Points

- `packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs` - Daemon sync worker
  startup, durable state seeding, pause handling, and sync loop integration.
- `packages/open-bitcoin-node/src/sync.rs` - Durable sync runtime, candidate
  peer resolution, target outbound peer handling, peer outcome recording, and
  durable sync state persistence.
- `packages/open-bitcoin-node/src/sync/progress.rs` - Peer progress,
  contribution, stalled-peer signal, and outcome conversion helpers.
- `packages/open-bitcoin-cli/src/operator/runtime/support.rs` - Operator
  `sync status|pause|resume` rendering and live/offline sync control behavior.

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets

- `scripts/run-live-mainnet-smoke.ts` already has typed `Options`,
  `PreflightCheck`, `SmokeReport`, `SyncStatusSnapshot`, and report rendering
  helpers. Phase 42 should evolve these rather than introduce ad hoc strings.
- `DurableSyncRuntime` already records `PeerSyncOutcome`,
  `PeerFailureReason`, `PeerContribution`, resolved endpoint, attempts,
  capabilities, and durable sync summaries. The live-smoke runner should surface
  this information where it is already persisted or make the missing projection
  explicit for planning.
- `docs/operator/runtime-guide.md` already documents JSONC manual peers,
  DNS seed overrides, and `target_outbound_peers`. Phase 42 can link or refresh
  that guidance instead of inventing new operator terminology.

### Established Patterns

- Public-network checks are opt-in review evidence and stay outside
  `bash scripts/verify.sh`.
- Operator-facing automation scripts in this repo use Bun/TypeScript for
  substantial logic and Bash only for thin wrappers.
- Reports under `packages/target/` are generated local evidence, not checked-in
  release fixtures.
- Operator docs should provide repo-local commands and avoid depending on an
  installed alias.

### Integration Points

- The smoke runner starts `open-bitcoind` with `-openbitcoinsync=mainnet-ibd`,
  polls `open-bitcoin-cli getblockchaininfo`, then reads final durable status
  through `open-bitcoin --format json sync status`.
- Phase 42 needs a cleaner bridge between endpoint/network attempt outcomes and
  the smoke report. That may come from daemon stderr/status, durable sync state,
  or a deterministic preflight helper, but the report must make the source of
  each outcome clear.
- Existing status snapshots only show header height, block height, outbound
  peers, phase, lifecycle, and last error. They do not currently explain why no
  peer was reached.

</code_context>

<specifics>
## Specific Ideas

- The latest UAT failure is the anchor case: preflight passed locally, the run
  timed out, final durable sync status showed `0 outbound peers`, and the daemon
  stderr noted mainnet sync preflight. Phase 42 should make that case produce
  a more precise next action.
- Manual peer support should be a first-class review path because it lets an
  operator distinguish DNS problems from broader outbound TCP or handshake
  problems.
- The Markdown report should remain useful to paste into a review thread, but
  the JSON report should be the authoritative artifact for tests and future
  tooling.

</specifics>

<deferred>
## Deferred Ideas

- Peer rotation/backoff and runtime survival under mixed public-peer failures
  belong to Phase 43.
- Per-peer contribution attribution belongs to Phase 44.
- Runtime resource bounds, durable store coordination, and pause/resume
  hardening belong to Phase 45.
- Restart/resume proof and invalid-data durable recovery belong to Phase 46.
- Final public-mainnet header/block/restart evidence belongs to Phase 50.

</deferred>

---

*Phase: 42-live-smoke-entry-and-network-preflight*
*Context gathered: 2026-05-24*
