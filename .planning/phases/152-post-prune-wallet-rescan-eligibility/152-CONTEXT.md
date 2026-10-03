---
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: 152-2026-10-03T00-26-52
generated_at: 2026-10-03T00:28:00Z
---

# Phase 152: Post-Prune Wallet Rescan Eligibility - Context

**Gathered:** 2026-10-02 CDT
**Status:** Ready for planning
**Mode:** Yolo

<domain>
## Phase Boundary

Close INT-02 and SNAP-01: node chunk/resume and durable RPC wallet rescans
must check creating-payload eligibility for every entry admitted to their
full wallet replacement, even before the requested start. Real paired
pruning, resume, and same-datadir reopen must preserve this contract.
Leftover snapshots remain non-authoritative. Automatic retention is Phase
153; this phase changes no pruning policy or public readiness claims.
</domain>

<decisions>
## Implementation Decisions

### Full replacement eligibility

- **D-01:** Preserve full replacement semantics. A requested midrange or
  chunk controls progress and stop height, not permission to admit older
  entries without checking their creating payloads. Carry forward Phase
  146 D-04 and D-08 together.
- **D-02:** Check the requested chunk/range payloads and the creating
  payload of every candidate wallet entry through the replacement height.
  Check older creating heights even when outside the range. Deduplicate
  probes where useful. Do not require unrelated older payloads merely
  because unrelated durable coins exist.
- **D-03:** Use one shared eligibility contract for both durable adapters.
  Wallet matching/selection remains pure; Fjall payload probes stay in the
  shell. Durable coins best-block and coins remain chain truth; never
  fall back to the leftover snapshot blob.

### Refusal and durable evidence

- **D-04:** Missing creating metadata, absent payloads, and payload read
  errors fail closed before persisting any replacement wallet. Preserve
  prior persisted wallet balances, UTXOs, and tip/progress on refusal.
- **D-05:** Save a durable Failed rescan-job record for eligibility
  failures, including payload-probe errors. Keep existing error categories
  where possible; record the failed boundary and height/hash when known,
  without raw storage paths or snapshot fallback. Propagate persistence
  errors visibly.
- **D-06:** Re-evaluate eligibility on every resumed chunk and after
  reopen; prior successful probes are not permanent permission after
  prune. A failed later chunk preserves the last successful wallet state.

### Proof and scope

- **D-07:** Runtime regressions must use real paired block/undo deletion,
  retained durable coins, and have-pruned evidence. Cover older creating
  heights outside the range, in-range absence, chunk resume/reopen,
  conflicting leftover snapshots, and retained-payload success controls
  in both adapters. Source-string checks cannot substitute for behavior.
- **D-08:** Reuse existing registry/job and storage interfaces; add no
  dependencies or broad wallet schema redesign. Investigate the audit's
  InterruptedTwoHeads and probe-error notes at touched scan seams;
  close directly related authority/refusal holes without expanding into
  prune retention or repair work.
- **D-09:** Run default `bash scripts/verify.sh`, including managed
  checks, coverage and Bazel smoke. Register new first-party Rust files
  in parity breadcrumbs and refresh relevant parity/operator docs.
  Commits and push wait for clean verification, per this strict wrapper.

### Agent's Discretion

Choose the smallest shared pure/shell seam, typed refusal shape, fixture
organization, and bounded probe strategy consistent with these decisions.
No pending todos matched this phase.
</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

- `.planning/ROADMAP.md` — Phase 152 scope, success criteria, dependency.
- `.planning/REQUIREMENTS.md` — SNAP-01 and deferred claims.
- `.planning/PROJECT.md` — pinned Knots baseline and functional core.
- `.planning/v2.4-MILESTONE-AUDIT.md` — INT-02 and Phase 146 residuals.
- `.planning/phases/146-wallet-leftover-snapshot-cutover/146-CONTEXT.md`
  — D-04 creating-payload eligibility and D-08 chunk refusal.
- `packages/open-bitcoin-wallet/src/wallet/scan.rs` — pure full replacement.
- `packages/open-bitcoin-node/src/sync/wallet_rescan.rs` — chunk/resume shell.
- `packages/open-bitcoin-rpc/src/context/rescan.rs` — durable RPC range shell.
- `packages/open-bitcoin-node/src/storage/fjall_store.rs` — durable authority.
- `packages/bitcoin-knots/src/wallet/wallet.cpp` — pinned rescan behavior.
- `packages/bitcoin-knots/src/wallet/rpc/transactions.cpp` — range refusals.
- `docs/parity/source-breadcrumbs.json` — first-party source anchors.
- `AGENTS.md`, `AGENTS.bright-builds.md`, `standards-overrides.md`,
  `standards/core/architecture.md`, `standards/core/code-shape.md`,
  `standards/core/testing.md`, `standards/core/verification.md`,
  `standards/languages/rust.md` — repository and standards constraints.
</canonical_refs>

<code_context>
## Existing Code Insights

- `WalletRescanRuntime` already owns bounded persisted job progress;
  its range-only gate misses coins created before the current chunk.
- Durable RPC uses the same registry but separately gates only its range.
- `Wallet::rescan_chainstate` replaces the complete UTXO set from a pure
  snapshot. A shared selection/eligibility seam can prevent divergence.
- Real paired deletion, durable coins, registry snapshots and leftover
  snapshot fixtures already exist in node/RPC tests.
</code_context>

<specifics>
## Specific Ideas

Prune creating height h, keep the matching coin durable, and scan from
h+1 through a retained tip. Both adapters must refuse, leave the previous
wallet intact, and persist Failed evidence even after datadir reopen.
The same scan succeeds when that creating payload is retained.
</specifics>

<deferred>
## Deferred Ideas

Automatic target retention is Phase 153. Incremental wallet scanning,
snapshot deletion, archive serving, assumeutxo, public defaults, repair,
and production-funds claims remain outside this closure.
</deferred>
