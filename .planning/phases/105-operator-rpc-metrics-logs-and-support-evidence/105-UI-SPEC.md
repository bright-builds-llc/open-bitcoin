---
phase: 105
slug: operator-rpc-metrics-logs-and-support-evidence
status: approved
shadcn_initialized: false
preset: none
generated_by: gsd-ui-researcher
lifecycle_mode: yolo
phase_lifecycle_id: 105-2026-07-01T20-32-29
created: 2026-07-01
generated_at: 2026-07-01T21:17:00Z
---

# Phase 105 - UI Design Contract

> Visual and interaction contract for operator-facing terminal, JSON, dashboard,
> and support Markdown surfaces. This is not a web UI contract.

## Surface Scope

| Surface | Contract |
| --- | --- |
| CLI human status | Add compact relay/mempool evidence lines to the existing status output without reordering unrelated sync, wallet, service, log, metric, or health lines. |
| CLI JSON status | Preserve `OpenBitcoinStatusSnapshot` as the JSON shape; add typed relay/mempool fields with explicit availability/classification states. |
| Terminal dashboard model | Add relay/mempool rows and optional fixed relay metric charts only from the shared snapshot. |
| Support Markdown | Add a bounded relay/mempool section with sanitized labels, counts, unavailable/deferred reasons, and no-claim next-action guidance. |
| Baseline RPC output | Keep baseline method output compatibility-oriented; do not make baseline RPC responses carry UI presentation text. |

## Design System

| Property | Value |
| --- | --- |
| Tool | none |
| Preset | not applicable |
| Component library | none |
| Icon library | none |
| Font | terminal default / Markdown default |

The repo's relevant UI is textual and operator-focused. Do not introduce
shadcn, Radix, browser components, images, cards, hero sections, decorative
gradients, or icon dependencies for Phase 105.

## Layout And Density

| Area | Rule |
| --- | --- |
| CLI human status | One line per relay/mempool concept using the existing `Label: value` style. |
| Dashboard rows | Keep row values single-line and scan-friendly. Prefer `label=count` pairs over prose paragraphs. |
| Dashboard charts | Use only fixed `MetricKind` candidates. Do not add per-peer or per-transaction charts. |
| Support Markdown | Use a dedicated `## Relay and Mempool Evidence` section with short bullet rows. |
| JSON | Use snake_case field names and existing `FieldAvailability` tagged format. |

## Spacing Scale

Declared values are textual equivalents, not CSS tokens:

| Token | Value | Usage |
| --- | --- | --- |
| xs | single space | Inline `key=value` separators and compact labels. |
| sm | one newline | Separate status lines and Markdown bullets. |
| md | one blank line | Separate Markdown sections. |
| lg | not applicable | No page-level spacing in terminal output. |

Exceptions: none.

## Typography

| Role | Size | Weight | Line Height |
| --- | --- | --- | --- |
| Body | terminal default | normal | default |
| Label | terminal default | normal | default |
| Heading | Markdown `##` only in support docs | normal renderer default | default |
| Display | not applicable | not applicable | not applicable |

Do not add ANSI color requirements for relay/mempool evidence. Existing no-color
paths and plain-text output must remain readable.

## Color

| Role | Value | Usage |
| --- | --- | --- |
| Dominant | terminal default | All CLI/dashboard text. |
| Secondary | terminal default | Section labels and row labels. |
| Accent | none | Not used for relay/mempool evidence. |
| Destructive | none | Not applicable to read-only evidence. |

Accent reserved for: none. Phase 105 adds read-only evidence; it must not create
new destructive or interactive operator controls.

## Copywriting Contract

| Element | Copy |
| --- | --- |
| Relay summary label | `Relay evidence` |
| Mempool summary label | `Mempool` |
| Rebroadcast boundary label | `Rebroadcast: deferred` |
| Unavailable state | `Unavailable: {reason}` |
| Deferred state | `Deferred: {reason}` |
| Intentionally different state | `Intentionally different: {reason}` |
| Support next action | `Next action: Treat relay/mempool evidence as bounded local status only; do not treat it as public propagation, compact-block relay, or production-readiness proof.` |

Avoid words that imply public propagation guarantees: `broadcasted to network`,
`public relay ready`, `production relay`, `compact block relay`, `full relay
parity`, and `production node ready`.

## Data Presentation Contract

| Evidence Type | Presentation |
| --- | --- |
| Outcome counts | Use fixed `accepted_count`, `rejected_count`, `orphaned_count`, `requested_count`, `served_count`, `announced_count`, `suppressed_count`, `evicted_count`, `expired_count`, and optional `rebroadcast_deferred_count`. |
| Latest labels | Use allowlisted labels only, for example `accepted`, `rejected`, `orphaned`, `served`, `announced`, `suppressed`, `evicted`, `expired`, `rebroadcast_deferred`. |
| Availability | Use `available`, `unavailable`, `deferred`, or `intentionally_different` semantics. If implemented as `FieldAvailability` plus status fields, renderer copy must still show the state clearly. |
| Sensitive material | Never render raw transaction hex, disallowed txids/wtxids, peer ids, peer endpoints, permission strings, credentials, cookies, secrets, or dynamic metric labels. |
| Baseline RPC txids | Existing `sendrawtransaction` response txid fields may remain in the RPC response, but do not copy them into default operator status/dashboard/support evidence. |

## Registry Safety

| Registry | Blocks Used | Safety Gate |
| --- | --- | --- |
| shadcn official | none | not applicable |
| third-party registries | none | not applicable |

## Checker Sign-Off

- [x] Dimension 1 Copywriting: PASS
- [x] Dimension 2 Visuals: PASS
- [x] Dimension 3 Color: PASS
- [x] Dimension 4 Typography: PASS
- [x] Dimension 5 Spacing: PASS
- [x] Dimension 6 Registry Safety: PASS

**Approval:** approved 2026-07-01
