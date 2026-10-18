---
phase: 165-shared-exact-work-authority
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 165-2026-10-14T16-09-59Z
generated_at: "2026-10-17T02:29:11Z"
status: in_progress
requirements-completed: []
requirements-contributed: [CWOR-01, CWOR-02, CWOR-03, CWOR-05]
---

# Phase 165 Acceptance Evidence

This record separates source/formula evidence, executed Rust behavior, local Open Bitcoin adapter refusal and whole-repository acceptance. Full native acceptance remains pending until an actual default verifier exit zero is recorded below. Requirement activation, independent final review/security, formal verification and normal-hook Git finalization remain parent-owned.

## Guidance and Scope

The executor loaded the full Plan 14, phase context/research/plan-check, predecessor summaries and current owner contracts. Repo-local `AGENTS.md`, the Bright Builds sidecar, placeholder-only overrides and architecture/code-shape/verification/testing/Rust/TypeScript guidance require pinned tools, functional-core boundaries, exact breadcrumbs, complete verification and preserved historical evidence. Both active lesson files were read completely: 5,230 global bytes plus 1,958 repository bytes, 7,188 combined bytes and 2,397 conservative estimated tokens. Parent orchestration owns initial sync/bootstrap and lesson-audit lifecycle.

The checkout began on `main` at base `d0104a997c0362bf65b81de1c8fec8ddc3012a1b`, with the entire phase implementation already uncommitted. Untracked source/tests belong to the review inventory. No commit-derived phase heuristic is used. This executor creates no worktree, stages/commits/pushes nothing, changes no hooks and does not mutate shared STATE/ROADMAP/REQUIREMENTS/config/todo.

All Bun/Cargo/check commands use the command-local pinned Bun 1.3.9 environment:

```bash
PATH='/Users/peterryszkiewicz/.bun/install/cache/@oven/bun-darwin-aarch64@1.3.9@@@1/bin:'"$PATH"
```

All ad-hoc Cargo/Bazel commands run through the existing command-timings cooperative lock with pinned Rust 1.94.1. Heavy commands are serialized. Tool yields and quiet output are not timeouts; actual session exits are recovered.

## Decision and Requirement Matrix

The listed tests are real registered behavioral owners, with predecessor execution counts attributed to their original summaries. This source matrix does not itself claim a fresh execution. Current preflight/native receipts appear separately below.

| Decision / requirement                   | Actual owner/functions                                                                                                          | Behavioral evidence owner / evidence class                                                                                                                                                                                        |
| ---------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| D-01 / CWOR-01                           | Existing consensus `ProofWork`; core `WorkAuthority`; raw imports remain numerical                                              | `chain_proof/tests/phase165_vectors.rs`; core `phase165_connected_work_ignores_maximum_caller_score` and `phase165_raw_snapshot_cannot_earn_parent_work`; fixed arithmetic plus live-owner behavior                               |
| D-02 / CWOR-01                           | `ProofWork::{wrapping_add,wrapping_sub,from_le_bytes,to_le_bytes,from_hex,to_hex}`, `block_proof`                               | Eleven fixed proof vectors, six sums, fourteen invalid compacts; high-limb ordering and endian/malformed tests; source-derived expectations independently reproduced with Bun BigInt                                              |
| D-03 / CWOR-01                           | Shared normalized compact decoder; `check_proof_of_work_with_params`; `check_block_header_with_params`                          | `crypto/tests/phase165_tests.rs`: normalized zero/sign/overflow, equivalent encodings, selected limit/hash equality and exceedance; actual Rust tests, not contextual-completeness qualification                                  |
| D-04 / CWOR-01                           | Fixed `phase165-work-vectors.json`; current checker corpus/pin controls                                                         | Plan 01 fixed expectations and Plan 12 real corpus mutations; arithmetic/source evidence only, no Knots daemon execution                                                                                                          |
| D-05 / CWOR-02                           | `HeaderStore::admit_header`, `maybe_proof_facts`, single `best_tip`                                                             | `phase165_shorter_greater_work_drives_tip_and_proof_projection`, `phase165_equal_work_preserves_real_admission_order`, taller-equal/import/raw-parent controls                                                                    |
| D-06 / CWOR-02/03                        | Checked-body registry `record_with_order`, `eligible_body_candidates`, Node retained-suffix query, `reconcile_best_chain_inner` | Pure FIFO/pending/prune/failure tests; Node `phase165_eligibility`; fifteen `phase165_candidates` cases covering opposing header/body order, missing-best side activation, shorter-harder and late parent                         |
| D-07 / CWOR-03                           | Core owner/generation stages, lower `authorize_direct_connect`, sealed `authorize_reorg` and `prepare_authorized_reorg`         | Core foreign/stale/raw/mechanical/rebind/preview controls; lower store spy and full Fjall identity preservation; Node `phase165_work_authority` tests and compile-fail privacy/exclusive-borrow examples                          |
| D-08 / CWOR-02/03                        | Immutable body linkage seals, separate sole-strong-owner/Weak epoch, sticky unknown cache provenance                            | `phase165_cached_order` plus Node freshness/history tests; strict-greater unknown candidate positive and unknown-equal refusal; no deterministic restart-tie qualification                                                        |
| D-09 / CWOR-05                           | Filesystem-only `open_ordinary`, `CompatibilityRefusal::{work_profile,work_decision,fmt}`                                       | All four selectors/six observations, unavailable-work checks, fixed suffix after guidance, zero backend-builder entries and closed physical manifests                                                                             |
| D-10 / CWOR-01/05                        | `snapshot_codec::work::{ClassifiedWorkRecord,decode_work_record,encode_work_record}`                                            | Sixteen focused codec cases / sixty-nine snapshot regressions in Plan 02; 512-byte closed shape, duplicate/malformed/legacy maximum-u128 and exact-byte controls; numerical classification only                                   |
| D-11 / CWOR-05                           | Existing daemon `open_runtime_store`, operator offline/default routes, actual normal daemon binary                              | Plan 10 six new physical/effect controls plus thirteen explicitly reused original cases; Plan 11 three factories, six CLI cases and one real normal-daemon case with thirty-two children; local Open Bitcoin refusal evidence     |
| D-12 / all four contributed requirements | Ordinary selectors remain gated; docs/current claim guards preserve future owners                                               | Current catalog/docs and real overclaim mutations; Phase 166 bootstrap/checkpoint/all-handle reopen/genesis BASIC, Phase 167 contextual completeness and Phase 168 actual pinned-Knots differential qualification remain deferred |

## Mutation Owners, Trust Boundaries and First Effects

| Entry / boundary                                                       | Required facts before first effect                                                                                                                       | First relevant effect / retained limit                                                                                              |
| ---------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------- |
| Raw `from_entries`, `seed_from_chain`, `record_position`; codec import | No imported number grants accepted ancestry, coins lineage or body order; accepted rows cannot be overwritten                                            | Raw record projection only; no live best tip/proof facts from imports                                                               |
| Direct/unbound or selected header insertion                            | Actual target/hash; accepted parent; selected store binds complete parameters; work derives from parent plus own proof                                   | Entry/accepted-receipt insertion; best changes strictly on greater work                                                             |
| P2P Headers / explicit fixture readmission                             | Canonical guard first; whole-message proof/continuity before contextual prefix; first parent must already connect                                        | Accepted valid prefix and validation identity publication; P2P cannot initialize an unknown root                                    |
| Body ingress and cached readmission                                    | Actual selected accepted headers and bytes; internally derived contextual prerequisites                                                                  | Checked retention/linkage; FIFO sequence only after transaction-linked ancestry; cached order remains unknown                       |
| Core direct connect/stage                                              | Private actual live parent/params; genuine empty origin differs from raw empty; caller scores ignored                                                    | Staged overlay is local; actual cache absorption requires matching private owner/tip/generation                                     |
| Local managed connect                                                  | Canonical/body guard, actual source/parent/proof preflight before retention, then checked-body authorization                                             | Candidate intake precedes pure stage/history/BASIC preparation only after preflight; mempool/publication follows sealed preparation |
| Stored managed connect                                                 | Canonical guard and actual source binding before intake; duplicate/nonextending cache dispositions do not grant coins lineage                            | Checked retention; connected path requires sealed body/source grant and derived stage position                                      |
| Network reorg / direct lower reorg                                     | Actual retained disconnect bytes/undo/ancestor, complete checked suffix, exact work and known tie history; public lower unqualified route always refuses | Exact checked intake only after authorization; lower preparation then history/BASIC; no weak/MAX/cross-owner/stale bypass           |
| Preview / commit / absorption                                          | Recheck candidate/source/epoch and actual stage binding; preview guard precedes BASIC freeze                                                             | One own preview only; exact sealed continuation commits once; unrelated mutation/clone/raw preview cannot refresh authority         |
| Consuming core/managed fixture transfer                                | Genuine live source, complete effective coins/B/heads, all active/undo/count facts, actual physical store identity under serialization                   | Assign target only after complete equality; fresh owner/generation; no ordinary reopen upgrade                                      |
| Fixture missing-count enrichment / early BASIC attachment              | Real flush, actual validated prefix, strict physical equality; missing genuine counts only; same fresh live owner/append proof                           | Narrow cfg-test writes/attachment; present conflicts/raw/unflushed/suspended owner refuse; no restored missing physical inputs      |
| Sync branch selection / request projection                             | Every retained eligible branch; full selected-parameter proof facts for both best and side requests                                                      | Existing network authorization; only typed consensus causes permit fallback; local/storage/stale/BASIC errors are fatal             |
| Accepted prefix / failed body                                          | Preserve exact nested consensus identity; requery eligibility after invalidation                                                                         | Actual connected-prefix progress ends current turn; queue only accepted final body and reacquire ancestry next turn                 |
| Scheduled BASIC                                                        | Invalidated saved target must match actual active core identity under runtime authority                                                                  | Refuse before proof/store work on mismatch; raw matching read-only recovery still grants no work authority                          |
| Ordinary backend/openers                                               | Bounded filesystem shape only, all observations refuse with unavailable work                                                                             | No Fjall open/builder/recovery/migration/init/prune-resume/BASIC effect; adapters propagate same fixed redacted diagnostic          |

## Historical Identity and Review Attribution

The exact Knots source pin remains `29.3.knots20260210`, commit `a9aee730466ac67d35a3c03ee24676be5e045878`. Original Phase 162/163 receipts/manifests/raw-tooling identities remain immutable. Current source contracts are independent of those historical outcomes.

The original protected Node preservation source is Git blob `0345bab1bfbd76be3078453a6b9201fdf73941cc`, 20,556 bytes, SHA-256 `3d004f91bd72a0f3902588a36fca0438b04b5ac034bc9e227893664496835630`. The current mechanically migrated source is separately attributed at 20,688 bytes, SHA-256 `e6cb5740cb5c83655aed14c43b1206847d264431d944741a365771c3e8d76f5b`; Plan 14 must not edit it. The source blob is not a task commit.

Plan 12's six exact verifier additions preserve every historical byte through the narrow mechanical view; the older mandatory current guard independently requires all six. Current reviewed source renewal does not permit dropping edges, broad allowlists or a historical rebaseline. Plan 13 records eighty current owners, twenty-six executable test owners, 213 ordered fixed inputs checked before reads, 131 older fixed Rust inputs/452 edges, 216 stable combined input/checker hashes, 1,198 breadcrumbs and thirty-eight new mapped Rust paths. Its actual 843/0 matrix is a predecessor receipt; Plan 14 native verification remains separate.

The preliminary independent core review is clean within its exact thirty-eight-file scope; the preliminary Node review is clean within forty-five files. Both are source-only, with per-file SHA-256s, and perform no tests/builds or phase acceptance. Plan 14 rehashed all eighty-three review rows with no mismatch before any proposed fixture-assembly implementation. Final complete independent review/security and formal goal verification remain parent-owned.

## Executed Evidence

Recorded direct preflight began at 2026-10-17T02:25:01Z. All logs below are ignored under `.local/open-bitcoin-dev/` and timing-wrapper receipts retain exact command arguments, exits and elapsed durations.

| Command key / action                                   | Actual outcome / session / log                                                                                                                                                                                                                                                                                                           |
| ------------------------------------------------------ | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `phase165-plan14-workspace-fmt`                        | `cargo fmt --manifest-path packages/Cargo.toml --all`: exit zero; session 28269; `phase165-plan14-preflight-fmt.log`                                                                                                                                                                                                                     |
| `phase165-plan14-workspace-clippy`                     | Strict workspace/all-target/all-feature clippy: exit zero; session 55846; compiler 13.20s; `phase165-plan14-preflight-clippy.log`                                                                                                                                                                                                        |
| `phase165-plan14-workspace-build`                      | Workspace/all-target/all-feature build: exit zero; session 91625; compiler 20.97s; `phase165-plan14-preflight-build.log`                                                                                                                                                                                                                 |
| `phase165-plan14-workspace-test`                       | Exact workspace/all-feature test command: exit 101; benchmark thirteen passed/three failed; `phase165-plan14-preflight-test.log`                                                                                                                                                                                                         |
| `phase165-plan14-bench-regression` first repair        | Fourteen passed/two failed, exit 101; session 72844; `phase165-plan14-bench-regression.log`; pure mechanical snapshot and real admitted locator fixes let execution reach scripted sync headers                                                                                                                                          |
| `phase165-plan14-bench-smoke-diagnostic`               | Actual benchmark binary exits 2 with sync-runtime.headers-sync height assertion; session 73424; `phase165-plan14-bench-smoke-diagnostic.log`                                                                                                                                                                                             |
| `phase165-plan14-bench-regression` mined header repair | Fourteen passed/two failed, exit 101; session 78787; `phase165-plan14-bench-regression-mined.log`; actual mined headers alone do not initialize the required named fixture root                                                                                                                                                          |
| `phase165-plan14-workspace-diagnostic`                 | Authorized all-workspace/all-feature `--no-fail-fast` collection: **exit 101**, session 8464, wrapper PID 79830; **4,461 passed / 249 failed / six pre-existing ignored**, forty-one result groups, six failing targets; `phase165-plan14-preflight-diagnostic.log`; diagnostic variant does not replace the default native test command |
| Post-diagnostic current165 checker                     | Exit zero; eighty reviewed owners / twenty-six executable test owners; `phase165-plan14-current165.log`                                                                                                                                                                                                                                  |
| Post-diagnostic current164 checker                     | Exit zero; 131 fixed Rust inputs / 452 reviewed edges; `phase165-plan14-current164.log`                                                                                                                                                                                                                                                  |
| Post-diagnostic breadcrumbs / source stability         | Exit zero for 1,198 Rust files; all 216 Plan 13 saved input/checker SHA-256s still match; all thirty-eight core and forty-five Node preliminary-review rows match; `phase165-plan14-breadcrumbs.log`                                                                                                                                     |
| Whitespace / index                                     | `git diff --check` passed; index remains empty. These checks do not close the failing workspace/native gate.                                                                                                                                                                                                                             |

### Completed Diagnostic Receipt

The timing-wrapper receipt records start `2026-10-17T02:27:46.585Z`, end `2026-10-17T02:44:51.094Z`, **1,024,509ms (17m4.509s)**, exit 101 and no signal. The actual child session exit was recovered after every Cargo child completed. The Node child PID 81601 was observed with changing CPU time, active CPU or I/O state and progressive suite output throughout; wrapper heartbeats were not interpreted as timeouts. No suspension, termination, retry timeout or heavy overlap occurred.

| Failing target             | Actual result                                                                |
| -------------------------- | ---------------------------------------------------------------------------- |
| Benchmark library          | Fourteen passed / two failed; 1.77s                                          |
| CLI `operator_flows`       | Six passed / five failed; 3.10s                                              |
| Node library               | 1,690 passed / 181 failed / four pre-existing ignored; 853.11s               |
| RPC library                | 354 passed / thirty-two failed / one pre-existing ignored; 43.24s            |
| Normal daemon unit harness | Eighty-seven passed / twenty-eight failed / one pre-existing ignored; 42.20s |
| `black_box_parity`         | Two passed / one failed; 0.62s                                               |

All 249 panic cases were extracted into forty-six canonicalized panic-location clusters, preserving each test name, exact file/line, reason and bounded detail in ignored `phase165-plan14-failure-roots.json`. The full original log retains complete assertion output. Shared setup failures are not counted as 249 independently established product bugs.

| Shared panic owner / cluster                                               | Observed cases / root signal                                                                                                                                                                                                                                                                                                                                       |
| -------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Node sync `support_runtime.rs`                                             | Forty-four: exact confirmation-count identity, one-block coinbase maturity or already-bound header admission prerequisites. Repair owning genuine seed setups; never normalize the strict guard.                                                                                                                                                                   |
| Node sync `filter_index/phase162.rs`                                       | Twenty-six normal-maturity connects from raw runtime owners.                                                                                                                                                                                                                                                                                                       |
| Node readiness `tests.rs`                                                  | Nineteen: raw owner connection or `Missing { provenance: UnknownLegacy }` before the expected pending barrier. Raw historical metadata must not become ScriptsValid to fix this.                                                                                                                                                                                   |
| Node catch-up `rpc_faults.rs`                                              | Fourteen raw genesis/child setups.                                                                                                                                                                                                                                                                                                                                 |
| Node `accepted.rs` / production announcement transport                     | Eight cases each: raw managed origins, expected accepted facts or a requested-body setup prerequisite.                                                                                                                                                                                                                                                             |
| Node header checkpoint faults / peer transport phase162 / runtime timing   | Seven each: raw managed owners before acceptance.                                                                                                                                                                                                                                                                                                                  |
| RPC listener fixtures / daemon BASIC fixtures                              | Sixteen / fifteen raw runtime acceptance prerequisites.                                                                                                                                                                                                                                                                                                            |
| RPC inbound peer phase162 / HTTP BASIC fixtures / daemon phase162 fixtures | Eleven / five / five raw acceptance prerequisites.                                                                                                                                                                                                                                                                                                                 |
| CLI domain fixtures                                                        | Five raw runtime acceptance prerequisites; later same-height ordering and BASIC lifecycle remain separate integration obligations.                                                                                                                                                                                                                                 |
| Remaining smaller clusters                                                 | Actual lower unqualified reorg refusal, raw origin in branch/query/transport/fencing/evidence helpers, header admission/root setup, two genuine branch preference premises, two unmined automatic-prune header setups, expected progress/classification and black-box tip establishment. Exact forty-six-owner inventory and every case remain in the JSON ledger. |

Diagnostic log SHA-256: `1b35b73cf31cde9d4bcfeafe9944378a216aaa2afdf79b9d8fe7d7abf0aef00d`. Final failure-root ledger SHA-256: `c4da4acfa59a0ff801080791dc459571eeb84011fe8bdb1bbb4dcd27a37e7d8f`. The derived detail renderer was corrected to trim leading blank lines before extracting the actual bounded panic block; counts, names, source locations and reasons were unchanged. These are discovery receipts, not successful native verification.

The companion `phase165-plan14-failure-owner-map.json` resolves every failed test to its actual Rust definition file, including all fifteen callers behind the forty-four shared `support_runtime.rs` panic cases; none is unresolved. Its SHA-256 is `4e29732194d643fdb8f4e7d59a51cf3c579e670ff76c9a1d7ce91475d5b944fc`.

The following bounded repair grouping is a planning handoff, not implementation authorization or executed verification. Every group stays sequential and reserves five paths for exact current164/current165 contract, test and fixed-input inventory maintenance; optional new breadcrumbs still keep each proposed plan below fifteen paths. Shared fixture repairs must preserve meaningful consumer assertions and source guards.

| Proposed group                             | Actual failure attribution                             | Maximum initial source / guard paths | Required focus                                                                                                                                           |
| ------------------------------------------ | ------------------------------------------------------ | ------------------------------------ | -------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Lower managed and BASIC owners             | Thirty-seven                                           | Eight / five                         | Actual live source for branch/transport/query/readiness/accepted/fencing fixtures; preserve pending/barrier/physical controls and lower reorg refusal.   |
| Catch-up, fault and header checkpoint      | Thirty-one                                             | Eight / five                         | Genuine setup before the existing intended failure boundary, retaining budget, fault, persistence and typed-error assertions.                            |
| Normal-maturity recipe adapters            | Fifty                                                  | Five / five                          | Actual unchanged shared recipe/body acceptance and scoped reopen observations; no shared recipe, historical receipt or corpus rewrite.                   |
| Shared peer fixture owners                 | Thirty-four, including twenty-nine shared-helper cases | Three / five                         | Genuine large-prefix validation, explicit read-only raw header admission, no re-admission of already bound owners and genuinely preferred fork premises. |
| Protocol fixture ingress                   | Fifteen                                                | Eight / five                         | Actual named roots/headers, meaningful inflight/peer/timestamp classifications and normal authorized lifecycle paths.                                    |
| Live event loops                           | Nineteen                                               | Four / five                          | Genuine local connection before announcement, compact-timeout, write-evidence and logging effects.                                                       |
| Progress and restart seeds                 | Twelve                                                 | Six / five                           | Genuine owning seed facts with strict complete identity, explicit raw reopen limits and preserved useful-work evidence.                                  |
| RPC and daemon fixtures                    | Forty-four                                             | Eight / five                         | Actual fixture acceptance before HTTP/listener/filter/prune effects, mined headers and actual black-box tip establishment.                               |
| Separate benchmark / CLI integration plans | Seven                                                  | Parent planner-owned                 | Explicit same-live durable fixture assembly, preserved real early body order and all-handle read-only observations.                                      |

These rows account for all 249 discovered failures. Exact source-file inventories and suggested timed test filters were sent to the parent; the case/owner JSON retains independently inspectable attribution. Any failure that remains after its shared setup is fixed requires a new concrete diagnosis and exact scope, rather than a broader raw-authority fallback.

## Historical Integration Diagnosis and Repair Handoff

The parent approved exact benchmark fixture scope before edits. `cases/chainstate.rs` now uses explicitly mechanical snapshot constructors solely for historical pure synthetic benchmark connect/reorg operations. It grants no managed or live authority. `cases/network.rs` admits actual already-mined fixture headers under selected parameters and preserves locator/roundtrip checks. `cases/sync_runtime.rs` uses actual mined blocks for the three scripted header rounds while retaining height/counter/persistence assertions. That diagnostic had not yet established the named-root prerequisite. Later checked same-live fixture plans resolve this setup without promoting raw owners or bypassing refusal.

These are integration prerequisite repairs caused by Phase 165's deliberate trust-boundary changes. Initial failures and each reached next failure remain visible above. No test was disabled, ignored, skipped or relabeled to manufacture success.

The first proposed post-open pure-core handoff amendment was independently rejected with four blockers and one warning before implementation. Its missing cross-crate method, same-live BASIC lineage, post-reopen custom-parameter/equal-order loss, refusal wake/read ordering, exact commands and oversized task required a replan. The failed amendment is retained in `165-14-REPLAN-CHECK.md`; no proposed public builder, helper exposure or source-guard renewal was implemented under it.

Source tracing established that an empty live core alone cannot create a BASIC owner: normal initialization requires real durable genesis/history and a current append proof; the recovered manager creates its lineage from that proof, while owner initialization returns without lineage. Public enable configures storage but does not attach a manager owner. Snapshot seeding invalidates validation coverage and cannot be a workaround. A checked duplicate genesis creates body linkage, but its duplicate disposition does not create an accepted active-preference binding. Later real connected blocks must earn their own bindings. These constraints and failure receipts formed the bounded sequential fixture handoff. Original diagnostic counts remain preserved; completed repairs and final native acceptance require separately attributed execution receipts.

## Coverage Scope and Completion Gate

Historical Plan 08 combined pure JSON remains literally 13,458/13,488 lines (99.77758%), with thirty residual aggregate lines: core apply one/overlay one plus twenty-eight network lines across nine paths. Plan 09 network-only JSON remains 10,093/10,121 (99.72335%), with eligibility 380/380 and selected projection covered. These partial receipts are not whole-crate 100% claims.

The actual native completion policy is `cargo llvm-cov --show-missing-lines --text` in `run_coverage_report`; an actual `^Uncovered Lines:` section blocks acceptance. Plan 14 will close actual native text misses without exclusions or meaningless generic-instantiation tests derived solely from JSON aggregates. Default native verification, benchmark smoke, Bazel smoke, managed checks, LOC freshness and final diff review remain pending.

## Simplification Pass and Retained Limits

The source audit retains one `ProofWork`, one accepted-header selector and existing checked-body/context/lifecycle owners. Immutable linkage event identity and mutable eligibility freshness express separate facts. Private capabilities bind existing origin/owner/tip/generation instead of adding score or ownership counters. Complete coins snapshots avoid hint-based reconstruction. The compatibility decision is derived from observation rather than stored as a second state. No new dependency, production Bitcoin library, parallel work engine, alternate selector or canonical opening capability is introduced.

Canonical ordinary support remains closed for all four selectors and networking stays default off. Phase 166 owns bootstrap, exact checkpoint/all-handle reopen and genesis BASIC; Phase 167 owns contextual completeness; Phase 168 owns actual pinned-daemon differential qualification. Local Open Bitcoin refusal children, source comparisons, arithmetic formulas and synthetic historical fixtures do not qualify those future outcomes.

## Accepted Repair Evidence Through Plan31

This dated subsection records accepted execution through Plan31 before the final
Plan14 native gate. It does not convert the earlier249-failure discovery receipt
into a successful workspace run. Plan30 and final native/review/formal outcomes
must be read from their actual later receipts before phase closure.

The public Plan28 normal100 post-close proof validates every actual prefix body,
derives logical Some metadata, consumes a strict Memory-to-same-Fjall rebind and
uses the public constructor before the first direct child extension. Complete
physical None counts and historical nonroot UNKNOWN order remain unchanged.
Plan32 preserves full daemon/coins consumer assertions. Plan31 preserves sparse
raw UntrustedWorkLineage/absent-cache controls, actual same-Fjall Dense0..1001 with
all1002 derived undos including root, independent per-height byte expectations,
positive cache0/1/713/714, complete Phase127 normal100 suffix/socket/RPC behavior
and both precise post-fault manifest-preserving negatives.

Plan31's actual full pruning module passed7/0, focused prefix refusals2/0 and
unfiltered black-box5/0; required fmt/Clippy/build passed. Its final current suites
passed1012/0 with1881 expectations and1934/0 with7179 expectations, followed by
both live checkers, breadcrumbs, managed checks and whitespace. All658 fixed
canonical/presence/length/raw entries and seven logs closed unchanged. These are
source/runtime/read-window scopes, not complete system/compiler or whole-phase
native qualification. See [Plan28](165-28-SUMMARY.md), [Plan32](165-32-SUMMARY.md)
and [Plan31](165-31-SUMMARY.md).

Current guard arrays at this Plan31 checkpoint are157 SOURCE,87 TEST,143 old fixed
Rust sources,606 old function edges and276 current inputs. They are a dated
checkpoint; any reviewed Plan30 delta must be recorded independently. The original
CFIX35 evidence iteration, immutable snapshots/manifests/recipes and original
receipts remain exact. Source identity review preparation over308 paths is not
canonical final code/security/formal acceptance.

## Accepted Plan30 Parent-Only Regression and Dated Guard Checkpoint

Plan 30 closed WR-01 with the original genuinely accepted core rather than a raw
snapshot replacement. The expectation-only RED kept the old setup and demanded
exact second-input MissingCoin; the actual result was UntrustedWorkLineage
(0 passed, 1 failed, exit 101 in 0.459 seconds). A normal closed CoinsFlush on the
same core/parent makes the original first coin parent-only with an empty overlay.
GREEN retains full original maturity-one parameters, P2SH, ordered inputs and
exact missing second txid-nine/vout-zero. It preserves zero cache occupancy,
spendable parent and unchanged snapshot/tip/coins-B observations.

The named GREEN passed 1/0 in 10.028 seconds. The full chainstate::tests module
passed 14/0 in 0.358 seconds (11 direct and 3 existing child tests); these overlap
and are not independent aggregates. Workspace format check, Node all-target/all-
feature strict Clippy and build passed. The actual focused selector passed 37/0
with 264 expectations: 26 new preparation controls, six enabled-owner controls
and five inventory controls. All nine focused/policy commands exited zero.

At the dated Plan30 checkpoint, arrays were 158 SOURCE, 88 TEST and 277 INPUT with inventory pin
9d2fb8f19b943757a968574d6997b65350d29a8a20e4d57b37f834d468f3ae8c.
The older fixed inventory remains 143 Rust sources and 606 edges. All prior 276
input paths retain relative order, and the static union/CLI remain 36/14 modules.
All 709 raw canonical/presence/length/hash entries and nine successful logs closed
unchanged; root independently accepted that window at 2026-10-17T21:12:59Z.
See [actual Plan30 evidence](165-30-SUMMARY.md), source/pre-pin review and the
ignored phase165-plan30-focused-closing-proof.json and
phase165-plan30-root-focused-acceptance.json receipts. The latter root acceptance
SHA is 6ee24e38ecb7e73b115a5f1e49a9e2b8f7020e0a4f2caad4dedcf214bd09aa29.

These are affected-scope checks. Plan 30 deliberately did not repeat full guard
matrices; the Plan31 matrix counts above remain their dated predecessor scope.
Plan14 must now run the actual full default native contract, including current
complete matrices, fresh pure-core coverage and Bazel. Final independent whole-
phase code/security/formal review, requirement activation and normal-hook Git
remain parent-owned. No final native result is recorded or implied here yet.

## First Full Default Native Attempt and Accepted Plan34 Handoff

The exact first default command, bash scripts/verify.sh with pinned Bun PATH and
normal repository timing/lock handling, ran at 2026-10-17T21:20:46.663Z and exited
1 at the Phase72 observability checker. The verifier recorded 4,163 ms before its
first failure; the repository verify-full wrapper recorded 4,679 ms. Earlier
hook/LOC/breadcrumb and Phase61–71 checks ran, but later native stages did not.
The immutable full log is .local/open-bitcoin-dev/phase165-final14-native.log,
SHA 0c021085f6a949259701fbcc27be80874e0f34a2a7ee072ed95a2dd164a416e1.
Its original result, timing receipt and source closure remain preserved; this
attempt is permanently a failed native receipt, not partial full acceptance.

Four live CLI/dashboard/RPC/metrics assertions already contained the exact
64-digit hexadecimal work value, while the Phase72 source matrix still demanded
decimal 840005. Both raw support/smoke fixture surfaces actually retain decimal
840005 and must stay independently checked. [Plan34](165-34-SUMMARY.md) addressed
only these source expectations and preserved all eleven original Phase73 tests,
every direct CLI check/diagnostic and all eight matrix rows.

The actual six new cases were RED 2 passed/4 failed, then GREEN 6/0. The complete
file was RED 13/4, then GREEN 17/0; direct Phase72 CLI changed from the original
four-diagnostic exit 1 to exit 0. Import was silent and invoked no verification
or source-corpus reads; module-loading I/O is separate. Expected hexadecimal work
is independently computed in the four live tests; the two raw tests compare full
available/unavailable rows. These selectors overlap and are not summed.

Source acceptance preceded new digest derivation: independent CLEAN review
165-PRE-REVIEW-INTEGRATION34.md SHA
800d33e3f018cce0cf363a20f1db2415c90e4e31d832a36644e9a8779cfee5ed,
and root source acceptance SHA
4b39fc0aa75e2c6066a11a6c47cdc1be606ce197e73c5862e0191c45c644bac6.
Only two ordinary TypeScript SOURCE rows append; there is no fake Rust TEST row.
Actual current arrays are 160 SOURCE, 88 TEST and 279 INPUT with inventory pin
80dca53e69a2f0c917ae78297108ee16ad1187f2afb6d46e255790585770f624.
Both complete pin-bearing files reverse to their pre34 bytes. All original
SOURCE/TEST prefixes, old164 arrays, input relative order and history stay exact.

The four reviewed owner lengths are 551/545/620/269 lines, all within 628.
Twelve actual proportional commands exited zero, including the four fresh
file-specific owner controls and a 17-case/21-expectation full Phase73 run.
All 4,829 declared canonical/presence/length/raw entries and twelve logs closed
unchanged; root acceptance SHA is
3b929009c731304a69b6c4701f50f2117770b11d9fb813a24234113680d7116e.
The top-level static entry union is 39, current CLI 14, and Phase72's source-corpus
boundary is 30 roots/133 files. These are explicit declared scopes, not an
all-child-process or compiler/system dependency graph.

All original Rust bytes, four-selector/default-off limits, CFIX35 evidence and
immutable historical receipts remain unchanged by Plan34. Plan31 and Plan30
counts above retain their dated checkpoint scopes. The second complete default
native invocation has not run at this documentation checkpoint; its actual
Rust/current full matrices/fresh eight-crate coverage/benchmark/Bazel results,
then code/security/formal review and normal-hook finalization remain separate.

## Second Full Native Failure and Accepted Source Repairs Through Plan 39

This dated checkpoint follows the accepted Plan 39 command closure. The second
entire default verifier ran at 2026-10-17T22:03:57.756585Z and exited 1 at Phase 73.
The verifier recorded 3,831 ms, the repository verify-full wrapper 4,300 ms and the
launcher 4,413 ms. Phase 72 and all 17 Phase 73 unit cases passed; the direct checker
then refused an obsolete coverage selector. The original log
.local/open-bitcoin-dev/phase165-final14-native-r2.log retains SHA256
6b892fd4d81d2c1cea6e81391a14e495854e0396424d73f96a6f04d3065754bc.
The immutable phase165-final14-native-r2-failure-proof.json records the failure
and unchanged 4,829-input window. Both R1 and R2 remain failed receipts; later Rust,
benchmark, fresh coverage and Bazel stages were not reached in either attempt.
The separate original 81-CLI source diagnostic remains 69 pass / 12 fail at its original
generation and is not relabeled by later repairs.

[Plan 35](165-35-SUMMARY.md) moved the exact 160-row source table through an existing
module edge, preserving its values, complete inverses and historical identities.
[Plan 36](165-36-SUMMARY.md), [Plan 37](165-37-SUMMARY.md) and
[Plan 38](165-38-SUMMARY.md) repaired current coverage/documentation/runtime,
durability/authorization and filter/prune/startup source seams respectively.
Their full-suite checkpoints passed 179 / 404 / 1,068 cases with 488 / 645 / 2,829 assertions
at their own accepted generations. These are dated source-policy runs, not
Rust execution or cumulative whole-phase coverage.

[Plan 39](165-39-SUMMARY.md) proves unconditional ordinary reorg refusal, actual
typed validation and genuine fixture/reopen helpers. It preserves the original
filter regressions, repairs one demonstrated ineffective mutation and keeps the
161 test file byte-exact. Its historical projections reconstruct complete
authenticated original sources; current 164/165 gates bind PASSED H/E generations
before imported classification. Original manifests, snapshots and receipts
remain exact. Complete H, E and Rust restorations keep their authenticated
original destinations; the current fixture-source adapter E stays byte-exact.

| Plan 39 final executed scope                       | Cases / assertions | Result |
| -------------------------------------------------- | ------------------ | ------ |
| Full159                                            | 654 /2051          | Exit0  |
| Full161                                            | 149 /180           | Exit0  |
| Historical projection                              | 118 /262           | Exit0  |
| Selected164, including 31 copied CLI cases         | 180 /858           | Exit0  |
| Selected165, with seven nonempty evidence families | 68 /295            | Exit0  |

All 16 separate final commands exited 0; overlapping selections are not summed.
Root authenticated the complete 5,431 canonical/presence/bytes/raw-SHA closure,
all 16 actual logs, case names and caps in
phase165-plan39-root-command-acceptance.json, SHA256
d20f22a64a9cef2b01b42e9475cf783fcd825acbead20446519d8caf474832a7.
These finite repository/provenance observations do not claim compiler or OS
dependency coverage. Actual Summary 39 validation and Root acceptance precede
application of this final Plan 14 documentation checkpoint.

The current inventory is 190 SOURCE owners, 88 TEST owners and 307 ordered fixed
inputs, with pin 625975bfe720b4d08d05429a31a0c2dd0d76cb044c0675617754ffa4cc1ce6bb.
The next fresh 81 prerequisite-free CLI receipt and entire default native R3
remain unexecuted here. Complete current matrices, workspace/CLI/bin/benchmark,
fresh eight-crate llvm coverage with no uncovered lines, Bazel/provenance and
whole-phase code/security/formal acceptance remain separate required gates.
Catalog status remains in_progress. All four ordinary selectors still refuse,
networking remains default off, and 166/167/168 ownership and normal-hook
parent-only Git finalization remain unchanged.

## Third Full Native Failure and Accepted Plan 40 Repair

This dated checkpoint follows [Plan 40](165-40-SUMMARY.md), its actual 101-command
closure and independently accepted Summary. Earlier checkpoint wording and counts
above retain their original dates and scopes.

The entire default native R3 ran from 2026-10-18T01:47:33.894426Z to
2026-10-18T02:01:47.176924Z and exited 1. Its complete Phase158 run passed 899 cases,
failed one and executed 2,112 assertions; the duplicated failure summary is not a
second failing case. The strict missing-root child could not load copied
`contracts.ts`, so it never reached its required bounded missing-evidence refusal.
The native verifier recorded 851,260 ms, its normal verify-full wrapper 852,071 ms
and its launcher 853,285 ms. The full immutable log is
`.local/open-bitcoin-dev/phase165-final14-r3-native.log`, SHA256
`68b9692be93c29920985d0d2d1410a55de13ef97e0db8d8ad57133da73211516`.
The result remains `d8daba73b000c5fb5ecfc2ee81178ace34644eb8ef2edd7e3a6f950e1a041fb8`;
Root's complete failure proof remains
`5089c818b8d515a44e504462979277abf050cb8a3294630a826a8822bb6bc663`.
All 5,550 frozen raw states closed unchanged. R1, R2 and R3 remain permanently
failed native receipts; later Rust, benchmark, fresh coverage and Bazel stages
were not reached. The R3 coverage observer recorded captured=false and supplies
no fresh native coverage claim.

R3's 81 source CLI executions all passed at that generation, but their registration
expression omitted four dotted release-boundary names. They remain a dated,
incomplete subset. Root's corrected actual run_step inventory is 87 registrations
and 85 prerequisite-free CLIs. The exact two exclusions are
`check-benchmark-report.ts` and `check-bazel-build-provenance.ts`; both remain
mandatory in the entire default native run. Actual argv preserve breadcrumbs
`--check` and the Phase73 `env -u OPEN_BITCOIN_PHASE73_REPO_ROOT` prefix. The
original 69-pass/12-fail diagnostic also retains its original generation.

Plan40 repairs complete bounded executable copying and explicit evidence-only
fixture membership without changing the strict missing-root assertion or any of
the original 900 reorg cases. The shared PASSED-generation binder and two complete
copier inverses precede imported classification. Unknown historical inputs remain
unchanged until explicit qualification refuses them. Original optional-loader
outcomes, all 31 older copied controls, historical manifests and complete H/E/Rust
restorations remain attributable. Independent source acceptance preceded inverse
and ordinary-data derivation; the final nine-owner review is
`165-PRE-REVIEW-INTEGRATION40-EVIDENCE-FINAL.md`, SHA256
`90ebda078ddbce0430a419411090f7343514e5d8dbb4e8f13b326fe32456228b`.

| Plan 40 final executed scope | Cases / assertions | Result |
| --- | --- | --- |
| Complete Phase158 | 900 / 2113 | Exit 0 |
| Complete Phase162 | 2620 / 9704 | Exit 0 |
| Complete native Phase163 test file | 160 / 1534 | Exit 0 |
| Complete historical file | 203 / 661 | Exit 0 |
| Selected164 | 169 / 953 | Exit 0 |
| Selected165 | 68 / 295 | Exit 0 |

All 101 separate final commands passed, including all 85 eligible source CLIs.
Overlapping selections are not summed and selected164/165 do not claim complete
matrices. The earlier Plan40 898-pass/two-failure optional-membership replay and
all failed observer, setup and RED receipts remain preserved separately.

Root authenticated the complete 7,693-state generation and all actual command
outputs in `phase165-tools40-final-batch-root-command-closure-acceptance.json`,
SHA256 `f7fd6b823360bb9b7714c0de061c46d93ce7ae7849b9d26cb54643d20656dc8b`.
The completion SHA is
`f9028bd4f59b86c6493e628afc732b801fe70338b0d2de19e8dc6c07ceccb4a5`;
Summary40 SHA is
`80e36efda7d8a5419d1bb7d85aa2a89b139d480355c76971bad10c59c11cf40b`,
with Root Summary acceptance
`57b22e913831a49db78e21e355d8ffb9e0990e89f36a7e25c2576fa0f9c8972a`.
These are finite repository/provenance observations, not an OS/compiler inventory.

The accepted current inventory is 192 SOURCE owners, 88 TEST owners and 307 ordered
fixed inputs. The ordered pin remains
`625975bfe720b4d08d05429a31a0c2dd0d76cb044c0675617754ffa4cc1ce6bb`.
The table host is `dcdbe558d3db1d7256682593f5b2273c80394ab558254f85037b2525a60ecb0b`;
the unchanged entry is
`ac44b9e9fb32ebb4545fc588d03963aeb6694cd00d1b270254728c2a2521a48f`.

The separately accepted R4 retention-helper smoke passed 22 synthetic cases; its
five retained payloads each contain 4,385 bytes. Those harmless fixture processes
and 120 captured artifacts qualify the instrumentation only. They are not native
coverage execution. After this documentation checkpoint, fresh R4 preflight must
execute all 85 eligible CLIs on its own frozen generation, followed by the ENTIRE
fresh default native contract with pinned Bun 1.3.9 and Rust 1.94.1. Actual native
exit, complete matrices, Rust/CLI/bin/benchmark, fresh eight-crate coverage with no
uncovered lines, Bazel/provenance and final code/security/formal/lifecycle review
remain required. Normal repository hooks and parent-owned strict-main Git
finalization remain mandatory. Catalog status stays in_progress. All four
ordinary selectors still refuse, networking remains default off, and no
Phase166/167/168 capability or qualification is claimed.

## Fourth Full Native Failure and Accepted Plan 41 Diagnostic Repair

This dated checkpoint follows [Plan 41](165-41-SUMMARY.md) and Root's accepted
complete regression/source-CLI closure. All earlier checkpoint prose, counts,
failed attempts and source identities above retain their original generations.

The entire default native R4 ran from 2026-10-18T07:03:13.800945Z to
2026-10-18T08:11:22.261340Z and exited 1 after 4,088,320 ms (68m8.320s).
The normal wrapper recorded 4,088,352 ms and the internal verifier 4,087,612 ms.
Its complete Phase165 suite recorded 1,965 passes, one failure, 7,351 assertions
and 1,966 cases in 1,229.50 seconds. The unchanged failing case was
"phase165 reviewed current158 history scripts/check-phase158-validated-reorg.test.ts".
The expected independent manifest/history finding was omitted after the shared
current copier-generation adapter had already refused the supplied mutation.
The earlier full Phase158/162/native163/main164 runs passed 900 / 2620 / 160 / 1179
cases; those results remain R4 receipts and do not turn its final exit into a pass.

The immutable R4 log is
`.local/open-bitcoin-dev/phase165-final14-r4-native-verify.log`, SHA256
`c6744da34caf7f5ea4631d8a7ed0facbf04174a89d22b52e79c2a3343123dba4`.
The outcome and result hashes are respectively
`5d9c9562b40aada7646bb9f6352fc5899a5a83fbc96fe15d8b79e4cae8beaf7f`
and `bd9914e6ef13487ce892712b2b53a593d0c3634f0007fcc44ca866e951d40d4d`.
Root's failed-R4 closure,
`318038ff3d124b14b78a3e7ff12e40043ff3b33dd6507411edc93102f57502b0`,
authenticated all 8,322 original inputs and closed 8,347 states, including actual
outputs and declared absences. R4 captured no fresh coverage: no qualified writer
FD was bound, and coverage text/binding outputs remained absent. Later Rust,
benchmark, fresh llvm coverage, Bazel and provenance stages were not reached.
The 22-case retention-helper smoke remains synthetic instrumentation evidence.
R1, R2, R3 and R4 all remain permanently failed native receipts.

R4's separate preflight did execute all 85 eligible source CLIs successfully
against its own frozen generation. Completion
`4f2985f459b8fd720568626ced8bedcef65ae29a40882eaf7b3dd001294f44fe`
records 87 actual registrations, the exact two generated-prerequisite exclusions
and no failures or unexecuted IDs. It supplies source-preflight evidence only;
benchmark-report and Bazel-provenance remain mandatory in the default verifier.

Plan41 removes only the conditional wrapper around the existing independent
historicalToolingView try/catch and renews the entrypoint's one existing ordinary
ownership digest. The shared PASSED-generation adapter, strict inverses, adapter
refusal catch, historyBound return and imported checkCurrentClaims gate remain
unchanged. Local checkWorkClaims still runs independently. Historical data
authentication cannot grant current classification after adapter refusal.
No test assertion, historical snapshot, manifest, H body, copier, descriptor,
Rust owner or registration changed.

Independent CODE review preceded exact application and Root SOURCE CLEAN
`8cdcde37088584f3851cff18389dcaca43532a40ed00417fa59d78e7feb4f548`.
The real stale-row CLI refused with exit 1 and its sole expected ownership
diagnostic; Root separately closed the structured CODE stage before ordinary
activation. Independent DATA review preceded Root's one-row approval and exact
application. The entrypoint now has SHA256
`cc8fb2ac578eb1a271e148d4d58ca54ac7ad40229669137ddc1bee6486b1302f`;
the table host has
`7263310cc12da65c08dcaa2980bf0eb5ed917a44a1864bc3523838accb8c8a27`.
All other 191 SOURCE rows, all 88 TEST rows and all 307 ordered INPUT paths remain
exact. Counts stay 192 SOURCE / 88 TEST / 307 INPUT and the ordered pin remains
`625975bfe720b4d08d05429a31a0c2dd0d76cb044c0675617754ffa4cc1ce6bb`.

| Plan 41 actual executed scope | Pass / fail | Assertions | Bun suite time |
| --- | --- | ---: | --- |
| Unchanged four-owner regression loop | 4 / 0; 1962 filtered | 96 | 6.24s |
| Entire Phase165 test file | 1966 / 0 | 7351 | 1222.90s |
| Entire Phase164 main test file | 1179 / 0 | 2787 | 605.23s |
| Entire historical test file | 203 / 0 | 661 | 46.69s |

These are separate scopes, not an aggregate count. Full matrix identities and
multiplicities remain unchanged. Direct164/165, breadcrumbs --check, managed
checks and whitespace checks also passed, followed by all 85 eligible source
CLIs in actual native order with their original wrappers and flags. All 94
commands exited zero. Completion
`612cdbb77eeaa3b4e369c9870c5f130518877a78aec4285cde00636803a55c51`
has no failures, unexecuted IDs or remaining future outputs. Root's closing audit
`dce27225bd10460c4e8aa00974a781effb6b17826eb5f08663efc103f64ccac7`
authenticated 8,978 canonical states and 1,723,968 complete historical observation
rows. This finite repository/provenance boundary does not inventory the OS or
compiler installation.

The actual Summary41 hash is
`132a0a5a541f513c7921ae1128bde25b305e4adcedbd81116d072c18e8f82a0d`.
Root Summary acceptance
`391fb4bdef3a43e4aa058ea1fe8ccc21dc5f042f8d04964179940ab13ce07ede`
accepts the bounded two-owner source repair and actual regression/CLI closure.
Its native_acceptance and phase_complete remain false; plan requirement IDs do
not establish global completion.

After the approved final documentation refresh and actual LOC generation, final14
still requires a new frozen 85-CLI preflight and the ENTIRE fresh default native
R5 on pinned Bun 1.3.9 and Rust 1.94.1. All required source matrices, Rust
format/lint/build/tests, CLI/bin/benchmark, actual fresh eight-crate coverage with
no uncovered lines, Bazel/provenance, canonical code/security/formal/lifecycle
reviews and normal-hook main-branch commit/push remain pending. Catalog status
stays in_progress. Phase165 remains incomplete and Phase166 is unstarted.
All four ordinary selectors still refuse, networking stays default off, and the
canonical bootstrap/reopen, contextual and actual pinned-daemon outcomes assigned
to Phases166–168 remain unclaimed.

## Accepted Full Default Native R5 and Supplemental Capture Closure

This dated checkpoint records the actual full default R5 execution after the
accepted Plan41 repair and the separate 85-of-85 source preflight. It preserves
every earlier diagnostic, R1–R4 failure, copied-loader failure and source generation
above. Process results and independent complete evidence acceptance are separate.

The actual command was Bash running scripts/verify.sh in full mode on pinned
Bun 1.3.9 and Rust 1.94.1. Native PID 87342 started at
2026-10-18T10:55:11.685402Z and exited 0 at 2026-10-18T12:33:05.263120Z.
Internal verifier time was 5,872,826 ms, the normal verify-full timing receipt
5,873,529 ms and the parent outcome 5,873,552 ms. Its complete log is
.local/open-bitcoin-dev/phase165-final14-r5b-native-verify.log, SHA256
2fd5e17b4456bd1ebed7cfee6b63cb1571bcbe730fab3c225e69ae9e922e0352;
the actual outcome is 5cb92ce07fae76bd636965d8691649d785e4d974fef0073414c4953a525a8f30.

All 76 single-file Bun run summaries recorded 11,130 pass occurrences and zero
failures. These are executed-run counts, not deduplicated unique behaviors.
Complete Phase158/162/native163/main164/main165 suites recorded
900 / 2620 / 160 / 1179 / 1966 passes respectively. All 59 Rust result occurrences
were successful; workspace and coverage repetitions are not summed as unique
tests. Six baseline ignored occurrences remain: one public-network opt-in, two
timing-only measurements and three bounded subprocess entrypoints. There is no
new Phase165 ignored test. Rust format/lint/build/test, operator binaries,
benchmark smoke/report validation, Bazel smoke/provenance and the native coverage
stage were reached by the actual full command.

The freshly retained llvm text report is 2,431,082 bytes, SHA256
44107b740baee0fe758c585da40dec865005174d85a09082a6005178ae59bcf3.
It has 161 annotated source-file sections across exactly the eight native recipe
crates and no line beginning Uncovered Lines:. Its text has no aggregate
percentage table, so no percentage is inferred. Binding
8ddb043b3d852fa98e7b2e99d3315b97af7d734c0c0dd94a0ce84ddb67a00260
and receipt 181d74ccfdd88ff6040a4b261d1b7a5917f5bc67fb48e53ec422a0b95ae981b0
record the actual child/writer lineage, completed writer, same whole buffer,
device/inode continuity and retained link count zero.

The original postprocessor validation remains false at SHA256
31b559aec8578c1c792ebf5b8ad814b10f5b9f15215862512ab5d85b9d81253d.
Its sole error was complete report/binding buffer identity mismatch. Independent
source/data diagnosis identified two tuple values serialized to JSON arrays:
the parsed capture receipt was compared directly with the tuple-bearing in-memory
object. All other identity operands passed, and the persisted result.capture
equals the complete persisted receipt. That serialization mismatch does not
change the actual native exit. The original executed tools and false validation
are retained. Root's independently reviewed data-only auditor then exited 0 and
accepted the complete actual evidence without rerunning the native command.
Closure d464b6d90fb8f1cef1ea583bf22e631f4019a3ca83ae1ba30f1e59a04231e5bd
checks all 9,504 input states before/after/current: 28,512 complete proof rows,
plus all ten actual outputs, native/timing outcomes, recorded FD lineage and
retained whole report. Separate Root acceptance
abc239023ef67348b228b68315755f3424fd633d673567d789c0f9b780798b68
accepts the actual full native run and fresh eight-crate report. The original
false validation remains unchanged; canonical JSON comparison resolves only its
tuple/list receipt mismatch. The audit does not import or execute the completed
native tools, helper, application, tests, Cargo or Bazel.

No source, H, copier, test, inverse or Rust owner changed after the actual
85-CLI preflight or during this documentation preparation. Final documentation
and actual LOC freshness, canonical code/security/formal/lifecycle acceptance,
global requirement and phase completion and the normal-hook main commit/push
remain Root-owned gates. Catalog status remains in_progress. All four ordinary
selectors still refuse; networking stays default off. Canonical bootstrap/reopen,
contextual completeness and actual pinned-daemon qualification remain assigned
to Phases166–168.

## Phase165 bounded acceptance (2026-10-18)

All41 plans are complete. The full default native run and independent capture audit support CWOR-01, CWOR-02, CWOR-03 and CWOR-05. Canonical [code review](165-REVIEW.md), [security verification](165-SECURITY.md) and [goal verification](165-VERIFICATION.md) record zero unresolved findings,184 closed threat dispositions and all146 plan truth entries plus the four mandatory roadmap criteria.

Only those four requirement IDs are closed here. CWOR-04 and ordinary canonical bootstrap/reopen remain Phase166 work; contextual completeness and actual pinned-daemon differential qualification remain with167–168. All four ordinary selectors still refuse and networking stays default off. This record precedes its own normal-hook Git transaction; repository history and its actual hook outcome establish saving and push.
