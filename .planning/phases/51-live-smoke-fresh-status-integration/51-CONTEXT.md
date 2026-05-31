---
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: 51-2026-05-31T21-21-57
generated_at: 2026-05-31T21:25:05.455Z
---

# Phase 51: Live Smoke Fresh Status Integration - Context

**Gathered:** 2026-05-31
**Status:** Ready for planning
**Mode:** Yolo

<domain>
## Phase Boundary

Phase 51 closes v1.3 milestone audit gap G-01 by making the opt-in
live-mainnet smoke runner poll fresh daemon sync-control status during the run.
It amends the Phase 50 evidence trail so auditors can see that progress and
diagnosed-blocker snapshots now come from the same live daemon truth surface as
final durable status. This phase does not add public-network checks to
`bash scripts/verify.sh`, broaden production-node claims, or rerun broad release
documentation work outside the affected evidence path.

</domain>

<decisions>
## Implementation Decisions

### Fresh Status Source

- **D-01:** The live-smoke polling loop must call RPC
  `openbitcoinsyncstatus` through `open-bitcoin-cli` instead of
  `getblockchaininfo`.
- **D-02:** Snapshot parsing must consume the Open Bitcoin sync-control
  response shape (`metadata.maybe_sync_state` plus `metadata.sync_control`) and
  may also tolerate raw runtime metadata for test overrides.
- **D-03:** Snapshot fields must be derived from fresh runtime metadata:
  lifecycle, phase, outbound peer count, latest error, paused state,
  `updated_at_unix_seconds`, header height, and connected block height.
- **D-04:** Final status stays on the operator CLI
  `open-bitcoin sync status --format json`; both per-poll snapshots and final
  status should now read equivalent durable runtime metadata shapes.

### Deterministic Proof

- **D-05:** Keep public-network smoke execution opt-in and outside
  `bash scripts/verify.sh`.
- **D-06:** Update the existing offline `scripts/test-run-live-mainnet-smoke.sh`
  regression so its mocked status command returns fresh sync-control metadata.
  The test must prove both progress detection and no-progress diagnosis use the
  fresh status fields instead of hardcoded `rpc_getblockchaininfo` values.
- **D-07:** The regression should assert status command recording includes
  `openbitcoinsyncstatus`, snapshots carry runtime lifecycle/phase/outbound
  peers, and progress deltas are computed from metadata-backed heights.

### Evidence Amendment

- **D-08:** Amend Phase 50 UAT rather than checking in generated live-smoke
  reports. The amendment should preserve the historical Phase 50 selected
  report, explicitly name its stale snapshot mismatch, and point to Phase 51
  runner/tests as the fix that future live-smoke reruns use.
- **D-09:** Refresh operator/parity docs only where they describe the smoke
  runner polling source or the v1.3 evidence closeout status.
- **D-10:** Mark PROOF-03, PROOF-04, PROOF-05, OBS-02, and SEC-03 complete only
  after the fresh-status runner, offline regression, docs amendment, and repo
  verification pass.

### the agent's Discretion

The planner may choose exact helper names and type aliases in
`scripts/run-live-mainnet-smoke.ts`, provided the parser remains boundary-based,
uses `maybe...` names for nullable internals, and keeps the script dependency
surface to Bun/Node standard libraries.

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Phase Scope And Audit Gap

- `.planning/ROADMAP.md` - Phase 51 goal, requirements, and success criteria.
- `.planning/REQUIREMENTS.md` - PROOF-03, PROOF-04, PROOF-05, OBS-02, and
  SEC-03 requirement text and traceability.
- `.planning/v1.3-MILESTONE-AUDIT.md` - G-01 evidence, impact, and close
  criteria.

### Prior Phase Decisions

- `.planning/phases/47-operator-sync-truth-surfaces/47-CONTEXT.md` - Shared
  status truth and conservative `getblockchaininfo` decisions.
- `.planning/phases/50-public-mainnet-progress-evidence-closeout/50-CONTEXT.md`
  - Live-smoke evidence strategy, public-network opt-in posture, and UAT
  expectations.
- `.planning/phases/50-public-mainnet-progress-evidence-closeout/50-UAT.md` -
  Historical selected Phase 50 diagnosed-blocker evidence that needs amendment.

### Runtime And Evidence Surfaces

- `scripts/run-live-mainnet-smoke.ts` - Opt-in live-smoke runner and JSON/MD
  report schema.
- `scripts/test-run-live-mainnet-smoke.sh` - Offline regression for the smoke
  runner.
- `packages/open-bitcoin-rpc/src/method.rs` - RPC method registry including
  `openbitcoinsyncstatus`.
- `packages/open-bitcoin-rpc/src/dispatch/node.rs` - Fresh sync-control RPC
  handler.
- `packages/open-bitcoin-rpc/src/context.rs` - Daemon sync-control status load
  path.
- `packages/open-bitcoin-node/src/storage.rs` - Runtime metadata shape.
- `docs/operator/runtime-guide.md` - Operator-facing runtime and live-smoke
  documentation.
- `docs/parity/checklist.md`, `docs/parity/index.json`, and
  `docs/parity/release-readiness.md` - v1.3 parity and evidence closeout roots.

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets

- `readFinalStatus` in `scripts/run-live-mainnet-smoke.ts` already parses raw
  runtime metadata into lifecycle, phase, outbound peer count, latest error,
  progress heights, and peer telemetry.
- `open_bitcoin_sync_status` in `packages/open-bitcoin-rpc/src/dispatch/node.rs`
  returns `OpenBitcoinSyncControlResponse { metadata }` by reading daemon
  runtime metadata through `ManagedRpcContext::daemon_sync_status`.
- `scripts/test-run-live-mainnet-smoke.sh` already uses mocked daemon/status
  commands and generated reports, making it the right deterministic proof
  surface.

### Established Patterns

- Repo-owned automation uses Bun TypeScript and shell wrappers; do not add
  Python or new package dependencies.
- Generated live-smoke/support artifacts remain under `packages/target` or an
  explicit operator-selected directory and are not checked into git.
- Default repo verification must stay public-network-free.

### Integration Points

- The live-smoke status command is assembled in
  `scripts/run-live-mainnet-smoke.ts` before daemon launch.
- Per-poll snapshots feed `result.headerDelta`, `result.blockDelta`,
  `result.progressDetected`, diagnosed-blocker messages, JSON report snapshots,
  and Markdown report rows.
- Phase closeout must update GSD lifecycle artifacts, Phase 50 UAT amendment,
  parity roots, requirements traceability, and roadmap status.

</code_context>

<specifics>
## Specific Ideas

- Prefer the fresh RPC method path (`openbitcoinsyncstatus`) over refreshing
  `getblockchaininfo` per request because the sync-control path already exists
  and is explicitly daemon-owned.
- Preserve compatibility with raw runtime metadata in test overrides so shell
  fixtures can stay small and the parser remains useful for both live RPC and
  operator status output shapes.

</specifics>

<deferred>
## Deferred Ideas

None - discussion stayed within phase scope.

</deferred>

---

*Phase: 51-live-smoke-fresh-status-integration*
*Context gathered: 2026-05-31*
