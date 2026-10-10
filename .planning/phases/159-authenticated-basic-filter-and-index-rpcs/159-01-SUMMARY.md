---
phase: 159-authenticated-basic-filter-and-index-rpcs
plan: "01"
subsystem: storage
tags: [validation-provenance, fjall, recovery, sealed-capabilities]
requires:
  - phase: 158-validated-reorg-and-retained-branch-identity
    provides: Existing same-store publication guard and admitted replacement count bound
provides:
  - Versioned monotonic validation-history codec and honest legacy coverage
  - Same-store sealed publication contracts and fail-closed exclusive recovery
  - Durable coverage invalidation for generic metadata, snapshot and header seeding
affects: [159-03, 159-04, 159-07, 159-08]
tech-stack:
  added: []
  patterns: [constructor-free sealed publication contracts, SyncAll monotonic history, store-bound recovered capability]
key-files:
  created:
    - packages/open-bitcoin-node/src/storage/validation_history.rs
    - packages/open-bitcoin-node/src/storage/validation_history/tests.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/validation_history.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/validation_history/tests.rs
  modified:
    - packages/open-bitcoin-node/src/storage.rs
    - packages/open-bitcoin-node/src/storage/fjall_store.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/coins.rs
key-decisions:
  - Complete coverage is granted only before schema initialization in an actually empty database; legacy absence stays UnknownLegacy.
  - AcceptedValidationBatch and AdmittedValidationHeaders have private fields and no raw, test, deserialize, clone or default constructor; Plan03 owns genuine receipt mint integration.
  - Generic metadata, snapshot and header seeding durably loses complete coverage before effects, retaining positive scripts-valid history.
patterns-established:
  - Validation history describes accepted validation, never coins durability, active membership or BASIC availability.
  - History writes serialize with the existing BASIC publication guard and use their own shared recovery/poison state.
requirements-addressed: [CFRP-01]
requirements-completed: []
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 159-2026-10-09T16-11-49
generated_at: "2026-10-09T16:55:16Z"
duration: approximately 12min
completed: 2026-10-09
---

# Phase 159 Plan 01: Validation History and Recovery Summary

**A retained per-hash validation ledger now checks typed identity, monotonic upgrades and exclusive recovery while preserving honest legacy uncertainty.**

## Performance

- Tasks: 2/2 storage/codec implementation tasks completed; dependent production integration remains explicitly below.
- Source files: 7 (four created, three modified).
- Completed: 2026-10-09T16:55:16Z.
- Duration: approximately 12 minutes; a precise plan-start timestamp was not recorded.

## Accomplishments

- Added `validated_block:v1:<raw_hash>` envelopes with version/status/height/hash/parent binding and the `validated_coverage:v1` marker. Known-only records upgrade to scripts-valid; conflicting identity and downgrade fail closed.
- Added checked count and encoded-byte limits: 128 identities, 70-byte envelopes, 83-byte keys and a 19,584-byte maximum encoded batch. Empty, oversized, overflow and duplicate batches reject. One unresolved acceptance batch is the dependent manager's responsibility in Plan03.
- Startup validates every history row and coverage key before schema migration or exposing the store, irrespective of BASIC activation. Legacy stores receive no inferred scripts-valid backfill. A genuinely empty database earns complete coverage; reopening retained history preserves it.
- Publication checks same-store capability identity, monotonic rows and SyncAll durability under the existing publication mutex. Before-commit failure leaves prior disk state; ambiguous after-commit failure poisons every history clone until exclusive reopen.
- Generic header/snapshot/chain-metadata writes invalidate complete coverage durably before effects. Positive scripts-valid records remain readable; old known-only records become conservatively unknown when coverage is lost.
- Simplification removed duplicated production/test Fjall initialization through `open_unrecovered`; the existing root storage file is 618 lines, below the managed 628-line limit.

## Verification Evidence

All Cargo commands used pinned Bun via `scripts/command-timings.ts`; no unwrapped or overlapping target work was launched.

| Check | Result |
| --- | --- |
| Task1 initial RED, `phase159-history-codec-red` | Expected compile failure for absent contracts; 52 missing-symbol errors. |
| Task2 initial RED, `phase159-history-store-red` | Expected missing store APIs/fault type; concurrent initial codec zero-constant diagnostic was also present and fixed. |
| Semantic RED, `phase159-history-store-red-conflict` | One real runtime failure: conflicting duplicate admitted identities were accepted; `assert!(result.is_err())` failed. |
| Task1 GREEN, `phase159-history-codec` | 5 passed, 0 failed. |
| Final combined GREEN, `phase159-history-all` | 12 passed, 0 failed, 1,356 filtered; 5 codec tests and 7 store tests; no lib-test warnings. |
| `rustfmt +1.94.1 --edition 2024 --config skip_children=true --check` on all seven owned Rust files | Passed. |
| `git diff --check` | Passed. |
| `bun scripts/bright-builds-check.ts all` | 0 findings; 1,351 tracked source files scanned, repository lessons within budget. Newly created Rust files were also measured individually and remain below 628 lines. |
| Scoped Clippy with `--lib --tests -- -D warnings`, `phase159-history-clippy` | Failed on exactly 16 dormant production `dead_code` diagnostic groups pending Plans03/04; no lint suppression added. |

Behavioral tests prove codec format/identity rejection, monotonic downgrade refusal, honest fresh-versus-legacy absence, true Fjall handle drop/reopen, raw clone metadata coverage loss, foreign/conflicting publication rejection, corruption blocking exclusive open with BASIC absent, and before/after commit fault behavior. Storage tests are deliberately private codec-owner tests inspecting sealed fields directly; they expose no test constructor and do not claim genuine node acceptance. Real acceptance/reorg/prune/daemon evidence belongs to dependent plans.

## Task Commits

1. Task1: Versioned history and coverage contracts — pending root consolidated commit.
2. Task2: Monotonic durable history and recovery — pending root consolidated commit.

No staging, commits, pushes, hook bypass, STATE/ROADMAP/REQUIREMENTS/todo/config changes were performed by this executor. Root owns final native verification and Git finalization.

## Interfaces and Dependent Integration

- Codec types: `BlockValidationIdentity`, `ValidationProvenance::{NeverConnected, ScriptsValid, UnknownLegacy}`, `ValidationHistoryRecord`, `ValidationCoverage`, `AcceptedValidationBatch`, and `AdmittedValidationHeaders` in `storage/validation_history.rs`.
- `FjallNodeStore::recovered_validation_history()` returns the store-bound `RecoveredValidationHistory`; `is_complete()` rejects stale coverage revisions and poisoned state, and `belongs_to()` rejects a foreign store.
- `publish_validation_history(&AcceptedValidationBatch)` borrows the consume-only capability so the owner can retain one bounded unresolved batch on error and retry idempotently. `publish_admitted_validation_headers(&AdmittedValidationHeaders)` requires complete coverage and refuses downgrades.
- `validation_provenance(hash)` reads one ledger envelope with typed absence/error separation. Recovery scans do not run on that query path.
- **Plan03 mint obligation:** add production mint methods in `storage/validation_history.rs` that accept genuine sealed chainstate/header-admission evidence. A crate-visible raw-identity factory remains forbidden. Private fields currently cannot be constructed by external, raw or generic callers. This constructor-free staging was explicitly approved by the parent to preserve standalone compilation and avoid fake acceptance authority.
- **Plan03 manager obligation:** capture every actually absorbed connect/reorg identity before fallible effects; retain at most one checked bounded unresolved batch, refuse additional absorption before effects while occupied, publish/retry only the same store's evidence, and expose accepted-but-unpublished facts honestly. Recovery evidence never backfills legacy acceptance.
- **Plan03 header obligation:** trusted new-header snapshot publication needs a sealed path separate from generic `save_header_entries`, which intentionally invalidates complete coverage. Existing legacy headers must remain unknown without complete trusted evidence.
- **Plan04 obligation:** consume provenance through the same managed authority, preserve accepted pending evidence, and keep bounded filter integrity/readiness semantics separate from ledger recovery.

## Outstanding Production Lint Diagnostics

The following 16 diagnostic groups must be closed through real dependent consumers before root's final Clippy/native gate. They are intentionally visible, with no allow/expect attributes:

1. `RecoveredValidationHistory` is not constructed in production.
2. Its `is_complete`/`belongs_to` methods are unused in production.
3. Store recovery/publication/provenance helper methods are unused in production.
4. `MAX_VALIDATION_IDENTITIES` is unused in production.
5. `MAX_BATCH_BYTES` is unused in production.
6. `BlockValidationIdentity::hash` is unused in production.
7. `ValidationProvenance` is unused in production.
8. `ValidationCoverage::absent_provenance` is unused in production.
9. `ValidationHistoryRecord::status`/`provenance` are unused in production.
10. `AcceptedValidationBatch` is not constructed in production.
11. Its `belongs_to`/`identities`/`validate` methods are unused in production.
12. `AdmittedValidationHeaders` is not constructed in production.
13. Its `belongs_to`/`identities` methods are unused in production.
14. `validate_batch_size` is unused in production.
15. `encode_record` is unused in production.
16. `validate_upgrade` is unused in production.

## Deviations from Plan

### Rule2: Close generic metadata coverage laundering

- Parent authorized one additional source file, `storage/fjall_store/coins.rs`, for a single coverage-invalidation call before raw `save_unverified_chain_meta` effects.
- The history mutex is separate and does not reacquire the already-held BASIC publication guard. The raw-clone metadata regression proves complete coverage is lost durably while accepted records survive.

### Approved contract staging

- Rust visibility cannot restrict sibling `storage` constructors to `crate::chainstate`. The parent approved constructor-free private-field contracts in this plan and sequential Plan03 mint integration accepting genuine sealed receipts. No raw or test factory was introduced; production integration and corresponding final lint remain honestly pending.

## Issues Encountered

- The pinned `BlockHash` type has no `ZERO` associated item; corrected to `from_byte_array([0; 32])`. Replacing a wildcard import briefly exposed a missing explicit `BlockHash` import, corrected before final GREEN. These transient compile failures were communicated so concurrent parser checks were not misreported as parser evidence.
- Semantic negative testing found duplicate admitted identities could overwrite within one batch; publication now rejects duplicates before effects and the regression passes.

## Known Stubs

No placeholder return values, TODO/FIXME branches, mock runtime data sources or empty-success substitutions were added. Production capability mint/manager/query integration is deliberately assigned to Plans03/04, as listed above, and is not claimed complete by this storage plan.

## Threat Review and Limits

The additive schema and stored-byte boundary are already covered by T-159-01/02; no network, authentication, cryptographic or dependency surface was added. Sealed same-store checks, explicit legacy uncertainty, bounded identities and publication poison address this plan's declared mitigations. Plan03 owns pending-batch admission enforcement for T-159-03; Plan06 owns public redaction for T-159-04. Plan08/root must register the four new Rust breadcrumb files, review parity/readme updates and run the complete native/source/security/lifecycle gates. No actual accepted stale/pruned daemon claim, requirement completion or production/funds claim is made here.

## User Setup Required

None.

## Next Plan Readiness

Plan03 can consume the codec/store contracts now and owns the sequential mint/manager/trusted-header integration. Plan04 can then consume the resulting provenance through the same authority. Root final lint and native verification remain required before any commit or requirement completion.

## Self-Check: PASSED

- All four created Rust files, the authorized coins.rs hook and this summary exist.
- Scoped behavior evidence is 12 passed and zero failed; formatting/managed checks/diff checks passed.
- No commit hashes are claimed because staging and commits remain root-owned and pending.
- All 16 production lint diagnostic groups and dependent mint/integration obligations are disclosed; full phase acceptance remains pending.
