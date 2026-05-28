# Phase 50: Public Mainnet Progress Evidence Closeout - Research

**Researched:** 2026-05-28 [VERIFIED: environment context]
**Domain:** Opt-in public-mainnet smoke evidence, support bundles, parity closeout [VERIFIED: `.planning/phases/50-public-mainnet-progress-evidence-closeout/50-CONTEXT.md`]
**Confidence:** HIGH for existing repo seams and artifact shapes; MEDIUM for live public-network outcome because peer reachability and progress are environment-dependent. [VERIFIED: `scripts/run-live-mainnet-smoke.ts`, `docs/operator/runtime-guide.md`]

<user_constraints>
## User Constraints From Context

- Use `scripts/run-live-mainnet-smoke.ts` as the authoritative public-mainnet evidence source. [VERIFIED: `50-CONTEXT.md`]
- Keep generated live-smoke and support-bundle artifacts local under `packages/target` or an explicit output directory; do not commit generated live reports. [VERIFIED: `50-CONTEXT.md`]
- Prefer successful progress evidence, but accept a diagnosed environment/network blocker when bounded attempts fail and the report has typed cause, endpoint outcomes, status snapshots, and next action. [VERIFIED: `50-CONTEXT.md`, `docs/parity/release-readiness.md`]
- Reuse the same datadir for restart/resume evidence. Claim restart/resume only if durable header, block, and runtime metadata remain coherent. [VERIFIED: `50-CONTEXT.md`]
- Default verification remains deterministic and public-network-free. [VERIFIED: `AGENTS.md`, `docs/operator/runtime-guide.md`]
</user_constraints>

<phase_requirements>
## Requirement Mapping

| ID | Requirement | Research Finding |
|----|-------------|------------------|
| PROOF-03 | Inspect a live smoke report showing first observed validated header-height increase with peer endpoint, source, timestamp, and before/after durable status. | `scripts/run-live-mainnet-smoke.ts` reports `result.headerDelta`, `snapshots`, `final_status`, `network_preflight.endpoint_outcomes`, and runtime peer rows. If no header increase occurs, Phase 49 allows diagnosed-blocker closeout. [VERIFIED: `scripts/run-live-mainnet-smoke.ts`, `docs/parity/release-readiness.md`] |
| PROOF-04 | Inspect a live smoke report showing first validated block connection beyond genesis/checkpoint or explicit diagnosis when block progress was not reached. | The runner reports `result.blockDelta` and typed `maybeNoProgressCause` with `nextAction` on no progress. [VERIFIED: `scripts/run-live-mainnet-smoke.ts`] |
| PROOF-05 | Interrupt/restart same datadir and see durable before/after evidence that header, block, and runtime metadata resume coherently. | The runner accepts explicit `--datadir=PATH`. A second invocation over the same datadir can provide before/after snapshots; support bundle can capture shared `OpenBitcoinStatusSnapshot`. [VERIFIED: `scripts/run-live-mainnet-smoke.ts`, `packages/open-bitcoin-cli/src/operator/support.rs`] |
| SEC-03 | UAT records successful public-mainnet evidence or diagnosed blocker with enough detail for next operator action. | Phase UAT should commit commands, local artifact paths, selected report fields, endpoint outcomes, no-progress cause when present, and next action. [VERIFIED: `docs/parity/release-readiness.md`, `50-CONTEXT.md`] |
</phase_requirements>

## Existing Evidence Surfaces

### Live-Mainnet Smoke Runner

The repo-owned command is:

```bash
bun run scripts/run-live-mainnet-smoke.ts --datadir=PATH [--config=PATH] [--manual-peer=HOST[:PORT]]... [--output-dir=PATH] [--timeout-seconds=N] [--poll-seconds=N] [--min-free-gib=N]
```

Important behavior:

- The selected datadir must already exist. Missing datadir produces a preflight-failed JSON/Markdown report and exits nonzero. [VERIFIED: `scripts/run-live-mainnet-smoke.ts`]
- Manual peers cannot be combined with `--config`; with manual peers and no config, the runner writes `open-bitcoin-live-mainnet-smoke.jsonc`, disables DNS seeds for that run, and sets `sync.target_outbound_peers = 1`. [VERIFIED: `scripts/run-live-mainnet-smoke.ts`, `docs/operator/runtime-guide.md`]
- The runner builds current binaries, starts `open-bitcoind` with explicit `mainnet-ibd` activation, polls `getblockchaininfo`, terminates its own daemon, and writes JSON/Markdown reports. [VERIFIED: `scripts/run-live-mainnet-smoke.ts`, `docs/operator/runtime-guide.md`]
- Report fields needed for Phase 50 already exist: `result.status`, `result.progressDetected`, `result.headerDelta`, `result.blockDelta`, `result.maybeNoProgressCause`, `result.nextAction`, `network_preflight.endpoint_outcomes`, `snapshots`, `final_status`, `daemon.stdoutTail`, and `daemon.stderrTail`. [VERIFIED: `scripts/run-live-mainnet-smoke.ts`]
- The runner exits nonzero for `preflight_failed`, `runtime_failed`, `no_progress`, and `cancelled`, even though those reports are still useful evidence for a diagnosed-blocker closeout. [VERIFIED: `scripts/run-live-mainnet-smoke.ts`]

### Support Bundle

The repo-local support bundle command is:

```bash
cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- \
  --datadir=PATH \
  support bundle \
  --output-dir=PATH \
  --include-live-smoke-report=PATH
```

It writes `support-evidence.json` and `support-evidence.md`, includes the shared `OpenBitcoinStatusSnapshot`, store health, redaction metadata, and an allowlisted live-smoke summary when supplied. It does not embed raw live-smoke input and is not a release validator. [VERIFIED: `packages/open-bitcoin-cli/src/operator/support.rs`, `docs/operator/runtime-guide.md`]

## Planning Implications

- Create one execution plan rather than multiple independent plans: evidence capture, summarization, closeout docs, and verification are sequential and share the same datadir/artifacts.
- The plan must treat nonzero live-smoke exit codes as expected on the diagnosed-blocker path and must inspect the generated report before deciding pass/fail.
- The plan should use explicit target-local paths such as `packages/target/phase50-mainnet-datadir`, `packages/target/live-mainnet-smoke-reports/phase50`, and `packages/target/phase50-support`.
- The plan should start with a bounded default attempt, then run a bounded same-datadir manual-peer retry if endpoint discovery/connectivity prevents useful progress. The exact manual peer can be adjusted at execution time.
- The committed UAT artifact should summarize generated evidence rather than checking generated live reports into git.
- Release-readiness updates should be small and outcome-driven: link the Phase 50 UAT/evidence artifact and record whether the closeout is progress evidence or diagnosed-blocker evidence.

## Risks

- Public-network reachability and useful peer contribution are environment-dependent. The live-smoke command may produce a valid diagnosed-blocker artifact instead of successful header/block progress.
- Block progress may lag header progress in a bounded run. PROOF-04 explicitly permits diagnosis when block progress is not reached.
- PROOF-05 should not be overstated. If no successful progress appears, the UAT can record same-datadir restart evidence for the blocker path, but should not claim durable resume success.
- Support bundle creation may still work when live-smoke exits nonzero, provided the generated report path exists and the datadir is readable.

## Verification Strategy

- UAT: Run the public-mainnet smoke command(s), inspect generated JSON with structured tooling, generate a support bundle when possible, and commit a Phase 50 evidence summary.
- Deterministic checks: run the required Rust pre-commit sequence and `bash scripts/verify.sh`.
- Lifecycle checks: create `50-VERIFICATION.md` with `status: passed` only after the UAT evidence and deterministic checks are complete, then validate the GSD lifecycle.

## RESEARCH COMPLETE
