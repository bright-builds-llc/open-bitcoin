---
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: 161-2026-10-11T04-48-29
generated_at: 2026-10-11T04:55:00Z
---

# Phase 161: Shared Operator Index and Retention Evidence - Context

**Gathered:** 2026-10-10 CDT
**Status:** Ready for research and planning
**Mode:** Yolo — one recommendation pass, all relevant areas selected.

Material guidance: `AGENTS.md`, `AGENTS.bright-builds.md`, placeholder-only `standards-overrides.md`, architecture, code-shape, testing, verification, local-guidance, Rust and terminal-relevant frontend standards. Both active lesson inputs were read completely (7,188 bytes; 2,397 conservative tokens), with no audit trigger. Clean main was synchronized with origin/main and the pinned submodule materialized before edits. The strict outer wrapper defers all nested workflow commits until clean phase verification; hooks remain mandatory.

<domain>
## Phase Boundary

Own CFOP-01: present configured BASIC capability, initial catch-up, current lag, safe durable progress, distinct missing-history/failure/availability facts and separately measured retained filter growth through the existing status RPC, repo-local CLI, terminal dashboard and redacted support evidence. Reuse the single active durable authority and existing operator surfaces. Phase 162 owns the complete integrated prune/reopen client proof and milestone closure.
</domain>

<decisions>
## Implementation Decisions

### One shared status contract

- **D-01:** Extend the existing `OpenBitcoinStatusSnapshot` and authoritative network operator snapshot with a typed BASIC section. RPC, CLI, dashboard and support must consume that projection, never independently guess state from config, service bits, RPC success or height equality. Keep existing `getindexinfo` and `getblockfilter` pinned result shapes unchanged.
- **D-02:** Show separately configured BASIC indexing, configured general peer serving and effective per-peer capability scope. Preserve Phase 160's explicit/all/implicit permission distinction; do not infer general service from one permissioned peer or infer a working index from `all` capability advertisement.
- **D-03:** Expose initial synchronization as the existing owner's latch, current lag against its accepted active target, processed endpoint and safe durable endpoint as distinct facts. Include the no-genesis/empty-prefix case without inventing height-zero completion. Later lag or reorg does not clear the earned initial-sync latch. Header height and processed rows cannot authorize durable progress.
- **D-04:** Preserve explicit availability through `FieldAvailability` and backward-compatible serde defaults. A stopped collector may report configured facts only when authoritatively known, while live progress, retention measurements and failures remain unavailable. The existing stopped JSONC config has no BASIC fields; do not infer false from absent fields, add a second resolver or claim disk configuration captures unseen daemon CLI overrides. Missing fields from an older daemon remain unavailable rather than disabled, caught up, zero bytes or healthy. Collection is read-only and must not activate/recover/index/network as a status side effect.

### Failure and retained availability semantics

- **D-05:** Use fixed low-cardinality categories for missing required body, missing required undo, persistence/backend failure, index integrity/invalid facts, resource-bound pause and lifecycle invalidation where the existing owner supplies those distinctions. Retain actionable, bounded guidance; never serialize raw backend errors, scripts, storage keys, paths or peer identifiers into this shared evidence.
- **D-06:** Keep failure/pause and retained-filter availability independent. A failed or lagged index can retain previously indexed active/stale filter records; availability evidence must describe what was actually observed and must not promise every arbitrary lookup succeeds. Integrity/backend faults must never become ordinary absence, an empty successful filter or healthy zero counts.
- **D-07:** Read coherent progress/protection/failure facts from the existing serialized chainstate/index authority, with the existing integrity/lifecycle epoch. Status must not clear sticky faults, release protection, advance a checkpoint or recreate missing history. Required body/undo refusal remains fail closed and implies no automatic repair, reconstruction or download.
- **D-08:** Preserve privacy in live shared evidence and exported support: fixed enums, booleans, heights and aggregate counts only for new BASIC facts. Redacted support agrees with the captured live snapshot, does not mutate the caller's snapshot, and excludes raw scripts/undo/filter payloads, tx/block/peer identifiers, addresses, credentials, backend keys, filesystem paths and raw lock names.

### Separate retained-index growth and prune protection

- **D-09:** Report retained BASIC record count and logical encoded filter/index bytes separately from the existing soft logical block/undo prune target. State the precise measurement basis. Include retained displaced records rather than presenting only the active prefix as total retained growth; immutable retries and reorg reuse must not double count.
- **D-10:** Collection must have explicit finite work/allocation limits and avoid a full index/history scan on every status poll. Research chooses the simplest recoverable accounting seam in existing publication/recovery authority; cached or persisted aggregates require tested freshness/error/reopen behavior. Incomplete, failed or over-budget measurements remain explicitly unavailable, never plausible partial totals. No new independent database, production crate, dependency or unbounded cache.
- **D-11:** Expose index-owned protection and its retention implication separately from growth and target accounting. Explain that protection can keep block/undo bytes above a soft target. Do not invent counterfactual physical bytes saved/blocked; show only authoritative protection bounds or measured retained facts. Preserve reserved ownership and existing prune semantics.
- **D-12:** Useful retained filters do not restore deleted block bodies, ordinary body-serving eligibility or wallet eligibility. Render this distinction in concise operator copy and documentation. The target is not a filesystem-size/disk-capacity guarantee and filter growth is not charged to it.

### Existing terminal consumers and evidence

- **D-13:** Reuse compact label/value rows and the existing terminal layout, styling, refresh cadence and explicit `Unavailable: {reason}` convention. Add a readable BASIC/index section without a new screen, new navigation, hosted frontend, chart family or controls. Preserve the existing four-band/eight-chart maximum. Generate a terminal-scoped UI contract and validate normal and narrow rendering behavior.
- **D-14:** Prove consumers agree for disabled/configured-only, initial catch-up, caught-up, earned-initial-sync plus later lag, processed-ahead-of-durable, missing body/undo, persistence/integrity/resource failure, retained stale/pruned records, protected retention, stopped collection and legacy missing fields. Use existing genuine validated-chain/Fjall fixtures and real RPC/CLI/support/dashboard callers; synthetic snapshots alone are insufficient for authoritative collection proof.
- **D-15:** Update relevant README/operator/status/parity docs and Rust parity breadcrumbs, add a narrowly scoped deterministic native claim/mutation checker where established patterns require it, and preserve historical guards. Provide copy-pasteable repo-local Cargo/Bazel UAT commands. Full `bash scripts/verify.sh`, independent source/security review, a simplification pass and lifecycle-valid formal verification gate commit/push.
- **D-16:** Preserve Phase 162 ownership and v2.5 pending claims. No BIP37/V0, filter import, wallet acceleration, reindex/repair/download, archive-scale, assumeutxo, public defaults/network CI, production service/readiness or funds-safety expansion. Status/accounting evidence does not strengthen existing v2.4 crash-window guarantees.

### Agent Discretion

Research and planning choose exact Rust enum/schema names, compact copy, bounded accounting mechanics, module seams and plan decomposition. Respect existing external schemas and established crate boundaries; prefer the simplest robust projection. No pending todos matched this phase.
</decisions>

<canonical-refs>
## Canonical References

Downstream agents MUST read these before planning or implementing.

### Scope and inherited decisions

- `.planning/ROADMAP.md` — Phase 161's three success criteria, Phase 162 boundary and dependency gates.
- `.planning/REQUIREMENTS.md` — CFOP-01 and excluded surfaces.
- `.planning/PROJECT.md` — single active durable authority and claim boundaries.
- `.planning/phases/144-operator-flush-and-availability-evidence/144-CONTEXT.md` — shared status, privacy and terminal layout decisions.
- `.planning/phases/150-operator-prune-surfaces-and-evidence/150-CONTEXT.md` — prune operator/support conventions.
- `.planning/phases/153-automatic-prune-retention-integration/153-CONTEXT.md` — logical soft-target accounting.
- `.planning/phases/155-recoverable-index-and-pre-prune-startup-protection/155-CONTEXT.md` — recovery and safe durable fences.
- `.planning/phases/156-index-owned-manual-and-automatic-prune-coordination/156-CONTEXT.md` — reserved protection and ownership transitions.
- `.planning/phases/157-safe-activation-and-scheduled-index-catch-up/157-CONTEXT.md` — progress, initial-sync and missing-history semantics.
- `.planning/phases/158-validated-reorg-and-retained-branch-identity/158-CONTEXT.md` — immutable displaced records and current active branch.
- `.planning/phases/159-authenticated-basic-filter-and-index-rpcs/159-CONTEXT.md` — exact RPC shapes and initial-sync latch.
- `.planning/phases/160-explicit-bounded-peer-filter-serving/160-CONTEXT.md` — capability distinctions and real serving evidence.

### Existing status/operator authority

- `packages/open-bitcoin-node/src/network/operator_snapshot.rs`, `packages/open-bitcoin-node/src/network/types.rs` — authoritative shared capture.
- `packages/open-bitcoin-node/src/chainstate/filter_index.rs`, `packages/open-bitcoin-chainstate/src/filter_index/catch_up.rs` — owner failure and progress facts.
- `packages/open-bitcoin-node/src/network/runtime_authority/filter_index/query.rs` — integrity-preserving query projection.
- `packages/open-bitcoin-node/src/storage/fjall_store/filters/` — immutable publication/recovery and retained records.
- `packages/open-bitcoin-node/src/status.rs`, `packages/open-bitcoin-node/src/status/prune_operator.rs` — shared snapshot and explicit availability patterns.
- `packages/open-bitcoin-rpc/src/dispatch/node.rs`, `packages/open-bitcoin-rpc/src/method/node.rs` — status RPC route/schema.
- `packages/open-bitcoin-cli/src/operator/status.rs`, `packages/open-bitcoin-cli/src/operator/status/render.rs` — live/stopped collection and human output.
- `packages/open-bitcoin-cli/src/operator/dashboard/model.rs`, `packages/open-bitcoin-cli/src/operator/dashboard/app.rs` — existing terminal view model/rendering.
- `packages/open-bitcoin-cli/src/operator/support.rs`, `packages/open-bitcoin-cli/src/operator/support/redaction.rs` — captured support evidence and privacy.

### Behavioral and documentation anchors

- `packages/bitcoin-knots/src/index/base.cpp`, `packages/bitcoin-knots/src/index/blockfilterindex.cpp` — pinned synchronization, checkpoint and retained index semantics.
- `packages/bitcoin-knots/src/rpc/blockchain.cpp`, `packages/bitcoin-knots/src/rpc/node.cpp`, `packages/bitcoin-knots/src/node/blockstorage.cpp` — pinned RPC/protection semantics; inspect actual definition locations.
- `docs/architecture/status-snapshot.md`, `docs/architecture/operator-observability.md`, `docs/operator/runtime-guide.md` — contributor/operator vocabulary.
- `docs/parity/catalog/basic-compact-filters.md`, `docs/parity/index.json`, `docs/parity/source-breadcrumbs.json` — scoped claims and breadcrumbs.
- `AGENTS.md`, `AGENTS.bright-builds.md`, `standards-overrides.md`, `standards/index.md`, `scripts/verify.sh` — material local guidance and complete verification contract.
- `https://github.com/bitcoin/bips/blob/master/bip-0157.mediawiki` — primary protocol background; pinned local Knots remains the behavioral authority.
</canonical-refs>

<code-context>
## Existing Code Insights

### Reusable Assets

`BasicIndexProgress` already distinguishes `initially_synchronized()`, `current_lag()`, `accepted_target()`, processed/safe-durable endpoints, protection and state. `AcceptedBasicIndexOwner` holds typed failure evidence. `FieldAvailability`, the network operator snapshot, status schema, dashboard view models and recursive support redaction supply the consumer seams.

### Established Patterns

Capture once under the existing authority, project pure data into all views, preserve unavailable facts, keep status read-only, earn durability from genuine fences, and isolate local logical accounting from physical disk promises.

### Integration Points

`operator_snapshot` feeds `authoritative_operator_snapshot`, `openbitcoinnetworkstatus` and CLI collection, which builds `OpenBitcoinStatusSnapshot` for dashboard/support. No BASIC section currently traverses this chain. Additive serde defaults are required at both network/status schemas. Existing configured-only collection and support redaction must be extended explicitly.
</code-context>

<specifics>
## Specific Ideas

Compare the same captured facts through live RPC, CLI JSON/human lines, dashboard rows and support JSON/text. A snapshot with initial synchronization earned and later lag must visibly retain both facts; an accepted processed endpoint ahead of its safe checkpoint must remain clear. Reorg/reopen and duplicate publication must keep retained growth accurate, while genuine pruned bodies stay absent and wallet/body eligibility unchanged.
</specifics>

<deferred>
## Deferred Ideas

Phase 162 owns the complete continuous validated-chain prune/reopen/RPC/both-transports integrated proof and remaining milestone claims. All excluded surfaces in D-16 remain deferred. No pending todos matched Phase 161.
</deferred>
