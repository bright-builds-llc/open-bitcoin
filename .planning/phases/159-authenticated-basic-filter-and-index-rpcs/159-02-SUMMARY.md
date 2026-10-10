---
phase: 159-authenticated-basic-filter-and-index-rpcs
plan: "02"
subsystem: rpc
tags: [rust, rpc, basic-filters, knots-parity, normalization]
requires: []
provides:
  - Pure typed getblockfilter/getindexinfo contracts and directly callable normalizers
  - Exact numeric -3/-5 error roundtrips and source-derived complete arity help
  - Allowlisted BASIC summary and lowercase filter/header projection
affects: [159-06, 159-07, 159-08]
tech-stack:
  added: []
  patterns: [dedicated pinned framework normalization before handler validation]
key-files:
  created:
    - packages/open-bitcoin-rpc/src/method/filter_index.rs
    - packages/open-bitcoin-rpc/src/method/filter_index/normalize.rs
    - packages/open-bitcoin-rpc/src/method/filter_index/tests.rs
  modified:
    - packages/open-bitcoin-rpc/src/method.rs
    - packages/open-bitcoin-rpc/src/error.rs
    - packages/open-bitcoin-cli/src/client.rs
    - packages/open-bitcoin-cli/src/client/tests.rs
    - packages/open-bitcoin-cli/src/args/tests.rs
key-decisions:
  - Keep registry variants, normalize_method_call routing and real dispatch together in Plan 159-06
  - Use pinned UniValue bool type spelling, overriding research shorthand boolean
  - Verify complete help against an independently rendered source-derived fixture, without claiming a running Knots oracle
patterns-established:
  - Named conversion then arity/help then aggregated types then strict raw hash then filter selection
requirements-addressed: [CFRP-01, CFRP-02]
requirements-completed: []
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 159-2026-10-09T16-11-49
generated_at: "2026-10-09T23:30:24Z"
duration: approximately 11min
completed: "2026-10-09"
---

# Phase 159 Plan 02: Pure Filter RPC Contracts and Normalization Summary

**Typed BASIC/V0 selection, strict raw-hash parsing, pinned framework error ordering and allowlisted filter/index JSON now pass 18 focused unit tests.**

## Performance

- Approximately 11 minutes including source reading and cooperative build-lock waits.
- First recorded test: 2026-10-09T16:45:50.313Z; implementation/verification completed at approximately 16:55 UTC.
- Tasks: 2/2 implemented and verified; 5 source files plus this summary.
- Those timing/file counts describe the original parser execution. The later CLI follow-up touches three additional existing files; its actual Rust recovery completed at 2026-10-09T23:25:36.767Z, as recorded below.
- Pinned baseline verified: `a9aee730466ac67d35a3c03ee24676be5e045878`; Rust 1.94.1 and Bun 1.3.9.

## Accomplishments

- Added `GetBlockFilterRequest { block_hash, filter_type }`, `GetIndexInfoRequest { maybe_index_name }`, `BlockFilterSelection::{Basic,V0}`, `GetBlockFilterResult` and `BasicFilterIndexSummary` without new dependencies or storage/authority calls. The existing node reexport supplies only the pure `BlockHash` primitive.
- `RpcErrorCode::TypeError` and `InvalidAddressOrKey` serialize and deserialize as -3 and -5. Existing failure kinds and HTTP mapping are unchanged.
- `GetBlockFilterResult::from_encoded` encodes filter bytes directly, reverses raw header bytes once, and emits lowercase hex with exactly `filter`/`header`. An asymmetric golden proves byte order. `GetIndexInfoRequest::project_basic` exposes only `basic block filter index` with `synced`/`best_block_height`; disabled/unmatched selections return `{}`.
- Dedicated exported normalizers preserve duplicate-name, unknown-name, mixed collision, null-hole, arity, aggregate-type and strict-hash precedence. Required null is a type error; optional null defaults. Strings permit uppercase ASCII hex but reject trimming, prefixes, nonhex and wrong UTF-8 byte length before resolving filter names.
- V0 is recognized as a selection only; its `disabled_failure` preserves `Index is not enabled for filtertype v0`. There is no V0 generator or serving implementation.

## Verification Evidence

All Cargo commands used `bun run scripts/command-timings.ts run --key <key> -- <command>` with the pinned Bun PATH. Cooperative target locking was retained throughout.

| Check | Evidence |
| --- | --- |
| Task 1 RED | `phase159-rpc-contracts-red`: exit 101, absent contracts and absent -3/-5 variants before implementation |
| Task 1 GREEN | `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --lib phase159_filter_rpc_contract`: **5 passed**, 0 failed |
| Task 2 RED | `phase159-rpc-parser-red`: exit 101, 23 missing dedicated-normalizer symbols before implementation |
| Task 2 GREEN | `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --lib phase159_filter_rpc_normalize`: **13 passed**, 0 failed |
| Final method regression | `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --lib method::`: **31 passed**, 0 failed, including all 18 new tests and 13 existing method tests after final lint fixes |
| Scoped production lint | `cargo clippy --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --lib --no-deps -- -D warnings`: exit 0; no RPC warnings/errors |
| Formatting | `rustfmt +1.94.1 --check --edition 2024 --config skip_children=true` on the five owned Rust files: exit 0 |
| Bright Builds | Final `bun scripts/bright-builds-check.ts all`: 0 findings |
| Diff review | Owned tracked changes contain only the two numeric code mappings and child module/export additions; all new files reviewed; whitespace check clean |

The initial contract GREEN binary paused before entering its harness, then completed successfully. It was polled without termination; process liveness was checked. A sample attempt found the process had already exited. That pause is not reported as a test deadlock or test timeout.

The first parser RED attempt was blocked by concurrent Plan 01's temporary `BlockHash::ZERO` compile errors. After its owner fixed them, the genuine parser RED was rerun and observed. A later lint retry encountered the same worker's transient missing `BlockHash` import; its owner corrected it before the final passing lint. Neither unrelated compilation failure is claimed as parser RED evidence. Final dependency compilation emitted 16 unused staging warnings from Plan 01 node declarations, whose integrations belong to later plans; the scoped RPC lint passed without suppressing those warnings.

## Source-Derived Exactness

Pinned `rpc/server.cpp::transformNamedArguments` provides duplicate detection, `args` array handling, null-hole insertion and mixed collision precedence. `rpc/util.cpp::RPCHelpMan::HandleRequest` provides arity before all argument type mismatches; `RPCArg::MatchesType` delegates type spelling to UniValue. Pinned `univalue.cpp::uvTypeName` spells boolean values **`bool`**. `univalue_write.cpp::writeObject` provides the exact four-space aggregate formatting.

`ParseHashV`, `uint256::FromHex`, `getblockfilter`, `getindexinfo`, `BlockFilterTypeByName` and the BASIC index constructor provide strict hash/error/result/name behavior. The full help fixtures independently implement the simple pinned `Sections::ToString` padding and `RPCResult::ToSections` rules, including retained maximum width after removal of a final comma. Whole-message equality covers signature, prose, arguments, result documentation, examples and final newlines. The pinned unquoted `txindex` RPC example and trailing space in the no-argument CLI example are preserved.

**Limit:** this is source-derived full-string evidence, not output captured from a running Knots node. No runnable pinned Knots binary was available, and no third-party build was introduced. For several simultaneous unknown names, pinned Knots chooses an unspecified unordered-map entry; the normalizer likewise reports one remaining entry and makes no cross-platform ordering claim for that unspecified choice.

## Task Commits

1. **Task 1: Add pure wire contracts and numeric error support** — pending root consolidated commit.
2. **Task 2: Normalize names, arity, aggregate types and hashes in pinned order** — pending root consolidated commit.

No files were staged or committed, no hooks bypassed, and no push performed. This follows the originating plan's strict finalization override. STATE, ROADMAP, REQUIREMENTS, todo and config remain root-owned and were not edited by this executor.

## Files Created/Modified

- `method/filter_index.rs`: pure contracts, complete help constants, index selection and response formatting.
- `method/filter_index/normalize.rs`: directly callable argument normalizers and strict parsing.
- `method/filter_index/tests.rs`: 5 contract tests and 13 parser tests, with source-derived help rendering.
- `method.rs`: child declaration and public exports only.
- `error.rs`: explicit numeric -3/-5 enum and conversions only.
- `packages/open-bitcoin-cli/src/client.rs`: explicit canonical wire conversion for the two registered typed calls, added in the later integration follow-up.
- `packages/open-bitcoin-cli/src/client/tests.rs`: complete envelope/selector/asymmetric-hash and authenticated root-endpoint controls.
- `packages/open-bitcoin-cli/src/args/tests.rs`: positional/named/default/V0/selector preservation and exact invalid-argument controls.

All three new Rust files contain pinned source breadcrumb comments. Plan 08 owns their manifest registration; their paths were reported to the root agent.

## Decisions Made

Followed revised staging: no `SupportedMethod`/`MethodCall` variants, no `normalize_method_call` routing and no fake handlers. The existing registry tests continue passing. Both public normalizers can be called directly by Plan 06's real routing. The pure BASIC summary helper receives readiness/height from its caller and never invents backend state.

The simplification pass retained one small shared framework checker and named conversion function for these two fixed argument lists. The helpers require no generalized schema layer, new dependency or duplicate authority. Lint-driven cleanup replaced an invariant `expect` with a defensive error guard and collapsed the mixed-collision guard.

## Deviations from Plan

**[Rule 1 - Bug] Corrected research shorthand for JSON boolean type diagnostics.** Pinned `univalue.cpp` returns `bool`, whereas the research example said `boolean`. Tests and normalizer use the exact pinned `bool` spelling; the root agent was notified. No scope or architecture deviation was required.

## Issues Encountered

The first scoped lint found the crate's production `expect_used` ban and a collapsible nested guard. Both were fixed locally; final lint and all method tests passed. The concurrent storage file initially exceeded 628 lines; its owner reduced it, and final Bright Builds passed. No unrelated files were modified by this executor.

## Known Stubs

None. Empty request vectors and `{}` projections implement required omitted/disabled/unmatched semantics. No TODO/FIXME, unimplemented handler, mock data source or placeholder response remains in the owned files. Production dispatch remains intentionally owned by Plan 06.

## Security and Integration Handoff

T-159-05 normalization and T-159-06 result projection have concrete unit evidence. T-159-07 method origin/scope remains Plan 06's registry responsibility. The new code introduces no endpoint, authentication path, file access, schema change or unplanned trust boundary.

Plan 06 must route the dedicated normalizers through baseline/node registration, reject wallet scope, preserve raw HTTP duplicate pairs and prove authenticated daemon behavior. V0 selection must return `disabled_failure` before storage access. Actual HTTP legacy/v2 response status evidence belongs there; existing failure-kind mapping has not been changed.

Plan 08/root retains breadcrumb manifest registration, full native verification including Bazel, independent source/security review, lifecycle validation, requirement completion and Git finalization. CFRP-01/02 are addressed locally but remain incomplete until those phase-wide gates pass. No external setup is required.

## Self-Check: PASSED

- All three created Rust files and the five owned source paths exist.
- 18 focused tests are discovered; both task selectors and all 31 method tests passed.
- Final production lint, formatting and Bright Builds passed; owned diffs and stub/threat scan reviewed.
- This summary carries the originating lifecycle and leaves requirements-completed empty.
- Commit existence check is intentionally not applicable: no commit was permitted by the strict root-owned finalization override; both task commits remain explicitly pending.

## CLI Follow-up: Actual Rust Verification Passed; Native Gate Pending

After Plan 06 registered `MethodCall::GetBlockFilter` and `GetIndexInfo`, root's native Rust preflight produced the genuine E0004 failure in the CLI's exhaustive `method_call_to_json` match. The follow-up adds explicit arms rather than a wildcard or request Serialize derives: reversed raw `BlockHash` bytes become canonical lowercase display `blockhash`; BASIC/V0 become `filtertype: basic/v0`; omitted/null index selection emits `{}`, while present empty/exact/unmatched selection emits the pinned `index_name` key. No internal request field names leak into JSON and no dependency or activation change was introduced.

Six new controls cover full V2 envelopes, positional/named/mixed/default/null requests, asymmetric uppercase/lowercase hash display, V0 forwarding, omitted/empty/exact/unmatched selectors, exact parameter failures, and an actual authenticated localhost HTTP request that stays on the root node endpoint despite `-rpcwallet`. All six now have actual GREEN evidence in root's successful Rust recovery 4:

| Suite/gate | Actual result |
| --- | --- |
| Full workspace Rust formatting, strict Clippy for all targets/features, and build | Passed |
| Workspace tests/doctests | 3,857 passed, 3 ignored |
| CLI library suite | 365 passed, including all three new argument controls; included in the workspace total |
| Explicit `open-bitcoin-cli` binary suite | 7 passed, including all three new client controls |

The successful timed `phase159-native-rust-recovery` attempt started at 2026-10-09T23:00:24.065Z and ended at 23:25:36.767Z with exit 0. Its record is `.local/open-bitcoin-dev/command-timings/phase159-native-rust-recovery/2026-10-09T23-00-24.065Z-0cca7b01-5e69-4178-8f24-573cbc697283.json`. The binary has `test = false` in Cargo metadata, so the explicit binary suite is essential; Plan 08 added that suite to default native verification and guards its ownership/route/selector discovery. The 18 original parser/contract tests above remain a separate, valid receipt.

The earlier CLI build-script attempt was diagnostically aborted only after fresh samples confirmed zero-CPU `_dyld_start` without Rust frames. That historical exit 101/SIGTERM is not a compiler/test failure or RED evidence; root's preceding E0004 is the real RED. The preserved [follow-up checkpoint](159-02-FOLLOWUP-CHECKPOINT.md) records the host evidence, all six exact test names and the later successful resolution.

At this update, **default native run 4 remains in progress** (root session 19945); no default native pass is claimed. Root reports the follow-up source review clean (91 checks) and 611 guard tests passed. Formal phase verification, requirement completion and Git finalization remain root-owned. `requirements-completed` stays empty; this executor ran no additional Cargo/tests and changed only these execution artifacts for this evidence update.
