---
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: 157-2026-10-05T14-50-48
generated_at: 2026-10-05T14:50:48Z
---

# Phase 157: Safe Activation and Scheduled Index Catch-Up - Context

**Gathered:** 2026-10-05 CDT
**Status:** Ready for planning
**Mode:** Yolo

Material guidance: `AGENTS.md`, `AGENTS.bright-builds.md`, placeholder-only `standards-overrides.md`, architecture, code-shape, testing, verification, local-guidance, Rust and Bun standards. Both active lesson files were read completely (7,188 bytes, 2,397 estimated tokens); the existing audit baseline requires no new audit. Main is clean and synchronized with origin. The strict wrapper owns final commit/push after clean verification.

Toolchain preflight: Rust 1.94.1 is correct. Default shell Bun is 1.4.2, but the repo pins 1.3.9; use the verified `/tmp/open-bitcoin-bun-1.3.9/bun-darwin-aarch64/bun` via a task-local PATH prefix for tests and verification.

<domain>
## Phase Boundary

Own CFAC-01, CFAC-02 and CFIX-01: explicit default-off BASIC index activation, non-mutating refusal when required history is absent, bounded startup/idle catch-up and ordered ongoing validated-connect work. Extend the existing index/prune owner and Fjall store. Runtime reorg, filter/index RPC, peer serving and broad operator projections remain later phases.
</domain>

<decisions>
## Implementation Decisions

### Activation and option contracts

- **D-01:** Support bare `blockfilterindex`, `1` and `basic`; omission and `0` disable. Preserve the pinned Knots repeated/mixed option selection and precedence for the BASIC-only subset, with an explicit truth table and boundary tests across supported config/daemon paths. Unknown values and excluded V0/type 2 must refuse clearly. Do not assume generic last-value-wins semantics without inspecting the pinned parser.
- **D-02:** Explicit activation selects recovered durable storage independently of sync, inbound listening or pruning. A valid datadir is required by the existing durable path. Enabling the index must not activate networking, listeners, relay or peer filter service.
- **D-03:** Configuration must reach the production startup path before interrupted prune resume can delete inputs. Reuse trusted lifecycle enable/disable transitions; establish conservative ownership before index work or dependent deletion, and stop/invalidate work before releasing ownership on disable.

### Missing history and saved prefixes

- **D-04:** Preflight the entire still-required active suffix from the recoverable validated prefix through the authoritative chainstate tip before activation mutates index ownership or resumes destructive work. Require actual bodies and validated non-genesis undo; genesis uses its proven no-undo special case. Fresh indexing needs the genesis prefix.
- **D-05:** Missing history refuses without deleting saved immutable rows/prefix, skipping heights, reconstructing scripts from current coins, downloading, reindexing or repair. Distinguish missing required body versus undo and provide bounded diagnostics. Already valid indexed history need not retain source bodies solely to resume beyond that prefix.
- **D-06:** Preserve Phase 155 recovered coins/chain-metadata fencing and Phase 156 reserved non-overridable protection. Ahead immutable rows are not safe resume authority; every persistence failure retains conservative protection and truthful progress.

### Bounded ordered owner and ordinary callers

- **D-07:** Use one ordered append owner for retained replay and live validated connects. Capture complete body-bound historical spent-output facts, including same-block spends, at the accepted state boundary. A later coins/metadata/payload persistence error must not disappear behind a success-only callback or let unsafe durable progress escape.
- **D-08:** Actual daemon startup performs a bounded first turn; an ordinary scheduled maintenance caller advances further turns while offline/idle without peer messages. Index-enabled startup must start the worker even without public sync or inbound activation. Yield between turns and use explicit bounded block and byte/work budgets; research must measure realistic fixture turns rather than invent an unbounded replay loop.
- **D-09:** While behind, validated connects enlarge the ordered backlog rather than append out-of-order headers. Revalidate branch/lifecycle identity at publication and preserve stale-work refusal. Separate initial synchronization completion, current lag, processed rows and safe durable checkpoint internally; incomplete initial work never reports completion. Branch replacement orchestration remains Phase 158.
- **D-10:** Do not add an index-owned full-history cache or duplicate chain/undo vectors. Keep decisions pure and effects in thin existing adapters; use small modules and existing typed BASIC inputs, immutable records, lifecycle generations and publication barriers.
- **D-13:** Preserve earlier Phase 142 accepted-unflushed coins behavior; do not force a coins flush on every connect merely to eliminate crash loss. Preserve Phase 153 automatic retention under its existing Periodic/Always owner, without a new deletion worker. Follow Phase 136's receive-independent injected-timer testing and Phase 127/134's short prepare/complete epochs where practical.
- **D-14:** Bound total scheduler-turn work, including validation and projection publication, not only generated record count. Existing full-prefix record/checkpoint/projection scans require explicit research and an incremental append design or an honest measured bound; retain complete integrity validation at startup/recovery.

### Evidence and finalization

- **D-11:** Prove configuration, actual daemon startup, offline scheduled multiple-turn progress, continuous consensus-validated historical/same-block spends, ordinary connect and accepted-state/persistence failures with real Fjall reopen and concrete production consumers. Include actual paired historical payload loss, fresh and saved-prefix refusal, safe resume after indexed source loss and source preservation on refusal.
- **D-12:** Document exact option semantics and resource bounds, add required source breadcrumbs, parity evidence and deterministic regression checks, review relevant README/UAT claims, and run `bash scripts/verify.sh` plus lifecycle/source/security review before final commit/push. Use repo-local Cargo/Bazel command forms in UAT. No new production dependency or crate, public-network verification, production or funds claims.

### Agent Discretion

Exact boundary types, scheduler interval/budgets after measurement, fault seams and plan decomposition belong to research/planning. Preserve existing unaffected activation surfaces and v2.4 advisories unless a narrow change is necessary for this contract. No frontend design contract is needed: option forms are CLI/config syntax, and broad dashboard/status projections belong to Phase 161.
</decisions>

<canonical-refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Scope, prior decisions and guidance

- `.planning/ROADMAP.md` — Phase 157 criteria, evidence gates and later ownership.
- `.planning/REQUIREMENTS.md` — CFAC-01, CFAC-02, CFIX-01 and exclusions.
- `.planning/PROJECT.md` — single-chainstate, default-off and no-repair boundaries.
- `.planning/research/ARCHITECTURE.md`, `.planning/research/PITFALLS.md`, `.planning/research/FEATURES.md` — startup ordering, catch-up and accepted-state hazards.
- `.planning/phases/154-basic-generation-and-commitment-parity/154-CONTEXT.md` — complete historical script inputs.
- `.planning/phases/155-recoverable-index-and-pre-prune-startup-protection/155-CONTEXT.md` — recovery, immutable prefix and durable fence.
- `.planning/phases/156-index-owned-manual-and-automatic-prune-coordination/156-CONTEXT.md` — ownership and generation transitions.
- `AGENTS.md`, `AGENTS.bright-builds.md`, `standards-overrides.md`, `standards/index.md` — local workflow and standards routing.

### Pinned baseline and concrete callers

- `packages/bitcoin-knots/src/init.cpp` — option selection and history startup gate.
- `packages/bitcoin-knots/src/common/args.cpp` — repeated/config option precedence.
- `packages/bitcoin-knots/src/index/base.cpp` — accepted callbacks, catch-up and committed progress.
- `packages/bitcoin-knots/src/index/blockfilterindex.cpp`, `packages/bitcoin-knots/src/undo.h` — historical append inputs.
- `packages/open-bitcoin-node/src/storage/fjall_store/filters/lifecycle.rs` — trusted enable preflight and disable ordering.
- `packages/open-bitcoin-node/src/chainstate/flush_lifecycle.rs` — pre-prune initialize boundary.
- `packages/open-bitcoin-node/src/sync/open_runtime.rs` — durable production open.
- `docs/parity/source-breadcrumbs.json`, `scripts/verify.sh` — source provenance and full verification.
- https://bips.dev/158/ — spent-output script requirement, checked against the pinned local implementation.
</canonical-refs>

<code-context>
## Existing Code Insights

### Reusable Assets

Typed BASIC construction, body-bound validated undo, recoverable immutable records, atomic SyncAll publisher, coins/metadata fence, reserved index lock and lifecycle generation. Trusted enable already preflights retained required history before acquisition.

### Established Patterns

One serialized chainstate/prune authority; conservative fail-closed errors; no silent recovery or history erasure. Bun owns higher-level automation. New Rust files require source breadcrumb registration.

### Integration Points

Carry explicit configuration into initialize before resumed prune deletion. Extend daemon durable selection independently of networking. Route bounded replay and accepted connects through the same owner and make the ordinary maintenance worker run for index-only activation.
</code-context>

<specifics>
## Specific Ideas

Start an index-only daemon on a retained continuous validated history larger than one turn. Observe a strict prefix after startup and suffix advancement from idle maintenance; reopen to prove fenced progress. Remove a required body or undo through real paired pruning and compare fresh versus saved-prefix refusal without mutating the valid prefix.
</specifics>

<deferred>
## Deferred Ideas

Runtime reorg (158), filter/index RPC (159), peer serving/service bits (160), dashboard and support projections (161), full post-prune client proof (162). No pending todos matched. Automatic repair/import, V0/BIP37, public defaults, production/funds claims and unrelated advisory cleanup remain excluded.
</deferred>
