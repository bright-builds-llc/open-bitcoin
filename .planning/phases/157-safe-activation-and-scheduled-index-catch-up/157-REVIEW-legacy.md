---
phase: 157-safe-activation-and-scheduled-index-catch-up
reviewed: 2026-10-05T22:49:18Z
depth: standard
generated_by: gsd-code-review
lifecycle_mode: yolo
phase_lifecycle_id: 157-2026-10-05T14-50-48
review_fix_iteration: 3
files_reviewed: 9
files_reviewed_list:
  - scripts/check-phase123-runtime-timing-evidence-integrity.test.ts
  - scripts/check-phase123-runtime-timing-evidence-integrity/checks.ts
  - scripts/check-phase128-production-compact-announcement-transport.test.ts
  - scripts/check-phase128-production-compact-announcement-transport.ts
  - scripts/check-phase128-production-compact-announcement-transport/impl-attributes.ts
  - scripts/check-phase134-apply-boundaries/aggregate-roots.ts
  - scripts/check-phase134-apply-boundaries/reachability.ts
  - scripts/check-phase134-authoritative-lifecycle.test.ts
  - scripts/check-phase134-authoritative-lifecycle.test/apply-helpers.ts
findings:
  critical: 0
  warning: 0
  info: 0
  total: 0
status: clean
source_fingerprint_algorithm: sha256(path-tab-sha256-newline-in-files-reviewed-list-order)
source_fingerprint: b9e0b1efab99ff1226484251742bc9d7c31cef6f671cf44cfa9d84dfed668317
source_fingerprints:
  scripts/check-phase123-runtime-timing-evidence-integrity.test.ts: 7fa536e33cf736e571a47f9842320819d0d46cf768e668764c049a102a5e6721
  scripts/check-phase123-runtime-timing-evidence-integrity/checks.ts: ca32f903da282afc28681993455a149ad1fc9173396360d9acacf0657026ba61
  scripts/check-phase128-production-compact-announcement-transport.test.ts: 9bd8a4584a0a2811ec5fb8acf53885aa3b67d3b8feb91e16dc84ec672eeef0fe
  scripts/check-phase128-production-compact-announcement-transport.ts: f96a9334f2943cc998d596201d254a007e4ab76e8dd7e489fed8efe83e2ec4cf
  scripts/check-phase128-production-compact-announcement-transport/impl-attributes.ts: c96066986863c90cf4f5a34d102f6b2b48b272a2a1d9b8ab11c20d370a1a2e5b
  scripts/check-phase134-apply-boundaries/aggregate-roots.ts: 87ae686fb5d7893dc8bc6a8ba721ed2d88ba1269c415be65ce207e1ff617c61d
  scripts/check-phase134-apply-boundaries/reachability.ts: 995235b55a56bcee5b64f4c185041e06e1af19737830306a3931099b5cea3d7c
  scripts/check-phase134-authoritative-lifecycle.test.ts: 2aa2515fbe92b4f81264da10affe11d1f7a951ba4b4fab2de816c6225a6b14fc
  scripts/check-phase134-authoritative-lifecycle.test/apply-helpers.ts: 574b61733a318667e25d50af076bf84fa60865a04366e912457c75a507aad2ea
---

# Phase 157: Legacy Verifier Code Review

**Reviewed:** 2026-10-05T22:49:18Z
**Depth:** standard
**Files Reviewed:** 9
**Status:** clean

## Summary

Read every file in the exact nine-file frozen legacy guard scope, including the new 26-line owning-implementation attribute helper. Independently checked current code and assertions against actual production routes, original WR-L01, iteration-three fix scope and execution logs. WR-L01 is closed; no new actionable bugs, security issues or maintainability findings were identified. All reviewed files meet quality standards. No issues found.

The original eight-file report remains preserved as `157-REVIEW-legacy.iter1.md`. All six Phase 123/134 files retain their original hashes; only two Phase 128 files changed and its coherent helper was added. The three Phase 128 hashes above exactly match iteration three's frozen hashes. The original 82-file phase scope is unchanged; these nine additional legacy files are explicitly listed here. Supporting Rust and scanner reads do not expand this durable list.

## WR-L01 Closure

`hasOrdinaryRuntimeOwner` at `scripts/check-phase128-production-compact-announcement-transport/impl-attributes.ts:2-25` requires the exact selected `impl DurableSyncRuntime` header and walks backward over its contiguous outer attributes. Balanced square brackets delimit each attribute, so `cfg(test)` is rejected before or after other attached attributes and across comments or whitespace. The caller supplies offset-preserving comment/literal-masked ordinary Rust ending at the owning implementation brace. Metadata literals cannot alter bracket or item boundaries. The scan stops when the preceding token is not an attribute, preventing an unrelated preceding test-only implementation or declaration from rejecting this owner.

The full-checker regression at test-file lines 46-60 retains an independent unmodified positive before the original intervening-attribute mutation and demands exactly the durable-trigger diagnostic. Table controls at lines 62-110 add seven further negatives and nine positives: reordered/multiple attributes, comments, bracketed literal metadata and spaced tokens; ordinary attributes, comment/string/raw-string decoys, non-test configuration and preceding-item bounds. These exercise the complete corpus and guard.

The genuine RED log reaches the final diagnostic assertion, receiving `[]` after its positive assertion passed. The GREEN log reports this regression passing with two assertions. The complete final Phase 128 log records 50 passing tests, zero failures and 67 assertions, including all 17 new controls. No shared parser change or global configuration-ignore behavior was introduced.

## Review Evidence

- Phase 123: `checks.ts:34-103` selects balanced, comment/literal-masked constructor bodies, checks compatibility delegation through `PreserveSaved`, requires configured initialization before recovery/network construction, and validates all six daemon arguments in order. Actual `sync/open_runtime.rs:29-94`, `sync.rs:110-122` and daemon `open-bitcoind.rs:302-319` match the predicates. Live/snapshot positives and independent configured-route, argument, order and comment mutations remain registered alongside all existing receive-clock, durable-write, inbound and verifier checks. Inspected the final log: 55 pass, zero fail, 103 assertions.
- Phase 128: `hasDurableSaveBeforeTip` at guard lines 202-250 still selects one ordinary `record_block_disposition`, requires its exact owner, and requires the requested best-chain arm to save unconditionally after the absent-block early return. The balanced error branch must end with the error return and contain no dependent hash/progress/tip/queue calls. Success hash evidence, accepted progress, tip classification and queueing follow the save in order. Actual `sync/block_response.rs:109-147` matches the route. All earlier save/error order, comment/literal, test-only method/module/implementation, unrelated-owner, transport, evidence, scope and verifier controls remain present. Imports proceed from Phase 128 to Phase 157's frozen helper to Phase 156 with no reverse cycle. The unchanged composed Phase 129 log records 12 pass, zero fail, 12 assertions.
- Phase 134: actual `lifecycle_projection/authority.rs:545-571` applies accepted dependent effects after its atomic core callback and before persistence error propagation. `aggregate-roots.ts:90-229` recognizes only the exact unconditional `apply_prepared_lifecycle(dependent); persist_result.map_err(LifecycleProjectionError::from)?; Ok(delta)` tail and blanks only the adapter span, preserving offsets. `reachability.ts:218-233` uses this narrow recognizer only for the connected root; extra or unrelated adapters remain fallible/unclassified. Atomic callback classification, strict syntax, qualified/aliased resolution, transitive helper inspection and mutation rejection remain active. The test entrypoint's `beforeAll` asserts live and snapshot positives before mutations. Eight accepted-order/propagation negatives and the comment/whitespace positive coexist with all aggregate, strict and token scanner suites. Inspected the final log: 257 pass, zero fail, 358 assertions, consistent with 235 negative and 22 positive cases plus two setup assertions.

Inspected evidence includes `/tmp/phase157-wrl01-red.log`, `/tmp/phase157-wrl01-green.log`, `/tmp/phase157-wrl01-phase128-final.log`, `/tmp/phase157-wrl01-phase129-final.log`, `/tmp/phase157-phase123-final-tests.log` and `/tmp/phase157-legacy134-final-tests.log`. Scoped evidence reports record passing live checks. This reviewer did not rerun their commands.

## Guidance, Simplification and Limits

AGENTS.md, AGENTS.bright-builds.md, placeholder-only standards-overrides.md and the local standards index, TypeScript, architecture, code-shape, testing and verification pages informed this review. Both canonical active global and repository lesson files were read completely: 7,188 bytes and 2,397 conservative estimated tokens. No archives were loaded. Project skill directories are absent; none of these paths match repository ignore patterns.

The explicit simplification pass confirms that the 26-line pure boolean helper bounds one local concern without a new parser framework, dependency or broad whitelist. The balanced save scan and shared connected-persistence recognizer retain their responsibilities. These guards are narrow structural contracts; they do not establish arbitrary Rust configuration semantics or replace runtime behavioral evidence.

Only this report was refreshed. No source, test, runtime, dependency, verifier, source-inventory or shared lifecycle file was changed. No test, Cargo, Bazel, native verifier or Git mutation was performed by this reviewer. Root owns aggregate review/source/security refresh, full native verification from its beginning after the initial pre-Rust failure, formal lifecycle closure and strict Git finalization. This clean partition report does not claim whole-phase completion or native verification success.

_Reviewer: gsd-code-reviewer_
_Depth: standard; review-fix iteration 3_
