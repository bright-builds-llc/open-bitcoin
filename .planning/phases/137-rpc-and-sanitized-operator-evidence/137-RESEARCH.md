# Phase 137: RPC and Sanitized Operator Evidence - Research

**Researched:** 2026-08-19
**Domain:** Knots-shaped package RPC plus identifier-free operator evidence projection
**Confidence:** HIGH

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions

### Package RPC and CLI contract

- **D-01:** Add Knots-named `testmempoolaccept` and `submitpackage` as
  `MethodOrigin::BaselineParity`. Preserve pinned Knots 29.3 invocation,
  parameter names, and result shapes. RPC-level errors stay limited to
  decode, count, and topology failures; per-transaction outcomes stay in the
  result body.
- **D-02:** Do not add extra Open Bitcoin keys onto those BaselineParity
  objects. Knots `allowed`, `reject-reason`, `package-error`, `package_msg`,
  `tx-results`, and `replaced-transactions` keep their Knots meanings.
- **D-03:** Project both Knots methods from the existing Phase 132
  `PackageReport`. Dry-run maps to the dry-run command and must leave mempool,
  rolling fee, relay, persistence, and evidence state unchanged. Submission
  maps to the submission command and must match authoritative final membership.
- **D-04:** Expose the typed Open Bitcoin package report on a separate
  `MethodOrigin::OpenBitcoinExtension` RPC and an `open-bitcoin package`
  operator path (`dry-run` and `submit`). That surface carries input-ordered
  member results, package-wide status, fingerprint, effective-fee groups, and
  dual-state admission/relay fields. `open-bitcoin-cli` continues to forward
  baseline method names; `open-bitcoin` owns the extension workflow.
- **D-05:** Both projections consume the same authoritative `PackageReport`.
  Planner and researcher must keep one projector family so Knots JSON and the
  typed report cannot drift.

### Shared evidence redaction

- **D-06:** Transaction identifiers, package fingerprints, and detailed
  per-member results may appear only on the originating authenticated RPC/CLI
  response that supplied them (`testmempoolaccept`, `submitpackage`,
  `sendrawtransaction`, and the Open Bitcoin package extension).
- **D-07:** `OpenBitcoinStatusSnapshot`, `openbitcoinnetworkstatus`, operator
  status, dashboard, metrics, structured logs, and support bundles stay
  identifier-free. They may use only fixed low-cardinality labels and
  aggregate counts. Status RPC is authenticated but is not the originating
  response; do not echo last-package member tables there.
- **D-08:** Support bundles remain shareable after the existing recursive
  redaction path. They must not copy txids, wtxids, package hashes, raw
  transaction hex, peer ids, endpoints, credentials, or dynamic labels.
- **D-09:** `getmempoolinfo` stays identity-free. Future Knots query RPCs such
  as `getrawmempool` are out of this phase unless required to project
  originating-response identifiers; they are not shared evidence.

### Operator field vocabulary and dashboard

- **D-10:** Extend the shared `OpenBitcoinStatusSnapshot.mempool` with
  distinct typed groups rather than inventing a parallel evidence DTO:
  resources, fee floors, pressure, eviction, checkpoint, recovery, and retry.
- **D-11:** Keep Phase 130 field separation on every surface. Virtual size,
  accounted usage, and accounted capacity are distinct. The four fee values
  stay distinct: static relay floor, rolling mempool floor, effective
  admission (`max(static, rolling)`), and incremental relay fee.
- **D-12:** Preserve Knots RPC aliases as compatibility only:
  `getmempoolinfo.bytes` is total vsize, `usage` is accounted memory,
  `maxmempool` is accounted capacity, and `mempoolminfee` is the effective
  admission floor. Dashboard, status JSON, metrics, logs, and support bundles
  use Open Bitcoin labels, not overloaded Knots names.
- **D-13:** Pressure removals, relay `evicted_count`, checkpoint outcomes,
  recovery drop counters, and retry counters are different concepts. Do not
  fold Phase 136 retry evidence into `rebroadcast_deferred_count`.
- **D-14:** Terminal dashboard consumes the same snapshot groups. Add or swap
  rows within the existing Ratatui 8-chart/slot discipline. Do not add a
  hosted web dashboard or a second local model.

### Local admission versus relay-suppressed truth

- **D-15:** Present two independent axes. Admission states are `accepted`,
  `still-present`, and `cleared`. Relay/fanout states are `eligible`,
  `queued`, `attempted`, `emitted`, `requested`, `served`, `suppressed`, and
  `relay_disabled`. A transaction can be accepted and still-present while
  relay is `suppressed` or `relay_disabled`.
- **D-16:** BaselineParity `sendrawtransaction` remains the Knots txid-only
  success shape. Dual-state for single-tx and package submission lives on the
  Open Bitcoin extension/operator path and on shared aggregate evidence. Do
  not change Knots success JSON into a propagation claim.
- **D-17:** `EligibleServe` stays a semantic serve classification. It is not
  membership-clear evidence and is not public or guaranteed delivery.
  Unbroadcast still clears only on `TransportWritten` or `LifecycleRemoval`.
- **D-18:** Do not add `propagated`, `broadcast`, `public_relay`, or
  guaranteed-delivery fields. `queued`, `attempted`, `emitted`, `requested`,
  and `served` are local transport facts. Successful local admission never
  implies public/default relay or network-wide propagation.

### Claude's Discretion

- Exact extension method and clap command names, provided BaselineParity
  method names stay `testmempoolaccept` and `submitpackage`.
- Exact nested snapshot field names and dashboard row titles, provided the
  seven groups in D-10 stay distinct and Knots aliases stay RPC-only.
- Whether the typed package extension is one RPC method with a mode argument
  or two methods, provided dry-run cannot mutate.
- MetricKind and structured-log key spelling, provided they are fixed,
  low-cardinality, and match the snapshot labels.

### Deferred Ideas (OUT OF SCOPE)

- Integrated Knots parity catalog closeout, adversarial pressure,
  benchmarks, and claim-guardrail enforcement — Phase 138.
- `getrawmempool`, `getmempoolentry`, and other high-cardinality mempool
  query RPCs unless a later phase owns them.
- Hosted or web dashboards.
- General package wire protocol, BIP331, arbitrary multi-parent peer
  reconstruction, whole-mempool rebroadcast.
- Public/default/production relay, guaranteed propagation, public-network
  CI, and production-readiness claims.

None — discussion stayed within phase scope.
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| MPOBS-01 | RPC and CLI expose scoped package dry-run, submission, and mempool-info with stable errors and per-transaction results matching the authoritative core | One projector family from `PackageReport`; Knots-named BaselineParity methods plus typed extension; local handle dry-run/submit through existing Phase 132 commands |
| MPOBS-02 | Status, dashboard, metrics, logs, and support distinguish vsize, accounted usage, capacity, fee floors, pressure/decay, eviction, checkpoint, recovery, and retry with fixed low-cardinality fields | Extend `OpenBitcoinStatusSnapshot.mempool` with seven typed groups; project the same groups through `openbitcoinnetworkstatus`, renderers, `MetricKind`, and structured logs |
| MPOBS-03 | Shared evidence is redacted and distinguishes accepted, still-present, eligible, queued, attempted, emitted, requested, served, suppressed, and cleared; identifiers stay on the originating authenticated response | Dual-state axes on the typed originating report; shared surfaces expose only aggregate counts; reuse Phase 105/108 redaction; never copy `MempoolRecoveryRecord.txid` or package fingerprints into snapshot/support |
</phase_requirements>

## Summary

Phase 137 is an adapter-projection phase, not a new admission engine. The authoritative core already exists: Phase 132 `PackageReport` / `DryRunPackageCommand` / `SubmitPackageCommand`, Phase 130 resource and fee roles on `ManagedMempoolInfo` and `getmempoolinfo`, Phase 134 identifier-free lifecycle aggregates, Phase 135 `CheckpointEvidenceSnapshot` and recovery counts, and Phase 136 retry/fanout receipts. What is missing is the operator contract: Knots-named package RPCs, a typed Open Bitcoin package surface, and seven distinct identifier-free groups on the shared snapshot.

The highest-risk seams are already visible in the live tree. `testmempoolaccept` and `submitpackage` are absent from `SupportedMethod`. Local operator package dry-run/submit is not on `ManagedNetworkHandle` (only peer package admission and singleton `sendrawtransaction` exist). `MempoolStatus` still holds only `transactions` plus Phase 105 `relay`. Operator status copies `getmempoolinfo.size` and `openbitcoinnetworkstatus.relay` and discards the Phase 130 fee/resource fields. `LifecycleEvidenceSnapshot` and `CheckpointEvidenceSnapshot` are authority-private and unpublished. Phase 136 retry facts must not be folded into `rebroadcast_deferred_count`.

**Primary recommendation:** Add one projector family over `PackageReport`, two BaselineParity RPCs with exact Knots 29.3 shapes, one `openbitcoinpackage` extension plus `open-bitcoin package {dry-run,submit}`, local handle methods that route dry-run to the non-mutating command and submit through `LifecycleCommand::PackageAdmission` with `AdmissionProjectionSource::Local`, and seven snapshot groups published through `openbitcoinnetworkstatus.mempool` so every renderer consumes one model.

## Project Constraints (from AGENTS.md / standards)

No `.cursor/rules/` directory exists in this repo. The following constraints are binding for planning:

- Functional core / imperative shell: projectors and snapshot mapping stay pure; RPC, CLI, dashboard, logs, and storage stay in shell adapters. `[VERIFIED: AGENTS.md, standards/core/architecture.md]`
- Do not use existing Rust Bitcoin libraries in the production path. `[VERIFIED: AGENTS.md]`
- Rust `1.94.1` / edition 2024; Cargo workspace under `packages/`; Bazel/Bzlmod for the smoke build. `[VERIFIED: rustc --version, AGENTS.md]`
- `bash scripts/verify.sh` is the verification contract. Do not run it during research. Default verification stays hermetic. `[VERIFIED: AGENTS.md]`
- UAT guidance must include copy-pasteable repo-local Cargo and Bazel commands, not only the installed `open-bitcoin` alias. `[VERIFIED: AGENTS.md, docs/parity/service-operation-expectations.md]`
- New first-party Rust sources under `packages/open-bitcoin-*/src` or `tests` need parity breadcrumbs via `docs/parity/source-breadcrumbs.json`. Use explicit `none` when no Knots anchor exists. `[VERIFIED: AGENTS.md]`
- Prefer early returns and `let...else`. Prefix optional names with `maybe`. Never `unwrap()` in production. `[VERIFIED: standards/core/code-shape.md, user code-styling]`
- Treat files over ~628 lines and functions over ~161 lines as refactor triggers. Several Phase 137 touch files are already near that gate. `[VERIFIED: standards/core/code-shape.md, wc -l]`
- Unit-test pure projector and mapping logic with Arrange / Act / Assert, one concept per test. `[VERIFIED: standards/core/testing.md]`
- Dashboard remains the existing Ratatui dark terminal UI. Do not add a web dashboard. `[VERIFIED: D-14, standards/core/frontend-ui.md]`
- `standards-overrides.md` has no active overrides (placeholder row only). `[VERIFIED: standards-overrides.md]`

## Standard Stack

This phase adds no new production crates. Use the repo-owned stack.

### Core

| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| Rust | 1.94.1 (2024 edition) | First-party implementation | Pinned by `rust-toolchain.toml` `[VERIFIED: rustc --version]` |
| serde / serde_json | workspace pin | Stable RPC and snapshot JSON | Existing RPC/status contract `[VERIFIED: packages/open-bitcoin-rpc]` |
| clap | workspace pin | `open-bitcoin package` subcommands | Existing operator CLI `[VERIFIED: packages/open-bitcoin-cli/src/operator.rs]` |
| Ratatui / Crossterm | workspace pin | Terminal dashboard rows | Existing 8-chart dashboard `[VERIFIED: dashboard/app.rs]` |
| Tokio / Axum | workspace pin | JSON-RPC server dispatch | Existing `open-bitcoind` `[VERIFIED: AGENTS.md stack]` |

### Supporting

| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| Bun | 1.3.14 | Breadcrumb / checker scripts | Parity breadcrumb updates `[VERIFIED: bun --version]` |
| Bazelisk / Bazel | installed | Repo-local UAT command form | Operator docs and UAT `[VERIFIED: command -v]` |
| Fjall | workspace pin | Durable stores already used by status/metrics | Do not invent a second evidence store `[VERIFIED: AGENTS.md]` |

### Alternatives Considered

| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| One `openbitcoinpackage` method with `mode` | Two extension methods | Two methods duplicate request/result types; one method keeps one projector entry. Dry-run mutation is a routing bug either way. Prefer one method. |
| Extending `getmempoolinfo` with package/retry keys | Snapshot groups + extension status | Locked by D-02/D-09/D-12. Knots aliases stay RPC-only. |
| New evidence DTO beside the snapshot | Seven groups on `MempoolStatus` | Locked by D-10. |
| Web dashboard | Ratatui row additions | Locked by D-14. |
| bitcoin crate / rust-bitcoin JSON helpers | Repo-owned hex decode + serde | Forbidden production-path Bitcoin libraries. `[VERIFIED: AGENTS.md]` |

**Installation:** none. Use existing workspace dependencies.

**Version verification:** Rust 1.94.1 (2026-03-25), Cargo 1.94.1, Bun 1.3.14 confirmed on the research host. `[VERIFIED: rustc/cargo/bun --version]`

## Architecture Patterns

### Recommended Project Structure

```
packages/open-bitcoin-rpc/src/
├── method.rs                    # register BaselineParity + extension names only
├── method/node.rs               # keep getmempoolinfo / sendrawtransaction
├── method/package.rs            # NEW: request/response types (split before 628)
├── dispatch.rs                  # match arms only
├── dispatch/node.rs             # existing node methods
└── dispatch/package.rs          # NEW: decode, count, topology, project, dispatch
packages/open-bitcoin-rpc/src/package_projection.rs
                                 # NEW: one projector family (pure)
packages/open-bitcoin-node/src/
├── status.rs                    # re-export; do not grow inline (594 lines)
├── status/mempool_groups.rs     # NEW: seven typed groups
├── network/admission_bridge/
│   └── local_package.rs         # NEW: local dry-run + submit handle methods
└── metrics.rs / logging.rs      # split if new kinds/keys push over 628
packages/open-bitcoin-cli/src/operator/
├── operator.rs                  # add Package(PackageArgs) only
└── package.rs                   # NEW: clap + RPC client for extension
```

Split before adding. Current sizes: `status.rs` 594, `dashboard/model.rs` 605, `metrics.rs` 599, `operator/status.rs` 553, `dispatch/node.rs` 382. `[VERIFIED: wc -l]`

### Pattern 1: One Projector Family Over PackageReport

**What:** One pure module maps `PackageReport` (plus optional dual-state facts for the extension only) into (1) Knots `testmempoolaccept` array JSON, (2) Knots `submitpackage` object JSON, and (3) the typed Open Bitcoin package report. No third mapping in CLI or tests.

**When to use:** Every package RPC/CLI success path.

**Why this is required:** D-05. Knots JSON and the typed report must not drift. `PackageReport` already checks input order, status, and fee-group eligibility. `[VERIFIED: packages/open-bitcoin-mempool/src/package/report.rs]`

**Do not put dual-state on `PackageReport` itself.** The core type has fingerprint, status, members, and fee groups only. `[VERIFIED: PackageReport fields]` Attach admission/relay axes in the extension DTO at projection time from local lifecycle/relay facts.

### Pattern 2: Dual RPC Origin

**What:** Register four methods:

| Method | Origin | Shape |
|--------|--------|-------|
| `testmempoolaccept` | `BaselineParity` | Knots array of per-tx objects |
| `submitpackage` | `BaselineParity` | Knots object with `package_msg`, `tx-results`, `replaced-transactions` |
| `openbitcoinpackage` | `OpenBitcoinExtension` | Typed report + dual-state; required `mode` = `dry-run` \| `submit` |
| existing `getmempoolinfo` | `BaselineParity` | Keep current aliases + Phase 130 extras; no package keys |

`open-bitcoin-cli` forwards baseline names once they are in `SupportedMethod`. `open-bitcoin package dry-run|submit` calls `openbitcoinpackage` over the existing authenticated HTTP client. `[VERIFIED: args.rs validate_supported_method, operator.rs command enum, D-04]`

**Dry-run routing:** `mode=dry-run` and `testmempoolaccept` call `DryRunPackageCommand` only. Assert mempool count, rolling fee, relay counters, dirty generation, and evidence counters are unchanged. `[VERIFIED: DryRunPackageCommand in package.rs; D-03]`

**Submit routing:** `mode=submit` and `submitpackage` call `SubmitPackageCommand` then `LifecycleCommand::PackageAdmission` with `AdmissionProjectionSource::Local`. Do not reuse the peer 1P1C bridge. `[VERIFIED: admission_bridge/package.rs is peer-only]`

### Pattern 3: Shared Snapshot Groups, One Collector Path

**What:** Extend `MempoolStatus` with seven optional/available groups. Publish the same object on `openbitcoinnetworkstatus.mempool`. Operator status already reads that RPC and today only copies `relay`. `[VERIFIED: operator/status.rs:231-256]`

Recommended machine names (discretion; groups locked by D-10):

```text
mempool.resources.{virtual_size, accounted_usage, accounted_capacity, transaction_count}
mempool.fee_floors.{static_relay_floor, rolling_mempool_floor, effective_admission_floor, incremental_relay_fee}
mempool.pressure.{pressure_removal_count, decay_half_life_label, occupancy_band}
mempool.eviction.{pressure_removal_count}   # distinct from relay.outcome_counters.evicted_count
mempool.checkpoint.{outcome, overdue, persistence_strength, age_seconds, loss_bound_seconds, dirty_generation_present}
mempool.recovery.{recovered_count, dropped_*_count}  # counts only
mempool.retry.{eligible, queued, attempted, emitted, requested, served, suppressed, relay_disabled, cleared}
mempool.admission.{accepted, still_present, cleared}  # aggregate dual-state for MPOBS-03
```

`mempool.relay` stays the Phase 105/107/108 contract. Do not overload it. `[VERIFIED: status/relay_evidence.rs, D-13]`

Stopped-node: every new group is `Unavailable` with the same reason already used for `mempool.transactions`. `[VERIFIED: operator/status.rs:306-309]`

### Pattern 4: Dashboard Rows, Not a Ninth Chart

**What:** Add or replace rows in the existing "Mempool and Wallet" section. Keep `MAX_DASHBOARD_CHARTS = 8`. Do not add charts for the seven groups. `[VERIFIED: dashboard/model/metrics.rs]`

Row titles (discretion): `Virtual size`, `Accounted usage`, `Accounted capacity`, `Static relay floor`, `Rolling mempool floor`, `Effective admission floor`, `Incremental relay fee`, `Pressure removals`, `Eviction (relay)`, `Checkpoint`, `Recovery`, `Retry`, `Admission states`, `Relay states`.

### Anti-Patterns to Avoid

- **Re-implementing package policy in RPC:** RPC decodes hex, checks count/topology, then calls the core. `[VERIFIED: PITFALLS.md Pitfall 1]`
- **Looping `sendrawtransaction` for packages:** Wrong partial-accept and mutation semantics. `[VERIFIED: PITFALLS.md Technical Debt]`
- **Echoing last package on status RPC:** Forbidden by D-07 even though status RPC is authenticated.
- **Copying `MempoolRecoveryRecord` into snapshot/support:** Records contain `txid`. `[VERIFIED: storage/mempool_snapshot.rs:145-148]`
- **Using Knots names on dashboard/metrics/logs:** `bytes`, `usage`, `maxmempool`, `mempoolminfee` are RPC aliases only. `[VERIFIED: D-12]`
- **Throwing a Knots-style broadcast error from `submitpackage`:** Knots calls `BroadcastTransaction` after accept and can throw. Open Bitcoin must not turn fanout failure into a package RPC error or a propagation claim. Catalog this as an intentional difference. `[VERIFIED: knots mempool.cpp:1401-1417; D-15/D-18]`
- **Adding `propagated` / `broadcast` / `public_relay` fields:** Locked by D-18.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Package validation and membership | RPC-local accept loop | `DryRunPackageCommand` / `SubmitPackageCommand` + `PackageReport` | Core already owns shape, fees, partial accept, post-trim `[VERIFIED: mempool package.rs]` |
| Hex / tx decode | Custom parsers | Existing RPC `decode::decode_hex` + `parse_transaction` | Same path as `sendrawtransaction` `[VERIFIED: dispatch/node.rs:280-283]` |
| Status/dashboard/support model | Parallel DTOs | `OpenBitcoinStatusSnapshot` | Sole shared model `[VERIFIED: docs/architecture/status-snapshot.md]` |
| Support redaction | New recursive walker | `support_status_for_bundle` + `redact_relay_mempool_evidence` | Phase 105/108 path already shareable `[VERIFIED: support/redaction.rs]` |
| Relay/retry transport | New fanout path | Existing Phase 136 enqueue + receipts | D-17/D-18; IBR already implemented |
| Metrics history | Ad-hoc series | Fixed `MetricKind` + `MetricRetentionPolicy` | Low-cardinality contract `[VERIFIED: operator-observability.md]` |
| CLI baseline forwarding | Second CLI parser | `SupportedMethod` + existing `open-bitcoin-cli` transport | `from_name` gates local normalize `[VERIFIED: args.rs:453-460]` |
| Operator HTTP/auth | New client | Existing `HttpStatusRpcClient` / CLI RPC client | Cookie/Basic already implemented |

**Key insight:** Phase 137 fails if adapters invent a second truth. Project the core report and the shared snapshot; do not re-summarize in renderers.

## Common Pitfalls

### Pitfall 1: Drifting Knots JSON vs Typed Report

**What goes wrong:** Tests pass on one surface while the other mis-maps `AlreadyPresent`, `PostTrimAbsent`, or fee groups.
**Why it happens:** Two hand-written serializers.
**How to avoid:** One projector; table-driven tests feed one `PackageReport` and assert both JSON trees plus the typed DTO.
**Warning signs:** CLI tests construct Knots JSON independently of RPC tests.

### Pitfall 2: testmempoolaccept vs submitpackage Result Vocabulary

**What goes wrong:** The same member variant is serialized with the wrong Knots keys.
**Why it happens:** Knots uses two different result schemas. `[VERIFIED: knots mempool.cpp:169-193 and 1295-1318]`

| `PackageMemberResult` | `testmempoolaccept` | `submitpackage` `tx-results[wtxid]` |
|-----------------------|---------------------|-------------------------------------|
| `FinallyPresent` | `allowed=true`, `vsize`, `fees.base`, `effective-feerate`, `effective-includes` | `vsize`, `fees` including effective fields |
| `AlreadyPresent` | Knots package dry-run does not allow `MEMPOOL_ENTRY` (`CHECK_NONFATAL`) — map as RPC-level policy/`allowed=false` with a stable reject-reason, or document the single-tx ProcessTransaction path | `MEMPOOL_ENTRY`: `vsize` + `fees.base` **without** effective-feerate |
| `SameTxidDifferentWitness` | `allowed=false` + reject-reason | `other-wtxid` |
| `HardRejected` / `Reconsiderable` | `allowed=false`, `reject-reason`, optional `reject-details` | `error` string |
| `PostTrimAbsent` | `allowed=false`, Knots `mempool full` style reason | `error` reflecting final absence |
| package-wide shape/policy | optional `package-error` on each element; later txs may omit `allowed` | `package_msg` != `success`; missing eval → `error=unevaluated` |

`testmempoolaccept` is a **JSON array**. `submitpackage` is a **JSON object**. `[VERIFIED: knots mempool.cpp]`

### Pitfall 3: RPC Errors vs Result-Body Outcomes

**What goes wrong:** Policy rejects become JSON-RPC errors, or decode failures become per-tx `allowed=false`.
**Why it happens:** D-01 limits RPC errors to decode, count, and topology.
**How to avoid:**

| Failure | Knots code | Open Bitcoin mapping |
|---------|------------|----------------------|
| empty / `> MAX_PACKAGE_COUNT` (25) | `RPC_INVALID_PARAMETER` (-8) | Add `RpcErrorCode` for -8; do not use only `-32602` |
| TX decode failed | `RPC_DESERIALIZATION_ERROR` (-22) | Add `RpcErrorCode` for -22 |
| `submitpackage` topology not child-with-parents | `JSONRPCTransactionError` / `RPC_VERIFY_ERROR` (-25) | Add or reuse a -25 code; current enum has no -25 `[VERIFIED: error.rs, protocol.h]` |
| member policy / fee / TRUC / RBF | result body | never an RPC error |

Current `RpcErrorCode` has `-32602`, `-26`, `-1` and lacks `-8`, `-22`, `-25`. `[VERIFIED: packages/open-bitcoin-rpc/src/error.rs]`

### Pitfall 4: Mutating on Dry-Run

**What goes wrong:** Dry-run bumps rolling fee, dirties checkpoint, or increments relay/retry counters.
**Why it happens:** Submit path is reused, or dry-run goes through lifecycle apply.
**How to avoid:** Dry-run calls `Mempool::dry_run_package` only. No `LifecycleCommand`. Tests compare a before/after authority snapshot.

`ManagedMempool` currently exposes `submit_package` / `prepare_package` and not a public dry-run facade. Add `dry_run_package` on the handle. `[VERIFIED: node/mempool.rs]`

### Pitfall 5: Identifier Leak Into Shared Evidence

**What goes wrong:** Support bundles or metrics include txids, wtxids, fingerprints, hex, peer ids, or dynamic reject strings.
**Why it happens:** Recovery records and package reports are convenient to serialize whole.
**How to avoid:** Snapshot groups take counts and fixed labels only. Extend `redact_relay_mempool_evidence` if new fields can hold free text. Recursive redaction must still drop 64-hex strings. `[VERIFIED: D-06..D-08, support/redaction.rs, PITFALLS.md]`

### Pitfall 6: Overloading Phase 105 Relay Counters

**What goes wrong:** Retry emissions increment `rebroadcast_deferred_count`, or pressure trims increment `evicted_count` as the only eviction story.
**Why it happens:** Those counters already exist and look similar. `[VERIFIED: RelayEvidenceCounters; STATE.md Phase 136: rebroadcast_deferred means first hop recorded and retry cycle not yet run]`
**How to avoid:** New `mempool.retry.*` and `mempool.pressure.pressure_removal_count` / `mempool.eviction` groups. Keep Phase 105 counters unchanged in meaning.

### Pitfall 7: Describing Admission as Propagation

**What goes wrong:** Docs, help text, dashboard, or logs say broadcast/propagated/public relay after local accept.
**Why it happens:** Knots `submitpackage` help warns about this and then still calls `BroadcastTransaction`. `[VERIFIED: knots mempool.cpp:1276, 1401-1417; PITFALLS.md Pitfall 10]`
**How to avoid:** Dual-state axes. Relay-disabled accept shows `accepted` + `still-present` and `relay_disabled`. No new claim fields. Phase 138 owns guardrail closeout, but Phase 137 copy must already use the bounded vocabulary.

### Pitfall 8: sendrawtransaction Shape Confusion

**What goes wrong:** Dual-state or replaced/evicted lists get treated as Knots success JSON, or this phase rewrites the existing object and breaks tests.
**Why it happens:** Live `SendRawTransactionResponse` is `{txid_hex, replaced_txids, evicted_txids}`, serialized as a JSON object, not a Knots hex string. Tests assert `txid_hex`. `[VERIFIED: method/node.rs:124-128, dispatch.rs:62-64, dispatch/tests/transaction_methods.rs:61]`
**How to avoid:** Do not add dual-state or propagation fields to `sendrawtransaction`. Leave the existing object alone in this phase. Put single-tx dual-state on `openbitcoinpackage` with a one-member package (or the same typed report). Record the object-vs-hex-string gap for Phase 138; D-16's intent is "no propagation claim," not a wire rewrite.

### Pitfall 9: Client maxfeerate / maxburnamount / ignore_rejects

**What goes wrong:** Baseline scripts pass Knots optional args and get silent ignore or a crash.
**Why it happens:** Core has no `ignore_rejects`. `sendrawtransaction` already fail-closes on explicit `maxfeerate` / `maxburnamount`. `[VERIFIED: dispatch/node.rs:271-277; grep ignore_rejects in mempool = none]`
**How to avoid:** Accept the Knots parameter names on BaselineParity requests. Empty / omitted / default-zero stay compatible. Non-empty `ignore_rejects` or explicit non-default max fee/burn fail closed with a stable invalid-params message, same precedent as Phase 8 `sendrawtransaction`. Do not silently drop.

### Pitfall 10: File-Length and Collector Drift

**What goes wrong:** Groups exist on the snapshot type but status/dashboard/support/metrics/logs each invent a subset.
**Why it happens:** Operator status currently maps only `size` → `transactions`. `[VERIFIED: operator/status.rs:253-256]`
**How to avoid:** Daemon publishes groups on `openbitcoinnetworkstatus`. Collector copies them. Renderers read snapshot fields only. Split files before adding.

## Code Examples

Verified patterns from this repo and the pinned Knots tree.

### Register BaselineParity vs Extension

```rust
// Source: packages/open-bitcoin-rpc/src/method.rs
pub const fn origin(self) -> MethodOrigin {
    match self {
        Self::OpenBitcoinNetworkStatus
        | Self::OpenBitcoinSyncStatus
        | Self::OpenBitcoinSyncPause
        | Self::OpenBitcoinSyncResume
        | Self::BuildTransaction
        | Self::BuildAndSignTransaction => MethodOrigin::OpenBitcoinExtension,
        _ => MethodOrigin::BaselineParity,
    }
}
```

Add `TestMempoolAccept` and `SubmitPackage` to the `_ => BaselineParity` arm. Add `OpenBitcoinPackage` to the extension arm. Keep `scope()` as `MethodScope::Node`.

### Knots testmempoolaccept RPC-level count error

```cpp
// Source: packages/bitcoin-knots/src/rpc/mempool.cpp
if (raw_transactions.size() < 1 || raw_transactions.size() > MAX_PACKAGE_COUNT) {
    throw JSONRPCError(RPC_INVALID_PARAMETER,
                       "Array must contain between 1 and " + ToString(MAX_PACKAGE_COUNT) + " transactions.");
}
```

`MAX_PACKAGE_COUNT` is 25. Match the message closely enough that existing scripts keep working. `[VERIFIED: knots mempool.cpp:208-211]`

### getmempoolinfo alias precedent (do not copy onto package results)

```rust
// Source: packages/open-bitcoin-rpc/src/dispatch/node.rs
Ok(GetMempoolInfoResponse {
    size: info.transaction_count,
    bytes: info.total_virtual_size,
    usage: info.accounted_memory,
    maxmempool: info.mempool_capacity,
    mempoolminfee: info.effective_admission_fee_rate_sats_per_kvb,
    minrelaytxfee: info.static_relay_fee_rate_sats_per_kvb,
    incrementalrelayfee: info.incremental_relay_fee_rate_sats_per_kvb,
    rollingmempoolfee: info.rolling_mempool_fee_rate_sats_per_kvb,
    effectiveadmissionfee: info.effective_admission_fee_rate_sats_per_kvb,
    capacityenforcement: info.capacity_enforcement.as_str().to_string(),
    loaded: true,
    // ...
})
```

Identity-free extras on `getmempoolinfo` are allowed. Extra keys on `testmempoolaccept` / `submitpackage` objects are not. `[VERIFIED: D-02, D-09, method/node.rs:42-55]`

### Dry-run must not apply lifecycle

```rust
// Source: packages/open-bitcoin-mempool/src/package.rs
pub struct DryRunPackageCommand {
    pub package: WellFormedPackage,
    pub context: AdmissionContext,
}
pub struct DryRunPackageResult {
    pub report: PackageReport,
}
```

Submit pairs the same report with `MempoolLifecycleDelta`. Projector input for Knots JSON is the report; extension dual-state may use the delta plus current relay/unbroadcast facts after submit only.

### Support redaction extension point

```rust
// Source: packages/open-bitcoin-cli/src/operator/support/redaction.rs
pub(crate) fn support_status_for_bundle(
    mut status: OpenBitcoinStatusSnapshot,
) -> OpenBitcoinStatusSnapshot {
    redact_relay_mempool_evidence(&mut status.mempool.relay);
    // NEW: redact any free-text group fields; counts need no redaction
    status
}
```

### UAT command form (copy-pasteable)

```bash
# Source pattern: docs/parity/service-operation-expectations.md
cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --bin open-bitcoind -- \
  -datadir=/tmp/open-bitcoin-mainnet -server=1
bazel run //packages/open-bitcoin-rpc:open_bitcoind -- \
  -datadir=/tmp/open-bitcoin-mainnet -server=1

cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin-cli -- \
  -datadir=/tmp/open-bitcoin-mainnet testmempoolaccept '["<hex>"]'
bazel run //packages/open-bitcoin-cli:open_bitcoin_cli -- \
  -datadir=/tmp/open-bitcoin-mainnet testmempoolaccept '["<hex>"]'

cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- \
  --datadir=/tmp/open-bitcoin-mainnet package dry-run --hex '<hex>'
bazel run //packages/open-bitcoin-cli:open_bitcoin -- \
  --datadir=/tmp/open-bitcoin-mainnet package dry-run --hex '<hex>'

cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- \
  --datadir=/tmp/open-bitcoin-mainnet status --format json
bazel run //packages/open-bitcoin-cli:open_bitcoin -- \
  --datadir=/tmp/open-bitcoin-mainnet status --format json
```

Exact `open-bitcoin-cli` Bazel target name must match the live BUILD label when plans are written; prefer the target already used in `docs/parity/service-operation-expectations.md` for `open-bitcoin` and the existing CLI package target for baseline forwarding.

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| Singleton `sendrawtransaction` only | Local package dry-run/submit via Phase 132 commands | Phase 132 core done; RPC missing | Phase 137 adds the operator seam |
| `MempoolStatus { transactions, relay }` | Seven additional groups beside `relay` | Phase 105/108 added relay only | Status collector must stop discarding fee/resource fields |
| Peer package admission only | Local `AdmissionProjectionSource::Local` | Phase 133/134 peer path | Do not reuse 1P1C origins for RPC |
| `rebroadcast_deferred_count` as the retry story | Distinct retry group | Phase 136 defined first-hop vs retry cycle | D-13 forbids folding |
| Identifier-complete recovery records | Count-only operator recovery | Phase 134/135 | Never serialize `MempoolRecoveryRecord` on shared surfaces |

**Deprecated/outdated:**

- Treating `getmempoolinfo` extras as permission to decorate Knots package result objects. `[VERIFIED: D-02, CONTEXT specifics]`
- Describing v2.2 as general package relay or public/default relay. `[VERIFIED: REQUIREMENTS FUT-12..FUT-17, ROADMAP Phase 138]`

## Assumptions Log

| # | Claim | Section | Risk if Wrong |
|---|-------|---------|---------------|
| A1 | Discretion names (`openbitcoinpackage`, snapshot field spellings, row titles) are acceptable as planned defaults | Architecture Patterns | User may want different names; groups and baseline method names stay locked |
| A2 | Leaving `sendrawtransaction` as the existing `{txid_hex,...}` object satisfies D-16 for this phase | Pitfall 8 | If D-16 is read as "emit a Knots hex string now," existing tests and clients break |
| A3 | Not throwing Knots `BroadcastTransaction` failures from `submitpackage` is the correct intentional difference | Pitfall 7 | Baseline scripts that expect a broadcast-error throw will diverge; catalog in Phase 138 |
| A4 | Fail-closed unsupported `ignore_rejects` / explicit max fee/burn is enough for MPOBS-01 | Pitfall 9 | Scripts that rely on those Knots options will not work until a later phase |

**If A1 is rejected:** rename only; do not change group count or BaselineParity method names.

## Open Questions

1. **Exact Bazel label for `open-bitcoin-cli`**
   - What we know: service-operation docs show `//packages/open-bitcoin-cli:open_bitcoin` for the operator binary.
   - What's unclear: the baseline-forwarding binary target name in the live BUILD file.
   - Recommendation: copy the existing verified target from the CLI package BUILD when writing UAT; do not invent a label.

2. **Decay occupancy-band labels**
   - What we know: Phase 131 uses 12/6/3-hour half-lives by occupancy. `[CITED: REQUIREMENTS PRESS-03, FEATURES.md]`
   - What's unclear: whether snapshot `pressure.decay_half_life_label` should be `half_life_12h` / `6h` / `3h` or a numeric seconds field.
   - Recommendation: fixed labels `half_life_12h`, `half_life_6h`, `half_life_3h`, plus `not_decaying` before a post-bump block. No wall-clock ETA.

3. **Retry counter sources**
   - What we know: `LifecycleEvidenceSnapshot.retry_clears` and relay counters exist; Phase 136 enqueue/receipt path exists. `[VERIFIED: lifecycle_projection.rs, relay_fanout.rs]`
   - What's unclear: whether queued/attempted/emitted are already counted separately from announced/requested/served.
   - Recommendation: planner inventories the live fanout/serving counters first; add only missing fixed fields; do not invent a second counter authority.

## Environment Availability

Step 2.6 applies: plans will run Cargo/Bazel/Bun commands.

| Dependency | Required By | Available | Version | Fallback |
|------------|------------|-----------|---------|----------|
| rustc / cargo | Unit tests, binaries | ✓ | 1.94.1 | — |
| bun | Breadcrumb / checker scripts | ✓ | 1.3.14 | — |
| bazel / bazelisk | UAT command form, smoke build | ✓ | installed | Cargo form still required in docs |
| bitcoin-knots submodule | RPC shape / breadcrumb anchors | ✓ (path present) | 29.3.knots20260210 | `git submodule update --init --recursive` if missing |
| Public network / live peers | — | n/a | — | Do not use; hermetic fixtures only |

**Missing dependencies with no fallback:** none identified.

**Missing dependencies with fallback:** none identified.

## Security Domain

### Applicable ASVS Categories

| ASVS Category | Applies | Standard Control |
|---------------|---------|-----------------|
| V2 Authentication | yes | Existing RPC cookie / Basic auth; package methods stay authenticated node methods |
| V3 Session Management | no | No new session surface |
| V4 Access Control | yes | Originating identifiers only on the authenticated call that supplied the hex; status RPC is not that call |
| V5 Input Validation | yes | Hex decode, count 1..=25, topology check, `deny_unknown_fields` on extension requests |
| V6 Cryptography | no | No new crypto; do not add Bitcoin library helpers |

### Known Threat Patterns for this stack

| Pattern | STRIDE | Standard Mitigation |
|---------|--------|---------------------|
| Package hex DoS (huge arrays / oversized txs) | Denial of Service | Fail closed on count/weight/shape before expensive validation; reuse Phase 132 limits |
| Identifier leak in shareable bundles | Information Disclosure | D-06..D-08; redaction path; counts only |
| Credential copy into support | Information Disclosure | Existing bundle rules; metadata-only credential reporting `[VERIFIED: operator-observability.md, service-operation-expectations.md]`
| Dynamic metric/log labels | Information Disclosure | Fixed `MetricKind` and allowlisted log keys |
| Confused deputy: status RPC treated as originating package response | Information Disclosure | D-07; no last-package tables on `openbitcoinnetworkstatus` |
| Claim inflation (public relay / guaranteed delivery) | Spoofing / Elevation | Dual-state vocabulary; no `propagated`/`broadcast`/`public_relay` fields |

## Sources

### Primary (HIGH confidence)

- `.planning/phases/137-rpc-and-sanitized-operator-evidence/137-CONTEXT.md` — locked D-01..D-18
- `.planning/REQUIREMENTS.md` — MPOBS-01..03, FUT-12..17
- `.planning/ROADMAP.md` — Phase 137 success criteria vs Phase 138
- `packages/bitcoin-knots/src/rpc/mempool.cpp` — `testmempoolaccept`, `submitpackage`, `getmempoolinfo`
- `packages/bitcoin-knots/src/rpc/protocol.h` — `-8`, `-22`, `-25`, `-26`
- `packages/open-bitcoin-mempool/src/package/report.rs` — `PackageReport`
- `packages/open-bitcoin-rpc/src/method.rs`, `method/node.rs`, `dispatch/node.rs`, `error.rs`
- `packages/open-bitcoin-node/src/status.rs`, `network/types.rs`, `network/admission_bridge/package.rs`
- `packages/open-bitcoin-cli/src/operator.rs`, `status.rs`, `dashboard/model.rs`, `support/redaction.rs`
- `docs/architecture/status-snapshot.md`, `docs/architecture/operator-observability.md`
- `docs/parity/service-operation-expectations.md`, `docs/parity/catalog/rpc-cli-config.md`
- `AGENTS.md`, `AGENTS.bright-builds.md`, `standards/core/{architecture,code-shape,testing,frontend-ui,verification}.md`

### Secondary (MEDIUM confidence)

- `.planning/research/FEATURES.md` — local package surfaces and operator outcomes
- `.planning/research/PITFALLS.md` — admission-as-broadcast and identifier leaks
- `.planning/STATE.md` — Phase 130/136 counter meanings

### Tertiary (LOW confidence)

- None. No unverified web claims were used.

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH — pinned repo toolchain, no new libraries
- Architecture: HIGH — live code + locked CONTEXT; remaining naming is discretion
- Pitfalls: HIGH — Knots source, current Open Bitcoin gaps, and milestone PITFALLS.md agree

**Research date:** 2026-08-19
**Valid until:** 2026-09-18 (137 planning/execution window; Knots pin is stable)

## RESEARCH COMPLETE

**Phase:** 137 - RPC and Sanitized Operator Evidence
**Confidence:** HIGH

### Key Findings

- `PackageReport` is authoritative; RPC/CLI must project it twice (Knots JSON + typed extension) from one module.
- Local dry-run/submit handle methods do not exist yet; peer package admission is the wrong entry point.
- Shared snapshot still lacks the seven D-10 groups; status currently keeps only `size` and `relay`.
- Dual-state and identifiers stay on originating package/extension responses; recovery records contain txids and must not be published.
- Do not fold retry into `rebroadcast_deferred_count`, do not add propagation fields, and do not rewrite `sendrawtransaction` in this phase.

### File Created

`.planning/phases/137-rpc-and-sanitized-operator-evidence/137-RESEARCH.md`

### Confidence Assessment

| Area | Level | Reason |
|------|-------|--------|
| Standard Stack | HIGH | Existing workspace stack; versions verified locally |
| Architecture | HIGH | Code + locked decisions; names are discretion only |
| Pitfalls | HIGH | Knots RPC source and live Open Bitcoin seams checked |

### Open Questions

Exact `open-bitcoin-cli` Bazel label, decay-band label spelling, and which retry counters already exist vs need new fields.

### Ready for Planning

Research complete. Planner can now create PLAN.md files.
