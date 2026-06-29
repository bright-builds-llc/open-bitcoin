# Milestones: Open Bitcoin

## v2.0 Transaction Relay and Mempool Participation Boundary (Active: started 2026-06-29)

**Planned:** Bounded transaction relay and mempool participation through explicit activation, permission semantics, txid/wtxid inventory, bounded transaction download, orphan handling, mempool admission and lifecycle evidence, relay serving/fanout, operator surfaces, support redaction, parity traceability, and deterministic release-boundary guardrails.

**Phases planned:** 7 phases, Phase 100 through Phase 106, covering 32 scoped requirements.

**Key planned outcomes:**

- Define default-off transaction relay activation and scoped effects for `relay`, `forcerelay`, and `mempool` without activating bloom/filter or compact-block behavior.
- Add txid/wtxid transaction inventory, request, response, duplicate, mismatch, timeout, `notfound`, and disconnect cleanup policy.
- Add bounded transaction download and missing-parent/orphan handling that preserves v1.9 resource-governance limits.
- Add a stable mempool admission, removal, pressure, chainstate, persistence, and restart recovery outcome contract.
- Add relay serving, announcement, fanout, local submission, and rebroadcast evidence without guaranteeing public propagation.
- Align RPC, CLI, dashboard, metrics, logs, and support bundles around shared redacted relay/mempool status.
- Close the milestone with Knots parity anchors, repo-local UAT commands, deterministic no-claim checkers, and docs that keep public relay defaults, compact blocks, package relay, public-network CI, production service operation, production full-node readiness, and production-funds wallet use deferred.

**Current artifacts:**

- `.planning/REQUIREMENTS.md`
- `.planning/ROADMAP.md`
- `.planning/research/SUMMARY.md`

**What's next:** Discuss Phase 100 with `/gsd-discuss-phase 100`.

***

## v1.9 Inbound Peer Serving and Network Participation Boundary (Shipped: 2026-06-29)

**Delivered:** Opt-in inbound peer serving and network participation evidence through disabled-by-default listener activation, inbound admission, permission classes, bounded address behavior, eviction/ban/misbehavior policy, DoS/resource governance, retained inbound metrics, peer-policy runtime bridging, structured peer-policy logs, and deterministic release-boundary guardrails. The milestone deliberately does not claim transaction relay, compact block relay, mempool propagation, public inbound defaults, production service operation, or production full-node readiness.

**Phases completed:** 10 phases, 56 plans, 81 counted summary tasks

**Key accomplishments:**

- Added explicit opt-in inbound listener and admission policy with deterministic bind/preflight diagnostics, typed inbound peer records, duplicate/self protections, caps, reserved/protected slots, and operator-visible status evidence.
- Added Knots-aligned peer permission and connection-class modeling while keeping relay-like, mempool, filter, and compact-block-style permissions inert labels for v1.9.
- Added scoped listener advertisement, bounded `getaddr` response behavior, learned-address evidence, and release wording that avoids full address-relay or broader discovery claims.
- Added eviction, ban, unban, discourage, misbehavior, reconnect suppression, and peer-policy runtime evidence across status, RPC, CLI, support bundles, metrics, and sanitized structured logs.
- Added inbound DoS/resource governance for malformed messages, payload limits, queues, request caps, backpressure, timeouts, churn, and reconnect limits with deterministic local checkers.
- Added retained low-cardinality inbound metric samples and dashboard/status/support projection without exposing peer identifiers or dynamic metric labels.
- Closed traceability and audit gaps so v1.9 has 28/28 requirements, 10/10 integration categories, and 8/8 cross-phase flows passing.

**Stats:**

- 28/28 v1.9 requirements complete.
- 10 phases, 56 plans, and 81 counted summary tasks complete.
- 175,519 tracked first-party lines in the final LOC report at archive time.
- 312 files changed across the post-v1.8 delivery range, with 61,671 insertions and 1,060 deletions before archive.
- Git range after v1.8 archive: `1edc11e9` -> `49d9b473`.
- Milestone audit status: `passed` with 28/28 requirements, 10/10 integration categories, 8/8 cross-phase flows, and zero critical gaps.
- Full repo-native verification passed during the v1.9 audit and was rerun during archive closeout.

**Archived artifacts:**

- `.planning/milestones/v1.9-ROADMAP.md`
- `.planning/milestones/v1.9-REQUIREMENTS.md`
- `.planning/milestones/v1.9-MILESTONE-AUDIT.md`

**Residual risk:**

- Relay-like permission tokens remain inactive v1.9 labels; broader transaction relay, mempool propagation, and compact-block behavior remain future scope.
- INB-01 through INB-04 have intentionally non-obvious ownership: Phase 90 remains historical implementation evidence, while Phase 98 owns final traceability closure.
- Historical Phase 98-era notes may still mention prior INT-04/FLOW-04 caveats; the archived v1.9 audit supersedes them with no active integration or flow gaps.

**What's next:** v2.0 is active; see `.planning/ROADMAP.md`.

---

## v1.8 Production Full-Node Readiness Boundary (Shipped: 2026-06-25)

**Delivered:** Source-built production readiness boundary documentation, support classification, upgrade and rollback policy, operator runbooks, service-operation expectations, release-readiness evidence, and deterministic claim guardrails. The milestone deliberately does not claim production full-node readiness; it defines the gates and no-claim boundary that future production-readiness work must satisfy.

**Phases completed:** 8 phases, 26 plans, 49 counted summary tasks

**Key accomplishments:**

- Defined canonical production-readiness vocabulary, support terms, claim and evidence gates, and deferred-surface inventory.
- Added support matrix and redacted issue-evidence expectations without broadening production, wallet, relay, migration, packaging, hosted-dashboard, GUI, or CI claims.
- Added source-built upgrade and rollback policy with pre-upgrade evidence, state/schema compatibility, rollback guidance, backup expectations, and no-hidden-mutation boundaries.
- Added operator runbooks and service-operation expectations with repo-local Cargo and Bazel command forms, opt-in UAT posture, and unsupported production-service boundaries.
- Added release-readiness checklist and parity roots mapping all 23 v1.8 requirements to evidence, deterministic checks, residual risk, and no-claim boundaries.
- Added deterministic Phase 88 and Phase 89 claim guardrails covering release/operator docs plus upgrade policy, runbooks, and service expectations, with fixture tests for deferred-surface promotion.
- Closed the milestone audit through Phase 89; all 23 requirements, 8 phases, 23/23 integration checks, and 8/8 flows passed, with only a checker-hardening opportunity as tech debt.

**Stats:**

- 23/23 v1.8 requirements complete.
- 8 phases, 26 plans, and 49 counted summary tasks complete.
- 144,055 tracked first-party lines in the final LOC report at archive time.
- 136 files changed across the v1.8 delivery range, with 26,861 insertions and 159 deletions before archive.
- Git range after v1.7: `2187c8c` -> `1a405d1`.
- Milestone audit status: `tech_debt` with zero requirement, integration, or flow blockers; remaining debt is Phase 87 row-parser hardening.
- Full repo-native verification passed during Phase 89 closeout.

**Archived artifacts:**

- `.planning/milestones/v1.8-ROADMAP.md`
- `.planning/milestones/v1.8-REQUIREMENTS.md`
- `.planning/milestones/v1.8-MILESTONE-AUDIT.md`

**Residual risk:**

- v1.9 later shipped opt-in inbound peer serving and network participation boundaries. Production full-node readiness, relay, production-funds wallet use, migration apply mode, signed packaging, hosted dashboards, GUI parity, public-network CI, destructive repair, automatic support-bundle upload, and production-service claims remain future milestone scope.
- Phase 87 checker hardening remains a non-blocking improvement: parse strict release-readiness checklist rows instead of checking only requirement ID presence.

**What's next:** v1.9 has shipped. Start the next milestone with `/gsd-new-milestone`.

---

## v1.7 Full-Sync Soak and Recovery Hardening (Shipped: 2026-06-20)

**Delivered:** Source-built, explicit opt-in full-sync soak and recovery hardening through durable soak ledgers, bounded resource evidence, typed recovery diagnosis, progress guarantees, support-bundle forensics, opt-in UAT guidance, deterministic release-boundary checks, and closed audit traceability. The shipped scope does not broaden the project into production-node readiness, inbound serving, relay, production-funds wallet use, migration apply mode, packaging, hosted dashboards, GUI work, or public-network default verification.

**Phases completed:** 7 phases, 37 plans, 65 counted summary tasks

**Key accomplishments:**

- Added datadir-owned soak run identity, bounded stop conditions, resume-safe report state, typed stop reasons, and deterministic synthetic soak replay coverage.
- Added resource-bound status, preflight, runtime stop, support, dashboard, docs, and checker coverage for disk, cache, queue, log, metric, peer, and support-evidence pressure.
- Added non-mutating lock and corruption recovery diagnosis with safe retry, read-only inspection, backup-then-rebuild, and stop-and-escalate guidance across status, support, dashboard, and soak evidence.
- Added progress-credit and stall-diagnosis contracts so long-run progress is credited only for validated durable work or stay-current evidence, with operator-visible causes for stalled paths.
- Added redacted support-bundle forensics that reconstruct soak timelines, checkpoint chains, resource pressure, recovery events, peer outcomes, and final verdicts.
- Added opt-in soak UAT commands, release-boundary docs, parity roots, and deterministic Phase 75 through Phase 80 checker coverage while keeping `bash scripts/verify.sh` public-network-free.
- Closed the v1.7 milestone audit through Phase 81 by making RES/REC verification evidence explicit and refreshing stale roadmap, requirements, and release-boundary traceability.

**Stats:**

- 24/24 v1.7 requirements complete.
- 7 phases, 37 plans, and 65 counted summary tasks complete.
- 136,450 tracked first-party lines in the final LOC report at archive time, including 51,585 production Rust lines.
- 219 files changed across the derived Phase 75 through Phase 81 delivery range, with 41,984 insertions and 1,588 deletions before archive.
- Git range before archive: `3c88d2f` -> `3752340`.
- Milestone audit status: `passed` with 24/24 requirements, 7/7 phases, 11/11 integration checks, and 6/6 flow checks.
- Full repo-native verification passed during Phase 81 closeout; the milestone audit validation passed on 2026-06-20.

**Archived artifacts:**

- `.planning/milestones/v1.7-ROADMAP.md`
- `.planning/milestones/v1.7-REQUIREMENTS.md`
- `.planning/milestones/v1.7-MILESTONE-AUDIT.md`

**Residual risk:**

- Public-network multi-day soak and real service-manager evidence remain explicit opt-in UAT, not default deterministic verification.
- Production-node readiness, inbound serving, relay, production-funds wallet use, migration apply mode, automatic destructive repair, automatic support-bundle upload, signed packaging, Windows service support, GUI parity, hosted dashboards, and public-network CI remain future milestone scope.

**What's next:** Start the next milestone with `/gsd-new-milestone`.

---

## v1.6 Mainnet Full-Sync Completion (Shipped: 2026-06-14)

**Delivered:** Source-built, explicit opt-in mainnet full-sync completion evidence through durable active-chain validation, tip tracking, stay-current behavior, reorg and peer recovery, resource-bounded restart/resume, coherent operator observability, redacted support evidence, opt-in UAT, deterministic verification, and scoped release-boundary checks.

**Key accomplishments:**

- Added durable active-chain, UTXO, undo, block-index, and runtime metadata needed for a truthful sync-to-tip claim.
- Exposed best-known tip source, cumulative work, freshness, peer agreement, catch-up, at-tip, stale-tip, recovering, and stay-current states through shared status evidence.
- Hardened reorg, peer rotation, stale in-flight, no-progress, and storage-blocker paths with typed outcomes and deterministic tests.
- Documented and tested long-sync resource bounds, same-datadir restart/resume, storage pressure guidance, and synthetic long-chain behavior.
- Aligned CLI, dashboard, RPC, metrics, structured logs, live-smoke reports, and support bundles around one full-sync truth contract.
- Added opt-in public-mainnet UAT guidance and deterministic verification checkers while keeping `bash scripts/verify.sh` public-network-free.
- Closed release-boundary docs and parity roots around the explicit opt-in full-sync completion claim without broad production-node readiness claims.

**Stats:**

- 26/26 v1.6 requirements complete.
- 7 phases and 27 plans complete.
- Full repo-native verification passed during Phase 74 closeout.

**Archived artifacts:**

- `.planning/milestones/v1.6-ROADMAP.md`
- `.planning/milestones/v1.6-REQUIREMENTS.md`

**Residual risk:**

- Public-network full-sync and stay-current evidence remains operator environment dependent and opt-in.
- Inbound serving, address relay, block serving, transaction relay, compact block relay, production-funds wallet safety, migration apply mode, signed packaging, Windows service support, GUI parity, hosted dashboards, public-network CI, release-blocking live sync, and broad production-node readiness remain future milestone scope.

**What's next:** Start the next milestone with `/gsd-new-milestone`.

---

## v1.5 Unattended Mainnet Node Operation Readiness (Shipped: 2026-06-10)

**Delivered:** Source-built, explicit opt-in unattended mainnet operator-review readiness through bounded daemon sync looping, typed recovery and resource evidence, truthful long-run observability, service lifecycle and restart/resume review, redacted support bundles, an operator compatibility wrapper, and deterministic v1.5 release-boundary checks.

**Phases completed:** 8 phases, 22 plans, 23 requirements

**Key accomplishments:**

- Added an explicit bounded unattended review loop for `open-bitcoind` with durable stop reasons, retry/backoff behavior, pause/resume, and clean shutdown preservation.
- Added typed recovery categories, storage-first recovery precedence, bounded metrics/log/support evidence, and deterministic resource-recovery guardrails.
- Aligned status, dashboard, RPC warnings, metrics, structured logs, and live-smoke snapshots around shared long-run sync truth.
- Added user-scope launchd/systemd lifecycle actions plus same-datadir restart/resume evidence for clean versus unclean shutdown, durable progress, stale in-flight work, and next-action guidance.
- Added redacted v1.5 support evidence and operator review docs with repo-local Cargo and Bazel commands for deterministic and opt-in review workflows.
- Exposed the compatibility transcript harness through `open-bitcoin compatibility harness` with stable JSON and Markdown reports.
- Added a v1.5 threat model, release-readiness matrix, parity roots, and deterministic checker that keeps public-network and real service-manager checks outside default verification.
- Completed UAT for all v1.5 phases and closed the milestone audit with no requirement, integration, flow, or current tech-debt gaps.

**Stats:**

- 23/23 v1.5 requirements complete.
- 8 phases and 22 plans complete.
- 108,164 tracked first-party lines in the final LOC report at archive time.
- 165 files changed across the v1.5 delivery range.
- Git range after v1.4: `e69978d` -> `5e7c413`.
- Milestone audit status: `passed` with zero blockers and `tech_debt: []`.
- Full repo-native verification passed during Phase 67 closeout and the traceability cleanup commit hook.

**Archived artifacts:**

- `.planning/milestones/v1.5-ROADMAP.md`
- `.planning/milestones/v1.5-REQUIREMENTS.md`
- `.planning/milestones/v1.5-MILESTONE-AUDIT.md`

**Residual risk:**

- Public-network long-run checks and real service-manager review remain explicit opt-in UAT evidence, not default deterministic verification.
- Production full-node support, inbound serving, transaction relay, compact block relay, production-funds wallet use, migration apply mode, signed packaging, hosted dashboard, and GUI work remain future milestone scope.

**What's next:** Start the next milestone with `/gsd-new-milestone`.

---

## v1.4 Mainnet IBD Convergence and Peer Compatibility (Shipped: 2026-06-05)

**Delivered:** Opt-in outbound IBD convergence evidence through public peer compatibility, validated header progress, bounded block download/connect progress, same-datadir restart/resume evidence, redacted operator support evidence, and explicit v1.4 threat/release boundaries.

**Phases completed:** 6 phases, 15 plans, 25 tasks

**Key accomplishments:**

- Daemon sync now accepts completed outbound handshakes while rejecting incompatible peers as typed, uncredited outcomes.
- Header sync now has deterministic bounded convergence evidence and opt-in live-smoke first-header-progress reporting.
- Best-chain-only block request tests provide retry-safe in-flight cleanup for `notfound`, disconnect, invalid, and malformed peer paths.
- Peer block responses now produce typed connect/no-credit outcomes before live evidence consumes them.
- Durable sync status now reports downloaded and connected block height/hash evidence while preserving the legacy connected-height alias.
- Live-smoke reports now prove first connected block progress or return a typed block-progress diagnosis.
- Deterministic same-datadir reopen coverage for durable headers, downloaded/connected block hashes, and no duplicate block requests.
- Opt-in two-session same-datadir restart smoke evidence with compact JSON and Markdown report fields.
- Storage-first restart recovery diagnosis plus operator and parity docs for same-datadir resume evidence.
- Deterministic OBS-01 fixture coverage now keeps header, downloaded block, connected block, peer, signal, and latest-error evidence aligned across operator surfaces.
- Redacted support bundles now expose compact v1.4 live-smoke progress, restart, recovery, peer-outcome, and final-status evidence without embedding raw local report artifacts.
- v1.4 operator docs now give repo-local evidence commands, exact report-field pass/fail rules, shared status-truth wording, and credential redaction boundaries.
- v1.4 threat, parity, and release-readiness roots now bound the opt-in outbound IBD evidence claim while preserving v1.3 as historical evidence.
- Default repo verification now enforces the v1.4 release-boundary parity roots while keeping public-network live smoke opt-in only.

**Stats:**

- 22/22 v1.4 requirements complete.
- 6 phases and 15 plans complete.
- 99,241 tracked first-party lines in the final LOC report at archive time.
- Milestone audit status: `tech_debt` with zero blockers; planning traceability was corrected during archive preparation.
- Full repo-native verification passed during Phase 59 closeout and commit hooks.

**Archived artifacts:**

- `.planning/milestones/v1.4-ROADMAP.md`
- `.planning/milestones/v1.4-REQUIREMENTS.md`
- `.planning/milestones/v1.4-MILESTONE-AUDIT.md`

**Residual risk:**

- A future operator workflow could expose the Phase 54 compatibility harness directly through CLI or script glue, rather than requiring contributors to invoke the Rust harness path.

**What's next:** Start the next milestone with `/gsd-new-milestone`.

---

## v1.3 Public Mainnet Sync Proof and Node Hardening (Shipped: 2026-06-02)

**Delivered:** Public-mainnet proof and node-hardening evidence for the explicit opt-in daemon sync workflow, with fresh diagnosed-blocker closeout and no broadened production-node claim.

**Phases completed:** 12 phases, 13 plans, 6 recorded summary tasks

**Key accomplishments:**

- Added opt-in live-mainnet smoke preflight, endpoint outcomes, manual-peer retry evidence, typed no-progress causes, and actionable report output.
- Hardened outbound peer lifecycle behavior with bounded peer counts, retry/backoff, stall handling, peer replacement, and contribution attribution.
- Added resource bounds, durable recovery, invalid-data handling, and truth-aligned status, dashboard, metrics, logs, and RPC-facing sync surfaces.
- Produced redacted support evidence, repo-local operator runbooks, a v1.3 threat model, and explicit release-boundary documentation.
- Closed the fresh-status audit gap and remaining live-evidence debt with Phase 53 schema v2 `openbitcoinsyncstatus` diagnosed-blocker evidence.

**Stats:**

- 22/22 v1.3 requirements complete.
- 12 phases and 13 plans complete.
- 93,773 tracked first-party lines in the final LOC report at archive time.
- Milestone audit status: `ready_for_archive` with zero tracked tech-debt items.
- Full repo-native verification passed during Phase 53 closeout and commit hooks.

**Archived artifacts:**

- `.planning/milestones/v1.3-ROADMAP.md`
- `.planning/milestones/v1.3-REQUIREMENTS.md`
- `.planning/milestones/v1.3-MILESTONE-AUDIT.md`

**Residual risk:**

- Phase 53 did not observe successful public-mainnet header, block, restart/resume, or contribution progress in this environment. The milestone closes through fresh typed diagnosed-blocker evidence (`handshake_failure`) and preserves the next operator action for retrying with a different reachable manual peer.

**What's next:** Start the next milestone with `/gsd-new-milestone`.

---

## v1.2 Full Mainnet Network Syncing (Shipped: 2026-05-23)

**Delivered:** An opt-in `open-bitcoind` public-mainnet initial block download workflow with daemon-owned sync activation, outbound peer lifecycle, validated header and block progress, durable restart/recovery, operator observability and control, live-smoke evidence, and security closeout. The shipped scope does not include production-node operation, production-funds wallet use, inbound serving, transaction relay, or packaged-service hardening.

**Phases completed:** 7 phases, 13 plans

**Key accomplishments:**

- Added explicit daemon mainnet sync activation and preflight while keeping public-network behavior opt-in and outside default hermetic verification.
- Built bounded mainnet peer discovery and outbound lifecycle behavior with resolver injection, retry/backoff, stall handling, and peer telemetry.
- Integrated header-first sync, block download/connect, restart recovery, and reorg-aware durable state transitions for the v1.2 IBD claim.
- Made sync progress and health visible through status, dashboard, metrics, logs, RPC-facing output, and operator pause/resume controls.
- Added opt-in live-mainnet smoke reporting and refreshed operator/parity docs so shipped claims stay bounded and auditable.
- Completed the Phase 41 security-analysis audit with `threats_open: 0`, `needs_phase_count: 0`, and no new security implementation phase required before archive.

**Stats:**

- 26/26 v1.2 requirements complete.
- 7 phase directories archived under `.planning/milestones/v1.2-phases/`.
- 89,674 tracked first-party lines in the final LOC report at archive time.
- Full repo-native verification passed during Phase 41 closeout and again through the Phase 41 UAT commit hook.

**Archived artifacts:**

- `.planning/milestones/v1.2-ROADMAP.md`
- `.planning/milestones/v1.2-REQUIREMENTS.md`
- `.planning/milestones/v1.2-phases/`

**Known gaps:**

- No dedicated `.planning/v1.2-MILESTONE-AUDIT.md` was created before archive. The archive uses Phase 40 live-smoke closeout plus Phase 41 security audit, verification, and UAT as the milestone-closeout evidence trail.

**What's next:** Start the next milestone with `/gsd-new-milestone`.

---

## v1.1 Operator Runtime and Real-Network Sync (Shipped: 2026-04-30)

**Delivered:** A shipped terminal-first operator runtime milestone with durable storage, real-network sync foundations, truthful status and service surfaces, practical wallet workflows, and audited dry-run migration guidance. The shipped scope does not include unattended public-mainnet full sync through `open-bitcoind`.

**Phases completed:** 22 phases, 69 plans, 60 recorded summary tasks

**Key accomplishments:**

- Shipped the `open-bitcoin` operator binary with rich status output, config-path discovery, and idempotent onboarding backed by hermetic binary coverage.
- Added durable Fjall-backed runtime storage, restart recovery, real-network sync foundations, bounded metrics history, and structured log retention.
- Delivered truthful launchd/systemd service lifecycle behavior plus a Ratatui dashboard that reuses the shared status, metrics, logs, and service contracts.
- Expanded the operator-facing wallet surface with preview/confirm send, managed-wallet backup export, wallet freshness reporting, and scoped RPC wallet selection.
- Made migration auditable and dry-run-first with Core/Knots detection, parity-ledger-backed notices, custom source selection, and selected-source service review truth.
- Closed the post-audit cleanup debt with runtime-backed benchmark fidelity, configless live bootstrap truth, operator-surface cleanup, and the `DetectionScan` ownership-model refactor.

**Stats:**

- 44/44 v1.1 requirements complete.
- 330 files changed across the milestone delivery range.
- 42,363 production Rust lines and 83,494 total first-party lines at ship time.
- Full repo-native verification passed on the archive closeout path, including benchmarks and Bazel smoke builds.

**Archived artifacts:**

- `.planning/milestones/v1.1-ROADMAP.md`
- `.planning/milestones/v1.1-REQUIREMENTS.md`
- `.planning/milestones/v1.1-MILESTONE-AUDIT.md`
- `.planning/milestones/v1.1-phases/`

**What's next:** v1.2 Full Mainnet Network Syncing shipped on 2026-05-23; see the entry above.

---

## v1.0 Headless Parity (Shipped: 2026-04-26)

**Delivered:** A headless Rust Bitcoin node and wallet baseline with audited behavioral parity against Bitcoin Knots `29.3.knots20260210` for the scoped v1.0 surfaces.

**Phases completed:** 22 phases, 80 plans, 72 counted summary tasks

**Key accomplishments:**

- Pinned the Bitcoin Knots baseline and built first-party Rust workspace, Cargo, and Bazel/Bzlmod guardrails.
- Implemented typed primitives, serialization, consensus validation, chainstate, mempool policy, networking, wallet, RPC, CLI, and config surfaces for the scoped headless milestone.
- Added reusable parity evidence through fixtures, cross-implementation harnesses, property-style protocol coverage, benchmark smoke reports, and parity checklist documentation.
- Preserved functional-core boundaries with architecture-policy checks and kept persistence, network, process, and runtime effects in adapter-owned surfaces.
- Closed consensus parity gaps for contextual headers, lax DER handling, subsidy-plus-fees reward limits, and MoneyRange fee-boundary behavior.
- Hardened reachable panic-like production paths into typed errors and added a guard against new unclassified panic sites.
- Archived the milestone after the v1.0 audit closed GAP-01 through GAP-04 with no open blockers.

**Stats:**

- 28/28 v1 requirements complete.
- 22 phase directories retained in `.planning/phases/` for raw execution history.
- Archive artifacts written under `.planning/milestones/`.

**Archived artifacts:**

- `.planning/milestones/v1.0-ROADMAP.md`
- `.planning/milestones/v1.0-REQUIREMENTS.md`
- `.planning/milestones/v1.0-MILESTONE-AUDIT.md`

**What's next:** v1.1 and v1.2 have shipped; see the newer entries above.

---
