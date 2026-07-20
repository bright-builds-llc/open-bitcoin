---
phase: 128-production-compact-announcement-transport
fixed_at: 2026-07-20T09:36:02Z
review_path: .planning/phases/128-production-compact-announcement-transport/128-REVIEW.md
iteration: 1
findings_in_scope: 2
fixed: 2
skipped: 0
status: all_fixed
---

# Phase 128: Code Review Fix Report

**Fixed at:** 2026-07-20T09:36:02Z
**Source review:** `.planning/phases/128-production-compact-announcement-transport/128-REVIEW.md`
**Iteration:** 1

**Summary:**

- Findings in scope: 2
- Fixed: 2
- Skipped: 0

## Fixed Issues

### WR-01: Idle inbound sessions do not wake for queued announcements

**Status:** fixed: requires human verification
**Files modified:** `docs/metrics/lines-of-code.md`, `packages/open-bitcoin-node/src/sync.rs`, `packages/open-bitcoin-node/src/sync/session.rs`, `packages/open-bitcoin-rpc/src/inbound_listener.rs`, `packages/open-bitcoin-rpc/src/inbound_listener/connection_runtime.rs`, `packages/open-bitcoin-rpc/src/inbound_listener/tests.rs`, `scripts/check-phase128-production-compact-announcement-transport.ts`, `scripts/check-phase128-production-compact-announcement-transport.test.ts`
**Commit:** `89d6b813`
**Applied fix:** Added cancellation-safe per-peer outbox readiness notifications and selected them alongside the in-progress socket read and shutdown signal. Idle inbound peers now drain queued announcements through the existing post-write receipt path without cancelling a partial-frame read. Added a loopback regression proving an idle peer receives the queued wire message and credits the effect exactly once.
**Verification:** Focused loopback test passed; Phase 128 checker passed 19/19; ordered Rust format, clippy, build, and all-features tests passed; `bash scripts/verify.sh` passed.

### WR-02: Shared authority and outbox registry use colliding peer-ID allocators

**Status:** fixed: requires human verification
**Files modified:** `docs/metrics/lines-of-code.md`, `packages/open-bitcoin-node/src/lib.rs`, `packages/open-bitcoin-node/src/sync.rs`, `packages/open-bitcoin-node/src/sync/runtime_state.rs`, `packages/open-bitcoin-node/src/sync/session.rs`, `packages/open-bitcoin-node/src/sync/tests.rs`, `packages/open-bitcoin-node/src/sync/types.rs`, `packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs`, `packages/open-bitcoin-rpc/src/inbound_listener.rs`, `packages/open-bitcoin-rpc/src/inbound_listener/connection_runtime.rs`, `packages/open-bitcoin-rpc/src/inbound_listener/tests.rs`
**Commit:** `77ccae7d`
**Applied fix:** Replaced independent inbound and outbound counters with one process-wide peer identity authority shared by the durable sync runtime and inbound accept loop. Duplicate live outbox registration now fails instead of aliasing ownership, and outbound teardown releases only the outbox and network peer that the current attempt successfully acquired. Added concurrent identity/outbox isolation and failed-setup cleanup regressions.
**Verification:** Both focused regressions passed; ordered Rust format, clippy, build, and all-features tests passed; Phase 128 checker passed 19/19; production file-length, Bazel, coverage, and the complete `bash scripts/verify.sh` contract passed.

***

_Fixed: 2026-07-20T09:36:02Z_
_Fixer: the agent (gsd-code-fixer)_
_Iteration: 1_
