---
phase: 158-validated-reorg-and-retained-branch-identity
reviewed: 2026-10-09T03:52:53Z
generated_by: gsd-code-reviewer
phase_lifecycle_id: 158-2026-10-08T02-17-15
depth: standard
diff_base: 2f21ac2052208c7e7a084ed006d908c5ccb714aa
files_reviewed: 12
files_reviewed_list:
  - README.md
  - packages/README.md
  - docs/parity/index.json
  - docs/parity/source-breadcrumbs.json
  - docs/parity/v2-5-validated-reorg.md
  - scripts/check-phase158-validated-reorg.ts
  - scripts/check-phase158-validated-reorg.test.ts
  - scripts/check-phase157-index-catch-up.ts
  - scripts/check-phase157-index-catch-up.test.ts
  - scripts/check-phase157-index-catch-up/contracts.ts
  - scripts/verify.sh
  - .planning/phases/158-validated-reorg-and-retained-branch-identity/158-UAT.md
findings:
  critical: 0
  warning: 1
  info: 0
  total: 1
status: issues_found
---

# Phase 158: Automation and Documentation Code Review

**Reviewed:** 2026-10-09T03:52:53Z
**Depth:** standard
**Files Reviewed:** 12
**Status:** issues_found

## Summary

Reviewed the exact twelve current paths, including untracked Phase 158 artifacts, against the baseline and Phase 158 Context/Plan 07. Material guidance was AGENTS.md, AGENTS.bright-builds.md, placeholder-only standards-overrides.md, and architecture, code-shape, testing, verification and TypeScript/JavaScript standards. Both active lesson files were loaded completely: 7,188 bytes and 2,397 conservative estimated tokens. No project skill directories or .claudeignore were present; git check-ignore excluded none of the twelve paths.

The documentation scopes genuine internal reorg, recovery and retained-hash evidence, preserves explicit finite-resource and maturity/halving differences, and keeps CFIX-03 pending until native, independent source/security and formal lifecycle gates. It does not introduce public serving, a new reorg command or production/funds claims. UAT provides explicit timed Cargo/Bazel forms. The only ignored Phase 158 reorg test is the separately documented timing experiment; required behavioral tests are ordinary registered tests.

The Phase 157 migration follows current proof acquisition, append callee and consumed budget locations while retaining predecessor verification, same-store/generation/frontier checks and forbidden full-scan contracts. Full JSON parsing and semantic comparison found only the scoped Phase 157 evidence additions, new unique CFIX-03 owner and breadcrumb additions/group migration; the breadcrumb registry has 987 unique file mappings and all mapped paths exist. Phase 158 uses bounded current roots and prior plan summaries, with no requirement for future Phase 158 completion/security reports. Both documented and executed verifier paths run the Phase 158 test/check pair immediately after Phase 157.

## Warnings

### WR-158-02: The private-field check exempts every `pub(super)` field

**File:** `/Users/peterryszkiewicz/Repos/open-bitcoin/scripts/check-phase158-validated-reorg.ts:123`

**Related test:** `/Users/peterryszkiewicz/Repos/open-bitcoin/scripts/check-phase158-validated-reorg.test.ts:106`

**Issue:** The negative lookahead permits `pub(super)` on every field of every type in SEALED. Only `BasicFilterAppendProof.identity` currently needs that visibility. The other sealed stage/receipt/accepted-position capabilities must retain private fields. Broadening a stage or receipt field allows its parent module to alter genuine validated facts or authority while the default guard still reports success. Existing mutations test only plain `pub`, so they miss the exception spreading to other fields/types. This is a checker regression gap, not a claim that current Rust fields are already exposed.

**Evidence:** Using pinned Bun 1.3.9 with an in-memory mocked readFileSync, the unmodified checker returned `[]`. Each following isolated mutation also returned `[]`, without modifying repository sources:

| Type | Isolated mutation accepted by the checker |
| --- | --- |
| StagedChainstateReorg | `overlay: CoinsOverlay` → `pub(super) overlay: CoinsOverlay` |
| AcceptedChainstateReorg | `maybe_old_endpoint` → `pub(super) maybe_old_endpoint` |
| ValidatedBasicFilterReorg | `facts: PendingBasicFilterReorg` → `pub(super) facts: PendingBasicFilterReorg` |
| BasicFilterAppendProof | `publication` → `pub(super) publication` |

**Fix:** Reject every `pub` visibility in every sealed struct, with one explicit exception tied to the exact `BasicFilterAppendProof.identity` field using `pub(super)`. Keep all other append-proof fields private. Add independent mutations for restricted visibility on each sealed type and each otherwise-private proof field; include `pub(crate)` and `pub(in ...)` controls. Preserve the Phase 157 check's existing exact-field exception and negative controls.

## Verification and limits

- Both live Phase 157 and Phase 158 checker executions passed against the reviewed roots. Bash syntax validation of scripts/verify.sh passed.
- The independent in-memory mutations above established the false negative despite a clean live result.
- Independent targeted run: `bun test ./scripts/check-phase157-index-catch-up.test.ts` passed 600/600 tests, 1,668 assertions, zero failures, in 57.16 seconds using pinned Bun 1.3.9. A redundant Phase 158 suite was omitted at the parent's request because the executor had already run it and native verification will rerun it.
- No Cargo/Bazel commands, source modifications, root-state edits or Git mutations were performed. The parent retains ownership of the separate Rust fix, default native verification, source/security consolidation and lifecycle completion. This report does not activate CFIX-03.

## Reviewed SHA-256 snapshot

Captured at 2026-10-09T03:52:53Z. Any subsequent change requires a delta review.

| Path | SHA-256 |
| --- | --- |
| README.md | ff3f31bd9efdeda9b357ede8cc4e74343251a0b617916fc07c188b3112811e9b |
| packages/README.md | 25d116698abb36b53e5821abffc133bdfb191fe0bb561f24ba1f499fcb2623fe |
| docs/parity/index.json | 6255e3ced72d031bf4a6d93994ba6a56c5eada0fe6bead87b87750621f99553d |
| docs/parity/source-breadcrumbs.json | 02c2b000bb46934ed4261185f4ff86551d16d4e634f1079df46bdf077eefb67d |
| docs/parity/v2-5-validated-reorg.md | 166b597562ea22020690e0cd8270dfab1edee6510277ca60e77fb0be3a688bc7 |
| scripts/check-phase158-validated-reorg.ts | a4dae159ae457b6cc208417a05563564ba632727749ca2526afc1bf6ee72025b |
| scripts/check-phase158-validated-reorg.test.ts | 6b0d447c9d8facd496c88843442fce6def6898ca7fa4e13d04570adf1d8c0fca |
| scripts/check-phase157-index-catch-up.ts | 0b5890ac87f4d27e0fbeb5f0e28c36119045ade3db92853ca5deacaaca9d7ad0 |
| scripts/check-phase157-index-catch-up.test.ts | 53d897f7720f9a2989399a641956058f3937fa1507f39e02e8e605c182d60e6c |
| scripts/check-phase157-index-catch-up/contracts.ts | eeaa568a615924dacdc253da4551203e7c31ebe25f40b0ec5d63936f102256f1 |
| scripts/verify.sh | d9c6fbff98535c283c3adc01726ea18f58e159c541e01bc27a238414520af3fc |
| .planning/phases/158-validated-reorg-and-retained-branch-identity/158-UAT.md | 8446c109998d9afd5c032fd4470bb51c3254c2d9ada397e499327a7b4538a938 |

_Reviewer: gsd-code-reviewer; standard depth; no commit._
