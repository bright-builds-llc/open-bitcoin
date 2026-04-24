---
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: 10-2026-04-24T10-47-33
generated_at: 2026-04-24T10:47:33.305Z
---

# Phase 10: Benchmarks and Audit Readiness - Context

**Gathered:** 2026-04-24
**Status:** Ready for planning
**Mode:** Yolo

<domain>
## Phase Boundary

Phase 10 adds benchmark coverage and audit-ready parity artifacts for the
headless v1 milestone. It should make performance posture, parity status,
intentional deviations, suspected unknowns, and release readiness visible from
repo-owned files without adding GUI, dashboard, or production runtime scope.

</domain>

<decisions>
## Implementation Decisions

### Benchmark Coverage and Comparison Policy

- **D-01:** Benchmark the critical first-party paths that affect node and wallet
  confidence: consensus/script validation, block or transaction
  parsing/serialization, chainstate connect/disconnect or reorg helpers,
  mempool/policy hot paths where existing pure APIs support it, network message
  encode/decode or sync planning helpers, wallet balance/coin-selection/signing
  helpers, and RPC/CLI dispatch paths that are cheap to exercise.
- **D-02:** Keep benchmark targets in the first-party Rust workspace and out of
  production runtime dependencies. A dev-only benchmark dependency is acceptable
  only if the researcher/planner can justify maintenance status and minimal
  blast radius; otherwise prefer a small repo-owned timing/report harness using
  stable Rust APIs.
- **D-03:** Compare against the pinned Knots baseline where meaningful by
  recording a mapping from each Open Bitcoin benchmark group to the relevant
  Knots benchmark/source surface. Do not make local verification depend on
  building or running the full Knots benchmark binary unless the plan proves it
  is deterministic and practical in the current repo.
- **D-04:** Benchmark results should be useful for trend and audit review, not
  flaky release gates. Verification may assert that benchmarks build, execute
  smoke-sized samples, and emit structured reports; it should not enforce
  machine-dependent wall-clock thresholds by default.
- **D-05:** Reports should be machine-readable first and human-readable second:
  emit stable JSON for tooling and Markdown summaries for reviewers. Generated
  timing outputs should live in a gitignored or target output directory unless a
  checked-in fixture/sample is explicitly stable.

### Parity Checklist and Status Taxonomy

- **D-06:** Produce an audit checklist that covers every in-scope surface from
  the existing parity catalog and v1 requirements: reference baseline,
  architecture/workspace, core serialization, consensus, chainstate, mempool,
  networking, wallet, RPC/CLI/config, verification harnesses/fuzzing, and
  benchmarks/audit readiness.
- **D-07:** The checklist status taxonomy is exactly: `planned`,
  `in_progress`, `done`, `deferred`, `out_of_scope`. Include concise evidence
  links for every `done` item and explicit rationale for every `deferred` or
  `out_of_scope` item.
- **D-08:** Keep `docs/parity/index.json` as the machine-readable root. Add or
  update Markdown companion docs under `docs/parity/` only where they make the
  JSON easier to review. Avoid duplicating large phase reports verbatim.
- **D-09:** Surface existing suspected unknowns and known gaps instead of hiding
  them. Phase 10 can classify and document risk, but broad implementation
  closure for newly discovered gaps belongs in follow-on phases unless a gap is
  small, local, and required for this phase's artifacts to be truthful.

### Audit Readiness and Milestone Handoff

- **D-10:** Add a release-readiness or milestone-handoff artifact that answers
  at a glance: what is complete, what is intentionally deferred, what remains
  unknown, which verification commands prove the current state, and what a
  reviewer should inspect before a release decision.
- **D-11:** Tie audit readiness to evidence that already exists: phase
  verification files, summaries, parity catalog entries, `scripts/verify.sh`,
  CI parity-report collection, and the new benchmark/checklist outputs.
- **D-12:** Keep audit artifacts deterministic and reviewable in normal git
  diffs. Do not introduce a public dashboard, GUI, or service dependency in
  this phase.
- **D-13:** If roadmap/state bookkeeping is stale relative to verified phase
  artifacts, record the discrepancy in audit output and use deterministic GSD
  tooling where possible. Do not rewrite unrelated planning history by hand.

### Folded Todos

- **AI-agent-friendly CLI surface:** Fold into audit readiness as an evidence
  item, not as new Phase 10 CLI functionality. The checklist should verify that
  Phase 8's CLI/RPC artifacts expose non-interactive behavior, structured
  outputs, actionable errors, and scriptable flows where implemented.
- **Sweep panics and illegal states:** Fold into audit readiness as a risk and
  follow-up capture item, not as a broad refactor. Phase 10 may add a lightweight
  audit report/checklist entry for panic/illegal-state exposure, but widespread
  code changes should become a separate phase if needed.

### the agent's Discretion

- Choose the exact benchmark framework/harness shape after researching the
  current workspace, provided it preserves D-02 through D-05.
- Choose filenames for the benchmark and audit reports, provided the root paths
  are discoverable from `docs/parity/README.md`, `docs/parity/index.json`, or
  an equivalent repo-owned index.
- Decide whether benchmark smoke execution belongs directly in
  `scripts/verify.sh` or behind a dedicated script that `verify.sh` invokes,
  based on runtime cost and flake risk.

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Project Scope and Requirements

- `.planning/PROJECT.md` - Core value, headless scope, parity and auditability
  constraints.
- `.planning/REQUIREMENTS.md` - `PAR-02` and `AUD-01` requirements plus v1/v2
  boundary.
- `.planning/ROADMAP.md` - Phase 10 scope and plan seeds.
- `AGENTS.md` - Repo-local verification, Knots baseline, GSD workflow, and Rust
  guidance.
- `AGENTS.bright-builds.md` - Bright Builds workflow and code quality
  guidance.
- `standards-overrides.md` - Local standards exceptions, currently none
  substantive.

### Existing Parity and Audit Surfaces

- `docs/parity/README.md` - Parity artifact contract and deviation rules.
- `docs/parity/index.json` - Machine-readable parity catalog root.
- `docs/parity/catalog/README.md` - Catalog entry expectations.
- `docs/parity/catalog/core-domain-and-serialization.md` - Core serialization
  catalog entry and suspected unknowns.
- `docs/parity/catalog/consensus-validation.md` - Consensus catalog entry.
- `docs/parity/catalog/chainstate.md` - Chainstate catalog entry.
- `docs/parity/catalog/mempool-policy.md` - Mempool policy catalog entry.
- `docs/parity/catalog/p2p.md` - P2P catalog entry.
- `docs/parity/catalog/wallet.md` - Wallet catalog entry.
- `docs/parity/catalog/rpc-cli-config.md` - RPC/CLI/config catalog entry and
  deferred surfaces.
- `docs/parity/catalog/verification-harnesses.md` - Phase 9 harness and
  property coverage catalog entry.

### Verification and Phase Evidence

- `scripts/verify.sh` - Repo-native verification contract and parity report
  defaulting.
- `.github/workflows/ci.yml` - CI verification and parity report artifact
  upload.
- `.planning/phases/08-rpc-cli-and-config-parity/08-VERIFICATION.md` - Phase 8
  verified RPC/CLI/config evidence.
- `.planning/phases/09-parity-harnesses-and-fuzzing/09-CONTEXT.md` - Phase 9
  locked decisions for harness and fuzzing.
- `.planning/phases/09-parity-harnesses-and-fuzzing/09-RESEARCH.md` - Phase 9
  research on parity harnesses and property coverage.
- `.planning/phases/09-parity-harnesses-and-fuzzing/09-01-SUMMARY.md` - RPC
  black-box harness implementation evidence.
- `.planning/phases/09-parity-harnesses-and-fuzzing/09-02-SUMMARY.md` -
  Isolation and report helper evidence.
- `.planning/phases/09-parity-harnesses-and-fuzzing/09-03-SUMMARY.md` -
  Property-style coverage evidence.
- `.planning/phases/09-parity-harnesses-and-fuzzing/09-04-SUMMARY.md` - CI
  parity report evidence.
- `.planning/phases/09-parity-harnesses-and-fuzzing/09-VERIFICATION.md` - Phase
  9 verification status and residual risk.

### Benchmark Baseline References

- `packages/bitcoin-knots/doc/benchmarking.md` - Upstream benchmark workflow.
- `packages/bitcoin-knots/src/bench/bench.h` - Upstream benchmark API shape.
- `packages/bitcoin-knots/src/bench/bench_bitcoin.cpp` - Upstream benchmark
  binary entrypoint.
- `packages/bitcoin-knots/src/bench/verify_script.cpp` - Script validation
  benchmark reference.
- `packages/bitcoin-knots/src/bench/checkblock.cpp` - Block validation
  benchmark reference.
- `packages/bitcoin-knots/src/bench/readwriteblock.cpp` - Block serialization
  benchmark reference.
- `packages/bitcoin-knots/src/bench/mempool_stress.cpp` - Mempool stress
  benchmark reference.
- `packages/bitcoin-knots/src/bench/rpc_mempool.cpp` - RPC/mempool benchmark
  reference.
- `packages/bitcoin-knots/src/bench/wallet_balance.cpp` - Wallet balance
  benchmark reference.
- `packages/bitcoin-knots/src/bench/coin_selection.cpp` - Coin selection
  benchmark reference.
- `packages/bitcoin-knots/src/bench/wallet_create_tx.cpp` - Wallet transaction
  creation benchmark reference.
- `packages/bitcoin-knots/src/bench/addrman.cpp` - Networking/address-manager
  benchmark reference.
- `packages/bitcoin-knots/src/bench/peer_eviction.cpp` - Peer policy benchmark
  reference.

### Folded Todo Inputs

- `.planning/todos/pending/2026-04-18-ai-agent-friendly-cli-surface.md` -
  Audit evidence item for machine-friendly operator surfaces.
- `.planning/todos/pending/2026-04-18-sweep-panics-and-illegal-states.md` -
  Audit risk capture item for panic/illegal-state exposure.

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets

- `open-bitcoin-test-harness` already has report helpers and target-neutral
  case APIs that can inform benchmark or audit report shapes.
- `scripts/verify.sh` is the repo-native aggregate verification entrypoint and
  already wires parity report output into `packages/target/parity-reports`.
- `docs/parity/index.json` and `docs/parity/catalog/*.md` provide the existing
  parity status source of truth.
- Phase 9 summaries and verification provide reusable evidence for harness,
  property, CI, and parity report claims.

### Established Patterns

- Planning and verification artifacts should link to concise evidence rather
  than copy whole phase reports.
- Optional heavyweight baseline comparisons are env-gated in current parity
  harness work; absent Knots runtime configuration records skipped behavior
  rather than failing local verification.
- First-party production crates avoid third-party Rust Bitcoin libraries; test,
  benchmark, and dev-only dependencies still need minimal blast radius.

### Integration Points

- `packages/Cargo.toml` and root `BUILD.bazel` are the workspace integration
  points for first-party benchmark or report crates/binaries.
- `scripts/verify.sh` and `.github/workflows/ci.yml` are the verification and CI
  integration points.
- `docs/parity/README.md`, `docs/parity/index.json`, and catalog docs are the
  reviewer-facing discovery points for parity and audit artifacts.

</code_context>

<specifics>
## Specific Ideas

- Prefer benchmark smoke checks that prove the harness executes and writes
  reports over hard timing thresholds.
- Treat Knots benchmark parity as a mapping/reporting obligation first, with
  actual Knots binary execution optional unless the plan can make it reliable.
- Make audit reports useful for humans and agents by using stable status
  fields, explicit evidence paths, and clear follow-up lists.

</specifics>

<deferred>
## Deferred Ideas

- Public benchmark dashboards or richer published observability belong to v2
  `OBS-01`, not this phase.
- Broad CLI feature additions beyond documenting/evidencing Phase 8 behavior
  are deferred unless a small documentation or audit fix is required.
- Broad panic/illegal-state refactors are deferred to a separate code-quality
  phase unless a small local fix is required for truthful Phase 10 artifacts.

### Reviewed Todos (not folded)

- `.planning/todos/pending/2026-04-18-reduce-nesting-with-early-returns.md` -
  reviewed by the recommendation engine but not folded; Phase 10 is an audit
  and benchmark phase, not a broad control-flow refactor.

</deferred>

---

*Phase: 10-benchmarks-and-audit-readiness*
*Context gathered: 2026-04-24*
