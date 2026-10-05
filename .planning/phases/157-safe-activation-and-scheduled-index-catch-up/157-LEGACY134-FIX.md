---
phase: 157-safe-activation-and-scheduled-index-catch-up
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 157-2026-10-05T14-50-48
generated_at: 2026-10-05T22:33:00Z
status: targeted-checks-passed
git_finalization: pending root whole-phase verification and strict finalization
---

# Phase 134 accepted persistence ordering guard correction

The current connected-block transaction applies dependent lifecycle projections after its atomic core commit and before propagating `persist_result`. Plan 157-06 intentionally established this ordering: an already accepted block retains its mempool, peer, cache and dependent effects even when later persistence fails. Its behavioral evidence records a failing old-order control and a passing accepted-failure regression. This correction changes only historical TypeScript guard expectations and mutation controls.

## Checkable scope

- [x] Diagnose the actual connected root, Plan 06 and both failed positive boundaries before editing.
- [x] Recognize the exact accepted-effects-before-persistence-error tail, preserving all other classification and effect restrictions.
- [x] Require live and captured snapshot positives before any mutation control runs.
- [x] Retain inherited controls and independently reject reordered errors, omitted or conditional accepted effects, omitted propagation, changed mapper, extra `map_err` and comment spoofing.
- [x] Run the complete mutation suite, both live commands, managed checks, scoped TypeScript parsing, whitespace review and simplification review.
- [ ] Root independently reviews the final guard changes and refreshes source/security evidence.
- [ ] Root completes the full native verifier and strict Git finalization.

## Diagnosis and final behavior

The initial live apply command exited 1. Debug output showed `connectedBlockRootValid: false` and two reachable violations: fallible/effectful critical slice and unclassified `persist_result.map_err`. Both children stripped only the obsolete error-before-effects spelling. The initial complete suite reported 227 pass, 21 fail and 348 assertions in 12.71s. Twenty positive controls failed, and one negative fixture used a stale adjacent `apply`/`Ok(delta)` anchor. Those initial negative passes do not count as mutation evidence because the positive baseline failed.

The shared matcher operates on comments/literals-masked text and recognizes only the unconditional `self.apply_prepared_lifecycle(dependent); persist_result.map_err(LifecycleProjectionError::from)?; Ok(delta)` tail. It blanks only the exact persistence adapter span, preserving source length and receiver-resolution offsets. The structural root still requires its one accepted apply and unchanged atomic callback. Reachability uses the matcher only for the connected root; unrelated `map_err` calls remain unclassified and fallible. Existing helper recursion, qualified/aliased resolution, strict syntax, receiver purity and mutation rules remain active. No new parser or broad Result allowlist was added.

The test module checks the live repository and a copied source snapshot in `beforeAll`; a failing positive stops mutation execution. The stale order fixture now removes the actual apply statement, and eight independent negative controls cover the accepted-persistence tail. A positive with comments and whitespace around the exact adapter checks masking compatibility.

## Actual changed paths

1. `scripts/check-phase134-apply-boundaries/aggregate-roots.ts`
2. `scripts/check-phase134-apply-boundaries/reachability.ts`
3. `scripts/check-phase134-authoritative-lifecycle.test.ts`
4. `scripts/check-phase134-authoritative-lifecycle.test/apply-helpers.ts`
5. This evidence file.

The owned `fixture.ts`, `aggregate-reachability.ts`, `strict-reachability.ts` and `token-scanner-reachability.ts` required no changes. Their controls remain registered. Production Rust, dependencies, schema, verifier dispatch, shared state and other workers' files were not changed by this correction.

## Verification

Commands used pinned Bun 1.3.9 through `PATH=/tmp/open-bitcoin-bun-1.3.9/bun-darwin-aarch64:$PATH`.

| Check | Actual result |
| --- | --- |
| `bun test ./scripts/check-phase134-authoritative-lifecycle.test.ts` | Final **257 passed**, 0 failed, **358 assertions**, **15.41s**. Includes **235 negative controls** and **22 positive controls**, plus both mandatory positive assertions in `beforeAll`. |
| `bun scripts/check-phase134-apply-boundaries.ts` | Passed live target discovery, atomic core classification, connected order and transitive mutation-safety checks. |
| `bun scripts/check-phase134-authoritative-lifecycle.ts` | Passed live authority, target, effect, scenario, evidence and scope checks. |
| `bun scripts/bright-builds-check.ts all` | Zero findings; 1,324 tracked source files scanned, active repository lessons 1,958 bytes/653 estimated tokens. |
| Scoped `Bun.Transpiler({loader: "ts"})` and trailing-whitespace check | Four changed TypeScript paths parse and have no trailing whitespace; each remains below 628 lines. |

Logs are `/tmp/phase157-legacy134-before.log` and `/tmp/phase157-legacy134-final-tests.log`. The simplification pass consolidated the two exact adapter recognizers into one pure helper, retained the existing scanner and reused existing fixture mutation functions. Owned patches and final source were reviewed for unintended changes. Independent Git diff review remains root-owned; this worker makes no native or phase completion claim. No Cargo or Bazel work, staging, commit, push, shared lifecycle update or ignored build-cache enumeration occurred.

## Guidance and limits

AGENTS.md, its Bright Builds sidecar, placeholder-only overrides, the standards index, architecture, code-shape, testing, verification and TypeScript guidance informed this work. Both active global and repository lessons were read completely: 7,188 bytes and 2,397 conservative estimated tokens; no lesson archive was loaded. Existing GSD execution and the explicit scope amendment authorize these narrow guard edits; root owns synchronization and finalization. Installed `mdformat` exposes no parser extensions, so it was not used on this frontmatter/table evidence file. No new formatter, dependency or runtime trust surface was added.

## Self-Check: PASSED

The four changed source paths and this evidence file exist. Final complete positive/negative results and exact scoped ownership are recorded above. No goal-blocking stub or new security-relevant runtime surface was introduced. Commits are deliberately deferred to root after independent review and whole-phase native verification.
