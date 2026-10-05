---
phase: 156-index-owned-manual-and-automatic-prune-coordination
plan: "03"
subsystem: storage
tags: [basic-filter, lifecycle, disable, re-enable, fjall, authority]
requires:
  - phase: 156-02
    provides: Guarded ownership, opaque work tokens and lifecycle-aware startup
provides:
  - Ordered durable work invalidation followed by reserved protection release
  - History-preflighted atomic Active ownership and protection acquisition
  - Specialized trusted durable-host transitions and production reopen evidence
affects: [156-04, 156-05, 156-06, 156-07, 156-08, 157]
tech-stack:
  added: []
  patterns: [two-step SyncAll disable, guarded initialization, trusted host lifecycle]
key-files:
  created:
    - packages/open-bitcoin-node/src/storage/fjall_store/filters/lifecycle.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/filters/tests/lifecycle.rs
    - packages/open-bitcoin-node/src/network/runtime_authority/filter_index.rs
    - packages/open-bitcoin-node/src/sync/tests/filter_index/lifecycle.rs
  modified:
    - packages/open-bitcoin-node/src/storage/fjall_store/filters.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/filters/publication.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/filters/tests.rs
    - packages/open-bitcoin-node/src/network/runtime_authority.rs
    - packages/open-bitcoin-node/src/sync/tests/filter_index.rs
    - docs/parity/source-breadcrumbs.json
key-decisions:
  - Disable holds the existing publication guard across two SyncAll batches; failed replies require reopen rather than rollback.
  - Re-enable reconciles using saved protection and preserves stronger retained protection before historical preflight.
  - Trusted host methods return transition completion; opaque work remains independently minted and revalidated after durable acquisition.
patterns-established:
  - Guarded initialization avoids recursive publication locking for genuinely absent activation.
  - Public specialized host calls accept no reserved name, range, checkpoint, generation or external fence.
requirements-completed: []
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 156-2026-10-04T20-28-36
generated_at: 2026-10-04T22:18:47Z
commits: []
git_finalization: pending consolidated root commit after clean whole-phase verification and full native gate
duration: 13min from first recorded TDD command
completed: 2026-10-04
---

# Phase 156 Plan 03: Trusted Disable and Re-enable Summary

**Two-step durable BASIC disable and history-preflighted protected re-enable through the production chainstate authority.**

## Performance

- First recorded TDD command: 2026-10-04T22:03:37.910Z.
- Final targeted test completion: 2026-10-04T22:16:02.700Z.
- Tasks completed: 2/2.
- Source, test and mapping files changed: 10; summary added separately.
- Both active lesson files were read completely: 7,188 bytes, 2,397 conservative estimated tokens. The existing audit baseline and size/date did not trigger an audit.

## Accomplishments

- Disable loads validated ownership under the existing shared publication guard, checks complete retained forest/checkpoint integrity and advances the generation with checked arithmetic. Its first SyncAll batch writes only Disabled ownership; its second removes only the reserved BASIC lock while preserving ordinary named locks and all filter artifacts.
- Holding the guard stops token issuance during effects. Before/after disable and release injections poison live guarded work until reopen. Reopen truthfully observes Active with old protection, Disabled with conservative protection, or Disabled with released protection according to the committed boundary. Repeated Disabled release does not advance generation.
- Re-enable verifies the real recovered coins/metadata fence, validates complete saved history, scans the retained checkpoint for reconciliation, preserves stronger conservative protection and preflights required historical bodies and non-genesis undo with `BasicFilterInputs::from_historical`. One SyncAll batch installs the reconciled state, fresh Active generation and covering lock map before new work can be minted.
- Genuinely absent activation preflights genesis through the durable tip, checks any live prune intent and delegates to the extracted guarded initializer. It atomically installs Empty checkpoint, Active generation zero and FromHeight(0) protection. Genesis body is required; undo is not.
- Added narrow public Rust methods only on `ManagedNetworkHandle<FjallChainstateStore, FjallCoinsView>`. Both acquire existing `mutate` authority; enable derives all fence/history facts from `store.inner()`. Neither accepts arbitrary ownership facts or adds configuration, RPC, timer or scheduler activation.
- Production runtime tests prove disable/re-enable, Disabled reopen with and without conservative protection, idempotence, stale record-only and checkpoint completion, missing body/undo nonmutation, safe Disabled intent finish and malformed Disabled owner refusal.

## Task Finalization Records

The strict wrapper defers all Git finalization. No task/TDD/metadata commit, push or hook bypass was performed.

1. Task 1, stop/invalidate before owned-lock release: implementation, seven storage lifecycle tests and real reopen matrix verified; pending consolidated root finalization.
1. Task 2, specialized host transitions and production reopen: implementation and six production lifecycle tests verified; pending consolidated root finalization.

STATE, ROADMAP, REQUIREMENTS and config remain root-owned. CFPR-01 completion remains pending whole-phase evidence.

## Verification

Bun 1.3.9 was selected from `/tmp/open-bitcoin-bun-1.3.9/bun-darwin-aarch64` and reprobed. Ad-hoc Cargo commands were timing-wrapped and serialized. No Bazel command was run by this plan.

| Check               | Command                                                                                                                                                                                 | Evidence                                                                                                                                                                                      |
| ------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Task 1 RED          | `bun run scripts/command-timings.ts run --key phase156-store-lifecycle-red -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --lib storage::fjall_store::filters`  | Expected exit 101: missing lifecycle methods and fault variants.                                                                                                                              |
| Storage GREEN       | `bun run scripts/command-timings.ts run --key phase156-store-disable-enable -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --lib storage::fjall_store::filters` | Initial 35/35 passed; final 39/39 passed after all store additions, 1,037 filtered out, test body 24.82s. Final timing duration 70.524s.                                                      |
| Task 2 RED          | `bun run scripts/command-timings.ts run --key phase156-runtime-lifecycle-red -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --lib filter_index`                 | Expected exit 101: missing specialized host methods. Also exposed invalid private raw-read calls in the new fixture; fixed using the existing state codec and a cfg(test) lifecycle accessor. |
| Runtime GREEN       | `bun run scripts/command-timings.ts run --key phase156-runtime-disable-enable -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --lib filter_index`                | First run 88/90 passed with two fixture assertion failures; corrected final run passed 91/91, 985 filtered out, test body 50.41s. Final timing duration 51.262s.                              |
| Source breadcrumbs  | `bun run scripts/check-parity-breadcrumbs.ts --check`                                                                                                                                   | Passed for 956 first-party Rust files. All four new paths registered using exact `git add -N` and matching in-source anchors.                                                                 |
| File lengths        | `bun run scripts/bright-builds-check.ts file-lengths`                                                                                                                                   | Passed, 1,275 scanned files, zero findings.                                                                                                                                                   |
| Source formatting   | Scoped `rustfmt --check --edition 2024 --config skip_children=true` on all nine owned Rust paths                                                                                        | Passed; unrelated children and other agents' edits preserved.                                                                                                                                 |
| Diff and call graph | `git diff --check`, owned diff review and lifecycle/initializer/live-writer caller search                                                                                               | Passed. Live writers remain token-gated; guarded initialization and lifecycle effects never reacquire the publication mutex.                                                                  |

Markdown verification with explicit GFM/frontmatter extensions is recorded in the self-check below. The final source was unchanged after the final 39/39 and 91/91 runs. Root owns full native Clippy/build/test/doctest/coverage/Bazel/claim checks and final hook verification.

## Fault and Reopen Observations

| Boundary      | Real reopen result                                                              |
| ------------- | ------------------------------------------------------------------------------- |
| BeforeDisable | Active generation zero, unchanged state and old lock.                           |
| AfterDisable  | Disabled generation one, unchanged state and old conservative lock.             |
| BeforeRelease | Disabled generation one with old conservative lock.                             |
| AfterRelease  | Disabled generation one with reserved lock absent; ordinary locks survive.      |
| BeforeEnable  | Disabled generation one and no newly acquired lock.                             |
| AfterEnable   | Active generation two with complete state and covering lock committed together. |

Each injected effect failure blocks subsequent live work minting in the same store until actual close/reopen. Retry completes the committed lifecycle without wrapping or needless generation churn. Max-generation transitions refuse without changing owner, state or locks.

The raw storage test compares actual saved state, immutable-record and active-projection bytes after disable/reopen. Production tests compare encoded checkpoint state, immutable record identities and exact validated payload/undo values. Re-enable may intentionally refresh the saved coins fence or reconcile the prefix; subsequent disable preserves that resulting state. Missing-history refusal additionally compares the entire real block-index snapshot after all handles close.

The recovered-coins reconciliation fixture retains the hidden immutable/projection suffix and installs the covering earlier protection. That storage fixture is codec-valid evidence, not consensus provenance. Production host tests separately use genuinely staged `ValidatedHistory` spend/undo facts. The resumed-intent test uses the explicitly labeled sparse deletion-order fixture.

## Decisions Made

Local AGENTS guidance, Bright Builds sidecar, placeholder-only overrides and architecture/code-shape/testing/verification/local-guidance/Rust standards informed this implementation. It adds no dependency, crate or synchronization authority.

Successful enable returns lifecycle completion. The existing crate-scoped opaque work loader then acquires the same guard, reloads Active ownership and verifies the fence before issuing a capability. Host callers cannot obtain or construct arbitrary token fields. The generation gate stops future issuance; no fictitious worker join or scheduler was introduced.

The guarded initializer and batch-finisher visibility changed only to sibling scope inside the owned publication module. Re-enable publishes through its explicit lifecycle batch after complete recovery/preflight, rather than attempting to bypass the Active-only worker checkpoint gate.

## Threat Mitigations

| Threat   | Concrete mitigation and evidence                                                                                                                                                                                              |
| -------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| T-156-08 | Checked durable Disabled generation precedes reserved release; both stale writers refuse before and after same-store re-enable. Only specialized parameter-free trusted host calls expose transitions.                        |
| T-156-09 | Real historical body/undo correspondence preflight, genesis/no-undo handling, complete checkpoint recovery and atomic Active/state/lock acquisition. Missing required body/undo preserves Disabled prefix and index snapshot. |
| T-156-10 | Six before/after lifecycle fault points extend the existing PublicationControl; actual close/reopen establishes the committed owner/protection truth and safe idempotent retry.                                               |
| T-156-11 | Existing managed authority precedes shared publication; guarded initialization/finish helpers do not reacquire it. Production transition/reopen suites complete with no second mutex or recursive locking route.              |

No unplanned network endpoint, authentication path or filesystem trust boundary was introduced. No unresolved HIGH finding was identified in the owned diff; whole-phase security review remains pending with root.

## Simplification Review

The implementation reuses one ownership model, the existing pure generation transitions, checkpoint scanner, historical input validator, publication mutex and paired-delete owner. Only lifecycle effects are added. There is no duplicated full-index scanner on prune candidates, new authority, fake accounting, swallowed error, name-derived authorization or new dependency. Complete validation remains at lifecycle recovery/release, with bounded work comparison unchanged.

## Deviations from Plan

The root approved the specialized host API returning completion and keeping opaque work issuance in the existing guarded loader instead of returning internal work through a public host signature. Protection is committed before that loader can return new work. No ownership model or security guarantee was changed.

The wrapper explicitly replaces atomic commits and shared state updates with pending root finalization records. No mutation outside the ten declared source/test/mapping paths and this summary was performed.

## Issues Encountered

- Task 2 RED also found attempted private raw-read calls in new sync fixtures. A cfg(test)-only lifecycle accessor and existing state codec replaced them without widening production read visibility.
- The first runtime behavioral run failed two new fixture assertions: `ValidatedHistory` stores empty genesis undo, and protected re-enable refreshes the saved fence from old B to current B. The no-undo test now deliberately removes only that fixture undo row before activation; the second disable compares against the state after successful re-enable. Final 91/91 passed; no production behavior fix was needed.
- Host paging delayed first linked-binary harness startup. Commands were polled within 60 seconds and timing/process evidence distinguished host delay from test execution. No build duplication, cache deletion, host process change or estimate-based termination occurred.

## Known Stubs

None. The scoped placeholder/TODO/FIXME/unimplemented scan found no production or test stub.

## Next Plan Readiness and Limits

The explicit production lifecycle and safe enable seam are ready for dependent protection/application plans and later activation adapters. Concrete poisoned deletion refusal, apply-time manual/automatic protection and measurement invalidation are still dependent Plans 04–06 responsibilities; this plan does not claim isolated proof of those paths. The existing concrete delete path is outside this plan's file ownership. Full fault/concurrency and daemon evidence continue in Plan 07.

No public configuration/RPC activation, scheduler, runtime reorg, filter serving or complete client-after-prune proof was added. No requirement or phase completion claim is activated by this summary.

## Self-Check: PASSED

All four declared new Rust files and this summary exist. Final storage and full filter-index commands passed with nonzero counts, 39/39 and 91/91. Source breadcrumbs, file lengths, scoped Rust formatting and diff whitespace checks passed. The summary was checked first, formatted only within its own path using explicit installed GFM/frontmatter extensions, and its final Markdown check passed. No commit existence claim applies: `commits: []` and root Git finalization are explicitly pending.
