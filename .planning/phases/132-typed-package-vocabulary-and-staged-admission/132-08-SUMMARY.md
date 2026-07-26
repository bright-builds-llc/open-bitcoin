---
phase: 132-typed-package-vocabulary-and-staged-admission
plan: "08"
subsystem: mempool-package-policy
tags: [rust, mempool, packages, parity, verification]
generated_by: gsd-execute-plan
generated_at: 2026-07-26T09:42:45Z
lifecycle_mode: yolo
phase_lifecycle_id: 132-2026-07-25T18-13-00
requirements-addressed: [PACK-01, PACK-02, PACK-03, PACK-04, PACK-05, PACK-06, PACK-07]
requirements-completed: [PACK-01, PACK-02, PACK-03, PACK-04, PACK-05, PACK-06, PACK-07]

requires:
  - phase: 132-01
    provides: opaque package, report, and effective-fee-group vocabulary
  - phase: 132-02
    provides: revision-bound sparse mempool patches
  - phase: 132-03
    provides: prospective overlay and recomputation oracles
  - phase: 132-04
    provides: ordered dry-run and staged-submit package admission
  - phase: 132-05
    provides: fee grouping, one-trim finalization, and truthful outcomes
  - phase: 132-06
    provides: limited package replacement
  - phase: 132-07
    provides: TRUC, P2A, and ephemeral-dust policy
provides:
  - integrated pinned-Knots package-policy closure at exact package bounds
  - mutation-tested PACK-01 through PACK-07 structural and claim guardrails
  - bounded local package-admission documentation and default verifier ownership
affects: [phase-133-peer-package-assembly, phase-134-lifecycle-integration, phase-138-parity-verification]

tech-stack:
  added: [Bun structural checker]
  patterns: [crate-internal parity matrix, mutation-tested claim guardrail, generated sparse-overlay oracle]

key-files:
  created:
    - packages/open-bitcoin-mempool/src/pool/tests/package_parity_cases.rs
    - scripts/check-phase132-typed-package-staged-admission.ts
    - scripts/check-phase132-typed-package-staged-admission.test.ts
  modified:
    - packages/open-bitcoin-mempool/src/pool/tests.rs
    - docs/parity/source-breadcrumbs.json
    - docs/parity/catalog/mempool-policy.md
    - README.md
    - packages/README.md
    - scripts/check-phase131-rolling-fee-expiry-pressure.test.ts
    - scripts/verify.sh
    - docs/metrics/lines-of-code.md

key-decisions:
  - "Close package parity with one crate-internal max-bound matrix that compares sparse staged state against independent recomputation."
  - "Enforce opaque constructors, error-code-specific privacy doctests, policy order, breadcrumbs, and narrow claims with local mutation tests."
  - "Describe Phase 132 as bounded local pure-core admission while keeping peer assembly, wire/RPC adapters, public relay, propagation, and production claims deferred."

patterns-established:
  - "Public invariant-bearing package types are guarded by both compile-fail doctests and source-structure mutations."
  - "Final package truth is tested after one pressure trim against ordered reports, lifecycle facts, and recomputed state."

duration: 45m22s
completed: 2026-07-26
---

# Phase 132 Plan 08: Package Parity and Proof Closure Summary

Integrated package-policy parity now exercises exact bounds, ordered partial
acceptance, sparse revision-bound application, limited RBF, TRUC, P2A,
ephemeral dust, one final trim, and lifecycle truth, backed by mutation-tested
PACK-01 through PACK-07 guardrails.

## Performance

- **Duration:** 45m22s
- **Started:** 2026-07-26T08:57:23Z
- **Completed:** 2026-07-26T09:42:45Z
- **Tasks:** 3
- **Files changed:** 11

## Accomplishments

- Added 12 hermetic crate-internal parity cases, including generated
  `MAX_PACKAGE_COUNT` sparse-overlay comparison with zero full clones, zero
  production recomputes, and one final trim.
- Proved package API opacity through five E0451/E0616 compile-fail doctests and
  26 independent structural mutation cases.
- Registered the complete Phase 132 package-policy closure in the breadcrumb
  manifest and documented exact Knots anchors, Rust-owned intentional
  differences, and downstream deferred boundaries.
- Made the Phase 132 checker and its mutation suite part of both visible and
  executable default verifier order immediately after Phase 131.

## Task Commits

1. **Task 1 RED: package parity closure matrix** — `69d2f299`
2. **Task 1 GREEN: registered integrated parity suite** — `72a83e48`
3. **Task 2 RED: proof-closure mutations** — `bd0abc29`
4. **Task 2 GREEN: checker, docs, claims, and verifier wiring** — `967f9199`
5. **Task 3: final verification and scope review** — verification-only; no
   additional source change was required

## Verification

- `cargo test -p open-bitcoin-mempool --doc`: 5 passed
- `cargo test -p open-bitcoin-mempool`: 335 unit, 5 public parity, and 5
  doctests passed
- `bun test scripts/check-phase132-typed-package-staged-admission.test.ts`: 26
  passed
- `bun run scripts/check-phase132-typed-package-staged-admission.ts`: PACK-01
  through PACK-07 passed
- `bun run scripts/check-parity-breadcrumbs.ts`: 439 Rust files verified
- `bazel build //packages/open-bitcoin-mempool:open_bitcoin_mempool_lib`: passed
- `bash scripts/verify.sh`: passed in 2m50.555s
- `git diff --check`: passed

## Decisions Made

- The max-bound closure uses deterministic generated members rather than
  timing-sensitive or public-network fixtures.
- The structural checker inspects private fields, checked constructor names,
  read-only accessors, error-code-specific doctests, stage ordering, and
  prohibited adapter/claim families; behavior tests remain the execution
  oracle.
- README claims stop at bounded local pure-core package admission. Peer
  assembly, general package wire, RPC package adapters, public/default relay,
  guaranteed propagation, public-network gates, and production readiness stay
  explicitly deferred.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Updated the Phase 131 verifier-order mutation fixture**

- **Found during:** Task 2 normal pre-commit verification
- **Issue:** The existing Phase 131 mutation fixture assumed direct adjacency
  between the Phase 131 checker and the Phase 117 gate, so the required Phase
  132 insertion made the fixture needle stale.
- **Fix:** Mutate the Phase 131 checker line independently, preserving the
  original missing-checker assertion while allowing later phase checkers before
  Phase 117.
- **Files modified:**
  `scripts/check-phase131-rolling-fee-expiry-pressure.test.ts`
- **Commit:** `967f9199`

## Authentication Gates

None.

## Known Stubs

None.

## Deferred Issues

None.

## Threat Review

No new network endpoint, authentication path, filesystem trust boundary, or
schema surface was introduced. The new checker is deterministic and local; the
new Rust code is test-only.

## Self-Check: PASSED

All declared created files, task commits, lifecycle identity fields, and
requirement completion fields were verified against the working tree and Git
history.
