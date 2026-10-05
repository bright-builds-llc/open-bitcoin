---
phase: 157-safe-activation-and-scheduled-index-catch-up
plan: "07"
subsystem: node
tags: [rust, basic-filters, bounded-turns, measured-budgets, private-proof]
requires:
  - phase: 157-03
    provides: Ordered pure owner and checked work contracts
  - phase: 157-04
    provides: Sealed recovered manager and genuine own-flush authority
  - phase: 157-05
    provides: Budgeted consuming append and exact-tip safe release
  - phase: 157-06
    provides: Accepted next-height complete facts and retained error state
provides:
  - Actual configured startup first turn and ordinary parameter-free maintenance API
  - Pre-decode input admission and complete prefix-independent turn ledger
  - Measured production aggregate/singleton policy and real failure/reopen evidence
affects: [157-08, 157-09, 157-10]
tech-stack:
  added: []
  patterns: [upfront-private-proof, monotonic-budget-reduction, achieved-safe-bookkeeping]
key-files:
  created:
    - packages/open-bitcoin-node/src/network/runtime_authority/filter_index/catch_up.rs
    - packages/open-bitcoin-node/src/network/runtime_authority/filter_index/catch_up/inputs.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/filters/turn_inputs.rs
    - packages/open-bitcoin-node/src/sync/tests/filter_index/catch_up.rs
    - packages/open-bitcoin-node/src/sync/tests/filter_index/catch_up/fixtures.rs
    - packages/open-bitcoin-node/src/sync/tests/filter_index/catch_up/measurements.rs
    - packages/open-bitcoin-node/src/sync/tests/filter_index/catch_up/failures.rs
    - .planning/phases/157-safe-activation-and-scheduled-index-catch-up/157-TURN-MEASUREMENTS.md
  modified:
    - packages/open-bitcoin-node/src/chainstate/filter_index.rs
    - packages/open-bitcoin-node/src/sync/open_runtime.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/filters/ownership.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/filters/append.rs
    - packages/open-bitcoin-chainstate/src/filter_index/catch_up.rs
    - packages/open-bitcoin-node/src/sync/tests/filter_index/startup/configured.rs
key-decisions:
  - "Generate under the existing serialized authority with admitted work and measured hold time."
  - "Acquire one private capability before inputs, then only decrease remaining caps without changing identity or double-counting acquisition."
  - "Preserve actual stronger covering protection until a new exact durable-tip checkpoint is achieved."
  - "Retain txid-bearing BASIC body fields and genuine undo without witness copies."
requirements-completed: []
requirements-addressed: [CFIX-01]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 157-2026-10-05T14-50-48
generated_at: 2026-10-05T20:44:54Z
duration: 53min
completed: 2026-10-05
commits: []
git_finalization: pending root whole-phase verification and strict finalization
---

# Phase 157 Plan 07: Measured Ordered Runtime Turn Summary

**Configured recovered startup and the ordinary owned handle now advance bounded BASIC suffixes with pre-work admission, private achieved publication and measured prefix-independent costs.**

## Performance and Scope

- First registered RED: **2026-10-05T19:51:48.057Z**. Final checks completed by **20:44:54Z**, approximately **53 minutes**, excluding initial context loading.
- **7/7 amended tasks complete**: driver; deterministic suite; calibration; bounded inputs; pure achieved bridge/reexports; actual covering protection; two positive startup assertion migrations.
- **20 Rust paths**, including **7 new files**; **2 planning documents**. All new Rust files carry pinned breadcrumbs immediately. Plan 10 owns catalog registration.
- Pinned Bun 1.3.9 and the timing wrapper ran Cargo serially. No preserved old target was enumerated or changed. No Git command, staging, commit, push, shared STATE/ROADMAP/REQUIREMENTS/config change or requirement activation occurred.

## Implemented Behavior

Actual `open_with_configured_runtime_activation` retains Plan 04's sealed constructor, attaches pure progress only to its tracked genuine recovered manager, and executes one first bounded turn. Generic/public constructors and Clone remain untracked. The specialized `ManagedNetworkHandle<FjallChainstateStore, FjallCoinsView>::drive_basic_filter_index_turn()` takes no caller-selected source range, fence or height and returns immutable `BasicFilterTurnOutcome`.

The driver uses direct canonical positions, one retained next-height accepted facts object or bounded native body reads, and already-owned authoritative undo. It copies no chain or undo history. Native row length reserves scanner, decoded container and generation work before count loops or allocation. Malformed counts refuse before the allocating codec. Retained BASIC facts preserve the staged txid-bearing body/header and genuine complete undo while stripping witnesses; the retained logical allocation cap is **384 MiB**, with the existing **1,000,000 raw-script-item** cap. These are explicit adapter limits, not measured RSS or universal Bitcoin payload support.

One opaque budgeted proof is acquired **before inputs**, using the componentwise maximum of normal and singleton caps. Captured acquisition cost participates in admission. `with_remaining_budget` accepts only componentwise decreases that still cover captured work; it changes no capability identity. Driver-only costs are subtracted before tightening to the selected policy. Captured acquisition, selected blocks and actual encoded output each enter the merged achieved ledger once. Preparation and completion consume the same private graph and recheck store/lifecycle/branch/frontier/durability under publication.

Generation stays inside the existing authority for this implementation. Pure progress is trial-validated before effects and applied only after achieved append publication. A pure achieved-safe bridge preserves accepted targets and the initial-synchronization latch without creating a full fence. It grants no storage authority. Source failures preserve progress/facts; a restored source can resume only with unchanged genuine authority. Poisoned writer errors refuse before generation and require genuine reopen. The turn never forces coins flush or deletes payloads.

Safe release waits for the exact sealed durable tip. Intermediate processed suffixes retain the old safe checkpoint and actual stronger effective covering lock. A genuine later own flush can earn a bounded zero-record release. Continuous durable growth while behind can therefore prolong conservative retention.

## RED / GREEN / Simplification

| Boundary                      | Actual RED / discovery                                                                                                                                          | Final result                                                                                                                                                            |
| ----------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Startup driver                | Registered test failed compilation with missing ordinary turn method; exit 101.                                                                                 | Actual configured startup consumes the next height, with startup/stale controls passing.                                                                                |
| Absolute first candidate      | A genuine control returned an empty successful turn instead of absolute refusal.                                                                                | Explicit first-candidate refusal prevents no-progress spin; body/output exact and one-under controls pass.                                                              |
| Stronger protection           | Genuine lock-zero fixture observed protection 16 instead of the actual covering lock zero.                                                                      | Proof/outcome use effective protection; intermediate appends preserve it until new exact-tip achievement.                                                               |
| Accepted writer fault         | Initial test incorrectly expected poisoned BeforeUndo publication to resume in process.                                                                         | Corrected control proves refusal and complete retained facts; independent clean accepted-unflushed/later-flush control proves ahead processing and no-record release.   |
| Prefix fixture                | Initial explicit measurement failed at prefix 512 because the general fixture projection publisher caps an append at 128.                                       | Seed genuine prefixes through bounded publisher chunks; final 16/128/512 measurement passes.                                                                            |
| Positive startup expectations | Full regression initially passed 142/144 ordinary controls; two Plan 02 assertions expected pre-driver empty/old protection after actual first-turn completion. | Narrow migration proves exact earned checkpoint/protection, absent indexed old body/undo, and exact genesis-through-tip generated records; all negatives remain intact. |

Two early expected-counter/clone-reservation thresholds were corrected to actual complete observations, not claimed product-bug REDs. New helper/bridge tests added with their implementation have no separately claimed pre-implementation RED. The final monotonic-control test rejects inflation, discarded acquisition cost and unbudgeted conversion.

The simplification pass separates admission/generation from achieved publication, reuses existing mutexes, staging and reducers, and splits failure tests into one child. Thin registry wrappers follow the repo's existing formatter pattern; every touched source stays at most 628 lines. No dependency, crate, schema, full-history cache, global permit, fake call or warning suppression was added. The immutable handle progress observer is test-only; production obtains progress from the actual outcome.

## Verification

All Cargo checks used `--manifest-path packages/Cargo.toml`, pinned Bun and `scripts/command-timings.ts`. Counts overlap and are not a distinct-test sum.

| Actual check                                                                                                          | Result                                                                                                                    |
| --------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------- |
| Node `test --lib filter_index`, final source and migrated startup assertions                                          | **145 passed**, 0 failed, **1 intentionally ignored measurement**, **94.27s**                                             |
| Node `test --lib phase157_turn_`, after upfront proof/reduction and before the additional test-only monotonic control | **15 passed**, 0 failed, **27.13s**; final filter suite additionally includes the new monotonic control                   |
| Node `test --lib phase157_measure_turns -- --ignored --nocapture`, final production source                            | **1 passed**, **18.65s** harness; **25.409s** wrapper, **20:36:57.917–20:37:23.326 UTC**                                  |
| Node `test --lib phase157_proof_`, final source                                                                       | **36 passed**, 0 failed, **21.90s**                                                                                       |
| Node `test --lib phase157_append_`, achieved effective-protection implementation                                      | **35 passed**, 0 failed, **18.20s**; subsequent proof bookkeeping adds no change to existing append identity/effect logic |
| Pure chainstate `test --lib filter_index`, including three new achieved-bridge controls                               | **99 passed**, 0 failed, **0.00s** harness                                                                                |
| Node `clippy --all-targets --all-features -- -D warnings`, final source                                               | **Passed**, **4.43s**; staged normal-lib warning graph has actual production consumers                                    |
| Workspace `cargo fmt --all -- --check`, final source                                                                  | **Passed**                                                                                                                |

Root reported a clean diff whitespace check during execution; final diff/native verification, coverage, Bazel, lifecycle/source/security review and Git finalization remain root-owned. The two historical constructor-source assertions and Phase 155 Bun anchor migration remain Plan 10-owned; no fake constructor call/comment was added. No Cargo, rustc or node test process remained at the **20:44:54 UTC** handoff check.

## Measurement and Policy Handoff

Exact command, sample values, cold/warm definition, fixture provenance and limitations are in [157-TURN-MEASUREMENTS.md](157-TURN-MEASUREMENTS.md). Final whole eight-block subsequent turns have **70 record operations**, **36 projection operations**, **5,415 checkpoint/map/undo structural units**, **40 actual indexed point reads**, **8 actual body reads/decodes**, **4.507 MB logical copy reservation**, **1,936,752 item-work** and **17.400 MB byte-work** reservations. Actual examined facts are **40 scripts / 65,568 bytes**; actual hashed/sorted unique elements are **16 / 16**. Undo is borrowed, with zero turn-side undo wire reads. Reservations are not measured comparisons/RSS; record-operation categories are not filesystem syscall counts.

Chosen normal defaults are **8 blocks**, **1 MiB body**, **4 MiB undo**, **16 MiB copy/allocation**, **4 Mi item-work**, **32 Mi byte-work**, **1 MiB encoded output**, **512 record operations**, **1,000,000 checkpoint/map/structural units** and **256 projection operations**. The separate absolute singleton and codec ceilings are documented in the measurement artifact and consumed by production. A genuinely validated **989,871-byte** large singleton progressed under these defaults with **65,560,780** copy reservation and **43.230ms** measured hold, including **5.098ms** append completion. This proves that concrete legal case, not every consensus-legal Bitcoin output representation or storage latency.

Plan 08 can call the public parameter-free ordinary method from the existing elapsed maintenance body; configured open already performs the first turn. `BasicFilterTurnOutcome` is reexported from the node and contains achieved work, batch bytes/count, body/undo/generation observations, authority/completion durations and optional immutable progress. The runtime caller must yield/wait between turns and retain the existing Periodic/Always coins/automatic-prune owner. Full startup/preflight is chain-wide; the serialized turn has bounded admitted work but no hard wall-clock/fsync guarantee.

## Deviations and Threat Review

Root approved the extra bounded-input child/registration, pure achieved bridge/reexports, actual protection getter/outcome, failure-test child and narrow positive-startup migration as Tasks 4–7. Rule 1 fixes cover absolute empty-turn behavior and effective-lock retention. The fixture/counter/poison expectation corrections above do not invent bug evidence. Strict wrapper instructions defer all commits and shared state activation to root.

AGENTS, its Bright Builds sidecar, placeholder-only overrides and architecture/code-shape/testing/verification/local-guidance/Rust standards informed the implementation. Active global/repository lessons were loaded within their **7,188-byte / 2,397-estimated-token** budget; no archives or new audit trigger were used. Project skill directories are absent.

T-157-20 is covered by sealed proof identities, trial/achieved completion and lifecycle/source/poison controls. T-157-21 is covered by pre-scanner/pre-allocation admission, monotonic complete accounting, legal singleton and exact/one-under/absolute controls, prefix 16/128/512 and a real full-ancestry reader control whose observed reads exceed the bounded turn. T-157-22 is covered by actual registered final-source measurements with numeric observations and explicit software/storage limits. The new decoder preflight uses the existing private BlockIndex trust boundary; no new endpoint, authentication path, schema or filesystem namespace is introduced.

No goal-blocking stub was found. Known limitations are the existing first-party 10,000-byte ScriptBuf representation boundary, explicit adapter/map resource refusal, software rather than hardware power-loss faults, serialized generation/storage hold, and conservative exact-tip retention. No public-mainnet, archive-scale, production/funds or whole-phase completion claim is made.

## Task Commits and Self-Check

Tasks 1–7 and metadata are **pending root finalization** after the whole-phase gate; no hashes are claimed. `requirements-completed` remains empty. All 20 source paths and both planning artifacts exist, all seven new Rust files are registered and breadcrumbed, final nonzero evidence appears above, no goal-blocking placeholder exists, and final source lengths are within 628. Commit-existence checks are inapplicable under the explicit root deferral.

## Self-Check: PASSED
