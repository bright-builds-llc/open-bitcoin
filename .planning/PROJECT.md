# Open Bitcoin

## What This Is

Open Bitcoin is a Bitcoin node and wallet implementation in Rust, built to preserve externally observable behavior from Bitcoin Knots `29.3.knots20260210` where a behavior is in scope. Through the shipped v2.4 milestone, the project includes a headless parity baseline, a terminal-first operator surface, opt-in public-mainnet sync and inbound serving, bounded transaction relay, default-off block serving and compact-block relay, the scoped v2.2 package and long-lived mempool surface, and the scoped v2.3 durability surface: disk-backed per-outpoint coins, typed cache-flush policy, fuller chainstate-manager behavior for the single active chainstate, and honest stored-block availability that serves or reports a stored block only when the payload bytes are present. v2.4 adds single-chainstate height-window pruning of paired Fjall block/undo keys, ordinary automatic retention, limited serving, earned Pruned labels, durable locks, post-prune wallet eligibility, and sanitized operator evidence. The automatic target is a soft logical payload-retention target, not a physical disk-capacity guarantee. Archive-node or production-scale historical serving, assumeutxo, public/default relay, public-network CI, production service operation, production-funds wallet use, and production full-node readiness remain deferred.

It is for contributors and operators who want a reference-grade node with a cleaner, more type-safe internal architecture, auditable parity, and a strict separation between pure domain logic and effectful adapters.

## Core Value

When a behavior is in scope, Open Bitcoin must behave like the pinned Knots baseline on the outside while staying simpler and safer on the inside.

## Current State

Phase 159 completed authenticated BASIC filter/index RPCs on 2026-10-10 (UTC). [Verification](phases/159-authenticated-basic-filter-and-index-rpcs/159-VERIFICATION.md) passed 26/26 truths and all four roadmap criteria; full default native verification passed in 36m 0.155s with 3,864 primary Rust passes, benchmark smoke, six Bazel targets/provenance and zero uncovered pure-core lines. Independent review is clean across 91 paths/all seven findings closed; all 27 security mitigations are closed. CFRP-01/02 are Complete. Configured authenticated queries share the durable node authority, preserve exact BASIC results/error ordering and initial-sync semantics, and serve retained active/stale/pruned records after genuine paired deletion and all-handle reopen. UnknownLegacy absent rows fail closed; numeric read/waiter policies and synthetic genesis/maturity-one/manual-owner-plan limits remain explicit. Phase 160 is ready for context; peer/operator/integrated-client products and v2.5 completion remain pending.

Earlier entries below preserve the scope and next-step status at their original verification dates.

Phase 158 completed internal validated BASIC branch replacement on 2026-10-09. [Verification](phases/158-validated-reorg-and-retained-branch-identity/158-VERIFICATION.md) passed 21/21 truths and all three roadmap criteria; the default native contract passed in 19m24.571s with 3,735 primary Rust tests/doctests, benchmark smoke, six Bazel targets/provenance and no uncovered pure-core lines. Independent source review is clean, both review findings are fixed and all 25 declared mitigations are closed. CFIX-03 is Complete. Genuine accepted receipts and positions enforce common-ancestor replacement, immutable displaced hash lookup after actual reopen, conservative coins fencing and explicit required-body/undo refusal. A retained shorter unflushed return regression and missing/mismatched fence controls pass. Existing finite caps can refuse fully retained forks; maturity/halving and software-only fault limits remain explicit. Phase 159 is ready for context; RPC/peer/operator/integrated-client products remain pending.

Phase 157 completed safe BASIC activation and scheduled catch-up on 2026-10-05. [Verification](phases/157-safe-activation-and-scheduled-index-catch-up/157-VERIFICATION.md) passed 34/34 truths and all four roadmap criteria; the complete default native contract passed in 17m22.546s with Bun 1.3.9/Rust 1.94.1, including 3604 primary Rust tests, benchmarks, Bazel/provenance and zero uncovered pure-core lines. Independent source review is clean across 91 files; all 33 declared mitigations are closed. CFAC-01/02 and CFIX-01 are Complete. Explicit bare/1/basic or zero selects configured durable startup without P2P activation; full required-history preflight precedes prune effects, and actual startup/one-second idle turns share the ordered accepted-state owner. Real paired loss/refusal, persistence faults, immutable retries and normal checkpoint/714 paired-delete evidence preserve conservative progress. Enabled empty storage still requires retained validated genesis/history; exact-tip retention, representation/resource limits and the existing three v2.4 advisories remain. Phase 158 subsequently completed internal validated reorg; filter/index RPC, peer serving, operator projections and integrated retained-client proof remain pending.

Phase 156 completed internal index-owned manual and automatic prune coordination on 2026-10-05. [Verification](phases/156-index-owned-manual-and-automatic-prune-coordination/156-VERIFICATION.md) passed 28/28 must-haves and all four roadmap criteria; the default full native verifier passed in 32m 37.914s with Bun 1.3.9 and Rust 1.94.1. Source review is clean across 67 files plus two generated locks, and all 27 declared security mitigations are closed. CFPR-01 is Complete. Actual legal-target daemon retention, 714 paired deletes, lifecycle/work fencing, authenticated ownership refusal and real reopen provide scoped software evidence. Large fixtures are codec-valid; small accepted engine histories use custom maturity. Phases 157–158 subsequently completed scoped activation/catch-up and internal validated reorg; filter/index RPC, peers, operator projections and integrated client proof remain pending.

Phase 155 completed recoverable immutable BASIC storage and pre-prune startup protection on 2026-10-04. [Verification](phases/155-recoverable-index-and-pre-prune-startup-protection/155-VERIFICATION.md) passed 10/10 distinct truths and all four roadmap criteria; the full default native verifier passed in 50m 23.935s with pinned Bun 1.3.9 and Rust 1.94.1. Source review is clean across 36 files and all 17 declared threats are closed. CFIX-02/04 and CFPR-03 are Complete. Real Fjall/runtime reopen, validated spend/fork and coins/metadata failure tests prove conservative progress and protection. Phases 156–158 subsequently completed ordinary ownership, explicit activation/catch-up and internal validated reorg; RPC/peer serving and the rest of v2.5 remain pending.

Phase 154 completed the v2.5 BASIC construction and commitment foundation: exact pinned bytes/hash/header, complete body-bound historical inputs and independent corpus/edge/validated-chain proof. Its [verification](phases/154-basic-generation-and-commitment-parity/154-VERIFICATION.md) passed 11/11 must-haves and all three roadmap criteria; the full default verifier passed in 31m 19.275s and all 15 declared threats are closed. CFIL-01/02 remain Complete.

v2.4 Prune-Mode Product Behavior shipped and was archived on 2026-10-03 after Phases 146–153 completed 34/34 plans and all 17 requirements. The [archived audit](milestones/v2.4-MILESTONE-AUDIT.md) records 20/20 connected seams, 10/10 scoped flows, zero blocking gaps and three accepted nonblocking advisories. Its `tech_debt` status preserves those advisories rather than claiming they were fixed.

The repository now includes durable Fjall-backed runtime storage, disk-backed per-outpoint coins, typed cache-flush policy, a single-chainstate manager that restarts from coins best-block, honest stored-block availability, the terminal-first operator surface, opt-in inbound serving and transaction relay, validated block serving, compact-block relay, bounded local package admission, same-peer 1P1C assembly, accounted-memory pressure and rolling-fee decay, source-only mempool snapshot recovery, receive-independent initial-broadcast retry, sanitized operator evidence, and last-gate claim guardrails.

The shipped prune surface selects disabled, manual-only or automatic mode with a minimum 550 MiB target; preserves the last 288 blocks, network prune-after threshold and ten-block lock buffer; and applies paired durable deletes through one serialized owner. Nonempty automatic plans force the existing full coins/chain-metadata checkpoint. Explicit prune configuration with a datadir selects recovered durable storage without activating networking. `NODE_NETWORK_LIMITED` advertisement and the 288+2 serving window keep removed bodies unavailable, while `Pruned` requires durable have-pruned plus absent payload bytes. Wallet replacements check requested and selected creating payloads before persistence and preserve prior wallet/checkpoint state on refusal.

The 2026-10-02 audit found two product integration gaps despite the original phase reports passing; its [historical report](reports/v2.4-MILESTONE-AUDIT-2026-10-02-073cf664.md) is preserved. Phase 152 closed post-prune wallet creating-payload eligibility (SNAP-01). Phase 153 closed ordinary automatic retention (PRUN-01/PRUN-02); its [verification](phases/153-automatic-prune-retention-integration/153-VERIFICATION.md) passed 22/22 truths. The default full native verifier passed in 45m47.953s and the feature-finalization commit hook passed in 37m21.068s. Independent source review was clean across 26 files and all 14 declared security mitigations closed. These are recorded local results, not new public-network or CI evidence.

The accepted advisories are stale durable metadata causing permitted finish-or-Repair refusal after interrupted pruning, the generic paired-unlink sink's no-op default despite concrete Fjall overrides, and support counters undercounting a crash between unlink and summary persistence. Logical accounting includes protected and nonactive payloads while only active heights are deletion candidates. Wallet probes and persistence remain separate effects; there is no atomic presence-at-save guarantee for concurrent callers, including automatic maintenance. Sparse codec-valid fixtures and a later metadata-fault fixture with `MemoryCoinsView` do not prove continuous consensus-chain acceptance, hardware resilience or a second production durable-coins fault checkpoint.

Historical phase directories remain tracked because repository verifiers reference selected evidence. The [archived roadmap](milestones/v2.4-ROADMAP.md) and [requirements](milestones/v2.4-REQUIREMENTS.md) preserve the completed scope.

## Current Milestone: v2.5 Prune-Aware Compact-Filter Serving (BIP157/158)

**Goal:** Let lightweight clients fetch Knots-compatible basic compact filters from an explicitly enabled node, with durable indexing that remains correct across pruning, restart and reorgs.

**Target features:**

- Deterministic BIP158 basic filters, filter hashes and BIP157 filter headers with pinned Knots parity evidence
- Durable filter indexing, catch-up, restart and reorg handling coordinated with the existing single-chainstate prune owner and locks
- Explicit, bounded compact-filter P2P serving and truthful service advertisement, with missing-history and index-readiness outcomes
- Operator index status and sanitized support evidence, plus deterministic generation/persistence/prune/restart/serving proof

Continue at Phase 160 and preserve earlier phase directories required by repository verifiers. Phases 157–158 completed explicit activation, bounded catch-up and internal validated reorg with missing-history refusal. Filters must be persisted before dependent history can be deleted; missing history cannot silently become a complete index. Review the three accepted v2.4 advisories where they affect index/prune coordination without promising unrelated cleanup.

FUT-20 compact-filter serving is selected for v2.5. BIP37 bloom serving, assumeutxo/dual chainstate, archive-scale serving, public defaults, public-network CI gates and production/funds claims remain deferred. v2.5 is planned scope, not a shipped capability.

## Latest Completed Milestone: v2.4 Prune-Mode Product Behavior

**Status:** Shipped and archived on 2026-10-03 after both audit integration gaps closed.

**Goal:** Add Knots-aligned prune product behavior for the single active chainstate, so the node can delete old block and undo payloads by removing Fjall keys inside a height window and still tell the truth about what remains.

Initialized through `/gsd-new-milestone` after the archived v2.3 closeout; all 17 scoped requirements are now validated.

**Shipped features:**

- Height windows, paired Fjall key unlinking, `m_have_pruned`, and durable prune locks
- `NODE_NETWORK_LIMITED` serving limits for the pruned window
- Emit `Pruned` only after a durable delete; keep `Unavailable` for a missing payload without have-pruned
- Coins-backed staged wallet replacement with requested/creating-payload eligibility and truthful refusal checkpoints
- Exact logical payload accounting, ordinary automatic retention, full checkpoint/reopen and receipt-owned cache/undo cleanup
- RPC, CLI, read-only dashboard and redacted support evidence with pinned parity and no-claim guardrails

<details>
<summary>Previous shipped milestone: v2.3 Chainstate Durability and Historical Serving</summary>

## Completed Milestone: v2.3 Chainstate Durability and Historical Serving

v2.3 Chainstate Durability and Historical Serving shipped and was archived on 2026-09-20 after Phases 139–145 completed 29/29 plans and all 15 requirements. The final audit passed with 8/8 production seams, 8/8 end-to-end flows, and no blocking gaps.

**Status:** Shipped and archived on 2026-09-20 after Phase 145 closed parity, UAT, and no-claim guardrails.

**Goal:** Replace snapshot-style coin persistence with disk-backed coins, cache-flush policy, and fuller chainstate-manager behavior, while keeping block-serving honest about what is actually stored.

**Shipped features:**

- Typed DIRTY/FRESH coins overlay so connect, disconnect, and reorg do not clone or rewrite the whole UTXO set.
- I/O-free flush and recovery decisions (`IfNeeded`, `Periodic`, `Always`, disk-space refusal) executed by one shell-owned manager.
- Per-outpoint Fjall coins records with best-block and interrupted-flush markers; leftover snapshot blobs are non-authoritative.
- Same-datadir restart from durable coins best-block, with interrupted-flush replay or fail-closed recovery.
- Payload-byte honest availability: Available only when bytes are present; missing payloads refuse as Unavailable.
- Sanitized flush, recovery, cache-size, and have-bytes versus do-not evidence plus a last-gate D-14/D-16 claim checker.

</details>

## Completed Milestone: v2.2 Package Relay and Long-Lived Mempool Policy

**Status:** Shipped and archived on 2026-08-22 after Phase 138 closed parity, UAT, restart, and release-boundary guardrails.

At the v2.2 archive boundary, disk-backed coins databases, cache-flush policy, fuller chainstate-manager behavior, prune/archive modes, compact-filter serving, public relay defaults, public-network CI, production full-node readiness, and production-funds wallet use remained deferred. v2.3 later shipped only the storage-first chainstate-durability and honest-availability portion of that inventory.

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

- ✓ CFIX-02, CFIX-04 and CFPR-03 validated in Phase 155: integrity-checked immutable Fjall records, recovered-coins/ancestry-fenced progress, atomic publication, conservative faults and mandatory protection before startup prune resume; full native verification passed.

- ✓ CFIL-01/02 validated in Phase 154: pinned BASIC bytes, typed hash/header commitments, validated historical/same-block inputs, missing/body-substitution refusal and independent parity vectors; full native verification passed.

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

- ✓ v2.3 validated all 15 chainstate-durability and honest-availability requirements across coins overlay/cache, typed flush policy, durable Fjall coins, manager flush/restart, payload-byte serving, sanitized operator evidence, and last-gate no-claim guardrails. Archive: `.planning/milestones/v2.3-REQUIREMENTS.md`

- ✓ v2.4 validated all 17 prune requirements across wallet eligibility, mode/automatic/manual policy, paired durable unlink/recovery, buffered durable locks, limited serving, honest labels, operator/support evidence and claim guardrails. Archive: `.planning/milestones/v2.4-REQUIREMENTS.md`

- ✓ PRUN-01/PRUN-02 validated in Phase 153 after the Phase 147 foundations: configured mode drives measured ordinary durable retention, honors keep/lock/network thresholds and forces a full checkpoint for nonempty plans.

- ✓ PRUN-03/LOCK-01 validated in Phase 147: manual keep-window refusal and ten-block buffered lock protection.

- ✓ OPER-01/OPER-02/OPER-03/LOCK-02 validated in Phase 150: shared status, manual requests, durable lock CRUD and sanitized support counts.

- ✓ GRD-01 validated in Phase 151: pinned prune anchors, Fjall-versus-flat-file difference and deterministic no-claim guardrails.

- ✓ SNAP-01 validated in Phase 152 after the Phase 146 foundations: both durable rescan adapters check every replacement entry's creating payload, preserve prior wallet/checkpoint state on refusal and ignore leftover snapshot authority.

- ✓ UNLK-01, UNLK-02, and UNLK-03 validated in Phase 148: paired Fjall unlink, have-pruned only after a durable delete, and finish-or-refuse restart.

- ✓ SERV-01, SERV-02, SERV-03, and LABL-01 validated in Phase 149: limited-service advertisement, out-of-window and removed-payload refusal, and an earned `Pruned` label.

- ✓ CFPR-01 validated in Phase 156: internally owned protection, fresh manual/automatic deletion checks, exact work fencing and ordered disable/re-enable; scoped full native and lifecycle verification passed.

- ✓ CFAC-01/02 and CFIX-01 validated in Phase 157: exact default-off options, configured pre-prune history refusal, actual bounded startup/idle maintenance and accepted-before-persist handoff;34/34 formal truths, full native, source and security verification passed.

- ✓ CFIX-03 validated in Phase 158: genuine branch replacement, immutable displaced lookup after reopen and explicit required-source refusal; 21/21 truths, full native and all 25 security mitigations passed.

- ✓ CFRP-01/02 validated in Phase 159: authenticated retained BASIC lookup and exact initial-sync index summaries; 26/26 formal truths, full native, clean source review and 27/27 security mitigations passed.

### Active
- [ ] CFPR-02: Retained filter service after pruning
- [ ] CFNET-01 through CFNET-06: Explicit bounded BIP157 serving with branch-correct ranges and achieved-write evidence
- [ ] CFOP-01: Shared sanitized operator evidence and separate filter-growth disclosure
- [ ] CFGR-01/02: Parity guardrails and continuous validated-chain integrated proof

CFIL-01/02, CFIX-01/02/03/04, CFAC-01/02, CFPR-01/03 and CFRP-01/02 are Complete and ten detailed requirements remain Pending in [REQUIREMENTS.md](REQUIREMENTS.md), mapped once across Phases 154–162. All v2.4 requirements remain Validated.

### Out of Scope

The v2.5 boundary selects compact-filter serving while keeping archive-node product modes, assumeutxo and IBD snapshot shortcuts, BIP37 serving, general package wire, public relay defaults, public-network CI, production full-node readiness, and production-funds wallet use deferred. The shipped v2.4 prune contract remains the foundation.

- Faithful Qt GUI parity or porting the upstream GUI code - shipped milestones remain terminal-first and headless.
- Windows service integration - still deferred until a later milestone.
- Automatic destructive migration of existing Bitcoin Core or Bitcoin Knots data - migration must be dry-run-first, explicit, and backup-aware.
- Broad unsupported drop-in replacement claims beyond the audited evidence surface - parity claims remain scoped to shipped artifacts and documented deviations.
- Public marketing sites or hosted dashboards - completed milestones prioritize local operator surfaces and node correctness.
- Replacing `bitcoin.conf` compatibility with an Open Bitcoin-only config format - JSONC layers on top of, not instead of, baseline config behavior.
- Production full-node readiness, production-funds wallet use, migration apply mode, signed packaging, hosted dashboards, GUI parity, public-network CI, destructive repair, automatic support-bundle upload, and release-blocking live sync - these remain deferred to future milestones.
- assumeutxo, assumevalid, and IBD snapshot shortcuts - v2.3 is durability and honest availability, not a sync-speed milestone.
- Archive-mode product behavior, including archive-node or production-scale historical serving - honesty about stored bytes is not an archive claim, and archive serving is the opposite operator problem from prune.
- Public relay by default or unbounded public-network relay participation - v2.0 should keep relay activation scoped, observable, and evidence-backed until a later production-readiness milestone deliberately changes that boundary.
- Public compact-block relay defaults or production-scale block-serving claims - v2.1 should keep block-serving and compact-block relay scoped, observable, and evidence-backed until production-readiness and public-default requirements deliberately change that boundary.
- Public inbound serving by default - inbound participation remains opt-in unless a later milestone deliberately changes that boundary with evidence.
- Broad address relay network participation - v1.9 scoped listener advertising and bounded `getaddr` response behavior, but full address-relay parity remains a future claim unless requirements expand explicitly.
- Claiming v1.8 as production full-node ready by default - this milestone defines gates and guardrails before such a claim is allowed.

## Context

- The repository has first-party pure-core domain and codec crates under `packages/`, plus parity catalog artifacts under `docs/parity/`.
- Bitcoin Knots `29.3.knots20260210` is the pinned behavioral reference baseline.
- The pre-archive v2.4 LOC report totals 355,534 tracked first-party lines, including 308,584 code/content lines; production Rust is 118,495 physical / 102,749 code lines, and test Rust is 141,316 physical / 119,570 code lines. The tracked report refreshes during verification.
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
- v2.3 chainstate durability reused the existing pure-core UTXO engine and node-side snapshot adapter, then added disk-backed coins, cache-flush policy, and manager behavior while citing pinned Knots `coins.h`, `coins.cpp`, `validation.cpp`, `node/chainstate.cpp`, and `node/blockstorage.cpp` anchors or documenting intentional differences.
- v2.4 prune-mode work builds on that honest-availability and durable-coins foundation. Cite pinned Knots prune anchors (`-prune`, block-file unlinking, `m_have_pruned`, prune locks, `NODE_NETWORK_LIMITED`) or document intentional differences in `docs/parity/`. Functional-core crates stay I/O-free, and historical `.planning/phases/` directories stay tracked.

## Constraints

- **Behavioral baseline**: Match Bitcoin Knots `29.3.knots20260210` for all in-scope surfaces - parity claims must be auditable.
- **Architecture**: Follow functional core / imperative shell boundaries - pure business logic stays free of direct I/O and runtime side effects.
- **Dependency policy**: Keep dependencies minimal and security-conscious, and do not use existing Rust Bitcoin libraries in the production path - the project owns its own domain model and implementation surface.
- **Build tooling**: Use Bazelisk and Bazel with Bzlmod for first-party workspace builds - multi-package growth should remain manageable from the repo root.
- **Verification**: Enforce formatting, linting, build, testing, coverage, architecture-policy, panic-site, parity-breadcrumb, and benchmark checks through repo-native verification.
- **Scope**: Completed milestones are headless and terminal-first; future GUI work must be planned explicitly.

## Key Decisions

| Decision | Rationale | Outcome |
| -- | -- | -- |
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
| Scope v2.3 to chainstate durability and honest historical availability | After v2.2, disk-backed coins, cache-flush policy, and fuller chainstate-manager behavior are the missing foundation; prune/archive modes, assumeutxo, compact filters, and production claims stay later | Shipped and archived on 2026-09-20 with 15/15 requirements, 8/8 seams, and 8/8 flows passing |
| Scope v2.4 to prune-mode product behavior | v2.3 shipped honest availability and durable coins so paired keys can be deleted without lying about payload presence; archive serving, assumeutxo, public defaults, and production claims stay later | ✓ Good: shipped and archived on 2026-10-03 with 17/17 requirements, 20/20 seams and 10/10 flows; three nonblocking advisories accepted |
| Keep prune policy pure and concrete deletion in Fjall adapters | Typed height/lock decisions reuse one owner; paired SyncAll deletion earns have-pruned and cleanup receipts | ✓ Good: real deletion, error cleanup and reopen verified; generic no-op sink default remains an accepted advisory |
| Refuse manual targets inside the keep window instead of Knots RPC clamping | Explicit refusal preserves the documented operator contract and recent history | ✓ Good: PRUN-03 validated and intentional difference recorded |
| Stage full wallet replacements and gate requested plus creating payloads | Existing full UTXO replacement needs older matching creating data; leftover snapshots must not restore missing history | ✓ Good: SNAP-01 closed after real prune/resume/reopen; atomic probe/save remains outside the guarantee |
| Measure complete logical payload usage and select only active candidates | Protected/nonactive bytes must count without becoming eligible history; snapshots and metadata are not payload usage | ✓ Good: legal 550 MiB fixture measured 578,359,864 bytes and proved actual ordinary deletion; target remains soft |
| Reuse serialized ordinary flush activity and force full checkpoints for nonempty plans | Avoid a second retention worker, stale candidate reuse or split lock authority; offline explicit mode still needs recovery | ✓ Good: cadence, durable locks, configured offline startup and production checkpoint/reopen verified |
| Keep support counters separately persisted and sanitized | Counters describe earned deletes without exposing backend paths or lock names | ⚠ Revisit: crash between durable unlink and summary persistence can undercount; retries do not invent success |
| Preserve finish-or-Repair refusal for stale interrupted-prune metadata | Recovery must not invent blocks, reindex or silently mutate authority | ⚠ Revisit: accepted UNLK-03 advisory, not proof every crash window is eliminated |
| Scope v2.5 to BASIC compact-filter indexing and explicit serving | Shipped prune locks and honest availability now support an independent retained filter index; V0, BIP37, archive scale and production claims remain separate | In progress: ten requirements Complete through Phase 158; 12 Pending; serving not shipped |
| Refuse missing-history activation and protect index inputs before prune recovery | Pruned body/undo cannot be reconstructed from current coins; startup can otherwise resume deletion before manager construction | Phases 155–157 verified fenced recovery, pre-prune protection, ordinary ownership and explicit activation/catch-up with real faults/reopen |
| Preserve Knots enablement-based filter capability advertisement | Capability and initial/current index progress differ; complete indexed ranges can be served during catch-up | Planned: exact BASIC/per-peer behavior plus distinct operator evidence |

## Evolution

This document evolves at phase transitions and milestone boundaries.

**After each phase transition** (via `/gsd-transition`):

1. Requirements invalidated? -> Move to Out of Scope with reason
1. Requirements validated? -> Move to Validated with phase reference
1. New requirements emerged? -> Add to Active
1. Decisions to log? -> Add to Key Decisions
1. "What This Is" still accurate? -> Update if drifted

**After each milestone** (via `/gsd-complete-milestone`):

1. Full review of all sections
1. Core Value check - still the right priority?
1. Audit Out of Scope - reasons still valid?
1. Update Context with current state

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
- v2.3 archive: `.planning/milestones/v2.3-ROADMAP.md`, `.planning/milestones/v2.3-REQUIREMENTS.md`, `.planning/milestones/v2.3-MILESTONE-AUDIT.md`
- v2.4 archive: `.planning/milestones/v2.4-ROADMAP.md`, `.planning/milestones/v2.4-REQUIREMENTS.md`, `.planning/milestones/v2.4-MILESTONE-AUDIT.md`
- Active phase execution directories are created under `.planning/phases/` during an active milestone. Historical phase directories remain tracked when verifier scripts depend on them. Archived roadmap, requirements, audit, and the v1.1/v1.2 raw phase archives remain under `.planning/milestones/`.

</details>

______________________________________________________________________

*Last updated: 2026-10-10 after Phase 159 verification*
