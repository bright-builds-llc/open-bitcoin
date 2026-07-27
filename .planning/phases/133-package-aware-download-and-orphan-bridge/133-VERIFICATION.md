---
phase: 133-package-aware-download-and-orphan-bridge
verified: 2026-07-27T00:08:09Z
status: passed
score: 19/19 must-haves verified
generated_by: gsd-verifier
lifecycle_mode: yolo
phase_lifecycle_id: 133-2026-07-26T16-12-51
generated_at: 2026-07-27T00:08:09Z
lifecycle_validated: true
overrides_applied: 0
---

# Phase 133: Package-Aware Download and Orphan Bridge Verification Report

**Phase Goal:** Peer-originated reconsiderable transactions can form only the pinned bounded same-peer 1P1C candidate and reuse authoritative package policy without a new wire protocol.
**Verified:** 2026-07-27T00:08:09Z
**Status:** passed
**Re-verification:** No — initial verification

## Goal Achievement

### Observable Truths

The three roadmap success criteria are the non-negotiable contract. The remaining
rows preserve the additional observable truths declared across all four plan
frontmatter blocks.

| # | Truth | Status | Evidence |
| --- | --- | --- | --- |
| 1 | Peer evidence distinguishes hard rejects from reconsiderable package candidates while retaining only bounded, rotating candidate and reject state. | ✓ VERIFIED | `reject_evidence.rs` owns separate typed hard and reconsiderable filters with fixed preallocation, three-generation rotation, and 120,000/0.000001 parameters; orphan bodies, announcers, and candidate cursors are independently count- and byte-bounded. |
| 2 | Ordinary transaction messages can assemble only sender-aware same-peer one-parent/one-child candidates with preserved origin and exact package identity. | ✓ VERIFIED | Ordinary `inv`/`getdata`/`tx` handling captures receipt provenance, stages one shared orphan body, requires one missing parent plus the supplying peer's retained announcement, and emits an opaque `[parent, child]` candidate with `[peer, peer]` origins. |
| 3 | Peer candidates receive the same authoritative package-admission outcomes as local submissions; no package wire message or arbitrary multi-parent assembly is introduced. | ✓ VERIFIED | The node refines through `WellFormedPackage` and `SubmissionPackage`, then calls the same `ManagedMempool::submit_package`/`Mempool::submit_package` engine used by package submission. Wire and inventory enums contain no package variant, and multi-parent candidates are excluded. |
| 4 | Unique hard-reject and reconsiderable floods cannot grow retained evidence beyond two fixed allocations derived from 120,000 insertions and a 0.000001 false-positive target. | ✓ VERIFIED | Each filter allocates 161,750 `u64` words once with 20 probes; the one-million-insertion oracle passed in the 93-test network run. |
| 5 | Hard transaction wtxids, reconsiderable transaction wtxids, and failed package fingerprints remain distinct typed evidence domains and never share the old exact reject set. | ✓ VERIFIED | `HardRejectEvidence` accepts only `Wtxid`; `ReconsiderableEvidenceKey` separates `Transaction(Wtxid)` from `Package([u8; 32])`; no exact recent-reject set remains. |
| 6 | Ordinary inventory suppression may consult hard and reconsiderable membership, while orphan-parent requests ignore reconsiderable membership and remain possible. | ✓ VERIFIED | `record_announcement` passes `include_reconsiderable=true`; `request_parent` passes `false`; both paths retain hard-reject suppression. |
| 7 | Both rolling filters reset together after each actual active-chain-tip mutation and do not reset for duplicate, non-extending, disconnected, or failed transitions. | ✓ VERIFIED | One `PeerManager::on_active_tip_changed` clears both domains after successful local connect, stored connect, and reorg. Seventeen lifecycle tests covered positive and negative transitions. |
| 8 | One orphan body retains a bounded set of announcers, and disconnect removes only the departing announcer until no announcer remains. | ✓ VERIFIED | `BoundedOrphanAnnouncers`, global/per-peer/announcer/byte caps, shared-body storage, and `cleanup_peer` implement this rule; adversarial and disconnect tests passed. |
| 9 | Either child-first or parent-first ordinary transaction arrival can reach a parent-triggered candidate without retaining a reconsiderable-parent body cache. | ✓ VERIFIED | Child-first and parent-first retransmission cases passed. The persistent cursor retains one current parent plus child identities, not a parent-body cache or copied child bodies. |
| 10 | Only the newest eligible child spending the parent and announced by the parent-supplying peer can form exactly `[parent, child]` with aligned `[peer, peer]` origins. | ✓ VERIFIED | The `Reverse<u64>` parent index, single-parent predicate, matching-announcer check, opaque candidate fields, and aligned origin construction are present and covered by wrong-peer/newest-child tests. |
| 11 | Candidate traversal, announcer storage, TTL, global/per-peer bodies, and per-parent work remain explicitly bounded. | ✓ VERIFIED | Defaults cap 100 bodies, 25 bodies per peer, 8 announcers, 40,000,000 retained bytes, a 20-minute injected-time TTL, and 32 candidate visits per parent. Cursor creation also checks the aggregate byte budget. |
| 12 | The node refines each neutral pair once through `WellFormedPackage` and `SubmissionPackage`, then calls the Phase 132 package engine exactly once. | ✓ VERIFIED | The only candidate handoff is in `admission_bridge/package.rs`; the successful candidate path has one managed `submit_package` call, and the call-count integration test passed. |
| 13 | The exact cached `PackageFingerprint`, ordered `PackageReport`, and committed `MempoolLifecycleDelta` survive the bridge without flattening or recomputation. | ✓ VERIFIED | The fingerprint is read from the checked package, the report copies that same fingerprint, and `ManagedPeerPackageAdmission` retains `SubmittedPackageResult` unchanged. Exact report/order/fingerprint/delta equality passed. |
| 14 | Hard, terminal, missing-input, other reconsiderable, and failed-package feedback update only candidate/reject/orphan correctness state. | ✓ VERIFIED | Exhaustive `PackageMemberResult` and `PackageStatus` matches record the appropriate typed evidence, retire terminal members, or restage exact missing parents. All feedback variants passed. |
| 15 | Package admission does not project serving, fanout, compact, persistence, unbroadcast, retry, or operator effects before Phase 134. | ✓ VERIFIED | Action translation stores package admissions separately. Before/after tests prove serving, fanout, compact, and transaction-storage projections remain unchanged; no Phase 136/137 surface was added. |
| 16 | Contributors can trace PPKG-01 through PPKG-03 to exact Rust behavior, pinned Knots anchors, deterministic tests, and explicit intentional differences. | ✓ VERIFIED | The parity catalog, machine index, checklist, source breadcrumbs, and READMEs name all three requirements, exact sources/tests, Knots anchors, and the intentional wtxid/fingerprint scope difference. |
| 17 | Default verification fails if fixed-memory evidence, same-peer provenance, exact 1P1C identity, one authoritative package call, or feedback boundaries regress. | ✓ VERIFIED | The filesystem-only checker is wired twice in `scripts/verify.sh`; all 30 independent checker mutations passed. |
| 18 | Project claims say bounded opportunistic same-peer 1P1C over ordinary transaction messages and reject general package wire, arbitrary multi-parent, premature fanout/receipt, public/default relay, and production claims. | ✓ VERIFIED | README, package README, parity catalog, and the live Phase 133 checker all preserve the narrow claim and explicit deferrals. |
| 19 | The complete Rust, Bazel, coverage, breadcrumb, checker, and repository contract passes with tracked LOC freshness. | ✓ VERIFIED | Current `HEAD` is security-report commit `03480866`, whose full repository hook passed after clean re-review `e115943c`. This verification independently passed the 30-mutation checker, live checker, 93 network tests, 7 node bridge tests, 17 lifecycle tests, 18 mempool package tests, breadcrumb validation, and `git diff --check`. |

**Score:** 19/19 truths verified

### Required Artifacts

| Artifact | Expected | Status | Details |
| --- | --- | --- | --- |
| `packages/open-bitcoin-network/src/peer/transaction_relay/reject_evidence.rs` | Fixed-allocation rotating typed reject evidence | ✓ VERIFIED | Exists, substantive, exported through transaction-relay state, owned by `PeerManager`, and exercised under one million insertions. |
| `packages/open-bitcoin-network/src/peer.rs` | One network owner for scheduler, orphanage, and evidence | ✓ VERIFIED | Owns both filters and orphanage; exposes narrow query, record, reset, staging, and candidate methods. |
| `packages/open-bitcoin-network/src/peer/transaction_relay/scheduler.rs` | Ordinary-message request scheduling and provenance capture | ✓ VERIFIED | Captures txid/wtxid announcers before cleanup and distinguishes ordinary from parent-request suppression. |
| `packages/open-bitcoin-network/src/peer/transaction_relay/orphanage.rs` | Bounded shared bodies, provenance, indexes, and cleanup | ✓ VERIFIED | Count, peer, announcer, TTL, retained-byte, and traversal bounds are substantive and wired. |
| `packages/open-bitcoin-network/src/peer/transaction_relay/orphanage/candidate.rs` | Opaque newest-first same-peer 1P1C proof | ✓ VERIFIED | Private construction, identity-only child cursor, canonical body lookup, aligned origins, and aggregate byte accounting are wired through `PeerManager`. |
| `packages/open-bitcoin-node/src/network/mempool_lifecycle.rs` | Authoritative active-tip reset seams | ✓ VERIFIED | All three successful tip mutation paths call the paired reset after chainstate success. |
| `packages/open-bitcoin-mempool/src/package/report.rs` | Exact typed package outcomes | ✓ VERIFIED | Missing parents and hard policy categories remain typed; report construction preserves request order and cached fingerprint. |
| `packages/open-bitcoin-node/src/mempool.rs` | Thin authoritative package adapter | ✓ VERIFIED | Delegates directly to `Mempool::submit_package` and returns `SubmittedPackageResult` unchanged. |
| `packages/open-bitcoin-node/src/network/admission_bridge/package.rs` | Candidate refinement and one authoritative submission | ✓ VERIFIED | Consumes the opaque candidate, refines once, checks cached fingerprint, submits once, and preserves exact truth. |
| `packages/open-bitcoin-node/src/network/tests/package_bridge_cases.rs` | End-to-end package bridge proof | ✓ VERIFIED | Seven behavior tests cover arrival order, provenance, exact handoff, typed feedback, fallback, and graph exclusion. |
| `docs/parity/catalog/mempool-policy.md` | Auditable Phase 133 claim and deferrals | ✓ VERIFIED | PPKG-01/02/03, Knots anchors, intentional differences, and later-phase boundaries are explicit. |
| `scripts/check-phase133-package-aware-download-orphan-bridge.ts` | Fail-closed deterministic source/claim guard | ✓ VERIFIED | Live checker passes and 30 mutations prove independent enforcement. |
| `scripts/verify.sh` | Default verifier ownership | ✓ VERIFIED | Checker test and live checker appear immediately after Phase 132 in both ordering surfaces. |

The generic plan artifact verifier reported literal-pattern misses for
`reject.*facts`, `same_peer`, and `record.*feedback`. Manual Level 1–3 checks
closed all three as renamed implementations: `TxDownloadLocalFacts`,
`begin_same_peer_candidate`, and `apply_package_status_feedback` /
`apply_package_member_feedback`.

### Key Link Verification

| From | To | Via | Status | Details |
| --- | --- | --- | --- | --- |
| `inventory_state.rs` | reject evidence owned by `PeerManager` | Semantic `TxDownloadLocalFacts` | ✓ WIRED | Authoritative txid-to-wtxid resolution feeds hard/reconsiderable booleans without cloning filter storage. |
| `mempool_lifecycle.rs` | `PeerManager` | `on_active_tip_changed` | ✓ WIRED | Called after successful local connect, stored connect, and reorg only. |
| `scheduler.rs` | orphanage | `ReceivedTransactionProvenance` through `PeerAction` | ✓ WIRED | Announcers are captured before cleanup and passed into provenance-aware staging. |
| `orphanage.rs` | `PeerManager` / node bridge | Opaque consume-only candidate | ✓ WIRED | Only retained same-peer evidence can construct the candidate; node code cannot forge eligibility. |
| `admission_bridge/package.rs` | Phase 132 mempool engine | `ManagedMempool::submit_package` | ✓ WIRED | One eligible candidate produces one authoritative call and unchanged result. |
| `admission_bridge.rs` | orphan/reject evidence | Exhaustive typed feedback | ✓ WIRED | Terminal, hard, missing-input, reconsiderable, and package-status outcomes reach only bounded correctness state. |
| `action_translation.rs` | caller-visible package result | `package_admissions` collection | ✓ WIRED | Exact report and delta leave the bridge without premature lifecycle projection. |
| `scripts/verify.sh` | Phase 133 checker | Checker tests and live run | ✓ WIRED | Both commands are present in both verifier ordering surfaces. |
| `source-breadcrumbs.json` | pinned Knots sources/tests | Direct source lineage | ✓ WIRED | Breadcrumb validation passed for 444 Rust files. |

### Data-Flow Trace (Level 4)

| Artifact | Data | Source | Produces Real Data | Status |
| --- | --- | --- | --- | --- |
| Transaction scheduler/orphanage | Delivering peer plus retained txid/wtxid announcers | Ordinary `inv`/`getdata`/`tx` state | Yes — live scheduler maps are captured before cleanup and bounded on retention | ✓ FLOWING |
| Same-peer candidate | Parent body, canonical child body, aligned origins/provenance | Reconsiderable parent evidence plus indexed retained orphan | Yes — candidate construction resolves current child bodies from the canonical orphan map | ✓ FLOWING |
| Node package bridge | Checked pair and cached fingerprint | Opaque network candidate | Yes — the pair is refined through Phase 132 types and submitted to the live mempool | ✓ FLOWING |
| Package feedback | Ordered member outcomes and lifecycle delta | Authoritative `SubmittedPackageResult` | Yes — exact report drives bounded feedback while the exact delta is retained for the caller | ✓ FLOWING |

### Behavioral Spot-Checks

All Cargo commands used `CARGO_TARGET_DIR=/private/tmp/open-bitcoin-phase133-target.xgN5jr`
and ran sequentially through `scripts/command-timings.ts`.

| Behavior | Command | Result | Status |
| --- | --- | --- | --- |
| Guard mutations independently detect contract loss | `bun test scripts/check-phase133-package-aware-download-orphan-bridge.test.ts` through timing wrapper | 30 passed, 0 failed | ✓ PASS |
| Current source/docs satisfy the Phase 133 guard | `bun run scripts/check-phase133-package-aware-download-orphan-bridge.ts` through timing wrapper | Passed | ✓ PASS |
| Reject evidence, scheduling, orphan provenance, and bounded candidates execute | `cargo test -p open-bitcoin-network transaction_relay` through timing wrapper | 93 passed, 0 failed | ✓ PASS |
| Same-peer package bridge and no-projection behavior execute | `cargo test -p open-bitcoin-node package_bridge_cases` through timing wrapper | 7 passed, 0 failed | ✓ PASS |
| Successful-tip-only paired filter reset executes | `cargo test -p open-bitcoin-node mempool_lifecycle_cases` through timing wrapper | 17 passed, 0 failed | ✓ PASS |
| Authoritative package reports and exact missing-parent behavior execute | `cargo test -p open-bitcoin-mempool package_parity_cases` through timing wrapper | 18 passed, 0 failed | ✓ PASS |
| Rust source lineage is complete | `bun run scripts/check-parity-breadcrumbs.ts` through timing wrapper | 444 files verified | ✓ PASS |

### Requirements Coverage

| Requirement | Source Plans | Description | Status | Evidence |
| --- | --- | --- | --- | --- |
| PPKG-01 | 133-01, 133-02, 133-04 | Distinguish hard/reconsiderable evidence and retain only bounded rotating state. | ✓ SATISFIED | Typed fixed filters, paired tip reset, bounded shared orphan/cursor state, adversarial runtime tests, and mutation guard. |
| PPKG-02 | 133-02, 133-03, 133-04 | Assemble only sender-aware same-peer 1P1C candidates over ordinary messages with exact identity. | ✓ SATISFIED | Provenance capture, one shared body, newest single-parent predicate, opaque aligned-origin candidate, both arrival orders, and wrong-peer/multi-parent exclusions. |
| PPKG-03 | 133-03, 133-04 | Reuse authoritative package admission rather than reimplement policy. | ✓ SATISFIED | One node-owned Phase 132 refinement and mempool call, unchanged report/fingerprint/delta, typed feedback, no network-to-mempool dependency, and no package wire variant. |

No Phase 133 requirement is orphaned. `PPKG-04` is explicitly assigned to Phase
136 and is not a Phase 133 gap.

### Anti-Patterns Found

| File | Line | Pattern | Severity | Impact |
| --- | --- | --- | --- | --- |
| — | — | No TODO/FIXME/placeholder, exact unbounded reject collection, package wire variant, network-to-mempool dependency, or goal-blocking stub found | ℹ️ Info | None |
| `admission_bridge.rs`, `admission_bridge/package.rs`, `action_translation.rs` | Various exhaustive matches | Empty terminal/no-effect match arms | ℹ️ Info | Intentional typed boundaries; these variants require no Phase 133 projection and are covered by exhaustive tests. |

### Disconfirmation Pass

- **Closest scope ambiguity:** PPKG-03 currently proves the shared authoritative
  engine and exact result contract, not an RPC-level comparison. That is the
  intended Phase 133 boundary; Phase 137 owns the RPC/operator adapter.
- **Potentially misleading test boundary:** The node package-bridge suite invokes
  the node admission seam directly rather than decoding a wire frame. Separate
  scheduler/peer tests prove ordinary receipt wiring, and the actual wire and
  inventory enums were inspected to confirm no package message exists.
- **Error-path check:** A debug assertion alone would not prove fingerprint
  preservation in release builds. Here `PackageReport::try_new` copies the
  fingerprint from the same checked package, exact equality is asserted in the
  runtime integration test, and the mutation checker independently guards the
  cached-fingerprint seam.

None of these checks produced a gap.

### Human Verification Required

None. Phase 133 is a headless, pure/runtime-core integration with deterministic
in-process behavior and no visual, real-time, external-service, or public-network
acceptance criterion.

### Gaps Summary

No actionable gaps were found. All 19 merged roadmap and plan truths, all three
requirements, all required artifacts, and every critical link are verified.
Later-phase lifecycle projection (Phase 134), fanout/receipts (Phase 136), and
RPC/operator surfaces (Phase 137) remain explicitly deferred and guarded; they
do not reduce Phase 133's score.

## Provenance and Review Evidence

- `133-CONTEXT.md`, all four plans, and all four summaries use
  `lifecycle_mode: yolo` and
  `phase_lifecycle_id: 133-2026-07-26T16-12-51`.
- This report uses the required `generated_by: gsd-verifier` provenance.
- Clean re-review commit `e115943c` and security report commit `03480866` are
  ancestors of the verified `HEAD`; security closed 22/22 declared threats.
- Repo-local `AGENTS.md`, `AGENTS.bright-builds.md`,
  `standards-overrides.md` (no active exception), and the architecture,
  code-shape, testing, verification, and Rust standards informed the audit.

***

_Verified: 2026-07-27T00:08:09Z_
_Verifier: the agent (gsd-verifier)_
