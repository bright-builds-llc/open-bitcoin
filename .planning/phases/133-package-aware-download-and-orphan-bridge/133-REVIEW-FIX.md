---
phase: 133-package-aware-download-and-orphan-bridge
fixed_at: 2026-07-26T23:17:54Z
review_path: .planning/phases/133-package-aware-download-and-orphan-bridge/133-REVIEW.md
iteration: 1
findings_in_scope: 4
fixed: 4
skipped: 0
status: all_fixed
---

# Phase 133: Code Review Fix Report

**Fixed at:** 2026-07-26T23:17:54Z
**Source review:** `.planning/phases/133-package-aware-download-and-orphan-bridge/133-REVIEW.md`
**Iteration:** 1

**Summary:**

- Findings in scope: 4
- Fixed: 4
- Skipped: 0

## Fixed Issues

### CR-01: Persistent candidate cursors retain unbounded transaction bodies

**Status:** fixed: requires human verification
**Files modified:** `docs/metrics/lines-of-code.md`, `packages/open-bitcoin-network/src/lib.rs`, `packages/open-bitcoin-network/src/peer.rs`, `packages/open-bitcoin-network/src/peer/tests.rs`, `packages/open-bitcoin-network/src/peer/transaction_relay.rs`, `packages/open-bitcoin-network/src/peer/transaction_relay/orphanage.rs`, `packages/open-bitcoin-network/src/peer/transaction_relay/orphanage/candidate.rs`, `packages/open-bitcoin-network/src/peer/transaction_relay/tests/orphanage_cases.rs`, `packages/open-bitcoin-node/src/network/tests/admission_bridge_cases.rs`
**Commit:** 9f284662
**Applied fix:** Candidate cursors now retain one parent body plus boxed child identities, resolve each child from the canonical orphan map on demand, account orphan and cursor allocations against an aggregate retained-byte policy, and reject or evict state growth that exceeds the byte budget. Regression tests cover large bodies, late-announcer byte growth, identity-only cursor retention, and cursor-creation budgeting.

### WR-01: Late announcers bypass the per-peer orphan cap

**Status:** fixed: requires human verification
**Files modified:** `docs/metrics/lines-of-code.md`, `packages/open-bitcoin-network/src/peer/transaction_relay/orphanage.rs`, `packages/open-bitcoin-network/src/peer/transaction_relay/tests/orphanage_cases.rs`
**Commit:** 4c67e41b
**Applied fix:** `add_announcer` now rejects a peer at its configured orphan cap before mutating announcer or per-peer indexes. A regression test proves the peer cannot acquire a second retained orphan and that both canonical orphan bodies remain coherent.

### WR-02: Singleton package admission collapses typed policy failures into internal invariants

**Status:** fixed: requires human verification
**Files modified:** `docs/metrics/lines-of-code.md`, `packages/open-bitcoin-mempool/src/package/report.rs`, `packages/open-bitcoin-mempool/src/package/tests.rs`, `packages/open-bitcoin-mempool/src/pool/package_admission.rs`, `packages/open-bitcoin-mempool/src/pool/tests/package_parity_cases.rs`, `packages/open-bitcoin-node/src/network/admission_bridge.rs`, `packages/open-bitcoin-node/src/network/admission_bridge/package.rs`, `packages/open-bitcoin-node/src/network/tests/package_bridge_cases.rs`
**Commit:** c40edd7e
**Applied fix:** Policy failures now capture their typed `MempoolRejectionCategory` from the originating `MempoolError`, and the singleton bridge consumes that stored category verbatim. A bridge regression covers validation, non-standard, conflict, limit, and internal-invariant categories.

### WR-03: The Phase 133 guard does not enforce its resource-bound claims

**Status:** fixed
**Files modified:** `docs/metrics/lines-of-code.md`, `scripts/check-phase133-package-aware-download-orphan-bridge.ts`, `scripts/check-phase133-package-aware-download-orphan-bridge.test.ts`
**Commit:** e27a9a8d
**Applied fix:** The checker now requires aggregate byte budgeting, the late-announcer peer-cap guard, identity-only cursor storage, canonical child lookup, and the new behavior-focused Rust oracles. Mutation tests prove the checker rejects peer-cap bypass, retained child transaction bodies, missing canonical lookup, removed retained-byte oracles, and collapsed singleton rejection categories.

***

_Fixed: 2026-07-26T23:17:54Z_
_Fixer: the agent (gsd-code-fixer)_
_Iteration: 1_
