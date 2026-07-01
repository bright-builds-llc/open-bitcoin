---
generated_by: gsd-phase-researcher
lifecycle_mode: yolo
phase_lifecycle_id: 105-2026-07-01T20-32-29
generated_at: 2026-07-01T21:13:00Z
---

# Phase 105: Operator, RPC, Metrics, Logs, and Support Evidence - Research

**Researched:** 2026-07-01
**Domain:** shared relay/mempool evidence status, RPC operator projection,
low-cardinality telemetry, structured logs, support-bundle sanitization
**Confidence:** HIGH

<user_constraints>
## User Constraints From CONTEXT.md

### Locked Decisions

- Use one typed sanitized relay/mempool evidence projection in
  `OpenBitcoinStatusSnapshot`; do not duplicate classifications in each
  renderer.
- Keep baseline `sendrawtransaction`, `getmempoolinfo`, and `getnetworkinfo`
  response shapes compatibility-oriented. Put Open Bitcoin-specific evidence in
  `openbitcoinnetworkstatus` and shared operator status.
- Classify every exposed relay/mempool field as implemented, unavailable,
  deferred, or intentionally different.
- Preserve the Phase 104 distinction between local admission, queued relay
  evidence, transaction serving/request evidence, and public propagation. RPC
  success must not become a public-propagation claim.
- CLI, dashboard, metrics, logs, and support bundles must consume shared
  fixed-label/count evidence and must not expose raw transaction hex, txids,
  wtxids, peer ids, endpoints, raw permission strings, credential material, or
  dynamic metric labels.
- Support bundle output is local troubleshooting/parity-review evidence only,
  not a release validator, public-network proof, compact-block proof,
  production-service proof, production full-node readiness proof, or
  production-funds wallet safety proof.

### Deferred Ideas

- Periodic rebroadcast scheduling remains deferred; Phase 105 may expose
  `rebroadcast_deferred` only as bounded evidence.
- Sensitive debug mode, pseudonymized transaction aliases, and peer correlation
  aliases remain future design work.
- Compact block relay, package relay, bloom/filter serving, public relay
  defaults, public-network relay CI, production service operation, production
  full-node readiness, and production-funds wallet use remain out of scope.
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
| --- | --- | --- |
| OBS-01 | RPC surfaces such as `sendrawtransaction`, `getmempoolinfo`, `getnetworkinfo`, and Open Bitcoin network status report relay and mempool participation truthfully. | Extend `OpenBitcoinNetworkStatusResponse` and context/status projection with sanitized relay/mempool evidence while keeping baseline RPC structs narrow. |
| OBS-02 | CLI and dashboard surfaces render relay and mempool state from the shared status contract without raw transaction, peer, permission, or credential leakage. | Add relay/mempool fields to `OpenBitcoinStatusSnapshot`, then render compact rows in `status/render.rs`, dashboard model rows/charts, and support markdown from that shared snapshot. |
| OBS-03 | Metrics and structured logs use fixed low-cardinality relay outcomes for accepted, rejected, orphaned, requested, served, announced, suppressed, evicted, and expired events. | Add fixed `MetricKind` variants and sanitized relay structured-log helpers mirroring existing inbound metric/log patterns. |
| OBS-04 | Support bundles sanitize relay and mempool evidence, including raw transaction hex, disallowed txids/wtxids, peer endpoints, permission strings, dynamic labels, and credentials. | Extend `support_status_for_bundle` redaction and tests so support JSON and Markdown receive the same sanitized projection. |
</phase_requirements>

## Summary

Phase 105 should extend the existing shared observability contract, not add a
parallel telemetry subsystem. The repo already routes operator status through
`OpenBitcoinStatusSnapshot`, metrics through fixed `MetricKind` enum variants,
structured logs through fixed sources and sanitized key/value messages, and
support bundles through `support_status_for_bundle` before writing JSON and
Markdown. The Phase 104 relay work already produced safe internal evidence:
`LocalRelaySubmissionEvidence`, `ManagedRelayFanoutInfo`,
`ManagedRelayServingInfo`, `TxServeOutcomeLabel`, `TxFanoutAction`, and
`rebroadcast_deferred`.

The cleanest plan is to introduce a relay/mempool status projection in
`open-bitcoin-node` that maps existing Phase 104 evidence into fixed counts,
outcome labels, and classification states. RPC should expose that through
`openbitcoinnetworkstatus`, operator status should merge it into the shared
snapshot, CLI/dashboard/support should render from that snapshot, and metrics
and logs should derive fixed outcome counters/records from the same vocabulary.

The main planning risks are schema drift and leakage. Schema drift happens if
RPC, CLI, dashboard, logs, metrics, and support each translate Phase 104
evidence independently. Leakage happens if raw transaction identifiers or peer
material enter the shared snapshot before support redaction. Both risks are best
handled by a typed allowlist projection whose public fields are only labels,
counts, availability states, and bounded reason strings.

## Existing Code Map

### Shared Status And Runtime Evidence

- `packages/open-bitcoin-node/src/status.rs` owns `FieldAvailability<T>`,
  `MempoolStatus`, and `OpenBitcoinStatusSnapshot`. Add relay/mempool evidence
  types here or in a child status module re-exported from this file.
- `packages/open-bitcoin-node/src/network/relay_fanout.rs` exposes
  `ManagedRelayFanoutInfo`, `ManagedRelayFanoutActionInfo`,
  `LocalRelaySubmissionEvidence`, `LocalRelaySubmissionLabel`, and
  `RebroadcastEvidenceLabel`.
- `packages/open-bitcoin-node/src/network/relay_serving.rs` exposes
  `ManagedRelayServingInfo` and low-cardinality latest serving outcomes.
- `packages/open-bitcoin-node/src/network.rs` has `mempool_info()`,
  `network_info()`, `relay_fanout_info()`, `relay_serving_info()`, and
  `latest_local_submission_evidence()` accessors or adjacent integration points.
- `packages/open-bitcoin-rpc/src/context/network.rs` exposes RPC context methods
  for mempool info, network info, metrics, and latest local submission evidence;
  add relay status accessors here so dispatch does not inspect internals.

### RPC Surfaces

- `packages/open-bitcoin-rpc/src/method/node.rs` defines baseline-shaped
  `GetMempoolInfoResponse`, `GetNetworkInfoResponse`,
  `SendRawTransactionResponse`, and Open Bitcoin-specific
  `OpenBitcoinNetworkStatusResponse`.
- `packages/open-bitcoin-rpc/src/dispatch/node.rs` builds `getmempoolinfo`,
  `getnetworkinfo`, `sendrawtransaction`, and `openbitcoinnetworkstatus`.
  Preserve `sendrawtransaction` success fields and add separate Open Bitcoin
  relay status evidence.
- `packages/open-bitcoin-rpc/src/dispatch/tests.rs` already has tests proving
  `getnetworkinfo` omits Open Bitcoin inbound details, `getmempoolinfo` returns
  implemented mempool fields, and `sendrawtransaction` records local relay
  evidence without propagation fields.

### CLI, Dashboard, And Support Surfaces

- `packages/open-bitcoin-cli/src/operator/status.rs` collects live RPC status
  from `getnetworkinfo`, `getmempoolinfo`, and `openbitcoinnetworkstatus`, then
  produces `OpenBitcoinStatusSnapshot`. Add older-daemon fallback reasons here.
- `packages/open-bitcoin-cli/src/operator/status/render.rs` renders human and
  JSON status from the shared snapshot. Add compact relay/mempool lines here,
  not separate probes.
- `packages/open-bitcoin-cli/src/operator/dashboard/model.rs` projects dashboard
  rows from the shared snapshot; add relay/mempool rows to the existing
  "Mempool and Wallet" or a compact adjacent section.
- `packages/open-bitcoin-cli/src/operator/dashboard/model/metrics.rs` chooses
  fixed dashboard metric kinds. Add relay metrics only through fixed enum
  variants and bounded candidates.
- `packages/open-bitcoin-cli/src/operator/support.rs` calls
  `support_status_for_bundle(status)` before writing support JSON/Markdown.
- `packages/open-bitcoin-cli/src/operator/support/redaction.rs` centralizes
  support sanitizer behavior. Extend it with relay/mempool redaction helpers
  that reject raw tx hex, txid/wtxid-like strings, endpoints, permission strings,
  credentials, and dynamic-label shapes.
- `packages/open-bitcoin-cli/src/operator/support/render.rs` writes support
  Markdown. Add a compact section that renders sanitized relay/mempool evidence
  and no-claim next-action guidance.

### Metrics And Structured Logs

- `packages/open-bitcoin-node/src/metrics.rs` owns `MetricKind::ALL`, string
  names, retention, samples, and inbound metric projection. Add fixed relay
  outcome counter variants and a projection helper.
- `packages/open-bitcoin-node/src/metrics/tests.rs` asserts metric names and
  low-cardinality properties. Extend these tests with relay metric names and
  forbidden dynamic-label substrings.
- `packages/open-bitcoin-node/src/logging.rs` owns `StructuredLogRecord`,
  fixed log sources, and sanitizer helpers for inbound resource governance and
  peer policy. Add relay outcome log helpers with fixed source names and
  allowlisted message fields.
- `packages/open-bitcoin-rpc/src/context/resource_governance.rs` and
  `packages/open-bitcoin-rpc/src/context/peer_policy.rs` show the managed RPC
  context pattern for appending structured log records and saturating write
  failure counters.

## Recommended Plan Shape

### Plan 105-01: Shared Relay/Mempool Status And RPC Projection

Implement the typed status contract first. This plan should:

- Add status structs/enums for relay/mempool evidence classification and
  outcome counters.
- Map `ManagedRelayFanoutInfo`, `ManagedRelayServingInfo`,
  `LocalRelaySubmissionEvidence`, `ManagedMempoolInfo`, and `ManagedNetworkInfo`
  into sanitized status.
- Extend `OpenBitcoinNetworkStatusResponse` and `open_bitcoin_network_status()`
  to include the new status.
- Keep `getnetworkinfo` free of Open Bitcoin-specific relay details and keep
  `sendrawtransaction` response shape unchanged.
- Add RPC dispatch tests for availability/deferred/intentionally-different
  classifications and no raw relay details in baseline methods.

### Plan 105-02: CLI, Dashboard, Metrics, And Structured Logs

Once the shared contract exists, wire operator projections and telemetry:

- Extend live status collection and stopped/unavailable fallbacks.
- Add human status and dashboard rows derived only from
  `OpenBitcoinStatusSnapshot`.
- Add fixed `MetricKind` relay counters and tests.
- Add sanitized relay structured-log helpers and tests. If runtime append hooks
  are added in this plan, follow the existing managed context log append pattern
  and keep records fixed-label only.

### Plan 105-03: Support Redaction, Docs, Checker, And Closeout Evidence

Finish with support and parity guardrails:

- Extend support bundle sanitizer and tests for raw tx hex, txid/wtxid-like
  strings, endpoints, permission strings, credentials, and dynamic metric-label
  shapes.
- Add support Markdown relay/mempool section and no-claim next-action guidance.
- Update parity docs/index/checklist for the Phase 105 OBS-* surface.
- Add a deterministic Phase 105 checker if docs/parity/verifier wiring changes,
  and wire it after Phase 104 in `scripts/verify.sh`.
- Add `105-VERIFICATION.md` only after implementation and verification pass.

## Validation Architecture

### Required Automated Checks

- RPC dispatch tests:
  - `openbitcoinnetworkstatus` exposes sanitized relay/mempool status.
  - `getnetworkinfo` does not expose Open Bitcoin-specific relay status.
  - `sendrawtransaction` keeps success shape and separates propagation evidence.
  - `getmempoolinfo` remains truthful for implemented and unavailable/deferred
    mempool fields.
- CLI status tests:
  - Human output includes compact relay/mempool state lines.
  - JSON status includes availability/classification fields.
  - Older-daemon fallback uses unavailable/deferred reasons.
  - Output does not contain raw transaction hex, disallowed txid/wtxid, endpoint,
    permission string, or credential probes.
- Dashboard tests:
  - Dashboard rows derive relay/mempool state from the shared snapshot.
  - Relay charts, if added, use fixed `MetricKind` variants.
- Metrics tests:
  - `MetricKind::ALL` includes fixed relay outcome counters.
  - Relay metric names end in `_count` and contain no dynamic-label substrings.
  - Relay metric projection maps accepted, rejected, orphaned, requested,
    served, announced, suppressed, evicted, and expired.
- Logging tests:
  - Relay structured log records use fixed source names.
  - Suspicious raw fields are redacted.
  - No raw hex, txid/wtxid-like material, endpoints, peer ids, permissions,
    credentials, cookies, or secrets survive sanitized messages.
- Support tests:
  - `support_status_for_bundle` redacts relay/mempool sensitive material before
    JSON and Markdown write.
  - Support Markdown contains bounded relay/mempool summary and no-claim
    guidance.
  - Fixtures reject raw transaction hex, disallowed txids/wtxids, endpoints,
    permission strings, credentials, and dynamic metric labels.
- Phase checker tests if docs/parity/verifier changes:
  - Required OBS-01 through OBS-04 evidence roots are present.
  - Phase 105 checker appears after Phase 104 in visible and executable
    `scripts/verify.sh` order.
  - Forbidden positive claims fail fixture tests.

### Required Closeout Command

```bash
bash scripts/verify.sh
```

Before committing implementation work in this Rust project, also satisfy the
repo's Rust pre-commit expectations:

```bash
cargo fmt --all
cargo clippy --all-targets --all-features -- -D warnings
cargo build --all-targets --all-features
cargo test --all-features
```

The repo-native `bash scripts/verify.sh` remains the final contract and includes
the Bazel smoke build.

## Security And Privacy Notes

- Treat raw transaction hex, txids, wtxids, peer ids, peer endpoints,
  permission strings, credentials, cookies, secrets, and dynamic metric labels
  as sensitive in operator/support surfaces by default.
- If exact txids/wtxids remain necessary in baseline RPC responses, keep them
  confined to those baseline response fields and do not copy them into shared
  operator evidence or support artifacts.
- Prefer allowlisted fixed labels over denylist-only sanitization.
- Keep support evidence local and bounded; do not add automatic upload,
  public-network proof, or release-validator semantics.

## Dependency Guidance

No new dependencies are recommended. Existing first-party crates and
serde/serde_json support the needed status, RPC, metrics, log, and support
projections. Adding an external telemetry or Bitcoin library would conflict with
the project's minimal dependency and first-party domain-model constraints.

## Open Planning Questions

- Exact type names for the relay/mempool status structs and whether to place
  them in `status.rs` or `status/relay.rs`.
- Whether runtime structured relay log append hooks belong in Plan 105-02 or
  should be limited to pure log record helpers plus tests if no stable event
  call site exists.
- Whether dashboard should show relay evidence as rows only or replace optional
  charts with fixed relay metric candidates when samples exist.
- Exact no-claim checker corpus if Phase 105 updates docs beyond parity roots.

## Recommended Planner Constraints

- Every plan should explicitly include OBS requirement IDs in frontmatter.
- Do not create a broad all-doc scanner; use fixed-file deterministic checkers
  if guardrails are needed.
- Keep default verification deterministic and local.
- Add parity breadcrumbs for new first-party Rust source or test files under
  `packages/open-bitcoin-*/src` or `packages/open-bitcoin-*/tests`.
- Use repo-local Cargo and Bazel command forms in any UAT text.
