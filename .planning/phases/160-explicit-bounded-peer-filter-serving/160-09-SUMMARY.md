---
phase: 160-explicit-bounded-peer-filter-serving
plan: "09"
subsystem: inbound-network
tags: [bip157, tokio, actual-loopback, affine-cleanup, mixed-pending]
requires:
  - phase: 160-08
    provides: Complete persisted request-bound filter response preparation
  - phase: 160-14
    provides: Atomic shared generic-buffer/scratch and CF-active governance
provides:
  - Actual inbound socket serving for cfilter, cfheaders and cfcheckpt
  - Ordered response carriers and guarded ordinary/announcement encoding
  - Complete-write-only receipts and cancellation-safe unwritten suffix retirement
  - Actual loopback ingress, permission, absence, partial-write and disconnect evidence
affects: [160-11, 160-12]
tech-stack:
  added: []
  patterns: [ordered-affine-steps, same-ledger-encoding, synchronous-suffix-retirement, absolute-write-deadline]
key-files:
  created:
    - packages/open-bitcoin-rpc/src/inbound_listener/filter_serving.rs
    - packages/open-bitcoin-rpc/src/inbound_listener/tests/peer_filters.rs
    - packages/open-bitcoin-rpc/src/inbound_listener/tests/peer_filters/failures.rs
  modified:
    - packages/open-bitcoin-rpc/src/context.rs
    - packages/open-bitcoin-rpc/src/context/inbound_wire.rs
    - packages/open-bitcoin-rpc/src/context/network.rs
    - packages/open-bitcoin-rpc/src/inbound_listener/connection_runtime.rs
    - packages/open-bitcoin-rpc/src/inbound_listener/resource_runtime.rs
    - packages/open-bitcoin-rpc/src/inbound_listener/tests.rs
    - packages/open-bitcoin-rpc/src/inbound_listener/tests/block_serving.rs
    - packages/open-bitcoin-rpc/src/inbound_listener/tests/listener_fixtures.rs
    - packages/open-bitcoin-rpc/src/inbound_listener/tests/announcement_successful_prefix.rs
    - packages/open-bitcoin-network/src/resource.rs
    - packages/open-bitcoin-network/src/resource/tests.rs
key-decisions:
  - Use the configured context handle's same pending ledger for ordinary responses and announcements.
  - Keep CF receipt settlement synchronous immediately after the complete socket write.
  - Retire queued capabilities synchronously and retain drop guards across every cancellable await.
  - Box the larger ordinary step variant to satisfy strict Clippy without optional carrier states.
requirements-addressed: [CFNET-01, CFNET-02, CFNET-03, CFNET-04, CFNET-05, CFNET-06]
requirements-completed: []
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 160-20261010-peer-filter-serving
generated_at: "2026-10-11T01:41:47Z"
duration: approximately 27min including context loading
completed: 2026-10-10
---

# Phase 160 Plan 09: Actual Inbound Filter Transport Summary

**The opt-in Tokio listener serves all three persisted BASIC families through request-bound full-frame receipts, with the configured shared pending ledger and cancellation-safe cleanup for ordinary, filter and announcement output.**

## Accomplishments and Acceptance

- [x] `InboundWireStep::{Encoded(Box<EncodedWireResponse>), Filter(PreparedFilterResponse)}` preserves ordered owned output. `PreparedFilterServe` consumes the complete lease, typed unavailable outcomes produce no normal response, and an unresolved intent fails resolution.
- [x] The real ingress command gate now admits the three CF request commands. Actual loopback RED exposed their previous rejection before codec/dispatch. Incoming payload limits and unsupported response commands remain unchanged.
- [x] Ordinary responses reserve/encode through `encode_pending_network_message` before retaining bytes. Every encoded batch suffix keeps its private `GenericPendingBuffer`; no untracked encoding fallback exists. Announcement encoding explicitly uses the context's configured CF handle and keeps its owner through the entire write. Announcements encode sequentially, so their remaining suffix owns affine unencoded emissions rather than prematurely allocated frames.
- [x] CF admission uses no caller pressure snapshot: `next_frame()` reads the same registry's mixed counters. Existing generic 1MiB/128-message local policy remains; only the request-bound CF writer receives the explicit 4,000,024-byte local frame exception. Count bounds remain active for that exception.
- [x] CF currentness is checked immediately before every frame, the actual write occurs outside context/authority/registry locks, and only `WriteWireMessageOutcome::Written` consumes the private receipt. Receipt settlement follows synchronously, with no cancellable await after achievement.
- [x] Partial first frames earn zero; a complete first frame followed by a partial second earns exactly one physical prefix and no successful range. Invalidating a fully written first frame returns physical-only evidence and stops the suffix.
- [x] Explicit terminal cleanup visits every unwritten ordinary/filter capability. Drop guards cover task cancellation for the active ordinary buffer, queued batch and active/queued announcements. Resolver settlement and suffix retirement contain no cancellation window; acknowledgement takes a capability only after acquiring context.
- [x] Original CF transport failure, physical prefix, progress-read failure and cleanup failure remain represented together. Ordinary terminal events retain socket/settlement cause plus cleanup status; announcement cleanup failure records its original terminal cause while preserving legacy `AbortFailed` semantics.
- [x] Socket writes use one absolute deadline across partial retries and reject an already-expired deadline. Queue pressure, expired deadline, disconnect and dropped futures release all unwritten reservations.

## TDD and Verification

All Cargo commands used pinned Bun1.3.9 through the serialized timing runner. The first recorded RED began at2026-10-11T01:19:30.549Z; context loading preceded that record. Every required filter executed nonzero tests.

| Check | Actual evidence |
| -- | -- |
| Initial executor RED | New pending-charge test could not compile against the missing filter carrier arms. |
| Actual ingress behavioral RED | Real listener requests reset because `resource.rs` rejected all three CF commands before codec/dispatch; the narrow allowlist fix makes actual all-family and no-response tests pass. |
| Required phase160_inbound_executor | **10 executed, 10 passed**, final run4.64s, zero ignored. Includes partial first frame, physical-only late write, blocked cancellation/readmission refusal, actual disconnect, local gate, generic batch guards, shared scratch refusal and announcement charge/drop controls. |
| Required phase160_inbound_transport | **6 executed, 6 passed**, final run7.03s, zero ignored. Actual listener exact families, permission versions/refusal, all-family disabled-backend no-response, missing later row, forced second partial frame and expired production socket deadline. |
| Final inbound_listener regressions | **72 library tests and1 daemon test executed, all passed**, library12.71s. Existing body, durable restart/corruption, TX/announcement prefix, admission, envelope, timeout, reserved-capacity and idle-announcement assertions remain intact. |
| Final RPC Clippy all targets/features, -D warnings | Passed. |
| Scoped rustfmt check, whitespace and diff review | Passed across all13 owned Rust paths. |
| Managed all | Passed, zero findings. |
| Parity breadcrumbs | Passed for1095 Rust paths after parent serial registration. |

The final announcement diagnostics callback was additionally covered by the final full inbound regression and strict Clippy runs. Full native workspace/Bazel/coverage and independent final reviews remain parent-owned gates; this plan does not claim them.

## Concrete Evidence and Limits

The durable fixture accepts40 genuinely validated distinct-script coinbase blocks through a managed runtime, persists bodies, flushes a real durable coins fence, reopens configured BASIC indexing and serves an indexed prefix while `synced=false`. It is a finite coinbase-only transport fixture. Spending histories, retained branches, positive1000-height checkpoint socket matrices and independent pinned Python wire-oracle comparisons remain Plan11's concrete evidence work.

The real listener observes configured general, effective `all`, explicit `blockfilters` and ordinary pre-catch-up versions with truthful `NETWORK_LIMITED` composition. Ordinary unadvertised capability receives an actual disconnect. The disabled-backend `all` session returns no normal response for each of the three valid families, demonstrated by the next exact Pong sentinel. An indexed prefix followed by an unindexed later record produces no cfilter prefix.

Actual successful sockets compare entire framed bytes for three nonempty persisted cfilters, a cfheaders response with a nonzero predecessor and a legitimate below1000 empty checkpoint response. Expected frames use persisted checked records and the first-party serializer; they are not independent oracle evidence. The successful listener test retains a generic pending owner on the same configured ledger through all CF writes and observes its exact surviving charge afterward.

Forced partial controls use the same CF executor called by `handle_inbound_stream`: they physically write13 bytes of the failing frame into a real loopback socket. The successful prefix and partial remainder are separately read. The timeout case injects an expired zero deadline into the real socket writer; it does not claim a timed large-filter backpressure soak. The disconnect case fills a real socket and closes its peer before executing the prepared response.

Blocked-write controls actively acquire both context and authority. Disable drops the queued suffix while the handed frame and peer slot remain charged; re-enable cannot readmit that peer until the old future/frame actually drops. Generic suffix tests retain all encoded owners and observe only the handed owner's charge after batch drop. Announcement cancellation verifies that actual encoded bytes drop before its capability retirement and all three current/suffix capabilities abort.

The mixed scratch test retains a real400k generic witness buffer and hands a CF frame, then refuses a near4M generic message because its encoder peak plus live bytes exceeds8MiB. Existing charges remain exact and all recover after actual owner drop. This synthetic codec input is a resource test, not a consensus-valid maximum block claim. Plan14 owns maximum-weight validation and Plan11 owns larger cross-direction races/boundary evidence.

## Deviations, Scope and Simplification

**[Rule3 - Existing test helper migration]** Parent approved the eleventh existing path `context/network.rs`: cfg(test) response helpers now construct the same private guarded carrier and propagate typed encoding diagnostics. Test-only observation copies remain cfg(test).

**[Rule3 - Missing actual ingress]** Parent approved the twelfth existing path `network/src/resource.rs` after witnessed socket RED. Only `getcfilters`, `getcfheaders` and `getcfcheckpt` were added; the generic receive envelope was preserved.

**[Rule2 - Cancellation correctness]** Existing worker shutdown aborts connection tasks. Guarded batches/current responses and announcements now synchronously retire unwritten capabilities on future drop, without early uncharging live allocations. Resolver/acknowledgement await ordering closes capability ownership gaps.

**[Rule1 - Deadline correctness]** The writer formerly restarted its timeout for every partial retry. One absolute deadline now bounds the full write, including zero-deadline refusal before any byte is attempted.

**[Rule3 - Strict lint and readable tests]** The ordinary step is boxed to avoid the512-byte enum variant expansion. Parent approved the thirteenth source path, `peer_filters/failures.rs`, after formatted tests exceeded the managed file-size trigger. Both test files and the production entrypoint remain below628 lines.

The explicit simplification pass unified ordinary/TX encoding, centralized ordinary acknowledgement, extracted the existing affine announcement executor beside the new terminal helper, retained sequential announcement encoding and reused one existing registry/authority. No node governance API change, new crate/dependency, store/schema, listener default, public endpoint or Phase161/162 product was added.

AGENTS.md, the Bright Builds sidecar, placeholder overrides, architecture/code-shape/testing/verification/operability/Rust standards, both active lesson files and finalized context/research/decision coverage materially informed execution.

## Security and Stub Review

- T-160-25: Only actual full framed writes consume CF receipts; exact successful and forced partial socket bytes are observed.
- T-160-26: Ordinary/announcement factory owners and CF admission use the configured shared ledger; local generic gates remain independent. Batch, blocked-write, cancellation, scratch refusal and cleanup controls pass.
- T-160-27: Live checks precede writes and receipt settlement rechecks lifecycle; late invalidated full writes remain physical-only, and handed charges survive until actual drop.

The stub scan found no TODO/FIXME/placeholder production implementation. No additional unplanned endpoint, auth path, filesystem access pattern or trust-boundary schema was introduced. The request command allowlist is the planned protocol ingress surface.

All three new paths use ordered pinned anchors `packages/bitcoin-knots/src/net.cpp`, `packages/bitcoin-knots/src/net_processing.cpp`, `packages/bitcoin-knots/test/functional/p2p_blockfilters.py`. Parent registered them and the parity check passed.

## Task Commits, Shared State and Next Readiness

Both tasks and this summary await the strict outer consolidated commit. No staging, commit, push, branch/worktree change, hook bypass or shared STATE/ROADMAP/REQUIREMENTS/todo/parity-manifest mutation was performed by this executor. Requirements-completed intentionally remains empty. No authentication gate or unresolved owned implementation issue remains.

The outbound adapter remains Plan10-owned; Plan11 can consume the ordered boxed ordinary/filter steps and actual socket seams. Parent owns final native verification, source/security review, requirement closure and git finalization.

## Self-Check: PASSED

All13 owned Rust paths and this summary exist. Required executor/transport filters executed10/6 passing tests; final inbound regression executed72 library and1 daemon passing tests. Strict all-target/all-feature RPC Clippy, scoped rustfmt, managed checks, parity1095 and whitespace/diff review passed. Commit-existence checks are inapplicable under the explicit no-intermediate-commit lifecycle; no commit hash, independent oracle result, full native result or completed requirement was fabricated.

## Post-review remediation | 2026-10-11

Independent source review WR-02 found that the newly decoded unsolicited CF responses could not reach the intended inert dispatch through the inbound command allowlist. The parent added only cfilter/cfheaders/cfcheckpt to that same allowlist, preserving the1MiB envelope and all existing frame/checksum checks. The additional existing resource/tests.rs path makes this original adapter plan14paths. One executed bounded-envelope regression failed before the fix and passed afterward; Plan11's actual unsolicited-response/Pong and pre-handshake controls subsequently passed. This supports inert receive handling, not a filter-client product. Final source and native gates remain parent-owned.
