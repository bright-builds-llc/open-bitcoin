---
phase: 126-compact-relay-residual-hardening
reviewed: 2026-07-18T22:36:40Z
depth: standard
files_reviewed: 24
files_reviewed_list:
  - MODULE.bazel.lock
  - README.md
  - docs/parity/catalog/mempool-policy.md
  - docs/parity/catalog/p2p.md
  - docs/parity/index.json
  - docs/parity/release-readiness.md
  - docs/parity/source-breadcrumbs.json
  - packages/Cargo.lock
  - packages/open-bitcoin-network/src/error.rs
  - packages/open-bitcoin-network/src/peer/compact_download_state.rs
  - packages/open-bitcoin-network/src/peer/message_dispatch.rs
  - packages/open-bitcoin-network/src/peer/tests.rs
  - packages/open-bitcoin-node/BUILD.bazel
  - packages/open-bitcoin-node/Cargo.toml
  - packages/open-bitcoin-node/src/network.rs
  - packages/open-bitcoin-node/src/network/tests.rs
  - scripts/check-phase117-parity-uat-release-boundary.test.ts
  - scripts/check-phase117-parity-uat-release-boundary.ts
  - scripts/check-phase124-milestone-closeout-reconciliation.fixtures.ts
  - scripts/check-phase124-milestone-gap-closure.test.ts
  - scripts/check-phase124-milestone-gap-closure.ts
  - scripts/check-phase126-compact-relay-residual-hardening.test.ts
  - scripts/check-phase126-compact-relay-residual-hardening.ts
  - scripts/verify.sh
findings:
  critical: 0
  warning: 1
  info: 0
  total: 1
status: issues_found
verdict: findings
generated_by: gsd-code-reviewer
lifecycle_mode: yolo
phase_lifecycle_id: 126-2026-07-18T16-09-20
---

# Phase 126: Code Review Report

**Reviewed:** 2026-07-18T22:36:40Z
**Depth:** standard
**Files Reviewed:** 24
**Status:** issues found

## Summary

The Phase 126 implementation correctly closes the compact-relay runtime seams in
scope. Generic factless compact-block dispatch fails with a typed adapter error,
both managed receive paths supply live mempool and bounded-extra candidates,
outbound compact nonces are obtained lazily from system entropy, and entropy
failure falls back without recording a compact announcement. Cargo and Bazel
dependency declarations agree, and the Phase 117/124/126 regression guards pass.

One documentation-consistency warning remains. Two parity catalog sections still
describe Phase 126 as a pending candidate even though the same reviewed change
set promotes the phase to independently verified and archive-ready. The current
guards do not detect this contradiction.

## Warnings

### WR-01: Archive-ready parity catalogs still claim Phase 126 is pending

**Files:**

- `docs/parity/catalog/mempool-policy.md:201-224`
- `docs/parity/catalog/p2p.md:1430-1463`
- `scripts/check-phase126-compact-relay-residual-hardening.ts:14-25`
- `scripts/check-phase126-compact-relay-residual-hardening.ts:187-232`

**Issue:** The mempool catalog calls the implementation a “runtime candidate”
and states that all Phase 126 requirements remain pending. The P2P catalog
likewise labels it “candidate evidence only” and says all six requirements
remain pending. Those claims contradict the reviewed `README.md`,
`docs/parity/release-readiness.md`, lifecycle artifacts, and requirement
reconciliation, which identify Phase 126 as independently verified and the
milestone as archive-ready. The Phase 126 verifier's target corpus omits both
catalog files, while its parity check validates only `index.json` and source
breadcrumbs. Consequently, all current Phase 117, Phase 124, and Phase 126
checks pass while the contributor-facing parity catalogs remain stale.

**Fix:** Promote both catalog sections from candidate/pending wording to
verified/archive-ready wording while preserving their explicit deferred and
no-claim boundaries. Add both catalog files to the Phase 126 or milestone
closeout verifier corpus, reject candidate/pending Phase 126 wording after
promotion, and add mutation tests proving each stale statement fails closed.

## Confirmed Behavior

- Generic compact-block dispatch requires adapter-supplied receive facts instead
  of silently constructing empty candidate sets.
- Managed compact-block receive paths use current mempool and bounded recent
  extras for reconstruction.
- Compact announcement nonce generation occurs only after the compact action is
  selected; entropy failure takes a peer-safe fallback and records no compact
  provenance or achieved-effect evidence.
- Cargo, Bazel, and generated dependency lock state agree on `getrandom`.
- Phase 117 retains Phase 125/126 ownership after promotion, and Phase 124
  closeout checks remain fail-closed for incomplete lifecycle evidence.

## Verification

- `bun test scripts/check-phase126-compact-relay-residual-hardening.test.ts scripts/check-phase117-parity-uat-release-boundary.test.ts` — 37 passed, 0 failed.
- `bun run scripts/check-phase126-compact-relay-residual-hardening.ts` — passed.
- `bun run scripts/check-phase117-parity-uat-release-boundary.ts` — passed.
- `bun test scripts/check-phase124-milestone-gap-closure.test.ts` — 41 passed, 0 failed.
- `bun run scripts/check-phase124-milestone-closeout-reconciliation.ts` — passed.
- Targeted Cargo tests for the factless-dispatch guard and three Phase 126
  announcement cases — 4 passed, 0 failed.
- `bazel build //packages/open-bitcoin-node:open_bitcoin_node_lib` — passed.
- `bash scripts/verify.sh --fast` through the repo command-timing wrapper —
  passed in 2m 39s, including guard suites, formatting/lint checks, workspace
  tests, and doctests.

______________________________________________________________________

_Reviewed: 2026-07-18T22:36:40Z_
_Reviewer: the agent (gsd-code-reviewer)_
_Depth: standard_
