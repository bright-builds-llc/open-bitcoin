---
phase: 165-shared-exact-work-authority
reviewed: 2026-10-17T06:02:55Z
client_date: "2026-10-17"
depth: standard
review_kind: preliminary_source_only
review_state: completed_preliminary_source_review
status: clean
files_in_scope: 4
files_reviewed: 4
findings:
  critical: 0
  warning: 0
  info: 0
  total: 0
modified_rust_owners: 2
read_only_rust_owners: 2
files_reviewed_list:
  - packages/open-bitcoin-node/src/sync/tests/filter_index/phase162.rs
  - packages/open-bitcoin-node/src/sync/tests/peer_filter_transport/phase162.rs
  - packages/open-bitcoin-node/src/sync/tests/filter_index/phase162/missing_history.rs
  - packages/open-bitcoin-node/src/sync/tests/filter_index/phase162/publication_faults.rs
generated_by: gsd-code-reviewer
lifecycle_mode: yolo
phase_lifecycle_id: 165-2026-10-14T16-09-59Z
---

# Phase 165: Preliminary Plan 25 Rust Review

The final preliminary Rust source review is clean: zero actionable correctness, security or test-reliability findings across the two changed adapters and two unchanged callers. Final closure below supersedes earlier preparation/observer premises while retaining every earlier diagnostic and source identity. No tests, builds, heavy Bun execution, source/shared-state edits or Git mutations were performed by this reviewer. C3/native and whole-phase review remain separate required gates.

## Authenticated Baseline and Context

The saved pre-C source bundle was authenticated before reading its rows: exactly 124,139 bytes, SHA-256 `4f557ede3d82567c18c44ba7fc86c88afb9f83c00bfdb110ad26a15040bee981`. Its capture is `2026-10-17T05:20:52.045Z`, fixed Git base `d0104a997c0362bf65b81de1c8fec8ddc3012a1b`, and scope is prospective legacy function-contract bridge only. Each of the four scoped row sources independently hashes to its saved SHA. This is pre-C workspace evidence, not an original qualification receipt or a new current behavioral result.

Current Plan25 SHA-256 is `91d4bf8b1d3bba3b00617f175b5f4b126ade3c54e3f9dead79b777ab6239f0ef`. The corrected C integration plan and independent recheck preserve earlier rejected receipt findings and add the explicit C3 bridge obligation. Current planning approval is conditional on actual predecessor GREEN and sequential25→19→26. C3's old raw-opener function contracts remain deferred to26; this preliminary Rust review cannot certify them or full C/native readiness.

Root instructions, Bright Builds sidecar, placeholder-only overrides, architecture/code-shape/verification/testing/Rust standards and both active lesson inputs were loaded in this ongoing review session. Lessons total7,188 bytes and a conservative2,397-token estimate; no archive was loaded. No extra helper/source ownership is assumed. Existing strict source APIs and normal Regtest maturity100 remain the required setup.

## Original Assertion and Fault Inventory

Authenticated pre-C source was used to inventory the original assertions before final repair review.

| Owner                                        | Original intent to preserve                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| -------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Node `filter_index/phase162.rs`              | Real normal-maturity100 recipe and flags; every actual receipt height/hash, including genesis; real genesis durability and configured Enabled state at that boundary; actual historical/same-block spends; exact independent oracle record/hash/header/script checks; physical B/metadata/undo/coins/wallet evidence; bounded turns; complete cleanup and all-handle raw opening.                                                                                   |
| Outbound `peer_filter_transport/phase162.rs` | Shared actual102-block fixture and safe101, three h101 transactions; exact pinned oracle wire frames; actual loopback socket writes; 31-byte partial frames, deadline, cancel, epoch and disconnect behavior; physical complete-frame receipts; independent generic charge retained/released; released authority during write; no resurrected range; final empty resource ledger and handle-free raw cleanup.                                                       |
| `missing_history.rs`                         | Caller-created h102–104 suffix after initial fixture return, immutable prefix equality and actual receipts; normal maturity100 historical/same-block undo; saved/fresh prefix distinctions; deliberate missing body versus undo refusal with every source/coins/meta/immutable/checkpoint/lock/intent byte preserved; irrelevant unflushed future mate absence survives raw reopening without phantom active authority or repair.                                   |
| `publication_faults.rs`                      | Alternate h102 accepted/indexed first and its actual undo retained; then separately constructed canonical h102–104 replaces it with greater actual work; stale immutable row retained, durable safe101 versus accepted104; eight actual undo/coins/meta/record/checkpoint/protection/commit faults; typed backend/poison/readiness/prune effects; exact changed/unchanged raw rows and physical pairs; complete immutable union and second byte-stable cold reopen. |

## Preparation Observations and Open Checks

At capture, the Node adapter and outbound delegate have changed; the two future callers still exactly match authenticated pre-C source. Their new canonical suffix and alternate/canonical header prerequisites are therefore explicitly pending. Initial fixture setup must not predict those branches or fabricate their body arrival.

The current adapter accepts genuine normal-maturity100 genesis on the same owned Fjall core, returns the actual receipt, performs a real same-core flush with empty header entries, then uses the public fixture constructor. The initial known nonroot recipe suffix uses a modeled Version/Verack and contiguous header ingress. The helper only handles headers, requires an existing accepted parent and rejects already-present/root entries. These are in-progress source observations, not final certification.

The outbound setup delegates to `AcceptedRecipe::at` and keeps finish/flush, safe101 and transaction-shape checks. Its actual write/oracle/socket/resource test body remains available for exact final comparison. Final review must verify a meaningful separately isolated raw genesis observation where needed, without retaining a hidden live owner across an all-handle-close claim. The removed intermediate raw reopen cannot be reintroduced as later live work authority.

Required final checks include actual caller-supplied future headers, separate alternate-first and canonical batches, body ordering before reorg, ZERO numerical carrier and strictly greater derived proof, unchanged original stale/physical/error assertions, exact raw observers/cleanup, no root rebinding or raw header writes, no reconstructed old tie order and no new helper path. Every owned Rust file must remain within628 lines; no cap exception or undeclared helper ownership is approved.

## Preparation Identities

These are complete in-progress working-tree bytes, not closing identities. Every original source hash below was validated against the authenticated bundle row.

| File                                                                                    | Pre-C SHA-256                                                      | Preparation SHA-256                                                | Lines |
| --------------------------------------------------------------------------------------- | ------------------------------------------------------------------ | ------------------------------------------------------------------ | ----- |
| `packages/open-bitcoin-node/src/sync/tests/filter_index/phase162.rs`                    | `623b910dba79389972fcbaf4c22c69d6af827b1531035133a95b729fee384580` | `60cdc7899b28295f00ced0218722e057b8e635af215a71216c14541262ea55f6` | 549   |
| `packages/open-bitcoin-node/src/sync/tests/peer_filter_transport/phase162.rs`           | `3610463d888bb31d2319d435270bc59f71448c20d536f7bb0d242ac6f6cd7020` | `c35b11b189473843d2c34b95a75109c30bb064925019f8bf916c56fdf9241d39` | 311   |
| `packages/open-bitcoin-node/src/sync/tests/filter_index/phase162/missing_history.rs`    | `43b233fa1b7f9685f3ab8f24c508f84bd50b2731739b733801d57e4eca5c021a` | `43b233fa1b7f9685f3ab8f24c508f84bd50b2731739b733801d57e4eca5c021a` | 360   |
| `packages/open-bitcoin-node/src/sync/tests/filter_index/phase162/publication_faults.rs` | `3001c1bad93739a7e84e1fc9aea1d43a0634c2eb9882bc0d5f8a5abcaaa6e813` | `3001c1bad93739a7e84e1fc9aea1d43a0634c2eb9882bc0d5f8a5abcaaa6e813` | 571   |

Preparation ordered scope digest: `5213d89c0bb4b451d705309a5814e37c7143674fd310e05845d6f19e7bf68113`.

Digest construction is SHA-256 over the four frontmatter paths in order, each encoded as UTF-8 path, tab, lowercase complete-file SHA-256, newline. Immutable `support_runtime.rs` still matches `5a25857c552bb7dcfdd040ad15e19c4616f52ef8b35df69b4f11a42dcd5b1f05`. Final drift will be recorded rather than mislabeling this changing snapshot stable.

## Executor Evidence Boundary

The parent reports initial recipe-selected execution of three cases with two passing and one UntrustedWorkLineage failure, and outbound seven of seven failing. The first adapter/delegation is compiling and running actual pressure/oracle behavior; three import corrections were compiler diagnostics only. The two future callers are not yet repaired at this preparation snapshot. No current behavioral GREEN, C3 resolution, native acceptance or whole-phase qualification follows from these reports.

Only this preparation artifact is written. Await stable Rust before the final preliminary source verdict; formal whole-phase review/security/native/formal/normal-hook/Git gates remain separate.

## Executed C2 Correction and Source Recheck | 2026-10-17T05:55:08Z

The earlier preparation's assertion that the two future callers needed explicit separate header ingress is **superseded**. Actual execution disproved that static premise before either caller was edited: the parent and executor report missing_history5/5 and publication_faults8/8 GREEN unchanged after the genuine adapter repair. No behavioral RED for those callers exists, and no source omission is relabeled runtime failure.

The amended Plan25 SHA-256 is `5dfe17c0cfe9383f43a1336a7cf146723f8116406b2f76315538e841bed34622`. It now has five modified implementation owners: two Rust adapters plus three current165 guard files. The two future caller files are explicitly read-only and must remain byte-identical to authenticated pre-C source. Its Task2 is verification-only. Source review may proceed while the independent amendment check is pending, but no final/stable or matrix acceptance follows.

### Causal Path Confirmed by Source

`ManagedPeerNetwork::connect_local_block` first checks genuine direct-connect authority, then invokes `retain_checked_candidate` under the same full parameters. With the actual accepted parent present, that path calls `HeaderStore::check_body_eligibility_inner`. The latter checks immutable parameters and header context, validates header/contextual block rules, inserts the actual header with those parameters and only then records body linkage. Therefore the unchanged caller's actual sequential body connections establish their own new header facts.

`authorize_reorg` similarly checks actual ancestor continuity, calculates each replacement's `block_proof`, invokes `check_cached_body_eligibility` with the full selected parameters and compares the resulting exact cumulative work before preference authorization. The old numerical carrier in unchanged publication_faults is ignored as authority. Alternate h102 is actually accepted/indexed before the canonical replacement is built; the real three-block replacement has greater derived work. No predicted branches, synthetic arrival, order setter or reconstructed historical tie is required. Read-only source tracing confirms the passing caller premise without redundant caller edits.

### Current Two-Adapter Review Observations

The Node adapter retains the real normal-maturity100 genesis receipt, flushes the same concrete source with actual body/undo/positions and empty header entries, verifies durable genesis B and assembles the same genuine owner through the public constructor. Its BASIC mode assertion remains at actual post-genesis initialization. Known initial nonroot headers use the local normal Version/Verack/Headers helper before actual suffix connections; no raw header saving, root rebinding or public production API is introduced. The helper now has private visibility; only `AcceptedRecipe::at` needs cross-test-module visibility for delegation.

An exact comparison against authenticated pre-C source proves outbound `fn frames` through end-of-file byte-identical. This preserves the independent oracle frames, actual sockets,31-byte physical prefixes, deadlines/cancel/epoch/disconnect behavior, authority-release check, exact physical receipts, independent generic pressure and cleanup assertions. Only duplicated raw setup delegates to the genuine adapter.

The new isolated one-genesis observer is source-sound at this snapshot: `AcceptedRecipe::new(1)` provides the actual live receipt, only its value-only position/path are copied, and the complete fixture is dropped before raw configured reopen. It then checks durable B, identical active position, retained BASIC observation and explicitly unearned work origin, with no subsequent work mutation. It drops the raw runtime and performs another independent store open before filesystem cleanup. Temporary network handles do not survive expression boundaries; there is no hidden live owner across either close. Its Rust formatting/execution/final hashes remain pending.

The parent reports Task1's earlier recipe3/3, including the1001-block pressure/oracle case in60.97 seconds, and outbound7/7. Those receipts predate the new isolated observer; its test is not yet claimed executed. Three import fixes were compiler diagnostics only. Both unchanged caller hashes were recomputed again:

- missing_history: `43b233fa1b7f9685f3ab8f24c508f84bd50b2731739b733801d57e4eca5c021a`.
- publication_faults: `3001c1bad93739a7e84e1fc9aea1d43a0634c2eb9882bc0d5f8a5abcaaa6e813`.

No actionable source issue was found in this in-progress recheck. Final observer execution, strict checks and stable closing identities are still required before a clean final preliminary verdict. C3's legacy executable-contract bridge remains Plan26 work, and no C/native/whole-phase qualification is implied.

## Observer Replan Source Check | 2026-10-17T05:58:17Z

**PASS for the narrow observer setup replan before another execution attempt.** This is not final behavioral or whole-scope acceptance. Two actual failures are retained: immutable `Recipe::new(1)` is invalid because its permitted range begins at102; using legal102-block recipe input but actually accepting only genesis then preserved raw B/position while raw BASIC progress was absent. Neither failure justifies changing recipe bounds or weakening the raw assertions.

The bounded correction keeps legal `Recipe::new(102)`, passes only its actual genesis into the existing genuine-genesis helper, and adds exactly one ordinary managed `network.flush_coins(Always)` before dropping all live owners. Source tracing shows why: the helper's earlier real source flush intentionally passed empty header entries, after which constructor assembly validates/adopts the genesis header into the live graph. It has not yet published that graph checkpoint durably.

The normal flush follows `automatic_prune::flush` → `flush_and_evict_pruned_blocks` → `flush_applying_plan_with_header_checkpoint`. After the ordered durable prefix, it captures the existing network header graph as a same-store `TrustedHeaderSnapshot`, then persists through `save_trusted_header_snapshot`, which checks admitted validation identities when coverage is Complete. This is normal checkpoint publication of the already-genuine accepted genesis; it is not a raw header writer or a repair of deliberately absent body/undo, and does not mint source authority on reopen.

The revised observer must still actually accept only one block, retain all durable-B/identical-position/saved-BASIC-present/unearned-origin assertions, drop the entire genuine runtime before raw reopen, perform no later work mutation, then drop raw and independently reopen before cleanup. The current source has this normal pre-close flush and retains those assertions. Its next actual execution, strict checks and final hashes remain pending. No product API/source change, extra owner, count normalization or missing-data restoration is approved by this PASS.

## Actual Observer Result and Narrower Observation Contract | 2026-10-17T05:59:47Z

The parent reports the third observer attempt completed normally with exit101 and0/1: actual genesis B, identical active position and ordinary flush assertions passed, but raw `maybe_basic_index_progress().is_some()` still failed. No broader test ran; implementation paused. This actual result supersedes the preceding expectation that adding an ordinary graph checkpoint could satisfy that runtime-progress assertion. The earlier source PASS proved that normal flushing safely publishes existing admitted facts; it did not establish the correct runtime-capability premise or behavioral GREEN.

Independent source tracing now distinguishes saved durable BASIC metadata from active in-memory BASIC progress. The progress getter in `chainstate/filter_index.rs` only maps `maybe_basic_index_owner`. Ordinary `from_recovered_chainstate` can create BASIC lineage from matching store append proof without requiring live work origin, so a blanket rule that every raw runtime lacks BASIC progress would be inaccurate.

For this actual PreserveSaved path, `recover_basic_filter_index_guarded` calls `invalidate_append`, which explicitly clears `control.maybe_append_identity`. With saved Active ownership and a Keep plan, it validates the saved forest/checkpoint/fence and returns without `install_recovered_basic_filter_append_guarded`. In contrast, the Enabled lifecycle's already-active/no-reconcile branch explicitly reinstalls that append identity. Proof acquisition returns None when the in-memory append identity is absent. The recovered manager therefore has no BASIC lineage, and `initialize_basic_index_owner` returns early. This configuration/proof path explains the absent runtime getter without erasing saved durable metadata or granting work authority.

The minimal corrected observer should remove the speculative extra flush and capture a real nonempty saved `maybe_basic_filter_checkpoint()` before closing the genuine runtime, then compare the exact same saved checkpoint after raw reopening. Exact `StoredFilterState` equality is an additional direct durable observation where space permits. Retain actual genesis B/identical position, explicit unearned work, both all-handle closures and no work mutation after raw reopen. This is a strong exact persistence assertion rather than weakening a runtime assertion to None. The extra header-graph flush is unnecessary for that original durable observation and cannot alter this PreserveSaved proof-install branch.

This recommendation requires its own actual execution and final stable source review. No product/helper change, raw origin promotion, synthetic count, missing-data restoration or global raw-BASIC claim is authorized. C3 and whole-phase gates remain pending.

## Final Rust Source Closure | 2026-10-17T06:02:55Z

**Verdict: clean; zero critical, warning or info findings.** Current Plan25 SHA-256 is `b13e8ee0fc8a8cd25d685a63af199151e2107c872bd1cb872e79d878f45fa977`, including the corrected explicit observer contract. It retains two modified Rust adapters, three guard owners and two read-only callers. All four scoped Rust files were read; current complete bytes were compared with the authenticated pre-C bundle and the final hashes supplied by the parent. This is source review, not independently executed behavior or guard acceptance.

### Genuine Adapter and Preserved Effects

The final Node adapter constructs the immutable normal Regtest maturity100 recipe with P2SH and keeps full parameter/flag assertions. Actual managed genesis acceptance supplies the first real `ChainPosition`; the same core/sink is consumed through a real genesis body/undo/position flush with empty header entries, verifies durable B and enters the public fixture constructor. Each later original body is persisted and actually connected with exact receipt height/hash assertions. Requested BASIC mode is observed at the live post-genesis boundary. No raw intermediate reopen is used for later acceptance.

The private descendant helper validates the already-admitted parent, excludes known/root entries, enforces contiguous new ancestry and dispatches normal Version/Verack/Headers under the same parameters and actual time bound, then checks identities and disconnects its peer. It does not touch bodies or source ownership. Only the known initial recipe suffix is admitted there; no later alternate/future branch is predicted by setup. Cross-test-module visibility is limited to `AcceptedRecipe::at`; the header helper is private.

Exact source comparison proves outbound `fn frames` through EOF byte-identical to authenticated pre-C source. Its independent oracle frame generation, physical31-byte prefixes, real socket/deadline/cancel/epoch/disconnect distinctions, complete-frame receipts, independent generic pressure, released-authority writer check, resource ledger and handle-free raw cleanup retain their original behavior. The setup only delegates to the genuine102-block adapter and preserves finish/flush, safe101 and h101 three-transaction assertions. Existing Node oracle, cost/receipt/undo/coins/wallet/script and deterministic recipe assertions remain intact; the new observer is separate.

### Corrected Isolated Raw Observer

The final observer uses legal immutable `Recipe::new(102)` solely to obtain its actual genesis, then passes only that block into `live_genesis_runtime`. Its actual accepted position is height zero and the genuine owner is observed before closure. It captures nonempty saved `FilterCheckpoint` **and** nonempty `StoredFilterState` through direct store APIs, then drops the complete runtime. These saved values are owned data; the local store binding is a borrow, not a retained Fjall clone.

After real all-handle raw PreserveSaved reopening, the observer asserts exact durable B, identical one-element active position list, exact same saved checkpoint and exact same saved state, together with explicitly unearned work origin. It performs no later work mutation. It drops raw, independently opens/closes the store again, removes the directory and asserts absence. No live source survives either closure. The speculative extra ordinary flush is removed.

This final assertion observes persistent BASIC metadata without claiming an active runtime BASIC progress owner. The precise PreserveSaved/append-proof distinction traced above remains valid; the review does not infer a blanket raw-runtime ⇒ no BASIC owner rule. The original live configured BASIC mode assertion remains in `AcceptedRecipe::at`, while meaningful cold durable observation is now isolated from later live acceptance.

All three actual observer diagnostics remain historical: invalid count-one recipe, then legal102 recipe/one actual genesis with absent raw runtime-progress owner, then an ordinary graph flush that still did not provide that owner. The parent records that the third execution started just before pause delivery and completed normally; it was not interrupted or relabeled success. The last failed premise was corrected through direct saved-metadata identity, without changing the product, immutable recipe or actual source helper.

### Read-Only Callers and Actual Authority

Both future callers remain complete-byte identical to authenticated pre-C source. The earlier proposed C2 header and carrier edits were never applied. The source path independently confirms the actual execution result: `connect_local_block` invokes checked candidate retention, which checks the full selected header/context and inserts the actual header before body linkage. `authorize_reorg` validates each cached replacement with the same full parameters and derives cumulative proof from actual headers.

Thus `missing_history::extend` actually constructs/connects only its own later suffix with unchanged immutable-prefix and receipt assertions. `saved_with_stale` actually accepts/indexes alternate h102 first, retains its actual undo and stale immutable row, then builds canonical h102–104 and performs a greater-derived-work reorg. The unchanged untrusted numeric carrier is ignored as authority. No synthetic arrival or old tie reconstruction was introduced. Every original missing-mate, physical B/metadata/undo, safe101/accepted104, typed effect failure, poison/protection, immutable union, exact allowed row delta, all-handle refusal and second stable cold-reopen assertion is preserved by full-byte equality.

### Final Identities and Preparation Drift

| File                                                                                    | Final lines | Closing SHA-256                                                    | Drift from initial preparation                                                                                                             |
| --------------------------------------------------------------------------------------- | ----------- | ------------------------------------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------ |
| `packages/open-bitcoin-node/src/sync/tests/filter_index/phase162.rs`                    | 621         | `677a3a5def6d6be380454ee029bfecf18b1d0de0fd3a25a0248d7f9ee4638160` | Private helper visibility, corrected isolated one-genesis observer, exact saved metadata assertions and Rust formatting; completely reread |
| `packages/open-bitcoin-node/src/sync/tests/peer_filter_transport/phase162.rs`           | 311         | `c35b11b189473843d2c34b95a75109c30bb064925019f8bf916c56fdf9241d39` | None after initial preparation; final hash matches                                                                                         |
| `packages/open-bitcoin-node/src/sync/tests/filter_index/phase162/missing_history.rs`    | 360         | `43b233fa1b7f9685f3ab8f24c508f84bd50b2731739b733801d57e4eca5c021a` | None; exact authenticated pre-C source                                                                                                     |
| `packages/open-bitcoin-node/src/sync/tests/filter_index/phase162/publication_faults.rs` | 571         | `3001c1bad93739a7e84e1fc9aea1d43a0634c2eb9882bc0d5f8a5abcaaa6e813` | None; exact authenticated pre-C source                                                                                                     |

Closing ordered four-file digest: `d986c3b6043cf126a0efbd740c378118ba661280633c6f638e8805e41eaf6f65`, using the same path-tab-hash-newline construction documented above. All four files fit628 lines with no exception. Immutable `support_runtime.rs` remains `5a25857c552bb7dcfdd040ad15e19c4616f52ef8b35df69b4f11a42dcd5b1f05`. Subsequent changes require delta review; this verdict applies only to these exact identities.

### Executor Receipts and Remaining Gates

The parent reports the corrected isolated observer1/1 in0.73 seconds; complete Node phase16238 passed/zero failed/one pre-existing unchanged process.rs ignored child,39 discovered,78.20 seconds; outbound7/7 in7.93 seconds. The final strict sequence passed format, Node all-target/all-feature Clippy in6.38 seconds and build in0.08 seconds, session54587 exit zero. Logs use `.local/open-bitcoin-dev/phase165-plan25-final4-<filter>.log` and the corresponding format/Clippy/build receipts. These are executor results, not commands run by this reviewer. Earlier recipe3/3 predates the observer and is not misused as its evidence.

Current guard pin renewal/matrices are separate from this source verdict. C3's original raw-opener function-contract bridge remains Plan26 work; final C/native/workspace/coverage/security/goal review and normal-hook Git finalization remain unclaimed. No Phase166/167/168 qualification is implied. Only this preliminary report was written by the reviewer.
