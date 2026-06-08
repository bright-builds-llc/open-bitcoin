---
generated_by: gsd-plan-phase
lifecycle_mode: yolo
phase_lifecycle_id: 66-2026-06-08T21-58-25
generated_at: 2026-06-08T21:58:25.000Z
phase: 66
requirements: [COMPAT-01, COMPAT-02, COMPAT-03]
---

# Phase 66: Compatibility Harness Operator Wrapper - Research

## RESEARCH COMPLETE

Phase 66 should be implemented as a narrow operator wrapper around the existing Phase 54 pure compatibility harness. The core diagnosis surface already exists in `open-bitcoin-network`; the new work is making it runnable and reviewable through stable operator-facing files without turning public-network checks into default verification.

## Existing Implementation Surfaces

### Pure Compatibility Harness

- `packages/open-bitcoin-network/src/compatibility.rs` exposes `evaluate_transcript`, `CompatibilityDiagnosis`, `CompatibilityReport`, `TranscriptEvent`, and `TranscriptStep`.
- The harness already covers all required diagnosis variants: `Compatible`, `VersionRejected`, `NetworkMismatch`, `ServiceBitMismatch`, `UnsupportedMessageOrder`, `Timeout`, `PeerDisconnect`, `MalformedPayload`, and `LocalConfigurationFailure`.
- The harness already prevents failed peer transcripts from receiving useful-progress credit, which is the daemon-alignment behavior Phase 66 needs to preserve in reports.

### Operator CLI Shell

- `packages/open-bitcoin-cli/src/operator.rs` owns the Clap command contract for `open-bitcoin`.
- `packages/open-bitcoin-cli/src/operator/runtime.rs` routes parsed commands and resolves global network/datadir/output format configuration.
- `packages/open-bitcoin-cli/src/operator/support.rs` and `support/render.rs` show the repo pattern for local JSON/Markdown report writing and human/JSON command outcomes.
- `packages/open-bitcoin-cli/tests/operator_binary.rs` provides end-to-end binary test patterns for JSON output, report file creation, and redaction assertions.

### Docs And Deterministic Checkers

- `docs/operator/runtime-guide.md` is the right operator command home.
- `docs/parity/catalog/p2p.md` already carries compatibility harness and opt-in evidence boundaries.
- `scripts/check-phase65-support-review.ts` is the most relevant pattern for a Phase 66 deterministic source/docs/default-verification checker.
- `scripts/verify.sh` is the repo-native deterministic verification contract and must not gain live public-network commands.

## Recommended Plan

Use a single plan:

1. Add the `open-bitcoin compatibility harness` command and stable report DTO/rendering.
2. Add operator-binary tests for JSON/Markdown report shape and each required diagnosis scenario.
3. Document Cargo/Bazel wrapper commands, update parity wording, add a deterministic Phase 66 checker, and wire it into `scripts/verify.sh`.

## Risks And Mitigations

- **Risk:** CLI duplicates peer diagnosis logic and drifts from daemon behavior.
  **Mitigation:** Generate reports from `open-bitcoin-network::evaluate_transcript` and map only presentation fields in the CLI layer.
- **Risk:** Operators mistake a local compatibility report for a live public-network success claim.
  **Mitigation:** Include explicit report redaction/boundary text and runtime-guide wording that the wrapper is opt-in local evidence outside default verification.
- **Risk:** Default verification starts contacting public peers.
  **Mitigation:** Add a Bun checker that requires the wrapper docs/source strings and rejects public-network live-smoke/manual-peer commands in `scripts/verify.sh`.

## Verification Strategy

- Focused Rust tests:
  - `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli open_bitcoin_compatibility --all-features`
  - `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-network compatibility --all-features`
- Deterministic checker:
  - `bun run scripts/check-phase66-compatibility-wrapper.ts`
- Full repo contract:
  - `bash scripts/verify.sh`
