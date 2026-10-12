---
phase: 162-real-prune-retention-and-integrated-parity-proof
reviewed: 2026-10-11T17:37:46Z
initial_reviewed: 2026-10-11T17:23:03Z
depth: standard
diff_base: a77edab276a9daa086055aa61e6143ce86bc1218
review_scope: supplemental-plan01-foundation
files_reviewed: 8
files_reviewed_list:
  - packages/testdata/phase162/recipe.rs
  - packages/testdata/phase162/process.rs
  - packages/open-bitcoin-node/src/sync/tests/filter_index/phase162.rs
  - packages/open-bitcoin-node/src/sync/tests/filter_index.rs
  - scripts/phase162-filter-oracle.ts
  - scripts/phase162-filter-oracle.test.ts
  - scripts/basic-filter-oracle.py
  - docs/parity/source-breadcrumbs.json
findings:
  critical: 0
  warning: 0
  info: 0
  total: 0
resolved_findings:
  warning: 2
  total: 2
status: clean
---

# Phase 162: Supplemental Foundation Code Review

**Reviewed:** 2026-10-11T17:37:46Z (initial review: 2026-10-11T17:23:03Z)
**Depth:** standard
**Files Reviewed:** 8
**Status:** clean; two original warnings resolved

## Summary

This early review covers the frozen Plan01 foundation, including complete new/untracked source files and tracked changes against the stated baseline. It does not approve the phase, activate requirements, or review the concurrently executing daemon plan. The initial review found two warnings concerning subprocess containment and the upper recipe boundary; both are resolved in the corrected foundation. No unresolved issue was found. The original findings and original evidence are preserved below, followed by the resolution evidence.

The review used root `AGENTS.md`, `AGENTS.bright-builds.md`, `standards-overrides.md` (no substantive exception), the architecture, code-shape, testing, verification, local-guidance, Rust and TypeScript/JavaScript standards, current `162-CONTEXT.md`, Plan01 and its summary. Both active lesson files were read completely: 7,188 bytes and 2,397 estimated tokens. No project skill directories were present. Source was left unchanged; no Cargo, staging or commits were performed.

## Original Warnings (Resolved)

### WR-01: Rust oracle wrapper does not contain the outer Bun subprocess — resolved

**File:** `/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-node/src/sync/tests/filter_index/phase162.rs:183-203`

**Issue:** `independent_oracle` spawns Bun with three pipes, performs blocking `write_all` to stdin and then calls unrestricted `wait_with_output`. The 16 MiB stdout assertion runs only after the child exits and all stdout/stderr have been allocated; stderr has no limit. A Bun process that stalls before consuming stdin, stops progressing between Python calls, or emits excessive output can therefore hang or exhaust the test runner even though its Python/Git grandchildren use `runBounded`. A failed stdin write panics before killing and reaping the child. The existing deadline/output tests exercise the inner Bun runner and do not protect this Rust boundary. This affects test reliability and the Plan01 subprocess containment claim.

**Fix:** Keep the correction test-only and use the standard library: serialize and reject input above a named finite input budget before spawn; move stdin writing and capped stdout/stderr draining to workers; supervise the child with `try_wait`, `Instant` and a finite total deadline chosen for the bounded multi-batch driver; report worker failures through a channel. On deadline, either output cap, or stdin/read failure, kill and reap the child before joining workers or reporting failure. Preserve a bounded stderr excerpt for nonzero exits. A cleanup guard must also reap the child on an unexpected early return/panic. Do not implement the cap by calling `wait_with_output` first, or the deadline only after `write_all` returns. Reuse an existing test process supervisor if one already supplies these guarantees.

Add focused tests with a small test child that (1) never reads sufficiently large stdin, (2) sleeps without exiting, (3) continuously writes stdout, (4) continuously writes stderr, and (5) exits nonzero or closes stdin early. Assert finite completion, bounded captured output, error propagation and child reaping. Keep the real raw-oracle success comparison. These tests should avoid spawning another genuine Fjall history.

### WR-02: Valid bounded forks extending beyond h1099 are rejected — resolved

**File:** `/Users/peterryszkiewicz/Repos/open-bitcoin/scripts/phase162-filter-oracle.ts:76-79`

**Additional locations:** `/Users/peterryszkiewicz/Repos/open-bitcoin/scripts/phase162-filter-oracle.ts:89`; `/Users/peterryszkiewicz/Repos/open-bitcoin/packages/testdata/phase162/recipe.rs:33-34,68-80`

**Issue:** The declared contract allows up to 1,100 active blocks plus eight branch blocks. `Recipe::new` accepts 1,100 blocks and `fork` accepts up to eight children from any allowed ancestor. The oracle instead rejects every row with absolute height at least 1,100, including branch rows, and response validation separately caps node height at 1,099. A 1,100-block active prefix followed by fork h1099..1100 from ancestor1098 contains only 1,102 total rows and two branch rows, but fails with `active block budget exceeded`. Thus an input produced within the shared recipe API's documented bounds cannot reach the independent oracle. The default h1000 scenario is unaffected.

**Fix:** Bound the initial active-prefix count independently from the post-divergence branch count. Retain the total cap of 1,108 and branch cap of eight, and allow branch heights through the resulting maximum h1107. Update `validateNode` consistently. Add an upper-bound positive test using the shared API's largest active/fork shape, plus separate negative controls for a 1,101-row active prefix and a ninth branch row. A focused validator-only reproduction already confirmed the h1100 rejection without invoking a child.

## Evidence Integrity and Verified Behavior

- The configured-owner correction is present: genuine genesis acceptance and `Always` flush precede all-handle close and configured `Enabled` reopen, with an actual owner assertion at adapter lines55-77. The subsequent probe obtains ordinary acceptance receipts; it does not seed a snapshot or change coinbase maturity.
- Probe lines244-257 compare every one of the 1,001 stored BASIC byte/hash/header triples with the independently decoded raw-block oracle. Lines260-285 load undo for every one of the 1,000 non-genesis blocks and compare historical/same-block spent scripts against both raw recipe facts and independent Python output. Rust-generated filter values are comparison subjects only, never oracle expectation inputs.
- Probe lines287-339 assert actual durable coins B, chain metadata tip, descriptor funding coin, processed h1000, safe durable h1000 and the persisted checkpoint matching the final record. Cleanup closes the runtime, successfully reopens the store, deletes the datadir and checks absence on the success path. The recorded125 turns/31,385ms measurement belongs to the executor's prior Rust execution; this review inspected those assertions but did not rerun Cargo.
- Python reconstructs branch-local output availability from ancestry deltas and transaction order, removes same-block spends from the retained delta, rejects unresolved/sibling-only references, and derives commitments from raw decoded blocks. The Bun boundary rejects caller-provided spent/filter/hash/header expectations, checks the exact clean tracked Knots pin, validates raw/predecessor/commitment identity, and normalizes only frame magic after strict pinned serializer validation.
- The independently encoded Bun recipe proves distinct positive h1000 checkpoint ancestry for both leaves and all three response families. The Rust alternate branch is deterministic raw fixture evidence here; actual accepted branch application remains a later-plan gate.
- Simplification review found the ancestry-delta approach and shared peer serializer validator appropriate. The material simplification opportunity is one reusable bounded Rust subprocess helper for WR-01, rather than repeating unconstrained process handling in later callers.

## Executed Review Checks

All Bun invocations and their children used `PATH=/tmp/open-bitcoin-bun-1.3.9-fresh:$PATH`; version1.3.9 was confirmed.

- `bun test ./scripts/phase162-filter-oracle.test.ts ./scripts/generate-peer-filter-fixtures.test.ts`: **79 passed,0failed,141expect calls across2files,12.33s**. This includes the raw-spend mutation, sibling refusal, strict schemas, child-failure propagation, positive checkpoint ancestry, real inner-helper timeout and unlimited-writer refusal, and unchanged default BASIC corpus reproduction.
- `bun scripts/generate-basic-filter-vectors.ts --check`: unchanged ten pinned agreements and frozen edge commitments reproduced.
- `bun scripts/generate-peer-filter-fixtures.ts --check`: unchanged six families/source arithmetic reproduced,1,458,391 fixture bytes.
- Scoped tracked `git diff --check`: passed. The new files were read in full rather than inferred from `git diff`.
- The whole-tree breadcrumb checker currently reports `phase162-actual-idle-worker: mapping did not match any in-scope Rust file`, reflecting a concurrent Plan02 registry entry whose file is not yet created. The frozen foundation group and its three source comments agree. This transient wider-tree state is not attributed to the foundation.

## Scope Limits

The current Open Bitcoin runtime Regtest parameters retain `subsidy_halving_interval: 210_000` (`packages/open-bitcoin-node/src/sync/types.rs:151`), while the pinned Knots Regtest parameters use150 (`packages/bitcoin-knots/src/kernel/chainparams.cpp:568`). The recipe issues50BTC through h1000 and follows the actual Open Bitcoin runtime parameters as D-01 requires. Therefore this is normal-maturity **Open Bitcoin runtime validation** plus independently pinned BASIC/header/wire derivation; it is not whole-chain Knots Regtest consensus differential acceptance. Current proof documentation must preserve that inherited maturity/halving boundary. No production consensus change is requested by this review.

Prune deletion, pressure retention, actual daemon idle scheduling, accepted reorg, post-prune authenticated RPC/socket writes, wallet refusal and final full native/lifecycle verification remain later-plan gates. No archive-scale, public-default, readiness, funds-safety or milestone-shipment claim follows from this foundation review.

***

## Corrected Foundation Re-review: 2026-10-11T17:37:46Z

### WR-01 Resolution

The node oracle now invokes the shared test-only `process::run` with `ORACLE_LIMITS` (`packages/open-bitcoin-node/src/sync/tests/filter_index/phase162.rs:184-195`). The daemon oracle also uses that same helper and limits; this observation checks the handoff only, and does not review the daemon implementation as a whole.

The complete new `packages/testdata/phase162/process.rs` was inspected, including failure paths and controls:

- Lines20-24 define8MiB input,16MiB stdout,64KiB stderr and a150-second total process deadline. Input is rejected before spawn at lines340-343. The150-second cap covers the bounded multi-batch Bun driver; its Git/Python subprocesses retain their10-second caps.
- Lines345-370 start timing before process creation, extract all three pipes and put stdin writing and both drains in separate workers. The supervisor remains able to enforce the deadline while stdin is blocked. Reader workers retain at most their respective caps, send a failure immediately on overflow, and drain excess bytes into a sink while cleanup proceeds (lines193-251). Each worker sends at most one bounded result; the channel does not accumulate streaming output.
- Lines69-74 create a fresh Unix process group from the spawned child. The group-kill adapter receives only that child-derived PID, refuses parent/system identities, uses fixed `/bin/kill` argv and no shell, and is independently supervised with a one-second kill-command deadline (lines105-151). No raw recipe field supplies a PID, process group or command.
- Lines154-180 and372-401 perform group cleanup, direct-child termination when still needed and reaping before joins on success and failure paths. The drop guard repeats cleanup after unexpected unwinding. The shared two-second drain/join budget reports an explicit incomplete-cleanup failure instead of joining an inherited pipe indefinitely. Expected helper descendants inherit the owned group; the inherited-pipe control exercises an exited leader with a still-running descendant.
- Lines452-538 contain nine ordinary tests with specific failure reasons, finite elapsed-time assertions, captured-byte bounds and direct-child reaping assertions. They cover blocked stdin, a sleeping child, inherited-pipe descendant, stdout and stderr overflow, nonzero exit, early stdin close, input rejection before spawn and successful capture. The single ignored child entry is executable subprocess-control infrastructure, not a skipped behavior assertion.

These changes resolve the original blocking-stdin, output-allocation, deadline and child-cleanup defect. The supervisor is test-only and Unix-specific; this is appropriate to the existing Unix fixture footprint code. No production process or authority API is introduced.

### WR-02 Resolution

`scripts/phase162-filter-oracle.ts:62-86` now counts the uninterrupted initial active prefix separately from records after divergence. It enforces active<=1100, branches<=8 and total<=1108. The response-node height cap is now1107 (line94), consistent with the maximum resulting ancestry. Parent identities still must precede their children; the correction does not weaken schema, identity, raw commitment or independent prevout checks.

The new positive Bun test at `scripts/phase162-filter-oracle.test.ts:121-135` drives the real oracle with1100 active blocks plus a two-block fork throughh1100 and checks its independently reconstructed spent scripts. Separate negative controls at lines137-153 refuse1101 canonical records and a ninth branch record while each remains within the other budget. The Rust recipe-to-oracle boundary is also explicitly covered at `packages/open-bitcoin-node/src/sync/tests/filter_index/phase162.rs:198-228`, including comparison of both fork spent-script lists.

### Re-review Verification and Limits

- Independently reran `PATH=/tmp/open-bitcoin-bun-1.3.9-fresh:$PATH bun test ./scripts/phase162-filter-oracle.test.ts`: **28passed,0failed,44expect calls,3.28s**. All three new boundary controls passed, along with the original independent provenance/refusal tests.
- The executor reports the final focused node run as **12passed,0failed,1ignored**, including the nine ordinary process controls and three recipe tests, and strict Clippy passing. The fresh indexed probe reports30,976ms,125 turns, all1001 stored filter comparisons, all1000 undo comparisons, actual safeh1000 and cleanuptrue. Those Rust runs were not duplicated because the daemon executor owns the shared Cargo target. The re-review inspected the concrete assertions and helper implementation rather than using counts alone.
- The inherited Open Bitcoin210000 versus pinned Knots Regtest150 subsidy-halving boundary remains unchanged. `162-12-PLAN.md:74` now explicitly requires current proof documents to distinguish configured Open Bitcoin runtime acceptance from whole-chain Knots consensus differential acceptance. That documentation requirement is a later-plan gate, not a production consensus fix or unresolved foundation defect.
- The earlier79-test and unchanged legacy-corpus results remain historical evidence; no unrelated broad suite was repeated. No source files, production behavior, commits or Cargo artifacts were changed by this review. Full phase review and native/lifecycle verification remain pending after all plans.

**Current conclusion:** Both original warnings are resolved. The corrected foundation has0critical,0warning and0info findings within this eight-file scope.

_Reviewer: gsd-code-reviewer_
_Depth: standard; supplemental frozen Plan01 scope_
