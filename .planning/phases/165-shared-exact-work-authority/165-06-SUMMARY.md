---
phase: 165-shared-exact-work-authority
plan: "06"
subsystem: network
tags: [body-eligibility, proof-work, fifo, transaction-linkage, parity]
requires:
  - phase: 165-05
    provides: Selected-parameter accepted header ancestry and managed ingress
provides:
  - Checked-body transaction linkage independent of header arrival
  - Per-parent insertion-order FIFO breadth-first descendant release
  - All retained eligible candidates ordered by exact work and earned sequence
  - Historical linkage separated from current body availability and failed suffixes
  - Managed body checks and read-only owner/handle eligibility queries
affects: [165-07, 165-08, 165-09, 165-12, 165-13, 165-14]
tech-stack:
  added: []
  patterns: [Shared pure context derivation, checked body bytes before linkage, historical linkage versus availability]
key-files:
  created:
    - packages/open-bitcoin-network/src/header_store/eligibility.rs
    - packages/open-bitcoin-network/src/header_store/eligibility/tests.rs
    - packages/open-bitcoin-network/src/peer/eligibility.rs
    - packages/open-bitcoin-node/src/network/candidate_eligibility.rs
    - packages/open-bitcoin-node/src/network/tests/phase165_eligibility.rs
  modified:
    - packages/open-bitcoin-network/src/header_store.rs
    - packages/open-bitcoin-network/src/lib.rs
    - packages/open-bitcoin-network/src/peer.rs
    - packages/open-bitcoin-node/src/network.rs
    - packages/open-bitcoin-node/src/network/header_sync.rs
    - packages/open-bitcoin-node/src/network/mempool_lifecycle.rs
    - packages/open-bitcoin-node/src/network/runtime_authority.rs
    - packages/open-bitcoin-node/src/network/runtime_authority/prune_flush.rs
    - packages/open-bitcoin-node/src/network/tests.rs
    - docs/parity/source-breadcrumbs.json
    - scripts/check-phase164-canonical-identity/contracts.ts
    - scripts/check-phase164-canonical-identity.test.ts
key-decisions:
  - "TRANSACTIONS-valid checked retained bodies and historical ancestor linkage earn sequence before script/UTXO connection."
  - "Pending children release by each parent's insertion order and FIFO breadth-first traversal, never global raw-arrival sorting."
  - "Full selected ConsensusParams and accepted ancestry derive body context internally; public delegates accept actual bytes, time and parameters."
  - "Historical sequence survives pruning; current missing nonactive suffixes and marked failed ancestry cannot rank."
  - "Raw unknown-parent cache dispositions remain unproved and earn no eligibility."
patterns-established:
  - "One HeaderStore context builder serves header and full-body contextual checks."
  - "Read-only eligibility facts are not accepted-coins lineage or activation capabilities."
requirements-completed: ["CWOR-02", "CWOR-03"]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 165-2026-10-14T16-09-59Z
generated_at: 2026-10-14T20:36:46Z
duration: 20min
completed: 2026-10-14
---

# Phase 165 Plan 06: Body and Ancestor Eligibility Summary

**Actual checked bodies earn transaction-linkage sequence independently of header order, with FIFO breadth-first orphan release and separate availability/failed-ancestry activation facts.**

## Performance

- First recorded verification: 2026-10-14T20:16:36.557Z; initial context loading was not separately timed.
- Summary finalized: 2026-10-14T20:36:46Z.
- Duration: approximately 20min from that first timing record.
- Tasks: 2/2.
- Ownership: 17 approved source/checker/breadcrumb paths, including five new Rust files, plus this summary. The parent amended and formatted the plan before ownership expansion.

## Accomplishments and APIs

- HeaderStore owns a pure body registry. Its sequence starts at 2 and can be earned only through an actual full-body/selected-context check. Raw imported work/header records, a cached Block, local-known-hash bookkeeping and unaccepted ancestry do not produce an earned sequence.
- HeaderStore now binds full ConsensusParams rather than only powLimit. Selected parameter changes, selected-to-unbound downgrade and unbound-to-selected ancestry promotion refuse. Generic unbound header admission retains its existing explicitly unbound meaning.
- HeaderStore::header_validation_context reuses the existing header context, retarget and minimum-difficulty recovery logic relocated from the node adapter. Header and body paths share that implementation. Body checking derives height, prior header and MTP from accepted ancestry; callers cannot supply a fabricated context or sequence.
- HeaderStore::check_body_eligibility and the narrow PeerManager delegate check actual Block bytes with selected header proof and check_block_contextual. This covers full transaction structure, Merkle integrity, contextual finality/BIP34/witness/weight prerequisites before registry mutation. No spend/script or accepted-coins claim follows from this check.
- Checked bodies whose parent transaction linkage is missing remain pending. When ancestry links, a VecDeque releases each parent's children in insertion order using push_back/pop_front breadth-first traversal. A later-received sibling precedes an earlier grandchild when that is what the pinned traversal requires. Present-body duplicates preserve the original sequence and pending position.
- eligible_body_candidates enumerates every retained eligible candidate by greatest shared ProofWork, then smallest earned sequence, including the active candidate's actual sequence. Header arrival, hash and height do not break a defined body-activation tie.
- Historical linkage survives body removal. Missing nonactive suffix data excludes that branch; reaching an active ancestor does not require its pruned bytes. Explicit failure facts exclude descendants. Local/stored script or UTXO validation errors mark the exact failed body; storage errors are not classified as consensus failure.
- Local/stored/reorg body owners retain checked bytes and invoke the linkage seam before later coins/script connection. The existing raw disconnected/nonextending cache disposition remains available when accepted header ancestry is absent, while eligibility stays absent. This preserves the existing receipt contract without certifying raw cache data.
- ManagedPeerNetwork and ManagedNetworkHandle expose read-only maybe_body_eligibility and eligible_body_candidates. Successful physical prune removals and the existing test payload-removal seam revoke availability without deleting historical sequence.
- Node queries additionally require actual retained bytes throughout the nonactive candidate suffix. Real bytes checked through the public peer delegate alone do not populate node retention and cannot rank there. Fresh managed checked readmission restores availability while preserving the earned sequence; active historical ancestors remain exempt from requiring pruned bytes.
- Open Bitcoin resource policy bounds retained registry entries using the existing 50,000-item MAX_INV_SIZE budget and checks counter exhaustion before publishing a release. This is an explicit project policy; no corresponding registry cap or checked exhaustion is attributed to the pin.

## Task Commits

Task, RED/GREEN and metadata commits are **deferred to the parent strict finalization** after whole-phase native verification, independent review/security/goal checks and normal hooks. This executor staged, committed and pushed nothing and did not edit shared STATE, ROADMAP, REQUIREMENTS, config or todo files. No commit hashes or qualification receipts are invented.

## Executed Evidence

All Cargo commands ran sequentially through command-timings with command-local Bun1.3.9 and pinned Rust1.94.1. Each Bun/Cargo command used:

```bash
PATH='/Users/peterryszkiewicz/.bun/install/cache/@oven/bun-darwin-aarch64@1.3.9@@@1/bin:'"$PATH"
```

| Check                                                                                                                                          | Actual result                                                                                                                                                                                                                                                                                                                      |
| ---------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| phase165-eligibility-red: network phase165_eligibility filter                                                                                  | Meaningful RED: 1 discovered case failed because all released sequences remained None instead of 2/3/4/5; exit101. First GREEN1/1, expanded GREEN9/9. Final network suite contains17 new eligibility cases.                                                                                                                        |
| phase165-managed-eligibility-red: node phase165_eligibility filter                                                                             | Meaningful RED: local connection had no earned body sequence; 1 discovered case failed, exit101. First GREEN1/1. Expanded diagnostic9/10 exposed an invalid fixture assumption about imported header order; the test now uses real ordered P2P Headers ingress and passed.                                                         |
| phase165-eligibility-network-regression                                                                                                        | Full network regression626 unit,2 parity and2 property tests passed before the additional direct pure context/delegate cases.                                                                                                                                                                                                      |
| phase165-eligibility-core-coverage-final: cargo llvm-cov -p open-bitcoin-network --all-features --show-missing-lines --summary-only -- --quiet | Latest634 unit,2 parity and2 property tests passed. No handwritten Uncovered Lines reported. Literal summary: eligibility283/284 lines99.65%, peer/eligibility15/15 lines100%, whole network99.71%; these literal percentages are not relabeled as100%. No coverage exclusion was added.                                           |
| phase165-eligibility-affected-node                                                                                                             | Initial diagnostic107/115 passed,8 failed: six raw unknown-parent receipt contracts and two reorg event-clock fixtures. Root-cause adjustments preserved those contracts; subsequent115/115 passed.                                                                                                                                |
| phase165-eligibility-final-node                                                                                                                | Before the final retention refinement,118/118 passed. Final phase165-eligibility-retention-node119/119 passed, zero failed/ignored; includes14 new managed cases,15 predecessor header cases and existing disposition/mempool/pruning/serving cases. Compile10.64s, execution7.77s.                                                |
| phase165-eligibility-clippy: node/network --all-targets --all-features -- -D warnings                                                          | Final exit0,14.60s; the first run found one nonminimal boolean expression, corrected without changing its meaning.                                                                                                                                                                                                                 |
| phase165-eligibility-format: cargo fmt --manifest-path packages/Cargo.toml --all                                                               | Passed, including final scoped source formatting.                                                                                                                                                                                                                                                                                  |
| bun test ./scripts/check-phase164-canonical-identity.test.ts                                                                                   | Full405 passed, zero failed,491 expectations,98.22s. After the final clippy predicate spelling/hash renewal,22 focused current/changed-edge/mutation controls passed with32 expectations. After final retained-byte guards,29 focused controls passed with41 expectations,8.18s; the four added cases were included in this rerun. |
| bun run scripts/check-phase164-canonical-identity.ts                                                                                           | Final101 fixed Rust inputs,278 reviewed source edges; passed. The original moved context edge is represented by its shared pure owner and helpers; no historical receipt or snapshot was changed.                                                                                                                                  |
| bun scripts/check-parity-breadcrumbs.ts --check                                                                                                | Passed:1,171 Rust files, including all five new source/test files.                                                                                                                                                                                                                                                                 |
| bun scripts/bright-builds-check.ts all                                                                                                         | Passed:1,521 scanned files, zero exceptions/findings. New source/test files also remain physically below the628-line cap.                                                                                                                                                                                                          |
| Owned diff, threat/stub review and git diff --check                                                                                            | Passed; no new dependency, endpoint, file-access trust boundary, schema change or goal-blocking stub. Git index remains empty.                                                                                                                                                                                                     |

Whole-workspace native/Bazel acceptance remains Plan14 and the parent's responsibility. Network-only coverage does not qualify the whole workspace.

## Decisions and Simplification

AGENTS.md, the Bright Builds sidecar, placeholder-only overrides, architecture/code-shape/verification/testing/Rust/TypeScript standards and both complete active lesson files (7,188 bytes;2,397 conservative estimated tokens) informed execution. The parent owned sync/tool bootstrap. No project skill directory was present.

The simplification pass moved the existing context/retarget helpers into the existing pure header owner, removed the now-redundant separate powLimit binding and kept peer delegates in one small focused child module. There is one context formula, no extra arithmetic implementation, no mutable HeaderStore escape and no arbitrary sequence setter or deserialization path.

The source checker was renewed only for reviewed current changes: five additional inputs,22 added edges and replacement of the relocated edge produce278 total. New mutation controls cover ignored body validation, fabricated parent proof, selected-parameter bypass, FIFO reversal, exhaustion bypass, hash tie-breaking and lost prune revocation. A newly proposed guard for the existing cfg_attr test helper was omitted because the conservative ownership parser rejects that attribute; the actual production prune owner is guarded and the helper is behaviorally exercised. No prior guard was dropped.

## Deviations from Plan

1. **[Rule3 - Blocking; parent approved] Actual owner and query inventory.** The plan was expanded before edits to include crate exports, peer delegates, local/stored/reorg ingress, handle queries, the existing context builder and prune-removal hooks. These are the actual ownership edges needed to make the registry concrete. Five new Rust files all have registered breadcrumbs.
1. **[Rule2 - Correctness; parent approved] Public checked-byte boundary.** A cross-crate label or caller-supplied validation context could fabricate prerequisites. Full selected parameters now bind the store and context is derived internally from accepted ancestry before checking actual bytes.
1. **[Rule1 - Bug; parent approved] Preserve raw receipt and clock meanings.** Eager linkage initially rejected existing raw unknown-parent disconnected/nonextending receipts and interpreted a mempool event clock as block validation time. Unproved cache classifications now earn no sequence, and existing modeled local/reorg paths use their block clock behind checked selected ancestry. Actual P2P admission continues to validate the supplied ingress time.
1. **[Rule3 - Blocking; parent approved] Current guard maintenance.** Reviewed current source inputs/edges and failing mutation controls were updated for the new pure owner and changed ingress; frozen historical tooling, manifests, receipts and support labels were preserved.
1. **[Rule2 - Correctness; parent approved] Actual node retention guard.** Final self-review found that a direct peer checked-byte call can create linkage without populating the node cache. Owner queries now check the actual retained nonactive suffix, with an absent-bytes negative and fresh managed-readmission positive. Two further reviewed edges and mutation controls protect this guard. Final node119/119, focused guard29/29 and all-target clippy exit0 in7.15s cover the refinement.

## Deferred Issues and Handoff

- **Parent-owned pre-existing diagnostic:** bash scripts/check-panic-sites.sh returned exit1 for test-only expects in earlier plans' consensus/src/crypto/phase165_tests.rs and network/src/header_store/phase165_tests.rs. The scanner recognizes tests.rs or /tests/ paths and does not recognize those filenames. No Plan06 production panic-like site was reported. The parent will repair test placement/ownership before final native verification; this executor made no out-of-scope scanner, test or allowlist edits.
- **Plan08:** Exact failed-prefix attribution for a direct reorg remains with its activation owner. The existing generic prepare_reorg error does not identify the failing replacement block; this plan does not fabricate failure of every replacement. Work/owner/stale/storage refusals must remain distinct from consensus invalidity. Actual work/lineage/preference authorization before preview/commit also remains Plans07/08.
- **Plan09:** A raw cached body received before accepted headers must pass an explicit fresh checked-body readmission after those headers arrive. Raw cache content never auto-certifies itself. Candidate reconciliation must consume all eligible retained branches rather than only best-header ancestry.
- Requested pruned-body redownload sequence reassignment, preciousblock negative sequence and ambiguous internal/restart tie qualification remain bounded/deferred. Present-body duplicate idempotence does not claim those surfaces.
- Phase166 owns canonical bootstrap/checkpoint/all-handle reopen/genesis BASIC, Phase167 contextual completeness and Phase168 actual pinned-daemon qualification. Ordinary canonical support, the four selector refusal classes and networking defaults remain unchanged. Parent Plan13 owns consolidated contributor-facing status updates.

## Self-Check: PASSED

All17 approved paths and all five new mapped Rust files exist. The owned source/current-guard diff was reviewed; focused behavior, full network regression, network-only handwritten coverage, affected all-target clippy, formatting, managed caps, breadcrumbs and diff checks have actual evidence above. No executor command session remains. Commit existence and shared state/tracker updates are intentionally deferred to the authorized parent wrapper. Ready for165-07.
