---
phase: 111-full-block-serving-request-path
reviewed: 2026-07-04T18:46:30Z
depth: standard
files_reviewed: 18
files_reviewed_list:
  - docs/architecture/status-snapshot.md
  - docs/metrics/lines-of-code.md
  - docs/operator/runtime-guide.md
  - docs/parity/catalog/p2p.md
  - docs/parity/checklist.md
  - docs/parity/index.json
  - docs/parity/source-breadcrumbs.json
  - packages/open-bitcoin-network/src/peer/inventory_state.rs
  - packages/open-bitcoin-network/src/peer/tests.rs
  - packages/open-bitcoin-node/src/network.rs
  - packages/open-bitcoin-node/src/network/block_serving.rs
  - packages/open-bitcoin-node/src/network/inventory.rs
  - packages/open-bitcoin-node/src/network/relay_serving.rs
  - packages/open-bitcoin-node/src/network/tests.rs
  - packages/open-bitcoin-node/src/network/tests/relay_serving_cases.rs
  - scripts/check-phase111-full-block-serving-request-path.test.ts
  - scripts/check-phase111-full-block-serving-request-path.ts
  - scripts/verify.sh
findings:
  critical: 0
  warning: 2
  info: 0
  total: 2
status: issues_found
---

# Phase 111: Code Review Report

**Reviewed:** 2026-07-04T18:46:30Z
**Depth:** standard
**Files Reviewed:** 18
**Status:** issues_found

## Summary

Reviewed the Phase 111 full block-serving request-path implementation, tests, parity docs, and verifier wiring. The Rust request path is directionally consistent with the documented default-off, opt-in serving boundary, but the Phase 111 checker has two guardrail gaps that can let parity documentation or evidence regressions pass verification.

## Warnings

### WR-01: Checker does not read every evidence source it requires

**File:** `scripts/check-phase111-full-block-serving-request-path.ts:18-42`

**Issue:** `TARGET_FILES` is the corpus actually read by the checker, but it omits `packages/open-bitcoin-network/src/peer/inventory_state.rs` and `packages/open-bitcoin-node/src/network/tests/relay_serving_cases.rs` even though `REQUIRED_EVIDENCE` requires both. The actual validation paths use only `TARGET_FILES` (`scripts/check-phase111-full-block-serving-request-path.ts:191-203`), and the test fixture mirrors the same omission (`scripts/check-phase111-full-block-serving-request-path.test.ts:14-26`, `scripts/check-phase111-full-block-serving-request-path.test.ts:200-212`). This means the checker can still pass if the peer `getdata` source path or relay-serving branch evidence regresses, as long as the docs and other test files still contain the expected strings.

**Fix:** Add the omitted evidence roots to `TARGET_FILES` and the test fixture map, then add a regression test that removes a Phase 111-only term from `inventory_state.rs` or `relay_serving_cases.rs` and expects a failure.

### WR-02: Broad `only` marker can hide positive forbidden claims

**File:** `scripts/check-phase111-full-block-serving-request-path.ts:128-160`

**Issue:** `NO_CLAIM_MARKERS` treats the standalone word `only` as sufficient no-claim context. Since `checkNoForbiddenClaim` suppresses failures whenever `isNoClaimContext` returns true (`scripts/check-phase111-full-block-serving-request-path.ts:333-340`, `scripts/check-phase111-full-block-serving-request-path.ts:360-397`), a positive forbidden sentence such as "Phase 111 only supports BIP152 compact block payload serving" would match both `supports` and the forbidden phrase, but be allowed because `only` is in the nearby context. That weakens the release-boundary guard for compact-block, archive, public-network, and production-readiness claims.

**Fix:** Remove `only` from `NO_CLAIM_MARKERS`, or replace it with narrow no-claim phrases that cannot also express support. Add a test case like `Phase 111 only supports BIP152 compact block payload serving.` and assert it produces `forbidden Phase 111 positive claim`.

---

_Reviewed: 2026-07-04T18:46:30Z_
_Reviewer: Claude (gsd-code-reviewer)_
_Depth: standard_
