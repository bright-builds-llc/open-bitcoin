---
phase: 162-real-prune-retention-and-integrated-parity-proof
plan: "13"
subsystem: config
tags: [rust, disabled-sync, selected-network, dns, daemon]
requires:
  - phase: 162-01
    provides: Independent normal-maturity Regtest recipe and bounded oracle
provides:
  - Disabled selected-network and exact default-DNS preservation
  - Four-network loader and actual durable/transient consumer regressions
  - Unchanged strong configured Phase162 daemon fixture passes
affects: [162-02, configured-runtime, prune-network, wire-identity]
tech-stack:
  added: []
  patterns: [Existing producer correction with independent selected-chain test expectations]
key-files:
  created: []
  modified:
    - packages/open-bitcoin-rpc/src/config/loader/open_bitcoin_runtime.rs
    - packages/open-bitcoin-rpc/src/config/tests/sync_configuration.rs
    - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/daemon_sync.rs
    - packages/open-bitcoin-rpc/src/config/tests/blockfilter.rs
key-decisions:
  - Preserve public Mainnet constructors and all activation/override guards; correct only the existing Disabled producer arm.
  - Independently derive the amended blockfilter helper's expected network and DNS from config.chain while retaining complete sync equality.
requirements-completed: []
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 162-2026-10-11T16-25-37
generated_at: 2026-10-11T18:01:33Z
duration: 8 min
completed: 2026-10-11
---

# Phase 162 Plan 13: Disabled Selected-Network Preservation Summary

**Disabled daemon sync retains the parsed chain's existing runtime network and exact DNS defaults, restoring genuine Regtest configured startup without activating outbound work.**

## Performance and Scope

- First recorded execution clock: 2026-10-11T17:52:19Z; earlier context loading was not separately timed.
- Final verification clock: 2026-10-11T17:59:56Z. Summary preparation clock: 2026-10-11T18:00:17Z.
- Summary creation timestamp is its measured filesystem birthtime, 2026-10-11T18:01:33.062417Z, rounded down to seconds in generated_at.
- Tasks: 2/2. Source files: four existing files under the checked amendment.
- Production diff: only `daemon_sync_config_for_mode`'s existing Disabled arm, 16 additions and one replacement. Map `AddressNetwork` to existing `SyncNetwork`, assign `runtime.network`, and collect that network's exact `default_dns_seeds` into Strings.
- Test diffs: sync_configuration.rs adds 307 lines; daemon_sync.rs adds 114; blockfilter.rs helper has 10 additions and one replacement. Formatted file lengths are 618, 579 and 627 respectively, below the managed boundary.
- Repo-local AGENTS guidance, Bright Builds sidecar/overrides, architecture/code-shape/testing/verification/Rust standards and both active lesson files informed execution. Active lessons totaled 7,188 bytes with 2,397 conservatively estimated tokens and were loaded completely.

## Precise Failure and Result

The loader already parsed `-regtest` correctly. Missing/default/explicit disabled sync configuration then called the no-argument Mainnet constructor, losing selected runtime identity and retaining Mainnet DNS candidates. The actual frozen daemon fixture compared those Mainnet consensus parameters against independently constructed Regtest parameters before opening its configured BASIC owner.

Parent originally reported one fixture failure in 13.24s. This executor directly reran it before the production edit: **0 passed, 1 failed, 13.39s**, at unchanged fixtures.rs:172. The difference was pow-limit bits 486604799 versus 545259519 and false versus true for minimum-difficulty/no-retargeting flags; maturity remained 100. That is a measured receipt separate from the original parent report.

The pre-fix loader selector discovered five tests: **1 passed, 4 failed**. Failures included exact Testnet network mismatch and exact Mainnet-versus-Testnet DNS-vector mismatch. After the single producer correction, **5/5 loader tests**, **2/2 mode-guard tests**, the **actual eight-case daemon consumer matrix**, and the **unchanged strong fixture** passed. No fixture source, independent recipe or equality assertion was edited. Earlier interim worker passes remain superseded as selected-network evidence.

## Regression Coverage

- Four networks across eight absent/default/disabled/off/CLI-override entries, with all six independent requested activation settings both false and true: 64 loader cases. Exact network, DNS vectors, consensus, magic, port and prune-after identity are checked; Regtest defaults are empty.
- CLI flag aliases, canonical `-chain` names, global bitcoin.conf selectors and conflicting CLI-over-conf cases: 12 selector cases. Configless bootstrap remains equal to public Mainnet defaults.
- Eight disabled CLI override cases retain every default runtime field while ignoring malformed manual peers, custom DNS and all supplied zero bounds.
- MainnetIbd CLI/JSONC acceptance and three non-mainnet refusals: eight cases. Both partial JSONC forms are refused without CLI override and honor explicit disabled/enabled CLI precedence.
- Actual parsed durable Fjall and transient consumers on four networks: eight cases. Durable `config().network` and exact DNS defaults, selected wire magic, no preflight, no sync worker, zero peers and unchanged requested independent activation are verified. BASIC/peer-filter activation stays false for this empty-bootstrap control. Context/authority/runtime/store handles drop before unique datadir removal. These are consumer controls, not accepted-chain proofs.
- Existing enabled peer/resource override, config/auth/precedence and daemon pause/failure/success/shutdown controls remain passing.

## Commands and Measured Receipts

Every Cargo invocation was serial and used this exact pinned-runtime wrapper, with KEY and CARGO_ARGS supplied below:

```bash
export PATH=/tmp/open-bitcoin-bun-1.3.9-fresh:$PATH
bun run scripts/command-timings.ts run --key KEY -- cargo CARGO_ARGS
```

All test commands used `--manifest-path packages/Cargo.toml -p open-bitcoin-rpc`, `--all-features`, and `-- --test-threads=1`; `--nocapture` was also supplied for focused selectors and both fixture runs. The library/binary and selector are explicit below. Times are libtest time followed by total wrapper duration; wrapper timestamps come from `.local/open-bitcoin-dev/command-timings/<key>/` JSON receipts.

| Key / invocation                                                                                                                             | UTC start    | Result                        | Test / wrapper time  |
| -------------------------------------------------------------------------------------------------------------------------------------------- | ------------ | ----------------------------- | -------------------- |
| phase162-disabled-network-config: test --lib phase162_disabled_network (RED)                                                                 | 17:53:43.705 | 1 passed, 4 failed; exit 101  | 0.02s / 5.015s       |
| phase162-strong-fixture-before-network-fix: test --bin open-bitcoind phase162_daemon_fixture                                                 | 17:54:28.078 | 0 passed, 1 failed; exit 101  | 13.39s / 13.793s     |
| phase162-disabled-network-config: test --lib phase162_disabled_network (GREEN)                                                               | 17:54:55.982 | 5 passed; exit 0              | 0.03s / 3.936s       |
| phase162-sync-mode-guards: test --lib phase162_sync_mode_guard                                                                               | 17:55:12.195 | 2 passed; exit 0              | 0.00s / 0.264s       |
| phase162-disabled-network-daemon: test --bin open-bitcoind phase162_disabled_network_daemon                                                  | 17:55:16.940 | 1 passed; exit 0              | 5.27s / 10.654s      |
| phase162-strong-fixture-after-network-fix: test --bin open-bitcoind phase162_daemon_fixture                                                  | 17:55:34.920 | 1 passed; exit 0              | 15.00s / 15.270s     |
| phase162-config-regressions: test --lib config::tests (before amendment)                                                                     | 17:55:53.725 | 89 passed, 2 failed; exit 101 | 0.06s / 0.215s       |
| phase162-blockfilter-active-network-regression: test --lib phase157_blockfilter_loader_active_network_precedes_default_and_ignores_inactive  | 17:58:38.412 | 1 passed; exit 0              | 0.02s / 4.077s       |
| phase162-blockfilter-conf-precedence-regression: test --lib phase160_config_conf_precedence_and_sections                                     | 17:59:03.279 | 1 passed; exit 0              | 0.01s / 2.467s       |
| phase162-config-regressions: test --lib config::tests (after amendment)                                                                      | 17:59:14.545 | 91 passed; exit 0             | 0.06s / 0.321s       |
| phase162-daemon-sync-regressions: test --bin open-bitcoind daemon_sync                                                                       | 17:59:18.297 | 14 passed; exit 0             | 11.49s / 11.651s     |
| phase162-disabled-network-clippy: clippy --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --all-targets --all-features -- -D warnings | 17:59:34.055 | passed; exit 0                | 4.46s Cargo / 4.578s |

Final non-Cargo checks passed:

```bash
rustfmt --edition 2024 --config skip_children=true --check packages/open-bitcoin-rpc/src/config/loader/open_bitcoin_runtime.rs packages/open-bitcoin-rpc/src/config/tests/sync_configuration.rs packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/daemon_sync.rs packages/open-bitcoin-rpc/src/config/tests/blockfilter.rs
bun scripts/check-parity-breadcrumbs.ts --check
bun scripts/bright-builds-check.ts all
git diff --check
```

Parity verified 1,126 Rust files. Managed checks scanned 1,463 files with zero exceptions/findings and validated active repo lessons. Initial scoped rustfmt check reported ordinary layout changes in the two new test blocks; formatting was applied only to those owned test files before the passing checks. An attempted timing `summary` subcommand printed usage; receipts were subsequently read directly from actual JSON files, without affecting verification results.

## Checked Scope Amendment

Full config verification exposed two inherited failures in blockfilter.rs's shared `assert_activation_unchanged`: Regtest's full sync configuration was compared with Mainnet defaults. Execution stopped before editing this fourth file, reported exact failures and proposed a helper-only amendment. Parent amended Plan13 and reported independent checker PASS with zero blockers/warnings; execution resumed under that authorization, with the recorded resume clock 17:58:30Z.

The helper independently maps `config.chain` to expected network/default DNS, then retains the complete `config.sync == neutral.sync` assertion and every inbound/relay/block-serving/prune assertion. It never copies actual `config.sync` identity into expected values. No production scope expanded. Both named failures and the full 91-test suite passed afterward. This is the sole checked scope adjustment.

## Simplification, Security and Stubs

The explicit simplification pass retained one local enum mapping and existing default-DNS method; no new constructor, consumer workaround, public API, dependency, authority, networking effect or repair path is needed. Public Mainnet defaults, MainnetIbd refusal, enabled-only override guards and independent activation settings are unchanged. Existing breadcrumb blocks remain intact; no new Rust file requires registration. No README change is needed for this internal producer correction.

The declared selected-chain, activation, DNS-candidate and false-proof threats are covered by the loader/consumer matrix and frozen fixture. No new security-relevant surface beyond the declared trust boundaries was introduced. No TODO/FIXME/placeholder stub was found in the changed files; empty Regtest seeds and empty-bootstrap state are intentional tested values.

## Task Commits and Parent-Owned Gates

Task 1, Task 2, RED/GREEN and metadata commits are **deferred** under D-14. No staging, commits, branches, push or hook bypass occurred. Parent-owned STATE, ROADMAP, REQUIREMENTS, todo, registries and generated LOC were not edited by this executor. `requirements-completed` intentionally remains empty.

Full default `bash scripts/verify.sh`, independent source/security review, formal lifecycle verification, requirement activation and final Git remain parent-owned. Scoped Clippy compiled all RPC targets, but this summary does not claim the full workspace build/test/Bazel gate or Phase162 completion. Plan02 may resume from this prerequisite; this executor does not resume it. Cargo is idle.

## Self-Check: PASSED

The four owned existing source files are present; the sole production hunk is the Disabled arm, the sole blockfilter hunk is the checked helper, focused tests have nonzero discovery, and all declared final scoped gates passed. Existing strong fixture and recipe were not edited. No task commits exist to verify because D-14 explicitly defers all commits. Summary path and lifecycle metadata are checked after creation.

Summary Markdown check initially requested formatting; scoped mdformat with installed GFM/frontmatter extensions was applied and checked again. No generic Markdown normalization touched other artifacts.

## Owned RED-Probe Cleanup | 2026-10-11T18:08:34Z

Parent requested cleanup of the exact leftover `/var/folders/b6/j7bsvp3j6jzbl8r28p9wqtnh0000gn/T/open-bitcoind-sync-preflight-phase162-daemon-0-3927-1791741268481041000`. Its encoded timestamp (17:54:28.481041Z), measured birthtime (17:54:28.482881Z) and final mtime (17:54:41.862811Z) match this executor's directly measured failed frozen-fixture receipt (17:54:28.078Z–17:54:41.871Z, exit 101). The exact path was a real directory, not a symlink. `lsof +D` returned exit 1 with empty stdout/stderr (no open handles), and `ps -p 3927` returned exit 1 with empty stdout (owner process absent). Removed only this owned failed-probe directory with `shutil.rmtree` and verified its absence. No other temporary path, source, Cargo target, Git index or parent-owned artifact was changed during this cleanup follow-up.
