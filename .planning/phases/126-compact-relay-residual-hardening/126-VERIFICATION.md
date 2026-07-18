---
phase: 126-compact-relay-residual-hardening
verified: 2026-07-18T22:07:46Z
status: passed
score: 11/11 must-haves verified
generated_by: gsd-verifier
lifecycle_mode: yolo
phase_lifecycle_id: 126-2026-07-18T16-09-20
generated_at: 2026-07-18T22:07:46Z
lifecycle_validated: true
overrides_applied: 0
requirements_verified:
  - CMP-05
  - RCN-02
  - RCN-03
  - GOV-04
  - BOUND-01
  - HARD-05
canonical_requirement_state: complete
canonical_audit_state: passed
re_verification:
  previous_status: passed
  previous_score: 10/10
  gaps_closed:
    - "The final 126-04 summary now exists and the canonical corpus is coherently projected to archive-ready."
  gaps_remaining: []
  regressions: []
---

# Phase 126: Compact Relay Residual Hardening Verification Report

**Phase Goal:** Close the approved compact-relay runtime and parity debt, then reconcile the complete v2.1 corpus for a fresh archive decision.
**Verified:** 2026-07-18T22:07:46Z
**Status:** passed
**Re-verification:** Yes — fresh post-summary archive-ready verification

## Verdict

Phase 126 achieved its goal. Production-capable compact-block receive adapters
cannot invent empty facts, both managed receive paths supply live mempool and
bounded-extra candidates, compact announcements acquire a lazy randomized
nonce in the node shell, and failed entropy cannot produce compact output or
false compact success evidence. Exact pinned Knots anchors and deterministic
mutation guards cover both hardened seams.

The final summary is present. Requirements, roadmap, project state, milestone
ledger, audit, README, and release-readiness now agree at 39/39 requirements,
17/17 implementation phases, Phase 126 at 4/4 plans, and the sole primary route
`/gsd-complete-milestone v2.1`. This report is newer than every Phase 126
summary and supplies the required final archive-ready verifier handoff.

## Goal Achievement

### Observable Truths

| # | Truth | Status | Evidence |
| ---: | --- | --- | --- |
| 1 | Generic compact-block dispatch fails closed instead of inventing receive facts. | ✓ VERIFIED | `PeerManager::handle_message` returns peer-neutral `NetworkError::CompactBlockReceiveFactsRequired` before reconstruction; the regression proves no compact in-flight state is created. |
| 2 | Both production-capable managed receive entrypoints supply explicit live mempool and bounded-extra snapshots. | ✓ VERIFIED | `receive_message` and `receive_sync_message` both intercept `CompactBlock` and call `handle_compact_block_receive`, which snapshots the managed mempool and `CompactExtraTxnBuffer` into explicit `CompactBlockReceiveFacts`. |
| 3 | Genuine empty live snapshots remain valid while collision, duplicate, missing, and reconstruction outcomes stay typed. | ✓ VERIFIED | Empty live candidates produce bounded `GetBlockTxn`; live candidate, sync-receive, collision, duplicate-short-ID, missing-index, fallback, and lifecycle cases pass. `CompactBlockReceiveFacts` has no `Default`. |
| 4 | Production compact-announcement nonce generation is randomized in the node shell and deterministically injectable in tests. | ✓ VERIFIED | `ManagedPeerNetwork::announce_block` uses a call-scoped `[u8; 8]`, `getrandom::fill`, and `u64::from_le_bytes`; the pure peer/consensus path still receives an explicit nonce. Fixed-nonce tests prove exact propagation and one invocation. |
| 5 | Entropy is lazy and consumed only after `AnnounceCompactBlock` is selected. | ✓ VERIFIED | The fallible closure is invoked only in the compact action branch. Headers, inventory, and suppression prove zero nonce calls. Cargo and Bazel declare the node-shell dependency; network and consensus do not acquire entropy. |
| 6 | Entropy failure cannot emit `cmpctblock` or record false compact provenance/evidence. | ✓ VERIFIED | Failure delegates to the peer's headers/inventory fallback. Provenance and evidence derive from the actual emitted message. The failure regression proves inventory fallback, no compact provenance, compact count 0, and fallback count 1. |
| 7 | Parity evidence names concrete receive-candidate and randomized-nonce Knots anchors. | ✓ VERIFIED | Index, breadcrumbs, and catalogs cite exact `net_processing.cpp/.h`, `blockencodings.cpp/.h`, and `p2p_compactblocks.py` roots containing `FastRandomContext().rand64()`, live mempool/recent-extra supply, short-ID reconstruction, and fallback behavior. |
| 8 | Deterministic local mutations guard every hardened runtime and parity boundary. | ✓ VERIFIED | The Phase 126 suite passes 13/13 mutations covering factless routing, `Default`, both managed routes, live facts, lazy/fallible and hash-derived nonce mutations, failure emission, provenance, achieved-effect evidence, build dependency agreement, parity roots, and verifier order. |
| 9 | The closeout guard preserves exactly four coherent Phase 126 lifecycle states and rejects mixed projections. | ✓ VERIFIED | Candidate, verified-pre-promotion, promoted-pre-summary, and archive-ready fixtures pass; lifecycle mismatch, premature promotion, mixed counts, stale progress, stale verification, and stale routing mutations fail. The live archive-ready corpus passes. |
| 10 | The exact six requirements are complete with auditable implementation and verification evidence. | ✓ VERIFIED | `CMP-05`, `RCN-02`, `RCN-03`, `GOV-04`, `BOUND-01`, and `HARD-05` are checked, mapped exactly once to Phase 126, listed in the final summary, and satisfied in the passed audit. |
| 11 | The complete v2.1 corpus is coherent and routes directly to archival. | ✓ VERIFIED | Requirements show 39/39 complete; roadmap and milestone artifacts show 17/17 phases and Phase 126 at 4/4; audit is passed with 13/13 requirements links and 11/11 integration flows; canonical and release-facing files use only `/gsd-complete-milestone v2.1`. |

**Score:** 11/11 truths verified

## Requirement-by-Requirement Evidence

| Requirement | Status | Implementation and verification evidence |
| --- | --- | --- |
| `CMP-05` | ✓ SATISFIED | Existing activation, negotiation, header, availability, and resource decisions gate `CompactAnnouncementAction`. Phase 126 uses lazy call-scoped OS entropy, preserves deterministic injection, falls back safely, and records success only after actual compact emission. |
| `RCN-02` | ✓ SATISFIED | Generic compact dispatch fails closed. Both managed receive paths snapshot current mempool candidates plus bounded extras and pass witness-hash-keyed facts into reconstruction. Live and sync-receive candidate tests pass. |
| `RCN-03` | ✓ SATISFIED | The factful route preserves stable typed collision, duplicate, missing, invalid, fallback, and completion outcomes. Focused network/node compact suites exercise each case. |
| `GOV-04` | ✓ SATISFIED | Managed mempool and bounded-extra candidates feed receive; connected-block and mempool-removal hooks clear volatile matched state; default package/filter activation remains off. |
| `BOUND-01` | ✓ SATISFIED | Machine-readable index, source breadcrumbs, human catalogs, and mutation guards cite concrete pinned Knots receive, reconstruction, nonce, fallback, peer-state, and resource-governance anchors. |
| `HARD-05` | ✓ SATISFIED | Requirements, roadmap, project, state, milestones, audit, final summary, README, and release-readiness agree at 39/39, 17/17, 4/4, passed, archive-ready, with direct milestone-completion routing. |

**Requirement score:** 6/6 exact Phase 126 requirements satisfied.

## Required Artifacts

| Artifact | Expected | Status | Details |
| --- | --- | --- | --- |
| `packages/open-bitcoin-network/src/peer/message_dispatch.rs` | Fail-closed generic compact routing | ✓ VERIFIED | Exists, substantive, wired through the public generic dispatcher, and returns before compact reconstruction. |
| `packages/open-bitcoin-network/src/peer/compact_download_state.rs` | Explicit factful reconstruction seam | ✓ VERIFIED | `CompactBlockReceiveFacts` has no `Default`; real candidate and extra iterators flow into `init_compact_block_download`. |
| `packages/open-bitcoin-node/src/network.rs` | Managed receive interception and randomized node-shell nonce adapter | ✓ VERIFIED | Both receive entrypoints and announcement action/effect flow are wired; entropy remains in the imperative shell. |
| `packages/open-bitcoin-node/src/network/compact_receive_candidates.rs` | Live mempool and bounded-extra snapshots | ✓ VERIFIED | Real managed data is snapshotted into owned pairs and explicit borrowed facts. |
| `packages/open-bitcoin-node/src/network/tests.rs` and split compact tests | Runtime behavior regressions | ✓ VERIFIED | Fixed/failing/counting nonce, both receive entrypoints, empty/live facts, typed outcomes, lifecycle, and no-scope-expansion cases pass under the repository test contract. |
| `scripts/check-phase126-compact-relay-residual-hardening.ts` | Fixed-corpus deterministic guard | ✓ VERIFIED | Substantive local-file checker, exported for mutation tests and wired into `scripts/verify.sh`. |
| `scripts/check-phase126-compact-relay-residual-hardening.test.ts` | One-concern mutation coverage | ✓ VERIFIED | 13/13 mutations pass with stable Phase 126 diagnostics. |
| `scripts/check-phase124-milestone-gap-closure.ts` and fixtures/tests | Four-stage closeout state machine | ✓ VERIFIED | Four legal Phase 126 stages pass and incoherent projections fail. |
| `docs/parity/index.json` and `docs/parity/source-breadcrumbs.json` | Exact machine-readable Knots anchors | ✓ VERIFIED | Receive and announcement surfaces retain all required source and functional-test roots; 383 Rust breadcrumbs verify. |
| `.planning/phases/126-compact-relay-residual-hardening/126-04-SUMMARY.md` | Lifecycle-valid final summary | ✓ VERIFIED | Exists, declares the exact six completed requirements, shares the phase lifecycle identity, and authorizes the archive-ready projection. |
| Canonical planning and release-facing corpus | Coherent archive-ready state | ✓ VERIFIED | Requirements, roadmap, project, state, milestones, audit, README, and release-readiness agree on counts, status, and route. |

Automated plan-frontmatter checks passed 11/11 declared artifacts across all four
plans.

## Key Link and Data-Flow Verification

| From | To | Via | Status | Details |
| --- | --- | --- | --- | --- |
| Generic `PeerManager::handle_message` | Compact reconstruction | Typed adapter-routing rejection | ✓ WIRED | `CompactBlock` returns before `handle_compact_block_download`; no invented state is created. |
| `receive_message` / `receive_sync_message` | `handle_compact_block_receive` | Explicit compact interception | ✓ WIRED | Exactly two managed calls are present and mutation-guarded. |
| Managed mempool and `CompactExtraTxnBuffer` | `CompactBlockReceiveFacts` | Owned snapshot then borrowed reference slices | ✓ FLOWING | Real data sources populate candidate and extra iterators; explicit empty live snapshots remain valid. |
| `announce_block` policy decision | `announce_block_with_nonce` | Lazy fallible closure | ✓ WIRED | Entropy acquisition follows decision and occurs only in the compact branch. |
| `getrandom::fill` | Pure compact payload builder | Explicit `u64` through `announce_block_with_action` | ✓ FLOWING | Node Cargo/Bazel dependencies agree; entropy is not acquired by the functional core. |
| Actual outbound message | Provenance and evidence counters | Post-emission message match and evidence mapping | ✓ FLOWING | Only actual compact output records compact provenance/`CompactAnnounced`; headers/inventory map to fallback. |
| Phase 126 checker | `scripts/verify.sh` | Visible and executable ordered steps | ✓ WIRED | Runs after active traceability and before the unchanged Phase 117 final no-claim gate. |
| Final Phase 126 summary and verifier | Canonical archive-ready corpus | Lifecycle-valid freshness gate | ✓ WIRED | The final summary exists and this report is newer; the Phase 124 checker rejects stale or mismatched verification. |
| Canonical archive-ready corpus | Milestone archival | Sole primary route | ✓ WIRED | Roadmap, project, state, milestones, audit, README, and release-readiness route to `/gsd-complete-milestone v2.1`. |

Automated plan-frontmatter checks passed 7/7 declared key links across all four
plans.

## Behavioral Spot-Checks

| Behavior | Command | Result | Status |
| --- | --- | --- | --- |
| Phase 126 structural/mutation guard | `bun test scripts/check-phase126-compact-relay-residual-hardening.test.ts` | 13 passed, 0 failed | ✓ PASS |
| Live Phase 126 corpus | `bun run scripts/check-phase126-compact-relay-residual-hardening.ts` | Validated | ✓ PASS |
| Four-state closeout compatibility | `bun test scripts/check-phase124-milestone-gap-closure.test.ts` | 41 passed, 0 failed | ✓ PASS |
| Live four-state closeout guard | `bun run scripts/check-phase124-milestone-gap-closure.ts` | Exit 0 | ✓ PASS |
| Full closeout reconciliation corpus | `bun test scripts/check-phase124-milestone-closeout-reconciliation.test.ts` | 65 passed, 0 failed | ✓ PASS |
| Live archive-ready closeout state | `bun run scripts/check-phase124-milestone-closeout-reconciliation.ts` | Passed | ✓ PASS |
| Active milestone lifecycle traceability | `bun test scripts/check-active-milestone-verification-traceability.test.ts` and live checker | 21 passed, 0 failed; live checker passed | ✓ PASS |
| Release no-claim and ownership boundary | `bun test scripts/check-phase117-parity-uat-release-boundary.test.ts` and live checker | 24 passed, 0 failed; live checker passed | ✓ PASS |
| Parity breadcrumb integrity | `bun run scripts/check-parity-breadcrumbs.ts --check` | 383 Rust files verified | ✓ PASS |
| Phase lifecycle provenance | `gsd-tools verify lifecycle 126 --require-plans --require-verification --raw` | `valid`, exit 0 | ✓ PASS |
| Full repository contract | `bash scripts/verify.sh` | Exit 0 in 3m 19.158s | ✓ PASS |

## Requirements Coverage

| Requirement | Source plan | Description | Status | Evidence |
| --- | --- | --- | --- | --- |
| `CMP-05` | 126-01, 126-02, 126-04 | Gated compact announcement with real randomized effect and truthful fallback evidence | ✓ SATISFIED | Runtime adapter, dependencies, fixed/failing nonce tests, mutation guard, exact audit row |
| `RCN-02` | 126-01, 126-02, 126-04 | Reconstruction from live mempool plus bounded extra inputs | ✓ SATISFIED | Fail-closed generic path, two managed snapshots, live receive tests, exact audit row |
| `RCN-03` | 126-01, 126-02, 126-04 | Stable typed collision, duplicate, missing, and failure outcomes | ✓ SATISFIED | Factful core and focused typed-outcome regressions |
| `GOV-04` | 126-01, 126-02, 126-04 | Mempool/relay/connect-disconnect integration without package/filter activation | ✓ SATISFIED | Managed candidates, lifecycle cleanup hooks, default-off regression |
| `BOUND-01` | 126-02, 126-04 | Concrete Knots anchors for all compact-relay boundaries | ✓ SATISFIED | Index, breadcrumbs, catalogs, checker mutations, 383-file audit |
| `HARD-05` | 126-02, 126-04 | Canonical coverage, phase, audit, and archival route agreement | ✓ SATISFIED | Four-stage checker, 39/39, 17/17, 4/4, passed audit, sole completion route |

No Phase 126 requirement is orphaned. Each exact requirement is owned once in
traceability and appears in the final summary and passed audit.

## Anti-Patterns and Disconfirmation Pass

| Finding | Classification | Assessment |
| --- | --- | --- |
| No TODO/FIXME/placeholder, factless `Default`, hash-derived announcement nonce, or hollow production data path was found in the Phase 126 runtime/checker/parity surface. | ℹ Info | No goal-blocking anti-pattern. Explicit empty live snapshots and parser `null` results are substantive states, not stubs. |
| `scripts/check-phase124-milestone-gap-closure.ts` is 1,505 lines after adding the four-stage model. | ⚠ Warning | Above the managed code-shape refactor trigger. It is substantive and extensively mutation-tested, so this is maintainability debt rather than a phase-goal gap. |
| Focused non-compact nonce tests inject announcement decisions and therefore do not alone prove policy selection. | ℹ Info | Independent Phase 113/118 policy and managed-announcement regressions cover activation, negotiation, header, availability, and resource selection; Phase 126 correctly exercises the effect seam. |
| Actual operating-system entropy failure is not induced through `getrandom::fill`. | ℹ Info | The call-scoped fallible seam deterministically exercises the identical fallback/provenance branch. A live entropy-failure test would be platform-dependent and unsuitable for the deterministic default contract. |
| Historical text still names earlier promoted-pre-summary states. | ℹ Info | Matches occur only in the final summary and audit as chronology/state-model descriptions. Current-state prose consistently says archive-ready and uses the milestone-completion route. |

## Lifecycle and Canonical Projection

All formal Phase 126 artifacts share:

- `lifecycle_mode: yolo`
- `phase_lifecycle_id: 126-2026-07-18T16-09-20`
- compliant generator provenance for context, plans, summaries, executor
  evidence, and this verifier-owned report

The final canonical state is:

- all six exact Phase 126 requirements checked and Complete
- 39/39 active v2.1 requirements complete with 0 pending
- 17/17 implementation phases complete
- Phase 126 complete at 4/4 plans
- canonical audit `status: passed`, 13/13 requirement links, and 11/11 flows
- sole primary route `/gsd-complete-milestone v2.1`

No later phase exists in the active milestone, so no failed or uncertain item is
deferred.

## Human Verification Required

None. The phase affects headless runtime, local deterministic verification, and
planning/parity metadata. Visual behavior, public-network operation, external
services, and production deployment are explicitly outside this milestone
claim.

## Residual Risks

- Randomness quality and freshness are trusted to the pinned `getrandom` OS
  adapter. Deterministic verification proves call scope, injection,
  provenance, and failure semantics without making a flaky statistical claim.
- The Phase 126 checker intentionally uses fixed structural anchors.
  Legitimate refactors may require updating checker expectations and mutations
  together.
- This report does not expand v2.1 to package relay, bloom/filter serving,
  compact filters, public serving defaults, public-network CI, archive-node
  behavior, production full-node readiness, production-funds wallet use,
  migration apply mode, packaging, hosted services, GUI work, or unattended
  public-mainnet full-sync operation.

## Gaps Summary

No gaps remain. All eleven observable must-haves and all six exact Phase 126
requirements have substantive, wired, data-flowing, behaviorally exercised,
and archive-consistent evidence.

_Verified: 2026-07-18T22:07:46Z_
_Verifier: the agent (gsd-verifier)_
