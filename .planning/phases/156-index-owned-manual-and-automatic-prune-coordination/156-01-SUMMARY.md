---
phase: 156-index-owned-manual-and-automatic-prune-coordination
plan: "01"
subsystem: chainstate
tags: [basic-filter, lifecycle, prune-protection, ownership, compatibility]
requires:
  - phase: 155-recoverable-index-and-pre-prune-startup-protection
    provides: Immutable BASIC identities, verified coins fences and direct input protection
provides:
  - Pure checked lifecycle generations and validated effective ownership snapshots
  - Exact generation, checkpoint, fence and protection work comparisons
  - Additive bounded 11-byte persisted owner envelope
affects: [156-02, 156-03, 156-05, 156-06, 156-07]
tech-stack:
  added: []
  patterns: [pure lifecycle facts, immutable ownership snapshots, fixed-size owner codec]
key-files:
  created:
    - packages/open-bitcoin-chainstate/src/filter_index/lifecycle.rs
    - packages/open-bitcoin-chainstate/src/filter_index/tests/lifecycle.rs
    - packages/open-bitcoin-node/src/storage/filter_index/ownership.rs
  modified:
    - packages/open-bitcoin-chainstate/src/filter_index.rs
    - packages/open-bitcoin-chainstate/src/filter_index/tests.rs
    - packages/open-bitcoin-node/src/storage/filter_index.rs
    - packages/open-bitcoin-node/src/storage/filter_index/tests.rs
    - docs/parity/source-breadcrumbs.json
key-decisions:
  - Missing owner plus valid saved state is legacy Active generation zero; only complete artifact absence is legacy absent.
  - Work identity includes exact saved checkpoint commitments, saved fence, prepared fence, lifecycle and effective protection.
  - Disabled retains valid saved state and may retain conservative protection independently of work issuance.
patterns-established:
  - Public pure ownership facts confer no storage write capability.
  - Owner decoding rejects corruption before typed use without changing existing v1 envelopes.
requirements-completed: []
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 156-2026-10-04T20-28-36
generated_at: 2026-10-04T21:19:27Z
commits: []
git_finalization: pending consolidated root commit after clean whole-phase verification and full native gate
duration: 15min from first recorded TDD command
completed: 2026-10-04
---

# Phase 156 Plan 01: Index Ownership Contracts Summary

**Checked BASIC lifecycle generations and exact work identities with a compatible 11-byte owner envelope.**

## Performance

- First recorded TDD command: 2026-10-04T21:04:46.997Z.
- Completed targeted execution: 2026-10-04T21:19:27Z.
- Tasks completed: 2/2.
- Source, test and mapping files changed: 8; summary added separately.
- Both active lesson files were read completely: 5,230 global bytes plus 1,958 repository bytes, 2,397 conservative estimated tokens.

## Accomplishments

- Added `IndexGeneration` with checked increment and typed exhaustion refusal; disable increments Active, Disabled retries are idempotent, and re-enable increments Disabled.
- Added `EffectiveIndexOwnership` validation for genuine artifact absence, legacy Active generation zero, explicit Active and explicit Disabled. Missing state, malformed reserved locks and insufficient protection refuse.
- Added immutable checkpoint/fence identity and pure work comparison. A matching generation alone cannot accept an older frontier, different branch/fence, changed protection or disabled work.
- Preserved direct genesis/height-one input protection and explicit height exhaustion, using the existing protection and buffered-lock contracts without altering ordinary lock arithmetic.
- Added `basic_filter:v1:owner` codec: version 1, BASIC type 0, mode Active=0/Disabled=1, little-endian u64 generation, exactly 11 bytes. Unknown fields, every truncation and trailing data yield BlockIndex corruption with Repair guidance. Existing record/state/projection codecs and schema-2 coins were not changed.

## Task Finalization Records

The strict phase wrapper defers Git finalization. No task, RED, GREEN, metadata commit, push or hook bypass was performed.

1. Task 1, pure lifecycle transitions and full work identity: implementation and tests verified; pending consolidated root finalization.
1. Task 2, additive bounded persisted owner envelope: implementation and tests verified; pending consolidated root finalization.

STATE, ROADMAP, REQUIREMENTS and config updates remain root-owned. CFPR-01 is not activated by this summary.

## Verification

Pinned Bun 1.3.9 was selected from `/tmp/open-bitcoin-bun-1.3.9/bun-darwin-aarch64` and reprobed. Commands below ran through that executable; all ad-hoc Cargo work used the timing wrapper.

| Check              | Command                                                                                                                                                                    | Evidence                                                                                                  |
| ------------------ | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------- |
| Task 1 RED         | `bun run scripts/command-timings.ts run --key phase156-lifecycle-core-red -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-chainstate --lib filter_index` | Expected exit 101: unresolved lifecycle module before implementation.                                     |
| Task 1 GREEN       | `bun run scripts/command-timings.ts run --key phase156-lifecycle-core -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-chainstate --lib filter_index`     | Initial 35/35 passed; final run after edge-case tests passed 42/42, including 17 lifecycle tests.         |
| Task 2 RED         | `bun run scripts/command-timings.ts run --key phase156-owner-codec-red -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --lib storage::filter_index` | Expected exit 101: unresolved ownership module before implementation.                                     |
| Task 2 GREEN       | `bun run scripts/command-timings.ts run --key phase156-owner-codec -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --lib storage::filter_index`     | 13/13 passed, including five additive ownership/projection tests and existing frozen-v1 controls.         |
| Source breadcrumbs | `bun run scripts/check-parity-breadcrumbs.ts --check`                                                                                                                      | Passed for 950 discovered first-party Rust files; all three new paths registered with exact `git add -N`. |
| Formatting         | Scoped `rustfmt --edition 2024` and `rustfmt --check` on owned Rust files                                                                                                  | Passed; module entry files formatted with `skip_children=true` to preserve unrelated children.            |
| Diff review        | Owned diff review and `git diff --check`                                                                                                                                   | Passed; no unrelated source, dependency, schema or ordinary-lock formula change.                          |

A final node-test import shortening and module ordering formatting followed the node GREEN run; behavior is unchanged and the root native gate will compile the final consolidated source.

No real Fjall write, close/reopen or fault-injection test was performed by this pure-policy/codec plan. Those persistence and production-caller observations belong to the dependent phase plans. Targeted passes establish these contracts only; the root full native verifier, coverage, Bazel, lifecycle review and clean final hook remain pending.

## Decisions Made

Local AGENTS guidance, Bright Builds sidecar, placeholder-only overrides and architecture/code-shape/testing/verification/local-guidance/Rust standards informed the implementation. The functional core contains no I/O; the codec reuses the existing cursor and bounded diagnostics. `maybe_` names advertise optional success values and fields.

The validated snapshot separates saved checkpoint protection from effective durable protection. This represents Disabled with extra conservative retention without admitting Active with missing required protection. Comparing the complete snapshot also invalidates work when the exact saved frontier/fence or effective reserved range changes.

## Threat Mitigations

| Threat   | Concrete mitigation and evidence                                                                                                                                                                                                                                                          |
| -------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| T-156-01 | Fixed length 11 before parsing, existing version/type validation, exact mode parsing; codec tests exercise both modes, zero/max generations, unknown fields, every truncation and trailing bytes.                                                                                         |
| T-156-02 | Checked lifecycle arithmetic and exact immutable snapshot plus prepared-fence comparison; tests refuse disabled, different-generation, newer-frontier and changed-fence/protection work. Public facts grant no write capability; the opaque adapter token remains Plan 02 responsibility. |
| T-156-03 | All-artifact absence check, legacy Active generation zero and covering protection validation; pure and codec tests refuse orphan owner/lock/artifacts, weak/malformed protection and decoded owner without state.                                                                         |

No additional unplanned endpoint, authentication path or filesystem trust boundary was introduced. No unresolved HIGH finding was identified in the owned diff; whole-phase security review remains pending.

## Simplification Review

One immutable ownership model reuses existing checkpoint and direct input-protection rules. The 41-line owner codec reuses the v1 cursor and allocates no encode buffer dynamically. No full-index scanner, separate authority, dependency, crate, new ordinary-lock formula or name-inferred authorization was added. The storage publication guard and paired-delete owner are intentionally consumed by dependent plans.

## Deviations from Plan

None in implementation scope. The plan explicitly replaces atomic task/TDD commits and state updates with pending consolidated root finalization; this summary follows that requirement.

## Issues Encountered

- Cursor rust-analyzer independently ran `cargo check --workspace --all-targets`, holding the Cargo artifact lock. Process identity and progressing live compiler children were observed, and the timed commands were polled within 60 seconds. The waits were not treated as timeouts and no editor/build process was interrupted.
- The node GREEN run reports `OWNER_KEY` unused because Plan 02 has not yet wired storage ownership. No warning suppression was added; the dependent integration must consume the key before the strict native gate.

## Known Stubs

None. The scoped scan found no TODO/FIXME, placeholder output or unimplemented production body in added/modified code. Adapter integration is a declared dependent plan rather than a substitute for this plan's implementation.

## Next Plan Readiness

Plan 02 can consume `chainstate::filter_index::lifecycle` and `storage::filter_index::ownership` for guarded owner loading, opaque store-bound work tokens and actual deletion gates. Public activation, scheduler, reorg, RPC and peer filter surfaces remain later-phase work.

## Self-Check: PASSED

All three declared new Rust files and this summary exist. Both targeted GREEN commands passed with nonzero test counts, and source mapping, formatting and whitespace checks passed. No commit existence claim applies: `commits: []` and consolidated Git finalization remain explicitly pending.
