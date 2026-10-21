---
phase: 166-canonical-durable-bootstrap-and-genesis-basic
plan: "07"
subsystem: runtime
tags: [canonical-genesis, earned-work, recovered-lineage, pre-effect-refusal]
requires:
  - phase: 166-06
    provides: Exclusive proved original store and consuming generation-bound genesis capability
provides:
  - Earned managed genesis work and complete coins binding for the four selectors
  - Restricted canonical network construction with earned header root and disabled activation
  - Canonical configured runtime startup without ordinary-history qualification
  - Sync refusal before reconciliation evidence changes
affects: [166-08, 166-09, 166-10, 166-11, 166-12, 166-13, 166-14]
tech-stack:
  added: []
  patterns: [sealed recovery consumption before startup writes, shared runtime finishing, immutable canonical admission]
key-files:
  created:
    - packages/open-bitcoin-node/src/chainstate/fjall_store/canonical_genesis.rs
    - packages/open-bitcoin-node/src/chainstate/fjall_store/canonical_genesis/tests.rs
    - packages/open-bitcoin-node/src/sync/canonical_genesis.rs
    - packages/open-bitcoin-node/src/sync/canonical_genesis/tests.rs
  modified:
    - packages/open-bitcoin-node/src/chainstate/fjall_store.rs
    - packages/open-bitcoin-node/src/network/canonical_admission.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/canonical_genesis.rs
    - packages/open-bitcoin-node/src/sync.rs
    - docs/parity/source-breadcrumbs.json
key-decisions:
  - Consume and recheck the original-store capability before initialize_configured or index writes, then construct core genesis from the complete coins view.
  - Keep earned core work authority independent of BASIC activation; append lineage requires the genuine current active BASIC proof.
  - Reuse existing runtime finishing while installing an earned header root through a crate-only constructor rather than fixture header import.
  - Refuse canonical sync at the shared entrypoint before resetting existing reconciliation evidence.
requirements-completed: [CBOT-02, CWOR-04]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 166-2026-10-18T22-36-40Z
generated_at: "2026-10-20T17:15:28Z"
commits: false
duration: 22 min recorded implementation and verification window
completed: 2026-10-20
---

# Phase 166 Plan 07: Earned Canonical Genesis Runtime Summary

**A sealed original store now becomes an earned genesis chainstate and header owner, while unsupported descendants and sync refuse before their effects.**

## Completion Boundary

Tasks: **2/2 complete**. The recorded window runs from the first retained RED at 2026-10-20T16:53:35Z through final checks at 2026-10-20T17:15:28Z; read-only dependency preparation is excluded. Nine implementation/evidence paths changed, plus this summary. No staging, commits, push, shared state/task edits or hook bypass occurred. Requirement frontmatter records this plan's contribution, not whole-phase closure.

Repo-local AGENTS guidance, the Bright Builds sidecar, architecture/code-shape/verification/testing/Rust standards and the no-concrete-exception override file informed execution. Active global and repository lessons were fully read: 8,856 bytes and 2,953 conservative estimated tokens, with no archive reads or omissions. The strict wrapper retains final native verification and normal-hook saving at root/166-14.

## Recovered Owners and Refusal Boundaries

`packages/open-bitcoin-node/src/chainstate/fjall_store/canonical_genesis.rs` consumes the restricted recovered capability, checks its actual instance/write generation before startup writes, initializes the existing lifecycle and constructs core genesis from the complete coins snapshot. Exact work, B and the empty genesis coins set agree for all selectors. Foreign stores, intervening identical-tip writes, changed tips and raw reconstructions refuse or remain untrusted. Active BASIC lineage binds the genuine current store/generation/branch and accepted genesis endpoint; a later unvalidated write fences its captured append proof.

Earned core genesis authority exists independently of indexing. Disabled and absent PreserveSaved startup do not initialize BASIC authority. Conditional active append lineage does not claim fresh canonical BASIC qualification, which remains 166-08's integration boundary.

`packages/open-bitcoin-node/src/network/canonical_admission.rs` adds only a crate-level constructor. It checks live core origin, selected parameters, exact genesis tip, coins B and network magic before installing `HeaderStore::from_canonical_genesis`, immutable canonical admission and default-disabled relay/block-relay/inbound flags. Mechanical append proof cannot promote reconstructed core state. Existing fixture factories and import restrictions remain intact.

`packages/open-bitcoin-node/src/sync/canonical_genesis.rs` checks config selection and consumes `OpenedCanonicalGenesis::into_parts()` before runtime writes. It reuses `finish_initialized_runtime` in `packages/open-bitcoin-node/src/sync/open_runtime.rs`; that file required no edit. This simplification avoids a second runtime engine. The shared sync entrypoint in `packages/open-bitcoin-node/src/sync.rs` now refuses before clearing reconciliation evidence. No worker starts during construction.

All-selector runtime tests compare chainstate, headers, peer state and durable rows around headers/body/compact/blocktxn dispatch through general, sync and durable-serving paths; stored/local connects, fixture-header import and reorg also refuse unchanged. The Signet refusal retains the missing challenge-validation reason. A positive fixture sync still reaches its resolver. Explicit daemon activation-request preflight/process evidence belongs to 166-09; no extra activation API was invented here.

## Automated Evidence

All receipts below are under `.local/open-bitcoin-dev/phase166/`. Cargo commands used pinned Bun, canonical TMPDIR and the exclusive phase166 Cargo target through `scripts/command-timings.ts`.

| Receipt | Actual outcome |
| --- | --- |
| `plan07-managed-red.log` | Exit 101: absent managed constructor; compilation RED, not executed assertion evidence. |
| `plan07-runtime-red.log` | Exit 101: absent canonical runtime opener; compilation RED. |
| `plan07-runtime-guard-state-mutation-red.log` | Exit 101: removing only the new guard erased prior Some(NoChange) reconciliation evidence to None before lower refusal. Guard restored. |
| `plan07-managed-verified.log` | 6 passed, 0 failed, including same/changed-tip fencing and active generation binding. |
| `plan07-runtime-verified.log` | 9 passed, 0 failed, including restored stateful guard and mechanical-proof refusal. |
| `plan07-canonical-network-regression.log` | 16 passed, 0 failed. Three new constructor tests overlap the runtime filter; counts are not additive distinct coverage. |
| `plan07-sync-regression.log` | 505 discovered: 501 passed, 0 failed, 4 ignored, completed in 622.49 seconds. |
| `plan07-clippy.log` | Node all-target/all-feature clippy with -D warnings: exit 0. |
| `plan07-fmt.log` | Workspace cargo fmt --all --check: exit 0. |
| `plan07-breadcrumbs.log` | 1,231 Rust files verified. |
| `plan07-managed-check.log` | 1,574 source files scanned; all managed checks report zero findings. |

The sync suite preserved four pre-existing ignores: public-network smoke, explicit phase157 timing measurement, explicit phase158 reorg measurement and a bounded subprocess child control. There is no live-network or actual-Knots differential claim. Focused filters also discover no matching tests in the separate phase165 integration binary; their positive counts above are library tests.

Intermediate receipts are retained honestly. `plan07-managed-complete.log` had 4 passing and 2 failing expanded tests because they initially acquired unbudgeted proofs for a budgeted append operation; using the existing production budget API fixed the test setup. `plan07-runtime-complete.log` had 4 passing and 2 failing tests due to incorrect test assumptions: unavailable block-serving evidence was treated as a flag getter, and the Signet diagnostic code uses underscores. Actual private activation flags are now asserted in the constructor seam tests.

`plan07-runtime-guard-mutation-red.log` initially passed because the fresh runtime had no prior reconciliation evidence and the lower guard also refused; it is **not** a RED claim. The intermediate `plan07-runtime-guard-mutation-strengthened-red.log` failed on validation ordering but was superseded by the stateful mutation above. Initial passing green/final receipts remain present alongside the final verified receipts. No failing receipt was erased or replaced with invented results.

## Plan Adjustments and Remaining Boundaries

Root-approved amendments added the narrow network constructor seam and removal of the staged into_parts dead-code expectation in `packages/open-bitcoin-node/src/storage/fjall_store/canonical_genesis.rs`. No other ownership expansion was needed. Existing ordinary-history, fixture and activation contracts are preserved; their relevant regressions pass.

The owned diff was reviewed and `git diff --check` passed. The simplification and stub/threat scans found no unresolved stubs, new dependency, endpoint, auth route or new filesystem trust surface. Supported-app lease and engine-lock exclusion inherit the existing contract, without claiming protection from arbitrary filesystem/direct-engine writers. Fresh enabled BASIC indexing, operator/daemon wiring, release-all-handles recovery scenarios, contributor docs and whole-phase closure remain with subsequent plans.

## Self-Check: PASSED

All created source files, the summary and cited final receipts exist. Final focused and regression tests, clippy, formatting, breadcrumb, managed and diff checks passed. No commits were requested or created by this executor.
