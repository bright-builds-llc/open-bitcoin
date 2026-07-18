---
phase: 126-compact-relay-residual-hardening
reviewed: 2026-07-18T22:36:40Z
resolved: 2026-07-18T22:54:41Z
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
  warning: 0
  info: 0
  total: 0
status: clean
verdict: clean
generated_by: gsd-code-reviewer
resolved_by: gsd-code-fixer
lifecycle_mode: yolo
phase_lifecycle_id: 126-2026-07-18T16-09-20
---

# Phase 126: Code Review Report

**Reviewed:** 2026-07-18T22:36:40Z
**Resolved:** 2026-07-18T22:54:41Z
**Depth:** standard
**Files Reviewed:** 24
**Status:** clean

## Summary

The Phase 126 implementation correctly closes the compact-relay runtime seams in
scope. Generic factless compact-block dispatch fails with a typed adapter error,
both managed receive paths supply live mempool and bounded-extra candidates,
outbound compact nonces are obtained lazily from system entropy, and entropy
failure falls back without recording a compact announcement. Cargo and Bazel
dependency declarations agree, and the Phase 117/124/126 regression guards pass.

The documentation-consistency warning is resolved. Both parity catalogs now
report the archive-ready 39/39 requirements, 17/17 phases, and Phase 126 4/4
state while retaining their explicit deferred and no-claim boundaries. The Phase
126 checker now guards those lifecycle claims only in `archive_ready`, preserving
the earlier legal lifecycle fixtures.

## Resolved Findings

### WR-01: Archive-ready parity catalogs still claim Phase 126 is pending

**Resolution:** Fixed by `d95220ff` after the mutation-only RED commit
`96d64660`.

**Applied fix:** Promoted both catalog sections to verified/archive-ready
wording, added both catalogs plus `.planning/STATE.md` to the deterministic
Phase 126 corpus, and rejected either stale lifecycle claim when the active state
is `archive_ready`. Separate mutations prove the mempool and P2P claims fail
closed, while a pre-archive fixture proves candidate wording remains legal
before promotion.

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

- Phase 126 test/live checker — 16 passed, 0 failed; live checker passed.
- Phase 117 test/live checker — 24 passed, 0 failed; live checker passed.
- Phase 124 closeout test/live checker — 65 passed, 0 failed across candidate,
  verified-pre-promotion, promoted-pre-summary, and archive-ready fixtures; live
  checker passed.
- Active milestone traceability test/live checker — 21 passed, 0 failed; live
  checker passed.
- Parity breadcrumb checker — passed for 383 Rust files.
- Phase 126 lifecycle validation — `valid`.
- LOC freshness and `git diff --check` — passed.
- Default `bash scripts/verify.sh` through the repo command-timing wrapper —
  passed in 3m 19.062s, including deterministic guards, Rust formatting, clippy,
  all-target build, tests, coverage, and Bazel build/run smoke.

______________________________________________________________________

_Reviewed: 2026-07-18T22:36:40Z_
_Resolved: 2026-07-18T22:54:41Z_
_Reviewer: the agent (gsd-code-reviewer)_
_Fixer: the agent (gsd-code-fixer)_
_Depth: standard_
