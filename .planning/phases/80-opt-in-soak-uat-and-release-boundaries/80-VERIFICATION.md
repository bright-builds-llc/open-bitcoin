---
phase: 80-opt-in-soak-uat-and-release-boundaries
verified: 2026-06-18T06:09:36Z
verified_at: 2026-06-18T06:09:36Z
status: passed
score: "8/8 must-haves verified"
requirements: [VER-05, VER-06, VER-07, REL-04]
generated_by: gsd-verifier
lifecycle_mode: yolo
phase_lifecycle_id: 80-2026-06-17T22-54-57
generated_at: 2026-06-18T06:09:36Z
lifecycle_validated: true
overrides_applied: 0
---

# Phase 80: Opt-In Soak UAT and Release Boundaries Verification Report

**Phase Goal:** Contributors keep default verification deterministic while operators get copy-pasteable long-run UAT commands and reviewers can audit the scoped v1.7 claim.
**Verified:** 2026-06-18T06:09:36Z
**Status:** passed
**Re-verification:** No - prior verification had no `gaps:` section, so this was verified from the roadmap and actual files.

## Goal Achievement

### Observable Truths

| # | Truth | Status | Evidence |
|---|---|---|---|
| 1 | `bash scripts/verify.sh` runs without internet access, public peers, real service managers, multi-day sleeps, current-tip timing, or large disk consumption. | VERIFIED | `scripts/verify.sh` runs local Bun/Rust/Bazel checks only; forbidden-string scan found no live-mainnet, manual-peer, service-manager, process-table, current-tip, multi-day sleep, or large-disk gate strings. Full `bash scripts/verify.sh` passed in 37m 8.046s. |
| 2 | Operator docs provide repo-local Cargo and Bazel commands for opt-in multi-day soak, bounded recovery drills, support-bundle generation, and post-failure diagnosis. | VERIFIED | `docs/operator/runtime-guide.md` contains `Phase 80 v1.7 opt-in soak UAT matrix` with exactly 4 workflow rows and both command forms. |
| 3 | Parity breadcrumbs, fixtures, support-bundle schemas, deterministic checkers, and operator docs cover every new v1.7 source, test, and evidence surface. | VERIFIED | `bun run scripts/check-parity-breadcrumbs.ts --check` verified 268 Rust files; Phase 80 checker requires support/soak source anchors, parity roots, fixture tests, and operator docs. |
| 4 | v1.7 docs and status surfaces describe only explicit opt-in soak and recovery hardening, not broad production-node readiness. | VERIFIED | README and parity roots describe `explicit opt-in full-sync soak and recovery hardening` and preserve non-claims for inbound serving, relay, production wallet use, migration apply, packaging, GUI, hosted dashboards, public-network CI, release-blocking live sync, automatic upload, destructive repair, and broad production-node readiness. |
| 5 | Phase 80 UAT matrix exists and has exactly four workflows with proof/non-proof boundaries. | VERIFIED | Section row count returned `4`; rows are multi-day soak lifecycle, bounded recovery drill, support-bundle generation, and post-failure diagnosis, with `Evidence proves` and `Does not prove` columns. |
| 6 | Existing parity roots include the v1.7 closeout surface and VER-05, VER-06, VER-07, REL-04 traceability without a new evidence manifest. | VERIFIED | `docs/parity/index.json` has exactly one top-level and one checklist `v1-7-full-sync-soak-recovery-release-boundaries` surface, status `done`, all four requirements, 17 evidence paths, and no `docs/parity/*manifest*v1.7*` or `*evidence*v1.7*.json` file exists. |
| 7 | Phase 80 checker and tests guard UAT anchors, parity roots, forbidden manifests, support/soak source anchors, verifier exclusions, and Phase 75-80 ordering. | VERIFIED | `bun test scripts/check-phase80-opt-in-soak-uat-release-boundaries.test.ts` passed 6 tests and 13 assertions; checker passed against the real worktree. |
| 8 | Clean review warning is fixed: removing either or both Phase 80 verifier commands fails the checker. | VERIFIED | Checker iterates every `REQUIRED_VERIFY_ORDER` command with `requireContains`; test `fails_when_phase80_verify_commands_are_missing` covers both removed together, and code-level guard covers either command removed individually. |

**Score:** 8/8 truths verified

### Required Artifacts

| Artifact | Expected | Status | Details |
|---|---|---|---|
| `docs/operator/runtime-guide.md` | Focused v1.7 opt-in soak UAT matrix | VERIFIED | Exists, substantive, checker-guarded; Phase 80 section has 4 workflow rows and Cargo/Bazel commands. |
| `README.md` | Contributor-facing v1.7 operator/release posture | VERIFIED | Names v1.7 boundary and deferred production-adjacent non-claims. |
| `docs/parity/release-readiness.md` | v1.7 claim-boundary matrix and traceability | VERIFIED | Contains current v1.7 matrix and all 24 v1.7 requirement IDs, including VER-05, VER-06, VER-07, REL-04. |
| `docs/parity/index.json` | Machine-readable v1.7 closeout root | VERIFIED | JSON parses; one `v1-7-full-sync-soak-recovery-release-boundaries` surface and one checklist surface are present with status `done`. |
| `docs/parity/checklist.md` | Human-readable v1.7 closeout checklist row | VERIFIED | Contains closeout row, evidence paths, non-claims, and requirements. |
| `docs/parity/README.md` | Parity entrypoint for current v1.7 closeout | VERIFIED | Points to Phase 80 root and says no new evidence manifest was added. |
| `docs/parity/deviations-and-unknowns.md` | Deferred production-adjacent scope register | VERIFIED | Contains v1.7 deferred scope and non-claim list. |
| `docs/parity/catalog/operator-runtime-release-hardening.md` | Operator-runtime Phase 80 closeout row | VERIFIED | Contains Phase 80 row referencing existing roots and checkers. |
| `scripts/check-phase80-opt-in-soak-uat-release-boundaries.ts` | Deterministic Phase 80 checker | VERIFIED | Exists, substantive, passes against real worktree, and is wired in `scripts/verify.sh`. |
| `scripts/check-phase80-opt-in-soak-uat-release-boundaries.test.ts` | Fixture-root checker regression tests | VERIFIED | 6 passing tests cover positive fixture, missing UAT anchors, missing parity requirements, verifier drift, missing Phase 80 commands, broad claims, and forbidden manifest. |
| `scripts/verify.sh` | Repo-native verifier wiring after Phase 79 | VERIFIED | Runs Phase 80 test and checker immediately after Phase 79 checker. |

### Key Link Verification

| From | To | Via | Status | Details |
|---|---|---|---|---|
| `docs/operator/runtime-guide.md` | `packages/open-bitcoin-cli` | Repo-local Cargo commands | VERIFIED | GSD key-link verifier found required Cargo command pattern. |
| `docs/operator/runtime-guide.md` | `//packages/open-bitcoin-cli:open_bitcoin` | Repo-local Bazel commands | VERIFIED | GSD key-link verifier found required Bazel command pattern. |
| `docs/parity/index.json` | `docs/parity/release-readiness.md` | Evidence path | VERIFIED | GSD key-link verifier found path. |
| `docs/parity/index.json` | `scripts/verify.sh` | Deterministic verification evidence path | VERIFIED | GSD key-link verifier found path. |
| `scripts/verify.sh` | `scripts/check-phase80-opt-in-soak-uat-release-boundaries.ts` | Bun deterministic checker | VERIFIED | GSD key-link verifier found command. |
| `scripts/check-phase80-opt-in-soak-uat-release-boundaries.ts` | `docs/parity/index.json` | Structured parity root parse | VERIFIED | Checker parses JSON and validates closeout surfaces. |

### Data-Flow Trace (Level 4)

| Artifact | Data Variable | Source | Produces Real Data | Status |
|---|---|---|---|---|
| `scripts/check-phase80-opt-in-soak-uat-release-boundaries.ts` | Local file text and parsed parity index | `readText`, `JSON.parse`, `scripts/verify.sh`, docs, Rust source anchors | Yes - real worktree files are read and validated; checker also supports fixture roots through `OPEN_BITCOIN_PHASE80_REPO_ROOT`. | VERIFIED |
| `docs/operator/runtime-guide.md` | UAT matrix rows | Static operator documentation | Not dynamic; checker verifies exact section, row count, command forms, proof boundaries. | VERIFIED |
| `docs/parity/index.json` | v1.7 closeout surface | JSON parity root | Yes - parsed and structurally validated for one surface, requirements, status, audit metadata, and evidence paths. | VERIFIED |

### Behavioral Spot-Checks

| Behavior | Command | Result | Status |
|---|---|---|---|
| Phase 80 fixture tests pass | `bun test scripts/check-phase80-opt-in-soak-uat-release-boundaries.test.ts` | 6 pass, 0 fail, 13 assertions | PASS |
| Phase 80 checker passes real tree | `bun run scripts/check-phase80-opt-in-soak-uat-release-boundaries.ts` | `validated Phase 80 opt-in soak UAT and release boundaries` | PASS |
| Source breadcrumb audit passes | `bun run scripts/check-parity-breadcrumbs.ts --check` | `Parity breadcrumbs verified for 268 Rust file(s).` | PASS |
| Default verifier has no forbidden live/default gates | `rg -n "run-live-mainnet-smoke|--manual-peer|--restart-after-progress|systemctl|launchctl|openbitcoinsync=mainnet-ibd|sleep 86400|sleep 259200|release-blocking live sync|lsof|/proc|fallocate|mkfile|dd if=|107374182400" scripts/verify.sh` | No matches | PASS |
| Full repo-native verification passes | `bash scripts/verify.sh` | Completed successfully in 37m 8.046s | PASS |

### Requirements Coverage

| Requirement | Source Plan | Description | Status | Evidence |
|---|---|---|---|---|
| VER-05 | 80-03 | Contributor can run `bash scripts/verify.sh` without internet access, public peers, real service managers, multi-day sleeps, current-tip timing, or large disk consumption. | SATISFIED | `scripts/verify.sh` ordering and forbidden-string checks pass; full verifier passed. |
| VER-06 | 80-01, 80-03 | Operator can run copy-pasteable repo-local Cargo and Bazel commands for opt-in multi-day soak, bounded recovery drills, support-bundle generation, and post-failure diagnosis. | SATISFIED | Runtime guide matrix has exactly four workflows with Cargo and Bazel command forms. |
| VER-07 | 80-02, 80-03 | Contributor can audit parity breadcrumbs, fixtures, support bundle schemas, deterministic checkers, and operator docs for every new v1.7 source, test, and evidence surface. | SATISFIED | Parity root, checker, checker tests, support/soak source anchors, and breadcrumb check all pass. |
| REL-04 | 80-01, 80-02, 80-03 | Contributor can verify v1.7 docs and status surfaces describe only explicit opt-in soak and recovery hardening, not broad production-node readiness. | SATISFIED | README and parity docs preserve scoped v1.7 claim and explicit deferred non-claims. |

No orphaned Phase 80 requirements were found in `.planning/REQUIREMENTS.md`; VER-05, VER-06, VER-07, and REL-04 are all mapped to Phase 80 and claimed by plan frontmatter.

### Anti-Patterns Found

| File | Line | Pattern | Severity | Impact |
|---|---:|---|---|---|
| `scripts/check-phase80-opt-in-soak-uat-release-boundaries.ts` | 96, 102-103 | Placeholder strings | Info | These are forbidden-string guard constants, not placeholders. |
| `scripts/check-phase80-opt-in-soak-uat-release-boundaries.ts` | 413, 442, 452 | `return null` | Info | Parser/control-flow sentinel branches for invalid parity JSON or missing surfaces; not stubs. |
| `scripts/check-phase80-opt-in-soak-uat-release-boundaries.ts` | 532 | `console.log` | Info | CLI success output after all failures have been checked; not a log-only implementation. |
| `docs/parity/catalog/operator-runtime-release-hardening.md` | 57 | "not as placeholders" | Info | Historical docs wording that rejects placeholder docs; not a stub. |

### Human Verification Required

None for Phase 80 goal closure. The multi-day public-network soak workflows are intentionally opt-in UAT examples and are not required default-verification gates.

### Gaps Summary

No gaps found. Phase 80 achieved the goal: default verification remains deterministic, operator UAT commands are documented with proof boundaries, v1.7 parity/release roots are auditable without a new manifest, and the checker/test wiring guards the prior code-review warning.

---

_Verified: 2026-06-18T06:09:36Z_
_Verifier: the agent (gsd-verifier)_
