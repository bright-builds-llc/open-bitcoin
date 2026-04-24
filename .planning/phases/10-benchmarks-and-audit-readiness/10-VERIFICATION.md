---
phase: 10-benchmarks-and-audit-readiness
verified: 2026-04-24T12:53:22Z
status: passed
score: "25/25 must-haves verified"
generated_by: gsd-verifier
lifecycle_mode: yolo
phase_lifecycle_id: 10-2026-04-24T10-47-33
generated_at: 2026-04-24T12:53:22Z
lifecycle_validated: true
overrides_applied: 0
---

# Phase 10: Benchmarks and Audit Readiness Verification Report

**Phase Goal:** Deliver benchmark/audit readiness artifacts that make parity claims auditable and provide deterministic benchmark smoke coverage without replacing Knots parity as the behavioral source of truth.
**Verified:** 2026-04-24T12:53:22Z
**Status:** passed
**Re-verification:** No - initial verification

## Goal Achievement

Phase 10 achieved the goal. The repository now has a first-party benchmark crate, bounded smoke/full benchmark runner, JSON and Markdown report formats, repo-owned benchmark wrapper, `scripts/verify.sh` and CI wiring, parity checklist artifacts, deviations/unknowns audit documentation, and a release-readiness handoff. The default benchmark comparison remains mapping-only against the pinned Bitcoin Knots `29.3.knots20260210` benchmark/source surfaces; optional Knots JSON/bin paths are metadata enrichment only.

### Observable Truths

| # | Truth | Status | Evidence |
| --- | --- | --- | --- |
| 1 | Benchmarks measure critical node and wallet paths and compare to Knots where meaningful. | VERIFIED | `open-bitcoin-bench` registers and executes all seven groups, records Knots benchmark/source mappings, and smoke report JSON contains 7 groups with `optional_knots_source: null` by default. |
| 2 | The parity checklist reports status for every in-scope surface. | VERIFIED | `docs/parity/index.json` contains 11 required checklist surfaces and exact taxonomy `planned`, `in_progress`, `done`, `deferred`, `out_of_scope`; `docs/parity/checklist.md` mirrors it. |
| 3 | Audit artifacts make deviations, unknowns, and milestone readiness easy to review. | VERIFIED | `docs/parity/deviations-and-unknowns.md` and `docs/parity/release-readiness.md` list deferrals, suspected unknowns, verification commands, benchmark outputs, and reviewer inspection points. |
| 4 | A contributor can build a first-party benchmark crate without adding a production runtime dependency per D-02. | VERIFIED | `packages/open-bitcoin-bench` is a first-party workspace member; dependencies are first-party crates plus `serde`/`serde_json`; root `//:bench` Bazel alias builds it. |
| 5 | The benchmark registry contains every D-01 benchmark group and required Knots mapping metadata per D-03. | VERIFIED | `registry.rs` has all seven group ids and mapping markers: `VerifyScriptBench`, `DeserializeBlockTest`, `ComplexMemPool`, `RpcMempool`, `WalletBalance`, `CoinSelection`, `WalletCreateTx`, `AddrMan`, `EvictionProtection`. |
| 6 | The runner exposes bounded smoke/full execution modes without wall-clock pass/fail thresholds per D-04. | VERIFIED | `RunConfig::smoke` caps 1..=10 iterations, `RunConfig::full` caps 1..=10000, and reports include timings without threshold status. |
| 7 | The report schema is JSON-first and includes Markdown summary support per D-05. | VERIFIED | `BenchReport` serializes `schema_version`, `baseline`, `mode`, `generated_at_unix_seconds`, `groups`, and `optional_knots_source`; Markdown writer escapes table cells. |
| 8 | Benchmark smoke execution covers all critical first-party paths named in D-01. | VERIFIED | Cases cover consensus script, codec, chainstate, mempool policy, network wire/sync, wallet, and RPC/CLI through public first-party APIs. |
| 9 | Stateful benchmark cases reset or clone prepared fixtures so mutation does not leak across iterations. | VERIFIED | Chainstate/mempool/wallet cases construct or clone snapshots per run; unit test `stateful_core_cases_are_repeatable` passes. |
| 10 | Optional Knots JSON/bin inputs are recorded as report metadata only; default execution remains mapping-only per D-03. | VERIFIED | Main CLI builds `KnotsSource` only from explicit paths; default smoke report has `optional_knots_source: null`; unit test accepts missing optional paths without reading them. |
| 11 | A smoke run emits JSON and Markdown reports without timing thresholds per D-04 and D-05. | VERIFIED | `bash scripts/run-benchmarks.sh --smoke` wrote `open-bitcoin-bench-smoke.json` and `.md`; schema check found no `threshold_status`. |
| 12 | A contributor can run benchmarks through a repo-owned `scripts/run-benchmarks.sh` entrypoint. | VERIFIED | Wrapper supports `--list`, `--smoke`, `--full`, `--iterations`, `--output-dir`, `--knots-json`, `--knots-bin`, and `--format`. |
| 13 | `scripts/verify.sh` invokes only bounded smoke mode after Plan 10-02 proves smoke execution. | VERIFIED | `scripts/verify.sh` exports `OPEN_BITCOIN_BENCHMARK_REPORT_DIR` and invokes `bash scripts/run-benchmarks.sh --smoke --output-dir "$OPEN_BITCOIN_BENCHMARK_REPORT_DIR"`. |
| 14 | CI preserves generated benchmark reports as artifacts without making timing thresholds release gates. | VERIFIED | `.github/workflows/ci.yml` uploads `benchmark-reports` from `packages/target/benchmark-reports` with `if-no-files-found: ignore`; no timing threshold gate exists. |
| 15 | Reviewers can inspect checked-in benchmark scope, report locations, and Knots mapping policy. | VERIFIED | `docs/parity/benchmarks.md` documents groups, mapping-only default, optional Knots enrichment, report paths, and non-goals. |
| 16 | A contributor can inspect one parity checklist covering every in-scope initial-milestone surface per D-06. | VERIFIED | `docs/parity/index.json` checklist includes reference, architecture, core serialization, consensus, chainstate, mempool, P2P, wallet, RPC/CLI/config, verification/fuzzing, and benchmark/audit readiness. |
| 17 | Checklist statuses use exactly the locked taxonomy per D-07. | VERIFIED | Node validation confirmed exact taxonomy array and every surface status is allowed. |
| 18 | Every `done` item has concise evidence links, and every `deferred` or `out_of_scope` item has rationale per D-07. | VERIFIED | JSON validation confirmed all `done` records have evidence; no deferred/out_of_scope record lacks rationale. |
| 19 | `docs/parity/index.json` remains the machine-readable root per D-08. | VERIFIED | Checklist and audit roots live in `docs/parity/index.json`; Markdown docs point back to it as the source. |
| 20 | Known gaps, suspected unknowns, and folded todo risks are surfaced without broad follow-on implementation per D-09. | VERIFIED | `deviations-and-unknowns.md` records deferred surfaces, suspected unknowns, AI-agent CLI risk, and panic/illegal-state risk as audit notes only. |
| 21 | A reviewer can answer what is complete, deferred, unknown, and worth inspecting before a release decision per D-10. | VERIFIED | `release-readiness.md` contains required sections including complete surfaces, deferrals, known gaps/unknowns, and reviewer inspection checklist. |
| 22 | Readiness evidence links to phase verification, summaries, parity docs, `scripts/verify.sh`, CI report collection, and benchmark/checklist outputs per D-11. | VERIFIED | `release-readiness.md` links Phase 8/9 verification, Phase 10 summaries, `scripts/verify.sh`, CI artifact uploads, checklist docs, and generated smoke report paths. |
| 23 | Audit artifacts remain deterministic, repo-local, and reviewable in git diffs per D-12. | VERIFIED | Audit artifacts are checked-in JSON/Markdown/docs and scripts; generated timing output remains under gitignored `packages/target/benchmark-reports`. |
| 24 | Stale roadmap or state bookkeeping is recorded as an audit note instead of silently rewritten by hand per D-13. | VERIFIED | `release-readiness.md` `Bookkeeping Notes` records stale `.planning/STATE.md` and `.planning/ROADMAP.md` discrepancies with file references. |
| 25 | The final parity checklist points at benchmark and readiness outputs. | VERIFIED | `benchmarks-audit-readiness` checklist row is `done` with evidence for `docs/parity/benchmarks.md`, `docs/parity/release-readiness.md`, `scripts/run-benchmarks.sh`, `scripts/verify.sh`, and smoke JSON path. |

**Score:** 25/25 truths verified

### Required Artifacts

| Artifact | Expected | Status | Details |
| --- | --- | --- | --- |
| `packages/open-bitcoin-bench/src/registry.rs` | Static benchmark group and Knots mapping registry | VERIFIED | Exists, substantive, wired to cases, includes all required mappings. |
| `packages/open-bitcoin-bench/src/runner.rs` | Bounded timing loop using `Instant` and `black_box` | VERIFIED | Smoke/full iteration caps, no threshold gate, unit tests pass. |
| `packages/open-bitcoin-bench/src/report.rs` | JSON and Markdown benchmark report schema | VERIFIED | Typed serde schema and Markdown escaping implemented. |
| `BUILD.bazel` | Root Bazel alias for benchmark binary | VERIFIED | `name = "bench"` targets `//packages/open-bitcoin-bench:open_bitcoin_bench`. |
| `packages/open-bitcoin-bench/src/fixtures.rs` | Deterministic benchmark fixtures | VERIFIED | Shared fixtures compose public first-party APIs and checked-in codec bytes. |
| `packages/open-bitcoin-bench/src/cases.rs` | Measured case registration across groups | VERIFIED | Registers all seven case modules; repeatability tests pass. |
| `packages/open-bitcoin-bench/src/main.rs` | CLI execution for list/smoke/full and optional Knots inputs | VERIFIED | Writes JSON/Markdown reports, records optional Knots metadata, rejects conflicting modes. |
| `scripts/run-benchmarks.sh` | Repo-owned benchmark wrapper | VERIFIED | Uses Bash arrays, rejects unsupported options, no `eval`/`bash -c`. |
| `scripts/verify.sh` | Repo-native smoke benchmark invocation | VERIFIED | Runs bounded smoke benchmark and builds `//:bench`. |
| `.github/workflows/ci.yml` | Benchmark report artifact upload | VERIFIED | Sets report dir env var and uploads `benchmark-reports`. |
| `docs/parity/benchmarks.md` | Reviewer-facing benchmark scope and mapping guide | VERIFIED | Documents groups, reports, mapping-only default, and non-goals. |
| `docs/parity/index.json` | Machine-readable checklist/readiness root | VERIFIED | Contains checklist and audit roots with final Phase 10 status. |
| `docs/parity/checklist.md` | Human-readable checklist | VERIFIED | Contains every required surface exactly once and marks Phase 10 done. |
| `docs/parity/deviations-and-unknowns.md` | Known gaps, unknowns, deferrals, folded todos | VERIFIED | Captures current audit risks without broad implementation scope. |
| `docs/parity/release-readiness.md` | Milestone handoff and release-readiness artifact | VERIFIED | Contains required headings, commands, evidence links, and bookkeeping notes. |

Artifact verification via `gsd-tools verify artifacts` passed for all 17 plan-declared artifacts. Key-link verification passed for all 10 plan-declared links.

### Key Link Verification

| From | To | Via | Status | Details |
| --- | --- | --- | --- | --- |
| `registry.rs` | `packages/bitcoin-knots/src/bench` | Static benchmark/source mappings | WIRED | Knots mapping paths and benchmark names present. |
| `BUILD.bazel` | `packages/open-bitcoin-bench/BUILD.bazel` | Root benchmark alias | WIRED | `//:bench` aliases the benchmark binary. |
| `cases.rs` | `registry.rs` | Case group ids and Knots mappings | WIRED | Registry groups reference executable case arrays. |
| `main.rs` | `report.rs` | JSON and Markdown report writing | WIRED | Main calls report writers for both output files. |
| `scripts/verify.sh` | `scripts/run-benchmarks.sh` | Bounded smoke invocation | WIRED | Verify script calls wrapper with `--smoke`. |
| `docs/parity/benchmarks.md` | `packages/target/benchmark-reports` | Generated report path | WIRED | Docs list smoke/full JSON and Markdown output names. |
| `index.json` | `checklist.md` | `checklist.path` | WIRED | Index points to `checklist.md`. |
| `checklist.md` | `.planning/phases` | Evidence links | WIRED | Done rows link phase evidence. |
| `release-readiness.md` | `scripts/verify.sh` | Verification evidence command | WIRED | Readiness doc cites command and script. |
| `release-readiness.md` | `packages/target/benchmark-reports` | Benchmark evidence | WIRED | Readiness doc cites smoke JSON and Markdown paths. |

### Data-Flow Trace (Level 4)

| Artifact | Data Variable | Source | Produces Real Data | Status |
| --- | --- | --- | --- | --- |
| `open-bitcoin-bench` reports | `BenchReport.groups` | `main.rs` -> `run_benchmarks(benchmark_groups(), ...)` -> executable `BenchCase::run_once` functions | Yes | FLOWING |
| Benchmark cases | `BenchCase::run_once` | Deterministic fixtures plus consensus, codec, chainstate, mempool, network, wallet, RPC, and CLI APIs | Yes | FLOWING |
| Smoke report files | `open-bitcoin-bench-smoke.{json,md}` | `scripts/run-benchmarks.sh --smoke` and `scripts/verify.sh` | Yes | FLOWING |
| Parity checklist | `checklist.surfaces` | `docs/parity/index.json` static source of truth rendered in `docs/parity/checklist.md` | Yes | FLOWING |
| Release readiness | Evidence links and reviewer checklist | `docs/parity/release-readiness.md` links phase verification, summaries, scripts, CI, and generated report paths | Yes | FLOWING |

### Behavioral Spot-Checks

| Behavior | Command | Result | Status |
| --- | --- | --- | --- |
| Benchmark crate tests pass | `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-bench --all-features` | 18 tests passed | PASS |
| Benchmark wrapper lists required groups | `bash scripts/run-benchmarks.sh --list` | Printed all seven required group ids | PASS |
| Smoke run emits JSON and Markdown report | `bash scripts/run-benchmarks.sh --smoke --output-dir packages/target/phase10-verify.*` | Wrote both files; JSON schema v1, mode `smoke:1`, 7 groups, `optional_knots_source: null` | PASS |
| Checklist taxonomy and Phase 10 evidence validate | Node JSON validation of `docs/parity/index.json` | 11 surfaces, release readiness `done`, Phase 10 `done` | PASS |
| Registry, wrapper, and verify wiring validate | Node marker validation | Required mappings present; wrapper has no `eval`/`bash -c`; `verify.sh` calls smoke run | PASS |
| Repo-native verification passes | `bash scripts/verify.sh` | Passed in 19.439s, including format, clippy, build, tests, benchmark smoke, Bazel smoke, coverage, and architecture checks | PASS |

### Requirements Coverage

| Requirement | Source Plan | Description | Status | Evidence |
| --- | --- | --- | --- | --- |
| `PAR-02` | 10-01, 10-02, 10-03, 10-05 | Benchmarks measure critical node and wallet performance paths and compare against the pinned baseline where meaningful. | SATISFIED | First-party benchmark crate covers seven critical groups, smoke reports execute all groups, registry records Knots mappings, wrapper/verify/CI/docs preserve report evidence without timing gates. |
| `AUD-01` | 10-04, 10-05 | Contributors can inspect a parity checklist reporting each in-scope surface as planned, in progress, done, deferred, or out of scope. | SATISFIED | `docs/parity/index.json` checklist root, `docs/parity/checklist.md`, `docs/parity/deviations-and-unknowns.md`, and `docs/parity/release-readiness.md` provide status, evidence, deferrals, unknowns, and release handoff. |

No Phase 10 requirements are orphaned. `.planning/REQUIREMENTS.md` maps only `PAR-02` and `AUD-01` to Phase 10, and Phase 10 plans claim both.

### Anti-Patterns Found

| File | Line | Pattern | Severity | Impact |
| --- | --- | --- | --- | --- |
| `docs/parity/*` | Multiple | `todo` text | Info | Legitimate folded-todo audit prose and links, not implementation placeholders. |
| `packages/open-bitcoin-bench/src/main.rs` | 80 | `OutputFormat::Summary => {}` | Info | Intentional no-op for summary-only stdout mode after report files are written. |
| `docs/parity/release-readiness.md` | 114 | `console.log` in documented Node command | Info | Documentation command for reviewers, not an implementation handler. |

No blocker anti-patterns were found. No TODO/FIXME/placeholder/coming-soon/not-implemented stubs block the Phase 10 goal.

### Human Verification Required

None. The phase deliverables are code, scripts, JSON, Markdown, and CI wiring that were verified programmatically. Release approval remains a human product decision, but no human-only test blocks Phase 10 goal verification.

### Residual Risks

- Benchmark comparison is mapping-only by default. Numeric Knots benchmark execution is intentionally optional metadata enrichment, consistent with the phase goal and D-03.
- Benchmark reports are trend and audit evidence, not performance regression gates. This is intentional per D-04.
- `docs/parity/release-readiness.md` records stale `.planning/STATE.md` and `.planning/ROADMAP.md` bookkeeping discrepancies; those are planning-state hygiene issues, not Phase 10 benchmark/audit artifact gaps.
- Future GUI/dashboard observability belongs to v2 scope (`OBS-01`) and is not required for this phase.

### Gaps Summary

No gaps found. Phase 10 satisfies `PAR-02` and `AUD-01` based on actual benchmark code, repo verification wiring, generated smoke reports, parity checklist data, and release-readiness documentation.

---

_Verified: 2026-04-24T12:53:22Z_
_Verifier: the agent (gsd-verifier)_
