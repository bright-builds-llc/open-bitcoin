---
phase: 158-validated-reorg-and-retained-branch-identity
plan: "05"
subsystem: testing
tags: [rust, fjall, compact-filters, validated-reorg, standard-maturity, retained-history]
requires:
  - phase: 158-04
    provides: Production retained-input preflight and ordinary accepted-position scheduled catch-up
provides:
  - Continuous production fork matrix with exact independently staged expected commitments
  - Separate unchanged runtime maturity-100 spending fork with ordinary flush and real reopen
  - Immutable displaced history, actual paired shared-source pruning and continuous pre-prune startup refusal evidence
affects: [158-06, 158-07]
tech-stack:
  added: []
  patterns: [separate genuine staged oracle, ordinary serialized authority, complete handle-drop reopen]
key-files:
  created:
    - packages/open-bitcoin-node/src/sync/tests/filter_index/reorg.rs
    - packages/open-bitcoin-node/src/sync/tests/filter_index/reorg/fixtures.rs
    - packages/open-bitcoin-node/src/sync/tests/filter_index/reorg/branches.rs
    - packages/open-bitcoin-node/src/sync/tests/filter_index/reorg/retention.rs
  modified:
    - packages/open-bitcoin-node/src/sync/tests/filter_index.rs
    - docs/parity/source-breadcrumbs.json
key-decisions:
  - Observe the configured startup turn instead of injecting lagging progress
  - Discard the separately validated oracle stage before production network reorg
  - Preserve compact maturity-one fixtures while separately passing unchanged actual runtime parameters and flags
  - Earn durability through ordinary own flush and drop every live handle before actual Fjall reopen
  - Distinguish pruned shared-prefix retention from mandatory disconnected consensus sources
requirements-addressed: [CFIX-03]
requirements-completed: []
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 158-2026-10-08T02-17-15
generated_at: "2026-10-09T02:56:38Z"
duration: 16min
completed: 2026-10-09
---

# Phase 158 Plan 05: Continuous Validated Fork and Durable Retention Summary

**Production reorgs replace exact ancestor-bound BASIC commitments, preserve displaced hash records through real reopen, and pass a separate unchanged maturity-100 spending fork.**

## Performance and Scope

- Tasks: 2/2 implemented and locally verified, including the root's standard-maturity amendment.
- Approximately sixteen minutes after the root's 02:41 UTC Plan 05 dispatch through final verification and this summary. Dates are 2026-10-08 CDT / 2026-10-09 UTC; elapsed execution time is not a reorg latency measurement.
- Four new Rust modules, one parent module registration and four additions to the existing breadcrumb group; this summary is the seventh owned artifact. Existing predecessor registry entries and other agents' changes were preserved.
- Baseline HEAD remains `2f21ac2052208c7e7a084ed006d908c5ccb714aa`. No Git staging, commit, push, hook bypass, worktree, state update or requirement activation occurred.
- This plan changes test fixtures/evidence only. No production dependency, endpoint, schema, authority factory, receipt constructor or test-only progress injection was introduced.

## Accomplishments

- Sixteen named `phase158_validated_reorg` tests exercise concrete Fjall coins/storage and the normal configured Enabled runtime. The equal-height cases seed zero/eight/twenty-four records and use actual post-startup processed heights 7/15/23 around ancestor 10. Startup's own eight-block turn is observed, never replaced with injected progress.
- Equal-height, longer, shorter and disconnect-only replacements verify exact accepted height/hash, renewed generation/branch identity, lag, retained initial-sync latch and common processed prefix. Full disconnect publishes Empty with no fabricated accepted target or nonempty turn. Catch-up runs through parameter-free ordinary scheduled turns without peer input, reopen or re-enable to resume live work.
- A genuine independent oracle stages the full fork from the current validated snapshot, borrows its accepted positions and generated undo, checks the exact two known historical/same-block spent scripts, and generates expected records from the common predecessor header. The stage is explicitly dropped before `network.reorg_to_branch`; no oracle receipt or manager state is installed. Complete record equality checks bytes, block/parent identity, filter hash, filter header and predecessor header, and the replacement tip header differs from the displaced tip.
- Stored replacement bodies precede the existing production reorg caller, matching sync reception's ownership contract. Live inbound Version/Verack/Headers processing followed by `block_reconcile::reconcile_best_chain` independently proves the actual sync route. Its later normal turns complete the same exact expected branch.
- A→B→A restores the original immutable commitments, retains B's records and accepts a subsequent genuinely stored child through `connect_stored_block`. An actually acquired budgeted old append preparation is refused after replacement.
- Replacement acceptance and completed catch-up leave the old durable coins endpoint in place; safe progress remains at or below the common ancestor. Normal `flush_coins(Always, ...)` reports an actual coins write, and the following ordinary turn earns the replacement checkpoint. No forced reorg-specific checkpoint shortcut was added.
- Reopen evidence consumes the fixture, explicitly drops its runtime (including manager, store and announcement closure handles), then opens the same Fjall path and normal configured Enabled runtime. No store clone or publication proof is retained. Saved actual old rows remain byte-for-byte equal while active-height lookup returns the replacement and masks the old suffix. Operator locks and the reserved BASIC lock survive reopen; operator reserved-lock clearing still refuses.
- A continuous 401-height compact history earns actual ordinary paired body/undo deletion at shared height 20, outside the keep window. Forks at 390 and return to A succeed while this shared pair remains absent. The return's ordinary turn reuses immutable suffix records with positive `reused_records` and zero generations. After own flush and actual reopen the pruned shared filter and displaced B rows remain exact. This proves no blanket shared-history regeneration prerequisite; it does not claim the pruned shared row itself was selected as new turn work.
- Separate missing-body and missing-undo cases remove an actual required disconnected source despite existing immutable filters. Production reorg refuses explicitly before preview; the validated snapshot, progress, lock map and every immutable record remain unchanged. These named-row loss cases are Unavailable evidence, distinct from the real paired-prune case.
- A separate continuous 401-height startup case has safe prefix 15 and reserved lock from 16. Interrupted prune intent 20 is outside the keep window but still required by BASIC. After all runtime handles are dropped, real configured startup refuses before deletion; complete index key/value equality, body/undo, live intent, reserved lock and unchanged `have_pruned` prove ordering.

## Unchanged Standard-Maturity Runtime Case

`ForkFixture::standard` reads the actual configured runtime's consensus parameters and verification flags from an empty real store, drops that observer, then validates genesis and heights 1–100 as coinbase-only. Height 101 spends the height-1 reward at maturity 100 and spends its output in the same block. Heights 102–104 spend the preceding non-coinbase final output and a same-block child. Every block has a valid height-prefixed reward, correct merkle root, mined easy regtest PoW and executable OP_TRUE scripts.

The actual production runtime is opened with those validated coins and complete retained bodies/undo. Its unchanged `runtime.consensus_params` and `runtime.verify_flags` are passed directly to both the separate oracle and production reorg. A branch-distinct fork at 100 reaches 106, catches up through ordinary turns, earns its own normal flush, drops all handles and passes actual configured Fjall reopen. Displaced heights 101–104 remain exact by hash. No maturity-one parameter helper or mutation is used in this case; reopened maturity remains 100 and actual verification flags remain P2SH.

All heights stay below 150. The current runtime's halving interval is 210,000 while pinned Knots regtest uses 150; this fixture proves unchanged runtime validation and standard maturity, **not full Knots regtest activation/parameter parity**. Compact branch/prune cases remain explicitly maturity one for short inexpensive histories.

## Reusable Fixture Contract

The existing `TurnHistory`/`ValidatedHistory` types remain unchanged. The new test-only fixture exposes:

```rust
ForkFixture::compact(prefix: usize, suffix: usize, indexed: usize) -> Self
ForkFixture::standard() -> Self
ForkFixture::fork(&self, ancestor: usize, count: usize, tag: u8) -> ForkBranch
ForkFixture::apply(&self, branch: &ForkBranch)
ForkFixture::finish(&self)
ForkFixture::flush(&self)
ForkFixture::reopen(self) -> Self
ForkFixture::cleanup(self)
```

`history` carries recipes/expected rows and `runtime` owns actual production authority. `ForkBranch` carries disconnected bodies, anchored replacements and expected records, never a stage or permission. Current `fork` recipes use an ancestor in the original canonical common prefix; Plan 06 should preserve that contract or explicitly extend the oracle for an ancestor within a replacement branch. Generate the branch before removing required inputs so the oracle is genuine and loss is tested at the production preflight boundary. `apply` saves replacement bodies before calling the ordinary network reorg; it never supplies manufactured undo or owner progress.

## Verification Evidence

All Cargo commands used Bun 1.3.9 through `scripts/command-timings.ts`, its cooperative target lock and Rust 1.94.1. Cargo access was serialized, resumable sessions were polled, and no command was terminated or treated as timed out. Host execution entered Rust normally; no restart/cause inference or security/cache change was made.

| Check / timing key | Actual result |
| --- | --- |
| Initial compile / phase158-valid-forks-compile | Exit 101: fixture Vec type inference and one unused import, corrected |
| Initial behavior / phase158-valid-forks-behavior | 1 passed / 9 failed: direct caller fixtures omitted retained replacement bodies; stale fixture used unbudgeted acquisition |
| phase158-valid-forks-ready | 10 passed, zero failed/ignored, 9.95s harness |
| phase158-valid-reopen-prune | 13 passed, zero failed/ignored, 38.51s harness |
| phase158-valid-forks-complete | 13 passed / 1 failed, 34.22s; test peer default nonce triggered expected SelfConnection refusal |
| phase158-valid-sync-route | Correct distinct peer nonce; 1 passed, zero failed/ignored, 1.44s harness |
| phase158-valid-forks-final | First 15 named tests passed, zero failed/ignored, 34.45s harness |
| phase158-valid-startup-order | Added continuous startup-order case passed, zero failed/ignored, 8.38s harness |
| Final phase158-valid-forks-allphase | **82 passed, zero failed/ignored**, 72.78s harness; includes all sixteen new tests and all 66 predecessor node cases |
| Final phase158-valid-forks-startup-format | Cargo workspace format passed |
| Final phase158-valid-forks-style-final | Strict node all-target/all-feature Clippy `-D warnings` passed, 4.49s; no suppression |
| Bright Builds all / breadcrumb --check / diff --check | Zero findings; 997 tracked Rust files verified; diff check clean |
| Manual new-file provenance and review | All four exact manifest entries/comments/pinned paths verified; files 20/419/375/333 lines, below 628 |

The new tests describe existing production behavior implemented in Plans 01–04. Recorded failures were fixture-contract/setup failures, not a claim that Plan 05 fixed production behavior. No intentionally broken production code or fake RED test was introduced. The final broader run verifies all new cases on the final formatted source.

## Deviations, Simplification and Limits

1. **Root amendment: separate unchanged standard-maturity evidence.** Added a dedicated constructor and fork case while preserving the existing compact fixtures and their honest parameter limits.
2. **[Rule 3 - Fixture integration] Match real retained-body and budget contracts.** Saved actual replacement bodies before direct reorg and acquired actual budgeted stale work. The normal production caller and capability checks were preserved.
3. **[Rule 3 - Fixture integration] Distinct live peer nonce.** Corrected the synthetic peer's self-connection setup; the normal handshake check remained intact.

The simplification pass keeps one small shared fixture module, separates branch and retention tests, uses the existing generator and native authority APIs, and shares no production history cache, authority factory or duplicate worker. No production code change was necessary. The test-only 256-turn termination guard bounds the small fixture driver; it is not a new production resource policy. Plan 06 still owns integrated fault/prune/resource measurements and numeric-limit calibration.

Threat mitigations T-158-05-01/02/03 have exact script/commitment, all-handle drop/reopen and genuine prune/source-refusal/startup-order evidence. No known production stub or new unmodeled endpoint/schema/file-access boundary was introduced. Direct hash lookup here is internal store behavior; external filter RPC/P2P remains deferred to Phases 159/160. Full native coverage/build/Bazel, independent source/security review, formal lifecycle validation and CFIX-03 activation remain root gates. No production, public-default, archive-scale or funds claim is made.

## Guidance and Task Commits

Material guidance came from repo AGENTS/Repo-Local Guidance, the Bright Builds sidecar, placeholder-only overrides, architecture/code-shape/testing/verification/Rust standards, canonical phase contexts and milestone references, actual predecessor summaries/source routes and pinned Knots rewind/retention/undo contracts. Both active lesson files were fully loaded: 7,188 bytes / 2,397 conservative estimated tokens. No new audit trigger, explicit user correction or authentication gate occurred.

Both task commits and summary metadata finalization remain **deferred to the root strict phase wrapper**. Root owns STATE.md, ROADMAP.md, todo, REQUIREMENTS.md and all Git actions. New files are intentionally untracked under the no-staging instruction; root must rerun the tracked-file breadcrumb check after its permitted staging/finalization step. No nonexistent commit or full-native result is claimed.

## Self-Check: PASSED

All six implementation/registry paths and this summary exist. Every new module has exact registered breadcrumbs resolving to the pinned source. The summary has only its two opening/closing frontmatter delimiters, preserves the originating lifecycle and leaves requirements-completed empty. Final formatted-source verification passed all 82 Phase 158 node tests, including the sixteen new cases, and strict all-target/all-feature Clippy. Policy/provenance/scoped diff checks passed and HEAD is unchanged. Commit-existence and state updates remain intentionally root-deferred.
