---
phase: 162-real-prune-retention-and-integrated-parity-proof
plan: "07"
subsystem: testing
tags: [fjall, missing-history, prune-intent, recovered-coins, normal-maturity]
requires:
  - phase: 162-01
    provides: Shared normal-maturity accepted recipe and actual Fjall adapter
  - phase: 162-06
    provides: Completed long same-datadir successful lifecycle and both client directions
provides:
  - Four real missing-body/undo activation refusals with exact durable preservation across closed reopen
  - Private persisted-intent conflicts and eligible serialized resume with exact allowed effects
  - Recoverable unflushed ahead rows distinguished from genuine coins/metadata incompatibility refusal
affects: [162-08, 162-10, 162-12]
tech-stack:
  added: []
  patterns: [test-only accepted prefixes, guarded cold reopen, exact persisted namespace comparison]
key-files:
  created:
    - packages/open-bitcoin-node/src/sync/tests/filter_index/phase162/missing_history.rs
    - packages/open-bitcoin-node/src/sync/tests/filter_index/phase162/recovery.rs
  modified:
    - packages/open-bitcoin-node/src/sync/tests/filter_index/phase162.rs
key-decisions:
  - Fresh faults preserve absent BASIC ownership; saved faults retain actual safe h101 and protection from h102.
  - Forced eligible intent uses genuine settled h309 to obey the real keep window, while remaining explicitly below normal prune-after1000.
  - Actual unflushed index work and BeforeChainMeta effects distinguish recoverable old prefix from incompatible coins/metadata refusal.
requirements-completed: []
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 162-2026-10-11T16-25-37
generated_at: 2026-10-11T22:03:56Z
duration: 10 min recorded receipt window
completed: 2026-10-11
---

# Phase 162 Plan 07: Missing History and Interrupted Recovery Summary

**Genuine Fjall histories preserve exact required mates, coins and valid filter prefixes across missing-input and pending-intent refusals; ahead rows recover only to real coins authority or explicitly refuse incompatibility.**

## Performance and Ownership

- Tasks: 2/2. Exactly three assigned Rust paths changed. No production source, visibility, API, dependency or shared raw recipe was changed.
- First source/API compile receipt: 2026-10-11T21:54:02.559Z. Final lint receipt ended 22:03:29.325Z; this summary was generated at 22:03:56Z. Mandatory context loading preceded the first receipt.
- AGENTS, Bright Builds sidecar, placeholder-only overrides and architecture/code-shape/testing/verification/Rust standards informed the implementation. STATE, execute-plan reasoning/template, phase context/research/UI contract and relevant preceding acceptance summaries were inspected. Both active lesson inputs were fully loaded: 7,188 bytes and 2,397 conservative estimated tokens. No archive or new lesson-audit trigger was involved; no project skill directories exist.
- The parent supplied synchronized pinned baseline a77edab276a9daa086055aa61e6143ce86bc1218 and materialized Knots. Read-only GSD initialization confirmed 16 plans, ten prior summaries and six incomplete plans. Chain auto mode is active; lifecycle identity is preserved.
- Parent owns STATE/ROADMAP/REQUIREMENTS/todo, parity registries, generated LOC, independent reviews, full native/formal gates and Git finalization. No staging, commit, branch, push or hook bypass occurred. Requirements remain pending.

## Task 1: Actual Missing-Mate Matrix

All histories use `phase162-normal-maturity-1`, actual runtime consensus parameters and normal maturity 100. Every block has an ordinary saved body and accepted `connect_local_block` receipt; Always settlement writes actual coins, undo and active metadata. Historical and same-block spent scripts at h102 are checked against genuinely persisted undo. No MemoryOnly backend, seeded coins, maturity override or fabricated safe authority is used.

The adapter's original `new` retains its existing behavior. A private supplied-path/startup-mode constructor lets fault tests declare cleanup before the first runtime owner and build genuinely fresh BASIC absence. Saved cases first complete and settle h0..101, then accept and settle h102..104 without another index turn. Actual coins B and complete metadata are h104 while saved rows/checkpoint remain h101 with protection from102.

| Concrete case          | Actual missing input                                                                         | Preserved owner/prefix/protection                                        | Configured closed reopen outcome                                               |
| ---------------------- | -------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------ | ------------------------------------------------------------------------------ |
| Fresh body             | Real physical body at h102 removed after closure                                             | No owner, rows, checkpoint or reserved lock acquired                     | Typed BlockIndex corruption/Repair; missing BASIC activation body at102, twice |
| Fresh undo             | Real physical undo at h102 removed after closure                                             | Same fresh absence                                                       | Typed BlockIndex corruption/Repair; missing BASIC activation undo at102, twice |
| Saved/lagged body      | Real physical body at h102 removed after closure                                             | Exact102 immutable rows through101, exact checkpoint101 and lock from102 | Same explicit body refusal, twice                                              |
| Saved/lagged undo      | Real physical undo at h102 removed after closure                                             | Same exact valid prefix and conservative protection                      | Same explicit undo refusal, twice                                              |
| Irrelevant future body | Actual accepted unflushed h104 body removed after closure                                    | Genuine B/meta/checkpoint101 and102 immutable rows                       | Successful configured reopen at101, twice                                      |
| Irrelevant future undo | Actual accepted h104 undo persisted through ordinary `save_undo`, then removed after closure | Same actual old authority; remaining body preserved                      | Successful configured reopen at101, twice                                      |

Future undo is taken from the real managed accepted snapshot, never invented; persisting that genuine source mate does not advance coins or metadata. This is a focused irrelevant-suffix control, not a crash-recovery or ordinary flush claim.

Every required-input fault captures both decoded mates before removal, asserts the targeted raw key exists, removes exactly one physical key using existing private raw namespace helpers only after every owner closes, then compares all key/value rows in block_index/chainstate/coins after each failed configured reopen. Subsequent decoded inspection verifies the other mate, actual B/meta104, exact immutable rows, owner, checkpoint, protection, absent intent and unearned have-pruned. Activation preflight refuses before startup catch-up can publish progress; neither request fallback nor successful-empty response is substituted.

## Task 2: Private Intent and Ahead-Row Recovery

| Concrete case                       | Actual durable facts before production reopen                                                                                | Observed result and second closed reopen                                                                                                                                                           |
| ----------------------------------- | ---------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Fresh required-input intent         | Forced private intent identifies genuine h102 pair; B/meta104, no BASIC owner/prefix                                         | Typed required-input conflict before resume; same private intent, both mates and every persisted byte retained twice                                                                               |
| Saved/lagged required-input intent  | Same actual h102 intent and pair; B/meta104, prefix/checkpoint101, protection102                                             | Same pre-resume conflict; exact saved prefix and all durable bytes retained twice                                                                                                                  |
| Eligible completed-prefix intent    | Genuine accepted/settled h0..309, B/meta309 and complete310-row prefix before forced h20 intent                              | Existing serialized initialize resumes exactly that pair; same coins/meta/owner/checkpoint/protection/rows; intent cleared and have-pruned earned; second cold open stays settled                  |
| Ordinary unflushed ahead rows       | Genuine B/meta101, processed104, safe101,105 actual immutable rows; ordinary accepted h102..104 unflushed                    | Production configured recovery returns processed/safe101 and lag0; h102..104 hash rows survive but active lookup returnsNone; all persisted bytes stay equal on both opens                         |
| Genuine incompatible coins/metadata | Actual BeforeChainMeta effect leaves completed B104 with metadata101, safe101,105 immutable rows and all actual source pairs | Live failed flush returns no success receipt; later index turn refuses. Two configured cold opens explicitly refuse coins-best-block/durable-metadata mismatch without changing any persisted byte |

Pending intents are deliberately constructed through the pre-existing private raw-intent seam, representing focused interruption/control origin below actual prune-after1000. They do **not** prove normal RPC/automatic threshold planning. Plan06 independently owns that long successful planner proof. The eligible control still obeys the real288-block keep window: h20 is289 behind actual settled tip309. No guard was relaxed.

Resume effects are constrained to the exact target body key, exact target undo key, prune_intent, have_pruned and the existing prune_summary. Decoded summary must move from zero to exactly one batch, one height and last20, and remain unchanged after the second reopen. Every other block-index/chainstate byte and every coins byte is equal; active metadata, immutable prefix, checkpoint, owner and protection also receive decoded comparisons. Required-intent identity is directly read through node-private storage before/after refusal, rather than inferred from have-pruned.

The incompatible case uses the existing actual publication fault. Coins and undo really complete before metadata fails; no B/metadata rewrite pretends recovery succeeded. All non-genesis body/undo pairs remain through post-refusal inspection. Recoverable old-prefix success and incompatibility refusal are separate named tests.

## Verification and Initial Failures

All Cargo commands ran serially with pinned Bun1.3.9 on PATH and Rust1.94.1 through `bun run scripts/command-timings.ts run --key KEY -- cargo ...`. Test commands use `test --manifest-path packages/Cargo.toml -p open-bitcoin-node --all-features SELECTOR -- --test-threads=1 --nocapture`; nonzero discovery is required. No long584-row pressure corpus or unaffected broad suite was rerun.

| Timing key                          | Result                                                                                             | Wrapper ms |
| ----------------------------------- | -------------------------------------------------------------------------------------------------- | ---------- |
| phase162-fault-source-preflight     | Existing source/API no-run compile passed,0.42s Cargo; no behavioral discovery claimed             | 733        |
| phase162-fault-matrix-compile       | New matrix no-run compile passed,9.89s Cargo                                                       | 10,148     |
| phase162-missing-history, first     | 2 passed / 3 failed, 6.36s; saved setup violated recipe minimum 102                                | 6,772      |
| phase162-missing-history, corrected | 5 passed / 0 failed, 16.33s                                                                        | 21,239     |
| phase162-recovery, first            | 4 passed / 1 failed, 13.96s; eligible intent correctly refused real keep window                    | 21,542     |
| phase162-recovery, second           | 4 passed / 1 failed, 20.12s; successful resume changed shipped summary omitted from test whitelist | 28,224     |
| phase162-eligible-intent-replan     | Exact eligible selector 1 passed / 0 failed, 17.06s                                                | 38,942     |
| phase162-recovery-final             | Final 5 passed / 0 failed, 39.91s                                                                  | 40,808     |
| phase162-missing-history-final      | Final 5 passed / 0 failed, 35.08s                                                                  | 35,780     |
| phase162-fault-matrix-clippy        | node all-targets/all-features with -D warnings passed,12.62s Cargo                                 | 13,044     |

Initial failures were test-setup/expected-effect errors, not production defects or successful semantic RED for new functionality. This plan adds evidence for existing behavior; no artificial failing production implementation was introduced. The shared recipe's constructor at packages/testdata/phase162/recipe.rs:32 enforces102..1100; owned setup was corrected to102 without changing that contract. The eligible case first exercised the genuine keep-window refusal in storage/fjall_store/prune.rs:301. After two same-step failures, the executor reported a concrete checked replan and the parent explicitly acknowledged it: inspect storage/fjall_store/prune/records.rs:96, permit only shipped prune_summary alongside exact existing effects, assert its decoded0→1/1/20 transition, run the exact eligible selector, then both final matrices and strict lint. No blind rerun, waiver, guard weakening or ownership expansion occurred.

Exact final receipts:

- Recovery: `.local/open-bitcoin-dev/command-timings/phase162-recovery-final/2026-10-11T22-01-53.528Z-380c6e00-e946-42ef-9e31-7267484d55c8.json`.
- Missing history: `.local/open-bitcoin-dev/command-timings/phase162-missing-history-final/2026-10-11T22-02-37.461Z-798f3ae8-a1ca-4931-bd5a-5575fe3c55e7.json`.
- Checked exact eligible retry: `.local/open-bitcoin-dev/command-timings/phase162-eligible-intent-replan/2026-10-11T22-01-05.431Z-050cde98-8e39-4f9f-b3ac-5a8d88e9adee.json`.
- Strict lint: `.local/open-bitcoin-dev/command-timings/phase162-fault-matrix-clippy/2026-10-11T22-03-16.281Z-89a9a6bc-8bbb-4b93-b42f-8e80f4dfb838.json`.

Scoped rustfmt check passes for all three owned paths. Parent-registered breadcrumbs verify 1,137 Rust files. Managed all checks report 0 findings; Phase160 peer-filter and Phase161 operator claim guards pass. Owned-source whitespace/stub scans and the three-path source review pass. Clippy supplies compiled diagnostics; no semantic-navigation diagnostic tool was available. Quiet yields were polled normally and no running command was interrupted or treated as timed out.

## Simplification, Cleanup and Source Handoff

The explicit simplification pass keeps one private supplied-path adapter constructor and small shared accepted-extension/flush-only/snapshot helpers. Separate files contain the missing-history and recovery concerns. No generic fault framework, dependency, second authority or production visibility change is needed. Functions remain below the approximate161-line trigger; owned file sizes are424/360/500 lines, all below628.

The final ten tests cover 11 concrete cases/datadirs: six missing-mate cases and five recovery cases. Matrix observations include 14 typed configured-open refusals and 8 successful configured cold opens, excluding fixture setup/genesis and cleanup opens. All failed and successful fixtures use cleanup guards declared before owners. Guards actually reopen Fjall after owners close, then remove the datadir and assert path absence. Final actual TMPDIR and /tmp inventory finds zero remaining focused-fault directories. There are no owned live Cargo sessions.

| Owned source                                                                       | SHA256                                                           |
| ---------------------------------------------------------------------------------- | ---------------------------------------------------------------- |
| packages/open-bitcoin-node/src/sync/tests/filter_index/phase162.rs                 | 0a99e440e5bd2cd89c689b4cf5575fcc12951d042af162b31e17c81f269d3d90 |
| packages/open-bitcoin-node/src/sync/tests/filter_index/phase162/missing_history.rs | 43b233fa1b7f9685f3ab8f24c508f84bd50b2731739b733801d57e4eca5c021a |
| packages/open-bitcoin-node/src/sync/tests/filter_index/phase162/recovery.rs        | 9deca92df80b1259c1dd7d04a592fd31058af6795127cb77315318a364d210b7 |

Both actual new child paths were immediately reported and parent-registered. Their exact ordered anchors are packages/bitcoin-knots/src/index/base.cpp; packages/bitcoin-knots/src/index/blockfilterindex.cpp; packages/bitcoin-knots/src/node/blockstorage.cpp.

For parent-owned parity publication, add this summary and the `phase162_missing_history`/`phase162_recovery` selectors and final receipts to the retained-filter recovery evidence beside the long Plan06 lifecycle. Preserve distinctions between required missing input, irrelevant future input, forced private interruption, ordinary unflushed work and incompatible actual effect failure. No production behavior difference or requirement owner is created by these tests.

## Declared Mitigations and Limits

| Threat     | Concrete executed evidence                                                                                                                                                                                                  |
| ---------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| T-162-07-1 | Direct private intent identity and both required mates preserved before resume across two fresh/saved configured refusals; eligible serialized resume changes only exact authorized existing effects.                       |
| T-162-07-2 | Ordinary unflushed processed104/safe101 reopens only at genuine B/meta101 while retaining immutable extras; real B104/meta101 effect mismatch explicitly refuses unchanged.                                                 |
| T-162-07-3 | Four genuine missing-mate combinations require typed nonempty error categories and byte-identical valid-prefix/source preservation across two closed opens; irrelevant future controls succeed without fabricated progress. |

No blocking stub, unwired data source, new production endpoint/auth/schema/dependency or unmodeled production trust surface was added. Raw test keys and errors stay in test source; this report contains aggregate categories/counts/timings only. Independent source/security review, unresolved-High disposition, full default `bash scripts/verify.sh`, formal lifecycle validation and evidence-gated requirements remain parent gates. No ASVS certification is claimed.

The synthetic root, caller-supplied branch work and inherited Open Bitcoin210000 versus Knots Regtest150 halving interval remain explicit limits. These focused branches are first-party acceptance/recovery evidence, not whole-chain Knots consensus differential or most-work selection. They do not replace the long successful retention/client scenario. BIP37/V0, assumeutxo/dual-chainstate, archive scale, public defaults/network CI, implicit repair/import/download, production services/readiness/funds and milestone audit/archive/shipment remain excluded. Phase161 presentation/schema/redaction and prior v2.4 crash-window advisories remain unchanged.

## Task Commits and Next Handoff

Task1, Task2 and metadata commits are explicitly deferred by D-14. Shared lifecycle/requirement files and Git were not mutated by this executor. No authentication gate or outside-owned production defect occurred. Cargo is idle; the remaining checked plans and final parent gates may proceed.

## Self-Check: PASSED

All three owned Rust paths and this summary exist; final source hashes match both final matrix and lint evidence. The exact eligible retry and final missing/recovery/lint receipts exist with exit0. Frontmatter has exactly two top delimiters, the matching lifecycle identity, generated_by and empty requirements-completed. Final scoped formatting, source whitespace, breadcrumbs, managed/claim checks and cleanup inventory pass. No commits are falsely claimed because D-14 defers them. Root retains state updates and final native/formal/Git gates.
