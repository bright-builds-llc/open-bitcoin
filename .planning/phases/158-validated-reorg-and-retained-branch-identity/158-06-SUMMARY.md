---
phase: 158-validated-reorg-and-retained-branch-identity
plan: "06"
subsystem: testing
tags: [rust, fjall, compact-filters, validated-reorg, fault-recovery, resource-accounting]
requires:
  - phase: 158-05
    provides: Continuous genuine spending forks, retained sources and configured full-handle reopen
provides:
  - Integrated pre-preview loss, preview versus absorption and persistence fault matrix
  - Three configured reopened catch-up gates with incompatible physical suffix rows
  - Reserved protection, genuine automatic pruning and ordinary flush cadence evidence
  - Actual 54-configuration preparation and 149-turn measured work evidence
affects: [158-07]
tech-stack:
  added: []
  patterns: [immutable test-only work observations, full-handle configured recovery, checked composition of existing caps]
key-files:
  created:
    - packages/open-bitcoin-node/src/sync/tests/filter_index/reorg/failures.rs
    - packages/open-bitcoin-node/src/sync/tests/filter_index/reorg/measurements.rs
    - packages/open-bitcoin-node/src/sync/tests/filter_index/reorg/protection.rs
    - .planning/phases/158-validated-reorg-and-retained-branch-identity/158-REORG-MEASUREMENTS.md
  modified:
    - packages/open-bitcoin-node/src/sync/tests/filter_index/reorg.rs
    - packages/open-bitcoin-node/src/sync/tests/filter_index/reorg/fixtures.rs
    - packages/open-bitcoin-node/src/chainstate/filter_reorg.rs
    - packages/open-bitcoin-node/src/network/runtime_authority/filter_index.rs
    - docs/parity/source-breadcrumbs.json
key-decisions:
  - Inspect physical rows only after dropping every runtime and store owner
  - Measure immutable work components and whole-call times without inventing isolated index latency
  - Reuse existing independent caps and preserve explicit retained-fork resource refusal
  - Cross the actual regtest automatic-prune threshold with continuous validated compact history
  - Keep the automatic proof focused on replacement rows, exact-tip fencing and actual retained/deleted inputs
requirements-addressed: [CFIX-03]
requirements-completed: []
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 158-2026-10-08T02-17-15
generated_at: "2026-10-09T03:34:14Z"
duration: 35min
completed: 2026-10-09
---

# Phase 158 Plan 06: Reorg Refusal, Recovery, Protection and Measured Work Summary

**Actual configured recovery replaces incompatible masked projections without losing immutable branch history, while genuine source loss, writer faults and resource exhaustion preserve conservative authority.**

## Performance and Scope

- Tasks: **3/3 implemented and locally verified**.
- Approximately 35 minutes from the root's Plan 06 dispatch around 02:59 UTC through final source checks at 03:31 UTC and summary/self-check completion at 03:34 UTC. Dates are 2026-10-08 CDT / 2026-10-09 UTC. This is execution duration, not a reorg latency measurement.
- Sixteen required new named tests, plus one explicitly executed ignored timing experiment. No required behavior test is ignored.
- Four new implementation/evidence artifacts, five shared modified implementation/registry paths and this summary. The final formatted Rust files are 616/399/364 lines for failures/measurements/protection, 198 for the network adapter and 282 for filter_reorg; the existing chainstate parent remains 627.
- No production dependency, crate, endpoint, schema, worker, cache, authority factory, source repair/download, forced index-specific coins flush or new numeric cap was added.
- Root retains STATE.md, ROADMAP.md, todo, requirements, full native verification and all Git finalization. One initial read-only status inspection occurred; no staging, commit, push, worktree or hook bypass occurred.

## Task 1: Refusal and Actual Reopen Matrix

Eight required named failure tests cover twenty-two concrete cases:

| Boundary | Actual evidence |
| --- | --- |
| Missing required body versus undo | A genuine 160-block history and depth-32 fork are generated before deleting exactly one durable mate at height 140. The other mate remains. Before preview, production refusal preserves chainstate, owner/progress, mempool, all hash commitments, locks and an unfrozen append capability. Full drop/reopen also preserves the exact BASIC namespace key/value prefix and refuses the same physical loss. |
| Reopened missing undo | The earlier core stage emits the precise `missing undo data` category because the cache was genuinely lost. The live native preflight emits `required undo`; both are explicit pre-preview refusals. |
| Preview/mempool failure | The existing Serving preparation guard is invoked through actual network reorg. Preview does not advance the explicit accepted BASIC target or generation, does not change mempool/checkpoint/locks, and suspends later publication. Real configured reopen returns to old recoverable authority. |
| Accepted body/undo/coins/metadata failure | Actual BeforeBody, BeforeUndo, BeforeCoins and BeforeChainMeta seams preserve genuinely accepted replacement identity and conservative safe/protection state. The body writer failure uses the existing host persistence-failure projection. No later turn publishes from poison. |
| Coins persisted before metadata | BeforeChainMeta leaves new coins with old durable metadata; actual configured reopen explicitly refuses `coins best block differs from durable metadata`. The exact saved index namespace remains unchanged by refusal. No repair success is invented. |
| Rewind publication | BeforeRecords, BeforeCheckpoint, BeforeProtection and AfterCommit occur after genuine absorption. The explicit accepted target is the replacement even when pure achieved progress is paused. Old immutable commitments survive, and actual reopen follows old recoverable coins. |
| First/subsequent replacement append | Four publication seams are tested on both the first and a subsequent bounded turn. Actual written immutable replacement rows survive conservative configured recovery; no phantom safe advancement is claimed. |

The existing Phase 158 source regression suite also passed the genuine same-shape foreign undo refusal, corrupt undo, raw-metadata authority refusal, old-position/foreign-position proof refusal, stale already-owned append and generation/revision exhaustion tests. These are reused predecessor evidence, not counted as new Plan 06 tests.

## Mandatory Configured Reopened Catch-Up Gates

All three amended gates passed through genuine Enabled startup and parameter-free ordinary bounded turns:

1. **Index ahead of old coins:** A's durable tip remains height 39 while B's complete replacement projection is physically present from height 11. After full drop, raw keys prove the conflicting B projection. Configured reopen catches up to canonical A using retained genuine A sources.
2. **Checkpoint rewind with conflicting suffix:** A→B earns B's ordinary own flush; B→A then replaces projection without flushing A. A's physical height-11 projection conflicts with durable B and the safe cursor is shared height 10. Full drop/configured reopen resumes B and overwrites the masked incompatible suffix through accepted positions.
3. **New coins with partial indexed replacement:** B processes eight rows, earns ordinary coins/metadata persistence and performs the next ordinary eight-row turn, leaving sixteen genuine immutable B rows and an incompatible physical A projection at height 27. After full drop, configured startup and later ordinary turns complete canonical B.

The fixture's narrow `reopen_inspecting` callback runs **after runtime/store/proof owners are gone and before reopening the configured runtime**. It grants no capability. The tests compare raw immutable envelopes with exact expected encoded records and physical projections, then assert canonical active records, accepted lag zero, exact safe tip and conservative protection through subsequent turns. Old and already-written new bytes/header/hash/parent commitments remain exact. No retained stage, manual enable, repair, download or reopen workaround is used for normal live progress.

The ordinary subsequent validated-connect target advancement remains Plan 05's continuously exercised A→B→A→child regression and passed in the full Phase 158 run.

## Task 2: Protection and Ordinary Durability

Four required named tests cover reserved lock CRUD, equal/longer no-pressure cadence, stale manual work, interrupted startup and genuine automatic pruning.

- Equal-height and longer accepted replacements finish index work while normal IfNeeded reports no coins write. Durable coins and metadata remain exact old values; shared safe progress and reserved locks remain conservative. Later ordinary Always reports an actual own coins write and earns the exact replacement tip checkpoint.
- Operator clear and replacement of the reserved BASIC identity both refuse. A preplanned manual height and empty stale caller locks cannot override newly renewed generation/protection; the concrete paired-delete owner independently refuses the required source before mutation. This small manual fixture has a secondary keep-window constraint; the direct required-input check and preexisting continuous outside-window proofs retain their distinct evidence.
- An interrupted intent after accepted/indexed reorg is rejected by actual configured startup before deletion. Exact index bytes, live intent, required body/undo and replacement immutable rows survive. Plan 05's separate 401-height case retains actual outside-keep-window startup evidence.
- The automatic case validates **1,025 continuous compact-history blocks**, crosses the unchanged actual regtest threshold 1,000, warms the old generation's automatic state, accepts a longer fork and invokes Periodic again in the same second. Test-only `Automatic { target_mib: 0 }` creates pressure through the existing trusted test handle; this does not claim production option-parser minimum-target parity. Newly legal shared-prefix candidates are genuinely deleted; the normal owner reports a coins write. Required replacement bodies/undo survive, the final checkpoint/coins/lock match the replacement tip, all sixteen replacement active records match, and an actually pruned shared height-20 commitment remains exact.

The simplification pass removed a redundant full ancestry walk for each of 1,017 shared-prefix records from the automatic protection test. It keeps the actual threshold, continuous history, deletions, own checkpoint, all replacement rows and exact pruned shared commitment. The final focused protection run passed in 129.37s versus the earlier broader-assertion run's 247.36s; these are harness observations, not a production performance delta.

## Task 3: Actual Work and Finite Refusal

[158-REORG-MEASUREMENTS.md](158-REORG-MEASUREMENTS.md) preserves every actual observation from **54 configurations and 149 complete ordinary turns**, the exact commands/tool versions, component ledgers, whole-call timings, old/processed/safe identities and residual scope.

- Prefixes 16/128/512, depths 1/8/32, equal or three-block-longer replacements and three seeded positions produce observed progress before, within or after the fork where startup leaves an interior.
- The normal fixed-depth-eight test asserts constant preparation record/projection operations and completed-turn record/projection/point reads across all three prefixes.
- Every preparation observation is individually checked against existing storage, source and accepted-fact ceilings; checked addition equals the reported total and fits the checked sum of independent existing caps. Every observed ordinary turn includes acquisition through append completion and fits the selected existing policy.
- Fully indexed representatives use 20 storage-preparation record, 8 projection and 3,221 checkpoint operations, 5,096 reserved clone bytes, 16 actual preparation BASIC reads and 36 actual complete production-reorg BASIC reads. Source work scales with required depth/backlog and actual envelope sizes.
- The first full eight-record replacement turn borrows one captured fact: seven body reads/decodes/undo borrows, eight generations, 72 record / 42 projection operations and 44 BASIC reads. Later full eight-body turns have 98 record operations and 56 BASIC reads, with projection work reflecting actual suffix replacement.
- Exact observed record acquisition admits **five** operations and refuses **four** before effects. Absolute acquisition exhaustion produces no new rows/checkpoint/protection advancement.
- A separate fully retained 401-height fork with actual processed height 23 and ancestor 390 explicitly refuses the existing source budget before preview, mempool, progress, checkpoint or locks change. This records the real finite policy; arbitrary retained-fork or unlimited-lag liveness is not claimed.

Preparation time includes existing consensus staging; acceptance time includes existing preview/history and mempool paths. New index preparation components and actual BASIC reads are separately observed, but isolated storage-rewind timing and a completed-rewind ledger readout are not invented. The existing guarded publisher independently admits completion under its storage cap. No whole-reorg constant-work, constant-memory, hard-latency, archive/public-mainnet or funds claim follows from these small fixtures.

## Narrow Root-Approved Instrumentation

Only cfg(test) immutable observations were added to existing production source files:

```rust
PreparedChainstateReorg::maybe_basic_filter_component_work(
    &self,
) -> Option<[TurnWork; 4]> // storage, native sources, facts, checked total

ManagedNetworkHandle<FjallChainstateStore, FjallCoinsView>::
    measure_reorg_preparation_for_test(...)
    -> Result<([TurnWork; 4], Duration), ManagedNetworkAuthorityError>

ManagedNetworkHandle<...>::maybe_basic_index_accepted_target_for_test(...)
    -> Result<Option<AcceptedIndexTarget>, ManagedNetworkAuthorityError>

ManagedNetworkHandle<...>::reorg_with_mempool_failure_for_test(...)
    -> Result<ChainTransition, ManagedNetworkAuthorityError>
```

The existing serialized read wrapper performs genuine preparation, reads Copy work observations and drops the stage/capabilities before returning. The mempool wrapper invokes the existing private guard through the real production caller. No authority constructor, owner injection, public visibility widening, skipped validation, completed-publication timing plumbing or persistent observation field exists in non-test builds. The component getter impl remains in filter_reorg.rs to avoid growing the 627-line chainstate parent.

## Verification Evidence

All Cargo commands used Bun 1.3.9's timing/cooperative-lock runner and Rust 1.94.1. Cargo target access was serialized; quiet sessions were polled with advancing process/CPU or output evidence. No command was terminated, no host restart was assumed, and no host security/cache or foreign process was changed.

| Check / timing key | Actual result |
| --- | --- |
| phase158-fork-failures-ready | Compile exit 101: missing fault enum import, corrected |
| phase158-fork-failures-behavior | 3 passed / 2 setup failures: active lookup correctly masks physical rows; changed inspection to post-drop raw keys |
| phase158-fork-failures-expanded | 7 passed / 1 assertion mismatch: reopened core emits earlier precise missing-undo category; corrected assertion |
| phase158-reorg-matrix-initial | Compile exit 101: ambiguous integer type in harness arithmetic; annotated usize |
| phase158-reorg-matrix-ready | 11 passed / 2 fixture refusals / 1 timing experiment ignored; unbounded shared-gap setup exceeded existing source caps |
| phase158-reorg-protection-ready | 3 passed, zero failed/ignored, 247.36s |
| phase158-reorg-measure | **1 passed, zero failed/ignored, 295.15s; 54 configurations / 149 turns** |
| phase158-reorg-allphase-final | **98 passed, zero failed, 1 explicitly executed timing experiment ignored, 319.62s** |
| phase158-fork-failures-final | **8 passed, zero failed/ignored, 27.69s**, after exact BASIC-prefix preservation strengthening |
| phase158-reorg-protection-final | **4 passed, zero failed/ignored, 129.37s**, after focused automatic assertions |
| phase158-reorg-style-final | Strict node all-target/all-feature Clippy `-D warnings` passed, 6.93s |
| phase158-reorg-format-final | Workspace Cargo fmt check passed |
| phase158-reorg-style-optional-final | Final source after optional-local naming cleanup: strict Clippy passed, 8.29s; no suppression |
| Final Bright Builds all / breadcrumbs | Zero findings; 997 tracked Rust files passed, three new files manually checked against exact registry entries/comments/pinned anchors |

The broad 98-test run precedes the final two scoped behavioral refinements; each refined fault/protection module was then rerun successfully. The later optional-local naming cleanup is lexical only and final formatting/strict Clippy passed. Required test discovery is nonzero, all new behaviors execute normally, and the single ignored timing experiment was explicitly earned.

Tests were written against behavior implemented by Plans 01–04. Initial failures were genuine fixture/API/setup mismatches, not evidence of a new production bug. No deliberately broken production implementation or artificial failing assertion was introduced to manufacture RED.

## Deviations, Simplification and Limits

1. **Root-approved test-only observations:** Added immutable component and explicit accepted-target reads plus the existing private mempool fault wrapper in two existing source files. Production authority/caps/behavior remain unchanged.
2. **[Rule 3 - Fixture integration] Masked row inspection:** Replaced incorrect active lookup of deliberately hidden physical rows with exact post-drop raw inspection.
3. **[Rule 3 - Fixture integration] Precise earlier core refusal:** Accepted only the actual reopened missing-undo core category rather than demanding a later native-preflight label.
4. **[Rule 3 - Fixture integration] Bounded prune shared gap:** Preserved existing source caps and changed positive fixtures to admitted bounded gaps. Retained the original fully retained over-budget shape as a required refusal test.
5. **Simplification:** Reused one genuine fixture/reopen path, existing publication and mempool seams, existing caps and one ordinary append owner. The automatic proof focuses on its necessary invariants instead of repeating full-prefix ancestry validation.

No authentication gate, unresolved implementation bug, known stub or new unmodeled production trust boundary was found. T-158-06-01/02/03/04 have actual refusal/acceptance/recovery, prune ownership, complete accounting and bounded diagnostic evidence. Source/history complexity and internal retained-fork resource refusal are explicit residual limits. **Plan 07 must record that resource-policy difference/limit in parity documentation**, retain historical Phase 157 numbers, and follow the moved production_budget and ownership/proofs source paths.

Material guidance came from repo AGENTS/Repo-Local Guidance, the Bright Builds sidecar, placeholder-only overrides, architecture/code-shape/testing/verification/Rust standards, phase decisions, predecessor summaries and concrete fixture/authority/store contracts. Both active lesson files were fully loaded: 7,188 bytes / 2,397 conservative estimated tokens; no new audit or user-correction trigger occurred. Pinned Knots remains the behavioral authority; [BIP157 filter headers](https://bips.dev/157/) support ancestry-dependent commitments.

## Task Commits and Remaining Root Gates

Task and summary commits are **deferred to the root strict wrapper**. No staging, commit, push, worktree or hook bypass occurred. Root owns shared planning state and CFIX-03 activation. Full default native coverage/build/tests/Bazel, independent source/security review, formal lifecycle validation and final Git remain future root/Plan 07 gates. Local evidence does not claim full-phase completion or serving/operator scope from Phases 159–162.

## Self-Check: PASSED

All four created implementation/evidence paths, five shared modified implementation/registry paths and this summary exist. Every new Rust module has exact registered pinned breadcrumbs. Files stay below 628 lines, the summary has only its two top frontmatter delimiters, the originating lifecycle is preserved, and requirements-completed remains empty. Actual broad and focused behavior receipts, explicitly executed measurement evidence, final source formatting/strict Clippy and policy/provenance checks support the claims. Commit existence, shared-state updates, full native verification and activation remain intentionally root-owned.
