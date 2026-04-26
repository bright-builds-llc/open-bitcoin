---
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: 13-2026-04-26T18-50-22
generated_at: 2026-04-26T18:50:22.441Z
---

# Phase 13: Operator Runtime Foundations - Context

**Gathered:** 2026-04-26
**Status:** Ready for planning
**Mode:** Yolo

<domain>
## Phase Boundary

Define the stable contracts and dependency decisions that v1.1 runtime, CLI, service, dashboard, storage, and sync work consume. This phase is contract-first: it creates architecture records, typed data models, parser/routing boundaries, and initial tests, but it does not implement durable storage adapters, real sync, service installation, the onboarding wizard, or the Ratatui dashboard.

</domain>

<decisions>
## Implementation Decisions

### Storage foundation
- **D-01:** Create an inspectable storage ADR before durable adapters land. It must compare Rust-native and RocksDB-style options against chainstate, headers, block index, wallet, metrics, recovery, Bazel, and dependency constraints.
- **D-02:** Select `fjall` as the v1.1 default decision target unless later measured storage and recovery checks disprove the choice. Keep `redb` as the strongest Rust-native fallback and `rocksdb` as a fallback only after explicit evidence justifies the native dependency cost.
- **D-03:** Add storage namespace, schema version, persist-mode, recovery-action, and storage-error contracts in the node shell crate only. Do not add a database dependency or filesystem/database side effects in this phase.

### Observability contracts
- **D-04:** Define bounded metrics defaults as a 30 second sampling interval, 2880 samples per series, and a 24 hour max age.
- **D-05:** Define structured log retention defaults as daily rotation, 14 files, 14 days, and 268435456 bytes. Treat rolling-file creation and retention pruning as separate Phase 16 responsibilities.
- **D-06:** Metrics and logging contracts must be serializable data models that later status/dashboard consumers share, not renderer-local state or live writers.

### Shared status model
- **D-07:** Create one shared `OpenBitcoinStatusSnapshot` model for running and stopped nodes.
- **D-08:** Model unavailable live fields explicitly with an `Unavailable { reason }` shape instead of omitting fields when RPC, peers, mempool, wallet, or sync data cannot be collected.
- **D-09:** Include daemon state, version, commit/build provenance, datadir, config paths, network, chain tip, sync progress, peer counts, mempool, wallet, service, logs, metrics, and health signals in the shared model.

### CLI command boundary
- **D-10:** Define an `open-bitcoin` operator path owned by clap for future `status`, `config`, `service`, `dashboard`, and `onboard` commands.
- **D-11:** Preserve `open-bitcoin-cli` as the baseline-compatible RPC client path that continues to use the existing `parse_cli_args` behavior for `-named`, `-stdin`, `-stdinrpcpass`, `-getinfo`, RPC method names, and positional JSON parameters.
- **D-12:** Phase 13 may add command/routing contracts and tests, but must not implement the status command, service manager, dashboard, onboarding wizard, or RPC transport rewrite.

### Config ownership and precedence
- **D-13:** Use `open-bitcoin.jsonc` for Open Bitcoin-only wizard/onboarding, dashboard, service, migration, metrics, logging, storage, and sync settings.
- **D-14:** Keep baseline Bitcoin/Knots-compatible keys in `bitcoin.conf`; Open Bitcoin-only keys must not be written there.
- **D-15:** Document and test precedence as `CLI flags > environment > Open Bitcoin JSONC > bitcoin.conf > cookies > defaults`.

### the agent's Discretion
- Exact Rust field grouping and helper method names are discretionary when they preserve the public contracts and tests above.
- Documentation layout is discretionary, but each ADR/contract document must name exact defaults and phase obligations so later implementers do not infer their own boundaries.

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Phase scope
- `.planning/ROADMAP.md` - Phase 13 goal, requirements, success criteria, and downstream phase dependencies.
- `.planning/REQUIREMENTS.md` - OBS-01, OBS-03, OBS-04, CLI-03, CLI-05, CLI-06, and DB-01 definitions.
- `.planning/PROJECT.md` - v1.1 operator runtime direction, parity constraints, and out-of-scope boundaries.
- `.planning/phases/13-operator-runtime-foundations/13-RESEARCH.md` - Phase 13 technical research and recommended contract shape.

### Existing code boundaries
- `packages/open-bitcoin-node/src/lib.rs` - Node shell crate export surface.
- `packages/open-bitcoin-cli/src/args.rs` - Existing baseline-compatible RPC invocation parser.
- `packages/open-bitcoin-rpc/src/config.rs` - Existing runtime config API.
- `packages/open-bitcoin-rpc/src/config/loader.rs` - Existing `bitcoin.conf` loader and config-key boundary.
- `docs/parity/source-breadcrumbs.json` - Required source breadcrumb manifest for new Rust files.

### Standards
- `AGENTS.md` - Repo-local workflow, parity, verification, and GSD rules.
- `AGENTS.bright-builds.md` - Bright Builds workflow and standards routing.
- `standards-overrides.md` - Local standards exceptions.
- `../coding-and-architecture-requirements/standards/core/architecture.md` - Functional core / imperative shell and typed boundary guidance.
- `../coding-and-architecture-requirements/standards/core/code-shape.md` - Early returns and optional naming guidance.
- `../coding-and-architecture-requirements/standards/core/testing.md` - Unit test expectations.
- `../coding-and-architecture-requirements/standards/languages/rust.md` - Rust module, typing, and test guidance.

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `packages/open-bitcoin-cli/src/args.rs`: Preserve as the RPC compatibility parser and route `open-bitcoin-cli` tokens to it unchanged.
- `packages/open-bitcoin-rpc/src/config/loader.rs`: Reuse its strict unsupported-key behavior to protect the `bitcoin.conf` compatibility boundary.
- `docs/parity/source-breadcrumbs.json`: Extend with Open Bitcoin-only `none` entries for contract modules and Knots breadcrumbs for compatibility-boundary modules.

### Established Patterns
- First-party source files carry top-of-file parity breadcrumb comments.
- Tests are colocated as `foo/tests.rs` with `#[cfg(test)] mod tests;` for multi-file module tests.
- Pure domain contracts should stay data-only; adapters own files, clocks, networking, subprocesses, service managers, and database access.

### Integration Points
- Node status, metrics, logging, and storage contracts are exported from `open-bitcoin-node`.
- CLI operator routing is exported from `open-bitcoin-cli` without changing `src/main.rs`.
- Open Bitcoin JSONC parsing is exported from `open-bitcoin-rpc::config` without merging it into the existing `bitcoin.conf` loader yet.

</code_context>

<specifics>
## Specific Ideas

- Operator-facing v1.1 features should stay quiet, information-dense, and work-focused.
- `open-bitcoin-cli` compatibility must stay auditable against Bitcoin Knots/Core behavior.
- Open Bitcoin JSONC is an additive owner-specific config layer, not a replacement for `bitcoin.conf`.

</specifics>

<deferred>
## Deferred Ideas

- Durable database adapter implementation belongs to Phase 14.
- Real peer sync and persistence belongs to Phase 15.
- Runtime metrics/log writers and retention pruning belong to Phase 16.
- Rich status rendering and onboarding wizard implementation belong to Phase 17.
- launchd/systemd service lifecycle implementation belongs to Phase 18.
- Ratatui dashboard implementation belongs to Phase 19.

</deferred>

---

*Phase: 13-operator-runtime-foundations*
*Context gathered: 2026-04-26*
