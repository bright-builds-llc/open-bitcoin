# Open Bitcoin

## What This Is

Open Bitcoin is a Bitcoin node and wallet implementation in Rust, built to preserve externally observable behavior from Bitcoin Knots `29.3.knots20260210` where a behavior is in scope. After shipping v1.1, the project includes a headless parity baseline plus a terminal-first operator surface for status, service management, dashboard workflows, wallet operations, and dry-run migration planning. The active v1.2 milestone is focused on making full public-mainnet initial block download an explicit, operator-ready `open-bitcoind` workflow.

It is for contributors and operators who want a reference-grade node with a cleaner, more type-safe internal architecture, auditable parity, and a strict separation between pure domain logic and effectful adapters.

## Core Value

When a behavior is in scope, Open Bitcoin must behave like the pinned Knots baseline on the outside while staying simpler and safer on the inside.

## Current State

v1.0 Headless Parity shipped on 2026-04-26, and v1.1 Operator Runtime and Real-Network Sync shipped on 2026-04-30.

The repository now includes durable Fjall-backed runtime storage, real-network sync foundations, bounded metrics and structured logs, the `open-bitcoin` operator binary, launchd/systemd service flows, a Ratatui dashboard, practical wallet runtime workflows, and an auditable dry-run migration surface for existing Core or Knots installs. v1.2 exists to wire those sync foundations into `open-bitcoind` as an opt-in public-mainnet sync workflow with validated headers, blocks, durable restart/resume, and operator observability.

Milestone archives live under `.planning/milestones/`, including the shipped roadmap, requirements, and final passed audit for each completed milestone. One residual risk remains from the v1.1 audit: dashboard pseudoterminal repaint and raw-input behavior is still a manual validation surface rather than an end-to-end automated regression.

## Current Milestone Goals

v1.2 Full Mainnet Network Syncing is the active milestone. Its goal is to make public-mainnet initial block download an explicit, operator-ready `open-bitcoind` workflow without overclaiming full production-node or production-wallet readiness.

- Wire daemon startup, config, and shutdown around an explicit mainnet sync mode.
- Resolve and maintain bounded outbound mainnet peer connections.
- Perform validated header-first sync, block download, block connection, durable restart/resume, and reorg-aware recovery.
- Surface truthful progress, health, resource pressure, and stop/resume state through status, dashboard, metrics, logs, and RPC-facing output.
- Keep default verification hermetic while adding opt-in live mainnet smoke and benchmark evidence.
- Update operator and parity docs so shipped claims match the new daemon sync behavior.

## Requirements

### Validated

- ✓ v1.0 validated all 28 source-of-truth requirements across reference baseline, architecture, verification, consensus, chainstate, mempool, networking, wallet, RPC, CLI, performance, and auditability surfaces. Archive: `.planning/milestones/v1.0-REQUIREMENTS.md`
- ✓ v1.1 validated all 44 operator-runtime requirements across observability, dashboard, CLI and onboarding, service lifecycle, durable storage, sync, wallet, migration, benchmark, and documentation surfaces. Archive: `.planning/milestones/v1.1-REQUIREMENTS.md`

### Active

See `.planning/REQUIREMENTS.md` for the active v1.2 requirements. The active requirement groups are:

- Daemon Sync Activation and Safety
- Peer Discovery and Connectivity
- Headers, Blocks, and Chain Progress
- Runtime Resilience
- Operator Observability
- Verification, Parity Evidence, and Docs

### Out of Scope

- Faithful Qt GUI parity or porting the upstream GUI code - shipped milestones remain terminal-first and headless.
- Windows service integration - still deferred until a later milestone.
- Automatic destructive migration of existing Bitcoin Core or Bitcoin Knots data - migration must be dry-run-first, explicit, and backup-aware.
- Broad unsupported drop-in replacement claims beyond the audited evidence surface - parity claims remain scoped to shipped artifacts and documented deviations.
- Public marketing sites or hosted dashboards - completed milestones prioritize local operator surfaces and node correctness.
- Replacing `bitcoin.conf` compatibility with an Open Bitcoin-only config format - JSONC layers on top of, not instead of, baseline config behavior.
- Full production-node, production-funds wallet, inbound peer serving, address relay, compact block relay, and mempool transaction relay claims - these are deferred beyond v1.2.

## Context

- The repository has first-party pure-core domain and codec crates under `packages/`, plus parity catalog artifacts under `docs/parity/`.
- Bitcoin Knots `29.3.knots20260210` is the pinned behavioral reference baseline.
- The current codebase totals 83,494 first-party lines, including 42,363 production Rust lines at the v1.1 archive point.
- Repo-native verification remains centered on `bash scripts/verify.sh`, including Rust checks, parity breadcrumbs, benchmark smoke and report validation, and Bazel smoke builds.
- Bun is a pinned runtime for repo-owned TypeScript automation, not a package-install surface; there is no `package.json` or `bun install` bootstrap step.
- Operator-facing surfaces should stay quiet, information-dense, and work-focused: terminal dashboard controls, status output, onboarding copy, service actions, and migration guidance should help operators make decisions without marketing language.
- Any migration from Bitcoin Core or Bitcoin Knots must treat the existing datadir and wallet data as high-value user data. Detection and explanation are in scope before automated mutation, while destructive apply-mode work remains deferred.
- First-party code should continue to live in well-bounded packages, with Bazelisk and Bazel/Bzlmod as the top-level build entrypoint unless a later decision replaces that choice.
- The project explicitly avoids existing Rust Bitcoin libraries in the production path and instead exports first-party Rust Bitcoin libraries from this repository.
- Verification must emphasize externally observable parity, pure-core correctness, hermetic integration testing, and contributor guardrails against accidental architectural drift.
- v1.2 public-network checks must remain opt-in so `bash scripts/verify.sh` stays deterministic by default.

## Constraints

- **Behavioral baseline**: Match Bitcoin Knots `29.3.knots20260210` for all in-scope surfaces - parity claims must be auditable.
- **Architecture**: Follow functional core / imperative shell boundaries - pure business logic stays free of direct I/O and runtime side effects.
- **Dependency policy**: Keep dependencies minimal and security-conscious, and do not use existing Rust Bitcoin libraries in the production path - the project owns its own domain model and implementation surface.
- **Build tooling**: Use Bazelisk and Bazel with Bzlmod for first-party workspace builds - multi-package growth should remain manageable from the repo root.
- **Verification**: Enforce formatting, linting, build, testing, coverage, architecture-policy, panic-site, parity-breadcrumb, and benchmark checks through repo-native verification.
- **Scope**: Completed milestones are headless and terminal-first; future GUI work must be planned explicitly.

## Key Decisions

| Decision | Rationale | Outcome |
|----------|-----------|---------|
| Use Bitcoin Knots `29.3.knots20260210` as the reference baseline | The project needs one pinned behavioral contract for parity work and regression detection | Implemented and archived in v1.0 |
| Prioritize behavioral parity over line-by-line source parity | Rust internals should be allowed to become safer and clearer without breaking external behavior | Implemented as the project parity model |
| Use functional core / imperative shell boundaries throughout first-party code | Strong boundaries improve testability, make illegal states unrepresentable, and prevent I/O drift into the pure core | Enforced by architecture policy and verification |
| Use Bazelisk and Bazel/Bzlmod for first-party workspace builds | The repository is expected to become a multi-package workspace with repeatable top-level builds | Implemented for first-party packages |
| Keep v1.0 headless and defer any GUI to a future milestone | GUI parity would slow core correctness work and should be designed on its own terms later | Implemented; v1.1 added a terminal dashboard instead of a desktop GUI |
| Avoid third-party Rust Bitcoin libraries in the production path | The project wants full ownership of domain abstractions, invariants, and behavior | Implemented for the production path |
| Adopt a terminal-first operator surface for v1.1 | A Ratatui dashboard and rich CLI status move operator usability forward without changing the headless product boundary | Shipped in v1.1 |
| Treat migration as explicit, dry-run-first, and reversible | Existing Core or Knots datadirs and wallets are high-value user data and must not be mutated implicitly | Shipped and audited in v1.1 |
| Keep shared service definitions at scan scope through `DetectionScan` | Future consumers should opt into service ownership association explicitly instead of inheriting misleading per-installation copies | Implemented in Phase 34 and archived with v1.1 |
| Scope v1.2 to opt-in daemon initial block download | Full mainnet sync should first be proven through `open-bitcoind` headers, blocks, restart/resume, and observability before broader P2P, wallet, or production service claims | Active v1.2 planning decision |

## Historical Context

<details>
<summary>Archived milestone planning context</summary>

- v1.0 archive: `.planning/milestones/v1.0-ROADMAP.md`, `.planning/milestones/v1.0-REQUIREMENTS.md`, `.planning/milestones/v1.0-MILESTONE-AUDIT.md`
- v1.1 archive: `.planning/milestones/v1.1-ROADMAP.md`, `.planning/milestones/v1.1-REQUIREMENTS.md`, `.planning/milestones/v1.1-MILESTONE-AUDIT.md`
- Raw phase execution history for v1.0 remains in `.planning/phases/`, while the v1.1 phase history lives in `.planning/milestones/v1.1-phases/`.

</details>

---
*Last updated: 2026-05-01 after defining the v1.2 full mainnet network syncing milestone*
