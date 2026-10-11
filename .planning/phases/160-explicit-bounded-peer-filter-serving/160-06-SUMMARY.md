---
phase: 160-explicit-bounded-peer-filter-serving
plan: "06"
subsystem: network-storage
tags: [bip157, basic-filters, resource-budgets, fjall, measurements]
requires:
  - phase: 160-05
    provides: Exact branch authorization and bounded captured identities
provides:
  - Checked request work, suffix retention and pending-frame capacity arithmetic
  - Immediate guarded complete-batch persisted reads with target and parent integrity
  - Genuine finite-range structural measurements and high-stop refusal evidence
affects: [160-07, 160-08, 160-09, 160-10, 160-11, 160-12]
tech-stack:
  added: []
  patterns: [charge-before-parse-and-copy, complete-batch-or-no-prefix, measured-local-refusal]
key-files:
  created:
    - packages/open-bitcoin-network/src/filter_serving/resource.rs
    - packages/open-bitcoin-network/src/filter_serving/resource/tests.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/filters/query/peer.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/filters/query/peer/tests.rs
    - packages/open-bitcoin-node/src/sync/tests/filter_index/peer_fixtures.rs
    - packages/open-bitcoin-node/src/sync/tests/filter_index/peer_measurements.rs
    - docs/parity/fixtures/peer-filter-resource-measurements.json
  modified:
    - packages/open-bitcoin-network/src/filter_serving.rs
    - packages/open-bitcoin-network/src/lib.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/filters/query.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/filters/query/tests.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/filters/tests.rs
    - packages/open-bitcoin-node/src/sync/tests/filter_index.rs
    - packages/open-bitcoin-node/src/network/runtime_authority/filter_index/branch.rs
key-decisions:
  - Metadata ceiling is 256, accommodating bounded stale authorization cost 138 plus publication-owner inspection without changing other limits.
  - Persisted-read measurements precede transport integration; response encoding copies and checksum work remain explicit Plan 08 responsibilities.
  - Sparse refusal uses genuine accepted headers and durable coins with a short indexed prefix, without claiming full-index or configured-startup success.
requirements-addressed: [CFNET-02, CFNET-03, CFNET-04, CFNET-05, CFNET-06]
requirements-completed: []
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 160-20261010-peer-filter-serving
generated_at: "2026-10-10T19:56:52Z"
duration: approximately 20min
completed: 2026-10-10
---

# Phase 160 Plan 06: Measured Persisted Resource Boundaries Summary

**Checked resource meters and complete guarded Fjall reads admit genuine 1,000-filter/2,000-header ranges while refusing oversized wire records and excessive checkpoint ancestry without returning a prefix.**

## Accomplishments

- Added eight independent checked work axes: ancestry links, header operations, record reads, metadata units, validated envelope bytes, hash-padded bytes, logical copy bytes and encoded suffix bytes. Every failed charge preserves that counter; overflow is a refusal.
- Added pure request/suffix retention and pending-frame admission helpers. One suffix is one request; one peer request and four aggregate requests retain at most 8MiB/32MiB. A narrow CF frame allows 4,000,024 pending bytes per peer while the combined aggregate remains 8MiB, including other pending frames. Generic message/effect limits remain unchanged.
- Added immediate peer reads under one existing publication guard. Record-read admission precedes backend lookup; borrowed lengths and target framing admission precede full encoding/hash validation, predecessor validation and owned filter copies. Batch-container bytes are admitted before allocation. Any missing target yields no complete batch; a present target with a missing/bad predecessor remains a storage error.
- Preserved the RPC point-query path and its codec-capacity singleton. The 33,554,432-byte stored filter remains present and queryable; peer cfilter admission refuses it before filter hash/copy work and leaves its persisted envelope intact.
- Added genuine accepted spending histories, durable coins/Fjall rows, a consensus-validated 40,000-distinct-output block, and an actual shared-authority measurement seam. No dependency, crate, persisted schema, alternate filter store, generation-on-request or transport activation was added.

## Task Acceptance

### Task 1: Finite resource arithmetic

- [x] Every independent work axis has finite named defaults and exact/one-over/overflow checks.
- [x] Full cfilter, cfheaders and cfcheckpt frame formulas use CompactSize and the 24-byte wire header.
- [x] Retained request/peer/aggregate suffix capacity and pending-frame admission include overflow and other pending bytes.
- [x] Collections require finite count/byte admission; the generic 128-effect queue cannot truncate the planned 1,000-filter suffix.

### Task 2: Guarded peer reads

- [x] Target, immediate-parent and genesis integrity reuse full `parse_record`, `verify_parent` and `checkpoint_identity`; no `parse_record_fields` shortcut was introduced.
- [x] Every charged stored-read axis passes its observed exact boundary and refuses one less with no successful result.
- [x] A valid approximately 1MiB hidden predecessor is refused before its hash work when the envelope allowance is one byte short.
- [x] Captured height/parent mismatches, target/parent corruption, missing predecessor, backend fault, disabled backend and invalidated integrity epoch remain distinct from missing target.
- [x] Complete-batch preparation discards already prepared rows when a later target is absent. Fixed `Display` categories omit injected backend paths.
- [x] Codec storage capacity is preserved independently from the 4,000,000-byte wire payload policy.

### Task 3: Genuine measured evidence

- [x] Actual persisted reads cover 1/20/400/1,000 cfilters, 2,000 cfheaders and checkpoint stop 3,001; repeated reads and a real close/reopen reproduce structural counters.
- [x] Every measured counter is checked against its named default; each axis receives complete-path exact/one-under controls.
- [x] Forty thousand distinct consensus-valid outputs generate an actually large 105,277-byte filter; the full cfilter frame is 105,339 bytes and is admitted.
- [x] A genuinely accepted stop at 9,301 requires 8,302 ancestry vertices and is locally refused by the 8,192-link policy before any persisted filter-row read. No protocol stop-height maximum is inferred.
- [x] Checked-in structural JSON is copied from observed counters and asserted against fresh measurements. Timings remain observations outside the checked-in structural fixture.

## TDD and Verification Evidence

All Cargo commands used pinned Bun 1.3.9 and `bun run scripts/command-timings.ts run --key phase160-persisted-resources --`. Sessions were polled within 60 seconds. A transient rust-analyzer artifact lock was observed and allowed to finish; no build was terminated or lock altered.

| Stage | Cargo command after timing wrapper | Actual evidence |
| -- | -- | -- |
| Task 1 initial RED | `test --manifest-path packages/Cargo.toml -p open-bitcoin-network phase160_resources -- --nocapture` | Missing resource module failed compilation before implementation; the next run executed 2 tests with 1 failing due to an incorrect expected u32::MAX frame value, corrected to 137,439,006. |
| Task 1 final GREEN | Same command | 3 executed, 3 passed, zero ignored. |
| Task 2 behavioral RED | `test --manifest-path packages/Cargo.toml -p open-bitcoin-node phase160_budgeted_read -- --nocapture` | 2 executed: exact bounded singleton passed; the not-yet-implemented complete batch failed its required four-row result. |
| Task 2 final GREEN | Same command | 6 executed, 6 passed, zero ignored. |
| Task 3 behavioral RED | `test --manifest-path packages/Cargo.toml -p open-bitcoin-node phase160_peer_measurements -- --nocapture` | 1 executed, failed because no 1,000-row measured range was prepared yet. Genuine 3,002-entry fixture construction ran. |
| Task 3 first implementation | Same command | Complete matrix and genuine large case passed; sparse configured startup correctly refused missing future activation body at height 3. |
| Sparse fixture correction | Same command with filter `phase160_peer_measurements_sparse` | 1 executed, 1 passed; genuine accepted/durable history refused ancestry with zero persisted filter reads. |
| Task 3 final GREEN | Original three-test command | 3 executed, 3 passed, zero ignored; fresh structural fixture equality passed. Test execution 124.62 seconds, independent of compilation. |
| Existing point-query regression | `test --manifest-path packages/Cargo.toml -p open-bitcoin-node filters::tests::query -- --nocapture` | 6 executed, 6 passed, including 32MiB codec capacity, clone epoch loss, target/parent integrity, absence and exact bounds. |
| Targeted lint | `clippy --manifest-path packages/Cargo.toml -p open-bitcoin-network -p open-bitcoin-node --all-targets --all-features -- -D warnings` | Passed. |
| Scoped formatting | `rustfmt --edition 2024 --config skip_children=true --check` on all 13 touched Rust paths | Passed. |
| Managed/parity/whitespace checks | Managed `all`, `check-parity-breadcrumbs.ts --check`, scoped diff review and `git diff --check` | Zero managed findings; 1,077 Rust breadcrumb paths verified; whitespace clean. |

Nonmatching network integration binaries executed zero matching tests; required unit filters executed the nonzero counts above. Native workspace/build/coverage, independent source/security review and final requirement completion remain the parent's consolidated gate.

## Structural Measurements

These counters cover branch capture, guarded persisted validation, owned stored-record views, bounded containers/identity bytes and full response suffix **size admission**. They do not claim that response encoding or sockets already ran. Metadata is conservative authorization work plus actual publication-owner inspection, rather than an assertion of seven Fjall metadata reads.

| Case | Returned records | Ancestry | Header ops | Record reads | Metadata | Envelopes | Hash padded | Logical copies | Framed suffix |
| -- | -- | -- | -- | -- | -- | -- | -- | -- | -- |
| cfilters 1 | 1 | 1 | 4 | 2 | 7 | 348 | 832 | 748 | 62 |
| cfilters 20 | 20 | 20 | 80 | 40 | 7 | 6,982 | 16,640 | 13,701 | 1,251 |
| cfilters 400 | 400 | 400 | 1,600 | 800 | 7 | 139,222 | 332,800 | 272,101 | 24,811 |
| cfilters 1,000 | 1,000 | 1,000 | 4,000 | 2,000 | 7 | 348,022 | 832,000 | 680,101 | 62,011 |
| cfheaders 2,000 | 2,000 | 2,001 | 8,003 | 4,001 | 7 | 696,196 | 1,664,512 | 1,360,401 | 64,092 |
| checkpoint stop 3,001 | 3 | 2,002 | 2,011 | 6 | 7 | 1,044 | 2,496 | 2,108 | 154 |
| genuine large cfilter | 1 | 1 | 4 | 2 | 7 | 105,621 | 106,112 | 211,294 | 105,339 |

Final first/repeated authority holds in microseconds were 75/64, 1,281/970, 21,564/21,973, 53,360/52,834, 104,843/103,532 and 1,136/1,100 for the six range cases respectively. The reopened checkpoint hold was 1,081us, genuine-large hold 21,945us, and sparse refusal hold 4us. Total durations were separately printed by tests, including mutex acquisition. These are local observations, not limits or latency promises. “First” means the first request after recovered runtime construction; recovery itself reads the forest, so no cold disk-cache claim is made.

## Interfaces and Required Plan 08 Handoff

- Pure resource APIs are exported as `open_bitcoin_network::basic_peer_resource`, avoiding the existing internal generic `resource` module name. `BasicPeerServeBudget::{with_limit, remaining, work, charge}` operates on named `BasicPeerServeAxis`; `BasicPeerServeWork::get` reports consumed work.
- `admit_response_reservation` checks whole suffix/request occupancy; `admit_active_filter_frame` includes other pending peer/aggregate transport bytes. Plan 07 supplies affine ownership and release; these pure helpers do not themselves own a live lease.
- `cfilter_frame_bytes(n) = 57 + CompactSizeBytes(n) + n`; `cfheaders_frame_bytes(k) = 89 + CompactSizeBytes(k) + 32k`; `cfcheckpt_frame_bytes(h) = 57 + CompactSizeBytes(floor(h/1000)) + 32floor(h/1000)`. Checked arithmetic refuses overflow.
- `FjallNodeStore::with_basic_peer_query(&mut budget, |reader, meter| ...)` owns one publication guard. It charges each actual OWNER/legacy STATE lookup before reading and reports Disabled distinctly. It never invokes RPC readiness, indexing, body/undo acquisition or repair.
- `reader.maybe_peer_record(hash, meter, filter_frame)` returns exact owned `BasicFilterRecordView` or absence. `filter_frame=true` additionally reserves its full cfilter suffix bytes before target parse/copy. For headers/checkpoints, their owner must reserve the full family frame before collections/encoding.
- `reader.maybe_peer_records(identities, meter, filter_frame)` reserves the bounded view container, verifies each captured height/parent and returns every view, `None`, or a typed error. A cfheaders predecessor remains a separate required read. The owner must preserve captured lifecycle/integrity checks and sanitize errors into public outcomes.
- Branch capture charges observed active 6 or conservative stale 138 metadata units; publication OWNER adds one, yielding active 7 and stale 139 in composed reads. The 256-unit ceiling accommodates these without changing the bounded 128-provenance batch.
- **Plan 08 must charge additional actual response construction, encoding and checksum work before those effects.** For `encode_wire` payload length `p`, checksum SHA256d adds `ceil((p+9)/64)*64 + 64` padded bytes and `p+32` hash-input copy bytes. Encoding a payload and framed output adds logical lengths `p + (p+24)`; together with checksum input copies this is `3p+56`, plus any additional message-vector/source ownership copies actually made. Charge those through the same generic meter. The measured persisted-read table above deliberately excludes effects not executed yet.
- Reserve peer/aggregate request ownership before branch/output preparation, carry finite remaining ancestry/header/copy limits into branch admission, preserve invalid semantic precedence, and ensure later transport uses the narrow frame override without changing generic policies. Socket completion and cleanup remain Plans 07–10.

## Deviations from Plan

1. **[Rule 2 - Missing usable metadata headroom]** The original 128-unit ceiling could not admit Plan 05's already established stale charge 138. Parent approved 256 and aligned the plan/research; all other defaults remained unchanged. Tests and structural fixture record the finite policy.
2. **[Rule 3 - API/test visibility]** Parent approved the `basic_peer_resource` reexport in network/lib.rs, test-only fixture module visibility in filters/tests.rs, and a `cfg(test)` measurement method in branch.rs. The latter holds the actual shared authority once and distinguishes hold from total duration. No production endpoint was introduced.
3. **[Rule 3 - Sparse fixture activation boundary]** A first sparse fixture used configured Enabled startup with missing future bodies and correctly failed activation at height 3. The corrected test constructs the actual network over genuine accepted history and durable coins with a three-record index prefix, without starting catch-up or claiming configured full-startup/full-index success. It proves only early ancestry refusal and zero persisted filter reads.

AGENTS.md, the Bright Builds sidecar, placeholder-only overrides, architecture/code-shape/testing/verification/operability/Rust standards, finalized context/research/decision coverage and the parent-loaded active lessons informed execution. Parent synchronized the baseline and owns shared planning state.

## Breadcrumbs, Scope and Security Review

Parent serially registered all six new Rust files and verified 1,077 paths. Resource files use ordered anchors net_processing.cpp, net.h, p2p_blockfilters.py; peer-read files use index/blockfilterindex.cpp, blockfilter.cpp; sync fixture/measurement files use net.cpp, net_processing.cpp, p2p_blockfilters.py. Existing modified files retain their established comments.

- T-160-16: Borrowed target/parent lengths and checked envelope/hash/copy charges precede full parse; hidden large predecessor and codec-capacity refusal controls pass.
- T-160-17: Checked finite axes, complete legal ranges, exact controls and genuine sparse ancestry refusal bound local work; there is no checkpoint protocol stop-height cap.
- T-160-18: Present-but-corrupt/missing-parent failures are typed storage failures; complete-batch ownership exposes no successful prefix.
- Simplification kept one indexed counter policy, one capacity helper and existing proof-preserving parsers. Error display remains fixed and sanitized. RPC behavior and durable schema remain untouched.
- Stub scan found no remaining TODO/FIXME/placeholder implementation; `Ok(None)` paths represent actual absence. No new network endpoint, authentication path, production filesystem pattern or trust-boundary schema was introduced.

## Task Commits and Shared State

All three tasks and this summary await the strict outer consolidated commit. No staging, commit, push, branch, worktree, hook bypass, STATE/ROADMAP/REQUIREMENTS/todo or shared breadcrumb-manifest mutation was performed by this executor. Requirements-completed intentionally remains empty until the parent final gate.

## Authentication Gates and Deferred Issues

None. Response effect accounting, affine retention ownership, normal dispatch/transport integration and consolidated native verification remain explicitly assigned downstream plan work.

## Self-Check: PASSED

All six new Rust files, the structural fixture and this summary exist; all seven modified Rust paths remain present. Final required unit filters passed 3/6/3 tests, the six existing point-query regressions passed, scoped Clippy and formatting passed, and final parity/managed/whitespace checks passed. Parent owns final native verification and shared state. Commit-existence checks are deliberately inapplicable because the strict outer lifecycle forbids intermediate commits; no commit hash or requirement completion is fabricated.
