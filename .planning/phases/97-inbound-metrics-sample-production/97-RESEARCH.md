# Phase 97: Inbound Metrics Sample Production - Research

**Researched:** 2026-06-28
**Domain:** Rust runtime observability, retained metrics, inbound peer status projection
**Confidence:** HIGH for existing contracts and append path; MEDIUM for final runtime bridge placement until the planner chooses the integration boundary

<user_constraints>
## User Constraints (from CONTEXT.md)

The following locked decisions, discretion areas, and deferred ideas are copied verbatim from `.planning/phases/97-inbound-metrics-sample-production/97-CONTEXT.md`. [VERIFIED: 97-CONTEXT.md]

### Locked Decisions

## Implementation Decisions

### Metric Source
- **D-01:** Treat `OpenBitcoinStatusSnapshot.peers.inbound` and `InboundPeerServingStatus` as the canonical source for inbound metric sample production.
- **D-02:** Do not create duplicate counter state for metrics. Runtime metric samples should be derived from the shared inbound status projection that already receives admission, permission, peer-policy, and resource-governance updates.
- **D-03:** When inbound status is unavailable, emit no inbound metric samples rather than manufacturing zero-valued availability evidence.

### Metric Mapping
- **D-04:** Map each existing fixed inbound `MetricKind` variant to one numeric aggregate from `InboundPeerServingStatus`.
- **D-05:** Preserve bounded low-cardinality metrics only: sample kind, numeric value, and timestamp. Do not attach peer ids, endpoints, raw permission class strings, ban scopes, reasons, labels, addresses, or other runtime-created dimensions.
- **D-06:** Keep the pure mapping testable without storage, network, or runtime side effects.

### Runtime Persistence
- **D-07:** Extend the existing metrics append path instead of creating a new history store. Inbound samples should flow through `FjallNodeStore::append_metric_samples` and the existing retention policy.
- **D-08:** Persist inbound samples alongside the existing sync samples during runtime progress collection so dashboard/status history sees one retained metrics snapshot.
- **D-09:** The implementation should be public-network-free and exercise local synthetic or loopback evidence only.

### Operator Evidence
- **D-10:** The dashboard already has labels for registered inbound metric kinds; this phase should prove retained inbound samples can be rendered by that path instead of adding new dashboard UI.
- **D-11:** Status/support evidence should remain redacted and aggregate. Support-bundle and runtime-guide changes should document the closed flow only when needed to make `INB-05` and `DOS-04` auditable.

### Verification
- **D-12:** Add a deterministic checker for Phase 97 that proves inbound metric samples are produced from real runtime/status evidence, use existing fixed `MetricKind`s, are retained through the existing metrics history path, and remain public-network-free.
- **D-13:** Default verification must include the Phase 97 checker through `bash scripts/verify.sh`.

### Folded Todos
No pending todos matched this phase.

### the agent's Discretion
The agent may choose the exact helper/module placement, test fixture construction, and checker implementation details, provided the implementation keeps the mapping pure, uses existing runtime/status contracts, and preserves the public-network-free verification boundary.

### Deferred Ideas (OUT OF SCOPE)

Traceability reconciliation for stale requirement ownership remains Phase 98. Public-network listener exposure, relay behavior, production node readiness, new dashboard UX, and dynamic metric dimensions remain out of scope.
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| INB-05 | Operator status, metrics, logs, RPC-facing status, and support evidence distinguish inbound serving from outbound sync and expose admission and handshake outcomes. | Existing inbound status already carries aggregate admission and policy evidence; Phase 97 must convert that status projection into retained `MetricSample`s and prove dashboard/status history consumes them. [VERIFIED: .planning/REQUIREMENTS.md, status/inbound.rs, metrics.rs, dashboard/model.rs] |
| DOS-04 | Resource pressure and abuse responses appear in metrics, structured logs, support bundles, and operator status with clear next actions. | Resource-governance counters already exist on `InboundPeerServingStatus`; Phase 97 must map them to fixed inbound resource metric kinds and persist them through the existing retention path. [VERIFIED: .planning/REQUIREMENTS.md, status/inbound.rs, metrics.rs, fjall_store.rs] |
</phase_requirements>

## Summary

Phase 97 should close a broken runtime flow, not invent a new metrics vocabulary. The repo already defines fixed inbound `MetricKind` variants, the dashboard already has labels for those variants, and Fjall already owns a retained metrics append/prune path. [VERIFIED: metrics.rs, dashboard/model.rs, fjall_store.rs] The current gap is that `SyncRunSummary::metric_samples` produces only sync/outbound samples, and `DurableSyncRuntime::persist_metrics` appends only those samples. [VERIFIED: sync/types/summary.rs, sync/runtime_state.rs, v1.9-MILESTONE-AUDIT.md]

The plan should add a pure inbound status-to-samples mapper, keep unavailable inbound status as an empty sample set, append inbound samples through `FjallNodeStore::append_metric_samples`, and add a deterministic Phase 97 checker wired into `bash scripts/verify.sh`. [VERIFIED: 97-CONTEXT.md, metrics.rs, fjall_store.rs, scripts/verify.sh] The planner must explicitly resolve two permission metric mapping gaps before implementation: `InboundInactivePermissionEffectCount` is documented as observation-count based but `InboundPeerServingStatus` currently exposes only inactive-effect labels, and `InboundPermissionValidationFailureCount` has a metric kind but no verified status aggregate or runtime producer. [VERIFIED: operator-observability.md, status/inbound.rs, network/inbound.rs, rg permission_validation]

**Primary recommendation:** Add `inbound_metric_samples(inbound: &FieldAvailability<InboundPeerServingStatus>, timestamp: u64) -> Vec<MetricSample>` near the `metrics.rs`/status boundary, extend the runtime append caller to persist `summary.metric_samples(timestamp)` plus inbound samples in one call, and test the mapper, storage retention, dashboard rendering, and checker deterministically. [VERIFIED: metrics.rs, status.rs, sync/runtime_state.rs, fjall_store.rs, dashboard/model.rs]

## Project Constraints (from AGENTS.md)

- Repo work must follow `AGENTS.md`, `AGENTS.bright-builds.md`, `standards-overrides.md`, and relevant standards under `standards/`. [VERIFIED: AGENTS.md]
- `bash scripts/verify.sh` is the repo-native verification contract for first-party code; `--fast` is only for local iteration. [VERIFIED: AGENTS.md, scripts/verify.sh]
- Rust is pinned by `rust-toolchain.toml`; the local toolchain reports `rustc 1.94.1` and `cargo 1.94.1`. [VERIFIED: rust-toolchain.toml, rustc --version, cargo --version]
- The workspace uses Rust 2024 edition in `packages/Cargo.toml`. [VERIFIED: packages/Cargo.toml]
- Functional core / imperative shell boundaries apply: pure business logic should stay free of direct filesystem, process, network, terminal, RPC, service-manager, and durable-storage effects. [VERIFIED: AGENTS.md, standards/core/architecture.md]
- Do not use existing Rust Bitcoin libraries in the production path; this phase does not need any new Bitcoin dependency. [VERIFIED: AGENTS.md]
- New Rust source or test files under `packages/open-bitcoin-*/src` or `packages/open-bitcoin-*/tests` require parity breadcrumbs and `docs/parity/source-breadcrumbs.json` coverage; editing existing files avoids new breadcrumb records. [VERIFIED: AGENTS.md, docs/parity/source-breadcrumbs.json]
- Rust code should avoid production `unwrap()`, prefer `let...else` for early returns, use `maybe_` for optional values, and keep comments focused on why. [VERIFIED: AGENTS.md, standards/languages/rust.md]
- Tests should verify behavior, one concern per test, and use Arrange, Act, Assert comments where they improve clarity. [VERIFIED: AGENTS.md, standards/core/testing.md]
- Bun is the canonical runtime for repo-owned TypeScript automation; the installed version is `1.3.9`, and this repo has no `package.json` bootstrap step. [VERIFIED: AGENTS.md, .bun-version, bun --version]
- No project skills exist under `.claude/skills/` or `.agents/skills/`. [VERIFIED: find .claude/skills .agents/skills]

## Standard Stack

### Core

| Component | Version | Purpose | Why Standard |
|-----------|---------|---------|--------------|
| Rust workspace | 1.94.1 / edition 2024 | Implement mapper, runtime append, and tests. | Existing first-party code is Rust and the toolchain is pinned. [VERIFIED: rust-toolchain.toml, packages/Cargo.toml] |
| `open-bitcoin-node::metrics` | local crate | Own `MetricKind`, `MetricSample`, retention, and append/prune helper. | This is the existing metrics contract consumed by storage and dashboard. [VERIFIED: metrics.rs] |
| `InboundPeerServingStatus` | local status type | Canonical aggregate source for inbound admission, permission, peer-policy, and resource-governance counters. | User decisions D-01 and D-02 lock this as the source and forbid duplicate counter state. [VERIFIED: 97-CONTEXT.md, status/inbound.rs] |
| `FjallNodeStore::append_metric_samples` | local storage adapter | Persist and prune retained metric samples. | User decision D-07 locks the existing append path and retention policy. [VERIFIED: 97-CONTEXT.md, fjall_store.rs] |
| `DurableSyncRuntime::persist_metrics` | local runtime shell | Existing runtime progress collection append point. | It currently appends sync samples only, so it is the narrowest existing persistence path to extend. [VERIFIED: sync/runtime_state.rs] |
| Bun TypeScript checker | 1.3.9 | Deterministic Phase 97 structural checker and tests. | Phase 90-96 checkers already use TypeScript/Bun and `scripts/verify.sh` already executes them. [VERIFIED: .bun-version, scripts/check-phase96-peer-policy-runtime-bridge.ts, scripts/verify.sh] |

### Supporting

| Component | Version | Purpose | When to Use |
|-----------|---------|---------|-------------|
| `serde` / `serde_json` | `serde` 1.0.228, `serde_json` 1.0.149 | Stable status, metrics, and snapshot data shapes. | Use existing derives and JSON tests; no new serializer is needed. [VERIFIED: Cargo.lock, metrics.rs, status/inbound.rs] |
| `fjall` | 3.1.4 | Durable storage backing for metrics snapshots. | Use only through `FjallNodeStore`; do not add a second metrics store. [VERIFIED: Cargo.lock, fjall_store.rs] |
| `ratatui` / `crossterm` | `ratatui` 0.30.0, `crossterm` 0.29.0 | Existing dashboard rendering stack. | No new UI library is needed; dashboard model already maps metrics to charts. [VERIFIED: Cargo.lock, dashboard/model.rs] |

### Alternatives Considered

| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| Existing `MetricSample` retention | New inbound metrics table/store | Rejected by D-07 and would split dashboard/status history. [VERIFIED: 97-CONTEXT.md, fjall_store.rs] |
| Fixed `MetricKind` variants | Dynamic labels or dimensions | Rejected by D-05 and observability docs because peer ids, endpoints, permission classes, reasons, labels, and addresses must not become metric dimensions. [VERIFIED: 97-CONTEXT.md, operator-observability.md] |
| Pure mapper from status | Directly sampling network internals | Rejected by D-01 and D-02 because it would bypass canonical status and risk duplicate counter state. [VERIFIED: 97-CONTEXT.md, status/inbound.rs, network/inbound.rs] |
| Existing dashboard path | New inbound dashboard UI | Rejected by D-10; labels already exist and this phase only needs to prove retained samples render through the registered metric path. [VERIFIED: 97-CONTEXT.md, dashboard/model.rs] |

**Installation:** No new packages are recommended for Phase 97. [VERIFIED: Cargo.lock, 97-CONTEXT.md]

**Version verification:** Dependency versions above were verified from `Cargo.lock`, `rust-toolchain.toml`, `.bun-version`, and local `--version` commands. [VERIFIED: Cargo.lock, rust-toolchain.toml, .bun-version, rustc --version, cargo --version, bun --version]

## Architecture Patterns

### Recommended Project Structure

```text
packages/open-bitcoin-node/src/
|-- metrics.rs                 # add pure inbound status -> MetricSample mapper
|-- metrics/tests.rs           # mapping, no-status, low-cardinality tests
|-- sync/runtime_state.rs      # append sync + inbound samples through one store call
|-- storage/fjall_store.rs     # retain existing append API; add tests only if needed
`-- status/inbound.rs          # add missing numeric permission aggregates only if required

packages/open-bitcoin-cli/src/operator/dashboard/
`-- model.rs                   # register/render inbound metric kinds through existing chart path

scripts/
|-- check-phase97-inbound-metrics.ts
|-- check-phase97-inbound-metrics.test.ts
`-- verify.sh                  # wire Phase 97 test and checker after Phase 96
```

This structure keeps pure mapping in the node crate, storage in the existing Fjall adapter, dashboard projection in the existing CLI model, and verification in the existing checker family. [VERIFIED: metrics.rs, fjall_store.rs, dashboard/model.rs, scripts/check-phase96-peer-policy-runtime-bridge.ts]

### Pattern 1: Pure Status-To-Sample Mapper

**What:** Map `FieldAvailability<InboundPeerServingStatus>` to a `Vec<MetricSample>` using only fixed `MetricKind` variants and numeric aggregate fields. [VERIFIED: status.rs, status/inbound.rs, metrics.rs]

**When to use:** Use this for all inbound admission, permission, peer-policy, and resource-governance metric production in Phase 97. [VERIFIED: 97-CONTEXT.md]

**Example:**

```rust
// Source: local pattern from SyncRunSummary::metric_samples plus FieldAvailability.
pub fn inbound_metric_samples(
    inbound: &FieldAvailability<InboundPeerServingStatus>,
    timestamp_unix_seconds: u64,
) -> Vec<MetricSample> {
    let FieldAvailability::Available(status) = inbound else {
        return Vec::new();
    };

    vec![
        MetricSample::new(
            MetricKind::InboundAdmittedPeerCount,
            f64::from(status.admitted_inbound_peers),
            timestamp_unix_seconds,
        ),
        MetricSample::new(
            MetricKind::InboundRejectedPeerCount,
            f64::from(status.rejected_inbound_peers),
            timestamp_unix_seconds,
        ),
    ]
}
```

The full implementation should map every fixed inbound `MetricKind`; unavailable inbound status must return an empty vector. [VERIFIED: 97-CONTEXT.md, metrics.rs, status/inbound.rs]

### Pattern 2: Append One Combined Sample Batch

**What:** Build sync samples and inbound samples for the same timestamp, then pass one combined slice to `FjallNodeStore::append_metric_samples`. [VERIFIED: sync/runtime_state.rs, fjall_store.rs]

**When to use:** Use this at the runtime progress collection boundary so existing retention, bucket coalescing, and per-series caps stay centralized. [VERIFIED: metrics.rs, fjall_store.rs, 97-CONTEXT.md]

**Example:**

```rust
// Source: local persist_metrics and append_metric_samples contracts.
let mut samples = summary.metric_samples(timestamp);
samples.extend(inbound_metric_samples(&inbound_status, timestamp));
self.store.append_metric_samples(
    &samples,
    MetricRetentionPolicy::default(),
    timestamp,
    self.config.persist_mode,
)?;
```

The final integration boundary must avoid a second inbound counter source; pass the canonical status projection into the append boundary instead. [VERIFIED: 97-CONTEXT.md, status/inbound.rs, sync/runtime_state.rs]

### Pattern 3: Deterministic Checker Mirrors Phase 90-96

**What:** Add a Bun checker that reads target files, fails with actionable messages, and has mutation-style tests for missing mapper, storage append, dashboard registration, low-cardinality guardrails, and `verify.sh` wiring. [VERIFIED: scripts/check-phase96-peer-policy-runtime-bridge.ts, scripts/check-phase96-peer-policy-runtime-bridge.test.ts, scripts/verify.sh]

**When to use:** Use this for Phase 97 final verification in addition to Rust unit tests. [VERIFIED: 97-CONTEXT.md, scripts/verify.sh]

### Candidate Metric Mapping

| MetricKind | Source Aggregate | Planning Status |
|------------|------------------|-----------------|
| `InboundAdmittedPeerCount` | `admitted_inbound_peers` | Direct mapping exists. [VERIFIED: metrics.rs, status/inbound.rs] |
| `InboundRejectedPeerCount` | `rejected_inbound_peers` | Direct mapping exists. [VERIFIED: metrics.rs, status/inbound.rs] |
| `InboundCapRejectCount` | `cap_rejects` | Direct mapping exists. [VERIFIED: metrics.rs, status/inbound.rs] |
| `InboundReservedSlotRejectCount` | `reserved_slot_rejects` | Direct mapping exists. [VERIFIED: metrics.rs, status/inbound.rs] |
| `InboundDuplicateRejectCount` | `duplicate_rejects` | Direct mapping exists. [VERIFIED: metrics.rs, status/inbound.rs] |
| `InboundSelfConnectionRejectCount` | `self_connection_rejects` | Direct mapping exists. [VERIFIED: metrics.rs, status/inbound.rs] |
| `InboundPermissionedAdmitCount` | `permissioned_inbound_peers` | Direct mapping exists. [VERIFIED: metrics.rs, status/inbound.rs] |
| `InboundProtectedAdmitCount` | `protected_inbound_peers` | Direct mapping exists. [VERIFIED: metrics.rs, status/inbound.rs] |
| `InboundInactivePermissionEffectCount` | Missing numeric observation aggregate on status; network admission info has `inactive_permission_effect_observations`. | Add or expose a numeric status aggregate before mapping; do not use dynamic labels. [VERIFIED: metrics.rs, status/inbound.rs, network/inbound.rs, operator-observability.md] |
| `InboundPermissionValidationFailureCount` | No verified status aggregate or runtime producer found. | Open planning gap; add a status aggregate only if a real validation-failure producer exists, otherwise document and test the defensible zero path. [VERIFIED: metrics.rs, status/inbound.rs, rg permission_validation] |
| `InboundEvictionCandidateCount` | `eviction_candidates_evaluated` | Direct mapping exists. [VERIFIED: metrics.rs, status/inbound.rs] |
| `InboundDisconnectCount` | `disconnects_requested` | Direct mapping exists. [VERIFIED: metrics.rs, status/inbound.rs] |
| `InboundActiveBanCount` | `active_bans` | Direct mapping exists. [VERIFIED: metrics.rs, status/inbound.rs] |
| `InboundMisbehaviorObservationCount` | `misbehavior_observations` | Direct mapping exists. [VERIFIED: metrics.rs, status/inbound.rs] |
| `InboundProtectedNoActionCount` | `protected_no_actions` | Direct mapping exists. [VERIFIED: metrics.rs, status/inbound.rs] |
| `InboundResourcePressureActiveCount` | `resource_pressure_events` | Direct mapping exists. [VERIFIED: metrics.rs, status/inbound.rs] |
| `InboundReadQueuePressureCount` | `read_queue_pressure_events` | Direct mapping exists. [VERIFIED: metrics.rs, status/inbound.rs] |
| `InboundWriteQueuePressureCount` | `write_queue_pressure_events` | Direct mapping exists. [VERIFIED: metrics.rs, status/inbound.rs] |
| `InboundRequestCapReachedCount` | `request_cap_events` | Direct mapping exists. [VERIFIED: metrics.rs, status/inbound.rs] |
| `InboundPayloadRejectedCount` | `payload_rejections` | Direct mapping exists. [VERIFIED: metrics.rs, status/inbound.rs] |
| `InboundTimeoutDisconnectCount` | `timeout_disconnects` | Direct mapping exists. [VERIFIED: metrics.rs, status/inbound.rs] |
| `InboundChurnRejectedCount` | `churn_rejections` | Direct mapping exists. [VERIFIED: metrics.rs, status/inbound.rs] |
| `InboundReconnectSuppressedCount` | `reconnect_suppressions` | Direct mapping exists. [VERIFIED: metrics.rs, status/inbound.rs] |

### Anti-Patterns to Avoid

- **Sampling network internals directly:** This violates D-01 and D-02 because metrics would no longer derive from the canonical inbound status projection. [VERIFIED: 97-CONTEXT.md]
- **Manufacturing zero samples when inbound status is unavailable:** This violates D-03 and creates false availability evidence. [VERIFIED: 97-CONTEXT.md, status/inbound.rs]
- **Adding labels or dimensions:** This violates D-05 and the observability docs; metric samples have only kind, value, and timestamp. [VERIFIED: 97-CONTEXT.md, metrics.rs, operator-observability.md]
- **Creating a new retention path:** This violates D-07 and duplicates `append_and_prune_metric_samples`. [VERIFIED: 97-CONTEXT.md, metrics.rs, fjall_store.rs]
- **Adding dashboard UI before registering existing chart kinds:** This violates D-10; the existing chart path filters samples by registered kinds and labels them with `metric_label`. [VERIFIED: 97-CONTEXT.md, dashboard/model.rs]

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Retained metrics history | A new inbound metrics database, ring buffer, or JSON file | `FjallNodeStore::append_metric_samples` | Existing retention handles age windows, buckets, and per-series caps. [VERIFIED: fjall_store.rs, metrics.rs] |
| Metric vocabulary | Dynamic series names, labels, or dimensions | Existing fixed `MetricKind::Inbound*` variants | User decisions and docs require low-cardinality fixed series only. [VERIFIED: 97-CONTEXT.md, metrics.rs, operator-observability.md] |
| Source counters | New metrics-only admission/resource counters | `InboundPeerServingStatus` | D-01 and D-02 lock the shared inbound status projection as source of truth. [VERIFIED: 97-CONTEXT.md, status/inbound.rs] |
| Dashboard rendering | Custom inbound chart UI | `DASHBOARD_METRIC_KINDS`, `dashboard_charts`, and `metric_label` | Existing dashboard model already renders retained samples by metric kind. [VERIFIED: dashboard/model.rs] |
| Verification framework | A bespoke script style or shell-only checker | Phase 90-96 Bun checker pattern | Existing checkers are deterministic, tested, and wired through `scripts/verify.sh`. [VERIFIED: scripts/check-phase96-peer-policy-runtime-bridge.ts, scripts/verify.sh] |

**Key insight:** The hard part is preserving one source of truth. A direct network-to-metrics shortcut would be faster to write but would make status, support, metrics, and dashboard drift possible. [VERIFIED: 97-CONTEXT.md, status/inbound.rs, metrics.rs]

## Common Pitfalls

### Pitfall 1: Treating Existing Metric Kinds As Existing Sample Production

**What goes wrong:** The codebase already has inbound metric kinds and labels, so an implementation may skip runtime production. [VERIFIED: metrics.rs, dashboard/model.rs]

**Why it happens:** `MetricKind::ALL` and `metric_label` are complete, but `SyncRunSummary::metric_samples` produces only six sync/outbound samples today. [VERIFIED: metrics.rs, dashboard/model.rs, sync/types/summary.rs]

**How to avoid:** Add tests that fail unless a status snapshot with non-zero inbound counters produces retained inbound samples after `append_metric_samples`. [VERIFIED: fjall_store.rs, 97-CONTEXT.md]

**Warning signs:** Checker only searches for `MetricKind::Inbound*` names and not for a mapper, append path, and retained dashboard sample evidence. [VERIFIED: scripts/check-phase96-peer-policy-runtime-bridge.ts]

### Pitfall 2: Permission Metrics Need Numeric Status Aggregates

**What goes wrong:** `InboundInactivePermissionEffectCount` could accidentally count distinct inactive labels instead of inactive effect observations, and `InboundPermissionValidationFailureCount` could be manufactured as zero without a real source. [VERIFIED: operator-observability.md, status/inbound.rs, network/inbound.rs, rg permission_validation]

**Why it happens:** `ManagedInboundAdmissionInfo` tracks `inactive_permission_effect_observations`, but `InboundPeerServingStatus` currently exposes only `inactive_permission_effects`; no status field or producer for permission validation failures was found. [VERIFIED: network/inbound.rs, status/inbound.rs, rg permission_validation]

**How to avoid:** Make a Wave 0 planning task to close permission mapping gaps before wiring persistence. [VERIFIED: metrics.rs, status/inbound.rs]

**Warning signs:** Mapping uses `inactive_permission_effects.len()` without explaining the semantics, or emits `InboundPermissionValidationFailureCount` with a constant zero and no test documenting why. [VERIFIED: operator-observability.md, status/inbound.rs]

### Pitfall 3: Runtime Shells Are Separate

**What goes wrong:** The inbound listener and RPC context can collect inbound status while the sync runtime persists sync metrics separately, so a naive change may not have access to canonical inbound status at `persist_metrics`. [VERIFIED: open-bitcoind.rs, rpc/context/network.rs, sync/runtime_state.rs]

**Why it happens:** `ManagedRpcContext::current_inbound_status()` builds the inbound projection, while `DurableSyncRuntime::persist_metrics` currently accepts only `SyncRunSummary`. [VERIFIED: rpc/context/network.rs, sync/runtime_state.rs]

**How to avoid:** Pass the canonical inbound status projection across a boundary, or add a small runtime-facing provider that returns `FieldAvailability<InboundPeerServingStatus>`; do not recreate counters in `DurableSyncRuntime`. [VERIFIED: 97-CONTEXT.md, status/inbound.rs]

**Warning signs:** New counters appear in runtime state solely for metrics, or tests do not exercise a real status projection. [VERIFIED: 97-CONTEXT.md]

### Pitfall 4: Dashboard Labels Do Not Equal Dashboard Registration

**What goes wrong:** The planner may assume labels are enough for chart history, but `dashboard_charts` only renders kinds in `DASHBOARD_METRIC_KINDS`. [VERIFIED: dashboard/model.rs]

**Why it happens:** `metric_label` covers all inbound metric kinds, while `DASHBOARD_METRIC_KINDS` currently contains only eight non-inbound kinds. [VERIFIED: dashboard/model.rs]

**How to avoid:** If success criteria require inbound charts, register inbound fixed kinds in `DASHBOARD_METRIC_KINDS` and test that retained inbound samples produce chart points through `dashboard_charts`. [VERIFIED: ROADMAP.md, dashboard/model.rs]

**Warning signs:** Tests assert label coverage only, not retained inbound chart points. [VERIFIED: dashboard/model.rs]

### Pitfall 5: Public-Network Evidence Leaks Into Verification

**What goes wrong:** Tests or checker instructions could require real public inbound connectivity. [VERIFIED: 97-CONTEXT.md, operator-observability.md]

**Why it happens:** Inbound serving is network-facing by domain, but Phase 97 explicitly requires synthetic or loopback evidence only. [VERIFIED: 97-CONTEXT.md]

**How to avoid:** Use local fixtures, unit tests, temporary Fjall stores, and checker text scans; avoid live mainnet or public listener claims. [VERIFIED: 97-CONTEXT.md, fjall_store.rs, scripts/check-phase96-peer-policy-runtime-bridge.ts]

**Warning signs:** Checker or docs mention production readiness, public inbound defaults, relay support, or live-network telemetry as required for Phase 97. [VERIFIED: 97-CONTEXT.md, operator-observability.md]

## Code Examples

### Complete Mapper Test Shape

```rust
#[test]
fn inbound_metric_samples_maps_available_status_to_fixed_samples() {
    // Arrange
    let status = FieldAvailability::available(InboundPeerServingStatus {
        admitted_inbound_peers: 2,
        rejected_inbound_peers: 1,
        resource_pressure_events: 3,
        ..inbound_status_fixture()
    });

    // Act
    let samples = inbound_metric_samples(&status, 123);

    // Assert
    assert!(samples.iter().any(|sample| {
        sample.kind == MetricKind::InboundAdmittedPeerCount
            && sample.value == 2.0
            && sample.timestamp_unix_seconds == 123
    }));
}
```

This follows the existing unit-test style and keeps storage/network effects outside the mapper test. [VERIFIED: standards/core/testing.md, metrics/tests.rs, status/inbound/tests.rs]

### Storage Retention Test Shape

```rust
#[test]
fn append_metric_samples_retains_inbound_samples_through_existing_history() {
    // Arrange
    let store = fjall_store_fixture();
    let samples = vec![MetricSample::new(
        MetricKind::InboundResourcePressureActiveCount,
        4.0,
        1_000,
    )];

    // Act
    store
        .append_metric_samples(
            &samples,
            MetricRetentionPolicy::default(),
            1_000,
            PersistMode::Immediate,
        )
        .expect("fixture store should append metrics");
    let status = store
        .load_metrics_status(MetricRetentionPolicy::default())
        .expect("fixture store should load metrics status");

    // Assert
    assert!(status.samples.iter().any(|sample| {
        sample.kind == MetricKind::InboundResourcePressureActiveCount && sample.value == 4.0
    }));
}
```

This uses the existing append/load contract rather than a new history mechanism. [VERIFIED: fjall_store.rs, storage/fjall_store/tests.rs]

### Phase 97 Checker Targets

```typescript
const REQUIRED_INBOUND_METRICS = [
  "InboundAdmittedPeerCount",
  "InboundRejectedPeerCount",
  "InboundResourcePressureActiveCount",
  "InboundReconnectSuppressedCount",
] as const;
```

The actual checker should cover all fixed inbound kinds, the pure mapper, existing Fjall append path, dashboard registration/rendering, public-network-free wording, and `scripts/verify.sh` execution order. [VERIFIED: metrics.rs, dashboard/model.rs, scripts/check-phase96-peer-policy-runtime-bridge.ts, scripts/verify.sh]

## State of the Art

| Old Approach | Current Approach for Phase 97 | When Changed | Impact |
|--------------|--------------------------------|--------------|--------|
| Retained samples are produced only from `SyncRunSummary::metric_samples`. | Produce sync and inbound samples from one runtime progress collection path and persist them through `FjallNodeStore::append_metric_samples`. | Phase 97. [VERIFIED: ROADMAP.md, sync/types/summary.rs, sync/runtime_state.rs, fjall_store.rs] | Closes `INT-02` and `FLOW-02` for INB-05 and DOS-04. [VERIFIED: v1.9-MILESTONE-AUDIT.md] |
| Inbound metric kinds and labels exist without sample history. | Fixed inbound metric kinds get numeric samples derived from canonical inbound status. | Phase 97. [VERIFIED: metrics.rs, dashboard/model.rs, 97-CONTEXT.md] | Dashboard/status history can reflect real runtime inbound outcomes. [VERIFIED: ROADMAP.md] |
| Permission/resource observability remains in status/support/log evidence only. | Resource-governance and peer-policy counters become retained metric samples without adding labels or peer material. | Phase 97. [VERIFIED: status/inbound.rs, operator-observability.md] | DOS-04 can be audited through metrics as well as existing status/support/log paths. [VERIFIED: REQUIREMENTS.md, v1.9-MILESTONE-AUDIT.md] |

**Deprecated/outdated:**
- Treating metric kind registration as sufficient evidence is outdated for Phase 97; retained sample production must be proven. [VERIFIED: v1.9-MILESTONE-AUDIT.md]
- Relying on public-network evidence for default verification is out of scope; synthetic or loopback evidence is required. [VERIFIED: 97-CONTEXT.md, operator-observability.md]

## Assumptions Log

| # | Claim | Section | Risk if Wrong |
|---|-------|---------|---------------|
| A1 | No new external dependency is required for Phase 97. [ASSUMED] | Standard Stack | If a hidden runtime bridge requires a new crate, the planner must verify maintenance and security before adding it. |

## Open Questions (RESOLVED)

1. **What should back `InboundPermissionValidationFailureCount`?**
   - **RESOLVED:** Back it with a numeric config/admission validation aggregate on `RuntimeConfig` and `InboundPeerServingStatus`, sourced by a pure `count_inbound_permission_validation_failures` helper that reuses permission-class parsing and duplicate-address validation. It must not create dynamic labels or expose class names, tokens, addresses, reasons, or peer material.
   - What we know: The metric kind and dashboard label exist, but targeted search found no `InboundPeerServingStatus` field or obvious runtime producer for a permission validation failure counter. [VERIFIED: metrics.rs, dashboard/model.rs, status/inbound.rs, rg permission_validation]
   - What is unclear: Whether Phase 91 intended a future validation-failure producer or whether validation failures are currently impossible after config resolution. [VERIFIED: .planning/phases/91-peer-permissions-and-connection-classes/91-05-PLAN.md, rg permission_validation]
   - Recommendation: Make the first planning task implement and test the aggregate source before persistence wiring. [VERIFIED: 97-CONTEXT.md]

2. **Should `InboundInactivePermissionEffectCount` count observations or distinct labels?**
   - **RESOLVED:** Count inactive permission effect observations, not distinct inactive label values. Add a numeric `inactive_permission_effect_observations` aggregate to shared inbound status and map the metric from that field.
   - What we know: Observability docs say it counts inactive effect observations; network admission info tracks `inactive_permission_effect_observations`, but status exposes `inactive_permission_effects` labels. [VERIFIED: operator-observability.md, network/inbound.rs, status/inbound.rs]
   - What is unclear: Whether Phase 97 should add observation counts to `InboundPeerServingStatus` or reinterpret the metric as distinct inactive labels. [VERIFIED: 97-CONTEXT.md]
   - Recommendation: Prefer adding a numeric status aggregate sourced from existing admission info, because D-04 requires mapping from `InboundPeerServingStatus` and D-05 forbids labels as metric dimensions. [VERIFIED: 97-CONTEXT.md, network/inbound.rs, status/inbound.rs]

3. **Where should the runtime bridge obtain inbound status?**
   - **RESOLVED:** The runtime bridge should obtain canonical inbound status through `ManagedRpcContext::current_inbound_status()` and pass that `FieldAvailability<InboundPeerServingStatus>` into the metrics append boundary through a provider hook. It must not sample managed network internals directly or create metrics-only counters.
   - What we know: `ManagedRpcContext::current_inbound_status()` builds the canonical status projection, while `DurableSyncRuntime::persist_metrics` currently persists only sync samples. [VERIFIED: rpc/context/network.rs, sync/runtime_state.rs]
   - What is unclear: Whether the smallest implementation is to pass status into the sync runtime append boundary, add a callback/provider, or append from the RPC/open-bitcoind shell with the same Fjall store. [VERIFIED: open-bitcoind.rs, sync/runtime_state.rs]
   - Recommendation: Keep the mapper in `open-bitcoin-node` and choose the narrowest shell change that passes a `FieldAvailability<InboundPeerServingStatus>` without duplicating counters. [VERIFIED: 97-CONTEXT.md, status/inbound.rs]

4. **How many inbound metric kinds should dashboard charts register?**
   - **RESOLVED:** Keep the live dashboard chart row bounded. Preserve the existing eight-chart row capacity and add a deterministic selector that can substitute retained inbound series with samples into existing chart slots without adding a second row, new screen, metric picker, filter, or expanded chart count. Label coverage still remains available for every fixed inbound `MetricKind`.
   - What we know: Labels exist for every inbound kind, but `DASHBOARD_METRIC_KINDS` currently contains only eight non-inbound kinds. [VERIFIED: dashboard/model.rs]
   - What is unclear: Whether the dashboard should register all inbound kinds or a curated fixed subset for Phase 97 success criteria. [VERIFIED: ROADMAP.md, 97-CONTEXT.md]
   - Recommendation: Prove retained inbound samples render through existing `DashboardState::from_snapshot` and `metric_label` while keeping `state.charts.len() <= 8`; do not add a new dashboard UI. [VERIFIED: 97-CONTEXT.md, dashboard/model.rs, 97-UI-SPEC.md]

## Environment Availability

| Dependency | Required By | Available | Version | Fallback |
|------------|-------------|-----------|---------|----------|
| Rust toolchain | Rust mapper, runtime, storage, dashboard tests | Yes | `rustc 1.94.1`, `cargo 1.94.1` | None needed. [VERIFIED: rustc --version, cargo --version] |
| Bun | Phase 97 checker and checker tests | Yes | `1.3.9` | None needed. [VERIFIED: .bun-version, bun --version] |
| Bazel/Bazelisk command path | Repo-native verify smoke build | Yes | `bazel 8.6.0` | `scripts/verify.sh --fast` only for local iteration, not final verification. [VERIFIED: bazel --version, AGENTS.md] |
| cargo-llvm-cov | Repo-native coverage gate | Yes | `0.8.5` | None documented. [VERIFIED: cargo llvm-cov --version, scripts/verify.sh] |
| Git | Status, diff, optional research commit | Yes | `2.53.0` | None needed. [VERIFIED: git --version] |
| Public network | Phase 97 verification | Not required | N/A | Use synthetic or loopback evidence. [VERIFIED: 97-CONTEXT.md, operator-observability.md] |

**Missing dependencies with no fallback:** None found for planning Phase 97. [VERIFIED: command availability audit]

**Missing dependencies with fallback:** Public network access is not required and should be replaced with synthetic or loopback evidence. [VERIFIED: 97-CONTEXT.md]

## Security Domain

### Applicable ASVS Categories

| ASVS Category | Applies | Standard Control |
|---------------|---------|------------------|
| V2 Authentication | No direct change | Phase 97 should not alter RPC auth, cookies, or credentials. [VERIFIED: 97-CONTEXT.md] |
| V3 Session Management | No | No session mechanism is introduced. [VERIFIED: 97-CONTEXT.md] |
| V4 Access Control | No direct change | Preserve existing status/support exposure boundaries; do not add privileged APIs. [VERIFIED: 97-CONTEXT.md] |
| V5 Input Validation | Yes | Validate by construction: only fixed `MetricKind` variants and numeric aggregates enter retained samples. [VERIFIED: metrics.rs, status/inbound.rs] |
| V6 Cryptography | No | No cryptographic primitive or secret storage change is introduced. [VERIFIED: 97-CONTEXT.md] |
| V7 Error Handling and Logging | Yes | Keep support/status evidence aggregate and redacted; do not store raw peer ids, endpoints, reasons, permission strings, or dynamic labels in metrics. [VERIFIED: 97-CONTEXT.md, operator-observability.md] |
| V12 File and Resources | Yes | Use existing Fjall append/prune retention rather than unbounded files. [VERIFIED: fjall_store.rs, metrics.rs] |

ASVS category names were checked against the current OWASP ASVS repository. [CITED: github.com/OWASP/ASVS]

### Known Threat Patterns for Retained Inbound Metrics

| Pattern | STRIDE | Standard Mitigation |
|---------|--------|---------------------|
| Peer or endpoint deanonymization through metrics | Information disclosure | Store only fixed metric kind, numeric value, and timestamp; no peer ids, endpoints, labels, addresses, or raw reasons. [VERIFIED: 97-CONTEXT.md, metrics.rs] |
| Unbounded metric cardinality or storage growth | Denial of service | Use fixed `MetricKind::ALL` series and `MetricRetentionPolicy` through `append_and_prune_metric_samples`. [VERIFIED: metrics.rs, fjall_store.rs] |
| False availability evidence | Spoofing/repudiation | Return no inbound samples when inbound status is unavailable, per D-03. [VERIFIED: 97-CONTEXT.md, status/inbound.rs] |
| Runtime state divergence | Tampering/repudiation | Derive samples from `InboundPeerServingStatus` and avoid metrics-only duplicate counters. [VERIFIED: 97-CONTEXT.md, status/inbound.rs] |

## Sources

### Primary (HIGH confidence)

- `.planning/phases/97-inbound-metrics-sample-production/97-CONTEXT.md` - locked decisions, discretion, deferred scope. [VERIFIED: file read]
- `.planning/REQUIREMENTS.md` - `INB-05` and `DOS-04` wording and ownership. [VERIFIED: file read]
- `.planning/ROADMAP.md` - Phase 97 goal, plan split, success criteria. [VERIFIED: file read]
- `.planning/v1.9-MILESTONE-AUDIT.md` - `INT-02` and `FLOW-02` gap evidence. [VERIFIED: file read]
- `AGENTS.md`, `AGENTS.bright-builds.md`, `standards/*` - repo rules, verification, testing, architecture, Rust, TypeScript. [VERIFIED: file read]
- `packages/open-bitcoin-node/src/metrics.rs` - metric kinds, samples, retention, append/prune helper. [VERIFIED: file read]
- `packages/open-bitcoin-node/src/status/inbound.rs` - canonical inbound aggregate status fields. [VERIFIED: file read]
- `packages/open-bitcoin-node/src/network/inbound.rs` - admission permission observation counters. [VERIFIED: rg/sed]
- `packages/open-bitcoin-node/src/sync/types/summary.rs` - current sync-only sample producer. [VERIFIED: file read]
- `packages/open-bitcoin-node/src/sync/runtime_state.rs` - current persisted metrics append call. [VERIFIED: file read]
- `packages/open-bitcoin-node/src/storage/fjall_store.rs` - Fjall metrics history append/load path. [VERIFIED: file read]
- `packages/open-bitcoin-rpc/src/context/network.rs` - `ManagedRpcContext::current_inbound_status()` projection boundary. [VERIFIED: rg/sed]
- `packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs` and `packages/open-bitcoin-rpc/src/inbound_listener.rs` - daemon split between inbound listener/RPC context and sync runtime worker. [VERIFIED: rg/sed]
- `packages/open-bitcoin-cli/src/operator/dashboard/model.rs` - dashboard metric registration, chart derivation, labels. [VERIFIED: file read]
- `scripts/verify.sh` and `scripts/check-phase96-peer-policy-runtime-bridge.ts` - checker and verification pattern. [VERIFIED: file read]

### Secondary (MEDIUM confidence)

- `docs/architecture/operator-observability.md`, `docs/architecture/status-snapshot.md`, `docs/operator/runtime-guide.md` - documented low-cardinality and public-network-free observability constraints. [VERIFIED: file read]
- `https://github.com/OWASP/ASVS` - current ASVS category source for security-domain framing. [CITED: github.com/OWASP/ASVS]

### Tertiary (LOW confidence)

- None used for implementation guidance. [VERIFIED: research log]

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH - this phase uses existing repo-local Rust, Fjall, dashboard, and Bun checker contracts with no new dependencies. [VERIFIED: Cargo.lock, metrics.rs, fjall_store.rs, dashboard/model.rs]
- Architecture: MEDIUM-HIGH - pure mapper and existing append path are locked; exact runtime bridge placement remains a planner decision because RPC context and sync runtime are separate shells. [VERIFIED: 97-CONTEXT.md, rpc/context/network.rs, sync/runtime_state.rs]
- Pitfalls: HIGH - gaps are directly evidenced by audit findings and targeted code search. [VERIFIED: v1.9-MILESTONE-AUDIT.md, rg permission_validation, rg inactive_permission_effect_observations]
- Security: MEDIUM-HIGH - threats are local to metrics retention and cardinality; ASVS framing is cited, while project-specific controls are verified in code/docs. [CITED: github.com/OWASP/ASVS] [VERIFIED: 97-CONTEXT.md, metrics.rs]

**Research date:** 2026-06-28
**Valid until:** 2026-07-28
