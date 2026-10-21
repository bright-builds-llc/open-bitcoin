---
phase: 166-canonical-durable-bootstrap-and-genesis-basic
status: passed
threats_total: 42
threats_closed: 42
threats_open: 0
threats_mitigate: 39
threats_accept: 3
threats_transfer: 0
unregistered_flags: 0
asvs_level: 1
block_on: open
generated_by: gsd-security-auditor
lifecycle_mode: yolo
phase_lifecycle_id: 166-2026-10-18T22-36-40Z
generated_at: "2026-10-21T11:01:46.414175+00:00"
---

# Phase 166 — Security

**All 42 declared threats are closed by their planned dispositions: 39 verified mitigations and three explicit accepted risks. No actionable or unresolved high-severity finding remains in this scoped audit.** This verifies declared threat controls for bounded canonical-genesis support; it does not certify the whole node.

## Review Basis

The audit covers all sixteen current PLAN threat models, fifteen accepted canonical execution summaries, the receipt-bearing Plan14 draft, the independent goal checkpoint's 54 obligations and the final source review. Every threat ID, category and disposition was compared with its owning plan; none was skipped or converted from mitigate to accept. No transfer disposition exists. Security enforcement remains enabled; ASVS 5.0 Level1 supplies scoped validation, authorization, file/data and error-handling vocabulary only.

Repo AGENTS.md, AGENTS.bright-builds.md, standards-overrides.md, standards/index.md and relevant verification/architecture/testing/Rust guidance informed the retained source audit. The exact fixture panic override remains visible below. Both active lesson files were read completely: global 6,898 bytes plus repository 1,958 bytes, combined 8,856 bytes and 2,953 conservative estimated tokens; no archives were loaded. Implementation files were read-only. This auditor ran no tests, builds, verifier, hooks or Git mutations.

The final reviewed source contains 113 paths with SHA256 `861a298d938014739c0f311a292e270bf268c10e6e635d7ad4520195ef516535`. The auditor independently rehashed every file and recomputed the aggregate using sorted path/individual-SHA rows, with no trailing newline; every raw witness matches `pure-genesis-coverage01/candidate-source-basis113.json`. Retained source findings were carried forward only after that equality check and current anchor reinspection. The scope includes 74 Rust files: 69 original raw witnesses, one fixture annotation-only delta, the trusted core constructor and its existing test, and two narrow legacy test corrections.

Current ownership DATA SHA256 is `24ea4a2a6c03a62e80056eec4e51b07e9806e9e7a57141af35fa3205f231e328`; EDGE DATA SHA256 is `4ee7948d3b50e1c03f2adaa8b614bc61a29f34cbd4959adbf1eedab95ab01f4b`. Retained independent CODE review covers narrowly authorized existing-value renewals, not fresh ownership credit. The fixed structures remain 197 SOURCE_CONTRACTS, 148 SOURCE_FILES, 26 TEST_MODULES, 88 TEST_OWNERS, 617 EDGES, 312 ordered reader inputs and thirteen immutable historical files. Original inverse destinations, historical fixtures and pins are preserved.

## Trust Boundaries and Verified Controls

| Boundary | Verification result |
| --- | --- |
| Selected identity/body → immutable genesis facts | CanonicalGenesisProof::prove compares the exact selected complete body/header and derives proof internally. Private immutable fields accept no caller work, height or acceptance flag. Selection remains identity-only. |
| Complete coins/header facts → earned core owners | CompleteCoinsSnapshot must have B=genesis, empty coins and no H. Genesis transaction connection is skipped; full output script remains available for BASIC. HeaderStore uses checked insertion, not imported numeric-score promotion. |
| Source selection → supported storage open | Unix source normalization checks existing ancestors, refuses parent traversal/symlink aliases and binds missing suffixes. Opaque stable private single-link external lease files are not replaced/deleted by supported code. Every supported backend builder requires the token; non-Unix refuses. |
| Supported handles → final lease release | Store fields drop engine owners before the Arc lease. FjallCoinsView retains its store last, after extracted database/keyspace handles. Store/adapter/view clones retain exclusion; snapshots return detached domain data. Actual child controls prove contention and final-drop reopening. |
| Existing original bytes → private scratch | Retained application lease plus actual existing Fjall lock precede bounded regular-file inventory/copy. Bounds are 4,096 entries, 512MiB payload, depth16, 4,096 path bytes and 64KiB copy chunks. Files are create-new0600, directories0700. Metadata/inode/content/mode/mtime are rechecked; atime is excluded. |
| Scratch recovery → detached parent records | Same current executable and exact reserved argv only; dispatch precedes Clap/Tokio startup. The child receives scratch identity/result inode/nonce via capped stdin, never source path or source locks. Child-only panic hook exits78 without payload; stdout/stderr are null. Actual raw Fjall recovery and teardown occur only in that child. |
| Child bytes/status → proof callback | Version/framing/count/byte/identity/nonce/duplicate/trailing checks plus delivered job and exit0 after handle teardown are mandatory. MAX_JOB=16KiB, MAX_RESULT=512MiB+4096*24+64; combined namespace/row units=4096 and aggregate logical payload=512MiB. |
| Child lifecycle → cleanup/admission | The 30s monitor begins before spawn and includes stdin delivery. Parent requires verified reaping before recovered-tree traversal/chmod/checked root cleanup. Wait/kill/reap uncertainty refuses, retains scratch, performs no chmod/removal or proof callback. Successful result does not bypass live/nonzero status controls. No parent raw fallback exists. |
| Detached records → original engine | The source is rechecked while source locks remain held. Exact-root cleanup precedes detached proof callback; incompatible state never opens original. App lease spans temporary engine-lock handoff, lowest-level original open and immediate equality reproof. Generic migration/hydration runs only after proved admission. |
| Parsed stored numbers → canonical authority | Strict bounded row parsing preserves absent versus empty. Exactly one height-zero/null-parent header/index/active position, full selected body, network/root, coverage/validation identity and B must agree with internally recomputed 32-byte work. Extra C/H/undo, descendants, legacy/unproved values, unknown authority rows or unresolved prune/index transitions refuse. |
| Fresh genesis → durable publication | All encoded authority records are validated first and inserted in one cross-keyspace SyncAll batch. Commit error/uncertainty grants no capability. BASIC is separate. Postcommit complete reproof precedes the sealed owner. This is not whole-flush or hardware power-loss atomicity. |
| Sealed store → runtime/BASIC | OpenedCanonicalGenesis has private fields, no raw-store accessor/Clone; consumption checks live instance Arc identity, DB sequence before/after complete reproof. Managed constructor rechecks again before initialization and checks complete coins. Existing append owner/generation/fence drives BASIC; fresh Disabled/absent PreserveSaved create no index authority, saved Active uses Enabled recovery. |
| Managed checkpoint → strict reopen | save_validated_chain_meta preserves only an already-decoded count map when the entire stored ancestry is identical; absence stays absent. Decode errors occur before invalidation/replacement. Public writes retain authority invalidation. All-selector Enabled/Disabled managed checkpoints reopen through independent children. |
| Runtime configuration → RPC/daemon effects | Specialized RPC factory consumes the sealed opener by value, creates exactly one runtime and derives its network/store/selected params. Both selectors, peer-filter configuration and all five activation conditions are checked before initialization. Context effect paths derive only from runtime.store().datadir(), including divergent caller-config controls. Daemon activation preflight precedes the opener. |
| RPC wallet surface → wallet/mempool effects | WalletState::Unavailable has no wallet/registry owner. All fourteen direct match branches, selection helper, wallet_info and rescan entrypoints return the fixed unavailable error. Counted rescan callbacks remain zero; no wallet/mempool recovery builder is called by the factory. Existing HTTP/direct dispatch controls preserve logical state. |
| Genesis observation → ordinary history/public activation | Ordinary constructors/guards remain refused for all four selectors, including missing Signet challenge. Managed messages, imports, stored connects/reorg and sync preflight retain history checks before effects. CLI status uses existing-only reopen; wallet/mutation/soak routes retain open_ordinary. |
| Current source/evidence → retained history/claims | Current semantic controls bind actual changed functions/predicates/guard depths and enabled tests. Digest renewal follows source review and named real mutants. Exact five-helper reader delta, exact current rows, strict historical inverse and original claim domains remain bounded; original receipts are not renewed as current acceptance. |

A containing0700 scratch root protects raw recovery outputs while the child is running; after confirmed child completion the parent checks and normalizes recovered directories/files to0700/0600 before checked removal. No guarantee of retained scratch cleanup, OS dump redaction or memory-resource containment is added for parent/system failure.

## Threat Verification

Each row corresponds to exactly one current PLAN threat. For mitigate, cited controls were found in the implementation and retained source review, then bound to unchanged final bytes and actual whole-native execution. For accept, the specific accepted-risk log entry below supplies closure. Source anchors are repository-relative; abbreviated same-module references continue the preceding full path.

| Threat ID | Category | Disposition | Status | Verified control / evidence |
| --- | --- | --- | --- | --- |
| T-166-01-01 | S/T | mitigate | CLOSED | `packages/open-bitcoin-consensus/src/chain_spec/bootstrap.rs:20; chain_spec.rs:83; bootstrap/tests.rs:11` |
| T-166-01-02 | R/I | mitigate | CLOSED | `packages/open-bitcoin-consensus/src/chain_spec/bootstrap.rs:21; scripts/phase166-genesis-vectors.ts:148` |
| T-166-02-01 | E | mitigate | CLOSED | `packages/open-bitcoin-chainstate/src/engine/canonical_genesis.rs:29; packages/open-bitcoin-network/src/header_store/canonical_genesis.rs:12` |
| T-166-02-02 | R/I | mitigate | CLOSED | `packages/open-bitcoin-chainstate/src/engine/canonical_genesis.rs:33; engine/canonical_genesis/tests.rs:1` |
| T-166-03-01 | T/E | mitigate | CLOSED | `packages/open-bitcoin-node/src/storage/fjall_store/open_lease.rs:38; opening.rs:19; fjall_store.rs:69; storage/coins_view.rs:37` |
| T-166-03-02 | R/I | mitigate | CLOSED | `packages/open-bitcoin-node/src/storage/fjall_store/open_lease.rs:162; open_lease/tests.rs:270` |
| T-166-04-01 | T/D/I | mitigate | CLOSED | `packages/open-bitcoin-node/src/storage/fjall_store/inspection.rs:66; inspection/copy.rs:17; copy.rs:196; copy.rs:246` |
| T-166-04-02 | R/I | mitigate | CLOSED | `packages/open-bitcoin-node/src/storage/fjall_store/inspection.rs:251; inspection/worker/tests.rs:301` |
| T-166-04-03 | D/I | mitigate | CLOSED | `packages/open-bitcoin-node/src/storage/fjall_store/inspection/worker.rs:44; worker.rs:76; inspection.rs:27` |
| T-166-05-01 | S/T/E | mitigate | CLOSED | `packages/open-bitcoin-node/src/storage/canonical_bootstrap/records.rs:66; records.rs:206; proof.rs:40` |
| T-166-05-02 | R/I | mitigate | CLOSED | `packages/open-bitcoin-node/src/storage/canonical_bootstrap.rs:20; canonical_bootstrap/qualification_tests.rs:105` |
| T-166-06-01 | T/E | mitigate | CLOSED | `packages/open-bitcoin-node/src/storage/fjall_store/canonical_genesis.rs:87; canonical_genesis.rs:132; canonical_genesis.rs:194` |
| T-166-06-02 | R/I | mitigate | CLOSED | `packages/open-bitcoin-node/src/storage/canonical_bootstrap.rs:34; fjall_store/canonical_genesis/tests.rs:370` |
| T-166-07-01 | E/T | mitigate | CLOSED | `packages/open-bitcoin-node/src/storage/fjall_store/canonical_genesis.rs:44; chainstate/fjall_store/canonical_genesis.rs:19; network/canonical_admission.rs:105; sync.rs:257` |
| T-166-07-02 | R/I | mitigate | CLOSED | `packages/open-bitcoin-node/src/sync/canonical_genesis.rs:29; network/canonical_admission.rs:34` |
| T-166-08-01 | S/T/E | mitigate | CLOSED | `packages/open-bitcoin-node/src/storage/canonical_bootstrap/proof.rs:94; sync/canonical_genesis.rs:42; sync/canonical_genesis/basic.rs:14; sync/open_runtime.rs:154` |
| T-166-08-02 | R/I | mitigate | CLOSED | `packages/open-bitcoin-node/src/sync/canonical_genesis/basic_tests.rs:392; storage/canonical_bootstrap.rs:34` |
| T-166-09-01 | E/I | mitigate | CLOSED | `packages/open-bitcoin-rpc/src/bin/open_bitcoind/canonical_startup.rs:36; packages/open-bitcoin-cli/src/operator/runtime/support/canonical_genesis.rs:6` |
| T-166-09-02 | R/I | mitigate | CLOSED | `packages/open-bitcoin-rpc/src/bin/open_bitcoind/canonical_startup.rs:21; packages/open-bitcoin-cli/src/operator/runtime/support/canonical_genesis.rs:17` |
| T-166-10-01 | T/R | mitigate | CLOSED | `packages/open-bitcoin-node/src/storage/canonical_bootstrap/qualification_tests.rs:105; qualification_tests.rs:155; qualification_tests.rs:369; qualification_tests.rs:415` |
| T-166-10-02 | R/I | mitigate | CLOSED | `packages/open-bitcoin-node/src/storage/canonical_bootstrap/qualification_tests.rs:134; storage/fjall_store/tests/checkpoint_metadata.rs:1` |
| T-166-11-01 | T/E/R | mitigate | CLOSED | `scripts/check-phase166-canonical-bootstrap.ts:40; contracts.ts:22; contracts.ts:221; scripts/check-phase164-canonical-identity.ts:146` |
| T-166-11-02 | R/I | mitigate | CLOSED | `scripts/check-phase166-canonical-bootstrap.ts:14; check-phase166-canonical-bootstrap.ts:191; check-phase166-canonical-bootstrap.test.ts:444` |
| T-166-12-01 | T/R/E | mitigate | CLOSED | `scripts/check-phase164-canonical-identity/claims.ts:45; scripts/check-phase165-shared-exact-work.ts:23; scripts/check-phase164-canonical-identity/historical-tooling.ts:47; scripts/verify.sh:369` |
| T-166-12-02 | R/I | mitigate | CLOSED | `packages/open-bitcoin-rpc/tests/phase165_work_startup.rs:117; scripts/check-phase166-canonical-bootstrap.test.ts:90; check-phase166-canonical-bootstrap.test.ts:162` |
| T-166-13-01 | R/I/T | mitigate | CLOSED | `docs/parity/v2-6-canonical-durable-bootstrap.md:1; README.md:1; .codex/tasks/todo.md:1; AGENTS.md:1` |
| T-166-13-02 | R/I | mitigate | CLOSED | `docs/parity/v2-6-canonical-durable-bootstrap.md:1; .planning/phases/166-canonical-durable-bootstrap-and-genesis-basic/166-13-SUMMARY.md:1` |
| T-166-14-01 | R/E/T | mitigate | CLOSED | `scripts/verify.sh`; actual manual05 exit0 and retained fresh coverage; final `166-REVIEW.md`; goal54 source checkpoint and receipt-bearing Plan14 draft; controlled root wrapper handoff described below. |
| T-166-14-02 | R/I | mitigate | CLOSED | All four failed native attempts and component failures retained below; manual05 result and source-bound receipts separately identified; fixed diagnostics and exact-source review. |
| T-166-14-03 | D | accept | CLOSED | `packages/open-bitcoin-rpc/src/context/canonical_chain.rs:185`; exact `scripts/panic-sites.allowlist` entry; `standards-overrides.md` Rust panic avoidance row; Accepted Risks Log A3 |
| T-166-15-01 | D | mitigate | CLOSED | `packages/open-bitcoin-node/src/storage/fjall_store/inspection/worker.rs:44; worker.rs:148; worker.rs:202; worker.rs:264` |
| T-166-15-02 | T/E | mitigate | CLOSED | `packages/open-bitcoin-node/src/storage/fjall_store/inspection/protocol.rs:44; protocol.rs:105; protocol.rs:130; protocol.rs:216; inspection/worker.rs:76; worker.rs:245` |
| T-166-15-03 | I | mitigate | CLOSED | `packages/open-bitcoin-node/src/storage/fjall_store/inspection.rs:163; inspection.rs:206; inspection/copy.rs:196; copy.rs:246; inspection/worker.rs:50; worker.rs:153` |
| T-166-15-04 | T | mitigate | CLOSED | `packages/open-bitcoin-node/src/storage/fjall_store/inspection.rs:66; inspection.rs:137; inspection/worker.rs:116; fjall_store/canonical_genesis.rs:132` |
| T-166-15-05 | R | mitigate | CLOSED | `packages/open-bitcoin-node/src/storage/fjall_store/inspection/tests.rs; inspection/worker/tests.rs:112; scripts/check-phase166-canonical-bootstrap.test.ts:334` |
| T-166-15-06 | D/I | accept | CLOSED | `Accepted Risks Log A1 below; 166-15-PLAN.md T-166-15-06; docs/parity/v2-6-canonical-durable-bootstrap.md`; A1. |
| T-166-16-01 | S/E | mitigate | CLOSED | `packages/open-bitcoin-rpc/src/context/canonical_genesis.rs:17; packages/open-bitcoin-node/src/storage/fjall_store/canonical_genesis.rs:20` |
| T-166-16-02 | T/E | mitigate | CLOSED | `packages/open-bitcoin-rpc/src/context/canonical_genesis.rs:21; canonical_genesis.rs:39; canonical_genesis.rs:57; context/canonical_chain.rs:91` |
| T-166-16-03 | E/T | mitigate | CLOSED | `packages/open-bitcoin-rpc/src/context/wallet_state.rs:24; wallet_state.rs:36; wallet_state.rs:315; context/rescan.rs:55; rescan.rs:144; context/canonical_genesis.rs:41` |
| T-166-16-04 | I/R | mitigate | CLOSED | `packages/open-bitcoin-rpc/src/context/canonical_chain.rs:48; canonical_genesis/tests.rs:324; tests.rs:450; tests/refusals.rs:31; tests/refusals.rs:172` |
| T-166-16-05 | D/T | mitigate | CLOSED | `packages/open-bitcoin-rpc/src/lib.rs:43; context/canonical_genesis/tests.rs:344; packages/open-bitcoin-node/src/storage/fjall_store/inspection.rs:66` |
| T-166-16-06 | T/D | accept | CLOSED | `Accepted Risks Log A2 below; 166-16-PLAN.md T-166-16-06; docs/parity/v2-6-canonical-durable-bootstrap.md`; A2. |

## Accepted Risks Log

A1/A2 retain the two original checked-plan accepted limits. A3 is the subsequently explicit root-approved T-166-14-03 disposition and narrow standards override. This auditor invents no acceptance, and no threat originally marked mitigate is converted to accept.

| Risk | Threat reference | Existing acceptance and rationale | Scope retained |
| --- | --- | --- | --- |
| A1 | T-166-15-06 | Checked166-15 yolo plan, category D/I: cleanup requires a live parent; OS process/resource/crash-report handling is outside the adapter guarantee. | Parent/system crash, forced termination/OOM and OS crash dumps may leave private scratch or expose OS-managed data. No crash-proof cleanup or payload-free OS-dump claim. |
| A2 | T-166-16-06 | Checked166-16 yolo plan, category T/D: inherited supported-app/advisory-lock and live-parent limits. | Arbitrary direct-engine/filesystem writers and hardware power-loss are outside the contract. Supported cooperating Unix exclusion and actual macOS test evidence remain distinct. |
| A3 | T-166-14-03 | Root explicitly accepted in amended166-14 yolo plan on2026-10-21; category D, existing explicit fixture constructor. | Public `from_fixture_runtime_config` may panic on invalid configuration or storage/network/recovery error. It remains fixture/tooling-only, with a fallible `from_fixture_runtime_config_with_store(config, None)` alternative and one exact path/literal allowance. Ordinary and sealed startup never call it. This is not infallibility or permission for new panic sites. |

## Summary Threat Flags

All fifteen accepted canonical summaries and the receipt-bearing Plan14 draft were inspected. No canonical summary has a `## Threat Flags` register or an unmapped threat_flag entry. Narrative process/storage/RPC/evidence surfaces map to the existing declared threats. The fixture exception maps to T-166-14-03; the process crash limits map to T-166-15-06 and T-166-16-06. Unregistered flags: zero. This is finite verification of declared controls, not a blind vulnerability scan.

## Actual Final Verification Evidence

Paths in this section are relative to `.local/open-bitcoin-dev/phase166/plan14/` unless stated otherwise. Overlapping component and focused results are never added into a unique coverage total.

| Evidence | Actual result and boundary |
| --- | --- |
| `manual05/native-outcome.json` | Exactly `bash scripts/verify.sh`, started 2026-10-21T08:54:10.022Z, finished 2026-10-21T10:55:33.413Z, exit0/no signal, 7283.397265791 seconds. Pinned Bun1.3.9, physical TMPDIR, isolated phase Cargo target and unique report directories. No fast/skip/deadline modifications. |
| `manual05/native.log` | Nonempty 2,279,815-byte raw log; SHA256 `76fabae24b50d48cb7a32cac53b255e3447b87239862f29cc491ea2283ec6db7`. All 78 native Bun suite footers have zero failures. Default verification includes formatting, strict workspace/all-target/all-feature Clippy, all-target build, complete workspace tests, CLI tests, benchmark list/smoke/report, Bazel build/provenance and fresh pure-core coverage. |
| `manual05/terminal-audit.json` | Actual passed finite audit: all 21 checks, 184 successful serial stage receipts, unchanged final113 raw witnesses and native bindings; every Rust footer attributed to its proper group. Workspace: 29 harnesses plus thirteen doctest packages across 43 footers, 4957 passed/zero failed/six ignored/zero filtered. Extra CLI command: seven passed. |
| `manual05/coverage-observation.json` and `coverage-report.txt` | Actual nativeExit0 and captured=true. Read-only observer retained the genuine current-run report through its cleanup. Report 2,438,922 bytes; SHA256 `589fbac073c775c7cbd285c964c74a6c27e022006fda9b675408ef0fdbf9db41`, independently rehashed, no `Uncovered Lines:` heading. Fresh clean precedes exact eight-package coverage; sixteen footers contain 1832 passed/zero failed/zero ignored/zero filtered. Requested/selected package argv and independently observed report paths remain separate observations. |
| `manual05/root-terminal-audit.json` | Root independently confirms unchanged113/861 source, native0, report/log hashes and recorded runner/native/coverage PIDs absent. Process absence is limited to recorded PIDs, not an assertion that persistent Bazel services or every historical descendant exited. |
| Final canonical `166-REVIEW.md` | Source-clean113 with zero actionable critical/warning/info findings and actual manual05 result, refreshed 2026-10-21T10:57:05Z. SHA256 `04a0281ccacb98fb32fb9544267b9d7c1524970232f9b6cf22a092874c7a69a9`. Accepted fixture panic risk remains explicit. |
| Goal/source checkpoint and Plan14 draft | The existing goal checkpoint traces fifty PLAN truths plus four mandatory roadmap criteria: 54 obligations, sixteen plans, 36 tasks, 22 artifact declarations/21 unique paths and 23 key links. The final receipt-bearing Plan14 draft binds the actual completed native run. Its observed SHA256 is `752703ad187963b83f927730061b9d8c90565f63c440ecf030e2976e552e5c36` and Task1 is complete, while Tasks2/3 remain pending their canonical handoff. It supports sequential security closure; it does not assert that the future canonical goal report already exists. |

The final pure-core simplification does not weaken an error boundary: CanonicalGenesisProof's private immutable fields and closed four-selector constructors establish the null parent before the core uses infallible ChainPosition::new with height0. Complete snapshot propagation, all three B/noC/noH checks, selected exact work/time, Live binding and the public Result remain. The existing all-selector test now explicitly checks null parent. No coverage exclusion, unsafe construction or forged proof was added. Earlier fresh coverage on that same final source independently passed, but manual05 supplies the required complete default run.

## Retained Failures and Evidence Limits

The four earlier complete default attempts remain failed. No later component or whole-command success relabels them.

| Attempt | Retained actual failure |
| --- | --- |
| manual01 | Exit1: Phase163 native158 passed/two failed of160, aggregate historical-layout child deadlines. Later stages were not reached. Subsequent per-layout registration preserves all ten layouts, two contexts, six boundaries and original deadlines. |
| manual02 | Exit1: Phase163 native178 passed, Phase1641183 passed/seven copied inverse syntax failures. Literal callback replacement corrected replacement-token expansion; original failures remain. |
| manual03 | Exit1: Phase1641190, Phase1652020 and Phase166490 passed, then Phase12339 passed/63 stale configured-startup owner failures. Later legacy/Rust stages were not reached. Independently reviewed compatibility repairs retain ordinary and sealed-genesis authority checks. |
| manual04 | Exit1/no signal: Phase163 native176 passed/two failed of178. Next-active audit child's 30000ms watchdog and final parent's 55000ms process timeout are distinct observed failures. Two subsequent exact-budget diagnostic passes on identical source did not reproduce the failures; the original cause remains unestablished. No timeout/source/cache change was made before manual05. |

Earlier Plan04 raw malformed-recovery SIGABRT, APFS non-UTF8 setup failure, initial daemon/UAT harness failures, aggregate default-timeout results, copied-control harness preconditions, suffix guard/test/coverage failures, remaining-target executable mismatch and the mistaken doctest-footer postprocessor remain in the retained logs and summaries. Focused positives are not promoted into whole native or cross-platform proof. The old missing-lines report remains failed even though its coverage process exited0. The corrected final default run is separately identified above.

Actual four-selector durable mutation coverage includes seventy named preserved refusals, all 32 coordinated work-byte mutations and independent checkpoint/reopen children. Existing BASIC append owner, generation/fencing and zero predecessor remain genuine. Cargo/Bazel no-job worker exits establish packaging/dispatch only; actual malformed/open/reopen child cases supply recovery evidence. Physical equality claims are limited to their test boundaries; post-capability factory refusals assert logical preservation/zero application initialization rather than inventing byte equality across engine teardown.

Evidence is bounded to source-derived genesis/BASIC, exact arithmetic, software storage/fault/process controls and local macOS product behavior. Ordinary descendants remain Phase167 work and actual pinned-Knots daemon differential remains Phase168 work. There is no wallet authority, public activation, unattended full-sync, whole-flush/hardware power-loss, hostile-writer, crash-proof cleanup or production/funds-readiness claim. The accepted fixture API may panic on its documented error conditions.

## T-166-14-01 Closure and Wrapper Handoff

The prior administrative dependency is now closed by actual complete default native0 on the unchanged reviewed source, clean final independent CODE review, this all-threat security verification, the goal checkpoint's direct54-obligation evidence and the receipt-bearing final Plan14 draft. No risk acceptance substitutes for the execution gate. The canonical goal verifier must independently read this report and the final summary/metadata before phase lifecycle closure; its future report is not assumed to exist or pass here.

Root alone owns shared metadata, formal lifecycle/summary validation and the concrete saving candidate. The invoked strict wrapper requires the normal pre-commit verifier to pass before commit/push, followed by actual remote verification, and work stops before Phase167. This report grants no hook bypass and claims no future hook, commit or push result. Security closure records established controls and observed evidence; saving remains an enforced subsequent wrapper operation.

## Security Audit Trail

| Audit | Threats | Closed | Open | Result |
| --- | --- | --- | --- | --- |
| Retained source preparation through final113/861 | 42 | 41 | 1 | T-166-14-01 administrative full-execution dependency; three accepted risks already explicit |
| 2026-10-21 final actual-native closure | 42 | 42 | 0 | Thirty-nine mitigations found; three planned acceptances logged; zero unregistered flags/actionable high findings |

## Sign-Off

- [x] All sixteen threat models classified exactly once: 42 threats.
- [x] All 39 mitigate dispositions have implementation/evidence support.
- [x] All three accept dispositions retain their explicit accepted-risk entries.
- [x] No transfer or unregistered flag is omitted.
- [x] Actual full default native0 and final unchanged source are authenticated.
- [x] Implementation files remain unchanged by this auditor.
- [x] `threats_open: 0` and `status: passed` reflect this scoped audit.

**Security approval:** verified 2026-10-21. Formal goal/lifecycle and normal-hook saving remain root-owned subsequent gates.
