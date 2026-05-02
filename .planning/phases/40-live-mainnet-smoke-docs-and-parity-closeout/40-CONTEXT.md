---
phase: 40
phase_name: "Live Mainnet Smoke, Docs, and Parity Closeout"
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: "40-2026-05-02T13-22-45"
generated_at: "2026-05-02T13:22:45.481Z"
---

# Phase 40 Context: Live Mainnet Smoke, Docs, and Parity Closeout

**Gathered:** 2026-05-02
**Status:** Ready for planning

<domain>
## Phase Boundary

Close the v1.2 milestone by proving the daemon-owned mainnet sync path with explicit opt-in live evidence, then refresh operator-facing and parity-facing docs so the shipped claim matches the code. This phase owns live smoke or benchmark entrypoints, actionable prerequisite failures, support-evidence capture, README or runtime-guide truth, and parity-ledger closeout. It does not widen the node into a production-node claim, move public-network checks into default verification, or reopen the underlying sync mechanics from Phases 35 through 39.

</domain>

<decisions>
## Implementation Decisions

### Live evidence contract
- **D-01:** Keep live mainnet validation explicitly opt-in and outside `bash scripts/verify.sh`; the default local verification contract must remain hermetic.
- **D-02:** Reuse repo-owned operator and daemon surfaces for live evidence instead of asking reviewers to inspect internal store files or assemble ad hoc shell recipes.
- **D-03:** Emit reproducible local evidence as generated report artifacts, not checked-in golden files or timing gates.

### Failure and support guidance
- **D-04:** Fail live smoke or benchmark entrypoints early with actionable messages when datadir, config, disk, network, time, or other runtime prerequisites are missing or unsafe.
- **D-05:** Treat status snapshots, pause or resume controls, structured operator output, and report files as the supported evidence path for troubleshooting and audit handoff.

### Documentation and parity truth
- **D-06:** Refresh README, operator runtime docs, and parity-ledger surfaces together so v1.2 shipped claims, known limitations, and deferred Knots or Core behaviors stay consistent and machine-readable.
- **D-07:** Record v1.2 mainnet-sync closeout in parity docs as an auditable support slice, not as a blanket statement that Open Bitcoin is now production-ready or fully Knots-equivalent for all public-network behavior.

### the agent's Discretion
- Whether the live evidence entrypoint lands as a dedicated script, a benchmark-mode extension, or a thin wrapper over an existing binary, as long as it stays explicit, reproducible, and outside default verify.
- The exact report schema and filenames, provided they are local artifacts with enough provenance for reruns and audit review.
- The smallest set of README, runtime-guide, parity-ledger, and milestone-closeout updates needed to keep contributor-facing claims honest.

</decisions>

<specifics>
## Specific Ideas

- The existing ignored live-network smoke test is a useful starting point for the explicit opt-in path, but the user-facing workflow should become a repo-owned evidence command rather than a raw test-only secret.
- Benchmark and report tooling already writes JSON and Markdown artifacts under `packages/target/benchmark-reports`; Phase 40 should extend that evidence mindset instead of inventing a separate checked-in artifact policy.
- The operator runtime guide already has mainnet activation, status, pause/resume, and benchmark sections; it should become the canonical home for prerequisites, startup, status interpretation, stop/resume, troubleshooting, and known limitations.
- README and parity-ledger copy still says live-mainnet smoke evidence is future work; Phase 40 should remove that stale wording only where the shipped code really supports it.

</specifics>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Milestone scope
- `.planning/REQUIREMENTS.md` — `SYNCMAIN-04`, `CHAINMAIN-05`, `RESUME-03`, `OBSMAIN-04`, `VERMAIN-01`, `VERMAIN-03`, `VERMAIN-04`, and `VERMAIN-05`
- `.planning/ROADMAP.md` — Phase 40 goal, success criteria, and closeout boundary
- `.planning/STATE.md` — current milestone state and archive handoff expectations

### Workflow and standards
- `AGENTS.md` — repo-local GSD, verification, parity breadcrumb, doc freshness, and README-update expectations
- `AGENTS.bright-builds.md` — Bright Builds sync-before-edit and repo-native verification requirements
- `standards-overrides.md` — local override status
- Bright Builds `standards/index.md`
- Bright Builds `standards/core/architecture.md`
- Bright Builds `standards/core/code-shape.md`
- Bright Builds `standards/core/testing.md`
- Bright Builds `standards/core/verification.md`
- Bright Builds `standards/languages/rust.md`

### Existing code and doc seams
- `packages/open-bitcoin-node/src/sync/tests.rs` — ignored opt-in live-network smoke coverage and durable runtime behavior
- `packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs` — daemon-owned mainnet sync activation and worker lifecycle
- `packages/open-bitcoin-cli/src/operator.rs` and `packages/open-bitcoin-cli/src/operator/runtime/support.rs` — supported operator sync control and status surfaces
- `packages/open-bitcoin-bench/src/main.rs` and `packages/open-bitcoin-bench/src/registry.rs` — current repo-owned benchmark entrypoints and report policy
- `scripts/run-benchmarks.sh` and `scripts/verify.sh` — benchmark wrapper contract and hermetic default verification boundary
- `README.md` — contributor-facing preview claims that still mark live mainnet closeout as future work
- `docs/operator/runtime-guide.md` — operator-facing runtime, benchmark, limitation, and troubleshooting contract
- `docs/parity/index.json`, `docs/parity/checklist.md`, `docs/parity/README.md`, and `docs/parity/release-readiness.md` — machine-readable and human-readable parity closeout surfaces

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `packages/open-bitcoin-node/src/sync/tests.rs` already contains an ignored live-network smoke test guarded by `OPEN_BITCOIN_LIVE_SYNC_SMOKE=1`, which proves the codebase already has an explicit public-network test boundary.
- `packages/open-bitcoin-bench/src/main.rs` and `scripts/run-benchmarks.sh` already implement repo-owned report generation with JSON and Markdown outputs, provenance fields, and clear local output directories.
- `packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs` and `packages/open-bitcoin-cli/src/operator/runtime/support.rs` already surface the daemon sync lifecycle, pause/resume controls, and durable status truth needed for operator evidence.

### Established Patterns
- `bash scripts/verify.sh` keeps public-network work out of the default verification contract while still validating generated local evidence for deterministic benchmark flows.
- The parity ledger already treats generated runtime evidence as local artifacts and records shipped versus deferred behavior through `docs/parity/index.json` plus companion docs.
- Contributor and operator docs already distinguish shipped behavior from non-claims; Phase 40 should preserve that tone rather than over-promising.

### Integration Points
- README and `docs/operator/runtime-guide.md` still contain explicit “Phase 40 later work” wording that should be replaced only after the live evidence path exists.
- The current benchmark registry covers deterministic sync-runtime and operator-runtime cases but does not yet represent live mainnet progress evidence.
- The current parity ledger reflects v1 and v1.1 release-hardening slices; it needs a v1.2-aware update so live mainnet support is described as shipped-with-limits instead of still deferred.

</code_context>

<deferred>
## Deferred Ideas

- Moving public-network smoke into `bash scripts/verify.sh` or any other default local gate.
- Production-node, production-funds, packaged-install, Windows-service, inbound-peer, address-relay, or broad mempool-relay claims.
- Timing-threshold release gates or checked-in live-network report fixtures.
- Broader daemon supervision or service hardening beyond the explicit v1.2 closeout surface.

</deferred>

---

*Phase: 40-live-mainnet-smoke-docs-and-parity-closeout*
*Context gathered: 2026-05-02*
