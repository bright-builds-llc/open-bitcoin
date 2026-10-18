---
phase: 165-shared-exact-work-authority
reviewed: 2026-10-17T05:23:02Z
client_date: "2026-10-17"
depth: standard
review_kind: preliminary_source_only
review_state: completed_preliminary_source_review
status: clean
diff_base: d0104a997c0362bf65b81de1c8fec8ddc3012a1b
files_in_scope: 8
files_reviewed: 8
findings:
  critical: 0
  warning: 0
  info: 0
  total: 0
files_reviewed_list:
  - packages/open-bitcoin-node/src/sync/tests/filter_index/catch_up.rs
  - packages/open-bitcoin-node/src/sync/tests/filter_index/catch_up/failures.rs
  - packages/open-bitcoin-node/src/sync/tests/filter_index/catch_up/rpc_faults.rs
  - packages/open-bitcoin-node/src/sync/tests/filter_index/evidence/accepted_faults.rs
  - packages/open-bitcoin-node/src/sync/tests/filter_index/evidence/retention.rs
  - packages/open-bitcoin-node/src/sync/tests/filter_index/operator_status.rs
  - packages/open-bitcoin-node/src/sync/runtime_state/tests/header_checkpoint.rs
  - packages/open-bitcoin-node/src/sync/runtime_state/tests/header_checkpoint/faults.rs
generated_by: gsd-code-reviewer
lifecycle_mode: yolo
phase_lifecycle_id: 165-2026-10-14T16-09-59Z
---

# Phase 165: Preliminary Plan 18 Rust Review

The final source-only review is clean within these eight Rust files: no actionable correctness, security or test-reliability findings. The closure below supersedes the earlier preparation state while preserving its initial hashes and observations. All eight complete stable owners and their fixed-base changes were read. No tests, builds, heavy Bun execution, source/shared-state changes or Git mutations were performed by this reviewer. Final whole-phase review remains required.

## Context and Evidence Boundary

Current Plan 18, Plan 17's completed summary, latest plan-check amendments and current inventory-check/parser amendments were consulted. Root instructions, Bright Builds sidecar, placeholder-only overrides, architecture/code-shape/verification/testing/Rust standards and both active lesson inputs were loaded in this ongoing review session. Lessons total 7,188 bytes and a conservative 2,397-token estimate; no archive was loaded. No project skills or `.claudeignore` exists.

Plan 17 supplies genuine actual-prefix adapters, including `ValidatedHistory::live_source/live_runtime` under its full maturity-one parameters. The parent approves reuse where those full parameters and actual prefix match, without editing earlier helpers. RPC fault setup has a distinct full Regtest parameter selection; matching maturity alone is insufficient. The approved checkpoint-local wrapper must preserve genuinely empty and zero-cache pressure premises. No extra helper path or API ownership is inferred from tight files.

The preparation baseline shows only the two header-checkpoint owners already differ from the fixed Git base; the other six Rust owners are byte-identical to that base at capture. Existing checkpoint helper movement and prior authority changes therefore cannot be attributed wholesale to Plan 18. Final review must distinguish this earlier cumulative delta from the new repairs.

## Original Assertion and Fault Inventory

The fixed-base tests, assertion locations, flushes and fault setup were inventoried before implementation review.

| Owner                         | Original intent to preserve                                                                                                                                                                                                                                                                                                                                                 |
| ----------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `catch_up.rs`                 | Exact/one-under acquisition, output/body/item budgets; bounded prefix-independent operation counts and legal singleton; stronger saved lock; actual accepted unflushed facts processed ahead while old safe B remains; later real flush releases zero-record work; disabled/stale/idle/startup next-height behavior.                                                        |
| `catch_up/failures.rs`        | Mutated-body pause/retry keeps target; genuine accepted writer error preserves complete facts and refuses poisoned publication; append fault commit boundaries and conservative reopen from safe prefix.                                                                                                                                                                    |
| `catch_up/rpc_faults.rs`      | Actual Pending/final-read readiness before publication or query faults; sticky safe operator failure, bounded/redacted reads and unchanged namespaces; backend error versus absence; missing target/parent/corruption distinctions; cancellation and raw clone invalidation; same genuine live owner through accepted successor, then separate raw all-handle observations. |
| `evidence/accepted_faults.rs` | Accepted target/scripts survive later undo/coins/meta failures; original old physical B and safe prefix; requested-body real GetData/visible error/zero credit/absent body; before/after commit immutable row counts and retry identity; disabled prepared completion causes no release.                                                                                    |
| `evidence/retention.rs`       | Only uncheckpointed accepted suffix disappears on raw reopen; checkpoint at 19 while accepted suffix 20–400 remains unflushed; Periodic/Always/manual release must use actual own flush; paired body/undo and filter protection remain; automatic pruning retains exact deletion count, usage reduction, durable intent completion and untouched residual body commitments. |
| `operator_status.rs`          | Genuine later lag/processed-ahead/flush status transition; bounded readonly collection, disabled/no-genesis/legacy meanings; missing body/undo and persistence poison retain evidence; equal-height shared fixture case remains separately owned by Plan 20.                                                                                                                |
| `header_checkpoint.rs`        | Real periodic/manual/automatic wiring; admitted header ahead/successor without scripts authority; failed admission typed before writer; empty/legacy unchanged coverage; unsupported memory no-op; exact selected zero-cache pressure effect; no-backfill stale header refusal; conservative raw reopen observations.                                                       |
| `header_checkpoint/faults.rs` | Snapshot before commit preserves old bytes and permits retry; after-commit error retains durable new bytes but fails until reopen; skipped turns preserve attempts and armed fault; typed Headers backend errors; same genuine original source before indexed stale-branch work; exact B, prune locks, body/undo and coverage/snapshot assertions.                          |

## Checks Required at Source Closure

- Genuine actual source assembly must finish before targeted faults are armed. Faults must reach the intended writer/read/snapshot effect, rather than constructor reads or an earlier origin/parameter refusal.
- Full selected parameters must remain consistent. The shared maturity-one helper is not a replacement for RPC Regtest parameters with other differences.
- Unflushed suffix acceptance must preserve actual old physical B, safe prefix/protection and later genuine flush semantics; no missing/pruned payload restoration or count normalization.
- Populated constructor graphs are already bound. New nonroot descendants use normal contiguous ingress under the same parameters, with no null-parent genesis Headers, root rebinding or raw header writes.
- Header-only acceptance, ScriptsValid provenance and live work lineage remain distinct; raw observations are not promoted into authority.
- Original physical absence, exact rows, error categories, accounting, prune/body/undo/script assertions and all-handle cleanup must remain effective.
- Immutable `support_runtime.rs` and earlier prefix helpers remain unchanged. `rpc_faults.rs` starts at 625 lines and `operator_status.rs` at 618; final files must fit the existing cap without extra undeclared helper ownership.
- Broader operator-status/readiness failures sharing Plan 20 premises remain pending. Owned filters do not establish their closure.

## Preparation Identities

Initial whole-file hashes were captured while baseline failure collection was active. They are not closing identities.

| File                                                                                  | Initial lines | Initial SHA-256                                                    |
| ------------------------------------------------------------------------------------- | ------------- | ------------------------------------------------------------------ |
| `packages/open-bitcoin-node/src/sync/tests/filter_index/catch_up.rs`                  | 592           | `d93590c2a15e0180cf7308ac811f57d8d1f0029f77ab0acfba80fd03817d9453` |
| `packages/open-bitcoin-node/src/sync/tests/filter_index/catch_up/failures.rs`         | 186           | `e031790ac760e83c70f08689c22251c97889418d576beb7fa9d97269cc15cda7` |
| `packages/open-bitcoin-node/src/sync/tests/filter_index/catch_up/rpc_faults.rs`       | 625           | `56fb0db4ced62105f30737f3ecea131b852fa63084dfec97090fdc0990614a67` |
| `packages/open-bitcoin-node/src/sync/tests/filter_index/evidence/accepted_faults.rs`  | 388           | `4d5ed033abbfb8af3ee72ccb57efccf031b06ae318c6a77b175d9af8a1342af8` |
| `packages/open-bitcoin-node/src/sync/tests/filter_index/evidence/retention.rs`        | 351           | `761c7b2bc8500eeb420841424c837d36ed0ec31000f60ab5da70cf9c44eda198` |
| `packages/open-bitcoin-node/src/sync/tests/filter_index/operator_status.rs`           | 618           | `cab922f8165c70920d14012364e68ab4bfd42b95ec9b367ea1ee7d5ba8f12a6b` |
| `packages/open-bitcoin-node/src/sync/runtime_state/tests/header_checkpoint.rs`        | 528           | `5918608616577c5236b87fb8b2e495d46b0700ca707b181f1bc9efaacebbc1c6` |
| `packages/open-bitcoin-node/src/sync/runtime_state/tests/header_checkpoint/faults.rs` | 401           | `0ada40e24ecb25997a1ebf595d861fd2c7a0611e4ad7c78982bb3f640401b9fc` |

Initial ordered scope digest: `035f518c60ea1ae512dd91e7a1ab2c5101a711745407f31a3a89dfd1a348daad`.

Digest construction is SHA-256 over the eight frontmatter paths in order, each encoded as UTF-8 path, tab, lowercase complete-file SHA-256, newline. Immutable `packages/open-bitcoin-node/src/sync/tests/support_runtime.rs` still has SHA-256 `5a25857c552bb7dcfdd040ad15e19c4616f52ef8b35df69b4f11a42dcd5b1f05` at capture. Final source drift will be explicitly attributed after the executor's stability signal.

## Preparation Boundary (Historical)

Wait for stable Rust source, read all eight final complete owners and their final changes, then capture closing identities and actual executor receipts. This preparation makes no behavioral GREEN, full source acceptance, native verification, later-phase completion or Git finalization claim. Final whole-phase review remains required.

## Focused Retention Boundary Check | 2026-10-17T05:15:27Z

**PASS for the proposed new helper precondition only.** The new post-constructor assertion may expect the actual desired durable height 19 rather than the initially preseeded filter height 15. This does not issue a general final review verdict or certify execution.

The original `seed_at_checkpoint` helper in the fixed base had no assertion. Its caller passed prefix 16 and desired durable height 19, then called ordinary `open_configured` before any suffix work. That original startup already initialized the BASIC owner and drove an ordinary index turn. The genuine constructor uses the same normal finish stage. Thus a post-startup checkpoint at 19 reflects the same actual durable B19 boundary. No original `records[15]` or safe15 observation exists in this file.

An exact source comparison proves the original measured block from `finish(&runtime)` through final cleanup is byte-identical in the current repair: processed400, safe19, physical B19, paired-delete refusal and body/undo20 presence before flush, actual Periodic/Always flush, zero-record release to400, paired deletion20, retained immutable filter and reopened B400. Only the added helper precondition is at issue; no original safe15 assertion was moved or weakened. Later source changes require rechecking this preservation proof.

The parent reports initial 49 cases with 18 passing and 31 failing, an unchanged original volume case passing in 185.65 seconds, and first repaired selected execution 47/48. Those are executor-attributed diagnostics, not tests run by this reviewer. The root-requested narrow comparison used only source reads and an in-memory exact substring comparison. The overall Plan18 review remains pending source stability.

## Final Source Review Closure | 2026-10-17T05:23:02Z

**Verdict: clean; zero critical, warning or info findings.** All eight complete final files were inspected after the stable-source signal. The report is an independent source review with executor-reported behavioral evidence separately attributed. It does not replace final whole-phase review/security/native/formal gates.

### Source and Assertion Preservation

- `catch_up.rs` and `catch_up/failures.rs` change only the live-mutation prerequisite to the already-reviewed Plan17 `history.live_runtime()`. Actual old-prefix acceptance/flush/BASIC precede successor work and pressure/fault injection. Original accepted-facts, safe-prefix, poison, script and release assertions remain unchanged. Raw budget/codec/recovery tests retain their earlier semantics. The existing mutated-body retry intentionally restores its corrupted body as before; these repairs introduce no missing/pruned-data restoration.
- `catch_up/rpc_faults.rs::genuine` uses the existing Plan15 factory with the full local `params()` value, whose base is `SyncNetwork::Regtest.consensus_params()` plus maturity one. It does not substitute Plan17's different default-based parameter set. Only the old two-block prefix is actually accepted and flushed; constructor assembly and indexing finish before the same live owner accepts the pending successor. The factory source is consumed and its extra store wrapper dropped, so later raw all-handle error/recovery observations remain real. No raw reopen is promoted to authorize that successor.
- All RPC publication/read faults are armed after genuine setup and, where applicable, after actual Pending capture or row publication. Original typed backend-versus-corruption errors, MissingTarget versus missing-parent handling, request-time ScriptsValid provenance, readiness cancellation and sticky bounded/redacted operator failures remain asserted. Raw clone invalidation and malformed ancestry still refuse; no corruption is repaired to force a successful reopen.
- `evidence/accepted_faults.rs` uses the same genuine old prefix before actual unflushed acceptance. Intended undo/coins/metadata faults remain armed at the later normal flush; old physical B, old metadata/safe prefix, retained script facts and prune locks are asserted independently from accepted height two. The requested-body case reads the constructor's two-header prefix, explicitly proves the future body absent, then uses Version/Verack/new nonroot Headers/Block intake with actual GetData and visible error assertions. The failed body remains absent and raw reopen retains only the old durable tip. Original immutable commit-boundary row counts, retry byte identity and disabled prepared-completion controls are unchanged.
- `evidence/retention.rs::seed_at_checkpoint` actually accepts and flushes only the desired durable prefix under the selected parameters. Before normal startup it explicitly observes the preseeded safe15 checkpoint; after normal constructor startup it observes safe19 for durable B19. This matches the original startup boundary and preserves both observations without moving an original assertion. Every later body/undo added by the repair comes from actual successful live suffix acceptance, not from copying a future full snapshot into source provenance.
- Exact in-memory comparison again proves the original measured retention block from `finish(&runtime)` through final cleanup byte-identical to the fixed base. It retains processed400/safe19/B19, protected paired payload20, due Periodic/Always flush, zero-record safe release to400, actual paired deletion20, retained filter and reopened B400. The complete nonactive-volume helpers and automatic-volume test through end-of-file are also byte-identical, including the actual >550MiB threshold, 714 paired deletes, untouched nonactive bodies, usage reduction, completed intent, raw reopen accounting and final body commitments.
- `operator_status.rs` restores the original `opened()` helper byte-for-byte and changes only the intended later-successor test's call to `history.live_runtime()`. All its original lag/processed-ahead/durable status assertions remain. No-genesis/disabled/raw readonly/negative cases retain their previous setup. The shared equal-height reorg case remains unchanged and is not qualified by this plan's one-test status filter.
- `header_checkpoint.rs::runtime` adds local Atomic uniqueness to the existing fresh-directory label and uses the existing cfg-test strict empty-source adoption only on the actually empty target. It does not modify the shared runtime helper or snapshot attempt baselines. Root header fixture admission stays outside P2P through the existing helper; new descendants use normal ingress. The pressure case consumes a genuine empty owner into its already-initialized cache and keeps the exact zero-cache lifecycle/threshold policy rather than replacing it with normal-cache startup.
- `header_checkpoint/faults.rs::stale_runtime_configured` keeps full ordinary Regtest parameters and the explicit maturity100 assertion. Indexed setup flushes the actual same genesis/old-prefix core, then invokes normal BASIC construction before later branch work; it only reads the already-captured prefix. Nonindexed setup admits actual initial headers once, then actually accepts bodies. Replacement headers form a separate contiguous nonroot branch before genuine strictly greater-work reorg. Caller numeric carriers do not establish preference. Extra handle clones are dropped before later closure.
- Header snapshot faults remain armed after stale setup or after a prepared capture. Original old/new snapshot bytes, exact attempt counters, physical B/metadata, BASIC safe/immutable rows, prune protection, retained stale body/undo, skipped-turn unconsumed-fault behavior and typed Headers backend errors remain. Empty/genesis ordering and legacy coverage controls retain their original effects. Conservative raw reopen explicitly lacks accepted best proof before separate header-only fixture revalidation; it does not acquire live body work or reconstructed historical tie authority. Missing stale header snapshots remain missing without backfill.

No new production API, helper file, dependency, count normalization, immutable equality relaxation, missing-input fallback or ordinary recovery promotion was found in this eight-file repair. The temporary path Atomic is local to the owned checkpoint helper. Earlier genuine factory and accepted-prefix helpers remain unchanged.

### Closing Identities and Drift

All eight closing hashes match the executor's saved stable Rust map and were independently recomputed from the files after complete source reads. All eight differ from the preparation capture; each changed file was fully reread. The two header owners already contained earlier phase changes at preparation, as recorded above. All final files fit the 628-line cap.

| File                                                                                  | Closing lines | Closing SHA-256                                                    | Preparation-to-closing change                                                         |
| ------------------------------------------------------------------------------------- | ------------- | ------------------------------------------------------------------ | ------------------------------------------------------------------------------------- |
| `packages/open-bitcoin-node/src/sync/tests/filter_index/catch_up.rs`                  | 587           | `d906e48ba644026f7c9905e27b2812089f7011a24e3f05d0b641389dae10cac6` | Only accepted-unflushed live setup                                                    |
| `packages/open-bitcoin-node/src/sync/tests/filter_index/catch_up/failures.rs`         | 181           | `7bcf72e05d87049e85eb7d0a48c1ab7ab109a381a17a22d2ac45c5b5daa757d2` | Only accepted-writer live setup                                                       |
| `packages/open-bitcoin-node/src/sync/tests/filter_index/catch_up/rpc_faults.rs`       | 606           | `526debe0db3ea972f982f1b0ac88fb7c77dcd38f61831abb1b4fd0a7ac8f8d7f` | Full-parameter same-owner factory, remove raw intermediate reopening                  |
| `packages/open-bitcoin-node/src/sync/tests/filter_index/evidence/accepted_faults.rs`  | 394           | `40048b2bde5f3a1a2907b2d5e6182d9b79c39c2b9d8f60cabdb41357f34ac944` | Genuine fault prerequisites and explicit requested-body absence                       |
| `packages/open-bitcoin-node/src/sync/tests/filter_index/evidence/retention.rs`        | 396           | `4ab8ee124dfd42263727b936491647cbe8d918ba00a7f6626a295d11d6a33426` | Genuine selected checkpoint, both startup observations, real accepted suffix payloads |
| `packages/open-bitcoin-node/src/sync/tests/filter_index/operator_status.rs`           | 618           | `ef9795540086f247408f60ea02d5965749a3b172ad667760355bdc4bad267583` | One intended successor caller; original shared opened helper retained                 |
| `packages/open-bitcoin-node/src/sync/runtime_state/tests/header_checkpoint.rs`        | 577           | `e449aeed8ae0a65e23e7e654a20cccd4cc8609f982e2008b48aa372dbb777e8d` | Local empty/stale source adapters, preserved pressure lifecycle and unique labels     |
| `packages/open-bitcoin-node/src/sync/runtime_state/tests/header_checkpoint/faults.rs` | 408           | `418404faac133e33635924f4881458a73bc9402f50df1d414dfd571ed12373ac` | Same-core indexed stale setup and normal new branch ingress                           |

Closing ordered scope digest: `45f25a085071d8f743ce60ebfddb53641b7832287563b6aae4f31f684e6ece55`, using the path-tab-hash-newline construction documented above. Immutable `support_runtime.rs` remains SHA-256 `5a25857c552bb7dcfdd040ad15e19c4616f52ef8b35df69b4f11a42dcd5b1f05`; Plan17's `accepted.rs` remains `5a261b2a919c7b0022166bd727296947ef9faf04111b3c0436306e2c1afb9d0e`. None of the eight reviewed paths is ignored. This clean result applies to these exact identities; subsequent changes require delta review.

### Execution Attribution and Remaining Gates

The parent reports final Rust execution of 49/49 cases across seven groups, including retention3/3 and its actual >550MiB case in190.85 seconds. It separately reports passing format, strict Node all-target/all-feature Clippy and build. These are executor results; the reviewer ran no tests, builds or heavy Bun commands. The earlier 49-case RED and 47/48 intermediate diagnostic remain historical evidence, not current acceptance.

The parent also reports eight new current guard owners, 57 enabled static declarations and 236 fixed inputs, with no corresponding old164 owners to renew. Full guard matrices are starting under the executor's exclusive slot. Those TypeScript changes/matrices are outside this Rust-only preliminary review and are not prematurely certified here.

The broader status/readiness cases with later Plan20 prerequisites remain pending. No whole-workspace/default-native, complete coverage, ordinary selector/bootstrap/reopen authority, contextual completeness or pinned-Knots differential qualification is claimed. Only this report was written. Final whole-phase source review, security audit, goal verification and normal native/hook/Git finalization remain required.
