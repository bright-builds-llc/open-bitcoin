---
phase: 157-safe-activation-and-scheduled-index-catch-up
reviewed: 2026-10-05T21:56:51Z
depth: standard
files_reviewed: 15
files_reviewed_list:
  - README.md
  - docs/parity/catalog/basic-compact-filters.md
  - docs/parity/checklist.md
  - docs/parity/index.json
  - docs/parity/source-breadcrumbs.json
  - packages/open-bitcoin-node/src/sync/tests/restart_chainstate/persist_and_hydrate.rs
  - scripts/check-phase135-snapshot-recovery.test.ts
  - scripts/check-phase135-snapshot-recovery.ts
  - scripts/check-phase155-filter-index.test.ts
  - scripts/check-phase155-filter-index.ts
  - scripts/check-phase157-index-catch-up.test.ts
  - scripts/check-phase157-index-catch-up.ts
  - scripts/check-phase157-index-catch-up/contracts.ts
  - scripts/check-phase157-index-catch-up/rust-evidence.ts
  - scripts/verify.sh
findings:
  critical: 0
  warning: 1
  info: 0
  total: 1
status: issues_found
generated_by: gsd-code-review
lifecycle_mode: yolo
phase_lifecycle_id: 157-2026-10-05T14-50-48
generated_at: 2026-10-05T21:56:51Z
review_partition: closeout
scope_sha256: 0f2a948d7e2a2d5d4220c000aa8f25b89fdc71f57d9294bf67af7ce6afbd5f5a
source_fingerprint: c5d9b4b0faa207a2a69913527f1d84543af12cf0ad8d5ddbe0db92f4facb9471
whole_phase_review_complete: false
verification_owner: root
file_sha256:
  README.md: cfe66c84b40a754df96c355b92f735c785052be38b9e687f39f270c4240231d3
  docs/parity/catalog/basic-compact-filters.md: bacf158451af4e749071960b0160d0f08624f402eb50a9a6dfc69cf782058eb1
  docs/parity/checklist.md: 018cc32d42e1ff7e85c49351e145a053ae2f241da4a571389c68da14a1971f37
  docs/parity/index.json: 28056f24f78ffa2be60b0388df4776232940c7dcc1ac4158f2682225e00d35d5
  docs/parity/source-breadcrumbs.json: 2c7fbd2740857a42e0df4d344ff22a5438e2929acc837b0c0a1c4186ae41a218
  packages/open-bitcoin-node/src/sync/tests/restart_chainstate/persist_and_hydrate.rs: 7167928042729b4fdd94680541777dd9d08560f597a695d2061a6c1c539c303a
  scripts/check-phase135-snapshot-recovery.test.ts: e0f8d462c7b7ce5fb1549ce08174b908c47ed2a2cd292a4158735531e9dcc5da
  scripts/check-phase135-snapshot-recovery.ts: a1dde8e9ac8ae2a74056b8fddff374aaa0a451f4aec3c71dedad5e84356e4bfb
  scripts/check-phase155-filter-index.test.ts: 8c09262bf1e323768d05946ed25e01e89e62b64df32924fc25d7066314be0a38
  scripts/check-phase155-filter-index.ts: ce6c2304fc9590961722e370795e821892c7c215bbfc71c15fbd058a3dd98cc0
  scripts/check-phase157-index-catch-up.test.ts: 48f362b3f6963ca6be6002a303bc350e36b02ba6413a1e9d6f708678334e9784
  scripts/check-phase157-index-catch-up.ts: 1a668faacbad31137afa29a1980b16ed68642f8733047f54c4d19fb985662c0d
  scripts/check-phase157-index-catch-up/contracts.ts: 1382b3f5673a7d885a16044ffe065aae3a5955e746d4c2b975526ace056660ad
  scripts/check-phase157-index-catch-up/rust-evidence.ts: f42969f1c01c16b062f6e34f456d21c0856b644cd6c00c11a386135cb1a6e07a
  scripts/verify.sh: d1d7681caccb93f70056fcee47baf6ce70e3a167b48d45fe41fd744a326c4ab1
---

# Phase 157: Closeout Code Review Report

**Reviewed:** 2026-10-05T21:56:51Z\
**Depth:** standard\
**Files Reviewed:** 15\
**Status:** issues_found

## Summary

The exact closeout partition covers Plan 10 source/claim guards, mutation controls, scoped documentation, parity/provenance registries, default verification integration, migrated Phase 155 startup guards and restart assertions, and the Phase 135 shutdown guard adaptation. One actionable warning concerns production-source classification of the actual async daemon function. No critical or informational findings were identified in this partition.

Material guidance came from AGENTS.md, AGENTS.bright-builds.md, placeholder-only standards-overrides.md, the standards index, architecture, code-shape, verification, testing and Rust/TypeScript standards. Both canonical active lesson inputs were loaded completely: 7,188 bytes and 2,397 estimated tokens. Archives were excluded. Phase context, plan/summary intent, final turn measurements, UAT and the iteration-one shutdown fix informed the review. Other source partitions and root verification/security/formal closure remain separate.

## Warnings

### WR-02: Test-only async daemon body still qualifies as ordinary production evidence

**File:** `/Users/peterryszkiewicz/Repos/open-bitcoin/scripts/check-phase157-index-catch-up/rust-evidence.ts:8`\
**Related:** `/Users/peterryszkiewicz/Repos/open-bitcoin/scripts/check-phase157-index-catch-up.ts:28-30`\
**Severity:** Warning

**Issue:** The test-only masker permits `pub` followed by `mod` or `fn`, but does not recognize `async fn`. The actual D-08 daemon boundary is `async fn serve_authoritative_runtime<...>`. Adding `#[cfg(test)]` immediately before that function leaves its entire body visible to `maybeFunction(source, symbol, true)`. The contract loop checks the returned body anchors without rejecting its test-only attribute, so this named ordinary daemon boundary can become test-only while the structural source guard still accepts it. Existing private/public/visibility-qualified module controls do not exercise this direct async-function case. This weakens the explicitly advertised T-157-30 ordinary-source regression gate; the full Rust build remains a separate detector.

**Evidence:** A read-only Bun diagnostic loaded the actual daemon source, inserted the attribute only in an in-memory string, and called the shipped production selector. Its output was:

```json
{"originalExists":true,"disabledStillProduction":true,"visibleAttribute":true}
```

The source, filesystem fixtures and registered test suites were unchanged. This is evidence about the structural selector, not Rust execution or runtime behavior.

**Fix:** Extend the existing masker to handle direct `#[cfg(test)] async fn` definitions, including their visibility-qualified form, or explicitly reject test-only attributes in production function selection. Add an independent mutation that marks the actual `serve_authoritative_runtime` function test-only and requires the ordinary-boundary diagnostic. A narrow modifier change suffices for the observed case:

```typescript
// Within the existing test-only definition matcher:
(?:mod|(?:async\s+)?fn)\s+\w+
```

Verify both the unmodified positive snapshot and that negative control using the explicit pinned-Bun test-file path. No custom Rust parser or broad guard rewrite is required.

## Scope Evidence and Limits

- Configured Enabled startup routes require full fresh/saved required-suffix body/undo validation before ownership/protection effects, saved-Active return or prune resume. Independent PreserveSaved compatibility routes retain full integrity/recovery and conservative protection checks. The migrated guards follow the actual recovered constructor.
- The Phase 135 adaptation follows the actual eager shutdown coordinator and daemon callbacks, with first-error and clean-marker ordering controls. The separate daemon partition owns behavioral rereview of the WR-01 production repair.
- The default verifier executes the new explicit-file mutation stage and live guard after Phase 156; the inert legacy command inventory is not treated as execution evidence.
- Published defaults and complete ledger observations distinguish admitted software work from RSS, comparisons, storage latency and whole-startup bounds. Closed-store reopen, genuine accepted-state/persistence failures, continuous validated history, 714 achieved paired deletes and codec-only nonactive volume are described separately.
- The current Phase 157 ledger remains in_progress. Empty enabled-history refusal and omitted double-negative warning are explicit differences. Deferred Phases 158–162, representation limits, exact-tip retention and the three v2.4 advisories remain visible. Synthetic done-gate report metadata is described as a metadata control, not formal proof.
- Existing whole-file README/checklist formatter drift was treated as preserved baseline evidence, not a new style finding. The explicit simplification pass favors one small matcher/control correction while retaining the existing declarative inventory, masker and snapshot helpers.

The existing final mutation log was inspected: 556 passed, zero failed and 1,470 expectations. Root handoff additionally reports 93 historical Phase 155, 84 unchanged Phase 156, 88 migrated Phase 135 controls, passing live guards and 997 checked Rust breadcrumbs. Those results were not rerun by this reviewer. No build, Git operation, registered test suite, production-source edit, shared-state edit or subagent was performed. Only this report was created.

Full native verification, source aggregation, security and formal lifecycle verification remain root-owned. This report does not mark the phase complete or establish arbitrary call-graph safety, performance, hardware power loss, public-network behavior, production readiness or funds safety.

## Fingerprint Contract

The durable 15-path list above is identical to `/tmp/phase157-closeout-review-files.json`. Each file hash is SHA256 of its bytes at review. `scope_sha256` hashes the ordered relative paths joined with one trailing LF per path. `source_fingerprint` hashes the ordered concatenation of each relative path, NUL, lowercase file SHA256 and LF. Any relevant source change requires refreshed fingerprints and rereview.

______________________________________________________________________

_Reviewer: gsd-code-reviewer_\
_Depth: standard; closeout partition only_
