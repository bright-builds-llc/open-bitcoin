---
phase: 156-index-owned-manual-and-automatic-prune-coordination
plan: "06"
subsystem: retention
tags: [basic-filter, automatic-prune, protection, memoization, fjall]
requires:
  - phase: 156-05
    provides: Validated immutable protection snapshots and authoritative concrete deletion gates
provides:
  - Fresh normalized ownership identity before both automatic measurement gates
  - Direct required-input candidate exclusion with unchanged logical usage
  - Same-second lifecycle, checkpoint, failure and durable receipt evidence
affects: [156-07, 156-08, 157]
tech-stack:
  added: []
  patterns: [protection-keyed measurement, shared invalidation, explicit fixture accounting]
key-files:
  created:
    - packages/open-bitcoin-node/src/network/runtime_authority/automatic_prune/tests/protection.rs
  modified:
    - packages/open-bitcoin-node/src/network/runtime_authority/automatic_prune.rs
    - packages/open-bitcoin-node/src/network/runtime_authority/automatic_prune/tests.rs
    - packages/open-bitcoin-node/src/network/runtime_authority/automatic_prune/tests/fixtures.rs
    - docs/parity/source-breadcrumbs.json
key-decisions:
  - Reuse the complete immutable snapshot identity and normalize only lock ordering.
  - Remove required heights from candidate sizes while preserving actual total retained bytes.
  - Suffix protection requires separate valid protected-low and safe-higher controls.
patterns-established:
  - Ownership changes invalidate both gates without depending on handle callbacks.
  - Errors and network reset clear completed key, timer and last measured identity together.
requirements-completed: []
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 156-2026-10-04T20-28-36
generated_at: 2026-10-05T00:32:24Z
commits: []
git_finalization: pending consolidated root commit after clean whole-phase verification and full native gate
duration: 15min from first recorded TDD command
completed: 2026-10-05
---

# Phase 156 Plan 06: Protection-Keyed Automatic Measurement Summary

**Fresh ownership and normalized locks invalidate automatic reuse and Periodic timing immediately, while required payload bytes stay honestly counted and ineligible for deletion.**

## Performance

- First recorded TDD command: 2026-10-05T00:17:34.946Z.
- Final targeted completion: 2026-10-05T00:31:22.166Z.
- Tasks completed: 2/2; finalization remains root-owned.
- Five source/test/mapping paths changed; this summary is separate.
- Both active lessons read fully: 7,188 bytes and 2,397 conservative estimated tokens. No audit trigger applied.

## Accomplishments

- Eligible automatic turns load validated current protection before both completed-key equality and the independent 60-second Periodic throttle. Equality includes lifecycle/generation, exact checkpoint/fence, saved/effective protection and normalized current locks. Changed protection clears the completed key, measurement timestamp and last measured identity; unchanged identities retain coalescing.
- The same invalidation helper clears all three fields after preparation, accounting, post-flush revision or flush errors and network reset. Payload-only changes still use the existing 60-second coalescing. Always still measures, and cheap None/mode/target/short-chain/no-tip gates remain before accounting.
- Directly required heights, including zero and one, are removed from measured candidate sizes before planning. Actual current_usage_bytes remains intact, so required bytes cannot falsely pay the deletion budget or force an unnecessary Always coins checkpoint. Existing ordinary lock buffering remains unchanged.
- Added six counted policy tests and three real Fjall/production-host tests. The real-backed TestStore forwards actual snapshot/accounting; no production accounting override or lower target was introduced. Plan 05's explicit no-index fixture migration and the root debugger's writer allocator/regression were preserved.

## Task Finalization Records

The strict wrapper defers every task/TDD/metadata commit. No commit, push or hook bypass occurred.

1. Task 1: six policy cases added before production changes; RED reproduced five missing behaviors with 27 passing controls. GREEN passed 34/34 after the policy and first two real proofs.
1. Task 2: real clone lifecycle/progress, failed-publication/reopen and preplanned/safe-receipt evidence complete. Final planned default-parallel sweep passed 35/35. The real proofs extend the already implemented contract; no artificial separate failing test is claimed for them.

STATE, ROADMAP, REQUIREMENTS and config remain root-owned. CFPR-01 is not activated by this summary.

## Verification

Pinned Bun 1.3.9 was selected and reprobed from `/tmp/open-bitcoin-bun-1.3.9/bun-darwin-aarch64`. All three Cargo commands were timing-wrapped, serialized and used default test parallelism. No Bazel command ran in this plan.

| Check                | Exact command                                                                                                                                                                   | Evidence                                                                                                                |
| -------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------- |
| Policy RED           | `bun run scripts/command-timings.ts run --key phase156-auto-protection-policy-red -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --lib automatic_prune` | Expected exit 101: 27 passed, five new failures, 1,082 filtered, 4.59s body, 470.001s total.                            |
| Policy GREEN         | `bun run scripts/command-timings.ts run --key phase156-auto-protection-policy -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --lib automatic_prune`     | 34/34 passed, 1,082 filtered, 6.24s body, 84.466s total.                                                                |
| Final races/controls | `bun run scripts/command-timings.ts run --key phase156-auto-protection-races -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --lib automatic_prune`      | 35/35 passed, 1,082 filtered, 4.89s body, 103.836s total. Includes all five debugger-preserved writer controls.         |
| Source provenance    | `bun run scripts/check-parity-breadcrumbs.ts --check`                                                                                                                           | Final pass for 963 Rust files. New path mapped and in-source breadcrumbs synchronized; exact git add -N inventory only. |
| Managed checks       | `bun scripts/bright-builds-check.ts all`                                                                                                                                        | Passed: 1,282 source files, repository lesson structure, zero findings.                                                 |
| Source formatting    | `rustfmt --check --edition 2024 --config skip_children=true` on four owned Rust paths                                                                                           | Passed; unrelated module children and writers.rs preserved.                                                             |
| Diff review          | Owned diff review and `git diff --check`                                                                                                                                        | Passed; no dependency, target, scheduler, configuration or ordinary lock formula change.                                |

Final Rust source did not change after the 35-test run. Full native Clippy/build/test/doctest/coverage/Bazel/provenance and lifecycle verification plus the final hook remain mandatory root gates. Scoped Markdown evidence is recorded below.

## Persistence, Clone and Fault Observations

- Counted tests prove same-second ordinary lock strengthening/release, same-range generation, Active/Disabled mode and fence identity changes. Order-only map changes coalesce even outside the timer interval. Protection/accounting errors recover with a same-second new scan. Under-target protection changes do not alter independent next_write timing; unchanged Always and network reset controls pass.
- Real clone acquisition, genuine checkpoint progress, external cloned-store disable and trusted production host re-enable cause five scans at the same injected second with unchanged payload revision. The following unchanged turn coalesces. Actual usage, required body/undo, have-pruned and zero earned support remain intact before any deletion.
- An actual BeforeCheckpoint publication failure poisons the live protection loader, so an otherwise reusable same-second measurement refuses without another scan or payload/intent/receipt effect. All real bytes survive. After every live handle/token is dropped, actual production reopen restores conservative empty-prefix protection and identical total usage.
- A candidate prepared before ownership transitions skips after protected re-enable. Genuine fenced prefix publication then permits a separate legal higher height-2 pair through the existing production manual owner. Body and undo disappear together, exactly one height receipt is earned, height-20 body/undo remain, and production reopen preserves the deletion and receipt.
- Existing channel-ordered in-flight/overlapping writers, partial-effect errors and poisoned payload controls pass unchanged under default parallelism. New real roots use atomic create_dir reservation with suffix retry, preserving the debugger's collision lesson.

The dense real fixture has historical coinbase inputs and metadata through height 1,001. It is deletion-order evidence, not consensus-validated sync. Actual automatic accounting is below the unchanged legal 550 MiB target; real automatic deletion under target pressure remains Plan 07's required proof. Counted above-target policy fixtures supplement actual accounting rather than overriding it. No hardware power-loss, public activation, scheduler, reorg or complete client-after-prune claim is made.

## Decisions and Simplification Review

Repo-local AGENTS guidance, Bright Builds sidecar, placeholder-only overrides and architecture/code-shape/testing/verification/Rust standards informed this work. The active phase's root owns sync, tracking and finalization.

One existing immutable ownership model supplies both gates. Only ordering is normalized; ranges, names and protection are preserved. One invalidation helper handles all applicable resets. The snapshot's short publication guard is released before payload revision/measurement and nested flush effects; the existing publication-before-payload concrete owner remains authoritative after clone changes. Complete filter forest scans stay at startup/publication. No new synchronization authority, dependency, crate, deletion owner, capability, full-index scanner or swallowed error was added.

Direct protection is a suffix. A valid snapshot protecting zero/one necessarily protects every higher input, so protected-low exclusion and higher-safe release use separate valid snapshots. Root clarified this in the plan rather than admitting impossible synthetic ownership facts.

## Threat Mitigations

| Threat   | Evidence                                                                                                                                                                                                                         |
| -------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| T-156-19 | Fresh full normalized snapshot comparison before both gates; same-second mode/generation/fence/lock tests, error reset and order-only coalescing pass.                                                                           |
| T-156-20 | Real external clone lifecycle/checkpoint publication is observed without handle callbacks; production host transition, failure/reopen and unchanged channel writer controls pass. Guards are released before nested effects.     |
| T-156-21 | Required entries cannot pay the candidate budget; actual total remains counted, protected low plan stays empty without forced coins, real bytes/mates survive stall/failure and only a safe actual paired receipt earns support. |

No unplanned endpoint, authentication path, schema or filesystem trust boundary was introduced. No unresolved HIGH finding was identified in the owned diff; whole-phase review remains root-owned.

## Deviations and Issues Encountered

- Root explicitly reserved writers.rs for the debugger's allocator repair; this executor read it and included its tests without editing it. The root's plan clarifies separate valid low-protection/higher-safe controls.
- RED failed only on the five expected missing behaviors. Both GREEN commands passed without a corrective behavioral iteration.
- The first compile took 4m57s; the newly linked harness then waited before executing its tests. Process status and wrapper heartbeats established continued liveness. Commands were polled within 60 seconds; no process/cache/host change or elapsed-time termination occurred, and no concurrent Cargo build was started.

## Known Stubs

None introduced. Synthetic facts are explicitly test-only policy evidence, real-backed fixtures forward actual storage, and unsupported sinks retain their explicit refusal. Full phase evidence and activation remain dependent work.

## Next Plan Readiness

Plan 07 may consume the fresh gates for legal-target real automatic deletion and broader lifecycle fault closure. This plan does not execute Plan 07 or activate requirement completion. Consolidated Git finalization remains pending the clean whole-phase and native gates.

## Self-Check: PASSED

The declared new Rust path and this summary exist. Final default-parallel verification passed 35/35 with nonzero matching counts; source provenance, managed checks, all four scoped Rust formatting checks and diff whitespace passed. The owned Rust stub scan found no TODO/FIXME/placeholder/unimplemented body. The summary was checked before formatting with installed explicit GFM/frontmatter extensions, then scoped formatting and its final Markdown check passed. No commit-existence claim applies: commits are empty and consolidated root finalization is pending.
