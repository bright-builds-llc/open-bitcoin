# Phase 52: Operator Evidence Cleanup - Research

**Researched:** 2026-06-01  
**Domain:** Rust operator evidence, daemon preflight truth, deterministic audit cleanup  
**Confidence:** HIGH

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions

### Support Bundle Live-Smoke Summary

- **D-01:** Treat schema v2 live-smoke reports as the primary input shape. The
  support bundle should summarize nested `result` fields for `status`,
  `progressDetected`, `maybeNoProgressCause`, `nextAction`, `headerDelta`, and
  `blockDelta` instead of reporting `summary_fields_unavailable`.
- **D-02:** Preserve the existing top-level allowlist as a compatibility
  fallback for older or hand-authored live-smoke report fixtures, but do not let
  top-level compatibility hide schema v2 nested `result` data.
- **D-03:** Keep support ingestion summary-only and redacted. Do not embed raw
  live-smoke input, raw daemon output tails, raw status snapshots, or raw
  options. Recursively sanitize any summarized strings.
- **D-04:** Markdown output should present the same allowlisted summary values
  reviewers need for audit: status, progress detection, typed no-progress cause,
  next action, header delta, and block delta.

### Deterministic Evidence Tests

- **D-05:** Add or update deterministic support-bundle tests using a schema v2
  fixture with nested `result` fields and raw secret-like report data. The tests
  must prove the nested summary is present and raw live-smoke input remains
  absent from JSON and Markdown.
- **D-06:** Keep missing-artifact and redaction behavior covered. A missing
  live-smoke report should stay a non-fatal unavailable evidence state with a
  reason.

### Daemon Preflight Truth

- **D-07:** Refresh `open-bitcoind` preflight wording to state that preflight
  opened the durable store and that the daemon will start the explicit opt-in,
  bounded mainnet sync worker when enabled.
- **D-08:** The wording must still preserve the non-claim: this is not
  unattended production-node operation or a packaged service guarantee.
- **D-09:** Add a deterministic unit assertion for the rendered preflight
  message so stale wording cannot regress silently.

### Docs And Audit References

- **D-10:** Update operator docs and v1.3 audit references only where readers
  currently have to reconcile stale support-summary or preflight wording debt.
- **D-11:** Mark the Phase 52 cleanup as deterministic debt closure after code,
  docs, and repo-native verification pass. Do not refresh live public-network
  evidence; that remains Phase 53.

### the agent's Discretion

The planner may choose exact helper names, summary key names, and Markdown
formatting, provided the JSON shape remains stable, tests cover the schema v2
nested `result` path, and the implementation stays within the existing
support/preflight modules without adding new dependencies.

### Deferred Ideas (OUT OF SCOPE)

- Phase 53 owns live public-network evidence refresh and historical Phase 50
  caveat retirement. Phase 52 should not rerun or replace those live artifacts.
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| OBS-02 | Operator-facing status, dashboard, metrics, structured logs, and RPC-facing blockchain info stay consistent and never imply full sync before validated chainstate reaches the selected tip. [VERIFIED: .planning/REQUIREMENTS.md:35-39] | Refresh `open-bitcoind` preflight wording in `packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs` so it matches the current opt-in worker while preserving production-node non-claims. [VERIFIED: packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs:44-47,130-170; .planning/v1.3-MILESTONE-AUDIT.md:159-168] |
| OBS-03 | Operator can generate a redacted support evidence bundle containing config sources, versions, sync status, peer outcomes, logs, metrics, store health, and live smoke artifacts. [VERIFIED: .planning/REQUIREMENTS.md:35-39] | Extend existing support bundle live-smoke ingestion in `packages/open-bitcoin-cli/src/operator/support.rs` to summarize schema v2 nested `result` fields while keeping summary-only redaction behavior. [VERIFIED: packages/open-bitcoin-cli/src/operator/support.rs:442-537; scripts/run-live-mainnet-smoke.ts:112-157] |
</phase_requirements>

## Summary

Phase 52 is a deterministic cleanup phase, not a public-network evidence refresh. The two code targets are narrowly scoped: `packages/open-bitcoin-cli/src/operator/support.rs` / `support/render.rs` for schema v2 live-smoke summaries, and `packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs` for daemon sync preflight wording. [VERIFIED: .planning/phases/52-operator-evidence-cleanup/52-CONTEXT.md; packages/open-bitcoin-cli/src/operator/support.rs:442-537; packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs:96-170]

The current support bundle reads an optional report path, returns `unavailable` with a reason for missing/unreadable/invalid reports, and sanitizes summarized JSON strings recursively. [VERIFIED: packages/open-bitcoin-cli/src/operator/support.rs:442-537] Its current summary helper only scans top-level keys and returns `{"status":"summary_fields_unavailable"}` when the top-level allowlist is absent, which is why schema v2 reports with nested `result` fields trigger audit debt D-02. [VERIFIED: packages/open-bitcoin-cli/src/operator/support.rs:34-56,493-508; .planning/v1.3-MILESTONE-AUDIT.md:137-148]

The current `open-bitcoind` startup path runs `preflight_daemon_sync`, reports a preflight line, then starts `start_daemon_sync_worker` when sync is enabled. [VERIFIED: packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs:44-47] The existing preflight line still says peer transport and unattended full IBD are not started by this phase, which conflicts with the current opt-in worker and is tracked as audit debt D-04. [VERIFIED: packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs:130-138; .planning/v1.3-MILESTONE-AUDIT.md:159-168]

**Primary recommendation:** Plan one focused Rust change that adds a nested-result allowlist helper, updates Markdown field rendering, adds support-bundle and preflight message assertions, and amends only the stale operator/audit references. [VERIFIED: .planning/phases/52-operator-evidence-cleanup/52-CONTEXT.md; scripts/run-live-mainnet-smoke.ts:146-155; packages/open-bitcoin-cli/tests/operator_binary.rs:716-824; packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs:280-352]

## Project Constraints (from AGENTS.md)

- Use `AGENTS.md` as the repo-local entrypoint, then read `AGENTS.bright-builds.md`, `standards-overrides.md`, and relevant canonical Bright Builds standards pages before planning or implementation. [VERIFIED: AGENTS.md; AGENTS.bright-builds.md; standards-overrides.md; CITED: https://raw.githubusercontent.com/bright-builds-llc/bright-builds-rules/05f8d7a6c9c2e157ec4f922a05273e72dab97676/standards/index.md]
- Use `rust-toolchain.toml` as the Rust source of truth; the pinned toolchain is Rust `1.94.1`. [VERIFIED: AGENTS.md; rust-toolchain.toml]
- Use `bash scripts/verify.sh` as the repo-native verification contract for first-party code. [VERIFIED: AGENTS.md; scripts/verify.sh:96-147]
- Do not add public-network checks to default verification; live-mainnet evidence remains opt-in and outside `bash scripts/verify.sh`. [VERIFIED: .planning/REQUIREMENTS.md:66-83; scripts/check-v1.3-release-boundaries.ts:166-171]
- Use Bun for repo-owned TypeScript automation and keep Bash for thin wrappers. [VERIFIED: AGENTS.md; .bun-version; scripts/run-live-mainnet-smoke.ts:1; scripts/test-run-live-mainnet-smoke.sh:1-2]
- Use repo-local Cargo and Bazel commands in operator-facing docs and UAT. [VERIFIED: AGENTS.md; docs/operator/runtime-guide.md:491-509]
- Treat `docs/metrics/lines-of-code.md` as a tracked generated artifact that may need freshness updates after verification. [VERIFIED: AGENTS.md; scripts/verify.sh:114]
- Add parity breadcrumbs for new first-party Rust source/test files under `packages/open-bitcoin-*`; use `none` when no Knots source anchor exists. [VERIFIED: AGENTS.md; packages/open-bitcoin-cli/src/operator/support.rs:1-4; packages/open-bitcoin-cli/src/operator/support/render.rs:1-4]
- Keep functional core / imperative shell boundaries and prefer pure helpers for decision logic. [VERIFIED: AGENTS.md; CITED: https://raw.githubusercontent.com/bright-builds-llc/bright-builds-rules/05f8d7a6c9c2e157ec4f922a05273e72dab97676/standards/core/architecture.md]
- Prefer early returns, `let...else` guard extraction, `maybe_` names for optional internals, focused tests, and Arrange/Act/Assert comments. [VERIFIED: AGENTS.md; CITED: https://raw.githubusercontent.com/bright-builds-llc/bright-builds-rules/05f8d7a6c9c2e157ec4f922a05273e72dab97676/standards/core/code-shape.md; CITED: https://raw.githubusercontent.com/bright-builds-llc/bright-builds-rules/05f8d7a6c9c2e157ec4f922a05273e72dab97676/standards/core/testing.md; CITED: https://raw.githubusercontent.com/bright-builds-llc/bright-builds-rules/05f8d7a6c9c2e157ec4f922a05273e72dab97676/standards/languages/rust.md]
- No project-local skills were found under `.claude/skills` or `.agents/skills`. [VERIFIED: filesystem find]

## Standard Stack

### Core

| Library / Tool | Version | Purpose | Why Standard |
|----------------|---------|---------|--------------|
| Rust | `1.94.1` | Compile and test first-party Rust crates. | Pinned in `rust-toolchain.toml` and required by repo guidance. [VERIFIED: rust-toolchain.toml; AGENTS.md] |
| Rust 2024 edition | `2024` | Workspace language edition. | Set in `packages/Cargo.toml` workspace metadata. [VERIFIED: packages/Cargo.toml] |
| `serde` | `1.0.228` | Serialize support evidence models. | Already used by `SupportEvidenceBundle` and workspace crates; no new dependency needed. [VERIFIED: packages/open-bitcoin-cli/Cargo.toml; packages/open-bitcoin-cli/src/operator/support.rs:18,148-280] |
| `serde_json` | `1.0.149` | Parse live-smoke reports as `Value`, build allowlisted JSON summaries, and write support JSON. | Existing support ingestion already uses `serde_json::{Map, Value, json}` and should be extended instead of replaced. [VERIFIED: packages/open-bitcoin-cli/Cargo.toml; packages/open-bitcoin-cli/src/operator/support.rs:18-19,474-508] |
| `open-bitcoin-cli` support module | workspace crate `0.1.0` | Owns `open-bitcoin support bundle`, evidence model, redaction, live-smoke ingestion, and Markdown rendering. | Existing Phase 48 implementation surface for OBS-03 support evidence. [VERIFIED: packages/Cargo.toml; packages/open-bitcoin-cli/src/operator.rs:186-203; .planning/phases/48-support-evidence-and-operator-runbooks/48-SUMMARY.md] |
| `open-bitcoin-rpc` `open-bitcoind` binary | workspace crate `0.1.0` | Owns daemon sync preflight and opt-in sync worker startup wording. | Existing runtime surface for D-04 and OBS-02 preflight truth. [VERIFIED: packages/Cargo.toml; packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs:38-170] |

### Supporting

| Tool / File | Version | Purpose | When to Use |
|-------------|---------|---------|-------------|
| Bun | `1.3.9` | Run repo-owned TypeScript validation and deterministic live-smoke regression scripts. | Use for `scripts/test-run-live-mainnet-smoke.sh` and `scripts/check-v1.3-release-boundaries.ts` verification; do not add TS dependencies. [VERIFIED: .bun-version; bun --version; scripts/verify.sh:114-116] |
| Bazel/Bazelisk | Bazelisk `1.28.1`, Bazel `8.6.0` | Repo smoke build and operator CLI Bazel run examples. | Use through `bash scripts/verify.sh` and repo-local operator docs. [VERIFIED: bazel version; scripts/verify.sh:127-128; docs/operator/runtime-guide.md:496-499] |
| `cargo-llvm-cov` | `0.8.5` | Coverage gate used by `bash scripts/verify.sh`. | Required for full repo-native verification before closeout. [VERIFIED: cargo llvm-cov --version; scripts/verify.sh:96-97,140-147] |

### Alternatives Considered

| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| Extend `serde_json::Value` allowlist extraction. [VERIFIED: packages/open-bitcoin-cli/src/operator/support.rs:493-508] | Add a strict Rust `SmokeReport` DTO mirroring the TypeScript schema. | Strict DTOs reject older or hand-authored fallback fixtures unless compatibility code is added; the phase explicitly requires a top-level compatibility fallback. [VERIFIED: .planning/phases/52-operator-evidence-cleanup/52-CONTEXT.md] |
| Small nested `result` helper. [VERIFIED: .planning/phases/52-operator-evidence-cleanup/52-CONTEXT.md] | Broad recursive flattening of the report. | Flattening makes it easier to accidentally summarize raw options, snapshots, or daemon output tails, which violates the summary-only redaction boundary. [VERIFIED: scripts/run-live-mainnet-smoke.ts:119-156; .planning/phases/52-operator-evidence-cleanup/52-CONTEXT.md] |
| `daemon_sync_preflight_message` helper returning a string. [VERIFIED: .planning/phases/52-operator-evidence-cleanup/52-CONTEXT.md] | Assert only on `eprintln!` side effects in a process-level test. | A helper keeps the wording deterministic and cheap to test inside the existing `open-bitcoind.rs` unit module. [VERIFIED: packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs:280-352] |

**Installation:** No package installation is recommended for this phase. [VERIFIED: .planning/phases/52-operator-evidence-cleanup/52-CONTEXT.md; packages/open-bitcoin-cli/Cargo.toml; packages/open-bitcoin-rpc/Cargo.toml]

**Version verification:** Recommended stack versions were verified from repo manifests and local CLI probes rather than upstream registries because this phase should not add or update dependencies. [VERIFIED: rust-toolchain.toml; packages/open-bitcoin-cli/Cargo.toml; packages/open-bitcoin-rpc/Cargo.toml; cargo --version; bun --version; bazel version]

## Architecture Patterns

### Recommended Project Structure

```text
packages/open-bitcoin-cli/src/operator/
├── support.rs          # evidence model, live-smoke ingestion, redaction helpers [VERIFIED: packages/open-bitcoin-cli/src/operator/support.rs]
└── support/render.rs   # Markdown and command-output rendering [VERIFIED: packages/open-bitcoin-cli/src/operator/support/render.rs]

packages/open-bitcoin-cli/tests/
└── operator_binary.rs  # support bundle integration tests [VERIFIED: packages/open-bitcoin-cli/tests/operator_binary.rs]

packages/open-bitcoin-rpc/src/bin/
└── open-bitcoind.rs    # preflight, worker startup, preflight unit tests [VERIFIED: packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs]

docs/
├── operator/runtime-guide.md        # operator support/live-smoke wording [VERIFIED: docs/operator/runtime-guide.md]
└── parity/                         # release-readiness, checklist, index, threat-model references [VERIFIED: docs/parity]
```

### Pattern 1: Prefer Nested `result` First, Then Top-Level Fallback

**What:** `live_smoke_summary` should first check `value["result"]` for schema v2 fields and only fall back to the existing top-level allowlist when nested `result` fields are unavailable. [VERIFIED: scripts/run-live-mainnet-smoke.ts:146-155; packages/open-bitcoin-cli/src/operator/support.rs:493-508]

**When to use:** Use this for any support bundle that receives `--include-live-smoke-report` with a schema v2 report from `scripts/run-live-mainnet-smoke.ts`. [VERIFIED: docs/operator/runtime-guide.md:501-510; scripts/run-live-mainnet-smoke.ts:1693-1703]

**Example:**

```rust
// Source: repo pattern in packages/open-bitcoin-cli/src/operator/support.rs.
// Planner should preserve the existing top-level fallback and sanitize every value.
fn live_smoke_summary(value: &Value) -> Option<Value> {
    let object = value.as_object()?;
    if let Some(summary) = live_smoke_summary_from_result(object.get("result")) {
        return Some(summary);
    }

    live_smoke_summary_from_top_level(object)
}
```

### Pattern 2: Keep Summary Values Allowlisted and Sanitized

**What:** Summary extraction should copy only `status`, `progressDetected`, `maybeNoProgressCause`, `nextAction`, `headerDelta`, and `blockDelta` from nested `result`, and each copied value should pass through `sanitize_json_value`. [VERIFIED: .planning/phases/52-operator-evidence-cleanup/52-CONTEXT.md; packages/open-bitcoin-cli/src/operator/support.rs:510-537]

**When to use:** Use this for both JSON output and Markdown output; do not expose `daemon.stderrTail`, `daemon.stdoutTail`, `options`, `snapshots`, `network_preflight`, or `preflight.checks` through support bundle summaries. [VERIFIED: scripts/run-live-mainnet-smoke.ts:119-156; .planning/phases/52-operator-evidence-cleanup/52-CONTEXT.md]

**Example:**

```rust
// Source: schema fields from scripts/run-live-mainnet-smoke.ts and sanitizer from support.rs.
const LIVE_SMOKE_RESULT_SUMMARY_KEYS: &[&str] = &[
    "status",
    "progressDetected",
    "maybeNoProgressCause",
    "nextAction",
    "headerDelta",
    "blockDelta",
];
```

### Pattern 3: Render Markdown as Named Audit Fields

**What:** `render_support_markdown` should render the same allowlisted summary values reviewers need as explicit Live Smoke lines instead of one opaque JSON object. [VERIFIED: packages/open-bitcoin-cli/src/operator/support/render.rs:112-125; .planning/phases/52-operator-evidence-cleanup/52-CONTEXT.md]

**When to use:** Use this when `bundle.live_smoke.summary` is present; missing summary and unavailable reason behavior should stay visible. [VERIFIED: packages/open-bitcoin-cli/src/operator/support/render.rs:112-125]

**Example:**

```rust
// Source: existing Live Smoke markdown section in support/render.rs.
// Recommended output labels: Status, Progress detected, No-progress cause,
// Next action, Header delta, Block delta.
```

### Pattern 4: Make Preflight Wording a Pure Helper

**What:** Add a helper such as `daemon_sync_preflight_message(&DaemonSyncPreflight) -> String`, call it from `report_daemon_sync_preflight`, and assert the exact string in the existing test module. [VERIFIED: packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs:130-138,280-352; .planning/phases/52-operator-evidence-cleanup/52-CONTEXT.md]

**When to use:** Use this for D-04 so future wording drift fails a deterministic unit test without needing to launch a daemon process. [VERIFIED: packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs:280-352]

**Example:**

```rust
// Source: current preflight data model in open-bitcoind.rs.
fn daemon_sync_preflight_message(preflight: &DaemonSyncPreflight) -> String {
    format!(
        "open-bitcoind mainnet sync preflight opened durable store: mode={}, datadir=\"{}\", best_header_height={}, best_block_height={}; enabled startup will run the explicit opt-in bounded mainnet sync worker. This is not an unattended production-node or packaged-service guarantee.",
        preflight.mode,
        preflight.data_dir.display(),
        preflight.best_header_height,
        preflight.best_block_height
    )
}
```

### Anti-Patterns to Avoid

- **Schema v2 hidden by fallback:** Do not run the old top-level allowlist before checking nested `result`, because schema v2 reports can otherwise keep producing `summary_fields_unavailable`. [VERIFIED: packages/open-bitcoin-cli/src/operator/support.rs:493-508; .planning/v1.3-MILESTONE-AUDIT.md:137-148]
- **Raw report embedding:** Do not include report input, daemon output tails, snapshots, raw options, or endpoint tables in the support bundle summary. [VERIFIED: scripts/run-live-mainnet-smoke.ts:119-156; .planning/phases/52-operator-evidence-cleanup/52-CONTEXT.md]
- **New public-network gate:** Do not add `run-live-mainnet-smoke` to `bash scripts/verify.sh`; `scripts/check-v1.3-release-boundaries.ts` currently asserts the verify script does not contain that command. [VERIFIED: scripts/verify.sh:114-128; scripts/check-v1.3-release-boundaries.ts:166-171]
- **Historical evidence rewrite:** Do not silently alter historical generated-output quotes in Phase 50 UAT; add a Phase 52 amendment or note where needed so the historical artifact stays auditable while readers see the debt closure. [VERIFIED: .planning/phases/50-public-mainnet-progress-evidence-closeout/50-UAT.md:103-107,231-235; .planning/phases/52-operator-evidence-cleanup/52-CONTEXT.md]

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Live-smoke JSON parsing | String matching, regex extraction, or path splitting. | `serde_json::Value` and `serde_json::Map` already used in `support.rs`. [VERIFIED: packages/open-bitcoin-cli/src/operator/support.rs:18-19,474-508] | JSON report fields include nested objects, strings needing redaction, numbers, bools, and nullable values. [VERIFIED: scripts/run-live-mainnet-smoke.ts:112-157] |
| Redaction | New ad hoc secret filters in Markdown renderer. | Reuse `sanitize_json_value` and `redact_sensitive_text` before values reach rendering. [VERIFIED: packages/open-bitcoin-cli/src/operator/support.rs:510-537] | Current sanitizer recursively handles strings inside arrays/objects and redacts known credential/private-material markers. [VERIFIED: packages/open-bitcoin-cli/src/operator/support.rs:510-537] |
| Live evidence validation | New release validator or generated live report ingestion in `verify.sh`. | Existing deterministic tests plus `scripts/check-v1.3-release-boundaries.ts`. [VERIFIED: scripts/verify.sh:114-128; scripts/check-v1.3-release-boundaries.ts:113-171] | Phase 52 must not change the public-network claim boundary or refresh live artifacts. [VERIFIED: .planning/phases/52-operator-evidence-cleanup/52-CONTEXT.md] |
| Preflight test harness | Process-level daemon launch just to capture stderr. | Pure preflight message helper in `open-bitcoind.rs` unit tests. [VERIFIED: packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs:280-352] | Existing tests already construct `RuntimeConfig` and temporary stores for preflight behavior. [VERIFIED: packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs:302-352] |

**Key insight:** The planner should close the gap by refining existing bounded evidence transforms, not by adding a schema engine, new dependencies, a release validator, or live network execution. [VERIFIED: .planning/phases/52-operator-evidence-cleanup/52-CONTEXT.md; packages/open-bitcoin-cli/Cargo.toml; packages/open-bitcoin-rpc/Cargo.toml]

## Common Pitfalls

### Pitfall 1: Summary Fallback Masks Schema v2

**What goes wrong:** The support bundle keeps returning `summary_fields_unavailable` for schema v2 reports. [VERIFIED: .planning/v1.3-MILESTONE-AUDIT.md:137-148]  
**Why it happens:** The current helper scans only top-level keys and schema v2 stores audit-critical values under `result`. [VERIFIED: packages/open-bitcoin-cli/src/operator/support.rs:493-508; scripts/run-live-mainnet-smoke.ts:146-155]  
**How to avoid:** Check nested `result` first, then run the existing top-level fallback for older fixtures. [VERIFIED: .planning/phases/52-operator-evidence-cleanup/52-CONTEXT.md]  
**Warning signs:** Tests only assert top-level `status` / `maybeNoProgressCause` and do not include `schema_version: 2` with nested `result`. [VERIFIED: packages/open-bitcoin-cli/tests/operator_binary.rs:772-824]

### Pitfall 2: Redaction Regression Through Markdown

**What goes wrong:** JSON output is sanitized but Markdown prints a raw or differently formatted value that leaks secret-like report data. [VERIFIED: packages/open-bitcoin-cli/src/operator/support/render.rs:112-125; packages/open-bitcoin-cli/tests/operator_binary.rs:821-823]  
**Why it happens:** Rendering can bypass extraction helpers if it reads the raw report or manually formats unsanitized strings. [VERIFIED: packages/open-bitcoin-cli/src/operator/support.rs:442-537]  
**How to avoid:** Ensure Markdown consumes only `bundle.live_smoke.summary` values already produced by the sanitized support ingestion path. [VERIFIED: packages/open-bitcoin-cli/src/operator/support.rs:485-489,510-537; packages/open-bitcoin-cli/src/operator/support/render.rs:112-125]  
**Warning signs:** Test fixtures include secret-like strings only under top-level ignored fields, not inside nested `result.nextAction` or another summarized string. [VERIFIED: packages/open-bitcoin-cli/tests/operator_binary.rs:779-787]

### Pitfall 3: Missing Report Becomes Fatal

**What goes wrong:** A missing `--include-live-smoke-report` path fails the whole support bundle command. [VERIFIED: packages/open-bitcoin-cli/src/operator/support.rs:451-460]  
**Why it happens:** It is easy to convert optional evidence loading into a hard parse path while refactoring summary extraction. [VERIFIED: packages/open-bitcoin-cli/src/operator/support.rs:442-489]  
**How to avoid:** Preserve `LiveSmokeEvidence { state: Unavailable, reason: Some(...) }` for missing/unreadable/invalid report states. [VERIFIED: packages/open-bitcoin-cli/src/operator/support.rs:442-489]  
**Warning signs:** There is no integration test that passes a missing live-smoke report path and asserts non-fatal `unavailable` evidence with a reason. [VERIFIED: rg over packages/open-bitcoin-cli/tests/operator_binary.rs]

### Pitfall 4: Preflight Wording Overclaims Production Readiness

**What goes wrong:** The new message fixes the stale peer-transport phrase but implies unattended production-node operation or packaged service guarantees. [VERIFIED: .planning/phases/52-operator-evidence-cleanup/52-CONTEXT.md; docs/parity/release-readiness.md:180-190]  
**Why it happens:** The current worker exists, but v1.3 still keeps production-node operation, packaging, and service guarantees out of scope. [VERIFIED: docs/operator/runtime-guide.md:433-436; docs/parity/release-readiness.md:180-190]  
**How to avoid:** State both truths in one line: preflight opened durable store, enabled startup will run the explicit opt-in bounded mainnet sync worker, and this is not unattended production-node or packaged-service readiness. [VERIFIED: packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs:108-127,140-170; .planning/phases/52-operator-evidence-cleanup/52-CONTEXT.md]  
**Warning signs:** The message contains only "worker will start" without an explicit non-claim, or still contains "peer transport ... not started by this phase." [VERIFIED: packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs:130-138; .planning/v1.3-MILESTONE-AUDIT.md:159-168]

### Pitfall 5: Docs Cleanup Rewrites More Than the Debt

**What goes wrong:** Phase 52 becomes a broad release-doc rewrite or updates parity roots unrelated to D-02/D-04. [VERIFIED: .planning/phases/52-operator-evidence-cleanup/52-CONTEXT.md]  
**Why it happens:** The stale phrases appear near broader v1.3 release and evidence language. [VERIFIED: docs/parity/release-readiness.md:80-145; .planning/phases/50-public-mainnet-progress-evidence-closeout/50-UAT.md:103-107,231-235]  
**How to avoid:** Target only `docs/operator/runtime-guide.md`, `.planning/v1.3-MILESTONE-AUDIT.md`, `.planning/phases/50-public-mainnet-progress-evidence-closeout/50-UAT.md`, and parity roots if their current language forces reconciliation. [VERIFIED: .planning/phases/52-operator-evidence-cleanup/52-CONTEXT.md; rg results]  
**Warning signs:** The diff changes future-scope lists, claim matrices, or Phase 53 live-evidence language unrelated to the support-summary/preflight debt. [VERIFIED: docs/parity/release-readiness.md:172-190; .planning/ROADMAP.md:175-178]

## Exact Planning Targets

| Target | Current State | Plan Action |
|--------|---------------|-------------|
| `packages/open-bitcoin-cli/src/operator/support.rs` | `collect_live_smoke_evidence` handles optional report states and calls `live_smoke_summary`; `live_smoke_summary` scans only top-level keys. [VERIFIED: packages/open-bitcoin-cli/src/operator/support.rs:442-508] | Add nested-result extraction before top-level fallback; keep recursive sanitization and unavailable states. [VERIFIED: .planning/phases/52-operator-evidence-cleanup/52-CONTEXT.md] |
| `packages/open-bitcoin-cli/src/operator/support/render.rs` | Live Smoke Markdown prints `- Summary: {json}` when a summary exists. [VERIFIED: packages/open-bitcoin-cli/src/operator/support/render.rs:112-125] | Render named allowlisted fields for schema v2 summary values and keep reason/report path lines. [VERIFIED: .planning/phases/52-operator-evidence-cleanup/52-CONTEXT.md] |
| `packages/open-bitcoin-cli/tests/operator_binary.rs` | Existing support bundle tests cover redaction and a top-level live-smoke summary. [VERIFIED: packages/open-bitcoin-cli/tests/operator_binary.rs:716-824] | Replace or extend the live-smoke fixture with schema v2 nested `result`, assert all six fields in JSON/Markdown, assert raw secret-like report data absent, and add/keep missing-report unavailable behavior. [VERIFIED: .planning/phases/52-operator-evidence-cleanup/52-CONTEXT.md] |
| `packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs` | `report_daemon_sync_preflight` directly formats stale stderr; tests cover disabled/enabled/missing-datadir preflight but not message rendering. [VERIFIED: packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs:130-138,302-352] | Add pure message helper and unit assertion; update wording to describe durable-store preflight plus opt-in bounded worker and non-claims. [VERIFIED: .planning/phases/52-operator-evidence-cleanup/52-CONTEXT.md] |
| `docs/operator/runtime-guide.md` | Support bundle docs say allowlisted live-smoke fields include status, typed no-progress cause, next action, manual peers, timing, and report paths. [VERIFIED: docs/operator/runtime-guide.md:515-530] | Update support bundle wording to match schema v2 result summary and keep raw-report/redaction boundaries. [VERIFIED: .planning/phases/52-operator-evidence-cleanup/52-CONTEXT.md] |
| `.planning/v1.3-MILESTONE-AUDIT.md` | D-02 and D-04 are open tech-debt items. [VERIFIED: .planning/v1.3-MILESTONE-AUDIT.md:23-35,137-168] | Mark Phase 52 closure after code/docs/verification; do not close D-01/D-03 or Phase 53 live evidence. [VERIFIED: .planning/REQUIREMENTS.md:118-129; .planning/ROADMAP.md:175-178] |
| `.planning/phases/50-public-mainnet-progress-evidence-closeout/50-UAT.md` | Historical UAT includes stale preflight stderr and a note that support summary was `summary_fields_unavailable`. [VERIFIED: .planning/phases/50-public-mainnet-progress-evidence-closeout/50-UAT.md:103-107,231-235] | Add an amendment/note pointing to Phase 52 deterministic cleanup; preserve the historical quote unless explicitly choosing to rewrite historical evidence. [VERIFIED: .planning/phases/52-operator-evidence-cleanup/52-CONTEXT.md] |
| `docs/parity/release-readiness.md`, `docs/parity/checklist.md`, `docs/parity/index.json` | Parity roots reference support evidence and Phase 51 fresh-status closeout. [VERIFIED: docs/parity/release-readiness.md:80-145,165-170; docs/parity/checklist.md:30-31; docs/parity/index.json:897-919] | Update only if needed to route readers to Phase 52 closure or remove stale reconciliation burden; keep release-boundary validator strings intact. [VERIFIED: scripts/check-v1.3-release-boundaries.ts:113-171] |
| `.planning/REQUIREMENTS.md`, `.planning/ROADMAP.md`, `.planning/STATE.md` | Tech-debt table shows D-02 and D-04 pending; roadmap shows Phase 52 pending; state stopped at Phase 52 context. [VERIFIED: .planning/REQUIREMENTS.md:118-129; .planning/ROADMAP.md:163-173,220-227; .planning/STATE.md] | Close Phase 52 bookkeeping only after implementation and verification. [VERIFIED: .planning/phases/52-operator-evidence-cleanup/52-CONTEXT.md] |

## Code Examples

### Schema v2 Live-Smoke Fixture Shape

```rust
// Source: scripts/run-live-mainnet-smoke.ts:146-155 and support test style in operator_binary.rs.
let live_smoke_fixture = json!({
    "schema_version": 2,
    "result": {
        "status": "no_progress",
        "progressDetected": false,
        "maybeNoProgressCause": "handshake_failure",
        "nextAction": "Retry after rotating manual peers; rpcpassword=fixture-secret",
        "headerDelta": 0,
        "blockDelta": 0,
        "message": "raw result message should not be copied unless allowlisted"
    },
    "daemon": {
        "stderrTail": "live-smoke-secret"
    },
    "options": {
        "manualPeers": ["192.0.2.10:8333"],
        "rpcpassword": "live-smoke-secret"
    },
    "snapshots": [
        { "phase": "header_sync", "secret": "live-smoke-secret" }
    ]
});
```

### Missing Live-Smoke Report Assertion

```rust
// Source: current non-fatal unavailable behavior in support.rs:451-460.
assert_eq!(decoded["live_smoke"]["state"], "unavailable");
assert!(
    decoded["live_smoke"]["reason"]
        .as_str()
        .expect("reason")
        .contains("does not exist")
);
```

### Preflight Message Assertion

```rust
// Source: current open-bitcoind.rs test module and proposed helper.
#[test]
fn enabled_sync_preflight_message_describes_opt_in_worker_without_production_claim() {
    // Arrange
    let preflight = DaemonSyncPreflight {
        mode: DaemonSyncConfig::mainnet_ibd().mode,
        data_dir: PathBuf::from("/tmp/open-bitcoin-mainnet"),
        best_header_height: 12,
        best_block_height: 3,
    };

    // Act
    let message = daemon_sync_preflight_message(&preflight);

    // Assert
    assert!(message.contains("opened durable store"));
    assert!(message.contains("explicit opt-in bounded mainnet sync worker"));
    assert!(message.contains("not an unattended production-node"));
    assert!(message.contains("not a packaged-service guarantee"));
    assert!(!message.contains("peer transport and unattended full IBD are not started"));
}
```

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| Support bundle summarized a shallow top-level live-smoke allowlist. [VERIFIED: packages/open-bitcoin-cli/src/operator/support.rs:34-56,493-508] | Schema v2 reports store audit-critical fields under nested `result`, and support should summarize those fields first. [VERIFIED: scripts/run-live-mainnet-smoke.ts:146-155; .planning/phases/52-operator-evidence-cleanup/52-CONTEXT.md] | Schema v2 is present in the current live-smoke runner. [VERIFIED: scripts/run-live-mainnet-smoke.ts:155,1702] | D-02 closes when support summaries expose result status, progress detection, typed cause, next action, and deltas. [VERIFIED: .planning/v1.3-MILESTONE-AUDIT.md:137-148] |
| Phase 35 preflight wording described durable preflight without starting peer transport. [VERIFIED: .planning/milestones/v1.2-phases/35-daemon-mainnet-sync-activation/35-02-SUMMARY.md; packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs:130-138] | Current `open-bitcoind` starts an opt-in daemon sync worker after preflight when sync is enabled. [VERIFIED: packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs:44-47,140-170] | Worker startup exists before Phase 52 and is the basis for D-04. [VERIFIED: .planning/v1.3-MILESTONE-AUDIT.md:159-168] | D-04 closes when wording describes both durable preflight and enabled worker startup without production-node claims. [VERIFIED: .planning/phases/52-operator-evidence-cleanup/52-CONTEXT.md] |
| Phase 50 UAT used generated live and support artifacts as historical evidence. [VERIFIED: .planning/phases/50-public-mainnet-progress-evidence-closeout/50-UAT.md:80-107,213-235] | Phase 51 amended the evidence trail for fresh status without checking in generated live reports. [VERIFIED: .planning/phases/51-live-smoke-fresh-status-integration/51-01-SUMMARY.md] | 2026-05-31 Phase 51. [VERIFIED: .planning/phases/51-live-smoke-fresh-status-integration/51-01-SUMMARY.md] | Phase 52 should follow the same pattern for deterministic debt closure and avoid replacing live artifacts. [VERIFIED: .planning/phases/52-operator-evidence-cleanup/52-CONTEXT.md] |

**Deprecated/outdated:**

- The phrase `peer transport and unattended full IBD are not started by this phase` is outdated for current `open-bitcoind` startup because the binary now calls `start_daemon_sync_worker` after preflight. [VERIFIED: packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs:44-47,130-170; .planning/v1.3-MILESTONE-AUDIT.md:159-168]
- A support summary status of `summary_fields_unavailable` is outdated for schema v2 reports that contain nested `result` fields. [VERIFIED: packages/open-bitcoin-cli/src/operator/support.rs:493-508; scripts/run-live-mainnet-smoke.ts:146-155; .planning/v1.3-MILESTONE-AUDIT.md:137-148]

## Assumptions Log

| # | Claim | Section | Risk if Wrong |
|---|-------|---------|---------------|
| - | No `[ASSUMED]` claims are intentionally used; recommendations are grounded in repo files, local command probes, or cited Bright Builds / OWASP sources. | All | Planner does not need user confirmation for assumed facts before execution. |

## Open Questions (RESOLVED)

1. **Should historical Phase 50 UAT raw stderr be amended in place or left as a quoted historical artifact with a Phase 52 note?**
   - What we know: The UAT contains the exact stale preflight line and the support summary caveat. [VERIFIED: .planning/phases/50-public-mainnet-progress-evidence-closeout/50-UAT.md:103-107,231-235]
   - What's unclear: The phase context requires reducing reader reconciliation burden but does not explicitly say to rewrite historical generated-output quotes. [VERIFIED: .planning/phases/52-operator-evidence-cleanup/52-CONTEXT.md]
   - Recommendation: Preserve the historical quote and add a nearby Phase 52 amendment/note after implementation. [VERIFIED: .planning/phases/51-live-smoke-fresh-status-integration/51-01-SUMMARY.md]
   - RESOLVED: Plan 52-01 selected the preservation path. The executor must leave historical Phase 50 quotes intact and add `## Phase 52 Support Summary Amendment` plus `## Phase 52 Preflight Wording Amendment` notes.

## Environment Availability

| Dependency | Required By | Available | Version | Fallback |
|------------|-------------|-----------|---------|----------|
| `cargo` | Rust format/build/test and package-scoped verification. | yes [VERIFIED: command -v cargo] | `cargo 1.94.1` [VERIFIED: cargo --version] | none |
| `rustc` | Rust compile/test. | yes [VERIFIED: rustc --version] | `rustc 1.94.1` [VERIFIED: rustc --version] | none |
| `bun` | TypeScript release-boundary checks and live-smoke regression wrapper. | yes [VERIFIED: command -v bun] | `1.3.9` [VERIFIED: bun --version; .bun-version] | none |
| `bazel` / Bazelisk | Repo smoke build and Bazel operator command examples. | yes [VERIFIED: command -v bazel] | Bazelisk `1.28.1`, Bazel `8.6.0` [VERIFIED: bazel version] | none |
| `cargo-llvm-cov` | Full `bash scripts/verify.sh` coverage gate. | yes [VERIFIED: command -v cargo-llvm-cov] | `0.8.5` [VERIFIED: cargo llvm-cov --version] | none |
| `git` | Repo-root discovery and GSD/doc workflow. | yes [VERIFIED: git --version] | `2.53.0` [VERIFIED: git --version] | none |

**Missing dependencies with no fallback:** None found. [VERIFIED: local command probes]

**Missing dependencies with fallback:** None found. [VERIFIED: local command probes]

## Verification Strategy

Nyquist validation is disabled in `.planning/config.json`, so no `## Validation Architecture` section is required. [VERIFIED: .planning/config.json]

Recommended targeted checks for the planner:

```bash
cargo fmt --manifest-path packages/Cargo.toml --all
cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli open_bitcoin_support_bundle --test operator_binary
cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --bin open-bitcoind
bun run scripts/check-v1.3-release-boundaries.ts
bash scripts/verify.sh
```

These commands align with the existing Phase 48 support verification pattern, Phase 51 full-verification pattern, and repo-native verification contract. [VERIFIED: .planning/phases/48-support-evidence-and-operator-runbooks/48-SUMMARY.md; .planning/phases/51-live-smoke-fresh-status-integration/51-01-SUMMARY.md; scripts/verify.sh:96-147]

## Security Domain

Security enforcement is enabled by default because `.planning/config.json` does not set `security_enforcement` to `false`. [VERIFIED: .planning/config.json]

### Applicable ASVS Categories

| ASVS Category | Applies | Standard Control |
|---------------|---------|------------------|
| V2 Authentication | Partial. Support evidence reports credential source metadata but must not expose credential values. [VERIFIED: packages/open-bitcoin-cli/src/operator/support.rs:214-233; docs/parity/threat-model-v1.3.md:49] | Metadata-only credential evidence and redaction boundaries. [VERIFIED: docs/operator/runtime-guide.md:520-530] |
| V3 Session Management | No direct session surface in this phase. [VERIFIED: target files are CLI support bundle and daemon preflight] | No new session state; do not add hosted support upload or remote administration. [VERIFIED: docs/parity/release-readiness.md:172-190] |
| V4 Access Control | Partial for operator claim boundaries; support bundles must not imply remote admin or production-node readiness. [VERIFIED: docs/parity/release-readiness.md:87-89,180-190] | Keep local-only support evidence and explicit non-claims. [VERIFIED: docs/operator/runtime-guide.md:526-530] |
| V5 Input Validation | Yes. The support command ingests an operator-supplied file path and parses untrusted JSON report content. [VERIFIED: packages/open-bitcoin-cli/src/operator.rs:197-203; packages/open-bitcoin-cli/src/operator/support.rs:442-489] | Parse JSON with `serde_json`, summarize known keys only, and treat malformed/missing input as unavailable evidence with reasons. [VERIFIED: packages/open-bitcoin-cli/src/operator/support.rs:451-508] |
| V6 Cryptography | No new cryptographic behavior in this phase. [VERIFIED: target files and Cargo manifests] | Do not add custom crypto; keep credential values out of reports. [VERIFIED: docs/parity/threat-model-v1.3.md:49] |
| V7 Error Handling and Logging | Yes for redaction/log-output handling. [CITED: https://devguide.owasp.org/en/06-verification/01-guides/03-asvs/; VERIFIED: docs/parity/threat-model-v1.3.md:49] | Do not embed raw daemon stderr/stdout tails or raw live-smoke input; summarize and sanitize allowlisted values only. [VERIFIED: .planning/phases/52-operator-evidence-cleanup/52-CONTEXT.md; scripts/run-live-mainnet-smoke.ts:119-156] |

### Known Threat Patterns for This Stack

| Pattern | STRIDE | Standard Mitigation |
|---------|--------|---------------------|
| Support bundle leaks RPC passwords, cookies, private keys, seed phrases, or raw live-smoke input. [VERIFIED: docs/parity/threat-model-v1.3.md:49] | Information Disclosure | Reuse recursive sanitizer and assert secret-like fixture strings are absent from JSON and Markdown. [VERIFIED: packages/open-bitcoin-cli/src/operator/support.rs:510-537; packages/open-bitcoin-cli/tests/operator_binary.rs:765-768,821-823] |
| Stale preflight wording misleads operators about current sync behavior. [VERIFIED: .planning/v1.3-MILESTONE-AUDIT.md:159-168] | Spoofing / Repudiation | Add exact message unit assertion and keep production-node non-claim in the wording. [VERIFIED: packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs:280-352; .planning/phases/52-operator-evidence-cleanup/52-CONTEXT.md] |
| Support bundle summary overclaims public-mainnet progress. [VERIFIED: docs/parity/threat-model-v1.3.md:49-50] | Repudiation / Spoofing | Summarize live-smoke `result` status/deltas/cause/next action exactly; do not treat support bundle existence as public-mainnet proof. [VERIFIED: scripts/run-live-mainnet-smoke.ts:146-155; docs/parity/release-readiness.md:142-145] |
| Malformed or missing live-smoke report blocks support evidence generation. [VERIFIED: packages/open-bitcoin-cli/src/operator/support.rs:451-489] | Denial of Service | Preserve non-fatal unavailable evidence states with reasons. [VERIFIED: packages/open-bitcoin-cli/src/operator/support.rs:442-489] |

## Sources

### Primary (HIGH confidence)

- `.planning/phases/52-operator-evidence-cleanup/52-CONTEXT.md` - locked decisions, target files, phase boundary, and deferred Phase 53 live evidence.
- `.planning/REQUIREMENTS.md` - OBS-02, OBS-03, v1.3 tech-debt traceability.
- `.planning/STATE.md` - current post-Phase 51 state and claim boundaries.
- `.planning/ROADMAP.md` - Phase 52 scope and success criteria.
- `.planning/v1.3-MILESTONE-AUDIT.md` - D-02 and D-04 debt descriptions.
- `packages/open-bitcoin-cli/src/operator/support.rs` - support evidence model, live-smoke ingestion, sanitization.
- `packages/open-bitcoin-cli/src/operator/support/render.rs` - Markdown rendering.
- `packages/open-bitcoin-cli/tests/operator_binary.rs` - support bundle integration tests.
- `packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs` - daemon sync preflight, worker startup, preflight tests.
- `scripts/run-live-mainnet-smoke.ts` - schema v2 report shape and Markdown result fields.
- `scripts/test-run-live-mainnet-smoke.sh` - deterministic live-smoke fixture regression.
- `docs/operator/runtime-guide.md`, `docs/parity/release-readiness.md`, `docs/parity/checklist.md`, `docs/parity/index.json`, `docs/parity/threat-model-v1.3.md` - operator and parity/audit wording.
- `AGENTS.md`, `AGENTS.bright-builds.md`, `standards-overrides.md` - repo workflow and Bright Builds routing.

### Secondary (MEDIUM confidence)

- Bright Builds canonical standards at commit `05f8d7a6c9c2e157ec4f922a05273e72dab97676` - architecture, code shape, verification, testing, Rust, and TypeScript guidance. [CITED: https://raw.githubusercontent.com/bright-builds-llc/bright-builds-rules/05f8d7a6c9c2e157ec4f922a05273e72dab97676/standards/index.md]
- OWASP Developer Guide ASVS page - security category framing for ASVS-style controls. [CITED: https://devguide.owasp.org/en/06-verification/01-guides/03-asvs/]

### Tertiary (LOW confidence)

- None.

## Metadata

**Confidence breakdown:**

- Standard stack: HIGH - no new dependencies are needed; versions and tools were verified from repo manifests and local commands. [VERIFIED: rust-toolchain.toml; packages/open-bitcoin-cli/Cargo.toml; packages/open-bitcoin-rpc/Cargo.toml; local command probes]
- Architecture: HIGH - target modules and helpers are already present and the phase decisions are locked. [VERIFIED: .planning/phases/52-operator-evidence-cleanup/52-CONTEXT.md; packages/open-bitcoin-cli/src/operator/support.rs; packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs]
- Pitfalls: HIGH - each pitfall maps to an existing audit debt item, code path, or release-boundary test. [VERIFIED: .planning/v1.3-MILESTONE-AUDIT.md; scripts/check-v1.3-release-boundaries.ts]
- Security/redaction: HIGH - the existing threat model names information disclosure for support evidence and code has sanitizer/test coverage to extend. [VERIFIED: docs/parity/threat-model-v1.3.md:49; packages/open-bitcoin-cli/src/operator/support.rs:510-537; packages/open-bitcoin-cli/tests/operator_binary.rs:716-824]

**Research date:** 2026-06-01  
**Valid until:** 2026-07-01 for repo-local patterns; re-check if Phase 53 changes live evidence schema or support bundle contracts. [VERIFIED: .planning/ROADMAP.md:175-178]
