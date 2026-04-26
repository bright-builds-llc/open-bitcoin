---
phase: 10
slug: benchmarks-and-audit-readiness
status: verified
threats_open: 0
asvs_level: 1
created: 2026-04-26
updated: 2026-04-26
---

# Phase 10 - Security

Per-phase security contract: threat register, accepted risks, and audit trail.

## Scope

Phase 10 delivers benchmark and audit-readiness infrastructure: a first-party
`open-bitcoin-bench` crate, deterministic benchmark fixtures and executable
cases for the seven parity surfaces, a bounded benchmark runner, JSON and
Markdown benchmark reports, a contributor-facing benchmark wrapper, CI report
artifact upload, parity checklist metadata, deviations and unknowns docs, and a
release-readiness handoff.

The Phase 10 plans declared 20 threats across benchmark report integrity,
iteration bounds, static Knots mappings, stateful fixture isolation, shell
argument forwarding, CI artifact hygiene, checklist provenance, and readiness
claim accuracy. Phase 10 summaries reported no additional open threat flags.

## Trust Boundaries

| Boundary | Description | Data Crossing |
|----------|-------------|---------------|
| CLI args to benchmark binary | User-provided benchmark mode, iterations, output path, format, and optional Knots path flags enter the benchmark executable. | Raw args, parsed run config, output directory path, optional JSON/bin paths, and report format. |
| Static registry to benchmark reports | Static benchmark group ids and Knots mapping metadata become machine-readable JSON and Markdown evidence. | Group ids, case ids, benchmark names, Knots source paths, and mapping notes. |
| Benchmark fixtures to first-party APIs | Deterministic fixtures call consensus, codec, chainstate, mempool, network, wallet, RPC, and CLI APIs. | Blocks, transactions, scripts, mempool contexts, network messages, wallet snapshots, and RPC calls. |
| Report writer to filesystem | Benchmark helpers create JSON and Markdown files in the selected report directory. | Report schema, elapsed times, source mappings, optional Knots metadata, and generated artifact paths. |
| Shell args to benchmark wrapper | User-provided wrapper args enter `scripts/run-benchmarks.sh` before Cargo invocation. | Mode flags, iterations, output path, optional Knots paths, output format, and list requests. |
| Benchmark reports to CI artifacts | Generated benchmark files are collected by CI after repo verification. | `packages/target/benchmark-reports` JSON and Markdown files. |
| Parity JSON to reviewers/tools | Machine-readable checklist and audit metadata drive human docs and future tooling. | Status taxonomy, surface ids, evidence links, known gaps, suspected unknowns, and release-readiness status. |
| Documentation to release reviewers | Benchmark docs, checklist docs, deviations, and release-readiness notes frame audit and release claims. | Human-readable coverage claims, deferrals, commands, artifact paths, and stale bookkeeping notes. |

## Threat Register

| Threat ID | Category | Component | Disposition | Mitigation | Status |
|-----------|----------|-----------|-------------|------------|--------|
| T-10-01-01 | T | `packages/open-bitcoin-bench/src/report.rs` | mitigate | Reports serialize through `serde_json::to_string_pretty`, and Markdown table output escapes pipe and newline characters before writing report cells. | closed |
| T-10-01-02 | D | `packages/open-bitcoin-bench/src/runner.rs` | mitigate | `RunConfig` validates smoke runs to 1..=10 iterations and full runs to 1..=10000 iterations before any case loop executes. | closed |
| T-10-01-03 | S | `packages/open-bitcoin-bench/src/registry.rs` | mitigate | Benchmark ids, groups, and Knots mappings are static constants; unit tests verify exact required group ids and required Knots mapping markers. | closed |
| T-10-01-04 | I | `packages/open-bitcoin-bench/src/report.rs` | mitigate | Reports carry only explicit optional Knots metadata fields and benchmark data; targeted review found no environment dump or credential serialization path. | closed |
| T-10-02-01 | D | `packages/open-bitcoin-bench/src/main.rs` | mitigate | CLI parsing requires exactly one mode, rejects invalid iteration values, defaults smoke to one iteration, and delegates limits to `RunConfig`. | closed |
| T-10-02-02 | T | `packages/open-bitcoin-bench/src/cases/*.rs` | mitigate | Stateful cases clone snapshots or create fresh `Mempool`/`Wallet`/`Chainstate` state on each run, with a repeatability test for stateful core cases. | closed |
| T-10-02-03 | I | `packages/open-bitcoin-bench/src/main.rs` | mitigate | `knots_source` records only display strings for explicit optional Knots JSON/bin paths and does not open, read, execute, or dump those paths. | closed |
| T-10-02-04 | R | `packages/open-bitcoin-bench/src/report.rs` | mitigate | Smoke reports include schema version, mode, groups, and optional Knots metadata, and contain no timing threshold status field. | closed |
| T-10-03-01 | E | `scripts/run-benchmarks.sh` | mitigate | The wrapper uses Bash arrays for Cargo and benchmark args, rejects unsupported options, requires values for path/format options, and does not use shell-evaluated command construction. | closed |
| T-10-03-02 | D | `scripts/verify.sh` | mitigate | Repo verification invokes `bash scripts/run-benchmarks.sh --smoke --output-dir "$OPEN_BITCOIN_BENCHMARK_REPORT_DIR"`, relying on the bounded smoke path. | closed |
| T-10-03-03 | I | `.github/workflows/ci.yml` | mitigate | CI uploads only `packages/target/parity-reports` and `packages/target/benchmark-reports`; no environment dump or credential artifact path is configured. | closed |
| T-10-03-04 | S | `docs/parity/benchmarks.md` | mitigate | Benchmark docs explicitly state reports are audit/trend evidence, not release timing gates, and that default Knots comparison is mapping-only. | closed |
| T-10-04-01 | S | `docs/parity/index.json` | mitigate | The checklist root locks the exact status taxonomy, requires evidence for done surfaces, and marks `benchmarks-audit-readiness` done with concrete evidence links. | closed |
| T-10-04-02 | T | `docs/parity/checklist.md` | mitigate | Checklist Markdown is a human-readable projection of `index.json`, and validation confirms the 11 required surface ids exactly once in order. | closed |
| T-10-04-03 | I | `docs/parity/deviations-and-unknowns.md` | mitigate | Deviations and unknowns summarize deferred scope and residual risks without copying secrets, RPC auth values, or raw environment values. | closed |
| T-10-04-04 | R | `docs/parity/deviations-and-unknowns.md` | mitigate | Deferred surfaces, suspected unknowns, folded todo status, and follow-up triggers preserve rationale for future audit instead of silently omitting gaps. | closed |
| T-10-05-01 | S | `docs/parity/release-readiness.md` | mitigate | Release-readiness claims point to explicit verification commands, benchmark report paths, checklist docs, phase artifacts, and CI artifact handling. | closed |
| T-10-05-02 | R | `docs/parity/release-readiness.md` | mitigate | `## Bookkeeping Notes` records stale planning state and roadmap discrepancies with concrete file references instead of rewriting unrelated history. | closed |
| T-10-05-03 | I | `docs/parity/release-readiness.md` | mitigate | Readiness docs record command names, artifact paths, and skip/follow-up rationale only; targeted review found no credentials, RPC auth values, or raw environment dumps. | closed |
| T-10-05-04 | T | `docs/parity/index.json` | mitigate | Node validation confirms release readiness is `done`, the checklist status taxonomy is locked, and every done checklist surface carries evidence. | closed |

Status: open, closed.
Disposition: mitigate (implementation required), accept (documented risk), transfer (third-party).

## Controls Verified

| Control | Evidence | Status |
|---------|----------|--------|
| First-party benchmark package ownership | `packages/open-bitcoin-bench` is a first-party crate with Cargo and Bazel targets, and root verification builds `//:bench`. | verified |
| Static benchmark coverage for seven parity surfaces | `registry.rs` defines the seven required group ids and static Knots mappings; `cases.rs` aggregates executable cases for consensus script, codec, chainstate, mempool, network, wallet, and RPC/CLI. | verified |
| Iteration denial-of-service bounds | `runner.rs` enforces smoke and full iteration bounds, and tests reject zero, smoke values above 10, and full values above 10000. | verified |
| No benchmark timing release gate | `BenchReport` includes elapsed nanos for audit/trend inspection but no threshold status; docs state reports are not release timing gates. | verified |
| Report serialization and Markdown hygiene | JSON uses typed `serde_json` serialization, and Markdown table cells escape pipe and newline characters before output. | verified |
| Stateful fixture isolation | Chainstate, mempool, and wallet benchmark cases create fresh or cloned mutable state per `run_once`, and tests prove stateful cases can run twice. | verified |
| Optional Knots path metadata boundary | CLI optional Knots paths are stored as metadata strings only, and tests pass missing paths to prove no file read or binary execution occurs. | verified |
| Shell wrapper argument safety | `scripts/run-benchmarks.sh` forwards only planned options through arrays, rejects unsupported combinations, and never constructs shell-evaluated commands. | verified |
| Repo verification smoke path | `scripts/verify.sh` creates the benchmark report directory and runs the bounded smoke wrapper as part of the repo-native verification contract. | verified |
| CI artifact hygiene | `.github/workflows/ci.yml` runs `bash scripts/verify.sh` and uploads only parity and benchmark report directories with `if-no-files-found: ignore`. | verified |
| Machine-readable audit root | `docs/parity/index.json` records the exact checklist taxonomy, 11 expected surfaces, final benchmark-readiness status, and release-readiness evidence root. | verified |
| Human-readable audit docs | Checklist, benchmark, deviations, and release-readiness docs preserve evidence, deferrals, follow-up triggers, and stale bookkeeping notes for reviewer audit. | verified |
| Panic-site guardrail | `scripts/check-panic-sites.sh` found no unclassified production panic-like sites. | verified |

## Accepted Risks Log

No accepted risks.

## Security Audit Trail

| Audit Date | Threats Total | Closed | Open | Run By |
|------------|---------------|--------|------|--------|
| 2026-04-26 | 20 | 20 | 0 | Codex |

## Verification Evidence

| Command | Result |
|---------|--------|
| Targeted `rg` scan for `<threat_model>` and `## Threat Flags` across Phase 10 plan and summary artifacts. | Found 20 declared threats across the five Phase 10 plans. Summaries reported no additional open threat flags. |
| Targeted source/doc review across `packages/open-bitcoin-bench`, `scripts/run-benchmarks.sh`, `scripts/verify.sh`, `.github/workflows/ci.yml`, and `docs/parity/*` audit docs. | Controls for static mappings, iteration caps, report hygiene, optional Knots metadata, wrapper argument forwarding, CI artifacts, checklist provenance, and readiness claims reviewed. |
| Targeted `rg` scan for shell eval, environment dumps, credential terms, and threshold status across benchmark code, wrapper, CI, and parity docs. | Found no shell-evaluated command construction, no credential dump path, and no benchmark threshold status field. |
| `bash scripts/check-panic-sites.sh` | Passed: no unclassified production panic-like sites. |
| `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-bench --all-features` | Passed: 15 library tests, 3 binary tests, and doc-tests passed. |
| `bash scripts/run-benchmarks.sh --list` | Passed: listed all seven required benchmark groups. |
| `bash scripts/run-benchmarks.sh --smoke --output-dir packages/target/benchmark-reports` | Passed: wrote smoke JSON and Markdown reports. |
| Node validation of `packages/target/benchmark-reports/open-bitcoin-bench-smoke.json` | Passed: schema version 1, mode `smoke:1`, seven expected groups, null optional Knots source, and no `threshold_status`. |
| Node validation of `docs/parity/index.json` | Passed: exact status taxonomy, 11 expected checklist surfaces, evidence for done surfaces, and release readiness `done`. |
| `git diff --check` | Passed. |
| `cargo fmt --manifest-path packages/Cargo.toml --all` | Passed. |
| `cargo clippy --manifest-path packages/Cargo.toml --all-targets --all-features -- -D warnings` | Passed. |
| `cargo build --manifest-path packages/Cargo.toml --all-targets --all-features` | Passed. |
| `cargo test --manifest-path packages/Cargo.toml --all-features` | Passed. |
| `bash scripts/verify.sh` | Passed. |

## Sign-Off

- [x] All threats have a disposition (mitigate / accept / transfer)
- [x] Accepted risks documented in Accepted Risks Log
- [x] `threats_open: 0` confirmed
- [x] `status: verified` set in frontmatter

Approval: verified 2026-04-26
