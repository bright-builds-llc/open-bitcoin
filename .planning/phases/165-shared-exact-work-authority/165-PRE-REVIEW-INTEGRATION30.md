---
phase: 165-shared-exact-work-authority
plan: 30
reviewed: 2026-10-17T21:08:54Z
depth: deep
files_reviewed: 5
files_reviewed_list:
  - packages/open-bitcoin-node/src/chainstate/tests.rs
  - scripts/check-phase165-shared-exact-work.ts
  - scripts/check-phase165-shared-exact-work/contracts.ts
  - scripts/check-phase165-shared-exact-work.test.ts
  - scripts/check-phase164-canonical-identity/test-modules.ts
findings:
  critical: 0
  warning: 0
  info: 0
  total: 0
status: clean
scope: independent-source-before-pins-and-inert-activation-proposal
phase_acceptance: false
native_qualification: false
---

# Plan 30 independent source review before pin activation

CLEAN for the exact five current source identities and the exact inert proposal below. No actionable correctness, security, integration, or test-reliability finding remains in this bounded change. The earlier fixture proof regression is repaired: the target test now reaches the intended missing second input through a genuine earned owner instead of passing on an earlier lineage rejection.

This is a preliminary independent review within lifecycle `165-2026-10-14T16-09-59Z`. It does not activate arrays or pins, accept Phase 165, or qualify ordinary networking or a Knots daemon. Root owns finalization. The reviewer made no source, pin, plan, shared-metadata, or Git changes and ran no Cargo, Bazel, native tests, or full Bun matrix. Only bounded, isolated source-policy unit calls were run.

## Inputs and exact current scope

Repo AGENTS.md, Bright Builds sidecar/index, applicable architecture, code-shape, testing, verification, Rust and TypeScript standards, overrides, and active lessons were loaded. Global and repository active lessons totaled 7,188 bytes, below both loading limits; both were read completely without archives. The existing GSD workflow authorizes this scoped report.

Authenticated complete bytes:

| Input                              | SHA-256                                                            |
| ---------------------------------- | ------------------------------------------------------------------ |
| Current 165-30-PLAN.md             | `486ad9b2cbfefe5b7b77ad07dbea73d9188f3e304dbb27fd0ab6aaae66344b0e` |
| phase165-plan30-before.json        | `f31e31e4e0a1819a9032aaaff9e7d032fd90b0fefbe9778da95663ffc1127c25` |
| phase165-plan30-prepin-review.json | `23c43eeee05e08d5fe4ca57836f4b0040dc9f4c3e3c3f7cf4b753a1b649440c7` |
| phase165-plan30-proposal.json      | `4548c5c83f8adf440a3f297c646c8738711289d13f075097c1e9197aa63b69fc` |
| Complete inert activation previews | `1aabbc4de64d67f9f9dcd81cf91f2528c33a5aad87eae7708b55358e45c36ea7` |

All five complete current bodies matched the prepin packet again immediately before writing:

| Source                                                    | Lines | Full-file SHA-256                                                  |
| --------------------------------------------------------- | ----: | ------------------------------------------------------------------ |
| packages/open-bitcoin-node/src/chainstate/tests.rs        |   613 | `6a654bf137f8be887a525c64f025c9e080a78f08d3daf3bb90e94b382c33cfb6` |
| scripts/check-phase165-shared-exact-work.ts               |   269 | `3d99c51d718773c93e25cc9cc66f94761547a173ff5c58cd1a67c9f31e0cfc11` |
| scripts/check-phase165-shared-exact-work/contracts.ts     |   617 | `985ab8fe99477c201f82157afc34088cc76e7630f38033b9d905a89d2cc75878` |
| scripts/check-phase165-shared-exact-work.test.ts          |   627 | `6dcde6c9b4c563326904fe9bc6952ee152596601bdb5fd274b4a37c61ca708d4` |
| scripts/check-phase164-canonical-identity/test-modules.ts |   387 | `e3c0e7fd6d67a7bb7d6a0278003e446c9e44dc39cfd51400f147b6920d845eab` |

All five respect the approved 628-line cap. Complete before/after bodies were inspected; the two unchanged activation owners were also authenticated.

## Genuine owner and failure-signal repair

The target `prepare_connect_then_drop_leaves_parent_unpopulated` consumes the genuinely accepted seeded manager into its original store and core, prepares the current coins best block, and uses the closed `CoinsFlush` trait to flush genuine effective coins into their parent. Rewrapping preserves the original core and lifecycle. Live origin, earned work, and binding under the complete existing maturity-one parameters are asserted explicitly. There is no raw snapshot promotion, mutable coin import, count normalization, new production observer, or production change.

The first input is the actual genesis outpoint, available in the parent while cache occupancy is zero. The second input is the exact absent txid-nine/vout-zero outpoint. Assertions establish two-input order before preparation. The error must equal `MissingCoin { outpoint: missing_second }`; a generic error no longer passes. The existing production transaction-context loop traverses inputs in order using `peek_coin`, so reaching this exact second error proves the available first parent coin was traversed. Cache occupancy remains zero, the first parent coin remains spendable, and complete snapshot, tip, and coins-best-block observations remain unchanged.

An independent inverse removed the target-function delta and the minimal `CoinsFlush` import, reproducing the complete original source SHA `a77c8ca532e9d14716adacbb8dd8f9bf786367a05da9a1502aef88a64c10e472`. All other ten direct tests and helpers are byte-exact. The saved RED source was separately authenticated at `1bdc751c518ad49d11d003c3a5aa01d6738003a4284f63e808260eb324e7e8f3`; reversing only its precise-error assertion change reproduces the original setup.

## Actual execution evidence

Receipt JSON, actual argv, exit values, logs, and complete log hashes were authenticated, rather than inferred from a summary. Native commands were executed by the executor through the pinned Bun 1.3.9 timing wrapper.

| Actual command scope                      | Result                                                 | Wrapper duration | Log SHA-256                                                        |
| ----------------------------------------- | ------------------------------------------------------ | ---------------: | ------------------------------------------------------------------ |
| Exact target discovery                    | Exit 0; exact named test listed                        |         10.285 s | `e48bb48bb757b00b8555e1f8a0c42e8141a922f54459510564aa3e5250dcf42c` |
| RED exact target                          | Exit 101; 0 passed, 1 failed, 0 ignored, 1909 filtered |          0.459 s | `f0298db6ffca74a45530fe33283a1f3d17d25e0f4ab6eb8abd6173b4c97901f2` |
| GREEN exact target                        | Exit 0; 1 passed, 0 failed, 0 ignored, 1909 filtered   |         10.028 s | `dc9217a2b78c648f87d9d7b6bf1c61f33816dc313dff5bbc51351060dc110042` |
| Full chainstate test module               | Exit 0; 14 passed, 0 failed, 0 ignored, 1896 filtered  |          0.358 s | `730e8b7d6b2a6f4ac3634f9cf70186cddd07951756ad53711b579acedfb7f458` |
| cargo fmt --all -- --check                | Exit 0                                                 |          1.789 s | `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855` |
| Strict Node all-target/all-feature Clippy | Exit 0                                                 |          8.064 s | `f3915cb2363166a1c280bda3ad5c079a8332a7eca94838e0cfa9233353098c50` |
| Node all-target/all-feature build         | Exit 0                                                 |          0.387 s | `a5f4c585ee974ca44916ac30a98bbc189e067a7e0a6bc6d2e8d6bc525be724af` |

RED actually reports `UntrustedWorkLineage` against expected exact `MissingCoin`; GREEN reaches the latter. The full module comprises eleven direct tests plus three existing child tests. The exact target overlaps that module and is not additional independent coverage.

## Controls and inert activation

All 26 unique source-control triples were inspected against their actual Rust anchors. They cover consumed ownership, raw rewrap refusal, best-block preparation, real closed flush, origin and earned authority, full parameters, initial/intermediate/final zero cache occupancy, exact missing outpoint and input order, exact error, snapshot/tip/best-block immutability, parent retention, and test disabling/removal. Each anchor occurs once; the two declaration controls intentionally share an anchor with distinct mutations.

The contracts delta adds only its target constant and inert typed control array. Removing that complete declaration delta reproduces the original contracts body. The existing main test loop has exactly three changes: importing the new group, adding it to the existing strict mutation iteration, and including it in the existing unique-anchor/passing-baseline condition. Reversing those three additions reproduces its complete old body.

Independent isolated unit calls imported the actual policy functions and actual exported arrays. The old baseline and proposed in-memory baseline both returned zero findings. Every one of the 26 mutations returned the exact target file and `ownership` rule. The stale live pin rejected the proposed input inventory before reads. All temporary exported-array changes were restored in `finally`; the process did not change live source or pins. These are source-policy controls, not executed Rust mutants. The complete independent program and 26 results are preserved in `.local/open-bitcoin-dev/phase165-independent-plan30-review-audit.json`.

The proposed literal activation adds exactly one SOURCE row at 157 and one TEST row at 87. The latter only appends the target's count eleven to the existing old-164 leaf; no historical owner, type, helper, other array, or source history changes. A restricted literal parser independently compared complete old and proposed arrays. The target source identity independently derives as `d8722a49e277139f00bd3f9e2996fe808b2fe7a7641388eb8d81aefcb6ffc32c`.

The derived inventories are 158 SOURCE rows, 88 TEST owners, and 277 unique INPUT paths. The sole new input is the target at index 263; removing it restores all 276 old paths in the same relative order. Independent compact JSON hashing gives `9d2fb8f19b943757a968574d6997b65350d29a8a20e4d57b37f834d468f3ae8c`.

Each complete prospective file was independently inverted to its complete prepin body:

| Prospective owner                         | Lines | Full-file SHA-256                                                  |
| ----------------------------------------- | ----: | ------------------------------------------------------------------ |
| Current-165 contracts, sole SOURCE append |   618 | `64008967e4446d07b70e6514d2bdfa7e99ae44b911da2d2866992f016e854c8f` |
| Old-164 leaf, sole TEST append            |   388 | `c021c8a3f6e12a410d986d63050e166c32a2345a4045d2422ed2d4f4e881b15f` |
| Current-165 entry, sole pin replacement   |   269 | `9adb6a82bbfe62f8624cc008000d3d0409a4e4d08079c8eb54c580250d13ac5c` |

## Authenticated read-only dependencies and limits

The nine complete packet dependency bodies also matched current files:

| Dependency                                               | Full-file SHA-256                                                  |
| -------------------------------------------------------- | ------------------------------------------------------------------ |
| node/src/chainstate.rs                                   | `1d25580fd120dd80c5129d065ae5e31da460392be9112f6c2ef767f2defb2ba8` |
| node/src/chainstate/tests/filter_index.rs                | `710affcbc9848b91ec6f8c95fadaac7384626726902b315c165944c5b0ceaac3` |
| chainstate/src/engine.rs                                 | `c1d99d5b719d3c04176442f5fe3faadf18dbd8d6bdc71865a92d17a367306156` |
| chainstate/src/engine/work_authority.rs                  | `6c3412159f23e761a283adcccbaf45af3b0cf9aca62af607184d5fc612785dbb` |
| chainstate/src/engine/apply.rs                           | `d8aa7b003e59c5304eadfc3dc7ba6e4b59a3d90c3883ebe0640c909decac7223` |
| chainstate/src/coins.rs                                  | `cd59da7b3d47af02d06266ef1fcfaaa4e7d9984438d2a28ef36f201284aa54b2` |
| chainstate/src/coins/cache.rs                            | `c4607eb78e30621e1edc093563cc7fb1fe650d72eadf3430a3aae78508222664` |
| chainstate/src/types.rs                                  | `6ecbd780c0c0501a28b749a496ca4f7431f8ab89c36a0af1a5c487ca9c5b4f62` |
| scripts/check-phase165-shared-exact-work/test-support.ts | `0f10303178fc3cad81296092694d551d6fa2ddfe832f2f58f434a12fdf0b9c2e` |

Package abbreviations in this dependency table mean `packages/open-bitcoin-node` and `packages/open-bitcoin-chainstate`. No independent re-review of all production behavior is implied.

Live activation, actual focused guards after activation, final input closure, full matrices, fresh coverage, documentation review, and final strict verification remain separate gates. Reuse this review only for exact recorded identities or the exact recorded prospective literal-only activation. Phase 165 covers CWOR01/02/03/05; CWOR04 and ordinary canonical bootstrap/reopen, contextual validation, and actual daemon qualification remain deferred to 166–168.

## Final identity observation

After the independent CLEAN decision was sent to root and this report was first saved, the final read-only identity check observed the three activation owners at exactly their reviewed prospective hashes: entry `9adb6a82bbfe62f8624cc008000d3d0409a4e4d08079c8eb54c580250d13ac5c`, contracts `64008967e4446d07b70e6514d2bdfa7e99ae44b911da2d2866992f016e854c8f`, and old leaf `c021c8a3f6e12a410d986d63050e166c32a2345a4045d2422ed2d4f4e881b15f`. The Rust owner, main test owner, and all nine dependencies remained byte-exact to the prepin packet. This observed transition introduces no unreviewed source delta. It does not establish post-activation guard results; root and executor own those gates.
