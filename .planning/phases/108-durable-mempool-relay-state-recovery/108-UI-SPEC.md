---
phase: 108
slug: durable-mempool-relay-state-recovery
status: approved
shadcn_initialized: false
preset: not applicable
generated_by: gsd-ui-researcher
lifecycle_mode: yolo
phase_lifecycle_id: 108-2026-07-03T14-09-06
created: 2026-07-03
generated_at: 2026-07-03T14:31:06Z
reviewed_at: 2026-07-03T14:33:29Z
---

# Phase 108 - UI Design Contract

> Visual and interaction contract for operator-facing terminal, Ratatui dashboard,
> JSON status, support Markdown, and sanitized recovery evidence surfaces. This is
> not a web frontend contract.

## Source Decisions

| Source | Decisions Used |
| --- | --- |
| `108-CONTEXT.md` | Recovery replay must reuse existing relay serving/fanout/status contracts; no socket I/O, public propagation guarantees, raw transaction or peer material, destructive repair, or public relay default claims. |
| `108-RESEARCH.md` | Recovery UI should be fixed aggregate evidence over managed recovery replay, lifecycle cleanup, operator status, support redaction, and deterministic no-claim guardrails. |
| `REQUIREMENTS.md` | Phase owns MEM-04, MEM-05, MEM-06, REL-01, and REL-02, with operator evidence kept truthful and sanitized through existing OBS contracts. |
| Existing code | Preserve `Relay evidence`, `Mempool evidence`, `Relay fanout`, `Relay serving`, `Rebroadcast: deferred`, `Public relay`, `redacted_relay_mempool_evidence`, and existing `RelayEvidenceField` states. |

## Surface Scope

| Surface | Contract |
| --- | --- |
| CLI human status | Render recovery evidence as compact `Label: value` lines near existing relay/mempool evidence. Do not reorder unrelated sync, wallet, service, log, metric, or health lines. |
| CLI JSON status | Keep recovery evidence under the shared status shape, either in `OpenBitcoinStatusSnapshot.mempool.relay` or the matching `openbitcoinnetworkstatus.relay` projection. Use snake_case fixed fields only. |
| Ratatui dashboard rows | Add at most one recovery-specific row if existing relay rows cannot carry the evidence clearly. Values must remain single-line and scan-friendly. |
| Support Markdown | Keep a bounded `## Relay and Mempool Evidence` section with short bullets, sanitized recovery labels/counts, and no-claim next-action guidance. |
| Structured logs and metrics | Project only fixed aggregate recovery outcomes. No dynamic labels, free-form reasons, peer dimensions, or transaction identifiers. |
| Baseline RPC output | Keep baseline-compatible RPC response shapes presentation-free. Recovery-specific copy belongs in Open Bitcoin status/support surfaces only. |

## Design System

| Property | Value |
| --- | --- |
| Tool | not applicable |
| Preset | not applicable |
| Component library | not applicable |
| Icon library | not applicable |
| Font | terminal default / Markdown renderer default |
| Registry | not applicable |

Do not introduce shadcn, Radix, browser components, icons, images, cards, hero
sections, decorative gradients, ANSI color requirements, or web layout assets
for Phase 108.

## Layout And Density

| Area | Rule |
| --- | --- |
| CLI human status | One logical line per relay/mempool/recovery concept using the existing `Label: value` style. Prefer `key=value` pairs over prose. |
| Existing relay counter line | Preserve the current counter order: `accepted_count`, `rejected_count`, `orphaned_count`, `requested_count`, `served_count`, `announced_count`, `suppressed_count`, `evicted_count`, `expired_count`, `rebroadcast_deferred_count`. |
| Recovery counter line | If recovery counters are added, use a separate `Relay recovery: ...` line instead of widening `Relay evidence` with recovery-only fields. |
| Dashboard rows | Preserve the existing row-label style. Do not add multi-line cells, nested panels, cards, decorative separators, or marketing copy. |
| Support Markdown | Use one heading and bullet rows. No tables for volatile recovery evidence, no raw logs, and no pasted structured-log bodies. |
| JSON | Use structured status fields, not human prose. No arrays of recovered transactions or peer-specific recovery objects. |

## Spacing Scale

Declared values are terminal text equivalents, not CSS tokens:

| Token | Value | Usage |
| --- | --- | --- |
| xs | single ASCII space | Inline `key=value` separators and compact label text. |
| sm | one newline | Separate status lines and Markdown bullets. |
| md | one blank line | Separate Markdown sections. |
| lg | not applicable | No page-level spacing in terminal/operator output. |

Exceptions: none. Do not add decorative cards, framed boxes, marketing sections,
or hero text.

## Typography

| Role | Size | Weight | Line Height |
| --- | --- | --- | --- |
| Body | terminal default | normal | default |
| Label | terminal default | normal | default |
| Heading | Markdown `##` only in support docs | normal renderer default | default |
| Display | not applicable | not applicable | not applicable |

Do not rely on ANSI color, bold, italics, icons, or terminal theme support to
communicate recovery state. Plain text must be complete.

## Color

| Role | Value | Usage |
| --- | --- | --- |
| Dominant | terminal default | All CLI/dashboard text. |
| Secondary | terminal default | Section labels and row labels. |
| Accent | none | Not used for recovery evidence. |
| Destructive | none | No destructive actions in Phase 108 UI surfaces. |

Accent reserved for: none. Phase 108 adds read-only evidence and must not create
new destructive or interactive operator controls.

## Copywriting Contract

| Element | Copy |
| --- | --- |
| Primary CTA | Not applicable. No new interactive CTA or repair command. |
| Relay summary label | `Relay evidence` |
| Recovery summary label | `Relay recovery` |
| Mempool summary label | `Mempool` |
| Mempool evidence label | `Mempool evidence` |
| Relay fanout label | `Relay fanout` |
| Relay serving label | `Relay serving` |
| Rebroadcast boundary label | `Rebroadcast: deferred` |
| Public relay boundary label | `Public relay` |
| Implemented state | `Implemented: {capability}` |
| Unavailable state | `Unavailable: {reason}` |
| Deferred state | `Deferred: {reason}` |
| Intentionally different state | `Intentionally different: {reason}` |
| Empty recovery state | `Relay recovery: recovered_count=0 dropped_confirmed_count=0 dropped_duplicate_count=0 dropped_missing_parent_count=0 dropped_policy_incompatible_count=0 dropped_evicted_count=0` |
| Sanitized error state | `Relay recovery: Unavailable: redacted_relay_mempool_evidence` |
| Non-sensitive error state | `Relay recovery: Unavailable: recovery evidence unavailable` |
| Support next action | `Next action: Treat recovered relay/mempool evidence as bounded local status and local troubleshooting/parity-review evidence only; do not treat it as public propagation, compact-block relay, production-readiness proof, a release validator, public-network proof, production-service proof, production full-node readiness proof, production-funds wallet safety proof, or authorization for destructive repair.` |
| Destructive confirmation | Not applicable. Destructive repair, source datadir mutation, store surgery, automatic support upload, and compaction are out of scope. |

Avoid copy that implies public relay readiness or guaranteed propagation:
`broadcasted to network`, `public relay ready`, `production relay`, `compact
block relay`, `package relay`, `full relay parity`, `production node ready`,
`repair completed`, and `safe for production funds`.

## Data Presentation Contract

| Evidence Type | Presentation |
| --- | --- |
| Existing outcome counts | Preserve the fixed relay counter vocabulary and order already used by status, dashboard, metrics, logs, and support Markdown. |
| Recovery counts | If a new aggregate is needed, use fixed fields only: `recovered_count`, `dropped_confirmed_count`, `dropped_duplicate_count`, `dropped_missing_parent_count`, `dropped_policy_incompatible_count`, `dropped_evicted_count`. |
| Recovery labels | Use only fixed low-cardinality labels: `recovered`, `dropped_confirmed`, `dropped_duplicate`, `dropped_missing_parent`, `dropped_policy_incompatible`, `dropped_evicted`, `served`, `suppressed`, `announced`, `evicted`, `expired`. |
| Capability states | Use existing `implemented`, `unavailable`, `deferred`, and `intentionally_different` semantics. Do not add a recovery-only state enum. |
| JSON naming | Use snake_case field names and tagged status objects consistent with `RelayEvidenceField`. |
| Support Markdown | Preserve safe counts and bounded labels; replace sensitive recovery reasons with `redacted_relay_mempool_evidence`. |
| Baseline RPC txids | Existing baseline RPC txid responses may remain where protocol-compatible, but do not copy txids or wtxids into default status, dashboard, support, metrics, or logs. |

## Redaction And Safety

Never render raw transaction hex, txids, wtxids, peer ids, peer endpoints,
socket-address shapes, permission strings, raw class names, credentials, cookies,
secrets, dynamic metric labels, raw structured-log bodies, free-form rejection
text, or raw storage/corruption payload material.

Sensitive recovery reason text must sanitize to:

```text
redacted_relay_mempool_evidence
```

Corrupt, stale, incompatible, or unrecoverable durable records must surface as
typed aggregate diagnosis only. They must not imply source datadir mutation,
manual store surgery, destructive repair, public-network proof, or production
readiness.

## Registry Safety

| Registry | Blocks Used | Safety Gate |
| --- | --- | --- |
| shadcn official | none | not applicable |
| third-party registries | none | not applicable |
| terminal/Ratatui widgets | existing project code only | no external registry |

## Checker Sign-Off

| Dimension | PASS-ready criteria |
| --- | --- |
| Dimension 1 Copywriting | PASS when all labels and next-action copy match this contract, preserve no-claim boundaries, and avoid public propagation or production-readiness promises. |
| Dimension 2 Visuals | PASS when terminal/dashboard/support surfaces are compact plain text with no decorative cards, hero text, marketing copy, icons, images, raw logs, or multi-line dashboard cells. |
| Dimension 3 Color | PASS when recovery evidence remains readable without ANSI color and adds no accent/destructive color semantics. |
| Dimension 4 Typography | PASS when output relies only on terminal defaults and Markdown `##` support headings, with no style-dependent meaning. |
| Dimension 5 Spacing | PASS when CLI lines, dashboard rows, and Markdown bullets use the declared newline/blank-line structure and keep evidence scan-friendly. |
| Dimension 6 Registry Safety | PASS when shadcn, component-library, icon-library, and third-party registry usage remain not applicable. |

**Approval:** approved 2026-07-03

## UI-SPEC COMPLETE
