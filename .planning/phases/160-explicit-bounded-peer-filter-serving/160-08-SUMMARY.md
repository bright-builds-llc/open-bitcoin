---
phase: 160-explicit-bounded-peer-filter-serving
plan: "08"
subsystem: network-storage
tags: [bip157, complete-preparation, fjall, resource-budgets, affine-handoff]
requires:
  - phase: 160-03
    provides: Configured general and permission-derived BASIC capability
  - phase: 160-05
    provides: Exact semantic authorization and requested-branch ancestry
  - phase: 160-06
    provides: Finite meters and complete guarded persisted reads
  - phase: 160-07
    provides: Affine bounded response leases and private receipts
  - phase: 160-13
    provides: Opaque monotonic shared publication epochs
  - phase: 160-15
    provides: Canonical pure request intents and preparation outcomes
provides:
  - Complete immediate persisted cfilter, cfheaders and checkpoint preparation
  - Normal sync, durable inbound and ordinary handle intent resolution
  - Charged encoding and guarded transfer of actual buffers into response ownership
  - Genuine branch, catch-up, absence, fault and concurrent invalidation evidence
affects: [160-14, 160-09, 160-10, 160-11, 160-12]
tech-stack:
  added: []
  patterns: [complete-before-emission, charge-before-encoding, guarded-buffer-transfer, live-peer-grant]
key-files:
  created:
    - packages/open-bitcoin-node/src/network/runtime_authority/filter_index/serving.rs
    - packages/open-bitcoin-node/src/network/runtime_authority/filter_index/serving/tests.rs
    - packages/open-bitcoin-node/src/network/runtime_authority/filter_index/serving/tests/families.rs
  modified:
    - packages/open-bitcoin-node/src/network/runtime_authority/filter_index.rs
    - packages/open-bitcoin-node/src/network/runtime_authority.rs
    - packages/open-bitcoin-node/src/network/runtime_authority/filter_index/branch.rs
    - packages/open-bitcoin-node/src/network/runtime_authority/filter_index/transport.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/filters/query/peer.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/filters/query.rs
    - packages/open-bitcoin-node/src/sync/tests/filter_index/peer_fixtures.rs
key-decisions:
  - Authority remains held after publication release until local frames transfer into registry ownership or are dropped; lifecycle revocation cannot release builder quota early.
  - Reserve the finite 8MiB worst-case suffix before allocation and shrink to exact measured framing while authority still serializes admission.
  - Current capability requires the captured advertised bit and a live general or effective per-peer grant.
  - All response materialization and encode_wire payload, frame and checksum work is admitted separately from persisted-read work.
requirements-addressed: [CFNET-01, CFNET-02, CFNET-03, CFNET-04, CFNET-05, CFNET-06]
requirements-completed: []
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 160-20261010-peer-filter-serving
generated_at: "2026-10-10T21:47:19Z"
duration: approximately 50min including formal prerequisite extraction and Plan15 execution
completed: 2026-10-10
---

# Phase 160 Plan 08: Complete Persisted Filter Preparation Summary

**All three persisted filter families now prepare complete requested-branch frames through ordinary managed handles, with finite encoding admission, live permission checks and quota ownership maintained through buffer transfer.**

## Accomplishments and Acceptance

- [x] Fixed semantic gates run before peer/aggregate quota refusal: BASIC plus the session's advertised capability, known/allowed stop, reverse check and inclusive family limit. The existing public branch-capture API retains its behavior through private semantic/resolution helpers; its four regressions pass.
- [x] One authority turn and one publication guard resolve every requested identity, validate exact target/parent/height commitments, capture the opaque publication epoch and encode the complete family. No public point-query loop, readiness wait, generation-on-request, history repair or socket I/O occurs.
- [x] Cfilters preserve complete forward order and exact stored bytes. Cfheaders preserve exact start-minus-one header, zero only at genesis, and raw filter hashes. Checkpoints preserve positive 1000/2000/3000 ancestry; stop999 produces one legitimate empty-vector response.
- [x] Legal already-indexed prefixes prepare during initial catch-up with synced=false. Accepted/index progress and full durable chainstate snapshots remain unchanged by serving.
- [x] NoBackend, Missing, ResourceRefused, StorageFailure and Invalidated remain typed sanitized unavailable outcomes. Present targets with missing/corrupt predecessors fail as storage failures; missing later targets/checkpoints expose no shortened prefix.
- [x] Both normal sync and durable inbound receive wrappers resolve Plan15 intents into affine prepared carriers. The ordinary receive wrapper also resolves them. Filter output never enters Immediate/outbound message vectors, and preparation begins with zero achieved-write evidence.
- [x] Live currentness uses the session's advertised CF bit AND (current general grant OR current effective per-peer grant). Preparation, binding, pre-write checks and the collector share that predicate. Permission-only all/explicit requests work with the global bit off; valid disabled-backend requests return NoBackend while semantic violations still disconnect.
- [x] Every additional response source copy, hash-vector copy, payload construction, full-frame construction, checksum input copy and padded SHA256d work is charged before its associated effect. Bounded frame-container ownership is charged too. The genuine complete encoding path passes exact/one-under controls for all eight work axes and exceeds the read-only copy/hash counters.
- [x] A request slot and 8MiB worst-case suffix are admitted before ancestry/record/output allocations. Exact suffix shrinking preserves typed registry failures. Authority remains held through publication release, exact shrinking, locked epoch binding and registry adoption of actual frames; error paths explicitly drop local frames before releasing their lease.
- [x] The deterministic handoff control starts concurrent disable/re-enable and later same-peer admission while complete frames are still local. It observes the actual held authority mutex and full charge, then proves the old terminal response cannot revive and only the later request remains charged.
- [x] All narrowly temporary preparation/peer-reader/epoch dead-code annotations were removed. The separate locking binder/reserver survive only as cfg(test) wrappers forwarding the same locked production helpers.

## TDD and Final Verification

All Cargo work used pinned Bun1.3.9 and the serialized timing runner with stable key phase160-filter-preparation. Required task filters executed nonzero tests. Every result below is from the final guard-retained implementation.

| Check | Actual evidence |
| -- | -- |
| Initial preparation RED | The new encoding-budget test failed compilation before the missing helper existed; its first GREEN executed one test. |
| Permission-only behavioral RED | One executed managed-dispatch test failed because lease currentness incorrectly required the global CF flag. The shared live per-peer/global predicate fixes it. |
| Handoff behavioral RED | One executed deterministic test failed its authority-held assertion while encoded buffers were local after the read guard ended. Guard-retained transfer fixes the witnessed capacity-lifetime gap. |
| Task1 required phase160_complete_preparation | **4 executed, 4 passed**, zero ignored, final execution 68.29s. Includes genuine 1000/2000 ranges and 3001/999 checkpoints in both directions, aggregate pressure/semantic precedence, checksum arithmetic and complete-path eight-axis exact/one-under controls. |
| Task2 required phase160_managed_dispatch | **1 executed, 1 passed**, final execution 2.52s. Exercises all/explicit permission-only capability, collector retention, ordinary and durable wrappers, all three disabled-backend families, malformed requests and actual semantic disconnect. |
| Task3 required phase160_managed_completeness | **6 executed, 6 passed**, final execution 15.04s. Includes genuine accepted/displaced fork A, reorg revocation, initial catch-up, final/middle gaps, predecessor/commitment/backend/OWNER faults, concurrent raw/config mutation and deterministic charged handoff. |
| Existing phase160_filter_ regressions | **23 executed, 23 passed**, including registry/authority/store poison, cancellation, exact receipts, late physical prefix, terminal ABA and failed-cleanup readiness stop. |
| Existing phase160_branch regressions | **4 executed, 4 passed**, preserving real ScriptsValid authorization, pruned stale identity, rejection precedence and metadata bounds. |
| Existing phase159_basic_readiness regressions | **21 executed, 21 passed**, preserving wake ordering, reentrancy, disable/reorg/raw invalidation, authority failure and stop/drop cleanup. |
| Final node Clippy, all targets/features, -D warnings | Passed. |
| Final scoped rustfmt, managed all, whitespace/diff review | Passed; zero managed findings and clean git diff --check. |
| Final parity check | Passed for **1086 Rust files**; parent serially registered all three new paths. |

Injected panic output comes from explicit existing poison controls, all passing. No full native workspace/Bazel/coverage or real socket verification is claimed here; the parent owns those final gates and Plans09/10 own actual adapters.

## Resource and Ownership Evidence

Persisted-row work from Plan06 is reused, rather than treated as encoding evidence. For a payload length p, encode_wire adds p payload bytes, p+24 framed bytes and p+32 checksum input copy bytes: 3p+56 logical copies. SHA256d padded work is ceil((p+9)/64)*64+64. Response-source/hash-vector copies and bounded frame container ownership are additional charges. These are finite logical work/ownership bounds, not allocator-capacity or RSS measurements.

The default metadata ceiling remains256 and suffix ceiling remains8MiB; no policy limit was raised. Branch work is pre-admitted using the remaining ancestry/header/copy allowances. The whole suffix is reserved as one request, independent of the generic effect count. A failed complete preparation releases all actual owned output and request charges.

The first exact-shrink draft collapsed registry errors into ResourceRefused; the final typed shrink distinguishes capacity, invalidation and internal failure. Review then identified that merely moving shrink/bind outside authority could retire an unbound reservation before locally owned frames were dropped. The final ownership transition retains authority through transfer, releases publication before memory-only epoch validation and performs no socket wait while either guard exists.

## Exact Downstream Handoff

1. Actual managed receive callers return Plan15's PreparedFilterServe(Box<PreparedFilterResponse>) only after complete persisted validation and encoding. FilterServeUnavailable carries a sanitized no-normal-response disposition. A leaked raw FilterRequestIntent is not ready output or completion authority.
2. Preparation now uses bind_basic_filter_response_locked under the retained authority guard, after the query publication guard ends. The older test binder acquires authority and forwards this helper; production must not recursively acquire authority during transfer.
3. Before every actual write, check basic_filter_response_is_current. Borrow frame.bytes(), write outside authority/publication/governance guards and acknowledge only the trusted writer's complete-frame success. Preparation itself earns no written prefix.
4. Plan14 still owns atomic mixed generic/CF transport admission and the replacement of caller-supplied pending-byte arithmetic. The isolated preparation tests have no other pending traffic and write only to std::io::sink for decoding/draining; they establish no inbound/outbound socket evidence. Do not hardcode zero pending bytes in actual adapters.
5. Plans09/10 must handle actual full writes, partial failures, cancellation and truthful physical-only prefix after invalidation. Existing Plan07 carrier/receipt regressions remain green; these production transport obligations are not completed by this plan.

## Deviations, Scope and Simplification

The parent formally extracted eight-path pure contract Plan15 before Plan08 crossed its original path budget. This revised plan owns ten Rust paths. The only additional child is the approved test-only serving/tests/families.rs; it keeps the family matrix and aggregate control readable without enlarging production APIs. The existing fixture module includes preparation tests through an explicit path, following the established query/readiness fixture pattern.

**[Rule1 - Live grant bug]** Existing transport currentness used only the global CF bit and rejected genuine permission-only sessions. The shared predicate now rechecks both independent grants plus captured advertisement/session authority; actual managed requests and the collector pass.

**[Rule1 - Builder reservation lifetime]** Review exposed unbound retirement while local frames remained owned. The deterministic RED/GREEN control verifies the corrected authority-held transfer and actual charge lifetime. Local frames drop before the lease on shrink/bind error.

**[Rule3 - Genuine reorg fixture inputs]** The first empty-disconnect/unflushed fork recipe correctly encountered the existing required-undo refusal. The corrected recipe genuinely accepts a nonempty fork A and performs its ordinary coins/undo flush before branch B displaces it. No serving request supplies missing undo, repairs history or forges ScriptsValid provenance. A test module path was made explicit after Rust's path-included module resolution exposed the nested child location.

The simplification pass centralized current grant inspection, kept one guarded reader and one existing lease/epoch mechanism, extracted the long family test into a focused helper and used local_config.magic directly to avoid cloning unrelated network-info data. No dependency, crate, store, schema, listener default, RPC wait path or body/TX serving redesign was added.

AGENTS.md, the Bright Builds sidecar, placeholder-only overrides, architecture/code-shape/testing/verification/operability/Rust standards, both active lessons and finalized context/research/coverage materially informed execution.

## Fixture Limits, Security and Stub Review

The continuous fixture uses genuinely validated finite histories and concrete Fjall durable coins, with maturity-one test parameters inherited from the established fixture. Fork A's ScriptsValid provenance comes from actual managed acceptance and survives actual managed replacement. Expected filter commitments come from the independently prepared historical fixture records; independent pinned wire-oracle and actual socket evidence remain Plan11/09/10 work. No full-scale/default-mainnet acceptance or production-readiness claim is made.

- T-160-22: Entire target/predecessor/checkpoint sets and encoded sizes validate before any prepared family is exposed; gaps, corruption and handoff invalidation release real buffers/charges.
- T-160-23: Permission-derived advertised capability remains distinct from backend presence; valid disabled-backend requests are no-response and invalid semantics disconnect in pinned order.
- T-160-24: Actual serving preserves accepted/index/durable progress and performs no readiness wait, owner-driven indexing, repair or socket write.

The production preparation scan found no TODO/FIXME/placeholder path, RPC point-query call, index-driving call or socket write. No new endpoint, authentication path, production filesystem access pattern or trust-boundary schema was introduced.

All three new Rust paths use ordered pinned anchors packages/bitcoin-knots/src/net_processing.cpp then packages/bitcoin-knots/src/index/blockfilterindex.cpp. Parent owns the manifest and its final audit.

## Task Commits, Shared State and Deferred Issues

All three tasks and this summary await the strict outer consolidated commit. No staging, commit, push, branch/worktree change, hook bypass or shared STATE/ROADMAP/REQUIREMENTS/todo/manifest mutation was performed by this executor. Requirements-completed intentionally remains empty.

No authentication gate occurred. A forced private-registry resize-poison injection is unavailable through the existing Plan08 seams; no new poison bypass was added. Existing typed registry-poison receipt regressions passed, and exact shrinking now uses the same explicit typed conversion instead of discarding errors. Atomic transport governance, actual socket adapters, independent evidence, native verification and final source/security review remain assigned downstream.

## Self-Check: PASSED

All ten owned Rust paths and this summary exist. Final required filters passed 4/1/6 tests; relevant regressions passed23/4/21. Final strict node Clippy, scoped formatting, managed/parity checks and whitespace/diff review passed. Commit-existence checks are inapplicable under the explicit no-intermediate-commit lifecycle; no commit hash, actual socket completion or completed requirement was fabricated.


## Post-review fixture alignment | 2026-10-11

WR-01's complete-handshake gate required the existing permission-only managed fixture and family fixture to send Verack after Version. The parent updated the former and Plan11 updated the latter in already-owned files. Direct preparation APIs retain their policy-test scope. Actual final inbound controls and outbound regressions passed; the full native gate remains pending.

## Workspace preflight fixture correction | 2026-10-11

The broad Rust preflight exposed three manual-peer fixtures that still omitted the handshake required by WR-01. The original node batch recorded1539passes/3failures/3ignored in611.26s; failures were absent prepared A, only1rather than4reservations, and an absent catch-up preparation. Root added only existing complete_peer_handshake calls in serving/tests.rs, serving/tests/families.rs and peer_fixtures.rs. Each existing assertion was preserved and passed individually (7.55s catch-up,0.96s aggregate pressure,1.22s genuine reorg). Production Rust and policy did not change. Full native verification on the final snapshot remains the outer gate.
