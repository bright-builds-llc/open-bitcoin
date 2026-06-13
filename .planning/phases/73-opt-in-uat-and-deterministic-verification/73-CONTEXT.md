---
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: 73-2026-06-13T22-08-43
generated_at: 2026-06-13T22:08:43.206Z
---

# Phase 73: Opt-In UAT and Deterministic Verification - Context

**Gathered:** 2026-06-13
**Status:** Ready for planning
**Mode:** Yolo

<domain>

## Phase Boundary

Phase 73 closes the v1.6 verification and operator UAT gap. Contributors must be
able to run `bash scripts/verify.sh` as a deterministic local gate without
internet access, public peers, real service managers, long-running sync, or
current-tip timing. Operators must also get copy-pasteable repo-local commands
for explicit opt-in public-mainnet full-sync, stay-current, restart/resume,
status comparison, live-smoke, and support-bundle review.

This phase owns VER-01 through VER-04 only: hermetic default verification,
deterministic coverage for full-sync durability/recovery behaviors, opt-in UAT
commands, and auditability of parity breadcrumbs, fixtures, compatibility
harness reports, and deterministic checkers. It does not own new sync runtime
behavior unless the coverage audit finds a narrow missing deterministic test. It
does not move public-network checks, real service-manager actions, or live
timing thresholds into `bash scripts/verify.sh`.

</domain>

<decisions>

## Implementation Decisions

### Default Hermetic Verification

- **D-01:** Keep `bash scripts/verify.sh` as the repo-native deterministic
  verification contract. Extend it with a Phase 73 Bun checker only if that
  checker remains local, short-running, public-network-free, service-manager
  free, and timing-stable.
- **D-02:** Follow the existing Phase 61 through Phase 72 checker pattern:
  explicit required files, required test/doc needles, ordered checker wiring,
  and scoped forbidden-default-verification strings.
- **D-03:** The Phase 73 checker must guard against accidental default
  invocation of live-mainnet smoke, manual peers, `--restart-after-progress`,
  real `systemctl` or `launchctl`, `-openbitcoinsync=mainnet-ibd`, and
  current-tip or wall-clock release gates in `scripts/verify.sh`.
- **D-04:** Do not add strict Cargo/Bazel offline flags to the normal
  `scripts/verify.sh` path unless a post-bootstrap offline audit mode is
  deliberately documented. Fresh contributors should not be blocked by cache
  state while the default contract still forbids public-network runtime checks.

### Deterministic Coverage Scope

- **D-05:** Create a Phase 73 coverage map, implemented either inside the Phase
  73 checker or as a small local manifest consumed by it, that maps VER-02 to
  explicit existing or new deterministic tests for durable UTXO/undo writes,
  block connect/disconnect/reorg across restart, best-chain header selection,
  peer response failures, crash recovery, duplicate connect prevention, and
  resource bounds.
- **D-06:** Audit Phase 68 through Phase 72 tests before adding new tests. If a
  VER-02 behavior already has explicit assertions, reference it from the
  coverage map instead of duplicating fixtures.
- **D-07:** Add narrow hermetic Rust gap tests only where the audit finds missing
  explicit coverage. Prefer existing `DurableSyncRuntime`, Fjall temp-store,
  scripted transport, chainstate, block reconcile, and synthetic long-chain
  fixtures over a new process-level crash harness.
- **D-08:** Treat crash recovery as deterministic durable reopen/recovery
  evidence for this phase unless planning proves an actual process-level crash
  harness can be short, hermetic, platform-stable, and worth the added moving
  parts.

### Opt-In Public-Mainnet UAT Commands

- **D-09:** Add a central Phase 73 opt-in UAT matrix in
  `docs/operator/runtime-guide.md`, with links or short pointers from nearby
  workflow sections rather than scattering the authoritative command list.
- **D-10:** Every operator-facing UAT workflow in the matrix should include
  copy-pasteable repo-local Cargo and Bazel command forms where the workflow is
  served by the operator CLI. Use:
  `cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- ...`
  and `bazel run //packages/open-bitcoin-cli:open_bitcoin -- ...`.
- **D-11:** The matrix should cover full-sync activation/review,
  stay-current/status review, same-datadir restart/resume review, status-surface
  comparison, live-smoke report collection, and support-bundle collection.
  `bun run scripts/run-live-mainnet-smoke.ts` remains the repo-owned wrapper for
  live public-mainnet evidence and must be labeled opt-in UAT.
- **D-12:** Command descriptions must state what evidence each workflow proves
  and what it does not prove. Bundle existence, daemon startup, elapsed time, or
  peer reachability alone are not sync-to-tip proof.

### Parity And Evidence Auditability

- **D-13:** Prefer the existing checker-plus-breadcrumb approach for a narrow
  Phase 73 closeout. Introduce a small phase-scoped evidence manifest only if
  planning adds enough non-Rust UAT/report/fixture surfaces that plain checker
  constants become hard to audit.
- **D-14:** Keep `docs/parity/source-breadcrumbs.json` and
  `scripts/check-parity-breadcrumbs.ts` as the required path for any new
  first-party Rust source or test files under `packages/open-bitcoin-*/src` or
  `packages/open-bitcoin-*/tests`.
- **D-15:** The Phase 73 checker should verify that new or referenced UAT,
  fixture, compatibility-harness, support-bundle, live-smoke, and deterministic
  checker surfaces are documented as local evidence and not as production-node,
  inbound-serving, relay, production-wallet, migration-apply, packaging, GUI,
  hosted-dashboard, or public-network CI claims.
- **D-16:** Do not introduce SLSA, in-toto, signed attestations, or generated
  provenance systems in Phase 73. Those are future release-engineering scope
  unless a later milestone explicitly adopts them.

### the agent's Discretion

- The planner may split Phase 73 into coverage audit/gap tests, UAT command
  matrix docs, deterministic checker wiring, and parity/auditability closeout.
- The executor may keep Phase 73 implementation mostly in docs and Bun checker
  code if the coverage map proves existing deterministic tests already satisfy
  VER-02.
- The executor may add a small manifest such as `docs/parity/v1.6-evidence.json`
  or an embedded checker constant table if that makes evidence mapping clearer,
  but should avoid broad generated artifacts or new dependencies.

</decisions>

<canonical_refs>

## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Phase Scope

- `.planning/ROADMAP.md` - Phase 73 goal, dependency on Phase 72, success
  criteria, and deferred Phase 74 boundary.
- `.planning/REQUIREMENTS.md` - VER-01 through VER-04 and v1.6 out-of-scope
  public-network/default-verification boundaries.
- `.planning/PROJECT.md` - v1.6 milestone goal, pinned Knots baseline,
  functional-core boundary, and production-claim limits.
- `.planning/STATE.md` - Current milestone state. Note that the file may lag the
  Phase 72 roadmap/disk state; verify with `roadmap analyze` and phase
  artifacts before relying on the summary.
- `AGENTS.md` - Repo-local GSD workflow, Rust, parity breadcrumb, UAT command,
  generated artifact, and verification requirements.
- `AGENTS.bright-builds.md` - Bright Builds workflow and standards routing.
- `standards-overrides.md` - Current local standards override registry.

### Prior Phase Decisions And Evidence

- `.planning/phases/67-release-boundaries-and-deterministic-verification/67-CONTEXT.md`
  - Prior release-boundary checker posture and default verification exclusions.
- `.planning/phases/68-full-active-chain-validation-and-durable-persistence/68-CONTEXT.md`
  - Validated active-chain progress, durable UTXO/undo persistence, duplicate
  connect prevention, no-credit peer outcomes, and verification posture.
- `.planning/phases/68-full-active-chain-validation-and-durable-persistence/68-VERIFICATION.md`
  - Passed Phase 68 evidence to audit for VER-02 coverage.
- `.planning/phases/69-tip-tracking-and-stay-current-operation/69-CONTEXT.md`
  - Best-known tip evidence, stay-current status, and deterministic coverage
  decisions.
- `.planning/phases/69-tip-tracking-and-stay-current-operation/69-VERIFICATION.md`
  - Passed Phase 69 evidence to audit for best-chain/tip coverage.
- `.planning/phases/70-reorg-peer-rotation-and-no-progress-recovery/70-CONTEXT.md`
  - Reorg, peer response failure, stale in-flight, and no-progress recovery
  decisions.
- `.planning/phases/70-reorg-peer-rotation-and-no-progress-recovery/70-VERIFICATION.md`
  - Passed Phase 70 evidence to audit for reorg and peer failure coverage.
- `.planning/phases/71-resource-bounds-and-durable-restart-resume/71-CONTEXT.md`
  - Resource bounds, same-datadir restart/resume, storage pressure, and
  deterministic synthetic long-chain coverage.
- `.planning/phases/71-resource-bounds-and-durable-restart-resume/71-VERIFICATION.md`
  - Passed Phase 71 evidence to audit for restart/resource coverage.
- `.planning/phases/72-operator-observability-and-support-evidence/72-CONTEXT.md`
  - Shared truth contract, support verdicts, live-smoke summary, docs, and
  checker decisions.
- `.planning/phases/72-operator-observability-and-support-evidence/72-VERIFICATION.md`
  - Passed Phase 72 evidence and required docs/checker/status surfaces.

### Implementation And Verification Surfaces

- `scripts/verify.sh` - Repo-native deterministic verification contract and
  ordered checker wiring.
- `scripts/check-phase68-active-chain-persistence.ts` - Existing active-chain
  persistence checker pattern and Phase 68 coverage anchors.
- `scripts/check-phase69-tip-stay-current.ts` - Existing best-tip and
  stay-current checker pattern.
- `scripts/check-phase70-reorg-recovery.ts` - Existing reorg, peer rotation,
  and no-progress checker pattern.
- `scripts/check-phase71-resource-restart.ts` - Existing resource/restart
  checker and forbidden default-verification guard pattern.
- `scripts/check-phase72-observability-evidence.ts` - Existing cross-surface
  evidence checker pattern to follow for Phase 73.
- `scripts/check-parity-breadcrumbs.ts` - Required parity breadcrumb checker
  for new first-party Rust source/test files.
- `scripts/run-live-mainnet-smoke.ts` - Opt-in public-mainnet live-smoke wrapper
  and report schema.
- `scripts/test-run-live-mainnet-smoke.sh` - Deterministic live-smoke fixture
  check that remains local and may be referenced by Phase 73.
- `packages/open-bitcoin-node/src/sync/tests.rs` - Existing scripted transport,
  durable runtime, restart, peer failure, synthetic long-chain, and resource
  bound fixtures.
- `packages/open-bitcoin-chainstate/src/engine.rs` and
  `packages/open-bitcoin-chainstate/src/types.rs` - Chainstate connect,
  disconnect, reorg, UTXO, undo, and snapshot behavior.
- `packages/open-bitcoin-node/src/storage.rs`,
  `packages/open-bitcoin-node/src/storage/fjall_store.rs`, and
  `packages/open-bitcoin-node/src/storage/snapshot_codec.rs` - Durable storage,
  runtime metadata, recovery, and versioned snapshot surfaces.
- `packages/open-bitcoin-cli/src/operator/support.rs`,
  `packages/open-bitcoin-cli/src/operator/support/evidence.rs`,
  `packages/open-bitcoin-cli/src/operator/support/live_smoke.rs`, and
  `packages/open-bitcoin-cli/src/operator/support/render.rs` - Support bundle
  and summary-only live-smoke evidence surfaces.
- `packages/open-bitcoin-cli/src/operator/status/render.rs` and
  `packages/open-bitcoin-cli/src/operator/dashboard/model.rs` - Status surfaces
  used by opt-in UAT comparison.
- `packages/open-bitcoin-rpc/src/dispatch/tests.rs` and
  `packages/open-bitcoin-rpc/src/method/node.rs` - RPC status and baseline
  exclusion tests.

### Operator Docs And Parity Roots

- `docs/operator/runtime-guide.md` - Authoritative location for the Phase 73
  opt-in UAT matrix and repo-local command forms.
- `docs/architecture/status-snapshot.md` - Shared status contract and field
  interpretation.
- `docs/architecture/operator-observability.md` - Metrics/log/support evidence,
  retention, compact snapshots, and deterministic verification boundaries.
- `docs/architecture/storage-decision.md` - Durable storage and recovery
  posture.
- `docs/parity/catalog/p2p.md` - Public peer, full-sync, and deferred
  production-node boundaries.
- `docs/parity/catalog/chainstate.md` - Active-chain, UTXO/undo, reorg, and
  persistence parity scope.
- `docs/parity/catalog/operator-runtime-release-hardening.md` - Operator
  runtime and evidence boundary catalog.
- `docs/parity/index.json`, `docs/parity/checklist.md`, and
  `docs/parity/README.md` - Parity roots that must remain consistent with
  Phase 73 verification and UAT claims.
- `docs/parity/source-breadcrumbs.json` - Required breadcrumb registry for new
  first-party Rust source/test files.

### Baseline Anchors

- `packages/bitcoin-knots/src/validation.cpp` - Block validation, active-chain
  connection, disconnect, and reorg behavior anchor.
- `packages/bitcoin-knots/src/node/blockstorage.cpp` - Block storage and
  restart/recovery anchor.
- `packages/bitcoin-knots/src/coins.h` and `packages/bitcoin-knots/src/coins.cpp`
  - UTXO view and undo persistence anchors.
- `packages/bitcoin-knots/src/net_processing.cpp` - Peer response, block/header
  progress, invalid data, and no-credit behavior anchor.
- `packages/bitcoin-knots/src/headerssync.cpp` - Header sync and best-chain
  selection anchor.

</canonical_refs>

<code_context>

## Existing Code Insights

### Reusable Assets

- `scripts/verify.sh` already runs Bun checkers through Phase 72 before Rust
  format, clippy, build, tests, benchmarks, Bazel smoke build, and coverage.
- Phase 68 through Phase 72 checkers already verify ordered checker wiring,
  required source/docs/test anchors, and forbidden default public-network or
  service-manager commands.
- `packages/open-bitcoin-node/src/sync/tests.rs` contains deterministic scripted
  transport, durable store, peer failure, restart/resume, synthetic long-chain,
  metrics/logs, and resource-bound tests that should be audited before adding
  new fixtures.
- `scripts/test-run-live-mainnet-smoke.sh` provides deterministic fixture
  validation for live-smoke schema fields without contacting public peers.
- `docs/operator/runtime-guide.md` already contains many repo-local command
  examples, but they are spread across sections; Phase 73 should make the UAT
  matrix authoritative and auditable.
- `docs/parity/source-breadcrumbs.json` and `scripts/check-parity-breadcrumbs.ts`
  are the existing source/test audit mechanism.

### Established Patterns

- Public-mainnet full-sync, manual-peer, restart-after-progress, long-running,
  and real service-manager review are opt-in local UAT, never default
  verification.
- Deterministic checkers are Bun TypeScript scripts with explicit string/path
  assertions and no external dependencies.
- Operator docs must use repo-local Cargo and Bazel command forms instead of an
  installed `open-bitcoin` alias when giving UAT instructions.
- Missing evidence should be rendered or documented as unavailable/diagnostic,
  not silently treated as proof.
- New Rust source or test files require parity breadcrumbs. Docs and scripts do
  not use source breadcrumbs but still need checker coverage when they support a
  parity or evidence claim.

### Integration Points

- Add `scripts/check-phase73-uat-verification.ts` or a similarly named checker
  and wire it after `scripts/check-phase72-observability-evidence.ts`.
- Update `docs/operator/runtime-guide.md` with a central Phase 73 opt-in UAT
  matrix covering full-sync, stay-current, restart/resume, status comparison,
  live-smoke, and support-bundle review.
- Add or update deterministic tests only where the Phase 73 coverage map finds
  missing VER-02 behavior.
- Update parity docs and breadcrumbs only for changed surfaces. If no new Rust
  files are added, preserve existing breadcrumbs and have the Phase 73 checker
  assert that coverage explicitly.

</code_context>

<specifics>

## Specific Ideas

- A coverage table in the Phase 73 checker can map each VER-02 behavior to
  named tests and prior phase checker evidence, then fail on missing anchors.
- The UAT matrix should include both Cargo and Bazel forms for:
  `sync status --format json`, `status --format json`, `support bundle`, service
  status/restart for explicit local service review, and any operator CLI
  workflow used to inspect full-sync evidence.
- The live-smoke command remains Bun-based because it is a repo-owned script:
  `bun run scripts/run-live-mainnet-smoke.ts --datadir=/tmp/open-bitcoin-mainnet`
  with optional `--manual-peer` and `--restart-after-progress` examples labeled
  opt-in.
- Phase 73 should mention `bash scripts/test-run-live-mainnet-smoke.sh` as a
  deterministic fixture check, not as public-network UAT.

</specifics>

<deferred>

## Deferred Ideas

- Public-network CI, release-blocking live sync, current-tip timing thresholds,
  production-node/inbound serving/relay claims, production-funds wallet use,
  migration apply mode, packaging distribution, hosted dashboards, GUI, Windows
  service support, signed attestations, SLSA/in-toto provenance, and generated
  release attestation systems remain future scope.

</deferred>

---

*Phase: 73-opt-in-uat-and-deterministic-verification*
*Context gathered: 2026-06-13*
