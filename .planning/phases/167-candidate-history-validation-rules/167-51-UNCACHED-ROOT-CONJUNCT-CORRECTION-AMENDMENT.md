---
phase: 167-candidate-history-validation-rules
plan: "51"
type: checked-execution-amendment
generated_by: gsd-execute-plan
generated_at: "2026-10-30T16:20:45Z"
lifecycle_mode: yolo
phase_lifecycle_id: 167-2026-10-21T16-38-23
commits: false
---

# Plan 51 exact absent-cache conjunct for genesis

The first root-only E1 correction made four old actual root/reopen/witness Duplicate controls pass 1/1 each, and E4 cached-descendant/error 11/11 plus E3 failed-ancestor 1/1 remained green. Its predicate nevertheless used `cached == Some(block) || root_predicate`, so the root exception could pass even when a **different** body occupied the volatile cache. The checked root-Duplicate amendment authorized only an **uncached** proved genesis body. Independent source review returned HOLD on this exact authority distinction. All four v3 task selectors passed 7/20/20/11 but are diagnostic; no final twenty-command v3 run began.

ROOT grants a second exact E1-only correction after the current whole body/hash and complete v3 frame are copied. Factor the cache lookup once and require `cached == Some(block) || (cached.is_none() && root_predicate)` (or an equivalent guard with identical meaning). Keep the root predicate's full selected canonical genesis body/hash and active height-zero proof, and keep all current header/binding/epoch/availability/not-failed and `eligible_body_candidates` full-linkage checks. Every non-root body remains subject to exact cache equality. Do not insert into the cache, bypass a conflicting cache entry, widen a public constructor, or change E4's same-store load and real error/None stages.

No genuine wrong-cached-root state is reachable through the existing public/owned test APIs without direct private cache mutation: canonical `guard_body` checks the full genesis block including body/witness before candidate cache insertion. Do not add a private mutation hook or a new test owner solely to fabricate this state. Independent source audit of the exact conjunct, the four old root positives, E4 cached descendant/corrupt/missing controls and E3 failed-ancestor negative are the bounded proof. Retain the first incomplete fix and independent HOLD as diagnostic history.

After the one-owner fix, rerun those focused controls, normal rustfmt/strict source fit, all four literal task selectors, complete source/native/Knots frame and strict original-before BSD `patch -F0` inverse/forward. Only after independent source re-review may all twenty published final commands restart fresh from format; v2 command 1–10 passes/command 11 failure and v3 task passes are not final-matrix credit. No target/cache override, unowned source edit, Git action, global CVLD closure or Phase 168 work is granted.
