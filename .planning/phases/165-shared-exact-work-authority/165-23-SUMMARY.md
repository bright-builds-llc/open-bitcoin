---
phase: 165-shared-exact-work-authority
plan: "23"
subsystem: durable-sync-progress-and-restart-fixtures
tags: [genuine-fixtures, raw-reopen, useful-work, source-guards]
requires:
  - phase: 165-22
    provides: Reviewed timing and protocol fixture prerequisites
provides:
  - Genuine complete active-prefix seeds for progress and restart consumers
  - Explicit unearned raw reopen observations with original physical and request assertions
  - Six enabled owner registrations, exact263-input inventory and stable490-path matrix evidence
affects: [165-28, 165-24, 165-14]
tech-stack:
  added: []
  patterns: [Existing genuine seed, Header-only raw observations, Exact current source controls]
key-files:
  created: []
  modified:
    - packages/open-bitcoin-node/src/sync/tests/durable_tip.rs
    - packages/open-bitcoin-node/src/sync/tests/no_progress_diagnosis.rs
    - packages/open-bitcoin-node/src/sync/tests/soak.rs
    - packages/open-bitcoin-node/src/sync/tests/stay_current_persistence.rs
    - packages/open-bitcoin-node/src/sync/tests/stay_current_progress.rs
    - packages/open-bitcoin-node/src/sync/tests/restart_resume_matrix.rs
    - scripts/check-phase165-shared-exact-work.ts
    - scripts/check-phase165-shared-exact-work/contracts.ts
key-decisions:
  - Seed only active genesis for the two-body durable-tip reconciliation.
  - Retain inactive downloaded bodies separately through ordinary storage.
  - Keep real reopened status and request owners header-only and unearned.
  - Preserve rejected activity hints and stronger headers whose bodies remain absent.
  - Reuse existing mutation loops without growing the executable test module.
requirements-completed: []
requirements-contributed: [CWOR-02, CWOR-03, CWOR-05]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 165-2026-10-14T16-09-59Z
generated_at: "2026-10-17T12:14:04.901Z"
client_date: "2026-10-17"
duration: approximately40min
completed: 2026-10-17
tasks_completed: 3
files_modified: 8
---

# Phase 165 Plan 23: Genuine Progress Seeds and Unearned Restart Observations Summary

**Complete genuine fixture facts now reach the original durable-tip, diagnosis, soak, stay-current and restart assertions; real raw reopen observations remain explicitly unearned.**

## Scope and Completion

All three tasks completed within the12 declared implementation owners. Eight paths changed: six existing Rust test files plus the current165 contract data and inventory pin. All three old164 tooling files and the current165 executable test module remained byte-identical. No new Rust path, helper seam, production API, module, breadcrumb row, dependency or cap exception was introduced.

The executed plan is `ea23f17c02c6cc532e7efad9a7c62337bf4d0b971cffe32fc853068b31619763`. Before capture occurred2026-10-17T11:35:09.223714Z; complete matrix closure occurred2026-10-17T12:14:04.901Z. Root activated the independently checked28-plan graph before the execution freeze. This executor did not modify shared state, ROADMAP, requirements, hooks or Git. Existing base commit `d0104a997c0362bf65b81de1c8fec8ddc3012a1b` remains the commit reference; no task commit is claimed.

Repository/global instructions, Bright Builds guidance and relevant architecture/code-shape/verification/testing/Rust/TypeScript standards were loaded in this continuous session. Both active lesson inputs totaled7,188bytes/2,397 conservative tokens, without archives or an audit trigger. No external research or project skill was needed.

## Actual Fixture and Observation Behavior

Durable reconciliation now seeds only genuine active genesis on an absent path, retaining child and grandchild separately with ordinary save_block. Strict complete-prefix replay/adoption remains unchanged. The original connected_count2 and single captured final grandchild event both execute and pass. The already-passing direct-sync genesis initialization and duplicate-body/persistence assertions remain intact.

The at-tip diagnosis case and false-progress soak case use seed_genuine_fixture_history, validating actual active bodies and persisting their complete coins/positions/undo/count facts. Every distinct diagnosis/help string, previous useful-work assertion and header/download rejection remains. The evidence-only hint ending0002 is unchanged and remains rejected against actual earned active work. No message, header or downloaded body gains connected-chain credit.

The soak partial-sync sequence still performs real flush, drops the runtime, reopens the same store and requests the next missing body. Reopened admission is header-only, with explicit unearned assertions before and after request processing. Original best-header95/active18, hash, work ending0026, duplicate-GetData exclusion, next-missing19 and unchanged active tip remain asserted. The96-block long run, resource stop and original real cleanup remain intact.

Stay-current setup uses genuine complete active prefixes. Real fresh child acceptance, header-only no-credit, heartbeat/backoff, prior useful-work, fresh/stale tip and branch-competition assertions remain exact. The stronger B branch still has absent bodies and cannot receive connected credit. The persistence reopen retains actual drop/reopen and metadata comparisons, admits headers only and verifies unearned origin after the observation.

The restart matrix retains clean, unclean, mid-download, mid-connect and stale-inflight scenarios. Genuine seeds preserve the actual active prefix; the mid-download inactive child is retained separately. Observation/request owners use header-only admission with explicit unearned assertions. All original connected/downloaded identities, work, real drop/reopen, stale-inflight clearing, duplicate request exclusion and next-missing request assertions remain. No raw owner receives count enrichment, missing-data repair or reconstructed arrival order.

## Baselines, Compiler Recovery and Scoped Rust Evidence

The fresh baseline reproduced10pass/12fail across22 discovered cases, with zero ignored cases. The failures reached strict actual confirmation-count identity. Already-passing historical cases were retained; no artificial RED was created. Exact commands, exits, durations and hashes are in `.local/open-bitcoin-dev/phase165-p23-baseline-results.json`; its five original logs remain `.local/open-bitcoin-dev/phase165-p23-baseline-1.log`, `.local/open-bitcoin-dev/phase165-p23-baseline-2.log`, `.local/open-bitcoin-dev/phase165-p23-baseline-3.log`, `.local/open-bitcoin-dev/phase165-p23-baseline-4.log` and `.local/open-bitcoin-dev/phase165-p23-baseline-5.log`.

Two initial repair attempts stopped during compilation and produced no test results. The first new post-observation assertions preceded runtime declarations: `.local/open-bitcoin-dev/phase165-p23-green-1.log`, exit101/SHA `bd52294b6d790c0673c55e6d2772bf745161a4181ef85580ff644891650a6883`, E0425. After declaration placement was corrected, the soak assertion followed its existing drop: `.local/open-bitcoin-dev/phase165-p23-corrected-1.log`, exit101/SHA `809a5bf8981898963b942f923492cec51f01773f6113885ac83765a304ea81d7`, E0382. These are compiler diagnostics, not executed behavioral RED.

Execution paused under the two-attempt rule. Root inspected and approved the bounded recovery: move only the existing soak assertion immediately before the existing drop. Before packet `.local/open-bitcoin-dev/phase165-p23-scope-recovery-before.json` has SHA `63638c57ad4379e9f44698e4a6a63b457546aabbd2da450133e70c347a4758a5`. Exact six-source recovery proof `.local/open-bitcoin-dev/phase165-p23-recovery-scope-proof.json` has SHA `c9ac436f85f6c301f2b6ca94c78a5071cede10d5d9622b4b0f24c9f6d97c7fe0`. Lifecycle/cleanup order was preserved.

Before rerunning suites, timing-wrapped format check exited0 in1,486ms and Node all-target/all-feature cargo check exited0 in12,461ms. Receipts are `.local/open-bitcoin-dev/phase165-p23-recovery-native-gates.json`; check log `.local/open-bitcoin-dev/phase165-p23-recovery-check.log` has SHA `54adfe6fabf59e6c9d231ec466403c2be1e457db1087b356ec47e4735eceda18`.

All Cargo work used pinned Rust1.94.1 and Bun1.3.9 PATH through command-timings, serially, with one literal test filter. The final command is cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --all-features --lib followed by the complete filter below.

| Complete filter                    | Baseline pass/fail | Final pass/fail/ignored | Wrapper ms | Exact final log                                      | Log SHA-256                                                      |
| ---------------------------------- | ------------------ | ----------------------- | ---------- | ---------------------------------------------------- | ---------------------------------------------------------------- |
| sync::tests::durable_tip           | 1/1                | 2/0/0                   | 9689       | .local/open-bitcoin-dev/phase165-p23-recovered-1.log | af5c8a4017095b5b04830873a90979ecdbb9367ca863b072bd62dcdd750dc823 |
| sync::tests::no_progress_diagnosis | 6/1                | 7/0/0                   | 3424       | .local/open-bitcoin-dev/phase165-p23-recovered-2.log | 54586b7ae10b7090d6ec51fc72e56f5ee98fc1d317b0005f0b6d88c728e7ab51 |
| sync::tests::soak                  | 3/1                | 4/0/0                   | 6424       | .local/open-bitcoin-dev/phase165-p23-recovered-3.log | 31c2c82d560733e9cf3e7a2a6ebbab9d9bf3007143d577351be7a4d9287f9cbb |
| sync::tests::stay_current          | 0/8                | 8/0/0                   | 3707       | .local/open-bitcoin-dev/phase165-p23-recovered-4.log | 72d7a118597862bdc18b51b3b139c184a21c3d75e9bde75628b52f676ce7cb76 |
| sync::tests::restart_resume_matrix | 0/1                | 1/0/0                   | 3988       | .local/open-bitcoin-dev/phase165-p23-recovered-5.log | ffbbabc465496cf83067977fefe35c56f4f15cf9682c1ff195071f863d3cfc3f |

The final22/0/0 receipt envelope is `.local/open-bitcoin-dev/phase165-p23-recovered-results.json`. Required cargo fmt --manifest-path packages/Cargo.toml --all -- --check, strict cargo clippy --manifest-path packages/Cargo.toml -p open-bitcoin-node --all-targets --all-features -- -D warnings and cargo build --manifest-path packages/Cargo.toml -p open-bitcoin-node --all-targets --all-features all exited0. Wrapper times were1,543/6,470/320ms. Complete receipts/log hashes are `.local/open-bitcoin-dev/phase165-p23-native-gates.json`, `.local/open-bitcoin-dev/phase165-p23-fmt.log`, `.local/open-bitcoin-dev/phase165-p23-clippy.log` and `.local/open-bitcoin-dev/phase165-p23-build.log`.

## Independent Review and Exact Current Guard Renewal

Authentic before packet `.local/open-bitcoin-dev/phase165-p23-before.json` has SHA `27db84227d1da2780d21b452f11e81568ffeb40e728d66229473736ed213b5cc`. Frozen12-source/control/proposal packet `.local/open-bitcoin-dev/phase165-p23-prepin-review.json` has SHA `cfa7bd255dff3234e30ee7d54b0d83e23474e0061ef6e3828b06899c31a66b75`, ordered source digest `90dfe5708d54821d786b804135e8f011fc7ba8e1b7ebd90a4b10fc6651f1ce39`.

Independent clean review is recorded in `.planning/phases/165-shared-exact-work-authority/165-PRE-REVIEW-INTEGRATION23.md`. Root authenticated all12 full sources/caps and actual prior/proposed arrays before renewal. Ten exact loaded-source mutations reuse the existing loop: replace genuine seeds with raw seeds, omit actual inactive-body writes, and replace header-only raw observation with full adoption. Every selected function contains its real anchor exactly once, replacement changes source and the attributed owning work-owner finding is required against a passing baseline. Existing enabled-test and missing/extra/reordered/redirected inventory controls remain intact.

Only six SOURCE rows and six TEST rows2/7/4/2/6/1 were appended. Existing128 source rows and64 test rows remain exact; current totals are134/70. Inputs grow257→263 at indices244–249, preserving every prior relative input position. No owner belongs to old164's independent135-input/477-edge scope, so its three declared files remain unchanged.

After reviewed rows were applied with the old pin, `.local/open-bitcoin-dev/phase165-p23-stale-pin.log` produced actual exit1, fixed input inventory changed before any source read, SHA `23c1e64fed4753c2397625462c6823bedb6e76f5cd34fec20537cd874304a2da`. This is a pre-read diagnostic. Only the inventory literal then changed from `68896e7925de650b2cfa0371a517248b46dbb408de3cf533cfdd7870407204db` to `53b26c6f2cd457c1f2df85b22095f192de09b76ca2256c9a3eff9f89d3fd208a`.

Root reversed only12 appended rows and that literal to recover both complete pre-pin sources, with the other ten owners exact. Root proof `.local/open-bitcoin-dev/phase165-p23-root-pin-proof.json` has SHA `d33e840e3f216919dfcfe3acd1ea76c91c74e4e4d2838df5683552d644a12bb0`. Actual postpin source/arrays `.local/open-bitcoin-dev/phase165-p23-postpin-proof.json` have SHA `015858606818c86c87ae5351197c7df7b335a6f30065db83eb3d12a77f98d48b`, ordered12-owner digest `05af667c8560170ba770e1e014029ad4d23c61f1e4ab15803b58d0d00df0694c`.

Targeted controls passed51/0/215expectations in24.449s, SHA `68eefd192ac5a8421e5e27dffbc3e12c4a4daedd862dfa14ecb545d158957a46`, `.local/open-bitcoin-dev/phase165-p23-targeted-controls.log`. Current158/162 checkers passed; after root's final metadata update, only these cheap checkers were rerun with fresh logs, both exit0 in1.727/.916s. Complete receipts are `.local/open-bitcoin-dev/phase165-p23-targeted-results.json` and `.local/open-bitcoin-dev/phase165-p23-final-metadata-results.json`. The full165 run subsequently covers all targeted controls against the final frozen metadata.

## Source Identity, Caps and Simplification

| Changed implementation path                                           | Lines | Complete raw SHA-256                                             |
| --------------------------------------------------------------------- | ----- | ---------------------------------------------------------------- |
| packages/open-bitcoin-node/src/sync/tests/durable_tip.rs              | 96    | 4293c6da937a9df88a77532bcc1614d42a434f7a72f0960b9044d5d78fcc7286 |
| packages/open-bitcoin-node/src/sync/tests/no_progress_diagnosis.rs    | 265   | 4e1dfdd2bb5485c31217860499b5fb0e35010e0ca7009a12857c12e6b7cd465d |
| packages/open-bitcoin-node/src/sync/tests/soak.rs                     | 380   | a060815ad8c2d5cfc71dc4d42e9d4aee97b4ef18f41b545c250adeb418cbc76c |
| packages/open-bitcoin-node/src/sync/tests/stay_current_persistence.rs | 158   | 3d6840b8c85cc25b3d9b7e6cd3a4228ed0b363818d49d4b19b94d0df18606b2a |
| packages/open-bitcoin-node/src/sync/tests/stay_current_progress.rs    | 510   | 43322767cdea3f93b90ef685b6d8f2177c850a6ff533f3162153749288760cdf |
| packages/open-bitcoin-node/src/sync/tests/restart_resume_matrix.rs    | 240   | 7fb23c23a06243af9a39bfb45f0f0451b31b7a9ced9b5e201eb3552095030310 |
| scripts/check-phase165-shared-exact-work.ts                           | 266   | 68fb5efa600e55220e819fd8c578c13aac8e877996e6bcdd9c5eb29e874ba503 |
| scripts/check-phase165-shared-exact-work/contracts.ts                 | 590   | 160ff50aad2601a453e90a2d7c1cb610bf76fcad4d96cb7b846219a1a0ca1cdf |

The unchanged declared paths are `scripts/check-phase164-canonical-identity/contracts.ts`515lines, `scripts/check-phase164-canonical-identity/source-files.ts`137lines, `scripts/check-phase164-canonical-identity.test.ts`598lines and `scripts/check-phase165-shared-exact-work.test.ts`627lines. All twelve remain within628.

The explicit simplification pass reused existing genuine seed and header-only APIs, retained bodies in one small ordinary-write loop, and reused existing data-driven source/disabled-test controls. No new fixture abstraction or source module was needed. Seed-only versus raw-observation choices match the actual effects each test performs.

Protected `packages/open-bitcoin-node/src/sync/tests/support_runtime.rs` remains SHA `5a25857c552bb7dcfdd040ad15e19c4616f52ef8b35df69b4f11a42dcd5b1f05`. Protected `packages/open-bitcoin-node/src/storage/chain_compatibility/tests/preservation.rs` remains20,688bytes/SHA `e6cb5740cb5c83655aed14c43b1206847d264431d944741a365771c3e8d76f5b`, with original20,556bytes/SHA `3d004f91bd72a0f3902588a36fca0438b04b5ac034bc9e227893664496835630` attribution separate. Existing breadcrumbs cover all six Rust owners.

Original162/163 manifests/recipes/snapshots/receipts, working historical adapters/inverses, mixed bridge data, six mandatory verify lines and old independent removal controls remain unchanged. Evidence class is local source-derived fixture behavior, with ordinary selectors refused and networking default-off.

## Complete Matrix Execution and Raw Closure

The new collector `.local/open-bitcoin-dev/phase165-plan23-freeze.ts` preserves22's fixed raw lists, actual factory initializer/corpus, optional/canonical checks, modules, entries and critical helpers/children/support_blocks. Changes are23 receipt labels and the reviewed263 input identity. The serial driver `.local/open-bitcoin-dev/phase165-plan23-matrices.ts` changes only the unique receipt prefix.

Freeze review caught one copied scope label still saying Plan22. No matrix had run. The original prepared collector is preserved as `.local/open-bitcoin-dev/phase165-plan23-freeze-scope22-preparation.ts`, SHA `6b5c8b2969bb48ddcb8bef4decf65735f62d9480df544111d475621e40f76427`; its unexecuted capture is `.local/open-bitcoin-dev/phase165-plan23-full-freeze-unexecuted-scope22.json`, SHA `bf8ff25eb1387d56a121494f9b68659f05c442f789480bb9ecabc975a6efe4b5`. Root approved changing only that ignored scope literal and recapturing; no test rerun or implementation/read-set change followed.

The corrected collector has SHA `36cf570a067f28ae81b2692432fa97500f9ec680f7ce3bc49d14116e4383bd5e`; driver SHA `e39e453665bbc1ec2b2f944c54fbfaac651925d6074055800d9d011d4e698daf`. Root freeze proof `.local/open-bitcoin-dev/phase165-p23-root-freeze-proof.json` has SHA `b836d474b53b7354eaffeface3256eeae8d26fdf7a560674c412c8aa1020f43b`.

Approved pre-run `.local/open-bitcoin-dev/phase165-plan23-full-freeze.json` has SHA `5ba803a7ff16fcf32e2d44d4f194419aa163d276e6beff8726dabd3f5b1f977e`. Its490 canonical paths comprise488 present files and two permitted absences,36 support/test modules and14 CLI modules. This execution union is separate from current165's263 fixed inputs. It retains all earlier critical dependencies, all actual fixed loader/factory reads and complete original raw bytes.

| Full current matrix | Pass/fail | Expectations | Wall seconds  | Exact log                                                   | Log SHA-256                                                      |
| ------------------- | --------- | ------------ | ------------- | ----------------------------------------------------------- | ---------------------------------------------------------------- |
| 164                 | 868/0     | 1565         | 342.4164315   | .local/open-bitcoin-dev/phase165-plan23-matrix-164-full.log | f8fe6d1aa55901011f9207193485f3787c2c750d921d5aa620f2496624dc9df7 |
| 165                 | 1687/0    | 6059         | 795.731516958 | .local/open-bitcoin-dev/phase165-plan23-matrix-165-full.log | 98f7ae1c59bb245fbcb9a284c7a19073450d2b77393fa4b07fa2b71b3abd81ec |

Both live checkers, breadcrumbs, managed rules and whitespace exited0. Live164 reports135 fixed inputs/477 source edges; live165 reports134 reviewed owners/70 executable test-owner files. Breadcrumbs verify1,203 Rust files. Managed rules scan1,521files with zero cap exceptions/findings. Exact seven commands, exits, durations and all seven log hashes are in `.local/open-bitcoin-dev/phase165-plan23-matrix-matrix-results.json`, SHA `bdb502593be1bab83e649c052430dd5a9229e3d09d02d049f05bcba9e8ee927c`.

At2026-10-17T12:14:04.901Z, `.local/open-bitcoin-dev/phase165-plan23-full-freeze-closed.json`, SHA `7abbda043cbbb0449f393809e0c6a1708b52085f4f04ecf6f006477c01fc5059`, confirms all490 raw hashes/canonical paths/lengths/presence/fixed lists unchanged. Sessions were polled within60seconds and healthy jobs continued. No source or frozen document changed during execution.

Root independently rehashed all490 entries and all seven successful command logs. Its closing proof `.local/open-bitcoin-dev/phase165-p23-root-final-closure.json` has SHA `286f3b83907689dc05114b6abdd6fb18c9db78c187261ad66e914eb35c5c6c9a`.

The current derived168-file corpus is4,406,522bytes/SHA `eada61e40b5874c61175dc8d7741b8db75dc349877841abd5fde0ef25480a13f`. Its difference from22 is the root-activated ROADMAP counts before this freeze. Immutable historical snapshots and qualification receipts remain separate.

## Issues, Deviations and Remaining Coordinator Gates

All12 original failures now reach and pass their intended consumer assertions. The two compiler-only diagnostics required the documented bounded recovery; the copied receipt label required the documented preparation correction. No failure was relabeled as a behavioral pass, no test was disabled and no new scope was taken.

Stub/source review found no unfinished implementation or new production endpoint, filesystem authority, schema or unplanned trust surface. Intentional absent bodies, raw unearned owners and rejected hints are asserted test states. Pure/shell boundaries remain intact.

Plans28/24 and final14 remain coordinator-owned. Root retains complete repo-native scripts/verify.sh, workspace/coverage/Bazel/benchmark checks, whole-phase code/security/formal/lifecycle review, shared-state updates and normal-hook Git. Requirements are contributed here, not declared globally complete. Phases166/167/168 retain canonical bootstrap/exact all-handle recovery/genesis BASIC, contextual completeness and actual pinned-Knots qualification. This plan claims no ordinary support, whole-phase acceptance or later-phase execution.

## Self-Check: PASSED

All three tasks are complete. The Summary and eight changed implementation owners exist within the twelve declared paths. All22 Rust cases, strict native gates, focused controls, final-metadata158/162 checkers, both complete current matrices, both live checkers, breadcrumbs, managed rules and whitespace passed. All490 canonical entries retain their authenticated raw bytes, hashes and presence states. Independent source/data review and exact pin reversal are recorded; all Rust/TS caps remain within628. Scoped GFM/frontmatter and summary parser checks passed. Git finalization remains coordinator-owned.
