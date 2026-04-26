# Open Bitcoin

## What This Is

Open Bitcoin is a Bitcoin node and wallet implementation in Rust, built to preserve externally observable behavior from Bitcoin Knots `29.3.knots20260210` where a behavior is in scope. After v1.0, the project is moving from a headless parity baseline toward an operator-usable node that can sync against the real Bitcoin network, persist runtime state, expose clear status and dashboard surfaces, and support careful migration from existing Bitcoin Core or Bitcoin Knots installs.

It is for contributors and operators who want a reference-grade node with a cleaner, more type-safe internal architecture, auditable parity, and a strict separation between pure domain logic and effectful adapters.

## Core Value

When a behavior is in scope, Open Bitcoin must behave like the pinned Knots baseline on the outside while staying simpler and safer on the inside.

## Current State

v1.0 Headless Parity shipped on 2026-04-26. The repository contains a headless Rust node and wallet implementation with scoped consensus, validation, chainstate, mempool, networking, wallet, RPC, CLI, config, parity-harness, benchmark, audit, and panic-guard surfaces wired and archived for the initial milestone.

The v1.0 archive lives under `.planning/milestones/`, with the detailed shipped roadmap, requirements ledger, and milestone audit preserved as historical artifacts.

v1.1 Phases 13 through 16 are complete. The active milestone now has shared operator-runtime contracts, durable Fjall-backed runtime storage, hermetic real-network sync foundations, and bounded metrics/log/sync telemetry ready for the CLI, service, and dashboard phases.

## Current Milestone: v1.1 Operator Runtime and Real-Network Sync

**Goal:** Make Open Bitcoin usable as an operator-facing, service-managed node that can begin true network sync work with durable storage, richer CLI/TUI surfaces, migration guidance, and expanded parity coverage.

**Target features:**
- Operator readiness: rich `status`, historical metrics, log rotation, and a sensible Ratatui dashboard for live node state.
- Parity expansion: a full drop-in audit against Bitcoin Knots and Bitcoin Core operator expectations, with deviations and migration risks documented explicitly.
- Persistence and runtime hardening: durable database-backed chainstate, header, block, wallet, and sync state with restart and recovery behavior.
- Wallet expansion: broader send, rescan, multiwallet, descriptor, and migration-safe wallet flows on top of the durable runtime.
- CLI and onboarding: expand the existing `clap` dependency into the main command model, add an idempotent first-run wizard, and store Open Bitcoin-only settings without breaking `bitcoin.conf` compatibility.
- Service lifecycle: install, uninstall, enable, disable, and status support for macOS first and Linux second.
- Real sync: peer discovery, long-running socket orchestration, real-network headers and block download, and benchmarks that measure true sync behavior.

## Requirements

### Validated

- v1.0 validated all 28 source-of-truth requirements across reference baseline, architecture, verification, consensus, chainstate, mempool, networking, wallet, RPC, CLI, performance, and auditability surfaces.
- The detailed v1 requirement ledger is archived at `.planning/milestones/v1.0-REQUIREMENTS.md`.
- The v1.0 audit passed with GAP-01 through GAP-04 closed and no open blockers; the audit is archived at `.planning/milestones/v1.0-MILESTONE-AUDIT.md`.

### Active

- [ ] v1.1 gives operators a clear status surface for running or stopped nodes, including config, service, datadir, sync, peers, mempool, wallet, metrics, and logs.
- [ ] v1.1 adds a Ratatui dashboard that makes live node health and sync progress understandable without becoming a GUI replacement.
- [ ] v1.1 adds idempotent onboarding and migration flows that detect existing Bitcoin Core or Bitcoin Knots installs and explain tradeoffs before touching user data.
- [ ] v1.1 adds macOS-first and Linux-second service lifecycle integration for installing, enabling, disabling, uninstalling, and inspecting the daemon.
- [ ] v1.1 adds durable database-backed runtime state and real-network sync foundations, with restart recovery and benchmarks.
- [ ] v1.1 expands wallet behavior enough for practical runtime use while preserving v1.0 wallet and RPC parity.
- [ ] v1.1 keeps every new operator behavior tied to Knots/Core audit evidence, explicit deviations, and repo-native verification.

### Out of Scope

- Faithful Qt GUI parity or porting the upstream GUI code - v1.1 builds a terminal dashboard, not a desktop GUI.
- Windows service integration - v1.1 focuses on macOS and Linux, with macOS the higher priority.
- Automatic destructive migration of existing Bitcoin Core or Bitcoin Knots data - migration must be dry-run-first, explicit, and backup-aware.
- Claiming full drop-in replacement status before the v1.1 audit closes - parity claims remain evidence-scoped.
- Public marketing sites or hosted dashboards - v1.1 is about local operator surfaces and node correctness.
- Replacing `bitcoin.conf` compatibility with an Open Bitcoin-only config format - any JSONC config must layer on top of, not break, baseline config behavior.

## Context

- The repository has first-party pure-core domain and codec crates under `packages/`, plus seeded parity catalog artifacts under `docs/parity/catalog/`.
- Bitcoin Knots `29.3.knots20260210` is the pinned behavioral reference baseline.
- The current CLI crate already depends on `clap` with derive support; v1.1 should expand that into a coherent command tree while preserving baseline-compatible `bitcoin-cli` invocation behavior where it remains in scope.
- The current node adapters persist chainstate and wallet snapshots only through in-memory stores; v1.1 needs a durable database layer before real-network sync can be made honest.
- The current networking surface has message, peer, header-store, and managed-node logic, but real long-running socket orchestration, peer discovery, address relay, and production sync loops remain follow-up work.
- The current RPC and CLI catalog explicitly defers `getpeerinfo`, `-netinfo`, daemon supervision, broader process-control helpers, richer wallet-send flows, and multiwallet endpoint selection.
- Operator-facing v1.1 features should stay quiet, information-dense, and work-focused: terminal dashboard controls, status output, and onboarding copy should help operators make decisions without marketing language.
- Any migration from Bitcoin Core or Bitcoin Knots must treat the existing datadir and wallet data as high-value user data. Detection and explanation are in scope before automated mutation.
- First-party code should continue to live in well-bounded packages, with Bazelisk and Bazel/Bzlmod as the top-level build entrypoint unless a later decision replaces that choice.
- The project explicitly avoids existing Rust Bitcoin libraries in the production path and instead exports first-party Rust Bitcoin libraries from this repository.
- Verification must emphasize externally observable parity, pure-core correctness, hermetic integration testing, and contributor guardrails against accidental architectural drift.

## Constraints

- **Behavioral baseline**: Match Bitcoin Knots `29.3.knots20260210` for all in-scope surfaces - parity claims must be auditable.
- **Architecture**: Follow functional core / imperative shell boundaries - pure business logic stays free of direct I/O and runtime side effects.
- **Dependency policy**: Keep dependencies minimal and security-conscious, and do not use existing Rust Bitcoin libraries in the production path - the project owns its own domain model and implementation surface.
- **Build tooling**: Use Bazelisk and Bazel with Bzlmod for first-party workspace builds - multi-package growth should remain manageable from the repo root.
- **Verification**: Enforce formatting, linting, build, testing, coverage, architecture-policy, panic-site, parity-breadcrumb, and benchmark checks through repo-native verification.
- **Scope**: v1.1 adds local terminal/operator surfaces and service integration, not a graphical desktop client.

## Key Decisions

| Decision | Rationale | Outcome |
|----------|-----------|---------|
| Use Bitcoin Knots `29.3.knots20260210` as the reference baseline | The project needs one pinned behavioral contract for parity work and regression detection | Implemented and archived in v1.0 |
| Prioritize behavioral parity over line-by-line source parity | Rust internals should be allowed to become safer and clearer without breaking external behavior | Implemented as the v1.0 parity model |
| Use functional core / imperative shell boundaries throughout first-party code | Strong boundaries improve testability, make illegal states unrepresentable, and prevent I/O drift into the pure core | Enforced by architecture policy and verification |
| Use Bazelisk and Bazel/Bzlmod for first-party workspace builds | The repository is expected to become a multi-package workspace with repeatable top-level builds | Implemented for first-party packages |
| Keep v1.0 headless and defer any GUI to a future milestone | GUI parity would slow core correctness work and should be designed on its own terms later | Implemented; v1.1 adds terminal dashboard only |
| Avoid third-party Rust Bitcoin libraries in the production path | The project wants full ownership of domain abstractions, invariants, and behavior | Implemented for the production path |
| Archive v1.0 before new milestone planning | The next milestone needs a clean requirements and roadmap surface while preserving historical evidence | v1.0 archive created under `.planning/milestones/` |
| Adopt a terminal-first operator surface for v1.1 | A Ratatui dashboard and rich CLI status move operator usability forward without changing the headless product boundary | Pending v1.1 execution |
| Treat migration as explicit and reversible | Existing Core/Knots datadirs and wallets are high-value user data and must not be mutated implicitly | Pending v1.1 execution |

## Evolution

This document evolves at phase transitions and milestone boundaries.

**After each phase transition** (via `/gsd-transition`):
1. Requirements invalidated? -> Move to Out of Scope with reason
2. Requirements validated? -> Move to Validated with phase reference
3. New requirements emerged? -> Add to Active
4. Decisions to log? -> Add to Key Decisions
5. "What This Is" still accurate? -> Update if drifted

**After each milestone** (via `/gsd-complete-milestone`):
1. Full review of all sections
2. Core Value check - still the right priority?
3. Audit Out of Scope - reasons still valid?
4. Update Context with current state

---
*Last updated: 2026-04-26 after Phase 16 completion*
