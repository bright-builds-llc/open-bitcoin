---
phase: 166-canonical-durable-bootstrap-and-genesis-basic
plan: "02"
subsystem: chainstate
tags: [genesis, coins, header-root, earned-work, complete-snapshot]
requires:
  - phase: 166-01
    provides: Sealed selected complete-genesis proof and independent frozen work vectors
  - phase: 165-shared-exact-work-authority
    provides: Owner/generation-bound work lineage and checked header admission
provides:
  - Complete empty-genesis coins constructor with internally earned work lineage
  - Matching canonical header root through existing checked admission
  - Shared selected genesis parameter binding with unchanged import semantics
affects: [166-05, 166-06, 166-07, 166-08, 166-09, 166-10]
tech-stack:
  added: []
  patterns: [complete effective snapshot before authority, checked root admission, shared genesis parameter mapping]
key-files:
  created:
    - packages/open-bitcoin-chainstate/src/engine/canonical_genesis.rs
    - packages/open-bitcoin-chainstate/src/engine/canonical_genesis/tests.rs
    - packages/open-bitcoin-network/src/header_store/canonical_genesis.rs
    - packages/open-bitcoin-network/src/header_store/canonical_genesis/tests.rs
  modified:
    - packages/open-bitcoin-chainstate/src/engine.rs
    - packages/open-bitcoin-network/src/header_store.rs
    - docs/parity/source-breadcrumbs.json
key-decisions:
  - Share the existing selected-network parameter mapping through the owned chainstate genesis module; it binds roots without qualifying ordinary history.
  - Reuse checked header admission directly instead of importing entries and changing acceptance flags.
patterns-established:
  - Complete snapshot B must equal the sealed genesis hash, with no coin rows or interrupted H markers, before minting a live root owner.
  - Header proof arithmetic and connected work agree with independent frozen vectors while body/history authority remains separate.
requirements-completed: [CBOT-02, CWOR-04]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 166-2026-10-18T22-36-40Z
generated_at: "2026-10-18T23:13:26Z"
commits: false
duration: 5 min recorded implementation window, initial reading excluded
completed: 2026-10-18
---

# Phase 166 Plan 02: Earned Canonical Genesis Owners Summary

**Complete empty genesis coins now earn one parameter-bound live position, and checked header admission creates its matching exact-work root without promoting imported numbers.**

## Performance

- Recorded implementation window: 2026-10-18T23:08:45Z through 2026-10-18T23:13:26Z; initial instruction and source reading is excluded.
- Tasks: 2/2. Owned implementation files: 7, plus this summary.
- No staging, commits, push, branch/worktree creation or hook bypass. Root retains shared STATE, ROADMAP, REQUIREMENTS, task progress and final Git saving.

## Accomplishments and API

`Chainstate::from_canonical_genesis(&CanonicalGenesisProof, CoinsCache<V>) -> Result<Self, ChainstateError>` reads the complete effective snapshot before granting authority. It requires B equal to the proved genesis hash, no coins and no H markers. Unsupported completeness propagates `CompleteCoinsSnapshotUnavailable`; inconsistent complete state returns `WorkRebindMismatch`. It derives the sole height-zero position, header, hash, median time and exact work from the sealed proof, creates fresh private owner/generation identity and binds selected parameters through the existing work-authority mechanism.

Genesis transaction connection is skipped entirely: no output application, undo or confirmed-transaction entry is synthesized. Tests query and attempt to spend the actual genesis coinbase outpoint for all four selectors and observe absence. Both parent and overlay coins, arbitrary outpoints, wrong/missing/foreign B and interrupted H markers refuse. Public coins mutation still invalidates earned lineage, and raw snapshot/cache imports retain their untrusted origin.

`engine::canonical_genesis::consensus_params(&CanonicalGenesisProof) -> ConsensusParams` reproduces the current `SyncNetwork::consensus_params` mapping exactly, including minimum-difficulty/no-retarget switches and all remaining fields. It is a genesis parameter-binding helper, with no caller-selected parameters or history-qualified result. The parent explicitly accepted its placement in the owned public module because network already depends on chainstate and the sealed consensus proof currently has no parameter accessor.

`HeaderStore::from_canonical_genesis(&CanonicalGenesisProof) -> Result<Self, NetworkError>` reuses `insert_header_with_params` on an empty store. Existing hash, target, arithmetic, parent and parameter checks earn the root; no `from_entries` flag promotion is used. Four frozen Phase 166 fixture rows supply independent 256-bit work expectations, and the same test compares the connected owner. Imports retain no accepted proof facts, and competing roots or wrong/unbound parameters preserve the entire store. Body eligibility is absent; ordinary-history and Signet challenge prerequisites still refuse.

## Automated Evidence

All commands used the pinned Bun 1.3.9 PATH. Cargo ran serially through `scripts/command-timings.ts`. Actual logs are retained under ignored `.local/open-bitcoin-dev/phase166/plan02/`.

| Check | Actual result | Log |
| --- | --- | --- |
| Task 1 RED, planned chainstate `phase166` filter | Exit 101; seven discovered, all seven failed against intentionally refusing constructor | `task1-red.log` |
| Task 1 GREEN | Exit 0; nine passed, zero failed after adding owner/parameter and mutation controls | `task1-green.log` |
| Task 1 final after rustfmt | Exit 0; nine passed, zero failed | `task1-final.log` |
| Task 2 RED, planned network `phase166` filter | Exit 101; six discovered, two passed/four failed against intentionally refusing constructor | `task2-red.log` |
| Task 2 GREEN | Exit 0; six passed, zero failed | `task2-green.log` |
| Task 2 final after rustfmt and simplifying foreign-proof test | Exit 0; six passed, zero failed | `task2-final.log` |
| Existing Phase 165 focused owner/header regressions | Exit 0; 41 chainstate and 43 network tests passed | `phase165-regressions.log` |
| Scoped Clippy, both affected crates, all targets/features, warnings denied | Exit 0 | `clippy.log` |
| Supported breadcrumb `--check` | Exit 0; 1210 Rust files verified | `breadcrumbs.log` |

The focused new checks total 15 distinct passing Rust tests; existing regressions add 84 distinct tests. Integration harnesses with zero matching tests are excluded from these counts. Scoped `rustfmt --check` on the six new/touched Rust files and `git diff --check` pass. New breadcrumb mappings cover all four new Rust files while preserving Plan 01's mappings. No dependency, crate, arithmetic implementation, historical receipt or source-contract digest was changed.

## Task Commits

Deferred by the checked strict-wrapper contract until complete phase acceptance; `commits: false` is intentional. No commit or hook outcome is claimed.

## Decisions Made

The shared helper avoids two new mappings across the coins/header constructors and keeps their binding identical to the existing node selector. Later node integration must compare its selected parameters to this earned root and preserve its existing ordinary-history gate. The simplification pass retained the existing checked header operation and the existing private work-authority binding mechanism; no configurable constructor, boolean authority switch or new acceptance token was introduced.

Guidance applied: repo AGENTS/native-verifier/tool-pin/parity rules; Bright Builds sidecar; standards overrides with no concrete exception; architecture, code-shape, testing, verification, local-guidance and Rust standards; and both complete canonical active lesson files (7,188 bytes / 2,397 conservative estimated tokens). Neither project skill directory exists. Root owns the active lifecycle and final aggregate verification.

## Deviations from Plan

**1. [Rule 3 - Blocking] Locate the shared selected-parameter binding in an owned module.** The required constructors need identical selected parameters, but `CanonicalGenesisProof` has no parameter accessor and the existing node mapping is private. The parent accepted the small public helper in `engine::canonical_genesis` within declared ownership. No dependency or selector semantics changed. All fields are checked against the current mapping, and owner/header tests pass. No commit was made under the strict wrapper.

## Issues Encountered

The initial RED invocation waited on the Cargo artifact-directory lock held by Cursor rust-analyzer's automatic workspace check. Parent inspection observed active rustc processes; the editor job was preserved and the timed test proceeded normally. Deliberate RED failures are retained above and resolved by the implementations. No unresolved implementation issue remains.

## Next Plan Readiness and Limits

Ready for Plan 166-03. These APIs establish pure genesis coins/header ownership; durable storage publication, safe inspection/reopen, runtime binding and BASIC integration remain later plans. Existing core numerical descendant APIs are unchanged, and product descendant acceptance remains the node admission gate's responsibility. No public-network activation, actual-daemon differential or durable-reopen claim is made. Full default native verification, LOC freshness, shared lifecycle updates and final normal-hook Git saving remain with Plan 166-14/root.

## Self-Check: PASSED

All seven owned implementation/evidence files and this summary exist. Both task commands, focused predecessor regressions, scoped Clippy, formatting, breadcrumbs and diff whitespace checks pass. The owned source scan contains no placeholders or unfinished implementation. No additional unmodeled security surface was introduced.
