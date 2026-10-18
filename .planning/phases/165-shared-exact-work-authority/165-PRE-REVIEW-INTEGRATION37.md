---
phase: 165-shared-exact-work-authority
reviewed: "2026-10-17T23:16:18Z"
depth: standard
generated_by: gsd-code-reviewer
lifecycle_mode: yolo
phase_lifecycle_id: 165-2026-10-14T16-09-59Z
review_scope: plan37-task1-source-before-identities
files_reviewed: 5
files_reviewed_list:
  - scripts/check-phase128-production-compact-announcement-transport.ts
  - scripts/check-phase128-production-compact-announcement-transport/durable-trigger.ts
  - scripts/check-phase128-production-compact-announcement-transport.test.ts
  - scripts/check-phase134-apply-boundaries/aggregate-roots.ts
  - scripts/check-phase134-authoritative-lifecycle.test/apply-helpers.ts
findings:
  critical: 0
  warning: 0
  info: 0
  total: 0
status: clean
---

# Plan37 Source Review Before Identity Activation

## Result

**CLEAN: no open findings in the five final Task1 owners.** This independent review covers the completed source, mutation tests, actual raw command evidence and full-file inverses before source identity derivation or row/pin activation. It is not the canonical whole-phase code review, a native verification pass, security acceptance or Phase165 completion.

Material guidance: repository AGENTS.md and AGENTS.bright-builds.md; standards/index.md; architecture, code-shape, testing and verification standards; TypeScript/JavaScript and Rust guidance; and the checked Plan37 contract. standards-overrides.md contains no applicable exception. Both active lesson files were read completely: 7188 combined bytes and 2397 estimated tokens. No lesson archive was loaded, and neither project skill directory exists. The review follows the existing GSD phase workflow and leaves Git finalization to Root.

## Authenticated scope

The complete preparation packet `.local/open-bitcoin-dev/phase165-plan37-before.json` has SHA256 `59815295052425c09f52a05e545d0bcd0b814a7edb5778a088268e25b74444ef`. Its four original source bodies and new-helper absence were read, and all stored body hashes were independently recomputed. The checked plan is `165-37-PLAN.md`, SHA256 `d383153ecf174623ab20b6d45ae823b76ba6cad953e2924383f3a46e5170a914`.

The final Task1 packet `.local/open-bitcoin-dev/phase165-plan37-task1-final-source-review.json` has SHA256 `bba3bdccc613443e457cbb055b3ac2c2e24887d1a3307f7e112c86913cd884b4`. All five complete final bodies match their actual canonical working files, byte counts and hashes. All five inverse records were independently applied in memory: four recover their authentic full original bodies exactly, and one recovers the recorded new-module absence. The 13991-byte final diff has SHA256 `517fd8ed637e10ab074ac424c5c0196fed6ffe0f11f77bb98fd004035e9c4066` and was read in full; the separately captured new helper was also read in full.

| Final source owner          | Lines | Full raw SHA256                                                  |
| --------------------------- | ----- | ---------------------------------------------------------------- |
| Phase128 checker            | 566   | 43d202575c3ed366901a03861f05fb2790ed2b8cdece0fea7e55384bf33341e7 |
| Phase128 durable-trigger.ts | 81    | 90a131b2707648385d5fc15e329a777bb1bb65619062bcce5b14e6bdab2793a5 |
| Phase128 test               | 447   | 4ebfcc735f6e9f08b24597709caf85aa27a52ef443b201c15e3cfd03ab6636cc |
| Phase134 aggregate-roots.ts | 357   | 4e8b12f6dda81dd430a4e337d66efb727a256ae6452e40492607ad182f7be9d0 |
| Phase134 apply-helpers.ts   | 604   | 74f589b1c048da3363f36ae439910ee59d596670ffff91ef5371011b8ef49ea2 |

The three held activation dependencies and both actual Rust dependencies match the preparation packet completely, including their current working bytes:

| Read-only dependency              | Full raw SHA256                                                  |
| --------------------------------- | ---------------------------------------------------------------- |
| Phase164 test-modules.ts          | 4428312183d12e6ce73cae507b4a6b546c39106a70552a16da040d57344a863a |
| Phase165 checker entry            | 92c545cb37f04967349a085517739872f93bc5e2499b23918a88929f37e851c7 |
| Phase165 contracts.ts             | c6bbdb06f393d41f00fe15b52f81026802aa9a5bc2bdd4345ab91b2f8f753ef4 |
| Node sync/block_response.rs       | 718f8eb2fa0e552cb26fd43e62cd92ade6145973fc96b957885d54121060d11c |
| Node network/mempool_lifecycle.rs | fa186eeafab8925c40006f7a20671c265af261a364698712b751562b6baa8bbb |

The existing ordinary-impl attribute helper, Rust evidence helpers and Phase134 test registration were inspected as read-only context. No production Rust, Phase129 source, historical adapter or recorded historical identity was changed by these five owners.

## Source assessment

### Requested bodies and actual active-tip announcements

`durable-trigger.ts:9` retains the existing pure predicate interface and existing diagnostic caller. The checker extraction removes only the old local predicate and relocates its imports. Negotiation, transport/write/failure/acknowledgment, observability, verifier and bounded-scope logic remain byte-exact outside that extraction/import change.

The new predicate follows actual `block_response.rs:118`: every requested present body is saved before disposition, including pending and accepted-side bodies. It requires the exact two failure exits before local-hash, accepted-progress or queue effects. The save-error body permits the existing BASIC failure notification and propagates both notification failure and original save failure. After successful persistence, the required sequence is local-hash recording, Connected disposition, requested/present body, accepted-progress recording, actual active-tip check and durable announcement queue. Duplicate, disconnected and nonextending progress handling remain required.

A preparation review observation identified a call-only proof gap: the first GREEN generation required the active-tip helper call but did not prove its implementation. Root and the executor resolved this within the same Task1 truth before identities. Three new tests first demonstrated actual false negatives for a constant-true helper, inverted hash equality and any-present-tip acceptance. The final predicate now requires the unique ordinary active-tip helper to share the identical enclosing ordinary DurableSyncRuntime impl with disposition. Its masked executable body must compute the candidate header hash, propagate the live maybe_chain_tip lookup error and compare that hash with the active tip using is_some_and. Comments, test-only helpers and unrelated owners cannot supply the required helper. The final fifteen new Phase128 cases include these controls and the original persistence/qualification mutations.

This narrow source contract deliberately follows current executable Rust shape. It does not claim to replace compilation or runtime behavior tests.

### Authorization and aggregate lifecycle boundaries

`aggregate-roots.ts:232` adapts only the two connected seam selectors and their authorization-aware inspection. Both actual seams in mempool_lifecycle.rs acquire `self.authorize_connect(block, consensus_params)?`, then the same chainstate receiver consumes that capability before mempool preparation, sealing and the existing transaction root. The predicate binds the exact acquisition receiver and arguments, the actual prepared-chainstate receiver, two executable authorization uses, the authorized preparation argument vector and call ordering. Current local header time and stored timestamp shapes remain consistent with their lifecycle context.

The ordinary aggregate root, connected transaction root, callback order, accepted persistence-error adapter and fallible/effectful scanner remain unchanged. The prior connected-path mutation exclusion and sealing-to-transaction effect exclusion remain active. The existing Phase134 discovery, reachability and persistence modules are not replaced or suppressed.

Fourteen new mutations exercise seven independent failures on each direct/stored seam: absent acquisition, substituted receiver, substituted block, substituted consumed capability, substituted chainstate receiver, acquisition after preparation, and an effect between sealing and transaction. Existing factory tables and all original mutation bodies remain, except the required stale preparation-method retarget. The helper now rejects unchanged or duplicate function-local anchors before mutation. The complete suite's positive and negative discovery remains nonzero.

### Test preservation and simplification

All original test registrations, loops and bodies remain, with only the diagnosed 24-to-16-space Phase128 local-hash anchor and old-to-authorized Phase134 preparation anchor retargeted. New Phase128 cases prove a clean baseline and a unique, changed anchor before checking the exact intended diagnostic. Phase134 retains its baseline beforeAll, actual function-local required anchors and exact apply-boundary failure expectations.

The final shape is one focused pure extracted helper plus one narrow existing seam adaptation. No generic parser, external dependency, broad allowlist or packed source was introduced. All five owners remain below628 readable physical lines. No additional simplification is needed for this bounded change.

## Actual verification evidence

The reviewer independently authenticated all six complete receipt bodies against their packet command records and recomputed every one of the23 referenced raw log hashes. These are executor-run Bun1.3.9 commands, not commands rerun by this reviewer.

| Receipt                         | Full raw SHA256                                                  |
| ------------------------------- | ---------------------------------------------------------------- |
| Original source/baseline RED    | 28afd4c99f53a5340e9d3cc672f2cf39634be2283304e5927bf025e47cfc015c |
| First GREEN generation          | 1f30b10032766283da26145ebfef30a2c6a5efe850792c7b6c79c42582a91159 |
| First generation scope          | fbe4ac3e85a329ca825a7c943b4622d04a5cc86325310dc298494982c81963d1 |
| Actual active-tip mutation RED  | 7abc04668bfd3569632f96cfb86eb05935c9ae65a6d23362f6d1b1dae7427783 |
| Final complete GREEN generation | e4cf3846abd76b96011a065e8b0252edc68f086c24db8130c02347f48add3904 |
| Final physical caps/whitespace  | b094c5e2a072fd795026c195d90f0813f981caabe4ee2fc6df749c3e90768bef |

The initial128 and134 CLIs each exited1 at their diagnosed source contract. The initial128 positive test failed at its expected empty-failure assertion;134 failed its baseline beforeAll empty-failure assertion. These were real guard failures, not missing anchors or syntax/import failures. The first full GREEN was92/293/12 passing tests, preserved as its own earlier generation.

The subsequent active-tip RED log has SHA256 `b5c8953695134fde9878ef3c9db02e5737474ad67cd3ebb389006531b4882a37`: three failures after each clean baseline and changed-anchor assertions, at the expected-finding assertion because the weakened helpers were incorrectly accepted. Its12 assertions and three named failures establish the actual gap. Final whole suites and all four CLIs ran again after repair:

| Final command              | Actual result                                                           | Launcher duration | Full raw log SHA256                                              |
| -------------------------- | ----------------------------------------------------------------------- | ----------------- | ---------------------------------------------------------------- |
| Phase128 full test file    | 99 pass,0 fail,195 assertions                                           | 2068ms            | 4a1eef63325f38e407787f9c66c0aadc929b7bc581ac2827a294bc7c4ff9a1e1 |
| Phase134 full test file    | 293 pass,0 fail,438 assertions                                          | 17151ms           | 907db8b7aa09103b0488bad4139590d6fa3de6b3d7432954707c41c9418fe355 |
| Phase129 full test file    | 12 pass,0 fail,12 assertions                                            | 1360ms            | 9db276dfc8ab3451b098115f0113537c629d7a4f5edc995add8588f955a84c63 |
| Phase128 CLI               | exit0                                                                   | 126ms             | 850a0ebe54f2445eec538f2fe66ef40987379a0f8d1f00ac346b73a77001dd17 |
| Phase134 apply CLI         | exit0; eight target apply functions and classified transitive discovery | 138ms             | 5f400593c56e20bcc8f3ac6417fee2a4cf8c1ba4bca4d444e9255f02061c3b2a |
| Phase134 authoritative CLI | exit0                                                                   | 47ms              | 921a79fcf94dec8130088f50924c9a8b47f63025c1552fa89b6e1e271a4c5d56 |
| Phase129 CLI               | exit0                                                                   | 757ms             | f7740916e2d66e4b0e854a8f0ae49c9c4ad27788f949353ceca97576710c6235 |

Final physical-cap command and git diff --check each exited0. The cap log is `713871435c0fe67099f2ffd9ded4f62689c24e0fef02efe62d01485b7aed83a5`; the empty whitespace log is `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855`. Full raw logs are retained beneath `.local/open-bitcoin-dev/phase165-plan37-*` and authenticated by the final packet.

## Remaining gates

Root still owns independent source acceptance, source identity derivation, inert row/pin proposal review, real stale-pin refusal, activation, focused ancestry/ownership checks and complete raw generation closure for Task2. Those actions are not accepted by this source-only report.

Both earlier failed native attempts remain failed evidence. Remaining filter guards, the entire fresh default native verifier including Rust, coverage and Bazel, final documentation/LOC generation, canonical whole-phase code review, security verification, formal requirement/lifecycle verification and normal-hook commit/push remain pending. This review neither completes CWOR requirements nor permits Phase166 work.

The reviewer made no source, test, activation, metadata or Git changes and ran no Bun, Cargo, Bazel or native verification command. This report is the only reviewer-owned file created.
