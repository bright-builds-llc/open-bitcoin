---
generated_by: gsd-phase-researcher
lifecycle_mode: yolo
phase_lifecycle_id: 48-2026-05-27T13-21-54
generated_at: 2026-05-27T13:33:45Z
status: complete
requirements:
  - OBS-03
  - OBS-04
---

# Phase 48 Research: Support Evidence and Operator Runbooks

## Research Complete

Phase 48 can be implemented as an operator CLI and documentation extension over
existing status, config, durable-store, metrics, logs, and live-smoke evidence
surfaces. No consensus, P2P, wallet-spend, or sync-protocol semantic changes are
needed.

## Existing Surfaces To Reuse

### Operator CLI

- `packages/open-bitcoin-cli/src/operator.rs` defines top-level operator
  subcommands and global `--datadir`, `--config`, `--network`, `--format`, and
  `--no-color` flags.
- `packages/open-bitcoin-cli/src/operator/runtime.rs` owns operator command
  dispatch, config resolution, detection roots, and status runtime assembly.
- `packages/open-bitcoin-cli/src/operator/runtime/support.rs` is an existing
  support-helper module for sync command execution and config-path rendering.
  It is a reasonable local pattern for keeping small operator support helpers
  close to runtime dispatch, though a new `operator/support.rs` module will be
  clearer if the bundle logic grows.

### Status and Observability

- `packages/open-bitcoin-cli/src/operator/status.rs` builds an
  `OpenBitcoinStatusSnapshot` from config resolution, detection evidence,
  optional live RPC, service manager, durable sync state, logs, metrics, wallet
  access, and build metadata.
- `packages/open-bitcoin-node/src/status.rs` defines the shared status snapshot
  consumed by CLI, dashboard, RPC-facing projections, and support paths.
- `packages/open-bitcoin-cli/src/operator/status/render.rs` renders the shared
  snapshot to stable JSON or compact human output and already contains redaction
  regression coverage around credentials and cookie contents.
- `docs/architecture/status-snapshot.md` names the snapshot as the single shared
  model for status, dashboard, service diagnostics, and support reports.

### Config, Store, Logs, and Metrics

- `resolve_operator_config` reports Open Bitcoin JSONC, baseline `bitcoin.conf`,
  datadir, log directory, metrics store directory, network, and source
  precedence without exposing credential values.
- `packages/open-bitcoin-node/src/storage/fjall_store.rs` can open the selected
  datadir and load runtime metadata and metrics status. Store-health evidence
  can be represented as available/unavailable diagnostics without inventing a
  new storage inspection layer.
- `packages/open-bitcoin-node/src/logging/writer.rs` exposes `load_log_status`,
  which reads bounded managed structured-log signals without dumping raw log
  files into the bundle.
- `packages/open-bitcoin-node/src/metrics.rs` exposes `MetricsStatus`,
  retention policy, enabled series, and bounded samples.

### Live-Smoke Evidence

- `scripts/run-live-mainnet-smoke.ts` already writes
  `open-bitcoin-live-mainnet-smoke.json` and
  `open-bitcoin-live-mainnet-smoke.md` under a local report directory.
- That script records manual peers, generated config path, endpoint outcomes,
  typed no-progress causes, status snapshots, daemon output tails, suggested
  next action, and peer contribution rows.
- Phase 48 should consume live-smoke artifacts optionally by path or report
  directory. Absence should be represented as unavailable evidence, not a
  command failure.

## Recommended Implementation Shape

### Command Shape

Add a top-level operator subcommand:

```text
open-bitcoin support bundle --output-dir PATH [--include-live-smoke-report PATH]
```

The command should respect existing global flags:

```text
open-bitcoin --datadir PATH --config PATH --network mainnet support bundle --output-dir PATH
open-bitcoin --format json --datadir PATH support bundle --output-dir PATH
```

Recommended output files:

- `support-evidence.json` for stable machine-readable review.
- `support-evidence.md` for concise human review.

The command stdout can report the two generated paths in human mode and the
bundle metadata in JSON mode.

### Evidence Contract

The JSON evidence should include at least:

- bundle metadata: generated timestamp, selected datadir, command/output paths,
  and redaction summary.
- config evidence: selected Open Bitcoin JSONC path, baseline `bitcoin.conf`
  path, datadir, logs path, metrics path, network, and source precedence.
- status evidence: `OpenBitcoinStatusSnapshot`.
- sync control/runtime evidence: `RuntimeMetadata` when available, or an
  unavailable reason.
- store health evidence: whether the durable store opened, whether runtime
  metadata and metrics were readable, and any typed error strings.
- log evidence: `LogStatus` with bounded recent health signals, not raw
  unbounded logs.
- metrics evidence: `MetricsStatus` with retention and bounded samples.
- live-smoke evidence: optional artifact paths and selected parsed summary
  fields, or an unavailable reason.

The Markdown summary should mirror the JSON sections in a compact, quiet,
operator-oriented format:

- bundle overview
- redaction summary
- config and local paths
- status and sync summary
- peer/log/metrics/store-health summary
- live-smoke artifact summary
- next diagnostic steps

### Redaction Rules

The bundle should never include:

- RPC cookie contents.
- `rpcpassword` values.
- raw wallet bytes or wallet file contents.
- private keys, descriptors containing private keys, or WIF strings.
- raw unbounded log files.

Allowed review context:

- credential source metadata such as cookie path/source and presence.
- config and datadir paths.
- endpoint host/port strings already exposed by status/live-smoke reports.
- peer state, contribution counters, failure reasons, timestamps, and typed
  recovery guidance.
- bounded structured log health signals.
- metrics retention and bounded sample values.

Implement redaction as explicit data shaping rather than string replacement
over a raw archive. Tests should assert known secret fixture values do not
appear in JSON, Markdown, or stdout.

## Documentation Requirements

Update `docs/operator/runtime-guide.md` with a support evidence/runbook section
that includes copy-pasteable repo-local Cargo and Bazel commands.

Cargo example:

```bash
cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- \
  --datadir=/tmp/open-bitcoin-mainnet support bundle --output-dir=/tmp/open-bitcoin-support
```

Bazel example:

```bash
bazel run //packages/open-bitcoin-cli:open_bitcoin -- \
  --datadir=/tmp/open-bitcoin-mainnet support bundle --output-dir=/tmp/open-bitcoin-support
```

Live-smoke examples should keep the repo-owned script primary:

```bash
bun run scripts/run-live-mainnet-smoke.ts --datadir=/tmp/open-bitcoin-mainnet \
  --timeout-seconds=60 --poll-seconds=5 --manual-peer=HOST[:PORT]
```

Docs should explain:

- disk and network expectations for deterministic support bundle generation and
  optional live-mainnet smoke.
- how to interpret missing live-smoke artifacts.
- pass/fail interpretation for OBS-03 and OBS-04.
- redaction scope and what operators should still review before sharing.
- that the bundle is local-only and does not mutate source datadirs, services,
  wallets, configs, or daemon sync control.

## Testing Strategy

Use deterministic local tests only:

- unit tests for support evidence data shaping and Markdown rendering.
- binary/integration test that runs `open-bitcoin --datadir TEMP support bundle
  --output-dir TEMP` and asserts both files exist.
- fixture test with `bitcoin.conf` containing `rpcpassword=secret`, `.cookie`
  containing a secret, and wallet-like fixture files to assert secret strings do
  not appear in JSON/Markdown/stdout.
- fixture test for optional live-smoke JSON/Markdown paths proving the bundle
  records artifact availability and selected summary fields.
- status/store tests using temporary Fjall stores, matching existing operator
  binary test style.

Run targeted verification during implementation:

```bash
cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli operator::support
cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli --test operator_binary support
```

Final verification remains:

```bash
bash scripts/verify.sh
```

## Risks and Mitigations

- **Risk:** accidentally archiving raw sensitive files.
  **Mitigation:** do not copy raw config, cookie, wallet, or log files; shape
  explicit JSON structs from already-redacted sources.
- **Risk:** bundle duplicates status DTOs and drifts from status/dashboard/RPC.
  **Mitigation:** embed the shared `OpenBitcoinStatusSnapshot` directly.
- **Risk:** public-network proof scope leaks into Phase 48.
  **Mitigation:** consume live-smoke artifacts optionally and leave final proof
  to Phase 50.
- **Risk:** docs regress to installed-binary-only examples.
  **Mitigation:** make Cargo and Bazel examples primary in the runbook.

## Planning Recommendation

One implementation plan is sufficient:

1. Add typed support bundle CLI contract and redacted evidence rendering.
2. Add deterministic tests for file output, redaction, optional live-smoke
   artifacts, and local store/status behavior.
3. Update operator runbooks and architecture/status docs.
4. Refresh parity breadcrumbs and tracked LOC if verification regenerates them.

## RESEARCH COMPLETE

Research artifact written to
`.planning/phases/48-support-evidence-and-operator-runbooks/48-RESEARCH.md`.
