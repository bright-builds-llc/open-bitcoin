---
phase: 165-shared-exact-work-authority
reviewed: 2026-10-17T20:07:25Z
depth: deep
files_reviewed: 12
files_reviewed_list:
  - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/automatic_prune/fixtures.rs
  - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/automatic_prune/index_protection.rs
  - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/automatic_prune.rs
  - packages/open-bitcoin-rpc/tests/black_box_parity/phase127_composition.rs
  - packages/open-bitcoin-rpc/tests/black_box_parity/fixtures.rs
  - scripts/check-phase164-canonical-identity/contracts.ts
  - scripts/check-phase164-canonical-identity/source-files.ts
  - scripts/check-phase164-canonical-identity/test-modules.ts
  - scripts/check-phase165-shared-exact-work/contracts.ts
  - scripts/check-phase164-canonical-identity.test.ts
  - scripts/check-phase165-shared-exact-work.ts
  - scripts/check-phase165-shared-exact-work.test.ts
findings:
  critical: 0
  warning: 0
  info: 0
  total: 0
status: clean
scope: plan31-tooling-moves-controls-and-exact-inert-proposal-before-pins-with-accepted-rust-binding
---

# Plan31 tooling and pre-pin proposal review

## Verdict and scope

CLEAN for saved ALL12 pre-pin packet `.local/open-bitcoin-dev/phase165-p31-prepin-review.json`, SHA256 `b112118a1f9289ae8f971ae677e89348e737ac609a7f3455087f2a8271a1f07e`, captured `2026-10-17T20:02:30.863656+00:00`, and exact inert proposal `.local/open-bitcoin-dev/phase165-p31-reviewed-data-proposal.json`, SHA256 `e0342b5bdab0daff2d546c9aa15be718a8861780ecac532a472e1c388451b68d`. No remaining blocker was found. This review supports the parent's independent source/row/pin/read-set decision before activation. It does not assert an activated pin, passing loaded controls, current guard baseline, full matrix, native closure or completed Plan31.

All twelve complete source bodies, canonical paths, byte lengths, physical caps, raw hashes and live equality were checked. Five Rust bodies equal accepted packet `232d5d8d80121fca14d1ff81bcaefdc159086d4a753f4c1d867ba81cd8fc7e1d` exactly. Root acceptance `2c5c4794aef008f355ff8a8373fcf296bd0f0ab980e8d64090aee85cfdcc4453` binds independent Rust CLEAN `339072140733ee4477a6846527b9aa9f72bfc3f269c37960e8bf3d304118fc48`; it authorizes guard preparation, not pins/full matrices. This review inspects actual source-control semantics and bindings without replacing that independent Rust review or borrowing its receipts for changed tooling.

Active checked Plan31 source is SHA256 `806d863b14d91d323cfa27370851b5438e5aef5bfd7606e3b725796fcab09376`. Repo AGENTS, sidecar, overrides and relevant architecture/code-shape/testing/verification/Rust/TypeScript guidance remain applicable. The previously loaded active lessons total7,188 bytes/2,397 conservative estimated tokens, without omitted active blocks or a new audit trigger. No other agent's source, plan or shared state was edited.

## Exact source inventory before pins

These are complete raw identities. Later source drift requires affected review; the expected activation previews below are not silently included in this inventory.

| Source                                                      | Physical lines | Raw SHA256                                                         |
| ----------------------------------------------------------- | -------------: | ------------------------------------------------------------------ |
| RPC `tests/automatic_prune/fixtures.rs`                     |            546 | `c26e7b2448a7f18b8e4111271a2f7976722b6366e63cddd5f852c2bc0f000e70` |
| RPC `tests/automatic_prune/index_protection.rs`             |            524 | `9c475cdd66a9bc2004e30bf4839547a77f77d6f387948fbbdd8124d0a7001910` |
| RPC `tests/automatic_prune.rs`                              |            546 | `7d11b72eada22f2fe86018b16acf90b6381fa7185a48bc2e6a344c883743ced1` |
| RPC integration `black_box_parity/phase127_composition.rs`  |            626 | `a787810f831b3f68e78d4a85890e9b9fa05aeafade8317aa0e4e1465037fca99` |
| RPC integration `black_box_parity/fixtures.rs`              |            490 | `299555281521d51b7e19cb59cd2467f720357c2fc866b4f8446e0be2bd7c2304` |
| `scripts/check-phase164-canonical-identity/contracts.ts`    |             97 | `b89a08014fb53193ea1d2539990e4fcffb373112a9744dcb568a1ed58462e618` |
| `scripts/check-phase164-canonical-identity/source-files.ts` |            526 | `e7bd19c176acb390c90fa035c4019bee8afda79de1d831f26533d855ceda607f` |
| `scripts/check-phase164-canonical-identity/test-modules.ts` |            380 | `abe91b6cc84843bed7cbe9b5ffc8be038fa45c2941ac6eb257f9577e38b09262` |
| `scripts/check-phase165-shared-exact-work/contracts.ts`     |            581 | `4303b2907cb75af7e152a2b6ffad331aa20b92d7fdfc37af8611e195147d3ea4` |
| `scripts/check-phase164-canonical-identity.test.ts`         |            625 | `aaa1d40efc31284db15906d5c25a3643bf8ddbc6061410c06178ba8e23c10cd6` |
| `scripts/check-phase165-shared-exact-work.ts`               |            269 | `621668aed07ae2dded1757ca906369e6f402a8f8dc79e98ddc7941ef3c2c7f4a` |
| `scripts/check-phase165-shared-exact-work.test.ts`          |            627 | `02384f3557ba7a9010dd1cd2f88a9a2f0c02de1124ebeaaa0d429352770e43f0` |

Rust RPC unit paths in the first three rows are under `packages/open-bitcoin-rpc/src/bin/open_bitcoind/`; integration paths are under `packages/open-bitcoin-rpc/tests/`. Read-only integration root `packages/open-bitcoin-rpc/tests/black_box_parity.rs` remains2,768 bytes/73 lines, raw `e508e3206f10d10956b8611b9d23cce9d635a8b2cc9fcb39752c70490755e07d`, source identity `be3edb7097cb757729548961c2357523041546d69be1160a85456834599d6597`, zero functions and zero direct tests.

## Independently verified move-only checkpoint

Authenticated actual imported BEFORE capture `phase165-p31-before-move-complete-data.json`, SHA256 `e46a74df20f63cc77f50e0b595d5c81db0615ad3e1f107c2cb7db4fcf2baab0b`, actual AFTER MOVE-ONLY capture `phase165-p31-after-move-only-data.json`, SHA256 `bb1e6a1453a86234a9e1ef7e9eeeb2a7cb60b22f37355a2eba7f32aa328637ee`, and closure `phase165-p31-move-only-closure.json`, SHA256 `67d7a3041559a0f512dc92a5c23a8fba4e8314b048359dff7f024a83cf1d6965`.

Four complete before hashes independently match final33: old164 contracts `c365814a57962fb4d42e700a716d30c8f619da0705afb26e19053a1de59bcde7`, source-files `8a119054832dabfef84abfd11bb7a769b2ea168857a00c059e125302d54d2154`, test-modules `be66396b89e593c7493fb8e8a6a0701c0a758566fd31f768b777f4ac796a29b5`, current165 contracts `30d68aee8fec18f86cb274011e6b334783ef4605096fca048692409970ae94fb`.

Read the actual capture driver: it imports the actual BEFORE arrays through public consumer APIs, captures complete source bytes and records fresh actual AFTER exports separately. It does not execute proposed code as the expected oracle. Independently parsed the three expressions with a restricted recursive grammar permitting arrays/JSON strings/integers/booleans and ordinary commas/whitespace only; identifiers, calls, spreads and executable expressions refuse. Values equal all actual BEFORE/AFTER ordered exports. Whole declaration bytes, wrappers and types are exact:

| Declaration, including trailing LF              | Rows / physical lines | Raw declaration SHA256                                             |
| ----------------------------------------------- | --------------------- | ------------------------------------------------------------------ |
| EDGES with exact readonly five-tuple annotation | 522 / 524             | `72f07b956a55513ff347ef9e1b82b021c363f31b6c2394d3bb7e307141ec3bd2` |
| SOURCE_FILES with original `as const`           | 140 / 142             | `8a119054832dabfef84abfd11bb7a769b2ea168857a00c059e125302d54d2154` |
| TEST_OWNERS with exact readonly pair annotation | 83 / 85               | `7b6fd45ea9ca2a30ea74b24e6aca6cc952554f6fa709a6e6a606833a452a29ef` |

Independently applied both complete forward and inverse transformations for all four files. Only whole declaration moves, named compatibility re-exports and ordinary separators differ. Original entire TEST_MODULES/comment/body/order and every type/helper/inverse outside the moved spans remain exact. Three other captured tool files stay exact at the move-only stage. The actual resulting capacities are97/526/380/534. These saved bytes qualify ONLY the move-only state, not later live controls.

All six ordered arrays and actual inventory hash remain151 SOURCE/83 TEST/140 SOURCE_FILES/522 EDGES/273 INPUT with `c1189173084e431bd849a4d1fa75b2797eb6224f4b262cff5d9fd9d98f1ad282`. Recorded actual CLI14 and complete test/checker union36 path sets are unchanged. Independently checked graph node hashes, edges and cycle absence. Only source-files164→test-modules164 and contracts165→test-modules164 are added; existing contracts164 imports remain. The leaf has no imports. Public EDGES/SOURCE_FILES/TEST_OWNERS import APIs and readonly types stay compatible; no consumer relies on a removed local binding. No new module or hidden algorithm appears.

## Control preparation and meaningful targets

Controls preparation receipt is `phase165-p31-controls-preparation.json`, SHA256 `6b96636abaf7358e9008f42d35eef9fdb3876da3d5951087bb857936f70a48f3`. Independently proved that post-move current165 contracts differ only by the appended complete inert PRUNE_SOURCE_MUTATIONS declaration. The test differs only in the three existing lines that import, spread and apply the existing strict unique-anchor/passing-baseline condition to that group. Reversing those changes recovers the complete move-only files exactly. Other five tools remain exact to that checkpoint; old164 test is unchanged.

All44 rows have unique actual nonempty before anchors, distinct replacements and changed literal-bearing source identities. Each maps to its actual proposed SOURCE ownership rule. The existing mutation helper rejects missing/no-op input; the existing refusal helper requires the exact file and rule, not an unrelated finding. Every old data/control/test body is retained. No separate mutation loop, support edit, dummy call, blanket allowance or source-history normalization is introduced.

Coverage includes mined headers, exact typed raw UntrustedWorkLineage/cache absence/unchanged physical facts, genuine Dense complete0..1001 replay/full params/P2SH/ZERO hint,1002 derived undos including root, real paired byte accounting, disabled initial index/generation0, retained versus evicted cache positions0/1/713/714, all original-handle close, actual persisted prefix replay/position/undo comparison, derived counts/strict consuming transfer, physicalNone and UNKNOWN sequence, full maturity extension, both typed refusal reasons and post-fault manifest preservation.

Seven helper controls are substantive: three full predicate→true changes, separate block/transaction type-only weakenings that discard expected identity, and independent removals of exact NotFound length and object-hash checks. They target the actual public-scope pure predicates without moving any call site or effectful logic. One root route control cfg-disables actual `mod phase127_composition;`. Whole read-only root ownership also protects its exact path and fixture declaration; no fake helper/root test count is added.

These are reviewed source-policy controls. They have not yet been executed against a passing activated guard baseline, and this report does not represent them as executed Rust mutations or full guard acceptance.

## Exact inert proposal and four activation inverses

The actual current arrays are still initial151/83/140/522/273+c118. The inert proposal was independently checked against all complete ordered originals:

- SOURCE157: append only six actual owners, the five accepted Rust files and read-only root; no old SOURCE row renewal or reordering.
- TEST87: append only automatic_prune5, sparse fixtures1, Dense1 and Phase1273. Actual declarations and enabled attributes match5/1/1/3, including the existing two-thread Tokio case and both real typed negatives. Helper and read-only root have zero tests and NO TEST rows.
- SOURCE_FILES143: append sparse fixtures, Dense and shared black-box fixtures only. Existing automatic_prune, Phase127 and integration root were already read.
- EDGES606: preserve all522 old rows/order except precisely the actual automatic_prune test rename/raw digest and actual changed Phase127 test digest; append84 actual missing function regions. No blanket renewal.
- INPUT276: preserve all273 prior paths/order, inserting only the same three new source paths at140–142. Old164 INPUTS172→175 is the same explicit insertion. No other dependency is discovered or substituted.

Independently calculated fresh compact JSON-array pin `bc7fe4254b605e7b5ae5b9e888dc7bddc718b5c8bb9f3f9a85088a44ebcabe9b`. All source/test/file/function/input keys are unique. Independently masked Rust strings/comments/characters and scanned original function regions, recomputing all six new SOURCE identities and all86 NEW/RENEWED raw function digests. Complete five-Rust function coverage totals88:84 appended functions plus four existing rows, of which two are renewed and two remain unchanged. All88 final raw function digests independently match. Every actual function of all five Rust owners is covered exactly once in the final EDGES inventory; the read-only root has none. The existing helper's prior26 plus three relocated pure predicates are covered, as are ordinal-distinct transport methods. No expected digest is derived from historical Rust projection. Separate root prepin-only audit `c899320b4489f88446b8d522a9fe256007c6b497b2706fc684046ba4ae1762d5` records the same86-versus88 distinction; neither count is a runtime test count or activation acceptance.

Independently reconstructed each complete activation preview from actual prep bytes using ONLY the named rows and pin, then reversed each complete preview back to its exact current source. The sole extra source separator is the necessary comma after the formerly last SOURCE_FILES literal; its inverse restores the original no-comma bytes. TEST_MODULES remains exact. All typed wrappers, helper bodies, old rows and unrelated bytes are unchanged.

| Prospective activated file | Lines | Exact reviewed preview raw SHA256                                  |
| -------------------------- | ----: | ------------------------------------------------------------------ |
| Current165 contracts       |   587 | `44cd851ae445d64c41592b04e6cd268ba7d1f59b1df823f19d393fc8454dbebf` |
| Old164 test-modules        |   387 | `e3c0e7fd6d67a7bb7d6a0278003e446c9e44dc39cfd51400f147b6920d845eab` |
| Old164 source-files        |   610 | `7a62e1c9ff71b3d86c9afd1ce381edf7fa53124e25537696a5002b43b585a9ee` |
| Current165 entry           |   269 | `3d99c51d718773c93e25cc9cc66f94761547a173ff5c58cd1a67c9f31e0cfc11` |

No preview was activated or executed by this reviewer. Actual stale pre-read refusal, approved-only activation and the parent's independent actual row/pin reversal remain mandatory before matrix freeze. Expected capacities stay within628 with ordinary formatting; test627 and old test625 remain intact.

## Historical integrity and remaining gates

The complete old164 normalizer, ten Rust/three adapter inverse descriptors, sparse-pair rejection, original-span uniqueness/order/nonoverlap, hardcoded2d/b92/FAA expectations and BOTH unconditional PASSED-map generation bindings are unchanged by exact move and append proofs. All36 recorded module hashes also match actual live bytes. Explicitly authenticated current raw Rust1c, adaptercec, historical-tooling47bb, immutable bundle1330515f, manifest9783fa03, preservatione6cb5740 and arithmetic corpus3261c2d2 identities. Other seven legacy tuples,933/FAA original attribution and original captures/receipts remain exact. Current raw158 predicates and current source/edge checks do not receive historical Rust normalization.

Only read-only static source/hash/literal/graph/inverse checks and formatting this owned report were performed. No imported Bun module, test, Cargo/build/heavy command, source/pin/plan/shared metadata/Git edit or other agent rollback occurred. Source hashes remained exact at final check. No ignored proof snapshot was needed.

Remaining execution requirements are independent parent pre-pin approval, actual stale pre-read refusal and applied-row/pin inverse, passing loaded baselines and all44 attributed controls, complete fixed/static/test/checker/CLI/helper/factory/literal/driver/observed dynamic raw union including mutable test-modules and new edges, fresh full164/165 matrices, required native/strict/prune/black-box commands and complete unchanged raw/log closure. No33/older/partial/zero-filtered receipt qualifies changed31 tooling. Actual Summary31/root acceptance precede30. Final14 default native/fresh coverage/Bazel/code/security/formal/normal-hook Git gates and canonical/recovery/pinned-Knots/future qualification remain unclaimed.

```yaml
issues: []
```
