---
phase: 134-authoritative-cross-cache-lifecycle-integration
verified: 2026-07-28T22:50:05Z
status: gaps_found
score: "1/4 roadmap must-haves verified"
generated_by: gsd-verifier
lifecycle_mode: yolo
phase_lifecycle_id: 134-2026-07-28T01-41-12
generated_at: 2026-07-28T22:50:05Z
lifecycle_validated: true
overrides_applied: 0
requirements:
  - id: MPLIFE-01
    status: satisfied
    evidence: "Scoped runtime admission, maintenance, snapshot, relay, and receipt mutations remain behind ManagedNetworkHandle; admission and maintenance enter apply_lifecycle_command."
  - id: MPLIFE-02
    status: blocked
    evidence: "Txid-alias orphan removal can leave a stale candidate cursor, so one delta does not project every removal completely; the unbroadcast oracle also misses absent expected members."
  - id: MPLIFE-03
    status: blocked
    evidence: "Same-txid/different-wtxid orphan cleanup can leave stale dependent candidate state and reconciliation can report it clean."
  - id: MPLIFE-04
    status: blocked
    evidence: "Receipts are typed and I/O is outside the lock, but authority epochs/effect IDs collide across independent handles, completion consumes pending entries by raw effect ID, peer evidence uses a second mutation, and failed capabilities leak pending capacity."
gaps:
  - truth: "Storage and network effects use authority-incarnation-bound receipts applied through one short authoritative follow-up mutation"
    status: failed
    reason: "Every authority starts at AuthorityEpoch::INITIAL and each effect ledger allocates from zero. Completion records/removes by raw effect ID before validating the full receipt binding, so a receipt from authority A can consume authority B's pending peer or snapshot effect and can falsely clear B's dirty generation. Peer emission evidence is then recorded under a second lock, and failed or dropped capabilities have no abort path."
    artifacts:
      - path: packages/open-bitcoin-node/src/network/relay_serving.rs
        issue: "Every ManagedPeerNetwork initializes the same authority epoch and fresh zero-based ledgers."
      - path: packages/open-bitcoin-node/src/network/runtime_authority/lifecycle.rs
        issue: "Completion checks and records raw effect_id before validating the full authority/generation/session or snapshot binding."
      - path: packages/open-bitcoin-node/src/network/lifecycle_effects.rs
        issue: "Pending ledgers are keyed only by family effect ID and expose no exact-binding take or abort operation."
      - path: packages/open-bitcoin-node/src/network/runtime_authority/effects.rs
        issue: "complete_peer_emission completes the receipt and records evidence in two separate authority mutations."
    missing:
      - "Give each authority incarnation a genuinely unique identity and include it in exact pending-ledger keys."
      - "Atomically validate and consume the complete receipt binding before changing completion, dirty, or evidence state."
      - "Complete peer receipt classification and evidence recording in one lifecycle command/lock."
      - "Add consuming abort or explicit reconciliation commands for pre-achievement failures and unsent suffixes."
      - "Add two-independent-handle collision regressions for both peer and snapshot effect families."
  - truth: "One lifecycle delta completely projects admissions and removals into every dependent cache"
    status: failed
    reason: "Peer orphan removal selects entries by txid or wtxid but candidate-cursor removal compares child identities only against the affected canonical wtxids. With the same txid and a different witness identity, the orphan is removed while a cursor containing the orphan's old wtxid survives."
    artifacts:
      - path: packages/open-bitcoin-network/src/peer/transaction_lifecycle.rs
        issue: "orphan_removals can be selected by txid while candidate_removals do not include the removed orphan's actual wtxid."
      - path: packages/open-bitcoin-network/src/peer/transaction_relay/orphanage/candidate.rs
        issue: "remove_orphan_without_candidate_scan intentionally does not repair candidate cursors."
      - path: packages/open-bitcoin-network/src/peer/transaction_lifecycle/reconciliation.rs
        issue: "Reconciliation repeats the canonical-wtxid-only cursor comparison and can report the stale cursor as clean."
      - path: packages/open-bitcoin-node/src/network/lifecycle_projection/reconciliation.rs
        issue: "Unbroadcast reconciliation checks unexpected current members only, not eligible canonical members missing from the set."
    missing:
      - "Carry the exact wtxids of all selected orphan removals into candidate-cursor cleanup."
      - "Make peer reconciliation identity-complete across both txid and wtxid aliases."
      - "Compare unbroadcast expected and actual membership in both directions."
      - "Add same-txid/different-wtxid cursor and missing-expected-unbroadcast corruption regressions."
  - truth: "Lifecycle preparation is bounded and a validated transition cannot become stale before apply"
    status: failed
    reason: "The peer lifecycle caps each package's members but not the number of accepted packages, checks fingerprint capacity before same-transition retirements, and the public mempool validate/apply split returns an owned capability that can overwrite a newer mempool mutation."
    artifacts:
      - path: packages/open-bitcoin-network/src/peer/transaction_lifecycle.rs
        issue: "accepted_packages.len() is unbounded and transient pre-retirement fingerprint size can reject a valid replacement."
      - path: packages/open-bitcoin-mempool/src/pool/prepared_lifecycle.rs
        issue: "ValidatedMempoolTransition does not retain a mutable borrow or recheck revision at consuming apply."
    missing:
      - "Bound accepted-package count and avoid retaining duplicate fingerprint admissions."
      - "Apply same-transition fingerprint retirements before validating committed capacity."
      - "Encode validation/apply exclusivity in the Rust API or recheck revision atomically at commit."
      - "Add cap, retirement-at-capacity, and intervening-mutation regressions."
  - truth: "Phase guardrails and parity evidence fail closed on lifecycle and scope drift"
    status: failed
    reason: "The apply-boundary checker inspects only lexical apply bodies and can be bypassed through an effectful helper. The deferred-scope guard checks exact sentences only in the root README. Canonical parity surfaces already say done while their own requirement/verification gate remains pending."
    artifacts:
      - path: scripts/check-phase134-apply-boundaries.ts
        issue: "No transitive call-graph or strict pure-helper enforcement."
      - path: scripts/check-phase134-authoritative-lifecycle.ts
        issue: "Deferred-claim coverage is exact-sentence and root-README-only."
      - path: scripts/check-phase134-authoritative-lifecycle.test.ts
        issue: "Mutations do not cover effectful helper indirection, parity surfaces, or wording variants."
      - path: docs/parity/index.json
        issue: "Phase 134 surface is done while known_gaps says MPLIFE requirements await phase verification."
      - path: docs/parity/checklist.md
        issue: "Phase 134 row is done while requirement checkboxes remain pending."
    missing:
      - "Reject unclassified/transitively effectful helper calls from protected apply bodies."
      - "Scan all canonical public/parity claim surfaces with normalized or structured claim checks."
      - "Keep Phase 134 parity status in_progress until gaps close and requirements are independently verified."
---

# Phase 134: Authoritative Cross-Cache Lifecycle Integration Verification Report

**Phase Goal:** Every package or mempool mutation has one authoritative, complete consequence across serving, relay, peer, compact, retry, persistence, and evidence state.
**Verified:** 2026-07-28T22:50:05Z
**Status:** gaps_found
**Re-verification:** No — initial verification

## Goal Achievement

### Observable Truths

| # | Roadmap truth | Status | Evidence |
| --- | --- | --- | --- |
| 1 | Package admission, pressure policy, maintenance, snapshots, relay queues, and transport receipts mutate runtime state only through `ManagedNetworkHandle`. | ✓ VERIFIED | Singleton/package admission and maintenance call the shared handle authority dispatcher. Effect preparation/completion also enters through handle methods. The peer-evidence follow-up is still non-atomic, but it does not introduce a mutation owner outside `ManagedNetworkHandle`. |
| 2 | One lifecycle delta projects admissions and removals into serving, fanout, peer request/known state, orphan/package candidates, compact inputs, unbroadcast state, persistence dirtiness, and operator evidence. | ✗ FAILED | `prepare_orphan_lifecycle` can remove an orphan by txid without removing a cursor that contains the orphan's different wtxid. The dependent peer target is therefore incomplete. |
| 3 | Replacement, pressure eviction, expiry, block connection, reorg, and failed admission leave no stale descendant or accepted-identity entries. | ✗ FAILED | Same-txid/different-wtxid orphan teardown can leave a stale candidate cursor, and the reconciliation oracle repeats the same blind spot. |
| 4 | Storage and network work occurs after authority release, with bounded typed receipts applied through a short follow-up mutation. | ✗ FAILED | I/O is outside the authority lock and types/nominal caps exist, but foreign receipts can consume another authority's pending effect, peer completion/evidence spans two locks, and pre-achievement failures permanently retain reservations. |

**Score:** 1/4 roadmap truths verified

The first truth passes only as a mutation-ownership statement. It does not neutralize the effect-family correctness failures under truth 4 or the phase goal's requirement that the consequence be authoritative and complete.

### Required Artifacts

| Artifact | Expected | Status | Details |
| --- | --- | --- | --- |
| `packages/open-bitcoin-mempool/src/pool/prepared_lifecycle.rs` | Sealed prepare/validate/apply lifecycle capability | ⚠️ PARTIAL | Exists, is substantive, and is wired, but the public validated capability can become stale between validation and apply. |
| `packages/open-bitcoin-node/src/network/lifecycle_projection.rs` | Closed command and complete target plan | ✓ VERIFIED | Exists and requires the concrete target family; no I/O occurs in its projection contract. `AuthorityEpoch::INITIAL` does not uniquely identify an authority incarnation. |
| `packages/open-bitcoin-node/src/network/lifecycle_projection/authority.rs` | Ordered infallible aggregate apply | ✓ VERIFIED | Core, compact, serving, fanout, peer, unbroadcast, persistence, and evidence application is wired in the required order. |
| `packages/open-bitcoin-network/src/peer/transaction_lifecycle.rs` | Bounded identity-complete peer projection | ✗ DEFECTIVE | Substantive and wired, but accepted-package count is unbounded, txid-alias cursor cleanup is incomplete, and fingerprint capacity is checked before retirements. |
| `packages/open-bitcoin-node/src/network/runtime_authority/lifecycle.rs` | Sole lifecycle/effect dispatcher | ✗ DEFECTIVE | Substantive and used, but effect completion consumes by raw effect ID before exact binding validation. |
| `packages/open-bitcoin-node/src/network/lifecycle_effects.rs` | Authority-bound typed receipts and bounded ledgers | ✗ DEFECTIVE | Family types and caps exist, but ledgers key pending state only by effect ID and have no abort path. |
| `packages/open-bitcoin-node/src/network/runtime_authority/effects.rs` | Thin effect facades | ⚠️ PARTIAL | Preparation/completion use `LifecycleCommand`; peer evidence is recorded in a second `try_mutate` lock. |
| `packages/open-bitcoin-node/src/storage/fjall_store/mempool.rs` | Outside-lock snapshot execution | ✓ VERIFIED | Consumes owned snapshot work, saves through the current schema, and acknowledges only after success. |
| `packages/open-bitcoin-rpc/src/inbound_listener/connection_runtime.rs` | Outside-lock peer write successful-prefix execution | ✓ VERIFIED | Encode/write precedes acknowledgement and shared completion; dedicated RPC tests exist. |
| `packages/open-bitcoin-node/src/network/tests/lifecycle_projection_cases/oracle.rs` | Independent completeness oracle | ⚠️ PARTIAL | Substantive and independent, but production reconciliation cannot detect missing expected unbroadcast entries or the txid-alias cursor case. |
| `scripts/check-phase134-authoritative-lifecycle.ts` | Fail-closed architecture/scope checker | ⚠️ PARTIAL | Live checker and 88 mutations pass; claim corpus and apply-helper coverage are too narrow. |
| `docs/parity/index.json` / `docs/parity/checklist.md` | Truthful phase state and evidence | ✗ INCONSISTENT | Both mark the surface `done` while stating MPLIFE-01 through MPLIFE-04 remain pending until phase verification. |

The mechanical artifact verifier reported 15/19 exact `contains` checks passing. The four misses are pattern drift rather than missing files: reorg tests moved/renamed, and Plan 12 diagnostic tokens changed. Manual inspection confirmed all four artifacts are substantive. All 13 declared key-link patterns were found, but semantic tracing exposed the defects above.

### Key Link Verification

| From | To | Via | Status | Details |
| --- | --- | --- | --- | --- |
| Admission and maintenance facades | `apply_lifecycle_command` | Typed admission/expiry/block/reorg commands | ✓ WIRED | Production entrypoints use the shared authority dispatcher. |
| Closed lifecycle plan | Core plus seven dependent targets | `validate_prepared_lifecycle` then ordered apply | ✓ WIRED | All targets are concrete and aggregate apply is unit-returning. |
| Lifecycle delta | Peer orphan/package state | `PreparedPeerTransactionLifecycle` | ✗ PARTIAL | Wiring exists, but txid-selected orphan removals do not necessarily clear cursors holding the removed orphan's actual wtxid. |
| Peer write | Receipt completion and evidence | `acknowledge_write` then `complete_peer_emission` | ✗ PARTIAL | Successful-prefix wiring exists; full completion/evidence is not atomic and foreign receipts can consume local pending IDs. |
| Fjall snapshot save | Snapshot completion | `SnapshotWriteReceipt` | ✗ UNSAFE | Save occurs outside the lock, but the receipt binding is not unique across authority instances. |
| Canonical state | Audit reconciliation | Fixed seven-target report | ✗ PARTIAL | Unbroadcast comparison is one-directional and peer cursor comparison shares the txid/wtxid blind spot. |

### Data-Flow Trace (Level 4)

| Artifact | Data variable | Source | Produces real data | Status |
| --- | --- | --- | --- | --- |
| Aggregate lifecycle projection | `PreparedLifecycleFacts` / `MempoolLifecycleDelta` | Real mempool admission and maintenance preparations | Yes | ⚠️ PARTIAL — data flows, but one peer alias edge is omitted. |
| Peer lifecycle projection | orphan removals and candidate removals | Canonical txid/wtxid identities plus current orphanage | Yes | ✗ HOLLOW EDGE — txid-selected removal does not propagate the removed orphan's actual wtxid to cursor cleanup. |
| Snapshot effect | owned `MempoolSnapshot` | Current managed mempool under authority, then Fjall outside lock | Yes | ✗ UNSAFE COMPLETION — foreign receipt collision can clear unrelated dirty state. |
| Peer emission effect | owned wire message and capability | Relay preparation, external write, typed receipt | Yes | ⚠️ PARTIAL — successful prefix is real, but completion/evidence is split and reservations leak on failure. |
| Reconciliation | canonical membership and dependent caches | Current authoritative aggregate | Yes | ⚠️ PARTIAL — two completeness checks are one-sided/alias-incomplete. |

### Behavioral Spot-Checks

| Behavior | Command or evidence | Result | Status |
| --- | --- | --- | --- |
| Protected target apply bodies remain lexically infallible | `bun run scripts/check-phase134-apply-boundaries.ts` | Eight protected apply bodies discovered; check passed | ✓ PASS |
| Phase 134 checker mutations | `bun test scripts/check-phase134-authoritative-lifecycle.test.ts` | 88 passed, 0 failed | ✓ PASS |
| Live lifecycle checker | `bun run scripts/check-phase134-authoritative-lifecycle.ts` | Passed | ✓ PASS |
| Parity JSON syntax | `jq empty docs/parity/index.json docs/parity/source-breadcrumbs.json` | Passed | ✓ PASS |
| Verifier shell syntax and diff hygiene | `bash -n scripts/verify.sh`; `git diff --check` | Passed | ✓ PASS |
| Documented phase implementation commits | `git cat-file -e HASH^{commit}` for all 30 plan task hashes | All present | ✓ PASS |
| Full repository contract | Plan 13 explicit `bash scripts/verify.sh` run; parent-provided repeated pass evidence | Passed in 14m20s and subsequently through the normal review hook | ✓ PASS (existing evidence) |
| Foreign receipt isolation | Static trace: every authority uses epoch 1 and effect ID 0 initially; completion records ID before full binding validation | Authority A's first receipt can consume authority B's first pending effect | ✗ FAIL |

Passing tests do not cover the failing receipt-collision, orphan-alias, missing-unbroadcast, helper-indirection, or claim-surface cases.

### Requirements Coverage

| Requirement | Source plans | Description | Status | Evidence |
| --- | --- | --- | --- | --- |
| MPLIFE-01 | 02, 05, 06, 07, 08, 12, 13 | `ManagedNetworkHandle` is the sole scoped runtime mutation authority. | ✓ SATISFIED | Admission/maintenance production paths and effect facades remain behind the handle. This does not mean all handle operations are semantically correct. |
| MPLIFE-02 | 01, 02, 03, 04, 05, 06, 11, 12, 13 | One delta projects all admission/removal consequences. | ✗ BLOCKED | Txid-alias orphan teardown can leave a stale candidate cursor; unbroadcast reconciliation also cannot detect missing expected members. |
| MPLIFE-03 | 01, 03, 04, 05, 06, 07, 11, 12, 13 | Lifecycle families leave no stale descendants or accepted identities. | ✗ BLOCKED | The stale cursor is dependent candidate state left behind after an authoritative member is removed. |
| MPLIFE-04 | 08, 09, 10, 11, 12, 13 | Effects execute outside authority and bounded typed receipts complete safely. | ✗ BLOCKED | I/O placement passes, but receipt identity/cross-authority isolation, atomic peer evidence, and failed-capability cleanup do not. |

The milestone requirement ledger intentionally remains pending. This verification does not promote any checkbox.

### Anti-Patterns and Correctness Findings

| File | Pattern | Severity | Impact |
| --- | --- | --- | --- |
| `network/relay_serving.rs`, `runtime_authority/lifecycle.rs`, `lifecycle_effects.rs` | Shared initial authority epoch and raw-ID completion | 🛑 Blocker | Cross-authority peer/snapshot receipts can consume another authority's pending effect and falsely clear dirty state. |
| `peer/transaction_lifecycle.rs` | Unbounded accepted-package count | ⚠️ Warning | Bounded lifecycle preparation can allocate/retain arbitrarily many fingerprint admissions. |
| `peer/transaction_lifecycle.rs` and peer reconciliation | Txid-alias cursor cleanup blind spot | ⚠️ Warning | Stale dependent candidate state violates MPLIFE-02 and MPLIFE-03. |
| `peer/transaction_lifecycle.rs` | Capacity checked before retirements | ⚠️ Warning | Valid replace-at-capacity transitions can be rejected by transient size. |
| `mempool/pool/prepared_lifecycle.rs` | Stale owned validated capability | ⚠️ Warning | An intervening mutation can be overwritten through safe public Rust APIs. |
| `runtime_authority/effects.rs` | Two-lock peer completion/evidence | ⚠️ Warning | Evidence can attach to a newer session or fail after irreversible completion. |
| `network.rs` / `runtime_authority/lifecycle.rs` | Global peer-session generation | ⚠️ Warning | Unrelated peer churn makes valid achieved writes stale. |
| Effect ledgers and executors | No capability abort path | ⚠️ Warning | Failed/dropped work permanently consumes bounded pending capacity. |
| `lifecycle_projection/reconciliation.rs` | One-way unbroadcast comparison | ⚠️ Warning | Missing expected unbroadcast state is invisible to the audit oracle. |
| `check-phase134-apply-boundaries.ts` | Lexical-only apply inspection | ⚠️ Warning | An effectful helper can bypass the no-I/O/fallibility guarantee. |
| `docs/parity/index.json`, `docs/parity/checklist.md` | Premature `done` state | ⚠️ Warning | Canonical parity state contradicts the pending verification gate. |
| Phase 134 lifecycle checker/tests | Root-README exact-sentence scope guard | ⚠️ Warning | Equivalent overclaims in canonical parity/package docs can bypass default verification. |

### Human Verification Required

None. The blocking failures are established by deterministic code traces and do not depend on visual behavior, external services, or subjective operator assessment.

### Deferred-Item Filter

No gap was deferred. Phase 135 owns schema/recovery, Phase 136 owns retry scheduling/fanout, Phase 137 owns operator presentation, and Phase 138 owns release/adversarial proof. None explicitly owns fixing Phase 134's receipt binding, dependent-cache cleanup, prepared-capability safety, or verification/parity guardrails.

### Gaps Summary

Phase 134 is not goal-complete despite the full verifier, targeted Rust suites, and mutation checkers passing. The core aggregate and most production wiring are real, but the phase promises authoritative and complete consequences, not merely a shared dispatcher:

1. A foreign effect receipt can be accepted by another authority and can clear snapshot dirty state without persisting that authority's mempool.
2. A txid-selected orphan removal can leave a same-entry wtxid cursor behind while reconciliation reports clean.
3. Lifecycle bounds and prepared-capability exclusivity have uncovered correctness holes.
4. Structural and parity guardrails overstate what they detect and prematurely label the surface done.

These are implementation and verification gaps within Phase 134, not human-verification items or later-phase deferrals. The escalation gate should route them back for gap-closure planning before any MPLIFE requirement promotion.

***

_Verified: 2026-07-28T22:50:05Z_
_Verifier: the agent (gsd-verifier)_
