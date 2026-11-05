---
phase: 167-candidate-history-validation-rules
plan: "20"
type: checked-execution-amendment
generated_by: gsd-execute-plan
generated_at: "2026-10-31T16:43:29Z"
lifecycle_mode: yolo
phase_lifecycle_id: 167-2026-10-21T16-38-23
commits: false
---

# Plan 20 panic-site test-only module ownership

The seventh complete default verifier attempt passed the checked Phase127/128/135 and active-milestone guards, then stopped at the panic-site policy CLI before Cargo, Bazel or coverage. The panic checker’s own 51 tests passed, but its CLI reported 244 unclassified panic-like sites across 16 first-party `src` files. Independent source and checker audits traced 235 sites to 15 external test-child files with real direct parent `mod` registrations under `#[cfg(test)]` or exact `#[cfg(all(test, unix))]`, including one explicit daemon `#[path]` child and a network `lib.rs` child. The other nine sites are inside `inspection.rs`'s inline `#[cfg(all(test, unix))] mod namespace_tests`. No reported site is in ordinary production code. The checker recognizes only three older hardcoded transitive children and exact `#[cfg(test)]`; the new test modules therefore lack source-derived attribution. Neither adding 244 allowlist entries nor editing Rust tests would address that ownership rule.

ROOT adds exactly `scripts/check-panic-sites.ts` and `scripts/check-panic-sites.test.ts` to Plan20 Task1. The checked direct owner set becomes **31 files**, Task1 **28** and Task2 **three**; of the 1,828 accepted product paths, **1,802 remain protected**. LOC stays a separate generated owner and four reports are truly absent. Before either new owner changes, close all jobs, capture its exact original/live whole body and current provisional LOC in a 31-owner stage frame, issue a supplementary ROOT source release and obtain independent review. The default-v7 panic finding and 51/51 checker tests remain diagnostic for their original source frame.

Add an explicit 15-row full child-path → real parent/module/optional-path manifest, and verify each unique live parent declaration through the existing Rust-aware parser and `selectedModule`; do not confer test authority from a filename pattern or blanket allowlist. Preserve the three existing special transitive CHILDREN controls. Recognize only exact `#[cfg(test)]` and `#[cfg(all(test, unix))]` as test-only gates for these current owners and inline masking; `cfg(any(test, ...))`, dead `cfg(any())`, `cfg_attr`, comment/string decoys and incomplete source cannot qualify. Add these 15 children to known authority files and reject any second module registration or `include!` alias to a known child, even if the alias is itself test-gated; ordinary module/include aliases remain rejected. Retain reporting of a genuine unguarded production panic.

Add source-derived tests for each parent gate, removed/redirected path, duplicated or ordinary alias, dead/conditional/comment decoy, exact inline compound-gate positive and ungated production panic. Run the original 51 tests plus new cases and the full real checkout panic-site CLI to GREEN, with zero unclassified production findings after exact module attribution. Preserve `scripts/panic-sites.allowlist`, every Rust file and all other checker/historical owners byte-for-byte. Then regenerate provisional worktree LOC, perform fresh locked coverage clean and rerun complete default `bash scripts/verify.sh` on one frozen source. Keep raw attempts, discovery, source before/after and liveness. Independent source/security/goal reports and ROOT staged-index LOC/hook reconciliation remain mandatory. No target/cache/flag/toolchain override, staging, Git action, global CVLD closure or Phase168 work is granted.
