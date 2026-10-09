---
phase: 158-validated-reorg-and-retained-branch-identity
reviewed: 2026-10-09T03:55:25Z
generated_by: gsd-code-reviewer
phase_lifecycle_id: 158-2026-10-08T02-17-15
depth: standard
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
  warning: 0
  info: 0
  total: 0
status: clean
resolved_findings: [WR-158-02]
---

# Phase 158: Independent Automation Fix Check

WR-158-02 is resolved. Independent delta review found no new issue. This closes the sole finding in [the original automation review](158-REVIEW-AUTOMATION.md); it does not replace the remaining native, security or formal lifecycle gates.

## Exact correction

The checker at `scripts/check-phase158-validated-reorg.ts:123` now masks only the exact `pub(super) identity:` field inside `BasicFilterAppendProof` before rejecting all remaining `pub` tokens. Other sealed types receive no visibility exception. The existing required identity seam remains usable; publication, work and budget fields stay private. No predecessor, proof, scope, claim or verifier contract was removed or weakened.

The test addition at `scripts/check-phase158-validated-reorg.test.ts:119` supplies 35 independent controls across four sealed stage/receipt/position types and the three otherwise-private proof fields, using `pub`, `pub(super)`, `pub(crate)`, `pub(self)` and `pub(in crate)`. Five additional controls reject broader or alternate identity visibility. Existing tests remain unchanged. Reversing only this checker correction and removing only the new test block reproduces both original review SHA-256 hashes exactly, proving there are no additional edits inside either file.

## Independent verification

Using pinned Bun 1.3.9 and mocked in-memory file reads, the unchanged current roots returned `[]` before and after mutation controls. All 40 independent restricted-visibility mutations returned the expected private-field finding. In particular, each original false negative now rejects independently:

| Mutation | Actual checker result |
| --- | --- |
| StagedChainstateReorg.overlay → pub(super) | private StagedChainstateReorg fields required |
| AcceptedChainstateReorg.maybe_old_endpoint → pub(super) | private AcceptedChainstateReorg fields required |
| ValidatedBasicFilterReorg.facts → pub(super) | private ValidatedBasicFilterReorg fields required |
| BasicFilterAppendProof.publication → pub(super) | private BasicFilterAppendProof fields required |

The actual `BasicFilterAppendProof.identity` remains `pub(super)` and the positive roots pass; removing that required seam was not used to obtain green. These are supplementary lexical guard checks, not Rust behavior or durability proof.

The parent reported the executor's full checker suite green at 449 passed, zero failed, 849 assertions in 34.48 seconds. This reviewer did not rerun that entire suite. No Cargo/Bazel, source, root-state or Git mutation was performed.

## All twelve original fingerprints compared

Compared every original fingerprint from the 2026-10-09T03:52:53Z review. Exactly the two Phase 158 checker files changed; all ten other paths remained byte-identical. Final hashes were rechecked after the independent mutations at 2026-10-09T03:55:25Z and were stable.

| Path | Original SHA-256 | Comparison |
| --- | --- | --- |
| README.md | ff3f31bd9efdeda9b357ede8cc4e74343251a0b617916fc07c188b3112811e9b | unchanged |
| packages/README.md | 25d116698abb36b53e5821abffc133bdfb191fe0bb561f24ba1f499fcb2623fe | unchanged |
| docs/parity/index.json | 6255e3ced72d031bf4a6d93994ba6a56c5eada0fe6bead87b87750621f99553d | unchanged |
| docs/parity/source-breadcrumbs.json | 02c2b000bb46934ed4261185f4ff86551d16d4e634f1079df46bdf077eefb67d | unchanged |
| docs/parity/v2-5-validated-reorg.md | 166b597562ea22020690e0cd8270dfab1edee6510277ca60e77fb0be3a688bc7 | unchanged |
| scripts/check-phase158-validated-reorg.ts | a4dae159ae457b6cc208417a05563564ba632727749ca2526afc1bf6ee72025b | exact reviewed correction only |
| scripts/check-phase158-validated-reorg.test.ts | 6b0d447c9d8facd496c88843442fce6def6898ca7fa4e13d04570adf1d8c0fca | exact 40-control insertion only |
| scripts/check-phase157-index-catch-up.ts | 0b5890ac87f4d27e0fbeb5f0e28c36119045ade3db92853ca5deacaaca9d7ad0 | unchanged |
| scripts/check-phase157-index-catch-up.test.ts | 53d897f7720f9a2989399a641956058f3937fa1507f39e02e8e605c182d60e6c | unchanged |
| scripts/check-phase157-index-catch-up/contracts.ts | eeaa568a615924dacdc253da4551203e7c31ebe25f40b0ec5d63936f102256f1 | unchanged |
| scripts/verify.sh | d9c6fbff98535c283c3adc01726ea18f58e159c541e01bc27a238414520af3fc | unchanged |
| .planning/phases/158-validated-reorg-and-retained-branch-identity/158-UAT.md | 8446c109998d9afd5c032fd4470bb51c3254c2d9ada397e499327a7b4538a938 | unchanged |

| Changed path | Final bytes | Final SHA-256 |
| --- | --- | --- |
| scripts/check-phase158-validated-reorg.ts | 27,191 | e9b8ae3ddc7a6ccaffa67a0b5bf7b7614eae7938d3cc1829de4cd2ca9aaa64e9 |
| scripts/check-phase158-validated-reorg.test.ts | 13,572 | 05d98dcd2abf30dad2df49a2f5ab058fbf60d070d76ea38e88b6c65f033ef109 |

_Reviewer: gsd-code-reviewer; standard delta review; no commit._
