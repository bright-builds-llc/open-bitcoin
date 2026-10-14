---
phase: 164-canonical-chain-identity-and-compatibility-gates
plan: "01"
subsystem: consensus
tags: [rust, canonical-genesis, chain-identity, validation-scope]
requires:
  - phase: 164-research
    provides: Pinned four-network genesis vectors and validation support matrix
provides:
  - Pure canonical specifications for mainnet, testnet3, default Signet and default regtest
  - Exact genesis header/body predicates with independently frozen serialization evidence
  - Immutable identity-only validation scope and exhaustive ordinary-history refusal
affects: [164-02, 164-03, 164-05, 164-06, 165, 166, 167]
tech-stack:
  added: []
  patterns: [private immutable support facts, exhaustive typed canonical profiles, complete root equality]
key-files:
  created:
    - packages/open-bitcoin-consensus/src/chain_spec.rs
    - packages/open-bitcoin-consensus/src/chain_spec/genesis.rs
    - packages/open-bitcoin-consensus/src/chain_spec/tests.rs
  modified:
    - packages/open-bitcoin-consensus/src/lib.rs
    - docs/parity/source-breadcrumbs.json
key-decisions:
  - Identity recognition returns no ordinary admission capability; every current profile refuses ordinary history.
  - Default Signet challenge is an inspectable profile fact and never proves solution validation.
  - Genesis construction uses existing domain constructors and crypto with no production hex parser or dependencies.
patterns-established:
  - CanonicalChain::spec is the sole constructor for private immutable ChainSpec and ValidationScope facts.
  - Root identity compares all header fields and the entire transaction body, including witness metadata.
requirements-completed: [CBOT-01]
requirements-addressed: [CBOT-01, CVLD-05]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 164-2026-10-14T01-59-33Z
generated_at: "2026-10-14T03:06:00Z"
duration: 8min
completed: 2026-10-14
---

# Phase 164 Plan 01: Canonical Identity Core Summary

**Four exact pinned genesis bodies and immutable support contracts reject foreign or altered roots without granting ordinary validation authority.**

## Performance

- Tasks completed: 2/2.
- Implementation files: 5; this summary is the sixth artifact.
- Execution duration: approximately 8 minutes.
- Commits: deferred strict phase finalization; no staging, commit, push, hook skip or tracker mutation performed by this executor.
- Requirement completion remains parent-owned after full phase verification.

## Accomplishments

- Exported `CanonicalChain`, `ChainSpec`, `ValidationScope`, `ValidationEvidenceClass`, `MissingValidationPrerequisite`, `UnsupportedValidationScope` and `RootIdentityError` from consensus.
- Constructed exact version-1 pinned genesis headers and the common 204-byte coinbase through existing primitives. Frozen tests establish four 80-byte headers, four 285-byte bodies, four pinned Bitcoin hashes, the common txid/merkle root and distinct auxiliary whole-body SHA-256 digests.
- Rejected all 12 foreign-network pairs, every header-field mutation, altered reward/input script/output script/sequence/witness/locktime/transaction version/outpoint, missing or extra transactions, and an arbitrary null-parent regtest header that satisfies its claimed PoW.
- Recorded immutable canonical bootstrap, exact work and candidate-history prerequisites for every profile, plus absent Signet challenge validation. Successful exact root recognition still refuses ordinary history.
- Registered all three new Rust paths against pinned `kernel/chainparams.cpp` and `validation.cpp` breadcrumbs.

## Task Commits

1. Task 1, pure identity/support contracts: deferred strict phase finalization.
2. Task 2, genesis construction/adversarial vectors: deferred strict phase finalization.

No task commit hashes exist; the strict wrapper reserves commits for the parent after a clean complete phase verification.

## Verification and TDD Evidence

Commands ran serially through the checkout's cooperative Cargo timing wrapper.

| Stage | Command | Actual result |
| --- | --- | --- |
| Task 1 RED | `bun run scripts/command-timings.ts run --key phase164-spec-contract -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-consensus --lib canonical_chain_support` | Exit 101, missing prescribed canonical/support types before implementation; session 65388. |
| Task 1 GREEN | Same command | Exit 0, 5 passed, 196 filtered; session 86815. |
| Task 2 RED | `bun run scripts/command-timings.ts run --key phase164-canonical-genesis -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-consensus --lib canonical_chain` | Exit 101, missing genesis predicates/constructors and challenge accessor before implementation; session 31305. |
| Task 2 GREEN | Same command | Exit 0, 31 passed, 196 filtered; session 67601. |
| Consensus lint | `bun run scripts/command-timings.ts run --key phase164-spec-clippy -- cargo clippy --manifest-path packages/Cargo.toml -p open-bitcoin-consensus --all-targets --all-features -- -D warnings` | Exit 0; session 49619. |
| Consensus regression | `bun run scripts/command-timings.ts run --key phase164-spec-lib -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-consensus --lib` | Exit 0, 227 passed, no failures or filtered tests. |
| Breadcrumbs | `bun scripts/check-parity-breadcrumbs.ts --check` | Exit 0, 1,144 Rust paths verified. |
| Formatting | `rustfmt --check --edition 2024` on the four touched Rust paths | Exit 0 after scoped formatting. |
| Whitespace | `git diff --check` | Exit 0. |

The default repo-native full verifier, independent phase reviews and final commit hook are deferred to the parent after all plans, as prescribed by this plan.

## API Notes for Downstream Executors

- `CanonicalChain::{Mainnet, Testnet3, Signet, Regtest}` exposes `spec()`, `name()` and canonical `Display`. Testnet identity spells `testnet3`; existing adapter selector spelling remains adapter-owned.
- `ChainSpec` exposes all prescribed methods: `chain`, `genesis_header`, `genesis_block`, `genesis_hash`, `check_genesis_header`, `check_genesis_block`, `validation_scope`, `require_ordinary_history`. Header mismatch returns `NonCanonicalHeader` before body comparison; otherwise a differing body returns `NonCanonicalBody`.
- `ValidationScope` exposes `chain()`, `evidence_class() -> ValidationEvidenceClass::IdentityOnly`, `ordinary_history() -> false`, and `missing_prerequisites() -> &'static [MissingValidationPrerequisite]`. All fields are private and there are no caller setters or supported flags.
- `UnsupportedValidationScope` exposes `scope()`, `chain()` and `missing_prerequisites()`. Its `Display` uses bounded fixed canonical names and prerequisite codes only.
- `MissingValidationPrerequisite` exposes `code()` and `Display`: `canonical_bootstrap`, `exact_work`, `candidate_history`, `signet_challenge_validation`.
- `ChainSpec::maybe_default_signet_challenge()` returns only the immutable pinned default challenge for Signet, otherwise `None`. Shared custom-Signet genesis never authorizes custom challenges.

## Decisions Made

Followed D-01, D-02, D-06 and D-07 without new dependencies, network selectors, consensus-rule changes, UTXO connection, BASIC writes or runtime I/O. The local `AGENTS.md`, Bright Builds sidecar, overrides, architecture/code-shape/testing/verification/Rust pages and both active lesson files informed execution.

Existing primitive constructors are fallible even for compile-time fixed valid amounts/scripts. The private coinbase constructor narrowly documents and expects those proven constant bounds (77/67 bytes below the script limit and 50 BTC within the money range); a scoped `#[expect(clippy::expect_used)]` records that invariant. No candidate input enters this function, no errors are swallowed, and focused production/test clippy passes.

## Simplification and Diff Review

Kept a single exhaustive profile enum, one common coinbase constructor, private support facts derived from the chain, and full domain equality. This avoids configurable capabilities, duplicated transaction bytes in production, production hex parsing and extra adapter abstractions. The new source files are 250/84/471 lines. Reviewed the five implementation paths and source imports; no filesystem, process, network, runtime imports, placeholder implementations or unplanned trust boundaries were introduced. README/operator changes remain with the later adapter/documentation plans.

## Deviations from Plan

None. Per-plan task commits, state/roadmap/requirements updates and full verification were explicitly reserved for strict parent phase finalization.

## Issues Encountered

None. Expected TDD missing-API compilation failures were followed by passing focused and consensus regression runs. No authentication gate occurred.

## User Setup Required

None.

## Next Phase Readiness

The pure identity and refusal APIs are ready for storage, admission, RPC/CLI and diagnostics adapters. Ordinary canonical bootstrap, work and candidate-history admission remain deliberately refused until their later phase owners deliver verified support.

## Self-Check: PASSED

All three created Rust files, the consensus export file, breadcrumb registry and this summary exist. Recorded RED/GREEN/lint/regression outputs substantiate the claims above. Commit checks are intentionally inapplicable because strict phase finalization is parent-owned; no fabricated hashes or completed requirements are recorded.

## Parent Accepted Requirement Closure | 2026-10-14

CBOT-01 has unique completion ownership in this summary after full default native session41473 exited zero (1h0m21.973s), clean 146-file review, 30 closed security controls and lifecycle-valid 27/27 phase verification. The original execution date and earlier evidence remain preserved. All four ordinary histories remain refused pending Phases165–167; no current ordinary qualification is asserted. Wrapper-owned normal-hook commit and push remain the final transaction.
