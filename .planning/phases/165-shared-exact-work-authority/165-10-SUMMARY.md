---
phase: 165-shared-exact-work-authority
plan: "10"
subsystem: storage-compatibility-refusal
tags: [proof-work, compatibility, preservation, backend-spy, closed-manifests]
requires:
  - phase: 165-09
    provides: Earned work authority and guarded sync without ordinary canonical admission
provides:
  - Typed work compatibility refusal separate from unavailable work observation
  - Fixed redacted work_decision diagnostic suffix
  - Closed legacy/raw/forged source preservation and actual owned-clone effect controls
affects: [165-11, 165-12, 165-14, 166]
tech-stack:
  added: []
  patterns:
    - Derive immutable refusal policy from bounded filesystem observation
    - Reuse unchanged original preservation assertions through an exact guarded include
    - Count actual raw backend boundary entries independently of refusal policy
key-files:
  created:
    - packages/open-bitcoin-node/src/storage/chain_compatibility/tests/phase165_work.rs
  modified:
    - packages/open-bitcoin-node/src/storage/chain_compatibility.rs
    - packages/open-bitcoin-node/src/storage/chain_compatibility/tests.rs
    - packages/open-bitcoin-node/src/storage/fjall_store.rs
    - scripts/check-phase164-canonical-identity.ts
    - scripts/check-phase164-canonical-identity/contracts.ts
    - scripts/check-phase164-canonical-identity.test.ts
    - docs/parity/source-breadcrumbs.json
key-decisions:
  - "ExistingUnproved is a refusal policy, never a positive identification of legacy work."
  - "Schema-one coins migration controls already contain exact untrusted fixture numbers; legacy sources are never converted for positive controls."
  - "Original preservation source and registration remain unchanged; thirteen original cases are duplicated explicitly through include reuse."
requirements-completed: []
requirements-contributed: [CWOR-05]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 165-2026-10-14T16-09-59Z
generated_at: 2026-10-15T07:06:37Z
duration: 19min
completed: 2026-10-15
tasks_completed: 2
files_modified: 8
---

# Phase 165 Plan 10: Explicit Work Compatibility Refusal Summary

**Filesystem-only ordinary refusal now reports a typed work decision while closed-source manifests and actual backend/migration/prune controls prove the pre-effect boundary.**

## Delivered Behavior and API Handoff

`storage::chain_compatibility::WorkCompatibilityDecision` is public through the existing module. `CompatibilityRefusal::work_decision()` derives its immutable decision from the observation:

| Observation                                         | Decision               | Fixed diagnostic code   |
| --------------------------------------------------- | ---------------------- | ----------------------- |
| Missing or empty directory                          | `BootstrapUnavailable` | `bootstrap_unavailable` |
| Existing unproved bytes                             | `ExistingUnproved`     | `existing_unproved`     |
| Unsafe path                                         | `UnsafePath`           | `unsafe_path`           |
| Inspection I/O failure, including permission denial | `InspectionIoFailure`  | `inspection_io_failure` |

`WorkProfileObservation::Unavailable` remains the only work observation. The decision never claims that layout, markers, JSON scores, or exact-looking records proved legacy or accepted work. Display and its existing string serialization append `; work_decision=<fixed code>` after the unchanged guidance. Existing reason ordering, selected network/genesis facts, missing prerequisites, redaction and bounded diagnostics remain intact. The new matrix exercises all four selectors and all six observations.

The ordinary opener and inspector are unchanged: ancestor metadata and at most the first directory entry, always refusal. No source-file contents, backend builder/open, coins/schema migration, recovery, initialization, BASIC writes or live authority creation were added. All four selector support/refusal classifications and default-off networking remain unchanged.

The test-only raw backend boundary now records an entry immediately before the original panic spy and actual `Database::builder` call. The shared ordinary-refusal helper checks zero counter increment for every real refused open. Independent positive calls prove the probe is live and successful raw opens reach the builder.

## Closed Sources and Effective Controls

The separate registered `phase165_work` module uses the exact `include!("preservation.rs");`. Its **19 discovered cases comprise six new cases and thirteen duplicated original controls**. The duplicates are reuse costs, not thirteen new independent proofs. The original preservation registration, helpers, physical-manifest assertions, sentinels and mutation controls were not edited.

New closed sources contain actual schema-one bare maximum-u128 and tagged legacy records, schema-two raw exact-looking synthetic fixture records, mixed exact/legacy records and malformed exact records. Both forged work-profile markers accompany those sources. Wallet/configuration bytes, private Unix modes and the existing unrelated file/directory/link and namespace sentinels are included in the closed manifests. Another source combines actual pending prune intent/body/undo, coins B and H, malformed recovery bytes and malformed BASIC rows. Original reused cases retain valid and malformed BASIC/history coverage.

Every source is closed before its recursive manifest is taken. The original manifest records names, entry types, bytes, lengths, link targets, modification times, readonly state and Unix modes; access times are intentionally excluded. All four selectors refuse with unavailable work and explicit unproved policy while manifests remain equal and backend counter increments remain zero. Unsafe regular-file ancestors, symlinks, trailing dots/slashes and injected permission/other I/O failures retain the same refusal/preservation boundary. Existing missing/empty physical controls remain active.

The new positive controls copy closed sources into distinct owned clone paths. On the migration clone, the actual raw opener creates durable coins B/C from the schema-one snapshot, and the exact untrusted fixture work number remains unchanged. Schema one here names the coins layout: this positive fixture already has exact untrusted numeric records. No legacy-u128 record is silently reinterpreted or replaced for this control. On the separate prune clone, actual raw reopen followed by the existing `resume_prune_intent` deletes both block and undo, clears the intent and records pruning. Both clone manifests change; the source manifests remain identical. Neither destructive positive operates on a preserved source.

## Historical Attribution

Independently checked Git blob `0345bab1bfbd76be3078453a6b9201fdf73941cc` contains the original **20,556-byte** Phase 164 preservation source with SHA-256 **`3d004f91bd72a0f3902588a36fca0438b04b5ac034bc9e227893664496835630`**. Its historical reports, tooling, corpora and receipts remain unchanged.

The current preservation source is **20,688 bytes**, SHA-256 **`e6cb5740cb5c83655aed14c43b1206847d264431d944741a365771c3e8d76f5b`**. Its diff from that blob is solely the earlier `ChainPosition::untrusted_fixture_work` constructor conversion and corresponding rustfmt layout. This plan did not edit it, and does not assign the historical hash to the changed current file.

## Executed RED/GREEN and Final Verification

Every Bun/Cargo command used the command-local pinned Bun 1.3.9 PATH. Every Cargo command used `bun run scripts/command-timings.ts run --key <stable-key> -- ...`, with pinned Rust and no overlapping Cargo jobs. Heavy native and complete Bun suites ran sequentially; no live test was suspended, interrupted or given a relaxed timeout.

| Check                                                                            | Actual result                                                                                                                                                                                    |
| -------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Task 1 diagnostic RED                                                            | One discovered case failed because `work_decision=` was absent.                                                                                                                                  |
| First Task 1 Cargo GREEN                                                         | 25 passed, zero failed/ignored; 13.06s.                                                                                                                                                          |
| First guard diagnostic                                                           | 653 passed / 1 failed: a new mutation anchor hit the older observation code instead of the new policy code. Replaced it with the unique bootstrap policy-code anchor.                            |
| Builder-probe compile diagnostic                                                 | Initial use of unstable `LocalKey<Cell>::update` failed compilation. Replaced it with stable `with`/`Cell::set`; this is not behavioral RED evidence.                                            |
| Task 2 actual builder-counter RED                                                | One discovered probe failed: with counting disabled, actual protected raw entry gave zero instead of one. The original spy also fired before directory creation. Restoring counting resolved it. |
| First broader Task 2 diagnostic                                                  | 73 passed / 1 failed: the new fixture addressed JSON positions outside the versioned `payload` envelope. Corrected fixture construction.                                                         |
| Focused module GREEN                                                             | 19/19 passed, zero failed/ignored; 17.57s, before the final explicit legacy-version/private-mode refinements.                                                                                    |
| New loaded-source controls                                                       | 10/10 passed; actual anchors required. Includes work-policy/diagnostic changes and removal, comments, redirect and disabled registration/include cases.                                          |
| Final Task 1 declared Cargo command, `phase165-compatibility-work`               | 44/44 passed, zero failed/ignored; 28.05s. Includes the thirteen explicitly duplicated controls.                                                                                                 |
| Final complete current-source Bun guard                                          | **670/670 passed**, zero failed, 874 expectations; 252.70s.                                                                                                                                      |
| Task 1 declared current boundary CLI                                             | Passed: 127 fixed Rust inputs / 423 reviewed source edges.                                                                                                                                       |
| Final Task 2 declared broad Cargo command, `phase165-compatibility-preservation` | **74/74 passed**, zero failed/ignored; 17.82s. Includes 19 module cases and 55 existing matching work cases elsewhere.                                                                           |
| Task 2 declared current boundary CLI                                             | Passed: 127 fixed Rust inputs / 423 reviewed source edges.                                                                                                                                       |
| Workspace formatting                                                             | `cargo fmt --all --check` passed.                                                                                                                                                                |
| Scoped node lint                                                                 | All-target/all-feature clippy with `-D warnings` passed; 10.32s.                                                                                                                                 |
| Scoped node build                                                                | All targets/all features passed; 8.92s.                                                                                                                                                          |
| Managed checks                                                                   | 1,521 scanned inputs, zero findings/exemptions. New untracked Rust file also explicitly measured below the cap.                                                                                  |
| Breadcrumbs                                                                      | Passed for 1,195 Rust files.                                                                                                                                                                     |
| Diff whitespace / stub scan                                                      | Passed; no TODO/FIXME/placeholder occurrences in owned implementation/test/checker paths.                                                                                                        |

The two task command groups stopped on their actual earlier nonzero results and were subsequently repaired and verified. Per the parent's instruction against redundant verification, **one actual final stable-source full guard result serves both task gates**; no second identical guard run is claimed. All **158 loaded guard input SHA-256s** were saved after that guard and verified unchanged after the subsequent broad Cargo run. The ignored artifacts are `.local/open-bitcoin-dev/phase165-plan10-task1-final-guard.log` and `phase165-plan10-stable-input-hashes.json`; the earlier failed guard log remains separately attributable.

No pure-core crate changed in this plan. The complete new policy projection/code branches are exercised by the exhaustive selector/observation matrix. No whole-crate or whole-phase coverage percentage is inferred from these focused shell tests; parent Plan 14 retains full native/coverage finalization.

## Deviations and Scope

The parent approved narrow additional ownership of `scripts/check-phase164-canonical-identity.ts` to enforce the exact executable new module path and exact original-source include. Those guards are needed because top-level include/registration changes are outside the existing function-region digest model. Existing guard logic and historical tooling were retained. `test-modules.ts` was approved and inspected but required no edit: path-attribute registration is enforced by the dedicated exact-path check while its existing inventory stays unchanged.

Only reviewed current regions were added or renewed: the decision, policy code, reason, diagnostic, counter/real opener boundary, ordinary-refusal helper and new test/helper owners. The final boundary has 127 fixed inputs and 423 reviewed edges. The simplification pass kept the decision as a derived accessor rather than storing a second state field; no new admission token, generic callback framework or work conversion was introduced. The test-only closed-clone helper is confined to the owned fixture module.

All three plan threats are covered by the pre-backend refusal/manifest/probe controls, unavailable observation under forged markers and fixed redacted diagnostics. No additional production endpoint, schema, auth path or file-access surface was introduced. There are no blocking stubs or unresolved plan-local issues.

CWOR-05's core gate is delivered; Plan 11 owns operator adapter propagation and the parent owns requirement/state finalization. Phase 166 retains canonical bootstrap, all-authority exact reopen acceptance and genesis BASIC; 167 retains contextual completeness; 168 retains actual pinned-daemon qualification. No canonical support or qualification receipt is claimed. Existing contributor-facing support claims remain accurate; this internal refusal API hands off to Plan 11.

AGENTS.md, its Bright Builds sidecar, placeholder-only overrides and the architecture/code-shape/verification/testing/Rust/TypeScript standards informed execution. Both active lesson inputs were loaded completely: 7,188 bytes and 2,397 conservative estimated tokens. The first recorded RED command began at 2026-10-15T06:46:11.635Z; summary preparation followed at 07:03Z and final artifact checks at 07:05Z. The approximately 19-minute recorded execution interval is workflow duration, not performance evidence.

## Task Commits

All task, RED/GREEN and metadata commits are explicitly deferred to parent strict finalization. This executor staged, committed and pushed nothing; it did not edit hooks or shared STATE/ROADMAP/REQUIREMENTS/config/todo. No task commit hash is invented.

## Self-Check: PASSED

All eight changed implementation/test/checker/mapping paths and this summary exist. The new Rust module has its immediate registered parity breadcrumb and recognized `tests/` path. Owned Rust/TypeScript files are at most 628 lines: the largest is `fjall_store.rs` at 627; the new test module is 313. Original preservation source attribution and constructor-only diff were checked, and original registration/assertions remain unchanged. Final stable guard, both declared Cargo commands and CLI gates, formatting, clippy, build, managed checks, breadcrumbs and whitespace checks passed. Saved guard source hashes are unchanged, the Git index is empty, and no historical receipt was replaced. Shared planning-state updates and full phase native verification remain assigned to the parent.
