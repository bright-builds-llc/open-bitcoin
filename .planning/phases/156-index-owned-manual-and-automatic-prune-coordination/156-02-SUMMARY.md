---
phase: 156-index-owned-manual-and-automatic-prune-coordination
plan: "02"
subsystem: storage
tags: [basic-filter, ownership, work-token, publication, startup, fjall]
requires:
  - phase: 156-01
    provides: Checked lifecycle generations, exact ownership/work identities and additive owner codec
provides:
  - Guarded bounded effective-owner loader and opaque same-store work tokens
  - Token-gated record and checkpoint publication with atomic release proof
  - Lifecycle-aware pre-prune startup validation and compatible legacy materialization
affects: [156-03, 156-04, 156-05, 156-06, 156-07, 156-08, 157]
tech-stack:
  added: []
  patterns: [shared publication guard, exact work revalidation, local-edge identity, exclusive startup recovery]
key-files:
  created:
    - packages/open-bitcoin-node/src/storage/fjall_store/filters/ownership.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/filters/tests/ownership.rs
  modified:
    - packages/open-bitcoin-chainstate/src/filter_index.rs
    - packages/open-bitcoin-chainstate/src/filter_index/tests/commitments.rs
    - packages/open-bitcoin-node/src/storage/filter_index.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/filters.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/filters/publication.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/filters/startup.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/filters/tests.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/filters/tests/faults.rs
    - packages/open-bitcoin-node/src/sync/tests/filter_index.rs
    - packages/open-bitcoin-node/src/sync/tests/filter_index/recovery.rs
    - packages/open-bitcoin-node/src/sync/tests/filter_index/faults.rs
    - docs/parity/source-breadcrumbs.json
key-decisions:
  - Store incarnation is the existing shared publication-control Arc; tokens contain private exact work facts and no caller-name authority.
  - Bounded local identity validates commitments and immediate ancestry only; full forest and projection proof remains mandatory at startup and release.
  - Startup validates saved records before bounded checkpoint lookup to preserve existing corruption diagnostics and checks unsafe intent before owner materialization or reconciliation.
patterns-established:
  - Every live immutable-record append and checkpoint publication revalidates current Active owner and durable preparation fence under one guard.
  - Disabled saved state retains its rows/checkpoint and may retain conservative protection while issuing no work.
requirements-completed: []
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 156-2026-10-04T20-28-36
generated_at: 2026-10-04T21:58:58Z
commits: []
git_finalization: pending consolidated root commit after clean whole-phase verification and full native gate
duration: 35min from first recorded TDD command
completed: 2026-10-04
---

# Phase 156 Plan 02: Guarded Work and Publication Summary

**Opaque store-bound BASIC work authority with fresh lifecycle/frontier fencing, atomic checkpoint release and lifecycle-aware pre-prune startup.**

## Performance

- First recorded TDD command: 2026-10-04T21:24:02.566Z.
- Targeted verification completed: 2026-10-04T21:58:58Z.
- Tasks completed: 3/3.
- Source, test and mapping files changed: 14; summary added separately.
- Both active lesson files read completely: 5,230 global bytes plus 1,958 repository bytes; 2,397 conservative estimated tokens.

## Accomplishments

- Added one guarded effective-owner loader covering absent/partial legacy artifacts, explicit Active/Disabled state, malformed owner/protection, exact checkpoint/fence identity and effective reserved protection. A 129-row fixture proves five filter point reads for a current ownership snapshot, independent of prefix length; full integrity remains a separate startup/release obligation.
- Added crate-scoped `BasicFilterWorkToken` with private fields. Minting verifies Active ownership and the durable coins/metadata preparation fence; valid legacy Active generation zero is materialized by SyncAll before work returns. The token holds the existing publication-control Arc, so clones share an incarnation and another store/reopen cannot reuse its work.
- Both live writers require a token and verified fence. They reject stale generation, older same-generation frontier, changed branch/fence, Disabled owner and foreign incarnation before preparing any write. Record-only rows cannot advance projection, saved checkpoint or release protection.
- Checkpoint publication reuses complete immutable forest/prefix/projection checks, coins-heads/best-block and durable metadata verification, immutable conflict refusal and the existing SyncAll batch. Rows, projection, checkpoint and preserved ordinary lock map commit together. Ambiguous commit/reply failures poison subsequent guarded work until reopen.
- Startup holds the shared publication guard, validates retained history, checks direct genesis/height-one intent protection before any materialization/reconciliation, and uses the private guarded batch implementation. Disabled retains valid saved rows/checkpoint without requiring an already released lock or issuing work.
- Migrated all existing live writer callers to explicit current work. Deliberately orphaned rows use a labeled cfg(test) raw fixture; the missing-predecessor and conflicting-row tests now initialize explicit Active state so they continue testing record validation rather than failing at absent authority.

## Task Finalization Records

The strict phase wrapper defers Git finalization. No task, RED, GREEN, metadata commit, push or hook bypass was performed.

1. Task 1, guarded ownership loading and opaque work minting: implementation and bounded ownership/legacy/stale tests verified; pending consolidated root finalization.
1. Task 2, live publication gates and startup recovery: production library check passed before fixture migration; fault/reopen and lifecycle compatibility evidence verified; pending consolidated root finalization.
1. Task 3, existing recovery caller migration: full filter-index regression suite and refined legacy materialization case verified; pending consolidated root finalization.

STATE, ROADMAP, REQUIREMENTS and config updates remain root-owned. CFPR-01 is not activated by this summary.

## Verification

Pinned Bun 1.3.9 was selected and reprobed from `/tmp/open-bitcoin-bun-1.3.9/bun-darwin-aarch64`; Rust 1.94.1 remained pinned. All ad-hoc Cargo commands were timing-wrapped and serialized.

| Check                                          | Command                                                                                                                                                                                                                                                                                           | Evidence                                                                                                                                                                                                  |
| ---------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Task 1 RED                                     | `bun run scripts/command-timings.ts run --key phase156-owner-loader-red -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --lib filter_index`                                                                                                                                | Expected exit 101: unresolved work mint and missing token/fence writer contract.                                                                                                                          |
| Production contract before remaining migration | `bun run scripts/command-timings.ts run --key phase156-gated-publication -- cargo check --manifest-path packages/Cargo.toml -p open-bitcoin-node --lib`                                                                                                                                           | Passed in 23.37s. Compile evidence only; not counted as tests.                                                                                                                                            |
| Core direct-edge contract                      | `bun run scripts/command-timings.ts run --key phase156-direct-edge-core -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-chainstate --lib filter_index`                                                                                                                          | Initial and final rerun both passed 44/44, including correct direct edge, malformed parent facts and changed commitment tests. Final rerun followed coherent test relocation.                             |
| Publication/recovery regressions               | `bun run scripts/command-timings.ts run --key phase156-publication-regressions -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --lib filter_index`                                                                                                                         | Final full run passed 78/78, 985 filtered out, test body 33.42s. Includes ten new ownership tests plus existing codec/storage/production recovery/fault controls.                                         |
| Refined nonempty legacy materialization        | `bun run scripts/command-timings.ts run --key phase156-legacy-owner-startup -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --lib storage::fjall_store::filters::tests::ownership::filter_index_work_mint_materializes_legacy_owner_before_returning_authority -- --exact` | Passed 1/1 after the full suite. Parameterizes production startup guard versus direct work mint, with nonempty saved prefix, exact retained rows, durable Active generation zero and actual close/reopen. |
| Existing recovery guard checker                | `bun run scripts/check-phase155-filter-index.ts`                                                                                                                                                                                                                                                  | Passed source/parity/scoped claim links; not substituted for durable tests.                                                                                                                               |
| Source breadcrumbs                             | `bun run scripts/check-parity-breadcrumbs.ts --check`                                                                                                                                                                                                                                             | Passed for 952 discovered first-party Rust files. Both new Rust paths registered using exact `git add -N` and matching in-source breadcrumbs.                                                             |
| File lengths                                   | `bun run scripts/bright-builds-check.ts file-lengths`                                                                                                                                                                                                                                             | Passed, 1,271 scanned files, zero findings.                                                                                                                                                               |
| Formatting and diff                            | Scoped `rustfmt --check --edition 2024 --config skip_children=true` on owned Rust files and `git diff --check`                                                                                                                                                                                    | Passed; unchanged module children and other agents' edits preserved.                                                                                                                                      |

The full 78-test pass preceded a narrow refinement of one existing legacy test and new optional snapshot binding names; its exact refined case passed afterward. Production source did not change after that full pass. The root-owned native verifier will compile/test the final whole-phase source.

## Fault and Reopen Observations

Pre-record/checkpoint/protection faults preserve conservative saved state and locks. Record-only AfterCommit returns failure but real reopen observes ahead immutable rows with the old cursor/protection. Checkpoint AfterCommit returns failure but reopen observes the complete newly committed checkpoint/protection/map; failed replies are not treated as rollback evidence. Legacy materialization failure poisons retry, keeps old conservative state/lock and absent owner before reopen, then permits a fresh materialization.

Stale frontier/generation/branch and foreign-store tests compare state, locks, owner and row absence after actual close/reopen. A token intentionally surviving its store's close is rejected by the reopened incarnation. Disabled retained-history tests validate both released and extra conservative protection, reject old work and keep immutable rows/checkpoint. These Disabled scenarios seed explicit lifecycle through cfg(test) raw fixtures; production disable/re-enable transitions and runtime-constructor lifecycle proof remain Plans 03/07 responsibility. The refined legacy case calls the actual production startup guard directly, not the runtime constructor.

Original validated-spend, equal-height fork, ahead-row, real H/B partial-coins replay, metadata failure, unsafe-intent and low-height production-runtime checks remain in the full suite. Sparse deletion-order fixtures retain their existing label and are not promoted to consensus/client-after-prune proof.

## Decisions Made

Local AGENTS guidance, Bright Builds sidecar, placeholder-only overrides and architecture/code-shape/testing/verification/Rust standards informed this work. Functional decisions stay in the pure model; the adapter only loads facts, checks exact authority and performs effects. Lock order remains authority → automatic state → publication → payload, and private guarded helpers do not reacquire the publication mutex.

Bounded checkpoint construction required a pure local-edge constructor and a codec accessor. `FilterRecordIdentity::new` delegates to the same validators, preserving its behavior; the new constructor explicitly does not prove complete ancestry or confer release authority. Startup and release still perform full forest/projection validation.

## Threat Mitigations

| Threat   | Concrete mitigation and evidence                                                                                                                                                                                                                     |
| -------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| T-156-04 | Private same-incarnation token, current Active owner, exact generation/frontier/branch/protection comparison and durable fence verification before either writer prepares rows; stale/foreign/Disabled snapshot and real reopen tests pass.          |
| T-156-05 | Complete immutable forest and saved prefix/projection validation plus current recovered coins/metadata fence; existing atomic SyncAll batch publishes proof and full protection map together, preserves ordinary locks and refuses conflicts/bounds. |
| T-156-06 | Exclusive guard-held startup, direct intent check before materialization/reconciliation, Disabled-compatible retained-history validation and poison-on-ambiguous-commit/reply; existing low-height, unsafe intent and faults pass.                   |
| T-156-07 | Actual close/reopen state/lock/owner/row observations distinguish before-commit old state from AfterCommit durable new proof; no failed-reply rollback claim or payload disclosure in diagnostics.                                                   |

No new network endpoint, authentication path or filesystem trust boundary was introduced. No unresolved HIGH issue was identified in the owned diff; whole-phase security review remains root-owned and pending.

## Simplification Review

One pure ownership model, one shared publication guard and one opaque capability reuse existing store synchronization and atomic batches. The bounded loader does not scan the forest per application; complete scans stay at startup/release. A small direct-edge constructor avoids duplicating parsers or implementing a second checkpoint identity model. No new dependency, crate, mutex, cache, caller-name authorization, deletion owner, fabricated accounting or ordinary lock formula was added.

## Deviations from Plan

### Authorized Scope Refinements

1. The bounded loader could not construct the pure checkpoint identity from existing private codec fields without recursively scanning ancestry. The parent authorized narrow extensions to core `filter_index.rs`, its existing `tests/commitments.rs` and node `storage/filter_index.rs`, then updated Plan 02 scope/actions. Existing validators are reused; complete release/startup proof was preserved.
1. Added the coherent `filters/tests/ownership.rs` module and relocated three shared fixture helpers into existing `tests/faults.rs` to meet source length limits. The two direct-edge tests moved into the existing commitments module rather than leaving core `tests.rs` above 628 lines. Breadcrumbs were registered in the same task.

The strict wrapper explicitly replaces task/TDD commits and state mutations with pending root finalization records; no deviation from that constraint occurred.

## Issues Encountered

- The first migrated node compile found a missing `CoinsView` trait import in the sync helper. Added the import before the behavioral run.
- The first behavioral run passed 73/78 and failed five existing exact corruption diagnostic checks. The bounded lookup had changed missing/weak protection, same-height fence and malformed projection categories. Grouped diagnostic mapping and full startup forest validation before bounded lookup restored the original messages without changing refusal behavior; the full rerun passed 78/78.
- After two failed node attempts, verification was explicitly reassessed: grouped diagnostics/format fix, core suite after relocation, full node suite, then only the necessary exact legacy refinement. No wider test matrix or unrelated fix was introduced.
- Cursor rust-analyzer independently held the shared artifact lock; live compiler children were observed and the lock serialized the commands. Later root read-only host diagnostics identified severe paging pressure. Core recompilation took about seven minutes and newly linked binaries paused before their test harnesses; process/CPU evidence distinguished those boundaries from test execution. Commands were polled within 60 seconds and no primary process was terminated for elapsed time or silence.

## Known Stubs

None. No TODO/FIXME, placeholder output or unimplemented production body was introduced. Public activation, scheduled workers, real disable/re-enable transitions and broader prune wiring remain declared dependent work.

## Next Plan Readiness

Plan 03 can consume the guarded effective-owner/lifecycle helpers and private publication batches for internal lifecycle transitions. Plans 04–06 retain ownership of reserved CRUD, actual deletion and automatic measurements. Full native formatting/lint/build/tests, coverage, Bazel/provenance, phase lifecycle validation, security review and the final commit hook remain mandatory root gates.

## Self-Check: PASSED

All 14 declared source/test/mapping paths and this summary exist. Core 44/44, node 78/78 and the refined legacy exact case 1/1 passed with nonzero counts. Scoped Rust formatting, source breadcrumbs, source file lengths and whitespace checks passed. Markdown was checked and formatted with installed GFM/frontmatter extensions. No commit existence claim applies: `commits: []` and consolidated root Git finalization remain pending.
