---
phase: 156-index-owned-manual-and-automatic-prune-coordination
reviewed: 2026-10-05T02:23:48Z
initial_reviewed: 2026-10-05T01:50:48Z
depth: standard
diff_base: 3f5189eb
scope: current uncommitted packages changes through Plan 07, Fjall dependency repair and Plan 08 supplemental scripts/docs
files_reviewed: 67
files_reviewed_list:
  - packages/open-bitcoin-chainstate/src/filter_index.rs
  - packages/open-bitcoin-chainstate/src/filter_index/lifecycle.rs
  - packages/open-bitcoin-chainstate/src/filter_index/tests.rs
  - packages/open-bitcoin-chainstate/src/filter_index/tests/commitments.rs
  - packages/open-bitcoin-chainstate/src/filter_index/tests/lifecycle.rs
  - packages/open-bitcoin-node/Cargo.toml
  - packages/open-bitcoin-node/src/chainstate.rs
  - packages/open-bitcoin-node/src/chainstate/fjall_store.rs
  - packages/open-bitcoin-node/src/chainstate/flush_lifecycle.rs
  - packages/open-bitcoin-node/src/chainstate/flush_lifecycle/fjall_sink.rs
  - packages/open-bitcoin-node/src/chainstate/flush_lifecycle/prune_apply.rs
  - packages/open-bitcoin-node/src/chainstate/flush_lifecycle/prune_apply/tests.rs
  - packages/open-bitcoin-node/src/chainstate/flush_lifecycle/tests/execute_flush.rs
  - packages/open-bitcoin-node/src/network/runtime_authority.rs
  - packages/open-bitcoin-node/src/network/runtime_authority/automatic_prune.rs
  - packages/open-bitcoin-node/src/network/runtime_authority/automatic_prune/tests.rs
  - packages/open-bitcoin-node/src/network/runtime_authority/automatic_prune/tests/fixtures.rs
  - packages/open-bitcoin-node/src/network/runtime_authority/automatic_prune/tests/protection.rs
  - packages/open-bitcoin-node/src/network/runtime_authority/automatic_prune/tests/writers.rs
  - packages/open-bitcoin-node/src/network/runtime_authority/filter_index.rs
  - packages/open-bitcoin-node/src/network/runtime_authority/prune_flush.rs
  - packages/open-bitcoin-node/src/network/runtime_authority/tests.rs
  - packages/open-bitcoin-node/src/storage/filter_index.rs
  - packages/open-bitcoin-node/src/storage/filter_index/ownership.rs
  - packages/open-bitcoin-node/src/storage/filter_index/tests.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/filters.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/filters/lifecycle.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/filters/ownership.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/filters/publication.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/filters/startup.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/filters/tests.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/filters/tests/faults.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/filters/tests/lifecycle.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/filters/tests/ownership.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/prune.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/prune/records.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/tests/prune_records.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/tests/prune_unlink.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/tests/prune_unlink/protection.rs
  - packages/open-bitcoin-node/src/sync/tests/filter_index.rs
  - packages/open-bitcoin-node/src/sync/tests/filter_index/faults.rs
  - packages/open-bitcoin-node/src/sync/tests/filter_index/lifecycle.rs
  - packages/open-bitcoin-node/src/sync/tests/filter_index/prune_coordination.rs
  - packages/open-bitcoin-node/src/sync/tests/filter_index/prune_faults.rs
  - packages/open-bitcoin-node/src/sync/tests/filter_index/recovery.rs
  - packages/open-bitcoin-node/src/sync/tests/filter_index/startup.rs
  - packages/open-bitcoin-node/src/sync/tests/filter_index/startup/recovery.rs
  - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/automatic_prune.rs
  - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/automatic_prune/fixtures.rs
  - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/automatic_prune/index_protection.rs
  - packages/open-bitcoin-rpc/src/context/prune.rs
  - packages/open-bitcoin-rpc/src/dispatch.rs
  - packages/open-bitcoin-rpc/src/dispatch/prune.rs
  - packages/open-bitcoin-rpc/src/dispatch/prune/tests.rs
  - packages/open-bitcoin-rpc/src/dispatch/prune/tests/ownership.rs
  - packages/open-bitcoin-rpc/src/http/tests.rs
  - packages/open-bitcoin-rpc/src/http/tests/prune_ownership.rs
  - scripts/check-phase156-prune-coordination.ts
  - scripts/check-phase156-prune-coordination/contracts.ts
  - scripts/check-phase156-prune-coordination/rust-evidence.ts
  - scripts/check-phase156-prune-coordination.test.ts
  - scripts/verify.sh
  - README.md
  - docs/parity/index.json
  - docs/parity/catalog/basic-compact-filters.md
  - docs/parity/checklist.md
  - docs/parity/source-breadcrumbs.json
generated_files_checked: 2
generated_files_checked_list:
  - packages/Cargo.lock
  - MODULE.bazel.lock
findings:
  critical: 0
  warning: 0
  info: 0
  total: 0
resolved_findings:
  critical: 0
  warning: 2
  info: 1
  total: 3
status: clean
native_verification: pending root-owned full native gate
git_finalization: pending root-owned consolidated commit after all gates
---

# Phase 156: Code Review Report

**Reviewed:** 2026-10-05T02:23:48Z; initial packages review 2026-10-05T01:50:48Z
**Depth:** standard
**Files Reviewed:** 67 source/manifest/script/doc files; two generated lockfiles checked separately
**Status:** clean

## Summary

The explicit scope was every existing path returned by `git diff --name-only HEAD -- packages MODULE.bazel.lock` at review time. The 57 nongenerated source/manifest files were reviewed in context, including every changed test module. Cargo and Bazel lockfiles were inspected for dependency consistency rather than counted as source files. No scoped path is ignored; `.claudeignore` and project skill directories are absent. Planning artifacts are review context, not source scope.

All reviewed files meet quality standards. No issues found. This means no actionable correctness, security, or maintainability finding in the reviewed change; it does not establish that the phase's complete native verification or later public activation surfaces have passed.

Material guidance came from `AGENTS.md`, `AGENTS.bright-builds.md`, placeholder-only `standards-overrides.md`, `standards/index.md`, architecture/code-shape/testing/verification/Rust standards, the Phase 156 context and execution evidence, and `156-FJALL-SHUTDOWN-FIX.md`. Both active lesson inputs were measured and read completely: 7,188 bytes and 2,397 conservative estimated tokens. No lesson content was omitted.

## Safety and Integration Assessment

| Concern                           | Reviewed behavior                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| --------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Ownership and work identity       | The bounded additive owner codec rejects invalid bytes and never infers Disabled from absence. Pure ownership requires valid state and covering protection. Private work tokens bind the same store incarnation, Active generation, exact checkpoint/fence/protection facts and preparation fence. Both record-only and checkpoint publishers revalidate before preparing writes.                                                                       |
| Reserved CRUD and map replacement | RPC context and managed authority refuse reserved-name set/clear even when the entry is absent. Concrete whole-map replacement holds publication, validates current ownership and preserves the exact fresh reserved entry; duplicate entries refuse. Unrelated named locks retain their ordinary behavior.                                                                                                                                             |
| Checkpoint release                | Publication proves the complete immutable forest/projection and current recovered coins/metadata fence, then SyncAll-publishes checkpoint, records/projection and full protection map atomically. Ahead records alone do not move the cursor. Ambiguous commit/reply failure poisons live publication until actual reopen.                                                                                                                              |
| Actual deletion                   | Snapshot loading validates the saved release against current recovered durable ancestry. Standalone intent, direct paired deletion and resumed deletion reload ownership under publication and bind the candidate's hash to its durable height. Required-input checks directly cover heights 0/1 and occur before destructive payload mutation or intent clearing. Ordinary locks and caller restrictions can only strengthen application restrictions. |
| Lock ordering                     | Managed calls hold authority before automatic state. Publication helpers release short snapshot guards before nested flush/payload effects. Actual paired deletion holds publication through payload mutation; guarded private helpers do not reacquire publication. No reverse acquisition introduced by the reviewed call paths was identified.                                                                                                       |
| Disable/re-enable                 | Disable stops issuance, advances/persists generation before releasing only the owned lock, and permits idempotent Disabled-with-lock completion. Re-enable validates the current fence, retained historical bodies/non-genesis undo and pending intent, then atomically publishes Active ownership plus conservative protection. Missing history refuses without acquisition or mutation.                                                               |
| Automatic decisions               | Fresh normalized full ownership and lock identity precedes both completed-measurement reuse and the periodic timer. Protection changes invalidate both gates; errors reset all cache/timer identity. Required-input bytes stay counted in total usage but are removed from the candidate budget. Concrete application remains the final safety gate after a stale snapshot.                                                                             |
| Tests and claims                  | Reviewed assertions cover actual Fjall close/reopen, stale work, clone ordering, software publication/lifecycle faults, runtime startup, manual application and authenticated RPC reserved ownership. Tests distinguish synthetic policy facts, sparse deletion fixtures, engine-accepted spend history and actual ordinary daemon retention with a legal 550 MiB target.                                                                               |

## Dependency Consistency

The manifest retains the existing Fjall dependency and disabled default features, selecting official upstream revision `aa30dca811399a201e0b9595da93a4582dcb2b57`. Parsed Cargo package comparison found only Fjall/lsm-tree 3.1.4 replaced by 3.1.10; every shared package entry is identical. Parsed Bazel metadata selects the same Fjall remote/commit and no stale Fjall or lsm-tree 3.1.4 reference remains. The repair evidence identifies a backend shutdown cause and an exact upstream fix; the reviewed first-party diff introduces no workaround that leaks a database, suppresses shutdown, disables durability or changes worker behavior.

This is consistency review of the pinned inputs and supplied repair evidence, not a fresh execution of the upstream suite or a claim that this snapshot includes every later upstream fix.

## Simplification Pass

One typed ownership model, one existing shared publication guard and one paired-delete owner supply the enforcement. Public snapshots remain facts; opaque same-store tokens supply write capability. Bounded ownership reads remain distinct from complete startup/publication integrity scans. Cache invalidation is centralized. No duplicate deletion authority, new production crate, hidden public activation route or causal root-fix issue was found.

## Verification and Limits

The reviewer executed read-only source/diff inspection, ignored-path checking, parsed dependency comparison and `git diff --check HEAD -- packages MODULE.bazel.lock`; the whitespace check passed. The reviewer did not invoke Cargo, Bazel, tests or `scripts/verify.sh`, preserving the root/executor's exclusive build lane. Targeted execution evidence belongs to the executors and dependency-repair artifact; the root-owned default native gate remains pending when this report is written.

Software fault and reopen assertions do not prove hardware power-loss or disk-controller resilience. The legal-target daemon fixture proves ordinary actual retention/deletion over codec-valid dense ancestry; it is explicitly not public-mainnet consensus sync or complete post-prune client serving evidence. Public configuration/activation, scheduled catch-up, runtime reorg orchestration, filter RPC, peers and broader operator surfaces remain deferred. Retained v2.4 advisories, including partial-batch support accounting, were not expanded into unrelated findings.

Plan 08's supplemental scripts/docs were initially excluded and are reviewed in the supplement below. No source file was modified, and no commit or push was made by the reviewer.

## Plan 08 Supplemental Review

**Reviewed:** 2026-10-05T02:16:02Z
**Additional scope:** Ten files listed after the original 57 paths in frontmatter: four TypeScript checker/test/module files, the native Bash verifier, README, parity catalog/checklist, machine parity ledger and source-breadcrumb registry. The combined source/doc scope is 67, with the original two generated lockfile consistency checks retained separately. All ten supplemental paths exist and none is ignored. TypeScript/JavaScript standards and the Plan 08 execution contract informed this review.

The supplement initially found two guard correctness issues and one incorrect provenance citation. All three were repaired by the source owner and the actual repaired code was re-inspected; they are retained below as resolved findings. The original packages review remains clean. Final unresolved finding counts are zero, with `status: clean` describing the reviewed repaired state rather than pretending the initial supplement was clean.

### WR-01: Unrelated deferred clause exempted a positive activation claim — Resolved

**File:** `scripts/check-phase156-prune-coordination.ts:184-200` (initial claim logic at lines 176-186).

**Issue:** The previous claim check exempted an entire sentence whenever it contained `remain deferred`. `Public filter index activation is enabled and scheduled catch-up remain deferred.` therefore escaped the guard, despite explicitly claiming a deferred product was enabled. The worker confirmed the mixed-clause mutation failed its expected refusal assertion before repair.

**Fix:** Split independent subject clauses and associate a positive predicate with its subject. Scan individual parity string values rather than relying on serialized neighboring fields for prose boundaries. The repaired code uses conjunction-aware clause splitting, subject-relative positive matching and recursive string traversal. Mixed-clause and parity-rationale mutation controls now pass. Precision: the originally suggested parity-rationale-only mutation already refused because a semicolon elsewhere in the serialized row separated the positive claim from deferred text; that was not an independently observed original failure. Recursive string scanning is related hardening, not a claimed separate RED reproduction.

### WR-02: Visibility-qualified test module could substitute for ordinary production code — Resolved

**File:** `scripts/check-phase156-prune-coordination/rust-evidence.ts:68-80`.

**Issue:** The initial test-module mask handled plain `mod` and `pub mod`, but not `#[cfg(test)] pub(crate) mod` and other qualified visibility forms. A removed ordinary function could consequently be supplied inside a test-only module while satisfying the named body anchors.

**Fix:** Recognize optional Rust visibility qualifiers when masking the complete balanced test-module body before ordinary production discovery. The repaired regular expression accepts `pub(crate)`, `pub(super)` and `pub(in crate::storage)`; `checkBody` requests production discovery. The worker reports all three new substitution controls failed before repair and passed 3/3 afterward. Comments, quoted/raw literals and ordinary test-only functions remain masked or refused, while executable test discovery uses its separate test path.

### IN-01: Cited pinned functional test did not exist — Resolved

**File:** `docs/parity/index.json:4266` (initial new citation at line 4275); related inherited citation at line 866.

**Issue:** `feature_blockfilterindex.py` did not exist in the materialized pinned Knots baseline. The new Phase 156 row repeated that inherited filename, leaving an auditable test reference unresolvable.

**Fix:** Use the actual pinned `feature_index_prune.py`, which exercises pruning with blockfilter indices and disabled/restarted indices, alongside `feature_pruning.py`. Both new Phase 156 and narrowly corrected inherited Phase 155 citations now resolve. The checker includes these selected files in required reads and validates their cited test anchors; missing-anchor and missing-file mutation controls were added. Phase 155 status and behavior scope were preserved by this citation correction.

### Completion Wording and Evidence Boundaries

The claim guard intentionally refuses an unqualified current full-native success assertion. The root's concrete post-verification wording, `Evidence for the full native gate is recorded in [Phase 156 verification](...)`, is accepted by the inspected guard: it points to separately earned canonical evidence without matching a shipped-product or unsupported success predicate. The current implementation does not require a future verification file while drafting. The root must replace pending prose and reconcile ledger status only after actual native and formal phase/lifecycle proof. This completion-wording concern is resolved by the agreed qualified link form; it is not an unresolved source finding.

The current catalog's node/RPC/daemon counts and logical-byte totals match the Plan 07 evidence summary. Historical Phase 154/155 verification is distinguished from pending Phase 156 root proof. Deferred public activation, catch-up, reorg/filter serving/operator/client scope and hardware limits remain explicit. The new verifier calls are actual `run_step` statements after the Phase 155 steps, outside the legacy command-order heredoc. New Rust files retain unique mapped groups with matching source breadcrumbs. No new dependency, effectful pure-core policy or production activation path was introduced by this supplement.

### Supplemental Verification and Simplification

The reviewer performed read-only source/diff review, JSON/evidence-path inspection, actual repaired-function inspection and scoped whitespace checks. The worker/root reports 81/81 checker tests with 145 assertions after the first claim/provenance repairs, followed by the three visibility-qualified controls passing 3/3 after their confirmed RED failures. The final combined sweep and full native/security/canonical proof remain root/executor-owned at this report timestamp. These execution results are attributed evidence; the reviewer invoked no checker, test, Cargo, Bazel or native verifier.

The explicit simplification pass retains one contract table, one narrow Rust source masker/body extractor and one current-claim scanner. Test and production discovery remain distinct; parity string traversal removes an accidental coupling to JSON serialization. The source guard remains supplemental structural evidence, not a Rust parser, runtime safety proof or replacement for the original typed owner/publication guard/paired-delete authority reviewed above.

## Targeted Review After First Native Attempt

**Reviewed:** 2026-10-05T02:23:48Z
**File:** `packages/open-bitcoin-node/src/storage/fjall_store/prune.rs:185-191`
**Finding:** None; current clean status and file counts are unchanged.

The root reports that the first full native attempt exited 101 after 2m56.280s at Clippy's `nonminimal_bool` finding. The preceding native guards and panic-site check passed, but this attempted full gate did not pass.

The reviewer inspected the actual code and diff for the root's narrow predicate simplification. The former `!maybe_position(...).is_some_and(|position| position.block_hash == saved.fence_hash())` is now `maybe_position(...).is_none_or(|position| position.block_hash != saved.fence_hash())`. Their refusal truth table is identical:

| Durable saved-height position | Previous refusal predicate | Current refusal predicate |
| ----------------------------- | -------------------------- | ------------------------- |
| Absent                        | true                       | true                      |
| Present, hash differs         | true                       | true                      |
| Present, hash matches         | false                      | false                     |

Absence and mismatch still return the same fail-closed error before payload effects. A match still continues to the existing checkpoint and candidate checks; it does not by itself authorize deletion. The surrounding ownership, recovered coins/metadata, publication synchronization and deletion ordering are unchanged. No source edit or Cargo/Bazel/test invocation was made by the reviewer. The root's fresh complete native run remains required to verify the final consolidated source.

______________________________________________________________________

_Reviewed: 2026-10-05T02:23:48Z_
_Reviewer: the agent (gsd-code-reviewer)_
_Depth: standard_
