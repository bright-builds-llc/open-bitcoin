---
phase: 144
slug: operator-flush-and-availability-evidence
status: draft
shadcn_initialized: false
preset: none
generated_by: gsd-ui-researcher
lifecycle_mode: yolo
phase_lifecycle_id: 144-2026-09-17T11-23-17
created: 2026-09-17
generated_at: 2026-09-17T12:00:00Z
---

# Phase 144 — UI Design Contract

> Visual and interaction contract for operator-facing terminal, Ratatui dashboard,
> human/JSON CLI status, `openbitcoinnetworkstatus`, metrics, structured logs,
> and support Markdown. This is not a web frontend contract.

Do not add a hosted web dashboard, browser chrome, marketing footer, OpenLinks
disclosure, shadcn, Radix, cards, heroes, or a second local presentation model.
Do not invent a `getblock` product UI.

---

## Source Decisions

| Source | Decisions Used |
| --- | --- |
| `144-CONTEXT.md` | D-01 through D-23: one dedicated snapshot field; `FieldAvailability`; do not fold into `recovery_evidence` or `block_relay`; no `getblock`; cache-size / last-flush / write-kind / readiness vocabularies; coins-marker recovery distinct from sync recovery; coins best-block height+hash; have-bytes vs do-not labels; no `pruned` on this field; CLI `Label: value` after block-relay; dashboard rows only; fixed metrics/logs; bounded support; docs + checker. |
| `144-RESEARCH.md` | Field name `chainstate_durability`; types in `status/chainstate_durability.rs`; compact cluster after block-relay; last labels plus three D-12 counters; log source `chainstate_durability`; no ninth chart. |
| `REQUIREMENTS.md` | CSOBS-01, CSOBS-02. Success: sanitized flush/recovery/have-bytes surfaces; cache-size OK/LARGE/CRITICAL and last flush reason; coins best-block, interrupted-flush/replay, and availability labels without peer ids or coin dumps. |
| `137-UI-SPEC.md` | TUI-only baseline: `MAX_DASHBOARD_CHARTS = 8`, four vertical bands, 35/35/30 split, one-line rows, `Unavailable: {reason}`, Cyan/Bold titles, Gray labels. |
| `docs/architecture/status-snapshot.md` | `OpenBitcoinStatusSnapshot` is the sole shared status model. Missing fields render `Unavailable: {reason}`. |
| `docs/architecture/operator-observability.md` | Fixed low-cardinality metrics/logs; existing redaction; no dynamic labels. |
| `standards/core/frontend-ui.md` | Dark default. Open-source identity disclosure applies to public web apps only — not this terminal surface. |
| Existing dashboard | Keep Ratatui 8-chart layout, Cyan/Bold titles, Gray labels, Green sparklines, Yellow destructive service keys, 25-row minimum, and existing Phase 105/108/116/137 rows. |

---

## Surface Scope

| Surface | Contract |
| --- | --- |
| CLI human status | Insert the new `Label: value` cluster immediately after the existing `block_relay` lines and before `Wallet:`. Do not reorder sync, inbound, mempool-policy, relay, block-relay, wallet, service, log, metric, or health lines. |
| CLI JSON status | Serialize the shared `OpenBitcoinStatusSnapshot` only. New facts live under top-level `chainstate_durability`. No coin tables, request logs, or `getblock` objects. |
| Ratatui dashboard | Add or swap **rows** after existing `block_relay_rows` inside `Mempool and Wallet`. Keep `MAX_DASHBOARD_CHARTS = 8`. Do not add a chart, panel, or section. |
| `openbitcoinnetworkstatus` | Publish the same `chainstate_durability` field. Baseline `getblockchaininfo`, `getnetworkinfo`, and any existing stored-block RPC stay unchanged. |
| Metrics and structured logs | Fixed `MetricKind` / allowlisted keys matching snapshot labels. No dynamic labels. No coins-best-block hash in log fields. |
| Support Markdown | One heading plus short bullets from the shared field. Counts, fixed labels, and the existing tip-hash pattern only. Reuse the Phase 105/108/116 redaction path. |
| Baseline Knots RPC | Presentation-free. No extra Open Bitcoin keys on Knots-shaped objects. |

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

Do not introduce shadcn, Radix, Base UI, browser components, images, cards, hero
sections, decorative gradients, icon packs, or web layout assets.

---

## Layout And Density

| Area | Rule |
| --- | --- |
| Dashboard chrome | Keep the existing four vertical bands: title height 3, sections `Min(10)`, chart strip height 8, action bar height 4. Horizontal section split stays 35% / 35% / 30%. |
| Dashboard charts | Exactly eight slots. Existing kinds remain: header, downloaded, connected, sync, peers, plus the three optional inbound/relay substitutions already implemented. Phase 144 fields **do not** get sparkline slots. |
| Dashboard rows | One label, one single-line value. Prefer `key=value` pairs. No nested tables, request logs, or wrapped identifier columns. |
| CLI human status | One line per locked label using `Label: value`. New durability lines use the same labels as dashboard rows. |
| Support Markdown | One heading plus short bullets. No volatile tables, raw logs, pasted JSON-RPC bodies, coin dumps, or undo blobs. |
| JSON | snake_case machine names. Tagged `FieldAvailability` (`state` / `value`). |

Stopped-node or unprojected runtime: every new row/line renders `Unavailable: {reason}` using the **same** snapshot reason on `chainstate_durability`. Do not invent a second stopped copy or scan Fjall from the CLI.

Do not add a sixth dashboard section. New rows stay inside `Mempool and Wallet` so the current left `[0..2]` / middle `[2..4]` / right `[4..]` column split is unchanged.

---

## Spacing Scale

Declared values are terminal-cell / text equivalents of the 8-point scale, not CSS:

| Token | Value | Usage |
| --- | --- | --- |
| xs | 4 | Action-bar band height; inline `key=value` gap is one ASCII space inside that compact band. |
| sm | 8 | Chart-strip band height; compact row padding inside bordered lists. |
| md | 16 | Default gap between logical status clusters in human CLI (one blank line only in support Markdown, not between CLI lines). |
| lg | 24 | Unused in this phase. Do not add 24-cell decorative margins. |
| xl | 32 | Unused in this phase. |
| 2xl | 48 | Unused in this phase. |
| 3xl | 64 | Unused in this phase. |

Exceptions:

- Existing title band stays **3** rows (`Constraint::Length(3)`). Do not restyle it to 4.
- Interactive dashboard remains blocked below **25** rows (`MIN_INTERACTIVE_DASHBOARD_HEIGHT`). Keep the existing small-window blocker copy.
- CLI human status stays single-spaced (`\n` between lines). Do not insert blank lines between the new durability lines.

---

## Typography

Terminal faces cannot scale independently. Declare four roles and two weights.
Implementation uses one cell size; weight 600 is `Modifier::BOLD` only.

| Role | Size | Weight | Line Height |
| --- | --- | --- | --- |
| Label | 14px role / 1 terminal cell | 400 | 1.0 |
| Body | 16px role / 1 terminal cell | 400 | 1.0 |
| Heading | 20px role / 1 terminal cell | 600 | 1.2 |
| Display | 28px role / 1 terminal cell | 600 | 1.2 |

Role assignment:

- **Label (400):** dashboard row labels (`Color::Gray`), CLI `Label:` prefixes.
- **Body (400):** dashboard row values, CLI values, `key=value` tokens, JSON is machine text not typography.
- **Heading (600):** section titles (`Node`, `Mempool and Wallet`, …) and support `##` headings.
- **Display (600):** `Open Bitcoin Dashboard` title only.

Do not rely on italics, underline, icons, or color-only meaning. Plain text
(CLI `--no-color`, JSON, support Markdown) must remain complete.

---

## Color

Keep the existing dark Ratatui theme. Values below are the ANSI/Ratatui lock
plus hex approximations for the 60 / 30 / 10 contract.

| Role | Value | Usage |
| --- | --- | --- |
| Dominant (60%) | terminal default dark background / `#000000`; body text `Color::White` / `#E5E5E5` | Dashboard ground, list interiors, CLI/JSON default text. |
| Secondary (30%) | `Color::Gray` / `#808080` labels; `Color::DarkGray` / `#555555` action separators and borders | Row labels, `|` separators, `Block` borders. |
| Accent (10%) | `Color::Cyan` / `#00FFFF` | Display title and section headings only. |
| Chart accent | `Color::Green` / `#00FF00` | The existing eight sparklines only. |
| Destructive | `Color::Yellow` / `#FFFF00` | Existing service-action keys (`t` start, `o` stop, `x` restart, `i` install, `u` uninstall, `e` enable, `d` disable) only. |
| Warning blocker | `Color::Yellow` + bold | Existing small-window blocker heading only. |

Accent reserved for:

1. `Open Bitcoin Dashboard` title.
2. Section titles already rendered in Cyan + Bold.
3. Existing eight Green sparklines.

Accent is **not** reserved for Phase 144 rows, cache-size tokens, recovery
outcomes, have-bytes status, or flush reasons. Those stay default body color.

Do not add a light theme, a second accent for `available` / `OK`, or
color-coding that implies prune-mode, archive-node honesty, or public historical
serving (for example green = have-bytes).

---

## Copywriting Contract

| Element | Copy |
| --- | --- |
| Primary CTA | `Inspect status` |
| Dashboard refresh | Keep existing `r refresh`. No new dashboard key for flush or availability actions. |
| Empty-state heading | `Have-bytes counts: available_count=0` |
| Empty-state body | Show every new durability row from an `Available` projection with zero classification counts, for example `Have-bytes counts: available_count=0 unavailable_count=0 index_known_without_payload_count=0`. Next step: `Inspect status with the repo-local open-bitcoin status command. This does not flush coins, serve a stored block, or change recovery state.` |
| Stopped / unreachable | `Unavailable: {reason}` using the collector reason already applied to `chainstate_durability`. |
| Missing extension | `Unavailable: openbitcoinnetworkstatus unavailable: {detail}` — never invent field-local prose. |
| Error state | `{problem}. {next step}` — example: `Chainstate durability evidence unavailable. Inspect status after the node has projected flush and availability facts; do not scan coins from the CLI.` Renderer-local Fjall reads are forbidden. |
| Small-window blocker | Keep existing: `Interactive dashboard unavailable` / needs at least 25 rows / `Resize the terminal to continue. Press q, Esc, or Ctrl-C to quit.` |
| Support next action | `Next action: Treat flush, coins recovery, cache-size, and have-bytes versus do-not as bounded local operator status. This is not prune-mode, archive-node serving, public-default historical serving, or production readiness.` |
| Destructive confirmation | Not applicable for Phase 144 evidence. Do not add confirmations for status, dashboard refresh, or support export. Existing service start/stop/restart/install/uninstall/enable/disable confirmations stay unchanged. |

### Locked dashboard and human-status labels

Use these exact title-case labels on dashboard rows and CLI human lines.
Insert them immediately after the existing block-relay cluster.

| Label | Value format when available |
| --- | --- |
| `Chainstate durability` | `cache_size={OK\|LARGE\|CRITICAL} last_flush_reason={none\|needed\|periodic\|always\|failed_disk} write_kind={none\|flush\|sync\|refuse_disk_space} readiness={not_ready\|ready_to_flush}` |
| `Cache occupancy` | `cache_bytes={n} cache_byte_limit={n}` |
| `Coins best-block` | `height={n} hash={64-hex}` using the same full-hash pattern as existing `Connected block` / tip-hash rows. If recovery is `fail_closed` or `interrupted` without coins `B`, this line is `Unavailable: {reason}` — never invent a tip from interrupted `H` hashes. |
| `Coins recovery` | `{consistent\|replayed\|interrupted\|fail_closed}` |
| `Have-bytes` | `{available\|unavailable} payload_present={true\|false} index_known={true\|false} validated_on_active_chain={true\|false}` |
| `Have-bytes counts` | `available_count={n} unavailable_count={n} index_known_without_payload_count={n}` |

When the whole `chainstate_durability` field is unavailable, each of the six
lines/rows is `Unavailable: {reason}` with that same reason.

Human `cache_size` tokens are `OK` / `LARGE` / `CRITICAL`. JSON, metrics, and
logs use `ok` / `large` / `critical`. Other enum tokens stay snake_case on every
surface.

`Have-bytes` serving status is `unavailable` whenever `payload_present` is
false, even if `index_known` or `validated_on_active_chain` is true.

Preserve existing Phase 105/108/116/137 labels unchanged, including:

`Mempool`, `Relay evidence`, `Relay recovery`, `Recovery` (mempool counts),
`Block relay evidence`, `Block relay activation`, `Block relay eligibility`,
`Block relay status`, compact-relay rows, and `Public relay`.

`Coins recovery` is distinct from `Recovery` (Phase 135/137 mempool recovery
counts), `Relay recovery` (Phase 108), `recovery_evidence` (Phase 77
lock/corruption), and `sync.recovery_category` (`clean_shutdown`,
`store_corruption`, …). Do not reuse those labels or taxonomies.

### Support Markdown bullets

Heading: `## Chainstate Durability`

Required bullets, in this order:

- `- Chainstate durability: {value}`
- `- Cache occupancy: {value}`
- `- Coins best-block: {value}`
- `- Coins recovery: {value}`
- `- Have-bytes: {value}`
- `- Have-bytes counts: {value}`
- `- Next action: Treat flush, coins recovery, cache-size, and have-bytes versus do-not as bounded local operator status. This is not prune-mode, archive-node serving, public-default historical serving, or production readiness.`

Values match the dashboard/CLI formats above. One heading, short bullets, no
tables.

### Forbidden copy

Never emit on the new `chainstate_durability` field, its dashboard/CLI/support
projections, metrics, logs, help, or docs added by this phase:

`pruned`, `block_status_pruned`, `getblock`, `peer_id`, `peer id`, outpoint
shapes (`txid:vout`), coin dumps, UTXO maps, undo blobs, raw hex payloads,
`archive-node`, `archive node`, `prune mode`, `public default`,
`public-default historical serving`, `production ready`, `production readiness`,
`assumeutxo`, `compact filter`, or `cmpctblock`.

Existing `block_relay.block_serving.status.pruned_count` may remain on the old
Phase 116 surface only. Do not document it as prune-mode and do not copy it onto
`chainstate_durability`.

`EligibleServe`, compact-relay, and mempool dual-state copy stay on their
existing surfaces. This phase does not retitle them.

---

## Data Presentation Contract

| Evidence type | Presentation |
| --- | --- |
| Snapshot field | Top-level `chainstate_durability: FieldAvailability<ChainstateDurabilityEvidence>`. Distinct from `recovery_evidence` and `block_relay`. |
| Cache-size | Current occupancy class from observability-only `FlushMode::None` at projection time. Machine `ok` / `large` / `critical`. Human `OK` / `LARGE` / `CRITICAL`. Do not recompute 90% / 10 MiB thresholds in CLI, RPC, or dashboard. |
| Last flush reason | Last real `execute_flush` reason: `none` / `needed` / `periodic` / `always` / `failed_disk`. A later `None` classification must not overwrite it. |
| Write kind | Last write-kind: `none` / `flush` / `sync` / `refuse_disk_space`. |
| Readiness | CanFlush: `not_ready` / `ready_to_flush`. |
| Occupancy | Aggregate `cache_bytes` and `cache_byte_limit` only. Never Fjall item count or per-coin occupancy. |
| Coins best-block | Height plus 64-hex hash, matching existing tip-hash rows. Chain-authority evidence only. |
| Coins recovery | `consistent` / `replayed` / `interrupted` / `fail_closed`. Distinguishable. Fail-closed must not present an invented consistent tip. |
| Have-bytes vs do-not | Last serving status `available` / `unavailable` plus `payload_present`, `index_known`, `validated_on_active_chain`. |
| Availability counts | `available_count`, `unavailable_count`, `index_known_without_payload_count` only. No hash-keyed request log. |
| Identifiers | Forbidden on snapshot, dashboard, status, metrics, logs, and support except the existing tip/coins-best-block hash pattern on status/CLI/dashboard/support Markdown. No peer ids, coin dumps, outpoints, or raw hex. |
| JSON machine names | Exact snake_case fields below. |
| Support redaction | Counts and fixed labels need no redaction. Free-text reasons sanitize through the existing recursive path. Reject raw coin records, undo blobs, peer identifiers, dynamic labels, and raw hex. Coins best-block hash follows the existing tip-hash exception, not a new hash table. |

### Locked JSON field names

`OpenBitcoinStatusSnapshot.chainstate_durability` and
`openbitcoinnetworkstatus.chainstate_durability` serialize this shape when
available (`rename_all = "snake_case"`):

| Machine name | Values |
| --- | --- |
| `cache_size` | `ok` \| `large` \| `critical` |
| `last_flush_reason` | `none` \| `needed` \| `periodic` \| `always` \| `failed_disk` |
| `write_kind` | `none` \| `flush` \| `sync` \| `refuse_disk_space` |
| `readiness` | `not_ready` \| `ready_to_flush` |
| `cache_bytes` | `u64` |
| `cache_byte_limit` | `u64` |
| `recovery_outcome` | `consistent` \| `replayed` \| `interrupted` \| `fail_closed` |
| `maybe_coins_best_block_height` | `null` or integer |
| `maybe_coins_best_block_hash` | `null` or 64 lowercase hex |
| `last_serving_status` | `available` \| `unavailable` |
| `last_payload_present` | `true` \| `false` |
| `last_index_known` | `true` \| `false` |
| `last_validated_on_active_chain` | `true` \| `false` |
| `available_count` | `u64` |
| `unavailable_count` | `u64` |
| `index_known_without_payload_count` | `u64` |

Wrap the struct in `FieldAvailability`. A stopped node is one unavailable
reason, not a fabricated zeroed object. Zeroed counters are acceptable only
inside `Available` after the runtime has projected.

### Locked metrics and log labels

Structured-log source: `chainstate_durability`.

Allowlisted log keys only: `cause`, `outcome`, `label`, plus numeric counts and
optional height. Example message shape:

`outcome=projected cause=status_projection label=chainstate_durability cache_size=ok last_flush_reason=periodic recovery_outcome=replayed last_serving_status=unavailable available_count=0 unavailable_count=1`

Do not put coins-best-block hash, peer ids, outpoints, or raw hex in log fields.

Fixed metric kinds (closed `MetricKind`, no dynamic labels):

| Kind (illustrative serde name) | Sample |
| --- | --- |
| `chainstate_durability_cache_size_class` | current class `0` / `1` / `2` for `ok` / `large` / `critical` |
| `chainstate_durability_last_flush_reason_class` | last reason `0..4` for `none` / `needed` / `periodic` / `always` / `failed_disk` |
| `chainstate_durability_write_kind_class` | last write-kind `0..3` for `none` / `flush` / `sync` / `refuse_disk_space` |
| `chainstate_durability_recovery_class` | `0..3` for `consistent` / `replayed` / `interrupted` / `fail_closed` |
| `chainstate_durability_available_count` | counter |
| `chainstate_durability_unavailable_count` | counter |
| `chainstate_durability_index_known_without_payload_count` | counter |

These series must not bind to a ninth dashboard chart.

---

## Interaction And State Contract

| Interaction | Contract |
| --- | --- |
| Dashboard keys | No new keys. Keep `r` refresh, `s` status, service keys, `h`/`?` help, `q`/`Esc`/`Ctrl-C` quit, `y`/`n` for existing service confirmations. |
| Refresh | `r` and the existing tick (`tick_ms.max(250)`) rebuild `DashboardState::from_snapshot` from the shared collector. Renderers must not reclassify flush, cache-size, recovery, or have-bytes locally. |
| Product actions | None. This phase does not add flush, replay, or stored-block fetch commands. |
| Confirmation | None for status, dashboard refresh, or support. Service lifecycle confirmations unchanged. |
| JSON vs human | Same snapshot. Human uses title-case labels and `OK`/`LARGE`/`CRITICAL`; JSON uses snake_case `ok`/`large`/`critical`. |
| Identifier leak | Status RPC is authenticated and is still **not** a coin dump or `getblock`. No last-request hash table. |

---

## Component Inventory

| Piece | Action |
| --- | --- |
| `OpenBitcoinStatusSnapshot` | Add `#[serde(default)] chainstate_durability`. Do not extend `recovery_evidence` or `block_relay`. |
| `OpenBitcoinNetworkStatusResponse` | Publish the same field so live CLI collection can copy it. |
| `DashboardState::from_snapshot` | Consume `chainstate_durability`. Do not add a parallel DTO. |
| `mempool_and_wallet` / `block_relay_rows` | Append the six locked rows after existing block-relay rows. Keep wallet rows after. |
| `render_human_status` / `block_relay_evidence_lines` | Insert matching human lines immediately after the block-relay cluster and before `Wallet:`. |
| `MetricKind` / structured logs | Add only the fixed kinds/keys above. Do not bind them to a ninth chart. |
| `support_status_for_bundle` | Render `## Chainstate Durability` bullets; redact free-text; leave counts. |
| Ratatui widgets | Reuse. No new widget types. |

---

## Registry Safety

| Registry | Blocks Used | Safety Gate |
| --- | --- | --- |
| shadcn official | none | not applicable — 2026-09-17. Not a React/Next.js/Vite surface; `components.json` is absent and must stay absent. |
| third-party registries | none | not applicable — 2026-09-17. No third-party registry declared; vetting gate not required. |
| terminal / Ratatui widgets | existing project code only | no external registry — 2026-09-17 |

---

## Checker Sign-Off

- [ ] Dimension 1 Copywriting: PASS
- [ ] Dimension 2 Visuals: PASS
- [ ] Dimension 3 Color: PASS
- [ ] Dimension 4 Typography: PASS
- [ ] Dimension 5 Spacing: PASS
- [ ] Dimension 6 Registry Safety: PASS

**Approval:** pending

Checker PASS-ready criteria:

| Dimension | PASS when |
| --- | --- |
| Copywriting | Labels, machine field names, empty/error/support copy, cache-size tokens, recovery/have-bytes vocabularies, and the forbidden-word list match this contract. `pruned` / `getblock` stay off the new field. |
| Visuals | Rows only inside the existing Mempool and Wallet section after block-relay; eight charts unchanged; no web chrome, cards, request tables, new panel, or marketing identity footer. |
| Color | Existing dark theme only; Cyan/Green reserved as listed; no color meaning for have-bytes, cache-size, or recovery. |
| Typography | Two weights (400/600); roles above; plain text remains complete without color. |
| Spacing | 4/8/16 scale tokens plus the listed exceptions; CLI lines stay single-spaced; chart strip stays 8 rows; 35/35/30 split unchanged. |
| Registry Safety | Tool none; no shadcn; no third-party blocks; no OpenLinks disclosure. |
