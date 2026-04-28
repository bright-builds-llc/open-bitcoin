---
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: 24-2026-04-28T17-43-00
generated_at: 2026-04-28T17:43:00.000Z
---

# Phase 24 Discussion Log

## Prompt

Repair the Phase 24 audit gaps without widening the operator surface beyond the
roadmap goal: keep status and dashboard truthful when wallet selection is
ambiguous and surface real build provenance.

## Decisions Made

1. **Do not add a new status/dashboard wallet-selection flag in this phase.**
   The gap is about truthful live projection, not a new command surface. The
   phase should work with existing local selected-wallet metadata and current
   wallet-routed RPC behavior.

2. **Treat wallet failures as wallet failures, not node failures.**
   The node should only become `unreachable` when node-scoped RPC calls fail.
   Wallet-scope failures should degrade wallet fields and emit wallet
   diagnostics while preserving sync, mempool, and peer truth.

3. **Use local wallet registry state only when it is trustworthy.**
   A selected wallet or a sole wallet is actionable. An empty store is not
   enough evidence to infer that the live node has zero wallets, because the
   operator may be pointed at a healthy remote or fake RPC target.

4. **Parse JSON-RPC error payloads instead of relying on HTTP status.**
   The Open Bitcoin RPC server returns JSON-RPC v2 bodies with HTTP 200 even
   for method-level failures, so the status adapter must inspect the `error`
   object directly to recover `WalletNotFound` and `WalletNotSpecified`.

5. **Put build metadata generation in the CLI crate, not the shared node model.**
   The shared node model should remain data-only. The CLI runtime owns status
   and dashboard rendering, so it can populate the shared model from compile
   metadata before projection.

## Alternatives Considered

- **Add `--wallet` to `open-bitcoin status` and `open-bitcoin dashboard`.**
  Rejected for this phase because the roadmap goal is to make the current
  operator surface truthful, not broader.

- **Treat an empty local wallet registry as zero-wallet truth.**
  Rejected because the local store may be absent or unrelated to the RPC target,
  which would create false negatives in tests and remote-operator scenarios.

- **Populate build metadata inside `open-bitcoin-node`.**
  Rejected because that would couple the pure shared model crate to build-system
  details instead of letting the CLI runtime populate it at the shell boundary.

## Resulting Plan Shape

- Plan 01: wallet-aware live status routing and wallet-only degradation
- Plan 02: compile-time build provenance population and rendering
- Plan 03: verification, roadmap refresh, and requirements traceability closeout
