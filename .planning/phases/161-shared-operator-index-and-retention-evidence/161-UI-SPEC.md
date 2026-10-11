---
phase: "161"
slug: "shared-operator-index-and-retention-evidence"
status: approved
reviewed_at: "2026-10-11T04:59:03Z"
shadcn_initialized: false
preset: none
created: "2026-10-11"
generated_by: gsd-ui-researcher
lifecycle_mode: yolo
phase_lifecycle_id: 161-2026-10-11T04-48-29
---

# Phase 161 — Terminal UI Design Contract

> CFOP-01 contract for the existing Rust terminal dashboard, human status and redacted support evidence. Generated in yolo mode from locked decisions D-01–D-16; independently approved across all six dimensions.

## Design System

| Property | Value |
| --- | --- |
| Tool | none; reuse existing Rust terminal renderers |
| Preset | Not applicable |
| Component library | Existing Ratatui widgets and Crossterm terminal adapter |
| Icon library | none; text labels and existing borders |
| Font | User-selected terminal monospace font, inherited size |
| Platform | Terminal cells; deterministic plain text and existing JSON output |
| Initialization gate | Not applicable: Rust terminal application, no React/Next.js/Vite frontend or shadcn installation |

Material guidance: `AGENTS.md`, `AGENTS.bright-builds.md`, placeholder-only `standards-overrides.md`, `standards/index.md`, architecture, frontend-ui, operability, testing, verification and Rust standards. The headless project boundary and D-13 determine terminal defaults; web-only identity, pixel typography and registry rules do not introduce browser chrome. Both active lesson files were read completely: 7,188 bytes, 2,397 conservative estimated tokens, with an existing audit baseline and no new audit trigger. No project skills were found in `.claude/skills/` or `.agents/skills/`.

Use the sole authoritative BASIC projection carried by `OpenBitcoinStatusSnapshot`. Rendering is pure: no additional RPC query, index scan, store reopen, activation, checkpoint, recovery, prune-lock change or network action. Preserve `getindexinfo` and `getblockfilter` result shapes. Rust type and JSON field names belong to technical research; the human labels and semantic distinctions below are the contract.

Sources: Phase 161 context D-01–D-16, ROADMAP Phase 161 success criteria, REQUIREMENTS CFOP-01; existing CLI `status.rs`, `status/render.rs`, `status/render/prune.rs`, dashboard `model.rs`, `model/prune_section.rs`, `app.rs`, `mod.rs`, support `render.rs`, `render/prune.rs`, `redaction.rs`, and `docs/architecture/status-snapshot.md`. Phase 144 and Phase 150 terminal/privacy conventions and Phase 153 logical accounting are inherited. No RESEARCH.md was present at initial scout; the parent supplied accounting and stopped-configuration research findings during drafting, incorporated below.

## Spacing Scale

Pixel tokens are not applicable to Ratatui. Do not introduce an artificial 4/8/16px scale, terminal font scaling or blank-line padding to imitate a browser. This is an explicit terminal adaptation of the template's multiples-of-four pixel rule.

| Terminal token | Value | Usage |
| --- | --- | --- |
| label separator | `: `, two cells | Existing `Label: value` rows |
| row interval | One terminal row | One label and one single-line value |
| section interval | One heading row | Cyan/bold section heading; no extra blank padding in interactive lists |
| text section interval | One blank line | Existing sectioned dashboard and support text |
| border | One cell | Existing `Borders::ALL` widgets |

Preserve the four vertical bands: title 3 rows, status content minimum 10 rows, charts 8 rows, actions 4 rows. Preserve status columns at 35% / 35% / 30% and the eight-chart maximum. BASIC uses rows in the status band, with no new panel, chart family, screen, navigation or control. Keep the 25-row interactive safety threshold and existing smaller-window blocker.

## Typography

| Role | Size | Weight | Line height |
| --- | --- | --- | --- |
| Body/value | Inherited terminal cell | Normal | One row |
| Label | Inherited terminal cell | Normal | One row |
| Heading | Inherited terminal cell | Bold | One row |
| Display/title | Inherited terminal cell | Bold | One row |

Exactly two emphasis styles, normal and bold; all four semantic roles use the one terminal font size. The template's three-to-four pixel sizes do not apply to this platform. Retain the existing gray label plus raw/default value spans. No italics, custom glyphs, icons, uppercase alarm banners or new font dependencies. Heights, counts and bytes use decimal integers with explicit units where needed; no rounded-only values or progress percentages inferred from heights.

## Color

| Role | Value | Usage |
| --- | --- | --- |
| Dominant, approximately 60% | Terminal default background | Existing background and body surfaces |
| Secondary, approximately 30% | Terminal default foreground, `Color::Gray` / `Color::DarkGray` | Values, labels, borders and existing action separators |
| Accent, at most 10% | Existing `Color::Cyan` and `Color::Green` | Cyan/bold dashboard title and section headings; green existing sparklines |
| Warning | Existing `Color::Yellow` | Existing destructive service-action labels and too-small-window warning |
| Destructive | No new color token | No new destructive BASIC action |

The 60/30/10 split describes visual emphasis, not enforced pixel area or hard-coded RGB colors. Inherit terminal theme and existing styles; do not force a background. Accent is reserved for existing title/section headings and sparklines. BASIC facts remain legible through text alone, including monochrome/plain output. Never color `all` permission, configured capability, earned initial sync or zero lag as proof that an index is healthy or durable.

## Layout and Component Inventory

| Existing component | Phase 161 contract |
| --- | --- |
| Human status | Insert a `BASIC index` heading and the shared BASIC rows immediately before existing Prune rows. Keep the rest of the existing status order. |
| Dashboard model/text | Add a `BASIC index` section. Keep Node, Sync and Peers, Mempool and Wallet, and Service ordering; within the right-column group order BASIC index, Logs and Health, then Prune. |
| Interactive right column | Render BASIC index first. Its state, latch, lag, endpoints, failure and retained availability precede lower-priority explanatory rows. Do not append BASIC below the current Prune section where it would be silently clipped. |
| Existing Prune section | Preserve configured mode, prune-height, automatic target, locks and manual-prune outcome semantics. A BASIC protection summary is independent of raw named-lock rows. |
| Support Markdown | Add `## BASIC index` immediately before `## Prune`; use the shared labels, row order and safe value formatter on the captured redacted snapshot. |
| JSON outputs | Serialize the authoritative typed section with field availability and backward-compatible defaults. Do not create a renderer-owned status model or stringify unavailable fields into successful facts. |
| Charts/actions | Preserve existing chart selection and `MAX_DASHBOARD_CHARTS = 8`, key bindings, action ordering, confirmation behavior and refresh cadence (`tick_ms.max(250)`). |

### BASIC row order and exact labels

All full text outputs use this order. Labels are intentionally short so critical facts fit the existing narrow right column. Section context makes `State`, `Lag` and endpoints BASIC-specific. Every row remains present; unavailable data gets explicit text rather than row omission.

| Order | Label | Value contract |
| --- | --- | --- |
| 1 | State | Owner-projected fixed state: `disabled`, `catching up`, `current`, `lagged`, `paused`, `failed` or `invalidated`; otherwise `Unavailable: {reason}`. No renderer derives state from endpoint equality. |
| 2 | Init sync | `earned` or `not earned`, from the owner's initial-synchronization latch; otherwise unavailable. Later lag/reorg does not reset `earned`. |
| 3 | Lag | `{n} blocks` against the accepted active target; otherwise unavailable. No header-height substitution. |
| 4 | Processed | `{height}` for an actual processed endpoint; `none` for an observed empty prefix; otherwise unavailable. |
| 5 | Durable | `{height}` for the safe durable endpoint; `none` for an observed empty durable prefix; otherwise unavailable. |
| 6 | Failure | `none`, `missing body`, `missing undo`, `backend`, `integrity`, `resource`, `invalidated`, or unavailable. `resource` means resource-bound pause. Map only categories supplied by the owner. |
| 7 | Retained | `observed` or `none seen`, only from actual retained-filter availability evidence; otherwise unavailable. This is independent of index state and measurement availability. |
| 8 | Index bytes | `{n} B` complete logical encoded immutable record key/value bytes, including envelope and key; otherwise unavailable. |
| 9 | Filter bytes | `{n} B` complete encoded BASIC filter payload bytes, a separately named subset of index bytes; otherwise unavailable. Do not add it to index bytes. |
| 10 | Records | `{n}` complete unique retained BASIC record count, including displaced records; otherwise unavailable. |
| 11 | Indexing | Configured BASIC indexing `true` or `false`; otherwise unavailable. |
| 12 | Peer serving | Configured general BASIC peer serving `true` or `false`; otherwise unavailable. |
| 13 | Peer scope | Fixed aggregate modeled permission/capability counts for `general`, `explicit`, `all`, `implicit` and `offered`. Use bounded counts supplied by the authority, no peer identifiers or raw permission strings. The current model has no implicit-origin resolver/provenance, so implicit is zero; ordinary peers are not relabeled implicit. `all` may advertise with no backend. Full text must disclose the modeled-origin limit. Render unavailable facts explicitly. |
| 14 | Target | `{height}` for the accepted active target; `none` for an observed absent target; otherwise unavailable. |
| 15 | Protection | `none` or `heights {first}..{last}` for authoritative inclusive BASIC-owned bounds; otherwise unavailable. No raw lock name or invented physical bytes blocked/saved. |
| 16 | Basis | `immutable record keys/values; includes stale branches; excludes Fjall overhead` |
| 17 | Next action | Fixed, bounded guidance from the state/failure table below; otherwise unavailable. |
| 18 | Retention note | `Filter growth is separate from the soft block/undo target; protection can retain more history.` |
| 19 | Body/wallet note | `Retained filters do not restore deleted bodies or wallet eligibility.` |

`none` is an available absence, `none seen` is a bounded observed availability result, and `Unavailable: {reason}` means the fact was not obtained. They are never interchangeable. A no-genesis prefix is `none`, never an invented height `0`; genesis really processed/durable may display `0`.

Accounting basis is fixed by Phase 161 technical research: unique immutable record count, encoded filter payload bytes, and encoded immutable record key/value bytes. Key/value bytes include the record envelope and key. All include retained displaced/stale records. They exclude active projections, runtime state, owner/checkpoint/protection/lock metadata, Fjall engine overhead and physical filesystem allocation. These are logical retained-record growth facts, not a complete database footprint. The renderer uses the authoritative basis enum/constant, never guesses it. Incomplete/failed/over-budget measurement returns unavailable for each incomplete aggregate, not a plausible partial total; retries/reorg reuse do not double count immutable records. No storage migration is introduced for this evidence.

### Normal, narrow and overflow rendering

At 160x55 cells, the interactive BASIC section must show every semantic row above. Keep one label/value line per interactive row. At 80x30, the actual rendered buffer must visibly show `BASIC index`, `State`, `Init sync`, `Lag`, `Processed`, `Durable`, `Failure` and `Retained`, including their distinguishing short values. The first seven rows and heading take precedence over measurement, configuration and prose rows. Do not insert blank rows before them. `Init sync: Unavailable` is 22 cells, fitting the normal 80-column right-column interior; the longer `Initial sync` label would clip the availability word.

For the normal interactive display use these exact compact values for long explanatory rows: `Basis: immutable keys/values`, `Retention note: filters outside target`, `Body/wallet note: eligibility unchanged`. Full text keeps the complete copy in the table. `Next action` may show the fixed leading problem category, while full text carries its complete guidance. If peer scope contains several categories, an interactive compact summary may use `G`, `E`, `A`, `I` in that order for general/explicit/all/implicit; full text expands the same authoritative values. Do not replace unknown booleans with false or include raw permission strings. These are bounded presentation abbreviations, not additional data states or a new mode.

For unavailable values, the visible narrow value must retain `Unavailable`; full text retains `Unavailable: {reason}`. For failures, retain the entire fixed short category (especially `missing body` versus `missing undo`) before optional guidance. For safe durable/processed endpoints, retain their decimal heights. These tests use bounded representative heights and category fixtures; arbitrary unbounded text cannot be used as a reason to skip the core assertions.

The current List renderer does not wrap rows and fixed columns clip long values. Preserve the compact row pattern; do not add a new compact-mode API, screen or navigation. If lower-priority details overflow, show a bounded `Details: use status output` row when space permits; complete facts remain in existing human status, dashboard text/JSON and support output. This guidance is informational and adds no key binding. Never hide a failure behind only `...`, an omitted row or a cheerful summary. Existing `s` means service status, so do not advertise it as an index-detail action.

For heights below 25 rows retain the exact existing `Window too small` blocker, `Interactive dashboard unavailable`, required/current height text and `Resize the terminal to continue. Press q, Esc, or Ctrl-C to quit.` Destructive actions remain blocked. Narrow widths must not panic or overlap Actions with the status band. Text output remains the existing complete inspection fallback; no automatic alternate application or screen is introduced.

## Copywriting Contract

| Element | Copy |
| --- | --- |
| Primary CTA | Existing `r refresh`; no new BASIC CTA. Refresh reads facts only. |
| Section heading | `BASIC index` |
| Empty prefix | `Processed: none`, `Durable: none`; guidance `No BASIC prefix observed. Check index configuration and required history.` |
| Empty retained evidence | `Retained: none seen`, only when absence was successfully observed; measurements are zero only when actually measured. |
| Catch-up | `State: catching up`, `Init sync: not earned`; guidance `Initial catch-up is incomplete. Read lag and safe durable progress.` |
| Current | `State: current`; guidance `Read safe durable progress and retained availability separately.` |
| Later lag | `State: lagged`, `Init sync: earned`, `Lag: {n} blocks`; guidance `Initial sync was earned; current work remains behind the accepted target.` |
| Processed ahead | Show both `Processed: {higher}` and `Durable: {lower}`; guidance `Processed work exceeds the safe durable endpoint.` Only use when the authoritative projection provides this fact. |
| Disabled | `State: disabled`, `Indexing: false`; guidance `BASIC indexing is disabled. Capability and retained evidence are separate facts.` |
| Configured-only/stopped | Known config booleans remain visible; live facts use `Unavailable: live BASIC evidence unavailable`. Guidance `Check daemon status, then collect live BASIC evidence.` |
| Legacy missing fields | `Unavailable: BASIC evidence not reported`; do not change it to disabled/healthy/zero. |
| Collection/measurement unavailable | `Unavailable: {allowlisted bounded reason}`; guidance `Collect status again and inspect bounded support evidence.` No renderer-local retry loop. |
| Destructive confirmation | Not applicable: no BASIC mutation, prune, repair, reindex or download control. Existing service confirmations are unchanged. |

Stopped configuration is visible only when an authoritative input already establishes it. The existing stopped `OpenBitcoinConfig` JSONC resolver has no BASIC fields; do not infer disabled capability from their absence or add a separate configuration resolver as part of rendering. Runtime-resolved baseline config/CLI facts may be used where genuinely available. Otherwise `Indexing`, `Peer serving` and effective `Peer scope` remain unavailable, alongside the live facts.

Unavailable reasons are fixed safe labels, including `live BASIC evidence unavailable`, `BASIC evidence not reported`, `retention measurement incomplete`, `retention measurement failed` and `retention measurement budget exceeded`. Technical research may map existing availability categories to these phrases; it must preserve distinct causes and may not interpolate raw backend errors, keys, scripts or paths.

### Failure/pause guidance

| Failure value | Next action |
| --- | --- |
| `none` | `Read progress, retained availability and protection separately.` |
| `missing body` | `Required block body is missing. Inspect history evidence; no automatic repair or download.` |
| `missing undo` | `Required undo is missing. Inspect history evidence; no automatic repair or download.` |
| `backend` | `Index backend failed. Inspect bounded support evidence; read safe durable progress separately.` |
| `integrity` | `Index integrity check failed. Inspect bounded support evidence; do not treat failed lookup as absence.` |
| `resource` | `Index work is paused at a resource bound. Inspect bounded resource evidence before continuing.` |
| `invalidated` | `Index lifecycle changed. Collect current evidence; stale work cannot earn progress.` |

Do not suggest deleting protection, reconstructing scripts, downloading missing history, enabling public serving, or repairing/reindexing as an automatic next step. Status does none of these actions. Do not display `none` for a failed/unavailable failure collector.

### State coexistence and privacy

- A failed/paused/lagged index can still show `Retained: observed`; this does not promise every arbitrary lookup succeeds. Backend/integrity failure does not become successful empty results or healthy zero counts.
- Earned initial synchronization and current lag remain visible together. A processed endpoint above the durable endpoint does not upgrade the latter. Header height, RPC success and service advertisement cannot supply either fact.
- Stopped/configured-only collection preserves known configuration but reports live progress, failures, retained availability and growth as unavailable. Legacy missing fields also stay unavailable through serde defaults.
- The support JSON and Markdown BASIC section must agree with the same captured live snapshot for every safe enum, boolean, height, aggregate count and unavailable category. Support export operates on a copy and does not change the caller's snapshot or re-collect facts midway.
- New BASIC shared evidence excludes raw scripts, undo/filter bytes, transaction/block/peer identifiers, addresses, credentials, storage keys, filesystem paths and raw lock names. Only fixed categories, booleans, heights and aggregate counts cross the boundary. Existing support report destination metadata is outside the BASIC facts; do not copy those paths into the new section.
- Prune target copy must say soft logical block/undo target. Retained BASIC bytes are separate, and protection may keep block/undo retention above the target. No disk-capacity/physical-size guarantee or fabricated bytes-saved estimate.

## Registry Safety

| Registry | Blocks Used | Safety Gate |
| --- | --- | --- |
| none | none | Not applicable — existing Rust terminal widgets only; no registry code or dependencies introduced |

No browser design system, third-party registry, remote component block or new production dependency is part of this phase. There is no registry vetting assertion to fabricate.

## Render and Cross-Surface Verification Contract

Verification must check behavior through actual renderers, not only source-string presence. Synthetic snapshots can prove formatting; authoritative collection proof also requires existing genuine validated-chain/Fjall fixtures and production RPC/CLI/dashboard/support consumers (D-14).

1. Table-driven row/render tests cover disabled and configured-only, initial catch-up, current, earned-initial-sync plus later lag, processed ahead of durable, no-genesis/empty prefix and legacy missing fields. Assert exact labels/order, booleans, heights and reasons; zero and unavailable are distinct.
2. Failure tests separately cover missing body, missing undo, backend, integrity, resource bound and lifecycle invalidation where the owner provides them. A failed/lagged snapshot with retained filters must visibly retain both facts; incomplete/failed/over-budget accounting must not produce partial totals.
3. Ratatui `TestBackend` tests draw the real dashboard at 160x55 and 80x30. Assert meaningful BASIC labels and their adjacent values are present in visible cells, with initial sync/lag/processed/durable/failure/retained distinctions readable. Normal size also proves record/byte units, configured capability, protection and basis/notes are exposed. Assert Actions stays in its band, chart count never exceeds eight, and the four bands/column split persist.
4. Draw at 80x25 and 80x24, plus a very narrow 40x30 safety case. At 25 rows the critical BASIC order is preserved without panic/overlap; at 24 rows the existing blocker and quit behavior remain. At 40 columns require safe bounded rendering and complete plain-text fallback, not a misleading clipped success claim. Existing destructive-action confirmation and blocked-window tests continue to pass.
5. Human status, dashboard text and support Markdown use the shared BASIC formatter, with identical labels, safe semantic values and full unavailable reasons. JSON preserves typed availability. Compare the same captured fixture through all callers; do not compare snapshots collected at different times as proof of consistency.
6. Support redaction tests inject private marker strings into all new free-text boundaries and confirm neither JSON nor Markdown contains them, that safe heights/counts/categories survive, and that the original snapshot is unchanged. Preserve existing redaction tests and raw-lock removal.
7. Retained displaced/pruned filter records, protected retention, reorg/reopen and immutable retry fixtures verify measurement truth independently of layout. Status polling is bounded and read-only; rendering does not scan history, clear faults or change protection. The full integrated post-prune client proof remains Phase 162.

Use explicit Arrange/Act/Assert in nontrivial unit tests, with one concern per test. Keep unit projections pure and use production boundaries for integration evidence. Parent phase execution owns source review, simplification, `bash scripts/verify.sh`, lifecycle verification and the clean commit gate. This contract-only task adds no source/Cargo changes and creates no commit.

When showing UAT instructions, use repository-local commands, for example:

```bash
cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- status
bazel run //packages/open-bitcoin-cli:open_bitcoin -- status
```

Execute ad-hoc Cargo/Bazel checks through `bun run scripts/command-timings.ts run --key <stable-key> -- <command>`; do not overlap Cargo jobs. Terminal evidence does not upgrade v2.5 readiness, v2.4 crash-window guarantees or excluded wallet/public-network capabilities.

## Checker Sign-Off

- [x] Dimension 1 Copywriting: PASS
- [x] Dimension 2 Visuals: PASS — terminal scope and real normal/narrow rendering
- [x] Dimension 3 Color: PASS — inherited ANSI theme, text conveys state
- [x] Dimension 4 Typography: PASS — terminal font exception, two emphasis styles
- [x] Dimension 5 Spacing: PASS — cell-based exception, existing layout preserved
- [x] Dimension 6 Registry Safety: PASS — not applicable, no registry/dependency

**Approval:** independent gsd-ui-checker review passed all six dimensions with no recommendations. At 80×25, eight interior rows fit the heading plus seven critical facts; the 22-cell right-column interior fits compact unavailable/failure/height examples. At 160×55, 38 interior rows fit all 19 BASIC rows. Actual TestBackend implementation evidence remains required. Accounting basis is bound to the technical research seam above.

**Scoped copy reconciliation:** Plan 10 proves only modeled peer provenance (implicit origins are not modeled); Plan 11 adds genuine backend read faults alongside persistence failures. Peer-scope and backend guidance above were made precise for those implemented producers without changing the approved layout, compact labels, controls or state distinctions.
