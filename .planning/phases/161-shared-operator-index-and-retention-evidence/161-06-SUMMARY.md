---
phase: 161-shared-operator-index-and-retention-evidence
plan: "06"
subsystem: cli
tags: [basic-filters, human-status, support, privacy, shared-rendering]
requires:
  - phase: 161-05
    provides: Existing RPC and CLI BASIC snapshot forwarding
  - phase: 161-11
    provides: Sticky backend and integrity failure evidence
provides:
  - One pure nineteen-row BASIC formatter shared by human status and support Markdown
  - Exact common reason normalization for serde, direct human rendering and support copies
  - Actual export-file agreement, single-capture, immutability and hostile-boundary evidence
affects: [161-07, 161-08, 161-09]
tech-stack:
  added: []
  patterns: [Captured typed facts, shared label/value formatter, exact reason allowlist]
key-files:
  created:
    - packages/open-bitcoin-cli/src/operator/status/render/basic_index.rs
    - packages/open-bitcoin-cli/src/operator/status/render/tests/basic_index.rs
    - packages/open-bitcoin-cli/src/operator/support/redaction/basic_index.rs
    - packages/open-bitcoin-cli/src/operator/support/render/basic_index.rs
    - packages/open-bitcoin-cli/src/operator/support/tests/basic_index.rs
  modified:
    - packages/open-bitcoin-cli/src/operator/status.rs
    - packages/open-bitcoin-cli/src/operator/status/render.rs
    - packages/open-bitcoin-cli/src/operator/status/render/tests.rs
    - packages/open-bitcoin-cli/src/operator/status/tests/snapshot.rs
    - packages/open-bitcoin-cli/src/operator/support/redaction.rs
    - packages/open-bitcoin-cli/src/operator/support/render.rs
    - packages/open-bitcoin-cli/src/operator/support/render/inbound.rs
    - packages/open-bitcoin-cli/src/operator/support/tests.rs
    - packages/open-bitcoin-cli/src/operator/support/tests/recovery_progress_inbound.rs
    - packages/open-bitcoin-node/src/status/basic_index.rs
    - docs/parity/source-breadcrumbs.json
key-decisions:
  - Reuse the node contract's exact reason allowlist before direct Rust rendering as well as serde.
  - Keep the render module private and expose its pure formatter through the existing status module.
  - Preserve supplied owner state, latch, progress, failure, retention and protection without renderer inference.
  - Full peer-scope copy discloses modeled-origin limits; backend guidance covers read failures as well as persistence.
requirements-completed: []
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 161-2026-10-11T04-48-29
generated_at: "2026-10-11T07:27:59Z"
duration: 13min context and evidence window
completed: 2026-10-11
---

# Phase 161 Plan 06: Shared BASIC Human and Support Evidence Summary

**Human status and support Markdown now share complete typed BASIC rows, while exact reason normalization protects direct Rust rendering and exported JSON without changing captured safe facts.**

## Accomplishments and API

All three tasks are complete across sixteen source/manifest paths. Five Rust children were added and registered; the other eleven paths contain integration, the reusable existing reason utility or focused compatibility updates. Prior plans' changes were preserved. No dependency, crate, endpoint, persisted schema, state collector or configuration resolver was added.

Plan07 can import `crate::operator::status::basic_index_entries(&BasicIndexOperatorStatus) -> Vec<(String, String)>`. It returns exactly nineteen rows in approved order: State, Init sync, Lag, Processed, Durable, Failure, Retained, Index bytes, Filter bytes, Records, Indexing, Peer serving, Peer scope, Target, Protection, Basis, Next action, Retention note, Body/wallet note. The existing render module stays private. Human output inserts its heading and rows immediately before Prune; support inserts `## BASIC index` in the same relative position and consumes the same formatter.

The formatter reads only supplied typed fields. Earned initial sync, current lag, processed and safe durable endpoints stay separate. Observed `none`, actual genesis `0`, `none seen`, measured zero and unavailable evidence retain different meanings. Failure categories remain missing body, missing undo, backend, integrity, resource and invalidated; no-failure is distinct from unavailable. Failed owner evidence can coexist with independently supplied retained observations and protection. Payload bytes remain a named subset and are never added to record key/value bytes.

Full Basis copy includes the approved immutable-record wording and explicit exclusions for active projections, runtime state, owner/checkpoint/protection/lock metadata, Fjall overhead and physical filesystem allocation. Full peer-scope copy reports all five supplied counts and explains the producer's modeled-origin limit: implicit origins are not modeled and count zero, ordinary peers are not relabeled implicit, and all capability can advertise without a backend. Synthetic fixtures now use a zero implicit count consistent with that producer. Safe decimal counts are preserved, rather than rederived or coerced by presentation.

All fourteen BASIC `FieldAvailability` reason boundaries use the existing node vocabulary. `BasicIndexUnavailableReason::normalize(&str) -> &'static str` exposes the previous serde-only exact whole-string allowlist. Serde, direct Rust human formatting and support-copy normalization call the same utility. Unknown, suffixed, newline or ANSI text maps to `BASIC evidence not reported`; valid exact causes survive. Available typed values are left intact. The existing `support_status_for_bundle` sanitizes its owned copy before either JSON or Markdown rendering, and export accepts the already captured snapshot without any live client reference.

The stale inbound support sentence now preserves inactive relay/mempool/bloom evidence, states that BIP37/bloom remains unsupported, and explains default-off BASIC serving's capability and readiness prerequisites. It does not infer general serving from an all-capability session.

## Executed Verification

Every Cargo command ran serially through `bun run scripts/command-timings.ts run --key KEY -- ...`. `/tmp/open-bitcoin-bun-1.3.9-fresh` was prepended to PATH and re-probed as Bun 1.3.9, matching `.bun-version`; Rust remains 1.94.1. No command was killed on yield or an estimate.

| Timing key / check | Actual result | Duration |
| --- | --- | ---: |
| `phase161-human-render-red`: CLI `phase161_basic_render` | Five discovered tests failed on missing actual BASIC human behavior | 15,151 ms |
| `phase161-support-red`: CLI `phase161_basic_support` | One discovered test failed because the direct Rust bundle copy still retained a hostile reason | 9,611 ms |
| `phase161-human-render`: CLI `phase161_basic_render` GREEN | Twelve discovered, twelve passed | 4,977 ms |
| `phase161-support`: CLI `phase161_basic_support` GREEN | Seven discovered, seven passed | 3,335 ms |
| `phase161-support-regression`: CLI `--all-features operator::support::` | Seventy discovered, sixty-nine passed; stale exact permission sentence failed and was updated | 9,146 ms |
| `phase161-consumer-regression`: CLI `--all-features operator::` | 380 discovered, 378 passed; two old adjacency assertions failed and were updated | 7,448 ms |
| `phase161-consumer-regression-final`: CLI `--all-features operator::` | 380 discovered, 380 passed on final behavior/copy | 7,059 ms |
| `phase161-reason-contract`: node `--all-features phase161_contract` | Thirteen discovered, thirteen passed | 7,727 ms |
| `phase161-consumer-clippy-final`: CLI and node `--all-targets --all-features -- -D warnings` | Passed on final source | 8,358 ms |
| Scoped Rustfmt, edition 2024, `skip_children=true` | All fifteen owned Rust paths passed check mode | Passed |
| `bun scripts/check-parity-breadcrumbs.ts --check` | 1,114 Rust paths verified | Passed |
| `bun scripts/bright-builds-check.ts all` | 1,439 inputs, zero findings; three repository lessons, zero findings | Passed |
| `git diff --check`, owned source review and stub scan | Passed; no implementation stubs found | Passed |

Exact final Cargo commands, each through its timing key above:

```bash
cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli --all-features operator:: -- --nocapture
cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --all-features phase161_contract -- --nocapture
cargo clippy --manifest-path packages/Cargo.toml -p open-bitcoin-cli -p open-bitcoin-node --all-targets --all-features -- -D warnings
```

The final two selected test runs cover **393 distinct cases**, including twenty new cases from this plan: twelve human, seven support and one node normalization test. Earlier focused runs overlap those final selections and are not additional distinct cases. One intermediate compile attempt identified the private render-module import, resolved through the authorized one-line re-export before GREEN; it discovered no tests.

The privacy matrix executes fourteen reason boundaries times fifteen hostile strings, including scripts, undo/filter bytes, transaction/block/peer identifiers, addresses, credentials, storage keys, filesystem paths, raw lock names, ANSI, Markdown newlines and a valid-label/private-suffix attack. It checks the original source, the exact expected sanitized copy, structured JSON, Markdown and direct human output. A separate fourteen-by-eight matrix preserves every exact allowlisted reason. The node utility checks every permitted label and rejects private suffix, ANSI prefix and newline variants.

Actual support-command tests inspect `support-evidence.json` and `support-evidence.md` written by the existing exporter. One wraps the existing RPC fixture in a counting adapter, collects live status once and proves the export preserves every BASIC safe field with exactly one network capture total. Another actual file export poisons every reason and verifies normalized structured fields and absent injected content. The complete BASIC structured JSON is deserialized and compared with captured input; ordered Markdown rows and human rows are compared with the same shared formatter.

These are actual collector/export adapter tests with controlled typed fixtures. Genuine Fjall authority-to-RPC-to-all-consumer proof remains Plan08's responsibility; no synthetic snapshot is presented as a new storage-authority proof.

## Threat Mitigations

| Threat | Evidence |
| --- | --- |
| T-161-06-1: disclosure through free-text reasons | One exact node allowlist shared by serde, direct human values and support-copy sanitization; all fourteen boundaries receive hostile controls. |
| T-161-06-2: captured safe fact tampering | Available values preserved; caller/source and expected-copy equality; full structured JSON round-trip equality; actual export files; one counted live collection. |
| T-161-06-3: guidance implies authority or mutation | Typed fixed guidance only; separate safe progress, availability, target and protection; no recovery, activation, downloading, repair or lock-release operation. |

No new network, authentication, schema or filesystem boundary was introduced. The support writer remains the existing authorized export path. No additional threat flag was identified; independent final review remains root-owned.

## Deviations and Simplification

Four narrow additional paths were authorized by the parent and preserved their existing changes:

1. **Rule 2, privacy correctness:** `node/src/status/basic_index.rs` exposes the existing reason allowlist and adds direct exact-vocabulary regression coverage. This prevents separate serde and Rust-renderer privacy rules. No vocabulary or wire shape changed.
2. **Rule 3, module visibility:** `cli/src/operator/status.rs` adds one crate-visible re-export because its render module is private. The module stays private and collection behavior is unchanged.
3. **Rule 1, compatibility expectation:** `support/tests/recovery_progress_inbound.rs` updates only the stale exact permission-copy expectation. Its other permission assertions remain intact.
4. **Rule 1, compatibility expectation:** `status/tests/snapshot.rs` now asserts BASIC-before-Prune and shifts the existing Prune-before-Wallet positions. The owned render test receives the equivalent insertion-aware assertion. Production collection was not changed to satisfy old adjacency.

Task2's initial support test was declared once in the final owned `support/tests.rs` location while Tasks2/3 were developed together, avoiding a temporary test declaration and relocation. There is no duplicate module discovery. The full operator selection was used once the support regression exposed a compatibility expectation; its only additional failures were the two related row-adjacency checks, and the corrected full selection passed.

The explicit simplification pass retained one generic availability renderer, small typed copy matches, a single shared row source and the existing support-copy owner. There is no renderer-owned status model, serializer round-trip sanitizer, duplicated reason vocabulary, resolver, retry, state inference or generic rendering framework.

Material guidance included AGENTS/sidecar, placeholder-only overrides, architecture/code-shape/testing/verification/operability/Rust standards, finalized context/research and revised approved terminal UI contract, and Plans04/05/11's actual APIs. Both canonical active lesson inputs were read completely (7,188 bytes; 2,397 conservative estimated tokens); existing audit baselines did not trigger another audit. Project skill directories are absent. Work stayed within the parent GSD lifecycle and ownership contract.

## Known Stubs and Residual Limits

No stubs prevent this plan's goal. Unknown or uncollected evidence remains explicitly unavailable. Plan07 owns compact interactive presentation and normal/narrow buffer proof; it should abbreviate full rows in the view without combining earned latch, lag or endpoints. Plan08 owns genuine cross-layer evidence, and Plan09/root owns the full native frozen-source gate, independent source/security review, formal requirement closure and finalization.

CFOP-01 remains pending. D-16 and Phase162 ownership remain intact. No repair/download, wallet acceleration, public-default, archive-scale, production-service/readiness, funds-safety or stronger crash-window claim is made.

## Task Commits

None under the explicit outer-wrapper contract. This executor staged, committed and pushed nothing and made no STATE, ROADMAP, REQUIREMENTS, config, todo or lesson edits.

## Self-Check: PASSED

All sixteen source/manifest paths and this summary exist. All five new Rust paths are registered. Final nonzero test selections, strict CLI/node lint, scoped formatting, parity, managed checks and whitespace checks passed. Commit checks are inapplicable because the parent owns finalization.
