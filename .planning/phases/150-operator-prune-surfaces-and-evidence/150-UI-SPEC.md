---
phase: 150
slug: operator-prune-surfaces-and-evidence
status: approved
reviewed_at: 2026-09-28T18:55:00Z
shadcn_initialized: false
preset: none
generated_by: gsd-ui-researcher
lifecycle_mode: yolo
phase_lifecycle_id: 150-2026-09-28T17-08-23
created: 2026-09-28
generated_at: 2026-09-28T18:30:00Z
---

# Phase 150 — UI Design Contract

> Visual and interaction contract for the Ratatui dashboard, human/JSON CLI status, and support Markdown prune section. This is not a web frontend contract.

Reuse the existing dark terminal dashboard. Do not add a hosted web dashboard, browser chrome, marketing footer, OpenLinks disclosure, shadcn, Radix, cards, heroes, or a second presentation model.

---

## Source Decisions

| Source | Decisions Used |
| --- | --- |
| `150-CONTEXT.md` | D-01 through D-17, D-09, D-13: quartet on status, CLI, and dashboard; `pruned` is mode-on, not the earned label; omit keys by presence rules; dashboard shows manual-prune height or typed refusal and has no delete control; dashboard lists locks and does not edit them; support reports counts and last prune height without paths, hashes, or lock names. D-18 forbids archive, assumeutxo, BIP37, compact filters, public defaults, and production-readiness claims. |
| `150-RESEARCH.md` | Append a `Prune` section after the existing five sections. Do not nest facts in `chainstate_durability` or `sections[2]`. Three distinct integers. Stopped reads show JSONC mode only; height, locks, and counters stay unavailable. CLI spelling `open-bitcoin prune …`. |
| `REQUIREMENTS.md` | OPER-01, OPER-02, OPER-03, LOCK-02. Dashboard reads. Destructive request stays on RPC and CLI. |
| `144-UI-SPEC.md` | Existing Ratatui lock: four bands, 35/35/30 split, eight charts, Cyan/Bold titles, Gray labels, Green sparklines, Yellow destructive service keys, 25-row minimum. |
| Existing dashboard | `packages/open-bitcoin-cli/src/operator/dashboard/`. Sections are `Node`, `Sync and Peers`, `Mempool and Wallet`, `Service`, `Logs and Health`. `sections[2]` must remain `Mempool and Wallet`. |

---

## Surface Scope

| Surface | Contract |
| --- | --- |
| Ratatui dashboard | Append one read-only `Prune` section. Show the quartet rows when those facts exist, the lock list, and the last manual-prune outcome. Add no action that deletes payloads or edits locks. |
| CLI human status | Insert the same rows immediately after the chainstate-durability lines and before `Wallet:`. Single-spaced `Label: value`. |
| CLI JSON status | Serialize the shared snapshot. Configured-mode facts and earned `pruned_count` stay separate. Do not open Fjall from the CLI. |
| Support Markdown and JSON | Sibling `## Prune` section after `## Chainstate Durability`. Counts and last deleted height only. No lock names, paths, Fjall key names, or block hashes. |
| RPC and CLI commands | `pruneblockchain`, `listprunelocks`, `setprunelock`, and `clearprunelock` stay the mutation path. Dashboard keys do not call them. |

---

## Design System

| Property | Value |
| --- | --- |
| Tool | none |
| Preset | not applicable |
| Component library | existing Ratatui / Crossterm widgets only (`Block`, `Borders`, `List`, `Paragraph`, `Sparkline`) |
| Icon library | none |
| Font | terminal default (dashboard) / Markdown renderer default (support) |
| Registry | not applicable |

shadcn gate: `components.json` is absent. This phase is Ratatui, not React, Next.js, or Vite. Do not initialize shadcn.

---

## Layout And Density

| Area | Rule |
| --- | --- |
| Dashboard chrome | Keep the existing four vertical bands: title height 3, sections `Min(10)`, chart strip height 8, action bar height 4. Horizontal split stays 35% / 35% / 30%. |
| Section index | Keep the current five sections at indexes 0–4. Append `Prune` as index 5 so `sections[4..]` renders `Logs and Health` then `Prune` in the right column. `sections[0..2]` and `sections[2..4]` stay unchanged. `sections[2]` remains `Mempool and Wallet`. |
| Dashboard rows | One label, one single-line value. One row per lock. No nested tables, editors, height inputs, or confirmation dialogs. |
| Dashboard charts | Keep `MAX_DASHBOARD_CHARTS = 8`. Prune facts get no sparkline. |
| Dashboard actions | Keep the existing service action bar. Do not add a prune, lock-set, or lock-clear `ActionEntry`. |
| CLI human status | One line per visible label. No blank line before `Wallet:`. |
| Support Markdown | One `## Prune` heading plus short bullets. No tables, paths, or lock names. |

The interactive dashboard stays blocked below 25 rows. Keep the existing small-window blocker copy.

---

## Spacing Scale

Declared values are terminal-cell equivalents of the 8-point scale, not CSS:

| Token | Value | Usage |
| --- | --- | --- |
| xs | 4 | Action-bar band height. |
| sm | 8 | Chart-strip band height. |
| md | 16 | One blank line before a support `##` heading only. |
| lg | 24 | Unused. Do not add decorative margins. |
| xl | 32 | Unused. |
| 2xl | 48 | Unused. |
| 3xl | 64 | Unused. |

Exceptions:

- Title band stays 3 rows.
- Minimum interactive height stays 25 rows.
- CLI human status stays single-spaced.

---

## Typography

Terminal faces use one cell size. Weight 600 is `Modifier::BOLD` only.

| Role | Size | Weight | Line Height |
| --- | --- | --- | --- |
| Label | 14px role / 1 terminal cell | 400 | 1.0 |
| Body | 16px role / 1 terminal cell | 400 | 1.0 |
| Heading | 20px role / 1 terminal cell | 600 | 1.2 |
| Display | 28px role / 1 terminal cell | 600 | 1.2 |

- **Label (400):** row labels, `Color::Gray`.
- **Body (400):** row values, lock rows, refusal text, CLI values.
- **Heading (600):** section title `Prune` and support `## Prune`.
- **Display (600):** `Open Bitcoin Dashboard` title only.

Plain text (`--no-color`, JSON, support Markdown) must stay complete without color.

---

## Color

Keep the existing dark Ratatui theme.

| Role | Value | Usage |
| --- | --- | --- |
| Dominant (60%) | terminal default dark background / `#000000`; body `Color::White` / `#E5E5E5` | Dashboard ground and prune row values. |
| Secondary (30%) | `Color::Gray` / `#808080` labels; `Color::DarkGray` / `#555555` separators and borders | Row labels and borders. |
| Accent (10%) | `Color::Cyan` / `#00FFFF` | Display title and section headings only, including `Prune`. |
| Chart accent | `Color::Green` / `#00FF00` | The existing eight sparklines only. |
| Destructive | `Color::Yellow` / `#FFFF00` | Existing service-action keys only. |
| Warning blocker | `Color::Yellow` + bold | Existing small-window blocker heading only. |

Accent reserved for:

1. `Open Bitcoin Dashboard` title.
2. Existing section titles, including the new `Prune` title, in Cyan + Bold.
3. Existing eight Green sparklines.

Do not color prune mode, automatic pruning, refusals, or lock rows. Do not add a light theme. Yellow stays on the existing service keys, not on prune copy.

---

## Copywriting Contract

| Element | Copy |
| --- | --- |
| Primary CTA | `Inspect prune status` |
| Empty state heading | `Prune locks: none` |
| Empty state body | `No prune lock is set. List, set, and clear locks with the CLI prune lock commands. The dashboard only lists locks.` |
| Error state | `Manual prune: refused {reason}` |
| Destructive confirmation | Not applicable on the dashboard. Existing service confirmations stay. `pruneblockchain` and `open-bitcoin prune run` run on RPC and CLI with no dashboard prompt. |

### Status rows

Show a row only when that fact exists.

| Label | When shown | Value |
| --- | --- | --- |
| `Prune mode` | Always when the collector resolved config or a live node | `true` or `false`. `true` means `PruneMode::ManualOnly` or `PruneMode::Automatic`. |
| `Prune height` | Prune mode is on and the node was read | Integer. `0` when mode is on and nothing is pruned. This is `getblockchaininfo.pruneheight` (last pruned height plus one, or `0`). |
| `Automatic pruning` | Prune mode is on | `true` only for automatic mode. Manual-only is `false`. |
| `Prune target` | Automatic pruning is on | Byte count, for example `576716800`. Manual-only and disabled omit this row. |

Disabled mode shows `Prune mode: false` and omits `Prune height`, `Automatic pruning`, and `Prune target`. Do not print `0` for an omitted target.

Stopped or unreachable, with JSONC mode resolved and the node unread:

- `Prune mode` comes from the resolved `prune` integer (`0` disabled, `1` manual-only, `N >= 550` automatic MiB converted to bytes for the target).
- `Automatic pruning` and `Prune target` follow that same config when mode is on.
- `Prune height`, the lock list, the manual-prune outcome, and support counters render `Unavailable: {reason}` with the collector reason. Do not substitute `0` or the last deleted height.

### Lock list

Read-only. Dashboard and CLI status may show names. Support evidence must not.

| State | Copy |
| --- | --- |
| Live and empty | `Prune locks: none` |
| Live with locks | One row per lock: `Prune lock: name={name} height_first={n} height_last={n}` |
| Unread | `Prune locks: Unavailable: {reason}` |

No dashboard control changes a name or range. Same-name replace and clear stay on:

```text
open-bitcoin prune lock list
open-bitcoin prune lock set --name <name> --height-first <n> --height-last <n>
open-bitcoin prune lock clear --name <name>
```

RPC names stay `listprunelocks`, `setprunelock`, and `clearprunelock`.

### Manual prune outcome

The dashboard and CLI status show the last outcome as text. They do not accept a height and they do not delete payloads.

| State | Copy |
| --- | --- |
| No request yet on a live node | `Manual prune: none` |
| Success | `Manual prune: height={n}` where `{n}` is the `pruneblockchain` result, including `-1` when nothing was pruned. Do not show the plus-one info field here. |
| Disabled | `Manual prune: refused node is not in prune mode` |
| Chain too short | `Manual prune: refused blockchain is too short for pruning` |
| Target above tip | `Manual prune: refused blockchain is shorter than the attempted prune height` |
| Keep window | `Manual prune: refused target is inside the 288-block keep window` |
| Negative height | `Manual prune: refused negative block height` |
| Unread | `Manual prune: Unavailable: {reason}` |

The keep-window line must say refused. It must not say the height was clamped.

Request path, not a dashboard key:

```text
open-bitcoin prune run <height-or-timestamp>
```

RPC name stays `pruneblockchain`.

### Three heights, three labels

| Fact | Label | Formula |
| --- | --- | --- |
| Info field | `Prune height` | Last contiguous pruned height plus one, or `0` while mode is on and nothing is pruned. Absent when mode is off. |
| Manual RPC result | `Manual prune: height={n}` | Same helper without plus one, or `-1` when nothing is pruned. |
| Support last batch | `Last prune height` | Highest height deleted by the last successful batch. Absent until that batch exists. |

Do not render these three as the same number or the same label.

### Support Markdown

Heading: `## Prune`, immediately after `## Chainstate Durability`. Do not put these bullets inside chainstate durability.

Loaded and never deleted:

- `- Prune batches: 0`
- `- Pruned heights: 0`
- `- Next action: Read prune batch counts and the last deleted height as local operator evidence. Request a manual prune or a lock change from RPC or the CLI.`

After a successful delete, add `- Last prune height: {n}` between the height count and the next-action bullet. Omit that bullet when no successful batch exists. Do not print `Last prune height: 0`.

Probe-only or unread:

- `- Prune batches: Unavailable: {reason}`
- `- Pruned heights: Unavailable: {reason}`
- `- Last prune height: Unavailable: {reason}`
- `- Next action: …` (same sentence)

Support JSON uses `successful_batch_count`, `pruned_height_count`, and an absent `last_prune_height` until a delete exists. It does not include `name`, datadir, Fjall key names, or block hashes.

### Forbidden copy

On the new prune dashboard section, CLI prune lines, support prune section, and help added by this phase, do not claim:

`archive-node`, `archive node`, `assumeutxo`, `assumevalid`, `BIP37`, `compact filter`, `public default`, `production ready`, `production readiness`, `production-funds`.

Do not copy earned `pruned_count` or `BlockServingDataAvailability::Pruned` into this section. A missing payload without have-pruned stays `Unavailable` on the existing availability surface.

---

## Interaction And State Contract

| Interaction | Contract |
| --- | --- |
| Status | Refresh with existing `r` and the existing tick (`tick_ms.max(250)`). Renderers display the projected quartet. They do not recompute prune height locally. |
| Locks | Display only. No selection, rename, set, or clear key. |
| Manual prune | Display the height or the typed refusal. No key, input, or confirmation deletes payloads. |
| Existing keys | Keep `r`, `s`, `t`, `o`, `x`, `i`, `u`, `e`, `d`, `h`/`?`, `q`/`Esc`/`Ctrl-C`, `y`/`n`. Service confirmations stay. |
| Help line | Keep the existing prompt. Do not advertise a prune key. |

---

## Component Inventory

| Piece | Action |
| --- | --- |
| `DashboardState::from_snapshot` | Append the `Prune` section from the shared snapshot. Do not add a parallel DTO or a delete action. |
| `dashboard_actions` | Unchanged. |
| `render_human_status` | Insert the same lines after chainstate durability and before `Wallet:`. |
| Support render | Add `## Prune` after chainstate durability. |
| Ratatui widgets | Reuse `List` and `Paragraph`. No new widget types. |

---

## Registry Safety

| Registry | Blocks Used | Safety Gate |
| --- | --- | --- |
| shadcn official | none | not applicable — 2026-09-28. Ratatui surface; `components.json` is absent and must stay absent. |
| third-party registries | none | not applicable — 2026-09-28. No third-party registry declared; vetting gate not required. |
| terminal / Ratatui widgets | existing project code only | no external registry — 2026-09-28 |

---

## Checker Sign-Off

- [ ] Dimension 1 Copywriting: PASS
- [ ] Dimension 2 Visuals: PASS
- [ ] Dimension 3 Color: PASS
- [ ] Dimension 4 Typography: PASS
- [ ] Dimension 5 Spacing: PASS
- [ ] Dimension 6 Registry Safety: PASS

**Approval:** pending

| Dimension | PASS when |
| --- | --- |
| Copywriting | Quartet, lock list, manual-prune outcome, empty/error/support lines, and the three height labels match this contract. No archive, assumeutxo, or production-readiness claim. |
| Visuals | New section is read-only `Prune` at index 5. `sections[2]` stays `Mempool and Wallet`. Eight charts and the service action bar stay. No delete or lock-edit control. |
| Color | Existing dark theme. Cyan for the `Prune` title only. No extra color meaning on mode, locks, or refusals. |
| Typography | Two weights (400/600). Roles above. Plain text stays complete. |
| Spacing | 4/8/16 scale plus the listed exceptions. CLI lines stay single-spaced. |
| Registry Safety | Tool none. No shadcn. No third-party blocks. |
