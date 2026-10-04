---
phase: 155-recoverable-index-and-pre-prune-startup-protection
verified: 2026-10-04T19:23:50Z
status: passed
score: 10/10 must-haves verified
generated_by: gsd-execute-phase
lifecycle_mode: yolo
phase_lifecycle_id: 155-2026-10-04T16-01-08
generated_at: 2026-10-04T19:23:50Z
lifecycle_validated: true
overrides_applied: 0
full_native_gate: passed
source_review: clean
security_closure: secured
re_verification:
  previous_status: "gaps_found"
  previous_score: 9/10
  gaps_closed:
    - Default full native gate and security threat T-155-15 closure.
  gaps_remaining: []
  regressions: []
must_haves:
  truths:
    - Real Fjall reopen recovers integrity-checked records and safe progress or refuses without erasing a valid prefix or advancing a phantom cursor.
    - Index writes preceding coins/metadata flush retain immutable records and reconcile active progress to the recovered branch and durable checkpoint.
    - Production reopen validates index protection before initialize resumes prune intent and preserves required bodies/undo on refusal.
    - Record/checkpoint/protection persistence faults retain conservative protection and explicit failure evidence without record-only cursor authority.
    - Invalid BASIC encoding and bounded resource accounting are rejected.
    - Recovery authority depends on recovered coins and compatible ancestry rather than numeric height.
    - Next-required-input protection covers genesis/height one and avoids arithmetic overflow.
    - Same-database publication atomically persists records, visible checkpoint and protection within a verified coins fence.
    - Full index absence preserves legacy startup and safe already-indexed prune recovery.
    - Contributor evidence is auditable through the passed native contract without claiming deferred product capabilities.
---

# Phase 155: Recoverable Index and Pre-Prune Startup Protection Verification Report

**Phase Goal:** Operators can reopen a durable index without trusting progress beyond recovered chainstate or losing required history to startup prune recovery.
**Verified:** 2026-10-04T19:23:50Z
**Status:** passed
**Re-verification:** Yes — provisional native-evidence gap closed; source regression check found no changes

All ten distinct must-haves are verified. The parent-owned default full native gate passed, and SECURITY records all 17 threats closed. The provisional evidence gap is closed; no implementation gap, regression, human verification item or override remains.

Material guidance: repo-local AGENTS.md, AGENTS.bright-builds.md, placeholder-only standards-overrides.md, standards/index.md, architecture, code-shape, local-guidance, testing, verification, Rust and TypeScript/JavaScript standards. Both active lessons were read completely: 7,188 bytes and 2,397 conservative estimated tokens; no archive was loaded and no audit trigger fired. Read CONTEXT/RESEARCH, all four PLANs/SUMMARYs, ROADMAP/REQUIREMENTS, REVIEW/SECURITY, production source and behavioral assertions. SUMMARY execution claims were cross-checked against source and timing records rather than accepted as implementation proof.

## Goal Achievement

### Observable Truths

The score counts ten distinct truths. All four roadmap success criteria are retained verbatim below. Six plan truths restate them; six add detail. The next table maps every plan truth, so deduplication does not reduce scope.

| # | Truth | Status | Evidence |
| --- | --- | --- | --- |
| 1 | An operator reopens real Fjall after successful or interrupted index writes and recovers integrity-checked records plus safe progress, or receives a fail-closed diagnostic without erasing a valid prefix or advancing a phantom cursor. (CFIX-02) | VERIFIED | `storage/filter_index.rs:193` validates bounded bytes/hash/header; `fjall_store/filters.rs:194` validates all immutable rows and direct ancestry edges. Store close/reopen success/conflict/corruption tests and `sync/tests/filter_index/faults.rs:11` demonstrate retained ahead records after an actual committed-write error. |
| 2 | When filter writes precede a successful coins/chain-metadata flush, restart retains valid immutable records but reconciles active projection and resume cursor to the recovered branch and durable chainstate checkpoint. (CFIX-04) | VERIFIED | `filter_index/recovery.rs:96` selects a contiguous common recovered prefix; `filters.rs:162` hides suffix beyond checkpoint. Validated spend/fork production tests at `sync/tests/filter_index/recovery.rs:250`, `:374` retain both immutable branches, while `faults.rs:59`, `:158` exercise concrete partial coins replay and metadata failure. |
| 3 | A production durable-runtime reopen with a live interrupted prune intent validates index-owned protection before initialize can call resume_prune_intent; required bodies and undo survive, or startup refuses before unsafe deletion. (CFPR-03) | VERIFIED | `sync/open_runtime.rs:38` calls initialize before manager construction; `flush_lifecycle.rs:235` unconditionally calls the index guard before resume/readiness/cache. `filters/startup.rs:95` directly checks intent before mutation. `sync/tests/filter_index/startup.rs:12` asserts exact required body/undo, live intent, unchanged index snapshot, checkpoint and protection after actual runtime reopen refusal. |
| 4 | Faults in record, checkpoint or protection persistence leave conservative protection and explicit failure evidence; stored filter records alone never authorize a cursor ahead of recoverable chainstate. (CFIX-02, CFIX-04, CFPR-03) | VERIFIED | `filters/publication.rs:132` batches complete state/protection with SyncAll; `:331` poisons uncertain publication and propagates errors. Concrete before-record/checkpoint/protection and after-commit faults are reopened through production runtime in `sync/tests/filter_index/faults.rs:239`; startup reconciliation faults are in `startup/faults.rs:11`. |
| 5 | Stored BASIC bytes are rejected when their encoding or bounded resource accounting is invalid. | VERIFIED | Codec `block_filter/validation.rs:46` bounds bytes/count/minimum bits and streams checked deltas; its tests reject noncanonical count, truncation, out-of-range values, overflow, padding and trailing bytes. Node parser validates before hashing and accepts no malformed empty substitute. |
| 6 | Safe recovery progress depends on recovered coins and compatible ancestry rather than a numeric height alone. | VERIFIED | Core `filter_index.rs:189` recomputes metadata hashes, proves unique contiguous genesis ancestry and requires recovered B equal to its tip; publication `:209` rereads durable B/H/meta and compares the entire fence. Equal-height wrong branch and B/meta mismatch tests exercise actual production reopen. |
| 7 | Recovery facts protect the next required input, including genesis and height 1, without height arithmetic overflow. | VERIFIED | Core `filter_index.rs:149` uses checked next height and explicit exhaustion; `:236` validates reserved lock endpoint; `:269` directly rejects intent at/above earliest-required. Actual production tests cover 0/1/u32::MAX and malformed overflowing protection; terminal exhaustion is proven by pure tests. |
| 8 | Same-database publication atomically persists records, visible checkpoint and protection only within a verified coins fence. | VERIFIED | `filters/publication.rs:132` validates fence, saved protection, candidate identity and full target prefix; one same-database SyncAll batch publishes immutable rows, changed projection, explicit state and preserved full operator lock map. Rewind retains all immutable/suffix rows. |
| 9 | Fully absent index metadata preserves legacy startup and safe already-indexed prune recovery. | VERIFIED | `filters/startup.rs:44` permits absence only when state, rows, projection and reserved protection are absent. Production controls `startup.rs:109`, `:138`, `:501` actually delete eligible paired payloads or remaining mate and clear intent; partial artifacts refuse. |
| 10 | Contributors can reproduce auditable native evidence without claiming activation, catch-up, serving or later pruning behavior. | VERIFIED | Unique parity owner, mapped breadcrumbs, default executable checker/test steps and truthful scoped documentation exist. Independent pinned Bun 1.3.9 checker and 53 mutation tests pass. Default full native run completed successfully with workspace/integration/doc tests, benchmark smoke, Bazel provenance and zero uncovered pure-core lines; T-155-15 is CLOSED. |

**Score:** 10/10 distinct truths verified; 4/4 roadmap criteria verified. Zero overrides were requested or applied.

### Complete PLAN Truth Coverage

| Plan / truth | Required behavior | Observable truth |
| --- | --- | --- |
| 01 / 1 | Invalid encoding/resource accounting refusal | 5 |
| 01 / 2 | Coins/ancestry recovery authority | 6 |
| 01 / 3 | Required input and height arithmetic | 7 |
| 02 / 1 | Real reopen, immutable conflicts/corrupt envelopes | 1 |
| 02 / 2 | Atomic fenced state/protection publication | 8 |
| 02 / 3 | Record/checkpoint/protection failure safety | 4 |
| 03 / 1 | Production guard before prune resume | 3 |
| 03 / 2 | Missing/corrupt/weak protection and unsafe intent refusal | 3 |
| 03 / 3 | Legacy absence and safe intent controls | 9 |
| 04 / 1 | Retained ahead/displaced rows and recovered cursor | 2 |
| 04 / 2 | Production persistence/coins-recovery failure matrix | 4 |
| 04 / 3 | Auditable native evidence and scoped claims | 10 |

### Required Artifacts

All fourteen declared artifacts exist, are substantive and are wired to their required consumer. Tests are wired through registered Rust modules or default native Bun steps; no public activation consumer is required by this phase.

| Artifact | Expected / wiring | Status |
| --- | --- | --- |
| `packages/open-bitcoin-codec/src/block_filter/validation.rs` | Public bounded validator/error, exported by block_filter.rs and consumed by node parser | VERIFIED |
| `packages/open-bitcoin-chainstate/src/filter_index.rs` | Exported identity/fence/checkpoint/protection types consumed by storage/startup | VERIFIED |
| `packages/open-bitcoin-chainstate/src/filter_index/recovery.rs` | Pure reducer consumed by concrete checkpoint scan/startup | VERIFIED |
| `packages/open-bitcoin-node/src/storage/filter_index.rs` | Internal versioned bounded codecs consumed by Fjall reads/publication | VERIFIED |
| `packages/open-bitcoin-node/src/storage/fjall_store/filters.rs` | Concrete reads/integrity/projection scan consumed by mandatory startup guard | VERIFIED |
| `packages/open-bitcoin-node/src/storage/fjall_store/filters/publication.rs` | SyncAll checkpoint/protection transition called by startup reconciliation | VERIFIED |
| `packages/open-bitcoin-node/src/storage/fjall_store/filters/tests/faults.rs` | Registered real-store fault/reopen and linear-read assertions | VERIFIED |
| `packages/open-bitcoin-node/src/storage/fjall_store/filters/startup.rs` | Mandatory recovered-coins pre-prune consumer | VERIFIED |
| `packages/open-bitcoin-node/src/chainstate/flush_lifecycle.rs` | Unconditional early guard before resumed deletion | VERIFIED |
| `packages/open-bitcoin-node/src/sync/tests/filter_index/startup.rs` | Registered production reopen refusal and safe deletion controls | VERIFIED |
| `packages/open-bitcoin-node/src/sync/tests/filter_index/recovery.rs` | Registered actual validated spend/fork plus real runtime reopen | VERIFIED |
| `packages/open-bitcoin-node/src/sync/tests/filter_index/faults.rs` | Registered concrete coins/meta and publication fault evidence | VERIFIED |
| `scripts/check-phase155-filter-index.ts` | Tested structural/scope guard executed by default verify.sh | VERIFIED |
| `docs/parity/index.json` | Exactly one evidence owner for the three phase requirements | VERIFIED |

Initial `gsd-tools verify artifacts` reported 13/14 because its parser treated the inline YAML exports list as one literal name. Both public declarations and their actual consumer were directly verified. The parent subsequently normalized the same two names to a block list without changing semantics; rerunning Plan 01 artifact verification passed 3/3, bringing all declared artifacts to 14/14. No override was used.

### Key Link Verification

All eight declared links passed the tool check and substantive manual trace.

| From | To / connection | Status | Evidence |
| --- | --- | --- | --- |
| codec block_filter.rs | validation.rs | WIRED | Public module export; node parser imports/calls validator. |
| core recovery.rs | filter_index.rs types | WIRED | Typed reducer/fence/checkpoint/protection decisions. |
| Fjall filters.rs | storage/filter_index.rs | WIRED | Decode/parse calls consume actual BlockIndex bytes. |
| publication.rs | Fjall BlockIndex and prune_locks | WIRED | One real SyncAll batch with complete state and full lock map. |
| sync/open_runtime.rs | flush_lifecycle initialize | WIRED | Actual production constructor calls initialize before manager/cache use. |
| initialize | filters/startup.rs | WIRED | Unconditional guard after coins recovery, before lock reload/resume. |
| production recovery tests | DurableSyncRuntime production constructor | WIRED | Helper drops stores, opens real Fjall, calls DurableSyncRuntime::open, which delegates to open_with_runtime_activation. |
| verify.sh | Phase 155 checker/test | WIRED | Default executable run_step commands at line 311; no comment-only or optional fast-path substitute. |

### Data-Flow Trace (Level 4)

No component renders dynamic UI in this phase. Equivalent persisted-state flows were traced beyond symbol existence.

| Consumer / data | Actual source | Real data / decision |
| --- | --- | --- |
| Startup recovered best block | FjallCoinsView B after apply_recovery_decision consumes H/B replay | FLOWING — not SyncProgress/status/header height. |
| VerifiedChainstateFence | Recovered B plus loaded full durable active chain metadata | FLOWING — hash/parent/unique-height proof and B-tip equality. |
| Immutable record validity | Actual BlockIndex prefix iteration and parent-key reads | FLOWING — bounded encoding/hash/header/direct-edge checks for every row, including hidden branches. |
| Safe recovery endpoint | Contiguous saved projection decoded from real keys plus recovered ancestry | FLOWING — common prefix only, never opportunistic promotion of record-only rows. |
| Pre-delete protection | Saved state/checkpoint plus reserved persisted lock, then pure recovered required-input predicate | FLOWING — missing/weak/corrupt pair refuses; live intent checked before publication/deletion. |
| Reconciled durable state | Same-store atomic SyncAll batch and checkpoint-gated visibility | FLOWING — immutable/suffix bytes retained and stronger protection recoverable before resume. |

### Behavioral Spot-Checks and Execution Evidence

Verifier-owned checks were brief; no Cargo/Bazel process or server was started while the parent held the build lock.

| Behavior | Command / evidence | Result | Status |
| --- | --- | --- | --- |
| Source/parity/guard/scope links | Pinned Bun 1.3.9 `run scripts/check-phase155-filter-index.ts` | Exit 0; actual evidence tree accepted | PASS |
| Missing links/order/false claims rejected | Pinned Bun 1.3.9 `test ./scripts/check-phase155-filter-index.test.ts` | 53 passed, 0 failed, 93 assertions; 0.91s | PASS |
| Native shell syntax | `bash -n scripts/verify.sh` | Exit 0 | PASS |
| Diff whitespace | `git diff HEAD --check` | Exit 0 | PASS |
| Artifact/key-link analysis | gsd-tools on all four PLANs, Plan 01 rerun after YAML-only normalization | 14/14 artifacts and 8/8 links; direct substantive/wiring trace also passed | PASS |
| Lifecycle upstream provenance | `gsd-tools verify lifecycle 155 --require-summaries` | valid=true, no reasons; context, four plans and four summaries share formal lifecycle | PASS |
| Actual expanded runtime recovery | Timing record `phase155-production-recovery-matrix` | success/exitStatus 0, 18:15:28.526Z–18:16:38.127Z; SUMMARY reports 63 matching tests; assertions independently inspected | PASS — executor evidence corroborated |
| Final changed fault fixtures | Timing record `phase155-final-fault-matrix` | success/exitStatus 0, 18:17:41.406Z–18:20:16.532Z; four tests reported; concrete flush/intent source inspected | PASS — executor evidence corroborated |
| Existing initialize/runtime restart | Corresponding timing JSON | Both success/exitStatus 0; 23 initialize / 10 restart tests reported | PASS — overlapping suites, not additive count |
| Scoped pure-core coverage | Timing `phase155-core-coverage`, core-coverage.json | Success/exitStatus 0; subsequently corroborated by final full native coverage | PASS |
| Default full native contract | Parent `bash scripts/verify.sh`, `.local/open-bitcoin-dev/phase155/full-native.log`, verify-full timing JSON | Exit 0; terminal duration 50m 23.935s (3,023,935ms); wrapper success/exitStatus 0, 18:28:50.266Z–19:19:14.762Z; pinned Bun 1.3.9 and Rust 1.94.1 | PASS |
| Full coverage, builds and integration | Actual full log plus verify.sh/helper contract | Format, Clippy, all-target build, workspace/integration/doc tests, benchmark smoke, Bazel smoke and provenance passed. Coverage helper returns failure on any Uncovered Lines; successful final default run proves zero uncovered pure-core lines. | PASS |
| Security closure | Final SECURITY frontmatter and T-155-15 gate delta | secured; 17 closed, 0 open; native_gate passed | PASS |

### Requirements Coverage

| Requirement | Source plan | Description | Status / evidence |
| --- | --- | --- | --- |
| CFIX-02 | 01–04 | Real Fjall successful/interrupted write recovery without phantom cursor | SATISFIED — truths 1/4/5/8, immutable conflict/corruption and actual fault/reopen matrix, corroborated by the passed full gate. |
| CFIX-04 | 01–04 | Recovered coins/metadata fence and branch reconciliation after ahead work | SATISFIED in implementation — truths 2/6, genuine validated spend/alternative branch, actual partial coins and failed metadata flush. |
| CFPR-03 | 01–04 | Protection validation before resumed startup deletion | SATISFIED in implementation — truths 3/7/9, mandatory production constructor/initialize seam and separate body/undo/intent assertions. |

Requirements traceability assigns exactly those three IDs to Phase 155; all four plans claim them. No orphaned phase requirement exists. All three implementation requirements and the full final gate are satisfied. Canonical checkbox updates remain parent-owned.

### Decision Coverage D-01–D-11

| Decision | Source/evidence conclusion |
| --- | --- |
| D-01 | Existing BlockIndex keyspace, additive v1 keys, schema 2 unchanged, schema-1 confirmation migration test; no crate/dependency manifest change. |
| D-02 | Canonical bounded byte/envelope/key/hash/header/parent validation, idempotent full equality and conflicting immutable rewrite refusal. |
| D-03 | Immutable hash identity separated from saved active projection/checkpoint; hidden suffix/fork rows never erased or promoted. |
| D-04 | VerifiedChainstateFence from recovered B and complete compatible metadata; no numeric/status authority. |
| D-05 | Actual ahead/fork restart chooses verified common saved prefix; missing/inconsistent authority refuses without repairing source history. |
| D-06 | Real same-database SyncAll state/protection batches, pre/post software fault seams, live publisher poisoned on uncertainty. |
| D-07 | initialize calls guard after coins recovery and before resume_prune_intent/readiness/cache. |
| D-08 | Reserved identity, saved-pair validation, stronger protection retained, direct 0/1/max intent predicate and no recreation on refusal. |
| D-09 | Actual DurableSyncRuntime reopen consumes initialize; safe absent-index and already-indexed controls preserve legacy behavior. |
| D-10 | Real drop/reopen matrix, actual staged spend/fork, concrete coins/meta failures and deterministic 256-record work proof. |
| D-11 | Pure decisions and thin storage/startup adapters, AAA behavior assertions, registered breadcrumbs and passed full native contract including zero uncovered pure-core lines. |

### Anti-Patterns and Disconfirmation Pass

Thirty changed source/test files were scanned. No blocker, placeholder, empty user-visible success, disconnected required consumer or swallowed new storage error was found.

| File / line | Observation | Severity / implication |
| --- | --- | --- |
| storage/filter_index.rs:58; filters.rs:161; publication.rs:73, :115 | Four documented future-scope dead_code allowances | INFO — generation/activation/catch-up/serving have tests but public runtime consumers belong to Phases 157/159–160. The phase-required scanner/publisher/guard are wired. |
| core recovery.rs:116 | Empty successful match arm | INFO — guard fallthrough, followed by authority and projection checks; no empty implementation. |
| check-phase155-filter-index.ts:192 | Success console output | INFO — CLI reports evaluated failures/success after substantive checks, not a log-only handler. |
| Existing prune module allowances / coins.rs success arm | Pre-existing adapter patterns | INFO — no new stub or phase safety bypass. |

Inversion checks sought row-only cursor promotion, unsafe rewind protection and late startup guarding. Each has concrete contrary source and real failure/control assertions. The disconfirmation pass also found limits that must remain explicit: the 256-row read-count test checks the complete record/projection scanner rather than measuring total startup time/memory; the synthetic maximum-height intent tests the direct refusal boundary rather than validating a physically present maximum-height chain; sparse height-400 fixtures prove deletion ordering rather than consensus. Genuine staged spend/fork fixtures and concrete persistence faults supply the separate provenance/authority evidence. Hardware/torn-media errors are not simulated; real backend failures propagate fail-closed and software pre/post-commit uncertainty is tested. These distinctions are documented, and none is an unmet Phase 155 criterion.

### Later-Phase Scope Review

Full current roadmap was checked. Phase 156 explicitly owns manual/automatic deletion enforcement, reserved operator CRUD and disable/re-enable; Phase 157 activation and scheduled catch-up; Phase 158 runtime reorg orchestration; Phases 159–162 RPC, peers, operator consumers and integrated retained-client proof. No Phase 155 failed truth was moved into these phases. No structured deferred gap is needed: those product capabilities were never Phase 155 must-haves. Full native evidence cannot be deferred.

### Human Verification Required

None for this internal local-storage phase. Actual production reopen/deletion/refusal behaviors are hermetically checkable and have concrete assertions. No visual UI, external-service integration or real-time product flow was introduced. Archive-scale performance, hardware power loss, public-mainnet operation and production-funds safety are excluded claims, not implied acceptance tests.

### Provenance, Security and Gaps Summary

Formal CONTEXT, all four PLANs and all four SUMMARYs share `lifecycle_mode: yolo` and `phase_lifecycle_id: 155-2026-10-04T16-01-08`, with expected generators and compliant chronology; this final report is generated through parent-invoked gsd-execute-phase after all plans/summaries and final security closure. No direct-fallback artifact or override exists. `gsd-tools verify lifecycle 155 --require-summaries --require-verification` passed for the final report with `valid=true` and no reasons.

REVIEW is clean for 36 reviewed files; no implementation source changed after that review or the full native pass. SECURITY is secured with 17/17 declared threats closed, zero open and native_gate passed. The full native result is corroborated by its terminal log and timestamped successful timing record; the Bun release checksum correspondence was verified by the parent. The only intervening PLAN change normalized YAML exports syntax, and its artifact check passed. No gap remains and no follow-up implementation plan or human gate is required for Phase 155. Existing provenance, linear-work, runtime-serialization, sparse-fixture and software-fault limitations remain explicit and unchanged.

***

_Verified: 2026-10-04T19:23:50Z_
_Verifier: the agent (gsd-verifier), through gsd-execute-phase_
