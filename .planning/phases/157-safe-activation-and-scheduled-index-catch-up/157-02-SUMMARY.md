---
phase: 157-safe-activation-and-scheduled-index-catch-up
plan: "02"
subsystem: storage
tags: [basic-filters, startup, history-preflight, fjall, rust]
requires:
  - phase: 155
    provides: Recovered coins fencing and immutable BASIC checkpoint inspection
  - phase: 156
    provides: Durable lifecycle generations and reserved prune ownership
provides:
  - Explicit configured index policy before initialized runtime exposure and prune resume
  - Non-mutating missing-history refusal for fresh, Active, Disabled and rewound checkpoints
  - Actual configured reopen fault controls distinguishing achieved durable effects
affects: [157-05, 157-06, 157-10, durable-startup]
tech-stack:
  added: []
  patterns: [read-only recovery inspection before publication, independent required-input and retention boundaries]
key-files:
  created:
    - packages/open-bitcoin-node/src/sync/tests/filter_index/startup/configured.rs
    - packages/open-bitcoin-node/src/sync/tests/filter_index/startup/configured/faults.rs
  modified:
    - packages/open-bitcoin-node/src/chainstate.rs
    - packages/open-bitcoin-node/src/chainstate/flush_lifecycle.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/filters/startup.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/filters/lifecycle.rs
    - packages/open-bitcoin-node/src/sync/open_runtime.rs
    - packages/open-bitcoin-node/src/sync/tests/filter_index/startup.rs
    - packages/open-bitcoin-node/src/sync/tests/restart_chainstate/persist_and_hydrate.rs
key-decisions:
  - Required history begins after the recovered validated checkpoint, independently of stronger retained locks.
  - Configured Enabled preflights before saved-owner materialization, reconciliation or protection acquisition.
  - Empty Enabled storage refuses validated genesis history required without fabricating an anchor.
  - Existing internal initialize and open wrappers explicitly retain PreserveSaved compatibility.
requirements-completed: []
requirements-addressed: [CFAC-02, CFAC-01]
generated_by: gsd-execute-phase
lifecycle_mode: yolo
phase_lifecycle_id: 157-2026-10-05T14-50-48
generated_at: 2026-10-05T16:20:14Z
commits: []
git_finalization: pending root whole-phase native verification and strict finalization
duration: 17min
completed: 2026-10-05
---

# Phase 157 Plan 02: Configured History Preflight Summary

**Configured durable startup validates the complete still-required BASIC history before index publication or prune resume, while preserving independently stronger ownership and existing internal startup behavior.**

## Performance

- First measured verification began 2026-10-05T16:02:54.661Z; final restart verification completed at 16:17:11.012Z, and summary creation completed at 16:20 UTC. The reported duration describes this verification/documentation window, not an independently instrumented total implementation duration.
- Tasks: 3/3 complete, including the root-approved configured fault-control task.
- Files: nine owned Rust source/test files plus this summary.
- All Cargo commands used the pinned Bun timing wrapper and the shared-target cooperative lock, ran serially, and were polled within 60 seconds. No cache enumeration, deletion, host security changes, network test or new dependency occurred.

## Accomplishments

- Added `BasicFilterStartupMode::{PreserveSaved, Disabled, Enabled}` and `initialize_configured` through the existing chainstate public registration. `DurableSyncRuntime::open_configured` and `open_with_configured_runtime_activation` pass the policy before constructing the cache-backed managed runtime. Existing internal `initialize`, `open`, block-relay and runtime-activation wrappers retain PreserveSaved.
- Enabled startup performs coins H/B recovery first, requires a recovered coins B/full compatible ancestry fence, and uses read-only saved immutable/projection/checkpoint inspection to derive a recoverable checkpoint. Its history preflight begins at that checkpoint's next required height, even for saved Active. Ahead immutable rows cannot advance this boundary.
- Preflight streams actual bodies and body-bound non-genesis historical undo through the authoritative tip. Genesis requires its body and does not require undo. Missing body and undo diagnostics distinguish the category and numeric height. Empty enabled storage explicitly refuses `validated genesis history required` without publishing a zero-hash or other invented anchor.
- Legacy owner materialization, checkpoint reconciliation, lifecycle generation changes, protection publication and resumed deletion occur only after required input validation passes. Stronger interrupted-disable retention survives without forcing reads of already indexed/pruned sources. Active idempotence is retained after inspection/preflight rather than before it.
- Disabled configured startup uses the existing generation-invalidation batch before releasing only reserved BASIC ownership, then resumes a legal intent. Saved records and checkpoint survive; missing indexing history does not prevent explicit disable. Injected publication errors stop startup before prune resume and actual reopen reports the effects already durably committed.

## Task Finalization

1. Task 1: read-only recovery inspection and actual required-history boundary implemented with behavioral RED/GREEN evidence.
1. Task 2: configured initialize/runtime policy and concrete production reopen safety controls implemented with RED/GREEN evidence.
1. Task 3: two configured lifecycle fault controls and the narrowly approved legacy initializer source-assertion migration implemented and verified.

No task, TDD or metadata commit was created and no staging or push occurred. The strict phase wrapper owns all finalization after whole-phase verification. Shared STATE, ROADMAP, REQUIREMENTS and config were preserved.

## Verification Evidence

All commands below used `PATH=/tmp/open-bitcoin-bun-1.3.9/bun-darwin-aarch64:$PATH bun run scripts/command-timings.ts run --key KEY -- cargo ...` with `--manifest-path packages/Cargo.toml -p open-bitcoin-node`.

| Boundary                            | Cargo arguments                                                                         | Observed result                                                                                                                                                                                                                      |
| ----------------------------------- | --------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Task 1 RED                          | `test --lib phase157_activation_`                                                       | Expected exit 101: 0/3 passed. Active and ahead-row activation skipped missing input; stronger retention incorrectly reread a pruned indexed body. Timing: 11.303s.                                                                  |
| Task 1 GREEN                        | `test --lib phase157_activation_`                                                       | 3/3 passed, 1,123 filtered, execution 3.20s; timing 10.701s.                                                                                                                                                                         |
| Task 2 RED                          | `test --lib phase157_activation_`                                                       | Exit 101: 3/9 passed. Four new configured-policy controls failed; two new intent-success fixtures additionally exposed the existing keep-window requirement and were corrected to height-400 legal-intent controls. Timing: 17.009s. |
| Task 2 GREEN                        | `test --lib phase157_activation_`                                                       | 9/9 passed, execution 10.27s; timing 16.771s.                                                                                                                                                                                        |
| Inherited filter index regression   | `test --lib filter_index`                                                               | 114/114 passed, 1,021 filtered, execution 66.32s; timing 73.800s. This run includes 12 new controls and predates the two added fault controls.                                                                                       |
| Final configured and fault controls | `test --lib phase157_activation_`                                                       | 14/14 passed, 1,123 filtered, execution 16.53s; timing 24.550s.                                                                                                                                                                      |
| Initializer regression              | `test --lib initialize`                                                                 | Final 23/23 passed, 1,114 filtered, execution 10.28s; timing 16.783s. Initial run exposed one stale literal-source assertion; 22 behavioral controls already passed.                                                                 |
| Restart regression                  | `test --lib restart_chainstate`                                                         | 10/10 passed, 1,127 filtered, execution 3.76s; timing 4.229s.                                                                                                                                                                        |
| Production/test diagnostics         | `clippy --all-targets --all-features -- -D warnings`                                    | Passed, timing 4.715s after fixing an introduced test-only import warning.                                                                                                                                                           |
| Owned source formatting and diff    | Scoped `rustfmt --check --edition 2024 --config skip_children=true`; `git diff --check` | Passed across all nine owned Rust paths.                                                                                                                                                                                             |

Counts overlap and must not be added as a distinct-test total. Root owns the full native formatter/build/test/coverage/Bazel gate, catalog registration and final security/source review.

## Reopen and Preservation Evidence

The complete raw BlockIndex and Chainstate key/value snapshots are compared only after all Fjall/runtime handles close. They include saved immutable/filter projection/state/owner/protection rows, live intent, block bodies, undo and chain metadata. Ordinary missing-history refusals leave both snapshots equal. Surviving intent body/undo values and the exact live intent are also independently decoded after reopen.

The matrix covers fresh, saved Active and saved Disabled missing body versus missing non-genesis undo; ahead immutable rows; a validated equal-height fork requiring checkpoint rewind; fresh missing genesis body versus proven no-genesis-undo success; and undo whose shape cannot describe the spending body. Separate controls use the concrete paired-delete adapter to actually remove required body/undo in fresh and Disabled stores, then prove unchanged refusal. The historical spend fixtures are genuinely staged and accepted with custom maturity 1, including historical and same-block spends; they are not public-mainnet provenance.

Successful prune-resume and persistence fault controls use explicitly codec-valid height-400 coinbase history so the selected intents are outside the existing keep window. They do not claim consensus validation. A safe indexed intent actually deletes its pair; a second configured reopen succeeds without rereading that indexed source. An interrupted-disable stronger lock from height zero is preserved while activation reads only the required validated suffix after a pruned indexed height.

| Injected boundary            | Durable reopen observation before retry                                                                                   |
| ---------------------------- | ------------------------------------------------------------------------------------------------------------------------- |
| BeforeDisable                | Original Active generation zero, original lock and every index/history row unchanged.                                     |
| AfterDisable / BeforeRelease | Disabled generation one, old conservative reserved lock retained, checkpoint/history/intent unchanged.                    |
| AfterRelease                 | Disabled generation one, reserved lock released and ordinary operator lock retained; checkpoint/history/intent unchanged. |
| BeforeEnable                 | Original Disabled generation one, no acquired reserved protection and exact rows unchanged.                               |
| AfterEnable                  | Complete Active generation two and covering protection committed together; saved checkpoint/history/intent unchanged.     |

Every injected error prevents resumed pair deletion. Retry completes the existing durable transition without extra generation churn and then finishes the legal intent. This is injected software persistence-boundary evidence, not a hardware power-loss simulation.

## Decisions and Simplification

Local AGENTS guidance, Bright Builds sidecar, placeholder-only overrides, architecture, code-shape, testing, verification, local-guidance and Rust standards informed the implementation. Both active lesson sources were read completely: 7,188 bytes and 2,397 conservatively estimated tokens; the existing audit baseline requires no new audit.

The simplification pass kept one saved checkpoint inspection shared by recovery and enable, one existing publication guard and one existing lifecycle batch. Required-input protection comes directly from the recovered checkpoint; independently stronger retention remains a separate fact. No duplicate index-owned history vector, scanner, acquisition/repair path, worker or deletion authority was added. Bounded scheduler turns belong to subsequent plans; this history-wide startup preflight is deliberately not a bounded-latency claim.

## Deviations and Issues Encountered

- Root approved the narrowly required `chainstate.rs` public re-export registration and added it to Plan 02 scope.
- Root added Task 3 and `configured/faults.rs` to preserve coherent tests while keeping `configured.rs` at 558 lines and the fault module at 174 lines.
- Root approved the exact source-assertion adjustment in `restart_chainstate/persist_and_hydrate.rs`. It now identifies `initialize_configured` and the lifecycle/cache tuple separately, preserving all existing behavioral assertions and avoiding an unnecessary duplicate initializer.
- Removing the production Active shortcut made the lifecycle enum import test-only. The first Clippy run exposed that introduced warning; a cfg(test) import corrected it and final Clippy passed.
- The inherited Phase 155 Bun guard still requires the old initializer literal/unconditional saved-only guard. Root explicitly assigned migration of that guard and independent configured/compatibility mutation controls to Plan 10. No historical checker was edited by this plan, and this summary does not claim those pending guard checks pass.

No architectural deviation, authentication gate or unresolved production defect was found. Full root finalization and phase-level evidence remain pending.

## Threat Mitigations and Known Limits

- T-157-04: complete required suffix preflight precedes configured index mutation and resumed deletion; exact real reopen snapshots prove refusal preservation.
- T-157-05: recovered B/metadata and the complete valid saved prefix derive the cursor; ahead records never shorten the required suffix and a rewind is inspected before effects.
- T-157-06: one historical body/undo/input bundle is processed at a time, typed historical validation binds undo to the body, and missing-input diagnostics expose only category and height.
- T-157-07: absent validated genesis refuses before index publication; explicit disable invalidates generations before reserved release and retains ordinary locks.

No new endpoint, authentication path, schema format or external filesystem trust boundary was introduced. No stub prevents the plan goal. Existing independently required coins recovery may have effects before configured preflight; the ordinary missing-history snapshot controls use already consistent coins authority and claim exact nonmutation at that boundary.

CFAC-02 and CFAC-01 are addressed only at this configured node entrypoint. Actual daemon option wiring, bounded replay/scheduling and continuous ordinary-connect proof are later Phase 157 work. No requirement checkbox or whole-phase completion claim is activated. Plan 10 owns new-path source breadcrumb catalog registration; both new files already contain exact pinned source comments. Contributor status/README/UAT updates remain in the declared closeout plan.

## Next Plan Readiness

Later daemon startup should call `open_with_configured_runtime_activation(..., BasicFilterStartupMode::Disabled/Enabled)` explicitly; PreserveSaved is solely internal compatibility. This constructor establishes policy before ownership and prune resume but does not itself start a network listener or scheduled catch-up worker. Root can proceed with the remaining plans and eventual full native verification.

## Self-Check: PASSED

All nine declared Rust files and this summary exist. Nonzero final configured/fault, inherited filter-index, initialize and restart controls passed with the exact counts above; scoped source formatting, diff whitespace and production/test Clippy checks passed. Stub and threat-surface scans found no blocking stub or unplanned trust boundary. The summary contains exactly two standalone frontmatter delimiters. Markdown was checked before scoped formatting with the installed GFM/frontmatter extensions; the final check passed. No commit-existence claim applies because `commits: []` and root strict finalization are pending. Shared execution state and requirement checkboxes remain root-owned.
