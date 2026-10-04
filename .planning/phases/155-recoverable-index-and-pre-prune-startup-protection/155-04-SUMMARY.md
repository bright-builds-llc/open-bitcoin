---
phase: 155-recoverable-index-and-pre-prune-startup-protection
plan: "04"
subsystem: testing
tags: [basic-filters, fjall, recovery, parity, native-verification]
requires:
  - phase: 155-01
    provides: Bounded BASIC integrity and pure fenced recovery/protection
  - phase: 155-02
    provides: Immutable Fjall publication and complete linear integrity scans
  - phase: 155-03
    provides: Mandatory production initialize guard before prune resume
provides:
  - Actual validated spend/fork recovery through real Fjall and production runtime reopen
  - Concrete partial coins and failed metadata flush boundaries without phantom cursor
  - Unique parity evidence owner and tested default native source/claim guard
affects: [phase-155-verification, compact-filter-index, durable-startup]
tech-stack:
  added: []
  patterns: [actual adapter fault seams, real runtime reopen evidence, injectable-root mutation checker]
key-files:
  created:
    - packages/open-bitcoin-node/src/sync/tests/filter_index/recovery.rs
    - packages/open-bitcoin-node/src/sync/tests/filter_index/faults.rs
    - scripts/check-phase155-filter-index.ts
    - scripts/check-phase155-filter-index.test.ts
  modified:
    - packages/open-bitcoin-node/src/sync/tests/filter_index.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/filters/publication.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/coins.rs
    - docs/parity/source-breadcrumbs.json
    - docs/parity/index.json
    - docs/parity/catalog/basic-compact-filters.md
    - docs/parity/catalog/README.md
    - README.md
    - scripts/verify.sh
key-decisions:
  - Keep genuine validated spend/branch evidence distinct from sparse deletion-order fixtures.
  - Reuse the existing concrete publication fault control for a cfg(test)-only metadata failure seam.
  - Source and scoped claim checks supplement durable behavior rather than establish completion by themselves.
requirements-completed: []
requirements-addressed: [CFIX-02, CFIX-04, CFPR-03]
generated_by: gsd-execute-phase
lifecycle_mode: yolo
phase_lifecycle_id: 155-2026-10-04T16-01-08
generated_at: 2026-10-04T18:23:48Z
commits: []
git_finalization: pending consolidated parent commit after formal phase verification and full native gate
duration: 24min
completed: 2026-10-04
---

# Phase 155 Plan 04: Cross-Boundary Recovery and Native Evidence Summary

**Validated spend/fork rows survive concrete coins and metadata failures, while production reopen fences visible progress and native mutation checks guard the evidence and deferred scope.**

## Performance and Verification

- First recorded Rust command: 2026-10-04T18:05:45.906Z; summary written 2026-10-04T18:23:48Z.
- All 3 tasks complete; 13 owned source/test/docs/verifier files plus this summary.
- Rust RED: `phase155-production-recovery-red` and confirmed RED exited 101 for the missing metadata fault variant, with an initially private initialize import corrected to its existing public re-export.
- Initial GREEN: `phase155-production-recovery-green` passed all 61 matching node library tests in 25.77s execution time.
- Expanded matrix: `phase155-production-recovery-matrix` passed all 63 matching node library tests in 30.19s execution time (69.601s total wrapper duration). Seven new Plan 04 tests supplement the 56 prior matching tests.
- Final fault rerun after moving live intent setup before the actual flush: `phase155-final-fault-matrix` passed all 4 cross-boundary fault tests in 5.29s execution time. The 155.126s wrapper duration includes compiler/launch overhead; it is not a storage latency result.
- Node Clippy, all targets/all features, `-D warnings`: initial `phase155-cross-boundary-clippy` passed; final `phase155-final-cross-boundary-clippy` passed after all Rust changes (11.12s Cargo time).
- Checker RED: absent checker import failed before its implementation. Final `bun test ./scripts/check-phase155-filter-index.test.ts`: 53 passed, 93 assertions, zero failures. Check mode preserves every evidence file byte; incomplete CLI fixtures exit 1 with actionable diagnostics.
- Existing Phase 154 checker and its 28 tests/53 assertions passed. Phase 123/135 live checkers and their combined 122 tests/185 assertions passed.
- Parity breadcrumbs passed for 947 tracked Rust files; both new Rust paths are registered and staged for inventory.
- Managed `all` checker passed: 1,266 source files, zero exceptions/findings. Panic-site and pure-core dependency/import checks passed.
- Scoped rustfmt, Bash syntax, and staged/unstaged diff whitespace review passed.

All Cargo work used the timing wrapper and library scoping, without overlapping owned builds. Quiet resumable sessions were polled and allowed to finish; no process was terminated and no execution security metadata changed. Direct Bun runs used the available PATH runtime 1.4.2; the repository/CI pin remains 1.3.9 and the parent owns full environment/native gate evidence.

## Durable Cross-Boundary Evidence

The new `ValidatedHistory` reuses existing real-store reopen/snapshot helpers. It mines easy-target blocks and drives first-party staged consensus validation under explicit fixture coinbase maturity 1. Genesis, funding and a three-transaction spending block provide historical funding script 51 and same-block script 52; committed snapshots prove both spent coins disappear. A second independently validated branch shares genesis/funding and changes the final spend output. These are scoped protocol fixtures, not public-mainnet or production-funds evidence.

The matrix proves:

- Record-only rows written ahead of deferred coins/meta advancement survive production reopen; B, saved fence, cursor and protection remain at the old consistent funding checkpoint, with no visible height-2 projection and exact immutable/body/undo retention.
- An equal-height validated replacement branch rewinds to the common indexed funding ancestor, republishes a compatible current fence and stronger protection, hides the displaced suffix, and retains both branches' immutable rows and payloads.
- Removing a required predecessor or projection refuses actual runtime reopen. Byte-for-byte BlockIndex snapshots, exact body/non-genesis undo, protection and live intent remain intact.
- The existing concrete coins view actually commits a partial capped batch and returns an interrupted-write error. Production startup consumes H/B replay first. Compatible target metadata permits readiness with exact recovered UTXOs while the saved index cursor remains old; stale metadata refuses after replay has consumed H and advanced B, preserving index bytes, payloads and unsafe intent.
- A cfg(test)-only hook in the actual Fjall metadata writer stops `FlushLifecycle::execute_flush` after successful coins B publication. Metadata/cursor/protection remain old; production reopen refuses the B/meta disagreement with exact rows, required payloads and pre-existing live intent retained.
- Before-record/checkpoint/protection and after-checkpoint commit faults are followed by actual production runtime reopen. The resulting cursor, visible projection, immutable row and protection are the complete old or new pair; body/undo remain independently checked.
- An error after the actual record-only commit retains ahead rows through runtime reopen without granting a cursor or projection. Returned errors never establish rollback.

Compatible replay target metadata and deliberate missing-prefix states are test setup facts, not an automatic repair behavior. Plan 03's sparse height-400 codec-valid fixtures continue to isolate genuine pre-delete ordering outside ordinary keep-window refusal; those fixtures do not claim consensus validation.

## Parity, Native Wiring and Scope

The unique `v2-5-recoverable-basic-index-and-startup-protection` surface owns exactly CFIX-02/CFIX-04/CFPR-03. CFIL-01/02 stay with their Phase 154 owner. The catalog/root README preserve Phase 154 oracle and heading anchors while describing the internal Phase 155 foundations and timing-wrapped reproduction commands.

Pinned Knots `index/base.cpp`, `index/blockfilterindex.cpp` and `node/blockstorage.cpp` anchors document committed-progress fences, retained branch rows and protection. Intentional differences explain additive versioned Fjall schema-2 envelopes versus flat files/LevelDB, conservative B/meta refusal, checkpoint-visible projection, historical higher-fence provenance and direct heights-0/1 startup protection.

The injectable-root checker requires source/manifest/parity/breadcrumb links, unchanged schema/dependency names, pure policy separation, mandatory recovered-coins guard ordering, preserved runtime constructor anchors, named validated/runtime/fault tests and scope caveats. Mutation tests remove each selected mechanism, move the guard past resume/readiness/cache construction, remove real reopen/validation/fault evidence, corrupt ownership/mapping, add I/O/dependencies and overclaim later surfaces. Actual executable checker/test `run_step` lines are in default verify; historical literal contracts and Phase 154 steps remain intact.

Full startup integrity scanning remains linear retained-history work with bounded additional record memory. The existing concrete 256-record/256-projection scan test passed within the expanded suite and measures exactly 1,023 reads (`4N - 1`), with no per-row full ancestor traversal. This is not a total runtime/memory cap, archive-scale or hardware power-loss claim.

Phase 156 ordinary prune ownership/CRUD/disable, 157 activation/scheduled catch-up, 158 runtime reorg and 159–162 RPC/peer/operator serving and integrated retained-client proof remain deferred. No implicit repair/download, public activation, scheduler, network endpoint or operator capability was added.

## Simplification and Source/Security Inventory

The explicit simplification pass retains the existing root fixture and raw reopen/snapshot helpers, one small validated-history recipe, one existing test-only fault control and a dependency-free source checker. There is no parallel memory store, second cache/deletion manager, generic fault framework or new production dependency. Every touched source/test file is below 628 lines; the new recovery and fault modules are 443 and 321 lines.

| Plan | Reviewed proof boundary | Evidence/limitation |
| --- | --- | --- |
| 01 | Hostile BASIC bytes and recovered authority | Bounded canonical codec, typed identities, borrowed B/meta ancestry fence, refusal-poisoned reducer and direct 0/1/max protection. Parent retains full pure-core coverage gate. |
| 02 | Durable envelopes and publication uncertainty | Bound before hashing, immutable equality, same-database SyncAll state/protection, serialized clones, unit-returning direct-edge proofs and linear full scan. Arbitrary raw coins writers still require runtime serialization. |
| 03 | Startup before irreversible pruning | H/B recovery precedes index guard; guard precedes resume/readiness/cache; saved protection and unsafe intents are checked before reconciliation. Historical higher fences are provenance only. |
| 04 | Cross-boundary behavior and release evidence | Actual validated branch/spend, partial coins commit and real metadata failure followed by production reopen; unique parity owner and source/claim mutation guard. Static checks do not replace Rust or formal native proof. |

T-155-14 is addressed by actual validated recovered authority plus exact immutable/payload/intent checks. T-155-15 is addressed by unique requirement ownership, real durable test links and executable default mutation guards, with parent formal/full verification still pending. T-155-16 retains category-only runtime failures without filter/script dumps. T-155-17 remains accepted: linear full-history work is required and its bounded corpus is not a performance promise. Applicable ASVS L1 validation/internal authorization/error-handling concerns are covered by these boundaries; no certification is claimed. No unresolved HIGH issue was identified in this scoped inventory.

No new unmodeled network, authentication, filesystem or schema trust boundary was introduced. Stub scan found no placeholder or unwired path needed for this plan. Existing narrow future activation/catch-up/serving allowances remain intentional future-phase boundaries.

## Deviations from Plan

**[Rule 2 - Missing critical evidence seam] Concrete metadata-flush failure.** Existing adapter faults did not cover a real metadata writer failing after successful coins publication. Added `BeforeChainMeta` and a cfg(test)-only method in the existing publication control, called only under cfg(test) by `FjallNodeStore::save_chain_meta`. Ownership was narrowly extended to `filters/publication.rs` and `coins.rs`. Production builds elide the seam. The actual FlushLifecycle/reopen regression and final Clippy prove the path. Commit: pending consolidated parent finalization.

The larger deterministic corpus was already implemented in Plan 02; Plan 04 reuses its measured test rather than duplicate it. No scope expansion was required.

Plan execution rules explicitly defer commits, STATE/ROADMAP/REQUIREMENTS edits, full native/Bazel/integration/coverage verification and requirement activation to the parent. No commits, hook bypass, external setup or authentication gate occurred.

## Handoff and Residual Risks

Ready for parent source/security/goal verification and full native gate, including ordered Rust checks, integration suites, coverage/LOC freshness and Bazel. All three requirement IDs remain Pending until that gate. The v2.5 milestone and later product capabilities remain pending. Software-close/reopen faults and easy-target maturity-1 fixtures do not prove hardware durability, mainnet operation, archive-scale behavior or production-funds safety.

Material guidance: local AGENTS/sidecar, placeholder overrides, architecture/code-shape/testing/verification/Rust/TS standards, both completely loaded active lessons, exact phase context/research/prior summaries, production source and pinned Knots anchors.

## Self-Check: PASSED

All 13 owned source/test/docs/verifier paths and this summary exist; scoped tests/checks above passed. Commit existence checks are intentionally inapplicable to the authorized consolidated-parent workflow (`commits: []`). Canonical STATE/ROADMAP/REQUIREMENTS were not edited by this executor.
