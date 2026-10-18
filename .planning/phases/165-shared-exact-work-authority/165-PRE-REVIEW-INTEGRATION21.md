---
phase: 165-shared-exact-work-authority
reviewed: "2026-10-17T10:33:30Z"
client_date: "2026-10-17"
phase_lifecycle_id: 165-2026-10-14T16-09-59Z
lifecycle_mode: yolo
depth: deep
status: clean
scope: "Plan21 fourteen-owner source and exact prospective current165 row/input renewal review"
implementation_verdict: clean_before_exact_approved_renewal
files_reviewed: 14
files_reviewed_list:
  - packages/open-bitcoin-node/src/network/tests/runtime_projection.rs
  - packages/open-bitcoin-node/src/network/tests/lifecycle_projection_cases/maintenance.rs
  - packages/open-bitcoin-node/src/sync/tests/sync_cycles.rs
  - packages/open-bitcoin-node/src/sync/tests/inflight_failures.rs
  - packages/open-bitcoin-node/src/sync/tests/phase70_peer.rs
  - packages/open-bitcoin-node/src/sync/tests/phase70_peer/failure_classification.rs
  - packages/open-bitcoin-node/src/sync/tests/phase70_peer/fallback_rotation.rs
  - packages/open-bitcoin-node/src/sync/tests/peer_resolution/resolver_rotation.rs
  - scripts/check-phase164-canonical-identity/contracts.ts
  - scripts/check-phase164-canonical-identity/source-files.ts
  - scripts/check-phase164-canonical-identity.test.ts
  - scripts/check-phase165-shared-exact-work.ts
  - scripts/check-phase165-shared-exact-work/contracts.ts
  - scripts/check-phase165-shared-exact-work.test.ts
findings:
  critical: 0
  warning: 0
  info: 0
  total: 0
---

# Plan21 Preliminary Independent Review Preparation

**PREPARED.** Read all three tasks of the actual amended21 plan, all eight authentic original Rust owners and relevant immutable root/seed/authorization interfaces. The amended plan hashes `04c941c6568e6af8a700f8e3b36d4b77b49e93e83618f59c186203ac35fe9c9b`. Root's fourteen-owner capture `.local/open-bitcoin-dev/phase165-p21-before.json` independently hashes `a3f809124341c54364962810b5792ff8be052a160d781c9b581dde6d5721361b`; every captured full source matches its declared SHA and the capture binds that exact amended plan.

At10:26UTC all fourteen working sources still matched those original identities. Ordered baseline/preparation SHA is `80d96a2e80a065cec7b2ac54f322da1546b8aac9d2d13c509d18a4d9bfedcbe6`, computed over packet-order path, tab, full source SHA, newline. This is preparation evidence only. No implementation verdict, source stability during later edits, pin renewal or runtime success is inferred.

The reviewer owns only this report; no implementation/shared-state/Git changes, Cargo/Bun heavy commands or nested agents. Executor owns all implementation and verification. Existing AGENTS/sidecar/overrides and architecture/code-shape/testing/verification/Rust/TypeScript guidance apply. Active lessons were fully read earlier in this continuous session:7,188bytes/2,397 conservative tokens, no archives; no project skills were present. Final phase/native/security/formal acceptance remains separate.

## Original Rust Scope and Exact Intent

All paths below are relative to `packages/open-bitcoin-node/src/`. Direct test counts describe the original files only; they are not filtered execution counts or future enabled-owner approval.

| Original owner                                          | Lines | Direct tests | Complete-file SHA-256                                            |
| ------------------------------------------------------- | ----- | ------------ | ---------------------------------------------------------------- |
| network/tests/runtime_projection.rs                     | 375   | 5            | c7872186146ce1bd4b727f2594db05128ba44fbd43a5dd75b6f36316804192aa |
| network/tests/lifecycle_projection_cases/maintenance.rs | 392   | 4            | bcd62d97210188fa4da607ca3216ac19feb3b072a3abf21ad689e647eebef358 |
| sync/tests/sync_cycles.rs                               | 222   | 4            | 7ad1d7c85c5108b20b75a2a43524b99fbfc1ba2a17ae29bde94b000e5eb977eb |
| sync/tests/inflight_failures.rs                         | 107   | 2            | 308eb00d6464c1550d736561461fbeb0e857e7d9b429a18563ec2baa3a4e9079 |
| sync/tests/phase70_peer.rs                              | 77    | 0            | 1e0dc03d57ddfa5fb01c89b68c35ebc3abc0ffd560fa19ce240cbc66a880d924 |
| sync/tests/phase70_peer/failure_classification.rs       | 260   | 6            | aa2e40fd61be829c4df36d8c2ed6d721d454ce4449c00b551cd93336a261d0aa |
| sync/tests/phase70_peer/fallback_rotation.rs            | 209   | 3            | 61b1a56bacf974aa8352ddef78edf4888a3cd9fa503b89cd9c514c8c79089094 |
| sync/tests/peer_resolution/resolver_rotation.rs         | 211   | 4            | 46c61a6bb66ff5941266a6d8e4af9373d812e03c5ccf984a48bf680da7fe60f9 |

`phase70_peer.rs` owns strict rotation/credit helpers and backoff/failure/fallback module registrations. It has no direct executable tests. A source row/loaded helper mutation may be legitimate after actual edits, but a fabricated nonzero TEST_OWNERS row is not. Existing unowned backoff_rotation remains a dependency of the full parent corpus, not an implicit implementation owner.

### Network Projection and Future Timestamp

Preserve original local source/sink block synchronization, real message delivery sequence and transaction relay/mempool entry. Source's actual two local connects already establish selected headers; removing its redundant post-connect admission must not remove sink's genuine root/body initialization. Existing resource/fee projections, memory-versus-vsize distinctions, peer role totals and privacy/redaction assertions remain.

The future case has a real accepted genesis parent and deliberately too-early body message time `future_block.header.time - 7201`. Complete a real Version/Verack handshake and admit only the future child header at that header's valid time before sending its body at the early time. Require the actual nested typed time-too-new rejection cause, not generic error, unknown-header failure or a skipped body. No re-admission of the established graph, hand-edited private header receipts or raw-source promotion. Original rejection specificity is a proof obligation even if its outer wrapper legitimately changes.

### Sequential Reorg Lifecycle

The maintenance case must retain both actual replacement bodies before competing old-parent/old-child bodies earn their linkage. Keep actual common fork, original spend transactions and parent/child reconstruction. Consume network.authorize_reorg→retain_authorized_reorg→chainstate.prepare_authorized_reorg→commit_prepared_reorg. The original lower unqualified reorg is no longer a grant; caller numeric work is not authority.

After actual chainstate transition, retain the original separate replacement-block loop. Each connected replacement must have empty lifecycle delta and complete empty projection/generation0. Reconsider parent separately, assert Accepted with exact txid and all present projections/generation1; then reconsider child separately, assert Accepted exact txid and both members/generation2. An all-at-once lifecycle helper would erase the test's original causal assertion intent and is forbidden here. Other maintenance expiry/conflict roles, descendant order, evidence counters and empty-pressure/generation assertions remain unchanged.

### Sync Cycles and Inflight Failures

Header scripts must admit one real mined model root outside P2P, then send only genuine contiguous descendants with real handshakes. Keep original complete SyncProgress object, messages_processed3/headers_received2/blocks_received0 for its two received descendants, all physical saved header rows, GetHeaders/GetData signals and accepted/root-derived total height accounting. Any height/target shift must be exact throughout the full object, stored row count and TargetHeaderReached payload. Preserve the three actual advancing rounds, Info target diagnosis, and distinct Warn NoProgress rounds2/durable no_progress phase. Do not lower counters or delete complete assertions to accommodate setup.

Invalid/malformed inflight cases require an actual genuine empty source with accepted genesis body, valid accepted child header, then deliberately invalid child body or transport decode failure. Invalid body must reach consensus body classification rather than unknown-root InvalidData. Retain exactly one failed peer, zero contribution, request multiplicity at least2, runtime inflight empty, dead first-peer inflight released, invalid body physically absent, downloaded/best connected height truth and cleanup. Initial accepted genesis is a prerequisite, never credit for the failed child. Any legitimate new hashes/heights must come from real setup and be checked coherently.

### Peer Classification, Fallback and Resolver

Keep strict helper equality for exact failure reasons, no failed-block credit and first-peer backoff. Invalid/malformed peer cases retain attempted2/failed1/connected1, second Connected, retry multiplicity and released both inflight domains. Validation-stall evidence remains typed Validation subsystem with invalid-block evidence and unavailable new progress credit. Duplicate/disconnected/nonextending controls retain their earliest actual legitimate typed refusal and all original no-effect/cleanup conditions; if a stronger source/header gate necessarily precedes the old classification, preserve a separate authorized case that reaches the intended deeper classification rather than relabeling every case generically.

Fallback seeds must use the unchanged genuine absent-path seeder with actual full active bodies, full params and actual retained child headers. The seeder validates bodies on a new genuine source and saves complete actual facts; reopening saved bytes is still untrusted until an explicit strict fixture replay. Header/request-only consumers admit protocol facts and assert raw work remains unearned. No count normalization, missing-body/undo restoration or equal-work arrival reconstruction. Retain notfound reassignment/retry, stale inflight cleanup, last validated useful-work height1 or0 as originally intended, new progress unavailable, last peer MessagesOnly, exact second peer identity, SlowOrStalledPeers diagnosis and original backoff.

Resolver rotation must keep the same resolver mappings and peer2 message contribution exactly1header. Pre-admit a real root and send its actual mined child from the alternative peer; retain first Stalled/Stall, second Connected, attempted2/outcomes2, waiting RetryBackoff/consecutive_failures=1 and Warn waiting-for-peers projections. An invalid root script cannot stand in for this successful child ingress.

## Protected Inputs and Final Review Checklist

At preparation `sync/tests/support_runtime.rs` remains full SHA `5a25857c552bb7dcfdd040ad15e19c4616f52ef8b35df69b4f11a42dcd5b1f05`; current20,688-byte preservation remains `e6cb5740cb5c83655aed14c43b1206847d264431d944741a365771c3e8d76f5b`. Original20,556-byte preservation attribution remains separate3d004f91bd72a0f3902588a36fca0438b04b5ac034bc9e227893664496835630. Neither authority/helper source belongs to21. Actual existing authorization checks consume owner/tip/generation/header eligibility and cannot be replaced by a count/score or invented receipt.

Current165 starts117source owners/54test files/246inputs; old164 stays135inputs/477edges. Final review must inspect actual implementation and real typed/counter/assertion deltas before any proposed identity renewal, then independently check each precise changed/new source row, actual enabled declaration count, parent/helper registration, loaded unique source mutation, original relative inventory order and new exact inventory digest. Preserve all old164 unaffected identities, historical inverses/data/manifests/receipts/default six registrations and their independent removal controls. No extra owner/module/dependency is presumed.

Await a final stable fourteen-owner packet with before/current full sources and actual executor receipts. Bind review findings to actual pre/closing hashes and disclose any source movement. Require nonzero discovered focused groups and honest existing ignored counts; record real initial failure/next-gate evidence rather than manufacturing RED. No source/matrix/runtime result or clean implementation verdict has been claimed in this preparation. Only this report changed.

## Final Frozen Pre-Pin Source Review | 2026-10-17T10:33:30Z

**CLEAN for the bounded Plan21 source and exact proposed current165 data renewal.** No actionable finding remains. The preceding preparation/pending wording is retained history. Full current guard matrices, final input/module closure and whole-phase/native/security/formal acceptance are separate pending gates. No tests/builds/heavy Bun execution, implementation/shared-state/Git mutations or nested agents were used by this reviewer.

Frozen packet `.local/open-bitcoin-dev/phase165-p21-prepin-review.json` independently hashes `a1652d2430cec3fb0dec88e9ff208af04bc3c1da17f34e768bb729e2321a3115` and authenticates the original a3f809 before capture. Every actual source matches its packet SHA. Its ordered digest construction is SHA-256 of `path:SHA` rows joined by newline without trailing newline, independently yielding `affc2da20d4544edb006900af5856a08e27b58ef4e2462791d35d5d11f66306e`. For continuity with this report's preparation convention, path/tab/SHA/newline construction yields `8d2836f1c2bdd61c6990c36c5aadd7f438c7dad7aded702a43886e7ae031173f`; the differing constructions are not source drift.

| Closing pre-pin source                                                                 | Lines | Complete-file SHA-256                                            |
| -------------------------------------------------------------------------------------- | ----- | ---------------------------------------------------------------- |
| packages/open-bitcoin-node/src/network/tests/runtime_projection.rs                     | 392   | 7d25d6927a2cac24ec90a13e3b858b60f040bf99016f0ef9c15647d281394de4 |
| packages/open-bitcoin-node/src/network/tests/lifecycle_projection_cases/maintenance.rs | 408   | cc9cba526f4d1b5c7740506228a7880570ca1f85cfe4fe1ccfaa43615532b27f |
| packages/open-bitcoin-node/src/sync/tests/sync_cycles.rs                               | 231   | 0cc1856646f04a3a1e4f1cd308b3ffe40e444643fdfb180fd269fa0d77f8199e |
| packages/open-bitcoin-node/src/sync/tests/inflight_failures.rs                         | 111   | bcd897d7a89429a0921ec339bb67b805b9d3bb0ca8f2fa189006b4d335b284b6 |
| packages/open-bitcoin-node/src/sync/tests/phase70_peer.rs                              | 77    | 1e0dc03d57ddfa5fb01c89b68c35ebc3abc0ffd560fa19ce240cbc66a880d924 |
| packages/open-bitcoin-node/src/sync/tests/phase70_peer/failure_classification.rs       | 268   | 78d7020082fa177dc1c70b3731e440f04bf63a8066b07cb7a3ca6408765e44b4 |
| packages/open-bitcoin-node/src/sync/tests/phase70_peer/fallback_rotation.rs            | 217   | 353567ef09b01758c751f7a2970a7fbbb873700e8f8507435a0b5eb137499f89 |
| packages/open-bitcoin-node/src/sync/tests/peer_resolution/resolver_rotation.rs         | 211   | 44632b985da2d5ea00988aac12171fc369d34f135288aa61d7c7d8f478ad437b |
| scripts/check-phase164-canonical-identity/contracts.ts                                 | 515   | f1e694c6463af0b30b904c3a4f7d5008aeaedbe2117eba2da711a0d71d1f451f |
| scripts/check-phase164-canonical-identity/source-files.ts                              | 137   | 5cd178af07ea7f27204a2586d94bf8b1731dc6fd6b32c7b1ded0e07b6fc32832 |
| scripts/check-phase164-canonical-identity.test.ts                                      | 598   | a13cb68af20f25669d6bdffb4657cf6c426272872c6596109a121314f577abb3 |
| scripts/check-phase165-shared-exact-work.ts                                            | 266   | 5365d5be2beaf8f9f2cb66e0db4e1cd68af3477b230360470acc07ab438dde64 |
| scripts/check-phase165-shared-exact-work/contracts.ts                                  | 532   | 14cb239ac57961ee281d7b64e713e52037a32eb38a3b93cb21513a80301e078c |
| scripts/check-phase165-shared-exact-work.test.ts                                       | 627   | 810f649a91c18edcfe601f7de7e328bc5b576d01dc418941a21951ce84b6144f |

Seven Rust sources change. Helper-only phase70_peer, all three old164 owners, current165 entry and executable main test remain byte-identical to authentic before21. Only the existing downstream literal mutation data grows in current165 contracts; no new helper, module or test-loop path is introduced. All fourteen sources fit628. Protected source and existing breadcrumbs are checked below.

### Actual Protocol and Lifecycle Behavior

Runtime projection removes only source's redundant bound-header admission; the sink's real root admission/genesis body and the entire original sync/transaction relay assertions stay. The future case now actually handshakes Version/Verack, admits the real child header at its valid own time, then sends the unchanged body at time minus7201. The exact nested error is now ManagedNetworkError::Network(NetworkError::InvalidHeader) with reject_reason equal time-too-new. This is the actual body eligibility/header-context boundary, not generic error or absent-header refusal; selected candidate intake checks body eligibility at the body message's current time. The old outer Chainstate wrapper is no longer the earliest reached cause, while the exact rejection reason and temporal premise remain. No private header mutation or re-admission is introduced.

Maintenance retains two actually checked replacement bodies before the old parent/child tip bodies. It then obtains the actual sealed network authorization, retains only that grant's intake, prepares with authorization and consumes commit. No generic lower reorg or all-at-once lifecycle helper remains in the replaced setup. The complete original assertion sequence after the transition is unchanged: each replacement has empty delta/projection0, parent reconsideration Accepted and projection1, child reconsideration Accepted and projection2 with exact original transaction identities/members. Existing source-generation/preference authority supplies the grant; the old AnchoredBlock numeric fields do not mint it.

Sync cycles add genuine model roots outside the received scripts. The existing two received descendants still contribute headers2/messages3/blocks0, with root-derived best/header height2 and physically saved rows3. The complete SyncProgress object remains asserted. Three advancing rounds now report heights1/2/3; both actual summary assertions become3. The target becomes3 consistently in config/summary/TargetHeaderReached, retains all three descendant batches and unchanged Info/durable phase assertions. NoProgress case is byte-identical. Handshakes/GetHeaders/GetData/durable evidence remain; no null-parent root is sent through P2P.

Invalid/malformed inflight and peer scripts add real genesis initialization and target actual valid child headers. They keep the deliberate invalid child body mutation/transport failure, zero failed-block contribution, retry request multiplicity, runtime/peer inflight release, physical absence of invalid body, original block/download heights and backoff/second-peer classification. Initial accepted genesis is not credited as a failed child. The duplicate/disconnected/nonextending source tail independently compares byte-identical to before21, preserving its exact original reasons and no-credit/cleanup conditions. No lost data is restored to reach a deeper gate.

### Precise Validation-Stall Projection Adjustment

The first actual peer run already matched InvalidBlock and Validation but failed only the old BehindAwaitingHeaders basis after the valid child header was accepted. Final source changes that one string to BranchCompetitionAwaitingBodies and adds explicit best_header_height1/best_block_height0 assertions. This follows the real reducer: block reconciliation returns BranchCompetitionAwaitingBodies for the accepted suffix whose valid body is absent, and progress diagnosis selects that variant before its fallback BehindAwaitingHeaders. The invalid body does not become durable accepted data. InvalidBlock, unavailable progress credit, Validation serialized/typed subsystem and recovery_category/peer_failure evidence remain. This is a justified causal projection update with stronger concrete header/body observations, not a broad failure-expectation weakening.

The initial peer11pass/1fail/0ignored run remains retained, exit101, log SHA `31032039ce617ce99ec5f894478aa228229555a884dfe33c27a5da7294f16fe0`. The later complete peer12/0/0 run separately closes that precise delta; it does not erase the earlier receipt.

Fallback replaces only the three raw seed calls with the existing absent-path genuine seeder. That seeder performs actual full selected-body validation and persists complete genuine facts; subsequent explicit full fixture replay is therefore count/physical-identity checked. No normalization or unavailable history repair is added, and initial reopened bytes do not themselves earn work. All notfound reassignments, prior useful-work0/1, MessagesOnly/second-peer identity, stale cleanup diagnosis, unavailable new credit, stall subsystem and backoff assertions remain unchanged. Resolver adds one real root and makes the alternative peer send its actual child; the entire original mapping/backoff/Stall/Connected and exact one-header contribution assertions remain.

### Exact Loaded Mutations and Proposed Data Renewal

All nine proposed changes are appended to the existing function-scoped DOWNSTREAM_SOURCE_MUTATIONS table. Independent selection of each actual named function finds exactly one before-token and a different after-token. They cover lost sink root admission, lost future Verack, replacing actual checked body order with unknown cached order, lost retained sealed intake, lost cycles root, lost inflight/peer genesis, raw fallback seed restoration and lost resolver root. The unchanged main loop requires a passing real baseline, actual region/anchor presence and changed raw bytes, updates only a cloned loaded map, then requires the exact modified file/work-owner finding. No no-op control, alternate source reader or test-loop weakening is present. Full mutation execution remains pending owner/input registration; source-policy mutations do not claim to execute those alternative Rust programs.

Only the packet's seven proposed current165 source rows, seven actual test rows and the matching inventory literal may be renewed. All seven sourceIdentity values were independently recalculated from reviewed source after the same leading commentary boundary and match:

| Appended owner suffix under packages/open-bitcoin-node/src/ | Rule       | Reviewed sourceIdentity                                          | Tests |
| ----------------------------------------------------------- | ---------- | ---------------------------------------------------------------- | ----- |
| network/tests/runtime_projection.rs                         | work-owner | 43184c4d36dc9cdcd9bd2c67dd1d1338233b0fbde7372a46d025800c2e1f9051 | 5     |
| network/tests/lifecycle_projection_cases/maintenance.rs     | work-owner | c880e9faec7536caf80d4c5a71ecd4921da5a8c48a8ea19434a2d9c11852bfbe | 4     |
| sync/tests/sync_cycles.rs                                   | work-owner | 8e4c180c1a166f3763de1fb00b0bf9fee49e0e1a48de160727af6df3c6fc1a5c | 4     |
| sync/tests/inflight_failures.rs                             | work-owner | 067a847a0da8ff5008e2a7242630a1a27d89a8bd34fe2621c379344a96a0412f | 2     |
| sync/tests/phase70_peer/failure_classification.rs           | work-owner | 1bede1f72ed5517d7502274554bc444767ed8de262559d053ef747a5e4e80908 | 6     |
| sync/tests/phase70_peer/fallback_rotation.rs                | work-owner | dd3199f3b106a439bf7c1b89aea80486f6bdf2d6a018c7c75d884e1b63e2531e | 3     |
| sync/tests/peer_resolution/resolver_rotation.rs             | work-owner | 0065f192f60b6e055d83915c68045293877b7ced9eeaa2408cbc619cf59e0f3e | 4     |

Counts independently match actual enabled #[test] declarations; no changed source has added ignore/disabled cases. SOURCE_CONTRACTS grows117→124, TEST_OWNERS54→61, preserving all old rows and relative order. Unchanged phase70_peer is not assigned a fabricated nonzero test count. Old164135inputs/477edges and all three owned old164 sources need no renewal.

The seven exact paths above enter the derived ordered inventory at indices233–239. Independent reconstruction from authentic predecessor246 proves all253 paths unique, every old246 path preserved in relative order and compact JSON SHA `3d966febfb92e7579d0cab0dd58eaab094c2785256a4b3b74c9c9671bb2b0a85`. Only INPUT_INVENTORY_SHA may change to that value; no extra path/glob/source import or inventory bypass is approved. Existing missing/extra/reordered/redirected tests remain byte-identical and must rerun. Source row pins are review attribution, not runtime or historical qualification.

### Actual Executor Evidence and Protected Closure

Read executor result envelopes and independently matched each log SHA; these are executor runs, never reviewer execution. All six final full filters passed with no ignored cases:

| Full Rust filter   | Pass/fail/ignored | Libtest duration | Complete log SHA-256                                             |
| ------------------ | ----------------- | ---------------- | ---------------------------------------------------------------- |
| runtime_projection | 5/0/0             | 0.01s            | e92b5ecc2df355e6ea039da8a7f710ff45f79ecfc37b1abbf9d853a29b28c214 |
| maintenance        | 4/0/0             | 0.01s            | a2b1452c8b7866c7f812bed861367f6e0e820f0f155860f784f70d859fa9195f |
| sync_cycles        | 4/0/0             | 2.19s            | 53ed0199a0b0d7e9e85833139a54f5e0981f176270d13108c4c97b98e1ef2654 |
| inflight_failures  | 2/0/0             | 1.07s            | 68dd17e35b5a642d56b66c9c3f8f2bbd7b2ce34d2308b73bbfa36d88ee28f837 |
| phase70_peer       | 12/0/0            | 5.01s            | 822827afccc9c71628a0cf1f11ee70deb551bbace2347adc9d27668696947821 |
| peer_resolution    | 11/0/0            | 4.50s            | 9be20cdcbae987040637099f248933a47a60c9f2751478e291edb8b8b9f641dd |

Format/strict Node all-target/all-feature Clippy/build envelopes each report exit0. Format elapsed1,531ms has empty log SHAe3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855; this empty content alone is not an exit receipt. Clippy elapsed6,348ms log SHA `ccbd9225aa19e8e8757b77067b212ea0fd784fc6943280b54a2ab5dfd265f864`; build elapsed292ms log SHA `8c233a54790c4800d3dd585436af679aa838da259bfdb129412e2d51cdb75cdc`. Original initial baselines and next-reached peer projection failure remain retained execution evidence, without manufactured RED or broadened qualification.

Protected source hashes independently match support5a25857c552bb7dcfdd040ad15e19c4616f52ef8b35df69b4f11a42dcd5b1f05, preservatione6cb5740cb5c83655aed14c43b1206847d264431d944741a365771c3e8d76f5b, fixture-sourceb92d2d91da1dd7715e7de37737769f30bd543cb2fd8a7a1eecb92aaf3335f3e3 and historical-tooling47bbcd914cf5a632f618310b69b987462551ca2f103bce823a8d0cc6326a5f65. All seven changed Rust owners have one existing manifest breadcrumb group with matching source comments; no breadcrumb file/path change is needed. Historical inverse/receipt/data and all six default verification lines are outside this delta and unchanged.

No implementation byte drift was observed during this review. The root may release only the exact proposed data/pin delta, verify it independently, then execute full current164/current165 matrices and complete input/module read-set closure. All phase/native/coverage/security/formal/normal-hook/Git gates remain parent-owned and pending. No raw reopen authority, canonical bootstrap, contextual or pinned-Knots daemon qualification is claimed. Only this report changed.

### Approved Renewal Observed at Closing Read | 2026-10-17T10:35:06Z

After the clean scoped verdict was sent to root, the worker applied the approved renewal during report formatting. Exactly current165 entry and contracts changed. Independent reversal of only the inventory SHA and seven appended source/seven appended test rows reconstructs both pre-pin complete sources byte-for-byte. All other twelve packet identities remain unchanged; no helper, old164 or protected dependency moved.

- Current165 entry closing SHA: `31e89aeaa1c20cdcff7dea3acb46f7e1dba82f3657ab67a66b4b3eaefe349792`.
- Current165 contracts closing SHA: `004bb92cf133b418cb3eeae973bbfe9e36bb485099f725aad559409d26365f37`,546lines.
- Post-renewal fourteen-owner colon/joined-newline digest: `29d1c9ca0be9f35fd0a4b47a941215f847627aaf33d5fc7fe99fa3f6b1936ef0`.
- Equivalent report-convention tab/trailing-newline digest: `f14561fdb11caa0ad27f60dfe598acd04c536b40ac9e066b921060d6969a4bab`.

This confirms only the exact approved source/data delta. Full current guard/mutation/input-module matrices remain pending at this closing observation. Reviewer changed only this report; scoped formatting and diff checks pass.
