---
phase: 53-live-evidence-refresh
plan: 01
status: passed
verified_at: 2026-06-01T04:07:00Z
generated_by: gsd-verify-work
generated_at: 2026-06-01T04:07:00Z
lifecycle_mode: yolo
phase_lifecycle_id: 53-2026-06-01T02-51-22
lifecycle_validated: true
requirements:
  - PEER-03
  - PROOF-03
  - PROOF-04
  - PROOF-05
  - OBS-02
  - SEC-03
closeout_mode: satisfied-by-fresh-diagnosed-blocker
---

# Phase 53 Verification

## Result

Status: passed.

Phase 53 closes through fresh diagnosed-blocker evidence, not live progress.
The selected report is
`packages/target/live-mainnet-smoke-reports/phase53-manual-peer/open-bitcoin-live-mainnet-smoke.json`.
It is a schema v2 report with `result.status=no_progress`,
`result.progressDetected=false`, `result.headerDelta=0`,
`result.blockDelta=0`, `result.maybeNoProgressCause=handshake_failure`, and a
concrete `result.nextAction`.

The selected report's status command uses `openbitcoinsyncstatus`, so it
supersedes the historical Phase 50 fresh-status caveat without claiming header
or block progress.

## Requirement Verdicts

| Requirement | Verdict | Evidence |
| --- | --- | --- |
| `PEER-03` | Passed by fresh diagnosed blocker | Phase 53 UAT records 68 runtime peer rows with failed/stalled states, zero contribution counters, typed `handshake_failure`, and next action. |
| `PROOF-03` | Passed by fresh diagnosed blocker | Phase 53 UAT records fresh `openbitcoinsyncstatus` snapshots and no validated header-height increase. |
| `PROOF-04` | Passed by fresh diagnosed blocker | Phase 53 UAT records `result.blockDelta=0` and no validated block connection. |
| `PROOF-05` | Passed by fresh diagnosed blocker | Default and manual-peer attempts reused `packages/target/phase53-mainnet-datadir`; the retry produced coherent fresh-status snapshots. |
| `OBS-02` | Passed by fresh diagnosed blocker | Per-poll snapshots and final status agree on lifecycle, phase, outbound peers, heights, and last error. |
| `SEC-03` | Passed by fresh diagnosed blocker | UAT records typed public-mainnet blocker evidence, endpoint outcomes, support context, and next operator action. |

## UAT Evidence

Live evidence was generated through the existing opt-in smoke runner:

```bash
bun run scripts/run-live-mainnet-smoke.ts \
  --datadir=/Users/peterryszkiewicz/Repos/open-bitcoin/packages/target/phase53-mainnet-datadir \
  --output-dir=/Users/peterryszkiewicz/Repos/open-bitcoin/packages/target/live-mainnet-smoke-reports/phase53-default \
  --timeout-seconds=180 \
  --poll-seconds=5 \
  --min-free-gib=1
```

The same-datadir manual-peer retry produced the selected report:

```bash
bun run scripts/run-live-mainnet-smoke.ts \
  --datadir=/Users/peterryszkiewicz/Repos/open-bitcoin/packages/target/phase53-mainnet-datadir \
  --output-dir=/Users/peterryszkiewicz/Repos/open-bitcoin/packages/target/live-mainnet-smoke-reports/phase53-manual-peer \
  --timeout-seconds=180 \
  --poll-seconds=5 \
  --min-free-gib=1 \
  --manual-peer=dnsseed.bluematt.me:8333 \
  --manual-peer=seed.bitcoin.jonasschnelli.ch:8333
```

Support evidence was generated with the repo-local Cargo command:

```bash
cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- \
  --datadir=/Users/peterryszkiewicz/Repos/open-bitcoin/packages/target/phase53-mainnet-datadir \
  support bundle \
  --output-dir=/Users/peterryszkiewicz/Repos/open-bitcoin/packages/target/phase53-support \
  --include-live-smoke-report=/Users/peterryszkiewicz/Repos/open-bitcoin/packages/target/live-mainnet-smoke-reports/phase53-manual-peer/open-bitcoin-live-mainnet-smoke.json
```

Bazel equivalent for operators:

```bash
bazel run //packages/open-bitcoin-cli:open_bitcoin -- \
  --datadir=/Users/peterryszkiewicz/Repos/open-bitcoin/packages/target/phase53-mainnet-datadir \
  support bundle \
  --output-dir=/Users/peterryszkiewicz/Repos/open-bitcoin/packages/target/phase53-support \
  --include-live-smoke-report=/Users/peterryszkiewicz/Repos/open-bitcoin/packages/target/live-mainnet-smoke-reports/phase53-manual-peer/open-bitcoin-live-mainnet-smoke.json
```

Generated reports, datadirs, and support bundles remain local under
`packages/target` and are not checked into git.

## Deterministic Verification

Passed:

```bash
bash scripts/test-run-live-mainnet-smoke.sh
cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli open_bitcoin_support_bundle --test operator_binary
bun run scripts/check-v1.3-release-boundaries.ts
cargo fmt --manifest-path packages/Cargo.toml --all
cargo clippy --manifest-path packages/Cargo.toml --workspace --all-targets --all-features -- -D warnings
cargo build --manifest-path packages/Cargo.toml --workspace --all-targets --all-features
cargo test --manifest-path packages/Cargo.toml --workspace --all-features
bash scripts/verify.sh
```

Additional closeout checks passed:

```bash
node -e 'JSON.parse(require("fs").readFileSync("docs/parity/index.json","utf8")); console.log("index json ok")'
node "$HOME/.codex/get-shit-done/bin/gsd-tools.cjs" audit-uat --raw
git ls-files packages/target/live-mainnet-smoke-reports packages/target/phase53-mainnet-datadir packages/target/phase53-support
bash -lc '! rg -n "run-live-mainnet-smoke" scripts/verify.sh'
```

The `git ls-files` check produced no output.

Code review scope: skipped because Phase 53 changed planning and documentation
artifacts only; no Rust, TypeScript, or shell source files were modified.

## Residual Risk

The public network did not allow accepted useful peer contribution, validated
header progress, or validated block progress in this environment. That is an
accepted opt-in UAT outcome for Phase 53 because the selected report is fresh,
typed, actionable, and generated after the Phase 51 fresh-status fix. The next
operator action is to retry with a different reachable manual peer or inspect
daemon stderr and endpoint outcomes for the selected report.

## Self-Check: PASSED
