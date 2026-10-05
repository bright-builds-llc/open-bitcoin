---
phase: 156-index-owned-manual-and-automatic-prune-coordination
plan: "07"
subsystem: verification
tags: [basic-filter, prune-protection, daemon, durability, compatibility, fjall]
requires:
  - phase: 156-06
    provides: Fresh protection-keyed automatic decisions and authoritative paired deletion
provides:
  - Actual offline Periodic and Always retention above the legal 550 MiB target
  - Production runtime legacy-prefix, enable fault, stale-work and exhaustion controls
  - Criterion-to-test manual, direct, resumed and authenticated ownership evidence
affects: [156-08, 157, 162]
tech-stack:
  added: []
  patterns: [atomically reserved fixture roots, actual encoded payload accounting, independent reopen assertions]
key-files:
  created:
    - packages/open-bitcoin-node/src/sync/tests/filter_index/prune_faults.rs
    - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/automatic_prune/index_protection.rs
  modified:
    - packages/open-bitcoin-node/src/sync/tests/filter_index.rs
    - packages/open-bitcoin-node/src/sync/tests/filter_index/prune_coordination.rs
    - packages/open-bitcoin-node/src/sync/tests/filter_index/lifecycle.rs
    - packages/open-bitcoin-node/src/sync/tests/filter_index/faults.rs
    - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/automatic_prune.rs
    - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/automatic_prune/fixtures.rs
    - docs/parity/source-breadcrumbs.json
key-decisions:
  - Large deletion fixtures establish codec-valid ancestry and historical correspondence; accepted engine spend provenance remains a separate small custom-maturity fixture.
  - Accumulate actual encoded body lengths and calibrated stored undo lengths, retaining all protected and nonactive bytes in the legal target total.
  - A failed reply must be followed by actual close/reopen observations of the committed lifecycle and checkpoint.
requirements-completed: []
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 156-2026-10-04T20-28-36
generated_at: 2026-10-05T01:54:46Z
commits: []
git_finalization: pending consolidated root commit after clean whole-phase verification and full native gate
duration: 76min from first recorded command, including root-owned backend repair hold
completed: 2026-10-05
---

# Phase 156 Plan 07: Runtime and Ordinary Retention Proof Summary

**Ordinary offline daemon retention protects every required input above 550 MiB, while real runtime reopen proves legacy ownership, lifecycle faults and safe deletion receipts.**

## Performance and Scope

- First recorded command: 2026-10-05T00:38:49.213Z.
- Tasks completed: 3/3; whole-phase finalization remains root-owned.
- Nine declared source/test/mapping paths changed; this summary is separate.
- Both active lesson files were read completely: 7,188 bytes, 2,397 conservative estimated tokens. No audit trigger applied.
- No commit, push, hook bypass, shared tracking/config update, requirement activation or further phase execution was performed.

## Accomplishments

- Added five runtime controls: the actual constructor materializes legacy owner absence with a nonempty saved prefix as durable Active generation zero; a second constructor preserves the resulting rows/state. Production host enable faults, max-generation Active/Disabled refusal, same-generation stale frontier and reopened foreign incarnation exercise both record and checkpoint writers.
- Strengthened existing record/checkpoint and disable/release fault cases with poisoned live-delete/work refusal, explicit lifecycle generation and unearned receipt/intent assertions. Successful reopened state reflects the committed boundary rather than assuming every error rolled back.
- Added distinct genesis/no-undo and height-one/undo direct and resumed-intent controls over the production runtime/store. Exact required-input diagnostics exclude hash/window/buffer confounders. The existing positive fenced-prefix manual case genuinely deletes safe pairs and retains the required suffix.
- Added one dense 1,002-position ordinary daemon fixture, with linked headers, coinbase-containing bodies, matching merkle roots and non-genesis undo. Its trusted host controller establishes Active Empty ownership before Periodic retention. Sync workers/listeners remain disabled; no peer prerequisite or public activation route is used.
- Reused the existing nonactive bulk buffer and real storage accounting. Generalized undo calibration to the actual stored height-one body, so existing empty-body sparse controls and the new coinbase fixture both accumulate the correct encoded bytes.
- Atomically reserved new fixture roots using create-directory/AlreadyExists retry. Preserved the debugger's writer allocator and all Plan 05/06 changes. The inherited metadata-fault sink now forwards its real inner protection snapshot, preserving later-error receipt evidence without an unsupported or fabricated ownership contract.

## Task Finalization Records

The strict wrapper replaces task/TDD/metadata commits with pending consolidated root finalization.

1. Task 1: runtime compatibility, lifecycle/fault, stale-writer and exhaustion evidence complete. These test-only extensions exercise existing production contracts; no artificial expected RED or missing production behavior is claimed. The first focused run's fixture expectation error is recorded below.
1. Task 2: legal-target ordinary daemon protection, same-second transitions, actual paired deletion and Disabled history-loss reopen/refusal complete; inherited ordinary daemon controls remain passing.
1. Task 3: manual/direct/resume positive and refusal controls, current authenticated ownership tests, provenance and simplification review complete.

## Verification

Pinned Bun 1.3.9 was selected and reprobed from `/tmp/open-bitcoin-bun-1.3.9/bun-darwin-aarch64`. Every ad-hoc Cargo invocation used `bun run scripts/command-timings.ts run --key <key> -- <command>`, one at a time with polling within 60 seconds. No Bazel command was run by this plan.

| Check                           | Timing key and exact Cargo command                                                                                                                                  | Result                                                                                                                                                                                  |
| ------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Initial runtime controls        | `phase156-prune-runtime-controls -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --lib sync::tests::filter_index::prune_faults`              | Exit 101: 4 passed, 1 fixture expectation failed, 1,117 filtered. Expected state was corrected to distinguish a committed fence refresh.                                                |
| Historical full fault selection | `phase156-prune-fault-matrix -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --lib filter_index`                                             | 101/102 returned before an inherited Fjall 3.1.4 destructor deadlock. Root authorized diagnostic-based SIGTERM of only the owned test PID; Cargo exit 101, child signal 15. Not a pass. |
| Root repair gate                | `phase156-fjall-filter-index-suite -- cargo test --locked --manifest-path packages/Cargo.toml -p open-bitcoin-node --lib filter_index`                              | Root repair agent passed 102/102, 1,021 filtered, 52.92s body, 53.097s overall, default parallelism. Source unchanged afterward.                                                        |
| Legal ordinary daemon           | `phase156-daemon-prune-protection -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --bin open-bitcoind index_protection -- --nocapture`        | Final 1/1, 42 filtered, 128.82s body, 322.084s overall. Initial compile exit 101 on two missing fixture imports; explicit consensus imports fixed it.                                   |
| Inherited daemon controls       | `phase156-daemon-prune-controls -- cargo test --locked --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --bin open-bitcoind automatic_prune_ -- --nocapture` | 5/5, 38 filtered, 19.03s body, 19.845s overall. Includes genuine no-index deletion/later-error/retry/reopen and offline selection/worker controls.                                      |
| Cross-boundary node             | `phase156-cross-boundary-node -- cargo test --locked --manifest-path packages/Cargo.toml -p open-bitcoin-node --lib prune`                                          | 129/129, 994 filtered, 45.83s body, 290.563s overall, default parallelism. Includes concrete/direct/resumed gates, maps, handle CRUD, automatic and runtime controls.                   |
| Authenticated RPC               | `phase156-cross-boundary-rpc -- cargo test --locked --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --lib prune`                                            | 38/38, 232 filtered, 9.67s body, 233.583s overall. Auth precedence, reserved refusal and ordinary persistent controls pass.                                                             |
| Source provenance               | `bun scripts/check-parity-breadcrumbs.ts --check`                                                                                                                   | Passed for 965 Rust files. Both new paths have mapped/in-source breadcrumbs and exact `git add -N` inventory only.                                                                      |
| Managed checks                  | `bun scripts/bright-builds-check.ts all`                                                                                                                            | Passed: 1,284 source files, repository active lesson structure, zero findings.                                                                                                          |
| Formatting and review           | Scoped `rustfmt --check --edition 2024 --config skip_children=true` on all eight owned Rust paths; `git diff --check` and owned diff/stub/caller review             | Passed. Root owns subsequent `cargo fmt --all` and full native validation.                                                                                                              |

The root-owned [shutdown repair evidence](156-FJALL-SHUTDOWN-FIX.md) records the preserved process sample, exact backend pin, 31 independent successful fault/reopen invocations and the full 102-test rerun. The pin selects official Fjall revision `aa30dca811399a201e0b9595da93a4582dcb2b57` (3.1.10 base plus shutdown fix), with required LSM 3.1.10. This executor changed no manifest, lockfile, dependency cache or backend code. The repair is not an adoption of every later upstream fsync/deletion fix or exhaustive data migration proof.

Compiler/cache enumeration and executable pre-harness waits are distinct from the sampled destructor deadlock. Commands stayed live while those boundaries progressed. No elapsed estimate or quiet polling interval was treated as a timeout, and no duplicate build, cache change or host setting change occurred.

## Actual Legal-Target Accounting and Receipts

| Measured value                               | Exact bytes/count                                |
| -------------------------------------------- | ------------------------------------------------ |
| Unchanged legal target                       | 576,716,800 bytes (550 MiB)                      |
| Initial retained logical values              | 578,571,326 bytes                                |
| Nonactive bulk pairs                         | 235 pairs, 578,358,970 bytes                     |
| Each bulk encoded body / stored default undo | 2,461,034 / 68 bytes                             |
| Dense active bodies                          | 1,002 values, 144,288 encoded bytes              |
| Dense non-genesis undo                       | 1,001 values at 68 encoded bytes                 |
| Genuine ordinary deleted pairs               | 714, heights 0 through 713                       |
| Independently accumulated deleted bytes      | 151,300 bytes                                    |
| Remaining retained logical values            | 578,420,026 bytes, still above the target        |
| Earned support                               | 1 successful batch, 714 heights, last height 713 |

The body lengths come from the real block codec. The default undo length is calibrated from a real stored height-one pair minus its encoded body, then accumulated separately for every applicable mate. The measured total is checked against those accumulated actual values; no production usage override, fake byte count, lowered target or giant decoded bulk chain is used.

At the future coins-write deadline, stalled Active Empty Periodic retention deletes nothing, does not force coins, retains every body/non-genesis undo and earns no marker/counter. Always independently forces the coins checkpoint while retaining all required inputs. Disable followed by re-enable restores the same covering range with generation two before the next Periodic application at the same injected second. A later explicit disable yields generation three and no effective reserved protection; the same-second ordinary Periodic turn genuinely deletes eligible heights 0..713 despite the previously reusable scan/timer.

Deletion returns the exact ordered hashes, removes each body/undo mate together, evicts sampled deleted cache entries and all deleted in-memory undo, preserves retained cache/undo at height 714, and decreases actual usage by exactly 151,300 bytes. Repeated Always earns no extra receipts. The nonactive bulk and trailing 288 active heights keep the target unattainable.

Every live store/runtime/handle clone is dropped before ordinary production reopen. Disabled generation three, its saved Empty checkpoint/fence, exact ownership and earned counters survive. Re-enable refuses missing required body before acquisition and preserves ownership/checkpoint/usage/support and coins/metadata; a second actual reopen preserves the same result. Daemon private intent bytes are not publicly exposed: direct exact intent assertions are in node controls, while this dependent daemon fixture proves successful recovery/readiness and durable receipts through its existing public callers.

## Fault and Compatibility Matrix

| Concrete boundary / control                                       | Independent observation                                                                                                                                                         |
| ----------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| BeforeRecords                                                     | No ahead row or checkpoint release; poisoned live work/delete refuse; old Active generation and protection survive actual reopen.                                               |
| BeforeCheckpoint / BeforeProtection                               | Atomic checkpoint/protection publication retains the old prefix and covering lock, payloads, no intent or earned support.                                                       |
| AfterCommit, record-only                                          | Ahead immutable row is recoverable while old cursor/visible projection/protection remain. Failed reply does not authorize release.                                              |
| AfterCommit, checkpoint                                           | Actual reopen observes newly committed rows/projection/checkpoint and covering protection together. Required payloads remain.                                                   |
| BeforeChainMeta / partial H/B controls                            | Actual coins replay/flush and incompatible metadata cannot mint a cursor; old filter prefix/rows/required payloads and unsafe intent are preserved with explicit refusal.       |
| BeforeDisable                                                     | Active generation zero and covering lock remain.                                                                                                                                |
| AfterDisable / BeforeRelease                                      | Disabled generation one with conservative retained lock and unchanged saved prefix; live work/delete poisoned until reopen.                                                     |
| AfterRelease                                                      | Disabled generation one, owned lock absent, retained saved history; safe retry is idempotent.                                                                                   |
| BeforeEnable                                                      | Disabled generation one, no new lock, unchanged saved prefix/fence and payloads.                                                                                                |
| AfterEnable / enable AfterCommit                                  | Actual reopen observes Active generation two, saved prefix with refreshed current coins fence and covering lock committed together.                                             |
| Legacy owner absence, nonempty prefix                             | Actual `DurableSyncRuntime::open` materializes Active generation zero without resetting saved rows/prefix; a second constructor preserves the exact resulting index snapshot.   |
| Stale frontier / old generation / foreign incarnation             | Record-only and checkpoint completions refuse, preserving current immutable/state/lock facts. Same-generation frontier and reopened incarnation have explicit runtime controls. |
| Generation exhaustion / missing required body or non-genesis undo | Trusted transitions refuse without owner wrap, prefix reset, acquisition or history mutation; actual reopen retains the valid state.                                            |

All injections are software commit/reply boundaries. They do not establish hardware, disk-controller or power-loss resilience. Corrupt/missing prefix and equal-height replacement-branch controls retain their existing explicit refusal/common-prefix semantics without claiming runtime reorg orchestration.

## Criterion-to-Test Evidence

| Roadmap criterion                                                                | Named production-caller evidence                                                                                                                                                                                                                                                                                                                                                                                                        |
| -------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Manual and ordinary automatic application enforce fresh protection               | `filter_index_preplanned_manual_owner_retains_inputs_until_genuine_fenced_release`; `filter_index_clone_change_after_application_snapshot_reaches_final_concrete_refusal`; `index_protection_ordinary_legal_target_retains_then_deletes_only_after_disable`; inherited `automatic_prune_real_stalled_and_failed_publication_keeps_required_mates_and_refuses_reuse`.                                                                    |
| Authenticated/operator/direct CRUD cannot weaken ownership                       | `prune_http_authentication_precedes_reserved_dispatch_and_parsing`; `prune_http_authenticated_reserved_mutations_refuse_all_lifecycle_states`; `prune_http_ordinary_authenticated_crud_persists_with_reserved_owner`; real store `prune_map_requires_exact_reserved_entry_and_preserves_unrelated_updates` and both stale cloned-map controls.                                                                                          |
| Recoverable rows plus fenced checkpoint own release                              | `filter_index_production_reopens_every_record_and_checkpoint_fault_boundary`; `filter_index_production_after_record_commit_error_retains_ahead_rows_without_cursor`; accepted-spend ahead/H/B/metadata/fork controls; positive manual prefix deletion; `filter_index_runtime_direct_and_resume_required_genesis_and_height_one_refuse`; inherited direct forged-tuple/current-coins/branch and positive same-branch extension controls. |
| Disable invalidates work before release; re-enable acquires before dependent use | `filter_index_production_host_disable_enable_rejects_old_work_and_preserves_history`; `filter_index_production_host_disable_faults_recover_valid_retained_lifecycle`; `filter_index_runtime_enable_faults_reopen_only_committed_ownership`; stale-frontier/foreign/exhaustion controls; same-second legal daemon reacquisition and missing-history refusal.                                                                             |

The daemon path has no direct scan counter; genuine same-second deletion after release and protected survivors prove the ordinary decision behavior. Plan 06 separately measures exact scan counts with its real-backed/counting tests. Above-target daemon evidence demonstrates a stalled Empty index; failed-publication/poisoned-owner safety has separate real store/runtime proof rather than an invented public daemon fault seam.

## Evidence Limits and Threat Mitigations

The dense 1,002-height daemon fixture and 401-height sparse-payload deletion fixture are codec-valid ancestry/history-correspondence evidence. They are not consensus-accepted chains. `ValidatedHistory` separately stages and commits three contiguous blocks in the actual engine, including chained spends and body-bound undo, with explicit coinbase maturity one and synthetic supplied chain work. Those small accepted-engine controls establish spend/recovery provenance under their stated parameters, not full ordinary-daemon chain acceptance or network consensus history.

No public BASIC activation/options, catch-up scheduler, worker join/stop proof, runtime reorg, filter/index RPC, P2P serving, operator projection or complete client-after-prune product is claimed. HTTP tests exercise the existing authenticated ownership boundary, not a newly exposed filter product. Root owns the remaining full native and phase/lifecycle evidence gates.

| Threat   | Concrete evidence                                                                                                                                                                  |
| -------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| T-156-22 | Exact fault-mode/generation/checkpoint/protection/payload/intent/receipt observations after real drop/reopen; failed reply versus committed proof distinguished.                   |
| T-156-23 | Actual offline `flush_cycle` Periodic/Always at unchanged 550 MiB, exact encoded totals, protected survivors, real paired receipts and target still unreachable.                   |
| T-156-24 | Authenticated reserved refusal with auth precedence/ordinary persistence, exact map/race controls, fresh manual/direct/resume gates and positive safe deletion.                    |
| T-156-25 | Explicit codec versus accepted-engine provenance, actual constructor/reopen versus live clone, direct counter/intent observation limits and software versus hardware fault limits. |

No unplanned production endpoint, auth path, schema or filesystem trust boundary was introduced by this plan. No unresolved HIGH finding was identified in the owned source review; independent whole-phase review/native gates remain root-owned.

## Decisions and Simplification Review

Repo-local AGENTS guidance, the Bright Builds sidecar, placeholder-only overrides and architecture/code-shape/testing/verification/Rust standards informed this work. One shared ownership model, publication guard and paired-delete owner remain. Test modules add no production capability, authority, scanner, dependency or accounting override. Helpers keep the daemon stages and assertion/reopen concerns bounded; one large real store reuses one bulk buffer. All source remains within declared ownership, preserving prior agents and the debugger fixture repair.

## Deviations and Issues Encountered

- Initial five-case runtime selection passed four and failed one fixture expectation: successful re-enable legitimately refreshes the saved fence from accepted height one to current height two while preserving checkpoint one. The before/after expectations were corrected; no production source fix was needed.
- The full fault selection exposed an inherited Fjall 3.1.4 shutdown deadlock at actual store drop. A two-second process sample showed Flume blocking send with no worker receiver; root authorized SIGTERM only after this evidence. The interrupted 101/102 run is retained as historical failure. Root independently repaired/pinned the backend, ran 31 exact fault/reopen cases and passed the full 102-test selection before returning build ownership. See the dedicated repair artifact for scope and limitations.
- Initial RPC daemon compilation found missing explicit consensus imports in the new fixture. Added the two imports before the successful run; no behavior or production API adjustment occurred.
- The existing metadata-fault fixture needed real snapshot forwarding after Plan 05's trait contract. This narrow owned test adapter adjustment preserves its earlier fault/receipt behavior; inherited five-case daemon verification passes.
- No known remaining scoped issue is deferred. Full native formatting/Clippy/build/test/doctest/coverage/Bazel/claim and lifecycle gates, generated LOC freshness, tracking and Git finalization remain root-owned.

## Known Stubs

None introduced. Intentional empty fixture coins/undo/projection facts are labeled test arrangements, not unwired production data. The scoped TODO/FIXME/placeholder/unimplemented scan found no added stub.

## Self-Check: PASSED

All nine declared paths and this summary exist. New files have registered source provenance. Final legal daemon 1/1, inherited daemon 5/5, node prune 129/129 and RPC prune 38/38 passed with nonzero counts; root repair's full filter-index 102/102 is independently recorded. Scoped source formatting, provenance, managed checks and whitespace review passed. This summary is checked/formatted with installed explicit GFM/frontmatter extensions and checked again before handoff. No commit-existence claim applies: commits are empty and consolidated root Git finalization is pending.
