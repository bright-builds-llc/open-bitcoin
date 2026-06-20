# Open Bitcoin

## What This Is

Open Bitcoin is a Bitcoin node and wallet implementation in Rust, built to preserve externally observable behavior from Bitcoin Knots `29.3.knots20260210` where a behavior is in scope. After shipping v1.7, the project includes a headless parity baseline, a terminal-first operator surface for status, service management, dashboard workflows, wallet operations, and dry-run migration planning, plus an explicit opt-in `open-bitcoind` workflow for public-mainnet initial block download, unattended operator review, full-sync completion evidence, multi-day soak evidence, resource-bound enforcement, recovery diagnosis, support-bundle forensics, audited node-hardening, public peer compatibility, validated header and block progress, same-datadir restart/resume evidence, service lifecycle evidence, redacted support bundles, compatibility harness reports, sync-to-tip evidence, stay-current evidence, and truthful release boundaries. The active v1.8 milestone defines the production full-node readiness boundary and evidence gates before any production-readiness claim is made.

It is for contributors and operators who want a reference-grade node with a cleaner, more type-safe internal architecture, auditable parity, and a strict separation between pure domain logic and effectful adapters.

## Core Value

When a behavior is in scope, Open Bitcoin must behave like the pinned Knots baseline on the outside while staying simpler and safer on the inside.

## Current State

v1.0 Headless Parity shipped on 2026-04-26, v1.1 Operator Runtime and Real-Network Sync shipped on 2026-04-30, v1.2 Full Mainnet Network Syncing shipped on 2026-05-23, v1.3 Public Mainnet Sync Proof and Node Hardening shipped on 2026-06-02, v1.4 Mainnet IBD Convergence and Peer Compatibility shipped on 2026-06-05, v1.5 Unattended Mainnet Node Operation Readiness shipped on 2026-06-10, v1.6 Mainnet Full-Sync Completion shipped on 2026-06-14, and v1.7 Full-Sync Soak and Recovery Hardening was completed on 2026-06-19 after Phase 81 audit traceability closure and formally archived on 2026-06-20. v1.8 Production Full-Node Readiness Boundary started planning on 2026-06-20.

The repository now includes durable Fjall-backed runtime storage, real-network sync foundations, bounded metrics and structured logs, the `open-bitcoin` operator binary, launchd/systemd service flows, a Ratatui dashboard, practical wallet runtime workflows, an auditable dry-run migration surface for existing Core or Knots installs, daemon-owned opt-in mainnet IBD review, resilient outbound peer lifecycle behavior, resource bounds, durable recovery, invalid-data handling, truth-aligned sync surfaces, live-smoke reporting, redacted support evidence, public peer compatibility diagnosis, validated header progress, bounded block download/connect progress, same-datadir restart/resume evidence, unattended sync loop control, service-supervised restart/resume evidence, compatibility harness operator reports, sync-to-tip evidence, stay-current evidence, multi-day soak evidence, bounded recovery diagnosis, support-bundle forensics, explicit v1.3 through v1.7 release-boundary evidence, and deterministic guards that keep public-network checks outside default verification.

Milestone archives live under `.planning/milestones/`, including shipped roadmap and requirements archives, final audit artifacts where they exist, and raw phase histories for v1.1 and v1.2. Raw v1.0, v1.3, v1.4, v1.5, v1.6, and v1.7 phase histories remain in `.planning/phases/` for parity and UAT traceability. One residual risk remains from the v1.1 audit: dashboard pseudoterminal repaint and raw-input behavior is still a manual validation surface rather than an end-to-end automated regression. v1.2 did not create a dedicated milestone audit artifact; Phase 40 closeout and Phase 41 security audit, verification, and UAT are the closeout evidence trail. v1.3 archived with a `ready_for_archive` milestone audit and closes public-network evidence through fresh diagnosed-blocker evidence rather than a successful live-progress claim. v1.4 archived with a `tech_debt` milestone audit: implementation and integration coverage passed, while planning traceability was corrected during archive prep and a future operator wrapper around the compatibility harness remained optional cleanup. v1.5 archived with a `passed` milestone audit and no open requirement, integration, flow, or current tech-debt gaps. v1.6 archived as source-built, explicit opt-in full-sync completion evidence. v1.7 archived with a `passed` milestone audit after Phase 81 closed RES/REC verification traceability and refreshed stale milestone state, with production-node readiness, inbound serving, relay, production-funds wallet use, migration apply mode, packaging, hosted dashboards, GUI work, public-network CI, automatic support-bundle upload, destructive repair, and release-blocking live sync still deferred. v1.8 is planned as a boundary-setting milestone for support policy, upgrade expectations, service runbooks, release-readiness evidence, and claim guardrails before any production full-node readiness statement.

## Current Milestone: v1.8 Production Full-Node Readiness Boundary

**Goal:** Define and enforce the support, upgrade, service, runbook, release-readiness, and evidence boundaries required before Open Bitcoin may truthfully claim production full-node readiness.

**Target features:**
- Production terminology and support-boundary matrix separating supported, preview, opt-in UAT, unsupported, and deferred surfaces.
- Upgrade policy for source-built installs, state and schema compatibility, rollback guidance, backup expectations, and operator decision points.
- Operator runbooks for preflight, long-run operation, service supervision, failure triage, recovery, support bundles, and escalation.
- Service expectation docs that distinguish source-built daemon operation, launchd/systemd supervision, public-network dependencies, and unsupported packaged-service claims.
- Release-readiness checklist and deterministic verification checks that prevent overbroad production, wallet, relay, inbound-serving, migration, packaging, hosted-dashboard, or public-network CI claims.
- Explicit production definition with evidence gates and a no-claim boundary until those gates are satisfied.

## Completed Milestone: v1.7 Full-Sync Soak and Recovery Hardening

**Status:** Shipped and archived on 2026-06-20 after Phase 81 audit traceability closure.

**Goal:** Make multi-day explicit opt-in full-sync runs diagnosable, bounded, restart-safe, and supportable when they fail or degrade.

**Target features:**
- Multi-day explicit opt-in soak execution with durable run identity, resumable report state, bounded stop conditions, and deterministic synthetic coverage.
- Disk, storage, cache, queue, log, metric, and support-bundle bounds that remain visible and actionable during long runs.
- Corruption, schema, partial-write, lock-contention, and stale-lock recovery guidance without hidden datadir mutation.
- Progress guarantees and stall diagnosis that prevent false progress while explaining public-network, peer, validation, storage, at-tip, and local-stop causes.
- Redacted "what happened" support bundles and failure narratives that reconstruct timelines, checkpoints, peer outcomes, resource pressure, recovery events, and final verdicts.
- Verification and release boundaries that keep public-network soak checks opt-in and preserve deferred production-node, inbound-serving, relay, wallet, migration, packaging, GUI, and hosted-dashboard scope.

## Requirements

### Validated

- ✓ v1.0 validated all 28 source-of-truth requirements across reference baseline, architecture, verification, consensus, chainstate, mempool, networking, wallet, RPC, CLI, performance, and auditability surfaces. Archive: `.planning/milestones/v1.0-REQUIREMENTS.md`
- ✓ v1.1 validated all 44 operator-runtime requirements across observability, dashboard, CLI and onboarding, service lifecycle, durable storage, sync, wallet, migration, benchmark, and documentation surfaces. Archive: `.planning/milestones/v1.1-REQUIREMENTS.md`
- ✓ v1.2 validated all 26 full-mainnet-sync requirements across daemon activation, peer discovery, headers, blocks, restart/resume, observability, docs, live-smoke evidence, and security closeout. Archive: `.planning/milestones/v1.2-REQUIREMENTS.md`
- ✓ v1.3 validated all 22 public-mainnet proof and node-hardening requirements across opt-in live-smoke evidence, peer lifecycle resilience, resource bounds, durable recovery, observability, support evidence, threat modeling, and release-boundary documentation. Archive: `.planning/milestones/v1.3-REQUIREMENTS.md`
- ✓ v1.4 validated all 22 mainnet IBD convergence and peer-compatibility requirements across compatibility diagnosis, header progress, block download/connect progress, same-datadir restart/resume evidence, operator evidence, support redaction, threat modeling, and release-boundary documentation. Archive: `.planning/milestones/v1.4-REQUIREMENTS.md`
- ✓ v1.5 validated all 23 unattended mainnet node operation readiness requirements across unattended loop control, resource/recovery taxonomy, sync truth surfaces, service lifecycle, service restart/resume evidence, support review docs, compatibility wrapper reporting, and deterministic release-boundary documentation. Archive: `.planning/milestones/v1.5-REQUIREMENTS.md`
- ✓ v1.6 validated all 26 mainnet full-sync completion requirements across active-chain validation, tip tracking, stay-current behavior, reorg and peer recovery, resource/restart evidence, observability, support evidence, opt-in UAT, deterministic verification, and release-boundary documentation. Archive: `.planning/milestones/v1.6-REQUIREMENTS.md`
- ✓ v1.7 validated all 24 full-sync soak and recovery hardening requirements across multi-day soak evidence, resource bounds, recovery diagnosis, progress guarantees, support-bundle forensics, opt-in UAT, deterministic verification, parity roots, scoped release-boundary documentation, and Phase 81 audit traceability closure. Archive: `.planning/milestones/v1.7-REQUIREMENTS.md`

### Active

- [ ] Define production full-node terminology, support levels, and evidence gates before any readiness claim.
- [ ] Document source-built upgrade, rollback, backup, and state/schema compatibility expectations.
- [ ] Provide operator runbooks for preflight, service supervision, long-run operation, triage, recovery, support bundles, and escalation.
- [ ] Define service expectations for source-built daemon operation while keeping packaged-service and broad distribution claims deferred.
- [ ] Add release-readiness documentation and deterministic checks that prevent false production, wallet, relay, inbound-serving, migration, packaging, hosted-dashboard, or public-network CI claims.

### Out of Scope

- Faithful Qt GUI parity or porting the upstream GUI code - shipped milestones remain terminal-first and headless.
- Windows service integration - still deferred until a later milestone.
- Automatic destructive migration of existing Bitcoin Core or Bitcoin Knots data - migration must be dry-run-first, explicit, and backup-aware.
- Broad unsupported drop-in replacement claims beyond the audited evidence surface - parity claims remain scoped to shipped artifacts and documented deviations.
- Public marketing sites or hosted dashboards - completed milestones prioritize local operator surfaces and node correctness.
- Replacing `bitcoin.conf` compatibility with an Open Bitcoin-only config format - JSONC layers on top of, not instead of, baseline config behavior.
- Full production-node, production-funds wallet, inbound peer serving, address relay, compact block relay, and mempool transaction relay claims - these are deferred beyond shipped milestones.
- Claiming v1.8 as production full-node ready by default - this milestone defines gates and guardrails before such a claim is allowed.

## Context

- The repository has first-party pure-core domain and codec crates under `packages/`, plus parity catalog artifacts under `docs/parity/`.
- Bitcoin Knots `29.3.knots20260210` is the pinned behavioral reference baseline.
- The current codebase totals 136,450 tracked first-party lines, including 51,585 production Rust lines in the final v1.7 archive LOC report.
- Repo-native verification remains centered on `bash scripts/verify.sh`, including Rust checks, parity breadcrumbs, benchmark smoke and report validation, and Bazel smoke builds.
- Bun is a pinned runtime for repo-owned TypeScript automation, not a package-install surface; there is no `package.json` or `bun install` bootstrap step.
- Operator-facing surfaces should stay quiet, information-dense, and work-focused: terminal dashboard controls, status output, onboarding copy, service actions, and migration guidance should help operators make decisions without marketing language.
- Any migration from Bitcoin Core or Bitcoin Knots must treat the existing datadir and wallet data as high-value user data. Detection and explanation are in scope before automated mutation, while destructive apply-mode work remains deferred.
- First-party code should continue to live in well-bounded packages, with Bazelisk and Bazel/Bzlmod as the top-level build entrypoint unless a later decision replaces that choice.
- The project explicitly avoids existing Rust Bitcoin libraries in the production path and instead exports first-party Rust Bitcoin libraries from this repository.
- Verification must emphasize externally observable parity, pure-core correctness, hermetic integration testing, and contributor guardrails against accidental architectural drift.
- Public-network checks must remain opt-in unless a future milestone deliberately changes the verification contract, so `bash scripts/verify.sh` stays deterministic by default.
- Production full-node readiness must remain a gated claim, not a marketing label. v1.8 should make the boundary explicit across support docs, upgrade policy, runbooks, service expectations, and release-readiness checks.

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
| Scope v1.2 to opt-in daemon initial block download | Full mainnet sync should first be proven through `open-bitcoind` headers, blocks, restart/resume, and observability before broader P2P, wallet, or production service claims | Shipped in v1.2 |
| Scope v1.3 to public-mainnet proof and node hardening | The v1.2 live UAT did not observe header or block progress, so v1.3 needed to close that evidence gap before expanding wallet, inbound-serving, relay, packaging, or migration claims | Shipped in v1.3 with Phase 53 fresh diagnosed-blocker evidence; no successful live-progress claim was added |
| Scope v1.4 to mainnet IBD convergence and peer compatibility | v1.3 closed cleanly through typed diagnosed-blocker evidence, so the next highest-leverage claim was successful opt-in live header, block, and restart/resume progress rather than inbound serving, relay, packaging, wallet, or migration apply mode | Shipped in v1.4 with compatibility, header, block, restart/resume, operator evidence, support redaction, threat-model, and release-boundary evidence |
| Scope v1.5 to unattended mainnet node operation readiness | v1.4 proved bounded IBD progress and restart/resume evidence, so the next step is making the opt-in daemon workflow safe and observable for extended unattended operator review before expanding inbound serving, relay, wallet, migration apply, or packaging claims | Shipped in v1.5 with bounded loop control, resource/recovery taxonomy, service evidence, support bundles, compatibility wrapper reports, and deterministic release-boundary checks |
| Scope v1.6 to mainnet full-sync completion | v1.5 made long-running operator review bounded and observable, so the highest-leverage next claim is syncing the active mainnet chain to tip and staying current before inbound serving, relay, packaging, migration apply, or production-wallet scope | Shipped in v1.6 with explicit opt-in full-sync completion evidence |
| Scope v1.7 to full-sync soak and recovery hardening | v1.6 proved the scoped sync-to-tip and stay-current claim, so the next highest-leverage work is multi-day stability, bounded resources, recovery diagnosis, progress guarantees, and support evidence before production-node expansion | Shipped in v1.7 with opt-in UAT and deterministic release-boundary checks |
| Scope v1.8 to production full-node readiness boundary | v1.7 left production-node readiness deferred, so the next safe step is defining support, upgrade, service, runbook, release-readiness, and evidence gates before any production claim | Planned |

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

## Historical Context

<details>
<summary>Archived milestone planning context</summary>

- v1.0 archive: `.planning/milestones/v1.0-ROADMAP.md`, `.planning/milestones/v1.0-REQUIREMENTS.md`, `.planning/milestones/v1.0-MILESTONE-AUDIT.md`
- v1.1 archive: `.planning/milestones/v1.1-ROADMAP.md`, `.planning/milestones/v1.1-REQUIREMENTS.md`, `.planning/milestones/v1.1-MILESTONE-AUDIT.md`
- v1.2 archive: `.planning/milestones/v1.2-ROADMAP.md`, `.planning/milestones/v1.2-REQUIREMENTS.md`, `.planning/milestones/v1.2-phases/`
- v1.3 archive: `.planning/milestones/v1.3-ROADMAP.md`, `.planning/milestones/v1.3-REQUIREMENTS.md`, `.planning/milestones/v1.3-MILESTONE-AUDIT.md`
- v1.4 archive: `.planning/milestones/v1.4-ROADMAP.md`, `.planning/milestones/v1.4-REQUIREMENTS.md`, `.planning/milestones/v1.4-MILESTONE-AUDIT.md`
- v1.5 archive: `.planning/milestones/v1.5-ROADMAP.md`, `.planning/milestones/v1.5-REQUIREMENTS.md`, `.planning/milestones/v1.5-MILESTONE-AUDIT.md`
- v1.6 archive: `.planning/milestones/v1.6-ROADMAP.md`, `.planning/milestones/v1.6-REQUIREMENTS.md`
- v1.7 archive: `.planning/milestones/v1.7-ROADMAP.md`, `.planning/milestones/v1.7-REQUIREMENTS.md`, `.planning/milestones/v1.7-MILESTONE-AUDIT.md`
- Raw phase execution history for v1.0, v1.3, v1.4, v1.5, v1.6, and v1.7 remains in `.planning/phases/`, while the v1.1 and v1.2 phase histories live in `.planning/milestones/`.

</details>

---
*Last updated: 2026-06-20 after starting v1.8 milestone planning*
