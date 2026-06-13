---
phase: 72-operator-observability-and-support-evidence
fixed_at: 2026-06-13T20:39:26Z
review_path: .planning/phases/72-operator-observability-and-support-evidence/72-REVIEW.md
status: fixed
findings_in_scope: 4
fixed: 4
skipped: 0
iteration: 1
generated_by: gsd-code-review-fix
lifecycle_mode: yolo
phase_lifecycle_id: 72-2026-06-13T16-25-04
---

# Phase 72: Code Review Fix Report

## Scope

Fixed all warning findings from `72-REVIEW.md`. The fixes were applied inline under the wrapper-owned final-commit flow, so no intermediate fix commits were created.

## Fixes Applied

### WR-01: Peer shortfall overstated as `diagnosed_blocker`

Updated support evidence verdict derivation so resource pressure is blocking only when block in-flight pressure reaches the configured total cap. A shortfall between connected and target outbound peers no longer escalates a missing sync-to-tip proof to `diagnosed_blocker` by itself.

Regression added: `phase72_support_verdict_peer_shortfall_without_blocking_signal_is_inconclusive`.

### WR-02: Support Markdown hid partial active-chain height

Updated support Markdown rendering to always include active-chain height/hash/work slots and append the exact `Unavailable: {reason}` text when one field is missing. The binary support-bundle test now asserts unavailable hash/work output still preserves `height=840004`.

### WR-03: Live-smoke synthesized validated active-chain height

Updated live-smoke final-status projection so missing `validated_active_chain_height` remains `null` with `maybeValidatedActiveChainHeightUnavailableReason: "validated active-chain height unavailable"`. The Markdown report now renders that specific unavailable reason instead of deriving a height from connected or block height.

Regression added to `scripts/test-run-live-mainnet-smoke.sh` using a local final-status fixture with `validated_active_chain_height` removed.

### WR-04: Phase 72 checker had false-positive `evidence_verdict` coverage

Moved positive verdict assertions to support evidence files and kept RPC `getblockchaininfo` as an explicit baseline exclusion guard. The checker now verifies the support bundle verdict path and separately verifies the RPC test forbids `evidence_verdict`.

## Verification

- `cargo fmt --manifest-path packages/Cargo.toml --all --check`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli --lib phase72_support_verdict_ --all-features -- --nocapture`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli --test operator_binary open_bitcoin_support_bundle_includes_phase72_full_sync_evidence_and_typed_verdict --all-features -- --nocapture`
- `bash scripts/test-run-live-mainnet-smoke.sh`
- `bun run scripts/check-phase72-observability-evidence.ts`
- `bun run scripts/check-phase71-resource-restart.ts`
- `bun run scripts/check-parity-breadcrumbs.ts --check`
- `bash scripts/check-file-lengths.sh`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc phase72 --all-features -- --nocapture`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node phase72_summary_metrics_and_logs_carry_full_sync_truth_dimensions --all-features -- --nocapture`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node sync_summary --all-features -- --nocapture`

## Residual Risk

No known Phase 72 code-review findings remain. Public-network live-smoke remains opt-in UAT evidence and is intentionally outside default verification.
