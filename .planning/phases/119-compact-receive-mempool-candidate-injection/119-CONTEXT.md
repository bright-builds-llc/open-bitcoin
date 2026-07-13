---
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: 119-2026-07-13T16-08-52
generated_at: 2026-07-13T16:09:36.781Z
---

# Phase 119: Compact Receive Mempool Candidate Injection - Context

**Gathered:** 2026-07-13
**Status:** Ready for planning
**Mode:** Yolo

<domain>
## Phase Boundary

Phase 119 closes the v2.1 audit gap for RCN-02, RCN-03, and GOV-04: live inbound `CompactBlock` receive must feed mempool and bounded extra candidates into reconstruction, and mempool-remove lifecycle must clear matching volatile partial compact state.

Today `WireNetworkMessage::CompactBlock` dispatch always calls `handle_compact_block_download` with `CompactBlockReceiveFacts::default()` (empty candidates/extras). Pure reconstruction and typed outcomes already exist from Phase 114/115, but the runtime path never supplies candidates, so reconstruction and lifecycle hooks cannot fire on live receive.

This phase wires that feed and lifecycle hook only. It must not schedule compact-download timeout expiration or escalate compact misbehavior (Phase 120), project block-relay metrics/logs through DurableSyncRuntime (Phase 121), enable package relay, bloom/filter or compact-filter serving, change public defaults, add public-network CI gates, claim archive-node behavior, claim production full-node readiness, or claim production-funds wallet safety.

</domain>

<decisions>
## Implementation Decisions

### Receive Candidate Supply Seam

- **D-01:** Inbound `CompactBlock` dispatch must stop always using `CompactBlockReceiveFacts::default()`. Live receive must supply mempool candidates and bounded extras into `handle_compact_block_download`.
- **D-02:** Keep `PeerManager` free of `open-bitcoin-mempool` coupling (Phase 114 D-08). Gather candidate/extra slices in the node shell (`ManagedPeerNetwork`) and pass `CompactBlockReceiveFacts` into the network download API — prefer intercepting `CompactBlock` in `receive_message` / `receive_sync_message` (or a focused helper) rather than baking mempool into `message_dispatch`.
- **D-03:** Prefer the smallest API change that makes non-empty facts reachable on the live path: e.g. call `handle_compact_block_download` directly from the shell for `CompactBlock`, or add a PeerManager entry that accepts facts without pulling mempool into the network crate. Empty-facts `handle_message` CompactBlock branch may become a test-only or deprecated path, but production receive must use the injected path.

### Mempool And Extra Sources

- **D-04:** Mempool candidates are the current mempool's `(Wtxid, Transaction)` view at receive time — shell adapts mempool iteration into the existing `CompactBlockReceiveFacts` slice shape.
- **D-05:** Bounded extras follow Knots-shaped recent/extra compact txn inputs (bounded buffer of recent or orphan-adjacent transactions suitable for reconstruction). Prefer a dedicated bounded extra buffer owned by the node shell over unbounded history or inventing package-relay surfaces. Exact buffer size/eviction policy is Claude's Discretion within a Knots-aligned bound.
- **D-06:** Candidate and extra collection must remain read-only relative to chainstate: no chainstate mutation from partial compact state (RCN-06 preserved).

### Mempool Removal Lifecycle Hook

- **D-07:** Hook `on_mempool_transaction_removed` (or the PeerManager-level forwarder over compact-download partial state) from mempool lifecycle when transactions leave the mempool — at minimum from `apply_connected_block_mempool_lifecycle` removals, and from other removal paths the shell already treats as mempool exits (evict/expire) when wtxid is available.
- **D-08:** Lifecycle hook clears matching volatile partial compact slots only. Do not activate package relay, bloom/filter serving, or compact filters. Do not schedule timeout ticks (Phase 120).

### Verification And Parity

- **D-09:** Runtime/unit tests must prove: (1) live CompactBlock receive with mempool candidates reconstructs or reports Ready/missing without empty-facts only, (2) collision, duplicate, and missing outcomes remain typed on the injected path, (3) mempool removal clears matching volatile slots via the lifecycle hook, (4) package/filter/public-default surfaces stay untouched.
- **D-10:** New or touched first-party Rust source/test files under `packages/open-bitcoin-*/src` or `packages/open-bitcoin-*/tests` require parity breadcrumbs and `docs/parity/source-breadcrumbs.json` entries unless an explicit `none` breadcrumb is defensible. Prefer Knots `blockencodings.cpp` / `net_processing.cpp` reconstruction and extra-txn anchors.
- **D-11:** Verification remains deterministic and local through repo-native checks and `bash scripts/verify.sh`. Public-network compact-relay review stays opt-in UAT only.

### Claude's Discretion

The planner/researcher may choose exact helper placement (shell receive intercept vs PeerManager facts API), whether extras live as a small ring buffer beside ManagedPeerNetwork, how wtxid is obtained from mempool removal summaries, and how tests inject candidate sets. Prefer early returns, iterator/slice adapters, and the smallest seam that closes the audit gap without reopening Phase 114 reconstruction policy.

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Project And Phase Scope

- `AGENTS.md` — repo-local verification, submodule, parity breadcrumb, UAT command, and GSD workflow guidance.
- `AGENTS.bright-builds.md` — Bright Builds workflow, functional-core, verification, and testing rules.
- `standards/core/architecture.md` — functional core / imperative shell and domain-type rules.
- `standards/core/code-shape.md` — early-return, optional-name, and file/function shape rules.
- `standards/core/testing.md` — focused unit test and Arrange/Act/Assert expectations.
- `standards/core/verification.md` — repo-native verification and clean commit gate expectations.
- `standards/languages/rust.md` — Rust module, invariant, optional naming, and verification guidance.
- `.planning/PROJECT.md` — active v2.1 scope, parity value, architecture constraints, and deferred public/production claims.
- `.planning/REQUIREMENTS.md` — RCN-02, RCN-03, GOV-04 ownership for Phase 119.
- `.planning/ROADMAP.md` — Phase 119 goal, success criteria, and gap-closure framing.
- `.planning/STATE.md` — current milestone state and deterministic verification caveats.
- `.planning/v2.1-MILESTONE-AUDIT.md` — RCN-02/RCN-03/GOV-04 gap evidence: empty CompactBlockReceiveFacts on live receive; mempool lifecycle does not call on_mempool_transaction_removed.

### Prior Locked Decisions

- `.planning/phases/114-compact-block-reconstruction-from-mempool-state/114-CONTEXT.md` — iterator-based mempool/extra inputs, typed outcomes, lifecycle hooks on PartialCompactBlock, no mempool crate coupling in network.
- `.planning/phases/115-missing-transaction-round-trip-fallback-and-validation-handoff/115-CONTEXT.md` — download/init path and getblocktxn/blocktxn ownership (do not reopen).
- `.planning/phases/118-outbound-compact-block-announcement-wiring/118-CONTEXT.md` — explicitly deferred receive candidate injection to Phase 119.
- `.planning/phases/113-compact-relay-negotiation-and-announcement-policy/113-CONTEXT.md` — negotiation independence from package/filter/public defaults.
- `.planning/phases/117-parity-traceability-uat-and-release-guardrails/117-CONTEXT.md` — no-claim and verifier-boundary posture to preserve.

### Existing Code Integration Points

- `packages/open-bitcoin-network/src/peer/message_dispatch.rs` — CompactBlock → `CompactBlockReceiveFacts::default()` (primary empty-facts seam).
- `packages/open-bitcoin-network/src/peer/compact_download_state.rs` — `CompactBlockReceiveFacts`, `handle_compact_block_download`.
- `packages/open-bitcoin-network/src/compact_reconstruction.rs` — `init_partial_compact_block`, `on_mempool_transaction_removed`.
- `packages/open-bitcoin-node/src/network.rs` — `receive_message` / `receive_sync_message` dispatch into PeerManager.
- `packages/open-bitcoin-node/src/network/mempool_lifecycle.rs` — `apply_connected_block_mempool_lifecycle` (missing compact removal hook).
- `packages/open-bitcoin-mempool/src/pool/lifecycle.rs` — `MempoolLifecycleSummary` / removal records.
- `docs/parity/source-breadcrumbs.json` — required breadcrumb registry for new first-party Rust source/test files.
- `scripts/verify.sh` — repo-native verification contract.

### Bitcoin Knots Anchors

- `packages/bitcoin-knots/src/blockencodings.cpp` — InitData mempool and extra-txn matching.
- `packages/bitcoin-knots/src/blockencodings.h` — `PartiallyDownloadedBlock` / extra txn inputs.
- `packages/bitcoin-knots/src/net_processing.cpp` — compact receive and mempool/extra candidate supply.
- `packages/bitcoin-knots/test/functional/p2p_compactblocks.py` — reconstruction behavior examples.

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets

- `CompactBlockReceiveFacts { candidates, extra }` already accepted by `handle_compact_block_download`.
- Phase 114 reconstruction and Phase 115 download/init outcomes already typed for Ready/Invalid/Failed and missing indexes.
- `PartialCompactBlock::on_mempool_transaction_removed` exists and is unit-tested; only the shell hook is missing.
- `ManagedPeerNetwork` already owns mempool and calls PeerManager for message handling — natural place for candidate adapters.

### Established Patterns

- Functional core stays free of mempool crate deps; node shell supplies iterators/slices.
- Gap-closure phases prefer smallest production seam wire-up with deterministic local tests.
- Evidence/announce patterns from Phase 118 keep shell responsible for effects that need node-owned state.

### Integration Points

- Replace or bypass empty-facts CompactBlock branch on the live `ManagedPeerNetwork::receive_*` path.
- After mempool removals in `mempool_lifecycle`, forward removed wtxids into PeerManager compact-download partial state.
- Runtime tests in `open-bitcoin-node` should exercise injected candidates; network-crate tests may keep empty facts for eligibility-only cases.

</code_context>

<specifics>
## Specific Ideas

- Treat the audit break point literally: `CompactBlockReceiveFacts::default()` in message_dispatch is the broken E2E seam for inbound reconstruct.
- Prefer shell intercept of `WireNetworkMessage::CompactBlock` so PeerManager APIs stay pure and reusable in tests with explicit facts.
- Mempool removal hook should use wtxid when available so short-ID matched slots clear correctly (Phase 114 D-11).

</specifics>

<deferred>
## Deferred Ideas

Compact-download timeout scheduling and misbehavior escalation (Phase 120), DurableSyncRuntime block-relay metrics/log projection (Phase 121), package relay, bloom/filter serving, compact filter serving, public serving defaults, public-network CI gates, archive-node claims, production full-node readiness, and production-funds wallet safety remain outside Phase 119.

</deferred>

---

*Phase: 119-compact-receive-mempool-candidate-injection*
*Context gathered: 2026-07-13*
