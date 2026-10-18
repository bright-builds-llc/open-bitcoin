---
phase: 165-shared-exact-work-authority
plan: "02"
subsystem: storage
tags: [proof-work, snapshot-codec, legacy-classification, parity]
requires:
  - phase: 165-01
    provides: Existing ProofWork lossless 32-byte little-endian import and export
provides:
  - Tagged versioned exact-number and legacy-synthetic work records
  - Explicit historic bare-u128 classification without accepted lineage
  - Strict malformed-record refusals and fixed-width boundary tests
affects: [165-03, 165-10, 165-13, 165-14]
tech-stack:
  added: []
  patterns: [Numeric representation separated from validation provenance, strict boundary parsing]
key-files:
  created:
    - packages/open-bitcoin-node/src/storage/snapshot_codec/work.rs
    - packages/open-bitcoin-node/src/storage/snapshot_codec/work/tests.rs
  modified:
    - packages/open-bitcoin-node/src/storage/snapshot_codec.rs
    - packages/open-bitcoin-node/src/storage/snapshot_codec/tests/chainstate_wallet_and_headers.rs
    - docs/parity/source-breadcrumbs.json
key-decisions:
  - "ExactNumber preserves all 256 bits but never asserts accepted lineage."
  - "Historic bare integers and tagged synthetic scores remain LegacySynthetic(u128)."
  - "Use RawValue and direct u128 parsing to avoid lossy serde_json::Value intermediates."
  - "Work record version 1 is independent of the unchanged global Fjall schema."
patterns-established:
  - "Strict record shapes refuse unknown, duplicate and conflicting fields."
  - "Codec-only public APIs remain usable before the atomic live carrier migration."
requirements-completed: [CWOR-01, CWOR-05]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 165-2026-10-14T16-09-59Z
generated_at: 2026-10-14T17:08:49Z
duration: 5min
completed: 2026-10-14
---

# Phase 165 Plan 02: Lossless Work Records and Legacy Classification Summary

**Strict tagged 256-bit work records preserve every byte while historic u128 scores remain explicitly untrusted legacy data.**

## Performance

- Implementation start: approximately 2026-10-14T17:02:09Z, measured from the new module's creation time; preliminary context loading was not separately timed.
- Final focused verification: 2026-10-14T17:07:05Z, with regression completion and checker results immediately afterward.
- Duration: approximately five minutes of implementation and verification.
- Tasks: 2/2.
- Source/test/breadcrumb files created or modified: 5; this summary is separate.

## Accomplishments

- Added `snapshot_codec::work` and public re-exports of its classified number and codec entrypoints, without changing any live snapshot carrier or current schema.
- Exact tagged records roundtrip mixed limbs, byte31, ZERO, MAX and values above 128 bits using existing `ProofWork` byte APIs. An independent fixed JSON byte array checks the expected unsigned hexadecimal order.
- Bare historic JSON scores 0 and max-u128, plus tagged legacy max-u128, retain `LegacySynthetic`. The existing header DTO reader/writer remains compatible, and a new test explicitly classifies its unchanged bare score 2.
- Unknown/missing/contradictory tags, unsupported versions, extra/duplicate fields, negative/floating/overflow legacy numbers, mixed exact/legacy fields, wrong byte lengths and out-of-range bytes refuse. Direct record decoding rejects inputs above 512 bytes before JSON parsing and returns bounded `StorageError::Corruption` diagnostics in the supplied namespace.
- Serde support allows the classified record to become a field in Plan 03 DTOs. Embedded exact and legacy classifications roundtrip; the classifier also refuses oversized embedded shapes after raw extraction.
- Existing wallet, chainstate and header snapshot regressions remain intact. No fixture identity, historical receipt or source datadir changed.

## Task Commits

Task 1, Task 2, RED/GREEN/REFACTOR commits and metadata finalization are **deferred to the parent strict finalization wrapper** under the approved execution contract. This executor staged nothing and created no commits, pushes, worktrees or hook bypasses. Commit hashes and normal-hook receipts are not asserted. Shared STATE.md, ROADMAP.md, REQUIREMENTS.md, config and todo were not edited.

## Executed Evidence

All Cargo commands ran sequentially through the checkout's command-timings cooperative lock. Each command selected Bun 1.3.9 with this command-local prefix:

```bash
PATH='/Users/peterryszkiewicz/.bun/install/cache/@oven/bun-darwin-aarch64@1.3.9@@@1/bin:'"$PATH"
```

Apply that prefix to each command below when the shell selects another Bun version.

| Check                                                                                                                                                                                                     | Actual result                                                                                       |
| --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------- |
| Task 1 RED: `bun run scripts/command-timings.ts run --key phase165-work-codec -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --all-features phase165_work_codec`                  | Compilation failed as intended: classified record and codec functions were absent                   |
| Task 1 initial GREEN, same command                                                                                                                                                                        | 3 passed, 0 failed/ignored                                                                          |
| Task 2 meaningful RED: `bun run scripts/command-timings.ts run --key phase165-work-codec-regression -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --all-features snapshot_codec` | 65 discovered, 64 passed and 1 failed as intended: oversized record decoded as legacy zero          |
| Final work codec command                                                                                                                                                                                  | 16 passed, 0 failed/ignored                                                                         |
| Final snapshot codec regression command                                                                                                                                                                   | 69 passed, 0 failed/ignored                                                                         |
| `bun run scripts/command-timings.ts run --key phase165-work-codec-format -- cargo fmt --manifest-path packages/Cargo.toml -p open-bitcoin-node`                                                           | Exit 0                                                                                              |
| `bun run scripts/command-timings.ts run --key phase165-work-codec-lint -- cargo clippy --manifest-path packages/Cargo.toml -p open-bitcoin-node --all-targets --all-features -- -D warnings`              | Exit 0                                                                                              |
| `bun scripts/check-parity-breadcrumbs.ts --check`                                                                                                                                                         | 1,164 Rust files verified                                                                           |
| `bun scripts/bright-builds-check.ts all`                                                                                                                                                                  | 0 findings; file lengths and active repo lessons passed                                             |
| `git diff --check`, owned source/test/breadcrumb diff review                                                                                                                                              | Passed                                                                                              |
| Stub and threat surface scan                                                                                                                                                                              | No stubs or unregistered threat surfaces; only the declared numeric-record trust boundary was added |

Final focused checks discovered nonzero relevant tests. Full native verification, workspace/Bazel coverage and normal-hook finalization remain Plan 14/root responsibilities and are not claimed here. The frontmatter requirement IDs record this plan's contribution; phase-wide requirement acceptance still depends on those later owners.

## API Handoff to Plan 03

The public module is `open_bitcoin_node::storage::snapshot_codec::work`; all three API names are also re-exported from `snapshot_codec`.

```rust
pub enum ClassifiedWorkRecord {
    LegacySynthetic(u128),
    ExactNumber(ProofWork),
}
pub fn encode_work_record(
    namespace: StorageNamespace,
    record: &ClassifiedWorkRecord,
) -> Result<Vec<u8>, StorageError>;
pub fn decode_work_record(
    namespace: StorageNamespace,
    bytes: &[u8],
) -> Result<ClassifiedWorkRecord, StorageError>;
```

`ClassifiedWorkRecord` is Copy/Eq and implements JSON Serialize/Deserialize. Compact emitted shapes are:

```json
{"version":1,"kind":"exact_number","bytes_le":[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]}
```

```json
{"version":1,"kind":"legacy_synthetic","score":0}
```

Bare historical JSON integers are accepted only as `LegacySynthetic`. Field shapes are closed. Record version 1 is local to this representation and does not bump `SchemaVersion::CURRENT`. There is no legacy-to-ProofWork conversion or accepted-lineage constructor in this codec. Plan 03 must make any mechanical fixture adaptation explicit and untrusted.

## Files Created/Modified

- `work.rs`: classified numeric data, strict tagged JSON, serde embedding, bounded direct codec functions.
- `packages/open-bitcoin-node/src/storage/snapshot_codec/work/tests.rs`: 15 codec tests for numeric preservation, legacy precision and malformed/oversized refusal.
- `snapshot_codec.rs`: module registration and public codec exports only.
- `packages/open-bitcoin-node/src/storage/snapshot_codec/tests/chainstate_wallet_and_headers.rs`: one existing-header compatibility classification test; original assertions retained.
- `docs/parity/source-breadcrumbs.json`: narrow new work-record group anchored to pinned `arith_uint256.h`; predecessor mappings preserved.

## Decisions Made

Followed D-01/D-02/D-10/D-12. The simplification pass kept one existing arithmetic type and two small strict wire structs, avoiding a second numeric engine or permissive generic field map. Raw JSON preserves legacy u128 precision; schema classification does not infer validation. The 512-byte cap comfortably covers compact 32-byte tagged records and bounds the direct entrypoint before parsing.

AGENTS.md local timing, breadcrumb and shared-checkout rules; the Bright Builds sidecar; placeholder-only overrides; architecture/code-shape/verification/testing/Rust/TypeScript standards; and both complete active lesson files informed execution. No project skills were present. The parent owns initial sync/bootstrap and active-lesson audit lifecycle.

## Deviations from Plan

None in source scope or architecture. Record-size refusal is a narrow implementation of the planned bounded malformed-input controls. Deferred commits and shared tracker updates follow the approved execution contract.

## Issues Encountered

- A new malformed-input test initially used unsupported array-repeat syntax directly inside `serde_json::json!`; changed the test fixture to `vec![0; 32]` and reran the meaningful RED check.
- The added historic header test initially omitted imports from its grandparent codec module; added the explicit public imports and reran GREEN. Final lint and both focused checks pass without suppressions.
- The intended RED oversize failure was resolved by checking the 512-byte limit in the direct helper and embedded classifier. The limit boundary and both entry paths have tests.

## User Setup Required

None.

## Next Phase Readiness

Plan 03 can embed the classified record for chain/meta/header writes and perform the atomic ProofWork carrier transition. The existing DTO fields remain u128 until that plan. No dead-code allowance, schema migration, ordinary backend opening or recovery authorization was needed to stage this public codec.

Canonical bootstrap and earned exact-work checkpoint/all-handle reopening acceptance remain Phase 166; contextual prerequisites remain Phase 167; actual pinned-daemon differential qualification remains Phase 168. Networking defaults and selector support/refusal classes are unchanged.

## Self-Check: PASSED

All five owned source/test/breadcrumb files and this summary exist. Final focused tests, formatting, lint, breadcrumbs, managed checks and whitespace review pass. The index is empty, and there are no active Cargo/timing sessions. Commit-existence verification and shared tracker updates are intentionally deferred to the parent wrapper; no executor commit is claimed. Predecessor and shared tracker changes were preserved.
