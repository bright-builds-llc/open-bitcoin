---
phase: 85-operator-runbooks
verified_at: 2026-06-22T16:31:11Z
status: passed
requirements: [RUN-01, RUN-02, RUN-03]
generated_by: gsd-verifier
generated_at: 2026-06-22T16:31:11Z
lifecycle_mode: yolo
phase_lifecycle_id: 85-2026-06-22T11-57-13
lifecycle_validated: true
---

# Phase 85 Verification Report

**Phase Goal:** Operators have a single parity-rooted runbook for v1.8 preflight,
long-run monitoring, no-progress diagnosis, recovery decisions, and support
bundle collection while the default verifier stays deterministic and local.

## Status

Passed. Phase 85 delivered the canonical operator runbook, parity ledger roots,
entrypoint links, deterministic checker coverage, generated LOC freshness, and
full repo-native verification evidence.

## Requirements Coverage

| Requirement | Status | Evidence |
| --- | --- | --- |
| RUN-01 | SATISFIED | `docs/parity/operator-runbooks.md` documents the preflight boundary, required evidence, repo-local Cargo/Bazel status commands, support terminology, and explicit non-claims. |
| RUN-02 | SATISFIED | The runbook covers structured logs, metrics, support-bundle summaries, soak reports, live-smoke reports, checkpoint timeline, stalled subsystem diagnosis, public-network opt-in, stay-current opt-in, multi-day soak opt-in, and proof/non-proof language. |
| RUN-03 | SATISFIED | `docs/parity/index.json`, `docs/parity/checklist.md`, parity docs, README entrypoints, runtime guide, and the operator runtime catalog all reference the Phase 85 runbook root and requirements. |

## Key Artifacts

| Artifact | Status | Details |
| --- | --- | --- |
| `docs/parity/operator-runbooks.md` | VERIFIED | Canonical Phase 85 runbook with `v1-8-operator-runbooks` surface id and support-bundle/recovery guidance. |
| `scripts/check-phase85-operator-runbooks.ts` | VERIFIED | Deterministic checker for runbook content, parity roots, entrypoint links, pointer-doc duplication, verifier order, and unsafe default-verifier commands. |
| `scripts/check-phase85-operator-runbooks.test.ts` | VERIFIED | Nine fixture tests cover required content and drift failures. |
| `scripts/verify.sh` | VERIFIED | Runs Phase 85 checker tests and checker after Phase 84. |
| `docs/metrics/lines-of-code.md` | VERIFIED | Regenerated with entries for both Phase 85 checker files. |

## Verification Commands

| Command | Result |
| --- | --- |
| `bun run scripts/generate-loc-report.ts --source=worktree --output=docs/metrics/lines-of-code.md --check` | PASS |
| `bun test scripts/check-phase85-operator-runbooks.test.ts` | PASS, 9 tests |
| `bun --check scripts/check-phase85-operator-runbooks.ts` | PASS |
| `bun run scripts/check-phase85-operator-runbooks.ts` | PASS |
| `rg -n "check-phase85-operator-runbooks" docs/metrics/lines-of-code.md` | PASS |
| `git diff --check -- docs/metrics/lines-of-code.md scripts/check-phase85-operator-runbooks.ts scripts/check-phase85-operator-runbooks.test.ts scripts/verify.sh` | PASS |
| `bash scripts/verify.sh` | PASS, completed in 1h 11m 36.168s |

## Default Verification Boundary

The default `bash scripts/verify.sh` path remained deterministic and local. It
does not opt into public-network sync, real service-manager operations, or
multi-day soak execution. Those workflows remain explicit operator UAT surfaces
documented by the runbook.

## Full Repo-Native Verification

Full repo-native verification passed on 2026-06-22. The verifier completed hook
setup, generated LOC freshness, parity and release-boundary checkers through
Phase 85, pure-core dependency/import guards, production Rust file-length and
panic-site checks, Cargo clippy/build/test, benchmark smoke report validation,
Bazel smoke build, and coverage tests.

## Residual Risks

- Public-network live smoke, stay-current review, and multi-day soak remain explicit opt-in operator workflows outside default verification.
- Real service-manager behavior remains outside deterministic default verification.
- Destructive repair, migration apply, automatic support-bundle upload, and broad claim scanning remain deferred to their owning future phases.

---

_Verified: 2026-06-22T16:31:11Z_
_Verifier: the agent (gsd-verifier)_
