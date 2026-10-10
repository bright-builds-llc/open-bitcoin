---
phase: 159-authenticated-basic-filter-and-index-rpcs
plan: "06"
subsystem: authenticated-rpc
tags: [basic-filters, shared-dispatch, authentication, raw-parameters, async-readiness]
requires:
  - phase: 159-02
    provides: Pure pinned request normalizers and exact result projections
  - phase: 159-05
    provides: Same-authority queries and request-bound captured-frontier completion
provides:
  - Baseline node-scoped registration and actual shared getblockfilter/getindexinfo dispatch
  - Authenticated duplicate-preserving HTTP prepare/await/complete outside request locks
  - Actual daemon maintenance failure/shutdown settlement of pending reads
affects: [159-07, 159-08]
tech-stack:
  added: []
  patterns: [typed pending dispatch, scoped raw named parameters, fixed public failures]
key-files:
  created:
    - packages/open-bitcoin-rpc/src/context/filter_index.rs
    - packages/open-bitcoin-rpc/src/dispatch/filter_index.rs
    - packages/open-bitcoin-rpc/src/dispatch/filter_index/tests.rs
    - packages/open-bitcoin-rpc/src/http/filter_index.rs
    - packages/open-bitcoin-rpc/src/http/tests/filter_index.rs
    - packages/open-bitcoin-rpc/src/http/tests/filter_index/fixtures.rs
  modified:
    - packages/open-bitcoin-rpc/src/method.rs
    - packages/open-bitcoin-rpc/src/method/tests.rs
    - packages/open-bitcoin-rpc/src/context.rs
    - packages/open-bitcoin-rpc/src/dispatch.rs
    - packages/open-bitcoin-rpc/src/http.rs
    - packages/open-bitcoin-rpc/src/http/request.rs
    - packages/open-bitcoin-rpc/src/http/tests.rs
    - packages/open-bitcoin-rpc/src/bin/open_bitcoind/coins_flush.rs
key-decisions:
  - Keep Pending typed through HTTP and complete the original requested hash on the captured same-authority handle.
  - Validate the complete JSON document and existing request envelope before duplicate-name semantics.
  - Keep unrelated methods on existing collapsed-map parameter and transport behavior.
  - Retain the first maintenance failure through shutdown while attempting final coins flush and readiness stop.
patterns-established:
  - Clear request wallet scope and release the context mutex before every readiness await.
  - Readiness completion authorizes one checked original-hash read, never recapture or request-owned indexing.
requirements-addressed: [CFRP-01, CFRP-02]
requirements-completed: []
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 159-2026-10-09T16-11-49
generated_at: "2026-10-09T18:36:33Z"
duration: approximately 23min including coordinated Cargo-slot wait
completed: 2026-10-09
---

# Phase 159 Plan 06: Authenticated Shared Filter RPC Summary

**Authenticated BASIC queries now use the configured shared authority, preserve raw duplicate names, and await the ordinary owner outside request locks with exact pinned result/error projection.**

## Accomplishments

- All three implementation tasks are complete with scoped verification; task commits remain pending root's consolidated gate. Six new Rust files and eight existing-source changes are listed exactly above. The initial executor timestamp was not separately recorded; the first timing record is 18:13:57 UTC and the genuine dispatcher RED began at 18:14:18 UTC.
- Added GetBlockFilter/GetIndexInfo to the complete method registry, name lookup, serde names, baseline origin, node scope, typed MethodCall and dedicated normalizer routing together with real dispatch handlers. Memory/default authorities remain disabled; no production dependency, index owner, independent store/cache, frontend or activation default was added.
- BASIC dispatch clones the context's existing ManagedNetworkHandle. Found uses exactly lowercase filter/header, reversing only the raw header for uint256 display order. Missing NeverConnected, initial ScriptsValid and ready ScriptsValid produce the pinned -5/-1/-32603 messages. UnknownLegacy absence and every authority/storage/readiness failure use fixed -32603 `Block filter index is unavailable`, without formatting internal error details. V0 returns its recognized disabled-index failure before backend access.
- getindexinfo projects only `basic block filter index` with exactly `synced` and `best_block_height`. Unmatched selection returns an empty object before backend access; absent/null/empty/exact BASIC selection follows the existing helper. Summary readiness remains the initial latch and height remains processed progress, independently of the safe durable checkpoint.
- Scoped serde visitors retain parameter-object pairs until the method is known. Only the two new normalizers receive raw pairs; unrelated methods retain Value map semantics. The whole document must deserialize before any request executes, and the existing envelope decoder runs before duplicate semantic checks. Raw duplicate names in both methods, batches, malformed later syntax and missing-method envelopes have actual HTTP evidence.
- HTTP still authenticates before parsing or acquiring the context. Typed Pending captures the original request and authority under the brief context lock; request wallet scope is cleared and the guard dropped before awaiting the std Future. The shell reacquires the context and consumes completion with the original hash on the same handle. It performs no indexing, retry loop, recapture, timeout-based public classification or forced coins flush.
- The existing daemon maintenance loop calls stop_basic_filter_readiness on scheduled/periodic failure and shutdown, even when final coins flushing fails. It retains the first failure across later ticks, attempts both final flush and stop, reports secondary failures, and preserves existing join/clean-marker coordination. Maintenance Display/Debug and HTTP auth Debug redact private details while the internal maintenance error retains its typed source.

Material guidance: AGENTS.md, Bright Builds sidecar, placeholder-only overrides and architecture/code-shape/testing/verification/Rust standards; both active lesson files were completely loaded (7,188 bytes, 2,397 conservative estimated tokens, no audit trigger). Approved CONTEXT/RESEARCH/PLAN-CHECK/nonvisual UI contract and actual Plan02/04/05 summaries/APIs informed integration. The parent supplied a synchronized main checkout and materialized pinned Knots. No skill demanded an additional approval gate.

## Verification Evidence

All Cargo commands used pinned Bun PATH and the timing wrapper, with serialized target access. Final checks ran after the node owner's stable WR03 poison-wakeup fix. No staging, commit, push, hook bypass, native full verifier, or root STATE/ROADMAP/REQUIREMENTS/todo/config mutation was performed.

| Check | Evidence |
| --- | --- |
| phase159-dispatch-red, corrected behavioral RED | One discovered test failed MethodNotFound before registry/real dispatch. Initial test import/construction errors were corrected first and are not claimed as behavioral RED. |
| phase159-dispatch | One initial dispatcher test passed after atomic integration. |
| phase159-http-red, corrected behavioral RED | One discovered raw duplicate test failed because Value maps collapsed duplicate index_name keys. Initial sibling-test visibility errors were corrected before this genuine RED. |
| phase159-http-compile | Initial raw HTTP duplicate test passed. |
| phase159-http-lock-red mutation | One discovered concurrent-summary test failed its bounded timeout when the context guard was deliberately retained across readiness await. The mutation was restored immediately. |
| phase159-rpc-regression | 320 library tests passed, zero failed/ignored; 42.15s execution. The first broad run found only the existing registry snapshot missing the two new names; the two-line snapshot update preceded this GREEN. |
| phase159-rpc-final | 32 phase159 library tests passed, zero failed/ignored, 288 filtered; 12.29s execution. Includes 18 Plan02 tests and 14 new dispatcher/HTTP tests. |
| phase159-http-final-envelope | Final nine HTTP tests passed, including full-document syntax and missing-method precedence over raw duplicates. |
| phase159-maintenance | 14 actual maintenance tests passed, zero failed/ignored, 51 filtered; 10.13s execution. Includes two new Plan06 tests and existing Always/no-extra-turn, all-worker settlement, retained-failure and no-clean-marker regressions. |
| phase159-rpc-clippy and final recheck | RPC all-targets/all-features --no-deps -- -D warnings passed, including the final envelope test; no warning suppression. |
| Scoped Rust 1.94.1 formatting and diff checks | Passed on all 14 owned/approved-glue source paths. |
| Bright Builds all | Zero findings; new files also individually measured below 628 lines. |

Authentication controls spawn independent unauthorized malformed requests while holding the shared Tokio context guard, observe bounded completion, then release the guard before assertions. Missing/wrong credentials both return identical empty 401 responses. Cookie and password requests succeed; HTTP state debug exposes neither username nor password. Parser/transport evidence covers positional/named/null, duplicate blockhash/index_name, mixed collision, legacy/v2 IDs/statuses, empty/nonempty batches, notifications and wallet-path rejection.

The initial fixture genuinely accepts forty continuous blocks from an empty store through the production shared network, flushes the real coins fence, closes/reopens configured BASIC, and observes an available genesis row while summary remains false at processed height 7. An unavailable genuinely accepted later row returns the exact initial-indexing error; unknown hash returns exact -5 Block not found. No scripts-valid bit is seeded.

The later-lag fixture accepts an unflushed successor after initial sync, polls the actual HTTP request to Pending, successfully executes a separate summary request, then drives the ordinary owner independently. Returned bytes/header equal the same authority's immutable row, the original string ID remains intact, processed height advances to 2 and the safe durable prefix remains at 1. A deliberately changed original hash passed to the completion adapter is rejected with the fixed public error. Test-only timeouts detect hangs; none grants production readiness.

Terminal controls cover explicit stop, disable, real validated reorg and producer/fence failure. The failure uses the public coins adapter to write an unrelated durable marker after Pending capture, invalidating append authority; the actual scheduled turn refuses publication and settles the HTTP wait. Genuine accepted input is preserved. Reorg retains the actual accepted undo before replacement without a coins fence. Responses contain neither datadir nor credential strings. Injected storage/backend/authority private-marker errors are also checked against public RpcFailure Debug and the fixed error boundary.

## Issues Encountered

The first failure fixture incorrectly assumed omitting an explicit body save would fail a later owner turn. Production acceptance persistence and retained acceptance facts correctly made progress. A codec-valid overwrite likewise did not establish the required decoder fault. Those fixture attempts were removed; the final producer/fence mutation is explicit and demonstrably terminal. No production cache or input validation was weakened to force a fault.

Two parallel fixtures obtained the same host SystemTime timestamp and collided on Fjall's database lock. Temp naming now includes process ID, timestamp and an independent atomic suffix. The reorg fixture also needed to persist its genuine accepted undo before replacement; it now does so without advancing coins durability. These are setup corrections, not evidence of broken production RPC semantics. No loader termination or blind retry was needed in this plan.

## Deviations from Plan

1. **Rule3: Reusable bounded test fixture child.** Parent approved `packages/open-bitcoin-rpc/src/http/tests/filter_index/fixtures.rs` to keep substantive consensus/history setup out of the HTTP test root. It uses external node/core imports only and introduces no production fixture API. New Rust breadcrumb registration remains Plan08-owned.
2. **Rule3: Existing registry snapshot glue.** The full library regression required adding the two registered names to `packages/open-bitcoin-rpc/src/method/tests.rs`. Only that expected-list block changed; the parent was notified with the exact path.
3. **Rule2: Terminal maintenance failures must remain honest.** Within the owned coins_flush.rs, periodic errors now propagate to the same stop handoff and the first failure remains retained through later ticks and final shutdown. Both final effects are attempted and secondary failures remain visible; existing shutdown/join/clean-marker regressions pass.

The explicit simplification pass retained a small pure missing-row classifier used by real dispatch, reused Plan02 projections and the existing shared authority, kept nonfilter calls on their prior synchronous implementation, and isolated only raw-parameter/async glue in the child adapter. No generalized RPC framework, alternate storage, request worker, additional index owner or retry mechanism was introduced. Existing root files and every new child remain under 628 lines.

## API and Source Handoff to Plan07

- Public handle_http_request/router signatures are unchanged; existing shared-context construction automatically reaches both real methods. Use the same opened daemon authority when building ManagedRpcContext; an unrelated metrics store grants no BASIC authority.
- SupportedMethod::{GetBlockFilter,GetIndexInfo} and MethodCall variants are fully wired as baseline/node methods. Pure dedicated normalizers remain the Plan02 exports.
- Crate-internal dispatch/filter_index::prepare returns PreparedDispatch::{Complete,Pending { request,barrier }}. HTTP consumes Pending; existing synchronous dispatch supports completed calls and explicitly refuses Pending with a fixed asynchronous-dispatch internal error instead of diagnosing corruption or inventing a result. Clients requiring readiness should use the authenticated async HTTP path.
- http/filter_index::finish awaits the existing barrier with no context guard, reacquires context, and calls the captured network.complete_basic_filter_read(request.block_hash,completion). It never requests a newer frontier. Lifecycle changes return one fixed terminal error rather than retry forever.
- The new test-only fixtures.rs can be included by path from another test owner. History::new(count) exposes path, genuinely accepted blocks and configured runtime; accept_next() accepts/persists one unflushed successor; cleanup(self) requires callers to drop their external context/handle/store clones first. next_block and params are external-import-only helpers. Test consensus uses maturity one and easy local headers; no standard-maturity or public-mainnet claim follows.
- Plan07 retains actual daemon configuration/startup/shared-dispatch proof, stale retrieval, real paired-prune/all-handle reopen and its remaining fault matrix. This plan does not substitute its smaller HTTP fixture for those gates and does not start Plan07.
- Plan08 owns manifest registration for all six new Rust files above, parity/docs/README integration, full native/Bazel verification and final source/security/lifecycle acceptance. Root owns all Git finalization and requirement completion.

## Task Commits and Limits

1. Task1 — shared context/registry/dispatch and pinned classification: pending root consolidated commit.
2. Task2 — scoped raw parameters, async readiness and maintenance handoff: pending root consolidated commit.
3. Task3 — authenticated transport/concurrency/terminal regression evidence: pending root consolidated commit.

No authentication gate or external setup was needed. No requirement is marked complete. Source-derived exactness remains anchored to the pinned Knots implementation and the previously verified Plan02 whole-help fixtures; this plan does not claim a running Knots comparison or production/funds readiness. There is no new public/default activation, peer serving, V0 implementation, repair/download behavior or production storage reopen.

## Known Stubs

None. Empty objects implement the required disabled/unmatched summary result; empty vectors in the scoped deserializer are internal document-shape carriers, while actual batch requests use their retained children. No unimplemented handler, mock production data source, TODO/FIXME or fabricated readiness remains.

## Security Handoff

T-159-19 has auth-before-lock/parse and wallet-scope controls; T-159-20 has raw pairs plus syntax/envelope/semantic precedence; T-159-21 has lock-retention mutation detection, same-handle completion and actual owner-stop/failure settlement; T-159-22 has fixed error projection and redacted authentication/maintenance Debug. No security-relevant surface outside the plan's declared RPC/authority boundaries was introduced. Independent full-phase security review remains root-owned.

## Self-Check: PASSED

- All six created Rust files, all eight modified source paths and this summary exist.
- Behavioral RED/GREEN and restored mutation evidence, final focused HTTP/phase159 passes, full library and maintenance regressions, strict scoped lint, formatting and diff checks are recorded above.
- No commit hash, completed requirement, full native pass, actual paired-prune daemon proof or independent security sign-off is claimed. Those remain the dependent/root consolidated gates.
