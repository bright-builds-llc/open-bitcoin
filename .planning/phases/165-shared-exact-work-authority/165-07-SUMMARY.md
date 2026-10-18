---
phase: 165-shared-exact-work-authority
plan: "07"
subsystem: chainstate
tags: [proof-work, earned-lineage, stage-provenance, complete-coins-view, fixture-transfer]
requires:
  - phase: 165-06
    provides: Selected-header ancestry and checked-body eligibility independent of coins acceptance
provides:
  - Internally derived live child work from privately accepted parent lineage
  - Private connect/reorg stages bound to owner, source tip and single-use generation
  - Exclusive validated absorption capabilities and single authorized reorg preview
  - Raw, mechanical, preview and mutable-coins-tainted origins distinguished from live work
  - Complete coherent coins snapshots and consuming genuine-owner transfer
  - Closed persistence surface preserving live authority without mutable coins escape
affects: [165-08, 165-09, 165-12, 165-13, 165-14]
tech-stack:
  added: []
  patterns: [Private allocation identity, generation token rotation, exclusive owner borrow, fail-closed full snapshots]
key-files:
  created:
    - packages/open-bitcoin-chainstate/src/engine/work_authority.rs
    - packages/open-bitcoin-chainstate/src/engine/tests/phase165_tests.rs
    - packages/open-bitcoin-chainstate/src/engine/tests/phase165_rebind.rs
    - packages/open-bitcoin-node/src/chainstate/live_fixture.rs
    - packages/open-bitcoin-node/src/network/runtime_authority/live_fixture.rs
    - packages/open-bitcoin-node/src/network/tests/phase165_rebind.rs
  modified:
    - packages/open-bitcoin-chainstate/src/engine.rs
    - packages/open-bitcoin-chainstate/src/engine/stage.rs
    - packages/open-bitcoin-chainstate/src/engine/overlay_apply.rs
    - packages/open-bitcoin-chainstate/src/coins.rs
    - packages/open-bitcoin-chainstate/src/coins/cache.rs
    - packages/open-bitcoin-chainstate/src/coins/memory.rs
    - packages/open-bitcoin-chainstate/src/error.rs
    - packages/open-bitcoin-node/src/chainstate.rs
    - packages/open-bitcoin-node/src/chainstate/validation_history.rs
    - packages/open-bitcoin-node/src/chainstate/flush_lifecycle.rs
    - packages/open-bitcoin-node/src/storage/coins_view.rs
    - packages/open-bitcoin-node/src/sync/tests/support_runtime.rs
    - docs/parity/source-breadcrumbs.json
    - scripts/check-phase164-canonical-identity/contracts.ts
    - scripts/check-phase164-canonical-identity.test.ts
key-decisions:
  - "Numerical ProofWork, raw metadata and BASIC receipts cannot construct live work lineage."
  - "Independent clones receive fresh owner and generation allocations; held stages prevent allocation reuse from forging identity."
  - "Validate and exclusively borrow the actual owner before infallible absorption."
  - "Unrestricted coins mutation taints lineage; ordinary persistence uses a closed current-B/flush/sync interface."
  - "Only consuming genuine live authority plus complete coherent state equality can rebind a coins view."
  - "Fixture-enriched confirmation counts plus actual body-prefix revalidation do not qualify ordinary production recovery."
requirements-completed: ["CWOR-03"]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 165-2026-10-14T16-09-59Z
generated_at: 2026-10-15T04:03:11Z
duration: 54min
completed: 2026-10-15
---

# Phase 165 Plan 07: Earned Parent Lineage and Sealed Stages Summary

**Live connection derives exact parent-plus-target work, and every stage requires its actual owner and generation before effects; complete coins equality permits transfer only from an already genuine live owner.**

## Performance

- Tasks: 3/3, including the parent-approved, formally added transfer task.
- Ownership: 39 approved source/test/checker/breadcrumb paths plus this summary; six new Rust files.
- First recorded RED run: 2026-10-15T03:09:36.637Z. Context loading was not separately timed.
- Summary created: 2026-10-15T04:03:11Z; approximately 54min from the first recorded run.

## Accomplishments and APIs

- Live connection validates actual block/header/context and transaction effects, checks the private accepted parent lineage and selected parameters, and computes parent work wrapping_add(block_proof(actual bits)). Maximum caller work cannot replace the result. Reorg suffixes use the same derivation. Raw snapshot, parent and coins-cache constructors retain Untrusted origin even when every visible number, hash and coins B agrees.
- Explicit new/from_snapshot/from_parent/from_coins_cache_for_mechanical_fixture boundaries preserve historical transaction and synthetic-score exercises. Their owner and stage has_earned_work facts remain false after connection, absorption and independent cloning. They provide no canonical or ordinary authority.
- Both staged types have private fields. Connect consumers borrow position(), undo() and BASIC inputs. Neither staged type is Clone. Private Arc allocation identity binds owner and generation; binding also captures the complete source tip. Independent live clones preserve private validated facts but receive fresh identities. Preview clones are untrusted.
- commit_staged_connect/reorg and the Result-returning absorb_staged\_\* wrappers reject foreign/stale input before cache changes. validate_staged_connect/reorg consumes the stage and returns a private-field capability holding an exclusive mutable owner borrow; capability.absorb() remains infallible. Rust prevents intervening owner mutation after validation.
- A reorg preview is allowed exactly once on its original owner/generation. Only its own sealed continuation can subsequently absorb it. Pending preview does not report earned work, start another live stage or grant clone authority. Any intervening mutation abandons preview authority permanently; merely clearing the preview marker cannot certify the installed candidate. Node checks the preview-specific binding before BASIC freeze.
- validate_live_origin() is a read-only check of actual private origin. Genuine owned-empty state returns Ok while has_earned_work() remains false until body acceptance. Raw empty imports, mechanical roots, previews and mutable-coins-tainted owners refuse. This distinction is required by Plan08; tip absence alone proves nothing.
- Public coins_mut() invalidates stages and work provenance. The CoinsFlush trait exposes metrics, preparation of the existing B, flush and sync, with no arbitrary coin mutation, caller-selected B or mutable callback. Existing node persistence uses this closed surface. Pending preview cannot flush, while benign persistence preserves live lineage and rotates stage generation.
- CoinsView::complete_snapshot() returns all effective UTXOs, B and heads; its default explicitly refuses unsupported views. Memory implements it exactly, cache merging includes tombstones and B overrides, and Fjall reads all rows and markers from one database snapshot. collect_unspent_hint is never completeness evidence.
- rebind_into consumes a genuine live source, compares complete source/target coins and endpoint, empty heads, active positions, undo and counts before any target mutation, then carries only genuine source metadata/parameters and installs fresh owner/generation. Only pristine genuinely empty states normalize absent versus known-empty confirmation counts. Target cache flags are preserved; old source and target stages refuse afterward.
- The two small Node live_fixture child modules are cfg-test owned. Managed adoption checks the actual concrete Fjall parent, current physical metadata/undo/counts and complete coins under the existing publication guard. The runtime delegate additionally requires Fixture scope, holds the actual authority mutex and rotates its incarnation. A canonical handle cannot use this route. BASIC acceptance remains separate from work lineage.

## Fixture Recovery Evidence Boundary

Production open_configured remains untrusted, and the production metadata writer still stores unknown confirmation counts. The affected restart fixture exposed both a missing retained genesis body and deliberately unavailable persisted counts. Named fixture initialization now retains the actual accepted genesis body. Its force-flush helper can enrich only missing counts captured from an actual live owner after checking unchanged physical positions, complete coins, B and empty heads under the publication guard. Matching existing counts are idempotent; conflicting counts, raw sources and unflushed state refuse. Only the counts-bearing fixture metadata row is written through the existing test API.

Reopening still requires actual retained active bodies to be fully revalidated in a new core owner and strict complete identity checks before transfer. Missing history remains untrusted. This is **fixture-enriched counts plus actual body-prefix revalidation**, not ordinary production recovery or Phase166 qualification. Original backend-handle closure, no-duplicate-getdata and durable branch assertions remain. No detached authority is presented as process or all-authority reopening.

## Task Commits

Task, RED/GREEN and metadata commits are deferred to the parent strict finalization after whole-phase native verification, independent review/security/goal checks and normal hooks. This executor staged, committed and pushed nothing; shared STATE, ROADMAP, REQUIREMENTS, config, hooks and todo files were not edited. No hashes or qualification receipts are invented.

## Executed Evidence

All Cargo commands ran sequentially through command-timings with pinned Rust1.94.1. Every Bun/Cargo command used command-local Bun1.3.9:

```bash
PATH='/Users/peterryszkiewicz/.bun/install/cache/@oven/bun-darwin-aarch64@1.3.9@@@1/bin:'"$PATH"
```

| Check                                                        | Actual result                                                                                                                                                                                                                                                                                                                                                                                                                                |
| ------------------------------------------------------------ | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| phase165-stage-authority-red                                 | Three discovered tests failed, exit101: maximum caller work was retained, a raw snapshot could stage a child, and an equal-looking foreign owner could commit a stage. First expanded GREEN11/11; final stage corpus has18 cases.                                                                                                                                                                                                            |
| phase165-live-rebind-red                                     | Two discovered cases: genuine-empty transfer failed, raw refusal passed; exit101. First GREEN2/2; final transfer corpus has19 core cases.                                                                                                                                                                                                                                                                                                    |
| phase165-managed-live-rebind-red                             | One discovered actual managed positive failed, exit101. Final phase165-managed-live-rebind-final:14/14 passed, zero failed/ignored,12.02s execution.                                                                                                                                                                                                                                                                                         |
| Core regression diagnostics                                  | Initial336/340 unit tests passed; three fixtures changed selected parameters after genesis and one assumed caller scores. Consistent genesis parameters and explicitly mechanical score fixtures preserved their original transaction assertions. Then340 unit tests passed; two parity fixtures required the equivalent corrections.                                                                                                        |
| Affected Node diagnostics                                    | Initial104/152 passed; imported factory state correctly lacked authority. Actual live transfer/prefix validation raised this to151/152. The remaining restart case first lacked retained genesis bytes, then exposed Some(actual counts) versus persisted None. The explicit fixture-data changes above resolved both without relaxing guards. Final combined phase165-stage-affected-node-final:152/152 passed, zero failed/ignored,71.72s. |
| Parallel fixture diagnostic                                  | Expansion to14 managed cases exposed four temporary-directory timestamp collisions, not authority failures. A test-only atomic suffix fixes uniqueness; the final14/14 receipt above includes parallel execution. No production global owner counter was added.                                                                                                                                                                              |
| phase165-stage-core-coverage-final                           | Final366 unit plus3 parity tests passed. No handwritten Uncovered Lines reported. Work authority231/231, cache385/385, stage249/249, engine420/420 lines all100%. Literal whole-crate line coverage99.94% is preserved as reported, not relabeled100%. No exclusions or ignored tests added. Stale cross-crate instrumented profiles were explicitly cleaned before final coverage.                                                          |
| phase165-stage-clippy-final                                  | Chainstate and Node --all-targets --all-features -- -D warnings passed,19.50s. This also compiles affected consumers and test targets.                                                                                                                                                                                                                                                                                                       |
| phase165-stage-doctests-final                                | All21 passed:2 regular and19 compile-fail,13.18s. Covers field/privacy construction, mutation, replay and exclusive owner borrowing.                                                                                                                                                                                                                                                                                                         |
| cargo fmt --manifest-path packages/Cargo.toml --all          | Passed after final source changes.                                                                                                                                                                                                                                                                                                                                                                                                           |
| bun test ./scripts/check-phase164-canonical-identity.test.ts | Full509 passed, zero failed,633 expectations,163.06s. Earlier focused103/103 passed. One intermediate control search needed its formatted multiline spelling corrected; it did not indicate an owner-guard failure.                                                                                                                                                                                                                          |
| bun run scripts/check-phase164-canonical-identity.ts         | Passed:115 fixed Rust inputs and342 reviewed edges. Reviewed current additions total14 inputs/64 edges; frozen historical tooling, manifests, snapshots and receipts were preserved.                                                                                                                                                                                                                                                         |
| Breadcrumbs / managed checks                                 | Breadcrumbs passed for1,177 Rust files, including all six new files. Managed checks passed with1,521 scanned files, zero exceptions/findings. All39 owned paths exist and every owned Rust file is at most628 lines.                                                                                                                                                                                                                         |
| Panic policy / diff / stub review                            | Native panic-site policy passed during stage work; final strict all-target clippy also passes. Owned diff and git diff --check passed; the index remains empty. No goal-blocking stub, new dependency, endpoint or undeclared production schema surface was found.                                                                                                                                                                           |

Whole-workspace native/Bazel acceptance remains Plan14 and the parent; this bounded execution does not qualify the whole workspace.

## Decisions and Simplification

AGENTS.md, the Bright Builds sidecar, placeholder-only overrides, architecture/code-shape/verification/testing/Rust/TypeScript standards and both complete active lesson files informed execution. Active lessons totaled7,188 bytes and2,397 conservative estimated tokens. The parent owned initial sync/tool bootstrap; no project skill directory was present.

The simplification pass reused the existing ProofWork arithmetic, replaced numeric owner counters with private held allocation identities, and used exclusive borrows rather than a fallible post-validation commit window. Full coins facts are one coherent value, separate from admission hints. Closed persistence avoids a second accepted-UTXO commitment/map. Small cfg-test adapter modules keep owner files below the managed cap without an exception.

## Deviations from Plan

1. **[Rule3 - Blocking; parent approved] Sealed-field consumers and historical fixtures.** The parent expanded the exact inventory before edits. BASIC consumers now use borrowed accessors, absorption callers handle explicit pre-effect refusal, and old synthetic-score tests opt into named mechanical roots. Difficulty/BIP30/maturity fixtures use consistent selected genesis parameters. No substantive UTXO/undo assertion was removed.
1. **[Rule2 - Correctness; parent formalized Task3] Genuine fixture authority transfer.** Strict raw-origin rejection exposed historical factories that reconstructed active owners from raw caches. Complete snapshots, consuming live transfer and actual serialized adapter checks preserve those tests without a mechanical managed-authority exception or raw acceptance shortcut.
1. **[Rule2 - Correctness; parent approved] Mutable coins escape.** Public arbitrary cache mutation could otherwise forge a live transfer source. It now taints provenance; normal persistence uses the closed interface and retains its native behavior.
1. **[Rule3 - Blocking; parent approved] Explicit fixture data for revalidation.** Retained genesis bytes and missing-count enrichment address concrete restart diagnostics. Production persistence, unknown/conflicting count refusal and ordinary recovery boundaries stay intact.
1. **[Rule3 - Blocking; parent approved] Caps and current source controls.** Two cfg-test child modules keep existing owners under628 lines. New actual guards and mutation controls were added after source review; historical receipts were not renewed to imply current qualification.

## Deferred Issues and Plan08 Handoff

- Use actual chainstate.validate_live_origin() before prepare/preview/mempool or publication effects. Genuine owned-empty Live state is distinct from raw empty state even though both has_earned_work values are false. Never authorize from tip absence, imported arithmetic, coins B, a BASIC receipt, peer registry facts or a mechanical origin.
- Stages and receipts do not replace Plan08's actual owner work/preference/body-eligibility checks. Validate actual cumulative work and retained candidate eligibility before preview. Core mechanical fixture staging intentionally remains available for transaction tests and must never become a managed authority exception.
- Call check_staged_reorg_preview before any BASIC freeze; check_staged_reorg permits only original state or the exact authorized single preview for final absorption. Use validate_staged\_\*()?.absorb() for the exclusive, infallible accepted window. Public absorb wrappers return Result and refuse stale input before effects.
- Direct reorg failed-prefix attribution remains Plan08: identify the actual failed replacement, or conservatively leave its suffix unavailable. UntrustedWorkLineage, WorkParametersMismatch, StageOwnerMismatch, StaleStage, CompleteCoinsSnapshotUnavailable, WorkRebindMismatch and storage refusals are not consensus-invalid body facts.
- Plan09 still owns fresh checked readmission of cached pre-header bodies and enumeration of all eligible retained side candidates.
- Phase166 owns canonical bootstrap/exact checkpoint/all-handle reopen/genesis BASIC, Phase167 contextual completeness and Phase168 actual pinned-daemon qualification. The four ordinary selector support/refusal classes and default-off networking are unchanged. Parent Plan13 owns consolidated contributor-facing status updates.

## Self-Check: PASSED

All39 approved paths and all six new mapped Rust files exist. Final core, managed transfer, combined affected Node, doctest, strict lint, full509-control regression, current boundary, breadcrumb, managed-cap and diff checks have actual results above. New pure handwritten paths are covered with no exclusions. Stub/threat review found no unimplemented goal or unregistered production trust surface. No executor command remains active; the Git index is empty. Commits and shared state/tracker updates remain intentionally deferred to the authorized parent wrapper. Ready for165-08.
