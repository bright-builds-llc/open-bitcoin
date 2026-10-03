---
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: 153-2026-10-03T04-01-54
generated_at: 2026-10-03T04:04:26Z
---

# Phase 153: Automatic Prune Retention Integration - Context

**Gathered:** 2026-10-02 CDT
**Status:** Ready for planning
**Mode:** Yolo

<domain>
## Phase Boundary

Close INT-01 and PRUN-01/PRUN-02: an automatic target of at least 550 MiB
must drive real paired payload deletion during ordinary durable lifecycle
activity. Preserve the existing pure planner, single durable owner, locks,
288-block window, network prune-after threshold, recovery, cache/undo
cleanup, earned labels and Phase 152 wallet eligibility.
</domain>

<decisions>
## Implementation Decisions

### Authoritative usage and target

- **D-01:** Measure retained block and encoded undo payload value bytes
  from the actual durable store. Include protected/recent and nonactive
  payloads in total usage; candidate sizes refer only to current active
  height/hash pairs. Count either present mate and omit both-absent pairs.
  Do not use snapshot size, file allocation, invented averages, or decoded
  block size as the retained-byte authority.
- **D-02:** Pass configured mode, measured usage, current active tip,
  injected network prune-after height and current durable locks to the
  existing pure automatic planner. Retain its legal minimum and overflow
  behavior. Protected or nonactive bytes can keep usage above target;
  the target never permits deleting protected history.

### Ordinary lifecycle and authority

- **D-03:** Reuse the existing Periodic worker and Always shutdown flush
  path through `ManagedNetworkHandle::flush_coins`; add no independent
  deletion worker or alternate unlink owner. Automatic retention must be
  able to run when coins policy has no write due, using existing nonempty
  prune-plan semantics. A nonempty automatic plan must request the existing
  full coins/chain-metadata checkpoint, following Knots' prune-triggered
  flush rule. Keep `FlushMode::None`, disabled/manual-only,
  under-target, short-chain and fully protected cases as no-op gates.
- **D-04:** Assemble/apply against current authoritative chain facts and
  durable locks. Serialize lock updates with automatic planning/deletion,
  or use an equally strong shared guard; a stale lock snapshot must not
  allow deletion across a newly committed lock.
- **D-05:** Explicit prune configuration with a datadir must select the
  durable lifecycle even when network activation is disabled. Preserve
  opt-in networking and existing startup recovery before readiness.

### Failures and evidence

- **D-06:** Propagate accounting/storage errors and refuse without a
  fabricated plan. Apply only through paired durable unlink. Evict deleted
  hashes and forget undo on successful deletes even if a later flush fails;
  counters and have-pruned remain earned by committed deletes only.
- **D-07:** Reopen must finish or refuse interrupted pruning under current
  durable lock rules. Subsequent cycles must remeasure retained data.
  Automatic deletion must preserve serving labels and wallet eligibility;
  never introduce leftover-snapshot fallback.
- **D-08:** Behavioral tests must drive the production automatic lifecycle
  against real storage with measured bytes exceeding a legal 550 MiB
  target. Cover protected windows/locks, prune-after, no-op modes, repeated
  activity, later-flush error, restart and operator/wallet regressions.
  Keep fixtures hermetic and resource-conscious; source-string checks or
  synthetic production accounting overrides cannot prove this closure.
- **D-09:** Register new Rust source/test paths in parity breadcrumbs,
  document logical Fjall-byte versus Knots flat-file accounting differences,
  refresh contributor evidence, review and simplify the touched seams,
  then run default `bash scripts/verify.sh` including Bazel and coverage.
  Re-audit integration after closure; milestone archival stays separate.
- **D-10:** This strict wrapper defers all commits and push until clean
  phase verification and lifecycle validation. Earlier workflow commit
  steps become pending finalization records; never bypass hooks.

### Agent's Discretion

Choose the smallest store-accounting interface, bounded measurement
strategy, lock synchronization mechanism and fixture organization that
honor these decisions. No pending todos matched the phase.
</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

- `.planning/ROADMAP.md`, `.planning/REQUIREMENTS.md`, `.planning/PROJECT.md`
  — closure scope, PRUN-01/PRUN-02 and deferred capabilities.
- `.planning/v2.4-MILESTONE-AUDIT.md` — INT-01 production-consumer gap.
- `.planning/phases/147-pure-prune-policy-and-lock-windows/147-CONTEXT.md`
  — pure planning and legal retention windows.
- `.planning/phases/148-fjall-payload-unlink-and-have-pruned/148-CONTEXT.md`
  — paired delete, recovery and cache/undo ownership.
- `.planning/phases/152-post-prune-wallet-rescan-eligibility/152-CONTEXT.md`
  — wallet creating-payload eligibility after actual deletion.
- `packages/open-bitcoin-chainstate/src/prune/plan.rs` — automatic planner.
- `packages/open-bitcoin-node/src/chainstate/flush_lifecycle.rs`
  — production durable flush owner and existing prune-plan execution.
- `packages/open-bitcoin-node/src/chainstate/fjall_store.rs`
  — production store adapter.
- `packages/open-bitcoin-node/src/network/runtime_authority.rs`
  and `packages/open-bitcoin-node/src/network/runtime_authority/prune_flush.rs`
  — ordinary flush and cache eviction authority.
- `packages/open-bitcoin-node/src/storage/fjall_store/prune.rs`
  — durable paired unlink, locks and counters.
- `packages/open-bitcoin-rpc/src/bin/open_bitcoind/coins_flush.rs`
  and `packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs`
  — daemon lifecycle cadence and durable activation.
- `packages/open-bitcoin-rpc/src/dispatch/prune.rs`
  — manual prune and durable lock mutation seams.
- `packages/open-bitcoin-rpc/src/context/prune.rs`
  — durable lock mutation and network prune-after mapping.
- `packages/bitcoin-knots/src/node/blockstorage.cpp`
  and `packages/bitcoin-knots/src/validation.cpp` — pinned prune anchors.
- `docs/parity/source-breadcrumbs.json` and `docs/parity/index.json`
  — source anchors and intentional behavior differences.
- `AGENTS.md`, `AGENTS.bright-builds.md`, `standards-overrides.md`,
  `standards/core/architecture.md`, `standards/core/code-shape.md`,
  `standards/core/testing.md`, `standards/core/verification.md`,
  `standards/languages/rust.md` — required local and managed rules.
</canonical_refs>

<code_context>
## Existing Code Insights

The daemon already ticks Periodic every second and Always on shutdown.
The coins-write jitter is separate from these ticks. Ordinary `flush_coins`
currently injects an empty prune plan; the existing flush lifecycle can
apply a nonempty plan when coins policy returns no write. Cache eviction
already uses deletion receipts on success and failure. RPC durable lock
writes currently use a cloned store outside the network authority mutex,
so lock serialization needs explicit treatment.
</code_context>

<specifics>
## Specific Ideas

Configure automatic 550 MiB, retain measured eligible payloads above the
target, trigger ordinary Periodic activity and observe real paired deletes,
have-pruned, earned counters, cache eviction and successful reopen. Repeat
with durable locks and recent heights to prove those bytes are retained.
</specifics>

<deferred>
## Deferred Ideas

Temporary IBD targets, archive serving, assumeutxo, BIP37, automatic repair,
public defaults, public-network release gates, production readiness and
production-funds wallet claims remain deferred.
</deferred>
