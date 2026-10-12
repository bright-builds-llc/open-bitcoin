---
phase: 162-real-prune-retention-and-integrated-parity-proof
plan: "01"
subsystem: testing
tags: [regtest, fjall, basic-filters, independent-oracle, normal-maturity]
requires:
  - phase: 158
    provides: Validated spend/fork patterns and ordinary BASIC owner turns
provides:
  - Shared bounded normal-maturity raw block recipe with deterministic descriptor funding
  - Concrete live-acceptance Fjall adapter and measured indexed cost probe
  - Bounded independently decoded branch-local BASIC and peer-wire expectations
affects: [162-02, 162-03, 162-04, 162-05, 162-06, 162-07, 162-12]
tech-stack:
  added: []
  patterns: [test-only shared include, branch-local raw-output deltas, pinned bounded child oracle]
key-files:
  created:
    - packages/testdata/phase162/recipe.rs
    - packages/testdata/phase162/process.rs
    - packages/open-bitcoin-node/src/sync/tests/filter_index/phase162.rs
    - scripts/phase162-filter-oracle.ts
    - scripts/phase162-filter-oracle.test.ts
  modified:
    - packages/open-bitcoin-node/src/sync/tests/filter_index.rs
    - scripts/basic-filter-oracle.py
key-decisions:
  - "Configured Enabled reopen after genuine persisted genesis creates the actual runtime BASIC owner."
  - "Direct bounded owner turns belong to the cost probe; Plan02 must exercise its actual idle worker."
  - "Oracle state consists of independently decoded ancestry/output deltas and never caller-supplied filter or spent-script expectations."
requirements-completed: []
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 162-2026-10-11T16-25-37
generated_at: 2026-10-11T17:36:29Z
duration: 35min
completed: 2026-10-11
---

# Phase 162 Plan 01: Shared Normal-Maturity Recipe and Independent Oracle Summary

**A genuine 1,001-block Fjall history matches independently decoded BASIC bytes, hashes, headers and spent facts; bounded branch expectations include two distinct positive-height checkpoints.**

## Performance and Scope

- Source execution began at 2026-10-11T17:01:21Z; summary generated at 2026-10-11T17:18:47Z.
- Tasks: 2/2. Source files: 7, including the supplemental review's shared process helper. No production API, crate, dependency, network endpoint or support schema was added.
- Loaded repo AGENTS, Bright Builds sidecar, overrides, applicable architecture/code-shape/testing/verification/language guidance, GSD execution workflow/template and phase context/research/UI references. Both active lessons were fully loaded: 7,188 bytes and 2,397 conservative estimated tokens.
- The parent owns state, roadmap, requirements, task ledger, parity registries, final full native verification, independent reviews and Git finalization. Requirements remain pending here.

## Accomplishments

1. Shared `RECIPE_ID = phase162-normal-maturity-1` supports normal 100-block maturity with canonical heights 0..1000, the first h1 coinbase spend at h101, subsequent historical and same-block spends, and a deterministic alternate h999..1002 branch from ancestor998. Canonical parentage and fork determinism are tested. A real 25,000-satoshi h20 coinbase output uses the existing published regtest descriptor through Wallet API derivation and remains unspent.
2. The concrete adapter reads actual runtime parameters/flags, saves ordinary bodies, obtains 1,001 actual `connect_local_block` receipts, flushes genuine coins/undo/metadata, and verifies actual persisted undo against raw-outpoint facts. No snapshot seeding or maturity override is used.
3. The file-based Bun driver accepts only recipe ID, network magic and ordered named raw blocks. The narrow Python mode uses pinned CBlock/transaction helpers to reconstruct branch-local referenced outputs, including transaction-order same-block spends. BASIC commitments feed the pinned peer serializers; only configured network magic is replaced after strict frame validation.
4. Finite batches retain independently produced ancestor deltas, rather than a copied UTXO map per block. Limits are at most 1,100 active blocks plus 8 branch blocks, 256 cases per batch, 8 MiB input, 16 MiB output and 10 seconds per child. Duplicate names/raw identities, unresolved/future/sibling-only prevouts, unexpected expectation fields, bad raw hex, pin drift, oversized I/O, child failures and malformed/self-inconsistent child results fail visibly.

## Executed Cost Probe and Cleanup

The early serial acceptance-only probe ran before multiplying any accepted fixture: 13,093 ms, 1,060,526 logical retained body/undo bytes, 3,723,264 allocated file bytes and successful all-handle-close reopen plus deletion. It did **not** prove an installed BASIC owner or indexed completion.

The final fresh **indexed** probe measured:

| Observation | Actual result |
| --- | --- |
| Consensus-accepted canonical blocks | 1,001; heights 0..1000; normal maturity100 |
| Normal bounded owner catch-up turns | 125; cap256 |
| Probe elapsed including raw oracle/assertions | 31,385 ms |
| Logical retained body/undo bytes | 1,060,526 |
| Temporary datadir allocated file bytes after flush | 4,005,888 |
| Accepted, processed and safe durable endpoint | h1000 with actual matching coins B/metadata/checkpoint |
| Independent BASIC comparison | all1,001 stored byte/hash/header triples |
| Independent spent facts versus persisted undo | all1,000 non-genesis blocks |
| All-handle-close reopen and datadir removal | succeeded; path absent afterward |

Allocated file bytes are the sum of filesystem `st_blocks * 512` at the post-flush sample, not an RSS, peak-space, Fjall-overhead or archive-scale guarantee. Logical payload accounting excludes index overhead. No pressure rows or 550MiB retention case were created. Only one fixture database was live at a time. Three closed datadirs left by failed setup probes were explicitly removed after all test processes exited.

## Verification

Every Bun command/child ran with `PATH=/tmp/open-bitcoin-bun-1.3.9-fresh:$PATH`; Bun reported1.3.9. Cargo used pinned Rust1.94.1 and ran serially through the timing wrapper.

- `bun run scripts/command-timings.ts run --key phase162-recipe -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --all-features phase162_recipe -- --test-threads=1 --nocapture`: final indexed probe **2 passed**,0failed,1,635filtered; test execution31.66s, compilation7.43s. Nonzero discovery verified.
- `bun test ./scripts/phase162-filter-oracle.test.ts`: final **25 passed**,0failed,39expect calls,2.02s. Includes raw mutation, positive h1000 branch checkpoints, all three response families, bounded batches, schema/provenance refusals and stdin/API equivalence.
- `bun test ./scripts/phase162-filter-oracle.test.ts ./scripts/generate-peer-filter-fixtures.test.ts ./scripts/generate-basic-filter-vectors.test.ts`: **79 passed across2 discovered files**,0failed,141expect calls,12.41s. The last requested basic-vector test path does not exist and contributed no tests; actual default BASIC preservation is covered by the existing peer suite and the explicit generator check below.
- `bun test ./scripts/check-phase154-basic-filters.test.ts`: **28 passed**,0failed,53expect calls,518ms.
- `bun scripts/check-phase154-basic-filters.ts`: provenance/evidence/scoped-claim guard passed.
- `bun scripts/generate-basic-filter-vectors.ts --check`: unchanged ten-row pinned corpus and frozen edge commitments reproduced.
- `bun scripts/generate-peer-filter-fixtures.ts --check`: unchanged six-family/source-arithmetic fixtures reproduced;1,458,391 fixture bytes.
- `bun scripts/bright-builds-check.ts all`:0findings.
- `bun scripts/check-parity-breadcrumbs.ts`: passed for1,123 Rust files after parent registration.
- `bun run scripts/command-timings.ts run --key phase162-recipe-clippy -- cargo clippy --manifest-path packages/Cargo.toml -p open-bitcoin-node --all-targets --all-features -- -D warnings`: passed; final5.62s.
- `bun run scripts/command-timings.ts run --key phase162-format-check -- cargo fmt --manifest-path packages/Cargo.toml --all --check`: passed.
- Explicit `rustfmt --edition 2024 --check --config skip_children=true` on shared recipe and adapter: passed. Python AST parse, owned tracked diff whitespace and new-file length/trailing-whitespace checks passed.
- No repository TypeScript typecheck/lint configuration exists for these standalone Bun scripts; executed Bun tests and existing strict native claim/schema checks cover their scoped validation. Parent full default `bash scripts/verify.sh` and independent source/security reviews remain required for phase completion.

## Initial Failures and Approved Adapter Replan

1. RED Rust run failed because the shared include file did not yet exist; RED Bun run failed because its driver module did not yet exist. These are recorded as compile/module-discovery RED signals, not passing behavior tests.
2. First recipe compile revealed the wallet-owned AddressNetwork import and signed satoshi type; both were corrected.
3. Opening an empty datadir with configured Enabled truthfully refused `validated genesis history required`.
4. Accepting genesis under PreserveSaved and calling enable afterward did not install an absent runtime owner; a stronger owner assertion found None. The parent explicitly approved closing after genuine genesis flush and reopening with configured Enabled.
5. With a real installed owner, the safe endpoint remained h0 after only one bounded turn. This was truthful scheduled lag, not a persistence or production bug. The parent explicitly approved the existing bounded finish pattern before the final flush/promotion.
6. The final sequence is PreserveSaved → actual genesis acceptance/Alwaysflush → all-handle close → configured Enabled reopen with ownerSome → remaining ordinary connects → at most256 normal owner turns until authoritative lag0 → Alwaysflush → actual safe-fence promotion.
7. Two new Bun test doubles initially returned the pin for both rev-parse and dirty-status probes and therefore stopped at the dirty-source guard. They were corrected to distinguish Git operations; no guard was weakened.
8. Scoped Clippy requested `unwrap_or_default()` for the absent parent hash; the recipe was simplified accordingly and lint rerun passed.

These are test-adapter/oracle setup fixes within ownership. No production defect or architecture change was found. The explicit simplification pass retained linear ancestry deltas, reused the existing strict peer-frame validator and shared a single hex rendering closure.

## Parent Handoff and Registry Additions

Exact new first-party Rust path:

- `packages/open-bitcoin-node/src/sync/tests/filter_index/phase162.rs`

Exact breadcrumb anchors already applied by the parent under group `phase162-continuous-validated-recipe`:

- `packages/bitcoin-knots/src/validation.cpp`
- `packages/bitcoin-knots/src/index/blockfilterindex.cpp`
- `packages/bitcoin-knots/src/node/blockstorage.cpp`

The source file contains that exact comment block. `packages/testdata/phase162/recipe.rs` is outside the first-party src/tests registry, remains test-only and imports only its containing module's `core_types` alias plus std. No child Rust test modules were introduced.

Plan01 parity evidence paths for the parent's eventual integrated parity registry entry are the shared recipe, node adapter, `scripts/phase162-filter-oracle.ts`, `scripts/phase162-filter-oracle.test.ts` and extended `scripts/basic-filter-oracle.py`. Independent source anchors additionally include `src/blockfilter.cpp`, `src/net_processing.cpp`, pinned `test/functional/test_framework/messages.py`, `test/functional/test_framework/p2p.py` and SipHash helper under `packages/bitcoin-knots/`. No new intentional production behavior difference is claimed.

Test-scoped node helpers available to later plans: `AcceptedRecipe::{new,finish,flush,cleanup}`, `oracle_input(&Recipe, Option<(ancestor,&[Block])>)` and `independent_oracle(&Value)`. Shared APIs include `Recipe::{new,fork,fork_spent_scripts}`, blocks/spent scripts, wallet funding outpoint/script, actual params/flags, recipe ID and published descriptor.

**Plan02 must preserve the configured Enabled owner invariant, not reuse this fully indexed probe or its direct turn strategy.** It may accept the full chain with indexing disabled, flush/close, then use configured Enabled startup to obtain a real partial prefix and exercise its actual zero-peer idle worker. Direct owner turns here prove only the cost-probe adapter.

## Task Commits

Task1, Task2 and this summary: **explicitly deferred by D-14**. No staging, commits, pushes, branch changes, hook bypass or parent-owned state/requirement mutations were performed.

## Known Stubs

None. No placeholder, TODO/FIXME or mock commitment source remains in owned implementation. The deterministic alternate branch is raw test recipe/oracle evidence here; actual accepted reorg, retention, daemon scheduler, wallet and socket proofs belong to their later plans.

## Threat Mitigations and Residual Limits

- T-162-01-1: raw-input-only public schema, pinned decoder and independently resolved prevouts; mutated raw funding changes spent facts/commitment, sibling-only references fail, and actual accepted undo plus stored BASIC commitments match independent output.
- T-162-01-2: fixed argv, existing bounded child runner/foreign-Git environment, finite recipe/batch caps, strict bounded response validation and explicit deadline/output-failure tests.
- T-162-01-3: scripts, identifiers and raw commitments remain in hermetic test I/O. Summary reports only fixed public recipe ID and aggregate measured facts; operator/support schemas are unchanged.

No new production trust surface was introduced. This plan does not prove prune deletion, actual idle-worker scheduling, accepted branch application, post-prune RPC/socket writes, wallet refusal, public defaults, archive scale, production readiness, funds safety or milestone shipment. Independent final source/mitigation reviews remain parent gates.

## Self-Check: PASSED

All four created source files and this summary exist. Baseline a77edab276a9daa086055aa61e6143ce86bc1218 remains HEAD and is a valid commit; no task commits exist because D-14 defers them. Executed nonzero Rust/Bun results, parent breadcrumb registration and actual cleanup evidence are recorded above. Parent-owned state updates remain delegated.

## Supplemental Foundation Review Fixes | 2026-10-11T17:36:29Z

Both actionable findings in `162-FOUNDATION-REVIEW.md` were corrected within Plan01 ownership. The earlier subprocess containment claim covered Python/Git grandchildren and was insufficient for the outer Bun process; the correction below supersedes that claim. Independent review/final phase verification remain parent gates.

### WR-01: Bound the outer process and its cleanup

Added `packages/testdata/phase162/process.rs`, a standard-library-only test helper shared by the node and daemon callers. The node wrapper now calls `process::run(&mut Command, Vec<u8>, process::ORACLE_LIMITS)`; the daemon executor independently migrated its owned caller to the same API. No daemon source was changed by this executor.

The helper rejects more than 8 MiB of stdin before spawning, concurrently writes stdin and drains stdout/stderr, caps retained stdout at 16 MiB and stderr at 64 KiB, and supervises write/read/wait with one 150-second total deadline. Failure reporting includes bounded counts and at most a 2 KiB stderr excerpt. A child guard kills/reaps on early returns or unwinding; a separate native-signal-child guard also kills/reaps on unexpected return/unwinding. Output-cap events reach the supervisor immediately while the worker drains further bytes without retaining them.

The parent explicitly approved the Unix signal seam: `CommandExt::process_group(0)` creates a fresh child group; fixed `/bin/kill -KILL -- -<derived-child-pid>` argv signals only that owned group. PID1 and this parent PID are refused; no shell, unsafe code, libc or dependency is used. The native signal command has a one-second wait cap, and worker drain/join has a two-second cleanup cap. Incomplete cleanup returns an explicit failure instead of hanging or claiming success. Non-Unix startup returns Unsupported before spawning. Executed portability evidence here is native macOS; no Windows or executed Linux claim is made.

Nine ordinary tests execute: unread sufficiently large stdin, a sleeping child, stdout overflow, stderr overflow, nonzero exit, early stdin closure, oversize input before spawn, successful capture/reap, and an actual descendant inheriting both output pipes after its parent exits. Each failure case asserts finite completion, capped retained output and actual child reaping. One ignored entrypoint is only a subprocess control child and is explicitly launched by these ordinary tests; it is not an ignored regression test. Including this helper in both crates adds one such ignored control entrypoint per crate to eventual native test counts.

The first standalone controls run selected zero tests in its subprocesses because `module_path!()` included the crate prefix while libtest names omit it: 2 passed, 6 failed, 1 ignored. The selector was corrected before treating the controls as evidence. The final standalone run executed all controls: **9 passed, 0 failed, 1 ignored, 2.30s**. Scoped Clippy then requested collapsing one nested guard; the guard was simplified and Clippy rerun passed. No checks or process bounds were weakened.

### WR-02: Separate canonical and branch budgets

The Bun schema validator now counts the initial active prefix separately from all post-divergence branch records. It retains the 1,108 total-row cap, 1,100 canonical-row cap and eight-branch-row cap while accepting branch node heights through h1107. Parentage, unique identities, raw header identities and branch-local prevout resolution remain enforced.

Added executed positive coverage for a 1,100-record canonical prefix plus a two-block fork from ancestor1098 extending through h1100. The independent Bun encoder and the actual shared Rust `Recipe::new(..., 1100).fork(1098, 2, ...)` both reach the raw oracle successfully. Separate negative controls reject 1,101 canonical records and a ninth branch record while staying below the competing budget. No additional Fjall history was created for the upper-bound proof.

### Fresh verification and handoff

All Bun/child invocations continued using the pinned1.3.9 PATH. Cargo started only after the parent granted the serial build handoff; the target was returned to the daemon executor after final scoped lint completed.

- `bun test ./scripts/phase162-filter-oracle.test.ts`: **28 passed**, 0 failed, 44 expectations, 2.72s.
- `bun run scripts/command-timings.ts run --key phase162-foundation-review-fixes -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --all-features phase162_ -- --test-threads=1 --nocapture`: **12 passed**, 0 failed, 1 ignored control entrypoint, 1,635 filtered; 34.55s tests plus 11.92s compilation. Includes the real accepted Fjall/oracle comparison, actual shared upper-bound recipe and all nine process controls.
- That fresh indexed history measured **30,976 ms**, 125 turns, **1,060,526 logical body/undo bytes**, **4,005,888 allocated file bytes** and successful all-handle-close reopen/removal. Accepted/processed/safe h1000, independent stored commitments, actual undo and wallet funding assertions all passed.
- After adding the native-signal cleanup guard, `bun run scripts/command-timings.ts run --key phase162-process-review-controls -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --all-features phase162_process -- --test-threads=1 --nocapture`: **9 passed**, 0 failed, 1 ignored control entrypoint, 2.34s tests plus 8.59s compilation.
- Final standard-library helper compilation/execution: `rustc --edition 2024 --test packages/testdata/phase162/process.rs -o /tmp/open-bitcoin-phase162-process-tests` followed by that executable with `phase162_process --test-threads=1`: **9 passed**, 0 failed, 1 ignored, 2.30s after final guard simplification.
- `bun run scripts/command-timings.ts run --key phase162-foundation-review-clippy -- cargo clippy --manifest-path packages/Cargo.toml -p open-bitcoin-node --all-targets --all-features -- -D warnings`: final **passed**, 6.63s.
- Explicit rustfmt checks on helper/adapter and scoped diff whitespace checks passed. The helper has 579 lines, below the 628-line boundary. No new first-party src/tests file or breadcrumb registration was needed for this testdata helper.

The inherited consensus boundary remains explicit: the recipe follows actual Open Bitcoin Regtest parameters (maturity100, subsidy-halving interval210,000); pinned Knots Regtest has halving interval150. This proves Open Bitcoin runtime acceptance plus independently pinned BASIC/header/wire derivation, not whole-chain Knots Regtest consensus differential acceptance. Production consensus behavior is unchanged.

**Review-fix self-check: PASSED.** The new shared helper exists and both current callers use its bounded API. Review controls and upper-bound cases have nonzero executed discovery. No production dependency/API or parent-owned state/requirement/registry edit was added. Requirements remain empty in this summary, all commits remain explicitly deferred by D-14, and parent independent review/full native/lifecycle gates remain outstanding.
