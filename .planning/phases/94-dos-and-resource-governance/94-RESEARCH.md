# Phase 94: DoS and Resource Governance - Research

**Researched:** 2026-06-26 [VERIFIED: current_date/environment_context]  
**Domain:** Rust Bitcoin P2P inbound message-envelope, resource-governance, peer-pressure, and operator-evidence policy [VERIFIED: .planning/ROADMAP.md; .planning/REQUIREMENTS.md]  
**Confidence:** HIGH [VERIFIED: local codebase audit; pinned Knots submodule; repo instructions]

<user_constraints>
## User Constraints (from CONTEXT.md)

Source: `.planning/phases/94-dos-and-resource-governance/94-CONTEXT.md` [VERIFIED: .planning/phases/94-dos-and-resource-governance/94-CONTEXT.md]

### Locked Decisions

## Implementation Decisions

### Message Envelope And Payload Allocation

- **D-01:** Add a typed resource-governance gate before allocation-heavy inbound message handling. The gate must reject wrong network magic, malformed headers, unsupported commands, oversized payloads, checksum failures, malformed payloads, and trailing data through stable labels before creating unbounded buffers or peer-side work.
- **D-02:** Preserve and centralize existing hard caps such as `MAX_SIZE`, `MAX_HEADERS_RESULTS`, `MAX_INV_SIZE`, `DEFAULT_MAX_BLOCKS_IN_FLIGHT_PER_PEER`, and `PHASE92_ADDR_BATCH_LIMIT`. If Phase 94 adds new caps, they should be named constants in a focused policy module with tests proving the boundary.
- **D-03:** Unsupported command handling should remain bounded evidence, not a feature expansion. Repeated unsupported commands may feed Phase 93 misbehavior decisions, but Phase 94 must not start handling mempool, compact-block, filter, or relay commands.
- **D-04:** Parser errors should map to low-cardinality resource or violation labels such as `wrong_network_magic`, `malformed_header`, `payload_oversized`, `invalid_checksum`, `unsupported_command`, `malformed_payload`, and `trailing_payload`. These labels should be usable by tests, metrics, logs, status, support bundles, and docs.

### Request, Queue, And Backpressure Bounds

- **D-05:** Model request and queue governance as pure data-in/data-out policy before runtime effects. Suggested inputs include peer role, handshake state, permission effects, current per-peer queued reads/writes, aggregate queued reads/writes, requested inventory counts, block/header/transaction request counts, and resource-pressure observations.
- **D-06:** Enforce explicit per-peer and aggregate read/write queue limits with stable outcomes. The runtime may apply socket backpressure or disconnects, but it should consume a policy output rather than recalculating queue pressure in the accept loop.
- **D-07:** Bound inventory and request surfaces without enabling transaction relay. `inv`, `getdata`, `headers`, `getheaders`, block, and transaction request tracking should have caps for inbound peers, but transaction relay, mempool propagation, compact blocks, BIP37, and compact-filter serving remain inactive or deferred.
- **D-08:** Permissioned and protected peers may receive scoped policy treatment, but they still count toward resource evidence. `download`, `addr`, `noban`, and `forceinbound` effects can influence bounded policy decisions; relay-like inactive effects must not grant extra queues, request capacity, or serving behavior.

### Timeouts, Churn, Idle Peers, And Reconnects

- **D-09:** Slow handshakes, idle peers, churn, repeated failures, and reconnect suppression should be represented by typed policy decisions evaluated from injected timestamps and counters. Runtime clocks belong in shell adapters; pure policy accepts `now` as data.
- **D-10:** Phase 94 should define deterministic labels for timeout and churn outcomes, such as `slow_handshake`, `idle_peer`, `connection_churn_limited`, `repeated_failure_limited`, `reconnect_suppressed_banned`, and `reconnect_suppressed_discouraged`.
- **D-11:** Banned and discouraged reconnect attempts should use the Phase 93 ban/discourage model as an input and produce explicit evidence. Do not hide broad bans in the listener runtime, and do not silently drop protected-peer violations.
- **D-12:** Tests must avoid wall-clock sleeps. Use injected timestamps, synthetic peer records, loopback-safe fixtures, and deterministic counters to prove timeout, idle, churn, and reconnect behavior.

### Operator Evidence, Metrics, Logs, And Support

- **D-13:** Resource-governance evidence belongs in the shared inbound status/support contract first, then CLI status, RPC/Open Bitcoin status, metrics, logs, and support renderers project the same fields. Avoid renderer-local resource summaries.
- **D-14:** Evidence must stay low-cardinality, bounded, and redacted. Status/support output may include aggregate counters, latest stable event, reason, source, and next action, but not raw peer ids, raw endpoint tables, raw message payloads, raw permission config strings, credentials, or unbounded queue contents.
- **D-15:** Resource-pressure evidence should include useful next actions. Suggested labels include `resource_pressure_active`, `read_queue_pressure`, `write_queue_pressure`, `request_cap_reached`, `payload_rejected`, `timeout_disconnect`, `churn_rejected`, and `reconnect_suppressed`.
- **D-16:** Metrics should remain fixed `MetricKind` variants or equivalent aggregate counters. Do not introduce dynamic labels for peer id, endpoint, command payload, permission class name, ban scope, or raw address.

### Verification, UAT, And Boundaries

- **D-17:** Default verification remains `bash scripts/verify.sh`, deterministic, local, public-network-free, service-manager-free, and short-running. Use pure policy tests, synthetic wire fixtures, and loopback-safe checks instead of public inbound exposure.
- **D-18:** Add unit tests for the pure policy and parser boundaries using Arrange, Act, Assert. Cover wrong magic, malformed header, oversized payload, unsupported command, malformed payload, queue pressure, request caps, backpressure, slow handshake, idle peer, churn, reconnect suppression, protected-peer evidence, and no relay side effects.
- **D-19:** Add deterministic checker coverage if docs/parity evidence is updated. The checker should follow Phase 90-93 patterns and reject positive claims for transaction relay, compact blocks, mempool propagation, public inbound defaults, public-network readiness, production service, or production full-node readiness.
- **D-20:** Any operator UAT text must include repo-local Cargo and Bazel command forms, not an installed alias alone.

### the agent's Discretion

The planner may choose exact cap values, type names, and module splits. Prefer focused pure policy modules over expanding already-large files such as `message.rs`, `peer.rs`, `inbound.rs`, or `metrics.rs`; use small integration points in those files only when that preserves a clear public API. Keep runtime socket/backpressure behavior thin and driven by policy outputs.

### Deferred Ideas (OUT OF SCOPE)

## Deferred Ideas

- Phase 95 owns v1.9 release-boundary docs, no-claim evidence, final parity traceability, and cross-phase non-regression closure.
- Future milestones own transaction relay, compact block relay, mempool propagation, BIP37/compact-filter serving, full address relay, public inbound defaults, public-network CI, production service packaging, and production full-node readiness.
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| DOS-01 | Inbound sessions enforce network magic, message header, payload size, malformed message, and unsupported command limits before allocating unbounded memory. [VERIFIED: .planning/REQUIREMENTS.md] | Implement a typed pre-allocation envelope gate around `parse_message_header`, expected `NetworkMagic`, supported command names, `MAX_SIZE`, checksum, payload decoder errors, and trailing-data detection. [VERIFIED: packages/open-bitcoin-codec/src/network.rs; packages/open-bitcoin-network/src/message.rs; packages/open-bitcoin-rpc/src/inbound_listener.rs] |
| DOS-02 | Inbound sessions enforce per-peer and aggregate read/write queues, inventory/request bounds, header/block/transaction request caps, and backpressure behavior. [VERIFIED: .planning/REQUIREMENTS.md] | Add a pure request and queue pressure policy that consumes peer role, handshake state, permission effects, queue counts, inventory counts, and in-flight counts; preserve existing caps and add new named constants only in a focused policy module. [VERIFIED: packages/open-bitcoin-network/src/peer.rs; packages/open-bitcoin-network/src/peer/inventory_state.rs; packages/open-bitcoin-network/src/inbound.rs] |
| DOS-03 | The node limits connection churn, slow handshakes, idle peers, repeated failures, and banned or discouraged reconnect attempts with deterministic synthetic tests. [VERIFIED: .planning/REQUIREMENTS.md] | Use injected timestamps and counters in pure policy; feed Phase 93 `PeerBanBook` and discourage inputs into reconnect suppression decisions instead of duplicating ban logic. [VERIFIED: packages/open-bitcoin-network/src/peer_policy.rs; .planning/phases/94-dos-and-resource-governance/94-CONTEXT.md] |
| DOS-04 | Resource pressure and abuse responses appear in metrics, structured logs, support bundles, and operator status with clear next actions. [VERIFIED: .planning/REQUIREMENTS.md] | Extend `InboundPeerServingStatus` first, then project fixed `MetricKind` counters, low-cardinality structured events, CLI status lines, and support markdown from the shared status contract. [VERIFIED: packages/open-bitcoin-node/src/status/inbound.rs; packages/open-bitcoin-node/src/metrics.rs; packages/open-bitcoin-cli/src/operator/status/render/inbound.rs; packages/open-bitcoin-cli/src/operator/support/render/inbound.rs] |
| DOS-05 | Default verification covers inbound DoS/resource policy deterministically and keeps public-network listener exposure outside `bash scripts/verify.sh`. [VERIFIED: .planning/REQUIREMENTS.md] | Add pure Rust tests, loopback-safe runtime tests only where adapter behavior is required, and a Phase 94 Bun checker after docs/parity updates; wire the checker into both `VERIFY_COMMAND_ORDER` and `run_step` order after Phase 93. [VERIFIED: scripts/verify.sh; scripts/check-phase93-peer-policy.ts; packages/open-bitcoin-rpc/src/inbound_listener.rs] |
</phase_requirements>

## Summary

Phase 94 should be planned as a pure policy expansion with thin runtime adapters: add one or more focused `open-bitcoin-network` resource-governance modules, then consume their decisions from `open-bitcoin-rpc` inbound socket handling and `open-bitcoin-node` status/metrics projections. [VERIFIED: standards/core/architecture.md; .planning/ARCHITECTURE.md; .planning/phases/94-dos-and-resource-governance/94-CONTEXT.md]

The highest-risk existing gap is that `open-bitcoin-rpc/src/inbound_listener.rs` reads a 24-byte header, extracts the payload length, allocates a payload buffer, and only later calls `ParsedNetworkMessage::decode_wire`; this means magic, supported command, checksum, and malformed payload classification currently occur after runtime allocation. [VERIFIED: packages/open-bitcoin-rpc/src/inbound_listener.rs; packages/open-bitcoin-network/src/message.rs]

The planner should avoid broad rewrites of `message.rs`, `peer.rs`, `inbound.rs`, and `metrics.rs` because those files are already near the repo file-length trigger; Phase 94 behavior should live in focused pure modules with small integration edits. [VERIFIED: bash scripts/check-file-lengths.sh; wc audit of packages/open-bitcoin-network/src/message.rs, peer.rs, inbound.rs, and packages/open-bitcoin-node/src/metrics.rs]

**Primary recommendation:** Create a typed inbound resource-governance policy surface in `open-bitcoin-network`, use it before payload allocation in `open-bitcoin-rpc`, and project bounded labels/counters through existing shared inbound status before renderer or metrics changes. [VERIFIED: .planning/phases/94-dos-and-resource-governance/94-CONTEXT.md; packages/open-bitcoin-node/src/status/inbound.rs]

## Project Constraints (from AGENTS.md)

- Use `bash scripts/verify.sh` as the repo-native verification contract before marking Phase 94 complete; it includes Bun checkers, parity breadcrumbs, Rust format/lint/build/test, Bazel smoke build, benchmark smoke, and pure-core coverage in full mode. [VERIFIED: AGENTS.md; scripts/verify.sh]
- Use `rust-toolchain.toml` as the Rust source of truth; this workspace is pinned to Rust `1.94.1`. [VERIFIED: AGENTS.md; rust-toolchain.toml]
- Preserve externally observable Bitcoin Knots `29.3.knots20260210` behavior for in-scope surfaces and record intentional differences in `docs/parity/`. [VERIFIED: AGENTS.md; .planning/PROJECT.md]
- Keep functional-core behavior free of direct filesystem, socket, clock, storage, terminal, RPC, service-manager, or process effects; put those effects in shell adapters. [VERIFIED: AGENTS.bright-builds.md; standards/core/architecture.md; .planning/ARCHITECTURE.md]
- Do not add third-party Rust Bitcoin libraries to production paths; the project owns its first-party domain model and implementation surface. [VERIFIED: AGENTS.md; .planning/PROJECT.md]
- Add parity breadcrumb blocks and `docs/parity/source-breadcrumbs.json` entries when adding first-party Rust source or test files under `packages/open-bitcoin-*/src` or `packages/open-bitcoin-*/tests`. [VERIFIED: AGENTS.md; scripts/check-parity-breadcrumbs.ts]
- Use Bun as the canonical runtime for repo-owned TypeScript automation; this repo has no `package.json`, so checker work should not add a `bun install` bootstrap. [VERIFIED: AGENTS.md; .planning/STACK.md]
- Use Arrange, Act, Assert comments in non-trivial unit tests; test behavior rather than implementation details. [VERIFIED: standards/core/testing.md; standards/languages/rust.md]
- Use early returns and typed domain errors rather than hidden failures, panics, `unwrap()`, or swallowed command errors. [VERIFIED: AGENTS.md; standards/core/code-shape.md; standards/languages/rust.md]
- Include repo-local Cargo and Bazel command forms in UAT text, not only an installed `open-bitcoin` alias. [VERIFIED: AGENTS.md; .planning/phases/94-dos-and-resource-governance/94-CONTEXT.md]
- Treat `docs/metrics/lines-of-code.md` as a tracked generated artifact that may need freshness updates after verification. [VERIFIED: AGENTS.md; scripts/verify.sh]
- Check whether README files need updates after substantial feature, parity, operator-surface, or workflow changes. [VERIFIED: AGENTS.md]

Project skills: no `.claude/skills/` or `.agents/skills/` project skill directories with `SKILL.md` were found, so no repo-local skill pattern constrains Phase 94 planning. [VERIFIED: local project skill directory audit]

## Standard Stack

### Core

| Library / Module | Version | Purpose | Why Standard |
|------------------|---------|---------|--------------|
| Rust / Cargo | `1.94.1` | Compile, test, lint, and type-check all first-party Rust crates. | Workspace and repo instructions pin this toolchain, and `scripts/verify.sh` uses Cargo as the Rust verification runner. [VERIFIED: rust-toolchain.toml; scripts/verify.sh] |
| `open-bitcoin-network` | first-party workspace crate | Pure peer, inbound, permission, message, request, ban, and resource-governance policy. | Phase 94 decisions require pure data-in/data-out policy before socket effects. [VERIFIED: packages/Cargo.toml; .planning/phases/94-dos-and-resource-governance/94-CONTEXT.md] |
| `open-bitcoin-codec` | first-party workspace crate | Message header parsing, compact-size parsing, `MAX_SIZE`, and codec errors. | Existing wire parsing already centralizes header and compact-size invariants used by the envelope gate. [VERIFIED: packages/open-bitcoin-codec/src/network.rs; packages/open-bitcoin-codec/src/compact_size.rs] |
| `open-bitcoin-rpc` | first-party workspace crate | Tokio listener and runtime adapter for opt-in inbound serving. | Existing inbound listener accepts loopback-safe sockets and is the allocation/backpressure integration point. [VERIFIED: packages/open-bitcoin-rpc/src/inbound_listener.rs] |
| `open-bitcoin-node` | first-party workspace crate | Shared status, metrics, structured logs, and managed peer projections. | Phase 94 evidence must enter `InboundPeerServingStatus` before CLI/support/metrics renderers. [VERIFIED: packages/open-bitcoin-node/src/status/inbound.rs; packages/open-bitcoin-node/src/metrics.rs] |
| Bun | `1.3.9` | Run deterministic TypeScript verifier scripts. | Existing Phase 90-93 checkers and `scripts/verify.sh` use Bun without a package-install step. [VERIFIED: .bun-version; scripts/verify.sh; local `bun --version`] |
| Bazel / Bazelisk command surface | `8.6.0` available locally | Top-level smoke build and UAT command form. | Repo instructions require Bazel command forms for UAT and `scripts/verify.sh` full mode runs Bazel builds. [VERIFIED: AGENTS.md; scripts/verify.sh; local `bazel --version`] |

### Supporting

| Library / Module | Version | Purpose | When to Use |
|------------------|---------|---------|-------------|
| `tokio` | `1.52.1` | Async TCP listener and socket reads/writes in the shell adapter. | Use only in runtime adapters, not pure resource-governance policy. [VERIFIED: Cargo.lock; packages/open-bitcoin-rpc/src/inbound_listener.rs] |
| `serde` / `serde_json` | `1.0.228` / `1.0.149` | Stable status, support, and metric serialization. | Use for shared evidence contracts and tests that assert status/support JSON shape. [VERIFIED: Cargo.lock; packages/open-bitcoin-node/src/status/inbound.rs; packages/open-bitcoin-node/src/metrics.rs] |
| `tracing` | `0.1.44` | Structured runtime log emission where existing logging contracts are extended. | Use for low-cardinality resource-pressure events if Phase 94 adds runtime logs. [VERIFIED: Cargo.lock; packages/open-bitcoin-node/src/logging.rs] |
| `cargo-llvm-cov` | `0.8.5` available locally | Full verification pure-core coverage gate. | Needed only for full `bash scripts/verify.sh`; no new Phase 94 dependency should depend on it. [VERIFIED: scripts/verify.sh; local `cargo llvm-cov --version`] |

### Alternatives Considered

| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| First-party pure policy module | Runtime-only Tokio listener checks | Runtime-only checks are harder to deterministically test and conflict with the functional-core boundary. [VERIFIED: standards/core/architecture.md; .planning/phases/94-dos-and-resource-governance/94-CONTEXT.md] |
| Existing codec/message parser primitives | A new custom wire parser | A second parser risks divergent header, compact-size, and trailing-data behavior; the current codec already owns these invariants. [VERIFIED: packages/open-bitcoin-codec/src/network.rs; packages/open-bitcoin-network/src/message.rs] |
| Fixed `MetricKind` counters | Dynamic metric labels keyed by peer, endpoint, command, or permission name | Dynamic labels violate the Phase 94 low-cardinality metric decision and existing metric tests. [VERIFIED: .planning/phases/94-dos-and-resource-governance/94-CONTEXT.md; packages/open-bitcoin-node/src/metrics.rs] |
| Pure injected timestamps | Wall-clock sleeps in tests | Sleeps are forbidden by Phase 94 and make timeout/churn tests slow and nondeterministic. [VERIFIED: .planning/phases/94-dos-and-resource-governance/94-CONTEXT.md; standards/core/testing.md] |

**Installation:** No new external package should be installed for Phase 94; use existing workspace crates, Rust/Cargo, Bun, and Bazel. [VERIFIED: .planning/STACK.md; cargo metadata; scripts/verify.sh]

```bash
# No npm, bun, or cargo dependency additions are recommended for this phase.
```

**Version verification:** Recommended package/tool versions were verified from local pinned files, `Cargo.lock`, `cargo metadata`, and local command output, not training data. [VERIFIED: rust-toolchain.toml; Cargo.lock; local environment audit]

## Architecture Patterns

### Recommended Project Structure

```text
packages/open-bitcoin-network/src/
├── resource.rs                  # Pure Phase 94 resource labels, caps, envelope gate, queue/request policy. [VERIFIED: standards/core/architecture.md]
├── resource/
│   └── tests.rs                 # Pure Arrange/Act/Assert resource-governance tests. [VERIFIED: standards/core/testing.md]
├── message.rs                   # Small integration exports only; preserve existing parser/cap behavior. [VERIFIED: packages/open-bitcoin-network/src/message.rs]
├── peer.rs                      # Small hook to consume resource/request decisions, not a broad expansion. [VERIFIED: packages/open-bitcoin-network/src/peer.rs]
└── inbound.rs                   # Reuse permission/admission records as policy inputs. [VERIFIED: packages/open-bitcoin-network/src/inbound.rs]

packages/open-bitcoin-rpc/src/
└── inbound_listener.rs          # Thin socket adapter: read header, call policy, allocate only after accept. [VERIFIED: packages/open-bitcoin-rpc/src/inbound_listener.rs]

packages/open-bitcoin-node/src/
├── status/inbound.rs            # Shared resource evidence contract first. [VERIFIED: packages/open-bitcoin-node/src/status/inbound.rs]
├── network/inbound.rs           # Managed projection from policy events into status. [VERIFIED: packages/open-bitcoin-node/src/network/inbound.rs]
└── metrics.rs                   # Fixed aggregate MetricKind counters only. [VERIFIED: packages/open-bitcoin-node/src/metrics.rs]

packages/open-bitcoin-cli/src/operator/
├── status/render/inbound.rs     # Render shared resource evidence. [VERIFIED: packages/open-bitcoin-cli/src/operator/status/render/inbound.rs]
└── support/render/inbound.rs    # Render bounded support next actions. [VERIFIED: packages/open-bitcoin-cli/src/operator/support/render/inbound.rs]

scripts/
├── check-phase94-dos-resource-governance.ts       # Add only if docs/parity/status evidence changes. [VERIFIED: scripts/check-phase93-peer-policy.ts]
└── check-phase94-dos-resource-governance.test.ts  # Bun tests for checker no-claim behavior. [VERIFIED: scripts/check-phase93-peer-policy.test.ts]
```

### Pattern 1: Pre-Allocation Envelope Gate

**What:** Read exactly the 24-byte message header, parse it with the codec, reject wrong network magic, unsupported commands, and oversized payload length before allocating the payload buffer, then validate checksum and payload decoding after a bounded read. [VERIFIED: packages/open-bitcoin-codec/src/network.rs; packages/open-bitcoin-network/src/message.rs; packages/open-bitcoin-rpc/src/inbound_listener.rs]

**When to use:** Use for every inbound runtime socket read before `vec![0_u8; payload_len]` or peer-side work. [VERIFIED: packages/open-bitcoin-rpc/src/inbound_listener.rs; .planning/REQUIREMENTS.md]

**Example:**

```rust
// Pattern derived from packages/open-bitcoin-codec/src/network.rs and packages/open-bitcoin-network/src/message.rs. [VERIFIED]
let header = parse_message_header(header_bytes)?;
let decision = InboundEnvelopePolicy::default().evaluate_header(
    expected_magic,
    header.command.as_str(),
    header.magic,
    header.payload_size as usize,
);
let EnvelopeDecision::ReadPayload { payload_len } = decision else {
    return Err(decision.into_resource_event());
};
```

### Pattern 2: Pure Queue And Request Policy

**What:** Represent queue counts, request counts, in-flight counts, permission effects, and handshake state as a pure input struct, and return a typed decision such as `Accept`, `Backpressure`, `Disconnect`, or `RecordMisbehavior`. [VERIFIED: .planning/phases/94-dos-and-resource-governance/94-CONTEXT.md; packages/open-bitcoin-network/src/inbound.rs; packages/open-bitcoin-network/src/peer.rs]

**When to use:** Use before adding inventory requests, serving block/header/transaction responses, queuing outbound bytes, or accepting new inbound work from a peer. [VERIFIED: .planning/REQUIREMENTS.md; packages/open-bitcoin-network/src/peer/inventory_state.rs]

**Example:**

```rust
// Source pattern: pure input/output policies in packages/open-bitcoin-network/src/inbound.rs and peer_policy.rs. [VERIFIED]
let input = ResourcePressureInput {
    peer_role,
    handshake_state,
    active_permission_effects,
    peer_read_queue_len,
    peer_write_queue_len,
    aggregate_read_queue_len,
    aggregate_write_queue_len,
    requested_inventory_count,
    in_flight_block_count,
};
let decision = ResourceGovernancePolicy::default().decide(input);
```

### Pattern 3: Shared Status Before Renderers

**What:** Add `InboundResourceEvidence`, aggregate counters, latest safe event, and next action to `InboundPeerServingStatus`; CLI, support, metrics, logs, and JSON status should project from that shared model. [VERIFIED: packages/open-bitcoin-node/src/status/inbound.rs; docs/architecture/status-snapshot.md]

**When to use:** Use for DOS-04 before editing CLI renderers or support-bundle text, because renderer-local summaries are explicitly disallowed. [VERIFIED: .planning/phases/94-dos-and-resource-governance/94-CONTEXT.md]

**Example:**

```rust
// Source pattern: existing InboundPeerPolicyEvent and InboundAddressDecisionEvent shapes. [VERIFIED]
pub struct InboundResourceGovernanceEvent {
    pub outcome: String,
    pub reason: String,
    pub label: String,
    pub source: String,
    pub message: String,
    pub next_action: String,
}
```

### Pattern 4: Knots-Anchored But Scoped Parity

**What:** Cite Knots `net.cpp`, `net_processing.cpp`, and `banman.cpp` for envelope rejection, request caps, timeouts, and banned/discouraged handling, but implement only the v1.9 scoped inbound-serving behavior. [VERIFIED: packages/bitcoin-knots/src/net.cpp; packages/bitcoin-knots/src/net_processing.cpp; packages/bitcoin-knots/src/banman.cpp; .planning/REQUIREMENTS.md]

**When to use:** Use in parity docs, source breadcrumbs, and checker evidence whenever Phase 94 claims an externally observable inbound resource boundary. [VERIFIED: AGENTS.md; docs/parity/source-breadcrumbs.json]

### Anti-Patterns to Avoid

- **Allocating from the wire length before policy validation:** This is the primary DOS-01 risk because the current listener allocates after only checking a runtime 32 MiB bound. [VERIFIED: packages/open-bitcoin-rpc/src/inbound_listener.rs]
- **Adding broad command handling to make unsupported-command tests pass:** `mempool`, compact-block, filter, force-relay, and relay behavior remain deferred. [VERIFIED: .planning/phases/94-dos-and-resource-governance/94-CONTEXT.md; .planning/REQUIREMENTS.md]
- **Embedding clock reads or sleeps in policy tests:** Pure timeout/churn decisions must consume injected `now` and counters. [VERIFIED: .planning/phases/94-dos-and-resource-governance/94-CONTEXT.md]
- **Creating dynamic metric labels or raw support evidence:** Phase 94 evidence must use fixed counters and bounded/redacted status/support fields. [VERIFIED: packages/open-bitcoin-node/src/metrics.rs; packages/open-bitcoin-cli/src/operator/support/redaction.rs]
- **Expanding already-large files with most Phase 94 logic:** Add focused modules and leave existing files as integration surfaces. [VERIFIED: standards/core/code-shape.md; file-length audit]

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Bitcoin wire header parsing | A second ad hoc parser in `open-bitcoin-rpc` | `parse_message_header` plus typed resource labels | The codec already parses 24-byte headers and command bytes; duplicating it risks divergent malformed-header behavior. [VERIFIED: packages/open-bitcoin-codec/src/network.rs; packages/open-bitcoin-rpc/src/inbound_listener.rs] |
| Compact-size and payload cap handling | New unchecked integer or byte-count helpers | Existing `MAX_SIZE`, compact-size parser, `MAX_HEADERS_RESULTS`, `MAX_INV_SIZE`, `PHASE92_ADDR_BATCH_LIMIT`, and named Phase 94 constants | Existing parsers reject oversized compact sizes and known message-specific counts before allocation-heavy vectors. [VERIFIED: packages/open-bitcoin-codec/src/compact_size.rs; packages/open-bitcoin-network/src/message.rs; packages/open-bitcoin-network/src/address.rs] |
| Ban/discourage reconnect logic | New listener-local ban tables | Phase 93 `PeerBanBook`, misbehavior policy, and managed peer-policy projection | The Phase 93 model already owns ban, discourage, expiry, manual unban, and protected no-action evidence. [VERIFIED: packages/open-bitcoin-network/src/peer_policy.rs; packages/open-bitcoin-node/src/network/inbound.rs] |
| Runtime timeout tests | Sleeps, real network peers, or public listener exposure | Injected timestamps, synthetic peer records, and loopback-safe fixtures | Phase 94 and repo verification require deterministic local tests. [VERIFIED: .planning/phases/94-dos-and-resource-governance/94-CONTEXT.md; scripts/verify.sh] |
| Metrics cardinality | Dynamic labels keyed by peer id, endpoint, command payload, permission class, ban scope, or raw address | Fixed `MetricKind` aggregate counters and shared inbound status fields | Existing metric tests enforce stable inbound counter names and low cardinality. [VERIFIED: packages/open-bitcoin-node/src/metrics.rs] |
| Docs release-boundary checks | Manual review only | Bun checker following Phase 90-93 patterns | `scripts/verify.sh` already runs Phase 90-93 checkers in order and release-boundary checks inspect docs as text. [VERIFIED: scripts/verify.sh; scripts/check-phase93-peer-policy.ts] |

**Key insight:** Phase 94 is not a new networking feature phase; it is a bounded policy and evidence phase, so custom runtime-only fixes will miss deterministic tests, shared operator evidence, and parity traceability. [VERIFIED: .planning/ROADMAP.md; .planning/phases/94-dos-and-resource-governance/94-CONTEXT.md]

## Common Pitfalls

### Pitfall 1: Allocation Before Full Envelope Classification

**What goes wrong:** A peer-controlled payload size can cause a runtime allocation before wrong magic, unsupported command, checksum, or malformed payload labels are emitted. [VERIFIED: packages/open-bitcoin-rpc/src/inbound_listener.rs; packages/open-bitcoin-network/src/message.rs]

**Why it happens:** The current listener reads the header and allocates by payload length before calling `ParsedNetworkMessage::decode_wire`. [VERIFIED: packages/open-bitcoin-rpc/src/inbound_listener.rs]

**How to avoid:** Move header classification and supported-command/payload-size decisions into a pure gate used before allocation; preserve checksum and payload decoder classification after bounded reads. [VERIFIED: .planning/phases/94-dos-and-resource-governance/94-CONTEXT.md]

**Warning signs:** New tests assert only `decode_wire` errors and do not inspect runtime pre-allocation decisions. [VERIFIED: packages/open-bitcoin-network/src/message/tests.rs; packages/open-bitcoin-rpc/src/inbound_listener.rs]

### Pitfall 2: Accidentally Enabling Relay Surfaces

**What goes wrong:** Adding request governance can drift into serving transaction relay, mempool propagation, compact blocks, filters, or force-relay behavior. [VERIFIED: .planning/REQUIREMENTS.md; .planning/phases/94-dos-and-resource-governance/94-CONTEXT.md]

**Why it happens:** `WireNetworkMessage` includes `Tx`, `Block`, `Inv`, `GetData`, and `NotFound`, so request caps are near relay-shaped message types. [VERIFIED: packages/open-bitcoin-network/src/message.rs]

**How to avoid:** Treat transaction/block/header handling as bounded request accounting and scoped serving only; add negative tests/checker assertions for no relay side effects. [VERIFIED: .planning/phases/94-dos-and-resource-governance/94-CONTEXT.md]

**Warning signs:** Docs or operator text start claiming mempool, transaction relay, compact block relay, public readiness, or production readiness. [VERIFIED: scripts/check-phase93-peer-policy.ts; .planning/REQUIREMENTS.md]

### Pitfall 3: Duplicating Ban And Discourage State

**What goes wrong:** Listener-local reconnect suppression can disagree with durable Phase 93 ban/discourage evidence. [VERIFIED: packages/open-bitcoin-network/src/peer_policy.rs; packages/open-bitcoin-node/src/network/inbound.rs]

**Why it happens:** Reconnect decisions are runtime-adjacent and easy to place directly in accept-loop code. [VERIFIED: packages/open-bitcoin-rpc/src/inbound_listener.rs]

**How to avoid:** Use Phase 93 ban/discourage state as policy input and emit Phase 94 resource labels from the shared status projection. [VERIFIED: .planning/phases/94-dos-and-resource-governance/94-CONTEXT.md; packages/open-bitcoin-network/src/peer_policy.rs]

**Warning signs:** A new map of banned endpoints appears in `open-bitcoin-rpc` or socket code. [VERIFIED: standards/core/architecture.md]

### Pitfall 4: Renderer-Local Evidence

**What goes wrong:** CLI status, support markdown, metrics, and logs can disagree if each computes resource pressure independently. [VERIFIED: docs/architecture/status-snapshot.md; .planning/phases/94-dos-and-resource-governance/94-CONTEXT.md]

**Why it happens:** Existing renderers already contain inbound formatting logic, so adding local summaries there is tempting. [VERIFIED: packages/open-bitcoin-cli/src/operator/status/render/inbound.rs; packages/open-bitcoin-cli/src/operator/support/render/inbound.rs]

**How to avoid:** Extend `InboundPeerServingStatus` and managed projections first, then render those fields. [VERIFIED: packages/open-bitcoin-node/src/status/inbound.rs; packages/open-bitcoin-node/src/network/inbound.rs]

**Warning signs:** A renderer has resource counters that are not present in `InboundPeerServingStatus`. [VERIFIED: docs/architecture/status-snapshot.md]

### Pitfall 5: Non-Hermetic Verification

**What goes wrong:** Tests depend on public peers, public listener exposure, service managers, DNS, long sleeps, or wall-clock timing. [VERIFIED: scripts/verify.sh; .planning/phases/94-dos-and-resource-governance/94-CONTEXT.md]

**Why it happens:** Knots functional tests exercise public-network-shaped P2P behavior, while Open Bitcoin default verification must stay local and short-running. [VERIFIED: packages/bitcoin-knots/test/functional/p2p_invalid_messages.py; packages/bitcoin-knots/test/functional/p2p_timeouts.py; scripts/verify.sh]

**How to avoid:** Port the behavior into synthetic unit tests and loopback-safe adapter tests, not public-network functional tests. [VERIFIED: .planning/phases/94-dos-and-resource-governance/94-CONTEXT.md]

**Warning signs:** A Phase 94 test opens non-loopback sockets, waits for real timeouts, or requires a service manager. [VERIFIED: scripts/verify.sh]

## Code Examples

Verified patterns from official/local sources:

### Stable Label Enum

```rust
// Source: Phase 90-93 as_str() label enums and Phase 94 D-04/D-10. [VERIFIED]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResourceViolationLabel {
    WrongNetworkMagic,
    MalformedHeader,
    PayloadOversized,
    InvalidChecksum,
    UnsupportedCommand,
    MalformedPayload,
    TrailingPayload,
}

impl ResourceViolationLabel {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongNetworkMagic => "wrong_network_magic",
            Self::MalformedHeader => "malformed_header",
            Self::PayloadOversized => "payload_oversized",
            Self::InvalidChecksum => "invalid_checksum",
            Self::UnsupportedCommand => "unsupported_command",
            Self::MalformedPayload => "malformed_payload",
            Self::TrailingPayload => "trailing_payload",
        }
    }
}
```

### Deterministic Timeout Decision

```rust
// Source: Phase 94 D-09/D-12 and Phase 93 injected-time ban policy. [VERIFIED]
let input = PeerTimeoutInput {
    handshake_state: InboundHandshakeState::AwaitingVersion,
    connected_at_unix_seconds: 100,
    last_activity_unix_seconds: 100,
    now_unix_seconds: 160,
    failure_count: 0,
};

let decision = ResourceGovernancePolicy::default().decide_timeout(input);
assert_eq!(decision.label(), "slow_handshake");
```

### Checker Wiring Shape

```bash
# Source: scripts/verify.sh Phase 90-93 checker order. [VERIFIED]
bun test scripts/check-phase94-dos-resource-governance.test.ts
bun run scripts/check-phase94-dos-resource-governance.ts
```

## State of the Art

| Old / Current Approach | Current Phase 94 Approach | When Changed | Impact |
|------------------------|---------------------------|--------------|--------|
| Runtime listener checks only payload length before allocation. [VERIFIED: packages/open-bitcoin-rpc/src/inbound_listener.rs] | Pure pre-allocation gate rejects malformed header, wrong magic, unsupported command, and oversized payload before payload buffer allocation. [VERIFIED: .planning/phases/94-dos-and-resource-governance/94-CONTEXT.md] | Phase 94 [VERIFIED: .planning/ROADMAP.md] | DOS-01 becomes testable without public-network exposure. [VERIFIED: .planning/REQUIREMENTS.md] |
| `ParsedNetworkMessage::decode_wire` validates checksum and payload after a full byte slice exists. [VERIFIED: packages/open-bitcoin-network/src/message.rs] | Keep decoder behavior, but add a runtime gate before constructing the full byte slice. [VERIFIED: packages/open-bitcoin-network/src/message.rs; packages/open-bitcoin-rpc/src/inbound_listener.rs] | Phase 94 [VERIFIED: .planning/ROADMAP.md] | Avoids duplicating message decoding while reducing allocation risk. [VERIFIED: standards/core/architecture.md] |
| Existing peer policy covers eviction, ban, discourage, and misbehavior without broad DoS/resource governance. [VERIFIED: packages/open-bitcoin-network/src/peer_policy.rs; docs/parity/index.json] | Use Phase 93 policy as input and add resource-governance labels/counters for reconnect, churn, and pressure. [VERIFIED: .planning/phases/94-dos-and-resource-governance/94-CONTEXT.md] | Phase 94 [VERIFIED: .planning/ROADMAP.md] | Prevents duplicate ban state and gives operators explicit reconnect-suppression evidence. [VERIFIED: DOS-03; DOS-04] |
| Existing inbound status exposes listener, admission, permission, address, and peer-policy evidence. [VERIFIED: packages/open-bitcoin-node/src/status/inbound.rs] | Extend the same shared contract with resource evidence before renderer changes. [VERIFIED: .planning/phases/94-dos-and-resource-governance/94-CONTEXT.md] | Phase 94 [VERIFIED: .planning/ROADMAP.md] | Keeps CLI, support, metrics, logs, and RPC status consistent. [VERIFIED: docs/architecture/status-snapshot.md] |

**Deprecated/outdated for Phase 94:**

- Runtime-only resource governance is out of scope because decisions must be deterministic pure policy before socket effects. [VERIFIED: standards/core/architecture.md; .planning/phases/94-dos-and-resource-governance/94-CONTEXT.md]
- Public-network or service-manager verification is out of scope for `bash scripts/verify.sh`. [VERIFIED: scripts/verify.sh; .planning/REQUIREMENTS.md]
- Positive v1.9 claims for transaction relay, compact blocks, mempool propagation, public inbound defaults, or production full-node readiness remain out of scope. [VERIFIED: .planning/REQUIREMENTS.md; .planning/PROJECT.md]

## Assumptions Log

| # | Claim | Section | Risk if Wrong |
|---|-------|---------|---------------|
| A1 | Research validity is estimated at 30 days, or sooner if Phase 90-93 inbound APIs change. [ASSUMED: planning freshness estimate] | Metadata | Planner might rely on stale integration points after nearby inbound code changes. |
| A2 | The GSD ASVS table is used as a planning taxonomy for application-security concerns, not as a formal OWASP ASVS compliance claim. [ASSUMED: GSD template interpretation] | Security Domain | A compliance reviewer could require a separate ASVS mapping using the exact current OWASP chapter taxonomy. |

## Open Questions

1. **Exact new cap values for Phase 94 queues/timeouts/churn**
   - What we know: The user explicitly left exact cap values to planner discretion while requiring named constants and boundary tests. [VERIFIED: .planning/phases/94-dos-and-resource-governance/94-CONTEXT.md]
   - What's unclear: The exact numeric defaults for read/write queues, aggregate queues, repeated failures, churn windows, slow handshakes, and idle peers are not locked. [VERIFIED: .planning/phases/94-dos-and-resource-governance/94-CONTEXT.md]
   - Recommendation: Choose conservative named defaults in the new pure policy module and assert boundary behavior; do not change existing caps unless a task explicitly documents the parity reason. [VERIFIED: D-02; D-20]

2. **How much structured log emission is required beyond status/support/metrics**
   - What we know: DOS-04 requires metrics, structured logs, support bundles, and operator status. [VERIFIED: .planning/REQUIREMENTS.md]
   - What's unclear: Existing inbound listener evidence is status-centric, while structured log projection may require a small managed runtime event bridge. [VERIFIED: packages/open-bitcoin-rpc/src/inbound_listener.rs; packages/open-bitcoin-node/src/logging.rs]
   - Recommendation: Plan at least one low-cardinality structured resource event path using the same event labels as status; avoid raw payload or endpoint logs. [VERIFIED: D-14; D-16]

3. **Whether `DEFAULT_MAX_BLOCKS_IN_FLIGHT_PER_PEER = 128` should remain different from Knots' in-transit cap**
   - What we know: Phase 94 D-02 says preserve `DEFAULT_MAX_BLOCKS_IN_FLIGHT_PER_PEER`, and local code defines it as `128`; Knots `net_processing.cpp` defines `MAX_BLOCKS_IN_TRANSIT_PER_PEER = 16`. [VERIFIED: packages/open-bitcoin-network/src/peer.rs; packages/bitcoin-knots/src/net_processing.cpp; .planning/phases/94-dos-and-resource-governance/94-CONTEXT.md]
   - What's unclear: Whether a future parity phase will intentionally lower this default is not part of Phase 94. [VERIFIED: .planning/REQUIREMENTS.md]
   - Recommendation: Preserve the Open Bitcoin constant in Phase 94 and document any new resource cap separately. [VERIFIED: D-02]

## Environment Availability

| Dependency | Required By | Available | Version | Fallback |
|------------|-------------|-----------|---------|----------|
| Rust `rustc` | Rust source build and tests | yes [VERIFIED: local `rustc --version`] | `1.94.1` [VERIFIED: local `rustc --version`; rust-toolchain.toml] | Blocking if missing; install pinned toolchain. [VERIFIED: AGENTS.md] |
| Cargo | Workspace checks | yes [VERIFIED: local `cargo --version`] | `1.94.1` [VERIFIED: local `cargo --version`] | Blocking if missing; install pinned toolchain. [VERIFIED: scripts/verify.sh] |
| Bun | TypeScript checkers | yes [VERIFIED: local `bun --version`] | `1.3.9` [VERIFIED: .bun-version; local `bun --version`] | Blocking for checker work; no `bun install` needed. [VERIFIED: .planning/STACK.md] |
| Bazel | Full verify and UAT command form | yes [VERIFIED: local `bazel --version`] | `8.6.0` [VERIFIED: local `bazel --version`] | Full verify blocking if missing; install Bazelisk/Bazel. [VERIFIED: scripts/verify.sh] |
| `cargo-llvm-cov` | Full verify pure-core coverage | yes [VERIFIED: local `cargo llvm-cov --version`] | `0.8.5` [VERIFIED: local `cargo llvm-cov --version`] | Full verify blocking if missing; install with `cargo install cargo-llvm-cov --locked`. [VERIFIED: scripts/verify.sh] |
| Bitcoin Knots submodule | Parity anchors and source breadcrumbs | yes [VERIFIED: `git submodule status packages/bitcoin-knots`] | `v29.3.knots20260210` commit `a9aee730466ac67d35a3c03ee24676be5e045878` [VERIFIED: git submodule status] | Blocking for parity anchoring; run `git submodule update --init --recursive`. [VERIFIED: AGENTS.md] |

**Missing dependencies with no fallback:** None found. [VERIFIED: environment availability audit]

**Missing dependencies with fallback:** None found. [VERIFIED: environment availability audit]

## Security Domain

OWASP ASVS is an application security verification standard with a current official project page, and OWASP's official GitHub release API exposes a bleeding-edge `latest` release published on 2026-03-17 with ASVS 5.0.0 assets. [CITED: https://owasp.org/www-project-application-security-verification-standard/; CITED: https://api.github.com/repos/OWASP/ASVS/releases/latest]

### Applicable ASVS Categories

| ASVS Category | Applies | Standard Control |
|---------------|---------|------------------|
| V2 Authentication | no | Phase 94 inbound P2P governance does not add user authentication or credential flows. [VERIFIED: .planning/REQUIREMENTS.md] |
| V3 Session Management | partial | Treat peer lifecycle as bounded connection state, not web session state; use injected timestamps and peer records for slow handshake, idle, and reconnect decisions. [VERIFIED: DOS-03; D-09] |
| V4 Access Control | yes | Preserve permissioned/protected peer effects only for scoped admission/download/address/diagnostic behavior; relay-like inactive effects must not grant capacity or serving behavior. [VERIFIED: D-08; packages/open-bitcoin-network/src/inbound/permissions.rs] |
| V5 Input Validation | yes | Validate network magic, message header, payload length, command support, checksum, payload shape, and trailing bytes before unbounded allocation or peer-side work. [VERIFIED: DOS-01; D-01] |
| V6 Cryptography | partial | Use existing checksum/hash primitives only for protocol checksum verification; do not add custom cryptography or treat checksums as authentication. [VERIFIED: packages/open-bitcoin-network/src/message.rs; standards/core/architecture.md] |

### Known Threat Patterns for Rust P2P Inbound

| Pattern | STRIDE | Standard Mitigation |
|---------|--------|---------------------|
| Oversized payload allocation | Denial of Service | Pre-allocation envelope gate and named payload caps. [VERIFIED: DOS-01; D-01] |
| Malformed message parse amplification | Denial of Service / Tampering | Stable parser labels and bounded decode paths using existing codec errors. [VERIFIED: packages/open-bitcoin-network/src/message.rs; D-04] |
| Request or inventory queue exhaustion | Denial of Service | Per-peer and aggregate read/write queue limits plus request/inventory caps. [VERIFIED: DOS-02; D-05; D-06; D-07] |
| Slow handshake or idle peer slot exhaustion | Denial of Service | Injected-time timeout decisions and disconnect labels. [VERIFIED: DOS-03; D-09; D-10] |
| Reconnect churn by banned or discouraged peers | Denial of Service | Phase 93 ban/discourage input plus explicit reconnect-suppression evidence. [VERIFIED: D-11; packages/open-bitcoin-network/src/peer_policy.rs] |
| Metrics/support evidence cardinality blowup | Information Disclosure / Denial of Service | Fixed `MetricKind` counters, bounded status fields, redacted support output, and no raw payloads/endpoints. [VERIFIED: D-14; D-16; packages/open-bitcoin-node/src/metrics.rs] |

## Sources

### Primary (HIGH confidence)

- `.planning/phases/94-dos-and-resource-governance/94-CONTEXT.md` - locked Phase 94 decisions, discretion areas, deferred scope, integration points, and suggested labels. [VERIFIED]
- `.planning/REQUIREMENTS.md` - DOS-01 through DOS-05 and v1.9 future/deferred relay and production boundaries. [VERIFIED]
- `.planning/ROADMAP.md` - Phase 94 goal and success criteria. [VERIFIED]
- `.planning/STATE.md` - current v1.9 state and public-network-free verification posture. [VERIFIED]
- `AGENTS.md`, `AGENTS.bright-builds.md`, and `standards/` files - repo workflow, Bright Builds standards, Rust/testing/verification constraints. [VERIFIED]
- `packages/open-bitcoin-codec/src/network.rs` and `packages/open-bitcoin-codec/src/compact_size.rs` - header parsing and `MAX_SIZE`/compact-size behavior. [VERIFIED]
- `packages/open-bitcoin-network/src/message.rs`, `peer.rs`, `peer/inventory_state.rs`, `inbound.rs`, and `peer_policy.rs` - current message, peer, inbound, inventory, permission, eviction, ban, discourage, and misbehavior policy surfaces. [VERIFIED]
- `packages/open-bitcoin-rpc/src/inbound_listener.rs` - runtime inbound listener allocation and loopback-safe adapter tests. [VERIFIED]
- `packages/open-bitcoin-node/src/status/inbound.rs`, `network/inbound.rs`, `metrics.rs`, and `logging.rs` - shared inbound status, managed projections, metric constraints, and structured log contracts. [VERIFIED]
- `packages/open-bitcoin-cli/src/operator/status/render/inbound.rs` and `packages/open-bitcoin-cli/src/operator/support/render/inbound.rs` - CLI/status/support projection patterns. [VERIFIED]
- `scripts/verify.sh` and Phase 90-93 checker scripts - deterministic checker and verification wiring pattern. [VERIFIED]
- `packages/bitcoin-knots/src/net.cpp`, `net_processing.cpp`, `banman.cpp`, and relevant functional tests - pinned Knots anchors for envelope rejection, request caps, timeouts, and banned/discouraged behavior. [VERIFIED]

### Secondary (MEDIUM confidence)

- OWASP ASVS official project page and GitHub release API - used only to confirm ASVS exists as an application security verification standard and that OWASP's release assets include ASVS 5.0.0. [CITED: https://owasp.org/www-project-application-security-verification-standard/; CITED: https://api.github.com/repos/OWASP/ASVS/releases/latest]

### Tertiary (LOW confidence)

- None. [VERIFIED: source audit]

## Metadata

**Confidence breakdown:**

- Standard stack: HIGH - all tool and dependency versions were verified from local pinned files, `Cargo.lock`, `cargo metadata`, or local command output. [VERIFIED: rust-toolchain.toml; Cargo.lock; environment audit]
- Architecture: HIGH - Phase 94 decisions align directly with repo functional-core/imperative-shell standards and existing pure policy modules. [VERIFIED: standards/core/architecture.md; packages/open-bitcoin-network/src/inbound.rs; packages/open-bitcoin-network/src/peer_policy.rs]
- Pitfalls: HIGH - each pitfall maps to a current code path, locked decision, or verification constraint. [VERIFIED: packages/open-bitcoin-rpc/src/inbound_listener.rs; .planning/phases/94-dos-and-resource-governance/94-CONTEXT.md; scripts/verify.sh]

**Research date:** 2026-06-26 [VERIFIED: current_date/environment_context]  
**Valid until:** 2026-07-26, or sooner if Phase 90-93 inbound APIs change. [ASSUMED: planning freshness estimate]
