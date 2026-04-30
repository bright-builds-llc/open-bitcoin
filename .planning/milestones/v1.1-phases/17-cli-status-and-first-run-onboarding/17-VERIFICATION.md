---
phase: 17-cli-status-and-first-run-onboarding
verified: 2026-04-27T01:45:00Z
status: passed
score: 8/8 requirements verified
generated_by: gsd-verifier
lifecycle_mode: yolo
phase_lifecycle_id: 17-2026-04-26T23-56-00
generated_at: 2026-04-27T01:45:00Z
lifecycle_validated: true
overrides_applied: 0
deferred:
  - truth: "Service lifecycle mutation is a Phase 18 deliverable."
    addressed_in: "Phase 18"
    evidence: "open-bitcoin service commands return an explicit Phase 18 boundary message."
  - truth: "Dashboard rendering is a Phase 19 deliverable."
    addressed_in: "Phase 19"
    evidence: "open-bitcoin dashboard returns an explicit Phase 19 boundary message."
---

# Phase 17: CLI Status and First-Run Onboarding Verification Report

**Phase Goal:** Give operators a usable `open-bitcoin` command surface for status, configuration discovery, and first-run onboarding without mutating existing Core/Knots data.
**Verified:** 2026-04-27T01:45:00Z
**Status:** passed

## Goal Achievement

Phase 17 achieved the planned operator CLI surface. The implementation provides typed operator command parsing, config precedence and detection contracts, shared status rendering backed by `OpenBitcoinStatusSnapshot`, and an actual `open-bitcoin` binary with idempotent onboarding and hermetic binary tests.

### Observable Truths

| # | Truth | Status | Evidence |
|---|-------|--------|----------|
| 1 | `open-bitcoin` is a separate operator binary and does not replace the `open-bitcoin-cli` compatibility path. | VERIFIED | `packages/open-bitcoin-cli/src/bin/open-bitcoin.rs` calls `OperatorCli::parse` and `execute_operator_cli`; `operator_flows` compatibility tests still pass. |
| 2 | `open-bitcoin status` works for stopped nodes and emits stable JSON with explicit unavailable live fields. | VERIFIED | `open_bitcoin_status_json_succeeds_for_stopped_node` passes and asserts top-level status fields plus stopped/unavailable values. |
| 3 | `open-bitcoin status` can map deterministic fake RPC responses into running-node status. | VERIFIED | `open_bitcoin_status_json_uses_fake_running_rpc` passes and asserts running state, regtest network, chain height, peers, mempool, and wallet balance. |
| 4 | Human status output is support-oriented and honors `--no-color`. | VERIFIED | `open_bitcoin_status_human_no_color_is_support_oriented` passes and asserts all required labels with no ANSI escape sequences. |
| 5 | Config paths report Open Bitcoin JSONC, bitcoin.conf, datadir, logs, metrics, and source precedence. | VERIFIED | `open_bitcoin_config_paths_reports_sources` passes and checks labels plus `cli_flags > environment > open_bitcoin_jsonc`. |
| 6 | First-run onboarding is idempotent, writes only Open Bitcoin JSONC after approval, and does not create or modify `bitcoin.conf`. | VERIFIED | Onboarding unit tests and `open_bitcoin_onboard_non_interactive_is_idempotent` pass, including unchanged second-run contents and no `bitcoin.conf` creation. |
| 7 | Read-only Core/Knots detection evidence appears with source paths and uncertainty language. | VERIFIED | Binary status/onboarding tests assert detected candidate paths and `uncertain`; detector unit tests cover macOS/Linux candidate discovery. |
| 8 | Service and dashboard commands are explicit downstream boundaries. | VERIFIED | Runtime dispatch returns Phase 18 for service commands and Phase 19 for dashboard; existing operator flow tests cover deferred surfaces. |

**Score:** 8/8 requirements verified

### Required Artifacts

| Artifact | Expected | Status | Details |
|---|---|---|---|
| `packages/open-bitcoin-cli/src/operator.rs` | Operator command and flag contract | VERIFIED | Includes status, config, onboard, service, and dashboard clap surfaces plus onboarding flags. |
| `packages/open-bitcoin-cli/src/operator/config.rs` | Config precedence resolver | VERIFIED | Resolves CLI/env/JSONC/bitcoin.conf/cookie/defaults and reports source names. |
| `packages/open-bitcoin-cli/src/operator/detect.rs` | Read-only Core/Knots detection | VERIFIED | Detects candidate data dirs, configs, cookies, wallets, and service definitions without mutation. |
| `packages/open-bitcoin-cli/src/operator/status.rs` | Shared status collector | VERIFIED | Collects stopped, unreachable, and live RPC status into `OpenBitcoinStatusSnapshot`. |
| `packages/open-bitcoin-cli/src/operator/status/render.rs` | Human/JSON status renderer | VERIFIED | JSON serializes the shared snapshot; human output includes required support labels. |
| `packages/open-bitcoin-cli/src/operator/status/http.rs` | Runtime HTTP status RPC adapter | VERIFIED | Maps resolved local RPC config to `StatusRpcClient`; binary fake RPC test proves live path. |
| `packages/open-bitcoin-cli/src/operator/onboarding.rs` | Onboarding planner and write shell | VERIFIED | Pure plan plus approved JSONC-only write behavior; no `bitcoin.conf` mutation. |
| `packages/open-bitcoin-cli/src/operator/runtime.rs` | Command dispatch runtime | VERIFIED | Dispatches status, config paths, onboard, and downstream boundary errors. |
| `packages/open-bitcoin-cli/src/bin/open-bitcoin.rs` | Operator binary | VERIFIED | Parses `OperatorCli`, executes runtime, prints stdout/stderr, and returns typed exit code. |
| `packages/open-bitcoin-cli/tests/operator_binary.rs` | End-to-end binary tests | VERIFIED | Covers stopped/live status, human output, config paths, and onboarding idempotency. |
| `README.md` and `docs/architecture/cli-command-architecture.md` | Contributor-facing operator docs | VERIFIED | Updated to describe executable operator status/config/onboard surface. |
| `docs/parity/source-breadcrumbs.json` | Breadcrumb coverage | VERIFIED | New Rust files registered and check mode passes. |

### Requirements Coverage

| Requirement | Status | Evidence |
|---|---|---|
| OBS-01 | SATISFIED | Shared status snapshot JSON/human rendering includes node, config, service, sync, peers, mempool, wallet, logs, metrics, health, and build fields. |
| OBS-02 | SATISFIED | Status paths include stopped, unreachable, and fake live RPC evidence, with explicit unavailable reasons. |
| CLI-03 | SATISFIED | Actual `open-bitcoin` binary exists in Cargo and Bazel and routes through `execute_operator_cli`. |
| CLI-04 | SATISFIED | Config path command reports selected config, bitcoin.conf, datadir, logs, metrics, and sources. |
| CLI-05 | SATISFIED | Onboarding asks practical first-run questions through an injected prompter and supports non-interactive automation. |
| CLI-06 | SATISFIED | Existing `open-bitcoin.jsonc` is preserved without `--force-overwrite`; missing non-interactive values fail deterministically. |
| CLI-07 | SATISFIED | Status rendering and detection evidence are support-oriented and stable for automation. |
| MIG-02 | SATISFIED | Detection is read-only and surfaces Core/Knots candidates with uncertainty instead of mutating source data. |

### Behavioral Verification

| Check | Result |
|---|---|
| `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli operator::` | PASS |
| `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli --test operator_binary` | PASS |
| `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli --test operator_flows` | PASS |
| `bun run scripts/check-parity-breadcrumbs.ts --check` | PASS |
| `bazel build //packages/open-bitcoin-cli:open_bitcoin //packages/open-bitcoin-cli:open_bitcoin_cli` | PASS |
| `cargo fmt --manifest-path packages/Cargo.toml --all` | PASS |
| `cargo clippy --manifest-path packages/Cargo.toml --all-targets --all-features -- -D warnings` | PASS |
| `cargo build --manifest-path packages/Cargo.toml --all-targets --all-features` | PASS |
| `cargo test --manifest-path packages/Cargo.toml --all-features` | PASS |
| `bash scripts/verify.sh` | PASS |
| `gsd-tools verify lifecycle 17 --require-plans --require-verification` | PASS |

### Anti-Patterns Found

| File | Pattern | Severity | Impact |
|---|---|---|---|
| - | None blocking | - | Runtime was simplified below the file-size guideline by moving HTTP status RPC into `operator/status/http.rs`. No credential rendering, Core/Knots mutation, or public-network default tests were found. |

### Human Verification Required

None. The phase is CLI/runtime work with hermetic tests and no external service setup required.

### Gaps Summary

No blocking gaps found. Service lifecycle effects and dashboard rendering are intentionally deferred to Phases 18 and 19 and return explicit boundary messages in Phase 17.

---

_Verified: 2026-04-27T01:45:00Z_
_Verifier: Codex (GSD yolo wrapper)_
