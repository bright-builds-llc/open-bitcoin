---
phase: 117-parity-traceability-uat-and-release-guardrails
reviewed: 2026-07-10T06:49:28Z
depth: standard
files_reviewed: 28
files_reviewed_list:
  - README.md
  - docs/architecture/operator-observability.md
  - docs/architecture/status-snapshot.md
  - docs/operator/runtime-guide.md
  - docs/parity/README.md
  - docs/parity/catalog/consensus-validation.md
  - docs/parity/catalog/p2p.md
  - docs/parity/checklist.md
  - docs/parity/deviations-and-unknowns.md
  - docs/parity/index.json
  - docs/parity/production-claim-boundary.md
  - docs/parity/release-readiness.md
  - docs/parity/source-breadcrumbs.json
  - docs/parity/support-matrix.md
  - packages/open-bitcoin-network/src/compact_download.rs
  - packages/open-bitcoin-network/src/compact_download/tests.rs
  - packages/open-bitcoin-network/src/peer/compact_download_state.rs
  - packages/open-bitcoin-network/src/peer/compact_relay.rs
  - packages/open-bitcoin-network/src/peer/compact_relay/tests.rs
  - packages/open-bitcoin-network/src/peer/message_dispatch.rs
  - packages/open-bitcoin-node/src/network/block_relay_evidence.rs
  - scripts/check-phase100-relay-activation-boundary.test.ts
  - scripts/check-phase100-relay-activation-boundary.ts
  - scripts/check-phase103-mempool-lifecycle.test.ts
  - scripts/check-phase103-mempool-lifecycle.ts
  - scripts/check-phase117-parity-uat-release-boundary.test.ts
  - scripts/check-phase117-parity-uat-release-boundary.ts
  - scripts/verify.sh
finding_counts:
  critical: 0
  warning: 5
  info: 0
  total: 5
status: issues_found
---

# Phase 117: Code Review Report

**Reviewed:** 2026-07-10T06:49:28Z
**Depth:** standard
**Files Reviewed:** 28
**Status:** issues_found

## Summary

The current worktree preserves the intended bounded, explicit, default-off v2.1 documentation claim, and the Rust changes are breadcrumb-only with no runtime behavior change. No current documentation/implementation contradiction or secret/raw-evidence exposure was found in the reviewed files.

The new aggregate checker nevertheless has several false-negative paths. These matter because Phase 117 records the checker as deterministic proof for Knots anchors, exactly-once release state, claim boundaries, and default-verifier scope. A passing `bash scripts/verify.sh` does not expose these gaps because the checker and its fixtures currently share the same assumptions.

Verification performed during review:

- `bun test scripts/check-phase100-relay-activation-boundary.test.ts scripts/check-phase103-mempool-lifecycle.test.ts scripts/check-phase117-parity-uat-release-boundary.test.ts` passed: 29 tests, 0 failures.
- The Phase 100, Phase 103, and Phase 117 repository-mode checkers each passed.
- `git diff --check` passed for all 28 reviewed files.
- The parent workflow reports that the complete `bash scripts/verify.sh` contract passed before review.

## Warnings

### WR-01: Knots-anchor validation is self-satisfied by the checker source

**File:** `scripts/check-phase117-parity-uat-release-boundary.ts:266`
**Issue:** `checkRequiredEvidence` joins every target file, including `scripts/check-phase117-parity-uat-release-boundary.ts` and its test, and then searches that combined corpus for `REQUIRED_KNOTS_ANCHORS`. Every required anchor already appears in the checker constant itself, so deleting an anchor from all parity documentation and breadcrumb evidence still leaves the repository check green. The mutation fixture hides this by removing the anchor from every fixture file, including the checker source. This makes the BOUND-01 anchor assertion vacuous in the real repository.

**Fix:** Validate anchors only in evidence-bearing structures: parse the relevant `docs/parity/index.json` upstream source/test arrays, parse the required breadcrumb groups and require their concrete breadcrumbs, and optionally require the anchors in the named catalog sections. Change the mutation test to remove an anchor only from the evidence files while leaving checker/test source unchanged.

### WR-02: Verifier-boundary validation can accept wrong commands and prohibited external gates

**File:** `scripts/check-phase117-parity-uat-release-boundary.ts:297`
**Issue:** The executable-order check searches only `run_step` labels, not the full command, so a line with the expected label can execute a different program and still pass. The visible-order check does not include the pure-core marker, so the visible Phase 117 pair can move after pure-core checks without failing. The forbidden-gate scan only examines individual lines beginning with `run_step` and recognizes six narrow strings; public-network, live-mainnet, soak, wall-clock, or service-manager commands under other names or on continuation lines pass despite BOUND-04 and the Phase 117 plan explicitly excluding them.

**Fix:** Require the exact full visible and executable command lines in Phase 116 → Phase 117 → pure-core order. Parse logical `run_step` commands including continuations, and reject the complete forbidden boundary vocabulary (`public-network`, `live-mainnet`, `soak`, `wall-clock`, `service-manager`, `systemd`, `launchd`, and production deployment forms). Add mutations for a correct label with a wrong command, visible placement after pure-core, a generic public-network/soak script, and a multiline command.

### WR-03: A no-claim word anywhere in a clause suppresses unrelated positive overclaims

**File:** `scripts/check-phase117-parity-uat-release-boundary.ts:327`
**Issue:** `checkClaims` skips an entire clause or Markdown table row when any `NO_CLAIM_MARKERS` value appears. A mixed statement such as “Package relay remains deferred, while production service operation is supported” contains both `deferred` and a forbidden positive production claim, but the early return at line 333 accepts the whole clause. Broad markers such as `outside`, `future`, and `optional uat` create the same bypass. This is the qualifier-masking risk the Phase 117 research identified as high.

**Fix:** Evaluate each dangerous topic occurrence with local grammatical/context windows and require the negative marker to qualify that same topic. Continue scanning the remainder of the clause after a valid negative occurrence. Add mixed-positive/negative sentence and table-row mutations.

### WR-04: Phase 100's later-phase compatibility exception disables every claim check for the unit

**File:** `scripts/check-phase100-relay-activation-boundary.ts:355`
**Issue:** Any context unit naming Phase 101 through Phase 117 and not Phase 100 returns before checking any forbidden claim. The regression needed only to permit the later compact-block surface, but the implementation also permits package/filter/public-default/production claims in the same Phase-tagged unit. That is broader than the Phase 103 compatibility change, which limits exceptions to an explicit owned-claim set, and it weakens the historical checker for future documentation edits.

**Fix:** Replace the whole-unit return with an explicit mapping of later-phase-owned claim phrases and continue checking every other forbidden phrase. Add a negative fixture such as “Phase 117 provides production full-node readiness” alongside the valid compact-block fixture.

### WR-05: Invalid checklist status values satisfy the done-surface assertion

**File:** `scripts/check-phase117-parity-uat-release-boundary.ts:225`
**Issue:** The aggregate surface check fails only when a checklist status is exactly `planned`. A missing status or any other invalid/non-done value such as `blocked`, `partial`, or `unknown` passes as long as the top-level surface is `done`. The closeout can therefore report successful exactly-once ownership while the human/machine checklist surface is not actually complete.

**Fix:** Require `top.status === "done"` and `checklist.status === "done"` independently. Add mutations for missing and non-done checklist status values. Also consider requiring exactly one top-level and one checklist entry per expected surface instead of selecting the first match.

## Review Conclusion

The reviewed runtime/doc changes are scoped correctly, but the five release-checker false negatives should be fixed before treating Phase 117 verification as a durable release guardrail.

_Reviewer: the agent (gsd-code-reviewer)_
