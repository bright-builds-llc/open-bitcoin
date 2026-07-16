---
phase: 123-runtime-timing-and-evidence-integrity
status: passed
verified_at: "2026-07-16T06:56:27Z"
score: "31/31 plan truths verified"
generated_by: gsd-verifier
lifecycle_mode: yolo
phase_lifecycle_id: 123-2026-07-15T18-12-00
generated_at: "2026-07-16T06:56:27Z"
lifecycle_validated: true
requirements_verified:
  - HARD-02
  - HARD-03
  - HARD-04
review_fixes_verified:
  - CR-01
human_verification_required: false
overrides_applied: 0
---

# Phase 123: Runtime Timing and Evidence Integrity Verification Report

**Phase Goal:** Make compact-relay timing and operator evidence reflect authoritative live runtime events rather than receive activity or proxy counts.

**Status:** passed

**Score:** 31/31 plan truths verified

This verification was materially informed by the repo-local `AGENTS.md`, `AGENTS.bright-builds.md`, `standards-overrides.md`, `standards/index.md`, and the architecture, code-shape, operability, verification, testing, Rust, and TypeScript/JavaScript standards. It verifies actual production paths and achieved effects, not plan or summary completion.

## Requirement Accounting

Every requirement ID declared by every Phase 123 plan exists in `.planning/REQUIREMENTS.md`, is assigned to Phase 123 by both REQUIREMENTS traceability and ROADMAP, and is accounted for below. No plan references an orphaned, misspelled, or out-of-phase requirement.

| Plan | Declared requirement IDs | Accounting |
| --- | --- | --- |
| `123-01` | `HARD-02` | Mapped to Phase 123; verified by typed receive, caller-clock, idle-maintenance, cadence, cancellation, and response-lifecycle paths. |
| `123-02` | `HARD-03` | Mapped to Phase 123; verified by the private achieved-write counter and non-serialized snapshot. |
| `123-03` | `HARD-03` | Mapped to Phase 123; verified at the durable sync send-success boundary. |
| `123-04` | `HARD-03` | Mapped to Phase 123; verified at the inbound listener write-success boundary. |
| `123-05` | `HARD-03`, `HARD-04` | Both mapped to Phase 123; verified by explicit served-count projection and direct sync-owned sampling. |
| `123-06` | `HARD-04` | Mapped to Phase 123; verified by the migrated Phase 121 checker and operator provenance. |
| `123-07` | `HARD-02`, `HARD-03`, `HARD-04` | Exact three-ID parity ownership, mutation coverage, breadcrumbs, and default verifier wiring verified. |

| Requirement | Normative obligation | Result |
| --- | --- | --- |
| `HARD-02` | Compact-download expiration advances on a deterministic runtime schedule without another received message. | SATISFIED |
| `HARD-03` | Served-block evidence derives from successful `WireNetworkMessage::Block` emission, not eligible-peer proxies. | SATISFIED |
| `HARD-04` | Runtime block-relay metrics and logs sample the authoritative network owned by `DurableSyncRuntime`. | SATISFIED |

The unchecked requirement boxes and `Pending` traceability rows in `.planning/REQUIREMENTS.md`, plus stale Phase 122/123 plan prompts in `.planning/ROADMAP.md`, are milestone-rollup reconciliation assigned explicitly to Phase 124 (`HARD-05`). They do not contradict the current Phase 123 implementation, parity surface, checker, summaries, review-fix report, or successful verifier evidence. No source, ROADMAP, REQUIREMENTS, or STATE metadata was changed by this verification.

## Goal Achievement

### HARD-02: Receive-Independent Timing and Usable Fallback

| Verified behavior | Actual implementation and evidence |
| --- | --- |
| Idle, close, and message are distinct | `SyncPeerReceiveOutcome` exposes `Message`, `Idle`, and `Closed`; the node crate re-exports it and every first-party `SyncPeerSession` implementation, including `open-bitcoin-bench`, uses it. |
| TCP framing distinguishes clean idle/close from truncation | `sync/tcp.rs::read_stage` returns clean `Idle` or `Closed` only before any header byte. Partial header timeout/EOF and every incomplete payload are errors. Four deterministic read-stage tests cover the boundary. |
| Every message and idle wake uses a current injected timestamp | `sync/session.rs::sync_connected_peer_with_cancel` calls the injected clock in both `Message` and `Idle` branches. Message timestamps flow through activity recording, message dispatch, block handling, and reconciliation; slow-message and message-after-idle regressions prove no stale initial timestamp is reused. |
| Idle maintenance consumes no message budget or progress credit | Only the `Message` branch increments `messages_received` and calls `progress.record_activity`; the `Idle` branch expires timeouts and continues without either mutation. The focused idle-budget test passes. |
| Default cadence survives through expiration | `SyncRuntimeConfig::default().read_timeout_ms` is `5_000`; compact timeout is `60` seconds. The runtime test drives thirteen five-second idle wakes and proves no fallback before expiration and full-block fallback after 65 seconds. |
| Idle sessions remain bounded | Sessions with no compact or ordinary block-response work yield on their first idle wake. Sessions with compact work remain while bounded compact state exists. No timer thread, async redesign, duplicate state machine, or dependency was introduced. |
| Cancellation stops a silent live peer session | The daemon passes `should_cancel` into `sync_until_idle_with_clock_and_cancel`; the runtime checks before receive and immediately after idle receive. `phase123_daemon_shutdown_cancels_live_silent_peer_session` proves cooperative shutdown at the bounded read boundary. |
| Timeout fallback remains on the owning session | Timeout actions retain `PeerId`; the complete target batch is validated before any write, and a mismatch returns a fixed sanitized error. Same-peer fallback is sent through the retained session. |
| Timeout fallback enters ordinary tracked download state | The idle branch extracts the expired full-block hash and calls `block_reconcile::request_tracked_blocks`, which records the hash in the peer's `requested_blocks` and the runtime's `inflight_blocks` before sending `GetData(Block)`. |
| Matching fallback response is consumed before yield | Session retention uses `peer_has_pending_download_work`, including ordinary requested blocks. A matching `Block` is classified as requested before release, dispatched through validation/connect, persisted, credited, and cleared from both peer/runtime tracking. `NotFound` clears both tracking layers without false block credit. |

`HARD-02` is satisfied end to end: timeout scheduling is receive-independent, and the resulting fallback is usable rather than merely emitted.

### HARD-03: Successful-Write-Only Served Evidence

| Verified behavior | Actual implementation and evidence |
| --- | --- |
| One private evidence owner | `ManagedBlockRelayEvidenceState::served_count` is private. `BlockRelayRuntimeEvidenceSnapshot` is crate-private and does not derive serialization. Public `BlockRelayEvidenceStatus` and `BlockServingStatusCounters` remain unchanged. |
| Typed acknowledgement only recognizes blocks | `record_wire_message_written` returns early for every non-`WireNetworkMessage::Block` and advances the private counter only for typed blocks. |
| Sync writes acknowledge after success | `DurableSyncRuntime::send_all` calls `session.send(...)?` before `acknowledge_wire_message_written`. Failed sends count zero; each successful block prefix is retained exactly once if a later batch item fails. |
| Inbound writes retain type through the effect boundary | `EncodedWireResponse` carries the original typed message beside encoded bytes. Complete-batch encoding succeeds before carrier creation; the listener never decodes response bytes to infer identity. |
| Inbound acknowledgement is `Written`-only | `acknowledge_inbound_response_write` returns unless the result is exactly `Ok(WriteWireMessageOutcome::Written)`. Rejection, write error, failed encoding, and successful non-block writes count zero; successful block prefixes survive later failure. |
| Production activation is real | `open-bitcoind` constructs `DurableSyncRuntime` with `runtime.block_serving`, and `ManagedRpcContext::from_runtime_config_with_store` passes the same resolved block-relay activation into inbound serving. Enabled runtime tests serve and acknowledge blocks; disabled runtime tests emit no block and count zero. Defaults remain off. |
| Metrics/logs do not use eligibility proxies | `block_relay_metric_samples` and `block_relay_log_record` take explicit `served_count`; neither implementation reads `eligible_peer_count`. Tests deliberately use eligibility `2` and served count `9`. |

The Phase 123 diff adds no Cargo manifest or lockfile change and no public block-relay status/RPC/CLI/dashboard/support field. Existing transaction-relay fields named `served_count` are unrelated pre-existing schema and are not used for block-serving evidence.

### HARD-04: Authoritative Runtime Projection

| Verified behavior | Actual implementation and evidence |
| --- | --- |
| Source is the sync runtime's own network | `maybe_authoritative_block_relay_snapshot` calls `self.network.block_relay_runtime_evidence_snapshot()` directly. The former block-relay provider field/setter and daemon `ManagedRpcContext` closure are absent. |
| Sampling happens after peer processing | The sync cycle processes peers, refreshes summary progress, then obtains one `maybe_block_relay_snapshot`. |
| Metrics and logs share the same snapshot | The same local snapshot reference is passed to `persist_metrics` and `write_block_relay_log`; neither effect resamples. Status and private served count therefore have one tick-local provenance. |
| Unavailable evidence is omitted | Activation availability gates the snapshot once. `None` produces neither the nine block-relay metric kinds nor a `block_relay` structured-log record; normal sync-height and inbound metrics remain intact. |
| Sync-owned compact activity projects coherently | The runtime projection regression performs real compact activity and nine typed block acknowledgements on `runtime.network`, then observes the same compact and served values in retained metrics and the structured log. |
| Phase 121 was migrated, not bypassed | `check-phase121-block-relay-metrics-log-runtime.ts` now requires direct sync-owned sampling, same-snapshot reuse, provider absence, omission, fixed helpers, persistence, logs, leakage guards, verifier inclusion, and no twin worker. Operator documentation explicitly describes `ManagedRpcContext` as a separate network. |

## Plan Must-Have Evidence

All 31 frontmatter `must_haves.truths` across Plans 01–07 are verified. Their 23 declared artifacts exist and provide the described production/test/checker behavior, and all 14 declared key links are wired. The highest-risk links were traced beyond static presence:

1. idle wake → fresh clock → compact expiration → target validation → ordinary tracked fallback → successful send → retained same-peer receive;
2. matching block → requested classification → in-flight release → validation/connect → persistence → accepted-block credit → cleanup;
3. typed block → successful transport write → one private acknowledgement → explicit served-count projection;
4. sync-owned network → one availability-gated snapshot → retained metrics and structured logs.

## Iteration-3 Review-Fix Disposition

`123-REVIEW.md` iteration-3 `CR-01` is resolved in actual code, not only in `123-REVIEW-FIX.md`.

The expiration branch no longer sends an untracked `GetData` and immediately disconnects. It converts the compact timeout into an ordinary tracked block request, keeps the same session while response work is pending, accepts a matching best-chain/active-tip-extending block as requested, persists it, gives one block of progress credit, and clears in-flight state before the following idle wake yields.

The focused verifier reran `phase123_compact_timeout_fallback_consumes_matching_block_before_yield`: 1 passed, 0 failed. The Phase 123 mutation suite also independently rejects removal of fallback tracking, session retention, active-tip classification, response consumption, persistence assertion, cadence coverage, and cancellation wiring.

## Parity, Checker, and Evidence Integrity

- `docs/parity/index.json` has exactly one done `v2-1-runtime-timing-evidence-integrity` surface owning exactly `HARD-02`, `HARD-03`, and `HARD-04`, with pinned `net.cpp`, `net_processing.cpp`, and `p2p_compactblocks.py` anchors and explicit bounded deviations/no-claims.
- The checklist and P2P catalog match the machine-readable claim. The runtime-only served count is described as non-serialized, public defaults remain off, and archive/package/filter/public-network CI/production/funds claims remain deferred.
- `runtime_timing_cases.rs`, `runtime_write_evidence_cases.rs`, and `runtime_projection_cases.rs` have non-`none` breadcrumb entries. Existing `block_reconcile.rs` and `block_response.rs` remain in the node-sync-runtime breadcrumb group.
- `scripts/check-phase123-runtime-timing-evidence-integrity.ts` includes the iteration-3 production files and exact fallback-consumption regression. Its 34-test suite includes a real-repository corpus test and independent mutations for timing, clocks, target checks, tracking, response retention, persistence, cancellation, activation, write ordering, encoding, schema, projection, parity, and verifier wiring.
- `scripts/verify.sh` runs the Phase 123 mutation and live checker commands in both its visible and executable regions, after Phase 122 and before Phase 117.
- The unrelated `28403e54` open-stdin stabilization is test-only: it warms the CLI binary before the existing open-stdin watchdog timing assertion. It changes no production runtime, protocol, evidence, schema, or dependency and does not substitute for any Phase 123 proof.

No stale or false Phase 123 runtime/parity/checker evidence was found. The remaining stale milestone rollups are explicitly Phase 124 metadata work, not evidence used to pass this verification.

## Automated Evidence

| Check | Result |
| --- | --- |
| Most recent full `bash scripts/verify.sh` supplied to verifier | PASS: 4m 21.943s; formatting, warnings-denied lint, all-target build/tests, coverage, and Bazel verification |
| Most recent focused checker mutation suites supplied to verifier | PASS: 62 total across Phase 107/121/123 at review-fix time |
| Most recent focused node suite supplied to verifier | PASS: 27 passed |
| Most recent focused RPC suite supplied to verifier | PASS: 10 passed across library and daemon targets |
| `cargo test ... phase123_compact_timeout_fallback_consumes_matching_block_before_yield` through timing wrapper | PASS: 1 passed, 0 failed |
| `bun test scripts/check-phase123-runtime-timing-evidence-integrity.test.ts` | PASS: 34 passed, 0 failed |
| Phase 121 live checker | PASS |
| Phase 123 live checker | PASS |
| `bun run scripts/check-parity-breadcrumbs.ts --check` | PASS: 383 Rust files |
| LOC worktree freshness check | PASS |
| `git diff --check` | PASS |

The full verifier was not duplicated because the supplied post-review-fix run is current and the additional deterministic checks directly exercised the previously critical gap and the current live corpus.

## Scope and Simplification Review

The final implementation retains one receive enum, one injected clock path, one timeout state machine, one managed-network acknowledgement API, one private served counter/snapshot, one typed inbound carrier, and one authoritative tick snapshot. There is no compatibility provider, serialized block-served field, second snapshot, metrics-global atomic, response decoder, timer worker, async redesign, public-network test gate, or new dependency.

## Human Verification

None required. The phase goal is deterministic and locally observable through runtime tests, mutation tests, static live-corpus checks, parity/breadcrumb validation, persistence inspection, and the repository verifier.

## Residual Risks

- Cancellation is cooperative at receive boundaries; one blocking TCP read can delay shutdown by at most the configured read timeout, which defaults to five seconds. This is intentional and bounded.
- Runtime served evidence is aggregate and process-local by design. It is omitted when unobserved and is not a public status field.
- Phase 124 still needs to reconcile REQUIREMENTS/ROADMAP/STATE rollups and archive readiness under `HARD-05`; that metadata work is outside Phase 123's goal and this verification's edit scope.

## Gaps Summary

No Phase 123 goal gaps found. All 31 plan truths, all three assigned requirements, production activation, idle timing/cancellation, usable tracked fallback, successful-write-only evidence, authoritative projection, migrated Phase 121 guardrails, parity/breadcrumb wiring, and iteration-3 review fix are verified.

## Verification Complete

**Status:** passed — **Score:** 31/31 plan truths verified — **Gaps:** none.
