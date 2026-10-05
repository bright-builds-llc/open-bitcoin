---
phase: 157-safe-activation-and-scheduled-index-catch-up
reviewed: 2026-10-05T21:50:58Z
depth: standard
files_reviewed: 9
files_reviewed_list:
  - packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs
  - packages/open-bitcoin-rpc/src/bin/open_bitcoind/checkpoint.rs
  - packages/open-bitcoin-rpc/src/bin/open_bitcoind/coins_flush.rs
  - packages/open-bitcoin-rpc/src/bin/open_bitcoind/coins_flush/tests.rs
  - packages/open-bitcoin-rpc/src/bin/open_bitcoind/coins_flush/tests/filter_index.rs
  - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests.rs
  - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/checkpoint.rs
  - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index.rs
  - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/fixtures.rs
findings:
  critical: 0
  warning: 0
  info: 0
  total: 0
status: clean
generated_by: gsd-code-reviewer
lifecycle_mode: yolo
phase_lifecycle_id: 157-2026-10-05T14-50-48
generated_at: 2026-10-05T21:50:58Z
review_partition: daemon-bin
review_iteration: 2
scope_source_sha256: ac51fad08894c4455610d1ace026825e9cf864dcdb35433f020673b6bdec113f
base_commit: 26bc454a66d6d7a241fc01be84dc6fbf0416d15a
source_state: uncommitted frozen implementation including explicitly scoped untracked source
whole_phase_review_complete: false
verification_owner: root
---

# Phase 157: Daemon Partition Code Review Report

**Reviewed:** 2026-10-05T21:50:58Z\
**Depth:** standard\
**Files Reviewed:** 9\
**Status:** clean within this explicit partition

## Summary

The corrected production shutdown path closes iteration-one WR-01. A retained BASIC failure no longer bypasses retry joining or final mempool settlement. Worker and HTTP failures remain visible and prevent clean-marker publication. No new actionable bug, security issue or reliability finding was found in the frozen nine-file scope. All reviewed files meet quality standards. No issues found.

Every listed file was read completely, including registered tests and fixtures. Constructor and worker callees were cross-referenced for actual wiring and ordering. Phase CONTEXT, Plan 08, its summary, the preserved iteration-one review, REVIEW-FIX and SHUTDOWN-FIX-SCOPE supplied intent and reported verification provenance; source and tests supplied the independent review evidence.

## WR-01 Closure Evidence

- `open-bitcoind.rs:176-195` completes HTTP serving and inbound shutdown, then calls production `settle_daemon_shutdown` with actual sync, coins, retry and mempool worker callbacks. The durable clean marker is a separate callback. Coins failure no longer returns before the remaining settlements.
- `checkpoint.rs:106-136` eagerly evaluates sync, coins, retry and checkpoint callbacks before inspecting failures, together with the HTTP result. It reports every returned error, retains the first, and returns it before the clean callback. Retry quiesces before final mempool capture. The coordinator is production code; old isolated clean conveniences are test-only.
- `coins_flush.rs:76-92` and `checkpoint.rs:71-91` evaluate joins even when signaling fails, and report secondary join/settlement failures. Final mempool settlement itself does not publish a clean marker.
- `coins_flush.rs:140-162` stops index scheduling at shutdown, runs Always, and returns any retained index error. It does not swallow that failure or release saved Active protection.
- `coins_flush/tests/filter_index.rs:15-154` exercises the production coordinator with an actual CoinsFlushWorker running the real loop, channel stop and join, plus actual retry and mempool workers. Its accepted-unflushed B40 has no persisted body; catch-up through B39 precedes a genuine BASIC failure. Assertions require successful Always to B40, retry joining, current-generation mempool evidence, visible failure, withheld clean marker, and unchanged Active protection/prune locks. Cleanup of workers skipped by a regression occurs after evidence capture and does not set the success flags.
- `tests/checkpoint.rs:418-524` covers eager settlement after multiple worker failures, HTTP failure withholding the marker, and marking clean only after successful ordered settlement. `tests/checkpoint.rs:366-414` checks the actual daemon callbacks and coordinator sequence. These controls support the fix without a separate test-only shutdown implementation.

## Other Reviewed Boundaries

| Boundary | Source-backed assessment |
| --- | --- |
| Durable activation | `open-bitcoind.rs:373-403` requires an existing directory for BASIC. Explicit zero with a valid directory selects durable open. Omission without another durable trigger stays transient; existing sync/inbound/prune triggers remain. |
| Startup mode and first turn | `open-bitcoind.rs:306-324` passes Enabled for BASIC and Disabled for actual durable omission/zero. The configured constructor initializes with that mode before owner construction and performs the first turn. The daemon adds no duplicate turn. |
| Refusal and preservation | Registered actual-loader/startup controls cover bare/1/basic, missing/nonexistent datadir, empty history, separate missing body/undo, preserved sources/owner state, saved checkpoint zero/reopen, and omitted transient preservation. Height 15 after one ordinary turn supports one eight-row startup prefix; direct private evidence remains in the runtime partition. |
| Networking | Relay/block-serving/inbound settings pass through unchanged. The sync selector returns None when sync is disabled. Controls inspect disabled inbound listener, absent endpoints/workers, zero peers and absent compact-filter service bit. Existing authenticated local RPC remains present. |
| Ordinary scheduler | Concrete durable and transient adapters avoid downcasts. Durable storage selects the existing coins worker, including index-only startup. Each elapsed event performs Periodic maintenance and one bounded index turn, then returns to the one-second wait. |
| Progress and durability | The production elapsed body is exercised across prefixes 15/23/31/39/40, genuine accepted connect, truthful completion, bounded counters, Periodic B40, zero-generation protection advancement and closed configured Fjall reopen. Shutdown adds no index turn and preserves Active ownership. |
| Registration and fixtures | Module declarations register all controls. Continuous consensus staging includes historical and same-block spends; an atomic fixture label prevents parallel path reuse. Source assertions complement behavioral controls. |

## Material Guidance and Scope

AGENTS.md, AGENTS.bright-builds.md, placeholder-only standards-overrides.md, standards/index.md, and architecture, code-shape, testing, verification, local-guidance and Rust standards materially informed this review. The GSD code-review skill and reviewer contract were applied. Project skill directories are absent.

Both canonical active lesson files were read completely: global 5,230 bytes and repository 1,958 bytes, totaling 7,188 bytes and 2,397 conservative estimated tokens. No archive was loaded. Existing phase context records the audit baseline; no lesson or audit mutation occurred.

The exact nine-file scope comes from `/tmp/phase157-daemon-rereview-files.json` and is preserved in frontmatter. None is ignored or generated. The fingerprint is SHA256 over entries in inventory order, concatenating each UTF-8 repository-relative path, NUL, lowercase SHA256 of its bytes, and newline. Constructor and existing worker callees were read only to cross-reference scoped consumers; they do not expand this partition. Only this review artifact was changed.

## Verification Provenance and Limits

This reviewer ran no Cargo, Bazel, build, test or Git command and did not enumerate quarantined dependencies. Root and REVIEW-FIX report final shutdown controls **4 passing**, inherited daemon controls **63 passing**, strict normal RPC Clippy and scoped formatting passing, Phase 135 live guard passing, and **88 passing** mutation/snapshot controls. The inherited daemon run preceded final real coins-thread test strengthening and secondary signal/join diagnostics; affected shutdown controls, strict Clippy and formatting passed afterward. Plan 08 additionally reports 9 startup and 7 idle controls passing. Counts overlap and are not summed.

Independent closure relies on actual source and regression structure, not summary intent as execution proof or a newly executed reproduction. Fixtures use synthetic easy-proof headers and test maturity one; they do not establish public-mainnet behavior. Index work budgets do not impose hard latency bounds on startup preflight, ordinary coins/prune work or storage. Clean-marker conclusions here apply to the coordinator's sync/coins/retry/mempool and HTTP results, preserving existing inbound adapter behavior.

## Balanced Simplification Pass

One small eager coordinator in the existing checkpoint adapter makes ordering and the clean gate explicit. Settlement-only worker ownership avoids duplicated per-worker branching and a new aggregate public error type or service. Existing typed adapters, configured constructor and maintenance loop remain sufficient. All nine files remain below the managed 628-line refactor trigger. No additional worker, history cache, downcast or dependency is needed.

## Result and Remaining Review

Nine files reviewed at standard depth: zero critical findings, zero warnings and zero informational findings. Iteration-one WR-01 is closed for this partition. The clean 58-file runtime report is unchanged. The excluded ten guard/documentation/catalog paths form the pending third partition; native whole-phase verification, coverage, Bazel, lifecycle/security closure and Git finalization remain root-owned. This report does not claim whole-phase completion.

______________________________________________________________________

_Reviewed: 2026-10-05T21:50:58Z_\
_Reviewer: gsd-code-reviewer_\
_Depth: standard; daemon/bin partition only_
