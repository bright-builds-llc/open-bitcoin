---
phase: 165-shared-exact-work-authority
reviewed: 2026-10-17T18:19:36Z
depth: deep
files_reviewed: 1
files_reviewed_list:
  - scripts/check-phase165-shared-exact-work.test.ts
findings:
  critical: 0
  warning: 0
  info: 0
  total: 0
status: clean
scope: plan33-r2-single-inline-loop-anchor-correction-and-unchanged-generation1-boundary
---

# Plan33 R2 affected-source review

## Verdict

CLEAN for the actual single-tuple correction in `scripts/check-phase165-shared-exact-work.test.ts:42`, raw SHA256 `6b778a3c2cac1411258823536ff08b4d0613d3565d1f24970f48d870eb1b0f08`. The intended complete-prefix omission mutation now changes the actual current seed loop. Both original assertions remain exact. No production Rust, contract, inventory, pin, inverse, classifier or test scenario was changed. This source review supports the parent's retry-freeze decision; it does not mark new full matrices, native gates or Plan33 complete.

## Authentic before, exact change and meaning

The complete before packet is `.local/open-bitcoin-dev/phase165-plan33-r2-test-before.json`, SHA256 `feb401373c1e3b8f17a8dec1f24be6a06c2a34e08e1e94b1ff9fba39304dff2c`; complete after packet is `phase165-plan33-r2-test-after.json`, SHA256 `ac5bab45691538983e33aa06a07d15017f15ff43072dfc78ac9f1121746e234b`. Both complete source hashes were independently computed and compared to live bytes. Both files are exactly38,530 bytes/627 physical lines. Exact diff artifact is `phase165-plan33-r2-test-exact.diff`, SHA256 `3e68341493e5e6265fe07db67a824c188497ba7381cbcfa72aaa8d9ecd10c532`.

Original raw test source is `25a3a4f6a271136b93885bae58b6be9295e12cbbb8dd974410b4210cf508c1d0`. Independently replacing exactly one original tuple with the new tuple produces the complete actual after body, and the exact inverse recovers the complete authentic before body:

```text
before tuple:
["recipe.blocks.iter().enumerate()", "recipe.blocks.iter().skip(1).enumerate()"]

after tuple:
["for block in &recipe.blocks {", "for block in recipe.blocks.iter().skip(1) {"]
```

The other tuple in the same inline table, P2SH→NONE recipe flags, is unchanged. The actual runtime fixture raw hash remains `1c1e5c2fcba87ea6ffda69c3ecfbe569a309d74bc39c011a0648bdc3afa36513`. Its `seed_count` function has exactly one `for block in &recipe.blocks {` at line215, following genuine empty-owner transfer and Managed source creation. Every recipe block, starting with genesis, is physically persisted and connected with ZERO hint, actual recipe flags and full selected params; the receipt height/hash is checked and appended. The new replacement skips the first block in that real loop, so it represents the original intended complete-prefix omission. The obsolete enumerate anchor is absent from this current Rust file; replacing it was a no-op.

At test lines43–45 the unchanged statements still require `changed` to differ from actual `bridgeReal` source, then require `legacyFunctionContractView` to throw the same migrated-fixture legacy source-version diagnostic. The control changes actual raw source before its exact authenticated version check; it neither substitutes old fixture bytes nor weakens the intended refusal. This is source-policy evidence, not execution of the malformed Rust mutation. No assertion, case, owner rule or diagnostic is removed.

## Independently closed unchanged boundary

Authenticated generation1 post-pin packet is `phase165-plan33-postpin-review.json`, SHA256 `20d784eda28c6fb7f4a66946cc8c6713833f343cb6bfe989a9aeaf4d8205043d`. All other eight complete tool bodies still equal that approved packet and live files:

| File under `scripts/`                                 | Physical lines | Unchanged raw SHA256                                               |
| ----------------------------------------------------- | -------------: | ------------------------------------------------------------------ |
| `check-phase164-canonical-identity/contracts.ts`      |            621 | `c365814a57962fb4d42e700a716d30c8f619da0705afb26e19053a1de59bcde7` |
| `check-phase164-canonical-identity/source-files.ts`   |            142 | `8a119054832dabfef84abfd11bb7a769b2ea168857a00c059e125302d54d2154` |
| `check-phase164-canonical-identity.test.ts`           |            625 | `aaa1d40efc31284db15906d5c25a3643bf8ddbc6061410c06178ba8e23c10cd6` |
| `check-phase165-shared-exact-work.ts`                 |            269 | `621668aed07ae2dded1757ca906369e6f402a8f8dc79e98ddc7941ef3c2c7f4a` |
| `check-phase165-shared-exact-work/contracts.ts`       |            618 | `30d68aee8fec18f86cb274011e6b334783ef4605096fca048692409970ae94fb` |
| `check-phase164-canonical-identity/fixture-source.ts` |            187 | `cec4709ae363dc7fd7d675288d5d45b696f6ac5106a6f81aa68b14d10af6a7ae` |
| `check-phase164-canonical-identity.ts`                |            264 | `61a17138fc78fde48bf3f2eb3e29e1300acf0225e403bafed13f1aa2ffe15555` |
| `check-phase165-shared-exact-work/test-support.ts`    |            183 | `0f10303178fc3cad81296092694d551d6fa2ddfe832f2f58f434a12fdf0b9c2e` |

All17 complete dependency bodies/hash identities remain exact to the approved packet, including seven accepted Rust files and five ancestry sources. Previous independent ancestry/inverse review applies only to these unchanged hashes. No count-writer, snapshot promotion, header/body authority, hardcoded2d/b92/933/FAA expectation or historical artifact changed.

Independently parsed current SOURCE_CONTRACTS151, TEST_OWNERS83, SOURCE_FILES140 and EDGES522 literal rows and compared full ordered values to approved actual arrays. Approved INPUT273 remains unchanged, and its compact JSON SHA equals the actual current inventory literal `c1189173084e431bd849a4d1fa75b2797eb6224f4b262cff5d9fd9d98f1ad282`.

Additionally reread ALL516 entries in generation1 freeze `phase165-plan33-full-freeze.json`, SHA256 `3323097b955b89d1cb51731f84e8dbec7999bafb6bf73e907d79261ec7fc819a`, checking canonical path, presence, byte size and full raw hash. The ONLY changed entry is the authorized current165 test file. Every other515 entry remains exact, including514 total present entries across the boundary and the two preserved absences. Its historical closed receipt `phase165-plan33-full-freeze-closed.json`, SHA256 `e54e0ea10d2e5c47ec35240a4d3489d407418930e086b3264db8c82d9ddaf0b2`, remains intact. R2 requires a new capture/closure; this report does not rewrite the failed generation's identity.

## Actual failure and focused retry evidence

Authenticated generation1 results manifest `phase165-plan33-matrix-matrix-results.json`, SHA256 `3033b065b814e43da40cbd2e606f7d2384cba8f5ff0f8252508b0587b58353ac`, and independently hashed both full logs:

- Old164:928 pass/0 fail,1797 expectations, exit0,439.209931292s; log SHA256 `6231c442f19955387c49b4775da2bedc8d17203c273a7bbbf4f187e9a5670525`.
- Current165:1864 pass/1 fail,6846 expectations, exit1,1036.9140585s; log SHA256 `b4142247b241b72422f4a0006abd1c60235df9066e0f9d68f166d70af96f1762`. Sole failed case is the stale `recipe.blocks.iter().enumerate()` inline bridge control, at the unchanged line44 no-op assertion. Later driver gates did not run.

The completed owning-group retry log `phase165-plan33-r2-bridge-focused.log` is SHA256 `5b7239cb38211ecd2cd5682a179f8c4cdd0eecc0b27b8a291f854c85a659f00c`,19,127 bytes. Pinned Bun1.3.9 reports151 passes/0 failures,1714 filtered,410 expectations,20.57s. Both actual inline cases appear as passes, including `phase165 C legacy bridge refuses changed recipe/loop for block in &recipe.blocks {`. This receipt validates the focused bridge group for the corrected source. It is not a new complete165 matrix or native receipt.

## Limits and remaining gates

This reviewer performed only read-only source/hash/literal/inverse/log checks and scoped formatting of this owned report. No Bun/project import, test, Cargo/build/heavy command, source/pin/metadata/Git edit or rollback ran. Parallel Plan31 planning artifacts were not changed. Existing repo instructions/standards and fully loaded active lessons remain applicable.

Parent must capture and independently approve the new complete retry read/freeze boundary, run the required fresh complete matrices and all later fresh driver/native gates, and close the actual raw inputs/logs before Summary33 acceptance. Earlier successful or failed receipts do not qualify changed test bytes as full GREEN. Original failures remain preserved. Final14 default native/coverage/Bazel/review/security/formal/normal-hook commit/push and future phase qualification remain unclaimed.

```yaml
issues: []
```
