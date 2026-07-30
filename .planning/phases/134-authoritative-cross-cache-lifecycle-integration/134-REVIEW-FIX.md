---
phase: 134-authoritative-cross-cache-lifecycle-integration
fixed_at: 2026-07-30T10:48:57Z
review_path: .planning/phases/134-authoritative-cross-cache-lifecycle-integration/134-REVIEW.md
iteration: 7
findings_in_scope: 1
fixed: 1
skipped: 0
status: all_fixed
---

# Phase 134: Code Review Fix Report

**Fixed at:** 2026-07-30T10:48:57Z
**Source review:** `.planning/phases/134-authoritative-cross-cache-lifecycle-integration/134-REVIEW.md`
**Iteration:** 7 (post-`459797ea` confirmation)

**Summary:**

- Findings in current review scope: 1
- Fixed in this pass: 1
- Skipped in this pass: 0
- Cumulative findings fixed: 19
- Cumulative findings skipped: 0

## Fixed Issues

### WR-02: Local-item and function-pointer syntax boundaries remain incomplete

**Status:** fixed: requires human verification
**Files modified:** `docs/metrics/lines-of-code.md`, `scripts/check-phase134-apply-boundaries/rust-calls.ts`, `scripts/check-phase134-apply-boundaries/strict-syntax.ts`, `scripts/check-phase134-authoritative-lifecycle.test/apply-helpers/token-scanner-reachability.ts`
**Commit:** 709a42cc
**Applied fix:** Assignment analysis now precomputes the matched body-closing tokens of named local function items and treats only those closures as statement boundaries. A runtime assignment after a local generic helper can no longer inherit the helper's declaration exemption, while braced destructuring such as `let Self { ... } = self` remains a binding. Call scanning also classifies immediate bare `fn(...)` tokens as function-pointer type syntax rather than call-like syntax. Exact negative and positive fixtures cover the linked bypass, associated-type equality, the nearby pure binding, and a `type Callback = fn(u8)` alias.

## Cumulative Fix History

### Prior loop iteration 1 — CR-01: Block and reorg APIs can fail after chainstate has already committed

**Status:** fixed: requires human verification
**Commit:** 2b286626
**Applied fix:** Added non-mutating prepared chainstate transitions and made block and reorg paths complete fallible aggregate preparation before exposing chainstate or lifecycle mutations. Public-path failure tests cover atomic no-op behavior and retry convergence.

### Prior loop iteration 1 — WR-01: Orphan-policy limits reject valid whole-mempool lifecycle deltas

**Status:** fixed
**Commit:** df51d8a6
**Applied fix:** Removed orphan-policy work limits from authoritative lifecycle cleanup while retaining bounds on stored packages, fingerprints, orphan state, candidates, and per-peer state. Added large connected-block and full fingerprint-cache cleanup regressions.

### Prior loop iteration 2 — WR-01: Public sealed transition can overwrite a newer mempool revision

**Status:** fixed: requires human verification
**Commit:** bdddbee1
**Applied fix:** Replaced the forgeable seal/commit gap with an instance-bound, revision-checked, exclusive transaction operation and added stale same-instance plus cross-instance regression coverage.

### Prior loop iteration 3 — IN-01: Apply-boundary guard omits the connected-block transaction root

**Status:** fixed
**Commit:** c4457484
**Applied fix:** Extended the structural apply-boundary guard through the connected-block transaction root and both public block seams, with mutation tests for missing, reordered, bypassed, and newly fallible stages.

### Fresh bounded pass iteration 1 — WR-01: Aggregate-root helper calls bypass the atomic-boundary guard

**Status:** fixed
**Commit:** f8b2a2dc
**Applied fix:** Added interval-specific recursive free-function reachability with same-file, alias, and module resolution, fail-closed unresolved-call handling, transitive effect/mutation checks, and positive plus negative mutation controls.

### Post-confirmation iteration 2 — WR-01: Strict reachability ignores direct methods and allowlists mutating helpers

**Status:** fixed
**Commit:** b0230f11
**Applied fix:** Unified critical-slice and recursive-helper inspection across assignments, methods, and functions; separated read-only calls from structurally mutating methods; and added direct, reached, nested, and pure-control mutation fixtures.

### Post-confirmation iteration 2 — WR-02: Qualified calls can resolve to the wrong helper outside the source corpus

**Status:** fixed
**Commit:** b0230f11
**Applied fix:** Preserved canonical Rust module identities through extraction and required exact qualified resolution, with unscanned collision, unresolved path, and duplicate benign module controls.

### Post-confirmation iteration 3 — WR-01: Generic pure method names bypass repo-owned method traversal

**Status:** fixed
**Commit:** fe3f9bcb
**Applied fix:** Removed generic method-name purity and made extracted repo candidates authoritative unless an exact known receiver path applies. Unique repo methods are traversed and ambiguous names fail closed, with effectful `get`, `iter`, and `len` regressions plus pure repo and standard receiver controls.

### Post-confirmation iteration 3 — WR-02: Macro invocations and indexed assignments evade strict mutation analysis

**Status:** fixed
**Commit:** fe3f9bcb
**Applied fix:** Added conservative strict-slice syntax checks for macros, all assignment forms, mutable borrows, closures, async and unsafe blocks, and unresolved control flow, with direct and reached regression fixtures and no pure macro exemption.

### Post-confirmation iteration 4 — WR-01: Parenthesized receiver and turbofish calls bypass reachability

**Status:** fixed: requires human verification
**Commit:** 2b53f2e0
**Applied fix:** Added balanced token-aware call extraction with parenthesized receiver normalization, nested and qualified turbofish resolution, and fail-closed unknown call-like handling.

### Post-confirmation iteration 4 — WR-02: Exact receiver-name classifications can be spoofed

**Status:** fixed: requires human verification
**Commit:** 2b53f2e0
**Applied fix:** Replaced global local-name classifications with type, initializer, and exact function-local structural evidence before allowing standard-library receiver methods.

### Post-confirmation iteration 4 — WR-03: Strict syntax rejects valid pure Rust

**Status:** fixed: requires human verification
**Commit:** 2b53f2e0
**Applied fix:** Tokenized strict syntax and tracked delimiter depth so pure bit-or, immutable declarations, typed arrays, nested generics, and comment or literal contents no longer produce false positives.

### Post-confirmation iteration 5 — WR-01: Parenthesized functions and tuple-field methods bypass call scanning

**Status:** fixed: requires human verification
**Commit:** 10ecefdf
**Applied fix:** Added exact parenthesized function-path classification, fail-closed unknown parenthesized calls, tuple-aware numeric lexing, and numeric receiver projections.

### Post-confirmation iteration 5 — WR-02: Const-expression braces truncate extracted function bodies

**Status:** fixed: requires human verification
**Commit:** 10ecefdf
**Applied fix:** Added token-balanced signature scanning so nested const-expression braces cannot be selected as function bodies.

### Post-confirmation iteration 5 — WR-03: Nested type names can spoof pure receiver evidence

**Status:** fixed: requires human verification
**Commit:** 10ecefdf
**Applied fix:** Required the receiver parameter's outer type path to match `BTreeSet` exactly before applying standard-library purity.

### Post-confirmation iteration 5 — WR-04: Local type aliases are rejected as mutation

**Status:** fixed: requires human verification
**Commit:** 10ecefdf
**Applied fix:** Classified all equality tokens within local type-alias declarations as type syntax, including associated-type equality.

### Post-confirmation iteration 6 — WR-01: Textual receiver evidence allows effectful standard-name spoofs

**Status:** fixed: requires human verification
**Commit:** 459797ea
**Applied fix:** Removed broad textual receiver trust, retained only exact scoped production evidence, and required repo-owned same-name methods to traverse or fail closed.

### Post-confirmation iteration 6 — WR-02: Associated-type equality in local generic function signatures is rejected

**Status:** fixed: requires human verification
**Commit:** 459797ea
**Applied fix:** Treated equality inside local function signatures as declaration syntax and excluded generic function declarations from call extraction without weakening runtime assignment checks.

### Post-confirmation iteration 7 — WR-02: Local-item and function-pointer syntax boundaries remain incomplete

**Status:** fixed: requires human verification
**Commit:** 709a42cc
**Applied fix:** Closed assignment exemptions at matched local function-item boundaries and excluded bare function-pointer type parameter lists from call extraction.

## Verification

- Two linked post-`459797ea` outcomes reproduced before editing while all prior 246 tests passed: an assignment after a local generic function item incorrectly returned `[]`, and a pure `type Callback = fn(u8)` alias was rejected as an unclassified call.
- `bun test scripts/check-phase133-package-aware-download-orphan-bridge.test.ts` — 31 passed, 0 failed
- `bun test scripts/check-phase134-authoritative-lifecycle.test.ts` — 248 passed, 0 failed, 349 expectations
- `bun scripts/check-phase134-apply-boundaries.ts` — passed
- `bun scripts/check-phase134-authoritative-lifecycle.ts` — passed
- `bun scripts/check-phase133-package-aware-download-orphan-bridge.ts` — passed
- `bun scripts/bright-builds-check.ts all` — passed with zero findings
- `git diff --check` — passed
- `bash scripts/verify.sh` through `scripts/command-timings.ts` on final source state — passed in 17m 33.637s
- Normal pre-commit hook reran the full repository contract and completed for atomic commit `709a42cc`.

***

_Fixed: 2026-07-30T10:48:57Z_
_Fixer: the agent (gsd-code-fixer)_
_Iteration: 7 (post-`459797ea` confirmation)_
