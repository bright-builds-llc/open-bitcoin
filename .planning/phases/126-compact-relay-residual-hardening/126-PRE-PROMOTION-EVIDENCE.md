---
phase: 126-compact-relay-residual-hardening
artifact: pre-promotion-evidence
status: candidate_passed
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 126-2026-07-18T16-09-20
generated_at: 2026-07-18T20:39:52Z
requirements-completed: []
---

# Phase 126 Pre-Promotion Evidence

## Provenance Boundary

This executor-authored artifact records command evidence for the Phase 126
candidate projection. It is input to—not a substitute for—the independent
`gsd-verifier` revision gate. It does not claim independent verification,
does not own `126-VERIFICATION.md`, and does not authorize requirement or
milestone promotion.

## Candidate Result

Status: **candidate_passed**.

The final candidate corpus passed after the repository-owned LOC generator
refreshed the tracked report and one no-claim wording bug was corrected. The
candidate remains deliberately unpromoted:

| Projection | Candidate state |
| --- | --- |
| Phase 126 requirements | `CMP-05`, `RCN-02`, `RCN-03`, `GOV-04`, `BOUND-01`, and `HARD-05` unchecked/Pending |
| Requirement score | `33/39` |
| Phase score | `16/17` |
| Audit status | `gaps_found` |
| Primary route | `/gsd-execute-phase 126` |

## Final Successful Command Corpus

| Command | Exit | Exact result |
| --- | ---: | --- |
| `bun test scripts/check-phase126-compact-relay-residual-hardening.test.ts` | 0 | `13 pass`, `0 fail`, `16 expect() calls`; 13 tests across 2 files |
| `bun run scripts/check-phase126-compact-relay-residual-hardening.ts` | 0 | `Phase 126 compact relay residual hardening validated.` |
| `bun run scripts/check-phase124-milestone-closeout-reconciliation.ts` | 0 | `Phase 124 milestone closeout reconciliation checker passed.` |
| `bun run scripts/check-active-milestone-verification-traceability.ts` | 0 | `Active milestone verification traceability checker passed.` |
| `bun run scripts/check-parity-breadcrumbs.ts --check` | 0 | `Parity breadcrumbs verified for 383 Rust file(s).` |
| `node "$HOME/.codex/get-shit-done/bin/gsd-tools.cjs" verify lifecycle 126 --require-plans --raw` | 0 | `valid` |
| `bun test scripts/check-phase103-mempool-lifecycle.test.ts` | 0 | `8 pass`, `0 fail`, `14 expect() calls`; 8 tests across 2 files |
| `bun run scripts/check-phase103-mempool-lifecycle.ts` | 0 | `Phase 103 mempool lifecycle evidence validated.` |
| `bash scripts/verify.sh` | 0 | `verify.sh completed in 3m 19.503s (199503ms)` |

The successful full verifier included LOC freshness, parity breadcrumbs, all
deterministic phase and release-boundary checker suites, pure-core dependency
and panic-site checks, file-length policy, Cargo formatting, Clippy with
warnings denied, all-target/all-feature build and tests, benchmark smoke,
Bazel smoke/provenance, and pure-core coverage.

## Rerun And Repair Audit Trail

The first full-verifier attempt exited 1 after 240ms with:

```text
error: stale LOC report: docs/metrics/lines-of-code.md
run: bun run scripts/generate-loc-report.ts --source=worktree --output=docs/metrics/lines-of-code.md
```

The report was refreshed only through the prescribed repository generator:

| Command | Exit | Exact result |
| --- | ---: | --- |
| `bun run scripts/generate-loc-report.ts --source=worktree --output=docs/metrics/lines-of-code.md` | 0 | `Wrote docs/metrics/lines-of-code.md (236,187 lines counted).` |

The next full-verifier attempt exited 1 after 38.753s because
`docs/parity/catalog/mempool-policy.md:222` used `adds no package relay`,
which the Phase 103 release-boundary parser did not recognize as established
no-claim grammar. The wording was corrected to `does not add package relay`
in commit `74f826e3`. The focused Phase 103 suite and live checker then passed,
followed by the successful full verifier recorded above.

## Independent Revision Gate

The execute-phase orchestrator must now spawn a fresh `gsd-verifier`. Only that
verifier may create `126-VERIFICATION.md` with `generated_by: gsd-verifier`,
this exact lifecycle identity, `lifecycle_validated: true`, and a truthful
status. Plan 126-04 remains blocked until the verifier reports `passed` and:

```text
node "$HOME/.codex/get-shit-done/bin/gsd-tools.cjs" verify lifecycle 126 --require-plans --require-verification --raw
```

returns `valid`.
