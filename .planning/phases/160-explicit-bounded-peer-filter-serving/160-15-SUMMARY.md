---
phase: 160-explicit-bounded-peer-filter-serving
plan: "15"
subsystem: network
tags: [bip157, dispatch, affine-carriers, relay-isolation]
requires:
  - phase: 160-07
    provides: Affine prepared response carriers and exact lease identity
provides:
  - Exact pure ServeBasicFilter request actions and ordered managed intents
  - Canonical preparation and unavailable outcomes in filter_transport
  - Explicit inert handling in relay-only and legacy parity consumers
affects: [160-08, 160-14, 160-09, 160-10, 160-11, 160-12]
tech-stack:
  added: []
  patterns: [intent-is-not-authority, exhaustive-relay-isolation, canonical-carrier-ownership]
key-files:
  created: []
  modified:
    - packages/open-bitcoin-network/src/peer.rs
    - packages/open-bitcoin-network/src/peer/message_dispatch.rs
    - packages/open-bitcoin-network/tests/parity.rs
    - packages/open-bitcoin-node/src/network/action_translation.rs
    - packages/open-bitcoin-node/src/network/types.rs
    - packages/open-bitcoin-node/src/network/filter_transport.rs
    - packages/open-bitcoin-node/src/network/admission_bridge.rs
    - packages/open-bitcoin-node/src/network/runtime_authority/filter_index/serving.rs
key-decisions:
  - Pure carrier outcomes live in filter_transport; durable preparation imports them without a reverse implementation dependency.
  - Filter actions remain explicit ignored cases in transaction-only and legacy parity consumers, retaining exhaustive future checks.
requirements-addressed: [CFNET-02, CFNET-03, CFNET-04, CFNET-05, CFNET-06]
requirements-completed: []
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 160-20261010-peer-filter-serving
generated_at: "2026-10-10T21:19:30Z"
duration: approximately 8min excluding the preceding scope-review pause
completed: 2026-10-10
---

# Phase 160 Plan 15: Pure Filter Dispatch Contracts Summary

**Three parsed filter families now cross the managed boundary as ordered typed intents, while explicit relay-only consumers remain inert and canonical outcomes avoid a reverse durable-preparation dependency.**

## Accomplishments and Acceptance

- [x] `GetCFilters`, `GetCFHeaders` and `GetCFCheckpt` preserve raw type, start and stop through `PeerAction::ServeBasicFilter(FilterRequest)`. Unsupported values remain untrusted intent fields for Plan08's ordered semantic gates.
- [x] Malformed complete request payloads and unsolicited response families remain inert. Unknown peer IDs produce the existing typed unknown-session error instead of an intent.
- [x] General action translation preserves exact filter-intent request order, emits no normal wire messages and performs no index/resolver or achieved-write effect.
- [x] `ManagedInboundResponsePlanItem` carries `FilterRequestIntent`, affine `PreparedFilterServe` and typed `FilterServeUnavailable`. A parsed intent cannot supply an affine response or receipt.
- [x] `BasicFilterPrepareOutcome` and `BasicFilterServeUnavailable` are defined canonically in `network/filter_transport.rs`; serving imports them. The former reverse serving reexport is removed. Prepared carrier equality compares the private affine owner Arc identity without adding Clone authority.
- [x] `transaction_relay_messages` explicitly ignores filter actions alongside other non-transaction actions. A transaction request between two filter actions remains exactly unchanged.
- [x] The legacy network parity runner explicitly ignores the new action, preserving its existing block/inventory-only fixture role.
- [x] Narrow shared serving ownership was limited to outcome-definition removal/import migration, the witnessed `Invalid(_reason) | ResourceRefused(_)` E0408 binding correction, and formatting. Plan08's preparation draft is preserved and remains unfinished here.

## TDD and Verification

All Cargo commands used pinned Bun1.3.9 and the timing runner, with no overlapping Cargo job from this executor.

| Check | Actual evidence |
| -- | -- |
| Task1 RED, required `phase160_dispatch_contract` filter | Integration compilation exposed the missing exhaustive `ServeBasicFilter` arm in `network/tests/parity.rs`. No task success was claimed from the incomplete compilation. |
| Task1 GREEN, same required filter | **3 executed, 3 passed**, zero ignored. Identity preservation, malformed/unsolicited inert behavior and unknown-session rejection run without a serving backend. |
| Task2 RED, required `phase160_dispatch_bridge` filter | Node compilation reproduced the missing exhaustive `ServeBasicFilter` arm in `admission_bridge.rs`. |
| Task2 final GREEN, same required filter | **2 executed, 2 passed**, zero ignored, repeated after the final test-module placement and Copy cleanup. |
| Existing `phase160_wire_dispatch` regressions | **4 executed, 4 passed**, preserving short complete payload behavior, trailing request bytes, broken frames and strict response/non-CF parsing. |
| Strict Clippy, network and node, all targets/features, `-D warnings` | Passed. The initial lint run identified test-module placement and a redundant Copy clone; both were fixed within the owned bridge file. |
| Scoped rustfmt check, managed `all`, whitespace/diff review | Passed. Managed checks reported zero findings; `git diff --check` is clean. |

The integration binaries have zero matching tests under these focused filters; the nonzero unit counts above are the required evidence. Consolidated native verification, source/security review and actual socket behavior remain downstream gates.

## Exact Plan08 Handoff

- Canonical outcomes are exported directly from `open_bitcoin_node::network::filter_transport`.
- `BasicFilterPrepareOutcome::{InvalidDisconnect, Unavailable, Prepared}` separates protocol rejection from local absence/resource/storage/invalidation and affine immutable output.
- `BasicFilterServeUnavailable::{NoBackend, Missing, ResourceRefused, StorageFailure, Invalidated}` authorizes no normal wire message.
- The exact managed variants are `FilterRequestIntent(FilterRequest)`, `PreparedFilterServe(Box<PreparedFilterResponse>)` and `FilterServeUnavailable(BasicFilterServeUnavailable)`.
- Plan08 alone resolves intents in `runtime_authority.rs`, performs complete persisted preparation and binds live peer/publication identity. This prerequisite does not establish that the paused preparation draft works, that transport is integrated, or that any requirement is complete.

## Deviations and Simplification

The parent formally extracted these contracts from Plan08 before the latter crossed its path budget. Existing drafts were retained and assigned to this checked prerequisite rather than reverted or concealed as parent metadata.

**[Rule3 - Exhaustive integration fixture]** The required network test command builds `tests/parity.rs`, which had another exhaustive action consumer. The parent explicitly added that eighth existing path before editing. Its one inert arm closes the compile failure without extending the legacy fixture into a filter-serving implementation.

The simplification pass kept direct typed mappings and one canonical outcome location. No conversion framework, dependency, crate, resolver abstraction, storage owner or wildcard suppression was added. Diff review found no unintended changes to ordinary relay behavior.

AGENTS.md, the Bright Builds sidecar, placeholder-only overrides, architecture/code-shape/testing/verification/operability/Rust standards, active lessons and finalized context/research/coverage informed this work. The parent owns shared planning state and final source registration.

## Security and Stub Review

T-160-39 is covered by exact raw identity, inert malformed/unsolicited/unknown-session behavior, ordered intent-only translation and explicit relay isolation tests. No new endpoint, auth path, production filesystem access pattern or trust-boundary schema was introduced. No placeholder/TODO/FIXME implementation was added. Plan08's preserved draft is explicitly not counted as this plan's completed durable behavior.

No new Rust file was created by Plan15. The existing Plan08 draft serving path and its eventual breadcrumb registration remain Plan08/parent responsibilities.

## Task Commits, Authentication and Deferred Issues

Both tasks and this summary await the strict outer consolidated commit. No staging, commit, push, branch/worktree operation, hook bypass, STATE/ROADMAP/REQUIREMENTS/todo or manifest mutation was performed. Requirements-completed remains empty. No authentication gate occurred.

Plan08 preparation, Plan14 atomic mixed-traffic governance, both actual socket adapters and final native/review gates remain separately assigned work.

## Self-Check: PASSED

All eight owned source paths and this summary exist. Final required filters passed 3/3 and 2/2 tests; four existing wire regressions passed. Strict all-target network/node Clippy, scoped formatting, managed checks and whitespace review passed. Commit-existence checks are inapplicable under the explicit no-intermediate-commit lifecycle; no commit, physical socket evidence or requirement completion was fabricated.

## Post-review remediation | 2026-10-11

Independent source review WR-01 found that parsed CF requests could authorize an intent before VERSION/VERACK completion. The parent corrected all three existing dispatch arms to check the existing complete-handshake predicate after unknown-peer lookup. One new executed regression failed before the fix; four dispatch-contract tests passed afterward. The existing identity test now performs an actual Version/Verack exchange. Plan11 supplies actual inbound pre-VERSION/pre-VERACK valid/invalid-family controls; its four final library tests passed. No new source path or serving capability was added. Final native verification remains the outer gate.
