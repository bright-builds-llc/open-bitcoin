# Phase 121: Block Relay Metrics and Log Runtime Projection - Research

**Researched:** 2026-07-14
**Domain:** DurableSyncRuntime metrics persist + structured-log projection for OBS-03
**Confidence:** HIGH

## Summary

OBS-03 is unsatisfied because Phase 116 already shipped pure helpers (`block_relay_metric_samples`, `block_relay_log_record`) with unit coverage, but `DurableSyncRuntime::persist_metrics` only appends sync + inbound samples, and sync structured logs never emit `block_relay` records. [VERIFIED: .planning/v2.1-MILESTONE-AUDIT.md] [VERIFIED: packages/open-bitcoin-node/src/sync/metrics.rs]

Phase 121 is a narrow runtime seam close: mirror the Phase 97 inbound provider → availability gate → append pattern for metrics, and emit `block_relay_log_record` through the existing `append_structured_record` shell path under the same availability gate. Helpers, MetricKinds, and fixed log vocabulary stay unchanged. Production wiring must feed evidence from the RPC `ManagedRpcContext` network (where block-serving/compact evidence is recorded), not from DurableSyncRuntime’s separate outbound-sync `ManagedPeerNetwork`. [VERIFIED: packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs] [VERIFIED: packages/open-bitcoin-node/src/network/inventory.rs]

**Primary recommendation:** Add one shared `set_block_relay_metric_status_provider` returning `FieldAvailability<BlockRelayEvidenceStatus>`; gate both `persist_metrics` sample append and structured-log emission on `Available`; wire the provider in `open-bitcoind` from `shared_context.block_relay_evidence_status()`; prove with DurableSyncRuntime tests + `scripts/check-phase121-*.ts` in `verify.sh`.

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions

#### Metric Source And Provider Wiring

- **D-01:** Treat `BlockRelayEvidenceStatus` from managed network evidence (`ManagedPeerNetwork::block_relay_evidence_status` / equivalent shared status projection) as the canonical source for block-relay metric samples. Do not duplicate counters.
- **D-02:** Mirror the Phase 97 inbound pattern: add a `DurableSyncRuntime` provider setter (for example `set_block_relay_metric_status_provider`) that returns `FieldAvailability<BlockRelayEvidenceStatus>` (or an equivalent available/unavailable wrapper), and call `block_relay_metric_samples` only when status is available.
- **D-03:** Extend `DurableSyncRuntime::persist_metrics` to append `block_relay_metric_samples(...)` alongside existing sync and inbound samples through `FjallNodeStore::append_metric_samples` and the existing retention policy. Do not create a parallel metrics store.

#### Persist Omission Semantics

- **D-04:** When block-relay status is unavailable, emit no block-relay metric samples (same posture as inbound D-03 in Phase 97). Do not manufacture zero-valued availability evidence that would imply runtime projection occurred.
- **D-05:** Reuse existing fixed `MetricKind` variants and the Phase 116 helper mapping unchanged. No new kinds, no peer ids, endpoints, permission strings, credentials, transaction payloads, or dynamic label dimensions.

#### Structured Log Emission Path

- **D-06:** Emit `block_relay_log_record` through the sync runtime structured-log path (same effectful append used by `write_summary_logs` / `append_structured_record`), not through a new log writer or by parsing log text.
- **D-07:** Emit the block-relay log record when the same availability condition as metrics is met (status available). Reuse the existing `block_relay_log_record` helper and its fixed low-cardinality `outcome`/`cause`/`label` vocabulary without adding sensitive or dynamic fields.
- **D-08:** Keep pure helpers side-effect-free. Filesystem append stays in the sync runtime shell adapter.

#### Verification And Leakage Guardrails

- **D-09:** Add runtime-level tests (DurableSyncRuntime persist/log path) that prove samples and log records appear when a provider returns available status, and are omitted when unavailable — beyond Phase 116 helper-only unit coverage.
- **D-10:** Prove no raw peer, permission, credential, or transaction payload leakage in persisted metric sample kinds/messages and emitted structured log records (reuse existing sanitization/redaction assertions patterns).
- **D-11:** Add a deterministic Phase 121 checker (Bun/TypeScript under `scripts/`) proving production-callable wiring into `persist_metrics` and structured-log emission, helper reuse, and verifier inclusion; wire it into `bash scripts/verify.sh`.
- **D-12:** Default verification remains `bash scripts/verify.sh` — deterministic, local, public-network-free.

#### Folded Todos

No pending todos matched this phase.

### Claude's Discretion

Exact provider type wrapper naming, whether log emission shares the metrics provider or a twin setter, module placement within `sync/`, checker script naming, and fixture construction are agent discretion — provided the Phase 97 mirror, availability-gated emission, helper reuse, and leakage guardrails hold.

### Deferred Ideas (OUT OF SCOPE)

None — discussion stayed within phase scope. Package relay, bloom/filter serving, public defaults, production readiness, and new operator UI remain out of scope.
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| OBS-03 | Metrics and structured logs use fixed low-cardinality labels for served, suppressed, compact-announced, reconstructed, missing-requested, fallback, malformed, timeout, and cleanup outcomes. | Wire Phase 116 helpers through DurableSyncRuntime persist + structured-log path; runtime tests + Phase 121 checker close the audit seam that left helpers unused outside unit tests. |
</phase_requirements>

## Project Constraints (from AGENTS.md / Bright Builds)

No `.cursor/rules/` directory found in this workspace. [VERIFIED: glob .cursor/rules]

Actionable repo constraints for this phase:

- Functional core / imperative shell: keep `block_relay_metric_samples` / `block_relay_log_record` pure; filesystem/store append only in sync/runtime shell. [CITED: standards/core/architecture.md]
- Prefer `?` / early returns; never add `unwrap()` in non-test production paths. [CITED: AGENTS.md repo-local / user code-styling]
- Verification contract: `bash scripts/verify.sh` (deterministic, public-network-free). [CITED: AGENTS.md]
- Bun for repo TypeScript checkers; no `package.json` / no `bun install`. [CITED: AGENTS.md]
- New/touched first-party Rust under `packages/open-bitcoin-*/src` or `tests` needs parity breadcrumbs via `docs/parity/source-breadcrumbs.json` unless explicit `none` is defensible. [CITED: AGENTS.md]
- Arrange / Act / Assert unit-test structure. [CITED: standards/core/testing.md]

## Standard Stack

### Core

| Library / Surface | Version | Purpose | Why Standard |
|-------------------|---------|---------|--------------|
| Rust toolchain | 1.94.1 | First-party crates | Pinned by `rust-toolchain.toml` / Cargo. [VERIFIED: rustc --version] |
| `open-bitcoin-node` | workspace | Sync runtime, metrics, logging, status | Owns DurableSyncRuntime + helpers. [VERIFIED: packages/open-bitcoin-node] |
| `FjallNodeStore::append_metric_samples` | existing | Retained metrics history | Same path as sync/inbound samples + retention. [VERIFIED: sync/metrics.rs] |
| `append_structured_log_record` / `append_structured_record` | existing | Datadir structured logs | Same shell path as `write_summary_logs`. [VERIFIED: runtime_state.rs] |
| Bun | 1.3.9 | Phase checker scripts | Repo-canonical TS automation runtime. [VERIFIED: bun --version] |

### Supporting

| Library / Surface | Version | Purpose | When to Use |
|-------------------|---------|---------|-------------|
| `FieldAvailability<T>` | existing | Available/unavailable outer gate | Provider return type; omit when Unavailable. [VERIFIED: status + Phase 97] |
| `BlockRelayEvidenceStatus` | existing | Shared evidence contract | Canonical sample/log input. [VERIFIED: status/block_relay_evidence.rs] |
| `MetricKind::{BlockServedCount,...CompactCleanupCount}` | existing | Fixed series vocabulary | Do not add kinds. [VERIFIED: metrics.rs] |
| `BLOCK_RELAY_LOG_SOURCE` (`"block_relay"`) | existing | Structured log source label | Assert in runtime log tests. [VERIFIED: logging.rs] |

### Alternatives Considered

| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| Provider setter (locked) | Call `self.network.block_relay_evidence_status()` inside DurableSyncRuntime | Sync runtime’s network is outbound-sync scoped; serving/compact evidence is recorded on RPC context’s ManagedPeerNetwork — reading sync network would stay permanently `default_unavailable`. [VERIFIED: network/inventory.rs vs sync.rs] |
| Twin log provider | Shared metrics provider for logs | Twin setters duplicate wiring; one provider keeps metrics/logs coherent (discretion recommendation). |
| Change helper to take `FieldAvailability` | Gate outside helper | Locked D-05/D-07: reuse helpers unchanged; inbound gates inside helper, block_relay does not — gate at call site. [VERIFIED: metrics.rs vs metrics/block_relay.rs] |
| New metrics store / log writer | Existing append APIs | Forbidden by D-03/D-06. |

**Installation:** None — reuse existing crates and Bun scripts. No new Cargo/npm dependencies.

**Version verification:** Rust `1.94.1`, Bun `1.3.9` confirmed on research host (2026-07-14).

## Architecture Patterns

### Recommended Project Structure

```
packages/open-bitcoin-node/src/
├── sync.rs                    # DurableSyncRuntime field + tick calls persist/logs
├── sync/metrics.rs            # provider setter + persist_metrics append
├── sync/runtime_state.rs      # write_block_relay_log (or extend write_summary_logs)
├── sync/tests.rs              # runtime persist + log emission/omission + leakage tests
├── metrics/block_relay.rs     # UNCHANGED helper
└── logging.rs                 # UNCHANGED block_relay_log_record

packages/open-bitcoin-rpc/src/bin/
├── open-bitcoind.rs           # wire set_block_relay_metric_status_provider beside inbound
└── open_bitcoind/…            # only if a small helper is needed for availability mapping

scripts/
├── check-phase121-block-relay-metrics-log-runtime.ts
├── check-phase121-block-relay-metrics-log-runtime.test.ts
└── verify.sh                  # wire bun test + bun run (visible + run_step)
```

### Pattern 1: Phase 97 Persist Extension (metrics)

**What:** Optional Arc provider on DurableSyncRuntime; `persist_metrics` builds sync samples, extends with family samples when provider is set, then one `append_metric_samples` call.

**When to use:** Any additional fixed metric family projected into retained history.

**Example (canonical inbound — mirror this):**

```rust
// Source: packages/open-bitcoin-node/src/sync/metrics.rs [VERIFIED]
pub fn set_inbound_metric_status_provider<F>(&mut self, provider: F)
where
    F: Fn() -> FieldAvailability<InboundPeerServingStatus> + Send + Sync + 'static,
{
    self.maybe_inbound_metric_status_provider = Some(Arc::new(provider));
}

pub(super) fn persist_metrics(
    &self,
    summary: &SyncRunSummary,
    timestamp: i64,
) -> Result<(), SyncRuntimeError> {
    let timestamp = u64::try_from(timestamp).unwrap_or(0);
    let summary = self.summary_with_configured_targets(summary);
    let mut samples = summary.metric_samples(timestamp);
    if let Some(provider) = self.maybe_inbound_metric_status_provider.as_ref() {
        samples.extend(inbound_metric_samples(&provider(), timestamp));
    }
    // Phase 121: same if-let for block_relay provider, but gate Available
    // before calling block_relay_metric_samples(&status, timestamp)
    self.store.append_metric_samples(
        &samples,
        MetricRetentionPolicy::default(),
        timestamp,
        self.config.persist_mode,
    )?;
    Ok(())
}
```

**Block-relay call-site gate (required asymmetry):** `inbound_metric_samples` accepts `&FieldAvailability<_>` and returns `Vec::new()` when Unavailable. [VERIFIED: metrics.rs] `block_relay_metric_samples` accepts `&BlockRelayEvidenceStatus` and always emits nine samples (nested unavailable fields become zeros). [VERIFIED: metrics/block_relay.rs] Calling it on `default_unavailable()` would persist zero-valued series and violate D-04. Therefore:

```rust
// Recommended call-site pattern [ASSUMED: exact helper name for unavailable]
if let Some(provider) = self.maybe_block_relay_metric_status_provider.as_ref() {
    if let FieldAvailability::Available(status) = provider() {
        samples.extend(block_relay_metric_samples(&status, timestamp));
    }
}
```

### Pattern 2: Shared Provider For Metrics And Logs (discretion)

**What:** One `maybe_block_relay_metric_status_provider` field; both `persist_metrics` and a `write_block_relay_log` (or equivalent) invoke the same provider.

**When to use:** Locked D-07 same availability condition; avoids twin wiring drift.

**Recommended placement:** Prefer a small `write_block_relay_log` next to `write_summary_logs` in `runtime_state.rs` that calls `append_structured_record(&block_relay_log_record(...))` when Available; invoke from the same sync tick that already calls `persist_metrics` then `write_summary_logs` in `sync.rs` (~lines 203–214). [VERIFIED: sync.rs]

### Pattern 3: Production Provider Wiring (open-bitcoind)

**What:** Beside inbound provider, set block-relay provider from `ManagedRpcContext::block_relay_evidence_status()`.

**Canonical inbound wiring:**

```rust
// Source: packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs [VERIFIED]
sync_runtime.set_inbound_metric_status_provider(move || {
    let Ok(context) = shared_context.try_lock() else {
        return inbound_status_unavailable();
    };
    context.current_inbound_status()
});
```

**Block-relay wiring recommendation:**

```rust
sync_runtime.set_block_relay_metric_status_provider(move || {
    let Ok(context) = shared_context.try_lock() else {
        return FieldAvailability::unavailable(BLOCK_SERVING_EVIDENCE_UNAVAILABLE_REASON);
        // or a dedicated BLOCK_RELAY_*_UNAVAILABLE_REASON constant [discretion]
    };
    let status = context.block_relay_evidence_status();
    match &status.block_serving.activation {
        FieldAvailability::Available(_) => FieldAvailability::available(status),
        FieldAvailability::Unavailable { .. } => {
            FieldAvailability::unavailable(BLOCK_SERVING_EVIDENCE_UNAVAILABLE_REASON)
        }
    }
});
```

Rationale: `BlockRelayEvidenceStatus::default_unavailable()` sets `block_serving.activation` to Unavailable with `BLOCK_SERVING_EVIDENCE_UNAVAILABLE_REASON` while nested counters may still be Available(zeros). [VERIFIED: status/block_relay_evidence.rs] Outer Unavailable must omit samples/logs.

### Pattern 4: Phase Checker (Bun static corpus)

**What:** Mirror `scripts/check-phase97-inbound-metrics.ts`: fixed TARGET_FILES, require needles for provider setter, persist append, log emission call, helper reuse, runtime test names, open-bitcoind wiring, verify.sh inclusion, and no-claim creep strings.

**When to use:** D-11 deterministic production-callable wiring proof without public network.

### Anti-Patterns to Avoid

- **Calling `block_relay_metric_samples` on `default_unavailable`:** Persists zero series → false “projection occurred” evidence (D-04).
- **Reading DurableSyncRuntime’s own `network.block_relay_evidence_status()` as sole production source:** Serving evidence is recorded on RPC-managed network inventory/announce paths, not the outbound sync network. [VERIFIED: network/inventory.rs, network.rs]
- **New MetricKind / dynamic labels / peer ids in log messages:** Violates D-05/D-07 and OBS-03.
- **Parsing log text to invent metrics:** Forbidden by D-06 and Phase 116 D-13.
- **Skipping open-bitcoind wiring:** Provider `None` means persist never extends (same as inbound); OBS-03 runtime seam stays open in daemon path.
- **Changing Phase 116 helper unit tests as the only proof:** Audit requires runtime projection beyond helper-only coverage.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Metric sample mapping | New mapper / kinds | `block_relay_metric_samples` | Already maps all OBS-03 counters to fixed MetricKinds. |
| Structured log shape | New log formatter | `block_relay_log_record` | Fixed outcome/cause/label vocabulary + source `block_relay`. |
| Metrics retention store | Parallel history DB | `FjallNodeStore::append_metric_samples` | Existing retention + dashboard/status consumers. |
| Log filesystem writer | New writer | `append_structured_record` | Same datadir log dir + retention as sync logs. |
| Availability wrapper | Custom Option/enum | `FieldAvailability<BlockRelayEvidenceStatus>` | Matches Phase 97 and status contracts. |
| Checker framework | Ad-hoc shell greps only | Bun `check-phase121-*.ts` + `.test.ts` | Matches Phase 97/99/116 verifier contract. |

**Key insight:** The hard part is not mapping — Phase 116 finished that. The hard part is availability-correct wiring from the RPC evidence network through DurableSyncRuntime’s persist/log tick without inventing zero availability samples.

## Common Pitfalls

### Pitfall 1: Zero-Sample False Projection

**What goes wrong:** Provider returns `Available(BlockRelayEvidenceStatus::default_unavailable())`; nine zero samples land in metrics history.
**Why it happens:** Helper treats nested Unavailable as 0, not “omit family.”
**How to avoid:** Outer `FieldAvailability` gate; map activation-Unavailable / lock failure to Unavailable.
**Warning signs:** Tests pass “samples present” with all zeros when no block-relay activity observed.

### Pitfall 2: Wrong Network As Evidence Source

**What goes wrong:** Sync runtime persists forever-empty block-relay series because its ManagedPeerNetwork never records serving evidence.
**Why it happens:** Confusing D-01’s ManagedPeerNetwork API with DurableSyncRuntime’s private sync network instance.
**How to avoid:** Production provider reads `ManagedRpcContext::block_relay_evidence_status()` (same network that records inventory/announce evidence).
**Warning signs:** Unit tests with injected Available status pass; daemon path never shows block-relay kinds.

### Pitfall 3: Metrics Wired, Logs Forgotten (or Twin Provider Drift)

**What goes wrong:** Samples appear; sync logs lack `source=block_relay`, or providers disagree.
**Why it happens:** D-06/D-07 are easy to implement incompletely if only `persist_metrics` is touched.
**How to avoid:** Shared provider; call log writer on same tick; tests assert both surfaces.
**Warning signs:** Checker only greps `persist_metrics`, not `block_relay_log_record` + `append_structured_record`.

### Pitfall 4: Leakage In Runtime Fixtures

**What goes wrong:** Runtime test fixtures embed peer endpoints, permission strings, tx hex, cookies into status or expected messages.
**Why it happens:** Copying rich network fixtures into observability tests.
**How to avoid:** Build minimal `BlockRelayEvidenceStatus::with_components(...)` aggregates (as in `logging/tests.rs`); assert absence of sensitive markers like Phase 116 log leakage tests.
**Warning signs:** Message contains `127.0.0.1`, `credential`, `cookie`, hex payloads, or dynamic label keys.

### Pitfall 5: Checker / verify.sh Partial Wiring

**What goes wrong:** Checker exists but only in one of the dual verify.sh lists (visible vs `run_step`).
**Why it happens:** `verify.sh` duplicates Phase 97/99/116 entries in two places. [VERIFIED: scripts/verify.sh]
**How to avoid:** Wire both; Phase 97 checker itself asserts verify.sh contains its commands — mirror that.
**Warning signs:** Local `bun run` green but pre-commit verify misses Phase 121.

### Pitfall 6: Crate Root Re-export Gap

**What goes wrong:** `sync/metrics.rs` cannot `use crate::block_relay_metric_samples` the way it uses `inbound_metric_samples`.
**Why it happens:** `lib.rs` pub-uses inbound/relay samples but not `block_relay_metric_samples`. [VERIFIED: lib.rs]
**How to avoid:** Add `block_relay_metric_samples` to crate root `pub use metrics::{...}` (or import `crate::metrics::block_relay_metric_samples` consistently).
**Warning signs:** Compile error or inconsistent import paths.

## Code Examples

### Runtime Persist Test Shape (mirror Phase 97)

```rust
// Source pattern: packages/open-bitcoin-node/src/sync/tests.rs
// persist_metrics_appends_inbound_status_samples_with_sync_samples [VERIFIED]
#[test]
fn persist_metrics_appends_block_relay_status_samples_with_sync_samples() {
    // Arrange
    let path = temp_store_path("metrics-block-relay");
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("store");
    let mut runtime = DurableSyncRuntime::open(store, sync_config()).expect("runtime");
    let block_relay = /* BlockRelayEvidenceStatus::with_components(...) */;
    runtime.set_block_relay_metric_status_provider(move || {
        FieldAvailability::available(block_relay.clone())
    });
    let summary = runtime.snapshot_summary();

    // Act
    runtime.persist_metrics(&summary, 1_777_225_022).expect("persist metrics");

    // Assert
    let metrics = runtime.store().load_metrics_snapshot()...;
    assert!(metrics.samples.iter().any(|s| {
        s.kind == MetricKind::CompactAnnouncedCount && s.value == /* fixture */
    }));
    // also assert SyncHeight still present
}
```

### Runtime Log Test Shape

```rust
// Source patterns: write_summary_logs + load_structured_log_records [VERIFIED: sync/tests.rs]
#[test]
fn write_block_relay_log_emits_when_status_available() {
    // Arrange: sync_config_with_log_dir, set provider Available
    // Act: runtime.write_block_relay_log(timestamp)  // or tick helper
    // Assert: records with source == BLOCK_RELAY_LOG_SOURCE,
    //         message contains outcome=projected cause=status_projection label=block_relay
    //         and does not contain peer/credential/cookie/hex markers
}
```

### Checker Needles (recommended minimum)

| Area | Required needles |
|------|------------------|
| Provider | `set_block_relay_metric_status_provider`, `FieldAvailability<BlockRelayEvidenceStatus>` |
| Persist | `block_relay_metric_samples`, `samples.extend`, `append_metric_samples` |
| Logs | `block_relay_log_record`, `append_structured_record` / write helper name |
| Tests | available + unavailable persist tests; available + unavailable log tests; leakage asserts |
| Daemon | `set_block_relay_metric_status_provider`, `block_relay_evidence_status` |
| Verify | `check-phase121-...` in both verify.sh regions |
| No-claim | reject package relay / public default / production readiness claim strings |

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| Helper-only OBS-03 (Phase 116) | Runtime projection via DurableSyncRuntime (Phase 121) | v2.1 audit gap closure | Metrics/logs carry block-relay series in retained paths |
| Inbound family via provider (Phase 97) | Same pattern for block_relay | Phase 97 established | Planner should copy, not invent |
| Phase 117 maps OBS → 116 | `expectedPhase("OBS-03")` → `"121"` | Gap remap in checker | Completing 121 aligns REQUIREMENTS ownership with Phase 117 checker. [VERIFIED: check-phase117-parity-uat-release-boundary.ts] |

**Deprecated/outdated:** Treating Phase 116 helper unit tests as sufficient OBS-03 evidence — audit explicitly rejected that.

## Assumptions Log

| # | Claim | Section | Risk if Wrong |
|---|-------|---------|---------------|
| A1 | Activation-Unavailable is the correct outer unavailable signal for production mapping of `BlockRelayEvidenceStatus` | Architecture Pattern 3 | Wrong gate may omit valid zero-counter observed evidence or emit default_unavailable zeros |
| A2 | Dedicated `BLOCK_RELAY_STATUS_UNAVAILABLE_REASON` vs reusing `BLOCK_SERVING_EVIDENCE_UNAVAILABLE_REASON` is discretionary and not locked | Pattern 3 | Naming inconsistency only if docs/checkers hardcode a specific reason string |
| A3 | open-bitcoind provider wiring is in-scope for Phase 121 (not deferred) because provider-None never projects | Pitfall 2 / Architecture | If planner omits daemon wiring, daemon OBS-03 remains broken while unit tests pass |

**If A1 needs confirmation:** Prefer documenting the activation-based gate in PLAN.md acceptance criteria; it matches `default_unavailable()` shape verified in status code.

## Open Questions (RESOLVED)

1. **Should unavailable reason be a new constant?**
   - What we know: Inbound uses `INBOUND_STATUS_UNAVAILABLE_REASON`; block serving uses `BLOCK_SERVING_EVIDENCE_UNAVAILABLE_REASON`.
   - What's unclear: Whether checkers/docs should mention a block-relay-specific reason string.
   - Recommendation: Reuse `BLOCK_SERVING_EVIDENCE_UNAVAILABLE_REASON` for lock-failure / default-unavailable mapping unless a one-line new constant improves clarity; do not invent dynamic reasons.
   - RESOLVED: Reuse `BLOCK_SERVING_EVIDENCE_UNAVAILABLE_REASON` for lock-failure / default-unavailable mapping; do not invent a dynamic or block-relay-specific reason string unless a one-line constant clearly improves clarity without changing gate semantics.

2. **Is a separate open-bitcoind worker needed (like inbound metrics worker)?**
   - What we know: Inbound has both DurableSyncRuntime provider and `start_inbound_metrics_worker` for sync-disabled persistence. [VERIFIED: open_bitcoind/inbound_metrics.rs]
   - What's unclear: Whether block-relay must also persist when sync is disabled.
   - Recommendation: Phase 121 success criteria name DurableSyncRuntime persist/log path only — do **not** require a twin sync-disabled worker unless discuss-phase expands scope. Sync tick + provider wiring satisfies the audit seam.
   - RESOLVED: No sync-disabled twin worker in Phase 121 — DurableSyncRuntime persist/log path + provider wiring closes OBS-03; expand only if a future discuss-phase adds sync-disabled scope.

## Environment Availability

| Dependency | Required By | Available | Version | Fallback |
|------------|------------|-----------|---------|----------|
| rustc / cargo | Runtime tests + build | ✓ | 1.94.1 | — |
| bun | Phase 121 checker | ✓ | 1.3.9 | — |
| bash scripts/verify.sh | D-12 verification | ✓ | repo script | — |
| Public network | — | n/a | — | Must not be required |

**Missing dependencies with no fallback:** None.

**Missing dependencies with fallback:** None.

Step 2.6: External tools probed; no blockers.

## Security Domain

### Applicable ASVS Categories

| ASVS Category | Applies | Standard Control |
|---------------|---------|------------------|
| V2 Authentication | no | N/A — no auth surface change |
| V3 Session Management | no | N/A |
| V4 Access Control | no | N/A |
| V5 Input Validation | yes | Fixed MetricKind enum + fixed log label vocabulary; no free-form operator labels |
| V6 Cryptography | no | N/A — no new crypto |
| V7 Error Handling / Logging | yes | Structured logs must not leak peer endpoints, permissions, credentials, tx payloads (D-10) |

### Known Threat Patterns for observability projection

| Pattern | STRIDE | Standard Mitigation |
|---------|--------|---------------------|
| High-cardinality label explosion | Information Disclosure / DoS | Fixed MetricKind + fixed log labels only (D-05/D-07) |
| Peer/PII leakage into metrics/logs | Information Disclosure | Aggregate counters only; reuse Phase 116 sanitization; leakage asserts in runtime tests |
| Credential/cookie leakage | Information Disclosure | Forbid sensitive markers in fixtures and emitted messages (logging/tests.rs pattern) |
| Fabricated availability zeros | Spoofing / Tampering of evidence | Omit family when Unavailable (D-04) |

## Sources

### Primary (HIGH confidence)

- `.planning/phases/121-.../121-CONTEXT.md` — locked D-01..D-12
- `.planning/v2.1-MILESTONE-AUDIT.md` — OBS-03 unsatisfied evidence
- `.planning/REQUIREMENTS.md` / `.planning/ROADMAP.md` — OBS-03 → Phase 121
- `packages/open-bitcoin-node/src/sync/metrics.rs` — Phase 97 persist pattern
- `packages/open-bitcoin-node/src/metrics/block_relay.rs` — helper signature + kinds
- `packages/open-bitcoin-node/src/logging.rs` — `block_relay_log_record`
- `packages/open-bitcoin-node/src/sync/runtime_state.rs` — `write_summary_logs` / `append_structured_record`
- `packages/open-bitcoin-node/src/sync.rs` — tick order persist → summary logs
- `packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs` — inbound provider wiring
- `packages/open-bitcoin-rpc/src/context/network.rs` — `block_relay_evidence_status`
- `scripts/check-phase97-inbound-metrics.ts` — checker template
- `scripts/verify.sh` — dual wiring sites for Phase 97/99/116
- `scripts/check-phase117-parity-uat-release-boundary.ts` — OBS-03 expected phase `121`

### Secondary (MEDIUM confidence)

- `.planning/phases/97-.../97-CONTEXT.md` — inbound canonical decisions
- `.planning/phases/116-.../116-CONTEXT.md` — OBS-03 vocabulary locks
- `.planning/phases/99-.../99-CONTEXT.md` — structured-log emission + sanitization posture
- `docs/architecture/operator-observability.md` — low-cardinality block-relay constraints

### Tertiary (LOW confidence)

- None material; phase is codebase-pattern research, not external library selection.

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH — all surfaces exist in-repo; versions probed
- Architecture: HIGH — Phase 97 mirror + evidence-source split verified in code
- Pitfalls: HIGH — D-04 zero-sample and wrong-network pitfalls verified against helper/status code

**Research date:** 2026-07-14
**Valid until:** 2026-08-14 (stable internal seam; revisit if DurableSyncRuntime metrics path is refactored)
