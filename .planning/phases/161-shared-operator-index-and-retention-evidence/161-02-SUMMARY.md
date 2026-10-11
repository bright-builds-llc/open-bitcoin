---
phase: 161-shared-operator-index-and-retention-evidence
plan: "02"
subsystem: storage
tags: [basic-filters, fjall, retained-accounting, recovery, bounded-observation]
requires:
  - phase: 161-01
    provides: Typed BASIC measurement and fixed unavailable categories
provides:
  - Complete checked immutable-row measurements from existing integrity validation
  - Finite observation budgets that preserve mandatory recovery validation
  - Constant-cost inspection with raw-write and failed-recovery revocation
affects: [161-03, 161-04, 161-05, 161-08, 161-09]
tech-stack:
  added: []
  patterns: [Publication-owned scalar observation, complete-pass installation, checked optional budgets]
key-files:
  created:
    - packages/open-bitcoin-node/src/storage/fjall_store/filters/retention.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/filters/tests/retention.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/filters/tests/retention/recovery.rs
  modified:
    - packages/open-bitcoin-node/src/storage/fjall_store/filters.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/filters/publication.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/filters/publication/epoch.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/filters/startup.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/filters/ownership.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/filters/lifecycle.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/filters/tests.rs
    - docs/parity/source-breadcrumbs.json
key-decisions:
  - Retained observation shares PublicationControl and its integrity lifetime, without persisted schema or another scan.
  - Exactly-at budgets remain measurable; exceeding 2000000 rows or 256 GiB stops optional aggregation while mandatory validation continues.
  - Raw integrity invalidation clears observation; ordinary append revision changes preserve immutable totals.
requirements-completed: []
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 161-2026-10-11T04-48-29
generated_at: "2026-10-11T09:23:05Z"
duration: 31min evidence window including correction
completed: 2026-10-11
---

# Phase 161 Plan 02: Recovered Retained BASIC Totals Summary

**Existing complete Fjall integrity recovery now earns checked immutable-record count, encoded payload bytes and logical record key/value bytes, with explicit unavailable states and constant-cost inspection.**

**Correction:** the initial version of this summary incorrectly claimed there was no additional startup accounting traversal. The wrapper still performed a final full scan after the selected mode had already validated the forest. Its per-pass point-read counter reset hid that duplication. The parent found this through source review; the correction and cumulative production-caller evidence below supersede that original claim.

## Performance and Accomplishments

- Two tasks completed across eleven source/manifest paths after correction. Recorded evidence spans 2026-10-11T05:25:41.199Z through 2026-10-11T05:56:34Z; implementation began before the first recorded command.
- The existing immutable forest traversal accumulates each primary row once. Parent and active-projection verification reads do not contribute; disabled, displaced and processed-ahead rows do. No new accounting traversal, persisted aggregate, schema migration, dependency or crate was added.
- Only a completed immutable plus hidden-projection validation result reaches the guarded installer. Startup revokes previous observation first and marks failed recovery unavailable. Known empty initialization earns actual zero; reopened handles remain incomplete until the existing explicit recovery succeeds.
- Observation limits are 2,000,000 immutable rows and 256 GiB cumulative immutable key/value bytes. All three totals and key-plus-value addition use checked arithmetic. Over-budget aggregation stops, but mandatory forest/projection validation still completes or rejects corruption. Smaller injected budgets exercise the actual startup path.
- Direct recovered enable carries the already-earned scan result to ownership proof installation. Metadata-only revision/disable preserves valid immutable observation. Raw invalidation resets it, and publication poison prevents the getter from reading old memory.

## Initial Verification Evidence

Every Cargo invocation ran serially through `bun run scripts/command-timings.ts run --key KEY -- ...`. The temporary executable was re-probed as Bun 1.3.9 and its directory prepended to PATH, matching `.bun-version`. Timing records are under ignored `.local/open-bitcoin-dev/command-timings/KEY/`; durations are diagnostics, not performance guarantees.

| Key / check | Actual result | Wrapper duration |
| --- | --- | --- |
| `phase161-retention-red-confirm` | 1 discovered, 1 failed actual assertion: measured empty zero versus `RetentionIncomplete` | 9,768 ms |
| `phase161-retention-recovery` | 1 discovered, 1 passed; transient private-interface warning subsequently corrected | 32,423 ms |
| `phase161-retention-expanded` | 14 discovered, 14 passed | 26,668 ms |
| `phase161-retention-recovery-final` | 14 discovered, 14 passed after real-startup budget injection and inspection counters | 22,315 ms |
| `phase161-retention-storage-regression` | 155 discovered, 155 passed, including all final 16 recovery tests | 107,061 ms |
| `phase161-retention-clippy` | Strict `--all-targets --all-features -- -D warnings` exited 101 on exactly two dormant consumer methods, listed below | 8,993 ms |
| `phase161-retention-clippy-observe` | Ordinary scoped Clippy passed with exactly the same two warnings and no other warnings | 15,776 ms |
| Scoped Rustfmt write plus final `--check`, edition 2024, `skip_children=true` | Passed for all nine owned Rust paths | Passed |
| `bun scripts/check-parity-breadcrumbs.ts --check` | Verified 1,103 Rust paths, including both newly registered files | Passed |
| `bun scripts/bright-builds-check.ts all` | 1,439 file-length inputs, zero findings; repository active lessons 1,958 bytes / 653 estimated tokens, zero findings | Passed |
| Final `git diff --check`, owned diff review and simplification pass | Passed; publication file remains 628 lines, new helper 124 and test file 470 | Passed |

The initial 16 recovery tests cover measured empty/no-genesis, exact genesis encoding basis, disabled real reopen, stale/displaced plus processed-ahead rows, corrupt/missing parent, corrupt hidden projection, missing active projection, failed initialization, exact/one-over row and byte budgets, continuing mandatory integrity after observation exhaustion, checked count/payload/key-value overflow, raw clone invalidation, poison, direct recovered enable and ordinary revision/disable preservation. These tests did not establish the total startup scan count. Fixtures are real Fjall stores with codec-valid historical records; they do not claim consensus validation. Plan 08 owns genuine validated-chain consumer evidence.

Independent expected totals enumerate raw primary keys/values in tests and read the stored envelope's payload length. The production accounting does not invoke that helper. For 1-row and 48-row stores, 64 getter calls increment the test-only inspection counter by exactly 64 and add zero filter-integrity point reads. The getter performs only the existing publication guard and a fixed scalar copy; it has no record/history/body/undo access or recovery call. Test instrumentation is absent from production.

The initial Cargo cache wait resolved normally. A concurrent IDE workspace check was observed as the lock holder; no job was killed. Resumable sessions were polled while running, including the regression runner's 60-second liveness notification.

## Corrected Startup Evidence

The actual configure caller now holds one publication guard across the complete startup mode, its existing validation, lifecycle effects, protection checks and final observation installation. `configure_basic_filter_index_before_prune_with_retention_limits` calls `apply_basic_filter_startup_guarded`, which dispatches to `recover_basic_filter_index_guarded`, `enable_basic_filter_index_guarded` or `disable_basic_filter_index_guarded`. Each receives the same guard and injected limits. There is no wrapper scan and no unlocked intermediate observation. Disabled observation installs only after its successful lifecycle/release boundary. A final held-guard `advance_integrity_epoch` preserves the original capture revocation semantics without traversing records.

The independent test-only `BASIC_FILTER_VALIDATION_SCANS` counts validator invocations cumulatively on the synchronous caller thread and is never reset inside validation. The older per-pass point-read counter retains its historical behavior for existing linear-work tests and is no longer used to substantiate total startup traversal claims.

| Correction key / check | Actual result | Wrapper duration |
| --- | --- | --- |
| `phase161-retention-startup-scan-red` | 1 discovered, 1 failed: cumulative startup scans were 2, expected 1 | 11,230 ms |
| `phase161-retention-startup-scan-green` | 18 discovered, 18 passed after guard-owned result carry | 21,090 ms |
| `phase161-retention-startup-final` | 20 discovered, 20 passed including mode budgets, reconciliation and corruption | 26,092 ms |
| `phase161-retention-startup-storage-regression` | 173 passed / 174 discovered; unchanged Phase160 epoch control failed because removing the wrapper scan also removed its closing epoch advance | 128,849 ms |
| `phase161-retention-startup-corrected` | 22 discovered, 22 passed after restoring epoch advancement without a scan and adding orphan/unclaimed controls | 28,762 ms |
| `phase161-retention-startup-storage-corrected` | 176 discovered, 176 passed, 0 failed; includes all 22 recovery tests, 15 Plan03 publication tests and the unchanged Phase160 +2 epoch assertion | 128,349 ms |
| Corrected-source scoped Rustfmt `--check`, parity and managed checks | Passed; parity verifies 1,105 Rust paths; managed checks report zero findings | Passed |
| Corrected-source diff review, simplification and `git diff --check` | Passed; production Plan03 append/publication/retention files remained untouched during correction | Passed |

The six new recovery controls prove:

- Exactly one full validation for genuine existing-state fixtures in each of Enabled, Disabled and PreserveSaved. Exact budgets and one-over row/byte budgets are honored through each actual mode caller; optional exhaustion does not reject valid startup. Across the nine mode/budget cases, 64 observations each add zero validation invocations (576 total observations).
- Fresh truly empty Enabled startup performs zero full forest scans. This is earned by existing bounded prefix-absence evidence: `maybe_basic_filter_owner_guarded_counted` calls `basic_filter_artifacts` when state is absent, and `EffectiveIndexOwnership::maybe_from_artifacts` refuses any records, projections, lifecycle or reserved lock. `initialize_basic_filter_state_guarded` independently rechecks absence before installing zero. Owner/state absence alone is not used as a zero proof.
- Reconciliation retains its two existing mandatory validations, one recovery pass and one checkpoint-publisher pass, with no accounting-only third traversal. The caller's injected over-budget observation survives the publisher's internal default-limit measurement without leaking an intermediate available total.
- Corrupt hidden projections refuse recovery in every startup mode even when the observation budget is exhausted; no partial total is installed.
- Existing state with an empty checkpoint and three unclaimed/ahead immutable rows counts all three in each mode using one scan. Row presence is never inferred from checkpoint height.
- Valid and corrupt orphan-only rows with no saved state refuse Enabled, Disabled and PreserveSaved, remain unavailable and do not create state. Existing `PartialState` refusal semantics are preserved rather than admitting an orphan repair path.

The failed 174-test regression was corrected in production code, without changing or suppressing the original Phase160 test. The final 176-test run explicitly passed `phase160_query_epoch_recovery_same_generation_never_revives_token` and its exact `epoch_value + 2` assertion.

## Public Internal API and Handoff

- `FjallNodeStore::basic_filter_retention()` returns `Result<BasicRetentionMeasurement, BasicIndexUnavailableReason>` and never initiates validation or recovery.
- `RetainedBasicFilterObservation` is `Incomplete`, `Failed`, `BudgetExceeded` or `Complete(measurement)`. Unavailable categories are fixed safe contract reasons; no backend text, keys, identifiers or paths cross this boundary.
- `PublicationControl.retention` is installed through the existing held guard. `measure_basic_filter_records(limits)` returns its result to the caller and never recursively locks publication.
- The accounting basis is `ImmutableRecordKeysAndValuesIncludingStaleBranches`. Payload bytes are a subset of immutable key/value bytes. Active projections, state, owner/checkpoint/protection/locks and Fjall physical overhead are excluded.
- Achieved deduplicated deltas for compatibility writers and the primary append path belong to Plan 03; this correction preserved that already integrated implementation unchanged. Unknown observation must never be restored from a partial delta.

## Threat Mitigations

| Threat | Evidence |
| --- | --- |
| T-161-02-1: incomplete or forged successful retained measurement | Complete-pass return value, guarded installation, startup-before-effects revocation, failed-recovery state, raw clone invalidation and poison tests. Corrupt hidden projections reject the complete result even after all immutable rows were accumulated. |
| T-161-02-2: accounting denial of service | Reuses the existing bounded-per-record validator, constant-size scalar state, checked arithmetic, finite optional budgets and 64-read structural controls across 1/48-row fixtures. Observation exhaustion does not weaken mandatory validation. |

No new endpoint, authentication path, storage namespace, schema or trust boundary was introduced. Status cannot activate, recover, index, download history, clear faults, advance durability or release prune protection. Retained growth does not change block/undo soft targets or body/wallet eligibility. No stronger crash-window, archive-scale, public-default, production-readiness or funds-safety claim is made; Phase 162 retains integrated-proof ownership.

## Deviations from Plan

1. **Authorized scope adjustment:** the root reserved existing `filters/lifecycle.rs` and `filters/publication/epoch.rs` for the minimal already-earned scan-result wiring and raw invalidation reset. This avoids another accounting scan and prevents a later integrity reinstallation from reviving an old total.
2. **[Rule 3 - Blocking] Publication length:** moved the existing unchanged `invalidate_append` implementation into the existing epoch module, keeping freshness logic together and the publication module at its 628-line managed limit. No guarded publication algorithm was moved or weakened.
3. Used the explicitly allowed single `mod retention` declaration in existing `filters/tests.rs` to reuse private real-store fixtures. There is no duplicate module declaration.
4. **[Rule 1 - Bug] Duplicate startup traversal:** parent source review disproved the initial no-extra-scan claim. Replaced wrapper rescanning with guard-owned result carry and cumulative outer-caller validation controls. The correction adds one focused recovery test child and its scoped parity mapping.
5. **[Rule 1 - Bug] Closing epoch regression:** the first corrected broader run found 173/174 passing because the removed wrapper also supplied a historical epoch advance. Restored that advance under the held guard without a scan; the unchanged original test passed in the final 176/176 run.

Material guidance loaded: repo AGENTS and Bright Builds sidecar, placeholder-only overrides, architecture/code-shape/testing/verification/operability/Rust standards, finalized lifecycle context/research/UI contract, Plan 01 summary and actual shared API. Both canonical active lesson inputs were read completely: global 5,230 bytes plus repository 1,958 bytes, total 7,188 bytes / 2,397 conservative tokens. Parent-owned synchronization, trackers and unrelated source changes were preserved.

## Deferred Issues and Remaining Gates

Strict lint is intentionally not reported clean at this intermediate boundary. At the initial lint evidence boundary, exactly two production consumer methods were dormant until the later authoritative producer consumes the store API (line numbers below identify that recorded initial diagnostic):

- `filters/retention.rs:81`: `RetainedBasicFilterObservation::measurement` is never used in the non-test library.
- `filters/retention.rs:102`: `FjallNodeStore::basic_filter_retention` is never used in the non-test library.

Both are used by meaningful tests. No `allow`, `expect`, dummy consumer or lint suppression was added. The authority plan must wire the real consumer, and the root's final strict lint/native gate must clear both diagnostics. Achieved publication accounting is recorded by Plan 03; live projection/rendering, validated-chain evidence, independent source/security review and formal lifecycle closure remain later plans/root. CFOP-01 is therefore deliberately absent from `requirements-completed` in this intermediate summary.

## Known Stubs

No placeholder or mock implementation prevents the recovered-accounting slice. The production getter is complete but awaits its planned authority consumer; achieved publication accounting is separately owned by Plan 03 and preserved during correction. This summary does not claim final phase completion or clean native verification.

## Task Commits

None by design. This executor staged, committed and pushed nothing and did not edit STATE, ROADMAP, REQUIREMENTS or todo. The root owns finalization.

## Self-Check: PASSED

All eleven source/manifest paths and this summary exist. The three new Plan02 Rust paths are registered with existing pinned `index/base.cpp` and `index/blockfilterindex.cpp` targets. Corrected source passes scoped formatting and whitespace checks; the final 22 recovery tests ran successfully within the 176-test regression, including the unchanged Phase160 epoch control. The cumulative counter proves actual startup traversal counts and zero poll scans, superseding the initial insufficient per-pass evidence. Strict lint's two deferred initial diagnostics are recorded accurately. Commit checks do not apply because nested commits are expressly prohibited.

## Native 2 Diagnostic-Precedence Follow-Up

The parent reported a subsequent full native run stopping after 1,440,476 ms with node results of 1,631 passed, one failed and three existing ignored. Its only node failure was the unchanged production test `sync::tests::filter_index::startup::filter_index_production_reopen_refuses_each_lone_partial_artifact`. The earlier storage-only regression did not exercise this production runtime diagnostic assertion. No full native success is claimed here; RPC, Bazel and coverage were not reached in that parent run.

The exact test was reproduced before editing: one discovered, one failed. A projection-only partial artifact reached the forest validator first and produced `missing BASIC suffix projection record`, instead of the baseline refusal category `BASIC state is absent but index artifacts exist`. Both paths refused startup, but diagnostic precedence is observable behavior and required correction.

The minimal correction in `recover_basic_filter_index_guarded` restores the baseline order. A present saved state still requires forest measurement before owner/protection inspection. With absent state, no forest scan runs: a private candidate zero is installed only after the existing bounded owner/artifact classifier proves complete absence. Records, projections or reserved ownership artifacts instead fail with the original partial-state category before lower-level forest diagnostics. This preserves the BlockIndex/Repair refusal, existing bytes and body/undo effects verified by the original unchanged test. It does not admit or repair orphan-only partial state.

The existing empty-recovery test was strengthened to assert zero cumulative forest validations. All existing-state one-scan, fresh-empty zero-scan, reconciliation two-scan and zero poll-scan controls remain intact. Only `filters/startup.rs`, the targeted empty assertion in `filters/tests/retention.rs`, and this summary changed in this follow-up. Other agents' source, original production tests and root-owned trackers were preserved.

Every Cargo command again used pinned Bun 1.3.9 and the serial timing runner. Follow-up evidence:

| Key / check | Actual result | Wrapper duration |
| --- | --- | --- |
| `phase161-startup-diagnostic-red` | Exact original production test: 1 discovered, 1 failed on diagnostic precedence | 1,921 ms |
| `phase161-startup-diagnostic-green` | Exact unchanged test: 1 discovered, 1 passed; its loop covers record-only, projection-only and reserved-lock-only artifacts and checks unchanged stored/payload evidence | 8,384 ms |
| `phase161-startup-diagnostic-startup-regressions` | All node tests selected by `startup`: 45 discovered, 45 passed | 38,995 ms |
| `phase161-startup-diagnostic-retention-regressions` | Recovery/cumulative controls: 22 discovered, 22 passed | 18,123 ms |
| `phase161-startup-diagnostic-clippy` | Node `--all-targets --all-features -- -D warnings`: passed, no warnings; the initially dormant methods are now consumed by later phase wiring | 5,519 ms |
| Scoped Rustfmt `--check`, parity and `git diff --check` | Passed; parity verified 1,122 Rust files | Passed |
| Historical live guards 157, 158, 159 and 160 | Passed | Passed |
| Historical live guards 155 and 156 | Reported parser/shape mismatches on baseline-compatible conditional measurement; root owns guard adaptation | Expected compatibility follow-up |

Phase 155 reports the ordinary-body/required-inspection condition and missing literal `self.measure_basic_filter_records(limits)?;`; Phase 156 reports T-156-06 ordinary-body/conditional-route findings. These are explicitly reported rather than suppressed or silently marked passing. This executor did not modify historical guard scripts. The parent owns their compatibility update and the next complete native run.

This is a Rule 1 observable-behavior correction. The original test and expected category were not weakened. No schema, dependency, accounting scan, checkpoint authority, protection relaxation or privacy surface was added. Requirement activation remains deliberately empty until the final formal lifecycle gate.

## Native 2 Follow-Up Self-Check: PASSED

The two targeted source paths and this summary exist. The exact RED/GREEN, 45 startup tests, 22 cumulative recovery controls and strict node Clippy results above were observed directly. Formatting/parity/whitespace checks passed. Historical 155/156 compatibility work and complete native verification remain accurately assigned to the parent. No staging, commits, pushes or tracker edits occurred.
