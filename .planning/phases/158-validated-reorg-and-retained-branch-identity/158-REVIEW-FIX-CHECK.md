---
phase: 158-validated-reorg-and-retained-branch-identity
reviewed: 2026-10-09T03:51:45Z
depth: standard
files_reviewed: 2
files_reviewed_list:
  - packages/open-bitcoin-node/src/storage/fjall_store/filters/reorg.rs
  - packages/open-bitcoin-node/src/sync/tests/filter_index/reorg/branches.rs
findings:
  critical: 0
  warning: 0
  info: 0
  total: 0
status: clean
resolved_findings:
  - WR-158-01
historical_review: 158-REVIEW-STORAGE.md
fix_evidence: 158-REVIEW-FIX.md
---

# Phase 158: WR-158-01 Independent Fix Check

**Status:** clean for the two-file repair scope; WR-158-01 is resolved.
**Reviewed:** 2026-10-09T03:51:45Z
**Depth:** standard, with exact authority and test-path cross-checks.

## Scope and Conclusion

Both complete changed files were independently read, including the entire new production regression, the shared negative-control body and all three new negative tests. The pre-fix 20-file report remains historical evidence. Comparing every original reviewed SHA-256 with current content found exactly one changed original file, `storage/fjall_store/filters/reorg.rs`; all other 19 original files remain byte-for-byte identical. The new regression file was separately reviewed in full for this repair.

The revised guard at `/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-node/src/storage/fjall_store/filters/reorg.rs:182-185` is:

```rust
if (previous.durable_height > old.0 && !proof.durable_displaced())
    || matches!(previous.processed.prefix(), IndexPrefix::Committed(id) if id.height() > old.0)
{
    return Err(index_corruption("BASIC displaced endpoint behind progress"));
}
```

This implements the narrow requested exception. The existing `durable_displaced()` in the unchanged `filters/append.rs:590-593` succeeds only when the private achieved replacement marker equals the proof's complete actual durable `(height, hash)` tuple. The same-store captured identity, current control revision, ownership generation/checkpoint, processed frontier and actual coins checks precede this guard and remain unchanged. The processed-height condition is independent. Genuine manager staging/accepted lineage and complete guarded publication still apply after preparation. Next-marker recomputation still checks the genuine replacement branch, allowing a return to already durable A to clear displacement without forging a new coins achievement.

No public constructor, production test factory, authority flag, alternate publication route, forced index-specific flush, retained cache, new schema, dependency or interface was added. The negative setup lives inside `#[cfg(test)]` and corrupts only previously earned identity; it cannot mint a valid displaced binding in production.

## Production and Negative Evidence

The new `phase158_validated_reorg_shorter_unflushed_return_preserves_coins_fence_and_reopens` at `sync/tests/filter_index/reorg/branches.rs:259` genuinely accepts A23 → B14, finishes ordinary catch-up, retains B undo from the accepted snapshot, and then invokes the same ordinary network reorg caller for B14 → A23 before any own coins flush.

Its explicit `save_undo` operations retain required historical source payloads only; they do not write coins or chain metadata. The test asserts actual coins still identify A23 and safe progress is ancestor 10 before the return. After return it asserts processed/safe 10 and conservative protection, completes normal catch-up, verifies exact original A active commitments, zero accepted lag, unchanged actual A23 coins and retained B hash records, then performs ordinary own flush, full configured reopen and exact A/B retention checks. Safe A23 after catch-up is correct because the returned accepted branch matches the existing real durable A23 fence.

The three negative cases at `filters/reorg.rs:528,533,541` start from genuine A2 → B1 staging, absorption and append. They remove the marker, change its height, or change its hash. The helper first proves `durable_displaced()` is false and that the ordinary guarded proof authentication still passes, then demands the precise height-guard refusal and unchanged actual coins/checkpoint. This isolates the new exception and does not substitute a fake genuine receipt.

The repair is simpler than adding another ancestry rule: it reuses the existing exact predicate, preserves the independent processed check and exercises the actual serialized production caller. No further concrete bug, security issue or test-reliability finding was identified in the repair.

## Verification Attribution and Limits

The fix worker's checked-in `158-REVIEW-FIX.md`, corroborated directly by the parent and worker, records actual executed results:

| Timing key | Result |
| --- | --- |
| `phase158-wr01-red-retained-sources` | Original guard: exit 101; 0 passed / 1 failed / 0 ignored / 1,355 filtered; precise production `BASIC displaced endpoint behind progress` failure; 1.25s harness |
| `phase158-wr01-green` | Repaired guard: exit 0; 1 passed / 0 failed / 0 ignored / 1,355 filtered; 1.87s harness |
| `phase158-wr01-negative` | Exit 0; 3 passed / 0 failed / 0 ignored / 1,353 filtered; 2.12s harness |

The evidence also records workspace formatting, breadcrumb validation and diff checks passing. These are worker executions, not reruns by this reviewer. No Cargo/Bazel or source/Git mutations occurred in this independent check. Full Phase 158 tests, strict Clippy and the final native contract remain parent/fix-worker gates; this report does not claim their pending outcomes.

Material guidance remains the previously fully loaded AGENTS/sidecar/overrides/core standards/Rust/phase contracts and bounded active lessons. The prior report's scope and evidence limitations remain intact.

## Source Fingerprints

Captured at 2026-10-09T03:51:45Z. Both repair hashes match the fix worker's evidence. The other 19 original reviewed hashes match the historical report.

| File | SHA-256 |
| --- | --- |
| `packages/open-bitcoin-node/src/storage/fjall_store/filters/reorg.rs` | `f85e0da039b66b24669b3a085cca2d5c70889d5d28ef5b0098439f37b581e715` |
| `packages/open-bitcoin-node/src/sync/tests/filter_index/reorg/branches.rs` | `a9bcf1d5e3de9e719f29cbff8e6fac29511dc3571c1326b52cd880497e23654e` |

***

_Reviewer: gsd-code-reviewer, independent core/storage fix check_
