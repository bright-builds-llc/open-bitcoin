---
phase: 158-validated-reorg-and-retained-branch-identity
verified: "2026-10-09T04:33:17Z"
status: passed
score: 21/21 must-haves verified
generated_by: gsd-verifier
lifecycle_mode: yolo
phase_lifecycle_id: 158-2026-10-08T02-17-15
generated_at: "2026-10-09T04:33:17Z"
lifecycle_validated: true
overrides_applied: 0
---

# Phase 158: Validated Reorg and Retained Branch Identity Verification Report

**Phase Goal:** Clients obtain branch-correct replacement filters after validated reorgs while previously indexed displaced blocks remain addressable by hash.
**Verified:** 2026-10-09T04:33:17Z
**Status:** passed
**Re-verification:** No — initial verification; no earlier VERIFICATION.md existed.

The three roadmap success criteria and all seven plans' must-haves are satisfied within the scoped internal software contract. Verification traced actual source, concrete assertion bodies, capability producers/consumers, native output and achieved receipts. SUMMARY statements were context, not sufficient proof.

## Method and Provenance

Completed verification plan: load guidance/lessons/phase contracts; trace observable behavior, artifacts, links and data; inspect refusal/recovery/resource and disconfirmation cases; corroborate native and independent review/security evidence; write this report and run the same-attempt lifecycle validator. No Cargo/Bazel job, Git action or implementation edit was performed by this verifier. The sole owned mutation is this report.

Material guidance: AGENTS.md/Repo-Local Guidance, AGENTS.bright-builds.md, placeholder-only standards-overrides.md, standards/index.md and architecture/code-shape/verification/testing/Rust/TypeScript standards. Both active lesson files were read completely: 7,188 bytes, 2,397 conservative estimated tokens. Project skill directories were absent; no new audit trigger or user correction occurred. Verification override/gate and structured verification references informed the checks.

CONTEXT, all seven PLAN frontmatters and all seven SUMMARY frontmatters share `lifecycle_mode: yolo` and `phase_lifecycle_id: 158-2026-10-08T02-17-15`, with compliant discuss/plan/execute generators and actual generated timestamps. This report carries that exact attempt and `generated_by: gsd-verifier`; none is direct-fallback. Native evidence and final security use the same lifecycle. No override was present or applied.

## Goal Achievement

### Observable Truths

Paths below are repository-relative. For readability, `core/` means `packages/open-bitcoin-chainstate/src/`; `node/` means `packages/open-bitcoin-node/src/`; `fork-tests/` means `node/sync/tests/filter_index/reorg/`. The full artifact/link tables below identify concrete paths. All cited Phase 158 required Rust tests executed successfully in the complete default native run; its normal output contains 119 `phase158_` successes across core and node.

| # | Truth | Status | Evidence |
| --- | --- | --- | --- |
| 1 | A continuous validated local fork rewinds the active index to its common ancestor and derives replacement filters and headers from that ancestor, including equal-height replacements. (Roadmap SC1; Plan05 truth1) | VERIFIED | `core/filter_index/catch_up/reorg.rs:28` checks old/generation/hash/shared endpoints. `node/storage/fjall_store/filters/reorg.rs:259` resolves the indexed common prefix by staged position and exact projection/hash. `fork-tests/fixtures.rs:103` separately stages genuine historical/same-block spends; `branches.rs:8` compares every expected active commitment and predecessor header for equal/longer/shorter/lagged cases. |
| 2 | After reorg and real Fjall reopen, previously indexed displaced blocks remain retrievable by block hash while the active height projection identifies the replacement branch. (Roadmap SC2; Plan05 truth3) | VERIFIED | `fork-tests/retention.rs:9` snapshots actual original records, performs production reorg, ordinary catch-up/own flush and configured reopen, then checks complete record equality by hash and active replacement by height. `fixtures.rs:263` drops runtime before reopening the same path; no proof/store clone is retained. |
| 3 | A reorg requiring missing retained body/undo inputs refuses explicitly and preserves conservative progress/protection; it never substitutes current coins, a different branch or hidden redownload. (Roadmap SC3; Plan06 truth1) | VERIFIED | `node/chainstate/filter_reorg/preflight.rs:176` reads actual retained body and exact genuine undo; `turn_inputs/undo.rs` checks decoded equality. `fork-tests/failures.rs:252` physically removes each durable mate, verifies the other remains, checks pre-preview state/mempool/locks/history equality, then drops/reopens and refuses again. |
| 4 | Equal-height different hashes cause a real branch transition, never height-only append. (01) | VERIFIED | Pure reducer renews branch from new hash; sealed storage captures old/new endpoints independently of height. `branches.rs` equal-height tests exercise progress before ancestor, within suffix and at old tip with generation/hash changes and replacement headers. |
| 5 | Lagging progress rewinds to its verified indexed common prefix without fabricating ancestor progress. (01) | VERIFIED | Reducer `check_shared_endpoint` requires `min(old.height, ancestor.height)` and exact old/ancestor identity. Storage independently verifies that prefix. `replacement_case` asserts actual observed startup progress rewinds to `observed.min(10)`, rather than claiming height10 when lagging. |
| 6 | Preview cannot mint accepted replacement facts or durable authority. (01) | VERIFIED | Core stage/accepted fields are private; only `absorb_staged_reorg_with_receipt` installs genuine effects and mints accepted receipt. Preview clones metadata without receipt; compiled external negative doctests and manager preview/lineage refusal assertions execute. |
| 7 | Accepted branch rewind atomically strengthens protection, renews generation and hides incompatible active suffix. (02) | VERIFIED | Storage `complete_basic_filter_reorg` rechecks store/revision/owner/coins and publishes state, owner and reserved lock in one SyncAll batch. New processed/safe shared prefixes mask physical suffix; preview/stale barriers apply. Reopen tests inspect actual conflicting raw rows before ordinary canonical rebinding. |
| 8 | Only authenticated next-height replacement work overwrites active projection rows; immutable hash records retain their original headers. (02) | VERIFIED | Live confirmed manager borrows current accepted positions; `ValidatedBasicFilterAppendPositions::validate_for` checks store/generation/branch/revision/frontier and every height/hash/parent. Append permits conflicting projection only for authenticated next height and requires immutable encoded-byte equality. Raw/foreign/old-position negative tests and retention assertions execute. |
| 9 | An old durable coins fence on the displaced branch cannot earn replacement safe progress. (02) | VERIFIED | `filters/append.rs:364` holds shared safe when exact `durable_displaced()` matches achieved `(height,hash)`. Ordinary own flush authenticates both coins and metadata before promotion. Repaired A23→B14→A23 regression plus missing/wrong-height/wrong-hash marker controls execute; returned A can earn its already genuine durable A fence. |
| 10 | Preview work is frozen without claiming accepted replacement. (03) | VERIFIED | `freeze_basic_index_reorg` validates composed work, sets same-store suspension and PreviewFrozen. Ordinary writers reject suspension. `failures.rs:344` checks old accepted target/generation/checkpoint/locks and blocked later work after actual mempool refusal. |
| 11 | Genuine absorption changes accepted identity before later persistence failure and successful reorg continues ordinary ordered catch-up. (03) | VERIFIED | `node/chainstate.rs:429` absorbs first, updates explicit accepted target through `accept_basic_index_reorg`, then performs fallible index publication/persist. Accepted failure tests distinguish replacement target from conservative achieved progress; ordinary live reorg and subsequent connect finish without reopen/re-enable. |
| 12 | Replacement safe progress advances only after a genuine ordinary own coins/metadata flush. (03) | VERIFIED | `flush_applying_plan` prepares lineage, completes actual coins+metadata writes and confirms private receipt; ownership checks same store, exact two-write revision, actual coins and accepted endpoint. No-pressure IfNeeded leaves coins unchanged. This obligation concerns earning a new replacement fence; returning to exact already-durable A reuses its earlier genuine fence, as explicitly tested. |
| 13 | Missing genuinely required body or exact body-bound historical undo refuses before preview/mempool effects. (04) | VERIFIED | Network `reorg_to_branch` calls production prepare before preview and mempool. Prepare performs genuine stage and required-source preflight; no partial success. Missing/corrupt/other-branch undo and deep physical source-loss tests check exact refusal and unchanged owner/mempool/history/protection. |
| 14 | Production reorg and sync reconciliation autonomously resume bounded branch-correct scheduled work. (04) | VERIFIED | `sync/block_reconcile.rs:271` invokes actual network reorg; authority serializes mutation. Catch-up `drive_turn` uses restored live owner, accepted positions and ordinary production budget. Real live Headers reconciliation and A→B→A→ordinary-child tests execute successfully. |
| 15 | Indexed immutable records can be reused after source pruning without substituting consensus-required inputs. (04) | VERIFIED | Required-source classification skips regeneration only for verified reusable exact common records. `retention.rs:86` actually prunes shared height20, performs repeated branches, asserts nonzero reuse with zero generation, reopens and checks exact immutable record while both sources remain absent. Separate disconnect source-loss tests still refuse despite indexed rows. |
| 16 | Equal-height, longer, lagging and repeated branch transitions run through production authority and ordinary scheduled turns. (05 truth2) | VERIFIED | `branches.rs` production matrix uses `ForkFixture::apply`→network reorg and `finish`→ordinary scheduler, with actual durable coins/Fjall. Standard fixture uses untouched runtime maturity100/flags, spends at101, forks at100 through106 and genuinely reopens. |
| 17 | Accepted persistence errors and interrupted index writes preserve truthful accepted versus safe state and reopen conservatively. (06 truth2) | VERIFIED | `failures.rs` covers preview, absorbed rewind faults, first/subsequent append faults and real body/undo/coins/metadata writer boundaries. Ambiguous AfterCommit poisons authority. Three configured recovery cases resume conflicting projections; coins-before-metadata mismatch explicitly refuses with exact saved index unchanged. |
| 18 | New index-only reorg work is bounded independently of unrelated prefix length and does not force a coins flush. (06 truth3) | VERIFIED | Fixed-size reducer/local shared endpoint publication, checked separate storage/source/fact ledgers and bounded admitted turns. `measurements.rs:163` asserts fixed-depth new record/projection/point operations across prefixes16/128/512; exact/one-under/exhaustion controls preserve effects. The separately executed experiment preserves 54 configurations/149 turns. Existing whole consensus stage/preview and required backlog retain their own scaling costs. |
| 19 | Contributor claims accurately describe tested continuous reorg/reopen/internal hash lookup and explicit input refusal without claiming external serving. (07 truth1) | VERIFIED | Scoped parity page, unique CFIX-03 surface, README links and UAT describe internal authority, actual finite refusal and maturity/halving limits; RPC159/peers160/operators161/integration162 remain pending. Mutation tests reject independent unsupported claims. Root subsequently activated canonical CFIX-03 and reconciled contributor claims; the independent final closure review is clean. |
| 20 | Native verification rejects broken lineage/projection/refusal/breadcrumb/verifier links and unsupported scope claims. (07 truth2) | VERIFIED | Fixed current-root checker traces actual ordered caller/callee boundaries; all449 mutation tests pass with849 assertions, including40 sealed visibility controls after WR-158-02. This verifier independently executed449/0 and live checker exit0; complete native repeats both. Source strings supplement real Rust evidence. |
| 21 | Full native verification, independent source/security review and lifecycle validation gate final completion. (07 truth3) | VERIFIED | Actual full exit file0, completed native footer19m24.571s, successful full timing JSON and40 normal result summaries totaling3,735/0/3 independently read. All 67 final review fingerprints match; the 55 non-claim source/test/script files remain identical to the native-tested snapshot. Final security is25/25 closed,0 open. Exact same-attempt provenance is checked for context/plans/summaries/report; the stronger lifecycle gate passed and is rerun after this metadata refresh. Canonical CFIX-03 is Complete; strict Git finalization remains root-owned. |

**Score:** 21/21 truths verified. The three roadmap criteria take precedence. Plan05 truths1/3 and Plan06 truth1 are direct restatements covered by rows1/2/3; all remaining plan truths appear separately, preserving every plan obligation without double counting.

### Context Decisions

| Decision | Evidence mapping | Result |
| --- | --- | --- |
| D-01 existing serialized owner, autonomous continuation | Rows11/14/16, live reorg and ordinary child | VERIFIED |
| D-02 preview/accepted/durable separation | Rows6/10/11/12/17, genuine absorb and fault matrix | VERIFIED |
| D-03 ancestry/hash/common indexed prefix | Rows1/4/5/8, predecessor headers and lagging cases | VERIFIED |
| D-04 trusted generation, bounded single owner | Rows7/8/14/18, stale work and fixed-depth ledger | VERIFIED |
| D-05 immutable retained hash records | Rows2/8/15/17, full record equality after real reopen | VERIFIED |
| D-06 recoverable coins/metadata fencing, no forced index flush | Rows9/12/18, actual own receipt and no-pressure cadence | VERIFIED |
| D-07 truthful progress/protection on failure | Rows3/7/17, poisoned publication/recovery or explicit refusal | VERIFIED |
| D-08 required input validation before preview | Rows3/13, actual body/undo loss and mempool equality | VERIFIED |
| D-09 genuine historical/same-block facts and valid reuse | Rows1/8/13/15/16, exact script expectations and mandatory disconnect inputs | VERIFIED |
| D-10 reserved prune/generation/startup ordering | Rows7/9/15/17, reserved CRUD, stale plan and interrupted-intent protection tests | VERIFIED |
| D-11 concrete continuous durable fixtures and full failure matrix | Rows1–3/16–18, default test execution and actual measurement experiment | VERIFIED |
| D-12 parity/breadcrumbs/README/UAT/native/source/security/lifecycle | Rows19–21, 997-file breadcrumb native gate, final 67-path review and25-threat audit | VERIFIED |

### Required Artifacts

Every declared artifact passed `gsd-tools verify artifacts`; all18 exist, are substantive and have concrete consumers or executed test/document registrations. Pure utilities/configuration have no UI data-source obligation.

| Artifact | Expected | Status | Substantive implementation and wiring |
| --- | --- | --- | --- |
| `packages/open-bitcoin-chainstate/src/filter_index/catch_up/reorg.rs` | Constant-size checked reducer | VERIFIED | Imported through catch_up module; called by manager preparation with independently authenticated endpoints. |
| `packages/open-bitcoin-chainstate/src/engine/stage.rs` | Borrowed stage facts and absorbed receipt | VERIFIED | Engine registers stage; network manager stages/absorbs through its implemented methods. |
| `packages/open-bitcoin-node/src/storage/fjall_store/filters/reorg.rs` | Guarded same-store preparation/acceptance | VERIFIED | Lineage bridge consumes prepared token and genuine accepted receipt; guarded SyncAll publisher mints achievement. |
| `packages/open-bitcoin-node/src/storage/fjall_store/filters/tests/reorg.rs` | Concrete projection/hash evidence | VERIFIED | Registered in filters/tests; real store/manager fixture, equality and foreign/refusal tests execute. |
| `packages/open-bitcoin-node/src/chainstate/filter_reorg.rs` | Preview/accepted/durable manager transition | VERIFIED | Registered under chainstate; prepare/freeze/accept/durable calls appear in ordinary manager flow. |
| `packages/open-bitcoin-node/src/chainstate/fjall_store.rs` | Tracked lineage/own receipt | VERIFIED | Genuine recovered manager owns private lineage; ordinary flush completes and ownership confirms receipts. |
| `packages/open-bitcoin-node/src/network/runtime_authority/filter_index/reorg.rs` | Production retained-source preflight route | VERIFIED | Mempool lifecycle calls wrapper→manager prepare→native required-source preflight before preview. |
| `packages/open-bitcoin-node/src/network/mempool_lifecycle.rs` | Preflight before preview with original ordering | VERIFIED | Serialized authority and actual sync reconciliation invoke implemented reorg path; mempool precedes commit as required. |
| `packages/open-bitcoin-node/src/sync/tests/filter_index/reorg/fixtures.rs` | Continuous coins/Fjall validated forks | VERIFIED | Registered recipe used by every integrated branch/fault/protection/work module; genuine oracle, actual configured runtime. |
| `packages/open-bitcoin-node/src/sync/tests/filter_index/reorg/branches.rs` | Production branch matrix | VERIFIED | Registered non-ignored branch tests call ordinary reorg/turn/flush/reopen, compare commitments and actual coins. |
| `packages/open-bitcoin-node/src/sync/tests/filter_index/reorg/retention.rs` | Exact immutable hash/reopen evidence | VERIFIED | Registered tests compare complete old/new records after actual closure and verify real pruned sources. |
| `packages/open-bitcoin-node/src/sync/tests/filter_index/reorg/failures.rs` | Physical loss and writer/reopen matrix | VERIFIED | Registered non-ignored tests use actual writer seams and configured recovered turns; no success inferred from injection alone. |
| `packages/open-bitcoin-node/src/sync/tests/filter_index/reorg/measurements.rs` | Concrete ignored measurement harness | VERIFIED | One opt-in timing experiment separately executed; required fixed-count/admission tests run normally. |
| `.planning/phases/158-validated-reorg-and-retained-branch-identity/158-REORG-MEASUREMENTS.md` | Actual accounting and finite-limit rationale | VERIFIED | 54 preparations/149 turns, preserved ledgers/timings and explicit scope, linked by parity/UAT/checker. |
| `scripts/check-phase158-validated-reorg.ts` | Deterministic bounded current-root guard | VERIFIED | Default verifier executes it; current live roots pass and mutations cover actual routes/claims/breadcrumbs. |
| `scripts/check-phase158-validated-reorg.test.ts` | Checker mutation rejection | VERIFIED | Explicit default Bun file run,449 tests/849 assertions; no required ignored test. |
| `.planning/phases/158-validated-reorg-and-retained-branch-identity/158-UAT.md` | Copyable repo-local software workflow | VERIFIED | Timed Cargo/Bazel commands, actual named tests and scoped expected evidence, linked by parity. Optional manual future review creates no current human gate. |
| `.planning/phases/158-validated-reorg-and-retained-branch-identity/158-NATIVE-EVIDENCE.md` | Actual final native outcomes/limits | VERIFIED | Exit file, full log/result totals, full timing JSON and reviewed hashes corroborate the recorded pass. |

### Key Link Verification

All17 declared links were traced manually. The generic key-link helper passed the three text links in Plan07 but reported the14 Rust links as `Target not referenced in source`: it looks for literal target paths rather than Rust module/method resolution. Actual registered modules and implemented caller/callee chains below establish those links; this is not an override or a hidden wiring gap.

| Plan | From → To | Via | Status | Concrete trace |
| --- | --- | --- | --- | --- |
| 01 | core catch_up.rs → catch_up/reorg.rs | replace_validated_branch | WIRED | Module registration plus implemented reducer invoked by prepare_basic_index_reorg. |
| 01 | core engine.rs → engine/stage.rs | genuine absorption receipt | WIRED | Private types in engine; stage module implements receipt creation only after effect absorption. |
| 02 | node filters/reorg.rs → filters/publication.rs | guard and SyncAll batch | WIRED | Imports fault gates; filter_publication_guard and finish_basic_filter_batch used around atomic state/owner/lock write. |
| 02 | node filters/append.rs → filters/ownership.rs | replacement permission/fence | WIRED | Shared append identity/proof, ValidatedReorg/RecoveredPrefix checks and exact displaced tuple predicate. |
| 03 | node chainstate.rs → chainstate/filter_reorg.rs | prepare/preview/absorb/failure | WIRED | prepare_basic_index_reorg, freeze, check, accept and note-durable called at actual state boundaries. |
| 03 | node chainstate/fjall_store.rs → filters/ownership.rs | completed own flush | WIRED | Pending→CompletedValidatedFlush through actual coins/metadata execution; confirm_basic_filter_flush checks receipt and two revisions. |
| 04 | node network/mempool_lifecycle.rs → authority/filter_index/reorg.rs | preflight before preview | WIRED | prepare_reorg wrapper→manager prepare→required_sources succeeds before install_prepared_reorg_preview. |
| 04 | node catch_up/inputs.rs → chainstate/filter_reorg.rs | canonical next-height facts | WIRED | Inputs module supplies bounded undo acquisition; owning catch_up.rs reads manager accepted facts and current accepted-position token before replacement append. The helper does not itself mint authority. |
| 04 | node sync/block_reconcile.rs → network/mempool_lifecycle.rs | production reorg | WIRED | Stored replacement/disconnect bodies→runtime.network.reorg_to_branch→serialized network implementation. |
| 05 | fork-tests/branches.rs → network/mempool_lifecycle.rs | ordinary authority/turns | WIRED | Fixture apply delegates reorg_to_branch; finish calls drive_basic_filter_index_turn; assertions check achieved branch. |
| 05 | fork-tests/retention.rs → sync/open_runtime.rs | real configured reopen | WIRED | fixture.reopen→drop(runtime)→FjallNodeStore::open(same path)→DurableSyncRuntime::open_configured. |
| 06 | fork-tests/failures.rs → chainstate/filter_reorg.rs | disposition assertions | WIRED | Real reorg/fault entry points assert preview versus explicit absorbed target and safe state, then actual recovered runtime. |
| 06 | fork-tests/protection.rs → fjall_store/prune.rs | reserved/stale generation | WIRED | Actual lock CRUD and flush_applying_prune_plan/automatic owner/intent recovery drive guarded paired deletion. |
| 06 | fork-tests/measurements.rs → filters/reorg.rs | actual work observations | WIRED | Serialized genuine preparation getter observes checked storage/source/fact ledgers; actual reorg/turn point/batch counters read concrete store. |
| 07 | scripts/verify.sh → Phase158 checker | explicit default test/checker | WIRED | Both documented/executed run_step sections run Phase158 after157; full log corroborates execution. |
| 07 | docs/parity/index.json → v2-5-validated-reorg.md | unique scoped CFIX-03 evidence | WIRED | Pinned baseline, unique surface, exact source/test/doc roots and intentional finite-policy difference checked. |
| 07 | phase UAT → fork-tests registration | reproducible Cargo tests/reopen | WIRED | Actual --lib phase158_ selectors and required registered modules, plus existing operator Cargo/Bazel forms. |

### Data-Flow Trace (Level 4)

There is no dynamic UI in this phase. Dynamic internal data was nevertheless traced end to end; no static fallback or empty prop substitutes for real history.

| Artifact/consumer | Data | Actual upstream source | Produces real data | Status |
| --- | --- | --- | --- | --- |
| Manager reorg preparation | ancestry/undo/common indexed prefix | Validated staged overlay from genuine old chain and replacement bodies, exact Fjall projections/hash rows | Yes; independently checked identities | FLOWING |
| Ordinary catch-up | next body and historical/same-block spent scripts | Captured genuine first replacement facts, later admitted native bodies and exact undo/current accepted positions | Yes; missing required sources refuse | FLOWING |
| Guarded append | bytes/hash/header/parent and projection | StoredFilterRecord generation from historical inputs and exact ancestor predecessor; immutable equality | Yes; authenticated contiguous rows | FLOWING |
| Durable checkpoint | safe endpoint/coins fence/lock | Actual ordinary coins+metadata completed receipt, or already genuine matching recovered fence | Yes; displaced fence cannot be credited | FLOWING |
| Internal active/hash reads | replacement active row, displaced immutable record | Real persisted Fjall keys after all-handle drop and configured reopen | Yes; full record equality tested | FLOWING |

### Behavioral Spot-Checks and Native Evidence

Only read-only checks/saved output inspection ran in this verifier; the root/executor earned the serialized Rust/Bazel contract. No server or external service was started.

| Behavior | Command/evidence | Actual result | Status |
| --- | --- | --- | --- |
| Current source/claim routes | `/tmp/open-bitcoin-bun-1.3.9-fresh/bun run scripts/check-phase158-validated-reorg.ts` | Exit0, current source/claims checked | PASS |
| Required artifact completeness | gsd-tools verify artifacts for all seven plans | 18/18 passed after native artifact arrived | PASS |
| Reviewed snapshot integrity | SHA-256 recomputation of final consolidated 67-path inventory | 67 matches, 0 mismatches; closure review confirms 55 non-claim files unchanged | PASS |
| Native exit/normal test counts | Read `/tmp/phase158-native.exit` and parse40 normal summaries in `/tmp/phase158-native.log`, excluding coverage rerun | Exit0;3,735 passed/0 failed/3 opt-in exclusions;119 required Phase158 successes | PASS |
| Native full completion | Full timing receipt `2026-10-09T03-59-49.916Z-d41180bd-1821-41b8-a17d-ad0f740bac81.json` | Full mode success,exitStatus0;03:59:49.916Z–04:19:15.143Z; native footer1,164,571ms | PASS |
| Lifecycle consistency | `node /Users/peterryszkiewicz/.codex/get-shit-done/bin/gsd-tools.cjs verify lifecycle 158 --require-plans --require-verification --raw` | Initial report and final canonical-closure metadata refresh both returned `valid`, exit 0 | PASS |

Additional independently executed evidence: all449 checker mutation tests passed,0 failed,849 assertions in34.87s. This whole suite is supplementary and exceeds the ten-second lightweight spot-check window. Default native repeats it and all600 inherited Phase157 mutations. Complete default formatting, all-target/all-feature strict Clippy, build, Rust tests, benchmark smoke/report, six Bazel targets/provenance and pure-core no-uncovered-lines gate passed. The coverage helper covers its eight listed pure crates; no adapter-wide or invented percentage claim follows.

The three normal opt-in exclusions are the public-network smoke and Phase157/158 timing experiments. No required behavior test is ignored. Plan06's separately executed Phase158 experiment earned1 pass/0 failed/0 ignored,54 configurations/149 ordinary turns; those timings are not counted as native normal tests.

### Requirements Coverage

| Requirement | Source plans | Description | Status | Evidence |
| --- | --- | --- | --- | --- |
| CFIX-03 | 158-01 through158-07 | Branch-correct replacement filters/headers, indexed displaced lookup by hash, explicit missing deep-reorg input refusal | SATISFIED / COMPLETE | Roadmap truths1–3 and all detailed rows; exact real source/fork/reopen/failure evidence and completed default native run. Root's phase-complete CLI activation is reflected by the checked canonical requirement, unique Phase158 Complete traceability row, done parity owner and Plan07 `requirements-completed: [CFIX-03]` with earned closure note. |

No orphaned requirement exists: REQUIREMENTS maps only CFIX-03 to Phase158, and all seven PLAN requirement fields claim it. Root's earned closure records five of nine phases and 33/33 created plans complete, with ten requirements Complete and twelve Pending. This verifier only corroborated canonical activation; it did not mutate requirement tracking.

### Anti-Patterns and Disconfirmation

| File/concern | Pattern inspected | Severity | Finding/impact |
| --- | --- | --- | --- |
| New/changed reorg reducer, stage, manager, store, preflight, integrated tests and checker | TODO/FIXME/XXX/HACK/placeholder/empty implementation | None | No goal-blocking stub, orphan or static history fallback found. Initial optional absence and empty disconnect result are legitimate checked states. |
| Concrete fault fixture | Test-only helpers/empty callback | Info | `reopen_inspecting` default no-op callback merely permits optional post-drop observation; real runtime drop/reopen still executes. Test helpers cannot mint production capability. |
| WR-158-01 | Taller genuine displaced fence falsely refused | Resolved | Exact tuple predicate reused; independent repair review and genuine retained-source RED/GREEN plus3 negative controls; final native regression passed. |
| WR-158-02 | Broad pub(super) visibility exemption | Resolved | Exception narrowed to exact existing proof identity;40 visibility controls, independent delta review and449 mutation passes. |
| Historical Phase103 harness | Two grouped tests exceeded default deadline | Resolved | Eight isolated cases preserve all mutations/fresh fixtures/default5,000ms timeout;14 cases/16 assertions and independent delta review, final native pass. |

Inversion checked three plausible false passes: equal height hiding a different branch; old taller durable coins credited to replacement; in-memory source cache hiding physical undo loss. Actual ancestry/reducer, exact achieved tuple/negative controls and full-drop physical source-loss tests address each.

Disconfirmation limits were retained rather than generalized away. A fully retained401-height shared gap can exceed existing source work and explicitly refuse before effects; arbitrary retained-fork liveness is only partially supported and is not this contract. The expected-filter oracle shares the production encoder, so it alone would not independently prove BASIC protocol generation; the fixture separately asserts exact historical/same-block scripts/ancestor identities and prior pinned BASIC oracle/native gates remain supplementary generation evidence. The BeforeBody writer case is an actual storage fault with explicit host failure projection, not an autonomous new daemon body-error recovery proof. Coins-before-metadata failure is deliberately covered by explicit startup refusal, not successful repair. None changes the achieved Phase158 truths.

### Independent Security and Source Evidence

`158-REVIEW.md` is clean across the final union of 67 source/test/script/doc/closure paths, with WR-158-01/02 explicitly resolved. All 67 reviewed hashes matched current files in this verifier. Original independent core/storage, runtime/fixture, automation and delta/historical-harness reports preserve their actual findings and resolution; the native-tested harness hash matches. `158-REVIEW-CLOSURE-CHECK.md` independently reviews all twelve final claim/tracking artifacts and confirms the 55 non-claim source/test/script files are unchanged. Five claim updates and seven additional closure metadata/evidence paths reconcile earned completion without invalidating native behavior evidence. Generated LOC remains separately owned freshness data.

`158-SECURITY.md` is final (`status: secured`, `final_snapshot: true`),25/25 declared mitigations CLOSED,0 open, all dispositions mitigate and no inferred accepted risk. The final auditor inspected native/Plan07 receipts and same-attempt lifecycle implementation. Earlier six summaries lack dedicated Threat Flags sections; their narrative limitations were explicitly mapped and Plan07 provides its actual None-unregistered section. This is a declared-mitigation audit, not a broad security certification or funds-readiness proof.

### Human Verification Required

None for the Phase158 contract. This is an internal headless lifecycle with deterministic local software fixtures and reproducible automated branch/reopen/failure checks. Optional future manual review in UAT creates no current human gate. No visual, external service, public-network, real-time UX or physical power-loss behavior is required for these truths.

### Deferred Scope and Residual Limits

No identified in-scope gap was deferred. The full milestone roadmap explicitly assigns authenticated filter/index RPC to159, peer filter families/transport to160, operator/status/dashboard/support projections to161 and integrated post-prune client proof to162. Those absent external surfaces are planned later work, not Phase158 failures. Public defaults/network attempts, V0/type2/BIP37, archive-scale and production/funds claims remain excluded.

Existing finite storage/source/fact envelopes may refuse fully retained deep/backlogged forks before effects; parity records this intentional local resource policy. New index-local work is bounded without a duplicate full-history cache, while existing consensus stage/preview clones and necessary source processing retain their costs. Measurements are software reservations and whole-call/stage-inclusive times, not RSS, isolated fsync/publication latency or hard latency guarantees.

Compact fixtures use explicitly documented maturity1/easy PoW. Separate default-runtime maturity100/actual-flags proof spends at101 and forks below150; runtime halving210,000 differs from Knots regtest150, so full regtest parameter parity is not claimed. Software injected writer failures and true reopen do not establish hardware crash/power-loss resilience. Normal ordinary coins cadence and reserved prune authority remain enforced.

### Gaps Summary

No must-have, artifact, key link, requirement or blocker remains unsatisfied. Native execution and final declared-security/source evidence were earned before the initial pass. The same-attempt validator accepted the formal report with both plans and verification required (`valid`, exit 0). Root then activated CFIX-03 and reconciled canonical requirement/completion claims; the clean twelve-path closure review and all 67 final fingerprints corroborate that refresh. The 21/21 score and passed status are unchanged. Strict Git finalization remains root-owned; no commit, mandatory hook or push success is inferred before its actual execution.

***

_Verified: 2026-10-09T04:33:17Z_
_Verifier: gsd-verifier_
