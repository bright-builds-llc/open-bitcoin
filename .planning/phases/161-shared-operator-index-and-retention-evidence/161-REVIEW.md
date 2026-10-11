---
phase: 161-shared-operator-index-and-retention-evidence
reviewed: 2026-10-11T08:37:50Z
re_reviewed: 2026-10-11T10:17:57Z
depth: standard
review_kind: full_phase_changed_source
diff_base: a7963f943de6a779b6b1dcafd0f42bdd6ded9066
generated_by: gsd-code-reviewer
lifecycle_mode: yolo
phase_lifecycle_id: 161-2026-10-11T04-48-29
generated_at: "2026-10-11T10:17:57Z"
files_reviewed: 113
files_reviewed_list:
  - README.md
  - docs/architecture/operator-observability.md
  - docs/architecture/status-snapshot.md
  - docs/metrics/lines-of-code.md
  - docs/operator/runtime-guide.md
  - docs/parity/catalog/basic-compact-filters.md
  - docs/parity/index.json
  - docs/parity/source-breadcrumbs.json
  - packages/README.md
  - packages/open-bitcoin-bench/src/cases/operator_runtime.rs
  - packages/open-bitcoin-cli/src/operator/dashboard/app.rs
  - packages/open-bitcoin-cli/src/operator/dashboard/app/tests.rs
  - packages/open-bitcoin-cli/src/operator/dashboard/app/tests/basic_index.rs
  - packages/open-bitcoin-cli/src/operator/dashboard/model.rs
  - packages/open-bitcoin-cli/src/operator/dashboard/model/basic_index_section.rs
  - packages/open-bitcoin-cli/src/operator/dashboard/model/tests.rs
  - packages/open-bitcoin-cli/src/operator/dashboard/model/tests/basic_index.rs
  - packages/open-bitcoin-cli/src/operator/dashboard/model/tests/projection.rs
  - packages/open-bitcoin-cli/src/operator/dashboard/model/tests/prune.rs
  - packages/open-bitcoin-cli/src/operator/soak/tests/runtime/runtime_fixtures.rs
  - packages/open-bitcoin-cli/src/operator/status.rs
  - packages/open-bitcoin-cli/src/operator/status/render.rs
  - packages/open-bitcoin-cli/src/operator/status/render/basic_index.rs
  - packages/open-bitcoin-cli/src/operator/status/render/tests.rs
  - packages/open-bitcoin-cli/src/operator/status/render/tests/basic_index.rs
  - packages/open-bitcoin-cli/src/operator/status/tests.rs
  - packages/open-bitcoin-cli/src/operator/status/tests/basic_index.rs
  - packages/open-bitcoin-cli/src/operator/status/tests/mempool_policy.rs
  - packages/open-bitcoin-cli/src/operator/status/tests/rendering_and_service.rs
  - packages/open-bitcoin-cli/src/operator/status/tests/snapshot.rs
  - packages/open-bitcoin-cli/src/operator/status/tests/status_inputs.rs
  - packages/open-bitcoin-cli/src/operator/support/redaction.rs
  - packages/open-bitcoin-cli/src/operator/support/redaction/basic_index.rs
  - packages/open-bitcoin-cli/src/operator/support/render.rs
  - packages/open-bitcoin-cli/src/operator/support/render/basic_index.rs
  - packages/open-bitcoin-cli/src/operator/support/render/inbound.rs
  - packages/open-bitcoin-cli/src/operator/support/tests.rs
  - packages/open-bitcoin-cli/src/operator/support/tests/basic_index.rs
  - packages/open-bitcoin-cli/src/operator/support/tests/recovery_progress_inbound.rs
  - packages/open-bitcoin-cli/src/operator/support/tests/sync_fixtures.rs
  - packages/open-bitcoin-cli/tests/operator_flows.rs
  - packages/open-bitcoin-cli/tests/operator_flows/basic_index.rs
  - packages/open-bitcoin-cli/tests/operator_flows/basic_index/captured.rs
  - packages/open-bitcoin-cli/tests/operator_flows/domain_fixtures.rs
  - packages/open-bitcoin-cli/tests/operator_flows/rpc_harness.rs
  - packages/open-bitcoin-network/src/filter_serving/capability.rs
  - packages/open-bitcoin-network/src/lib.rs
  - packages/open-bitcoin-network/src/peer.rs
  - packages/open-bitcoin-network/src/peer/basic_operator.rs
  - packages/open-bitcoin-network/src/peer/inbound_state.rs
  - packages/open-bitcoin-network/src/peer/inventory_state.rs
  - packages/open-bitcoin-network/src/peer/message_dispatch.rs
  - packages/open-bitcoin-node/src/network.rs
  - packages/open-bitcoin-node/src/network/limited_serve.rs
  - packages/open-bitcoin-node/src/network/operator_snapshot.rs
  - packages/open-bitcoin-node/src/network/operator_snapshot/tests.rs
  - packages/open-bitcoin-node/src/network/peer_network_clone.rs
  - packages/open-bitcoin-node/src/network/relay_serving.rs
  - packages/open-bitcoin-node/src/network/runtime_authority.rs
  - packages/open-bitcoin-node/src/network/runtime_authority/filter_index.rs
  - packages/open-bitcoin-node/src/network/runtime_authority/filter_index/serving/tests.rs
  - packages/open-bitcoin-node/src/network/tests/filter_capability.rs
  - packages/open-bitcoin-node/src/network/types.rs
  - packages/open-bitcoin-node/src/status.rs
  - packages/open-bitcoin-node/src/status/basic_index.rs
  - packages/open-bitcoin-node/src/status/tests.rs
  - packages/open-bitcoin-node/src/status/tests/snapshot_projection.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/filters.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/filters/append.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/filters/lifecycle.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/filters/ownership.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/filters/publication.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/filters/publication/epoch.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/filters/query.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/filters/query/peer.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/filters/query/peer/tests.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/filters/query/tests.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/filters/retention.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/filters/startup.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/filters/tests.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/filters/tests/retention.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/filters/tests/retention/publication.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/filters/tests/retention/recovery.rs
  - packages/open-bitcoin-node/src/sync/open_runtime.rs
  - packages/open-bitcoin-node/src/sync/tests/filter_index.rs
  - packages/open-bitcoin-node/src/sync/tests/filter_index/catch_up/rpc_faults.rs
  - packages/open-bitcoin-node/src/sync/tests/filter_index/catch_up/rpc_faults/captures.rs
  - packages/open-bitcoin-node/src/sync/tests/filter_index/operator_status.rs
  - packages/open-bitcoin-node/src/sync/tests/filter_index/operator_status/captures.rs
  - packages/open-bitcoin-node/testdata/basic_operator_captures.json
  - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index.rs
  - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/operator_evidence.rs
  - packages/open-bitcoin-rpc/src/context/inbound_status.rs
  - packages/open-bitcoin-rpc/src/dispatch/node.rs
  - packages/open-bitcoin-rpc/src/dispatch/tests/network_status_schema.rs
  - packages/open-bitcoin-rpc/src/method/node.rs
  - packages/open-bitcoin-rpc/tests/black_box_parity/phase127_composition.rs
  - scripts/check-phase127-authoritative-network-state-unification.test.ts
  - scripts/check-phase127-authoritative-network-state-unification.ts
  - scripts/check-phase155-filter-index.test.ts
  - scripts/check-phase155-filter-index.ts
  - scripts/check-phase156-prune-coordination.test.ts
  - scripts/check-phase156-prune-coordination.ts
  - scripts/check-phase156-prune-coordination/contracts.ts
  - scripts/check-phase157-index-catch-up.test.ts
  - scripts/check-phase157-index-catch-up.ts
  - scripts/check-phase157-index-catch-up/contracts.ts
  - scripts/check-phase159-filter-rpcs.test.ts
  - scripts/check-phase159-filter-rpcs.ts
  - scripts/check-phase159-filter-rpcs/contracts.ts
  - scripts/check-phase161-operator-index-evidence.test.ts
  - scripts/check-phase161-operator-index-evidence.ts
  - scripts/verify.sh
findings:
  critical: 0
  warning: 0
  info: 0
  total: 0
status: clean
resolved_findings: [WR-02, WR-03]
---

# Phase 161: Code Review Report

**Reviewed:** 2026-10-11T08:37:50Z
**Depth:** standard, with focused cross-file authority and caller tracing
**Files Reviewed:** 113
**Status:** clean — no open findings in the final reviewed source

## Summary

The complete explicit scope was reconciled against `/tmp/phase161-review-scope.json`: 113 distinct existing paths, none ignored or missing. This includes the original 99 implementation/test/fixture/historical-guard paths, all ten final Plan 09 documentation/checker/verifier paths, the generated LOC report, the later corrected existing RPC black-box schema consumer, and its two existing Phase 127 guard/test paths. Scope follows the actual phase changes relative to the supplied baseline, including later authorized remediation and module splits, rather than only SUMMARY file lists. Planning artifacts and root trackers are contextual inputs, not source scope.

All reviewed files meet quality standards. No issues found in the final reviewed source. The two actionable findings discovered during this full review were corrected and independently re-reviewed before this report. The earlier supplemental core review was contextual evidence; it was not used as blanket full-phase approval.

The final Plan 09 source release, its `161-09-SOURCE-HANDOFF.md`, final Plan 11 correction, historical guard correction and root-owned UAT were inspected before closure. Plan 09 Task 3, its eventual final SUMMARY, full default native verification, final security/formal lifecycle decisions and git finalization remain root-owned. This report approves the reviewed source; it does not claim those later gates passed.

## Resolved Findings

### WR-02: Failed direct disable retained healthy read and growth trust — closed

**Severity:** Warning\
**File:** `/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-node/src/storage/fjall_store/filters/lifecycle.rs:50`\
**Affected consumers:** `filters/retention.rs:166`, `filters/query.rs:120`, managed host disable and operator status.

**Issue:** Starting from a healthy, measured forest, direct disable advanced the epoch and invalidated append proof before fallible owner/forest/checkpoint validation. Those operations preserved `read_integrity` and the complete retained aggregate. Undiscovered corruption could make disable fail while the old total remained available and a new read epoch could still be issued. Earlier host-failure tests began with an already-latched read fault and missed this starting state.

**Fix:** The guarded disable now invokes `control.invalidate_read_integrity()?` before `invalidate_append` and every fallible loader. This uses the existing helper to revoke trust and totals while preserving the same single epoch advance. Successful complete existing paths restore trust; failure cannot expose the previous complete aggregate. No additional scan, persisted field or authority was added.

**Closure evidence:** The new storage test at `filters/tests/retention/recovery.rs:9` covers eight direct/configured forest, projection, owner and backend-failure scenarios beginning healthy, and checks unavailable growth, denied new authority, invalid old tokens, no fabricated query-fault category, exact epoch changes and existing scan counts. The genuine accepted/Fjall managed test at `network/runtime_authority/filter_index/serving/tests.rs:20` verifies the actual host-to-status path. The executor records one discovered failing RED followed by two passing GREEN tests and 74 passing related regressions; this reviewer inspected their source and success/failure timing receipts. Both historical guards now require revocation and explicitly reject substitution with the previous epoch-only call. The root-executed combined guard suite records 273 passing tests, 544 assertions and zero failures. Final relevant live guards also passed independently in this review.

### WR-03: Published terminal test command selected an empty binary target — closed

**Severity:** Warning\
**File:** `/Users/peterryszkiewicz/Repos/open-bitcoin/docs/parity/catalog/basic-compact-filters.md:870`

**Issue:** The draft reproduction command selected `--bin open-bitcoin` for `phase161_basic_dashboard_cells`, although these tests belong to the library operator module. That command would discover no matching tests and could falsely appear successful.

**Fix:** The released command selects `--lib`. Module ownership was checked against `src/lib.rs`, the binary entrypoint and the actual test declaration. The terminal selector and requirement for nonzero discovery now agree. The frozen documentation also correctly describes human dashboard output as interactive on terminal stdout and complete text when redirected; no unsupported output-format flag remains.

The inherited supplemental WR-01 concerning discovered immutable read faults was separately traced through its final producer, observation, projection, recovery and receipt code. Its source-local classification, sticky precedence and verified-reset correction remain intact; no open residual WR-01 finding was identified.

## Behavioral and Security Assessment

| Boundary | Review conclusion |
| --- | --- |
| Recovery and retained accounting | Mandatory immutable/projection verification supplies complete aggregates under the existing publication guard. Startup carries the existing result through its mode rather than adding an accounting traversal. Failed/incomplete/budget-exceeded observations remain unavailable. Default recovery observation budgets are 2,000,000 rows/256 GiB; bounded incremental appends use their existing separate finite admission and checked arithmetic. These are not a permanent aggregate-value cap. |
| Achieved publication and freshness | Primary append and compatibility writers derive unique missing-row deltas from admitted encoded buffers, check before effects and install after successful SyncAll publication. Identical retries and within-batch duplicates do not double count; conflicting duplicates refuse. Raw mutation revokes shared trust, ordinary metadata revisions preserve immutable totals, and poison prevents old memory from becoming trusted observation. |
| Read faults and transport | Actual backend/codec/required-parent sources record fixed private categories under the already-held guard. Policy, absence, resource, cancellation and stale refusals do not manufacture faults. First observation revokes captured epochs and append proof once; Integrity dominates Backend, and both dominate owner pauses. Old handed frames retain physical accounting until settlement and cannot earn successful current-range completion after revocation. |
| Progress, durability and protection | The producer checks generation, branch, accepted target, active tip, processed endpoint, safe checkpoint, lifecycle and publication revision. Height equality alone grants no current/durable fact. The earned initial-sync latch, later lag, processed-ahead state and independently known protection/configuration remain distinct. Reserved protection and the soft block/undo target are unchanged by growth evidence. |
| Peer capability and consumers | Actual add/version/disconnect/remove transitions maintain checked fixed-size peer counters. The BASIC capture copies them without traversing peers. General configuration, explicit permission, requested all and actual offers remain separate; unmodeled implicit origins are disclosed. RPC and CLI forward the existing captured group once with unavailable serde defaults for older/stopped inputs. Baseline filter/index RPC shapes and authentication remain unchanged. |
| Privacy and terminal rendering | All fourteen new reason boundaries share exact normalization in serde, direct human rendering and support-copy sanitization. Available payloads use fixed enums, booleans, heights and counts. Support preserves safe captured facts without recollection or caller mutation. One nineteen-row formatter feeds consumers; actual normal/narrow buffer tests check adjacent values, complete unavailable/failure text, safe fallback, action boundaries and eight charts. |
| Evidence and claims | Genuine public accepted/Fjall daemon/client cases are distinguished from fresh private producer captures followed by authenticated response replay. Controlled owner classifications, software read/publication faults, isolated physical acknowledgements and manual owner-plan prune fixtures are described at their actual proof strength. Historical guards retain their refusal, preflight, ordering, byte-budget, ownership and genuine-selector controls. Final Plan 09 checks supplement runtime evidence and preserve Phase 162/v2.5 exclusions. |

No new production endpoint, authentication bypass, dependency, database, storage schema, raw-error export, source-datadir mutation or public-network activation was identified. Test fixture generation is explicit, private, exact-env and serial; normal freshness comparison is read-only and rejects missing, stale, malformed or private extra fields.

## Verification and Attribution

This reviewer independently executed these checks against the final released source using the re-probed pinned Bun 1.3.9:

| Reviewer-executed check | Result |
| --- | --- |
| Explicit `bun test ./scripts/check-phase161-operator-index-evidence.test.ts` | 34 passed, zero failed, 64 assertions |
| Live Phase 161 checker | Passed: 28 production seams, six caller selectors and scoped claims |
| Live Phase 155, 156, 157 and 159 source/claim checkers | Each exited zero |
| `bash -n scripts/verify.sh` | Passed |
| `git diff --check` | Passed |
| Final explicit-scope existence/uniqueness/ignore reconciliation | 113 distinct present paths; none ignored |

The first pre-release Phase 161 test run observed the author's meaningful unsupported-readiness RED while that checker was still evolving. The final released 34-test run above passed; no failing pre-release result is represented as final approval.

Runtime test counts are attributed to the completed executor/root evidence, not inferred from source strings or timing-only JSON. The reviewer inspected actual source-visible tests and timing receipts recording exit status zero for capture freshness, actual CLI/daemon callers, terminal cells, final strict Clippy, and every WR-02 regression selector; the RED receipt records exit 101. Timing receipts confirm command completion and duration, not test discovery counts by themselves.

`161-CALLER-EVIDENCE.md` records 111 distinct selected cases across its public/private producer and consumer windows; nine replay rows are internal cases, not nine extra discovered tests. Plan 07's twelve actual terminal-buffer cases are inherited presentation proof. The updated Plan 11 evidence records 76 passing WR-02 correction/regression executions, including two new tests; repeated prior cases are not counted again as new distinct coverage. No Cargo/Bazel job, source edit, tracker edit, staging, commit or push was performed by this reviewer.

The LOC report is reviewed as a derived tracked-file inventory. Its generator intentionally uses `git ls-files`, so the pre-stage report omits new untracked source files. Root owns final regeneration and hook validation after those files enter the index; current LOC totals are not used as phase-size or full-verification proof. Root also recorded a separate size inventory of the new source files.

## Explicit Simplification Pass

The final change retains one publication authority, a constant-size aggregate and fault observation, fixed lifecycle counters, thin RPC/collector forwarding and one shared presentation formatter. The WR-02 correction reuses the existing invalidator in one production line. Separate small test/render modules fit existing ownership seams. No further simplification is required to close a correctness finding; an additional cache, resolver, scanner or generic rendering framework would add responsibility without solving an outstanding problem.

## Guidance and Limits

Material guidance: repository `AGENTS.md`, `AGENTS.bright-builds.md`, placeholder-only `standards-overrides.md`, standards index and relevant architecture, code-shape, verification, testing, operability, Rust and Bun/TypeScript rules; phase context, all eleven plans, ten completed summaries with later corrections, Plan 09 source handoff, UI contract, caller evidence, supplemental core review, guard compatibility evidence and root UAT. Both canonical active lesson files were loaded completely: 7,188 bytes and 2,397 conservative estimated tokens. No lesson archive or project skill directory was loaded; project skill directories were absent.

This source review does not certify protocol behavior, archive-scale latency, physical power-loss safety or production readiness. The fixtures retain their disclosed synthetic-genesis, easy-PoW, maturity-one, manual owner-plan and limited-serving-window boundaries. Retained filters do not restore deleted bodies or wallet eligibility. Phase 162 owns the complete integrated continuous-chain/prune/reopen/RPC/both-transports proof and remaining milestone claims. BIP37/V0, filter import, repair/reindex/download, wallet acceleration, assumeutxo, public defaults/network CI, production service/readiness, funds safety and stronger v2.4 crash-window guarantees remain excluded.

Full default `bash scripts/verify.sh` and subsequent hook-protected verification are separate root-owned gates. They were starting/pending when this report was written; no full-native success or formal CFOP-01 completion is asserted here.

______________________________________________________________________

_Reviewer: gsd-code-reviewer_
_Depth: standard with focused cross-file tracing_

## Canonical Test-Layout Reinspection — 2026-10-11T08:53:49Z

The first root-owned default native attempt exited 1 after 651,964 ms at `scripts/check-panic-sites.sh`, before its Cargo stages. The failure is preserved in `161-NATIVE-EVIDENCE.md`. The guard identifies separate test files by canonical `*/tests.rs` or `*/tests/*` paths rather than following a parent module's `#[cfg(test)]`; it therefore treated seven test-only `expect` calls in the previously named `operator_snapshot/basic_tests.rs` as unclassified production sites. That failed attempt is not a full-native pass.

This reviewer inspected the released correction: the child is now `packages/open-bitcoin-node/src/network/operator_snapshot/tests.rs`, the parent declaration at `operator_snapshot.rs:465` remains `#[cfg(test)]` with `mod tests`, and its single existing `phase161-basic-operator-authority` breadcrumb mapping names the new path. The old file is absent. The child's SHA-256 is `bb80882d322ef3567841b339e23ae50d53a92e13137506956021ea843b8ba85c`, matching the recorded pre-rename contents. The four pure tests and all assertions remain intact. No production behavior, panic allowlist, checker or public contract changed.

The updated Plan 04 appendix records 18 discovered/passing authority tests under the renamed module, a passing panic policy, strict node Clippy and scoped formatting. This reviewer independently inspected the three timing receipts: canonical tests, panic guard and strict Clippy each record success with exit 0 (19,140 / 99,364 / 92,214 ms respectively, including documented cooperative-lock waits). Test discovery is attributed to the executor's recorded output, not inferred from timing receipts. This reviewer independently reran the parity checker: all 1,122 current Rust paths passed. Diff whitespace checks also passed.

The current frontmatter inventory replaces the old child path with the canonical path and still matches the refreshed explicit scope of 110 distinct files. WR-02 and WR-03 remain closed; no new actionable correctness or security finding was identified. The rename is the simplest correction because it preserves existing assertions and the existing policy unchanged.

Root's second default native attempt is running separately (`/tmp/phase161-native-full-2.log`, root session 89678). This reinspection does not assert its success, formal CFOP-01 completion or hook success. Root owns derived LOC regeneration and the final tracked inventory as before. Only this review artifact was changed by the reviewer; no source, Cargo/Bazel, tracker or git-index/history mutation was performed.

## Startup Diagnostic-Precedence Reinspection — 2026-10-11T09:27:51Z

The subsequent root-owned native attempt 2 exited 101 after 1,440,476 ms. Node tests reported 1,631 passed, one failed and three existing opt-in ignores; later RPC/Bazel/coverage stages were not reached. The unchanged failing production test was `filter_index_production_reopen_refuses_each_lone_partial_artifact` at `sync/tests/filter_index/startup.rs:537`. A projection-only index without saved state still refused with BlockIndex corruption/Repair, but forest measurement produced `missing BASIC suffix projection record` before the established `BASIC state is absent but index artifacts exist` diagnostic. Observable refusal precedence required correction. Both failed native attempts remain preserved in `161-NATIVE-EVIDENCE.md`; neither constitutes passing full-native evidence.

This reviewer inspected the released production correction at `filters/startup.rs:132`: present saved state still requires the complete forest/projection measurement before ownership inspection. Absent state produces only a private candidate empty observation. The unconditional existing `maybe_basic_filter_owner_guarded(control)?` at line 140 must then prove complete absence before that candidate can be installed. Its concrete loader checks record/projection prefix presence when state is absent, and `EffectiveIndexOwnership::maybe_from_artifacts` refuses any lifecycle, protection, reserved lock, records or projections without a checkpoint. Thus orphan-only partial state retains its original refusal and cannot earn zero totals or recovery authority. Existing-state order and injected observation limits are preserved; no accounting traversal, repair path or new authority was introduced. The strengthened empty-recovery test explicitly requires zero cumulative forest scans.

The original production test and assertion helper have no diff against the review base. They still cover record-only, projection-only and reserved-lock-only artifacts, require typed BlockIndex/Repair and the original diagnostic, compare unchanged index rows and verify retained body/undo. The executor's Plan 02 appendix records one discovered failing RED followed by the same unchanged test passing, 45 passing startup tests, 22 passing recovery/cumulative controls and passing strict node Clippy/format/parity. This reviewer inspected the corresponding timing receipts: RED is failure/101; GREEN, startup regressions, retention regressions and strict Clippy are success/0, at 8,384 / 38,995 / 18,123 / 5,519 ms respectively. Selector counts are attributed to the recorded test output; potentially overlapping regression selectors are not summed as distinct cases.

The five root-owned historical guard changes were also inspected: Phase 155 checker/test and Phase 156 checker/contract/test. They require the exact state-present predicate, propagated full measurement at branch depth 1, explicit candidate empty observation, unconditional ownership proof at depth 0 and no early success or total installation before that proof. Each suite adds five negatives for false/inverted state, omitted proof, premature totals and early return. The compact-code return check now uses the existing prefix convention, so `returnOk` cannot bypass it. Prior pruning, ownership, conditional-validation, source masking and refusal controls remain intact; no shared helper, panic policy or allowlist changed.

Root records 283 passing combined mutation tests, 564 assertions and zero failures after the guard repair. This reviewer independently ran the ten newly added state-gated recovery negatives: ten passed, zero failed, 20 assertions, with 273 unrelated tests filtered out. Both live affected guards, parity for all 1,122 current Rust paths and diff whitespace checks passed independently. These ten cases are a subset of root's 283, not additional distinct tests.

The explicit source inventory remains exactly 110 distinct present paths, including the canonical `operator_snapshot/tests.rs`. Existing resolved findings remain closed and no new open correctness/security finding was identified. The state-gated result plus existing owner/artifact classifier is the simplest fix and preserves the original regression assertions.

Root's default native attempt 3 is running separately (`/tmp/phase161-native-full-3.log`, root session 25307) after formatting and derived LOC regeneration. This report approves the re-reviewed source only; attempt 3, final hook verification and formal CFOP-01 completion remain pending/root-owned. Only this review artifact was changed by this reviewer; no source, Cargo/Bazel, tracker or git-index/history mutation was performed.

## Existing Black-Box Schema Consumer Reinspection — 2026-10-11T10:04:34Z

Root's default native attempt 3 subsequently exited 101 after 1,634,308 ms. Node passed 1,632 tests with zero failures and three existing ignores; RPC library passed 348 and daemon passed 87. The previously corrected startup diagnostic test passed. The black-box target then reported two passed and one failed because `phase127_production_composition_shares_sync_serving_and_operator_authority` still expected seven status-response keys. The actual additive `basic_index` eighth field caused the failure. Later benchmark/Bazel/coverage success was not established by that attempt. All three failed full-native attempts remain preserved in `161-NATIVE-EVIDENCE.md` and this review history.

The authorized addition to review scope is the existing `packages/open-bitcoin-rpc/tests/black_box_parity/phase127_composition.rs`, now 454 lines. This reviewer read the complete file and inspected its diff. At line 427, the exact sorted-key assertion retains every original key and adds `basic_index` in its correct first position. The preceding blockchain/network key assertions, serving checks and original block-relay equality remain intact. At line 379, a single `authoritative_operator_snapshot()` is bound while holding the same existing context lock; both block-relay and BASIC are serialized from that one snapshot. The new BASIC JSON equality at line 444 compares the actual HTTP response with that captured authority. No second snapshot, guessed default, mock, weakened original assertion or new production behavior was introduced.

Plan 05's appended correction records the native failure as RED and three passing exact black-box cases after the fix. Its authorized remaining-workspace preflight excludes only the unchanged node crate already passed in native attempt 3: 2,596 passed, zero failed/ignored across 38 result groups, comprising 2,577 ordinary tests and nineteen doctests. The separately selected CLI client binary passes seven tests, and strict RPC all-target/all-feature Clippy passes. This reviewer inspected the four timing receipts: exact black-box, remaining workspace, CLI client and RPC lint all record success/0 at 9,790 / 271,253 / 350 / 10,202 ms. Discovery counts are attributed to the executor's recorded output, not inferred from those receipts; the exact and broader selections overlap and are not summed as distinct tests.

This reviewer independently reran the live Phase 161 source/claim checker, parity validation for all 1,122 current Rust paths and diff whitespace checks; each passed. The added file already has its established parity breadcrumbs, and no manifest, guard, production API or dependency change accompanies this correction. Scope reconciliation now confirms exactly 111 distinct present paths with none ignored or missing; the current frontmatter lists all 111. Earlier 110-file inventories above describe their historical review windows. WR-02 and WR-03 remain closed, and no open source correctness or security finding remains after this reinspection.

The minimal exact-key update plus two projections from one existing snapshot preserves authority and increases the assertion strength. Earlier focused checks did not cover this independent legacy exact-key consumer; this appended review does not represent those earlier checks as complete black-box compatibility evidence.

Root is running the remaining benchmark/Bazel/pure-core coverage preflight (`/tmp/phase161-tail-preflight.log`, root session 7623) before the required fourth complete default native attempt. Neither that preflight, native attempt 4, final hooks nor formal CFOP-01 completion is asserted passed here. Generated LOC and its final tracked inventory remain root-owned. Only this review artifact was changed by this reviewer; no source, Cargo/Bazel, tracker or git-index/history mutation was performed.

## Phase 127 Exact-Schema Guard Reinspection — 2026-10-11T10:17:57Z

The final narrow scope addition comprises `scripts/check-phase127-authoritative-network-state-unification.ts` and its existing `.test.ts`. This reviewer read both complete files and their diff. The checker changes one exact frozen-array literal at line 356: require `basic_index` first while retaining all seven previous keys unchanged. There is no wildcard, optional-key allowance or changed authority, durable-serving, owned-snapshot, restart, redaction, parity, local-only or verifier-order logic. Shared source/evidence helpers, Rust and production APIs are unchanged.

The added test at `.test.ts:51` uses the existing isolated source-corpus fixture. It removes only `basic_index` from the precise status-key assertion, first requires the unmodified live proof to pass, and then requires exactly the original integration-proof diagnostic from the mutated fixture. The replacement helper fails if its expected mutation target is absent. All seventeen prior tests remain unchanged. The two script files remain below the managed length limit at 556 and 618 lines.

Before this repair, a separate targeted preflight recorded 15 passing and two failing tests plus the same live checker mismatch against its stale seven-key anchor. Root intentionally stopped native attempt 4 before reaching that known later Phase 127 step: only verified live Bun child PID 52695 received SIGTERM, and the normal Bash/timing runner exited 143 after 283,445 ms and released its cooperative lock. This was an intentional stop during live progress, not a timeout, stall, direct Phase 127 failure within that native attempt or new Rust failure. The three earlier failed complete attempts remain preserved above and in `161-NATIVE-EVIDENCE.md`.

This reviewer independently executed the entire final Phase 127 test file using pinned Bun 1.3.9: 18 passed, zero failed, 19 assertions, approximately 15 seconds. The live Phase 127 checker and diff whitespace check also independently exited zero. These results agree with the root-owned guard compatibility appendix, whose earlier final 18-test run is overlapping evidence rather than another eighteen distinct cases. Scope reconciliation confirms exactly 113 distinct present paths, none ignored or missing; the current frontmatter lists every path. Prior 110/111 inventories above describe their historical windows. Existing resolved findings remain closed, with no new open source correctness/security finding.

The exact-anchor update and one omission-negative are the simplest compatible correction. They preserve the existing historical contract and the stronger black-box BASIC authority assertion without changing another guard or helper. Root records the remaining workspace/CLI preflights and all benchmark, six-target Bazel, provenance and exact native pure-core coverage preflights as passed. These partial/preflight results still do not establish a clean complete default run.

Root's complete default native attempt 5 is active separately (`/tmp/phase161-native-full-5.log`, root session 16380), following actual formatting and derived LOC regeneration. This report approves the final re-reviewed source only; attempt 5, mandatory hook completion and formal CFOP-01 closure remain pending/root-owned. Only this review artifact was changed by this reviewer; no source, Cargo/Bazel, tracker or git-index/history mutation was performed.
