---
phase: 167-candidate-history-validation-rules
plan: "20"
type: checked-execution-amendment
generated_by: gsd-execute-plan
generated_at: "2026-10-31T04:04:12Z"
lifecycle_mode: yolo
phase_lifecycle_id: 167-2026-10-21T16-38-23
commits: false
---

# Plan 20 LOC and staged-source finalization

The repository's LOC generator enumerates `git ls-files` in both worktree and index modes. At the post-Plan17 checkpoint, 120 LOC-included first-party files are untracked; Plan18 creates three more TypeScript files. `scripts/verify.sh` checks the worktree LOC report before its other gates. The normal pre-commit hook regenerates the report from the staged index, stages that generated change and reruns the full default verifier. Therefore a pre-staging worktree LOC check can pass while omitting new source that the final index report includes.

Plan20 first generates the provisional worktree LOC report with `bun run scripts/generate-loc-report.ts --source=worktree --output=docs/metrics/lines-of-code.md` under the pinned Bun path, then runs the complete default `bash scripts/verify.sh`, fresh pure-core coverage and Bazel on the final worktree source, with no staging or Git action by its executor. Its `167-VALIDATION.md` must label that LOC check **provisional until staging** and bind all other command outcomes to exact source hashes. Plan20's independent source/security/goal review must cover the complete new source bytes, including untracked files, and define its digest scope explicitly. A code/config/document digest may exclude only the generated LOC artifact if that exclusion and separate LOC hash are recorded in each report; no other source is excluded.

ROOT alone performs final normal-hook Git staging after Plan20's clean handoff. Stage all intended source, docs and summaries before committing; verify the index contains each new file. Capture the pre-hook and post-hook `docs/metrics/lines-of-code.md` hashes and the exact index delta. The hook's index-mode LOC generation and full default verifier must exit zero on staged bytes with no bypass. If LOC is the sole delta and the canonical reports explicitly exclude LOC from their matching code/config/document digest, independently review and bind the exact LOC delta to that unchanged digest. For every report whose matching final-source digest includes LOC, reissue or revalidate that report against the post-hook staged-index hash even when LOC is the sole delta. Any other changed reviewed byte invalidates its dependent source/security/goal or default verification gate and requires a rerun. Keep final hook logs and reconciliation in an ignored ROOT receipt before push, then commit and push the verified index on `main`.

This amendment changes no Plan20 owner, verifier command, coverage threshold, frozen historical artifact, lifecycle, staging authority or phase boundary. It grants no early phase acceptance. Stop before Phase168.
