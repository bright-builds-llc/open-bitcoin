---
phase: 159-authenticated-basic-filter-and-index-rpcs
reviewed: "2026-10-09T23:05:07Z"
initial_reviewed: "2026-10-09T17:32:16Z"
depth: standard
scope: final-whole-phase-git-source
final_phase_review: clean
cli_guard_extension_review: clean
recovery_guard_extension_review: clean
native_verification: pending
diff_base: 8f5acfb928c6258b5f899273369ea43a13b70d9d
files_reviewed: 91
files_reviewed_list:
  - README.md
  - docs/parity/catalog/basic-compact-filters.md
  - docs/parity/catalog/rpc-cli-config.md
  - docs/parity/deviations-and-unknowns.md
  - docs/parity/index.json
  - docs/parity/source-breadcrumbs.json
  - packages/README.md
  - packages/open-bitcoin-cli/src/args/tests.rs
  - packages/open-bitcoin-cli/src/client.rs
  - packages/open-bitcoin-cli/src/client/tests.rs
  - packages/open-bitcoin-node/src/chainstate.rs
  - packages/open-bitcoin-node/src/chainstate/fjall_store.rs
  - packages/open-bitcoin-node/src/chainstate/tests.rs
  - packages/open-bitcoin-node/src/chainstate/validation_history.rs
  - packages/open-bitcoin-node/src/chainstate/validation_history/tests.rs
  - packages/open-bitcoin-node/src/chainstate/validation_history/tests/coverage.rs
  - packages/open-bitcoin-node/src/chainstate/validation_history/tests/fixtures.rs
  - packages/open-bitcoin-node/src/chainstate/validation_history/tests/network.rs
  - packages/open-bitcoin-node/src/lib.rs
  - packages/open-bitcoin-node/src/network.rs
  - packages/open-bitcoin-node/src/network/runtime_authority.rs
  - packages/open-bitcoin-node/src/network/runtime_authority/effects.rs
  - packages/open-bitcoin-node/src/network/runtime_authority/filter_index.rs
  - packages/open-bitcoin-node/src/network/runtime_authority/filter_index/catch_up/tests.rs
  - packages/open-bitcoin-node/src/network/runtime_authority/filter_index/query.rs
  - packages/open-bitcoin-node/src/network/runtime_authority/filter_index/query/tests.rs
  - packages/open-bitcoin-node/src/network/runtime_authority/filter_index/readiness.rs
  - packages/open-bitcoin-node/src/network/runtime_authority/filter_index/readiness/owner.rs
  - packages/open-bitcoin-node/src/network/runtime_authority/filter_index/readiness/tests.rs
  - packages/open-bitcoin-node/src/network/runtime_authority/filter_index/readiness/tests/failures.rs
  - packages/open-bitcoin-node/src/network/runtime_authority/filter_index/readiness/tests/interleavings.rs
  - packages/open-bitcoin-node/src/network/runtime_authority/lifecycle.rs
  - packages/open-bitcoin-node/src/network/runtime_authority/validation_history.rs
  - packages/open-bitcoin-node/src/network/validation_history.rs
  - packages/open-bitcoin-node/src/storage.rs
  - packages/open-bitcoin-node/src/storage/fjall_store.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/coins.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/filters.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/filters/ownership.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/filters/publication.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/filters/query.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/filters/query/tests.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/filters/startup.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/filters/tests.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/filters/tests/append.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/filters/tests/append/admission.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/filters/tests/append/fencing.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/payload_usage.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/validation_history.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/validation_history/tests.rs
  - packages/open-bitcoin-node/src/storage/validation_history.rs
  - packages/open-bitcoin-node/src/storage/validation_history/tests.rs
  - packages/open-bitcoin-node/src/sync/runtime_state.rs
  - packages/open-bitcoin-node/src/sync/runtime_state/tests.rs
  - packages/open-bitcoin-node/src/sync/tests/filter_index/catch_up.rs
  - packages/open-bitcoin-node/src/sync/tests/filter_index/catch_up/rpc_faults.rs
  - packages/open-bitcoin-rpc/src/bin/open_bitcoind/coins_flush.rs
  - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index.rs
  - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/fixtures.rs
  - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/rpc.rs
  - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/rpc/failures.rs
  - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/rpc/retention.rs
  - packages/open-bitcoin-rpc/src/context.rs
  - packages/open-bitcoin-rpc/src/context/filter_index.rs
  - packages/open-bitcoin-rpc/src/dispatch.rs
  - packages/open-bitcoin-rpc/src/dispatch/filter_index.rs
  - packages/open-bitcoin-rpc/src/dispatch/filter_index/tests.rs
  - packages/open-bitcoin-rpc/src/error.rs
  - packages/open-bitcoin-rpc/src/http.rs
  - packages/open-bitcoin-rpc/src/http/filter_index.rs
  - packages/open-bitcoin-rpc/src/http/request.rs
  - packages/open-bitcoin-rpc/src/http/tests.rs
  - packages/open-bitcoin-rpc/src/http/tests/filter_index.rs
  - packages/open-bitcoin-rpc/src/http/tests/filter_index/fixtures.rs
  - packages/open-bitcoin-rpc/src/method.rs
  - packages/open-bitcoin-rpc/src/method/filter_index.rs
  - packages/open-bitcoin-rpc/src/method/filter_index/normalize.rs
  - packages/open-bitcoin-rpc/src/method/filter_index/tests.rs
  - packages/open-bitcoin-rpc/src/method/tests.rs
  - scripts/check-parity-breadcrumbs.test.ts
  - scripts/check-parity-breadcrumbs.ts
  - scripts/check-current-documentation-reconciliation.ts
  - scripts/check-current-documentation-reconciliation.test.ts
  - scripts/check-phase134-authoritative-lifecycle.test/mutations.ts
  - scripts/check-phase157-index-catch-up/contracts.ts
  - scripts/check-phase158-validated-reorg.ts
  - scripts/check-phase159-filter-rpcs.test.ts
  - scripts/check-phase159-filter-rpcs.ts
  - scripts/check-phase159-filter-rpcs/contracts.ts
  - scripts/check-phase159-filter-rpcs/evidence.ts
  - scripts/verify.sh
findings:
  critical: 0
  warning: 0
  info: 0
  total: 0
initial_findings:
  critical: 0
  warning: 2
  info: 0
  total: 2
closed_findings: [WR-01, WR-02, WR-03, WR-04, WR-05, WR-06, WR-07]
status: clean
generated_by: gsd-code-review
lifecycle_mode: yolo
phase_lifecycle_id: 159-2026-10-09T16-11-49
generated_at: "2026-10-09T23:05:07Z"
---

# Phase 159: Final Whole-Phase Source Review

**Reviewed:** 2026-10-09T23:05:07Z; all historical findings, closures and subsequent recovery/fixture deltas are preserved below.
**Depth:** standard
**Files Reviewed:** 91: 72 Rust files, twelve scripts and seven documentation/manifests; the renamed runtime test module replaces its former path.
**Status:** clean; all seven findings are closed, including actual regression GREEN for WR-07. Original descriptions and closure evidence are retained.
**Final source review:** clean across the reviewed 91-file scope and all assigned deltas; the final one-file fixture correction has actual exact-case GREEN. Continuing workspace/CLI tests, full native execution and formal phase/security/lifecycle gates remain independent; this report does not claim those gates passed.

## Summary

Final whole-phase scope was independently reconciled from the tracked and nonignored untracked Git inventory, rather than only SUMMARY metadata. Subsequently assigned deltas bring reviewed coverage to 91 paths: 72 Rust, twelve scripts and seven documentation/manifests. Five newly changed tracked test paths join the prior 86-file scope for the native-3 recovery repair. The canonical runtime test-module rename and all prior scopes remain represented. Both frozen CLI and later 125-selector recovery guard extensions are reviewed on the existing script paths. Planning/tracker/config state is outside source review, and the intentionally generated LOC report was read as context rather than counted as source. The pinned Knots gitlink remains unchanged; ignored/generated/vendor source was not selected. No Git operation was run for this assigned delta.

Reused unchanged Plans 01–07 review evidence and rechecked evolved shared-source connections, guards/mutations, historical checker edits, documentation/parity mappings and UAT targets/flags. The six earlier issues remain closed. Native attempt 3 exposed WR-07; recovery run 2 passed its four original cases and new live/reopen regression. The last fixture correction pairs store/manager reopen and asserts the already-restored exact genesis endpoint without an extra worker turn. Its exact test now passes, closing the source-review execution follow-up. No new production source change is introduced by that final fixture repair.

The genuine acceptance receipts retain all replacement positions before fallible effects, and occupied history admission precedes network mempool preparation. The parser follows the pinned framework's named conversion, arity/help, aggregate type, hash and filter-selection order. Both original raw-write coverage findings remain closed: raw state cannot retain unsupported complete coverage or alter genuine positive history. The later compatibility issue concerns strict parsing of the raw DTO before that conservative invalidation, not permission to fabricate accepted history.

The incremental Plan 04 assessment found no additional issues. Its completed read path holds the configured network authority and shared publication guard, resolves known identity before row inspection, and fully parses at most target plus immediate parent under maintained all-record recovery authority. Found, unknown, missing provenance and typed failures remain distinct. Plan 05's omitted poison settlement and WR-07's compatibility regression are closed. Plans 06/07 authenticated integration, maintenance handoff and retention/fault evidence retain their reviewed guarantees. The fixture has exact-case GREEN; root's continuing workspace/CLI and full native/formal gates remain independent.

Material guidance: repository `AGENTS.md`, `AGENTS.bright-builds.md`, placeholder-only `standards-overrides.md`, and the local architecture, code-shape, verification, testing and Rust standards. Both active lesson inputs were completely loaded: 7,188 bytes and 2,397 conservative estimated tokens; an existing audit baseline was present and no audit trigger applied. Phase plans/context/research and pinned Knots RPC/framework sources informed the review. No ignored source files were selected.

## Closed Native Compatibility Finding

### WR-07: Sparse raw metadata was treated as corrupt accepted identity

**File:** `/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-node/src/storage/fjall_store/validation_history.rs:63-94`

**Status:** closed by actual regression evidence and source recheck at 2026-10-09T22:45:43Z; initially implemented/source-reviewed at 21:56:46Z.

**Issue and observed failure:** The earlier `check_validation_coverage_for_metadata` constructed `BlockValidationIdentity` from every caller-supplied `ChainPosition` before looking for authentic ledger evidence. A legacy sparse position can be representable raw metadata while failing accepted-identity invariants—for example, height 50 with a zero parent. The native-3 run exposed four legacy prune/flush errors from this strict raw parse, including failure after unlink effects. Raw metadata lacks authority and must lose complete coverage, rather than being rejected as if it were a corrupt stored accepted record. This compatibility regression is distinct from the correctly closed WR-02 requirement that raw metadata cannot preserve complete coverage.

**Fix reviewed:** The helper now strictly decodes the actual same-store ledger row first and compares its authenticated status/hash/parent/height directly with the raw DTO. Absent, mismatching or non-ScriptsValid rows cause durable UnknownLegacy invalidation before metadata effects. Existing row read/decode errors still propagate; corrupt ledger data is not converted into absence. The accepted-identity constructor and genuine receipt mint/publication remain strict and unchanged. No new raw authority flag or weakened constructor is introduced.

**Regression and closure criteria:** New `phase159_validation_history_coverage_sparse_raw_metadata_invalidates_without_identity_parse_error` covers both absent-row and matching-hash/wrong-height sparse metadata through the public sink. It asserts the strict constructor still rejects, raw metadata persists, coverage is unknown live and after all-handle Fjall reopen, and a separate genuine positive record remains ScriptsValid. The initial source review required actual execution of this regression and the four original failing legacy prune/flush cases, plus relevant append/recovery/source-assertion controls; the full native gate remains independent.

**Closure evidence:** Root's ordered Rust recovery run 2 actually passed all four original Group A cases and the new sparse raw metadata live/reopen regression. It also passed the four repaired append cases, new raw-clone invalidation/recovery-refusal control and moved-producer source assertion. The node suite was **1,428 passed, one failed, three ignored**, so this establishes WR-07's targeted closure rather than a whole-run pass. Its sole remaining failure is the separately diagnosed store/manager fixture branch mismatch below; no production metadata regression remains observed. The reviewer reused the already-inspected strict-ledger/permissive-raw source fix and did not execute tests.

## Closed Final Checker Warnings

These descriptions preserve the source and reproduction scenarios from the initial final review at 19:29:59Z. Each fix was independently rechecked at 19:52:19Z and is no longer an open finding.

### WR-04: Required Rust tests can be compiled out while counted as executable evidence

**Status:** closed by source recheck at 2026-10-09T19:52:19Z.

**File:** `/Users/peterryszkiewicz/Repos/open-bitcoin/scripts/check-phase159-filter-rpcs.ts:34-37`

**Issue:** The required-selector check only finds a test attribute, rejects `#[ignore]`, and looks for assertions. It does not reject an excluding `cfg` on the function or its owning module. `checkRegistrations` also checks module/path tokens without accounting for the owning attributes. A required test can therefore disappear from Cargo discovery while the guard still reports 117 executable selectors. Full workspace testing need not fail when those tests silently compile out.

**Reproduction:** In a disposable guard fixture, add `#[cfg(any())]` immediately before the attributes of `phase159_filter_rpc_contract_numeric_errors_roundtrip`; leave its body unchanged. The current attribute/body conditions still accept it, but Rust never builds the test. An owning-module variant, such as adding the same false cfg to the daemon `mod rpc;` include, leaves the child source selectors and module tokens intact. `#[cfg(not(test))]` is another excluding test configuration. These are source-derived mutation scenarios, not tests executed by this reviewer.

**Fix:** Validate the executable attributes of each required selector and its complete owning module/include chain for the supported test configuration. Keep legitimate `#[cfg(test)]` ownership; reject false/excluding cfg and conditional-ignore mutations rather than accepting textual test attributes alone. Add independent negative mutations for function-level and owning-module exclusion, plus the existing valid configured ownership controls. Keep actual executed Rust tests as the behavioral proof; the lexical guard must not label compiled-out selectors executable.

**Closure:** The new evidence helper checks selector declarations, enclosing inline modules, inner file attributes and all 67 fixed actual external owner edges, including physical-path overrides and exact include resolution. It accepts canonical `cfg(test)` only for test ownership and rejects unsupported excluding cfg/cfg_attr/ignore configurations conservatively. The existing conditional Clippy deny policy is allowed through a fully anchored allowlist for the complete lint-only payload; combined ignore/cfg payloads cannot match it. Production contracts also check supported ownership. Mutations cover every external edge plus function, inline/file, conditional-ignore/exclusion and valid inner-body controls. This is supported-configuration validation, not a universal Rust configuration evaluator.

### WR-05: Constant comparisons satisfy the claimed nonempty behavior check

**Status:** closed by source recheck at 2026-10-09T19:52:19Z.

**File:** `/Users/peterryszkiewicz/Repos/open-bitcoin/scripts/check-phase159-filter-rpcs.ts:120-135`

**Issue:** `meaningfulAssertions` rejects only assertion arguments that reduce exactly to `true`, `false`, `0` or `1`. It counts `assert_eq!(1, 1)`, `assert_ne!(1, 2)` and `assert!(1 == 1)` as meaningful. Required selectors without a dedicated TEST_CONTRACT can thus be replaced with passing no-op constant assertions, while the phase guard and Cargo both pass. The mutation suite currently exercises only `assert!(true)` as its no-op replacement.

**Reproduction:** Replace the body of required `phase159_filter_rpc_contract_numeric_errors_roundtrip` with `{ assert_eq!(1, 1); }` in a disposable fixture. That selector is still present and nonignored; it has no TEST_CONTRACT body anchors, and `compact("1, 1")` does not match the trivial-value regex. The current guard accepts a test that exercises no RPC code. This is a source-derived mutation scenario; this reviewer did not execute it.

**Fix:** Reject assertions whose operands/conditions are only constants or constant expressions, including optional message arguments, and ensure required behavioral selectors retain a real action/assertion or named meaningful helper connection. Extend no-op mutations with constant equality/inequality/tautology cases rather than only a bare Boolean. Preserve the checker as a narrow supplement to executed behavior tests, without claiming that arbitrary expression semantics have been proven by token presence.

**Closure:** The helper now rejects literal-only conditions/equality/inequality/arithmetic expressions and simple self-comparisons, excluding optional message arguments from meaningful operands. Independent helper tests retain a real action while replacing the assertion, so the new action checks cannot conceal a weak assertion classifier. All 117 fixed selectors additionally require a current minimum action connection, and the numeric roundtrip has an explicit code/serialization/deserialization contract. Every minimum action has a removal mutation preserving names/assertions. The checks remain deliberately narrow and do not claim arbitrary Rust expression or general dataflow proof; actual behavior tests remain necessary.

### WR-06: Phase-specific breadcrumb discovery includes Git-ignored source

**Status:** closed by source recheck at 2026-10-09T19:52:19Z.

**File:** `/Users/peterryszkiewicz/Repos/open-bitcoin/scripts/check-phase159-filter-rpcs.ts:169-178`

**Issue:** The new phase guard recursively enumerates every `.rs` beneath node/RPC source directories with `readdirSync`, independently of Git ignores and gitlinks. The canonical breadcrumb checker was correctly changed to cached plus nonignored untracked inventory, but the phase guard can still inspect ignored build/scratch files and fail their missing mappings. Because it runs in the default verifier, an otherwise valid ignored generated file can block verification.

**Reproduction:** In a disposable Git checkout, place a Rust file under `packages/open-bitcoin-node/src/target/generated.rs`. The repository's `target` ignore rule excludes it from `git ls-files --others --exclude-standard` and from the canonical checker. The current phase-specific recursive scan still discovers it and returns a missing breadcrumb registration error. The same issue occurs with an exact Git ignore rule for a generated `.rs` under either scanned root. This is a source-derived scenario; no ignored file was created or reviewed in this checkout.

**Fix:** Reuse an ignore-aware current-worktree inventory consistent with `check-parity-breadcrumbs.ts`, retaining fixed required NEW_RUST paths while excluding ignored files, generated descendants and submodule contents from discovery. Make fixture inventory explicit where needed. Add real Git regressions proving required untracked source is checked, ignored generated Rust is skipped, and gitlinks are not recursively treated as first-party source. Avoid maintaining two divergent breadcrumb discovery policies.

**Closure:** The phase guard imports the canonical exported `worktreePaths` helper with unchanged cached plus others/exclude-standard semantics. The canonical checker CLI is gated by `import.meta.main`, so importing its inventory does not execute the CLI or exit the process. Fixed required NEW_RUST paths remain explicit. Temporary-Git controls prove ignored untracked generated Rust is excluded, ignored tracked source remains checked, gitlinks are not recursively scanned, and previously required manifested/unmapped untracked cases still work. The duplicate recursive traversal is removed; no checkout index mutation was performed by this reviewer.

## Closed Readiness Warning

The initial description below records the source at the Plan 05 review. The subsequent closure is verified separately and it is no longer an open finding.

### WR-03: Direct lifecycle dispatchers observe poisoned authority without settling pending reads

**Status:** closed by source recheck at 2026-10-09T18:24:23Z.

**File:** `/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-node/src/network/runtime_authority/lifecycle.rs:48-51`

**Related call paths:** the same file's checkpoint completion at lines 69–76 and checkpoint abort at lines 88–95; `network/runtime_authority/effects.rs:268-271` has another direct lock in the public checkpoint-evidence read. Public `prepare_peer_relay_effect` reaches the first path through `apply_lifecycle_command`.

**Issue:** New shared `read`/`mutate` helpers call `basic_filter_authority_unavailable` when locking fails, collecting terminal results and waking registered BASIC readers after releasing the poisoned guard. The direct dispatcher paths instead return an authority error immediately. A pending Future stores only the readiness registry, so it cannot discover authority poison by itself. If one of these dispatchers observes the failure while handles remain alive, the requester can stay pending indefinitely until some other notifying operation or shutdown happens. This violates Plan 05's terminal authority-settlement contract. The existing poison regression calls `maybe_basic_index_summary`, which exercises the corrected shared helper and misses these direct paths.

**Reproduction:** Reuse the genuine runtime from `phase159_basic_readiness_owner_poisoned_authority_wakes_typed_failure`: accept an unprocessed block, register and poll a barrier with a counting waker, then poison the real authority. Replace the summary call with `runtime.network.prepare_peer_relay_effect(1)`. It returns an authority error, but the original branch neither increments the wake count nor records a result; a subsequent direct barrier poll remains Pending. Prepare genuine checkpoint receipt/abort carriers before poison to exercise the two preserving dispatchers, and use the public `checkpoint_evidence` method for the evidence-read variant. This was a source-derived regression scenario; the executor subsequently reproduced it in RED and this reviewer ran no tests.

**Fix:** Route every production direct authority-lock poison branch through the same readiness failure handler. Explicitly drop `poison.into_inner()` before collecting or invoking wakers, preserve existing typed errors and receipt/abort ownership, and keep callbacks outside the authority, publication and registry guards. Alternatively reuse the shared read/mutate helper where its signature already fits. Add direct lifecycle and checkpoint/evidence regressions with counting and reentrant wakers, proving terminal `AuthorityUnavailable` without a second summary/read operation or last-handle drop.

**Closure:** The private `lock_authority` helper in `readiness/owner.rs` now releases the `PoisonError`-contained guard before calling the shared readiness settlement handler. General read/mutate/stop and all three direct lifecycle dispatchers use it; the checkpoint-evidence facade uses shared read. Source search confirms production authority locking is centralized, with only test poison/try-lock helpers outside it. Existing typed lifecycle errors and boxed checkpoint receipt/abort returns are retained. The new direct-relay regression counts a wake before repoll; the dispatcher-family test covers relay, checkpoint evidence, a genuine Sync-persisted completion receipt, a real retained adapter abort, and poisoned stop, asserting authority/registry/publication guard availability during wake and terminal `AuthorityUnavailable`. Receipt/abort ownership is checked against the original carriers. Revised Plan 05 evidence records the actual wake-count-zero RED, 35 passing combined query/readiness tests, 47 managed-effect regressions, eight checkpoint regressions and clean strict scoped lint/format/diff checks. These commands were not rerun by this reviewer.

## Closed Initial Warnings

The initial descriptions and reproduction scenarios below are preserved for traceability; they describe the source before the fixes. They are not open findings against the rechecked source.

### WR-01: Raw snapshot seeding invalidates coverage after fallible state writes

**Status:** closed by source recheck at 2026-10-09T17:38:38Z.

**File:** `/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-node/src/storage/fjall_store/coins.rs:353-365`

**Issue:** `seed_coins_from_snapshot` invalidates BASIC append authority, then writes migrated coins and each undo record. Validation coverage is invalidated only afterward through `save_unverified_chain_meta` at lines 367–370, whose invalidation is at line 51. A write error or interruption before that final call leaves raw-seeded durable state while the shared and persisted validation-coverage marker still says `Complete`. This violates the new raw-seeding boundary and lets absent accepted history retain a definite `NeverConnected` classification.

**Reproduction:** Open a fresh complete-coverage store, construct a representable raw snapshot with a tip and at least one undo record, inject the existing `FilterPublicationFault::BeforeUndo`, and call `seed_coins_from_snapshot`. The call returns an error after the coins write but before coverage invalidation. Check coverage and absent-hash provenance live and after dropping every handle and reopening: the current control flow retains complete coverage. This is a source-derived regression scenario; this reviewer did not execute it.

**Fix:** Durably invalidate validation coverage at the beginning of the raw seed operation, before any coins or undo effects; release the publication guard before invoking nested writers that acquire it. Preserve all existing positive `ScriptsValid` records. Add the `BeforeUndo` failure/reopen regression and a successful raw-seed control, asserting unknown coverage and retained positives. Keep genuine managed flushes eligible to preserve coverage.

**Closure:** `seed_coins_from_snapshot` now calls `invalidate_validation_coverage()` inside its initial publication-guard scope, before any migrated coins/undo write, and releases that guard before nested writers. The invalidation helper commits the coverage marker with `SyncAll`, rejects failed invalidation before effects, and never modifies positive ledger records. The new `phase159_validation_history_coverage_raw_seed_undo_fault_invalidates_before_coins_mutation` proves coins changed before the injected `BeforeUndo` failure while coverage is unknown live/reopen and genuine prior acceptance survives. The successful raw-seed test covers the non-fault path.

### WR-02: The public flush sink bypasses raw metadata coverage invalidation

**Status:** closed by source recheck at 2026-10-09T17:38:38Z.

**File:** `/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-node/src/storage/fjall_store/coins.rs:74-115`

**Related call path:** `chainstate/flush_lifecycle/fjall_sink.rs:47-48` forwards public `FlushPersistSink::persist_chain_meta` directly to this method; `chainstate/fjall_store.rs:312-317` exposes the same forwarding through `FjallChainstateStore`.

**Issue:** `save_chain_meta` now invalidates validation coverage, but callers can reach `save_validated_chain_meta` through the public sink instead. That writer accepts raw `ChainPosition` values and writes metadata without checking genuine validation-history evidence or invalidating coverage. When `maybe_pending_coins` is absent, the conditional BASIC fence checks are skipped entirely. The existing BASIC completion receipt protects BASIC append authority, but it does not prevent this raw path from preserving the new complete-history marker. A raw clone can therefore seed chain metadata while retaining complete coverage, including across reopen.

**Reproduction:** Open a fresh store and clone it. Pass an arbitrary representable nonempty `ChainPosition` list to `FlushPersistSink::persist_chain_meta(&mut clone, &positions)`. With no pending BASIC coins publication, metadata is written successfully. Synchronize the database through an ordinary public sync write, drop all handles, and reopen. Coverage remains complete and the seeded hash's absent ledger row yields `NeverConnected`, whereas unauthenticated metadata must preserve legacy uncertainty. The same omission is observable immediately without reopen. This is a source-derived regression scenario; this reviewer did not execute it.

**Fix:** Make the raw/public sink path invalidate coverage before effects. Preserve coverage only when the existing managed producer supplies sealed same-store validated evidence, or when every supplied position is checked against matching authentic `ScriptsValid` history; do not treat the method name, raw positions or nominal BASIC pending state as sufficient evidence. Reuse the existing lineage/receipt design where practical. Add a public-sink raw-clone regression proving `UnknownLegacy` live/reopen and positive-row retention, plus the existing genuine-genesis forced-flush control proving trusted coverage remains complete.

**Closure:** The shared `save_validated_chain_meta` writer now calls `check_validation_coverage_for_metadata` under the existing publication guard before metadata effects or BASIC pending-state checks. That helper preserves complete coverage only for a nonempty list whose every position matches this store's authentic `ScriptsValid` record, including hash, height and parent. Empty/missing/unaccepted identities durably invalidate coverage; malformed or failed reads reject before effects. The history guard is released before invalidation reacquires it. Both public forwarding paths therefore share the fix, without a trusted Boolean or new framework. The new `phase159_validation_history_coverage_public_metadata_sink_cannot_preserve_unaccepted_positions` checks direct `FjallNodeStore` and wrapped `FjallChainstateStore` calls live/reopen with retained genuine positives. The existing genuine-genesis forced-flush test remains the positive control for trusted complete coverage.

## Incremental Plan 04 Assessment

**Reviewed:** 2026-10-09T17:51:37Z. **Open findings:** zero. Read `159-04-PLAN.md`, `159-04-SUMMARY.md` and `159-04-QUERY-MEASUREMENTS.md`, inspected actual query/storage code and tests, and traced authority producers and immutable writers through existing recovery/lifecycle/append code.

- Read integrity is enabled only after explicit initialization proves an empty forest, a complete retained-row scan plus validated immutable publication, or the existing exclusive lifecycle recovery producer has completed its forest/projection preflight. Shared poison prevents reads after ambiguous publication; raw BASIC put/remove clears integrity across clones under the publication mutex before effects. Startup clears authority before recovery and again before its final all-record scan. Trusted append/reorg paths preserve immutable integrity without introducing a second store or cache.
- Genesis requires one immutable record; other queries require at most target and immediate parent. Both use full `parse_record`, including bounded encoding, byte hash and own header commitment; the target verifies its immediate edge and the known block's height/parent. No serving call reaches the ancestry walkers, projection scan, body/undo access or filter generator. Retained identities use bounded borrowed validation-ledger decoding instead of copying arbitrary values before the 70-byte check.
- The per-record ceiling is 33,554,602 bytes, two-record ceiling 67,109,204 bytes, and hexadecimal response ceiling 67,108,928 bytes. Checked accounting precedes parsing/copying: SHA input copies include both full filter hashes, second digests, header commitments and the target's repeated identity commitment; padding matches the current `Sha256::digest` implementation. Maximum logical-copy and padded-hash ceilings are 100,663,648 and 67,109,696 bytes. These are logical lengths, not allocator capacity, reallocations, RSS or serialization-peak measurements.
- Known resolution prefers sealed pending acceptance, then direct header lookup, then one retained-ledger point read. Missing-row provenance uses the same manager, including live pending acceptance after failed history publication. Lifecycle/record/provenance reads are distinguished in the measurements. Summary reads processed height and the initial-sync latch with zero fallback, using the configured lifecycle under the same snapshot; it performs no history scan.
- Reviewed corruption/missing-parent/raw-ancestor controls, actual reopened stale data, initial found/missing behavior, pending sealed identity, summary lag/reopen controls, exact envelope/hex/overflow limits and full codec-capacity admission. The recorded parent-fields-only mutation fails the corrupt-parent test. The 32 MiB fixture is synthetic codec-capacity evidence; the legal large-block fixture produces seven filter bytes because repeated scripts deduplicate. Neither demonstrates a consensus-generated 32 MiB filter or a production latency guarantee.

Plan 04 reports 14 passing focused tests, three measurement tests, strict scoped Clippy and formatting success, and the parent-parse mutation RED. Those are executor-reported results checked against actual source/tests, not commands rerun by this reviewer. Plans 05–07 were subsequently reviewed below; active Plan 08 edits and final whole-phase verification still require independent review.

## Incremental Plan 05 Assessment

**Reviewed:** 2026-10-09T18:15:20Z; closure recheck 18:24:23Z. **Open findings:** zero; WR-03 is preserved above as closed. Read the complete Plan 05 summary/plan, all five new readiness source/test files, changed query/lifecycle/reader integration, moved shared authority helpers and the effect-facade closure.

- The registry admits exactly 64 outstanding registrations and uses checked independent `u64` IDs. Capacity/overflow checks precede insertion; completion polling and Drop remove the slot. Original frontier and request identity/provenance are privately retained, and the final completion token cannot be cloned or constructed publicly.
- Poll clones the incoming waker before acquiring the registry mutex. Replaced/completed/canceled wakers move out and drop after the registry guard releases. Collection snapshots bounded frontier data under the registry mutex, inspects storage outside it while holding authority, then takes the latest stored waker under the registry mutex. Owner paths release authority and all publication guards before invoking callbacks. Completion before first poll and replacement during inspection cannot lose a recorded terminal result.
- Actual processed-prefix coverage, canonical position at the captured height, incarnation, generation and branch earn completion. New acceptance cannot extend the target. Owner failure, missing history, invalidation and explicit stop produce typed results; no request performs indexing, blocks on a condition variable or forces a coins flush. Direct poison paths now use the same terminal-settlement helper, as verified in WR-03's closure.
- Post-initial unresolved work now returns Pending before immutable lookup. Final reads consume the token and check the same handle's incarnation/generation/branch/captured position, original requested hash, known height/parent and shared record integrity. Missing rows use request-time provenance even if the requested header connects while waiting; successful stored bytes still win. A final read cannot recapture a newer target or return Pending.
- Reviewed tests for fixed original target versus newer accepted work, ordinary unflushed owner completion, capacity/overflow/cancellation, completion before polling, latest-waker replacement, reentrant wake/drop, wrong request hash, distinct incarnations, raw integrity invalidation, genuine header-only provenance, reorg, publication faults, accepted-history faults, invalid required history, explicit stop and last-handle drop. Closure tests additionally cover the direct typed lifecycle/checkpoint/evidence poison paths with actual effect carriers.

Plan 05's initial 33-test result is superseded by 35 passing combined query/readiness tests after WR-03 closure, plus 47 managed-effect and eight checkpoint regressions with strict scoped lint/format success. Its premature-completion and wake-under-lock mutations each failed their intended test; direct-relay poison RED reproduced zero wakes before the fix. These are executor-reported results checked against actual source/tests; the reviewer did not run Cargo/tests. The separately sampled pre-Rust loader interruption and successful same-binary harness retry are host execution evidence, not a source deadlock finding.

## Incremental Plan 06 Assessment

**Reviewed:** 2026-10-09T18:41:28Z. **Open findings:** zero. Read the complete Plan 06 summary/plan and all fourteen exact Rust paths, including the six new production/test modules. Traced actual authenticated HTTP through scoped parsing, registry/context/dispatch, captured readiness completion and maintenance shutdown; cross-referenced pinned Knots RPC/request/HTTP source for the relevant transport contract.

- Both methods are exhaustively wired through supported names, serde names, baseline origin, node scope, typed calls, normalizers and real dispatch. BASIC uses the context's existing network handle; V0 returns its recognized disabled error before backend access. Projection uses exact lowercase filter/header fields with raw-header reversal once. Index summary allows only the pinned BASIC name/fields and selector behavior. Fixed internal errors discard private backend/authority/readiness detail.
- HTTP checks credentials before any new JSON parse, normalization or context acquisition. The scoped visitor deserializes the entire document before execution, preserves parameter-object pairs for only these methods, and retains existing collapsed-map behavior elsewhere. Existing request envelope validation precedes duplicate/name semantics; the dedicated normalizers retain named/default/null/arity/type/hash/filter ordering. Raw batches and notifications keep existing sequential execution, IDs, omitted notification responses and legacy/v2 status behavior. No unrelated envelope-policy change is claimed.
- Prepare captures the original request and configured handle under the context lock, validates node scope, clears request-local wallet scope and releases the guard before readiness await. Cancellation drops the barrier's owned registration. Finish consumes completion on that captured handle and original hash under a checked final read; it cannot loop, recapture a moving target or generate filters. The synchronous Value facade explicitly rejects Pending as requiring asynchronous dispatch; the authenticated HTTP product path uses the typed adapter.
- The existing maintenance owner propagates periodic failures into readiness stop, retains the first failure through subsequent ticks and attempts both final coins flush and stop on shutdown. Secondary settlement failures remain visible, worker signaling still joins, and existing daemon settlement only writes the clean marker after all worker settlements succeed. The full daemon server gracefully drains HTTP before worker shutdown; the direct worker-loop tests prove owner-stop settlement, while Plan 07 retains actual daemon lifecycle proof. This review does not infer a deadlock from that graceful drain order.
- Reviewed actual tests for unauthorized malformed input under a held context lock, password/cookie success, raw duplicate names and mixed collisions, malformed later syntax, missing-method envelope precedence, legacy/v2 IDs/statuses, batches/notifications, wallet-path rejection, initial available and accepted-missing rows, concurrent summary during Pending, ordinary owner completion with safe durability behind processed progress, terminal stop/disable/failure/real reorg, original-hash mismatch and public-error/auth-debug redaction. The HTTP History fixture genuinely accepts its blocks through the external shared node API; its empty raw-coins bootstrap is not treated as accepted validation authority.

Executor-reported evidence is 320 passing RPC library tests, 32 phase-focused tests, nine final HTTP tests, fourteen maintenance tests and strict scoped lint/format success. The deliberate context-lock retention mutation failed the concurrent-summary test and was restored. This reviewer checked the actual code/test behavior and performed scoped diff checks, but ran no Cargo/tests. Plan 07 was subsequently reviewed below; final consolidated native verification remains outside this limited clean result.

## Incremental Plan 07 Assessment

**Reviewed:** 2026-10-09T19:08:50Z. **Open findings:** zero. Read all nine source paths in `159-07-SUMMARY.md`, the DAEMON-PROOF report, the amended plan and its independent checker addendum. Traced configured fixtures, assertions and private injections against the existing genuine acceptance, query/completion and RPC projection paths.

- The configured fixture opens an empty store directly, genuinely accepts every block, saves bodies and flushes real coins before closing and selecting BASIC through actual configuration/store/runtime/context/HTTP entrypoints. The old raw-seeded History remains a separately named legacy negative control. Initial available/missing, later Pending with independent summary and actual maintenance-worker completion are exercised through authenticated HTTP on that configured authority.
- Historical input checking uses rotating coinbase scripts and a real same-block transaction dependency. The oracle explicitly asserts the expected historical and same-block spent scripts and proves output-only omission changes encoded bytes before comparing HTTP commitments. It independently checks input selection against accepted positions/undo, but uses the production `BasicFilterInputs`/BASIC generation and encoding implementation; it is not a second independent encoding algorithm or a live Knots oracle.
- Retention begins with 401 accepted blocks, heights 0–400. The actual serialized manual owner-plan deletes height 20 and the test asserts exactly one returned deleted hash, absent body and undo, durable `have_pruned`, reduced measured logical payload values and unchanged HTTP response. It then genuinely accepts original 401 without indexing it, validates eleven replacements at 391–401, and checks indexed stale 400, replacement active 401 and genuine accepted-but-missing stale 401 independently. The ready missing-row diagnostic survives actual reopen through retained acceptance, rather than a fixture validity flag.
- A live second Fjall open is required to refuse. Retention creates no maintenance worker; temporary snapshots are dropped, and `close(self)` consumes HTTP state, shared context, configured authoritative runtime, store and config before a fresh production configured open. Only path and block recipes survive. Original active/stale/pruned responses compare byte-for-byte after reopen, body/undo remain absent, and the BASIC summary is exactly synced at processed height 401.
- The ten node-private cases start from genuine empty-store acceptance, actual coins flush and configured recovered integrity, then call the same production owner/query/completion APIs used by RPC. All four publication boundaries, corrupt target/parent, missing parent/accepted target, backend read failure and raw-clone invalidation are covered. Inconsistent projections/ancestry refuse real reopen; no test forces a clean epoch. The per-store `BeforeQueryRead` variant/branch, mutation enum and mutation helper are all `cfg(test)` and crate-private; neither helper resets integrity, installs progress or mints acceptance. The normal raw API remains separately invalidating.
- The approved evidence placement is explicitly composed: private node owner/query fault outcomes plus the real production RPC mapper/error matrix and source links. Private injections did not run inside the normally compiled node dependency of daemon HTTP. Accessible auth/header-only/legacy/fence/reorg/disable/stop cases do run through configured authenticated HTTP. Test counts overlap and are not summed as distinct coverage.

Executor-reported evidence is nine configured HTTP tests, all 74 daemon regressions, ten new private-node cases, 35 reused query/readiness tests, five actual mapper tests and strict scoped lint/format success. Empty-prune and disabled-read-fault mutations each failed their intended assertion and were restored. The reviewer checked source/assertions and scoped diff cleanliness, without executing tests/builds. The reported 1,126-byte deletion is logical stored payload loss, not physical disk, fsync or hardware-loss evidence. Synthetic genesis, easy PoW, maturity one and explicit manual owner-plan deletion do not establish hard-coded network genesis, maturity 100, ordinary prune-RPC eligibility, automatic threshold behavior, public-network synchronization or funds safety. Graceful HTTP drain versus immediate explicit owner stop remains clearly distinguished.

## Final Plan 08, Scope and Simplification Assessment

**Reviewed:** 2026-10-09T19:29:59Z. Read the Plan 08 checkpoint/plan, full new guard/contracts/mutation sources, canonical breadcrumb checker and three Git regressions, current 157/158 guard changes, verifier routing and changed documentation/manifests. Scoped `git diff --check` produced no output.

- Exact breadcrumb registrations cover the 34 new Rust files. The two prior broad RPC direct-child globs were narrowed to existing explicit paths while preserving their anchors. Parity adds one CFRP-01/02 owner, removes no historical surface and changes the 157/158 rows only to follow the moved accepted implementation path. Status remains in progress with native/security/lifecycle gates and all later-phase/resource/legacy/evidence limitations explicit.
- The 157 connect and 158 reorg guard paths follow the relocated methods without losing their acceptance/preflight/mempool/fence contracts. Maintenance contracts follow first-failure retention and both final flush/stop outcomes; existing all-worker/join/clean-marker checks remain. Phase 159 test/check routes appear immediately after 158 in documented and executed verifier order. The default verifier still includes formatting, lint/build/tests, coverage/architecture checks and Bazel; focused guard results or fast mode do not satisfy that gate.
- Canonical breadcrumb inventory includes cached and nonignored untracked files before staging, and its three temporary-Git tests cover manifested, unmapped and ignored source. The final phase guard reuses the same inventory and adds tracked-ignore/gitlink controls. Fixed test/body/action contracts, supported owning configurations and independent mutations close WR-04–WR-06. Lexical contracts remain supplementary to actual Rust behavior and cannot independently establish parity or a native pass.
- Cargo/Bazel daemon and client target names match package BUILD files and the actual CLI/config entrypoints. UAT explicitly uses retained validated Open Bitcoin regtest data, cookie auth and replacement placeholders; it does not point at a Knots datadir or imply empty-store genesis installation. Timed long-running commands are explicitly isolated, with a separate build/start recipe for interactive clients so the cooperative build lock is not held by the daemon.
- The docs preserve exact normalization/output/error behavior, UnknownLegacy limits, genuine retention and composed private-fault evidence. Query logical lengths are separated from HTTP serialization/RSS, and synthetic codec capacity from a genuinely generated filter. Generated LOC was refreshed by the parent and read as a generated context artifact; no native freshness execution is claimed here.

**Explicit simplification pass:** The runtime reuses one configured store/authority and a std-only finite readiness registry; public errors use one real mapper and immutable rows need only a target/parent read. The guard's fixed path/contract child remains a reasonable separation. Closure now reuses the canonical Git inventory and a focused executable-evidence helper, removing duplicated traversal and keeping configuration/assertion checks bounded rather than expanding into a Rust interpreter. No broad runtime, transport or historical-guard rewrite is warranted.

The revised Plan 08 checkpoint records 27 reproduced bypass RED cases, followed by 555 passing guard tests/1,648 assertions (the prior 330 plus 225 closure controls), 1,057 historical mutation tests, three canonical inventory tests and the 1,055-file breadcrumb check. The reviewer inspected the actual fixes and independent assertions without rerunning those tests. Root has not supplied a full native pass at this review. Requirements, independent security/lifecycle acceptance and Git finalization remain parent-owned.

**Final closure recheck:** 2026-10-09T19:52:19Z. Read the complete new helper and changed guard/contracts/mutation/inventory sources, including combined cfg_attr handling and all fixed owner/action mappings. Reconciled Git source inventory again: 79 paths, no removed prior path and no production Rust change in this closure. Whole tracked-diff `git diff --check` remains clean. Final source status is clean with all six findings preserved as closed.

## Verification and Limits

### Post-Native-1 Delta: Phase 134 Mutation Insertion Marker

**Reviewed:** 2026-10-09T20:03:11Z. **Status:** clean; no new finding.

The parent reports native attempt 1 exited 1 after **7m30.555s**, before Cargo: the historical Phase 134 suite had 252 passing cases and five failures because the I/O mutations could not find their old insertion marker. This is a real failed native attempt, not a native pass or a production Rust behavioral failure. Root changed exactly two lines in `scripts/check-phase134-authoritative-lifecycle.test/mutations.ts`: the marker now matches `let outcome = apply_lifecycle_command(&mut network, command);`, and insertion adds a newline rather than the semicolon formerly needed after the returned expression.

Source review confirms all five unchanged statements—TCP connect, Fjall open, write_all, socket write and await—insert immediately after that current statement and before wake collection/`drop(network)`, while the actual authority guard remains held. The mutation still expects the unchanged storage/network-I/O diagnostic; `insertAfter` still asserts the marker exists, and `assertExactFailure` still requires exactly that diagnostic, with no suppression or waiver. No production implementation changed. Scoped diff cleanliness passes. The parent subsequently reports all 257 Phase 134 cases passed inside native attempt 2; that is parent execution evidence, not a rerun by this reviewer.

### Post-Native-2 Delta: Current RPC Documentation Reconciliation

**Reviewed:** 2026-10-09T20:13:42Z. **Status:** clean; no new finding.

The parent reports native attempt 2 exited 1 after **7m21.043s**, before Cargo: current-documentation reconciliation had sixteen passing cases and two failures because its fixed expected count was 27 and its RPC catalog omitted the two implemented Phase 159 methods. The earlier Phase 134 reconciliation passed all 257 cases in that run. Both native attempts remain failed attempts; no full native pass is claimed.

Reviewed only the three-file delta against the actual `SupportedMethod` registry/origin implementation. Static source counting confirms **29 unique serde method names**, **eight unchanged Open Bitcoin extensions**, and therefore **21 baseline-backed methods**. The catalog adds `getblockfilter`/`getindexinfo` only to the baseline group and updates the current count. Its query paragraph links the existing scoped contract and preserves UnknownLegacy/resource/evidence limits, default-off/dependency behavior, pending CFRP/native/security/lifecycle gates and all later-phase exclusions.

The checker changes only the fixed cardinality from 27 to 29: exact missing/extra set differences and baseline/extension grouping checks remain intact. The two new mutation controls establish a clean current fixture, remove each new catalog method independently, and require both its precise missing-method diagnostic and the baseline-grouping failure. Existing extra-method, missing-method and scope/relay/deferral controls are unchanged. Root reports twenty tests/45 assertions passed, plus the checker and Bright Builds with zero findings; the reviewer inspected the source/assertions and clean scoped diff without executing those checks. Root's active Rust preflight and subsequent full native run remain separate pending gates.

### Production CLI Adapter and Canonical Test-Module Delta

**Reviewed:** 2026-10-09T20:27:54Z. **Status:** clean for the assigned production/rename delta; unfinished guard extension excluded.

Root's workspace Rust preflight reached a real E0004 exhaustive-match error after the independently diagnosed host-loader boundary cleared: the existing production CLI adapter had no arms for the two new MethodCall variants. The source fix is confined to `client.rs` and three new client plus three new argument controls in the two existing test files. The reviewer did not run Cargo/tests. The queued test process was still at the pre-Rust loader boundary when assigned; it supplies neither a passing test result nor a semantic RED claim, and may read already-fixed source.

The adapter retains exhaustive typed matching, with no wildcard or broad Serialize change. `GetBlockFilter` reverses the parsed raw hash back to lowercase display order once and emits named `blockhash`/`filtertype` JSON; Basic and V0 remain explicit `basic`/`v0`. `GetIndexInfo` emits `{}` for omitted/null-normalized selection and exact `index_name` for empty or supplied selectors. Existing normalization handles positional, named and mixed inputs before encoding, preserving strict hash/type/name errors. Tests inspect asymmetric raw/display order, defaults/null/V0/mixed calls, exact index selectors and complete request envelopes. The client transport control exercises actual local HTTP with authentication and the root endpoint even when `-rpcwallet` is supplied, rather than only serializing a DTO.

The runtime header-history module is now the canonical `sync/runtime_state/tests.rs`, with unchanged three positive behavior controls and exact upstream breadcrumbs. The owning `#[cfg(test)] mod tests;` remains test-only; the old path is absent. Manifest/index, existing guard path/test/action references and Plan 03 metadata follow the rename. No production test panic is hidden through a new allowance. Root reports the existing live 117-selector guard and 1,055-file breadcrumbs pass after the rename; the panic rescan was still root-owned and running at assignment.

**Simplification pass:** Two small explicit wire-adapter arms reuse the existing normalizer and request builder, preserving domain types and all other exhaustive arms. The rename follows the established test-module convention instead of adding scanner suppressions. The remaining build-contract gap is known: the CLI binary is `test=false`, so the parent is adding an explicit native CLI-bin test step and focused guard coverage. That unfinished change is not approved here and requires the promised follow-up review and actual execution evidence before final source/native approval.

### Frozen CLI Guard and Explicit Native Binary-Test Follow-Up

**Reviewed:** 2026-10-09T20:34:32Z. **Status:** clean. This closes the source-review follow-up above; actual Rust execution/native gates remain pending.

Reviewed the frozen five-file extension in the phase guard, contracts, evidence helper, mutation tests and `verify.sh`. The source scope remains 86 paths. Six CLI controls join the fixed inventory, for **123 selectors** with minimum actions. The ownership graph has **71 external edges/76 owner files** and follows actual `main.rs → client.rs → client/tests.rs` and `lib.rs → args.rs → args/tests.rs` links. The resolver explicitly treats both lib/main as crate roots and preserves exact parent declarations/path resolution. Required CLI declarations/file configurations inherit the existing conservative supported-ownership checks; no test-only producer can supply production conversion.

Production contracts trace arguments through shared normalization, explicit conversion, method scope, authenticated HTTP and extraction. Literal checks inspect the actual function span, cross-check masked code offsets, and require exact blockhash/filtertype/index_name/basic/v0/lowercase/auth fields; a comment cannot supply those projections. Main route validation uses a `client` word boundary, rejecting the previously weak `detached_client::run_cli` substring case. CLI breadcrumb discovery reuses the canonical cached/nonignored inventory alongside node/RPC; no second filesystem policy is introduced.

The default verifier now explicitly executes `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin-cli --all-features` immediately after whole-workspace tests. This targets the entire hermetic RPC-client binary suite despite the manifest's `test=false`; it does not change default target policy or add a filter that could discover zero tests. Mutations reject command removal, `--no-run`, `-- --list`, an absent selector, missing workspace adjacency, conversion/hash/key/type/auth/node-route loss, fake-comment projection and detached entrypoint routing. Existing no-op/ignore/action/ownership generators also extend to the new selectors/edges.

Root reports six pre-extension bypasses reproduced in RED. The first focused extension run had forty passes and one exact-entrypoint weakness; the boundary fix then yielded **41 passes, zero failures, 122 assertions, 20.40s, 553 filtered**. The full expanded guard contains 594 tests but has **not** yet been completely rerun; the prior full 555-test result is not represented as a current 594-test pass. Live 123-selector guard, 1,055-file breadcrumbs, Bright Builds and shell checks are parent-reported passing focused evidence. The reviewer inspected code/assertions without executing tests. The CLI Rust process remains sampled at the pre-main host-loader boundary, so it provides no Rust harness pass or source deadlock finding.

**Simplification pass:** The resolver's existing root/override model is extended narrowly for the actual CLI crate split, rather than adding another ownership mechanism. The explicit native command closes the binary-test coverage gap without broad Cargo manifest changes. Shared normalization, inventory and assertion helpers remain reused; no broad serializer, wildcard adapter or production policy expansion is added. Final source review is clean with all six historical findings closed; full Rust/native/security/lifecycle acceptance remains root-owned.

### Post-Native-3 Recovery and Test-Fixture Delta

**Reviewed:** 2026-10-09T21:56:46Z. **Source assessment:** the seven-file frozen repair is sound; WR-07 and new test outcomes remain pending actual execution.

Root reports native attempt 3 failed after **1h5m5.214s**: the node library suite had **1,417 passes, ten failures and three existing ignored cases**, following **1,931 earlier passes** in that run. These counts are not a native success and are not summed as unique evidence. Four failures came from the sparse raw metadata regression in WR-07; other failures exposed stale historical fixture authority assumptions or source-selector paths. The root owns all recovery execution and any later full native rerun.

Group A changes only the production metadata-authentication helper and its coverage regression. Decoded authentic ledger fields are compared directly with the permissive raw DTO; absent/mismatch/non-ScriptsValid loses complete coverage durably. Actual stored-row decode/backend errors still propagate. The regression checks two sparse identity cases, live and real reopen, strict accepted-constructor refusal and positive history retention. This removes the fallible raw-identity parse without weakening any accepted identity or adding an authority factory.

Group B changes four existing test paths. Valid hidden immutable rows first demonstrably revoke append authority through actual raw writes, then use actual configured complete recovery/preflight while required inputs still exist. They do not install a fixture-selected clean proof. The catch-up reuse test earns a recovered-prefix turn before deleting body/undo and retains its one reuse/one generation/zero lag assertions. Local immutable-conflict, projection and predecessor controls inject explicit private backend tampering underneath a previously earned proof to exercise the original local validation boundary; they retain refusal, no-new-row and no-rewrite assertions. This is test-only external-corruption simulation, not a change to raw public-writer authority.

New `phase159_append_raw_clone_corruption_revokes_proof_and_genuine_recovery_refuses` separately exercises the real raw clone API, captured-proof invalidation, absent live proof, actual recovery refusal on a self-consistent row with an incompatible parent header, untouched corrupt bytes and no new immutable row. The test thereby keeps raw invalidation and local corruption validation as independent concerns. No integrity flag setter, forced recovery success or acceptance receipt is introduced.

The producer flush-policy assertion in `chainstate/tests.rs` now reads the two moved commit methods from `chainstate/validation_history.rs`; other producers remain sourced from `chainstate.rs`. Exact `fn persist(` and other open-parenthesis markers avoid the former accidental `persist_block` match, and assertions still forbid Always/Periodic in each actual producer body. Root removed one needless borrow in the private projection injection after the first recovery Clippy stopped before tests; this does not alter mutation semantics.

**Simplification pass:** The production change treats the persisted ledger as the single strict authority input and keeps raw DTOs permissive until conservative coverage invalidation. Tests reuse the existing full recovery route and preserve original refusal/no-effect assertions instead of weakening proofs or adding fixture constructors. Source selectors follow actual module ownership with exact function markers. No public/default activation, dependency, schema or broader runtime change is present.

No Rust GREEN is asserted for these repairs. Root's first recovery preflight exited 101 at the needless-borrow Clippy diagnostic before tests, then restarted formatting/full-workspace Clippy/build/tests and explicit CLI-bin tests in its serialized session. The 125-selector guard update was subsequently frozen and source-reviewed below. Close WR-07 only with appropriate actual discovered Rust regression evidence; whole-phase approval additionally requires full current guard execution and a native pass.

### Frozen Recovery Guard Follow-Up

**Reviewed:** 2026-10-09T22:02:12Z. **Source status:** clean; no additional finding. This closes the guard-review follow-up without closing WR-07's runtime-evidence gate.

The metadata contract now requires the actual same-store ledger lookup with `?` propagation, decoded identity, ScriptsValid status and exact hash/parent/height comparisons, then history-guard release before conservative invalidation. A separate check rejects reintroducing `BlockValidationIdentity::new` on permissive raw metadata in this helper. That exception is deliberately narrow: the actual accepted-identity constructor remains independently required to reject zero hash, self-parent and inconsistent genesis-parent/height, and accepted admission still requires the strict constructor. Source masks ensure comments/test-only code cannot provide the production checks.

The two new Rust regressions join the fixed test and minimum-action inventories, now **125 selectors**. Their actual owning paths are derived through **73 external edges/78 owner files**, including append's existing test module and child admission file. Body contracts require both sparse metadata variants, genuine acceptance, constructor rejection, public sink, real handle drop/reopen, retained positive/unknown states, raw clone invalidation and actual configured recovery refusal. The reusable append fixture must retain body/undo setup and real configured recovery; an artificial proof-installer mutation fails that contract. Existing executable/no-op/ignore/action checks extend to these selectors without a new authority mechanism.

Independent mutations reintroduce raw DTO strict parsing, loosen accepted construction, remove status/hash/parent/height binding, swallow ledger errors, move invalidation under the history lock, and replace actual recovery. Root reports the two initial guard REDs were rejected, then **18 focused passes/53 assertions/8.96s** and a separate owner control **one pass/three assertions/871ms**, plus live guard, 1,055 breadcrumbs, Bright Builds and shell checks passing. These are focused software-guard results, not Rust execution. The current complete guard has **611 tests and has not yet been fully executed**. Native attempt 3 did pass the prior 594-test guard before failing Rust; neither that older full result nor these focused results is a full current native/611-test pass.

**Simplification pass:** The revised contract follows the repaired strict-ledger/permissive-raw boundary instead of preserving an invalid constructor token. Accepted authority remains independently strict, and fixed ownership/action checks reuse existing helpers. No new path, runtime dependency, production hook, waiver or broad checker rewrite is introduced. Root's ordered Rust recovery verification remains running; WR-07 stays implemented/source-reviewed but pending actual discovered regression GREEN.

### Recovery-2 Execution and Paired Store/Manager Fixture Reopen

**Reviewed:** 2026-10-09T22:45:43Z. **Source status:** clean, conditional on the latest fixture's pending actual execution. Scope remains 91 paths.

Recovery run 2 exited 101 with **1,428 node passes, one failure and three existing ignored tests**. All WR-07 closure criteria and the other repaired append/source-selector cases passed. The one remaining catch-up reuse failure was independently reproduced with the already-linked exact binary: **zero passes/one failure in 12.59s**, returning `untracked replacement append positions` at the newly added worker turn. This is real test execution, not a loader inference. Store-only configured recovery had legitimately rebound its branch identity to the durable tip while the already-live manager retained its earlier branch identity; the mismatch correctly refused authorization.

The sole source delta is in `network/runtime_authority/filter_index/catch_up/tests.rs`. After preparing pure replacement/disconnect recipes and seeding the raw immutable row, the test still asserts captured append authority is revoked. It now drops the old staged preview and external store clone, consumes the fixture through its existing `ReorgFixture::reopen()`, reacquires the reopened clone and constructs the network from the rebound manager. Helper inspection confirms the old manager/store are dropped, a real Fjall open occurs, `initialize_configured` performs recovered startup, and `from_recovered_chainstate` binds the manager to that same store's coins parent, durable tip, generation and branch proof. No field setter, arbitrary generation or fake integrity epoch is introduced.

The same actual one-block owner turn then earns processed genesis before body/undo removal; reorg/reuse still requires exactly one reused record, one generation, zero accepted lag and absent common payload. Replacement/disconnect vectors and hashes are pure owned recipes, so they do not keep the old database open. This corrects test recovery sequencing without changing production authorization or weakening refusal assertions. The existing helper is supporting source context, not another changed path.

**Simplification pass:** Reopening store and manager together reuses the established paired recovery owner instead of manually synchronizing branch identities or weakening the valid authorization denial. Root owns the new ordered recovery run—format, strict full-workspace Clippy/build, no-run precompile, exact catch-up case, whole-workspace tests and explicit CLI-bin tests. Its outcome is pending. Root also reports the full **611-test guard passed with 1,816 assertions in 312.02s** before this minor fixture sequencing edit; TS contracts are unchanged. That supersedes the earlier focused-only guard receipt but is not a full native pass.

### Final Fixture Cursor Assertion and Exact-Case GREEN

**Reviewed:** 2026-10-09T23:05:07Z. **Status:** clean, source and exact-case execution follow-up complete. Scope remains 91 paths and all seven findings remain closed.

The final small delta removes the extra owner turn from `phase158_preflight_lagging_common_immutable_row_reuses_unavailable_payload`. Paired fixture reopen already restores processed genesis. The test now captures `fixture.records[0].identity()` before moving the fixture into the network and requires the read-only `maybe_basic_index_progress().maybe_processed_endpoint()` to equal **Some of that exact identity**. None, a different hash/header or a summary zero fallback cannot satisfy it. It installs no progress/epoch and does not advance the worker into the common height before its inputs are removed.

The actual paired reopen helper, raw-proof invalidation assertion, physical body/undo removal, validated reorg and original reuse-one/generation-one/lag-zero/missing-payload assertions remain intact. Reusing the restored cursor is the simpler valid fixture setup; it avoids both a synthetic cursor and the unintended extra turn. No production source or TS contract changed.

Read the parent's actual `packages/target/phase159-rust-recovery-4.log`: the exact catch-up test records **one passed, zero failed, zero ignored, 1,431 filtered, 0.87s** at lines 34–38, after formatting, strict workspace Clippy (14.63s), build (7.63s) and test precompile. This is a real entered-harness pass, not an inference from pre-main waiting. The reviewer inspected that receipt and the source without running Cargo/tests. Whole-workspace and explicit CLI-bin tests were continuing in the root's serialized session; no complete native-4 pass or benchmark/Bazel/coverage result is implied.

- Source-only review; no Cargo, Bazel, test, staging or commit commands were run, as required by the assigned scope.
- Reviewed existing meaningful tests for real acceptance, before/after history publication faults, coins/BASIC failure after absorption, every replacement position, preview/rejected stages, 128/129 admission, populated-mempool refusal, trusted 2,000-header admission and actual reopen. The new closure regression file now covers both initial raw-entry scenarios plus successful raw seeding, with explicit genuine-positive retention.
- Closure execution evidence recorded by the parent and revised Plan 03 summary: both initial scenarios failed RED with complete coverage live/reopen; GREEN had 29 history tests, 36 historical proof tests, zero scoped node Clippy warnings, and clean scoped rustfmt/diff checks. This reviewer verified the actual source and test assertions but did not rerun those commands.
- Compared RPC normalization and error ordering with pinned Knots `rpc/server.cpp`, `rpc/util.cpp`, `rpc/blockchain.cpp` and `rpc/node.cpp`; source-derived help tests are not a claim of a live Knots oracle run.
- Whole tracked-diff `git diff --check` completed without output, including final guard fixes. Full native verification remains the parent workflow's responsibility.
- Source files were left unchanged by this reviewer; only this report was updated. Whole-phase source scope and all six closures are reconciled. Subsequent source changes require an affected-path recheck; full native/security/lifecycle gates remain independent.

______________________________________________________________________

_Reviewed: 2026-10-09T23:05:07Z; seven findings closed, final fixture cursor correction and exact-case GREEN preserved_
_Reviewer: the agent (gsd-code-reviewer)_
_Depth: standard; final source review clean with all seven findings closed; continuing workspace/CLI/native/security/lifecycle gates pending_
