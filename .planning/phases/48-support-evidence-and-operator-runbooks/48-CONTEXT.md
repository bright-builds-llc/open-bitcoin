---
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: 48-2026-05-27T13-21-54
generated_at: 2026-05-27T13:24:25.133Z
---

# Phase 48: Support Evidence and Operator Runbooks - Context

**Gathered:** 2026-05-27
**Status:** Ready for planning
**Mode:** Yolo

<domain>
## Phase Boundary

Phase 48 gives operators and reviewers a local, redacted support-evidence
surface and repo-local runbooks for v1.3 proof. It should collect existing
config-source, status, sync, peer, log, metrics, store-health, build, and
live-smoke artifact evidence into files that can be reviewed without hosted
services, packaged installs, destructive migration, or public-network checks in
the default verification gate.

This phase does not own final public-mainnet header/block/restart proof, the
v1.3 threat model, release-claim boundaries, inbound serving, transaction relay,
production-funds wallet behavior, migration apply mode, packaging, hosted
dashboards, or GUI work.

</domain>

<decisions>
## Implementation Decisions

### Support Evidence Surface

- **D-01:** Add an explicit operator-facing support evidence command rather than
  making reviewers stitch together `status`, `sync status`, config paths, logs,
  metrics, and live-smoke report paths by hand.
- **D-02:** Keep evidence local and file-based. The command should write a
  small support bundle directory with machine-readable JSON and a concise
  Markdown summary instead of uploading data or requiring a hosted service.
- **D-03:** Build the bundle from existing authoritative surfaces:
  `OpenBitcoinStatusSnapshot`, `RuntimeMetadata`, config path resolution,
  bounded metrics/log metadata, durable store open/read diagnostics, and
  optional live-smoke report artifacts.
- **D-04:** Treat live-smoke artifacts as optional inputs. If a report directory
  or report path is absent, the bundle must say the artifact is unavailable with
  a reason rather than failing the whole command.

### Redaction And Safety

- **D-05:** Redaction is a first-class part of the evidence contract. Cookie
  contents, RPC passwords, raw wallet data, and other credential-like values
  must never appear in JSON, Markdown, tests, or docs.
- **D-06:** Preserve useful review context while redacting sensitive values:
  include paths, source names, endpoint host/port strings already exposed by
  status/live-smoke evidence, availability states, counts, timestamps, and typed
  failure/recovery reasons.
- **D-07:** Support evidence must not mutate source datadirs, source services,
  wallet files, config files, or live daemon sync state. It is an inspection and
  packaging surface only.
- **D-08:** The command should be deterministic and testable with local fixture
  files and temporary stores. Public-network behavior remains opt-in and outside
  `bash scripts/verify.sh`.

### Operator Runbooks

- **D-09:** Update operator docs with copy-pasteable repo-local Cargo and Bazel
  commands for support bundle generation, status inspection, sync status,
  live-smoke runs, and manual-peer examples.
- **D-10:** Runbooks must describe disk/network expectations, redaction scope,
  local artifact locations, troubleshooting branches, and pass/fail
  interpretation for v1.3 evidence.
- **D-11:** Documentation may mention an installed `open-bitcoin` binary only as
  a convenience. The primary commands must be repo-local `cargo run` and
  `bazel run` forms.

### Integration With Later Phases

- **D-12:** The bundle and runbooks should create stable inputs for Phase 49
  threat/release review and Phase 50 live-mainnet evidence closeout, but should
  not claim those later phase outcomes are already proven.
- **D-13:** The evidence summary should clearly distinguish deterministic local
  evidence from optional live-mainnet artifacts and diagnosed environment or
  network blockers.

### the agent's Discretion

The planner may choose exact command names, module boundaries, output filenames,
and helper types as long as the command is discoverable from the existing
operator CLI, the JSON/Markdown outputs are stable enough for tests and docs,
redaction is covered by deterministic tests, and the docs keep repo-local
commands primary.

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Phase Scope

- `.planning/ROADMAP.md` - Phase 48 goal, requirements, and success criteria.
- `.planning/REQUIREMENTS.md` - OBS-03 and OBS-04 acceptance requirements.
- `.planning/PROJECT.md` - v1.3 public-mainnet proof scope and deferred
  production-node claims.
- `.planning/STATE.md` - Current v1.3 state and latest Phase 47 completion
  context.

### Prior Phase Decisions

- `.planning/phases/44-peer-contribution-attribution/44-CONTEXT.md` - Peer
  contribution evidence semantics and live-smoke support evidence expectations.
- `.planning/phases/45-runtime-resource-bounds-and-store-coordination/45-CONTEXT.md`
  - Resource-bound, metrics/log retention, and second-writer diagnostic
  decisions.
- `.planning/phases/46-durable-recovery-and-invalid-data-handling/46-CONTEXT.md`
  - Durable progress, invalid-data, and recovery guidance decisions.
- `.planning/phases/47-operator-sync-truth-surfaces/47-CONTEXT.md` - Shared
  status truth model and repo-local UAT command decision.
- `.planning/phases/47-operator-sync-truth-surfaces/47-SUMMARY.md` - Actual
  Phase 47 status, CLI, dashboard, metrics, logs, and RPC surface changes.

### Operator And Evidence Surfaces

- `packages/open-bitcoin-cli/src/operator.rs` - Operator CLI subcommand shape.
- `packages/open-bitcoin-cli/src/operator/runtime.rs` - Operator command
  dispatch, status runtime assembly, config resolution, and detection flow.
- `packages/open-bitcoin-cli/src/operator/runtime/support.rs` - Existing sync
  command support helpers and config-path rendering patterns.
- `packages/open-bitcoin-cli/src/operator/status.rs` - Shared status collection
  inputs and snapshot assembly.
- `packages/open-bitcoin-cli/src/operator/status/render.rs` - Existing
  human/JSON status rendering style and redaction conventions.
- `packages/open-bitcoin-node/src/status.rs` - `OpenBitcoinStatusSnapshot`,
  build provenance, sync, peer, logs, metrics, and health contracts.
- `packages/open-bitcoin-node/src/storage/fjall_store.rs` - Durable metadata and
  metrics loading behavior for store-health evidence.
- `scripts/run-live-mainnet-smoke.ts` - Existing opt-in live-smoke JSON and
  Markdown artifact generation.
- `scripts/test-run-live-mainnet-smoke.sh` - Deterministic live-smoke report
  fixture coverage.

### Documentation

- `docs/operator/runtime-guide.md` - Operator runtime guide, live-smoke
  commands, report interpretation, and known limitations.
- `docs/architecture/status-snapshot.md` - Shared status snapshot ownership,
  stopped-node behavior, sync progress semantics, metrics, logs, and build
  provenance.
- `docs/architecture/config-precedence.md` - Config path/source and
  credential-reporting rules.
- `docs/architecture/operator-observability.md` - Metrics/log retention and
  observability boundaries.

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets

- `OperatorCommand` and `execute_operator_cli_inner` already route top-level
  operator subcommands through typed outcomes.
- `resolve_operator_config` already collects datadir, Open Bitcoin JSONC,
  baseline `bitcoin.conf`, log directory, metrics directory, network, and
  source precedence.
- `status_runtime_parts` already assembles the same config, detection,
  live-RPC, service, and wallet access inputs used by human/JSON status.
- `collect_status_snapshot` already returns one shared status model containing
  node, config, service, sync, peers, mempool, wallet, logs, metrics, health,
  and build sections.
- `FjallNodeStore` already loads runtime metadata and metrics status from the
  durable store.
- `scripts/run-live-mainnet-smoke.ts` already writes JSON and Markdown evidence
  reports with endpoint outcomes, status snapshots, daemon output tails, typed
  no-progress causes, and peer contribution rows.

### Established Patterns

- Unavailable evidence should be represented explicitly with a reason rather
  than by omitting fields or guessing defaults.
- Operator outputs stay quiet, local, and audit-oriented. Markdown is acceptable
  for human review, JSON is preferred for machine-readable evidence.
- Tests use temporary datadirs/stores and fixture files rather than requiring a
  public network.
- Status and migration tests already assert redaction of credentials, cookie
  contents, and raw wallet data.

### Integration Points

- Add a new operator CLI command under `packages/open-bitcoin-cli/src/operator.rs`.
- Add support evidence collection/rendering under the operator CLI module,
  reusing existing status/config/store helpers where practical.
- Add binary-level tests in `packages/open-bitcoin-cli/tests/operator_binary.rs`
  and focused module tests if the support evidence helpers are nontrivial.
- Update `docs/operator/runtime-guide.md` and any architecture doc that defines
  support/status evidence ownership.
- Update parity breadcrumbs for any new first-party Rust source or tests under
  `packages/open-bitcoin-*/src` or `packages/open-bitcoin-*/tests`.

</code_context>

<specifics>
## Specific Ideas

- A command shape like `open-bitcoin support bundle --output-dir PATH
  [--include-live-smoke-report PATH]` would keep the operator action explicit
  and testable, but exact naming is left to the planner.
- Write bundle outputs such as `support-evidence.json` and
  `support-evidence.md` under the selected output directory.
- Include a redaction manifest or `redaction` section that names classes of
  omitted data, for example RPC secrets, cookie contents, wallet files, and raw
  logs beyond bounded tails.
- Include repo-local command examples in docs for both Cargo and Bazel:
  `cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin
  open-bitcoin -- ...` and
  `bazel run //packages/open-bitcoin-cli:open_bitcoin -- ...`.

</specifics>

<deferred>
## Deferred Ideas

None - discussion stayed within phase scope.

</deferred>

---

*Phase: 48-support-evidence-and-operator-runbooks*
*Context gathered: 2026-05-27*
