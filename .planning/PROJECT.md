# Open Bitcoin

## What This Is

Open Bitcoin is a Bitcoin node and wallet implementation in Rust, built to preserve externally observable behavior from Bitcoin Knots `29.3.knots20260210` where a behavior is in scope. Through the shipped v2.2 milestone, the project includes a headless parity baseline, a terminal-first operator surface, opt-in public-mainnet sync and inbound serving, bounded transaction relay, default-off block serving and compact-block relay, and the scoped v2.2 package and long-lived mempool surface: bounded local-package APIs, same-peer 1P1C assembly over ordinary transaction messages, ordinary transaction fanout, and initial-broadcast-retry of locally submitted unbroadcast members. v2.2 does not imply a general package wire protocol, public/default relay, guaranteed propagation, public-network CI, production service operation, production-funds wallet use, or production full-node readiness.

It is for contributors and operators who want a reference-grade node with a cleaner, more type-safe internal architecture, auditable parity, and a strict separation between pure domain logic and effectful adapters.

## Core Value

When a behavior is in scope, Open Bitcoin must behave like the pinned Knots baseline on the outside while staying simpler and safer on the inside.

## Current State

v2.2 Package Relay and Long-Lived Mempool Policy shipped and was archived on 2026-08-22 after Phases 130–138, including inserted 133.1, completed 95/95 plans and all 40 requirements. The final audit passed with 8/8 production seams, 8/8 end-to-end flows, and no blocking gaps.

The repository now includes durable Fjall-backed runtime storage, the terminal-first operator surface, opt-in inbound serving and transaction relay, validated block serving, compact-block relay, bounded local package admission, same-peer 1P1C assembly, accounted-memory pressure and rolling-fee decay, source-only mempool snapshot recovery, receive-independent initial-broadcast retry, sanitized package and mempool evidence, and last-gate claim guardrails.

Milestone v2.3 is active after initialization through `/gsd-new-milestone`. Phase 139 shipped the in-memory DIRTY/FRESH coins overlay and engine apply without cloning the UTXO map. Phase 140 shipped the I/O-free flush and recovery decision machine (`decide_flush` / `decide_recovery`). Phase 141 shipped the durable Fjall coins adapter: per-outpoint `C` records, `B`/`H` markers, schema 1→2 one-way leftover migration, and fail-closed disk reads. Phase 142 shipped manager flush lifecycle and restart from durable coins best-block. Phase 143 shipped honest stored-block availability: Available only after a cache-or-store payload-byte probe, missing payload refuses as Unavailable (not Pruned), and `payload_present` / `index_known` / `validated_on_active_chain` are distinguishable. Historical phase directories remain tracked because repository verifiers reference selected evidence.

## Current Milestone: v2.3 Chainstate Durability and Historical Serving

**Goal:** Replace snapshot-style coin persistence with disk-backed coins, cache-flush policy, and fuller chainstate-manager behavior, while keeping block-serving honest about what is actually stored.

**Target features:**

- Disk-backed coins database and cache-flush policy for the active chainstate.
- Fuller chainstate-manager behavior around that durable coins view.
- Honest availability: serve or report a stored block only when the payload is actually present; refuse cleanly when it is not.
- Operator and parity evidence for the new persistence and availability truth.

## Latest Completed Milestone: v2.2 Package Relay and Long-Lived Mempool Policy

**Status:** Shipped and archived on 2026-08-22 after Phase 138 closed parity, UAT, restart, and release-boundary guardrails.

At the v2.2 archive boundary, disk-backed coins databases, cache-flush policy, fuller chainstate-manager behavior, prune/archive modes, compact-filter serving, public relay defaults, public-network CI, production full-node readiness, and production-funds wallet use remained deferred. v2.3 now activates only the storage-first chainstate-durability and honest-availability portion of that inventory.

**Goal:** Extend the bounded v2.0 relay and mempool foundation with Knots-aligned package admission and opportunistic same-peer 1P1C relay plus durable, observable policy behavior during long-running and sustained-pressure operation.

**Shipped features:**

- Bounded local package dry-run and submit with cheap-first validation, partial acceptance, effective-fee grouping, and selected replacement, TRUC, and ephemeral-dust policy.
- Sender-aware same-peer 1P1C assembly over ordinary transaction messages, routed through one authoritative package engine and lifecycle projector.
- Accounted-memory capacity enforcement, descendant-score eviction, expiry cleanup, and block-gated rolling-fee decay.
- Source-only durable mempool snapshots, fail-closed recovery, rolling-fee reset, and reminted retry timers.
- Receive-independent initial-broadcast retry of locally submitted unbroadcast members and parent-before-child ordinary fanout.
- Redacted RPC, CLI, dashboard, metrics, logs, and support evidence plus a last-gate D-21/D-22 claim checker.

## Completed Milestone: v2.1 Block Serving and Compact Block Relay Boundary

**Status:** Shipped and archived on 2026-07-22 after Phase 129 closed integration guardrails and milestone reconciliation.

v2.1 does not imply public relay defaults, production service operation, production-funds wallet use, public-network CI, or production full-node readiness. Residual v2.1 inventory kept package relay, bloom/filter serving, public relay defaults, public-network CI, production full-node readiness, and production-funds wallet use deferred. v2.2 later shipped only the scoped package-relay and long-lived mempool-policy portion of that inventory.

**Goal:** Add bounded, opt-in block-serving and compact-block relay behavior with auditable Bitcoin Knots parity while preserving deterministic default verification and avoiding public-default or production-readiness claims.

**Shipped features:**

- Default-off validated block serving with deterministic peer eligibility, durable availability checks, request/resource limits, and cleanup policy.
- First-party BIP152 codecs, bilateral compact negotiation, live reconstruction candidates, bounded missing-transaction recovery, timeout fallback, and validation handoff.
- One authoritative network and chainstate view shared by durable sync, inbound serving, RPC, CLI, dashboard, metrics, logs, and support evidence.
- Production compact/header/inventory announcement transport with successful-write-only achieved-effect evidence.
- Sanitized operator evidence, exact Knots parity roots, local UAT commands, and deterministic no-claim and integration guards.

## Completed Milestone: v1.9 Inbound Peer Serving and Network Participation Boundary

**Status:** Shipped and archived on 2026-06-29 after Phase 99 closed peer-policy structured-log audit debt.

**Goal:** Let Open Bitcoin accept and serve inbound peers under explicit admission, permission, address, eviction/ban, and resource-governance rules while keeping relay and production participation claims deferred.

**Shipped features:**
- Opt-in inbound listener and admission path with deterministic limits, handshake lifecycle, and diagnostics.
- Peer permission and connection-class policy aligned to Knots concepts without granting transaction relay, compact block relay, or mempool propagation by accident.
- Address advertisement and peer discovery boundaries that distinguish local listener advertising, `getaddr` response behavior, and broader address relay.
- Eviction, ban, discourage, and misbehavior policy with durable/operator-visible evidence and permission-aware handling.
- DoS and resource governance for inbound sockets, message sizes, queues, timeouts, churn, and observability.
- Retained inbound metrics, peer-policy runtime bridge evidence, automatic sanitized peer-policy structured logs, and release-boundary checks that keep transaction relay, compact blocks, mempool propagation, public inbound defaults, production service operation, and production readiness outside v1.9.

## Completed Milestone: v1.8 Production Full-Node Readiness Boundary

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
- ✓ v1.8 validated all 23 production-readiness boundary requirements across production terminology, support boundaries, upgrade policy, runbooks, service expectations, release-readiness evidence, deterministic claim guardrails, parity roots, and no-claim release boundaries. Archive: `.planning/milestones/v1.8-REQUIREMENTS.md`
- ✓ v1.9 validated all 28 inbound peer serving and network participation boundary requirements across opt-in listener admission, peer permissions, address advertisement, eviction/ban policy, DoS/resource governance, retained inbound metrics, peer-policy runtime evidence, structured logs, traceability closure, and release-boundary no-claim guardrails. Archive: `.planning/milestones/v1.9-REQUIREMENTS.md`
- ✓ v2.0 validated all 32 transaction relay and mempool participation boundary requirements across explicit relay activation, txid/wtxid inventory, bounded download/orphan handling, mempool admission and durable recovery, relay serving/fanout, sanitized operator evidence, parity roots, UAT, and deterministic no-claim guardrails. Archive: `.planning/milestones/v2.0-REQUIREMENTS.md`
- ✓ v2.1 validated all 39 block-serving and compact-relay requirements across explicit activation, validated durable serving, BIP152 codecs and negotiation, reconstruction and fallback, authoritative runtime state, production announcement transport, sanitized operator evidence, parity roots, UAT, and deterministic no-claim/integration guardrails. Archive: `.planning/milestones/v2.1-REQUIREMENTS.md`
- ✓ v2.2 validated all 40 package-relay and long-lived mempool-policy requirements across resource/fee primitives, pressure and expiry, typed package admission, same-peer 1P1C, authoritative lifecycle projection, snapshot recovery, initial-broadcast retry, sanitized operator evidence, and last-gate claim guardrails. Archive: `.planning/milestones/v2.2-REQUIREMENTS.md`

### Active

- [ ] Disk-backed coins databases and cache-flush policy persist and recover the active chainstate without snapshot-only coin truth.
- [ ] Fuller chainstate-manager behavior owns the durable coins view, flush points, and restart-safe cache lifecycle.
- [x] Block serving reports a stored block only when the payload is actually present and refuses cleanly when it is not. Validated in Phase 143: Honest Stored-Block Availability. Full operator flush/availability evidence rollout remains Phase 144.
- [ ] Parity and operator evidence keep the new persistence and availability truth auditable without broadening public or production claims.

### Out of Scope

The boundary keeps prune and archive product modes, assumeutxo and IBD snapshot shortcuts, compact-filter and BIP37 serving, general package wire, public relay defaults, public-network CI, production full-node readiness, and production-funds wallet use deferred beyond v2.3.

- Faithful Qt GUI parity or porting the upstream GUI code - shipped milestones remain terminal-first and headless.
- Windows service integration - still deferred until a later milestone.
- Automatic destructive migration of existing Bitcoin Core or Bitcoin Knots data - migration must be dry-run-first, explicit, and backup-aware.
- Broad unsupported drop-in replacement claims beyond the audited evidence surface - parity claims remain scoped to shipped artifacts and documented deviations.
- Public marketing sites or hosted dashboards - completed milestones prioritize local operator surfaces and node correctness.
- Replacing `bitcoin.conf` compatibility with an Open Bitcoin-only config format - JSONC layers on top of, not instead of, baseline config behavior.
- Production full-node readiness, production-funds wallet use, migration apply mode, signed packaging, hosted dashboards, GUI parity, public-network CI, destructive repair, automatic support-bundle upload, and release-blocking live sync - these remain deferred to future milestones.
- assumeutxo, assumevalid, and IBD snapshot shortcuts - v2.3 is durability and honest availability, not a sync-speed milestone.
- Prune-mode and archive-mode product behavior, including archive-node or production-scale historical serving - later work can add those modes on top of durable coins and honest availability.
- Public relay by default or unbounded public-network relay participation - v2.0 should keep relay activation scoped, observable, and evidence-backed until a later production-readiness milestone deliberately changes that boundary.
- Public compact-block relay defaults or production-scale block-serving claims - v2.1 should keep block-serving and compact-block relay scoped, observable, and evidence-backed until production-readiness and public-default requirements deliberately change that boundary.
- Public inbound serving by default - inbound participation remains opt-in unless a later milestone deliberately changes that boundary with evidence.
- Broad address relay network participation - v1.9 scoped listener advertising and bounded `getaddr` response behavior, but full address-relay parity remains a future claim unless requirements expand explicitly.
- Claiming v1.8 as production full-node ready by default - this milestone defines gates and guardrails before such a claim is allowed.

## Context

- The repository has first-party pure-core domain and codec crates under `packages/`, plus parity catalog artifacts under `docs/parity/`.
- Bitcoin Knots `29.3.knots20260210` is the pinned behavioral reference baseline.
- The current codebase totals 323,209 tracked first-party lines in the v2.2 archive-time LOC report, including 281,974 code/content lines.
- Repo-native verification remains centered on `bash scripts/verify.sh`, including Rust checks, parity breadcrumbs, benchmark smoke and report validation, and Bazel smoke builds.
- Bun is a pinned runtime for repo-owned TypeScript automation, not a package-install surface; there is no `package.json` or `bun install` bootstrap step.
- Operator-facing surfaces should stay quiet, information-dense, and work-focused: terminal dashboard controls, status output, onboarding copy, service actions, and migration guidance should help operators make decisions without marketing language.
- Any migration from Bitcoin Core or Bitcoin Knots must treat the existing datadir and wallet data as high-value user data. Detection and explanation are in scope before automated mutation, while destructive apply-mode work remains deferred.
- First-party code should continue to live in well-bounded packages, with Bazelisk and Bazel/Bzlmod as the top-level build entrypoint unless a later decision replaces that choice.
- The project explicitly avoids existing Rust Bitcoin libraries in the production path and instead exports first-party Rust Bitcoin libraries from this repository.
- Verification must emphasize externally observable parity, pure-core correctness, hermetic integration testing, and contributor guardrails against accidental architectural drift.
- Public-network checks must remain opt-in unless a future milestone deliberately changes the verification contract, so `bash scripts/verify.sh` stays deterministic by default.
- Production full-node readiness must remain a gated claim, not a marketing label. v1.8 should make the boundary explicit across support docs, upgrade policy, runbooks, service expectations, and release-readiness checks.
- v1.9 built inbound peer serving from the existing pure `PeerManager`/`ManagedPeerNetwork` models and added socket/listener behavior in shell-owned runtime adapters, preserving functional-core boundaries.
- Pinned Knots anchors for v1.9 include `net.cpp`, `net_processing.cpp`, `addrman.cpp`, `banman.cpp`, and `net_permissions.cpp`; future network-participation work should cite these anchors or explain intentional behavior differences in `docs/parity/`.
- Future relay, mempool, and peer-participation work should continue citing pinned Knots anchors such as `net_processing.cpp`, `txmempool.cpp`, `validation.cpp`, `policy/`, and related relay tests, or document intentional behavior differences in `docs/parity/`.
- v2.1 block-serving and compact-block relay work should cite pinned Knots anchors for block inventory, `sendcmpct`, `cmpctblock`, `getblocktxn`, `blocktxn`, compact-block reconstruction, block serving, validation, peer state, and resource-governance behavior, or document intentional behavior differences in `docs/parity/`.
- v2.2 package relay and long-lived mempool policy should reuse v2.0 admission, lifecycle, recovery, and relay foundations plus v2.1 authoritative peer transport and observability, while citing pinned Knots package-policy, rolling-fee, rebroadcast, eviction, and mempool-pressure anchors.
- v2.3 chainstate durability should reuse the existing pure-core UTXO engine and node-side snapshot adapter, then add disk-backed coins, cache-flush policy, and manager behavior while citing pinned Knots `coins.h`, `coins.cpp`, `validation.cpp`, and `node/blockstorage.cpp` anchors or documenting intentional differences.

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
| Scope v1.8 to production full-node readiness boundary | v1.7 left production-node readiness deferred, so the next safe step is defining support, upgrade, service, runbook, release-readiness, and evidence gates before any production claim | Shipped in v1.8 with Phase 89 gap closure, deterministic claim guardrails, and a `tech_debt` audit limited to closeout metadata and checker hardening |
| Scope v1.9 to inbound peer serving and network participation boundaries | v1.8 defined claim gates, so the next safe expansion is opt-in inbound serving with admission, permissions, address, eviction/ban, and DoS governance before relay or production participation claims | Shipped in v1.9 with 28/28 requirements, 10/10 integration categories, and 8/8 flows passing; transaction relay, compact blocks, mempool propagation, public inbound defaults, and production readiness remain deferred |
| Scope v2.0 to transaction relay and mempool participation boundaries | v1.9 created opt-in inbound serving and left relay-like permission labels inert, so the next fundamental node capability is bounded transaction relay and mempool propagation before compact blocks or production full-node readiness | Shipped on 2026-07-03 with 32/32 requirements complete through Phases 100 through 108 and Phase 109 archive-readiness audit debt closure |
| Scope v2.1 to block serving and compact block relay boundaries | v2.0 shipped bounded transaction relay and mempool participation, so the next safe node-participation expansion is serving validated blocks and compact-block relay before package relay, public defaults, or production full-node readiness | Shipped and archived on 2026-07-22 with 39/39 requirements, 13/13 integration links, and 11/11 flows passing |
| Scope v2.2 to package relay and long-lived mempool policy | v2.0 established bounded mempool and transaction relay while v2.1 supplied authoritative peer transport and observability, making package policy, rolling fees, rebroadcast, and sustained-pressure behavior the next coherent parity boundary | Shipped and archived on 2026-08-22 with 40/40 requirements, 8/8 seams, and 8/8 flows passing |
| Scope v2.3 to chainstate durability and honest historical availability | After v2.2, disk-backed coins, cache-flush policy, and fuller chainstate-manager behavior are the missing foundation; prune/archive modes, assumeutxo, compact filters, and production claims stay later | — Pending |

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
- v1.8 archive: `.planning/milestones/v1.8-ROADMAP.md`, `.planning/milestones/v1.8-REQUIREMENTS.md`, `.planning/milestones/v1.8-MILESTONE-AUDIT.md`
- v1.9 archive: `.planning/milestones/v1.9-ROADMAP.md`, `.planning/milestones/v1.9-REQUIREMENTS.md`, `.planning/milestones/v1.9-MILESTONE-AUDIT.md`
- v2.0 archive: `.planning/milestones/v2.0-ROADMAP.md`, `.planning/milestones/v2.0-REQUIREMENTS.md`, `.planning/milestones/v2.0-MILESTONE-AUDIT.md`
- v2.1 archive: `.planning/milestones/v2.1-ROADMAP.md`, `.planning/milestones/v2.1-REQUIREMENTS.md`, `.planning/milestones/v2.1-MILESTONE-AUDIT.md`
- v2.2 archive: `.planning/milestones/v2.2-ROADMAP.md`, `.planning/milestones/v2.2-REQUIREMENTS.md`, `.planning/milestones/v2.2-MILESTONE-AUDIT.md`
- Active phase execution directories are created under `.planning/phases/` during an active milestone. Historical phase directories remain tracked when verifier scripts depend on them. Archived roadmap, requirements, audit, and the v1.1/v1.2 raw phase archives remain under `.planning/milestones/`.

</details>

***
*Last updated: 2026-09-17 after completing Phase 143*
