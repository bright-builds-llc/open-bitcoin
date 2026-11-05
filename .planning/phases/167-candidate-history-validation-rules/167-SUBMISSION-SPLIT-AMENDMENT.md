---
generated_by: gsd-plan-phase
lifecycle_mode: yolo
phase_lifecycle_id: 167-2026-10-21T16-38-23
generated_at: "2026-10-22T07:12:00Z"
commits: false
---

# Phase 167 local submission provider and RPC split

This formal GSD planning candidate inserts provider Plan 30 after Plan 12, then adds real authenticated submission wiring to Plan 13. It preserves every original task block and command in all 29 earlier plans. The candidate has 30 plans, 53 tasks and 25 execution waves, subject to independent validation. No source execution, Plan 12 acceptance, phase acceptance or Git saving follows from this document.

Root released formal planning through `amend13submission/root-split-planning-release.json`, SHA256 `7d75f774ce457c7fe7ce12b3eee56f52df2a1b1748949888c58a7ddc2c4665b0`, accepting the earlier split feasibility report only as design evidence. The formal gsd-planner revision workflow and phase-prompt template were loaded; lifecycle is the existing yolo attempt. Root alone owns STATE, ROADMAP, todo, metrics, config, requirements and Git. Independent checker alone appends the canonical verdict.

## Why a separate provider is necessary

The current RPC method registry, typed MethodCall normalization and dispatcher have no submitblock. An actual authenticated normal daemon test cannot be replaced by a direct Rust context call, stale executable, binary catch-all or invented peer. Genuine local connection also needs prior earned header context, while its ordinary IfNeeded persistence does not prove full restart/BASIC durability.

The initial 14-owner floor omitted the second ManagedRpcContext initializer. A dedicated private bridge field must be initialized in both canonical_chain.rs::new and context/network.rs::assemble_context. The former is already owned; the latter adds a fifteenth source to the unsplit plan. Generic HTTP/dispatch remains polymorphic over S,V, and public from_chainstate permits a pure canonical core with arbitrary storage. A scope label/core check cannot replace a real product command. Root rejected packing this into fourteen owners or hiding the capability in unrelated fields.

Move actual node/pure providers to Plan 30. Plan 13 then has its existing nine owners plus five RPC registry/handler/initializer owners, fourteen total. Its two original task blocks remain byte-exact, with one explicit intermediate wiring task. This preserves the full original scope rather than reducing it.

## Exact ownership

### Plan 30: six provider owners

| Owner                                                                                | Baseline lines | Responsibility                                                                                                                   |
| ------------------------------------------------------------------------------------ | -------------: | -------------------------------------------------------------------------------------------------------------------------------- |
| packages/open-bitcoin-consensus/src/block/witness.rs                                 |            251 | Pure uncommitted witness normalization beside the existing selector/validator; inline tests.                                     |
| packages/open-bitcoin-consensus/src/block.rs                                         |            554 | Thin documented public wrapper; no separate lib export.                                                                          |
| packages/open-bitcoin-codec/src/block.rs                                             |            144 | Separate prefix parser reusing the current parser; strict parse_block remains strict; incremental block-vector growth and tests. |
| packages/open-bitcoin-codec/src/transaction.rs                                       |            296 | Narrow incremental/fallible input/output/witness vector growth; no valid-wire, signature or limit change.                        |
| packages/open-bitcoin-node/src/network/runtime_authority/validation_history.rs       |             13 | Safe paired factories, opaque command/result, serialized local admission/validation/durable continuation.                        |
| packages/open-bitcoin-node/src/network/runtime_authority/validation_history/tests.rs |            New | Substantive factory/identity/processing/flush/BASIC/refusal/reopen tests.                                                        |

The original four-owner proposal gained two explicit, source-grounded pure owners. First, current parse_block calls Reader::finish, but pinned DecodeHexBlk permits a suffix. The already-public codec block module can expose a separate prefix API without weakening strict callers or adding lib.rs. Second, current block/transaction decoders eagerly reserve CompactSize-declared vectors before the input proves their elements exist. Tiny malformed data can therefore reserve enormous storage. Root separately authorized the sixth owner for incremental/fallible growth. Valid accepted bytes, CompactSize MAX_SIZE, signatures and strict trailing behavior stay unchanged; no budget increase or error-mapping owner is hidden.

Plan 30 Task 1 owns the four pure files. Task 2 owns the two node files. Its six-owner scope is below fifteen. The node parent runtime_authority.rs remains unowned at 627 lines; its existing child registration is sufficient. A conventional cfg(test) child declaration is made only in the declared provider. No dependency/crate, second worker, node parent export or new storage/proof owner is added.

### Plan 13: fourteen RPC owners

All paths below are relative to packages/open-bitcoin-rpc/:

1. src/context/canonical_chain.rs
1. src/context/canonical_genesis.rs
1. src/context/canonical_history.rs
1. src/context/canonical_history/tests.rs
1. src/bin/open_bitcoind/canonical_startup.rs
1. src/bin/open_bitcoind/phase167_tests.rs
1. src/bin/open-bitcoind.rs
1. tests/phase167_canonical_startup.rs
1. src/context.rs
1. src/method.rs
1. src/method/tests.rs
1. src/dispatch.rs
1. src/dispatch/submit_block.rs
1. src/context/network.rs

The last five are the additional RPC owners. Original Task 1 and Task 2 are unchanged. New Task 1b lists all ten files it touches: those five plus context.rs, canonical_chain.rs, canonical_genesis.rs, canonical_history.rs and its tests. Its warning is ten cohesive wiring files, not merely five new owners. Fourteen total owners and this ten-file task require explicit independent/root scope assessment. Every final source must be at most 628 formatted lines. No minification, inline-path trick, hidden extra owner or file-limit exception is authorized.

This document explicitly supersedes the earlier context registrar amendment only for its registrar-only context.rs allowance, nine-owner scope and affected execution schedule. context.rs now additionally owns a dedicated private bridge field/type registration. The earlier amendment, root conditional acceptance and checker history remain byte-exact and attributable. Its original security, genesis, process and no-release conditions remain in force. No other original task XML, requirement or lifecycle is rewritten.

## Dependency graph and waves

The new provider depends on accepted Plan 12 and executes at wave 16. Plan 13 retains its direct Plan 12 dependency and adds Plan 30, moving to wave 17. Recursive dependency recomputation, using original wave floors and checking all shared owners, shifts only the following later wave fields:

| Plan | Before | Candidate | Dependency retained |
| ---- | -----: | --------: | ------------------- |
| 13   |     16 |        17 | 12 plus new 30      |
| 14   |     17 |        18 | 01,13               |
| 15   |     18 |        19 | 02,12,14            |
| 16   |     19 |        20 | 02,06,08,15         |
| 17   |     20 |        21 | 16                  |
| 18   |     21 |        22 | 17                  |
| 19   |     22 |        23 | 18                  |
| 25   |     23 |        24 | 19                  |
| 20   |     24 |        25 | 19,25               |

No other earlier plan is changed. New 30 has two tasks and 13 gains one: 50+2+1=53 tasks, 29+1=30 plans. Same-wave owners must not overlap, all shared-owner reuse must be dependency-ordered, and all dependencies must exist and precede consumers. Those are validation requirements rather than a source acceptance claim. Original Plan 02's intentional higher-wave independent producer remains an inherited advisory.

The original factory task may establish the safe factory before endpoint wiring. Task 1b adds the field and both required initializers together and switches final factories to the actual paired command producer. It must not leave Task 1 compiling against a field whose initializer is deferred. Its verification reruns the original factory test selection on final wired source. All five original commands, including real product/process genesis selections, still execute against final source; no earlier run automatically qualifies later changes.

## Safe factories and actual command identity

Plan 30 defines two public inherent methods on DurableSyncRuntime in the already-registered child:

```text
open_canonical_submission_configured(path, SyncRuntimeConfig, BasicFilterStartupMode)
  -> Result<(DurableSyncRuntime, Option<CanonicalSubmissionCommand>), SyncRuntimeError>

open_genesis_submission_configured(OpenedCanonicalGenesis, SyncRuntimeConfig, BasicFilterStartupMode)
  -> Result<(DurableSyncRuntime, Option<CanonicalSubmissionCommand>), SyncRuntimeError>
```

The first takes configuration/path only. After root accepts the actual Plan 12 runtime correction, missing/empty uses sealed fresh genesis, existing Regtest goes directly through the safe full-history wrapper, and other selectors keep strict genesis/fixed ordinary refusals. Filesystem shape selects a safe prover; it grants no authority. UnsupportedHistory-only retry misses actual Coins and segmented rows, and broad raw fallback is forbidden.

The second consumes the real private-field OpenedCanonicalGenesis through the existing safe genesis runtime constructor. The pinned old genesis helper already holds that seal and lease; reopening the same path would contend with itself. Preserve its exact function signature/test and directly pass the held seal. Non-Regtest genesis remains supported with no submission command. This genuine-seal route is root's explicitly accepted alternative to path-only acquisition, not permission for public arbitrary-runtime acceptance.

Both factories return the actual runtime and privately mint the command from that same runtime's actual network/store/selected params/incarnation. The command has private fields, no public Runtime/store/bool producer, no Default and no capability setter. A private internal helper may accept the runtime just produced by those two safe public paths; generic public runtime fixture constructors cannot call it. At each actual serialized command, check the captured store against concrete network.chainstate.store().inner() using shares_validation_history_store, actual authority incarnation, live origin, selected params and actual held HeaderStore context. A chain label is only an additional consistency check after safe construction.

The public command and typed outcome may remain inside the private child module. RPC obtains them through inference from the public inherent factories. A public consuming fold projects accepted position, duplicate hash, conclusive reject reason and actual inconclusive result without exporting hidden enum paths or constructing a bag of authority booleans. Operational errors remain a separate existing public error type. External no_run doctests on both factories must discover and compile the inferred submit/fold use; no parent reexport is assumed or silently added.

Public signatures are planned contracts. Before any 30/13 source work, root must accept final Plan 12 and verify its actual safe external APIs, complete original reproof/consume, root-plus-known handling and effect ordering. Current reviewed hashes/scoped strict passes and still-running required suites do not qualify that dependency.

## Peer-free processing and durable result

The node command takes only a block. It samples shell time internally and genuinely admits its header against the actual earned parent, retaining the exact HeaderStore observation. It publishes only genuine KnownOnly identities and reuses actual selected body, script, reward, BIP30, Coins and mempool stages. Reuse existing intrinsic CheckBlock logic and preserve source-pinned rejection ordering; selected PoW/branch context cannot be replaced by caller flags, block timestamp or numeric work. No fake outbound peer, handshake or fixture-header method is allowed.

Witness normalization is pure and separate: actual earned CandidateRules selects Segwit applicability; the helper reuses current witness_commitment_index and supplies only the applicable missing 32-byte reserve. It does not synthesize commitments, overwrite existing witness, recompute activation independently or mint canonical authority. The existing validation path checks the resulting block normally.

Accepted submission retains the actual normalized body, genuine trusted header checkpoint, full undo/counts/Coins/provenance and executes the existing ordered Always flush with the actual disk probe. Continue the existing BASIC owner/turn/barriers to the submitted durable/safe endpoint while retaining all ordinary per-turn/budget limits. A raw snapshot write, fake full flush or assumed background worker is insufficient; canonical daemon assembly suppresses the ordinary checkpoint workers. Keep one engine/lease, with clones sharing that actual owner.

The private command retains an operation mutex across this sequence. Existing authority/automatic-prune lock order remains; calling a public handle API that relocks authority from within an authority closure is forbidden. Use the existing private serialized helpers for admission/connect/flush, then release the inner guard for actual BASIC turns while retaining the operation mutex and rechecking actual target/identity/receipts. A concurrent external handle change cannot be represented as stale durable success.

Invalid consensus input preserves accepted tip/work/Coins/counts/undo/ScriptsValid/BASIC even if a genuinely admitted header remains KnownOnly. Duplicate, nonextending, disconnected and inconclusive states must be classified from real processing/receipt evidence; do not guess a string from a generic exception or scalar height. Operational failure after absorption must remain operational and retain the actual advanced state/receipts, not be reported as a nonmutating consensus rejection. No broad fork/differential qualification beyond the locked phase scope is claimed.

## Private RPC bridge and constructor closure

The dedicated maybe_canonical_submission field contains a private CanonicalSubmissionBridge. Its typed closure captures the actual opaque node command, not a replacement runtime/store supplied at invocation. Only the config-only factory or actual-genesis-seal consuming factory creates that bridge from the paired safe producer. There is no public bridge constructor/setter, Any/downcast, global registry or hidden token in daemon control, metrics or block-source fields.

Both actual context literals initialize None: canonical_chain.rs::new and context/network.rs::assemble_context. All generic config/store/network-handle/fixture builders delegate to one of these and remain closed. The independent UnprovedOwner guard remains required; global qualification gates stay Err, and Phase 14 adds separate implementation facts only. HTTP Clone shares the same Arc context. ManagedRpcContext has no Default/Clone reconstruction, and no public network-replacement setter exists. A pure canonical core plus arbitrary S or canonical envelope cannot mint submission authority.

The final factory must call the paired safe producer once, not first open a runtime and then reopen its locked datadir for a command. Existing genuine genesis factory helpers may be narrowly shared within the already-owned RPC files for selected/config/startup validation; no public arbitrary-runtime composition is added. Factory and invocation tests compare actual runtime/context/store/params and reject generic/fixture/raw/foreign attempts before effects.

## Exact pinned transport and BIP22 boundary

The primary pin is Knots `a9aee730466ac67d35a3c03ee24676be5e045878`. The following source chain governs the new handler:

- rpc/mining.cpp:1122–1170 defines required STR_HEX hexdata, optional ignored STR dummy, DecodeHexBlk, actual processing catcher and duplicate/inconclusive selection; BIP22ValidationResult is at 591–606.
- rpc/util.cpp:659–690,761–771,960–970 supplies arity/type rules: missing/excess count is RPC miscellaneous -1; wrong type is -3; optional dummy null is allowed. Named/mixed conflict/unknown/duplicate semantics follow the existing pinned framework, including args handling.
- util/strencodings.cpp:45–51 makes IsHex nonempty, even length and all hex digits, rejecting all outer/interior whitespace. Reuse the current converter only after that exact precheck, so its trim does not broaden accepted input.
- core_read.cpp:220–234 decodes one witness block and does not require stream exhaustion. The separate prefix parser returns a consumed length and permits remaining bytes for this endpoint; strict parse_block keeps Reader::finish and its old refusal tests.
- validation.cpp:4376–4397,4924–4971 and mining_basic.py's submitblock cases prove that parseable no-transaction/non-coinbase blocks reach conclusive bad-blk-length/bad-cb-missing, with valid arranged PoW/merkle where required.

The earlier root shorthand suggesting a -22 coinbase precheck was corrected after these exact source/functional-test reads. It was wrong for this pin. Preserve that research chronology; do not import another Core version's precheck. Malformed hex/witness serialization is -22 `Block decode failed`; correctly parsed coinbase violations are ordinary BIP22 reject strings after actual validation. Block error precedence still matters: malformed PoW/merkle can fail earlier.

Actual accepted is JSON null. Actual duplicate is `duplicate`; a genuinely absent conclusive processing result is `inconclusive`; conclusive invalid is its exact reason or `rejected` if empty; operational verification is -25. Do not invent duplicate-invalid or reuse transaction/package -26 mapping. Tests cover every distinction and full accepted-state equality for consensus rejection, while preserving post-absorption operational state honestly. This closes the specified source-derived boundary, not actual pinned-daemon differential qualification.

The real registry/normalizer/dispatcher is mandatory. method.rs remains compact with SubmitBlock enum/request/registry entries and an associated normalizer delegate. The cohesive new submit_block.rs can implement the feature's pure parameter/decode methods and thin runtime adapter together; it does not intercept HTTP or manufacture a second server. Tests update the explicit method list by exactly one Node baseline method and exercise positional/named/mixed/dummy-null/count/type/strict-hex/prefix/witness/BIP22 behavior through normalize_method_call and dispatch.

The codec allocation change is deliberately narrow. Current block, input, output and witness vectors reserve declared count before verifying input. Replace only allocation strategy with incremental fallible growth, retaining accepted bytes, CompactSize cap, wire signatures, valid/malformed behavior and strict trailing policy. A bounded pre-fix declaration such as 4096 demonstrates eager capacity without risking multi-gigabyte RED allocation. cfg(test)-only observations must show actual Vec capacity; an EOF assertion alone is not regression evidence. Final huge-declaration/tiny-input controls must verify no eager reserve. No production test flag or budget change is introduced.

## Source size and critical interface sketches

Normal-format sketches are kept under `amend13submission/plan/`; none is compilation or implementation evidence. The existing full field/default sketches are replayed against this exact baseline: context.rs 501→503, context/network.rs 579→580 and canonical_chain.rs 263→264, with exact whole-file inverses and no extra formatting. A planned inferred command/outcome and private closure contract makes every authority parameter explicit.

Expected readable envelopes, subject to actual final measurement:

| Owner group                                   | Target ceiling | Placement                                                               |
| --------------------------------------------- | -------------: | ----------------------------------------------------------------------- |
| consensus block.rs                            |            580 | Thin wrapper only; existing 554 lines leave limited room.               |
| consensus witness.rs                          |            500 | Pure helper and meaningful inline tests.                                |
| codec block.rs / transaction.rs               |      400 / 500 | Prefix/incremental allocation plus focused tests; retain old tests.     |
| node provider / new test child                |      450 / 600 | Orchestration and substantive behavioral matrix separated.              |
| RPC method.rs / method tests                  |      570 / 620 | Compact registry/type/delegate and inventory/normalization cases.       |
| RPC new handler / new canonical history/tests |       Each 620 | Feature-local pure boundary, private bridge and complete real controls. |
| RPC context/network.rs                        |            600 | Required None initializer with no unrelated refactor.                   |
| Every other owner                             |            628 | No packing, limits exemption or silent new child.                       |

These are feasibility envelopes, not permission to omit behavior. If actual implementation needs a helper/export/test owner, reaches 629 lines or cannot fit a reliable execution context, stop for another checked amendment. No projected skeleton is a test pass.

## Threat extension and decision coverage

Plan 30 contains its own STRIDE register. The unchanged Plan 13 register is extended by these concrete Task 1b obligations:

| ID          | Category | Boundary                  | Disposition | Mitigation                                                                                                       |
| ----------- | -------- | ------------------------- | ----------- | ---------------------------------------------------------------------------------------------------------------- |
| T-167-13-04 | S/E      | Generic context → command | mitigate    | Dedicated private bridge, both literals None, safe factory-only binding and foreign/generic refusal tests.       |
| T-167-13-05 | T/R      | Request/result projection | mitigate    | Exact pin params/hex/prefix and typed BIP22/operational projection; preserve full actual state/outcome receipts. |
| T-167-13-06 | D        | Untrusted block vectors   | mitigate    | Declared pure allocation owners, existing limits and actual capacity regression evidence.                        |

Security enforcement remains enabled. High findings block acceptance. No new authentication system or web certification is claimed.

Full below means complete planning coverage with original owners, never source or phase acceptance:

| Decision | Candidate task coverage                      | Status                                                                                |
| -------- | -------------------------------------------- | ------------------------------------------------------------------------------------- |
| D-01     | 30.2; original 13 tasks and 13.1b            | Full: selected safe producers, strict genesis/qualification/fixture boundaries.       |
| D-02     | 30.2 and unchanged 13 process task           | Full: actual ordinary default maturity/halvings and no spendable genesis.             |
| D-03     | 30.1/30.2; 13.1b                             | Full: actual candidate rules/selected body context, not caller activation.            |
| D-04     | 30.1/30.2; original 13 process evidence      | Full: existing witness selector and actual scripts/commitment checks.                 |
| D-05     | 30.2                                         | Full: real header admission and held shell observation, no timestamp-as-now.          |
| D-06     | 30.2 and existing prior owners               | Full: actual Coins/count/undo/BIP30 path and exact durable preservation.              |
| D-07     | 30.2; all 13 tasks                           | Full: actual peer-free owner, private command and normal authenticated process.       |
| D-08     | Both 30 tasks; 13.1b and original tasks      | Full: complete refusal/identity/all-handle durable comparisons.                       |
| D-09     | Every candidate task and original 17/19      | Full: exact baseline/inverses/pin/evidence attribution and source-before-guards.      |
| D-10     | Candidate checks and original 17/18/19/20/25 | Full: full native/fresh coverage/Bazel/security/goal/lifecycle and root saving gates. |

No deferred wallet/public-network/automatic repair/import/differential/prune/history qualification is added. No decision is reduced to a label or partial substitute.

## Evidence, full inverses and release

The immutable formal baseline is `amend13submission/plan/before-manifest.json`. It captures the exact 29 plans/50 tasks, 41 protected phase documents, 19 initial source/existence states plus separately captured transaction.rs, pinned source references and 3547 historical planning/research receipt hashes. All active Plan 12 source files, execution receipts/SUMMARY and root live metadata are excluded. Legitimate ongoing 12 work is not an immutability failure.

Original Plan 13 is the conditionally accepted SHA256 `31d302118652e0616d623e2794ec9d3ba7d05b73648fdb37e7f76bff4e594e8b`; its earlier registrar amendment remains `386f2df8dcd0889cd3d37c62479617f3ed06d9615e33dd9e0aa134fa5d028057`. The canonical checker prefix is exactly 232108 bytes, SHA256 `c9bcf1bdfaa2aff5f9cebd61c8979362be2ee192562d75ae4ab84b83ddf1ed73`. No planner verdict is appended to it.

Complete inverses must restore all 29 old plans byte-for-byte: remove 13's five owner entries, new context and intermediate task, restore its dependency/wave, and restore only the eight later wave fields. Original 13 task XML and all five commands must remain byte-exact; every other original task/command remains exact. The new 30 inverse is absence. The new amendment is a separately attributable file; prior amendments/reports/historical receipt bytes stay unchanged.

Planner checks cover YAML/frontmatter/task structure, lifecycle/requirements, complete contracts/inverses, actual DAG/shared owners, Markdown dialect/residuals, source/protected hashes and normal-format sketches. Preserve earlier Plan 13's existing formatter residual rather than normalize its old body; only new plan/amendment may be formatted. Those checks grant no execution or independent approval.

Actual execution must bind accepted 12 and 30 APIs/sources to all final six provider/fourteen RPC hashes, all original/new commands, named discovery, actual capacity observations, full local/header/Coins/flush/BASIC outcomes and the real authenticated daemon stop/reap/drop/restart evidence. Preserve failures and source classes. No old script or provider result becomes daemon evidence, and no local source-derived result becomes actual Knots differential success.

New sources need pinned breadcrumbs; 19 alone owns manifest reconciliation and contributor/parity docs. Original 17 source review precedes guard renewal; 17/20 retain compact-fallback adjudication; 18/19/20/25 retain every declared independent source/security/goal/native/fresh pure-core coverage/Bazel/lifecycle gate. Root alone saves with normal hooks after full acceptance. No source execution is released by this planning candidate.

## Planner checks awaiting independent review

The actual candidate inventory and dependency/shared-owner check derive 30 plans, 53 tasks and 25 waves, with no cycles, same-wave collisions or unordered shared owners. All 30 frontmatters/task structures validate. Complete inverses restore all 29 original plan bytes, including both original 13 task blocks and five commands; the new 30 inverse is absence. Twenty captured source/existence states, 41 protected documents, 3547 historical receipts and the exact 232108-byte canonical prefix remain intact. The sole inherited structural advisory is Plan 02's independent producer at a higher wave.

New plan/amendment and scoped whitespace checks pass; original formatter residuals are retained through normalized whole-file inverse checks on every modified old plan. Critical field/default sketches format to 503/580/264 lines with complete inverses, and the prefix-interface sketch leaves the entire strict parse_block function unchanged. The node/pure/RPC interface sketch is normal formatted; no API or source was compiled.

An initial validator expected the literal wording “actual Vec capacity,” while the required behavior says “actual block/input/output/witness Vec capacity.” That metadata-only false failure and its script remain as planner-validation-first-capacity-wording.json and validate-first-capacity-wording.ts. The initial isolated new-section formatter check requested dedenting only the new task's closing metadata after its multi-paragraph action; that receipt/draft remains under first-sections-check. Only new additions were corrected. No original task/body was formatted, no result was promoted to source evidence, and no independent approval is claimed.

## Newly accepted dependency during planning

Root accepted final Plan 12 while these formal candidates were being validated. New separate inputs under plan/accepted12/ preserve only the explicitly authorized acceptance JSON and accepted SUMMARY, leaving the original before-manifest and its exclusions unchanged. Acceptance SHA256 is `f7ec6e6455ba1002196062d072d430459e53f19323e8d40d1558266ab8be6268`; accepted SUMMARY is `5e72ccfb241a8b3d935bb70dad0fda81d28ed317d3b3479bb3c61d76e476415a`; accepted source freeze is `ac5639f959643cada49c16ce40912c9bfc95c6fb1e9ee7ac739b00b8a48c5713`. Root's earlier producer SUMMARY `d32f8cf9...` remains historical before its setup-cause clarification; no source/test change is attributed to that edit.

Read-only API inspection confirms public DurableSyncRuntime::reopen_canonical_history_configured(path,config,mode), open_canonical_history_configured(actual opened seal,config,mode), and open_canonical_genesis_configured(actual genesis seal,config,mode). The first genuinely obtains and consumes the storage seal. These are accepted dependency APIs, not the new planned submission factories. No current12 source or execution tree was added to the immutable baseline. Independent checking and root release of 30/13 remain pending; dependency acceptance is not an automatic next-plan execution release.
