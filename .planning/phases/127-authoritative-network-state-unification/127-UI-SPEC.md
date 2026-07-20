---
phase: "127"
slug: "authoritative-network-state-unification"
status: approved
shadcn_initialized: false
preset: none
created: "2026-07-19"
---

# Phase 127 — UI Design Contract

> Phase 127 changes authoritative runtime provenance only. Existing RPC, CLI, Ratatui dashboard, metrics/log, and support-bundle presentation is frozen.

______________________________________________________________________

## Contract Scope

- No new screen, page, panel, widget, command, interaction, shortcut, layout, theme, visual state, or operator-facing field.
- Preserve existing serialized RPC schemas and existing human/JSON output.
- Preserve the current CLI and dashboard copy, ordering, availability states, terminal input behavior, and keyboard behavior.
- Preserve existing fixed low-cardinality metrics/log labels and support-bundle redaction.
- A change is in scope only when it replaces a non-authoritative data source with the Phase 127 shared runtime while leaving the rendered contract unchanged.

______________________________________________________________________

## Design System

| Property | Value |
| --- | --- |
| Tool | none — terminal/RPC contract only |
| Preset | not applicable |
| Component library | existing Ratatui/Crossterm components only |
| Icon library | none added |
| Font | terminal-owned; unchanged |

______________________________________________________________________

## Spacing Scale

Not applicable to Phase 127. Existing terminal layout and spacing remain byte-for-byte or snapshot-equivalent for identical status input.

Exceptions: none.

______________________________________________________________________

## Typography

Not applicable to Phase 127. Existing terminal emphasis, labels, headings, and text styles remain unchanged.

______________________________________________________________________

## Color

Not applicable to Phase 127. Existing terminal colors and availability/error styling remain unchanged; no new semantic color is introduced.

______________________________________________________________________

## Copywriting Contract

| Element | Contract |
| --- | --- |
| RPC fields | Preserve existing names, shapes, availability wrappers, and omission behavior. |
| CLI human output | Preserve existing labels, ordering, unavailable wording, and sanitization. |
| CLI JSON output | Preserve existing serialized schema and field semantics. |
| Dashboard | Preserve existing block-relay labels, panels, keyboard/input behavior, and unavailable states. |
| Metrics and logs | Preserve fixed low-cardinality labels; do not introduce peer-derived labels. |
| Support bundles | Preserve removal of raw endpoints, permission strings, credentials, transaction payloads, and dynamic peer identifiers. |
| Error state | Shared-authority failure renders through the existing unavailable/error vocabulary; it must not display stale data as authoritative. |

______________________________________________________________________

## Interaction And State Contract

- Operator polling must receive an owned authoritative snapshot and must not hold the network guard during serialization or rendering.
- A unavailable/poisoned authority must fail closed through existing error or unavailable states.
- The same authoritative snapshot must feed RPC, CLI/dashboard, metrics/log, and support consumers.
- Deterministic tests must compare existing output/schema behavior while varying only the underlying runtime provenance.

______________________________________________________________________

## Registry Safety

| Registry | Blocks Used | Safety Gate |
| --- | --- | --- |
| shadcn official | none | not applicable |
| third-party | none | no new registry or UI dependency permitted |

______________________________________________________________________

## Checker Sign-Off

- [x] Dimension 1 Copywriting: PASS
- [x] Dimension 2 Visuals: PASS
- [x] Dimension 3 Color: PASS
- [x] Dimension 4 Typography: PASS
- [x] Dimension 5 Spacing: PASS
- [x] Dimension 6 Registry Safety: PASS

**Approval:** approved 2026-07-19
