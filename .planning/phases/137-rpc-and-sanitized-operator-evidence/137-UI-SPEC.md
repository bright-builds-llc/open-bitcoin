---
phase: 137
slug: rpc-and-sanitized-operator-evidence
status: approved
reviewed_at: 2026-08-19T22:25:00Z
shadcn_initialized: false
preset: none
generated_by: gsd-ui-researcher
lifecycle_mode: yolo
phase_lifecycle_id: 137-2026-08-19T21-52-42
created: 2026-08-19
generated_at: 2026-08-19T22:20:00Z
---

# Phase 137 — UI Design Contract

> Visual and interaction contract for operator-facing terminal, Ratatui dashboard,
> human/JSON CLI status, package CLI originating responses, metrics, structured
> logs, and support Markdown. This is not a web frontend contract.

Do not add a hosted web dashboard, browser chrome, marketing footer, OpenLinks
disclosure, shadcn, cards, heroes, or a second local presentation model.

---

## Source Decisions

| Source | Decisions Used |
| --- | --- |
| `137-CONTEXT.md` | D-01 through D-18: Knots BaselineParity package RPC vs typed Open Bitcoin package path; identifier-free shared evidence; seven snapshot groups; Open Bitcoin labels on dashboard/status; dual-state axes; no public/default relay or guaranteed-propagation copy. |
| `137-RESEARCH.md` | One projector family; row titles; snapshot machine names; decay labels `half_life_12h` / `half_life_6h` / `half_life_3h` / `not_decaying`; `MAX_DASHBOARD_CHARTS = 8`; no ninth chart; no Knots aliases on operator surfaces. |
| `REQUIREMENTS.md` | MPOBS-01, MPOBS-02, MPOBS-03. Success criteria: package dry-run/submit, distinct resource/fee/pressure/checkpoint/recovery/retry fields, redacted dual-state, relay-disabled admission without propagation claims. |
| `docs/architecture/status-snapshot.md` | `OpenBitcoinStatusSnapshot` is the sole shared status model. Missing fields render `Unavailable: {reason}`. |
| `docs/architecture/operator-observability.md` | Fixed low-cardinality metrics/logs; Phase 105/108 redaction; no dynamic labels. |
| `standards/core/frontend-ui.md` | Dark default. Open-source identity disclosure applies to public web apps only — not this terminal surface. |
| Existing dashboard | Keep Ratatui 8-chart layout, Cyan/Bold titles, Gray labels, Green sparklines, Yellow destructive service keys, 25-row minimum, and existing Phase 105/108/116 rows. |

---

## Surface Scope

| Surface | Contract |
| --- | --- |
| CLI human status | Insert the new mempool-policy lines after `Mempool:` and before `Relay evidence:`. Do not reorder sync, inbound, wallet, service, log, metric, or health lines. |
| CLI JSON status | Serialize the shared `OpenBitcoinStatusSnapshot` only. New fields live under `mempool` groups. No last-package member tables. |
| Ratatui dashboard | Add or swap **rows** in `Mempool and Wallet` from the same snapshot groups. Keep `MAX_DASHBOARD_CHARTS = 8`. Do not add a chart, panel, or section. |
| `open-bitcoin package dry-run\|submit` | Originating authenticated response. May show input-ordered member results, fingerprint, and dual-state. Must not claim public/default relay or propagation. |
| `open-bitcoin-cli testmempoolaccept\|submitpackage` | Knots 29.3 JSON only. Presentation-free. No extra Open Bitcoin keys. |
| `getmempoolinfo` | Keep Knots aliases (`bytes`, `usage`, `maxmempool`, `mempoolminfee`) on this BaselineParity RPC only. |
| Metrics and structured logs | Fixed `MetricKind` / allowlisted keys matching snapshot labels. No dynamic labels. |
| Support Markdown | Bounded sanitized bullets. Counts and fixed labels only. Reuse the Phase 105/108 redaction path. |
| Baseline `sendrawtransaction` | Leave the existing object alone. No dual-state or propagation fields. |

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
| Dashboard charts | Exactly eight slots. Existing kinds remain: header, downloaded, connected, sync, peers, plus the three optional inbound/relay substitutions already implemented. Phase 137 groups **do not** get sparkline slots. |
| Dashboard rows | One label, one single-line value. Prefer `key=value` pairs. No nested tables, member lists, or wrapped identifier columns. |
| CLI human status | One line per concept using `Label: value`. New policy lines use the same labels as dashboard rows. |
| Package CLI | Human output may use multiple lines for the originating report. Shared status/dashboard/support must not echo that report. |
| Support Markdown | One heading plus short bullets. No volatile tables, raw logs, or pasted JSON-RPC bodies. |
| JSON | snake_case machine names. Tagged `FieldAvailability` / existing unavailable wrappers. |

Stopped-node: every new group renders `Unavailable: {reason}` using the **same**
reason already used for `mempool.transactions`. Do not invent a second stopped
copy.

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
- CLI human status stays single-spaced (`\n` between lines). Do not insert blank lines between the new mempool-policy lines.

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

Accent is **not** reserved for Phase 137 rows, package CLI, dual-state tokens,
fee floors, checkpoint, recovery, or retry. Those stay default body color.

Do not add a light theme, a second accent for “accepted”, or color-coding that
implies public relay (for example green = propagated).

---

## Copywriting Contract

| Element | Copy |
| --- | --- |
| Primary CTA | `Dry-run package` |
| Secondary mutating CTA | `Submit package` |
| Dashboard refresh | Keep existing `r refresh`. No new dashboard key for package actions. |
| Empty-state heading | `Mempool: 0 transactions` |
| Empty-state body | Show every new policy row with zero counts, for example `Admission states: accepted=0 still_present=0 cleared=0`. Next step: `Dry-run a package with open-bitcoin package dry-run. This does not change mempool, relay, persistence, or evidence state.` |
| Stopped / unreachable | `Unavailable: {reason}` using the collector reason already applied to `mempool.transactions`. |
| Missing extension | `Unavailable: openbitcoinnetworkstatus unavailable: {detail}` — never invent group-local prose. |
| Error state | `{problem}. {next step}` — example: `Package decode failed. Provide 1 to 25 raw transaction hex strings and retry dry-run.` RPC-level count/topology errors stay on the originating CLI/RPC path, not on dashboard rows. |
| Small-window blocker | Keep existing: `Interactive dashboard unavailable` / needs at least 25 rows / `Resize the terminal to continue. Press q, Esc, or Ctrl-C to quit.` |
| Support next action | `Next action: Treat package, pressure, checkpoint, recovery, and retry evidence as bounded local operator status. Successful local admission is not public or default relay and is not network-wide propagation.` |
| Destructive confirmation | Not applicable for Phase 137 evidence. Do not add confirmations for status, dashboard refresh, dry-run, or support export. Existing service start/stop/restart/install/uninstall/enable/disable confirmations stay unchanged. |

### Locked dashboard and human-status labels

Use these exact title-case labels on dashboard rows and CLI human lines.
Do **not** use Knots aliases `bytes`, `usage`, `maxmempool`, or `mempoolminfee`.

| Label | Value format when available |
| --- | --- |
| `Virtual size` | `{n} vbytes` |
| `Accounted usage` | `{n} accounted bytes` |
| `Accounted capacity` | `{n} accounted bytes` |
| `Static relay floor` | `{n} sat/kvB` |
| `Rolling mempool floor` | `{n} sat/kvB` |
| `Effective admission floor` | `{n} sat/kvB` |
| `Incremental relay fee` | `{n} sat/kvB` |
| `Pressure removals` | `pressure_removal_count={n} decay={half_life_12h\|half_life_6h\|half_life_3h\|not_decaying}` |
| `Eviction (relay)` | `evicted_count={n}` from Phase 105 `relay.outcome_counters` only |
| `Checkpoint` | `outcome={label} overdue={true\|false} persistence_strength={label} age_seconds={n} loss_bound_seconds={n} dirty_generation_present={true\|false}` |
| `Recovery` | count-only `recovered_count={n}` plus existing `dropped_*_count` keys; never a txid |
| `Retry` | `eligible={n} queued={n} attempted={n} emitted={n} requested={n} served={n} suppressed={n} relay_disabled={n} cleared={n}` |
| `Admission states` | `accepted={n} still_present={n} cleared={n}` |
| `Relay states` | `eligible={n} queued={n} attempted={n} emitted={n} requested={n} served={n} suppressed={n} relay_disabled={n}` |

Preserve existing Phase 105/108 labels unchanged:

`Mempool`, `Relay evidence`, `Relay recovery`, `Mempool evidence`,
`Relay local submission`, `Relay fanout`, `Relay serving`,
`Rebroadcast: deferred`, `Public relay`.

`Recovery` (Phase 135/137 mempool recovery counts) is distinct from
`Relay recovery` (Phase 108). `Retry` is distinct from
`rebroadcast_deferred_count`. `Eviction (relay)` is distinct from
`Pressure removals`.

### Package CLI originating copy

| Mode | Heading | Required disclaimer |
| --- | --- | --- |
| dry-run | `Package dry-run` | `Dry-run does not change mempool, relay, persistence, or evidence state.` |
| submit | `Package submit` | `Local admission only. This is not public or default relay and not network-wide propagation.` |

Dual-state on the typed originating report uses exactly these tokens:

- Admission: `accepted`, `still-present`, `cleared`
- Relay/fanout: `eligible`, `queued`, `attempted`, `emitted`, `requested`, `served`, `suppressed`, `relay_disabled`

A relay-disabled accept must be showable as `accepted` + `still-present` and
`relay_disabled` on the typed report and as aggregate counts on shared surfaces.

### Forbidden copy

Never use on dashboard, human/JSON status, metrics, logs, support, help, or
extension success text:

`propagated`, `broadcast`, `broadcasted`, `public_relay`, `public relay ready`,
`guaranteed delivery`, `guaranteed propagation`, `production relay`,
`production node ready`, `mempoolminfee`, or Knots `bytes` as a dashboard label.

`EligibleServe` must not be labeled as a clear, a delivery, or a public result.

---

## Data Presentation Contract

| Evidence type | Presentation |
| --- | --- |
| Resources | Three separate rows/lines. Virtual size ≠ accounted usage ≠ accounted capacity. |
| Fee floors | Four separate rows/lines. Effective admission is `max(static, rolling)` and must not be labeled `mempoolminfee`. Incremental stays distinct. |
| Pressure | Pressure-removal count plus fixed decay label. No wall-clock ETA. |
| Eviction | Phase 105 `evicted_count` only. Do not increment it from pressure trims or retry. |
| Checkpoint | Freshness, overdue, persistence strength, age, loss bound, dirty generation. Fixed labels from Phase 135 evidence. |
| Recovery | Aggregate drop/recovered counts only. Never serialize `MempoolRecoveryRecord` or a txid. |
| Retry | New `mempool.retry.*` counts. Do not fold into `rebroadcast_deferred_count`. |
| Dual-state aggregates | `Admission states` and `Relay states` counts only. No per-member table. |
| Identifiers | Allowed only on originating `testmempoolaccept`, `submitpackage`, `sendrawtransaction`, and `openbitcoinpackage` / `open-bitcoin package` responses. Forbidden on snapshot, dashboard, status, metrics, logs, and support. |
| JSON machine names | `mempool.resources.{virtual_size, accounted_usage, accounted_capacity, transaction_count}`; `mempool.fee_floors.{static_relay_floor, rolling_mempool_floor, effective_admission_floor, incremental_relay_fee}`; `mempool.pressure.*`; `mempool.eviction.*`; `mempool.checkpoint.*`; `mempool.recovery.*`; `mempool.retry.*`; `mempool.admission.*`. `mempool.relay` stays the Phase 105/107/108 contract. |
| Support redaction | Counts need no redaction. Free-text reasons sanitize to `redacted_relay_mempool_evidence`. Recursive redaction still drops 64-hex strings. |

---

## Interaction And State Contract

| Interaction | Contract |
| --- | --- |
| Dashboard keys | No new keys. Keep `r` refresh, `s` status, service keys, `h`/`?` help, `q`/`Esc`/`Ctrl-C` quit, `y`/`n` for existing service confirmations. |
| Refresh | `r` and the existing tick (`tick_ms.max(250)`) rebuild `DashboardState::from_snapshot` from the shared collector. Renderers must not summarize locally. |
| Package actions | CLI only (`open-bitcoin package dry-run\|submit` and baseline `open-bitcoin-cli` method names). Not dashboard buttons. |
| Dry-run | Non-mutating. Dashboard/status counters must not change because a dry-run ran. |
| Submit | May change admission/retry aggregates. Must not present success as broadcast. |
| Confirmation | None for dry-run, submit, status, or support. Service lifecycle confirmations unchanged. |
| JSON vs human | Same snapshot. Human uses title-case labels; JSON uses snake_case groups. |
| Identifier leak | Status RPC is authenticated and still **not** an originating package response. No last-package echo. |

---

## Component Inventory

| Piece | Action |
| --- | --- |
| `DashboardState::from_snapshot` | Consume new `mempool` groups. Do not add a parallel DTO. |
| `mempool_and_wallet_rows` | Insert the locked rows after `Mempool` and before `Relay evidence`. Keep wallet and `block_relay_rows` after. |
| `render_human_status` / `relay_evidence_lines` | Insert matching human lines; keep existing relay lines. |
| `openbitcoinnetworkstatus.mempool` | Publish the seven groups so the collector stops discarding fee/resource fields. |
| `MetricKind` / structured logs | Add only fixed kinds/keys that match snapshot labels. Do not bind them to a ninth chart. |
| `support_status_for_bundle` | Redact any new free-text; leave counts. |
| Ratatui widgets | Reuse. No new widget types. |

---

## Registry Safety

| Registry | Blocks Used | Safety Gate |
| --- | --- | --- |
| shadcn official | none | not applicable — 2026-08-19. Not a React/Next.js/Vite surface; `components.json` is absent and must stay absent. |
| third-party registries | none | not applicable — 2026-08-19. No third-party registry declared; vetting gate not required. |
| terminal / Ratatui widgets | existing project code only | no external registry — 2026-08-19 |

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
| Copywriting | Labels, dual-state tokens, empty/error/support copy, and forbidden-word list match this contract. Knots aliases stay RPC-only. |
| Visuals | Rows only inside the existing Mempool and Wallet section; eight charts unchanged; no web chrome, cards, member tables, or marketing identity footer. |
| Color | Existing dark theme only; Cyan/Green reserved as listed; no color meaning for propagation. |
| Typography | Two weights (400/600); roles above; plain text remains complete without color. |
| Spacing | 4/8/16 scale tokens plus the listed exceptions; CLI lines stay single-spaced; chart strip stays 8 rows. |
| Registry Safety | Tool none; no shadcn; no third-party blocks. |
