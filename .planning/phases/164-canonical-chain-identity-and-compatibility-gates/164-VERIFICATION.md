---
phase: 164-canonical-chain-identity-and-compatibility-gates
verified: 2026-10-14T10:38:59Z
status: passed
score: 27/27 must-haves verified
generated_by: gsd-verifier
lifecycle_mode: yolo
phase_lifecycle_id: 164-2026-10-14T01-59-33Z
generated_at: 2026-10-14T11:09:32Z
closure_reconciled_at: 2026-10-14T10:58:14Z
saving_candidate_reconciled_at: 2026-10-14T11:09:32Z
lifecycle_validated: true
overrides_applied: 0
required_artifacts_verified: 29
key_links_verified: 22
reviewed_source_files: 146
source_snapshot_sha256: d9738977df3ea3c28f802db21fadcf6707b6717b7824118ab132ae6d6340274f
native_verification_source_snapshot_sha256: da4b07c93d86bdc74aa93efb2ff62ebfa290c8e790cca66bd08656df6c0340c0
core_review_snapshot_sha256: c237c7b7ca73909d279c8687fff1a2712d6b844cd954b1a8eec8a327632adbf5
shell_review_snapshot_sha256: e6fb1285c6716328b75a55774dc460555f08a574eb58c89437150cfbad63f434
automation_review_snapshot_sha256: 62a501bcf4635645e1301c7e227cc86701eecf0b5e415b1affed8bd4f74dd764
security_threats_closed: 30
security_threats_open: 0
native_verification:
  command: "bash scripts/verify.sh"
  mode: full-default
  session: 41473
  exit_code: 0
  duration_ms: 3621973
  log: /tmp/open-bitcoin-phase164-native-sixth.log
  workspace_tests_passed: 4450
  rpc_client_tests_passed: 7
  pure_core_coverage_tests_passed: 1706
  rust_result_summaries: 57
  rust_test_executions_passed: 6163
  rust_test_executions_failed: 0
  existing_ignored_tests: 6
current_ordinary_qualification: false
deferred:
  - truth: "Exact target-derived work and legacy synthetic-work decisions"
    addressed_in: "Phase 165"
    evidence: "Shared Exact Work Authority; CWOR-01, CWOR-02, CWOR-03 and CWOR-05."
  - truth: "Positive canonical durable bootstrap, all-handle reopen and genesis BASIC"
    addressed_in: "Phase 166"
    evidence: "Canonical Durable Bootstrap and Genesis BASIC; CBOT-02, CBOT-03 and CWOR-04."
  - truth: "Ordinary network- and candidate-history-derived validation"
    addressed_in: "Phase 167"
    evidence: "Candidate-History Validation Rules; CVLD-01 through CVLD-04 and CVLD-06."
  - truth: "Actual pinned-Knots ordinary-validation differential qualification"
    addressed_in: "Phase 168"
    evidence: "Actual Pinned-Knots Differential Qualification; CDIF-01 through CDIF-05."
---

# Phase 164: Canonical Chain Identity and Compatibility Gates Verification Report

**Phase Goal:** Operators can identify the selected canonical chain and receive a nonmutating refusal for incompatible roots, networks or unsupported validation modes.

**Verified:** 2026-10-14T10:38:59Z
**Status:** passed
**Re-verification:** No — initial formal verification. Five failed native attempts and separate preflight failures were repaired before this report.
**Closure documentation reconciled:** 2026-10-14T10:58:14Z. Actual native acceptance remains bound to its original source snapshot.
**Saving-candidate EOF reconciled:** 2026-10-14T11:09:32Z. One extra terminal LF was removed from the contract data file; no data or executable behavior changed.

The phase achieves exact identity recognition and enforced ordinary refusal. All four ordinary histories remain unsupported. Exact matching genesis predicates return identity success; they create no accepted history, canonical bootstrap, work authority, spendable genesis output or BASIC authority. The passed score therefore does not rely on vacuous “accepted root” behavior.

## Scope and Evidence Method

Read the three complete roadmap criteria, requirements, CONTEXT/RESEARCH and all thirteen PLAN/SUMMARY artifacts. The plans supply 33 truth entries, 28 unique after exact deduplication. Four plan entries restate parts of the three roadmap criteria; retaining the full roadmap wording yields 27 merged truths. No roadmap criterion was dropped.

Independently traced actual selected-chain mappings, storage openers, owner guards, effects, fallback paths and historical attribution. SUMMARY statements were treated as inventories, then checked against source, actual logs and immutable bytes. No prior VERIFICATION.md or verification overrides existed.

Path abbreviations below are repo-relative: C = packages/open-bitcoin-consensus/src; N = packages/open-bitcoin-node/src; R = packages/open-bitcoin-rpc/src; L = packages/open-bitcoin-cli/src/operator; A = scripts. The moved preservation artifact is N/storage/chain_compatibility/tests/preservation.rs.

AGENTS.md Repo-Local Guidance, AGENTS.bright-builds.md, placeholder-only standards-overrides.md, managed architecture/code-shape/verification/testing/Rust/TypeScript standards and both complete active lesson files informed the checks. Active lessons total 7,188 bytes and 2,397 conservative estimated tokens. No project skill directories were present. Verification stays within the parent GSD execute-phase lifecycle.

## Goal Achievement

### Observable Truths

| # | Truth | Status | Evidence |
| --- | --- | --- | --- |
| 1 | A contributor can check the exact pinned genesis header/body identity for the selected existing network and observe rejection of a different null-parent root. [CBOT-01] | VERIFIED | C/chain_spec.rs compares all header fields and the complete body. Frozen 80-byte headers, 204-byte transaction, 285-byte bodies, four hashes, 12 foreign pairs and mutations are tested. Independent byte hashing reproduces all four pinned hashes and the common txid. |
| 2 | An operator opening foreign-network or noncanonical-root durable state gets a bounded diagnostic while prior data remains intact, before automatic coins/schema migration, initialization, prune resume or BASIC writes. The same read-only inspection boundary is available to the later legacy-work decision. [CBOT-04] | VERIFIED | N/storage/chain_compatibility.rs::open_ordinary has only a filesystem-derived Err path. Closed Fjall preservation tests compare complete physical manifests, exercise real migration/prune controls on separate fixtures and prove the backend spy is live before Database::builder. Identity and work observations remain separate Unavailable values. |
| 3 | An operator can identify the supported validation/evidence scope of mainnet, testnet3, signet and regtest; missing consensus prerequisites cause explicit unsupported refusal before accepting history, including absent Signet challenge validation. [CVLD-05] | VERIFIED | Private immutable C/chain_spec.rs facts expose identity-only, ordinary_history=false and bootstrap/work/history prerequisites; Signet additionally requires signet_challenge_validation. All ordinary constructors and admission paths enforce the refusal. |
| 4 | Missing paths stay absent and empty directories stay empty when ordinary activation refuses. | VERIFIED | Ordinary opening never creates a backend; actual node, daemon, CLI and direct-binary tests compare absent/empty physical shapes before and after refusal. |
| 5 | Refusal distinguishes unavailable identity proof from a proved wrong stored network. | VERIFIED | CompatibilityRefusal exposes selected expected identity and unavailable observed network/root/work. ExistingUnproved is not reported as proved wrong-network state. Fixed text and JSON remain below 1,024 UTF-8 bytes. |
| 6 | Production header and direct-body admission reject foreign/null-parent roots before any state or receipt changes. | VERIFIED | N/network/validation_history.rs preflights the batch and checks within insertion closure; canonical_admission.rs checks selected roots; mempool_lifecycle.rs checks bodies before staging/cache effects. |
| 7 | Ordinary local/stored/reorg/import paths refuse missing canonical prerequisites, including Signet challenge validation. | VERIFIED | Shared owner guards run before local/stored connect, cached disposition shortcuts, replacement preview and canonical header assignment. Empty batches and empty replacement branches also require support. |
| 8 | Explicit lower-level synthetic fixture owners retain narrow regression behavior without granting product authority. | VERIFIED | Private AdmissionScope distinguishes Fixture from immutable Canonical binding. Rebinding and wrappers preserve the first selection; supplied fixtures cannot satisfy ordinary RPC qualification. Existing regression suites pass. |
| 9 | Daemon durable attempts refuse before backend open, including sync, inbound, prune and BASIC modes. | VERIFIED | R/bin/open-bitcoind.rs::main calls ordinary store/authority selection before serving. All 31 nonzero durable-trigger combinations across four selectors are exercised; explicit BASIC disable also routes through the gate. |
| 10 | The metrics fallback cannot mutate an unproved store. | VERIFIED | R/bin/open_bitcoind/inbound_metrics.rs uses open_ordinary for fallback and requires support for supplied handles before worker spawn. Real closed/missing/empty sources retain physical equality. |
| 11 | Explicit legacy daemon fixtures remain separately classified regression evidence. | VERIFIED | Named fixture_runtime helpers are under actual cfg(test) ownership; ordinary main and refusal tests retain product calls. Shared worker/assembly logic preserves original fixture behavior. |
| 12 | A supplied raw/fixture/mismatched network handle refuses before RPC prune/wallet/mempool mutation. | VERIFIED | R/context/network.rs resolves selectors, checks actual immutable owner, rejects UnprovedOwner/ChainMismatch, then requires scope before configure_context_network. Memory/durable owners, actual daemon handle type and zero-store-call assertions are tested. |
| 13 | Wallet/mempool fallback cannot mutate an unproved store or silently replace it with accepted memory authority. | VERIFIED | R/context/wallet_state.rs and mempool_recovery.rs use ordinary open/support guards. Errors propagate; raw fallback remains only in explicitly classified fixture helpers. |
| 14 | Memory-backed ordinary RPC construction and direct body submission obey the selected network's refusal scope. | VERIFIED | All ordinary aliases delegate to try_from_runtime_config_with_store and require support before wallet/network construction. Actual owner body submission uses the same guard. |
| 15 | Offline status/pause/resume refuse unproved durable data without mutable reads or writes. | VERIFIED | L/runtime/support.rs opens ordinarily before runtime metadata read/save. Four selectors, three operations, two formats and seven source shapes produce 168 preserved-source attempts. |
| 16 | Wallet backup/name resolution cannot turn refusal into a guessed wallet or backup artifact. | VERIFIED | L/wallet/name_resolution.rs and wallet.rs gate before registry/name and destination/export effects. Actual tests cover 56 lookup and 336 backup attempts, plus binary refusal cases. |
| 17 | Soak enrichment reports compatibility unavailable without opening or changing source. | VERIFIED | L/soak/runtime/chain_compatibility.rs projects typed unavailable facts and preserves prior sync/peer observations; soak start returns before index/ledger/report writes. Tests cover 28 enrichment attempts and both start bounds. |
| 18 | Operators can audit exact identity/support/refusal for all four selectors and safe next steps. | VERIFIED | Current companion, README, support matrix and parity catalog agree with actual fixed diagnostics, testnet selector spelling, Signet limitation and separate-datadir guidance. UAT uses repo-local Cargo/Bazel commands. |
| 19 | Historical v2.5 receipts retain their original source-bound synthetic evidence meaning. | VERIFIED | Independently compared 123 existing phase artifacts and all 12 original tooling snapshots with fixed origin 6cf34f1b. Exact historical readers preserve original digest 14fa9cc5… and refuse unknown/missing current bytes. Raw current digest differs and ordinary qualification remains false. |
| 20 | Offline native verification rejects production fixture bypasses, unguarded openers and overstated canonical claims. | VERIFIED | Current checker binds 93 Rust inputs/246 exact executable regions, immutable facts, actual ownership and default registration. All 355 final guard tests and the complete native replay pass. Literal whitespace and comment/attribute decoys are rejected. |
| 21 | Existing daemon fixture callers preserve original narrower assertions through explicit factories. | VERIFIED | Plan 09 caller diffs retain arguments/assertions; ordinary daemon edge is unchanged. Full daemon suite passes 112 tests with one existing ignored control. |
| 22 | Ordinary daemon and canonical tests retain guarded constructors without a cfg(test) bypass. | VERIFIED | Actual product helper calls remain ordinary in test builds; named fixture adapters have separate ownership. Executable registration checks and mutation tests reject disabled/commented/spoofed declarations. |
| 23 | Existing context and cross-crate fixture callers preserve narrower assertions through explicit factories. | VERIFIED | Plan 10 conversions are scoped to fixture calls. Canonical/context/config/black-box/operator regressions and complete workspace replay pass. |
| 24 | Existing dispatch and benchmark fixture callers preserve narrower assertions through explicit factories. | VERIFIED | Plan 11 keeps mixed-file test ownership; sync-status fixture reuses its already-open store instead of swallowing a second-open error. Dispatch and benchmark verification pass. |
| 25 | Existing inbound fixture callers preserve narrower assertions through explicit factories. | VERIFIED | Plan 12 retains peer behavior, arguments and assertions. Inbound regression and full workspace replay pass; no new ignored test was introduced. |
| 26 | Existing HTTP fixture callers and coupled closure preserve narrower assertions through explicit factories. | VERIFIED | Plan 13 mechanical factory changes retain HTTP assertions and actual ordinary context boundaries. HTTP and whole workspace replay pass. |
| 27 | All 39 affected callers compile and pass actual coupled regressions before native finalization. | VERIFIED | Actual coupled logs show canonical 11, context 49, config 91, repaired dispatch 86, inbound 94, HTTP 19, daemon 112, black-box 3, operator flows 11 and benchmark all-target compilation. Final native replays the whole workspace successfully. |

**Score:** 27/27 truths verified. No overrides applied.

### Required Artifacts

All 29 planned artifact entries are substantive and wired. The preservation implementation moved unchanged to a conventional test path; its full 20,556-byte SHA remains 3d004f91bd72a0f3902588a36fca0438b04b5ac034bc9e227893664496835630.

| Plan | Artifact | Status | Substance and wiring |
| --- | --- | --- | --- |
| 01 | C/chain_spec.rs | VERIFIED | Public typed identity/support APIs, exact predicates and immutable refusal; exported by lib.rs and consumed by node/adapters. |
| 01 | C/chain_spec/genesis.rs | VERIFIED | Closed exact pinned constants/common coinbase; called by ChainSpec. |
| 01 | C/chain_spec/tests.rs | VERIFIED | Independent vectors, adversarial identity/support and real formatter-failure behavior; cfg-test registration and native execution. |
| 02 | N/storage/chain_compatibility.rs | VERIFIED | Bounded classifier, typed diagnostic and inherent ordinary opener; public storage module and adapter callers. |
| 02 | N/storage/chain_compatibility/tests/preservation.rs | VERIFIED | Real closed-Fjall physical equality and mutation controls; explicit compiler-valid cfg-test path. Replaces original planned preservation.rs location. |
| 02 | N/storage/fjall_store.rs | VERIFIED | Raw recovery/migration infrastructure documentation and live pre-builder test spy; ordinary API is implemented in the sibling compatibility module. |
| 03 | N/network/canonical_admission.rs | VERIFIED | Private immutable owner scope and shared guards; called by admission/connect/import paths. |
| 03 | N/network/canonical_admission/tests.rs | VERIFIED | Whole-owner state/dispatch/cache/import/reorg preservation; real test registration. |
| 03 | N/sync/open_runtime.rs | VERIFIED | Ordinary support check before initialize_configured; product daemon uses ordinary entrypoint. |
| 04 | R/bin/open_bitcoind/tests/canonical_chain.rs | VERIFIED | Actual ordinary daemon/metrics selectors, triggers, sources and zero-effect regressions. |
| 04 | R/bin/open-bitcoind.rs | VERIFIED | Main uses ordinary open/authority before listeners, context and workers. |
| 05 | R/context/canonical_chain.rs | VERIFIED | Fallible ordinary aliases, fixed typed errors and separate fixture API; exported context construction. |
| 06 | L/runtime/chain_compatibility.rs | VERIFIED | Shared config-derived selected identity; used by offline controls and wallet/soak adapters. |
| 06 | L/wallet/tests.rs | VERIFIED | Actual production lookup/backup/live failure source/destination preservation tests. |
| 06 | L/soak/tests/recovery_resume_bounds.rs | VERIFIED | Real unavailable-enrichment preservation behavior. |
| 07 | docs/parity/v2-6-canonical-chain-identity.md | VERIFIED | Exact source-pin/support/refusal/UAT surface; current catalog and navigation reference it. |
| 08 | A/check-phase164-canonical-identity.ts | VERIFIED | Fixed bounded source inputs, raw-byte/ownership/fact/claim/history checks; default native stage. |
| 08 | A/check-phase164-canonical-identity.test.ts | VERIFIED | Real normal and adversarial source mutations, including literal bytes and executable ownership. |
| 08 | A/verify.sh | VERIFIED | Exactly one mandatory current checker/test pair after provenance; full native acceptance observed. |
| 09 | R/bin/open_bitcoind/inbound_startup.rs | VERIFIED | Fixture-only helper uses explicit factory; product shared-context path remains separate. |
| 09 | R/bin/open_bitcoind/tests/inbound_runtime.rs | VERIFIED | Actual fixture callers retain original arguments and assertions. |
| 10 | R/config/tests/prune_config.rs | VERIFIED | Named fixture construction retains prune behavior assertions. |
| 10 | packages/open-bitcoin-cli/tests/operator_flows/rpc_harness.rs | VERIFIED | Cross-crate explicit fixture harness with retained operator assertions. |
| 11 | R/dispatch/filter_index.rs | VERIFIED | Factory changes remain inside original cfg-test module; production dispatch unchanged. |
| 11 | packages/open-bitcoin-bench/src/cases/rpc_cli.rs | VERIFIED | Explicit fixture benchmark construction; native list/smoke/report pass. |
| 12 | R/inbound_listener/tests.rs | VERIFIED | Explicit test factories and original test ownership. |
| 12 | R/inbound_listener/tests/reconnect_policy.rs | VERIFIED | Named fixture construction with retained reconnect assertions. |
| 13 | R/http/filter_index.rs | VERIFIED | Fixture factory remains within original test ownership. |
| 13 | R/http/tests/filter_index.rs | VERIFIED | Explicit HTTP fixtures retain authenticated filter behavior assertions. |

The GSD artifact helper reported 27/29 because it treated each inline exports array in Plans 01/02 as one literal export name. Manual Rust declarations, lib.rs exports, public storage module wiring and compilation resolve both parser false positives. This is not a verification override or a missing implementation. All 22 helper key-link checks passed, then actual call ordering was independently traced.

### Key Link Verification

| # | From | To | Status | Actual connection |
| --- | --- | --- | --- | --- |
| 1 | C/chain_spec.rs | C/chain_spec/genesis.rs | WIRED | Exact selected header/body construction and comparisons. |
| 2 | C/chain_spec/tests.rs | pinned kernel/chainparams.cpp | WIRED | Frozen wire bytes and asserted hashes match local pin a9aee730…. |
| 3 | N/storage/fjall_store.rs | N/storage/chain_compatibility.rs | WIRED | Same FjallNodeStore inherent ordinary method; no raw backend call on ordinary path. |
| 4 | N/storage/chain_compatibility.rs | C/chain_spec.rs | WIRED | Typed selected identity, immutable scope and prerequisites populate refusal. |
| 5 | N/network/validation_history.rs | N/network/canonical_admission.rs | WIRED | Batch guard and insertion-closure check precede insertion/publication. |
| 6 | N/network/mempool_lifecycle.rs | N/network/canonical_admission.rs | WIRED | Body/reorg guards precede caches, preparation and preview. |
| 7 | N/sync/open_runtime.rs | C/chain_spec.rs | WIRED | require_ordinary_history precedes initialize_configured. |
| 8 | R/bin/open-bitcoind.rs | N/storage/chain_compatibility.rs | WIRED | Selected-chain open_ordinary before authority construction. |
| 9 | R/context/network.rs | N/network/canonical_admission.rs | WIRED | Actual owner query/binding and support before context mutation. |
| 10 | R/context/network.rs | R/context/wallet_state.rs | WIRED | Fallible guarded wallet construction; no swallowed ordinary fallback. |
| 11 | R/context/mempool_recovery.rs | N/storage/fjall_store.rs | WIRED | Ordinary fallback before recovery and policy clock use. |
| 12 | L/runtime/support.rs | L/runtime/chain_compatibility.rs | WIRED | Resolved selected chain precedes offline store access. |
| 13 | L/wallet.rs | N/storage/chain_compatibility.rs | WIRED | open_ordinary precedes backup/name/destination effects. |
| 14 | L/soak/runtime.rs | N/storage/chain_compatibility.rs | WIRED | Enrichment helper returns typed unavailable bounds; start refuses before ledger creation. |
| 15 | docs/parity/index.json | canonical identity companion | WIRED | Unique current surface has exactly CBOT-01/CBOT-04/CVLD-05 and current evidence paths. |
| 16 | A/verify.sh | A/check-phase164-canonical-identity.ts | WIRED | Mandatory default test/check pair; actual full replay passes. |
| 17 | Current checker | R/bin/open-bitcoind.rs | WIRED | Actual ordinary opener/authority functions are exact raw-byte contract inputs. |
| 18 | R/bin/open_bitcoind/inbound_startup.rs | explicit context factory | WIRED | Named from_fixture_runtime_config under real cfg-test ownership. |
| 19 | R/config/tests/prune_config.rs | explicit context factory | WIRED | Explicit fixture constructor retains original config. |
| 20 | R/dispatch/filter_index.rs | explicit context factory | WIRED | Original test module owns the call. |
| 21 | R/inbound_listener/tests.rs | explicit context factory | WIRED | Fixture constructor never selected by ordinary configuration. |
| 22 | R/http/filter_index.rs | explicit context factory | WIRED | Original HTTP test module owns the call. |

Additional first-effect trace covers Headers/Block/CompactBlock/BlockTxn dispatch before peer/reconstruction evidence and ReceivedBlock before compact cleanup. Immutable binding survives clone/shared wrappers. Generic networkless arithmetic/data carriers and raw storage remain lower-level fixture/infrastructure APIs; they cannot qualify an ordinary product route.

### Data-Flow Trace (Level 4)

No frontend renderer is introduced. The dynamic outputs are diagnostics and operator availability facts; their sources were traced rather than inferred from placeholder values.

| Artifact/output | Data | Real source | Status |
| --- | --- | --- | --- |
| CompatibilityRefusal | Selected identity/hash, observation, missing prerequisites | Existing resolved selector → pure ChainSpec; actual symlink metadata/first directory entry → typed observation | FLOWING; observed root/network/work correctly unavailable |
| Ordinary RPC errors | Owner proof/mismatch/support refusal | Actual immutable managed owner query and configured selectors before any context effect | FLOWING; no accepted memory fallback |
| Offline CLI refusal | Selected identity and bounded error category | Config resolution → shared selector → actual ordinary opener result → human/JSON error projection | FLOWING |
| Soak unavailable bounds | Fixed compatibility unavailable reason | Actual ordinary refusal; prior sync/peer observations remain untouched | FLOWING; no invented durable progress |
| Current-source checker | Real current source/facts/ownership | Fixed bounded regular-file reads; raw current regions remain distinct from historical attribution maps | FLOWING |
| Historical attribution | Original versus current digest | Exact immutable snapshots plus strict complete-file original/current SHA pins | FLOWING; currentOrdinaryQualification=false |

### Behavioral Spot-Checks

| Behavior | Check | Actual result | Status |
| --- | --- | --- | --- |
| Exact independent genesis identity | Read-only stdlib byte hashing of frozen transaction/headers | Four 80-byte headers, four 285-byte blocks, common 204-byte transaction; all pinned header hashes, txid and auxiliary digests match | PASS |
| Current ordinary/evidence boundary | Read-only pinned Bun current checker | 93 fixed Rust inputs, 246 reviewed source edges, zero findings; native also replays checker and all 355 mutations | PASS |
| Nonmutating refusal | Actual registered Rust tests, closed-source physical manifests and backend spy | 24 storage checks; owner/header/body/context/daemon/CLI source/destination preservation passes in complete native replay | PASS |
| Required full verification | Parent-owned default bash scripts/verify.sh, session 41473 | Actual exit 0; log completion 1h 0m 21.973s, 3,621,973 ms | PASS |

The verifier started no server, Cargo/Bazel job, additional heavy suite or source mutation. It read existing actual execution logs and independently checked source/bytes. The only written artifact is this report.

## Accepted Native Evidence and Failure History

Accepted log: /tmp/open-bitcoin-phase164-native-sixth.log, source snapshot **da4b07c93d86bdc74aa93efb2ff62ebfa290c8e790cca66bd08656df6c0340c0**. Parent reported actual process exit 0; the verifier independently read completion and summed all 57 Rust result summaries: **6,163 passing executions, zero failed, six existing ignored**. This is **4,450 workspace executions plus seven RPC-client executions plus 1,706 repeated pure-core coverage executions**, not 6,163 distinct tests.

The full default contract passed all registered Bun tests/checkers/generators, panic/file-length/architecture/parity policies, cargo fmt check, all-target/all-feature Clippy with warnings denied, all-target/all-feature workspace build, workspace tests and doc-tests, explicit RPC-client tests, benchmark list/smoke/report validation, six Bazel targets and build provenance, and native pure-core coverage. No --fast or stage suppression supplied acceptance.

The direct coverage text evidence has no Uncovered Lines section and no zero-count rendered instrumented rows, including formatter error propagation. It contains 26,296 plain integer-count rows and 1,720 positive abbreviated-count rows. This establishes executed-line coverage; it is not a branch-coverage or actual Knots qualification claim.

Earlier outcomes remain failures, separately attributable:

| Attempt | Session | Actual outcome | Failure and retained log |
| --- | --- | --- | --- |
| Native 1 | 63399 | Exit 1 after 2.979s | Obsolete Phase 75 ordinary ledger-write test anchor; /tmp/open-bitcoin-phase164-native.log |
| Native 2 | 24793 | Exit 1 after 8.055s | Phase 97 stale fixture constructor and former ordinary-alias location; /tmp/open-bitcoin-phase164-native-retry.log |
| Native 3 | 29227 | Exit 1 after 12m14.140s | Phase 159 original-proof factory hashed raw migrated fixture: 636 passed/1 failed; /tmp/open-bitcoin-phase164-native-third.log |
| Native 4 | 16533 | Exit 1 after 24m49.836s | Lexical panic policy classified the old test-helper path, fixed genesis expectations and lint attribute; /tmp/open-bitcoin-phase164-native-fourth.log |
| Native 5 | 8391 | Exit 101 after 24m47.226s | Three test-only clippy::err_expect findings; /tmp/open-bitcoin-phase164-native-fifth.log |
| Focused path preflight | Plan 08 recorded run | Compilation failed | Initial short preservation path resolved to nonexistent src/storage/tests/preservation.rs; /tmp/phase164-panic-node.log |
| Separate coverage preflight | 91777 | Exit 1 | Sole uncovered initial fallible diagnostic write at chain_spec.rs:218; formatter-failure test subsequently proves propagation |

Repairs retained original assertions and receipt bytes: narrowed current producer/fixture contracts; exact raw-byte literal/ownership checks; three historical factory import/hash substitutions; byte-identical preservation relocation; four proven constant/lint classification entries; equivalent typed test assertions; one meaningful failing-writer test. None supplies an ordinary acceptance bypass.

The prior 4,774 focused checker passes belong to the earlier source snapshot. Later whole guards/provenance and selected native/factory checks retain their exact scopes; complete native replay supplies whole-suite acceptance for the recorded native snapshot. The earlier 82/82 normal inventories alone were never treated as full acceptance. Subsequent closure-document checks are separately attributed below.

### Source, Review and Security Binding

All **146** current implementation/test/automation/parity/snapshot fingerprints independently match the saving-candidate 164-REVIEW.md. Ordered repo-relative file:SHA rows joined with LF, without trailing LF, recompute to **d9738977df3ea3c28f802db21fadcf6707b6717b7824118ab132ae6d6340274f**. The accepted native snapshot remains **da4b07c93d86bdc74aa93efb2ff62ebfa290c8e790cca66bd08656df6c0340c0**; the intervening closure-document snapshot was **0d1d320914d988e60cfcbf680c2ab71a06675c03a724b9f766fe006864ea4b21**.

Exactly three current documentation files changed after native acceptance: README.md adds the verification link, the canonical-identity companion records identity/refusal closure, and docs/parity/index.json records the current surface's completed status/evidence/rationale. One additional source row subsequently changed only by removing an extra terminal LF from scripts/check-phase164-canonical-identity/contracts.ts. Its final SHA is 0a3f44db805a87f4aa53e170aebdc2b0d57930e101719461c46406087a77fda9; independently appending exactly one LF reconstructs prior SHA 2c9b7f8819534ac29d19836b6f2f7f60c6715e553e19473b6c779375fd182167. All contract arrays/rows and every other byte are unchanged. Replacing just these four current hashes with their prior reviewed hashes independently reconstructs the complete accepted native aggregate. Thus all **142** other source rows remain byte-identical; all Rust/runtime/test/snapshot bytes are unchanged. CORE/SHELL bindings are unchanged; AUTOMATION56 is renewed to the frontmatter aggregate. The clean closure/EOF audit confirms bounded identity/refusal claims and exact-byte reversals, all 82 recorded normal closure checks have actual exit 0 and existing logs, and the normal current checker still reports 93 inputs/246 edges. Eight metadata artifacts were also trimmed at EOF without content changes. This is documentation/EOF reconciliation, not a fabricated full-native rerun or new ordinary qualification. The final normal pre-commit hook remains parent-owned.

Clean scoped reviews cover CORE 22, SHELL 68 and AUTOMATION 56 paths, with their exact aggregates in this report frontmatter. The security audit independently binds the same 146-file snapshot and reports **30 declared controls closed, zero open, no accepted/ transferred risks**. Scope is the declared phase controls, not a whole-repository security certification. Native acceptance is independently established by the completed parent run above.

All 123 historical phase artifacts and twelve original-tooling snapshots remain byte-identical to origin 6cf34f1b96cdb7e719752395b2c52c8f531b559c. The original nine snapshot bindings remain intact; three exact frozen factory bindings were appended. Current tooling digest **6e1e19fd449f2bf77f4511cdd1cec8599fb45148ac87808d8be9575f48ba7570** stays distinct from original historical digest **14fa9cc585c9cf8d9b2c0ae04d7f92e6411b38fb7d19128359126eee2735f977**. Original receipts, source IDs, dates, claims and raw physical preservation assertions were not renewed into current ordinary qualification.

### Requirements Coverage

All plan requirement declarations resolve to the three Phase 164 IDs. REQUIREMENTS.md maps no additional orphaned requirement to this phase.

| Requirement | Primary owner | Description | Status | Verified evidence |
| --- | --- | --- | --- | --- |
| CBOT-01 | 164-01 | Selected existing network binds exact pinned genesis identity and rejects different null-parent roots | SATISFIED | Truth 1; frozen vectors, independent pin/hash checks, actual shared root guards. Identity-only success is explicitly separate from accepted history. |
| CBOT-04 | 164-02 | Bounded refusal before migration/init/prune/index effects, prior data intact | SATISFIED | Truths 2, 4–5 and ordinary adapter traces; real closed-source physical equality and pre-backend spy. Observed stored identity remains unavailable. |
| CVLD-05 | 164-06 | Exact supported/evidence scope and refusal before unsupported block acceptance, including Signet | SATISFIED | Truth 3 and admission/RPC/CLI support traces; all four scopes refused, Signet challenge missing, fixed operator diagnostics and source claims. |

Supporting plans retain cross-cutting coverage. After actual passed goal verification and lifecycle validation, the parent recorded exactly three unique completed owners: Summary 01 → CBOT-01, Summary 02 → CBOT-04, Summary 06 → CVLD-05. The other ten summaries claim no completed requirement. REQUIREMENTS.md has three Complete and 27 Pending rows; only Phase 164 is complete with 13/13 plans. Phase 165 remains unstarted, and automatic chaining remains disabled. The independent closure audit also preserves the archived v2.3 metric of 69. This verifier independently checked the three summary owners and requirement counts and changed none of those flags.

### Deferred Items

No in-scope Phase 164 truth is deferred. These roadmap-owned capabilities explain the currently enforced missing prerequisites:

| Capability | Later owner | Specific roadmap evidence |
| --- | --- | --- |
| Exact work and safe synthetic-work compatibility | Phase 165 | Target-derived work, shared header/connect/reorg authority and pre-mutation legacy refusal |
| Positive canonical bootstrap/reopen and genesis BASIC | Phase 166 | Canonical best block, unspendable genesis outputs, interrupted initialization and bound work reopen |
| Candidate-history validation | Phase 167 | Network subsidy/maturity, history-derived deployments/script flags/difficulty and BIP30 contexts |
| Actual pinned Knots differential qualification | Phase 168 | Actual pinned executable, canonical regtest maturity/halvings and paired process outcomes |

### Anti-Patterns and Disconfirmation

No blocker or incomplete in-scope implementation was found. Anti-pattern scanning covered current reviewed paths. The only TODO-like matches were deny-lint names and explicit documentation that later ordinary prerequisites are not implemented. Empty collections in owned fixture setup are populated/arranged evidence, not product data stubs. Low-level raw open/fixture constructors are explicit mechanisms, with actual ordinary call-site exclusion and guards.

Disconfirmation checked three concrete risks: (1) partial support mistaken for complete consensus — all missing prerequisites remain enforced and later-owned; (2) genesis vector Ok mistaken for runtime acceptance — the companion identity-success test requires ordinary refusal, and owner tests prove unchanged state; (3) uncovered refusal errors — filesystem permission/other-entry failures are typed and tested, and the actual formatting error now propagates in a meaningful failing-writer test. No unresolved in-scope error path was found.

### Human Verification Required

None for this phase's bounded headless identity/refusal contract. Actual software tests, fixed diagnostic facts and byte-preservation checks cover the stated criteria. This report asserts neither subjective UX quality nor external/public-network behavior.

### Gaps Summary and Lifecycle

No actionable phase gap remains. All 27 truths, 29 artifacts and 22 links are verified; all declared controls are closed; the actual full default native gate passed against its recorded reviewed source. Subsequent changes are limited to the three separately reviewed closure-document files and one contract-file terminal LF. No override was necessary.

CONTEXT, RESEARCH, all thirteen PLAN/SUMMARY frontmatter blocks and this report share yolo provenance and phase_lifecycle_id 164-2026-10-14T01-59-33Z. Required generated_by/generated_at fields are present, no direct-fallback provenance was found, and this report is generated after the upstream artifacts. lifecycle_validated is true.

The parent completed requirement activation and Phase 164 lifecycle closure after the actual passed report, then obtained the clean closure documentation/metadata audit. Normal-hook commit and push remain parent-owned; their success is not claimed here. This verifier updated only VERIFICATION.md and made no source/tracker/Git changes or commit.

***

_Verified: 2026-10-14T10:38:59Z_
_Verifier: gsd-verifier_
