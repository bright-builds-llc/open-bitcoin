# Phase 105: Operator, RPC, Metrics, Logs, and Support Evidence - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md; this log preserves the alternatives considered.

**Date:** 2026-07-01T20:32:29Z
**Phase:** 105-operator-rpc-metrics-logs-and-support-evidence
**Mode:** Yolo
**Areas discussed:** Shared relay/mempool RPC status contract, CLI/dashboard relay/mempool rendering, low-cardinality telemetry, support bundle sanitization and evidence boundaries

## Shared Relay/Mempool RPC Status Contract

| Option | Description | Selected |
| --- | --- | --- |
| Typed status-matrix extension on `OpenBitcoinStatusSnapshot` and `openbitcoinnetworkstatus` | One shared truth contract, preserves baseline RPC shapes, and supports CLI/dashboard/support/metrics/log projections. | yes |
| Open Bitcoin RPC sidecar only | Keeps baseline RPC methods untouched but risks drift from status snapshot and operator surfaces. | no |
| Per-RPC adapter annotations | Minimal modeling but scatters classifications across RPC adapters. | no |
| Docs/parity matrix plus regression checks | Low runtime impact but does not provide live evidence for operator surfaces. | no |

**User's choice:** Auto-selected recommended option in yolo mode.
**Notes:** Baseline `sendrawtransaction`, `getmempoolinfo`, and `getnetworkinfo` should remain compatibility-oriented. Open Bitcoin-specific relay/mempool truth belongs in `openbitcoinnetworkstatus` and the shared snapshot.

## CLI/Dashboard Relay/Mempool Rendering

| Option | Description | Selected |
| --- | --- | --- |
| Shared sanitized presentation projection | Single contract for CLI, dashboard, support, and RPC extension; prevents leaks before rendering. | yes |
| Renderer-level allowlists only | Smaller change but JSON status could still leak and renderers can drift. | no |
| Reuse support-bundle redaction for CLI/dashboard | Fast reuse but support redaction is bundle-specific and does not model relay truth. | no |
| Explicit sensitive-debug mode layered on shared projection | Useful future extension, but not the default baseline. | no |

**User's choice:** Auto-selected recommended option in yolo mode.
**Notes:** CLI and dashboard should render fixed labels/counts from shared status and avoid raw transaction, peer, permission, or credential material.

## Low-Cardinality Telemetry

| Option | Description | Selected |
| --- | --- | --- |
| Status-first fixed outcome aggregate | One shared truth source with fixed `MetricKind` counters and support projection. | yes |
| Metrics/logs-only fixed counters and sanitized log records | Small runtime surface but weaker RPC/status/support consistency. | no |
| Dedicated `relay_mempool_telemetry` projection module | Centralized allowlist but possibly heavier than needed. | no |
| Structured-log event ledger as source of truth | Strong audit trail but makes effectful logs the truth source and depends on retention. | no |

**User's choice:** Auto-selected recommended option in yolo mode.
**Notes:** Add fixed outcome counters and sanitized structured logs derived from the same status evidence. Avoid dynamic labels and raw identifiers.

## Support Bundle Sanitization And Evidence Boundaries

| Option | Description | Selected |
| --- | --- | --- |
| Shared typed allowlist evidence contract | One truth source for all surfaces and prevents raw transaction/peer material from entering artifacts. | yes |
| Support-bundle-only sanitizer | Smallest support change but leaves earlier surfaces at risk. | no |
| Pseudonymized correlation aliases | Future diagnostic value but adds re-identification and retention policy risk. | no |
| Docs/checker claim guardrails only | Protects prose but cannot sanitize runtime evidence. | no |

**User's choice:** Auto-selected recommended option in yolo mode.
**Notes:** Extend the existing support-bundle redaction path with relay/mempool-specific sanitizer coverage and no-claim language.

## the agent's Discretion

- Exact type names, module split, metric-kind names, and renderer row labels.
- Whether the sanitized projection lives directly in `status.rs` or a child status module.
- Exact deterministic checker shape if Phase 105 adds docs/parity/verifier wiring.
- Exact test slicing, as long as RPC dispatch, CLI renderer, dashboard/status, metrics/log, support redaction, and `bash scripts/verify.sh` coverage are represented.

## Deferred Ideas

- Timer-driven periodic rebroadcast scheduling.
- Sensitive debug mode or pseudonymized transaction/peer correlation aliases.
- Compact block relay, package relay, bloom/filter serving, public relay defaults, public-network relay CI, production service operation, production full-node readiness, and production-funds wallet use.
