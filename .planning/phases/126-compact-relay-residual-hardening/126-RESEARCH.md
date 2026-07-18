---
phase: 126-compact-relay-residual-hardening
generated_by: gsd-phase-researcher
lifecycle_mode: yolo
phase_lifecycle_id: 126-2026-07-18T16-09-20
generated_at: 2026-07-18T18:34:12.965Z
status: complete
---

# Phase 126 Research

## Research Outcome

Phase 126 is a bounded hardening and closeout phase, not a new compact-relay implementation. The existing managed receive path already supplies live mempool and bounded extra-transaction snapshots, the compact payload builder already accepts a pure nonce input, and the Phase 124/125 checkers already encode most lifecycle and claim-boundary rules. The work is to make those boundaries structural, add exact parity and mutation evidence, and promote the six remaining requirements only after lifecycle-valid verification.

The required plan coverage is `CMP-05`, `RCN-02`, `RCN-03`, `GOV-04`, `BOUND-01`, and `HARD-05`. The `init plan-phase` helper currently reports `phase_req_ids: null`, so the planner and checker must use the explicit Phase 126 roadmap requirement list rather than treating the phase as requirement-free.

## Runtime Findings

### Compact Receive Routing

- `packages/open-bitcoin-network/src/peer/message_dispatch.rs` still routes `WireNetworkMessage::CompactBlock` through `CompactBlockReceiveFacts::default()`. Its comment says the branch is test-only, but the public generic dispatcher remains callable by a future production adapter.
- `packages/open-bitcoin-network/src/peer/compact_download_state.rs` derives `Default` for `CompactBlockReceiveFacts<'a>`, making the factless route easy to reconstruct accidentally.
- `packages/open-bitcoin-node/src/network.rs` already intercepts `CompactBlock` in both `receive_message` and `receive_sync_message` before calling generic dispatch.
- `packages/open-bitcoin-node/src/network/compact_receive_candidates.rs` already snapshots the current mempool and `CompactExtraTxnBuffer`, builds explicit candidate and extra reference slices, and calls `handle_compact_block_download`.
- An explicitly supplied pair of empty slices is valid when both live sources are genuinely empty. The invariant is adapter routing and snapshot provenance, not non-empty content.
- At least sixteen focused tests currently call `CompactBlockReceiveFacts::default()`. They should use explicit field construction or a test-local helper after `Default` is removed.

Recommended shape:

1. Add a stable `NetworkError` variant representing an adapter-routing contract failure. It must not carry peer blame and must not translate to compact misbehavior or disconnect evidence.
2. Make generic `PeerManager::handle_message` return that error for `CompactBlock` without entering reconstruction.
3. Remove `Default` from `CompactBlockReceiveFacts`.
4. Keep `handle_compact_block_download` as the factful pure/network-core seam and keep `ManagedPeerNetwork` as the production shell.
5. Add direct regressions for generic fail-closed dispatch, both managed receive entrypoints, genuine explicit-empty snapshots, and typed reconstruction outcomes.

### Compact Announcement Nonce

- `ManagedPeerNetwork::announce_block` in `packages/open-bitcoin-node/src/network.rs` decides the announcement action, then always derives a deterministic nonce from the first eight little-endian bytes of the block hash.
- `PeerManager::announce_block_with_action` and `build_compact_block_payload(block, nonce)` already preserve the desired pure boundary.
- `getrandom` `0.3.4` is already locked and is a direct dependency of `open-bitcoin-rpc`; the node crate does not yet declare it.
- `packages/open-bitcoin-node/Cargo.toml` and `packages/open-bitcoin-node/BUILD.bazel` must add matching direct dependencies.
- Existing achieved-effect logic records compact provenance and `CompactAnnounced` only after the actual output message is known. Entropy failure must reuse that boundary.

Recommended shape:

1. Decide `CompactAnnouncementAction` before invoking entropy.
2. Use a private production function that fills `[u8; 8]` with `getrandom::fill` and converts it with `u64::from_le_bytes`.
3. Route through a narrow helper accepting a lazy fallible closure such as `FnOnce() -> Result<u64, CompactNonceError>` so fixed, failing, and invocation-counting tests do not require stored RNG state.
4. Call the closure only for `AnnounceCompactBlock`; headers, inventory, and suppression paths must not consume entropy.
5. On entropy failure, use the peer's existing typed headers/inventory fallback (or safe suppression if required by the concrete API). Never call compact payload construction, record compact provenance, increment `compact_announced_count`, or emit `CompactAnnounced`.
6. Keep `open-bitcoin-consensus` and `open-bitcoin-network` free of entropy dependencies.

## Parity And Evidence Findings

The exact Knots boundary is split across:

- `packages/bitcoin-knots/src/net_processing.cpp`: `FastRandomContext().rand64()` at compact announcement construction and live mempool/recent-extra candidate supply for reconstruction.
- `packages/bitcoin-knots/src/net_processing.h`: bounded compact extra-transaction constants and ownership.
- `packages/bitcoin-knots/src/blockencodings.cpp`: `CBlockHeaderAndShortTxIDs` nonce consumption and `PartiallyDownloadedBlock::InitData`.
- `packages/bitcoin-knots/src/blockencodings.h`: compact payload and randomized nonce contract.
- `packages/bitcoin-knots/test/functional/p2p_compactblocks.py`: behavioral coverage.

Update `docs/parity/index.json`, `docs/parity/source-breadcrumbs.json`, `docs/parity/catalog/p2p.md`, and `docs/parity/catalog/mempool-policy.md` so the receive-routing and randomized-announcement seams have concrete anchors. If Phase 126 adds any first-party Rust source or test file, register its required breadcrumb through `docs/parity/source-breadcrumbs.json`; modifying existing Rust files does not require a new source-file entry, but their current breadcrumbs must remain accurate.

Add a Phase 126 Bun checker and focused mutation test, then wire both after the Phase 124 checker and active milestone traceability checker but before the Phase 117 final no-claim gate. The checker should prove:

- generic compact dispatch cannot construct default/empty facts;
- `CompactBlockReceiveFacts` does not derive or implement `Default`;
- both managed receive paths still intercept compact blocks and supply live snapshots;
- production compact nonce uses the node-shell entropy adapter rather than block-hash bytes;
- entropy is lazy and failure cannot produce compact wire output or achieved-effect evidence;
- Cargo and Bazel node dependencies agree;
- required parity and breadcrumb anchors exist;
- default verification remains deterministic and public-network-free.

Mutation tests should remove or invert each of those anchors independently. Prefer focused structural assertions plus real behavioral tests over repository-wide prose scans.

## Closeout And Lifecycle Findings

The canonical audit is intentionally `gaps_found` at `33/39` requirements and `16/17` phases. All six requirements remain owned by Phase 126. The Phase 124 closeout checker already understands several pre- and post-verification states, but it still contains historical ownership and stage assumptions that must be reconciled with Phase 126.

Use four explicit legal states:

1. **Candidate:** runtime, parity, checker, and plan summaries may exist; all six requirements remain pending, the audit remains non-passed, and `/gsd-execute-phase 126` remains the primary route.
2. **Verified pre-promotion:** lifecycle-valid `126-VERIFICATION.md` exists with `status: passed`; requirements and audit remain pending until the full repository contract passes against this state.
3. **Promoted pre-summary:** all six checklist/traceability rows and the canonical audit are promoted, but the final plan summary or final phase-completion projection is not yet present. The guard must require the passed verification and reject mixed counts or stale routes.
4. **Archive-ready:** the final Phase 126 summary exists, Phase 126 is complete, all canonical counts agree at `39/39` and `17/17`, the refreshed audit is `passed`, and `.planning/ROADMAP.md`, `.planning/STATE.md`, and the audit route only to `/gsd-complete-milestone v2.1`.

The generic active-milestone verification-orphan checker remains the lifecycle coverage authority. `CONTEXT.md`, all plan frontmatter, summaries, and `VERIFICATION.md` must carry lifecycle `126-2026-07-18T16-09-20` and `lifecycle_mode: yolo` where their schemas require those fields.

Do not promote requirements or archive routing in an early runtime plan. If the fresh audit finds a genuine gap, retain `gaps_found`, leave the requirements pending, and stop without manufacturing Phase 127.

## Recommended Plan Structure

### Plan 126-01: Receive And Nonce Runtime Hardening

Own the Rust behavior and focused Rust tests:

- `packages/open-bitcoin-network/src/error.rs`
- `packages/open-bitcoin-network/src/peer/message_dispatch.rs`
- `packages/open-bitcoin-network/src/peer/compact_download_state.rs`
- `packages/open-bitcoin-network/src/peer/tests.rs`
- `packages/open-bitcoin-node/src/network.rs`
- `packages/open-bitcoin-node/src/network/compact_receive_candidates.rs`
- `packages/open-bitcoin-node/src/network/tests.rs` and relevant split test modules
- `packages/open-bitcoin-node/Cargo.toml`
- `packages/open-bitcoin-node/BUILD.bazel`
- `packages/Cargo.lock` if Cargo metadata refresh changes it

Verify focused network/node tests, formatting, Clippy/build, and the Bazel node target through the repo timing wrapper.

### Plan 126-02: Parity And Deterministic Guardrails

Depend on 126-01. Own parity roots, breadcrumbs, the new Phase 126 checker/mutation tests, verifier wiring, and Phase 124 legal intermediate-state support. Keep all six requirements pending and the audit non-passed.

### Plan 126-03: Lifecycle Verification Evidence

Depend on 126-02. Run the focused checkers plus the full `bash scripts/verify.sh` contract, create lifecycle-valid Phase 126 verification evidence, and prove the verified-pre-promotion state. Do not promote the audit or requirements in the same pre-verification task.

### Plan 126-04: Final Promotion And Archive Projection

Depend on 126-03. Promote the six requirements and traceability rows, refresh the one canonical audit in place, reconcile ROADMAP/STATE/PROJECT/MILESTONES and any release-facing docs required by the audit, run the full verifier again against the final projection, and leave only `/gsd-complete-milestone v2.1` as the primary route. If any check fails, keep or restore the non-passed candidate state and stop.

Sequential waves are safer because Plans 02-04 share verifier and planning-corpus files and because the closeout guard intentionally validates transitions.

## Validation Architecture

### Fast Feedback

- `bun test scripts/check-phase126-compact-relay-residual-hardening.test.ts`
- `bun test scripts/check-phase124-milestone-closeout-reconciliation.test.ts`
- `bun test scripts/check-active-milestone-verification-traceability.test.ts`
- focused Cargo tests for the changed network and node modules
- `cargo fmt --all --check` and focused Clippy/build commands
- Bazel smoke target for `//packages/open-bitcoin-node:open_bitcoin_node_lib`

Run ad hoc Cargo/Bazel commands through `bun run scripts/command-timings.ts run --key <stable-key> -- <command>` and do not overlap Cargo jobs against the shared target directory.

### Plan And Lifecycle Gates

- Plan checker must cover all six explicit IDs even though `init plan-phase` returns a null requirement list.
- `node "$HOME/.codex/get-shit-done/bin/gsd-tools.cjs" verify lifecycle 126 --require-plans --raw` must pass before execution.
- After verification, require `verify lifecycle 126 --require-plans --require-verification --raw`.
- `126-VERIFICATION.md` must have `status: passed`, `lifecycle_mode: yolo`, and the exact Phase 126 lifecycle ID.

### Full Gates

Run `bash scripts/verify.sh` at the verified-pre-promotion state and again after final promotion. The final strict wrapper must also inspect `git diff --check`, the final worktree, lifecycle validity, verification status, and the absence of unintended positive claims before committing/pushing.

## Risks

| Risk | Mitigation |
| --- | --- |
| Adapter-routing failure is mistaken for peer misbehavior | Give it a distinct typed error and add a regression proving no disconnect/misbehavior action. |
| Removing `Default` creates broad test churn | Use explicit facts or a test-local constructor; do not add a public production footgun. |
| Entropy is consumed on non-compact paths | Inject a lazy closure after action selection and test invocation count. |
| Entropy failure records false success | Derive evidence and provenance only from the actual emitted message. |
| Cargo and Bazel dependencies drift | Add `getrandom` to both node manifests and enforce both in the Phase 126 checker. |
| Requirements are promoted before proof | Encode the four legal stages and run the full verifier before and after promotion. |
| Final metadata becomes self-attesting | Keep the lifecycle-valid verification artifact and generic orphan checker as independent inputs. |
| Archive routing hides a fresh gap | Preserve the non-passed audit and Phase 126 route whenever final verification is not clean. |
