---
phase: 167-candidate-history-validation-rules
plan: "16"
type: owner-refinement-amendment
generated_by: gsd-plan-phase
generated_at: "2026-10-27T21:32:56Z"
lifecycle_mode: yolo
phase_lifecycle_id: 167-2026-10-21T16-38-23
commits: false
---

# Plan 16 historical test module split

After accepted FINAL15, the five original Plan 16 owners are three present bodies and two true absences. The shared `phase167_support.rs` is 507 lines and `sync/tests.rs` is 611 lines; the new historical test parent must cover a bounded 77 MB source corpus, three exact script exceptions, actual creating-context kernel probes, both BIP30 repeats and physical restoration. The source-only [post-FINAL15 preparation](../../../.local/open-bitcoin-dev/phase167/plan16/post-final15-planning-v1/HANDOFF.md) and [independent review](../../../.local/open-bitcoin-dev/phase167/plan16/post-final15-planning-v1/independent-owner-refinement-review.json) passed planning checks only. Their exact index SHA-256 is `3103b5af822b8859f7802797d127515c25f6178191521e4355f6c2faff39c62b`; the review SHA-256 is `d055704bc9fc9950bc2ee477f12d418ac98373fa32ae8c38d6dab92aa8e13e2a`.

The five additional paths in the checked Plan 16 owner list were all physically absent before publication. They are test-only children of `sync/tests/phase167_historical.rs`: `source.rs` owns bounded source/context proof shared by both tasks; `scripts.rs` and `contexts.rs` own Task 1 script and creating-context cases; `bip30_repeats.rs` and `bip30_restore.rs` own Task 2 repeat/ancestry and physical restoration cases. The parent owns mechanical manager/checkpoint helpers and registers those children; `sync/tests.rs` adds only the parent module. Shared ordinary support and `candidate_rules.rs` remain within their original ownership, with no extra production authority or dependency.

The accepted FINAL15 causal frame has 467 source paths. Two present Plan 16 owners are in that frame; `candidate_rules.rs` is present outside it. The Plan 16 pre-source union is therefore 468 present paths with 465 nonowner paths. Creating the two originally absent owners and five new children yields a prospective 475-path union, ten owners and 465 protected nonowners. These are membership calculations, not a final source capture or an implementation pass. All children must receive pinned-source breadcrumb comments and fit normal formatting at no more than 628 lines; Plan 19 later reconciles their breadcrumb manifest entries.

The original two Task XML actions, acceptance criteria, command strings, dependency list, wave 33, requirements and lifecycle remain unchanged. The only task metadata change is the additive `<files>` allocation; the frontmatter `files_modified` union adds the five children. Task 1 still must prove exact script and creating-MTP behavior through the real managed kernel, and Task 2 must prove both exact repeats, physical all-handle reopen, complete prior-Coin restoration and all negatives. No test, Cargo, Bazel, source lease or contribution acceptance is supplied by this planning split.

For flag sensitivity, `candidate_rules.rs` forwards a flag argument that the actual apply site re-derives from selected history. Mutating only that inert argument earns no evidence. An invalid and valid flag-sensitive script pair under identical scalar facts must expose an interpreter bypass or wrong **consumed** flags. If a source-mutation inverse at the consumed site is required, it needs a separately reviewed disposable scratch edit with exact restoration; this amendment grants no live `apply.rs` ownership.

After a fresh before/after/protected/native recheck and a separate Root source lease, execute the two original commands serially with nonzero named discovery and genuine RED/GREEN. A diagnosed isolated `CARGO_TARGET_DIR` may be used without changing the literal Cargo argv or timeout contract. Full source/fixture/resource/coverage/Bazel/security/goal/lifecycle reviews, normal-hook commit/push on main, and STOP before Phase 168 remain required.
