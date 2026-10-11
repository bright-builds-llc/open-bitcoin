---
phase: 161-shared-operator-index-and-retention-evidence
plan: "01"
subsystem: status
tags: [basic-filters, serde, operator-evidence, privacy]
requires:
  - phase: 160-explicit-bounded-peer-filter-serving
    provides: Existing BASIC capability and index authority
provides:
  - Typed additive BASIC operator status with unavailable legacy defaults
  - Migrated node, CLI, dashboard, soak and support snapshot constructors
affects: [161-02, 161-04, 161-05, 161-06, 161-10]
tech-stack:
  added: []
  patterns: [Independent field availability, fixed reason allowlist, validated inclusive bounds]
key-files:
  created:
    - packages/open-bitcoin-node/src/status/basic_index.rs
  modified:
    - packages/open-bitcoin-node/src/status.rs
    - packages/open-bitcoin-node/src/status/tests.rs
    - packages/open-bitcoin-node/src/status/tests/snapshot_projection.rs
    - packages/open-bitcoin-cli/src/operator/status.rs
    - packages/open-bitcoin-cli/src/operator/status/render/tests.rs
    - packages/open-bitcoin-cli/src/operator/status/tests/rendering_and_service.rs
    - packages/open-bitcoin-cli/src/operator/dashboard/model/tests.rs
    - packages/open-bitcoin-cli/src/operator/soak/tests/runtime/runtime_fixtures.rs
    - packages/open-bitcoin-cli/src/operator/support/tests/sync_fixtures.rs
    - docs/parity/source-breadcrumbs.json
key-decisions:
  - All uncollected BASIC facts default to unavailable, including configuration and initial sync.
  - The BASIC serde boundary normalizes arbitrary unavailable reasons to fixed safe labels.
  - Protection ranges use private validated bounds and reject reversed ranges during construction and deserialization.
requirements-completed: []
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 161-2026-10-11T04-48-29
generated_at: "2026-10-11T05:22:23Z"
duration: 7min evidence window
completed: 2026-10-11
---

# Phase 161 Plan 01: Shared BASIC Contract Summary

**Additive typed BASIC observations preserve unknown configuration, empty endpoints, earned synchronization, failures, retained growth and reserved protection independently.**

## Accomplishments

- Completed all three tasks across the eleven planned source/manifest paths. Added `basic_index` with serde defaults to the shared snapshot and migrated two node constructors, two production CLI constructors and five CLI fixtures.
- Implemented the exact fourteen public contract fields. Height and lag wire names omit internal `maybe_` prefixes; available absence remains JSON null, distinct from unavailable and actual genesis/zero lag.
- Added fixed operational, failure, retained-availability, guidance and logical-measurement enums; aggregate peer provenance counts; and validated inclusive protection bounds without identifiers or raw lock names.
- Preserved all existing support fixture private markers. The actual support snapshot-copy sanitizer retains captured BASIC facts and leaves the caller unchanged.

## Verification Evidence

Every Cargo command ran serially through `bun run scripts/command-timings.ts run --key KEY -- ...`, with `/tmp/open-bitcoin-bun-1.3.9-fresh` prepended to PATH. The executable reported 1.3.9, matching `.bun-version`. Advisory timing records are in the ignored `.local/open-bitcoin-dev/command-timings/KEY/` directories. Times below are actual wrapper duration, not performance promises.

| Key / check | Actual result | Duration |
| --- | --- | --- |
| `phase161-contract-red`: node `phase161_contract` | 1 discovered, 0 passed, 1 failed at `explicit BASIC facts`, establishing missing-section regression | 8,106 ms |
| `phase161-contract`: node `phase161_contract` | 10 discovered, 10 passed, 0 failed | 14,865 ms |
| `phase161-contract-integration`: node `phase161_contract` | Expanded suite: 12 discovered, 12 passed, 0 failed | 9,221 ms |
| `phase161-cli-contract`: CLI `cargo check --all-targets` | Passed, including all migrated literals | 10,335 ms |
| `phase161-cli-contract-support`: CLI `phase161_contract` | 1 discovered library test, 1 passed; other filtered targets discovered 0 | 15,907 ms |
| `phase161-contract-clippy`: node + CLI `--all-targets --all-features -- -D warnings` | Passed | 13,423 ms |
| `phase161-status-regression`: node `status::` | 81 discovered, 81 passed, 0 failed | 612 ms |
| `phase161-cli-status-regression`: CLI `operator::status::` | 74 discovered library tests, 74 passed, 0 failed; other filtered targets discovered 0 | 598 ms |
| Scoped Rustfmt (`--edition 2024 --config skip_children=true`) | Passed for all ten owned Rust paths | 251 ms |
| `bun scripts/check-parity-breadcrumbs.ts --check` | Verified 1,101 Rust files | Passed |
| `bun scripts/bright-builds-check.ts all` | 1,439 file-length inputs; 0 findings; active repo lessons 1,958 bytes / 653 estimated tokens; 0 findings | Passed |
| `git diff --check` and owned diff review | Passed; fixture edits remain mechanical except focused support/legacy tests | Passed |

The final contract suite covers partial legacy defaults, empty versus genesis endpoints and lag, earned initial sync with later lag and processed-ahead-of-durable, six detailed failures and observed no failure, untrusted reason normalization on both serde directions, protection range validation/absence/exhaustion, independent retained growth during failure, aggregate capability provenance and rejection of arbitrary guidance strings. The support test invokes the real existing sanitizer on a captured snapshot clone.

## Public API Decisions

- `BasicPeerScopeStatus` has `general_sessions`, `explicit_sessions`, `all_sessions`, `implicit_sessions` and `offered_sessions`, all `u64`; categories may overlap for a session.
- Operational variants are `Disabled`, `CatchingUp`, `Current`, `Lagged`, `Paused`, `Failed`, `Invalidated`. Failure variants match the plan exactly.
- Retained availability is `Observed` or `NoneSeen`. Measurement basis is `ImmutableRecordKeysAndValuesIncludingStaleBranches`; payload bytes are a subset of record key/value bytes. Both exclude active projections, owner/lifecycle/checkpoint/lock metadata and Fjall filesystem overhead.
- Protection is `None`, `InclusiveRange(BasicIndexProtectionRange)` or `HeightSpaceExhausted`. `BasicIndexProtectionRange::new(first, last)` is fallible; accessors are `height_first()` and `height_last()`.
- `BasicIndexOperatorStatus::default()` uses `BasicIndexUnavailableReason::NotReported` (`BASIC evidence not reported`). Stopped CLI collection uses `LiveUnavailable` (`live BASIC evidence unavailable`) for every BASIC fact, because its actual JSONC input proves no BASIC configuration.
- Optional field API names and serde names match the plan; no nullable-field deviation occurred.

## Threat Mitigations

| Threat | Evidence |
| --- | --- |
| T-161-01-1: identifier/raw backend disclosure | New available payloads contain only fixed enums, booleans, heights and aggregate counts. Every BASIC availability field uses a serde allowlist; hostile paths, endpoint text and ANSI markers are replaced, and arbitrary guidance is rejected. |
| T-161-01-2: false healthy/zero legacy semantics | The legacy round-trip enumerates all fourteen facts as unavailable. Tests distinguish available null, real zero and unavailable; initial synchronization and failure have independent availability. |

No new endpoint, authentication mechanism, storage access, network action or database schema was introduced. The new schema is within the plan's existing shared-snapshot trust boundary.

## Simplification and Guidance

The simplification pass retained one contract module and one generic serde adapter rather than duplicating sanitization per field or adding another resolver. Production contract code occupies 281 lines; the 583-line file also contains the focused tests. Only the new Rust path was added to the parity manifest, using pinned `index/base.cpp` and `index/blockfilterindex.cpp` anchors.

Material guidance loaded: repo `AGENTS.md`, Bright Builds sidecar and placeholder-only overrides; architecture, code shape, testing, verification, operability, local-guidance and Rust standards; finalized phase context/research/UI contract; and both complete active lesson inputs (7,188 bytes / 2,397 conservative tokens). Parent-owned synchronization, planning and tracker changes were preserved.

## Deviations from Plan

None. Extra scoped existing-status regression runs and a real support-sanitizer test strengthened evidence without expanding source ownership. All staging, commits, pushes and tracker mutations were deferred to the root clean gate as explicitly required.

## Known Stubs and Residual Limitations

No implementation stubs prevent this contract plan's goal. Production live BASIC facts intentionally remain explicitly unavailable until Plan 05 carries the authoritative network projection; Plans 02–04/10 own accounting, capability and runtime authority. Plan 06 owns human/dashboard/support rendering and broader redaction proof. This summary records only the CFOP-01 contract slice; full requirement closure, native verification, independent source/security review and formal lifecycle verification remain parent/Plan 09 responsibilities.

This plan adds no status-triggered activation, recovery, indexing, history scan, fault clearing, durability advance or protection release. It makes no new crash-window, archive-scale, production-readiness, public-serving or funds-safety claim; Phase 162 integrated proof remains pending.

## Task Commits

None by design. No files were staged or committed, and STATE/ROADMAP/REQUIREMENTS/todo were not mutated by this executor.

## Self-Check: PASSED

All eleven planned source/manifest paths and this summary exist. Nonzero targeted test discovery and successful focused checks are recorded above; the final owned diff passed whitespace review. Commit checks do not apply because the plan expressly prohibits nested commits.
