---
phase: 164-canonical-chain-identity-and-compatibility-gates
plan: "07"
subsystem: documentation
tags: [canonical-identity, parity, compatibility-refusal, operator-uat]
requires:
  - phase: 164-01
    provides: Exact four-network source-derived identity vectors and immutable support facts
  - phase: 164-02
    provides: Non-opening filesystem classification and physical source preservation
  - phase: 164-03
    provides: Immutable ordinary admission refusal
  - phase: 164-04
    provides: Ordinary daemon/metrics gating and selected-network mapping
  - phase: 164-05
    provides: Ordinary RPC context guards and explicit synthetic fixture APIs
  - phase: 164-06
    provides: Offline CLI and soak refusal plus separate authenticated live paths
  - phase: 164-13
    provides: Actual coupled GREEN and preserved historical lifecycle regression
provides:
  - Source-pinned four-network identity and current refusal companion
  - Pending current-source parity surface and intentional ordinary-history difference
  - Current README/support navigation and safe repo-local Cargo/Bazel UAT
affects: [164-08, 165, 166, 167, 168]
tech-stack:
  added: []
  patterns: [Selected identity separate from unavailable stored observations, historical evidence preservation]
key-files:
  created:
    - docs/parity/v2-6-canonical-chain-identity.md
  modified:
    - README.md
    - docs/parity/index.json
    - docs/parity/support-matrix.md
key-decisions:
  - Current parity owner remains in_progress pending full phase verification; no requirements are completed here.
  - Retain original historical inventory entries and receipt text while separately explaining the current ordinary refusal boundary.
  - UAT uses exact existing testnet parser spelling and owned physical temporary roots without public activation.
requirements-completed: []
requirements-addressed: [CBOT-01, CBOT-04, CVLD-05]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 164-2026-10-14T01-59-33Z
generated_at: "2026-10-14T05:20:16Z"
duration: approximately 10min
completed: 2026-10-14
---

# Phase 164 Plan 07: Canonical Identity and Refusal Documentation Summary

**Exact pinned four-network genesis identities, non-opening ordinary refusal and safe repo-local UAT are documented without upgrading historical synthetic evidence to canonical acceptance.**

## Performance and Accomplishments

- Tasks completed: 2/2; four documentation/catalog files plus this summary.
- Approximate duration: 10 minutes including source/evidence reading and scoped checks.
- Added the source-pinned companion with all four exact hashes, header fields, common genesis transaction facts, default Signet challenge, testnet/testnet3 distinction and identity-only evidence class.
- Documented all current missing prerequisites and later ownership: exact work 165, canonical durable bootstrap/reopen 166, candidate-history validation 167 and actual Knots differential qualification 168.
- Added one new top-level surface and one checklist owner with `in_progress` status plus the intentional-difference deviation. Original inventory entries are unchanged.
- Added README/support-matrix navigation and four identity/refusal preview rows plus deferred ordinary-history row using the existing support vocabulary. Current boundary explicitly constrains older historical runtime classifications.
- Documented source-preserving guidance, unavailable observed network/root/work, offline wallet/control refusal, bounded soak unavailability before ledger/index writes, and separately authenticated live RPC ownership.
- Kept historical v2.5 receipts, source IDs, digests, dates, counts, maturity/work/synthetic-root limits and halving difference attributable and unchanged. README adds context before the old exact acceptance sentence rather than rewriting it.

## Actual Verification

| Check | Actual result |
| --- | --- |
| Root, `status`, `sync` and `soak start` help | Executed `packages/target/debug/open-bitcoin` with the corresponding `--help`; existing flags and `mainnet/testnet/signet/regtest` choices agree with documentation. |
| Actual offline JSON refusal | Bun spawned the already-built binary for four networks × missing/empty sources × sync status/pause/resume: 24/24 nonzero refusals, parsed JSON errors, fixed `chain_compatibility_refused` and `unsupported_validation_scope`, selected canonical identity, shape observation, ordinary_history=false and Signet prerequisite. |
| Temporary-root preservation | Empty source remained empty; missing source remained absent after all 24 attempts. Canonicalized owned temporary parent was removed afterward; no user datadir or public networking was used. |
| Plan task doc/UAT checks | Required network names, Cargo/Bazel command forms, scope/prerequisite/observation facts and README/support navigation passed. |
| Catalog ownership and references | Exactly one pending checklist owner, exact three requirement IDs, and every listed evidence/upstream file exists. |
| Historical index equality | Removed only the new surface/checklist owner/deviation from the parsed current index and deep-compared to HEAD: all original values unchanged. |
| Exact source-derived hashes | All four documented hashes match pinned chainparams.cpp hash assertions; submodule HEAD is exactly a9aee730466ac67d35a3c03ee24676be5e045878. |
| Local companion links | Eight existing local target links passed before summary creation; summary target verified in final self-check. |
| Historical artifacts | `git diff --exit-code` passed for the two v2.5 companion pages and historical Phase 162/163 artifact directories: no changes. |
| Managed checks | `bun scripts/bright-builds-check.ts all` passed: 1,494 files scanned, no file-length or lesson findings. |
| Whitespace/diff review | Scoped `git diff --check` passed; reviewed complete owned additions and catalog placement for unintended side effects. |

The exact Cargo/Bazel UAT commands are reproduction instructions with expected
outcomes; this executor did not run them or a Cargo/Bazel job. Direct-binary help
and refusal checks are the separate actual execution evidence above. Full
default native verification, independent review/security and normal-hook Git
finalization remain parent-owned. No actual Knots daemon execution, canonical
bootstrap/reopen, whole-chain/full-node or production/funds claim follows.

## Existing Historical Checker Reconciliation

`bun scripts/check-phase162-integrated-filter-proof.ts` exited 1 with five
reported items:

1. T-162-10 source hash for `bin/open_bitcoind/tests/filter_index/phase162/fixtures.rs`.
2. Required ordered `fixture` edge in `inbound_listener/tests/peer_filters/phase162.rs`.
3. Required ordered `open_policy` edge in daemon `tests/filter_index/phase162/fixtures.rs`.
4. `docs/parity/index.json` unsupported T-162-10 claim.
5. `docs/parity/v2-5-integrated-filter-proof.md` unsupported T-162-10 claim.

These existing current-source bindings reference constructor spans changed by
the separately planned explicit fixture migration. Plan 08/parent owns narrow
reconciliation and its tests. The failure was reported immediately; no old
receipt, hash, assertion, checker or historical document was rewritten here.
This result is not presented as a passing historical checker or full-phase gate.

## Decisions, Simplification and Scope Review

Followed D-01–08 and the plan's threat mitigations T-164-21/T-164-25. Kept one
companion as the exact identity/diagnostic/UAT reference, with short navigation
and support rows elsewhere. Used existing catalog schema and support terms
rather than adding a schema, evidence class API or alternate support vocabulary.
Genesis identity and stored observations remain distinct; unavailable identity
is never an invented wrong-network finding.

Loaded local AGENTS guidance, Bright Builds sidecar, overrides, managed index,
architecture/verification/testing/TypeScript guidance and both complete active
lesson inputs. Checked formatter guidance/configuration before edits; no
first-party Markdown formatter contract/configuration was found, so no bare
mdformat run, plugin installation or broad normalization was performed. Existing
Markdown/table conventions and JSON indentation were preserved. No dependencies,
production source, new endpoint/auth/schema/file-access trust surface or
goal-blocking stub was added. Stub-pattern scan of the touched Markdown found
no placeholders. No authentication gate occurred.

## Deviations and Remaining Ownership

None within Plan 07. Parent explicitly authorized actual help/parser validation
through the already-built binary to avoid an unnecessary Cargo job. The known
historical-checker adaptation is a planned Plan 08 responsibility, not an
undisclosed documentation pass or a new repair made here.

Tasks 1 and 2 commits are deferred to strict parent phase finalization. No
staging, commits, pushes, hook skip, worktree operation or shared
STATE/ROADMAP/REQUIREMENTS/todo/PROJECT edit was performed. No hashes or
requirement-completion flags are fabricated. Parent owns phase verification,
shared bookkeeping and Git finalization.

## Self-Check: PASSED

All four plan-owned documentation/catalog files and this summary exist. Exact
source pin/hash checks, actual 24 refusal checks, catalog/reference/history
equality, managed checks and scoped whitespace review substantiate the claims.
Frontmatter uses only the two leading delimiters. No commit check is applicable
under parent-owned Git finalization. The existing five-item historical checker
failure remains explicitly disclosed for Plan 08/parent reconciliation.
