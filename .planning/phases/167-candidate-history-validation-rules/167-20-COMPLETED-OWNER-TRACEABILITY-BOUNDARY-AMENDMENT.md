---
phase: 167-candidate-history-validation-rules
plan: "20"
type: checked-execution-amendment
generated_by: gsd-execute-plan
generated_at: "2026-10-31T12:20:32Z"
lifecycle_mode: yolo
phase_lifecycle_id: 167-2026-10-21T16-38-23
commits: false
---

# Plan 20 completed owner verification traceability

The fifth complete default verifier attempt passed the checked current guard chain through Phase165, Phase166 and the Phase167 checker, then stopped at active-milestone verification traceability before Cargo, Bazel or coverage. Its real-repository test discovered 23 cases: 22 passed and one failed because bounded Phase167 plan summaries list CVLD-02/03 in `requirements-completed` while `.planning/REQUIREMENTS.md` still marks all five CVLD IDs Pending and no truthful `167-VERIFICATION.md` exists yet. The checker currently treats every summary ID as globally activated and requires `status: passed` / `lifecycle_validated: true` verification before Plan20's full native gate can finish. Phase164's passed verification happens to mention CVLD-01/04/06 as deferred prose, so the current cross-phase text scan falsely covers those three. Writing an early passed Phase167 report, rewriting historical summaries or changing accurate requirement status would fabricate completion. This is a traceability checker workflow cycle and owner-boundary bug, not a product behavior failure.

ROOT adds exactly four present files to Plan20 Task1 ownership: `scripts/check-active-milestone-verification-traceability/checks.ts`, `success-and-coverage.ts`, `lifecycle-scenarios.ts` and `ownership.ts`. Plan20 now has **21 unique direct owners**, Task1 **18** and Task2 **three**. Of the 1,828 accepted product-source paths, **1,812 remain protected**; LOC is a separate generated owner and four reports are truly absent. Before editing, close jobs, capture each file's exact original/live whole body and the provisional LOC in a 21-owner stage frame, issue a supplementary ROOT source release and obtain independent review. The fifth default run remains diagnostic with source unchanged and no native/Bazel/coverage credit.

Keep activation and lifecycle validation of every active-phase summary, including cross-phase contributions, and retain the unique owner map, inconsistent checklist/table status errors and completed-without-summary errors. Require a passed, lifecycle-validated VERIFICATION from the requirement's **traceability owner phase** only when that requirement is both checklist-checked and traceability Complete. Run `lifecycleValidCoverage` once per active phase corpus with only its owned eligible IDs so invalid verification lifecycle errors are still seen and unrelated-phase text cannot confer coverage. Pending bounded plan contributions do not require a passed final report; they remain Pending until Plan20's actual native, source/security/goal and lifecycle gates pass.

Use only the three owned existing test modules to preserve strict negative behavior: locally promote tested synthetic RCN requirements to checked+Complete before expecting missing/malformed/near-token or archived-token verification orphans; keep duplicate/missing owner and inconsistent/completed-no-summary controls. Add a genuine RED for a Pending summarized requirement without any passed verification that should not be an orphan, and a regression where a completed requirement's token appears only in another phase's valid VERIFICATION and still must fail. The 176-line shared test-fixtures module remains protected; no helper or historical artifact edit is granted.

Run the focused RED cases and full active-milestone traceability suite/checker to GREEN, then the affected source/evidence guards and complete default `bash scripts/verify.sh` on one frozen source frame after provisional LOC regeneration and fresh locked coverage clean. Preserve raw attempts, discovery, source before/after and liveness. Plan20's final `167-VERIFICATION.md` remains Task2-owned and may declare passed only after actual full native/coverage/Bazel and independent source/security/goal acceptance. ROOT still owns final staged-index LOC/hook reconciliation. No production Rust, historical summary, REQUIREMENTS/STATE rewrite, fake passed verification, target/cache/flag/toolchain override, staging, Git action, global CVLD closure or Phase168 work is granted.
