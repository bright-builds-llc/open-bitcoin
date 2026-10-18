---
phase: 165-shared-exact-work-authority
plans_checked: ["17", "18", "19", "20"]
generated_by: gsd-plan-checker
lifecycle_mode: yolo
phase_lifecycle_id: 165-2026-10-14T16-09-59Z
generated_at: "2026-10-17T03:09:37Z"
status: passed
---

## PLAN CHECK PASSED

Four plans checked statically against actual source and the completed Plan 14 diagnostic. No concrete blocker or required plan revision found in Plans 17–20. This result assumes the separately checked Plan 15 source contract; it does not approve that API independently or authorize execution before predecessor approval.

### Checked identities and scope

| Plan | SHA-256 | Tasks | Declared paths | Dependency / wave |
| --- | --- | --- | --- | --- |
| 165-17 | `e9b439d398cead54558d515a871d0cb045a4099268b8c65ec2c3bd768358fbb7` | 3 | 14 | 165-16 / 16 |
| 165-18 | `db238eaaf405f089e054d29d081913bc35d14ffd8de4ce832772f9db09a06654` | 3 | 14 | 165-17 / 17 |
| 165-19 | `54028619f1be2a21591ce17b2eb030cec6e1a857cc54f49e6733c7c2af071c19` | 3 | 12 | 165-18 / 18 |
| 165-20 | `6adf29ca196a87772b96847e4323282cd299b794b0e19e8060c740915a9a12b5` | 3 | 12 | 165-19 / 19 |

Plan 15 provisional API identity: `ac9382e108f160e43bfc1bde409c70d1905d41d82a7a889a87adb30b7499c64a`. Its full-prefix independent replay, complete physical identity and before/after normal BASIC comparisons are dependencies of this result. A revision changing that contract requires dependent rechecking. Plan 16 is the direct predecessor of Plan 17; these plans do not create an alternative entry around it.

Each plan reserves the same five exact guard paths. Source/helper/breadcrumb ownership is respectively 9/9/7/7 paths; each implementation task owns at most five paths. All plans remain below fifteen declared paths, with no task owning ten files. The large repeated guard inventory is sequential maintenance rather than additional independent feature scope. Plans 19/20 explicitly count their new helper and breadcrumb map. Plan 17 intentionally permits no new helper/breadcrumb-map path. Any extraction beyond these inventories requires a bounded ownership amendment first.

### Evidence loaded

The repo AGENTS.md and Bright Builds sidecar, placeholder-only standards overrides, standards index and architecture/code-shape/verification/testing/Rust/TypeScript guidance materially informed this review. Relevant active lessons were read completely: global 5,230 bytes plus repo 1,958 bytes = 7,188 bytes / 2,397 conservative estimated tokens; no archives were loaded. No project skill directory was found. Context D-01–D-12, roadmap/requirements/project constraints, relevant Plans 07–09 source and summary contracts, Plan 14 acceptance/diagnostic evidence and the four full plans were inspected.

The actual companion ledger is `.local/open-bitcoin-dev/phase165-plan14-failure-owner-map.json`; the abbreviated `failure-owner-map.json` path is not present. The canonical evidence identities match acceptance:

| Input | SHA-256 |
| --- | --- |
| Plan 14 diagnostic log | `1b35b73cf31cde9d4bcfeafe9944378a216aaa2afdf79b9d8fe7d7abf0aef00d` |
| Failure roots | `c4da4acfa59a0ff801080791dc459571eeb84011fe8bdb1bbb4dcd27a37e7d8f` |
| Failure owner map | `4e29732194d643fdb8f4e7d59a51cf3c579e670ff76c9a1d7ce91475d5b944fc` |
| Strict support_runtime.rs | `5a25857c552bb7dcfdd040ad15e19c4616f52ef8b35df69b4f11a42dcd5b1f05` |

The initial diagnostic remains exit 101, 4,461 passed / 249 failed / six pre-existing ignored, 1,024,509ms (17m4.509s), mapped to forty-six panic clusters. This report neither reruns nor replaces it. Shared source panics are evaluated as fixture prerequisites, not 249 independent product bugs.

### Concrete plan findings

**Plan 17:** The branch fixture currently constructs an untrusted empty coins cache; genuinely empty consuming transfer addresses its actual failure. Transport's live extension needs a locally validated history instead of the codec-only query recipe. Accepted/query/readiness cases need an actual same-Fjall old prefix; history.seed(1) also persists future undo, so old-prefix replay cannot simply pass strict complete adoption. The plan addresses the owning setup and explicitly forbids relaxing support_runtime, copying future undo or deleting evidence. Actual accepted-fault height1-versus2 and UnknownLegacy-versus-Pending failures must reach the original acceptance/persistence boundary after repair. Requested-body tests retain header roots, real getdata and actual first-error classifications. The approved storage lock caller switches only to existing reopen_live_fixture; generic reopen/stage remain unchanged. The plan adds the missing before-count-write physical undo comparison without claiming that counts or raw reopen earn authority.

**Plan 18:** rpc_faults::genuine currently closes at the old fence and then connects a successor on raw reopened ownership. Keeping the actual original owner through normal BASIC assembly resolves that premise while retaining separate raw observations. Arming undo/coins/meta/query/publication faults after genuine setup preserves their intended consumption. Retention still requires old durable19 versus processed400 and actual paired-delete refusal; no logical-only substitute is permitted. Header checkpoint setup keeps actual snapshot counters, initialized zero-cache pressure and unconsumed faults, with normal maturity100 parameters for stale history. The plan correctly avoids re-admission on already bound headers and does not broaden shared runtime_state ownership.

**Plan 19:** The actual Node and RPC recipes attempt their first genesis acceptance on raw runtime cores and close/reopen after genesis. The planned helper performs real genesis acceptance and real same-source flush before normal constructor assembly, retains the actual genesis receipt, then accepts the unchanged mature suffix. All selected Regtest parameters, maturity100, flags, raw recipes, IDs and actual oracle/socket assertions remain explicit. RPC's public helper stays inside recognized tests/support and is included only from existing test modules; no cfg-test Node authority API becomes public. Existing 627/628-line adapters can replace duplicated setup through that counted helper. Daemon handoffs remain values only, all handles close, raw observers remain untrusted and the old ignored long measurement stays unchanged. Any executed later live mutation requiring recovery must be replanned rather than granted authority by this helper.

**Plan 20:** Large-prefix replay uses actual original maturity1 bodies before staging the legal 40,000-output block; snapshots are derived evidence. Shared serving-only reopens switch to header-only admission, preserving unearned core work and physical missing/corrupt inputs. Already genuine ForkFixture consumers remove redundant header-owner replacement. The explicit compact_with_early_branch(16,8,24,15,8,0x76) lifecycle earns the exact side bodies before A16..23 on the same actual network, retains selected parameters and complete identity, and attaches BASIC through the existing cfg-test seam. It does not infer arrival from saved bodies or make ordinary compact universally early. The readiness replacement really ends at23 after active24; the approved count13→15 adjustment ends at25 and preserves the pending invalidation/wake/latch/undo intent. The separate operator test remains equal-height and retains its 32-record assertion. Existing compact_early supplies a checked lifecycle model; reuse that behavior where feasible rather than inventing a second authority path.

### Dependencies, coverage and executable gates

The dependency chain 16→17→18→19→20 and waves16–19 are valid and acyclic. Shared guard/breadcrumb edits are serialized; shared operator_status ownership is explicitly Plan18 successor setup followed by Plan20 equal-height premise. No incompatible data transformation or loss of raw source bytes is planned.

All twelve tasks have files, concrete action, automated verify and done fields; gsd-tools plan-structure returned valid with zero errors/warnings for all four plans. All Cargo commands select actual lib/bin targets, pin Bun PATH and use the timing wrapper. Pipefail/tee plus nonzero successful libtest checks prevent a zero-discovery GREEN. Commands are sequential; heavy guard matrices cannot overlap Cargo/Fjall. Tests retain enabled status and real effect assertions. Future summaries must record exact discovered/pass/fail/ignored counts, exit, duration, logs and any newly reached failure rather than treating compiler success as behavioral evidence.

Plan17's exact top-level readiness/interleavings filters exclude the D-owned reorg-pending case. Plan18's direct-turn/failures/RPC filters and exact status-successor filter avoid claiming the pending equal-height status/readiness cases. Plan20 explicitly closes full reorg/operator_status/readiness/catch_up/peer/transport/write-evidence/candidate suites after Plans17–19. Static mapping of the existing failed cases found no required later-plan repair in the earlier focused filters. Shared consumers remain tested through their actual source owner, including the 29 unchanged peer consumers; no skip/ignore is introduced.

| Requirement | Coverage relevant to these repairs |
| --- | --- |
| CWOR-01 | Existing arithmetic plans/corpus preserved; no second implementation or omitted phase requirement inferred from this four-plan subset. |
| CWOR-02 | Plan20 real equal-body eligibility and selected-parameter header admission; Plans17–19 preserve existing selectors. |
| CWOR-03 | Plans17–20 genuine source setup, actual accepted facts, strict source/owner/generation and physical identity before effects. |
| CWOR-05 | Every plan preserves pre-mutation ordinary refusal, historical evidence and immutable default verification registrations. |

D-01–D-04 remain inherited unchanged; D-05–D-08 are respected by actual same-source and explicit early-body setup; D-09–D-11 remain enforced through unchanged ordinary refusals and guard renewal restrictions. D-12's deferred canonical bootstrap/all-handle exact recovery/genesis BASIC/context/actual pinned-daemon outcomes are excluded. There is no silent simplified scope or new Mechanical managed exception.

Current source renewal is bounded to actually reviewed changed/new owners, real enabled test ownership, exact ordered inputs and actual loaded-source mutations with unique present anchors and attributed findings. Old164 independent controls, all-six-removal detection, historical162/163 identities and receipt bytes remain immutable. Source hashes and mutation checks are source evidence, not runtime acceptance. The actual 32 local Open Bitcoin refusal children remain distinct from pinned-Knots qualification.

Dimension 8: SKIPPED (workflow.nyquist_validation is false). Research open questions carry explicit RESOLVED decisions. Repo conventions, parity mapping, cap628, normal verification/hook requirements and parent-owned sync/bootstrap are accounted for.

### Remaining execution limits

No application, builds, tests, native verifier, coverage, Bazel, review/security/formal or Git operations were run by this checker. Only static reads, structural plan validation and this report write were performed. No implementation, plan, tracker, shared state or historical receipt was edited. File caps and actual adapter compilation remain prospective checks; unforeseen required source expansion or failed preserved assertions require concrete replan evidence.

The parent must independently approve Plan15 and its prerequisites before this chain executes. Final Plan14 closure remains after Plan24 and must include the default bash scripts/verify.sh, actual native coverage policy, full review/security/formal checks, normal hooks and push. Phase166 remains closed; focused GREEN cannot substitute for that final gate.

```yaml
issues: []
```

## ISSUES FOUND

### Contract refresh | 2026-10-17T03:24:31Z

This appended review supersedes the initial conditional PASS for execution against the revised Plan15 contract. The initial checked hashes, source observations and pass attribution above are preserved. The root reports a scoped mdformat pass; current Plans17–20 retain the originally reviewed task/dependency/ownership/test-filter semantics, with formatter changes including XML-block indentation, escaped prose angle/comparison characters, blank lines and table alignment. This refresh is a semantic review of those current bytes against the NEW upstream contract, not a replacement of the original recorded hashes.

Generated by: `gsd-plan-checker`; lifecycle: `yolo`; phase lifecycle ID: `165-2026-10-14T16-09-59Z`; actual checked UTC: `2026-10-17T03:24:31Z`.

| Current checked plan | SHA-256 |
| --- | --- |
| 165-15 | `de54b38f05b8dd0a4cca02d100ff36c884ea823710017a7c19d6b07778de63a3` |
| 165-16 | `23457ba216ffca1733c82f7e8b6b91ca4c153ffb235a018a9603a5025ab425d2` |
| 165-17 | `b4c22b50efa61df48a7cd40163f455bf61afd670b452abc505eb14e14fced678` |
| 165-18 | `4e214f8521c09f170f6c805e462839fae16c31b30a4ccba4f2e38b9550a032d5` |
| 165-19 | `dc3c14985c362f9c3195479e7f5fcae58080488820b7cffdac7ecb6c3a4bdea8` |
| 165-20 | `7a6facec6de6c827492883b95432749912b7ae7f3ab90a684e60f1a3b8c5bd4f` |

**Current result: one cross-plan data-contract blocker requiring a narrow plan amendment before execution.** No implementation/build/test result is asserted. Plan15 remains subject to its separate independent approval.

Final Plan15 now constructs the populated source header/body graph internally from an independently validated, captured complete active prefix. It binds the actual header owner once, checks the unique root linkage and imports historical NONroot bodies through cached sticky-Unknown order, without an accepted-tip body-preference binding. After normal BASIC/source-graph assembly it checks the same complete captured expected coins/B/heads/positions/undo/count/body/validation identity again. Complete independent replay also defeats the same-Fjall alias-poisoning hazard. Those changes are compatible with genuine strict extensions in Plans17–19; they do not qualify historical equal-work order.

A genuine EMPTY source with BASIC Disabled still has no header owner/root: that caller may explicitly admit its root once before any P2P flow. Every populated Plan15 consumer already has root+active-prefix graph and MUST NOT invoke try_admit_fixture_headers, admit_fixture_runtime_root, admit_fixture_runtime_headers or admit_fixture_runtime_headers_only on that owner. Needed NEW NONroot headers must use existing normal managed ingress, contiguous batches with an actually accepted parent and the full same selected parameters. A P2P Headers message must not contain the null-parent model root. Existing local body connection may validate its actual new header when the accepted parent graph already exists.

**Blocker:** Plan17 Task1 at current line150 explicitly says requested-block cases must admit the actual named root and selected successor AFTER building their populated two-block Plan15 prefix. Plan18 Task1 at current line148 likewise tells requested-body cases to admit named root/successor headers after populated construction. Rebinding that owner now refuses before the intended BODY/UNDO/COINS/META failure/getdata observation; sending root via P2P also does not fix it. The shared interfaces in all four plans still say only that later bodies need admitted model-root headers and omit the new empty-versus-populated distinction. Their existing generic warning against re-admission does not resolve the contradictory task instruction.

Minimal revision, within the existing ownership/file budgets:

1. In Plans17–20 shared interfaces, state the complete final15 source-graph contract above, including populated already-bound root, historical nonroot sticky Unknown, no accepted preference, and genuine empty Disabled root admission exactly once.
2. In Plan17 accepted.rs requested_failure and Plan18 evidence/accepted_faults requested-body setup, assert/read the already captured actual root+old active-prefix graph. Remove the instruction to admit that root again. Admit only any missing future successor through normal managed protocol ingress or the existing scripted successor Headers, with accepted parent/full params. Keep the future BODY absent until its actual request and the fault armed only after genuine setup; retain actual getdata, exact accepted-prefix/error classification, old cursor and physical failure assertions. No additional file is needed.
3. In Plan19 Task1 and the shared RPC helper/callers, replace the ambiguous phrase “selected fixture header admission before each required body flow” with explicit normal NONroot ingress/local connect after the genesis-only constructor. Preserve actual returned genesis receipt. No helper may rebind/admit root a second time. The new constructor itself supplies the formerly missing first-ancestor body linkage, so no duplicate genesis acceptance is needed.
4. In Plan20 Task2, clarify that initialize_fixture_runtime_genesis owns the one initial root binding on its fresh empty network; subsequent full active/fork headers are NORMAL contiguous descendant Headers batches omitting root, as in existing compact_early. Its raw serving-only open/sparse/reopen paths remain a separate unbound-header case and may still use one header-only admission. Do not broadly remove those legitimate raw serving admissions or change strict support_runtime.

Plan20's actual early-body mechanism remains feasible: either the existing fresh same-network lifecycle or a genesis-only populated source can admit NEW side B bodies before NEW competing A children; unique root linkage permits genuinely earned new order. A full historical populated source cannot recover old equal-work preference. The plan already excludes that shortcut and retains the exact operator compact_with_early_branch(16,8,24,15,8,0x76), unchanged equal-height assertions and approved stronger readiness replacement. Normal compact/standard history must retain Unknown where appropriate. Plans17/18's precise positive filters and Plan20's deferred full-suite closure remain valid and unchanged.

No new source/header/cache API, second validator, raw snapshot promotion, count writer, future-undo import, missing/pruned-input repair, Mechanical managed exception or all-handle recovery is needed for this amendment. Faults must continue to reach their original effects after setup. Source guard renewal must include actual changed admission tokens/call ownership and preserve complete ordered inventory/digest and enabled-test checks; neither formatting nor this review is runtime evidence.

```yaml
issues:
  - plan: "165-17,165-18"
    dimension: cross_plan_data_contracts
    severity: blocker
    task: 1
    description: "Requested-body tasks instruct named-root admission after populated Plan15 assembly, but final Plan15 already binds that header source graph once; re-admission refuses before the intended measured operation."
    fix_hint: "Amend shared17–20 interfaces and exact17/18 requested callers: inspect captured root/prefix, send only missing new nonroot descendant headers through normal managed ingress with full params; clarify19 successor ingress and20 once-only root lifecycle without changing legitimate raw serving admissions."
```

All initial scope/requirement/dependency/strict-source/historical-evidence findings otherwise stand. Only this report was appended; no plans, source, historical evidence, tracker, state or Git were changed and no builds/tests ran. Final default-native/coverage/full review/security/formal/normal-hook/push closure remains parent-owned after Plan24. Phase166 remains closed. Recheck the narrow amendment and Plan15 approval before execution.

## PLAN CHECK PASSED

### Revised-contract recheck | 2026-10-17T03:27:40Z

**Current verdict: PASS; zero blockers, zero warnings.** This appended verdict closes the preceding cross-plan data-contract issue for the exact amended plan bytes below. The initial conditional PASS and subsequent ISSUES FOUND sections remain historical attribution; they have not been overwritten or silently rehashed.

Generated by `gsd-plan-checker`, lifecycle `yolo`, phase lifecycle ID `165-2026-10-14T16-09-59Z`, actual rechecked UTC `2026-10-17T03:27:40Z`.

| Current checked plan | SHA-256 | Tasks / declared files | Dependency / wave |
| --- | --- | --- | --- |
| 165-17 | `13152093fcb4fb6b2f3b9e268f19e20dc563654c7117e4c68ba67c0307eb7860` | 3 / 14 | 165-16 / 16 |
| 165-18 | `e1720a8977b52c40ea649ffdabf61874f93e82046785c97b1d3b3a49c4f7b25c` | 3 / 14 | 165-17 / 17 |
| 165-19 | `86364a17eabe1210b98976e225b48957a2e96dd99f9450681d2d96252e7dae64` | 3 / 12 | 165-18 / 18 |
| 165-20 | `32f7708a505f1b906598973084bc46325a94a8d4942e1a778951e93509f3e37e` | 3 / 12 | 165-19 / 19 |

Upstream contracts checked against Plan15 `de54b38f05b8dd0a4cca02d100ff36c884ea823710017a7c19d6b07778de63a3` and Plan16 `23457ba216ffca1733c82f7e8b6b91ca4c153ffb235a018a9603a5025ab425d2`. The parent reports their independent static checks and the upstream ledger now pass. This verdict still requires **actual Plan15 and Plan16 execution GREEN before dependent execution**; static plan approval and partially written implementation do not satisfy that dependency.

The amended common interfaces in all four plans now explicitly distinguish genuine EMPTY/unbound Disabled sources from populated/genesis sources with an already-bound captured root/active header-body graph. They preserve the checked unique root, cached historical nonroot sticky UNKNOWN order and absence of accepted-tip preference. They forbid root re-admission, root P2P Headers, raw header writes and invented historical order. New descendants require normal managed Headers ingress, known accepted parent, full selected params and contiguous ancestry, with diverging side branches in separate parent-anchored batches.

- Plan17 Task1 now asserts/reads the captured actual old-prefix graph, explicitly prohibits a second try_admit_fixture_headers call, retains real getdata/body intake and arms faults after genuine setup. No source helper or file ownership was broadened.
- Plan18 Task1 applies the same exact distinction to its requested-body callers, keeps future bodies absent until requested, and preserves actual pending/error/physical effects. Its header-checkpoint source paths continue to prohibit re-admission after binding.
- Plan19 Task1 now defines “selected fixture header admission” precisely as normal NEW nonroot managed ingress on the already-bound genesis graph. Its test-only RPC helper inherits that explicit common contract and returns the real original genesis receipt. Normal mature recipe bytes, public-only source assembly, physical socket assertions and handle-free raw observations remain unchanged.
- Plan20 Task2 now treats initialize_fixture_runtime_genesis as the single root-graph owner. Original and pending fork headers omit root and enter separate contiguous normal Headers batches after their parent is known; actual pending B bodies precede competing NEW A suffix body events on the SAME network. Historical nonroot Unknown still cannot grant an old equal-work tie. The distinct raw serving-only open/sparse/reopen cases retain their legitimate one-time header-only admission. The exact equal-height operator caller and approved genuinely stronger readiness replacement are unchanged.

The source/count/undo/physical identity, selected-parameter, same-core/BASIC, historical evidence and no-repair constraints from both preceding reviews still apply. Independent full-prefix replay and captured before/after source identity remain Plan15's production constructor contract; consumer adapters may not substitute aliased source==physical equality, future undo imports, missing-body restoration, count writes or a Mechanical managed exception. Strict support_runtime stays outside edit ownership.

Scope and dependency/test contracts are unchanged:14/14/12/12 declared files including the five exact guard paths;3 tasks each;16→17→18→19→20 and waves16–19; sequential shared operator_status/guard/breadcrumb ownership. Plans17/18 keep their precise positive readiness/status filters and explicitly defer the D-owned cases. Plan20 still closes full readiness/operator_status/catch_up/peer/transport/write-evidence/candidate suites. No broad suite is newly claimed GREEN. All four independent gsd-tools plan-structure checks again returned valid with zero errors/warnings and complete task fields.

```yaml
issues: []
resolved_issues:
  - dimension: cross_plan_data_contracts
    plans: ["165-17", "165-18", "165-19", "165-20"]
    resolution: "Populated graph is inspected, never rebound; only new nonroot headers use normal contiguous managed ingress, with the empty-source exception explicit."
```

This is a plan-contract recheck only. No partially written Plan15 implementation was reviewed or approved, no application/build/tests/native verification ran, and only this report was appended. Parent retains final Plan14 closure after Plan24: default native/coverage, full review/security/formal checks, normal hooks and push. Phase166 remains closed.
