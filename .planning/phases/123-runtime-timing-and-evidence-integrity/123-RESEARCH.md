---
phase: 123-runtime-timing-and-evidence-integrity
generated_by: gsd-phase-researcher
lifecycle_mode: yolo
phase_lifecycle_id: 123-2026-07-15T18-12-00
generated_at: 2026-07-16T00:07:22Z
status: complete
requirements:
  - HARD-02
  - HARD-03
  - HARD-04
---

# Phase 123: Runtime Timing and Evidence Integrity - Research

**Researched:** 2026-07-15
**Domain:** Blocking sync-session maintenance, achieved-effect evidence, and authoritative runtime observability
**Confidence:** HIGH

## Summary

Phase 123 is a narrow correction of three already-built runtime seams. It does not need a new timer runtime, metrics subsystem, or shared network architecture.

1. The blocking sync transport currently maps EOF, `WouldBlock`, and `TimedOut` to the same `Option::None`; `sync_connected_peer` treats all of them as terminal and returns. This makes the Phase 120 compact timeout forwarder receive-driven. Introduce a typed receive outcome (`Message`, `Idle`, `Closed`), retain the session on `Idle`, obtain a fresh timestamp from a caller-injected clock, expire compact downloads, preserve each returned peer target, and send fallback only through its owning session.
2. `BlockServedCount` currently reads `eligible_peer_count` in both `block_relay_metric_samples` and `block_relay_log_record`. Add one aggregate successful-block-write counter to managed-network evidence and increment it only after `SyncPeerSession::send` or inbound `WriteWireMessageOutcome::Written` completes for an actual `WireNetworkMessage::Block`.
3. Phase 121 installed a provider that samples `ManagedRpcContext`, but `DurableSyncRuntime` owns a different `ManagedPeerNetwork`. Remove that provider and daemon closure. After peer processing, take one availability-gated snapshot directly from `self.network`; pass that exact tick-local value to both metric persistence and structured-log emission.

The most important compatibility consequence is that the Phase 121 checker and `docs/architecture/operator-observability.md` currently require and describe the provider being removed. Phase 123 must migrate those artifacts in the same plan or `bash scripts/verify.sh` will fail while enforcing obsolete architecture.

**Primary recommendation:** implement this as three focused runtime/evidence changes plus one deterministic closeout plan: (1) typed idle maintenance, (2) post-write served-block acknowledgement at both transports, (3) direct same-snapshot runtime projection, and (4) checker/parity/docs reconciliation. Keep the synchronous driver, existing timeout forwarder, existing metric/log helpers, and default local verifier.

## Locked Decisions and Requirement Mapping

| Requirement | Locked behavior | Planning consequence |
| --- | --- | --- |
| `HARD-02` | Timeout expiration advances while a live peer is idle. | `SyncPeerSession::receive` can no longer return an ambiguous `Option`; live daemon wiring must supply a fresh clock on every idle wake. |
| `HARD-03` | Served evidence derives from successful `WireNetworkMessage::Block` emission. | Eligibility/status decisions remain pre-effect evidence; a separate aggregate counter advances after each successful write only. |
| `HARD-04` | Metrics/logs sample the network owned by `DurableSyncRuntime`. | Delete `maybe_block_relay_metric_status_provider`, its setter, and the `ManagedRpcContext` closure; reuse one direct snapshot for metrics and logs. |

All context decisions D-01 through D-12 are binding. In particular:

- idle is neither receive activity nor EOF;
- caller clocks remain explicit and testable;
- partial batches acknowledge every successful block before a later failure returns;
- failed encoding/writes and non-block messages never increment served count;
- unobserved/unavailable evidence omits the metric family and log record;
- no public-network, service-manager, wall-clock soak, or production-readiness gate enters default verification.

## Project and Standards Constraints

Material guidance loaded for this research:

- `AGENTS.md` and `AGENTS.bright-builds.md`: use the pinned Knots submodule, Bun for substantial automation, parity breadcrumbs for touched/new Rust paths, command-timing wrappers for ad-hoc Cargo/Bazel, and `bash scripts/verify.sh` as the final contract.
- `standards/core/architecture.md`: clocks and writes stay in imperative shells; evidence classification and message identity remain typed data.
- `standards/core/code-shape.md`: prefer enums and early returns; split focused helpers instead of enlarging the already-large sync loop.
- `standards/core/testing.md`: one concern per test with explicit Arrange/Act/Assert.
- `standards/core/verification.md`: prefer repo-native verification and do not weaken a failing older checker.
- `standards/languages/rust.md`: use typed enums, `let...else`, `maybe_` names for `Option`, no `unwrap()` in production, and `foo.rs` plus `foo/` for any new multi-file module.
- `standards/languages/typescript-javascript.md`: keep the checker as pure Bun/TypeScript functions with deterministic fixture mutations.
- `standards-overrides.md`: no active local exception applies.

## Standard Stack

No new dependency is needed.

| Concern | Use | Do not add |
| --- | --- | --- |
| Idle pulse | Existing `TcpStream` read timeout plus a typed receive outcome | Tokio timer, async transport rewrite, background timer thread |
| Clock | Caller-injected `FnMut() -> i64` or an equivalently required clock collaborator | `SystemTime::now()` inside node-core/session logic |
| Timeout action | `ManagedPeerNetwork::expire_compact_download_timeouts(now_unix_seconds)` | Duplicate compact timeout state machine |
| Served evidence | Existing `ManagedBlockRelayEvidenceState` and shared status contract | Separate metrics-only atomic/proxy counter |
| Successful send boundary | `SyncPeerSession::send` success and `WriteWireMessageOutcome::Written` | Enqueue/construction/eligibility inference |
| Metrics/logs | Existing `block_relay_metric_samples` and `block_relay_log_record` | New metric kinds, dynamic labels, parallel store/writer |
| Checker | Bun 1.3.9, fixed corpus, pure checks, mutation fixtures | Shell grep chain, network test, nondeterministic process inspection |
| Final verification | `bash scripts/verify.sh` | Public network or wall-clock soak requirement |

Environment probes found Rust/Cargo 1.94.1, Bun 1.3.9, Bazelisk 1.28.1/Bazel 8.6.0, and the Knots submodule pinned at `a9aee730...` (`v29.3.knots20260210`).

## Current Architecture and Exact Gaps

### HARD-02: receive-idle is terminal today

`packages/open-bitcoin-node/src/sync/types.rs` defines:

```rust
pub trait SyncPeerSession {
    fn receive(
        &mut self,
        magic: NetworkMagic,
    ) -> Result<Option<WireNetworkMessage>, SyncRuntimeError>;
}
```

`packages/open-bitcoin-node/src/sync/tcp.rs::read_exact_or_stall` returns `Ok(None)` for all of:

- `UnexpectedEof` (closed/truncated stream),
- `WouldBlock` (idle wake),
- `TimedOut` (idle wake).

`packages/open-bitcoin-node/src/sync.rs::sync_connected_peer` returns from the loop for `None`, marks an incomplete handshake stalled, and disconnects the peer during cleanup. The live session therefore cannot run compact timeout maintenance while idle.

The existing reusable timeout boundary is already correct:

- `ManagedPeerNetwork::expire_compact_download_timeouts(now_unix_seconds)` returns `Vec<(PeerId, WireNetworkMessage)>`;
- `network/tests/compact_timeout_cases.rs` proves `GetData(Block)`, timeout evidence, targeted peer preservation, and volatile-only cleanup;
- Knots runs `ProcessMessages` and `SendMessages` from `ThreadMessageHandler` on an independent roughly 100 ms wake and performs download timeout checks in `SendMessages`, so maintenance is not contingent on inbound message receipt.

### HARD-03: served count is an eligibility proxy

`ManagedBlockRelayEvidenceState::record_block_serving` records eligibility and block status when a serve decision is made. That is useful decision evidence but not proof of a write.

Both projections are currently wrong:

- `packages/open-bitcoin-node/src/metrics/block_relay.rs::block_relay_metric_samples` maps `BlockServedCount` from `block_serving.eligibility.eligible_peer_count`;
- `packages/open-bitcoin-node/src/logging.rs::block_relay_log_record` prints the same proxy.

Two successful-write seams exist:

- `packages/open-bitcoin-node/src/sync/session.rs::send_all`: retains the typed `WireNetworkMessage` and returns after `session.send` succeeds;
- `packages/open-bitcoin-rpc/src/inbound_listener.rs`: receives encoded `Vec<u8>` responses and sees `WriteWireMessageOutcome::Written`, but message identity has already been erased by `ManagedRpcContext::receive_inbound_wire_message -> Vec<Vec<u8>>`.

The inbound path therefore needs a small typed carrier that pairs a `WireNetworkMessage` (or a block/non-block acknowledgement token) with its encoded bytes until the successful write branch. Do not decode bytes after the write and do not guess from command bytes.

### HARD-04: runtime projection samples another network

`DurableSyncRuntime` owns `network: ManagedPeerNetwork<MemoryChainstateStore>` but also stores `maybe_block_relay_metric_status_provider`. `start_daemon_sync_worker` wires the provider to `ManagedRpcContext::block_relay_evidence_status()`, whose context owns a separate network.

`persist_metrics` calls the provider once and `write_block_relay_log` calls it again. Even if the provider were authoritative, those are two independently sampled values and can disagree within one tick.

The correct seam is within `sync_once_with_resolver`, immediately after peer processing and summary refresh, before metrics/log persistence:

1. obtain `self.network.block_relay_evidence_status()` once;
2. convert its existing outer activation/observation state to `Option`/`FieldAvailability` once;
3. borrow the same local snapshot into `persist_metrics` and `write_block_relay_log`;
4. omit both when unavailable.

## Architecture Patterns

### Pattern 1: typed receive outcome, not sentinel absence

Add an enum near `SyncPeerSession`:

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SyncPeerReceiveOutcome {
    Message(WireNetworkMessage),
    Idle,
    Closed,
}
```

The exact name is discretionary. The important invariant is that no value can mean both “keep polling” and “tear down the session.” Update all session implementations and scripted sessions to return this enum.

For the TCP implementation, classify only a timeout/would-block before a new frame has consumed bytes as `Idle`; classify a zero-byte read before a frame as `Closed`; treat partial-header/payload EOF or timeout as an error unless buffering is retained explicitly. `std::io::Read::read_exact` does not report how many bytes it consumed before error, so blindly remapping its `TimedOut` result to `Idle` can discard a partial frame and desynchronize the stream. A small read loop that tracks `filled` is safer than extending the existing `read_exact_or_stall` sentinel.

### Pattern 2: required caller clock on the live path

Keep wall-clock acquisition in `open-bitcoind`, where `current_timestamp_unix_seconds()` already exists. Prefer a required explicit clock argument/collaborator on the daemon-used sync method over an optional setter that can silently be absent.

A low-churn shape is:

- keep existing timestamp-taking wrappers for deterministic callers;
- add an internal/public `*_with_clock` path taking `&mut impl FnMut() -> i64`;
- make the daemon call the clocked path with `current_timestamp_unix_seconds`;
- make old wrappers delegate with a fixed clock only where idle advancement is not expected;
- make focused tests pass a scripted clock sequence.

On every `Idle` outcome:

1. call the clock exactly once;
2. call `expire_compact_download_timeouts(now)` exactly once;
3. retain `(target_peer_id, message)` until target validation;
4. send only messages whose target is the session's `peer_id`;
5. continue polling without `progress.record_activity` and without incrementing processed-message limits.

The current runtime holds one live peer session at a time, so other-peer timeout actions should not normally exist. Still, do not erase the target and misattribute a fallback. Treat a non-owning target as an invariant violation/error or preserve it for a future dispatcher; never send it to the current socket.

### Pattern 3: one post-write acknowledgement API

Add a small managed-network method with typed message identity, for example:

```rust
pub fn acknowledge_wire_message_written(&mut self, message: &WireNetworkMessage) {
    if !matches!(message, WireNetworkMessage::Block(_)) {
        return;
    }
    self.block_relay_evidence.record_block_written();
}
```

The method is idempotent only by call discipline: invoke it exactly once per successful write. Do not call it from block lookup, `serve_managed_block_request`, action translation, encoding, queue admission, or send intent.

Change `DurableSyncRuntime::send_all` from `&self` to `&mut self`. For each message, call `session.send(...)` first, then acknowledge. The loop naturally provides partial-batch semantics: prior successful blocks remain counted if a later write fails.

For inbound serving, carry typed identity through encoding, then call the matching `ManagedRpcContext` acknowledgement only inside `Ok(WriteWireMessageOutcome::Written)`. Encoding failure never enters the write loop; rejected/failed writes never acknowledge.

### Pattern 4: add served count to the status contract, not eligibility

The least surprising shared location is `BlockServingStatusCounters`, adding `served_count: u64`, because serving is an achieved outcome rather than peer eligibility. `ManagedBlockRelayEvidenceState` remains the sole aggregate owner and projects the value through `BlockRelayEvidenceStatus`.

Consequences the plan must include:

- update all nine current `BlockServingEligibilityCounters`/status fixture families as required by the actual chosen field location;
- update metric and log helper expectations from 2 (eligible) to an independently chosen served value;
- confirm CLI/dashboard/support serialization tolerates the added aggregate field and remains redacted;
- preserve existing eligibility/status counters unchanged.

Do not overload `validated_count` or `available_count`; those are decision/classification counts and can exceed successful writes.

### Pattern 5: one tick-local authoritative snapshot

Replace the provider with a local snapshot passed explicitly:

```rust
let maybe_block_relay_status = self.maybe_authoritative_block_relay_status();
self.persist_metrics(&summary, timestamp, maybe_block_relay_status.as_ref())?;
self.write_summary_logs(&mut summary, timestamp);
self.write_block_relay_log(
    &mut summary,
    timestamp,
    maybe_block_relay_status.as_ref(),
);
```

The helper should call `self.network.block_relay_evidence_status()` directly and use the established activation/observation availability gate. Do not clone or re-sample between the metric and log calls. Keep inbound metrics on their existing provider; D-08 removes only the block-relay provider.

## Exact File and Symbol Map

| File | Symbols / current role | Required planning work |
| --- | --- | --- |
| `packages/open-bitcoin-node/src/sync/types.rs` | `SyncPeerSession::receive` | Add typed `Message`/`Idle`/`Closed` outcome and update trait signature. |
| `packages/open-bitcoin-node/src/sync/tcp.rs` | `TcpPeerSession::receive`, `read_message_header`, `read_exact_or_stall` | Distinguish idle, close, and partial-frame errors without losing frame bytes silently. |
| `packages/open-bitcoin-node/src/sync.rs` | `DurableSyncRuntime`, `sync_once_with_resolver`, `sync_peer_with_retries`, `sync_connected_peer` | Remove block-relay provider field; thread caller clock; run idle maintenance; take one direct post-peer snapshot. |
| `packages/open-bitcoin-node/src/sync/session.rs` | `send_all` | Make mutable and acknowledge each successful typed block send after `send`. |
| `packages/open-bitcoin-node/src/sync/metrics.rs` | setter, `persist_metrics` | Delete block-relay setter/provider lookup; accept the tick-local optional snapshot. |
| `packages/open-bitcoin-node/src/sync/runtime_state.rs` | `write_block_relay_log` | Accept the same tick-local optional snapshot instead of calling provider. |
| `packages/open-bitcoin-node/src/sync/tests.rs` and optional focused child | scripted transports/sessions, provider tests | Convert receive fixtures; add idle/fake-clock, successful/failed/partial-send, omission, and authoritative projection tests; replace obsolete provider tests. |
| `packages/open-bitcoin-node/src/network/block_relay_evidence.rs` | `ManagedBlockRelayEvidenceState`, projection methods | Store/record successful block writes and expose typed acknowledgement through `ManagedPeerNetwork`. |
| `packages/open-bitcoin-node/src/status/block_serving.rs` | `BlockServingStatusCounters` | Add the dedicated aggregate served field if this recommended contract placement is used. |
| `packages/open-bitcoin-node/src/metrics/block_relay.rs` | `block_relay_metric_samples` | Map `BlockServedCount` only from the new served field. |
| `packages/open-bitcoin-node/src/logging.rs` | `block_relay_log_record` | Render `block_served_count` from the same new field. |
| `packages/open-bitcoin-node/src/metrics/tests.rs`, `logging/tests.rs`, status/CLI fixtures | aggregate status literals | Set independent eligible and served values so proxy regression is observable. |
| `packages/open-bitcoin-rpc/src/context/network.rs` | `receive_inbound_wire_message` | Preserve typed message identity alongside encoded bytes and expose acknowledgement delegation. |
| `packages/open-bitcoin-rpc/src/inbound_listener.rs` | response write loop | Acknowledge only in `WriteWireMessageOutcome::Written`; retain identity across queue/write checks. |
| `packages/open-bitcoin-rpc/src/inbound_listener/tests.rs` | listener effect tests | Prove written block increments, rejected/failed write does not, and non-block write does not. |
| `packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs` | `start_daemon_sync_worker`, `daemon_sync_worker` | Delete block-relay context closure/imports; inject live clock into daemon sync path. |
| `scripts/check-phase121-block-relay-metrics-log-runtime.ts` and `.test.ts` | obsolete provider requirements | Migrate Phase 121's checker to require helper reuse/persist/log integration without requiring wrong-network provider wiring. |
| `scripts/check-phase123-runtime-timing-evidence-integrity.ts` and `.test.ts` | new deterministic gate | Enforce all three hardening invariants, parity evidence, mutation coverage, and verifier wiring. |
| `scripts/verify.sh` | visible command list and `run_step` list | Add Phase 123 test/check after Phase 122 and before Phase 117 in both regions. |
| `docs/architecture/operator-observability.md` | Phase 121 provider prose | Correct provenance to the direct authoritative sync network and same-snapshot flow. |
| `docs/parity/index.json`, `docs/parity/checklist.md`, `docs/parity/catalog/p2p.md` | v2.1 evidence roots | Add a Phase 123 surface owning exactly `HARD-02`, `HARD-03`, `HARD-04` and bounded Knots anchors/claims. |
| `docs/parity/source-breadcrumbs.json` | source file ownership | Ensure every touched/new Rust file is covered; add new focused test files if created. |

## Recommended Plan Decomposition

### Plan 123-01: deterministic idle maintenance (`HARD-02`)

- Introduce the typed receive outcome and safe TCP classification.
- Thread a caller clock from daemon to `sync_connected_peer` without a new async dependency.
- On idle, expire and emit same-session targeted fallback, then continue.
- Add deterministic scripted idle/fake-clock tests and TCP classification tests.

### Plan 123-02: successful block-emission evidence (`HARD-03`)

- Add `served_count` to authoritative aggregate evidence/status.
- Add one typed post-write acknowledgement method.
- Wire `send_all` and inbound `Written` seams, including typed encoded response carrier.
- Add success/failure/non-block/partial-batch tests and update projection fixtures.

### Plan 123-03: authoritative same-snapshot projection (`HARD-04`)

- Remove the block-relay provider field/setter and daemon `ManagedRpcContext` closure.
- Sample `self.network` once after runtime peer processing.
- Pass the same availability-gated snapshot to metric and log paths.
- Replace provider tests with unobserved omission and sync-runtime compact-activity projection tests.
- Update Phase 121 checker/tests and operator-observability prose so old guardrails describe the new architecture.

### Plan 123-04: deterministic evidence and parity closeout (`HARD-02`–`HARD-04`)

- Add Phase 123 Bun checker and mutation suite.
- Add/update parity surface, checklist/catalog evidence, and breadcrumb manifest.
- Wire both checker commands into both `verify.sh` regions.
- Run focused tests, checker compatibility tests, simplification pass, and full repository verification.

Plans 123-01 and 123-02 can be developed independently, but 123-03 must see the final evidence contract from 123-02. Plan 123-04 should follow all runtime changes because its checker fixture encodes the final structure.

## Don't Hand-Roll

- Do not create another compact timeout scheduler or duplicate `COMPACT_BLOCK_DOWNLOAD_TIMEOUT_SECONDS`; call the existing managed-network forwarder.
- Do not add Tokio, async traits, channels, or a timer thread for one blocking-session wake.
- Do not infer message type by parsing encoded response bytes after the write; retain typed identity.
- Do not use an atomic or metrics-global served counter detached from `ManagedBlockRelayEvidenceState`.
- Do not create separate metric/log snapshots or stores; reuse one status and the existing projection helpers.
- Do not unify the RPC and sync `ManagedPeerNetwork` instances in this phase; context explicitly defers broader shared mutable ownership.
- Do not rewrite the Phase 121 checker away. Migrate its obsolete provider assertions while preserving its helper/persist/log/leakage guarantees.
- Do not add public-network timing tests or real sleeps. Fake the receive outcomes and clock.

## Common Pitfalls

### Pitfall 1: treating a timeout after a partial frame as harmless idle

`read_exact` may consume bytes before returning an error. Returning `Idle` then starting a new header loses framing. Track bytes explicitly; only a zero-progress timeout is a clean idle wake, or retain partial buffering deliberately.

### Pitfall 2: counting idle wakes as progress

Calling `progress.record_activity`, incrementing message count, or consuming the `max_messages_per_peer` budget on idle creates false progress and can defeat stall/no-progress logic. Only `Message` does those things.

### Pitfall 3: erasing timeout target identity

`expire_compact_download_timeouts` returns `(PeerId, message)` for a reason. Sending an other-peer fallback over the current session is worse than dropping it. Validate ownership before write.

### Pitfall 4: acknowledging before the effect

Recording after block construction or before `session.send` repeats the Phase 118 category error at a later boundary. `HARD-03` requires completed write, not intent, encoding, queueing, or eligibility.

### Pitfall 5: losing partial-batch evidence

Do not acknowledge at the end of `send_all`. If messages 1 and 2 write and message 3 fails, successful block writes among 1 and 2 must already be counted exactly once.

### Pitfall 6: changing only the metric helper

The log helper currently uses the same eligibility proxy. Both must read one new served field, and tests should choose different eligible/served values.

### Pitfall 7: direct sampling twice

Replacing the provider with two calls to `self.network.block_relay_evidence_status()` fixes provenance but not same-tick consistency. Snapshot once and pass references.

### Pitfall 8: manufacturing zero evidence

`BlockRelayEvidenceStatus::default_unavailable` contains several available zero subfields. Gate at the established observation/activation boundary before invoking helpers, or an unobserved runtime will persist misleading zero-valued samples.

### Pitfall 9: stale Phase 121 verifier contradiction

`check-phase121-block-relay-metrics-log-runtime.ts` currently requires `set_block_relay_metric_status_provider` and daemon provider wiring. Removing production code without updating this checker guarantees full verification failure.

### Pitfall 10: broad contract fallout from a new struct field

Adding `served_count` to a serialized counter struct affects Rust literals in node, CLI status/dashboard/support, and tests. Use compiler errors to enumerate every fixture, then verify support redaction and stable serialization.

## Code Examples

Illustrative shapes only; exact names remain planner discretion.

### Idle maintenance branch

```rust
let mut messages_received = 0_usize;
while messages_received < self.config.max_messages_per_peer {
    match session.receive(self.config.network.magic())? {
        SyncPeerReceiveOutcome::Message(message) => {
            messages_received = messages_received.saturating_add(1);
            progress.record_activity(message_timestamp);
            // Existing receive/process/send path.
        }
        SyncPeerReceiveOutcome::Idle => {
            let now_unix_seconds = clock();
            let targeted = self
                .network
                .expire_compact_download_timeouts(now_unix_seconds)?;
            self.send_targeted_for_session(&mut session, peer_id, &targeted)?;
        }
        SyncPeerReceiveOutcome::Closed => break,
    }
}
```

### Per-message achieved-effect acknowledgement

```rust
for message in messages {
    session.send(message, self.config.network.magic())?;
    self.network.acknowledge_wire_message_written(message);
}
```

### Same tick-local projection

```rust
let maybe_block_relay = self.maybe_authoritative_block_relay_status();
self.persist_metrics(&summary, timestamp, maybe_block_relay.as_ref())?;
self.write_summary_logs(&mut summary, timestamp);
self.write_block_relay_log(&mut summary, timestamp, maybe_block_relay.as_ref());
```

## Testing Strategy

### Rust behavior tests

Each test should prove one concern with Arrange/Act/Assert.

`HARD-02` focused cases:

1. idle before timeout: no fallback, session retained, no receive-progress credit;
2. idle after fake clock crosses timeout: same peer receives `GetData(Block)` without another inbound message;
3. idle wake does not count as received/processed activity;
4. EOF returns `Closed` and tears down normally;
5. partial-frame timeout/EOF is not reclassified as clean idle;
6. target mismatch is never written to the current session;
7. timeout cleanup and chainstate invariants from Phase 120 remain green.

`HARD-03` focused cases:

1. successful sync-session block write increments once;
2. failed sync-session block write increments zero;
3. non-block success increments zero;
4. `[Block success, non-block success, Block failure]` records one;
5. `[Block success, Block success, later failure]` records two;
6. inbound `Written` block increments once;
7. inbound rejected/write-error block increments zero;
8. independent eligible and served fixture values prove metric and log map served only.

`HARD-04` focused cases:

1. unobserved authoritative network omits all block-relay metric samples and log record;
2. compact activity performed on `DurableSyncRuntime::network` appears in the persisted same-tick metric/log projection;
3. metric and log receive the same snapshot (use a value whose change would be observable, not two all-zero fixtures);
4. there is no block-relay provider setter or daemon closure;
5. inbound metric provider behavior is unchanged.

Where the existing `sync/tests.rs` size makes new scenarios hard to read, add one focused child file such as `sync/tests/runtime_integrity_cases.rs`, register it from `sync/tests.rs`, and add its parity breadcrumb. Avoid a broad test-module reorganization during this phase.

### Deterministic Bun checker

Create `scripts/check-phase123-runtime-timing-evidence-integrity.ts` as a pure exported function over a fixed target-file corpus. Use ordered assertions where order carries the invariant.

Required checker groups:

- typed receive outcome contains distinct message/idle/closed states;
- idle branch obtains caller time, calls timeout expiration, validates peer target, sends fallback, and continues;
- `send_all` orders `session.send` before acknowledgement;
- inbound acknowledgement occurs only under `WriteWireMessageOutcome::Written` and typed identity survives encoding;
- both metric and log helpers read the new served field and do not read `eligible_peer_count` for `block_served_count`;
- runtime snapshots `self.network.block_relay_evidence_status()` once and passes the same local snapshot to metrics and logs;
- obsolete block-relay provider setter/field/daemon closure are absent;
- Phase 121 checker has been migrated and still checks helper reuse, persistence, structured logging, leakage, and verifier inclusion;
- required focused Rust test names are present;
- parity index/checklist/catalog own exactly `HARD-02`, `HARD-03`, `HARD-04` with Knots anchors and no-claim text;
- `scripts/verify.sh` contains test and checker commands in both visible and `run_step` regions after Phase 122.

Mutation tests should each break one invariant: collapse `Idle` into `Closed`, remove fresh clock use, drop target validation, move acknowledgement before send, remove inbound acknowledgement, restore eligibility mapping, restore provider wiring, re-sample for logs, remove an omission test, mutate one HARD ID, and remove one verifier command. The real repository corpus run remains the final checker proof; fixtures only prove sensitivity.

## Validation Architecture

Nyquist validation is disabled for this milestone, so Phase 123 does not need a separate validation plan or public-network harness. The phase still needs layered deterministic evidence:

| Layer | What it proves | Primary gate |
| --- | --- | --- |
| Pure/typed unit | receive state cannot conflate idle/close; projection selects served count | focused Rust unit tests |
| Adapter effect | acknowledgement occurs after actual write; partial batches count correctly | scripted session + inbound listener tests |
| Runtime integration | idle tick emits fallback; runtime-owned compact evidence reaches persistence/log | `open-bitcoin-node` sync runtime tests |
| Static corpus | production wiring, ordering, old-checker migration, parity roots, verifier inclusion | Phase 123 Bun checker + mutations |
| Repository contract | formatting, clippy, all targets/features, all tests, breadcrumbs, Bazel smoke, existing phases | `bash scripts/verify.sh` |

No layer may substitute for the layer below it: the Bun checker cannot prove write behavior, and a Rust unit test cannot prove `verify.sh` actually invokes the checker.

## Parity and Documentation Impact

### Canonical Knots anchors

- `packages/bitcoin-knots/src/net.cpp::CConnman::ThreadMessageHandler`: message processing and `SendMessages` run on an independent wake, providing the receive-independent maintenance precedent.
- `packages/bitcoin-knots/src/net_processing.cpp::PeerManagerImpl::SendMessages`: obtains current time and checks in-flight block download timeout outside receipt of a new message.
- `packages/bitcoin-knots/src/net_processing.cpp` block serving paths: `MakeAndPushMessage(..., BLOCK, ...)` provides the typed wire-emission provenance anchor.
- `packages/bitcoin-knots/test/functional/p2p_compactblocks.py`: compact-block fallback/request behavior remains the functional reference.

Open Bitcoin deliberately keeps its existing compact-download timeout constant/policy and blocking sync architecture; the parity claim is about receive-independent advancement and truthful effects, not identical thread cadence.

### Source breadcrumbs

All likely touched existing Rust files already have manifest groups:

- `node-sync-runtime`, `node-sync-types`, `node-sync-tcp`, `node-sync-tests`;
- `node-network-block-relay-evidence-adapter`, `node-network-adapter`;
- `node-observability-contracts`, `node-status-contract`;
- `rpc-inbound-listener` and `rpc-surface`/context groups.

If a new test module is added, add it to the matching group with `net.cpp`, `net_processing.cpp`, and `p2p_compactblocks.py` where defensible. Keep `none` only for genuinely Open Bitcoin-only support contracts such as metric formatting; do not use it for the runtime timing/write paths that have direct Knots anchors.

### Evidence surface

Add a single `v2-1-runtime-timing-evidence-integrity` surface in `docs/parity/index.json` and `docs/parity/checklist.md` owning `HARD-02`, `HARD-03`, and `HARD-04`. Update `docs/parity/catalog/p2p.md` and `docs/architecture/operator-observability.md`. Do not claim:

- public block serving or compact relay by default;
- archive behavior;
- package relay, bloom serving, or compact filters;
- public-network CI;
- production full-node readiness or production-funds use;
- unified RPC/sync network provenance beyond the specific authoritative sync projection.

## Verification Sequence

Ad-hoc Cargo/Bazel commands must run sequentially through the repo timing wrapper; do not overlap the shared target directory.

Recommended implementation loop:

1. `bun run scripts/command-timings.ts run --key phase123-node-runtime-tests -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node runtime_integrity`
2. `bun run scripts/command-timings.ts run --key phase123-node-metrics-tests -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node block_relay`
3. `bun run scripts/command-timings.ts run --key phase123-rpc-inbound-tests -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc inbound_listener`
4. `bun test scripts/check-phase121-block-relay-metrics-log-runtime.test.ts`
5. `bun run scripts/check-phase121-block-relay-metrics-log-runtime.ts`
6. `bun test scripts/check-phase123-runtime-timing-evidence-integrity.test.ts`
7. `bun run scripts/check-phase123-runtime-timing-evidence-integrity.ts`
8. `bun run scripts/check-parity-breadcrumbs.ts --check`
9. `bash scripts/verify.sh`

The final command is the required pre-commit/release contract and already self-wraps with command timing outside CI. Poll resumable verification at least every 60 seconds; do not terminate a quiet Cargo/Bazel step merely because an estimate elapsed. Review `git diff --check`, the full diff, and generated `docs/metrics/lines-of-code.md` freshness after verification.

## Simplification Pass

Before closing implementation, explicitly check:

- Can the receive enum replace the old `Option` without parallel sentinel helpers?
- Can one clocked internal method preserve old public wrappers instead of adding runtime clock state?
- Can one acknowledgement method serve both outbound and inbound paths?
- Can one `served_count` field replace all proxy logic without adding another status object?
- Can one local `maybe_block_relay_status` feed both outputs without cloning?
- Can the Phase 121 checker be migrated narrowly instead of duplicated into Phase 123?

The expected answer is a small typed seam at each boundary, not a generalized receipt framework or transport redesign.

## Security and Operability Notes

- The new counter remains aggregate and low-cardinality; it must not contain peer IDs, endpoints, block hashes, payloads, permission strings, credentials, or dynamic labels.
- Retaining typed identity until acknowledgement does not mean persisting or logging the message. Drop the typed carrier after write processing.
- Queue pressure and write rejection remain authoritative; rejected writes must not be reclassified as served.
- Fake-clock tests avoid nondeterministic sleeps and timing flakes.
- Availability omission prevents operators from reading “0 served” as proof that the runtime was observed when it was not.

## Sources

### Primary, HIGH confidence

- `.planning/phases/123-runtime-timing-and-evidence-integrity/123-CONTEXT.md` — locked D-01 through D-12.
- `.planning/REQUIREMENTS.md` and `.planning/ROADMAP.md` — `HARD-02` through `HARD-04` ownership and success criteria.
- `.planning/v2.1-MILESTONE-AUDIT.md` — the three concrete debt findings.
- `packages/open-bitcoin-node/src/sync.rs`, `sync/types.rs`, `sync/tcp.rs`, `sync/session.rs`, `sync/metrics.rs`, `sync/runtime_state.rs` — live session, send, and projection seams.
- `packages/open-bitcoin-node/src/network.rs`, `network/block_relay_evidence.rs`, `network/tests/compact_timeout_cases.rs` — authoritative network and timeout/evidence behavior.
- `packages/open-bitcoin-node/src/metrics/block_relay.rs`, `logging.rs`, and tests — current proxy and fixed projections.
- `packages/open-bitcoin-rpc/src/context/network.rs`, `inbound_listener.rs`, `inbound_listener/resource_runtime.rs`, `bin/open-bitcoind.rs` — encoded response, successful write, wrong provider, and daemon clock seams.
- `scripts/check-phase121-block-relay-metrics-log-runtime.ts`, matching tests, `scripts/check-phase122-compact-relay-peer-completion.ts`, and `scripts/verify.sh` — deterministic checker patterns and compatibility constraint.
- `docs/parity/index.json`, `docs/parity/source-breadcrumbs.json`, `docs/architecture/operator-observability.md` — evidence roots and stale provider prose.
- `packages/bitcoin-knots/src/net.cpp`, `src/net_processing.cpp`, and `test/functional/p2p_compactblocks.py` — pinned maintenance, timeout, send, and compact behavior anchors.

### Secondary, HIGH confidence

- Phase 118 context — achieved compact-announcement evidence precedent.
- Phase 120 context — explicit timestamp and peer-targeted timeout forwarder precedent.
- Phase 121 context/research — availability omission and helper reuse; its provider provenance conclusion is superseded by the Phase 123 audit/context.
- Phase 122 context — current compact peer serving/provenance boundary.
- Bright Builds architecture, code-shape, testing, verification, Rust, and TypeScript standards loaded through the repo-local managed corpus.

## Confidence and Open Questions

**Confidence: HIGH.** All three gaps are observable in current code, the replacement seams already exist, no external library decision is required, and the pinned Knots references confirm the lifecycle direction.

No user decision is blocking planning. The planner retains discretion over helper names, exact clock API shape, and whether focused tests live in existing files or new child modules. The plan should not reopen the locked choices of blocking runtime, post-write acknowledgement, direct sync-network projection, or default deterministic verification.

## RESEARCH COMPLETE
