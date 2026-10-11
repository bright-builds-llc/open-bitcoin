---
phase: 160-explicit-bounded-peer-filter-serving
plan: "05"
subsystem: network
tags: [bip157, exact-proof, stale-branches, bounded-ancestry, validation-history]
requires:
  - phase: 160-02
    provides: Own-session advertised BASIC capability
  - phase: 160-04
    provides: Exact 256-bit proof and signed proof-equivalent time
provides:
  - Incremental and recovered exact header proof facts independent of synthetic fork choice
  - Ordered semantic request validation and bounded stop-branch ancestry selection
  - Opaque shared-authority branch capture using genuine accepted validation history
affects: [160-06, 160-08, 160-11, 160-12]
tech-stack:
  added: []
  patterns: [separate semantic and resource outcomes, read-only authority captures, checked direct active lookup]
key-files:
  created:
    - packages/open-bitcoin-network/src/header_store/proof.rs
    - packages/open-bitcoin-network/src/header_store/proof/tests.rs
    - packages/open-bitcoin-network/src/header_store/tests.rs
    - packages/open-bitcoin-network/src/filter_serving/policy.rs
    - packages/open-bitcoin-network/src/filter_serving/policy/tests.rs
    - packages/open-bitcoin-node/src/network/runtime_authority/filter_index/branch.rs
    - packages/open-bitcoin-node/src/network/runtime_authority/filter_index/branch/tests.rs
  modified:
    - packages/open-bitcoin-network/src/header_store.rs
    - packages/open-bitcoin-network/src/filter_serving.rs
    - packages/open-bitcoin-network/src/lib.rs
    - packages/open-bitcoin-node/src/network/runtime_authority/filter_index.rs
    - packages/open-bitcoin-node/src/network.rs
    - packages/open-bitcoin-node/src/lib.rs
key-decisions:
  - Derive proof metadata within HeaderStore without persisting another index or changing synthetic fork choice.
  - Equal exact work uses deterministic height then hash ordering for recovery consistency; this is an intentional local convention, not identical Knots admission-sequence or pointer tie semantics.
  - Capture private authority/session/lifecycle facts and expose branch data only through immutable accessors.
  - Preserve semantic disconnect precedence over local saturation, while charging bounded semantic metadata conservatively.
requirements-addressed: [CFNET-02, CFNET-03, CFNET-04, CFNET-05]
requirements-completed: []
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 160-20261010-peer-filter-serving
generated_at: "2026-10-10T19:34:18Z"
duration: approximately 17min
completed: 2026-10-10
---

# Phase 160 Plan 05: Exact Branch Authorization Summary

**Exact target-derived header proof, strict signed stale ages, and genuine scripts-valid history now authorize bounded identities from the requested stop branch.**

## Accomplishments

- HeaderStore maintains derived exact cumulative work on admitted headers and accepted positions, and rebuilds it in height/hash order during recovery. Invalid targets, header-hash mismatches and unusable parent/height facts stay unavailable. No persisted namespace, filter cache or request-time history recomputation was added.
- The pure policy applies BASIC/own-advertisement, known/allowed stop, reverse-range and inclusive family limits in pinned order. Stale stops require ScriptsValid and both timestamp and exact proof-equivalent age strictly below 2,592,000 seconds. Future timestamps retain signed behavior.
- Ancestry admission computes finite vertex and identity-byte costs before allocation or traversal. Each visited vertex performs one header point lookup and verifies its expected decreasing height. Filters/headers use inclusive ranges; headers capture start-minus-one separately. Checkpoints select positive multiples of 1,000 without a protocol stop-height cap.
- One shared authority turn captures the peer's actual local-version services/session, direct checked active position, exact proof facts, genuine validation provenance and lifecycle identity. It reads no body/undo/filter payload, readiness barrier or alternate store. Captured branch and authority fields are private, with read-only accessors.

## Task Acceptance

### Task 1: Maintain exact serving proof facts

- [x] A shorter harder fork supplies best exact-header evidence independently of synthetic u128 best_tip.
- [x] Mixed recovered insertion order, deterministic recovered equal-work ties, invalid parent/identity/height facts, seed reset and nonuniform accumulation have passing evidence.
- [x] The existing synthetic fork-choice contract and durable schema remain unchanged.

### Task 2: Ordered request and bounded ancestry policy

- [x] Type/capability, unknown/disallowed stop, reverse and range failures retain semantic precedence over saturated local work.
- [x] Inclusive 1,000 cfilters and 2,000 cfheaders pass; 1,001 and 2,001 reject. Genesis predecessor handling is explicit.
- [x] Checkpoint stops 0/999/1000/1001/2000/3001 select only positive interval identities; u32::MAX stop height is semantically valid and locally work-refused.
- [x] Future timestamps, exact timestamp/proof 30-day boundaries and one-below boundaries pass their signed policy assertions.
- [x] A close-timestamp, short-height, nonuniform-work branch rejects using the selected best header's own easy target denominator, defeating timestamp/height/synthetic-score shortcuts.
- [x] Requested stale ancestry and predecessor differ correctly from active heights. Missing or changed ancestry is a local refusal.

### Task 3: Bind genuine shared authority

- [x] A genuinely connected then disconnected child retains ScriptsValid and authorizes after real paired body/undo pruning.
- [x] Header-only, raw-body and raw-header-snapshot controls remain NeverConnected/UnknownLegacy and cannot grant stale eligibility.
- [x] Ordinary active membership requires checked height-offset lookup and exact hash equality; missing peers cannot supply advertised capability.
- [x] Invalid requests retain disconnect precedence with zero local metadata/ancestry budgets.

## TDD and Verification Evidence

All Cargo commands used pinned Bun 1.3.9 and `bun run scripts/command-timings.ts run --key <stable-key> --`, respecting the cooperative target-directory lock. Queued sessions were polled; the other executor's live regression was not interrupted.

| Stage | Cargo command after timing wrapper | Actual result |
| -- | -- | -- |
| Task 1 RED | `test --manifest-path packages/Cargo.toml -p open-bitcoin-network phase160_header_proof -- --nocapture` | 1 executed, 1 failed because exact facts were unavailable before implementation. |
| Task 1 final GREEN | Same command | 4 executed, 4 passed, zero ignored. |
| Task 2 RED | `test --manifest-path packages/Cargo.toml -p open-bitcoin-network phase160_range_policy -- --nocapture` | 1 executed, 1 failed because ancestry resolution refused before implementation. |
| Task 2 final GREEN | Same command | 9 executed, 9 passed, zero ignored. |
| Task 3 RED | `test --manifest-path packages/Cargo.toml -p open-bitcoin-node phase160_branch -- --nocapture` | 1 executed, 1 failed on the expected pre-implementation local refusal. |
| Task 3 final GREEN | Same command | 4 executed, 4 passed, zero ignored. |
| Network regression | `test --manifest-path packages/Cargo.toml -p open-bitcoin-network --lib` | 577 executed, 577 passed; subsequent final focused runs additionally verified the new nonuniform-age regression and recovered tie assertion. |
| Final targeted lint | `clippy --manifest-path packages/Cargo.toml -p open-bitcoin-network -p open-bitcoin-node --all-targets --all-features -- -D warnings` | Passed. |
| Scoped formatting | `rustfmt --edition 2024 --config skip_children=true --check` on all 13 owned Rust paths | Passed. |
| Whitespace/source scope | `git diff --check` and scoped diff/read review | Passed. |

The initial node test setup required explicit CoinsCache/FlushLifecycle construction and ScriptVerifyFlags::P2SH before its nonzero behavioral RED/GREEN. Clippy identified a large outcome variant; boxing only the authorized capture resolved it. No dependency or runtime workaround was added. Nonmatching existing integration binaries ran zero matching tests; required unit filters each executed the nonzero counts above.

Native full verification, broad workspace build, coverage, independent source/security review and lifecycle-valid requirement completion remain the parent consolidated gate. No achieved socket-serving claim is made by these branch tests.

## Exported Interfaces and Downstream Accounting

- `HeaderStore::{maybe_proof_facts, maybe_best_proof_header}` returns `HeaderProofFacts` containing exact cumulative work, block proof, selected hash/height and timestamp. Facts do not assert scripts validation.
- `open_bitcoin_network::policy` exports `FilterRequest`, `StopProvenance`, `validate_filter_request`, private-field `ValidatedFilterRequest`, `resolve_filter_branch`, `BranchWorkLimit`, `AuthorizedFilterBranch`, `FilterBranchIdentity`, `FilterRequestInvalid` and `BranchResourceRefusal`.
- `ManagedNetworkHandle::capture_basic_filter_branch(peer_id, request, params, BasicFilterBranchLimit)` returns `BasicFilterBranchOutcome::{Invalid, ResourceRefused, Authorized(Box<BasicFilterBranchCapture>)}`. Internal `capture_network` is visible within crate::network for complete preparation.
- Capture getters expose `branch()`, `metadata_units()`, `authority_incarnation()`, `peer_id()`, `peer_session_generation()`, `own_services()`, `maybe_index_generation()` and `maybe_index_branch_identity()`. No public constructor accepts storage-authorizing provenance assertions.
- One ancestry unit represents one visited-header lookup. Identity storage charges 68 bytes per selected identity, predecessor and captured stop. This is not a filter-copy, encoded-payload or socket-queue budget; Plans 06/08 retain those responsibilities.
- Active metadata charges 6 units; stale metadata conservatively charges `MAX_VALIDATION_IDENTITIES + 10`, currently 138. The existing sealed pending-validation batch is bounded to 128 identities by acceptance validation; durable provenance has constant coverage/point lookup cost. Fixed bounded semantic reads happen before resource refusal to preserve invalid-request precedence, and their charge is included in admission accounting.
- Plan 08 must preserve the existing separate pure semantic validator and expensive bounded resolver when placing aggregate reservation between them. This plan implements branch authorization and local work admission; it does not claim the later request reservation ledger or complete persisted preparation.

## Intentional Tie Convention and Parity Handoff

The serving exact-header selector orders cumulative ProofWork, then height, then hash. Recovery and insertion produce the same equal-work selection. Knots also has admission-sequence/pointer tie paths; this implementation deliberately does not persist those identities or claim identical tie semantics. Parent approved the deterministic convention and assigned Plan 12 its explicit scoped parity documentation. Synthetic best_tip ordering is unchanged. Stale checks always use the chosen exact header's own timestamp and target proof.

## Breadcrumb Registration Handoff

The parent serially owns source-breadcrumbs.json; this executor did not mutate it. Parent reported registration complete at 1,071 Rust paths and managed starter checks with zero findings. All seven new paths contain pinned in-source comments:

- `header_store/proof.rs` and `header_store/proof/tests.rs`: ordered anchors `packages/bitcoin-knots/src/chain.cpp`, `packages/bitcoin-knots/src/net_processing.cpp`.
- `filter_serving/policy.rs` and `filter_serving/policy/tests.rs`: ordered anchors `packages/bitcoin-knots/src/net_processing.cpp`, `packages/bitcoin-knots/src/chain.cpp`.
- Node `runtime_authority/filter_index/branch.rs` and `branch/tests.rs`: ordered anchors `packages/bitcoin-knots/src/net_processing.cpp`, `packages/bitcoin-knots/src/chain.h`, `packages/bitcoin-knots/src/validation.cpp`.
- Network `header_store/tests.rs`: preserved ordered anchors `packages/bitcoin-knots/src/headerssync.cpp`, `packages/bitcoin-knots/src/sync.cpp`, `packages/bitcoin-knots/src/net_processing.cpp`, `packages/bitcoin-knots/src/node/txdownloadman_impl.cpp`, `packages/bitcoin-knots/src/node/txdownloadman.h`, `packages/bitcoin-knots/test/functional/p2p_handshake.py`, `packages/bitcoin-knots/test/functional/p2p_initial_headers_sync.py`.

## Deviations from Plan

1. **[Rule 2 - Repo standard enforcement] Extracted existing inline header tests.** Adding proof ownership grew header_store.rs from 621 to 631 lines, exceeding the managed 628-line check. With parent approval, moved the unchanged existing test bodies to header_store/tests.rs and preserved their anchors. Production is now 329 lines; no unrelated cleanup occurred.
2. **[Rule 3 - Public API wiring] Added narrow exports in network/lib.rs and node/network.rs plus node/lib.rs.** The existing private module boundaries otherwise made planned downstream types inaccessible. Parent explicitly approved these additional paths.

AGENTS.md, the Bright Builds sidecar, placeholder-only overrides, architecture/code-shape/testing/verification/operability/Rust standards, finalized context/research and decision coverage informed implementation. Parent-loaded active lessons and synchronized baseline were honored.

## Task Commits and Shared State

Tasks 1/2/3 and this summary await the strict outer consolidated commit. No staging, commits, pushes, branches, worktrees, hook bypass, STATE/ROADMAP/REQUIREMENTS/todo or shared breadcrumb-manifest mutations occurred. Requirement completion intentionally remains empty until final phase verification.

## Security and Simplification Review

- T-160-13: Genuine ScriptsValid plus strict signed timestamp and exact proof age authorize stale stops; payload presence is irrelevant.
- T-160-14: Requested stop-parent identities and checked decreasing heights preserve stale ancestry and predecessor identity.
- T-160-15: Upfront vertex/identity-byte admission bounds allocation and traversal; every visited header is charged, and local refusal has no invented protocol height cap.
- Simplification retained one exact-facts owner, one pure validator/resolver pair and one thin authority adapter. Removed the extra parent-prefetch from traversal so one charged vertex equals one actual header lookup. No alternate store, new dependency, public listener or schema was introduced.
- Stub scan found no TODO/FIXME/placeholder implementation. No unmodeled endpoint, auth path or filesystem/schema boundary was added.

## Authentication Gates and Deferred Issues

None. Plan 12 parity documentation for the explicit tie convention and the parent consolidated verification/commit gate remain assigned workflow work.

## Self-Check: PASSED

All seven new Rust files, six modified Rust files and this summary exist. Final focused tests executed 4/9/4 passing cases, targeted Clippy passed, and formatting/whitespace checks passed. Commit-existence checks are deliberately inapplicable because the outer lifecycle forbids intermediate commits; no commit hash or requirement-completion claim is fabricated.
