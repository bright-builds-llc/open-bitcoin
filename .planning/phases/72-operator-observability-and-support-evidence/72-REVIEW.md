---
phase: 72-operator-observability-and-support-evidence
reviewed: 2026-06-13T20:26:54Z
depth: standard
files_reviewed: 30
files_reviewed_list:
  - docs/architecture/operator-observability.md
  - docs/architecture/status-snapshot.md
  - docs/metrics/lines-of-code.md
  - docs/operator/runtime-guide.md
  - docs/parity/catalog/chainstate.md
  - docs/parity/catalog/operator-runtime-release-hardening.md
  - docs/parity/catalog/p2p.md
  - docs/parity/source-breadcrumbs.json
  - packages/open-bitcoin-cli/src/operator.rs
  - packages/open-bitcoin-cli/src/operator/dashboard/model.rs
  - packages/open-bitcoin-cli/src/operator/dashboard/model/tests.rs
  - packages/open-bitcoin-cli/src/operator/status/render.rs
  - packages/open-bitcoin-cli/src/operator/status/render/tests.rs
  - packages/open-bitcoin-cli/src/operator/support.rs
  - packages/open-bitcoin-cli/src/operator/support/evidence.rs
  - packages/open-bitcoin-cli/src/operator/support/live_smoke.rs
  - packages/open-bitcoin-cli/src/operator/support/render.rs
  - packages/open-bitcoin-cli/src/operator/support/tests.rs
  - packages/open-bitcoin-cli/src/operator/sync_truth_render.rs
  - packages/open-bitcoin-cli/tests/operator_binary.rs
  - packages/open-bitcoin-node/src/metrics.rs
  - packages/open-bitcoin-node/src/sync/tests.rs
  - packages/open-bitcoin-node/src/sync/types/summary.rs
  - packages/open-bitcoin-node/src/sync/types/summary/tests.rs
  - packages/open-bitcoin-rpc/src/dispatch/tests.rs
  - scripts/check-phase71-resource-restart.ts
  - scripts/check-phase72-observability-evidence.ts
  - scripts/run-live-mainnet-smoke.ts
  - scripts/test-run-live-mainnet-smoke.sh
  - scripts/verify.sh
findings:
  critical: 0
  warning: 4
  info: 0
  total: 4
status: issues_found
---

# Phase 72: Code Review Report

**Reviewed:** 2026-06-13T20:26:54Z
**Depth:** standard
**Files Reviewed:** 30
**Status:** issues_found

## Summary

Reviewed the Phase 72 operator observability, support evidence, live-smoke, docs, parity, and checker changes at standard depth. Repo guidance from `AGENTS.md`, `AGENTS.bright-builds.md`, and `standards-overrides.md` materially informed this review; local `standards/` and project skill directories were not present in the worktree.

The main risks are support verdicts overstating diagnosed blockers, human support Markdown dropping partial active-chain evidence, live-smoke final status synthesizing validated active-chain height from other fields, and the deterministic Phase 72 checker accepting a forbidden-field test as positive evidence coverage.

Verification run during review:

- `bun --check scripts/check-phase72-observability-evidence.ts`
- `bun run scripts/check-phase72-observability-evidence.ts`

## Warnings

### WR-01: Support Verdict Treats Peer Shortage As A Diagnosed Blocker

**File:** `packages/open-bitcoin-cli/src/operator/support/evidence.rs:310`
**Issue:** `resource_pressure_indicates_blocker` returns true whenever `outbound_peers < target_outbound_peers`. That condition can happen during ordinary catch-up, DNS/manual-peer churn, or a still-progressing run, but `derive_full_sync_evidence` upgrades it to `diagnosed_blocker` when sync-to-tip evidence is missing. Phase 72's verdict contract says `diagnosed_blocker` should be evidence-derived from explicit blocking evidence, not normal partial peer availability.
**Fix:** Remove peer-count shortfall from this blocker predicate or gate it behind an explicit no-progress/recovery signal.

```rust
fn resource_pressure_indicates_blocker(value: &SyncResourcePressure) -> bool {
    value.max_blocks_in_flight_total > 0
        && value.blocks_in_flight >= value.max_blocks_in_flight_total
}
```

Add a regression test with `outbound_peers < target_outbound_peers`, no blocking no-progress diagnosis, and missing tip match that must remain `inconclusive`.

### WR-02: Support Markdown Hides Available Active-Chain Height

**File:** `packages/open-bitcoin-cli/src/operator/support/render.rs:192`
**Issue:** `active_chain_summary` returns only `Unavailable: {reason}` whenever hash or work is missing. The JSON evidence can still contain an available `height`, but the human support bundle drops that partial evidence. This weakens the support handoff and conflicts with the Phase 72 requirement to preserve available values alongside unavailable reasons.
**Fix:** Always render height/hash/work slots and append the unavailable reason separately.

```rust
fn active_chain_summary(evidence: &super::ActiveChainEvidence) -> String {
    let base = format!(
        "height={} hash={} work={}",
        evidence.height.map(|value| value.to_string()).unwrap_or_else(|| "Unavailable".to_string()),
        evidence.hash.as_deref().unwrap_or("Unavailable"),
        evidence.work.as_deref().unwrap_or("Unavailable"),
    );
    match evidence.maybe_unavailable_reason.as_ref() {
        Some(reason) => format!("{base} unavailable_reason={reason}"),
        None => base,
    }
}
```

Extend the existing unavailable support-bundle test to assert the Markdown still contains `height=840004`.

### WR-03: Live-Smoke Synthesizes Validated Active-Chain Height

**File:** `scripts/run-live-mainnet-smoke.ts:1740`
**Issue:** When `validated_active_chain_height` is absent from final sync status, the live-smoke report falls back to `connectedBlockHeight`, then `blockHeight`, then `0`. Phase 72 deliberately keeps validated active-chain height as a separate evidence field. Synthesizing it can make older or partial status output look like explicit validated active-chain proof.
**Fix:** Preserve absence as `null` and render a specific unavailable reason instead of deriving this field from connected/block height.

```typescript
validatedActiveChainHeight:
  maybeProgress === null || typeof maybeProgress.validated_active_chain_height !== "number"
    ? null
    : maybeProgress.validated_active_chain_height,
```

Add fixture coverage where sync progress is available but `validated_active_chain_height` is missing; the JSON and Markdown should report it unavailable, not `0` or the connected height.

### WR-04: Phase 72 Checker Has A False-Positive Evidence Assertion

**File:** `scripts/check-phase72-observability-evidence.ts:369`
**Issue:** `verifyStatusSurfaces` joins all status-surface test files and requires the string `evidence_verdict`. The only matching status-surface occurrence is the `get_blockchain_info_does_not_expose_phase72_support_fields` forbidden-field assertion in `packages/open-bitcoin-rpc/src/dispatch/tests.rs:945`. That means the checker can pass because a baseline-compatibility test says the field must not be exposed, not because the intended support/evidence surface positively carries verdict evidence.
**Fix:** Move `evidence_verdict` out of the status-surface positive needle list. Check support verdict evidence in `SUPPORT_FILES`, and keep baseline exclusion as an explicit negative assertion against `getblockchaininfo` output tests or production RPC code.

```typescript
// In verifyStatusSurfaces: remove "evidence_verdict" from positive status needles.
// In verifySupportEvidence: require "verdict" / "SupportEvidenceVerdict" in support evidence files.
// In a separate baseline guard: require the getblockchaininfo test to forbid "evidence_verdict".
```

---

_Reviewed: 2026-06-13T20:26:54Z_
_Reviewer: the agent (gsd-code-reviewer)_
_Depth: standard_
