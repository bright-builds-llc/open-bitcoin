---
phase: "162"
slug: "real-prune-retention-and-integrated-parity-proof"
status: approved
reviewed_at: "2026-10-11T16:32:42Z"
shadcn_initialized: false
preset: none
created: "2026-10-11"
generated_by: gsd-ui-researcher
lifecycle_mode: yolo
phase_lifecycle_id: 162-2026-10-11T16-25-37
---

# Phase 162 — Preserved Operator Presentation Contract

> Backend integrated-proof phase. Preserve the approved Phase 161 terminal, CLI, RPC and support contract while documenting reproducible prune/reopen evidence. No new presentation surface is authorized.

## Scope and Sources

Phase 162 owns CFPR-02, CFGR-01 and CFGR-02. ROADMAP explains that the UI hint also matches protocol header terminology; only Phase 161 changes the terminal dashboard. Context D-12 explicitly reuses that presentation and excludes new screens, navigation, GUI and formatter schema. D-01–D-11 determine evidence truth; D-13–D-15 determine verification and claim boundaries.

The approved [Phase 161 UI contract](../161-shared-operator-index-and-retention-evidence/161-UI-SPEC.md) remains the visual and interaction source of truth. Existing implementation was scouted in `packages/open-bitcoin-cli/src/operator/status/render/basic_index.rs`, `dashboard/model/basic_index_section.rs`, `dashboard/app/tests/basic_index.rs` and `support/render/basic_index.rs`.

Material guidance: `AGENTS.md`, `AGENTS.bright-builds.md`, placeholder-only `standards-overrides.md`, `standards/index.md`, frontend-ui, operability and verification standards. Both active lesson files were read fully: 7,188 bytes and 2,397 conservative estimated tokens. Existing audit baseline has no new trigger. No project skills or frontend design-system configuration were found. Phase 162 RESEARCH was not available during this scout; no stack choice depends on it. Decisions below are inherited, with no new user preferences or visual defaults introduced.

## Design System

| Property | Value |
| --- | --- |
| Tool | none; existing Rust terminal renderers |
| Preset | Not applicable |
| Component library | Existing Ratatui widgets and Crossterm adapter |
| Icon library | none; existing text labels and borders |
| Font | User-selected terminal monospace font and size |
| Initialization gate | Not applicable: no React, Next.js, Vite or browser frontend |
| New components | none |

Preserve the sole BASIC projection in `OpenBitcoinStatusSnapshot` and the shared `basic_index_entries` formatter. Rendering reads captured facts without additional RPC queries, scans, store reopens, recovery, index activation, pruning or networking. No output schema or status-collection authority is added.

## Spacing Scale

New pixel spacing tokens are not applicable to this phase or terminal cells. Preserve Phase 161's `Label: value` rows, one-row interval, existing section separation and borders. Preserve title/status/charts/actions bands, 35%/35%/30% status columns, eight-chart maximum and 25-row safety threshold. D-12 does not authorize layout work to satisfy a browser spacing scale.

## Typography

New pixel font sizes and line-height ratios are not applicable. Preserve the inherited terminal size, one terminal row per value, normal body/label text and bold existing headings/title. These are two emphasis styles with one terminal font size; do not invent three-to-four pixel sizes or change terminal settings.

## Color

New palette selection and enforced 60/30/10 pixel-area allocation are not applicable. Preserve Phase 161's approximate emphasis: 60% terminal-default background, 30% default/gray labels and borders, at most 10% existing accents. Cyan remains reserved for existing title/section headings; green remains reserved for existing sparklines. Existing yellow warnings and service-action styles remain unchanged. No new destructive color or theme is introduced.

Every state remains understandable through text in monochrome/plain output. Capability, earned initial synchronization, current lag, retained availability and safe durable progress must not be collapsed into a colored health indicator.

## Preserved Components and State Semantics

| Existing consumer | Required preservation |
| --- | --- |
| Human CLI status | `BASIC index` and existing shared rows before Prune; complete fixed guidance and reasons |
| Terminal dashboard | Existing BASIC section first in the right column, existing compaction and fallback, keys and confirmations |
| Status RPC / JSON | Existing typed fields and availability; no additional formatter schema |
| Authenticated `getblockfilter` | Existing BASIC result shape, error codes/order and retained hash lookup |
| Authenticated `getindexinfo` | Existing index name, `synced`, `best_block_height`, exact selection and empty-object absence |
| Support Markdown / JSON | Same captured and redacted snapshot; shared safe categories and full copy |

Keep `State`, `Init sync`, `Lag`, `Processed`, `Durable`, `Failure` and `Retained` distinct. Preserve the remaining accounting, configuration, peer scope, target, protection, basis and guidance rows. `none`, `none seen`, zero and `Unavailable: {reason}` retain different meanings. Missing history, backend/integrity failures and incomplete measurements never become successful empty results or fabricated zero totals.

Preserve logical retained immutable key/value bytes, filter payload subset and unique record counts, including stale branches. Exclude Fjall overhead and physical allocation; filter payload bytes are not added again to index bytes. Initial synchronization remains an earned latch independent of subsequent lag. Processed work never upgrades the coins-fenced safe durable endpoint. Modeled implicit peer origins remain explicitly unsupported; capability and effective serving permissions remain separate.

## Copywriting Contract

| Element | Exact inherited copy / rule |
| --- | --- |
| Primary CTA | Existing `r refresh`; no new CTA or control |
| Section heading | `BASIC index` |
| Empty prefix | `Processed: none`, `Durable: none`; `No BASIC prefix observed. Check index configuration and required history.` |
| Empty retained evidence | `Retained: none seen` only for observed absence |
| Missing body | `Required block body is missing. Inspect history evidence; no automatic repair or download.` |
| Missing undo | `Required undo is missing. Inspect history evidence; no automatic repair or download.` |
| Backend failure | `Index backend failed. Inspect bounded support evidence; read safe durable progress separately.` |
| Integrity failure | `Index integrity check failed. Inspect bounded support evidence; do not treat failed lookup as absence.` |
| Unavailable | `Unavailable: {allowlisted bounded reason}`; never interpolate raw backend errors |
| Retention note | `Filter growth is separate from the soft block/undo target; protection can retain more history.` |
| Body/wallet note | `Retained filters do not restore deleted bodies or wallet eligibility.` |
| Destructive confirmation | No new action or confirmation; existing service/prune contracts remain unchanged |

All other copy remains defined by Phase 161. New documentation must distinguish validated continuous history from codec-only storage-pressure rows, exact retained responses from transport plans, and scoped completion from milestone shipment. State normal 100-block maturity, continuous history above 1,000, a positive checkpoint interval and the legal 550 MiB automatic target where describing the integrated fixture. Report actual bounded space/runtime measurements with their command and environment; do not invent estimates or guarantees.

## Accessibility and Narrow Rendering

Preserve actual renderer coverage at 160x55, 80x30, 80x25, 80x24 and 40x30. Normal size retains all nineteen BASIC facts; 80-column narrow rendering retains the heading and seven critical label/value rows, including complete short failure categories and `Unavailable`. Full plain text retains complete reasons and guidance. Lower-priority overflow continues through existing compaction/text fallback, without a new screen or compact-mode API.

Below 25 rows preserve `Window too small`, `Interactive dashboard unavailable`, required/current height text and `Resize the terminal to continue. Press q, Esc, or Ctrl-C to quit.` Preserve quit behavior, blocked destructive actions and nonoverlapping Actions. No color-only meaning, new custom glyphs, font dependencies or key bindings.

Support evidence continues to exclude raw scripts, undo/filter bytes, transaction/block/peer identifiers, addresses, credentials, storage keys, filesystem paths and raw lock names from BASIC facts. Preserve fixed categories, booleans, heights and bounded aggregate counts; support export does not mutate or recollect its captured snapshot.

## UAT and Evidence Presentation

Give complete copy-pasteable repository-local commands, not an installed alias or a reference to an unspecified equivalent. Existing status inspection commands are:

```bash
bun run scripts/command-timings.ts run --key phase162-uat-cargo-status -- cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- status
bun run scripts/command-timings.ts run --key phase162-uat-bazel-status -- bazel run //packages/open-bitcoin-cli:open_bitcoin -- status
```

Status inspection alone does not prove CFPR-02 or CFGR-02. The implementation plan must supply exact commands for its final integrated test/fixture entrypoints, explicit temporary datadir setup/cleanup and measured runtime/space bounds. Do not publish invented future target names. Serialize ad-hoc Cargo/Bazel jobs through the timing wrapper. Run the full default native contract for phase completion:

```bash
bash scripts/verify.sh
```

UAT documentation pairs each command with an observable assertion: actual paired Fjall body/undo absence after reopen; unchanged durable coins/checkpoint identity; exact retained BASIC bytes/hash/header including displaced records; authenticated RPC replies and all three peer families through both socket directions; a nonempty positive-height checkpoint; complete achieved writes versus partial-write refusal; continued old-body and wallet restrictions. Reuse one identifiable validated history and show every handoff. Synthetic formatting fixtures cannot establish that lifecycle.

Fault evidence must report missing body/undo, pending prune intent, cursor ahead of recovered chainstate, index/protection/checkpoint write failures, corrupted records and partial writes through production effect boundaries. Preserve valid prefixes and fail-closed durable facts. Report limitations beside claims, with pinned source/parity evidence links and deterministic negative claim checks; historical reports retain their original boundary.

Preserve exclusions for BIP37/V0, assumeutxo/dual chainstate, archive scale, public defaults/public-network CI, implicit repair/import/download, production service/readiness/funds and unrelated v2.4 advisory cleanup. No status read, test or phase-completion sentence authorizes these capabilities or ships the milestone.

## Registry Safety

| Registry | Blocks Used | Safety Gate |
| --- | --- | --- |
| none | none | Not applicable — no registry, remote component or dependency introduced |

## Checker Sign-Off

- [ ] Dimension 1 Copywriting: pending — exact inherited copy and scoped documentation claims
- [ ] Dimension 2 Visuals: pending — no new visual design; preserve terminal/narrow rendering
- [ ] Dimension 3 Color: pending — inherited terminal palette; new palette not applicable
- [ ] Dimension 4 Typography: pending — inherited terminal font/emphasis; pixel sizes not applicable
- [ ] Dimension 5 Spacing: pending — inherited terminal cell layout; pixel scale not applicable
- [ ] Dimension 6 Registry Safety: pending — registry adoption/vetting not applicable

**Approval:** pending independent checker. This contract adds no source behavior; phase execution owns simplification, source/mitigation review, full native verification, formal lifecycle verification and the deferred clean commit/push gate.
