---
phase: 165-shared-exact-work-authority
plan: "05"
subsystem: network
tags: [proof-work, selected-parameters, header-admission, contextual-prefix, parity]
requires:
  - phase: 165-04
    provides: Accepted-header ancestry and sole strict exact-work selector
provides:
  - Selected-parameter managed Headers admission through all three dispatch routes
  - Immutable full consensus parameters preserved across owner clones
  - Whole-message proof/continuity refusal and sequential contextual prefix retention
  - First-parent connection enforcement including known genesis refusal
  - Explicit checked fixture re-admission with header-known-only history publication
  - Target-derived numerical body-work addition without granting body provenance
affects: [165-06, 165-07, 165-08, 165-09, 165-12, 165-13, 165-14]
tech-stack:
  added: []
  patterns: [One accepted header selector, explicit fixture initialization, numbers separated from lineage]
key-files:
  created:
    - packages/open-bitcoin-node/src/network/tests/phase165_headers.rs
  modified:
    - packages/open-bitcoin-node/src/network.rs
    - packages/open-bitcoin-node/src/network/header_sync.rs
    - packages/open-bitcoin-node/src/network/validation_history.rs
    - packages/open-bitcoin-node/src/network/canonical_admission.rs
    - packages/open-bitcoin-node/src/network/inventory.rs
    - packages/open-bitcoin-node/src/network/mempool_lifecycle.rs
    - packages/open-bitcoin-node/src/network/action_translation.rs
    - packages/open-bitcoin-node/src/network/runtime_authority.rs
    - packages/open-bitcoin-network/src/peer/message_dispatch.rs
    - scripts/check-phase164-canonical-identity/contracts.ts
    - scripts/check-phase164-canonical-identity.test.ts
    - docs/parity/source-breadcrumbs.json
key-decisions:
  - "Managed owners bind full selected ConsensusParams; clones preserve the binding."
  - "PoW/continuity faults reject whole messages; contextual faults retain accepted prefixes and arrival precedence."
  - "P2P requires the first header's parent in the accepted index, even for known genesis duplicates."
  - "Explicit fixture admission rechecks real headers and ignores imported work numbers; it grants no body lineage."
  - "Early body numerical correction adds block_proof of the actual header; provenance and preference authorization remain Plans07/08."
patterns-established:
  - "Controlled fixture/root initialization stays separate from ordinary P2P."
  - "Cold numerical imports remain unproved until an explicit fresh model admission."
requirements-completed: ["CWOR-01", "CWOR-02"]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 165-2026-10-14T16-09-59Z
generated_at: 2026-10-14T20:11:09Z
duration: 86min
completed: 2026-10-14
---

# Phase 165 Plan 05: Managed Selected-Work Admission Summary

**All managed Headers routes enforce selected PoW, accepted parent connection and exact work, preserving the pin's contextual prefix order and canonical refusal.**

## Performance

- First recorded verification: 2026-10-14T18:45:24.983Z; initial context loading was not separately timed.
- Summary written: 2026-10-14T20:11:09Z.
- Duration: 86min from that first timing record.
- Tasks: 2/2.
- Ownership inventory: 46 source, fixture, checker and breadcrumb paths in the approved plan; one new Rust test file, plus this summary.

## Accomplishments and APIs

- Ordinary, durable-serving and sync Headers dispatch now reach the selected checked owner. Ordinary/durable dispatch retain HeadersAndBlocks policy; sync retains HeadersOnly. No generic unbound insertion bypass remains in these routes.
- The owner binds full ConsensusParams, preserving them in clones and rejecting changes before effects. Shared check_block_header_with_params checks normalized target, selected powLimit and actual hash before accepted insertion.
- Entire-message PoW and internal-continuity faults publish no prefix. Continuous, PoW-valid headers then pass contextual admission sequentially: a later difficulty/MTP/time failure retains genuinely accepted prefix headers, known-only history and first equal-work precedence.
- The first P2P header must have an accepted parent. Both unknown and already-known null-parent genesis announcements refuse through the existing bounded MissingHeaderAncestor error; no P2P initialization or presync qualification is claimed. Non-root duplicates preserve their accepted work.
- try_admit_fixture_headers on the owner and handle explicitly re-admits real bytes into a fresh selected store, verifies identities/heights, ignores imported scores and publishes only header-known identities. Existing ScriptsValid facts remain separately owned. A bound owner cannot be replaced/reseeded; subsequent headers use ingress. Raw seed_header_store remains unproved and cannot overwrite a bound receipt.
- Converting a fixture owner into an immutable canonical scope clears fixture header receipts. All four canonical ordinary scopes continue to refuse before peer, header, history or cache effects. Mainnet/testnet3 root identity remains distinct despite their shared powLimit.
- Peer follow-up header progress compares accepted work, rather than height. A legal mineable model uses delayed minimum difficulty, retargeting and recovery to prove that a shorter stronger branch controls the actual owner, handle queries, locator and GetHeaders request.
- The parent-approved early numerical dependency correction makes next_chain_work consume the actual incoming header, add block_proof(bits) to the current numerical parent or ZERO modulo 2^256, and propagate invalid compact errors. It certifies no imported parent or body lineage. Local and received-block callers pass their actual header; invalid compact input fails before compact cleanup.
- Current Phase164 contracts cover eleven additional actual functions and three additional fixed inputs, including selected admission, first-parent connection and numerical target use. Historical snapshots, manifests, wire fixtures and qualification receipts were preserved.

## Task Commits

Task, RED/GREEN and metadata commits are **deferred to the parent strict finalization** after whole-phase native verification, independent review/security/goal checks and normal hooks. This executor staged, committed and pushed nothing, and did not edit shared STATE, ROADMAP, REQUIREMENTS, config or todo files.

## Executed Evidence

All Cargo commands ran sequentially through command-timings with command-local Bun1.3.9 and pinned Rust1.94.1. Use this prefix for each command below:

```bash
PATH='/Users/peterryszkiewicz/.bun/install/cache/@oven/bun-darwin-aarch64@1.3.9@@@1/bin:'"$PATH"
```

| Check                                                                                                                                                             | Actual result                                                                                                                                                                                                                             |
| ----------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| phase165-live-headers: cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --all-features phase165_headers                                        | Meaningful RED reproduced ordinary Headers accepting an above-selected-limit target; initial corrected fixture run2/3. Initial implementation GREEN3/3; extended final header/numerical coverage15 cases.                                 |
| phase165-header-affected-fixtures: scoped libtest filters with --test-threads=4                                                                                   | Diagnostic batches progressed100/116,109/116,113/116 and114/117. The last three concrete failures were corrected and all passed in the final focused retest below. These intermediate batches are not a single all-green117-case receipt. |
| phase165-header-last-regressions: scoped header/numerical tests plus the last three failing fixture cases                                                         | Final18 passed, zero failed/ignored. Includes15 header/numerical cases, invalid-body attribution, explicit durable model root settlement and post-catch-up work/status.                                                                   |
| phase165-header-relocation-smoke: phase165_headers, header_checkpoint and canonical fixture binding filters                                                       | After the structural moves,28 passed, zero failed/ignored;13.96s compile plus7.28s execution.                                                                                                                                             |
| phase165-header-block-requests: cargo test … sync::tests::block_requests --quiet                                                                                  | 9 passed, zero failed/ignored;4.86s.                                                                                                                                                                                                      |
| Canonical ownership in the bounded affected batch                                                                                                                 | All13 canonical cases passed; an earlier separate run had one temporary backend-lock failure before any guard, later absent in the bounded runs.                                                                                          |
| phase165-fixture-node-compile: cargo check --manifest-path packages/Cargo.toml -p open-bitcoin-node --all-targets --all-features                                  | Passed before the final structural moves; final all-target clippy compiled the moved targets.                                                                                                                                             |
| phase165-header-clippy: cargo clippy --manifest-path packages/Cargo.toml -p open-bitcoin-node -p open-bitcoin-network --all-targets --all-features -- -D warnings | Final post-move exit0,8.60s.                                                                                                                                                                                                              |
| phase165-header-format: cargo fmt --manifest-path packages/Cargo.toml --all                                                                                       | Passed, including the final moved code.                                                                                                                                                                                                   |
| bun test ./scripts/check-phase164-canonical-identity.test.ts                                                                                                      | 376 passed, zero failed;452 expectations;87.37s. Afterwards only the reviewed method relocation/delegate edge was renewed; the actual current checker passed again.                                                                       |
| bun run scripts/check-phase164-canonical-identity.ts                                                                                                              | Final passed:96 fixed Rust inputs,257 reviewed source edges; no canonical qualification receipt generated.                                                                                                                                |
| bun scripts/check-parity-breadcrumbs.ts --check                                                                                                                   | Passed:1,166 Rust files. No new source file lacks breadcrumbs.                                                                                                                                                                            |
| bun scripts/bright-builds-check.ts all                                                                                                                            | Final passed:1,521 scanned files, zero exceptions/findings; active lesson check also passed.                                                                                                                                              |
| Owned source/current-contract diff, stub/threat scan and git diff --check                                                                                         | Passed; no goal-blocking stub, new dependency, endpoint or unregistered threat surface. Git index is empty.                                                                                                                               |

The early full node-library diagnostic reported1716 passed,44 failed,4 ignored in794.75s before the fixture and dependency corrections. It was used to inventory affected paths and is not final acceptance. No whole-workspace/native/Bazel acceptance is claimed here; Plan14 and the parent own that contract.

## Decisions and Simplification

AGENTS.md, the Bright Builds sidecar, placeholder-only overrides, architecture/code-shape/verification/testing/Rust/TypeScript standards and both complete active lesson files (7,188 bytes;2,397 estimated tokens) informed execution. The parent owned initial sync/bootstrap.

The simplification pass reused shared selected header validation and the existing known-only history publisher. To meet managed file caps without exceptions, the redundant header delegate was removed, transient_runtime moved into the existing canonical/fixture constructor module, handshake/hash/cleanup helpers moved to existing peer_evidence, and stale checkpoint setup moved to existing faults. Public APIs and assertions were preserved; no new Rust module was required.

## Deviations from Plan

1. **[Rule3 - Blocking; parent approved] Actual owner inventory expansion.** The real selected ingress was in validation_history; ordinary and durable dispatch bypassed it, and request progress used height. Eight additional production owner paths (including the narrowly approved local-block callsite) and29 exact fixture paths were approved and added to the plan before edits. Raw saved/imported numbers remain explicitly unproved; positive models use actual selected admission and, where needed, explicit named genesis body setup before P2P.
1. **[Rule1 - Bug; parent-approved dependency correction] Numerical body addition.** The temporary +1 helper disagreed with accepted header proof, breaking meaningful at-tip/equal-work assertions. The actual header's proof now supplies the addend, with errors propagated. Plans07/08 retain all provenance, sealed-stage, accepted-header agreement and preference authorization work.
1. **[Rule2 - Correctness; pinned-source refinement] Batch and genesis distinctions.** Pinned net_processing whole-message checks and validation sequential contextual processing required separate behavior, rather than blanket rollback. The first-parent lookup precedes already-known shortcuts, so known genesis duplicates also refuse. Dedicated prefix/equal-work, continuity, genesis and non-root duplicate controls establish those boundaries.
1. **[Rule3 - Blocking] File-cap restructuring.** Four over-cap files were reduced through the approved existing-module relocations above. No cap exclusion or assertion weakening was added.

## Scope Limits and Next Plan Readiness

No body or coins lineage token is minted by arithmetic, decoded work, header-known publication or fixture re-admission. The early numerical helper may read a raw numerical parent; Plans07/08 must still refuse unearned parents/stages and enforce actual body-owner authorization. Plan06 owns body/ancestor eligibility and arrival order; Plan09 owns actual candidate reconciliation.

Ordinary canonical bootstrap, exact checkpoint/all-handle reopening authority and genesis BASIC remain Phase166; complete contextual prerequisites remain Phase167; paired actual-daemon qualification remains Phase168. Networking defaults and the four selector support/refusal classes are unchanged. Explicit model re-admission is not an ordinary reopening qualification, and imported/restart tie order has no historical Knots claim.

## Self-Check: PASSED

All declared owned paths and the new mapped Rust test exist. Current source guards, focused remaining regressions, post-relocation smoke, all-target clippy, formatting, managed caps, breadcrumbs and diff checks pass. No executor sessions remain. Commit existence and shared tracker updates are intentionally deferred to the authorized parent wrapper.
