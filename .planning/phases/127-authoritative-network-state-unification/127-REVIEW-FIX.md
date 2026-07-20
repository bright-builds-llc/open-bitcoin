---
phase: 127
fixed_at: 2026-07-20T00:48:00Z
review_path: .planning/phases/127-authoritative-network-state-unification/127-REVIEW.md
iteration: 3
findings_in_scope: 2
fixed: 2
skipped: 0
status: all_fixed
---

# Phase 127: Code Review Fix Report

**Fixed at:** 2026-07-20T00:48:00Z
**Source review:** `.planning/phases/127-authoritative-network-state-unification/127-REVIEW.md`
**Iteration:** 3

**Summary:**

- Findings in scope: 2
- Fixed: 2
- Skipped: 0
- Final verification: both atomic commits passed the normal `bash scripts/verify.sh` hook

## Fixed Issues

### WR-02: Unknown `getdata` inventory now receives a response that Knots never sends

**Status:** fixed: requires human verification
**Files modified:** `packages/open-bitcoin-node/src/network/inventory.rs`, `packages/open-bitcoin-rpc/tests/black_box_parity.rs`, `docs/metrics/lines-of-code.md`
**Commit:** 24e93bec
**Applied fix:** Unknown inventory queue heads are consumed silently while retaining the existing transaction/block cycle ordering. The real loopback regression now covers `[Unknown, available transaction]` and `[missing transaction, Unknown, available transaction]`; a ping/pong barrier proves each exact response sequence has no extra `notfound` frame without relying on a timeout.

### WR-03: The Phase 127 checker still validates anchors rather than the executed data flow

**Status:** fixed: requires human verification
**Files modified:** `scripts/check-phase127-authoritative-network-state-unification.ts`, `scripts/check-phase127-authoritative-network-state-unification.test.ts`, `scripts/rust-source-invariants.ts`, `docs/metrics/lines-of-code.md`
**Commit:** d06e22c8
**Applied fix:** The checker now validates the live `context` binding before `shared_context`, rejects duplicate authority constructors through chained type aliases across every non-test daemon helper, requires the durable-source binding to feed the block-resolution match and response builder, and inspects only the fields of the actual returned `OpenBitcoinNetworkStatusResponse`. Three adversarial mutations preserve the old dead or unused anchors while replacing each live data flow, and all three are rejected.

## Verification

- WR-02 RED: the real loopback test failed because unknown inventory emitted an unexpected leading `notfound`.
- WR-02 GREEN: the same loopback test passed with exact `Tx, Pong` and `NotFound(missing), Tx, Pong` sequences.
- Mandatory ordered Rust checks passed before the WR-02 commit: `cargo fmt --all`, Clippy for all targets/features with warnings denied, all-target/all-feature build, and all-feature workspace tests.
- WR-03 RED: all three new adversarial fixtures returned an empty checker failure list.
- WR-03 GREEN and final disconfirmation: 15/15 mutation tests passed, including the chained-alias constructor, replacement match discriminant, and dead-field/live-shorthand cases.
- `bun run scripts/check-phase127-authoritative-network-state-unification.ts` passed against the live repository.
- The final focused Phase 127 real-loopback black-box test passed through the command-timing wrapper.
- `git diff --check` passed.
- Both normal commit hooks completed the full repository verifier successfully and refreshed the tracked LOC artifact.

***

_Fixed: 2026-07-20T00:48:00Z_
_Fixer: the agent (gsd-code-fixer)_
_Iteration: 3_
