---
phase: 165-shared-exact-work-authority
reviewed: 2026-10-17T04:10:01Z
client_date: "2026-10-16"
depth: standard
review_kind: preliminary_source_only
diff_base: d0104a997c0362bf65b81de1c8fec8ddc3012a1b
files_reviewed: 4
files_reviewed_list:
  - packages/open-bitcoin-bench/src/cases/sync_runtime.rs
  - packages/open-bitcoin-bench/src/cases/storage_recovery.rs
  - packages/open-bitcoin-cli/tests/operator_flows/domain_fixtures.rs
  - packages/open-bitcoin-cli/tests/operator_flows/basic_index.rs
findings:
  critical: 0
  warning: 0
  info: 0
  total: 0
status: clean
generated_by: gsd-code-reviewer
lifecycle_mode: yolo
phase_lifecycle_id: 165-2026-10-14T16-09-59Z
---

# Phase 165: Preliminary Plan 16 Rust Integration Review

**Reviewed:** 2026-10-17T04:10:01Z (client date 2026-10-16, America/Chicago)
**Depth:** Standard, with targeted cross-file authority and lifetime tracing
**Files reviewed:** Four complete Rust files and their diff from the fixed base
**Status:** Clean within this preliminary scope

## Summary

No actionable correctness, security or test-reliability issues were found in the four reviewed files. The benchmark setup exercises genuine empty-source connection, and the CLI fixtures retain the same genuine owner and explicit maturity-one parameters until legitimate work mutations finish. Raw persistence and final reopened observations remain separate from accepted work authority.

This report is preliminary source analysis. It does not replace the final whole-phase source review, security audit, default native verification or goal verification. No test, build, Cargo or Bun matrix was executed by this reviewer.

## Guidance and Scope

The review used root `AGENTS.md`, `AGENTS.bright-builds.md`, `standards-overrides.md`, `standards/index.md`, and the architecture, code-shape, verification, testing and Rust standards. The overrides file contains no applicable active exception. Neither `.claude/skills/` nor `.agents/skills/` exists. `.claudeignore` is absent; none of the four source paths is ignored by Git.

Both canonically resolved active lesson files were read completely: global 5,230 bytes and repository 1,958 bytes, totaling 7,188 bytes and a conservative 2,397-token estimate. No archive was loaded. Plan 16, the current `165-PLAN-CHECK.md` amendments, and the Plan 15 constructor summary informed the review. Parent-approved added ownership of `storage_recovery.rs` is explicit in the amendments; the older eight-path Plan 16 inventory alone does not describe that later approval.

The scope is the four files in frontmatter, including their existing assertion intent at the fixed base. Related constructor, managed network, candidate authorization, scripted transport, CLI harness and captured-export source were read only to trace the reviewed behavior. Those supporting files are not independently certified by this report.

## Behavior Traced

- `sync_runtime.rs:30`: The helper consumes a genuine empty core transferred into the actual Fjall view, selects explicit parameters, uses BASIC Disabled and default-off relay/inbound arguments, then admits only the root header outside P2P. It does not preconnect the genesis body.
- `sync_runtime.rs:95`: Mined child, grandchild and duplicate nonroot grandchild pass through three actual scripted rounds. Height two, final-round header/message counters, zero failed peers, exactly two GetHeaders, three Version handshakes and NoProgress after three rounds match the traced runtime behavior. `sync_until_idle` returns its last round's summary, so `headers_received == 1` and `messages_processed == 3` are deliberate per-round values.
- `sync_runtime.rs:161`: Initial absent active tip excludes a preconnected Duplicate result. Actual Version/Verack/Block intake must connect the genesis endpoint with its derived proof, report one block, and leave the exact body durably readable. Error propagation stays fallible through `BenchError`.
- `storage_recovery.rs:114`: Raw sample headers must retain exact entry count and complete per-entry equality, including their numerical carriers. Best tip remains absent, best height stays minus one and the accepted chain remains empty. The description states the raw evidence boundary. All other storage write/read and metadata, metrics and block roundtrip checks remain intact relative to the base.
- `domain_fixtures.rs:15`: Original transaction construction retains historical and same-block spends. Public source assembly actually accepts and flushes genesis with empty header entries, retains the same core and supplies the same maturity-one parameters to normal BASIC construction and subsequent ingress.
- `domain_fixtures.rs:293`: A future-header batch and the separate B400 batch use an actual modeled Version/Verack handshake and contiguous Headers messages, excluding the null-parent root. Actual header identities and parent-derived work are checked before body intake; the ingress peer is disconnected afterward. Populated source ownership is not rebound.
- `domain_fixtures.rs:59`: Duplicate root linkage changes no active position. The precomputed B400 bytes are retained and checked as NonExtending while genesis remains active, before any A child body. A399 later releases the pending body before A400 intake; actual A400 acceptance supplies the active preference binding. The managed candidate and reorg authorization paths enforce this independently of the ZERO numerical compatibility carrier.
- `domain_fixtures.rs:364`: Replacement loads the retained deterministic body and uses the genuine owner's normal reorg path. Its proof matches the displaced position's cumulative work, and final height, exact work and hash are asserted. No inflated score, order setter, longer replacement or raw reopened owner grants preference.
- `basic_index.rs:276`: Catching/current/automatic-target/lagged/ahead/durable transitions occur on the original live owner. Final BASIC equality and real operator/RPC/export consumers remain after all-handle closure. The shared RPC harness joins its server thread and releases context/store clones on return from each consumer assertion.
- `basic_index.rs:421`: Real serving-positive, prune-height-20 body and undo deletion, wallet refusal, NotFound/no DurableBlock, same-height replacement and retained displaced-row checks remain. The named network handle is explicitly dropped before runtime closure; result objects retain value-only response plans, not Fjall owners. Final reopen preserves BASIC and physical-absence/serving negatives.
- `domain_fixtures.rs:226`: Both final raw reopen paths attempt a real later connection and require the exact UntrustedWorkLineage cause, unchanged chainstate and complete physical coins snapshot, and absence of the refused body. They perform no replay or promotion to live work authority.

## Evidence Attribution

The parent relayed executor results of repeatability 1/1, complete benchmark library 16/16, benchmark smoke exit zero and CLI BASIC 6/6, plus passing strict lint/build and no later Rust source changes. These are reported executor evidence, not independently executed reviewer results. This reviewer performed source reads, fixed-base diff inspection, ignore checks, content hashing and scoped whitespace checking only. The scoped whitespace check produced no findings.

The reviewed changes neither establish canonical bootstrap/reopened work authority nor contextual completeness or pinned-Knots daemon differential qualification. Those later-phase boundaries remain outside this report.

## File Identity and Drift

Baseline hashes were captured before source analysis and recaptured at closure. Every closing hash equals its baseline; no scope drift was observed. SHA-256 values below apply to the complete working-tree bytes, not merely diff hunks.

| File                                                                | Bytes  | Lines | Baseline and closing SHA-256                                       |
| ------------------------------------------------------------------- | ------ | ----- | ------------------------------------------------------------------ |
| `packages/open-bitcoin-bench/src/cases/sync_runtime.rs`             | 8,019  | 225   | `62668b6f6627ef7b7d7ebb376c16d453748afbeeea78cc02bb8e6d0fdf3902cd` |
| `packages/open-bitcoin-bench/src/cases/storage_recovery.rs`         | 8,255  | 208   | `43eb1347f17f63431f98a542355a8352f2d51e5b34c7293f8057fe9c5eed116b` |
| `packages/open-bitcoin-cli/tests/operator_flows/domain_fixtures.rs` | 16,200 | 457   | `fd0934bb72ed28362477092f6216b1aa4a78b093cce453d29f756e22f3276525` |
| `packages/open-bitcoin-cli/tests/operator_flows/basic_index.rs`     | 19,615 | 571   | `d3b6c196c8ca8cca19d2ba64ae778e71b7207843bab7923a04781c1f008a99bc` |

Ordered scope digest, before and after: `f78bd439c739333034a09e62217da8ab21c912ca39d6e0707f54e3cd05b742d9`.

Digest construction is SHA-256 over the four frontmatter paths in that order, each encoded as UTF-8 `path`, tab, lowercase complete-file SHA-256, newline. A later change to any reviewed file requires delta review; this preliminary clean result applies only to the identities above.

## Reviewer Boundary

Only this report was written. No source, shared planning state, Git index, commit or remote was changed by this reviewer.

_Reviewer: gsd-code-reviewer_
