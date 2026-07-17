---
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: 126-2026-07-17T20-08-24
generated_at: 2026-07-17T20:13:44.089Z
---

# Phase 126: Compact Relay Residual Hardening - Context

**Gathered:** 2026-07-17
**Status:** Ready for planning
**Mode:** Yolo

<domain>
## Phase Boundary

Close the six approved v2.1 residual-hardening requirements: remove the production-capable empty-candidate compact receive bypass, generate outbound compact nonces from a Knots-aligned randomized shell source with deterministic test injection, extend exact parity and regression evidence across those seams, and reconcile the canonical milestone corpus to a fresh archive decision only after lifecycle-valid verification passes.

This phase does not expand compact-relay scope, add public relay defaults, activate package or filter serving, require public-network verification, or reopen the Phase 114 reconstruction and Phase 118 announcement-policy decisions.

</domain>

<decisions>
## Implementation Decisions

### Production Receive Invariant

- **D-01:** The invariant is that production-capable compact receive explicitly supplies a snapshot of live mempool and bounded extra candidates. The slices may legitimately be empty when both live sources are empty; non-emptiness is not the invariant.
- **D-02:** Make generic `PeerManager::handle_message` compact-block dispatch fail closed instead of constructing `CompactBlockReceiveFacts::default()`. A `WireNetworkMessage::CompactBlock` reaching the generic dispatcher must produce a stable adapter-routing error or equivalent typed failure that cannot be mistaken for peer misbehavior.
- **D-03:** Keep `ManagedPeerNetwork` as the authoritative production shell for both `receive_message` and `receive_sync_message`. It must continue snapshotting the current mempool and bounded `CompactExtraTxnBuffer`, then pass explicit facts to `handle_compact_block_download`.
- **D-04:** Preserve focused pure tests through direct factful APIs or a clearly named test-only empty-facts helper. Remove `Default` from `CompactBlockReceiveFacts` if that is the smallest structural guard against accidental production fallback.
- **D-05:** Do not inject a mempool provider into `PeerManager` or couple `open-bitcoin-network` to `open-bitcoin-mempool`; retain the Phase 114 iterator/slice boundary and the functional-core/imperative-shell split.

### Compact-Block Nonce Source

- **D-06:** Keep `build_compact_block_payload(block, nonce)` pure. Randomness belongs in the `open-bitcoin-node` announcement shell, matching the Knots boundary where `net_processing.cpp` supplies `FastRandomContext().rand64()` to the compact payload constructor.
- **D-07:** Use a call-scoped system-entropy adapter for a fresh production `u64` compact nonce. Prefer the already workspace-used `getrandom` crate over a larger stateful RNG dependency; add the direct Cargo and Bazel dependency only where the node shell needs it.
- **D-08:** Acquire entropy only for `CompactAnnouncementAction::AnnounceCompactBlock`. Headers, inventory, and suppression paths must not consume the compact nonce source.
- **D-09:** Provide a narrow deterministic injection seam for fixed and failing nonce sources in tests without storing RNG state on `ManagedPeerNetwork` or changing the pure consensus builder.
- **D-10:** Entropy failure must fall back through the existing typed headers/inventory behavior or suppress safely. It must never emit a compact block, increment `compact_announced_count`, or record `CompactAnnounced`.

### Evidence And Final Reconciliation

- **D-11:** Use a staged Phase 126 closeout: runtime/parity candidate state, lifecycle-valid verified promotion, then archive-ready projection. Keep the canonical audit non-passed, all six Phase 126 requirements pending, and archival routing absent until runtime, parity, deterministic checker, lifecycle, and full-verifier gates pass.
- **D-12:** Add focused deterministic regressions for the fail-closed generic receive path, factful live receive path, randomized production nonce boundary, deterministic/failing nonce injection, and achieved-effect evidence. Include mutation coverage in a Phase 126 Bun checker and keep default verification public-network-free.
- **D-13:** Update exact parity index entries and source breadcrumbs for the receive and announcement seams, citing the pinned Knots `PartiallyDownloadedBlock::InitData`, compact extra-transaction, `CBlockHeaderAndShortTxIDs`, and `FastRandomContext().rand64()` anchors.
- **D-14:** Extend the Phase 124 closeout guard to recognize legal Phase 126 intermediate and final states without weakening earlier Phase 124 or Phase 125 evidence. Continue using the generic active-milestone verification-orphan checker for lifecycle-valid requirement coverage.
- **D-15:** Only after Phase 126 verification is lifecycle-valid and the full default `bash scripts/verify.sh` contract passes may `CMP-05`, `RCN-02`, `RCN-03`, `GOV-04`, `BOUND-01`, and `HARD-05` become complete, the canonical `.planning/v2.1-MILESTONE-AUDIT.md` be refreshed to `passed`, and `/gsd-complete-milestone v2.1` become the sole primary route.
- **D-16:** If the fresh audit finds a genuine remaining gap, keep the audit non-passed and archive routing blocked. Do not hide the gap, split ownership into a competing active audit, or create Phase 127 merely to avoid the staged Phase 126 lifecycle.

### Folded Todos

No pending todos matched Phase 126.

### the agent's Discretion

The planner may choose the exact typed routing error, the test-only fact constructor name, the nonce-source function signature, the precise headers/inventory fallback selection on entropy failure, and the plan split between runtime hardening, parity/checker work, and final metadata promotion. Prefer the smallest API changes, no stored RNG state, explicit achieved-effect evidence, targeted planning-file edits, and deterministic Arrange/Act/Assert regressions.

</decisions>

<canonical-refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Project Rules And Phase Contract

- `AGENTS.md` — repo-local GSD workflow, Rust verification, parity breadcrumbs, generated artifact, and timing rules.
- `AGENTS.bright-builds.md` — managed Bright Builds workflow and cross-cutting standards.
- `standards-overrides.md` — local exceptions; no substantive override currently applies.
- `standards/core/architecture.md` — functional-core/imperative-shell and boundary parsing rules.
- `standards/core/code-shape.md` — early-return, optional naming, and module-size guidance.
- `standards/core/testing.md` — focused Arrange/Act/Assert test requirements.
- `standards/core/verification.md` — sync-first and repo-native verification gates.
- `standards/languages/rust.md` — Rust module, invariant, optional-name, and adapter rules.
- `.planning/ROADMAP.md` § Phase 126 — fixed goal, dependency, six requirement IDs, and success criteria.
- `.planning/REQUIREMENTS.md` — normative `CMP-05`, `RCN-02`, `RCN-03`, `GOV-04`, `BOUND-01`, and `HARD-05` definitions and ownership.
- `.planning/PROJECT.md` — bounded v2.1 claim and archive-blocked current state.
- `.planning/STATE.md` — active Phase 126 route and milestone continuity.
- `.planning/v2.1-MILESTONE-AUDIT.md` — authoritative residual receive, nonce, parity, and closeout findings.

### Existing Open Bitcoin Decisions And Runtime Seams

- `.planning/phases/114-compact-block-reconstruction-from-mempool-state/114-CONTEXT.md` — explicit mempool/extra iterator boundary and typed reconstruction outcomes.
- `.planning/phases/118-outbound-compact-block-announcement-wiring/118-CONTEXT.md` — pure payload builder, action honor, fallback, and achieved-effect evidence decisions.
- `.planning/phases/119-compact-receive-mempool-candidate-injection/119-CONTEXT.md` — managed-shell candidate injection and bounded extra-buffer decisions.
- `.planning/phases/124-milestone-closeout-reconciliation/124-CONTEXT.md` — staged evidence-first metadata and audit reconciliation precedent.
- `.planning/phases/125-compact-download-verification-traceability-closure/125-CONTEXT.md` — lifecycle-valid verification ownership and generic orphan-checker decisions.
- `packages/open-bitcoin-node/src/network.rs` — authoritative managed receive paths and current hash-derived compact nonce.
- `packages/open-bitcoin-node/src/network/compact_receive_candidates.rs` — live mempool/extra snapshots and factful compact receive adapter.
- `packages/open-bitcoin-network/src/peer/message_dispatch.rs` — audited generic default-empty compact dispatch path.
- `packages/open-bitcoin-network/src/peer/compact_download_state.rs` — `CompactBlockReceiveFacts` and factful reconstruction entrypoint.
- `packages/open-bitcoin-network/src/peer.rs` — typed announcement actions and nonce-consuming payload emission.
- `packages/open-bitcoin-consensus/src/compact_block_build.rs` — pure compact payload builder.
- `packages/open-bitcoin-node/Cargo.toml` — direct node dependencies.
- `packages/open-bitcoin-node/BUILD.bazel` — Bazel dependency surface for the node crate.

### Pinned Bitcoin Knots Anchors

- `packages/bitcoin-knots/src/net_processing.cpp` — `FastRandomContext().rand64()` compact announcement nonce generation, compact reconstruction candidate supply, and recent/extra transaction lifecycle.
- `packages/bitcoin-knots/src/blockencodings.cpp` — nonce-consuming `CBlockHeaderAndShortTxIDs` construction and `PartiallyDownloadedBlock::InitData`.
- `packages/bitcoin-knots/src/blockencodings.h` — randomized nonce contract and compact payload types.
- `packages/bitcoin-knots/src/net_processing.h` — bounded compact extra-transaction constants and state ownership.
- `packages/bitcoin-knots/test/functional/p2p_compactblocks.py` — compact relay behavior reference coverage.

### Parity, Checkers, And Final Verification

- `docs/parity/index.json` — machine-readable v2.1 parity evidence roots.
- `docs/parity/source-breadcrumbs.json` — source-level Knots anchor ownership.
- `docs/parity/catalog/p2p.md` — human-readable compact relay parity catalog.
- `docs/parity/catalog/mempool-policy.md` — mempool lifecycle and reconstruction-candidate parity catalog.
- `scripts/check-phase124-milestone-closeout-reconciliation.ts` — staged closeout guard that must recognize Phase 126 legal states.
- `scripts/check-phase124-milestone-closeout-reconciliation.test.ts` — existing closeout mutation coverage.
- `scripts/check-active-milestone-verification-traceability.ts` — generic lifecycle-valid verification-orphan gate.
- `scripts/check-active-milestone-verification-traceability.test.ts` — generic traceability checker coverage.
- `scripts/verify.sh` — required deterministic repository verification contract.

</canonical-refs>

<code-context>
## Existing Code Insights

### Reusable Assets

- `ManagedPeerNetwork::handle_compact_block_receive` already snapshots live mempool and bounded extras and passes explicit `CompactBlockReceiveFacts`.
- `PeerManager::handle_compact_block_download` already exposes the factful pure/network-core reconstruction seam used by focused tests.
- `build_compact_block_payload(block, nonce)` already accepts an injected nonce and therefore needs no randomness or signature redesign.
- `block_relay_evidence::compact_announce_evidence_reason` and post-emission recording already provide the achieved-effect evidence boundary that entropy failure must preserve.
- Phase 124 and active-milestone traceability checkers provide reusable parsing, staged-state, mutation-test, and lifecycle-validation patterns.

### Established Patterns

- Node adapters own mempool, clocks, storage, network effects, and future randomness; network/consensus crates consume explicit typed data.
- Production receive paths branch on `WireNetworkMessage::CompactBlock` before generic dispatch.
- Announcement evidence is recorded only after the actual outbound wire message is known.
- Repo-owned substantial automation is Bun/TypeScript and default verification remains deterministic and public-network-free.
- Requirement promotion follows passed verification and exact lifecycle provenance, not implementation intent or stale audit prose.

### Integration Points

- `ManagedPeerNetwork::{receive_message, receive_sync_message}` and `PeerManager::handle_message` define the production routing invariant.
- `ManagedPeerNetwork::announce_block` is the shell boundary for production nonce acquisition and emission fallback.
- Node Cargo/Bazel metadata must expose any direct system-entropy dependency consistently.
- Parity index, source breadcrumbs, Phase 126 checker wiring, Phase 124 closeout stages, active requirements, ROADMAP, PROJECT, STATE, MILESTONES, and the canonical audit must converge only after clean verification.

</code-context>

<specifics>
## Specific Ideas

- Treat an actually empty live mempool plus empty bounded-extra buffer as a valid explicitly supplied snapshot.
- Prefer a call-scoped `getrandom`-backed `u64` over storing a clone-sensitive RNG on `ManagedPeerNetwork`.
- On entropy failure, preserve the existing headers/inventory fallback vocabulary and prove no false compact-announcement evidence.
- Keep one canonical v2.1 audit refreshed in place and retain a concise resolved-debt ledger for the Phase 119 and Phase 118 findings.

</specifics>

<deferred>
## Deferred Ideas

- A compile-time routed non-compact message wrapper may be reconsidered if multiple production adapters appear and the wider migration becomes worthwhile.
- A stored or seeded RNG port may be reconsidered if runtime randomness consumers multiply or compact construction becomes high-volume.
- Package relay, bloom/filter serving, compact filters, public relay defaults, public-network CI, archive-node claims, production full-node readiness, and production-funds wallet use remain outside v2.1.

</deferred>

***

*Phase: 126-compact-relay-residual-hardening*
*Context gathered: 2026-07-17*
