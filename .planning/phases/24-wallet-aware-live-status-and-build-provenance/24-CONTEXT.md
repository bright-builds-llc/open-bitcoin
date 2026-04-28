---
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: 24-2026-04-28T17-43-00
generated_at: 2026-04-28T17:43:00.000Z
---

# Phase 24: Wallet-Aware Live Status and Build Provenance - Context

## Phase Boundary

**Goal:** Keep live status and dashboard snapshots truthful when wallet
selection is missing or ambiguous, and surface real build provenance through
the shared operator status model.

**Success criteria:**
1. Live status and dashboard snapshots no longer collapse to
   `NodeRuntimeState::Unreachable` when wallet selection is missing or
   ambiguous.
2. Wallet selection issues are surfaced as wallet-specific unavailable or
   diagnostic data while node reachability remains accurate.
3. Build provenance is populated in shared status snapshots when available and
   rendered through operator-facing status and dashboard surfaces.
4. Targeted verification covers zero-wallet, multiwallet, selected-wallet, and
   build-provenance runtime paths.

**Out of scope:**
- Migration source-selection hardening owned by Phase 25.
- Milestone evidence reconciliation owned by Phase 26.
- A new `--wallet` flag or other explicit wallet-selection surface for
  `open-bitcoin status` or `open-bitcoin dashboard`.
- Full multiwallet lifecycle parity beyond the shipped wallet-routed RPC subset.

## Requirements In Scope

- `OBS-01`: Operator status reports the shared node, wallet, service, health,
  and build surfaces truthfully.
- `OBS-02`: Machine-readable status output stays stable and truthful for
  automation.
- `WAL-05`: Wallet selection remains compatible with the shipped
  `-rpcwallet`-style routing and selection model.
- `DASH-01`: Dashboard continues to project live shared-status truth rather
  than a forked or misleading runtime view.

## Canonical References

- `.planning/ROADMAP.md` — Phase 24 goal, requirements, and success criteria.
- `.planning/REQUIREMENTS.md` — observability, dashboard, and wallet
  traceability ledger.
- `.planning/v1.1-MILESTONE-AUDIT.md` — blocker `INT-02` and build-provenance
  follow-up evidence.
- `AGENTS.md` and `AGENTS.bright-builds.md` — repo-native verification contract
  plus the sync-first and visible-build-provenance workflow rules.
- Bright Builds canonical standards pages:
  - `standards/core/architecture.md`
  - `standards/core/code-shape.md`
  - `standards/core/verification.md`
  - `standards/core/testing.md`
  - `standards/languages/rust.md`
- `packages/open-bitcoin-cli/src/operator/status.rs` — shared operator status
  collection and snapshot construction.
- `packages/open-bitcoin-cli/src/operator/status/http.rs` — HTTP-backed status
  RPC adapter.
- `packages/open-bitcoin-cli/src/operator/runtime.rs` — status and dashboard
  runtime wiring.
- `packages/open-bitcoin-cli/src/operator/status/render.rs` and
  `packages/open-bitcoin-cli/src/operator/dashboard/model.rs` — operator-facing
  build and wallet projection surfaces.
- `packages/open-bitcoin-rpc/src/context/wallet_state.rs` — wallet selection
  and RPC error semantics for root versus wallet-scoped calls.
- `packages/open-bitcoin-node/src/status.rs` — shared `BuildProvenance` and
  status model contract.

## Repo Guidance That Materially Informs This Phase

- Use `bash scripts/verify.sh` as the repo-native verification contract before
  git finalization.
- Keep business logic and projection decisions in pure helpers where practical,
  with runtime adapters staying thin.
- Prefer focused hermetic tests over environment-coupled operator runs.
- Surface build provenance in a visible operator surface and show unavailable
  reasons instead of silently omitting fields.

## Current State

- `collect_live_status_snapshot()` currently calls `getwalletinfo` and
  `getbalances` through the root RPC endpoint and treats any wallet-scope
  failure like node unreachability.
- `HttpStatusRpcClient::call()` only extracts the `result` field and therefore
  loses typed RPC error details such as `WalletNotFound` and
  `WalletNotSpecified`.
- The operator wallet workflow already knows how to prefer a selected or sole
  managed wallet from the local durable registry, but the status/dashboard path
  does not reuse that idea.
- Shared status snapshots always use `BuildProvenance::unavailable()`, so
  status and dashboard renderers never surface real commit, build time, target,
  or profile metadata.

## Decisions

1. **Node reachability is determined only from node-scoped RPC health.**
   If `getnetworkinfo`, `getblockchaininfo`, and `getmempoolinfo` succeed, the
   node remains `running` even when wallet-specific RPC calls fail.
2. **Local wallet registry state is used only when it provides a confident
   route.**
   Status may select a sole wallet or a recorded selected wallet, but an empty
   local registry must not be treated as proof that the remote node has no
   wallet loaded.
3. **Wallet-specific issues degrade only wallet fields.**
   Wallet selection ambiguity or wallet-not-loaded responses should mark the
   wallet portion of the snapshot unavailable and add wallet-scoped diagnostics
   instead of collapsing the entire snapshot to unreachable.
4. **Build provenance comes from compile-time metadata with graceful fallback.**
   A Cargo build script can emit commit, build time, target, and profile for
   Cargo builds, while `option_env!` keeps non-Cargo paths such as Bazel smoke
   builds working with unavailable reasons.
5. **Status and dashboard both consume the same repaired snapshot.**
   The phase should avoid dashboard-only logic forks and instead repair the
   shared collector plus shared build rendering.

## Key Files and Likely Change Surfaces

- `packages/open-bitcoin-cli/src/operator/status.rs`
- `packages/open-bitcoin-cli/src/operator/status/http.rs`
- `packages/open-bitcoin-cli/src/operator/status/wallet.rs`
- `packages/open-bitcoin-cli/src/operator/runtime.rs`
- `packages/open-bitcoin-cli/src/operator/status/render.rs`
- `packages/open-bitcoin-cli/src/operator/dashboard/model.rs`
- `packages/open-bitcoin-cli/build.rs`
- `packages/open-bitcoin-cli/src/operator/status/tests.rs`
- `.planning/ROADMAP.md`
- `.planning/REQUIREMENTS.md`

## Risks

- An empty or missing local Fjall store is not strong enough evidence to
  conclude that the live node has zero wallets, so the status runtime must not
  overfit to local state.
- JSON-RPC v2 returns HTTP 200 even when the response body contains an error, so
  the status adapter must inspect `error` payloads before trusting `result`.
- Build metadata injected through Cargo is not guaranteed under every build
  system, so the snapshot constructor must preserve unavailable reasons when the
  env vars are absent.

## Implementation Notes

- Reuse a small typed wallet-access enum so the runtime can distinguish root
  access, selected-wallet access, and locally-known ambiguity without illegal
  states.
- Keep wallet unavailability rendering inside shared wallet-status helpers so
  status and dashboard continue to stay aligned.
- Use focused status tests to prove the wallet-selection and build-provenance
  behavior instead of inventing a new end-to-end operator harness.
