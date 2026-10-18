---
phase: 165-shared-exact-work-authority
reviewed: "2026-10-17T12:20:15Z"
generated_by: gsd-code-reviewer
phase_lifecycle_id: 165-2026-10-14T16-09-59Z
lifecycle_mode: yolo
depth: deep
scope: preliminary-current-tooling-integrity
diff_base: d0104a997c0362bf65b81de1c8fec8ddc3012a1b
files_reviewed: 18
files_reviewed_list:
  - scripts/check-phase158-validated-reorg.test.ts
  - scripts/check-phase158-validated-reorg.ts
  - scripts/check-phase162-integrated-filter-proof.test.ts
  - scripts/check-phase162-integrated-filter-proof.ts
  - scripts/check-phase164-canonical-identity.test.ts
  - scripts/check-phase164-canonical-identity.ts
  - scripts/check-phase164-canonical-identity/contracts.ts
  - scripts/check-phase164-canonical-identity/fixture-source.ts
  - scripts/check-phase164-canonical-identity/historical-tooling.ts
  - scripts/check-phase164-canonical-identity/source-files.ts
  - scripts/check-phase164-canonical-identity/test-modules.ts
  - scripts/check-phase165-shared-exact-work.test.ts
  - scripts/check-phase165-shared-exact-work.ts
  - scripts/check-phase165-shared-exact-work/contracts.ts
  - scripts/check-phase165-shared-exact-work/test-support.ts
  - scripts/check-phase68-active-chain-persistence.test.ts
  - scripts/check-phase68-active-chain-persistence.ts
  - scripts/verify.sh
findings:
  critical: 0
  warning: 0
  info: 0
  total: 0
status: clean
---

# Phase 165: Preliminary Tooling Integrity Review

**Verdict: CLEAN for the exact 18-file tooling snapshot below.** No actionable bug, security issue, or proof-integrity regression was found in this static cross-integration review. This report does not constitute final whole-phase review or runtime qualification.

## Scope and Method

Reviewed all changed and new files under `scripts/` against `d0104a997c0362bf65b81de1c8fec8ddc3012a1b`, including the 158/162 compatibility changes, the 164 fixture and historical-tooling adapters, the extracted inventories, the new 165 checker/modules/tests, the 68 work projection guard, and native verification registration. Git scope discovery included untracked nonignored sources. No reviewed path was ignored, and no project skill directory was present.

Repo-local `AGENTS.md`, `AGENTS.bright-builds.md`, placeholder-only `standards-overrides.md`, `standards/index.md`, verification, code-shape, testing, and TypeScript/JavaScript standards informed this review. Both active lesson files were fully loaded: 7,188 bytes and 2,397 conservative estimated tokens; archives were excluded. Supporting unchanged region, ownership, claim, provenance, and historical evidence sources were followed where needed.

Analysis was read-only except this report. It used source/diff inspection and independent lightweight Python byte, JSON, and Git-object calculations. **No Bun execution, project module import, test, Cargo, Bazel, build, hook, staging, commit, push, or external message was performed by this reviewer.** Existing execution receipts were read only for context; this report claims no independently rerun matrix.

The source snapshot was captured at `2026-10-17T12:18:18.233733+00:00` and independently rehashed at `2026-10-17T12:20:15.741084+00:00`; all 18 files were unchanged between those observations. Plan 28 is now executing, so later source changes require affected-delta review against these hashes. The authentic parent-owned before-28 capture is separate evidence, not a runtime receipt.

## Integrity Conclusions

### Fixed loading and current source policy

The current 165 input list is built from code constants, then authenticated by its ordered inventory hash before the first source read. Independent static reconstruction produces **134 SOURCE_CONTRACTS, 70 TEST_OWNERS, and 263 INPUT_FILES**, with ordered JSON SHA-256 `53b26c6f2cd457c1f2df85b22095f192de09b76ca2256c9a3eff9f89d3fd208a`. Source and test-owner paths are unique, and each test-owner path has a current source contract.

Loaders use fixed paths, regular-file checks, no-follow descriptor opens, descriptor revalidation, bounded reads, and visible errors. Neither manifest JSON nor a supplied map can redirect filesystem reads or rescue a missing source from the checkout. Manifest data is inert during module import; the new import controls exercise the isolated 14-module CLI layout.

Current source identities retain literal-bearing source after leading masked commentary. This is an intentionally conservative source guard: renewing a hash still requires independent source review and actual Rust verification. It is not semantic runtime execution. Enabled-test controls separately inspect immediate registration, supported Tokio options, ignore attributes, and enclosing/file configuration. The tests mutate named declaration offsets without overwriting earlier tests or async modifiers, and require owning-file/rule diagnostics. Registration controls cover deletion, comments, cfg disabling and path redirection. No blanket test skip, broad source-directory exemption, or dummy-call acceptance was added.

### Current 158 semantics remain raw

The ordinary 158 checker directly invokes `checkCurrentReorgBoundaries` with its raw current reader. Historical claim translation is isolated from this semantic path; production Rust is not projected to historical source.

The current reorg contracts follow public validation through owner/generation/tip checking, consuming absorption, and the private effect callee. Complete Boolean clauses supplement the prior operand anchors. Selected generic impl configuration is checked before method selection. Exact sealed field inventories and private capability fields, refusal-only unqualified preparation, authorization before history/BASIC effects, checkpoint propagation, and current module/test ownership remain checked. The previous review's selected-impl and Boolean-clause findings are resolved in the inspected source.

The historical CFIX evidence iteration is explicitly fixed at its original 35 entries, including repeated paths. New current owners do not receive historical qualification merely by joining the current read set.

### Historical fixture and tooling attribution is exact and distinct

The eight-source legacy bundle is authenticated by complete byte length and SHA before JSON parsing, then by fixed metadata, ordered path/source rows, source lengths/hashes, and exact admitted current versions. It is restricted to prospective legacy structural contracts. Its eighth source matches the authenticated pre-phase Git object exactly at SHA `831e8741d8e4f7d1a746a85a5f359b9b18e00ff96a069d832a4a6517715185cb`; the original seven-source capture is not relabeled as eight. The relocated header child has its own required current identity. Unknown source versions throw rather than silently supplying another raw baseline.

The daemon claim adapter independently reverses the actual reviewed include/seed/import/context substitutions from raw `2d0a7e7d93034333e00f5c90977693bcfb7a03ff8ab704bf834d91fec680d307` to original `faa009380301d6e6acbf032fd814f5cf9e31ddcbdb629787477fe5fbc5d62615`. Its legacy structural fixture identity `933da5cf...` is separate. Partial, duplicate, reordered, altered or unrelated bytes cannot acquire the original claim identity because the complete reconstructed destination must match. Failed daemon attribution returns the complete raw source as unproved.

Independent decoding and literal application of the current and legacy tooling inverse data reconstructs the exact baseline Git bytes for all four changed 158/162 checker/test files:

| File                                                     | Reconstructed complete baseline SHA-256                            |
| -------------------------------------------------------- | ------------------------------------------------------------------ |
| `scripts/check-phase158-validated-reorg.ts`              | `385b6c6d62067ba13e8e4d29d0ea1005252614830430f800b0824ec7ba954fdd` |
| `scripts/check-phase158-validated-reorg.test.ts`         | `bebdd0c7f2cc6e787f34502bb82f6ae6fd27c7a5ce5ac9bdd14be73c054193a3` |
| `scripts/check-phase162-integrated-filter-proof.ts`      | `caf9147ecd5b4ac189486bca0d2e81298ff0cfc0ffc218769a5f690a20ca8350` |
| `scripts/check-phase162-integrated-filter-proof.test.ts` | `784e29775202503f892ee7b005a5595f77ac8d0f8bdb42ca19818c138a86f6d5` |

Each contextual current fragment occurs exactly once and each intermediate/final identity is required. Outer replacements use literal callbacks; the inner legacy replacement destinations contain no JavaScript replacement metasequences. Refusal returns the original raw source. The adapter is not an arbitrary hash allowlist or general stripping function.

All 14 current 165 historical-identity rows match their complete raw bytes. The manifest remains `9783fa03fdd2df1395445b7752e6c31d84985c351ef44de516656af7f82fc7c0`; the current mixed bundle remains `1330515f81cb39108b7605814f50ed4c1f1572a0280b8415902085791f9eab5c`. Git shows no changes under the existing phase162/163/164 testdata directories. Historical 162/163 report identities are enforced independently rather than renewed to bless current execution.

### Cross-integration and native registration

Only the 162 legacy function-contract loop consumes the authenticated structural view. Its raw module ownership, physical 31-byte constant, oracle, claim, frozen-tooling and native predicates still execute independently. The synthetic legacy mutation API has no CLI bypass, claim publication, or receipt authority. Targeted mutation tests require their actual structural or raw-native diagnostic rather than an unrelated version error.

The 164 checker retains exact current function-edge hashing and uninventoried sensitive constructor detection. Its current inventories contain 135 source paths and 477 unique function keys. Against the pre-phase baseline, no original source path was removed. The sole old edge key that moved is `transient_runtime`, from runtime_authority to canonical_admission with the same digest; this is an explicit relocation. New entries are concrete files and function owners. The extracted source/test-module data remains code-owned, with existing ownership requirements retained.

Native registration adds exactly six lines: declared/executed 68 test and 165 test/check pairs. Independent removal of those six lines yields every baseline verifier byte exactly, SHA `2925344e06e3604fbba1821e49a00b0f2f292448eae205107882d0ed65aacc43`. Registration placement and exact-once enforcement remain checked by the old and new guards. The 162 complete historical command comparison alone uses its narrow projection; raw mandatory Cargo/Bazel/benchmark/coverage commands and Phase161/162 exact ordering remain independently checked. Removing all six current additions still fails the already-mandatory 164 guard.

The full default verifier retains formatting, Clippy, all-target/all-feature build, workspace and CLI tests, benchmark smoke/report, Bazel/provenance and pure-core coverage. This inspection does not claim those commands have run on final phase bytes.

### Claim boundaries and limits

The 165 checker and its tests describe source-derived arithmetic and local refusal evidence. Current claim controls require the documented limits and reject tested affirmative canonical/bootstrap/reopen, restart-order, future-phase and actual pinned-daemon qualification claims, including independently scoped JSON strings. Static lexical policies supplement human review and do not constitute a general natural-language proof system.

The public fixture API proof in Plan 28 is pending at this snapshot. It cannot be inferred from hashes, the earlier fixture tests, or this review. Ordinary canonical network bootstrap/reopen, deterministic restart tie parity, and actual pinned-Knots daemon differential qualification remain outside this preliminary tooling verdict. Current phase requirements are CWOR-01/02/03/05; CWOR-04 belongs to Phase 166.

## Source Identity Inventory and Reuse Rule

These are full-file raw SHA-256 values, not sourceIdentity values. Reuse this review only while the corresponding complete file hash matches. Review every later affected delta, including current 165 entry/contracts/main changes from Plan 28 and potential compatibility changes from Plan 24; refresh final whole-phase/native/security/formal gates independently.

| File                                                              | Bytes  | Lines | Complete raw SHA-256                                               |
| ----------------------------------------------------------------- | ------ | ----- | ------------------------------------------------------------------ |
| `scripts/check-phase158-validated-reorg.test.ts`                  | 39706  | 628   | `418479c36f497919cf2ca289d93dcefaa8b009920dae467181cca528365c7f81` |
| `scripts/check-phase158-validated-reorg.ts`                       | 54228  | 491   | `1b87c3bddf0d65db9eea63039780d5b6b03bd3a6db9a99d56fd61637d0391c6c` |
| `scripts/check-phase162-integrated-filter-proof.test.ts`          | 41091  | 623   | `2932b0ee2de0ac1d600d454590fcbe2e9281f748bcb13c50048c7480da63dfcf` |
| `scripts/check-phase162-integrated-filter-proof.ts`               | 27883  | 434   | `5ffd8e4ea3d54197cd88923cbae2375f977fefe755e2258bd24c3450ec4f7c44` |
| `scripts/check-phase164-canonical-identity.test.ts`               | 49940  | 598   | `a13cb68af20f25669d6bdffb4657cf6c426272872c6596109a121314f577abb3` |
| `scripts/check-phase164-canonical-identity.ts`                    | 15668  | 245   | `da4c51f4a9f67b1386794d4b7eb2da03acd0c0f99f272d7699e8ec4333541f00` |
| `scripts/check-phase164-canonical-identity/contracts.ts`          | 96846  | 515   | `f1e694c6463af0b30b904c3a4f7d5008aeaedbe2117eba2da711a0d71d1f451f` |
| `scripts/check-phase164-canonical-identity/fixture-source.ts`     | 12957  | 183   | `b92d2d91da1dd7715e7de37737769f30bd543cb2fd8a7a1eecb92aaf3335f3e3` |
| `scripts/check-phase164-canonical-identity/historical-tooling.ts` | 104671 | 213   | `47bbcd914cf5a632f618310b69b987462551ca2f103bce823a8d0cc6326a5f65` |
| `scripts/check-phase164-canonical-identity/source-files.ts`       | 8832   | 137   | `5cd178af07ea7f27204a2586d94bf8b1731dc6fd6b32c7b1ded0e07b6fc32832` |
| `scripts/check-phase164-canonical-identity/test-modules.ts`       | 3081   | 151   | `be66396b89e593c7493fb8e8a6a0701c0a758566fd31f768b777f4ac796a29b5` |
| `scripts/check-phase165-shared-exact-work.test.ts`                | 38206  | 627   | `810f649a91c18edcfe601f7de7e328bc5b576d01dc418941a21951ce84b6144f` |
| `scripts/check-phase165-shared-exact-work.ts`                     | 14361  | 266   | `68fb5efa600e55220e819fd8c578c13aac8e877996e6bcdd9c5eb29e874ba503` |
| `scripts/check-phase165-shared-exact-work/contracts.ts`           | 75327  | 590   | `160ff50aad2601a453e90a2d7c1cb610bf76fcad4d96cb7b846219a1a0ca1cdf` |
| `scripts/check-phase165-shared-exact-work/test-support.ts`        | 1395   | 23    | `19cfa9b55b5a46be045fa6a7d6a24e79ffdc558290ed5a7227b5d512c04e6f6a` |
| `scripts/check-phase68-active-chain-persistence.test.ts`          | 1299   | 32    | `0733297ecdbe5c272fe41cf7df21bb8ccc6f36499a1c49c5b64800907a1bf9e4` |
| `scripts/check-phase68-active-chain-persistence.ts`               | 6221   | 199   | `b923b54c80d84dfbf6b2cde2b63fe115e70e73ac46a38bd0fcc5f7b512febf6e` |
| `scripts/verify.sh`                                               | 34943  | 439   | `e1823a94c0394107ecdfbb194b32d93f290cc7b79195dac5583dd7cc7a811fb7` |

All 18 files are at or below 628 physical lines. Hash/row inventory review is not a claim that every underlying Rust function was behaviorally reviewed here. Production Rust, fixture behavior, final documentation and formal/security acceptance require their assigned reviews.

## Remaining Gates

This preliminary report does not close Plan 28, Plan 24, any later bounded repair plan, Plan 14, whole-phase code/security review, fresh default native verification, goal-backward verification, or normal-hook commit/push. No final phase status, requirement completion, qualification, or commit is asserted.

_Reviewer: gsd-code-reviewer; static tooling review only._
