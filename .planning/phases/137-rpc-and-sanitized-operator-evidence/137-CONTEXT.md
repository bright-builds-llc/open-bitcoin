---
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: 137-2026-08-19T21-52-42
generated_at: 2026-08-19T21:55:22.609Z
---

# Phase 137: RPC and Sanitized Operator Evidence - Context

**Gathered:** 2026-08-19
**Status:** Ready for planning
**Mode:** Yolo

<domain>
## Phase Boundary

Operators can inspect and exercise the scoped package and long-lived mempool
behavior through one stable, redacted evidence contract.

This phase delivers MPOBS-01, MPOBS-02, and MPOBS-03. It adapts the already
authoritative Phase 132 package reports, Phase 130 fee/resource roles, Phase
134 lifecycle evidence, Phase 135 checkpoint/recovery facts, and Phase 136
retry/fanout receipts onto RPC, CLI, status, dashboard, metrics, logs, and
support bundles.

It does not add a general package wire protocol, arbitrary multi-parent
assembly, whole-mempool rebroadcast, public/default/production relay,
guaranteed propagation, public-network CI, or production-readiness claims
(Phase 138 owns parity, adversarial pressure, and claim-guardrail closeout).

</domain>

<decisions>
## Implementation Decisions

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

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Active milestone contract

- `.planning/ROADMAP.md` — Phase 137 goal, success criteria, UI hint, and
  separation from Phase 138.
- `.planning/REQUIREMENTS.md` — MPOBS-01, MPOBS-02, MPOBS-03, and v2.2
  exclusions (FUT-12 through FUT-17).
- `.planning/research/ARCHITECTURE.md` — Operator adapter layering over
  authoritative core reports.
- `.planning/research/FEATURES.md` — Local package surfaces and
  relay-disabled admission reporting.
- `.planning/research/PITFALLS.md` — Successful local admission described as
  guaranteed broadcast; identifier leaks in shared evidence.
- `.planning/research/SUMMARY.md` — Synthesized v2.2 operator-evidence
  boundary.

### Prior locked decisions

- `.planning/phases/130-resource-time-and-fee-primitives/130-CONTEXT.md` —
  Distinct vsize/usage/capacity and the four fee-floor roles; Knots
  `getmempoolinfo` aliases.
- `.planning/phases/132-typed-package-vocabulary-and-staged-admission/132-CONTEXT.md` —
  `PackageReport`, dry-run versus submit, input-ordered member results.
- `.planning/phases/134-authoritative-cross-cache-lifecycle-integration/134-CONTEXT.md` —
  Production evidence is identifier-free aggregates; exact IDs stay
  test-only or originating-response-only.
- `.planning/phases/135-snapshot-schema-checkpointing-and-recovery/135-CONTEXT.md` —
  Checkpoint and recovery facts that Phase 137 may render, not redefine.
- `.planning/phases/136-receive-independent-maintenance-and-transport-receipts/136-CONTEXT.md` —
  Retry/fanout receipts, `EligibleServe` semantics, and TransportWritten
  clear.

### Shared operator evidence contract

- `docs/architecture/status-snapshot.md` — `OpenBitcoinStatusSnapshot` is
  the sole shared status model.
- `docs/architecture/operator-observability.md` — Phase 100/105/107/108
  redaction, fixed labels, and shareable support-bundle rules.
- `docs/architecture/config-precedence.md` — Credential reporting is
  metadata-only; bundles must not copy secrets.
- `docs/parity/service-operation-expectations.md` — Repo-local Cargo and
  Bazel operator command forms for UAT.
- `docs/operator/runtime-guide.md` — Existing status, dashboard, and
  support-bundle operator workflows.

### Pinned Bitcoin Knots behavior

- `packages/bitcoin-knots/src/rpc/mempool.cpp` — `getmempoolinfo`,
  `testmempoolaccept`, and `submitpackage` shapes and error boundaries.
- `packages/bitcoin-knots/src/rpc/rawtransaction.cpp` — `sendrawtransaction`
  txid-only success.
- `packages/bitcoin-knots/src/rpc/protocol.h` — RPC error codes used by
  package and mempool methods.

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets

- `packages/open-bitcoin-mempool/src/package/report.rs` — `PackageReport` and
  `PackageMemberResult` (`FinallyPresent`, `AlreadyPresent`,
  `SameTxidDifferentWitness`, `HardRejected`, `Reconsiderable`,
  `PostTrimAbsent`).
- `packages/open-bitcoin-rpc/src/method.rs` — `SupportedMethod`,
  `MethodOrigin::{BaselineParity, OpenBitcoinExtension}`.
- `packages/open-bitcoin-rpc/src/method/node.rs` — Existing
  `GetMempoolInfoResponse` with Phase 130 extension fields.
- `packages/open-bitcoin-rpc/src/dispatch/node.rs` — Node RPC dispatch,
  including truthful `getmempoolinfo`.
- `packages/open-bitcoin-node/src/status.rs` — `MempoolStatus` currently
  exposes `transactions` plus `relay` only.
- `packages/open-bitcoin-node/src/network/types.rs` — `ManagedMempoolInfo`
  already holds resource and fee values.
- `packages/open-bitcoin-cli/src/operator/dashboard/` — Ratatui dashboard
  model and row projection.
- `packages/open-bitcoin-cli/src/operator/status.rs` — Human/JSON status
  renderers over the shared snapshot.
- `packages/open-bitcoin-cli/src/operator/support.rs` — Redacted support
  bundle projection.

### Established Patterns

- BaselineParity methods keep Knots names and shapes; Open Bitcoin extras
  use extension methods or clearly named extra keys only where Phase 130
  already did so for identity-free mempool info.
- One shared snapshot feeds status, dashboard, metrics, logs, and support.
  Renderers must not invent surface-local summaries.
- Shared evidence is fixed-label and redacted. Identities stay on
  originating authenticated responses.
- Default verification stays hermetic: no public-network or wall-clock gates.

### Integration Points

- `packages/open-bitcoin-node/src/network/admission_bridge/package.rs` —
  Node-facing package dry-run/submit handoff into the authoritative core.
- `packages/open-bitcoin-node/src/network/lifecycle_projection/` —
  Lifecycle evidence, retry clears, and reconciliation counts.
- `packages/open-bitcoin-rpc/src/bin/open_bitcoind/` — Local RPC server
  that must dispatch the new methods.
- `packages/open-bitcoin-cli/` — `open-bitcoin-cli` baseline forwarding and
  `open-bitcoin` operator commands.
- `docs/parity/catalog/` and `docs/architecture/operator-observability.md` —
  Register the new fields without claiming public relay or readiness.

</code_context>

<specifics>
## Specific Ideas

- Match Knots `testmempoolaccept` / `submitpackage` closely enough that
  existing scripts keep working, while keeping the richer Phase 132 report
  on an explicit Open Bitcoin surface.
- `getmempoolinfo` extra keys (`rollingmempoolfee`, `effectiveadmissionfee`,
  `capacityenforcement`) are the precedent for identity-free RPC extras.
  Do not treat that precedent as permission to overload package result
  objects.
- Phase 105 already has `accepted_count`, `suppressed_count`, and
  `rebroadcast_deferred_count`. Add distinct retry counters rather than
  widening those meanings.

</specifics>

<deferred>
## Deferred Ideas

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

</deferred>

---

*Phase: 137-rpc-and-sanitized-operator-evidence*
*Context gathered: 2026-08-19*
