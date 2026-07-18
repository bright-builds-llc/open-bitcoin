---
phase: 126-compact-relay-residual-hardening
verified: 2026-07-18T20:48:44Z
status: passed
score: 10/10 must-haves verified
generated_by: gsd-verifier
lifecycle_mode: yolo
phase_lifecycle_id: 126-2026-07-18T16-09-20
generated_at: 2026-07-18T20:48:44Z
lifecycle_validated: true
overrides_applied: 0
requirements_verified:
  - CMP-05
  - RCN-02
  - RCN-03
  - GOV-04
  - BOUND-01
  - HARD-05
canonical_requirement_state: pending_pre_promotion
canonical_audit_state: gaps_found
---

# Phase 126: Compact Relay Residual Hardening Verification Report

**Phase Goal:** Close the approved compact-relay runtime and parity debt, then reconcile the complete v2.1 corpus for a fresh archive decision.
**Verified:** 2026-07-18T20:48:44Z
**Status:** passed
**Re-verification:** No — initial independent pre-promotion verification

## Verdict

Phase 126 is independently verified for promotion. The runtime no longer has a
production-capable factless compact receive route, compact announcement nonces
come from a lazy call-scoped OS-entropy shell seam, entropy failure cannot
produce compact wire output or false compact success evidence, exact Knots
anchors cover both hardened boundaries, and deterministic mutation guards
protect the runtime, parity, verifier order, and four legal closeout states.

This verdict authorizes Plan 126-04; it does not perform promotion. The six
requirements correctly remain unchecked/Pending, the canonical audit correctly
remains `gaps_found` at 33/39 requirements and 16/17 phases, Phase 126 remains
3/4 and In Progress, and `/gsd-execute-phase 126` remains the primary route.

## Goal Achievement

### Observable Truths

| # | Truth | Status | Evidence |
| ---: | --- | --- | --- |
| 1 | Generic compact-block dispatch fails closed instead of inventing receive facts. | ✓ VERIFIED | `PeerManager::handle_message` returns peer-neutral `NetworkError::CompactBlockReceiveFactsRequired`; the focused regression also proves no compact in-flight state is created. |
| 2 | Both production-capable managed receive entrypoints supply explicit live mempool and bounded extra snapshots. | ✓ VERIFIED | `receive_message` and `receive_sync_message` both intercept `CompactBlock` and call `handle_compact_block_receive`; that adapter snapshots the managed mempool and `CompactExtraTxnBuffer` and constructs explicit `CompactBlockReceiveFacts`. |
| 3 | Genuine empty live snapshots remain valid while collision, duplicate, missing, and reconstruction outcomes stay typed. | ✓ VERIFIED | The empty-mempool live test produces bounded `GetBlockTxn`; live candidate, sync-receive, collision, duplicate-short-ID, missing-index, and lifecycle tests pass. `CompactBlockReceiveFacts` no longer implements `Default`. |
| 4 | Production compact announcement nonce generation is randomized in the node shell and deterministically injectable in tests. | ✓ VERIFIED | `ManagedPeerNetwork::announce_block` uses a call-scoped `[u8; 8]`, `getrandom::fill`, and `u64::from_le_bytes`; the pure peer/consensus builder still receives an explicit nonce. The injected fixed-nonce test proves exact propagation and one invocation. |
| 5 | Entropy is lazy and consumed only after `AnnounceCompactBlock` is selected. | ✓ VERIFIED | The helper invokes its `FnOnce` only in the compact action branch; headers, inventory, and suppression all prove zero nonce calls. Cargo and Bazel both declare the node-only `getrandom` dependency; network and consensus do not. |
| 6 | Entropy failure cannot emit `cmpctblock` or record false compact provenance/evidence. | ✓ VERIFIED | Failure delegates to the peer's headers/inventory fallback. Provenance is recorded only for an actual `WireNetworkMessage::CompactBlock`, and evidence derives from the emitted message. The failure regression proves inventory fallback, empty compact provenance, compact count 0, and fallback count 1. |
| 7 | Parity evidence names concrete receive-candidate and randomized-nonce Knots anchors. | ✓ VERIFIED | Index, breadcrumbs, and catalogs cite `net_processing.cpp/.h`, `blockencodings.cpp/.h`, and `p2p_compactblocks.py`; the pinned sources contain `FastRandomContext().rand64()`, `CBlockHeaderAndShortTxIDs`, live mempool/recent-extra supply, and `PartiallyDownloadedBlock::InitData`. |
| 8 | Deterministic local mutations guard every hardened runtime and parity boundary. | ✓ VERIFIED | The Phase 126 suite passes 13/13 mutations covering factless routing, `Default`, both managed routes, live facts, lazy/fallible and hash-derived nonce mutations, failure emission, provenance, achieved-effect evidence, build dependency agreement, parity roots, and verifier locality/order. |
| 9 | The closeout guard preserves exactly four coherent Phase 126 lifecycle states and rejects mixed projections. | ✓ VERIFIED | Candidate, verified-pre-promotion, promoted-pre-summary, and archive-ready fixtures pass; premature promotion, lifecycle mismatch, mixed counts, stale progress, and stale routing mutations fail. Live candidate state passes. |
| 10 | Candidate evidence is complete and truthfully separated from independent verification. | ✓ VERIFIED | Plans and summaries 01-03 plus `126-PRE-PROMOTION-EVIDENCE.md` share the exact lifecycle identity. Executor evidence is labeled `gsd-execute-plan`, disclaims verifier ownership, and reports a passing full repository contract; this report supplies the separate `gsd-verifier` provenance gate. |

**Score:** 10/10 truths verified

## Requirement-by-Requirement Evidence

| Requirement | Verification status | Actual evidence | Canonical state now |
| --- | --- | --- | --- |
| `CMP-05` | ✓ VERIFIED FOR PROMOTION | Existing activation, negotiation, header, availability, and resource decisions still gate `CompactAnnouncementAction`; Phase 126 replaces hash-derived nonce material with lazy OS entropy and proves compact success follows actual emission only. Focused policy, announcement, nonce, and fallback tests pass. | Unchecked / Pending, intentionally |
| `RCN-02` | ✓ VERIFIED FOR PROMOTION | Generic compact dispatch cannot synthesize facts; both managed receive paths snapshot current mempool candidates plus bounded extras and pass witness-hash keyed facts to reconstruction. Live and sync-receive candidate tests pass. | Unchecked / Pending, intentionally |
| `RCN-03` | ✓ VERIFIED FOR PROMOTION | The hardened factful route preserves stable typed collision, duplicate, missing, invalid, fallback, and completion behavior. Focused network/node compact suites pass with explicit facts. | Unchecked / Pending, intentionally |
| `GOV-04` | ✓ VERIFIED FOR PROMOTION | Live mempool and bounded extra candidates feed receive; connected-block and mempool-removal lifecycle clears volatile matched slots; tests prove package/filter/default activation remains untouched. | Unchecked / Pending, intentionally |
| `BOUND-01` | ✓ VERIFIED FOR PROMOTION | Machine-readable index, source breadcrumbs, human catalogs, and the mutation checker tie the receive and nonce seams to exact pinned Knots source/test roots. The breadcrumb checker verifies 383 Rust files. | Unchecked / Pending, intentionally |
| `HARD-05` | ✓ VERIFIED FOR PROMOTION | The current pre-promotion corpus agrees at 33/39, 16/17, 3/4, `gaps_found`, and the Phase 126 route. The Phase 124 guard accepts only the four legal transitions; Plan 04 requires this lifecycle-valid report, exact six-ID promotion, full re-verification, atomic archive projection, and fail-closed rollback. | Unchecked / Pending until Plan 04, intentionally |

**Requirement score:** 6/6 supported by implementation and evidence for exact
Plan 04 promotion. Canonical completion remains deliberately deferred until
the promoted and archive-ready gates pass.

## Required Artifacts

| Artifact | Expected | Status | Details |
| --- | --- | --- | --- |
| `packages/open-bitcoin-network/src/peer/message_dispatch.rs` | Fail-closed generic compact routing | ✓ VERIFIED | Exists, substantive, wired through the public generic dispatcher, and returns before reconstruction. |
| `packages/open-bitcoin-network/src/peer/compact_download_state.rs` | Explicit factful reconstruction seam | ✓ VERIFIED | `CompactBlockReceiveFacts` has no `Default`; candidate and extra iterators flow into `init_compact_block_download`. |
| `packages/open-bitcoin-node/src/network.rs` | Managed receive interception and node-shell nonce adapter | ✓ VERIFIED | Both receive entrypoints and announcement action/effect flow are wired; entropy remains in the imperative shell. |
| `packages/open-bitcoin-node/src/network/compact_receive_candidates.rs` | Live mempool and bounded-extra snapshots | ✓ VERIFIED | Real managed mempool and extra-buffer data flow into explicit reference slices and the network-core seam. |
| `packages/open-bitcoin-node/src/network/tests.rs` and split compact tests | Runtime behavior regressions | ✓ VERIFIED | Fixed/failing/counting nonce, both receive entrypoints, empty/live facts, typed outcomes, lifecycle, and no-scope-expansion cases pass. |
| `scripts/check-phase126-compact-relay-residual-hardening.ts` | Fixed-corpus deterministic guard | ✓ VERIFIED | Substantive local-file checker, exported for tests, wired into the repository verifier. |
| `scripts/check-phase126-compact-relay-residual-hardening.test.ts` | One-concern mutation coverage | ✓ VERIFIED | 13/13 mutations pass with stable Phase 126 diagnostics. |
| `scripts/check-phase124-milestone-gap-closure.ts` and fixtures/tests | Four-stage closeout state machine | ✓ VERIFIED | Four legal Phase 126 stages pass and incoherent projections fail. |
| `docs/parity/index.json` and `docs/parity/source-breadcrumbs.json` | Exact machine-readable Knots anchors | ✓ VERIFIED | Receive and announcement surfaces retain all required source and functional-test roots. |
| `126-PRE-PROMOTION-EVIDENCE.md` | Truthfully attributed candidate command evidence | ✓ VERIFIED | Lifecycle-valid executor artifact, explicitly not a substitute for this report. |

## Key Link and Data-Flow Verification

| From | To | Via | Status | Details |
| --- | --- | --- | --- | --- |
| Generic `PeerManager::handle_message` | Compact reconstruction | Typed adapter-routing rejection | ✓ WIRED | `CompactBlock` returns before `handle_compact_block_download`; no invented state is created. |
| `receive_message` / `receive_sync_message` | `handle_compact_block_receive` | Explicit compact interception | ✓ WIRED | Exactly two managed calls are present and mutation-guarded. |
| Managed mempool and `CompactExtraTxnBuffer` | `CompactBlockReceiveFacts` | Owned snapshot then borrowed reference slices | ✓ FLOWING | Real data sources populate candidate and extra iterators; explicit empty snapshots remain valid. |
| `announce_block` policy decision | `announce_block_with_nonce` | Lazy fallible closure | ✓ WIRED | Entropy acquisition occurs after decision and only in the compact branch. |
| `getrandom::fill` | Pure compact payload builder | Explicit `u64` passed through `announce_block_with_action` | ✓ FLOWING | Node Cargo/Bazel dependencies agree; no entropy dependency appears in network or consensus. |
| Actual outbound message | Provenance and evidence counters | Post-emission message match and evidence mapping | ✓ FLOWING | Only actual compact output records provenance/`CompactAnnounced`; headers/inventory map to fallback. |
| Phase 126 checker | `scripts/verify.sh` | Visible and executable ordered steps | ✓ WIRED | Runs after active traceability and before the unchanged Phase 117 final no-claim gate. |
| `126-VERIFICATION.md` | Plan 04 promotion gate | Exact lifecycle/provenance and six-ID evidence | ✓ WIRED | Plan 04 blocks without this passed report and lifecycle validation. |

## Behavioral Spot-Checks

| Behavior | Command | Result | Status |
| --- | --- | --- | --- |
| Phase 126 structural/mutation guard | `bun test scripts/check-phase126-compact-relay-residual-hardening.test.ts` | 13 passed, 0 failed | ✓ PASS |
| Live Phase 126 corpus | `bun run scripts/check-phase126-compact-relay-residual-hardening.ts` | Validated | ✓ PASS |
| Four-state closeout compatibility | `bun test scripts/check-phase124-milestone-gap-closure.test.ts` | 41 passed, 0 failed | ✓ PASS |
| Full closeout reconciliation corpus | `bun test scripts/check-phase124-milestone-closeout-reconciliation.test.ts` | 65 passed, 0 failed | ✓ PASS |
| Live candidate closeout state | `bun run scripts/check-phase124-milestone-closeout-reconciliation.ts` | Passed | ✓ PASS |
| Generic/factful compact core behavior | `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-network compact_block` via timing wrapper | 29 passed, 0 failed | ✓ PASS |
| Managed compact receive/announcement/lifecycle behavior | `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node compact` via timing wrapper | 58 passed, 0 failed | ✓ PASS |
| Announcement-specific behavior | `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node announce` via timing wrapper | 13 passed, 0 failed | ✓ PASS |
| Parity breadcrumb integrity | `bun run scripts/check-parity-breadcrumbs.ts --check` | 383 Rust files verified | ✓ PASS |
| Pre-report lifecycle provenance | `gsd-tools verify lifecycle 126 --require-plans --raw` | `valid` | ✓ PASS |
| Candidate metadata projection | Exact six-ID checklist/traceability and audit/roadmap/state assertions | 6 Pending; 33/39; 16/17; 3/4; `gaps_found` | ✓ PASS |
| Full repository contract | `bash scripts/verify.sh` | Exit 0 in 194.625 seconds; timing record outcome `success` | ✓ PASS |

## Anti-Patterns and Disconfirmation Pass

| Finding | Classification | Assessment |
| --- | --- | --- |
| No TODO/FIXME/placeholder, factless `Default`, hash-derived nonce, or hollow production data path was found in the Phase 126 runtime/checker/parity surface. | ℹ Info | No goal-blocking anti-pattern. Empty slices are explicit live/test states, not placeholders. |
| `scripts/check-phase124-milestone-gap-closure.ts` is now 1,505 lines after adding the four-stage model. | ⚠ Warning | Above the managed code-shape refactor trigger. It is substantive and fully mutation-tested, so this is maintainability debt, not a phase-goal gap. |
| The focused non-compact nonce test injects decisions directly and therefore does not by itself prove policy selection. | ℹ Info | Independent Phase 113/118 decision-policy and managed announcement tests cover activation/negotiation/gate selection; Phase 126 correctly focuses on the effect seam. |
| Actual OS entropy failure is not induced through `getrandom::fill` in production. | ℹ Info | The call-scoped fallible seam deterministically exercises the identical fallback/evidence branch. A live entropy-failure test would be platform-dependent and unsuitable for the deterministic default contract. |

## Lifecycle and Canonical Projection

All formal Phase 126 artifacts inspected share:

- `lifecycle_mode: yolo`
- `phase_lifecycle_id: 126-2026-07-18T16-09-20`
- compliant generator provenance for context, plans, summaries, executor
  pre-promotion evidence, and this verifier-owned report

At this gate, truthful canonical state is:

- six Phase 126 requirements unchecked/Pending
- 33/39 requirements and 16/17 phases
- Phase 126 at 3/4, In Progress
- audit `status: gaps_found`
- primary route `/gsd-execute-phase 126`

That state is a required precondition for passed verification, not a gap.
Plan 04 owns exact promotion, promoted-state full verification, final summary,
archive-ready projection, and rollback to this verified-pre-promotion corpus if
any later gate fails.

## Human Verification Required

None. The phase affects headless runtime and deterministic local verification
surfaces; no visual, public-network, service-manager, or external-service
behavior is part of this gate.

## Residual Risks

- Randomness quality and freshness are trusted to the pinned `getrandom` OS
  adapter. Deterministic verification proves call scope, provenance, injection,
  and failure semantics without making a flaky statistical claim.
- The Phase 126 checker intentionally uses fixed structural anchors. Legitimate
  refactors may require updating checker expectations and mutations together.
- `HARD-05` is verified here as a coherent, fail-closed promotion/archive
  mechanism. Its canonical completed state does not exist until Plan 04 runs
  and passes the promoted and archive-ready full gates.
- This report does not expand v2.1 to package relay, bloom/filter serving,
  compact filters, public serving defaults, public-network CI, archive-node
  behavior, production full-node readiness, production-funds wallet use,
  packaging, hosted services, GUI work, or migration apply mode.

## Gaps Summary

No gaps block Plan 04 promotion. All ten observable must-haves and all six exact
Phase 126 requirements have substantive, wired, behaviorally exercised evidence
appropriate to the independent pre-promotion gate.

_Verified: 2026-07-18T20:48:44Z_
_Verifier: the agent (gsd-verifier)_
