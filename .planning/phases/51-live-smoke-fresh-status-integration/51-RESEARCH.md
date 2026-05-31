---
generated_by: gsd-phase-researcher
lifecycle_mode: yolo
phase_lifecycle_id: 51-2026-05-31T21-21-57
generated_at: 2026-05-31T21:25:05.455Z
---

# Phase 51 Research: Live Smoke Fresh Status Integration

**Confidence:** HIGH for the local integration path and deterministic test
coverage; LOW for live public-network outcome because this phase does not
rerun public-mainnet evidence by default.

## Findings

### Audit Gap

- `.planning/v1.3-MILESTONE-AUDIT.md` identifies G-01 as blocking because the
  live-smoke runner polls `open-bitcoin-cli getblockchaininfo`, parses that as
  `RuntimeMetadataJson`, and hardcodes `outboundPeers: 0` and
  `phase: "rpc_getblockchaininfo"`.
- The same audit shows the mismatch directly: Phase 50 selected report
  snapshots had `lifecycle=synced` and `phase=rpc_getblockchaininfo`, while
  final durable status reported `lifecycle=active` and `phase=steady_state`.

### Fresh Status Path

- `packages/open-bitcoin-rpc/src/method.rs` registers
  `openbitcoinsyncstatus` as an Open Bitcoin extension method.
- `packages/open-bitcoin-rpc/src/dispatch/node.rs` maps that method to
  `open_bitcoin_sync_status`, which returns `OpenBitcoinSyncControlResponse`
  with fresh daemon runtime metadata.
- `packages/open-bitcoin-rpc/src/context.rs` implements
  `daemon_sync_status()` through the daemon sync-control path, avoiding the
  stored `ManagedRpcContext::maybe_durable_sync_state()` snapshot that
  `getblockchaininfo` uses.

### Existing Test Surface

- `scripts/test-run-live-mainnet-smoke.sh` already runs the smoke script with a
  mock daemon, mock status command, mock final status, and mocked network
  preflight fixture.
- This test is the right deterministic proof surface because it can validate
  report JSON/Markdown without public-network access and without adding
  `run-live-mainnet-smoke` to `scripts/verify.sh`.

### Documentation And Evidence

- `docs/operator/runtime-guide.md` still says the smoke runner polls
  `open-bitcoin-cli getblockchaininfo`.
- `.planning/phases/50-public-mainnet-progress-evidence-closeout/50-UAT.md`
  still records selected snapshots with `phase=rpc_getblockchaininfo`, which is
  exactly the historical mismatch Phase 51 needs to amend.
- `docs/parity/checklist.md`, `docs/parity/index.json`, and
  `docs/parity/release-readiness.md` should remain concise parity roots, not
  generated report stores.

## Recommended Approach

1. Change the live-smoke status command to call
   `openbitcoinsyncstatus`.
2. Replace the `getblockchaininfo` response parser with a runtime metadata
   parser that accepts either `{ "metadata": ... }` or raw metadata.
3. Derive `SyncStatusSnapshot` from `maybe_sync_state.sync`,
   `maybe_sync_state.peers`, `sync_control`, and
   `updated_at_unix_seconds`.
4. Update the shell regression fixtures to emit fresh sync-control metadata and
   assert status command method, lifecycle, phase, outbound peers, paused
   state, progress deltas, and no-progress diagnosis.
5. Amend operator docs and Phase 50/parity evidence text to state that Phase 51
   closes the stale snapshot gap without checking generated live reports into
   git.

## Verification Plan

- `bash scripts/test-run-live-mainnet-smoke.sh`
- `bun run scripts/check-v1.3-release-boundaries.ts`
- `rg -n "getblockchaininfo|openbitcoinsyncstatus|rpc_getblockchaininfo|fresh-status" scripts/run-live-mainnet-smoke.ts scripts/test-run-live-mainnet-smoke.sh docs/operator/runtime-guide.md .planning/phases/50-public-mainnet-progress-evidence-closeout/50-UAT.md docs/parity/checklist.md docs/parity/index.json docs/parity/release-readiness.md`
- Rust pre-commit sequence from `AGENTS.md`.
- `bash scripts/verify.sh`

## Risks

- The JSON shape differs between baseline `open-bitcoin-cli` RPC method output
  (`{ metadata: ... }`) and operator `open-bitcoin sync status --format json`
  output (raw metadata). Mitigate with a small parser boundary that normalizes
  both shapes.
- The live public network may still fail to produce progress. This phase should
  not overclaim live progress; it proves the report now diagnoses through fresh
  daemon status.
