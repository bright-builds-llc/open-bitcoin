---
phase: 161-shared-operator-index-and-retention-evidence
plan: "03"
subsystem: storage
tags: [basic-filters, fjall, immutable-publication, retention, bounded-accounting]
requires:
  - phase: 161-02
    provides: Recovered constant-size retained measurement and shared freshness authority
provides:
  - Checked pre-effect immutable growth installed only after successful publication
  - Unique missing-row deltas for compatibility publishers and retry/reorg reuse
  - Fault, raw-clone, stale-work, overflow and real-reopen accounting regression evidence
affects: [161-04, 161-08, 161-09, 161-10]
tech-stack:
  added: []
  patterns: [Borrow admitted encoded lengths, guarded achieved installation, unknown totals stay unknown]
key-files:
  created:
    - packages/open-bitcoin-node/src/storage/fjall_store/filters/tests/retention/publication.rs
  modified:
    - packages/open-bitcoin-node/src/storage/fjall_store/filters/append.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/filters/publication.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/filters/retention.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/filters/tests/retention.rs
    - docs/parity/source-breadcrumbs.json
key-decisions:
  - Reuse validated encoded row lengths; never decode, clone payloads or read rows for a delta.
  - Keep incomplete, failed and budget-exceeded starting observations unavailable after small appends.
  - Reuse the compatibility publisher's existing finite candidate comparison to skip identical duplicate and existing rows.
  - Revoke an arithmetically exhausted aggregate before refusing effects.
requirements-completed: []
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 161-2026-10-11T04-48-29
generated_at: "2026-10-11T05:46:08Z"
duration: 10min evidence window
completed: 2026-10-11
---

# Phase 161 Plan 03: Achieved Publication Accounting Summary

**Successful immutable publication updates complete retained totals once; retries, retained-branch reuse, raw invalidation and uncertain effects preserve truthful availability.**

## Accomplishments

Completed both tasks. The primary append and both compatibility publishers compute checked prospective totals from their already encoded unique missing-row buffers, before batch effects. They install those totals only after `finish_basic_filter_batch` succeeds under the existing guard. Checkpoint/projection/owner/lock/coins metadata contributes no immutable growth. SyncAll, safe checkpoints, work limits and reserved protection remain intact.

The compatibility comparison pass now excludes existing identical rows and identical duplicate candidate identities. It keeps the existing candidate-prefix comparison bound (at most 128 admitted candidates), performs no extra deduplication pass, and allocates no key set or additional payload copy. Conflicting identities fail before effects. Unknown starting observations cannot become complete from a partial delta; arithmetic overflow revokes the impossible total and refuses writes.

The existing Plan 02 raw put/remove invalidation, shared clone control, append revision, lifecycle epoch and poison mechanisms already satisfied the expanded uncertainty controls. This plan did not change `payload_usage.rs`, `publication/epoch.rs`, `lifecycle.rs` or `reorg.rs`. It added an explicit test-only `BeforeCommit` seam and connected the existing `BeforeRecords` seam to guarded compatibility checkpoint publication. Neither seam exists in production.

## Verification Evidence

All Cargo runs were serial through `bun run scripts/command-timings.ts run --key KEY -- ...`. The temporary pinned Bun executable was probed and its directory prepended to PATH; version 1.3.9 matches `.bun-version`. The timing runner recorded Rust 1.94.1. No running command was terminated for yielding or elapsed estimates.

| Check / timing key | Actual result | Wrapper duration |
| --- | --- | --- |
| `phase161-retention-publish-red`: node `phase161_retention_publication` | 4 discovered; 3 expected incomplete-measurement failures, 1 conflict/no-effects pass | 20,918 ms |
| `phase161-retention-publish`: initial GREEN | 4 discovered, 4 passed | 38,278 ms |
| `phase161-retention-publish-expanded`: first expanded run | 15 discovered; 9 passed, 6 fixture setup/authority failures | 39,824 ms |
| `phase161-retention-publish`: corrected final suite | 15 discovered, 15 passed; 20.24 s test execution | 28,434 ms |
| `phase161-retention-filter-regression`: node `storage::fjall_store::filters` | 170 discovered, 170 passed; 104.79 s test execution | 109,074 ms |
| `phase161-retention-publication-clippy`: node `--all-targets --all-features -- -D warnings` | Exactly 2 previously reported dormant consumer methods; no new diagnostic | 21,594 ms |
| Scoped Rustfmt check, edition 2024, `skip_children=true` | All five owned Rust paths passed | Passed |
| `bun scripts/check-parity-breadcrumbs.ts --check` | 1,104 Rust files verified | Passed |
| `bun scripts/bright-builds-check.ts all` | File lengths and active lessons: 0 findings | Passed |
| `bun scripts/check-phase158-validated-reorg.ts` | Production acceptance, source admission and scoped routes checked | Passed |
| `bun scripts/check-phase155-filter-index.ts` | Three source-assertion compatibility failures; detailed below | Pending root repair |
| `git diff --check`, owned diff and stub/threat review | Passed; no implementation placeholders or new trust surface | Passed |

The expanded suite proves all three counters against independently enumerated raw immutable keys and stored envelope lengths. It includes primary append/no-batch retry, compatibility append/checkpoint retry, duplicate candidates, conflicting candidates/no effects, six aggregate overflow cases across primary/record-only publishers, six unknown-starting-total cases, and actual coins/metadata flush plus disable epoch revocation.

Thirteen injected publication boundaries cover records/checkpoint/protection/pre-commit/post-commit as applicable to all three publishers. Repeated unavailable polls leave row counts unchanged and add zero row reads. Each injection drops all handles, really reopens Fjall, and checks the complete observed forest: post-commit failure retained three rows; all pre-commit failures retained zero. These are deterministic fault seams, not claims of forced device I/O failure or stronger crash guarantees.

Raw clone controls prove a write between prepare and completion rejects stale append work; stale captured epochs remain invalid. Put/remove refusal after revision exhaustion leaves stored rows unchanged and measurement unavailable. Removing a required row produces failed complete recovery rather than healthy zero. The primary healthy poll control repeats 64 reads with zero filter point reads. Existing small/many-row and epoch/corruption regressions also passed within the 170-test run.

The genuine `ReorgFixture` follows accepted original branch → replacement → original branch, with real spent-history records and tracked reorg authorization. Metadata rewind preserves totals; new replacement records grow them; returning to the original retained immutable rows grows none. All-handle configured reopen recomputes exactly the independent retained forest. The simpler codec fixtures make no consensus-validity claim.

The first expanded run incorrectly assumed PreserveSaved recovery issued append authority and that an already completed replacement suffix could be requested again as new accepted positions. Fixtures were corrected to retain bodies/undo and explicitly enable the existing authority, and to perform a genuine return reorg before reusing displaced rows. No production capability or acceptance rule was relaxed.

## Threat Mitigations

| Threat | Evidence |
| --- | --- |
| T-161-03-1: publication falsely earns accounting | Checked borrowed-length delta before effects; install after achieved SyncAll success; duplicate/conflict, overflow and all-publisher fault/reopen controls. |
| T-161-03-2: raw writers/clones/stale epochs forge freshness | Existing pre-effect integrity reset and poison guard; clone interleave, failed raw mutations, stale captures, disable and failed/complete reopen controls. |
| T-161-03-3: accounting work/copies cause denial of service | Shared finite envelope admission; existing bounded comparison reused; scalar checked arithmetic over admitted buffers; no new index scan, storage read, payload copy or key set. |

No network endpoint, authentication path, storage namespace, schema or file-access trust boundary was introduced. Read-only collection cannot activate, recover, index, clear faults, advance durability or release prune protection. Filter growth does not change the soft logical block/undo target or body/wallet eligibility. Phase 162 integrated proof, D-16 exclusions and existing crash-window/public-default/readiness/funds-safety claim limits remain intact.

## Simplification and Deviations

1. **Authorized scope adjustment:** root explicitly authorized `filters/tests/retention/publication.rs`, a single `mod publication` declaration in the existing retention test file, and its exact breadcrumb mapping. This keeps focused uncertainty tests separate and all touched files below the managed limit.
2. **[Rule 3 - Blocking] Shared envelope admission:** deduplicated identical production candidate/singleton/aggregate envelope bounds into the existing retention module. The append caller retains the same guard; compatibility preparation calls the same guard before allocation. Publication is 617 lines, append 586, retention 192 and the focused test child 582. No guarded publication algorithm was relocated.
3. **[Rule 2 - Missing critical] Accounting exhaustion:** pre-effect overflow explicitly revokes its aggregate in addition to refusing effects, as required by the finalized research freshness contract.

The simplification pass retained one checked delta implementation and shared existing envelope bounds. It avoided another row scanner, payload decoder, copied cache, set or durable schema.

Material guidance: repo AGENTS and Bright Builds sidecar, placeholder-only overrides, architecture/code-shape/testing/verification/operability/Rust standards, finalized phase context/research/UI contract, Plan 01/02 summaries and actual publication APIs. Both canonical active lesson inputs were read completely: global 5,230 bytes plus repository 1,958 bytes, total 7,188 bytes / 2,397 conservative tokens. Parent-owned synchronization, shared trackers and unrelated changes were preserved.

## Deferred Issues and Remaining Gates

Strict lint still reports exactly the two Plan 02 consumer methods awaiting their real authority consumer: `retention.rs:113` `RetainedBasicFilterObservation::measurement` and `retention.rs:145` `FjallNodeStore::basic_filter_retention`. Tests use both. No suppression, dummy consumer or warning expectation was added. Later authority wiring and root strict native verification must clear them.

The historical Phase 155 source checker requires a narrow compatibility repair under root ownership. Its publication assertions at `scripts/check-phase155-filter-index.ts:68` still require `BASIC_INDEX_MAX_ENCODED_BYTES` and `BASIC_INDEX_MAX_SINGLETON_ENCODED_BYTES` text in `publication.rs`; those enforced bounds now reside in `retention::check_envelope_bounds`, reached before allocation. Its configured-route assertions around line 164 also expect the former direct startup body, now forwarded through the Plan 02 retention-limits wrapper. Root received exact errors and source locations. The native checker and mutation fixtures must follow the real helper/wrapper edges without weakening admission or pre-prune ordering. Production code was not distorted merely to satisfy old strings.

Full `bash scripts/verify.sh`, historical-checker repair, independent source/security review, producer/consumer evidence and formal lifecycle closure remain root/later-plan gates. CFOP-01 therefore remains absent from `requirements-completed` here.

**Inherited Plan 02 traversal defect discovered by root:** `startup::configure_basic_filter_index_before_prune_with_retention_limits` calls `apply_basic_filter_startup`, whose recover/enable/disable paths already perform complete record measurement, and then unconditionally calls `measure_basic_filter_records(limits)` again. That is an extra accounting traversal despite the earlier summary's no-second-scan claim. Resetting test read counters inside measurement hides the earlier traversal. This executor did not change startup; root reserved and will assign the correction before Plan 10. Required proof is cumulative scan accounting across actual Enabled/PreserveSaved/Disabled routes, with no per-traversal counter reset hiding work and observation budgets preserved. The Phase 155 checker repair must follow that corrected producer. This residual prevents claiming the whole accounting lifecycle is complete from the passing Plan 03 suite.

## Known Stubs

No placeholder implementation prevents this publication-accounting slice. The complete production measurement getter awaits its explicitly planned authority consumer; it does not invent default progress or restore unavailable totals. This summary does not claim final phase completion or clean native verification.

## Task Commits

None by design. This executor staged, committed and pushed nothing and did not edit STATE, ROADMAP, REQUIREMENTS or todo. Root owns finalization.

## Self-Check: PASSED

All six owned source/manifest paths and this summary exist. The authorized child has pinned parity breadcrumbs and passed the parity check. Focused and regression discovery/results are recorded above, whitespace review passed, and the two exact lint diagnostics plus pending historical/source correction gates are disclosed. Commit checks do not apply because nested commits are prohibited.
