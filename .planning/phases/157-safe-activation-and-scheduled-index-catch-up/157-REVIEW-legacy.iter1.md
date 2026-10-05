---
phase: 157-safe-activation-and-scheduled-index-catch-up
reviewed: 2026-10-05T22:37:06Z
depth: standard
generated_by: gsd-code-review
lifecycle_mode: yolo
phase_lifecycle_id: 157-2026-10-05T14-50-48
files_reviewed: 8
files_reviewed_list:
  - scripts/check-phase123-runtime-timing-evidence-integrity.test.ts
  - scripts/check-phase123-runtime-timing-evidence-integrity/checks.ts
  - scripts/check-phase128-production-compact-announcement-transport.test.ts
  - scripts/check-phase128-production-compact-announcement-transport.ts
  - scripts/check-phase134-apply-boundaries/aggregate-roots.ts
  - scripts/check-phase134-apply-boundaries/reachability.ts
  - scripts/check-phase134-authoritative-lifecycle.test.ts
  - scripts/check-phase134-authoritative-lifecycle.test/apply-helpers.ts
findings:
  critical: 0
  warning: 1
  info: 0
  total: 1
status: issues_found
source_fingerprint_algorithm: sha256(path-tab-sha256-newline-in-files-reviewed-list-order)
source_fingerprint: 481ba4bb98066345a0d0db13599fd9c893a823f367a834bacf610059cbdd4f72
source_fingerprints:
  scripts/check-phase123-runtime-timing-evidence-integrity.test.ts: 7fa536e33cf736e571a47f9842320819d0d46cf768e668764c049a102a5e6721
  scripts/check-phase123-runtime-timing-evidence-integrity/checks.ts: ca32f903da282afc28681993455a149ad1fc9173396360d9acacf0657026ba61
  scripts/check-phase128-production-compact-announcement-transport.test.ts: 66b5d281c73bfa62f336baccff1ca19b063b5b9d88aa18c1f4b58e3bad3bd84e
  scripts/check-phase128-production-compact-announcement-transport.ts: e901cbbed2bc6c025152ca7905c037168fc4c5ba0bb8b813ccb9c974cc543911
  scripts/check-phase134-apply-boundaries/aggregate-roots.ts: 87ae686fb5d7893dc8bc6a8ba721ed2d88ba1269c415be65ce207e1ff617c61d
  scripts/check-phase134-apply-boundaries/reachability.ts: 995235b55a56bcee5b64f4c185041e06e1af19737830306a3931099b5cea3d7c
  scripts/check-phase134-authoritative-lifecycle.test.ts: 2aa2515fbe92b4f81264da10affe11d1f7a951ba4b4fab2de816c6225a6b14fc
  scripts/check-phase134-authoritative-lifecycle.test/apply-helpers.ts: 574b61733a318667e25d50af076bf84fa60865a04366e912457c75a507aad2ea
---

# Phase 157: Legacy Verifier Code Review

**Reviewed:** 2026-10-05T22:37:06Z
**Depth:** standard
**Files Reviewed:** 8
**Status:** issues_found

## Summary

Read every file in the exact eight-file additional guard scope. The Phase 123 configured startup and Phase 134 accepted-effects ordering corrections match the current production source. The Phase 128 correction checks the correct save-success and save-failure order, but its enclosing-implementation check can accept a test-only implementation when another attribute follows `cfg(test)`. One actionable warning remains in this partition.

This report covers only the listed guard and test files. Runtime files and frozen scanner helpers were read as supporting context, not added to the durable review scope. It makes no whole-phase verification or completion claim.

## Warnings

### WR-L01: An intervening attribute hides a test-only implementation

**File:** `/Users/peterryszkiewicz/Repos/open-bitcoin/scripts/check-phase128-production-compact-announcement-transport.ts:213-216`

**Issue:** `hasDurableSaveBeforeTip` rejects only the compact suffix `#[cfg(test)]implDurableSyncRuntime`. Its imported `ordinaryRust` masks test-only functions and modules, but not implementations. Consequently, replacing the current implementation header with the following valid Rust attributes leaves the save body visible and accepted:

```rust
#[cfg(test)]
#[allow(dead_code)]
impl DurableSyncRuntime {
    // Existing record_block_disposition body.
}
```

The compact owner still ends in `implDurableSyncRuntime`, while the rejection regex fails because `#[allow(dead_code)]` intervenes. `maybeFunction(..., true)` still selects the ordinary-looking method body because neither its method nor an enclosing module has a test attribute. All subsequent save/order predicates operate on the unchanged body. The verifier can therefore report a production durable-save path even though the implementation is excluded from non-test builds. The existing negative at test-file line 58 exercises only adjacent `cfg(test)` and `impl`.

**Fix:** Inspect the entire attribute block attached to the selected enclosing implementation, with comments/literals masked and a bounded preceding-item boundary. Reject a test configuration anywhere in that block regardless of the order or presence of other attributes, while retaining the exact `DurableSyncRuntime` owner requirement. Keep the frozen imported helper unchanged if it remains outside the repair scope. Add independent actual-corpus negatives for `cfg(test)` before and after another implementation attribute, requiring exactly the durable-trigger diagnostic and a passing unmodified positive first.

## Review Evidence

- Phase 123: inspected configured construction in `sync/open_runtime.rs`, block-relay compatibility in `sync.rs`, and daemon argument forwarding in `open-bitcoind.rs`. Balanced comment/literal-masked function extraction retains the initialization-before-recovery/network sequence and exact activation arguments. The tests assert an unchanged live snapshot before each table mutation, require the intended activation diagnostic, and cover removal, argument order, initialization order and comment decoys. Existing receive-clock, durable-write, inbound and verifier checks remain registered. The recorded final log contains 55 passing tests, zero failures and 103 assertions.
- Phase 128: inspected `record_block_disposition` in `sync/block_response.rs`. Requested best-chain acceptance saves before local-hash evidence, accepted progress, best-tip classification and announcement queueing; its explicit error branch returns before those effects. The new predicate requires that route and rejects missing/conditional save, ignored failure and dependent effects in the error branch. The thirteen new independent negatives demand exactly the durable-trigger diagnostic. Comment/literal masking and test-only function/module controls are present. The implementation-attribute exception described above is not covered. Imports go from Phase 128 to the frozen Phase 157 helper and then Phase 156; those helpers have no reverse Phase 128 import. The fix report records 33 Phase 128 and 12 unchanged Phase 129 tests passing; this reviewer did not rerun them.
- Phase 134: inspected `commit_connected_block_lifecycle_transaction` in `lifecycle_projection/authority.rs`. The actual atomic core callback precedes unconditional dependent application; persistence error propagation follows accepted effects. The shared recognizer masks only the exact `persist_result.map_err(LifecycleProjectionError::from)?;` span in the connected root and retains offsets. Unrelated adapters stay subject to fallibility and classification checks. Root callback classification, strict syntax, recursive helper inspection and mutation rejection remain active. Both live and snapshot positives run in `beforeAll` before the negative controls. Eight added negatives require the exact apply-boundary diagnostic for removed, conditional, reordered or spoofed effects/propagation and additional adapters. Existing aggregate, strict and token scanner suites remain imported and registered. The recorded final log contains 257 passing tests, zero failures and 358 assertions, consistent with 235 negative and 22 positive cases plus the two setup assertions.

## Guidance, Simplification and Limits

AGENTS.md, AGENTS.bright-builds.md, placeholder-only standards-overrides.md and the local standards index, TypeScript, architecture, code-shape, testing and verification pages informed this review. Both canonical active global and repository lesson files were read completely: 7,188 bytes and 2,397 conservative estimated tokens; no archive input was loaded. No project skill directory was present. The selected paths do not match the repository ignore patterns.

The explicit simplification pass favors the existing balanced scanners and the one shared connected-persistence recognizer. The warning can be repaired by bounding the enclosing implementation's attribute check; it does not require a new parser, dependency or production behavior change. Source guards remain structural contracts and do not replace runtime behavioral evidence.

No source, runtime, test, manifest, dependency, verifier, Git or shared lifecycle files were changed. No test, Cargo, Bazel or native verifier command was executed by this reviewer under the delegated read-only scope. Existing logs and assertion source were inspected; root owns fresh reproduction of the finding, correction review, source fingerprints, the aggregate scope amendment and a complete restart of the native verifier from its beginning. Full native verification remains pending after the initial pre-Rust failure.

_Reviewer: gsd-code-reviewer_
