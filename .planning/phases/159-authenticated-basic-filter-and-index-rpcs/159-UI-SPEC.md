---
phase: "159"
slug: "authenticated-basic-filter-and-index-rpcs"
status: approved
reviewed_at: "2026-10-09T16:18:19Z"
shadcn_initialized: false
preset: none
created: "2026-10-09"
generated_by: gsd-ui-researcher
lifecycle_mode: yolo
phase_lifecycle_id: 159-2026-10-09T16-11-49
generated_at: 2026-10-09T16:16:31Z
---

# Phase 159 — UI Design Contract

## Applicability and Sources

This phase exposes authenticated JSON-RPC methods on the existing headless Rust daemon. It has no visual frontend scope. The roadmap states that UI hints also match protocol-header terminology and that only Phase 161 changes the terminal dashboard. Phase 159 CONTEXT.md explicitly directs the UI gate to record this applicability decision without introducing a GUI or dashboard.

The contract below governs machine-readable interactions for CFRP-01 and CFRP-02. Visual dimensions are not applicable; adding arbitrary spacing, fonts or colors would contradict the established scope. Existing command-line clients remain consumers of the same RPC results; no new interactive CLI product is specified.

Material sources: `159-CONTEXT.md` decisions D-01–D-13 and Agent Discretion; `.planning/ROADMAP.md` Phase 159 success criteria and UI-hint explanation; `.planning/REQUIREMENTS.md` CFRP-01/02 and CFOP-01 ownership. Repository guidance, Bright Builds sidecar, placeholder-only overrides, frontend standards and both active lesson files informed applicability. The template is `~/.codex/get-shit-done/templates/UI-SPEC.md`.

## Design System

| Property | Value |
| --- | --- |
| Tool | none |
| Preset | Not applicable: headless JSON-RPC |
| Component library | none |
| Icon library | Not applicable |
| Font | Not applicable: terminal/client rendering is outside this phase |

Codebase scouting found no first-party `components.json`, web package manifest, Tailwind/PostCSS configuration or TSX/CSS frontend. The existing CLI uses clap and the existing dashboard uses Ratatui/Crossterm. Neither is a React/Next.js/Vite application. The shadcn initialization gate is not applicable; initialize no web design system and add no dependency.

## Spacing Scale

Not applicable. JSON results have no pixel layout. Preserve the existing client presentation; do not introduce spacing tokens or modify terminal layout in this phase. Phase 161 owns operator/dashboard presentation.

## Typography

Not applicable. No font family, size, weight or line-height choices are introduced. Machine-readable property names and hexadecimal strings are wire-contract data, not typography.

## Color

Not applicable. A 60/30/10 surface split, accent reservation and destructive color have no meaning for a JSON-RPC response. This phase introduces no visual surfaces or color tokens.

## Copywriting Contract

| Element | Contract |
| --- | --- |
| Primary CTA | Not applicable. Existing authenticated clients invoke `getblockfilter` or `getindexinfo`. |
| Empty state heading | Not applicable: no visual empty-state component. |
| Empty state body | `getindexinfo` returns `{}` for a disabled index or unmatched exact-name selection. Do not add explanatory keys or placeholder index entries. |
| Error state | Preserve pinned RPC codes, exact messages and precedence specified below; operational advice belongs in contributor documentation rather than extra JSON fields. |
| Destructive confirmation | None. Both methods are queries; neither activates an index, repairs data, downloads history or deletes data. |

### Result and Selection Contract

| Method | Request | Successful result |
| --- | --- | --- |
| `getblockfilter` | Positional/named `blockhash` and optional `filtertype`; omitted/null filter type defaults to BASIC. | Exactly `filter` and `header`, both lowercase hexadecimal. Header uses uint256 display order, reversed from raw commitment bytes. |
| `getindexinfo` | Optional positional/named `index_name`; omitted/null/empty string selects all enabled in-scope indexes; exact `basic block filter index` selects BASIC. | Only `basic block filter index`, whose value contains exactly Boolean `synced` and numeric `best_block_height`; disabled/unmatched selection returns `{}`. |

`synced` means initial synchronization has completed. Preserve its latch during later lag or reorg; never reinterpret it as current-tip equality or configured capability. `best_block_height` is processed progress with zero fallback, not the safe durable checkpoint or current tip. Source: D-08/D-09 and CFRP-02.

### Error and Readiness Contract

Apply existing RPC framework type/arity/named-argument checks first, preserving pinned error codes and exact malformed-hash diagnostics. Then process `getblockfilter` in this order: parse hash, resolve filter type, check enabled index, resolve known block/connected validity, obtain readiness, attempt persisted immutable lookup. Successful stored lookup wins during catch-up and for retained stale/pruned hashes. Missing records use genuine accepted-validation provenance; body availability and active membership are insufficient.

| Outcome | Code | Exact message or rule |
| --- | --- | --- |
| Unknown filter type, including uppercase BASIC or numeric names | -5 | `Unknown filtertype` |
| BASIC disabled | -1 | `Index is not enabled for filtertype basic` |
| Recognized V0 disabled | -1 | Preserve pinned disabled-index message for the recognized V0 name; V0 generation/serving remains excluded. |
| Unknown block | -5 | `Block not found` |
| Failed lookup for never-connected block | -5 | `Filter not found. Block was not connected to active chain.` |
| Failed lookup during initial indexing | -1 | `Filter not found. Block filters are still in the process of being indexed.` |
| Unexpected failed lookup after readiness | -32603 | `Filter not found. This error is unexpected and indicates index corruption.` |
| Backend/corrupt-record failure | Existing error boundary | Fail closed; never return an empty successful filter or ordinary absence. Redact backend details. |

Later pending owner work must not become a corruption diagnosis merely because the initial-sync latch is true. Requests use the existing shared durable authority and a bounded integrity-preserving read path, without regenerating history. Source: D-01–D-07/D-10.

### Authentication and Interaction Evidence

Retain authentication before JSON parsing and context-lock acquisition. Missing/wrong credentials on malformed requests disclose no index state. Cookie/password success, batches, notifications and node-scoped behavior follow the existing transport. Public diagnostics contain no credentials, datadir paths or raw backend details. Source: D-11.

Implementation verification must exercise actual authenticated daemon dispatch, exact keys/error precedence, active and retained stale/pruned lookup after real paired deletion/reopen, available/missing catch-up records, later lag, known-unconnected/connected absence, backend faults and concurrent lifecycle outcomes. Contributor UAT uses explicit repo-local Cargo/Bazel invocations. Native verification and parity evidence remain execution gates owned by the parent workflow; this document does not claim implementation has passed them. Source: D-12/D-13.

## Registry Safety

| Registry | Blocks Used | Safety Gate |
| --- | --- | --- |
| none | none | Not applicable, confirmed 2026-10-09T16:16:31Z: no shadcn initialization or registry blocks are part of this headless phase. |

## Checker Sign-Off

- [ ] Dimension 1 Copywriting: pending review of RPC result/error contract.
- [ ] Dimension 2 Visuals: pending applicability review; no visual frontend.
- [ ] Dimension 3 Color: pending applicability review; no rendered surfaces.
- [ ] Dimension 4 Typography: pending applicability review; no rendered text contract.
- [ ] Dimension 5 Spacing: pending applicability review; no pixel layout.
- [ ] Dimension 6 Registry Safety: pending review; no registry use.

**Approval:** pending. Checker should validate the documented nonvisual scope and RPC interaction contract without requiring invented visual tokens.
