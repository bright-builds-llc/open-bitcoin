---
phase: 165-shared-exact-work-authority
reviewed: 2026-10-17T06:36:07Z
client_date: "2026-10-17"
depth: standard
review_kind: preliminary_source_only
review_state: completed_scoped_source_review_before_pin_renewal
status: clean
implementation_paths: 13
files_reviewed: 9
files_reviewed_list:
  - packages/open-bitcoin-rpc/tests/support/phase165_live_source.rs
  - packages/open-bitcoin-rpc/src/inbound_listener/tests/peer_filters/phase162.rs
  - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/phase162/fixtures.rs
  - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/phase162/inbound.rs
  - scripts/check-phase164-canonical-identity/fixture-source.ts
  - scripts/check-phase164-canonical-identity.test.ts
  - scripts/check-phase165-shared-exact-work.ts
  - scripts/check-phase165-shared-exact-work.test.ts
  - scripts/check-phase159-filter-rpcs/evidence.ts
findings:
  critical: 0
  warning: 0
  info: 0
  total: 0
generated_by: gsd-code-reviewer
lifecycle_mode: yolo
phase_lifecycle_id: 165-2026-10-14T16-09-59Z
---

# Phase 165: Preliminary Plan 19 Source Review

The completed pre-pin source review is clean within the frozen nine-file scope: zero actionable correctness, security or test-reliability findings. Final closure below supersedes earlier preparation state while preserving its exact fragments, hashes and intermediate observations. Supplemental literal tables/breadcrumb ownership were also inspected as recorded. Current pin/inventory renewal, complete guard matrices, C3 and whole-phase/native/formal gates remain separate. No source/shared-state/Git edits, tests, builds or heavy Bun commands were performed by this reviewer. Only this report is written.

## Approved Boundary

The current Plan19 SHA-256 independently matches approved `87c0448c047af15e0f5df37c138e60e5a9f96fb75040c2150c96be8acabbec03`. Corrected C planning/recheck preserves the earlier receipt blocker and accepted bounded thirteen-owner risk, with sequential25→19→26 execution. C3's frozen historical function-contract correction belongs to26; Plan19 cannot silently edit those other tools or claim whole C/native completion.

Root AGENTS, Bright Builds sidecar, placeholder-only overrides and architecture/code-shape/verification/testing/Rust guidance were loaded in this ongoing review session. Both active lesson files were read completely within7,188 bytes/2,397 conservative tokens; no archive was loaded. Existing source/raw historical attribution is distinct from current runtime qualification.

## Independently Authenticated Old Daemon Bytes

The current daemon fixture is15,320 UTF-8 bytes, SHA-256 `933da5cfb57612c9e93a231967a17da3cc0163e1badcd5f7d44debde78868b18`. It matches the authenticated pre-C row. Existing `fixture-source.ts` is33 lines, SHA-256 `24224f0dea4e405d3a148a9fe148255884bc7c575092ad8b0c67507d157dbff2`.

The old prelude/receipt span begins at the unique indented `let runtime = DurableSyncRuntime::open(` inside `seed_count` and ends immediately before the following disabled-progress `assert!` whose first expression is `runtime`. Its exact length is1,095 bytes and SHA-256 is `b01011aa8b2703d2cc3fcb8f63963d9b291891e43bfbd23db2cd598e007b6f25`. The following is the literal old source, including eight-space indentation and a newline after its closing brace:

```rust
        let runtime = DurableSyncRuntime::open(
            FjallNodeStore::open(&path).expect("fresh real Fjall"),
            SyncRuntimeConfig {
                network: SyncNetwork::Regtest,
                ..Default::default()
            },
        )
        .expect("unindexed actual runtime");
        let params = runtime.config().network.consensus_params();
        let recipe = recipe::Recipe::new(params, ScriptVerifyFlags::P2SH, count);
        let mut receipts = Vec::with_capacity(recipe.blocks.len());
        for block in &recipe.blocks {
            runtime
                .store()
                .save_block(block, PersistMode::Sync)
                .expect("ordinary body persistence");
            let receipt = runtime
                .network_handle()
                .connect_local_block(block, recipe.flags, recipe.params)
                .expect("normal-maturity managed acceptance");
            assert_eq!(receipt.height as usize, receipts.len());
            assert_eq!(receipt.block_hash, block_hash(&block.header));
            receipts.push(receipt);
        }
```

The existing current import fragment occurs exactly once:

```rust
use crate::AuthoritativeNetworkRuntime;
use crate::tests::fixture_runtime::{
    fixture_open_authoritative_network_runtime as open_authoritative_network_runtime,
    fixture_open_runtime_store as open_runtime_store,
};
```

It inverses only to this exact original import:

```rust
use crate::{AuthoritativeNetworkRuntime, open_authoritative_network_runtime, open_runtime_store};
```

The second existing exact substitution inverses `ManagedRpcContext::from_fixture_runtime_config_with_network_handle(` to `ManagedRpcContext::from_runtime_config_with_network_handle(`. Each current fragment occurs once and its ordinary counterpart is absent. Performing exactly these two substitutions in memory produces15,190 bytes whose complete SHA-256 is independently `faa009380301d6e6acbf032fd814f5cf9e31ddcbdb629787477fe5fbc5d62615`, the required unchanged original whole-source identity. No adapter module was executed to obtain this proof.

The new implementation must add only the complete exact helper include after `use super::{core_types, recipe};` and the complete exact replacement prelude/receipt loop. Review must pin the final new fragments only after they actually exist. The original hashes above are constraints, not values that may be refreshed to fit the new source.

## Exact Inverse Review Checklist

- Retain original-historical and earlier exact two-substitution classification behavior.
- Require each complete finalized new include/prelude fragment exactly once, with fixed surrounding anchors. Partial, duplicate or mixed old/new wiring must not map to a historical claim.
- Remove only the complete reviewed include and replace only the exact reviewed new prelude/loop with the1,095-byte original above; reverse only the original two import/context substitutions.
- Require the entire reconstructed source to hash to the unchanged originalFAA identity. No regex block stripping, symbol-based deletion, unknown-current substitution, arbitrary source discovery, I/O or broad normalization.
- Unknown/tampered source must return raw input unchanged as `historicalSource`, raw SHA as both hash fields and `evidenceClass: unproved`.
- `historicalFixtureView` must copy the map, leave raw current inputs intact and affect historical claim attribution only. It must not silently replace executable current contracts.
- Actual new source must produce the expected positive RED against the old adapter before implementation; final positive must reconstruct exact original bytes and reclassify original-historical.
- Independent present-anchor mutations must cover recipe/count/flags, new and untouched loop/assertion bytes, include path/body, constructor/mode/actual receipt/descendant call, aliases/context, unrelated comment/newline, missing/duplicate/mixed fragments. Require unchanged positive baselines and targeted classification failure, not an incidental missing-input error.
- Current164 and current165 must independently pin the same reviewed new adapter SHA; no pin renewal precedes exact source review. Preserve old adapter attribution, manifest/snapshots/recipes/process/oracle/receipt bytes and all default native registration identities.

## Rust Behavior and Closure Checklist

The new recognized `tests/support/phase165_live_source.rs` does not yet exist. It must be test-only, included locally and immediately breadcrumbed. Its six-argument API must receive full caller parameters/config/flags/mode, return an actual managed genesis `ChainPosition`, accept with normal Regtest maturity100, flush the SAME core with actual genesis/undo/positions and empty header entries, and call the public seven-argument constructor. No Node cfg-test helper, count normalization or raw snapshot promotion is permitted.

The shared descendant helper must validate actual accepted parent and contiguous new nonroot ancestry, perform public modeled Version/Verack/Headers under identical parameters, observe exact admitted identities and disconnect its peer. It must not connect/save bodies, rebind the populated root, import raw headers or manufacture historical tie order.

RPC fixture review must preserve the unchanged102-block recipe, exact actual genesis/suffix receipt height/hash, safe101, maturity100, h101 three-transaction shape, historical/same-block undo/spent scripts, wallet and fork script assertions, independent oracle frames and every physical permission/partial/deadline/cancel/epoch/queue/future-drop/socket/resource/cleanup condition. Shared setup must reduce its current627 lines without deleting substantive assertions.

Daemon seed review must retain count1001/102 recipe bytes, real genesis receipt and each suffix receipt, Disabled BASIC observation, actual final flush/B/undo/spent-script assertions and every byte outside the narrowly approved wiring fragments. `Phase162Handoff` currently contains only path/recipe/provenance; it must remain handle-free. `close()` explicitly drops state, shared context, opened runtime, store and config before returning the handoff.

No-grant review must preserve all102 original ordered JSON block inputs, including genesis at index0, exact digest/provenance, maturity100 and actual flush. Runtime, handle, store and listener ownership must be released before raw handoff/reopen. All three denied request families must retain fresh physical connections, joined/cancelled listeners, permission decisions, no unsolicited reply, resource and cleanup assertions. Its current628 lines leave no room for duplicated setup or broad formatting tricks. Ignored long measurement remains historical, not newly executed qualification.

## Async Ownership Review Checklist

The actual RPC fixture contains11 `#[tokio::test] async fn` declarations; daemon inbound contains one. Current165's enabled counter and named-attribute mutator currently recognize only `#[test]` immediately before the function region and use `lastIndexOf("#[test]")`. Registering zero or omitting these actual enabled cases is unacceptable.

Frozen `scripts/check-phase159-filter-rpcs/evidence.ts` already strips public/async/const/unsafe/extern qualifiers before its attribute/owning-module checks. Its SHA-256 at preparation is `2f230b7b713a64bd8e538a16bf8a85db22902eb001f4e85beac9e82c175c302e`; it stays outside edit ownership. Fix recognition/mutation only in the owned current165 paths.

- Recognize real ordinary and Tokio async test attributes, including legitimate macro options, without counting strings/comments/helper functions.
- Preserve owning file/module cfg/ignore checks through the existing frozen159 API.
- Use exact calculated declaration/attribute offsets; a second named async mutation must preserve every earlier byte/test.
- Attribute per-owner enabled counts to actual declarations and execution, not a zero-case or disabled run. The helper-only source must not receive an invented test count.
- Loaded-source negatives must independently cover removed attribute, `ignore`, reasoned ignore, cfg and file disablement, plus exact macro-option/async handling and a later named declaration.
- Keep executable test declarations in main.test.ts. The only approved capacity move is ordered data-only transfer of the existing45 downstream mutation rows to owned contracts.ts, proving identical values/order. No new helper module, deleted test, cap exception or frozen159 change.

## Preparation Source Identities

These are pre-implementation hashes, not closing identities. Plan25 may still change its owned current165 guard paths before19 starts; that drift must be attributed to the actual predecessor rather than mistaken for19 changes.

| Owned path                                                                                | Lines  | Preparation SHA-256                                                |
| ----------------------------------------------------------------------------------------- | ------ | ------------------------------------------------------------------ |
| `packages/open-bitcoin-rpc/src/inbound_listener/tests/peer_filters/phase162.rs`           | 627    | `abe2651ff16c3d872695e8bde1222de63c2b5a28c519ffa9da299ac9166ab8b6` |
| `packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/phase162/fixtures.rs` | 444    | `933da5cfb57612c9e93a231967a17da3cc0163e1badcd5f7d44debde78868b18` |
| `packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/phase162/inbound.rs`  | 628    | `1d991eeabfe62e1722828a9cdeb79fbf9ebc122b7a9e0fdb3b7d6ac0b85e7c60` |
| `packages/open-bitcoin-rpc/tests/support/phase165_live_source.rs`                         | Absent | Planned new source                                                 |
| `docs/parity/source-breadcrumbs.json`                                                     | 3,761  | `c3aacf47b0e514adb8f65ad1d08312b0a52f30e6746925986045a60dffa2ce68` |
| `scripts/check-phase164-canonical-identity/contracts.ts`                                  | 485    | `d0d547888bc3ff98bae7b3f0b643c4fe48fad6275bb5a7f70e86227c2207afd1` |
| `scripts/check-phase164-canonical-identity/source-files.ts`                               | 137    | `5cd178af07ea7f27204a2586d94bf8b1731dc6fd6b32c7b1ded0e07b6fc32832` |
| `scripts/check-phase164-canonical-identity.test.ts`                                       | 561    | `fb0a15cd8e318e2d1d89d80abee594e48068f1604496c484d1b3d8dbb27ccb23` |
| `scripts/check-phase165-shared-exact-work.ts`                                             | 218    | `4489067649e5b0f7677e5c59eb52580a63f60a92957900f566158c80024a44ac` |
| `scripts/check-phase165-shared-exact-work/contracts.ts`                                   | 212    | `e20054073829206db19e1719a9806f2c769e751736390ef2ac0467a65d7f6af9` |
| `scripts/check-phase165-shared-exact-work.test.ts`                                        | 616    | `1640d5d075ed7418a5f136ffd6637cd5af3f8c734411a06085087c93064640b8` |
| `scripts/check-phase164-canonical-identity/fixture-source.ts`                             | 33     | `24224f0dea4e405d3a148a9fe148255884bc7c575092ad8b0c67507d157dbff2` |
| `scripts/check-phase164-canonical-identity.ts`                                            | 243    | `1d2990c348e4e9ef367f38320475266e300cbdab98de38693b67abc9d3e5e0f8` |

No concrete missing-owner risk was found beyond the explicitly declared new helper/breadcrumb and the documented thirteen-owner/cap discipline. Future implementation may expose a narrower conflict, which must return to root before edits expand. Actual runtime, inverse-negative, async and full guard evidence remain prospective. C3 and final default native/source/security/goal/normal-hook/Git gates remain separate.

## Task1 Independent Source Delta Review | 2026-10-17T06:25:36Z

**Task1 source result: no actionable findings at the identities below.** This is a bounded delta review of the new shared public helper, RPC inbound adapter and its breadcrumb registration. Daemon spans, exact inverse, async recognition, guard pin renewal and whole Plan19 source stability remain pending and are not certified by this appendix. No tests/build/heavy Bun or source/shared-state/Git mutation was performed by the reviewer.

The complete156-line helper uses only normally compiled public Node/network/core APIs. It consumes `Chainstate::new()` into the actual empty Fjall target, retains actual managed genesis acceptance under the full supplied parameters/flags, and returns the real `ChainPosition`. Extraction and real `FlushLifecycle::execute_flush` retain the SAME core/sink with actual undo/positions/genesis body and empty header entries; durable genesis B is checked before normal public constructor assembly. Supplied config/full parameters/mode remain intact, and relay/inbound arguments stay default-off. It contains no Node cfg-test adoption, count writer, snapshot promotion, missing-input restoration or alternate validator. Local extra store clones do not escape except through the returned runtime's own normal ownership.

The helper's header path requires an actual admitted parent, only new nonroot headers and contiguous ancestry. It performs normal public Version/Verack/Headers under full supplied flags/parameters and actual batch time bounds, checks the resulting identities and disconnects the modeled peer. It does not connect/save bodies, rebind the constructor graph, raw-save headers or mint tie order. Both helper functions are included only within existing test modules via the recognized support path. The comment change from an unsupported inner doc to an ordinary source comment is a compile repair with no runtime effect.

RPC `fixture` retains explicit full Regtest parameters/P2SH and the immutable102-block recipe. It calls the helper Enabled, admits only the known nonroot suffix, then selects the returned genuine receipt at height0 and performs actual original body persistence/connect at every later height. Existing height/hash assertions cover both genesis and suffix; normal final indexing/flush/safe101 and maturity100 remain. The shared helper removes duplicated raw intermediate reopening without changing the later physical test behavior.

Exact comparison against authenticated pre-C source proves `fn oracle` byte-identical and the entire RPC source from `let network = runtime.network_handle();` after setup through EOF byte-identical. Thus final catch-up, real undo/historical/same-block scripts, wallet/fork facts, independent frames and all physical permission,13/31-byte partial, deadline/queue/cancel/epoch/disconnect/future-drop/resource/cleanup assertions remain unchanged. The11 actual Tokio async declarations remain intact for the later current165 ownership work; this appendix does not certify the not-yet-implemented recognizer.

The immediate breadcrumb registration is exactly one `phase165-rpc-live-fixture-source` row containing only the new support file and the matching `validation.cpp` and `node/blockstorage.cpp` anchors. The file's comment names the same anchors. No new path or exemption was introduced beyond the plan.

| Task1 path                                                                      | Lines | Reviewed SHA-256                                                   |
| ------------------------------------------------------------------------------- | ----- | ------------------------------------------------------------------ |
| `packages/open-bitcoin-rpc/tests/support/phase165_live_source.rs`               | 156   | `51d11bc50395c7b1b074f936c77752e1b7f997652dfc42cca03b990da8368248` |
| `packages/open-bitcoin-rpc/src/inbound_listener/tests/peer_filters/phase162.rs` | 626   | `4c8250c11f1a2654b5c70c6c7594f238b5b68777a35992b6bb15933ff1e9951b` |
| `docs/parity/source-breadcrumbs.json`                                           | 3,771 | `8203a2a1a6923fe680a454b253d2be5745f2ed0d1d46c7e420d82736ec65809b` |

Both Rust owners fit628 lines. Scoped whitespace inspection found no issue. The parent reports actual RPC baseline9 passed/11 failed/one existing ignored with exit101, then20 passed/zero failed/one existing ignored. Those are executor receipts, not reviewer execution. Other Plan19 files are being implemented concurrently; no whole-source freeze or completed plan verdict follows. Subsequent Task1 byte drift requires review before current pin renewal.

## Narrow Inbound Cap Simplification Check | 2026-10-17T06:27:29Z

**PASS for the proposed same-file pure duplication removal.** The formatted daemon inbound adapter currently reaches638 lines; no assertion removal or cap exception is authorized. This approval concerns a concrete equivalence-preserving simplification before implementation, not final source/test acceptance.

Selecting `expected_exchanges = if granted { exchanges.as_slice() } else { &[] }` once, then retaining the existing frame-length sum and flattened byte-length sum over that borrowed slice, is equivalent to both original conditional expressions. Granted sessions sum exactly the same expected frames/bytes; denied sessions sum an empty slice to the same two zeros. The borrowed slice is used only for totals, and its borrow ends before the unchanged `exchanges.into_iter()` move into the async client. The final `(frames, bytes)` equality and separate fixed20/0 assertion must remain.

If further reduction is needed, an immutable borrow of `fixture.opened.network` after client outcome/abort handling may replace only the repeated cleanup peer-observation and final released-charge access chains. It creates no cloned store/authority owner, does not acquire a guard and does not extend the already-existing `&fixture` lifetime. The actual client timeout, abort-and-await, peer-disconnect polling, bounded listener shutdown/join and subsequent physical/frame/resource assertions must retain their exact current order.

No source change to the historical daemon fixture projection is needed for this math/access simplification, which stays inside declared inbound.rs ownership. Final Rust formatting/cap check and actual measured-case verification remain required; this source-only review ran no tests or builds and edited only this report.

## Daemon and Exact Inverse Source Delta Review | 2026-10-17T06:31:55Z

**Bounded source result: no actionable findings in the completed daemon/inverse delta.** This does not issue a final whole-Plan19 verdict or approve pin renewal while current165 async recognition is still being implemented. TypeScript/JavaScript standards were additionally loaded for the pure attribution code. No application/checker module or test suite was executed by this reviewer; independent proof used source parsing, exact byte replacement and hashing only.

### Daemon Rust and Physical Preservation

The new seed prelude uses explicit full Regtest parameters/P2SH and the existing immutable count1001/102 recipe. Shared source assembly is Disabled, matching the former default, and returns the actual genesis receipt. The known initial suffix is admitted normally; the loop still visits every original recipe block, uses that returned receipt at0 and performs actual original body persistence/connect at each later height. Receipt height/hash and pushes remain. Exact comparison proves the disabled-progress assertion immediately after the reviewed seed span through EOF byte-identical to authenticated pre-C source: original final flush/B/actual undo/spent-script, oracle/digest/provenance, open modes, physical and handle-free close behavior are preserved.

No-grant setup likewise uses the Disabled shared helper and retains all102 original ordered JSON input pushes, including genesis at index0. Full maturity100, actual genesis/suffix receipts, final flush, exact digest construction and explicit runtime/handle drop precede raw handoff. `Phase162Handoff` remains only path/recipe/provenance; it retains no runtime/store/authority owner. The original three physical denied-family requests, new sessions, permission state, listener shutdown/join, resource checks and all-handle cleanup remain intact.

The implemented `physical_session` cap simplification matches the approved pure equivalence: one borrowed expected slice feeds the exact original frame and byte sums; denied stays zero, granted sees every same expected item. Client transport/fault handling, timeout/abort-await, cleanup polling, bounded shutdown/join, outcome tuple and fixed20/0 checks remain in their original order. The later immutable network borrow adds no owner or guard and only replaces duplicate field access. Final inbound is exactly628 lines; fixture460. No historical fixture bytes outside the exact include/seed changes are normalized to hide this separate inbound simplification.

### Independent Exact Attribution Proof

All four expected current fragments occur exactly once. The literal finalized include has146 bytes/SHA-256 `1d94aee6942c500754957b01d815b66a251159eef672563b65d341aa34b3346e`. The complete new seed span has1,516 bytes/SHA-256 `12ca04c9a6fedda9c9a86a0d94045934d31d969ee90e3edc0cbd0a58c5b9aa2e`. The decoded old seed literal independently remains1,095 bytes/SHA-256 `b01011aa8b2703d2cc3fcb8f63963d9b291891e43bfbd23db2cd598e007b6f25`.

Removing only that complete new include and replacing only that complete new seed literal with the exact old literal reconstructs the entire authenticated pre-C933da5cf source byte-for-byte. Applying only the prior unique import/context inverse then produces exactly15,190 bytes with the unchanged original SHA-256 `faa009380301d6e6acbf032fd814f5cf9e31ddcbdb629787477fe5fbc5d62615`. The literals were decoded directly from source as data; no adapter code was run for this independent proof.

The actual inverse requires unique complete include/seed matches, no old seed mixed into the new shape, and exact surrounding include/prelude/disabled-assertion anchors before reversing those two fragments. Its shared historical step independently requires exactly one current import/context substitution and excludes ordinary counterparts. It returns a historical claim only if the complete reconstructed source hashes to the fixed original. If any new-shape condition fails, the candidate remains raw; the earlier exact two-substitution path still cannot authenticate mixed/partial/duplicate/unrelated bytes because the same complete original hash is mandatory. No broad stripping, unknown-current replacement, I/O, discovery or source rewriting is present.

Original-historical and pre-existing current933da5 classifications remain. Unknown output retains the raw input as `historicalSource`, raw current SHA in both hash fields and unproved classification. `historicalFixtureView` constructs a separate map; the current source/input map remains unchanged. This is historical-only attribution, not runtime qualification or executable-contract projection.

The26 mutation rows all have exactly one present current-source anchor. They cover removed/duplicate/mixed seed/include, loop/assertion/recipe flags/count, include path/body, constructor/mode/genesis receipt/descendant admission, receipt push, prior aliases/ordinary context and unrelated comment/assertion/whitespace. The positive test starts from the actual edited source, verifies exact original reconstruction/reclassification, verifies prior933da5 source classification and proves the input map unchanged. Every negative requires raw unproved output and its exact own hash. The parent reports actual old-adapter27 failures and final27/0 with166 expectations; those are executor results, not reviewer-executed tests.

### Current Reviewed Identities

| Path                                                                                      | Lines | SHA-256                                                            |
| ----------------------------------------------------------------------------------------- | ----- | ------------------------------------------------------------------ |
| `packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/phase162/fixtures.rs` | 460   | `2d0a7e7d93034333e00f5c90977693bcfb7a03ff8ab704bf834d91fec680d307` |
| `packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/phase162/inbound.rs`  | 628   | `d89e0e47cd23861c17706ab36be8d6efb92528fa6e82f9ddfd8a731d774652ca` |
| `scripts/check-phase164-canonical-identity/fixture-source.ts`                             | 47    | `dbafae5c3fa1e524251a4304860b4b7b99adbcbda054db3ea9f788acb21aa26b` |

Task1 helper/RPC hashes were independently recaptured and remain exactly `51d11bc50395c7b1b074f936c77752e1b7f997652dfc42cca03b990da8368248` and `4c8250c11f1a2654b5c70c6c7594f238b5b68777a35992b6bb15933ff1e9951b`; their preceding clean scoped delta applies unchanged. The parent reports daemon14/0/one pre-existing ignored in45.72 seconds. This is executor evidence only. Async recognition, final source/hash stability, affected pins/full matrices and C3/native/whole-phase gates remain pending.

## Final Frozen Source Closure Before Pin Renewal | 2026-10-17T06:36:07Z

**CLEAN: zero critical, warning or info findings in the frozen source scope.** The exact map captured at2026-10-17T06:35:38.627Z in `.local/open-bitcoin-dev/phase165-plan19-source-review-freeze.json` has nine ordered entries. Each current complete-file SHA was independently recomputed and matches. The four Rust owners and exact inverse remain byte-identical to their preceding reviewed identities, so those actual source proofs and original-assertion/handle-lifetime checks apply unchanged. This closes source review before affected pin renewal; it is not a current full-checker pass or completion of C3/native/formal work.

### Async Registration and Exact Attribute Mutation

The current165 recognizer locates attributes by stripping only trailing declaration modifiers from the offset-preserving masked prefix. It leaves actual source modifiers intact. An ordinary `#[test]` counts only a non-async declaration; an async declaration requires exactly one recognized Tokio registration. Raw attribute text is recovered from the exact masked offsets, preserving quoted option values without counting comment/string lookalikes.

Supported Tokio options are bounded to flavor, worker_threads and start_paused. Unknown names, duplicates, invalid flavors, zero/nondecimal worker counts, workers without multi_thread and paused mode outside current_thread refuse. Default current_thread and supported explicit option forms remain recognized. No attempt is made to accept arbitrary macro configuration. The current actual owners have11 RPC and one daemon `#[tokio::test] async fn` declarations, independently counted from source.

The existing frozen159 `supportedOwnership` still checks owning declaration, file and inline-module cfg/ignore restrictions after stripping async/public qualifiers. Its hash remains unchanged. Current165 adds explicit reasoned-ignore detection over exact declaration/file attribute ranges, because the old helper's `ignore(...)` handling alone does not include `ignore = "reason"`. This is local bounded logic, not a frozen159 edit or bypass.

Named mutations replace only the computed `[declaration.start, declaration.end)` attribute range, preserving the public/async modifier suffix and all earlier bytes. The second named async control checks both earlier-source equality and retained `async ` before the selected function. General ownership controls retain ordinary removal/ignore/cfg, add actual reasoned Tokio ignore, and keep file-level disablement and commented-cfg positives. New macro option controls mutate the actual first async declaration and require owner-attributed recognition/refusal, rather than an unrelated missing-input finding.

Two added full include declarations are literal actual source blocks in the existing unique-declaration loop: RPC `mod live_source { include!(...) }` and daemon `pub(super) mod live_source { include!(...) }`. Each complete block occurs once at its real owner. Removed/commented/cfg-disabled/redirected controls retain owner attribution. There is no new test module, source-file fallback or import-time data discovery.

### Exact Ordered Data Move and Harness Boundary

The saved original45-row mutation-table literal and current `DOWNSTREAM_SOURCE_MUTATIONS` literal are byte-identical and retain all45 ordered values. Independently parsing those literal four-string rows and serializing the ordered array without whitespace produces SHA-256 `109d9edbcc860289fadf873395bd85e2355e9ccf4f56ef100f30053c99d1a936`, exactly the recorded value. The raw multiline literal has a different SHA, `ed67fb8af788ed89333bbec58ba143097797d134311fdd8bfa3412bdc6b9bc18`; the serialized-value identity is not mislabeled as the raw file/literal hash.

Only data moved into the already-owned contracts module. The same one executable loop in main.test.ts imports/iterates that array, selects the same function, confirms the present anchor, mutates that region and requires the same owner/rule. Main.test.ts remains606 lines; no executable test declaration or negative control was removed to fit the cap.

The mutation helper reads only `real.get(file)`, asserts the preloaded source and present changed anchor, then returns a copied map with the replacement. It performs no disk fallback or broad source reads. Existing bounded loader/pre-read inventory, manifest import, original history, claim and native-registration tests remain present. The separate isolated-loader controls retain their own existing explicit filesystem purpose; the source mutation harness itself is map-only.

### Frozen Scope and Evidence

| Frozen path                                                                               | SHA-256                                                            |
| ----------------------------------------------------------------------------------------- | ------------------------------------------------------------------ |
| `packages/open-bitcoin-rpc/tests/support/phase165_live_source.rs`                         | `51d11bc50395c7b1b074f936c77752e1b7f997652dfc42cca03b990da8368248` |
| `packages/open-bitcoin-rpc/src/inbound_listener/tests/peer_filters/phase162.rs`           | `4c8250c11f1a2654b5c70c6c7594f238b5b68777a35992b6bb15933ff1e9951b` |
| `packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/phase162/fixtures.rs` | `2d0a7e7d93034333e00f5c90977693bcfb7a03ff8ab704bf834d91fec680d307` |
| `packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/phase162/inbound.rs`  | `d89e0e47cd23861c17706ab36be8d6efb92528fa6e82f9ddfd8a731d774652ca` |
| `scripts/check-phase164-canonical-identity/fixture-source.ts`                             | `dbafae5c3fa1e524251a4304860b4b7b99adbcbda054db3ea9f788acb21aa26b` |
| `scripts/check-phase164-canonical-identity.test.ts`                                       | `a13cb68af20f25669d6bdffb4657cf6c426272872c6596109a121314f577abb3` |
| `scripts/check-phase165-shared-exact-work.ts`                                             | `82546042898b5649eb7c2e97b3ee8ea3b55d4eb36dcf0790218f4ee825f7eac9` |
| `scripts/check-phase165-shared-exact-work.test.ts`                                        | `e5291503ac810c8b44791f6d0f7fdd24111a368115fd2157add5df7fffb25af9` |
| `scripts/check-phase159-filter-rpcs/evidence.ts`                                          | `2f230b7b713a64bd8e538a16bf8a85db22902eb001f4e85beac9e82c175c302e` |

Ordered frozen-scope digest: `d93578a6645419aeb8474557df43176f85ec5871a9e54df49fcd180e92ac0982`. Construction is SHA-256 over the nine map entries in saved order, each as UTF-8 path, tab, lowercase complete-file SHA, newline. Supplemental data-only current165 contracts identity at this check is `5c3072aa2be8790a386cd4865fb0380bb375d0c55bb0915c3895f9b47ca9e1b5`; it is outside that nine-entry frozen digest because pending owner/pin rows will subsequently change it. New breadcrumb registration was separately reviewed above. A later source change requires delta review; expected guard-row/pin-only changes require exact renewal attribution and actual full matrices.

The parent reports actual async recognizer RED with one failure, followed by8 passed/zero failed with79 expectations; strict format/RPC all-target/all-feature Clippy in12.78 seconds and build in5.07 seconds exited zero. Earlier actual RPC20/0/one existing ignored, daemon14/0/one existing ignored, and exact inverse27/0/166 expectations remain executor evidence. No test/build/heavy Bun was run by this reviewer. All reviewed changed Rust/TS files satisfy628 lines, including inbound628, RPC626, current164 test598 and current165 test606; scoped whitespace inspection found no issue.

Source review permits the next separately authorized precise pin/inventory renewal; it does not certify unexecuted current full matrices, C3's frozen legacy contract bridge, whole native/coverage/source/security/goal review or Git finalization. Original historicalFAA identity, the authenticated pre-C bundle and frozen159 remain unchanged. No ordinary selector/bootstrap/exact raw work reopen, contextual completeness or pinned-Knots qualification is claimed. Only this report was changed by the reviewer.
