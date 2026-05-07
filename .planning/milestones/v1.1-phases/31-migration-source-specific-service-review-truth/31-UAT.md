---
status: complete
phase: 31-migration-source-specific-service-review-truth
source:
  - 31-01-SUMMARY.md
  - 31-02-SUMMARY.md
started: 2026-05-07T11:41:12.765Z
updated: 2026-05-07T11:57:58.860Z
---

## Current Test

[testing complete]

## Tests

### 1. Custom-source migration plan excludes unrelated service paths
expected: From the repo root, run `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli --test operator_binary -- --nocapture open_bitcoin_migrate_plan_selects_explicit_custom_source_outside_default_roots`. The test passes. Its exercised operator flow is the explicit-source equivalent of `cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --network regtest --no-color --datadir=<target-datadir> migrate plan --source-datadir=<custom-source-datadir>` or `bazel run //packages/open-bitcoin-cli:open_bitcoin -- --network regtest --no-color --datadir=<target-datadir> migrate plan --source-datadir=<custom-source-datadir>`. The dry-run plan selects the custom source datadir, includes source config and wallet paths, does not print an unrelated service definition path, explains that the service could not be confidently tied to the selected source install, hides cookie and wallet contents, and leaves source files unchanged.
result: pass
evidence: `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli --test operator_binary -- --nocapture open_bitcoin_migrate_plan_selects_explicit_custom_source_outside_default_roots` passed with 1 test passed and 14 filtered out.

### 2. Detected-source migration plan includes only matched service paths
expected: Run `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli --test operator_binary -- --nocapture open_bitcoin_migrate_plan_is_dry_run_only_for_detected_source_install`. The test passes. The exercised operator flow is the detected-source equivalent of `cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --network regtest --no-color --datadir=<target-datadir> migrate plan` or `bazel run //packages/open-bitcoin-cli:open_bitcoin -- --network regtest --no-color --datadir=<target-datadir> migrate plan`. The dry-run plan includes the source service path only when that service definition points at the selected source datadir or config, keeps secrets out of stdout, and does not mutate source files.
result: pass
evidence: `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli --test operator_binary -- --nocapture open_bitcoin_migrate_plan_is_dry_run_only_for_detected_source_install` passed with 1 test passed and 14 filtered out.

### 3. Planner distinguishes matched service ownership from ambiguous ownership
expected: Run `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli planner_limits_service_review_to_selected_source_installation planner_uses_manual_service_review_when_service_ownership_is_ambiguous -- --nocapture`. Both tests pass. Matched service evidence is rendered as a concrete service review action for the selected source, while ambiguous service evidence becomes a manual review step and the ambiguous service path is not rendered as source-specific.
result: pass
evidence: `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli service_review -- --nocapture` passed both `planner_limits_service_review_to_selected_source_installation` and `planner_uses_manual_service_review_when_service_ownership_is_ambiguous`.

### 4. Operator docs and repo-native verification match the repaired behavior
expected: Inspect `docs/operator/runtime-guide.md` under "Migration Planning" and run `bash scripts/verify.sh`. The docs state that `--source-datadir` only shows concrete service review paths when a service definition can be tied to the selected source install and otherwise keeps service cutover review manual. The repo-native verification contract passes.
result: pass
evidence: `docs/operator/runtime-guide.md` states the selected-source service review rule and manual fallback. `bash scripts/verify.sh` completed successfully in 2m 2.229s.

## Summary

total: 4
passed: 4
issues: 0
pending: 0
skipped: 0
blocked: 0

## Gaps
