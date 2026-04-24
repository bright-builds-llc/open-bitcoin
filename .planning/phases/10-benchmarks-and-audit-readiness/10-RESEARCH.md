# Phase 10: Benchmarks and Audit Readiness - Research

**Researched:** 2026-04-24 [VERIFIED: environment current_date]  
**Domain:** Rust benchmark harnesses, Bitcoin Knots benchmark mapping, and repository-local parity audit artifacts [VERIFIED: .planning/phases/10-benchmarks-and-audit-readiness/10-CONTEXT.md]  
**Confidence:** HIGH for repo integration and audit structure; MEDIUM for exact benchmark fixture design until implementation validates fixture cost [VERIFIED: local code search] [ASSUMED]

<user_constraints>
## User Constraints (from CONTEXT.md)

Copied verbatim from `.planning/phases/10-benchmarks-and-audit-readiness/10-CONTEXT.md`. [VERIFIED: .planning/phases/10-benchmarks-and-audit-readiness/10-CONTEXT.md]

### Locked Decisions

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

### Deferred Ideas (OUT OF SCOPE)

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
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| PAR-02 | Benchmarks measure critical node and wallet performance paths and compare against the pinned baseline where meaningful. [VERIFIED: .planning/REQUIREMENTS.md] | Use a first-party benchmark binary that times Open Bitcoin public APIs, emits JSON and Markdown reports, and records a per-group Knots benchmark/source mapping; optional Knots JSON ingestion can compare external `bench_bitcoin -output-json` runs without making local verification build Knots. [VERIFIED: .planning/phases/10-benchmarks-and-audit-readiness/10-CONTEXT.md] [VERIFIED: packages/bitcoin-knots/doc/benchmarking.md] [VERIFIED: packages/bitcoin-knots/src/bench/bench_bitcoin.cpp] |
| AUD-01 | Contributors can inspect a parity checklist that reports each in-scope surface as planned, in progress, done, deferred, or out of scope. [VERIFIED: .planning/REQUIREMENTS.md] | Extend `docs/parity/index.json` as the machine-readable root with the exact status taxonomy, evidence links, known gaps, suspected unknowns, benchmark report links, and a concise Markdown readiness companion under `docs/parity/`. [VERIFIED: .planning/phases/10-benchmarks-and-audit-readiness/10-CONTEXT.md] [VERIFIED: docs/parity/index.json] [VERIFIED: docs/parity/README.md] |
</phase_requirements>

## Summary

Phase 10 should add a small first-party benchmark and audit-reporting surface, not a new production dependency or a dashboard. [VERIFIED: .planning/phases/10-benchmarks-and-audit-readiness/10-CONTEXT.md] The workspace already has the right integration pattern: Cargo workspace crates under `packages/`, Bazel smoke aliases from the repository root, and JSON plus Markdown report helpers in `open-bitcoin-test-harness`. [VERIFIED: packages/Cargo.toml] [VERIFIED: BUILD.bazel] [VERIFIED: packages/open-bitcoin-test-harness/src/report.rs] [VERIFIED: scripts/verify.sh]

The benchmark default should be a repo-owned binary crate, tentatively `open-bitcoin-bench`, using `std::time::Instant`, `std::hint::black_box`, existing first-party crates, and existing `serde_json`. [CITED: https://doc.rust-lang.org/std/time/struct.Instant.html] [CITED: https://doc.rust-lang.org/std/hint/fn.black_box.html] [VERIFIED: packages/Cargo.toml] [VERIFIED: packages/open-bitcoin-rpc/Cargo.toml] This satisfies the locked decision to avoid production runtime dependencies and keeps verification focused on "builds, smoke-runs, emits structured reports" instead of machine-specific thresholds. [VERIFIED: .planning/phases/10-benchmarks-and-audit-readiness/10-CONTEXT.md]

The audit default should extend `docs/parity/index.json` instead of creating a second source of truth. [VERIFIED: .planning/phases/10-benchmarks-and-audit-readiness/10-CONTEXT.md] [VERIFIED: docs/parity/index.json] The readiness artifact should summarize complete, deferred, unknown, verification evidence, benchmark evidence, and reviewer inspection points from existing phase verification files and catalog entries. [VERIFIED: .planning/phases/08-rpc-cli-and-config-parity/08-VERIFICATION.md] [VERIFIED: .planning/phases/09-parity-harnesses-and-fuzzing/09-VERIFICATION.md] [VERIFIED: docs/parity/catalog/*.md]

**Primary recommendation:** Build `open-bitcoin-bench` plus `scripts/run-benchmarks.sh`, make `scripts/verify.sh` call only smoke mode, update `docs/parity/index.json` with checklist/readiness roots, and add concise Markdown companions under `docs/parity/` for reviewer navigation. [VERIFIED: scripts/verify.sh] [VERIFIED: .planning/phases/10-benchmarks-and-audit-readiness/10-CONTEXT.md] [ASSUMED]

## Project Constraints (from AGENTS.md)

- Use `git submodule update --init --recursive` to materialize the pinned Knots baseline under `packages/bitcoin-knots`. [VERIFIED: AGENTS.md]
- Use `rust-toolchain.toml` as the Rust source of truth; this repo pins Rust `1.94.1`. [VERIFIED: AGENTS.md] [VERIFIED: rust-toolchain.toml]
- Use `bash scripts/verify.sh` as the repo-native verification contract for first-party code, including the Bazel smoke build. [VERIFIED: AGENTS.md] [VERIFIED: scripts/verify.sh]
- Install repo-managed hooks with `bash scripts/install-git-hooks.sh` once per clone. [VERIFIED: AGENTS.md]
- Record intentional in-scope behavior differences from Bitcoin Knots in `docs/parity/index.json` and companion docs under `docs/parity/`. [VERIFIED: AGENTS.md] [VERIFIED: docs/parity/README.md]
- Keep Open Bitcoin headless for this milestone; GUI and public dashboard work are out of scope. [VERIFIED: AGENTS.md] [VERIFIED: .planning/PROJECT.md]
- Preserve behavioral parity with Bitcoin Knots `29.3.knots20260210` for in-scope surfaces, with auditable parity claims. [VERIFIED: AGENTS.md] [VERIFIED: .planning/PROJECT.md]
- Keep pure business logic free of filesystem, socket, wall-clock, environment, process, thread, async-runtime, and randomness dependencies. [VERIFIED: AGENTS.md] [VERIFIED: .planning/PROJECT.md]
- Do not use existing Rust Bitcoin libraries in the production path. [VERIFIED: AGENTS.md] [VERIFIED: .planning/PROJECT.md]
- Before a git commit in this Rust project, run `cargo fmt --all`, `cargo clippy --all-targets --all-features -- -D warnings`, `cargo build --all-targets --all-features`, and `cargo test --all-features`, adapted to this repo's manifest and native verification entrypoint. [VERIFIED: AGENTS.md] [VERIFIED: scripts/verify.sh]
- Rust module additions should prefer `foo.rs` plus `foo/` over `foo/mod.rs`, avoid `unwrap()`, prefer `let...else` for guard extraction, and use `tracing` instead of `println!` for application logging. [VERIFIED: AGENTS.md] [CITED: https://raw.githubusercontent.com/bright-builds-llc/bright-builds-rules/main/standards/languages/rust.md]
- Unit tests should test behavior, one concern per test, and use Arrange, Act, Assert comments unless trivially obvious. [VERIFIED: AGENTS.md] [CITED: https://raw.githubusercontent.com/bright-builds-llc/bright-builds-rules/main/standards/core/testing.md]
- No project-local `.claude/skills/` or `.agents/skills/` directories were found. [VERIFIED: local directory scan]
- `standards-overrides.md` exists but contains only placeholder override rows, so no substantive local standards exception was found. [VERIFIED: standards-overrides.md]

## Standard Stack

### Core

| Library / Tool | Version | Purpose | Why Standard |
|---|---:|---|---|
| Rust `std::time::Instant` | Rust 1.94.1 project toolchain; std docs checked against 1.95.0 current docs | Monotonic-ish elapsed timing for repo-owned smoke/trend measurements. [CITED: https://doc.rust-lang.org/std/time/struct.Instant.html] [VERIFIED: rust-toolchain.toml] | It is available without a new dependency and is explicitly useful for timing operations, while docs warn it is not a steady clock and platform behavior varies. [CITED: https://doc.rust-lang.org/std/time/struct.Instant.html] |
| Rust `std::hint::black_box` | Rust 1.94.1 project toolchain | Prevent pure benchmark inputs/results from being optimized away. [CITED: https://doc.rust-lang.org/std/hint/fn.black_box.html] | Rust docs state `black_box` is generally relied on for benchmarking and should be used there, while not treating it as a cryptographic constant-time primitive. [CITED: https://doc.rust-lang.org/std/hint/fn.black_box.html] |
| First-party `open-bitcoin-*` crates | Workspace `0.1.0` | Benchmark public node, wallet, consensus, chainstate, mempool, networking, codec, RPC, and CLI surfaces. [VERIFIED: packages/Cargo.toml] [VERIFIED: rg public API scan] | Phase 10 is about Open Bitcoin's first-party performance paths and must not introduce existing Rust Bitcoin libraries into production behavior. [VERIFIED: .planning/phases/10-benchmarks-and-audit-readiness/10-CONTEXT.md] [VERIFIED: AGENTS.md] |
| `serde_json` | `1.0.149`, published 2026-01-06, current via `cargo info` and crates.io API | Emit stable JSON benchmark/checklist/readiness reports and optionally parse external Knots JSON. [VERIFIED: cargo info serde_json] [VERIFIED: crates.io API] | The workspace already uses `serde_json` for RPC/reporting, so reuse avoids a new JSON dependency. [VERIFIED: packages/open-bitcoin-rpc/Cargo.toml] [VERIFIED: packages/open-bitcoin-test-harness/Cargo.toml] |
| Bitcoin Knots `bench_bitcoin` | Baseline `29.3.knots20260210` vendored under `packages/bitcoin-knots` | Optional external benchmark result source and authoritative mapping target. [VERIFIED: docs/parity/index.json] [VERIFIED: packages/bitcoin-knots/doc/benchmarking.md] | Upstream supports benchmark filtering, listing, sanity check, CSV output, and JSON output; Phase 10 can map to it without requiring a local build by default. [VERIFIED: packages/bitcoin-knots/src/bench/bench_bitcoin.cpp] |

### Supporting

| Library / Tool | Version | Purpose | When to Use |
|---|---:|---|---|
| `serde` | `1.0.228`, published 2025-09-27, current via `cargo info` and crates.io API | Derive typed report structs only if the planner chooses strongly typed report serialization. [VERIFIED: cargo info serde] [VERIFIED: crates.io API] | Use only if report schemas become clearer with `Serialize`; otherwise `serde_json::json!` is enough. [VERIFIED: packages/open-bitcoin-test-harness/src/report.rs] |
| `jq` | `1.7.1-apple` available locally | Optional local inspection of generated JSON. [VERIFIED: environment availability audit] | Do not make it required for verification unless a script checks availability and has a no-`jq` fallback. [VERIFIED: scripts/verify.sh command requirements pattern] |
| CMake, Ninja, C++ compiler | CMake 3.27.9, Ninja 1.13.2, Apple clang 21 available locally | Optional build path for Knots `bench_bitcoin`. [VERIFIED: environment availability audit] | Use only for opt-in baseline benchmark generation, because D-03 forbids making normal local verification depend on building/running full Knots benchmarks unless proven practical. [VERIFIED: .planning/phases/10-benchmarks-and-audit-readiness/10-CONTEXT.md] |

### Alternatives Considered

| Instead of | Could Use | Tradeoff |
|---|---|---|
| Repo-owned `Instant` harness | `criterion = 0.8.2`, published 2026-02-04, Rust MSRV 1.86 [VERIFIED: cargo info criterion] [VERIFIED: crates.io API] | Criterion is statistics-driven and produces warmup/sample analysis, but default features add `rayon`, `plotters`, and cargo bench support; this is more dependency and output machinery than Phase 10 needs for smoke/trend audit reports. [CITED: https://bheisler.github.io/criterion.rs/book/user_guide/command_line_output.html] [VERIFIED: cargo info criterion] [VERIFIED: .planning/phases/10-benchmarks-and-audit-readiness/10-CONTEXT.md] |
| Repo-owned `Instant` harness | `divan = 0.1.21`, published 2025-04-10, Rust MSRV 1.80 [VERIFIED: cargo info divan] [VERIFIED: crates.io API] | Divan is smaller than Criterion but still adds a new benchmark framework; it does not solve the Phase 10-specific Knots mapping and audit JSON schema by itself. [VERIFIED: cargo info divan] [ASSUMED] |
| Repo-owned `Instant` harness | Rust `#[bench]` / `test::Bencher` | The Rust benchmark harness is behind the unstable `test` feature gate, so it conflicts with the pinned stable toolchain workflow. [CITED: https://doc.rust-lang.org/unstable-book/library-features/test.html] [VERIFIED: rust-toolchain.toml] |
| Optional Knots JSON ingestion | Build and run `packages/bitcoin-knots/build/bin/bench_bitcoin` during `scripts/verify.sh` | This gives direct baseline measurements but makes local verification slower and more environment-sensitive; D-03 says not to make that default unless the plan proves determinism and practicality. [VERIFIED: packages/bitcoin-knots/doc/benchmarking.md] [VERIFIED: .planning/phases/10-benchmarks-and-audit-readiness/10-CONTEXT.md] |

**Installation:**

```bash
# No new crates are recommended for the default Phase 10 benchmark path.
# If typed report derives are used, reuse existing workspace dependencies:
# serde = { version = "1.0.228", features = ["derive"] }
# serde_json = "1.0.149"
```

**Version verification:** `cargo info serde_json`, `cargo info serde`, `cargo info criterion`, `cargo info divan`, and crates.io version API requests verified the current versions and publish dates listed above on 2026-04-24. [VERIFIED: cargo info output] [VERIFIED: crates.io API]

## Architecture Patterns

### Recommended Project Structure

```text
packages/
|-- open-bitcoin-bench/
|   |-- Cargo.toml
|   |-- BUILD.bazel
|   `-- src/
|       |-- main.rs              # CLI shell: args, output dir, smoke/full selection
|       |-- report.rs            # JSON/Markdown report writing, modeled after test-harness reports
|       |-- runner.rs            # stable timing loop over registered benchmark cases
|       |-- registry.rs          # benchmark group list and Knots mapping metadata
|       |-- fixtures.rs          # deterministic first-party fixture builders
|       `-- cases/
|           |-- consensus.rs
|           |-- chainstate.rs
|           |-- mempool.rs
|           |-- network.rs
|           |-- wallet.rs
|           `-- rpc_cli.rs
scripts/
`-- run-benchmarks.sh            # default smoke command invoked by verify.sh
docs/parity/
|-- index.json                   # machine-readable root, extended with checklist/readiness/benchmarks
|-- benchmarks.md                # concise benchmark mapping and latest stable sample path
|-- checklist.md                 # status taxonomy view for reviewers
`-- release-readiness.md         # milestone handoff and reviewer checklist
```

This structure keeps benchmark measurement in a first-party workspace package and keeps generated timing output under `packages/target/benchmark-reports` by default. [VERIFIED: packages/Cargo.toml] [VERIFIED: .gitignore] [VERIFIED: .planning/phases/10-benchmarks-and-audit-readiness/10-CONTEXT.md] [ASSUMED]

### Pattern 1: First-Party Benchmark Binary

**What:** Add a binary crate that registers benchmark cases as data, runs each case for a bounded number of iterations, and emits a JSON object plus a Markdown summary. [VERIFIED: packages/open-bitcoin-test-harness/src/report.rs] [CITED: https://doc.rust-lang.org/std/time/struct.Instant.html]

**When to use:** Use for default local and CI smoke execution, because the phase requires structured report generation but forbids wall-clock thresholds as default gates. [VERIFIED: .planning/phases/10-benchmarks-and-audit-readiness/10-CONTEXT.md]

**Example:**

```rust
// Source: Rust std Instant and black_box docs; adapt into open-bitcoin-bench/src/runner.rs.
use std::hint::black_box;
use std::time::{Duration, Instant};

pub struct BenchCase {
    pub id: &'static str,
    pub group: &'static str,
    pub knots_mapping: &'static [&'static str],
    pub run_once: fn() -> Result<(), BenchError>,
}

pub fn run_case(case: &BenchCase, iterations: u64) -> Result<BenchResult, BenchError> {
    let start = Instant::now();
    for _ in 0..iterations {
        black_box((case.run_once)()?);
    }
    let elapsed = start.elapsed();
    Ok(BenchResult::from_elapsed(case, iterations, elapsed))
}
```

This pattern uses `black_box` so pure computations remain observable to the optimizer and uses `Instant` for elapsed durations without adding a benchmark framework dependency. [CITED: https://doc.rust-lang.org/std/hint/fn.black_box.html] [CITED: https://doc.rust-lang.org/std/time/struct.Instant.html]

### Pattern 2: Knots Mapping Before Knots Execution

**What:** Store each Open Bitcoin benchmark's relevant Knots source files and benchmark names in the report schema. [VERIFIED: .planning/phases/10-benchmarks-and-audit-readiness/10-CONTEXT.md] [VERIFIED: packages/bitcoin-knots/src/bench/*.cpp]

**When to use:** Use for every benchmark group; actual Knots numbers should be optional and imported from `bench_bitcoin -output-json` or an explicitly provided JSON file. [VERIFIED: packages/bitcoin-knots/src/bench/bench_bitcoin.cpp] [VERIFIED: packages/bitcoin-knots/doc/benchmarking.md]

**Example mapping targets:** `VerifyScriptBench`, `VerifyNestedIfScript`, `DeserializeBlockTest`, `DeserializeAndCheckBlockTest`, `ComplexMemPool`, `MempoolCheck`, `RpcMempool`, `WalletBalance*`, `CoinSelection`, `BnBExhaustion`, `WalletCreateTx*`, `WalletAvailableCoins`, `AddrMan*`, and `EvictionProtection*`. [VERIFIED: packages/bitcoin-knots/src/bench/verify_script.cpp] [VERIFIED: packages/bitcoin-knots/src/bench/checkblock.cpp] [VERIFIED: packages/bitcoin-knots/src/bench/mempool_stress.cpp] [VERIFIED: packages/bitcoin-knots/src/bench/rpc_mempool.cpp] [VERIFIED: packages/bitcoin-knots/src/bench/wallet_balance.cpp] [VERIFIED: packages/bitcoin-knots/src/bench/coin_selection.cpp] [VERIFIED: packages/bitcoin-knots/src/bench/wallet_create_tx.cpp] [VERIFIED: packages/bitcoin-knots/src/bench/addrman.cpp] [VERIFIED: packages/bitcoin-knots/src/bench/peer_eviction.cpp]

### Pattern 3: Checklist as Data, Markdown as View

**What:** Add checklist/readiness nodes to `docs/parity/index.json` and generate or hand-maintain concise Markdown views under `docs/parity/`. [VERIFIED: docs/parity/index.json] [VERIFIED: docs/parity/README.md] [VERIFIED: .planning/phases/10-benchmarks-and-audit-readiness/10-CONTEXT.md]

**When to use:** Use for `AUD-01`, because the requirement is an inspectable parity checklist and D-08 locks `docs/parity/index.json` as the machine-readable root. [VERIFIED: .planning/REQUIREMENTS.md] [VERIFIED: .planning/phases/10-benchmarks-and-audit-readiness/10-CONTEXT.md]

**Recommended JSON shape:**

```json
{
  "checklist": {
    "taxonomy": ["planned", "in_progress", "done", "deferred", "out_of_scope"],
    "surfaces": [
      {
        "id": "consensus-validation",
        "status": "done",
        "evidence": ["catalog/consensus-validation.md"],
        "deferred": [],
        "suspected_unknowns": []
      }
    ]
  },
  "benchmark_reports": {
    "default_output_dir": "packages/target/benchmark-reports",
    "stable_sample": "docs/parity/benchmark-sample.json"
  },
  "readiness": {
    "verification_commands": ["bash scripts/verify.sh"],
    "release_blockers": [],
    "reviewer_focus": []
  }
}
```

This shape preserves `index.json` as root while avoiding duplication of large phase reports. [VERIFIED: .planning/phases/10-benchmarks-and-audit-readiness/10-CONTEXT.md] [VERIFIED: docs/parity/index.json]

### Anti-Patterns to Avoid

- **Hard timing release gates:** Wall-clock thresholds are machine-dependent and D-04 says benchmark results are for trend and audit review, not flaky release gates. [VERIFIED: .planning/phases/10-benchmarks-and-audit-readiness/10-CONTEXT.md]
- **Benchmarking setup work by accident:** Knots benchmark docs and code keep setup outside the timed loop, and Rust unstable benchmark guidance says setup should be outside the iteration. [VERIFIED: packages/bitcoin-knots/src/bench/verify_script.cpp] [CITED: https://doc.rust-lang.org/unstable-book/library-features/test.html]
- **Accumulating mutable state across iterations:** Knots benchmark comments recreate mutable objects when repeated runs would change preconditions, such as `AddrManAddThenGood`. [VERIFIED: packages/bitcoin-knots/src/bench/addrman.cpp]
- **Second parity source of truth:** D-08 locks `docs/parity/index.json` as root, so Markdown must be a review view rather than an independent checklist database. [VERIFIED: .planning/phases/10-benchmarks-and-audit-readiness/10-CONTEXT.md]
- **Broad panic/illegal-state refactor creep:** The panic sweep todo is folded as audit risk capture only, not broad code refactoring. [VERIFIED: .planning/phases/10-benchmarks-and-audit-readiness/10-CONTEXT.md] [VERIFIED: .planning/todos/pending/2026-04-18-sweep-panics-and-illegal-states.md]

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---|---|---|---|
| JSON serialization and parsing | Ad hoc string concatenation or manual escaping | `serde_json` and typed report structs or `json!` values | Existing report helpers already use `serde_json` for stable JSON, and manual JSON escaping risks invalid or injectable reports. [VERIFIED: packages/open-bitcoin-test-harness/src/report.rs] [VERIFIED: packages/open-bitcoin-test-harness/Cargo.toml] |
| Knots benchmark execution | A custom parser for C++ benchmark registration or a bespoke C++ runner | Upstream `bench_bitcoin` with `-filter`, `-list`, `-sanity-check`, and `-output-json` when execution is explicitly enabled | The vendored runner already owns the Knots benchmark registry and JSON output path. [VERIFIED: packages/bitcoin-knots/src/bench/bench_bitcoin.cpp] |
| Statistical benchmarking framework | A partial reimplementation of Criterion-style confidence intervals | Either no statistics in Phase 10 smoke reports, or add Criterion later as a deliberate dev-only dependency | Criterion already performs warmup, sample collection, confidence intervals, and change reports; Phase 10 does not need to clone that complexity. [CITED: https://bheisler.github.io/criterion.rs/book/user_guide/command_line_output.html] [VERIFIED: cargo info criterion] |
| Cryptographic benchmarks | New benchmark-only crypto implementations | Existing first-party consensus, wallet, and secp256k1-backed APIs | Production path avoids third-party Rust Bitcoin libraries and should not get alternate benchmark-only behavior. [VERIFIED: AGENTS.md] [VERIFIED: packages/open-bitcoin-consensus/Cargo.toml] |
| Checklist taxonomy | Free-form status strings in multiple docs | Exact taxonomy `planned`, `in_progress`, `done`, `deferred`, `out_of_scope` in `docs/parity/index.json` | D-07 locks the taxonomy and D-08 locks the machine-readable root. [VERIFIED: .planning/phases/10-benchmarks-and-audit-readiness/10-CONTEXT.md] |

**Key insight:** Phase 10's hard problem is auditability, not high-fidelity performance science; use simple timing plus strong metadata now, and leave deeper statistical benchmarking for a later performance-optimization phase. [VERIFIED: .planning/phases/10-benchmarks-and-audit-readiness/10-CONTEXT.md] [ASSUMED]

## Common Pitfalls

### Pitfall 1: Flaky Timing Gates
**What goes wrong:** A normal laptop, CI runner, or background process changes wall-clock timings enough to fail a release gate. [VERIFIED: .planning/phases/10-benchmarks-and-audit-readiness/10-CONTEXT.md] [CITED: https://doc.rust-lang.org/std/time/struct.Instant.html]  
**Why it happens:** `Instant` is not guaranteed to be steady, and D-04 forbids default wall-clock thresholds. [CITED: https://doc.rust-lang.org/std/time/struct.Instant.html] [VERIFIED: .planning/phases/10-benchmarks-and-audit-readiness/10-CONTEXT.md]  
**How to avoid:** Verify that benchmark smoke mode builds, executes bounded iterations, and emits parseable JSON/Markdown. [VERIFIED: .planning/phases/10-benchmarks-and-audit-readiness/10-CONTEXT.md]  
**Warning signs:** A plan proposes `ns_per_iter < X` as a default CI assertion. [ASSUMED]

### Pitfall 2: Optimizer-Erased Pure Work
**What goes wrong:** Pure functions with unused results are optimized away, producing unrealistically low times. [CITED: https://doc.rust-lang.org/std/hint/fn.black_box.html]  
**Why it happens:** Rust docs show the compiler can remove pure unused calculations during optimization. [CITED: https://doc.rust-lang.org/std/hint/fn.black_box.html]  
**How to avoid:** Wrap inputs and results with `std::hint::black_box`. [CITED: https://doc.rust-lang.org/std/hint/fn.black_box.html]  
**Warning signs:** A benchmark closure calls a pure parser, validator, or encoder and discards the result without `black_box` or an assertion. [ASSUMED]

### Pitfall 3: Measuring Fixture Setup Instead of the Operation
**What goes wrong:** The reported benchmark is dominated by test fixture construction, not the node or wallet path being evaluated. [VERIFIED: packages/bitcoin-knots/src/bench/verify_script.cpp] [CITED: https://doc.rust-lang.org/unstable-book/library-features/test.html]  
**Why it happens:** Setup code is placed inside the timing loop. [CITED: https://doc.rust-lang.org/unstable-book/library-features/test.html]  
**How to avoid:** Build transactions, blocks, wallets, chain snapshots, and peer fixtures before timing unless the setup itself is the intended benchmark. [VERIFIED: packages/bitcoin-knots/src/bench/checkblock.cpp] [VERIFIED: packages/bitcoin-knots/src/bench/wallet_balance.cpp]  
**Warning signs:** Fixture builders allocate large vectors or import descriptors inside `run_once` for a benchmark named after validation/encoding. [ASSUMED]

### Pitfall 4: Mutable State Changes Each Iteration
**What goes wrong:** Each iteration measures different preconditions because the benchmark mutates shared state. [VERIFIED: packages/bitcoin-knots/src/bench/addrman.cpp]  
**Why it happens:** State like chainstate, mempool, wallet UTXOs, or peer sets is reused without reset or cloned fixtures. [VERIFIED: packages/open-bitcoin-chainstate/src/engine.rs] [VERIFIED: packages/open-bitcoin-mempool/src/pool.rs]  
**How to avoid:** Clone prepared snapshots, reconstruct mutable structures per iteration, or make the mutation itself the benchmark with documented preconditions. [VERIFIED: packages/bitcoin-knots/src/bench/addrman.cpp] [ASSUMED]  
**Warning signs:** Iteration counts change the number of UTXOs, mempool entries, or peers available to later iterations. [ASSUMED]

### Pitfall 5: Checklist Drift
**What goes wrong:** `docs/parity/index.json`, Markdown catalog pages, requirements traceability, and phase verification files disagree. [VERIFIED: .planning/STATE.md] [VERIFIED: .planning/ROADMAP.md] [VERIFIED: docs/parity/index.json]  
**Why it happens:** Some planning/state files are stale relative to verified Phase 8 and Phase 9 artifacts. [VERIFIED: .planning/STATE.md] [VERIFIED: .planning/phases/08-rpc-cli-and-config-parity/08-VERIFICATION.md] [VERIFIED: .planning/phases/09-parity-harnesses-and-fuzzing/09-VERIFICATION.md]  
**How to avoid:** Treat discrepancy recording as an audit output and use deterministic GSD tooling where possible instead of hand-rewriting unrelated history. [VERIFIED: .planning/phases/10-benchmarks-and-audit-readiness/10-CONTEXT.md]  
**Warning signs:** A checklist item says `done` without evidence links, or a deferred item lacks rationale. [VERIFIED: .planning/phases/10-benchmarks-and-audit-readiness/10-CONTEXT.md]

## Code Examples

Verified patterns from official or local sources.

### Stable Report Writer Pattern

```rust
// Source: packages/open-bitcoin-test-harness/src/report.rs
fn write_json_report(report: &SuiteReport, path: &Path) -> Result<(), ReportError> {
    let outcomes = report
        .outcomes
        .iter()
        .map(|outcome| {
            serde_json::json!({
                "case": outcome.case_name,
                "passed": outcome.passed,
                "detail": outcome.detail,
            })
        })
        .collect::<Vec<_>>();
    let body = serde_json::json!({
        "suite": report.suite,
        "target": report.target,
        "skipped": report.skipped,
        "outcomes": outcomes,
    });
    std::fs::write(path, serde_json::to_string_pretty(&body).expect("json report"))?;
    Ok(())
}
```

This is the existing local pattern for machine-readable-first reports with Markdown companions. [VERIFIED: packages/open-bitcoin-test-harness/src/report.rs]

### Knots Benchmark CLI Contract

```text
bench_bitcoin -filter=<regex> -sanity-check -output-json=<output.json>
```

`bench_bitcoin` defines `-filter`, `-list`, `-min-time`, `-output-csv`, `-output-json`, `-sanity-check`, and `-priority-level`. [VERIFIED: packages/bitcoin-knots/src/bench/bench_bitcoin.cpp]

### Open Bitcoin Benchmark Case Targets

```rust
// Source: public API scan across first-party crates.
let consensus_case = || open_bitcoin_consensus::check_block(&block).map(|_| ());
let codec_case = || open_bitcoin_codec::encode_block(&block).map(|_| ());
let network_case = || message.encode_wire(open_bitcoin_primitives::NetworkMagic::MAINNET).map(|_| ());
let wallet_case = || wallet.balance(100).map(|_| ());
```

These functions are public first-party APIs and align with Phase 10's critical-path coverage list. [VERIFIED: packages/open-bitcoin-consensus/src/lib.rs] [VERIFIED: packages/open-bitcoin-codec/src/lib.rs] [VERIFIED: packages/open-bitcoin-network/src/message.rs] [VERIFIED: packages/open-bitcoin-wallet/src/wallet.rs] [VERIFIED: .planning/phases/10-benchmarks-and-audit-readiness/10-CONTEXT.md]

## Benchmark Coverage Map

| Open Bitcoin Group | First-Party API Surface | Knots Mapping | Notes |
|---|---|---|---|
| consensus-script | `eval_script`, `verify_script`, `verify_input_script`, `validate_transaction_with_context` [VERIFIED: packages/open-bitcoin-consensus/src/lib.rs] | `VerifyScriptBench`, `VerifyNestedIfScript` [VERIFIED: packages/bitcoin-knots/src/bench/verify_script.cpp] | Keep signature fixtures prepared outside the measured loop. [VERIFIED: packages/bitcoin-knots/src/bench/verify_script.cpp] |
| block-codec-validation | `parse_block`, `encode_block`, `check_block`, `validate_block_with_context` [VERIFIED: packages/open-bitcoin-codec/src/lib.rs] [VERIFIED: packages/open-bitcoin-consensus/src/lib.rs] | `DeserializeBlockTest`, `DeserializeAndCheckBlockTest`, `SaveBlockBench`, `ReadBlockBench`, `ReadRawBlockBench` [VERIFIED: packages/bitcoin-knots/src/bench/checkblock.cpp] [VERIFIED: packages/bitcoin-knots/src/bench/readwriteblock.cpp] | Open Bitcoin currently lacks disk blockstorage parity, so disk read/write mapping should be documentation-only or deferred. [VERIFIED: docs/parity/catalog/chainstate.md] [ASSUMED] |
| chainstate | `Chainstate::connect_block`, `disconnect_tip`, `reorg` [VERIFIED: packages/open-bitcoin-chainstate/src/engine.rs] | `CCoinsCaching`, `CheckBlockIndex`, block connect/check-related Knots validation surfaces [VERIFIED: packages/bitcoin-knots/src/bench/ccoins_caching.cpp] [VERIFIED: packages/bitcoin-knots/src/bench/checkblockindex.cpp] [VERIFIED: packages/bitcoin-knots/src/bench/checkblock.cpp] | Use cloned snapshots to avoid cross-iteration mutation. [VERIFIED: packages/open-bitcoin-chainstate/src/engine.rs] [ASSUMED] |
| mempool-policy | `Mempool::accept_transaction`, policy helpers, trim/eviction side effects through admission [VERIFIED: packages/open-bitcoin-mempool/src/pool.rs] [VERIFIED: packages/open-bitcoin-mempool/src/policy.rs] | `ComplexMemPool`, `MempoolCheck`, `RpcMempool`, `MempoolEviction` [VERIFIED: packages/bitcoin-knots/src/bench/mempool_stress.cpp] [VERIFIED: packages/bitcoin-knots/src/bench/rpc_mempool.cpp] [VERIFIED: packages/bitcoin-knots/src/bench/mempool_eviction.cpp] | Existing public API exposes admission but not a public trim-only method. [VERIFIED: packages/open-bitcoin-mempool/src/pool.rs] |
| network-wire-sync | `WireNetworkMessage::encode_wire`, `ParsedNetworkMessage::decode_wire`, `HeaderStore::locator`, `PeerManager::handle_message` [VERIFIED: packages/open-bitcoin-network/src/message.rs] [VERIFIED: packages/open-bitcoin-network/src/header_store.rs] [VERIFIED: packages/open-bitcoin-network/src/peer.rs] | `AddrMan*`, `EvictionProtection*`, P2P message and sync code sources [VERIFIED: packages/bitcoin-knots/src/bench/addrman.cpp] [VERIFIED: packages/bitcoin-knots/src/bench/peer_eviction.cpp] [VERIFIED: docs/parity/catalog/p2p.md] | Open Bitcoin does not yet implement addrman or peer eviction parity, so those mappings should be marked deferred/out_of_scope for current benchmarks. [VERIFIED: docs/parity/catalog/p2p.md] |
| wallet | `Wallet::rescan_chainstate`, `balance`, `build_transaction`, `sign_transaction`, `build_and_sign` [VERIFIED: packages/open-bitcoin-wallet/src/wallet.rs] | `WalletBalance*`, `CoinSelection`, `BnBExhaustion`, `WalletCreateTx*`, `WalletAvailableCoins`, `SignTransaction*` [VERIFIED: packages/bitcoin-knots/src/bench/wallet_balance.cpp] [VERIFIED: packages/bitcoin-knots/src/bench/coin_selection.cpp] [VERIFIED: packages/bitcoin-knots/src/bench/wallet_create_tx.cpp] [VERIFIED: packages/bitcoin-knots/src/bench/sign_transaction.cpp] | Wallet fixtures can reuse existing wallet tests as design references, but planner must confirm fixture constructors stay private or support-only. [VERIFIED: packages/open-bitcoin-wallet/src/wallet/tests.rs] [ASSUMED] |
| rpc-cli | `dispatch`, method normalization, CLI arg parsing/output rendering [VERIFIED: packages/open-bitcoin-rpc/src/dispatch.rs] [VERIFIED: packages/open-bitcoin-rpc/src/method.rs] [VERIFIED: packages/open-bitcoin-cli/src/args.rs] [VERIFIED: packages/open-bitcoin-cli/src/output.rs] | `RpcMempool`, `BlockToJson*`, `interface_bitcoin_cli.py` docs/tests [VERIFIED: packages/bitcoin-knots/src/bench/rpc_mempool.cpp] [VERIFIED: docs/parity/catalog/rpc-cli-config.md] | Only cheap dispatch/normalization/rendering paths should be benchmarked by default. [VERIFIED: .planning/phases/10-benchmarks-and-audit-readiness/10-CONTEXT.md] |

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|---|---|---|---|
| Rust `#[bench]` with `test::Bencher` | Stable custom harness or external framework on stable Rust | `test` benchmarking remains unstable in the Rust Unstable Book [CITED: https://doc.rust-lang.org/unstable-book/library-features/test.html] | Do not plan nightly or `#![feature(test)]` in this stable Rust 1.94.1 repo. [VERIFIED: rust-toolchain.toml] |
| C++ baseline benchmark without structured output | Knots `bench_bitcoin` with `-output-json` and `-output-csv` options | Current vendored Knots runner includes these flags [VERIFIED: packages/bitcoin-knots/src/bench/bench_bitcoin.cpp] | Phase 10 can ingest optional JSON instead of screen-scraping tables. [VERIFIED: packages/bitcoin-knots/src/bench/bench_bitcoin.cpp] [ASSUMED] |
| Benchmark-only timing tables | JSON-first reports with Markdown summaries and artifact upload | Phase 9 already emits parity JSON/Markdown reports and CI uploads `packages/target/parity-reports` [VERIFIED: packages/open-bitcoin-test-harness/src/report.rs] [VERIFIED: .github/workflows/ci.yml] | Phase 10 should follow the existing report-output pattern under `packages/target/benchmark-reports`. [VERIFIED: .gitignore] [ASSUMED] |
| Public progress dashboard | Repo-local audit checklist/readiness docs | v2 `OBS-01` owns richer public dashboards, while Phase 10 defers them [VERIFIED: .planning/REQUIREMENTS.md] [VERIFIED: .planning/phases/10-benchmarks-and-audit-readiness/10-CONTEXT.md] | Keep outputs reviewable in git and CI artifacts. [VERIFIED: .planning/phases/10-benchmarks-and-audit-readiness/10-CONTEXT.md] |

**Deprecated/outdated:**
- Rust `test` benchmark harness for this repo: it requires the unstable `test` feature, so it is not compatible with the stable pinned Rust toolchain. [CITED: https://doc.rust-lang.org/unstable-book/library-features/test.html] [VERIFIED: rust-toolchain.toml]
- Screen-scraping Knots benchmark tables: the runner supports JSON output directly. [VERIFIED: packages/bitcoin-knots/src/bench/bench_bitcoin.cpp]

## Assumptions Log

| # | Claim | Section | Risk if Wrong |
|---|---|---|---|
| A1 | Exact benchmark fixture construction cost is still uncertain until implementation measures smoke runtime. | Metadata / Summary | Planner might put too many cases into `scripts/verify.sh` smoke mode. |
| A2 | A dedicated `open-bitcoin-bench` binary plus `scripts/run-benchmarks.sh` is clearer than adding benchmark code to `open-bitcoin-test-harness`. | Summary / Architecture Patterns | Planner may choose a different package boundary if code reuse is stronger inside the harness crate. |
| A3 | Divan does not materially improve the Phase 10-specific Knots mapping and audit JSON problem enough to justify adding it. | Alternatives Considered | If Divan's current reporting exactly fits the desired JSON/Markdown schema, a dev-only dependency could be reconsidered. |
| A4 | The recommended project structure will keep benchmark measurement isolated in a first-party workspace package and generated timing output under `packages/target/benchmark-reports`. | Architecture Patterns | If CI artifact paths or local cleanup expectations prefer a different target subdirectory, report paths need adjustment. |
| A5 | Phase 10's hard problem is auditability rather than high-fidelity performance science. | Don't Hand-Roll | Planner might underinvest in statistical benchmarking if the user expects precise performance-regression analysis now. |
| A6 | Timing-threshold, discarded-result, setup-in-loop, and mutable-state warning signs are useful planning heuristics. | Common Pitfalls | Planner may include weak verification checks or overfit to these heuristics without implementation proof. |
| A7 | Cloning prepared snapshots or reconstructing mutable structures is the right default for stateful benchmark cases. | Common Pitfalls / Benchmark Coverage Map | Planner may need a cheaper reset mechanism if cloning dominates smoke runtime. |
| A8 | Disk blockstorage benchmark mappings should be documentation-only or deferred for current Open Bitcoin because disk-backed blockstorage is not in the implemented chainstate slice. | Benchmark Coverage Map | Planner may need to add an explicit `deferred` checklist item rather than a benchmark case. |
| A9 | Benchmark fixtures can be adapted from existing tests without broad production visibility changes. | Benchmark Coverage Map | Planner may need to add narrow support constructors or keep some benchmark groups smaller. |
| A10 | Open Bitcoin can ingest optional Knots JSON instead of screen-scraping tables. | State of the Art / Resolved Benchmark Scope Choices | Planner should support an explicit optional Knots JSON path and keep mapping-only behavior as the default. |
| A11 | A new first-party `open-bitcoin-bench` crate keeps benchmark registration and CLI concerns isolated from parity test helpers. | Resolved Benchmark Scope Choices | Use the new crate boundary; share only patterns from `open-bitcoin-test-harness`, not crate ownership. |
| A12 | Full local Knots benchmark builds are optional and not part of default contributor verification. | Resolved Benchmark Scope Choices | Default comparison is mapping-only; optional Knots JSON/bin inputs may enrich reports when explicitly supplied. |
| A13 | `scripts/run-benchmarks.sh --smoke` should be invoked by `scripts/verify.sh` only after smoke mode is bounded. | Resolved Benchmark Scope Choices | Plan bounded smoke iterations and report-schema checks before adding the `verify.sh` call. |
| A14 | CMake's default generator is an acceptable fallback when Ninja is unavailable for optional Knots builds. | Environment Availability | Planner may need a more explicit platform-specific Knots build recipe. |
| A15 | `jq` should not be required by default for report inspection. | Environment Availability | If scripts rely on shell-side JSON validation, planner must add a Rust or Node fallback. |
| A16 | Benchmark IDs can be static registry data and Markdown cells can be escaped to mitigate report injection. | Security Domain | If benchmark names/details accept external input, stronger validation is needed. |
| A17 | Bounded `--smoke` defaults mitigate denial-of-service risk from unbounded benchmark fixture sizes. | Security Domain | Planner may need max iteration/case count enforcement in both CLI args and scripts. |
| A18 | Research validity dates are estimates: 30 days for repo architecture findings and 7 days for crate/security version claims. | Metadata | Planner may rely on stale dependency or security-standard facts if planning is delayed. |

## Open Questions (RESOLVED)

1. **Should `open-bitcoin-bench` live as a new crate or inside `open-bitcoin-test-harness`?**  
   - Resolution: Use a new first-party `open-bitcoin-bench` crate. [VERIFIED: packages/Cargo.toml] [VERIFIED: .planning/phases/10-benchmarks-and-audit-readiness/10-CONTEXT.md]
   - Rationale: Benchmark registration, CLI flags, smoke/full mode selection, optional Knots inputs, and report schemas are Phase 10-specific concerns. Keeping them in a dedicated crate avoids turning `open-bitcoin-test-harness` into a mixed parity-plus-benchmark shell while still allowing executors to copy report-writing patterns from `packages/open-bitcoin-test-harness/src/report.rs`. [VERIFIED: packages/open-bitcoin-test-harness/src/report.rs] [ASSUMED]

2. **How much Knots execution should Phase 10 support?**  
   - Resolution: Support optional Knots JSON and optional Knots benchmark binary paths, but keep default verification mapping-only. [VERIFIED: packages/bitcoin-knots/doc/benchmarking.md] [VERIFIED: packages/bitcoin-knots/src/bench/bench_bitcoin.cpp]
   - Rationale: D-03 requires meaningful pinned-baseline comparison without making local verification depend on building or running the full Knots benchmark binary by default. Plans should record Knots benchmark/source mappings for every Open Bitcoin benchmark group, optionally ingest a user-provided `bench_bitcoin -output-json` file, and optionally record an explicit Knots binary path for manual report enrichment. [VERIFIED: .planning/phases/10-benchmarks-and-audit-readiness/10-CONTEXT.md]

3. **Should benchmark smoke execution be in `scripts/verify.sh` or a separate script?**  
   - Resolution: Add `scripts/run-benchmarks.sh` as the dedicated entrypoint and have `scripts/verify.sh` invoke `scripts/run-benchmarks.sh --smoke` only after the benchmark crate has bounded smoke iterations and report-schema validation. [VERIFIED: scripts/verify.sh pattern] [VERIFIED: .planning/phases/10-benchmarks-and-audit-readiness/10-CONTEXT.md]
   - Rationale: This preserves a reusable local benchmark command while keeping repo-native verification threshold-free and bounded per D-04. The script/verify wiring should live in a later plan than the crate and cases so executors can prove smoke behavior before it becomes part of `scripts/verify.sh`. [ASSUMED]

## Environment Availability

| Dependency | Required By | Available | Version | Fallback |
|---|---|---:|---|---|
| Rust / Cargo | First-party benchmark crate, tests, verification | yes | cargo 1.94.1; rustc 1.94.1 [VERIFIED: environment availability audit] | None needed. [VERIFIED: rust-toolchain.toml] |
| Bazel / Bazelisk command | Repo-native smoke build | yes | Bazel 8.6.0 [VERIFIED: environment availability audit] | None needed because `scripts/verify.sh` requires `bazel`. [VERIFIED: scripts/verify.sh] |
| Node | `scripts/verify.sh` elapsed-time helper and GSD tooling | yes | v24.13.0 [VERIFIED: environment availability audit] | None needed because `scripts/verify.sh` requires `node`. [VERIFIED: scripts/verify.sh] |
| cargo-llvm-cov | Existing pure-core coverage verification | yes | 0.8.5 [VERIFIED: environment availability audit] | None needed because `scripts/verify.sh` requires it. [VERIFIED: scripts/verify.sh] |
| CMake | Optional Knots `bench_bitcoin` build | yes | 3.27.9 [VERIFIED: environment availability audit] | Skip Knots execution and use mapping-only reports. [VERIFIED: .planning/phases/10-benchmarks-and-audit-readiness/10-CONTEXT.md] |
| Ninja | Optional CMake generator for Knots build | yes | 1.13.2 [VERIFIED: environment availability audit] | Use CMake's default generator or mapping-only reports. [VERIFIED: packages/bitcoin-knots/doc/benchmarking.md] [ASSUMED] |
| C++ compiler | Optional Knots `bench_bitcoin` build | yes | Apple clang 21.0.0 [VERIFIED: environment availability audit] | Skip Knots execution and use mapping-only reports. [VERIFIED: .planning/phases/10-benchmarks-and-audit-readiness/10-CONTEXT.md] |
| jq | Optional JSON inspection in development | yes | jq 1.7.1-apple [VERIFIED: environment availability audit] | Use Rust JSON parsing or plain file upload; do not require jq by default. [ASSUMED] |

**Missing dependencies with no fallback:** None found for the recommended default path. [VERIFIED: environment availability audit]

**Missing dependencies with fallback:** None found locally; optional Knots execution should still have a mapping-only fallback on machines without CMake/C++ tooling. [VERIFIED: environment availability audit] [VERIFIED: .planning/phases/10-benchmarks-and-audit-readiness/10-CONTEXT.md]

## Security Domain

Security enforcement is enabled because `.planning/config.json` does not set `security_enforcement` to `false`. [VERIFIED: .planning/config.json]

### Applicable ASVS Categories

OWASP ASVS latest stable version is 5.0.0, and OWASP recommends version-qualified requirement references such as `v5.0.0-1.2.5`. [CITED: https://owasp.org/www-project-application-security-verification-standard/] [CITED: https://github.com/OWASP/ASVS/releases]

| ASVS v5 Category | Applies | Standard Control |
|---|---:|---|
| V1 Encoding and Sanitization | yes | Use `serde_json` for JSON output and escape Markdown table cells when writing reviewer summaries. [VERIFIED: packages/open-bitcoin-test-harness/src/report.rs] [CITED: https://owasp.org/www-project-application-security-verification-standard/] |
| V2 Validation and Business Logic | yes | Validate report statuses against the locked taxonomy and reject unknown benchmark/checklist IDs in tooling. [VERIFIED: .planning/phases/10-benchmarks-and-audit-readiness/10-CONTEXT.md] [CITED: https://github.com/OWASP/ASVS/releases] |
| V4 API and Web Service | limited | Benchmark RPC dispatch only through supported local surfaces; do not add new public API behavior in this phase. [VERIFIED: docs/parity/catalog/rpc-cli-config.md] [VERIFIED: .planning/phases/10-benchmarks-and-audit-readiness/10-CONTEXT.md] |
| V6 Authentication | no new scope | Phase 10 does not add auth behavior; it may cite Phase 8 auth evidence only. [VERIFIED: .planning/phases/08-rpc-cli-and-config-parity/08-VERIFICATION.md] [VERIFIED: .planning/phases/10-benchmarks-and-audit-readiness/10-CONTEXT.md] |
| V7 Session Management | no | The headless local RPC/CLI benchmark and docs phase does not introduce sessions. [VERIFIED: docs/parity/catalog/rpc-cli-config.md] |
| V8 Authorization | no new scope | Do not add authorization behavior; document current deferred ACL surfaces where relevant. [VERIFIED: docs/parity/catalog/rpc-cli-config.md] |
| V11 Cryptography | yes | Do not add benchmark-only crypto paths or weaken signing/verification; benchmark existing first-party APIs. [VERIFIED: AGENTS.md] [VERIFIED: packages/open-bitcoin-consensus/Cargo.toml] [CITED: https://github.com/OWASP/ASVS/releases] |
| V13 Configuration | yes | Gate optional Knots execution behind explicit env vars or CLI args and avoid secrets in reports. [VERIFIED: packages/open-bitcoin-test-harness/src/target.rs] [VERIFIED: .planning/phases/09-parity-harnesses-and-fuzzing/09-VERIFICATION.md] [CITED: https://github.com/OWASP/ASVS/releases] |
| V15 Secure Coding and Architecture | yes | Keep benchmark/report shell code outside pure-core production crates and preserve `#![forbid(unsafe_code)]` patterns in first-party Rust crates. [VERIFIED: AGENTS.md] [VERIFIED: packages/open-bitcoin-consensus/src/lib.rs] [VERIFIED: packages/open-bitcoin-wallet/src/lib.rs] [CITED: https://github.com/OWASP/ASVS/releases] |
| V16 Security Logging and Error Handling | yes | Emit actionable benchmark/report errors without hiding skipped optional Knots comparisons. [VERIFIED: .planning/phases/09-parity-harnesses-and-fuzzing/09-VERIFICATION.md] [VERIFIED: .planning/phases/10-benchmarks-and-audit-readiness/10-CONTEXT.md] [CITED: https://github.com/OWASP/ASVS/releases] |

### Known Threat Patterns for Rust Benchmark and Audit Tooling

| Pattern | STRIDE | Standard Mitigation |
|---|---|---|
| JSON or Markdown report injection through benchmark IDs or details | Tampering | Treat benchmark IDs as static registry data, serialize JSON with `serde_json`, and escape Markdown table separators. [VERIFIED: packages/open-bitcoin-test-harness/src/report.rs] [ASSUMED] |
| OS command injection through optional Knots binary/filter arguments | Elevation of Privilege | Avoid shell-evaluated command strings; run external commands with argument arrays in Rust or carefully quoted Bash functions. [VERIFIED: AGENTS.md Bash guidance] [CITED: https://owasp.org/www-project-application-security-verification-standard/] |
| Secret leakage from Knots RPC or environment variables into readiness reports | Information Disclosure | Record presence/absence and skip reasons, not raw credentials; Phase 9 already uses explicit Knots env vars for optional targets. [VERIFIED: .planning/phases/09-parity-harnesses-and-fuzzing/09-VERIFICATION.md] |
| False readiness from stale checklist evidence | Spoofing | Require every `done` item to include concise evidence links and every `deferred`/`out_of_scope` item to include rationale. [VERIFIED: .planning/phases/10-benchmarks-and-audit-readiness/10-CONTEXT.md] |
| Denial of service through unbounded benchmark fixture sizes | Denial of Service | Provide `--smoke` defaults with bounded iterations and make full benchmark mode explicit. [VERIFIED: .planning/phases/10-benchmarks-and-audit-readiness/10-CONTEXT.md] [ASSUMED] |

## Sources

### Primary (HIGH confidence)

- `.planning/phases/10-benchmarks-and-audit-readiness/10-CONTEXT.md` - locked decisions, discretion, deferred scope, benchmark/audit policy. [VERIFIED: local file]
- `.planning/REQUIREMENTS.md` - `PAR-02` and `AUD-01` requirement text. [VERIFIED: local file]
- `.planning/PROJECT.md`, `.planning/STATE.md`, `.planning/ROADMAP.md` - project constraints, history, Phase 10 seeds, stale-state signals. [VERIFIED: local files]
- `AGENTS.md`, `AGENTS.bright-builds.md`, `standards-overrides.md` - repo verification, Bright Builds standards routing, local override state. [VERIFIED: local files]
- Bright Builds canonical standards: architecture, code-shape, verification, testing, and Rust pages fetched from `https://raw.githubusercontent.com/bright-builds-llc/bright-builds-rules/main/...`. [CITED: GitHub raw URLs]
- `packages/Cargo.toml`, `rust-toolchain.toml`, `MODULE.bazel`, `BUILD.bazel`, `scripts/verify.sh`, `.github/workflows/ci.yml` - workspace and verification integration. [VERIFIED: local files]
- `docs/parity/README.md`, `docs/parity/index.json`, `docs/parity/catalog/*.md` - parity catalog and current status source of truth. [VERIFIED: local files]
- `packages/open-bitcoin-test-harness/src/report.rs` - existing JSON/Markdown report writer pattern. [VERIFIED: local file]
- `packages/bitcoin-knots/doc/benchmarking.md`, `packages/bitcoin-knots/src/bench/bench.h`, `packages/bitcoin-knots/src/bench/bench_bitcoin.cpp`, and relevant `packages/bitcoin-knots/src/bench/*.cpp` files - upstream benchmark runner and benchmark mapping. [VERIFIED: local files]
- Rust standard documentation for `Instant` and `black_box`; Rust Unstable Book for `test` benchmark instability. [CITED: https://doc.rust-lang.org/std/time/struct.Instant.html] [CITED: https://doc.rust-lang.org/std/hint/fn.black_box.html] [CITED: https://doc.rust-lang.org/unstable-book/library-features/test.html]
- Crates.io registry and `cargo info` for `serde`, `serde_json`, `criterion`, and `divan` versions. [VERIFIED: cargo info] [VERIFIED: crates.io API]
- OWASP ASVS 5.0.0 project and release pages for current security category framing. [CITED: https://owasp.org/www-project-application-security-verification-standard/] [CITED: https://github.com/OWASP/ASVS/releases]

### Secondary (MEDIUM confidence)

- Criterion.rs book pages for benchmark behavior, warmup, sample collection, reports, and target output. [CITED: https://bheisler.github.io/criterion.rs/book/getting_started.html] [CITED: https://bheisler.github.io/criterion.rs/book/user_guide/command_line_output.html]

### Tertiary (LOW confidence)

- None used as authoritative sources. [VERIFIED: source review]

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH - the recommended default uses stable Rust std APIs, existing first-party crates, and existing JSON dependencies verified locally and through official/registry sources. [VERIFIED: rust-toolchain.toml] [CITED: https://doc.rust-lang.org/std/time/struct.Instant.html] [VERIFIED: cargo info serde_json]
- Architecture: HIGH - workspace/Bazel/report integration is directly visible in the repo and matches Phase 9 patterns. [VERIFIED: packages/Cargo.toml] [VERIFIED: BUILD.bazel] [VERIFIED: packages/open-bitcoin-test-harness/src/report.rs]
- Benchmark fixture details: MEDIUM - public API surfaces are verified, but exact fixture builders and smoke runtimes require implementation proof. [VERIFIED: rg public API scan] [ASSUMED]
- Pitfalls: HIGH - timing gate, setup loop, optimizer, and checklist drift risks are grounded in locked decisions, Rust docs, Knots benchmark code, and current planning/docs state. [VERIFIED: .planning/phases/10-benchmarks-and-audit-readiness/10-CONTEXT.md] [CITED: https://doc.rust-lang.org/std/hint/fn.black_box.html] [VERIFIED: packages/bitcoin-knots/src/bench/*.cpp] [VERIFIED: .planning/STATE.md]

**Research date:** 2026-04-24 [VERIFIED: environment current_date]  
**Valid until:** 2026-05-24 for repo architecture findings; 2026-05-01 for crates/security-version claims because benchmark and security ecosystems can move quickly. [ASSUMED]
