---
status: complete
phase: 24-wallet-aware-live-status-and-build-provenance
source:
  - 24-01-SUMMARY.md
  - 24-02-SUMMARY.md
  - 24-03-SUMMARY.md
started: 2026-05-05T10:30:45Z
updated: 2026-05-06T02:13:18Z
---

## Current Test

[testing complete]

## Tests

### 1. Ambiguous wallet status keeps node truth live
expected: Run one of these commands against a live RPC target where wallet selection is missing or ambiguous: `cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- status` or `bazel run //packages/open-bitcoin-cli:open_bitcoin -- status`. The node, sync, mempool, peer, and health fields should remain live instead of reporting `NodeRuntimeState::Unreachable`, while wallet-specific fields should degrade with wallet-specific unavailable diagnostics.
result: pass

### 2. Selected or sole managed wallet routes wallet status calls
expected: With a trusted selected wallet or exactly one managed wallet in the local Open Bitcoin wallet registry, run `cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- status` or `bazel run //packages/open-bitcoin-cli:open_bitcoin -- status`. Wallet-scoped status calls should use the selected wallet route, so wallet fields reflect that wallet instead of failing through root RPC ambiguity.
result: skipped
reason: No user-facing wallet creation command is available in the shipped operator CLI for this UAT setup.

### 3. Status and dashboard show shared build provenance
expected: Run `cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- status --format json` or `bazel run //packages/open-bitcoin-cli:open_bitcoin -- status --format json`, then inspect the dashboard build summary with `cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- dashboard` or `bazel run //packages/open-bitcoin-cli:open_bitcoin -- dashboard`. When Cargo build metadata is available, both surfaces should show the same shared build provenance values for version, commit, build time, target, and profile. If a build system omits metadata, the fields should remain explicit about being unavailable instead of silently disappearing.
result: issue
reported: "Cargo status JSON reports build_time as an ISO datetime, while Bazel status JSON reports build_time as integer epoch seconds."
severity: minor

### 4. Phase 24 evidence is complete and traceable
expected: Inspect the Phase 24 planning artifacts. The roadmap and requirements ledger should mark Phase 24 complete, `OBS-01`, `OBS-02`, `WAL-05`, and `DASH-01` should trace to the completed work, and `24-VERIFICATION.md` should record the passing repo-native verification evidence including `bash scripts/verify.sh`.
result: pass

## Summary

total: 4
passed: 2
issues: 1
pending: 0
skipped: 1
blocked: 0

## Gaps

- truth: "Status and dashboard surfaces should show the same shared build provenance values for version, commit, build time, target, and profile."
  status: fixed
  reason: "User reported: Cargo status JSON reports build_time as an ISO datetime, while Bazel status JSON reports build_time as integer epoch seconds."
  severity: minor
  test: 3
  root_cause: "Cargo build.rs emits OPEN_BITCOIN_BUILD_TIME as ISO-8601 text via date -u +%Y-%m-%dT%H:%M:%SZ, while packages/open-bitcoin-cli/BUILD.bazel injects Bazel {BUILD_TIMESTAMP}, which is epoch seconds, and status.rs forwards the env var unchanged."
  artifacts:
    - path: "packages/open-bitcoin-cli/build.rs"
      issue: "Cargo build time uses ISO-8601 output."
    - path: "packages/open-bitcoin-cli/BUILD.bazel"
      issue: "Bazel build time passes {BUILD_TIMESTAMP} epoch seconds directly."
    - path: "packages/open-bitcoin-cli/src/operator/status.rs"
      issue: "Build provenance accepts build_time as a raw string without normalizing format."
  missing:
    - "Resolved: build_time now uses one stable ISO-8601 UTC format across Cargo and Bazel builds."
  debug_session: ""
  closure:
    fixed_by_plan: "24-04"
    evidence:
      - command: "SOURCE_DATE_EPOCH=1778032597 cargo run --quiet --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- status --format json | jq -r '.build.build_time.value'"
        observed: "2026-05-06T01:56:37Z"
      - command: "cargo run --quiet --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- status --format json | jq -r '.build.build_time.value'"
        observed: "2026-05-06T02:11:59Z"
      - command: "bazel run //packages/open-bitcoin-cli:open_bitcoin -- status --format json | jq -r '.build.build_time.value'"
        observed: "2026-05-06T02:12:44Z"
