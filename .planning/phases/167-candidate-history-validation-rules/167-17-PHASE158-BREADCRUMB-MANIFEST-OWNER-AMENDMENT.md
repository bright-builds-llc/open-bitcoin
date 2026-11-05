---
phase: 167-candidate-history-validation-rules
plan: "17"
type: checked-execution-amendment
generated_by: gsd-execute-plan
generated_at: "2026-10-31T01:23:27Z"
lifecycle_mode: yolo
phase_lifecycle_id: 167-2026-10-21T16-38-23
commits: false
---

# Plan 17 Phase 158 breadcrumb manifest dependency

The reviewed current Phase 158 guard must register the moved `complete_coins_write` owner at `packages/open-bitcoin-node/src/chainstate/flush_lifecycle/metadata.rs`. Its `CHECK_FILES` path reaches `checkBreadcrumbs`, which requires every Rust source input to have exactly one nonempty `docs/parity/source-breadcrumbs.json` group whose anchor comments match the source. The source has `validation.cpp` and `txdb.cpp` breadcrumb comments; the FINAL51 manifest has no group for this path. Removing the source from `CHECK_FILES` or exempting it would disconnect the current writer from the guard and leave the repository breadcrumb check red.

ROOT adds exactly `docs/parity/source-breadcrumbs.json` to Plan17 Task1 ownership, after the independently reviewed 23-owner Phase158 release. The checked allocation becomes **24 unique paths**, tasks **13 / 6 / 5**, with **23 original-present whole bodies and one truly absent Phase123 fixture child**. The post-FINAL51 1,817-file source frame now has **1,794 protected original paths**. This is a single current manifest repair, not ownership of the remaining Phase167 breadcrumb catalog. Before the manifest changes, close all owned Bun jobs, capture its exact original and live whole bytes in a 24-owner stage frame, issue an exact supplementary ROOT lease, and obtain independent release review. The worker must not edit the manifest under the prior 23-owner lease.

Add exactly one truthful, unique Phase167 metadata writer group for the moved source with the two existing Knots anchor comments. Preserve all prior groups, ordering constraints, frozen historical snapshots, and receipts. Keep the Phase158 checker’s normal Rust breadcrumb enforcement and the Phase164 strict current-to-prior-current tooling inverse. Test missing, duplicate, wrong-anchor, and absent-source failure paths where the existing test scope permits. Run the pinned Bun 1.3.9 standalone Phase158 checker and focused suite, and the repository breadcrumb checker; report any other Phase167 breadcrumb omissions as Plan19 work, with exact failure evidence rather than a waiver. Final Plan17 must still run Phase163 native/provenance, affected guards, four original Plan17 commands, original-before inverse and independent source/CORE QA on a frozen frame.

Plan19 retains ownership of the complete final Phase167 breadcrumb catalog and parity docs. It must rebase on this accepted Plan17 manifest addition instead of reverting or duplicating it. No production Rust, generic parser, verifier wiring, other parity document, historical artifact, additional source owner, Git action, global CVLD closure or Phase168 work is granted.
