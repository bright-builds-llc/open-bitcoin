---
phase: 160-explicit-bounded-peer-filter-serving
plan: "10"
subsystem: network-sync
tags: [bip157, tcp, exact-encoded-write, bounded-batches, physical-prefix]
requires:
  - phase: 160-08
    provides: Complete persisted affine filter response plans
  - phase: 160-14
    provides: Shared mixed generic/scratch/CF admission and private encoded buffers
provides:
  - Actual outbound sync consumption of all three prepared filter response families
  - Concrete Tcp exact-byte writes without a second encode
  - Guarded ordinary and announcement batches with cancellation and suffix cleanup
  - Physical prefix, lifecycle and mixed-pressure evidence through concrete loopback sockets
affects: [160-11, 160-12]
tech-stack:
  added: []
  patterns: [borrow-governed-bytes, complete-frame-receipts, abort-encoded-suffix]
key-files:
  created:
    - packages/open-bitcoin-node/src/sync/session/filter_serving.rs
    - packages/open-bitcoin-node/src/sync/tests/peer_filter_transport.rs
  modified:
    - packages/open-bitcoin-node/src/sync/types.rs
    - packages/open-bitcoin-node/src/sync/tcp.rs
    - packages/open-bitcoin-node/src/sync/session.rs
    - packages/open-bitcoin-node/src/sync/session/emission_terminal.rs
    - packages/open-bitcoin-node/src/sync/tests.rs
    - packages/open-bitcoin-node/src/sync/tests/runtime_write_evidence_cases.rs
    - packages/open-bitcoin-node/src/sync/tests/production_announcement_transport_cases.rs
    - packages/open-bitcoin-node/src/sync/tests/support_transport.rs
    - packages/open-bitcoin-node/src/sync/tests/filter_index.rs
    - packages/open-bitcoin-node/src/sync/tests/filter_index/peer_fixtures.rs
key-decisions:
  - Physical Tcp overrides send_encoded with write_all on the exact borrowed governed slice; the documented default is restricted to nonphysical scripted compatibility.
  - Prepare every generic batch buffer through the existing shared handle factory before the first write and retain the real suffix owners until send or drop.
  - Observe cancellation between generic and announcement frames; preserve completed writes and abort all unwritten encoded owners and effect capabilities.
  - Revoke a filter response before completing a full write observed after cancellation, preserving physical-only rather than live range evidence.
requirements-addressed: [CFNET-01, CFNET-02, CFNET-03, CFNET-04, CFNET-05, CFNET-06]
requirements-completed: []
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 160-20261010-peer-filter-serving
generated_at: "2026-10-11T01:43:04Z"
duration: approximately 29min
completed: 2026-10-10
---

# Phase 160 Plan 10: Outbound Governed Filter Transport Summary

**Actual outbound sync now sends complete persisted BASIC responses through concrete Tcp writes of the original governed bytes, with truthful completed-frame receipts and bounded generic/announcement suffix cleanup.**

## Accomplishments and Acceptance

- [x] `sync_connected_peer_with_cancel` consumes `inbound_response_plan` alongside ordinary outbound messages. Prepared filters use their affine carrier; typed unavailable outcomes authorize no normal response, and an unresolved intent is an explicit internal error.
- [x] `SyncPeerSession::send_encoded` provides the exact-frame seam. The sole first-party physical implementation, `TcpPeerSession`, concretely overrides it with `stream.write_all(bytes)` and does not decode, allocate a replacement frame or encode again. Legacy scripted adapters retain the explicitly documented nonphysical fallback. Production managed callers never call legacy `send`.
- [x] Ordinary responses, initial versions, idle/reconciliation messages and announcements all use `encode_pending_network_message(actual_peer, &message)` on the same handle and registry. Full generic batches retain every private encoded owner before their first physical write; admitted encoder scratch remains governed by Plan14's factory. There is no caller-zero admission, separate byte pool or unguarded encoded suffix.
- [x] CF frames call the no-argument atomic `next_frame()`, check captured authority currentness before writes and consume a private receipt only after a successful complete send. No authority/publication/registry lock spans the writer. Wrong targets fail before any socket byte or receipt.
- [x] Failed/partial writes abort the current frame and suffix. Actual seven-byte first-frame failure yields zero completed frames; second-frame failure preserves exactly one complete prior frame and excludes the partial seven bytes from credited framing.
- [x] Full two-frame Tcp writes earn `CurrentRangeComplete` with exactly two messages and the actual socket byte length. Cancellation, grant loss and disconnect during a full write preserve `PhysicalPrefixInvalidated`, one completed frame and its exact actual byte count, without completing the range.
- [x] The blocked writer control starts another thread that acquires shared authority, revokes serving and attempts same-peer readmission. Actual queued suffix buffers drop, while the handed frame remains included in `suffix_bytes`, `active_frame_bytes` and the retained request slot. Re-enabling the grant does not allow a second request until that owner settles; zero counters are asserted only afterward.
- [x] All four actual ordinary sync-send call sites pass the live cancellation control. Generic and announcement batches stop before the next frame, release the remaining encoded owners, and explicitly abort unwritten announcement capabilities. Completed block/effect writes keep their existing physical credit. Original send/admission/completion errors and abort-cleanup errors are combined visibly.
- [x] Concrete loopback observes the actual outbound version with COMPACT_FILTERS plus NETWORK_LIMITED and without NETWORK. The genuine 200-block durable fixture retains authoritative index lag before the actual version write. The same production receive loop returns two exact persisted cfilters, exact predecessor/hash cfheaders and the valid empty checkpoint vector for stop height2; raw frames equal record-derived expected framing.
- [x] Actual Tcp terminal controls exercise generic-first refusal before socket output and filter-first admission while the CF writer owns its frame. Real generic factory buffers approach the shared8MiB boundary, refuse further encoding, remain charged until drop and never bypass encoding-stage peak admission.

## TDD and Verification

All Cargo commands used pinned Bun1.3.9 through `scripts/command-timings.ts` with stable outbound keys and the cooperative build lock. Quiet/queued sessions were polled; no other agent or LSP was terminated. Required filters executed nonzero tests.

| Check | Actual evidence |
| -- | -- |
| Exact-send RED | New concrete Tcp test failed compilation because send_encoded did not exist. A test address-field typo was corrected separately. |
| Cancellation behavioral RED | One executed regression with the old post-write cancellation behavior failed: WrittenPrefix instead of PhysicalPrefixInvalidated. Restoring pre-completion revocation passes. |
| Dropped-plan behavioral RED | One executed actual-caller regression with the old dropped-plan route failed: zero observed cfilters instead of two. Restoring ordinary plan consumption passes. |
| Final Task1 phase160_outbound_exact_send | **1 executed, 1 passed**; raw nondecodable supplied bytes arrive unchanged through the concrete physical override. |
| Final Task2 phase160_outbound_executor | **12 executed, 12 passed**; current complete evidence, wrong target, grant loss, disconnect, filter/generic/announcement cancellation, generic and announcement admission cleanup, real partial writes and unavailable/leaked-intent handling. |
| Final Task3 phase160_outbound_transport | **5 executed, 5 passed**; actual caller/all families/version bits, first/second partial failures and both mixed admission orders. |
| Existing production_announcement_transport_cases | **9 executed, 9 passed**, including six existing fanout, compact/header/inventory, prefix, redaction, enqueue/unregister and malformed-encode controls plus the three new cases. |
| Existing phase123_sync_ | **6 executed, 6 passed**, preserving existing block write counters and compact-activity projection. |
| Existing phase160_filter_ | **23 executed, 23 passed**, preserving affine replay/sequence/foreign identities, poison, retained handed owners and physical-only terminals. Injected panic text is from passing poison controls. |
| Existing phase160_pending_encoding | **15 executed, 15 passed**, including exhaustive wire variants, CF bypass refusal, scratch, witness/compact structure, local envelope boundaries and a genuinely contextually valid maximum-weight block admitted by the shared factory. |
| Final strict node Clippy, all targets/features, -D warnings | Passed after the final production cancellation changes. |
| Final scoped rustfmt, managed all and diff check | Passed; no managed findings or whitespace errors. |
| Breadcrumb check after parent registration | Passed for **1095 Rust files**. |

The initial all-family test used a16-block partial prefix, but ordinary startup genuinely finished that small catch-up. The test now uses200 blocks with an8-record seeded prefix and asserts real authority lag before sending the version. A first retained-charge assertion incorrectly expected zero suffix bytes after revocation; it was corrected to the established handed-frame-inclusive accounting. No production counters were reset to satisfy either test.

## Downstream API and Ownership Handoff

1. Production ordinary paths use `send_all_for_peer_with_cancel`; test-only no-cancel wrappers preserve historical scripted fixtures. Each batch prepares its exact generic owners before sending and borrows only `buffer.bytes()` while the owner remains live. Existing ordinary block acknowledgement and announcement receipts are unchanged.
2. `session::filter_serving::send_filter_plans` is the actual outbound caller's terminal executor. Its `send_response` helper returns the terminal result plus a typed factual `FilterTransportCompletion` result for diagnostics/tests, including failure prefixes. It creates no alternative completion authority. Failed proof observation remains a typed error instead of a fabricated zero.
3. Concrete Tcp's cfg(test) `control_writes` accepts a Send closure over `&mut TcpStream` and the same borrowed frame slice. Success controls call real socket `write_all`; partial controls write a real seven-byte prefix and then return an injected error. The hook retains TcpPeerSession's Send capability for downstream barrier tests. Production builds contain neither this hook nor its field.
4. The finite existing socket write timeout remains configured from read_timeout_ms. The deterministic timeout case returns TimedOut after an actual partial socket prefix; no kernel-saturation latency guarantee is claimed. Cancellation is observed around synchronous frame writes, and lifecycle revocation can independently release queued CF buffers while a writer is blocked.
5. The4,000,000-byte outgoing payload plus24 framing envelope and8MiB mixed pending limit remain explicit new local resource policy. Inbound's historical1MiB gate was not copied into outbound. Generic CF response encoding remains forbidden; prepared CF frames are never converted back into messages for another encode.

## Deviations and Simplification

**[Rule3 - Genuine fixture visibility]** The parent approved two additional existing paths, keeping the plan at12 Rust paths: expose only the test peer_fixtures module from filter_index.rs, narrow its two history-field visibilities to avoid private-interface warnings, and add a lagging constructor that reuses actual TurnHistory.seed(indexed)/Self::open. Accepted history, durable coins, retained bodies and provenance are unchanged. No production export, synthetic validation shortcut or store mutation bypass was added.

**[Rule2 - Batch cancellation propagation]** The final scope review found the inherited ordinary/announcement helpers had no cancellation control. All actual call sites now pass it; new real-socket tests prove only the completed first generic/announcement frame survives and the unwritten owners/capabilities are released.

The explicit simplification pass moved generic terminal methods into the existing emission_terminal module, centralized the physical test loopback/pressure/expected-frame helpers in the already planned support_transport file, and removed a duplicate immutable announcement-target check after preflight. No new production abstraction, registry API, dependency, crate, durable owner or store was added. All 12 owned Rust files remain below 628 lines. Diff review confirmed preserved unrelated RPC edits and no listener/default, permission, indexing, prune or protocol-policy expansion.

## Scope, Security and Shared State

T-160-28 is covered by the actual caller's all-family socket test and its witnessed dropped-route failure. T-160-29 is covered by real encoded suffix ownership, admission/partial/timeout/cancellation cleanup and retained handed charges. T-160-30 is covered by wrong-target refusal, live currentness, disconnect/revocation during write, exact complete-prefix evidence and the existing affine replay/identity regressions.

Expected wire frames here come from genuine persisted records and the existing first-party serializer; this proves actual transport routing and exact bytes, not an independent Knots oracle. Positive1000 checkpoint intervals, independent wire expectations, exact8MiB barrier races and cross-direction integrated proof remain Plan11/16 work. Full native workspace/Bazel/coverage and independent final source/security review remain the parent's final gates. The maximum-weight regression proves shared-factory admission, not a new maximum-weight physical block-serving claim.

The production stub/surface scan found no TODO/FIXME/placeholder path, new endpoint, auth mechanism, production filesystem access pattern or schema change. Legacy Tcp.send still encodes for compatibility; managed production response and announcement paths use only its concrete exact-frame override. AGENTS.md, the Bright Builds sidecar, placeholder-only overrides, relevant architecture/code-shape/testing/verification/operability/Rust standards, active lessons and finalized context/research/coverage informed this implementation.

Both new Rust paths carry ordered pinned anchors packages/bitcoin-knots/src/net.cpp, packages/bitcoin-knots/src/net_processing.cpp and packages/bitcoin-knots/test/functional/p2p_blockfilters.py. The parent serially registered them. This executor made no manifest, STATE/ROADMAP/REQUIREMENTS/todo, staging, commit, push, branch/worktree or hook changes. All three task changes and this summary await the strict outer consolidated commit; requirements-completed intentionally remains empty. No authentication gate or deferred implementation blocker occurred.

## Self-Check: PASSED

All 12 owned Rust paths and this summary exist. Required filters passed 1/12/5 tests; relevant regression filters passed 9/6/23/15. Final strict node Clippy, scoped formatting, managed checks, parity 1095 and whitespace/diff review passed. Commit-existence checks are inapplicable under the explicit no-intermediate-commit lifecycle; no commit hash, completed requirement, independent oracle or full native result was fabricated.
