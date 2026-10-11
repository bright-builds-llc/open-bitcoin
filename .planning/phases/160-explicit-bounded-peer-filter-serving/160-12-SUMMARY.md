---
phase: 160-explicit-bounded-peer-filter-serving
plan: "12"
subsystem: parity-verification
tags: [bip157, native, source-guards, independent-review, bounded-serving]
requires:
  - phase: "160-11"
    provides: Concrete inbound/outbound, genuine provenance and mixed transport evidence
  - phase: "160-17"
    provides: Mutation-sensitive historical guard compatibility with governed transport
provides:
  - Scoped parity ledger, exact local policies and timed Cargo/Bazel reproduction commands
  - Default native routing for peer-serving and independent pinned fixture guards
  - Full default native exit-zero receipt and clean independent source/security reviews
affects: [161, 162]
tech-stack:
  added: []
  patterns: [single-durable-authority, exact-write-receipts, shared-pending-ledger]
key-files:
  created:
    - scripts/check-phase160-peer-filter-serving.ts
    - scripts/check-phase160-peer-filter-serving.test.ts
    - .planning/phases/160-explicit-bounded-peer-filter-serving/160-REVIEW.md
    - .planning/phases/160-explicit-bounded-peer-filter-serving/160-SECURITY.md
    - .planning/phases/160-explicit-bounded-peer-filter-serving/160-VERIFICATION.md
  modified:
    - docs/parity/source-breadcrumbs.json
    - docs/parity/index.json
    - docs/parity/catalog/basic-compact-filters.md
    - README.md
    - AGENTS.md
    - scripts/verify.sh
    - docs/metrics/lines-of-code.md
requirements-addressed: [CFNET-01, CFNET-02, CFNET-03, CFNET-04, CFNET-05, CFNET-06]
requirements-completed: [CFNET-01, CFNET-02, CFNET-03, CFNET-04, CFNET-05, CFNET-06]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 160-20261010-peer-filter-serving
generated_at: "2026-10-11T04:00:11Z"
completion_state: verified
duration: approximately 2h 22m including native and review orchestration
completed: "2026-10-11"
---

# Phase 160 Plan 12: Scoped Parity and Final Native Gate

The complete scoped peer implementation and evidence passed the unchanged full
native contract. Independent final phase verification passed 7/7 grouped truths, all five roadmap
criteria, all six requirements and all 20 locked decisions. Git finalization remains
the authorized outer step.

## Task Results

- [x] Register all new Rust paths and document exact peer policies, differences and
  reproducible local Cargo/Bazel UAT. Breadcrumb checking passes for 1,100 Rust paths.
- [x] Add the native scoped source/claim guard and mutation tests, preserving every
  historical verifier route and adding independent fixture regeneration checks.
- [x] Complete simplification, full native and independent source/security review.
- [x] Issue the lifecycle-valid final phase certificate and close the six
  requirements through the parent workflow. The summary was initially generated
  before that certificate; this targeted completion update preserves its provenance.

The seven documentation/guard implementation paths were prepared by a disjoint
executor; root serialized the breadcrumb manifest, native gate and review agents.
The default guard verifies 38 production edges / 46 executable selectors. Its 740
mutation tests passed, and all 54 independent generator tests reproduced the pinned
1,458,391-byte wire/proof pair. Synthetic protocol vectors, genuine accepted
histories and physical transport observations retain distinct provenance.

## Native Receipt

Command: PATH=/tmp/open-bitcoin-bun-1.3.9-fresh:$PATH bash scripts/verify.sh

Final source log: packages/target/phase160-native-final-3.log (ignored local
receipt). Exit0; completed in 26m 5.720s (1,565,720ms). Bun 1.3.9 and Rust 1.94.1.

- All 3,975 Bun pass records, deterministic source/claim guards and independent
  fixture checks passed.
- Ordered formatting, strict workspace all-target/all-feature Clippy, all-target
  build and workspace/explicit-client tests passed: 4,093 primary Rust passes;
  three deliberately ignored historical measurement controls remain explicit.
- Benchmark listing/smoke/report validation, six Bazel target builds and build
  provenance passed.
- Clean pure-core LLVM coverage passed with no Uncovered Lines. No production
  source/function or test path was excluded to meet this gate.
- LOC freshness, dependency, file-size and panic-site checks passed. The existing
  generator counts tracked inputs; the commit hook will regenerate from the full
  staged index before repeating the same native contract.

## Evidence-Driven Repairs

Native attempt 1 stopped after 7m 55.375s on historical source literals for the old
raw-buffer/inline-write shape. Independently checked Plan 17 repairs eight exact
legacy guard/test/README paths without changing production behavior or disabling
any historical protection. Final focused receipts include 90 P123 tests, 438
announcement/lifecycle/release tests and 757 related consumer tests. Independent
review also required live test-only reexport/carrier ownership decoys; both failed
before repair and passed afterward.

Broad Rust preflight exposed three old manually constructed peers that omitted
WR-01's handshake. Existing helper calls corrected the fixtures while preserving
all assertions; each focused regression passed, and the final workspace run passed.

Native attempt 2 passed tests/benchmarks/Bazel/provenance but stopped at final
coverage after 26m 7.784s. The existing codec, capability, proof/policy, resource and
dispatch scopes were reopened under160-COVERAGE-CLOSURE.md. Twelve focused
boundary tests and four existing dispatch tests passed; network-only LLVM
coverage passed 598 unit and four integration tests with zero uncovered lines.
The dispatcher fixture now directly constructs valid/invalid inputs instead of
retaining an impossible unreachable match arm. Production behavior is unchanged.

## Independent Review and Threat Coverage

160-REVIEW.md is clean across 123 exact product paths, with no open findings.
WR-01 handshake, WR-02 unsolicited inbound response handling and WR-03 guard
ownership findings remain documented as resolved. The source fingerprint is
821780ec7e24bbb973c0819db9e6c9d1ab28d241e59155e8c78c799e5c8a96ee.

160-SECURITY.md independently verifies 39 unique declared mitigations, zero open,
with T-160-35 owned uniquely by Plan 17. It is scoped source mitigation evidence;
its finalization control requires the actual native/phase gates before git, rather
than pretending a prior native failure was a pass. Neither report was self-certified
by an implementer. The independent parent verifier issued the passed final certificate.

## Scope, Simplification and Completion Boundary

The explicit simplification pass retained one durable authority, one guarded query
path, one affine registry and small socket adapters. Historical guards follow the
real delegated writer/acknowledgement chains, including ordinary compiled ownership,
exact encoded sends, Written-only prefix credit and suffix cleanup.

Docs distinguish the all-only disabled-backend corner, service bits before catch-up,
strict stale timestamp/proof ages, checkpoint intervals, complete missing-row
refusal and exact local work/byte/descriptor limits. Generic encoder peak differs
from retained frame size; resource counters are logical bounds, not RSS promises.
Read-only measurements are distinguished from encoding charges and disk-cache
coldness. The deterministic equal-work tie policy differs deliberately from Knots.

Phase 161 broad operator projections and Phase 162 integrated retained-client proof
remain pending. No V0/BIP37, public/default listener, archive, production-readiness,
funds-safety, independent second store or request-time generation claim is added.

## Task Commits and Shared State

No intermediate commit, staging, push, branch/worktree change or hook bypass.
All tasks await the strict outer commit/push after the final passed lifecycle
certificate. The parent alone owns requirements, STATE/ROADMAP and git actions.

## Self-Check: PASSED

Implementation artifacts, native exit 0, independent clean review/security and
passed 7/7 final verification exist. Lifecycle validation passes with all 17 plans
and summaries. All six CFNET requirements are supported; the outer workflow
retains the hook-protected commit/push as its final action.
