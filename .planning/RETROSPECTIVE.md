# Project Retrospective

*A living document updated after each milestone. Lessons feed forward into future planning.*

## Milestone: v1.0 - Headless Parity

**Shipped:** 2026-04-26
**Phases:** 22 | **Plans:** 80 | **Counted summary tasks:** 72

### What Was Built

- Headless Rust node and wallet implementation scoped to the v1.0 parity surface.
- First-party primitives, codec, consensus, chainstate, mempool, networking, wallet, RPC, CLI, and config crates under the repo workspace.
- Parity evidence through reference fixtures, cross-implementation harnesses, hermetic integration checks, property-style protocol coverage, benchmark smoke reports, and checklist documentation.
- Guardrails for pure-core architecture boundaries, panic-site classification, parity breadcrumbs, repo-native verification, and CI-facing validation.

### What Worked

- Treating Bitcoin Knots as a pinned behavioral baseline kept implementation and audit discussions concrete.
- The GSD phase and plan structure made late audit gaps traceable enough to close without weakening historical evidence.
- Keeping runtime behavior changes separate from planning artifact cleanup made Phase 12 safer to verify.

### What Was Inefficient

- Roadmap and state metadata drifted behind the actual phase summaries and needed a cleanup phase before archival.
- Some early summaries lacked modern frontmatter fields, which made the milestone audit depend on multiple evidence sources.
- Long extracted accomplishment lists needed manual curation for a useful milestone record.

### Patterns Established

- Preserve historical gap reports when they are superseded, and add an explicit closure trail instead of rewriting them.
- Keep parity claims tied to specific artifacts: requirements rows, verification reports, parity catalog pages, and executable checks.
- Use repo-owned automation for broad sweeps such as parity breadcrumbs and panic-site classification, then keep the rule in `scripts/verify.sh`.

### Key Lessons

1. Milestone archives should be created only after roadmap, requirements, and audit metadata agree with executable evidence.
2. Summary frontmatter is part of the project control plane; missing fields create avoidable audit friction later.
3. Broad parity work needs both source anchors and behavior checks, because breadcrumbs help review but do not prove parity by themselves.

### Cost Observations

- Model mix: not measured in repo artifacts.
- Sessions: multiple GSD planning, execution, UAT, security, audit, and archive turns.
- Notable: late artifact reconciliation was cheaper than changing runtime code, but would have been smaller if roadmap progress updates had stayed current after each inserted phase.

---

## Milestone: v1.1 - Operator Runtime and Real-Network Sync

**Shipped:** 2026-04-30
**Phases:** 22 | **Plans:** 69 | **Counted summary tasks:** 60

### What Was Built

- Durable Fjall-backed runtime storage, restart recovery, real-network sync foundations, and bounded metrics and structured logs for operator-facing node runtime work.
- The `open-bitcoin` operator binary with rich status output, config-path discovery, idempotent onboarding, service lifecycle commands, and a Ratatui dashboard.
- Practical wallet-runtime workflows for preview and confirm send, managed-wallet backup export, wallet freshness reporting, and scoped RPC wallet selection.
- An evidence-scoped migration and parity surface with read-only Core or Knots detection, dry-run planning, parity-ledger-backed notices, and selected-source service review truth.
- Post-audit cleanup phases that closed operator-surface truth, benchmark fidelity, configless bootstrap, and migration detection ownership debt before archive.

### What Worked

- The shared status, service, metrics, and migration contracts let later phases and cleanup work repair truthfulness without reopening the whole runtime architecture.
- Repo-native verification plus focused operator-binary tests caught regressions early enough that cleanup phases could stay narrow and auditable.
- Preserving each audit rerun as its own artifact made late closeout decisions easy to justify instead of relying on memory or informal notes.

### What Was Inefficient

- Archive tooling still preferred the older baseline audit and live roadmap shape, so milestone closeout required manual curation instead of a clean one-shot archive.
- Generated LOC bookkeeping resurfaced repeatedly whenever closeout or formatting changed the worktree after a prior refresh.
- Several optional cleanup phases were needed because operator-surface truth and evidence bookkeeping drift were discovered only after broader milestone audit passes.

### Patterns Established

- Keep gap-closure work as explicit cleanup phases with their own verification and audit trail rather than burying archive-readiness fixes inside unrelated commits.
- Treat status, dashboard, service, benchmark, and migration flows as one shared operator surface, with truthfulness checked end to end instead of per-command only.
- Use the parity ledger, requirements ledger, and milestone audits together as a control plane for operator-facing claims.

### Key Lessons

1. Archive tooling should prefer the latest passed audit artifact, not just the oldest canonical filename.
2. Generated reports that participate in verification should be refreshed as the last closeout step after formatting and archive edits settle.
3. Operator-surface milestones benefit from explicit post-audit cleanup phases because truthfulness gaps are usually cross-cutting rather than isolated to one feature.

### Cost Observations

- Model mix: not measured in repo artifacts.
- Sessions: multiple GSD execution, audit, cleanup, and archive turns across the v1.1 milestone.
- Notable: cleanup phases were cheaper than reopening the main milestone scope, but earlier end-to-end operator-flow checks would likely have avoided some late archive work.

---

## Milestone: v1.2 - Full Mainnet Network Syncing

**Shipped:** 2026-05-23
**Phases:** 7 | **Plans:** 13

### What Was Built

- Explicit opt-in `open-bitcoind` mainnet sync activation with daemon-owned durable store preflight and bounded public-network behavior.
- Mainnet peer discovery and outbound lifecycle support with injected resolvers, manual peers, retries, stall handling, and peer telemetry.
- Header-first sync plus block download, validation, connection, durable restart/resume, and reorg-aware recovery for the scoped IBD review claim.
- Operator sync observability and control through status, dashboard, metrics, logs, RPC-facing output, and authenticated daemon RPC pause/resume paths.
- Opt-in live-mainnet smoke reporting, refreshed parity and operator docs, and Phase 41 security-analysis closeout with no new security implementation phase required.

### What Worked

- Keeping live-mainnet evidence opt-in preserved the deterministic `bash scripts/verify.sh` contract while still giving reviewers a real public-network smoke path.
- The Phase 39 UAT rerun surfaced a real daemon-store locking issue, and the fix stayed narrow because the operator control boundary was already explicit.
- Phase 41’s consolidated security audit was a useful closeout gate for checking old STRIDE notes and deferred production-scope claims before archive.

### What Was Inefficient

- The active v1.2 requirements file was not updated continuously as phases shipped, so milestone completion had to reconcile stale checkboxes and `Planned` traceability rows against verification evidence.
- Several phase summaries lacked one-line frontmatter, which made the archive helper produce a sparse milestone accomplishment list that needed manual curation.
- No dedicated `v1.2-MILESTONE-AUDIT.md` was created before archive, so the archive depends on Phase 40 and Phase 41 closeout artifacts instead of a single milestone-audit file.

### Patterns Established

- Treat public-mainnet checks as review evidence, not default release gates, unless a future milestone deliberately expands the verification contract.
- Keep daemon-owned runtime control behind authenticated RPC instead of letting operator commands open a live daemon-owned store from a second process.
- Explicitly list deferred production-node, production-funds, inbound-serving, transaction-relay, and packaged-service claims in parity and security closeout docs.

### Key Lessons

1. Requirements checkboxes and traceability should be updated as each phase completes, not repaired at archive time.
2. Milestone summaries need curated one-liners before archive so automated milestone entries are useful without manual rewriting.
3. If a milestone skips a dedicated audit artifact, the substitute evidence trail should be named explicitly in MILESTONES.md.

### Cost Observations

- Model mix: not measured in repo artifacts.
- Sessions: multiple GSD execution, UAT, security, closeout, and archive turns across the v1.2 milestone.
- Notable: late closeout was mostly documentation and evidence reconciliation, but stale requirement bookkeeping added avoidable archive work.

---

## Milestone: v1.3 - Public Mainnet Sync Proof and Node Hardening

**Shipped:** 2026-06-02
**Phases:** 12 | **Plans:** 13 | **Counted summary tasks:** 6

### What Was Built

- Opt-in public-mainnet smoke evidence with explicit local prerequisites, DNS and manual-peer endpoint outcomes, typed no-progress causes, and local report artifacts.
- Peer lifecycle hardening for bounded outbound peers, retry/backoff, stall handling, replacement behavior, and validation-gated peer contribution attribution.
- Runtime hardening for resource bounds, single-writer durable-store coordination, recovery after partial work, invalid peer data handling, and coherent pause/resume/stop/status flows.
- Operator truth surfaces across JSON status, dashboard, metrics, structured logs, RPC-facing blockchain info, support bundles, and runbooks.
- Reviewer-facing threat model, release-readiness docs, parity roots, milestone audit closeout, and Phase 53 fresh diagnosed-blocker evidence using `openbitcoinsyncstatus` snapshots.

### What Worked

- Treating public-network evidence as opt-in UAT kept `bash scripts/verify.sh` deterministic while still preserving real network diagnostics.
- Phase 51 and Phase 53 made stale evidence debt auditable: historical artifacts stayed intact, and fresh-status supersession was recorded explicitly.
- The support-bundle cleanup in Phase 52 paid off immediately because Phase 53 could summarize schema v2 live-smoke fields without exposing raw report internals.

### What Was Inefficient

- Milestone archive tooling extracted a few noisy accomplishment lines from code-review artifacts, so the v1.3 MILESTONES entry still needed manual curation.
- The complete-milestone helper archived roadmap, requirements, and audit files but left ROADMAP and PROJECT evolution to manual edits.
- Public-network UAT consumed time without producing successful progress, which is acceptable evidence but still slower to reason about than deterministic tests.

### Patterns Established

- Close live-network failures as fresh diagnosed-blocker evidence only when the report is typed, actionable, and tied to fresh daemon sync-control status.
- Keep generated public-network reports under `packages/target` and commit only parsed UAT summaries and traceability paths.
- Preserve non-goal language for production-node, inbound-serving, relay, production-funds wallet, migration apply, packaging, hosted dashboard, and GUI claims.

### Key Lessons

1. Fresh-status evidence should be wired before public-network reruns, otherwise live reports can preserve stale truth even when the daemon is behaving correctly.
2. Support-bundle summaries need schema-aware extraction so reviewer packets stay compact without losing the fields that determine closeout mode.
3. Archive accomplishment extraction needs human review when phase summaries include review findings or bug titles that are not milestone-level achievements.

### Cost Observations

- Model mix: not measured in repo artifacts.
- Sessions: multiple GSD execution, audit, cleanup, live UAT, and archive turns across the v1.3 milestone.
- Notable: explicit diagnosed-blocker acceptance avoided broadening the claim boundary just to make the archive look more successful.

---

## Milestone: v1.4 - Mainnet IBD Convergence and Peer Compatibility

**Shipped:** 2026-06-05
**Phases:** 6 | **Plans:** 15 | **Counted summary tasks:** 25

### What Was Built

- Public peer compatibility diagnosis and daemon sync behavior that records completed outbound handshakes and typed, uncredited peer failures.
- Deterministic bounded header convergence with opt-in live-smoke first-header-progress evidence.
- Bounded block download/connect progress with durable downloaded and connected block height/hash evidence and typed no-credit block response handling.
- Same-datadir restart/resume evidence across deterministic durable-store tests and opt-in two-session live-smoke reporting.
- Operator truth surfaces, redacted support evidence, repo-local UAT/docs, v1.4 threat modeling, parity roots, and deterministic release-boundary checks.

### What Worked

- The phase chain kept public-network behavior opt-in while building deterministic fixtures for the same evidence schema.
- Cross-phase evidence stayed coherent because later operator surfaces reused shared status snapshots and release-boundary checks.
- The milestone audit caught stale planning traceability before archival, so archived requirements and roadmap files now match verified implementation evidence.

### What Was Inefficient

- Requirements and roadmap status drifted during phases 57 through 59 and had to be reconciled at archive time.
- The complete-milestone helper copied active planning files before doing semantic PROJECT and ROADMAP evolution, so manual cleanup was still required.
- The Phase 54 compatibility harness is useful and tested, but not yet exposed through a direct operator CLI or script wrapper.

### Patterns Established

- Treat first-header, first-block, restart/resume, and support-bundle evidence as one schema-backed operator evidence chain.
- Keep public-network live-smoke commands opt-in and prove default verification excludes them through deterministic release-boundary checks.
- Archive planning traceability only after requirements, phase summaries, and verification reports agree.

### Key Lessons

1. Requirements traceability should be marked complete as soon as a phase verification passes.
2. Milestone archive helpers need a post-helper sanity pass for state frontmatter, roadmap collapse, and generated accomplishment formatting.
3. Compatibility diagnostics are more useful when they are both testable as pure harnesses and easy to invoke from operator-facing tools.

### Cost Observations

- Model mix: not measured in repo artifacts.
- Sessions: multiple GSD yolo execution, verification, audit, and archive turns across the v1.4 milestone.
- Notable: the final archive mostly required planning-control cleanup, not runtime code changes, because phase verification had already closed implementation risk.

---

## Milestone: v1.7 - Full-Sync Soak and Recovery Hardening

**Shipped:** 2026-06-20
**Phases:** 7 | **Plans:** 37 | **Counted summary tasks:** 65

### What Was Built

- Durable, datadir-owned soak run identity, ledger, report projection, resume behavior, and typed stop reasons for explicit opt-in multi-day full-sync review.
- Resource-bound contracts, preflight/runtime enforcement, support projection, dashboard/status rendering, docs, and deterministic checker coverage for long-run disk and evidence pressure.
- Non-mutating lock and corruption recovery diagnosis across status, support, dashboard, live-smoke, and soak report surfaces.
- Progress-credit and stall-diagnosis contracts that prevent false progress and expose useful work, peer contribution, no-progress thresholds, and stalled subsystems.
- Redacted support-bundle forensics for soak timelines, checkpoint chains, resource pressure, recovery events, peer outcomes, and final verdicts.
- Opt-in UAT and release-boundary docs plus deterministic Phase 75 through Phase 80 checker coverage, followed by Phase 81 audit traceability closure.

### What Worked

- Keeping public-network soak evidence opt-in let the milestone add long-run operator workflows without weakening deterministic local verification.
- Shared contracts for resource, recovery, progress, and forensics kept status, dashboard, support, live-smoke, and soak evidence aligned.
- The milestone audit caught traceability gaps before archive, and Phase 81 closed them without changing the shipped runtime behavior.

### What Was Inefficient

- Phase 76 and Phase 77 verification artifacts originally described the right evidence but did not name every RES/REC requirement ID, forcing a late traceability closure phase.
- Automated accomplishment extraction pulled in code-review findings and noisy multi-line summaries, so `MILESTONES.md` still needed manual curation.
- The archive helper created the v1.7 archive files but left living ROADMAP/PROJECT/STATE/RETROSPECTIVE evolution to manual edits.

### Patterns Established

- Verification artifacts should name requirement IDs directly when the milestone audit depends on cross-source traceability.
- Long-run operator claims should be represented as deterministic synthetic checks plus explicit opt-in UAT, not default CI or local verifier gates.
- Support-bundle evidence works best when it is a redacted projection of shared domain contracts rather than a separate diagnostic interpretation layer.

### Key Lessons

1. Requirement IDs belong in verification frontmatter and tables as soon as a phase is verified.
2. Milestone archive helpers need curated accomplishment inputs or post-helper cleanup to avoid leaking review findings into shipped summaries.
3. A passed audit should be archived with the milestone, while prior failed audits should remain explainable through a closure trail.

### Cost Observations

- Model mix: not measured in repo artifacts.
- Sessions: multiple GSD yolo execution, code-review, verification, audit, gap-closure, and archive turns across the v1.7 milestone.
- Notable: Phase 81 was mostly evidence-control work, confirming that late audit failures can be cheaper to close when implementation evidence is already stable and requirement-mapped.

---

## Milestone: v1.8 - Production Full-Node Readiness Boundary

**Shipped:** 2026-06-25
**Phases:** 8 | **Plans:** 26 | **Counted summary tasks:** 49

### What Was Built

- Canonical production-readiness vocabulary, support levels, claim matrix, evidence gates, and deferred-surface inventory.
- Support matrix and issue-evidence policy that classify source-built environments, redacted support expectations, residual risk, and support-update boundaries.
- Source-built upgrade and rollback policy with pre-upgrade evidence, state/schema compatibility guidance, backup expectations, and no-hidden-mutation boundaries.
- Operator runbooks and service-operation expectations for preflight, monitoring, no-progress diagnosis, recovery, support-bundle timelines, escalation, and service-supervision posture.
- Release-readiness checklist and parity roots mapping all 23 v1.8 requirements to docs, deterministic checks, opt-in UAT posture, residual risk, and no-claim boundaries.
- Deterministic Phase 88 and Phase 89 claim guardrails that scan release/operator docs plus upgrade, runbook, and service policy docs for overbroad production-readiness or deferred-surface claims.

### What Worked

- Treating production readiness as a gated claim kept v1.8 useful without implying unsupported node, relay, wallet, migration, packaging, GUI, hosted-dashboard, service, or public-network CI readiness.
- Narrow Bun checkers made release-language policy enforceable in default verification without introducing public-network, real service-manager, or multi-day test gates.
- Phase 89 closed the audit gaps by expanding the canonical checker corpus and making Phase 88 evidence visible from the release-readiness checklist.

### What Was Inefficient

- Requirements, roadmap, and lifecycle metadata drifted after Phase 89 and needed a closeout refresh before archive.
- The first milestone audit correctly reported `tech_debt`, but the Phase 89 verification file still needed lifecycle metadata refresh before every phase could validate as current.
- The archive helper generated a noisy accomplishment list and stale state frontmatter, requiring a manual curation pass.

### Patterns Established

- Production-adjacent release language should be represented as evidence gates and deterministic no-claim guardrails before any future readiness claim is allowed.
- Canonical policy docs should be part of the guardrail corpus when they can accidentally promote deferred surfaces.
- Repo-local Cargo and Bazel command forms should appear in operator-facing UAT guidance so archived instructions remain copy-pasteable without assuming an installed alias.

### Key Lessons

1. Closeout metadata should be refreshed before archive so helper output copies clean source artifacts.
2. Verification artifacts must have current `generated_at` and `lifecycle_validated` metadata after the latest summary timestamp.
3. Release-readiness checkers should eventually parse strict table rows, not only requirement ID presence.

### Cost Observations

- Model mix: not measured in repo artifacts.
- Sessions: multiple GSD execution, audit, gap-closure, archive, and manual planning-curation turns across the v1.8 milestone.
- Notable: the remaining audit debt is checker hardening only; all requirement, integration, and release-flow blockers were closed before archive.

---

## Cross-Milestone Trends

### Process Evolution

| Milestone | Phases | Key Change |
| --- | ---: | --- |
| v1.0 | 22 | Established parity-first implementation, verification, audit, and archive workflow. |
| v1.1 | 22 | Extended the workflow to terminal-first operator surfaces, milestone rerun audits, and explicit post-audit cleanup phases before archive. |
| v1.2 | 7 | Added opt-in public-mainnet IBD review, live-smoke evidence, and security closeout while preserving hermetic default verification. |
| v1.3 | 12 | Added public-mainnet proof hardening, fresh-status live-smoke closeout, support evidence cleanup, and explicit release-boundary audit closure. |
| v1.4 | 6 | Converted diagnosed-blocker evidence into scoped outbound IBD convergence evidence with peer compatibility, header/block progress, restart/resume, support evidence, and release-boundary checks. |
| v1.5 | 8 | Made unattended mainnet operator review bounded, service-reviewable, supportable, and deterministic by default. |
| v1.6 | 7 | Converted unattended review into an explicit opt-in sync-to-tip and stay-current evidence claim. |
| v1.7 | 7 | Hardened full-sync evidence for multi-day soak, resource and recovery bounds, progress guarantees, forensics, opt-in UAT, and audit traceability. |
| v1.8 | 8 | Added production-readiness claim gates, support/update/runbook/service policies, release-readiness evidence, and deterministic no-claim guardrails. |

### Cumulative Quality

| Milestone | Requirements | Audit Status | Verification Posture |
| --- | ---: | --- | --- |
| v1.0 | 28/28 complete | Passed with GAP-01 through GAP-04 closed | Repo-native `scripts/verify.sh`, Rust checks, coverage, architecture policy, breadcrumb guard, and panic-site guard. |
| v1.1 | 44/44 complete | Passed after Phase 33 and Phase 34 cleanup rerun | Repo-native `scripts/verify.sh`, operator-binary coverage, benchmark smoke and report validation, and Bazel smoke builds. |
| v1.2 | 26/26 complete | Closed through Phase 40 live-smoke closeout and Phase 41 security audit/UAT; no dedicated milestone audit artifact | Repo-native `scripts/verify.sh`, deterministic sync regressions, opt-in live-mainnet smoke reporting, security audit, and UAT. |
| v1.3 | 22/22 complete | Ready for archive with zero tracked tech-debt items after Phase 53 | Repo-native `scripts/verify.sh`, release-boundary guard, schema v2 support-bundle tests, live-smoke regression, opt-in public-network UAT, and lifecycle validation. |
| v1.4 | 22/22 complete | Tech-debt audit with zero blockers; planning traceability corrected during archive prep | Repo-native `scripts/verify.sh`, compatibility/header/block/restart deterministic tests, schema v2 live-smoke fixtures, support redaction tests, release-boundary guard, and lifecycle validation. |
| v1.5 | 23/23 complete | Passed with no open requirement, integration, flow, or current tech-debt gaps | Repo-native `scripts/verify.sh`, bounded sync loop checks, service lifecycle/restart evidence, support bundles, compatibility wrapper reports, and deterministic release-boundary checks. |
| v1.6 | 26/26 complete | Closed through Phase 74 verification and source-built full-sync completion evidence | Repo-native `scripts/verify.sh`, active-chain validation, stay-current checks, reorg/peer recovery, support evidence, opt-in UAT, and release-boundary verification. |
| v1.7 | 24/24 complete | Passed after Phase 81 traceability closure | Repo-native `scripts/verify.sh`, Phase 75-80 deterministic checkers, resource/recovery/progress/forensics fixtures, opt-in UAT docs, and 11/11 integration plus 6/6 flow audit checks. |
| v1.8 | 23/23 complete | Tech-debt audit with zero requirement, integration, or flow blockers after Phase 89 closure | Repo-native `scripts/verify.sh`, production no-claim guardrails, support/update/runbook/service policy checks, release-readiness evidence, opt-in UAT docs, and 23/23 integration plus 8/8 flow audit checks. |

### Top Lessons

1. Keep milestone control artifacts as actively verified surfaces, not passive notes.
2. Prefer narrow, auditable parity claims over broad unsupported equivalence statements.
3. Close audit debt through explicit cleanup phases when the evidence trail matters as much as the fix itself.
4. Keep requirements traceability current during execution so archive work is historical packaging, not late evidence reconstruction.
5. Treat successful public-network progress and fresh diagnosed blockers as different outcomes; both can be valid, but the archive must say which one shipped.
6. Archive helpers need a final human sanity pass until they can safely evolve state, roadmap, project, milestones, and retrospective artifacts together.
7. Verification reports should name the requirement IDs they satisfy; prose-only evidence creates avoidable audit ambiguity.
8. Production-adjacent docs need deterministic guardrails as soon as the project introduces terms that sound stronger than the evidence actually supports.
