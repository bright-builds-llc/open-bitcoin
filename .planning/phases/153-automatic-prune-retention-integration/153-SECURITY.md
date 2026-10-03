---
phase: 153-automatic-prune-retention-integration
slug: automatic-prune-retention-integration
status: verified
threats_total: "14"
threats_closed: "14"
threats_open: "0"
asvs_level: "1"
block_on: high
security_enforcement: "true"
native_verification_status: passed
requirement_completion: complete
created: "2026-10-03"
audited_at: "2026-10-03T07:31:47Z"
generated_by: gsd-security-auditor
lifecycle_mode: yolo
phase_lifecycle_id: 153-2026-10-03T04-01-54
generated_at: "2026-10-03T07:31:47Z"
---

# Phase 153 — Security

All 14 declared threats have implemented mitigation controls. No declared mitigation gap remains. The full default native verifier has now passed with exit 0, including the genuine retention regression, coverage and Bazel. `status: verified` records this scoped source, control-flow, test-assertion and documentation audit; root lifecycle validation, integration re-audit and PRUN-01/PRUN-02 completion remain pending. T-153-14's native execution evidence is refreshed below; it does not authorize phase finalization.

## Scope and Method

Input state B: the phase had PLAN threat models and executed summaries but no SECURITY report. All four PLAN registers were read and merged by unique threat ID: T-153-01 through T-153-14, all `mitigate`, zero `accept`, zero `transfer`. Each threat was classified by its declared STRIDE category and disposition before verification. Mitigations were searched in their declared files, then checked against actual control flow and behavioral assertions. The auditor read CONTEXT, RESEARCH, summaries 01–03, implementation/test seams and the updated Stage 1 documentation. Summary 04 and full native results were pending at the initial audit; the timestamped refresh below records the subsequent confirmed native pass.

Configuration contains no explicit false security-enforcement setting, so enforcement is enabled. The delegated policy is ASVS level 1 and `block_on: high`. ASVS is a targeted mapping of existing validation, authorization and evidence boundaries; this report makes no certification claim. No broader vulnerability scan, production edit, Cargo/Bazel run, staging, commit or new risk acceptance was performed. Only this report was written. D-10 overrides the workflow's ordinary commit step; root owns finalization after clean gates.

## Trust Boundaries

| Boundary | Required control | Data crossing |
| --- | --- | --- |
| Live Fjall values → destructive policy facts | One guarded snapshot, exact lengths, checked arithmetic, explicit errors | Block/encoded-undo sizes and active height/hash candidates |
| Store-clone mutation → reusable measurement | Shared mutex spans invalidation, mutation and persistence; captured revision and poison refusal | Completed measurement identities |
| Authenticated lock publication → prune owner | Shared authority covers durable lock RMW and plan/application | Named inclusive locks and buffered ranges |
| Configured offline datadir → ready owner | Existing recovery before readiness, configured inputs before worker, opt-in networking | Mode, network parameters, durable coins/metadata |
| Committed unlink → cache, undo, wallet, serving, reopen | Earned receipts, current-lock recovery, fresh eligibility, no leftover resurrection | Deleted hashes, counters and retained authority |
| Observed evidence → parity and finalization | Truthful fixture limits, sanitized support, pending native/lifecycle gate | Measured logical bytes and scoped claims |

## Threat Register

Paths are repository-relative. Line references identify inspected controls or behavioral assertion entry points; test execution results are separately attributed to executor summaries below.

| Threat ID | Category | Component | Disposition | Mitigation and evidence | Status |
| --- | --- | --- | --- | --- | --- |
| T-153-01 | T — Tampering | Payload accounting | mitigate | Shared guard before one snapshot; complete block/undo prefixes, `Guard::size`, same-snapshot active `size_of`, present-mate handling and checked sums: `packages/open-bitcoin-node/src/storage/fjall_store/payload_usage.rs:127`, `:131`, `:146`, `:149`, `:215`. Exact serialized, nonactive/excluded, half-pair/zero-length, overflow and explicit-error tests: `packages/open-bitcoin-node/src/storage/fjall_store/payload_usage/tests.rs:72`, `:109`, `:150`, `:368`, `:378`. Unsupported sink accounting refuses at `packages/open-bitcoin-node/src/chainstate/flush_lifecycle.rs:95`. | CLOSED |
| T-153-02 | D — Denial of service | Payload traversal | mitigate | Accounting traverses size metadata and never decodes historical bodies: `packages/open-bitcoin-node/src/storage/fjall_store/payload_usage.rs:133`. Unchanged non-Always input skips the scan at `packages/open-bitcoin-node/src/network/runtime_authority/automatic_prune.rs:145`; behavioral scan-count assertion: `packages/open-bitcoin-node/src/network/runtime_authority/automatic_prune/tests.rs:99`. Scan cost remains proportional to retained keys and active positions; no production lock-hold SLA is claimed. | CLOSED |
| T-153-03 | T — Tampering | Revision publication | mitigate | Clone shares `Arc<Mutex<PayloadUsageState>>`: `packages/open-bitcoin-node/src/storage/fjall_store.rs:73`; guard spans complete raw payload writes and persistence, paired attempts, fact/revision capture and fallible revision reads: `packages/open-bitcoin-node/src/storage/fjall_store/payload_usage.rs:89`, `:131`, `:164`, `:172`, `:177`, `:184`; `packages/open-bitcoin-node/src/storage/fjall_store/prune.rs:98`. Checked identity/generation exhaustion never authorizes reuse: `payload_usage.rs:60`, `:82`. Paused equal-size/error writers, overlaps, reopen identities, overflow and real mutex poison: `packages/open-bitcoin-node/src/storage/fjall_store/payload_usage/tests.rs:324`, `:340`, `:387`, `:492`, `:564`; owner-level real-clone regressions: `packages/open-bitcoin-node/src/network/runtime_authority/automatic_prune/tests/writers.rs:53`, `:129`, `:200`. | CLOSED |
| T-153-04 | T/E — Tampering / elevation of privilege | Lock publication and application | mitigate | Authority mutation encloses current-lock load, same-name RMW and SyncAll forwarding; manual application reloads durable locks and adds caller restrictions: `packages/open-bitcoin-node/src/network/runtime_authority/prune_flush.rs:71`, `:86`, `:104`; `packages/open-bitcoin-node/src/storage/fjall_store/prune/records.rs:47`, `:109`. RPC routes use the owner after range/name/buffer validation: `packages/open-bitcoin-rpc/src/context/prune.rs:26`, `:42`, `:67`, `:74`; existing authentication precedes parsing: `packages/open-bitcoin-rpc/src/http.rs:136`. Stale-slice protection, replacement/missing clear, owner blocking and prune-first schedules: `packages/open-bitcoin-node/src/network/runtime_authority/automatic_prune/tests.rs:347`, `:374`, `:402`, `:515`. Metrics-only writer refusal and authority-backed status: `packages/open-bitcoin-rpc/src/dispatch/prune/tests.rs:81`, `:101`. | CLOSED |
| T-153-05 | D — Denial of service | Ordinary measurement gate | mitigate | None/nonautomatic/short-chain/overflow gates precede exact accounting; captured reusable key includes actual tip/hash, mode and locks; changed Periodic inputs coalesce to named 60 seconds, Always bypasses; deferred cycles return an empty plan: `packages/open-bitcoin-node/src/network/runtime_authority/automatic_prune.rs:23`, `:26`, `:121`, `:145`, `:152`, `:161`. Revision change refuses candidate application and errors clear reuse: `:79`, `:94`, `:164`. Deterministic coalescing/no-cached-delete, Always, changed-input and invalid revision tests: `packages/open-bitcoin-node/src/network/runtime_authority/automatic_prune/tests.rs:112`, `:132`, `:149`, `:212`, `:224`. | CLOSED |
| T-153-06 | T — Tampering | Nonempty automatic checkpoint | mitigate | Nonempty plan selects existing `FlushMode::Always`, then the same receipt wrapper: `packages/open-bitcoin-node/src/network/runtime_authority/automatic_prune.rs:84`. Current coins best-block is read fallibly and staged before real write, preserving an overlay tip: `packages/open-bitcoin-node/src/chainstate/flush_lifecycle.rs:539`. Repeated idle/staged-tip writes and parent-read refusal: `packages/open-bitcoin-node/src/chainstate/flush_lifecycle/tests/initialize.rs:15`, `:75`. Pure planner/range rules and application rechecks remain at `packages/open-bitcoin-chainstate/src/prune/plan.rs:39`, `packages/open-bitcoin-chainstate/src/prune/range.rs:9`, `packages/open-bitcoin-node/src/chainstate/flush_lifecycle/prune_apply.rs:43`. Genuine future-deadline checkpoint and protected survivors: `packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/automatic_prune.rs:137`, `:196`, `:205`, `:216`, `:248`. | CLOSED |
| T-153-07 | I — Information disclosure | Deletion error receipts | mitigate | Only `DeletedLiveMate` earns the immediate callback; manager forgets those undo records before returning either outcome; wrapper evicts the same success/error receipt hashes: `packages/open-bitcoin-node/src/chainstate/flush_lifecycle/prune_apply.rs:79`, `:87`; `packages/open-bitcoin-node/src/chainstate.rs:424`, `:439`; `packages/open-bitcoin-node/src/network/runtime_authority/prune_flush.rs:31`. Small later-error cleanup test: `packages/open-bitcoin-node/src/network/runtime_authority/automatic_prune/tests.rs:248`; actual-storage delegated error/delete/cache/undo assertions: `packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/automatic_prune.rs:295`, `:303`, `:459`. | CLOSED |
| T-153-08 | T — Tampering | Offline startup | mitigate | Explicit mode plus datadir selects durable store: `packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs:360`; existing runtime initializes/recoveries before constructing owner: `packages/open-bitcoin-node/src/sync/open_runtime.rs:30`, `:38`; network threshold/mode installed before serve starts its worker: `packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs:301`, `:320`, `:325`, `:145`. Actual explicit/disabled/no-datadir and disabled listener/sync controls: `packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/automatic_prune.rs:20`, `:52`, `:66`, `:106`. | CLOSED |
| T-153-09 | I/T — Information disclosure / tampering | Cache, undo and wallet after unlink | mitigate | Genuine ordinary deletion proves both mates/cache/undo absent, retained controls survive, both wallet adapters preserve saved wallets and record Failed creating-payload refusal: `packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/automatic_prune.rs:205`, `:232`, `:459`, `:482`. Conflicting leftover snapshot then production reopen does not replace durable truth or resurrect removed values: `:248`, `:254`, `:279`, `:330`. Actual Pruned/Unknown and explicitly injected Unavailable labels are separate at `:347`. Receipt implementation is cited under T-153-07. Existing direct-library wallet probe/save atomicity remains outside this phase's declared mitigation; no stronger claim or new acceptance is introduced. | CLOSED |
| T-153-10 | D — Denial of service | Legal-target fixture | mitigate | One default-suite large test, legal 550 MiB, reusable approximately 2.5 MiB body, modest undo, few active cached bodies, production owner opened before bulk seeding; RAII temp directory cleanup and no sockets: `packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/automatic_prune.rs:122`, `:133`, `:147`, `:372`; `packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/automatic_prune/fixtures.rs:21`, `:24`, `:38`, `:154`, `:202`, `:213`. Target crossing is exact encoded-value equality, not fake accounting. The fixture is codec-valid sparse integration; bounded construction memory is not total Fjall resident memory or consensus-chain validation. | CLOSED |
| T-153-11 | T — Tampering | Retry and reopen | mitigate | Pre-delete accounting refusal leaves deletes empty: `packages/open-bitcoin-node/src/network/runtime_authority/automatic_prune/tests.rs:237`. Paired SyncAll batch and finish-or-refuse validate active hash, keep window, current lock and best-block before recovery: `packages/open-bitcoin-node/src/storage/fjall_store/prune.rs:117`, `:147`, `:169`; actual current-lock readiness refusal/payload preservation: `packages/open-bitcoin-node/src/network/runtime_authority/automatic_prune/tests.rs:434`. Actual replenish/delete, later metadata injection, receipt cleanup, no duplicate counts, Always retry and final production reopen: `packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/automatic_prune.rs:271`. Fault delegates actual accounting/locks/unlink/counters and injects only metadata refusal: `packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/automatic_prune/fixtures.rs:239`, `:329`, `:337`, `:352`. | CLOSED |
| T-153-12 | R — Repudiation | Parity and current documentation | mitigate | Updated actual-byte/production-path evidence, pinned Knots roots, logical/physical distinction, soft target and accurate fault limits: `docs/parity/catalog/chainstate.md:43`, `:88`, `:98`, `:121`; unique historical surface preserves canonical Pending and linked source/summaries: `docs/parity/index.json:3948`, `:3967`, `:3983`; `docs/parity/checklist.md:92`; `README.md:28`; `docs/operator/runtime-guide.md:957`. Canonical requirements still Pending: `.planning/REQUIREMENTS.md:98`. No nonexistent completed Phase 153 verification link is used as proof. | CLOSED |
| T-153-13 | I — Information disclosure | Operator and support evidence | mitigate | Lock RPC failures discard backend text: `packages/open-bitcoin-rpc/src/context/prune.rs:28`, `:69`, `:77`; fixed status-unavailable labels: `packages/open-bitcoin-rpc/src/dispatch/prune/status.rs:14`, `:74`, `:87`; worker error maps to bounded Authority category: `packages/open-bitcoin-rpc/src/bin/open_bitcoind/coins_flush.rs:25`, `:38`. Earned counts are derived from actual nonempty live deletes, not attempts: `packages/open-bitcoin-node/src/storage/fjall_store/prune/records.rs:81`; genuine configured target/status and repeat counts: `packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/automatic_prune.rs:227`, `:234`, `:440`. Guidance uses Cargo/Bazel local forms and excludes raw backend paths/keys/payloads/credentials: `docs/operator/runtime-guide.md:987`. | CLOSED |
| T-153-14 | T — Tampering | Verification and finalization | mitigate | Default verifier starts full, runs workspace all-feature tests including the nonignored genuine binary test, all-target build, strict lint, pure-core coverage and Bazel; precommit calls the default contract: `scripts/verify.sh:10`, `:347`, `:351`, `:354`, `:360`, `:374`; `.githooks/pre-commit:10`. D-10 forbids commits/push before clean verification; Plan 04 Task 2 and root own lifecycle report, freshness and integration re-audit. Documentation preserves those pending gates: `docs/parity/catalog/chainstate.md:50`, `:141`, `README.md:28`. **Control present; full native execution passed** in root session 43990, as documented in the native refresh below. Formal lifecycle, requirement completion and finalization remain pending. | CLOSED |

## Accepted Risks Log

No accepted risks. All Phase 153 dispositions are `mitigate`; no transfer documentation is required. Existing phase boundaries and fixture limitations below are evidence limits, not newly accepted unmitigated threats.

## Summary Threat Flags

Summaries 01–03 have no `## Threat Flags` entries. Their `## Known Stubs and Threat Scan` sections report no additional security surface outside declared boundaries. Their guarded-writer injection, small synthetic counting sinks, sparse fixture, later metadata delegation and public return-type re-export are described explicitly and map to T-153-01/03/05/06/10/11/12. No unregistered flags were found in the available summaries. Summary 04 is pending; root must incorporate any later flags before final closeout.

## Evidence and Limits

The auditor inspected source and executable assertions; the following execution outcomes are reported in summaries 01–03, not independently rerun here:

| Evidence source | Reported result |
| --- | --- |
| Plan 01 accounting/storage | Final 16 accounting tests; 96 storage regressions; scoped strict Clippy/format/check pass |
| Plan 02 ordinary owner/RPC | Final 23 owner tests; 33 RPC prune tests; affected strict Clippy pass |
| Plan 03 genuine fixture | 578,359,864 actual initial logical bytes above legal 576,716,800-byte target; three initial deletes totaling 447 bytes; protected 510/714/1001 survive; first production checkpoint/reopen succeeds; final scenario 17.891221375s |
| Plan 03 related regressions | 25 automatic-prune, 42 lifecycle/recovery, four daemon flush, 33 RPC prune, 15 node wallet and 19 RPC wallet regressions pass; affected strict Clippy pass |
| Auditor checks | Declared mitigation searches, control-flow/behavior assertion review, updated documentation/requirement readback and `git diff --check` pass |
| Default native gate | Passed, exit 0 in root session 43990, latest attempt 3; full console `/tmp/open-bitcoin-phase153-verify.log` and successful timing record identified in the native refresh below |
| Lifecycle and requirement gates | Pending; no formal lifecycle/integration completion, requirement activation or Git finalization claimed |

All retained logical values, including protected/nonactive bytes, count toward usage. The bulk pairs are nonactive and only six small pairs form sparse active history; eligible deletion therefore cannot reach the target. This demonstrates safe soft-target behavior, not a physical disk cap, immediate reclamation, continuous consensus-chain acceptance or public-mainnet/funds readiness.

The first ordinary stage uses the production recovered durable owner and real coins checkpoint. The later `MetadataFaultStore` stage delegates actual Fjall sizes, revisions, locks, payload writes, paired deletes and earned counters, but constructs a fixture owner with `MemoryCoinsView`. Only `persist_chain_meta` is refused. Its actual unlink, cache/undo cleanup, retry and final production reopen do not constitute an actual hardware/disk fault or a second production durable-coins checkpoint proof. Writer persistence errors likewise use controlled injections; unwind tests exercise actual mutex poisoning.

## Guidance and Audit Trail

Material guidance: repo `AGENTS.md`, `AGENTS.bright-builds.md`, placeholder-only overrides, standards index and architecture/code-shape/testing/verification/local-guidance/Rust pages; current GSD secure-phase workflow and SECURITY template. Both canonical active lesson files were read completely: 5,230 global bytes plus 1,958 repository bytes, 7,188 total and 2,397 conservative estimated tokens. No archives were loaded and no lesson files were changed under report-only ownership. Existing repository lesson-audit baseline was present; root owns tracker maintenance.

| Audit Date | Total | Mitigated Closed | Accepted Closed | Open | Run By |
| --- | --- | --- | --- | --- | --- |
| 2026-10-03T06:18:47Z | 14 | 14 | 0 | 0 | gsd-security-auditor |
| 2026-10-03T06:40:49Z | 14 | 14 | 0 | 0 | gsd-security-auditor; targeted T-153-14 checker refresh |
| 2026-10-03T07:31:47Z | 14 | 14 | 0 | 0 | gsd-security-auditor; confirmed default native evidence refresh |

## Sign-Off

- [x] All four registers extracted; every unique threat classified and verified by disposition.
- [x] Available summary flags incorporated; no unregistered flag or new risk acceptance.
- [x] All 14 declared controls present; `threats_open: 0`; scoped `status: verified`.
- [x] Actual fixture and fault limits preserved; updated docs retain canonical Pending.
- [x] Implementation untouched; only this report written; no staging, build or commit.
- [x] Root full default native verification passed, including genuine retention, coverage and Bazel; actual logs and timing record inspected.
- [ ] Summary 04 flag refresh, lifecycle validation, integration re-audit, requirement completion and final freshness remain pending.

**Approval:** Scoped mitigation verification completed 2026-10-03. Full phase completion and finalization remain root-owned and pending.

## Security Audit T-153-14 Checker Refresh | 2026-10-03T06:40:49Z

The first native attempt stopped at the Phase 127 source-boundary checker because its helper scan excluded only the `tests.rs` basename and consequently treated the new nested test fixture's authority constructor as production. Root reported a meaningful focused RED of 16 passing/one failing test before the correction. This refresh reviews only that verification-control change and its two assigned checker files; no new threat scan or implementation edit was performed.

The scanner now excludes the existing `tests.rs` basename plus exact `tests` directory segments relative to the daemon helper root, using native `path.sep`: `scripts/check-phase127-authoritative-network-state-unification.ts:462`, `:475`. It still recursively collects other Rust helper files and feeds them to the unchanged production-constructor predicates: `:123`, `:145`, `:148`. The real Phase 153 fixture is beneath the daemon's `#[cfg(test)]` module (`packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs:579`), so this repairs the test-source classification rather than granting a production constructor exception. The negative fixture at `tests_support/fixtures.rs` remains scanned; the exclusion does not match arbitrary names containing `tests` or the basename `fixtures.rs`.

Behavioral regressions explicitly allow the exact nested test path and reject the production-like neighboring path with the same constructor text: `scripts/check-phase127-authoritative-network-state-unification.test.ts:187`, `:207`. Existing duplicate production/main/helper, aliased/dead-anchor, durable-serving and status-projection refusals remain. The auditor independently ran the focused Bun suite: **17 passed, zero failed, 17 assertions**, exit 0; the actual repository checker also validated with exit 0. `git diff --check` passed. No Cargo/Bazel command, staging or commit was run by the auditor.

T-153-14 remains **CLOSED** for the implemented verification control, and all 14 declared threats remain closed with `threats_open: 0`. Native verification remains **pending**: the earlier stopped run is not a pass, and the complete default verifier must rerun after this reviewed correction. Summary 04, lifecycle validation, canonical requirement completion, integration re-audit and final freshness remain root-owned and pending.

## Security Audit Native Gate Refresh | 2026-10-03T07:31:47Z

Root confirmed default `bash scripts/verify.sh` latest attempt 3 completed successfully in session **43990**, exit **0**. The auditor read the retained console at `/tmp/open-bitcoin-phase153-verify.log`, including the genuine runtime regression's passing result at line 6137, Bazel provenance pass at line 8090 and exact verifier footer at line 9581: `verify.sh completed in 45m 47.953s (2747953ms)`. Root's observed completion poll at 2026-10-03T07:28:54Z is the observation time, not the actual process-end timestamp.

The primary command timing record `.local/open-bitcoin-dev/command-timings/verify-full/2026-10-03T06-42-51.983Z-12e39330-617d-4761-936e-94d0adca2e44.json` records `verifyMode: full`, `outcome: success`, `exitStatus: 0`, start **2026-10-03T06:42:51.983Z**, end **2026-10-03T07:28:40.830Z** and outer duration **2,748,847 ms**. The outer timing includes wrapper overhead; the verifier's own duration is **2,747,953 ms**. Both describe the same successful attempt. The run used pinned Rust 1.94.1 and the default target against the current dirty checkout; no committed/pushed revision is asserted.

Root's confirmed full outcome covers formatting, strict Clippy, all-target/all-feature build, workspace tests including the nonignored genuine legal-target scenario, benchmark list/smoke, Bazel build/provenance and pure-core coverage. This updates `native_verification_status` to **passed** and adds actual execution support for the existing **CLOSED** T-153-14. All 14 mitigations remain closed with `threats_open: 0`; no new scan, source edit, test/build command, staging or commit occurred during this refresh.

The earlier failed-attempt/checker audit above remains historical evidence and is superseded only for native-run status by this actual full pass. `requirement_completion` remains **pending**. Summary 04 is being written, the formal phase verifier awaits it, and root lifecycle validation, integration re-audit, canonical completion and final freshness are still outstanding. This native pass does not claim a formal phase-verification pass, phase completion, commit or push.

## Final Canonical Activation | 2026-10-03

The paragraph above records the native refresh point. Formal Phase 153
verification subsequently passed 22/22 truths; root revalidated lifecycle
context, plans, summaries and verification, activated PRUN-01/PRUN-02 in
Summary 03 and REQUIREMENTS, and reconciled 8/8 phases and 34/34 plans.
The refreshed milestone audit has 17/17 canonical requirements, no blockers
and three inherited advisories. Current `requirement_completion` is
**complete**; all 14 mitigations remain closed. Git finalization evidence
is derived from the commit and upstream refs saving this report.
