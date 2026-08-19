# Phase 137: RPC and Sanitized Operator Evidence - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-08-19
**Phase:** 137-rpc-and-sanitized-operator-evidence
**Mode:** Yolo
**Areas discussed:** Package RPC/CLI contract, Shared evidence redaction, Operator field vocabulary and dashboard, Local-admission versus relay-suppressed truth

---

## Package RPC/CLI contract

| Option | Description | Selected |
|--------|-------------|----------|
| Knots-named methods only | `testmempoolaccept` + `submitpackage` as BaselineParity with exact Knots JSON | |
| Open Bitcoin extension methods only | Typed `PackageReport` on extension RPC / `open-bitcoin package` | |
| Dual origin: exact Knots methods + separate extension | Knots shapes stay unextended; typed fields live on an extension/operator path | ✓ |
| Augmented Knots JSON | Extra Open Bitcoin keys on the baseline package methods | |

**User's choice:** Dual origin: exact Knots methods + separate extension (recommended default)
**Notes:** [auto] Yolo selected the advisor recommendation so Knots invocation stays compatible and Phase 132 typed fields do not overload Knots meanings.

---

## Shared evidence redaction

| Option | Description | Selected |
|--------|-------------|----------|
| Originating authenticated RPC/CLI only | Identifiers and per-member results stay on the call that supplied them | ✓ |
| Authenticated JSON-RPC including `openbitcoinnetworkstatus` | Status RPC may echo last-package members | |
| Privileged local status/dashboard plus originating RPC | Local TUI may show member rows | |

**User's choice:** Originating authenticated RPC/CLI only (recommended default)
**Notes:** [auto] Status RPC is authenticated but shares the snapshot that support bundles embed, so IDs there would leak into shareable diagnostics.

---

## Operator field vocabulary and dashboard

| Option | Description | Selected |
|--------|-------------|----------|
| Extend `OpenBitcoinStatusSnapshot.mempool` with distinct typed groups | One shared model for status, dashboard, metrics, logs, and support | ✓ |
| Reuse Knots RPC names as dashboard/status labels | `bytes` / `usage` / `mempoolminfee` as operator labels | |
| Parallel operator-evidence DTO | Second schema beside the snapshot | |
| Overload existing relay counters | Fold retry/pressure into `evicted_count` / `rebroadcast_deferred_count` | |

**User's choice:** Extend the shared snapshot with distinct typed groups (recommended default)
**Notes:** [auto] Preserves Phase 130 D-01 (no overloaded numeric) and the existing snapshot contract. Dashboard stays Ratatui and consumes the same groups.

---

## Local-admission versus relay-suppressed truth

| Option | Description | Selected |
|--------|-------------|----------|
| Explicit dual-state | Admission axis independent from relay/fanout axis | ✓ |
| Knots-shaped success plus prose warning | txid / `package_msg=success` plus documentation only | |
| Status-only dual evidence, Knots submit shape | Dual-state only on shared surfaces | |

**User's choice:** Explicit dual-state using existing lifecycle vocabulary (recommended default)
**Notes:** [auto] Reconciled with D-01/D-02: Knots BaselineParity JSON stays exact; dual-state lives on the Open Bitcoin extension/operator path and on shared aggregate evidence. No `propagated` / `broadcast` / `public_relay` fields.

---

## Claude's Discretion

- Exact extension method and clap command names
- Exact nested snapshot field names and dashboard row titles
- One extension method with a mode argument versus two methods
- MetricKind and structured-log key spelling

## Deferred Ideas

- Phase 138 parity, adversarial pressure, benchmarks, and claim guardrails
- `getrawmempool` / `getmempoolentry` unless a later phase owns them
- Hosted or web dashboards
- General package wire, public/default relay, guaranteed propagation, production readiness
