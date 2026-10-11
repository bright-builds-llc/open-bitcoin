---
phase: 160-explicit-bounded-peer-filter-serving
plan: "07"
subsystem: network
tags: [basic-filters, affine-receipts, bounded-transport, lifecycle]
requires:
  - phase: 160-13
    provides: Opaque shared monotonic read-integrity epochs
provides:
  - One bounded request lease and ordered full-frame receipt sequence
  - Honest queued suffix and handed-frame ownership across cancellation and invalidation
  - Shared-authority currentness and separate physical-prefix/current-range completion
affects: [160-08, 160-09, 160-10, 160-11, 160-12, 161]
tech-stack:
  added: []
  patterns: [affine-request-lease, bounded-shared-registry, physical-versus-live-completion]
key-files:
  created:
    - packages/open-bitcoin-node/src/network/filter_transport.rs
    - packages/open-bitcoin-node/src/network/filter_transport/registry.rs
    - packages/open-bitcoin-node/src/network/filter_transport/tests.rs
    - packages/open-bitcoin-node/src/network/runtime_authority/filter_index/transport.rs
    - packages/open-bitcoin-node/src/network/runtime_authority/filter_index/transport/tests.rs
  modified:
    - packages/open-bitcoin-node/src/network.rs
    - packages/open-bitcoin-node/src/network/runtime_authority.rs
    - packages/open-bitcoin-node/src/network/runtime_authority/filter_index.rs
    - packages/open-bitcoin-node/src/network/runtime_authority/filter_index/readiness/owner.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/filters.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/filters/query.rs
key-decisions:
  - Cancellation frees actual queued buffers while a handed frame retains its bytes and peer/request slot until completion or drop.
  - Terminal request identity never revives after configuration re-enable or peer reconnect; late achieved writes remain physical-only.
  - Actual socket adapters acknowledge immutable frames only after complete writes; no public arbitrary-length or receipt constructor exists.
  - Mixed non-CF pending transport accounting must be serialized by downstream adapters through their shared governance boundary.
requirements-addressed: [CFNET-06]
requirements-completed: []
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 160-20261010-peer-filter-serving
generated_at: "2026-10-10T20:52:10Z"
duration: approximately 35min excluding prerequisite pause
completed: 2026-10-10
---

# Phase 160 Plan 07: Bounded Filter Transport Receipts Summary

**One affine request owns up to 1,000 immutable frames with checked suffix/frame quotas, exact one-use receipts and truthful physical-prefix evidence after lifecycle invalidation.**

## Accomplishments and Acceptance

- [x] All handle clones share one finite registry: one request per peer, four aggregate, 8MiB per suffix and 32MiB aggregate suffix. IDs use checked monotonic allocation; terminal history retains at most 64 IDs.
- [x] A 1,000-frame request traverses one lease and sequence, independently of the generic 128-effect ledger. Preparation and frame handoff do not advance written evidence.
- [x] Per-frame admission includes all CF frames already handed across the shared handle. The narrow peer frame maximum remains 4,000,024 bytes and combined aggregate maximum remains 8MiB.
- [x] Cancellation and invalidation drop actual queued frame Vecs and their container. A handed frame remains charged, including its request/peer slot, until completion or drop. A second same-peer request cannot reuse those charges early.
- [x] Exact full-frame receipts bind private request/session/sequence/length authority, are consumed once, and cannot Clone, deserialize or be publicly constructed. Full-range success requires the final expected sequence and zero remaining suffix; an empty checkpoint vector still requires its single response frame.
- [x] Peer and owner binding are explicit. Unbound preparation is revoked on teardown; cancelled preparation cannot bind afterward. Authority, peer session, index generation, branch anchor, stop identity and Plan13 opaque integrity epoch are checked without backend I/O.
- [x] Mempool expiry and an actually accepted tip extension preserve the unchanged immutable range. Foreign identity axes, same-generation recovery, config disable and disconnect revoke live success.
- [x] Configuration re-enable and disconnect/reconnect cannot revive terminal request identity. Late achieved writes preserve only physical prefix, including a dropped achieved receipt or completion after authority/store/registry failure.
- [x] Cancellation does not acquire the authority mutex. The shared collector snapshots at most four identities, releases the registry, then inspects currentness; no registry-to-authority acquisition or socket I/O is introduced.
- [x] Readiness shutdown still stops/wakes its ordinary waiters when transport cleanup fails, then returns the typed cleanup error. Normal readiness wake behavior passed its existing regressions.

## Verification

Every Cargo command used pinned Bun1.3.9 through `scripts/command-timings.ts` and its cooperative build lock.

| Check | Actual evidence |
| -- | -- |
| Initial Task1 RED | The two lease tests failed to compile before the missing affine owner/types existed; initial GREEN executed two tests. |
| Task1 required filter `phase160_filter_lease` | Seven executed, seven passed, including 1,000 frames, exact mixed pressure, checked growth/ID exhaustion, actual queued-buffer release and cancellation under held authority. |
| Task2 unbound teardown behavioral RED | One executed test failed with one leaked request after disconnect; final implementation releases it. |
| Task2 required filter `phase160_filter_receipts` | Fifteen executed and passed before the final readiness-stop regression was added. |
| Pre-write ABA behavioral RED | One executed test reproduced incorrectly current identity after config false→true; the final shared terminal-identity guard fixes this and reconnect. |
| Final combined filter `phase160_filter_` | **23 executed, 23 passed, zero ignored:** seven lease and sixteen receipt controls on the final source, including the failed-cleanup readiness stop regression. |
| Exact cancelled/foreign binding regression | One executed, one passed after adding the terminal binding guard. Included again in the final combined run. |
| Privacy `cargo test --doc … filter_transport` | Five executed compile-fail controls passed: lease/capability/receipt non-Clone, private receipt construction and absent receipt deserialization. |
| Existing `phase159_basic_readiness` regression filter | **21 executed, 21 passed**, including reentrant wakes, explicit stop/drop, disable/reorg, publication faults and authority poison. |
| Final node Clippy, all targets/features, `-D warnings` | Passed. The first lint pass identified four denied production `expect` calls; typed guards and direct private acknowledgement construction removed all four. |
| Scoped rustfmt, managed `all`, whitespace/diff review | Passed; zero managed findings and clean `git diff --check`. |
| Final parity check | Passed for **1,083 Rust files**; parent serially registered all five new paths. |

Injected panic output belongs to explicit poison tests, all of which pass. The finite stored fixture uses configured Enabled startup over a complete recovered prefix; the first PreserveSaved fixture had no append owner and correctly refused binding. Tests do not claim that the recovered synthetic prefix is a newly consensus-accepted continuous history. The extension test does perform actual acceptance of its new block. Plan11 owns the continuous genuine branch matrix; Plans08–10 own protocol encoding and real socket sends.

No Plan13 query tests were unnecessarily rerun after its separately accepted seven-epoch/six-point-query gate. Background LSP and unrelated host compilation delayed some Cargo sessions; they were polled with liveness evidence and never terminated merely for exceeding an estimate.

## Exact Plan08 Preparation Handoff

The public carrier namespace is `open_bitcoin_node::network::filter_transport`; flat node/network exports were avoided to keep the existing entrypoint within its managed file-length limit.

1. `handle.reserve_basic_filter_response(peer)` acquires the peer/request slot with a zero-byte suffix before output collection. It checks the live peer/config and returns an affine `FilterRequestLease`.
2. Capture the branch and complete guarded persisted reads with the existing finite work budget. Inside that publication guard, `reader.capture_basic_peer_epoch(meter)` charges one metadata unit and returns Plan13's opaque token.
3. **Release the publication and authority guards before calling** `handle.bind_basic_filter_response(&lease, &capture, epoch)`: this method acquires the authority itself and checks bounded in-memory currentness. It must not be called recursively while either guard is held. Peer and originating owner must match exactly.
4. `lease.resize_suffix(full_framed_bytes)` admits complete output retention before its allocation/encoding. `lease.prepare(Vec<Vec<u8>>)` checks nonempty frame count, maximum 1,000 frames, minimum wire-header length and exact admitted total; it then owns the immutable prepared response.
5. Plan08 must charge every additional actual payload/frame construction, checksum and copy effect through the existing meter before doing it; Plan07 supplies no encoding-budget bypass. Remove the narrowly documented temporary `cfg_attr(not(test), allow(dead_code))` annotations on the preparation methods and `reader.capture_basic_peer_epoch` once these production callers are wired.

## Exact Plan09/10 Transport Handoff

- Before **each** write call `handle.basic_filter_response_is_current(&response)`; it returns a typed failure for failed authority/registry/publication, false for revoked identity, and never acquires a socket or performs backend I/O.
- `response.next_frame(peer_other_pending, aggregate_other_pending)` returns one owned `PreparedFilterFrame`. `frame.bytes()` borrows its immutable complete encoded bytes. Write outside every context/authority/governance lock. Only after the existing complete-write success outcome may trusted first-party transport consume `frame.acknowledge_write()` and pass the resulting receipt to `handle.complete_basic_filter_frame(receipt)`.
- There is no public arbitrary-length acknowledgement or raw receipt constructor. The consuming no-argument acknowledgement follows the existing PeerEmission trusted-adapter boundary; Plans09/10 must prove their actual full-write callers and partial-write failures.
- `CurrentRangeComplete` is earned only by all intended full frames while current. `PhysicalPrefixInvalidated` records truthful full-frame bytes while denying live full success. `progress()` returns the factual prefix even after invalidation. A completed response yields `Ok(None)` from `next_frame`.
- On cancellation/error drop the current frame and consume `response.cancel()` or let owned carriers drop. Actual queued buffers are freed immediately; handed bytes/request admission remain charged until the frame owner settles. A dropped achieved receipt records physical-only prefix. Explicit cleanup errors must be retained alongside the original adapter write error, rather than replacing it.
- Typed faults are `AuthorityUnavailable`, `PublicationUnavailable` and registry `Poisoned`; failure never becomes ordinary stale/absence. Faulted registry snapshots remain typed errors instead of pretending to be empty. Test-only raw snapshots prove the actual resources are released.

### Mandatory mixed-traffic governance contract

The two pending-byte inputs are **actual other non-CF bytes**, excluding CF bytes already counted by the shared registry. `next_frame` adds every existing handed CF frame and applies a single combined 8MiB aggregate bound. Its pure arithmetic cannot atomically observe independent generic writers by itself.

Plans09/10 must perform the generic pending snapshot, CF admission and generic reservation transition under their **existing shared transport-governance serialization**, then release that guard before writing. Generic writers must honor the same boundary. Do not hardcode zero when other frames are pending, use a stale unsynchronized snapshot, double-count existing CF bytes, or allow separate 8MiB generic and CF pools. The mixed-pressure control holds 4,000,024 CF bytes, admits another 4,000,024 with exactly 388,560 other bytes, and refuses 388,561 other bytes. Real inbound/outbound mixed-traffic integration and its atomic ownership are explicit downstream obligations for final review.

## Deviations, Simplification and Scope

- The parent formally extracted monotonic freshness into prerequisite Plan13; this summary covers only the two Plan07 receipt/lease tasks. The store epoch implementation remains unchanged here apart from one temporary future-caller lint annotation.
- The parent approved the private `filter_transport/registry.rs` extraction when formatting exposed a 665-line contract. Current carrier/registry files are 512/299 lines, with a bounded shared identity lookup and no generic effect-ledger duplication.
- One test-only `pub(crate) mod tests` declaration in filters.rs reuses the existing persisted fixture instead of duplicating it. Its production visibility and behavior are unchanged.
- The existing generic lifecycle dispatcher already calls the shared readiness collector; it needed no separate source edit. Thin owner-hook wiring handles managed mutations, stop and unavailable authority without changing the existing wake algorithm.
- The simplification pass centralized terminal identity inspection and retained one capacity policy, one request sequence and the existing publication guard. Public API comments distinguish queued release from handed ownership. No dependency, crate, schema, filter store, endpoint, listener activation or socket wait was added.

AGENTS.md, the Bright Builds sidecar, placeholder-only overrides and architecture/code-shape/testing/verification/operability/Rust standards informed execution. Parent owns README/operator changes and final native/source/security verification for the entire phase.

## Security and Stub Review

- T-160-19: Private affine exact request/sequence/length receipts, compile-fail controls and replay/foreign/partial-write tests prevent a second live credit.
- T-160-20: Actual suffix Vecs and their container are dropped; handed-frame admission stays charged until settlement. Capacity cannot be reused early; cancellation, error, dropped receipt and poison paths preserve physical prefix and release reservations.
- T-160-21: Plan13 epochs and terminal proof identity deny lifecycle ABA; same-generation recovery, re-enable/reconnect, foreign axes and accepted-extension controls pass. Currentness performs no backend I/O and all transport remains outside authority locks.

No TODO/FIXME/placeholder implementation remains. The temporary lint annotations identify fully implemented APIs whose production callers belong to Plan08, rather than fake data or success stubs. No new security-relevant network endpoint, auth path, production filesystem pattern or trust-boundary schema was introduced.

All five new Rust paths use ordered pinned anchors `packages/bitcoin-knots/src/net.cpp`, `packages/bitcoin-knots/src/net_processing.cpp`, then `packages/bitcoin-knots/test/functional/p2p_blockfilters.py`.

## Task Commits and Shared State

Both tasks and this summary await the strict outer consolidated commit. This executor performed no staging, commit, push, branch, worktree, hook bypass or shared STATE/ROADMAP/REQUIREMENTS/todo/manifest mutation. Requirements-completed remains empty until the parent gate.

## Authentication Gates and Deferred Issues

None. Protocol encoding, real inbound/outbound writes, atomic mixed-traffic adapter governance and consolidated native verification remain explicitly assigned downstream plan work.

## Self-Check: PASSED

All five new Rust files, six modified paths and this summary exist. Final focused tests passed 23/23; existing readiness regressions passed 21/21; privacy controls passed 5/5; final strict node Clippy, scoped formatting, managed/parity checks and whitespace review passed. Commit-existence checks are inapplicable under the strict no-intermediate-commit lifecycle; no commit hash, socket verification or completed requirement was fabricated.
