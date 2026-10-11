---
phase: 160-explicit-bounded-peer-filter-serving
plan: "14"
subsystem: network
tags: [mixed-pending, affine-buffers, exact-wire-sizing, encoding-scratch]
requires:
  - phase: 160-07
    provides: Shared affine filter transport registry and completed-frame receipts
  - phase: 160-08
    provides: Complete persisted preparation with authority-held buffer handoff
provides:
  - Atomic combined generic-buffer/scratch and CF-active admission in the existing registry
  - Exact allocation-free payload/frame sizing for every existing wire variant
  - Private owned encoded generic buffers retaining charges through queueing and writes
affects: [160-09, 160-10, 160-11, 160-12]
tech-stack:
  added: []
  patterns: [inseparable-buffer-reservation, bounded-descriptor-map, checked-stage-peak]
key-files:
  created:
    - packages/open-bitcoin-node/src/network/filter_transport/pending.rs
    - packages/open-bitcoin-node/src/network/filter_transport/pending/tests.rs
    - packages/open-bitcoin-node/src/network/filter_transport/encoded_size.rs
    - packages/open-bitcoin-node/src/network/filter_transport/encoded_size/tests.rs
  modified:
    - packages/open-bitcoin-node/src/network/filter_transport.rs
    - packages/open-bitcoin-node/src/network/filter_transport/registry.rs
    - packages/open-bitcoin-node/src/network/filter_transport/tests.rs
    - packages/open-bitcoin-node/src/network/runtime_authority/filter_index/transport.rs
    - packages/open-bitcoin-node/src/network/runtime_authority/filter_index/transport/tests.rs
    - packages/open-bitcoin-node/src/network/runtime_authority/filter_index/serving/tests.rs
    - packages/open-bitcoin-node/src/sync/tests/filter_index/peer_fixtures.rs
key-decisions:
  - Generic reservations and active CF frames use the same existing registry lock and combined 8MiB admission.
  - Private encoded Vec ownership remains inseparable from its affine reservation and drops before the charge.
  - Reserve the maximum actual simultaneously retained encoding stage before encoding; shrink only after encode_wire returns and drops scratch.
  - The final framing peak also includes the actual owned MessageCommand String identified in the primary serializer source.
  - The 4M outbound payload envelope is a new explicit local resource policy; the existing inbound 1MiB and 128-message gates remain independent.
requirements-addressed: [CFNET-06]
requirements-completed: []
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 160-20261010-peer-filter-serving
generated_at: "2026-10-11T01:14:10Z"
duration: approximately 18min
completed: 2026-10-10
---

# Phase 160 Plan 14: Atomic Mixed Pending Ownership Summary

**Generic encoded buffers, encoder scratch and handed CF frames now share atomic 8MiB admission, with exact borrowed sizing and private affine buffer ownership.**

## Accomplishments and Acceptance

- [x] The existing `FilterTransportOwner.registry` owns both traffic classes. Both admission orders use its checked mixed total under one mutex; no second authority, separate generic pool or caller snapshot exists.
- [x] `PreparedFilterResponse::next_frame()` accepts no pending-byte arguments. It reads actual generic/scratch and active CF charges internally. CF per-peer active bytes retain the 4,000,024-byte maximum; all classes share the 8MiB aggregate maximum.
- [x] Generic descriptors are bounded at128 per peer and1024 aggregate, including zero-byte reservations. Rejected IDs and peers create no entries. There is no separate per-peer map to leak empty state; final release removes the descriptor itself. IDs and byte sums use checked arithmetic.
- [x] `GenericPendingBuffer` owns a private Vec followed by its private reservation. Rust field drop order frees the actual encoded allocation before releasing its charge. The public surface only borrows bytes; it provides no clone, extraction, separate guard or raw-byte factory.
- [x] Queued/backpressured owners remain charged until their actual buffer drops. Failed/poisoned registries refuse new admission and remain visibly failed after cleanup. CF cancellation still frees only queued suffix buffers while handed frames remain charged.
- [x] Allocation-free preflight exhaustively matches all26 enum variants:25 encodable variants have independent payload/frame serializer equality, while receive-only malformed CF requests reject. CompactSize uses the codec MAX_SIZE and never invents a nine-byte outgoing width.
- [x] Scalar compact-block validation rejects empty/count/differential/out-of-bounds/null-prefilled structures without calling allocating expansion helpers. O(1) vector/minimum-size checks precede bounded traversal; raw scripts and witness stacks remain borrowed.
- [x] Reservations cover payload construction, nested transaction/address/header/inventory temporaries, compact validation vectors, both SHA stages and final framing. The final stage also retains the source-proven command String. Mutually exclusive SHA/frame stages are combined by maximum rather than summed.
- [x] Encoding happens exactly once after reservation. Only after `encode_wire` returns and its real scratch drops does the charge shrink to the retained exact frame. Failure and size-drift paths drop bytes before releasing the reservation.
- [x] The generic factory rejects CFilter, CFHeaders and CFCheckpt with `FilterResponseRequiresLease`, preventing bypass of request-bound preparation/receipts. All malformed CF kinds reject without entering the encoder. Generic writes produce no filter receipt.
- [x] Outbound payloads receive a new explicit local4,000,000-byte envelope plus24 framing bytes. A full contextually validated exact4,000,000-weight witness block serializes and is admitted by the actual guarded encoder. Codec-only4M succeeds and4M+1 gets typed `LocalPayloadLimit` with no reservation.
- [x] Tiny Ping/Verack peaks are72/64 bytes. GetBlockTxn with55 payload bytes has a145-byte peak including its11-byte command String. Actual CF pressure plus exact/one-under generic headroom proves these peak-sized reservations.
- [x] Shared handle clones see the same buffers and cleanup.32 barrier-controlled races never admit both competitors when their actual combined charge exceeds8MiB. Cancellation does not uncharge live generic or handed CF allocations.

## TDD and Verification

Every Cargo command used pinned Bun1.3.9 through `scripts/command-timings.ts` and the checkout build lock. No Cargo jobs from this executor overlapped.

| Check | Actual evidence |
| -- | -- |
| Task1 RED, key phase160-mixed-pending-red | New tests failed compilation because reserve_pending and generic snapshot fields did not exist. |
| Task2 RED, key phase160-pending-encoding-red | New exact-peak tests failed compilation because wire_size did not exist. |
| Task3 API migration RED | Removing snapshot arguments exposed two existing fixture calls with E0061; the parent approved their narrow migration before editing. Atomic API replacement was interleaved with Task1 to test both traffic classes together; no separate failing concurrency baseline is claimed. |
| Final Task1 filter phase160_mixed_pending | **5 executed, 5 passed**, zero ignored. Exact/one-over,128/1024 zero-byte descriptors, overflow/ID exhaustion, retained-buffer failure and poison/drop controls. |
| Final Task2 filter phase160_pending_encoding | **15 executed, 15 passed**, zero ignored. Exhaustive serializers, CompactSize/witness/structure boundaries, source-derived scratch, CF bypass refusal, error cleanup, tiny/command headroom, local envelope and maximum valid weight. Repeated after the final explicit over-envelope factory assertion. |
| Final Task3 filter phase160_mixed_admission | **5 executed, 5 passed**, zero ignored. Both orders, exact/one-over,32 barrier races, reverse-order scratch refusal, actual shared clones and failed-owner cleanup. |
| Existing phase160_filter_ regressions | **23 executed, 23 passed**, preserving leases, receipts, cancellation, physical-only prefixes, poison and terminal ABA. |
| Existing phase160_complete_preparation regressions | **4 executed, 4 passed**,70.71s; includes complete genuine legal families and eight-axis exact/one-under work. |
| Existing phase160_managed_ regressions | **7 executed, 7 passed**,16.04s; includes permission-only dispatch, faults, reorg, catch-up and authority-held local-buffer handoff. |
| Privacy cargo test --doc filter_transport | **7 executed, 7 passed**, including new non-Clone/private-buffer extraction controls plus the five earlier lease/receipt controls. |
| Strict node Clippy, all targets/features, -D warnings | Passed. |
| Scoped rustfmt, managed all, whitespace and diff review | Passed; zero managed findings and clean git diff --check. |
| Breadcrumb check | Passed for1090 Rust paths after parent serial registration of the four new files. |

The first variant-table assertion expected24 instead of25 encodable variants; it was corrected. Source visibility/import mismatches were corrected during compilation without widening unrelated exports. Injected panic output comes from passing poison tests. A quiet70-second preparation run was polled through its resumable session; no process was terminated because of an estimate.

## Exact Plan09/10 Handoff

1. Call `handle.encode_pending_network_message(peer, &message)` for every ordinary outgoing message. It returns `Result<GenericPendingBuffer, FilterTransportError>`. The method copies this handle's configured network magic under its authority lock, releases that lock, preflights sizes, then reserves/encodes through the SAME shared filter owner.
2. Store the returned owner in the actual pending queue. Borrow only `buffer.bytes()` for the complete write. Keep the owner alive across queued suffix retention, backpressure, every partial-write retry and every socket wait; drop it only once its actual bytes are no longer retained. Never copy/extract its Vec, encode again or release a parallel guard early. No authority/publication/registry lock is held during the actual encoding or socket write.
3. Keep inbound's existing1MiB/128-message local generic gates in addition to this shared accounting. Outbound's4M payload plus24 frame envelope is new local resource policy, not a retrospective1MiB outgoing limit or a new pinned protocol claim. Synthetic larger codec-only messages receive a local typed refusal.
4. For filter replies retain Plan08's authority-held read→publication release→typed exact shrink→locked bind→registry frame adoption sequence. This plan does not move or weaken preparation handoff or permission checks.
5. Immediately before each actual CF write check `handle.basic_filter_response_is_current(&response)`, then use `response.next_frame()` with no arguments. It admits against actual mixed registry bytes atomically. Write `frame.bytes()` outside locks; acknowledge only the trusted writer's complete-frame success, then pass that private receipt to `handle.complete_basic_filter_frame`. Generic buffers have no CF receipt or success authority.
6. Encoder peak and final retained size are different. Generic-first maximum-payload encoding can shrink to4,000,024 bytes before a CF frame is admitted. CF-first can correctly refuse that same generic encoding because its roughly8M scratch peak plus live CF bytes exceeds8MiB, even though the two final frames would fit. Never uncharge live bytes to force equal outcomes across these stage orders.
7. Raw registry ownership tests prove exact8MiB steady mixed boundaries in both orders using real test-owned allocations. Full factory tests separately prove actual stage-peak admission and tiny-message headroom. These are bounded memory tests, not real inbound/outbound socket evidence; Plans09/10/11 must exercise the same owners on both actual transports.
8. Public snapshots now include `generic_pending_bytes` and `generic_descriptors` alongside requests, suffix bytes and active CF bytes. The first counter includes admitted scratch while encoding and exact retained frame lengths afterward. Snapshots are observations; they are never admission inputs. Failure remains a typed error rather than a false empty snapshot.

## Maximum-Weight Fixture Scope

The body fixture has actual mined easy-target PoW, correct transaction and witness merkle commitments, BIP34 height1, a valid subsidy, an exact4,000,000 consensus weight and a successful `validate_block_with_context`. Its large witness spends a supplied non-coinbase unknown-v2 witness-program UTXO under P2SH|WITNESS|TAPROOT consensus flags. Unknown-version witness data is consensus-valid here; this is not a mempool standardness claim.

The context supplies the spent output and easy-target previous header. This test does not prove a continuous historical/durable chain acceptance, public-mainnet sync, wallet/funds behavior or a real socket write. It does prove that a genuinely contextually valid maximum-weight witness body fits the finite new generic envelope and guarded encoder. Independently, the consensus relation total_wire_body+3*stripped_body<=weight<=4M preserves every valid in-scope body within the local4M payload policy.

## Deviations, Scope and Simplification

**[Rule3 - Existing API caller]** Parent explicitly approved the eleventh existing path `sync/tests/filter_index/peer_fixtures.rs` after the no-argument API exposed two historical snapshot calls. Those two calls changed to next_frame(); their underlying fixture behavior was preserved and the managed/preparation regressions passed. No additional source path or15-file replan threshold was reached.

**[Rule2 - Encoder temporary accounting]** Primary source review found that encode_wire retains its owned MessageCommand String in the MessageHeader through final append. Preflight now includes those actual command bytes in final-stage peak2P+24+command_len; parent updated the planning audit. Exact/one-under55-byte GetBlockTxn evidence covers the difference. Ping72, Verack64 and4M8,000,064 peaks remain unchanged because SHA dominates them.

The serializer's private64-address limit is mirrored narrowly with its source-named comment and independent codec limit/one-over tests, rather than adding an unrelated network export. No header outgoing-count limit was invented. Compact scalar validation duplicates only the existing allocation-producing checks needed before reservation; the serializer still validates independently afterward.

The explicit simplification pass retained one existing mutex, one checked mixed fold, one bounded descriptor map, one private affine reservation and one encode_reserved helper. It avoided a per-peer state map, a second authority, a second generic8MiB pool and an optional encoder rewrite. Diff review confirmed no header/work/permission semantic change, socket activation, dependency, crate, storage schema or unrelated body/TX redesign.

AGENTS.md, the Bright Builds sidecar, placeholder-only overrides, architecture/code-shape/testing/verification/operability/Rust standards, active lessons and the finalized generic source audit informed execution.

## Security, Stubs and Shared State

T-160-38 is covered by both admission orders, exact/one-over, descriptor cardinality, overflow, poison,32 controlled races, scratch peak checks and the live-buffer lifetime controls. CF factory categorization prevents generic responses from forging prepared request authority. No new network endpoint, authentication path, production filesystem pattern or trust-boundary schema was introduced. No TODO/FIXME/placeholder production implementation remains.

All three task changes and this summary await the strict outer consolidated commit. This executor performed no staging, commit, push, branch/worktree change, hook bypass or shared STATE/ROADMAP/REQUIREMENTS/todo/manifest edit. Requirements-completed intentionally remains empty. Parent owns final native verification, independent review, requirement completion and git finalization.

The four new source/test files have parity comments and were serially registered by the parent. Pending modules use ordered net.cpp/net.h/net_processing.cpp anchors; encoded-size modules use protocol.cpp/serialize.h/net.cpp, all under packages/bitcoin-knots/src/.

No authentication gate or deferred implementation issue remains. Actual transport adoption, independent socket evidence and consolidated native verification remain assigned downstream work.

## Self-Check: PASSED

All eleven owned Rust paths and this summary exist. Final required filters passed5/15/5 tests; earlier filter/preparation/managed regressions passed23/4/7 and privacy controls passed7. Strict node Clippy, scoped formatting, managed rules, parity and diff checks passed. Commit-existence checks are inapplicable under the explicit no-intermediate-commit lifecycle; no commit hash, socket completion or requirement completion was fabricated.
