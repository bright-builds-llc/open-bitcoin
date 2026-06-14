---
phase: 73-opt-in-uat-and-deterministic-verification
verified_at: 2026-06-14T08:01:06Z
verified: 2026-06-14T08:01:06Z
status: passed
score: 4/4 must-haves verified
generated_by: gsd-verifier
lifecycle_mode: yolo
phase_lifecycle_id: 73-2026-06-13T22-08-43
generated_at: 2026-06-14T08:01:06Z
lifecycle_validated: true
overrides_applied: 0
---

# Phase 73: Opt-In UAT and Deterministic Verification

**Phase Goal:** Contributors keep default verification deterministic while operators get repo-local opt-in commands for public-mainnet full-sync review.
**Status:** passed
**Score:** 4/4 must-haves verified
**Re-verification:** No - initial verification

## Goal Achievement

Phase 73 achieved its goal. The live repository keeps `bash scripts/verify.sh`
deterministic and local, while the operator guide now contains explicit opt-in
public-mainnet UAT commands and proof/non-proof semantics. The parity roots and
Phase 73 checker make the evidence trail auditable without claiming
production-node, relay, wallet-safety, migration-apply, packaging, GUI, hosted
dashboard, public-network CI, or release-blocking live-sync scope.

## Observable Truths

| # | Truth | Status | Evidence |
|---|---|---|---|
| 1 | `bash scripts/verify.sh` runs without internet access, public peers, real service managers, long-running sync, or current-tip timing. | VERIFIED | `scripts/verify.sh` runs Phase 72, then `bun test scripts/check-phase73-uat-verification.test.ts`, then `env -u OPEN_BITCOIN_PHASE73_REPO_ROOT bun run scripts/check-phase73-uat-verification.ts`; forbidden live-mainnet/manual-peer/service-manager/mainnet-IBD/timing strings are absent from the verifier. |
| 2 | Deterministic tests cover durable UTXO/undo writes, block connect/disconnect/reorg across restart, best-chain header selection, peer response failures, crash recovery, duplicate connect prevention, and resource bounds. | VERIFIED | `scripts/check-phase73-uat-verification.ts` defines `VER02_COVERAGE` with all seven behavior keys and anchors them to existing hermetic Rust tests in chainstate, snapshot codec, and node sync test files. |
| 3 | Operator docs provide copy-pasteable repo-local Cargo and Bazel commands for opt-in public-mainnet full-sync, stay-current, restart/resume, and support-bundle UAT. | VERIFIED | `docs/operator/runtime-guide.md` contains `### Phase 73 opt-in public-mainnet UAT matrix` with six workflows, Cargo/Bazel forms, live-smoke command forms, and explicit proof/non-proof wording. |
| 4 | Parity breadcrumbs, fixtures, compatibility harness reports, and deterministic checkers cover every new v1.6 source, test, and operator-evidence surface. | VERIFIED | Parity docs and ledgers contain `phase73-opt-in-uat-deterministic-verification`, list VER-01 through VER-04, reference breadcrumbs, live-smoke fixture/runner, compatibility harness, support evidence, and the Phase 73 checker. |

## Required Artifacts

| Artifact | Status | Details |
|---|---|---|
| `scripts/check-phase73-uat-verification.ts` | VERIFIED | Substantive checker with requirements, coverage-map, UAT-doc, verifier-order, parity-ledger, breadcrumb, and deferred-scope checks. |
| `scripts/check-phase73-uat-verification.test.ts` | VERIFIED | 11 Bun regression tests cover fixture success and failure cases for verifier hardening, requirement drift, UAT matrix drift, coverage-anchor drift, parity root drift, breadcrumb drift, and overclaim drift. |
| `scripts/verify.sh` | VERIFIED | Wires Phase 73 regression tests and hardened checker after Phase 72; full verifier completed successfully during verification. |
| `docs/operator/runtime-guide.md` | VERIFIED | Contains central Phase 73 opt-in public-mainnet UAT matrix and proof semantics. |
| `docs/parity/index.json`, `docs/parity/checklist.md`, `docs/parity/README.md` | VERIFIED | Phase 73 root exists, maps to VER-01 through VER-04, and keeps public-network UAT/default-verification boundaries explicit. |
| `docs/parity/catalog/p2p.md`, `docs/parity/catalog/chainstate.md`, `docs/parity/catalog/operator-runtime-release-hardening.md` | VERIFIED | Catalogs document opt-in UAT boundary, VER-02 coverage map, local evidence roots, and deferred-scope non-claims. |

## Key Link Verification

| Link | Status | Evidence |
|---|---|---|
| `scripts/verify.sh` -> Phase 73 checker | VERIFIED | Manual check confirmed Phase 73 test/checker ordering after Phase 72 and absence of forbidden default-verifier strings. |
| Phase 73 checker -> VER-02 Rust anchors | VERIFIED | GSD key-link checks passed for node sync and chainstate anchors; direct grep found representative tests such as `connect_disconnect_and_reorg_preserve_phase_four_outcomes`, `connected_active_chain_progress_survives_runtime_reopen`, `phase71_synthetic_long_chain_exercises_resource_bounds_without_public_network`, and `same_datadir_reopen_does_not_duplicate_connected_block_getdata`. |
| Runtime guide -> operator CLI/live-smoke surfaces | VERIFIED | GSD key-link checks passed for operator command grammar and `scripts/run-live-mainnet-smoke.ts`. |
| Phase 73 checker -> parity/breadcrumb roots | VERIFIED | GSD key-link checks passed for `docs/parity/source-breadcrumbs.json` and operator runtime catalog anchors. |

## Verification Commands

| Command | Result |
|---|---|
| `bash -n scripts/verify.sh` | PASS |
| `bun --check scripts/check-phase73-uat-verification.ts` | PASS |
| `bun test scripts/check-phase73-uat-verification.test.ts` | PASS, 11 tests |
| `env -u OPEN_BITCOIN_PHASE73_REPO_ROOT bun run scripts/check-phase73-uat-verification.ts` | PASS |
| `bun run scripts/check-parity-breadcrumbs.ts --check` | PASS, 240 Rust files verified |
| `bun -e 'JSON.parse(await Bun.file("docs/parity/index.json").text())'` | PASS |
| `bun run scripts/generate-loc-report.ts --source=worktree --output=docs/metrics/lines-of-code.md --check` | PASS |
| `bash scripts/verify.sh` | PASS, completed in 16m 11.094s |

## Requirements Coverage

| Requirement | Status | Evidence |
|---|---|---|
| VER-01 | SATISFIED | Default verifier is deterministic by construction and explicitly excludes live public-network/service-manager/mainnet-IBD/timing gates. |
| VER-02 | SATISFIED | Phase 73 coverage map and regression tests enforce all required deterministic coverage anchors. |
| VER-03 | SATISFIED | Operator guide matrix provides repo-local Cargo/Bazel/Bun opt-in UAT commands and states what each workflow proves and does not prove. |
| VER-04 | SATISFIED | Parity ledgers, catalogs, source breadcrumbs, live-smoke fixture/runner, compatibility harness reference, and checker enforcement are auditable. |

## Anti-Patterns

No blocking anti-patterns found. Stub scans found only non-blocking matches:
prose reference to placeholders in historical docs, the checker success
`console.log`, and local accumulator/default arrays used by the checker tests.

## Residual Risks

- Public-mainnet full-sync UAT remains opt-in operator evidence and was not run as part of default verification.
- Phase 74 still owns final release-boundary and documentation closeout for v1.6.
- Phase 73 intentionally does not make out-of-scope production/public-network claims.

## Gaps Summary

No blockers found. No structured gaps are required.

---

_Verified: 2026-06-14T08:01:06Z_
_Verifier: the agent (gsd-verifier)_
