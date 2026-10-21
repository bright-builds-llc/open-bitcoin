---
phase: 166-canonical-durable-bootstrap-and-genesis-basic
plan: "11"
subsystem: verification
tags: [canonical-genesis, semantic-guards, current-source-ownership, reviewed-renewal]
requires:
  - phase: 166-10
    provides: Actual durable qualification and unchanged-ancestry checkpoint count preservation
  - phase: 166-09
    provides: Configured daemon and existing-only operator canonical adapters
  - phase: 166-16
    provides: Sealed-consuming RPC factory, unavailable wallet and library worker registration
provides:
  - Explicit offline canonical bootstrap checker with independently reproduced genesis vectors
  - Fixed predicate, ownership and route mutation controls
  - Independently reviewed and narrowly renewed current source owners
  - Exact current-reader dependency handoff to Plan 12
affects: [166-12, 166-13, 166-14]
tech-stack:
  added: []
  patterns: [complete finite guard predicates, explicit early-success ownership, code-before-data review]
key-files:
  created:
    - scripts/check-phase166-canonical-bootstrap.ts
    - scripts/check-phase166-canonical-bootstrap/contracts.ts
    - scripts/check-phase166-canonical-bootstrap.test.ts
  modified:
    - scripts/check-phase157-index-catch-up.ts
    - scripts/check-phase157-index-catch-up/contracts.ts
    - scripts/check-phase157-index-catch-up.test.ts
    - scripts/check-phase164-canonical-identity.ts
    - scripts/check-phase164-canonical-identity.test.ts
    - scripts/check-phase164-canonical-identity/source-files.ts
    - scripts/check-phase164-canonical-identity/test-modules.ts
key-decisions:
  - Keep Phase 155 unchanged because its configured recovery and BASIC-before-prune contract still passes.
  - Bind complete promised predicates, bodies and depths instead of accepting retained predicate substrings.
  - Preserve historical Phase 157 evidence ownership while inspecting its actual current helper successors.
  - Renew only explicitly reviewed current rows after independent CODE acceptance.
  - Leave the current Phase 165 read-set pin, process rows, claims and default registration to Plan 12.
requirements-completed: [CBOT-02, CBOT-03, CWOR-04]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 166-2026-10-18T22-36-40Z
generated_at: "2026-10-20T20:25:10Z"
commits: false
duration: "58m recorded baseline-to-summary window"
completed: 2026-10-20
---

# Phase 166 Plan 11: Bounded Bootstrap Guards and Current-Owner Renewal Summary

**A 39-input offline guard now rejects named canonical authority, checkpoint, inspection and adapter bypasses, while current-owner data changes remain tied to independent source review.**

## Completion Boundary

Tasks: **2/2 scoped deliverables complete under the checked Plan 12 verification handoff**. Ten implementation/evidence files changed; the eleven-path amended plan also declared `scripts/check-phase155-filter-index.ts`, which remains unchanged. This summary is separate. Recorded baseline started 2026-10-20T19:27:07.515Z.

This is **not aggregate or whole-phase verification acceptance**. The corrected four-file run exits 1 because 52 imported Phase 165 controls encounter its intentionally unchanged current-input inventory pin before source reads. Plan 12 owns that precise transition and the complete Phase 165 rerun. Default registration remains Plan 12; full native, independent final reviews, LOC and normal-hook saving remain Plan 14/root.

No staging, commit, push, hook bypass, shared STATE/ROADMAP/REQUIREMENTS/task edit, LOC refresh, historical receipt change or historical provenance-input expansion occurred. Requirement frontmatter records this plan's contribution rather than whole-phase closure.

## Bounded Semantic Guard

The checker imports the existing exported `checkVectors` validator and explicitly reproduces the four complete genesis/proof/BASIC vectors. Altered body, proof, filter, predecessor or evidence-class fields refuse. Importing the checker is silent and does not launch its CLI or oracle. The loader uses fixed regular-file inputs, O_NOFOLLOW and a 512,000-byte limit; required missing inputs refuse visibly.

The final 106-case suite covers the actual reachable genesis and complete-coins constructors, strict stored records, exact-work ancestry, store/generation binding, atomic SyncAll publication, uncertain commit refusal, retained application lease and backend path, process-contained inspection, child status/reaping, source recheck and checked cleanup. It also covers the real checkpoint count writer, saved BASIC ownership, sealed RPC factory, unavailable wallet, selected parameters, pre-Tokio/pre-parser worker dispatch, existing-only CLI Status and retained ordinary wallet/soak gates.

Independent review exposed real checker gaps: retained condition tokens could sit behind false conditions, and nested successful returns could bypass proof. Actual RED receipts are retained below. The final finite policy binds 23 complete predicate/body blocks at their required lexical depths, nine mandatory first operations, complete earlier coins/stored/live/RPC refusal blocks, and successful-return restrictions on the fixed constructors. Three functions retain precisely counted legitimate early-success branches with their complete guarded bodies. This avoids a general Rust parser or a blanket ban on `false`.

These are conservative structural wiring controls plus source-derived vector checks. They do not execute Rust, grant runtime authority or replace accepted storage/process/checkpoint behavior receipts from Plans 03–10, 15 and 16. The fixed rule/control inventory is `.local/open-bitcoin-dev/phase166/plan11/predicate-coverage-refrozen.json`.

## Reviewed Current Successors and Exact Data Scope

The Phase 157 contracts now inspect the extracted daemon preflight/store/runtime helpers, actual consuming RPC mode selector, synchronous worker dispatch and delegated application wiring. The retry-join mutant now changes the actual optional-worker closure, with unique-anchor, before/after and changed-source assertions. Its former literal was absent and produced a no-op.

Only the two exact current startup helper paths project back to the original daemon owner for historical Phase 157 parity-evidence membership. The original catalog is unchanged; helper mutation controls still inspect the actual current source. Removing either the original daemon evidence or an unrelated original evidence path still refuses.

The Phase 164 route mutants target the actual daemon canonical opener/factory consumption and CLI observation child. Each proves a unique real edit, the intended Phase 164 boundary finding and a separate Phase 166 semantic finding. Existing incompatible-source, preservation, fixture, wallet and soak controls remain defined and enforced; the explicitly reader-blocked imported cases are identified below.

The exact ownership adjustment recognizes only the real Unix `open_lease.rs::acquire[1]` with its complete immediate cfg/visibility and the relocated test helper's canonical cfg(test) impl. It preserves all existing file/module/declaration checks and old Unix symlink exceptions. The helper retains the parser's mechanical `test: false` bucket; its explicit precheck proves test-only compilation and grants no ordinary authority. Eleven actual-source ownership controls cover missing, wrong, extra and false cfg, first/wrong owner and disabled helper cases.

Root obtained fresh independent CODE acceptance before granting data changes. The approved eight-file CODE snapshot is `.local/open-bitcoin-dev/phase166/plan11/approved-code-snapshot.json`. Final data renewal is exactly:

- 16 same-file edge renewals, five reviewed relocations and ten approved new edges: 606 → 616.
- Fifteen existing source-identity renewals and five exact helper identities: 192 → 197.
- Five exact helper memberships: 143 → 148.
- One daemon unit-test count, 3 → 4, and one canonical-admission cfg-test module addition.

The five helpers are `packages/open-bitcoin-node/src/storage/fjall_store/opening.rs`, `packages/open-bitcoin-node/src/storage/fjall_store/open_lease.rs`, `packages/open-bitcoin-rpc/src/bin/open_bitcoind/canonical_startup.rs`, `packages/open-bitcoin-rpc/src/context/canonical_genesis.rs` and `packages/open-bitcoin-cli/src/operator/runtime/support/canonical_genesis.rs`. No speculative run_application edge or unrelated row was added.

The original arrays and raw files are retained in `.local/open-bitcoin-dev/phase166/plan11/data-before.json`; exact approved deltas are in `.local/open-bitcoin-dev/phase166/plan11/data-delta.json`. The passing type/order/membership audit in `.local/open-bitcoin-dev/phase166/plan11/data-audit.json` proves all unlisted rows and original relative order, reserved process rows and eight approved CODE digests remain exact. Final source/digest/key-link/stub checks are in `.local/open-bitcoin-dev/phase166/plan11/final-source-selfcheck.json`.

## Automated Evidence

Bun uses the pinned 1.3.9 PATH and explicit `./` test-file paths. Guard/copy tests inherited the host TMPDIR alias; they exercise source fixtures, not product datadir authority. Final maintenance and reporting checks explicitly canonicalize TMPDIR. No Cargo/Bazel job or Rust runtime test ran in this plan.

| Check | Actual outcome | Full repository-relative receipt |
| --- | --- | --- |
| Untouched CLI baseline | 155 exit 0; 157/164/165 exit 1 at their actual stale contracts | `.local/open-bitcoin-dev/phase166/plan11/before-check-phase155-filter-index.log`, `.local/open-bitcoin-dev/phase166/plan11/before-check-phase157-index-catch-up.log`, `.local/open-bitcoin-dev/phase166/plan11/before-check-phase164-canonical-identity.log`, `.local/open-bitcoin-dev/phase166/plan11/before-check-phase165-shared-exact-work.log` |
| Initial inert-checker RED | Exit 1; 3 passed / 52 failed | `.local/open-bitcoin-dev/phase166/plan11/semantic-red.log` |
| Initial checker corrections | Both exit 1, 49/6; wrong wallet method/count assumptions, not product failures | `.local/open-bitcoin-dev/phase166/plan11/semantic-green-first.log`, `.local/open-bitcoin-dev/phase166/plan11/semantic-green-second.log` |
| Nonunique structural-mutant setup attempts | Exit 1; 62/1 and 63/1; excluded from behavioral RED claims | `.local/open-bitcoin-dev/phase166/plan11/semantic-code-ready.log`, `.local/open-bitcoin-dev/phase166/plan11/semantic-code-ready-v2.log` |
| Original catalog projection failure | CLI exit 1 with two new-helper attribution findings; tests 747/22 | `.local/open-bitcoin-dev/phase166/plan11/startup-cli.log`, `.local/open-bitcoin-dev/phase166/plan11/startup-tests.log` |
| Reviewed startup successors | Exit 0; 771/771 | `.local/open-bitcoin-dev/phase166/plan11/startup-tests-successor.log` |
| First independent-review predicate RED | Exit 1; 0/8, 68 filtered | `.local/open-bitcoin-dev/phase166/plan11/conditional-review-red.log` |
| First predicate repair | Exit 0; 76/76 | `.local/open-bitcoin-dev/phase166/plan11/conditional-review-green.log` |
| Ownership classifier controls included | Exit 0; 87/87 | `.local/open-bitcoin-dev/phase166/plan11/ownership-code-green.log` |
| Consolidated predicate-family RED | Exit 1; 0/10, 87 filtered | `.local/open-bitcoin-dev/phase166/plan11/predicate-family-red.log` |
| Whole-guard template setup failures | Exit 1; 91/6, 95/6 and 98/6; trailing-call syntax/actual constant corrections, not Rust failures | `.local/open-bitcoin-dev/phase166/plan11/predicate-family-green.log`, `.local/open-bitcoin-dev/phase166/plan11/predicate-audit-final.log`, `.local/open-bitcoin-dev/phase166/plan11/predicate-closure-final.log` |
| Final reviewed CODE guard suite | Exit 0; 106/106, 411 assertions | `.local/open-bitcoin-dev/phase166/plan11/code-systematic-refreeze.log` |
| Initial five-file attempt | Interrupted Bun status 130; partial observed 1276 pass / 22 fail markers; no completed-suite claim | `.local/open-bitcoin-dev/phase166/plan11/final-affected-tests.log`, `.local/open-bitcoin-dev/phase166/plan11/invalid-attempt-outcome.json` |
| Corrected four-file suite | Exit 1; 2175 passed / 52 reader-gate failures; 2227 tests, 5018 assertions, 938.94s | `.local/open-bitcoin-dev/phase166/plan11/final-corrected-tests.log`, `.local/open-bitcoin-dev/phase166/plan11/final-corrected-outcome.json` |
| Final explicit current CLIs | 155/157/164/166 exit 0; 165 exit 1 before source reads | `.local/open-bitcoin-dev/phase166/plan11/final-check-phase155-filter-index.log`, `.local/open-bitcoin-dev/phase166/plan11/final-check-phase157-index-catch-up.log`, `.local/open-bitcoin-dev/phase166/plan11/final-check-phase164-canonical-identity.log`, `.local/open-bitcoin-dev/phase166/plan11/final-check-phase165-shared-exact-work.log`, `.local/open-bitcoin-dev/phase166/plan11/final-check-phase166-canonical-bootstrap.log` |
| Final managed checks | Exit 0; zero findings | `.local/open-bitcoin-dev/phase166/plan11/managed-final.log` |
| Tracked whitespace and new-source whitespace | Tracked exit 0; explicit three-file newline/trailing-whitespace check passed | `.local/open-bitcoin-dev/phase166/plan11/diff-final.log`, `.local/open-bitcoin-dev/phase166/plan11/new-source-whitespace.json` |

The corrected run's primary per-file outcomes are **155: 161/0; 157: 771/0; 164: 1137/52; 166: 106/0**. All 52 error records are exactly `fixed input inventory changed before any source read`: 21 imported historical/current-reader cases and 31 isolated paired-adapter cases. Their downstream assertions were not reached and are not claimed passed. Bun repeats failed labels in its bottom summary; `final-corrected-outcome.json` excludes those duplicates.

### Initial Data-Placement Failure and Controlled Interruption

A scoped appender incorrectly searched for `];` when SOURCE_FILES ends `] as const;`, placing the five approved helper strings at TEST_OWNERS' tail. The exact array audit caught this before acceptance. The invalid generation and initial delta remain `.local/open-bitcoin-dev/phase166/plan11/data-placement-first.ts` and `.local/open-bitcoin-dev/phase166/plan11/data-delta-first.json`.

Root replanned the known-invalid attempt instead of requiring an expensive invalid-input sweep to finish. Exact Bun PID/parent/command and no descendants were verified before SIGINT; the parent reaped it before any data correction. This was termination for a proven setup defect, not elapsed time or quiet output. The receipt is `.local/open-bitcoin-dev/phase166/plan11/invalid-attempt-interruption.json`. Of the 22 partial failure markers, 21 were the existing Phase 165 reader transition and one was the unregistered moved daemon route. The five strings were then moved with an exact targeted patch. Array types, allowed deltas, membership, order and CODE equality passed before the corrected run.

## Deviations, Review and Remaining Handoff

Root checked three narrow ownership amendments: the previously no-op Phase 157 retry test, its two-path historical evidence projection, and the exact Phase 164 ownership classifier. Independent source/checker review required the finite predicate-family correction before data renewal. These are guard correctness fixes; no Rust product source, parser dependency or production surface was added here.

The simplification pass retained existing Rust helpers and the existing vector validator, used explicit finite guard data rather than a new Rust parser, and preserved all original historical owners. No broad skip list, provenance refresh loop, new production dependency, authentication gate or goal-blocking stub was introduced.

Plan 12 must earn the exact-five-helper current read-set pin transition, preserve the pre-read/regular-file/size/no-follow checks, rerun the complete Phase 165 and imported paired controls, review its final process rows, and perform exact default registration. This plan deliberately leaves `packages/open-bitcoin-rpc/tests/phase165_work_startup.rs` EDGES/SOURCE_CONTRACTS/TEST_OWNERS untouched. No native stage or deferred control is removed.

Plan 13 owns contributor/parity documentation; Plan 14/root owns whole-phase native, independent code/security/goal acceptance, generated LOC and final normal-hook Git saving. Ordinary descendants, Signet challenge qualification, public activation, wallet authority, actual pinned-Knots daemon differential and production/funds claims remain outside this plan.

Guidance applied: AGENTS/native verifier/tool pins/parity/LOC contracts, Bright Builds sidecar, overrides with no concrete exception, architecture/code-shape/verification/testing/language standards, and the execute-plan summary contract. Both active lesson files were read completely: 8,856 bytes / 2,953 conservative estimated tokens; no archives or project skills were loaded. The modeled current-source/evidence trust boundary covers the fixed bounded reader and mutation controls; no unmodeled endpoint, auth path or schema change was introduced.

## Task Commits

None. `commits: false` follows the strict wrapper; commit-existence checks are inapplicable. Shared lifecycle/state updates and all Git saving remain root-owned.

## Self-Check: PASSED

All ten implementation/evidence files, this summary and its 48 referenced source/receipt files exist. The eight approved CODE digests and two audited DATA digests match their accepted snapshots. Exact array types, approved deltas, original order and reserved process rows pass the recorded audit. The planned vector key link is present, the final 106-case guard suite and explicit 155/157/164/166 CLIs pass, and managed/whitespace checks pass. No goal-blocking stub remains. Frontmatter has exactly two delimiters, the expected lifecycle and `commits: false`; the checked Plan 12 reader dependency and full-verification ownership are explicitly recorded.
