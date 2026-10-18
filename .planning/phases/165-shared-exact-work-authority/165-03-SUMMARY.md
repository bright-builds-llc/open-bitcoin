---
phase: 165-shared-exact-work-authority
plan: "03"
subsystem: chainstate
tags: [proof-work, numeric-carrier, snapshot-codec, projection, parity]
requires:
  - phase: 165-01
    provides: ProofWork byte/hex boundaries and full-width arithmetic
  - phase: 165-02
    provides: Classified exact-number and legacy-synthetic records
provides:
  - Sole ProofWork numerical carrier across core, network, node and adapters
  - Tagged lossless chain, metadata and header recording with legacy hydration refusal
  - Fixed unsigned 64-digit hexadecimal work projections
  - Reviewed current Phase68/164 contracts with executable negative controls
affects: [165-04, 165-07, 165-08, 165-10, 165-13, 165-14]
tech-stack:
  added: []
  patterns: [Number width separated from earned authority, explicit numeric fixture boundary]
key-files:
  created:
    - scripts/check-phase68-active-chain-persistence.test.ts
  modified:
    - packages/open-bitcoin-chainstate/src/types.rs
    - packages/open-bitcoin-chainstate/src/engine.rs
    - packages/open-bitcoin-network/src/header_store.rs
    - packages/open-bitcoin-node/src/chainstate.rs
    - packages/open-bitcoin-node/src/network/mempool_lifecycle.rs
    - packages/open-bitcoin-node/src/network/runtime_authority.rs
    - packages/open-bitcoin-node/src/storage/snapshot_codec.rs
    - packages/open-bitcoin-node/src/storage/snapshot_codec/work.rs
    - packages/open-bitcoin-node/src/sync/runtime_state.rs
    - scripts/check-phase68-active-chain-persistence.ts
    - scripts/check-phase164-canonical-identity/contracts.ts
    - scripts/check-phase164-canonical-identity.test.ts
key-decisions:
  - "Use one explicit ChainPosition fixture-number helper rather than duplicated conversion functions."
  - "Exact tagged numbers do not prove accepted lineage; default legacy hydration refuses."
  - "Emit compact nested work records to preserve the strict 512-byte lexical bound."
patterns-established:
  - "Historic fixture numbers enter through named byte construction, never implicit From<u128>."
  - "Current contract renewal preserves every historical manifest and source receipt."
requirements-completed: [CWOR-01, CWOR-03, CWOR-05]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 165-2026-10-14T16-09-59Z
generated_at: 2026-10-14T18:17:26Z
duration: 66min
completed: 2026-10-14
---

# Phase 165 Plan 03: Atomic Exact Numerical Carrier Migration Summary

**ProofWork carries every live work number, tagged storage preserves all 256 bits, and projections render fixed unsigned hex without granting imported numbers authority.**

## Performance

- First recorded verification: 2026-10-14T17:11:37.396Z; context loading and initial edits were not separately timed.
- Final verification and summary: 2026-10-14T18:17:26Z.
- Duration: 66min, measured from that first recorded verification.
- Tasks: 3/3.
- Approved inventory: 166 paths, including 161 Rust paths, four TypeScript paths and the existing breadcrumb JSON.
- Actual Plan03 source/checker edits: 143 files (139 Rust, four TypeScript); this summary is separate.
- No new Rust files or dependencies. The predecessor breadcrumb JSON was preserved.

## Accomplishments and Group Ledger

| Group                               | Changed files                                  | Result                                                                                                                                       |
| ----------------------------------- | ---------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------- |
| Core/network and dependent fixtures | 27: chainstate16, network8, mempool2, wallet1  | ProofWork fields/signatures; explicit historic fixture numbers; core and dependent targets compile and test                                  |
| Node/codec                          | 82                                             | Wide connect/reorg/recording/progress carriers; exact tagged writes; legacy default hydration refusal; unchanged state-transition assertions |
| Adapters and current guards         | 30 Rust: CLI11, RPC16, bench3; four TypeScript | Exact rendering expectations and compiled callers; Phase68/164 negative controls remain enforced                                             |

The exact durable ownership inventory is in `165-03-PLAN.md` frontmatter and task file lists. The compiler-discovered extra-path ledger used before edits is `.local/open-bitcoin-dev/phase165-plan03-extra-paths.json` (63 paths, intentionally local/ignored). Small reviewed batches preserved the initial Plan01/02 and parent changes.

## API Handoff

- `ChainPosition.chain_work`, `AnchoredBlock.chain_work`, `HeaderEntry.chain_work`, `InsertedHeader.chain_work`, engine/managed connect parameters and `BlockProgressPoint.chain_work` now use the existing `ProofWork`.
- `ChainPosition::new(header, height, ProofWork, median_time_past)` and `ChainPosition::genesis(header, ProofWork, median_time_past)` remain raw numerical constructors. Stored/local/reorg callers carry the same number type. No accepted parent, stage or recovery token is returned by numerical import.
- `ChainPosition::untrusted_fixture_work(score: u128) -> ProofWork` copies the historic score's 16 LE bytes into a zeroed 32-byte array. Its documentation expressly denies accepted lineage, admission, stage and recovery authority. Call sites are test or benchmark fixture builders. There is no implicit `From<u128>`, and production selection/hydration does not use this helper.
- Chain-position, chain-meta and header writes embed `ClassifiedWorkRecord::ExactNumber` using version1, `kind: "exact_number"` and fixed `bytes_le: [u8; 32]`. The global Fjall schema is unchanged.
- The public Plan02 classifier still returns `LegacySynthetic(u128)` for historic bare integers and tagged legacy records. Default DTO hydration returns bounded `StorageError::Corruption` for that class rather than converting it into live work. Exact decoding is also only numerical data, with no recovery or validation provenance.
- `ClassifiedWorkRecord` serialization emits compact RawValue records even within pretty snapshots. The strict 512-byte lexical bound remains. A caller that reparses through `serde_json::Value` and adds enough indentation can exceed that bound; controlled schema-tag fixture mutation now serializes compactly. No format-independent whitespace acceptance is claimed.
- Production work projections use `ProofWork::Display`: exactly 64 lowercase unsigned hexadecimal digits, most significant first. CLI/RPC forward strings without narrowing. Tests cover high byte31/byte16 values in snapshot/chain-meta and connected-tip projections.
- Phase68 exports pure `checkCurrentWorkProjection(runtimeState: string): readonly string[]`. Six current-source tests cover the real source plus u128, count/height, cast truncation and comment decoys. Historical phase-artifact checks remain intact.

## Task Commits

All three task commits and this metadata commit are **deferred to the parent strict finalization wrapper**. This executor staged nothing, created no commits or pushes, bypassed no hooks, and did not change shared STATE.md, ROADMAP.md, REQUIREMENTS.md, config or todo. No invented hashes or normal-hook receipts are claimed.

## Executed Evidence

Every Cargo command ran sequentially through the cooperative command-timings lock with Rust1.94.1 and command-local Bun1.3.9:

```bash
PATH='/Users/peterryszkiewicz/.bun/install/cache/@oven/bun-darwin-aarch64@1.3.9@@@1/bin:'"$PATH"
```

Apply that prefix to each command below.

| Final check                                                                                                                                                                | Actual result                                                                                                   |
| -------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------- |
| `bun run scripts/command-timings.ts run --key phase165-carrier-core -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-chainstate --all-features`           | 329 unit, 3 parity, 2 BASIC integration and 10 compile-fail doctests passed                                     |
| `bun run scripts/command-timings.ts run --key phase165-carrier-node -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --all-features snapshot_codec`  | 71 passed, zero failed/ignored                                                                                  |
| `bun run scripts/command-timings.ts run --key phase165-carrier-cli -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli --all-features --lib`             | 418 passed, zero failed/ignored                                                                                 |
| Focused node regression command recorded under `phase165-carrier-node-regressions`                                                                                         | All seven observed node failures rerun directly: 7 passed, zero failed/ignored; 5.77s                           |
| `bun run scripts/command-timings.ts run --key phase165-carrier-rpc -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --all-features --lib sync_status` | 4 passed, zero failed/ignored; 2.21s                                                                            |
| `bun run scripts/command-timings.ts run --key phase165-carrier-workspace -- cargo check --manifest-path packages/Cargo.toml --workspace --all-targets --all-features`      | Exit0; final4.18s                                                                                               |
| `bun run scripts/command-timings.ts run --key phase165-carrier-workspace-tests -- cargo test --manifest-path packages/Cargo.toml --workspace --all-features`               | Exit0; 4,488 passed, zero failed, six pre-existing ignored across40 result groups, including doctests; 1324.18s |
| `bun test` `./scripts/check-phase68-active-chain-persistence.test.ts` `./scripts/check-phase164-canonical-identity.test.ts`                                                | 363 passed, zero failed; final rerun80.06s                                                                      |
| `bun run` `scripts/check-phase68-active-chain-persistence.ts`                                                                                                              | Passed                                                                                                          |
| `bun run` `scripts/check-phase164-canonical-identity.ts`                                                                                                                   | Passed: 93 fixed Rust inputs, all246 reviewed source edges retained                                             |
| `bun scripts/check-parity-breadcrumbs.ts --check`                                                                                                                          | 1,164 Rust files covered; no new Rust files                                                                     |
| `bun scripts/bright-builds-check.ts all`                                                                                                                                   | Zero findings, no exceptions added                                                                              |
| `cargo fmt --manifest-path packages/Cargo.toml --all` through `phase165-carrier-format`, `git diff --check` and bounded owned diff review                                  | Passed                                                                                                          |

Focused checks discovered relevant nonzero cases. Workspace ignored cases are pre-existing explicit live-smoke, subprocess and measurement cases; no new test was ignored or disabled. Full default native/Bazel verification, instrumented pure-core line coverage, independent review/security/goal verification and normal-hook finalization remain Plan14/root responsibilities and are not claimed here.

The exact seven-case regression command was:

```bash
bun run scripts/command-timings.ts run --key phase165-carrier-node-regressions -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --all-features --lib -- decode_versioned_accepts_blob_schema_one_after_current_is_two connected_active_chain_progress_survives_runtime_reopen phase165_tip_projections_render_all_256_work_bits phase75_synthetic_soak_reopen_preserves_resume_progress_without_duplicate_getdata phase71_same_datadir_resume_matrix_covers_clean_unclean_mid_download_mid_connect_and_stale_inflight phase72_summary_metrics_and_logs_carry_full_sync_truth_dimensions sync_summary_projects_consistent_operator_evidence_fields
```

## Current Versus Historical Evidence

Only eight reviewed Phase164 current region digests changed: two stored-connect signatures, one RPC work-projection fixture, two RPC snapshot setup tests, one supplied-snapshot rescan test, its fixture builder and the durable prune-context fixture. Each retains its function ordinal, ownership classification, sensitive calls and original non-work assertions. Fresh mutations reject renewed u128 signatures; existing canonical-guard, ordinary-opener and spoofed-fixture controls remain.

No historical manifest, frozen tooling adapter, raw receipt, migrated historical fixture attribution or archived summary changed. The preservation fixture's only semantic edit is its existing `position` builder's explicit numeric byte conversion; rustfmt expands that call. Every physical-manifest assertion, sentinel and raw migration/prune/refusal control remains.

- Original preservation Git blob: `0345bab1bfbd76be3078453a6b9201fdf73941cc`.
- Original SHA256: `3d004f91bd72a0f3902588a36fca0438b04b5ac034bc9e227893664496835630`.
- Current adapted source SHA256: `e6cb5740cb5c83655aed14c43b1206847d264431d944741a365771c3e8d76f5b`.

Those identities stay separate; current fixture source does not reuse an old executed qualification receipt.

## Decisions and Simplification

AGENTS.md timing/breadcrumb/native-contract guidance, the Bright Builds sidecar, placeholder-only overrides, architecture/code-shape/verification/testing/Rust/TypeScript pages and both complete active lesson files (7,188 bytes;2,397 estimated tokens) informed execution. The parent owned initial sync/bootstrap.

The simplification pass replaced duplicated conversion helpers with one documented numerical fixture boundary. One existing imported `ChainPosition` name shortened a test call to avoid a file-length regression; the managed check passes without exclusions. No new arithmetic engine, dual live score field, dependency, worktree or architecture was introduced.

## Deviations from Plan

**[Rule3 - Blocking] Corrected constructor-only consumer inventory.** Initial `chain_work` text search missed positional constructors, work-named aliases and one separate string assertion. The root approved each compiler/test-discovered batch before its edits and expanded the exact plan inventory103→166. The 63 added paths contain mechanical fixture adaptation only. Core integration fixtures moved earlier because the Task1 Cargo command builds them. Workspace diagnostics decreased15→10→1→0 after dependency-group closure.

**[Rule1 - Bug] Kept writer bytes inside the record bound.** Four codec roundtrips exposed embedded pretty records above512bytes. Compact RawValue serialization preserves the existing bound and tagged shape; all71 codec tests pass. A schema-tag-only fixture that re-pretty-printed a parsed Value needed compact serialization while keeping its schema1/2/3 assertions.

No source authority behavior or historical receipt scope expanded.

## Issues Encountered and Resolution

- Helper-consolidation import cleanup accidentally matched call text, and a diagnostic wrapper initially omitted two shorthand field labels. Diff/compiler review caught both. Exact fixture calls/labels were repaired, local duplicate helpers removed, and core rerun passed329unit plus allintegration/doctests.
- The first workspace run found two stale CLI expectations; both now preserve their represented values in64-digit hex, and the full CLI library rerun passed418cases.
- The next complete node run passed1,744 and failed7 with4pre-existing ignores. Failures were one schema-tag fixture lexical size, five stale decimal expectations/log needles, and a new high-width test literal containing66 rather than64digits. Independent reversed-byte calculation corrected the literal. The seven focused regressions and final complete workspace run pass.
- Quiet polling was never treated as a timeout. Node tests continued to emit passes and showed active CPU use; no running verification was interrupted.

## Known Stubs and Remaining Authority Work

No new goal-blocking stub exists. The deliberately temporary `header_work_score` count and `next_chain_work` count now use ProofWork and wrapping addition. Their accepted target-derived replacements belong to Plans04/07/08, as expressly allowed by Plan03. Numeric exactness here grants no ancestry, checked-body eligibility, stage or recovery authority.

## Next Plan Readiness

Plan04 can consume the ProofWork header/result fields and retire temporary count selection. Plans07/08 must still earn and guard owner lineage before actual connect/reorg effects. Plan10 can consume legacy classification and default refusal without opening a source backend. The workspace is GREEN at this atomic boundary.

Canonical durable bootstrap, earned exact-work checkpoint/all-handle reopening and genesis BASIC remain Phase166; contextual prerequisites remain Phase167; actual pinned-daemon differential qualification remains Phase168. Ordinary support/refusal classes and default-off networking remain unchanged.

## Self-Check: PASSED

All143 owned changed source/checker files and this summary exist. Final workspace and focused verification are GREEN, the index is empty, and no executor Cargo/timing/Bun sessions remain. The stub/threat scan found no new unregistered surface beyond the planned numeric-record/fixture boundaries. Commit existence and shared tracker updates are intentionally deferred to the approved parent wrapper.
